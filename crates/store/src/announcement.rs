//! **The scrolling banner's text**, set by a GM with `!announce` (`world::session::rates`).
//!
//! The owner, 2026-10-03: *"Remove the scrolling text at the top announcing the server's rates
//! on a cadence. The text should now be configurable with a GM command called !announce
//! <message> which will start that scrolling text"*, then *"The scrolling text should not start
//! if there is no configured message. The configured message should persist across server
//! restarts."*
//!
//! One row or none. In the database for the same reason the rates are (`crate::rates`): both
//! channel processes share only this file, so it is the one place a server-wide banner can live,
//! and it survives a restart because the file does.

use rusqlite::{params, Connection, OptionalExtension};

use crate::error::Result;
use crate::Store;

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS announcement (
            id     INTEGER PRIMARY KEY CHECK (id = 1),
            text   TEXT NOT NULL,
            set_at INTEGER NOT NULL
        );
        "#,
    )?;
    Ok(())
}

impl Store {
    /// The configured banner text, or `None` when there is none.
    pub fn announcement(&self) -> Result<Option<String>> {
        Ok(self
            .conn()
            .query_row("SELECT text FROM announcement WHERE id = 1", [], |r| r.get::<_, String>(0))
            .optional()?)
    }

    /// Set the banner text, or clear it with `None` (or an all-blank string).
    pub fn set_announcement(&self, text: Option<&str>, now_unix: i64) -> Result<()> {
        let conn = self.conn();
        match text.map(str::trim).filter(|t| !t.is_empty()) {
            Some(t) => conn.execute(
                "INSERT INTO announcement (id, text, set_at) VALUES (1, ?1, ?2)
                 ON CONFLICT(id) DO UPDATE SET text = excluded.text, set_at = excluded.set_at",
                params![t, now_unix],
            )?,
            None => conn.execute("DELETE FROM announcement", [])?,
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_banner_text_is_one_row_set_replaced_and_cleared() {
        let store = Store::open_in_memory().unwrap();
        assert_eq!(store.announcement().unwrap(), None, "nothing configured: no banner");
        store.set_announcement(Some("  Welcome to MapleCW!  "), 1).unwrap();
        assert_eq!(store.announcement().unwrap().as_deref(), Some("Welcome to MapleCW!"));
        store.set_announcement(Some("2x EXP this weekend"), 2).unwrap();
        assert_eq!(store.announcement().unwrap().as_deref(), Some("2x EXP this weekend"), "replaced, not added");
        store.set_announcement(Some("   "), 3).unwrap();
        assert_eq!(store.announcement().unwrap(), None, "blank clears");
        store.set_announcement(Some("back"), 4).unwrap();
        store.set_announcement(None, 5).unwrap();
        assert_eq!(store.announcement().unwrap(), None);
    }

    /// **It survives a restart**: a second `Store` on the same file reads it back.
    #[test]
    fn the_banner_text_survives_reopening_the_database() {
        let dir = std::env::temp_dir().join(format!("maplecw-announce-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("announce.db");
        let _ = std::fs::remove_file(&path);
        Store::open(&path).unwrap().set_announcement(Some("Server maintenance at 9pm"), 1).unwrap();
        assert_eq!(Store::open(&path).unwrap().announcement().unwrap().as_deref(), Some("Server maintenance at 9pm"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
