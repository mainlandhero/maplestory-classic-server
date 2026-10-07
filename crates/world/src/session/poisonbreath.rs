//! **Poison Breath** (`2101004`): the drop bursts on the mob it hits, and the burst poisons.
//!
//! The owner, 2026-10-05: *"Poison Breath the skill does not leave a cloud of area of effect that
//! damages monsters over time, this needs to be fixed"* - and, offered the classic behaviour or a
//! lingering cloud, chose the classic one.
//!
//! # Why the server has to do this
//!
//! Measured on the live server (2026-10-05, four casts by one character): every Poison Breath
//! arrives as an ordinary `0x00E1` naming `2101004` with **one target and a damage of 0**, and
//! nothing else follows. The skill's own level rows carry no `mad`, no `dot` and no area; all of
//! its numbers are on the hidden `2101005`, which `2101004`'s `extraSkillInfo` names with delay 0
//! (`research/second-job.md` §7). **The client never sends `2101005`.** So the burst is the
//! server's to run, and until now nothing ran it: the mob took 0 and the cast did nothing.
//!
//! # What the burst does - `2101005`'s row at the caster's level **[L]**
//!
//! * Centred on the mob the drop hit, a box `lt`..`rb` (`-130,-50`..`130,50` at level 1,
//!   `±150` at 30) covers up to `mobCount` 4 mobs, the struck one first, then the nearest.
//! * Each takes a magic hit at `mad`% Basic Attack, rolled through the client's own magic
//!   formula (`crate::magic::magic_hit_window`) against the mob's `MDDamage` and level.
//! * Each mob still standing is poisoned with `prop`% chance: `dot`% Basic Attack every
//!   `dotInterval` second for `dotTime` seconds.
//!
//! Two choices that are **not** in the data, and are the classic game's rules **[I]**:
//!
//! * **Poison never kills.** A tick that would take the last point leaves the mob on 1 HP.
//! * **A poisoned mob cannot be poisoned again** until its poison runs out, whoever cast it.
//!   It is held in `Fields::mob_debuffs` under [`POISON_KEY`], which every session sees.
//!
//! The element is taken as neutral (`1.00`): `mobtemplates.txt` carries no element column, so
//! a mob weak or strong to poison takes the plain number.
//!
//! # The poison mark: `0x03E6` status 23
//!
//! The burst and the ticks move the health bar on every screen (`deal_to_mob`). A poisoned mob
//! is also sent **status [`net::mobstat::POISON`]** - index 23, read off the client on
//! 2026-10-06 (`net::mobstat::POISON` has the listing): value = the damage per tick, reason =
//! Poison Breath, the poisoner's character id as its extra. The client then runs its **own**
//! once-a-second poison tick (`FUN_141c72e50`), drawing that value as a damage number over the
//! mob for as long as the status lasts. The server's ticks and the client's drawn numbers are
//! the same number on the same one-second beat; only the server's move the HP.
//!
//! **Not yet seen on a screen.** The index, the block and the extra are all from the listing;
//! whether the green animation appears is the measurement a first live cast makes.

use super::{Reply, Session};
use crate::fields::FieldKey;

/// Poison Breath, the skill the player casts.
pub const POISON_BREATH: u32 = 2_101_004;
/// Its hidden half: the burst's numbers. `invisible`, never granted, never sent by the client.
pub const POISON_BREATH_BURST: u32 = 2_101_005;

/// The key a poison is held under in `Fields::mob_debuffs`. **Not a client status index** -
/// it never goes on the wire, and it sits far above the 160 the `0x03E6` mask can name so it
/// cannot be mistaken for one. `Session::mob_attack_cut` reads only `net::mobstat::PAD`.
pub const POISON_KEY: u32 = 0xFFFF_0001;

/// One poison this connection is ticking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Poison {
    pub(super) map: FieldKey,
    pub(super) object_id: u32,
    pub(super) per_tick: u64,
    pub(super) interval_ms: u64,
    pub(super) next_ms: u64,
    pub(super) ticks_left: u32,
}

/// What one poison tick takes from a mob on `hp`: `per_tick`, but never the last point.
pub(crate) fn poison_tick_damage(per_tick: u64, hp: u64) -> u64 {
    per_tick.min(hp.saturating_sub(1))
}

