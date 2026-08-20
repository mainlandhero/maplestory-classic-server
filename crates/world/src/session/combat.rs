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
        // Remember where it says the mob is. This is the only source of a live mob position
        // - the client runs the movement and we only acknowledge it - and it is what lets a
        // drop fall where the mob died instead of at the player's feet.
        //
        // `x`/`y` are the path's START, so this is at most one report stale - about half a
        // second, which is a few pixels for a snail. The path's END would be exact and needs
        // the element walk; this is the cheap 95% and it is the difference between a drop at
        // the mob and a drop across the platform.
        self.mob_position.insert(req.object_id, (req.x, req.y));
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
                    self.schedule_respawn(map, target.object_id);
                    out.extend(self.drops_from_kill(template, target.object_id, id, map));
                    let worth = self.config.mob_exp.get(&template).copied().unwrap_or(0);
                    out.extend(self.award_experience(u64::from(worth), "a kill"));
                    let _ = id;
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
    pub(super) fn drops_from_kill(
        &mut self,
        template: u32,
        object_id: u32,
        killer: u32,
        map: u32,
    ) -> Vec<Reply> {
        if template == 0 {
            return Vec::new(); // an object id we never spawned; nothing to look up
        }
        // **Where the mob was, not where the player is.** The owner, after seeing it on screen:
        // *"they should drop from the killed mob's position, not from the player character
        // position"*. `mob_position` is fed by the client's own movement reports; the
        // player's position is the fallback for a mob that never moved.
        let at = self.mob_position.remove(&object_id).or(self.last_position);
        let Some((x, y)) = at else {
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
        // **Stagger them.** The owner, with a screenshot of the live server: *"the items that
        // drop should also be slightly staggered from each other"*. Three items landing on
        // exactly the same pixel render as one. Centred on the mob so a single drop is
        // exactly where it died, and spread outward from there.
        let n = rolled.len() as i16;
        for (i, r) in rolled.into_iter().enumerate() {
            let offset = (i as i16 - (n - 1) / 2) * crate::drops::DROP_STAGGER_PX;
            let x = x.saturating_add(offset);
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


    /// Give the character experience, level them up if it pays for one, and tell the client.
    ///
    /// **One path for every source of experience**, so a kill and `!exp` cannot drift: the
    /// levelling rule, the persistence and the `0x007C` all live here. `crate::expcurve`
    /// owns the arithmetic.
    ///
    /// The level-up animation comes free - the `0x007C` handler plays
    /// `Effect/BasicEff.img/LevelUp` itself when the level in the packet is higher than the
    /// one the client is holding, so there is no separate effect packet to send.
    ///
    /// Returns nothing at all for an award of zero, which is the ordinary case for a mob
    /// with no EXP value: a `0x007C` that changes nothing is a packet the client has to
    /// parse for no reason.
    pub(super) fn award_experience(&mut self, gained: u64, why: &str) -> Vec<Reply> {
        if gained == 0 {
            return Vec::new();
        }
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let before_level = chr.level;
        let a = self.config.exp_curve.award(chr.level, chr.exp, gained);
        chr.level = a.level;
        chr.exp = a.exp;
        if a.levels > 0 {
            // Gains apply to the maximums, and a level-up refills - which is this game's
            // behaviour and also the only reading under which the numbers cannot end up
            // above their own maximum.
            chr.max_hp += a.max_hp;
            chr.max_mp += a.max_mp;
            chr.hp = chr.max_hp;
            chr.mp = chr.max_mp;
            chr.ap += a.ap;
        }
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.notice(format!("Could not save your experience: {e}"));
        }

        let mut change = net::stats::StatChange::exp(chr.exp);
        if a.levels > 0 {
            change.level = Some(chr.level);
            change.max_hp = Some(chr.max_hp);
            change.max_mp = Some(chr.max_mp);
            change.hp = Some(chr.hp);
            change.mp = Some(chr.mp);
            change.ap = Some(chr.ap);
        }
        let mut out = vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: change.build(),
            what: format!(
                "StatChanged: +{gained} exp from {why} -> {} total{}. Bit 16 carries the NEW                  TOTAL; the client differences its own snapshot to draw the gain.",
                chr.exp,
                if a.levels > 0 {
                    format!(", LEVEL {before_level} -> {} (+{} ap)", chr.level, a.ap)
                } else {
                    String::new()
                }
            ),
        }];
        if a.levels > 0 {
            out.extend(self.notice(format!(
                "Level up! {} is now level {}. +{} AP, and HP/MP restored.",
                chr.name, chr.level, a.ap
            )));
        }
        out
    }


    /// A mob died: book its spawn point to refill.
    ///
    /// **The server respawns mobs and the client never does.** The owner, after a run: *"The mobs
    /// that I kill also do not respawn."* Before this, a map emptied permanently after one
    /// pass, because the only thing that ever sent a `MobEnterField` was field entry.
    ///
    /// The delay is the WZ's own `mobTime` for that spawn point - see
    /// [`crate::config::respawn_delay_ms`] for the three cases and why `0` must not be read
    /// as "never".
    pub(super) fn schedule_respawn(&mut self, map: u32, object_id: u32) {
        let mob_time = self.config.mob_respawn_s.get(&(map, object_id)).copied().unwrap_or(0);
        let Some(delay) = crate::config::respawn_delay_ms(mob_time) else {
            return; // the WZ says this spawn point never refills
        };
        self.dead_mobs.push((self.clock_ms.saturating_add(delay), map, object_id));
    }

    /// Refill every spawn point whose timer has come due. Called from [`Session::tick`].
    ///
    /// Sends the same pair a field entry does - `MobEnterField` then `MobChangeController` -
    /// because a mob the client has not been given control of is a picture that never moves,
    /// and the order matters (`research/mob-behaviour.md` §3).
    ///
    /// A respawned mob is at **full HP and its spawn position**, not where it died. That is
    /// what a spawn point is.
    pub(super) fn respawn_due_mobs(&mut self, now_ms: u64) -> Vec<Reply> {
        if self.dead_mobs.is_empty() {
            return Vec::new();
        }
        let Some(here) = self.claimed_character().map(|c| c.map_id) else { return Vec::new() };
        let mut out = Vec::new();
        let mut still_dead = Vec::with_capacity(self.dead_mobs.len());
        for (due, map, object_id) in std::mem::take(&mut self.dead_mobs) {
            if now_ms < due {
                still_dead.push((due, map, object_id));
                continue;
            }
            // A spawn point on a map the player has left is simply forgotten. The pool is
            // rebuilt from scratch on every field entry, so it will be full again when they
            // come back - re-sending it here would address a pool that no longer exists.
            if map != here {
                continue;
            }
            let Some(mob) = self
                .config
                .mobs
                .get(&map)
                .and_then(|list| list.iter().find(|m| m.object_id == object_id))
            else {
                continue;
            };
            self.mob_hp.insert(mob.object_id, mob.hp);
            self.mob_template.insert(mob.object_id, mob.template_id);
            out.push(Reply {
                opcode: net::mob::MOB_ENTER_FIELD,
                body: net::mob::mob_enter_field(mob),
                what: format!(
                    "MobEnterField: RESPAWN of template {} at ({}, {}), object id {}, hp {}.",
                    mob.template_id, mob.x, mob.y, mob.object_id, mob.hp
                ),
            });
            out.push(Reply {
                opcode: net::mobmove::MOB_CHANGE_CONTROLLER,
                body: net::mobmove::mob_change_controller(mob, net::mobmove::CONTROL_NORMAL),
                what: format!(
                    "MobChangeController: object id {} to this client after a respawn.",
                    mob.object_id
                ),
            });
        }
        self.dead_mobs = still_dead;
        out
    }
}
