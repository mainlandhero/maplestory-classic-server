//! **Where a character came into somewhere from**, remembered across a relog.
//!
//! The owner, 2026-09-09: *"The server should keep track of which town the user entered from, and
//! then when the user leaves the Free Market, it should return them to the proper portal which
//! they have entered from."*
//!
//! # A table of its own rather than two columns on `characters`
//!
//! `Store::characters_for` reads its row **positionally** - `row.get(19)` through
//! `row.get(24)` for the inventory slots - and carries a comment saying exactly what happens
//! if a column is inserted before them: *"Positional `row.get` indices are the one place where
//! adding a column silently reads the wrong field rather than failing: every slot would have
//! come back holding the experience."* That warning was written after it happened once. Two
//! more columns on that table for a niche feature is not worth re-opening it.
//!
//! # It is persisted, and the reason is that the alternative strands people
//!
//! Session state would be lost on a relog, and a player who logs out inside the Free Market
//! would come back with nothing remembered. The exit portal has no static destination - that
//! is the whole point of it - so "nothing remembered" means "no way out except a GM command".
//! A row costs almost nothing and removes that failure entirely.
//!
//! # `kind`, so this is not a Free Market table
//!
//! The same shape answers "which town did you enter the Free Market from", "which map did you
//! come into this dungeon from", and any other one-way door. Keying on `(character, kind)`
//! means a second such door is a new constant rather than a new table.

use rusqlite::{params, Connection, OptionalExtension};

use crate::error::Result;
use crate::Store;

/// The `kind` for the Free Market's one-way door. See [`crate::Store::set_field_return`].
pub const KIND_FREE_MARKET: &str = "freemarket";

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS field_return (
            character_id INTEGER NOT NULL,
            -- What kind of door this remembers. See KIND_FREE_MARKET.
            kind         TEXT    NOT NULL,
            -- The map to send them back to.
            map_id       INTEGER NOT NULL,
            -- The portal ON THAT MAP to put them at. A name, not an index: an index is a
            -- property of the current dump and would silently move if Map.wz were re-read.
            portal       TEXT    NOT NULL,
            PRIMARY KEY (character_id, kind)
        );
        "#,
    )?;
    Ok(())
}

impl Store {
    /// Remember where `character_id` came in from.
    ///
    /// **Replaces rather than accumulates.** Walking into the Free Market a second time from a
    /// different town must overwrite the first, or the exit would send them to a town they
    /// left long ago - which is a stranger bug than no memory at all, because it looks
    /// deliberate.
    pub fn set_field_return(
        &self,
        character_id: u32,
        kind: &str,
        map_id: u32,
        portal: &str,
    ) -> Result<()> {
        let conn = self.conn();
        create_tables(&conn)?;
        conn.execute(
            "INSERT INTO field_return (character_id, kind, map_id, portal)
                  VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(character_id, kind) DO UPDATE
                    SET map_id = excluded.map_id, portal = excluded.portal",
            params![i64::from(character_id), kind, i64::from(map_id), portal],
        )?;
        Ok(())
    }

    /// Where `character_id` came in from, if anything was remembered.
    pub fn field_return(&self, character_id: u32, kind: &str) -> Result<Option<(u32, String)>> {
        let conn = self.conn();
        create_tables(&conn)?;
        let got = conn
            .query_row(
                "SELECT map_id, portal FROM field_return WHERE character_id = ?1 AND kind = ?2",
                params![i64::from(character_id), kind],
                |r| Ok((r.get::<_, i64>(0)? as u32, r.get::<_, String>(1)?)),
            )
            .optional()?;
        Ok(got)
    }

    /// Forget it. Called once the character has actually been sent back.
    ///
    /// **Returns whether a row was there**, because "we sent them somewhere" and "we sent them
    /// to the fallback" are different outcomes and the caller logs which.
    pub fn clear_field_return(&self, character_id: u32, kind: &str) -> Result<bool> {
        let conn = self.conn();
        create_tables(&conn)?;
        let n = conn.execute(
            "DELETE FROM field_return WHERE character_id = ?1 AND kind = ?2",
            params![i64::from(character_id), kind],
        )?;
        Ok(n > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> Store {
        Store::open_in_memory().unwrap()
    }

    #[test]
    fn a_remembered_door_round_trips_and_can_be_forgotten() {
        let s = store();
        assert_eq!(s.field_return(200, KIND_FREE_MARKET).unwrap(), None, "nothing to start");
        assert!(!s.clear_field_return(200, KIND_FREE_MARKET).unwrap(), "and nothing to clear");

        s.set_field_return(200, KIND_FREE_MARKET, 10_001_040, "market00").unwrap();
        assert_eq!(
            s.field_return(200, KIND_FREE_MARKET).unwrap(),
            Some((10_001_040, "market00".to_string()))
        );
        assert!(s.clear_field_return(200, KIND_FREE_MARKET).unwrap(), "it was there");
        assert_eq!(s.field_return(200, KIND_FREE_MARKET).unwrap(), None, "and now it is not");
    }

    /// **A second entrance replaces the first.** Otherwise the exit sends them to a town they
    /// left long ago, which is worse than no memory because it looks deliberate.
    #[test]
    fn entering_again_from_another_town_overwrites() {
        let s = store();
        s.set_field_return(200, KIND_FREE_MARKET, 10_001_040, "market00").unwrap();
        s.set_field_return(200, KIND_FREE_MARKET, 20_001_010, "market00").unwrap();
        assert_eq!(
            s.field_return(200, KIND_FREE_MARKET).unwrap(),
            Some((20_001_010, "market00".to_string())),
            "El Nath, the one they actually came in from"
        );
    }

    /// Two characters and two kinds do not see each other's rows - the primary key is the
    /// pair, and getting that wrong would send one player to another player's town.
    #[test]
    fn rows_are_per_character_and_per_kind() {
        let s = store();
        s.set_field_return(200, KIND_FREE_MARKET, 10_001_040, "market00").unwrap();
        s.set_field_return(201, KIND_FREE_MARKET, 10_004_000, "market00").unwrap();
        s.set_field_return(200, "somewhere_else", 10_000_000, "in00").unwrap();

        assert_eq!(s.field_return(200, KIND_FREE_MARKET).unwrap().unwrap().0, 10_001_040);
        assert_eq!(s.field_return(201, KIND_FREE_MARKET).unwrap().unwrap().0, 10_004_000);
        assert_eq!(s.field_return(200, "somewhere_else").unwrap().unwrap().0, 10_000_000);
        // And clearing one leaves the others alone.
        s.clear_field_return(200, KIND_FREE_MARKET).unwrap();
        assert_eq!(s.field_return(200, KIND_FREE_MARKET).unwrap(), None);
        assert!(s.field_return(201, KIND_FREE_MARKET).unwrap().is_some());
        assert!(s.field_return(200, "somewhere_else").unwrap().is_some());
    }
}
