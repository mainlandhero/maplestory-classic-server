//! Per-pet state that outlives a session: its name, the skills it has learned, and whether it
//! was out when the player last left.
//!
//! The owner, 2026-09-15: *"Pets that were spawned from before does not survive a re-login, pets
//! that were previously summoned by user should keep their state."* And in the same breath:
//! a rename with a Pet Name Tag, and Auto HP / Auto MP / Auto Move skill items, none of which
//! had anywhere to land.
//!
//! # Keyed by `(character, pet item id)`, not by the item's slot
//!
//! An item has no stable row id - `inventory`'s primary key is `(character_id, inv_type,
//! slot)` - so a pet's state cannot hang off its row: the first move in the Cash tab would
//! orphan it. The item **id** is what the client itself pairs by (`petLockerSN` is derived
//! from it, `net::pet::pet_serial`), so it is the key here too. The one consequence: two of
//! the same pet on one character share a name and a skill set. This client accepts a single
//! summoned pet, and nobody has bought two Huskies; noted rather than solved.
//!
//! # `active` is the pet the next login re-summons
//!
//! Exactly one row per character may be active - `set_active` clears the others in the same
//! statement pair - and a put-away clears it. `Session::restore_active_pet` reads it once at
//! claim time and re-summons on the first field entry, which is the reference server's
//! `initPets` behaviour: what the row says is out, is out.
//!
//! # Nothing here authenticates
//!
//! The channel socket carries no credentials. A rename or a skill arrives on the say-so of
//! whoever holds the connection, like every other packet on it.

use rusqlite::{Connection, OptionalExtension};

use crate::db::Store;
use crate::error::Result;

/// What is stored about one pet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PetState {
    /// The player's name for it, or `None` for the item's own name.
    pub name: Option<String>,
    /// The learned-skill mask, in the client's own bit order (`net::bag::PET_SKILL_*`).
    pub skills: u16,
    /// Whether it was summoned when last seen.
    pub active: bool,
}

/// The mask a pet has before it learns anything - Item Pouch. Mirrors
/// `net::bag::PET_SKILLS_LEARNED_AT_START`; `store` does not depend on that constant so the
/// crate boundary stays one-way, and a test in `net` pins the two together.
pub const PET_SKILLS_AT_START: u16 = 1;

pub(crate) fn ensure_table(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS character_pets (
            character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            item_id      INTEGER NOT NULL,
            name         TEXT,
            skills       INTEGER NOT NULL DEFAULT 1,
            active       INTEGER NOT NULL DEFAULT 0,
            PRIMARY KEY (character_id, item_id)
        );",
    )?;
    Ok(())
}

impl Store {
    /// This pet's stored state, or the defaults when it has never been touched.
    pub fn pet_state(&self, character_id: u32, item_id: u32) -> Result<PetState> {
        let conn = self.conn();
        ensure_table(&conn)?;
        Ok(conn
            .query_row(
                "SELECT name, skills, active FROM character_pets
                 WHERE character_id = ?1 AND item_id = ?2",
                rusqlite::params![character_id, item_id],
                |r| {
                    Ok(PetState {
                        name: r.get::<_, Option<String>>(0)?,
                        skills: r.get::<_, i64>(1)? as u16,
                        active: r.get::<_, i64>(2)? != 0,
                    })
                },
            )
            .optional()?
            .unwrap_or(PetState { name: None, skills: PET_SKILLS_AT_START, active: false }))
    }

    /// The pet this character had out, if any.
    pub fn active_pet(&self, character_id: u32) -> Result<Option<u32>> {
        let conn = self.conn();
        ensure_table(&conn)?;
        Ok(conn
            .query_row(
                "SELECT item_id FROM character_pets WHERE character_id = ?1 AND active = 1",
                [character_id],
                |r| r.get::<_, i64>(0),
            )
            .optional()?
            .map(|v| v as u32))
    }

    /// Mark `item_id` as the pet that is out (and every other pet as put away), or put it
    /// away. One transaction, so a character can never have two active rows.
    pub fn set_pet_active(&self, character_id: u32, item_id: u32, active: bool) -> Result<()> {
        let mut conn = self.conn();
        ensure_table(&conn)?;
        let tx = conn.transaction()?;
        tx.execute("UPDATE character_pets SET active = 0 WHERE character_id = ?1", [character_id])?;
        tx.execute(
            "INSERT INTO character_pets (character_id, item_id, active) VALUES (?1, ?2, ?3)
             ON CONFLICT(character_id, item_id) DO UPDATE SET active = excluded.active",
            rusqlite::params![character_id, item_id, i64::from(active)],
        )?;
        tx.commit()?;
        Ok(())
    }

