//! A mob's MP and skills, on the acknowledgement of each move - for every mob
//! `gm-handbook/mobskills.txt` has a kit for. `crate::mobskills` has the data and the working.

use super::{Reply, Session};

/// Wall-clock milliseconds: MP and skill clocks belong to the mob, not to whichever session
/// is controlling it, and two sessions' `clock_ms` count from two different connections.
fn wall_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

impl Session {
    /// **What this move's acknowledgement carries**: `(mp, skill, level)`, and any packets the
    /// report itself caused.
    ///
    /// For a mob with a kit: an MP-costing attack in the report is charged; a summon the
    /// report names as USED is applied (once per its interval) and its `mpCon` charged; the
    /// MP after regeneration is what the controller is told; and one skill whose conditions
    /// hold - its effect applicable, off cooldown, HP at or under its threshold, MP enough,
    /// under its summon limit - is offered, at random when several do. Every other mob gets
    /// the old zeros.
    pub(super) fn mob_skill_ack(
        &mut self,
        map: crate::fields::FieldKey,
        req: &net::mobmove::MobMoveRequest,
    ) -> (u32, u32, u16, Vec<Reply>) {
        let template = self.fields.mob_template(map, req.object_id).unwrap_or(0);
        let Some(kit) = self.config.mob_skills.kit(template).cloned() else { return (0, 0, 0, Vec::new()) };
        let object_id = req.object_id;
        let now = wall_ms();
        let max_hp = self.config.mob_templates.get(&template).map(|t| u64::from(t.max_hp)).unwrap_or(0);
        let alive_of = |s: &Self, summons: &[u32]| {
            s.fields.mobs_on(map).iter().filter(|m| summons.contains(&m.spawn.template_id)).count()
        };
        let mut out = Vec::new();

        // An MP-costing attack in this report.
        if let Some(index) = crate::mobskills::attack_index(req.move_action) {
            let charged = self.fields.with_mob_mp(map, object_id, kit.max_mp, now, |m| (m.attacked(&kit, index, now), m.mp));
            if let Some((true, left)) = charged {
                crate::server::log(&format!(
                    "   mob mp: mob {object_id} (template {template}) used attack{} - {left}/{} MP left",
                    index + 1,
                    kit.max_mp
                ));
            }
        }

        // A skill this report says was USED.
        let used = (req.skill_id(), req.skill_level());
        if used.0 != 0 {
            let level = self.config.mob_skills.level(used.0, used.1).cloned();
            match level {
                // A summon whose last cast is still mostly standing is refused here too, not
                // only left un-offered: a report can arrive for an offer made before the cast
                // it would follow, or from a client that casts without one.
                Some(_) if used.0 == crate::mobskills::SUMMON && !self.fields.summons_cleared(map, object_id) => {
                    crate::server::log(&format!(
                        "   mob skill: mob {object_id} (template {template}) reported summon {} level {} - fewer than {}% of its last summons are defeated; nothing summoned, nothing charged",
                        used.0,
                        used.1,
                        crate::mobskills::RECAST_CLEARED_PERCENT
                    ));
                }
                Some(level) if kit.skills.contains(&used) && level.applicable(used.0) => {
                    let n = level.how_many(alive_of(self, &level.summons));
                    if n > 0 && self.fields.take_skill(map, object_id, used.0, now, level.interval_ms) {
                        let _ = self.fields.with_mob_mp(map, object_id, kit.max_mp, now, |m| {
                            m.regen(kit.max_mp, kit.regen, now);
                            m.spend(kit.max_mp, level.mp_con, now);
                        });
                        let at = self.fields.mob_site(map, object_id).unwrap_or((req.x, req.y));
                        crate::server::log(&format!(
                            "   mob skill: mob {object_id} (template {template}) used skill {} level {} on field {map} - summoning {n}",
                            used.0, used.1
                        ));
                        let why = format!("skill {} of mob {object_id}", used.0);
                        let (replies, summoned) = self.summon_mobs_listed(map, &level.summons[..n], at, &why);
                        self.fields.record_summons(map, object_id, summoned);
                        out.extend(replies);
                    }
                }
                _ => crate::server::log(&format!(
                    "   mob skill: mob {object_id} (template {template}) reported skill {} level {} - not one this server applies; ignored",
                    used.0, used.1
                )),
            }
        }

        // The MP the controller is told, after regeneration.
        let mp = self
            .fields
            .with_mob_mp(map, object_id, kit.max_mp, now, |m| {
                m.regen(kit.max_mp, kit.regen, now);
                m.mp
            })
            .unwrap_or(0);

        // One skill to offer, if any qualifies.
        let hp = self.fields.mob_hp(map, object_id).unwrap_or(0);
        let offers: Vec<(u32, u16)> = kit
            .skills
            .iter()
            .copied()
            .filter(|&(skill, lv)| {
                self.config.mob_skills.level(skill, lv).is_some_and(|l| {
                    l.applicable(skill)
                        // The owner, 2026-10-04: no new summons until 70% of the last are down.
                        && (skill != crate::mobskills::SUMMON || self.fields.summons_cleared(map, object_id))
                        && l.may_offer(mp, hp, max_hp, self.fields.skill_ready(map, object_id, skill, now), alive_of(self, &l.summons))
                })
            })
            .collect();
        let (skill, level) = if offers.is_empty() {
            (0, 0)
        } else {
            offers[(self.rng.next() % offers.len() as u64) as usize]
        };
        (mp, skill, level, out)
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
        self.summon_mobs_listed(key, templates, at, why).0
    }

    /// [`Session::summon_mobs_at`], and the object ids it put down - what a mob skill's cast
    /// records so its next cast can wait for them (`Fields::summons_cleared`).
    pub(super) fn summon_mobs_listed(
        &mut self,
        key: crate::fields::FieldKey,
        templates: &[u32],
        at: (i16, i16),
        why: &str,
    ) -> (Vec<Reply>, Vec<u32>) {
        let landed = self.config.footholds.landing(key.map, at.0, at.1);
        let (sx, sy, fh) = match landed {
            Some(l) => (l.x, l.y, i16::try_from(l.foothold).unwrap_or(0)),
            None => (at.0, at.1, 0),
        };
        let mut out = Vec::new();
        let mut ids = Vec::new();
        for &template in templates {
            let hp = self.config.mob_templates.get(&template).map(|t| u64::from(t.max_hp)).unwrap_or(1);
            let live = self.fields.summon_mob(key, template, (sx, sy), fh, hp);
            let mut mob = live.as_seen();
            ids.push(mob.object_id);
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
        (out, ids)
    }
}
