//! What is alive on every map of this channel - shared by every connection on it.
//!
//! # Why this is not on the `Session`
//!
//! It used to be. A `Session` is one connection, so mobs and drops lived and died with the
//! player looking at them: leaving a map forgot every mob's position, and a second player
//! would have seen an empty field beside the first player's full one.
//!
//! The owner set the model, and it is the one a real channel uses:
//!
//! > *"When the user leaves the map, all of the mobs should persist in their current
//! > location. Mob locations are stored per channel instance per map. Upon re-entering the
//! > map (or when a new player joins an existing player in the same map), the saved/current
//! > location for those mobs will be sent to the user and the animations carry on from
//! > there."*
//!
//! So the state is keyed by map and owned by the **channel** - one `Fields` per server
//! process, behind one lock, shared by every `Session`.
//!
//! # A field starts EMPTY and fills up
//!
//! > *"For the map, on first enter, no mobs should exist until the respawn timer kicks in."*
//!
//! That removes a whole code path rather than adding one. There is no "initial spawn": the
//! first time anyone enters a map, every spawn point is registered as **due**, and the
//! ordinary respawn tick fills them in over the following seconds exactly as it does after
//! a kill. One mechanism, used twice.
//!
//! # Positions come from the client, because only the client has them
//!
//! The server grants control of a mob and the client simulates it, reporting each path back
//! as `0x02FF`. So [`Fields::note_position`] is fed from those reports and is the only thing
//! that knows where anything is. A mob that has never moved sits at its spawn point.

use std::collections::HashMap;
use std::sync::Mutex;

use crate::config::Config;

/// What [`Fields::hurt`] did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hurt {
    /// Still up, with this much HP left.
    Alive(u64),
    /// Dead, and this is how the experience should be split - highest share first, exactly
    /// one of them flagged `majority`. Empty only if nothing was ever credited.
    Died(Vec<DamageShare>),
}

/// What one character contributed to a kill.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DamageShare {
    pub character: u32,
    /// Damage that actually landed - never more than the mob had left.
    pub dealt: u64,
    /// Everything that landed, from everyone. Never 0 when a share exists.
    pub total: u64,
    /// The largest contributor, and only ever one of them.
    ///
    /// This is the white-versus-yellow EXP line. The owner, 2026-08-20: *"if I was the person who
    /// dealt majority damage, I should see a white line of EXP gained. If I was not ... that
    /// line would be yellow."*
    pub majority: bool,
}

impl DamageShare {
    /// This character's cut of `exp`, rounded down but never to nothing.
    ///
    /// The majority holder takes the whole amount; everyone else takes their fraction. A
    /// contributor who earned a share is never paid zero - they hit it, and a zero would read
    /// on screen as the share being broken.
    pub fn cut_of(&self, exp: u64) -> u64 {
        if self.majority {
            return exp;
        }
        if exp == 0 || self.total == 0 {
            return 0;
        }
        (exp.saturating_mul(self.dealt) / self.total).max(1)
    }
}

/// One mob that exists right now, and where it is.
#[derive(Debug, Clone)]
pub struct LiveMob {
    /// The spawn point's own record - template, foothold, spawn position, full HP.
    pub spawn: net::mob::FieldMob,
    /// Remaining HP.
    pub hp: u64,
    /// Where the client last reported it. `None` means it has never moved.
    pub at: Option<(i16, i16)>,
    /// Who has hurt this mob, and by how much, **counting only damage that landed**.
    ///
    /// The owner, 2026-08-20: *"over-damage of a mob's HP does not count towards % sharing"* - so
    /// a 500-damage hit on a snail with 3 HP left credits 3, not 500. Without that cap the
    /// last hit would almost always look like the majority share whoever did the work.
    ///
    /// A `Vec` rather than a map: a mob is hit by a handful of people at most, the order is
    /// stable so a tie breaks the same way twice, and stable order is what keeps a packet
    /// built from it identical run to run.
    pub damage_by: Vec<(u32, u64)>,
}

