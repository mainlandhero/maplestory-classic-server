//! **Fame** - the up and down arrows on another player's Character Info window.
//!
//! The owner, 2026-09-18: *"Both the fame and the defame functionality should be available once
//! per character. A single character is allowed to fame another character once per day
//! (reset at midnight UTC), they are not allowed to fame the same character twice in a week,
//! resets on Monday midnight UTC."*
//!
//! Two rules, both about the **giver**, both in UTC, both here rather than in the session so
//! the gate and the number on screen are one fact:
//!
//! * **one gift a day**, raise or drop, to anyone - the day turns at 00:00 UTC;
//! * **one gift a week to the same character** - the week turns at Monday 00:00 UTC.
//!
//! The gift itself is a row in `fame_log` and a `+1`/`-1` on `characters.fame`, in one
//! transaction. The log is the guard: a rule that lived in a flag on the character would be
//! one column per rule, and the week rule needs a row per target anyway. Nothing prunes the
//! log; a row is 40 bytes and a character can write at most one a day.
//!
//! `characters.fame` did not exist before this - the character record sent a literal `0` for
//! the stat row the client names `fame` (`net::opcode::character_record`), which is why every
//! window said 0. It is ALTERed on with the usual `PRAGMA table_info` guard.

use rusqlite::{Connection, OptionalExtension};

use crate::db::Store;
use crate::error::Result;

const SECONDS_PER_DAY: i64 = 86_400;

/// The unix second the giver's **current day** and **current week** began, in UTC: 00:00
/// today, and 00:00 last Monday (or today, if it is Monday). Pure, so the calendar arithmetic
/// is a unit test - a `SystemTime` buried in a method would not be. 1970-01-01 was a
/// Thursday, which is the `+ 3` below.
pub fn fame_windows(now: i64) -> FameWindows {
    let day = now.div_euclid(SECONDS_PER_DAY);
    let weekday = (day + 3).rem_euclid(7); // 0 = Monday .. 6 = Sunday
    FameWindows {
        day_start: day * SECONDS_PER_DAY,
        week_start: (day - weekday) * SECONDS_PER_DAY,
    }
}

/// See [`fame_windows`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FameWindows {
    pub day_start: i64,
    pub week_start: i64,
}

/// What a gift of fame came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FameOutcome {
    /// Written: the target's fame is `fame` now, and `target_name` is what to print.
    Given { target_name: String, fame: i32 },
    /// The giver already gave fame (either way) since 00:00 UTC today.
    AlreadyToday,
    /// The giver already gave fame to this target since Monday 00:00 UTC.
    SameTargetThisWeek,
    /// No character by that id, or the giver is the target.
    NoSuchTarget,
}

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("PRAGMA table_info(characters)")?;
    let exists = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<std::result::Result<Vec<_>, _>>()?
        .iter()
        .any(|name| name == "fame");
    drop(stmt);
    if !exists {
        conn.execute("ALTER TABLE characters ADD COLUMN fame INTEGER NOT NULL DEFAULT 0", [])?;
    }
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS fame_log (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            giver     INTEGER NOT NULL,
            target    INTEGER NOT NULL,
            -- 1 = raised, 0 = dropped. Kept for the record; neither rule reads it.
            raise     INTEGER NOT NULL,
            -- The unix second, UTC. Both rules are windows over this.
            given_at  INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS fame_log_giver ON fame_log (giver, given_at);
        "#,
    )?;
    Ok(())
}

impl Store {
    /// A character's fame, or `None` for no such character.
    pub fn fame(&self, character_id: u32) -> Result<Option<i32>> {
        Ok(self
            .conn()
            .query_row(
                "SELECT fame FROM characters WHERE id = ?1",
                rusqlite::params![character_id],
                |row| row.get(0),
            )
            .optional()?)
    }

