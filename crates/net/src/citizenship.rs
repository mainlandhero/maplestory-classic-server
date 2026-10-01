//! Citizenship on the wire: the quest "ex" record that carries it, the two `0x0089` lines that
//! move it, and the contract window.
//!
//! **There is no citizenship packet.** The client keeps a character's citizenship as
//! `key=value;` pairs in the ex record of a hidden quest, [`CITIZENSHIP_QUEST`] - and the
//! Community Board's daily and weekly postings travel the same way, in quests
//! `510001..510004`. `research/citizenship-2026-09-27.md` §5 has every reader.
//!
//! Labels as in the research note: **[L]** read off this client.

use crate::packet::{PacketReader, PacketWriter};

/// The hidden quest whose ex record IS the citizenship. `st<town>` `gr<town>` `ct<town>`,
/// town 1 Henesys, 2 Kerning City. **[L]** `FUN_1402C8870` builds the keys, `FUN_140729FB0`
/// reads them.
pub const CITIZENSHIP_QUEST: u32 = 510_000;

/// `0x0089` sub-case **13**: replace one quest's ex record - `u32 questId, str value`.
/// `FUN_142D5C7B0` -> `FUN_142D5AA10` -> `FUN_1402E19A0`, which stores into the character's
/// `+0x12BB` map and **replaces the whole string**. **[L]**
///
/// Like sub-case 1, this is not for sending with or just before a `SetField`: the record's
/// block #28 carries the same map at field entry.
pub const MESSAGE_QUEST_EX: u8 = 13;

/// `0x0089` sub-case **35**: *"You have gained %s Contribution (+%d)"* - `u8 town, u32 amount`.
/// `FUN_142D975A0`, string `0x17CD`; a town outside 1..2 is dropped by the client. **[L]**
pub const MESSAGE_CONTRIBUTION: u8 = 35;

/// UserEffect 83, `Effect/BasicEff.img/CitizenshipGet`. **[L]** (`net::questeffect`'s table).
pub const EFFECT_CITIZENSHIP_GET: u8 = 83;
/// UserEffect 84, `Effect/BasicEff.img/CitizenshipGradeUp`. **[L]**
pub const EFFECT_CITIZENSHIP_GRADE_UP: u8 = 84;

/// The `0x0089` body for sub-case [`MESSAGE_QUEST_EX`].
pub fn quest_ex_record(quest_id: u32, value: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(MESSAGE_QUEST_EX);
    w.u32(quest_id);
    w.str(value);
    w.into_vec()
}

/// The `0x0089` body for sub-case [`MESSAGE_CONTRIBUTION`].
pub fn contribution_gained(town: u8, amount: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(MESSAGE_CONTRIBUTION);
    w.u8(town);
    w.u32(amount);
    w.into_vec()
}

/// The character record's block **#28**, behind presence byte [`PRESENCE_QUEST_EX`]:
/// `u16 count, count x (u32 questId, str value)`, stored by `FUN_1402E19A0` into `+0x12BB` -
/// the same map sub-case 13 writes. **[L]** (`research/charrecord-presence-map.md`, block #28,
/// three reads.)
pub fn quest_ex_block(entries: &[(u32, String)]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u16(entries.len().min(usize::from(u16::MAX)) as u16);
    for (quest_id, value) in entries.iter().take(usize::from(u16::MAX)) {
        w.u32(*quest_id);
        w.str(value);
    }
    w.into_vec()
}

/// The presence byte that gates block #28. **[L]** gate call `0x140308A36`, after the
/// completed-quests block (presence 14) and before the record's final ungated `u8`.
pub const PRESENCE_QUEST_EX: usize = 16;

// ---------------------------------------------------------------------------------------
// The contract window - ScriptMessage types 0x42..0x46
// ---------------------------------------------------------------------------------------

/// `0x055B` message types for the five contract windows. One window class for all five
/// (`FUN_1410DCC60`), laid out from `UI/Citizenship.img/contract`. **[L]**
pub const CONTRACT_OATH: u8 = 0x42;
pub const CONTRACT_TRANSFER: u8 = 0x43;
pub const CONTRACT_REACTIVATION: u8 = 0x44;
pub const CONTRACT_RENUNCIATION: u8 = 0x45;
pub const CONTRACT_GRADE_UPDATE: u8 = 0x46;
/// The second answer an accepted contract sends, when its stamp animation ends.
pub const CONTRACT_STAMP_DONE: u8 = 0x47;