impl LiveMob {
    /// Credit `damage` to `character`, capped at what is actually left to take.
    ///
    /// Returns what was really credited.
    fn credit(&mut self, character: u32, damage: u64) -> u64 {
        let landed = damage.min(self.hp);
        if landed == 0 {
            return 0;
        }
        match self.damage_by.iter_mut().find(|(who, _)| *who == character) {
            Some((_, total)) => *total = total.saturating_add(landed),
            None => self.damage_by.push((character, landed)),
        }
        landed
    }

    /// How the mob's experience should be split, once it is dead.
    ///
    /// Returns `(character, share_of_total, is_majority)` per contributor, highest first.
    /// **Exactly one entry is the majority** - the largest contributor, ties broken by who
    /// hit it first, which the `Vec`'s order gives for free.
    pub fn shares(&self) -> Vec<DamageShare> {
        let total: u64 = self.damage_by.iter().map(|(_, d)| *d).sum();
        if total == 0 {
            return Vec::new();
        }
        let mut ranked: Vec<DamageShare> = self
            .damage_by
            .iter()
            .map(|(who, dealt)| DamageShare {
                character: *who,
                dealt: *dealt,
                total,
                majority: false,
            })
            .collect();
        // Stable sort, so equal damage keeps first-hit order rather than shuffling. Reverse
        // rather than sorting ascending and reading backwards: reversing the ITERATION would
        // also reverse the tie order and quietly hand the majority to whoever hit it last.
        ranked.sort_by_key(|s| std::cmp::Reverse(s.dealt));
        if let Some(first) = ranked.first_mut() {
            first.majority = true;
        }
        ranked
    }

    /// The mob as it should be sent to a client arriving now, or to one being handed
    /// control: **at its current position and current HP**, not its spawn point, so the
    /// animation carries on from where it is.
    pub fn as_seen(&self) -> net::mob::FieldMob {
        let mut m = self.spawn;
        if let Some((x, y)) = self.at {
            m.x = x;
            m.y = y;
        }
        m.hp = self.hp;
        m
    }
}

/// One map's live contents.
#[derive(Debug, Default)]
struct FieldState {
    mobs: HashMap<u32, LiveMob>,
    /// Spawn points waiting to refill: `(due_ms, objectId)`.
    pending: Vec<(u64, u32)>,
    /// Set once the spawn points have been registered, so entering twice does not double
    /// the field.
    seeded: bool,
    drops: crate::drops::DropTable,
}

/// Every map on this channel.
#[derive(Debug)]
pub struct Fields {
    maps: Mutex<HashMap<u32, FieldState>>,
    /// Who is connected, and what each of them is owed.
    ///
    /// Hung here rather than threaded separately for one reason: this `Arc` is
    /// **already** handed to every `Session` (`Session::joining`, and the server's
    /// one-per-channel `Fields` in `crate::server::serve`), so a bus reached
    /// through it needs no change to any constructor, to the socket loop, or to
    /// the ~7000 lines of session tests. It is also not a category error - this
    /// type is what is alive on the channel's maps, and players are.
    ///
    /// Its lock is its own and is a **leaf**: nothing in `crate::broadcast` calls
    /// back into `Fields`, so the two are never held at once and cannot deadlock.
    bus: crate::broadcast::Bus,
    /// **Who controls each mob.** See [`crate::mobshare`], and hung here for exactly the
    /// reason `bus` is: this `Arc` already reaches every `Session`.
    ///
    /// Its lock is a **leaf** too - nothing in `mobshare` calls back into `Fields` or into
    /// the bus - so the rule is only "do not take it while holding `maps`", which is a rule
    /// about not inventing a cycle rather than about breaking one. Every caller in this file
    /// takes the `Controllers` answer *first* and then the map lock.
    controllers: crate::mobshare::Controllers,
    /// **Every party on this channel.** `crate::party`.
    ///
    /// Held here and **not used by anything in this file**: it is the seam the party agent
    /// wires, put here because `fields.rs` has one owner and two agents must not both edit
    /// it. `Parties` is plain data with no interior mutability, so unlike [`Fields::bus`]
    /// the accessor has to hand out a guard - see [`Fields::parties`].
    ///
    /// `Parties::new()` rather than `Parties::default()`: the default leaves `next_id` at 0
    /// and party ids are meant to start at `party::FIRST_PARTY_ID`, which is the same
    /// "never renumber from a small number" rule `store::FIRST_CHARACTER_ID` carries. That
    /// is why `Fields` implements `Default` by hand.
    parties: Mutex<crate::party::Parties>,
}

