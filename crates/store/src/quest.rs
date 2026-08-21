//! Quest state persistence - the thing that makes an accepted quest survive a relog.
//!
//! Until this existed, accepting a quest changed nothing anywhere: the same conversation came
//! back every time and the journal never filled.
//!
//! # The state numbers are the client's, not ours
//!
//! [`QuestState::InProgress`] is `1` and [`QuestState::Complete`] is `2` because those are the values
//! `net::quest::QUEST_STATE_*` carries on the wire, read off `FUN_142d5b750`'s three-way
//! fork. One namespace rather than two means a mapping table cannot drift, and the mapping
//! is the kind of thing that fails silently.
//!
//! A row **exists** only for a quest the character has touched; "not started" is the absence
//! of a row rather than a stored `0`. That is why `net::quest::QUEST_STATE_NONE` has no
//! [`QuestState`] variant - storing it would make "never seen" and "explicitly forgotten"
//! two different values for the same thing.
//!
//! # Exhaustive destructure, the same discipline `character.rs` uses
//!
//! [`Store::save_quest`] destructures [`QuestRow`] field by field, so adding a field to the
//! row stops this file compiling until the column exists. A field left out of a hand-written
//! mapping is state that silently does not persist, which is the same class of bug as the
//! avatar-look field order.
//!
//! # Nothing here authenticates
//!
//! These take a character id and no account. The channel connection carries no credentials
//! at all - it is identified only by the migration row it claimed - so there is no account
//! here to check against, exactly as in [`Store::set_character_map`].

use rusqlite::{Connection, OptionalExtension};

use crate::db::Store;
use crate::error::Result;

/// Where a character is on one quest.
///
/// The discriminants are the wire's; see the module docs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum QuestState {
    /// Accepted, not yet turned in. Goes in the record's `presence[9]` block.
    InProgress = net::quest::QUEST_STATE_IN_PROGRESS,
    /// Turned in. Goes in the record's `presence[14]` block.
    Complete = net::quest::QUEST_STATE_COMPLETE,
}

impl QuestState {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// `None` for anything that is not a stored state - including
    /// `net::quest::QUEST_STATE_NONE`, which is represented by the absence of a row.
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            net::quest::QUEST_STATE_IN_PROGRESS => Some(QuestState::InProgress),
            net::quest::QUEST_STATE_COMPLETE => Some(QuestState::Complete),
            _ => None,
        }
    }
}

/// One row of `quest_state`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestRow {
    pub quest_id: u32,
    pub state: QuestState,
    /// The free-form progress string the record and the update packet both carry. Empty for
    /// a quest with nothing to count; see `net::quest::StartedQuest::progress`.
    pub progress: String,
    /// Unix seconds, the same clock `Store::now` uses. Converted to the client's FILETIME at
    /// the wire edge by `net::quest::filetime_from_unix_secs`, never before - a stored
    /// FILETIME would be a second time format in the database for no gain.
    pub started_at: i64,
    /// Unix seconds, `None` while the quest is still in progress.
    pub completed_at: Option<i64>,
}

/// Create the quest tables. Called from `Store::init`.
///
/// A plain `CREATE TABLE IF NOT EXISTS` is enough here **because the table is new**. That is
/// the opposite of [`Store::add_inventory_slot_columns`]'s situation: `IF NOT EXISTS` does
/// nothing at all to a table that already exists, so a *column* added to an existing table
/// needs a guarded `ALTER`. A whole new table has no such problem, and this note is here so
/// the next person adding a column to `quest_state` reaches for the right idiom.
pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        -- One row per quest the character has touched. "Not started" is the ABSENCE of a
        -- row, not a stored zero, so there is exactly one representation of it.
        --
        -- `state` holds the client's own numbers (net::quest::QUEST_STATE_*): 1 in progress,
        -- 2 complete. Same namespace as the wire, so no mapping table can drift.
        CREATE TABLE IF NOT EXISTS quest_state (
            character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            quest_id     INTEGER NOT NULL,
            state        INTEGER NOT NULL,
            progress     TEXT    NOT NULL DEFAULT '',
            started_at   INTEGER NOT NULL,
            completed_at INTEGER,
            PRIMARY KEY (character_id, quest_id)
        );

        CREATE INDEX IF NOT EXISTS idx_quest_state_character ON quest_state(character_id);
        "#,
    )?;
    Ok(())
}

