//! The server's EXP, meso and drop multipliers.
//!
//! # Why this is in the database and not in a process
//!
//! The owner asked for the **global server's** rate, and a channel is a process:
//! `crates/world/src/lib.rs` says so and `tools/test-server.ps1` starts two of them. Two
//! processes share exactly one thing here, the SQLite file, so that is where a server-wide
//! setting has to live. A `static` would give channel 1 a different rate from channel 2 and
//! the bug would look like the multiplier being ignored.
//!
//! It persists across a restart as a side effect. That is the honest consequence of putting
//! it in the database rather than a deliberate feature, and `!exprate 1` is how it is undone.
//!
//! # The unit is HUNDREDTHS, and that is the whole point of [`Rate`]
//!
//! `CLAUDE.md` has a section called "the unit, not the arithmetic" listing three bugs this
//! month that were a correct number in the wrong unit. A multiplier is exactly that shape of
//! trap: `2` could mean 2x or 0.02x, and both read as a plausible integer. So the number
//! never travels as a bare integer - it travels as a [`Rate`], which knows it is hundredths,
//! parses the chat argument itself, formats itself for the banner, and does the
//! multiplication. Nothing outside this module divides by 100.
//!
//! # Why a row remembers when its event *ended*
//!
//! The owner, 2026-08-20: *"When either EXP or Meso is set back to 1x again, you should also
//! immediately display a scrolling notice."* A row that has gone back to 1x is
//! indistinguishable from a row that was never touched unless something records the
//! transition, so [`RateRow::ended_at`] does. It is cleared the moment a new event starts on
//! that rate, so an "ended" notice can never outlive the thing it is about.

use std::fmt;

use rusqlite::Connection;

use crate::{Result, Store};

/// A multiplier, stored as **hundredths**: 100 is 1x, 250 is 2.5x.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rate(u32);

/// Why a chat argument was not a multiplier. Worded to be shown to the player as-is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RateError {
    /// Not a number at all, or more than two decimal places.
    NotANumber(String),
    /// A number, but outside [`Rate::MIN`]..=[`Rate::MAX`].
    OutOfRange(String),
}

impl fmt::Display for RateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotANumber(s) => write!(
                f,
                "{s:?} is not a multiplier. Try 2, or 1.5, or 1 to put it back to normal."
            ),
            Self::OutOfRange(s) => write!(
                f,
                "{s} is out of range. A multiplier runs from {} to {} - and 0 is refused \
                 because a rate of zero stops experience entirely, which reads on screen as \
                 the feature being broken rather than as a setting.",
                Rate(Rate::MIN),
                Rate(Rate::MAX)
            ),
        }
    }
}

impl Rate {
    /// 1x - what every rate is until somebody changes it.
    pub const NORMAL: Rate = Rate(100);

    /// 0.01x. Zero is deliberately not representable; see [`RateError::OutOfRange`].
    pub const MIN: u32 = 1;

    /// 100x.
    pub const MAX: u32 = 10_000;

    /// Hundredths in, clamped to the allowed range.
    pub fn from_per_cent(per_cent: u32) -> Rate {
        Rate(per_cent.clamp(Self::MIN, Self::MAX))
    }

    /// The raw hundredths. For storage and tests; do not do arithmetic on it.
    pub fn per_cent(self) -> u32 {
        self.0
    }

    /// Is this 1x?
    pub fn is_normal(self) -> bool {
        self == Self::NORMAL
    }

