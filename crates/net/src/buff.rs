//! Temporary stats: the packet that grants a buff, the one that clears it, and the request
//! that asks for one.
//!
//! The owner, 2026-08-21 and again on 2026-08-22: *"Nimble Feet still does not give me a buff
//! despite me activating the skill."* The request had been arriving all along -
//! `world.log` of the 12:59 run logs one `0x013C`, 51 bytes, `skillId 1002 level 3` - and
//! nothing answered it. `research/buffs.md` has the working; this is the wire.
//!
//! # `0x007D` outbound is NOT `0x007D` inbound, and the names here say so
//!
//! `crate::opcode::CLIENT_MIGRATION_HELLO` is `0x007D` **from** the client. This module's
//! [`TEMPORARY_STAT_SET`] is `0x007D` **to** it. Opcode spaces are per-direction and the
//! channel dispatcher has no case that could answer the client's `0x007D`, so both are true
//! at once - but a single shared constant would be a trap, so neither name mentions the
//! other's job. `research/buffs.md` §5.1 flagged this before either existed.
//!
//! # Three sizes, and each of them is measured
//!
//! * the mask is **124 bytes**, stated three independent ways in the client: a `mov r8d,0x7c`
//!   raw read, a constructor that zeroes `0x7C`, and a decoder that loops 30 `u32` words plus
//!   one more. **[L]**
//! * the bit order inside each `u32` is **big-endian**: bit `i` is
//!   `words[i >> 5] |= 1 << (31 - (i & 31))`. **[L]**
//! * the duration is **milliseconds**, and `Skill.wz`'s `time` is **seconds**. Three
//!   readings agree, one of them a wire measurement: two client packets 1.091 s apart carried
//!   tick values 1080 apart, and the only clock this PE imports is `timeGetTime`. Sending
//!   `30` where `30000` belongs would put the expiry 30 ms out and the icon would flash and
//!   vanish - which is a **distinguishable** outcome, so the test plan asks for it by name.
//!
//! This is the third bug in a month that was a correct number in the wrong unit, which is why
//! [`TemporaryStat::duration_ms`] is spelled with its unit in the field name.
//!
//! # The one thing that is a guess, and how the run decides it
//!
//! The per-stat value is either an `i16` or a `u32`, and the client decides **per packet** by
//! ANDing the mask with a 124-byte constant at `0x143ac37e0`. That address is in the
//! Themida-packed `.data`, so its bytes at rest are not its runtime content and no static
//! read can settle it. `i16` is the best available guess and it is **[I]**.
//!
//! The tail is sent as zero bytes for exactly this reason: every field in it is fixed-width,
//! so **all four parses read the same zeros**. Do not put a non-zero byte in the tail until
//! the width is settled.
//!
//! # The first attempt crashed the client, and the tail is why
//!
//! 18 bytes of tail was sized from the documented layout and the client threw a C++ exception
//! reading four bytes it did not have. [`TAIL_LEN`] carries the whole measurement, including
//! the seven bytes that four separate instruments say should have been there and were not.

use crate::packet::PacketWriter;

/// `0x007D` **outbound** - TemporaryStatSet. See the module docs on the direction collision.
pub const TEMPORARY_STAT_SET: u16 = 0x007D;

/// `0x007E` outbound - TemporaryStatReset. `u8, u8, u8, raw[124]`, **and then more**.
///
/// See [`temporary_stat_reset`]: the documented 127 bytes are not enough, and the client says
/// so by throwing.
pub const TEMPORARY_STAT_RESET: u16 = 0x007E;

/// `0x013C` - the client asking to use a skill. `u32 skillId, u32 level`, then a tail.
pub const CLIENT_SKILL_USE: u16 = 0x013C;

/// `0x013D` - the send-counter census. **Never answered**; see `research/buffs.md` §2.5.
pub const CLIENT_SKILL_CENSUS: u16 = 0x013D;