impl Default for Fields {
    fn default() -> Self {
        Self::new()
    }
}

impl Fields {
    pub fn new() -> Self {
        Fields {
            maps: Mutex::new(HashMap::new()),
            bus: crate::broadcast::Bus::new(),
            controllers: crate::mobshare::Controllers::new(),
            parties: Mutex::new(crate::party::Parties::new()),
        }
    }

    /// This channel's message bus. See [`crate::broadcast`].
    pub fn bus(&self) -> &crate::broadcast::Bus {
        &self.bus
    }

    /// Who controls each mob on this channel. See [`crate::mobshare`].
    pub fn controllers(&self) -> &crate::mobshare::Controllers {
        &self.controllers
    }

    /// This channel's parties. See [`crate::party`].
    ///
    /// A guard rather than a reference, because `Parties` is plain data. **Nothing in this
    /// crate calls this yet** - it is here so the party agent has somewhere to put the
    /// registry without a second agent editing this file. Do not hold it across a call into
    /// `Fields`, `Bus` or a session.
    pub fn parties(&self) -> std::sync::MutexGuard<'_, crate::party::Parties> {
        self.parties.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Register a map's spawn points the first time anyone sets foot on it.
    ///
    /// **Every point starts due rather than alive**, so the field fills in over the next few
    /// seconds instead of arriving complete. Entering a second time does nothing - the field
    /// belongs to the channel and keeps running whether or not anyone is looking at it.
    pub fn seed(&self, map: u32, config: &Config, now_ms: u64) {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        let field = maps.entry(map).or_default();
        if field.seeded {
            return;
        }
        field.seeded = true;
        if !config.send_mobs {
            return;
        }
        let Some(points) = config.mobs.get(&map) else { return };
        let alive = crate::config::spawn_capacity(points.len(), 1);
        let alive = match config.mob_limit {
            Some(n) => alive.min(n),
            None => alive,
        };
        // Seeded from the map and the clock so two fresh spawns of the same field do not
        // lay the mobs out identically, and so a test can reproduce one exactly.
        let seed = (map as u64) << 32 ^ now_ms.wrapping_mul(0x9E37_79B9);
        for mob in crate::config::share_balanced(points, alive, seed) {
            let wz = config.mob_respawn_s.get(&(map, mob.object_id)).copied().unwrap_or(0);
            if let Some(delay) = crate::config::respawn_delay_ms(wz) {
                field.pending.push((now_ms.saturating_add(delay), mob.object_id));
            }
        }
    }

    /// Every mob currently alive on a map, at its current position - what an arriving player
    /// must be sent.
    pub fn mobs_on(&self, map: u32) -> Vec<LiveMob> {
        let maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        maps.get(&map).map(|f| f.mobs.values().cloned().collect()).unwrap_or_default()
    }

    /// Move a mob, **without asking who said so**.
    ///
    /// The raw write. Used by tests and by anything server-authoritative; the wire path is
    /// [`Fields::note_position_from`] and it is the one that enforces the controller rule.
    pub fn note_position(&self, map: u32, object_id: u32, at: (i16, i16)) {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(m) = maps.entry(map).or_default().mobs.get_mut(&object_id) {
            m.at = Some(at);
        }
    }

