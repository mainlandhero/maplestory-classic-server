//! Character persistence - the thing that makes a character survive a relaunch.
//!
//! Until this existed the character list was generated from a command-line argument at
//! launch, and a character created in one session was gone in the next; the client only
//! appeared to keep it because it added it locally when the create result came back.
//!
//! # Why this speaks the protocol's `Character` type
//!
//! A separate storage struct would mean mapping nineteen fields by hand in the login
//! server, and a field left out of that mapping is a stat that silently does not persist.
//! That is the same class of bug as the avatar-look field order, which cost a session and
//! was invisible to a round-trip test. So the row maps to [`net::opcode::Character`]
//! directly, and both directions **destructure it exhaustively** - add a field to
//! `Character` and this file stops compiling until the column exists.
//!
//! # Names are unique across the whole service
//!
//! `UNIQUE COLLATE NOCASE` on `characters.name`. That is what lets the name check
//! (`0x0081`) be answered truthfully, which is the first thing a real server does that
//! the harness could not: the harness always replied "available".

use crate::db::Store;
use crate::error::{Result, StoreError};
use net::opcode::Character;

/// Longest name the client can carry. The record writes the name into a fixed 13-byte
/// block, so 12 characters plus a terminator is the whole budget.
pub const MAX_CHARACTER_NAME_LEN: usize = 12;

/// Shortest name the creation screen accepts.
pub const MIN_CHARACTER_NAME_LEN: usize = 4;

/// Why a name cannot be used. The login server maps these onto the client's own result
/// codes, which distinguish "already used" from "not allowed".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameCheck {
    Available,
    AlreadyUsed,
    NotAllowed,
}

impl Store {
    /// Validate a character name the way the service will, without touching the database.
    ///
    /// Kept separate from [`Store::check_character_name`] so the rule can be tested and
    /// quoted without a connection.
    pub fn character_name_allowed(name: &str) -> bool {
        name.len() >= MIN_CHARACTER_NAME_LEN
            && name.len() <= MAX_CHARACTER_NAME_LEN
            && name.chars().all(|c| c.is_ascii_alphanumeric())
    }

    /// Is this name usable? This answers the client's name check.
    pub fn check_character_name(&self, name: &str) -> Result<NameCheck> {
        if !Self::character_name_allowed(name) {
            return Ok(NameCheck::NotAllowed);
        }
        let taken: i64 = self.conn().query_row(
            "SELECT COUNT(*) FROM characters WHERE name = ?1 COLLATE NOCASE",
            [name],
            |row| row.get(0),
        )?;
        Ok(if taken == 0 { NameCheck::Available } else { NameCheck::AlreadyUsed })
    }

    /// Persist a new character and return it with the id the database assigned.
    ///
    /// The id is the one that goes on the wire, and it has to be distinct per character:
    /// a canned reply that sent id 200 twice made the client silently drop the second
    /// character, because it believed it was being handed one it already had.
    pub fn create_character(
        &self,
        account_id: i64,
        world_id: u32,
        chr: &Character,
    ) -> Result<Character> {
        // Exhaustive destructure: a new field on Character breaks this line, not a run.
        let Character {
            id: _,
            name,
            gender,
            skin,
            face,
            hair,
            level,
            job,
            strength,
            dexterity,
            intelligence,
            luck,
            hp,
            max_hp,
            mp,
            max_mp,
            ap,
            map_id,
            equips,
        } = chr;

        match self.check_character_name(name)? {
            NameCheck::Available => {}
            NameCheck::AlreadyUsed => {
                return Err(StoreError::CharacterNameTaken { name: name.clone() })
            }
            NameCheck::NotAllowed => {
                return Err(StoreError::InvalidCharacterName { name: name.clone() })
            }
        }

        let mut conn = self.conn();
        let tx = conn.transaction()?;
        tx.execute(
            "INSERT INTO characters (
                 account_id, world_id, name, gender, skin, face, hair, level, job,
                 strength, dexterity, intelligence, luck,
                 hp, max_hp, mp, max_mp, ap, map_id, created_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13,
                       ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
            rusqlite::params![
                account_id,
                world_id,
                name,
                gender,
                skin,
                face,
                hair,
                level,
                job,
                strength,
                dexterity,
                intelligence,
                luck,
                hp,
                max_hp,
                mp,
                max_mp,
                ap,
                map_id,
                Store::now(),
            ],
        )?;
        let id = tx.last_insert_rowid();
        {
            let mut insert = tx.prepare(
                "INSERT INTO equipment (character_id, slot, item_id) VALUES (?1, ?2, ?3)",
            )?;
            for (slot, item_id) in equips {
                insert.execute(rusqlite::params![id, slot, item_id])?;
            }
        }
        tx.commit()?;

        let mut created = chr.clone();
        created.id = id as u32;
        Ok(created)
    }

