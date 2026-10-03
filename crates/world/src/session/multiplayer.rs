//! This connection's end of the channel's message bus: what it is owed,
//! and what it owes everyone else.
//!
//! The bus itself is `crate::broadcast`. It holds a mailbox per connection
//! and knows no packet layouts. This file is the half that does know them -
//! it turns "this character is standing here dressed like this" into the
//! bytes a remote client needs, and turns arriving mail into `Reply`s.
//!
//! # What is wired here today, and what is not
//!
//! **Wired - the delivery half.** Every connection takes a mailbox at
//! construction (`Session::joining`), hands it back in `Drop`, collects its
//! mail in both `Session::handle` and `Session::tick`, and drops out of its
//! field on a clean log out or a channel change. That machinery is tested
//! in `crate::broadcast`, and it is exactly what `research/exp-sharing.md`
//! and `session/combat.rs` both name as the missing piece:
//!
//! > *"`Fields::hurt` already returns the whole split; **only the delivery
//! > is missing**."*
//!
//! **Wired as of 2026-08-29 - and never seen on a screen.** Three packets
//! go out now: `0x0224` UserEnterField, `0x0225` UserLeaveField and
//! `0x0293` UserMoveRemote, decoded in `research/user-enter-field.md` and
//! `research/user-pool-tables.md`. Every offset is read out of this client
//! rather than taken from the v214 reference, and the enter/leave
//! assignment is **[L]** from the bodies (`0x0224` allocates a `CUser` and
//! inserts it; `0x0225` unlinks a node and calls its destructor) rather
//! than **[D]** from enum order.
//!
//! **Nothing *here* has been on the wire**, and that is still true of every
//! packet this file builds. What used to follow it was not: this block read
//! *"no packet in `0x224..0x39F` has ever been observed to do anything"*, and
//! `0x02D1` is in that range, appears in 49 archived files, and is recorded
//! in `STATUS.md` as working on screen twice - the quest fanfare and the blue
//! recovery number. Corrected 2026-08-31; `net::userpool`'s module docs carry
//! the counts and the control.
//!
//! It matters because that sentence was the reason the whole remote family
//! looked unproven. The route above the pool is live. The one hop still
//! unverified is whether `0x0224` puts a `CUser` in the pool. The first run should carry a watch on `0x1429ba60b` -
//! the allocation past every gate - because if that never fires, the
//! answer is one of the six gates in `user-enter-field.md` §5 and no
//! amount of body work will help.
//!
//! # The one thing known to be wrong, and it self-heals
//!
//! **A character who has just arrived has no position.** The server's only
//! source is the client's own `0x00D9` movement reports (and attack
//! packets), so a player who has not yet moved is announced at whatever
//! [`Session::last_position`] holds - `(0, 0)` on a fresh arrival, which is
//! the map origin.
//!
//! It corrects itself on that player's first step, because `0x0293` moves
//! a remote user the client already has. The visible residue is a
//! newly-arrived character appearing at the origin until they move, and a
//! player who arrives and stands **perfectly still** staying there.
//!
//! The real fix is portal coordinates: `gm-handbook/portals.txt` carries
//! `map, index, portal, target map, target portal` and **no x/y**, so
//! `tools/dump_portals.py` would have to emit them. That is a WZ dump
//! change and a handbook regeneration, which is why it is not in this
//! commit.
//!
//! # Why field entry is one call and not two
//!
//! `Bus::enter_field` both announces this player and returns everyone
//! already here, because the failure it prevents is asymmetric: A sees B and
//! B does not see A. That reads on one screen as "it works", and this
//! project has only ever had one screen.

use crate::broadcast::Bus;
use crate::session::{Reply, Session};

impl Session {
    /// This channel's bus. Reached through `Fields` because that `Arc` was
    /// already being handed to every session - see `crate::fields::Fields::bus`.
    pub(super) fn bus(&self) -> &Bus {
        self.fields.bus()
    }

    /// Everything other connections have said to this one since it last looked.
    ///
    /// Called from `Session::handle` (after the reply to whatever arrived) and
    /// from `Session::tick` (for a client that is standing still and therefore
    /// sending nothing). Both, because either alone leaves a case uncovered.
    pub(super) fn collect_mail(&mut self) -> Vec<Reply> {
        let mut out = self.bus().drain(self.subscriber);

        // **Facts become packets here, and only here.** See `crate::broadcast::Event` for
        // why they cross the bus in that shape rather than as a `Reply`: an EXP award
        // carries the recipient's own new total and level, which only this session can
        // compute from this character's own record.
        //
        // Bound to a local first so the `&Bus` borrow ends before the loop needs `&mut self`.
        let events = self.bus().drain_events(self.subscriber);
        for event in events {
            match event {
                crate::broadcast::Event::Experience { amount, why, white } => {
                    // **The recipient's own coupon, not the killer's.** A share arrives as a
                    // plain number computed on somebody else's connection; the buff that
                    // multiplies it belongs to whoever is being paid, and this is the only
                    // place that knows.
                    let (amount, coupon_note) = self.with_exp_coupon(amount);
                    out.extend(self.award_experience(
                        amount,
                        &format!("{why}{coupon_note}"),
                        white,
                        false,
                    ));
                }
                crate::broadcast::Event::PartyHeal { percent, caster } => {
                    out.extend(self.heal_percent(percent, &format!("Heal from character {caster}")));
                }
                crate::broadcast::Event::PartyMesos { amount, picker } => {
                    out.extend(self.receive_party_mesos(amount, picker));
                }
                crate::broadcast::Event::PartyBuff { skill_id, level, caster } => {
                    out.extend(self.receive_party_buff(skill_id, level, caster));
                }
                crate::broadcast::Event::PartyQuestEnter { map, why } => {
                    out.extend(self.enter_party_quest(map, &why));
                }
                crate::broadcast::Event::BoatWarp { voyage, map } => {
                    out.extend(self.boat_warp(voyage, map));
                }
                crate::broadcast::Event::ItemBlessing { item_id, giver } => {
                    out.extend(self.receive_item_blessing(item_id, &giver));
                }
                crate::broadcast::Event::PartyQuestReward { item, why } => {
                    out.extend(self.receive_party_quest_reward(item, &why));
                }
                crate::broadcast::Event::PartyQuestExp { percent, why } => {
                    out.extend(self.receive_party_quest_exp(percent, &why));
                }
                crate::broadcast::Event::FriendRequest => {
                    // Their list changed: redraw it, and say out loud anything that is now
                    // waiting on this player. session/friends.rs.
                    out.extend(self.friend_entry_replies());
                }
                crate::broadcast::Event::GiftDrop => {
                    // Two gifts queued back to back are two events and ONE box: the box lists
                    // "N more waiting", and the next opens behind the answer. A box already
                    // open for a gift is left alone rather than replaced under the player.
                    let already_open = self.conversation.as_ref().is_some_and(|c| crate::giftdrop::gift_id_from_path(&c.path).is_some());
                    let n = self.claimed.as_ref().and_then(|c| self.store.pending_gifts(c.character_id, c.account_id, store::Store::unix_now()).ok()).map(|g| g.len()).unwrap_or(0);
                    if n > 0 && !already_open {
                        out.extend(self.notice(crate::giftdrop::waiting_notice(n)));
                        out.extend(self.open_gift_drop());
                    }
                }
            }
        }
        out
    }

    /// **Give this connection's mobs on `map` to somebody still standing there.**
    ///
    /// The owner, 2026-09-01: *"If the person who is controlling the movement of the mob leaves the
    /// map, then the mob should not disappear ... The control of the mob should be handed over
    /// to another client who is still present in the map."*
    ///
    /// The mobs never did disappear - releasing a claim leaves the mob alive - but until this,
    /// **nobody was told**, so every monster on the map stood perfectly still on the remaining
    /// screens until somebody walked through a portal and back. That is worse than theirs
    /// worry: a frozen monster reads as a broken server, and there was no error and no log
    /// line to find it by. The existing test only re-granted because it called
    /// `on_field_entered` a second time, which a standing player never does.
    ///
    /// Replaces the bare `release_map` at every exit. When nobody is left it degrades to
    /// exactly that.
    ///
    /// **Nothing is sent to the leaver.** `CONTROL_RELEASE` is the client's only revoke and it
    /// *despawns*, so a farewell grant would delete the mob on the screen being left - and
    /// that client has already torn its mob pool down anyway.
    pub(super) fn hand_over_mobs(&mut self, map: crate::fields::FieldKey) {
        let me = self.subscriber.get();
        let Some(heir) = self.fields.bus().successor_on(map, self.subscriber) else {
            // Nobody left to drive them. Free the claims so the next arrival can take them;
            // that is what `on_field_entered` already asks for.
            let freed = self.fields.controllers().release_map(map, me);
            if freed > 0 {
                crate::server::log(&format!(
                    "   mob control: {freed} mob(s) on map {map} released - nobody else is here \
                     to hand them to. The next arrival claims them"
                ));
            }
            return;
        };
        let moved = self.fields.controllers().hand_over(map, me, heir.get());
        if moved.is_empty() {
            return;
        }
        // The grants have to name the mob as it is NOW - position and hp - because the heir's
        // client drives it from here. `mobs_on` is the live field, not a snapshot taken when
        // this connection arrived.
        let live = self.fields.mobs_on(map);
        let mut sent = 0usize;
        for object_id in &moved {
            let Some(mob) = live.iter().find(|m| m.spawn.object_id == *object_id) else { continue };
            // `as_seen`, not `spawn`: the heir has to be handed the mob where it is standing
            // and with the HP it has left, or its client resumes the wander from the spawn
            // point and the monster jumps across the map on every remaining screen.
            let mob = mob.as_seen();
            let landed = self.fields.bus().publish_to_subscriber(
                heir,
                Reply {
                    opcode: net::mobmove::MOB_CHANGE_CONTROLLER,
                    body: net::mobmove::mob_change_controller(&mob, net::mobmove::CONTROL_NORMAL),
                    what: format!(
                        "MobChangeController: object id {} handed to the connection still on \
                         map {map} - its previous controller left. Without this the mob stays \
                         alive and stops moving on every remaining screen",
                        mob.object_id
                    ),
                },
            );
            if landed {
                sent += 1;
            }
        }
        crate::server::log(&format!(
            "   mob control: {} mob(s) on map {map} handed from connection {me} to              connection {}, {sent} grant(s) delivered",
            moved.len(),
            heir.get()
        ));
    }

    /// **Hand over everything this connection controls, wherever it is.**
    ///
    /// The exits that have no map to hand: a log out, a channel change, and `Drop` - the
    /// socket dropping, the client crashing, the process being killed. They used to call
    /// [`crate::mobshare::Controllers::release_all`], which is correct about ownership and
    /// silent about it, so the mobs kept standing still on every screen that was left.
    ///
    /// Normally one map. It is a loop because a crash can leave claims on a map this
    /// connection already walked away from, if the walk is what killed it.
    pub(super) fn hand_over_all_mobs(&mut self) {
        for map in self.fields.controllers().maps_held_by(self.subscriber.get()) {
            self.hand_over_mobs(map);
        }
    }

    /// Drop out of the field without ending the connection.
    ///
    /// A log out or a channel change: the character stops being on this map and
    /// everyone still there must be told, but the socket lives on and the
    /// mailbox stays. A **portal walk is not this** - that is
    /// `Bus::enter_field` with the new map, which does the leaving as its first
    /// act so the two halves cannot half-happen.
    pub(super) fn leave_the_field(&mut self) {
        self.announce_offline_to_link();
        self.bus().leave_field(self.subscriber);
    }

    /// Where this character is, as far as anyone here knows.
    ///
    /// `last_position` is fed by `0x00D9` and by attack packets, and is
    /// `None` until one of them arrives - see the module docs for what that
    /// costs and why it heals. `foothold` stays `0`, which is legal and
    /// means "resolve it yourself"; sending a foothold id from the map we
    /// just left would be worse than sending none.
    pub(super) fn remote_at(&self) -> net::userpool::RemoteAt {
        let (x, y) = self.last_position.unwrap_or((0, 0));

        // **A foothold of `0` means "in the air", and that is why an existing player looked
        // like they were floating.**
        //
        // The owner, 2026-09-04: *"clients see those original players already present in the map
        // as 'floating' instead of the desired 'idle' position."*
        //
        // The doc that used to sit here said `0` was legal and meant "resolve it yourself",
        // and reasoned that *"sending a foothold id from the map we just left would be worse
        // than sending none"*. The first half was a guess and the second was answering a
        // question that no longer applies: this is called for a character standing on a map
        // we know, at a position we now keep up to date (`note_own_position`), so the
        // foothold under them is a lookup rather than a leftover.
        //
        // `Footholds::landing` is the same function drops use to find the floor - one table,
        // one answer, so a player and an item dropped at their feet cannot disagree about
        // where the ground is.
        //
        // Still `0` when the table has nothing to say: a map with no foothold data, or a
        // position genuinely in mid-air. That is the honest answer there, and it is the
        // behaviour every remote player had until now.
        let foothold = self
            .config
            .footholds
            .landing(self.claimed_character().map(|c| c.map_id).unwrap_or(0), x, y)
            // The wire field is an `i16`; a foothold id that does not fit is one this client
            // could not have meant, so it falls back to "in the air" rather than truncating
            // into a real id belonging to some other platform.
            .and_then(|l| i16::try_from(l.foothold).ok())
            .unwrap_or(0);

        // **The stance and the facing.** `0` here is action 0 facing RIGHT - a real value,
        // and the reason every remote player was drawn facing right until this was wired.
        // `MOVE_ACTION_STANDING` is the resting pose, which is the honest answer for somebody
        // who has not moved since arriving.
        let move_action =
            self.last_move_action.unwrap_or(net::userpool::MOVE_ACTION_STANDING);

        net::userpool::RemoteAt { x, y, move_action, foothold, active_effect_item: self.active_effect_item }
    }

    /// **Remember where this character is, and tell the bus, so a LATER joiner is not sent
    /// a position from minutes ago.**
    ///
    /// The owner, 2026-09-03, with two clients finally on one map: *"the positioning is off if
    /// someone joins the map later since they don't know where existing clients are. When
    /// someone new joins a map with existing clients, they should be aware of what existing
    /// clients' positions are."*
    ///
    /// They are right, and the cause is not that the server has no position - it has one, fed by
    /// `0x00D9` and by attack packets. The cause is that [`crate::broadcast::Presence`]'s
    /// `spawn` packet is **built once, at field entry**, and the bus hands that same frozen
    /// body to everyone who arrives afterwards. The existing player then appears wherever they
    /// were standing when THEY entered - which, before their first step, is the map origin.
    ///
    /// [`crate::broadcast::Bus::refresh_spawn`] was written for exactly this and **had zero
    /// callers**. `CLAUDE.md`'s "built is not wired": on screen an unwired subsystem is
    /// indistinguishable from one that does not exist.
    ///
    /// # One function, because two call sites is how the refresh gets forgotten
    ///
    /// `last_position` is written from the move handler and from the attack path. Setting the
    /// field and refreshing the bus are the same event, so they are the same function - the
    /// quest-payout lesson in `CLAUDE.md`, where every effect had to hang off the transition
    /// rather than be repeated beside it.
    ///
    /// Cheap by construction: it returns before touching the store when the position has not
    /// actually changed, which is most `0x00D9`s in a stationary crowd.
    ///
    /// **Takes the stance too, and the early return compares BOTH.** Turning on the spot and
    /// landing change the pose without moving a pixel, and a position-only comparison would
    /// skip the refresh and leave the announced stance one packet stale - which is the same
    /// class of bug as the frozen snapshot this function exists to fix, one field over.
    pub(super) fn note_own_position(&mut self, x: i16, y: i16, move_action: Option<u8>) {
        let action = move_action.or(self.last_move_action);
        if self.last_position == Some((x, y)) && self.last_move_action == action {
            return;
        }
        self.last_position = Some((x, y));
        self.last_move_action = action;
        // And the bus's copy, which is the only one another session can read.
        self.fields.bus().note_position(self.subscriber, x, y);
        // Only worth rebuilding while somebody could still arrive and be told. A connection
        // with no presence is not on a field.
        let Some(chr) = self.claimed_character() else { return };
        let spawn = self.presence(&chr).spawn;
        self.fields.bus().refresh_spawn(self.subscriber, spawn);
    }

    /// This character as everyone else on the field needs to hear about it.
    ///
    /// Both packets are built **now**, including the farewell, because the
    /// farewell has to survive into `Drop` - where there is no store to
    /// load a character from. See `crate::broadcast::Presence`.
    pub(super) fn presence(&self, chr: &net::opcode::Character) -> crate::broadcast::Presence {
        crate::broadcast::Presence {
            character: chr.id,
            map: self.field_of(chr),
            spawn: Reply {
                opcode: net::userpool::USER_ENTER_FIELD,
                body: net::userpool::user_enter_field(chr, self.remote_at()),
                what: format!(
                    "UserEnterField: {} ({}) on map {} at ({}, {}) - {} bytes. \
                     research/user-enter-field.md; nothing here authenticates anybody.",
                    chr.name,
                    chr.id,
                    chr.map_id,
                    self.remote_at().x,
                    self.remote_at().y,
                    net::userpool::user_enter_field_len(chr),
                ),
            },
            farewell: Reply {
                opcode: net::userpool::USER_LEAVE_FIELD,
                body: net::userpool::user_leave_field(chr.id),
                what: format!("UserLeaveField: character {} is gone", chr.id),
            },
            companions: self.pet_companions(chr),
        }
    }

