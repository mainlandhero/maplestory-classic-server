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
        self.note_activity();
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
            let left =
                self.fields.hurt(map, target.object_id, damage, chr_id, &self.config, self.clock_ms);
            if let crate::fields::Hurt::Died(shares) = left {
                out.extend(self.drops_from_kill(template, target.object_id, died_at, chr_id, map));
                let (worth, why) = self.exp_for_kill(template);
                out.extend(self.award_kill_experience(worth, &why, chr_id, &shares));
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
        // The server's drop rate scales every chance in the table. Read before the borrow
        // below, because it is a database query and `roll_at` holds `self.rng`.
        let drop_rate = self.rate(store::rates::RateKind::Drop);
        let rolled = {
            let rng = &mut self.rng;
            self.config.drops.roll_at(template, drop_rate, &mut || rng.next())
        };
        let mut out = Vec::new();
        // Read the meso rate ONCE, not once per drop: it is a database query, and it cannot
        // change between two items falling off the same mob.
        let meso_rate = self.rate(store::rates::RateKind::Meso);
        // **Stagger them.** The owner, with a screenshot of the live server: *"the items that
        // drop should also be slightly staggered from each other"*. Three items landing on
        // exactly the same pixel render as one. Centred on the mob so a single drop is
        // exactly where it died, and spread outward from there.
        let n = rolled.len() as i16;
        // The corpse, bound before the loop shadows `x` with the staggered landing spot.
        // Both ends of the arc have to exist at once or there is no arc.
        let (mob_x, mob_y) = (x, y);
        for (i, r) in rolled.into_iter().enumerate() {
            let offset = (i as i16 - (n - 1) / 2) * crate::drops::DROP_STAGGER_PX;
            let x = x.saturating_add(offset);
            // **Put it on the floor.** The owner, 2026-08-21: *"item drops still go through the
            // ground and are unable to be picked up."* The stagger above moves the item
            // sideways and nothing ever moved it vertically, so on any ground that is not
            // flat it ends up in mid-air or inside terrain - and the client's pick-up box
            // reaches only 10 px below the player's feet, so such an item is drawn and
            // uncollectable. A third of this client's 94089 floor segments are sloped.
            //
            // **Two different fallbacks, and the difference matters.** With no table loaded
            // at all, keep the staggered position - that is exactly what shipped before this
            // existed, and collapsing every drop onto the corpse would be a visible
            // regression in the case where we simply have no data. With a table loaded, a
            // `None` means the vertical through `x` meets no surface anywhere on the map, and
            // there the corpse is strictly better: a mob was standing on it, so it is
            // certainly a floor.
            let fallback = if self.config.footholds.is_empty() { (x, y) } else { (mob_x, mob_y) };
            let placed = self.config.footholds.landing(map, x, y);
            let (x, y) = placed.map(|l| (l.x, l.y)).unwrap_or(fallback);
            let (item, inv_type, meso) = if r.is_mesos() {
                // A placeholder item: `LiveDrop::is_meso` gates every read of it.
                // **The meso rate multiplies the pile on the floor, not the credit on
                // pick-up.** Either would show the right number in the end, but this way the
                // amount in the drop, the amount in the message and the amount added to the
                // balance are all the same number, and a run that disagrees with itself is
                // the kind of evidence this project keeps having to re-gather.
                let amount = meso_rate.apply(u64::from(r.quantity)).min(u64::from(u32::MAX)) as u32;
                (store::Item::bundle(0, 0), store::InventoryType::Etc, amount)
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
                    // The corpse, un-staggered: where the arc starts.
                    source_x: mob_x,
                    source_y: mob_y,
                    now_ms: now,
                })
            });
            // The landing goes in the log line, so "did the placement do anything" is a
            // `world.log` grep instead of a second manual launch. Only when it actually
            // moved the item - an unmoved drop on flat ground is the common case and would
            // bury the interesting ones.
            let mut reply = reply;
            if let Some(l) = placed.filter(|l| l.moved != 0) {
                reply.what.push_str(&format!(" [{}]", l.what()));
            }
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
            let mut mob = live.as_seen();
            // This one really is arriving while the player watches, so it keeps the spawn
            // effect. The pair of these two lines IS the feature - one value for "was already
            // here", another for "just turned up".
            mob.appear_type = net::mob::APPEAR_SPAWNING;
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
    /// What a kill of `template` is worth after the server's EXP rate, and the phrase that
    /// goes in the log beside it.
    ///
    /// **A seam, not a convenience.** The multiplication used to be two lines at the call
    /// site inside the attack handler, where the only way to test it was to build a
    /// 147-byte attack packet. Here a test can ask directly.
    ///
    /// The rate is global and lives in the database, because a channel is a process -
    /// `store::rates`. **`!exp` deliberately does not come through here**: it is a debugging
    /// command that means "give me exactly this much", and one that quietly doubled would be
    /// useless for checking the curve.
    pub(super) fn exp_for_kill(&self, template: u32) -> (u64, String) {
        let base = self.config.mob_exp.get(&template).copied().unwrap_or(0);
        let rate = self.rate(store::rates::RateKind::Exp);
        let worth = rate.apply(u64::from(base));
        let why = if rate.is_normal() {
            "a kill".to_string()
        } else {
            format!("a kill ({base} at {rate}x)")
        };
        (worth, why)
    }


    /// Award experience for a kill, splitting it by who actually did the damage.
    ///
    /// The owner, 2026-08-20, with a screenshot: *"if I was the person who dealt majority damage,
    /// I should see a white line of EXP gained. If I was not the person who dealt majority
    /// damage, I would only get a % portion of the EXP that belonged to the mob ... and that
    /// line would be yellow."*
    ///
    /// **This connection only ever pays itself.** The share list names every contributor, but
    /// there is no way to push a packet to another player's thread - a channel is a process
    /// and each connection is its own thread with its own socket, `crates/world/src/server.rs`.
    /// So each session finds *itself* in the list and pays its own cut. With one player that
    /// is the whole list; with two, each pays itself the moment it kills something, and the
    /// second player's share of a kill they helped with is **not yet delivered**. Said out
    /// loud rather than left to look finished: `Fields::hurt` already returns the whole split
    /// and it is only the delivery that is missing.
    ///
    /// **Parties do not exist**, so the 70/30 split and `You received party EXP` are not
    /// implemented. `research/exp-sharing.md` records the rule so it does not have to be
    /// asked for twice.
    pub(super) fn award_kill_experience(
        &mut self,
        worth: u64,
        why: &str,
        chr_id: u32,
        shares: &[crate::fields::DamageShare],
    ) -> Vec<Reply> {
        // Nothing credited means nothing landed - a mob that died without being hurt, which
        // only a bug produces. Pay the killer in full rather than nothing.
        let Some(mine) = shares.iter().find(|s| s.character == chr_id) else {
            return self.award_experience(worth, why, true, false);
        };
        let cut = mine.cut_of(worth);
        let why = if mine.majority {
            why.to_string()
        } else {
            format!("{why}, {}/{} of the damage", mine.dealt, mine.total)
        };
        self.award_experience(cut, &why, mine.majority, false)
    }


    pub(super) fn award_experience(
        &mut self,
        gained: u64,
        why: &str,
        white: bool,
        to_chat: bool,
    ) -> Vec<Reply> {
        if gained == 0 {
            return Vec::new();
        }
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let before_level = chr.level;
        // The job picks the HP/MP line. Every character is a beginner today - job
        // advancement is goal E - so this is +16/+12 in practice, and the other four lines
        // are there so levelling does not have to be revisited when E lands.
        let a = self.config.exp_curve.award(chr.job, chr.level, chr.exp, gained);
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
        // **Where the line lands is not the same for a kill and a quest.** The owner, 2026-08-21:
        // *"the quest completion EXP should not show up the same way as mob EXP. Quest EXP
        // and items should show up in the chat log as a gray text."*
        //
        // Same packet, one flag: `in_chat` non-zero posts to `FUN_1415eca30(text, 6)`, the
        // chat log, instead of the on-screen singleton. **[L]** for both destinations.
        out.push(Reply {
            opcode: net::message::MESSAGE,
            body: if to_chat {
                net::message::exp_gained_in_chat(gained, white)
            } else {
                net::message::exp_gained(gained, white)
            },
            what: format!(
                "Message: +{gained} exp, {}, to the {}",
                if white { "WHITE (majority damage)" } else { "yellow (a share)" },
                if to_chat { "CHAT LOG (type 6)" } else { "screen message area" }
            ),
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
        // Being hit counts as activity: standing in a mob's path is not resting.
        self.note_activity();
        if hit.damage == 0 {
            return Vec::new();
        }

        // **The server decides how much this hurt, and it did not used to.**
        //
        // The owner, 2026-08-21: *"all mobs should not only just deal 1 damage to the player."*
        // Every one of the twelve snail hits in that capture carried `damage = 1` - and this
        // handler applied the client's number verbatim, so the server had no opinion at all.
        //
        // The client's own data says a snail should hurt more than that. Template 2's
        // `PADamage` is **3**, and `damage::incoming_damage` over that character's real
        // numbers (level 7, 18 points of `incPDD`) gives **3 or 4**. `damage.rs` carries
        // that as a pinned test.
        //
        // So the two disagree, and the server's number is the one that moves the HP bar.
        // **Both go in the log line**, because that disagreement is the measurement the next
        // run is for: if the floating number over the player's head says one thing and the
        // bar drops by another, that is worth seeing rather than guessing about.
        //
        // Falls back to the client's number when the template is unknown or carries no
        // attack column - a mob we have no data for should still hurt.
        let claimed = hit.damage;
        let computed = self.incoming_damage_for(hit.mob_template_id, &chr);
        let applied = computed.unwrap_or(claimed);

        // **Magic Guard sends part of the damage to MP, and that split is the SERVER's job.**
        //
        // `research/magic-damage.md`: the client computes `mpLoss = guard% * damage / 100`
        // clamped to current MP - the hit handler reads `secStat+0x614`, multiplies, divides
        // by 100 and clamps - but it **never writes HP**. `user-hit.md` enumerated every write
        // to the HP field, closed the `lea`-handoff blind spot, and walked the hit path to
        // depth 8 without finding one. Only `0x007C` moves either bar.
        //
        // So setting CTS bit 97 buys an icon and nothing else. Without this block the buff
        // would look active, cost MP to cast, and change nothing about how much a hit hurt -
        // which is the shape of bug that gets reported as "the skill does nothing".
        //
        // **Clamped to current MP**, like the client's own arithmetic: what MP cannot absorb
        // still comes off HP. A player at 0 MP with Magic Guard up takes the full hit, which
        // is the behaviour the clamp in the client produces too.
        let guard_percent = self.magic_guard_percent();
        let to_mp = if guard_percent > 0 {
            (u64::from(applied) * u64::from(guard_percent) / 100).min(u64::from(chr.mp)) as u32
        } else {
            0
        };
        let to_hp = applied - to_mp;

        let before = chr.hp;
        chr.mp = chr.mp.saturating_sub(to_mp);
        chr.hp = chr.hp.saturating_sub(to_hp);
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.notice(format!("Could not save your health: {e}"));
        }

        // **HP and MP in ONE packet.** `hp_only` was right until Magic Guard existed; sending
        // two packets would let the client draw the HP bar against a stale MP value, and the
        // revive dialog below gates on the HP this packet sets.
        let mut out = vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange {
                hp: Some(chr.hp),
                mp: (to_mp > 0).then_some(chr.mp),
                ..Default::default()
            }
            .build(),
            what: format!(
                "StatChanged: hit by mob {} (template {}, attack index {}) for {applied} - hp {before} -> {}{}. The CLIENT claimed {claimed}{}. Bit 10 carries the NEW HP, not a delta.",
                hit.mob_object_id,
                hit.mob_template_id,
                hit.attack_index,
                chr.hp,
                if to_mp > 0 {
                    format!(", and MAGIC GUARD sent {to_mp} of it to MP ({guard_percent}%) -> {} mp", chr.mp)
                } else {
                    String::new()
                },
                match computed {
                    Some(_) if claimed != applied => " and the server overrode it",
                    Some(_) => " and the server agreed",
                    None => " and the server had no template to check it against",
                }
            ),
        }];

        // **Death.** The owner, 2026-08-21: *"My HP hit 0, I see the tombstone on my character,
        // but I do not see the revive confirmation."*
        //
        // The tombstone was already working: `hp = 0` in the `0x007C` above disables the
        // player through the ~65 sites `research/user-hit.md` §6.2 enumerates. What was
        // missing is the dialog, and **the client never opens it by itself** - a packet does,
        // `0x0315`, and `research/revive.md` traced it through a vtable slot with exactly one
        // construction site.
        //
        // **Order matters and it is not cosmetic.** The handler gates on a client-side HP
        // test of the value the server just wrote (`world[0x2358]`, the field `0x007C` bit 10
        // sets), so a `0x0315` that overtakes the `0x007C` is dropped **silently**. It goes
        // after, in the same batch.
        //
        // **Gated on the transition, not on the state.** `before > 0 && chr.hp == 0` fires
        // once; `chr.hp == 0` alone would re-open the dialog on every subsequent hit, and a
        // dead character can still be hit. That is the same rule the quest payouts had to
        // learn: every effect hangs off the transition.
        if before > 0 && chr.hp == 0 {
            out.push(Reply {
                opcode: net::revive::SHOW_REVIVE_DIALOG,
                body: net::revive::show_revive_dialog(),
                what: format!(
                    "ShowReviveDialog: character {} died to mob {} (template {}). 0x0315, and it MUST follow the 0x007C above - the client gates it on its own copy of the HP the server just set, and drops it silently if that is still positive. Clicking REVIVE IN TOWN sends 0x00D1 with targetField 0",
                    chr.id, hit.mob_object_id, hit.mob_template_id
                ),
            });
        }

        // **The damage number the player actually took.**
        //
        // The owner, 2026-08-27: *"When I died to the Drakes, I still visually took 1 damage, but
        // it wiped out my whole HP bar."* They are right, and this is the packet the comment
        // above asked for when it said the disagreement "is the measurement the next run is
        // for".
        //
        // # The client's own number is a stub, and that is now measured rather than suspected
        //
        // 224 `0x00E5` bodies across 22 capture files, decoded with a passing control: mob
        // templates whose `PADamage` runs from **3 to 287** - a 96x spread - every one of them
        // reporting **damage = 1**. There is an explicit `max(damage, 1)` floor at
        // `0x1428AB959` and all sixteen mob-family call sites hand it `nDamage = 0`. The
        // client is not computing a number we could correct; it never had one.
        //
        // # The colour is the SIGN, and `recovery_number` already carries it
        //
        // `FUN_142771360` forks at `0x142771395`: a positive amount picks digit set 2
        // (`NoBlue`), a negative one picks set 3 (`NoViolet`) - and set 3 is what the client's
        // own hit builder reaches with a non-positive argument at `0x1428ACA14`. So the blue
        // recovery number and the damage number are not merely the same renderer by analogy;
        // they are the same call with the sign flipped. `net::revive::recovery_number` already
        // preserves the sign, and its own test says why.
        //
        // Everything upstream of the fork is proven by the owner seeing the blue `+10`.
        //
        // # THIS DOES NOT DELETE THE CLIENT'S `1`, AND THAT IS THE COST
        //
        // The stub number is drawn at *send* time, before this packet exists, so the screen
        // will show **both**. The flag that suppresses it, `user+0x544a`, has exactly one
        // writer and it is in a function with no packet reads - no server route was found,
        // and the search that failed named its own blind spots.
        //
        // Two numbers is worse than one *correct* number and better than one *wrong* one, so
        // it goes in and the run says whether it reads acceptably. It is one push to remove.
        // The only route to a single correct number is `forcedStatPresent` at offset 10 of the
        // spawn body - sent as `0` today, never tried, and its parser reads twelve `u32` stat
        // overrides of which none is yet known to be attack power.
        out.push(Reply {
            opcode: net::stats::USER_EFFECT_LOCAL,
            body: net::revive::recovery_number(-(applied as i32), 0),
            what: format!(
                "UserEffectLocal effect 0x41 with a NEGATIVE amount: -{applied} over the \
                 player's head, in the damage colour. The sign is what selects it - positive \
                 is the blue recovery number the owner has already seen. The client ALSO draws its \
                 own stub 1, at send time, and nothing here can suppress it, so expect two \
                 numbers"
            ),
        });
        out
    }


    /// What a hit from `template` should actually take off, or `None` if we cannot say.
    ///
    /// `None` means the template is not in `gm-handbook/mobtemplates.txt` or carries no
    /// attack column, and the caller then keeps the client's number. That is the safe
    /// direction: a mob we have no data for should still hurt.
    ///
    /// # The defence is the player's worn `incPDD`, summed
    ///
    /// It is read off the equipped items rather than stored on the character, for the same
    /// reason [`Session::dressed`] resolves stats from the template: `crates/store` keeps
    /// `(slot, itemId)` and a defence column would be a second source of truth for a number
    /// the client already has its own copy of.
    ///
    /// At this scale defence barely matters and that is worth knowing before reading a run:
    /// 18 points of `incPDD` against a denominator of `5 * (level + 40)` is noise, so a
    /// naked character and a dressed one take the same from a snail. The term earns its keep
    /// later, not now.
    fn incoming_damage_for(
        &mut self,
        template: u32,
        chr: &net::opcode::Character,
    ) -> Option<u32> {
        let pa_damage = self.config.mob_attack.get(&template).copied().unwrap_or(0);
        if pa_damage == 0 {
            return None;
        }
        let wdef: u32 = self
            .dressed(chr)
            .iter()
            .map(|(_, _, stats)| u32::from(stats.stats.inc_pdd))
            .sum();
        // A roll in [1.1, 1.5), the half-open span `damage::incoming_window` pins both ends
        // of. Taken from the session rng so a run is varied and a test can seed it.
        let span = crate::damage::INCOMING_ROLL_SPAN;
        let roll = crate::damage::INCOMING_ROLL_LO
            + (self.rng.next() % 10_000) as f64 * span / 10_000.0;
        Some(crate::damage::incoming_damage(pa_damage, chr.level, wdef, roll))
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
