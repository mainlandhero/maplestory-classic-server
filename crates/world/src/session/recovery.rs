//! **Recovery** (skill `1001`) - the Beginner heal-over-time.
//!
//! The owner, 2026-08-29: *"We need to also implement Recovery the skill."*
//!
//! # Why it could not go in with the other buffs
//!
//! Every skill `crates/world/src/session/buff.rs` grants works the same way: look the level up
//! in a table, set a **CTS bit** to a value for a duration, and let the client draw the icon
//! and apply the effect. Recovery has no CTS bit anybody has identified - `net::buff`'s own
//! test says so - so `buff_level_for` returns `None` and a `0x013C` for it has been answered
//! with a chat line explaining that nothing happens.
//!
//! Recovery does not need one. It is not a stat: it is **HP arriving over time**, and HP is
//! the one number in this game the server unambiguously owns. The client never writes it.
//! So this is implemented the way idle regeneration already is - a tick that heals, a
//! `0x007C` that moves the bar, and the blue number over the head - and it works without
//! knowing the bit.
//!
//! **What is missing, and it is visible:** no buff icon appears in the tray, because that is
//! exactly what the CTS bit buys. The heal is real, the icon is absent, and those two facts
//! should be reported together. If the bit is ever found, this module keeps working and the
//! icon is a separate, additive change.
//!
//! # The numbers, and where the interval comes from
//!
//! `gm-handbook/skills.txt` gives three levels:
//!
//! ```text
//! level  mpCon  time  cooltime   x    tooltip
//!   1      5     30     120      4    "MP -5; Recover HP 24 in 30 sec. Cooldown: 2 min."
//!   2     10     30     120      8    "MP -10; Recover HP 48 in 30 sec."
//!   3     15     30     120     12    "MP -15; Recover HP 72 in 30 sec."
//! ```
//!
//! `x` is HP per tick and `time` is the duration. **The interval is not a column** - there is
//! no `dotInterval` on this skill - so it is derived, and the derivation is corroborated three
//! times: `24 / 4`, `48 / 8` and `72 / 12` are all **6**, and `30 / 6` is five seconds. Every
//! level agrees, which is what makes this a reading rather than a guess.
//!
//! [`RECOVERY_TICK_MS`] is therefore the constant and the tick count falls out of `time`. If a
//! future skill disagrees, the tooltip is the thing to check first: this project's most
//! expensive mistakes have been a correct number in the wrong unit.

use super::{Reply, Session};
use crate::skilltable::SkillLevel;

/// Beginner Recovery.
pub const RECOVERY_SKILL_ID: u32 = 1001;

/// One tick every five seconds. Derived from the tooltips; see the module docs.
pub const RECOVERY_TICK_MS: u64 = 5_000;

/// A heal-over-time in progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Recovering {
    /// HP per tick - `Skill.wz`'s `x`.
    pub per_tick: u32,
    /// How many ticks are still owed. Counts down; `0` means the effect is over.
    pub ticks_left: u32,
    /// When the next one is due, on the session clock.
    pub next_ms: u64,
    /// Kept for the log line, so a run can tell a level-1 cast from a level-3 one without
    /// arithmetic on the amounts.
    pub level: u32,
}

impl Recovering {
    /// What a cast of `level` sets up, or `None` if the skill row cannot support one.
    ///
    /// **A row with no `x` or no `time` yields `None` rather than a default.** A Recovery that
    /// heals `0`, or that heals forever, is worse than one that refuses: the first is
    /// indistinguishable on screen from the skill not being implemented, and this project has
    /// twice spent a client launch on exactly that confusion.
    pub fn from_skill(level_row: &SkillLevel, level: u32, now_ms: u64) -> Option<Self> {
        let per_tick = u32::try_from(level_row.x?).ok().filter(|x| *x > 0)?;
        let seconds = level_row.time_seconds.filter(|s| *s > 0)?;
        let ticks = u64::from(seconds) * 1000 / RECOVERY_TICK_MS;
        let ticks_left = u32::try_from(ticks).ok().filter(|t| *t > 0)?;
        Some(Recovering {
            per_tick,
            ticks_left,
            next_ms: now_ms.saturating_add(RECOVERY_TICK_MS),
            level,
        })
    }

