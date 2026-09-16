//! Per-pet state that outlives a session: its name, the skills it has learned, its vitals,
//! and whether it was out when the player last left.
//!
//! The owner, 2026-09-15: *"Pets that were spawned from before does not survive a re-login, pets
//! that were previously summoned by user should keep their state."* And in the same breath:
//! a rename with a Pet Name Tag, and Auto HP / Auto MP / Auto Move skill items, none of which
//! had anywhere to land.
//!
//! # Keyed by `pet_id`, a number of the pet's own - not the item id, not the slot
//!
//! Until 2026-09-16 this was keyed by `(character, pet item id)`, and the module doc said the
//! consequence out loud: *"two of the same pet on one character share a name and a skill
//! set ... noted rather than solved."* The owner: *"Two Husky should not share the same name. The
//! pets should in the background have different ids to identify them apart."*
//!
//! So every pet is a row in `pets` with an `AUTOINCREMENT` id, and the pet ITEM carries that
//! id in `inventory.pet_id` (`Item::pet_id`), the way `failed_slots` travels with an equip:
//! a move keeps it, a locker trip keeps it, and `place_into_bag` hands a fresh number to a
//! pet that arrives without one. The client-facing serial (`net::pet::pet_serial`) is built
//! from the pet id too, so the client pairs the Cash item and the field pet per pet, and a
//! name tag or a skill item used on one Husky names that Husky.
//!
//! # `active` is the pet the next login re-summons
//!
//! Exactly one row per character may be active - `set_pet_active` clears the others in the
//! same transaction - and a put-away clears it. `Session::restore_active_pet` reads it once
//! at claim time and re-summons on the first field entry, which is the reference server's
//! `initPets` behaviour: what the row says is out, is out.
//!
//! # The 2026-09-15 rows come along
//!
//! `create_tables` moves every `character_pets` row onto `pets` - name, skills, active and
//! vitals - and stamps the id onto the first un-numbered pet of that item in that character's
//! bag, then numbers whatever pets remain un-numbered. Runs on every open, does nothing the
//! second time (memory: `maplecw-live-db-migrations` - the live server is a different file).
//!
//! # Nothing here authenticates
//!
//! The channel socket carries no credentials. A rename or a skill arrives on the say-so of
//! whoever holds the connection, like every other packet on it.

use rusqlite::{Connection, OptionalExtension};

use crate::db::Store;
use crate::error::Result;
use crate::inventory::InventoryType;

/// What is stored about one pet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PetState {
    /// The player's name for it, or `None` for the item's own name.
    pub name: Option<String>,
    /// The learned-skill mask, in the client's own bit order (`net::bag::PET_SKILL_*`).
    pub skills: u16,
    /// Whether it was summoned when last seen.
    pub active: bool,
    /// `1..=30`; never lowered (`world::petlevel`).
    pub level: u8,
    /// Total closeness earned, the level table's input.
    pub closeness: u32,
    /// `0..=100`; a feed adds, five minutes summoned subtracts one.
    pub fullness: u8,
}

impl PetState {
    /// A pet nothing has happened to.
    pub fn fresh() -> Self {
        PetState { name: None, skills: PET_SKILLS_AT_START, active: false, level: 1, closeness: 0, fullness: 100 }
    }
}

/// The pet a character had out: which row, and which item it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivePetRow {
    pub pet_id: u32,
    pub item_id: u32,
}

/// The mask every pet has - Item Pouch, Expanded Auto Move, Auto Move: a vacuum pet (the owner,
/// 2026-09-16). Mirrors `net::bag::PET_SKILLS_LEARNED_AT_START`; `store` does not depend on
/// that constant so the crate boundary stays one-way, and a test here pins the two together.
/// **ORed into every mask on read**, so a row written before the auto-move bits were default
/// (the live server's rows are `1` or `3`) reads as a vacuum pet without a migration.
pub const PET_SKILLS_AT_START: u16 = 0b1101;