    /// **A client reported where a mob has moved to. Believe it only if it controls it.**
    ///
    /// Returns whether the report was believed, which is what the caller gates its `0x03E4`
    /// and its `0x03D9` on.
    ///
    /// # Why the check is here rather than only at the call site
    ///
    /// Because this is the row. Every session used to be granted every mob
    /// (`session/field.rs`), so two clients ran two independent wanders of the same monster
    /// and both wrote here - last write wins, and the position a drop lands on was whichever
    /// client reported most recently. Fixing the *grant* removes the second writer today;
    /// putting the predicate where the write happens is what keeps it removed. `CLAUDE.md`:
    /// a comment describing a guarantee is not the guarantee.
    ///
    /// A mob with **no** controller is refused too. It cannot legitimately be reporting - a
    /// mob's move sender is only reached once slot 8 has been switched on by a `0x03D2`
    /// (`research/mob-behaviour.md` §4) - so a report for one is either a grant left over
    /// from before this existed or an invented packet. Nothing on this socket authenticates
    /// anybody.
    ///
    /// **Lock order.** The registry is asked first and its guard dropped before the map lock
    /// is taken. The two locks are independent and neither calls the other; this is a rule
    /// about not inventing a cycle.
    pub fn note_position_from(
        &self,
        map: u32,
        object_id: u32,
        at: (i16, i16),
        reporting: crate::mobshare::SessionId,
    ) -> bool {
        let controller = self.controllers.controller_of(map, object_id);
        if !crate::mobshare::may_report_movement(controller, reporting) {
            return false;
        }
        self.note_position(map, object_id, at);
        true
    }

    /// A mob's remaining HP, or `None` if it is not alive on that map.
    pub fn mob_hp(&self, map: u32, object_id: u32) -> Option<u64> {
        let maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        maps.get(&map)?.mobs.get(&object_id).map(|m| m.hp)
    }

    /// Where a mob is, for a drop to land on.
    pub fn mob_position(&self, map: u32, object_id: u32) -> Option<(i16, i16)> {
        let maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        maps.get(&map)?.mobs.get(&object_id).and_then(|m| m.at)
    }

    /// A mob's template id, for its drop table.
    pub fn mob_template(&self, map: u32, object_id: u32) -> Option<u32> {
        let maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        maps.get(&map)?.mobs.get(&object_id).map(|m| m.spawn.template_id)
    }

    /// Apply damage. Returns the HP left, or `None` if it died - in which case the spawn
    /// point is booked to refill and the mob is gone from the field.
    pub fn hurt(
        &self,
        map: u32,
        object_id: u32,
        damage: u64,
        by: u32,
        config: &Config,
        now_ms: u64,
    ) -> Hurt {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        let field = maps.entry(map).or_default();
        let Some(m) = field.mobs.get_mut(&object_id) else { return Hurt::Alive(0) };
        // Credit BEFORE subtracting: the cap is "damage that landed", and after the subtract
        // there is nothing left to measure it against.
        m.credit(by, damage);
        m.hp = m.hp.saturating_sub(damage);
        if m.hp > 0 {
            return Hurt::Alive(m.hp);
        }
        let dead = field.mobs.remove(&object_id);
        let wz = config.mob_respawn_s.get(&(map, object_id)).copied().unwrap_or(0);
        if let Some(delay) = crate::config::respawn_delay_ms(wz) {
            field.pending.push((now_ms.saturating_add(delay), object_id));
        }
        Hurt::Died(dead.map(|m| m.shares()).unwrap_or_default())
    }

