//! Where the player is: field entry, portals, `!map`'s destination, Log Out, and the
//! channel switch.
//!
//! Split out of `session.rs` rather than rewritten - every method here kept its body and
//! its doc block. The file boundary is the only new thing.

use super::*;

impl Session {

    /// Populate the field the client has just finished entering.
    ///
    /// The client does **not** spawn NPCs from the map WZ - its field loader walks `life`
    /// only to preload art. The only code that builds a populated NPC takes a packet, so
    /// every NPC on every field is ours to send, and ours to re-send after every `SetField`
    /// because the pool is destroyed and rebuilt empty on each field entry.
    ///
    /// `research/npc-spawn.md` has the working, including how the routing was found.
    pub(super) fn on_field_entered(&mut self) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        // **A field entry means the client is in the field stage, whatever route it took.**
        // `on_cash_shop_exit` already clears this before its `SetField`; clearing it here as
        // well is the one that cannot be forgotten, because it does not depend on the Exit
        // button being the way out. See `Session::in_cash_shop`.
        self.in_cash_shop = false;
        // The NPC pool is destroyed and rebuilt on every field entry, so the chatter cursors
        // go with it: an object id from the previous map addresses nothing here, or worse,
        // addresses a different NPC.
        self.reset_chatter(chr.map_id, self.clock_ms);
        // **MEASURED AND REFUSED, 2026-08-28. The packet is no longer sent.**
        //
        // The owner: *"The 1 damage from mobs still show up, I do see /hitdamagetest 0 echoed in
        // chat."* Both halves matter, and the plan had already written down what they mean
        // together: the echo happens **before** the dispatch, so seeing it proves only that
        // the string arrived. The proof that the command *ran* is an inbound `0x0189`
        // carrying `u32 0x13D`, and `world.log` for that run has **none** - the two matches
        // for "0189" in it are our own outbound log line and a coincidence inside a move
        // packet's hex.
        //
        // So the client's permission gate refused `/hitdamagetest`, which was always the
        // `[D]` in this chain. Continuing to send it would put a line in the owner's chat window
        // every session and change nothing, and a visible no-op is worse than no attempt:
        // next session somebody sees the echo and concludes it worked.
        //
        // **The fallback is a five-byte hook patch** at `0x1428aca14`, written up in
        // `research/damage-number-suppress.md`. That is a client patch rather than a packet,
        // so it belongs to the hook and the launcher, not here.
        let mut out = Vec::new();
        // **Other players, before the NPCs and mobs.** `0x00DC` arrives once per
        // field entry, every time, and the client destroys and rebuilds its pools
        // on every `SetField` - so the user pool has to be refilled after each one
        // for exactly the reason the NPC re-send below does.
        //
        // This both announces us to everyone here and returns everyone here to us,
        // in one call, because half of a mutual sighting is invisible on one
        // screen. `crate::session::multiplayer`.
        out.extend(self.announce_field_entry());
        // The summoned pet, if any - the pools it lived in were just rebuilt. session/pet.rs.
        out.extend(self.pet_entry_replies(&chr));
        // The party window, once, for a member who was in a party when they last left: seats
        // persist across a disconnect. session/party.rs.
        if !self.party_window_sent {
            self.party_window_sent = true;
            out.extend(self.party_window_on_login());
        }
        let empty: Vec<net::opcode::FieldNpc> = Vec::new();
        out.extend(self.config.npcs.get(&chr.map_id).unwrap_or(&empty)
            .iter()
            .map(|npc| Reply {
                opcode: net::opcode::NPC_ENTER_FIELD,
                body: net::opcode::npc_enter_field(npc),
                what: format!(
                    "NpcEnterField: template {} at ({}, {}) on foothold {}, object id {} - the client cannot spawn this itself, it only preloads the art.",
                    npc.template_id, npc.x, npc.cy, npc.fh, npc.object_id
                ),
            }));

        // **The field clock, for the maps that have one.** The owner, 2026-09-09, in Ellinia
        // Station: *"the server clock does not seem to work. It just stays on 00:00."* The
        // client builds the widget itself from the map's `clock` node at field entry and
        // then waits for `0x01BC` to tell it the time. Nothing ever did. `net::clock` has
        // the decode.
        //
        // **Gated on the map, not sent everywhere**: the client's type-1 arm fetches the
        // widget with no null check and the fetch throws when the map built none.
        // `Config::clocks` is the list of maps whose image declares the node.
        //
        // **UTC, which is server time.** The first version sent the machine's local time and
        // The owner saw EDT on the wall: *"this needs to read the UTC time."* `crate::serverclock`.
        // It is also what every `world.log` stamp shows, so the wall and the log agree.
        // Whatever was held for this field: a revive's Safety Charm lines, which the client
        // drops while it has no field. By now it has one.
        out.append(&mut self.after_field_entry);
        if self.config.clocks.contains(&chr.map_id) {
            let (h, m, s) = crate::serverclock::utc_hms();
            out.push(Reply {
                opcode: net::clock::FIELD_CLOCK,
                body: net::clock::clock_hms(h, m, s),
                what: format!(
                    "FieldClock: type 1, {h:02}:{m:02}:{s:02} UTC - map {} declares a clock node, so the widget exists to receive it",
                    chr.map_id
                ),
            });
        }

        // Mobs, from the same WZ `life` walk that produced the NPCs and for the same
        // reason: the client's field loader only preloads `Mob/%07d.img` art, and the pool
        // is destroyed and rebuilt empty on every field entry, so they must be re-sent
        // after every SetField rather than once.
        //
        // **OFF BY DEFAULT since the run of 2026-08-19, because the body faults the
        // client.** TestCharD entered map 40 (Snail Hunting Ground I, 40 spawns); the two
        // NPCs dispatched cleanly and the client then died on the FIRST 0x03C6 with
        // 0xC0000005 at `0x141c810b0`. That is inside `FUN_141c81040`, and the faulting
        // instruction is `CMP qword ptr [RCX],RDX` after
        // `MOV RAX,[RSI+0x2b8] / LEA RCX,[RAX+0x828] / CMOVE RCX,RDX` - so **`mob+0x2b8`
        // was null** and the client dereferenced without a guard.
        //
        // Everything else on that entry worked, which is what makes the diagnosis narrow:
        // the character record decoded, the equipped list decoded all four items, and both
        // NPCs went through. Mobs are the only thing that changed the outcome.
        //
        // Turn back on with `--mobs` when the body is the variant under test.

