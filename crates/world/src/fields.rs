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
//! # A refill is a refill of the MAP, not of the point that emptied
//!
//! The owner, 2026-09-13: *"once the mob is dead, a completely random spawn point should be chosen
//! that's not necessarily the dead mob's spawn point. Once a mob is dead, the same one
//! shouldn't necessarily always come back alive."*
//!
//! Until then a kill booked the dead mob's own point, so a solo map was 49 mobs standing on
//! the same 49 of 66 points forever and the other 17 were never visited - the population sat
//! pinned at the cap and never moved. Now a refill draws one **free, ordinary** spawn point
//! uniformly from the whole map and that point's mob stands up, which may or may not be the
//! type that died. Drawing uniformly from the free points keeps each type's expected share
//! equal to its share of the map (`research/mob-spawn-selection.md` §3).
//!
//! # Ordinary refills come in WAVES, on the field's own clock
//!
//! A player, relayed by the owner 2026-10-01: *"Mob respawns should happen every 8 seconds,
//! currently when something is killed, it schedules another respawn 8 seconds later, this is
//! not a wave."* So a kill books nothing. Every [`crate::config::DEFAULT_RESPAWN_MS`] from the
//! field's seed, the field tops itself back up to its cap in one go - however many died, and
//! whenever in the interval they died. Kill five in the last second before a wave and all
//! five are back a second later; kill one just after and it waits the whole interval.
//!
//! Two kinds of point stay out of the draw and out of the wave. A point with a WZ
//! `mobTime > 0` is **timed** - a boss or a rare spawn - and comes back at its own place on its
//! own clock, as [`Refill::Point`], because that is what the delay in the data is attached to.
//! A point with `mobTime -1` never refills at all.
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
    /// The foothold under `at`, when the last reported path said. Sent with `at` to a
    /// joining client so it does not place the mob and then drop it onto the spawn point's
    /// floor - the owner's "snap". `None` keeps the spawn foothold.
    pub at_fh: Option<i16>,
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
        // The floor goes with the position. A reported (x, y) on the spawn point's foothold
        // is a mob the client places and then drops, which a joining player sees as a snap.
        if let Some(fh) = self.at_fh {
            m.fh = fh;
        }
        m.hp = self.hp;
        m
    }
}

/// What a booked refill puts back when it comes due.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Refill {
    /// This exact spawn point: the first fill of a field, and a timed point (WZ `mobTime > 0`)
    /// that returns at its own place on its own clock. Ordinary points are refilled by the
    /// field's wave instead (`FieldState::next_wave_ms`).
    Point(u32),
}

/// One map's live contents.
#[derive(Debug, Default)]
struct FieldState {
    mobs: HashMap<u32, LiveMob>,
    /// Refills waiting to come due: `(due_ms, what)`.
    pending: Vec<(u64, Refill)>,
    /// The splitmix state the random refills draw from. Seeded with the field, so a test can
    /// reproduce a sequence exactly and two fields never draw in lockstep.
    rng: u64,
    /// Set once the spawn points have been registered, so entering twice does not double
    /// the field.
    seeded: bool,
    /// How many ordinary-point mobs (WZ `mobTime 0`) the wave keeps standing.
    ordinary_cap: usize,
    /// When the next wave tops the ordinary points back up to `ordinary_cap`. `None` on a field
    /// that was never seeded with spawn points - a field of summoned mobs has no wave.
    next_wave_ms: Option<u64>,
    /// When each mob may next use each skill, wall-clock ms: `(object id, skill) -> ms`.
    /// `crate::mobskills`.
    skill_ready_at: HashMap<(u32, u32), u64>,
    /// Each live mob's MP, for the mobs `crate::mobskills` has a kit for.
    mob_mp: HashMap<u32, crate::mobskills::MobMp>,
    /// The next object id [`Fields::summon_mob`] will hand out on this map.
    ///
    /// `0` means "not started"; the first call begins at [`SUMMON_OBJECT_ID_BASE`]. A summoned
    /// mob has no spawn point, so it cannot borrow a spawn point's id, and it must not collide
    /// with one either - `due_respawns` looks its mobs up in `config.mobs` by object id and a
    /// collision would make a kill resurrect the wrong thing.
    next_summon_id: u32,
    drops: crate::drops::DropTable,
    /// The reactors standing on the map, by object id. A broken one is not here; it sits in
    /// `reactor_pending` until its `reactorTime` runs out.
    reactors: HashMap<u32, LiveReactor>,
    /// Broken reactors waiting to come back: `(due_ms, objectId)`.
    reactor_pending: Vec<(u64, u32)>,
    /// The placements this field was seeded from, so a respawn can rebuild a fresh reactor
    /// without the config in hand.
    reactor_spawns: Vec<crate::config::ReactorSpawn>,
}

/// One reactor standing on a field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveReactor {
    /// As `0x0484` describes it - the current state is in here.
    pub seen: net::reactor::FieldReactor,
    /// The state a hit on the last live state produces.
    pub broken_state: u8,
    /// Seconds from breaking to standing again.
    pub respawn_s: u32,
}

/// What a hit did. `state` is the state the reactor is now in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReactorHitOutcome {
    pub template_id: u32,
    pub state: u8,
    pub x: i16,
    pub y: i16,
    pub broken: bool,
    pub respawn_s: u32,
}

impl LiveReactor {
    fn fresh(spawn: &crate::config::ReactorSpawn) -> Self {
        LiveReactor {
            seen: net::reactor::FieldReactor {
                object_id: spawn.object_id,
                template_id: spawn.template_id,
                state: 0,
                x: spawn.x,
                y: spawn.y,
                flip: spawn.flip,
                name: spawn.name.clone(),
            },
            broken_state: spawn.break_at,
            respawn_s: spawn.respawn_s,
        }
    }
}

/// Where summoned-mob object ids start.
///
/// Map spawn points are numbered from 2000 by the config loader, so this is far clear of them,
/// and the gap is deliberate: an id that is merely *probably* free is the kind of thing that
/// works until a map with a lot of mobs comes along.
pub const SUMMON_OBJECT_ID_BASE: u32 = 100_000;

/// **Which field a call is about: the map, and which copy of it.**
///
/// `instance` is `0` for the ordinary world - one shared field per map, which is what every
/// map outside a party quest is. A non-zero instance is a private copy: its own mobs, its
/// own drops, its own reactors, and its own set of people who can see each other.
///
/// # Why this is a type and not a `u32`
///
/// The owner, 2026-09-22: *"every party's PQ instance will be independent. Other parties can be
/// in the same map in the same channel, but however people from other parties will
/// deliberately not see other parties on the same map because the server does not relay
/// that information. All mobs are also instanced per party."*
///
/// The obvious cheaper change was to keep `map: u32` everywhere and pass a synthetic id for
/// an instance. That was rejected: a call site that kept passing the plain map id would
/// still compile, and the bug it produced - one party seeing another's mobs, or a broadcast
/// crossing between instances - is silent, intermittent and invisible to every existing
/// test. Making it a distinct type turns each of those into a compile error instead, which
/// is the only instrument here that cannot miss one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct FieldKey {
    pub map: u32,
    pub instance: u32,
}

