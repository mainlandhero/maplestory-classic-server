//! **The client's options**: what it sends when one changes (`0x02EB`), and where it reads them
//! back from at field entry (two quest-ex stores in the character record).
//!
//! The owner, 2026-10-02: *"Saving of settings such as audio, HP thresholds, etc on server side per
//! account"* - and *"HP thresholds means pet auto pot HP thresholds"*.
//!
//! # What the client does [L]
//!
//! An options object holds two `std::map<int,int>` groups, and `FUN_140426d40(group, key)`
//! says which keys are the server's: group 0 keys `< 0x41`, group 1 keys `< 0x15`. Those keys
//! are **not written to the registry** - the registry under `SOFTWARE\Wizet\MapleStoryClassic`
//! holds window positions, screen mode and BGM, and has no `soHPFlash` or `soAutoConsumePetHP`
//! on a machine that has played for weeks.
//!
//! * **Sending**: six builders of `0x02EB` (`FUN_141601390`, `141601440`, `1416018a0`,
//!   `141601d10`, `141601dc0`, `1411f4f70`): `u32 group, u8 count, count x (u32 key, i32 value)`.
//!   A setter sends only what it changed - the HP/MP warning pair goes as two keys from
//!   `FUN_1415f1640` - and field entry sends the whole group. Every archived body is exactly
//!   `5 + 8n` bytes (13, 21, 109, 173, 389, 397, 405).
//! * **Reading back**, at field entry (`FUN_1415f7140` for group 1, `FUN_1415f22f0` for group
//!   0, both called from `0x141818ae0`): `FUN_140426ed0` turns a group-1 key into its name
//!   ([`GAME_OPTION_KEYS`], `FUN_140427040`) and looks it up in the **quest ex** record of
//!   quest 101563, then 101790 (`FUN_1402E01C0`, the `+0x12BB` map = record block #28).
//!   `FUN_140426d60` does the same for group 0 ([`SYSTEM_OPTION_KEYS`], `FUN_140427230`) in
//!   quests 368, 369, 370, 481 through `FUN_1402E0430` - the `+0x12D3` map, record block #32.
//!   The first quest that has the key wins; the value is the record's `key=value;` text.
//!
//! The HP warning (`flHP`, 0..19, default 10) is the threshold the pet's Auto HP drinks at;
//! its group-0 twin `wrnHP3` is the older slot the reader falls back to.

use crate::packet::{PacketReader, PacketWriter};

/// `0x02EB` - one or more options changed. Not latched: nothing is expected back.
pub const CLIENT_OPTIONS_CHANGED: u16 = 0x02EB;

/// Group 0: the system options. Names off `PTR_DAT_1432874f0`, 66 slots with `COUNT` last.
/// **[L]**
pub const SYSTEM_OPTION_KEYS: [&str; 0x41] = [
    "vBG1", "mBG1", "vE1", "mE1", "mAnd1", "vSE1", "mSE1", "vSV1", "mSV1", "vM1", "mM1", "vME1", "mME1",
    "fLoc1", "fType1", "sAuto1", "sCon1", "sNum1", "mobInf1", "qsEff1", "damEff1", "vSync1", "fSize2",
    "fcW2", "fcF2", "fcG2", "fcA2", "aMine2", "aOther2", "magUI1", "trem2", "aUI2", "pBack2", "simItm2",
    "cmbMsg2", "avMega2", "chPos2", "chTime2", "petHP2", "qsTime2", "wrnHP3", "wrnMP3", "wndHtK1",
    "gqI3", "gqB3", "gqE3", "gqP3", "gqT3", "gqL3", "gqCB3", "gqCE3", "gqCP3", "gqCT3", "gqCL3",
    "damAmt3", "silBo3", "silTy3", "silTh3", "mlUpAc2", "mlUpTy2", "aMyPet", "aUrPet", "aMyAnd",
    "aUrAnd", "infItm",
];

/// Group 1: the game options. Names off `PTR_DAT_143287b10`, 22 slots with `COUNT` last.
/// **[L]** `0x0F acpHP` is Auto HP on/off, `0x10 flHP` / `0x11 flMP` the warning thresholds.
pub const GAME_OPTION_KEYS: [&str; 0x15] = [
    "alWh", "chFr", "alMe", "alExch", "shMd", "shNk", "soulUI", "alPa", "alGu", "alAl", "chGu", "chAl",
    "alFr", "frOnNot", "soErUI", "acpHP", "flHP", "flMP", "alFol", "bufAual", "bufMin",
];

