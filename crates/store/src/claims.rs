//! **Which account the login server should serve the next game connection as.**
//!
//! # The problem this exists to solve
//!
//! The game socket carries no credentials. The client never sends a username - it sends a
//! login packet whose form is vestigial, and the server supplies both the result *and* the
//! account name. So `crates/login` is started with `--account NAME` and serves **every**
//! connection as that one account, which means the machine can only ever play one account.
//!
//! A claim moves that decision out of the command line and into the database, where a GUI
//! launcher can set it. The launcher authenticates for real - [`Store::authenticate`],
//! argon2id - and then *stakes a claim*: "the next client launch is this account". The login
//! server reads the claim once per connection and serves that account.
//!
//! # This is not authentication, and must never be reported as if it were
//!
//! `CLAUDE.md`: *"Nothing authenticates. The game socket carries no credentials."* That is
//! still true with this module in place. A claim does not prove who is on the far end of the
//! game socket; it decides **which account a credential-less connection is served as**, on a
//! machine where the only party that can stake one has already authenticated to the launcher.
//! The property is machine-local state, not identity - the same honest distinction
//! [`crate::migration`] draws about its `u32` seed.
//!
//! It follows that the claim is **global**, one row for the whole database, rather than
//! per-connection or per-machine: there is nothing on the game socket to key it by. That is
//! sound only because this deployment is one person on one machine, exactly the reasoning
//! `Store::claim_sole_migration_for_channel` already rests on. If a second player ever shares
//! this database, this design is wrong and has to be replaced rather than patched.
//!
//! # A claim is NOT single use, and that is the whole design
//!
//! The obvious shape - a `consumed_at` column, or a `DELETE` on read - would be wrong, and
//! wrong in a way that reads on screen as data loss.
//!
//! **The client makes a second login connection in the same launch.** `CLAUDE.md` records it
//! from a capture: `login.log` had **two** `0x0010`s in one run, because "Log Out" and
//! "Choose another world" both drop the login socket and reconnect. A single-use claim would
//! be spent by the first connection, and the second would fall back to whatever
//! `--account` says - a *different* account, with a different character list. On screen that
//! is not "the claim expired", it is **"my characters vanished when I logged out"**, and it
//! would be reported as a character-deletion bug rather than as a session bug.
//!
//! So a claim is a **standing setting with an expiry**, read as many times as the client
//! cares to reconnect. `reading_the_claim_twice_serves_the_same_account` is the regression
//! test for exactly that, and it is named so nobody deletes it as redundant.
//!
//! What replaces a claim is a *newer stake*, [`Store::clear_login_claims`], or the passage of
//! [`LOGIN_CLAIM_TTL_SECS`].
//!
//! # The token is recorded and is not yet read by anything
//!
//! `token_hash` is the SHA-256 of the session token the launcher was handed - never the token
//! ([`crate::session::hash_token`], and the standing constraint behind it). It is written so a
//! claim can be tied back to the login that produced it, and it is **write-only today**:
//! nothing in this module checks it, so it is not a guard and must not be described as one.
//! `CLAUDE.md` is blunt about the failure mode - *"a guard whose answer is ignored is not a
//! guard"* - so the honest statement is that the column is an audit trail waiting for a
//! reader, not a check.

use rusqlite::{Connection, OptionalExtension};

use crate::db::Store;
use crate::error::{Result, StoreError};
use crate::session::hash_token;

/// A live claim: who the next game connection should be served as.
#[derive(Debug, Clone)]
pub struct LoginClaim {
    pub account_id: i64,
    pub account_name: String,
    pub created_at: i64,
    pub expires_at: i64,
}

/// How long a claim stays live if nothing replaces it.
///
/// Twelve hours, which is far longer than a play session and deliberately so. The expiry is
/// not a security boundary - see the module docs on what a claim is not - it exists so a
/// machine left alone overnight does not silently launch into somebody's account tomorrow
/// because the launcher was used once yesterday.
pub const LOGIN_CLAIM_TTL_SECS: i64 = 12 * 3600;

