//! **One mob, one simulation, two screens.** Who controls each monster, which mob packets
//! are map-wide and which are private, and who gets to see the loot.
//!
//! Labels are the project's: **[L]** read off this client's listing or a capture, **[D]**
//! derived from two or more [L] facts, **[I]** inferred - policy nothing on this machine can
//! confirm.
//!
//! # WIRED 2026-09-01. This header used to say the opposite.
//!
//! It read *"Nothing here is wired"*, and that was true on the day it was written.
//! `crate::fields::Fields` now owns a [`Controllers`], `session/field.rs` claims on field
//! entry and releases on the way out, `session/combat.rs` gates every `0x02FF` on the
//! registry and publishes `0x03C6`, `0x03D9`, `0x03F0` and `0x03D1` to the map, and the
//! drops go to [`drop_audience`]'s winner over `Bus::publish_to_character`.
//!
//! **Left as a correction rather than deleted**, for the reason `crate::drops`'s own header
//! gives: a stale "not wired" banner is its own hazard, because it invites the next reader to
//! wire something twice. §"WHERE IT IS WIRED" at the bottom is the map, and
//! `research/mob-share.md` is the working.
//!
//! This module itself is still the **decision** and nothing else, the same shape as
//! [`crate::taxi`] and [`crate::secondjob`]: it sends no packets, touches no database, and
//! knows nothing about `Session`, so every branch below is a unit test rather than a client
//! run.
//!
//! **Two clients have never been connected to this server at once.** Every claim below about
//! what a second player *sees* is [I] on that point, however well-read the packet is - and
//! that has not changed by wiring it. What the tests prove is that the server does what it
//! was told to do.
//!
//! # The problem, measured rather than suspected - all four of these are now fixed
//!
//! `crates/world/src/session/field.rs` granted `MOB_CHANGE_CONTROLLER` for **every** mob to
//! **every** arriving session, and `crates/world/src/fields.rs` had no controller registry at
//! all. `research/mob-behaviour.md` §5.1 reads the wander out of the client: mob vtable slot
//! 19 `FUN_141c8d1b0` builds a list of 12-byte path elements out of the random source
//! `FUN_142f04924`, **once per path element, in the controlling client**, and hands it to the
//! move builder. **[D]** So two grants are two dice rolls, and the two screens diverge on the
//! first step and never reconverge.
//!
//! Three further consequences of the same missing registry, all read out of this repo's own
//! code rather than guessed, and each with the line that fixed it:
//!
//! * `Fields::due_respawns` **drains** `field.pending`. With two sessions ticking, whichever
//!   ticks first took the new mobs and the other session was never told they exist - a
//!   respawn was **unicast**. **[L]** from `fields.rs`. Fixed by publishing the `0x03C6` in
//!   `session/combat.rs::spawn_due_mobs`; the drain itself is untouched and does not need to
//!   change, because the mob is in the shared field either way.
//! * `Fields::note_position` was written by every `0x02FF` from every session. Two controllers
//!   are two writers of one field, last write wins, and the position a drop lands on was
//!   whichever client reported most recently. **[L]** from `fields.rs` and
//!   `session/combat.rs`. Fixed by `Fields::note_position_from`, which asks
//!   [`may_report_movement`] **at the row** rather than at the call site.
//! * `DropTable::sweep` removed an expired drop from the shared table and returned the
//!   `0x046F` to the **calling** session only, so a second player kept drawing an item that no
//!   longer existed. **[L]** from `drops.rs`. Fixed by `drops::Addressed`: the fade names its
//!   owner and `Fields::with_drops` posts it.
//! * And a fourth, found while wiring: `DropTable::field_entry` re-sent the **whole floor** to
//!   whoever walked in, which is a leak the moment drops are owner-scoped. Fixed by
//!   [`may_see_drop`] inside `field_entry`.
//!
//! # The model, in one sentence
//!
//! **Exactly one connection controls each mob, at every instant.** Control is claimed by a
//! session for itself on field entry, and afterwards moves only on two events: somebody hits
//! a mob they do not hold, and a holder leaves the map.
//!
//! > **RETRACTED 2026-09-04.** This said control *"is never revoked while its holder is still
//! > on the map, because the client's only revoke is a despawn"*. `CONTROL_RELEASE` does not
//! > despawn a live mob; the erase behind it is guarded twice and a mob that entered the
//! > field normally never reaches it. The owner said so and the listing agrees. The single owner
//! > of that fact is [`net::mobmove::CONTROL_RELEASE`]; the working is
//! > `research/control-release-does-it-despawn.md`. **This module states no version of it -
//! > flattening that claim into a second place is exactly what cost a revert.**
//!
//! ## Why control moves only on a hit or a departure
//!
//! Rotation IS available - `CONTROL_RELEASE` releases - but it is not free: each change is a
//! release to the old holder and a grant to the new one, **in that order**. Granting first
//! leaves two clients past their run gate, both rolling independent wanders, and the mob
//! visibly jumps between them. So control changes on the two events that need it: somebody
//! hits a mob they do not hold, and a holder leaves the map.