/// The mobs one burst hits: those whose position lies in `area` around `centre`, the struck
/// mob `impact` first and then nearest first, at most `max`. `mobs` is `(object id, position)`.
pub(crate) fn burst_targets(
    centre: (i16, i16),
    area: ((i32, i32), (i32, i32)),
    mobs: &[(u32, (i16, i16))],
    impact: u32,
    max: usize,
) -> Vec<u32> {
    let ((lx, ly), (rx, ry)) = area;
    let (cx, cy) = (i32::from(centre.0), i32::from(centre.1));
    let mut inside: Vec<(bool, i64, u32)> = mobs
        .iter()
        .filter(|(id, (x, y))| {
            let (dx, dy) = (i32::from(*x) - cx, i32::from(*y) - cy);
            *id == impact || (lx..=rx).contains(&dx) && (ly..=ry).contains(&dy)
        })
        .map(|(id, (x, y))| {
            let (dx, dy) = (i64::from(*x) - i64::from(cx), i64::from(*y) - i64::from(cy));
            (*id != impact, dx * dx + dy * dy, *id)
        })
        .collect();
    inside.sort();
    inside.into_iter().take(max).map(|(_, _, id)| id).collect()
}

impl Session {
    /// A uniform roll in `lo..=hi`.
    fn roll_between(&mut self, lo: u64, hi: u64) -> u64 {
        if hi <= lo {
            return lo;
        }
        lo + self.rng.next() % (hi - lo + 1)
    }

    /// One magic hit's damage on `template` at `percent`% Basic Attack, rolled through the
    /// client's formula. `None` without a claimed character.
    fn roll_magic_hit(&mut self, template: u32, percent: u32, mastery: u32) -> Option<u64> {
        let attacker = self.magic_attacker(percent, mastery)?;
        let player_level = self.claimed_character()?.level;
        let mob = self.config.mob_templates.get(&template).copied().unwrap_or_default();
        let target = crate::magic::MagicTarget {
            magic_defence: crate::magic::effective_magic_defence(mob.md_damage, 0, 0),
            element_code: Some(0),
            level: mob.level.max(1),
        };
        let (lo, hi) = crate::magic::magic_hit_window(
            &attacker,
            &target,
            player_level,
            crate::magic::CritStats { rate_percent: 0, damage_percent: 0 },
        );
        Some(self.roll_between(lo, hi))
    }

    /// **The burst of a Poison Breath that hit** `struck` (the attack's target ids, in the
    /// client's order). Returns the replies and the templates it damaged.
    pub(super) fn poison_breath_burst(
        &mut self,
        map: FieldKey,
        chr_id: u32,
        level: u32,
        struck: &[u32],
    ) -> (Vec<Reply>, Vec<u32>) {
        let mut out = Vec::new();
        let mut landed = Vec::new();
        let Some(row) = self.config.firstjob.level(POISON_BREATH_BURST, level.max(1)).copied() else {
            crate::server::log(&format!(
                "   Poison Breath lv{level}: no {POISON_BREATH_BURST} row at that level in skills.txt - no burst"
            ));
            return (out, landed);
        };
        // The drop hit the first target that is still a mob of ours. A miss has no burst.
        let Some((impact, centre)) =
            struck.iter().find_map(|&id| self.fields.mob_site(map, id).map(|at| (id, at)))
        else {
            return (out, landed);
        };
        let mobs: Vec<(u32, (i16, i16))> = self
            .fields
            .mobs_on(map)
            .iter()
            .map(|m| (m.spawn.object_id, m.at.unwrap_or((m.spawn.x, m.spawn.y))))
            .collect();
        let area = row.area.unwrap_or(((-130, -50), (130, 50)));
        let max = row.mob_count.unwrap_or(1).max(1) as usize;
        let targets = burst_targets(centre, area, &mobs, impact, max);
        let percent = row.mad_percent.unwrap_or(0);
        let mastery = row.mastery.unwrap_or(0);
        let now = self.clock_ms;
        let mut said = Vec::new();
        for object_id in targets {
            let template = self.fields.mob_template(map, object_id).unwrap_or(0);
            let damage = if percent > 0 { self.roll_magic_hit(template, percent, mastery).unwrap_or(0) } else { 0 };
            if damage > 0 {
                landed.push(template);
                out.extend(self.deal_to_mob(map, object_id, damage, chr_id));
            }
            let mut note = format!("{object_id} -{damage}");
            // The poison, on a mob the burst left standing.
            let poisoned = self.fields.mob_hp(map, object_id).is_some() && {
                let prop = u64::from(row.prop.unwrap_or(0));
                prop > 0 && self.rng.next() % 100 < prop
            };
            if poisoned {
                if let Some(poison) = self.start_poison(map, object_id, template, &row, now) {
                    note.push_str(&format!(" poisoned {} x{}", poison.per_tick, poison.ticks_left));
                    self.poisons.retain(|p| !(p.map == map && p.object_id == object_id));
                    self.poisons.push(poison);
                    out.extend(self.show_poison(map, object_id, chr_id, &poison));
                } else {
                    note.push_str(" (already poisoned)");
                }
            }
            said.push(note);
        }
        crate::server::log(&format!(
            "   Poison Breath lv{level}: burst at {centre:?} on {impact}, {percent}% - {}",
            said.join(", ")
        ));
        (out, landed)
    }