/// Create the claim table. Intended to be called from `Store::init` on every open, beside
/// `crate::skillpoints::create_tables`, so it must be idempotent.
///
/// A plain `CREATE TABLE IF NOT EXISTS` is enough **because the table is brand new** - the
/// same reasoning `crate::quest::create_tables` and `crate::skillpoints::create_tables` spell
/// out. It is the opposite of a *column* added to a table that already exists: `CREATE TABLE
/// IF NOT EXISTS` does nothing at all to an existing table, which is why
/// `Store::add_meso_column` and friends need an explicit `PRAGMA table_info` guard around an
/// `ALTER`. Nothing here touches `accounts`.
///
/// # Why every entry point below calls this as well
///
/// Not tidiness, and not a substitute for the line in `Store::init` - both should exist. This
/// module cannot add that line itself (`db.rs` belongs to the coordinator this session), and
/// `CLAUDE.md`'s "Built is not wired" section is about subsystems that were finished and never
/// connected, which *"on screen look identical to not existing"*.
///
/// Here that failure would be worse than invisible. [`Store::current_login_claim`] is read
/// once per login connection; with no table it returns `Err`, and a login server that answers
/// an error instead of a reply freezes the client's entire UI - the "always answer" rule.
/// Ensuring the table at each entry point costs one no-op `CREATE TABLE IF NOT EXISTS` per
/// login (not per packet) and makes the unwired state impossible rather than fatal.
pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        -- The account the next game connection is served as. At most one row in practice:
        -- `stake_login_claim` clears the table before inserting, so a stale row cannot sit
        -- here naming an account nobody is playing.
        --
        -- No `consumed_at`, and nothing deletes on read. A claim is a standing setting, not a
        -- ticket: the client opens a SECOND login connection after "Log Out" / "Choose another
        -- world" (two 0x0010s in one launch, from a capture), and a single-use claim would
        -- drop that reconnect back to the fallback account. See the module docs.
        --
        -- token_hash is the SHA-256 of the launcher's session token. The token itself is never
        -- stored, here or anywhere else in this crate.
        CREATE TABLE IF NOT EXISTS login_claims (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
            token_hash TEXT    NOT NULL,
            created_at INTEGER NOT NULL,
            expires_at INTEGER NOT NULL
        );

        -- The liveness predicate is the only filter on the hot read.
        CREATE INDEX IF NOT EXISTS idx_login_claims_expiry ON login_claims(expires_at);
        "#,
    )?;
    Ok(())
}

