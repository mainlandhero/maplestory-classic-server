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
    /// The mob as it should be sent to a client arriving now: **at its current position**,
    /// not its spawn point, so the animation carries on from where it is.
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
#[derive(Debug, Default)]
pub struct Fields {
    maps: Mutex<HashMap<u32, FieldState>>,
}

impl Fields {
    pub fn new() -> Self {
        Self::default()
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
        for mob in crate::config::share_balanced(points, alive) {
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

    /// The client reported where a mob it controls has moved to.
    pub fn note_position(&self, map: u32, object_id: u32, at: (i16, i16)) {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(m) = maps.entry(map).or_default().mobs.get_mut(&object_id) {
            m.at = Some(at);
        }
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

    /// Run something against a map's drop table.
    ///
    /// The table is per map and per channel for the same reason the mobs are: an item on the
    /// floor is a property of the field, not of whoever is looking at it. This is a closure
    /// rather than an accessor because the table lives behind the same lock.
    pub fn with_drops<T>(&self, map: u32, f: impl FnOnce(&mut crate::drops::DropTable) -> T) -> T {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        f(&mut maps.entry(map).or_default().drops)
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
}
