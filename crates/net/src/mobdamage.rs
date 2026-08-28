//! The mob's *attack power*, which is how the server sets the damage the client draws on
//! the player - inside `0x03C6 MobEnterField`'s `forcedStatPresent` block.
//!
//! # There is no "mob hit the player for N" packet, and that is the finding
//!
//! `research/mob-to-player-damage-packet.md` is the working. The short version:
//!
//! For contact damage the **client** computes the number, draws it, and *reports* it to the
//! server in outbound `0x00E5`. No inbound packet carries it. The server's only influence on
//! that number is the mob's attack power, and it has exactly two sources:
//!
//! 1. the mob's own WZ `PADamage` - `template+0x34`, written by the template loader
//!    `FUN_14047d990` at `0x140480ace`; or
//! 2. **this block**, whose fourth field overrides it.
//!
//! Both land in the same place: `*(mob+0x3c8) + 0x58`, read by `FUN_14025e540` and scaled by
//! `FUN_140265f00` into the drawn damage. The copy is `FUN_14046ad90`, and it prefers this
//! block whenever `mob+0xA20` is non-null:
//!
//! ```text
//! if (forcedStat != 0) {
//!     stat[0x16] = forcedStat[0x18];   // <- PAD, wire field 3.  THE ONE THAT MATTERS
//!     stat[0x20] = forcedStat[0x1c];   // <- MAD
//!     stat[0x1a] = forcedStat[0x20];
//!     stat[0x24] = forcedStat[0x24];
//!     stat[0x2a] = forcedStat[0x28];   // <- acc
//!     stat[0x2e] = forcedStat[0x2c];   // <- eva
//!     if (0 < forcedStat[0x38]) stat[0x32] = forcedStat[0x38];
//!     stat[0]    = forcedStat[0x30];   // <- LEVEL
//! }
//! ```
//!
//! # It is all-or-nothing, and that is the trap
//!
//! The block does **not** merge with the template. Once `mob+0xA20` exists, level, acc and
//! eva come from it too - and the client's hit-type function `FUN_140268140` divides by
//! `(playerLevel - mobLevel + 51) * 5` and compares `mobAcc` against the player's evasion.
//! A block that sets `pad` and leaves `level`/`acc` at zero changes three inputs, not one.
//!
//! [`MobForcedStat::from_template`] exists so the caller starts from the mob's real WZ row
//! (`gm-handbook/mobtemplates.txt`) and overrides only what it means to.
//!
//! # Sending it is not free
//!
//! `forcedStatPresent` is a `u8` at body offset 10. The client reads it at `141d33734` and
//! **skips `FUN_141cc9410` entirely when it is zero** - so today's `0` costs nothing and
//! sends nothing. A non-zero byte adds these **57 bytes** to every `0x03C6`, and a mismatch
//! desynchronises the rest of the body.
//!
//! **[L]** everywhere above: the parser is `FUN_14085acd0`
//! (`0x14085acd0..0x14085ad8b`, 187 bytes), disassembled field by field; the consumer is
//! `FUN_14046ad90` (`0x14046ad90`, 522 bytes), decompiled.

/// Length of an encoded [`MobForcedStat`]: **57 bytes** - `u64` + twelve `u32` + `u8`.
///
/// Counted off the parser's own reads at `0x14085ace3` (`u64`), eleven `u32`s at
/// `0x14085acef`..`0x14085ad68`, and the trailing `u8` at `0x14085ad73`. **[L]**
pub const MOB_FORCED_STAT_LEN: usize = 8 + 12 * 4 + 1;

/// The value of `forcedStatPresent` (body offset 10) that makes the client read the block.
///
/// The test at `141d33739` is `test al,al / je`, so **any** non-zero byte works. **[L]**
pub const FORCED_STAT_PRESENT: u8 = 1;