impl Store {
    /// **Bind subsequent game connections to `account_id`.** Replaces any existing claim.
    ///
    /// `token` is the session token the launcher just received from [`Store::authenticate`];
    /// only its SHA-256 is stored, never the token. Pass [`LOGIN_CLAIM_TTL_SECS`] for
    /// `ttl_secs` unless there is a reason not to; a `ttl_secs` of `0` or less produces a row
    /// that is already expired and will never be served, which is useful in tests and is
    /// otherwise a way of writing "no claim" the long way round.
    ///
    /// # Refusals
    ///
    /// An account that does not exist, or exists and is **disabled**, is refused with
    /// [`StoreError::NoSuchAccount`] rather than staked. The alternative - staking anyway -
    /// would hand the caller back a `LoginClaim` describing a claim that
    /// [`Store::current_login_claim`] can never return, because that read joins to `accounts`
    /// and skips disabled ones. A return value that says "done" about something that will
    /// never happen is the shape `CLAUDE.md` keeps warning about; failing loudly at the
    /// launcher, where a person is watching, is the cheaper end of that trade.
    ///
    /// (`NoSuchAccount` is reused rather than a new error variant added, because `error.rs` is
    /// shared and this module was scoped to one new file. The name it carries says which case
    /// it was.)
    pub fn stake_login_claim(
        &self,
        account_id: i64,
        token: &str,
        ttl_secs: i64,
    ) -> Result<LoginClaim> {
        // Hash before taking the lock: nothing that holds the connection should be handling
        // the plain token at all.
        let token_hash = hash_token(token);
        let now = Store::now();
        // Saturating, so an absurd ttl cannot wrap the expiry into the past and produce a
        // claim that is dead the instant it is written.
        let expires_at = now.saturating_add(ttl_secs);

        let mut conn = self.conn();
        create_tables(&conn)?;
        // One transaction: the clear and the insert are one replacement. A failure between
        // them must not leave the machine with no claim at all, which would silently fall the
        // next launch back to the `--account` default.
        let tx = conn.transaction()?;

        let account: Option<(String, i64)> = tx
            .query_row(
                "SELECT name, enabled FROM accounts WHERE id = ?1",
                rusqlite::params![account_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        // Both refusals drop the transaction unread, so a refused stake writes nothing and
        // leaves whatever claim was already there intact.
        let Some((account_name, enabled)) = account else {
            return Err(StoreError::NoSuchAccount { name: format!("account id {account_id}") });
        };
        if enabled == 0 {
            return Err(StoreError::NoSuchAccount { name: format!("{account_name} (disabled)") });
        }

        tx.execute("DELETE FROM login_claims", [])?;
        tx.execute(
            "INSERT INTO login_claims (account_id, token_hash, created_at, expires_at)
                  VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![account_id, token_hash, now, expires_at],
        )?;
        tx.commit()?;

        Ok(LoginClaim { account_id, account_name, created_at: now, expires_at })
    }

    /// **The account the next connection should be served as**, or `None` if no live claim.
    ///
    /// Read this once per login connection and fall back to the configured default account
    /// when it is `None`. **Read it again on the next connection** - it is not consumed, and
    /// the client reconnects within one launch; see the module docs.
    ///
    /// Three things make a claim live, and all three are enforced in the query rather than
    /// left to the caller:
    ///
    /// * `expires_at` is strictly in the future. `expires_at <= now` is not live.
    /// * The account still exists. A deleted one takes its claims with it through
    ///   `ON DELETE CASCADE`, and the join would drop it regardless.
    /// * The account is still **enabled**. Disabling an account is how this server locks
    ///   somebody out, and a claim staked before that must not outlive it.
    ///
    /// `account_name` comes from that join, not from the claim row, so a returned claim can
    /// never name an account that is not there - and cannot go stale if an account is ever
    /// renamed.
    ///
    /// Newest wins. In practice there is at most one row, because
    /// [`Store::stake_login_claim`] clears the table first; the ordering is what makes that a
    /// belt-and-braces detail rather than a load-bearing assumption.
    ///
    /// A caller must treat an `Err` here as "no claim, use the fallback", **not** as a reason
    /// to refuse the connection. An unanswered login freezes the client's whole UI.
    pub fn current_login_claim(&self) -> Result<Option<LoginClaim>> {
        let now = Store::now();
        let conn = self.conn();
        create_tables(&conn)?;
        conn.query_row(
            "SELECT c.account_id, a.name, c.created_at, c.expires_at
               FROM login_claims c
               JOIN accounts a ON a.id = c.account_id
              WHERE c.expires_at > ?1 AND a.enabled = 1
              ORDER BY c.created_at DESC, c.id DESC
              LIMIT 1",
            rusqlite::params![now],
            |row| {
                Ok(LoginClaim {
                    account_id: row.get(0)?,
                    account_name: row.get(1)?,
                    created_at: row.get(2)?,
                    expires_at: row.get(3)?,
                })
            },
        )
        .optional()
        .map_err(Into::into)
    }

    /// **Drop every claim.** Returns how many rows went.
    ///
    /// What a launcher's "sign out" calls. The next connection falls back to the configured
    /// default account, which is the behaviour this whole module is an alternative to rather
    /// than a replacement for.
    pub fn clear_login_claims(&self) -> Result<usize> {
        let conn = self.conn();
        create_tables(&conn)?;
        Ok(conn.execute("DELETE FROM login_claims", [])?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::AuthOutcome;

    /// A store with two accounts, so "the claim moved" can be told from "the claim is still
    /// whatever it was".
    fn store_with_accounts() -> (Store, i64, i64) {
        let store = Store::open_in_memory().unwrap();
        let wisp = store.create_account("wisp", "correct horse battery").unwrap();
        let other = store.create_account("wisp_alt", "correct horse battery").unwrap();
        (store, wisp, other)
    }

    /// The real token a launcher would hold: issued by `authenticate`, never invented.
    fn login(store: &Store, name: &str) -> String {
        let AuthOutcome::Ok { token, .. } = store.authenticate(name, "correct horse battery").unwrap()
        else {
            panic!("the test account must authenticate");
        };
        token
    }

    fn claim_rows(store: &Store) -> i64 {
        store
            .conn()
            .query_row("SELECT COUNT(*) FROM login_claims", [], |row| row.get(0))
            .unwrap()
    }

    /// **The positive control, and the wiring check.**
    ///
    /// A fresh store has never had `crate::claims::create_tables` called on it by anything but
    /// this module, so reaching a clean `None` here proves the table gets made on the read
    /// path. If this ever starts failing with "no such table", the entry-point ensure has been
    /// removed and the line in `Store::init` is the only thing holding the feature up.
    ///
    /// It is also the control for every test below: they assert `Some(..)`, and an empty
    /// answer from a broken query would be indistinguishable from "no claim staked" without
    /// this.
    #[test]
    fn a_store_with_no_claim_answers_none_rather_than_failing() {
        let (store, _, _) = store_with_accounts();
        assert!(store.current_login_claim().unwrap().is_none());
        assert_eq!(claim_rows(&store), 0);
        // And clearing an empty table is a no-op rather than an error.
        assert_eq!(store.clear_login_claims().unwrap(), 0);
    }

    /// Staking and then reading gives back the same account, by id and by name.
    #[test]
    fn a_staked_claim_reads_back_as_the_account_that_was_staked() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");

        let staked = store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();
        assert_eq!(staked.account_id, wisp);
        assert_eq!(staked.account_name, "wisp");
        assert_eq!(staked.expires_at, staked.created_at + LOGIN_CLAIM_TTL_SECS);

        let live = store.current_login_claim().unwrap().expect("the claim is live");
        assert_eq!(live.account_id, wisp);
        assert_eq!(live.account_name, "wisp", "the name comes from the accounts join");
        assert_eq!(live.created_at, staked.created_at);
        assert_eq!(live.expires_at, staked.expires_at);
        assert_eq!(claim_rows(&store), 1);
    }

    /// **The reconnect regression: reading a claim does not spend it.**
    ///
    /// The client opens a SECOND login connection in the same launch - "Log Out" and "Choose
    /// another world" both drop the socket and reconnect, and a capture recorded two `0x0010`s
    /// in one run. If a read consumed the claim, that second connection would be served as the
    /// fallback account, and on screen that reads as *"my characters vanished when I logged
    /// out"* rather than as a session bug.
    ///
    /// So: three reads, one stake, same answer every time - and the row is still there
    /// afterwards, because an answer that merely *looks* right could also come from a re-stake
    /// that nobody asked for.
    #[test]
    fn reading_the_claim_twice_serves_the_same_account() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();

        for connection in 1..=3 {
            let live = store
                .current_login_claim()
                .unwrap()
                .unwrap_or_else(|| panic!("login connection {connection} lost the claim"));
            assert_eq!(live.account_id, wisp, "connection {connection} was served the wrong account");
            assert_eq!(live.account_name, "wisp");
        }
        assert_eq!(claim_rows(&store), 1, "reading must not consume the row");
    }

    /// A second stake for a different account moves the claim, and leaves exactly one row.
    ///
    /// Both effects are checked. A store that inserted without clearing would pass the first
    /// assertion on the strength of "newest wins" alone while quietly accumulating rows that
    /// name accounts nobody is playing.
    #[test]
    fn a_second_stake_for_another_account_replaces_the_first() {
        let (store, wisp, other) = store_with_accounts();
        let wisp_token = login(&store, "wisp");
        let other_token = login(&store, "wisp_alt");

        store.stake_login_claim(wisp, &wisp_token, LOGIN_CLAIM_TTL_SECS).unwrap();
        assert_eq!(store.current_login_claim().unwrap().unwrap().account_id, wisp);

        store.stake_login_claim(other, &other_token, LOGIN_CLAIM_TTL_SECS).unwrap();
        let live = store.current_login_claim().unwrap().expect("the newer claim is live");
        assert_eq!(live.account_id, other, "the second stake did not take over");
        assert_eq!(live.account_name, "wisp_alt");
        assert_eq!(claim_rows(&store), 1, "the first claim was replaced, not stacked on");

        // And staking the first account back is not blocked by anything the second left.
        store.stake_login_claim(wisp, &wisp_token, LOGIN_CLAIM_TTL_SECS).unwrap();
        assert_eq!(store.current_login_claim().unwrap().unwrap().account_id, wisp);
    }

    /// An expired claim is not live. `expires_at <= now` is the predicate, so a ttl of exactly
    /// zero is already dead - and the row is still on disk, which is what separates "expired"
    /// from "never written".
    #[test]
    fn an_expired_claim_is_not_returned() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");

        store.stake_login_claim(wisp, &token, 0).unwrap();
        assert_eq!(claim_rows(&store), 1, "the row exists...");
        assert!(store.current_login_claim().unwrap().is_none(), "...and is not live");

        store.stake_login_claim(wisp, &token, -60).unwrap();
        assert_eq!(claim_rows(&store), 1);
        assert!(store.current_login_claim().unwrap().is_none());

        // A live ttl on the same store proves the query can return something at all - without
        // this the two assertions above would also pass against a query that never matches.
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();
        assert_eq!(store.current_login_claim().unwrap().unwrap().account_id, wisp);
    }

    /// Disabling an account kills its claim. Disabling is how this server locks somebody out,
    /// and a claim staked beforehand must not outlive it.
    #[test]
    fn a_claim_whose_account_was_disabled_is_not_returned() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();
        assert!(store.current_login_claim().unwrap().is_some());

        store.set_enabled("wisp", false).unwrap();
        assert_eq!(claim_rows(&store), 1, "the row is untouched...");
        assert!(
            store.current_login_claim().unwrap().is_none(),
            "...but a disabled account must not be served"
        );

        // Re-enabling brings the same claim back - the row was never destroyed.
        store.set_enabled("wisp", true).unwrap();
        assert_eq!(store.current_login_claim().unwrap().unwrap().account_id, wisp);
    }

