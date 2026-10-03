//! **Forced misses and shrunken hits: the client's `0x00E5`, checked against what it could be.**
//!
//! The owner, 2026-10-03: *"identify cheaters that are intentionally forcing themselves to
//! 'miss'/'guard' against monster attacks, or changing them to too low of a damage."*
//! **Identify, not refuse** - findings go to `mob-suspects.log` beside the channel log
//! (`session/mobwatch.rs` writes it); the hit is applied exactly as before.
//!
//! # Who decides a miss
//!
//! The client. It rolls the avoid in `FUN_140268140` and reports the outcome as damage `0`
//! (`net::userhit`); the server never sees the roll. Read off the listing
//! (`research/msexe-mobattackpower.txt`) with every constant read out of the image on
//! 2026-10-03 **[L]**:
//!
//! ```text
//! gap  = max(0, levelA - levelB)                     (which level is which: see below)
//! acc' = mobAcc * 100 / ((gap + 51) * 5)
//! eva' = (eva / (eva / 80 + 1)) / (gap / 40 + 1)
//! s    = 0.2 / (exp((acc' - eva') / 12) + 1)          FUN_142f21a80 is exp: its range
//!                                                     reduction multiplies by 64/ln 2
//! r    = uniform [0.85 - s, 1.15 + s]                 FUN_140286f00 [I] uniform
//! r * acc' < eva'  ->  a miss 92% of the time         (0.08 <= rand -> return 2)
//! (1.15 + s) * acc' < eva'  ->  first a 2-3% sure hit (clamp(.., 0.02, 0.03))
//! ```
//!
//! So **no evasion can make a character miss more than 92% of touches** - the ceiling
//! [`MISS_CEILING`], which does not depend on any stat the server might have wrong. That is
//! what makes this checkable at all: the stat that matters most (`playerStat+0x1c`, the
//! client's own evasion total) is something the server only estimates.
//!
//! After a hit roll there are two more ways to take nothing (`return 3`), and those are
//! **[I]** here:
//!
//! * `max(0.05, g / (g + 500))` for a player stat `g` at `playerStat+0x10` that has not been
//!   named - [`GUARD_ALLOWANCE`] stands in for it, generously;
//! * jobs 410-412 roll skill `4110000`'s `x` percent - [`JOB_DODGE_ALLOWANCE`].
//!
//! # The test
//!
//! Each touch gets the most it could honestly be avoided, [`avoid_ceiling`]: the formula at an
//! evasion **twice the server's estimate plus 20** (the estimate is `damage::evasion`, **[I]**,
//! plus equipment and the held buff), capped at the 92%, plus the allowances. Over the last
//! [`AVOID_WINDOW`] touches, a character whose avoids are more than an honest player at
//! those ceilings would produce once in a million ([`AVOID_TAIL`]) is written down. The
//! ceilings are upper bounds, so the honest probability is smaller still.
//!
//! What that means in practice: against a mob that outclasses the character, a cheat that
//! avoids everything shows within a few dozen touches; against snails at level 30, where
//! evasion legitimately reaches the ceiling, it needs ~200, and a character that avoids 90% is
//! indistinguishable from an honest one and is never named. Measured honest baseline: **50 of
//! 280** archived touches were avoided (18%).
//!
//! # Too low
//!
//! A claim above 1 is the client's own arithmetic and the server applies it
//! (`session/combat.rs`, `client_computed_it`). Over 125 archived such hits the honest claim
//! was never below **half** the server's model. A claim under a quarter of the model's
//! **lowest** roll ([`LOW_DIVISOR`]), on a mob whose lowest roll is at least [`LOW_MIN_FLOOR`],
//! is low; [`LOW_TO_FLAG`] of the last [`LOW_WINDOW`] is a finding. A claim of exactly 1 is not
//! counted - the server already ignores it and applies its own model.
//!
//! # What this cannot see
//!
//! A client that **never sends** `0x00E5` takes no damage and leaves nothing here to count.
//! That needs the mob and player positions side by side, which `session/mobwatch.rs` has the
//! half of. **Nothing authenticates**: a finding names the claimed character, not a person.