        // **The field belongs to the channel, not to this visit.** Registering it is a
        // no-op after the first time, and a brand-new field is EMPTY: every spawn point
        // starts due and the ordinary respawn tick fills them in over the next few seconds.
        // The owner: *"on first enter, no mobs should exist until the respawn timer kicks in."*
        //
        // So what goes out here is whatever is alive RIGHT NOW, at its current position -
        // which for a returning player is where the mobs actually wandered to, not their
        // spawn points. `crate::fields`.
        // The party quest's last stage fills all at once instead. session/firsttime.rs.
        self.fill_last_stage(&chr);
        self.fields.seed(self.field_of(&chr), &self.config, self.clock_ms);
        // **Who controls what, decided before a single packet is built.**
        //
        // This loop used to push a `MOB_CHANGE_CONTROLLER` for **every** mob to **every**
        // arriving session, and `crate::fields` had no registry at all - so two players on one
        // map were two clients each rolling their own wander for the same monster
        // (`research/mob-behaviour.md` §5.1: the path comes out of the client's own random
        // source, one call per element). The two screens diverged on the first step.
        //
        // Three calls, in this order, and the order matters:
        //
        // * `release_map` - **our own** claims on this field. The client has just torn its mob
        //   pool down, so every grant it held is void and has to be re-sent. Without this, a
        //   player returning to a map they already control gets `0x03C6` for mobs and no
        //   `0x03D2` for any of them, and every monster stands still forever.
        // * `reconcile` against the whole live list, which is its documented precondition. A
        //   non-zero return means a `forget` was missed on some death path, so it is logged
        //   rather than discarded.
        // * `claim_uncontrolled`, which is a test-and-set: the first player on a map takes all
        //   of them and the second takes none and is a spectator.
        //
        // **Lock order.** `mobs_on` returns owned data and its guard is gone before the
        // registry is touched - `crate::fields::Fields` says why that rule exists.
        let me = self.subscriber.get();
        let live_mobs = self.fields.mobs_on(self.field_of(&chr));
        let alive: Vec<u32> = live_mobs.iter().map(|m| m.spawn.object_id).collect();
        self.fields.controllers().release_map(self.field_of(&chr), me);
        let ghosts = self.fields.controllers().reconcile(self.field_of(&chr), &alive);
        let mine = self.fields.controllers().claim_uncontrolled(self.field_of(&chr), me, &alive);
        crate::server::log(&format!(
            "   map {} has {} mob(s); this connection now controls {}{}",
            chr.map_id,
            alive.len(),
            mine.len(),
            if ghosts > 0 {
                format!(" ({ghosts} STALE entries dropped - a forget was missed on a death path)")
            } else {
                String::new()
            }
        ));
        // The breakable boxes, before the mobs: scenery first. session/reactor.rs.
        out.extend(self.reactor_entry_replies(self.field_of(&chr)));
        for live in live_mobs {
            let mut mob = live.as_seen();
            // Already on the field when you walked in - no spawn effect. The owner: *"if the
            // destination map has mobs, they should show up instantly. Currently I see those
            // mobs fade in."*
            mob.appear_type = net::mob::APPEAR_ALREADY_THERE;
            // What the client needs to compute a contact hit at all. See `forced_stat_for`.
            mob.forced_stat = self.forced_stat_for(mob.template_id);
            out.push(Reply {
                opcode: net::mob::MOB_ENTER_FIELD,
                body: net::mob::mob_enter_field(&mob),
                what: format!(
                    "MobEnterField: template {} at ({}, {}) - its CURRENT position, object id {}, hp {}. The client cannot spawn this itself.",
                    mob.template_id, mob.x, mob.y, mob.object_id, mob.hp
                ),
            });
            // Granting control is what makes it move: the server does not drive mob
            // movement, it hands the mob to a client which then runs the wander locally and
            // reports each path back as 0x02FF. Order matters - after its MobEnterField,
            // per research/mob-behaviour.md section 3 - and the level must not be 0, which
            // despawns rather than releases.
            //
            // **Only for the mobs this connection actually claimed.** Everything else on the
            // map is drawn here and simulated somewhere else; it moves on this screen because
            // its controller's `0x02FF` is rebroadcast as `0x03D9` (`session/combat.rs`).
            if !mine.contains(&mob.object_id) {
                continue;
            }
            out.push(Reply {
                opcode: net::mobmove::MOB_CHANGE_CONTROLLER,
                body: net::mobmove::mob_change_controller(&mob, net::mobmove::CONTROL_NORMAL),
                what: format!(
                    "MobChangeController: object id {} to this client, which claimed it. The client runs the mob's movement and reports it as 0x02FF.",
                    mob.object_id
                ),
            });
        }

        // The character is dressed by the SetField record itself now, not from here. This
        // used to push a 0x0138 UserAvatarModified as a guess at the equipment problem;
        // that opcode is **dead code at byte level** - its apply is guarded by a call to
        // 0x1407f5ce0, which is three bytes of `xor eax,eax; ret`, followed by TEST/JZ. No
        // trigger and no timing would ever have made it work, so sending it was noise in
        // the log. See net::opcode::USER_AVATAR_MODIFIED and research/naked-character.md.

        // The floor. Drops already lying here have to be re-sent for the same reason mobs
        // do: the pool is destroyed and rebuilt empty on every field entry, so an item
        // dropped before a map change would otherwise be invisible on the way back - and an
        // invisible drop is one the player walks over without ever sending the pick-up
        // request this feature is waiting to see.
        //
        // **And only this player's own drops.** The owner, 2026-09-01: *"the drops can remain per
        // client."* Re-sending the whole floor was right while one character could be on a
        // map at a time, and is a leak the moment drops are owner-scoped - the client reads
        // `ownType` into `drop+0x70` and never tests it again (`net::drops`, **[L]**), so who
        // is sent the `0x046E` is the only thing that decides who can take the item.
        //
        // **This is the line the party seam was left open for**, and it is wired now.
        // `crate::mobshare::may_see_drop` is the predicate and `drops.rs` applies it; the only
        // question here is who counts as "us", and `Session::party_for` answers it from the
        // channel's real membership instead of assuming everyone is alone.
        let (map, now) = (self.field_of(&chr), self.clock_ms);
        let party = self.party_for(chr.id);
        let who = chr.id;
        out.extend(self.fields.with_drops(map, |d| d.field_entry(map, now, who, &party)));
        out.extend(self.restore_bag_and_mesos());
        // The party quest's countdown, if this entry is into one. session/firsttime.rs.
        out.extend(self.party_quest_clock());
        // And its gate, if this run already cleared the stage being entered.
        out.extend(self.party_quest_gate());