    /// Total HP this cast will deliver if it runs to the end on a bar with room.
    pub fn total(&self) -> u32 {
        self.per_tick.saturating_mul(self.ticks_left)
    }
}

impl Session {
    /// Start Recovery. The MP, the cooldown and the "do you have it" check have already
    /// happened in [`super::Session::on_skill_use`]; this is only the effect.
    ///
    /// **Recasting replaces rather than stacks.** Two overlapping heal-over-times on one
    /// character would double the rate for the overlap, which no tooltip promises, and the
    /// cooldown makes it unreachable in normal play anyway - `!buff` and a GM cooldown skip
    /// are the ways in. Replacing is the behaviour that cannot surprise anyone.
    pub(super) fn start_recovery(&mut self, level_row: &SkillLevel, level: u32) -> Vec<Reply> {
        let now = self.clock_ms;
        let Some(plan) = Recovering::from_skill(level_row, level, now) else {
            return self.notice(format!(
                "Recovery level {level} has no usable x/time in Skill.wz, so there is nothing \
                 to heal. Nothing was cast."
            ));
        };
        let replaced = self.recovering.is_some();
        self.recovering = Some(plan);
        self.notice(format!(
            "Recovery level {level}: +{} HP every {}s for {} ticks ({} total).{} No buff icon \
             appears - Recovery's stat bit has never been identified, so the heal is real and \
             the tray is empty.",
            plan.per_tick,
            RECOVERY_TICK_MS / 1000,
            plan.ticks_left,
            plan.total(),
            if replaced { " Replaced the cast that was still running." } else { "" }
        ))
    }

    /// One clock tick's worth of Recovery, if any is owed.
    ///
    /// Shaped like [`super::Session::regen_tick`] on purpose: the same `0x007C` to move the
    /// bar and the same `0x02D1` effect `0x41` for the blue number. `research/recovery-number.md`
    /// is why it is that packet and not `0x007C`'s recovery trailer, which reaches a
    /// statistics counter with no renderer on the path and drew nothing across three ticks of
    /// a real run.
    pub(super) fn recovery_tick(&mut self, now_ms: u64) -> Vec<Reply> {
        let Some(plan) = self.recovering else { return Vec::new() };
        if now_ms < plan.next_ms {
            return Vec::new();
        }
        let Some(mut chr) = self.claimed_character() else {
            // No character to heal - drop the effect rather than leave it pending forever
            // against a session that has moved on.
            self.recovering = None;
            return Vec::new();
        };

        // Consume the tick whether or not it heals anything. A full bar does not extend the
        // duration: the tooltip promises HP "in 30 sec", not thirty seconds of *effective*
        // healing, and a Recovery that paused while full would outlive its own cooldown.
        let remaining = plan.ticks_left.saturating_sub(1);
        self.recovering = (remaining > 0).then_some(Recovering {
            ticks_left: remaining,
            next_ms: now_ms.saturating_add(RECOVERY_TICK_MS),
            ..plan
        });

        let hp = chr.hp.saturating_add(plan.per_tick).min(chr.max_hp);
        let healed = hp - chr.hp;
        if healed == 0 {
            // Full bar: send nothing. Restating an unmoved value is exactly what `regen_tick`
            // was corrected for, and a `0x02D1` reading `+0` would put a meaningless number
            // over the player's head. The tick is still consumed above, so the effect ends on
            // time rather than waiting for damage.
            return Vec::new();
        }

        chr.hp = hp;
        if let Err(e) = self.store.save_character_progress(&chr) {
            return vec![Reply {
                opcode: net::notice::CHAT_NOTICE,
                body: net::notice::chat_notice("Could not save the health Recovery gave you."),
                what: format!("recovery: save failed: {e}"),
            }];
        }

        vec![
            Reply {
                opcode: net::stats::STAT_CHANGED,
                body: net::stats::StatChange { hp: Some(chr.hp), ..Default::default() }.build(),
                what: format!(
                    "StatChanged: Recovery level {} +{healed} hp -> {}/{} ({remaining} tick(s) left)",
                    plan.level, chr.hp, chr.max_hp
                ),
            },
            Reply {
                opcode: net::stats::USER_EFFECT_LOCAL,
                body: net::revive::recovery_number(healed as i32, 0),
                what: format!(
                    "UserEffectLocal effect 0x41: the blue +{healed} from Recovery. Positive is \
                     what selects the blue digits over the damage ones"
                ),
            },
        ]
    }

