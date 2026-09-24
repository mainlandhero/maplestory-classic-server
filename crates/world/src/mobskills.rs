//! **King Slime's jump attack and its summon** - what the mob's own data says it may do, and
//! the two numbers the server must hand the client for it to do them.
//!
//! The owner, 2026-09-24: *"King Slime in the PQ has different attacks other than chasing the
//! players. There should be a jump attack, and there should be a summon slime attack. Can we
//! implement these when the King Slime has aggression on someone?"*
//!
//! # What the client's data says **[L]**
//!
//! `Mob_000.wz/0800003.img/info` (read 2026-09-24):
//!
//! ```text
//!   maxMP 100, mpRecovery 10, firstAttack 1, boss 1
//!   attack/0   action 1, type 0, jumpAttack 1, conMP 10, attackRatio 125, afterDelay 3000
//!   skill/0    skill 200, level 1, action 1
//!   revive     eleven x 7 (Slime)             - the server spawns 20 instead, the owner's number
//! ```
//!
//! and `MobSkill_000.wz/200.img/level/1`: **summon 7, 7, 7** (three Slimes), `interval 15`
//! seconds, `hp 50` (only at or below half health), `limit 15` (no more than fifteen of its
//! summons alive), `summonEffect 3`.
//!
//! # Why neither ever happened: the MP the server sent was zero
//!
//! `0x03E4 MobCtrlAck` carries, at body offset 7, a `u32` the handler (`141c820b7`) stores
//! obfuscated into `mob+0x3b0`. The attack chooser `FUN_141c7c900` reads that same field for
//! each candidate attack and **skips the attack when the attack's `+0x24` exceeds it**
//! (`141c7d38f..141c7d3b2`: `deobf(mob+0x3b0)`, `cmp [attack+0x24], eax / jle`, else skip
//! unless `[attack+0x90]` is set) **[L]**. This server sent `0` there on every
//! acknowledgement since the packet was written. King Slime's jump attack is the one attack in
//! its data with a cost - `conMP 10` - so it was ruled out on every step, leaving only the
//! chase. That `+0x24` is `conMP` specifically is **[I]**: the loader reads it by a route none
//! of `xref.py`, `dataref.py` or an immediate scan could see; the v214 reference's
//! `ctrlAck` writes `mob.getMp()` in exactly this slot **[C]**.
//!
//! The summon is offered the same way: `MobCtrlAck` offset 11 is a skill id and 15 its level,
//! looked up in the mob's template (`FUN_14049a080`), and `0` short-circuits the skill block
//! (`141c820f0`) **[L]**. This server always sent `0`. When the client performs the skill it
//! reports it in `0x02FF`'s offset-8 `u64` - skill id in the low 16 bits, level in the next 16
//! **[C]**, the v214 reader - and the effect is the server's to apply: for a summon, spawning
//! the mobs.
//!
//! # Scope
//!
//! **King Slime only.** Sending a real MP makes every mob with a `conMP` attack able to use it
//! for the first time, which is a change to every map at once; `CLAUDE.md` says test one
//! variant at a time. Every other template still gets the old zeros.

/// King Slime.
pub const KING_SLIME: u32 = 800_003;

/// A skill a mob may be offered, with the conditions its `MobSkill.img` level carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Summon {
    pub skill: u32,
    pub level: u16,
    /// `interval`, in milliseconds.
    pub interval_ms: u64,
    /// `hp`: offered only at or below this percent of max HP.
    pub hp_percent: u64,
    /// `limit`: no more than this many of what it summons alive on the field.
    pub limit: usize,
    /// The templates one cast summons.
    pub mobs: &'static [u32],
}

/// **King Slime's summon** - `MobSkill.img/200/level/1`, **[L]**.
pub const KING_SLIME_SUMMON: Summon = Summon {
    skill: 200,
    level: 1,
    interval_ms: 15_000,
    hp_percent: 50,
    limit: 15,
    mobs: &[7, 7, 7],
};

/// What a template's acknowledgement carries beyond the attack grant: `Some` only for the
/// mobs this module has read.
pub fn kit(template: u32) -> Option<Summon> {
    (template == KING_SLIME).then_some(KING_SLIME_SUMMON)
}

/// Whether to OFFER the summon now: at or below its HP threshold, off cooldown, and under
/// the summon limit.
pub fn may_offer(s: &Summon, hp: u64, max_hp: u64, ready: bool, alive_summons: usize) -> bool {
    ready && max_hp > 0 && hp.saturating_mul(100) <= max_hp.saturating_mul(s.hp_percent) && alive_summons < s.limit
}

/// How many of one cast may actually be summoned without passing the limit.
pub fn how_many(s: &Summon, alive_summons: usize) -> usize {
    s.mobs.len().min(s.limit.saturating_sub(alive_summons))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The data's conditions: half HP or below, off cooldown, under fifteen - each one alone
    /// refuses; and a cast near the limit is trimmed, not skipped.
    #[test]
    fn the_summon_follows_its_own_data() {
        let s = KING_SLIME_SUMMON;
        assert!(may_offer(&s, 8_410, 16_820, true, 0), "exactly half");
        assert!(!may_offer(&s, 8_411, 16_820, true, 0), "above half");
        assert!(!may_offer(&s, 100, 16_820, false, 0), "cooling down");
        assert!(!may_offer(&s, 100, 16_820, true, 15), "at the limit");
        assert!(!may_offer(&s, 0, 0, true, 0), "no max HP known");
        assert_eq!(how_many(&s, 0), 3);
        assert_eq!(how_many(&s, 13), 2);
        assert_eq!(how_many(&s, 15), 0);
        assert_eq!(kit(KING_SLIME), Some(s));
        assert_eq!(kit(7), None, "only King Slime");
    }
}
