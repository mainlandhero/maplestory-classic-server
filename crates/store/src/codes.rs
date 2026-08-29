//! **Single-use codes: one to create an account, one to reset an account's password.**
//!
//! # What the owner asked for
//!
//! The owner, 2026-08-29: *"we also need to support registering an account. We also don't want
//! anyone to register, so I want to be able to generate a one time code to pass to other
//! clients so that they can register an account in our launcher. The launcher should also be
//! able to change passwords provided that they know their existing password or they have
//! obtained a recovery code from me the administrator."*
//!
//! So registration is invite-only, and a password reset has two doors: the old password (which
//! is [`Store::authenticate`] plus [`Store::set_password`], and is nothing to do with this
//! module) or an administrator-minted recovery code (which is).
//!
//! # Two kinds of code, and two tables. Why not one table with a `kind` column
//!
//! An **invite** authorises *creating an account*. It belongs to nobody, because the account
//! does not exist yet. A **recovery code** authorises *resetting one particular account's
//! password*. It names an account. They are not the same object with a label on it, and the
//! schema is where that should be said:
//!
//! * `recovery_codes.account_id` is `NOT NULL REFERENCES accounts(id) ON DELETE CASCADE`.
//!   Merging the tables makes that column nullable, and the moment it is nullable the database
//!   can no longer refuse a recovery code that names no account. That constraint is what lets
//!   [`Store::redeem_recovery_code`] read an `i64` out of the row instead of an `Option<i64>`
//!   it would have to decide what to do with.
//! * The **cross-kind confusion becomes impossible rather than guarded against.** With one
//!   table, every statement in this file needs `AND kind = 'invite'` or `AND kind = 'recovery'`
//!   appended, and forgetting it *once* means a recovery code registers a new account - a
//!   privilege escalation out of a missing predicate. `CLAUDE.md` has a whole section on a
//!   guard whose answer is ignored, and the cheapest guard is the one nobody has to write. With
//!   two tables, `redeem_invite_code` cannot see a recovery row at all;
//!   `an_invite_code_cannot_be_redeemed_as_a_recovery_code` is the regression test for it.
//! * The two have different TTLs, different return types and different refusal conditions. The
//!   only thing they share is the minting and the hashing, and those are free functions here.
//!
//! The cost is that [`Store::live_code_counts`] runs two `COUNT(*)`s instead of a `GROUP BY`.
//! That is the entire downside.
//!
//! # The plaintext exists exactly once, in the return value of a `create_*`
//!
//! The database stores **only the SHA-256 of the code** - [`crate::session::hash_token`], the
//! same standing constraint that governs session tokens, and for the same reason: a stolen
//! database must not yield working codes. There is deliberately **no API in this module that
//! reads a code back out**, and there cannot be one, because the plaintext is not there to
//! read. [`Store::live_code_counts`] returns counts and not rows for exactly that reason: an
//! administrator who has lost a code mints another one.
//!
//! [`NewCode`] implements `Debug` by hand and prints `<redacted>` where the code is, so a
//! `{:?}` in a log line cannot be how the plaintext escapes. That is a deviation from
//! [`crate::session::NewSession`], which derives `Debug`; the difference is that a code is
//! meant to be read aloud and pasted around by a person, so it will pass through more hands.
//!
//! # Single use, and the check and the consume are the same statement
//!
//! A redeem that reads "unused" and then writes "used" in two statements has a window between
//! them. Two launchers - and the launcher, the login server and the admin CLI are **separate
//! processes sharing only the database file**, the same arrangement [`crate::claims`] and
//! `migrations` rest on - can both read "unused", and one invite registers two accounts.
//!
//! So the guard lives entirely inside the `WHERE` clause of the `UPDATE` that consumes the row:
//!
//! ```sql
//! UPDATE invite_codes SET used_at = ?2
//!  WHERE code_hash = ?1 AND used_at IS NULL AND expires_at > ?2
//! ```
//!
//! The number of rows that statement changed **is** the answer. There is no window, because
//! there is no read. `expires_at > ?2` is in the same predicate, so expiry is enforced on the
//! redeem and not merely on the listing - an expired code that is never listed is still a code
//! somebody may type.
//!
//! [`Store::redeem_recovery_code`] needs the account id as well, which is a second statement,
//! so it runs both inside one `IMMEDIATE` transaction. `IMMEDIATE` takes the write lock at
//! `BEGIN` rather than on first write, so no other process can be inside a write while this one
//! decides; and the `SELECT` reads a row this transaction has already claimed. The guard is
//! still only in the `UPDATE`.
//!
//! # Where the all-or-nothing stops, which is a support call if nobody says so
//!
//! A redeem is atomic **with respect to its own row** and nothing else. It commits, and then
//! the caller creates the account or sets the password. If that later work fails, **the code is
//! spent and produced nothing.**
//!
//! That is the deliberate choice, not an oversight. The alternative ordering - create the
//! account first, redeem afterwards - fails the other way, and the other way is worse: it can
//! create an account against a code that turns out to be already spent. Registration should
//! fail closed. The remedy for the failure this module does have is that the administrator
//! mints another invite, which costs one command; the remedy for the other one is deleting an
//! account somebody may already be using.
//!
//! (The genuinely correct shape is one transaction spanning the redeem *and* the account
//! creation. It is not available from here: `Store` serialises on a single non-reentrant
//! `Mutex<Connection>`, so calling `Store::create_account` while holding a transaction on that
//! connection deadlocks. Doing it properly means a combined method in `db.rs`, which is not
//! this file.)
//!
//! # NOT WIRED: `Store::init` does not call [`create_tables`]
//!
//! Every other module's `create_tables` has a line in `db::Store::init`. This one does not,
//! because `db.rs` belongs to the coordinator this session and was off limits. **The
//! belt-and-braces ensure at the top of each entry point below is currently the only thing
//! creating these tables**, which is the "Built is not wired" failure `CLAUDE.md` describes,
//! caught early. It works - `a_store_with_no_codes_answers_rather_than_failing` is the proof
//! that the read path makes its own tables - but the line in `Store::init` should still be
//! added:
//!
//! ```ignore
//! crate::codes::create_tables(&conn)?;
//! ```
//!
//! The per-entry-point ensure stays even after that, for the reason `claims.rs` gives: these
//! are called from a launcher and an admin CLI where a "no such table" error is a dialog box,
//! not a crash, and the cost is one no-op `CREATE TABLE IF NOT EXISTS` per administrative
//! action.