/// The `pets` table, the migration of the old `character_pets` rows onto it, and a number
/// for every pet item that has none. Called from `Store::open` after `inventory` has its
/// `pet_id` column; every method below also calls it, so a store opened by a test that
/// bypasses `open` still has the table.
pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS pets (
            pet_id       INTEGER PRIMARY KEY AUTOINCREMENT,
            character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            item_id      INTEGER NOT NULL,
            name         TEXT,
            skills       INTEGER NOT NULL DEFAULT 1,
            active       INTEGER NOT NULL DEFAULT 0,
            level        INTEGER NOT NULL DEFAULT 1,
            closeness    INTEGER NOT NULL DEFAULT 0,
            fullness     INTEGER NOT NULL DEFAULT 100
        );
        CREATE INDEX IF NOT EXISTS idx_pets_character ON pets(character_id);",
    )?;
    migrate_character_pets(conn)?;
    number_unnumbered_pets(conn)?;
    Ok(())
}

/// Move the 2026-09-15 `character_pets` rows (keyed by character + item id) onto `pets`,
/// stamping each new id onto the first un-numbered pet of that item in that character's bag.
/// The old table is renamed, not dropped, so a second open finds nothing to do and the rows
/// are still there to look at.
fn migrate_character_pets(conn: &Connection) -> Result<()> {
    let old_exists: bool = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'character_pets'",
        [],
        |r| r.get::<_, i64>(0),
    )? > 0;
    if !old_exists {
        return Ok(());
    }
    // The old table may predate its vitals columns (716589b shipped without them).
    let mut have = std::collections::HashSet::new();
    {
        let mut stmt = conn.prepare("PRAGMA table_info(character_pets)")?;
        for name in stmt.query_map([], |row| row.get::<_, String>(1))? {
            have.insert(name?);
        }
    }
    let col = |name: &str, default: &str| if have.contains(name) { name.to_string() } else { default.to_string() };
    let select = format!(
        "SELECT character_id, item_id, name, skills, active, {}, {}, {} FROM character_pets ORDER BY rowid",
        col("level", "1"), col("closeness", "0"), col("fullness", "100")
    );
    let rows: Vec<(i64, i64, Option<String>, i64, i64, i64, i64, i64)> = {
        let mut stmt = conn.prepare(&select)?;
        let it = stmt.query_map([], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?, r.get(6)?, r.get(7)?))
        })?;
        it.collect::<rusqlite::Result<_>>()?
    };
    for (character_id, item_id, name, skills, active, level, closeness, fullness) in rows {
        conn.execute(
            "INSERT INTO pets (character_id, item_id, name, skills, active, level, closeness, fullness)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![character_id, item_id, name, skills, active, level, closeness, fullness],
        )?;
        let pet_id = conn.last_insert_rowid();
        conn.execute(
            "UPDATE inventory SET pet_id = ?1 WHERE rowid = (
                SELECT rowid FROM inventory
                WHERE character_id = ?2 AND item_id = ?3 AND pet_id IS NULL
                ORDER BY inv_type, slot LIMIT 1)",
            rusqlite::params![pet_id, character_id, item_id],
        )?;
    }
    conn.execute("ALTER TABLE character_pets RENAME TO character_pets_migrated_2026_09_16", [])?;
    Ok(())
}

/// Every pet item in every bag that has no number gets one. A pet bought before this existed,
/// or one whose `character_pets` row never existed because nothing happened to it.
fn number_unnumbered_pets(conn: &Connection) -> Result<()> {
    let rows: Vec<(i64, i64, i64)> = {
        let mut stmt = conn.prepare(
            "SELECT rowid, character_id, item_id FROM inventory
             WHERE pet_id IS NULL AND item_id BETWEEN 5000000 AND 5009999 ORDER BY rowid",
        )?;
        let it = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
        it.collect::<rusqlite::Result<_>>()?
    };
    for (rowid, character_id, item_id) in rows {
        conn.execute(
            "INSERT INTO pets (character_id, item_id) VALUES (?1, ?2)",
            rusqlite::params![character_id, item_id],
        )?;
        let pet_id = conn.last_insert_rowid();
        conn.execute("UPDATE inventory SET pet_id = ?1 WHERE rowid = ?2", rusqlite::params![pet_id, rowid])?;
    }
    Ok(())
}

