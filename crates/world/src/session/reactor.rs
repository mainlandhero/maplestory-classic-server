//! Reactors on the field: the breakable Wooden Boxes Pio's quest items come out of.
//!
//! The owner, 2026-09-13: *"The next quest we need to fix is 'Pio's Collecting Recycled Goods'.
//! The quest itself is expecting the correct items, but the items come out of breakable
//! wooden boxes which we do not spawn right now. We need to spawn them and provide the drops
//! for the Wooden Box."*
//!
//! Three moments, each one function:
//!
//! * **field entry** - every reactor standing on the map, `0x0484` each, right after the NPCs
//!   (`Session::on_field_entered` calls [`Session::reactor_entry_replies`]). The client's
//!   field loader reads the WZ `reactor` node only to preload `Reactor/%07d.img`; like NPCs
//!   and mobs, the objects themselves are the server's to send.
//! * **a hit** - `0x032F` from the client ([`Session::on_reactor_hit`]): the state advances,
//!   everybody on the map gets `0x0478`, and on the hit that breaks it the drop table for the
//!   reactor is rolled and the drops land at the box the way a mob's land at the corpse.
//! * **the respawn** - `reactorTime` seconds later, from the tick
//!   ([`Session::spawn_due_reactors`]): `0x0485` for the broken one, `0x0484` for the fresh
//!   one, same object id.
//!
//! The wire is `crates/net/src/reactor.rs`; the pool is `crate::fields`. Nothing here
//! authenticates.
use super::*;

/// The `stateLength` sent with every change. `0` is what the reference sends for a hit
/// that just advances the state; the box has no timed states.
const STATE_LENGTH_MS: u32 = 0;

impl Session {
    /// One `0x0484` per reactor standing on `map`, in object-id order.
    pub(super) fn reactor_entry_replies(&mut self, map: u32) -> Vec<Reply> {
        let mut live = self.fields.reactors_on(map);
        live.sort_by_key(|r| r.seen.object_id);
        live.into_iter()
            .map(|r| Reply {
                opcode: net::reactor::REACTOR_ENTER_FIELD,
                body: net::reactor::reactor_enter_field(&r.seen),
                what: format!(
                    "ReactorEnterField: reactor {} (Reactor.wz {:07}) at ({}, {}) state {}, object id {} - the client only preloads the art; the box itself is ours to send.",
                    r.seen.template_id, r.seen.template_id, r.seen.x, r.seen.y, r.seen.state, r.seen.object_id
                ),
            })
            .collect()
    }