impl Store {
    /// Every quest row for a character, quest id order.
    ///
    /// The order is ours and is only for determinism: the client keeps both collections in
    /// hash maps keyed by quest id, so nothing on screen depends on it - but a record whose
    /// bytes change between two identical loads is a nightmare to diff against a capture.
    pub fn quest_rows(&self, character_id: u32) -> Result<Vec<QuestRow>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT quest_id, state, progress, started_at, completed_at
               FROM quest_state
              WHERE character_id = ?1
              ORDER BY quest_id",
        )?;
        let rows = stmt.query_map([i64::from(character_id)], |row| {
            let state: u8 = row.get(1)?;
            Ok((
                QuestRow {
                    quest_id: row.get::<_, i64>(0)? as u32,
                    // A row whose state is neither 1 nor 2 cannot be represented, and it can
                    // only get there by hand-editing the database. Defaulting it to
                    // in-progress would silently resurrect a quest; it is dropped instead,
                    // and the caller sees a shorter list rather than a wrong one.
                    state: QuestState::InProgress,
                    progress: row.get(2)?,
                    started_at: row.get(3)?,
                    completed_at: row.get(4)?,
                },
                state,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (mut row, state) = row?;
            let Some(state) = QuestState::from_u8(state) else { continue };
            row.state = state;
            out.push(row);
        }
        Ok(out)
    }

    /// One quest's row, if the character has touched it.
    pub fn quest_row(&self, character_id: u32, quest_id: u32) -> Result<Option<QuestRow>> {
        let conn = self.conn();
        let row = conn
            .query_row(
                "SELECT state, progress, started_at, completed_at
                   FROM quest_state
                  WHERE character_id = ?1 AND quest_id = ?2",
                rusqlite::params![i64::from(character_id), i64::from(quest_id)],
                |row| {
                    Ok((
                        row.get::<_, u8>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, i64>(2)?,
                        row.get::<_, Option<i64>>(3)?,
                    ))
                },
            )
            .optional()?;
        let Some((state, progress, started_at, completed_at)) = row else { return Ok(None) };
        let Some(state) = QuestState::from_u8(state) else { return Ok(None) };
        Ok(Some(QuestRow { quest_id, state, progress, started_at, completed_at }))
    }

    /// Insert or replace one quest row.
    ///
    /// The destructure is exhaustive on purpose: add a field to [`QuestRow`] and this stops
    /// compiling until it has a column, rather than persisting nothing and looking fine.
    pub fn save_quest(&self, character_id: u32, row: &QuestRow) -> Result<()> {
        let QuestRow { quest_id, state, progress, started_at, completed_at } = row;
        self.conn().execute(
            "INSERT INTO quest_state
                 (character_id, quest_id, state, progress, started_at, completed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(character_id, quest_id) DO UPDATE SET
                 state        = excluded.state,
                 progress     = excluded.progress,
                 started_at   = excluded.started_at,
                 completed_at = excluded.completed_at",
            rusqlite::params![
                i64::from(character_id),
                i64::from(*quest_id),
                state.as_u8(),
                progress,
                started_at,
                completed_at,
            ],
        )?;
        Ok(())
    }

    /// Accept a quest. Returns `false` - and changes nothing - if the character already has
    /// it, in either state.
    ///
    /// **The refusal is the point.** Re-accepting an in-progress quest would reset its
    /// progress string, and re-accepting a completed one would put it back in the started
    /// list, which is exactly the "the same conversation every time" behaviour this whole
    /// feature exists to end. A caller that gets `false` should say something rather than
    /// silently re-sending an acceptance.
    pub fn start_quest(&self, character_id: u32, quest_id: u32) -> Result<bool> {
        let changed = self.conn().execute(
            "INSERT OR IGNORE INTO quest_state
                 (character_id, quest_id, state, progress, started_at, completed_at)
             VALUES (?1, ?2, ?3, '', ?4, NULL)",
            rusqlite::params![
                i64::from(character_id),
                i64::from(quest_id),
                QuestState::InProgress.as_u8(),
                Store::now(),
            ],
        )?;
        Ok(changed > 0)
    }

    /// Update a started quest's progress string. `false` if it is not in progress.
    pub fn set_quest_progress(
        &self,
        character_id: u32,
        quest_id: u32,
        progress: &str,
    ) -> Result<bool> {
        let changed = self.conn().execute(
            "UPDATE quest_state SET progress = ?3
              WHERE character_id = ?1 AND quest_id = ?2 AND state = ?4",
            rusqlite::params![
                i64::from(character_id),
                i64::from(quest_id),
                progress,
                QuestState::InProgress.as_u8(),
            ],
        )?;
        Ok(changed > 0)
    }

    /// Turn a quest in. Returns `false` unless it was in progress - a quest cannot be
    /// completed twice, and one that was never accepted cannot be completed at all.
    ///
    /// Returns the completion time on success so the caller can put it straight into
    /// `net::quest::quest_completed` without a second read.
    pub fn complete_quest(&self, character_id: u32, quest_id: u32) -> Result<Option<i64>> {
        let now = Store::now();
        let changed = self.conn().execute(
            "UPDATE quest_state SET state = ?3, completed_at = ?4
              WHERE character_id = ?1 AND quest_id = ?2 AND state = ?5",
            rusqlite::params![
                i64::from(character_id),
                i64::from(quest_id),
                QuestState::Complete.as_u8(),
                now,
                QuestState::InProgress.as_u8(),
            ],
        )?;
        Ok(if changed > 0 { Some(now) } else { None })
    }

    /// Forget a quest that is **in progress** - the row goes, so the character is back to
    /// never having accepted it. `false` if there was nothing to forget, *including* when
    /// the quest is already complete.
    ///
    /// **A completion is never deleted, and this used to delete it.** The forfeit handler's
    /// own documentation already said so - *"A forfeit undoes an acceptance. It must not
    /// silently wipe a completion the player earned"* - but that sentence was describing the
    /// wire flag it passes to `forfeit_reply`, and nothing enforced it here. The `DELETE` had
    /// no state predicate, so pressing give up on a finished quest removed the row, which put
    /// the character back to never having touched it: accept it again, take its `Act.0` items
    /// again, turn it in again for the experience. A full repeatable loop out of one missing
    /// `AND`.
    ///
    /// That is the shape `CLAUDE.md` keeps naming - a comment confidently describing
    /// behaviour the code does not have - and it is why the guard lives in the **store**
    /// rather than in the caller. Three separate call sites reached into quest state and only
    /// some of them checked it; the row is the authority, so the row refuses.
    ///
    /// The client is believed not to send a forfeit for a completed quest at all: its builder
    /// walks its own started map and returns without building anything if the id is not there.
    /// That is **[L]**, read off the listing, and it is exactly the kind of claim this server
    /// must not depend on.
    pub fn forget_quest(&self, character_id: u32, quest_id: u32) -> Result<bool> {
        let changed = self.conn().execute(
            "DELETE FROM quest_state
              WHERE character_id = ?1 AND quest_id = ?2 AND state = ?3",
            rusqlite::params![
                i64::from(character_id),
                i64::from(quest_id),
                QuestState::InProgress.as_u8(),
            ],
        )?;
        Ok(changed > 0)
    }

    /// The two record blocks' worth of state, ready for
    /// `net::opcode::character_record_for_set_field_with_quests`.
    ///
    /// The FILETIME conversion happens **here**, at the wire edge, rather than in the
    /// database: `net::quest::filetime_from_unix_secs`. A completed row with a null
    /// `completed_at` cannot happen through this module's API, and if one is hand-written it
    /// converts as the epoch rather than being dropped - a quest showing a wrong date is a
    /// far smaller problem than a quest silently vanishing from the journal.
    pub fn quest_book(&self, character_id: u32) -> Result<net::quest::QuestBook> {
        let mut book = net::quest::QuestBook::default();
        for row in self.quest_rows(character_id)? {
            match row.state {
                QuestState::InProgress => book.started.push(net::quest::StartedQuest {
                    quest_id: row.quest_id,
                    progress: row.progress,
                }),
                QuestState::Complete => book.completed.push(net::quest::CompletedQuest {
                    quest_id: row.quest_id,
                    completed_at: net::quest::filetime_from_unix_secs(
                        row.completed_at.unwrap_or(0),
                    ),
                }),
            }
        }
        Ok(book)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use net::opcode::Character;

    fn store_with_character() -> (Store, u32) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = store
            .create_character(account, 0, &Character { name: "Quester".into(), ..Character::default() })
            .unwrap();
        (store, chr.id)
    }

    /// The whole point: an accepted quest is still accepted after a reload.
    #[test]
    fn an_accepted_quest_survives_a_reload() {
        let (store, chr) = store_with_character();
        assert!(store.start_quest(chr, 1000).unwrap());

        let rows = store.quest_rows(chr).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].quest_id, 1000);
        assert_eq!(rows[0].state, QuestState::InProgress);
        assert_eq!(rows[0].progress, "");
        assert_eq!(rows[0].completed_at, None);

        let book = store.quest_book(chr).unwrap();
        assert_eq!(book.started.len(), 1);
        assert_eq!(book.started[0].quest_id, 1000);
        assert!(book.completed.is_empty());
    }

    /// Accepting twice is a no-op rather than a reset. This is the behaviour that stops the
    /// "same conversation every time" loop from coming back through a different door.
    #[test]
    fn a_quest_cannot_be_accepted_twice() {
        let (store, chr) = store_with_character();
        assert!(store.start_quest(chr, 1000).unwrap());
        store.set_quest_progress(chr, 1000, "007").unwrap();

        assert!(!store.start_quest(chr, 1000).unwrap(), "the second accept changes nothing");
        assert_eq!(store.quest_row(chr, 1000).unwrap().unwrap().progress, "007");

        store.complete_quest(chr, 1000).unwrap();
        assert!(!store.start_quest(chr, 1000).unwrap(), "nor does one after completion");
        assert_eq!(store.quest_row(chr, 1000).unwrap().unwrap().state, QuestState::Complete);
    }

    /// Completing moves the quest between the two record blocks, and it can only happen once.
    #[test]
    fn completing_moves_a_quest_from_started_to_completed() {
        let (store, chr) = store_with_character();
        store.start_quest(chr, 1000).unwrap();

        let at = store.complete_quest(chr, 1000).unwrap().expect("it was in progress");
        assert!(!book_has_started(&store, chr, 1000));
        let book = store.quest_book(chr).unwrap();
        assert_eq!(book.completed.len(), 1);
        assert_eq!(book.completed[0].quest_id, 1000);
        assert_eq!(
            book.completed[0].completed_at,
            net::quest::filetime_from_unix_secs(at),
            "the store keeps unix seconds; the FILETIME is made at the wire edge"
        );

        assert_eq!(store.complete_quest(chr, 1000).unwrap(), None, "not twice");
    }

    /// A quest that was never accepted cannot be completed. Without this the client could
    /// hand over any id and get a completion.
    #[test]
    fn a_quest_that_was_never_accepted_cannot_be_completed() {
        let (store, chr) = store_with_character();
        assert_eq!(store.complete_quest(chr, 1000).unwrap(), None);
        assert!(store.quest_rows(chr).unwrap().is_empty());
        assert!(!store.set_quest_progress(chr, 1000, "1").unwrap());
    }

    /// Progress only moves while a quest is in progress.
    #[test]
    fn progress_stops_being_writable_once_a_quest_is_done() {
        let (store, chr) = store_with_character();
        store.start_quest(chr, 1000).unwrap();
        assert!(store.set_quest_progress(chr, 1000, "003").unwrap());
        store.complete_quest(chr, 1000).unwrap();
        assert!(!store.set_quest_progress(chr, 1000, "999").unwrap());
        assert_eq!(store.quest_row(chr, 1000).unwrap().unwrap().progress, "003");
    }

    /// "Not started" is the absence of a row, so forgetting really removes it.
    #[test]
    fn forgetting_a_quest_removes_the_row_entirely() {
        let (store, chr) = store_with_character();
        store.start_quest(chr, 1000).unwrap();
        assert!(store.forget_quest(chr, 1000).unwrap());
        assert_eq!(store.quest_row(chr, 1000).unwrap(), None);
        assert!(store.quest_book(chr).unwrap().is_empty());
        assert!(!store.forget_quest(chr, 1000).unwrap(), "and there is nothing left to forget");
    }

    /// **A completed quest survives a forfeit**, which is the whole difference between
    /// giving up and undoing.
    ///
    /// The owner, 2026-08-21: *"I was able to complete the Heena quest multiple times, this is not
    /// okay."* The `DELETE` had no state predicate, so give-up on a finished quest removed the
    /// row and put the character back to never having touched it - accept, take `Act.0`, turn
    /// in for the experience, repeat. The handler above it already documented that this must
    /// not happen; the sentence was about a wire flag and nothing enforced the database half.
    #[test]
    fn forgetting_never_removes_a_completed_quest() {
        let (store, chr) = store_with_character();
        store.start_quest(chr, 1000).unwrap();
        store.complete_quest(chr, 1000).unwrap();

        assert!(
            !store.forget_quest(chr, 1000).unwrap(),
            "a completed quest is not a thing that can be given up"
        );
        assert_eq!(
            store.quest_row(chr, 1000).unwrap().unwrap().state,
            QuestState::Complete,
            "and the row is untouched"
        );
        assert_eq!(
            store.quest_book(chr).unwrap().completed.len(),
            1,
            "so the journal still shows it finished"
        );

        // The positive control, because a DELETE that refuses everything would pass the
        // assertions above while breaking the feature. Give-up must still work.
        store.start_quest(chr, 1001).unwrap();
        assert!(store.forget_quest(chr, 1001).unwrap(), "an in-progress quest still forfeits");
        assert_eq!(store.quest_row(chr, 1001).unwrap(), None);
    }

    /// Two characters do not share a journal.
    #[test]
    fn one_character_never_sees_anothers_quests() {
        let (store, first) = store_with_character();
        let account = store.create_account("other", "correct horse battery").unwrap();
        let second = store
            .create_character(account, 0, &Character { name: "Second".into(), ..Character::default() })
            .unwrap()
            .id;

        store.start_quest(first, 1000).unwrap();
        assert!(store.quest_book(second).unwrap().is_empty());
        assert_eq!(store.quest_book(first).unwrap().started.len(), 1);
    }

    /// Deleting a character takes its quests with it, the way it already takes its equipment.
    #[test]
    fn deleting_a_character_takes_its_quests() {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = store
            .create_character(account, 0, &Character { name: "Doomed".into(), ..Character::default() })
            .unwrap();
        store.start_quest(chr.id, 1000).unwrap();
        // Prove the row exists first - "no orphans" is also what a failed insert looks like.
        assert_eq!(quest_rows_total(&store), 1);

        store.delete_character(account, chr.id).unwrap();
        assert_eq!(quest_rows_total(&store), 0, "ON DELETE CASCADE should have taken them");
    }

    /// The book comes back in a stable order, so two identical loads produce identical bytes.
    #[test]
    fn the_book_is_ordered_by_quest_id() {
        let (store, chr) = store_with_character();
        for id in [1005u32, 1000, 1002] {
            store.start_quest(chr, id).unwrap();
        }
        let ids: Vec<u32> =
            store.quest_book(chr).unwrap().started.iter().map(|q| q.quest_id).collect();
        assert_eq!(ids, vec![1000, 1002, 1005]);
    }

    /// A hand-written row with a state the wire has no value for is dropped rather than
    /// guessed at. Defaulting it would resurrect a quest nobody accepted.
    #[test]
    fn a_row_with_an_unknown_state_is_dropped_not_guessed() {
        let (store, chr) = store_with_character();
        store.start_quest(chr, 1000).unwrap();
        store
            .conn()
            .execute(
                "INSERT INTO quest_state
                     (character_id, quest_id, state, progress, started_at, completed_at)
                 VALUES (?1, 1001, 7, '', 0, NULL)",
                [i64::from(chr)],
            )
            .unwrap();

        let rows = store.quest_rows(chr).unwrap();
        assert_eq!(rows.len(), 1, "only the representable one comes back");
        assert_eq!(rows[0].quest_id, 1000);
        assert_eq!(store.quest_row(chr, 1001).unwrap(), None);
    }

    /// The stored numbers are the wire's, so no mapping table exists to drift.
    #[test]
    fn the_stored_state_numbers_are_the_clients_own() {
        assert_eq!(QuestState::InProgress.as_u8(), net::quest::QUEST_STATE_IN_PROGRESS);
        assert_eq!(QuestState::Complete.as_u8(), net::quest::QUEST_STATE_COMPLETE);
        assert_eq!(
            QuestState::from_u8(net::quest::QUEST_STATE_NONE),
            None,
            "\"not started\" is the absence of a row, not a stored value"
        );
    }

    fn book_has_started(store: &Store, chr: u32, quest_id: u32) -> bool {
        store.quest_book(chr).unwrap().started.iter().any(|q| q.quest_id == quest_id)
    }

    fn quest_rows_total(store: &Store) -> i64 {
        store
            .conn()
            .query_row("SELECT COUNT(*) FROM quest_state", [], |row| row.get(0))
            .unwrap()
    }
}