        // **A character who was already dead when they arrived gets the dialog here.**
        //
        // The owner, 2026-08-22: *"My character 'Idiot' has 0 HP from last time, and I don't see
        // any revive dialogue when I login because I immediately spawned in dead."*
        //
        // `on_user_hit` opens the dialog on the **transition** from alive to dead, which is
        // right for combat and is the whole reason a dead character is not re-prompted on
        // every further hit. But logging in dead is not a transition - the HP was already 0
        // in the database - so nothing fired, and the player was stranded with no way out but
        // `!heal`. Death has to be recoverable from both directions or it is a trap.
        //
        // **This is the right moment, and that is measured rather than hoped.** Field entry
        // runs `FUN_142caa4e0`, which tears down dialogs silently - which is why `CLAUDE.md`
        // forbids sending a script with or just before a `SetField`. But `0x00DC` is emitted
        // from *inside* the `SetField` handler, and the client dispatches nothing until that
        // handler returns ~586 ms later, so a reply to `0x00DC` lands about a millisecond
        // after field entry has finished resetting everything. `research/npc-preload.md` §4.
        //
        // The `0x007C` goes first for the same reason it does in combat: the `0x0315` handler
        // tests the client's own copy of the HP and drops the packet **silently** if it is
        // still positive. The record in the `SetField` already carries `hp = 0`, so this is
        // belt and braces - but it costs 13 bytes and the failure it prevents is invisible.
        if chr.hp == 0 {
            out.push(Reply {
                opcode: net::stats::STAT_CHANGED,
                body: net::stats::StatChange::hp_only(0).build(),
                what: format!(
                    "StatChanged: character {} entered the field already DEAD - restating hp 0 so the revive dialog's own HP test cannot miss it",
                    chr.id
                ),
            });
            out.push(Reply {
                opcode: net::revive::SHOW_REVIVE_DIALOG,
                body: net::revive::show_revive_dialog(),
                what: format!(
                    "ShowReviveDialog on field entry: character {} arrived on map {} with 0 HP. Logging in dead is not a death TRANSITION, so on_user_hit never fires for it - without this the character is stranded with no way out but !heal",
                    chr.id, chr.map_id
                ),
            });
        }
        out
    }


    /// Tell the client about the bag and the meso balance it cannot read from the record.
    ///
    /// # This is why the owner's Etc items and mesos "did not persist"
    ///
    /// They persisted perfectly. `store::inventory` had every one of them and still does -
    /// what was missing is that **nothing ever told the client**, so a relog showed an empty
    /// Etc tab and 0 mesos over a database that held neither.
    ///
    /// Two separate reasons, both already written down elsewhere in this repo and neither
    /// noticed to be a bug:
    ///
    /// * **The character record cannot carry them.** `crates/net/src/bag.rs`: the Equip tab
    ///   rides `presence[2]` and is sent, but the Use / Set Up / Etc / Cash bags sit behind
    ///   presence bytes 3, 4, 5 and 6, and bytes 3/4/5 each open a further **undecoded**
    ///   block. Nobody has decoded them, so nobody could fill them.
    /// * **The stat block has no meso field at all.** `session::ground` says so where it
    ///   credits a meso drop: `0x007C` bit 18 is *"the ONLY way this client is ever told a
    ///   meso balance"*.
    ///
    /// So this does not decode anything. It re-sends what already works: `0x0070` mode 0 -
    /// the same packet `!item` and a shop purchase use to put a stack in a bag without a
    /// field re-entry - and one `0x007C` for the balance.
    ///
    /// **Equips are deliberately skipped.** They come through the record, and sending them
    /// twice would put a second copy of every item in the tab.
    /// Move every cash equip out of the Equip tab into the Deco tab. See the call site.
    fn relocate_cash_equips(&mut self, character_id: u32) -> Vec<Reply> {
        let strays: Vec<store::InvItem> = self
            .store
            .bag_items(character_id, store::InventoryType::Equip)
            .unwrap_or_default()
            .into_iter()
            .filter(|row| self.config.tab_for(row.item.item_id) == Some(store::InventoryType::Deco))
            .collect();
        let mut out = Vec::new();
        for row in strays {
            let Ok(Some(free)) = self.store.free_slot(character_id, store::InventoryType::Deco) else {
                out.extend(self.notice(format!(
                    "{} is a cash equip and belongs in your Deco tab, which is full; it stays in the Equip tab for now.",
                    row.item.item_id
                )));
                continue;
            };
            if self.store.set_inventory_slot(character_id, store::InventoryType::Deco, free, &row.item).is_err() {
                continue;
            }
            let _ = self.store.clear_inventory_slot(character_id, store::InventoryType::Equip, row.slot);
            crate::server::log(&format!(
                "   inventory: character {character_id}: cash equip {} moved from Equip slot {} to Deco slot {free} (it was placed by leading digit before 2026-09-11)",
                row.item.item_id, row.slot
            ));
        }
        out
    }

    pub(super) fn restore_bag_and_mesos(&mut self) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let mut out = Vec::new();
        // **A cash equip in the Equip tab is moved to the Deco tab first.** Until 2026-09-11
        // every 1xxxxxx id went to the Equip tab, and the Übel set's four are sitting there
        // in the owner's database; the client keeps cash equips in tab 6 and every request it
        // builds about one names that tab. Idempotent, one query when there is nothing to do.
        out.extend(self.relocate_cash_equips(chr.id));
        for inv in [
            store::InventoryType::Use,
            store::InventoryType::Setup,
            store::InventoryType::Etc,
            store::InventoryType::Cash,
            store::InventoryType::Deco,
        ] {
            let items = match self.store.bag_items(chr.id, inv) {
                Ok(items) => items,
                Err(e) => {
                    out.extend(self.notice(format!("Could not read your {inv:?} bag: {e}")));
                    continue;
                }
            };
            for item in items {
                // **All four tabs go out quiet (mode 5).** Until 2026-09-10 Etc alone did,
                // as the experiment, and Use / Set Up / Cash stayed on mode 0 as the control.
                // The control reported first: the owner, on the Cash tab - *"whenever I enter and
                // exit the cash shop, my Mystery Hair Coupon keeps highlighting itself as if
                // I recently gained them when I had them for quite some time."* Mode 0 is the
                // ADD, and the ADD is what marks a stack as newly gained; every SetField with
                // characterData = 1 (a shop exit is one) replays it. Mode 5 performs the same
                // store without the ADD's hooks (`inventory_restored_replies` has the read).
                // The readout is unchanged: a tab that comes up EMPTY names mode 5 as the
                // cause, and it goes back to mode 0 for that tab.
                out.extend(self.inventory_restored_replies(
                    inv,
                    &[item],
                    "restored on field entry (mode 5, quiet)",
                ));
            }
        }
        match self.store.mesos(chr.id) {
            // Zero is worth sending: the client starts a session believing whatever it last
            // held, and after a relog that is not necessarily zero.
            Ok(mesos) => out.push(Reply {
                opcode: net::stats::STAT_CHANGED,
                body: net::stats::StatChange {
                    meso: Some(u64::from(mesos)),
                    ..Default::default()
                }
                .build(),
                what: format!("StatChanged: {mesos} mesos restored on field entry - bit 18 is the only way this client is ever told a balance"),
            }),
            Err(e) => out.extend(self.notice(format!("Could not read your mesos: {e}"))),
        }
        out.extend(self.quest_helper_recount(chr.id));
        // The friend list, and a line for every request that arrived while this character was
        // away. The client rebuilds its friend manager on a field entry the same way it
        // rebuilds its pools, so the list has to be re-sent. `session/friends.rs`.
        out.extend(self.friend_entry_replies());
        // A gift queued while this character was away: its box opens on the first move.
        self.arm_gift_drop();
        out
    }

    /// **The Quest Helper's item counts, after the quiet restore.** The owner, 2026-09-18, two
    /// screenshots: *"when players enter a new map, the progress in quest helper completely
    /// zeroes out. But when you pick up an item in that map from killing mobs, it will return
    /// back to normal"* - `0/5 Bronze Ore` after the map change, `49/5` after one pickup.
    ///
    /// The helper caches its item counts and refreshes them from the quest hook the loud
    /// `0x0070` modes call (`FUN_142ce53e0`: for every quest that wants the item,
    /// `FUN_142ce5f80(quest)`). The bag is restored with mode 5 on purpose - it is the mode
    /// that skips the collection popup the owner asked to be rid of on 2026-08-30 - and mode 5
    /// skips that hook too, so the counts stay at the zero the empty bag left until the next
    /// real inventory change. The `0x0089` quest-record message's handler (`FUN_142d59e20`)
    /// calls the same `FUN_142ce5f80` and does not go near the popup (`FUN_142d9b200`, the
    /// inventory path only), so re-sending each in-progress quest's own record, progress
    /// string unchanged, after the bag is back makes the helper count again. **[L]** for
    /// both call chains (`tools/callers.py 0x142ce5f80`), **[I]** that the refresh re-reads
    /// the bag rather than a cached count - the run is the test.
    ///
    /// Sent after the restore, never before it, and never on the record itself: this packet
    /// is a no-op while the character-data object is null, which it is on the first
    /// `SetField` (`net::quest::quest_record`). Here the client has finished the field entry
    /// (`0x00DC`) and every `0x0070` before it has been handled.
    fn quest_helper_recount(&self, character_id: u32) -> Vec<Reply> {
        let Ok(rows) = self.store.quest_rows(character_id) else { return Vec::new() };
        rows.into_iter()
            .filter(|row| row.state == store::QuestState::InProgress)
            .map(|row| Reply {
                opcode: net::quest::MESSAGE,
                body: net::quest::quest_record(row.quest_id, &net::quest::QuestProgress::InProgress { progress: row.progress.clone() }),
                what: format!(
                    "QuestRecord: quest {} progress {:?} re-sent after the bag restore, unchanged - the Quest Helper recounts its items from it (mode 5 restores do not wake it)",
                    row.quest_id, row.progress
                ),
            })
            .collect()
    }


    /// Move a character to a map and tell the client, persisting the move.
    ///
    /// Shared by the portal walk and the `/map` GM command, so both go through one path -
    /// a second copy of this is how the two would drift.
    pub(super) fn go_to_map(&mut self, chr: &mut net::opcode::Character, map: u32, portal: u8, why: String)
        -> Vec<Reply>
    {
        // **The mobs this connection controls on the map it is leaving, given to somebody
        // still standing there.**
        //
        // Read before `chr.map_id` is overwritten, because that is the field the claims are
        // on. Control never rotates while its holder is on the map - the client's only revoke
        // is `CONTROL_RELEASE`, and `net::mobmove` says in its own doc that level 0 **deletes
        // the mob** rather than releasing it - so leaving is the only way a mob changes hands.
        // Zero packets go to this connection; one `0x03D2` per mob goes to the successor.
        //
        // This used to be a bare `release_map`, and the mobs then stood still on every
        // remaining screen: nothing in `tick` claims orphans, and the field entry that does
        // is something a standing player never performs. `crate::mobshare` §3.0.
        //
        // Named map, not `hand_over_all_mobs`, so a walk between two maps cannot touch a
        // third one this connection was never on.
        let leaving = self.field_of(chr);
        self.hand_over_mobs(leaving);
        chr.map_id = map;
        chr.portal = portal;
        // **Where they were standing is a fact about the map they just left.**
        //
        // `last_position` is fed only by `0x00D9` and by attack packets, so after a warp it
        // holds coordinates from the *previous* field until the player takes their first
        // step. Three things read it, and all three were wrong across a map change: the
        // `0x0224` spawn announced to everyone on the new map (`session::multiplayer`), the
        // fallback position for a drop (`session::combat`), and the item-drop request
        // (`session::ground`).
        //
        // `None` was the honest answer for a while, and each reader handled it - the spawn
        // fell back to the map origin and self-healed on the first step. The owner, 2026-09-14:
        // *"The first client also sees the client joining start from the origin of the map
        // and then snap to their real position."* The self-heal IS the snap.
        //
        // The arrival portal is a fact this server already holds - it just put it in the
        // SetField - so the announcement stands there instead, in the landing pose, on the
        // foothold under it. That is where the client is about to put them; the first step
        // then corrects nothing visible. **An off-map coordinate is still worse than no
        // coordinate**, so a `portals.txt` without positions falls back to `None` and the
        // origin, exactly as before, and the startup banner says so.
        self.last_position = self.config.portal_positions.get(&(map, portal)).copied();
        // The stance goes with it: landing, not the pose from the map we just left.
        // `remote_at` falls back to standing when this is `None`.
        self.last_move_action = self.last_position.map(|_| net::userpool::MOVE_ACTION_LANDING);
        // **And everyone on the map we are leaving is told, by us, now.**
        //
        // Until this, a server-initiated warp published a farewell only as a side effect of
        // the *arrival*: `Bus::enter_field` leaves the old field as its first act, and that
        // runs from `on_field_entered`, which fires on the client's `0x00DC`. So the old
        // map's players stopped seeing you **only if your client volunteered a packet**.
        //
        // It usually does. Measured across 444 archived logs, deduplicated on
        // `(time, direction, opcode)`: **273 SetFields against 268 markers.** The five that
        // did not answer are the whole point - a ghost is a character standing on a map that
        // nothing will ever remove, because the only thing that could have removed it was the
        // packet that did not arrive.
        //
        // `Bus::leave_field` reads the map out of the *stored presence*, not out of `chr`, so
        // it posts to the field we are leaving however late in `go_to_map` this sits. And it
        // is idempotent with the leave inside `enter_field`: that one finds no presence left
        // to take and simply enters. `CLAUDE.md`'s rule that a guard whose answer is ignored
        // is not a guard has a sibling here - a departure that depends on the departing
        // client's goodwill is not a departure.
        self.leave_the_field();
        let stored = self.store.set_character_map(chr.id, map);
        let warn = match stored {
            Ok(()) => String::new(),
            // Not fatal: the client is told where it is either way, and the next login puts
            // it back where it was.
            Err(e) => format!(" - WARNING: not stored ({e}), so this will not survive a relog"),
        };
        // **Dressed, exactly like the migration's SetField.** This sent the bare form until
        // 2026-08-19, which is why the owner's items had their stats on entering the world and
        // lost them the moment they used a portal or `!map`: every record after the first
        // carried EquipStats::default(), all zeros. It also explains the whole "the tooltip
        // reads a different object" investigation - they had reached map 40 with `!map`, so
        // the record they were hovering really did contain zeros. There was never a second
        // object.
        let dressed = self.dressed(chr);
        let (quests, quest_note) = self.quest_book(chr.id);
        // A character with no skills sends no skill block and does not set its presence
        // byte, so the record stays byte-identical to what this server sent before skills
        // existed. Only a character that has raised something gets the new block.
        let skills = self.store.skills(chr.id).unwrap_or_default();
        let mut out = vec![Reply {
            opcode: net::opcode::SET_FIELD,
            body: net::opcode::set_field_with_character_dressed_quests(
                chr,
                self.config.world_id,
                self.clock_base(),
                self.config.channel_id,
                &dressed,
                &quests,
                &skills,
            ),
            what: format!(
                "SetField, {why}, for character {} ({}){warn}{quest_note}",
                chr.id, chr.name
            ),
        }];
        // The saved key layout rides AFTER every SetField, this one included: a SetField
        // rebuilds the stage the keymap manager belongs to, and the login-time SetField in
        // `dispatch` already does this. The owner, 2026-09-12: "It did not survive a re-login" -
        // and a portal walk must not be the second way to lose it. session/keymap.rs.
        out.extend(self.keymap_replies());
        // The skill-point pools ride after the SetField too, and for the same reason as the
        // keymap: the stat block in the record carries an EMPTY SP table (`character_stat_block`
        // pushes one zero byte on the extended branch), so the new field's skill window would
        // read every pool as 0 and grey the `+` buttons. seedling, 2026-09-14: job-advanced to
        // Bowman at level 12, got the 7 SP in the advance 0x007C, then a portal wiped them.
        // Now every SetField re-sends the real balance. session/skills.rs.
        out.extend(self.skill_point_reply(chr));
        // The pet's long-range pickup box, for the same reason. session/pet.rs.
        out.push(self.pet_pickup_range_reply());
        out
    }


    /// **Change Channel.** The client pressed the button in the dialog.
    ///
    /// # This exists because enabling one byte made the button reachable
    ///
    /// `net::opcode::CHANNEL_ENABLED` went from `0` to `1` on 2026-08-20, which is what makes
    /// a channel row clickable at all. Before that the click produced nothing on the wire and
    /// this handler would have been dead code. `research/channel-two-greyed.md` establishes
    /// that draw and click read the **same** predicate - all six callers of `FUN_142cb9510`
    /// test its result with a bare `test eax,eax` - so there was never an option to make the
    /// row look enabled without making it send.
    ///
    /// **So this had to be answered in the same change.** An unanswered packet freezes the
    /// client's entire UI, quit prompt included, and shipping a newly-clickable button whose
    /// packet nobody answers would have been building the freeze deliberately.
    ///
    /// # The reply shape is inference, and the honest label is [I]
    ///
    /// No capture of a successful channel change exists. `net::opcode::migrate` is what the
    /// login server answers character-select with, and a channel change is the same
    /// operation - "connect to this address as this character" - so it is sent here too.
    /// `crates/net/src/channel.rs` records that expectation as **[I]**; this does not upgrade
    /// it. A wrong reply is a different failure from a frozen UI, and only one of the two is
    /// certain in advance.
    ///
    /// **The migration is minted here, and it is still not authentication.** A `u32` seed
    /// identifies a pending migration; it does not prove who is on the far end. Single use.
    /// **The channel stage's migrate reply. `0x001A`, and it is MEASURED.**
    ///
    /// `research/change-channel-reply.md` established everything about this packet except
    /// its opcode, which no scan could reach: `FUN_1415d8c00` has zero callers of every
    /// kind, zero 4-byte RVA references, and `.themida` has `SizeOfRawData = 0`. So ten
    /// candidates went out on 2026-08-21 and the hook log named the winner - it writes one
    /// dispatch line per inbound opcode, on handler return:
    ///
    /// ```text
    /// 100 opcode=0x0019 elapsed_us=64.0       ret=1 <- dispatched, no-op
    /// 101 opcode=0x001A elapsed_us=354121.0   ret=1036749576   <- 354 ms, then the socket
    /// closed. This is it.
    /// ```
    ///
    /// **The body decode is confirmed by the same run**: the client tore its connection
    /// down and connected to **127.0.0.1:8486**, which is `u32 ip` in network order and
    /// `u16 port` little-endian read exactly as decoded.
    pub(super) const MIGRATE_COMMAND_CHANNEL: u16 = 0x001A;

    pub(super) fn on_change_channel(&mut self, payload: &[u8]) -> Vec<Reply> {
        let request = net::channel::ChangeChannelRequest::parse(payload);
        let Some(req) = request else {
            return self.change_channel_refused(
                "the Change Channel body did not parse - see net::channel::ChangeChannelRequest"
                    .to_string(),
            );
        };
        let target = u32::from(req.target_channel);
        let Some(claimed) = self.claimed.clone() else {
            return self
                .change_channel_refused("no character is claimed on this connection".to_string());
        };
        let Some(listed) = self.config.channels.get(target as usize).copied() else {
            return self.change_channel_refused(format!(
                "this world has no address for channel {target} - pass --channels to the world server"
            ));
        };
        // The listed host is only what --channels said. The client is told the host decided
        // for THIS connection from the two ends of its socket. `net::advertise`.
        let (addr, advertised_as) = self.config.advertise.address_for(
            listed,
            self.peer_addr.map(|a| a.ip()),
            self.local_addr.map(|a| a.ip()),
        );
        if target == self.config.channel_id {
            return self.change_channel_refused(format!(
                "channel {target} is the one you are already on"
            ));
        }
        // Address-recorded like a login migration, so the target channel holds the claiming
        // connection to the same address under PeerPolicy::Require. Not token-bound: the
        // channel connection's own attestation is the login server's business and the owner's
        // rule is that channel-to-channel is fine once the first entry was legitimate.
        let seed = match self.store.create_migration_bound_hash(
            claimed.account_id,
            claimed.character_id,
            self.config.world_id,
            target,
            None,
            self.peer.as_deref(),
        ) {
            Ok(seed) => seed,
            Err(e) => return self.change_channel_refused(format!("could not mint a migration: {e}")),
        };
        // The socket that closes after this reply is a handover, not a departure: the party
        // keeps this character's seat for the session that claims the migration.
        self.handing_over = true;
        // **We used to answer this with `0x0011`, and that is worse than useless.**
        //
        // The owner, 2026-08-21: *"I tried swapping to channel 2, the transfer did not go
        // through, but I lost all ability to attack on my character once the attempt was
        // made."* Both halves follow from one fact: `0x0011` is a **login-stage** opcode,
        // below the channel switch's `0x70` floor, so a channel connection cannot dispatch
        // it at all. The client never saw a reply.
        //
        // And `0x00D2` latches on send, the way `0x0107`, `0x010E` and both ability-point
        // requests do. Only an inbound handler clears it, so an undispatchable reply leaves
        // the player stuck mid-migration - which on screen is a character that cannot
        // attack.
        //
        // The real reply is `FUN_1415d8c00`, a **socket-level** handler whose body is fully
        // measured (`u8 ok`, `u32 ip` in network order, `u16 port` little-endian) but whose
        // **opcode cannot be read statically**: zero callers, zero RVA references, and
        // `.themida` has `SizeOfRawData = 0`. `research/change-channel-reply.md`.
        //
        // So the button sends the sweep. It is no worse than what it replaced - that reply
        // was known not to arrive - and if one candidate is right the migration simply
        // happens. The hook log names which, because it writes a dispatch line per opcode on
        // handler return.
        // One packet now, not ten: the sweep found it on 2026-08-21 and there is nothing
        // left to search. `!migsweep` keeps the ranged form for the next unknown opcode.
        // The character is leaving this channel's field - and therefore this
        // channel's bus - who is watching it. Done here rather than on the
        // reply's arrival, because the reply's opcode is one of the sweep
        // candidates and we do not know that it lands; a migration that fails
        // leaves a client that has already been told to go. `Bus::part` in
        // `Drop` covers that case too, late, and a ghost on the field is
        // exactly the failure `crate::session::multiplayer` exists to avoid.
        self.leave_the_field();
        // ...and the mobs it was simulating are handed to somebody still on that map, who
        // is told so. Releasing alone leaves them alive and motionless on every remaining
        // screen. No packet to the leaver: its only revoke is a despawn. `crate::mobshare`.
        self.hand_over_all_mobs();
        let mut out = self.migrate_candidates(
            target,
            addr,
            seed,
            Self::MIGRATE_COMMAND_CHANNEL,
            Self::MIGRATE_COMMAND_CHANNEL,
        );
        for reply in &mut out {
            reply.what.push_str(&format!(". Advertised as {advertised_as}"));
        }
        out
    }


    /// The migrate reply, sent once per candidate opcode.
    ///
    /// Shared with `!migsweep` so the button and the command cannot drift: the command is
    /// the same packets with a caller-chosen range.
    ///
    /// **The 64 bytes of padding are load-bearing.** An over-read in the client throws
    /// (`1406e8b51` -> `_CxxThrowException` -> `int3`), so a wrong guess landing on a handler
    /// that wants a longer body would kill the client rather than be ignored.
    ///
    /// **The server must not close the socket.** `FUN_142caa360`'s first act is to tear the
    /// connection down client-side. Measured: after the login `0x0011` the client closed 8 ms
    /// later; after the failed channel `0x0011` it stayed open six seconds.
    pub(super) fn migrate_candidates(
        &self,
        target: u32,
        addr: std::net::SocketAddrV4,
        seed: u32,
        first: u16,
        last: u16,
    ) -> Vec<Reply> {
        let mut body = vec![1u8]; // ok
        body.extend_from_slice(&addr.ip().octets()); // NETWORK order, straight into sin_addr
        body.extend_from_slice(&addr.port().to_le_bytes()); // the client htons()es this itself
        body.extend_from_slice(&[0u8; 64]); // padding, so a wrong guess is inert, not fatal
        (first..=last)
            .map(|opcode| Reply {
                opcode,
                body: body.clone(),
                what: format!(
                    "migrate candidate {opcode:#06x}: character to channel {target} at {addr}, seed {seed:#010x} - the 7-byte body is MEASURED, the opcode is [I]. Single use, NOT authentication"
                ),
            })
            .collect()
    }


    /// Refuse a channel change **with a packet**, not with silence.
    ///
    /// `migrate_refused` carries a message the client renders, which is the same shape the
    /// login server uses when it cannot migrate. Saying nothing would freeze the UI.
    pub(super) fn change_channel_refused(&self, why: String) -> Vec<Reply> {
        vec![Reply {
            opcode: net::opcode::MIGRATE_COMMAND,
            body: net::opcode::migrate_refused("Cannot change channel right now."),
            what: format!("Change Channel REFUSED: {why}"),
        }]
    }


    /// Answer Log Out, and **this is not optional in the way most replies are**.
    ///
    /// `0x01BE`'s builder sets `world->[0x33f4] = 1`, and the `SetField` handler
    /// `FUN_142097f80` tests that byte immediately after reading its 8-byte FILETIME and
    /// returns to its epilogue if it is set. **Only `0x0106` clears it.** So an unanswered
    /// Log Out does not just leave the player stuck on the field - it makes **every
    /// subsequent `SetField` vanish in silence**: no dialog, no fault, nothing in any log.
    /// Same failure class as the `player->[0x2330]` latch in `research/npc-click.md`, far
    /// worse blast radius.
    ///
    /// The message must not be empty; an empty string is a no-op in the client.
    ///
    /// **The teardown is in place rather than a reconnect** - the handler builds no
    /// `sockaddr` and constructs a login stage directly. Whether the client also closes this
    /// socket is not settled, and the next run answers it for free: watch whether the
    /// following packet lands in `login.log` or `world.log`.
    pub(super) fn on_log_out(&mut self) -> Vec<Reply> {
        // The character is leaving the game: the party is told now, while the store and the
        // hub can still be reached in an orderly way. `session/party.rs`.
        self.leave_party_on_disconnect();
        // The conversation and the field's chatter belong to a session that is ending.
        self.conversation = None;
        self.chatter.clear();
        // And the character stops standing on the map, for everyone else on it.
        // The socket lives on - the client goes back to character select on this
        // same connection - so this is a leave rather than a part; `Drop` still
        // runs later and `Bus::part` is idempotent. `crate::session::multiplayer`.
        self.leave_the_field();
        // And its mobs become somebody else's mobs - handed to a connection still standing
        // there, and that connection is told. Idempotent, like `Bus::leave_field`: a session
        // that holds nothing finds no map to hand over.
        self.hand_over_all_mobs();
        vec![Reply {
            opcode: net::notice::LOG_OUT_RESULT,
            body: net::notice::log_out_result("Returning to the login screen."),
            what: "LogOutResult - and answering this is what clears world->[0x33f4]. Until it is cleared the client silently drops every SetField."
                .to_string(),
        }]
    }


    /// Answer the client walking into a portal.
    ///
    /// The reply is another `SetField` with `characterData = 1` - the **long** form, the one
    /// byte-identical to the packet that already worked, differing only in the map id at
    /// stat-block offset 84. The short `characterData = 0` form is the shape actually
    /// designed for "same character, new map", and it is probably correct here now that the
    /// client has a live field; but `research/transfer-field-request.md` could not prove its
    /// precondition (`world+0x2358`, the `CUserLocal` slot) is populated - an exhaustive scan
    /// found 222 readers and **no** store. Sending an unproven form to save 200 bytes would
    /// trade a working path for a guess. The long form is idempotent because the object it
    /// re-fetches is a lazy singleton.
    pub(super) fn on_transfer_field(&mut self, payload: &[u8]) -> Vec<Reply> {
        let req = parse_transfer_field(payload);
        let chr = self.claimed_character();

        // Always answer. Even a request we cannot resolve gets a SetField for the map the
        // character is already on, because silence freezes the client's entire UI.
        let Some(mut chr) = chr else {
            return vec![Reply {
                opcode: net::opcode::SET_FIELD,
                body: net::opcode::set_field_minimal(self.clock_base(), self.config.channel_id),
                what: "transfer-field request, but the character could not be loaded - answered with the minimal record rather than dropped, because an unanswered packet freezes the client's whole UI."
                    .to_string(),
            }];
        };

        // **A dead character's transfer request is a REVIVE, and it must be caught here
        // before the portal logic ever sees it.**
        //
        // Clicking REVIVE IN TOWN sends an ordinary `0x00D1` with `targetField = 0` and an
        // empty portal name - 25 bytes where a portal walk is 34, because the client skips
        // both position encodes when the name is empty. `parse_transfer_field` reads that
        // `0` as a perfectly good map id, so without this branch a revive would warp the
        // character **to map 0** - a map this client has no field image for - with no HP
        // restored. `research/revive.md`.
        //
        // **Branch on the server's own HP, not on the packet shape.** A 25-byte body with
        // target 0 is what the revive button happens to send today; the character being dead
        // is what actually makes this a revive. Matching on the shape would break the moment
        // a real map 0 existed or the client padded differently.
        if chr.hp == 0 {
            return self.revive(chr);
        }

        let Some(r) = req else {
            let here = chr.map_id;
            return self.go_to_map(
                &mut chr,
                here,
                0,
                "the body was too short to parse - re-sending the current map".to_string(),
            );
        };

        // A walked door that is not in the table re-sends the current map rather than
        // guessing a destination. That is the right answer for `0x00D1`, because the client
        // is waiting on a SetField; the script portal below chooses differently.
        let (target, arrival, note) = match self.resolve_named_portal(&chr, r.target_field, &r.portal_name) {
            Some(found) => found,
            None => (
                chr.map_id,
                0,
                format!(
                    "portal {:?} on map {} is NOT in the portal table, so this re-sends the current map rather than guessing a destination",
                    r.portal_name, chr.map_id
                ),
            ),
        };
        self.go_to_map(&mut chr, target, arrival, note)
    }

    /// `0x014A` - the player pressed up on a **script** portal (`pt` 7 or 8).
    ///
    /// The owner, 2026-09-10: *"The portal in Ellinia to go to Ellinia Station still currently does
    /// not exist."* It existed in the table since the day before; what did not exist was a
    /// handler for the packet a script portal sends, which is this one and never `0x00D1`.
    /// Their two presses sat in `world.log` as UNKNOWN. `net::portalscript` has the capture and
    /// the builder that agrees with it.
    ///
    /// Resolution is [`Self::resolve_named_portal`], shared with the walked door, so the
    /// Free Market's `pt 7` doors - which send this too - and the two derived script portals
    /// land the same way whichever packet carries them.
    ///
    /// **Unresolved is answered with the unlock, not a re-send of the map.** `rand_ola`, the
    /// PQ portals and Zakum's door are real script portals with no destination here; putting
    /// the player back through a `SetField` of the map they are standing in would be a
    /// visible stutter for nothing. Twelve unanswered presses did not freeze the client, so
    /// this may not latch at all, but the standing rule is to answer every request.
    pub(super) fn on_portal_script(&mut self, body: &[u8]) -> Vec<Reply> {
        let unlock = || {
            crate::mesodrop::unlock_unhandled_latching_request(
                net::portalscript::CLIENT_PORTAL_SCRIPT,
            )
        };
        let Some(req) = net::portalscript::parse_portal_script(body) else {
            crate::server::log(&format!(
                "   script portal: a {} byte 0x014A body that does not parse; unlock only",
                body.len()
            ));
            return unlock();
        };
        // First Time Together's `next00`, which opens per run. session/firsttime.rs.
        if let Some(replies) = self.party_quest_portal(&req.portal_name) {
            return replies;
        }
        let Some(mut chr) = self.claimed_character() else { return unlock() };
        match self.resolve_named_portal(&chr, None, &req.portal_name) {
            Some((target, arrival, note)) => {
                let note = format!("script {note}, pressed at ({}, {})", req.x, req.y);
                self.go_to_map(&mut chr, target, arrival, note)
            }
            None => {
                crate::server::log(&format!(
                    "   script portal: {:?} on map {} has no destination this server knows \
                     (world::scriptportals lists the ones it does not); unlock only",
                    req.portal_name, chr.map_id
                ));
                unlock()
            }
        }
    }

    /// Where a named portal on the character's current map leads: `(map, arrival index,
    /// note)`, or `None` when nothing here knows.
    ///
    /// **One path for both request opcodes.** The walked door (`0x00D1`) and the script
    /// portal (`0x014A`) both come here, so a destination cannot work on foot and fail by
    /// script or the reverse - which is exactly the shape the station door was in for a day.
    ///
    /// The arrival index matters: the source portal names its destination portal in the
    /// WZ's `tn`, and the stat block wants that portal's index on the target map. Without it
    /// every door lands on the map's spawn point, which is right for a login and wrong for a
    /// door.
    ///
    /// **The Free Market's doors are resolved before the table, because the table cannot
    /// hold them.** The owner, 2026-09-09: *"The server should keep track of which town the user
    /// entered from, and then when the user leaves the Free Market, it should return them to
    /// the proper portal which they have entered from."* All four have target `0` in
    /// `portals.txt` and no row anywhere names them. `crate::freemarket` has the working, and
    /// `free_market_door` writes the memory before returning, so the caller must warp.
    fn resolve_named_portal(
        &mut self,
        chr: &net::opcode::Character,
        explicit_target: Option<u32>,
        portal_name: &str,
    ) -> Option<(u32, u8, String)> {
        if let Some((to, at, note)) = self.free_market_door(chr, portal_name) {
            let arrival = self.config.portal_index.get(&(to, at)).copied().unwrap_or(0);
            return Some((to, arrival, note));
        }
        if let Some(t) = explicit_target {
            return Some((t, 0, format!("portal {portal_name:?} -> map {t} portal 0")));
        }
        let (to, tn) = self.config.portals.get(&(chr.map_id, portal_name.to_string()))?;
        let arrival = self.config.portal_index.get(&(*to, tn.clone())).copied().unwrap_or(0);
        Some((*to, arrival, format!("portal {portal_name:?} -> map {to} portal {arrival}")))
    }

    /// Bring a dead character back: town, 50 HP, and 10% of their experience unless they are
    /// level 10 or below.
    ///
    /// The owner, 2026-08-21: *"Reviving a character should warp them to the nearest town, start at
    /// 50 HP, and reduce their EXP by 10% unless they are level 10 or below."* All three
    /// numbers are their and none is read out of the client.
    ///
    /// # Why "nearest town" is a table lookup and not a walk
    ///
    /// `gm-handbook/returnmaps.txt`'s `reviveMap` column already encodes one **unconditional**
    /// `returnMap` hop followed by a walk to the first `town == 1` field, and re-deriving that
    /// here would step into both traps `research/return-maps.md` documents: `town == 1` is
    /// carried by shop interiors, and 94 of this client's 115 town fields point their own
    /// `returnMap` elsewhere - so stopping on the flag revives the player inside Southperry
    /// Armor Store. 33 fields are also `returnMap` self-loops.
    ///
    /// # Two packets, and the second is not optional
    ///
    /// `go_to_map` sends a `SetField` whose stat block carries the restored HP, but a `0x007C`
    /// goes out **after** it as well. `research/user-hit.md` §6.2 enumerated ~65 sites that
    /// gate an action on the sign of the client's HP; the character has to be positive in the
    /// client's own copy or it arrives in town unable to do anything.
    /// The Free Market's one-way door: `(destination map, arrival portal, log line)`.
    ///
    /// `None` for every portal that is not one of the four, which is all but four of 3679.
    ///
    /// # Entering writes the memory BEFORE the warp
    ///
    /// `CLAUDE.md`'s Heena rule in its most literal form. If the write fails, the warp does not
    /// happen - because the alternative is a character standing in a hall whose exit has
    /// nothing to read, and the exit is the only way out. Refusing to enter is recoverable;
    /// entering without a memory is the state that needs a GM.
    ///
    /// # Leaving clears it, and the fallback is only ever reached by a character who was
    /// already inside
    ///
    /// The memory is persisted, so a relog keeps it. [`crate::freemarket::FALLBACK`] exists
    /// for the one case the design cannot cover: someone standing in the hall at the moment
    /// this shipped. It is logged loudly when it fires, because if it ever fires for anyone
    /// else the write-before-warp rule above has been broken somewhere.
    fn free_market_door(
        &mut self,
        chr: &net::opcode::Character,
        portal: &str,
    ) -> Option<(u32, String, String)> {
        use crate::freemarket as fm;

        if fm::is_entrance(chr.map_id, portal) {
            if let Err(e) = self.store.set_field_return(
                chr.id,
                store::fieldreturn::KIND_FREE_MARKET,
                chr.map_id,
                portal,
            ) {
                crate::server::log(&format!(
                    "   free market: could NOT remember {}'s way back to map {} ({e}); refusing \
                     to let them in rather than stranding them in the hall",
                    chr.id, chr.map_id
                ));
                return None;
            }
            return Some((
                fm::HALL,
                fm::HALL_ARRIVAL.to_string(),
                format!(
                    "portal {portal:?} -> the Free Market. Remembered the way back: map {} \
                     portal {portal:?}",
                    chr.map_id
                ),
            ));
        }

        if !fm::is_exit(chr.map_id, portal) {
            return None;
        }
        let remembered = self
            .store
            .field_return(chr.id, store::fieldreturn::KIND_FREE_MARKET)
            .ok()
            .flatten();
        let (to, at, how) = match remembered {
            Some((map, p)) => (map, p, "the way they came in"),
            None => {
                crate::server::log(&format!(
                    "   free market: character {} is leaving with nothing remembered, so they \
                     go to the fallback. This should be unreachable - the memory is written \
                     before the warp in - so it means either they were already in the hall \
                     before this shipped, or the write-before-warp rule has been broken.",
                    chr.id
                ));
                (fm::FALLBACK.0, fm::FALLBACK.1.to_string(), "the FALLBACK - nothing remembered")
            }
        };
        // Forget it only once the destination is decided, so a failed read cannot lose the
        // memory as well as the answer.
        let _ = self.store.clear_field_return(chr.id, store::fieldreturn::KIND_FREE_MARKET);
        Some((
            to,
            at.clone(),
            format!("portal {portal:?} -> out of the Free Market to map {to} portal {at:?} ({how})"),
        ))
    }

    fn revive(&mut self, mut chr: net::opcode::Character) -> Vec<Reply> {
        let died_on = chr.map_id;
        let penalty = net::revive::death_exp_loss(chr.level, chr.exp);
        // **What the death costs.** The owner, 2026-09-24: *"dying in a party quest area should
        // not take any EXP penalty away from the player. Outside of the party quest area, if
        // the player has a Safety Charm, a safety charm will be removed in exchange for
        // keeping the player's current EXP level."* A charm is spent only when there is
        // something to protect - never at level 10 or below, never in a party quest.
        let why = if penalty == 0 {
            DeathCost::Free
        } else if crate::firsttime::is_quest_map(died_on) {
            DeathCost::PartyQuest
        } else if self.spend_safety_charm(chr.id) {
            DeathCost::SafetyCharm
        } else {
            DeathCost::Penalty
        };
        let lost = if why == DeathCost::Penalty { penalty } else { 0 };
        chr.exp = chr.exp.saturating_sub(lost);
        chr.hp = net::revive::REVIVE_HP.min(chr.max_hp);

        if let Err(e) = self.store.save_character_progress(&chr) {
            // Not fatal - the client is still told where it is and what its HP is, and a
            // relog puts it back. Silence here would freeze the UI.
            return self.notice(format!("Could not save your revival: {e}"));
        }

        // No row means the table was not generated. Leaving the character where they fell is
        // wrong but safe; sending them to a map with no field image strands them outright.
        let (target, where_note) = match self.config.revive_field(died_on) {
            Some(t) => (t, format!("map {died_on} -> {t}")),
            None => (
                died_on,
                format!(
                    "map {died_on} has no row in the revive table, so this leaves the character where they fell - regenerate with python tools/dump_returnmaps.py"
                ),
            ),
        };

        // Built before the call: `go_to_map` borrows `chr` mutably, so reading its fields in
        // the argument list would borrow it twice.
        let note = format!(
            "REVIVE: {where_note}, hp {}/{}, {}",
            chr.hp,
            chr.max_hp,
            match why {
                DeathCost::Penalty => format!("-{lost} exp (10% at level {}) -> {}", chr.level, chr.exp),
                DeathCost::Free => format!("no exp penalty at level {} (10 or below is free)", chr.level),
                DeathCost::PartyQuest => format!("no exp penalty: died in the party quest, on map {died_on} (would have been -{penalty})"),
                DeathCost::SafetyCharm => format!("a Safety Charm was spent instead of -{penalty} exp"),
            }
        );
        let mut out = self.go_to_map(&mut chr, target, 0, note);

        // After the SetField, deliberately. Without a positive HP in the client's own copy
        // the action gates stay shut and the player arrives in town unable to move.
        out.push(Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange {
                hp: Some(chr.hp),
                exp: Some(chr.exp),
                ..Default::default()
            }
            .build(),
            what: format!(
                "StatChanged after revive: hp {}/{}, exp {} - sent AFTER the SetField because ~65 client sites gate on the sign of HP, and a character revived without this arrives in town unable to act",
                chr.hp, chr.max_hp, chr.exp
            ),
        });
        // The charm's two lines wait for the town's `0x00DC`: sent now, beside the SetField,
        // the client has no field and drops the notice (see `EFFECT_SAFETY_CHARM_USED`).
        // The owner: *"The player should see that they have lost a Safety Charm when they respawn
        // and a notice that a safety charm has been used to protect their EXP loss."*
        if why == DeathCost::SafetyCharm {
            self.after_field_entry.push(Reply {
                opcode: net::stats::USER_EFFECT_LOCAL,
                body: net::message::item_lost_in_chat(net::revive::SAFETY_CHARM, 1),
                what: "UserEffectLocal item line: Safety Charm x1 lost - spent on a death".to_string(),
            });
            self.after_field_entry.push(Reply {
                opcode: net::stats::USER_EFFECT_LOCAL,
                body: net::revive::safety_charm_used(net::revive::SAFETY_CHARM),
                what: "UserEffectLocal effect 0x0C: the client's own 'The EXP did not drop after using Safety Charm item.'".to_string(),
            });
        }
        out
    }

    /// **Take one Safety Charm from the Cash tab**, if there is one. `true` when one was
    /// taken. The stack's new count is not sent now - it rides the `SetField`'s character
    /// record, which is read from the store after this.
    fn spend_safety_charm(&mut self, chr_id: u32) -> bool {
        let inv = store::InventoryType::Cash;
        let slot = self
            .store
            .bag_items(chr_id, inv)
            .unwrap_or_default()
            .into_iter()
            .find(|r| r.item.item_id == net::revive::SAFETY_CHARM && r.item.kind.quantity() > 0)
            .map(|r| r.slot);
        let Some(slot) = slot else { return false };
        match self.store.remove_item(chr_id, inv, slot, Some(1)) {
            Ok(_) => true,
            Err(e) => {
                crate::server::log(&format!("   revive: could not take a Safety Charm from Cash slot {slot}: {e}; the penalty applies"));
                false
            }
        }
    }

    /// `0x01E7` - "revive on the spot", which this server never enables but always answers.
    ///
    /// The button is **hidden** at create time unless an obfuscated counter at
    /// `world[0x2368]+0x200` is above zero, and nothing this server sends alters it - so in
    /// practice the client only ever offers the town button. It is answered anyway: the
    /// always-answer rule does not have an exception for a request that should be impossible,
    /// and if it ever does arrive, the log line below is how we find out.
    pub(super) fn on_revive_on_spot(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        if chr.hp > 0 {
            return self.notice(
                "You are not dead. (A revive request arrived for a living character.)".to_string(),
            );
        }
        let mut out = self.notice(
            "Reviving on the spot is not available - use the town button.".to_string(),
        );
        out[0].what = format!(
            "0x01E7 revive-on-the-spot from character {} on map {}, body {:02x?} - REFUSED and answered. This should not be reachable: the button is hidden unless world[0x2368]+0x200 > 0 and nothing here sets it, so seeing this line at all is the finding",
            chr.id, chr.map_id, payload
        );
        out
    }
}

