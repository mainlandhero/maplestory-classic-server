//! **A character's citizenship, per town.**
//!
//! The owner, 2026-09-27: *"Great, make the implementation"* - of the citizenship system
//! `research/citizenship-2026-09-27.md` decoded. The client keeps it as quest 510000's ex
//! record, `st<town>=..;gr<town>=..;ct<town>=..`; this is the server's copy, one row per town the
//! character has ever signed with, and `world::citizenship` turns it into that string.
//!
//! # Rows, not a string
//!
//! Storing the ex string itself would make every reader a parser and every writer a
//! string-splicer. The string is a wire format; the row is the state.
//!
//! # `state` is the client's own number
//!
//! `1` is the active citizenship - every lock in the client tests `st == 1` **[L]**. `2` is
//! ours for "frozen": a town the character transferred away from or renounced, which keeps its
//! grade and contribution for a reactivation. What the client would do with a `2` is nothing
//! (it is not `1`), which is the point. **[I]** for the number; the client never distinguishes.
//!
//! # `certified_grade`
//!
//! The grade the Town Clerk last handed a certificate for (the `0x46` contract window). A
//! grade-up happens at a quest turn-in, in the middle of an NPC conversation, and a second
//! script window then is a fight for the same UI - so the certificate waits for the next talk
//! to the clerk, and this column is how the clerk knows one is owed.
//!
//! # `board_pick` - what the Community Board offers this character this period
//!
//! The owner, 2026-10-01: *"Weeklies should only be allowed once per character, and the highest
//! level weekly at time of weekly reset is allowed"*, and of the dailies *"once the user
//! completes a daily, even if they advance in citizen rank, they should not be offered a new
//! daily quest"*. A posting computed from the grade each time moved with every grade-up, so one
//! week handed out three donations. The posting is now **chosen once per period and kept**: one
//! row per character and board group, overwritten when the period turns.

use rusqlite::{params, Connection};

use crate::error::Result;
use crate::Store;

/// The client's "active citizenship" state. **[L]**
pub const STATE_ACTIVE: u8 = 1;
/// Ours: signed once, not active now - transferred away from or renounced. Keeps its grade.
pub const STATE_FROZEN: u8 = 2;

/// One town's standing for one character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TownStanding {
    /// 1 Henesys, 2 Kerning City.
    pub town: u8,
    /// [`STATE_ACTIVE`] or [`STATE_FROZEN`].
    pub state: u8,
    /// 1..=10.
    pub grade: u8,
    /// The running total.
    pub contribution: u32,
    /// The grade the last certificate was handed out for.
    pub certified_grade: u8,
}

impl TownStanding {
    pub fn is_active(&self) -> bool {
        self.state == STATE_ACTIVE
    }
}

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS citizenship (
            character_id    INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            town            INTEGER NOT NULL,
            -- 1 active (the client's value), 2 frozen (ours).
            state           INTEGER NOT NULL,
            grade           INTEGER NOT NULL,
            contribution    INTEGER NOT NULL DEFAULT 0,
            certified_grade INTEGER NOT NULL DEFAULT 1,
            PRIMARY KEY (character_id, town)
        );
        -- One Community Board group's posting for one character, frozen for the period
        -- (UTC day, or Monday-started week) it was chosen in.
        CREATE TABLE IF NOT EXISTS board_pick (
            character_id    INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            record_quest    INTEGER NOT NULL,
            period          INTEGER NOT NULL,
            -- The posted quest ids, '|'-separated as the client's record has them.
            quests          TEXT NOT NULL,
            PRIMARY KEY (character_id, record_quest)
        );
        "#,
    )?;
    // `honor_earring` (2026-09-29): whether this town's Citizen of Honor earring has been
    // handed over. ALTERed on with the usual `PRAGMA table_info` guard, because the table
    // already exists on a database opened before it.
    let mut stmt = conn.prepare("PRAGMA table_info(citizenship)")?;
    let has = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<std::result::Result<Vec<_>, _>>()?
        .iter()
        .any(|name| name == "honor_earring");
    drop(stmt);
    if !has {
        conn.execute("ALTER TABLE citizenship ADD COLUMN honor_earring INTEGER NOT NULL DEFAULT 0", [])?;
    }
    Ok(())
}

