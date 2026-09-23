//! One channel's message bus: how a packet reaches a connection that did not ask
//! for it.
//!
//! # The gap this closes, which three files already name
//!
//! `crates/world/src/server.rs` gives every connection **its own thread and its own
//! socket**, and `Session` is a pure state machine - bodies in, bodies out. That is
//! deliberate and it is what makes ~7000 lines of session tests unit tests. It also
//! means a session has never had any way to say something to *another* session, and
//! the consequence is written down in three places already:
//!
//! * `crates/world/src/session/combat.rs::award_kill_experience` - *"there is no way
//!   to push a packet to another player's thread"*, so a second contributor's share
//!   of a kill is computed and then dropped.
//! * `research/exp-sharing.md` - *"`Fields::hurt` already returns the whole split;
//!   **only the delivery is missing**"*.
//! * `research/user-move.md` §6 - the one open question about `0x00D9` is *"whether
//!   the server is expected to broadcast this to other players. There is exactly one
//!   player here, so it has never mattered."*
//!
//! The owner, 2026-08-29: *"Right now according to the world log, the client's own
//! movement is completely disregarded. Now that we potentially will have multiple
//! clients and multiple characters appear in the same map at the same time, their
//! movements and their attacks need to be broadcasted and shown on all clients."*
//!
//! # The shape: a mailbox per connection, drained by the connection itself
//!
//! A publisher does **not** touch another socket. It appends to the other
//! connection's mailbox and returns; that connection picks its own mail up on its own
//! thread, in `Session::handle` (right after whatever it just asked for) and in
//! `Session::tick` (the 100 ms wakeup the socket loop already runs when nothing has
//! arrived). So:
//!
//! * no second writer ever touches a `TcpStream`, and the framer's cipher state -
//!   which is a *stream* cipher and therefore order-dependent - stays owned by one
//!   thread. Two threads framing onto one socket would corrupt every subsequent
//!   packet, and would do it intermittently.
//! * `Session` stays a pure state machine. It takes a `&Bus` the same way it already
//!   takes `&Fields`, and every test in `session/tests.rs` still constructs one
//!   without a socket.
//! * `crates/world/src/server.rs` needed **no change at all** except a shorter tick.
//!
//! # This module knows no packet layouts, on purpose
//!
//! A [`Presence`] carries its own already-built `spawn` and `farewell` packets. The
//! bus never looks inside them. That is what let this file be written, tested and
//! landed while the `0x0224` UserEnterField body was still being read out of the
//! client - and it is the right boundary anyway: the bus's job is delivery, and
//! delivery is the half `exp-sharing.md` says is missing.
//!
//! # Superseding, which is the whole memory story
//!
//! A mailbox is unbounded in the obvious design, and a client that stops reading
//! grows it forever. The fix is not a size cap - a cap has to choose what to throw
//! away, and throwing away a UserEnterField while keeping the movement that follows
//! it leaves a **ghost**: a character the observer's client has never been told
//! exists, moving.
//!
//! Instead a publisher may declare a broadcast *supersedable*: a later one with the
//! same opcode about the same character **replaces an earlier one still sitting in
//! the queue**. Movement is exactly that - a position that has been overtaken is
//! worth nothing, and an observer who has been away should see where you are, not
//! replay where you have been. So a stalled connection accumulates at most one
//! pending move per character, and spawns, farewells and attacks are never dropped.
//!
//! Attacks are **not** supersedable: two swings are two events.
//!
//! # The second channel: a *fact* to a *character*, not a packet to a map
//!
//! [`Bus::publish`] carries a finished [`Reply`] to everyone on a map. That works
//! because a movement packet says the same thing to every observer. **Experience does
//! not.**
//!
//! `0x007C` (`net::stats::STAT_CHANGED`) carries the recipient's **own new total and
//! own new level**. Only the recipient's session can compute those: it needs that
//! character's database record, its own `config.exp_curve.award(...)`, and its own
//! `store::save_character_progress`. If the killing session built that packet and
//! posted it, the second contributor would be shown *somebody else's* EXP total and
//! *somebody else's* level - and the level is the number a level-up effect hangs off,
//! so the error would not stay quiet.
//!
//! So the second channel carries **"you earned N, for reason R, in colour C"** and the
//! receiving session builds its own packet from its own record. That is why [`Event`]
//! is an `enum` and not a `Vec<u8>`: the bus is deliberately unable to express a
//! finished EXP packet, because a finished EXP packet cannot be correct for anyone but
//! its author.
//!
//! It is addressed to a **character**, not a map and not a [`SubscriberId`], because
//! the killing session knows *who contributed damage* - it has a character id out of
//! `Fields::hurt` - and has no idea which connection that is.
//!
//! ## The lifetime rule that is different from the packet queue
//!
//! [`Bus::enter_field`] clears `queue` and deliberately does **not** clear `events`.
//! A queued packet names an object in a field that the client has just torn down; a
//! fact is owed to a *person*. A contributor who lands a hit and walks through a
//! portal before the mob dies has still earned the share, and the portal is not a
//! reason to take it away. See the comment at the `clear` itself.
//!
//! ## What this channel cannot reach, stated rather than papered over
//!
//! Delivery matches on `mailbox.presence`, and `leave_field` clears the presence while
//! keeping the mailbox. So a connection sitting at character select - or between a
//! channel change's two halves - has **no character id to match on** and does not
//! receive. [`Bus::send_to_character`] returns `false` for it, the same as for a
//! player who has genuinely logged out. The two cases are not distinguished, and
//! nothing here retries.

use std::collections::HashMap;
use std::sync::Mutex;

use crate::session::Reply;

/// Which connection. Handed out by [`Bus::join`] and meaningless outside one `Bus`.
///
/// Not the character id: a connection has a mailbox from the moment it is accepted,
/// which is before it has claimed a migration and therefore before anyone knows which
/// character it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SubscriberId(u64);

impl SubscriberId {
    /// The raw number, for logging only.
    pub fn get(&self) -> u64 {
        self.0
    }
}

/// One player as everyone else on the field needs to hear about them.
///
/// The two packets are built by the session that owns the character, because that is
/// the only place that knows what the character looks like. The bus copies them and
/// never reads them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Presence {
    pub character: u32,
    pub map: u32,
    /// "This player is here" - sent to everyone already on the map when this player
    /// arrives, and sent to this player for everyone already there.
    pub spawn: Reply,
    /// "This player is gone" - sent to everyone still on the map when this player
    /// leaves the map, changes channel, or drops the connection.
    ///
    /// Built **up front**, at join time, rather than at departure. A connection that
    /// dies is dropped in `Session::drop`, where there is no store, no config and no
    /// character record to build anything from; a farewell that could only be built
    /// on a clean exit would be missing from exactly the case that needs it most.
    pub farewell: Reply,
    /// **What travels with this player** - packets a later arrival must receive right
    /// after `spawn`, because they name this character and the client drops a user-pool
    /// packet whose character it does not yet have. Today that is the summoned pet's
    /// `0x0277`. Empty for a player with nothing out.
    ///
    /// The owner, 2026-09-14: *"The second client does not see the Husky pet."* The summon
    /// and every walk were already published to the map, and a pet owner walking INTO a
    /// map already announced pet after player. The one direction nobody had covered was
    /// the other player walking in while the pet was already out: they got the spawn
    /// alone, and the pet existed only on its owner's screen.
    pub companions: Vec<Reply>,
}