use rand::RngCore;
use rusqlite::{Connection, OptionalExtension, TransactionBehavior};

use crate::db::Store;
use crate::error::{Result, StoreError};
use crate::session::hash_token;

/// A freshly minted code.
///
/// The plaintext exists **only** in this value, to be shown to the administrator once; the
/// database keeps a SHA-256. Nothing in this crate can hand it back afterwards.
#[derive(Clone)]
pub struct NewCode {
    /// The code to read out to the person who needs it, grouped with hyphens. Never store it.
    pub code: String,
    /// Unix seconds. The code stops working strictly after this instant - the redeem predicate
    /// is `expires_at > now`, so `expires_at == now` is already dead.
    pub expires_at: i64,
}

/// Redacted on purpose. A `{:?}` on a struct holding a live credential is how the credential
/// ends up in a log file; `crate::session::NewSession` derives `Debug` and this deliberately
/// does not. `a_new_code_does_not_print_its_plaintext` pins it.
impl std::fmt::Debug for NewCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewCode")
            .field("code", &"<redacted>")
            .field("expires_at", &self.expires_at)
            .finish()
    }
}

/// A week. An invite is handed to somebody who is not at the keyboard right now - passed on in
/// a chat message, read down a phone - so it has to survive their weekend.
pub const INVITE_TTL_SECS: i64 = 7 * 24 * 3600;

/// A day. A recovery code is minted in response to a person asking for it, so they are already
/// waiting; the short window is the point. Unlike an invite, this one can take over an existing
/// account, and the blast radius of a leaked one is somebody else's characters.
pub const RECOVERY_TTL_SECS: i64 = 24 * 3600;

/// The character set a code is drawn from: **30 characters**, digits `2`-`9` and the capital
/// letters except `I`, `L`, `O` and `U`.
///
/// This is typed by a human, off a screen or out of a chat window, so every pair that a person
/// confuses when reading is resolved by *removing both members* rather than by hoping:
/// `0`/`O`, `1`/`l`/`I`. Crockford's base32 solves the same problem the other way round - it
/// keeps `0` and `1` and maps `O`, `I`, `L` onto them when decoding - and that would work too,
/// at the cost of a normalisation table that has to agree with the alphabet forever. Deleting
/// the characters means there is nothing to map and nothing to keep in agreement. `U` is out
/// because it is what keeps a randomly generated string from occasionally spelling something
/// unfortunate.
///
/// `codes_contain_no_ambiguous_characters` asserts this against the ambiguous set rather than
/// trusting the string above, because the string above is a claim.
pub const CODE_ALPHABET: &str = "23456789ABCDEFGHJKMNPQRSTVWXYZ";

/// Characters in a code, not counting the hyphens.
///
/// # How much this is worth, stated plainly
///
/// 30 characters is `log2(30) = 4.907` bits each, so 16 of them is **about 78.5 bits**. A
/// session token is 32 bytes - **256 bits**. These are not equivalent and this module must not
/// be described as if they were: an invite code is roughly a third of a session token's
/// entropy.
///
/// It is still far more than enough for what it does. There is no offline attack - the hash is
/// SHA-256 of a high-entropy string, with nothing to guess at - so the only way in is to type
/// codes at a live redeem, and `2^78` guesses is not a thing that happens against a server on
/// one person's LAN. Note what that argument rests on, though: **this module does no rate
/// limiting.** The margin is what makes the missing rate limit harmless, so shortening a code
/// for convenience later is not a cosmetic change.
pub const CODE_CHARS: usize = 16;

/// Characters between hyphens. `XXXX-XXXX-XXXX-XXXX`.
const CODE_GROUP_LEN: usize = 4;

/// Mint a code from the OS CSPRNG.
///
/// The modulo is **rejection sampled**: 30 does not divide 256, so a plain `byte % 30` would
/// make the first 16 characters of the alphabet slightly likelier than the last 14. That is a
/// fraction of a bit and nobody would ever notice it, which is exactly why it is worth getting
/// right here rather than explaining later - the fix is one comparison.
fn mint_code() -> String {
    let alphabet = CODE_ALPHABET.as_bytes();
    let n = alphabet.len() as u32;
    // The largest multiple of `n` that fits in a byte: 8 * 30 = 240. Bytes at or above this are
    // thrown away, so every remaining byte maps to exactly one character.
    let limit = (256 / n) * n;

    let mut out = String::with_capacity(CODE_CHARS + CODE_CHARS / CODE_GROUP_LEN);
    let mut buf = [0u8; 64];
    let mut next = buf.len(); // forces a fill on the first pass
    let mut taken = 0usize;
    while taken < CODE_CHARS {
        if next == buf.len() {
            rand::rngs::OsRng.fill_bytes(&mut buf);
            next = 0;
        }
        let byte = u32::from(buf[next]);
        next += 1;
        if byte >= limit {
            continue;
        }
        if taken > 0 && taken.is_multiple_of(CODE_GROUP_LEN) {
            out.push('-');
        }
        out.push(alphabet[(byte % n) as usize] as char);
        taken += 1;
    }
    out
}

/// What a person actually typed, reduced to what was actually minted.
///
/// Upper-cases, then **drops every character that is not in the alphabet**. That covers the
/// hyphens this module puts in, the spaces a person puts in instead, and a trailing newline off
/// a paste. It also silently drops a typed `O` or `I` - correctly: those characters are not in
/// the alphabet, so a code containing one was never minted, and dropping them turns it into a
/// code of the wrong length that matches nothing. There is no lookalike mapping to keep in step
/// with the alphabet, which is the whole reason the ambiguous characters were removed rather
/// than aliased.
///
/// **This runs on both sides.** The hash stored by a `create_*` is the hash of the *normalised*
/// code, so a redeem hashing the normalised input can match it. If these ever diverge, every
/// redeem fails; `a_typed_code_is_accepted_in_lower_case_and_without_hyphens` is the check.
fn normalize_code(entered: &str) -> String {
    entered
        .chars()
        .flat_map(|c| c.to_uppercase())
        .filter(|c| CODE_ALPHABET.contains(*c))
        .collect()
}