//!
//! So **this server never sends `CONTROL_RELEASE` at all.** A controller loses its mobs only
//! by leaving the field, and a client that leaves a field has already torn its own mob pool
//! down - the `SetField` does it - so the transfer costs zero packets on the losing side.
//!
//! ## Almost every transition is a session claiming for itself
//!
//! [`Controllers::claim_uncontrolled`] is a compare-and-set under one lock. A session calls
//! it for **itself**, on its own map, in its own thread:
//!
//! | when | what it claims | what it sends |
//! |---|---|---|
//! | field entry | every uncontrolled mob on the map | `0x03C6` for **all** mobs, `0x03D2` for the claimed ones |
//! | respawn tick | the mobs its own `due_respawns` returned | `0x03C6` + `0x03D2` to itself, `0x03C6` published to the map |
//! | any later tick | anything orphaned since | `0x03D2` for the newly claimed |
//! | leaving, somebody left | gives them away - [`Controllers::hand_over`] | `0x03D2` **to the successor**, nothing to the leaver |
//! | leaving, nobody left | nothing - [`Controllers::release_map`] | **nothing.** There is no one to tell |
//!
//! ### The departure row is the one exception, and it was added because the rule was wrong
//!
//! This section used to end: *"no control packet is ever addressed to a connection other than
//! the one building it, so no new bus channel is needed for control, and the two-controllers
//! bug cannot be reintroduced by a routing mistake."* Cheap, and true of everything above the
//! last two rows. The cost was written down honestly right beside it - *"one tick of latency
//! when a controller departs ... nobody has watched it"* - and it was **not one tick.**
//!
//! Nothing in `tick` claims orphans. `claim_uncontrolled` runs on **field entry**, which a
//! player standing still never performs. So the mobs a departing controller left behind stood
//! motionless until somebody walked through a portal and came back, and the test that was
//! supposed to cover it called `on_field_entered` a second time to make the grants appear -
//! which is exactly the thing a standing player does not do.
//!
//! The owner, 2026-09-01: *"If the person who is controlling the movement of the mob leaves the map,
//! then the mob should not disappear. That's a jarring experience. The control of the mob
//! should be handed over to another client who is still present in the map."* They were describing
//! a worse symptom than the one that was there - they never disappeared - and was right about
//! the fix.
//!
//! So there is now exactly one control packet addressed to somebody else: the `0x03D2` a
//! departure sends to its successor. The two-controllers bug it was protecting against is
//! still impossible, and for a stronger reason than the routing rule: [`Controllers::hand_over`]
//! is a re-assignment under **one lock**, so there is no instant at which two ids hold the mob
//! and no instant at which none does.
//!
//! The general lesson is the cheaper one. **A cost written down as an estimate is a claim.**
//! "One tick" was never measured, and the number was not out by a factor - it was the wrong
//! quantity, because nothing was scheduled to pay it.
//!
//! ## What a mob with no controller is
//!
//! Alive, drawn on every screen that received its `0x03C6`, and standing still. Nothing
//! reports a position, so `Fields::note_position` stops being written and `LiveMob::as_seen`
//! keeps handing out the last place it was seen - which is exactly the behaviour a map with
//! nobody on it already has, and the owner's model for it: *"When the user leaves the map, all of
//! the mobs should persist in their current location."*
//!
//! # Which packets are map-wide, and which are private
//!
//! [`audience_for`] is the whole table, in one place, defaulting to **private** so an opcode
//! nobody has thought about cannot leak. The dangerous entry is `0x03D2`: fanning a
//! `MOB_CHANGE_CONTROLLER` out to the map would grant two clients control of one mob, which
//! is the bug this module exists to remove. `mob_change_controller_is_never_map_wide` pins
//! it.
//!
//! ## An unknown object id is safe on the map-wide ones - checked, not assumed
//!
//! A published `0x03F0`, `0x03D9` or `0x03D1` can reach a client whose mob pool has never
//! held that object - a player who joined mid-fight, or a duplicate from the narrow race in
//! §"WHERE IT IS WIRED". All three were read today with `tools/listing.py`, whose documented
//! positive control (`0x140304100` -> `raw`, `u8`, `u8`, then a run of `u16`) was run first
//! and passed:
//!
//! ```text
//! 0x03F0, 0x03D9  second dispatcher FUN_141d32b30
//!   141d32b4d  READ u32                  the object id
//!   141d32b57  call 0x141d2efc0          the pool lookup
//!   141d32b62  je   0x141d33432          not found -> straight to the epilogue
//!   141d33432  lea r11,[rsp+0x70] ... ret
//!
//! 0x03D1          FUN_141d33c70, which reads the WHOLE body first
//!   141d33d09  je   0x141d34254          bucket array null
//!   141d33d26  je   0x141d34254          bucket null
//!   141d33d3f  jmp  0x141d34254          chain walked out
//!   141d34254  mov rbx,[rsp+0xa8] ... ret
//! ```
//!
//! **[L]** for all three. An unknown object id costs a body read and a return; it does not
//! fault and it does not desynchronise the stream.
//!
//! # Drops: per client, to the top damager, and the party seam
//!
//! The owner, 2026-09-01: *"All clients need to see other clients damages to mobs, but the drops
//! can remain per client. If multiple clients hit the mob, the one who dealt the most damage
//! (without counting over-damage) will see the drops. All members of a party should see all
//! drops killed by members of the party."*
//!
//! **"Most damage without counting over-damage" already exists and is not rebuilt here.**
//! `LiveMob::credit` does `let landed = damage.min(self.hp);` before subtracting, and
//! `LiveMob::shares()` returns the split highest-first with exactly one `majority`, ties
//! broken by who hit first. That is the same ranking the EXP split and its white/yellow line
//! already use. [`drop_audience`] reads it and adds nothing to it.
//!
//! **The client cannot help.** `net::drops`: `ownType` is read at `0x1417a3539` into
//! `drop+0x70` and **never tested again**, so visibility is entirely a question of who the
//! server sends `0x046E` to. **[L]** [`own_type_for`] exists so the byte is not a lie, and
//! that is all it is for.
//!
//! **The seam is [`may_see_drop`].** One predicate, one [`Party`], and the party rule is a
//! one-line change at the call site that builds the `Party` - not a change in here and not a
//! change in `drops.rs`. There is no party system today and this module does not build one;
//! [`Party::solo`] is the whole of the current rule.
//!
//! # What this module deliberately does NOT do
//!
//! * It does not roll damage, EXP or drops. `Fields`, `crate::expcurve` and
//!   `crate::droptables` own those and are correct.
//! * It does not build a packet. Everything it returns is an id, a boolean or an
//!   [`Audience`].
//! * It does not decide the **owner lock**. `drops::OWNER_LOCK_MS` says a drop may be taken
//!   by anyone after 15 seconds; with visibility restricted to the audience, that window is
//!   **unobservable** - a player who may take a drop they cannot see will never ask for it.
//!   Said out loud rather than silently made dead, because a rule that quietly stops applying
//!   is exactly the shape `CLAUDE.md` warns about. Aligning the two is a `drops.rs` decision
//!   and `drops.rs` is not this agent's file.

use std::collections::HashMap;
use std::sync::Mutex;

use crate::fields::DamageShare;

/// Which connection controls a mob.
///
/// The raw `u64` out of `crate::broadcast::SubscriberId::get()`, not a character id, and the
/// difference is load-bearing: control is a property of a **client's mob pool**, so a
/// character who reconnects is a different controller with an empty pool. Taken as a plain
/// integer so this module depends on nothing and every branch is a unit test.
pub type SessionId = u64;

// ---------------------------------------------------------------------------------------
// The registry
// ---------------------------------------------------------------------------------------

/// **Exactly one connection controls each mob.** One per channel, beside
/// [`crate::broadcast::Bus`].
///
/// Interior mutability for the same reason `Bus` has it: the channel hands every session one
/// `&Fields`, and a registry reached through it must be usable from `&self`. Its lock is its
/// own and is a **leaf** - nothing in here calls back into `Fields`, `Bus` or a session - so
/// it cannot deadlock against either. The wiring must still never take it while holding
/// `Fields`'s own map lock; see §"WHERE IT IS WIRED".
///
/// # The invariant, and where it is actually enforced
///
/// "One controller per mob" is not a comment: it is the fact that
/// [`Controllers::claim_uncontrolled`] and [`Controllers::claim_one`] are a **test and set
/// inside one lock**. Two sessions racing to claim the same mob serialise, and the loser is
/// told it lost by getting the id back in nobody's list.
/// `two_sessions_racing_for_the_same_mobs_split_them_and_never_share_one` runs that race.
#[derive(Debug, Default)]
pub struct Controllers {
    /// `map -> object id -> the connection that controls it`.
    inner: Mutex<HashMap<u32, HashMap<u32, SessionId>>>,
}

impl Controllers {
    pub fn new() -> Self {
        Self::default()
    }

    /// **Claim every mob in `candidates` that nobody controls**, and return the ones claimed.
    ///
    /// Purely additive: it never takes a mob away from another session, and it never removes
    /// an entry. `candidates` may safely be a subset of what is on the map. Anything
    /// destructive is [`Controllers::reconcile`], which is a separate call on purpose - see
    /// the note there.
    ///
    /// Returns the ids in `candidates` order, so the packets a caller builds from it come out
    /// in a stable order run to run.
    pub fn claim_uncontrolled(
        &self,
        map: u32,
        session: SessionId,
        candidates: &[u32],
    ) -> Vec<u32> {
        let mut inner = self.lock();
        let held = inner.entry(map).or_default();
        let mut claimed = Vec::new();
        for object_id in candidates {
            if held.contains_key(object_id) {
                continue;
            }
            held.insert(*object_id, session);
            claimed.push(*object_id);
        }
        claimed
    }

