//! **The client's options, kept per account on the server.**
//!
//! The owner, 2026-10-02, from player complaints: *"Saving of settings such as audio, HP thresholds,
//! etc on server side per account"* - the HP threshold being the pet's auto-potion one.
//!
//! The client already sends every change (`0x02EB`, `net::clientsettings`) and reads them back
//! from quest ex records at field entry; nothing stored them, so every login started from the
//! defaults. A row is one `(group, key) = value`, exactly as the client numbers them - the names
//! and the quest each one travels in are `net::clientsettings`'s business, not the database's.
//!
//! **The pet's auto-potion keys are per CHARACTER.** The owner, 2026-10-02: *"The pet auto hp seems
//! to be account wide, when ideally this configuration should be saved per character, since
//! other characters may want to use different auto potion setup."* Those rows go in
//! `character_settings`, same shape; which keys they are is `net::clientsettings::is_per_character`.

use rusqlite::{params, Connection};

use crate::error::Result;
use crate::Store;

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS account_settings (
            account_id  INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
            grp         INTEGER NOT NULL,
            key         INTEGER NOT NULL,
            value       INTEGER NOT NULL,
            PRIMARY KEY (account_id, grp, key)
        );
        CREATE TABLE IF NOT EXISTS character_settings (
            character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            grp          INTEGER NOT NULL,
            key          INTEGER NOT NULL,
            value        INTEGER NOT NULL,
            PRIMARY KEY (character_id, grp, key)
        );
        "#,
    )?;
    Ok(())
}

impl Store {
    /// Write each `(key, value)` of one group, replacing what was there for those keys. Keys
    /// not named are left alone: the client sends a single change as a one-entry list.
    pub fn save_client_settings(&self, account_id: i64, group: u32, entries: &[(u32, i32)]) -> Result<()> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        for (key, value) in entries {
            tx.execute(
                "INSERT OR REPLACE INTO account_settings (account_id, grp, key, value) VALUES (?1, ?2, ?3, ?4)",
                params![account_id, i64::from(group), i64::from(*key), i64::from(*value)],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Every stored `(group, key, value)` for the account, ordered by group then key.
    pub fn client_settings(&self, account_id: i64) -> Result<Vec<(u32, u32, i32)>> {
        let conn = self.conn();
        let mut stmt =
            conn.prepare("SELECT grp, key, value FROM account_settings WHERE account_id = ?1 ORDER BY grp, key")?;
        let rows = stmt.query_map(params![account_id], |r| {
            Ok((
                u32::try_from(r.get::<_, i64>(0)?).unwrap_or(0),
                u32::try_from(r.get::<_, i64>(1)?).unwrap_or(0),
                i32::try_from(r.get::<_, i64>(2)?).unwrap_or(0),
            ))
        })?;
        Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
    }

    /// [`Store::save_client_settings`] for one character's own keys.
    pub fn save_character_settings(&self, character_id: u32, group: u32, entries: &[(u32, i32)]) -> Result<()> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        for (key, value) in entries {
            tx.execute(
                "INSERT OR REPLACE INTO character_settings (character_id, grp, key, value) VALUES (?1, ?2, ?3, ?4)",
                params![i64::from(character_id), i64::from(group), i64::from(*key), i64::from(*value)],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Every stored `(group, key, value)` for the character, ordered by group then key.
    pub fn character_settings(&self, character_id: u32) -> Result<Vec<(u32, u32, i32)>> {
        let conn = self.conn();
        let mut stmt = conn
            .prepare("SELECT grp, key, value FROM character_settings WHERE character_id = ?1 ORDER BY grp, key")?;
        let rows = stmt.query_map(params![i64::from(character_id)], |r| {
            Ok((
                u32::try_from(r.get::<_, i64>(0)?).unwrap_or(0),
                u32::try_from(r.get::<_, i64>(1)?).unwrap_or(0),
                i32::try_from(r.get::<_, i64>(2)?).unwrap_or(0),
            ))
        })?;
        Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_are_per_account_and_a_single_change_keeps_the_rest() {
        let store = Store::open_in_memory().unwrap();
        let a = store.create_account("wisp", "correct horse battery").unwrap();
        let b = store.create_account("cobalt", "correct horse battery").unwrap();
        store.save_client_settings(a, 1, &[(0x10, 7), (0x11, 10), (0x0f, 1)]).unwrap();
        store.save_client_settings(a, 0, &[(0x00, 30), (0x05, -1)]).unwrap();
        store.save_client_settings(a, 1, &[(0x10, 3)]).unwrap();
        assert_eq!(
            store.client_settings(a).unwrap(),
            vec![(0, 0, 30), (0, 5, -1), (1, 0x0f, 1), (1, 0x10, 3), (1, 0x11, 10)],
            "the one-key change replaced flHP and left flMP"
        );
        assert!(store.client_settings(b).unwrap().is_empty(), "another account sees none of it");
    }

    #[test]
    fn character_settings_belong_to_one_character_and_go_with_it() {
        let store = Store::open_in_memory().unwrap();
        let a = store.create_account("wisp", "correct horse battery").unwrap();
        let mk = |name: &str| {
            store
                .create_character(a, 0, &net::opcode::Character { name: name.into(), ..Default::default() })
                .unwrap()
                .id
        };
        let (one, two) = (mk("Pebble"), mk("Cobalt"));
        store.save_character_settings(one, 1, &[(0x10, 7), (0x11, 3)]).unwrap();
        store.save_character_settings(one, 1, &[(0x10, 4)]).unwrap();
        store.save_character_settings(two, 1, &[(0x10, 12)]).unwrap();
        assert_eq!(store.character_settings(one).unwrap(), vec![(1, 0x10, 4), (1, 0x11, 3)]);
        assert_eq!(store.character_settings(two).unwrap(), vec![(1, 0x10, 12)]);
        assert!(store.client_settings(a).unwrap().is_empty(), "nothing leaks into the account's rows");
    }
}
