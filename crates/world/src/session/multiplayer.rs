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
    pub(super) fn hand_over_mobs(&mut self, map: u32) {
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

        net::userpool::RemoteAt { x, y, move_action, foothold }
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
    fn presence(&self, chr: &net::opcode::Character) -> crate::broadcast::Presence {
        crate::broadcast::Presence {
            character: chr.id,
            map: chr.map_id,
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

    /// **Tell the field this character's hair or face just changed.** No field re-entry.
    ///
    /// The owner, 2026-09-12: *"The hair should just switch immediately on screen without a
    /// reload... The moment any hair or face change happens, it should also show up on other
    /// clients."* The player's own client already applied the look - the Beauty dialog
    /// previews it and commits it on Confirm - so the server owes only the OTHER clients an
    /// update. `0x0138` (the incremental avatar-modify) applies nothing in this client
    /// (`research/naked-character.md`), so the update rides the user-pool enter packet the
    /// field already uses: everyone here re-adds this character with its new look, and the
    /// stored spawn is refreshed so a later joiner gets it too.
    ///
    /// **[I] on the remote side**: whether a second `USER_ENTER_FIELD` for a character
    /// already in the pool redraws it or is ignored is not measured. It is the only live
    /// avatar mechanism this client has; the plan's step names the falsifier.
    pub(super) fn broadcast_look_change(&mut self, chr: &net::opcode::Character) {
        let Some(map) = self.bus().map_of(self.subscriber) else { return };
        let spawn = self.presence(chr).spawn;
        self.bus().refresh_spawn(self.subscriber, spawn.clone());
        self.bus().publish(self.subscriber, map, spawn, None);
        crate::server::log(&format!(
            "   look change for character {} (hair {}, face {}) broadcast to field {map}",
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

    fn presence(character: u32, map: u32) -> Presence {
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
        let map = 104_040_000;
        let make = |name: &str| {
            let chr = net::opcode::Character { name: name.to_string(), map_id: map, ..Default::default() };
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

        fields.bus().enter_field(watcher.subscriber, presence(200, 7));
        fields.bus().enter_field(mover.subscriber, presence(201, 7));
        assert_eq!(watcher.collect_mail().len(), 1, "the arrival of 201");

        fields.bus().publish(
            mover.subscriber,
            7,
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
        fields.bus().enter_field(watcher.subscriber, presence(200, 7));
        fields.bus().enter_field(mover.subscriber, presence(201, 7));
        let _ = watcher.collect_mail();

        fields.bus().publish(
            mover.subscriber,
            7,
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
        fields.bus().enter_field(watcher.subscriber, presence(200, 7));

        {
            let leaver = Session::joining(store, config, fields.clone());
            fields.bus().enter_field(leaver.subscriber, presence(201, 7));
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
        fields.bus().enter_field(watcher.subscriber, presence(200, 7));

        {
            let mut leaver = Session::joining(store, config, fields.clone());
            fields.bus().enter_field(leaver.subscriber, presence(201, 7));
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
            "the new map must not inherit the old map's coordinates"
        );
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
            1,
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
            let claimed = fields.controllers().claim_uncontrolled(7, id, &[2000, 2001, 2002]);
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
        let got = fields.controllers().claim_uncontrolled(7, next.subscriber.get(), &[2000, 2001, 2002]);
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
        fields.controllers().claim_uncontrolled(1, sub, &[3000, 3001]);
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
        let (drop_id, _enter) = fields.with_drops(104_040_000, |d| {
            d.drop_from_mob(crate::drops::DropFromMob {
                from_mob: true,
                map_id: 104_040_000,
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
            fields.with_drops(104_040_000, |d| d.get(drop_id).is_some()),
            false,
            "the drop is gone from the shared field"
        );
    }
}
