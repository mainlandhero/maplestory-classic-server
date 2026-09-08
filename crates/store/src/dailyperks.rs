//! **One claim per perk per UTC day**, and the transition that says a claim actually moved.
//!
//! The owner, 2026-09-08: the Maple Administrator becomes a quality-of-life NPC offering three
//! things - 1000 Leaf Points, a level, and an AP/SP reset - *"each option is usable once per
//! day, and the daily allowance resets at UTC midnight"*.
//!
//! This module is the whole gate. It knows nothing about Leaf Points, experience or ability
//! points; it answers exactly one question - **did this claim just move from "not claimed
//! today" to "claimed today"?** - and it answers it once, atomically, so the caller can hang
//! every effect off the answer.
//!
//! # Why the answer is a transition and not a boolean read
//!
//! `CLAUDE.md`'s Heena section: `store::complete_quest` guarded correctly on
//! `state = InProgress` for weeks while the payout sat *outside* the match on its return
//! value, so a repeat click re-paid the quest. The guard was asked and its answer ignored.
//!
//! A `has_claimed_today()` read followed by a `mark_claimed()` write is the same bug waiting
//! for two packets in flight: both reads say "no", both writes say "yes", and the player gets
//! two levels. So there is no such pair here. [`Store::claim_daily_perk`] does the read and
//! the write **inside one `BEGIN IMMEDIATE` transaction** and returns
//! [`DailyClaimOutcome::Claimed`] only for the run that actually moved the row.
//!
//! # Why the day is a number and not a formatted date
//!
//! The stored value is **days since 1970-01-01, UTC** - `unix_seconds.div_euclid(86400)`.
//!
//! Unix time has no leap seconds and no zone, so that division *is* the UTC calendar date;
//! there is no local time anywhere on this path and no `chrono` in the workspace to
//! accidentally introduce one. `2026-09-08T23:59:59Z` and `2026-09-09T00:00:00Z` are one
//! second apart and land on different integers, which is the whole requirement, and there is
//! a test that says so in both directions.
//!
//! `div_euclid` rather than `/`: a negative timestamp (a clock that has not been set) must
//! floor toward the past rather than truncate toward zero, or the day before the epoch and
//! the epoch itself would collide.
//!
//! [`utc_date`] renders one of these numbers back into `YYYY-MM-DD` **for log lines only**.
//! Nothing gates on the string, so the two cannot disagree about what a day is.
//!
//! # A clock that goes backwards must not re-open a claim
//!
//! The upsert's predicate is `stored_day < today`, not `stored_day != today`. If the machine's
//! clock is wound back - which on a home server is an NTP correction, not a hypothetical - a
//! claim already stamped with a *later* day stays claimed rather than becoming free again.
//! Tested.
//!
//! # Scope
//!
//! A claim is keyed on `(scope, scope_id, perk)` rather than on a character id, and every perk
//! shipped today uses [`SCOPE_CHARACTER`]. The column exists because one of the three grants -
//! Leaf Points - lands in the **account**-wide cash wallet, so a player with three characters
//! banks three times the daily allowance. That is a policy decision rather than a bug, it is
//! recorded in `world::dailyperks::Perk::scope`, and this schema is what makes changing it a
//! one-line edit instead of a migration.

use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};

use crate::db::Store;
use crate::error::Result;

/// Seconds in a UTC day. Unix time contains no leap seconds, so this is exact rather than
/// approximate - which is the property the whole module rests on.
pub const SECONDS_PER_DAY: i64 = 86_400;

/// A claim scoped to one character. Every perk uses this today.
pub const SCOPE_CHARACTER: &str = "character";

/// A claim scoped to one account, so every character on it shares the one allowance.
///
/// **Nothing uses this yet**, and it is here rather than added later because the alternative
/// is a schema change on a live database to answer a question `world::dailyperks::Perk::scope`
/// can otherwise answer in one line. Named loudly rather than left implicit -
/// `CLAUDE.md`'s *built is not wired* cuts both ways, and an unused constant that says why it
/// is unused is not the same thing as dead code.
pub const SCOPE_ACCOUNT: &str = "account";

/// The UTC day a unix timestamp falls on: days since 1970-01-01.
///
/// This is the only place a timestamp becomes a date, and it never consults a time zone.
pub fn utc_day(unix_secs: i64) -> i64 {
    unix_secs.div_euclid(SECONDS_PER_DAY)
}