/// What a death cost, decided once in `revive` and read by everything after it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeathCost {
    /// Level 10 or below: nothing to take.
    Free,
    /// Died on a party quest map: nothing taken, no charm spent.
    PartyQuest,
    /// A Safety Charm was spent instead of the experience.
    SafetyCharm,
    /// The 10% penalty.
    Penalty,
}

#[cfg(test)]
mod free_market_tests {
    use super::*;
    use crate::config::Config;
    use std::sync::Arc;
    use store::Store;

    /// A session with the REAL portal table, and a character standing on `map`.
    fn on_map(map: u32) -> Option<(Session, Arc<Store>, u32)> {
        let path = std::path::Path::new("../../gm-handbook/portals.txt");
        if !path.exists() {
            return None; // generated, gitignored
        }
        let (portals, portal_index) = Config::load_portals(path);
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character {
            name: "Walker".to_string(),
            map_id: map,
            ..Default::default()
        };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::new(
            store.clone(),
            Arc::new(Config { portals, portal_index, ..Config::default() }),
        );
        s.claim_for_character(id);
        Some((s, store, id))
    }

    /// The claimed character, standing on `map`.
    ///
    /// The map is set on the local record rather than written back, because
    /// `free_market_door` reads `chr.map_id` from the record it is handed - which is exactly
    /// how the real caller passes it, out of the live `Character` mid-transfer.
    fn standing_on(s: &Session, map: u32) -> net::opcode::Character {
        let mut chr = s.claimed_character().expect("a claimed character");
        chr.map_id = map;
        chr
    }