/// Create both code tables. Idempotent, and intended to be called from `Store::init` on every
/// open beside `crate::claims::create_tables` - **but it is not, see the module docs.**
///
/// A plain `CREATE TABLE IF NOT EXISTS` is enough **because both tables are brand new**, the
/// same reasoning `crate::quest::create_tables`, `crate::skillpoints::create_tables` and
/// `crate::claims::create_tables` spell out. It is the opposite of a *column* added to a table
/// that already exists: `CREATE TABLE IF NOT EXISTS` does nothing at all to an existing table,
/// which is why `Store::add_meso_column` and friends need an explicit `PRAGMA table_info` guard
/// around an `ALTER`. Nothing here touches `accounts`.
pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        -- Authorises CREATING an account. Belongs to nobody: there is no account_id, because
        -- at the time this row is written the account does not exist. That absence is the
        -- reason this is a separate table from recovery_codes rather than a `kind` column.
        --
        -- code_hash is the SHA-256 of the normalised code. The code itself is never stored,
        -- here or anywhere else in this crate, and no query in this module selects a plaintext
        -- because there is none to select.
        --
        -- used_at is NULL until redeemed, and is set by the same UPDATE whose WHERE clause
        -- checks it. The row is kept rather than deleted so "already used" can be told from
        -- "never existed" when somebody asks why their code stopped working.
        CREATE TABLE IF NOT EXISTS invite_codes (
            code_hash  TEXT    PRIMARY KEY,
            created_at INTEGER NOT NULL,
            expires_at INTEGER NOT NULL,
            used_at    INTEGER
        );

        -- The liveness predicate, which is the only filter both the redeem and the count use.
        CREATE INDEX IF NOT EXISTS idx_invite_codes_live
            ON invite_codes(used_at, expires_at);

        -- Authorises resetting ONE account's password. NOT NULL account_id is the difference
        -- that makes this a table of its own: a recovery code that names no account is not a
        -- degenerate case to handle at read time, it is a row the database refuses to hold.
        --
        -- ON DELETE CASCADE, so deleting an account cannot leave a live code pointing at a
        -- vanished id. It only works because Store::init turns foreign keys on.
        CREATE TABLE IF NOT EXISTS recovery_codes (
            code_hash  TEXT    PRIMARY KEY,
            account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
            created_at INTEGER NOT NULL,
            expires_at INTEGER NOT NULL,
            used_at    INTEGER
        );

        CREATE INDEX IF NOT EXISTS idx_recovery_codes_live
            ON recovery_codes(used_at, expires_at);
        CREATE INDEX IF NOT EXISTS idx_recovery_codes_account
            ON recovery_codes(account_id);
        "#,
    )?;
    Ok(())
}

impl Store {
    /// **Mint an invite: one account creation, by whoever holds the code.**
    ///
    /// Pass [`INVITE_TTL_SECS`] unless there is a reason not to. A `ttl_secs` of `0` or less
    /// writes a row that is already expired and can never be redeemed, which is useful in tests
    /// and is otherwise a long way of writing "no invite".
    ///
    /// The returned [`NewCode::code`] is the **only** copy. Show it to the administrator and
    /// let it go; there is no API here that can produce it again.
    ///
    /// This does not check who is asking. Deciding that only an administrator may call it is
    /// the caller's job - an admin CLI run by the person holding the database, or a launcher
    /// path gated some other way - and this module would be lying if it implied otherwise.
    pub fn create_invite_code(&self, ttl_secs: i64) -> Result<NewCode> {
        let code = mint_code();
        // Hash the normalised form, because that is what a redeem will hash.
        let code_hash = hash_token(&normalize_code(&code));
        let now = Store::now();
        // Saturating, so an absurd ttl cannot wrap the expiry into the past and mint a code
        // that is dead the instant it is written.
        let expires_at = now.saturating_add(ttl_secs);

        let conn = self.conn();
        create_tables(&conn)?;
        conn.execute(
            "INSERT INTO invite_codes (code_hash, created_at, expires_at, used_at)
                  VALUES (?1, ?2, ?3, NULL)",
            rusqlite::params![code_hash, now, expires_at],
        )?;
        Ok(NewCode { code, expires_at })
    }

    /// **Consume an invite. Single use, atomic.** `Ok(false)` if unknown, expired or already
    /// used.
    ///
    /// The three refusals are deliberately indistinguishable to the caller. There is nothing
    /// useful a launcher can do differently with them, and telling the far end which of the
    /// three it hit is telling it that a code exists.
    ///
    /// # The atomicity, and where it stops
    ///
    /// The check and the consume are one `UPDATE`: `used_at IS NULL` is in the same `WHERE`
    /// clause that sets `used_at`, so the rows-changed count *is* the answer and there is no
    /// window in which two processes both see "unused". Two callers racing one invite means one
    /// gets `true` and the other gets `false`; there is no third outcome. A single statement is
    /// its own transaction, which is why this needs no explicit `BEGIN`.
    ///
    /// **It is atomic with respect to this row only.** When it returns `true` the invite is
    /// spent and committed, and creating the account has not happened yet - so a failure in
    /// `Store::create_account` afterwards leaves a burnt invite and no account. See the module
    /// docs for why that is the right direction to fail in; the remedy is minting another
    /// invite.
    pub fn redeem_invite_code(&self, code: &str) -> Result<bool> {
        let entered = normalize_code(code);
        if entered.is_empty() {
            // Nothing typed, or nothing typed that could ever have been minted. Refused here
            // rather than hashed, so an empty string cannot go looking for a row.
            return Ok(false);
        }
        let code_hash = hash_token(&entered);
        let now = Store::now();

        let conn = self.conn();
        create_tables(&conn)?;
        let consumed = conn.execute(
            "UPDATE invite_codes
                SET used_at = ?2
              WHERE code_hash = ?1 AND used_at IS NULL AND expires_at > ?2",
            rusqlite::params![code_hash, now],
        )?;
        // code_hash is the primary key, so this is 0 or 1 and never more.
        Ok(consumed == 1)
    }