    /// **Drop every entry on `map` for a mob that is not in `alive`.** Returns how many.
    ///
    /// `alive` must be **every** object id currently on that map - `Fields::mobs_on(map)`.
    /// Belt and braces beside [`Controllers::forget`]: a registry that can only be corrected
    /// by a call at the exact moment of death goes stale the first time a death path changes,
    /// and a stale entry is a mob nobody will ever claim, which reads on screen as one
    /// monster that has quietly stopped moving.
    ///
    /// # Why this is not folded into `claim_uncontrolled`
    ///
    /// It was, for about ten minutes, and its own unit test caught why it should not be. A
    /// caller that passes a **partial** `alive` - a plausible mistake, and one nothing in the
    /// type system prevents - silently frees every *other* session's mobs on that map, and
    /// the symptom would be another player's monsters freezing on a screen nobody is looking
    /// at. A destructive step belongs where the caller can see it, which is `CLAUDE.md`'s
    /// rule about a guard whose answer is ignored, pointed the other way.
    ///
    /// A non-zero return is itself a finding: it means a `forget` was missed on some death
    /// path. Log it rather than discarding it.
    pub fn reconcile(&self, map: u32, alive: &[u32]) -> usize {
        let mut inner = self.lock();
        let Some(held) = inner.get_mut(&map) else { return 0 };
        let before = held.len();
        held.retain(|object_id, _| alive.contains(object_id));
        before - held.len()
    }

    /// Claim **one** mob, for a mob that has just spawned.
    ///
    /// `true` if this session now controls it. `false` means somebody else already does, and
    /// the caller must then not send a `0x03D2` - see [`audience_for`] on why a second grant
    /// is the one mistake this whole module exists to prevent.
    ///
    /// Separate from [`Controllers::claim_uncontrolled`] because a respawn has no `alive`
    /// list to reconcile against: the mob was inserted by `Fields::due_respawns` a moment ago
    /// and reconciling here against a stale snapshot would delete it again.
    pub fn claim_one(&self, map: u32, object_id: u32, session: SessionId) -> bool {
        let mut inner = self.lock();
        let held = inner.entry(map).or_default();
        match held.get(&object_id) {
            Some(other) => *other == session,
            None => {
                held.insert(object_id, session);
                true
            }
        }
    }

    /// Who controls this mob, if anyone.
    pub fn controller_of(&self, map: u32, object_id: u32) -> Option<SessionId> {
        self.lock().get(&map).and_then(|m| m.get(&object_id)).copied()
    }

    /// **Does `session` control this mob?**
    ///
    /// The gate on an inbound `0x02FF`. A report from a connection that is not the controller
    /// must not move the mob and must not be acknowledged: acknowledging it would pump a
    /// second simulation, which is the divergence this module removes.
    pub fn controls(&self, map: u32, object_id: u32, session: SessionId) -> bool {
        self.controller_of(map, object_id) == Some(session)
    }

    /// The mob is gone - it died, or left the field.
    ///
    /// Not required for correctness ([`Controllers::claim_uncontrolled`] reconciles) but
    /// called at the death anyway, so the count in a log line means what it says.
    pub fn forget(&self, map: u32, object_id: u32) {
        if let Some(held) = self.lock().get_mut(&map) {
            held.remove(&object_id);
        }
    }

    /// **Every map on which `session` still controls something**, lowest first.
    ///
    /// [`Controllers::release_all`] takes no map because it does not need one. A *handover*
    /// does: the successor has to be somebody standing on the same map, so the caller has to
    /// ask this question one map at a time. A crashed connection is the case that makes it
    /// more than one row - it can be holding a map it walked away from, if the walk itself is
    /// what killed it.
    ///
    /// Sorted so a handover is reproducible and a test can name the order.
    pub fn maps_held_by(&self, session: SessionId) -> Vec<u32> {
        let inner = self.lock();
        let mut maps: Vec<u32> = inner
            .iter()
            .filter(|(_, held)| held.values().any(|who| *who == session))
            .map(|(map, _)| *map)
            .collect();
        maps.sort_unstable();
        maps
    }

    /// **This connection is gone.** Free everything it held; returns how many.
    ///
    /// Every map, not one, because the caller is often `Drop` and has no map to hand -
    /// exactly the reason `Bus::part` takes no map either. Idempotent: a session that left
    /// its field cleanly and is then dropped calls this a second time and frees nothing.
    ///
    /// **No packet follows**, and here that is right for a reason unrelated to what a release
    /// does: a client that has left a field has already destroyed its own mob pool, so there
    /// is nobody to tell.
    pub fn release_all(&self, session: SessionId) -> usize {
        let mut inner = self.lock();
        let mut freed = 0;
        for held in inner.values_mut() {
            let before = held.len();
            held.retain(|_, who| *who != session);
            freed += before - held.len();
        }
        freed
    }

    /// Free everything this connection holds **on one map**, for a portal walk.
    ///
    /// [`Controllers::release_all`] would do as well and is what `Drop` uses; this exists so
    /// a walk between two maps cannot free mobs on a third one it was never on.
    /// **Move every mob `from` controls on `map` to `to`**, and say which moved.
    ///
    /// The owner, 2026-09-01: *"If the person who is controlling the movement of the mob leaves the
    /// map, then the mob should not disappear. That's a jarring experience. The control of the
    /// mob should be handed over to another client who is still present in the map."*
    ///
    /// They are right, and the symptom was worse than the one they named. The mobs never
    /// disappeared - [`release_map`] frees the claim and leaves the mob alive - but **nobody
    /// was told**, so they stood perfectly still on every remaining screen until somebody
    /// walked through a portal and came back. A frozen monster reads as a broken server more
    /// readily than a missing one does.
    ///
    /// # This is a re-assignment, not a revoke followed by a claim
    ///
    /// One call under one lock, because the two-step version has a window in which the mob
    /// belongs to nobody, and the whole point of the registry is that such a window does not
    /// exist. The caller still has to send `to` its grants; that is [`Controllers`]'s boundary
    /// - it knows who controls what and nothing about packets.
    ///
    /// **No packet goes to `from`.** Not because a release would be harmful - it would not,
    /// see [`net::mobmove::CONTROL_RELEASE`] - but because `from` has left the field and torn
    /// its own mob pool down, so the packet would name an object it no longer holds.
    /// **Give ONE mob to `to`, whoever held it.** Returns whether it changed hands.
    ///
    /// Returns `None` when `to` already held it - the common case in a fight, and it must
    /// cost no packet. Otherwise `Some(previous)`, and **the previous holder has to be told**:
    /// `net::mobmove::mob_release_controller`, sent to them BEFORE the grant goes to `to`.
    ///
    /// # This was reverted once, for a reason that turned out to be wrong
    ///
    /// Its first caller handed a mob to whoever hit it and made mobs teleport, because
    /// nothing told the old holder and two clients simulated one mob. The conclusion drawn -
    /// that control must never rotate while its holder is present - rested on
    /// `CONTROL_RELEASE` despawning, which **it does not**. The owner said so; the listing agrees.
    /// See `net::mobmove::CONTROL_RELEASE` for the retraction and
    /// `research/control-release-does-it-despawn.md` for the working.
    ///
    /// The single-mob twin of [`Controllers::hand_over`], and it exists because of what a
    /// flinch turns out to be.
    ///
    /// # Why an attacker has to own what it hits
    ///
    /// The owner, 2026-09-04: *"If the client that does not have mob control attacks a mob, the
    /// mob does not flinch and get pushed back."*
    ///
    /// **The flinch and the knockback are not a packet.** They are produced locally by the
    /// client that both swings *and* holds the mob's `0x03D2`, and they reach every other
    /// screen as that client's own `0x02FF` with the hit action - which this server already
    /// rebroadcasts as `0x03D9`. There is no `MobDamaged` opcode in this client, and none of
    /// the six mob opcodes that can set an action takes a damage value.
    ///
    /// Two archived runs measure it, same build and same map, differing only in who walked in
    /// first and therefore owns the mobs:
    ///
    /// ```text
    ///   controller swings      10 wounding hits -> 10 hit-action reports
    ///   non-controller swings  15 wounding hits ->  0
    /// ```
    ///
    /// `research/fixtures/controller-attacks-mob-flinches-10-of-10-world.log` and its
    /// `non-controller-...-0-of-15` sibling. It also follows from this module's own rule:
    /// [`Controllers::controls`] gates `0x02FF`, so a client that was never granted the mob
    /// cannot move it even if it wanted to.
    ///
    /// So the fix is ownership, not a new packet - and the packet that follows is the
    /// `0x03D2` this module already sends on field entry.
    ///
    /// # One lock, like its sibling
    ///
    /// There is no instant at which the mob belongs to nobody. Returns `false` when `to`
    /// already holds it, which is the common case in a fight and must cost no packet.
    pub fn hand_over_one(
        &self,
        map: u32,
        object_id: u32,
        to: SessionId,
    ) -> Option<Option<SessionId>> {
        let mut inner = self.lock();
        let held = inner.entry(map).or_default();
        match held.get(&object_id).copied() {
            Some(who) if who == to => None,
            previous => {
                held.insert(object_id, to);
                Some(previous)
            }
        }
    }

