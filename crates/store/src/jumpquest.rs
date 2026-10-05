//! **The jump quests' pity timer** - one row per character on a course, counting the time spent
//! on it.
//!
//! The owner, 2026-10-04: *"for all of the jump quests, start a 1 hour timer (per player), this
//! is the pity timer. When the player has expended all 1 hour of it, a yellow notice text in chat
//! will remind them every 5 minute that they have spent over an hour on this jump quest,
//! `!skipjq` will become available to them"*.
//!
//! **A table and not session memory**, because each channel is its own process: a channel change
//! or a reconnect would otherwise hand back a fresh hour. `spent_secs` only grows while the
//! character is on the course and online (`world::session::jumpquest` adds to it as it goes), so
//! time logged out is not spent. `notices` is how many reminders have been said, so a reconnect
//! does not repeat one. The rules about when are `world::jumpquest`'s; this only keeps the count.

use rusqlite::{params, Connection, OptionalExtension};

use crate::db::Store;
use crate::error::Result;

/// One character's run: which course (named by the NPC at its end), how long they have spent
/// on it, and how many reminders they have had.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JumpQuestRun {
    pub goal_npc: u32,
    pub spent_secs: u64,
    pub notices: u32,
}

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS jump_quest_runs (
            character_id INTEGER PRIMARY KEY,
            goal_npc     INTEGER NOT NULL,
            spent_secs   INTEGER NOT NULL DEFAULT 0,
            notices      INTEGER NOT NULL DEFAULT 0,
            started_at   INTEGER NOT NULL
        );
        "#,
    )?;
    Ok(())
}

impl Store {
    /// A fresh run on the course ending at `goal_npc`: no time spent, no reminders. Replaces
    /// whatever run the character had - one course at a time.
    pub fn start_jump_quest(&self, character_id: u32, goal_npc: u32) -> Result<()> {
        self.conn().execute(
            "INSERT OR REPLACE INTO jump_quest_runs (character_id, goal_npc, spent_secs, notices, started_at)
             VALUES (?1, ?2, 0, 0, ?3)",
            params![i64::from(character_id), i64::from(goal_npc), Store::unix_now()],
        )?;
        Ok(())
    }

    pub fn jump_quest_run(&self, character_id: u32) -> Result<Option<JumpQuestRun>> {
        Ok(self
            .conn()
            .query_row(
                "SELECT goal_npc, spent_secs, notices FROM jump_quest_runs WHERE character_id = ?1",
                params![i64::from(character_id)],
                |r| {
                    Ok(JumpQuestRun {
                        goal_npc: r.get::<_, i64>(0)? as u32,
                        spent_secs: r.get::<_, i64>(1)?.max(0) as u64,
                        notices: r.get::<_, i64>(2)?.max(0) as u32,
                    })
                },
            )
            .optional()?)
    }

    /// `secs` more on the course, and the run as it now stands. `None` - and nothing written -
    /// when the character has no run.
    pub fn add_jump_quest_time(&self, character_id: u32, secs: u64) -> Result<Option<JumpQuestRun>> {
        let changed = self.conn().execute(
            "UPDATE jump_quest_runs SET spent_secs = spent_secs + ?2 WHERE character_id = ?1",
            params![i64::from(character_id), i64::try_from(secs).unwrap_or(i64::MAX)],
        )?;
        if changed == 0 {
            return Ok(None);
        }
        self.jump_quest_run(character_id)
    }

    pub fn set_jump_quest_notices(&self, character_id: u32, notices: u32) -> Result<()> {
        self.conn().execute(
            "UPDATE jump_quest_runs SET notices = ?2 WHERE character_id = ?1",
            params![i64::from(character_id), i64::from(notices)],
        )?;
        Ok(())
    }

    /// The run is over - finished, left, skipped. `true` when there was one.
    pub fn end_jump_quest(&self, character_id: u32) -> Result<bool> {
        Ok(self.conn().execute("DELETE FROM jump_quest_runs WHERE character_id = ?1", params![i64::from(character_id)])? > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A run starts at zero, grows, remembers its reminders, is replaced by the next start and
    /// is gone when it ends; time added to no run writes nothing.
    #[test]
    fn a_run_counts_up_and_ends() {
        let store = Store::open_in_memory().unwrap();
        assert_eq!(store.add_jump_quest_time(200, 30).unwrap(), None, "no run, nothing written");
        assert_eq!(store.jump_quest_run(200).unwrap(), None);

        store.start_jump_quest(200, 611).unwrap();
        assert_eq!(store.jump_quest_run(200).unwrap(), Some(JumpQuestRun { goal_npc: 611, spent_secs: 0, notices: 0 }));
        store.add_jump_quest_time(200, 3_000).unwrap();
        let run = store.add_jump_quest_time(200, 700).unwrap().unwrap();
        assert_eq!(run.spent_secs, 3_700);
        store.set_jump_quest_notices(200, 2).unwrap();
        assert_eq!(store.jump_quest_run(200).unwrap().unwrap().notices, 2);
        assert_eq!(store.jump_quest_run(201).unwrap(), None, "per character");

        store.start_jump_quest(200, 420).unwrap();
        assert_eq!(store.jump_quest_run(200).unwrap(), Some(JumpQuestRun { goal_npc: 420, spent_secs: 0, notices: 0 }), "a new course starts afresh");
        assert!(store.end_jump_quest(200).unwrap());
        assert!(!store.end_jump_quest(200).unwrap());
        assert_eq!(store.jump_quest_run(200).unwrap(), None);
    }
}