impl FieldKey {
    /// The ordinary shared field for `map`.
    pub const fn world(map: u32) -> Self {
        Self { map, instance: 0 }
    }

    /// One private copy of `map`.
    pub const fn instanced(map: u32, instance: u32) -> Self {
        Self { map, instance }
    }

    /// Whether this is a private copy rather than the shared world.
    pub const fn is_instanced(&self) -> bool {
        self.instance != 0
    }
}

impl std::fmt::Display for FieldKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.instance {
            0 => write!(f, "{}", self.map),
            n => write!(f, "{}#{n}", self.map),
        }
    }
}

/// Every map on this channel, and every private copy of one.
#[derive(Debug)]
pub struct Fields {
    maps: Mutex<HashMap<FieldKey, FieldState>>,
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
    /// This channel's replica of the world's Maple Chat rooms - [`crate::messenger`]. Kept in
    /// step with every other channel by the hub, the same way `parties` is.
    messengers: Mutex<crate::messenger::Rooms>,
    /// This channel's party-quest runs. See [`crate::firsttime::Runs`].
    runs: Mutex<crate::firsttime::Runs>,
    /// This channel's ships to Orbis. See [`crate::boat::Voyages`].
    voyages: Mutex<crate::boat::Voyages>,
    /// The weather effect running on each map, if any: `(item, message, until_unix)`. So one
    /// runs at a time per map, and someone arriving mid-effect gets the rest of it.
    /// `session/weather.rs`.
    weather: Mutex<HashMap<FieldKey, (u32, String, i64)>>,
    /// **Statuses on live mobs** - Disorder's attack and defence cut, keyed by map and object
    /// id: `(status index, value, expires at clock ms)`. `crate::session::mobdebuff`. The
    /// client keeps its own copy from `0x03E6`; this one is for the server's own arithmetic.
    mob_debuffs: Mutex<HashMap<(FieldKey, u32), Vec<(u32, i32, u64)>>>,
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
            messengers: Mutex::new(crate::messenger::Rooms::new()),
            runs: Mutex::new(crate::firsttime::Runs::default()),
            voyages: Mutex::new(crate::boat::Voyages::default()),
            weather: Mutex::new(HashMap::new()),
            mob_debuffs: Mutex::new(HashMap::new()),
        }
    }

    /// Statuses on live mobs. See the field.
    pub fn mob_debuffs(&self) -> std::sync::MutexGuard<'_, HashMap<(FieldKey, u32), Vec<(u32, i32, u64)>>> {
        self.mob_debuffs.lock().unwrap_or_else(|e| e.into_inner())
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

    /// This channel's replica of the Maple Chat rooms. A guard, like [`Fields::parties`]: do
    /// not hold it across a call into `Fields`, `Bus` or a session.
    pub fn messengers(&self) -> std::sync::MutexGuard<'_, crate::messenger::Rooms> {
        self.messengers.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// This channel's party-quest runs. **Bind what you read to a local before locking
    /// anything else** - a guard held across a second `runs()` in one expression deadlocks.
    pub fn runs(&self) -> std::sync::MutexGuard<'_, crate::firsttime::Runs> {
        self.runs.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// This channel's ships to Orbis. Same rule as [`Fields::runs`]: bind what you read to a
    /// local before locking anything else.
    pub fn voyages(&self) -> std::sync::MutexGuard<'_, crate::boat::Voyages> {
        self.voyages.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// The weather effect running on each map. Same rule as [`Fields::runs`].
    pub fn weather(&self) -> std::sync::MutexGuard<'_, HashMap<FieldKey, (u32, String, i64)>> {
        self.weather.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Register a map's spawn points the first time anyone sets foot on it.
    ///
    /// **Every point starts due rather than alive**, so the field fills in over the next few
    /// seconds instead of arriving complete. Entering a second time does nothing - the field
    /// belongs to the channel and keeps running whether or not anyone is looking at it.
    pub fn seed(&self, key: FieldKey, config: &Config, now_ms: u64) {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        let field = maps.entry(key).or_default();
        if field.seeded {
            return;
        }
        field.seeded = true;
        // The reactors stand from the first entry, whatever the mob switch says: they are
        // scenery with a drop table, not monsters.
        if let Some(spawns) = config.reactors.get(&key.map) {
            for spawn in spawns {
                field.reactors.insert(spawn.object_id, LiveReactor::fresh(spawn));
            }
            field.reactor_spawns = spawns.clone();
        }
        if !config.send_mobs {
            return;
        }
        let Some(points) = config.mobs.get(&key.map) else { return };
        let mob_time = |id: u32| config.mob_respawn_s.get(&(key.map, id)).copied().unwrap_or(0);

        // **A boss is never left to the draw, and never makes a fresh map wait.** The owner,
        // 2026-09-24: *"let's say I go to Mushmom map and this is the first time the server has
        // tried to load the map (a.k.a no mobs on map), the Mushmom should be scheduled to spawn
        // instantly. Only after Mushmom is defeated, should the respawn timer kick in."*
        //
        // A point with its own `mobTime` - Mushmom's 3600, Jr. Balrog's 10800, a Fairy's 180 -
        // used to be booked at `now + mobTime`, so the first visitor waited the whole timer.
        // Worse, it went through the 75% draw below with everything else, and a refill after a
        // kill only ever picks ORDINARY points: a boss point the draw left out never spawned
        // until the server restarted - on a four-point map, one start in four. So every timed
        // point (and a `-1` point, which spawns once and never again) is booked now, due now,
        // outside the draw. After that nothing is different: `Fields::hurt` re-books the point
        // at its own `mobTime` when it dies, and that countdown is on the process clock, so it
        // keeps running with nobody on the map.
        let (timed, ordinary): (Vec<&net::mob::FieldMob>, Vec<&net::mob::FieldMob>) =
            points.iter().partition(|m| mob_time(m.object_id) != 0);
        for mob in &timed {
            field.pending.push((now_ms, Refill::Point(mob.object_id)));
        }

        // The ordinary points: 75% of them, drawn, each after the default delay - the fill-in
        // The owner asked for on 2026-08-19 (*"on first enter, no mobs should exist until the
        // respawn timer kicks in"*), unchanged.
        let ordinary: Vec<net::mob::FieldMob> = ordinary.into_iter().copied().collect();
        let alive = crate::config::spawn_capacity(ordinary.len(), 1);
        let alive = match config.mob_limit {
            Some(n) => alive.min(n),
            None => alive,
        };
        // Seeded from the map and the clock so two fresh spawns of the same field do not
        // lay the mobs out identically, and so a test can reproduce one exactly.
        let seed = (key.map as u64) << 32 ^ now_ms.wrapping_mul(0x9E37_79B9);
        field.rng = seed;
        // The first fill lands at the first wave; every wave after that tops up to the same cap.
        field.ordinary_cap = alive;
        field.next_wave_ms = Some(now_ms.saturating_add(crate::config::DEFAULT_RESPAWN_MS));
        for mob in crate::config::share_balanced(&ordinary, alive, seed) {
            if let Some(delay) = crate::config::respawn_delay_ms(mob_time(mob.object_id)) {
                field.pending.push((now_ms.saturating_add(delay), Refill::Point(mob.object_id)));
            }
        }
    }

    /// Whether mob `object_id` may use `skill` at `now_ms` (wall clock).
    pub fn skill_ready(&self, key: FieldKey, object_id: u32, skill: u32, now_ms: u64) -> bool {
        let maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        maps.get(&key).and_then(|f| f.skill_ready_at.get(&(object_id, skill))).map_or(true, |&at| now_ms >= at)
    }

    /// **Spend the mob's skill**: `true` for the caller that took it, and it is not ready
    /// again until `now_ms + interval_ms`. A test-and-set, so two reports of one cast - or two
    /// controllers racing a handover - cannot apply it twice.
    pub fn take_skill(&self, key: FieldKey, object_id: u32, skill: u32, now_ms: u64, interval_ms: u64) -> bool {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        let field = maps.entry(key).or_default();
        if field.skill_ready_at.get(&(object_id, skill)).is_some_and(|&at| now_ms < at) {
            return false;
        }
        field.skill_ready_at.insert((object_id, skill), now_ms.saturating_add(interval_ms));
        true
    }

    /// **Run `f` against this mob's MP**, creating it full on first sight. `None` for a mob
    /// that is not alive on the field - a corpse has no MP to keep.
    pub fn with_mob_mp<T>(
        &self,
        key: FieldKey,
        object_id: u32,
        max_mp: u32,
        now_ms: u64,
        f: impl FnOnce(&mut crate::mobskills::MobMp) -> T,
    ) -> Option<T> {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        let field = maps.get_mut(&key)?;
        if !field.mobs.contains_key(&object_id) {
            return None;
        }
        let mp = field.mob_mp.entry(object_id).or_insert_with(|| crate::mobskills::MobMp::full(max_mp, now_ms));
        Some(f(mp))
    }

    /// **Fill every spawn point on this field now** - the party quest's last stage. `true`
    /// the first time, and then the caller must run [`Fields::due_respawns`] to stand them
    /// up; `false` when the field was already seeded, which is the next member walking in.
    ///
    /// The owner, 2026-09-23: *"upon entry in the map for the party instance, it should
    /// automatically spawn the 10 mobs that should be present in the map (without waiting
    /// the full respawn timer of 3 minutes)."* [`Fields::seed`] cannot: it fills 75% of the
    /// points, each only after its own `mobTime` (180 s on this map), and a `mobTime -1` point -
    /// the King Slime - never at all. This books **every** point as due now instead, and
    /// changes nothing after that: a kill still books its point back by the WZ's own clock,
    /// so the Curse Eyes and Jr. Neckis return after 180 s and the King Slime (`-1`) does not -
    /// which is the owner's second message, *"The other mobs can respawn, only the King Slime
    /// should not respawn"*.
    pub fn seed_all_now(&self, key: FieldKey, config: &Config, now_ms: u64) -> bool {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        let field = maps.entry(key).or_default();
        if field.seeded {
            return false;
        }
        field.seeded = true;
        field.rng = (key.map as u64) << 32 ^ now_ms.wrapping_mul(0x9E37_79B9);
        if !config.send_mobs {
            return true;
        }
        let points = config.mobs.get(&key.map).map(Vec::as_slice).unwrap_or(&[]);
        for mob in points {
            field.pending.push((now_ms, Refill::Point(mob.object_id)));
        }
        // Every ordinary point is filled here, so the wave keeps every one of them filled.
        field.ordinary_cap = points
            .iter()
            .filter(|m| config.mob_respawn_s.get(&(key.map, m.object_id)).copied().unwrap_or(0) == 0)
            .count();
        field.next_wave_ms = Some(now_ms.saturating_add(crate::config::DEFAULT_RESPAWN_MS));
        true
    }

    /// **Put a mob on a map that has no spawn point for it.** A summoning sack.
    ///
    /// The owner, 2026-09-09: *"I just also tried summoning the GM Black Sack Jr. Balrog lvl 80"* -
    /// `0x0111`, which was decoded in full and never handled.
    ///
    /// # It is not a respawn, and that difference is the whole function
    ///
    /// [`Fields::due_respawns`] looks each pending id up in `config.mobs` and skips anything it
    /// cannot find, so a summoned mob could never come back through it. That is **correct**: a
    /// sack's mob is summoned once and stays dead. This inserts straight into the live pool and
    /// registers no pending entry, so a kill removes it and nothing refills it -
    /// `Fields::hurt`'s respawn push is already gated on `config.mob_respawn_s`, which has no
    /// row for an id that is not a spawn point.
    ///
    /// The id comes from a per-map counter starting at [`SUMMON_OBJECT_ID_BASE`] rather than
    /// from the spawn-point range, because a collision there would make killing a summoned mob
    /// schedule a respawn of somebody else's.
    ///
    /// `appear_type` is [`net::mob::APPEAR_SPAWNING`], and that is not cosmetic: it is the
    /// value that gives the client the spawn animation, and `net::mob`'s own docs record that
    /// the alternatives write gate fields this server does not fill.
    pub fn summon_mob(
        &self,
        key: FieldKey,
        template_id: u32,
        at: (i16, i16),
        fh: i16,
        hp: u64,
    ) -> LiveMob {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        let field = maps.entry(key).or_default();
        let mut id = if field.next_summon_id == 0 {
            SUMMON_OBJECT_ID_BASE
        } else {
            field.next_summon_id
        };
        // Skip anything the client cannot use as a pool key, and anything already live -
        // the second is belt and braces, but a duplicate key silently replaces a mob.
        loop {
            id = net::mob::next_usable_object_id(id);
            if !field.mobs.contains_key(&id) {
                break;
            }
            id = id.wrapping_add(1);
        }
        field.next_summon_id = id.wrapping_add(1);

        let mut spawn = net::mob::FieldMob::new(id, template_id, at.0, at.1, fh, hp);
        spawn.appear_type = net::mob::APPEAR_SPAWNING;
        let live = LiveMob { spawn, hp, at: Some(at), at_fh: None, damage_by: Vec::new() };
        field.mobs.insert(id, live.clone());
        live
    }

    /// Every mob currently alive on a map, at its current position - what an arriving player
    /// must be sent.
    /// Every reactor standing on `map` now - the broken ones are not standing.
    pub fn reactors_on(&self, key: FieldKey) -> Vec<LiveReactor> {
        let maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        maps.get(&key).map(|f| f.reactors.values().cloned().collect()).unwrap_or_default()
    }

    /// A hit on a standing reactor: the state advances by one. On the hit that reaches the
    /// broken state the reactor leaves the standing set and is scheduled to come back
    /// `respawn_s` later. `None` when no reactor by that id is standing on the map.
    pub fn hit_reactor(&self, key: FieldKey, object_id: u32, now_ms: u64) -> Option<ReactorHitOutcome> {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        let field = maps.get_mut(&key)?;
        let live = field.reactors.get_mut(&object_id)?;
        live.seen.state = live.seen.state.saturating_add(1);
        let broken = live.seen.state >= live.broken_state;
        let outcome = ReactorHitOutcome {
            template_id: live.seen.template_id,
            state: live.seen.state,
            x: live.seen.x,
            y: live.seen.y,
            broken,
            respawn_s: live.respawn_s,
        };
        if broken {
            let due = now_ms.saturating_add(u64::from(live.respawn_s) * 1000);
            field.reactors.remove(&object_id);
            field.reactor_pending.push((due, object_id));
        }
        Some(outcome)
    }

    /// Broken reactors whose time has come: put back standing, state 0, and returned so the
    /// caller can announce them. Drains the due entries; the not-yet-due stay.
    pub fn due_reactor_respawns(&self, key: FieldKey, now_ms: u64) -> Vec<LiveReactor> {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        let Some(field) = maps.get_mut(&key) else { return Vec::new() };
        let (due, later): (Vec<_>, Vec<_>) = field.reactor_pending.drain(..).partition(|(t, _)| *t <= now_ms);
        field.reactor_pending = later;
        let mut out = Vec::new();
        for (_, object_id) in due {
            // The spawn row is not kept on the field; rebuild the fresh reactor from what the
            // broken one was, which carries everything but the state.
            let Some(spawn) = field.reactor_spawns.iter().find(|s| s.object_id == object_id).cloned() else { continue };
            let fresh = LiveReactor::fresh(&spawn);
            field.reactors.insert(object_id, fresh.clone());
            out.push(fresh);
        }
        out
    }

    pub fn mobs_on(&self, key: FieldKey) -> Vec<LiveMob> {
        let maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        maps.get(&key).map(|f| f.mobs.values().cloned().collect()).unwrap_or_default()
    }

    /// Move a mob, **without asking who said so**.
    ///
    /// The raw write. Used by tests and by anything server-authoritative; the wire path is
    /// [`Fields::note_position_from`] and it is the one that enforces the controller rule.
    pub fn note_position(&self, key: FieldKey, object_id: u32, at: (i16, i16)) {
        self.note_position_and_floor(key, object_id, at, None);
    }

    /// [`Fields::note_position`] with the foothold the path named under that position.
    /// `None` leaves whatever floor was last known - the spawn's, if none was ever reported.
    pub fn note_position_and_floor(&self, key: FieldKey, object_id: u32, at: (i16, i16), fh: Option<i16>) {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(m) = maps.entry(key).or_default().mobs.get_mut(&object_id) {
            m.at = Some(at);
            if fh.is_some() {
                m.at_fh = fh;
            }
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
        key: FieldKey,
        object_id: u32,
        at: (i16, i16),
        fh: Option<i16>,
        reporting: crate::mobshare::SessionId,
    ) -> bool {
        let controller = self.controllers.controller_of(key, object_id);
        if !crate::mobshare::may_report_movement(controller, reporting) {
            return false;
        }
        self.note_position_and_floor(key, object_id, at, fh);
        true
    }

    /// A mob's remaining HP, or `None` if it is not alive on that map.
    pub fn mob_hp(&self, key: FieldKey, object_id: u32) -> Option<u64> {
        let maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        maps.get(&key)?.mobs.get(&object_id).map(|m| m.hp)
    }

    /// Where a mob has **reported** being. `None` means it has never sent a `0x02FF`.
    ///
    /// **This is the raw report and it stays that way.** `note_position_from` refuses a
    /// writer that does not control the mob, and the two tests below prove that refusal by
    /// asserting this answers `None` afterwards; a fallback in here would make those tests
    /// pass whatever the guard did. For "where is this mob, for a drop to land on", use
    /// [`Fields::mob_site`].
    pub fn mob_position(&self, key: FieldKey, object_id: u32) -> Option<(i16, i16)> {
        let maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        maps.get(&key)?.mobs.get(&object_id).and_then(|m| m.at)
    }

    /// **Where a mob is, for a drop to land on**: its reported position, else its spawn
    /// point. `None` only when the mob is not on that map at all.
    ///
    /// The spawn-point half is the fix for the quieter half of the drop-placement bug. A mob
    /// that has never moved has `at == None`, so `session/combat.rs` fell all the way through
    /// to the **player's own feet** - contradicting the owner's *"they should drop from the killed
    /// mob's position, not from the player character position"* while `LiveMob::spawn` sat in
    /// the same struct, one field away, carrying the exact pixel out of `Map.wz`. That mob is
    /// standing on its spawn point by definition: `gm-handbook/mobs.txt` names the very
    /// foothold, and `crate::footholds` measured that 10 235 of 10 236 `life` entries sit
    /// exactly on the foothold they name.
    ///
    /// [`LiveMob::as_seen`] has always done exactly this fold for the packet it builds. This
    /// is the same rule for the question the drop path asks, rather than a second one.
    pub fn mob_site(&self, key: FieldKey, object_id: u32) -> Option<(i16, i16)> {
        let maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        let m = maps.get(&key)?.mobs.get(&object_id)?;
        Some(m.at.unwrap_or((m.spawn.x, m.spawn.y)))
    }

    /// A mob's template id, for its drop table.
    pub fn mob_template(&self, key: FieldKey, object_id: u32) -> Option<u32> {
        let maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        maps.get(&key)?.mobs.get(&object_id).map(|m| m.spawn.template_id)
    }

    /// Apply damage. Returns the HP left, or `None` if it died - in which case the spawn
    /// point is booked to refill and the mob is gone from the field.
    pub fn hurt(
        &self,
        key: FieldKey,
        object_id: u32,
        damage: u64,
        by: u32,
        config: &Config,
        now_ms: u64,
    ) -> Hurt {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        let field = maps.entry(key).or_default();
        let Some(m) = field.mobs.get_mut(&object_id) else { return Hurt::Alive(0) };
        // Credit BEFORE subtracting: the cap is "damage that landed", and after the subtract
        // there is nothing left to measure it against.
        m.credit(by, damage);
        m.hp = m.hp.saturating_sub(damage);
        if m.hp > 0 {
            return Hurt::Alive(m.hp);
        }
        let dead = field.mobs.remove(&object_id);
        // Its MP and its skill clocks die with it: a respawn reuses the object id and must
        // start full and ready, not inherit a corpse's.
        field.mob_mp.remove(&object_id);
        field.skill_ready_at.retain(|(id, _), _| *id != object_id);
        // **Only a spawn point's death books a refill.** A summoned mob (`summon_mob`) has no
        // point behind it and stays dead; before refills could land anywhere, its stray
        // booking was harmless because `due_respawns` could not find it - now it would put a
        // random map mob up in its place, so the gate is explicit here.
        let is_spawn_point = config
            .mobs
            .get(&key.map)
            .is_some_and(|list| list.iter().any(|m| m.object_id == object_id));
        if is_spawn_point {
            // A timed point keeps its own place and clock. An ordinary one books nothing: the
            // field's next wave notices it is short and refills it (`due_respawns`).
            let wz = config.mob_respawn_s.get(&(key.map, object_id)).copied().unwrap_or(0);
            if wz > 0 {
                if let Some(delay) = crate::config::respawn_delay_ms(wz) {
                    field.pending.push((now_ms.saturating_add(delay), Refill::Point(object_id)));
                }
            }
        }
        Hurt::Died(dead.map(|m| m.shares()).unwrap_or_default())
    }

    /// Spawn every point on this map whose timer is due, run the field's wave if it is due,
    /// and return what arrived.
    ///
    /// Booked points first - the first fill and the timed returns - then the wave, so a wave
    /// landing on the same instant as the first fill counts the first fill as standing.
    pub fn due_respawns(&self, key: FieldKey, config: &Config, now_ms: u64) -> Vec<LiveMob> {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        let field = maps.entry(key).or_default();
        let Some(points) = config.mobs.get(&key.map) else { return Vec::new() };
        let (due, waiting): (Vec<_>, Vec<_>) =
            std::mem::take(&mut field.pending).into_iter().partition(|(at, _)| now_ms >= *at);
        field.pending = waiting;

        fn stand(field: &mut FieldState, points: &[net::mob::FieldMob], object_id: u32) -> Option<LiveMob> {
            let spawn = points.iter().find(|m| m.object_id == object_id)?;
            if field.mobs.contains_key(&object_id) {
                return None; // already standing - a timed return racing a wave's draw
            }
            let live = LiveMob { spawn: *spawn, hp: spawn.hp, at: None, at_fh: None, damage_by: Vec::new() };
            field.mobs.insert(object_id, live.clone());
            Some(live)
        }
        let mut out: Vec<LiveMob> = due.into_iter().filter_map(|(_, Refill::Point(id))| stand(field, points, id)).collect();

        // **The wave.** Not due, or no wave on this field: done.
        let Some(wave_at) = field.next_wave_ms.filter(|at| now_ms >= *at) else { return out };
        // The next one is on the same grid, past `now` - a field nobody ticked for a minute
        // runs ONE wave when it is next looked at, not seven.
        let every = crate::config::DEFAULT_RESPAWN_MS;
        field.next_wave_ms = Some(wave_at + (now_ms - wave_at) / every * every + every);

        let ordinary = |id: &u32| config.mob_respawn_s.get(&(key.map, *id)).copied().unwrap_or(0) == 0;
        // A point still booked for its first fill is spoken for: it counts toward the cap
        // and is not drawn, or a wave could double it.
        let booked: Vec<u32> = field.pending.iter().map(|(_, Refill::Point(id))| *id).collect();
        let standing = points
            .iter()
            .map(|m| m.object_id)
            .filter(|id| ordinary(id) && (field.mobs.contains_key(id) || booked.contains(id)))
            .count();
        // The free ordinary points: not standing, not booked, not timed. Drawn uniformly
        // without replacement, so no point on the map is favoured and the type that died has
        // no claim on the slot.
        let mut free: Vec<u32> = points
            .iter()
            .map(|m| m.object_id)
            .filter(|id| ordinary(id) && !field.mobs.contains_key(id) && !booked.contains(id))
            .collect();
        for _ in 0..field.ordinary_cap.saturating_sub(standing) {
            if free.is_empty() {
                break;
            }
            let draw = crate::config::splitmix64(&mut field.rng) as usize % free.len();
            let id = free.swap_remove(draw);
            out.extend(stand(field, points, id));
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
    /// A packet about a drop is owed to whoever was shown the drop, and that is very often
    /// not the connection that called this. [`crate::drops::DropTable::sweep`] is the case
    /// that forced it: every session on a map ticks, the first one to tick removes the
    /// expired drop from the shared table, and before this the `0x046F` went back to *that*
    /// session - so the owner kept drawing an item that no longer existed and a bystander was
    /// told about an object its pool never held. Then it went to the owner alone, and the
    /// party members and bystanders who had been shown the drop kept drawing it (the owner,
    /// 2026-09-18); a fade now goes to everyone on the drop's map.
    ///
    /// `DropTable` cannot deliver: it holds no bus, and it must not, because it is the file
    /// with no session and no socket in it. `Fields` holds both, so the outbox is drained
    /// here. [`crate::drops::Addressed`] is the whole channel, and
    /// `crate::broadcast::Bus::publish_to_map` posts only to connections standing on that
    /// map, so a fade cannot land on a field the drop was never on.
    ///
    /// **The map lock is released before anything is posted.** The bus lock is a leaf, so
    /// nesting them would not deadlock today - it would merely make a cycle possible for the
    /// next person, which is the same rule the `controllers` field carries.
    pub fn with_drops<T>(&self, key: FieldKey, f: impl FnOnce(&mut crate::drops::DropTable) -> T) -> T {
        let (out, mail) = {
            let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
            let table = &mut maps.entry(key).or_default().drops;
            let out = f(table);
            (out, table.take_addressed())
        };
        for a in mail {
            // The miss is ordinary: the owner logged out, or walked through a portal, and
            // their pool was rebuilt empty either way - or nobody is on the map at all.
            // Nothing to retry and nothing to log as an error - the same contract
            // `Bus::send_to_character` documents.
            let _ = self.bus.publish_to_map(a.map_id, a.reply);
        }
        out
    }

    /// How many mobs are alive on a map. For tests and the log.
    pub fn mob_count(&self, key: FieldKey) -> usize {
        let maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        maps.get(&key).map(|f| f.mobs.len()).unwrap_or(0)
    }

    /// How many spawn points are waiting to refill.
    pub fn pending_count(&self, key: FieldKey) -> usize {
        let maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        maps.get(&key).map(|f| f.pending.len()).unwrap_or(0)
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
            at: None, at_fh: None,
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
            at: None, at_fh: None,
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
            at: None, at_fh: None,
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
        f.seed(crate::fields::FieldKey::world(7), &c, 0);
        assert_eq!(f.mob_count(crate::fields::FieldKey::world(7)), 0, "nothing is alive the instant you walk in");
        assert!(f.pending_count(crate::fields::FieldKey::world(7)) > 0, "but the spawn points are booked");

        let arrived = f.due_respawns(crate::fields::FieldKey::world(7), &c, crate::config::DEFAULT_RESPAWN_MS);
        assert!(!arrived.is_empty(), "and they arrive when due");
        assert_eq!(f.mob_count(crate::fields::FieldKey::world(7)), arrived.len());
    }

    /// Entering twice does not double the field - it belongs to the channel, not the visit.
    #[test]
    fn seeding_a_field_twice_does_nothing_the_second_time() {
        let f = Fields::new();
        let c = config_with_one_map();
        f.seed(crate::fields::FieldKey::world(7), &c, 0);
        let booked = f.pending_count(crate::fields::FieldKey::world(7));
        f.seed(crate::fields::FieldKey::world(7), &c, 0);
        assert_eq!(f.pending_count(crate::fields::FieldKey::world(7)), booked);
    }

    /// **Mobs keep their position when the player leaves.** The field is not rebuilt on
    /// entry, so what a returning player is sent is where the mobs actually are.
    #[test]
    fn a_mob_keeps_its_position_across_a_visit() {
        let f = Fields::new();
        let c = config_with_one_map();
        f.seed(crate::fields::FieldKey::world(7), &c, 0);
        f.due_respawns(crate::fields::FieldKey::world(7), &c, 999_999);

        let ids: Vec<u32> = f.mobs_on(crate::fields::FieldKey::world(7)).iter().map(|m| m.spawn.object_id).collect();
        assert!(ids.len() >= 2, "need two live mobs for this test: {ids:?}");
        let (moved_id, still_id) = (ids[0], ids[1]);
        let still_home = f.mobs_on(crate::fields::FieldKey::world(7)).iter().find(|m| m.spawn.object_id == still_id).unwrap().spawn.x;
        f.note_position(crate::fields::FieldKey::world(7), moved_id, (742, 395));
        // The player leaves and comes back: nothing resets, because nothing is per-session.
        let seen = f.mobs_on(crate::fields::FieldKey::world(7));
        let moved = seen.iter().find(|m| m.spawn.object_id == moved_id).expect("still alive");
        assert_eq!(moved.at, Some((742, 395)));
        assert_eq!(moved.as_seen().x, 742, "and it is SENT at that position");

        let still = f.mobs_on(crate::fields::FieldKey::world(7)).iter().find(|m| m.spawn.object_id == still_id).unwrap().as_seen();
        assert_eq!(still.x, still_home, "one that never moved is still at its spawn point");
    }

    #[test]
    fn killing_a_mob_removes_it_and_the_next_wave_refills_it() {
        let f = Fields::new();
        let c = config_with_one_map();
        let key = crate::fields::FieldKey::world(7);
        f.seed(key, &c, 0);
        f.due_respawns(key, &c, 999_999);
        let alive = f.mob_count(key);
        assert!(alive >= 1);
        let victim = f.mobs_on(key)[0].spawn.object_id;

        // The waves run on the seed's grid: 8 s, 16 s, ... - 1 000 000 is one, and full, it adds
        // nothing. The kill lands just after it.
        assert!(f.due_respawns(key, &c, 1_000_000).is_empty());
        assert_eq!(f.hurt(key, victim, 10, 204, &c, 1_000_001), Hurt::Alive(20), "wounded, not dead");
        assert!(matches!(f.hurt(key, victim, 100, 204, &c, 1_000_001), Hurt::Died(_)), "dead");
        assert_eq!(f.mob_count(key), alive - 1);
        assert_eq!(f.pending_count(key), 0, "a kill books nothing - the wave notices the gap");

        assert!(f.due_respawns(key, &c, 1_007_999).is_empty(), "not before the wave");
        let back = f.due_respawns(key, &c, 1_008_000);
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].hp, 30, "at full HP");
        assert_eq!(back[0].at, None, "and at a spawn point, not where it died");
        assert_eq!(f.mob_count(key), alive, "back to the cap");
    }

    /// **A wave, not a timer per kill.** The player's words, relayed by the owner on 2026-10-01:
    /// *"Mob respawns should happen every 8 seconds, currently when something is killed, it
    /// schedules another respawn 8 seconds later, this is not a wave."* Three kills spread over
    /// one interval all come back at the same instant - the next wave - and a kill just after
    /// a wave waits the whole interval.
    #[test]
    fn kills_spread_over_an_interval_all_return_on_the_next_wave() {
        let mut mobs = HashMap::new();
        mobs.insert(7u32, (0..8).map(|i| net::mob::FieldMob::new(2000 + i, 2, 100 + 50 * i as i16, 395, 1, 30)).collect());
        let c = Config { mobs, send_mobs: true, ..Config::default() };
        let key = crate::fields::FieldKey::world(7);
        let f = Fields::new();
        f.seed(key, &c, 0);
        assert_eq!(f.due_respawns(key, &c, 8_000).len(), 6, "75% of eight, at the first wave");

        let standing: Vec<u32> = f.mobs_on(key).iter().map(|m| m.spawn.object_id).collect();
        for (victim, at) in standing.iter().take(3).zip([8_001u64, 11_000, 15_999]) {
            assert!(matches!(f.hurt(key, *victim, 1_000, 204, &c, at), Hurt::Died(_)));
            assert!(f.due_respawns(key, &c, at).is_empty(), "nothing comes back at the kill");
        }
        assert_eq!(f.mob_count(key), 3);
        let wave = f.due_respawns(key, &c, 16_000);
        assert_eq!(wave.len(), 3, "all three at the one wave, though they died up to 8 s apart");
        assert_eq!(f.mob_count(key), 6, "the cap, not more");
        assert!(f.due_respawns(key, &c, 23_999).is_empty(), "and nothing between waves");
    }

    /// **A field nobody looked at for a while runs one wave, not a backlog.** And a wave never
    /// overfills: with everything standing it puts up nothing.
    #[test]
    fn a_late_wave_runs_once_and_never_overfills() {
        let c = config_with_one_map();
        let key = crate::fields::FieldKey::world(7);
        let f = Fields::new();
        f.seed(key, &c, 0);
        assert_eq!(f.due_respawns(key, &c, 8_000).len(), 3);
        assert!(f.due_respawns(key, &c, 16_000).is_empty(), "full: the wave adds nothing");
        let victim = f.mobs_on(key)[0].spawn.object_id;
        assert!(matches!(f.hurt(key, victim, 1_000, 204, &c, 16_500), Hurt::Died(_)));
        assert_eq!(f.due_respawns(key, &c, 100_000).len(), 1, "a minute late: one wave, one mob");
        assert_eq!(f.mob_count(key), 3);
        let victim = f.mobs_on(key)[0].spawn.object_id;
        assert!(matches!(f.hurt(key, victim, 1_000, 204, &c, 100_001), Hurt::Died(_)));
        assert!(f.due_respawns(key, &c, 103_999).is_empty(), "the grid kept its phase: 104 000 is next, not 108 000");
        assert_eq!(f.due_respawns(key, &c, 104_000).len(), 1);
    }

    /// **A kill refills the map, not the point.** The owner, 2026-09-13: *"once the mob is dead, a
    /// completely random spawn point should be chosen that's not necessarily the dead mob's
    /// spawn point."* Four points, three standing: kill the same standing mob over and over
    /// and the refill must (a) sometimes land on the one point that was empty, i.e. not the
    /// victim's, (b) never land on a point that is already standing, and (c) never move the
    /// live count off the cap.
    #[test]
    fn a_kill_refills_a_random_free_point_rather_than_the_one_that_emptied() {
        let f = Fields::new();
        let c = config_with_one_map();
        f.seed(crate::fields::FieldKey::world(7), &c, 0);
        f.due_respawns(crate::fields::FieldKey::world(7), &c, 999_999);
        let cap = f.mob_count(crate::fields::FieldKey::world(7));
        assert_eq!(cap, 3, "75% of four points");

        let mut elsewhere = 0;
        let mut t = 1_000_000u64;
        for _ in 0..40 {
            let standing: Vec<u32> = f.mobs_on(crate::fields::FieldKey::world(7)).iter().map(|m| m.spawn.object_id).collect();
            let victim = standing[0];
            assert!(matches!(f.hurt(crate::fields::FieldKey::world(7), victim, 1_000, 204, &c, t), Hurt::Died(_)));
            let back = f.due_respawns(crate::fields::FieldKey::world(7), &c, t + crate::config::DEFAULT_RESPAWN_MS);
            assert_eq!(back.len(), 1, "one death, one refill");
            let came = back[0].spawn.object_id;
            let others: Vec<u32> = standing.iter().copied().filter(|id| *id != victim).collect();
            assert!(!others.contains(&came), "never on a point that is already standing");
            if came != victim {
                elsewhere += 1;
            }
            assert_eq!(f.mob_count(crate::fields::FieldKey::world(7)), cap, "the cap holds");
            t += 100_000;
        }
        assert!(elsewhere > 0, "in 40 kills the refill never left the victim's point - the draw is not random");
        assert!(elsewhere < 40, "and it never came back to the victim's point either - the draw excludes it, which it must not");
    }

    /// **A timed point comes back at its own place.** A WZ `mobTime > 0` is attached to a
    /// point, so its return is that point on that clock, not a random draw at the field rate.
    #[test]
    fn a_timed_spawn_point_returns_where_and_when_its_own_data_says() {
        let f = Fields::new();
        // One point, so the 75% draw cannot leave it out and the test always runs its kill.
        let mut mobs = HashMap::new();
        mobs.insert(7u32, vec![net::mob::FieldMob::new(2000, 2, 100, 395, 1, 30)]);
        let mut c = Config { mobs, send_mobs: true, ..Config::default() };
        c.mob_respawn_s.insert((7, 2000), 60); // a minute, at point 2000
        f.seed(crate::fields::FieldKey::world(7), &c, 0);
        assert_eq!(f.due_respawns(crate::fields::FieldKey::world(7), &c, 999_999).len(), 1, "the one point stands");
        assert!(matches!(f.hurt(crate::fields::FieldKey::world(7), 2000, 1_000, 204, &c, 1_000_000), Hurt::Died(_)));
        assert!(f.due_respawns(crate::fields::FieldKey::world(7), &c, 1_000_000 + crate::config::DEFAULT_RESPAWN_MS).is_empty(), "not at the field rate");
        let back = f.due_respawns(crate::fields::FieldKey::world(7), &c, 1_000_000 + 60_000);
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].spawn.object_id, 2000, "at its own point");
    }

    /// **A boss stands at once on a fresh map, is never left to the draw, and after a kill is
    /// due exactly its own `mobTime` later - on the process clock, so still due a day on.**
    /// The owner, 2026-09-24, with Mushmom (`mobTime 3600`) as the example.
    ///
    /// Four ordinary points and one boss point. Across 200 fresh seeds the boss is due at the
    /// very instant of the seed every time - the control is the ordinary points, which are
    /// NOT due then and of which only 75% are ever booked.
    #[test]
    fn a_boss_is_due_at_once_on_a_fresh_map_and_again_its_own_time_after_it_dies() {
        let key = crate::fields::FieldKey::world(7);
        let mut mobs = HashMap::new();
        let mut points: Vec<net::mob::FieldMob> =
            (0..4).map(|i| net::mob::FieldMob::new(2000 + i, 2, 100 + i as i16 * 50, 395, 1, 30)).collect();
        points.push(net::mob::FieldMob::new(2100, 700_000, 400, 395, 1, 20_000)); // Mushmom
        mobs.insert(7u32, points);
        let mut c = Config { mobs, send_mobs: true, ..Config::default() };
        c.mob_respawn_s.insert((7, 2100), 3_600);
        for n in 0..200u64 {
            let f = Fields::new();
            let t = 10_000 + n * 7_919;
            f.seed(key, &c, t);
            let now = f.due_respawns(key, &c, t);
            assert_eq!(now.iter().map(|m| m.spawn.object_id).collect::<Vec<_>>(), vec![2100], "seed {n}: the boss, at once, and only the boss");
            let later = f.due_respawns(key, &c, t + crate::config::DEFAULT_RESPAWN_MS);
            assert_eq!(later.len(), 3, "seed {n}: 75% of the four ordinary points, after the default delay");
        }
        // Killed: not back one second early, back on time - and still due if nobody looks
        // until a day later.
        let f = Fields::new();
        f.seed(key, &c, 0);
        let _ = f.due_respawns(key, &c, 0);
        let died = 50_000;
        assert!(matches!(f.hurt(key, 2100, 1_000_000, 204, &c, died), Hurt::Died(_)));
        assert!(f.due_respawns(key, &c, died + 3_599_999).iter().all(|m| m.spawn.object_id != 2100), "not early");
        let day = f.due_respawns(key, &c, died + 24 * 3_600_000);
        assert!(day.iter().any(|m| m.spawn.object_id == 2100), "a day later with nobody there: due, so it stands");
    }

    /// **A summoned mob's death books nothing.** It has no spawn point, so there is nothing
    /// to refill - and a random refill in its place would grow the field past the cap.
    #[test]
    fn a_summoned_mobs_death_does_not_book_a_refill() {
        let f = Fields::new();
        let c = config_with_one_map();
        f.seed(crate::fields::FieldKey::world(7), &c, 0);
        f.due_respawns(crate::fields::FieldKey::world(7), &c, 999_999);
        let sack = f.summon_mob(crate::fields::FieldKey::world(7), 2, (500, 395), 1, 10);
        assert_eq!(f.pending_count(crate::fields::FieldKey::world(7)), 0);
        assert!(matches!(f.hurt(crate::fields::FieldKey::world(7), sack.spawn.object_id, 1_000, 204, &c, 5_000), Hurt::Died(_)));
        assert_eq!(f.pending_count(crate::fields::FieldKey::world(7)), 0, "nothing booked for a mob with no point");
    }

    /// Two maps do not share anything.
    #[test]
    fn fields_are_keyed_by_map() {
        let f = Fields::new();
        let c = config_with_one_map();
        f.seed(crate::fields::FieldKey::world(7), &c, 0);
        f.due_respawns(crate::fields::FieldKey::world(7), &c, 999_999);
        assert!(f.mob_count(crate::fields::FieldKey::world(7)) > 0);
        assert_eq!(f.mob_count(crate::fields::FieldKey::world(8)), 0, "another map shares nothing");
    }

    /// The drop table is per map and per channel too, so a second player sees the floor.
    #[test]
    fn drops_are_per_map_and_shared() {
        let f = Fields::new();
        f.with_drops(crate::fields::FieldKey::world(7), |d| assert_eq!(d.len(), 0));
        assert_eq!(f.with_drops(crate::fields::FieldKey::world(7), |d| d.len()), 0);
        assert_eq!(f.with_drops(crate::fields::FieldKey::world(8), |d| d.len()), 0);
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
        f.seed(crate::fields::FieldKey::world(7), &c, 0);
        f.due_respawns(crate::fields::FieldKey::world(7), &c, 999_999);
        let id = f.mobs_on(crate::fields::FieldKey::world(7))[0].spawn.object_id;

        // Nobody controls it yet. A mob with no controller cannot legitimately be reporting -
        // the client's move sender is only reached once a 0x03D2 has switched slot 8 on.
        assert!(!f.note_position_from(crate::fields::FieldKey::world(7), id, (500, 395), None, CONTROLLER), "orphaned, so refused");
        assert_eq!(f.mob_position(crate::fields::FieldKey::world(7), id), None);

        f.controllers().claim_uncontrolled(crate::fields::FieldKey::world(7), CONTROLLER, &[id]);
        assert!(
            !f.note_position_from(crate::fields::FieldKey::world(7), id, (900, 395), None, STRANGER),
            "a second connection's report must not move the mob a third one is simulating"
        );
        assert_eq!(f.mob_position(crate::fields::FieldKey::world(7), id), None, "and must not have written the position");

        assert!(f.note_position_from(crate::fields::FieldKey::world(7), id, (500, 395), None, CONTROLLER), "the holder is believed");
        assert_eq!(f.mob_position(crate::fields::FieldKey::world(7), id), Some((500, 395)));
    }

    /// **`mob_site` falls back to the spawn point; `mob_position` never does.**
    ///
    /// The two questions are different and this is the test that keeps them apart. A mob that
    /// has not reported is standing on its spawn point, so a drop belongs there rather than
    /// at the player's feet - but the *report* is still absent, and the controller guard
    /// above proves itself by asserting exactly that absence.
    #[test]
    fn a_mob_that_has_never_reported_still_has_a_place_for_its_drops_to_fall() {
        const CONTROLLER: crate::mobshare::SessionId = 1;
        let f = Fields::new();
        let c = config_with_one_map();
        f.seed(crate::fields::FieldKey::world(7), &c, 0);
        f.due_respawns(crate::fields::FieldKey::world(7), &c, 999_999);
        let spawn = f.mobs_on(crate::fields::FieldKey::world(7))[0].spawn;
        let id = spawn.object_id;
        let home = (spawn.x, spawn.y);

        assert_eq!(f.mob_position(crate::fields::FieldKey::world(7), id), None, "it has never sent a 0x02FF");
        assert_eq!(f.mob_site(crate::fields::FieldKey::world(7), id), Some(home), "and it is standing on its spawn point");
        assert_ne!(home, (0, 0), "a spawn point of (0,0) would make this vacuous");

        // Once it reports, the report wins - the spawn point is a fallback, not a floor.
        f.controllers().claim_uncontrolled(crate::fields::FieldKey::world(7), CONTROLLER, &[id]);
        assert!(f.note_position_from(crate::fields::FieldKey::world(7), id, (742, 395), None, CONTROLLER));
        assert_eq!(f.mob_site(crate::fields::FieldKey::world(7), id), Some((742, 395)));
        assert_ne!(f.mob_site(crate::fields::FieldKey::world(7), id), Some(home), "it really moved away from home");

        // A mob that is not on the map has no site at all, and the caller must not invent one.
        assert_eq!(f.mob_site(crate::fields::FieldKey::world(7), 999_999), None);
        assert_eq!(f.mob_site(crate::fields::FieldKey::world(999), id), None);
    }

    /// **`with_drops` delivers what the table addressed to a map - to everyone on it.**
    ///
    /// The sweep is the case: any connection on the channel may be the one that ticks, and the
    /// `0x046F` is owed to whoever was sent the `0x046E` - the owner, a party member, a
    /// bystander looking at a public drop (the owner, 2026-09-18). `DropTable` holds no bus, so
    /// this is the only place that can post it - and if this ever stopped draining the outbox,
    /// every expiry would go silent with no error anywhere.
    #[test]
    fn a_fade_addressed_to_a_map_is_posted_to_every_mailbox_on_it() {
        use crate::broadcast::Presence;
        let f = Fields::new();
        let owner = f.bus().join();
        let bystander = f.bus().join();
        let elsewhere = f.bus().join();
        let reply = |what: &str| crate::Reply {
            opcode: 0,
            body: Vec::new(),
            what: what.to_string(),
        };
        for (id, chr, map) in [(owner, 200u32, crate::fields::FieldKey::world(7)), (bystander, 201, crate::fields::FieldKey::world(7)), (elsewhere, 202, crate::fields::FieldKey::world(8))] {
            f.bus().enter_field(
                id,
                Presence {
                    character: chr,
                    map,
                    spawn: reply("spawn"),
                    farewell: reply("farewell"),
                    companions: Vec::new(),
                },
            );
        }
        let _ = f.bus().drain(owner);
        let _ = f.bus().drain(bystander);
        let _ = f.bus().drain(elsewhere);

        f.with_drops(crate::fields::FieldKey::world(7), |d| {
            d.drop_from_mob(crate::drops::DropFromMob {
                from_mob: true,
                map_id: crate::fields::FieldKey::world(7),
                owner_id: 200,
                item: store::Item::bundle(4_000_001, 1),
                inv_type: store::InventoryType::Etc,
                meso: 0,
                x: 1,
                y: 1,
                source_x: 1,
                source_y: 1,
                now_ms: 0,
                party_id: 0,
            })
        });

        // The bystander sweeps, which is exactly the case that used to steal the fade.
        let handed_back = f.with_drops(crate::fields::FieldKey::world(7), |d| d.sweep(crate::fields::FieldKey::world(7), crate::drops::DROP_LIFETIME_MS + 1));
        assert!(handed_back.is_empty(), "the sweeper is handed nothing: {handed_back:?}");
        assert_eq!(f.with_drops(crate::fields::FieldKey::world(7), |d| d.len()), 0, "and the drop really is gone");

        assert_eq!(f.bus().drain(owner).len(), 1, "the owner is told their item faded");
        assert_eq!(f.bus().drain(bystander).len(), 1, "and so is everyone else on the map - a client that never held the id ignores it");
        assert!(f.bus().drain(elsewhere).is_empty(), "but nobody on another map");
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
