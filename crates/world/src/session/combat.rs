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
        if let Some(map) = self.claimed_character().map(|c| c.map_id) {
            self.fields.note_position(map, req.object_id, (req.x, req.y));
        }
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
        let Some((chr_id, map)) = killer else { return out };
        for target in &attack.targets {
            let Some(hp_before) = self.fields.mob_hp(map, target.object_id) else {
                continue; // not a mob of ours, or already dead and removed
            };
            let template = self.fields.mob_template(map, target.object_id).unwrap_or(0);
            let damage = target.total_damage();
            let hit = net::combat::apply_damage(hp_before, damage);
            // The field owns the death: it removes the mob and books its spawn point to
            // refill from the WZ's own mobTime. Nothing here has to remember a corpse.
            // **Where it is, BEFORE it dies.** `hurt` removes the mob from the field, so
            // asking afterwards returns nothing and every drop fell back to the player's
            // feet - which is exactly what the owner saw twice. Read it first, hand it down.
            let died_at = self.fields.mob_position(map, target.object_id);
            let left = self.fields.hurt(map, target.object_id, damage, &self.config, self.clock_ms);
            if left.is_none() {
                out.extend(self.drops_from_kill(template, target.object_id, died_at, chr_id, map));
                let worth = self.config.mob_exp.get(&template).copied().unwrap_or(0);
                out.extend(self.award_experience(u64::from(worth), "a kill"));
                out.extend(self.credit_kill_to_quests(template, chr_id));
            }
            // The template's real maxHP, because 0x03F0 carries a PERCENTAGE.
            let max_hp = self
                .config
                .mobs
                .get(&map)
                .and_then(|l| l.iter().find(|m| m.object_id == target.object_id))
                .map(|m| m.hp)
                .unwrap_or(hp_before);
            for (opcode, body) in net::combat::mob_hit_replies(target.object_id, &hit, max_hp) {
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
        // Where the mob was when it died, read BEFORE it was removed from the field.
        died_at: Option<(i16, i16)>,
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
        let _ = object_id;
        let at = died_at.or(self.last_position);
        let Some((x, y)) = at else {
            return self.notice(
                "A mob died with drops to give, but the server does not know where it was standing, so it dropped nothing rather than putting it out of reach."
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
            let now = self.clock_ms;
            let reply = self.fields.with_drops(map, |d| {
                d.drop_from_mob(crate::drops::DropFromMob {
                    map_id: map,
                    owner_id: killer,
                    item,
                    inv_type,
                    meso,
                    x,
                    y,
                    now_ms: now,
                })
            });
            out.push(reply);
        }
        out
    }


    /// Spawn everything on this map whose timer is due, and tell the client.
    ///
    /// **This is both the first fill of a field and every refill after a kill** - see
    /// `crate::fields`, where a newly-registered field starts with every spawn point due
    /// rather than alive. The owner: *"on first enter, no mobs should exist until the respawn
    /// timer kicks in"* and *"The mobs that I kill also do not respawn."*
    ///
    /// Sends the same pair a field entry does, in the same order: a mob the client has not
    /// been given control of is a picture that never moves.
    pub(super) fn spawn_due_mobs(&mut self, map: u32, now_ms: u64) -> Vec<Reply> {
        let arrived = self.fields.due_respawns(map, &self.config, now_ms);
        let mut out = Vec::new();
        for live in arrived {
            let mob = live.as_seen();
            out.push(Reply {
                opcode: net::mob::MOB_ENTER_FIELD,
                body: net::mob::mob_enter_field(&mob),
                what: format!(
                    "MobEnterField: SPAWN of template {} at ({}, {}), object id {}, hp {}.",
                    mob.template_id, mob.x, mob.y, mob.object_id, mob.hp
                ),
            });
            out.push(Reply {
                opcode: net::mobmove::MOB_CHANGE_CONTROLLER,
                body: net::mobmove::mob_change_controller(&mob, net::mobmove::CONTROL_NORMAL),
                what: format!("MobChangeController: object id {} to this client.", mob.object_id),
            });
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
        // **The right-hand message area, not the chat log.** The owner: *"it should actually show
        // on the right hand side of the client. We should not be outputting in the chat log
        // regarding level ups and item pickups."*
        //
        // `0x0089` type 3 carries the number and the client composes the sentence from its
        // own string table - `You received EXP (+211)` in this build. The third field is what
        // chooses the destination: non-zero routes the line to the chat log, zero routes it
        // to the other place. `research/client-messages.md`.
        //
        // The level-up itself gets no message: there is no level-up type in the table, and
        // `0x007C` already plays the animation when the level in it is higher than the one
        // the client holds. That is a "did not find", not a "there is none".
        out.push(Reply {
            opcode: net::message::MESSAGE,
            body: net::message::exp_gained(gained),
            what: format!("Message: +{gained} exp, in the screen message area, not the chat log"),
        });
        out
    }


    /// `0x00E5` - the client says the player took damage. **Apply it.**
    ///
    /// The owner, after a run: *"Getting hit by the mob does not subtract my HP."* The client
    /// computes the damage and does **not** apply it - established three ways with different
    /// blind spots in `research/user-hit.md` - so nothing moved their bar because nothing was
    /// moving it. The server is the authority and has to send the new HP back.
    ///
    /// **The new HP, not a delta.** `0x007C` mask bit 10 carries an absolute value; sending
    /// a difference would make the bar wander.
    ///
    /// Not a latch, so an unparseable body costs one hit rather than the session - which is
    /// why this can afford to be strict about the length.
    pub(super) fn on_user_hit(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(hit) = net::userhit::parse_user_hit(payload) else {
            return Vec::new();
        };
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        if hit.damage == 0 {
            return Vec::new();
        }

        let before = chr.hp;
        chr.hp = chr.hp.saturating_sub(hit.damage);
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.notice(format!("Could not save your health: {e}"));
        }

        let mut out = vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange::hp_only(chr.hp).build(),
            what: format!(
                "StatChanged: hit by mob {} (template {}, attack index {}) for {} - hp {} -> {}.                  Bit 10 carries the NEW HP, not a delta; the client computes damage and does                  not apply it.",
                hit.mob_object_id, hit.mob_template_id, hit.attack_index, hit.damage, before,
                chr.hp
            ),
        }];

        // **Death is not built.** `research/user-hit.md` established that `hp = 0` in a
        // `0x007C` is not a death packet and will not hang the client - what it does is
        // disable the player through some 65 sites that branch on the sign of HP - but what
        // plays the death sequence was not found. So say so out loud rather than leaving a
        // character wedged at zero with no explanation.
        if chr.hp == 0 {
            out.extend(self.notice(
                "You are out of HP. Death is not implemented yet - use !heal to carry on."
                    .to_string(),
            ));
        }
        out
    }


    /// Count a kill against every started quest that asked for that mob.
    ///
    /// The owner: *"I accepted Sam's suggestion which requires Snail kills, but the quest is not
    /// progressing even when I kill snails."* Accepting was recorded; nothing counted.
    ///
    /// # The count is a string, and that is not a stylistic choice
    ///
    /// The client stores a quest's progress as **three zero-padded decimal characters per
    /// mob requirement**, concatenated in slot order - four snails is the three bytes
    /// `"004"`. `FUN_14070cb70` takes `substr(slot*3, slot*3+3)` and converts base 10. A
    /// count sent as an integer would render as nothing at all.
    ///
    /// # A kill fans out
    ///
    /// One template can be named by several quests - template 13 by six of them - so this
    /// cannot stop at the first match.
    ///
    /// # Item quests are not counted here, deliberately
    ///
    /// The client counts the bag live whenever it checks, so an item quest needs no running
    /// total and gets no packet. Only mob requirements have a counter, which is why the
    /// progress string's length is measured in mob slots.
    pub(super) fn credit_kill_to_quests(&mut self, template: u32, character_id: u32) -> Vec<Reply> {
        let quests = self.config.quest_reqs.quests_for_mob(template);
        if quests.is_empty() {
            return Vec::new();
        }
        let mut out = Vec::new();
        for &quest_id in quests {
            let Some(reqs) = self.config.quest_reqs.get(quest_id) else { continue };
            // Only a quest the player actually has. `quest_row` returns the in-progress row
            // or nothing, so a completed or unaccepted quest counts nothing.
            let Ok(Some(row)) = self.store.quest_row(character_id, quest_id) else { continue };
            if row.state != store::QuestState::InProgress {
                continue;
            }
            let Some(next) = net::quest::apply_kill(reqs, &row.progress, template) else {
                continue; // no requirement on this template, or the counter is already full
            };
            if self.store.set_quest_progress(character_id, quest_id, &next).is_err() {
                continue;
            }
            out.push(Reply {
                opcode: net::quest::MESSAGE,
                body: net::quest::quest_record(
                    quest_id,
                    &net::quest::QuestProgress::InProgress { progress: next.clone() },
                ),
                what: format!(
                    "QuestRecord: quest {quest_id} progress {next:?} after killing template                      {template}. Three zero-padded characters per mob slot - an integer here                      renders as nothing."
                ),
            });
        }
        out
    }
}