/// The `forcedStatPresent` block of `0x03C6 MobEnterField`, in **wire order**.
///
/// The wire order is not the struct order - the parser writes fields 9, 10 and 11 to
/// `+0x34`, `+0x38` and `+0x30`, in that order. Field names below are the *consumer's*
/// meaning where one was traced, and `unknown_*` where none was.
///
/// | wire | struct | field | consumed by `FUN_14046ad90` as |
/// |---:|---|---|---|
/// | 0 | `+0x08` | [`max_hp`](Self::max_hp) | mob max HP (`FUN_141c8a730`, the HP bar) |
/// | 1 | `+0x10` | [`unknown_1`](Self::unknown_1) | nothing traced |
/// | 2 | `+0x14` | [`unknown_2`](Self::unknown_2) | nothing traced |
/// | 3 | `+0x18` | [`pad`](Self::pad) | **physical attack -> the damage drawn on the player** |
/// | 4 | `+0x1c` | [`mad`](Self::mad) | magic attack |
/// | 5 | `+0x20` | [`pdd`](Self::pdd) | `stat[0x1a]` |
/// | 6 | `+0x24` | [`mdd`](Self::mdd) | `stat[0x24]` |
/// | 7 | `+0x28` | [`acc`](Self::acc) | accuracy, in the hit-type test |
/// | 8 | `+0x2c` | [`eva`](Self::eva) | evasion |
/// | 9 | `+0x34` | [`unknown_9`](Self::unknown_9) | nothing traced |
/// | 10 | `+0x38` | [`speed`](Self::speed) | `stat[0x32]`, **only when > 0** |
/// | 11 | `+0x30` | [`level`](Self::level) | mob level |
/// | 12 | `+0x48` | [`unknown_12`](Self::unknown_12) | nothing traced |
/// | 13 | `+0x04` | [`flag`](Self::flag) | stored as `!= 0`; no consumer traced |
///
/// > **One field the consumer reads that the parser never writes.** `FUN_14046ad90` copies
/// > eight bytes from `forcedStat + 0x50`, and `FUN_14085acd0` writes nothing there. Whatever
/// > the allocation at `141cc9438` leaves is what gets copied. Not investigated. **[L]**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MobForcedStat {
    /// Wire 0, `u64`. Mob max HP. The HP bar divides by this.
    pub max_hp: u64,
    /// Wire 1. No traced consumer.
    pub unknown_1: u32,
    /// Wire 2. No traced consumer.
    pub unknown_2: u32,
    /// Wire 3. **Physical attack power.** This is the number that drives the damage the
    /// client computes, draws over the player, and reports back in `0x00E5`.
    pub pad: u32,
    /// Wire 4. Magic attack power.
    pub mad: u32,
    /// Wire 5. Physical defence.
    pub pdd: u32,
    /// Wire 6. Magic defence.
    pub mdd: u32,
    /// Wire 7. Accuracy. Feeds `FUN_140268140`'s hit-type test against the player's evasion.
    pub acc: u32,
    /// Wire 8. Evasion.
    pub eva: u32,
    /// Wire 9. No traced consumer.
    pub unknown_9: u32,
    /// Wire 10. Copied only when `> 0`.
    pub speed: u32,
    /// Wire 11. **Mob level.** `FUN_140268140` uses `playerLevel - mobLevel`; a zero here is
    /// not "unset", it is level zero.
    pub level: u32,
    /// Wire 12. No traced consumer.
    pub unknown_12: u32,
    /// Wire 13, `u8`. Stored as `!= 0` at `forcedStat+0x04`. No consumer traced.
    pub flag: u8,
}

