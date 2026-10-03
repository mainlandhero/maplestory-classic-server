//! **Every scroll used on the server, by its listed success rate** - for the drop-table page
//! (`world::dropweb`).
//!
//! The owner, 2026-10-03: *"it should also be tracking scroll success chances across the server
//! (without the usage of Lucky Day Scroll), which tracks the current overall scroll success
//! percentage chance across all usages depending on the scroll's success percent."*
//!
//! One row per listed rate (10, 30, 60, 70, 100, ...): how many scrolls at that rate were used,
//! how many succeeded and how many destroyed their item. **All time**, not the drop table's seven
//! days - "across all usages". A use under a Lucky Day mark is guaranteed whatever its rate, so
//! the caller does not record it at all (`world::session::realscroll`).
//!
//! Written once per use, directly: a scroll is a deliberate click, nowhere near the rate of a
//! kill, so there is nothing to batch.

use rusqlite::{params, Connection};

use crate::error::Result;
use crate::Store;

/// One listed rate's tally.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ScrollRate {
    /// The scroll's own success percent, as listed.
    pub success_pct: u16,
    pub uses: u64,
    pub successes: u64,
    /// Failures that also destroyed the item (a cursed scroll's second roll).
    pub destroyed: u64,
}

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS scroll_rates (
            success_pct INTEGER PRIMARY KEY,
            uses        INTEGER NOT NULL,
            successes   INTEGER NOT NULL,
            destroyed   INTEGER NOT NULL
        );
        "#,
    )?;
    Ok(())
}

impl Store {
    /// Count one use of a scroll listed at `success_pct`.
    pub fn record_scroll_use(&self, success_pct: u16, succeeded: bool, destroyed: bool) -> Result<()> {
        self.conn().execute(
            "INSERT INTO scroll_rates (success_pct, uses, successes, destroyed) VALUES (?1, 1, ?2, ?3)
             ON CONFLICT(success_pct) DO UPDATE SET
                uses = uses + 1,
                successes = successes + excluded.successes,
                destroyed = destroyed + excluded.destroyed",
            params![i64::from(success_pct), i64::from(succeeded), i64::from(destroyed)],
        )?;
        Ok(())
    }

    /// Every listed rate that has been used, lowest first.
    pub fn scroll_stats(&self) -> Result<Vec<ScrollRate>> {
        let conn = self.conn();
        let mut stmt =
            conn.prepare("SELECT success_pct, uses, successes, destroyed FROM scroll_rates ORDER BY success_pct")?;
        let rows = stmt.query_map([], |r| {
            Ok(ScrollRate {
                success_pct: r.get::<_, i64>(0)? as u16,
                uses: r.get::<_, i64>(1)? as u64,
                successes: r.get::<_, i64>(2)? as u64,
                destroyed: r.get::<_, i64>(3)? as u64,
            })
        })?;
        Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_add_up_per_listed_rate_and_come_back_lowest_first() {
        let store = Store::open_in_memory().unwrap();
        assert!(store.scroll_stats().unwrap().is_empty());
        store.record_scroll_use(60, true, false).unwrap();
        store.record_scroll_use(60, false, false).unwrap();
        store.record_scroll_use(10, false, true).unwrap();
        store.record_scroll_use(60, true, false).unwrap();
        assert_eq!(
            store.scroll_stats().unwrap(),
            vec![
                ScrollRate { success_pct: 10, uses: 1, successes: 0, destroyed: 1 },
                ScrollRate { success_pct: 60, uses: 3, successes: 2, destroyed: 0 },
            ]
        );
    }
}
