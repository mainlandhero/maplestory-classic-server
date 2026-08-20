//! Hitting things, and the mobs that get hit.
//!
//! The client computes its own damage - the server never sends a number, only the
//! consequence. And a mob move report must be answered or the client runs one simulation
//! step and stops.

use super::*;

impl Session {

    /// The client reporting where a mob it controls has moved to. **This must be answered.**
    ///
    /// # A retraction, and the reason it happened is worth more than the fix
    ///
    /// This handler returned nothing for one run, on the strength of
    /// `research/mob-behaviour.md` §6: a scan for `mob+0x2f4` found 20 sites and *"not one of
    /// them is inside any of the eight mob-pool packet handlers"*. Re-running that scan gives
    /// the **identical 20 sites** - it was never wrong. What was wrong is the set it was
    /// intersected against: there are **110** mob-pool handlers, not eight, and §2.2 of that
    /// same document had already said so. One of the 20 is `141c821f8` inside
    /// `FUN_141c82060`, whose only caller is the stub for **`0x03E4`**.
    ///
    /// Enumerate before you filter, failed twice inside one file. On screen it looked like
    /// mobs moving for half a second and then freezing forever.
    ///
    /// # What the answer does
    ///
    /// `0x03E4` **MobCtrlAck** de-obfuscates the client's own move counter from
    /// `mob+0x2f0`/`+0x2f4` - the pair the sender incremented - compares it against the
    /// `move_id` we echo (`141c82212 CMP EAX,ECX / JNS`, so `ack >= current` passes), and
    /// re-runs slot 8 `FUN_141c54200`. That slot no-ops when the animation is already running
    /// (`141c54248 JNE ret`), which is what makes it a pump rather than an initialiser.
    ///
    /// # The broadcast half, which has no recipient yet
    ///
    /// `0x03D9` is the *rebroadcast to every other client on the field* - the owner's point that a
    /// second player must see the same movement. `net::mobmove::mob_move_broadcast` builds it
    /// and is tested, but this server has no field-occupancy registry: a `Session` is one
    /// connection and knows of no other. **It is deliberately not sent to the mover** - that
    /// would be a different packet than the one they are owed. Wiring it needs the player
    /// list that `config::spawn_capacity`'s `players_here = 1` is also waiting on.
    pub(super) fn on_mob_move(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(req) = net::mobmove::parse_mob_move(payload) else {
            return Vec::new();
        };
        vec![Reply {
            opcode: net::mobmove::MOB_CTRL_ACK,
            body: net::mobmove::mob_ctrl_ack(req.object_id, req.move_id, false),
            what: format!(
                "MobCtrlAck: mob {} move {} acknowledged. Without this the client runs one \
                 simulation step and stops - measured twice, 30 grants and 30 reports all \
                 with moveId 1, then silence.",
                req.object_id, req.move_id
            ),
        }]
    }


    /// The player swung at something.
    ///
    /// **The client has already worked out the damage.** Each target block in `0x00DF` carries
    /// the numbers it intends to show, so nothing is sent back to make them appear - what the
    /// server owes is the *consequence*: the health bar, and the death.
    ///
    /// **Drops are rolled here**, on the death branch - see [`Session::drops_from_kill`].
    ///
    /// Still missing: **no EXP for a kill.** `0x007C` bit 16 carries it and experience now
    /// persists (`!exp` proves the whole chain), but the amount per kill is the EXP curve,
    /// which lives in the BSS tail of `.data` and cannot be read statically. `STATUS.md`.
    ///
    /// **A miss is not an error.** An attack with no targets is exactly what the client sends
    /// when it swings at empty air, and on 2026-08-19 it was also - wrongly - reported as
    /// proof that the client would not target our mobs at all. That claim came from combining
    /// two different sessions and is retracted; `research/mob-combat.md` §17.
    pub(super) fn on_attack(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Ok(attack) = net::combat::parse_attack(payload) else {
            return Vec::new();
        };
        // **The only coordinate pair this server reads from the client.** The attack body
        // carries the player's own position (fields 13/14), which is how the zero-target
        // captures were paired against mob positions in `research/mob-target-gates.md` §1.
        // Nothing parses `0x00D9`, so until something does, this is where a drop learns
        // where to land - see `Session::last_position`. Recording it here rather than in the
        // drop path means it survives the swing that produced it.
        self.last_position = Some((attack.x as i16, attack.y as i16));
        // Read once rather than per target: it is a database round trip, and a swing can
        // legitimately kill several mobs at once.
        let killer = self.claimed_character().map(|c| (c.id, c.map_id));
        let mut out = Vec::new();
        for target in &attack.targets {
            let Some(hp_before) = self.mob_hp.get(&target.object_id).copied() else {
                continue; // not a mob of ours, or already dead and removed
            };
            let hit = net::combat::apply_damage(hp_before, target.total_damage());
            if hit.died {
                // The id must never come back. A later hit on a corpse finds nothing here
                // and is ignored, which is what mob_hit_replies expects.
                self.mob_hp.remove(&target.object_id);
                let template = self.mob_template.remove(&target.object_id).unwrap_or(0);
                if let Some((id, map)) = killer {
                    out.extend(self.drops_from_kill(template, id, map));
                }
            } else {
                self.mob_hp.insert(target.object_id, hit.hp_after);
            }
            for (opcode, body) in net::combat::mob_hit_replies(target.object_id, &hit) {
                out.push(Reply {
                    opcode,
                    body,
                    what: format!(
                        "mob {} took {} ({} -> {}){}",
                        target.object_id,
                        hit.damage_applied,
                        hit.hp_before,
                        hit.hp_after,
                        if hit.died { " - DEAD, leaving the field" } else { "" }
                    ),
                });
            }
        }
        out
    }