use std::collections::VecDeque;

/// The most a character can miss, from evasion alone: `0.92 * (1 - 0.02)`, rounded up. **[L]**
pub const MISS_CEILING: f64 = 0.92;
/// The share of the remaining touches the unnamed `playerStat+0x10` roll may take. **[I]**
/// The client's own floor for that roll is 0.05; this is four times it.
pub const GUARD_ALLOWANCE: f64 = 0.20;
/// The same for jobs 410-412, whose skill `4110000` adds an `x`% dodge. **[I]**, generous.
pub const JOB_DODGE_ALLOWANCE: f64 = 0.50;
/// Never assume an honest character avoids more than this.
pub const AVOID_CAP: f64 = 0.97;

/// How many recent touches the avoid test looks at.
pub const AVOID_WINDOW: usize = 250;
/// No verdict on fewer than this.
pub const AVOID_MIN_TOUCHES: usize = 30;
/// An honest player at the ceilings does this well once in this many windows - or less often.
pub const AVOID_TAIL: f64 = 1e-6;

/// A claim times this under the model's lowest roll is low.
pub const LOW_DIVISOR: u32 = 4;
/// Ignore mobs whose lowest roll is under this - small integers make ratios meaningless.
pub const LOW_MIN_FLOOR: u32 = 8;
/// Recent client-computed hits considered.
pub const LOW_WINDOW: usize = 20;
/// Low hits among them before a finding.
pub const LOW_TO_FLAG: usize = 5;

/// The client's miss chance for one touch, `FUN_140268140` transcribed (module docs).
///
/// Which level is subtracted from which is **not** settled - the listing subtracts a value
/// read off one struct from a value on the other - so this takes the larger of the two
/// readings. That keeps it an upper bound either way.
pub fn miss_chance(mob_acc: u32, mob_level: u32, player_level: u32, eva: u32) -> f64 {
    let one = |gap: u32| {
        let gap = f64::from(gap);
        let acc = f64::from(mob_acc) * 100.0 / ((gap + 51.0) * 5.0);
        let eva = f64::from(eva);
        let eva = (eva / (eva / 80.0 + 1.0)) / (gap / 40.0 + 1.0);
        if eva <= 0.0 {
            return 0.0;
        }
        let s = 0.2 / (((acc - eva) / 12.0).exp() + 1.0);
        let (lo, hi) = (0.85 - s, 1.15 + s);
        if hi * acc < eva {
            return 0.98 * 0.92;
        }
        ((eva / acc - lo) / (hi - lo)).clamp(0.0, 1.0) * 0.92
    };
    one(player_level.saturating_sub(mob_level)).max(one(mob_level.saturating_sub(player_level)))
}

/// The most this touch could honestly be avoided (module docs, "The test").
pub fn avoid_ceiling(mob_acc: u32, mob_level: u32, player_level: u32, eva_estimate: u32, job: u16) -> f64 {
    let eva_hi = eva_estimate.saturating_mul(2).saturating_add(20);
    let miss = miss_chance(mob_acc, mob_level, player_level, eva_hi).min(MISS_CEILING);
    let other = if (410..=412).contains(&job) { JOB_DODGE_ALLOWANCE } else { GUARD_ALLOWANCE };
    (miss + (1.0 - miss) * other).min(AVOID_CAP)
}

/// `P(at least k of these happen)`, each independently with its own probability.
pub fn tail_at_least(ps: &[f64], k: usize) -> f64 {
    let mut dist = vec![1.0f64];
    for &p in ps {
        let mut next = vec![0.0; dist.len() + 1];
        for (n, &q) in dist.iter().enumerate() {
            next[n] += q * (1.0 - p);
            next[n + 1] += q * p;
        }
        dist = next;
    }
    dist.iter().skip(k).sum::<f64>().min(1.0)
}

/// What the watch concluded.
#[derive(Debug, Clone, PartialEq)]
pub enum Finding {
    /// More touches avoided than the ceilings allow.
    Avoids { avoided: usize, touches: usize, most_expected: f64, chance: f64 },
    /// Client-computed damage far under the mob's lowest roll.
    LowDamage { low: usize, of: usize, claimed: u32, floor: u32, mob_template: u32 },
}