    /// **`giver` raises (or drops) `target`'s fame at `now`**, under the two rules in the
    /// module docs. One transaction: the checks, the log row and the `+1`/`-1` commit
    /// together or not at all, and a refusal writes nothing.
    ///
    /// `now` is a parameter so the calendar is the caller's (and a test's), not this file's.
    pub fn give_fame(&self, giver: u32, target: u32, raise: bool, now: i64) -> Result<FameOutcome> {
        if giver == target {
            return Ok(FameOutcome::NoSuchTarget);
        }
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let Some(target_name) = tx
            .query_row(
                "SELECT name FROM characters WHERE id = ?1",
                rusqlite::params![target],
                |row| row.get::<_, String>(0),
            )
            .optional()?
        else {
            return Ok(FameOutcome::NoSuchTarget);
        };
        let w = fame_windows(now);
        let today: i64 = tx.query_row(
            "SELECT COUNT(*) FROM fame_log WHERE giver = ?1 AND given_at >= ?2",
            rusqlite::params![giver, w.day_start],
            |row| row.get(0),
        )?;
        if today > 0 {
            return Ok(FameOutcome::AlreadyToday);
        }
        let this_week: i64 = tx.query_row(
            "SELECT COUNT(*) FROM fame_log WHERE giver = ?1 AND target = ?2 AND given_at >= ?3",
            rusqlite::params![giver, target, w.week_start],
            |row| row.get(0),
        )?;
        if this_week > 0 {
            return Ok(FameOutcome::SameTargetThisWeek);
        }
        tx.execute(
            "INSERT INTO fame_log (giver, target, raise, given_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![giver, target, u8::from(raise), now],
        )?;
        tx.execute(
            "UPDATE characters SET fame = fame + ?2 WHERE id = ?1",
            rusqlite::params![target, if raise { 1 } else { -1 }],
        )?;
        let fame: i32 = tx.query_row(
            "SELECT fame FROM characters WHERE id = ?1",
            rusqlite::params![target],
            |row| row.get(0),
        )?;
        tx.commit()?;
        Ok(FameOutcome::Given { target_name, fame })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use net::opcode::Character;

    /// 2026-09-18 is a Friday. 2026-09-14 00:00 UTC is the Monday before it.
    const FRI_2026_09_18_15_00: i64 = 1_789_743_600; // 2026-09-18T15:00:00Z
    const MON_2026_09_14: i64 = 1_789_344_000; // 2026-09-14T00:00:00Z

    #[test]
    fn the_day_turns_at_midnight_utc_and_the_week_on_monday_midnight_utc() {
        let w = fame_windows(FRI_2026_09_18_15_00);
        assert_eq!(w.day_start, 1_789_689_600, "2026-09-18T00:00:00Z");
        assert_eq!(w.week_start, MON_2026_09_14);
        // On the Monday itself the week starts today.
        let w = fame_windows(MON_2026_09_14 + 5);
        assert_eq!(w.day_start, MON_2026_09_14);
        assert_eq!(w.week_start, MON_2026_09_14);
        // One second before, it is still last week (Monday 2026-09-07).
        let w = fame_windows(MON_2026_09_14 - 1);
        assert_eq!(w.week_start, MON_2026_09_14 - 7 * SECONDS_PER_DAY);
        assert_eq!(w.day_start, MON_2026_09_14 - SECONDS_PER_DAY, "Sunday");
        // 1970-01-01 was a Thursday: its week began on Monday 1969-12-29.
        assert_eq!(fame_windows(0).week_start, -3 * SECONDS_PER_DAY);
    }

    fn two_characters() -> (Store, u32, u32, u32) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        assert_eq!(account, 1, "the first account; characters_for(1, 0) below relies on it");
        let mk = |name: &str| {
            store
                .create_character(account, 0, &Character { name: name.into(), ..Character::default() })
                .unwrap()
                .id
        };
        let (wisp, t2, t3) = (mk("Wisp"), mk("Tester2"), mk("Tester3"));
        (store, wisp, t2, t3)
    }