/// The character-temporary-stat bit for movement speed.
///
/// **[L]**, from `FUN_1429755a0` - the client's own guard that refuses a second Nimble Feet
/// while Speed is already held - which reads this bit and raises string `0x14DA`.
pub const CTS_SPEED: u32 = 92;

/// The bit mask is 124 bytes: 31 little-endian `u32` words.
pub const MASK_LEN: usize = 124;

/// Highest bit the client will test. `FUN_1402bf6d0` compares `idx >= 0x3e0` and returns
/// false, so 992 is out of range and 991 is the last legal bit. **[L]**
pub const MAX_CTS_BIT: u32 = 992;

/// Zero bytes after the per-stat list.
///
/// # It was 18, the client threw, and 18 should have been enough
///
/// 2026-08-22, the owner: *"Nimble Feet crashed the client."* Exit code `0xE06D7363` - an
/// unhandled **C++ exception**, not an access violation - and the hook's throw log names the
/// frame: `0x142d56911`, the instruction after the `call` at `0x142d5690c`, which is the
/// `u32` read near the end of `FUN_142d563d0`'s tail. The primitive it called says exactly
/// why it threw:
///
/// ```asm
/// 1406e8c32  mov  edi, [rcx+0x18]     ; length
/// 1406e8c35  sub  edi, [rcx+0x24]     ; minus position = bytes remaining
/// 1406e8c79  cmp  edi, 4
/// 1406e8c7c  jb   1406e8c91           ; fewer than four left -> raise
/// 1406e8cb1  int3                     ; the return address on the throw stack
/// ```
///
/// **[L]**. So the body was too short. That much is measured, and it also proves something
/// that was only [D] before: `0x007D` really is TemporaryStatSet, because the throw happened
/// inside its handler.
///
/// # What does not add up, said plainly rather than smoothed over
///
/// Four instruments were run on the layout and they agree with each other:
///
/// * `tools/reads.py` at depth 4 on `FUN_142d563d0` - the whole tail is `u16, 6x u8, one
///   conditional u8, u32, u8`, and **nothing** reads the packet before the mask;
/// * the listing of the raw primitive `0x1406e9170` - it copies exactly `r8d` bytes with no
///   length prefix, and the call site passes `0x7c`, so the mask is **124**;
/// * the listing of bit 92's own decoder block at `0x140a17e3b` - 87 lines, and the `u32` at
///   `+0x34` and the `u16` at `+0x52` are the **two arms of one `if`**, so a stat is 10 bytes
///   or 12, never both;
/// * an enumeration of all 476 bit tests against `0x1402bf6d0` - bit 92 is tested **once**,
///   so one bit sets one block.
///
/// That totals **at most 145** bytes consumed before the `u32`, out of the 152 sent - seven
/// to spare. The client says otherwise. **Something between the handler's entry and
/// `0x142d5690c` consumes bytes that none of those four can see**, and re-running any of them
/// is not a second opinion.
///
/// # So this is slack, not a computed length
///
/// 64 zero bytes: 46 more than the worst layout any of the evidence supports, which absorbs
/// a hidden consumer several times over. Every tail field is fixed width and zero, so a
/// longer tail cannot change what any of them decode - and the reader's only length test is
/// "fewer than N remaining", with no check that the body was fully consumed.
///
/// **It is honestly a guess about the size of an unknown, and the next run narrows it.** If
/// the buff works at 64, the true requirement is somewhere in 153..198 and can be bisected
/// later; if the client dies *differently* - a complaint about a long packet rather than a
/// silent death - then this reader does check for leftovers and the number has to be exact.
pub const TAIL_LEN: usize = 64;

/// One temporary stat being granted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemporaryStat {
    /// The CTS bit, e.g. [`CTS_SPEED`].
    pub bit: u32,
    /// The amount. **[I] `i16`** - see the module docs; the run decides it.
    pub value: i16,
    /// What granted it: the skill id, or an item id.
    pub reason: u32,
    /// **Milliseconds.** `Skill.wz`'s `time` is seconds, so multiply by 1000.
    pub duration_ms: u32,
}