    /// Roll what a dead mob leaves on the floor, and put it there.
    ///
    /// **Two tables, in order: this mob's own, then the global one.** The owner asked for the
    /// global table so an event item can drop from anything without touching code; it is
    /// empty until there is an event. `crate::droptables` owns the rolling and the policy,
    /// and this function owns only the consequences.
    ///
    /// # Where it lands, and why it can decline
    ///
    /// At the **player's** last known position, not the mob's. The server does not track
    /// where a mob is - the client controls it and reports movement we only acknowledge - so
    /// the mob's own coordinates are not available at the moment it dies. The player is
    /// adjacent to whatever they just killed, and adjacent is what the client's pick-up
    /// sweep tests, so this is right in practice and wrong in principle; when mob positions
    /// are tracked, this should use them.
    ///
    /// With no known position it drops **nothing** and says so in the log rather than
    /// guessing. An item placed where the player cannot reach looks identical to no drop at
    /// all, and would make the next run unreadable.
    ///
    /// # Nothing can be picked up yet
    ///
    /// The player's pick-up request opcode is still unknown - see `crate::drops`. Items land
    /// and are visible; collecting them needs one run to name the opcode.
    pub(super) fn drops_from_kill(&mut self, template: u32, killer: u32, map: u32) -> Vec<Reply> {
        if template == 0 {
            return Vec::new(); // an object id we never spawned; nothing to look up
        }
        let Some((x, y)) = self.last_position else {
            return self.notice(
                "A mob died with drops to give, but the server does not know where you are                  standing, so it dropped nothing rather than putting it out of reach."
                    .to_string(),
            );
        };
        let rolled = {
            let rng = &mut self.rng;
            self.config.drops.roll(template, &mut || rng.next())
        };
        let mut out = Vec::new();
        for r in rolled {
            let (item, inv_type, meso) = if r.is_mesos() {
                // A placeholder item: `LiveDrop::is_meso` gates every read of it.
                (store::Item::bundle(0, 0), store::InventoryType::Etc, r.quantity)
            } else {
                // An id whose leading digit names no tab is not an item this game has. Skip
                // it rather than guess a bag: the same rule `!item` follows, and for the same
                // reason - the client has to render whatever arrives.
                let Some(inv) = store::InventoryType::for_item(r.item_id) else {
                    continue;
                };
                let item = if inv == store::InventoryType::Equip {
                    store::Item::equip(r.item_id)
                } else {
                    store::Item::bundle(r.item_id, r.quantity.min(u32::from(u16::MAX)) as u16)
                };
                (item, inv, 0u32)
            };
            let reply = self.drops.drop_from_mob(crate::drops::DropFromMob {
                map_id: map,
                owner_id: killer,
                item,
                inv_type,
                meso,
                x,
                y,
                now_ms: self.clock_ms,
            });
            out.push(reply);
        }
        out
    }
}
