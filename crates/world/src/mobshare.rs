//! **One mob, one simulation, two screens.** Who controls each monster, which mob packets
//! are map-wide and which are private, and who gets to see the loot.
//!
//! Labels are the project's: **[L]** read off this client's listing or a capture, **[D]**
//! derived from two or more [L] facts, **[I]** inferred - policy nothing on this machine can
//! confirm.
//!
//! # Nothing here is wired, and nothing here has ever been on a wire between two players
//!
//! This module is the **decision**, the same shape as [`crate::taxi`] and
//! [`crate::secondjob`]: it sends no packets, touches no database, and knows nothing about
//! `Session`, so every branch below is a unit test rather than a client run. §"WIRE IT LIKE
//! THIS" at the bottom is the patch, and `research/mob-share.md` is the working.
//!
//! `CLAUDE.md`'s *built is not wired*: until `session/field.rs`, `session/combat.rs` and
//! `session/mod.rs` call into this, two players on one map still run two independent
//! simulations of every monster and it looks on screen exactly as it does today.
//!
//! **Two clients have never been connected to this server at once.** Every claim below about
//! what a second player *sees* is [I] on that point, however well-read the packet is.
//!
//! # The problem, measured rather than suspected
//!
//! `crates/world/src/session/field.rs` grants `MOB_CHANGE_CONTROLLER` for **every** mob to
//! **every** arriving session, and `crates/world/src/fields.rs` has no controller registry at
//! all. `research/mob-behaviour.md` §5.1 reads the wander out of the client: mob vtable slot
//! 19 `FUN_141c8d1b0` builds a list of 12-byte path elements out of the random source
//! `FUN_142f04924`, **once per path element, in the controlling client**, and hands it to the
//! move builder. **[D]** So two grants are two dice rolls, and the two screens diverge on the
//! first step and never reconverge.
//!
//! Three further consequences of the same missing registry, all read out of this repo's own
//! code rather than guessed:
//!
//! * `Fields::due_respawns` **drains** `field.pending`. With two sessions ticking, whichever
//!   ticks first takes the new mobs and the other session is never told they exist. A
//!   respawn is therefore unicast today. **[L]** from `fields.rs`.
//! * `Fields::note_position` is written by every `0x02FF` from every session. Two controllers
//!   are two writers of one field, last write wins, and the position a drop lands on is
//!   whichever client reported most recently. **[L]** from `fields.rs` and
//!   `session/combat.rs`.
//! * `DropTable::sweep` removes an expired drop from the shared table and returns the
//!   `0x046F` to the **calling** session only, so a second player keeps drawing an item that
//!   no longer exists. **[L]** from `drops.rs`.
//!
//! # The model, in one sentence
//!
//! **Exactly one connection controls each mob; control is claimed by a session for itself,
//! never handed to another session; and it is never revoked while its holder is still on the
//! map, because the client's only revoke is a despawn.**
//!
//! ## Why control is sticky: the revoke deletes the mob
//!
//! `net::mobmove::CONTROL_RELEASE` is level `0`, and `141d30ef5 TEST EBP,EBP / JE 141d30f1c`
//! takes the zero branch straight into the pool's erase path. **[L]**, and
//! `mob_release_controller`'s own name says so. There is no "you are no longer the
//! controller, but keep drawing it" packet in this client, so a real server's proximity-based
//! rotation is not available: rotating would mean despawning the mob on the old controller's
//! screen and re-creating it with a fresh `0x03C6`, which is a visible pop and an animation
//! reset for a cosmetic gain.
//!
//! So **this server never sends `CONTROL_RELEASE` at all.** A controller loses its mobs only
//! by leaving the field, and a client that leaves a field has already torn its own mob pool
//! down - the `SetField` does it - so the transfer costs zero packets on the losing side.
//!
//! ## Every transition is a session claiming for itself
//!
//! [`Controllers::claim_uncontrolled`] is a compare-and-set under one lock. A session calls
//! it for **itself**, on its own map, in its own thread:
//!
//! | when | what it claims | what it sends |
//! |---|---|---|
//! | field entry | every uncontrolled mob on the map | `0x03C6` for **all** mobs, `0x03D2` for the claimed ones |
//! | respawn tick | the mobs its own `due_respawns` returned | `0x03C6` + `0x03D2` to itself, `0x03C6` published to the map |
//! | any later tick | anything orphaned since | `0x03D2` for the newly claimed |
//! | leaving | nothing - [`Controllers::release_all`] | **nothing.** The client's pool is already gone |
//!
//! That is what makes this design cheap: **no control packet is ever addressed to a
//! connection other than the one building it**, so no new bus channel is needed for control,
//! and the "two controllers" bug cannot be reintroduced by a routing mistake.
//!
//! The cost is **one tick of latency** when a controller departs: the mobs it held are
//! orphaned until some other session's next `tick` claims them, and an orphaned mob does not
//! move. The tick is the 100 ms wakeup `crate::broadcast` describes, so the visible artefact
//! is up to a tenth of a second of stillness. Nobody has watched it.
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
//! §"WIRE IT LIKE THIS". All three were read today with `tools/listing.py`, whose documented
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
/// `Fields`'s own map lock; see §"WIRE IT LIKE THIS".
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

    /// **This connection is gone.** Free everything it held; returns how many.
    ///
    /// Every map, not one, because the caller is often `Drop` and has no map to hand -
    /// exactly the reason `Bus::part` takes no map either. Idempotent: a session that left
    /// its field cleanly and is then dropped calls this a second time and frees nothing.
    ///
    /// **No packet follows.** The only revoke this client has is a despawn, and a client that
    /// has left a field has already destroyed its own mob pool. See the module docs.
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
// WIRE IT LIKE THIS
//
// Everything below is in files this agent does not own. Nothing in this module is called by
// anything yet, and until it is, two players on one map still run two simulations of every
// mob - `CLAUDE.md`'s "built is not wired".
//
// ---------------------------------------------------------------------------------------
// 0. `crates/world/src/fields.rs` - two lines, so the registry reaches a session
//
//    Beside the `bus` field, which is there for exactly the same reason ("this `Arc` is
//    already handed to every `Session`"):
//
//        /// Who controls each mob. See `crate::mobshare`.  Its lock is a leaf, like the
//        /// bus's: nothing in `mobshare` calls back into `Fields`.
//        controllers: crate::mobshare::Controllers,
//
//        pub fn controllers(&self) -> &crate::mobshare::Controllers { &self.controllers }
//
//    `Controllers: Default`, so `#[derive(Default)] struct Fields` is unchanged.
//
//    **Lock order.** Never call `fields.controllers()` from inside a `Fields::with_drops`
//    closure or between a `mobs_on` and its result being used - take the `Fields` answer
//    first, drop it, then claim. The two locks are independent and neither calls the other,
//    so this is a rule about not inventing a cycle rather than about breaking one.
//
// ---------------------------------------------------------------------------------------
// 1. `session/field.rs::on_field_entered` - the grant becomes a claim
//
//    Today the loop sends `0x03C6` + `0x03D2` for every mob to every arriving session. Split
//    it:
//
//        let live = self.fields.mobs_on(chr.map_id);
//        let alive: Vec<u32> = live.iter().map(|m| m.spawn.object_id).collect();
//        // `alive` is the WHOLE map here, which is `reconcile`'s precondition. A non-zero
//        // return means a `forget` was missed on some death path - log it, do not discard it.
//        let ghosts = self.fields.controllers().reconcile(chr.map_id, &alive);
//        let mine = self.fields.controllers().claim_uncontrolled(
//            chr.map_id, self.subscriber.get(), &alive);
//        for live in live {
//            ... push the MOB_ENTER_FIELD exactly as today ...
//            if mine.contains(&mob.object_id) {
//                ... push the MOB_CHANGE_CONTROLLER exactly as today ...
//            }
//        }
//        crate::server::log(&format!(
//            "   map {} has {} mob(s); this connection now controls {}{}",
//            chr.map_id, alive.len(), mine.len(),
//            if ghosts > 0 { format!(" ({ghosts} STALE entries dropped - a forget was missed)") }
//            else { String::new() }));
//
//    The first player on a map claims all of them; the second claims none and is a spectator.
//    The log line is the check that the split went where it was meant to.
//
// 2. `session/field.rs::go_to_map` and `session/mod.rs`'s `leave_the_field` callers -
//    release
//
//    Beside every existing `self.leave_the_field()` (a portal walk, Log Out, Change Channel,
//    the Cash Shop) and in `Drop for Session` beside `Bus::part`:
//
//        self.fields.controllers().release_all(self.subscriber.get());
//
//    `release_map(map, id)` is available where the map is known and is tidier; `release_all`
//    is the one `Drop` must use, because `Drop` has no character to read a map from. Both are
//    idempotent.
//
//    **`Drop` is the single point of failure for this registry.** A session that vanishes
//    without it leaves mobs claimed by a dead id and nobody will ever claim them again -
//    monsters that have stopped moving for no visible reason. `Bus::part` is already in
//    `Drop`; put this on the line beside it, not in a different function.
//
// 3. `session/combat.rs::spawn_due_mobs` - claim the new mobs, publish the spawn
//
//        let arrived = self.fields.due_respawns(map, &self.config, now_ms);
//        for live in arrived {
//            let mut mob = live.as_seen();
//            mob.appear_type = net::mob::APPEAR_SPAWNING;
//            mob.forced_stat = self.forced_stat_for(mob.template_id);
//            let spawn = Reply { opcode: net::mob::MOB_ENTER_FIELD, ... };   // as today
//            // Everyone else on the map, or they never learn this mob exists: due_respawns
//            // DRAINS the pending list, so only the session that ticked first sees it.
//            if crate::mobshare::audience_for(spawn.opcode, mob.object_id).is_map_wide() {
//                self.bus().publish(self.subscriber, map, spawn.clone(), None);
//            }
//            out.push(spawn);
//            // ...and the grant is ours alone.
//            if self.fields.controllers().claim_one(map, mob.object_id, self.subscriber.get()) {
//                out.push(Reply { opcode: net::mobmove::MOB_CHANGE_CONTROLLER, ... });
//            }
//        }
//
//    This also spreads control across connections over time, because whichever session ticks
//    first takes each new mob - which is worth having: with one session holding everything, a
//    single stalled client freezes every monster on the map.
//
//    **A narrow race, and it is benign.** A player entering the map at the instant another
//    session's tick publishes a spawn can be handed the same `0x03C6` twice - once from
//    `mobs_on`, once from the publish. `FUN_141d33630` looks the object id up in the pool at
//    `141d33711 call 0x141d2efc0` and, when it is found, takes the branch at `141d33725`
//    that re-initialises the existing mob instead of creating a second one (`je 0x141d33788`
//    is the create path). **[L]** for the fork; **[I]** that the visible effect is a
//    re-initialisation rather than a duplicate.
//
// 4. `session/combat.rs::on_mob_move` - only the controller is believed
//
//        let Some(map) = self.claimed_character().map(|c| c.map_id) else { return Vec::new() };
//        if !crate::mobshare::may_report_movement(
//            self.fields.controllers().controller_of(map, req.object_id),
//            self.subscriber.get(),
//        ) {
//            crate::server::log(&format!(
//                "   0x02FF for mob {} IGNORED: this connection does not control it (controller {:?})",
//                req.object_id, self.fields.controllers().controller_of(map, req.object_id)));
//            return Vec::new();
//        }
//        self.fields.note_position(map, req.object_id, (req.x, req.y));
//        // ...the 0x03E4 exactly as today...
//        // ...and now the other screens:
//        let a = crate::mobshare::audience_for(net::mobmove::MOB_MOVE, req.object_id);
//        if a.is_map_wide() {
//            self.bus().publish(self.subscriber, map, Reply {
//                opcode: net::mobmove::MOB_MOVE,
//                body: net::mobmove::mob_move_broadcast(&req),
//                what: format!("MobMove: mob {} to ({}, {}), {} path bytes copied verbatim",
//                              req.object_id, req.x, req.y, req.path.len()),
//            }, a.supersedes());
//        }
//
//    `Bus::publish` excludes the sender, and the sender is the controller, so the rule
//    *"never send `0x03D9` to the client that sent the `0x02FF`"* (`research/mob-behaviour.md`
//    §12.1 - the handler overwrites position, animation and `mob+0xcd0`, state the controller
//    owns) is satisfied by construction rather than by a second check.
//
// 5. `session/combat.rs::on_attack` - the damage and the death, to everyone
//
//    In the target loop, at the bottom where `mob_hit_replies` is turned into `Reply`s:
//
//        for (opcode, body) in net::combat::mob_hit_replies(target.object_id, &hit, max_hp) {
//            let reply = Reply { opcode, body, what: ...same as today... };
//            let a = crate::mobshare::audience_for(opcode, target.object_id);
//            if a.is_map_wide() {
//                self.bus().publish(self.subscriber, map, reply.clone(), a.supersedes());
//            }
//            out.push(reply);
//        }
//
//    **Publish the SAME `(opcode, body)` the attacker gets. Do not recompute.** `0x03F0`
//    carries a percentage, not an absolute (`net::combat::hp_percent`,
//    `research/mob-hp-bar.md`), and a second call site that looked up `max_hp` its own way is
//    exactly how that unit error would come back. One body, two destinations.
//
//    And on the death branch, beside the `Hurt::Died` arm:
//
//        self.fields.controllers().forget(map, target.object_id);
//
// 6. `session/combat.rs::drops_from_kill` - the top damager owns them
//
//    `on_attack` currently passes `chr_id` as the drop owner. Pass the audience instead:
//
//        if let crate::fields::Hurt::Died(shares) = left {
//            let ranked = crate::mobshare::drop_audience(&shares, chr_id);
//            out.extend(self.drops_from_kill(template, target.object_id, died_at, &ranked, map));
//            ...
//        }
//
//    and inside `drops_from_kill`, walk `ranked` and stop at the first candidate that is on
//    **this** map. `chr_id` is always in the list and always on the map, so the walk
//    terminates. The `DropFromMob.owner_id` becomes that character, which is what
//    `LiveDrop::may_be_taken_by` then enforces.
//
//    **This needs one bus method that does not exist**, because the winner may be a different
//    connection:
//
//        /// Queue a finished packet for whichever connection is playing `character` **on
//        /// `map`**. Returns whether anyone was listening.
//        ///
//        /// Unlike `send_to_character`, this carries bytes - which is only legitimate for a
//        /// packet whose content does not depend on who receives it. A `0x046E` is identical
//        /// for every viewer; a `0x007C` is not, and that is why `Event` exists.
//        ///
//        /// The `map` is not optional: a `0x046E` names a position in the field the
//        /// recipient is standing in, so delivering one to a character who has walked away
//        /// would put a phantom item on a map it was never dropped on.
//        pub fn publish_to_character(&self, character: u32, map: u32, reply: Reply) -> bool
//
//    Ten lines beside `send_to_character`: the same loop over `boxes`, matching
//    `p.character == character && p.map == map`, pushing `Queued { reply, supersedes: None }`
//    into `queue` rather than into `events`. It belongs to the coordinator because
//    `broadcast.rs` is a shared file.
//
//    *If the coordinator would rather not add it*, the fallback needs no shared-file change
//    at all: give the drop to the highest-ranked candidate **who is this session**, else to
//    `chr_id`. That is "the top damager sees the drops unless the killing blow came from
//    somebody else", which is not what the owner asked for, and it should be said out loud rather
//    than shipped quietly.
//
// 7. `session/field.rs::on_field_entered` and `session/mod.rs::tick` - the floor is private
//
//    Field entry currently re-sends the whole floor to whoever walks in. Filter it:
//
//        let party = crate::mobshare::Party::solo(chr.id);
//        out.extend(self.fields.with_drops(map, |d| {
//            let _ = d.field_entry(map, now);   // purges expired; its replies are rebuilt below
//            d.on_field(map)
//                .filter(|dr| crate::mobshare::may_see_drop(dr.owner_id, chr.id, &party))
//                .map(|dr| dr.enter_reply(net::drops::ENTER_INSTANT))
//                .collect::<Vec<_>>()
//        }));
//
//    The sweep in `tick` is the same problem from the other end: it removes an expired drop
//    from the shared table and returns the `0x046F` to whichever session ticked, which may
//    not be the one that can see it. Read the owners first, then route with
//    `publish_to_character` (item 6) by the object id in the reply's first four bytes -
//    `net::drops::drop_leave_field` writes it there.
//
// ---------------------------------------------------------------------------------------
// What is NOT in this plan, deliberately
//
// * **No `CONTROL_RELEASE`, ever.** It despawns. See the module docs.
// * **No proximity rotation.** Same reason.
// * **No timer.** `research/mob-behaviour.md` §13: both new packets are reactive - one
//   `0x03E4` per inbound `0x02FF`, one `0x03D9` per inbound `0x02FF` per other client on the
//   field. Nothing here needs a periodic send and none should be added speculatively.
// * **No party.** `Party::solo` is the whole rule until the party agent lands.
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
            at: None,
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
            at: None,
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