impl Store {
    /// Every town `character_id` has signed with, town order.
    pub fn citizenship(&self, character_id: u32) -> Result<Vec<TownStanding>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT town, state, grade, contribution, certified_grade FROM citizenship
              WHERE character_id = ?1 ORDER BY town",
        )?;
        let rows = stmt
            .query_map(params![i64::from(character_id)], |r| {
                Ok(TownStanding {
                    town: r.get::<_, i64>(0)?.clamp(0, 255) as u8,
                    state: r.get::<_, i64>(1)?.clamp(0, 255) as u8,
                    grade: r.get::<_, i64>(2)?.clamp(0, 255) as u8,
                    contribution: r.get::<_, i64>(3)?.clamp(0, i64::from(u32::MAX)) as u32,
                    certified_grade: r.get::<_, i64>(4)?.clamp(0, 255) as u8,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Write every town in `towns` for `character_id`, in one transaction - a transfer
    /// freezes one town and activates another, and a half-applied transfer would leave two
    /// active citizenships, which the whole system is built on never having.
    pub fn set_citizenship(&self, character_id: u32, towns: &[TownStanding]) -> Result<()> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        for t in towns {
            tx.execute(
                "INSERT INTO citizenship (character_id, town, state, grade, contribution, certified_grade)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                 ON CONFLICT(character_id, town) DO UPDATE SET
                    state = excluded.state, grade = excluded.grade,
                    contribution = excluded.contribution, certified_grade = excluded.certified_grade",
                params![
                    i64::from(character_id),
                    i64::from(t.town),
                    i64::from(t.state),
                    i64::from(t.grade),
                    i64::from(t.contribution),
                    i64::from(t.certified_grade),
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Whether `town`'s Citizen of Honor earring has already been handed to `character_id`.
    /// `false` for a town never signed with.
    pub fn honor_earring_given(&self, character_id: u32, town: u8) -> Result<bool> {
        let given: Option<i64> = self
            .conn()
            .query_row(
                "SELECT honor_earring FROM citizenship WHERE character_id = ?1 AND town = ?2",
                params![i64::from(character_id), i64::from(town)],
                |r| r.get(0),
            )
            .ok();
        Ok(given.unwrap_or(0) != 0)
    }

    /// Record that `town`'s earring has been handed over. `true` only on the transition - the
    /// test-and-set that makes a second hand-over impossible even if two paths race for it.
    pub fn mark_honor_earring(&self, character_id: u32, town: u8) -> Result<bool> {
        let changed = self.conn().execute(
            "UPDATE citizenship SET honor_earring = 1
              WHERE character_id = ?1 AND town = ?2 AND honor_earring = 0",
            params![i64::from(character_id), i64::from(town)],
        )?;
        Ok(changed > 0)
    }

    /// The board posting kept for `record_quest` (the group's `qrID`): `(period, quest ids)`.
    pub fn board_pick(&self, character_id: u32, record_quest: u32) -> Result<Option<(i64, Vec<u32>)>> {
        let row: Option<(i64, String)> = self
            .conn()
            .query_row(
                "SELECT period, quests FROM board_pick WHERE character_id = ?1 AND record_quest = ?2",
                params![i64::from(character_id), i64::from(record_quest)],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .ok();
        Ok(row.map(|(period, quests)| (period, quests.split('|').filter_map(|q| q.parse().ok()).collect())))
    }

    /// Keep `quests` as `record_quest`'s posting for `period`, replacing any earlier period's.
    pub fn set_board_pick(&self, character_id: u32, record_quest: u32, period: i64, quests: &[u32]) -> Result<()> {
        let list = quests.iter().map(u32::to_string).collect::<Vec<_>>().join("|");
        self.conn().execute(
            "INSERT INTO board_pick (character_id, record_quest, period, quests) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(character_id, record_quest) DO UPDATE SET period = excluded.period, quests = excluded.quests",
            params![i64::from(character_id), i64::from(record_quest), period, list],
        )?;
        Ok(())
    }

    /// Add `amount` to `town`'s contribution **only if that citizenship is active**, and
    /// return the standing after. `None` - nothing written - for a town the character is not
    /// an active citizen of: contribution banks only where you live (`Check.citizenshipTown`
    /// gates the quests that pay it, and this is that rule where it is enforced).
    pub fn add_contribution(&self, character_id: u32, town: u8, amount: u32) -> Result<Option<TownStanding>> {
        let changed = self.conn().execute(
            "UPDATE citizenship SET contribution = MIN(contribution + ?3, 4294967295)
              WHERE character_id = ?1 AND town = ?2 AND state = ?4",
            params![i64::from(character_id), i64::from(town), i64::from(amount), i64::from(STATE_ACTIVE)],
        )?;
        if changed == 0 {
            return Ok(None);
        }
        Ok(self.citizenship(character_id)?.into_iter().find(|t| t.town == town))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use net::opcode::Character;

    fn store_with_character() -> (Store, u32) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = store
            .create_character(account, 0, &Character { name: "Citizen".into(), ..Character::default() })
            .unwrap();
        (store, chr.id)
    }

    fn standing(town: u8, state: u8, grade: u8, contribution: u32) -> TownStanding {
        TownStanding { town, state, grade, contribution, certified_grade: grade }
    }

    #[test]
    fn a_character_starts_with_no_citizenship_and_keeps_what_is_written() {
        let (store, chr) = store_with_character();
        assert!(store.citizenship(chr).unwrap().is_empty());
        store.set_citizenship(chr, &[standing(1, STATE_ACTIVE, 1, 0)]).unwrap();
        store.set_citizenship(chr, &[standing(1, STATE_FROZEN, 3, 2100), standing(2, STATE_ACTIVE, 1, 0)]).unwrap();
        assert_eq!(
            store.citizenship(chr).unwrap(),
            vec![standing(1, STATE_FROZEN, 3, 2100), standing(2, STATE_ACTIVE, 1, 0)]
        );
    }

    /// The earring flag: off until marked, marked once, per town - and the column is added
    /// to a table created before it existed (the guarded ALTER).
    #[test]
    fn the_honor_earring_is_marked_once_per_town() {
        let (store, chr) = store_with_character();
        store.set_citizenship(chr, &[standing(1, STATE_ACTIVE, 10, 10_000), standing(2, STATE_FROZEN, 10, 10_000)]).unwrap();
        assert!(!store.honor_earring_given(chr, 1).unwrap());
        assert!(store.mark_honor_earring(chr, 1).unwrap());
        assert!(!store.mark_honor_earring(chr, 1).unwrap(), "once");
        assert!(store.honor_earring_given(chr, 1).unwrap());
        assert!(!store.honor_earring_given(chr, 2).unwrap(), "per town");
        store.set_citizenship(chr, &[standing(1, STATE_FROZEN, 10, 10_000)]).unwrap();
        assert!(store.honor_earring_given(chr, 1).unwrap(), "a later write of the standing keeps it");

        let old = Connection::open_in_memory().unwrap();
        old.execute_batch("CREATE TABLE citizenship (character_id INTEGER, town INTEGER, state INTEGER, grade INTEGER, contribution INTEGER, certified_grade INTEGER, PRIMARY KEY (character_id, town));").unwrap();
        create_tables(&old).unwrap();
        create_tables(&old).unwrap();
        let n: i64 = old.query_row("SELECT COUNT(*) FROM pragma_table_info('citizenship') WHERE name = 'honor_earring'", [], |r| r.get(0)).unwrap();
        assert_eq!(n, 1, "added once to the old table, and twice is harmless");
    }

    /// A board pick is kept per group and replaced, not added to, when the period turns.
    #[test]
    fn a_board_pick_is_kept_per_group_and_replaced_by_the_next_period() {
        let (store, chr) = store_with_character();
        assert_eq!(store.board_pick(chr, 510_002).unwrap(), None);
        store.set_board_pick(chr, 510_002, 2_960, &[506_025]).unwrap();
        store.set_board_pick(chr, 510_001, 20_730, &[506_005, 506_006]).unwrap();
        assert_eq!(store.board_pick(chr, 510_002).unwrap(), Some((2_960, vec![506_025])));
        assert_eq!(store.board_pick(chr, 510_001).unwrap(), Some((20_730, vec![506_005, 506_006])));
        store.set_board_pick(chr, 510_002, 2_961, &[]).unwrap();
        assert_eq!(store.board_pick(chr, 510_002).unwrap(), Some((2_961, vec![])), "an empty posting is a posting");
    }

    /// Contribution banks only in the active town; a frozen one is untouched and says so.
    #[test]
    fn contribution_is_added_only_where_the_citizenship_is_active() {
        let (store, chr) = store_with_character();
        assert_eq!(store.add_contribution(chr, 1, 100).unwrap(), None, "no citizenship at all");
        store.set_citizenship(chr, &[standing(1, STATE_FROZEN, 2, 1000), standing(2, STATE_ACTIVE, 1, 50)]).unwrap();
        assert_eq!(store.add_contribution(chr, 1, 100).unwrap(), None, "frozen");
        assert_eq!(store.citizenship(chr).unwrap()[0].contribution, 1000);
        let after = store.add_contribution(chr, 2, 150).unwrap().unwrap();
        assert_eq!(after.contribution, 200);
    }
}