    /// Parse what the player typed after `!exprate`.
    ///
    /// Accepts `2`, `2x`, `1.5`, `0.5x`, and tolerates the whitespace the chat box leaves.
    /// **No floating point anywhere**: the string is split on the decimal point and the two
    /// halves are combined as integers, so `1.15` is exactly 115 hundredths rather than
    /// whatever the nearest double happens to be.
    pub fn parse(text: &str) -> std::result::Result<Rate, RateError> {
        let raw = text.trim();
        let digits = raw.strip_suffix(['x', 'X']).unwrap_or(raw).trim();
        if digits.is_empty() {
            return Err(RateError::NotANumber(raw.to_string()));
        }
        let (whole, frac) = match digits.split_once('.') {
            Some((w, f)) => (w, f),
            None => (digits, ""),
        };
        // An empty whole part is how ".5" arrives; treat it as zero rather than rejecting it.
        let whole: u32 = if whole.is_empty() {
            0
        } else {
            whole.parse().map_err(|_| RateError::NotANumber(raw.to_string()))?
        };
        if frac.len() > 2 || !frac.chars().all(|c| c.is_ascii_digit()) {
            return Err(RateError::NotANumber(raw.to_string()));
        }
        // "5" is five TENTHS, "05" is five hundredths. Pad on the right, not the left.
        let hundredths: u32 = match frac.len() {
            0 => 0,
            1 => frac.parse::<u32>().unwrap() * 10,
            _ => frac.parse::<u32>().unwrap(),
        };
        let per_cent = whole
            .checked_mul(100)
            .and_then(|w| w.checked_add(hundredths))
            .ok_or_else(|| RateError::OutOfRange(raw.to_string()))?;
        if !(Self::MIN..=Self::MAX).contains(&per_cent) {
            return Err(RateError::OutOfRange(raw.to_string()));
        }
        Ok(Rate(per_cent))
    }

    /// Multiply a reward by this rate.
    ///
    /// Truncates, then **floors at 1 for anything that was not already zero**. Without the
    /// floor, a 0.5x event turns every 1-exp mob into a 0-exp mob and the first thing anyone
    /// would report is "the rate broke experience".
    pub fn apply(self, value: u64) -> u64 {
        if value == 0 {
            return 0;
        }
        let scaled = value.saturating_mul(u64::from(self.0)) / 100;
        scaled.max(1)
    }
}

/// `2`, `1.5`, `0.5` - no trailing zeros, and no `x`, which the caller adds.
impl fmt::Display for Rate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let whole = self.0 / 100;
        match self.0 % 100 {
            0 => write!(f, "{whole}"),
            r if r % 10 == 0 => write!(f, "{whole}.{}", r / 10),
            r => write!(f, "{whole}.{r:02}"),
        }
    }
}

/// Which rate. The string is the primary key in the table, so these three spellings are
/// on-disk format and cannot be renamed casually.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateKind {
    Exp,
    Meso,
    Drop,
}

/// Every rate there is, in the order the banner names them.
pub const ALL_KINDS: [RateKind; 3] = [RateKind::Exp, RateKind::Meso, RateKind::Drop];

impl RateKind {
    /// The database key.
    pub fn key(self) -> &'static str {
        match self {
            Self::Exp => "exp",
            Self::Meso => "meso",
            Self::Drop => "drop",
        }
    }

    /// How the banner names it: *"The Server's **EXP** rate"*.
    pub fn label(self) -> &'static str {
        match self {
            Self::Exp => "EXP",
            Self::Meso => "Meso",
            Self::Drop => "Drop",
        }
    }

    /// The chat command that sets it, without the `!`.
    pub fn command(self) -> &'static str {
        match self {
            Self::Exp => "exprate",
            Self::Meso => "mesorate",
            Self::Drop => "droprate",
        }
    }
}

/// One rate's whole story: what it is, when it was last set, and when its event ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateRow {
    pub rate: Rate,
    /// Unix seconds of the last change to this rate, or 0 if it has never been set.
    pub set_at: i64,
    /// Unix seconds of the moment this rate went from an event **back to 1x**, or 0.
    ///
    /// Cleared when a new event starts, so it never describes a stale event. Without it a
    /// rate that has ended is byte-identical to one that was never touched, and the "event
    /// has ended" notice would have nothing to fire on.
    pub ended_at: i64,
}

impl Default for RateRow {
    fn default() -> Self {
        RateRow { rate: Rate::NORMAL, set_at: 0, ended_at: 0 }
    }
}

/// All three rates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rates {
    pub exp: RateRow,
    pub meso: RateRow,
    pub drop: RateRow,
}

