//! **N uses per UTC day**, taken for a whole group at once or not at all.
//!
//! The owner, 2026-09-23: *"The PQ should also have an entry limit of 10 entries per character per
//! day."* [`crate::dailyperks`] is once per day; this is its many-times sibling, on the same
//! day number ([`crate::dailyperks::utc_day`] - UTC midnight, days since the epoch), so the
//! two cannot disagree about when a day ends.
//!
//! # Why a group, in one transaction
//!
//! A party enters together or not at all. Checking every member and then charging every
//! member as separate calls is the read-then-write pair `dailyperks` already warns about:
//! two leaders' clicks in flight could both see a member at 9 and both charge them. So
//! [`Store::take_daily_uses`] reads and writes every row inside one `BEGIN IMMEDIATE`, and
//! either charges all of them or none and names who was out.
//!
//! # Rows from an earlier day count as zero
//!
//! A row holds `(day, used)`. A row stamped with an earlier day is read as zero used and
//! overwritten; a row stamped **later** than today (a clock wound back) is read as its own
//! count, so winding the clock back cannot hand out fresh entries - the same rule as
//! `dailyperks`.

use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};

use crate::dailyperks::utc_day;
use crate::db::Store;
use crate::error::Result;

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS daily_count (
            character_id INTEGER NOT NULL,
            what         TEXT    NOT NULL,
            -- Days since 1970-01-01 UTC, as dailyperks::utc_day.
            day          INTEGER NOT NULL,
            used         INTEGER NOT NULL,
            PRIMARY KEY (character_id, what)
        );
        "#,
    )?;
    Ok(())
}

fn used_on(conn: &Connection, character: u32, what: &str, today: i64) -> Result<u32> {
    let row: Option<(i64, i64)> = conn
        .query_row(
            "SELECT day, used FROM daily_count WHERE character_id = ?1 AND what = ?2",
            params![character, what],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?;
    Ok(match row {
        Some((day, used)) if day >= today => u32::try_from(used).unwrap_or(u32::MAX),
        _ => 0,
    })
}

impl Store {
    /// How many times `character` has used `what` on the UTC day `now_secs` falls on.
    pub fn daily_uses(&self, character: u32, what: &str, now_secs: i64) -> Result<u32> {
        let conn = self.conn();
        create_tables(&conn)?;
        used_on(&conn, character, what, utc_day(now_secs))
    }

    /// [`Store::daily_uses`] against the wall clock.
    pub fn daily_uses_now(&self, character: u32, what: &str) -> Result<u32> {
        self.daily_uses(character, what, Store::now())
    }

    /// **Charge one use of `what` to every one of `characters`, or to none.** `Ok(Ok(()))`
    /// when all of them had a use left under `limit` and each was charged one;
    /// `Ok(Err(character))` names the first who had none, and nobody was charged.
    pub fn take_daily_uses(
        &self,
        characters: &[u32],
        what: &str,
        limit: u32,
        now_secs: i64,
    ) -> Result<std::result::Result<(), u32>> {
        let today = utc_day(now_secs);
        let mut conn = self.conn();
        create_tables(&conn)?;
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut counts = Vec::with_capacity(characters.len());
        for &c in characters {
            let used = used_on(&tx, c, what, today)?;
            if used >= limit {
                tx.commit()?;
                return Ok(Err(c));
            }
            counts.push((c, used));
        }
        for (c, used) in counts {
            tx.execute(
                "INSERT INTO daily_count (character_id, what, day, used) VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(character_id, what) DO UPDATE SET day = excluded.day, used = excluded.used",
                params![c, what, today.max(day_of(&tx, c, what)?), used + 1],
            )?;
        }
        tx.commit()?;
        Ok(Ok(()))
    }

    /// [`Store::take_daily_uses`] against the wall clock.
    pub fn take_daily_uses_now(&self, characters: &[u32], what: &str, limit: u32) -> Result<std::result::Result<(), u32>> {
        self.take_daily_uses(characters, what, limit, Store::now())
    }
}

/// The day a row is stamped with, or `i64::MIN` for no row - so a later-stamped row keeps its
/// stamp when charged rather than being pulled back to today.
fn day_of(conn: &Connection, character: u32, what: &str) -> Result<i64> {
    Ok(conn
        .query_row(
            "SELECT day FROM daily_count WHERE character_id = ?1 AND what = ?2",
            params![character, what],
            |r| r.get::<_, i64>(0),
        )
        .optional()?
        .unwrap_or(i64::MIN))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i64 = 86_400;
    const T: i64 = 20_000 * DAY + 3_600; // some UTC day, one hour in

    /// Ten a day, per character, all or nothing for a group, and back to ten at UTC midnight.
    #[test]
    fn a_group_is_charged_together_up_to_the_limit_and_the_day_resets_it() {
        let s = Store::open_in_memory().unwrap();
        for _ in 0..9 {
            assert_eq!(s.take_daily_uses(&[1, 2], "pq", 10, T).unwrap(), Ok(()));
        }
        // Character 2 spends its tenth alone.
        assert_eq!(s.take_daily_uses(&[2], "pq", 10, T).unwrap(), Ok(()));
        assert_eq!(s.daily_uses(1, "pq", T).unwrap(), 9);
        assert_eq!(s.daily_uses(2, "pq", T).unwrap(), 10);
        // Together: 2 is out, so NOBODY is charged - 1 keeps its tenth.
        assert_eq!(s.take_daily_uses(&[1, 2], "pq", 10, T).unwrap(), Err(2));
        assert_eq!(s.daily_uses(1, "pq", T).unwrap(), 9, "all or nothing");
        // Another thing is another count.
        assert_eq!(s.daily_uses(2, "other", T).unwrap(), 0);
        // One second before midnight is still today; midnight is a new day.
        let midnight = (T / DAY + 1) * DAY;
        assert_eq!(s.daily_uses(2, "pq", midnight - 1).unwrap(), 10);
        assert_eq!(s.daily_uses(2, "pq", midnight).unwrap(), 0);
        assert_eq!(s.take_daily_uses(&[1, 2], "pq", 10, midnight).unwrap(), Ok(()));
        assert_eq!(s.daily_uses(2, "pq", midnight).unwrap(), 1);
        // A clock wound back to yesterday does not re-open anything: today's row stands.
        assert_eq!(s.daily_uses(2, "pq", midnight - 5).unwrap(), 1, "a later row still counts");
    }
}
