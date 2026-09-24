//! A mob's MP and skills, on the acknowledgement of each move - King Slime's jump attack and
//! summon. `crate::mobskills` has the data and the working.

use super::{Reply, Session};

/// Wall-clock milliseconds: a skill's cooldown belongs to the mob, not to whichever session
/// is controlling it, and two sessions' `clock_ms` count from two different connections.
fn wall_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

impl Session {
    /// **What this move's acknowledgement carries**: `(mp, skill, level)` and any packets
    /// the report itself caused.
    ///
    /// For a mob `crate::mobskills` knows (King Slime), the MP is its full `maxMP` - so the
    /// client's attack chooser stops ruling out its 10-MP jump attack - and the summon is
    /// offered whenever its own data allows. A report that names the summon as USED spawns
    /// the mobs, once per cooldown. Every other mob gets the old zeros.
    pub(super) fn mob_skill_ack(
        &mut self,
        map: crate::fields::FieldKey,
        req: &net::mobmove::MobMoveRequest,
    ) -> (u32, u32, u16, Vec<Reply>) {
        let template = self.fields.mob_template(map, req.object_id).unwrap_or(0);
        let Some(kit) = crate::mobskills::kit(template) else { return (0, 0, 0, Vec::new()) };
        let (mp, max_hp) = self
            .config
            .mob_templates
            .get(&template)
            .map(|t| (t.max_mp, u64::from(t.max_hp)))
            .unwrap_or((0, 0));
        let now = wall_ms();
        let alive = |s: &Self| s.fields.mobs_on(map).iter().filter(|m| kit.mobs.contains(&m.spawn.template_id)).count();
        let mut out = Vec::new();

        // The client says it just USED the summon: spawn them, once per cooldown.
        if req.skill_id() == kit.skill && req.skill_level() == kit.level {
            let n = crate::mobskills::how_many(&kit, alive(self));
            if n > 0 && self.fields.take_skill(map, req.object_id, now, kit.interval_ms) {
                let at = self.fields.mob_site(map, req.object_id).unwrap_or((req.x, req.y));
                crate::server::log(&format!(
                    "   mob skill: mob {} (template {template}) used skill {} level {} on field {map} - summoning {n}",
                    req.object_id, kit.skill, kit.level
                ));
                out.extend(self.summon_mobs_at(map, &kit.mobs[..n], at, &format!("skill {} of mob {}", kit.skill, req.object_id)));
            }
        }

        let hp = self.fields.mob_hp(map, req.object_id).unwrap_or(0);
        let ready = self.fields.skill_ready(map, req.object_id, now);
        let offer = crate::mobskills::may_offer(&kit, hp, max_hp, ready, alive(self));
        if offer {
            (mp, kit.skill, kit.level, out)
        } else {
            (mp, 0, 0, out)
        }
    }

    /// **Put `templates` on the field at `at`**, like a summoning sack: never a spawn point,
    /// so a kill books nothing and they do not come back. Everyone on the field is shown them,
    /// and this connection takes control of any nobody holds.
    pub(super) fn summon_mobs_at(
        &mut self,
        key: crate::fields::FieldKey,
        templates: &[u32],
        at: (i16, i16),
        why: &str,
    ) -> Vec<Reply> {
        let landed = self.config.footholds.landing(key.map, at.0, at.1);
        let (sx, sy, fh) = match landed {
            Some(l) => (l.x, l.y, i16::try_from(l.foothold).unwrap_or(0)),
            None => (at.0, at.1, 0),
        };
        let mut out = Vec::new();
        for &template in templates {
            let hp = self.config.mob_templates.get(&template).map(|t| u64::from(t.max_hp)).unwrap_or(1);
            let live = self.fields.summon_mob(key, template, (sx, sy), fh, hp);
            let mut mob = live.as_seen();
            mob.forced_stat = self.forced_stat_for(mob.template_id);
            let spawn = Reply {
                opcode: net::mob::MOB_ENTER_FIELD,
                body: net::mob::mob_enter_field(&mob),
                what: format!("MobEnterField: template {template} summoned at ({sx}, {sy}), object id {} - {why}", mob.object_id),
            };
            self.bus().publish(self.subscriber, key, spawn.clone(), None);
            out.push(spawn);
            if self.fields.controllers().claim_one(key, mob.object_id, self.subscriber.get()) {
                out.push(Reply {
                    opcode: net::mobmove::MOB_CHANGE_CONTROLLER,
                    body: net::mobmove::mob_change_controller(&mob, net::mobmove::CONTROL_NORMAL),
                    what: format!("MobChangeController: summoned {} to this client - {why}", mob.object_id),
                });
            }
        }
        out
    }
}