/// One contract window, with the fields its handler reads after the script head. **[L]**
///
/// | type | handler | body |
/// |---|---|---|
/// | `0x42` | `FUN_141F765D0` | `u8 town, u32 npc` |
/// | `0x43` | `FUN_141F76920` | `u8 newTown, u8 oldTown, u32 npc` |
/// | `0x44` | `FUN_141F76C80` | `u8 town, u8 grade, u32 fee, u32 npc` |
/// | `0x45` | `FUN_141F76FF0` | `u8 town, u8 grade, u32 npc` |
/// | `0x46` | `FUN_141F77350` | `u8 town, u8 grade-1, u32 0, u32 npc` |
///
/// `npc` is a **template** looked up in the field's NPC pool for its name; one that is not
/// standing on the map draws an empty name rather than faulting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Contract {
    Oath { town: u8 },
    Transfer { new_town: u8, old_town: u8 },
    Reactivation { town: u8, grade: u8, fee: u32 },
    Renunciation { town: u8, grade: u8 },
    /// `grade` is the NEW grade, 1..=10. The window draws `+0x2AC + 1`'s name and uses
    /// `+0x2AC` for the badge, so the wire carries `grade - 1` **[D]**.
    GradeUpdate { town: u8, grade: u8 },
}

impl Contract {
    pub fn message_type(self) -> u8 {
        match self {
            Contract::Oath { .. } => CONTRACT_OATH,
            Contract::Transfer { .. } => CONTRACT_TRANSFER,
            Contract::Reactivation { .. } => CONTRACT_REACTIVATION,
            Contract::Renunciation { .. } => CONTRACT_RENUNCIATION,
            Contract::GradeUpdate { .. } => CONTRACT_GRADE_UPDATE,
        }
    }
}

/// The `0x055B` body for a contract window, spoken by `npc_template`.
///
/// The head is the ordinary script head (`net::script::Say`'s), `handle` in field 1 - these
/// five types keep it and it comes back in both answers.
pub fn contract(handle: u32, npc_template: u32, which: Contract) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(handle); //                head field 1, echoed in the 0x00F3 answers
    w.u8(0); //                      head field 2
    w.u32(npc_template); //          the speaker
    w.u8(0); //                      hasOverride
    w.u8(which.message_type());
    w.u16(0); //                     flags
    w.u8(0); //                      head field 8
    match which {
        Contract::Oath { town } => {
            w.u8(town);
        }
        Contract::Transfer { new_town, old_town } => {
            w.u8(new_town);
            w.u8(old_town);
        }
        Contract::Reactivation { town, grade, fee } => {
            w.u8(town);
            w.u8(grade);
            w.u32(fee);
        }
        Contract::Renunciation { town, grade } => {
            w.u8(town);
            w.u8(grade);
        }
        Contract::GradeUpdate { town, grade } => {
            w.u8(town);
            w.u8(grade.saturating_sub(1));
            w.u32(0); //             +0x2B0, nothing found reads it
        }
    }
    w.u32(npc_template); //          the NPC whose name the window draws
    w.into_vec()
}

/// What came back on `0x00F3` for a contract window. **[L]** `FUN_1410DE8C0`, `FUN_1410DE9A0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractAnswer {
    /// OK (button 1000): `handle, type, 1`. Both buttons disable and the window **waits for
    /// the server**: a `0x055B` force-close (`net::script::script_force_close`) with result 1
    /// plays the stamp and leads to [`ContractAnswer::StampDone`]; result 0 closes it with no
    /// stamp. Without either it stays open forever - `FUN_1410DEA80`, whose only caller is the
    /// `0x055B` reader's force-close branch at `0x141F6F486`, is what starts the stamp. **[L]**
    Accepted { message_type: u8 },
    /// Cancel (1001) or Esc: `handle, type, 0`. The window closes.
    Cancelled { message_type: u8 },
    /// The stamp finished, ~2 s after OK: `handle, 0x47, type`. The window closes.
    StampDone { message_type: u8 },
}