    /// Every character on this account in this world, oldest first.
    ///
    /// The order is the order the client draws them in, so it has to be stable across
    /// launches - `created_at` alone is not, since two characters made in the same second
    /// would tie. The id breaks the tie.
    pub fn characters_for(&self, account_id: i64, world_id: u32) -> Result<Vec<Character>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT id, name, gender, skin, face, hair, level, job,
                    strength, dexterity, intelligence, luck,
                    hp, max_hp, mp, max_mp, ap, map_id
               FROM characters
              WHERE account_id = ?1 AND world_id = ?2
              ORDER BY created_at, id",
        )?;
        let rows = stmt.query_map(rusqlite::params![account_id, world_id], |row| {
            Ok(Character {
                id: row.get::<_, i64>(0)? as u32,
                name: row.get(1)?,
                gender: row.get(2)?,
                skin: row.get(3)?,
                face: row.get(4)?,
                hair: row.get(5)?,
                level: row.get(6)?,
                job: row.get(7)?,
                strength: row.get(8)?,
                dexterity: row.get(9)?,
                intelligence: row.get(10)?,
                luck: row.get(11)?,
                hp: row.get(12)?,
                max_hp: row.get(13)?,
                mp: row.get(14)?,
                max_mp: row.get(15)?,
                ap: row.get(16)?,
                map_id: row.get(17)?,
                equips: Vec::new(),
            })
        })?;
        let mut characters: Vec<Character> = rows.collect::<rusqlite::Result<_>>()?;

        let mut equip_stmt = conn.prepare(
            "SELECT slot, item_id FROM equipment WHERE character_id = ?1 ORDER BY slot",
        )?;
        for chr in &mut characters {
            let equips = equip_stmt.query_map([i64::from(chr.id)], |row| {
                Ok((row.get::<_, u8>(0)?, row.get::<_, u32>(1)?))
            })?;
            chr.equips = equips.collect::<rusqlite::Result<_>>()?;
        }
        Ok(characters)
    }

    /// How many characters this account has in this world. The login result carries the
    /// account's slot count, and the client computes the free slot from the two.
    pub fn character_count(&self, account_id: i64, world_id: u32) -> Result<u32> {
        let n: i64 = self.conn().query_row(
            "SELECT COUNT(*) FROM characters WHERE account_id = ?1 AND world_id = ?2",
            rusqlite::params![account_id, world_id],
            |row| row.get(0),
        )?;
        Ok(n as u32)
    }

    /// Delete a character, but only if this account owns it.
    ///
    /// The ownership clause is in the statement rather than in a prior read: a check and
    /// then a delete is two statements a concurrent request can slip between, and the
    /// whole point of an ownership test is that it cannot be raced.
    pub fn delete_character(&self, account_id: i64, character_id: u32) -> Result<bool> {
        let deleted = self.conn().execute(
            "DELETE FROM characters WHERE id = ?1 AND account_id = ?2",
            rusqlite::params![i64::from(character_id), account_id],
        )?;
        Ok(deleted > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store_with_account() -> (Store, i64) {
        let store = Store::open_in_memory().unwrap();
        let id = store.create_account("wisp", "correct horse battery").unwrap();
        (store, id)
    }

    fn equipment_rows(store: &Store) -> i64 {
        store
            .conn()
            .query_row("SELECT COUNT(*) FROM equipment", [], |row| row.get(0))
            .unwrap()
    }

    fn named(name: &str) -> Character {
        Character { name: name.to_string(), ..Character::default() }
    }

    #[test]
    fn a_character_survives_being_read_back() {
        let (store, account) = store_with_account();
        let chr = Character {
            name: "Hello".into(),
            gender: 1,
            skin: 3,
            face: 21002,
            hair: 30030,
            strength: 13,
            dexterity: 6,
            intelligence: 5,
            luck: 4,
            equips: vec![(5, 1040002), (6, 1060002), (7, 1072001), (11, 1302000)],
            ..Character::default()
        };
        let created = store.create_character(account, 0, &chr).unwrap();

        let back = store.characters_for(account, 0).unwrap();
        assert_eq!(back.len(), 1);
        // Everything but the id, which the database assigns.
        assert_eq!(back[0], Character { id: created.id, ..chr });
    }

    #[test]
    fn the_first_character_id_is_the_one_the_client_has_accepted() {
        // Ids start at 200, not 1. On 2026-08-18 a create reply carrying id 1 was
        // byte-identical to one that had transitioned the client except for the two
        // copies of this id, and the client did not transition. 200 is the value that
        // was working.
        let (store, account) = store_with_account();
        let first = store.create_character(account, 0, &named("First")).unwrap();
        assert_eq!(first.id, crate::db::FIRST_CHARACTER_ID);
    }

    #[test]
    fn the_id_seed_does_not_add_a_second_sequence_row() {
        // sqlite_sequence has no UNIQUE constraint on `name`, so an INSERT OR IGNORE
        // would append a duplicate rather than skip - and two rows for one table is not
        // something AUTOINCREMENT is defined against. Guarded here because the schema
        // runs on every open, not just on create.
        let (store, account) = store_with_account();
        store.create_character(account, 0, &named("First")).unwrap();
        let rows: i64 = store
            .conn()
            .query_row(
                "SELECT COUNT(*) FROM sqlite_sequence WHERE name = 'characters'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(rows, 1);
    }

    #[test]
    fn the_assigned_id_is_distinct_per_character() {
        // A canned reply once sent id 200 twice and the client dropped the second
        // character, so distinctness is a protocol requirement rather than tidiness.
        let (store, account) = store_with_account();
        let a = store.create_character(account, 0, &named("Alpha")).unwrap();
        let b = store.create_character(account, 0, &named("Bravo")).unwrap();
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn a_taken_name_is_refused_whatever_its_case() {
        let (store, account) = store_with_account();
        store.create_character(account, 0, &named("Hello")).unwrap();

        assert_eq!(store.check_character_name("hello").unwrap(), NameCheck::AlreadyUsed);
        assert!(matches!(
            store.create_character(account, 0, &named("HELLO")),
            Err(StoreError::CharacterNameTaken { .. })
        ));
    }

    #[test]
    fn a_name_taken_on_another_account_is_still_taken() {
        // Names are unique across the service, not per account - the client's name check
        // has no account in it to scope by.
        let (store, first) = store_with_account();
        let second = store.create_account("someone_else", "correct horse battery").unwrap();
        store.create_character(first, 0, &named("Hello")).unwrap();
        assert_eq!(store.check_character_name("Hello").unwrap(), NameCheck::AlreadyUsed);
        assert!(store.create_character(second, 0, &named("Hello")).is_err());
    }

    #[test]
    fn names_the_client_could_not_carry_are_not_allowed() {
        let (store, _) = store_with_account();
        assert_eq!(store.check_character_name("abc").unwrap(), NameCheck::NotAllowed);
        assert_eq!(store.check_character_name("abcdefghijklm").unwrap(), NameCheck::NotAllowed);
        assert_eq!(store.check_character_name("has space").unwrap(), NameCheck::NotAllowed);
        // Twelve is the limit, and it is allowed.
        assert_eq!(store.check_character_name("abcdefghijkl").unwrap(), NameCheck::Available);
    }

    #[test]
    fn equipment_comes_back_in_slot_order() {
        let (store, account) = store_with_account();
        let chr = Character {
            name: "Dressed".into(),
            equips: vec![(11, 1302000), (5, 1040002), (7, 1072001)],
            ..Character::default()
        };
        store.create_character(account, 0, &chr).unwrap();
        let back = store.characters_for(account, 0).unwrap();
        assert_eq!(back[0].equips, vec![(5, 1040002), (7, 1072001), (11, 1302000)]);
    }

    #[test]
    fn characters_come_back_in_creation_order() {
        let (store, account) = store_with_account();
        for name in ["Alpha", "Bravo", "Charlie"] {
            store.create_character(account, 0, &named(name)).unwrap();
        }
        let names: Vec<String> =
            store.characters_for(account, 0).unwrap().into_iter().map(|c| c.name).collect();
        assert_eq!(names, vec!["Alpha", "Bravo", "Charlie"]);
    }

    #[test]
    fn one_account_never_sees_anothers_characters() {
        let (store, first) = store_with_account();
        let second = store.create_account("someone_else", "correct horse battery").unwrap();
        store.create_character(first, 0, &named("Mine")).unwrap();
        assert_eq!(store.characters_for(second, 0).unwrap(), vec![]);
        assert_eq!(store.character_count(second, 0).unwrap(), 0);
        assert_eq!(store.character_count(first, 0).unwrap(), 1);
    }

    #[test]
    fn worlds_do_not_share_a_character_list() {
        let (store, account) = store_with_account();
        store.create_character(account, 0, &named("Scania")).unwrap();
        assert_eq!(store.characters_for(account, 0).unwrap().len(), 1);
        assert_eq!(store.characters_for(account, 1).unwrap().len(), 0);
    }

    #[test]
    fn deleting_needs_ownership() {
        let (store, owner) = store_with_account();
        let other = store.create_account("someone_else", "correct horse battery").unwrap();
        let chr = store.create_character(owner, 0, &named("Doomed")).unwrap();

        assert!(!store.delete_character(other, chr.id).unwrap());
        assert_eq!(store.character_count(owner, 0).unwrap(), 1);

        assert!(store.delete_character(owner, chr.id).unwrap());
        assert_eq!(store.character_count(owner, 0).unwrap(), 0);
    }

    #[test]
    fn deleting_a_character_frees_its_name_and_its_equipment() {
        let (store, account) = store_with_account();
        let chr = Character {
            name: "Doomed".into(),
            equips: vec![(5, 1040002)],
            ..Character::default()
        };
        let created = store.create_character(account, 0, &chr).unwrap();
        // Prove the row exists first: "no orphans" is also what a failed insert looks
        // like, and an assertion that cannot fail measures nothing.
        assert_eq!(equipment_rows(&store), 1);

        store.delete_character(account, created.id).unwrap();

        assert_eq!(store.check_character_name("Doomed").unwrap(), NameCheck::Available);
        assert_eq!(
            equipment_rows(&store),
            0,
            "ON DELETE CASCADE should have taken the equipment too"
        );
    }

    #[test]
    fn deleting_an_account_takes_its_characters_with_it() {
        let (store, account) = store_with_account();
        store.create_character(account, 0, &named("Hello")).unwrap();
        store.conn().execute("DELETE FROM accounts WHERE id = ?1", [account]).unwrap();
        assert_eq!(store.check_character_name("Hello").unwrap(), NameCheck::Available);
    }
}