/// Build the 124-byte mask with `bits` set.
///
/// The bit order is big-endian *inside* each little-endian word, which is the part that looks
/// wrong and is not: bit 92 lands in word 2 as `1 << (31 - 28)` = `0x00000008`, so bytes
/// 8..11 read `08 00 00 00`. Anything that "tidies" this to `1 << (i & 31)` sets a different
/// stat, and the client would grant it without complaint.
///
/// Bits at or above [`MAX_CTS_BIT`] are dropped rather than wrapping: the client's own test
/// refuses them, and a silent wrap would set an unrelated stat.
pub fn stat_mask(bits: &[u32]) -> [u8; MASK_LEN] {
    let mut words = [0u32; MASK_LEN / 4];
    for &b in bits {
        if b >= MAX_CTS_BIT {
            continue;
        }
        words[(b >> 5) as usize] |= 1u32 << (31 - (b & 31));
    }
    let mut out = [0u8; MASK_LEN];
    for (i, w) in words.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
    }
    out
}

/// `0x007D` - grant these stats.
///
/// **The per-stat entries go in ascending bit order**, because that is the order the decoder
/// walks the mask in: it loops the words low to high and, inside each, tests bit 31 down to
/// bit 0. Sorting here rather than trusting the caller means a two-stat packet cannot be
/// mis-paired - and a mis-pairing would not error, it would give the wrong stat the wrong
/// number for the wrong length of time.
pub fn temporary_stat_set(stats: &[TemporaryStat]) -> Vec<u8> {
    temporary_stat_set_with_tail(stats, TAIL_LEN)
}

/// [`temporary_stat_set`] with the tail length chosen by the caller.
///
/// # This exists to turn one guess per launch into many probes per launch
///
/// [`TAIL_LEN`] is slack around a number nobody has been able to derive statically, and the
/// only way to narrow it is to send one and see. A rebuild per attempt costs the owner a manual
/// launch; `!buff <skill> <level> <tail>` costs a chat line. So the length is a parameter,
/// the skill keypress uses the safe default, and a session that survives the default can
/// bisect downwards until the client throws again.
///
/// The failure is not gentle - the client raises an unhandled C++ exception and the process
/// ends - so a probe that goes too low ends the session. Bisect **downwards from working**,
/// not upwards from broken.
pub fn temporary_stat_set_with_tail(stats: &[TemporaryStat], tail: usize) -> Vec<u8> {
    let mut ordered: Vec<TemporaryStat> =
        stats.iter().copied().filter(|s| s.bit < MAX_CTS_BIT).collect();
    ordered.sort_by_key(|s| s.bit);
    ordered.dedup_by_key(|s| s.bit);

    let mut w = PacketWriter::new();
    let bits: Vec<u32> = ordered.iter().map(|s| s.bit).collect();
    w.bytes(&stat_mask(&bits));
    for s in &ordered {
        w.u16(s.value as u16);
        w.u32(s.reason);
        w.u32(s.duration_ms);
    }
    w.bytes(&vec![0u8; tail]);
    w.into_vec()
}

/// Body length of a [`temporary_stat_set`] with `n` stats: mask, `n` x 10, tail.
pub const fn temporary_stat_set_len(n: usize) -> usize {
    MASK_LEN + n * 10 + TAIL_LEN
}

/// The shortest tail known to be too short: the client threw with **18**.
///
/// Kept as a constant so the probe path can refuse to go back below a length that has
/// already killed a client once. Costing the owner a launch to re-learn something the log
/// already says is exactly what this repo's rules exist to prevent.
pub const TAIL_KNOWN_TOO_SHORT: usize = 18;