/// Decode a contract answer. `None` for anything else - including every Say, menu and avatar
/// answer - so the caller falls through to the other parsers. It runs **before**
/// `net::script::parse_script_reply`, which reads a Say shape and would drop these 6 bytes.
pub fn parse_contract_answer(body: &[u8]) -> Option<ContractAnswer> {
    if body.len() != 6 {
        return None;
    }
    let mut r = PacketReader::new(body);
    let _handle = r.u32().ok()?;
    let first = r.u8().ok()?;
    let second = r.u8().ok()?;
    let is_contract = |t: u8| (CONTRACT_OATH..=CONTRACT_GRADE_UPDATE).contains(&t);
    if first == CONTRACT_STAMP_DONE && is_contract(second) {
        return Some(ContractAnswer::StampDone { message_type: second });
    }
    if !is_contract(first) {
        return None;
    }
    match second {
        1 => Some(ContractAnswer::Accepted { message_type: first }),
        0 => Some(ContractAnswer::Cancelled { message_type: first }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_ex_record_line_is_sub_case_13_with_the_whole_string() {
        let b = quest_ex_record(CITIZENSHIP_QUEST, "st1=1;gr1=1;ct1=0");
        assert_eq!(b[0], 13);
        assert_eq!(&b[1..5], &510_000u32.to_le_bytes());
        assert_eq!(&b[5..7], &17u16.to_le_bytes());
        assert_eq!(&b[7..], b"st1=1;gr1=1;ct1=0");
    }

    #[test]
    fn the_contribution_line_is_sub_case_35_town_then_amount() {
        assert_eq!(contribution_gained(2, 150), vec![35, 2, 150, 0, 0, 0]);
    }

    #[test]
    fn block_28_is_a_count_then_id_string_pairs() {
        let b = quest_ex_block(&[(510_000, "st1=1".into()), (510_001, "q1_d=506005|506006".into())]);
        assert_eq!(&b[0..2], &2u16.to_le_bytes());
        assert_eq!(&b[2..6], &510_000u32.to_le_bytes());
        assert_eq!(&b[6..8], &5u16.to_le_bytes());
        assert_eq!(&b[8..13], b"st1=1");
        assert_eq!(&b[13..17], &510_001u32.to_le_bytes());
        assert_eq!(quest_ex_block(&[]), vec![0, 0]);
    }

    /// The head is the Say head with the type at offset 10, and every variant's body is the
    /// handler's reads in order, ending with the NPC.
    #[test]
    fn each_contract_is_the_script_head_then_its_handlers_reads() {
        let head = |b: &[u8], t: u8| {
            assert_eq!(&b[0..4], &7u32.to_le_bytes(), "handle");
            assert_eq!(&b[5..9], &229u32.to_le_bytes(), "speaker");
            assert_eq!(b[9], 0);
            assert_eq!(b[10], t);
            b[14..].to_vec()
        };
        let npc = 229u32.to_le_bytes();
        let oath = contract(7, 229, Contract::Oath { town: 1 });
        assert_eq!(head(&oath, 0x42), [&[1u8][..], &npc].concat());
        let t = contract(7, 229, Contract::Transfer { new_town: 1, old_town: 2 });
        assert_eq!(head(&t, 0x43), [&[1u8, 2][..], &npc].concat());
        let r = contract(7, 229, Contract::Reactivation { town: 1, grade: 3, fee: 50_000 });
        assert_eq!(head(&r, 0x44), [&[1u8, 3][..], &50_000u32.to_le_bytes(), &npc].concat());
        let x = contract(7, 229, Contract::Renunciation { town: 1, grade: 3 });
        assert_eq!(head(&x, 0x45), [&[1u8, 3][..], &npc].concat());
        let g = contract(7, 229, Contract::GradeUpdate { town: 1, grade: 2 });
        assert_eq!(head(&g, 0x46), [&[1u8, 1][..], &[0u8; 4], &npc].concat(), "grade - 1");
    }

    #[test]
    fn the_three_contract_answers_decode_and_nothing_else_does() {
        let ans = |a: u8, b: u8| [&0u32.to_le_bytes()[..], &[a, b]].concat();
        assert_eq!(parse_contract_answer(&ans(0x42, 1)), Some(ContractAnswer::Accepted { message_type: 0x42 }));
        assert_eq!(parse_contract_answer(&ans(0x45, 0)), Some(ContractAnswer::Cancelled { message_type: 0x45 }));
        assert_eq!(parse_contract_answer(&ans(0x47, 0x42)), Some(ContractAnswer::StampDone { message_type: 0x42 }));
        // A menu cancel is also six bytes: `u32 0, u8 6, u8 0`. It must not read as a contract.
        assert_eq!(parse_contract_answer(&ans(6, 0)), None);
        assert_eq!(parse_contract_answer(&ans(0x47, 6)), None, "a force-close answer for a non-contract");
        assert_eq!(parse_contract_answer(&ans(0x42, 2)), None);
        assert_eq!(parse_contract_answer(&[&ans(0x42, 1)[..], &[0]].concat()), None, "exactly six bytes");
    }
}
