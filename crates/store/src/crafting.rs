//! What each character has learnt of the six crafting professions.
//!
//! The owner, 2026-09-21: *"We need to implement crafting in our server."*
//! `research/crafting-2026-09-21.md` is the decode.
//!
//! # Why this is its own table rather than six rows in `character_skills`
//!
//! A profession reaches the client as an ordinary skill - `92000000`..`92050000` - but its
//! `level` field on the wire is **`(level << 24) | masteryExp`**
//! ([`net::craft::packed_mastery`]). Writing that number into `character_skills.level` would
//! make every existing reader wrong at once: [`crate::Store::skill_level`] would answer
//! 16 777 216 for a level-1 Smith, and the skill-point spend would happily "raise" it.
//!
//! So the level and the mastery are kept as two honest integers here, and the packing
//! happens in exactly one place: [`crate::Store::skills`], which is what builds the record
//! and the `0x0081` list. Nothing else has to know.
//!
//! # A profession is learnt at level 1, and never unlearnt
//!
//! The six starter quests grant it (`Act.1.skill.0.id`), and the tab unlocks on the skill
//! being at level >= 1 - there is no other flag. A row here therefore means "this tab is
//! open"; no row means it is greyed out.

use rusqlite::Connection;

use crate::{Result, Store};

/// One profession a character has learnt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Profession {
    /// 0..5, the client's own tab order - `net::craft::PROFESSION_SKILLS`.
    pub profession: u8,
    /// 1..10.
    pub level: u32,
    /// Mastery towards the next level. Never more than 24 bits reach the wire.
    pub exp: u32,
}

impl Profession {
    /// The skill id this profession is, for the record and `0x0081`.
    pub fn skill_id(&self) -> u32 {
        net::craft::profession_skill(self.profession).unwrap_or(0)
    }

    /// The `level` field as the client wants it: level in the top byte, mastery below.
    pub fn packed_level(&self) -> u32 {
        net::craft::packed_mastery(self.level, self.exp)
    }
}

/// What one grant of mastery did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MasteryGain {
    /// The level after the grant.
    pub level: u32,
    /// The mastery after the grant.
    pub exp: u32,
    /// How many levels it crossed. 0 for the ordinary case.
    pub levels_gained: u32,
    /// Mastery that could not be kept because the character level caps the profession -
    /// the bar parks at 100% and the overflow is discarded.
    pub wasted: u32,
}

/// Create the table. Called from the schema on every open, so it must be idempotent.
pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS character_crafting (
            character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            profession   INTEGER NOT NULL,
            level        INTEGER NOT NULL,
            exp          INTEGER NOT NULL DEFAULT 0,
            PRIMARY KEY (character_id, profession)
        );
        "#,
    )?;
    Ok(())
}

impl Store {
    /// Every profession this character has learnt, lowest first.
    ///
    /// Ordered for the same reason [`Store::skills`] is: a record built twice has to be
    /// byte-identical.
    pub fn crafting(&self, character_id: u32) -> Result<Vec<Profession>> {
        let conn = self.conn();
        crafting_rows(&conn, character_id)
    }

    /// One profession's `(level, exp)`, or `(0, 0)` when it was never learnt.
    pub fn profession(&self, character_id: u32, profession: u8) -> Result<(u32, u32)> {
        Ok(self
            .crafting(character_id)?
            .into_iter()
            .find(|p| p.profession == profession)
            .map(|p| (p.level, p.exp))
            .unwrap_or((0, 0)))
    }

    /// Learn a profession at level 1. Returns `false` when it was already known, and then
    /// changes nothing - a second turn-in must not reset a level-7 Smith to 1.
    pub fn learn_profession(&self, character_id: u32, profession: u8) -> Result<bool> {
        let conn = self.conn();
        let changed = conn.execute(
            "INSERT OR IGNORE INTO character_crafting (character_id, profession, level, exp)
             VALUES (?1, ?2, 1, 0)",
            rusqlite::params![i64::from(character_id), i64::from(profession)],
        )?;
        Ok(changed > 0)
    }

    /// Set a profession outright - the GM command, and nothing else.
    ///
    /// Level 0 **deletes** the row, which re-greys the tab.
    pub fn set_profession(&self, character_id: u32, profession: u8, level: u32, exp: u32) -> Result<()> {
        let conn = self.conn();
        if level == 0 {
            conn.execute(
                "DELETE FROM character_crafting WHERE character_id = ?1 AND profession = ?2",
                rusqlite::params![i64::from(character_id), i64::from(profession)],
            )?;
            return Ok(());
        }
        conn.execute(
            "INSERT INTO character_crafting (character_id, profession, level, exp)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(character_id, profession) DO UPDATE SET level = ?3, exp = ?4",
            rusqlite::params![
                i64::from(character_id),
                i64::from(profession),
                i64::from(level),
                i64::from(exp)
            ],
        )?;
        Ok(())
    }

