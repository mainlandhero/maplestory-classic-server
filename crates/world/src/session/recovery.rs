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
//! **The icon (2026-10-01).** The owner: *"Recovery skill needs to be fixed from the Beginner
//! skill."* What a player saw was a heal with an empty tray and a developer's chat line saying
//! the bit was unknown. The bit has since turned up in the client's own CTS name table - **131,
//! `Regen`** (`net::buff::CTS_REGEN`, **[I]** that it is this skill's) - so a cast now sends it
//! for the skill's `time`, records it like any buff so the tick's `0x007E` takes the icon down
//! when the heal ends, and a right-click on the icon ends the heal with it. The heal is still
//! the server's, tick by tick, below.
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
        crate::server::log(&format!(
            "   recovery: level {level}, +{} HP every {}s for {} ticks ({} total){}",
            plan.per_tick,
            RECOVERY_TICK_MS / 1000,
            plan.ticks_left,
            plan.total(),
            if replaced { ", replacing the cast still running" } else { "" }
        ));
        // The icon, for exactly as long as the ticks run: the last tick lands at the same
        // instant the tick expires the bit, and `Session::tick` heals before it expires.
        let duration_ms = u32::try_from(u64::from(plan.ticks_left) * RECOVERY_TICK_MS).unwrap_or(u32::MAX);
        let stat = net::buff::TemporaryStat {
            bit: net::buff::CTS_REGEN,
            value: i16::try_from(plan.per_tick).unwrap_or(i16::MAX),
            reason: RECOVERY_SKILL_ID,
            duration_ms,
        };
        self.buffs.retain(|b| b.bit != stat.bit);
        self.buffs.push(super::buff::ActiveBuff {
            bit: stat.bit,
            skill_id: RECOVERY_SKILL_ID,
            expires_ms: now.saturating_add(u64::from(duration_ms)),
            value: stat.value,
            reason: stat.reason,
        });
        vec![Reply {
            opcode: net::buff::TEMPORARY_STAT_SET,
            body: net::buff::temporary_stat_set_with_tail(&[stat], net::buff::TAIL_LEN),
            what: format!(
                "TemporaryStatSet: Recovery level {level} - CTS {} (Regen) = {} for {duration_ms} ms, the icon for the heal-over-time",
                stat.bit, stat.value
            ),
        }]
    }

    /// The Recovery icon was right-clicked away: the heal stops with it.
    pub(super) fn stop_recovery(&mut self) {
        if self.recovering.take().is_some() {
            crate::server::log("   recovery: cancelled from its icon");
        }
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
        // **Death ends it.** A tick lifting 0 HP to anything is a revive, the same hole
        // `regen_tick` closed on 2026-08-27 and the pet's potion reopened on 2026-10-01.
        if chr.hp == 0 {
            self.recovering = None;
            crate::server::log("   recovery: the character died, so the heal ends here");
            return Vec::new();
        }

        // Consume the tick whether or not it heals anything. A full bar does not extend the
        // duration: the tooltip promises HP "in 30 sec", not thirty seconds of *effective*
        // healing, and a Recovery that paused while full would outlive its own cooldown.
        let remaining = plan.ticks_left.saturating_sub(1);
        self.recovering = (remaining > 0).then_some(Recovering {
            ticks_left: remaining,
            next_ms: now_ms.saturating_add(RECOVERY_TICK_MS),
            ..plan
        });

        // Capped at the ceiling the client draws (base plus Max HP Increase), not the base.
        let hp = chr.hp.saturating_add(plan.per_tick).min(self.pools(&chr).max_hp);
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

    /// Level 1's row exactly as `gm-handbook/skills.txt` has it, so the cast goes through the
    /// same loader the server uses.
    const LEVEL_ONE_ROW: &str = "1001, 0, 1, 3, , Recovery, 31, , , , , , , , 3, , 6, , , , , , , , , , , , , , h1, 5, , , , , , 30, , , 120, , , , , , , , , , , , , , , , , , , , , , , 4, , , , , , , , , , , , , , , , , , , , , , , , , , , , , , , , , , MP -5; Recover HP 24 in 30 sec. #cCooldown: 2 min.#";

    /// A Beginner with Recovery level 1, hurt, on a fresh connection.
    fn hurt_beginner() -> (std::sync::Arc<store::Store>, Session, u32) {
        use std::sync::Arc;
        let path = std::env::temp_dir().join(format!("maplecw-recovery-{}-{:?}.txt", std::process::id(), std::thread::current().id()));
        std::fs::write(&path, LEVEL_ONE_ROW).unwrap();
        let mut config = crate::config::Config::default();
        config.skills = crate::skilltable::SkillTable::load(&path);
        let _ = std::fs::remove_file(&path);
        assert!(config.skills.level(RECOVERY_SKILL_ID, 1).is_some(), "the row loaded");
        let store = Arc::new(store::Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Pebble".into(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        store.set_skill_level(id, RECOVERY_SKILL_ID, 1).unwrap();
        let mut s = Session::joining(store.clone(), Arc::new(config), Arc::new(crate::fields::Fields::new()));
        s.claim_for_character(id);
        let mut c = s.claimed_character().unwrap();
        (c.hp, c.max_hp, c.mp, c.max_mp) = (10, 100, 50, 50);
        store.save_character_progress(&c).unwrap();
        (store, s, id)
    }

    fn cast(s: &mut Session) -> Vec<Reply> {
        let mut b = net::buff::CLIENT_SKILL_USE.to_le_bytes().to_vec();
        b.extend_from_slice(&RECOVERY_SKILL_ID.to_le_bytes());
        b.extend_from_slice(&1u32.to_le_bytes());
        b.resize(2 + 51, 0);
        s.handle(&b)
    }

    /// **The cast puts Recovery's icon in the tray, and it comes down when the heal ends.** The
    /// owner, 2026-10-01: *"Recovery skill needs to be fixed from the Beginner skill."* Four
    /// effects: 5 MP spent, a `0x007D` on CTS 131 for 30 s with the skill as its reason, six heals
    /// of 4, and the `0x007E` at 30 s - after the last heal, in the same tick. No chat line.
    #[test]
    fn recovery_shows_its_icon_for_the_heal_and_takes_it_down_after_the_last_tick() {
        let (_store, mut s, _id) = hurt_beginner();
        s.clock_ms = 1_000;
        let out = cast(&mut s);
        assert_eq!(s.claimed_character().unwrap().mp, 45, "5 MP");
        let icon = net::buff::temporary_stat_set_with_tail(
            &[net::buff::TemporaryStat { bit: net::buff::CTS_REGEN, value: 4, reason: RECOVERY_SKILL_ID, duration_ms: 30_000 }],
            net::buff::TAIL_LEN,
        );
        assert!(out.iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_SET && r.body == icon), "the icon: {out:?}");
        assert!(!out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE), "no developer line in chat: {out:?}");

        // Idle regeneration heals on the same ticks, so Recovery's own heals are told apart by
        // their log line rather than by the HP total.
        let is_recovery_heal = |r: &Reply| r.opcode == net::stats::STAT_CHANGED && r.what.contains("Recovery level 1 +4 hp");
        let mut heals = 0;
        for t in [6_000u64, 11_000, 16_000, 21_000, 26_000] {
            let out = s.tick(t);
            heals += out.iter().filter(|r| is_recovery_heal(r)).count();
            assert!(!out.iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET), "icon still up at {t}");
        }
        let last = s.tick(31_000);
        let heal = last.iter().position(|r| is_recovery_heal(r)).expect("the sixth heal");
        let reset = last.iter().position(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET).expect("the icon comes down");
        assert!(heal < reset, "the last heal lands before the icon goes");
        assert_eq!(heals + 1, 6, "six heals of 4: the tooltip's 24 in 30 sec");
        assert!(!s.holds(net::buff::CTS_REGEN));
    }

    /// Right-clicking the icon away ends the heal too, or the HP would keep arriving with nothing
    /// in the tray to say why.
    #[test]
    fn right_clicking_the_icon_ends_the_heal() {
        let (_store, mut s, _id) = hurt_beginner();
        s.clock_ms = 1_000;
        let _ = cast(&mut s);
        let mut b = net::buff::CLIENT_SKILL_CANCEL.to_le_bytes().to_vec();
        let mut body = vec![0u8; net::buff::CLIENT_SKILL_CANCEL_LEN];
        body[..4].copy_from_slice(&RECOVERY_SKILL_ID.to_le_bytes());
        let bit = net::buff::CTS_REGEN as usize;
        body[9 + 4 * (bit >> 5) + 3 - ((bit >> 3) & 3)] |= 1 << (7 - (bit & 7));
        b.extend_from_slice(&body);
        let out = s.handle(&b);
        assert!(out.iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET), "{out:?}");
        assert!(s.recovering.is_none(), "the heal stopped");
        assert!(!s.tick(6_000).iter().any(|r| r.opcode == net::stats::STAT_CHANGED), "and nothing more arrives");
    }

    /// Dying ends the heal: a tick on 0 HP would be a revive, the hole the pet's potion opened
    /// on 2026-10-01. HP stays 0 and nothing is owed after.
    #[test]
    fn dying_ends_the_heal_instead_of_reviving() {
        let (store, mut s, _id) = hurt_beginner();
        s.clock_ms = 1_000;
        let _ = cast(&mut s);
        let mut chr = s.claimed_character().unwrap();
        chr.hp = 0;
        store.save_character_progress(&chr).unwrap();
        assert!(!s.tick(6_000).iter().any(|r| r.opcode == net::stats::STAT_CHANGED), "no heal for the dead");
        assert_eq!(s.claimed_character().unwrap().hp, 0, "still dead");
        assert!(s.recovering.is_none(), "and the heal is over, not paused");
    }

    #[test]
    fn a_time_shorter_than_one_interval_refuses_instead_of_rounding_to_nothing() {
        // 4 seconds / 5 = 0 ticks. Zero ticks is "cast succeeded, nothing happens", which is
        // the failure this module's docs are about.
        assert!(Recovering::from_skill(&row(Some(4), Some(4)), 1, 0).is_none());
    }
}