impl MobForcedStat {
    /// Start from the mob's own WZ row, so an override changes one input rather than eight.
    ///
    /// The arguments are the columns of `gm-handbook/mobtemplates.txt` in its own order, which
    /// is generated from this client's `Mob.wz` by `tools/dump_mobs.py`. Passing a mob's real
    /// row reproduces what the client would have used anyway; then set [`pad`](Self::pad).
    ///
    /// ```
    /// # use net::mobdamage::MobForcedStat;
    /// // template 2, the snail on map 40: maxHP 45, level 1, PADamage 3, acc 33
    /// let mut s = MobForcedStat::from_template(45, 1, 3, 0, 0, 0, 33, 0);
    /// s.pad = 250;
    /// assert_eq!(s.encode().len(), net::mobdamage::MOB_FORCED_STAT_LEN);
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn from_template(
        max_hp: u64,
        level: u32,
        pad: u32,
        mad: u32,
        pdd: u32,
        mdd: u32,
        acc: u32,
        eva: u32,
    ) -> Self {
        Self {
            max_hp,
            unknown_1: 0,
            unknown_2: 0,
            pad,
            mad,
            pdd,
            mdd,
            acc,
            eva,
            unknown_9: 0,
            // 0 is safe here: FUN_14046ad90 guards this one with `if (0 < ...)`, so zero
            // leaves the template's value in place. It is the only field that behaves that
            // way, which is why every other default below is a real value, not a zero.
            speed: 0,
            level,
            unknown_12: 0,
            flag: 0,
        }
    }

    /// The 57 bytes, in the parser's read order.
    pub fn encode(&self) -> Vec<u8> {
        let mut b = Vec::with_capacity(MOB_FORCED_STAT_LEN);
        b.extend_from_slice(&self.max_hp.to_le_bytes()); //     14085ace3 -> +0x08
        b.extend_from_slice(&self.unknown_1.to_le_bytes()); //  14085acef -> +0x10
        b.extend_from_slice(&self.unknown_2.to_le_bytes()); //  14085acfa -> +0x14
        b.extend_from_slice(&self.pad.to_le_bytes()); //        14085ad05 -> +0x18
        b.extend_from_slice(&self.mad.to_le_bytes()); //        14085ad10 -> +0x1c
        b.extend_from_slice(&self.pdd.to_le_bytes()); //        14085ad1b -> +0x20
        b.extend_from_slice(&self.mdd.to_le_bytes()); //        14085ad26 -> +0x24
        b.extend_from_slice(&self.acc.to_le_bytes()); //        14085ad31 -> +0x28
        b.extend_from_slice(&self.eva.to_le_bytes()); //        14085ad3c -> +0x2c
        b.extend_from_slice(&self.unknown_9.to_le_bytes()); //  14085ad47 -> +0x34
        b.extend_from_slice(&self.speed.to_le_bytes()); //      14085ad52 -> +0x38
        b.extend_from_slice(&self.level.to_le_bytes()); //      14085ad5d -> +0x30
        b.extend_from_slice(&self.unknown_12.to_le_bytes()); // 14085ad68 -> +0x48
        b.push(self.flag); //                                   14085ad73 -> +0x04
        debug_assert_eq!(b.len(), MOB_FORCED_STAT_LEN);
        b
    }
}

