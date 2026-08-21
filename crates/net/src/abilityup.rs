//! Spending ability points: `0x0138` (one point) and `0x0139` (N points at once).
//!
//! Every address here is read off this client's own listing. The working, the blind spots
//! and the "what I did NOT establish" list are in **`research/ap-allocation.md`**; read that
//! before changing a constant in this file.
//!
//! # NOT WIRED
//!
//! Nothing in `crates/world/` handles either opcode. It compiles, it is tested, and on
//! screen it is identical to not existing - the state `CLAUDE.md` § "Built is not wired"
//! exists to stop being mistaken for working. `research/ap-allocation.md` § "Wire it like
//! this" says how to connect it.
//!
//! # There are TWO requests, not one
//!
//! `STATUS.md` names only `0x0139`, because that is the one a bulk assign put on the wire.
//! The stat window's twelve buttons are six stats x two kinds, and they call two different
//! builders. **[L]**, `FUN_141152da2`:
//!
//! ```text
//! 141152e48  mov edx, 0x40 / mov rcx, rsi / call 0x142d4baa0   "strup"     -> 0x0138
//! 141152e70  mov edx, 0x40 / jmp 141152f44 -> call 0x141153070 "strupall"  -> 0x0139
//! ```
//!
//! | | [`CLIENT_ABILITY_UP`] `0x0138` | [`CLIENT_ABILITY_MASS_UP`] `0x0139` |
//! |---|---|---|
//! | button | the `+` beside a stat | the "auto/amount" button, which asks for a number |
//! | builder | `FUN_142d4baa0` | `FUN_142d4bbc0` |
//! | body | `u32 tick, u32 statMask` | `u32 tick, u32 count, count x (u32 statMask, u32 amount)` |
//! | length | **8** | **8 + 8n**, and every capture so far is n = 1, so 16 |
//! | amount | implicit **1** - there is no amount field | explicit, per entry |
//!
//! **A server that handles only `0x0139` will still look broken to anyone who clicks `+`.**
//!
//! # The one-request-outstanding latch, which is why "nothing happened" three times
//!
//! Both builders open with `FUN_142cc42d0(ctx, 500, 0)` and, after sending, call
//! `FUN_142cc4430(ctx, 1)`. That setter is `[ctx+0x2330] = 1; [ctx+0x2334] = now`, and the
//! check refuses unless `[ctx+0x2330] == 0`. **[L]**
//!
//! So **the client sends no further AP request until something clears that field.** The next
//! click builds nothing, sends nothing and says nothing.
//!
//! Both runs that contain a `0x0139` contain **exactly one each**, and ran on for another
//! 35 s and 49 s. But only one of the two is a clean confirmation - see
//! `research/ap-allocation.md` §5.2. In the other, an **idle-regen `0x007C`** arrived 11 s
//! later and cleared the latch by accident, because `crates/world/src/session/regen.rs`
//! defaults `excl_request_sent` to `true`. **Field entry clears it too** (`FUN_142caa4e0`),
//! and so do inbound `0x00B7`, `0x00F8`, `0x00F9` and `0x018F`, none of which this server
//! sends.
//!
//! That matters for testing rather than for this parser: standing still for ten seconds
//! between two clicks reopens the window on its own, so a second click that "works" after a
//! pause proves nothing about the reply.
//!
//! The thing that clears it *deliberately* is the **first byte of `0x007C`**:
//!
//! ```text
//! 142d547ad  call 0x1406e8ae0        ; u8 bExclRequestSent
//! 142d547b2  test al, al
//! 142d547b4  je   142d547c0          ; zero -> leave the latch alone
//! 142d547b6  xor  edx, edx
//! 142d547bb  call 0x142cc4430        ; [ctx+0x2330] = 0
//! ```
//!
//! [`crate::stats::StatChange`] already defaults `excl_request_sent` to `true`, so **one
//! `0x007C` both applies the stat and unblocks the next click.** A refusal must send one too.

