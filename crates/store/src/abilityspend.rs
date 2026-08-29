//! **How many ability points went into HP and MP**, so `!resetap` can give them back.
//!
//! The owner, 2026-08-29: *"The server should also keep track of how many AP is spent in HP and MP
//! as `!resetap` should also refund those points spend there."*
//!
//! # Why this needs a ledger when the four stats do not
//!
//! `!resetap` refunds STR/DEX/INT/LUK by **arithmetic that cannot be wrong**: a stat's value
//! *is* the record of what was spent on it, so `str - floor` is the refund and the totals
//! balance by construction. Run it twice and the second run refunds zero.
//!
//! HP and MP have no such property. A point spent on HP adds
//! [`net::abilityup::policy::MAX_HP_PER_AP`] to `max_hp` - but `max_hp` also grows on
//! **level-up**, and nothing in the stored value says which part came from which. Dividing
//! `max_hp` by 20 would refund the character's entire level history as ability points, which
//! is the "creating points out of thin air" failure the AP reset was written to avoid.
//!
//! So the count is recorded when it is spent, which is the only moment the two are
//! distinguishable.
//!
//! # It stores POINTS, not the HP they bought
//!
//! `ap_spent_hp` is a number of ability points. The HP to remove on a refund is
//! `points * MAX_HP_PER_AP`, computed at refund time from the same constant that granted it,
//! so the two cannot drift - and if that constant is ever corrected (it is **[I]**, from a
//! different game version, and this project's most expensive mistakes have been a right
//! number in the wrong unit) the stored ledger stays meaningful.
//!
//! # Outside `net::opcode::Character`, on purpose
//!
//! The `characters` table is column-for-field with the protocol's `Character` so that
//! `crate::character` can map one to the other by exhaustive destructure. These two are not
//! protocol fields and must not become them - the same reasoning `mesos` and `exp` carry.
//! They are reached through this module's methods instead.

use rusqlite::{params, Connection, OptionalExtension};

use crate::db::Store;
use crate::error::Result;

/// Ability points a character has put into max HP and max MP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ApSpend {
    pub hp: u32,
    pub mp: u32,
}

impl ApSpend {
    pub fn total(&self) -> u32 {
        self.hp.saturating_add(self.mp)
    }
}

/// Add the two columns to `characters`.
///
/// **`characters` is not a new table**, so this is an `ALTER` behind a `PRAGMA table_info`
/// guard rather than part of a `CREATE TABLE IF NOT EXISTS` - which does nothing at all to a
/// table that already exists, and the owner's database has characters in it. Same shape and same
/// reason as `Store::add_meso_column`.
///
/// **Every existing character starts at 0, and that is a statement of fact rather than a
/// default being imposed**: nothing has ever recorded an HP or MP ability spend, so there is
/// no value here that could be overwritten. It does mean points spent on HP *before* this
/// column existed are not refundable - they are indistinguishable from level-up HP, which is
/// exactly why the column had to exist.
pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    let existing: Vec<String> = {
        let mut stmt = conn.prepare("PRAGMA table_info(characters)")?;
        let rows = stmt
            .query_map([], |row| row.get::<_, String>(1))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        rows
    };
    for column in ["ap_spent_hp", "ap_spent_mp"] {
        if !existing.iter().any(|c| c == column) {
            conn.execute(
                &format!("ALTER TABLE characters ADD COLUMN {column} INTEGER NOT NULL DEFAULT 0"),
                [],
            )?;
        }
    }
    Ok(())
}