    /// Spawn every point on this map whose timer is due, and return what arrived.
    ///
    /// Drives both the first fill of a field and every refill after a kill - see the module
    /// docs on why those are the same mechanism.
    pub fn due_respawns(&self, map: u32, config: &Config, now_ms: u64) -> Vec<LiveMob> {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        let field = maps.entry(map).or_default();
        if field.pending.is_empty() {
            return Vec::new();
        }
        let (due, waiting): (Vec<_>, Vec<_>) =
            std::mem::take(&mut field.pending).into_iter().partition(|(at, _)| now_ms >= *at);
        field.pending = waiting;

        let mut out = Vec::new();
        for (_, object_id) in due {
            let Some(spawn) = config
                .mobs
                .get(&map)
                .and_then(|list| list.iter().find(|m| m.object_id == object_id))
            else {
                continue;
            };
            let live = LiveMob { spawn: *spawn, hp: spawn.hp, at: None, damage_by: Vec::new() };
            field.mobs.insert(object_id, live.clone());
            out.push(live);
        }
        out
    }

    /// Run something against a map's drop table, **and post whatever it addressed to an
    /// owner**.
    ///
    /// The table is per map and per channel for the same reason the mobs are: an item on the
    /// floor is a property of the field, not of whoever is looking at it. This is a closure
    /// rather than an accessor because the table lives behind the same lock.
    ///
    /// # It also delivers, and that is not decoration
    ///
    /// A drop is **private to its owner** now (`crate::mobshare`), so a packet about one has
    /// exactly one legitimate recipient and it is very often not the connection that called
    /// this. [`crate::drops::DropTable::sweep`] is the case that forced it: every session on
    /// a map ticks, the first one to tick removes the expired drop from the shared table, and
    /// before this the `0x046F` went back to *that* session - so the owner kept drawing an
    /// item that no longer existed and a bystander was told about an object its pool never
    /// held.
    ///
    /// `DropTable` cannot deliver: it holds no bus, and it must not, because it is the file
    /// with no session and no socket in it. `Fields` holds both, so the outbox is drained
    /// here. [`crate::drops::Addressed`] is the whole channel and
    /// `crate::broadcast::Bus::publish_to_character` matches the map as well as the
    /// character, so a fade cannot land on a field the drop was never on.
    ///
    /// **The map lock is released before anything is posted.** The bus lock is a leaf, so
    /// nesting them would not deadlock today - it would merely make a cycle possible for the
    /// next person, which is the same rule the `controllers` field carries.
    pub fn with_drops<T>(&self, map: u32, f: impl FnOnce(&mut crate::drops::DropTable) -> T) -> T {
        let (out, mail) = {
            let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
            let table = &mut maps.entry(map).or_default().drops;
            let out = f(table);
            (out, table.take_addressed())
        };
        for a in mail {
            // The miss is ordinary: the owner logged out, or walked through a portal, and
            // their pool was rebuilt empty either way. Nothing to retry and nothing to log
            // as an error - the same contract `Bus::send_to_character` documents.
            let _ = self.bus.publish_to_character(a.character, a.map_id, a.reply);
        }
        out
    }

    /// How many mobs are alive on a map. For tests and the log.
    pub fn mob_count(&self, map: u32) -> usize {
        let maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        maps.get(&map).map(|f| f.mobs.len()).unwrap_or(0)
    }