/// Group numbers as the wire carries them.
pub const GROUP_SYSTEM: u32 = 0;
pub const GROUP_GAME: u32 = 1;

/// The quest whose ex record carries the game options - the first of the two the reader
/// tries (`DAT_1432874e8` = 101563, 101790). **[L]**
pub const GAME_OPTIONS_QUEST: u32 = 101_563;

/// The four quests the system-option reader tries, in order (`DAT_1432874d8`). **[L]**
pub const SYSTEM_OPTIONS_QUESTS: [u32; 4] = [368, 369, 370, 481];

/// The presence byte gating record block #32 - `u16 count, count x (u32 questId, str value)`
/// into `+0x12D3` (`FUN_1402E1A20`). Gate `0x140308e79`, after the record's last ungated byte,
/// and the gates between it and that byte (37, 41, 23, 52, 29, 39) are never set by this
/// server. **[L]** `research/charrecord-presence-map.md`.
pub const PRESENCE_SHARED_QUEST_EX: usize = 19;

/// One `0x02EB`: a group and its `(key, value)` pairs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionsChanged {
    pub group: u32,
    pub entries: Vec<(u32, i32)>,
}

/// Parse a `0x02EB` body (opcode stripped). `None` unless it is exactly `5 + 8 x count` bytes.
pub fn parse_options_changed(body: &[u8]) -> Option<OptionsChanged> {
    let mut r = PacketReader::new(body);
    let group = r.u32().ok()?;
    let count = usize::from(r.u8().ok()?);
    if r.remaining() != count * 8 {
        return None;
    }
    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        entries.push((r.u32().ok()?, r.i32().ok()?));
    }
    Some(OptionsChanged { group, entries })
}

/// The name a `(group, key)` is stored under, or `None` for a key the client does not keep on
/// the server.
pub fn key_name(group: u32, key: u32) -> Option<&'static str> {
    let key = usize::try_from(key).ok()?;
    match group {
        GROUP_SYSTEM => SYSTEM_OPTION_KEYS.get(key).copied(),
        GROUP_GAME => GAME_OPTION_KEYS.get(key).copied(),
        _ => None,
    }
}

/// **The options kept per CHARACTER rather than per account**: the pet's auto-potion setup.
/// The owner, 2026-10-02: *"other characters may want to use different auto potion setup."*
/// `acpHP` / `flHP` / `flMP` are the game-option trio (the live server's log shows the window
/// sending `flHP` and `flMP`); `petHP2`, `wrnHP3` and `wrnMP3` are their group-0 twins, the
/// older slots the reader falls back to, kept with them so the fallback cannot leak another
/// character's threshold. Everything else - sound, screen, chat, UI - stays per account.
pub const PER_CHARACTER_KEYS: [&str; 6] = ["acpHP", "flHP", "flMP", "petHP2", "wrnHP3", "wrnMP3"];

/// Whether `(group, key)` is stored for the character ([`PER_CHARACTER_KEYS`]).
pub fn is_per_character(group: u32, key: u32) -> bool {
    key_name(group, key).is_some_and(|n| PER_CHARACTER_KEYS.contains(&n))
}

/// Which of [`SYSTEM_OPTIONS_QUESTS`] a system option goes in: its name's trailing digit -
/// `vBG1` in 368, `fSize2` in 369, `wrnHP3` in 370, the rest (`aMyPet`, `infItm`, ...) in 481.
/// Any of the four would be read; this keeps each record short.
pub fn system_option_quest(name: &str) -> u32 {
    match name.chars().last() {
        Some('1') => SYSTEM_OPTIONS_QUESTS[0],
        Some('2') => SYSTEM_OPTIONS_QUESTS[1],
        Some('3') => SYSTEM_OPTIONS_QUESTS[2],
        _ => SYSTEM_OPTIONS_QUESTS[3],
    }
}