impl Rates {
    /// What a server that has never been touched runs at.
    pub const NORMAL: Rates = Rates {
        exp: RateRow { rate: Rate::NORMAL, set_at: 0, ended_at: 0 },
        meso: RateRow { rate: Rate::NORMAL, set_at: 0, ended_at: 0 },
        drop: RateRow { rate: Rate::NORMAL, set_at: 0, ended_at: 0 },
    };

    /// Are all three rates 1x?
    pub fn all_normal(&self) -> bool {
        ALL_KINDS.iter().all(|k| self.get(*k).is_normal())
    }

    /// One rate by kind.
    pub fn get(&self, kind: RateKind) -> Rate {
        self.row(kind).rate
    }

    /// One row by kind.
    pub fn row(&self, kind: RateKind) -> RateRow {
        match kind {
            RateKind::Exp => self.exp,
            RateKind::Meso => self.meso,
            RateKind::Drop => self.drop,
        }
    }

    fn row_mut(&mut self, kind: RateKind) -> &mut RateRow {
        match kind {
            RateKind::Exp => &mut self.exp,
            RateKind::Meso => &mut self.meso,
            RateKind::Drop => &mut self.drop,
        }
    }

    /// The most recent change to **any** rate - the anchor the banner's cycle counts from.
    ///
    /// One timestamp for the set rather than one each, so that changing any rate restarts the
    /// cycle and every current message appears together from that moment.
    pub fn anchor(&self) -> i64 {
        ALL_KINDS.iter().map(|k| self.row(*k).set_at).max().unwrap_or(0)
    }
}

/// Create the table. Called from the schema on every open, so it must be idempotent.
pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS server_rates (
            kind     TEXT PRIMARY KEY,
            per_cent INTEGER NOT NULL,
            set_at   INTEGER NOT NULL
        );
        "#,
    )?;
    add_ended_at_column(conn)?;
    Ok(())
}

/// `server_rates.ended_at`, added after the fact.
///
/// **Not in the `CREATE TABLE` above, and that is not tidiness.** `CREATE TABLE IF NOT
/// EXISTS` does nothing at all to a table that already exists, and this table shipped
/// yesterday without the column - so a database that has already run the previous build has
/// the table and would never get the column. `ALTER TABLE ADD COLUMN` raises on a duplicate
/// and this runs on every open, hence the `PRAGMA` guard. Same shape and same reason as
/// `db.rs`'s meso column.
fn add_ended_at_column(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("PRAGMA table_info(server_rates)")?;
    let existing: Vec<String> =
        stmt.query_map([], |row| row.get::<_, String>(1))?.collect::<rusqlite::Result<_>>()?;
    if !existing.iter().any(|name| name == "ended_at") {
        conn.execute("ALTER TABLE server_rates ADD COLUMN ended_at INTEGER NOT NULL DEFAULT 0", [])?;
    }
    Ok(())
}

