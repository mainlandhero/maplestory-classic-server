//! SQLite storage for accounts and sessions.

use std::sync::Mutex;

use rusqlite::{params, Connection, OptionalExtension};

use crate::error::{Result, StoreError};
use crate::password::{hash_password, verify_password};
use crate::session::{hash_token, new_token, NewSession};

/// How long an issued session stays valid.
pub const SESSION_TTL_SECS: i64 = 15 * 60;

#[derive(Debug, Clone)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub enabled: bool,
    pub created_at: i64,
    pub last_login: Option<i64>,
}

/// Outcome of an authentication attempt.
///
/// Deliberately does **not** distinguish "no such account" from "wrong password":
/// telling them apart lets an attacker enumerate valid account names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthOutcome {
    Ok { account_id: i64, token: String },
    InvalidCredentials,
    Disabled,
}

/// The SQLite connection is wrapped in a `Mutex` so `Store` is `Send + Sync` and can
/// be shared across threads via `Arc`. `rusqlite::Connection` is `Send` but not `Sync`,
/// and the servers built on this are concurrent, so serialising access here avoids
/// every caller having to invent its own locking.
pub struct Store {
    conn: Mutex<Connection>,
}

impl Store {
    /// Open (creating if needed) a database file.
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let conn = Connection::open(path)?;
        Self::init(conn)
    }

    /// In-memory database, for tests.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Self::init(conn)
    }

    fn init(conn: Connection) -> Result<Self> {
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS accounts (
                id            INTEGER PRIMARY KEY AUTOINCREMENT,
                name          TEXT    NOT NULL UNIQUE COLLATE NOCASE,
                -- argon2id PHC string; never a plain-text or reversible password
                password_hash TEXT    NOT NULL,
                enabled       INTEGER NOT NULL DEFAULT 1,
                created_at    INTEGER NOT NULL,
                last_login    INTEGER
            );

            CREATE TABLE IF NOT EXISTS sessions (
                -- SHA-256 of the token; the token itself is never stored
                token_hash TEXT    PRIMARY KEY,
                account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
                issued_at  INTEGER NOT NULL,
                expires_at INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_sessions_account ON sessions(account_id);
            CREATE INDEX IF NOT EXISTS idx_sessions_expiry  ON sessions(expires_at);
            "#,
        )?;
        Ok(Self { conn: Mutex::new(conn) })
    }

    /// Lock the connection. Poisoning cannot lose data here - the recovered guard is
    /// still a usable connection - so the lock is recovered rather than panicking.
    fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn now() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    }

    /// Account names are used in URLs, logs, and the game protocol, so keep them plain.
    fn validate_name(name: &str) -> Result<()> {
        let bad = |reason| {
            Err(StoreError::InvalidAccountName {
                name: name.to_string(),
                reason,
            })
        };
        if name.len() < 3 || name.len() > 24 {
            return bad("must be 3-24 characters");
        }
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return bad("only letters, digits and underscore are allowed");
        }
        Ok(())
    }

    /// Create an account. The password is hashed here and never stored as given.
    pub fn create_account(&self, name: &str, password: &str) -> Result<i64> {
        Self::validate_name(name)?;
        let hash = hash_password(password)?;

        let existing: Option<i64> = self.conn()
            .query_row(
                "SELECT id FROM accounts WHERE name = ?1",
                params![name],
                |r| r.get(0),
            )
            .optional()?;
        if existing.is_some() {
            return Err(StoreError::AccountExists {
                name: name.to_string(),
            });
        }

        self.conn().execute(
            "INSERT INTO accounts (name, password_hash, enabled, created_at)
             VALUES (?1, ?2, 1, ?3)",
            params![name, hash, Self::now()],
        )?;
        Ok(self.conn().last_insert_rowid())
    }

    pub fn get_account(&self, name: &str) -> Result<Option<Account>> {
        let acc = self.conn()
            .query_row(
                "SELECT id, name, enabled, created_at, last_login
                 FROM accounts WHERE name = ?1",
                params![name],
                |r| {
                    Ok(Account {
                        id: r.get(0)?,
                        name: r.get(1)?,
                        enabled: r.get::<_, i64>(2)? != 0,
                        created_at: r.get(3)?,
                        last_login: r.get(4)?,
                    })
                },
            )
            .optional()?;
        Ok(acc)
    }

    pub fn list_accounts(&self) -> Result<Vec<Account>> {
        // Bind the guard: a prepared statement borrows the connection, so the lock has
        // to outlive it.
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT id, name, enabled, created_at, last_login FROM accounts ORDER BY id",
        )?;
        let rows = stmt
            .query_map([], |r| {
                Ok(Account {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    enabled: r.get::<_, i64>(2)? != 0,
                    created_at: r.get(3)?,
                    last_login: r.get(4)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn set_enabled(&self, name: &str, enabled: bool) -> Result<()> {
        let n = self.conn().execute(
            "UPDATE accounts SET enabled = ?2 WHERE name = ?1",
            params![name, i64::from(enabled)],
        )?;
        if n == 0 {
            return Err(StoreError::NoSuchAccount {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    pub fn set_password(&self, name: &str, password: &str) -> Result<()> {
        let hash = hash_password(password)?;
        let n = self.conn().execute(
            "UPDATE accounts SET password_hash = ?2 WHERE name = ?1",
            params![name, hash],
        )?;
        if n == 0 {
            return Err(StoreError::NoSuchAccount {
                name: name.to_string(),
            });
        }
        // Changing a password invalidates existing sessions.
        if let Some(acc) = self.get_account(name)? {
            self.revoke_account_sessions(acc.id)?;
        }
        Ok(())
    }

    /// Authenticate and, on success, issue a session token.
    ///
    /// The password is verified even when the account does not exist, so the two cases
    /// take comparable time and cannot be told apart by an observer.
    pub fn authenticate(&self, name: &str, password: &str) -> Result<AuthOutcome> {
        let row: Option<(i64, String, i64)> = self.conn()
            .query_row(
                "SELECT id, password_hash, enabled FROM accounts WHERE name = ?1",
                params![name],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .optional()?;

        let Some((id, stored, enabled)) = row else {
            // Verify against a dummy so a missing account costs roughly the same work
            // as a wrong password, and the two cannot be told apart by timing.
            let _ = verify_password(password, dummy_hash());
            return Ok(AuthOutcome::InvalidCredentials);
        };

        if !verify_password(password, &stored)? {
            return Ok(AuthOutcome::InvalidCredentials);
        }
        if enabled == 0 {
            return Ok(AuthOutcome::Disabled);
        }

        let now = Self::now();
        self.conn().execute(
            "UPDATE accounts SET last_login = ?2 WHERE id = ?1",
            params![id, now],
        )?;

        let NewSession { token, hash } = new_token();
        self.conn().execute(
            "INSERT INTO sessions (token_hash, account_id, issued_at, expires_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![hash, id, now, now + SESSION_TTL_SECS],
        )?;

        Ok(AuthOutcome::Ok { account_id: id, token })
    }

    /// Resolve a session token to its account, if it is valid and unexpired.
    pub fn validate_session(&self, token: &str) -> Result<Option<Account>> {
        let hash = hash_token(token);
        let now = Self::now();
        let account_id: Option<i64> = self.conn()
            .query_row(
                "SELECT account_id FROM sessions WHERE token_hash = ?1 AND expires_at > ?2",
                params![hash, now],
                |r| r.get(0),
            )
            .optional()?;

        let Some(id) = account_id else {
            return Ok(None);
        };
        let acc = self.conn()
            .query_row(
                "SELECT id, name, enabled, created_at, last_login
                 FROM accounts WHERE id = ?1 AND enabled = 1",
                params![id],
                |r| {
                    Ok(Account {
                        id: r.get(0)?,
                        name: r.get(1)?,
                        enabled: r.get::<_, i64>(2)? != 0,
                        created_at: r.get(3)?,
                        last_login: r.get(4)?,
                    })
                },
            )
            .optional()?;
        Ok(acc)
    }

    /// Consume a session: valid exactly once. Used for the launcher → game handoff, so
    /// a captured token cannot be replayed.
    pub fn consume_session(&self, token: &str) -> Result<Option<Account>> {
        let acc = self.validate_session(token)?;
        if acc.is_some() {
            self.conn().execute(
                "DELETE FROM sessions WHERE token_hash = ?1",
                params![hash_token(token)],
            )?;
        }
        Ok(acc)
    }

    pub fn revoke_account_sessions(&self, account_id: i64) -> Result<usize> {
        Ok(self.conn().execute(
            "DELETE FROM sessions WHERE account_id = ?1",
            params![account_id],
        )?)
    }

    /// Drop expired rows. Cheap to call periodically.
    pub fn purge_expired_sessions(&self) -> Result<usize> {
        Ok(self.conn().execute(
            "DELETE FROM sessions WHERE expires_at <= ?1",
            params![Self::now()],
        )?)
    }
}

/// An argon2id hash of a random secret, used only to spend comparable time when the
/// account does not exist. Nothing can authenticate against it.
///
/// Computed at runtime rather than hard-coded: a hand-written constant that fails to
/// parse would make `verify_password` return early, quietly removing the timing
/// equalisation this exists to provide.
fn dummy_hash() -> &'static str {
    static DUMMY: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    DUMMY.get_or_init(|| {
        let filler = crate::session::new_token().token;
        hash_password(&filler).expect("hashing a 64-char token cannot fail")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> Store {
        Store::open_in_memory().unwrap()
    }

    #[test]
    fn creates_and_authenticates() {
        let s = store();
        let id = s.create_account("player_one", "hunter2hunter2").unwrap();
        match s.authenticate("player_one", "hunter2hunter2").unwrap() {
            AuthOutcome::Ok { account_id, token } => {
                assert_eq!(account_id, id);
                assert!(!token.is_empty());
            }
            other => panic!("expected Ok, got {other:?}"),
        }
    }

    #[test]
    fn plain_password_is_never_stored() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        let stored: String = s.conn()
            .query_row(
                "SELECT password_hash FROM accounts WHERE name='player_one'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(!stored.contains("hunter2hunter2"));
        assert!(stored.starts_with("$argon2id$"));
    }

    #[test]
    fn wrong_password_and_unknown_account_look_the_same() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        assert_eq!(
            s.authenticate("player_one", "wrong-password").unwrap(),
            AuthOutcome::InvalidCredentials
        );
        assert_eq!(
            s.authenticate("nobody_here", "wrong-password").unwrap(),
            AuthOutcome::InvalidCredentials
        );
    }

    #[test]
    fn duplicate_names_are_rejected_case_insensitively() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        assert!(matches!(
            s.create_account("PLAYER_ONE", "another-password"),
            Err(StoreError::AccountExists { .. })
        ));
    }

    #[test]
    fn disabled_accounts_cannot_log_in() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        s.set_enabled("player_one", false).unwrap();
        assert_eq!(
            s.authenticate("player_one", "hunter2hunter2").unwrap(),
            AuthOutcome::Disabled
        );
    }

    #[test]
    fn session_validates_then_is_consumed_once() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        let AuthOutcome::Ok { token, .. } =
            s.authenticate("player_one", "hunter2hunter2").unwrap()
        else {
            panic!("login failed");
        };

        assert!(s.validate_session(&token).unwrap().is_some());
        assert!(s.consume_session(&token).unwrap().is_some());
        // Replaying the same token must fail.
        assert!(s.validate_session(&token).unwrap().is_none());
        assert!(s.consume_session(&token).unwrap().is_none());
    }

    #[test]
    fn raw_token_is_not_in_the_database() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        let AuthOutcome::Ok { token, .. } =
            s.authenticate("player_one", "hunter2hunter2").unwrap()
        else {
            panic!("login failed");
        };
        let stored: String = s.conn()
            .query_row("SELECT token_hash FROM sessions", [], |r| r.get(0))
            .unwrap();
        assert_ne!(stored, token);
    }

    #[test]
    fn changing_password_revokes_sessions() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        let AuthOutcome::Ok { token, .. } =
            s.authenticate("player_one", "hunter2hunter2").unwrap()
        else {
            panic!("login failed");
        };
        s.set_password("player_one", "a-brand-new-password").unwrap();
        assert!(s.validate_session(&token).unwrap().is_none());
        assert!(s.authenticate("player_one", "a-brand-new-password").is_ok());
    }

    #[test]
    fn invalid_names_are_rejected() {
        let s = store();
        assert!(s.create_account("ab", "hunter2hunter2").is_err());
        assert!(s.create_account("has space", "hunter2hunter2").is_err());
        assert!(s.create_account("drop;table", "hunter2hunter2").is_err());
    }

    #[test]
    fn dummy_hash_is_a_usable_argon2_hash() {
        // Regression: a hard-coded constant here was invalid Base64, so
        // verify_password returned early and the timing equalisation for unknown
        // accounts silently did nothing.
        let h = dummy_hash();
        assert!(h.starts_with("$argon2id$"), "got {h}");
        assert_eq!(
            verify_password("some guess", h).expect("dummy hash must parse"),
            false
        );
    }

    #[test]
    fn unknown_account_takes_comparable_time_to_a_wrong_password() {
        use std::time::Instant;
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();

        let t0 = Instant::now();
        s.authenticate("player_one", "wrong-password").unwrap();
        let wrong_pw = t0.elapsed();

        let t1 = Instant::now();
        s.authenticate("nobody_here", "wrong-password").unwrap();
        let unknown = t1.elapsed();

        // Argon2 dominates both paths; without the dummy verify the unknown-account
        // case returns almost instantly. Generous bound to stay stable on CI.
        assert!(
            unknown * 5 > wrong_pw,
            "unknown-account path far faster than wrong-password path \
             ({unknown:?} vs {wrong_pw:?}), which leaks account existence"
        );
    }

    #[test]
    fn short_password_is_rejected_at_creation() {
        let s = store();
        assert!(matches!(
            s.create_account("player_one", "short"),
            Err(StoreError::PasswordTooShort { .. })
        ));
    }
}
