//! **`0x03E6` - a status on a mob** (a debuff such as Disorder's, or a mob's own buff).
//!
//! The owner, 2026-10-02: *"skill debuffs do not work on mobs, such as Thief's Disorder"*, and after a
//! test run, *"Still no debuff status shown on mobs hit by disorder."* This server had never sent a
//! mob status at all (`research/first-job-buffs.md` §5). Everything below was read off the client
//! statically on 2026-10-02 - **nothing here has been on a wire yet**; plan step 42 asks.
//!
//! # Where it was found [L]
//!
//! The per-mob dispatcher `FUN_141d32b30` reads a `u32` object id, finds the mob, and jumps on
//! `opcode - 0x3D9` through a 0x75-entry table at `0x141d33448`. Decoding the table: `0x3E6` ->
//! `FUN_141c82270`, which reads **20 raw bytes** (a 160-bit mask) and calls the set decoder
//! `FUN_141cbe8f0`; `0x3E7` -> `FUN_141c82300`, the same 20 bytes into `FUN_141cc09d0`, the reset.
//!
//! # The body of a set [L]
//!
//! ```text
//! u32   mob object id                      (the dispatcher's own read)
//! u32 x 5   mask, MSB-first: status i is word i/32, bit 31 - i%32   (FUN_14046fba0 0x14046fc01)
//! per set status, in index order:
//!     i32 value, i32 reason (the skill id), i16 duration in 500 ms units
//!                                                    (`imul 0x1f4` then `+ now`, 0x14046fc7d)
//! extras, after every block, each only when its bit is set - PDR's is one u32 (0x140471d03)
//! u16   delay                              (FUN_141cbe8f0, 0x141cbecbb - unconditional)
//! u8                                       (0x141cbecc6 - unconditional, stored at mob+0x620)
//! u8                                       (0x141cbf4e2 - gated on FUN_140475070(mask); always sent:
//!                                           an unread trailing byte is harmless, a missing one is not)
//! ```
//!
//! Statuses 0..11 are a different, list-shaped "indie" family (`FUN_14036dec0`, a loop over the
//! mob's own vector) and are never sent here.
//!
//! # Which index is attack and which is defence [D]
//!
//! The fixed blocks start at index 12 (`test [mask], 0x80000`). The reference server (v214, a
//! different version, `research/` labels it a candidate) has `PAD 11, PDR 12, MAD 13, MDR 14`
//! and sends **one extra int for PDR and one for MDR** after the blocks. This client reads one
//! extra `u32` for bit 18 (index **13**) and one for bit 16 (index **15**) - the same two
//! extras, one index later. So **PAD = 12, PDR = 13, MAD = 14, MDR = 15**: derived from the
//! client's own extras lining up with the reference's, not taken from the reference.
//!
//! # No reset is sent
//!
//! The set stores `now + duration` as an absolute expiry (`mob+0x64`), so the client times the
//! status out itself. The reset's decoder branches in ways not yet traced, so it is not built;
//! the server keeps its own expiry for the one number it uses (`world::mobdebuff`).

use crate::packet::PacketWriter;

/// `0x03E6` - put statuses on a mob. **[L]**
pub const MOB_STAT_SET: u16 = 0x03E6;
/// `0x03E7` - take them off. **[L]** for the opcode; not built - see the module docs.
pub const MOB_STAT_RESET: u16 = 0x03E7;

/// Mob attack power. **[D]** - module docs.
pub const PAD: u32 = 12;
/// Mob weapon defence. **[D]**
pub const PDR: u32 = 13;

/// The statuses this builder knows the full shape of: their block, and their extras.
const SUPPORTED: [u32; 2] = [PAD, PDR];

/// One status to put on a mob.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MobStatus {
    /// Which status - [`PAD`] or [`PDR`].
    pub index: u32,
    /// The change, signed: Disorder's `-x` attack and `-y` defence.
    pub value: i32,
    /// The skill that caused it.
    pub reason: u32,
    /// How long, in **milliseconds**. Sent in 500 ms units, rounded up.
    pub duration_ms: u32,
}