    /// **Once a day, whoever the target; once a week to the same target; both in UTC.** And
    /// every refusal writes nothing: the fame and the log are unchanged after it.
    #[test]
    fn one_gift_a_day_and_one_a_week_per_target() {
        let (store, wisp, t2, t3) = two_characters();
        let now = FRI_2026_09_18_15_00;
        assert_eq!(store.fame(t2).unwrap(), Some(0), "the column exists and starts at 0");
        assert_eq!(
            store.give_fame(wisp, t2, true, now).unwrap(),
            FameOutcome::Given { target_name: "Tester2".into(), fame: 1 }
        );
        // Same day, another target: refused - one a day.
        assert_eq!(store.give_fame(wisp, t3, true, now + 3_600).unwrap(), FameOutcome::AlreadyToday);
        assert_eq!(store.fame(t3).unwrap(), Some(0), "a refusal moves nothing");
        // 23:59:59 UTC the same day: still today.
        assert_eq!(store.give_fame(wisp, t3, false, 1_789_689_600 + SECONDS_PER_DAY - 1).unwrap(), FameOutcome::AlreadyToday);
        // 00:00 UTC Saturday: a new day, but Tester2 was this week -> refused; Tester3 is fine.
        let sat = 1_789_689_600 + SECONDS_PER_DAY;
        assert_eq!(store.give_fame(wisp, t2, false, sat).unwrap(), FameOutcome::SameTargetThisWeek);
        assert_eq!(store.fame(t2).unwrap(), Some(1));
        assert_eq!(store.give_fame(wisp, t3, false, sat).unwrap(), FameOutcome::Given { target_name: "Tester3".into(), fame: -1 });
        // Sunday: Tester2 is still this week.
        assert_eq!(store.give_fame(wisp, t2, true, sat + SECONDS_PER_DAY).unwrap(), FameOutcome::SameTargetThisWeek);
        // Monday 00:00 UTC: the week turned - Tester2 again is allowed.
        let next_mon = MON_2026_09_14 + 7 * SECONDS_PER_DAY;
        assert_eq!(store.give_fame(wisp, t2, true, next_mon).unwrap(), FameOutcome::Given { target_name: "Tester2".into(), fame: 2 });
        // The rules are per GIVER: Tester3 famed nobody today and may fame the owner now.
        assert_eq!(store.give_fame(t3, wisp, true, next_mon).unwrap(), FameOutcome::Given { target_name: "Wisp".into(), fame: 1 });
        // Yourself, or nobody: refused, nothing written.
        assert_eq!(store.give_fame(t3, t3, true, next_mon + 100_000).unwrap(), FameOutcome::NoSuchTarget);
        assert_eq!(store.give_fame(t3, 9_999, true, next_mon + 100_000).unwrap(), FameOutcome::NoSuchTarget);
        let rows: i64 = store.conn().query_row("SELECT COUNT(*) FROM fame_log", [], |r| r.get(0)).unwrap();
        assert_eq!(rows, 4, "one log row per gift that went through");
    }

    /// The column lands on a database that predates it, and the character record reads it.
    #[test]
    fn the_fame_column_is_added_to_an_old_database_and_read_into_the_record() {
        let (store, wisp, t2, _) = two_characters();
        store.give_fame(wisp, t2, true, FRI_2026_09_18_15_00).unwrap();
        let chars = store.characters_for(1, 0).unwrap();
        let t2_record = chars.iter().find(|c| c.id == t2).unwrap();
        assert_eq!(t2_record.fame, 1, "the record carries the column");
        assert_eq!(chars.iter().find(|c| c.id == wisp).unwrap().fame, 0);
        // Re-opening (the migration runs again) is idempotent: the column is not re-added.
        crate::fame::create_tables(&store.conn()).unwrap();
        assert_eq!(store.fame(t2).unwrap(), Some(1));
    }
}