    /// `0x032F`: the player struck a reactor.
    ///
    /// The change of state goes to everyone on the map, this connection included. On the
    /// hit that breaks it, the reactor's drop table is rolled with the server's drop rate and
    /// each drop is placed at the box, owned by the hitter, and shown to the map the way a
    /// solo mob drop is. A hit on a reactor this map does not have - already broken, or a
    /// crafted id - is logged and answered with nothing: the client destroys nothing on its
    /// own and waits for `0x0478`, which never comes, so its box just stays.
    pub(super) fn on_reactor_hit(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(hit) = net::reactor::parse_reactor_hit(body) else {
            crate::server::log("   reactor: a 0x032F too short to name a reactor; nothing done");
            return Vec::new();
        };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let map = chr.map_id;
        let Some(outcome) = self.fields.hit_reactor(map, hit.object_id, self.clock_ms) else {
            crate::server::log(&format!(
                "   reactor: hit on object id {} on map {map}, which has no standing reactor by that id (broken and waiting, or not ours) - nothing done",
                hit.object_id
            ));
            return Vec::new();
        };
        let mut out = Vec::new();
        let change = Reply {
            opcode: net::reactor::REACTOR_CHANGE_STATE,
            body: net::reactor::reactor_change_state(
                hit.object_id,
                outcome.state,
                outcome.x,
                outcome.y,
                hit.delay,
                0,
                STATE_LENGTH_MS,
                chr.id,
            ),
            what: format!(
                "ReactorChangeState: object id {} (reactor {}) to state {}{} - hit by {} ({}), delay {} echoed from the client's own packet, hitOption {}, skill {}.",
                hit.object_id,
                outcome.template_id,
                outcome.state,
                if outcome.broken { " - BROKEN; drops follow, respawn scheduled" } else { "" },
                chr.id,
                chr.name,
                hit.delay,
                hit.hit_option,
                hit.skill_id
            ),
        };
        self.bus().publish(self.subscriber, map, change.clone(), None);
        out.push(change);
        if !outcome.broken {
            return out;
        }

        // The drops. Rolled like a mob's, placed at the box, owned by whoever broke it.
        let drop_rate = self.rate(store::rates::RateKind::Drop);
        let mut rolled = {
            let rng = &mut self.rng;
            self.config.reactor_drops.roll_at(outcome.template_id, drop_rate, &mut || rng.next())
        };
        // **The quest-item rule, the same one a mob's drops pass.** The owner, 2026-09-13: *"if it
        // is a Quest Item, normal quest item drop rules applies (if the user does not have the
        // quest, the item will not drop)."* The audience is the one who broke it - a box is
        // not a party kill - and `crate::questitems` owns every rule.
        let quest_audience = crate::questitems::audience_for(&self.store, &[chr.id]);
        let before = rolled.len();
        self.config.quest_items.filter_roll(&mut rolled, &quest_audience);
        if rolled.len() != before {
            crate::server::log(&format!(
                "   reactor: {} of {before} rolled item(s) from reactor {} are quest items that {} ({}) has no quest for, and were not offered",
                before - rolled.len(), outcome.template_id, chr.id, chr.name
            ));
        }
        let meso_rate = self.rate(store::rates::RateKind::Meso);
        let n = rolled.len() as i16;
        let mut placed_ids = Vec::new();
        for (i, r) in rolled.into_iter().enumerate() {
            // Staggered the way a mob's are, then dropped onto the nearest floor under it.
            let stagger = (i as i16 - n / 2) * 20;
            let x = outcome.x.saturating_add(stagger);
            let fallback = if self.config.footholds.is_empty() { (x, outcome.y) } else { (outcome.x, outcome.y) };
            let landing = self.config.footholds.landing(map, x, outcome.y);
            let (x, y) = landing.map(|l| (l.x, l.y)).unwrap_or(fallback);
            let (item, inv_type, meso) = if r.is_mesos() {
                let amount = meso_rate.apply(u64::from(r.quantity)).min(u64::from(u32::MAX)) as u32;
                (store::Item::bundle(0, 0), store::InventoryType::Etc, amount)
            } else {
                let Some(inv) = store::InventoryType::for_item(r.item_id) else { continue };
                let item = if inv == store::InventoryType::Equip {
                    store::Item::equip(r.item_id)
                } else {
                    store::Item::bundle(r.item_id, r.quantity.min(u32::from(u16::MAX)) as u16)
                };
                (item, inv, 0u32)
            };
            let now = self.clock_ms;
            let (drop_id, reply) = self.fields.with_drops(map, |d| {
                d.drop_from_mob(crate::drops::DropFromMob {
                    from_mob: false,
                    map_id: map,
                    owner_id: chr.id,
                    item,
                    inv_type,
                    meso,
                    x,
                    y,
                    source_x: outcome.x,
                    source_y: outcome.y,
                    now_ms: now,
                    party_id: 0,
                })
            });
            self.bus().publish(self.subscriber, map, reply.clone(), None);
            out.push(reply);
            placed_ids.push(drop_id);
        }
        crate::server::log(&format!(
            "   reactor: {} (reactor {}) broken on map {map} by {} - {} drop(s) placed at ({}, {}): {:?}; back in {} s",
            hit.object_id, outcome.template_id, chr.id, placed_ids.len(), outcome.x, outcome.y, placed_ids, outcome.respawn_s
        ));
        out
    }

    /// Reactors whose `reactorTime` has run out since they broke: `0x0485` for the broken
    /// object, `0x0484` for the fresh one - the same id, state 0 - to everyone on the map.
    pub(super) fn spawn_due_reactors(&mut self, map: u32, now_ms: u64) -> Vec<Reply> {
        let mut out = Vec::new();
        for r in self.fields.due_reactor_respawns(map, now_ms) {
            let leave = Reply {
                opcode: net::reactor::REACTOR_LEAVE_FIELD,
                body: net::reactor::reactor_leave_field(r.seen.object_id, r.broken_state, r.seen.x, r.seen.y),
                what: format!(
                    "ReactorLeaveField: object id {} (reactor {}) - the broken box is taken away before the fresh one is put down.",
                    r.seen.object_id, r.seen.template_id
                ),
            };
            let enter = Reply {
                opcode: net::reactor::REACTOR_ENTER_FIELD,
                body: net::reactor::reactor_enter_field(&r.seen),
                what: format!(
                    "ReactorEnterField: RESPAWN of reactor {} at ({}, {}), object id {}, state 0.",
                    r.seen.template_id, r.seen.x, r.seen.y, r.seen.object_id
                ),
            };
            self.bus().publish(self.subscriber, map, leave.clone(), None);
            self.bus().publish(self.subscriber, map, enter.clone(), None);
            out.push(leave);
            out.push(enter);
        }
        out
    }
}
