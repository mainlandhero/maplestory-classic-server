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

use crate::db::{Store, INVENTORY_SLOT_COLUMNS};
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

    /// **The id behind a character name**, or `None` if nobody has it.
    ///
    /// `COLLATE NOCASE` for the same reason [`Store::check_character_name`] uses it: the
    /// client shows one capitalisation and a player types another, and two characters cannot
    /// differ by case anyway because the name check refuses it.
    ///
    /// Added for `!party invite <name>`, which is how a party is formed while `0x0182`'s
    /// invite payload is still undecoded.
    pub fn character_id_by_name(&self, name: &str) -> Result<Option<u32>> {
        use rusqlite::OptionalExtension;
        Ok(self
            .conn()
            .query_row(
                "SELECT id FROM characters WHERE name = ?1 COLLATE NOCASE",
                [name],
                |row| row.get::<_, i64>(0),
            )
            .optional()?
            .map(|id| id as u32))
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
            // Not persisted: `portal` is where a character ARRIVES on a map, which is a
            // property of the walk that got them there, not of the character.
            portal: _,
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
            exp,
            equips,
            // Not persisted HERE. The Equip tab's contents live in the `inventory` table,
            // written by `Store::unequip_to_bag` and friends and read back by `Store::bag`;
            // a copy in `characters` would be a second source of truth for the same rows.
            // Named rather than `..`-ignored so this stays a decision someone made.
            equip_bag: _,
            inventory_slots,
            // A new character has no fame and the column defaults to 0; the only writer is
            // `Store::give_fame`, so a value on a `Character` handed in here is not honoured.
            fame: _,
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
            &format!(
                "INSERT INTO characters (
                     account_id, world_id, name, gender, skin, face, hair, level, job,
                     strength, dexterity, intelligence, luck,
                     hp, max_hp, mp, max_mp, ap, map_id, created_at, {}, exp
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13,
                           ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26,
                           ?27)",
                INVENTORY_SLOT_COLUMNS.join(", ")
            ),
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
                inventory_slots[0],
                inventory_slots[1],
                inventory_slots[2],
                inventory_slots[3],
                inventory_slots[4],
                inventory_slots[5],
                // Tacked on the end rather than slotted in beside `ap` and `map_id` where
                // it belongs, because these placeholders are numbered by hand: putting it
                // in the middle means renumbering seven of them, and a placeholder that
                // ends up one out writes a real value into the wrong column. A new
                // character's experience is 0 anyway, so nothing rides on it today - it is
                // here so that a caller passing a non-zero one is not silently ignored.
                exp,
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
        let mut stmt = conn.prepare(&format!(
            "SELECT id, name, gender, skin, face, hair, level, job,
                    strength, dexterity, intelligence, luck,
                    hp, max_hp, mp, max_mp, ap, map_id, exp, {}, fame
               FROM characters
              WHERE account_id = ?1 AND world_id = ?2
              ORDER BY created_at, id",
            INVENTORY_SLOT_COLUMNS.join(", ")
        ))?;
        let rows = stmt.query_map(rusqlite::params![account_id, world_id], |row| {
            Ok(Character {
                id: row.get::<_, i64>(0)? as u32,
                portal: 0, // the spawn point; a portal walk overrides it per arrival
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
                exp: row.get(18)?,
                equips: Vec::new(),
                // Filled by the caller that needs it, which is the world server on field
                // entry - see Store::bag. The login server's character-select list does not
                // send bag contents, so loading them here would be a query per character
                // for bytes nobody puts on the wire.
                equip_bag: Vec::new(),
                // 19.. and not 18.., because `exp` was inserted before the interpolated
                // slot columns on 2026-08-20. Positional `row.get` indices are the one
                // place where adding a column silently reads the wrong field rather than
                // failing: every slot would have come back holding the experience.
                inventory_slots: [
                    row.get(19)?,
                    row.get(20)?,
                    row.get(21)?,
                    row.get(22)?,
                    row.get(23)?,
                    row.get(24)?,
                ],
                // After the six slot columns, so their positional indices above do not move.
                fame: row.get(25)?,
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

    /// Move a character to a map, so a portal walk survives a relog.
    ///
    /// Deliberately does **not** take an account id. The channel connection carries no
    /// credentials at all - it is identified only by the migration row it claimed - so there
    /// is no account here to check against, and pretending otherwise would be security
    /// theatre. Nothing on this path authenticates anybody.
    pub fn set_character_map(&self, character_id: u32, map_id: u32) -> Result<()> {
        self.conn()
            .execute("UPDATE characters SET map_id = ?2 WHERE id = ?1", (character_id, map_id))?;
        Ok(())
    }

    /// Change a character's hair or face id. `Ok(false)` when no such character.
    ///
    /// **Nothing here checks that the id has art.** The client draws hair from
    /// `Character/Hair/%08d.img` and face from `Character/Face/%08d.img` by the id alone
    /// (`research/beauty-2026-09-09.md`), and an id with no image draws nothing - so the
    /// caller gates on a table of ids that exist. Same account-less shape as
    /// `set_character_map`, for the same reason: nothing on the channel authenticates.
    pub fn set_character_look(
        &self,
        character_id: u32,
        hair: Option<u32>,
        face: Option<u32>,
    ) -> Result<bool> {
        let conn = self.conn();
        let mut n = 0;
        if let Some(h) = hair {
            n += conn.execute("UPDATE characters SET hair = ?2 WHERE id = ?1", (character_id, h))?;
        }
        if let Some(f) = face {
            n += conn.execute("UPDATE characters SET face = ?2 WHERE id = ?1", (character_id, f))?;
        }
        Ok(n > 0)
    }

    /// Write back everything a character can *earn*: level, experience, job, the four base
    /// stats, the four HP/MP numbers and unspent AP.
    ///
    /// **Deliberately mechanical.** It applies whatever the caller decided and enforces no
    /// rule of its own - no experience curve, no HP-per-level, no cap. Those numbers have to
    /// match the ones the *client* already has baked in or the bar and the numbers disagree
    /// on screen, and none of them has been read out of the client yet. Putting a guess in
    /// the storage layer would make it look settled.
    ///
    /// Takes the whole character rather than a field, because a level-up moves seven of
    /// these columns at once and seven setters is seven chances to forget one. `map_id` is
    /// **not** here - it has [`Self::set_character_map`], which the portal walk already
    /// calls - and neither are name, face or hair, which are not earned.
    ///
    /// Like every other write on this path it takes no account id. The channel connection
    /// carries no credentials at all; it is identified only by the migration row it claimed.
    /// **Nothing here authenticates anybody.**
    pub fn save_character_progress(&self, chr: &Character) -> Result<()> {
        self.conn().execute(
            "UPDATE characters
                SET level = ?2, exp = ?3, job = ?4,
                    strength = ?5, dexterity = ?6, intelligence = ?7, luck = ?8,
                    hp = ?9, max_hp = ?10, mp = ?11, max_mp = ?12, ap = ?13
              WHERE id = ?1",
            rusqlite::params![
                chr.id,
                chr.level,
                chr.exp,
                chr.job,
                chr.strength,
                chr.dexterity,
                chr.intelligence,
                chr.luck,
                chr.hp,
                chr.max_hp,
                chr.mp,
                chr.max_mp,
                chr.ap,
            ],
        )?;
        Ok(())
    }

    /// Delete a character, but only if this account owns it.
    ///
    /// The ownership clause is in the statement rather than in a prior read: a check and
    /// then a delete is two statements a concurrent request can slip between, and the whole
    /// point of an ownership test is that it cannot be raced.
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

    /// A new character gets a bag, and it survives the round trip.
    ///
    /// The client is *told* its slot counts - `presence[7]` in the record - so a zero here
    /// is not "the client falls back to a default", it is an inventory the client believes
    /// has no slots. See `net::opcode::PRESENCE_INVENTORY_SIZE`.
    #[test]
    fn a_new_character_gets_a_bag_with_slots_in_it() {
        let (store, account) = store_with_account();
        let made = store.create_character(account, 0, &named("Wanderer")).unwrap();
        assert_eq!(made.inventory_slots, net::opcode::default_inventory_slots());
        assert_eq!(made.inventory_slots[5], 150, "the Deco tab starts at its ceiling");

        let loaded = store.characters_for(account, 0).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].inventory_slots, made.inventory_slots);
        assert!(
            loaded[0].inventory_slots.iter().all(|&n| n > 0),
            "a zero-slot inventory is a bag that refuses everything"
        );
    }

    /// Experience survives a relog. Nothing awards any yet - this is the storage half of
    /// goal D, built ahead of the half that is blocked.
    #[test]
    fn experience_survives_a_reload() {
        let (store, account) = store_with_account();
        let made = store.create_character(account, 0, &named("Grinder")).unwrap();
        assert_eq!(made.exp, 0, "a new character has earned nothing");

        let mut chr = made.clone();
        chr.exp = 42;
        store.save_character_progress(&chr).unwrap();

        let loaded = store.characters_for(account, 0).unwrap();
        assert_eq!(loaded[0].exp, 42);
        // And the write did not disturb the neighbouring columns. `exp` was appended to the
        // end of the INSERT's hand-numbered placeholder list, so "it went into the right
        // column" is a claim worth checking rather than assuming.
        assert_eq!(loaded[0].map_id, made.map_id);
        assert_eq!(loaded[0].ap, made.ap);
        assert_eq!(loaded[0].inventory_slots, made.inventory_slots);
    }

    /// A level-up moves seven columns at once, which is the reason the writer takes a whole
    /// character rather than a field at a time.
    #[test]
    fn a_level_up_shaped_write_moves_every_column_together() {
        let (store, account) = store_with_account();
        let made = store.create_character(account, 0, &named("Riser")).unwrap();

        let mut chr = made.clone();
        chr.level = 2;
        chr.exp = 3;
        chr.max_hp = 65;
        chr.hp = 65;
        chr.max_mp = 12;
        chr.mp = 12;
        chr.ap = 5;
        store.save_character_progress(&chr).unwrap();

        let loaded = &store.characters_for(account, 0).unwrap()[0];
        assert_eq!(
            (loaded.level, loaded.exp, loaded.hp, loaded.max_hp, loaded.mp, loaded.max_mp, loaded.ap),
            (2, 3, 65, 65, 12, 12, 5)
        );
        // Not a progression column, and it has its own setter. A writer that quietly moved
        // the character back to the start map on every level-up would be a nasty one to
        // find, because it would only show on the relog after the level.
        assert_eq!(loaded.map_id, made.map_id);
        assert_eq!(loaded.name, made.name, "and the identity columns are untouched");
    }

    /// A raised slot count persists. This is the whole reason the counts are columns rather
    /// than a constant: buying slots has to survive a relog or it is not a purchase.
    #[test]
    fn a_bought_slot_survives_a_reload() {
        let (store, account) = store_with_account();
        let mut chr = named("Buyer");
        chr.inventory_slots[1] = 48;
        store.create_character(account, 0, &chr).unwrap();

        let loaded = store.characters_for(account, 0).unwrap();
        assert_eq!(loaded[0].inventory_slots[1], 48);
        // And only that one moved.
        assert_eq!(loaded[0].inventory_slots[0], net::opcode::DEFAULT_INVENTORY_SLOTS);
        assert_eq!(loaded[0].inventory_slots[2], net::opcode::DEFAULT_INVENTORY_SLOTS);
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

impl Store {
    /// A character's name by id, or `None`. For packets that name somebody OTHER than the
    /// connection's own character - a party invite outcome names the target, a join names the
    /// joiner - where the session holds only an id from the state machine.
    pub fn character_name(&self, character_id: u32) -> Result<Option<String>> {
        use rusqlite::OptionalExtension as _;
        Ok(self
            .conn()
            .query_row(
                "SELECT name FROM characters WHERE id = ?1",
                rusqlite::params![character_id],
                |row| row.get(0),
            )
            .optional()?)
    }

    /// Name, job and level of one character, for a party seat. `None` when there is no
    /// such character. The level is as last saved; a member's own session holds the live
    /// value, and the world prefers that when it has it.
    pub fn character_brief(&self, character_id: u32) -> Result<Option<CharacterBrief>> {
        use rusqlite::OptionalExtension as _;
        Ok(self
            .conn()
            .query_row(
                "SELECT name, job, level FROM characters WHERE id = ?1",
                rusqlite::params![character_id],
                |row| {
                    Ok(CharacterBrief {
                        id: character_id,
                        name: row.get(0)?,
                        job: row.get(1)?,
                        level: row.get(2)?,
                    })
                },
            )
            .optional()?)
    }
}

/// What a party seat needs to know about a character who is not on this connection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterBrief {
    pub id: u32,
    pub name: String,
    pub job: u32,
    pub level: u32,
}

#[cfg(test)]
mod name_by_id_tests {
    use super::*;

    #[test]
    fn a_character_name_is_found_by_id_and_a_missing_id_is_none() {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Tester2".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        assert_eq!(store.character_name(id).unwrap().as_deref(), Some("Tester2"));
        assert_eq!(store.character_name(id + 1000).unwrap(), None);
    }
}
