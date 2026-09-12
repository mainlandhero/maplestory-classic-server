//! `0x0111` - **summoning sacks**, the eight `2100000`..`2100007` items.
//!
//! The owner, 2026-09-09: *"I just also tried summoning the GM Black Sack Jr. Balrog lvl 80"*. It
//! did nothing, and `world.log` says why in the same words as the two before it:
//!
//! ```text
//! 01:26:17.632 <- 0x0111 UNKNOWN, 10 byte body 466f15010b00260b2000
//! 01:26:17.632 -> 0x007C StatChanged: UNLOCK ONLY. 0x0111 is not handled ...
//! ```
//!
//! which decodes to slot **11**, item **2100006** - `GM Black Sack: Jr. Balrog Level 80`.
//! `research/summon-sacks-2026-09-09.md` had the whole thing decoded on the same day and it
//! was never wired. Its own answer table said so out loud: *"does the server handle `0x0111`
//! today - **No.**"*
//!
//! # What comes out of a sack is the client's own data
//!
//! `gm-handbook/summonsacks.txt`, from `0210.img/<id>/mob`, a **sibling of `info`** rather
//! than a child of it. A template listed twice means spawn two - `2100007` lists `700005`
//! twice and is the only one that does. `prob` is 100 on all nine entries across all eight
//! sacks, so nothing in this client exercises a probability roll and a server that ignores
//! `prob` cannot be distinguished from one that honours it.
//!
//! # The map gate is the client's, and the server cannot see it
//!
//! `fieldLimit` bit 2 refuses a sack **locally**, drawing *"You can't use that on this map"*
//! and sending nothing. So a map that refuses is indistinguishable here from nobody clicking,
//! and there is no server-side check to write: if a packet arrives, the client has already
//! decided the map allows it.

use super::{Reply, Session};

impl Session {
    /// `0x0111` - the player used a summoning sack.
    /// Send `0x03E8` to every summoned mob whose animation has ended - to the whole map, since
    /// every client on it holds the mob suspended. Called from [`Session::tick`].
    pub(super) fn suspend_reset_tick(&mut self, now_ms: u64) -> Vec<Reply> {
        if self.pending_suspend_resets.is_empty() {
            return Vec::new();
        }
        let (due, later): (Vec<_>, Vec<_>) =
            std::mem::take(&mut self.pending_suspend_resets).into_iter().partition(|(at, _, _)| now_ms >= *at);
        self.pending_suspend_resets = later;
        let mut out = Vec::new();
        for (_, map, object_id) in due {
            let reply = Reply {
                opcode: net::mobmove::MOB_SUSPEND_RESET,
                body: net::mobmove::mob_suspend_reset(object_id),
                what: format!(
                    "MobSuspendReset: object id {object_id} - its summoning animation has ended; mob+0x504 goes back to 0 and it can be hit. Map-wide."
                ),
            };
            self.bus().publish(self.subscriber, map, reply.clone(), None);
            out.push(reply);
        }
        out
    }

