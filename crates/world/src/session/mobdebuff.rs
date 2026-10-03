//! **Debuffs on mobs** - Disorder first. `net::mobstat` has the packet and how it was found.
//!
//! The owner, 2026-10-02: *"skill debuffs do not work on mobs, such as Thief's Disorder"* - and after
//! a test run, *"Still no debuff status shown on mobs hit by disorder."* The damage half of a
//! debuffing skill always worked (it is an ordinary attack); the status half needed a packet
//! nobody had decoded. On every mob a debuffing swing **actually damaged**, the server now:
//!
//! 1. records the status with its expiry (`Fields::mob_debuffs`), replacing an older one of the
//!    same kind - a second Disorder refreshes, it does not stack;
//! 2. sends `0x03E6` to the attacker and publishes it to everyone else on the map, so every
//!    screen draws the icon and every client's own damage maths sees the mob's lower defence;
//! 3. lowers that mob's attack in the server's touch-damage FALLBACK while it lasts
//!    ([`Session::mob_attack_cut`]). When the client computes the hit itself - the usual case
//!    with the hook - the client already knows the mob's attack is down.

use super::{Reply, Session};

/// Disorder. **[L]** `gm-handbook/skills.txt`: `x` is the mob's attack cut, `y` its weapon
/// defence cut, `time` seconds - *"Enemy's Attack Power -25; Weapon Def. -5 for 30 sec"*.
pub const DISORDER: u32 = 4_001_000;

impl Session {
    /// The statuses a landed `skill_id` at `level` puts on a mob, or none.
    fn debuffs_of(&self, skill_id: u32, level: u32) -> Vec<net::mobstat::MobStatus> {
        if skill_id != DISORDER {
            return Vec::new();
        }
        let Some(row) = self.config.firstjob.level(skill_id, level) else { return Vec::new() };
        let Some(seconds) = row.time_seconds.filter(|t| *t > 0) else { return Vec::new() };
        let duration_ms = seconds * 1000;
        let mut out = Vec::new();
        if let Some(x) = row.x.filter(|v| *v > 0) {
            out.push(net::mobstat::MobStatus { index: net::mobstat::PAD, value: -x, reason: skill_id, duration_ms });
        }
        if let Some(y) = row.y.filter(|v| *v > 0) {
            out.push(net::mobstat::MobStatus { index: net::mobstat::PDR, value: -y, reason: skill_id, duration_ms });
        }
        out
    }

    /// Put the statuses of a landed swing on each mob in `hit` (object ids that took damage).
    /// Returns the attacker's own copies; everyone else on the map gets theirs through the bus.
    pub(super) fn debuff_mobs(
        &mut self,
        map: crate::fields::FieldKey,
        skill_id: u32,
        level: u32,
        hit: &[u32],
    ) -> Vec<Reply> {
        let statuses = self.debuffs_of(skill_id, level);
        if statuses.is_empty() || hit.is_empty() {
            return Vec::new();
        }
        let now = self.clock_ms;
        let mut out = Vec::new();
        for &object_id in hit {
            {
                let mut table = self.fields.mob_debuffs();
                let held = table.entry((map, object_id)).or_default();
                held.retain(|(index, _, until)| *until > now && !statuses.iter().any(|s| s.index == *index));
                held.extend(statuses.iter().map(|s| (s.index, s.value, now + u64::from(s.duration_ms))));
            }
            let reply = Reply {
                opcode: net::mobstat::MOB_STAT_SET,
                body: net::mobstat::mob_stat_set(object_id, &statuses),
                what: format!(
                    "MobStatSet 0x03E6: mob {object_id} {} from skill {skill_id} lv{level} for {} s - FIRST EVER \
                     mob status; an icon over the mob is the measurement (plan step 42)",
                    statuses
                        .iter()
                        .map(|s| format!("{} {:+}", if s.index == net::mobstat::PAD { "attack" } else { "defence" }, s.value))
                        .collect::<Vec<_>>()
                        .join(", "),
                    statuses[0].duration_ms / 1000
                ),
            };
            self.bus().publish(self.subscriber, map, reply.clone(), None);
            out.push(reply);
        }
        out
    }

    /// How much a mob's attack is cut right now (a positive number), for the touch-damage
    /// fallback. Expired entries are dropped on the way.
    pub(super) fn mob_attack_cut(&self, map: crate::fields::FieldKey, object_id: u32) -> u32 {
        let now = self.clock_ms;
        let mut table = self.fields.mob_debuffs();
        let Some(held) = table.get_mut(&(map, object_id)) else { return 0 };
        held.retain(|(_, _, until)| *until > now);
        let cut = held
            .iter()
            .filter(|(index, _, _)| *index == net::mobstat::PAD)
            .map(|(_, value, _)| u32::try_from(-*value).unwrap_or(0))
            .sum();
        if held.is_empty() {
            table.remove(&(map, object_id));
        }
        cut
    }
}