    /// Announce this character to the field, and return everyone already on it.
    ///
    /// Called from `on_field_entered` - the `0x00DC` marker, which arrives
    /// once per field entry, every time, including after a portal walk.
    /// That is the right hook for the same reason the NPC and mob re-sends
    /// use it: the client destroys and rebuilds its pools on every
    /// `SetField`, so the user pool has to be refilled after each one too.
    ///
    /// **Our own character is not in the returned list.** A `0x0224`
    /// carrying our own id clears one dword and returns without reading the
    /// body (`research/user-enter-field.md` §5) - harmless, but there is no
    /// reason to send it, and `Bus::enter_field` excludes the sender.
    pub(super) fn announce_field_entry(&mut self) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        // And to the hub, so the other channels can find this character. Idempotent.
        self.announce_online_to_link();
        let presence = self.presence(&chr);
        let here = self.bus().enter_field(self.subscriber, presence);
        if here.is_empty() {
            return here;
        }
        crate::server::log(&format!(
            "   field {} has {} other player(s) - sending {} UserEnterField",
            chr.map_id,
            here.len(),
            here.len()
        ));
        here
    }

    /// **Tell the field this character's hair or face just changed.** No field re-entry for
    /// anyone: the other clients get a leave-then-enter for this ONE character.
    ///
    /// The owner, 2026-09-12: *"The moment any hair or face change happens, it should also show
    /// up on other clients."* And 2026-09-18, with two clients side by side: *"the hair
    /// instantly changed for the same client, but did not change for multiplayer clients"* -
    /// `world-ch0.log` shows the bystander received the `0x0224` and drew nothing. That is
    /// what `research/user-enter-field.md` §0 already said **[L]**: *re-sending `0x0224` for
    /// an id already in the pool is a silent no-op - the handler returns before it reads a
    /// byte of the body.* The 2026-09-12 version of this function sent the enter alone and
    /// filed that as its [I]; the run settled it.
    ///
    /// The leave-then-enter did work, and it was measured the same evening as **a blink**:
    /// *"a weird super brief character blink as it disappears and reappears on the map. The
    /// regular maplestory does not have this behavior"*, and with a pet out, *"the pet
    /// completely respawns and appear sad/hungry until moments later"* - the remote `CUser`
    /// and its pet are destroyed and rebuilt, which is what that sequence is.
    ///
    /// **Measured 2026-09-18 14:07, and `0x0138` did nothing**: read afterwards, its apply
    /// walks the user's *summoned* map and never the player, so it could not have. The owner:
    /// *"The leave-and-enter path causes the pets to reload for that client, and it causes a
    /// brief blink. That is undesirable. Please find another suitable way."*
    ///
    /// **The way is `0x02AE`** (`net::lookupdate`): a user-pool by-id packet on the chair
    /// relay's own router that decodes the look straight into the pooled user and rebuilds
    /// its avatar - the same rebuild that dressed it on entry. One per observer, carrying the
    /// changed character's id; the router cannot address the local player, and the local
    /// redraw is already the `0x007C` / `0x0070` path. `Config::look_change_reenter`
    /// (`--look-reenter`) keeps the leave + enter + pet as the fallback. Either way the stored
    /// spawn is refreshed so a later joiner gets the new look.
    /// `research/remote-redress-2026-09-18.md`; **[D]** that the rebuild draws - never on a
    /// screen. Plan step TO(c).
    pub(super) fn broadcast_look_change(&mut self, chr: &net::opcode::Character) {
        let Some(map) = self.bus().map_of(self.subscriber) else { return };
        let presence = self.presence(chr);
        self.bus().refresh_spawn(self.subscriber, presence.spawn.clone());
        if self.config.look_change_reenter {
            self.bus().publish(self.subscriber, map, presence.farewell, None);
            self.bus().publish(self.subscriber, map, presence.spawn, None);
            let pets = presence.companions.len();
            for c in presence.companions {
                self.bus().publish(self.subscriber, map, c, None);
            }
            crate::server::log(&format!(
                "   look change for character {} (hair {}, face {}): leave + enter + {pets} pet(s) \
                 broadcast to field {map} (--look-reenter; the copy blinks and its pet respawns)",
                chr.id, chr.hair, chr.face
            ));
            return;
        }
        let update = Reply {
            opcode: net::lookupdate::USER_LOOK_UPDATE_REMOTE,
            body: net::lookupdate::user_look_update_remote(chr),
            what: format!(
                "UserLookUpdateRemote: character {} redressed in place - hair {}, face {}, {} worn, {} bytes. \
                 The pool decodes the look into its copy and rebuilds the avatar; no leave, no enter.",
                chr.id,
                chr.hair,
                chr.face,
                chr.equips.len(),
                net::lookupdate::user_look_update_remote_len(chr)
            ),
        };
        self.bus().publish(self.subscriber, map, update, None);
        crate::server::log(&format!(
            "   look change for character {} (hair {}, face {}): one 0x02AE to field {map}, no leave, no enter, pet untouched",
            chr.id, chr.hair, chr.face
        ));
    }

    /// Rebroadcast a movement report to everyone else on this field.
    ///
    /// `body` is the `0x00D9` body with the opcode already stripped, and
    /// `m` is what [`net::usermove::parse_user_move`] made of it.
    ///
    /// **A path that did not walk closed is not rebroadcast.** `UserMove::path`
    /// returns `None` there, and that is a refusal, not an empty path: this
    /// server would be re-emitting bytes it could not itself account for,
    /// and a body of the wrong length has killed this client twice.
    ///
    /// Superseded per character, so a stalled observer gets the latest
    /// position rather than a replay - `crate::broadcast`.
    pub(super) fn publish_user_move(&mut self, m: &net::usermove::UserMove, body: &[u8]) {
        let Some(chr) = self.claimed.as_ref().map(|c| c.character_id) else { return };
        let Some(map) = self.bus().map_of(self.subscriber) else { return };
        let Some(path) = m.path(body) else {
            crate::server::log(
                "   move NOT rebroadcast: the element walk did not close, so the path \
                 cannot be re-emitted - see net::usermove::UserMove::walk_closed",
            );
            return;
        };
        // **A path with no elements dereferences null in the client**, and `walk_closed`
        // does not cover it.
        //
        // `141d59bef mov rax,[rbp+0x18]` reads the head of the decoded element list and
        // then eight `movups` out of its tail, unconditionally and with no null check.
        // The list is empty exactly when `element_count <= 0` - and a 25-byte `0x00D9`
        // with a count of zero **walks closed**, because `for _ in 0..count` runs no
        // iterations, `p` stays at the head length, and the key-state trailer then lands
        // exactly on the end. A negative count is the same hole: `0..negative` is an empty
        // range in Rust, so it closes too.
        //
        // **This is latent, not active.** All 170 806 movement paths in the archive were
        // checked - 6 689 `0x00D9` and 164 117 `0x02FF`, both written by the client's own
        // encoder `FUN_141d57c60` - and **not one** has an empty element list. So an honest
        // client never sends it. That is precisely why the guard belongs here rather than
        // nowhere: the packet is *rebroadcast to other people*, so a client that sends one
        // would be killing somebody else's session, and this server is the only thing
        // between the two. `CLAUDE.md`'s "always answer" is about the same asymmetry seen
        // from the other side.
        if m.element_count <= 0 {
            crate::server::log(&format!(
                "   move NOT rebroadcast: element_count is {}, and an empty element list \
                 dereferences null in the remote client at 141d59bef. Never seen in 170 806 \
                 archived paths, so a client sending this is not an honest one",
                m.element_count
            ));
            return;
        }
        self.bus().publish(
            self.subscriber,
            map,
            Reply {
                opcode: net::userpool::USER_MOVE_REMOTE,
                body: net::userpool::user_move_remote(chr, path),
                what: format!(
                    "UserMoveRemote: character {chr} to ({}, {}), {} path bytes copied \
                     verbatim (no key-state trailer - 1429d2eb5 XOR R8D,R8D)",
                    m.x,
                    m.y,
                    path.len()
                ),
            },
            Some(chr),
        );
    }

    /// Rebroadcast a swing to everyone else on this field.
    ///
    /// **The attack half of what the owner asked for on 2026-08-29.**
    /// [`Session::publish_user_move`] is the movement half and was, until this,
    /// the only production caller of [`crate::broadcast::Bus::publish`] in the
    /// whole crate.
    ///
    /// `opcode` and `payload` are the inbound `0x00DF`/`0x00E0`/`0x00E1`
    /// exactly as [`Session::on_attack`] received them.
    ///
    /// # Three things this deliberately does not do
    ///
    /// * **It does not echo the inbound body.** The two encoders are not
    ///   symmetric - 40 header fields in, 13 out - so `crate::remoteattack`
    ///   projects the parse field for field. `research/user-pool-tables.md` §4.
    ///   The movement rebroadcast's byte-for-byte copy is licensed by a shared
    ///   encoder that does not exist on this side.
    /// * **It does not take the character id from the packet.** Nothing on this
    ///   socket authenticates anybody, so the id comes from this session's own
    ///   claimed migration.
    /// * **It does not supersede.** `Bus::publish` is called with `None`
    ///   because an attack is an *event*: two swings are two events, and a
    ///   supersede key would render a whole fight as one hit. Pinned by
    ///   `remoteattack::tests::three_swings_are_three_packets_and_would_not_be_with_a_supersede_key`.
    ///
    /// # A parse failure is not a refusal to answer
    ///
    /// `CLAUDE.md`'s always-answer rule. This returns nothing on every failure
    /// path and the caller's reply to the attacker is untouched, so a body this
    /// server cannot re-encode costs the *observers* a swing and costs the
    /// attacker nothing at all.
    /// **Show everyone else the damage this player just took, and the flinch.**
    ///
    /// The owner, 2026-09-03: *"when one client is getting hurt by mobs, the other clients should
    /// also be displaying the damage that the client is taking and the blinking expression
    /// when they are taking damage."*
    ///
    /// `payload` is the client's own `0x00E5` body with its opcode already stripped - the
    /// same 147-byte HITINFO the client sent, passed on with one field replaced.
    ///
    /// # `applied`, never the client's claim
    ///
    /// The inbound body's `+8` is what the client says it took. This server computes and caps
    /// its own, and sends the capped figure in `0x007C`. Broadcasting the claim instead would
    /// put a number on everyone else's screen that disagrees with the HP bar the hurt player
    /// is watching.
    ///
    /// # Only touch damage
    ///
    /// The attack index at HITINFO `+4` is `-1` in every one of 331 captured bodies, and the
    /// handler short-circuits on it at `0x1429d494f`. **The `>= 0` and `<= -2` arms have not
    /// been traced**, so a numbered mob skill is not rebroadcast rather than rebroadcast on a
    /// guess - `crates/net/src/userpool.rs` records why a wrong body on this wire is the
    /// expensive kind of wrong.
    pub(super) fn publish_user_hit(&mut self, hit: &net::userhit::UserHit, payload: &[u8], applied: u32) {
        if !hit.is_touch() {
            crate::server::log(&format!(
                "   hit NOT rebroadcast: attack index {} is a numbered mob skill, and only the \
                 touch arm of 0x02A5 has been traced. The hurt player's own 0x007C is \
                 unaffected",
                hit.attack_index
            ));
            return;
        }
        let Some(chr) = self.claimed_character() else { return };
        let Some(map) = self.bus().map_of(self.subscriber) else { return };
        let Ok(damage) = i32::try_from(applied) else { return };

        let Some(body) = net::userpool::user_hit_remote(chr.id, payload, damage) else {
            crate::server::log(&format!(
                "   hit NOT rebroadcast: the body is {} bytes and the HITINFO is {}. A short \
                 one is a decoder disagreement, not something to pad",
                payload.len(),
                net::userpool::USER_HIT_REMOTE_HITINFO_LEN
            ));
            return;
        };

        // `supersedes: None` - being hit is an EVENT, not a state. Two hits in one tick are
        // two numbers on the screen, and coalescing them would drop one.
        self.bus().publish(
            self.subscriber,
            map,
            Reply {
                opcode: net::userpool::USER_HIT_REMOTE,
                body,
                what: format!(
                    "UserHitRemote: {} ({}) took {applied} from mob template {}. The number and \
                     the 1500 ms flinch on every OTHER screen. The client sends 0 in this field \
                     and the server fills it - an echo would draw a MISS",
                    chr.name, chr.id, hit.mob_template_id
                ),
            },
            None,
        );
    }

    pub(super) fn publish_user_attack(&mut self, opcode: u16, payload: &[u8]) {
        let Some(out_opcode) = crate::remoteattack::remote_attack_opcode(opcode) else {
            return; // 0x00E2 body attack - an undecoded layout, nothing to re-encode
        };
        let Some(chr) = self.claimed_character() else { return };
        let Some(map) = self.bus().map_of(self.subscriber) else { return };
        let attack = match net::attack::parse(opcode, payload) {
            Ok(a) => a,
            Err(e) => {
                crate::server::log(&format!(
                    "   swing NOT rebroadcast: {e}. The attacker's own reply is unaffected \
                     - this server would be emitting a body it could not account for, and a \
                     wrong length on this wire has killed this client twice."
                ));
                return;
            }
        };
        let body = crate::remoteattack::user_attack_remote(
            chr.id,
            // `[user + 0x406c]`, the same dword `0x0224` UserEnterField fills at its body
            // offset 12 with the level. See `crate::remoteattack`'s module docs: the
            // argument is consistency between the two packets, not identification.
            u16::try_from(chr.level).unwrap_or(u16::MAX),
            &attack,
        );
        let what = crate::remoteattack::describe(chr.id, out_opcode, &attack, body.len());
        self.bus().publish(
            self.subscriber,
            map,
            Reply { opcode: out_opcode, body, what },
            // An event, not a state. See the doc block above.
            None,
        );
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use store::Store;

    use crate::broadcast::Presence;
    use crate::config::Config;
    use crate::fields::Fields;
    use crate::session::{Reply, Session};

    /// The channel's shape: **one** `Fields`, and therefore one bus, shared by
    /// every connection on it - `crate::server::serve`. `Session::new` gives each
    /// session a private one, which is right for the other 490 tests in this
    /// crate and is exactly wrong for these.
    fn channel() -> (Arc<Store>, Arc<Config>, Arc<Fields>) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        // `Config::answer_packets` is on by default now; with it off `Session::handle`
        // and `Session::tick` both return early and nothing here answers at all.
        let config = Arc::new(Config::default());
        (store, config, Arc::new(Fields::new()))
    }

    fn presence(character: u32, map: crate::fields::FieldKey) -> Presence {
        Presence {
            character,
            map,
            spawn: Reply {
                opcode: 0x0224,
                body: Vec::new(),
                what: format!("spawn {character}"),
            },
            farewell: Reply {
                opcode: 0x0225,
                body: Vec::new(),
                what: format!("farewell {character}"),
            },
            companions: Vec::new(),
        }
    }

    /// An opcode this dispatch has no arm for, so `handle` returns whatever the
    /// mailbox contributed and nothing else.
    const NOTHING: [u8; 2] = [0xFF, 0xFE];

    /// Two sessions on one map. The speaker gets the echo on its own connection; the other
    /// hears it through the bus, with the speaker's character id; the speaker does not hear
    /// itself twice. The owner, 2026-09-05: *"each client was only able to see the message that
    /// they sent."*
    #[test]
    fn a_chat_line_reaches_everyone_else_on_the_map() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Cobalt", "Tester2"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: 104_040_000,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        let mut speaker = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut listener = Session::joining(store, config, fields);
        speaker.claim_for_character(ids[0]);
        listener.claim_for_character(ids[1]);
        speaker.on_field_entered();
        listener.on_field_entered();
        let _ = speaker.tick(1_000);
        let _ = listener.tick(1_000);

        let echo = speaker.say_out_loud("hello cobalt");
        assert_eq!(echo.len(), 1);
        assert_eq!(echo[0].opcode, net::userchat::USER_CHAT);

        let mail = listener.tick(2_000);
        let heard: Vec<&Reply> = mail.iter().filter(|r| r.opcode == net::userchat::USER_CHAT).collect();
        assert_eq!(heard.len(), 1, "the other player hears it once");
        assert_eq!(heard[0].body, echo[0].body, "the same packet: the speaker's id and the text");
        assert!(heard[0].what.contains("relayed"), "{}", heard[0].what);

        let own: Vec<Reply> = speaker.tick(3_000);
        assert!(own.iter().all(|r| r.opcode != net::userchat::USER_CHAT), "no double line for the speaker");
    }

    /// **Invite, dialog, accept - across two sessions.** The leader is told the outcome, the
    /// target is handed the 0x03 that opens the dialog (field 2 being the party id the answer
    /// echoes), and an Accept tells both that the joiner joined. Shapes [L]; what the dialog
    /// draws from fields 3-6 is [I] and belongs to the next run.
    #[test]
    fn a_party_invite_reaches_the_target_and_an_accept_tells_everyone() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Cobalt", "Tester2"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: 104_040_000,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        let mut leader = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut invitee = Session::joining(store, config, fields);
        leader.claim_for_character(ids[0]);
        invitee.claim_for_character(ids[1]);
        leader.on_field_entered();
        invitee.on_field_entered();
        let _ = leader.tick(1_000);
        let _ = invitee.tick(1_000);

        let created = leader.run_party_request(
            ids[0],
            crate::party::Request::Create { name: "Cobalt's Party".into() },
        );
        assert_eq!(created[0].body[0], net::party::result::CREATED, "{created:?}");
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());

        let out = leader.run_party_request(ids[0], crate::party::Request::Invite { target: ids[1] });
        assert_eq!(out.len(), 1, "{out:?}");
        assert_eq!(out[0].body[0], net::party::result::INVITE_OUTCOME, "not UNKNOWN_ERROR any more");
        assert_eq!(&out[0].body[1..5], &0i32.to_le_bytes(), "outcome 0: You have invited");
        assert!(out[0].what.contains("Tester2"), "{}", out[0].what);

        let mail = invitee.tick(2_000);
        let notify = mail
            .iter()
            .find(|r| r.opcode == net::party::PARTY_RESULT && r.body[0] == net::party::result::INVITE_NOTIFY_A)
            .unwrap_or_else(|| panic!("the target must be handed the dialog: {mail:?}"));
        assert_eq!(&notify.body[1..5], &ids[0].to_le_bytes(), "field 1: the inviter");
        assert_eq!(&notify.body[5..9], &party.to_le_bytes(), "field 2: the party id");

        let joined = invitee.run_party_request(ids[1], crate::party::Request::Accept { party });
        let join = joined
            .iter()
            .find(|r| r.body[0] == net::party::result::JOIN)
            .unwrap_or_else(|| panic!("the joiner is told: {joined:?}"));
        // **The name alone killed both clients on 2026-09-05.** After the name comes the
        // six-seat PARTYBLOCK: with two seats occupied that is well over 300 bytes, and the
        // block's first field - the party id - sits right after the name's bytes.
        let name_end = 1 + 2 + usize::from(u16::from_le_bytes([join.body[1], join.body[2]]));
        assert_eq!(&join.body[name_end..name_end + 4], &party.to_le_bytes(), "{:02x?}", join.body);
        assert!(join.body.len() > 300, "a 0x13 must carry the PARTYBLOCK: {} bytes", join.body.len());
        // And both occupied seats name real characters, in order: leader, then joiner.
        let seats = &join.body[name_end + 5..];
        assert_eq!(&seats[0..4], &ids[0].to_le_bytes(), "seat 0 is the leader");
        let mail = leader.tick(3_000);
        assert!(
            mail.iter().any(|r| r.opcode == net::party::PARTY_RESULT && r.body[0] == net::party::result::JOIN),
            "and so is the leader: {mail:?}"
        );

        // **The member leaves, and BOTH clients are told.** The owner, 2026-09-05: *"Tester2
        // cannot also leave the party."* The leaver reads its own WITHDRAW (0x10) directly;
        // the leader gets one over the bus. Before this the Leave transition was in the
        // undecoded list and answered with UNKNOWN_ERROR - the client showed an error and
        // stayed in the party.
        let left = invitee.run_party_request(ids[1], crate::party::Request::Leave);
        let leaver_told = left.iter().find(|r| {
            r.opcode == net::party::PARTY_RESULT && r.body[0] == net::party::result::WITHDRAW
        });
        let leaver_told = leaver_told.unwrap_or_else(|| panic!("the leaver must get a 0x10: {left:?}"));
        assert_eq!(&leaver_told.body[1..5], &ids[1].to_le_bytes(), "char_id is the leaver");
        assert_eq!(leaver_told.body[5], 1, "still_exists: the party lives on");

        let mail = leader.tick(4_000);
        assert!(
            mail.iter().any(|r| {
                r.opcode == net::party::PARTY_RESULT && r.body[0] == net::party::result::WITHDRAW
            }),
            "the leader is told the member left: {mail:?}"
        );
    }

    /// **Party members on one field are told each other's HP, and again when it changes.**
    ///
    /// The owner, 2026-09-05, with the screenshot of a blank bar: *"Party member HP should've been
    /// broadcasted to party members on the same map when the party is formed."* `0x02B2` -
    /// the chain from the packet to the gauge is in `net::userpool::USER_HP_REMOTE`'s doc.
    /// Three claims: nothing is sent before the party exists; once it does, each member's
    /// next tick puts the other's `(id, hp, max)` in their mailbox; and a change to one
    /// member's HP is sent again, while an unchanged tick sends nothing.
    #[test]
    fn party_members_on_one_field_are_told_each_others_hp_and_told_again_when_it_changes() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Cobalt", "Tester2"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: 104_040_000,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        let mut leader = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut member = Session::joining(store.clone(), config, fields);
        leader.claim_for_character(ids[0]);
        member.claim_for_character(ids[1]);
        leader.on_field_entered();
        member.on_field_entered();

        let is_hp = |r: &Reply| r.opcode == net::userpool::USER_HP_REMOTE;
        let hp_of = |r: &Reply| {
            (
                u32::from_le_bytes(r.body[0..4].try_into().unwrap()),
                u32::from_le_bytes(r.body[4..8].try_into().unwrap()),
                u32::from_le_bytes(r.body[8..12].try_into().unwrap()),
            )
        };

        // No party yet: a tick each, and neither mailbox holds an HP packet.
        let _ = leader.tick(1_000);
        let _ = member.tick(1_000);
        assert!(!leader.tick(1_100).iter().any(is_hp), "no party, no HP broadcast");
        assert!(!member.tick(1_100).iter().any(is_hp));

        // Form the party.
        let created = leader.run_party_request(
            ids[0],
            crate::party::Request::Create { name: "Cobalt's Party".into() },
        );
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        let _ = leader.run_party_request(ids[0], crate::party::Request::Invite { target: ids[1] });
        let _ = member.tick(2_000);
        let _ = member.run_party_request(ids[1], crate::party::Request::Accept { party });

        // Each member's own tick sends; the OTHER's next tick receives. The member's tick
        // below does both at once - it drains the leader's packet AND sends its own - so it
        // is the one to inspect. (A discarded tick here is a drained mailbox, which is how
        // the first version of this test asserted on an empty list.)
        let _ = leader.tick(3_000);
        let got = member.tick(3_000);
        let about_leader: Vec<_> = got.iter().filter(|r| is_hp(r)).map(hp_of).collect();
        assert_eq!(about_leader.len(), 1, "one HP packet about the leader: {got:?}");
        let leader_rec = store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == ids[0]).unwrap();
        assert_eq!(
            about_leader[0],
            (ids[0], u32::try_from(leader_rec.hp).unwrap(), u32::try_from(leader_rec.max_hp).unwrap())
        );
        let got = leader.tick(3_100);
        assert!(got.iter().filter(|r| is_hp(r)).any(|r| hp_of(r).0 == ids[1]), "and one about the member: {got:?}");

        // Nothing changed: the next ticks send nothing more.
        let _ = leader.tick(4_000);
        let _ = member.tick(4_000);
        assert!(!member.tick(4_100).iter().any(is_hp), "unchanged HP is not resent");

        // The leader loses HP (a save is what every HP path ends in), and the member is told.
        let mut hurt = leader_rec.clone();
        hurt.hp = hurt.hp.saturating_sub(1);
        store.save_character_progress(&hurt).unwrap();
        let _ = leader.tick(5_000);
        let got = member.tick(5_100);
        let again: Vec<_> = got.iter().filter(|r| is_hp(r)).map(hp_of).collect();
        assert_eq!(again.len(), 1, "the change is broadcast once: {got:?}");
        assert_eq!(again[0].1, u32::try_from(hurt.hp).unwrap(), "with the new HP");
    }

    /// **Standing up from a chair does not blank the stander's HP bar on a partner's screen.**
    ///
    /// The owner, 2026-09-24: *"When players initially sit on a chair then stand up, their HP bars
    /// appear empty when they are part of a party."* The client's stand-up (`SetSeat`) ends
    /// by sending `0x00DC`, the server answers it as a field entry, and the partner is sent the
    /// stander's `0x0225` and a fresh `0x0224` - a new `CUser` with an empty bar. This replays
    /// that sequence (sit, stand, `0x00DC`) with the HP unchanged throughout, and requires an
    /// `0x02B2` about the stander to reach the partner AFTER the new spawn.
    #[test]
    fn standing_up_from_a_chair_resends_the_standers_hp_after_their_new_spawn() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Cobalt", "Tester2"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: 104_040_000, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        let mut sitter = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut partner = Session::joining(store.clone(), config, fields);
        sitter.claim_for_character(ids[0]);
        partner.claim_for_character(ids[1]);
        sitter.on_field_entered();
        partner.on_field_entered();
        let created = sitter.run_party_request(ids[0], crate::party::Request::Create { name: "P".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        let _ = sitter.run_party_request(ids[0], crate::party::Request::Invite { target: ids[1] });
        let _ = partner.tick(1_000);
        let _ = partner.run_party_request(ids[1], crate::party::Request::Accept { party });
        for t in [2_000, 2_100, 2_200] {
            let _ = sitter.tick(t);
            let _ = partner.tick(t);
        }
        let about = |r: &Reply| u32::from_le_bytes(r.body[0..4].try_into().unwrap());
        let is_hp_of_sitter = |r: &Reply| r.opcode == net::userpool::USER_HP_REMOTE && about(r) == ids[0];
        assert!(!partner.tick(2_300).iter().any(is_hp_of_sitter), "baseline: the bar is already sent and nothing changed");

        // Sit on a Red Chair, stand, and the client's own 0x00DC from SetSeat.
        let mut sit = vec![0u8; 4];
        sit.extend_from_slice(&3_010_000u32.to_le_bytes());
        sit.extend_from_slice(&1u32.to_le_bytes());
        let _ = sitter.on_chair_sit(&sit);
        let _ = sitter.on_chair_cancel(&0xFFFFu16.to_le_bytes());
        let _ = sitter.on_field_entered();
        let _ = sitter.tick(3_000);

        let seen = partner.tick(3_010);
        let spawn = seen
            .iter()
            .rposition(|r| r.opcode == net::userpool::USER_ENTER_FIELD && about(r) == ids[0])
            .expect("the stand-up's 0x00DC re-announces the stander - the reason the bar empties");
        let hp = seen.iter().rposition(is_hp_of_sitter).expect("the stander's HP is sent again");
        assert!(hp > spawn, "the HP must follow the spawn that creates the CUser it fills");
        assert_eq!(
            u32::from_le_bytes(seen[hp].body[4..8].try_into().unwrap()),
            u32::try_from(sitter.claimed_character().unwrap().hp).unwrap(),
            "the unchanged HP - nothing moved but the chair"
        );
    }

    /// **A party member's HP waits for their own spawn** - the channel-change bug.
    ///
    /// The owner, 2026-09-21: *"the owner changing channels back from channel 2 to channel 1, the owner's HP
    /// bar was unavailable on Tester2's screen until much later."* `world-ch0.log` has the
    /// race exactly: the `0x02B2` left 121 ms after the migration hello and the
    /// `UserEnterField` that creates the `CUser` it describes left 220 ms **after that**, so
    /// the client dropped it - and the unchanged-value cache then suppressed every resend
    /// until regen moved their HP fifteen seconds later.
    ///
    /// A session is claimed and carries a map from `SetField` long before `on_field_entered`
    /// publishes its spawn, and that window is what this pins: no HP goes out during it, and
    /// one does as soon as the spawn has.
    #[test]
    fn a_members_hp_is_not_sent_until_their_own_spawn_has_reached_the_field() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Tester2", "Wisp"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: 104_040_000, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        let mut resident = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut arriving = Session::joining(store.clone(), config.clone(), fields.clone());
        resident.claim_for_character(ids[0]);
        arriving.claim_for_character(ids[1]);
        resident.on_field_entered();
        arriving.on_field_entered();

        let is_hp = |r: &Reply| r.opcode == net::userpool::USER_HP_REMOTE;
        let about = |r: &Reply| u32::from_le_bytes(r.body[0..4].try_into().unwrap());

        // Form the party while both are properly on the field.
        let created = resident
            .run_party_request(ids[0], crate::party::Request::Create { name: "Party".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        let _ = resident.run_party_request(ids[0], crate::party::Request::Invite { target: ids[1] });
        let _ = arriving.tick(1_000);
        let _ = arriving.run_party_request(ids[1], crate::party::Request::Accept { party });
        let _ = resident.tick(1_100);
        let _ = arriving.tick(1_100);
        // Drain what the pre-change session already queued, so the loop below cannot pass or
        // fail on mail that predates the channel change.
        let _ = resident.tick(1_200);
        assert!(
            !resident.tick(1_300).iter().any(|r| is_hp(r) && about(r) == ids[1]),
            "baseline: the mailbox is quiet before the channel change"
        );

        // **The channel change.** A fresh session for the same character: claimed, carrying
        // the map, but its spawn has not been announced - exactly the state the owner's client was
        // in between 01:36:38.045 and 01:36:38.384.
        drop(arriving);
        // A channel change is a new connection, so it stakes a new migration claim.
        store.create_migration(account, ids[1], 0, 0).unwrap();
        let mut rejoined = Session::joining(store.clone(), config, fields);
        rejoined.claim_for_character(ids[1]);
        assert!(rejoined.claimed_character().is_some(), "the rejoined session must hold the character");
        for t in [2_000, 2_100, 2_200] {
            let _ = rejoined.tick(t);
            let seen = resident.tick(t + 10);
            assert!(
                !seen.iter().filter(|r| is_hp(r)).any(|r| about(r) == ids[1]),
                "no HP about the arriving member before their spawn: {seen:?}"
            );
        }

        // The spawn goes out, and only now does the HP follow it.
        rejoined.on_field_entered();
        eprintln!("DEBUG map_of_self={:?} chr_map={:?}", rejoined.bus().map_of(rejoined.subscriber), rejoined.claimed_character().map(|c| c.map_id));
        let _ = rejoined.tick(3_000);
        let seen = resident.tick(3_100);
        assert!(
            seen.iter().filter(|r| is_hp(r)).any(|r| about(r) == ids[1]),
            "once the spawn has gone out the HP follows: {seen:?}"
        );
    }

    /// **The "First Time Together" gate, and the party landing on stage 1 together.**
    ///
    /// The owner, 2026-09-22: *"The entry fails if the person talking is not a party leader, if
    /// the person is not in a party with all members that are level 21 or above, is not in
    /// a party of at least two players... Once the entry requirement is fulfilled, all party
    /// members will be teleported into Stage 1."*
    ///
    /// Two sessions in Kerning City. A solo leader is refused; a member who is not the
    /// leader is refused; an under-levelled party is refused and **nobody moves**; and once
    /// the party is two members both at 21, the Yes puts the leader AND the member on
    /// `80000000` - the member through `Event::PartyQuestEnter`, on their own next tick.
    #[test]
    fn lakelis_gates_the_party_quest_and_pulls_the_whole_party_into_stage_one() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Leader", "Member"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: crate::firsttime::ENTRY_MAP,
                level: 21,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, crate::firsttime::ENTRY_MAP).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        let mut cfg = (*config).clone();
        cfg.npcs.insert(
            crate::firsttime::ENTRY_MAP,
            vec![net::opcode::FieldNpc { object_id: 900, template_id: crate::firsttime::LAKELIS, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 }],
        );
        cfg.fields.insert(crate::firsttime::ENTRY_MAP);
        cfg.fields.insert(crate::firsttime::STAGE_1);
        let config = Arc::new(cfg);
        let mut leader = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut member = Session::joining(store.clone(), config, fields);
        leader.claim_for_character(ids[0]);
        member.claim_for_character(ids[1]);
        leader.on_field_entered();
        member.on_field_entered();

        let click = |object_id: u32| {
            let mut b = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
            b.extend_from_slice(&object_id.to_le_bytes());
            b.extend_from_slice(&0i16.to_le_bytes());
            b.extend_from_slice(&0i16.to_le_bytes());
            b.extend_from_slice(&u32::MAX.to_le_bytes());
            b
        };
        // Lakelis' menu, answered: u32 handle, u8 6, u8 accepted, u32 line. `yes` is the
        // "enter" line - the name is from when they asked a yes/no.
        let pick = |line: u32| {
            let mut b = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
            b.extend_from_slice(&0u32.to_le_bytes());
            b.push(net::script::SCRIPT_TYPE_MENU);
            b.push(1);
            b.extend_from_slice(&line.to_le_bytes());
            b
        };
        let yes = || pick(crate::firsttime::LAKELIS_ENTER);
        let map_of = |s: &Session, id: u32| {
            s.store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().map_id
        };
        let said = |out: &[Reply]| {
            out.iter()
                .find(|r| r.opcode == net::script::SCRIPT_MESSAGE)
                .map(|r| String::from_utf8_lossy(&r.body).to_string())
                .unwrap_or_default()
        };

        // 1. Clicking their opens their line as a yes/no.
        let out = leader.handle(&click(900));
        assert!(said(&out).contains("great teamwork"), "their own line: {}", said(&out));

        // 2. Alone: refused, and nobody moves. A player on their own is in NO party rather
        //    than in a party of one, so this is the NoParty line - `firsttime::check`'s unit
        //    tests cover the party-of-one case, which needs members to have left.
        let out = leader.handle(&yes());
        assert!(said(&out).contains("not for one adventurer"), "alone: {}", said(&out));
        assert_eq!(map_of(&leader, ids[0]), crate::firsttime::ENTRY_MAP, "nobody moved");

        // 3. **A party of one is refused, and nobody moves.** A leader who has made a party
        //    and has nobody in it yet. The owner, 2026-09-23, ending the solo test: *"Remove the
        //    ability to enter the PQ with just a party of 1"* - this is the path that test ran.
        let created = leader.run_party_request(ids[0], crate::party::Request::Create { name: "P".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        let _ = leader.handle(&click(900));
        let out = leader.handle(&yes());
        assert!(said(&out).contains("only 1 of you"), "a party of one: {}", said(&out));
        assert_eq!(map_of(&leader, ids[0]), crate::firsttime::ENTRY_MAP, "a party of one stays in Kerning City");
        let none = leader.fields.runs().instance_of(ids[0]);
        assert_eq!(none, None, "and no run was opened");

        // 4. In a party, but asked by the member rather than the leader.
        let _ = leader.run_party_request(ids[0], crate::party::Request::Invite { target: ids[1] });
        let _ = member.tick(1_000);
        let _ = member.run_party_request(ids[1], crate::party::Request::Accept { party });
        let _ = member.handle(&click(900));
        let out = member.handle(&yes());
        assert!(said(&out).contains("leader"), "not the leader: {}", said(&out));
        assert_eq!(map_of(&member, ids[1]), crate::firsttime::ENTRY_MAP, "and still nobody moved");

        // 4. One member under 21: refused, named, and nobody moves.
        let mut low = store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == ids[1]).unwrap();
        low.level = 20;
        store.save_character_progress(&low).unwrap();
        let _ = leader.handle(&click(900));
        let out = leader.handle(&yes());
        assert!(said(&out).contains("Member") && said(&out).contains("21"), "under-levelled: {}", said(&out));
        assert_eq!(map_of(&leader, ids[0]), crate::firsttime::ENTRY_MAP);
        assert_eq!(map_of(&member, ids[1]), crate::firsttime::ENTRY_MAP);

        // 5. Everyone at 21, but the member has wandered out of Kerning City: refused by
        //    name, and nobody moves. This is the half the unit tests cannot reach - it is
        //    the bus's presence table feeding `online`/`here`.
        low.level = 21;
        store.save_character_progress(&low).unwrap();
        let mut wandered = store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == ids[1]).unwrap();
        member.go_to_map(&mut wandered, 40, 0, "wandering off".to_string());
        let _ = leader.handle(&click(900));
        let out = leader.handle(&yes());
        assert!(said(&out).contains("Kerning City"), "away: {}", said(&out));
        assert_eq!(map_of(&leader, ids[0]), crate::firsttime::ENTRY_MAP, "nobody moved");
        // Back they come.
        member.go_to_map(&mut wandered, crate::firsttime::ENTRY_MAP, 0, "back".to_string());
        member.on_field_entered();

        // 6a. The count line: ten left, and asking spends none.
        let _ = leader.handle(&click(900));
        let out = leader.handle(&pick(crate::firsttime::LAKELIS_COUNT));
        assert!(said(&out).contains("#b10#k more times today"), "{}", said(&out));
        assert_eq!(store.daily_uses_now(ids[0], crate::firsttime::ENTRY_COUNT_KEY).unwrap(), 0, "refusals and asking cost nothing");

        // 6. Everyone at 21 and together: the leader goes, and the member follows.
        low.level = 21;
        store.save_character_progress(&low).unwrap();
        let _ = leader.handle(&click(900));
        let out = leader.handle(&yes());
        assert!(out.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "the leader warps: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
        assert_eq!(map_of(&leader, ids[0]), crate::firsttime::STAGE_1, "leader on stage 1");
        assert_eq!(map_of(&member, ids[1]), crate::firsttime::ENTRY_MAP, "the member has not ticked yet");
        let pulled = member.tick(2_000);
        assert!(pulled.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "the member is pulled in: {:?}", pulled.iter().map(|r| &r.what).collect::<Vec<_>>());
        assert_eq!(map_of(&member, ids[1]), crate::firsttime::STAGE_1, "both on stage 1");

        // And the run is registered as one instance holding both of them.
        let inst = leader.fields.runs().instance_of(ids[0]).expect("an instance is open");
        let other = leader.fields.runs().instance_of(ids[1]);
        assert_eq!(other.map(|i| i.id), Some(inst.id), "one instance, both members");
        assert!(leader.fields.runs().close(inst.id));
        // One entry charged to each of them.
        for id in &ids {
            assert_eq!(store.daily_uses_now(*id, crate::firsttime::ENTRY_COUNT_KEY).unwrap(), 1);
        }

        // 7. **Out of entries.** Back in Kerning City, the member spends the other nine;
        //    the leader is refused by the member's name, nobody moves, and the leader - who
        //    still has nine - is charged nothing.
        let mut l = store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == ids[0]).unwrap();
        leader.go_to_map(&mut l, crate::firsttime::ENTRY_MAP, 0, "home".to_string());
        let mut m = store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == ids[1]).unwrap();
        member.go_to_map(&mut m, crate::firsttime::ENTRY_MAP, 0, "home".to_string());
        leader.on_field_entered();
        member.on_field_entered();
        for _ in 0..9 {
            assert_eq!(store.take_daily_uses_now(&[ids[1]], crate::firsttime::ENTRY_COUNT_KEY, 10).unwrap(), Ok(()));
        }
        let _ = leader.handle(&click(900));
        let out = leader.handle(&yes());
        assert!(said(&out).contains("Member") && said(&out).contains("10 times today"), "spent: {}", said(&out));
        assert_eq!(map_of(&leader, ids[0]), crate::firsttime::ENTRY_MAP, "nobody moved");
        assert_eq!(store.daily_uses_now(ids[0], crate::firsttime::ENTRY_COUNT_KEY).unwrap(), 1, "and the leader was not charged");
        let _ = leader.handle(&click(900));
        let out = leader.handle(&pick(crate::firsttime::LAKELIS_COUNT));
        assert!(said(&out).contains("#b9#k more times today"), "{}", said(&out));
    }

    /// **A level-up redraws the party window for everyone.**
    ///
    /// The owner, 2026-09-22: *"When a party member levels up, the level up does not reflect in
    /// the party list."* `party_block` always read the level correctly; it was simply never
    /// rebuilt, because a level is not a join, a leave or a rights change. Two members: one
    /// gains enough EXP to level, and the OTHER's mailbox gets a `PARTY_STATE` whose seat
    /// for the leveller carries the new number.
    #[test]
    fn a_level_up_re_sends_the_party_window_to_every_member() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Leader", "Member"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: 104_040_000, level: 1, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        // A curve, or nothing levels and the test passes for the wrong reason.
        let curve = crate::expcurve::ExpCurve::parse("1 | 15
2 | 34
3 | 57
");
        let config = Arc::new(Config { exp_curve: curve, ..(*config).clone() });
        let mut leader = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut member = Session::joining(store.clone(), config, fields);
        leader.claim_for_character(ids[0]);
        member.claim_for_character(ids[1]);
        leader.on_field_entered();
        member.on_field_entered();
        let created = leader.run_party_request(ids[0], crate::party::Request::Create { name: "P".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        let _ = leader.run_party_request(ids[0], crate::party::Request::Invite { target: ids[1] });
        let _ = member.tick(1_000);
        let _ = member.run_party_request(ids[1], crate::party::Request::Accept { party });
        let _ = leader.tick(1_100);
        let _ = member.tick(1_100);

        // The member levels. Enough EXP that the curve must carry them past level 5.
        let before = store.character_brief(ids[1]).unwrap().unwrap().level;
        let own = member.award_experience(20, "a test", true, false);
        let after = store.character_brief(ids[1]).unwrap().unwrap().level;
        assert!(after > before, "the member actually levelled: {before} -> {after}");

        // Their own client is told, and so is the leader's.
        // PARTY_STATE on the wire: u8 code, u8 1, u32 partyId, u8 0, then six members.
        // A member is u32 charId (0 ends an empty seat), str name, u32 job, u32, u32 LEVEL,
        // u32, u8, u32, u64, then 0x78 zero bytes - `net::party::write_member`.
        let seat_level = |r: &Reply| -> Option<u32> {
            let b = &r.body;
            if b.first() != Some(&net::party::result::PARTY_STATE) || b.get(1) != Some(&1) {
                return None;
            }
            let mut at = 2 + 4 + 1;
            for _ in 0..net::party::PARTY_SEATS {
                let id = u32::from_le_bytes(b[at..at + 4].try_into().ok()?);
                at += 4;
                if id == 0 {
                    continue;
                }
                let n = u16::from_le_bytes(b[at..at + 2].try_into().ok()?) as usize;
                at += 2 + n;
                let level = u32::from_le_bytes(b[at + 8..at + 12].try_into().ok()?);
                if id == ids[1] {
                    return Some(level);
                }
                at += 4 * 4 + 1 + 4 + 8 + 0x78;
            }
            None
        };
        let own_state: Vec<u32> = own.iter().filter(|r| r.opcode == net::party::PARTY_RESULT).filter_map(seat_level).collect();
        assert_eq!(own_state, vec![after], "the leveller's own window: {own_state:?}");

        let mail = leader.tick(2_000);
        let seen: Vec<u32> = mail.iter().filter(|r| r.opcode == net::party::PARTY_RESULT).filter_map(seat_level).collect();
        assert_eq!(seen, vec![after], "the other member's window carries the NEW level: {seen:?}");

        // A gain that does not level sends no party packet at all.
        let quiet = member.award_experience(1, "a crumb", true, false);
        assert!(
            !quiet.iter().any(|r| r.opcode == net::party::PARTY_RESULT),
            "no level, no redraw: {:?}",
            quiet.iter().map(|r| &r.what).collect::<Vec<_>>()
        );
    }

    /// **The quest's clock, Nella, and leaving the party.** The owner, 2026-09-22.
    ///
    /// Four claims: entering sends the type-2 countdown (never type 1, which would throw on
    /// these clock-less maps); Nella inside offers the way out and Yes lands them on the
    /// Exit; Nella on the Exit sends them to Kerning City; and a member who leaves the party
    /// is taken out of the quest without touching Nella at all.
    #[test]
    fn the_party_quest_counts_down_and_nella_shows_people_out() {
        use crate::firsttime;
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Runner".to_string(), map_id: firsttime::STAGE_1, level: 21, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.set_character_map(id, firsttime::STAGE_1).unwrap();
        store.create_migration(account, id, 0, 0).unwrap();
        let mut cfg = (*config).clone();
        for m in [firsttime::STAGE_1, firsttime::EXIT_MAP, firsttime::TOWN_MAP] {
            cfg.fields.insert(m);
        }
        cfg.npcs.insert(
            firsttime::STAGE_1,
            vec![net::opcode::FieldNpc { object_id: 910, template_id: firsttime::NELLA, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 }],
        );
        cfg.npcs.insert(
            firsttime::EXIT_MAP,
            vec![net::opcode::FieldNpc { object_id: 911, template_id: firsttime::NELLA, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 }],
        );
        for (item, name) in [(firsttime::PASS, "Pass"), (firsttime::COUPON, "Coupon"), (4_000_000, "Blue Snail Shell")] {
            cfg.item_names.insert(item, name.into());
        }
        let mut s = Session::joining(store.clone(), Arc::new(cfg), fields.clone());
        s.claim_for_character(id);
        let inst = fields.runs().open(9_001, vec![id], store::Store::unix_now());
        let held = |item: u32| -> u32 {
            store.bag_items(id, store::InventoryType::Etc).unwrap().iter().filter(|r| r.item.item_id == item).map(|r| u32::from(r.item.kind.quantity())).sum()
        };
        let _ = s.give_item(firsttime::PASS, 3, "test").unwrap();
        let _ = s.give_item(firsttime::COUPON, 5, "test").unwrap();
        let _ = s.give_item(4_000_000, 2, "test").unwrap();

        let click = |object_id: u32| {
            let mut b = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
            b.extend_from_slice(&object_id.to_le_bytes());
            b.extend_from_slice(&0i16.to_le_bytes());
            b.extend_from_slice(&0i16.to_le_bytes());
            b.extend_from_slice(&u32::MAX.to_le_bytes());
            b
        };
        let yes = || {
            let mut b = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
            b.extend_from_slice(&0u32.to_le_bytes());
            b.push(0);
            b.extend_from_slice(&0u32.to_le_bytes());
            b.extend_from_slice(&0u16.to_le_bytes());
            b.push(net::script::SCRIPT_ACTION_YES as u8);
            b
        };
        let map_now = |s: &Session| s.store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().map_id;

        // 1. Entering sends the countdown, as type 2 and with the limit on it.
        let entry = s.on_field_entered();
        let clock = entry.iter().find(|r| r.opcode == net::clock::FIELD_CLOCK).expect("the countdown");
        assert_eq!(clock.body[0], net::clock::CLOCK_TYPE_SECONDS, "type 2 - type 1 would throw here");
        let left = u32::from_le_bytes(clock.body[1..5].try_into().unwrap());
        assert!(left > firsttime::TIME_LIMIT_S - 5 && left <= firsttime::TIME_LIMIT_S, "{left}s of {}", firsttime::TIME_LIMIT_S);

        // 2. Nella inside asks, and Yes puts them on the Exit map.
        let out = s.handle(&click(910));
        let said = out.iter().find(|r| r.opcode == net::script::SCRIPT_MESSAGE).map(|r| String::from_utf8_lossy(&r.body).to_string()).unwrap_or_default();
        assert!(said.contains("leave this place"), "{said}");
        let out = s.handle(&yes());
        assert!(out.iter().any(|r| r.opcode == net::opcode::SET_FIELD));
        assert_eq!(map_now(&s), firsttime::EXIT_MAP, "out to the Exit");
        assert_eq!(fields.runs().instance_of(id), None, "and out of the run");
        assert_eq!((held(firsttime::PASS), held(firsttime::COUPON)), (3, 5), "Nella INSIDE takes nothing");

        // 3. Nella on the Exit map sends them to Kerning City - and takes every Pass and
        //    Coupon the moment they are spoken to. The owner, 2026-09-24: *"They may not be taken
        //    outside of the Party Quest area."* The snail shells are the control.
        let out = s.handle(&click(911));
        assert_eq!((held(firsttime::PASS), held(firsttime::COUPON)), (0, 0), "none leave the quest");
        assert_eq!(held(4_000_000), 2, "anything else is theirs to keep");
        let lines: Vec<Vec<u8>> = out.iter().filter(|r| r.opcode == net::stats::USER_EFFECT_LOCAL).map(|r| r.body.clone()).collect();
        assert_eq!(
            lines,
            vec![net::message::item_lost_in_chat(firsttime::COUPON, 5), net::message::item_lost_in_chat(firsttime::PASS, 3)],
            "a grey line for each"
        );
        assert_eq!(out.last().map(|r| r.opcode), Some(net::script::SCRIPT_MESSAGE), "and then they ask");
        let said = out.iter().find(|r| r.opcode == net::script::SCRIPT_MESSAGE).map(|r| String::from_utf8_lossy(&r.body).to_string()).unwrap_or_default();
        assert!(said.contains("Kerning City"), "{said}");
        let _ = s.handle(&yes());
        assert_eq!(map_now(&s), firsttime::TOWN_MAP, "home");
        assert!(fields.runs().close(inst.id) || true);

        // 4. **Leaving the party takes you out**, with no Nella involved. Back into a run,
        //    on a stage, then ejected.
        let mut back = store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
        s.go_to_map(&mut back, firsttime::STAGE_1, 0, "back in".to_string());
        let inst = fields.runs().open(9_002, vec![id], store::Store::unix_now());
        assert_eq!(fields.runs().instance_of(id).map(|i| i.id), Some(inst.id));
        let _ = s.eject_from_party_quest(id, "they left the party");
        assert_eq!(map_now(&s), firsttime::EXIT_MAP, "the leaver lands on the Exit");
        assert_eq!(fields.runs().instance_of(id), None);

        // 5. **The clock running out** ejects whoever is still inside.
        let mut back = store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
        s.go_to_map(&mut back, firsttime::STAGE_1, 0, "back in again".to_string());
        let expiring = fields.runs().open(9_003, vec![id], store::Store::unix_now() - i64::from(firsttime::TIME_LIMIT_S));
        assert!(expiring.is_over(store::Store::unix_now()));
        let _ = s.party_quest_timer_tick();
        assert_eq!(map_now(&s), firsttime::EXIT_MAP, "time up, out to the Exit");
        assert_eq!(fields.runs().instance_of(id), None, "and the run is forgotten");
    }

    /// **Two parties on one party-quest map cannot see each other, and their mobs are
    /// separate pools.** The owner, 2026-09-22: *"every party's PQ instance will be independent.
    /// Other parties can be in the same map in the same channel, but however people from
    /// other parties will deliberately not see other parties on the same map because the
    /// server does not relay that information. All mobs are also instanced per party."*
    ///
    /// Four characters, two parties, all four on `80000000`. The control is the pair in the
    /// SAME run: without it this test would pass on a server that simply never relays
    /// anything at all.
    #[test]
    fn two_parties_on_one_quest_map_neither_see_each_other_nor_share_mobs() {
        use crate::firsttime;
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut cfg = (*config).clone();
        cfg.fields.insert(firsttime::STAGE_1);
        // One spawn point, so "separate pools" is a number rather than an impression.
        cfg.mobs.insert(firsttime::STAGE_1, vec![net::mob::FieldMob::new(2000, 800_000, 100, 395, 1, 30)]);
        let config = Arc::new(cfg);

        let mut ids = Vec::new();
        let mut sessions = Vec::new();
        for name in ["Alfa", "Alto", "Bravo", "Bongo"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: firsttime::STAGE_1, level: 21, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, firsttime::STAGE_1).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            ids.push(id);
            sessions.push(s);
        }
        // Two runs: A1+A2, and B1+B2.
        let run_a = fields.runs().open(5_001, vec![ids[0], ids[1]], store::Store::unix_now());
        let run_b = fields.runs().open(5_002, vec![ids[2], ids[3]], store::Store::unix_now());
        assert_ne!(run_a.id, run_b.id);

        for s in sessions.iter_mut() {
            let _ = s.on_field_entered();
        }
        // Drain, so what follows is only what the entries caused.
        for s in sessions.iter_mut() {
            let _ = s.tick(1_000);
        }

        // **The keys differ, and that is the mechanism.**
        let key = |s: &Session| s.field();
        assert_eq!(key(&sessions[0]), key(&sessions[1]), "one run, one field");
        assert_eq!(key(&sessions[2]), key(&sessions[3]), "and the same for the other");
        assert_ne!(key(&sessions[0]), key(&sessions[2]), "two runs, two fields");
        assert_eq!(key(&sessions[0]).map, firsttime::STAGE_1, "both on the same MAP, though");
        assert_eq!(key(&sessions[2]).map, firsttime::STAGE_1);

        // **A1 moves. A2 hears it; B1 and B2 do not.**
        let spoke = Reply { opcode: 0x02B0, body: vec![1, 2, 3], what: "A1 moved".into() };
        sessions[0].bus().publish(sessions[0].subscriber, key(&sessions[0]), spoke, None);
        let heard = |s: &mut Session, t: u64| s.tick(t).iter().any(|r| r.what == "A1 moved");
        assert!(heard(&mut sessions[1], 2_000), "the partner in the same run hears it");
        assert!(!heard(&mut sessions[2], 2_000), "the other party does not");
        assert!(!heard(&mut sessions[3], 2_000), "nor their partner");

        // **The mobs are separate pools.** One spawn point per instance, so killing A's mob
        // must not touch B's.
        fields.seed(key(&sessions[0]), &sessions[0].config, 0);
        fields.seed(key(&sessions[2]), &sessions[2].config, 0);
        let _ = fields.due_respawns(key(&sessions[0]), &sessions[0].config, 999_999);
        let _ = fields.due_respawns(key(&sessions[2]), &sessions[2].config, 999_999);
        assert_eq!(fields.mob_count(key(&sessions[0])), 1, "A has its own Ligator");
        assert_eq!(fields.mob_count(key(&sessions[2])), 1, "B has its own");
        let a_mob = fields.mobs_on(key(&sessions[0]))[0].spawn.object_id;
        fields.hurt(key(&sessions[0]), a_mob, 9_999, ids[0], &sessions[0].config, 0);
        assert_eq!(fields.mob_count(key(&sessions[0])), 0, "A's is dead");
        assert_eq!(fields.mob_count(key(&sessions[2])), 1, "B's is untouched - separate pools");

        let closed_a = fields.runs().close(run_a.id);
        let closed_b = fields.runs().close(run_b.id);
        assert!(closed_a && closed_b);
    }

    /// **Stage 1: questions, Coupons, Passes.** The owner, 2026-09-23.
    ///
    /// A party of three - Leader, Mote, Nook - on stage 1. Claims, in order: a member's first
    /// click is the intro (without "except the party leader") and NOT a menu; its Next deals a
    /// question from the owner's eight, which a second visit cannot reroll; a wrong count - none,
    /// then one too many - repeats the question and pays nothing; the exact count pays one
    /// Pass and takes exactly those Coupons; a second visit pays no second Pass. The leader
    /// gets the two-line menu; its question line leads to the same intro; handing in fewer
    /// Passes than three is refused and takes none; three clears the stage - the three
    /// effects, reaching the other members - and costs exactly three.
    #[test]
    fn stage_one_is_questions_coupons_and_passes() {
        use crate::firsttime;
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut cfg = (*config).clone();
        cfg.fields.insert(firsttime::STAGE_1);
        cfg.npcs.insert(
            firsttime::STAGE_1,
            vec![net::opcode::FieldNpc { object_id: 920, template_id: firsttime::CLOTO, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 }],
        );
        for item in [firsttime::COUPON, firsttime::PASS] {
            cfg.item_names.insert(item, format!("item {item}"));
        }
        // Level 21 needs 1000 for 22, so 5% is 50.
        cfg.exp_curve = crate::expcurve::ExpCurve::parse("21 | 1000
22 | 1200
");
        let config = Arc::new(cfg);
        let mut ids = Vec::new();
        let mut sessions = Vec::new();
        for name in ["Leader", "Mote", "Nook"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: firsttime::STAGE_1, level: 21, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, firsttime::STAGE_1).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            ids.push(id);
            sessions.push(s);
        }
        let created = sessions[0].run_party_request(ids[0], crate::party::Request::Create { name: "P".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        for i in 1..3 {
            let _ = sessions[0].run_party_request(ids[0], crate::party::Request::Invite { target: ids[i] });
            let _ = sessions[i].tick(500);
            let _ = sessions[i].run_party_request(ids[i], crate::party::Request::Accept { party });
        }
        assert_eq!(fields.parties().party_of(ids[0]).map(|p| p.members.len()), Some(3));
        let run = fields.runs().open(5_301, ids.clone(), store::Store::unix_now());
        for s in sessions.iter_mut() {
            let _ = s.on_field_entered();
        }
        for s in sessions.iter_mut() {
            let _ = s.tick(1_000);
        }

        let mut click = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
        click.extend_from_slice(&920u32.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&u32::MAX.to_le_bytes());
        let next = {
            let mut b = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
            b.extend_from_slice(&0u32.to_le_bytes());
            b.push(0);
            b.extend_from_slice(&0u32.to_le_bytes());
            b.extend_from_slice(&0u16.to_le_bytes());
            b.push(net::script::SCRIPT_ACTION_YES as u8);
            b
        };
        let pick = |line: u32| {
            let mut b = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
            b.extend_from_slice(&0u32.to_le_bytes());
            b.push(net::script::SCRIPT_TYPE_MENU);
            b.push(1);
            b.extend_from_slice(&line.to_le_bytes());
            b
        };
        let said = |out: &[Reply]| {
            out.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).map(|r| String::from_utf8_lossy(&r.body).to_string()).collect::<String>()
        };
        let is_menu = |out: &[Reply]| out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE && r.body.get(10) == Some(&net::script::SCRIPT_TYPE_MENU));
        let effects = |out: &[Reply]| out.iter().filter(|r| r.opcode == net::fieldeffect::FIELD_EFFECT).count();
        let held = |id: u32, item: u32| -> u32 {
            store.bag_items(id, store::InventoryType::Etc).unwrap().iter().filter(|r| r.item.item_id == item).map(|r| u32::from(r.item.kind.quantity())).sum()
        };
        // The grey chat-log item lines (the owner, 2026-09-24), in the order they were sent.
        let item_lines = |out: &[Reply]| -> Vec<Vec<u8>> {
            out.iter().filter(|r| r.opcode == net::stats::USER_EFFECT_LOCAL).map(|r| r.body.clone()).collect()
        };

        // 1. Mote's first click: the intro, with a Next, and no menu.
        let out = sessions[1].handle(&click);
        assert!(said(&out).contains("welcome the first stage"), "{}", said(&out));
        assert!(!said(&out).contains("except the party leader"));
        assert!(!is_menu(&out), "a member gets no menu");
        // 2. Next deals a question from the owner's list.
        let out = sessions[1].handle(&next);
        let q = fields.runs().instance_of(ids[1]).unwrap().questions.iter().find(|(c, _)| *c == ids[1]).map(|(_, q)| *q).expect("dealt");
        let (text, answer) = firsttime::QUESTIONS[q];
        assert!(said(&out).contains(text), "{}", said(&out));
        // 3. None, then one too many: the question again, and no Pass.
        let out = sessions[1].handle(&click);
        assert!(said(&out).contains(text) && said(&out).contains("not the right number"), "{}", said(&out));
        let _ = sessions[1].give_item(firsttime::COUPON, u16::try_from(answer + 1).unwrap(), "test").unwrap();
        let out = sessions[1].handle(&click);
        assert!(said(&out).contains("not the right number"), "one too many is wrong too: {}", said(&out));
        assert!(item_lines(&out).is_empty(), "a wrong answer moves nothing, so says nothing");
        assert_eq!(held(ids[1], firsttime::PASS), 0);
        assert_eq!(held(ids[1], firsttime::COUPON), answer + 1, "a wrong answer costs nothing");
        let same = fields.runs().instance_of(ids[1]).unwrap().questions.iter().find(|(c, _)| *c == ids[1]).map(|(_, q)| *q);
        assert_eq!(same, Some(q), "revisiting does not reroll");
        // 4. Exactly the answer: one Pass, the Coupons gone.
        let _ = sessions[1].take_items(ids[1], store::InventoryType::Etc, firsttime::COUPON, 1);
        let out = sessions[1].handle(&click);
        assert!(said(&out).contains("That is right"), "{}", said(&out));
        assert_eq!(held(ids[1], firsttime::PASS), 1);
        assert_eq!(held(ids[1], firsttime::COUPON), 0);
        assert_eq!(
            item_lines(&out),
            vec![net::message::item_gained_in_chat(firsttime::PASS, 1), net::message::item_lost_in_chat(firsttime::COUPON, answer)],
            "the Pass earned and the Coupons lost, each a line in the chat log"
        );
        // 5. And only one.
        let _ = sessions[1].give_item(firsttime::COUPON, u16::try_from(answer).unwrap(), "test").unwrap();
        let out = sessions[1].handle(&click);
        assert!(said(&out).contains("already completed"), "{}", said(&out));
        assert_eq!(held(ids[1], firsttime::PASS), 1, "one Pass per member per run");

        // 6. The leader gets the menu; the question line is the same intro.
        let out = sessions[0].handle(&click);
        assert!(is_menu(&out), "the leader gets the two-line menu");
        let out = sessions[0].handle(&pick(firsttime::CLOTO_MENU_QUESTION));
        assert!(said(&out).contains("welcome the first stage"), "{}", said(&out));
        // 7. Two Passes of three: refused, nothing taken, nothing cleared.
        let _ = sessions[0].give_item(firsttime::PASS, 2, "test").unwrap();
        let _ = sessions[0].handle(&click);
        let out = sessions[0].handle(&pick(firsttime::CLOTO_MENU_PASSES));
        assert!(said(&out).contains("I need"), "{}", said(&out));
        assert_eq!(effects(&out), 0);
        assert_eq!(held(ids[0], firsttime::PASS), 2, "a refusal takes nothing");
        assert!(item_lines(&out).is_empty(), "and says nothing was lost");
        // 8. Three: the stage clears for everybody here, and it costs exactly three.
        let _ = sessions[0].give_item(firsttime::PASS, 2, "test").unwrap();
        let _ = sessions[0].handle(&click);
        let out = sessions[0].handle(&pick(firsttime::CLOTO_MENU_PASSES));
        assert_eq!(effects(&out), 3, "banner, fanfare, gate: {}", said(&out));
        assert_eq!(held(ids[0], firsttime::PASS), 1, "four held, three taken");
        assert_eq!(item_lines(&out), vec![net::message::item_lost_in_chat(firsttime::PASS, 3)], "the three Passes handed in");
        assert_eq!(effects(&sessions[2].tick(2_000)), 3, "Nook, same run, sees the clear");
        // **The clear pays 5% of the next level to every member** - the leader now, the
        // others on their own tick.
        let _ = sessions[1].tick(2_000);
        let exp_of = |id: u32| store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().exp;
        for id in &ids {
            assert_eq!(exp_of(*id), 50, "5% of 1000 for character {id}");
        }
        let cleared = fields.runs().is_cleared(ids[0], firsttime::STAGE_1);
        assert!(cleared);
        let closed = fields.runs().close(run.id);
        assert!(closed);
    }

    /// **Stage 2: the ropes.** The owner, 2026-09-23. A pair - Leader and Mote - on stage 2.
    ///
    /// Claims: Mote gets the intro, with the pair's own number in it; the leader gets it too
    /// while nobody is on a rope; one on a rope is a count line and no effect; two on the
    /// wrong pair plays WRONG - screen and sound, to Mote as well - and **no dialogue at
    /// all**; two on the dealt pair clears the stage. A rope is only counted from a position
    /// inside the client's `area` rectangle, so the bottom of a rope does not count.
    #[test]
    fn stage_two_is_the_right_ropes_or_wrong() {
        use crate::firsttime;
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut cfg = (*config).clone();
        cfg.fields.insert(firsttime::STAGE_2);
        cfg.npcs.insert(
            firsttime::STAGE_2,
            vec![net::opcode::FieldNpc { object_id: 921, template_id: firsttime::CLOTO, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 }],
        );
        let config = Arc::new(cfg);
        let mut ids = Vec::new();
        let mut sessions = Vec::new();
        for name in ["Leader", "Mote"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: firsttime::STAGE_2, level: 21, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, firsttime::STAGE_2).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            ids.push(id);
            sessions.push(s);
        }
        let created = sessions[0].run_party_request(ids[0], crate::party::Request::Create { name: "P".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        let _ = sessions[0].run_party_request(ids[0], crate::party::Request::Invite { target: ids[1] });
        let _ = sessions[1].tick(500);
        let _ = sessions[1].run_party_request(ids[1], crate::party::Request::Accept { party });
        let run = fields.runs().open(5_401, ids.clone(), store::Store::unix_now());
        for s in sessions.iter_mut() {
            let _ = s.on_field_entered();
        }
        for s in sessions.iter_mut() {
            let _ = s.tick(1_000);
        }

        let mut click = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
        click.extend_from_slice(&921u32.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&u32::MAX.to_le_bytes());
        let said = |out: &[Reply]| {
            out.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).map(|r| String::from_utf8_lossy(&r.body).to_string()).collect::<String>()
        };
        let screens = |out: &[Reply]| -> Vec<String> {
            out.iter().filter(|r| r.opcode == net::fieldeffect::FIELD_EFFECT).map(|r| String::from_utf8_lossy(&r.body[3..]).to_string()).collect()
        };
        let ropes = &firsttime::STAGE_2_ROPES;
        let on = |i: usize| (ropes[i].x1 + 2, ropes[i].y1 + 20);

        // 1. Mote: the intro, with "Two".
        let out = sessions[1].handle(&click);
        assert!(said(&out).contains("second stage") && said(&out).contains("Two of these"), "{}", said(&out));
        // 2. The leader, nobody on a rope: the intro too.
        let out = sessions[0].handle(&click);
        assert!(said(&out).contains("second stage"), "{}", said(&out));
        // 3. One on a rope; the other at the BOTTOM of a rope, which does not count.
        sessions[0].note_own_position(on(0).0, on(0).1, None);
        sessions[1].note_own_position(-753, 89, None); // rope 0's own bottom (ladderRope 7)
        let out = sessions[0].handle(&click);
        assert!(said(&out).contains("#b2 party members#k must each hang from") && said(&out).contains("only #r1 person#k on the ropes"), "{}", said(&out));
        assert!(screens(&out).is_empty());
        let answer = fields.runs().instance_of(ids[0]).unwrap().answers.iter().find(|(s, _)| *s == firsttime::STAGE_2).map(|(_, a)| a.clone()).expect("dealt");
        assert_eq!(answer.len(), 2, "a pair is dealt two ropes");
        // 4. Two on the wrong pair: WRONG, to both, and not a word from Cloto.
        let wrong: Vec<usize> = (0..4).flat_map(|a| (a + 1..4).map(move |b| vec![a, b])).find(|p| *p != answer).unwrap();
        sessions[0].note_own_position(on(wrong[0]).0, on(wrong[0]).1, None);
        sessions[1].note_own_position(on(wrong[1]).0, on(wrong[1]).1, None);
        let _ = sessions[1].tick(1_500);
        let out = sessions[0].handle(&click);
        assert!(said(&out).is_empty(), "no dialogue on a wrong combination: {}", said(&out));
        let s = screens(&out);
        assert!(s.iter().any(|x| x.contains("quest/party/wrong")) && s.iter().any(|x| x.contains("Party1/Failed")), "{s:?}");
        let theirs = screens(&sessions[1].tick(2_000));
        assert!(theirs.iter().any(|x| x.contains("quest/party/wrong")), "Mote sees WRONG too: {theirs:?}");
        let cleared = fields.runs().is_cleared(ids[0], firsttime::STAGE_2);
        assert!(!cleared);
        // 5. The dealt pair: cleared.
        sessions[0].note_own_position(on(answer[0]).0, on(answer[0]).1, None);
        sessions[1].note_own_position(on(answer[1]).0, on(answer[1]).1, None);
        let out = sessions[0].handle(&click);
        assert!(screens(&out).iter().any(|x| x.contains("quest/party/clear")), "{:?} {}", screens(&out), said(&out));
        let cleared = fields.runs().is_cleared(ids[0], firsttime::STAGE_2);
        assert!(cleared);
        let closed = fields.runs().close(run.id);
        assert!(closed);
    }

    /// **Stage 3: the platforms, for a party of three.** The owner, 2026-09-23: *"The server
    /// randomly selects 2 of these platforms (or 3, in a 3-4 player party)"*.
    ///
    /// Claims: a member gets the stage-3 intro with "three"; three are dealt out of five;
    /// someone on a platform's EDGE is not counted (the count line says 2 of 3); two on one
    /// platform with the right count is WRONG with no dialogue; the dealt three clear it.
    #[test]
    fn stage_three_is_three_of_five_platforms_for_a_party_of_three() {
        use crate::firsttime;
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut cfg = (*config).clone();
        cfg.fields.insert(firsttime::STAGE_3);
        cfg.npcs.insert(
            firsttime::STAGE_3,
            vec![net::opcode::FieldNpc { object_id: 922, template_id: firsttime::CLOTO, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 }],
        );
        let config = Arc::new(cfg);
        let mut ids = Vec::new();
        let mut sessions = Vec::new();
        for name in ["Leader", "Mote", "Nook"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: firsttime::STAGE_3, level: 21, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, firsttime::STAGE_3).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            ids.push(id);
            sessions.push(s);
        }
        let created = sessions[0].run_party_request(ids[0], crate::party::Request::Create { name: "P".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        for i in 1..3 {
            let _ = sessions[0].run_party_request(ids[0], crate::party::Request::Invite { target: ids[i] });
            let _ = sessions[i].tick(500);
            let _ = sessions[i].run_party_request(ids[i], crate::party::Request::Accept { party });
        }
        let run = fields.runs().open(5_501, ids.clone(), store::Store::unix_now());
        for s in sessions.iter_mut() {
            let _ = s.on_field_entered();
        }
        for s in sessions.iter_mut() {
            let _ = s.tick(1_000);
        }
        let mut click = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
        click.extend_from_slice(&922u32.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&u32::MAX.to_le_bytes());
        let said = |out: &[Reply]| {
            out.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).map(|r| String::from_utf8_lossy(&r.body).to_string()).collect::<String>()
        };
        let screens = |out: &[Reply]| -> Vec<String> {
            out.iter().filter(|r| r.opcode == net::fieldeffect::FIELD_EFFECT).map(|r| String::from_utf8_lossy(&r.body[3..]).to_string()).collect()
        };
        let p = &firsttime::STAGE_3_PLATFORMS;
        // Standing in the middle of platform i, on its floor (the rectangle's bottom edge
        // is just below the floor for every platform).
        let mid = |i: usize| ((p[i].x1 + p[i].x2) / 2, p[i].y2 - 10);
        let stand = |s: &mut Vec<Session>, who: usize, at: (i16, i16)| s[who].note_own_position(at.0, at.1, None);

        let out = sessions[1].handle(&click);
        assert!(said(&out).contains("third stage") && said(&out).contains("three of them are"), "{}", said(&out));

        // Two in the middle of platforms, the third on platform 0's very EDGE (x 600: the
        // footholds start at 597, the rectangle at 608).
        stand(&mut sessions, 0, mid(0));
        stand(&mut sessions, 1, mid(1));
        stand(&mut sessions, 2, (600, -135));
        let out = sessions[0].handle(&click);
        assert!(said(&out).contains("#b3 party members#k must each stand in the middle of") && said(&out).contains("only #r2 people#k on the platforms"), "{}", said(&out));
        let answer = fields.runs().instance_of(ids[0]).unwrap().answers.iter().find(|(s, _)| *s == firsttime::STAGE_3).map(|(_, a)| a.clone()).expect("dealt");
        assert_eq!(answer.len(), 3, "three of five for a party of three");
        assert!(answer.iter().all(|&i| i < 5));

        // Three counted, but two of them on one platform: WRONG, and no dialogue.
        stand(&mut sessions, 0, mid(answer[0]));
        stand(&mut sessions, 1, (mid(answer[0]).0 + 5, mid(answer[0]).1));
        stand(&mut sessions, 2, mid(answer[1]));
        let out = sessions[0].handle(&click);
        assert!(said(&out).is_empty(), "no dialogue: {}", said(&out));
        assert!(screens(&out).iter().any(|x| x.contains("quest/party/wrong")), "{:?}", screens(&out));

        // The dealt three.
        for (who, &i) in answer.iter().enumerate() {
            stand(&mut sessions, who, mid(i));
        }
        let out = sessions[0].handle(&click);
        assert!(screens(&out).iter().any(|x| x.contains("quest/party/clear")), "{:?} {}", screens(&out), said(&out));
        let cleared = fields.runs().is_cleared(ids[0], firsttime::STAGE_3);
        assert!(cleared);
        let closed = fields.runs().close(run.id);
        assert!(closed);
    }

    /// **Stage 4: the barrels, for a pair.** The owner, 2026-09-23: *"Same concept, 2 correct
    /// barrels generated in a 2 person party, or 3 correct barrels generated in a 3-4 person
    /// party."* Two of six dealt; one on a barrel and one on the ground is a count; the
    /// wrong pair is WRONG with no dialogue; the dealt pair clears.
    #[test]
    fn stage_four_is_two_of_six_barrels_for_a_pair() {
        use crate::firsttime;
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut cfg = (*config).clone();
        cfg.fields.insert(firsttime::STAGE_4);
        cfg.npcs.insert(
            firsttime::STAGE_4,
            vec![net::opcode::FieldNpc { object_id: 923, template_id: firsttime::CLOTO, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 }],
        );
        let config = Arc::new(cfg);
        let mut ids = Vec::new();
        let mut sessions = Vec::new();
        for name in ["Leader", "Mote"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: firsttime::STAGE_4, level: 21, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, firsttime::STAGE_4).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            ids.push(id);
            sessions.push(s);
        }
        let created = sessions[0].run_party_request(ids[0], crate::party::Request::Create { name: "P".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        let _ = sessions[0].run_party_request(ids[0], crate::party::Request::Invite { target: ids[1] });
        let _ = sessions[1].tick(500);
        let _ = sessions[1].run_party_request(ids[1], crate::party::Request::Accept { party });
        let run = fields.runs().open(5_601, ids.clone(), store::Store::unix_now());
        for s in sessions.iter_mut() {
            let _ = s.on_field_entered();
        }
        for s in sessions.iter_mut() {
            let _ = s.tick(1_000);
        }
        let mut click = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
        click.extend_from_slice(&923u32.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&u32::MAX.to_le_bytes());
        let said = |out: &[Reply]| {
            out.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).map(|r| String::from_utf8_lossy(&r.body).to_string()).collect::<String>()
        };
        let screens = |out: &[Reply]| -> Vec<String> {
            out.iter().filter(|r| r.opcode == net::fieldeffect::FIELD_EFFECT).map(|r| String::from_utf8_lossy(&r.body[3..]).to_string()).collect()
        };
        let b = &firsttime::STAGE_4_BARRELS;
        let top = |i: usize| ((b[i].x1 + b[i].x2) / 2, b[i].y2 - 5);

        let out = sessions[1].handle(&click);
        assert!(said(&out).contains("fourth stage") && said(&out).contains("two of them are"), "{}", said(&out));

        sessions[0].note_own_position(top(0).0, top(0).1, None);
        sessions[1].note_own_position(1147, -75, None); // beside Cloto, on the ground
        let out = sessions[0].handle(&click);
        assert!(said(&out).contains("stand on top of a different barrel") && said(&out).contains("only #r1 person#k on the barrels"), "{}", said(&out));
        let answer = fields.runs().instance_of(ids[0]).unwrap().answers.iter().find(|(s, _)| *s == firsttime::STAGE_4).map(|(_, a)| a.clone()).expect("dealt");
        assert_eq!(answer.len(), 2, "two of six for a pair");

        let wrong: Vec<usize> = (0..6).flat_map(|a| (a + 1..6).map(move |c| vec![a, c])).find(|p| *p != answer).unwrap();
        sessions[0].note_own_position(top(wrong[0]).0, top(wrong[0]).1, None);
        sessions[1].note_own_position(top(wrong[1]).0, top(wrong[1]).1, None);
        let out = sessions[0].handle(&click);
        assert!(said(&out).is_empty(), "no dialogue: {}", said(&out));
        assert!(screens(&out).iter().any(|x| x.contains("quest/party/wrong")), "{:?}", screens(&out));

        sessions[0].note_own_position(top(answer[0]).0, top(answer[0]).1, None);
        sessions[1].note_own_position(top(answer[1]).0, top(answer[1]).1, None);
        let out = sessions[0].handle(&click);
        assert!(screens(&out).iter().any(|x| x.contains("quest/party/clear")), "{:?} {}", screens(&out), said(&out));
        let cleared = fields.runs().is_cleared(ids[0], firsttime::STAGE_4);
        assert!(cleared);
        let closed = fields.runs().close(run.id);
        assert!(closed);
    }

    /// **The last stage.** The owner, 2026-09-23, and their follow-up *"The other mobs can respawn,
    /// only the King Slime should not respawn"*. A pair, Leader and Mote, on stage 5, with
    /// the map's real ten spawn points (3 Curse Eyes, 6 Jr. Neckis, the King Slime) and their
    /// real `mobTime`s (180, and -1 for the King).
    ///
    /// Claims: the first entry stands all ten up in the entry batch, not after 180 s; the
    /// second entry adds none; the King Slime's death leaves exactly one pair of shoes per
    /// member, each personal - owned by, shown to and takeable by that member only, even long
    /// after the owner lock - and twenty Slimes where it died; the King never comes back and a
    /// Jr. Necki does, after 180 s; Cloto refuses nine Passes and takes ten, and both members
    /// get a Companion's Magic Box.
    #[test]
    fn the_last_stage_is_ten_mobs_at_once_shoes_each_twenty_slimes_and_ten_passes() {
        use crate::firsttime;
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let stage = firsttime::STAGE_5;
        let mut cfg = (*config).clone();
        cfg.fields.insert(stage);
        cfg.npcs.insert(
            stage,
            vec![net::opcode::FieldNpc { object_id: 924, template_id: firsttime::CLOTO, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 }],
        );
        // gm-handbook/mobs.txt, map 80000400, in its own order.
        let points: Vec<(u32, i16, i16, i32)> = vec![
            (800_002, 297, -2175, 180), (800_002, 132, -2175, 180), (800_002, 433, -2175, 180),
            (800_001, -406, -1455, 180), (800_001, -187, -1455, 180), (800_001, 80, -1455, 180),
            (800_001, 435, -1455, 180), (800_001, 653, -1455, 180),
            (firsttime::KING_SLIME, 162, -435, -1),
            (800_001, 247, -1455, 180),
        ];
        let mut mobs = Vec::new();
        for (i, &(template, x, y, mob_time)) in points.iter().enumerate() {
            let id = 1_000 + i as u32;
            mobs.push(net::mob::FieldMob::new(id, template, x, y, 1, 100));
            cfg.mob_respawn_s.insert((stage, id), mob_time);
        }
        cfg.mobs.insert(stage, mobs);
        for (item, name) in [(firsttime::PASS, "Pass"), (firsttime::COMPANIONS_MAGIC_BOX, "Companion's Magic Box")] {
            cfg.item_names.insert(item, name.into());
        }
        cfg.exp_curve = crate::expcurve::ExpCurve::parse("21 | 1000
22 | 1200
");
        let config = Arc::new(cfg);
        let mut ids = Vec::new();
        let mut sessions = Vec::new();
        for name in ["Leader", "Mote"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: stage, level: 21, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, stage).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            ids.push(id);
            sessions.push(s);
        }
        let created = sessions[0].run_party_request(ids[0], crate::party::Request::Create { name: "P".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        let _ = sessions[0].run_party_request(ids[0], crate::party::Request::Invite { target: ids[1] });
        let _ = sessions[1].tick(500);
        let _ = sessions[1].run_party_request(ids[1], crate::party::Request::Accept { party });
        let run = fields.runs().open(5_701, ids.clone(), store::Store::unix_now());
        let key = crate::fields::FieldKey::instanced(stage, run.id);
        let entered = |out: &[Reply]| out.iter().filter(|r| r.opcode == net::mob::MOB_ENTER_FIELD).count();

        // 1. All ten, in the first entry's batch.
        let out = sessions[0].on_field_entered();
        assert_eq!(entered(&out), 10, "all ten at once, not after 180 s");
        assert_eq!(fields.mob_count(key), 10);
        // 2. The second member walking in adds none.
        let out = sessions[1].on_field_entered();
        assert_eq!(entered(&out), 10, "Mote is shown the same ten");
        assert_eq!(fields.mob_count(key), 10, "and no second fill");
        let _ = sessions[0].tick(1_000);
        let _ = sessions[1].tick(1_000);

        // 3. The King Slime dies.
        let king = fields.mobs_on(key).into_iter().find(|m| m.spawn.template_id == firsttime::KING_SLIME).expect("the King").spawn.object_id;
        let out = sessions[0].deal_to_mob(key, king, 1_000_000, ids[0]);
        let slimes = out.iter().filter(|r| r.opcode == net::mob::MOB_ENTER_FIELD).count();
        assert_eq!(slimes, firsttime::SLIMES_FROM_THE_KING, "twenty Slimes");
        let slime_count = fields.mobs_on(key).iter().filter(|m| m.spawn.template_id == firsttime::SLIME).count();
        assert_eq!(slime_count, 20);
        let shoes: Vec<crate::drops::LiveDrop> =
            fields.with_drops(key, |d| d.on_field(key).filter(|x| x.item_id() == firsttime::SLIME_SHOES).copied().collect());
        let mut owners: Vec<u32> = shoes.iter().map(|x| x.owner_id).collect();
        owners.sort_unstable();
        assert_eq!(owners, ids, "one pair each");
        let long_after = 10 * 60 * 1_000;
        for pair in &shoes {
            assert!(pair.personal);
            let other = if pair.owner_id == ids[0] { ids[1] } else { ids[0] };
            assert!(pair.may_be_taken_by(pair.owner_id, long_after, 15_000, &ids));
            assert!(!pair.may_be_taken_by(other, long_after, 15_000, &ids), "not the other member's, ever");
        }
        // The leader's own pair is in the leader's replies; Mote's reaches only Mote.
        assert_eq!(out.iter().filter(|r| r.opcode == net::drops::DROP_ENTER_FIELD).count(), 1, "the leader is shown one pair");
        let hers = sessions[1].tick(2_000);
        let shown_to_her = hers.iter().filter(|r| r.opcode == net::drops::DROP_ENTER_FIELD).count();
        assert_eq!(shown_to_her, 1, "Mote is shown theirs and not the leader's");
        // And re-entering shows each member their own pair only.
        let reentry = fields.with_drops(key, |d| d.field_entry(key, 3_000, ids[1], &crate::mobshare::Party::solo(ids[1])));
        let reentry_both = fields.with_drops(key, |d| {
            d.field_entry(key, 3_000, ids[1], &crate::mobshare::Party::of(ids[1], ids.clone()))
        });
        assert_eq!(reentry.len(), 1);
        assert_eq!(reentry_both.len(), 1, "even as a party member, only their own pair");

        // 4. The King never returns; a Jr. Necki does, after 180 s.
        let necki = fields.mobs_on(key).into_iter().find(|m| m.spawn.template_id == 800_001).unwrap().spawn.object_id;
        let _ = sessions[0].deal_to_mob(key, necki, 1_000_000, ids[0]);
        let now = sessions[0].clock_ms;
        let early = fields.due_respawns(key, &config, now + 179_000);
        assert!(early.is_empty(), "not before its 180 s");
        let back = fields.due_respawns(key, &config, now + 181_000 + 3_600_000);
        assert_eq!(back.iter().map(|m| m.spawn.object_id).collect::<Vec<_>>(), vec![necki], "the Necki returns, the King does not");
        assert!(!fields.mobs_on(key).iter().any(|m| m.spawn.template_id == firsttime::KING_SLIME));

        // 5. Cloto: nine is short and takes nothing; ten clears, and both are rewarded.
        let mut click = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
        click.extend_from_slice(&924u32.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&u32::MAX.to_le_bytes());
        let held = |id: u32, item: u32, inv: store::InventoryType| -> u32 {
            store.bag_items(id, inv).unwrap().iter().filter(|r| r.item.item_id == item).map(|r| u32::from(r.item.kind.quantity())).sum()
        };
        let _ = sessions[0].give_item(firsttime::PASS, 9, "test").unwrap();
        let out = sessions[0].handle(&click);
        assert_eq!(out.iter().filter(|r| r.opcode == net::fieldeffect::FIELD_EFFECT).count(), 0);
        assert_eq!(held(ids[0], firsttime::PASS, store::InventoryType::Etc), 9, "short: nothing taken");
        let _ = sessions[0].give_item(firsttime::PASS, 1, "test").unwrap();
        let exp_before: Vec<u64> = ids.iter().map(|id| store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == *id).unwrap().exp).collect();
        let out = sessions[0].handle(&click);
        assert_eq!(out.iter().filter(|r| r.opcode == net::fieldeffect::FIELD_EFFECT).count(), 3, "cleared");
        assert_eq!(held(ids[0], firsttime::PASS, store::InventoryType::Etc), 0, "all ten taken");
        assert_eq!(held(ids[0], firsttime::COMPANIONS_MAGIC_BOX, store::InventoryType::Use), 1, "the leader's box");
        // The owner, 2026-09-24: each item that moves is a grey line in the chat log - the ten
        // Passes going, then the box arriving.
        let item_lines = |out: &[Reply]| -> Vec<Vec<u8>> {
            out.iter().filter(|r| r.opcode == net::stats::USER_EFFECT_LOCAL).map(|r| r.body.clone()).collect()
        };
        assert_eq!(
            item_lines(&out),
            vec![
                net::message::item_lost_in_chat(firsttime::PASS, 10),
                net::message::item_gained_in_chat(firsttime::COMPANIONS_MAGIC_BOX, 1),
            ]
        );
        let hers = sessions[1].tick(5_000);
        assert_eq!(held(ids[1], firsttime::COMPANIONS_MAGIC_BOX, store::InventoryType::Use), 1, "Mote's box, through their own session");
        assert_eq!(item_lines(&hers), vec![net::message::item_gained_in_chat(firsttime::COMPANIONS_MAGIC_BOX, 1)], "and their own line for it");
        // 35% of 1000 each, for the last stage - on top of whatever the kills above paid,
        // so measured as the difference across the clear.
        let exp_after: Vec<u64> = ids.iter().map(|id| store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == *id).unwrap().exp).collect();
        assert_eq!(exp_after.iter().zip(&exp_before).map(|(a, b)| a - b).collect::<Vec<_>>(), vec![350, 350]);
        let closed = fields.runs().close(run.id);
        assert!(closed);
    }

    /// **The King Slime's shoes sit beside the Pass, never on it, and no screen has a hole.**
    /// The owner, 2026-09-24: the shoes landed *"right on top of the pass"*, and *"the clients
    /// should also not have weird drop placement such as empty spaces where they do not see a
    /// drop they can pick up because it's instanced for someone else."*
    ///
    /// Claims, with a Pass row and a meso row in the King's table and two members: every
    /// member's pair lands in the SAME slot; no shared drop is in that slot; and the drops each
    /// member can see - the shared ones plus their own pair - are one row at an even
    /// `DROP_STAGGER_PX` spacing, with no two on one spot and no gap. The control for "no gap"
    /// is the old layout, one slot per member: Mote's row would have had a hole where the
    /// leader's pair is.
    #[test]
    fn the_king_slimes_shoes_share_one_slot_after_the_pass() {
        use crate::firsttime;
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let stage = firsttime::STAGE_5;
        let mut cfg = (*config).clone();
        cfg.fields.insert(stage);
        cfg.mobs.insert(stage, vec![net::mob::FieldMob::new(1_000, firsttime::KING_SLIME, 162, -435, 1, 100)]);
        cfg.mob_respawn_s.insert((stage, 1_000), -1);
        cfg.drops = crate::droptables::DropTables::parse(
            "800003 | 4001002 | 100 | 1 | 1 | 1 | Pass\n800003 | 0 | 100 | 10 | 10 | 1 | mesos\n",
        );
        assert!(cfg.footholds.is_empty(), "no floor data, so every x below is the placement's own");
        // Squishy Shoes as gm-handbook/equips.txt has them, so item variance has a template.
        cfg.equips.insert(
            firsttime::SLIME_SHOES,
            crate::config::EquipTemplate { tuc: 5, inc_str: 1, inc_dex: 1, inc_int: 1, inc_luk: 1, inc_pdd: 18, inc_mdd: 7, req_level: 28, ..Default::default() },
        );
        let config = Arc::new(cfg);
        let mut ids = Vec::new();
        let mut sessions = Vec::new();
        for name in ["Leader", "Mote"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: stage, level: 21, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, stage).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            ids.push(id);
            sessions.push(s);
        }
        let created = sessions[0].run_party_request(ids[0], crate::party::Request::Create { name: "P".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        let _ = sessions[0].run_party_request(ids[0], crate::party::Request::Invite { target: ids[1] });
        let _ = sessions[1].tick(500);
        let _ = sessions[1].run_party_request(ids[1], crate::party::Request::Accept { party });
        let run = fields.runs().open(5_702, ids.clone(), store::Store::unix_now());
        let key = crate::fields::FieldKey::instanced(stage, run.id);
        let _ = sessions[0].on_field_entered();
        let _ = sessions[1].on_field_entered();
        let _ = sessions[0].tick(1_000);
        let _ = sessions[1].tick(1_000);

        let _ = sessions[0].deal_to_mob(key, 1_000, 1_000_000, ids[0]);
        let drops: Vec<crate::drops::LiveDrop> = fields.with_drops(key, |d| d.on_field(key).copied().collect());
        let shoes: Vec<_> = drops.iter().filter(|d| d.item_id() == firsttime::SLIME_SHOES).collect();
        let shared: Vec<_> = drops.iter().filter(|d| !d.personal).collect();
        assert_eq!(shoes.len(), 2, "one pair each");
        assert_eq!(shared.len(), 2, "the Pass and the mesos: {drops:?}");
        assert!(shared.iter().any(|d| d.item_id() == firsttime::PASS));

        // Item variance (the owner, 2026-09-24: *"including those dropped by party quests such as
        // Slime Shoes"*): every pair carries its own rolled stats, inside the caps.
        for pair in &shoes {
            let store::ItemKind::Equip(Some(st)) = pair.item.kind else { panic!("unrolled shoes: {pair:?}") };
            assert!((4..=32).contains(&st.stats.inc_pdd) && st.stats.inc_str <= 2, "{:?}", st.stats);
        }
        let slot = (shoes[0].x, shoes[0].y);
        assert!(shoes.iter().all(|d| (d.x, d.y) == slot), "every member's pair in the one slot: {shoes:?}");
        assert!(shared.iter().all(|d| d.x != slot.0), "nothing shared under the shoes: {drops:?}");
        for &viewer in &ids {
            let mut xs: Vec<i16> = drops.iter().filter(|d| !d.personal || d.owner_id == viewer).map(|d| d.x).collect();
            xs.sort_unstable();
            assert_eq!(xs.len(), 3, "character {viewer} sees the Pass, the mesos and their own shoes");
            let gaps: Vec<i16> = xs.windows(2).map(|w| w[1] - w[0]).collect();
            assert_eq!(gaps, vec![crate::drops::DROP_STAGGER_PX; 2], "character {viewer}: one even row, no overlap, no hole: {xs:?}");
            assert!(xs.contains(&162), "the row is still over the corpse: {xs:?}");
        }
        // And what each client is actually sent agrees: three drops each, their own pair only.
        let hers = sessions[1].tick(2_000);
        assert_eq!(hers.iter().filter(|r| r.opcode == net::drops::DROP_ENTER_FIELD).count(), 3);
        let _ = fields.runs().close(run.id);
    }

    /// **King Slime's MP and summon, on the move acknowledgement.** The owner, 2026-09-24: *"There
    /// should be a jump attack, and there should be a summon slime attack."*
    ///
    /// Claims, each against the mob's own data: the King's ack carries its full MP (100),
    /// which is what lets the client's chooser consider its 10-MP jump attack, and offers no
    /// skill while it is above half HP; at half or below it offers skill 200 level 1; a report
    /// that names the skill as used spawns exactly three Slimes where it stands, and the same
    /// report again inside the 15-second interval spawns none and the offer stops. The
    /// control: a Jr. Necki on the same field still gets the old zeros.
    #[test]
    fn king_slime_gets_its_mp_and_its_summon_and_nothing_else_changes() {
        use crate::firsttime;
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let stage = firsttime::STAGE_5;
        let mut cfg = (*config).clone();
        cfg.fields.insert(stage);
        cfg.mobs.insert(
            stage,
            vec![
                net::mob::FieldMob::new(1_000, firsttime::KING_SLIME, 162, -435, 1, 16_820),
                net::mob::FieldMob::new(1_001, 800_001, 80, -1455, 1, 765),
            ],
        );
        cfg.mob_respawn_s.insert((stage, 1_000), -1);
        cfg.mob_respawn_s.insert((stage, 1_001), 180);
        cfg.mob_templates.insert(firsttime::KING_SLIME, crate::config::MobTemplate { max_hp: 16_820, max_mp: 100, ..Default::default() });
        cfg.mob_templates.insert(800_001, crate::config::MobTemplate { max_hp: 765, max_mp: 30, ..Default::default() });
        cfg.mob_templates.insert(firsttime::SLIME, crate::config::MobTemplate { max_hp: 115, ..Default::default() });
        // King Slime's rows exactly as gm-handbook/mobskills.txt has them; the Necki has none.
        cfg.mob_skills = crate::mobskills::MobSkillTable::parse(
            "mob, 800003, 100, 10
attack, 800003, 0, 10, 3000
skill, 800003, 200, 1
level, 200, 1, 0, 15, 50, 15, 7 7 7
",
        );
        let config = Arc::new(cfg);
        let mut ids = Vec::new();
        let mut sessions = Vec::new();
        for name in ["Leader", "Mote"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: stage, level: 30, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, stage).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            ids.push(id);
            sessions.push(s);
        }
        let run = fields.runs().open(5_901, ids.clone(), store::Store::unix_now());
        let key = crate::fields::FieldKey::instanced(stage, run.id);
        let _ = sessions[0].on_field_entered(); // the leader controls both mobs
        let _ = sessions[1].on_field_entered();

        // A minimal 0x02FF: object id, move id, packed, action, the skill command, the rest empty.
        let report_as = |object_id: u32, move_id: u16, action: u8, skill: u64| {
            let mut b = net::mobmove::MOB_MOVE_REQUEST.to_le_bytes().to_vec();
            b.extend_from_slice(&object_id.to_le_bytes());
            b.extend_from_slice(&move_id.to_le_bytes());
            b.push(0);
            b.push(action);
            b.extend_from_slice(&skill.to_le_bytes());
            b.extend_from_slice(&[0u8; 2]);
            b.extend_from_slice(&[0u8, 0u8]);
            b.extend_from_slice(&0u32.to_le_bytes());
            b.extend_from_slice(&[0u8; 22]);
            b.extend_from_slice(&[0u8; net::mobmove::MOB_PATH_HEAD_LEN]);
            b
        };
        let report = |object_id: u32, move_id: u16, skill: u64| report_as(object_id, move_id, 0xFF, skill);
        let ack = |out: &[Reply]| -> (u32, u32, u16) {
            let a = out.iter().find(|r| r.opcode == net::mobmove::MOB_CTRL_ACK).expect("an ack");
            (
                u32::from_le_bytes(a.body[7..11].try_into().unwrap()),
                u32::from_le_bytes(a.body[11..15].try_into().unwrap()),
                u16::from_le_bytes(a.body[15..17].try_into().unwrap()),
            )
        };
        let slimes = || fields.mobs_on(key).iter().filter(|m| m.spawn.template_id == firsttime::SLIME).count();
        let used = 200u64 | (1 << 16);

        // Full HP: MP, no skill.
        let out = sessions[0].handle(&report(1_000, 1, 0));
        assert_eq!(ack(&out), (100, 0, 0), "full MP so the 10-MP jump attack can be chosen; no summon above half");
        // The control: a Jr. Necki still gets zeros.
        let out = sessions[0].handle(&report(1_001, 1, 0));
        assert_eq!(ack(&out), (0, 0, 0), "every other mob is unchanged");

        // **The jump costs 10, once**: attack1 is wire action 13, 26 facing left.
        let out = sessions[0].handle(&report_as(1_000, 2, 26, 0));
        assert_eq!(ack(&out), (90, 0, 0), "the server kept the MP and tells the controller 90");
        let out = sessions[0].handle(&report_as(1_000, 3, 26, 0));
        assert_eq!(ack(&out).0, 90, "the same jump in the next report is not charged again");
        // A skill it does not have, reported: ignored, nothing summoned.
        let _ = sessions[0].handle(&report(1_000, 4, 120 | (1 << 16)));
        assert_eq!(slimes(), 0);

        // Half HP: the summon is offered.
        let _ = sessions[0].deal_to_mob(key, 1_000, 8_410, ids[0]);
        let out = sessions[0].handle(&report(1_000, 5, 0));
        assert_eq!(ack(&out), (90, 200, 1));

        // The client used it: three Slimes, shown to Mote too; then not again for 15 s.
        let out = sessions[0].handle(&report(1_000, 6, used));
        assert_eq!(out.iter().filter(|r| r.opcode == net::mob::MOB_ENTER_FIELD).count(), 3, "three Slimes");
        assert_eq!(slimes(), 3);
        assert_eq!(ack(&out), (90, 0, 0), "cooling down: no offer (the summon costs no MP)");
        let seen = sessions[1].tick(5_000).iter().filter(|r| r.opcode == net::mob::MOB_ENTER_FIELD).count();
        assert_eq!(seen, 3, "the party sees them");
        let _ = sessions[0].handle(&report(1_000, 7, used));
        assert_eq!(slimes(), 3, "a second report inside the interval summons nothing");

        let closed = fields.runs().close(run.id);
        assert!(closed);
    }

    /// **Opening a Companion's Magic Box.** The owner, 2026-09-23, and 2026-10-03 for the four
    /// slots. Two boxes stacked in one Use slot; the `0x0114` for that slot opens ONE:
    /// exactly one prize from EACH slot - an equip, a use item, a scroll, an etc item - in its
    /// table quantity, the slot left holding one box (not emptied - boxes stack), the player
    /// told what they got, and the client's request latch cleared. A packet naming the wrong
    /// slot opens nothing and costs nothing.
    #[test]
    fn a_magic_box_opens_into_one_prize_from_each_slot() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut cfg = (*config).clone();
        cfg.item_names.insert(crate::magicbox::BOX, "Companion's Magic Box".into());
        for slot in &crate::magicbox::SLOTS {
            for &(id, _) in slot.prizes {
                cfg.item_names.insert(id, format!("item {id}"));
            }
        }
        let config = Arc::new(cfg);
        let chr = net::opcode::Character { name: "Opener".to_string(), map_id: 100_000_000, level: 30, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
        s.claim_for_character(id);
        let _ = s.give_item(crate::magicbox::BOX, 2, "test").unwrap();
        let bag = |inv: store::InventoryType| store.bag_items(id, inv).unwrap();
        let box_slot = bag(store::InventoryType::Use).iter().find(|r| r.item.item_id == crate::magicbox::BOX).unwrap().slot;
        let open = |slot: u16| {
            let mut b = net::cashitem::CLIENT_USE_CASH_ITEM.to_le_bytes().to_vec();
            b.extend_from_slice(&0u32.to_le_bytes());
            b.extend_from_slice(&slot.to_le_bytes());
            b.extend_from_slice(&crate::magicbox::BOX.to_le_bytes());
            b
        };
        let held_everywhere = || -> Vec<(u32, u32)> {
            let mut all = Vec::new();
            for inv in [store::InventoryType::Equip, store::InventoryType::Use, store::InventoryType::Etc] {
                for r in bag(inv) {
                    all.push((r.item.item_id, u32::from(r.item.kind.quantity()).max(1)));
                }
            }
            all
        };

        // The wrong slot: nothing opened, nothing spent.
        let out = s.handle(&open(box_slot + 1));
        assert!(!out.is_empty(), "answered, so the client's latch clears");
        assert_eq!(held_everywhere(), vec![(crate::magicbox::BOX, 2)]);

        // The right slot: one box becomes four prizes, one per slot, in slot order.
        let out = s.handle(&open(box_slot));
        let now = held_everywhere();
        let boxes: u32 = now.iter().filter(|(i, _)| *i == crate::magicbox::BOX).map(|(_, q)| q).sum();
        assert_eq!(boxes, 1, "one of the two was opened, and the other is still there");
        let prizes: Vec<(u32, u32)> = now.into_iter().filter(|(i, _)| *i != crate::magicbox::BOX).collect();
        assert_eq!(prizes.len(), 4, "exactly four prizes: {prizes:?}");
        let mut got = Vec::new();
        for slot in &crate::magicbox::SLOTS {
            let mine: Vec<&(u32, u32)> = prizes.iter().filter(|(i, _)| slot.prizes.iter().any(|p| p.0 == *i)).collect();
            assert_eq!(mine.len(), 1, "one {} prize: {prizes:?}", slot.name);
            let (item, qty) = *mine[0];
            let line = slot.prizes.iter().find(|p| p.0 == item).unwrap();
            assert_eq!(qty, u32::from(line.1), "{item} in the table's quantity");
            // And the drop-table page's seven days heard about it (the owner, 2026-10-03).
            assert!(crate::killstats::pending_drops(crate::magicbox::BOX, item) >= 1, "{item} counted for the page");
            got.push((item, qty));
        }
        // The owner, 2026-09-24: the box lost and each prize gained, as grey chat-log lines in
        // that order - and no yellow notice.
        let lines: Vec<&Vec<u8>> = out.iter().filter(|r| r.opcode == net::stats::USER_EFFECT_LOCAL).map(|r| &r.body).collect();
        let mut want = vec![net::message::item_lost_in_chat(crate::magicbox::BOX, 1)];
        want.extend(got.iter().map(|&(item, qty)| net::message::item_gained_in_chat(item, qty)));
        assert_eq!(lines, want.iter().collect::<Vec<_>>(), "one line for the box going, one per prize arriving");
        assert!(!out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE), "no yellow notice");
        assert!(out.len() > lines.len() + 1, "the stack change and the latch release are still sent");

        // The owner, 2026-10-03: Lakelis says what each slot gave, in one NPC box.
        let boxes: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).collect();
        assert_eq!(boxes.len(), 1, "exactly one NPC box");
        let given: Vec<(&str, crate::magicbox::Prize)> =
            crate::magicbox::SLOTS.iter().zip(&got).map(|(slot, &(i, q))| (slot.name, (i, q as u16))).collect();
        assert_eq!(boxes[0].body, net::script::npc_say(crate::firsttime::LAKELIS, &crate::magicbox::lakelis_text(&given), false, false));
        assert_eq!(s.conversation.as_ref().map(|c| c.path.as_str()), Some(crate::questroom::REFUSAL_PATH), "parked so its OK closes it");
        // OK on the box: silence, conversation cleared.
        let mut ok = Vec::new();
        ok.extend_from_slice(&0u32.to_le_bytes());
        ok.push(0);
        ok.extend_from_slice(&0u32.to_le_bytes());
        ok.extend_from_slice(&0u16.to_le_bytes());
        ok.push(net::script::SCRIPT_ACTION_YES as u8);
        let closed = s.on_script_reply(&ok);
        assert!(closed.is_empty(), "{:?}", closed.iter().map(|r| &r.what).collect::<Vec<_>>());
        assert!(s.conversation.is_none());

        // **Never a second box over an open one.** The last box opened while a conversation
        // is up: the prizes and chat lines still come, Lakelis does not.
        s.conversation = Some(crate::session::Conversation {
            npc_template: 2000,
            quest_id: None,
            path: "0".to_string(),
            sent: 0,
            awaiting_yes_no: false,
            sent_with_next: false,
        });
        let out = s.handle(&open(box_slot));
        assert_eq!(out.iter().filter(|r| r.opcode == net::stats::USER_EFFECT_LOCAL).count(), 5, "the box and four prizes, in chat");
        assert!(!out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE), "no second NPC box");
        assert_eq!(s.conversation.as_ref().map(|c| c.npc_template), Some(2000), "the open conversation is untouched");
    }

    /// **A full tab refuses the whole box.** The Etc tab packed with things that do not stack
    /// with any ore: the box stays, nothing from any other slot arrives either, and the player
    /// is told which tab to clear.
    #[test]
    fn a_magic_box_with_no_room_for_one_slot_gives_nothing_and_is_kept() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut cfg = (*config).clone();
        cfg.item_names.insert(crate::magicbox::BOX, "Companion's Magic Box".into());
        for slot in &crate::magicbox::SLOTS {
            for &(id, _) in slot.prizes {
                cfg.item_names.insert(id, format!("item {id}"));
            }
        }
        for n in 0..200 {
            cfg.item_names.insert(4_000_000 + n, format!("filler {n}"));
        }
        let config = Arc::new(cfg);
        let chr = net::opcode::Character { name: "Packed".to_string(), map_id: 100_000_000, level: 30, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
        s.claim_for_character(id);
        let _ = s.give_item(crate::magicbox::BOX, 1, "test").unwrap();
        let etc_slots = store.bag(id).unwrap().slots_in(store::InventoryType::Etc);
        for n in 0..u32::from(etc_slots) {
            let _ = s.give_item(4_000_000 + n, 1, "test").unwrap();
        }
        let box_slot = store.bag_items(id, store::InventoryType::Use).unwrap()[0].slot;
        let mut b = net::cashitem::CLIENT_USE_CASH_ITEM.to_le_bytes().to_vec();
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&box_slot.to_le_bytes());
        b.extend_from_slice(&crate::magicbox::BOX.to_le_bytes());
        let out = s.handle(&b);
        assert!(!out.is_empty(), "answered");
        let uses: Vec<u32> = store.bag_items(id, store::InventoryType::Use).unwrap().iter().map(|r| r.item.item_id).collect();
        assert_eq!(uses, vec![crate::magicbox::BOX], "the box is kept and no use item or scroll arrived");
        assert!(store.bag_items(id, store::InventoryType::Equip).unwrap().is_empty(), "no equip arrived");
        assert_eq!(store.bag_items(id, store::InventoryType::Etc).unwrap().len(), usize::from(etc_slots), "no ore arrived");
    }

    /// **A run of one does not go on.** The owner, 2026-09-23: *"A party of 1 should not be allowed
    /// to continue doing the party quest."*
    ///
    /// A pair on stage 2: Mote disconnects, and the Leader - now alone - is sent to the Exit
    /// with the reason, and the run is gone. The control: a trio where one walks out through
    /// Nella leaves two, and those two carry on where they stand.
    #[test]
    fn a_run_left_with_one_member_ends_and_sends_them_out() {
        use crate::firsttime;
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let stage = firsttime::STAGE_2;
        let mut cfg = (*config).clone();
        for m in [stage, firsttime::EXIT_MAP] {
            cfg.fields.insert(m);
        }
        cfg.npcs.insert(
            stage,
            vec![net::opcode::FieldNpc { object_id: 930, template_id: firsttime::NELLA, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 }],
        );
        let config = Arc::new(cfg);
        let mut ids = Vec::new();
        let mut sessions = Vec::new();
        for name in ["Leader", "Mote", "Tria", "Quad", "Quin"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: stage, level: 21, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, stage).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            ids.push(id);
            sessions.push(s);
        }
        let map_of = |id: u32| store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().map_id;
        let pair = fields.runs().open(5_801, vec![ids[0], ids[1]], store::Store::unix_now());
        let trio = fields.runs().open(5_802, vec![ids[2], ids[3], ids[4]], store::Store::unix_now());
        for s in sessions.iter_mut() {
            let _ = s.on_field_entered();
        }
        for s in sessions.iter_mut() {
            let _ = s.tick(1_000);
        }

        // The pair: Mote disconnects.
        let mote = sessions.remove(1);
        drop(mote);
        let gone = fields.runs().instance(pair.id);
        assert_eq!(gone, None, "a run of one is ended");
        let out = sessions[0].tick(2_000);
        assert!(out.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "the Leader is sent out");
        let told = out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE && String::from_utf8_lossy(&r.body).contains("too small to continue"));
        assert!(told, "and told why");
        assert_eq!(map_of(ids[0]), firsttime::EXIT_MAP, "to the Exit");

        // The control: Tria leaves the trio through Nella; two remain and stay put.
        let mut click = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
        click.extend_from_slice(&930u32.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&u32::MAX.to_le_bytes());
        let mut yes = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
        yes.extend_from_slice(&0u32.to_le_bytes());
        yes.push(0);
        yes.extend_from_slice(&0u32.to_le_bytes());
        yes.extend_from_slice(&0u16.to_le_bytes());
        yes.push(net::script::SCRIPT_ACTION_YES as u8);
        let _ = sessions[1].handle(&click);
        let _ = sessions[1].handle(&yes);
        assert_eq!(map_of(ids[2]), firsttime::EXIT_MAP, "Tria is out");
        let still = fields.runs().instance(trio.id).map(|i| i.members);
        assert_eq!(still, Some(vec![ids[3], ids[4]]), "two remain in the run");
        for i in [2, 3] {
            let out = sessions[i].tick(3_000);
            assert!(!out.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "and nobody else is moved");
        }
        assert_eq!(map_of(ids[3]), stage);
        let closed = fields.runs().close(trio.id);
        assert!(closed);
    }

    /// **Cloto will not clear a stage until the whole run is standing on it.** The owner,
    /// 2026-09-23: *"Do not clear a stage unless everyone is on same map that the stage is
    /// about to be cleared of."*
    ///
    /// A1 and A2 in one run; A2 is on stage 2. A1's click is refused by name and clears
    /// nothing - no effect to A1, none on the bus, the run still uncleared. The control:
    /// once A2 walks onto stage 1 the same click clears it. And a member who DISCONNECTS
    /// stops holding the stage up, because they leave the run.
    #[test]
    fn cloto_waits_for_the_whole_run_to_be_on_her_stage() {
        use crate::firsttime;
        // The last stage: the leader clears it with ten Passes, which is the simplest real
        // clear there is, so the rule under test is the only thing that can refuse it.
        let stage = firsttime::STAGE_5;
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut cfg = (*config).clone();
        cfg.fields.insert(stage);
        cfg.fields.insert(stage + 100);
        cfg.npcs.insert(
            stage,
            vec![net::opcode::FieldNpc { object_id: 920, template_id: firsttime::CLOTO, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 }],
        );
        cfg.item_names.insert(firsttime::PASS, "Pass".into());
        let config = Arc::new(cfg);
        let mut ids = Vec::new();
        let mut sessions = Vec::new();
        for (name, map) in [("Alfa", stage), ("Alto", stage + 100), ("Arco", stage)] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: map, level: 21, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, map).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            ids.push(id);
            sessions.push(s);
        }
        // Alfa leads all three; Arco is in it for the disconnect half.
        let created = sessions[0].run_party_request(ids[0], crate::party::Request::Create { name: "P".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        for i in 1..3 {
            let _ = sessions[0].run_party_request(ids[0], crate::party::Request::Invite { target: ids[i] });
            let _ = sessions[i].tick(500);
            let _ = sessions[i].run_party_request(ids[i], crate::party::Request::Accept { party });
        }
        let _ = sessions[0].give_item(firsttime::PASS, 10, "test").unwrap();
        let run = fields.runs().open(5_201, ids.clone(), store::Store::unix_now());
        for s in sessions.iter_mut() {
            let _ = s.on_field_entered();
        }
        for s in sessions.iter_mut() {
            let _ = s.tick(1_000);
        }
        let effects = |out: &[Reply]| out.iter().filter(|r| r.opcode == net::fieldeffect::FIELD_EFFECT).count();
        let said = |out: &[Reply]| {
            out.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).map(|r| String::from_utf8_lossy(&r.body).to_string()).collect::<String>()
        };
        let mut click = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
        click.extend_from_slice(&920u32.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&u32::MAX.to_le_bytes());

        // Alto is on stage 2: refused, by name, and nothing cleared or sent.
        let out = sessions[0].handle(&click);
        assert_eq!(effects(&out), 0, "no banner, no gate");
        assert!(said(&out).contains("Alto"), "the missing member is named: {}", said(&out));
        assert!(!said(&out).contains("Arco"), "a member who IS here is not: {}", said(&out));
        assert_eq!(effects(&sessions[2].tick(2_000)), 0, "and nothing reached the bus");
        let cleared = fields.runs().is_cleared(ids[0], stage);
        assert!(!cleared, "the stage is still shut");
        let passes: u32 = store.bag_items(ids[0], store::InventoryType::Etc).unwrap().iter().filter(|r| r.item.item_id == firsttime::PASS).map(|r| u32::from(r.item.kind.quantity())).sum();
        assert_eq!(passes, 10, "a refusal takes no Pass");

        // Alto DISCONNECTS on stage 2. They leave the run, so they no longer hold it up.
        let alto = sessions.remove(1);
        drop(alto);
        let still = fields.runs().instance_of(ids[1]);
        assert_eq!(still, None, "a dropped connection leaves the run");

        // The control: with everyone left in the run on the stage, the same click clears.
        let out = sessions[0].handle(&click);
        assert_eq!(effects(&out), 3, "banner, fanfare, gate: {}", said(&out));
        let cleared = fields.runs().is_cleared(ids[0], stage);
        assert!(cleared);
        let closed = fields.runs().close(run.id);
        assert!(closed);
    }

    /// **A disconnect never logs back in onto a stage.** The owner, 2026-09-23: *"they should
    /// log-in onto the Exit map to be returned to Kerning City. Disconnected players should
    /// never be logged back into to any of the PQ stages."*
    ///
    /// Every stage, 1 to Bonus, is sent to the Exit - checked through the real login path
    /// (a claim, then the hello's `SetField`) and on the saved record. The Exit itself logs
    /// in where it is, and so does an ordinary map: the control that the redirect is not
    /// simply "every login goes to the Exit".
    #[test]
    fn a_login_onto_any_party_quest_stage_lands_on_the_exit() {
        use crate::firsttime;
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut cfg = (*config).clone();
        for m in (0..=6).map(|n| firsttime::STAGE_1 + n * 100) {
            cfg.fields.insert(m);
        }
        cfg.fields.insert(firsttime::TOWN_MAP);
        let config = Arc::new(cfg);
        let stages: Vec<u32> = (0..=5).map(|n| firsttime::STAGE_1 + n * 100).collect();
        let cases = stages
            .iter()
            .map(|&m| (m, firsttime::EXIT_MAP))
            .chain([(firsttime::EXIT_MAP, firsttime::EXIT_MAP), (firsttime::TOWN_MAP, firsttime::TOWN_MAP)]);
        for (n, (saved, expect)) in cases.enumerate() {
            let name = format!("Login{n}");
            let chr = net::opcode::Character { name: name.clone(), map_id: saved, level: 21, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, saved).unwrap();
            // In a run too, as a disconnect mid-quest would leave them after a crash of the
            // SESSION but not of the server: the login must take them out of it.
            let run = fields.runs().open(6_000 + n as u32, vec![id], store::Store::unix_now());
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            let out = s.handle(&crate::session::CLIENT_MIGRATION_HELLO.to_le_bytes());
            let set = out.iter().find(|r| r.opcode == net::opcode::SET_FIELD).expect("the login SetField");
            assert!(set.what.contains(&format!("carrying map {expect} ")), "saved on {saved}: {}", set.what);
            let now = store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().map_id;
            assert_eq!(now, expect, "saved on {saved}, and the record says so too");
            if saved != expect {
                let inside = fields.runs().instance_of(id);
                assert_eq!(inside, None, "and out of the run");
            }
            let _ = fields.runs().close(run.id);
        }
    }

    /// **A stage clear belongs to one run.** The owner, 2026-09-23: *"just because one party
    /// instance cleared, doesn't mean that all party instances cleared. The PQ stage clears
    /// should be per instance, and never shared."*
    ///
    /// Three characters on stage 1: A1 and A2 in one run, B1 in another. A1 clicks Cloto.
    /// Claims, each with its control: A1 gets the three effects (banner, fanfare, gate); A2 -
    /// same run, same map - hears all three; B1 hears none. B1's `next00` stays shut and says
    /// so; A1's opens onto stage 2. A2 re-entering gets the gate again (it starts closed on
    /// every entry); B1 re-entering does not.
    #[test]
    fn a_stage_clear_opens_one_runs_portal_and_no_other_runs() {
        use crate::firsttime;
        // The last stage: the leader clears it with ten Passes, which is the simplest real
        // clear there is, so the rule under test is the only thing that can refuse it.
        let stage = firsttime::STAGE_5;
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut cfg = (*config).clone();
        cfg.fields.insert(stage);
        cfg.fields.insert(stage + 100);
        cfg.npcs.insert(
            stage,
            vec![net::opcode::FieldNpc { object_id: 920, template_id: firsttime::CLOTO, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 }],
        );
        cfg.item_names.insert(firsttime::PASS, "Pass".into());
        let config = Arc::new(cfg);

        let mut ids = Vec::new();
        let mut sessions = Vec::new();
        for name in ["Alfa", "Alto", "Bravo"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: stage, level: 21, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, stage).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            ids.push(id);
            sessions.push(s);
        }
        let created = sessions[0].run_party_request(ids[0], crate::party::Request::Create { name: "A".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        let _ = sessions[0].run_party_request(ids[0], crate::party::Request::Invite { target: ids[1] });
        let _ = sessions[1].tick(500);
        let _ = sessions[1].run_party_request(ids[1], crate::party::Request::Accept { party });
        let _ = sessions[0].give_item(firsttime::PASS, 10, "test").unwrap();
        let run_a = fields.runs().open(5_101, vec![ids[0], ids[1]], store::Store::unix_now());
        let run_b = fields.runs().open(5_102, vec![ids[2]], store::Store::unix_now());
        for s in sessions.iter_mut() {
            let entry = s.on_field_entered();
            assert!(
                !entry.iter().any(|r| r.opcode == net::fieldeffect::FIELD_EFFECT),
                "nothing is cleared yet, so no gate on entry"
            );
        }
        for s in sessions.iter_mut() {
            let _ = s.tick(1_000);
        }

        let effects = |out: &[Reply]| -> Vec<u8> {
            out.iter().filter(|r| r.opcode == net::fieldeffect::FIELD_EFFECT).map(|r| r.body[0]).collect()
        };
        let all_three = vec![
            net::fieldeffect::TYPE_SCREEN,
            net::fieldeffect::TYPE_SOUND,
            net::fieldeffect::TYPE_OBJECT_STATE,
        ];

        // A1 clicks Cloto.
        let mut click = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
        click.extend_from_slice(&920u32.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&0i16.to_le_bytes());
        click.extend_from_slice(&u32::MAX.to_le_bytes());
        let out = sessions[0].handle(&click);
        assert_eq!(effects(&out), all_three, "A1 sees the banner, hears the fanfare, and the gate opens");
        assert!(out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE), "and Cloto answers the click");
        assert_eq!(effects(&sessions[1].tick(2_000)), all_three, "A2 - same run - gets all three");
        assert_eq!(effects(&sessions[2].tick(2_000)), Vec::<u8>::new(), "B1 - other run, same map - gets nothing");
        let a1 = fields.runs().is_cleared(ids[0], stage);
        let a2 = fields.runs().is_cleared(ids[1], stage);
        assert!(a1 && a2);
        assert!(!fields.runs().is_cleared(ids[2], stage), "B's stage 1 is still shut");

        // A second click is not a second clear.
        let again = sessions[0].handle(&click);
        assert!(effects(&again).is_empty(), "already cleared: no second banner and no second gate");

        // The portal.
        let press = |name: &str| {
            let mut b = net::portalscript::CLIENT_PORTAL_SCRIPT.to_le_bytes().to_vec();
            b.push(0);
            b.extend_from_slice(&(name.len() as u16).to_le_bytes());
            b.extend_from_slice(name.as_bytes());
            b.extend_from_slice(&714i16.to_le_bytes());
            b.extend_from_slice(&106i16.to_le_bytes());
            b
        };
        let map_of = |id: u32| store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().map_id;
        let b_out = sessions[2].handle(&press(firsttime::NEXT_PORTAL));
        assert!(!b_out.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "B's portal is shut");
        assert!(!b_out.is_empty(), "and the press is still answered");
        assert_eq!(map_of(ids[2]), stage);

        // A2 re-enters stage 1 (the gate starts closed on every entry) and is shown it open;
        // B1 re-entering is not.
        assert_eq!(effects(&sessions[1].on_field_entered()), vec![net::fieldeffect::TYPE_OBJECT_STATE]);
        assert!(effects(&sessions[2].on_field_entered()).is_empty());

        let a_out = sessions[0].handle(&press(firsttime::NEXT_PORTAL));
        assert!(a_out.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "A's portal is open");
        assert_eq!(map_of(ids[0]), stage + 100, "onto stage 2");

        let closed_a = fields.runs().close(run_a.id);
        let closed_b = fields.runs().close(run_b.id);
        assert!(closed_a && closed_b);
    }

    /// **A party buff reaches every member on the caster's field, and nobody else.**
    ///
    /// The owner, 2026-09-06: *"party buffs should apply to everyone in the party who is in the
    /// same map."* Haste (4101001) is the specimen: `indieSpeed 10`, `indieJump 1`, 100 s at
    /// level 1, and the `lt`/`rb` rectangle that marks a party buff in the table. Four
    /// sessions on one channel: the caster, a member on the same field, a member on another
    /// field, and a stranger on the caster's field who is in no party. Claims: the caster's
    /// own `0x007D` carries bits 92 and 93; the same-field member's next tick delivers one
    /// too, value 10 with the skill as its reason; the other two get nothing; only the caster
    /// paid MP; and the **recipient owns the expiry** - its own tick past 100 s is what sends
    /// the `0x007E`, because only the session that holds that client can.
    #[test]
    fn a_party_buff_reaches_every_member_on_the_field_and_nobody_else() {
        let skills = std::path::Path::new("../../gm-handbook/skills.txt");
        if !skills.exists() {
            return; // generated, gitignored - python tools/dump_skills.py
        }
        let store = Arc::new(Store::open_in_memory().unwrap());
        let config = Arc::new(Config {
            firstjob: crate::firstjob::CombatTable::load(skills),
            ..Config::default()
        });
        let fields = Arc::new(Fields::new());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let make = |name: &str, map: u32| {
            let chr = net::opcode::Character { name: name.to_string(), map_id: map, ..Default::default() };
            let mut made = store.create_character(account, 0, &chr).unwrap();
            made.mp = 200;
            made.max_mp = 200;
            store.save_character_progress(&made).unwrap();
            store.create_migration(account, made.id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(made.id);
            s.on_field_entered();
            (s, made.id)
        };
        let (mut caster, caster_id) = make("Cobalt", 104_040_000);
        let (mut near, near_id) = make("Tester2", 104_040_000);
        let (mut far, far_id) = make("Farside", 100_000_000);
        let (mut stranger, _) = make("Stranger", 104_040_000);
        store.set_skill_level(caster_id, 4_101_001, 1).unwrap();

        // Form the party: caster leads, both members accept.
        let created = caster.run_party_request(
            caster_id,
            crate::party::Request::Create { name: "Cobalt's Party".into() },
        );
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        for (member, id) in [(&mut near, near_id), (&mut far, far_id)] {
            let _ = caster.run_party_request(caster_id, crate::party::Request::Invite { target: id });
            let _ = member.tick(1_000);
            let _ = member.run_party_request(id, crate::party::Request::Accept { party });
        }
        // Drain the party traffic so the buff packets below stand alone.
        for s in [&mut caster, &mut near, &mut far, &mut stranger] {
            let _ = s.tick(2_000);
            let _ = s.tick(2_100);
        }

        // The cast: `0x013C`, u32 skillId, u32 level.
        let mut body = net::buff::CLIENT_SKILL_USE.to_le_bytes().to_vec();
        body.extend_from_slice(&4_101_001u32.to_le_bytes());
        body.extend_from_slice(&1u32.to_le_bytes());
        let _ = caster.tick(3_000);
        let out = caster.handle(&body);

        let is_set = |r: &Reply| r.opcode == net::buff::TEMPORARY_STAT_SET;
        let own = out.iter().find(|r| is_set(r)).unwrap_or_else(|| panic!("the caster's own grant: {out:?}"));
        assert_eq!(net::buff::bits_in_mask(&own.body[..net::buff::MASK_LEN]), vec![92, 93], "Speed and Jump");

        let got = near.tick(3_100);
        let theirs: Vec<_> = got.iter().filter(|r| is_set(r)).collect();
        assert_eq!(theirs.len(), 1, "one grant for the member on the same field: {got:?}");
        let b = &theirs[0].body;
        assert_eq!(net::buff::bits_in_mask(&b[..net::buff::MASK_LEN]), vec![92, 93]);
        // Entries follow the mask in ascending bit order: bit 92 first - i16 value, u32 reason.
        let m = net::buff::MASK_LEN;
        assert_eq!(&b[m..m + 2], &10i16.to_le_bytes(), "Speed +10 at level 1");
        assert_eq!(&b[m + 2..m + 6], &4_101_001u32.to_le_bytes(), "the reason is the skill");

        assert!(!far.tick(3_100).iter().any(is_set), "a member on another field gets nothing");
        assert!(!stranger.tick(3_100).iter().any(is_set), "a stranger on the same field gets nothing");

        // Only the caster paid.
        let mp = |id: u32| {
            store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().mp
        };
        assert_eq!(mp(caster_id), 185, "Haste level 1 costs 15");
        assert_eq!(mp(near_id), 200, "the recipient paid nothing");

        // The recipient owns the expiry: nothing at 99 s, the reset from ITS tick at 101 s.
        assert!(!near.tick(3_100 + 99_000).iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET));
        let later = near.tick(3_100 + 101_000);
        assert!(
            later.iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET),
            "the member's own session sends the 0x007E: {later:?}"
        );
    }

    /// **Party EXP is a COPY per member on the field, at the `!setrates` party share.**
    ///
    /// The owner, 2026-09-06: *"you kill a shitty slime that gets you 100 EXP ... 30% split copy
    /// for party member means killer (70% - 70 EXP), party mem 2-6 (30% each, 30 EXP each),
    /// this mob awarded a total of 220 EXP; 50% ... 50 EXP each."* Three in the party on one
    /// field plus a stranger. Level 50 characters, so 70 EXP moves the number and not the
    /// level. Claims: killer +70 and white; each member +30 on their own tick, yellow; the
    /// stranger nothing; `!setrates 1 1 1 1 50` makes the next kill +50 each with the killer
    /// still +70; and `0` shares nothing at all.
    #[test]
    fn party_exp_is_a_copy_per_member_at_the_configured_share() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let config = Arc::new(Config::default());
        let fields = Arc::new(Fields::new());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        store.set_gm("maplecw", true).unwrap(); // after the account exists, or NoSuchAccount
        let make = |name: &str| {
            let chr = net::opcode::Character { name: name.to_string(), map_id: 104_040_000, ..Default::default() };
            let mut made = store.create_character(account, 0, &chr).unwrap();
            made.level = 50;
            made.exp = 0;
            store.save_character_progress(&made).unwrap();
            store.create_migration(account, made.id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(made.id);
            s.on_field_entered();
            (s, made.id)
        };
        let (mut killer, killer_id) = make("Cobalt");
        let (mut a, a_id) = make("Tester2");
        let (mut b, b_id) = make("Robin");
        let (mut stranger, stranger_id) = make("Stranger");
        let created = killer.run_party_request(killer_id, crate::party::Request::Create { name: "P".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        for (m, id) in [(&mut a, a_id), (&mut b, b_id)] {
            let _ = killer.run_party_request(killer_id, crate::party::Request::Invite { target: id });
            let _ = m.tick(1_000);
            let _ = m.run_party_request(id, crate::party::Request::Accept { party });
        }
        for s in [&mut killer, &mut a, &mut b, &mut stranger] {
            let _ = s.tick(2_000);
        }
        let exp = |id: u32| store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().exp;

        // Default share, 30%: 70 to the killer, 30 to each of the two members, total 130.
        let out = killer.party_exp_split(100, killer_id).expect("a party of three splits");
        assert!(out.iter().any(|r| r.what.contains("70")), "the killer's own 70: {out:?}");
        assert_eq!(exp(killer_id), 70, "70% to the killer");
        let _ = a.tick(3_000);
        let _ = b.tick(3_000);
        let _ = stranger.tick(3_000);
        assert_eq!((exp(a_id), exp(b_id)), (30, 30), "30 EACH, not 15 each - a copy, not a split");
        assert_eq!(exp(stranger_id), 0, "not in the party");

        // 50%: the killer still keeps 70, each member now gets 50.
        let said = killer.handle(&gm_chat_body("!setrates 1 1 1 1 50"));
        assert!(said.iter().any(|r| r.what.contains("Party EXP 50%")), "{said:?}");
        let _ = killer.party_exp_split(100, killer_id).unwrap();
        let _ = a.tick(4_000);
        let _ = b.tick(4_000);
        assert_eq!(exp(killer_id), 140);
        assert_eq!((exp(a_id), exp(b_id)), (80, 80), "+50 each at 50%");

        // 0%: nobody but the killer.
        let _ = killer.handle(&gm_chat_body("!setrates 1 1 1 1 0"));
        let _ = killer.party_exp_split(100, killer_id).unwrap();
        let _ = a.tick(5_000);
        assert_eq!(exp(killer_id), 210);
        assert_eq!(exp(a_id), 80, "a 0% share pays nothing - and is not floored to 1");
    }

    /// **A mob's mesos split like its EXP: 70% to the picker, the party share to every other
    /// member on the map, each, in yellow - and a player's dropped mesos do not.** The owner,
    /// 2026-09-14. Three effects per share and the test says something about all three: the
    /// purse row, the `0x007C` balance, and the message line (its kind, and that it is the
    /// `smallChange` field, which is the client's yellow "Spotting Small Change" line, with
    /// no white line beside it).
    #[test]
    fn party_mesos_from_a_mob_split_seventy_thirty_and_a_players_drop_does_not() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let config = Arc::new(Config::default());
        let fields = Arc::new(Fields::new());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        store.set_gm("maplecw", true).unwrap();
        let map = crate::fields::FieldKey::world(104_040_000);
        let make = |name: &str| {
            let chr = net::opcode::Character { name: name.to_string(), map_id: map.map, ..Default::default() };
            let made = store.create_character(account, 0, &chr).unwrap();
            store.create_migration(account, made.id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(made.id);
            s.on_field_entered();
            (s, made.id)
        };
        let (mut picker, picker_id) = make("Cobalt");
        let (mut a, a_id) = make("Tester2");
        let (mut b, b_id) = make("Robin");
        let (mut stranger, stranger_id) = make("Stranger");
        let created = picker.run_party_request(picker_id, crate::party::Request::Create { name: "P".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        for (m, id) in [(&mut a, a_id), (&mut b, b_id)] {
            let _ = picker.run_party_request(picker_id, crate::party::Request::Invite { target: id });
            let _ = m.tick(1_000);
            let _ = m.run_party_request(id, crate::party::Request::Accept { party });
        }
        for s in [&mut picker, &mut a, &mut b, &mut stranger] {
            let _ = s.tick(2_000);
        }
        let purse = |id: u32| store.mesos(id).unwrap();
        let meso_drop = |from_mob: bool, owner: u32, party_id: u32| {
            fields.with_drops(map, |d| {
                d.drop_from_mob(crate::drops::DropFromMob {
                    from_mob,
                    map_id: map,
                    owner_id: owner,
                    item: store::Item::bundle(0, 0),
                    inv_type: store::InventoryType::Etc,
                    meso: 1_000,
                    x: 400,
                    y: 395,
                    source_x: 400,
                    source_y: 395,
                    now_ms: 0,
                    party_id,
                })
            }).0
        };
        let pick_up = |id: u32| {
            let mut b = 0x032Cu16.to_le_bytes().to_vec(); // the measured pick-up opcode
            b.extend_from_slice(&[0u8; 13]);
            b.extend_from_slice(&id.to_le_bytes());
            b.extend_from_slice(&[0u8; 4]);
            b
        };

        // A mob's 1000 mesos, picked up by a party member with two others on the map.
        let drop_id = meso_drop(true, picker_id, party);
        let out = picker.handle(&pick_up(drop_id));
        let stat = out.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).unwrap_or_else(|| panic!("the picker's balance: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>()));
        assert!(stat.what.contains("+700 mesos") && stat.what.contains("70% of 1000; 2 party member(s) mailed 300 each"), "{}", stat.what);
        let line = out.iter().find(|r| r.opcode == net::message::MESSAGE).expect("the picker's line");
        assert_eq!(&line.body[4..8], &700i32.to_le_bytes(), "the picker's white line says +700");
        assert_eq!(&line.body[8..10], &[0, 0], "and carries no small change of its own");
        assert_eq!(purse(picker_id), 700, "70% to the picker");
        for (m, id, name) in [(&mut a, a_id, "a"), (&mut b, b_id, "b")] {
            let mail = m.tick(3_000);
            let stat = mail.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).unwrap_or_else(|| panic!("{name}: the balance"));
            assert!(stat.what.contains("+300 mesos -> 300, a party share"), "{name}: {}", stat.what);
            let line = mail.iter().find(|r| r.opcode == net::message::MESSAGE).unwrap_or_else(|| panic!("{name}: the yellow line"));
            assert_eq!(line.body[0], net::message::kind::DROP_PICKUP);
            assert_eq!(&line.body[4..8], &0i32.to_le_bytes(), "{name}: gain 0 - no white line");
            assert_eq!(&line.body[8..10], &300u16.to_le_bytes(), "{name}: smallChange 300 - 'Spotting Small Change (+300)', yellow");
            assert!(line.what.contains("YELLOW"), "{name}: {}", line.what);
            assert_eq!(purse(id), 300, "{name}: a COPY of the 30%, not a split");
        }
        let _ = stranger.tick(3_000);
        assert_eq!(purse(stranger_id), 0, "not in the party");

        // Mesos a PLAYER dropped: 100% to whoever picks them up, party or not.
        let drop_id = meso_drop(false, picker_id, 0);
        let out = picker.handle(&pick_up(drop_id));
        let stat = out.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).unwrap();
        assert!(stat.what.contains("+1000 mesos") && !stat.what.contains("party"), "{}", stat.what);
        assert_eq!(purse(picker_id), 1_700);
        let _ = a.tick(4_000);
        assert_eq!(purse(a_id), 300, "nothing more for the members");

        // A mob's mesos picked up by someone in NO party: all of it.
        let drop_id = meso_drop(true, stranger_id, 0);
        let _ = stranger.handle(&pick_up(drop_id));
        assert_eq!(purse(stranger_id), 1_000);

        // The party rate is the EXP rate: at 50% each member gets 500 and the picker still 700.
        let _ = picker.handle(&gm_chat_body("!setrates 1 1 1 1 50"));
        let drop_id = meso_drop(true, picker_id, party);
        let _ = picker.handle(&pick_up(drop_id));
        let _ = a.tick(5_000);
        assert_eq!(purse(picker_id), 2_400);
        assert_eq!(purse(a_id), 800, "+500 at 50%");
    }

    /// **Party chat reaches every other member on the channel, on any map, as the client's
    /// own 0x01B1 - and not the sender.** The owner, 2026-09-14: *"my party member does not
    /// receive the message."* The stranger on the same map hears nothing; the member on
    /// another map does.
    #[test]
    fn a_party_line_reaches_the_other_members_anywhere_on_the_channel_and_nobody_else() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let config = Arc::new(Config::default());
        let fields = Arc::new(Fields::new());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        store.set_gm("maplecw", true).unwrap();
        let make = |name: &str, map: u32| {
            let chr = net::opcode::Character { name: name.to_string(), map_id: map, ..Default::default() };
            let made = store.create_character(account, 0, &chr).unwrap();
            store.create_migration(account, made.id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(made.id);
            s.on_field_entered();
            (s, made.id)
        };
        let (mut leader, leader_id) = make("Cobalt", 104_040_000);
        let (mut same_map, same_id) = make("Tester2", 104_040_000);
        let (mut far, far_id) = make("Robin", 100_000_000);
        let (mut stranger, _) = make("Stranger", 104_040_000);
        let created = leader.run_party_request(leader_id, crate::party::Request::Create { name: "P".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        for (m, id) in [(&mut same_map, same_id), (&mut far, far_id)] {
            let _ = leader.run_party_request(leader_id, crate::party::Request::Invite { target: id });
            let _ = m.tick(1_000);
            let _ = m.run_party_request(id, crate::party::Request::Accept { party });
        }
        for s in [&mut leader, &mut same_map, &mut far, &mut stranger] {
            let _ = s.tick(2_000);
        }

        // The owner's captured request, with the client's own idea of the recipient list.
        let mut req = net::groupmessage::CLIENT_GROUP_MESSAGE.to_le_bytes().to_vec();
        req.extend_from_slice(&[1, 1, 0]); // kind 1, u16 count 1 - the log's own bytes
        req.extend_from_slice(&same_id.to_le_bytes());
        req.extend_from_slice(&5u16.to_le_bytes());
        req.extend_from_slice(b"Hello");
        let own = leader.handle(&req);
        assert!(own.iter().all(|r| r.opcode != net::groupmessage::GROUP_MESSAGE), "the sender's client draws its own line: {own:?}");

        for (m, name) in [(&mut same_map, "same map"), (&mut far, "another map")] {
            let mail = m.tick(3_000);
            let line = mail.iter().find(|r| r.opcode == net::groupmessage::GROUP_MESSAGE).unwrap_or_else(|| panic!("{name}: the line"));
            assert_eq!(line.body, net::groupmessage::group_message(1, u32::try_from(account).unwrap(), leader_id, 0, "Cobalt", "Hello"));
            assert!(line.what.contains("Cobalt says 'Hello'"), "{}", line.what);
        }
        let mail = stranger.tick(3_000);
        assert!(mail.iter().all(|r| r.opcode != net::groupmessage::GROUP_MESSAGE), "not in the party: {mail:?}");
    }

    /// **A whisper reaches the named character on any map and the sender is told it did;
    /// an unknown name is told "Could not find".** The owner, 2026-09-15, "Hello Whisper" to
    /// Tester2 - the request's bytes are the log's, the replies are the handler's arms.
    #[test]
    fn a_whisper_reaches_its_target_by_name_and_the_sender_is_answered_either_way() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let config = Arc::new(Config::default());
        let fields = Arc::new(Fields::new());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let make = |name: &str, map: u32| {
            let chr = net::opcode::Character { name: name.to_string(), map_id: map, ..Default::default() };
            let made = store.create_character(account, 0, &chr).unwrap();
            store.create_migration(account, made.id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(made.id);
            s.on_field_entered();
            (s, made.id)
        };
        let (mut wisp, wisp_id) = make("Wisp", 104_040_000);
        let (mut tester, tester_id) = make("Tester2", 100_000_000);
        let hex = |s: &str| -> Vec<u8> { (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect() };
        let mut req = net::whisper::CLIENT_WHISPER.to_le_bytes().to_vec();
        req.extend_from_slice(&hex("069465a11a0700546573746572320d0048656c6c6f2057686973706572"));

        let out = wisp.handle(&req);
        let sent = out.iter().find(|r| r.opcode == net::whisper::WHISPER).expect("the sender's 0x0A");
        assert_eq!(sent.body, net::whisper::whisper_sent("Tester2", true), "{}", sent.what);
        let mail = tester.tick(1_000);
        let got = mail.iter().find(|r| r.opcode == net::whisper::WHISPER).expect("the target's 0x12");
        assert_eq!(
            got.body,
            net::whisper::whisper_receive("Wisp", wisp_id, u32::try_from(account).unwrap(), 0, 0, "Hello Whisper")
        );
        assert_eq!(got.body[0], net::whisper::mode::RECEIVE);
        let _ = tester_id;

        // Nobody by that name: the sender gets 0x0A with found = 0, nothing else goes anywhere.
        let mut req = net::whisper::CLIENT_WHISPER.to_le_bytes().to_vec();
        req.push(6);
        req.extend_from_slice(&[0u8; 4]);
        req.extend_from_slice(&6u16.to_le_bytes());
        req.extend_from_slice(b"Nobody");
        req.extend_from_slice(&2u16.to_le_bytes());
        req.extend_from_slice(b"hi");
        let out = wisp.handle(&req);
        let sent = out.iter().find(|r| r.opcode == net::whisper::WHISPER).expect("still answered");
        assert_eq!(sent.body, net::whisper::whisper_sent("Nobody", false));
        assert!(tester.tick(2_000).iter().all(|r| r.opcode != net::whisper::WHISPER));
    }

    /// **Opening a Maple Chat with an invitee opens the opener's window and puts the dialog
    /// on the invitee's client.** The owner, 2026-09-15 - the request is the log's bytes.
    #[test]
    fn opening_a_maple_chat_answers_the_opener_and_invites_the_named_character() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let config = Arc::new(Config::default());
        let fields = Arc::new(Fields::new());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let make = |name: &str| {
            let chr = net::opcode::Character { name: name.to_string(), map_id: 104_040_000, ..Default::default() };
            let made = store.create_character(account, 0, &chr).unwrap();
            store.create_migration(account, made.id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(made.id);
            s.on_field_entered();
            (s, made.id)
        };
        let (mut wisp, wisp_id) = make("Wisp");
        let (mut tester, tester_id) = make("Tester2");
        let hex = |s: &str| -> Vec<u8> { (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect() };
        let mut req = net::messenger::CLIENT_MESSENGER.to_le_bytes().to_vec();
        req.extend_from_slice(&hex("0000000001070054657374657232"));
        let out = wisp.handle(&req);
        let mine: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::messenger::MESSENGER).collect();
        assert_eq!(mine.len(), 2, "mode 0 then mode 4: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
        let id = u32::from_le_bytes(mine[0].body[..4].try_into().unwrap());
        assert_eq!(&mine[0].body[4..], &[0u8; 8], "mode 0, result 0");
        assert_ne!(id, 0);
        // The six seats, the owner in seat 0 with their own look - what draws their avatar.
        let wisp_chr = store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == wisp_id).unwrap();
        let mut seats: [Option<net::messenger::Seat>; net::messenger::SEATS] = Default::default();
        seats[0] = Some(net::messenger::Seat { character_id: wisp_id, name: "Wisp".into(), look: net::opcode::avatar_look(&wisp_chr) });
        assert_eq!(mine[1].body, net::messenger::members(id, &seats));
        let mail = tester.tick(1_000);
        let inv = mail.iter().find(|r| r.opcode == net::messenger::MESSENGER).expect("mode 6 to the invitee");
        assert_eq!(inv.body, net::messenger::invite(id, wisp_id, "Wisp"));

        // Accept (mode 7, the log's bytes): Tester2 takes seat 1, gets mode 0 and all six
        // seats; the owner gets the one-record mode 4 with Tester2's look.
        let mut acc = net::messenger::CLIENT_MESSENGER.to_le_bytes().to_vec();
        acc.extend_from_slice(&7u32.to_le_bytes());
        acc.extend_from_slice(&id.to_le_bytes());
        let out = tester.handle(&acc);
        let theirs: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::messenger::MESSENGER).collect();
        assert_eq!(theirs.len(), 2, "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
        assert_eq!(theirs[0].body, net::messenger::self_enter(id, 0));
        let tester_chr = store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == tester_id).unwrap();
        let t_seat = net::messenger::Seat { character_id: tester_id, name: "Tester2".into(), look: net::opcode::avatar_look(&tester_chr) };
        seats[1] = Some(t_seat.clone());
        assert_eq!(theirs[1].body, net::messenger::members(id, &seats), "both seats, in order");
        let mail = wisp.tick(2_000);
        let joined = mail.iter().find(|r| r.opcode == net::messenger::MESSENGER).expect("the newcomer to the owner");
        // **The whole table, not just the newcomer.** A one-record mode 4 is what the client
        // answered with 0x009E CLIENT_PACKET_REJECTED before dying; `net::messenger::members`
        // carries the capture.
        assert_eq!(joined.body, net::messenger::members(id, &seats), "all six seats to the seated member too");

        // **A typed line reaches everyone, the speaker included.** The owner, 2026-09-22: "any
        // chat messages happen does not show on screen". The client's own send builder does
        // not draw the line locally, so the echo is not a nicety.
        let mut say = net::messenger::CLIENT_MESSENGER.to_le_bytes().to_vec();
        say.extend_from_slice(&3u32.to_le_bytes());
        say.extend_from_slice(&5u16.to_le_bytes());
        say.extend_from_slice(b"hello");
        let own = tester.handle(&say);
        let line = net::messenger::chat(id, 1, "Tester2", "hello");
        assert_eq!(
            own.iter().filter(|r| r.opcode == net::messenger::MESSENGER).map(|r| r.body.clone()).collect::<Vec<_>>(),
            vec![line.clone()],
            "the speaker is echoed their own line"
        );
        let heard = wisp.tick(2_500);
        assert!(
            heard.iter().any(|r| r.opcode == net::messenger::MESSENGER && r.body == line),
            "and the other member is told: {:?}",
            heard.iter().map(|r| &r.what).collect::<Vec<_>>()
        );

        // **Leaving frees the seat and the remaining member is sent the table again.**
        // The owner, 2026-09-22: "when the owner exit the chat room, Tester2's chat room still shows
        // that the owner is still in the room."
        let mut bye = net::messenger::CLIENT_MESSENGER.to_le_bytes().to_vec();
        bye.extend_from_slice(&1u32.to_le_bytes());
        bye.extend_from_slice(&id.to_le_bytes());
        let nothing = tester.handle(&bye);
        assert!(nothing.is_empty(), "the leaver's own window is already shut: {nothing:?}");
        seats[1] = None;
        let told = wisp.tick(3_000);
        let table = told
            .iter()
            .find(|r| r.opcode == net::messenger::MESSENGER)
            .unwrap_or_else(|| panic!("the remaining member is told: {:?}", told.iter().map(|r| &r.what).collect::<Vec<_>>()));
        assert_eq!(table.body, net::messenger::members(id, &seats), "six seats with the leaver's zeroed");

        // An accept for a room this channel does not hold: result 1, nothing opens.
        let mut acc = net::messenger::CLIENT_MESSENGER.to_le_bytes().to_vec();
        acc.extend_from_slice(&7u32.to_le_bytes());
        acc.extend_from_slice(&0x7_0009u32.to_le_bytes());
        let out = tester.handle(&acc);
        assert_eq!(out.iter().find(|r| r.opcode == net::messenger::MESSENGER).unwrap().body, net::messenger::self_enter(0x7_0009, 1));
    }

    /// **The pick-up-rights button is a toggle, answered with the client's 0x2D.** The owner,
    /// 2026-09-14: *"nothing happened"* - the reply was a bare 0x0D whose block carried a
    /// constant 0 in the rights byte. Each press flips All <-> Party Leader, every member
    /// gets `str name, u8 isPublic, u8 rights`, and the block's tail byte follows.
    #[test]
    fn the_pickup_rights_button_toggles_and_every_member_gets_the_status_packet() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let config = Arc::new(Config::default());
        let fields = Arc::new(Fields::new());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        store.set_gm("maplecw", true).unwrap();
        let make = |name: &str| {
            let chr = net::opcode::Character { name: name.to_string(), map_id: 104_040_000, ..Default::default() };
            let made = store.create_character(account, 0, &chr).unwrap();
            store.create_migration(account, made.id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(made.id);
            s.on_field_entered();
            (s, made.id)
        };
        let (mut leader, leader_id) = make("Cobalt");
        let (mut member, member_id) = make("Tester2");
        let created = leader.run_party_request(leader_id, crate::party::Request::Create { name: "Wisp's Party".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        let _ = leader.run_party_request(leader_id, crate::party::Request::Invite { target: member_id });
        let _ = member.tick(1_000);
        let _ = member.run_party_request(member_id, crate::party::Request::Accept { party });
        let _ = leader.tick(2_000);
        assert_eq!(fields.parties().party(party).unwrap().pickup_rights, crate::party::PICKUP_ALL, "a new party is All");

        // The request the button sends carries a constant 1; the value is ignored.
        let out = leader.run_party_request(leader_id, crate::party::Request::SetPickupRights { rights: 1 });
        let status = out.iter().find(|r| r.opcode == net::party::PARTY_RESULT && r.body[0] == net::party::result::PUBLIC_PRIVATE).expect("0x2D to the leader");
        assert_eq!(status.body, net::party::party_status("Wisp's Party", false, true), "name, isPublic 0, rights 1 = Party Leader");
        assert!(status.what.contains("Party Leader"), "{}", status.what);
        let mail = member.tick(3_000);
        let theirs = mail.iter().find(|r| r.opcode == net::party::PARTY_RESULT && r.body[0] == net::party::result::PUBLIC_PRIVATE).expect("0x2D to the member");
        assert_eq!(theirs.body, status.body, "the same bytes to every member");
        assert_eq!(fields.parties().party(party).unwrap().pickup_rights, crate::party::PICKUP_LEADER_ONLY);

        // Press again: back to All, and any later block carries 0 in that byte.
        let out = leader.run_party_request(leader_id, crate::party::Request::SetPickupRights { rights: 1 });
        let status = out.iter().find(|r| r.body.first() == Some(&net::party::result::PUBLIC_PRIVATE)).unwrap();
        assert_eq!(*status.body.last().unwrap(), 0, "rights byte 0 = All");
        assert_eq!(fields.parties().party(party).unwrap().pickup_rights, crate::party::PICKUP_ALL);

        // A member's press is refused as not-the-leader, and nothing flips.
        let out = member.run_party_request(member_id, crate::party::Request::SetPickupRights { rights: 1 });
        assert!(out.iter().all(|r| r.body.first() != Some(&net::party::result::PUBLIC_PRIVATE)), "{out:?}");
        assert_eq!(fields.parties().party(party).unwrap().pickup_rights, crate::party::PICKUP_ALL);
    }

    /// **A leader who leaves the game hands the crown to the highest-level member still online
    /// and keeps their seat; the last member online to leave - leader or not - disbands the
    /// party; a channel change hands nothing over.** The owner, 2026-09-18, all three messages,
    /// the last being *"If everyone is offline, the party shouldn't exist?"*. Three members,
    /// levels 10 / 15 / 20 in join order.
    #[test]
    fn a_leader_who_leaves_the_game_hands_the_crown_to_the_highest_level_member_online() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        // Two channels, so a Change Channel can be minted rather than refused.
        let config = Arc::new(Config {
            channels: vec!["127.0.0.1:8485".parse().unwrap(), "127.0.0.1:8486".parse().unwrap()],
            ..Config::default()
        });
        let fields = Arc::new(Fields::new());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let join = |id: u32| {
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            let entry = s.on_field_entered();
            let windows = entry.iter().filter(|r| r.opcode == net::party::PARTY_RESULT && r.body[0] == net::party::result::PARTY_STATE).count();
            (s, windows)
        };
        let make = |name: &str, level: u32| {
            let chr = net::opcode::Character { name: name.to_string(), map_id: 104_040_000, ..Default::default() };
            let mut made = store.create_character(account, 0, &chr).unwrap();
            made.level = level;
            store.save_character_progress(&made).unwrap();
            made.id
        };
        let (leader_id, mid_id, high_id) = (make("Cobalt", 10), make("Tester2", 15), make("Tester3", 20));
        let ((mut leader, w0), (mut mid, w1), (mut high, w2)) = (join(leader_id), join(mid_id), join(high_id));
        assert_eq!((w0, w1, w2), (0, 0, 0), "no party yet, no window on login");
        let created = leader.run_party_request(leader_id, crate::party::Request::Create { name: "Wisp's Party".into() });
        let party = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        for (s, id) in [(&mut mid, mid_id), (&mut high, high_id)] {
            let _ = leader.run_party_request(leader_id, crate::party::Request::Invite { target: id });
            let _ = s.tick(1_000);
            let _ = s.run_party_request(id, crate::party::Request::Accept { party });
            let _ = leader.tick(2_000);
        }
        let members = |fields: &Arc<Fields>| fields.parties().party(party).map(|p| (p.leader, p.members.clone()));
        assert_eq!(members(&fields), Some((leader_id, vec![leader_id, mid_id, high_id])), "join order");
        let _ = mid.tick(3_000);
        let _ = high.tick(3_000);

        // 1. The level-20 member logs out first: a member, so nothing moves.
        let _ = high.on_log_out();
        drop(high);
        assert_eq!(members(&fields), Some((leader_id, vec![leader_id, mid_id, high_id])), "a member's disconnect changes nothing");

        // 2. The leader logs out: the highest-level member ONLINE is the level-15 one - the
        //    level-20 one is offline and must not be crowned. The leader keeps their seat.
        let out = leader.on_log_out();
        assert!(out.iter().any(|r| r.opcode == net::notice::LOG_OUT_RESULT), "the log out is still answered");
        assert_eq!(members(&fields), Some((mid_id, vec![leader_id, mid_id, high_id])), "the crown moves to the online member; every seat stays");
        let mail = mid.tick(4_000);
        let codes: Vec<u8> = mail.iter().filter(|r| r.opcode == net::party::PARTY_RESULT).map(|r| r.body[0]).collect();
        assert!(codes.contains(&net::party::result::PARTY_STATE), "the new leader gets the refreshed window: {codes:?}");
        assert!(!codes.contains(&net::party::result::WITHDRAW), "and no one is said to have left: {codes:?}");
        drop(leader);
        assert_eq!(members(&fields), Some((mid_id, vec![leader_id, mid_id, high_id])), "said once; the drop after a log out says nothing more");

        // 3. The offline members log back in: still members, and the window is rebuilt at
        //    the login field entry (0x0D).
        let (back, windows) = join(leader_id);
        assert_eq!(windows, 1, "the returning member's window is rebuilt at the login field entry (0x0D)");
        assert_eq!(members(&fields), Some((mid_id, vec![leader_id, mid_id, high_id])));

        // 4. The new leader changes channel: a handover, the crown stays.
        let req = vec![1u8, 0, 0, 0];
        let cc = mid.on_change_channel(&req);
        let handing_over = mid.handing_over;
        drop(mid);
        assert!(handing_over, "the change marked the session as handing over: {:?}", cc.iter().map(|r| &r.what).collect::<Vec<_>>());
        assert_eq!(members(&fields), Some((mid_id, vec![leader_id, mid_id, high_id])), "a channel change keeps the leader and the seats");

        // 5. The returning old leader is now the only member online, and a plain member. Their
        //    crash is the last connection going: the party does not outlive it.
        drop(back);
        assert!(members(&fields).is_none(), "the last member online left: a party with nobody in the game does not exist");

        // 6. Whoever comes back next has no party and no window.
        let (alone, windows) = join(mid_id);
        assert_eq!(windows, 0, "no party, no 0x0D at login");
        drop(alone);
        assert!(members(&fields).is_none());

        // 7. The mirror of 5: a LEADER alone online, crashing, disbands too (both branches of
        //    the same request).
        let (mut a, _) = join(leader_id);
        let (mut b, _) = join(high_id);
        let created = a.run_party_request(leader_id, crate::party::Request::Create { name: "Again".into() });
        let again = u32::from_le_bytes(created[0].body[1..5].try_into().unwrap());
        let _ = a.run_party_request(leader_id, crate::party::Request::Invite { target: high_id });
        let _ = b.tick(5_000);
        let _ = b.run_party_request(high_id, crate::party::Request::Accept { party: again });
        let _ = a.tick(6_000);
        let seats = |fields: &Arc<Fields>| fields.parties().party(again).map(|p| (p.leader, p.members.len()));
        assert_eq!(seats(&fields), Some((leader_id, 2)));
        drop(b);
        assert_eq!(seats(&fields), Some((leader_id, 2)), "a member left with the leader online: nothing moves");
        drop(a);
        assert!(seats(&fields).is_none(), "the leader was the last one online: disbanded");
    }

    /// The `0x00E7` body the client sends for a typed line: u32 tick, the text, u8 tab.
    fn gm_chat_body(text: &str) -> Vec<u8> {
        let mut b = net::opcode::CLIENT_CHAT.to_le_bytes().to_vec();
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&(text.len() as u16).to_le_bytes());
        b.extend_from_slice(text.as_bytes());
        b.push(0);
        b
    }

    /// Two sessions, one channel, one map - and a **real** `0x0224` crosses
    /// between them. This is the end-to-end claim of the whole feature, and it
    /// is the test that fails if any link in the chain is unhooked: the
    /// `0x00DC` arm, `announce_field_entry`, `Bus::enter_field`, the mailbox,
    /// or the drain in `handle`.
    ///
    /// **Nothing here has been on a screen.** Every offset in the body is
    /// static analysis (`research/user-enter-field.md`), so this proves the
    /// server does what it was told to do, not that the client likes it.
    #[test]
    fn two_players_entering_one_map_are_announced_to_each_other() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Wanderer", "Stranger"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: 104_040_000,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }

        let mut first = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut second = Session::joining(store, config, fields.clone());
        first.claim_for_character(ids[0]);
        second.claim_for_character(ids[1]);

        // The first player arrives to an empty map and is told about nobody.
        let alone = first.on_field_entered();
        assert!(
            !alone.iter().any(|r| r.opcode == net::userpool::USER_ENTER_FIELD),
            "there is nobody else on the map yet"
        );

        // The second arrives, and is handed the first.
        let joined = second.on_field_entered();
        let spawns: Vec<&Reply> = joined
            .iter()
            .filter(|r| r.opcode == net::userpool::USER_ENTER_FIELD)
            .collect();
        assert_eq!(spawns.len(), 1, "the second player should be told about the first");
        assert_eq!(
            u32::from_le_bytes(spawns[0].body[4..8].try_into().unwrap()),
            ids[0],
            "and it should carry the first player's character id"
        );

        // ...and the first hears about the second without having asked.
        let mail = first.tick(1_000);
        let told: Vec<&Reply> = mail
            .iter()
            .filter(|r| r.opcode == net::userpool::USER_ENTER_FIELD)
            .collect();
        assert_eq!(told.len(), 1, "the sighting must go both ways: {mail:?}");
        assert_eq!(
            u32::from_le_bytes(told[0].body[4..8].try_into().unwrap()),
            ids[1],
        );
    }

    /// **A pet summoned BEFORE the other player arrives still reaches them.** The owner,
    /// 2026-09-14: *"The second client does not see the Husky pet."* The summon and every walk
    /// were already published to the map, and a pet owner walking into a map already went out
    /// as spawn-then-pet. The direction nobody covered was this one: the pet is out, someone
    /// else arrives, and they got the owner's spawn alone - the pet lived on one screen only.
    ///
    /// Three things are pinned. The arrival is handed the owner's `0x0224` and then the pet's
    /// `0x0277`, **in that order**, because the pet packet names a character the spawn creates
    /// and the client drops a user-pool packet for a character it does not have. The owner's
    /// own `0x0277` for the arrival is not sent back to the owner. And putting the pet away
    /// clears it, so the next arrival is not handed a pet that is no longer out.
    ///
    /// The pet-broadcast path, which is the default (`Config::broadcast_pets`). The move
    /// packet's own shape was the 2026-09-15 crash - see `net::pet::pet_move_broadcast`;
    /// `a_pet_is_owner_local_when_broadcast_is_disabled` pins the off fallback.
    #[test]
    fn a_pet_summoned_before_another_player_arrives_is_handed_to_them_after_the_owners_spawn() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Owner", "Arrival"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: 104_040_000, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        store.add_item(ids[0], store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
        let mut item_names = std::collections::HashMap::new();
        item_names.insert(5_000_006u32, "Husky".to_string());
        let config = Arc::new(Config { item_names, ..(*config).clone() });
        assert!(config.broadcast_pets, "pets are broadcast by default");

        let mut owner = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut arrival = Session::joining(store, config, fields.clone());
        owner.claim_for_character(ids[0]);
        arrival.claim_for_character(ids[1]);

        // The owner is alone, summons, and nobody is there to hear it.
        owner.on_field_entered();
        owner.last_position = Some((300, -50));
        let summon = owner.on_pet_activate(&super::tests::hex("509a18140100"));
        assert!(summon.iter().any(|r| r.opcode == net::pet::PET_ACTIVATED), "the owner sees the pet");

        // Then the other player arrives: spawn, then pet, in that order.
        let joined = arrival.on_field_entered();
        let about_owner: Vec<(u16, u32)> = joined
            .iter()
            .filter(|r| r.opcode == net::userpool::USER_ENTER_FIELD || r.opcode == net::pet::PET_ACTIVATED)
            .map(|r| {
                let at = if r.opcode == net::userpool::USER_ENTER_FIELD { 4 } else { 0 };
                (r.opcode, u32::from_le_bytes(r.body[at..at + 4].try_into().unwrap()))
            })
            .collect();
        assert_eq!(
            about_owner,
            vec![(net::userpool::USER_ENTER_FIELD, ids[0]), (net::pet::PET_ACTIVATED, ids[0])],
            "the arrival is handed the owner, then the owner's pet - the order the client needs"
        );
        let pet = joined.iter().find(|r| r.opcode == net::pet::PET_ACTIVATED).unwrap();
        assert_eq!(pet.body[8], 1, "activated");
        assert_eq!(&pet.body[10..14], &5_000_006u32.to_le_bytes(), "the Husky");
        assert_eq!(&pet.body[46..48], &net::pet::PET_SIZE_PERCENT.to_le_bytes(), "life-size on their screen too");

        // The owner hears about the arrival and is NOT handed their own pet back.
        let mail = owner.tick(1_000);
        assert!(mail.iter().any(|r| r.opcode == net::userpool::USER_ENTER_FIELD));
        assert!(!mail.iter().any(|r| r.opcode == net::pet::PET_ACTIVATED), "not echoed to the owner: {mail:?}");

        // Put away: the next arrival gets no pet. (The live observer got the put-away itself.)
        owner.on_pet_activate(&super::tests::hex("f29d18140100"));
        let _ = arrival.tick(2_000);
        let again = arrival.on_field_entered();
        assert!(
            !again.iter().any(|r| r.opcode == net::pet::PET_ACTIVATED),
            "a pet that has been put away must not travel with its owner any more"
        );
    }

    /// **A player arriving is handed the pet where it last walked, not where it was summoned.**
    ///
    /// The owner, 2026-09-24: *"Whenever a client enters the map, if other players have pets
    /// summoned, I see their pets snap to their proper positions. We fixed this for players."*
    /// The owner summons at (300, -50) and the pet then walks - a real `0x0202` whose path
    /// ends at (75, 274), facing left (`0x03`). The arrival's `0x0277` must carry that point
    /// and that stance. Then the control: a walk recorded on ANOTHER field says nothing about
    /// this one, and the arrival is handed the owner's position as before.
    #[test]
    fn an_arrival_is_handed_the_pet_where_it_last_walked_not_where_it_was_summoned() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Owner", "Arrival"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: 104_040_000, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        store.add_item(ids[0], store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
        let mut item_names = std::collections::HashMap::new();
        item_names.insert(5_000_006u32, "Husky".to_string());
        let config = Arc::new(Config { item_names, ..(*config).clone() });
        let mut owner = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut arrival = Session::joining(store, config, fields.clone());
        owner.claim_for_character(ids[0]);
        arrival.claim_for_character(ids[1]);
        owner.on_field_entered();
        owner.last_position = Some((300, -50));
        let _ = owner.on_pet_activate(&super::tests::hex("509a18140100"));

        // (x, y, stance) out of an arrival's 0x0277 for the owner's pet.
        let pet_at = |out: &[Reply]| -> (i16, i16, u8) {
            let b = &out.iter().find(|r| r.opcode == net::pet::PET_ACTIVATED).expect("the pet").body;
            let name_len = usize::from(u16::from_le_bytes([b[14], b[15]]));
            let at = 14 + 2 + name_len + 8;
            (i16::from_le_bytes([b[at], b[at + 1]]), i16::from_le_bytes([b[at + 2], b[at + 3]]), b[at + 4])
        };

        // The pet walks: 14:27:26.142 in previous-runs/world-ch0-20260922-195941.log, whose
        // path ends at (75, 274) - net::usermove's test pins that against the next report.
        let walk = super::tests::hex(
            "000000000000000000700012010000000003000070001201000000008f0000000000000016d20000005a0012016aff00008f0000000000000003c90000004b0012016aff00008a000000000000000363000000",
        );
        assert!(owner.on_pet_move(&walk).is_empty(), "a report; the owner is not answered");
        let (x, y, stance) = pet_at(&arrival.on_field_entered());
        assert_eq!((x, y), (75, 274), "where the pet walked to, not the summon point (300, -50)");
        assert_eq!(stance, 0x03, "and as it stood at the end of the walk");

        // The control: the same walk recorded on another field is not this field's answer.
        let _ = arrival.tick(1_000);
        let elsewhere = crate::fields::FieldKey { map: 100_000_000, instance: 0 };
        owner.pet_position = Some((elsewhere, 75, 274, Some(0x03)));
        let chr = owner.claimed_character().unwrap();
        let companions = owner.pet_companions(&chr);
        let (x, _, _) = pet_at(&companions);
        assert_eq!(x, 300, "a walk on another map says nothing here; the owner's position stands");
    }

    /// **`broadcast_pets = false` is owner-local: no other client is ever handed the pet.**
    /// The safe fallback if the remote pet ever misbehaves again - neither the live observer
    /// nor a later arrival gets a `0x0277`/`0x0278`, and the owner's own pet is unaffected.
    #[test]
    fn a_pet_is_owner_local_when_broadcast_is_disabled() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Owner", "Arrival"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: 104_040_000, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        store.add_item(ids[0], store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
        let mut item_names = std::collections::HashMap::new();
        item_names.insert(5_000_006u32, "Husky".to_string());
        let config = Arc::new(Config { item_names, broadcast_pets: false, ..(*config).clone() });

        let mut owner = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut observer = Session::joining(store, config, fields.clone());
        owner.claim_for_character(ids[0]);
        observer.claim_for_character(ids[1]);

        // The observer is already standing in the map, then the owner summons.
        observer.on_field_entered();
        owner.on_field_entered();
        owner.last_position = Some((300, -50));
        let summon = owner.on_pet_activate(&super::tests::hex("509a18140100"));
        assert!(summon.iter().any(|r| r.opcode == net::pet::PET_ACTIVATED), "the OWNER still sees the pet");

        // The present observer is told nothing about the pet - not the summon, and (since it
        // is never on their screen) never a move.
        let heard = observer.tick(1_000);
        assert!(
            !heard.iter().any(|r| r.opcode == net::pet::PET_ACTIVATED || r.opcode == net::pet::PET_MOVE),
            "no pet packet reaches a present observer: {:?}",
            heard.iter().map(|r| r.opcode).collect::<Vec<_>>()
        );
        // The owner's own pet walks: still nothing to the map.
        let body = super::tests::hex("000000000000000000360112010000000001000036011201000000002a0000000000000004fe010000");
        assert!(owner.on_pet_move(&body).is_empty());
        assert!(!observer.tick(2_000).iter().any(|r| r.opcode == net::pet::PET_MOVE), "no walk broadcast either");

        // And a player arriving after the summon is not handed the pet.
        let joined = observer.on_field_entered();
        assert!(
            !joined.iter().any(|r| r.opcode == net::pet::PET_ACTIVATED),
            "an arrival is not handed a pet that would crash on the owner's first step"
        );
    }

    /// **A hat put on the pet reaches the other player without a map change.** The owner,
    /// 2026-09-15: *"Wearing the Blue Top Hat on the pet does not show for different clients
    /// when first worn (upon loading into Cash Shop and then return it does show)."* The hat is
    /// in the character's look (cash slot 114 = body slot 14), so the observer gets this one
    /// character again the way a fresh sighting arrives - `0x0225`, the `0x0224` with the hat
    /// in the look, and the pet right behind it (2026-09-18: a bare second `0x0224` was a
    /// no-op in the pool, measured; the put-away-and-summon this used to add on top is gone
    /// with it). Since 2026-09-18 every worn change does this, not only the pet slot -
    /// `a_worn_change_is_re_announced_to_the_map` in `session::tests` has the regular and
    /// cash cases. **[I]** on the remote redraw; the wire is what this pins.
    #[test]
    fn a_hat_put_on_the_pet_is_re_announced_to_the_map() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Owner", "Watcher"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: 104_040_000, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        store.add_item(ids[0], store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
        store.add_item(ids[0], store::InventoryType::Deco, &store::Item::equip(1_802_006), 1).unwrap();
        let mut item_names = std::collections::HashMap::new();
        item_names.insert(5_000_006u32, "Husky".to_string());
        let config = Arc::new(Config { item_names, ..(*config).clone() });

        let mut owner = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut watcher = Session::joining(store.clone(), config, fields.clone());
        owner.claim_for_character(ids[0]);
        watcher.claim_for_character(ids[1]);
        watcher.on_field_entered();
        owner.on_field_entered();
        owner.last_position = Some((300, -50));
        owner.on_pet_activate(&super::tests::hex("509a18140100"));
        let _ = watcher.tick(1_000); // the spawn and the summon, already covered elsewhere

        // Deco slot 1 -> worn -114: world-ch0.log 02:59:25, byte for byte.
        let out = owner.on_inventory_move(&super::tests::hex("4feb50200601008effffff"));
        assert!(out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "the owner's own move reply");
        let worn = store.equipped_items(ids[0]).unwrap();
        assert!(worn.iter().any(|e| e.slot == crate::session::pet::PET_EQUIP_WORN_SLOT && e.item_id == 1_802_006), "{worn:?}");

        let heard: Vec<u16> = watcher.tick(2_000).into_iter().map(|r| r.opcode).collect();
        // One in-place 0x02AE with the hat in the look; no leave, no enter, and the pet is NOT
        // re-sent, because the observer's copy of the owner (and its pet) is never destroyed.
        // [D] whether the remote pet re-reads the hat from the rebuilt look; the plan says so.
        assert_eq!(heard, vec![net::lookupdate::USER_LOOK_UPDATE_REMOTE], "one in-place redress and nothing else: {heard:x?}");
        // And the announced look now carries the hat at body slot 14.
        let chr = owner.claimed_character().unwrap();
        let look_bytes = net::opcode::avatar_look(&chr);
        assert!(look_bytes.windows(5).any(|w| w[0] == 14 && w[1..5] == 1_802_006u32.to_le_bytes()), "slot 14 = the hat");

        // A move that does not touch the pet slot re-announces nothing.
        let quiet = watcher.tick(3_000);
        assert!(quiet.iter().all(|r| r.opcode != net::userpool::USER_ENTER_FIELD));
    }

    /// **The eating animation and the level-up flash reach the other player.** The owner,
    /// 2026-09-16: *"I do want the eating animation to play for the client and other players.
    /// When closeness levels up, it should also play an animation to the client and other
    /// players in the map."* The watcher gets `0x02AF` effect 9 / subtype 0 naming the owner.
    ///
    /// **And the pet's line is the owner's line** (the owner, 2026-09-25: *"sync the chat bubbles so
    /// that the dialogues are the same"*). The owner's client reports the line it picked
    /// (`0x0203`, bytes from the deployed capture) and the watcher is shown exactly that as
    /// `0x0279` - not a `0x027E` of its own, which made each client pick a random line. Three
    /// paths, all asserted: the report relays; a report with no feed waiting relays nothing;
    /// and a feed whose report never comes puts the `0x027E` on the map after the wait.
    #[test]
    fn a_feed_and_the_level_it_earns_are_seen_by_the_other_player() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Owner", "Watcher"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: 104_040_000, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        store.add_item(ids[0], store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
        store.add_item(ids[0], store::InventoryType::Use, &store::Item::bundle(2_120_000, 2), 100).unwrap();
        let food_slot = store.bag_items(ids[0], store::InventoryType::Use).unwrap().iter().find(|r| r.item.item_id == 2_120_000).unwrap().slot;
        let mut item_names = std::collections::HashMap::new();
        item_names.insert(5_000_006u32, "Husky".to_string());
        let config = Arc::new(Config { item_names, ..(*config).clone() });

        let mut owner = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut watcher = Session::joining(store.clone(), config, fields.clone());
        owner.claim_for_character(ids[0]);
        watcher.claim_for_character(ids[1]);
        watcher.on_field_entered();
        owner.on_field_entered();
        owner.last_position = Some((300, -50));
        owner.on_pet_activate(&super::tests::hex("509a18140100"));
        let pet_id = store.pet_id_at(ids[0], store::InventoryType::Cash, 1).unwrap().expect("the Husky is numbered");
        store.set_pet_vitals(pet_id, 1, 0, 50).unwrap();
        let _ = watcher.tick(1_000);
        let _ = owner.tick(1_000);

        let mut body = net::petfood::CLIENT_USE_PET_FOOD.to_le_bytes().to_vec();
        body.extend_from_slice(&0x2050_8e0au32.to_le_bytes());
        body.extend_from_slice(&food_slot.to_le_bytes());
        body.extend_from_slice(&2_120_000u32.to_le_bytes());
        let own = owner.handle(&body);
        assert!(own.iter().any(|r| r.opcode == net::pet::PET_ACTION_COMMAND), "the owner sees it eat");

        // 1. The owner's client reports its pet's line - Moth's deployed bytes.
        let mut report = net::pet::CLIENT_PET_LINE_REPORT.to_le_bytes().to_vec();
        report.extend(super::tests::hex("0000000061107122020a1500546869732069732064656c6963696f75732e2e2e21"));
        assert!(owner.handle(&report).is_empty(), "a report is never answered - the builder sets no latch");
        let heard = watcher.tick(2_000);
        assert!(!heard.iter().any(|r| r.opcode == net::pet::PET_ACTION_COMMAND), "no 0x027E of its own: it would pick its own line");
        let said = heard.iter().find(|r| r.opcode == net::pet::PET_ACTION).expect("the watcher is shown the owner's line");
        assert_eq!(&said.body[0..4], &ids[0].to_le_bytes(), "the owner's pet");
        assert_eq!((said.body[8], said.body[9]), (2, 0x0a), "the two bytes as the owner's client reported them");
        assert!(said.body.ends_with(b"This is delicious...!"), "the SAME line");
        let flash = heard.iter().find(|r| r.opcode == net::stats::USER_EFFECT_REMOTE).expect("the watcher sees the level-up");
        assert_eq!(&flash.body[0..4], &ids[0].to_le_bytes());
        assert_eq!(&flash.body[4..], &[net::pet::USER_EFFECT_PET, net::pet::PET_EFFECT_LEVEL_UP, 0, 0, 0, 0]);
        // The Cash item itself stays with the owner: nothing the watcher receives is an inventory op.
        assert!(heard.iter().all(|r| r.opcode != net::inventory::INVENTORY_OPERATION));

        // 2. A second report for the same feed - or a bystander's - relays nothing.
        assert!(owner.handle(&report).is_empty());
        let _ = owner.tick(5_000);
        assert!(!watcher.tick(5_000).iter().any(|r| r.opcode == net::pet::PET_ACTION), "one line per feed");

        // 3. A feed whose line never comes: the map gets the eating packet after the wait.
        let _ = owner.handle(&body);
        let _ = owner.tick(5_000 + 1_000);
        assert!(!watcher.tick(6_000).iter().any(|r| r.opcode == net::pet::PET_ACTION_COMMAND), "still waiting for the line");
        let _ = owner.tick(5_000 + super::super::pet::PET_LINE_WAIT_MS + 1);
        let late = watcher.tick(8_000);
        let ate = late.iter().find(|r| r.opcode == net::pet::PET_ACTION_COMMAND).expect("the fallback: the watcher sees it eat");
        assert_eq!(&ate.body[0..4], &ids[0].to_le_bytes(), "the owner's pet");
        assert_eq!((ate.body[8], ate.body[9]), (net::pet::PET_ACTION_FOOD, 1));
        // Food id 0 on the map too: a real id draws the auto-feed "Yum, yum!" balloon over the
        // pet on the watcher's screen as well (2026-09-18; net::pet::pet_ate).
        assert_eq!(&ate.body[10..14], &net::pet::PET_FOOD_NONE.to_le_bytes());
    }

    /// **A kill on one connection pays a character on another.** This is the whole
    /// EXP-share feature end to end, and it is the test that fails if any link is
    /// unhooked: `award_kill_experience`, `Bus::send_to_character`, the mailbox,
    /// `Bus::drain_events`, or the conversion back into a packet in `collect_mail`.
    ///
    /// The assertion that matters most is the one *before* the tick. The helper has
    /// earned nothing at the moment the killer is paid, because the fact has crossed
    /// the bus but nobody has turned it into a packet yet - which is the design:
    /// `0x007C` carries the recipient's own new total, so only the recipient's session
    /// may compute it. If a future change ever builds the helper's packet inside the
    /// killer's session, that line is what catches it.
    #[test]
    fn a_helpers_share_of_a_kill_reaches_their_own_session() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Killer", "Helper"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: 104_040_000,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }

        let mut killer = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut helper = Session::joining(store.clone(), config, fields.clone());
        killer.claim_for_character(ids[0]);
        helper.claim_for_character(ids[1]);
        killer.on_field_entered();
        helper.on_field_entered();
        // Clear the entry announcements so what is left is the EXP and nothing else.
        killer.tick(1_000);
        helper.tick(1_000);

        let exp_of = |id: u32| {
            store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().exp
        };

        // 70/30, so the killer holds the majority and the helper is owed 30 of 100.
        let shares = vec![
            crate::fields::DamageShare { character: ids[0], dealt: 70, total: 100, majority: true },
            crate::fields::DamageShare { character: ids[1], dealt: 30, total: 100, majority: false },
        ];
        let paid = killer.award_kill_experience(100, "a kill", ids[0], &shares);

        assert!(
            paid.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
            "the killer is paid in their own reply: {paid:?}"
        );
        assert_eq!(exp_of(ids[0]), 100, "the majority holder takes the whole amount");
        assert_eq!(
            exp_of(ids[1]),
            0,
            "and the helper has NOT been paid yet - the fact is on the bus, but only the              helper's own session may build a packet carrying the helper's own total"
        );

        // The helper collects their mail, and only now is anything written for them.
        let mail = helper.tick(2_000);
        assert!(
            mail.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
            "the helper's own session turns the fact into a packet: {mail:?}"
        );
        assert_eq!(exp_of(ids[1]), 30, "30/100 of the damage, so 30 of the 100 exp");

        // **And the killer collects their own mail too.** Without this tick the test
        // cannot see a double payout at all: a share the killer wrongly addressed to
        // itself would sit undrained in its own mailbox and every assertion above would
        // still pass. Verified by injecting exactly that bug - dropping the
        // `s.character != chr_id` filter in `award_kill_experience` - which this line
        // catches and nothing else did.
        let again = killer.tick(3_000);
        assert!(
            !again.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
            "the killer must not have addressed a share to itself: {again:?}"
        );
        assert_eq!(exp_of(ids[0]), 100, "and so it is still paid exactly once");
    }

    /// A share owed to somebody who is not on this channel is dropped, not retried and
    /// not panicked over. They logged out between landing the hit and the mob dying,
    /// which is ordinary. The killer is still paid.
    #[test]
    fn a_share_owed_to_an_absent_character_is_dropped_and_the_killer_still_pays_itself() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character {
            name: "Lonely".to_string(),
            map_id: 104_040_000,
            ..Default::default()
        };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();

        let mut killer = Session::joining(store.clone(), config, fields);
        killer.claim_for_character(id);
        killer.on_field_entered();
        killer.tick(1_000);

        // 9_999 is nobody: no session on this channel is playing them.
        let shares = vec![
            crate::fields::DamageShare { character: id, dealt: 70, total: 100, majority: true },
            crate::fields::DamageShare { character: 9_999, dealt: 30, total: 100, majority: false },
        ];
        let paid = killer.award_kill_experience(100, "a kill", id, &shares);

        assert!(paid.iter().any(|r| r.opcode == net::stats::STAT_CHANGED), "{paid:?}");
        let exp = store
            .characters_for(account, 0)
            .unwrap()
            .into_iter()
            .find(|c| c.id == id)
            .unwrap()
            .exp;
        assert_eq!(exp, 100, "the absent share changes nothing about the killer's own");
    }

    /// **A real captured walk crosses between two sessions.** This is the other
    /// half of what was asked for, end to end: a `0x00D9` body this client
    /// actually sent, fed to one session's `handle`, comes out of the other's as
    /// `0x0293` carrying the path **byte for byte**.
    ///
    /// The body is `14:01:40.792` from
    /// `research/fixtures/character-on-map1-playable-world.log` - the same
    /// capture `net::usermove`'s own tests parse.
    #[test]
    fn a_real_captured_walk_is_rebroadcast_to_the_other_player() {
        const MAP1_FIRST: &str = "0057a301a8c139cc04000000000043ffab010000000003000043\
ffd7010000a401000000000000ffff06d200000043ffe50100000000000000000000ffff061e00000043ffe5\
01000000002b0000000000ffff040e010011000000000000000000";
        let path_bytes: Vec<u8> = (0..MAP1_FIRST.len() / 2)
            .map(|i| u8::from_str_radix(&MAP1_FIRST[i * 2..i * 2 + 2], 16).unwrap())
            .collect();

        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Wanderer", "Stranger"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: 104_040_000,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        let mut watcher = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut mover = Session::joining(store, config, fields.clone());
        watcher.claim_for_character(ids[0]);
        mover.claim_for_character(ids[1]);
        watcher.on_field_entered();
        mover.on_field_entered();
        let _ = watcher.tick(1_000); // clear the arrival

        // The parse has to be the same one the session does, or this test is
        // asserting against its own arithmetic rather than the client's bytes.
        let parsed = net::usermove::parse_user_move(&path_bytes).expect("a real body");
        assert!(parsed.walk_closed, "the fixture must walk closed or nothing is sent");
        let path = parsed.path(&path_bytes).expect("a closed walk has a span");

        let mut packet = net::usermove::CLIENT_USER_MOVE.to_le_bytes().to_vec();
        packet.extend_from_slice(&path_bytes);
        assert!(mover.handle(&packet).is_empty(), "the mover itself is not answered");

        let mail = watcher.handle(&[0xFF, 0xFE]);
        let moves: Vec<&Reply> = mail
            .iter()
            .filter(|r| r.opcode == net::userpool::USER_MOVE_REMOTE)
            .collect();
        assert_eq!(moves.len(), 1, "one move should have crossed: {mail:?}");

        let body = &moves[0].body;
        assert_eq!(
            u32::from_le_bytes(body[0..4].try_into().unwrap()),
            ids[1],
            "addressed to the mover's character"
        );
        assert_eq!(&body[4..], path, "the path is copied verbatim");
        assert_eq!(
            body.len(),
            4 + path.len(),
            "no key-state trailer - 0x0293 passes zero at 1429d2eb5"
        );
        assert!(
            body.len() < path_bytes.len(),
            "and it is therefore SHORTER than the inbound body, which carries one"
        );
    }

    /// `previous-runs/world-20260820-121055.log` 16:10:28.598, `0x00DF`, 229 bytes: a
    /// plain swing that connected — mob 2002, one hit of 19, not critical. The same body
    /// `net::attack` and `crate::remoteattack` both test against, so all three agree about
    /// what the bytes mean.
    const MELEE_229: &str = "0001000000000000000000000000000001050000009fae340801040000003b80680a70028b010000000070028b0100000000000000000000000000000000000000000000000000000100000001000000000a0055736572204d656c65658901000000000000000000000000000000010000000000000000000000d20700000200000001000013000000000000000000000736028b0136028b0135027b01890100000000000001000002000000000001012302710148028b01000000007e6c3c6600000000030000000000d5c057820100000092e9bc2707000000bc6509e5000080e8da8f00";

    fn hex_body(hex: &str) -> Vec<u8> {
        (0..hex.len() / 2)
            .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).expect("fixture hex"))
            .collect()
    }

    /// Two characters on one map, and a **real captured swing** crossing between them.
    ///
    /// This is the attack half of the owner's 2026-08-29 sentence end to end, and it fails if
    /// any link is unhooked: the `is_attack_opcode` arm, `on_attack`,
    /// `publish_user_attack`, `crate::remoteattack`, `Bus::publish`, the mailbox, or the
    /// drain in `handle`.
    ///
    /// **Four effects are asserted, not one.** `CLAUDE.md`: a test that checks one of
    /// several effects gives false confidence about the rest.
    ///
    /// **Nothing here has been on a screen.** Every offset in the body is static analysis
    /// (`research/user-pool-tables.md` §4), so this proves the server does what it was told
    /// to do, not that the client likes it.
    #[test]
    fn a_real_captured_swing_is_rebroadcast_to_the_other_player() {
        let payload = hex_body(MELEE_229);
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Watcher", "Swinger"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: 104_040_000,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        let mut watcher = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut attacker = Session::joining(store.clone(), config, fields.clone());
        watcher.claim_for_character(ids[0]);
        attacker.claim_for_character(ids[1]);
        // The `u16` at body offset 4 is `[user + 0x406c]` - the same dword `0x0224` fills
        // with the level. Read it from the record rather than assuming, so this test also
        // pins WHICH number the session chose to send.
        let level = store
            .characters_for(account, 0)
            .unwrap()
            .into_iter()
            .find(|c| c.id == ids[1])
            .unwrap()
            .level;
        watcher.on_field_entered();
        attacker.on_field_entered();
        let _ = watcher.tick(1_000); // clear the arrivals
        let _ = attacker.tick(1_000);

        // The parse has to be the same one the session does, or this test asserts against
        // its own arithmetic rather than against the client's bytes.
        let parsed = net::attack::parse(net::combat::USER_MELEE_ATTACK, &payload)
            .expect("a real captured body");
        let expected = crate::remoteattack::user_attack_remote(
            ids[1],
            u16::try_from(level).unwrap(),
            &parsed,
        );

        let mut packet = net::combat::USER_MELEE_ATTACK.to_le_bytes().to_vec();
        packet.extend_from_slice(&payload);
        let own = attacker.handle(&packet);

        // (1) The attacker never receives their own swing - their client drew it already.
        assert!(
            !own.iter().any(|r| r.opcode == 0x029E),
            "the attacker must not be sent their own broadcast: {own:?}"
        );

        // (2) The other player does, exactly once.
        let mail = watcher.handle(&NOTHING);
        let swings: Vec<&Reply> = mail.iter().filter(|r| r.opcode == 0x029E).collect();
        assert_eq!(swings.len(), 1, "one swing should have crossed: {mail:?}");

        // (3) ...and the body is the one `crate::remoteattack` builds, byte for byte,
        //     addressed to the attacker and 85 bytes long.
        assert_eq!(swings[0].body, expected, "the body must be the projected re-encode");
        assert_eq!(
            u32::from_le_bytes(swings[0].body[0..4].try_into().unwrap()),
            ids[1],
            "addressed to the swinger"
        );
        assert_eq!(
            u16::from_le_bytes(swings[0].body[4..6].try_into().unwrap()),
            u16::try_from(level).unwrap(),
            "the u16 at 4 is the attacker's level, the same field 0x0224 sets at its 12"
        );
        assert_eq!(swings[0].body.len(), 85, "6 + 43 + 12 + (6 + 10 + 8)");
        assert!(
            swings[0].body.len() < payload.len(),
            "and it is SHORTER than the inbound body - 13 header fields of 40, no trailer"
        );
        assert_eq!(
            u64::from_le_bytes(swings[0].body[69..77].try_into().unwrap()),
            19,
            "the damage the attacker's own client computed, absolute and unscaled"
        );

        // (4) A second swing is a second packet. If `publish` ever gained a supersede key
        //     here, a whole fight would render as one hit.
        let _ = attacker.handle(&packet);
        let _ = attacker.handle(&packet);
        assert_eq!(
            watcher.handle(&NOTHING).iter().filter(|r| r.opcode == 0x029E).count(),
            2,
            "two swings are two events - nothing coalesces"
        );
    }

    /// **The broadcast is additive: the attacker gets exactly what they got before.**
    ///
    /// The same swing is run twice, on two independent channels - once with a second
    /// player on the map and once alone - and the attacker's own replies must be
    /// identical. That is the property the whole feature has to preserve, and it cannot be
    /// checked by looking at one session.
    ///
    /// The swing is patched to carry **Power Strike level 5** so the attacker's reply is
    /// genuinely non-empty (`session/combat.rs::spend_attack_mp` sends a `0x007C`);
    /// comparing two empty vectors would pass against a broadcast that had eaten the
    /// reply. That needs the generated skill table, so the test returns early without it,
    /// exactly as `session::tests::an_attack_skill_costs_mp_and_a_potion_does_not_undo_it`
    /// does.
    #[test]
    fn the_broadcast_does_not_change_what_the_attacker_gets() {
        let skills = std::path::Path::new("../../gm-handbook/skills.txt");
        if !skills.exists() {
            return; // generated, gitignored - python tools/dump_skills.py
        }
        const POWER_STRIKE: u32 = 1_001_001;

        let mut payload = hex_body(MELEE_229);
        payload[2..6].copy_from_slice(&POWER_STRIKE.to_le_bytes());
        payload[6] = 5;
        assert!(
            net::attack::parse(net::combat::USER_MELEE_ATTACK, &payload).is_ok(),
            "the captured body must still parse after patching"
        );
        let mut packet = net::combat::USER_MELEE_ATTACK.to_le_bytes().to_vec();
        packet.extend_from_slice(&payload);

        // One channel, one or two players on it, and the attacker's own reply.
        let swing_with = |companion: bool| -> Vec<(u16, Vec<u8>)> {
            let store = Arc::new(Store::open_in_memory().unwrap());
            let config = Arc::new(Config {
                firstjob: crate::firstjob::CombatTable::load(skills),
                ..Config::default()
            });
            let fields = Arc::new(Fields::new());
            let account = store.create_account("maplecw", "correct horse battery").unwrap();

            let make = |name: &str| {
                let chr = net::opcode::Character {
                    name: name.to_string(),
                    map_id: 104_040_000,
                    ..Default::default()
                };
                let id = store.create_character(account, 0, &chr).unwrap().id;
                let mut rec = store
                    .characters_for(account, 0)
                    .unwrap()
                    .into_iter()
                    .find(|c| c.id == id)
                    .unwrap();
                rec.job = 100;
                rec.mp = 100;
                rec.max_mp = 100;
                store.save_character_progress(&rec).unwrap();
                store.set_skill_level(id, POWER_STRIKE, 5).unwrap();
                store.create_migration(account, id, 0, 0).unwrap();
                id
            };
            let attacker_id = make("Swinger");
            let watcher_id = if companion { Some(make("Watcher")) } else { None };

            let mut attacker =
                Session::joining(store.clone(), config.clone(), fields.clone());
            attacker.claim_for_character(attacker_id);
            let mut watcher = watcher_id.map(|id| {
                let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
                s.claim_for_character(id);
                s
            });
            if let Some(w) = watcher.as_mut() {
                w.on_field_entered();
                let _ = w.tick(1_000);
            }
            attacker.on_field_entered();
            let _ = attacker.tick(1_000);

            attacker
                .handle(&packet)
                .into_iter()
                .map(|r| (r.opcode, r.body))
                .collect()
        };

        let alone = swing_with(false);
        let watched = swing_with(true);

        assert!(
            alone.iter().any(|(op, _)| *op == net::stats::STAT_CHANGED),
            "the MP spend must be in the reply, or this comparison is two empty vectors"
        );
        assert_eq!(
            watched, alone,
            "a second player on the map must not change one byte of what the attacker gets"
        );
        assert!(
            !watched.iter().any(|(op, _)| *op == 0x029E),
            "and the broadcast must never leak into the attacker's own reply"
        );
    }

    /// Different maps, no sighting. The bus filters by map, and a spawn packet
    /// for a character on another field names an object the client's pool does
    /// not have.
    #[test]
    fn two_players_on_different_maps_are_not_announced() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for (name, map) in [("Wanderer", 104_040_000u32), ("Stranger", 100_000_000)] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: map,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }

        let mut first = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut second = Session::joining(store, config, fields.clone());
        first.claim_for_character(ids[0]);
        second.claim_for_character(ids[1]);
        first.on_field_entered();

        let joined = second.on_field_entered();
        assert!(
            !joined.iter().any(|r| r.opcode == net::userpool::USER_ENTER_FIELD),
            "a different map is a different field"
        );
        assert!(
            !first
                .tick(1_000)
                .iter()
                .any(|r| r.opcode == net::userpool::USER_ENTER_FIELD),
            "and the first player hears nothing either"
        );
    }

    /// The wiring, in one test: a packet published by one session comes out of
    /// another session's `handle`. Everything else here is a corner of this.
    #[test]
    fn mail_published_by_one_session_comes_out_of_another_handle() {
        let (store, config, fields) = channel();
        let mut watcher = Session::joining(store.clone(), config.clone(), fields.clone());
        let mover = Session::joining(store, config, fields.clone());

        fields.bus().enter_field(watcher.subscriber, presence(200, crate::fields::FieldKey::world(7)));
        fields.bus().enter_field(mover.subscriber, presence(201, crate::fields::FieldKey::world(7)));
        assert_eq!(watcher.collect_mail().len(), 1, "the arrival of 201");

        fields.bus().publish(
            mover.subscriber,
            crate::fields::FieldKey::world(7),
            Reply { opcode: 0x02B0, body: vec![1], what: "201 walked".into() },
            Some(201),
        );

        let out = watcher.handle(&NOTHING);
        assert_eq!(out.len(), 1, "the walk should reach the other session");
        assert_eq!(out[0].what, "201 walked");
        assert!(watcher.handle(&NOTHING).is_empty(), "and only once");
    }

    /// The idle case, and the reason `tick` collects too. A player standing
    /// still sends no packets at all, so `handle` is never called on their
    /// connection - if the drain lived only there, the one client that most
    /// needs to see someone walk past would be the one that never did.
    #[test]
    fn a_session_that_sends_nothing_still_collects_its_mail_on_tick() {
        let (store, config, fields) = channel();
        let mut watcher = Session::joining(store.clone(), config.clone(), fields.clone());
        let mover = Session::joining(store, config, fields.clone());
        fields.bus().enter_field(watcher.subscriber, presence(200, crate::fields::FieldKey::world(7)));
        fields.bus().enter_field(mover.subscriber, presence(201, crate::fields::FieldKey::world(7)));
        let _ = watcher.collect_mail();

        fields.bus().publish(
            mover.subscriber,
            crate::fields::FieldKey::world(7),
            Reply { opcode: 0x02B0, body: vec![1], what: "201 walked".into() },
            Some(201),
        );

        let out = watcher.tick(1_000);
        assert!(
            out.iter().any(|r| r.what == "201 walked"),
            "a tick must deliver mail: {:?}",
            out.iter().map(|r| &r.what).collect::<Vec<_>>()
        );
    }

    /// A connection that dies without logging out out - a killed client, a dropped
    /// soccP socket - must still take its character off everyone else's field.
    /// This is the case `Drop for Session` exists for, and it is the one
    /// multiplayer bug that cannot be cleaned up from the client side.
    #[test]
    fn a_dropped_session_hands_back_its_mailbox_and_says_goodbye() {
        let (store, config, fields) = channel();
        let mut watcher = Session::joining(store.clone(), config.clone(), fields.clone());
        fields.bus().enter_field(watcher.subscriber, presence(200, crate::fields::FieldKey::world(7)));

        {
            let leaver = Session::joining(store, config, fields.clone());
            fields.bus().enter_field(leaver.subscriber, presence(201, crate::fields::FieldKey::world(7)));
            assert_eq!(fields.bus().subscribers(), 2);
            assert_eq!(watcher.collect_mail().len(), 1, "the arrival of 201");
        }

        assert_eq!(fields.bus().subscribers(), 1, "the mailbox went back");
        let out = watcher.handle(&NOTHING);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].what, "farewell 201");
    }

    /// A clean log out is the tidy version of the same thing, and it must
    /// announce the departure ***once*: the `Drop` that follows a moment later
    /// must not announce a second.
    #[test]