impl Store {
    /// All three rates as they stand.
    ///
    /// A missing row is 1x, so a database from before this table existed reads as a normal
    /// server rather than failing.
    pub fn rates(&self) -> Result<Rates> {
        let conn = self.conn();
        let mut stmt =
            conn.prepare("SELECT kind, per_cent, set_at, ended_at FROM server_rates")?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })?;
        let mut rates = Rates::NORMAL;
        for row in rows {
            let (key, per_cent, set_at, ended_at) = row?;
            let Some(kind) = ALL_KINDS.iter().find(|k| k.key() == key) else {
                continue; // a key nothing writes; ignore rather than fail a login
            };
            *rates.row_mut(*kind) = RateRow {
                rate: Rate::from_per_cent(per_cent.clamp(0, i64::from(u32::MAX)) as u32),
                set_at,
                ended_at,
            };
        }
        Ok(rates)
    }

    /// Set one rate, stamping it with `now` (unix seconds).
    ///
    /// **Going back to 1x records an ending; starting an event clears one.** That is the only
    /// way a later read can tell "this event just finished" from "this was never an event",
    /// and the notice the owner asked for depends on the difference.
    ///
    /// The timestamp is passed in rather than read here so the banner schedule can be tested
    /// against a clock the test controls.
    pub fn set_rate(&self, kind: RateKind, rate: Rate, now: i64) -> Result<()> {
        let was = self.rates()?.get(kind);
        // An event ends when a rate that was NOT 1x becomes 1x. Setting 1x on a rate that was
        // already 1x is not an ending and must not announce one.
        let ended_at = if rate.is_normal() && !was.is_normal() { now } else { 0 };
        self.conn().execute(
            "INSERT INTO server_rates (kind, per_cent, set_at, ended_at) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(kind) DO UPDATE SET per_cent = excluded.per_cent,
                                             set_at   = excluded.set_at,
                                             ended_at = excluded.ended_at",
            rusqlite::params![kind.key(), i64::from(rate.per_cent()), now, ended_at],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_shapes_a_player_would_type() {
        assert_eq!(Rate::parse("2").unwrap().per_cent(), 200);
        assert_eq!(Rate::parse("2x").unwrap().per_cent(), 200);
        assert_eq!(Rate::parse(" 3X ").unwrap().per_cent(), 300);
        assert_eq!(Rate::parse("1.5").unwrap().per_cent(), 150);
        assert_eq!(Rate::parse("0.5").unwrap().per_cent(), 50);
        assert_eq!(Rate::parse(".5").unwrap().per_cent(), 50);
        assert_eq!(Rate::parse("1").unwrap(), Rate::NORMAL);
    }

    #[test]
    fn one_decimal_digit_is_tenths_not_hundredths() {
        // The bug this test exists for: parsing the fraction as an integer gives "5" -> 5
        // hundredths, so 1.5x would quietly become 1.05x and the banner would still say
        // "1.5x". A wrong number that announces itself correctly is the worst kind here.
        assert_eq!(Rate::parse("1.5").unwrap().per_cent(), 150);
        assert_eq!(Rate::parse("1.05").unwrap().per_cent(), 105);
    }

    #[test]
    fn refuses_what_it_cannot_represent() {
        assert!(matches!(Rate::parse("0"), Err(RateError::OutOfRange(_))));
        assert!(matches!(Rate::parse("101"), Err(RateError::OutOfRange(_))));
        assert!(matches!(Rate::parse("fast"), Err(RateError::NotANumber(_))));
        assert!(matches!(Rate::parse(""), Err(RateError::NotANumber(_))));
        assert!(matches!(Rate::parse("1.234"), Err(RateError::NotANumber(_))));
        assert!(matches!(Rate::parse("-2"), Err(RateError::NotANumber(_))));
    }

    #[test]
    fn formats_without_trailing_zeros() {
        assert_eq!(Rate::from_per_cent(200).to_string(), "2");
        assert_eq!(Rate::from_per_cent(150).to_string(), "1.5");
        assert_eq!(Rate::from_per_cent(105).to_string(), "1.05");
        assert_eq!(Rate::from_per_cent(50).to_string(), "0.5");
        assert_eq!(Rate::NORMAL.to_string(), "1");
    }

    #[test]
    fn parse_and_format_round_trip() {
        for per_cent in [1u32, 50, 100, 105, 150, 200, 999, 10_000] {
            let r = Rate::from_per_cent(per_cent);
            assert_eq!(Rate::parse(&r.to_string()).unwrap(), r, "{per_cent}");
        }
    }

    #[test]
    fn applies_as_a_multiplier() {
        assert_eq!(Rate::NORMAL.apply(45), 45);
        assert_eq!(Rate::from_per_cent(200).apply(45), 90);
        assert_eq!(Rate::from_per_cent(150).apply(10), 15);
        assert_eq!(Rate::from_per_cent(50).apply(45), 22); // truncated, not rounded
    }

    #[test]
    fn nothing_worth_something_becomes_worth_nothing() {
        assert_eq!(Rate::from_per_cent(50).apply(1), 1);
        assert_eq!(Rate::from_per_cent(1).apply(1), 1);
        // ...but a mob that was already worth nothing stays worth nothing.
        assert_eq!(Rate::from_per_cent(1000).apply(0), 0);
    }

    #[test]
    fn a_huge_reward_does_not_wrap() {
        // saturating_mul pins at u64::MAX and the divide brings it back down, so the answer
        // is nonsense but it is *bounded* nonsense - it does not wrap round to a small
        // number, which is the failure that would matter.
        assert_eq!(Rate::from_per_cent(10_000).apply(u64::MAX), u64::MAX / 100);
    }

    #[test]
    fn every_kind_has_a_distinct_key_and_command() {
        let keys: Vec<&str> = ALL_KINDS.iter().map(|k| k.key()).collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), keys.len(), "the key is a primary key: {keys:?}");
        for k in ALL_KINDS {
            assert!(k.command().ends_with("rate"), "{}", k.command());
        }
    }

    fn store() -> Store {
        Store::open_in_memory().unwrap()
    }

    #[test]
    fn a_fresh_server_is_normal() {
        let s = store();
        assert_eq!(s.rates().unwrap(), Rates::NORMAL);
        assert!(s.rates().unwrap().all_normal());
    }

    #[test]
    fn setting_and_reading_back_survives_all_three() {
        let s = store();
        s.set_rate(RateKind::Exp, Rate::from_per_cent(200), 1_000).unwrap();
        s.set_rate(RateKind::Meso, Rate::from_per_cent(300), 1_010).unwrap();
        s.set_rate(RateKind::Drop, Rate::from_per_cent(150), 1_020).unwrap();
        let r = s.rates().unwrap();
        assert_eq!(r.exp.rate.per_cent(), 200);
        assert_eq!(r.meso.rate.per_cent(), 300);
        assert_eq!(r.drop.rate.per_cent(), 150);
        assert_eq!(r.anchor(), 1_020, "the anchor is the most recent of the three");
        assert!(!r.all_normal());
    }

    #[test]
    fn going_back_to_normal_records_an_ending() {
        let s = store();
        s.set_rate(RateKind::Exp, Rate::from_per_cent(200), 1_000).unwrap();
        assert_eq!(s.rates().unwrap().exp.ended_at, 0, "a running event has not ended");
        s.set_rate(RateKind::Exp, Rate::NORMAL, 2_000).unwrap();
        assert_eq!(s.rates().unwrap().exp.ended_at, 2_000);
    }

    #[test]
    fn setting_one_x_on_a_rate_that_was_already_one_x_is_not_an_ending() {
        // Otherwise `!exprate 1` on an untouched server announces the end of an event that
        // never happened, which is worse than doing nothing.
        let s = store();
        s.set_rate(RateKind::Exp, Rate::NORMAL, 1_000).unwrap();
        assert_eq!(s.rates().unwrap().exp.ended_at, 0);
    }

    #[test]
    fn starting_a_new_event_clears_the_old_ending() {
        let s = store();
        s.set_rate(RateKind::Exp, Rate::from_per_cent(200), 1_000).unwrap();
        s.set_rate(RateKind::Exp, Rate::NORMAL, 2_000).unwrap();
        s.set_rate(RateKind::Exp, Rate::from_per_cent(300), 3_000).unwrap();
        assert_eq!(
            s.rates().unwrap().exp.ended_at,
            0,
            "an 'ended' notice must never outlive the event it is about"
        );
    }

    /// The migration runs on a database that already has the table without the column - which
    /// is every database that ran yesterday's build.
    #[test]
    fn the_ended_at_column_is_added_to_an_existing_table() {
        let s = store();
        {
            let conn = s.conn();
            conn.execute("DROP TABLE server_rates", []).unwrap();
            conn.execute_batch(
                "CREATE TABLE server_rates (kind TEXT PRIMARY KEY, per_cent INTEGER NOT NULL, \
                 set_at INTEGER NOT NULL);
                 INSERT INTO server_rates VALUES ('exp', 200, 1000);",
            )
            .unwrap();
            create_tables(&conn).unwrap();
        }
        let r = s.rates().unwrap();
        assert_eq!(r.exp.rate.per_cent(), 200, "the old row survives");
        assert_eq!(r.exp.ended_at, 0, "and defaults rather than failing the read");
    }
}
