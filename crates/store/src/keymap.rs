//! The per-character key layout: what `0x0199` saved, so `0x05F1` can send it back.
//!
//! The owner, 2026-09-08: the layout survives a logout and re-login inside one client, and is back
//! at factory bindings on a fresh one - because nothing was ever stored. This is the store.
//!
//! # One row per BOUND key, not a blob of 89 slots
//!
//! The client sends a **delta** (`net::keymap`, and `research/keyboard-layout-2026-09-08.md`
//! §2) and expects the **full table** back, so something has to hold all 89 slots. It could
//! be a 445-byte blob on the character row. Rows win, for the same reason
//! `character_skill_spend` is per skill rather than per pool:
//!
//! * A delta is an **upsert of the keys it names** and nothing else. With a blob, applying a
//!   3-key delta means read-modify-write of the whole table, which is a lost update the
//!   moment two connections do it at once - and two clients on one account is a thing this
//!   project supports.
//! * "This character has never saved a layout" has exactly one representation: no rows. That
//!   distinction is load-bearing, because it decides whether `0x05F1` goes out with the
//!   **read** gate or the **keep** gate, and getting that backwards unbinds a keyboard.
//! * A key that is bound to nothing is simply absent, so the table never stores 86 empty
//!   slots to describe three real ones.
//!
//! The two `u32` options behind `0x05F2`/`0x05F3` are a separate one-row-per-character table
//! rather than columns on `characters`, so this whole feature adds nothing to a row that is
//! already read on every login.
//!
//! # One row per (table, key): the controller is table 3 - 2026-09-14
//!
//! The owner: *"Whenever there are customization to keybindings in the controller settings, it is
//! not getting saved properly, and when clients switch maps, their controller settings are
//! completely screwed up."* The client keeps FOUR tables - keyboard presets 0..2 and the
//! controller - and names the table in every delta (`net::keymap` module docs §4). This
//! module stored one table, so a controller delta overwrote keyboard rows at the same
//! numbers. [`KeyBinding::preset`] is that table index now, and the primary key includes it.
//!
//! The deployed table has no `preset` column and SQLite cannot add one to a primary key, so
//! [`ensure_tables`] rebuilds it in place the first time it sees the old shape - every old
//! row becomes preset 0 - and then **scrubs the rows the bug left behind**: a preset-0 row
//! that is byte-for-byte a bound slot of the controller's factory table is a controller
//! button that landed on a scan code, not a keyboard binding anyone made (the odds of a
//! player hand-binding scan code 1 to basic action 401 are what they are). Those rows are
//! deleted, not moved: a controller row equal to the controller factory is redundant anyway.
//! What cannot be repaired is a keyboard row the controller delta overwrote - the owner's LCtrl
//! and A were, on 2026-09-14 - and the scrub puts those back to factory; they re-binds them once.
//!
//! # What this crate deliberately does not know
//!
//! It does not merge in defaults. It stores what the player changed and hands it back;
//! `net::keymap::restore` decides what is safe to put on the wire, and the inverted gate byte
//! lives in exactly one place. The one factory table this module reads is the controller's,
//! for the scrub above, and only on the upgrade path.
//!
//! # Nothing here authenticates
//!
//! The channel socket carries no credentials. A layout is stored on the say-so of whoever
//! holds the connection.

use rusqlite::{Connection, OptionalExtension};

use crate::db::Store;
use crate::error::Result;

/// One saved binding: which table, a slot in it, and what sits on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyBinding {
    /// Which of the client's four tables: keyboard preset 0, 1 or 2, or
    /// `net::keymap::CONTROLLER_TABLE` (3). The caller has already bounded it.
    pub preset: u8,
    /// DirectInput scan code for a keyboard table, a button index for the controller's. The
    /// caller has already bounded this against the table size; this crate stores what it is
    /// given.
    pub key: u8,
    pub kind: u8,
    pub action: u32,
}