/// **Today's UTC day, off the wall clock.** The one clock in this feature.
///
/// It exists so `crates/world` never reads a timestamp of its own: a second
/// `SystemTime::now()` somewhere else is a second chance to divide it wrong, and the menu that
/// says "already used today" and the gate that refuses have to mean the same day or the screen
/// and the database disagree.
pub fn today() -> i64 {
    utc_day(Store::now())
}

/// The first instant of a UTC day, as a unix timestamp. The inverse of [`utc_day`], for tests
/// and for log lines that want to say when the next claim opens.
pub fn utc_day_start(day: i64) -> i64 {
    day.saturating_mul(SECONDS_PER_DAY)
}

/// `YYYY-MM-DD` for a day number. **Log lines only** - nothing gates on this string.
///
/// Howard Hinnant's `civil_from_days`, which is exact for the whole proleptic Gregorian range
/// rather than only for dates near today. Written out rather than pulled in because the
/// workspace has no date crate and adding one to render a log line would be a poor trade.
pub fn utc_date(day: i64) -> String {
    let z = day + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as i64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

/// What [`Store::claim_daily_perk`] did.
///
/// **Only [`DailyClaimOutcome::Claimed`] may pay anything.** The type exists to make that a
/// property of the code rather than a rule somebody has to remember: there is no field on
/// [`DailyClaimOutcome::AlreadyToday`] a caller could read to work out what to grant, exactly
/// the way `world::taxi::Step` puts the destination map only on the arm that took the fare.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DailyClaimOutcome {
    /// The row moved from "not claimed today" to "claimed today". **This is the transition.**
    ///
    /// `previous` is the day the row held before - `None` when there was no row at all - and
    /// exists for exactly one purpose: [`Store::release_daily_perk`], so a grant that could
    /// not be applied does not cost the player their day.
    Claimed { day: i64, previous: Option<i64> },
    /// Already claimed today. Nothing was written and nothing may be granted.
    AlreadyToday { day: i64 },
}

impl DailyClaimOutcome {
    /// The UTC day the outcome is about, whichever arm it is.
    pub fn day(&self) -> i64 {
        match self {
            DailyClaimOutcome::Claimed { day, .. } => *day,
            DailyClaimOutcome::AlreadyToday { day } => *day,
        }
    }

    pub fn is_claimed(&self) -> bool {
        matches!(self, DailyClaimOutcome::Claimed { .. })
    }
}

/// One row per `(scope, scope_id, perk)`, holding only the last day it was claimed.
///
/// **A whole new table**, so `CREATE TABLE IF NOT EXISTS` is enough - unlike
/// `abilityspend::create_tables`, which had to `ALTER` columns onto `characters`.
///
/// No history is kept. A ledger of every claim ever made would grow without bound on a server
/// that is meant to run for months, and nothing in the feature can answer a question about
/// yesterday: the gate needs the last day and nothing else. `claimed_at` is the raw unix
/// second the claim was stamped, kept beside the day for audit - a row whose `claimed_at` does
/// not fall inside `claimed_day` is a clock that moved, and that is worth being able to see.
pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS daily_claim (
            -- 'character' or 'account'. See SCOPE_CHARACTER / SCOPE_ACCOUNT.
            scope      TEXT    NOT NULL,
            -- The character id or the account id, depending on `scope`.
            scope_id   INTEGER NOT NULL,
            perk       TEXT    NOT NULL,
            -- Days since 1970-01-01 UTC. NEVER a local date - see utc_day().
            claimed_day INTEGER NOT NULL,
            -- The unix second the claim was stamped. Audit only; nothing gates on it.
            claimed_at  INTEGER NOT NULL,
            PRIMARY KEY (scope, scope_id, perk)
        );
        "#,
    )?;
    Ok(())
}

impl Store {
    /// The UTC day this perk was last claimed on, or `None` if it never was.
    ///
    /// A read, for showing "you have already had this today" on the menu. **It is not the
    /// guard** - see [`Store::claim_daily_perk`] for why a read-then-write pair is not one.
    pub fn daily_claim_day(&self, scope: &str, scope_id: i64, perk: &str) -> Result<Option<i64>> {
        let conn = self.conn();
        create_tables(&conn)?;
        let day = conn
            .query_row(
                "SELECT claimed_day FROM daily_claim
                  WHERE scope = ?1 AND scope_id = ?2 AND perk = ?3",
                params![scope, scope_id, perk],
                |r| r.get::<_, i64>(0),
            )
            .optional()?;
        Ok(day)
    }