    pub fn hand_over(&self, map: u32, from: SessionId, to: SessionId) -> Vec<u32> {
        if from == to {
            return Vec::new();
        }
        let mut inner = self.lock();
        let Some(held) = inner.get_mut(&map) else { return Vec::new() };
        let mut moved: Vec<u32> = held
            .iter()
            .filter(|(_, who)| **who == from)
            .map(|(object_id, _)| *object_id)
            .collect();
        // Sorted so a handover is reproducible and a test can name the order of the grants.
        moved.sort_unstable();
        for object_id in &moved {
            held.insert(*object_id, to);
        }
        moved
    }

    pub fn release_map(&self, map: u32, session: SessionId) -> usize {
        let mut inner = self.lock();
        let Some(held) = inner.get_mut(&map) else { return 0 };
        let before = held.len();
        held.retain(|_, who| *who != session);
        before - held.len()
    }

    /// How many mobs this connection controls, across every map. For the log and for tests.
    ///
    /// A server that says "this connection controls 23 of the 30 mobs on map 104040000" at
    /// the moment it grants is the cheapest possible check that the split went where it was
    /// meant to, and this project's usual failure is a fan-out that looks fine because nobody
    /// counted the recipients.
    pub fn held_by(&self, session: SessionId) -> usize {
        self.lock().values().map(|m| m.values().filter(|w| **w == session).count()).sum()
    }

    /// How many mobs have a controller at all, across every map.
    pub fn len(&self) -> usize {
        self.lock().values().map(HashMap::len).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// A poisoned registry is not a reason to kill a channel: what is behind the lock is a
    /// map of integers, and the worst a panicking claimer can leave is a half-inserted entry,
    /// which is still a well-formed entry. Same call `Fields` and `Bus` already make.
    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<u32, HashMap<u32, SessionId>>> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }
}

// ---------------------------------------------------------------------------------------
// Who a packet is for
// ---------------------------------------------------------------------------------------

/// Who a mob or drop packet goes to, once the acting session has built it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Audience {
    /// **This connection only.** Either the packet grants something only one client may
    /// hold, or its contents are private to one player.
    ///
    /// The default for anything [`audience_for`] does not recognise, which is the safe
    /// direction: a packet that should have been shared and was not is a missing update, and
    /// a packet that should have been private and was not is a second controller or somebody
    /// else's loot.
    OnlyTheActingSession,
    /// **Everyone else on the map.** `Bus::publish` already excludes the sender, who is
    /// receiving the same bytes in its own reply.
    ///
    /// `supersedes` is `Bus::publish`'s fourth argument: `Some(k)` means a later packet with
    /// the same opcode and the same `k` replaces this one while it is still unsent.
    EveryoneElseOnTheMap { supersedes: Option<u32> },
}

impl Audience {
    /// Convenience for a wiring that only wants to know whether to publish.
    pub fn is_map_wide(&self) -> bool {
        matches!(self, Audience::EveryoneElseOnTheMap { .. })
    }

    /// `Bus::publish`'s supersede argument, or `None`.
    pub fn supersedes(&self) -> Option<u32> {
        match self {
            Audience::EveryoneElseOnTheMap { supersedes } => *supersedes,
            Audience::OnlyTheActingSession => None,
        }
    }
}

/// **The table: which mob packets are map-wide, and which are private.** One place, so a new
/// mob packet has to be added here deliberately rather than defaulting into a broadcast.
///
/// `object_id` is the mob or drop the packet is about; it is used only as a supersede key.
///
/// | opcode | | audience | why |
/// |---|---|---|---|
/// | `0x03C6` | MobEnterField | map-wide | a respawn has to reach everyone standing there, and today it reaches whichever session ticked first |
/// | `0x03D1` | MobLeaveField | map-wide | *"the mob does not die on A's screen"* |
/// | `0x03F0` | MobHpChange | map-wide | *"A's HP bar does not move"* |
/// | `0x03D9` | MobMove | map-wide, **superseded** | the only thing that moves a mob on a non-controller's screen |
/// | `0x03D2` | MobChangeController | **private** | two grants are two simulations. This is the bug |
/// | `0x03E4` | MobCtrlAck | **private** | it echoes one client's own move counter |
/// | `0x046E` | DropEnterField | **private** | The owner: *"the drops can remain per client"* |
/// | `0x046F` | DropLeaveField | **private** | it must reach exactly the clients that were sent the `0x046E` |
///
/// # Only `0x03D9` supersedes, and the reason is a bound rather than a saving
///
/// `Bus::publish`'s supersede slot is documented as *"the character the packet is about"*,
/// and a mob object id is not a character id. The two namespaces overlap - character ids
/// start at 200 and mob object ids at 2000 - so this is a real question rather than a
/// pedantic one. It is safe **because the bus matches on the pair `(opcode, key)`** and
/// `0x03D9` is a mob-only opcode that no character-keyed packet can ever share. That
/// argument has to hold for every opcode given a key here, which is why only one is.
///
/// A mob wanders for as long as it is alive, so an observer who stops reading accumulates
/// `0x03D9` **without bound**; one pending position per mob is the right amount to keep, and
/// a superseded position is worth nothing to anybody. `0x03F0` is bounded by the length of a
/// fight, arrives beside a `0x029E` remote-attack packet that is already unsuperseded, and
/// carries an event as well as a state - `research/mob-hp-bar.md` and the doc on
/// `net::combat::mob_hp_change`: the same packet pushes into the mob's floating-number list
/// at `mob+0x6d8`. Superseding it would drop numbers. So it does not.
///
/// **This is a decision, not a measurement.** Both readings are defensible and neither has
/// been on a screen.
pub fn audience_for(opcode: u16, object_id: u32) -> Audience {
    match opcode {
        net::mobmove::MOB_MOVE => {
            Audience::EveryoneElseOnTheMap { supersedes: Some(object_id) }
        }
        net::mob::MOB_ENTER_FIELD
        | net::combat::MOB_LEAVE_FIELD
        | net::combat::MOB_HP_CHANGE => Audience::EveryoneElseOnTheMap { supersedes: None },
        _ => Audience::OnlyTheActingSession,
    }
}