/// A fresh `pets` row for a pet item that has none. Returns its id.
pub(crate) fn new_pet(conn: &Connection, character_id: u32, item_id: u32) -> Result<u32> {
    create_tables(conn)?;
    conn.execute(
        "INSERT INTO pets (character_id, item_id) VALUES (?1, ?2)",
        rusqlite::params![character_id, item_id],
    )?;
    Ok(u32::try_from(conn.last_insert_rowid()).unwrap_or(0))
}

/// A numbered pet arriving in `character_id`'s bag: re-home the row if it changed hands
/// (through storage, say), and never arrive summoned. Recreates the row if it is somehow
/// gone, so a pet id on an item always resolves.
pub(crate) fn adopt(conn: &Connection, pet_id: u32, character_id: u32, item_id: u32) -> Result<()> {
    create_tables(conn)?;
    let changed = conn.execute(
        "UPDATE pets SET character_id = ?2, active = CASE WHEN character_id = ?2 THEN active ELSE 0 END
         WHERE pet_id = ?1",
        rusqlite::params![pet_id, character_id],
    )?;
    if changed == 0 {
        conn.execute(
            "INSERT INTO pets (pet_id, character_id, item_id) VALUES (?1, ?2, ?3)",
            rusqlite::params![pet_id, character_id, item_id],
        )?;
    }
    Ok(())
}

impl Store {
    /// This pet's stored state, or the defaults when it has never been touched.
    pub fn pet_state(&self, pet_id: u32) -> Result<PetState> {
        let conn = self.conn();
        create_tables(&conn)?;
        Ok(conn
            .query_row(
                "SELECT name, skills, active, level, closeness, fullness FROM pets WHERE pet_id = ?1",
                [pet_id],
                |r| {
                    Ok(PetState {
                        name: r.get::<_, Option<String>>(0)?,
                        skills: (r.get::<_, i64>(1)? as u16) | PET_SKILLS_AT_START,
                        active: r.get::<_, i64>(2)? != 0,
                        level: r.get::<_, i64>(3)?.clamp(1, 30) as u8,
                        closeness: r.get::<_, i64>(4)?.max(0) as u32,
                        fullness: r.get::<_, i64>(5)?.clamp(0, 100) as u8,
                    })
                },
            )
            .optional()?
            .unwrap_or_else(PetState::fresh))
    }

    /// The pet this character had out, if any.
    pub fn active_pet(&self, character_id: u32) -> Result<Option<ActivePetRow>> {
        let conn = self.conn();
        create_tables(&conn)?;
        Ok(conn
            .query_row(
                "SELECT pet_id, item_id FROM pets WHERE character_id = ?1 AND active = 1 ORDER BY pet_id LIMIT 1",
                [character_id],
                |r| Ok(ActivePetRow { pet_id: r.get::<_, i64>(0)? as u32, item_id: r.get::<_, i64>(1)? as u32 }),
            )
            .optional()?)
    }

    /// Mark `pet_id` as the pet that is out (and every other pet of this character as put
    /// away), or put it away. One transaction, so a character can never have two active rows.
    pub fn set_pet_active(&self, character_id: u32, pet_id: u32, active: bool) -> Result<()> {
        let mut conn = self.conn();
        create_tables(&conn)?;
        let tx = conn.transaction()?;
        tx.execute("UPDATE pets SET active = 0 WHERE character_id = ?1", [character_id])?;
        tx.execute(
            "UPDATE pets SET active = ?2 WHERE pet_id = ?1 AND character_id = ?3",
            rusqlite::params![pet_id, i64::from(active), character_id],
        )?;
        tx.commit()?;
        Ok(())
    }

    /// The three numbers a feed, a command or a hungry five minutes move. Written together,
    /// because they change together and a level is a function of the closeness.
    pub fn set_pet_vitals(&self, pet_id: u32, level: u8, closeness: u32, fullness: u8) -> Result<()> {
        let conn = self.conn();
        create_tables(&conn)?;
        conn.execute(
            "UPDATE pets SET level = ?2, closeness = ?3, fullness = ?4 WHERE pet_id = ?1",
            rusqlite::params![pet_id, i64::from(level), i64::from(closeness), i64::from(fullness)],
        )?;
        Ok(())
    }