    /// **Status 23 on the mob**, to the caster and to everyone else on the map - the mark, and the
    /// client's own per-second poison numbers. `net::mobstat::POISON`.
    fn show_poison(&mut self, map: FieldKey, object_id: u32, chr_id: u32, poison: &Poison) -> Vec<Reply> {
        let status = net::mobstat::MobStatus {
            index: net::mobstat::POISON,
            value: i32::try_from(poison.per_tick).unwrap_or(i32::MAX),
            reason: POISON_BREATH,
            duration_ms: u32::try_from(u64::from(poison.ticks_left) * poison.interval_ms).unwrap_or(u32::MAX),
            extra: chr_id,
        };
        let reply = Reply {
            opcode: net::mobstat::MOB_STAT_SET,
            body: net::mobstat::mob_stat_set(object_id, &[status]),
            what: format!(
                "MobStatSet 0x03E6: mob {object_id} POISON (status 23) {} a tick for {} s from Poison Breath, \
                 poisoner {chr_id} - FIRST EVER poison status; a green mark and per-second numbers over \
                 the mob are the measurement",
                status.value,
                status.duration_ms / 1000
            ),
        };
        self.bus().publish(self.subscriber, map, reply.clone(), None);
        vec![reply]
    }

    /// Record a new poison on a mob, or `None` if one is still running on it.
    fn start_poison(
        &mut self,
        map: FieldKey,
        object_id: u32,
        template: u32,
        row: &crate::firstjob::CastNumbers,
        now: u64,
    ) -> Option<Poison> {
        let dot = row.dot.filter(|d| *d > 0)?;
        let seconds = row.dot_time_seconds.filter(|t| *t > 0)?;
        let interval = row.dot_interval_seconds.filter(|i| *i > 0).unwrap_or(1);
        let duration_ms = u64::from(seconds) * 1000;
        {
            let mut table = self.fields.mob_debuffs();
            let held = table.entry((map, object_id)).or_default();
            held.retain(|(_, _, until)| *until > now);
            if held.iter().any(|(index, _, _)| *index == POISON_KEY) {
                return None;
            }
            held.push((POISON_KEY, 0, now + duration_ms));
        }
        let per_tick = self.roll_magic_hit(template, dot, row.mastery.unwrap_or(0)).unwrap_or(0).max(1);
        Some(Poison {
            map,
            object_id,
            per_tick,
            interval_ms: u64::from(interval) * 1000,
            next_ms: now + u64::from(interval) * 1000,
            ticks_left: seconds / interval,
        })
    }

