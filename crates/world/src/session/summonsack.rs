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
            mob.appear_type = net::mob::APPEAR_SPAWNING;
            // Without this the client has no attack power for the mob and its own contact
            // damage floors at 1 - the same line every other spawn path carries.
            mob.forced_stat = self.forced_stat_for(mob.template_id);
            let spawn = Reply {
                opcode: net::mob::MOB_ENTER_FIELD,
                body: net::mob::mob_enter_field(&mob),
                what: format!(
                    "MobEnterField: SUMMONED template {} from sack {} at ({x}, {y}), object id \
                     {}, hp {hp}. Nothing authenticates.",
                    mob.template_id, req.item_id, mob.object_id
                ),
            };
            // Everyone on the map sees it. A summoned boss the rest of the map cannot see
            // would be hit by one person and invisible to everybody else.
            self.bus().publish(self.subscriber, map, spawn.clone(), None);
            out.push(spawn);
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