use crate::stats;

/// **`0x0138`** - one ability point into one stat. `u32 tick, u32 statMask`, 8 bytes.
///
/// Built by `FUN_142d4baa0`, whose whole body is: latch check, `FUN_142cbeca0` gate,
/// read AP, `test ax,ax / jle` bail, `mov edx,0x138`, tick, mask, send, set latch. **[L]**
///
/// > **This client sends nothing at all when it believes AP is 0** (`142d4bb3e`). So the
/// > `+` button going dead after the last point is spent is correct behaviour, not a bug -
/// > provided the server told it the new AP.
pub const CLIENT_ABILITY_UP: u16 = 0x0138;

/// **`0x0139`** - N points, as a counted list of `(statMask, amount)` pairs.
///
/// Built by `FUN_142d4bbc0`. The count is `(end - begin) >> 3` of a vector of 8-byte
/// elements, and the loop at `142d4bd00..142d4bd20` writes `[rbx]` then `[rbx+4]` per
/// element. **[L]** - so the body is variable length, and `research/msexe-packet-fields.txt`
/// showing a flat `u32,u32,u32,u32` is that dump flattening a loop into one iteration.
///
/// > **Direction matters.** `0x0138` *outbound* is `UserAvatarModified`, which this server
/// > already sends. Inbound and outbound opcode spaces are separate; these two constants are
/// > client -> server only.
pub const CLIENT_ABILITY_MASS_UP: u16 = 0x0139;

/// Which mask value means which stat.
///
/// **These are the same bit numbers as [`crate::stats::bits`], and that is measured rather
/// than assumed.** `FUN_141153070` - the function behind the amount dialog - switches on the
/// mask and loads one string id per arm:
///
/// ```text
/// 1411530de  cmp r12d, 0x200 / ja 141153125 / je 14115311c   0x200  -> SID_MSCW_STAT_LUK
/// 1411530ec  sub eax, 0x40  / je 141153113                   0x40   -> SID_MSCW_STAT_STR
/// 1411530f1  sub eax, 0x40  / je 14115310a                   0x80   -> SID_MSCW_STAT_DEX
/// 1411530f6  cmp eax, 0x80  / jne bail                       0x100  -> SID_MSCW_STAT_INT
/// 141153125  cmp r12d, 0x800  / je 141153144                 0x800  -> SID_MSCW_STAT_HP
/// 14115312e  cmp r12d, 0x2000 / jne bail                     0x2000 -> SID_MSCW_STAT_MP
/// ```
///
/// The strings are plain ASCII at `0x1433861e0`..`0x143386258`. Six accepted values and a
/// bail for everything else - **enumerated from the switch, not looked up in a list**, which
/// is the difference between this and the two wrong answers `CLAUDE.md` records.
///
/// Against `crate::stats::bits`, whose names came from the v214 reference and were tagged
/// **[I]**: `STR 0x40`, `DEX 0x80`, `INT 0x100`, `LUK 0x200`, `MAX_HP 0x800`,
/// `MAX_MP 0x2000`. **Six for six.** Those six names are now **[L]** from this client.
pub mod stat_bits {
    /// `SID_MSCW_STAT_STR`. Same bit as [`crate::stats::bits::STR`], which lands the value at
    /// `charstat+0x3b`.
    pub const STR: u32 = 0x0000_0040;
    /// `SID_MSCW_STAT_DEX` -> `charstat+0x43`.
    pub const DEX: u32 = 0x0000_0080;
    /// `SID_MSCW_STAT_INT` -> `charstat+0x4b`.
    pub const INT: u32 = 0x0000_0100;
    /// `SID_MSCW_STAT_LUK` -> `charstat+0x53`.
    pub const LUK: u32 = 0x0000_0200;
    /// `SID_MSCW_STAT_HP`, and **the bit is MAX hp**, not current hp.
    ///
    /// This is the unit trap `CLAUDE.md` warns about, one level up: the button says HP, the
    /// mask bit is `0x800` = [`crate::stats::bits::MAX_HP`] -> `charstat+0x67`. Current HP is
    /// bit `0x400` -> `charstat+0x5b`, and **the request cannot name it at all**.
    pub const MAX_HP: u32 = 0x0000_0800;
    /// `SID_MSCW_STAT_MP` -> [`crate::stats::bits::MAX_MP`], `charstat+0x7f`. Same note.
    pub const MAX_MP: u32 = 0x0000_2000;