    /// Recovery's row for a level, from the generated skill table.
    pub(super) fn recovery_level(&self, level: u32) -> Option<&SkillLevel> {
        self.config.skills.level(RECOVERY_SKILL_ID, level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skilltable::SkillLevel;

    /// Recovery level 1 as `gm-handbook/skills.txt` has it: 5 MP, 30 s, 120 s cooldown, x 4.
    ///
    /// Written out in full rather than `..Default::default()`, because `SkillLevel`
    /// deliberately does **not** derive `Default` - its own doc says a level with every number
    /// missing "is not a meaningful value, only a parse failure wearing one". Spelling the row
    /// out keeps that true here too.
    fn row(x: Option<i32>, time_seconds: Option<u32>) -> SkillLevel {
        SkillLevel {
            level: 1,
            mp_con: Some(5),
            time_seconds,
            cooltime_seconds: Some(120),
            x,
            y: None,
            mad: None,
            attack_count: None,
            mob_count: None,
            mastery_level: None,
            indie_pdd: None,
            indie_mdd: None,
            mmp_r: None,
            speed: None,
        }
    }

    fn level_one() -> SkillLevel {
        row(Some(4), Some(30))
    }

    #[test]
    fn a_level_one_cast_is_six_ticks_of_four_over_thirty_seconds() {
        // The whole derivation in one assertion. The tooltip says "Recover HP 24 in 30 sec"
        // and 24 is not a column - it falls out of x * (time / interval), so if the interval
        // were wrong this total would not be 24.
        let plan = Recovering::from_skill(&level_one(), 1, 0).expect("a usable plan");
        assert_eq!(plan.per_tick, 4);
        assert_eq!(plan.ticks_left, 6);
        assert_eq!(plan.total(), 24, "the tooltip's own number");
        assert_eq!(plan.next_ms, RECOVERY_TICK_MS, "the first tick is one interval away");
    }

    #[test]
    fn the_higher_levels_match_their_tooltips_too() {
        // 48 and 72. Three independent corroborations of the five-second interval - that is
        // what makes it a reading rather than a guess.
        for (x, total) in [(8, 48), (12, 72)] {
            assert_eq!(Recovering::from_skill(&row(Some(x), Some(30)), 2, 0).unwrap().total(), total);
        }
    }

    #[test]
    fn a_row_with_no_x_or_no_time_refuses_rather_than_healing_zero() {
        // A Recovery that heals 0 is indistinguishable on screen from one that is not
        // implemented, and that confusion has cost this project a client launch twice.
        assert!(Recovering::from_skill(&row(None, Some(30)), 1, 0).is_none());
        assert!(Recovering::from_skill(&row(Some(0), Some(30)), 1, 0).is_none());
        assert!(Recovering::from_skill(&row(Some(4), None), 1, 0).is_none());
        assert!(Recovering::from_skill(&row(Some(4), Some(0)), 1, 0).is_none());
    }

    #[test]
    fn a_time_shorter_than_one_interval_refuses_instead_of_rounding_to_nothing() {
        // 4 seconds / 5 = 0 ticks. Zero ticks is "cast succeeded, nothing happens", which is
        // the failure this module's docs are about.
        assert!(Recovering::from_skill(&row(Some(4), Some(4)), 1, 0).is_none());
    }
}