impl Finding {
    pub fn kind(&self) -> &'static str {
        match self {
            Finding::Avoids { .. } => "AVOIDS",
            Finding::LowDamage { .. } => "LOW-DAMAGE",
        }
    }

    pub fn describe(&self) -> String {
        match self {
            Finding::Avoids { avoided, touches, most_expected, chance } => format!(
                "avoided {avoided} of the last {touches} mob touches (miss/guard, damage 0); an honest character at the most generous ceilings would avoid at most ~{most_expected:.0} and this many with probability {chance:.1e} (archived honest rate 18%)"
            ),
            Finding::LowDamage { low, of, claimed, floor, mob_template } => format!(
                "{low} of the last {of} client-computed hits under 1/{LOW_DIVISOR} of the mob's lowest roll; latest: claimed {claimed} from mob template {mob_template} whose lowest is {floor} (honest claims were never under 1/2)"
            ),
        }
    }
}

/// One session's record of the touches it has reported.
#[derive(Debug, Default)]
pub struct HitWatch {
    touches: VecDeque<(bool, f64)>,
    low: VecDeque<bool>,
}

impl HitWatch {
    /// One touch: was it avoided, and the most it could honestly have been ([`avoid_ceiling`]).
    pub fn observe_touch(&mut self, avoided: bool, ceiling: f64) -> Option<Finding> {
        self.touches.push_back((avoided, ceiling));
        if self.touches.len() > AVOID_WINDOW {
            self.touches.pop_front();
        }
        if !avoided || self.touches.len() < AVOID_MIN_TOUCHES {
            return None;
        }
        let ps: Vec<f64> = self.touches.iter().map(|t| t.1).collect();
        let k = self.touches.iter().filter(|t| t.0).count();
        let chance = tail_at_least(&ps, k);
        (chance < AVOID_TAIL).then(|| Finding::Avoids {
            avoided: k,
            touches: ps.len(),
            most_expected: ps.iter().sum(),
            chance,
        })
    }

    /// One client-computed hit (claim above 1) against the model's lowest roll.
    pub fn observe_damage(&mut self, claimed: u32, floor: u32, mob_template: u32) -> Option<Finding> {
        if claimed <= 1 || floor < LOW_MIN_FLOOR {
            return None;
        }
        let is_low = claimed.saturating_mul(LOW_DIVISOR) < floor;
        self.low.push_back(is_low);
        if self.low.len() > LOW_WINDOW {
            self.low.pop_front();
        }
        let low = self.low.iter().filter(|l| **l).count();
        (is_low && low >= LOW_TO_FLAG).then(|| Finding::LowDamage { low, of: self.low.len(), claimed, floor, mob_template })
    }
}

