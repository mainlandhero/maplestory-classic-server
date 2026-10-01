//! **Which spawn point a character comes back in at.**
//!
//! The owner, 2026-09-26: *"can the server now spawn the player to the closest spawn point where they
//! last were before they disconnect, change channel, go into cash shop, or otherwise would cause
//! them to load in to the map again? If the player does log off, the server should store which
//! spawn point they should be spawned at when they come back in addition to what map they are
//! on."*
//!
//! Until then the arrival portal was per-connection state only (`net::opcode::Character::portal`,
//! "not persisted"), so every login put the character at portal 0 - on some maps thousands of
//! pixels from where they left.
//!
//! # A table of its own, like `fieldreturn`
//!
//! `Store::characters_for` reads `characters` **positionally**, and a column inserted in the
//! wrong place reads the wrong field silently (`fieldreturn`'s module docs have the history). One
//! row per character, keyed by id: the map it was recorded on, and the portal **index** on that
//! map.
//!
//! # The index, not the name
//!
//! `fieldreturn` stores a portal NAME because a name survives a re-dump of `Map.wz`. That does
//! not work here: most spawn points on a map share the name `sp` (map 10002070 has twelve), so
//! a name would not say which one. The index is what the `SetField` carries anyway.
//!
//! # The map is stored with it, and it is checked
//!
//! A character's map can change without this row changing - a login onto a party-quest stage
//! is moved to the Exit (`keep_out_of_party_quest_on_login`), a GM edit, an older build. A
//! portal index is only meaningful on the map it was recorded on, so the reader applies it only
//! when the maps agree; otherwise the character arrives at the map's default spawn as before.

use rusqlite::{params, Connection, OptionalExtension};

use crate::error::Result;
use crate::Store;

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS spawn_point (
            character_id INTEGER PRIMARY KEY,
            -- The map the portal below belongs to. Applied only when it is still the
            -- character's map.
            map_id       INTEGER NOT NULL,
            -- The portal INDEX on that map - what the SetField carries. Not a name: most
            -- spawn points on a map are all called `sp`.
            portal       INTEGER NOT NULL
        );
        "#,
    )?;
    Ok(())
}

impl Store {
    /// Remember that `character_id` should come back in at `portal` on `map_id`. Overwrites.
    pub fn set_spawn_point(&self, character_id: u32, map_id: u32, portal: u8) -> Result<()> {
        let conn = self.conn();
        conn.execute(
            "INSERT INTO spawn_point (character_id, map_id, portal) VALUES (?1, ?2, ?3)
             ON CONFLICT(character_id) DO UPDATE SET map_id = excluded.map_id, portal = excluded.portal",
            params![i64::from(character_id), i64::from(map_id), i64::from(portal)],
        )?;
        Ok(())
    }

    /// Where `character_id` was last recorded: `(map, portal index)`, or `None`.
    pub fn spawn_point(&self, character_id: u32) -> Result<Option<(u32, u8)>> {
        let conn = self.conn();
        let got = conn
            .query_row(
                "SELECT map_id, portal FROM spawn_point WHERE character_id = ?1",
                params![i64::from(character_id)],
                |r| Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?)),
            )
            .optional()?;
        Ok(got.and_then(|(map, portal)| Some((u32::try_from(map).ok()?, u8::try_from(portal).ok()?))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_spawn_point_is_remembered_overwritten_and_per_character() {
        let store = Store::open_in_memory().unwrap();
        assert_eq!(store.spawn_point(200).unwrap(), None, "nothing recorded yet");
        store.set_spawn_point(200, 10002070, 32).unwrap();
        store.set_spawn_point(201, 104040000, 3).unwrap();
        assert_eq!(store.spawn_point(200).unwrap(), Some((10002070, 32)));
        store.set_spawn_point(200, 10002070, 9).unwrap();
        assert_eq!(store.spawn_point(200).unwrap(), Some((10002070, 9)), "the latest wins");
        assert_eq!(store.spawn_point(201).unwrap(), Some((104040000, 3)), "and nobody else's moved");
    }

    /// **A database from before this table gains it on open**, which is the live server's case:
    /// it runs its own `maplecw.db`, not the repo's.
    #[test]
    fn an_older_database_gains_the_table_on_open() {
        let dir = std::env::temp_dir().join(format!("maplecw-spawnpoint-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("old.db");
        let _ = std::fs::remove_file(&path);
        {
            let store = Store::open(&path).unwrap();
            store.conn().execute("DROP TABLE spawn_point", []).unwrap();
        }
        let store = Store::open(&path).unwrap();
        store.set_spawn_point(200, 1, 2).unwrap();
        assert_eq!(store.spawn_point(200).unwrap(), Some((1, 2)));
        drop(store);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