/// One fact that crosses the bus, addressed to one character.
///
/// Not a packet. See the module docs: the recipient's session builds the packet,
/// because `0x007C` carries the recipient's own totals and nobody else can compute
/// them. Adding a variant here is adding a *fact*; if a variant is ever tempted to
/// carry bytes, it belongs in [`Bus::publish`] instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// `amount` experience is owed to this character, for `why`, drawn in white if
    /// `white` - the flag the client uses to distinguish a share from one's own kill.
    ///
    /// The **amount** is what crosses; the new total and the new level are not, and
    /// must not be. The receiving session applies its own curve and its own save -
    /// that is the whole point, since only it can turn an amount into *its* total.
    ///
    /// **The EXP rate is already in `amount` and must not be applied again.**
    /// `Session::exp_for_kill` multiplies by the rate *before* `Fields::hurt`'s split,
    /// so every share crosses pre-scaled. That is correct today only because
    /// `Session::rate` reads one server-wide table (`store.rates()`), so the killer's
    /// rate and the recipient's are the same number. **If per-character or per-party
    /// rates are ever added, this stops being equivalent** and the decision - whose
    /// rate applies to a share - has to be made deliberately rather than inherited
    /// from whoever happened to land the killing blow.
    Experience { amount: u64, why: String, white: bool },

    /// `amount` mesos are owed to this character as their **party share** of a pick-up by
    /// `picker`, from a drop a mob left.
    ///
    /// The owner, 2026-09-14: *"If mesos are picked up by a member of a party, the same split for
    /// EXP should exist. The picked up person should get 70%, and every other member should
    /// get a copy of the 30% provided that the mesos is from mob death."* Crosses as a fact
    /// for the same reason EXP does: the recipient's session owns their purse row and their
    /// client's `0x007C`, and is the only one that can say what their new balance is.
    PartyMesos { amount: u32, picker: u32 },

    /// **Go to `map`** - the party quest pulling a member in with its leader.
    ///
    /// `crate::firsttime`: the leader speaks to Lakelis and passes the gate, but only a
    /// member's OWN session can build that member's `SetField`, because only it holds their
    /// record and their client. So the warp crosses as a fact - "you are going there, and
    /// this is why" - exactly as EXP and a party buff do, rather than as bytes somebody
    /// else assembled on their behalf.
    PartyQuestEnter { map: u32, why: String },

    /// A **party buff** `caster` just put on themselves reaches this character too:
    /// `skill_id` at `level`, cast while both stood on one field.
    ///
    /// The owner, 2026-09-06: *"party buffs should apply to everyone in the party who is in the
    /// same map."* The recipient builds its own `0x007D` from its own tables and records the
    /// expiry in its own `buffs`, so the `0x007E` that has to follow comes from the session
    /// that owns the client. That is why a stat crosses as a fact and not as bytes: a buff's
    /// value can depend on the wearer (Iron Body's percent resolves against the wearer's own
    /// defence), and only the recipient can say what it is worth to them. No MP is spent and
    /// no cooldown is stamped on the recipient - those were the caster's.
    PartyBuff { skill_id: u32, level: u32, caster: u32 },

    /// A Cleric's **Heal** cast by `caster` on this field reaches this character: `percent`
    /// of the recipient's own drawn ceiling, computed on the recipient's session for the same
    /// reason a party buff crosses as a fact - only the owner knows its own maximum, and only
    /// the owning session can move that client's bar. `Session::heal_cast`.
    PartyHeal { percent: u32, caster: u32 },

    /// A GM queued a gift for this character while they are on this channel: open the
    /// Administrator's box now rather than at their next field entry. Carries nothing - the
    /// row in `store::gifts` is the gift, and the receiving session reads it.
    GiftDrop,

    /// **Somebody's friend list changed** - they were asked, answered, or removed.
    ///
    /// Carries nothing: the receiving session reads its own rows out of the store and builds
    /// its own `0x00A7`, for the same reason `GiftDrop` carries nothing. Only that session
    /// can say what its client should now draw. `world::session::friends`.
    FriendRequest,
}

/// One queued packet and whether a newer one may replace it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Queued {
    reply: Reply,
    /// `Some(character)` means: a later broadcast with this same opcode about this
    /// same character supersedes this one while it is still unsent. See the module
    /// docs - this is what bounds a stalled mailbox.
    supersedes: Option<u32>,
}

#[derive(Debug, Default)]
struct Mailbox {
    /// `None` until this connection has entered a field. A connection at character
    /// select, or one that has just been accepted, is subscribed to nothing.
    presence: Option<Presence>,
    queue: Vec<Queued>,
    /// The second channel, and it has a **different lifetime from `queue`**: a packet
    /// is owed to a place and an event is owed to a person. `enter_field` clears the
    /// one and not the other. Nothing supersedes here - two shares of two kills are
    /// two shares.
    events: Vec<Event>,
}

#[derive(Debug, Default)]
struct Inner {
    next: u64,
    boxes: HashMap<SubscriberId, Mailbox>,
}

/// Every live connection on this channel, and what each of them is owed.
///
/// One per channel process, hung off [`crate::fields::Fields`] so it reaches every
/// session through the `Arc` that was already being threaded there.
#[derive(Debug, Default)]
pub struct Bus {
    inner: Mutex<Inner>,
}

impl Bus {
    pub fn new() -> Self {
        Self::default()
    }

    /// A connection was accepted. Give it a mailbox.
    pub fn join(&self) -> SubscriberId {
        let mut inner = self.lock();
        inner.next += 1;
        let id = SubscriberId(inner.next);
        inner.boxes.insert(id, Mailbox::default());
        id
    }

    /// A connection ended, for any reason. Returns nothing to the caller - the point
    /// is the fan-out, and the caller is usually `Drop` and cannot send anyway.
    ///
    /// Idempotent: a session that left its field cleanly and is then dropped calls
    /// this with nothing left to announce, and that must not announce a second
    /// departure. `leave_field` clears the presence, so the second call is a no-op.
    ///
    /// **Undelivered [`Event`]s die here, and that is correct - checked, not assumed.**
    /// The `remove` below takes the whole `Mailbox`, `events` included, so no code was
    /// added for this. The connection is gone: there is nobody left to compute a new
    /// total for, and an EXP share that outlived its session would be applied to
    /// whichever connection next claimed that character - i.e. handed out twice. Pinned
    /// by `parting_drops_undelivered_events`.
    pub fn part(&self, id: SubscriberId) {
        let mut inner = self.lock();
        let gone = inner.boxes.remove(&id).and_then(|m| m.presence);
        if let Some(p) = gone {
            inner.post(id, p.map, p.farewell, None);
        }
    }

    /// This connection has entered `presence.map`, dressed as `presence.spawn` says.
    ///
    /// Does **both halves in one call**, deliberately: it publishes this player to
    /// everyone already on the map, and returns everyone already on the map so the
    /// caller can send them to this player. Splitting them into two calls is how one
    /// side of a mutual sighting goes missing - A sees B, B does not see A - and that
    /// asymmetry is invisible on one screen.
    ///
    /// Handles a map change too: if this connection was already present somewhere,
    /// its farewell goes to the old map first. Walking a portal is `leave` then
    /// `enter`, and doing it in one call means it cannot half-happen.
    ///
    /// Re-entering the **same** map is still a full leave and re-enter, because that
    /// is what the client does: a `SetField` tears the user pool down.
    pub fn enter_field(&self, id: SubscriberId, presence: Presence) -> Vec<Reply> {
        let mut inner = self.lock();
        if let Some(old) = inner.boxes.get_mut(&id).and_then(|m| m.presence.take()) {
            inner.post(id, old.map, old.farewell, None);
        }

        // Spawn first, then whatever travels with them, per player - the order the
        // receiving client needs, since a companion names a character the spawn creates.
        let here: Vec<Reply> = inner
            .boxes
            .iter()
            .filter(|(other, _)| **other != id)
            .filter_map(|(_, m)| m.presence.as_ref())
            .filter(|p| p.map == presence.map)
            .flat_map(|p| std::iter::once(p.spawn.clone()).chain(p.companions.iter().cloned()))
            .collect();

        let map = presence.map;
        let spawn = presence.spawn.clone();
        let companions = presence.companions.clone();
        if let Some(mine) = inner.boxes.get_mut(&id) {
            mine.presence = Some(presence);
            // Anything queued for the old field is addressed to objects the client
            // has just destroyed. A `SetField` empties the user pool, so a movement
            // packet for a character on the map we just left names an id the pool no
            // longer has - `research/user-chat-round2.md` records that the pool drops
            // an unknown id in silence, but a stale *spawn* would leave a character
            // standing on the new map who is not there.
            mine.queue.clear();
            // `mine.events` is deliberately NOT cleared, and the reasoning above does
            // not reach it. A queued packet is addressed to a *place* - it names an
            // object id in a user pool that a `SetField` has just emptied. An `Event`
            // is addressed to a *person*: it carries no object id, no map and no
            // packet, only "you earned N". A contributor can land a hit and walk
            // through a portal before the mob dies, and they have still earned it.
            // Clearing here would be a refusal reported to nobody - the share would
            // vanish with no error and no log line, which is precisely the failure
            // CLAUDE.md describes under "a guard whose answer is ignored", wearing a
            // portal. Pinned by `events_survive_a_field_change_while_queued_packets_do_not`.
        } else {
            // No mailbox: this connection has already parted. Do not resurrect it,
            // and do not announce it.
            return Vec::new();
        }
        inner.post(id, map, spawn, None);
        for c in companions {
            inner.post(id, map, c, None);
        }
        here
    }