fn logging_out_leaves_the_field_and_the_later_drop_says_nothing_more() {
        let (store, config, fields) = channel();
        let mut watcher = Session::joining(store.clone(), config.clone(), fields.clone());
        fields.bus().enter_field(watcher.subscriber, presence(200, crate::fields::FieldKey::world(7)));

        {
            let mut leaver = Session::joining(store, config, fields.clone());
            fields.bus().enter_field(leaver.subscriber, presence(201, crate::fields::FieldKey::world(7)));
            let _ = watcher.collect_mail();
            leaver.leave_the_field();
        }

        let out = watcher.handle(&NOTHING);
        assert_eq!(
            out.iter().map(|r| r.what.clone()).collect::<Vec<_>>(),
            vec!["farewell 201"],
            "one departure, not two"
        );
    }

    /// A real captured `0x00D9`: 14:01:40.792, map 1, three elements, walks closed.
    /// Byte-identical to `net::usermove::tests::MAP1_FIRST`, which asserts every field of
    /// it against the `world.log` line it came from.
    const REAL_MOVE: &str = "0057a301a8c139cc04000000000043ffab010000000003000043ffd7010000a401000000000000ffff06d200000043ffe50100000000000000000000ffff061e00000043ffe501000000002b0000000000ffff040e010011000000000000000000";

    fn hex(h: &str) -> Vec<u8> {
        (0..h.len() / 2).map(|i| u8::from_str_radix(&h[i * 2..i * 2 + 2], 16).unwrap()).collect()
    }

    /// A `0x00D9` packet: the opcode, then the body.
    fn move_packet(body: &[u8]) -> Vec<u8> {
        let mut b = net::usermove::CLIENT_USER_MOVE.to_le_bytes().to_vec();
        b.extend_from_slice(body);
        b
    }

    /// **The smallest body that walks closed with NO elements**: the 10-byte head, the
    /// 14-byte path head with `element_count = 0`, and a key-state count of 0.
    ///
    /// 25 bytes. Nothing in the archive looks like this - all 170 806 captured paths carry
    /// at least one element - which is exactly why it has to be constructed here rather
    /// than quoted from a capture.
    fn empty_element_body() -> Vec<u8> {
        let mut b = vec![0u8; net::usermove::USER_MOVE_HEAD_LEN];
        b.extend_from_slice(&[0u8; net::usermove::MOVE_PATH_HEAD_LEN]);
        // element_count is an i16 at path-head offset 12.
        let at = net::usermove::USER_MOVE_HEAD_LEN + 12;
        b[at..at + 2].copy_from_slice(&0i16.to_le_bytes());
        b.push(0); // key-state count
        b
    }

    /// **An empty element list is not rebroadcast, and a real path is.**
    ///
    /// The pair is the whole test: a guard that refuses everything passes the first half on
    /// its own, and this project has shipped exactly that shape before.
    ///
    /// # What the guard is for
    ///
    /// `141d59bef mov rax,[rbp+0x18]` reads the head of the decoded element list and then
    /// eight `movups` out of its tail, unconditionally and with no null check. The list is
    /// empty when `element_count <= 0`, and such a body **walks closed** - `for _ in
    /// 0..count` runs no iterations, so `p` never moves off the path head and the key-state
    /// trailer lands exactly on the end. `UserMove::walk_closed` therefore does not cover
    /// it, which is the reason this is a separate check and not a stronger `walk_closed`.
    ///
    /// It is **latent**: no honest client sends one. The guard exists because the packet is
    /// rebroadcast *to other people* - a client that sent this would be killing somebody
    /// else's session, and this server is the only thing in between.
    #[test]
    fn a_movement_path_with_no_elements_is_not_rebroadcast() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Watcher", "Mover"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: 1,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        let mut watcher = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut mover = Session::joining(store, config, fields.clone());
        watcher.claim_for_character(ids[0]);
        mover.claim_for_character(ids[1]);
        watcher.on_field_entered();
        mover.on_field_entered();
        watcher.collect_mail(); // the arrival of the mover; not what this is about

        // ---- the control: a real captured path IS rebroadcast --------------------------
        let real = hex(REAL_MOVE);
        assert_eq!(real.len(), 97, "the world.log line says 97 bytes");
        mover.handle(&move_packet(&real));
        let out = watcher.collect_mail();
        assert_eq!(out.len(), 1, "a real walk must reach the other player: {out:?}");
        assert_eq!(out[0].opcode, net::userpool::USER_MOVE_REMOTE);

        // ---- the guard: an empty element list is NOT --------------------------------
        let empty = empty_element_body();
        assert_eq!(empty.len(), 25);
        // It parses, and it walks closed - which is the trap.
        let parsed = net::usermove::parse_user_move(&empty).expect("it parses");
        assert_eq!(parsed.element_count, 0);
        assert!(parsed.walk_closed, "THIS is why walk_closed cannot be the guard");
        assert!(parsed.path(&empty).is_some(), "and the path accessor hands it over");

        mover.handle(&move_packet(&empty));
        assert!(
            watcher.collect_mail().is_empty(),
            "an empty element list must never reach another client - it dereferences null"
        );

        // And the mover is still able to move afterwards: the guard drops one packet, it
        // does not wedge the connection.
        mover.handle(&move_packet(&real));
        assert_eq!(watcher.collect_mail().len(), 1, "the next real walk still goes out");
    }

    /// **Where a player was standing is a fact about the map they just left.**
    ///
    /// `last_position` is fed only by `0x00D9` and by attack packets, so after a warp it
    /// held the *previous* field's coordinates until the first step. Three readers used it
    /// and all three were wrong across a map change - the `0x0224` spawn other players are
    /// told, and the two drop paths.
    ///
    /// `None` is the honest answer, and an off-map coordinate is worse than no coordinate:
    /// the origin is at least inside the field, and a drop outside the client's own pick-up
    /// box is drawn and can never be collected.
    #[test]
    fn a_map_change_forgets_where_the_player_was_standing() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character {
            name: "Walker".to_string(),
            map_id: 1,
            ..Default::default()
        };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store.clone(), config, fields);
        s.claim_for_character(id);

        // A real walk on map 1 gives the session a position - the control, because a test
        // that only checks `None` at the end passes on a field that was never set.
        s.handle(&move_packet(&hex(REAL_MOVE)));
        let before = s.last_position.expect("the walk must set a position");
        assert_ne!(before, (0, 0), "and it is a real coordinate, not the origin");

        let mut moved = s.claimed_character().expect("a claimed character");
        s.go_to_map(&mut moved, 104_040_000, 0, "a portal walk".to_string());
        assert_eq!(
            s.last_position, None,
            "the new map must not inherit the old map's coordinates - and this config has no              portal positions, so there is nothing better than None to offer"
        );
    }

    /// **An arrival stands at the portal, in the landing pose, from the first frame.** The owner,
    /// 2026-09-14: *"The first client also sees the client joining start from the origin of the
    /// map and then snap to their real position."* With the portal's position known, the
    /// `0x0224` the field is told carries it - and the foothold under it, and action 4 - so
    /// the newcomer's first step is a walk, not a snap. The previous test is the fallback when
    /// `portals.txt` has no positions; this is the case when it does.
    #[test]
    fn an_arrival_is_announced_at_the_portal_in_the_landing_pose_not_at_the_origin() {
        let (store, config, fields) = channel();
        let mut positions = std::collections::HashMap::new();
        positions.insert((104_040_000u32, 3u8), (-1200i16, 190i16));
        let config = Arc::new(Config { portal_positions: positions, ..(*config).clone() });
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Walker".to_string(), map_id: 1, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store.clone(), config, fields);
        s.claim_for_character(id);
        s.handle(&move_packet(&hex(REAL_MOVE)));
        assert!(s.last_position.is_some(), "the control: a position from the old map");

        let mut moved = s.claimed_character().expect("a claimed character");
        s.go_to_map(&mut moved, 104_040_000, 3, "a portal walk".to_string());
        assert_eq!(s.last_position, Some((-1200, 190)), "the arrival portal, not the old map, not the origin");
        let at = s.remote_at();
        assert_eq!((at.x, at.y), (-1200, 190));
        assert_eq!(at.move_action, net::userpool::MOVE_ACTION_LANDING, "action 4, the jump, facing right");

        // An unknown portal index on a known map is still the honest None, not a guess.
        let mut moved = s.claimed_character().expect("a claimed character");
        s.go_to_map(&mut moved, 104_040_000, 9, "a portal nobody mapped".to_string());
        assert_eq!(s.last_position, None);
        assert_eq!(s.remote_at().move_action, net::userpool::MOVE_ACTION_STANDING);
    }


    /// Two players on one map, and a helper to put them there.
    fn two_on_a_map(store: &Arc<Store>, map: u32) -> (u32, u32) {
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Stayer", "Leaver"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: map,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        (ids[0], ids[1])
    }

    /// **A server-initiated warp publishes its own farewell**, without waiting for the
    /// warped client to volunteer anything.
    ///
    /// Until this, the departure was a side effect of the *arrival*: `Bus::enter_field`
    /// leaves the old field first, and that runs from `on_field_entered`, which fires on the
    /// client's `0x00DC`. So the old map's players stopped seeing you only if your client
    /// sent a packet. 273 SetFields against 268 markers across 444 archived logs - and the
    /// five that did not answer are the entire point, because the thing that would have
    /// removed the ghost was the packet that never came.
    ///
    /// The test deliberately **never calls `on_field_entered` on the leaver after the warp**.
    /// That is what makes it a test of this fix rather than of the arrival path.
    #[test]
    fn a_server_warp_removes_the_player_from_the_old_map_by_itself() {
        let (store, config, fields) = channel();
        let (stayer_id, leaver_id) = two_on_a_map(&store, 1);
        let mut stayer = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut leaver = Session::joining(store, config, fields.clone());
        stayer.claim_for_character(stayer_id);
        leaver.claim_for_character(leaver_id);
        stayer.on_field_entered();
        leaver.on_field_entered();
        assert_eq!(
            stayer.collect_mail().len(),
            1,
            "the control: the stayer was told the leaver arrived"
        );

        let mut chr = leaver.claimed_character().expect("a claimed character");
        leaver.go_to_map(&mut chr, 104_040_000, 0, "a server warp".to_string());

        let out = stayer.collect_mail();
        assert_eq!(out.len(), 1, "the stayer must be told, without the leaver's help: {out:?}");
        assert_eq!(out[0].opcode, net::userpool::USER_LEAVE_FIELD);

        // And the leaver really is gone from the old field, not merely announced as gone -
        // a later publish on map 1 must not reach them.
        fields.bus().publish(
            stayer.subscriber,
            crate::fields::FieldKey::world(1),
            Reply { opcode: 0x02B0, body: vec![1], what: "the stayer waved".into() },
            None,
        );
        assert!(
            leaver.collect_mail().is_empty(),
            "a departed player must not keep receiving the old map's traffic"
        );
    }

    /// **Entering the Cash Shop publishes a farewell too.**
    ///
    /// The shop is a different stage: the client tears the field down and the player is not
    /// standing anywhere. Without this they stay drawn on the map they left, frozen, until
    /// they come back - and if they log out from inside the shop, until the socket drops.
    #[test]
    fn entering_the_cash_shop_removes_the_player_from_the_map() {
        let (store, config, fields) = channel();
        let (stayer_id, shopper_id) = two_on_a_map(&store, 1);
        let mut stayer = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut shopper = Session::joining(store, config, fields.clone());
        stayer.claim_for_character(stayer_id);
        shopper.claim_for_character(shopper_id);
        stayer.on_field_entered();
        shopper.on_field_entered();
        assert_eq!(stayer.collect_mail().len(), 1, "the control: the shopper arrived");

        shopper.on_cash_shop_request(&[]);
        let out = stayer.collect_mail();
        assert_eq!(out.len(), 1, "the shopper must vanish from the map: {out:?}");
        assert_eq!(out[0].opcode, net::userpool::USER_LEAVE_FIELD);
    }


    /// **A player who has not moved is announced STANDING, not facing-right-doing-nothing.**
    ///
    /// `move_action` is `(action << 1) | facing`, and `0` is action 0 facing right - a value
    /// this client has never emitted in 28 134 archived elements. Sending it is why every
    /// remote player was drawn facing right whatever they were doing.
    ///
    /// Both halves again, because only the pair is a check: the reported stance is used when
    /// there is one, and the standing fallback when there is not.
    #[test]
    fn the_announced_stance_is_the_reported_one_or_standing() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Poser".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store, config, fields);
        s.claim_for_character(id);

        assert_eq!(
            s.remote_at().move_action,
            net::userpool::MOVE_ACTION_STANDING,
            "nobody has moved yet, so they are standing - NOT action 0 facing right"
        );
        assert_ne!(s.remote_at().move_action, 0, "0 is a real pose and the wrong one");

        // A reported stance wins, facing bit and all. 0x03 is action 1 facing LEFT.
        s.note_own_position(500, 395, Some(0x03));
        assert_eq!(s.remote_at().move_action, 0x03);
        assert_eq!(s.remote_at().move_action & 1, 1, "bit 0 is the facing, and it is left");

        // **A turn on the spot still updates.** The early return compares the stance as well
        // as the position, or a character that turned without walking would be announced with
        // its old facing - the same staleness this whole path exists to remove.
        s.note_own_position(500, 395, Some(0x02));
        assert_eq!(s.remote_at().move_action, 0x02, "turning without moving must register");

        // And an attack, which carries no stance, leaves it alone rather than clearing it.
        s.note_own_position(600, 395, None);
        assert_eq!(s.remote_at().move_action, 0x02, "None means unchanged, not unknown");
    }

    /// **A standing player is sent the foothold under them, not `0`.**
    ///
    /// `0` means "not on a foothold" and the client draws it as a mid-air pose. Every
    /// `0x0224` this server sent carried `0`, so on 2026-09-04 every existing player on a map
    /// appeared to a joining client to be **floating**. The owner saw it; the field's own doc had
    /// said `0` was fine because "the client resolves it itself", which was a guess.
    ///
    /// Both halves are asserted, because only the pair is a check: the lookup finds ground
    /// where there IS ground, and still answers `0` where there is not. A test that only
    /// covered the first would pass for a builder that returned a constant.
    #[test]
    fn a_player_standing_on_ground_is_sent_its_foothold() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        // map, id, x1, y1, x2, y2 - one flat platform from x=0 to x=800 at y=400.
        let footholds = crate::footholds::Footholds::parse("7, 42, 0, 400, 800, 400\n");
        let config = Arc::new(Config {
            footholds,
            ..Config::default()
        });
        let fields = Arc::new(Fields::new());

        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character {
            name: "Stander".to_string(),
            map_id: 7,
            ..Default::default()
        };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store, config, fields);
        s.claim_for_character(id);

        // Standing on the platform.
        s.last_position = Some((400, 400));
        assert_eq!(
            s.remote_at().foothold,
            42,
            "the id of the platform under them, not 0 - 0 is the floating pose"
        );

        // **The control.** Off the end of the only platform there is, `0` is the honest
        // answer and the one every remote player used to get.
        s.last_position = Some((5_000, 400));
        assert_eq!(
            s.remote_at().foothold,
            0,
            "no ground under them means no foothold, and that must still be sayable"
        );
    }

    /// **A connection that dies without logging out gives its mobs back.**
    ///
    /// This is the exit nobody takes deliberately: the socket drops, the client crashes, the
    /// process is killed. `go_to_map`, the channel change and the log out all release on the
    /// way past; a crash goes through none of them, and `Drop` is the only thing left.
    ///
    /// Without it the claims stay held by a `SessionId` that will never exist again. The mobs
    /// are permanently uncontrolled, no later client is ever granted them, and on screen they
    /// stand still forever - **with no error, no log line and nothing to grep for.** That
    /// silence is why this is worth a test rather than a comment.
    #[test]
    fn a_connection_that_dies_without_logging_out_releases_its_mobs() {
        let (store, config, fields) = channel();
        let held = |s: u64| fields.controllers().held_by(s);

        let ghost_id = {
            let ghost = Session::joining(store.clone(), config.clone(), fields.clone());
            let id = ghost.subscriber.get();
            // Claim three of map 7's mobs the way field entry does.
            let claimed = fields.controllers().claim_uncontrolled(crate::fields::FieldKey::world(7), id, &[2000, 2001, 2002]);
            assert_eq!(claimed.len(), 3, "the control: this session really holds them");
            assert_eq!(held(id), 3);
            id
            // `ghost` is dropped here - no log out, no channel change, no portal walk.
        };

        assert_eq!(
            held(ghost_id),
            0,
            "a dropped connection must not keep its mobs claimed by an id that is gone"
        );

        // And the mobs are genuinely free: the next connection can claim them, which is the
        // property that actually matters on screen.
        let next = Session::joining(store, config, fields.clone());
        let got = fields.controllers().claim_uncontrolled(crate::fields::FieldKey::world(7), next.subscriber.get(), &[2000, 2001, 2002]);
        assert_eq!(got.len(), 3, "the next player takes over all three: {got:?}");
    }

    /// **A shopper's mobs go back to the field too.**
    ///
    /// The Cash Shop is a different stage - the client tears the field down - so a controller
    /// sitting in it is a controller that is not driving anything. Field entry re-claims on
    /// the way back, so this costs the shopper nothing.
    #[test]
    fn entering_the_cash_shop_releases_the_mobs_it_was_driving() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character {
            name: "Shopper".to_string(),
            map_id: 1,
            ..Default::default()
        };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store, config, fields.clone());
        s.claim_for_character(id);
        s.on_field_entered();

        let sub = s.subscriber.get();
        fields.controllers().claim_uncontrolled(crate::fields::FieldKey::world(1), sub, &[3000, 3001]);
        assert_eq!(fields.controllers().held_by(sub), 2, "the control: two claimed");

        s.on_cash_shop_request(&[]);
        assert_eq!(
            fields.controllers().held_by(sub),
            0,
            "a shopper is not on the map, so the mobs it was driving must be free"
        );
    }

    /// **A pick-up is seen by the whole field, and cannot happen twice.** The owner, 2026-09-14:
    /// *"when one person picks up the drops, all other people that see the drops also see it
    /// being picked up by that person. There should be no duplicates of drops, multiple people
    /// cannot pick up the same drop."*
    #[test]
    fn a_pick_up_reaches_the_field_and_a_second_taker_gets_nothing() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Cobalt", "Tester2"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: 104_040_000,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        let mut picker = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut watcher = Session::joining(store, config, fields.clone());
        picker.claim_for_character(ids[0]);
        watcher.claim_for_character(ids[1]);
        picker.on_field_entered();
        watcher.on_field_entered();
        let _ = picker.tick(1_000);
        let _ = watcher.tick(1_000);

        // A meso drop owned by the picker, placed straight into the shared field so the test
        // is about pick-up visibility and not about combat's distribution walk.
        let (drop_id, _enter) = fields.with_drops(crate::fields::FieldKey::world(104_040_000), |d| {
            d.drop_from_mob(crate::drops::DropFromMob {
                from_mob: true,
                map_id: crate::fields::FieldKey::world(104_040_000),
                owner_id: ids[0],
                item: store::Item::bundle(0, 0),
                inv_type: store::InventoryType::Etc,
                meso: 10,
                x: 0,
                y: 0,
                source_x: 0,
                source_y: 0,
                now_ms: 1_000,
                party_id: 0,
            })
        });

        let mut body = vec![0u8; 34];
        body[crate::drops::PICK_UP_OBJECT_ID_AT
            ..crate::drops::PICK_UP_OBJECT_ID_AT + 4]
            .copy_from_slice(&drop_id.to_le_bytes());

        // The picker takes it and is told directly.
        let picked = picker.on_pick_up(0x032C, &body);
        assert!(
            picked.iter().any(|r| r.opcode == net::drops::DROP_LEAVE_FIELD),
            "the picker is sent the leave: {picked:?}"
        );

        // The watcher hears the SAME leave over the bus on its next tick, with the picker's id.
        let mail = watcher.tick(2_000);
        let leaves: Vec<&Reply> =
            mail.iter().filter(|r| r.opcode == net::drops::DROP_LEAVE_FIELD).collect();
        assert_eq!(leaves.len(), 1, "the other player sees it leave once: {mail:?}");

        // And a second pick-up of the same id - by either player - finds nothing. The drop
        // left the shared table when the picker took it, so no duplicate credit is possible.
        let again = watcher.on_pick_up(0x032C, &body);
        assert!(
            again.iter().all(|r| r.opcode != net::drops::DROP_LEAVE_FIELD),
            "a drop already taken cannot be taken again: {again:?}"
        );
        assert_eq!(
            fields.with_drops(crate::fields::FieldKey::world(104_040_000), |d| d.get(drop_id).is_some()),
            false,
            "the drop is gone from the shared field"
        );
    }

    /// **An expired drop fades on EVERY screen it was on, whoever's tick swept it.** The owner,
    /// 2026-09-18: *"When a party loot expires for the client that killed the monster, other
    /// clients in the party who share the visual for that drop do not see the expired drop
    /// disappear ... If anyone drops items publicly and that item disappears, it should
    /// disappear for everyone who can see it."*
    ///
    /// Before this the sweep addressed the fade to the drop's owner alone. Here the NON-owner
    /// is the session that ticks past the lifetime, so its own sweep removes both drops - a
    /// party drop owned by the other player and that player's public meso drop - and both
    /// fades must still reach both mailboxes. One tick, two `0x046F`s each, leave type FADE.
    #[test]
    fn an_expired_drop_fades_for_everyone_on_the_map_not_only_its_owner() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Cobalt", "Tester2"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: 104_040_000, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        let mut owner = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut watcher = Session::joining(store, config, fields.clone());
        owner.claim_for_character(ids[0]);
        watcher.claim_for_character(ids[1]);
        owner.on_field_entered();
        watcher.on_field_entered();
        let _ = owner.tick(1_000);
        let _ = watcher.tick(1_000);

        let (party_drop, _) = fields.with_drops(crate::fields::FieldKey::world(104_040_000), |d| {
            d.drop_from_mob(crate::drops::DropFromMob {
                from_mob: true,
                map_id: crate::fields::FieldKey::world(104_040_000),
                owner_id: ids[0],
                item: store::Item::bundle(4_000_000, 1),
                inv_type: store::InventoryType::Etc,
                meso: 0,
                x: 0,
                y: 0,
                source_x: 0,
                source_y: 0,
                now_ms: 1_000,
                party_id: 7,
            })
        });
        let public_drop = fields
            .with_drops(crate::fields::FieldKey::world(104_040_000), |d| {
                d.drop_money(crate::drops::DropMoneyOnGround {
                    map_id: crate::fields::FieldKey::world(104_040_000),
                    character_id: ids[0],
                    meso: 50,
                    x: 0,
                    y: 0,
                    from_x: 0,
                    from_y: 0,
                    now_ms: 1_000,
                })
            })
            .object_id;
        assert_eq!(fields.with_drops(crate::fields::FieldKey::world(104_040_000), |d| d.len()), 2);

        // Only the watcher ticks past the lifetime: its sweep is the one that removes them.
        // `tick` collects mail BEFORE it sweeps, so the fades it posted to its own mailbox come
        // out on the tick after.
        let late = 1_000 + crate::drops::DROP_LIFETIME_MS + 1;
        let _ = watcher.tick(late);
        assert_eq!(fields.with_drops(crate::fields::FieldKey::world(104_040_000), |d| d.len()), 0, "the watcher's sweep removed both");
        let mail = watcher.tick(late + 50);
        let fades = |mail: &[Reply]| -> Vec<u32> {
            mail.iter()
                .filter(|r| r.opcode == net::drops::DROP_LEAVE_FIELD && r.body[4] == net::drops::leave_type::FADE)
                .map(|r| u32::from_le_bytes([r.body[0], r.body[1], r.body[2], r.body[3]]))
                .collect()
        };
        let mut seen = fades(&mail);
        seen.sort_unstable();
        let mut expected = vec![party_drop, public_drop];
        expected.sort_unstable();
        assert_eq!(seen, expected, "the non-owner who swept sees both fade: {mail:?}");

        // The owner, who did nothing, reads the same two fades out of its mailbox.
        let mail = owner.tick(late + 100);
        let mut seen = fades(&mail);
        seen.sort_unstable();
        assert_eq!(seen, expected, "the owner sees both fade too: {mail:?}");

        // And nobody is told twice.
        assert!(fades(&watcher.tick(late + 200)).is_empty());
        assert!(fades(&owner.tick(late + 200)).is_empty());
    }
}