    /// **Enter from Henesys, leave, land back in Henesys.** The owner: *"The server should keep
    /// track of which town the user entered from, and then when the user leaves the Free
    /// Market, it should return them to the proper portal which they have entered from."*
    ///
    /// Driven through `free_market_door` rather than the constants, so the memory, the store
    /// round trip and the clear are all exercised together.
    #[test]
    fn the_free_market_returns_you_to_the_town_you_came_from() {
        let Some((mut s, store, id)) = on_map(10_001_040) else { return };

        // In, from Henesys Market.
        let chr = standing_on(&s, 10_001_040);
        let (to, at, note) = s.free_market_door(&chr, "market00").expect("an entrance");
        assert_eq!(to, crate::freemarket::HALL);
        assert_eq!(at, crate::freemarket::HALL_ARRIVAL);
        assert!(note.contains("Remembered"), "{note}");
        assert_eq!(
            store.field_return(id, store::fieldreturn::KIND_FREE_MARKET).unwrap(),
            Some((10_001_040, "market00".to_string())),
            "the way back is written BEFORE the warp"
        );

        // Out again.
        let chr = standing_on(&s, crate::freemarket::HALL);
        let (to, at, note) = s.free_market_door(&chr, "out00").expect("the exit");
        assert_eq!((to, at.as_str()), (10_001_040, "market00"), "back to Henesys Market");
        assert!(note.contains("the way they came in"), "{note}");
        assert_eq!(
            store.field_return(id, store::fieldreturn::KIND_FREE_MARKET).unwrap(),
            None,
            "and the memory is spent"
        );
    }