    /// The poison ticks that are due. A poison stops when its mob is gone, when its ticks are
    /// spent, or when the caster has left the map - the caster's connection is the clock.
    pub(super) fn poison_tick(&mut self, now_ms: u64) -> Vec<Reply> {
        if self.poisons.is_empty() {
            return Vec::new();
        }
        let Some(chr) = self.claimed_character() else {
            self.poisons.clear();
            return Vec::new();
        };
        let here = self.field_of(&chr);
        let mut out = Vec::new();
        let mut keep = Vec::new();
        for mut poison in std::mem::take(&mut self.poisons) {
            if poison.map != here {
                continue;
            }
            while poison.ticks_left > 0 && poison.next_ms <= now_ms {
                let Some(hp) = self.fields.mob_hp(poison.map, poison.object_id) else {
                    poison.ticks_left = 0;
                    break;
                };
                // Poison never kills: the last point stays.
                let damage = poison_tick_damage(poison.per_tick, hp);
                if damage > 0 {
                    out.extend(self.deal_to_mob(poison.map, poison.object_id, damage, chr.id));
                }
                poison.ticks_left -= 1;
                poison.next_ms += poison.interval_ms;
            }
            if poison.ticks_left > 0 {
                keep.push(poison);
            }
        }
        self.poisons = keep;
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LEVEL_ONE_BOX: ((i32, i32), (i32, i32)) = ((-130, -50), (130, 50));

    /// **The struck mob first, then the nearest, at most four, and only inside the box.**
    #[test]
    fn the_burst_takes_the_struck_mob_and_the_nearest_inside_the_box() {
        let mobs = [
            (1, (0, 0)),     // struck
            (2, (120, 0)),   // inside, far
            (3, (-40, 10)),  // inside, near
            (4, (131, 0)),   // just outside on x
            (5, (0, -51)),   // just outside on y
            (6, (60, 40)),   // inside
            (7, (-100, -20)), // inside
            (8, (10, 0)),    // inside, nearest
        ];
        assert_eq!(burst_targets((0, 0), LEVEL_ONE_BOX, &mobs, 1, 4), vec![1, 8, 3, 6]);
        assert_eq!(burst_targets((0, 0), LEVEL_ONE_BOX, &mobs, 1, 8), vec![1, 8, 3, 6, 7, 2]);
    }

    /// **Poison never kills**: a tick bigger than what is left stops one short.
    #[test]
    fn a_poison_tick_never_takes_the_last_point() {
        assert_eq!(poison_tick_damage(12, 100), 12);
        assert_eq!(poison_tick_damage(12, 5), 4);
        assert_eq!(poison_tick_damage(12, 1), 0);
        assert_eq!(poison_tick_damage(12, 0), 0);
    }

    /// The box is relative to the struck mob, not the map origin.
    #[test]
    fn the_box_follows_the_struck_mob() {
        let mobs = [(1, (1000, 300)), (2, (1100, 340)), (3, (0, 0))];
        assert_eq!(burst_targets((1000, 300), LEVEL_ONE_BOX, &mobs, 1, 4), vec![1, 2]);
    }

    /// The real `skills.txt`: `2101005`'s level 1 is the burst described in the module docs.
    /// Skipped on a checkout without `gm-handbook/`, after saying so.
    #[test]
    fn the_burst_row_carries_the_numbers_the_tooltip_states() {
        let path = std::path::Path::new("../../gm-handbook/skills.txt");
        if !path.exists() {
            eprintln!("skipped: {} not generated", path.display());
            return;
        }
        let table = crate::firstjob::CombatTable::load(path);
        // Positive control: the visible half loads too.
        assert!(table.get(POISON_BREATH).is_some(), "Poison Breath itself must load");
        let one = table.level(POISON_BREATH_BURST, 1).expect("2101005 level 1");
        assert_eq!(one.mad_percent, Some(50));
        assert_eq!(one.mob_count, Some(4));
        assert_eq!(one.prop, Some(45));
        assert_eq!((one.dot, one.dot_time_seconds, one.dot_interval_seconds), (Some(10), Some(5), Some(1)));
        assert_eq!(one.area, Some(LEVEL_ONE_BOX));
        assert_eq!(one.mastery, Some(1));
        let thirty = table.level(POISON_BREATH_BURST, 30).expect("2101005 level 30");
        assert_eq!(thirty.area, Some(((-150, -50), (150, 50))));
        assert_eq!(thirty.dot, Some(40));
        // And the visible half carries none of it - which is why the server must run the burst.
        let cast = table.level(POISON_BREATH, 1).expect("2101004 level 1");
        assert_eq!((cast.mad_percent, cast.dot, cast.area), (None, None, None));
    }
}
