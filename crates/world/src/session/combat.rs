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
    /// Two things this deliberately does not do yet:
    ///
    /// * **No EXP.** `0x007C` bit 16 carries it and `net::combat::stat_changed` builds it, but
    ///   `Character` has no `exp` field and nothing persists one, so crediting a kill would
    ///   mean inventing a number that vanishes at the next login.
    /// * **No drops.** Nothing decodes the drop pool yet - `STATUS.md` goal C.
    ///
    /// **A miss is not an error.** An attack with no targets is exactly what the client sends
    /// when it swings at empty air, and on 2026-08-19 it was also - wrongly - reported as
    /// proof that the client would not target our mobs at all. That claim came from combining
    /// two different sessions and is retracted; `research/mob-combat.md` §17.
    pub(super) fn on_attack(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Ok(attack) = net::combat::parse_attack(payload) else {
            return Vec::new();
        };
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
}