    /// **Mint a recovery code for one account's password reset.**
    ///
    /// Pass [`RECOVERY_TTL_SECS`] unless there is a reason not to; `0` or less is already
    /// expired, as above.
    ///
    /// # Refusals
    ///
    /// An account that does not exist is refused with [`StoreError::NoSuchAccount`] rather than
    /// given a code. Minting anyway would hand back a `NewCode` describing something
    /// [`Store::redeem_recovery_code`] can never honour - a return value that says "done" about
    /// something that will never happen, which is the same reasoning `Store::stake_login_claim`
    /// gives for the same refusal. Failing at the administrator's console, where a person is
    /// watching, is the cheap end of that trade; failing at the far end a week later, in a
    /// launcher belonging to somebody who cannot see why, is the expensive end.
    ///
    /// A **disabled** account is *not* refused, and that is a decision rather than an omission.
    /// `stake_login_claim` refuses one because `current_login_claim` filters disabled accounts
    /// out, so the claim genuinely could not be honoured. Nothing of the sort applies here:
    /// `Store::set_password` works on a disabled account today, so the reset this code
    /// authorises would really happen. Nor is it a lockout bypass - resetting the password of a
    /// disabled account still leaves it disabled and unable to log in. If that should change,
    /// it is one `AND enabled = 1` in the lookup below plus a test.
    ///
    /// `account_name` is matched against `accounts.name` only, which is `COLLATE NOCASE`, so
    /// case is not a way to miss the account. It is deliberately **not**
    /// `Store::get_account_by_identity`, which would also accept an email: the parameter is
    /// named `account_name` and widening a contract quietly is worse than not widening it. If
    /// an administrator should be able to say the email instead, that is a one-line change to
    /// the `WHERE` and it should be made on purpose.
    pub fn create_recovery_code(&self, account_name: &str, ttl_secs: i64) -> Result<NewCode> {
        let code = mint_code();
        let code_hash = hash_token(&normalize_code(&code));
        let now = Store::now();
        let expires_at = now.saturating_add(ttl_secs);

        let mut conn = self.conn();
        create_tables(&conn)?;
        // One transaction, so the account cannot be deleted between the lookup and the insert
        // and leave this returning a code whose row was never written. (The foreign key would
        // also catch that, with a far less legible error.)
        let tx = conn.transaction()?;

        let account_id: Option<i64> = tx
            .query_row(
                "SELECT id FROM accounts WHERE name = ?1",
                rusqlite::params![account_name],
                |row| row.get(0),
            )
            .optional()?;
        // The refusal drops the transaction unread, so a refused mint writes nothing at all.
        let Some(account_id) = account_id else {
            return Err(StoreError::NoSuchAccount { name: account_name.to_string() });
        };

        tx.execute(
            "INSERT INTO recovery_codes (code_hash, account_id, created_at, expires_at, used_at)
                  VALUES (?1, ?2, ?3, ?4, NULL)",
            rusqlite::params![code_hash, account_id, now, expires_at],
        )?;
        tx.commit()?;

        Ok(NewCode { code, expires_at })
    }

    /// **Consume a recovery code and return the account id it authorises.** `Ok(None)` if
    /// unknown, expired or already used - indistinguishable, for the reason
    /// [`Store::redeem_invite_code`] gives.
    ///
    /// An invite code passed here is `None`, and a recovery code passed to
    /// [`Store::redeem_invite_code`] is `false`. That is not a check anybody wrote; the rows
    /// live in different tables, so neither query can see the other's rows at all.
    ///
    /// # The atomicity, and where it stops
    ///
    /// The guard is the same one statement: `used_at IS NULL AND expires_at > ?2` sits in the
    /// `WHERE` of the `UPDATE` that sets `used_at`, so the rows-changed count is the answer and
    /// there is no read-then-write window.
    ///
    /// Reading the account id back is a second statement, so both run inside one **`IMMEDIATE`**
    /// transaction: `IMMEDIATE` takes the write lock at `BEGIN` instead of on first write, so
    /// another process cannot be part-way through its own redeem while this one decides, and
    /// the `SELECT` reads a row this transaction has already claimed. The refusal path commits
    /// nothing - the transaction is dropped, which rolls it back.
    ///
    /// **All-or-nothing applies to this row only.** On `Some(id)` the code is spent and
    /// committed before `Store::set_password` is called, so a failure there burns the code
    /// without changing the password. The administrator mints another one; see the module docs
    /// for why this direction rather than the other.
    pub fn redeem_recovery_code(&self, code: &str) -> Result<Option<i64>> {
        let entered = normalize_code(code);
        if entered.is_empty() {
            return Ok(None);
        }
        let code_hash = hash_token(&entered);
        let now = Store::now();

        let mut conn = self.conn();
        create_tables(&conn)?;
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;

        let consumed = tx.execute(
            "UPDATE recovery_codes
                SET used_at = ?2
              WHERE code_hash = ?1 AND used_at IS NULL AND expires_at > ?2",
            rusqlite::params![code_hash, now],
        )?;
        if consumed == 0 {
            // Unknown, expired or already used. Nothing was written; the drop rolls back.
            return Ok(None);
        }

        // Safe to read now: this transaction holds the write lock and has already claimed the
        // row, so nothing can have changed it. The account is guaranteed to exist - account_id
        // is NOT NULL with a foreign key, and ON DELETE CASCADE would have taken the row with
        // the account.
        let account_id: i64 = tx.query_row(
            "SELECT account_id FROM recovery_codes WHERE code_hash = ?1",
            rusqlite::params![code_hash],
            |row| row.get(0),
        )?;
        tx.commit()?;

        Ok(Some(account_id))
    }