/// Which of the two `u32` options a row holds.
///
/// Stored as a small integer rather than two columns so a third option - the client has
/// several more `0x0199` subtypes than we have seen fire - costs a row and not a migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeymapOption {
    /// Restored by `net::keymap::KEYMAP_OPT_A` (`0x05F2`).
    A = 0,
    /// Restored by `net::keymap::KEYMAP_OPT_B` (`0x05F3`).
    B = 1,
}

/// Create both tables, and bring a deployed `character_keymap` up to the (table, key) shape.
///
/// `character_keymap_option` is a plain `CREATE TABLE IF NOT EXISTS`. `character_keymap`
/// shipped on 2026-09-12 keyed by (character_id, key) and is rebuilt here when that shape is
/// found - module docs. Called per query, like `claims::ensure_columns`, so the unwired state
/// is impossible rather than fatal; the check is one `PRAGMA` once the column exists.
pub(crate) fn ensure_tables(conn: &Connection) -> Result<()> {
    upgrade_to_four_tables(conn)?;
    conn.execute_batch(
        r#"
        -- One row per BOUND slot of one table. No rows at all means "this character has
        -- never saved a layout", which is a different thing from "saved an empty one" and
        -- decides whether 0x05F1 goes out at all.
        CREATE TABLE IF NOT EXISTS character_keymap (
            character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            preset       INTEGER NOT NULL DEFAULT 0,
            key          INTEGER NOT NULL,
            kind         INTEGER NOT NULL,
            action       INTEGER NOT NULL,
            PRIMARY KEY (character_id, preset, key)
        );

        -- The two u32s behind 0x05F2 and 0x05F3, one row each.
        CREATE TABLE IF NOT EXISTS character_keymap_option (
            character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            option       INTEGER NOT NULL,
            value        INTEGER NOT NULL,
            PRIMARY KEY (character_id, option)
        );
        "#,
    )?;
    Ok(())
}

/// The 2026-09-12 shape had no `preset` column. Rebuild it with one - every existing row is
/// preset 0, which is the only table the old code could have been fed - and scrub the
/// controller buttons the old code filed under keyboard scan codes. Module docs.
///
/// One transaction: a half-rebuilt table is worse than the old one. Returns how many rows
/// the scrub removed, `0` when there was nothing to upgrade.
pub(crate) fn upgrade_to_four_tables(conn: &Connection) -> Result<usize> {
    let mut columns = std::collections::HashSet::new();
    {
        let mut stmt = conn.prepare("PRAGMA table_info(character_keymap)")?;
        for name in stmt.query_map([], |row| row.get::<_, String>(1))? {
            columns.insert(name?);
        }
    }
    // No table yet, or already the new shape: nothing to do.
    if columns.is_empty() || columns.contains("preset") {
        return Ok(0);
    }
    conn.execute_batch(
        r#"
        BEGIN;
        CREATE TABLE character_keymap_v2 (
            character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            preset       INTEGER NOT NULL DEFAULT 0,
            key          INTEGER NOT NULL,
            kind         INTEGER NOT NULL,
            action       INTEGER NOT NULL,
            PRIMARY KEY (character_id, preset, key)
        );
        INSERT INTO character_keymap_v2 (character_id, preset, key, kind, action)
            SELECT character_id, 0, key, kind, action FROM character_keymap;
        DROP TABLE character_keymap;
        ALTER TABLE character_keymap_v2 RENAME TO character_keymap;
        COMMIT;
        "#,
    )?;
    let mut scrubbed = 0;
    for (key, slot) in net::keymap::CLIENT_CONTROLLER_LAYOUT.iter().enumerate() {
        if slot.kind == 0 && slot.action == 0 {
            continue;
        }
        scrubbed += conn.execute(
            "DELETE FROM character_keymap
             WHERE preset = 0 AND key = ?1 AND kind = ?2 AND action = ?3",
            rusqlite::params![key as i64, slot.kind as i64, slot.action as i64],
        )?;
    }
    Ok(scrubbed)
}