    /// **The town is remembered per entry, not hard-coded.** A test that only ever entered
    /// from Henesys would pass on an implementation that always returned Henesys - which is
    /// exactly what the fallback does.
    #[test]
    fn entering_from_el_nath_returns_to_el_nath_not_the_fallback() {
        let Some((mut s, _store, _id)) = on_map(20_001_010) else { return };
        assert_ne!(20_001_010, crate::freemarket::FALLBACK.0, "or this test proves nothing");

        let chr = standing_on(&s, 20_001_010);
        s.free_market_door(&chr, "market00").expect("El Nath is an entrance");
        let chr = standing_on(&s, crate::freemarket::HALL);
        let (to, at, _) = s.free_market_door(&chr, "out00").expect("the exit");
        assert_eq!((to, at.as_str()), (20_001_010, "market00"));
    }

    /// Leaving with nothing remembered lands on the fallback rather than failing - a player
    /// in the hall must always have a way out, because the exit has no static destination.
    #[test]
    fn leaving_with_no_memory_uses_the_fallback_instead_of_stranding() {
        let Some((mut s, _store, _id)) = on_map(crate::freemarket::HALL) else { return };
        let chr = standing_on(&s, crate::freemarket::HALL);
        let (to, at, note) = s.free_market_door(&chr, "out00").expect("the exit still answers");
        assert_eq!((to, at.as_str()), crate::freemarket::FALLBACK);
        assert!(note.contains("FALLBACK"), "and it says so: {note}");
    }

