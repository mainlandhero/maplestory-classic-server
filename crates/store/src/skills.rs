//! A character's skills, and what level each one is.
//!
//! # Why this is its own table
//!
//! One row per skill the character has ever raised, exactly like `equipment` and for the
//! same reason: the set is sparse and grows with the game. A column per skill on
//! `characters` would be several thousand columns, most of them zero.
//!
//! The row maps 1:1 onto `net::skills::Skill`, so nothing has to be translated on the way
//! to the wire - the same decision `crates/store/src/character.rs` explains at length for
//! `net::opcode::Character`.
//!
//! # A skill at level 0 is not stored
//!
//! The client is told a skill's level, and "not in the list" and "in the list at level 0"
//! are the same thing to it. Keeping level-0 rows would mean sending them, which is a longer
//! packet saying nothing.

use rusqlite::Connection;

use crate::{Result, Store};

/// Create the table. Called from the schema on every open, so it must be idempotent.
pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS character_skills (
            character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            skill_id     INTEGER NOT NULL,
            level        INTEGER NOT NULL,
            master_level INTEGER NOT NULL DEFAULT 0,
            expires_at   INTEGER NOT NULL DEFAULT 0,
            PRIMARY KEY (character_id, skill_id)
        );
        "#,
    )?;
    Ok(())
}

impl Store {
    /// Every skill this character has, lowest id first.
    ///
    /// Ordered so a record built twice is byte-identical - a `HashMap` walk is not, and a
    /// packet that differs run to run is one nobody can diff against a capture.
    pub fn skills(&self, character_id: u32) -> Result<Vec<net::skills::Skill>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT skill_id, level, master_level, expires_at
               FROM character_skills
              WHERE character_id = ?1
              ORDER BY skill_id",
        )?;
        let rows = stmt.query_map([i64::from(character_id)], |row| {
            Ok(net::skills::Skill {
                id: row.get::<_, i64>(0)? as u32,
                level: row.get::<_, i64>(1)? as u32,
                master_level: row.get::<_, i64>(2)? as u32,
                expires_at: row.get::<_, i64>(3)? as u64,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// One skill's level, or 0 if the character has never raised it.
    pub fn skill_level(&self, character_id: u32, skill_id: u32) -> Result<u32> {
        Ok(skill_level_row(&self.conn(), character_id, skill_id).unwrap_or(0))
    }

    /// Set a skill's level. Level 0 **deletes** the row - see the module docs.
    pub fn set_skill_level(&self, character_id: u32, skill_id: u32, level: u32) -> Result<()> {
        set_skill_level_row(&self.conn(), character_id, skill_id, level)
    }
}

/// Read one skill's level on a caller-supplied connection.
///
/// Extracted from [`Store::skill_level`] so a spend can read the level and charge the pool
/// inside **one** transaction - see [`Store::spend_and_raise_skill`], which lives in
/// [`crate::skillpoints`]. Same query, same "no row means 0" rule.
pub(crate) fn skill_level_row(conn: &Connection, character_id: u32, skill_id: u32) -> Result<u32> {
    let level: Option<i64> = conn
        .query_row(
            "SELECT level FROM character_skills WHERE character_id = ?1 AND skill_id = ?2",
            rusqlite::params![i64::from(character_id), i64::from(skill_id)],
            |row| row.get(0),
        )
        .ok();
    Ok(level.unwrap_or(0) as u32)
}

/// Write one skill's level on a caller-supplied connection. Level 0 deletes the row.
///
/// **The only copy of this statement pair.** [`Store::set_skill_level`] and the skill-up in
/// [`crate::skillpoints`] both go through here, because two copies of one rule is how one of
/// them ends up wrong - and the rule that a level of 0 is a *deletion* rather than a stored
/// zero is exactly the kind that gets missed in the second copy.
pub(crate) fn set_skill_level_row(
    conn: &Connection,
    character_id: u32,
    skill_id: u32,
    level: u32,
) -> Result<()> {
    if level == 0 {
        conn.execute(
            "DELETE FROM character_skills WHERE character_id = ?1 AND skill_id = ?2",
            rusqlite::params![i64::from(character_id), i64::from(skill_id)],
        )?;
        return Ok(());
    }
    conn.execute(
        "INSERT INTO character_skills (character_id, skill_id, level)
              VALUES (?1, ?2, ?3)
         ON CONFLICT(character_id, skill_id) DO UPDATE SET level = ?3",
        rusqlite::params![i64::from(character_id), i64::from(skill_id), i64::from(level)],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store_with_character() -> (Store, u32) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Skiller".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        (store, id)
    }

    #[test]
    fn a_raised_skill_survives_a_reload() {
        let (s, id) = store_with_character();
        assert_eq!(s.skill_level(id, 1000).unwrap(), 0, "nothing raised yet");
        assert!(s.skills(id).unwrap().is_empty());

        s.set_skill_level(id, 1000, 1).unwrap();
        assert_eq!(s.skill_level(id, 1000).unwrap(), 1);

        s.set_skill_level(id, 1000, 2).unwrap();
        assert_eq!(s.skill_level(id, 1000).unwrap(), 2, "raising it again updates the row");

        let all = s.skills(id).unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!((all[0].id, all[0].level), (1000, 2));
    }

    /// Level 0 removes the row: to the client, absent and level 0 are the same thing, and
    /// keeping it would mean sending a longer packet that says nothing.
    #[test]
    fn setting_a_skill_to_zero_removes_it() {
        let (s, id) = store_with_character();
        s.set_skill_level(id, 1000, 3).unwrap();
        s.set_skill_level(id, 1000, 0).unwrap();
        assert!(s.skills(id).unwrap().is_empty());
        assert_eq!(s.skill_level(id, 1000).unwrap(), 0);
    }

    /// Ordered by id, so a record built twice is byte-identical.
    #[test]
    fn skills_come_back_in_a_stable_order() {
        let (s, id) = store_with_character();
        for skill in [1002u32, 1000, 1001] {
            s.set_skill_level(id, skill, 1).unwrap();
        }
        let ids: Vec<u32> = s.skills(id).unwrap().iter().map(|k| k.id).collect();
        assert_eq!(ids, vec![1000, 1001, 1002]);
    }

    /// Two characters do not share skills.
    #[test]
    fn skills_are_per_character() {
        let (s, a) = store_with_character();
        let chr = net::opcode::Character { name: "Other".to_string(), ..Default::default() };
        let b = s.create_character(1, 0, &chr).unwrap().id;
        s.set_skill_level(a, 1000, 5).unwrap();
        assert_eq!(s.skill_level(b, 1000).unwrap(), 0);
    }
}