/// Stored `(group, key, value)` rows as the two record blocks want them:
/// `(block #28 entries, block #32 entries)`, each `(questId, "k=v;k=v")`. Unknown keys are
/// dropped; an empty group yields no entry.
pub fn records(rows: &[(u32, u32, i32)]) -> (Vec<(u32, String)>, Vec<(u32, String)>) {
    let mut game = Vec::new();
    let mut system: Vec<(u32, Vec<String>)> = Vec::new();
    for &(group, key, value) in rows {
        let Some(name) = key_name(group, key) else { continue };
        let pair = format!("{name}={value}");
        if group == GROUP_GAME {
            game.push(pair);
        } else {
            let quest = system_option_quest(name);
            match system.iter_mut().find(|(q, _)| *q == quest) {
                Some((_, v)) => v.push(pair),
                None => system.push((quest, vec![pair])),
            }
        }
    }
    let block28 = if game.is_empty() { Vec::new() } else { vec![(GAME_OPTIONS_QUEST, game.join(";"))] };
    system.sort_by_key(|(q, _)| *q);
    (block28, system.into_iter().map(|(q, v)| (q, v.join(";"))).collect())
}

/// Record block #32 - the same shape as block #28 (`crate::citizenship::quest_ex_block`).
pub fn shared_quest_ex_block(entries: &[(u32, String)]) -> Vec<u8> {
    crate::citizenship::quest_ex_block(entries)
}

/// A `0x02EB` body - for tests and smoke tools.
pub fn options_changed(group: u32, entries: &[(u32, i32)]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(group);
    w.u8(entries.len() as u8);
    for (k, v) in entries {
        w.u32(*k);
        w.i32(*v);
    }
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
    }

    /// The live server's own captures: the HP/MP warning pair, a single key, and a whole group.
    #[test]
    fn the_captured_bodies_parse_to_the_byte() {
        let pair = parse_options_changed(&hex("010000000210000000070000001100000003000000")).unwrap();
        assert_eq!(pair, OptionsChanged { group: 1, entries: vec![(0x10, 7), (0x11, 3)] });
        assert_eq!(key_name(1, 0x10), Some("flHP"));
        assert_eq!(key_name(1, 0x11), Some("flMP"));
        let one = parse_options_changed(&hex("00000000010900000008000000")).unwrap();
        assert_eq!(one, OptionsChanged { group: 0, entries: vec![(9, 8)] });
        assert_eq!(key_name(0, 9), Some("vM1"));
        let mut whole = hex("000000000d");
        for (k, v) in [(0, 15), (1, 0), (2, 15), (3, 0), (4, 1), (5, 15), (6, 0), (7, 15), (8, 0), (9, 19), (10, 0), (11, 15), (12, 0)] {
            whole.extend((k as u32).to_le_bytes());
            whole.extend((v as i32).to_le_bytes());
        }
        assert_eq!(whole.len(), 109, "the archived full-group length");
        assert_eq!(parse_options_changed(&whole).unwrap().entries.len(), 13);
        assert_eq!(parse_options_changed(&whole[..108]), None, "a short body is refused");
        assert_eq!(options_changed(1, &[(0x10, 7), (0x11, 3)]), hex("010000000210000000070000001100000003000000"));
    }

    #[test]
    fn the_tables_end_where_the_client_puts_count() {
        assert_eq!(GAME_OPTION_KEYS.len(), 0x15);
        assert_eq!(SYSTEM_OPTION_KEYS.len(), 0x41);
        assert_eq!(SYSTEM_OPTION_KEYS[0x28], "wrnHP3");
        assert_eq!(SYSTEM_OPTION_KEYS[0x26], "petHP2");
        assert_eq!(key_name(1, 0x15), None, "COUNT is not a key");
        assert_eq!(key_name(0, 0x41), None);
        assert_eq!(key_name(2, 0), None);
        // Every name fits the reader's 8-character limit (`FUN_140427040` refuses longer).
        assert!(GAME_OPTION_KEYS.iter().chain(SYSTEM_OPTION_KEYS.iter()).all(|n| n.len() <= 8));
    }

    #[test]
    fn rows_become_the_two_record_blocks() {
        let (b28, b32) = records(&[(0, 0, 30), (0, 1, 0), (0, 0x16, 2), (0, 0x28, 6), (0, 0x3c, 1), (1, 0x10, 7), (1, 0x11, 3), (1, 99, 1)]);
        assert_eq!(b28, vec![(GAME_OPTIONS_QUEST, "flHP=7;flMP=3".to_string())]);
        assert_eq!(
            b32,
            vec![
                (368, "vBG1=30;mBG1=0".to_string()),
                (369, "fSize2=2".to_string()),
                (370, "wrnHP3=6".to_string()),
                (481, "aMyPet=1".to_string()),
            ]
        );
        assert_eq!(records(&[]), (vec![], vec![]));
    }
}