    /// Every mask `FUN_141153070` accepts. Anything else falls to its `jne bail`.
    pub const ACCEPTED: u32 = STR | DEX | INT | LUK | MAX_HP | MAX_MP;
}

/// One of the six stats an ability point can be spent on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApStat {
    /// `0x40`.
    Str,
    /// `0x80`.
    Dex,
    /// `0x100`.
    Int,
    /// `0x200`.
    Luk,
    /// `0x800`. The button says "HP"; the field is **max** HP.
    MaxHp,
    /// `0x2000`. The button says "MP"; the field is **max** MP.
    MaxMp,
}

impl ApStat {
    /// The mask value on the wire, exactly one bit.
    pub fn mask(self) -> u32 {
        match self {
            ApStat::Str => stat_bits::STR,
            ApStat::Dex => stat_bits::DEX,
            ApStat::Int => stat_bits::INT,
            ApStat::Luk => stat_bits::LUK,
            ApStat::MaxHp => stat_bits::MAX_HP,
            ApStat::MaxMp => stat_bits::MAX_MP,
        }
    }

    /// Read a mask off the wire. `None` for anything the client's own switch would refuse -
    /// including `0`, including two bits at once.
    pub fn from_mask(mask: u32) -> Option<ApStat> {
        match mask {
            stat_bits::STR => Some(ApStat::Str),
            stat_bits::DEX => Some(ApStat::Dex),
            stat_bits::INT => Some(ApStat::Int),
            stat_bits::LUK => Some(ApStat::Luk),
            stat_bits::MAX_HP => Some(ApStat::MaxHp),
            stat_bits::MAX_MP => Some(ApStat::MaxMp),
            _ => None,
        }
    }

    /// The client's own name for it, from the string id in `FUN_141153070`.
    pub fn name(self) -> &'static str {
        match self {
            ApStat::Str => "STR",
            ApStat::Dex => "DEX",
            ApStat::Int => "INT",
            ApStat::Luk => "LUK",
            ApStat::MaxHp => "HP",
            ApStat::MaxMp => "MP",
        }
    }

    /// Whether this stat is stored as a `u16` (`STR`/`DEX`/`INT`/`LUK`) rather than a `u32`.
    ///
    /// The width is not cosmetic: `0x007C` has no length prefix and no resync point, so a
    /// `u32` written where the client reads a `u16` pushes every later value two bytes late.
    /// [`ApStat::apply_to`] is the way not to get this wrong by hand.
    pub fn is_u16(self) -> bool {
        !matches!(self, ApStat::MaxHp | ApStat::MaxMp)
    }

    /// Put this stat's **new total** into a [`crate::stats::StatChange`], in the right field
    /// and therefore at the right width.
    ///
    /// The client *assigns*, it does not add: every arm of `FUN_1402cbb50` calls the
    /// obfuscating setter `FUN_1402f7010(value, ptr)` with the value straight off the wire.
    /// So `new_total` is the character's whole new STR, not the points just spent. **[L]**
    pub fn apply_to(self, change: &mut stats::StatChange, new_total: u32) {
        match self {
            ApStat::Str => change.strength = Some(new_total as u16),
            ApStat::Dex => change.dexterity = Some(new_total as u16),
            ApStat::Int => change.intelligence = Some(new_total as u16),
            ApStat::Luk => change.luck = Some(new_total as u16),
            ApStat::MaxHp => change.max_hp = Some(new_total),
            ApStat::MaxMp => change.max_mp = Some(new_total),
        }
    }
}

