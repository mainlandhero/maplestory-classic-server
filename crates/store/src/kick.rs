//! **Server-initiated disconnects, requested in one process and carried out in another.**
//!
//! The owner, 2026-09-13: *"If someone were to login again on an account which there is an active
//! client connection with the server on a channel, can we disconnect that client? Make sure
//! every disconnect initiated by the server is logged clearly in the server channel logs."*
//!
//! The three servers are three processes - `maplecw-auth`, `maplecw-login`, `maplecw-world` -
//! and they share nothing but this database. So the sign-in cannot reach into the channel and
//! close a socket; it leaves a row, and the channel reads it.
//!
//! # One row per account, and a timestamp rather than a flag
//!
//! ```text
//! kick_requests(account_id PRIMARY KEY, requested_at, reason)
//! ```
//!
//! A **flag** would be wrong here for the same reason `crate::presence` gives for not having a
//! `logged_in` column: it has to be cleared by somebody, and the somebody is a process that
//! may have died. This is a timestamp, and a connection only honours a kick **newer than the
//! moment it joined**. That single rule does all the work a cleanup pass would:
//!
//! * the connection the sign-in is about to start is younger than the kick, so it is not
//!   kicked by the sign-in that authorised it - which is the whole trap in this feature;
//! * a kick that lands while nobody is connected does nothing, and does not ambush the next
//!   player to log in;
//! * a row left behind by a crashed process is inert.
//!
//! So nothing has to be deleted for correctness, and the row stays readable as an audit trail
//! of when an account was last thrown off and why. [`Store::clear_kick_request`] exists for a
//! caller that wants to withdraw one, not for hygiene.
//!
//! # This is not a ban and it is not authentication
//!
//! It disconnects a socket. It does not stop the same client reconnecting a second later, and
//! `CLAUDE.md`'s standing constraint is untouched: the game socket still carries no
//! credentials, so what is being disconnected is a connection the server *attributed* to an
//! account, by the migration row it claimed.

use rusqlite::{Connection, OptionalExtension};

use crate::db::Store;
use crate::error::Result;

/// A queued disconnect, as the channel reads it back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KickRequest {
    pub account_id: i64,
    /// Unix seconds, from [`Store::now`].
    pub requested_at: i64,
    /// Why, in words meant for a log line a person will read once.
    pub reason: String,
}

/// One connection's standing question: *has anybody asked for me to be thrown off since I
/// joined?*
///
/// Made by [`Store::watch_for_kicks`], which stamps the join time so a caller cannot supply
/// the wrong one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KickWatch {
    pub account_id: i64,
    /// Unix seconds at the moment this connection learned its account.
    pub joined_at: i64,
}

impl KickWatch {
    /// `Some` when this connection must be disconnected.
    pub fn poll(&self, store: &Store) -> Result<Option<KickRequest>> {
        store.kick_since(self.account_id, self.joined_at)
    }
}

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        -- ONE ROW PER ACCOUNT, replaced rather than accumulated: the newest request is the
        -- only one that can matter, because any connection old enough to be caught by an
        -- older one is caught by this one too.
        --
        -- No FOREIGN KEY to accounts on purpose. This table is written by the auth process
        -- inside the sign-in transaction and read by the channel; a cascade from an account
        -- delete would be nice to have and is not worth making the write depend on
        -- `PRAGMA foreign_keys` being on in whichever process got there first. A row for a
        -- deleted account is unreachable, not harmful.
        CREATE TABLE IF NOT EXISTS kick_requests (
            account_id   INTEGER PRIMARY KEY,
            requested_at INTEGER NOT NULL,
            reason       TEXT    NOT NULL
        );
        "#,
    )?;
    Ok(())
}

impl Store {
    /// **Ask for every connection this account has open to be disconnected.**
    ///
    /// Returns the `requested_at` that was written, which is [`Store::now`]. Connections that
    /// joined *before* that second will be disconnected the next time they look; connections
    /// that join after it will not.
    ///
    /// The one-second granularity is deliberate and it is the safe direction: a connection
    /// that joined in the same second as the kick **is** disconnected, because the comparison
    /// is `requested_at >= joined_at`. Erring the other way would let a client that reconnected
    /// instantly survive the kick, which is exactly the case this exists for.
    pub fn request_kick(&self, account_id: i64, reason: &str) -> Result<i64> {
        let conn = self.conn();
        create_tables(&conn)?;
        let now = Store::now();
        request_kick_in(&conn, account_id, reason, now)?;
        Ok(now)
    }

    /// A kick this connection has to obey, or `None`.
    ///
    /// `joined_at` is when this connection learned which account it is - for a channel, the
    /// moment it claimed its migration. A request from before that belongs to somebody else's
    /// session and is ignored.
    pub fn kick_since(&self, account_id: i64, joined_at: i64) -> Result<Option<KickRequest>> {
        let conn = self.conn();
        create_tables(&conn)?;
        conn.query_row(
            "SELECT requested_at, reason FROM kick_requests
              WHERE account_id = ?1 AND requested_at >= ?2",
            rusqlite::params![account_id, joined_at],
            |row| {
                Ok(KickRequest {
                    account_id,
                    requested_at: row.get(0)?,
                    reason: row.get(1)?,
                })
            },
        )
        .optional()
        .map_err(Into::into)
    }