/// The mask word and bit for a status index: MSB-first, as the client tests it.
pub fn mask_bit(index: u32) -> (usize, u32) {
    ((index / 32) as usize, 1u32 << (31 - index % 32))
}

/// The `0x03E6` body for `statuses` on mob `object_id`. Statuses this builder does not know the
/// shape of are dropped rather than guessed at - a wrong block misaligns everything after it.
pub fn mob_stat_set(object_id: u32, statuses: &[MobStatus]) -> Vec<u8> {
    let mut known: Vec<MobStatus> = statuses.iter().copied().filter(|s| SUPPORTED.contains(&s.index)).collect();
    known.sort_by_key(|s| s.index);
    known.dedup_by_key(|s| s.index);
    let mut mask = [0u32; 5];
    for s in &known {
        let (word, bit) = mask_bit(s.index);
        mask[word] |= bit;
    }
    let mut w = PacketWriter::new();
    w.u32(object_id);
    for word in mask {
        w.u32(word);
    }
    for s in &known {
        w.i32(s.value);
        w.u32(s.reason);
        w.i16(i16::try_from(s.duration_ms.div_ceil(500)).unwrap_or(i16::MAX));
    }
    // The extras, in the client's order. Only PDR's is reachable from SUPPORTED.
    if known.iter().any(|s| s.index == PDR) {
        w.u32(0);
    }
    w.u16(0); // delay
    w.u8(0);
    w.u8(0); // the gated byte - see the module docs
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The mask is MSB-first**: index 12 is word 0's `0x00080000` and 13 its `0x00040000` - the
    /// two `test dword ptr [r15]` constants at `0x14046fc53` and `0x14046fcab`.
    #[test]
    fn the_mask_bits_are_the_ones_the_client_tests() {
        assert_eq!(mask_bit(PAD), (0, 0x0008_0000));
        assert_eq!(mask_bit(PDR), (0, 0x0004_0000));
        assert_eq!(mask_bit(0), (0, 0x8000_0000));
        assert_eq!(mask_bit(32), (1, 0x8000_0000));
    }

    /// **Disorder level 20, byte for byte**: attack -25 and defence -5 for 30 s from skill 4001000.
    #[test]
    fn disorder_level_twenty_byte_for_byte() {
        let body = mob_stat_set(
            2042,
            &[
                MobStatus { index: PDR, value: -5, reason: 4_001_000, duration_ms: 30_000 },
                MobStatus { index: PAD, value: -25, reason: 4_001_000, duration_ms: 30_000 },
            ],
        );
        let mut want = Vec::new();
        want.extend(2042u32.to_le_bytes());
        want.extend(0x000C_0000u32.to_le_bytes());
        want.extend([0u8; 16]);
        want.extend((-25i32).to_le_bytes()); // PAD first: index order, whatever order given
        want.extend(4_001_000u32.to_le_bytes());
        want.extend(60i16.to_le_bytes()); // 30 s = 60 x 500 ms
        want.extend((-5i32).to_le_bytes());
        want.extend(4_001_000u32.to_le_bytes());
        want.extend(60i16.to_le_bytes());
        want.extend(0u32.to_le_bytes()); // PDR's extra
        want.extend(0u16.to_le_bytes());
        want.extend([0u8, 0u8]);
        assert_eq!(body, want);
    }

    /// Attack alone carries no extra; an unknown status is dropped, not guessed at.
    #[test]
    fn attack_alone_has_no_extra_and_unknown_statuses_are_dropped() {
        let pad = MobStatus { index: PAD, value: -5, reason: 1, duration_ms: 10_000 };
        let alone = mob_stat_set(7, &[pad]);
        assert_eq!(alone.len(), 4 + 20 + 10 + 2 + 2, "no PDR extra");
        let with_unknown = mob_stat_set(7, &[pad, MobStatus { index: 30, ..pad }]);
        assert_eq!(with_unknown, alone);
    }
}