/// **May this connection's `0x02FF` be believed?**
///
/// The gate on `Session::on_mob_move`. `false` means: do not call `Fields::note_position`, do
/// not send a `0x03E4`, and do not publish a `0x03D9` - the report came from a client that
/// does not own this mob's simulation.
///
/// Today that can only happen two ways, and both are worth a log line rather than silence:
/// a grant left over from before this module existed, or a client sending a report it was
/// never invited to send. Nothing on this socket authenticates anybody.
///
/// A mob with **no** controller is also refused. It cannot legitimately be reporting - a
/// mob's move sender is only reached once slot 8 has been switched on by a `0x03D2`
/// (`research/mob-behaviour.md` §4) - so a report for one is either stale or invented.
pub fn may_report_movement(
    controller: Option<SessionId>,
    reporting: SessionId,
) -> bool {
    controller == Some(reporting)
}

// ---------------------------------------------------------------------------------------
// Drops
// ---------------------------------------------------------------------------------------

/// Who, besides the owner, may see a drop.
///
/// **The party seam, and the whole of it.** There is no party system in this server - a
/// sibling agent is designing one - and this module does not build one. Today every
/// [`Party`] is a [`Party::solo`], and when parties exist the *only* change is the call site
/// that builds this value: `Party::of(character)` instead of `Party::solo(character)`.
/// Nothing in [`may_see_drop`], in `drops.rs` or in `net::drops` moves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Party {
    /// Every character who counts as "us", the owner included. Never empty.
    members: Vec<u32>,
}

impl Party {
    /// A character in no party: the only rule this server has today.
    pub fn solo(character: u32) -> Self {
        Party { members: vec![character] }
    }

    /// A real party. **Nothing constructs this yet** - it exists so the seam is a value the
    /// party agent can produce rather than a signature it has to change.
    ///
    /// `character` is included whether or not the caller listed it, so a party that has
    /// forgotten its own member cannot hide a drop from its owner.
    pub fn of(character: u32, members: impl IntoIterator<Item = u32>) -> Self {
        let mut all = vec![character];
        for m in members {
            if !all.contains(&m) {
                all.push(m);
            }
        }
        Party { members: all }
    }

    pub fn members(&self) -> &[u32] {
        &self.members
    }

    /// Is this character one of us?
    pub fn contains(&self, character: u32) -> bool {
        self.members.contains(&character)
    }

    /// Is this a party at all, or one person?
    pub fn is_solo(&self) -> bool {
        self.members.len() == 1
    }
}

/// **Who should be shown a mob's drops, best candidate first.**
///
/// The owner: *"the one who dealt the most damage (without counting over-damage) will see the
/// drops."* `shares` is `LiveMob::shares()` exactly as `Fields::hurt` returned it - already
/// ranked highest-first, over-damage already excluded by `LiveMob::credit`'s
/// `damage.min(self.hp)`, ties already broken by who hit first. **This function adds no
/// arithmetic**, and that is the point: a second implementation of "most damage" is a second
/// thing to get wrong, and the first one drives the EXP split and its white/yellow line.
///
/// # Why a ranked list rather than one winner
///
/// The top damager may not be on the map any more - they hit it and walked through a portal
/// while somebody else finished it. An item nobody can see is indistinguishable from no item
/// at all, which is the same sentence `session/combat.rs` already uses about a drop out of
/// reach. So the caller walks this list and stops at the first candidate it can actually
/// deliver to on **this** map, and `killer` is the guaranteed terminator: they are on the
/// map by definition, having just swung.
///
/// `killer` is appended if it is not already present, which covers the mob that died without
/// anyone being credited - `Fields::hurt` returns `Died(vec![])` for that, and
/// `award_kill_experience` already treats it as "pay the killer in full".
///
/// # A tie
///
/// Broken by first blood, because `shares()` is a **stable** sort by damage descending. Two
/// players who each did exactly half give the drops to whoever hit it first, and the same
/// fight scores the same way twice. That is inherited, not decided here.
pub fn drop_audience(shares: &[DamageShare], killer: u32) -> Vec<u32> {
    let mut ranked: Vec<u32> = shares.iter().map(|s| s.character).collect();
    if !ranked.contains(&killer) {
        ranked.push(killer);
    }
    ranked
}

/// **May `viewer` see - and therefore pick up - a drop owned by `owner`?**
///
/// One predicate, and the party rule is `party`. See [`Party`].
///
/// The `owner` is whoever [`drop_audience`] settled on, which is *not* necessarily the
/// character who landed the killing blow.
pub fn may_see_drop(owner: u32, viewer: u32, party: &Party) -> bool {
    owner == viewer || (party.contains(owner) && party.contains(viewer))
}

/// The `ownType` byte to stamp on a `0x046E`.
///
/// **Cosmetic, and stated as such.** `net::drops`: the client reads `ownType` at
/// `0x1417a3539` into `drop+0x70` and never tests it again, so this byte changes nothing on
/// screen and enforces nothing. It is set truthfully anyway, because the next person to read
/// a capture will check it against the rule and a lie there costs an hour.
pub fn own_type_for(party: &Party) -> u8 {
    if party.is_solo() {
        net::drops::OWN_TYPE_USER
    } else {
        net::drops::OWN_TYPE_PARTY
    }
}