    /// This connection left its field and is not in another one yet.
    ///
    /// Used by a channel change and by a log-out - anything that ends the field
    /// without ending the connection. A portal walk should use [`Bus::enter_field`]
    /// instead, which does this as its first act.
    pub fn leave_field(&self, id: SubscriberId) {
        let mut inner = self.lock();
        let gone = inner.boxes.get_mut(&id).and_then(|m| m.presence.take());
        if let Some(p) = gone {
            inner.post(id, p.map, p.farewell, None);
        }
    }

    /// **Everyone in a field on this channel**, as `(character, map)`. The fallback `!online`
    /// uses when no hub is linked, and the source `!track` uses for its own channel.
    pub fn everyone_here(&self) -> Vec<(u32, u32)> {
        let inner = self.lock();
        inner.boxes.values().filter_map(|m| m.presence.as_ref()).map(|p| (p.character, p.map)).collect()
    }

    /// Replace this connection's spawn packet without announcing anything.
    ///
    /// For a change that alters how the character looks to a later arrival - an
    /// equip, a level - where the *live* observers are told by their own packet and
    /// only the spawn used for future arrivals is now stale. Sending a second spawn
    /// to people who already have this character is the case
    /// `research/user-enter-field.md` has to answer before it would be safe.
    pub fn refresh_spawn(&self, id: SubscriberId, spawn: Reply) {
        let mut inner = self.lock();
        if let Some(p) = inner.boxes.get_mut(&id).and_then(|m| m.presence.as_mut()) {
            p.spawn = spawn;
        }
    }

    /// Replace what travels with this connection's player - see [`Presence::companions`] -
    /// without announcing anything. The live observers were told by the summon or the
    /// put-away itself; this is for whoever arrives next.
    pub fn set_companions(&self, id: SubscriberId, companions: Vec<Reply>) {
        let mut inner = self.lock();
        if let Some(p) = inner.boxes.get_mut(&id).and_then(|m| m.presence.as_mut()) {
            p.companions = companions;
        }
    }

    /// Send `reply` to everyone on `map` **except** `from`.
    ///
    /// `supersedes` is the character the packet is about, when a later packet with the
    /// same opcode about that character should replace this one rather than queue
    /// behind it. Pass `Some(character)` for movement; pass `None` for anything that
    /// is an event rather than a state - an attack, a chat line, a level-up effect.
    /// See the module docs.
    ///
    /// A publisher who is not on `map`, or not subscribed at all, is still allowed to
    /// publish: a GM command and a mob's death both legitimately speak about a field.
    pub fn publish(
        &self,
        from: SubscriberId,
        map: u32,
        reply: Reply,
        supersedes: Option<u32>,
    ) {
        self.lock().post(from, map, reply, supersedes);
    }

    /// Hand one [`Event`] to whichever connection is playing `character`.
    ///
    /// Returns **whether anyone was listening**.
    ///
    /// `false` is the ORDINARY case, not an error and **not retryable**. The other
    /// player may have logged out between landing a hit and the mob dying; that is a
    /// completely normal thing to do and the share is simply not owed to anyone. A
    /// caller that treats `false` as a failure - retrying it, logging it as an error,
    /// or refusing to pay out the other contributors - is wrong. Log it at most as a
    /// count, and carry on.
    ///
    /// **Two things it cannot see, said plainly rather than implied:**
    ///
    /// * A connection whose `presence` is `None` does not receive, because there is no
    ///   character id to match on. `leave_field` clears the presence and keeps the
    ///   mailbox, so a character sitting at character select, or mid-channel-change, is
    ///   indistinguishable here from one that has gone. Both give `false`.
    /// * It matches on the **character**, not the connection. If the same character
    ///   were somehow present twice - a duplicate login that the login server is
    ///   supposed to prevent - every match is delivered to, and this returns `true`.
    ///   It does not pick a winner, because it has no basis to.
    ///
    /// Nothing supersedes: two shares from two kills are two events, for the same
    /// reason two swings are two packets.
    ///
    /// # There is no `from`, so this does not exclude the caller
    ///
    /// [`Bus::publish`] takes a `from` and skips it. This cannot: it is addressed by
    /// character, and the bus has no idea which character the caller is. **A killing
    /// session that loops over every contributor and calls this for each one will send
    /// itself its own share** - and if it also awards itself directly, it pays twice.
    /// Skip your own character at the call site; the bus cannot do it for you.
    /// **A finished packet to one character, on one map.** Returns whether anybody got it.
    ///
    /// The third channel, and it exists because the other two cannot express this: [`publish`]
    /// goes to a whole map, and [`send_to_character`] carries an [`Event`] - a *fact* - and
    /// deliberately not bytes, so that the bus keeps knowing no packet layouts.
    ///
    /// # What needed it
    ///
    /// Drops. The owner, 2026-09-01: *"the drops can remain per client. If multiple clients hit the
    /// mob, the one who dealt the most damage (without counting over-damage) will see the
    /// drops."* **The top damager is very often not the connection that landed the killing
    /// blow**, and the killer's session is the one holding the finished `0x046E`. Without this
    /// the choice is to send the drop to the wrong player or not at all.
    ///
    /// # `map` is not optional, and that is the point of taking it
    ///
    /// A drop packet names an object id in a *field's* drop pool. Delivering one to a
    /// character who has walked through a portal since the mob died would put an item on a map
    /// it was never dropped on - and the client would draw it, because a drop's enter packet
    /// carries its own coordinates and asks the pool no questions. So the map is matched as
    /// well as the character, and a recipient who has left is simply not a recipient. That is
    /// the same reasoning `enter_field` uses when it clears `queue` on a field change and the
    /// reason it deliberately does *not* clear `events`.
    ///
    /// **Not supersedable.** Two drops are two items; coalescing them would silently lose one.
    pub fn publish_to_character(&self, character: u32, map: u32, reply: Reply) -> bool {
        let mut inner = self.lock();
        let Some(id) = inner.boxes.iter().find_map(|(id, m)| {
            let p = m.presence.as_ref()?;
            (p.character == character && p.map == map).then_some(*id)
        }) else {
            return false;
        };
        // `post` is the same path `publish` uses, so a packet addressed to one person and one
        // addressed to a map cannot get out of order with each other in a mailbox.
        inner.post_to(id, reply);
        true
    }

    /// **A finished packet to everyone standing on a map**, excluding nobody. Returns how
    /// many mailboxes took it; zero is ordinary (an empty field).
    ///
    /// The map-wide form of [`Bus::publish_to_character`], and unlike [`Bus::publish`] it
    /// has no publisher to leave out: it exists for a packet that no session produced on its
    /// own behalf - a drop's expiry fade, swept from the shared table by whichever session
    /// ticked first and owed to every screen the drop was on (`crate::drops::Addressed`).
    /// Same path as `publish`, so it cannot get out of order with a map broadcast in a
    /// mailbox. Not supersedable, for the same reason `publish_to_character` is not.
    pub fn publish_to_map(&self, map: u32, reply: Reply) -> usize {
        let mut inner = self.lock();
        let ids: Vec<SubscriberId> = inner
            .boxes
            .iter()
            .filter_map(|(id, m)| (m.presence.as_ref()?.map == map).then_some(*id))
            .collect();
        for id in &ids {
            inner.post_to(*id, reply.clone());
        }
        ids.len()
    }