    /// **Start watching, from now.** The channel calls this the moment a connection learns
    /// which account it is.
    ///
    /// The join time is captured here rather than passed to every poll, because the one way
    /// to get this feature dangerously wrong is to compare against the wrong instant - a poll
    /// that used "now" as its epoch would never see a kick, and a poll that used 0 would
    /// disconnect the very client the sign-in just authorised. Neither mistake is available
    /// through this type.
    pub fn watch_for_kicks(&self, account_id: i64) -> KickWatch {
        KickWatch { account_id, joined_at: Store::now() }
    }

    /// Withdraw a queued kick. `true` when there was one.
    ///
    /// Not needed for correctness - see the module header - and not called on the serving
    /// path. It is here so an operator command can undo a mistake before the player's next
    /// poll, and so the tests can prove the row is a row.
    pub fn clear_kick_request(&self, account_id: i64) -> Result<bool> {
        let conn = self.conn();
        create_tables(&conn)?;
        let n = conn.execute(
            "DELETE FROM kick_requests WHERE account_id = ?1",
            rusqlite::params![account_id],
        )?;
        Ok(n > 0)
    }
}

/// The write, on a connection the caller already holds - so a sign-in can queue the kick
/// inside the very transaction that superseded the claim, and a crash between the two cannot
/// leave a claim replaced with nobody thrown off.
pub(crate) fn request_kick_in(
    conn: &Connection,
    account_id: i64,
    reason: &str,
    now: i64,
) -> Result<()> {
    create_tables(conn)?;
    conn.execute(
        "INSERT INTO kick_requests (account_id, requested_at, reason)
              VALUES (?1, ?2, ?3)
         ON CONFLICT(account_id) DO UPDATE SET
             requested_at = excluded.requested_at,
             reason       = excluded.reason",
        rusqlite::params![account_id, now, reason],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> (Store, i64, i64) {
        let store = Store::open_in_memory().unwrap();
        let a = store.create_account("wisp", "correct horse battery").unwrap();
        let b = store.create_account("wisp_alt", "correct horse battery").unwrap();
        (store, a, b)
    }

    /// The positive control: the read can find a request, so a `None` below means something.
    #[test]
    fn a_request_is_read_back_by_a_connection_that_joined_before_it() {
        let (store, wisp, _) = store();
        let joined = Store::now() - 60;
        assert_eq!(store.kick_since(wisp, joined).unwrap(), None, "nothing queued yet");

        let at = store.request_kick(wisp, "signed in again").unwrap();
        let got = store.kick_since(wisp, joined).unwrap().expect("the request just written");
        assert_eq!(got.account_id, wisp);
        assert_eq!(got.requested_at, at);
        assert_eq!(got.reason, "signed in again");
    }

    /// **The trap this whole design is shaped around.** The sign-in that queues the kick is
    /// the prelude to a new client connecting; that client must not be thrown off by the
    /// request that authorised it.
    #[test]
    fn a_connection_that_joined_after_the_request_is_not_kicked() {
        let (store, wisp, _) = store();
        let at = store.request_kick(wisp, "signed in again").unwrap();
        assert_eq!(store.kick_since(wisp, at + 1).unwrap(), None);
    }

    /// Same second counts. Erring this way disconnects an instant reconnect; erring the other
    /// way lets it survive, which is the case the feature exists for.
    #[test]
    fn a_connection_that_joined_in_the_same_second_is_still_kicked() {
        let (store, wisp, _) = store();
        let at = store.request_kick(wisp, "signed in again").unwrap();
        assert!(store.kick_since(wisp, at).unwrap().is_some());
    }

    #[test]
    fn a_request_names_one_account_and_leaves_the_others_alone() {
        let (store, wisp, other) = store();
        let joined = Store::now() - 60;
        store.request_kick(wisp, "signed in again").unwrap();
        assert!(store.kick_since(wisp, joined).unwrap().is_some());
        assert_eq!(store.kick_since(other, joined).unwrap(), None);
    }

    #[test]
    fn a_second_request_replaces_the_first_rather_than_accumulating() {
        let (store, wisp, _) = store();
        store.request_kick(wisp, "first").unwrap();
        store.request_kick(wisp, "second").unwrap();
        let rows: i64 = store
            .conn()
            .query_row("SELECT COUNT(*) FROM kick_requests", [], |r| r.get(0))
            .unwrap();
        assert_eq!(rows, 1);
        let joined = Store::now() - 60;
        assert_eq!(store.kick_since(wisp, joined).unwrap().unwrap().reason, "second");
    }

    #[test]
    fn a_withdrawn_request_stops_being_read() {
        let (store, wisp, _) = store();
        let joined = Store::now() - 60;
        store.request_kick(wisp, "signed in again").unwrap();
        assert!(store.clear_kick_request(wisp).unwrap());
        assert_eq!(store.kick_since(wisp, joined).unwrap(), None);
        assert!(!store.clear_kick_request(wisp).unwrap(), "there is nothing left to withdraw");
    }

    /// The table is made on every entry point, so a database that has never seen one answers
    /// cleanly instead of failing with "no such table".
    #[test]
    fn a_database_that_has_never_had_a_kick_answers_none() {
        let store = Store::open_in_memory().unwrap();
        assert_eq!(store.kick_since(1, 0).unwrap(), None);
    }
}