/// What the client will compute for a contact hit, given the mob's attack power.
///
/// This is `FUN_140265f00` with every constant read out of `.rdata`, and it exists so a
/// prediction can be written down *before* a client run rather than explained after one.
///
/// ```text
/// r      = rand / 2^32 * 0.4 + 0.1          in [0.1, 0.5)
/// raw    = (r + 1.0) * pad                  in [1.1*pad, 1.5*pad)
/// raw   *= 1.0 - pdd / ((pdd + (playerLevel + 40) * 5) + raw * 1.2)
/// damage = clamp(raw, 1.0, 50_000_000.0)    as i32
/// ```
///
/// Constants, all `double`, all dumped from the image: `2.3283064365386963e-10` (= 2^-32) at
/// `0x14327a9c8`, `0.4` at `0x14327aa18`, `0.1` at `0x14327a9f0`, `1.0` at `0x1434b95e0`,
/// `1.2` at `0x14327aa48`, `5.0e7` at `0x14327ab18`. **[L]**
///
/// # The floor is 1, and it is the whole reason this module exists
///
/// `max(raw, 1.0)` is in the client's own arithmetic. **`pad = 0` produces exactly `1`, for
/// every mob, every hit, with no variance** - which is precisely what 198 of 198 archived
/// captures show. Any real `pad` produces a number that tracks it.
///
/// `roll` is the random draw in `[0.0, 1.0)`; pass `0.5` for the midpoint.
pub fn predicted_client_damage(pad: u32, pdd: u32, player_level: u32, roll: f64) -> i32 {
    let r = roll * 0.4 + 0.1;
    let mut raw = (r + 1.0) * f64::from(pad);
    let pdd = f64::from(pdd);
    raw *= 1.0 - pdd / ((pdd + f64::from(player_level + 40) * 5.0) + raw * 1.2);
    raw.clamp(1.0, 50_000_000.0) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_block_is_fifty_seven_bytes_in_parser_order() {
        let s = MobForcedStat::from_template(45, 1, 3, 0, 0, 0, 33, 0);
        let b = s.encode();
        assert_eq!(b.len(), MOB_FORCED_STAT_LEN, "FUN_14085acd0 reads 8 + 12*4 + 1");
        // max_hp is the u64 the HP bar divides by - first on the wire.
        assert_eq!(&b[0..8], &45u64.to_le_bytes(), "14085ace3 u64 -> +0x08");
        // Wire 0 is the u64, so wire field N >= 1 starts at 8 + (N - 1) * 4.
        // pad is wire 3 -> 8 + 2*4 = 16.
        assert_eq!(&b[16..20], &3u32.to_le_bytes(), "14085ad05 -> +0x18, the PAD");
        // acc is wire 7 -> 8 + 6*4 = 32.
        assert_eq!(&b[32..36], &33u32.to_le_bytes(), "14085ad31 -> +0x28, the acc");
        // level is wire 11 -> 8 + 10*4 = 48. Struct order would have put it before speed.
        assert_eq!(&b[48..52], &1u32.to_le_bytes(), "14085ad5d -> +0x30, the level");
        assert_eq!(b[56], 0, "14085ad73 the trailing u8");
    }

    #[test]
    fn overriding_pad_touches_only_pad() {
        let base = MobForcedStat::from_template(45, 1, 3, 0, 0, 0, 33, 0);
        let mut hot = base;
        hot.pad = 250;
        let (a, b) = (base.encode(), hot.encode());
        assert_eq!(a.len(), b.len());
        // Every byte OUTSIDE the PAD field is untouched. (Inside it, 3 -> 250 only moves the
        // low byte; asserting each of those four differs would be asserting something false.)
        for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
            if !(16..20).contains(&i) {
                assert_eq!(x, y, "byte {i} is outside the PAD field and must not move");
            }
        }
        assert_ne!(&a[16..20], &b[16..20], "the PAD field itself must change");
    }

    /// The claim this whole file rests on: a zero attack power is indistinguishable from the
    /// floor, and a real one is not. If this ever fails, the constants were misread.
    #[test]
    fn zero_attack_power_is_exactly_the_observed_one() {
        for roll in [0.0, 0.25, 0.5, 0.75, 0.999] {
            assert_eq!(
                predicted_client_damage(0, 30, 10, roll),
                1,
                "pad 0 must give the floor for every roll - this is the 198-of-198 signature"
            );
        }
        // Template 2, the snail: PADamage 3. Never 1, for any roll.
        for roll in [0.0, 0.5, 0.999] {
            assert!(
                predicted_client_damage(3, 30, 10, roll) >= 2,
                "PADamage 3 cannot look like the floor"
            );
        }
        // Template 45, the Drake that took the owner from 238 HP to 0: PADamage 287.
        assert!(predicted_client_damage(287, 30, 10, 0.5) > 200);
    }

    #[test]
    fn the_damage_is_monotonic_in_attack_power() {
        let mut last = 0;
        for pad in [0, 1, 3, 27, 287, 1000] {
            let d = predicted_client_damage(pad, 30, 10, 0.5);
            assert!(d >= last, "pad {pad} gave {d}, below the previous {last}");
            last = d;
        }
    }
}