// ---------------------------------------------------------------------------------------
// WHERE IT IS WIRED
//
// This section used to be called WIRE IT LIKE THIS and was a plan. It is now a map, kept so
// that the next person reading `mobshare.rs` can find every call site without grepping, and
// so that the three places the plan turned out to be wrong are written down rather than
// rediscovered.
//
// | what | where |
// |---|---|
// | the registry | `fields.rs` - a `controllers` field and a `controllers()` accessor |
// | claim on arrival | `session/field.rs::on_field_entered` |
// | handover on departure | `session/multiplayer.rs::hand_over_mobs` / `hand_over_all_mobs` |
// | ...its callers | `session/field.rs`: `go_to_map`, `on_change_channel`, `on_log_out`; `session/cashshop.rs`; `Drop for Session` in `session/mod.rs` |
// | respawn | `session/combat.rs::spawn_due_mobs` - publish the `0x03C6`, `claim_one` the grant |
// | movement | `session/combat.rs::on_mob_move` - `note_position_from`, then publish `0x03D9` |
// | damage and death | `session/combat.rs::on_attack` - publish the same bytes, `forget` the dead |
// | drops | `session/combat.rs::drops_from_kill_for` - walk `drop_audience`, `Bus::publish_to_character` |
// | the floor | `drops.rs::field_entry` filters by `may_see_drop`; `drops.rs::sweep` addresses its fades |
//
// ---------------------------------------------------------------------------------------
// Three things the plan got wrong, which is the part worth keeping
//
// 1. **A field entry has to RELEASE this connection's own claims before it re-claims.** The
//    plan said `reconcile` then `claim_uncontrolled`, and both are purely additive with
//    respect to a session that already holds a mob - so a player returning to a map it
//    already controlled got `0x03C6` for every mob and `0x03D2` for none of them, and every
//    monster on that screen stood still for the rest of the session. The client destroys its
//    mob pool on every `SetField`, so every grant it held is void and has to be re-sent.
//    `release_map` on the way in is what makes field entry self-sufficient, and it covers the
//    paths a departure hook cannot - the Cash Shop return in particular, whose
//    `leave_the_field()` lives in a file this change did not own.
//    Pinned by `coming_back_to_a_map_this_connection_controls_re_sends_every_grant`.
//
// 2. **`DropTable` cannot deliver a fade, and the plan put the fix in `session/mod.rs::tick`,
//    which is the coordinator's file.** So the routing went one layer up instead:
//    [`crate::drops::Addressed`] names the recipient and `Fields::with_drops` posts it,
//    because that is the only function holding both the table and the bus. The consequence is
//    that `DropTable::sweep` now returns an **always-empty** `Vec<Reply>` - the signature is
//    kept only so the existing `out.extend(...)` call site still compiles. The tidier form is
//    in the report.
//
// 3. **`drop_from_mob` has to hand back the object id.** The plan's audience walk picks a
//    winner and then builds the packet, but there is no way to ask the bus *"is this
//    character on this map"* without attempting a delivery - `publish_to_character` answers by
//    doing it. So the drop is minted for the top candidate, and
//    [`crate::drops::DropTable::readdress`] moves it down the ranking until one lands. Nothing
//    has been sent about the drop while that happens, so no client can tell.
//
// ---------------------------------------------------------------------------------------
// 4. The departure hooks were the wrong half of the problem
//
// The plan said "release on departure" and every hook did exactly that, correctly. What it
// did not say is who claims them next, and the answer turned out to be **nobody until
// somebody walks through a portal** - `claim_uncontrolled` runs on field entry and there is
// no orphan sweep in `tick`. Releasing is silent, so a map full of motionless monsters was
// the visible result and there was no error and no log line pointing at it.
//
// Every one of those hooks now calls `Session::hand_over_mobs`, which picks a successor and
// tells it. `Drop for Session` is on the list - a killed client and a dead socket go through
// no hook at all - and so is the Cash Shop, which item 1 above says heals itself on the way
// back in but leaves everybody else's screen frozen while the shopper browses.
//
// ---------------------------------------------------------------------------------------
// What is NOT here, deliberately
//
// * **`CONTROL_RELEASE` is sent on a handover, and only there.** It releases; it does not
//   despawn a live mob. `net::mobmove::CONTROL_RELEASE` owns that fact and this file does not
//   restate it.
// * **No proximity rotation.** Same reason.
// * **No timer.** `research/mob-behaviour.md` §13: both new packets are reactive - one
//   `0x03E4` per inbound `0x02FF`, one `0x03D9` per inbound `0x02FF` per other client on the
//   field. Nothing here needs a periodic send and none should be added speculatively.
// * **No party.** [`Party::solo`] is the whole rule until the party agent lands, and
//   `Fields::parties()` is where its registry now sits.
// ---------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const A: SessionId = 1;
    const B: SessionId = 2;
    const MAP: u32 = 104_040_000;

    fn share(character: u32, dealt: u64, total: u64, majority: bool) -> DamageShare {
        DamageShare { character, dealt, total, majority }
    }

    // -- the registry ----------------------------------------------------------------

    /// **The whole invariant in one test.** The first arrival takes every mob; the second
    /// takes none and is a spectator. Both halves are asserted, because a claim function
    /// that returned nothing to everybody would pass the second half on its own.
    #[test]
    fn the_first_session_on_a_map_claims_every_mob_and_the_second_claims_none() {
        let c = Controllers::new();
        let alive = [2000u32, 2001, 2002];

        assert_eq!(c.claim_uncontrolled(MAP, A, &alive), vec![2000, 2001, 2002]);
        assert!(c.claim_uncontrolled(MAP, B, &alive).is_empty(), "control is sticky");

        assert_eq!(c.held_by(A), 3);
        assert_eq!(c.held_by(B), 0);
        for id in alive {
            assert_eq!(c.controller_of(MAP, id), Some(A));
            assert!(c.controls(MAP, id, A));
            assert!(!c.controls(MAP, id, B), "two controllers is the bug this prevents");
        }
    }

    /// The handover, and it costs no packet: A leaves, B's next tick picks the mobs up.
    #[test]
    fn a_departing_controller_frees_its_mobs_for_the_next_session_that_asks() {
        let c = Controllers::new();
        let alive = [2000u32, 2001];
        c.claim_uncontrolled(MAP, A, &alive);
        assert!(c.claim_uncontrolled(MAP, B, &alive).is_empty());

        assert_eq!(c.release_all(A), 2);
        assert_eq!(c.held_by(A), 0);
        for id in alive {
            assert_eq!(c.controller_of(MAP, id), None, "orphaned, and standing still");
        }

        assert_eq!(c.claim_uncontrolled(MAP, B, &alive), vec![2000, 2001]);
        assert_eq!(c.held_by(B), 2);
    }

    /// `Drop` runs after a clean log out, so releasing twice must free nothing the second
    /// time and must not take somebody else's mobs with it.
    #[test]
    fn releasing_twice_is_idempotent_and_does_not_touch_another_session() {
        let c = Controllers::new();
        c.claim_uncontrolled(MAP, A, &[2000]);
        c.claim_uncontrolled(MAP, B, &[2001]);

        assert_eq!(c.release_all(A), 1);
        assert_eq!(c.release_all(A), 0, "the second call frees nothing");
        assert_eq!(c.controller_of(MAP, 2001), Some(B), "and B is untouched");
        assert_eq!(c.len(), 1);
    }

    /// **A handover moves exactly this session's mobs on exactly this map**, and says which.
    ///
    /// The three things a caller depends on: the returned ids are what it must send grants
    /// for, the mobs it did *not* control are untouched, and the map next door is untouched -
    /// the last one because `Drop` hands over map by map and a crash can leave claims on more
    /// than one.
    #[test]
    fn a_handover_moves_one_sessions_mobs_on_one_map() {
        let c = Controllers::default();
        c.claim_uncontrolled(MAP, A, &[2000, 2001, 2002]);
        c.claim_uncontrolled(MAP, B, &[2003]);
        c.claim_uncontrolled(999, A, &[9000]);

        assert_eq!(c.hand_over(MAP, A, B), vec![2000, 2001, 2002], "sorted, so a test can name them");
        assert_eq!(c.held_by(A), 1, "only the other map is left");
        assert_eq!(c.controller_of(MAP, 2003), Some(B), "B's own mob was not disturbed");
        assert_eq!(c.controller_of(999, 9000), Some(A), "and neither was the map next door");

        assert!(c.hand_over(MAP, A, B).is_empty(), "a second call moves nothing");
        assert!(c.hand_over(7777, A, B).is_empty(), "a map nobody is on moves nothing");
    }

    /// **Handing to yourself is a no-op**, and it has to return an empty list rather than the
    /// ids it did not move.
    ///
    /// The caller sends one `0x03D2` per returned id. Returning them here would re-grant the
    /// mob to the connection that is *leaving*, whose client has already destroyed its mob
    /// pool - a packet naming an object that no longer exists on that screen.
    #[test]
    fn handing_over_to_yourself_moves_nothing() {
        let c = Controllers::default();
        c.claim_uncontrolled(MAP, A, &[2000, 2001]);
        assert!(c.hand_over(MAP, A, A).is_empty());
        assert_eq!(c.held_by(A), 2, "and it certainly does not drop them");
    }

    /// `maps_held_by` is what makes a mapless exit - a crash, a log out - able to hand over.
    #[test]
    fn maps_held_by_names_every_map_this_session_is_driving() {
        let c = Controllers::default();
        c.claim_uncontrolled(30, A, &[2000]);
        c.claim_uncontrolled(10, A, &[2001]);
        c.claim_uncontrolled(20, B, &[2002]);
        assert_eq!(c.maps_held_by(A), vec![10, 30], "sorted, and B's map is not one of them");
        assert_eq!(c.maps_held_by(B), vec![20]);
        assert!(c.maps_held_by(99).is_empty(), "a session that drives nothing names no maps");

        c.release_all(A);
        assert!(c.maps_held_by(A).is_empty(), "and a map it no longer holds drops off the list");
    }

    /// A portal walk frees only the map being left. `release_all` would free both, which is
    /// right for `Drop` and wrong for a walk.
    #[test]
    fn releasing_one_map_leaves_the_same_session_holding_another() {
        let c = Controllers::new();
        c.claim_uncontrolled(MAP, A, &[2000]);
        c.claim_uncontrolled(100_000_000, A, &[2000]);

        assert_eq!(c.release_map(MAP, A), 1);
        assert_eq!(c.controller_of(MAP, 2000), None);
        assert_eq!(c.controller_of(100_000_000, 2000), Some(A), "a third map is not touched");
        assert_eq!(c.release_map(999, A), 0, "a map nobody is on frees nothing");
    }

    /// **The registry is reconciled against the truth, not merely corrected at the death.**
    /// A mob that is gone from `alive` loses its entry even if `forget` was never called -
    /// otherwise one missed call means a mob nobody can ever claim.
    #[test]
    fn reconciling_drops_the_entry_of_a_mob_that_is_no_longer_alive() {
        let c = Controllers::new();
        c.claim_uncontrolled(MAP, A, &[2000, 2001, 2002]);
        assert_eq!(c.held_by(A), 3);

        // 2001 died and nobody said so. B walks in and reconciles first.
        assert_eq!(c.reconcile(MAP, &[2000, 2002]), 1, "one ghost, and it says so");
        assert_eq!(c.controller_of(MAP, 2001), None);
        assert_eq!(c.len(), 2);
        assert!(
            c.claim_uncontrolled(MAP, B, &[2000, 2002]).is_empty(),
            "the two survivors are still A's"
        );
        assert_eq!(c.reconcile(999, &[]), 0, "a map nobody is on reconciles to nothing");
    }

    /// **Claiming is additive and cannot take a mob from anyone**, which is why the
    /// destructive half is a separate call. A partial `candidates` list is safe here; the
    /// same list passed to `reconcile` would free the mobs it omits.
    ///
    /// Both halves are asserted in one test, because the version that folded the two
    /// together passed a test of either half alone.
    #[test]
    fn claiming_a_subset_leaves_another_sessions_mobs_alone_and_reconciling_would_not() {
        let c = Controllers::new();
        c.claim_uncontrolled(MAP, A, &[2000]);
        c.claim_uncontrolled(MAP, B, &[2001]);

        assert_eq!(c.controller_of(MAP, 2000), Some(A), "B's partial claim took nothing");
        assert_eq!(c.controller_of(MAP, 2001), Some(B));
        assert_eq!(c.len(), 2);

        // The mistake the split exists to make visible, done deliberately.
        assert_eq!(c.reconcile(MAP, &[2001]), 1);
        assert_eq!(c.controller_of(MAP, 2000), None, "reconcile IS destructive, by name");
    }

    #[test]
    fn forgetting_a_dead_mob_drops_its_entry() {
        let c = Controllers::new();
        c.claim_uncontrolled(MAP, A, &[2000, 2001]);
        c.forget(MAP, 2000);
        assert_eq!(c.controller_of(MAP, 2000), None);
        assert_eq!(c.controller_of(MAP, 2001), Some(A));
        c.forget(MAP, 2000); // idempotent
        c.forget(999, 2000); // an unknown map is not a panic
        assert_eq!(c.len(), 1);
    }

    /// A respawn: the ticking session claims the new mob, and a second session asking for the
    /// same one is told no.
    #[test]
    fn a_new_mob_is_claimed_once_and_a_second_claim_is_refused() {
        let c = Controllers::new();
        assert!(c.claim_one(MAP, 2000, A));
        assert!(!c.claim_one(MAP, 2000, B), "B must not send a second 0x03D2");
        assert!(c.claim_one(MAP, 2000, A), "the holder asking again is still the holder");
        assert_eq!(c.controller_of(MAP, 2000), Some(A));
    }

    /// Two maps share nothing, the same way `Fields` keys by map.
    #[test]
    fn control_is_keyed_by_map() {
        let c = Controllers::new();
        c.claim_uncontrolled(MAP, A, &[2000]);
        assert_eq!(c.controller_of(100_000_000, 2000), None);
        assert_eq!(c.claim_uncontrolled(100_000_000, B, &[2000]), vec![2000]);
        assert_eq!(c.controller_of(MAP, 2000), Some(A));
        assert_eq!(c.controller_of(100_000_000, 2000), Some(B));
    }

    /// **The race, run rather than argued.** Four threads claim the same twenty mobs. The
    /// property is that the twenty claims are partitioned - every mob claimed exactly once,
    /// by exactly one session - which is the invariant the whole feature rests on and the one
    /// a `HashMap` without a lock would break intermittently.
    #[test]
    fn two_sessions_racing_for_the_same_mobs_split_them_and_never_share_one() {
        use std::sync::Arc;

        let c = Arc::new(Controllers::new());
        let alive: Vec<u32> = (2000..2020).collect();

        let handles: Vec<_> = (1..=4u64)
            .map(|session| {
                let c = c.clone();
                let alive = alive.clone();
                std::thread::spawn(move || c.claim_uncontrolled(MAP, session, &alive))
            })
            .collect();

        let mut claimed: Vec<u32> =
            handles.into_iter().flat_map(|h| h.join().expect("a claimer panicked")).collect();
        claimed.sort_unstable();

        assert_eq!(claimed, alive, "every mob claimed exactly once, and none twice");
        assert_eq!(c.len(), 20);
        let held: usize = (1..=4u64).map(|s| c.held_by(s)).sum();
        assert_eq!(held, 20, "and the twenty are partitioned across the four sessions");
    }

    // -- who a packet is for ---------------------------------------------------------

    /// **The one mistake this module exists to prevent.** A `MOB_CHANGE_CONTROLLER` that
    /// went to the map would grant two clients control of one mob, and the client rolls the
    /// wander itself - `research/mob-behaviour.md` §5.1 - so the two screens would diverge on
    /// the first step.
    #[test]
    fn mob_change_controller_is_never_map_wide() {
        assert_eq!(
            audience_for(net::mobmove::MOB_CHANGE_CONTROLLER, 2000),
            Audience::OnlyTheActingSession
        );
        assert_eq!(
            audience_for(net::mobmove::MOB_CTRL_ACK, 2000),
            Audience::OnlyTheActingSession,
            "the ack echoes ONE client's own move counter"
        );
    }

    /// The three packets the owner's sentence is about, and the fourth that makes a mob move on a
    /// screen that does not own it.
    #[test]
    fn damage_death_spawn_and_movement_all_go_to_the_map() {
        for opcode in [
            net::combat::MOB_HP_CHANGE,
            net::combat::MOB_LEAVE_FIELD,
            net::mob::MOB_ENTER_FIELD,
            net::mobmove::MOB_MOVE,
        ] {
            assert!(
                audience_for(opcode, 2000).is_map_wide(),
                "{opcode:#06x} has to reach the other screens"
            );
        }
    }

    /// Only movement supersedes, and it supersedes per mob. See [`audience_for`] for why the
    /// other three do not.
    #[test]
    fn only_movement_carries_a_supersede_key_and_it_is_the_mob() {
        assert_eq!(audience_for(net::mobmove::MOB_MOVE, 2000).supersedes(), Some(2000));
        assert_eq!(audience_for(net::mobmove::MOB_MOVE, 2001).supersedes(), Some(2001));
        for opcode in [
            net::combat::MOB_HP_CHANGE,
            net::combat::MOB_LEAVE_FIELD,
            net::mob::MOB_ENTER_FIELD,
        ] {
            assert_eq!(
                audience_for(opcode, 2000).supersedes(),
                None,
                "{opcode:#06x} must not coalesce"
            );
        }
    }

    /// Drops are per client, so neither drop packet is ever published to a map.
    #[test]
    fn drop_packets_are_private() {
        for opcode in [net::drops::DROP_ENTER_FIELD, net::drops::DROP_LEAVE_FIELD] {
            assert_eq!(audience_for(opcode, 20_000_000), Audience::OnlyTheActingSession);
        }
    }

    /// **The default is private**, which is the safe direction: an unshared update is a
    /// missing bar, a wrongly shared one is a second controller or somebody else's loot.
    #[test]
    fn an_opcode_nobody_has_thought_about_is_private() {
        for opcode in [0x0000u16, 0x007C, 0x0224, 0x0293, 0x029E, 0xFFFF] {
            assert_eq!(
                audience_for(opcode, 2000),
                Audience::OnlyTheActingSession,
                "{opcode:#06x} must be added to the table deliberately"
            );
        }
    }

    // -- movement reports ------------------------------------------------------------

    #[test]
    fn only_the_controller_may_report_a_mobs_movement() {
        assert!(may_report_movement(Some(A), A));
        assert!(!may_report_movement(Some(A), B), "B does not own this simulation");
        assert!(!may_report_movement(None, A), "an orphaned mob cannot be reporting");
    }

    // -- drops -----------------------------------------------------------------------

    /// A solo kill: one contributor, and they are the killer.
    #[test]
    fn a_solo_kill_gives_the_drops_to_the_only_person_who_hit_it() {
        let shares = [share(200, 45, 45, true)];
        assert_eq!(drop_audience(&shares, 200), vec![200]);
    }

    /// **The case the owner named.** A did 90, B landed the killing 10. A sees the drops even
    /// though B killed it - and B is still in the list, as the fallback if A has gone.
    #[test]
    fn the_top_damager_wins_even_when_somebody_else_lands_the_killing_blow() {
        let shares = [share(200, 90, 100, true), share(201, 10, 100, false)];
        assert_eq!(drop_audience(&shares, 201), vec![200, 201]);
    }

    /// Over-damage is already excluded upstream, and this test says where: a 500-damage
    /// finisher on a mob with 60 left is credited 60, so 40 does not beat it and the ranking
    /// `Fields` produced is used unchanged.
    #[test]
    fn over_damage_is_not_re_excluded_here_because_it_was_already_excluded_there() {
        use crate::fields::LiveMob;
        let mut m = LiveMob {
            spawn: net::mob::FieldMob::new(2000, 2, 0, 0, 1, 100),
            hp: 100,
            at: None, at_fh: None,
            damage_by: Vec::new(),
        };
        // The same sequence `fields::tests::a_killing_blow_is_credited_only_for_what_landed`
        // pins: 40 from 200, then a 500 overkill from 201 which credits 60.
        m.hp -= 40;
        m.damage_by.push((200, 40));
        m.damage_by.push((201, 60));
        m.hp = 0;

        let shares = m.shares();
        assert_eq!(shares[0].dealt, 60, "the cap is Fields', not ours");
        assert_eq!(drop_audience(&shares, 201), vec![201, 200]);
    }

    /// A tie is broken by first blood, because `shares()` sorts stably. The same fight scores
    /// the same way twice, which is the property that matters.
    #[test]
    fn a_tie_goes_to_whoever_hit_it_first() {
        use crate::fields::LiveMob;
        let mut m = LiveMob {
            spawn: net::mob::FieldMob::new(2000, 2, 0, 0, 1, 100),
            hp: 100,
            at: None, at_fh: None,
            damage_by: vec![(207, 50), (209, 50)],
        };
        m.hp = 0;
        let shares = m.shares();
        assert_eq!(drop_audience(&shares, 209), vec![207, 209], "first to hit takes it");
    }

    /// A mob that died with nothing credited - `Fields::hurt` returns `Died(vec![])` - still
    /// drops to somebody. `award_kill_experience` already treats that case as "pay the killer
    /// in full", and this matches it rather than inventing a second rule.
    #[test]
    fn a_kill_with_no_credited_damage_still_falls_back_to_the_killer() {
        assert_eq!(drop_audience(&[], 204), vec![204]);
    }

    /// The killer is always the terminator of the walk, even when they are not in the
    /// ranking, so a caller looking for "the first candidate on this map" always finds one.
    #[test]
    fn the_killer_is_always_in_the_list_and_always_last_resort() {
        let shares = [share(200, 90, 100, true), share(201, 10, 100, false)];
        let ranked = drop_audience(&shares, 999);
        assert_eq!(ranked, vec![200, 201, 999]);
        assert_eq!(*ranked.last().unwrap(), 999);
    }

    // -- the party seam --------------------------------------------------------------

    /// Today's whole rule: the owner sees their drops and nobody else does.
    #[test]
    fn solo_means_the_owner_and_only_the_owner() {
        let party = Party::solo(200);
        assert!(may_see_drop(200, 200, &party));
        assert!(!may_see_drop(200, 201, &party), "a bystander sees nothing");
        assert!(!may_see_drop(201, 200, &party), "and does not gain sight of theirs");
        assert!(party.is_solo());
        assert_eq!(own_type_for(&party), net::drops::OWN_TYPE_USER);
    }

    /// **The seam, exercised.** Nothing constructs a real `Party` yet; this proves the
    /// predicate is already the whole of the party rule, so landing parties is a change at
    /// the call site that builds the value and nowhere else.
    #[test]
    fn a_party_shares_its_members_drops_and_nobody_elses() {
        let party = Party::of(200, [201, 202]);
        assert!(may_see_drop(200, 201, &party), "a member sees a member's drop");
        assert!(may_see_drop(202, 200, &party));
        assert!(!may_see_drop(200, 300, &party), "an outsider still sees nothing");
        assert!(!may_see_drop(300, 200, &party), "and the party does not see an outsider's");
        assert!(!party.is_solo());
        assert_eq!(own_type_for(&party), net::drops::OWN_TYPE_PARTY);
    }

    /// A party that has forgotten its own member must not hide a drop from its owner.
    #[test]
    fn a_party_always_contains_the_character_it_was_built_for() {
        let party = Party::of(200, [201, 201, 200]);
        assert!(party.contains(200));
        assert_eq!(party.members(), &[200, 201], "and no duplicates");
        assert!(may_see_drop(200, 200, &party));
    }
}