    /// **Claim this perk for the UTC day `now_secs` falls on.** One transaction, one answer.
    ///
    /// Returns [`DailyClaimOutcome::Claimed`] for the caller that actually moved the row and
    /// [`DailyClaimOutcome::AlreadyToday`] for every other caller that day. Two concurrent
    /// claims cannot both come back `Claimed`: the read and the write are inside one
    /// `BEGIN IMMEDIATE`, so the second one blocks on the write lock and then reads the first
    /// one's row.
    ///
    /// The predicate is `stored < today`, so a clock wound backwards leaves a later claim
    /// standing rather than re-opening it.
    pub fn claim_daily_perk(
        &self,
        scope: &str,
        scope_id: i64,
        perk: &str,
        now_secs: i64,
    ) -> Result<DailyClaimOutcome> {
        let today = utc_day(now_secs);
        let mut conn = self.conn();
        create_tables(&conn)?;
        // IMMEDIATE rather than the default DEFERRED: the transaction takes the write lock up
        // front, so the SELECT below cannot be read by two writers who then both decide they
        // are the one that moved the row.
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let previous: Option<i64> = tx
            .query_row(
                "SELECT claimed_day FROM daily_claim
                  WHERE scope = ?1 AND scope_id = ?2 AND perk = ?3",
                params![scope, scope_id, perk],
                |r| r.get::<_, i64>(0),
            )
            .optional()?;
        if let Some(stored) = previous {
            if stored >= today {
                // Already had it today - or the clock moved backwards and the row is stamped
                // in the future, which must also refuse. Nothing is written.
                tx.commit()?;
                return Ok(DailyClaimOutcome::AlreadyToday { day: stored });
            }
        }
        tx.execute(
            "INSERT INTO daily_claim (scope, scope_id, perk, claimed_day, claimed_at)
                  VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(scope, scope_id, perk) DO UPDATE
                    SET claimed_day = excluded.claimed_day,
                        claimed_at  = excluded.claimed_at",
            params![scope, scope_id, perk, today, now_secs],
        )?;
        tx.commit()?;
        Ok(DailyClaimOutcome::Claimed { day: today, previous })
    }

    /// [`Store::claim_daily_perk`] against the wall clock.
    pub fn claim_daily_perk_now(
        &self,
        scope: &str,
        scope_id: i64,
        perk: &str,
    ) -> Result<DailyClaimOutcome> {
        self.claim_daily_perk(scope, scope_id, perk, Store::now())
    }