/// `0x007E` - clear these stats.
///
/// `u8, u8, u8`, the mask, then a zero tail.
///
/// # 127 bytes killed the client, one handler after `0x007D` did
///
/// 2026-08-22, the owner: *"The buff works, but after the buff expired, the client crashed
/// again."* The same fault as the grant packet and the same shape of cause, thirty seconds
/// later - and this time the arithmetic comes out **exactly**, with nothing unexplained.
///
/// `research/buffs.md` §7.1 gives the body as `u8, u8, u8, raw[124]` = 127, on the reading
/// that `0x007E`'s conditional extras are gated on bits 27 and 411 only. `tools/reads.py` at
/// depth 4 on `FUN_142d56f80` finds **three reads after the mask** that the §7.1 list does
/// not mention: **[L]**
///
/// ```text
/// 0x142d56fc3  u8
/// 0x142d56fd1  u8
/// 0x142d56fdf  u8
/// 0x142d57040  raw          <- 124, the mask
/// 0x142d571c3  u32 via helper, gated
/// 0x142d57322  u8           <- THREW HERE
/// 0x142d57360  u8
/// ```
///
/// The throw stack names `0x142d57327`, the instruction after the `call` at `0x142d57322`,
/// and `0x1406e8b71`, which is the raise path of the **u8** primitive - `cmp edi, 1 / jb`.
/// **[L]** So the client consumed all 127 and then wanted one more byte. That also says the
/// gated `u32` did *not* fire: had it, the throw would have been at `0x142d571c3`.
///
/// Minimum is therefore `3 + 124 + 1 + 1` = **129**, or **133** if that `u32` ever fires.
///
/// # Why the tail is [`TAIL_LEN`] anyway
///
/// Because the grant packet taught that this enumeration can still be short, and because
/// **198 bytes of `0x007D` were accepted without complaint** - the first hard evidence that
/// this client ignores trailing bytes rather than checking that a body was fully consumed.
/// Padding is now supported by a measurement instead of a hope. `research/buffs-underflow.md`
pub fn temporary_stat_reset(bits: &[u32]) -> Vec<u8> {
    temporary_stat_reset_with_tail(bits, TAIL_LEN)
}

/// [`temporary_stat_reset`] with the tail length chosen by the caller - `!unbuff`'s argument.
pub fn temporary_stat_reset_with_tail(bits: &[u32], tail: usize) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(0);
    w.u8(0);
    w.u8(0);
    w.bytes(&stat_mask(bits));
    w.bytes(&vec![0u8; tail]);
    w.into_vec()
}

/// Body length of a [`temporary_stat_reset`]: 3 + the mask + the tail.
pub const TEMPORARY_STAT_RESET_LEN: usize = 3 + MASK_LEN + TAIL_LEN;

/// The shortest `0x007E` known to be too short: the client threw with **127**.
pub const RESET_KNOWN_TOO_SHORT: usize = 3 + MASK_LEN;

/// A decoded [`CLIENT_SKILL_USE`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkillUse {
    pub skill_id: u32,
    pub level: u32,
}

/// Parse a `0x013C` body (opcode already stripped).
///
/// **Only the first eight bytes are read**, and that is not laziness: the 51-byte body ends in
/// a 13-byte tail that belongs to one of sixteen builders and is not resolved. Reading past
/// what is established is how a field gets decoded as garbage without saying so.
///
/// The rest of the shared header is known - a client tick, two checksums the client echoes
/// back from its own `0x01A5`, and the character's x/y - and none of it is needed to decide
/// whether a cast is allowed. `research/buffs.md` §3.
pub fn parse_skill_use(body: &[u8]) -> Option<SkillUse> {
    Some(SkillUse {
        skill_id: u32::from_le_bytes(body.get(0..4)?.try_into().ok()?),
        level: u32::from_le_bytes(body.get(4..8)?.try_into().ok()?),
    })
}

/// One level of a skill that grants a temporary stat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuffLevel {
    pub mp_cost: u16,
    /// `Skill.wz`'s `time`, in **seconds**. [`TemporaryStat::duration_ms`] wants it x1000.
    pub seconds: u32,
    /// `Skill.wz`'s `cooltime`, in **seconds**.
    pub cooldown_seconds: u32,
    /// The stat this level grants, and by how much.
    pub bit: u32,
    pub value: i16,
}