    /// Rename a pet. The caller has already bounded the name against the wire's 13-byte
    /// field; this stores what it is given.
    pub fn set_pet_name(&self, character_id: u32, item_id: u32, name: &str) -> Result<()> {
        let conn = self.conn();
        ensure_table(&conn)?;
        conn.execute(
            "INSERT INTO character_pets (character_id, item_id, name) VALUES (?1, ?2, ?3)
             ON CONFLICT(character_id, item_id) DO UPDATE SET name = excluded.name",
            rusqlite::params![character_id, item_id, name],
        )?;
        Ok(())
    }

    /// OR `bits` into the pet's learned-skill mask. Returns the new mask.
    pub fn learn_pet_skill(&self, character_id: u32, item_id: u32, bits: u16) -> Result<u16> {
        let conn = self.conn();
        ensure_table(&conn)?;
        conn.execute(
            "INSERT INTO character_pets (character_id, item_id, skills) VALUES (?1, ?2, ?3)
             ON CONFLICT(character_id, item_id) DO UPDATE SET skills = skills | excluded.skills",
            rusqlite::params![character_id, item_id, i64::from(PET_SKILLS_AT_START | bits)],
        )?;
        Ok(conn.query_row(
            "SELECT skills FROM character_pets WHERE character_id = ?1 AND item_id = ?2",
            rusqlite::params![character_id, item_id],
            |r| r.get::<_, i64>(0),
        )? as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store_with_character() -> (Store, u32) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Wisp".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        (store, id)
    }

    const HUSKY: u32 = 5_000_006;

    #[test]
    fn an_untouched_pet_has_the_item_name_the_starting_skill_and_is_put_away() {
        let (store, chr) = store_with_character();
        assert_eq!(
            store.pet_state(chr, HUSKY).unwrap(),
            PetState { name: None, skills: PET_SKILLS_AT_START, active: false }
        );
        assert_eq!(store.active_pet(chr).unwrap(), None);
    }

    /// The re-login case: summoned, gone, back - the row says it is still out.
    #[test]
    fn the_active_pet_survives_and_a_put_away_clears_it() {
        let (store, chr) = store_with_character();
        store.set_pet_active(chr, HUSKY, true).unwrap();
        assert_eq!(store.active_pet(chr).unwrap(), Some(HUSKY));
        // A second pet summoned puts the first away by itself.
        store.set_pet_active(chr, 5_000_001, true).unwrap();
        assert_eq!(store.active_pet(chr).unwrap(), Some(5_000_001));
        assert!(!store.pet_state(chr, HUSKY).unwrap().active);
        store.set_pet_active(chr, 5_000_001, false).unwrap();
        assert_eq!(store.active_pet(chr).unwrap(), None);
    }

    #[test]
    fn a_rename_and_a_learned_skill_keep_each_other_and_the_active_flag() {
        let (store, chr) = store_with_character();
        store.set_pet_active(chr, HUSKY, true).unwrap();
        store.set_pet_name(chr, HUSKY, "Dummy").unwrap();
        assert_eq!(store.learn_pet_skill(chr, HUSKY, 1 << 1).unwrap(), PET_SKILLS_AT_START | (1 << 1));
        assert_eq!(store.learn_pet_skill(chr, HUSKY, 1 << 4).unwrap(), PET_SKILLS_AT_START | (1 << 1) | (1 << 4));
        assert_eq!(
            store.pet_state(chr, HUSKY).unwrap(),
            PetState { name: Some("Dummy".to_string()), skills: 0b1_0011, active: true }
        );
        // Learning on a pet with no row yet starts from the default mask, not from zero.
        assert_eq!(store.learn_pet_skill(chr, 5_000_002, 1 << 3).unwrap(), PET_SKILLS_AT_START | (1 << 3));
    }

    #[test]
    fn two_characters_do_not_share_a_pet() {
        let (store, chr) = store_with_character();
        let other = store
            .create_character(
                store.create_account("someone", "else entirely here").unwrap(),
                0,
                &net::opcode::Character { name: "Other".to_string(), ..Default::default() },
            )
            .unwrap()
            .id;
        store.set_pet_name(chr, HUSKY, "Dummy").unwrap();
        store.set_pet_active(chr, HUSKY, true).unwrap();
        assert_eq!(store.pet_state(other, HUSKY).unwrap().name, None);
        assert_eq!(store.active_pet(other).unwrap(), None);
    }
}