    /// Add mastery, levelling as far as `max_level` allows.
    ///
    /// `max_level` is the character-level cap the caller works out
    /// (`crate::Store` knows nothing about character levels being worth 5 profession levels
    /// - that rule is `world::crafting::mastery_level_cap`). `needed` answers "how much
    /// mastery does level `n` require"; it is passed in for the same reason, and a `needed`
    /// of 0 means the level is the last one.
    ///
    /// **At the cap the bar parks at 100% and the overflow is discarded**, which is what
    /// `wasted` counts. A refusal is not silent: the caller is told what happened.
    pub fn add_mastery_exp(
        &self,
        character_id: u32,
        profession: u8,
        gain: u32,
        max_level: u32,
        needed: impl Fn(u32) -> u32,
    ) -> Result<MasteryGain> {
        let conn = self.conn();
        let (mut level, mut exp) = crafting_rows(&conn, character_id)?
            .into_iter()
            .find(|p| p.profession == profession)
            .map(|p| (p.level, p.exp))
            .unwrap_or((0, 0));
        if level == 0 {
            // Not learnt: there is nothing to add mastery to, and inventing a row here would
            // open a tab nobody earned.
            return Ok(MasteryGain { level: 0, exp: 0, levels_gained: 0, wasted: gain });
        }
        let before = level;
        let mut wasted: u32 = 0;
        exp = exp.saturating_add(gain);
        loop {
            let need = needed(level);
            if need == 0 || exp < need {
                break;
            }
            if level >= max_level {
                // Parked at 100%: keep the bar full, discard the rest.
                wasted = wasted.saturating_add(exp - need);
                exp = need;
                break;
            }
            exp -= need;
            level += 1;
        }
        conn.execute(
            "UPDATE character_crafting SET level = ?3, exp = ?4
              WHERE character_id = ?1 AND profession = ?2",
            rusqlite::params![
                i64::from(character_id),
                i64::from(profession),
                i64::from(level),
                i64::from(exp)
            ],
        )?;
        Ok(MasteryGain { level, exp, levels_gained: level - before, wasted })
    }
}

/// Read the rows on a caller-supplied connection - [`Store::skills`] needs this inside its
/// own borrow.
pub(crate) fn crafting_rows(conn: &Connection, character_id: u32) -> Result<Vec<Profession>> {
    let mut stmt = conn.prepare(
        "SELECT profession, level, exp
           FROM character_crafting
          WHERE character_id = ?1
          ORDER BY profession",
    )?;
    let rows = stmt.query_map([i64::from(character_id)], |row| {
        Ok(Profession {
            profession: row.get::<_, i64>(0)? as u8,
            level: row.get::<_, i64>(1)? as u32,
            exp: row.get::<_, i64>(2)? as u32,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> (Store, u32) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Wisp".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        (store, id)
    }

    /// Fifty for the first level, then the client's own curve - but the store is told the
    /// numbers rather than knowing them.
    fn needed(level: u32) -> u32 {
        match level {
            0 => 0,
            1 => 50,
            _ => {
                let mut v = 50f64;
                for _ in 2..=level {
                    v = (v * 1.32).trunc() + 100.0;
                }
                v as u32
            }
        }
    }

    /// A profession is learnt once; a second grant is a no-op rather than a reset.
    #[test]
    fn learning_twice_does_not_reset_a_levelled_profession() {
        let (store, id) = store();
        assert!(store.learn_profession(id, 2).unwrap(), "the first grant learns it");
        store.set_profession(id, 2, 7, 123).unwrap();
        assert!(!store.learn_profession(id, 2).unwrap(), "the second is refused");
        assert_eq!(store.profession(id, 2).unwrap(), (7, 123));
        // And an unlearnt one reads as zero rather than as an error.
        assert_eq!(store.profession(id, 5).unwrap(), (0, 0));
    }

    /// Mastery accumulates, crosses levels, and stops dead at the character-level cap.
    #[test]
    fn mastery_levels_up_and_is_discarded_at_the_cap() {
        let (store, id) = store();
        store.learn_profession(id, 0).unwrap();
        // 30 of the 50 the first level wants: no level, no waste.
        let g = store.add_mastery_exp(id, 0, 30, 10, needed).unwrap();
        assert_eq!((g.level, g.exp, g.levels_gained, g.wasted), (1, 30, 0, 0));
        // 25 more crosses 50 and leaves 5 towards level 2.
        let g = store.add_mastery_exp(id, 0, 25, 10, needed).unwrap();
        assert_eq!((g.level, g.exp, g.levels_gained, g.wasted), (2, 5, 1, 0));
        // Capped at level 2 by the character's level: the bar fills and the rest is gone.
        let g = store.add_mastery_exp(id, 0, 1_000, 2, needed).unwrap();
        assert_eq!((g.level, g.exp, g.levels_gained), (2, needed(2), 0));
        assert_eq!(g.wasted, 1_005 - needed(2));
        assert_eq!(store.profession(id, 0).unwrap(), (2, needed(2)));
        // A profession nobody learnt gains nothing at all - the whole grant is wasted, and
        // no row appears that would open a tab.
        let g = store.add_mastery_exp(id, 3, 500, 10, needed).unwrap();
        assert_eq!((g.level, g.wasted), (0, 500));
        assert!(store.crafting(id).unwrap().iter().all(|p| p.profession != 3));
    }

    /// The wire packing lives on the row, and level 0 deletes it.
    #[test]
    fn the_row_packs_itself_for_the_wire() {
        let (store, id) = store();
        store.set_profession(id, 1, 3, 200).unwrap();
        let rows = store.crafting(id).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].skill_id(), 92_010_000);
        assert_eq!(rows[0].packed_level(), (3 << 24) | 200);
        store.set_profession(id, 1, 0, 0).unwrap();
        assert!(store.crafting(id).unwrap().is_empty(), "level 0 closes the tab");
    }
}