    /// **How many live codes of each kind: `(invites, recovery)`.** For the admin CLI.
    ///
    /// Live means the same thing here that it means to a redeem - unused and unexpired - and it
    /// is the *same predicate*, deliberately. A count that filtered differently from the thing
    /// it is counting would be a listing that disagrees with reality, and this project has paid
    /// for that shape before.
    ///
    /// The tuple order is `(invites, recovery)`, matching the order the module talks about
    /// them; `live_code_counts_use_the_same_liveness_the_redeem_does` moves the two
    /// independently so a swapped pair cannot pass.
    ///
    /// This returns **counts, not rows**, and there is no sibling that returns rows. A listing
    /// could only ever show hashes, and an administrator who has lost a code mints another.
    pub fn live_code_counts(&self) -> Result<(usize, usize)> {
        let now = Store::now();
        let conn = self.conn();
        create_tables(&conn)?;

        let invites: i64 = conn.query_row(
            "SELECT COUNT(*) FROM invite_codes WHERE used_at IS NULL AND expires_at > ?1",
            rusqlite::params![now],
            |row| row.get(0),
        )?;
        // No join to `accounts`. It would be harmless - ON DELETE CASCADE means there are no
        // orphans - but it would also be a predicate the redeem does not have, and the two
        // must agree.
        let recovery: i64 = conn.query_row(
            "SELECT COUNT(*) FROM recovery_codes WHERE used_at IS NULL AND expires_at > ?1",
            rusqlite::params![now],
            |row| row.get(0),
        )?;

        Ok((invites as usize, recovery as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store_with_accounts() -> (Store, i64, i64) {
        let store = Store::open_in_memory().unwrap();
        let wisp = store.create_account("wisp", "correct horse battery").unwrap();
        let other = store.create_account("wisp_alt", "correct horse battery").unwrap();
        (store, wisp, other)
    }

    fn invite_rows(store: &Store) -> i64 {
        store
            .conn()
            .query_row("SELECT COUNT(*) FROM invite_codes", [], |row| row.get(0))
            .unwrap()
    }

    fn recovery_rows(store: &Store) -> i64 {
        store
            .conn()
            .query_row("SELECT COUNT(*) FROM recovery_codes", [], |row| row.get(0))
            .unwrap()
    }

    /// Every value in every column of a table, rendered as text.
    ///
    /// `SELECT *` and `column_count()` rather than naming columns, so this cannot be defeated by
    /// a column being added later that nobody thought to check - which is the exact way a
    /// "the secret is not stored" test rots into a test of one column.
    fn every_stored_value(store: &Store, table: &str) -> Vec<String> {
        let conn = store.conn();
        let mut stmt = conn.prepare(&format!("SELECT * FROM {table}")).unwrap();
        let columns = stmt.column_count();
        let mut values = Vec::new();
        let mut rows = stmt.query([]).unwrap();
        while let Some(row) = rows.next().unwrap() {
            for i in 0..columns {
                let value: rusqlite::types::Value = row.get(i).unwrap();
                values.push(format!("{value:?}"));
            }
        }
        values
    }

    /// **The positive control, and the wiring check.**
    ///
    /// Nothing has called [`create_tables`] on a fresh store - `Store::init` does *not* call it,
    /// see the module docs - so reaching a clean `(0, 0)` here proves the entry-point ensure is
    /// what builds the tables. If this starts failing with "no such table", that ensure has
    /// been removed and there is nothing behind it.
    ///
    /// It is also the control for every test below: they assert `true` / `Some(..)`, and an
    /// empty answer from a broken query is indistinguishable from "refused" without this.
    #[test]
    fn a_store_with_no_codes_answers_rather_than_failing() {
        let (store, _, _) = store_with_accounts();
        assert_eq!(store.live_code_counts().unwrap(), (0, 0));
        assert_eq!(invite_rows(&store), 0);
        assert_eq!(recovery_rows(&store), 0);
        // And redeeming against an empty table refuses rather than erroring.
        assert!(!store.redeem_invite_code("ABCD-EFGH-JKMN-PQRS").unwrap());
        assert_eq!(store.redeem_recovery_code("ABCD-EFGH-JKMN-PQRS").unwrap(), None);
    }

    /// An invite mints, counts as live, redeems once, and stops counting.
    ///
    /// All four effects are asserted rather than just the redeem: a store that returned `true`
    /// without writing `used_at` would pass a redeem-only test.
    #[test]
    fn an_invite_code_round_trips() {
        let (store, _, _) = store_with_accounts();

        let minted = store.create_invite_code(INVITE_TTL_SECS).unwrap();
        assert_eq!(invite_rows(&store), 1);
        assert_eq!(store.live_code_counts().unwrap(), (1, 0));
        assert!(minted.expires_at > Store::now(), "a fresh invite must be live");

        assert!(store.redeem_invite_code(&minted.code).unwrap(), "the invite must redeem");
        assert_eq!(store.live_code_counts().unwrap(), (0, 0), "and stop being live");
        assert_eq!(invite_rows(&store), 1, "the row is kept, marked used, not deleted");
    }

    /// **Single use: the second redeem of the same invite must fail.**
    ///
    /// This is the property the whole module exists for - the owner: *"We also don't want anyone to
    /// register"* - and a broken one means one invite registers as many accounts as anybody
    /// likes. The third attempt is there because "the second fails" could also be satisfied by
    /// an alternating bug.
    #[test]
    fn redeeming_an_invite_twice_fails_the_second_time() {
        let (store, _, _) = store_with_accounts();
        let minted = store.create_invite_code(INVITE_TTL_SECS).unwrap();

        assert!(store.redeem_invite_code(&minted.code).unwrap(), "first use is the real one");
        assert!(!store.redeem_invite_code(&minted.code).unwrap(), "second use must be refused");
        assert!(!store.redeem_invite_code(&minted.code).unwrap(), "and so must the third");

        // A second, independent invite still works - so the refusals above are about this code
        // and not about the table having become unusable.
        let another = store.create_invite_code(INVITE_TTL_SECS).unwrap();
        assert!(store.redeem_invite_code(&another.code).unwrap());
    }

    /// The same property for a recovery code: a reset code that can be replayed is a permanent
    /// key to somebody's account.
    #[test]
    fn redeeming_a_recovery_code_twice_fails_the_second_time() {
        let (store, wisp, _) = store_with_accounts();
        let minted = store.create_recovery_code("wisp", RECOVERY_TTL_SECS).unwrap();

        assert_eq!(store.redeem_recovery_code(&minted.code).unwrap(), Some(wisp));
        assert_eq!(store.redeem_recovery_code(&minted.code).unwrap(), None, "second use refused");
        assert_eq!(store.redeem_recovery_code(&minted.code).unwrap(), None, "and the third");

        let another = store.create_recovery_code("wisp", RECOVERY_TTL_SECS).unwrap();
        assert_eq!(store.redeem_recovery_code(&another.code).unwrap(), Some(wisp));
    }

    /// An expired invite is refused **on the redeem**, not merely left out of a listing.
    ///
    /// The row is asserted to exist first, which is what separates "expired" from "never
    /// written"; and a live invite is redeemed on the same store afterwards, without which
    /// every assertion here would also pass against a redeem that never matches anything.
    #[test]
    fn an_expired_invite_is_refused() {
        let (store, _, _) = store_with_accounts();

        let dead = store.create_invite_code(0).unwrap();
        assert_eq!(invite_rows(&store), 1, "the row exists...");
        assert!(!store.redeem_invite_code(&dead.code).unwrap(), "...and a ttl of 0 is dead");
        assert_eq!(store.live_code_counts().unwrap(), (0, 0));

        let long_dead = store.create_invite_code(-3600).unwrap();
        assert!(!store.redeem_invite_code(&long_dead.code).unwrap());

        let live = store.create_invite_code(INVITE_TTL_SECS).unwrap();
        assert!(store.redeem_invite_code(&live.code).unwrap(), "the control must redeem");
    }

    /// The same, for recovery codes.
    #[test]
    fn an_expired_recovery_code_is_refused() {
        let (store, wisp, _) = store_with_accounts();

        let dead = store.create_recovery_code("wisp", 0).unwrap();
        assert_eq!(recovery_rows(&store), 1, "the row exists...");
        assert_eq!(store.redeem_recovery_code(&dead.code).unwrap(), None, "...and is expired");

        let long_dead = store.create_recovery_code("wisp", -3600).unwrap();
        assert_eq!(store.redeem_recovery_code(&long_dead.code).unwrap(), None);
        assert_eq!(store.live_code_counts().unwrap(), (0, 0));

        let live = store.create_recovery_code("wisp", RECOVERY_TTL_SECS).unwrap();
        assert_eq!(store.redeem_recovery_code(&live.code).unwrap(), Some(wisp));
    }

    /// A code nobody minted is refused, and so is an empty or junk string.
    ///
    /// A live code exists on the store throughout, so "refused" here cannot be an empty table
    /// wearing a disguise.
    #[test]
    fn an_unknown_code_is_refused() {
        let (store, _, _) = store_with_accounts();
        let real_invite = store.create_invite_code(INVITE_TTL_SECS).unwrap();
        let real_recovery = store.create_recovery_code("wisp", RECOVERY_TTL_SECS).unwrap();

        for junk in ["", "   ", "-", "ABCD-EFGH-JKMN-PQRS", "not a code at all", "0000-0000"] {
            assert!(!store.redeem_invite_code(junk).unwrap(), "invite accepted {junk:?}");
            assert_eq!(
                store.redeem_recovery_code(junk).unwrap(),
                None,
                "recovery accepted {junk:?}"
            );
        }

        // Nothing above consumed the real codes.
        assert_eq!(store.live_code_counts().unwrap(), (1, 1));
        assert!(store.redeem_invite_code(&real_invite.code).unwrap());
        assert!(store.redeem_recovery_code(&real_recovery.code).unwrap().is_some());
    }

    /// A recovery code returns the id of the account it names, and not some other account's.
    ///
    /// Two accounts exist, so "returns the right id" is a claim that can come back false; with
    /// one account it would pass against a function that returned whatever id it found.
    #[test]
    fn a_recovery_code_returns_the_account_it_names() {
        let (store, wisp, other) = store_with_accounts();

        let for_wisp = store.create_recovery_code("wisp", RECOVERY_TTL_SECS).unwrap();
        let for_other = store.create_recovery_code("wisp_alt", RECOVERY_TTL_SECS).unwrap();
        assert_ne!(wisp, other, "the two ids must differ for this test to mean anything");

        assert_eq!(store.redeem_recovery_code(&for_other.code).unwrap(), Some(other));
        assert_eq!(store.redeem_recovery_code(&for_wisp.code).unwrap(), Some(wisp));

        // The account name is COLLATE NOCASE, so this is the same account.
        let by_case = store.create_recovery_code("WISP", RECOVERY_TTL_SECS).unwrap();
        assert_eq!(store.redeem_recovery_code(&by_case.code).unwrap(), Some(wisp));
    }

    /// A recovery code for an account that does not exist is refused at creation, and writes
    /// nothing.
    ///
    /// Handing back a code that can never work is a return value that says "done" about
    /// something that will never happen - the same reasoning `stake_login_claim` uses.
    #[test]
    fn a_recovery_code_for_a_missing_account_is_refused_at_creation() {
        let (store, wisp, _) = store_with_accounts();
        // A live code first, so "writes nothing" is measured against a table that works.
        store.create_recovery_code("wisp", RECOVERY_TTL_SECS).unwrap();

        assert!(matches!(
            store.create_recovery_code("nobody", RECOVERY_TTL_SECS),
            Err(StoreError::NoSuchAccount { .. })
        ));
        assert!(matches!(
            store.create_recovery_code("", RECOVERY_TTL_SECS),
            Err(StoreError::NoSuchAccount { .. })
        ));

        assert_eq!(recovery_rows(&store), 1, "the refusals wrote no rows");
        assert_eq!(store.live_code_counts().unwrap(), (0, 1));

        // A disabled account is deliberately NOT refused: set_password works on one, so the
        // reset this authorises would really happen. See create_recovery_code's doc block.
        store.set_enabled("wisp", false).unwrap();
        let minted = store.create_recovery_code("wisp", RECOVERY_TTL_SECS).unwrap();
        assert_eq!(store.redeem_recovery_code(&minted.code).unwrap(), Some(wisp));
    }

    /// **The plaintext code is never in the database**, in any column of either table.
    ///
    /// The standing constraint, asserted rather than trusted. Checked in three forms - as
    /// returned, normalised, and lower-cased - because a code that leaked through a different
    /// spelling has still leaked. What *is* stored is checked against [`hash_token`] as well,
    /// so "absent" cannot be satisfied by a store that writes nothing at all.
    #[test]
    fn the_plaintext_code_is_never_stored() {
        let (store, _, _) = store_with_accounts();
        let invite = store.create_invite_code(INVITE_TTL_SECS).unwrap();
        let recovery = store.create_recovery_code("wisp", RECOVERY_TTL_SECS).unwrap();

        for (table, minted) in [("invite_codes", &invite), ("recovery_codes", &recovery)] {
            let normalised = normalize_code(&minted.code);
            assert!(!normalised.is_empty(), "a blank code would make this test vacuous");

            for value in every_stored_value(&store, table) {
                for spelling in [&minted.code, &normalised, &minted.code.to_lowercase()] {
                    assert!(
                        !value.contains(spelling.as_str()),
                        "{table} stored {spelling:?} in {value}"
                    );
                }
            }

            // And what IS stored is the hash of it, so the absence above is not absence of
            // everything.
            let stored: String = store
                .conn()
                .query_row(&format!("SELECT code_hash FROM {table}"), [], |row| row.get(0))
                .unwrap();
            assert_eq!(stored, hash_token(&normalised));
            assert_ne!(stored, minted.code);
        }
    }

    /// Deleting an account takes its recovery codes with it - `ON DELETE CASCADE`, which only
    /// works because `Store::init` turns foreign keys on.
    ///
    /// The invite is there to prove the cascade is scoped: invites belong to nobody and must
    /// survive. The row counts are asserted **before** the delete as well, because "no orphans"
    /// is also what a failed insert looks like.
    #[test]
    fn deleting_the_account_removes_its_recovery_codes() {
        let (store, wisp, _) = store_with_accounts();
        let invite = store.create_invite_code(INVITE_TTL_SECS).unwrap();
        let for_wisp = store.create_recovery_code("wisp", RECOVERY_TTL_SECS).unwrap();
        store.create_recovery_code("wisp_alt", RECOVERY_TTL_SECS).unwrap();
        assert_eq!(recovery_rows(&store), 2);
        assert_eq!(store.live_code_counts().unwrap(), (1, 2));

        store.conn().execute("DELETE FROM accounts WHERE id = ?1", [wisp]).unwrap();

        assert_eq!(recovery_rows(&store), 1, "ON DELETE CASCADE should have taken one");
        assert_eq!(store.live_code_counts().unwrap(), (1, 1), "the other account keeps its own");
        assert_eq!(store.redeem_recovery_code(&for_wisp.code).unwrap(), None);
        assert!(store.redeem_invite_code(&invite.code).unwrap(), "invites belong to nobody");
    }

    /// **Neither redeem can see the other kind's rows.** This is the two-table design being
    /// load-bearing rather than tidy: with one table and a `kind` column, a forgotten
    /// `AND kind = ...` in one statement would let a recovery code register a new account.
    #[test]
    fn an_invite_code_cannot_be_redeemed_as_a_recovery_code() {
        let (store, _, _) = store_with_accounts();
        let invite = store.create_invite_code(INVITE_TTL_SECS).unwrap();
        let recovery = store.create_recovery_code("wisp", RECOVERY_TTL_SECS).unwrap();

        assert_eq!(store.redeem_recovery_code(&invite.code).unwrap(), None);
        assert!(!store.redeem_invite_code(&recovery.code).unwrap());

        // And the failed cross-redeems consumed nothing, so each still works as itself.
        assert_eq!(store.live_code_counts().unwrap(), (1, 1));
        assert!(store.redeem_invite_code(&invite.code).unwrap());
        assert!(store.redeem_recovery_code(&recovery.code).unwrap().is_some());
    }

    /// Two codes are never equal.
    ///
    /// 200 of them, and their hashes, because a mint that returned a constant would pass a
    /// two-sample test roughly never but a mint with a tiny period might. This is a smoke test
    /// for a wired-up CSPRNG, not a statistical one.
    #[test]
    fn two_codes_are_never_equal() {
        let mut seen = std::collections::HashSet::new();
        for _ in 0..200 {
            let code = mint_code();
            assert!(seen.insert(code.clone()), "mint_code repeated {code}");
        }
        assert_eq!(seen.len(), 200);

        // The same, through the store, so it is the real path being checked.
        let (store, _, _) = store_with_accounts();
        let a = store.create_invite_code(INVITE_TTL_SECS).unwrap();
        let b = store.create_invite_code(INVITE_TTL_SECS).unwrap();
        assert_ne!(a.code, b.code);
        assert_eq!(invite_rows(&store), 2, "two distinct hashes, so two rows");
    }

    /// **No ambiguous characters**, checked against the ambiguous set rather than against
    /// [`CODE_ALPHABET`] alone - the alphabet is a claim, and a test that only asserts "every
    /// character is in the alphabet" would keep passing if somebody put `O` back in it.
    ///
    /// 200 codes, because one 16-character sample could miss a character that appears one time
    /// in thirty.
    #[test]
    fn codes_contain_no_ambiguous_characters() {
        // The alphabet itself, first.
        for bad in ['0', 'O', '1', 'l', 'I', 'L', 'U', 'u'] {
            assert!(!CODE_ALPHABET.contains(bad), "the alphabet contains {bad}");
        }
        assert_eq!(
            CODE_ALPHABET.chars().collect::<std::collections::HashSet<_>>().len(),
            CODE_ALPHABET.len(),
            "a repeated character would skew the distribution and shrink the entropy"
        );
        assert_eq!(CODE_ALPHABET.len(), 30, "the entropy figure in the docs assumes 30");

        for _ in 0..200 {
            let code = mint_code();
            for c in code.chars() {
                assert!(
                    c == '-' || CODE_ALPHABET.contains(c),
                    "{code} contains {c}, which is not in the alphabet"
                );
            }
            assert_eq!(
                code.chars().filter(|c| *c != '-').count(),
                CODE_CHARS,
                "{code} is the wrong length"
            );
        }

        // The grouping, on a real one: four groups of four.
        let code = mint_code();
        let groups: Vec<&str> = code.split('-').collect();
        assert_eq!(groups.len(), CODE_CHARS / CODE_GROUP_LEN, "{code} is not grouped");
        assert!(groups.iter().all(|g| g.len() == CODE_GROUP_LEN), "{code} has an odd group");
    }

    /// A code typed by a person - lower case, spaces instead of hyphens, a stray newline off a
    /// paste - reaches the same row.
    ///
    /// This is the check that the normalisation on the mint side and the normalisation on the
    /// redeem side have not diverged. If they ever do, **every** redeem fails, and it fails
    /// identically to "wrong code", which is the sort of failure that gets diagnosed as a lost
    /// invite three times before anybody looks here.
    #[test]
    fn a_typed_code_is_accepted_in_lower_case_and_without_hyphens() {
        let (store, wisp, _) = store_with_accounts();

        /// One way a person might retype a code they were given.
        type Spelling = Box<dyn Fn(&str) -> String>;

        let spellings: Vec<Spelling> = vec![
            Box::new(|c: &str| c.to_string()),
            Box::new(|c: &str| c.to_lowercase()),
            Box::new(|c: &str| c.replace('-', "")),
            Box::new(|c: &str| c.replace('-', " ")),
            Box::new(|c: &str| format!("  {c}\r\n")),
            Box::new(|c: &str| c.to_lowercase().replace('-', "")),
        ];

        for spell in &spellings {
            let invite = store.create_invite_code(INVITE_TTL_SECS).unwrap();
            let typed = spell(&invite.code);
            assert!(store.redeem_invite_code(&typed).unwrap(), "invite refused {typed:?}");

            let recovery = store.create_recovery_code("wisp", RECOVERY_TTL_SECS).unwrap();
            let typed = spell(&recovery.code);
            assert_eq!(
                store.redeem_recovery_code(&typed).unwrap(),
                Some(wisp),
                "recovery refused {typed:?}"
            );
        }
    }

    /// The counts move independently, so a swapped tuple cannot pass; and used and expired codes
    /// drop out of both, with the same liveness the redeem uses.
    #[test]
    fn live_code_counts_use_the_same_liveness_the_redeem_does() {
        let (store, _, _) = store_with_accounts();
        assert_eq!(store.live_code_counts().unwrap(), (0, 0));

        // Two invites and one recovery: an asymmetric pair, so (2, 1) and (1, 2) differ.
        let first = store.create_invite_code(INVITE_TTL_SECS).unwrap();
        store.create_invite_code(INVITE_TTL_SECS).unwrap();
        store.create_recovery_code("wisp", RECOVERY_TTL_SECS).unwrap();
        assert_eq!(store.live_code_counts().unwrap(), (2, 1), "invites first, then recovery");

        // Used drops out.
        assert!(store.redeem_invite_code(&first.code).unwrap());
        assert_eq!(store.live_code_counts().unwrap(), (1, 1));

        // So does expired, without anything being deleted.
        store.create_invite_code(-1).unwrap();
        store.create_recovery_code("wisp", -1).unwrap();
        assert_eq!(store.live_code_counts().unwrap(), (1, 1), "expired rows are not live");
        assert_eq!(invite_rows(&store), 3, "and are still on disk");
        assert_eq!(recovery_rows(&store), 2);
    }

    /// `{:?}` on a [`NewCode`] must not print the code. A struct holding a live credential ends
    /// up in a log line eventually, and this is the cheapest place to stop it.
    #[test]
    fn a_new_code_does_not_print_its_plaintext() {
        let (store, _, _) = store_with_accounts();
        let minted = store.create_invite_code(INVITE_TTL_SECS).unwrap();

        let printed = format!("{minted:?}");
        assert!(!printed.contains(&minted.code), "Debug leaked the code: {printed}");
        assert!(printed.contains("redacted"));
        // The expiry is still printed - the point is redaction, not opacity.
        assert!(printed.contains(&minted.expires_at.to_string()));
        // And the code is still reachable through the field, or this would be useless.
        assert!(store.redeem_invite_code(&minted.code).unwrap());
    }

    /// Codes survive a reopen, and the schema runs twice without complaint.
    ///
    /// An in-memory store cannot catch a non-idempotent schema - it is a fresh database every
    /// time, so the second open never happens. This needs a real file for that reason, the same
    /// shape as `claims::tests::a_claim_survives_a_reopen`.
    #[test]
    fn codes_survive_a_reopen() {
        let dir = std::env::temp_dir().join(format!("maplecw-codes-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("codes.db");
        let _ = std::fs::remove_file(&path);
        for suffix in ["-wal", "-shm"] {
            let _ = std::fs::remove_file(path.with_extension(format!("db{suffix}")));
        }

        let wisp;
        let invite;
        let recovery;
        {
            let store = Store::open(&path).unwrap();
            wisp = store.create_account("wisp", "correct horse battery").unwrap();
            invite = store.create_invite_code(INVITE_TTL_SECS).unwrap();
            recovery = store.create_recovery_code("wisp", RECOVERY_TTL_SECS).unwrap();
            assert_eq!(store.live_code_counts().unwrap(), (1, 1));
        }

        let again = Store::open(&path).expect("the schema must be a no-op on the second open");
        assert_eq!(again.live_code_counts().unwrap(), (1, 1), "both codes persisted");
        assert!(again.redeem_invite_code(&invite.code).unwrap());
        assert_eq!(again.redeem_recovery_code(&recovery.code).unwrap(), Some(wisp));
        // And single use survives the reopen too - `used_at` is on disk, not in memory.
        assert!(!again.redeem_invite_code(&invite.code).unwrap());
        drop(again);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
