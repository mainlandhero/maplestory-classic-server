//! **Which effect item a character has switched on** - Shadow Style and the rest of `501xxxx`.
//!
//! The owner, 2026-09-30: *"The effect should persist and should be saved between logins."* One row
//! per character that has one on; switching it off deletes the row, so "none" has one
//! representation (the `quest_state` rule). A table of its own for the reason `spawnpoint.rs`
//! gives: `characters` is read positionally.

use rusqlite::{params, Connection, OptionalExtension};

use crate::error::Result;
use crate::Store;

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS effect_item (
            character_id INTEGER PRIMARY KEY REFERENCES characters(id) ON DELETE CASCADE,
            item_id      INTEGER NOT NULL
        );
        "#,
    )?;
    Ok(())
}

impl Store {
    /// The effect item `character_id` last switched on, `0` for none.
    pub fn effect_item(&self, character_id: u32) -> Result<u32> {
        let got: Option<i64> = self
            .conn()
            .query_row("SELECT item_id FROM effect_item WHERE character_id = ?1", params![i64::from(character_id)], |r| r.get(0))
            .optional()?;
        Ok(got.and_then(|v| u32::try_from(v).ok()).unwrap_or(0))
    }

    /// Remember `item_id` as switched on; `0` switches it off (the row goes).
    pub fn set_effect_item(&self, character_id: u32, item_id: u32) -> Result<()> {
        let conn = self.conn();
        if item_id == 0 {
            conn.execute("DELETE FROM effect_item WHERE character_id = ?1", params![i64::from(character_id)])?;
        } else {
            conn.execute(
                "INSERT INTO effect_item (character_id, item_id) VALUES (?1, ?2)
                 ON CONFLICT(character_id) DO UPDATE SET item_id = excluded.item_id",
                params![i64::from(character_id), i64::from(item_id)],
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use net::opcode::Character;

    #[test]
    fn the_effect_item_is_kept_and_off_removes_it() {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = store.create_character(account, 0, &Character { name: "Shade".into(), ..Character::default() }).unwrap().id;
        assert_eq!(store.effect_item(chr).unwrap(), 0);
        store.set_effect_item(chr, 5_010_005).unwrap();
        store.set_effect_item(chr, 5_010_005).unwrap();
        assert_eq!(store.effect_item(chr).unwrap(), 5_010_005);
        store.set_effect_item(chr, 0).unwrap();
        assert_eq!(store.effect_item(chr).unwrap(), 0);
    }
}