impl Store {
    /// What this character has put into HP and MP. `(0, 0)` for a character that has spent
    /// nothing, and for one that does not exist - a reset against a missing character should
    /// refund nothing rather than fail.
    pub fn ap_spend(&self, character_id: u32) -> Result<ApSpend> {
        let row = self
            .conn()
            .query_row(
                "SELECT ap_spent_hp, ap_spent_mp FROM characters WHERE id = ?1",
                params![character_id],
                |r| Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?)),
            )
            .optional()?;
        Ok(match row {
            Some((hp, mp)) => ApSpend {
                hp: u32::try_from(hp).unwrap_or(0),
                mp: u32::try_from(mp).unwrap_or(0),
            },
            None => ApSpend::default(),
        })
    }

    /// Record points that have just gone into HP and/or MP. Adds to what is already there.
    ///
    /// Called from the ability-up handler at the moment the points are spent, which is the
    /// only moment an HP increase can be told apart from a level-up.
    pub fn record_ap_spend(&self, character_id: u32, hp: u32, mp: u32) -> Result<()> {
        if hp == 0 && mp == 0 {
            return Ok(());
        }
        self.conn().execute(
            "UPDATE characters
                SET ap_spent_hp = ap_spent_hp + ?2,
                    ap_spent_mp = ap_spent_mp + ?3
              WHERE id = ?1",
            params![character_id, i64::from(hp), i64::from(mp)],
        )?;
        Ok(())
    }

    /// Zero the counters, and say what they were.
    ///
    /// Returns the previous value so the caller can refund it in the same breath. **Read and
    /// cleared in one transaction**, so a reset cannot read a count, fail to clear it, and
    /// hand the points out twice on the next run.
    pub fn take_ap_spend(&self, character_id: u32) -> Result<ApSpend> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let row = tx
            .query_row(
                "SELECT ap_spent_hp, ap_spent_mp FROM characters WHERE id = ?1",
                params![character_id],
                |r| Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?)),
            )
            .optional()?;
        let Some((hp, mp)) = row else {
            return Ok(ApSpend::default());
        };
        tx.execute(
            "UPDATE characters SET ap_spent_hp = 0, ap_spent_mp = 0 WHERE id = ?1",
            params![character_id],
        )?;
        tx.commit()?;
        Ok(ApSpend {
            hp: u32::try_from(hp).unwrap_or(0),
            mp: u32::try_from(mp).unwrap_or(0),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::FIRST_CHARACTER_ID;

    fn store_with_character() -> (Store, u32) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("player_one", "correct horse battery").unwrap();
        let chr = store
            .create_character(
                account,
                0,
                &net::opcode::Character { name: "Tester".to_string(), ..Default::default() },
            )
            .unwrap();
        assert!(chr.id >= FIRST_CHARACTER_ID);
        (store, chr.id)
    }

    #[test]
    fn a_fresh_character_has_spent_nothing() {
        let (s, id) = store_with_character();
        assert_eq!(s.ap_spend(id).unwrap(), ApSpend { hp: 0, mp: 0 });
    }

    #[test]
    fn spends_accumulate_rather_than_replace() {
        let (s, id) = store_with_character();
        s.record_ap_spend(id, 3, 0).unwrap();
        s.record_ap_spend(id, 2, 5).unwrap();
        assert_eq!(s.ap_spend(id).unwrap(), ApSpend { hp: 5, mp: 5 });
        assert_eq!(s.ap_spend(id).unwrap().total(), 10);
    }

    #[test]
    fn recording_nothing_is_a_no_op_rather_than_a_write() {
        let (s, id) = store_with_character();
        s.record_ap_spend(id, 0, 0).unwrap();
        assert_eq!(s.ap_spend(id).unwrap(), ApSpend::default());
    }

    #[test]
    fn taking_returns_what_was_there_and_leaves_zero() {
        let (s, id) = store_with_character();
        s.record_ap_spend(id, 4, 7).unwrap();
        assert_eq!(s.take_ap_spend(id).unwrap(), ApSpend { hp: 4, mp: 7 });
        assert_eq!(s.ap_spend(id).unwrap(), ApSpend::default());
    }

    /// The regression that matters: a second reset must refund nothing. If `take` returned
    /// the count without clearing it, `!resetap` would hand out the same points every time it
    /// was run - free stats out of a command whose whole job is to be safe to repeat.
    #[test]
    fn taking_twice_refunds_nothing_the_second_time() {
        let (s, id) = store_with_character();
        s.record_ap_spend(id, 4, 7).unwrap();
        assert_eq!(s.take_ap_spend(id).unwrap().total(), 11);
        assert_eq!(s.take_ap_spend(id).unwrap().total(), 0);
    }

    #[test]
    fn a_character_that_does_not_exist_refunds_nothing_rather_than_failing() {
        let (s, _) = store_with_character();
        assert_eq!(s.ap_spend(9999).unwrap(), ApSpend::default());
        assert_eq!(s.take_ap_spend(9999).unwrap(), ApSpend::default());
    }

    #[test]
    fn the_counters_survive_a_reopen() {
        let dir = std::env::temp_dir().join(format!("maplecw-apspend-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("maplecw.db");
        let _ = std::fs::remove_file(&path);

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
            s.record_ap_spend(chr.id, 6, 1).unwrap();
            chr.id
        };
        let s = Store::open(&path).unwrap();
        assert_eq!(s.ap_spend(id).unwrap(), ApSpend { hp: 6, mp: 1 });
    }
}