    /// Staking for a disabled account is refused rather than written. Otherwise the caller
    /// would get a `LoginClaim` back describing something `current_login_claim` can never
    /// return.
    #[test]
    fn staking_for_a_disabled_or_missing_account_is_refused_and_writes_nothing() {
        let (store, wisp, other) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();

        store.set_enabled("wisp_alt", false).unwrap();
        assert!(matches!(
            store.stake_login_claim(other, &token, LOGIN_CLAIM_TTL_SECS),
            Err(StoreError::NoSuchAccount { .. })
        ));
        assert!(matches!(
            store.stake_login_claim(9_999_999, &token, LOGIN_CLAIM_TTL_SECS),
            Err(StoreError::NoSuchAccount { .. })
        ));

        // The refusals wrote nothing and did not destroy the claim that was already there.
        assert_eq!(claim_rows(&store), 1);
        assert_eq!(store.current_login_claim().unwrap().unwrap().account_id, wisp);
    }

    /// **The raw token is never in the database.** The standing constraint, asserted rather
    /// than trusted: only its SHA-256 is stored, and the stored value is checked against
    /// [`hash_token`] so "absent" cannot be satisfied by storing nothing at all.
    #[test]
    fn the_raw_token_is_never_stored() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();

        let stored: String = store
            .conn()
            .query_row("SELECT token_hash FROM login_claims", [], |row| row.get(0))
            .unwrap();
        assert_ne!(stored, token);
        assert!(!stored.contains(&token), "the token must not appear even as a substring");
        assert_eq!(stored, hash_token(&token), "and what IS stored is the hash of it");
        assert!(!token.is_empty(), "a blank token would make the assertions above vacuous");
    }

    /// Clearing empties the table, and the answer goes with it.
    #[test]
    fn clearing_removes_every_claim() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();
        assert_eq!(claim_rows(&store), 1);

        assert_eq!(store.clear_login_claims().unwrap(), 1, "one row went");
        assert_eq!(claim_rows(&store), 0);
        assert!(
            store.current_login_claim().unwrap().is_none(),
            "the next connection falls back to the configured account"
        );
        // Clearing again is a no-op, not an error or a second phantom row.
        assert_eq!(store.clear_login_claims().unwrap(), 0);
    }

    /// Deleting the account takes its claim with it - `ON DELETE CASCADE`, which only works
    /// because `Store::init` turns foreign keys on.
    ///
    /// The row count is asserted **before** the delete as well: "no orphans" is also what a
    /// failed insert looks like.
    #[test]
    fn deleting_the_account_removes_its_claim() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();
        assert_eq!(claim_rows(&store), 1);

        store.conn().execute("DELETE FROM accounts WHERE id = ?1", [wisp]).unwrap();
        assert_eq!(claim_rows(&store), 0, "ON DELETE CASCADE should have taken it");
        assert!(store.current_login_claim().unwrap().is_none());
    }

    /// The claim survives a reopen, and the schema runs twice without complaint.
    ///
    /// An in-memory store cannot catch a non-idempotent schema: it is a fresh database every
    /// time, so the second open never happens. This needs a real file for that reason - the
    /// same shape as `skillpoints::tests::the_ledger_survives_a_reopen`.
    #[test]
    fn a_claim_survives_a_reopen() {
        let dir = std::env::temp_dir().join(format!("maplecw-claims-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("claims.db");
        let _ = std::fs::remove_file(&path);
        for suffix in ["-wal", "-shm"] {
            let _ = std::fs::remove_file(path.with_extension(format!("db{suffix}")));
        }

        let wisp;
        {
            let store = Store::open(&path).unwrap();
            wisp = store.create_account("wisp", "correct horse battery").unwrap();
            let token = login(&store, "wisp");
            store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();
        }

        let again = Store::open(&path).expect("the schema runs on every open and must be a no-op");
        let live = again.current_login_claim().unwrap().expect("the claim persisted");
        assert_eq!(live.account_id, wisp);
        assert_eq!(live.account_name, "wisp");
        drop(again);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
