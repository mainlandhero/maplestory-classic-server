//! **Kills and drops for the last seven days**, for the drop-table page (`world::dropweb`).
//!
//! The owner, 2026-10-01: *"a live drop stat for the last 7 days of all the kills along side the
//! drop table with how many player (and how many mob kills) within tracking period. This data
//! should be purged on a rolling 7 days."* And: *"Make this data cached so that it doesn't
//! burden the server or the database too much. This data at worst can be 30 or 60 minutes out
//! of date."*
//!
//! So nothing here is written per kill. Each channel counts in memory (`world::killstats`) and
//! hands this module a [`KillBatch`] every few minutes, which goes in as one transaction of
//! upserts. Every row is an **hour bucket**: kills per (hour, mob), the distinct killers per
//! (hour, mob), and drops per (hour, mob, item). Seven days of hours is 168 buckets, so the
//! tables stay small however many kills there are, and the purge is a range delete on the
//! leading key column.
//!
//! "The last seven days" is the buckets whose hour starts within the last 7 x 24 hours, so the
//! window is up to an hour longer than a week at the edge. The page says "7 days".

use std::collections::{HashMap, HashSet};

use rusqlite::{params, Connection};

use crate::error::Result;
use crate::Store;

/// How long a bucket is kept - the owner's seven days.
pub const KEEP_SECS: i64 = 7 * 24 * 3600;
/// One bucket per hour.
pub const BUCKET_SECS: i64 = 3600;

/// The bucket `unix` falls in: the start of its hour.
pub fn bucket(unix: i64) -> i64 {
    unix - unix.rem_euclid(BUCKET_SECS)
}

/// What one channel saw since its last flush.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KillBatch {
    /// `(hour, template) -> kills`.
    pub kills: HashMap<(i64, u32), u64>,
    /// `(hour, template, character)` - who killed it, for the distinct-player counts.
    pub killers: HashSet<(i64, u32, u32)>,
    /// `(hour, template, item) -> (times it dropped, total quantity)`. Item 0 is mesos.
    pub drops: HashMap<(i64, u32, u32), (u64, u64)>,
}

impl KillBatch {
    /// One kill of `template` by `character` at `unix`, and what fell: `(item, quantity)`.
    pub fn note(&mut self, unix: i64, template: u32, character: u32, drops: &[(u32, u32)]) {
        let hour = bucket(unix);
        *self.kills.entry((hour, template)).or_default() += 1;
        self.killers.insert((hour, template, character));
        for &(item, quantity) in drops {
            let e = self.drops.entry((hour, template, item)).or_default();
            e.0 += 1;
            e.1 += u64::from(quantity);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.kills.is_empty()
    }
}

/// The seven days, summed.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KillStats {
    /// Distinct characters who killed anything.
    pub players: u64,
    pub kills: u64,
    /// `template -> (kills, distinct killers)`.
    pub mobs: HashMap<u32, (u64, u64)>,
    /// `(template, item) -> (times it dropped, total quantity)`.
    pub drops: HashMap<(u32, u32), (u64, u64)>,
}

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS kill_hours (
            hour     INTEGER NOT NULL,
            template INTEGER NOT NULL,
            kills    INTEGER NOT NULL,
            PRIMARY KEY (hour, template)
        ) WITHOUT ROWID;
        CREATE TABLE IF NOT EXISTS kill_players (
            hour      INTEGER NOT NULL,
            template  INTEGER NOT NULL,
            character INTEGER NOT NULL,
            PRIMARY KEY (hour, template, character)
        ) WITHOUT ROWID;
        CREATE TABLE IF NOT EXISTS drop_hours (
            hour     INTEGER NOT NULL,
            template INTEGER NOT NULL,
            item     INTEGER NOT NULL,
            drops    INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            PRIMARY KEY (hour, template, item)
        ) WITHOUT ROWID;
        "#,
    )?;
    Ok(())
}