    /// Rename a pet. The caller has already bounded the name against the wire's 13-byte
    /// field; this stores what it is given.
    pub fn set_pet_name(&self, pet_id: u32, name: &str) -> Result<()> {
        let conn = self.conn();
        create_tables(&conn)?;
        conn.execute("UPDATE pets SET name = ?2 WHERE pet_id = ?1", rusqlite::params![pet_id, name])?;
        Ok(())
    }

    /// OR `bits` into the pet's learned-skill mask. Returns the new mask.
    pub fn learn_pet_skill(&self, pet_id: u32, bits: u16) -> Result<u16> {
        let conn = self.conn();
        create_tables(&conn)?;
        conn.execute(
            "UPDATE pets SET skills = skills | ?2 WHERE pet_id = ?1",
            rusqlite::params![pet_id, i64::from(PET_SKILLS_AT_START | bits)],
        )?;
        Ok(conn
            .query_row("SELECT skills FROM pets WHERE pet_id = ?1", [pet_id], |r| r.get::<_, i64>(0))
            .optional()?
            .map(|v| v as u16 | PET_SKILLS_AT_START)
            .unwrap_or(PET_SKILLS_AT_START | bits))
    }

    /// The pet id on the item in `slot` of this character's Cash tab, numbering it if it has
    /// none - a pet that reached the bag around `add_item` (a hand-edited row, or one that
    /// predates the column and slipped past the open-time pass) still gets an identity the
    /// first time anything is done with it.
    pub fn pet_id_at(&self, character_id: u32, inv_type: InventoryType, slot: u16) -> Result<Option<u32>> {
        let conn = self.conn();
        create_tables(&conn)?;
        let row: Option<(Option<i64>, i64)> = conn
            .query_row(
                "SELECT pet_id, item_id FROM inventory WHERE character_id = ?1 AND inv_type = ?2 AND slot = ?3",
                rusqlite::params![character_id, inv_type.as_u8(), slot],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;
        let Some((pet_id, item_id)) = row else { return Ok(None) };
        let item_id = u32::try_from(item_id).unwrap_or(0);
        if !net::inventory::is_pet(item_id) {
            return Ok(None);
        }
        if let Some(id) = pet_id {
            return Ok(u32::try_from(id).ok());
        }
        let id = new_pet(&conn, character_id, item_id)?;
        conn.execute(
            "UPDATE inventory SET pet_id = ?1 WHERE character_id = ?2 AND inv_type = ?3 AND slot = ?4",
            rusqlite::params![id, character_id, inv_type.as_u8(), slot],
        )?;
        Ok(Some(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::Item;

    fn store_with_character() -> (Store, u32) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Wisp".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        (store, id)
    }

    const HUSKY: u32 = 5_000_006;

    /// Put a pet in the Cash tab and hand back its number.
    fn buy(store: &Store, chr: u32, item_id: u32) -> u32 {
        let placed = store.add_item(chr, InventoryType::Cash, &Item::bundle(item_id, 1), 1).unwrap();
        placed[0].item.pet_id.expect("a pet leaves add_item numbered")
    }

    /// **Two Huskies are two pets.** The owner, 2026-09-16. Each gets its own number on the way
    /// into the bag, its own name, its own vitals, and summoning one does not summon the
    /// other - and `max_stack` cannot merge them into one slot.
    #[test]
    fn two_huskies_are_two_pets_with_two_names() {
        let (store, chr) = store_with_character();
        let a = buy(&store, chr, HUSKY);
        let b = buy(&store, chr, HUSKY);
        assert_ne!(a, b);
        let bag = store.bag(chr).unwrap();
        let cash: Vec<(u16, u32, Option<u32>)> = bag.items_in(InventoryType::Cash).map(|i| (i.slot, i.item.item_id, i.item.pet_id)).collect();
        assert_eq!(cash, vec![(1, HUSKY, Some(a)), (2, HUSKY, Some(b))], "two slots, two numbers - not one stack");
        store.set_pet_name(a, "Dummy").unwrap();
        store.set_pet_vitals(b, 3, 40, 55).unwrap();
        assert_eq!(store.pet_state(a).unwrap().name.as_deref(), Some("Dummy"));
        assert_eq!(store.pet_state(b).unwrap().name, None, "the other Husky keeps the item's name");
        assert_eq!((store.pet_state(a).unwrap().fullness, store.pet_state(b).unwrap().fullness), (100, 55));
        store.set_pet_active(chr, b, true).unwrap();
        assert_eq!(store.active_pet(chr).unwrap(), Some(ActivePetRow { pet_id: b, item_id: HUSKY }));
        assert!(!store.pet_state(a).unwrap().active);
        // The number travels with the item through a move.
        store.move_item(chr, InventoryType::Cash, 1, 5, None, 1).unwrap();
        let bag = store.bag(chr).unwrap();
        let at5 = bag.items_in(InventoryType::Cash).find(|i| i.slot == 5).unwrap();
        assert_eq!(at5.item.pet_id, Some(a));
        assert_eq!(store.pet_id_at(chr, InventoryType::Cash, 5).unwrap(), Some(a));
        assert_eq!(store.pet_id_at(chr, InventoryType::Cash, 2).unwrap(), Some(b));
        assert_eq!(store.pet_id_at(chr, InventoryType::Cash, 3).unwrap(), None, "an empty slot");
    }

    #[test]
    fn an_untouched_pet_has_the_item_name_the_starting_skill_and_is_put_away() {
        let (store, chr) = store_with_character();
        let id = buy(&store, chr, HUSKY);
        assert_eq!(store.pet_state(id).unwrap(), PetState::fresh());
        assert_eq!((PetState::fresh().level, PetState::fresh().closeness, PetState::fresh().fullness), (1, 0, 100));
        assert_eq!(store.active_pet(chr).unwrap(), None);
        assert_eq!(store.pet_state(9_999).unwrap(), PetState::fresh(), "an unknown id reads as fresh rather than erroring");
    }

    /// The re-login case: summoned, gone, back - the row says it is still out.
    #[test]
    fn the_active_pet_survives_and_a_put_away_clears_it() {
        let (store, chr) = store_with_character();
        let husky = buy(&store, chr, HUSKY);
        let other = buy(&store, chr, 5_000_001);
        store.set_pet_active(chr, husky, true).unwrap();
        assert_eq!(store.active_pet(chr).unwrap().map(|p| p.pet_id), Some(husky));
        // A second pet summoned puts the first away by itself.
        store.set_pet_active(chr, other, true).unwrap();
        assert_eq!(store.active_pet(chr).unwrap(), Some(ActivePetRow { pet_id: other, item_id: 5_000_001 }));
        assert!(!store.pet_state(husky).unwrap().active);
        store.set_pet_active(chr, other, false).unwrap();
        assert_eq!(store.active_pet(chr).unwrap(), None);
    }

    #[test]
    fn a_rename_and_a_learned_skill_keep_each_other_and_the_active_flag() {
        let (store, chr) = store_with_character();
        let husky = buy(&store, chr, HUSKY);
        store.set_pet_active(chr, husky, true).unwrap();
        store.set_pet_name(husky, "Dummy").unwrap();
        assert_eq!(store.learn_pet_skill(husky, 1 << 1).unwrap(), PET_SKILLS_AT_START | (1 << 1));
        assert_eq!(store.learn_pet_skill(husky, 1 << 4).unwrap(), PET_SKILLS_AT_START | (1 << 1) | (1 << 4));
        assert_eq!(
            store.pet_state(husky).unwrap(),
            PetState { name: Some("Dummy".to_string()), skills: 0b1_1111, active: true, ..PetState::fresh() }
        );
        // The vitals ride the same row and leave the rest alone.
        store.set_pet_vitals(husky, 4, 7, 63).unwrap();
        let st = store.pet_state(husky).unwrap();
        assert_eq!((st.level, st.closeness, st.fullness), (4, 7, 63));
        assert_eq!((st.name.as_deref(), st.skills, st.active), (Some("Dummy"), 0b1_1111, true));
        assert_eq!(PET_SKILLS_AT_START, net::bag::PET_SKILLS_LEARNED_AT_START, "the two crates agree on the default");
    }

    /// **The live server has the 2026-09-15 table** - keyed by character + item id, with or
    /// without the vitals columns - and pets in bags with no number. Opening it must move
    /// every row onto `pets`, stamp the id onto the matching bag item, number the rest, and
    /// do nothing at all the second time.
    #[test]
    fn the_first_shipped_table_and_unnumbered_pets_come_across_on_open() {
        let (store, chr) = store_with_character();
        {
            let conn = store.conn();
            // The bag as the old server left it: two Huskies (one row of state between them,
            // the old key could not tell them apart) and a Dragon with no row at all.
            for (slot, item) in [(1, HUSKY), (2, HUSKY), (3, 5_000_003)] {
                conn.execute(
                    "INSERT INTO inventory (character_id, inv_type, slot, item_id, kind, quantity) VALUES (?1, 5, ?2, ?3, 2, 1)",
                    rusqlite::params![chr, slot, item],
                )
                .unwrap();
            }
            conn.execute("DROP TABLE IF EXISTS pets", []).unwrap();
            conn.execute_batch(
                "CREATE TABLE character_pets (
                    character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
                    item_id INTEGER NOT NULL, name TEXT,
                    skills INTEGER NOT NULL DEFAULT 1, active INTEGER NOT NULL DEFAULT 0,
                    PRIMARY KEY (character_id, item_id));",
            )
            .unwrap();
            conn.execute(
                "INSERT INTO character_pets (character_id, item_id, name, skills, active) VALUES (?1, 5000006, 'Dummy', 3, 1)",
                [chr],
            )
            .unwrap();
            create_tables(&conn).unwrap();
            create_tables(&conn).unwrap(); // idempotent
        }
        let bag = store.bag(chr).unwrap();
        let ids: Vec<Option<u32>> = bag.items_in(InventoryType::Cash).map(|i| i.item.pet_id).collect();
        assert_eq!(ids.len(), 3);
        assert!(ids.iter().all(|i| i.is_some()), "every pet is numbered: {ids:?}");
        let mut distinct = ids.clone();
        distinct.sort();
        distinct.dedup();
        assert_eq!(distinct.len(), 3, "three different numbers: {ids:?}");
        // The old row's state landed on the FIRST Husky; the second is fresh.
        let first = store.pet_state(ids[0].unwrap()).unwrap();
        assert_eq!((first.name.as_deref(), first.skills, first.active, first.fullness), (Some("Dummy"), 3 | PET_SKILLS_AT_START, true, 100));
        assert_eq!(store.pet_state(ids[1].unwrap()).unwrap(), PetState::fresh());
        assert_eq!(store.active_pet(chr).unwrap(), Some(ActivePetRow { pet_id: ids[0].unwrap(), item_id: HUSKY }));
        let old_gone: i64 = store.conn().query_row("SELECT COUNT(*) FROM sqlite_master WHERE name = 'character_pets'", [], |r| r.get(0)).unwrap();
        assert_eq!(old_gone, 0, "the old table is renamed away so the move cannot run twice");
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
        let mine = buy(&store, chr, HUSKY);
        let theirs = buy(&store, other, HUSKY);
        store.set_pet_name(mine, "Dummy").unwrap();
        store.set_pet_active(chr, mine, true).unwrap();
        assert_eq!(store.pet_state(theirs).unwrap().name, None);
        assert_eq!(store.active_pet(other).unwrap(), None);
        // A pet that changes hands is re-homed and arrives put away.
        let taken = store.remove_item(chr, InventoryType::Cash, 1, None).unwrap();
        assert_eq!(taken.pet_id, Some(mine));
        store.add_item(other, InventoryType::Cash, &taken, 1).unwrap();
        assert_eq!(store.active_pet(chr).unwrap(), None);
        assert_eq!(store.active_pet(other).unwrap(), None, "arrives put away");
        assert_eq!(store.pet_state(mine).unwrap().name.as_deref(), Some("Dummy"), "and keeps its name");
    }
}