/// One `(statMask, amount)` pair.
///
/// The raw mask is kept rather than discarded so a body naming a stat this client's switch
/// refuses can be **logged** instead of silently vanishing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbilityUpEntry {
    /// Exactly as it arrived.
    pub raw_mask: u32,
    /// How many points. Always `1` for [`CLIENT_ABILITY_UP`], which has no amount field.
    pub amount: u32,
}

impl AbilityUpEntry {
    /// `None` if the mask is not one of the six.
    pub fn stat(&self) -> Option<ApStat> {
        ApStat::from_mask(self.raw_mask)
    }
}

/// A parsed `0x0138` or `0x0139`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityUpRequest {
    /// The client's tick. Echoed nowhere; it exists for the client's own rate limiting.
    pub tick: u32,
    /// One entry for `0x0138`; `count` entries for `0x0139`.
    pub entries: Vec<AbilityUpEntry>,
}

impl AbilityUpRequest {
    /// Total points asked for, saturating. This is the number to check against the
    /// character's AP.
    pub fn total_points(&self) -> u32 {
        self.entries.iter().fold(0u32, |a, e| a.saturating_add(e.amount))
    }

    /// True when every entry names one of the six stats and no amount is zero.
    pub fn is_well_formed(&self) -> bool {
        !self.entries.is_empty()
            && self.entries.iter().all(|e| e.stat().is_some() && e.amount > 0)
    }
}

/// A `0x0139` this long or longer is refused rather than allocated for.
///
/// The client's only builder pushes **one** entry (`FUN_141153070` at `141153450`), so
/// anything above six - the number of distinct stats - is already impossible from this
/// client's UI. 64 is slack, not a belief.
pub const MAX_ENTRIES: usize = 64;

/// Body length of [`CLIENT_ABILITY_UP`]: `u32 tick, u32 statMask`.
pub const ABILITY_UP_LEN: usize = 8;

fn u32_at(body: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(body.get(at..at + 4)?.try_into().ok()?))
}

/// Parse a [`CLIENT_ABILITY_UP`] `0x0138` body (opcode already stripped).
///
/// The amount is synthesised as `1`, because the packet has no amount field - the builder
/// writes the tick and the mask and nothing else.
///
/// > **`None` is not a licence to stay silent.** Both requests latch the client. Answer a
/// > body that fails to parse with an empty `0x007C` anyway; see the module docs.
pub fn parse_ability_up(body: &[u8]) -> Option<AbilityUpRequest> {
    if body.len() != ABILITY_UP_LEN {
        return None;
    }
    Some(AbilityUpRequest {
        tick: u32_at(body, 0)?,
        entries: vec![AbilityUpEntry { raw_mask: u32_at(body, 4)?, amount: 1 }],
    })
}

/// Parse a [`CLIENT_ABILITY_MASS_UP`] `0x0139` body (opcode already stripped).
///
/// `u32 tick, u32 count, count x (u32 statMask, u32 amount)`. The length must be exactly
/// `8 + 8 * count`; a body that disagrees with its own count is refused rather than read
/// short.
///
/// > Same warning as [`parse_ability_up`]: refusing to parse is not refusing to answer.
pub fn parse_ability_mass_up(body: &[u8]) -> Option<AbilityUpRequest> {
    let tick = u32_at(body, 0)?;
    let count = u32_at(body, 4)? as usize;
    if count > MAX_ENTRIES {
        return None;
    }
    if body.len() != 8 + count.checked_mul(8)? {
        return None;
    }
    let mut entries = Vec::with_capacity(count);
    for i in 0..count {
        let at = 8 + i * 8;
        entries.push(AbilityUpEntry { raw_mask: u32_at(body, at)?, amount: u32_at(body, at + 4)? });
    }
    Some(AbilityUpRequest { tick, entries })
}