impl super::Session {
    /// Feed one parsed `0x00E5` to the watch, before anything decides what it does. Never
    /// changes the hit.
    pub(super) fn watch_user_hit(&mut self, hit: &net::userhit::UserHit, chr: &net::opcode::Character) {
        // Numbered and magic attacks may avoid by other rules; touches are the measured case.
        if !hit.is_touch() {
            return;
        }
        let Some(mob) = self.config.mob_templates.get(&hit.mob_template_id) else { return };
        let (acc, mob_level) = (mob.accuracy, mob.level);
        let equipment: u32 = self.dressed(chr).iter().map(|(_, _, s)| u32::from(s.stats.inc_eva)).sum();
        let held = u32::try_from(self.held_value(net::jobbuffs::CTS_AVOIDABILITY)).unwrap_or(0);
        let eva = crate::damage::evasion(u32::from(chr.dexterity), u32::from(chr.luck)) + equipment + held;
        let ceiling = avoid_ceiling(acc, mob_level, chr.level, eva, chr.job);
        let mut found = self.hit_watch.observe_touch(hit.damage == 0, ceiling);
        let pa = self.config.mob_attack.get(&hit.mob_template_id).copied().unwrap_or(0);
        if hit.damage > 1 && pa > 0 {
            let wdef = u32::try_from(self.weapon_defence(chr)).unwrap_or(0) + self.held_weapon_defence();
            let (floor, _) = crate::damage::incoming_window(pa, chr.level, wdef);
            found = found.or(self.hit_watch.observe_damage(hit.damage, floor, hit.mob_template_id));
        }
        if let Some(f) = found {
            self.record_suspicion(f.kind(), &f.describe());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The ceiling is a ceiling**: no evasion, however large, against any mob, gets past 92%.
    #[test]
    fn no_evasion_misses_more_than_ninety_two_percent() {
        for acc in [0, 1, 33, 120, 500] {
            for eva in [0, 5, 40, 300, 5_000] {
                for (ml, pl) in [(1, 1), (1, 70), (70, 1), (30, 30)] {
                    let p = miss_chance(acc, ml, pl, eva);
                    assert!((0.0..=MISS_CEILING).contains(&p), "acc {acc} eva {eva} levels {ml}/{pl}: {p}");
                }
            }
        }
        assert_eq!(miss_chance(33, 1, 10, 0), 0.0, "no evasion, no miss");
        assert!(miss_chance(33, 1, 10, 200) > 0.9, "a snail against a dodger");
        assert!(miss_chance(64, 10, 10, 8) < 0.01, "an even mob against a beginner");
    }

    /// **A cheat that avoids everything** against a mob the character cannot honestly dodge
    /// is named within a few dozen touches...
    #[test]
    fn avoiding_everything_against_an_outclassing_mob_is_named_quickly() {
        let mut w = HitWatch::default();
        // A level-30 mob with 120 accuracy against a level-10 beginner with 8 evasion.
        let ceiling = avoid_ceiling(120, 30, 10, 8, 100);
        assert!(ceiling <= 0.25, "{ceiling}");
        let first = (1..=AVOID_WINDOW).find(|_| w.observe_touch(true, ceiling).is_some());
        assert!(first.is_some_and(|n| n <= 40), "{first:?}");
    }

    /// ...and an honest character at the very ceiling is never named, even over a long run.
    #[test]
    fn an_honest_character_at_the_ceiling_is_never_named() {
        let mut w = HitWatch::default();
        let ceiling = AVOID_CAP;
        let mut x: u64 = 0x9e37_79b9_7f4a_7c15;
        for _ in 0..20_000 {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            let avoided = (x % 10_000) as f64 / 10_000.0 < ceiling;
            assert!(w.observe_touch(avoided, ceiling).is_none());
        }
        // An honest 18% against a 30% ceiling, the archived rate, likewise.
        let mut w = HitWatch::default();
        for i in 0..2_000 {
            assert!(w.observe_touch(i % 50 < 9, 0.3).is_none());
        }
    }

    #[test]
    fn the_tail_is_a_binomial_tail_when_the_chances_agree() {
        let t = tail_at_least(&[0.5; 10], 10);
        assert!((t - 1.0 / 1024.0).abs() < 1e-12, "{t}");
        assert!((tail_at_least(&[0.3; 5], 0) - 1.0).abs() < 1e-12, "at least none is certain");
    }

    /// **Shrunken hits**: five claims of 2 against a floor of 40 in twenty hits. Honest hits at
    /// half the floor, the archived worst, are never low.
    #[test]
    fn hits_far_under_the_floor_are_named_on_the_fifth() {
        let mut w = HitWatch::default();
        for _ in 0..30 {
            assert!(w.observe_damage(20, 40, 7).is_none(), "half the floor is honest");
        }
        let got: Vec<_> = (0..5).map(|_| w.observe_damage(2, 40, 7)).collect();
        assert!(got[..4].iter().all(Option::is_none));
        assert!(matches!(got[4], Some(Finding::LowDamage { low: 5, claimed: 2, floor: 40, .. })), "{got:?}");
        assert!(w.observe_damage(1, 40, 7).is_none(), "a claim of 1 is the server's to price");
        assert!(HitWatch::default().observe_damage(1, 7, 2).is_none());
    }
}