    /// **A finished packet to one character, wherever on this channel they are.** Returns
    /// whether anybody got it.
    ///
    /// [`Bus::publish_to_character`] matches the map as well, because a drop belongs to a
    /// field. A party invite does not: it is owed to a person, on whatever map they are
    /// standing, and a target who walked through a portal between the leader typing the name
    /// and the packet being built is still the target. Same-channel only, like everything on
    /// this bus - an invite to a character on another channel process is not delivered, and
    /// the `false` is the caller's cue to say so.
    pub fn publish_to_character_anywhere(&self, character: u32, reply: Reply) -> bool {
        let mut inner = self.lock();
        let Some(id) = inner.boxes.iter().find_map(|(id, m)| {
            let p = m.presence.as_ref()?;
            (p.character == character).then_some(*id)
        }) else {
            return false;
        };
        inner.post_to(id, reply);
        true
    }

    /// Whether `character` is playing on this channel right now - has a mailbox with a
    /// presence. The party's successor choice asks this (and the hub's directory for the
    /// other channels) so a crown never goes to someone who is offline.
    pub fn character_online(&self, character: u32) -> bool {
        self.lock().boxes.values().any(|m| m.presence.as_ref().is_some_and(|p| p.character == character))
    }

    /// **Every character playing on this channel right now** - one per connection with a
    /// presence. For `!giftall`, which has one box to open per screen.
    pub fn online_characters(&self) -> Vec<u32> {
        self.lock().boxes.values().filter_map(|m| m.presence.as_ref().map(|p| p.character)).collect()
    }

    /// **Which of `characters` are playing on `map` right now.** Order preserved, duplicates
    /// in the input preserved, so a caller can keep a party's seat order.
    ///
    /// This is the presence query the EXP and quest split need *before* they pay anyone: the
    /// share is divided by how many party members are on the killer's field, and that count
    /// has to be known before the first payment. `publish_to_character` answers "did it land"
    /// after the fact, which is one payment too late.
    pub fn characters_on(&self, map: u32, characters: &[u32]) -> Vec<u32> {
        let inner = self.lock();
        let here: std::collections::HashSet<u32> = inner
            .boxes
            .values()
            .filter_map(|m| m.presence.as_ref())
            .filter(|p| p.map == map)
            .map(|p| p.character)
            .collect();
        characters.iter().copied().filter(|c| here.contains(c)).collect()
    }

    /// [`Bus::send_to_character`], named for the call sites that read better that way.
    pub fn publish_event_to_character(&self, character: u32, event: Event) -> bool {
        self.send_to_character(character, event)
    }

    pub fn send_to_character(&self, character: u32, event: Event) -> bool {
        let mut inner = self.lock();
        let mut delivered = false;
        for mailbox in inner.boxes.values_mut() {
            match mailbox.presence.as_ref() {
                Some(p) if p.character == character => {}
                _ => continue,
            }
            mailbox.events.push(event.clone());
            delivered = true;
        }
        delivered
    }

    /// Everything this connection is owed, oldest first. Clears the mailbox.
    ///
    /// Packets only. [`Bus::drain_events`] is a separate channel and this does not
    /// touch it - a session must call both.
    pub fn drain(&self, id: SubscriberId) -> Vec<Reply> {
        let mut inner = self.lock();
        match inner.boxes.get_mut(&id) {
            Some(m) => std::mem::take(&mut m.queue).into_iter().map(|q| q.reply).collect(),
            None => Vec::new(),
        }
    }

    /// Every fact this connection has been handed since it last looked. Clears them.
    ///
    /// Oldest first, and the caller must apply them in that order: two EXP awards
    /// against one record are two `save_character_progress` calls, and the second one's
    /// total depends on the first.
    ///
    /// Separate from [`Bus::drain`] on purpose - draining packets does not drain
    /// events and vice versa, so a session that forgets one of the two calls fails
    /// loudly on that feature rather than quietly on both. An unknown `id` gives an
    /// empty `Vec`, the same as `drain`.
    pub fn drain_events(&self, id: SubscriberId) -> Vec<Event> {
        let mut inner = self.lock();
        match inner.boxes.get_mut(&id) {
            Some(m) => std::mem::take(&mut m.events),
            None => Vec::new(),
        }
    }

    /// Which map this connection is on, if it is on one.
    pub fn map_of(&self, id: SubscriberId) -> Option<u32> {
        self.lock().boxes.get(&id).and_then(|m| m.presence.as_ref()).map(|p| p.map)
    }

    /// How many other characters are on `map` right now, not counting `id`.
    ///
    /// For a log line and for tests. A server that says "2 players on map 104040000"
    /// at the moment it broadcasts is the cheapest possible check that the fan-out
    /// went to the right set, and this project's usual failure is a delivery that
    /// looks fine because nobody counted the recipients.
    pub fn others_on(&self, id: SubscriberId, map: u32) -> usize {
        self.lock()
            .boxes
            .iter()
            .filter(|(other, _)| **other != id)
            .filter_map(|(_, m)| m.presence.as_ref())
            .filter(|p| p.map == map)
            .count()
    }

    /// **The subscriber behind a raw id, if that connection is still here.**
    ///
    /// `crate::mobshare` stores a `SessionId`, which is a bare `u64` taken from a
    /// `SubscriberId`. Addressing that connection again needs the typed handle back, and it
    /// has to be able to fail: a mob's holder may have dropped between claiming it and
    /// somebody else hitting it, and a release aimed at a departed mailbox must be a no-op
    /// rather than a panic or a silent send into nothing.
    pub fn subscriber_of(&self, id: u64) -> Option<SubscriberId> {
        let inner = self.lock();
        inner.boxes.keys().find(|s| s.get() == id).copied()
    }

    /// **Somebody else still standing on this map**, or `None` if nobody is.
    ///
    /// Exists for one caller: a controller leaving a field has to give its mobs to a
    /// connection that is still there, and the registry deals in session ids while the only
    /// thing that knows who is *present* is this bus.
    ///
    /// The choice among several is arbitrary and deliberately so - it is the lowest id, which
    /// is stable and reproducible in a test. Nothing about the mobs makes one observer a
    /// better controller than another; what matters is that exactly one is picked.
    pub fn successor_on(&self, map: u32, except: SubscriberId) -> Option<SubscriberId> {
        self.lock()
            .boxes
            .iter()
            .filter(|(other, _)| **other != except)
            .filter(|(_, m)| m.presence.as_ref().is_some_and(|p| p.map == map))
            .map(|(id, _)| *id)
            .min()
    }

    /// A finished packet to one **connection**, chosen by the caller. Returns whether it
    /// landed.
    ///
    /// [`publish_to_character`] addresses a person and checks their map; this addresses a
    /// mailbox that has already been picked, which is what a handover needs - the successor
    /// was chosen *because* of where it is, so re-deriving that would be asking the same
    /// question twice and getting a different answer if it moved in between.
    pub fn publish_to_subscriber(&self, id: SubscriberId, reply: Reply) -> bool {
        let mut inner = self.lock();
        if !inner.boxes.contains_key(&id) {
            return false;
        }
        inner.post_to(id, reply);
        true
    }

    /// How many connections hold a mailbox. Logging and tests.
    pub fn subscribers(&self) -> usize {
        self.lock().boxes.len()
    }

    /// A poisoned bus is not a reason to kill a channel: the data behind the lock is
    /// a queue of packets, and the worst a panicking publisher can leave is a
    /// half-appended `Vec`, which is still a well-formed `Vec`. Same call this crate's
    /// `Fields` already makes.
    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }
}

impl Inner {
    /// The fan-out itself. Called with the lock already held.
    /// [`post`], to one mailbox that has already been chosen.
    ///
    /// Never supersedes: the only caller is [`Bus::publish_to_character`], whose packets are
    /// drops, and two drops are two items. It shares `Queued` with the map-wide path so a
    /// packet addressed to a person and one addressed to a place cannot get out of order in
    /// the same mailbox.
    fn post_to(&mut self, id: SubscriberId, reply: Reply) {
        if let Some(mailbox) = self.boxes.get_mut(&id) {
            mailbox.queue.push(Queued { reply, supersedes: None });
        }
    }