    pub(super) fn on_summon_sack(&mut self, body: &[u8]) -> Vec<Reply> {
        let unlock =
            || crate::mesodrop::unlock_unhandled_latching_request(net::summon::CLIENT_SUMMON_SACK);
        let Some(req) = net::summon::parse_summon_sack(body) else {
            crate::server::log(&format!(
                "   sack: a {} byte 0x0111 body (expected {}); answering with the unlock only",
                body.len(),
                net::summon::SUMMON_SACK_BODY_LEN
            ));
            return unlock();
        };
        let Some(chr) = self.claimed_character() else { return unlock() };

        // **The slot must really hold the item the packet names.** The client sends both, and
        // believing the id alone would let a crafted packet summon a Balrog from a slot
        // holding a potion.
        let holding = self
            .store
            .bag_items(chr.id, store::InventoryType::Use)
            .ok()
            .into_iter()
            .flatten()
            .find(|r| r.slot == req.slot)
            .map(|r| r.item.item_id);
        if holding != Some(req.item_id) {
            crate::server::log(&format!(
                "   sack: character {} asked to use {} from Use slot {}, which holds {:?}. \
                 Refused; nothing was consumed.",
                chr.id, req.item_id, req.slot, holding
            ));
            return unlock();
        }

        let Some(sack) = self.config.summon_sacks.get(&req.item_id).cloned() else {
            crate::server::log(&format!(
                "   sack: {} is not in gm-handbook/summonsacks.txt, so there is nothing to \
                 summon. Regenerate with: python tools/dump_summon_sacks.py",
                req.item_id
            ));
            return unlock();
        };

        // Where the player is standing. Same reasoning as the drop handlers: a mob put down
        // where the server does not know the player is would appear somewhere they are not.
        let Some((x, y)) = self.last_position else {
            crate::server::log(
                "   sack: the server does not know where this character is standing; refused",
            );
            return unlock();
        };
        // Rest it on a foothold and use that foothold's own id, exactly as a map's spawn
        // points carry one. `landing` gives both in a single lookup, so the position and the
        // `fh` field cannot end up describing different surfaces.
        let landed = self.config.footholds.landing(chr.map_id, x, y);
        let (x, y, fh) = match landed {
            Some(l) => (l.x, l.y, i16::try_from(l.foothold).unwrap_or(0)),
            None => (x, y, 0),
        };

        // **Consume first.** `CLAUDE.md`'s Heena rule: if the bag will not give the sack up,
        // nothing may be summoned, and the player keeps the item.
        if self
            .store
            .remove_item(chr.id, store::InventoryType::Use, req.slot, Some(1))
            .is_err()
        {
            crate::server::log("   sack: the store would not take the sack; nothing was summoned");
            return unlock();
        }

        let map = chr.map_id;
        let mut out = Vec::new();
        for template_id in &sack.mobs {
            let hp = self
                .config
                .mob_templates
                .get(template_id)
                .map(|t| u64::from(t.max_hp))
                .unwrap_or(1);
            let live = self.fields.summon_mob(map, *template_id, (x, y), fh, hp);
            let mut mob = live.as_seen();
            // **The summoning animation, for everyone on the map.** The owner, 2026-09-12: *"Upon
            // summon, it is also missing the summon effect that is played for all players."*
            // `appear_type >= 0` is `Effect/Summon.img/<summonType>` - and a mob that arrives
            // that way is SUSPENDED (untargetable) until a 0x03E8 says otherwise, which is
            // what the old "never send summonType" warning was really about. The reset is
            // scheduled below for when the animation ends. `net::mob::FieldMob::appear_type`.
            let summon_type = self.config.mob_templates.get(template_id).map(|t| t.summon_type);
            let effect = summon_type.and_then(net::mob::appear_with_summon_effect);
            mob.appear_type = effect.unwrap_or(net::mob::APPEAR_SPAWNING);
            mob.appear_option = 0;
            // Without this the client has no attack power for the mob and its own contact
            // damage floors at 1 - the same line every other spawn path carries.
            mob.forced_stat = self.forced_stat_for(mob.template_id);
            let spawn = Reply {
                opcode: net::mob::MOB_ENTER_FIELD,
                body: net::mob::mob_enter_field(&mob),
                what: format!(
                    "MobEnterField: SUMMONED template {} from sack {} at ({x}, {y}), object id \
                     {}, hp {hp}, appear {} ({}). Nothing authenticates.",
                    mob.template_id,
                    req.item_id,
                    mob.object_id,
                    mob.appear_type,
                    match effect {
                        Some(n) => format!("Effect/Summon.img/{n}; SUSPENDED until the 0x03E8 in {} ms", net::mob::summon_effect_ms(n as u32)),
                        None => "no summon effect for this template; a plain spawn".to_string(),
                    }
                ),
            };
            if let Some(n) = effect {
                self.pending_suspend_resets.push((
                    self.clock_ms.saturating_add(net::mob::summon_effect_ms(n as u32)),
                    map,
                    mob.object_id,
                ));
            }
            // Everyone on the map sees it. A summoned boss the rest of the map cannot see
            // would be hit by one person and invisible to everybody else.
            self.bus().publish(self.subscriber, map, spawn.clone(), None);
            out.push(spawn);
            // **And somebody has to run it.** The owner, 2026-09-12, with a Balrog standing
            // beside them: *"the resulting mob in the game does not have AI and does not have
            // movement and does not use skills."* The server never drives a mob; it hands
            // each one to exactly one client with 0x03D2, and that client runs the wander,
            // the aggro and the skills and reports the path back as 0x02FF. Field entry and
            // the respawn tick both grant it; this path spawned the mob and stopped. The
            // grant is unicast on purpose - two controllers roll two paths and the screens
            // diverge (`spawn_due_mobs` has the argument) - and `claim_one` is the test-and-set
            // that makes it exactly one.
            if self.fields.controllers().claim_one(map, mob.object_id, self.subscriber.get()) {
                out.push(Reply {
                    opcode: net::mobmove::MOB_CHANGE_CONTROLLER,
                    body: net::mobmove::mob_change_controller(&mob, net::mobmove::CONTROL_NORMAL),
                    what: format!(
                        "MobChangeController: summoned object id {} to this client, which claimed it. Without this the Balrog stood still with no AI (2026-09-12).",
                        mob.object_id
                    ),
                });
            }
        }

        // The sack's own slot: mode 1 with what is left, mode 3 when that was the last one.
        out.push(self.scroll_slot_reply(chr.id, req.slot, req.item_id));
        crate::server::log(&format!(
            "   sack: character {} used {} and summoned {:?} at ({x}, {y}) on map {map}",
            chr.id, req.item_id, sack.mobs
        ));
        out
    }
}