/// Server policy, **[I]**, in one place so it is one edit to change.
///
/// None of this is in the client. `CLAUDE.md`'s rule for exactly this situation: ship the
/// numbers, label them, and put them in one named table rather than scattering them.
pub mod policy {
    /// Max HP gained per ability point spent on "HP".
    ///
    /// **[I]**, from the v214 reference (`UserStatHandler.handleUserAbilityUpRequest` uses
    /// `20`), which is a **different game version** and scored 1 of 8 against a held-out
    /// control. The classic-era value is job- and randomness-dependent and is not in this
    /// client's WZ in any form this pass could find.
    pub const MAX_HP_PER_AP: u32 = 20;
    /// Max MP gained per ability point spent on "MP". Same provenance, same caveat. **[I]**
    pub const MAX_MP_PER_AP: u32 = 20;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both real `0x0139` bodies, from `previous-runs/`. Nothing else in the repo has ever
    /// carried this opcode.
    const REAL: [(&str, u32, u32); 2] = [
        // world-20260821-001440.log 04:13:51.922 - the owner assigned 30 points to STR
        ("dfbdfe0c01000000400000001e000000", 0x0cfe_bddf, 30),
        // world-20260820-233248.log 03:32:12.356 - and 45 the run before
        ("a7a5d80c01000000400000002d000000", 0x0cd8_a5a7, 45),
    ];

