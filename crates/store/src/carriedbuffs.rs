//! **The buffs a character is holding as they change channel.**
//!
//! A player, relayed by the owner 2026-10-01: *"Once buff expires, it does not go away."* A
//! buff's icon comes down only when the server sends `0x007E` (`world::session::buff`), and
//! the server that knew about the buff was the channel the player LEFT: each channel is its own
//! process and a new connection starts with no buffs at all, so nothing was ever going to
//! expire it. The leaving channel writes what is held here; the arriving one takes it back.
//!
//! Rows are wall-clock and short-lived: a take ignores anything saved more than
//! [`CARRY_TTL_MS`] ago (a channel change that never arrived) and deletes what it read, so a
//! row is used at most once.

use rusqlite::{params, Connection};

use crate::error::Result;
use crate::Store;

/// How long a saved set waits for the arriving channel - the migration's own lifetime
/// (`migration::MIGRATION_TTL_SECS`).
pub const CARRY_TTL_MS: i64 = crate::migration::MIGRATION_TTL_SECS * 1_000;

/// `expires_unix_ms` for a buff with no expiry - a toggle such as Magic Guard.
pub const NEVER: i64 = i64::MAX;

/// One held temporary stat, in wall-clock terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CarriedBuff {
    /// The CTS bit.
    pub bit: u32,
    /// The skill or item that granted it.
    pub source: u32,
    /// What the client draws it as: the skill id, or the item's negative reason.
    pub reason: u32,
    pub value: i16,
    /// Unix milliseconds, or [`NEVER`].
    pub expires_unix_ms: i64,
}

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS carried_buffs (
            character_id    INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            bit             INTEGER NOT NULL,
            source          INTEGER NOT NULL,
            reason          INTEGER NOT NULL,
            value           INTEGER NOT NULL,
            expires_unix_ms INTEGER NOT NULL,
            saved_unix_ms   INTEGER NOT NULL,
            PRIMARY KEY (character_id, bit)
        );
        "#,
    )?;
    Ok(())
}

impl Store {
    /// Replace whatever is saved for `character_id` with `buffs`, stamped `now_unix_ms`.
    pub fn save_carried_buffs(&self, character_id: u32, buffs: &[CarriedBuff], now_unix_ms: i64) -> Result<()> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM carried_buffs WHERE character_id = ?1", params![i64::from(character_id)])?;
        for b in buffs {
            tx.execute(
                "INSERT OR REPLACE INTO carried_buffs
                 (character_id, bit, source, reason, value, expires_unix_ms, saved_unix_ms)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    i64::from(character_id),
                    i64::from(b.bit),
                    i64::from(b.source),
                    i64::from(b.reason),
                    i64::from(b.value),
                    b.expires_unix_ms,
                    now_unix_ms
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Take back what was saved for `character_id`: the rows saved within [`CARRY_TTL_MS`]
    /// whose buff has not run out by `now_unix_ms`. Every row for the character is deleted,
    /// read or not.
    pub fn take_carried_buffs(&self, character_id: u32, now_unix_ms: i64) -> Result<Vec<CarriedBuff>> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let got = {
            let mut stmt = tx.prepare(
                "SELECT bit, source, reason, value, expires_unix_ms FROM carried_buffs
                 WHERE character_id = ?1 AND saved_unix_ms >= ?2 AND expires_unix_ms > ?3
                 ORDER BY bit",
            )?;
            let rows = stmt.query_map(
                params![i64::from(character_id), now_unix_ms - CARRY_TTL_MS, now_unix_ms],
                |r| {
                    Ok(CarriedBuff {
                        bit: u32::try_from(r.get::<_, i64>(0)?).unwrap_or(0),
                        source: u32::try_from(r.get::<_, i64>(1)?).unwrap_or(0),
                        reason: u32::try_from(r.get::<_, i64>(2)?).unwrap_or(0),
                        value: i16::try_from(r.get::<_, i64>(3)?).unwrap_or(0),
                        expires_unix_ms: r.get(4)?,
                    })
                },
            )?;
            rows.collect::<std::result::Result<Vec<_>, _>>()?
        };
        tx.execute("DELETE FROM carried_buffs WHERE character_id = ?1", params![i64::from(character_id)])?;
        tx.commit()?;
        Ok(got)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use net::opcode::Character;

    fn buff(bit: u32, expires_unix_ms: i64) -> CarriedBuff {
        CarriedBuff { bit, source: 2_001_002, reason: 2_001_002, value: 10, expires_unix_ms }
    }

    #[test]
    fn a_carried_set_is_taken_once_without_the_expired_or_the_stale() {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = store.create_character(account, 0, &Character { name: "Shade".into(), ..Character::default() }).unwrap().id;

        store.save_carried_buffs(chr, &[buff(92, 50_000), buff(88, 9_000), buff(4, NEVER)], 1_000).unwrap();
        let got = store.take_carried_buffs(chr, 10_000).unwrap();
        assert_eq!(got, vec![buff(4, NEVER), buff(92, 50_000)], "the one that ran out at 9 s stays behind");
        assert!(store.take_carried_buffs(chr, 10_000).unwrap().is_empty(), "taken once");

        store.save_carried_buffs(chr, &[buff(92, 500_000)], 1_000).unwrap();
        assert!(store.take_carried_buffs(chr, 1_000 + CARRY_TTL_MS + 1).unwrap().is_empty(), "a change that never arrived");
        assert!(store.take_carried_buffs(chr, 2_000).unwrap().is_empty(), "and the stale take deleted it");

        store.save_carried_buffs(chr, &[buff(92, 500_000), buff(88, 500_000)], 1_000).unwrap();
        store.save_carried_buffs(chr, &[buff(88, 500_000)], 2_000).unwrap();
        assert_eq!(store.take_carried_buffs(chr, 3_000).unwrap(), vec![buff(88, 500_000)], "a save replaces, not adds");
    }
}