    /// **Put a claim back**, because the grant that hung off it could not be applied.
    ///
    /// This is the one thing that keeps "claim first, then grant" honest without a distributed
    /// transaction across four unrelated tables. A `save_character_progress` that fails after
    /// the day was stamped would otherwise cost the player their level *and* give them nothing;
    /// the caller passes back the `previous` from [`DailyClaimOutcome::Claimed`] and the row
    /// returns to exactly what it was.
    ///
    /// `restore_to = None` deletes the row, which is what "there was no row before" means.
    ///
    /// **It is deliberately not a general un-claim.** It takes the previous day rather than
    /// computing one, so it cannot be used to hand somebody a second go at today - the only
    /// value that makes it a no-op-in-reverse is the one the claim itself returned.
    pub fn release_daily_perk(
        &self,
        scope: &str,
        scope_id: i64,
        perk: &str,
        restore_to: Option<i64>,
    ) -> Result<()> {
        let conn = self.conn();
        create_tables(&conn)?;
        match restore_to {
            Some(day) => {
                conn.execute(
                    "UPDATE daily_claim SET claimed_day = ?4
                      WHERE scope = ?1 AND scope_id = ?2 AND perk = ?3",
                    params![scope, scope_id, perk, day],
                )?;
            }
            None => {
                conn.execute(
                    "DELETE FROM daily_claim WHERE scope = ?1 AND scope_id = ?2 AND perk = ?3",
                    params![scope, scope_id, perk],
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::FIRST_CHARACTER_ID;

    const PERK: &str = "levelup";
    const OTHER: &str = "leafpoints";

    fn store_with_character() -> (Store, i64) {
        let (store, _, id) = store_with_account();
        (store, id)
    }

    fn store_with_account() -> (Store, i64, i64) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("player_one", "correct horse battery").unwrap();
        let chr = store
            .create_character(
                account,
                0,
                &net::opcode::Character { name: "Tester".to_string(), ..Default::default() },
            )
            .unwrap();
        assert!(chr.id >= FIRST_CHARACTER_ID, "ids start at 200, never at 1");
        (store, account, i64::from(chr.id))
    }

    /// `2026-09-08T00:00:00Z`. Derived from the day number rather than typed, so the constant
    /// and the arithmetic under test cannot both be wrong in the same direction.
    fn midnight(date: &str) -> i64 {
        // Search rather than convert: the range is small and this needs no second algorithm.
        for day in 20_000..21_000 {
            if utc_date(day) == date {
                return utc_day_start(day);
            }
        }
        panic!("{date} is outside the range this helper searches");
    }

    // -- the day arithmetic --------------------------------------------------------------

    /// **The boundary, stated the way the requirement states it.** 23:59:59Z and 00:00:00Z are
    /// one second apart and are two different days.
    #[test]
    fn one_second_across_utc_midnight_is_two_different_days() {
        let start = midnight("2026-09-08");
        let last_second = start + SECONDS_PER_DAY - 1; // 2026-09-08T23:59:59Z
        let next_midnight = start + SECONDS_PER_DAY; //   2026-09-09T00:00:00Z
        assert_eq!(next_midnight - last_second, 1, "they really are one second apart");
        assert_ne!(utc_day(last_second), utc_day(next_midnight));
        assert_eq!(utc_day(next_midnight), utc_day(last_second) + 1);
        assert_eq!(utc_date(utc_day(last_second)), "2026-09-08");
        assert_eq!(utc_date(utc_day(next_midnight)), "2026-09-09");
    }

    /// The other half of the same claim: every instant *inside* a day is the same day. The
    /// boundary test above passes for a function that returns the timestamp unchanged.
    #[test]
    fn every_instant_within_one_utc_day_is_the_same_day() {
        let start = midnight("2026-09-08");
        let day = utc_day(start);
        for offset in [0, 1, 3599, 3600, 43_200, SECONDS_PER_DAY - 2, SECONDS_PER_DAY - 1] {
            assert_eq!(utc_day(start + offset), day, "offset {offset} left the day");
        }
        assert_ne!(utc_day(start - 1), day, "and the second before it did not");
    }

    /// **Nothing here consults a local zone**, and this is the test that would catch it if
    /// something did: a UTC day boundary is a fixed integer, so no offset in `-12..=+14` hours
    /// can shift it. A `chrono::Local` slipped into `utc_day` would fail here on every machine
    /// that is not on UTC.
    #[test]
    fn the_boundary_does_not_move_with_any_time_zone_offset() {
        let start = midnight("2026-09-08");
        assert_eq!(utc_day(start), (start / SECONDS_PER_DAY), "exact division, no remainder");
        for hours in -12i64..=14 {
            let shifted = start + hours * 3600;
            let expect = if hours < 0 { utc_day(start) - 1 } else { utc_day(start) };
            assert_eq!(utc_day(shifted), expect, "{hours}h from UTC midnight");
        }
    }

    /// A clock that has never been set reads as a small or negative number. `div_euclid`
    /// floors, so the day before the epoch is `-1` rather than colliding with `0`.
    #[test]
    fn a_timestamp_before_the_epoch_floors_rather_than_truncating() {
        assert_eq!(utc_day(0), 0);
        assert_eq!(utc_day(-1), -1, "one second before the epoch is the previous day");
        assert_eq!(utc_day(-SECONDS_PER_DAY), -1);
        assert_eq!(utc_day(-SECONDS_PER_DAY - 1), -2);
        assert_eq!(utc_date(0), "1970-01-01");
        assert_eq!(utc_date(-1), "1969-12-31");
    }

    /// The date renderer against dates chosen to break a naive one: a leap day, the day after
    /// it, a century that is not a leap year, and one that is.
    #[test]
    fn the_date_renderer_handles_leap_years_and_centuries() {
        for (day, date) in [
            (0i64, "1970-01-01"),
            (11_016, "2000-02-29"), // a leap year that IS divisible by 100
            (11_017, "2000-03-01"),
            (18_993, "2022-01-01"),
            (20_704, "2026-09-08"),
        ] {
            assert_eq!(utc_date(day), date, "day {day}");
        }
        // Round-trips, which is the property that does not depend on any of the constants
        // above being right.
        for day in [-3650i64, 0, 1, 11_016, 18_993, 20_704, 40_000] {
            assert_eq!(utc_day(utc_day_start(day)), day);
        }
    }

    // -- the claim -----------------------------------------------------------------------

    #[test]
    fn a_fresh_character_has_never_claimed() {
        let (s, id) = store_with_character();
        assert_eq!(s.daily_claim_day(SCOPE_CHARACTER, id, PERK).unwrap(), None);
    }

    /// **The same second twice must not grant twice.** The requirement, verbatim.
    #[test]
    fn the_same_second_twice_claims_once() {
        let (s, id) = store_with_character();
        let t = midnight("2026-09-08") + 12 * 3600;
        let first = s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, t).unwrap();
        assert!(first.is_claimed(), "the first call is the transition");
        assert_eq!(first, DailyClaimOutcome::Claimed { day: utc_day(t), previous: None });
        for _ in 0..5 {
            let again = s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, t).unwrap();
            assert_eq!(again, DailyClaimOutcome::AlreadyToday { day: utc_day(t) });
            assert!(!again.is_claimed(), "and nothing after it is");
        }
    }

    /// **23:59:59Z then 00:00:00Z**: two claims, one second apart, both granted.
    #[test]
    fn a_claim_at_the_last_second_of_a_day_reopens_one_second_later() {
        let (s, id) = store_with_character();
        let last = midnight("2026-09-08") + SECONDS_PER_DAY - 1;
        let next = last + 1;

        let a = s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, last).unwrap();
        assert!(a.is_claimed());
        assert_eq!(utc_date(a.day()), "2026-09-08");
        // Still the same day one second earlier in the same day - the refusal is real.
        assert!(!s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, last).unwrap().is_claimed());