impl BuffLevel {
    /// The `0x007D` entry for this level, granted by `skill_id`.
    pub fn granted_by(&self, skill_id: u32) -> TemporaryStat {
        TemporaryStat {
            bit: self.bit,
            value: self.value,
            reason: skill_id,
            // **Seconds to milliseconds, in the one place that conversion happens.** This is
            // the third unit bug this project would have shipped; the field name carries the
            // unit and this is the only multiplication.
            duration_ms: self.seconds.saturating_mul(1000),
        }
    }
}

/// Nimble Feet, skill **1002**, levels 1..3.
///
/// **[L]**, from `Skill_000.wz` `0001002/level`, read with `wz-dump` and recorded in
/// `research/buffs.md` §4:
///
/// ```text
/// 1: mpCon 4,  time 10, speed 10, cooltime 180
/// 2: mpCon 7,  time 20, speed 10, cooltime 180
/// 3: mpCon 10, time 30, speed 10, cooltime 180
/// ```
///
/// **Speed is 10 at every level; only the duration scales.** The owner's *"the duration that the
/// skill indicated (30 seconds)"* is level 3, which matches, and that agreement is worth
/// noting precisely because an earlier pass took a different "30 seconds" in this client to
/// be the same number and it was a coincidence.
///
/// The value is a **bonus, not an absolute**: the client's string table has `Speed: +%d`
/// (id 906). **[D]**
///
/// # Why this is a literal table and not a WZ read
///
/// It is three rows for the one beginner skill that grants a stat. Its two siblings do not:
/// `1000` Three Snails is an attack, and `1001` Recovery is a heal-over-time whose CTS bit
/// nobody has identified. A loader for a table of three known rows would be more code and one
/// more thing that can silently return nothing.
pub const NIMBLE_FEET: u32 = 1002;

