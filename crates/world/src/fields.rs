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

/// One mob that exists right now, and where it is.
#[derive(Debug, Clone)]
pub struct LiveMob {
    /// The spawn point's own record - template, foothold, spawn position, full HP.
    pub spawn: net::mob::FieldMob,
    /// Remaining HP.
    pub hp: u64,
    /// Where the client last reported it. `None` means it has never moved.
    pub at: Option<(i16, i16)>,
}

impl LiveMob {
    /// The mob as it should be sent to a client arriving now: **at its current position**,
    /// not its spawn point, so the animation carries on from where it is.
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
        config: &Config,
        now_ms: u64,
    ) -> Option<u64> {
        let mut maps = self.maps.lock().unwrap_or_else(|e| e.into_inner());
        let field = maps.entry(map).or_default();
        let Some(m) = field.mobs.get_mut(&object_id) else { return Some(0) };
        m.hp = m.hp.saturating_sub(damage);
        if m.hp > 0 {
            return Some(m.hp);
        }
        field.mobs.remove(&object_id);
        let wz = config.mob_respawn_s.get(&(map, object_id)).copied().unwrap_or(0);
        if let Some(delay) = crate::config::respawn_delay_ms(wz) {
            field.pending.push((now_ms.saturating_add(delay), object_id));
        }
        None
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
            let live = LiveMob { spawn: *spawn, hp: spawn.hp, at: None };
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

        assert_eq!(f.hurt(7, victim, 10, &c, 1_000), Some(20), "wounded, not dead");
        assert_eq!(f.hurt(7, victim, 100, &c, 1_000), None, "dead");
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