    fn post(&mut self, from: SubscriberId, map: u32, reply: Reply, supersedes: Option<u32>) {
        for (id, mailbox) in self.boxes.iter_mut() {
            if *id == from {
                continue;
            }
            match &mailbox.presence {
                Some(p) if p.map == map => {}
                // Not on this map, or not in a field at all. A connection sitting at
                // the migration hello must not accumulate a backlog it will be sent
                // the moment it arrives somewhere.
                _ => continue,
            }
            let queued = Queued { reply: reply.clone(), supersedes };
            match supersedes.and_then(|c| {
                mailbox
                    .queue
                    .iter_mut()
                    .find(|q| q.supersedes == Some(c) && q.reply.opcode == reply.opcode)
            }) {
                Some(slot) => *slot = queued,
                None => mailbox.queue.push(queued),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reply(opcode: u16, what: &str) -> Reply {
        Reply { opcode, body: what.as_bytes().to_vec(), what: what.to_string() }
    }

    fn presence(character: u32, map: u32) -> Presence {
        Presence {
            character,
            map,
            spawn: reply(0x0224, &format!("spawn {character}")),
            companions: Vec::new(),
            farewell: reply(0x0225, &format!("farewell {character}")),
        }
    }

    fn whats(replies: &[Reply]) -> Vec<String> {
        replies.iter().map(|r| r.what.clone()).collect()
    }

    /// **A successor is somebody on THIS map**, and never the connection asking.
    ///
    /// Its one caller is a departing mob controller looking for someone to hand its monsters
    /// to. Picking a connection on the wrong map would grant control of a mob that client
    /// cannot see, and picking the leaver would grant it to a client whose mob pool is gone -
    /// both silent, because a `0x03D2` naming an unknown object is dropped by the client
    /// without complaint (`crate::mobshare`).
    ///
    /// Every session test for the handover runs on one map, which is why this one is here.
    #[test]
    fn a_successor_is_somebody_else_standing_on_the_same_map() {
        let bus = Bus::new();
        let (a, b, elsewhere, nowhere) = (bus.join(), bus.join(), bus.join(), bus.join());
        bus.enter_field(a, presence(200, 104_040_000));
        bus.enter_field(b, presence(201, 104_040_000));
        bus.enter_field(elsewhere, presence(202, 100_000_000));
        // `nowhere` has a mailbox and has never entered a field - a connection at character
        // select. It must not be handed anything.

        assert_eq!(bus.successor_on(104_040_000, a), Some(b), "the other one on this map");
        assert_eq!(bus.successor_on(104_040_000, b), Some(a), "and it works both ways");
        assert_eq!(
            bus.successor_on(100_000_000, elsewhere),
            None,
            "alone on its own map, so there is nobody to hand to"
        );
        assert_eq!(bus.successor_on(999, a), None, "a map nobody is on has no successor");
        assert_eq!(
            bus.successor_on(104_040_000, nowhere),
            Some(a),
            "the control: this map DOES have candidates, so the None answers above are about \
             the map and not about the bus being empty"
        );

        // A departure removes the candidate, which is the case the handover actually hits.
        bus.leave_field(b);
        assert_eq!(bus.successor_on(104_040_000, a), None, "b left, and a cannot pick itself");
    }

    /// A grant addressed to a mailbox that is gone says so rather than vanishing.
    ///
    /// The handover picks a successor and then sends to it, and the two are not atomic. If
    /// that connection dropped in between, the caller has to know the grant did not land -
    /// otherwise it logs a handover that never happened and the mobs are stranded silently.
    #[test]
    fn publishing_to_a_departed_subscriber_reports_that_it_did_not_land() {
        let bus = Bus::new();
        let a = bus.join();
        bus.enter_field(a, presence(200, 104_040_000));
        assert!(bus.publish_to_subscriber(a, reply(0x03D2, "grant")), "the control: it lands");
        assert_eq!(whats(&bus.drain(a)), vec!["grant"]);

        bus.part(a);
        assert!(!bus.publish_to_subscriber(a, reply(0x03D2, "grant")), "the mailbox is gone");
    }

    /// The whole point, in one test: two connections on one map see each other, and
    /// **both** directions work. One-directional sighting is the failure this bus is
    /// shaped to make impossible, so it is the first thing asserted.
    #[test]
    fn two_players_on_one_map_each_learn_about_the_other() {
        let bus = Bus::new();
        let a = bus.join();
        let b = bus.join();

        // A arrives first, to an empty map.
        assert!(bus.enter_field(a, presence(200, 104_040_000)).is_empty());
        assert!(bus.drain(a).is_empty(), "nobody to hear A yet");

        // B arrives and is handed A.
        let seen_by_b = bus.enter_field(b, presence(201, 104_040_000));
        assert_eq!(whats(&seen_by_b), vec!["spawn 200"]);

        // ...and A is told about B without having asked.
        assert_eq!(whats(&bus.drain(a)), vec!["spawn 201"]);
        assert!(bus.drain(a).is_empty(), "a drain empties the mailbox");
    }

    #[test]
    fn a_different_map_hears_nothing() {
        let bus = Bus::new();
        let a = bus.join();
        let b = bus.join();
        bus.enter_field(a, presence(200, 104_040_000));

        let seen_by_b = bus.enter_field(b, presence(201, 100_000_000));
        assert!(seen_by_b.is_empty(), "different map");
        assert!(bus.drain(a).is_empty(), "different map");

        bus.publish(b, 100_000_000, reply(0x02A5, "b was hit"), None);
        assert!(bus.drain(a).is_empty());
    }

    #[test]
    fn a_publisher_never_hears_itself() {
        let bus = Bus::new();
        let a = bus.join();
        let b = bus.join();
        bus.enter_field(a, presence(200, 1));
        bus.enter_field(b, presence(201, 1));
        let _ = bus.drain(a);
        let _ = bus.drain(b);

        bus.publish(a, 1, reply(0x02A5, "a swung"), None);
        assert!(bus.drain(a).is_empty(), "a session must not receive its own broadcast");
        assert_eq!(whats(&bus.drain(b)), vec!["a swung"]);
    }

    /// A connection that has a mailbox but has not entered a field must not collect a
    /// backlog. It would otherwise be flushed the instant it arrived somewhere,
    /// carrying packets about a map it is not on.
    #[test]
    fn a_connection_not_in_a_field_collects_nothing() {
        let bus = Bus::new();
        let a = bus.join();
        let waiting = bus.join();
        bus.enter_field(a, presence(200, 1));

        bus.publish(a, 1, reply(0x02A5, "a swing"), None);
        assert!(bus.drain(waiting).is_empty());

        // And it still gets a clean arrival afterwards.
        let seen = bus.enter_field(waiting, presence(201, 1));
        assert_eq!(whats(&seen), vec!["spawn 200"]);
        assert!(bus.drain(waiting).is_empty(), "no backlog from before it arrived");
    }

    #[test]
    fn leaving_the_field_tells_everyone_still_on_it() {
        let bus = Bus::new();
        let a = bus.join();
        let b = bus.join();
        bus.enter_field(a, presence(200, 1));
        bus.enter_field(b, presence(201, 1));
        let _ = bus.drain(a);

        bus.leave_field(b);
        assert_eq!(whats(&bus.drain(a)), vec!["farewell 201"]);
    }

    /// The case the `Drop` path exists for: a client that is killed, or a socket that
    /// dies, never sends a log-out. If only the clean exit announced a departure, the
    /// dirty one would leave a character standing on the field forever.
    #[test]
    fn a_dropped_connection_still_says_goodbye() {
        let bus = Bus::new();
        let a = bus.join();
        let b = bus.join();
        bus.enter_field(a, presence(200, 1));
        bus.enter_field(b, presence(201, 1));
        let _ = bus.drain(a);

        bus.part(b);
        assert_eq!(whats(&bus.drain(a)), vec!["farewell 201"]);
        assert_eq!(bus.subscribers(), 1);
    }

    /// A clean log-out followed by the socket closing is the ordinary path, and it
    /// must announce **one** departure, not two.
    #[test]
    fn leaving_then_parting_says_goodbye_once() {
        let bus = Bus::new();
        let a = bus.join();
        let b = bus.join();
        bus.enter_field(a, presence(200, 1));
        bus.enter_field(b, presence(201, 1));
        let _ = bus.drain(a);

        bus.leave_field(b);
        bus.part(b);
        assert_eq!(whats(&bus.drain(a)), vec!["farewell 201"]);
    }

    /// Walking a portal is a leave and an enter, and both halves have to reach the
    /// right map. The bug this pins is the one where the old map keeps a ghost.
    #[test]
    fn a_portal_walk_leaves_the_old_map_and_joins_the_new_one() {
        let bus = Bus::new();
        let stays = bus.join();
        let walker = bus.join();
        let over_there = bus.join();
        bus.enter_field(stays, presence(200, 1));
        bus.enter_field(over_there, presence(202, 2));
        bus.enter_field(walker, presence(201, 1));
        let _ = bus.drain(stays);
        let _ = bus.drain(over_there);

        let seen = bus.enter_field(walker, presence(201, 2));

        assert_eq!(whats(&seen), vec!["spawn 202"], "the new map's occupants");
        assert_eq!(whats(&bus.drain(stays)), vec!["farewell 201"], "the old map");
        assert_eq!(whats(&bus.drain(over_there)), vec!["spawn 201"], "the new map");
    }

    /// A `SetField` empties the client's user pool, so anything queued about the map
    /// we are leaving is addressed to objects that will not exist.
    #[test]
    fn entering_a_field_discards_mail_about_the_old_one() {
        let bus = Bus::new();
        let a = bus.join();
        let mover = bus.join();
        bus.enter_field(a, presence(200, 1));
        bus.enter_field(mover, presence(201, 1));

        bus.publish(a, 1, reply(0x02A5, "a swing on map 1"), None);
        let seen = bus.enter_field(mover, presence(201, 2));

        assert!(seen.is_empty(), "map 2 is empty");
        assert!(bus.drain(mover).is_empty(), "the map-1 swing must not follow it over");
    }

    /// Movement supersedes: a stalled observer accumulates one pending move per
    /// character, not a replay of every step. See the module docs for why a size cap
    /// is the wrong tool - it cannot avoid dropping a spawn and leaving a ghost.
    #[test]
    fn a_newer_move_replaces_an_unsent_older_one() {
        let bus = Bus::new();
        let watcher = bus.join();
        let mover = bus.join();
        bus.enter_field(watcher, presence(200, 1));
        bus.enter_field(mover, presence(201, 1));
        let _ = bus.drain(watcher);

        for step in 0..50 {
            bus.publish(mover, 1, reply(0x02B0, &format!("move to {step}")), Some(201));
        }

        let mail = bus.drain(watcher);
        assert_eq!(whats(&mail), vec!["move to 49"], "only the latest position survives");
    }

    #[test]
    fn superseding_is_per_character_and_per_opcode() {
        let bus = Bus::new();
        let watcher = bus.join();
        let one = bus.join();
        let two = bus.join();
        bus.enter_field(watcher, presence(200, 1));
        bus.enter_field(one, presence(201, 1));
        bus.enter_field(two, presence(202, 1));
        let _ = bus.drain(watcher);

        bus.publish(one, 1, reply(0x02B0, "201 moves"), Some(201));
        bus.publish(two, 1, reply(0x02B0, "202 moves"), Some(202));
        // Same character, different opcode - a different kind of event entirely.
        bus.publish(one, 1, reply(0x02A5, "201 is hit"), Some(201));
        bus.publish(one, 1, reply(0x02B0, "201 moves again"), Some(201));

        assert_eq!(
            whats(&bus.drain(watcher)),
            vec!["201 moves again", "202 moves", "201 is hit"],
            "two characters and two opcodes are four independent slots, and a \
             superseded entry keeps its place in the queue"
        );
    }

    /// The other half of the same rule: an attack is an event, and two swings are two
    /// swings. If this ever coalesced, a fight would render as one hit.
    #[test]
    fn events_published_without_a_supersede_key_all_survive() {
        let bus = Bus::new();
        let watcher = bus.join();
        let attacker = bus.join();
        bus.enter_field(watcher, presence(200, 1));
        bus.enter_field(attacker, presence(201, 1));
        let _ = bus.drain(watcher);

        for n in 0..3 {
            bus.publish(attacker, 1, reply(0x02B2, &format!("swing {n}")), None);
        }
        assert_eq!(whats(&bus.drain(watcher)), vec!["swing 0", "swing 1", "swing 2"]);
    }

    #[test]
    fn counting_the_recipients() {
        let bus = Bus::new();
        let a = bus.join();
        let b = bus.join();
        let c = bus.join();
        bus.enter_field(a, presence(200, 1));
        bus.enter_field(b, presence(201, 1));
        bus.enter_field(c, presence(202, 2));

        assert_eq!(bus.others_on(a, 1), 1, "b is here, a does not count itself");
        assert_eq!(bus.others_on(c, 2), 0);
        assert_eq!(bus.subscribers(), 3);
        assert_eq!(bus.map_of(a), Some(1));
        bus.leave_field(a);
        assert_eq!(bus.map_of(a), None);
        assert_eq!(bus.others_on(b, 1), 0);
    }

    /// A session that has parted must not be resurrected by a late `enter_field` from
    /// its own thread, and must not announce an arrival nobody can see the end of.
    #[test]
    fn entering_after_parting_does_nothing() {
        let bus = Bus::new();
        let a = bus.join();
        let gone = bus.join();
        bus.enter_field(a, presence(200, 1));
        bus.part(gone);

        assert!(bus.enter_field(gone, presence(201, 1)).is_empty());
        assert!(bus.drain(a).is_empty(), "no arrival from a connection that is gone");
        assert_eq!(bus.subscribers(), 1);
    }

    /// Fan-out is per map and the bus is shared by every thread on the channel, so
    /// the interesting property is that concurrent publishing loses nothing. A
    /// counted total is the check: 4 publishers x 25 packets, each seen by the other
    /// four subscribers.
    #[test]
    fn concurrent_publishers_lose_nothing() {
        use std::sync::Arc;

        let bus = Arc::new(Bus::new());
        let watcher = bus.join();
        bus.enter_field(watcher, presence(200, 7));

        let publishers: Vec<SubscriberId> = (0..4)
            .map(|n| {
                let id = bus.join();
                bus.enter_field(id, presence(300 + n, 7));
                id
            })
            .collect();
        // The five arrivals the watcher was told about; not what this test is measuring.
        let arrivals = bus.drain(watcher).len();
        assert_eq!(arrivals, 4);

        let handles: Vec<_> = publishers
            .into_iter()
            .map(|id| {
                let bus = bus.clone();
                std::thread::spawn(move || {
                    for n in 0..25 {
                        bus.publish(id, 7, reply(0x02B2, &format!("swing {n}")), None);
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().expect("a publisher thread panicked");
        }

        assert_eq!(bus.drain(watcher).len(), 100);
    }

    /// **A map-wide post stops at the map's edge.** The owner, 2026-09-18: *"Clients don't need
    /// to be told about things happening on another map if they are not on that map."* Every
    /// field-level packet on this bus is map-scoped - `publish`, `publish_to_character` and
    /// this - and only person-addressed things (party, whisper, an EXP share) may cross a map.
    /// The `nobody else` half is the one worth asserting: a fade that fanned out to the
    /// channel would cost every idle client a packet per expired drop anywhere.
    #[test]
    fn a_map_wide_post_reaches_everyone_on_that_map_and_nobody_off_it() {
        let bus = Bus::new();
        let a = bus.join();
        let b = bus.join();
        let elsewhere = bus.join();
        let nowhere = bus.join();
        bus.enter_field(a, presence(200, 1));
        bus.enter_field(b, presence(201, 1));
        bus.enter_field(elsewhere, presence(202, 2));
        for id in [a, b, elsewhere] {
            let _ = bus.drain(id);
        }
        assert_eq!(bus.publish_to_map(1, reply(0x046F, "fade")), 2, "two on map 1, counted");
        assert_eq!(bus.drain(a).len(), 1);
        assert_eq!(bus.drain(b).len(), 1, "the poster is not excluded - there is none");
        assert!(bus.drain(elsewhere).is_empty(), "map 2 hears nothing");
        assert!(bus.drain(nowhere).is_empty(), "a connection in no field hears nothing");
        assert_eq!(bus.publish_to_map(3, reply(0x046F, "fade")), 0, "an empty map is ordinary");
    }

    // ---------------------------------------------------------------------------
    // The second channel: a fact to a character.
    // ---------------------------------------------------------------------------

    fn exp(amount: u64, why: &str) -> Event {
        Event::Experience { amount, why: why.to_string(), white: true }
    }

    fn amounts(events: &[Event]) -> Vec<u64> {
        events
            .iter()
            .map(|e| match e {
                Event::Experience { amount, .. } => *amount,
                // These tests only ever queue EXP shares; a party buff here is a test bug.
                Event::PartyBuff { skill_id, .. } => panic!("not an EXP share: skill {skill_id}"),
                Event::PartyHeal { caster, .. } => panic!("not an EXP share: heal from {caster}"),
                Event::PartyMesos { picker, .. } => panic!("not an EXP share: mesos from {picker}"),
                Event::GiftDrop => panic!("not an EXP share: a gift drop"),
                Event::PartyQuestEnter { map, .. } => panic!("not an EXP share: a warp to {map}"),
            Event::FriendRequest => panic!("not an EXP share: a friend request"),
            })
            .collect()
    }

    /// Addressed to a character, delivered to that character's connection, and to no
    /// other. The `nobody else` half is the one worth asserting: an EXP share that
    /// fanned out like a packet would credit the whole map.
    #[test]
    fn an_event_reaches_the_named_character_and_nobody_else() {
        let bus = Bus::new();
        let killer = bus.join();
        let helper = bus.join();
        let bystander = bus.join();
        bus.enter_field(killer, presence(200, 1));
        bus.enter_field(helper, presence(201, 1));
        bus.enter_field(bystander, presence(202, 1));

        assert!(bus.send_to_character(201, exp(37, "kill share")), "someone was listening");

        assert_eq!(amounts(&bus.drain_events(helper)), vec![37]);
        assert!(bus.drain_events(killer).is_empty(), "the sender is not a recipient");
        assert!(bus.drain_events(bystander).is_empty(), "the map is not the address");
    }

    /// A character on another map is still reachable - this is not a field broadcast.
    /// A contributor who walked out of the map before the mob died is owed the share,
    /// and that is the whole reason the address is a character rather than a map.
    #[test]
    fn an_event_is_not_scoped_to_a_map() {
        let bus = Bus::new();
        let killer = bus.join();
        let elsewhere = bus.join();
        bus.enter_field(killer, presence(200, 1));
        bus.enter_field(elsewhere, presence(201, 999));

        assert!(bus.send_to_character(201, exp(12, "kill share")));
        assert_eq!(amounts(&bus.drain_events(elsewhere)), vec![12]);
    }

    /// The ordinary miss. `false`, and **nothing** was queued anywhere - neither an
    /// event on some other mailbox nor a stray packet. Both are asserted because a
    /// delivery that goes to the wrong mailbox and a delivery that goes nowhere both
    /// return `false` from a test that only reads the return value.
    #[test]
    fn an_event_for_an_absent_character_returns_false_and_queues_nothing() {
        let bus = Bus::new();
        let a = bus.join();
        let b = bus.join();
        bus.enter_field(a, presence(200, 1));
        bus.enter_field(b, presence(201, 1));
        let _ = bus.drain(a);
        let _ = bus.drain(b);

        // 999 has logged out - or was never here.
        assert!(!bus.send_to_character(999, exp(50, "kill share")), "nobody was listening");

        for who in [a, b] {
            assert!(bus.drain_events(who).is_empty(), "no event went to the wrong box");
            assert!(bus.drain(who).is_empty(), "and no packet was invented either");
        }
    }

    /// **The rule most likely to be broken by a later edit, so it is pinned with both
    /// halves in one test.** `enter_field` clears `queue` and must not clear `events`.
    ///
    /// Asserting only that events survive would pass against a version that had
    /// stopped clearing the queue too - which is a different bug (a stale spawn leaves
    /// a ghost on the new map). The contrast is the measurement, so both are observed
    /// under the same conditions, in the same call.
    #[test]
    fn events_survive_a_field_change_while_queued_packets_do_not() {
        let bus = Bus::new();
        let other = bus.join();
        let walker = bus.join();
        bus.enter_field(other, presence(200, 1));
        bus.enter_field(walker, presence(201, 1));
        let _ = bus.drain(walker);

        // Both channels have something pending for the walker on map 1.
        bus.publish(other, 1, reply(0x02B2, "a swing on map 1"), None);
        assert!(bus.send_to_character(201, exp(80, "kill share")));

        // ...and then the walker takes a portal before draining either.
        let seen = bus.enter_field(walker, presence(201, 2));
        assert!(seen.is_empty(), "map 2 is empty");

        assert!(bus.drain(walker).is_empty(), "the packet named an object map 2 has not");
        assert_eq!(
            amounts(&bus.drain_events(walker)),
            vec![80],
            "the share is owed to a person, not to a place - a portal does not cancel it"
        );
    }

    /// The same rule, in the direction the feature actually needs: hit the mob, walk
    /// out, mob dies afterwards. The share is sent while the contributor is already on
    /// the new map and must land there.
    #[test]
    fn a_share_sent_after_the_contributor_walked_away_still_lands() {
        let bus = Bus::new();
        let killer = bus.join();
        let helper = bus.join();
        bus.enter_field(killer, presence(200, 1));
        bus.enter_field(helper, presence(201, 1));
        let _ = bus.drain(helper);

        bus.enter_field(helper, presence(201, 2));
        let _ = bus.drain(helper);

        assert!(bus.send_to_character(201, exp(80, "kill share")), "still a live character");
        assert_eq!(amounts(&bus.drain_events(helper)), vec![80]);
    }

    /// `part` removes the whole mailbox, so undelivered events go with it. No code was
    /// added for this - the test exists to prove the claim in `part`'s doc comment is
    /// true rather than merely written down.
    #[test]
    fn parting_drops_undelivered_events() {
        let bus = Bus::new();
        let a = bus.join();
        let leaver = bus.join();
        bus.enter_field(a, presence(200, 1));
        bus.enter_field(leaver, presence(201, 1));

        assert!(bus.send_to_character(201, exp(80, "kill share")));
        bus.part(leaver);

        assert!(bus.drain_events(leaver).is_empty(), "a gone mailbox owes nothing");
        assert_eq!(bus.subscribers(), 1);
        // And the character is now unreachable, which is the ordinary `false`.
        assert!(!bus.send_to_character(201, exp(80, "too late")));
        assert!(bus.drain_events(a).is_empty(), "and it did not fall through to a neighbour");
    }

    /// `leave_field` keeps the mailbox and clears the presence. Two consequences, and
    /// the honest one is the second: **already-queued events survive**, but a *new*
    /// event cannot be addressed, because there is no character id left to match on.
    /// That is a real limitation of this design and it is pinned rather than hidden.
    #[test]
    fn leaving_the_field_keeps_owed_events_but_makes_the_character_unaddressable() {
        let bus = Bus::new();
        let a = bus.join();
        let quitter = bus.join();
        bus.enter_field(a, presence(200, 1));
        bus.enter_field(quitter, presence(201, 1));

        assert!(bus.send_to_character(201, exp(80, "earned before leaving")));
        bus.leave_field(quitter);

        // The miss, and it is silent by design.
        assert!(
            !bus.send_to_character(201, exp(90, "earned while at character select")),
            "no presence means no character id to match on"
        );

        assert_eq!(
            amounts(&bus.drain_events(quitter)),
            vec![80],
            "what was already owed is still owed; what arrived after is gone"
        );
    }

    /// The two channels are independent in **both** directions. A session drains both;
    /// if either drain silently emptied the other, one feature would go missing with no
    /// error - and it would be the feature nobody was looking at.
    #[test]
    fn the_two_drains_do_not_touch_each_other() {
        let bus = Bus::new();
        let watcher = bus.join();
        let other = bus.join();
        bus.enter_field(watcher, presence(200, 1));
        bus.enter_field(other, presence(201, 1));
        let _ = bus.drain(watcher);

        bus.publish(other, 1, reply(0x02B2, "a swing"), None);
        assert!(bus.send_to_character(200, exp(15, "kill share")));

        // Packets first: the event must survive it.
        assert_eq!(whats(&bus.drain(watcher)), vec!["a swing"]);
        assert_eq!(amounts(&bus.drain_events(watcher)), vec![15]);

        // Now the other order.
        bus.publish(other, 1, reply(0x02B2, "another swing"), None);
        assert!(bus.send_to_character(200, exp(16, "kill share")));
        assert_eq!(amounts(&bus.drain_events(watcher)), vec![16]);
        assert_eq!(whats(&bus.drain(watcher)), vec!["another swing"]);
    }

    /// `drain_events` clears, and clears completely - a second look owes nothing.
    /// Order is oldest-first, which the caller depends on: two awards against one
    /// record are two saves and the second one's total depends on the first.
    #[test]
    fn drain_events_returns_them_in_order_and_clears() {
        let bus = Bus::new();
        let a = bus.join();
        let earner = bus.join();
        bus.enter_field(a, presence(200, 1));
        bus.enter_field(earner, presence(201, 1));

        for n in 1..=4u64 {
            assert!(bus.send_to_character(201, exp(n * 10, "kill share")));
        }

        assert_eq!(amounts(&bus.drain_events(earner)), vec![10, 20, 30, 40]);
        assert!(bus.drain_events(earner).is_empty(), "a drain empties the events");
    }

    /// Nothing supersedes on this channel. Two shares of two kills are two shares, and
    /// coalescing them would silently halve a party's experience - the same rule that
    /// keeps two swings from rendering as one hit, with money attached.
    #[test]
    fn identical_events_are_not_coalesced() {
        let bus = Bus::new();
        let a = bus.join();
        let earner = bus.join();
        bus.enter_field(a, presence(200, 1));
        bus.enter_field(earner, presence(201, 1));

        for _ in 0..3 {
            assert!(bus.send_to_character(201, exp(25, "kill share")));
        }
        assert_eq!(amounts(&bus.drain_events(earner)), vec![25, 25, 25]);
    }

    /// A connection that has a mailbox but has never entered a field has no character
    /// id, so it cannot be addressed - and, importantly, entering a field afterwards
    /// must not hand it a backlog of somebody else's facts.
    #[test]
    fn a_connection_that_never_entered_a_field_is_unaddressable() {
        let bus = Bus::new();
        let a = bus.join();
        let waiting = bus.join();
        bus.enter_field(a, presence(200, 1));

        assert!(!bus.send_to_character(201, exp(80, "for a character not in a field")));

        let _ = bus.enter_field(waiting, presence(201, 1));
        assert!(bus.drain_events(waiting).is_empty(), "no backlog from before it arrived");
        // ...and now it is reachable.
        assert!(bus.send_to_character(201, exp(80, "kill share")));
        assert_eq!(amounts(&bus.drain_events(waiting)), vec![80]);
    }

    /// `drain_events` on an id that was never issued, or has parted, is empty rather
    /// than a panic - the same contract `drain` already has.
    #[test]
    fn draining_events_for_an_unknown_subscriber_is_empty() {
        let bus = Bus::new();
        let gone = bus.join();
        bus.part(gone);
        assert!(bus.drain_events(gone).is_empty());
    }

    /// The `white` flag and the `why` string cross unchanged. They are the two fields
    /// the receiving session cannot re-derive: whether this was a share or its own
    /// kill, and what to write in the log.
    #[test]
    fn the_event_carries_its_reason_and_its_colour_unchanged() {
        let bus = Bus::new();
        let a = bus.join();
        let earner = bus.join();
        bus.enter_field(a, presence(200, 1));
        bus.enter_field(earner, presence(201, 1));

        assert!(bus.send_to_character(
            201,
            Event::Experience { amount: 7, why: "party share".to_string(), white: false }
        ));
        assert_eq!(
            bus.drain_events(earner),
            vec![Event::Experience { amount: 7, why: "party share".to_string(), white: false }]
        );
    }

    /// **A finished packet to one character, and to nobody else.**
    ///
    /// The channel drops needed: the top damager is often not the connection that landed the
    /// killing blow, and the killer's session is the one holding the built `0x046E`.
    #[test]
    fn a_packet_addressed_to_a_character_reaches_only_them() {
        let bus = Bus::new();
        let killer = bus.join();
        let winner = bus.join();
        let bystander = bus.join();
        bus.enter_field(killer, presence(200, 7));
        bus.enter_field(winner, presence(201, 7));
        bus.enter_field(bystander, presence(202, 7));
        // Clear the arrival mail so what follows is only what this test posted.
        bus.drain(killer);
        bus.drain(winner);
        bus.drain(bystander);

        let drop = Reply { opcode: 0x046E, body: vec![9], what: "a drop for 201".into() };
        assert!(bus.publish_to_character(201, 7, drop.clone()), "201 is on map 7");

        assert_eq!(bus.drain(winner).len(), 1, "the top damager gets it");
        assert!(bus.drain(bystander).is_empty(), "and nobody else on the map does");
        assert!(bus.drain(killer).is_empty(), "including the one who built it");
    }

    /// **A recipient who has walked away is not a recipient**, and the map argument is what
    /// makes that true.
    ///
    /// A drop names an object id in a *field's* pool, and its enter packet carries its own
    /// coordinates - the client draws it without asking the pool anything. Delivering one to
    /// somebody who took a portal between the killing blow and the drop would put an item on a
    /// map it was never dropped on.
    #[test]
    fn a_character_who_left_the_map_is_not_sent_its_drops() {
        let bus = Bus::new();
        let a = bus.join();
        let b = bus.join();
        bus.enter_field(a, presence(200, 7));
        bus.enter_field(b, presence(201, 7));
        bus.drain(a);
        bus.drain(b);

        // 201 walks through a portal to map 9 before the mob finishes dying.
        bus.enter_field(b, presence(201, 9));
        bus.drain(a);
        bus.drain(b);

        let drop = Reply { opcode: 0x046E, body: vec![9], what: "a drop on map 7".into() };
        assert!(
            !bus.publish_to_character(201, 7, drop.clone()),
            "the character is not on map 7 any more, so this must report undelivered"
        );
        assert!(bus.drain(b).is_empty(), "and must post nothing");

        // The control: addressed to where they actually are, it arrives. Without this the
        // test above would pass on a function that never delivers anything.
        assert!(bus.publish_to_character(201, 9, drop), "on their real map it lands");
        assert_eq!(bus.drain(b).len(), 1);
    }

    /// **Two drops are two items.** `publish` may supersede; this must never.
    #[test]
    fn drops_addressed_to_one_character_are_never_coalesced() {
        let bus = Bus::new();
        let a = bus.join();
        bus.enter_field(a, presence(200, 7));
        bus.drain(a);

        for n in 0..3u8 {
            let r = Reply { opcode: 0x046E, body: vec![n], what: format!("drop {n}") };
            assert!(bus.publish_to_character(200, 7, r));
        }
        let out = bus.drain(a);
        assert_eq!(out.len(), 3, "three drops, not one: {out:?}");
        assert_eq!(out[0].what, "drop 0", "and in the order they were posted");
        assert_eq!(out[2].what, "drop 2");
    }

    /// An unknown character is reported undelivered rather than silently swallowed - the
    /// caller has a real packet in hand and needs to know it went nowhere.
    #[test]
    fn a_character_nobody_is_playing_reports_undelivered() {
        let bus = Bus::new();
        let a = bus.join();
        bus.enter_field(a, presence(200, 7));
        let r = Reply { opcode: 0x046E, body: vec![1], what: "nobody".into() };
        assert!(!bus.publish_to_character(999, 7, r));
    }

}