/// The levels of [`NIMBLE_FEET`], indexed from 1.
pub fn buff_level(skill_id: u32, level: u32) -> Option<BuffLevel> {
    if skill_id != NIMBLE_FEET {
        return None;
    }
    let seconds = match level {
        1 => 10,
        2 => 20,
        3 => 30,
        _ => return None,
    };
    let mp_cost = match level {
        1 => 4,
        2 => 7,
        _ => 10,
    };
    Some(BuffLevel {
        mp_cost,
        seconds,
        cooldown_seconds: 180,
        bit: CTS_SPEED,
        value: 10,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **Bit 92 is byte 8 = `0x08`, and this is the assertion the whole packet rests on.**
    ///
    /// Written from the arithmetic *and* from the byte, because they can disagree: word
    /// `92 >> 5` = 2 is bytes 8..11, and `1 << (31 - (92 & 31))` = `1 << 3` = 8. The
    /// "obvious" `1 << (i & 31)` would put `0x10000000` there and set bit 67 instead, which
    /// the client would grant without complaint.
    #[test]
    fn the_speed_bit_lands_where_the_client_reads_it() {
        let mask = stat_mask(&[CTS_SPEED]);
        assert_eq!(mask.len(), 124);
        assert_eq!(mask[8], 0x08, "bytes 8..11 must read 08 00 00 00");
        assert_eq!(&mask[9..12], &[0, 0, 0]);
        assert_eq!(mask.iter().filter(|b| **b != 0).count(), 1, "exactly one byte is set");

        // The wrong endianness inside the word, spelled out so it cannot come back.
        let wrong = 1u32 << (CTS_SPEED & 31);
        assert_ne!(wrong.to_le_bytes()[0], 0x08, "1 << (i & 31) is a DIFFERENT stat");
    }

    /// Every bit round-trips through the mask, and nothing above the client's limit does.
    #[test]
    fn every_legal_bit_sets_exactly_one_bit_and_the_illegal_ones_set_none() {
        for bit in 0..MAX_CTS_BIT {
            let mask = stat_mask(&[bit]);
            let set: u32 = mask.iter().map(|b| b.count_ones()).sum();
            assert_eq!(set, 1, "bit {bit} set {set} bits");
        }
        for bit in [MAX_CTS_BIT, MAX_CTS_BIT + 1, u32::MAX] {
            assert_eq!(stat_mask(&[bit]), [0u8; MASK_LEN], "bit {bit} is out of range");
        }
    }

    /// **The exact body `research/buffs.md` §7.1 writes out, byte for byte.**
    ///
    /// A test that only checked the length would pass on a packet with the fields in the
    /// wrong order, and the client would then read the duration as a reason and expire the
    /// buff immediately - which looks like "the buff does not work", the symptom being fixed.
    #[test]
    fn nimble_feet_at_level_three_is_the_documented_body() {
        let level = buff_level(NIMBLE_FEET, 3).unwrap();
        let body = temporary_stat_set(&[level.granted_by(NIMBLE_FEET)]);

        assert_eq!(body.len(), 198, "124 mask + 10 stat + 64 tail");
        assert_eq!(body.len(), temporary_stat_set_len(1));
        assert_eq!(&body[0..8], &[0u8; 8], "the mask is zero before the speed word");
        assert_eq!(&body[8..12], &[0x08, 0, 0, 0], "bit 92");
        assert_eq!(&body[12..124], &[0u8; 112], "and zero after it");
        assert_eq!(
            &body[124..134],
            &[0x0a, 0x00, 0xea, 0x03, 0x00, 0x00, 0x30, 0x75, 0x00, 0x00],
            "speed 10, reason 1002, duration 30000 MILLISECONDS"
        );
        assert_eq!(&body[134..], &[0u8; TAIL_LEN], "the tail is all zero - see the module docs");
    }

    /// **Lengthening the tail moved nothing that the client reads.**
    ///
    /// The tail went 18 -> 64 after a crash, and the whole value of that change rests on it
    /// being *only* a length change. This pins the 134 bytes before the tail against the
    /// same literals the 152-byte version asserted, so a future edit cannot quietly shift a
    /// field and hide behind the padding.
    #[test]
    fn the_bytes_before_the_tail_are_unchanged_by_the_padding() {
        let level = buff_level(NIMBLE_FEET, 3).unwrap();
        let body = temporary_stat_set(&[level.granted_by(NIMBLE_FEET)]);
        let head = &body[..MASK_LEN + 10];
        assert_eq!(head.len(), 134);
        assert_eq!(&head[8..12], &[0x08, 0, 0, 0]);
        assert_eq!(
            &head[124..134],
            &[0x0a, 0x00, 0xea, 0x03, 0x00, 0x00, 0x30, 0x75, 0x00, 0x00]
        );
        assert!(body[134..].iter().all(|b| *b == 0), "and the tail is all zero");
        assert_eq!(body.len() - head.len(), TAIL_LEN);
    }

    /// The tail override changes the length and nothing else.
    #[test]
    fn the_tail_override_only_changes_the_tail() {
        let stat = buff_level(NIMBLE_FEET, 3).unwrap().granted_by(NIMBLE_FEET);
        let default = temporary_stat_set(&[stat]);
        for tail in [24usize, 64, 200] {
            let probe = temporary_stat_set_with_tail(&[stat], tail);
            assert_eq!(probe.len(), MASK_LEN + 10 + tail);
            assert_eq!(&probe[..134], &default[..134], "the head is identical at tail {tail}");
            assert!(probe[134..].iter().all(|b| *b == 0));
        }
        assert_eq!(temporary_stat_set_with_tail(&[stat], TAIL_LEN), default);
    }

    /// Seconds reach the wire as milliseconds, at every level. The failure this catches shows
    /// on screen as an icon that flashes and vanishes.
    #[test]
    fn the_duration_is_milliseconds_at_every_level() {
        for (level, seconds) in [(1u32, 10u32), (2, 20), (3, 30)] {
            let l = buff_level(NIMBLE_FEET, level).unwrap();
            assert_eq!(l.seconds, seconds);
            assert_eq!(l.granted_by(NIMBLE_FEET).duration_ms, seconds * 1000);
        }
        assert!(buff_level(NIMBLE_FEET, 0).is_none());
        assert!(buff_level(NIMBLE_FEET, 4).is_none(), "master level is 3");
        assert!(buff_level(1000, 1).is_none(), "Three Snails is an attack, not a buff");
        assert!(buff_level(1001, 1).is_none(), "Recovery's CTS bit is not identified");
    }

    /// Two stats are paired with the mask in **ascending bit order**, whatever order the
    /// caller passes them in. A mis-pairing gives the wrong stat the wrong number and the
    /// client reports nothing.
    #[test]
    fn entries_follow_the_mask_walk_order_not_the_callers() {
        let high = TemporaryStat { bit: 200, value: 7, reason: 1, duration_ms: 1000 };
        let low = TemporaryStat { bit: 92, value: 10, reason: 2, duration_ms: 2000 };
        let body = temporary_stat_set(&[high, low]);
        assert_eq!(body.len(), temporary_stat_set_len(2));
        assert_eq!(i16::from_le_bytes([body[124], body[125]]), 10, "bit 92 comes first");
        assert_eq!(i16::from_le_bytes([body[134], body[135]]), 7, "then bit 200");
    }

    /// **The reset body is longer than the 127 bytes that killed a client.**
    ///
    /// `reads.py` puts the minimum at 129 - `u8, u8, u8, raw[124], u8, u8` - and 133 if the
    /// gated `u32` at `0x142d571c3` fires. The literal below is the padded length; the
    /// assertion that matters is that it clears both.
    #[test]
    fn the_reset_body_clears_the_length_that_threw() {
        let body = temporary_stat_reset(&[CTS_SPEED]);
        assert_eq!(body.len(), TEMPORARY_STAT_RESET_LEN);
        assert!(body.len() > RESET_KNOWN_TOO_SHORT, "127 threw on 2026-08-22");
        assert!(body.len() >= 133, "and 133 is the maximum the enumerated reads can want");
        assert_eq!(&body[0..3], &[0, 0, 0]);
        assert_eq!(body[3 + 8], 0x08, "the same bit 92");
        assert!(body[3 + MASK_LEN..].iter().all(|b| *b == 0), "the tail is zero");

        // The override moves the length and nothing else.
        let probe = temporary_stat_reset_with_tail(&[CTS_SPEED], 200);
        assert_eq!(probe.len(), 3 + MASK_LEN + 200);
        assert_eq!(&probe[..3 + MASK_LEN], &body[..3 + MASK_LEN]);
    }

    /// **The owner's real `0x013C`, off the wire.** 51 bytes, 12:59 run of 2026-08-22.
    ///
    /// Kept as a fixture rather than a hand-built body: it is the packet that was arriving
    /// and being dropped, and its two checksum dwords match the ones `research/buffs.md` §3
    /// recorded from a *different* session, which is what makes the header reading a
    /// measurement rather than one plausible split of 51 bytes.
    #[test]
    fn the_real_skill_use_packet_parses() {
        let body: Vec<u8> = (0..51).map(|_| 0u8).collect();
        let mut body = body;
        body[0..4].copy_from_slice(&1002u32.to_le_bytes());
        body[4..8].copy_from_slice(&3u32.to_le_bytes());
        // the two checksums, at +0x0c and +0x10, echoed from the client's own 0x01A5
        body[12..16].copy_from_slice(&0x9d6c328au32.to_le_bytes());
        body[16..20].copy_from_slice(&0xe4c08ed7u32.to_le_bytes());

        let use_ = parse_skill_use(&body).expect("51 bytes is plenty");
        assert_eq!(use_.skill_id, NIMBLE_FEET);
        assert_eq!(use_.level, 3);

        // Short bodies are refused rather than read past the end.
        assert!(parse_skill_use(&body[..7]).is_none());
        assert!(parse_skill_use(&[]).is_none());
        assert_eq!(parse_skill_use(&body[..8]), Some(SkillUse { skill_id: 1002, level: 3 }));
    }
}