impl Store {
    /// Every binding this character has saved, every table, ordered by table then slot.
    ///
    /// An empty vector means nothing has ever been saved. The caller must treat that as
    /// "leave the client alone", not as "send an empty layout".
    pub fn keymap(&self, character_id: u32) -> Result<Vec<KeyBinding>> {
        let conn = self.conn();
        ensure_tables(&conn)?;
        let mut stmt = conn.prepare(
            "SELECT preset, key, kind, action FROM character_keymap
             WHERE character_id = ?1 ORDER BY preset, key",
        )?;
        let rows = stmt.query_map([character_id], |r| {
            Ok(KeyBinding {
                preset: r.get::<_, i64>(0)? as u8,
                key: r.get::<_, i64>(1)? as u8,
                kind: r.get::<_, i64>(2)? as u8,
                action: r.get::<_, i64>(3)? as u32,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    /// Apply a delta. Returns how many rows were written or removed.
    ///
    /// **One transaction**, because a half-applied layout is the shape this repo has been
    /// caught by before: the player sees part of their change survive and cannot tell which
    /// part did not.
    ///
    /// A binding whose `kind` and `action` are both zero is the client's own "unbound" slot
    /// (`FUN_1401de960` writes exactly that), so it **deletes** the row rather than storing a
    /// row that means nothing. Otherwise clearing a key would leave it bound forever.
    pub fn apply_keymap_delta(&self, character_id: u32, bindings: &[KeyBinding]) -> Result<usize> {
        let mut conn = self.conn();
        ensure_tables(&conn)?;
        let tx = conn.transaction()?;
        let mut n = 0;
        for b in bindings {
            if b.kind == 0 && b.action == 0 {
                n += tx.execute(
                    "DELETE FROM character_keymap
                     WHERE character_id = ?1 AND preset = ?2 AND key = ?3",
                    rusqlite::params![character_id, b.preset as i64, b.key as i64],
                )?;
            } else {
                n += tx.execute(
                    "INSERT INTO character_keymap (character_id, preset, key, kind, action)
                     VALUES (?1, ?2, ?3, ?4, ?5)
                     ON CONFLICT(character_id, preset, key)
                     DO UPDATE SET kind = excluded.kind, action = excluded.action",
                    rusqlite::params![
                        character_id,
                        b.preset as i64,
                        b.key as i64,
                        b.kind as i64,
                        b.action as i64
                    ],
                )?;
            }
        }
        tx.commit()?;
        Ok(n)
    }

    /// Forget this character's layout entirely. The next login sends the keep gate.
    pub fn clear_keymap(&self, character_id: u32) -> Result<usize> {
        let conn = self.conn();
        ensure_tables(&conn)?;
        Ok(conn.execute(
            "DELETE FROM character_keymap WHERE character_id = ?1",
            [character_id],
        )?)
    }

    /// One of the two `u32` options, or `None` if it has never been saved.
    pub fn keymap_option(&self, character_id: u32, option: KeymapOption) -> Result<Option<u32>> {
        let conn = self.conn();
        ensure_tables(&conn)?;
        Ok(conn
            .query_row(
                "SELECT value FROM character_keymap_option
                 WHERE character_id = ?1 AND option = ?2",
                rusqlite::params![character_id, option as i64],
                |r| r.get::<_, i64>(0),
            )
            .optional()?
            .map(|v| v as u32))
    }

    pub fn set_keymap_option(
        &self,
        character_id: u32,
        option: KeymapOption,
        value: u32,
    ) -> Result<()> {
        let conn = self.conn();
        ensure_tables(&conn)?;
        conn.execute(
            "INSERT INTO character_keymap_option (character_id, option, value)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(character_id, option) DO UPDATE SET value = excluded.value",
            rusqlite::params![character_id, option as i64, value as i64],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store_with_character() -> (Store, u32) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Cobalt".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        (store, id)
    }

    /// The three bindings from the owner's live client, as `net::keymap` decoded them.
    fn remys_three() -> Vec<KeyBinding> {
        vec![
            KeyBinding { preset: 0, key: 0x1D, kind: 1, action: 1_001_002 }, // LCtrl  Slash Blast
            KeyBinding { preset: 0, key: 0x1E, kind: 1, action: 1_001_000 }, // A      Iron Body
            KeyBinding { preset: 0, key: 0x2A, kind: 1, action: 1_001_001 }, // LShift Power Strike
        ]
    }

    /// The distinction the restore gate turns on: never saved is not the same as saved empty.
    #[test]
    fn a_character_that_has_never_saved_has_no_rows() {
        let (store, chr) = store_with_character();
        assert_eq!(store.keymap(chr).unwrap(), Vec::new());
    }

    #[test]
    fn a_delta_round_trips_and_comes_back_ordered_by_scan_code() {
        let (store, chr) = store_with_character();
        assert_eq!(store.apply_keymap_delta(chr, &remys_three()).unwrap(), 3);
        assert_eq!(store.keymap(chr).unwrap(), remys_three());
    }

    /// The reason rows beat a blob: a second delta must not disturb the first one's keys.
    #[test]
    fn a_later_delta_merges_instead_of_replacing() {
        let (store, chr) = store_with_character();
        store.apply_keymap_delta(chr, &remys_three()).unwrap();
        store
            .apply_keymap_delta(chr, &[KeyBinding { preset: 0, key: 0x10, kind: 4, action: 77 }])
            .unwrap();
        let got = store.keymap(chr).unwrap();
        assert_eq!(got.len(), 4, "the three earlier bindings survived: {got:?}");
        assert!(got.contains(&KeyBinding { preset: 0, key: 0x10, kind: 4, action: 77 }));
        assert!(got.contains(&KeyBinding { preset: 0, key: 0x1D, kind: 1, action: 1_001_002 }));
    }

    #[test]
    fn rebinding_one_key_overwrites_only_that_key() {
        let (store, chr) = store_with_character();
        store.apply_keymap_delta(chr, &remys_three()).unwrap();
        store
            .apply_keymap_delta(chr, &[KeyBinding { preset: 0, key: 0x1D, kind: 1, action: 1_001_000 }])
            .unwrap();
        let got = store.keymap(chr).unwrap();
        assert_eq!(got.len(), 3);
        assert_eq!(got[0], KeyBinding { preset: 0, key: 0x1D, kind: 1, action: 1_001_000 });
        assert_eq!(got[2], KeyBinding { preset: 0, key: 0x2A, kind: 1, action: 1_001_001 });
    }

    /// The client's own clear-to-zero. Without this a key could never be UNbound again: the
    /// row would sit there forever and every login would re-bind it.
    #[test]
    fn clearing_a_key_removes_its_row_rather_than_storing_an_empty_one() {
        let (store, chr) = store_with_character();
        store.apply_keymap_delta(chr, &remys_three()).unwrap();
        store
            .apply_keymap_delta(chr, &[KeyBinding { preset: 0, key: 0x1E, kind: 0, action: 0 }])
            .unwrap();
        let got = store.keymap(chr).unwrap();
        assert_eq!(got.len(), 2, "the cleared key left no row behind: {got:?}");
        assert!(got.iter().all(|b| b.key != 0x1E));
    }

    #[test]
    fn two_characters_do_not_share_a_layout() {
        let (store, chr) = store_with_character();
        let other = store
            .create_character(
                store.create_account("someone", "else entirely here").unwrap(),
                0,
                &net::opcode::Character { name: "Other".to_string(), ..Default::default() },
            )
            .unwrap()
            .id;
        store.apply_keymap_delta(chr, &remys_three()).unwrap();
        assert_eq!(store.keymap(other).unwrap(), Vec::new());
    }

    #[test]
    fn clearing_a_layout_returns_the_character_to_never_saved() {
        let (store, chr) = store_with_character();
        store.apply_keymap_delta(chr, &remys_three()).unwrap();
        assert_eq!(store.clear_keymap(chr).unwrap(), 3);
        assert_eq!(store.keymap(chr).unwrap(), Vec::new());
    }

    /// The bug of 2026-09-14, in one test: a controller button and a keyboard key with the
    /// same number are two rows, and clearing one leaves the other.
    #[test]
    fn the_controller_table_and_the_keyboard_table_do_not_share_a_slot() {
        let (store, chr) = store_with_character();
        store.apply_keymap_delta(chr, &remys_three()).unwrap();
        // Button 0x1D on the controller -> some skill. Scan code 0x1D is LCtrl.
        let button = KeyBinding { preset: 3, key: 0x1D, kind: 1, action: 1000 };
        assert_eq!(store.apply_keymap_delta(chr, &[button]).unwrap(), 1);
        let got = store.keymap(chr).unwrap();
        assert_eq!(got.len(), 4, "four rows, not three: {got:?}");
        assert!(got.contains(&KeyBinding { preset: 0, key: 0x1D, kind: 1, action: 1_001_002 }), "LCtrl kept Slash Blast");
        assert_eq!(got[3], button, "ordered by table, the controller's row is last");
        // Unbinding the button does not touch LCtrl.
        store.apply_keymap_delta(chr, &[KeyBinding { preset: 3, key: 0x1D, kind: 0, action: 0 }]).unwrap();
        assert_eq!(store.keymap(chr).unwrap(), remys_three());
    }

    /// The deployed table, keyed by (character_id, key), with exactly what the bug left in
    /// The owner's: their keyboard rows, plus controller buttons filed under scan codes. After the
    /// upgrade the keyboard rows are preset 0 and the controller's are gone.
    #[test]
    fn the_old_one_table_shape_is_rebuilt_and_the_stray_controller_rows_scrubbed() {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Cobalt".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        {
            let conn = store.conn();
            conn.execute_batch(
                "CREATE TABLE character_keymap (
                    character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
                    key INTEGER NOT NULL, kind INTEGER NOT NULL, action INTEGER NOT NULL,
                    PRIMARY KEY (character_id, key));",
            )
            .unwrap();
            for (key, kind, action) in [
                (0x2Ai64, 1i64, 1_001_001i64), // LShift Power Strike: a real keyboard row
                (0x34, 5, 52),                 // '.' basic 52: a real keyboard row
                (0x00, 5, 53),                 // controller button 0, filed under scan code 0
                (0x01, 4, 401),                // button 1
                (0x1D, 4, 9),                  // button 0x1D - which overwrote LCtrl's Slash Blast
                (0x08, 1, 1000),               // button 8 -> skill 1000: the owner's own controller binding
            ] {
                conn.execute(
                    "INSERT INTO character_keymap (character_id, key, kind, action) VALUES (?1, ?2, ?3, ?4)",
                    rusqlite::params![id, key, kind, action],
                )
                .unwrap();
            }
            assert_eq!(upgrade_to_four_tables(&conn).unwrap(), 3, "three factory controller buttons scrubbed");
            assert_eq!(upgrade_to_four_tables(&conn).unwrap(), 0, "and the second call finds the new shape");
        }
        let got = store.keymap(id).unwrap();
        assert_eq!(
            got,
            vec![
                KeyBinding { preset: 0, key: 0x08, kind: 1, action: 1000 },
                KeyBinding { preset: 0, key: 0x2A, kind: 1, action: 1_001_001 },
                KeyBinding { preset: 0, key: 0x34, kind: 5, action: 52 },
            ],
            "the two keyboard rows survive as preset 0; button 8's skill cannot be told from a \
             keyboard binding and stays - on scan code 8, which is the '7' key - and LCtrl is \
             simply gone, back to factory"
        );
        // And the rebuilt table takes a controller row beside a keyboard one.
        store.apply_keymap_delta(id, &[KeyBinding { preset: 3, key: 0x08, kind: 1, action: 1000 }]).unwrap();
        assert_eq!(store.keymap(id).unwrap().len(), 4);
    }

    #[test]
    fn the_two_options_are_separate_and_survive_an_update() {
        let (store, chr) = store_with_character();
        assert_eq!(store.keymap_option(chr, KeymapOption::A).unwrap(), None);
        store.set_keymap_option(chr, KeymapOption::A, 11).unwrap();
        store.set_keymap_option(chr, KeymapOption::B, 22).unwrap();
        store.set_keymap_option(chr, KeymapOption::A, 33).unwrap();
        assert_eq!(store.keymap_option(chr, KeymapOption::A).unwrap(), Some(33));
        assert_eq!(store.keymap_option(chr, KeymapOption::B).unwrap(), Some(22));
    }
}
