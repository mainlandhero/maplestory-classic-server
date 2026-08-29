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

        let here: Vec<Reply> = inner
            .boxes
            .iter()
            .filter(|(other, _)| **other != id)
            .filter_map(|(_, m)| m.presence.as_ref())
            .filter(|p| p.map == presence.map)
            .map(|p| p.spawn.clone())
            .collect();

        let map = presence.map;
        let spawn = presence.spawn.clone();
        if let Some(mine) = inner.boxes.get_mut(&id) {
            mine.presence = Some(presence);
            // Anything queued for the old field is addressed to objects the client
            // has just destroyed. A `SetField` empties the user pool, so a movement
            // packet for a character on the map we just left names an id the pool no
            // longer has - `research/user-chat-round2.md` records that the pool drops
            // an unknown id in silence, but a stale *spawn* would leave a character
            // standing on the new map who is not there.
            mine.queue.clear();
        } else {
            // No mailbox: this connection has already parted. Do not resurrect it,
            // and do not announce it.
            return Vec::new();
        }
        inner.post(id, map, spawn, None);
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

    /// Everything this connection is owed, oldest first. Clears the mailbox.
    pub fn drain(&self, id: SubscriberId) -> Vec<Reply> {
        let mut inner = self.lock();
        match inner.boxes.get_mut(&id) {
            Some(m) => std::mem::take(&mut m.queue).into_iter().map(|q| q.reply).collect(),
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
            farewell: reply(0x0225, &format!("farewell {character}")),
        }
    }

    fn whats(replies: &[Reply]) -> Vec<String> {
        replies.iter().map(|r| r.what.clone()).collect()
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
}
