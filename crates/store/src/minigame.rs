//! **Mini-game records** - wins, ties, losses and points, per character and per game.
//!
//! The owner, 2026-10-04: *"the minigame UI keeps track of how many win/lose/ties the player
//! has, make sure that is properly handled as well"*, and *"The win/lose/tie record is stored
//! per game"* - an Omok record and a Match Cards record, kept apart.
//!
//! The client draws the record in its side panel as PTS over W / L / D (the baked labels of
//! `UI/Minigame.img/Common/score`), from the 20 bytes `net::minigame::Record` carries.
//!
//! **Points are this server's rule**, not read off anything: a new record starts at
//! [`POINTS_START`], a win adds [`POINTS_STEP`], a loss takes it away (never below zero), a tie
//! leaves it.

use rusqlite::{Connection, OptionalExtension};

use crate::db::Store;
use crate::error::Result;

/// A record nobody has played yet.
pub const POINTS_START: u32 = 2000;
/// What a win adds and a loss takes.
pub const POINTS_STEP: u32 = 10;

/// One character's record at one game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiniGameRecord {
    pub wins: u32,
    pub ties: u32,
    pub losses: u32,
    pub points: u32,
}

impl Default for MiniGameRecord {
    fn default() -> Self {
        Self { wins: 0, ties: 0, losses: 0, points: POINTS_START }
    }
}

/// How one game ended for one player.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Tie,
    Loss,
}

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        -- `game` is the miniroom type: 3 Omok, 4 Match Cards.
        CREATE TABLE IF NOT EXISTS minigame_records (
            character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            game         INTEGER NOT NULL,
            wins         INTEGER NOT NULL DEFAULT 0,
            ties         INTEGER NOT NULL DEFAULT 0,
            losses       INTEGER NOT NULL DEFAULT 0,
            points       INTEGER NOT NULL,
            PRIMARY KEY (character_id, game),
            CHECK (wins >= 0 AND ties >= 0 AND losses >= 0 AND points >= 0)
        );
        "#,
    )?;
    Ok(())
}

fn read(conn: &Connection, character_id: u32, game: u32) -> Result<MiniGameRecord> {
    Ok(conn
        .query_row(
            "SELECT wins, ties, losses, points FROM minigame_records WHERE character_id = ?1 AND game = ?2",
            [i64::from(character_id), i64::from(game)],
            |r| {
                Ok(MiniGameRecord {
                    wins: r.get::<_, i64>(0)? as u32,
                    ties: r.get::<_, i64>(1)? as u32,
                    losses: r.get::<_, i64>(2)? as u32,
                    points: r.get::<_, i64>(3)? as u32,
                })
            },
        )
        .optional()?
        .unwrap_or_default())
}

impl Store {
    /// `character_id`'s record at `game`, or a fresh one.
    pub fn minigame_record(&self, character_id: u32, game: u32) -> Result<MiniGameRecord> {
        read(&self.conn(), character_id, game)
    }

    /// Count one finished game for `character_id` at `game`. Returns the record after it.
    pub fn record_minigame_result(&self, character_id: u32, game: u32, outcome: Outcome) -> Result<MiniGameRecord> {
        let conn = self.conn();
        let mut r = read(&conn, character_id, game)?;
        match outcome {
            Outcome::Win => {
                r.wins += 1;
                r.points += POINTS_STEP;
            }
            Outcome::Tie => r.ties += 1,
            Outcome::Loss => {
                r.losses += 1;
                r.points = r.points.saturating_sub(POINTS_STEP);
            }
        }
        conn.execute(
            "INSERT INTO minigame_records (character_id, game, wins, ties, losses, points) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(character_id, game) DO UPDATE SET wins = excluded.wins, ties = excluded.ties,
             losses = excluded.losses, points = excluded.points",
            rusqlite::params![
                i64::from(character_id),
                i64::from(game),
                i64::from(r.wins),
                i64::from(r.ties),
                i64::from(r.losses),
                i64::from(r.points)
            ],
        )?;
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_game_keeps_its_own_record() {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("pebble", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Pebble".into(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        assert_eq!(store.minigame_record(id, 3).unwrap(), MiniGameRecord::default(), "fresh: 0/0/0, 2000 points");
        store.record_minigame_result(id, 3, Outcome::Win).unwrap();
        store.record_minigame_result(id, 3, Outcome::Tie).unwrap();
        let omok = store.record_minigame_result(id, 3, Outcome::Loss).unwrap();
        assert_eq!(omok, MiniGameRecord { wins: 1, ties: 1, losses: 1, points: POINTS_START });
        assert_eq!(store.minigame_record(id, 3).unwrap(), omok, "it was written");
        assert_eq!(store.minigame_record(id, 4).unwrap(), MiniGameRecord::default(), "Match Cards is untouched");
        let cards = store.record_minigame_result(id, 4, Outcome::Win).unwrap();
        assert_eq!((cards.wins, cards.points), (1, POINTS_START + POINTS_STEP));
    }
}