        let b = s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, next).unwrap();
        assert!(b.is_claimed(), "UTC midnight reopened it");
        assert_eq!(utc_date(b.day()), "2026-09-09");
        assert_eq!(b, DailyClaimOutcome::Claimed { day: utc_day(next), previous: Some(utc_day(last)) });
        // And the new day is now closed too.
        assert!(!s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, next).unwrap().is_claimed());
    }

    /// One perk's claim says nothing about another's, and one character's says nothing about
    /// another's. Three options that share a gate would be one option.
    #[test]
    fn perks_and_scopes_are_gated_independently() {
        let (s, account, id) = store_with_account();
        let other = s
            .create_character(
                account,
                0,
                &net::opcode::Character { name: "Second".to_string(), ..Default::default() },
            )
            .unwrap();
        let t = midnight("2026-09-08");

        assert!(s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, t).unwrap().is_claimed());
        assert!(
            s.claim_daily_perk(SCOPE_CHARACTER, id, OTHER, t).unwrap().is_claimed(),
            "a different perk on the same character is a different claim"
        );
        assert!(
            s.claim_daily_perk(SCOPE_CHARACTER, i64::from(other.id), PERK, t).unwrap().is_claimed(),
            "the same perk on a different character is a different claim"
        );
        assert!(
            s.claim_daily_perk(SCOPE_ACCOUNT, id, PERK, t).unwrap().is_claimed(),
            "and the account scope shares no row with the character scope"
        );
        // All four are now closed for the day.
        assert!(!s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, t).unwrap().is_claimed());
        assert!(!s.claim_daily_perk(SCOPE_CHARACTER, id, OTHER, t).unwrap().is_claimed());
        assert!(!s
            .claim_daily_perk(SCOPE_CHARACTER, i64::from(other.id), PERK, t)
            .unwrap()
            .is_claimed());
        assert!(!s.claim_daily_perk(SCOPE_ACCOUNT, id, PERK, t).unwrap().is_claimed());
    }

    /// An NTP correction that winds the clock back must not hand out a second claim.
    #[test]
    fn a_clock_wound_backwards_does_not_reopen_a_claim() {
        let (s, id) = store_with_character();
        let today = midnight("2026-09-08");
        assert!(s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, today).unwrap().is_claimed());

        let yesterday = today - SECONDS_PER_DAY;
        let out = s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, yesterday).unwrap();
        assert_eq!(
            out,
            DailyClaimOutcome::AlreadyToday { day: utc_day(today) },
            "the stored day is later than the clock, and that refuses"
        );
        assert_eq!(
            s.daily_claim_day(SCOPE_CHARACTER, id, PERK).unwrap(),
            Some(utc_day(today)),
            "and the stored day was NOT moved backwards"
        );
    }

    /// A day skipped entirely still opens: the gate is "is the stored day earlier than
    /// today", not "is it exactly yesterday".
    #[test]
    fn a_gap_of_several_days_reopens_the_claim() {
        let (s, id) = store_with_character();
        let t = midnight("2026-09-08");
        assert!(s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, t).unwrap().is_claimed());
        let much_later = t + 30 * SECONDS_PER_DAY;
        let out = s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, much_later).unwrap();
        assert_eq!(
            out,
            DailyClaimOutcome::Claimed {
                day: utc_day(much_later),
                previous: Some(utc_day(t))
            }
        );
    }

    // -- the release ---------------------------------------------------------------------

    /// A grant that could not be applied gives the day back, and the row returns to exactly
    /// what it was - which for a first claim is *no row at all*.
    #[test]
    fn releasing_a_first_claim_removes_the_row_entirely() {
        let (s, id) = store_with_character();
        let t = midnight("2026-09-08");
        let DailyClaimOutcome::Claimed { previous, .. } =
            s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, t).unwrap()
        else {
            panic!("the first claim is a transition")
        };
        assert_eq!(previous, None);
        s.release_daily_perk(SCOPE_CHARACTER, id, PERK, previous).unwrap();
        assert_eq!(s.daily_claim_day(SCOPE_CHARACTER, id, PERK).unwrap(), None);
        assert!(
            s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, t).unwrap().is_claimed(),
            "and the player may try again today"
        );
    }

    /// Releasing a later claim restores the earlier day rather than deleting the row, so
    /// yesterday's claim is not resurrected as a free go.
    #[test]
    fn releasing_a_later_claim_restores_the_previous_day() {
        let (s, id) = store_with_character();
        let day1 = midnight("2026-09-08");
        let day2 = day1 + SECONDS_PER_DAY;
        s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, day1).unwrap();
        let DailyClaimOutcome::Claimed { previous, .. } =
            s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, day2).unwrap()
        else {
            panic!("a new day is a transition")
        };
        assert_eq!(previous, Some(utc_day(day1)));
        s.release_daily_perk(SCOPE_CHARACTER, id, PERK, previous).unwrap();
        assert_eq!(s.daily_claim_day(SCOPE_CHARACTER, id, PERK).unwrap(), Some(utc_day(day1)));
        assert!(
            s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, day2).unwrap().is_claimed(),
            "day 2 is free again"
        );
    }

    // -- persistence ---------------------------------------------------------------------

    /// A claim outlives the process. A gate that resets on restart is not a daily gate.
    #[test]
    fn a_claim_survives_a_reopen() {
        let dir = std::env::temp_dir().join(format!("maplecw-dailyperks-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("maplecw.db");
        let _ = std::fs::remove_file(&path);
        let t = midnight("2026-09-08");

        let id = {
            let s = Store::open(&path).unwrap();
            let account = s.create_account("player_one", "correct horse battery").unwrap();
            let chr = s
                .create_character(
                    account,
                    0,
                    &net::opcode::Character { name: "Tester".to_string(), ..Default::default() },
                )
                .unwrap();
            assert!(s
                .claim_daily_perk(SCOPE_CHARACTER, i64::from(chr.id), PERK, t)
                .unwrap()
                .is_claimed());
            i64::from(chr.id)
        };

        let s = Store::open(&path).unwrap();
        assert_eq!(s.daily_claim_day(SCOPE_CHARACTER, id, PERK).unwrap(), Some(utc_day(t)));
        assert!(
            !s.claim_daily_perk(SCOPE_CHARACTER, id, PERK, t).unwrap().is_claimed(),
            "a relog is not a new day"
        );
        let _ = std::fs::remove_file(&path);
    }

    /// The wall-clock wrapper reaches the same row as the explicit-timestamp one. Without this
    /// the whole module could be correct and the thing the server actually calls could be
    /// gating on something else.
    #[test]
    fn the_wall_clock_wrapper_uses_the_same_row() {
        let (s, id) = store_with_character();
        assert!(s.claim_daily_perk_now(SCOPE_CHARACTER, id, PERK).unwrap().is_claimed());
        assert!(!s.claim_daily_perk_now(SCOPE_CHARACTER, id, PERK).unwrap().is_claimed());
        assert_eq!(
            s.daily_claim_day(SCOPE_CHARACTER, id, PERK).unwrap(),
            Some(utc_day(Store::now())),
            "and it stamped today's UTC day"
        );
    }
}