    fn body(hex: &str) -> Vec<u8> {
        (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect()
    }

    /// The two captures decode, and the amount is the number the owner typed - which is the only
    /// field in either body that a person chose, so it is the one that makes this a decode
    /// rather than a plausible reading of some bytes.
    #[test]
    fn the_two_real_bodies_decode() {
        for (hex, tick, amount) in REAL {
            let b = body(hex);
            assert_eq!(b.len(), 16, "both captures are 16 bytes");
            let r = parse_ability_mass_up(&b).expect("a real body must parse");
            assert_eq!(r.tick, tick);
            assert_eq!(r.entries.len(), 1, "count = 1 in both");
            assert_eq!(r.entries[0].stat(), Some(ApStat::Str), "0x40 is STR");
            assert_eq!(r.entries[0].amount, amount, "the number the owner typed");
            assert_eq!(r.total_points(), amount);
            assert!(r.is_well_formed());
        }
    }

    /// The count is a **pair count**, and the loop at `142d4bd00` means the body can be
    /// longer than the 16 bytes every capture shows. Nothing in this client builds one, but a
    /// parser that assumed 16 would read a longer body wrong rather than refuse it.
    #[test]
    fn the_count_is_a_pair_count_and_the_body_can_be_longer() {
        let mut b = Vec::new();
        b.extend_from_slice(&1u32.to_le_bytes()); // tick
        b.extend_from_slice(&2u32.to_le_bytes()); // count
        b.extend_from_slice(&stat_bits::STR.to_le_bytes());
        b.extend_from_slice(&3u32.to_le_bytes());
        b.extend_from_slice(&stat_bits::MAX_HP.to_le_bytes());
        b.extend_from_slice(&4u32.to_le_bytes());
        assert_eq!(b.len(), 8 + 8 * 2);

        let r = parse_ability_mass_up(&b).expect("two entries must parse");
        assert_eq!(r.entries.len(), 2);
        assert_eq!(r.entries[0].stat(), Some(ApStat::Str));
        assert_eq!(r.entries[1].stat(), Some(ApStat::MaxHp));
        assert_eq!(r.total_points(), 7);
    }

    /// A body that disagrees with its own count is refused rather than read short. The
    /// handler still has to answer it - that is the module doc's job, not this test's.
    #[test]
    fn a_body_that_lies_about_its_count_is_refused() {
        let mut b = body(REAL[0].0);
        b[4] = 2; // claim two entries, still carry one
        assert!(parse_ability_mass_up(&b).is_none());

        let short = &body(REAL[0].0)[..15];
        assert!(parse_ability_mass_up(short).is_none());
        assert!(parse_ability_mass_up(&[]).is_none());

        let mut absurd = Vec::new();
        absurd.extend_from_slice(&0u32.to_le_bytes());
        absurd.extend_from_slice(&u32::MAX.to_le_bytes());
        assert!(parse_ability_mass_up(&absurd).is_none(), "no allocation from a wire u32");
    }

    /// `0x0138` has **no amount field**. Synthesised from `FUN_142d4baa0`'s listing - tick,
    /// mask, send - because no capture in this repo contains one; that is said out loud here
    /// rather than left for someone to assume.
    #[test]
    fn the_single_point_body_is_eight_bytes_and_means_one_point() {
        let mut b = Vec::new();
        b.extend_from_slice(&0x0cfe_bddfu32.to_le_bytes());
        b.extend_from_slice(&stat_bits::DEX.to_le_bytes());
        assert_eq!(b.len(), ABILITY_UP_LEN);

        let r = parse_ability_up(&b).expect("the shape the builder writes");
        assert_eq!(r.tick, 0x0cfe_bddf);
        assert_eq!(r.entries.len(), 1);
        assert_eq!(r.entries[0].stat(), Some(ApStat::Dex));
        assert_eq!(r.entries[0].amount, 1, "implicit, there is no field for it");

        // and it is not the mass-up shape
        assert!(parse_ability_up(&body(REAL[0].0)).is_none());
        assert!(parse_ability_mass_up(&b).is_none());
    }

    /// The six masks the client's own switch accepts, and nothing else.
    #[test]
    fn exactly_six_masks_are_accepted() {
        let all = [
            (stat_bits::STR, ApStat::Str, "STR"),
            (stat_bits::DEX, ApStat::Dex, "DEX"),
            (stat_bits::INT, ApStat::Int, "INT"),
            (stat_bits::LUK, ApStat::Luk, "LUK"),
            (stat_bits::MAX_HP, ApStat::MaxHp, "HP"),
            (stat_bits::MAX_MP, ApStat::MaxMp, "MP"),
        ];
        for (mask, stat, name) in all {
            assert_eq!(ApStat::from_mask(mask), Some(stat));
            assert_eq!(stat.mask(), mask);
            assert_eq!(stat.name(), name);
            assert_eq!(mask.count_ones(), 1, "one bit each");
        }
        assert_eq!(all.iter().fold(0, |a, (m, _, _)| a | m), stat_bits::ACCEPTED);

        // everything the switch bails on
        for bad in [0u32, 0x400, 0x1000, 0x4000, 0x10, 0x20, 0xC0, u32::MAX] {
            assert_eq!(ApStat::from_mask(bad), None, "mask {bad:#x} is not one of the six");
        }
    }

    /// The request mask and the `0x007C` mask are the **same bit numbers**. This is the
    /// hypothesis the goal asked to test rather than assume, and it holds six for six.
    ///
    /// The two that would be easy to get wrong are HP and MP: the button says HP, the bit is
    /// `MAX_HP`. Current HP (`0x400`) and current MP (`0x1000`) cannot be named by a request
    /// at all.
    #[test]
    fn the_request_mask_is_the_stat_changed_mask() {
        assert_eq!(stat_bits::STR, stats::bits::STR);
        assert_eq!(stat_bits::DEX, stats::bits::DEX);
        assert_eq!(stat_bits::INT, stats::bits::INT);
        assert_eq!(stat_bits::LUK, stats::bits::LUK);
        assert_eq!(stat_bits::MAX_HP, stats::bits::MAX_HP);
        assert_eq!(stat_bits::MAX_MP, stats::bits::MAX_MP);

        assert_ne!(stat_bits::MAX_HP, stats::bits::HP, "the button lies about which HP");
        assert_ne!(stat_bits::MAX_MP, stats::bits::MP);
        assert_eq!(stat_bits::ACCEPTED & stats::bits::HP, 0, "current hp is unreachable");
        assert_eq!(stat_bits::ACCEPTED & stats::bits::MP, 0);
        assert_eq!(stat_bits::ACCEPTED & !stats::bits::DECODED, 0, "all six are decoded bits");
    }

    /// The confirm goes out in the right field, and therefore at the right width. A `u32`
    /// where the client reads a `u16` desynchronises everything after it in a body with no
    /// resync point.
    #[test]
    fn apply_to_puts_the_value_in_the_right_field() {
        let mut c = stats::StatChange::new();
        ApStat::Str.apply_to(&mut c, 34);
        c.ap = Some(11);
        assert_eq!(c.strength, Some(34));
        assert_eq!(c.mask(), stats::bits::STR | stats::bits::AP);
        // 3 head + 4 mask + 2 str + 2 ap + 2 trailers
        assert_eq!(c.build().len(), 13);

        let mut c = stats::StatChange::new();
        ApStat::MaxHp.apply_to(&mut c, 70);
        assert_eq!(c.max_hp, Some(70));
        assert_eq!(c.hp, None, "spending AP on HP must not be written to CURRENT hp");
        assert_eq!(c.mask(), stats::bits::MAX_HP);

        for s in [ApStat::Str, ApStat::Dex, ApStat::Int, ApStat::Luk] {
            assert!(s.is_u16());
            let mut c = stats::StatChange::new();
            s.apply_to(&mut c, 5);
            assert_eq!(c.mask(), s.mask(), "one stat sets exactly its own bit");
        }
        for s in [ApStat::MaxHp, ApStat::MaxMp] {
            assert!(!s.is_u16());
            let mut c = stats::StatChange::new();
            s.apply_to(&mut c, 5);
            assert_eq!(c.mask(), s.mask());
        }
    }

    /// The refusal is a real packet. An empty `0x007C` is 9 bytes, changes nothing, and its
    /// first byte is what clears `[ctx+0x2330]` - without it the stat window is dead for the
    /// rest of the session.
    #[test]
    fn the_refusal_reply_still_clears_the_latch() {
        let refusal = stats::StatChange::new();
        assert!(refusal.excl_request_sent, "byte 0 non-zero is the whole point");
        assert_eq!(refusal.mask(), 0, "and it announces no stat");
        assert_eq!(refusal.build(), vec![1, 0, 1, 0, 0, 0, 0, 0, 0]);
    }

    /// A zero amount, or a stat this client cannot name, is not well formed - but it is still
    /// a body that must be answered.
    #[test]
    fn well_formed_rejects_zero_amounts_and_unknown_stats() {
        let mut b = body(REAL[0].0);
        b[12] = 0; // amount 0
        assert!(!parse_ability_mass_up(&b).unwrap().is_well_formed());

        let mut b = body(REAL[0].0);
        b[8] = 0x10; // mask 0x10 = the LEVEL bit, which the switch bails on
        let r = parse_ability_mass_up(&b).unwrap();
        assert!(!r.is_well_formed());
        assert_eq!(r.entries[0].raw_mask, 0x10, "kept, so it can be logged");
        assert_eq!(r.entries[0].stat(), None);
    }

    #[test]
    fn the_opcodes_are_the_ones_read_off_the_two_builders() {
        assert_eq!(CLIENT_ABILITY_UP, 0x0138);
        assert_eq!(CLIENT_ABILITY_MASS_UP, 0x0139);
        // one slot below the skill-up request, which is what put 0x0139 in the frame
        assert_eq!(CLIENT_ABILITY_MASS_UP + 2, 0x013B);
    }
}