    /// How many spawn points are waiting to refill.
    pub fn pending_count(&self, map: u32) -> usize {
        let maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        maps.get(&map).map(|f| f.pending.len()).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    /// Over-damage does not count. The owner: *"over-damage of a mob's HP does not count towards
    /// % sharing"*.
    #[test]
    fn a_killing_blow_is_credited_only_for_what_landed() {
        let mut m = LiveMob {
            spawn: net::mob::FieldMob::new(1, 2, 0, 0, 1, 100),
            hp: 100,
            at: None,
            damage_by: Vec::new(),
        };
        assert_eq!(m.credit(1, 40), 40);
        m.hp -= 40;
        // 500 into a mob with 60 left credits 60, not 500. Without the cap this hit would
        // hold 500/540 of the share and the player who did the work would be a minority.
        assert_eq!(m.credit(2, 500), 60);
        m.hp = 0;
        let shares = m.shares();
        assert_eq!(shares.len(), 2);
        assert_eq!((shares[0].character, shares[0].dealt), (2, 60));
        assert!(shares[0].majority);
        assert_eq!((shares[1].character, shares[1].dealt), (1, 40));
        assert!(!shares[1].majority, "exactly one majority");
        assert_eq!(shares[0].total, 100);
    }

    #[test]
    fn the_majority_takes_the_whole_amount_and_a_minority_takes_its_fraction() {
        let majority = DamageShare { character: 1, dealt: 60, total: 100, majority: true };
        let minority = DamageShare { character: 2, dealt: 40, total: 100, majority: false };
        assert_eq!(majority.cut_of(100), 100, "the majority is not scaled down");
        assert_eq!(minority.cut_of(100), 40);
        // A share earned is never paid zero: they hit it, and 0 reads as broken.
        assert_eq!(DamageShare { character: 2, dealt: 1, total: 1000, majority: false }.cut_of(2), 1);
        assert_eq!(minority.cut_of(0), 0, "...but a worthless mob is still worthless");
    }

    /// A tie breaks by who hit it first, so the same fight scores the same way twice.
    #[test]
    fn an_equal_split_is_decided_by_first_blood() {
        let mut m = LiveMob {
            spawn: net::mob::FieldMob::new(1, 2, 0, 0, 1, 100),
            hp: 100,
            at: None,
            damage_by: Vec::new(),
        };
        m.credit(7, 50);
        m.hp -= 50;
        m.credit(9, 50);
        m.hp = 0;
        let shares = m.shares();
        assert_eq!(shares[0].character, 7, "first to hit wins a tie");
        assert!(shares[0].majority);
    }

    #[test]
    fn a_mob_nobody_hurt_splits_nothing() {
        let m = LiveMob {
            spawn: net::mob::FieldMob::new(1, 2, 0, 0, 1, 100),
            hp: 100,
            at: None,
            damage_by: Vec::new(),
        };
        assert!(m.shares().is_empty());
    }

    use super::*;

    fn config_with_one_map() -> Config {
        let mut mobs = HashMap::new();
        // Four points, because `spawn_capacity` fills only 75% of them for a solo player -
        // a two-point map would hold one mob and make every count below a puzzle.
        mobs.insert(
            7u32,
            (0..4)
                .map(|i| net::mob::FieldMob::new(2000 + i, 2, 100 + 100 * i as i16, 395, 1, 30))
                .collect(),
        );
        Config { mobs, send_mobs: true, ..Config::default() }
    }

    /// **A field starts empty and fills in.** The owner: *"on first enter, no mobs should exist
    /// until the respawn timer kicks in."*
    #[test]
    fn a_field_is_empty_until_the_timer_fires() {
        let f = Fields::new();
        let c = config_with_one_map();
        f.seed(7, &c, 0);
        assert_eq!(f.mob_count(7), 0, "nothing is alive the instant you walk in");
        assert!(f.pending_count(7) > 0, "but the spawn points are booked");

        let arrived = f.due_respawns(7, &c, crate::config::DEFAULT_RESPAWN_MS);
        assert!(!arrived.is_empty(), "and they arrive when due");
        assert_eq!(f.mob_count(7), arrived.len());
    }

    /// Entering twice does not double the field - it belongs to the channel, not the visit.
    #[test]
    fn seeding_a_field_twice_does_nothing_the_second_time() {
        let f = Fields::new();
        let c = config_with_one_map();
        f.seed(7, &c, 0);
        let booked = f.pending_count(7);
        f.seed(7, &c, 0);
        assert_eq!(f.pending_count(7), booked);
    }

    /// **Mobs keep their position when the player leaves.** The field is not rebuilt on
    /// entry, so what a returning player is sent is where the mobs actually are.
    #[test]
    fn a_mob_keeps_its_position_across_a_visit() {
        let f = Fields::new();
        let c = config_with_one_map();
        f.seed(7, &c, 0);
        f.due_respawns(7, &c, 999_999);

        let ids: Vec<u32> = f.mobs_on(7).iter().map(|m| m.spawn.object_id).collect();
        assert!(ids.len() >= 2, "need two live mobs for this test: {ids:?}");
        let (moved_id, still_id) = (ids[0], ids[1]);
        let still_home = f.mobs_on(7).iter().find(|m| m.spawn.object_id == still_id).unwrap().spawn.x;
        f.note_position(7, moved_id, (742, 395));
        // The player leaves and comes back: nothing resets, because nothing is per-session.
        let seen = f.mobs_on(7);
        let moved = seen.iter().find(|m| m.spawn.object_id == moved_id).expect("still alive");
        assert_eq!(moved.at, Some((742, 395)));
        assert_eq!(moved.as_seen().x, 742, "and it is SENT at that position");

        let still = f.mobs_on(7).iter().find(|m| m.spawn.object_id == still_id).unwrap().as_seen();
        assert_eq!(still.x, still_home, "one that never moved is still at its spawn point");
    }

    #[test]
    fn killing_a_mob_removes_it_and_books_a_refill() {
        let f = Fields::new();
        let c = config_with_one_map();
        f.seed(7, &c, 0);
        f.due_respawns(7, &c, 999_999);
        let alive = f.mob_count(7);
        assert!(alive >= 1);
        let victim = f.mobs_on(7)[0].spawn.object_id;

        assert_eq!(f.hurt(7, victim, 10, 204, &c, 1_000), Hurt::Alive(20), "wounded, not dead");
        assert!(matches!(f.hurt(7, victim, 100, 204, &c, 1_000), Hurt::Died(_)), "dead");
        assert_eq!(f.mob_count(7), alive - 1);
        assert_eq!(f.pending_count(7), 1, "and its point is booked to refill");

        let back = f.due_respawns(7, &c, 1_000 + crate::config::DEFAULT_RESPAWN_MS);
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].hp, 30, "at full HP");
        assert_eq!(back[0].at, None, "and at its spawn point, not where it died");
    }

    /// Two maps do not share anything.
    #[test]
    fn fields_are_keyed_by_map() {
        let f = Fields::new();
        let c = config_with_one_map();
        f.seed(7, &c, 0);
        f.due_respawns(7, &c, 999_999);
        assert!(f.mob_count(7) > 0);
        assert_eq!(f.mob_count(8), 0, "another map shares nothing");
    }

    /// The drop table is per map and per channel too, so a second player sees the floor.
    #[test]
    fn drops_are_per_map_and_shared() {
        let f = Fields::new();
        f.with_drops(7, |d| assert_eq!(d.len(), 0));
        assert_eq!(f.with_drops(7, |d| d.len()), 0);
        assert_eq!(f.with_drops(8, |d| d.len()), 0);
    }

    /// **Only the controller may move a mob, enforced at the row.**
    ///
    /// Both halves in one test and in this order, because "the stranger was refused" is
    /// worthless without "the controller was believed" beside it: a `note_position_from` that
    /// refused everybody would pass the first assertion and freeze every monster in the game.
    #[test]
    fn a_position_report_from_a_connection_that_does_not_control_the_mob_is_refused() {
        const CONTROLLER: crate::mobshare::SessionId = 1;
        const STRANGER: crate::mobshare::SessionId = 2;
        let f = Fields::new();
        let c = config_with_one_map();
        f.seed(7, &c, 0);
        f.due_respawns(7, &c, 999_999);
        let id = f.mobs_on(7)[0].spawn.object_id;

        // Nobody controls it yet. A mob with no controller cannot legitimately be reporting -
        // the client's move sender is only reached once a 0x03D2 has switched slot 8 on.
        assert!(!f.note_position_from(7, id, (500, 395), CONTROLLER), "orphaned, so refused");
        assert_eq!(f.mob_position(7, id), None);

        f.controllers().claim_uncontrolled(7, CONTROLLER, &[id]);
        assert!(
            !f.note_position_from(7, id, (900, 395), STRANGER),
            "a second connection's report must not move the mob a third one is simulating"
        );
        assert_eq!(f.mob_position(7, id), None, "and must not have written the position");

        assert!(f.note_position_from(7, id, (500, 395), CONTROLLER), "the holder is believed");
        assert_eq!(f.mob_position(7, id), Some((500, 395)));
    }

    /// **`with_drops` delivers what the table addressed to an owner.**
    ///
    /// The sweep is the case: any connection on the channel may be the one that ticks, and the
    /// `0x046F` is owed to whoever was sent the `0x046E`. `DropTable` holds no bus, so this is
    /// the only place that can post it - and if this ever stopped draining the outbox, every
    /// expiry would go silent with no error anywhere.
    #[test]
    fn a_fade_addressed_to_an_owner_is_posted_to_that_owners_mailbox() {
        use crate::broadcast::Presence;
        let f = Fields::new();
        let owner = f.bus().join();
        let bystander = f.bus().join();
        let reply = |what: &str| crate::Reply {
            opcode: 0,
            body: Vec::new(),
            what: what.to_string(),
        };
        for (id, chr) in [(owner, 200u32), (bystander, 201)] {
            f.bus().enter_field(
                id,
                Presence {
                    character: chr,
                    map: 7,
                    spawn: reply("spawn"),
                    farewell: reply("farewell"),
                },
            );
        }
        let _ = f.bus().drain(owner);
        let _ = f.bus().drain(bystander);

        f.with_drops(7, |d| {
            d.drop_from_mob(crate::drops::DropFromMob {
                map_id: 7,
                owner_id: 200,
                item: store::Item::bundle(4_000_001, 1),
                inv_type: store::InventoryType::Etc,
                meso: 0,
                x: 1,
                y: 1,
                source_x: 1,
                source_y: 1,
                now_ms: 0,
            })
        });

        // The bystander sweeps, which is exactly the case that used to steal the fade.
        let handed_back = f.with_drops(7, |d| d.sweep(7, crate::drops::DROP_LIFETIME_MS + 1));
        assert!(handed_back.is_empty(), "the sweeper is handed nothing: {handed_back:?}");
        assert_eq!(f.with_drops(7, |d| d.len()), 0, "and the drop really is gone");

        assert_eq!(f.bus().drain(owner).len(), 1, "the owner is told their item faded");
        assert!(f.bus().drain(bystander).is_empty(), "and nobody else is");
    }

    /// The two registries `Fields` now carries reach every session through the one `Arc`, and
    /// the party one starts at `party::FIRST_PARTY_ID` rather than at 0 - which is why
    /// `Fields` implements `Default` by hand instead of deriving it.
    #[test]
    fn the_controller_and_party_registries_are_shared_and_start_empty() {
        let f = Fields::default();
        assert!(f.controllers().is_empty());
        assert!(f.parties().is_empty());
        // `Parties::new()` and `Parties::default()` differ only in `next_id` - 1 against 0 -
        // and neither is reachable through a getter, so the check is the `Debug` form. A
        // derived `Default` on `Fields` would silently pick the wrong one and the first party
        // ever created would be id 0, which is the small-number failure `FIRST_PARTY_ID`
        // exists to avoid.
        assert_eq!(
            format!("{:?}", *f.parties()),
            format!("{:?}", crate::party::Parties::new()),
            "Fields must build its registry with Parties::new(), not Parties::default()"
        );
        assert_ne!(
            format!("{:?}", crate::party::Parties::new()),
            format!("{:?}", crate::party::Parties::default()),
            "positive control: the two really are distinguishable this way"
        );
    }
}