impl Store {
    /// Add a channel's batch, in one transaction. Two channels flushing the same bucket add up.
    pub fn flush_kill_stats(&self, batch: &KillBatch) -> Result<()> {
        if batch.is_empty() {
            return Ok(());
        }
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        for (&(hour, template), &kills) in &batch.kills {
            tx.execute(
                "INSERT INTO kill_hours (hour, template, kills) VALUES (?1, ?2, ?3)
                 ON CONFLICT(hour, template) DO UPDATE SET kills = kills + excluded.kills",
                params![hour, i64::from(template), i64::try_from(kills).unwrap_or(i64::MAX)],
            )?;
        }
        for &(hour, template, character) in &batch.killers {
            tx.execute(
                "INSERT OR IGNORE INTO kill_players (hour, template, character) VALUES (?1, ?2, ?3)",
                params![hour, i64::from(template), i64::from(character)],
            )?;
        }
        for (&(hour, template, item), &(drops, quantity)) in &batch.drops {
            tx.execute(
                "INSERT INTO drop_hours (hour, template, item, drops, quantity) VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(hour, template, item) DO UPDATE SET
                    drops = drops + excluded.drops, quantity = quantity + excluded.quantity",
                params![
                    hour,
                    i64::from(template),
                    i64::from(item),
                    i64::try_from(drops).unwrap_or(i64::MAX),
                    i64::try_from(quantity).unwrap_or(i64::MAX)
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Delete every bucket older than seven days before `now_unix`. Returns the rows removed.
    pub fn purge_kill_stats(&self, now_unix: i64) -> Result<usize> {
        let cutoff = window_start(now_unix);
        let conn = self.conn();
        let mut n = 0;
        for table in ["kill_hours", "kill_players", "drop_hours"] {
            n += conn.execute(&format!("DELETE FROM {table} WHERE hour < ?1"), params![cutoff])?;
        }
        Ok(n)
    }

    /// The last seven days before `now_unix`, summed.
    pub fn kill_stats(&self, now_unix: i64) -> Result<KillStats> {
        self.kill_stats_apart(now_unix, None)
    }

    /// [`Store::kill_stats`], with `apart` - a source counted in the same tables that is not a
    /// monster, such as an opened reward box - left out of the two totals, `kills` and
    /// `players`. Its own row in `mobs` and `drops` is still there.
    pub fn kill_stats_apart(&self, now_unix: i64, apart: Option<u32>) -> Result<KillStats> {
        let since = window_start(now_unix);
        let apart = apart.map_or(-1, i64::from);
        let conn = self.conn();
        let mut out = KillStats::default();
        let mut stmt = conn.prepare("SELECT template, SUM(kills) FROM kill_hours WHERE hour >= ?1 GROUP BY template")?;
        for row in stmt.query_map(params![since], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?)))? {
            let (t, k) = row?;
            out.mobs.entry(t as u32).or_default().0 = k as u64;
            if t != apart {
                out.kills += k as u64;
            }
        }
        let mut stmt = conn.prepare(
            "SELECT template, COUNT(DISTINCT character) FROM kill_players WHERE hour >= ?1 GROUP BY template",
        )?;
        for row in stmt.query_map(params![since], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?)))? {
            let (t, p) = row?;
            out.mobs.entry(t as u32).or_default().1 = p as u64;
        }
        out.players = conn.query_row(
            "SELECT COUNT(DISTINCT character) FROM kill_players WHERE hour >= ?1 AND template != ?2",
            params![since, apart],
            |r| r.get::<_, i64>(0),
        )? as u64;
        let mut stmt = conn.prepare(
            "SELECT template, item, SUM(drops), SUM(quantity) FROM drop_hours WHERE hour >= ?1 GROUP BY template, item",
        )?;
        for row in stmt.query_map(params![since], |r| {
            Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?, r.get::<_, i64>(2)?, r.get::<_, i64>(3)?))
        })? {
            let (t, i, d, q) = row?;
            out.drops.insert((t as u32, i as u32), (d as u64, q as u64));
        }
        Ok(out)
    }
}

/// The first bucket inside the seven days ending at `now_unix`.
pub fn window_start(now_unix: i64) -> i64 {
    bucket(now_unix) - KEEP_SECS + BUCKET_SECS
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i64 = 24 * 3600;

    /// Two channels' batches add up; distinct players are distinct across mobs and channels;
    /// and a kill eight days old is out of the sums and gone after the purge.
    #[test]
    fn batches_add_up_and_the_week_rolls() {
        let store = Store::open_in_memory().unwrap();
        let now = 1_800_000_000;
        let mut ch0 = KillBatch::default();
        ch0.note(now, 2, 200, &[(4_000_001, 1), (0, 5)]);
        ch0.note(now, 2, 201, &[(0, 4)]);
        ch0.note(now - 8 * DAY, 2, 202, &[(4_000_001, 1)]);
        let mut ch1 = KillBatch::default();
        ch1.note(now - DAY, 2, 200, &[]);
        ch1.note(now - DAY, 3, 203, &[(1_002_007, 1)]);
        store.flush_kill_stats(&ch0).unwrap();
        store.flush_kill_stats(&ch1).unwrap();

        let s = store.kill_stats(now).unwrap();
        assert_eq!(s.kills, 4, "the eight-day-old kill is outside the window");
        assert_eq!(s.players, 3, "200 on both channels is one player; 202 is too old");
        assert_eq!(s.mobs[&2], (3, 2));
        assert_eq!(s.mobs[&3], (1, 1));
        assert_eq!(s.drops[&(2, 0)], (2, 9), "two meso drops, 9 mesos");
        assert_eq!(s.drops[&(2, 4_000_001)], (1, 1));

        assert_eq!(store.purge_kill_stats(now).unwrap(), 3, "its kill, killer and drop rows");
        assert_eq!(store.kill_stats(now).unwrap(), s, "the purge removed only what was already outside");
        assert_eq!(store.kill_stats(now + 7 * DAY).unwrap().kills, 0, "a week later, nothing");
    }

    /// A source set apart (an opened box) keeps its own row but is not a kill or a killer.
    #[test]
    fn a_source_set_apart_is_out_of_the_totals_only() {
        let store = Store::open_in_memory().unwrap();
        let now = 1_800_000_000;
        let mut b = KillBatch::default();
        b.note(now, 2, 200, &[]);
        b.note(now, 2_430_000, 200, &[(1_002_007, 1)]);
        b.note(now, 2_430_000, 201, &[(2_000_000, 50)]);
        store.flush_kill_stats(&b).unwrap();
        let s = store.kill_stats_apart(now, Some(2_430_000)).unwrap();
        assert_eq!((s.kills, s.players), (1, 1), "201 only opened a box");
        assert_eq!(s.mobs[&2_430_000], (2, 2), "the box's own row is whole");
        assert_eq!(s.drops[&(2_430_000, 2_000_000)], (1, 50));
        assert_eq!(store.kill_stats(now).unwrap().kills, 3, "without it, everything counts");
    }

    #[test]
    fn a_bucket_is_the_start_of_its_hour() {
        assert_eq!(bucket(7_200), 7_200);
        assert_eq!(bucket(7_199), 3_600);
        assert_eq!(window_start(KEEP_SECS + 10), BUCKET_SECS, "168 buckets, the current one included");
    }
}