    /// **Every other portal is untouched.** This hook runs before the static table on every
    /// walk in the game, so a predicate that was even slightly too broad would reroute
    /// ordinary doors.
    #[test]
    fn no_ordinary_portal_is_diverted() {
        let Some((mut s, _store, _id)) = on_map(10_002_000) else { return };
        let chr = standing_on(&s, 10_002_000);
        for portal in ["west00", "top00", "in00", "in01", "in03", "out00", "nonsense"] {
            assert!(
                s.free_market_door(&chr, portal).is_none(),
                "Ellinia's {portal} was diverted into the Free Market path"
            );
        }
        // Including a town that HAS an entrance, on its other portals.
        let chr = standing_on(&s, 10_004_000);
        assert!(s.free_market_door(&chr, "market00").is_some(), "Perion's IS an entrance");
        for portal in ["out00", "in00", "west00"] {
            assert!(s.free_market_door(&chr, portal).is_none(), "Perion's {portal}");
        }
    }
}

#[cfg(test)]
mod death_cost_tests {
    use super::*;
    use crate::config::Config;
    use std::sync::Arc;
    use store::Store;

    /// A dead level-20 character with 1000 EXP on `map`, holding `charms` Safety Charms.
    fn dead_on(map: u32, charms: u16) -> (Session, Arc<Store>, u32) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Faller".to_string(), map_id: map, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut cfg = Config::default();
        cfg.item_names.insert(net::revive::SAFETY_CHARM, "Safety Charm".into());
        let mut s = Session::new(store.clone(), Arc::new(cfg));
        s.claim_for_character(id);
        if charms > 0 {
            s.give_item(net::revive::SAFETY_CHARM, charms, "test").unwrap();
        }
        let mut chr = s.claimed_character().unwrap();
        chr.level = 20;
        chr.exp = 1_000;
        chr.hp = 0;
        chr.max_hp = 200;
        store.save_character_progress(&chr).unwrap();
        store.set_character_map(id, map).unwrap();
        (s, store, id)
    }

    /// The town button: target 0, empty portal name.
    fn revive(s: &mut Session) -> Vec<Reply> {
        let mut body = vec![0u8; 16];
        body.extend_from_slice(&0u32.to_le_bytes());
        body.extend_from_slice(&0u16.to_le_bytes());
        s.on_transfer_field(&body)
    }

    fn exp(s: &Session) -> u64 {
        s.claimed_character().unwrap().exp
    }

    fn charms(store: &Store, id: u32) -> u32 {
        store
            .bag_items(id, store::InventoryType::Cash)
            .unwrap()
            .iter()
            .filter(|r| r.item.item_id == net::revive::SAFETY_CHARM)
            .map(|r| u32::from(r.item.kind.quantity()))
            .sum()
    }

    fn effects(out: &[Reply]) -> Vec<Vec<u8>> {
        out.iter().filter(|r| r.opcode == net::stats::USER_EFFECT_LOCAL).map(|r| r.body.clone()).collect()
    }

    /// **A Safety Charm is spent instead of the EXP**, and its two lines - the charm lost,
    /// the client's own "The EXP did not drop" - wait for the town's field entry: sent beside
    /// the SetField, the second is dropped by a client with no field. Exactly one charm of
    /// two, once; the next death spends the second, and the one after that pays.
    #[test]
    fn a_safety_charm_is_spent_instead_of_the_exp_and_says_so_once_the_town_has_loaded() {
        let (mut s, store, id) = dead_on(40, 2);
        let out = revive(&mut s);
        assert_eq!(exp(&s), 1_000, "the EXP is kept");
        assert_eq!(charms(&store, id), 1, "one charm of two spent");
        assert!(effects(&out).is_empty(), "nothing beside the SetField - the client has no field to post it in");

        let entered = s.on_field_entered();
        assert_eq!(
            effects(&entered),
            vec![
                net::message::item_lost_in_chat(net::revive::SAFETY_CHARM, 1),
                net::revive::safety_charm_used(net::revive::SAFETY_CHARM),
            ],
            "the charm lost, then the notice, on the town's field entry"
        );
        assert!(effects(&s.on_field_entered()).is_empty(), "once, not on every field entry after");

        // The second death spends the second charm; the third pays the penalty.
        for (charms_left, exp_after) in [(0, 1_000), (0, 900)] {
            let mut chr = s.claimed_character().unwrap();
            chr.hp = 0;
            store.save_character_progress(&chr).unwrap();
            let _ = revive(&mut s);
            assert_eq!(charms(&store, id), charms_left);
            assert_eq!(exp(&s), exp_after);
        }
        assert!(effects(&s.on_field_entered()).len() <= 2, "the paid death adds no charm line");
    }

    /// **No charm, no protection**: the 10% penalty, and nothing said about a charm.
    #[test]
    fn without_a_charm_the_penalty_applies_and_no_charm_line_is_sent() {
        let (mut s, _store, _id) = dead_on(40, 0);
        let _ = revive(&mut s);
        assert_eq!(exp(&s), 900);
        assert!(effects(&s.on_field_entered()).is_empty());
    }

    /// **A party quest death costs nothing - not the EXP, and not a charm.** The owner,
    /// 2026-09-24: *"dying in a party quest area should not take any EXP penalty away from
    /// the player."* Every First Time Together map, stage 1 to the Exit.
    #[test]
    fn a_death_in_the_party_quest_costs_neither_exp_nor_a_charm() {
        for map in [crate::firsttime::STAGE_1, crate::firsttime::STAGE_3, crate::firsttime::STAGE_5, crate::firsttime::EXIT_MAP] {
            for held in [0, 1] {
                let (mut s, store, id) = dead_on(map, held);
                let _ = revive(&mut s);
                assert_eq!(exp(&s), 1_000, "map {map}, {held} charm(s): no penalty");
                assert_eq!(charms(&store, id), u32::from(held), "map {map}: the charm is not spent");
                assert!(effects(&s.on_field_entered()).is_empty());
            }
        }
    }

    /// **Level 10 or below has nothing to protect**, so the charm stays in the bag.
    #[test]
    fn a_free_death_does_not_spend_a_charm() {
        let (mut s, store, id) = dead_on(40, 1);
        let mut chr = s.claimed_character().unwrap();
        chr.level = 10;
        store.save_character_progress(&chr).unwrap();
        let _ = revive(&mut s);
        assert_eq!(exp(&s), 1_000);
        assert_eq!(charms(&store, id), 1);
    }
}
