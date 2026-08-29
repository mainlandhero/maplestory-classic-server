//! **Which account the login server should serve a given game connection as.**
//!
//! # The problem this exists to solve
//!
//! The game socket carries no credentials. The client never sends a username - it sends a
//! login packet whose form is vestigial, and the server supplies both the result *and* the
//! account name. So `crates/login` was started with `--account NAME` and served **every**
//! connection as that one account, which means the machine could only ever play one account.
//!
//! A claim moves that decision out of the command line and into the database, where a GUI
//! launcher can set it. The launcher authenticates for real - argon2id, through
//! `crates/auth` - and then *stakes a claim*: "a client launch belongs to this account".
//!
//! # THE BUG THIS MODULE WAS REWRITTEN TO FIX
//!
//! The first version of this file kept **one global row**. `stake_login_claim` ran
//! `DELETE FROM login_claims` and inserted a single row, and `current_login_claim` returned
//! whatever that row said. Measured on 2026-08-29 through the real
//! `auth::AuthService::login` path, two accounts, one store:
//!
//! ```text
//! after otter: claim=Some("otter")
//! after owl:   claim=Some("owl")
//! login connection 1 served as Some("owl")
//! login connection 2 served as Some("owl")
//! ```
//!
//! So the second sign-in **evicted** the first, and *both* login connections were then served
//! as the second account - full character list, twelve-hour window. No race was needed and no
//! migration was involved. That is a worse and simpler impersonation hole than the migration
//! one, and it sat upstream of it.
//!
//! The table is now keyed **per launch** rather than per machine: one row per sign-in, keyed
//! by the SHA-256 of the session token that sign-in issued. A second sign-in adds a row; it
//! does not remove anybody else's.
//!
//! # This is still not authentication, and must never be reported as if it were
//!
//! `CLAUDE.md`: *"Nothing authenticates. The game socket carries no credentials."* That is
//! still true. A claim does not prove who is on the far end of the game socket; it decides
//! **which account a credential-less connection is served as**, using whatever the server can
//! observe about that connection. The strength of that decision is carried out of
//! [`Store::resolve_login_claim`] as [`ResolvedBy`], precisely so a caller's log line can say
//! how weak it was.
//!
//! # How a connection is matched to a claim
//!
//! [`ClaimEvidence`] is what a server can say about a connection. It is tried in strength
//! order and the first rule that identifies **exactly one** live claim wins:
//!
//! | # | evidence | [`ResolvedBy`] | strength |
//! |---|----------|----------------|----------|
//! | 1 | a session token the connection presented | [`ResolvedBy::Token`] | a credential; nothing on the wire presents one today |
//! | 2 | the OS-attributed owning process of the peer socket | [`ResolvedBy::LaunchPid`] | same machine only; separates two clients on one box |
//! | 3 | the source address, when it picks out exactly one claim | [`ResolvedBy::PeerAddress`] | separates two machines; useless for two clients on one |
//! | 4 | there is exactly one live claim in the whole database | [`ResolvedBy::SoleLiveClaim`] | not a discriminator at all - it is "there was only one answer" |
//!
//! and if none of them does, the answer is [`ClaimResolution::Ambiguous`], **not** a guess.
//!
//! **Rule 3 is why `peer` is here at all, and it does not contradict the owner's constraint.**
//! The owner, 2026-08-29: *"The login MapleCW Launcher needs to be able to potentially handle
//! multiple connections from the same IP as well, IP cannot be the sole discriminator."* It is
//! not the sole discriminator: two clients on one machine share an address, rule 3 finds two
//! matching claims, and it declines rather than picking one. Rule 2 is what separates those
//! two, and rule 2 is an address-independent fact about a process.
//!
//! # Ambiguity is refused, and refusing is the whole point
//!
//! [`ClaimResolution::Ambiguous`] exists because *"return the newest"* is exactly the bug
//! above wearing a different hat. With two live claims and nothing to tell the connections
//! apart, there is no honest answer, and a caller must fall back to its configured account
//! rather than serve one player as the other.
//!
//! That is a **deliberate degrade, not a refusal of the connection**: `CLAUDE.md`'s "always
//! answer" rule means an unanswered login freezes the client's entire UI. The caller answers,
//! with the fallback account, and logs which rule it used and why. See
//! [`ClaimResolution::why`] for the sentence.
//!
//! # A claim is NOT single use, and that is the whole design
//!
//! The obvious shape - a `consumed_at` column, or a `DELETE` on read - would be wrong, and
//! wrong in a way that reads on screen as data loss.
//!
//! **The client makes a second login connection in the same launch.** `CLAUDE.md` records it
//! from a capture: `login.log` had **two** `0x0010`s in one run, because "Log Out" and
//! "Choose another world" both drop the login socket and reconnect. A single-use claim would
//! be spent by the first connection, and the second would fall back to whatever `--account`
//! says - a *different* account, with a different character list. On screen that is not "the
//! claim expired", it is **"my characters vanished when I logged out"**, and it would be
//! reported as a character-deletion bug rather than as a session bug.
//!
//! So a claim is a **standing setting with an expiry**, read as many times as the client
//! cares to reconnect. `reading_the_claim_twice_serves_the_same_account` is the regression
//! test for exactly that, and it is named so nobody deletes it as redundant.
//!
//! What ends a claim is [`LOGIN_CLAIM_TTL_SECS`], [`Store::clear_login_claims`],
//! [`Store::clear_login_claims_for`], or a newer stake **carrying the same token**.
//!
//! # The token hash is now a key and a reader, not an audit column
//!
//! `token_hash` is the SHA-256 of the session token the launcher's sign-in issued - never the
//! token ([`crate::session::hash_token`], and the standing constraint behind it). The previous
//! version of this file described it as *"an audit trail waiting for a reader"*. It now has
//! two readers: it is the table's unique key, and [`ResolvedClaim::token_hash`] hands it to
//! the login server so a migration can be bound to **the claim this connection resolved to**
//! rather than to whichever claim happens to be newest.

use rusqlite::{Connection, OptionalExtension};

use crate::db::Store;
use crate::error::{Result, StoreError};
use crate::session::{hash_token, new_token};

/// A live claim: who a game connection resolved to this should be served as.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginClaim {
    pub account_id: i64,
    pub account_name: String,
    pub created_at: i64,
    pub expires_at: i64,
}

/// What a sign-in gets back: the claim, and the one handle that can bind a process to it.
///
/// # Why the launch id is not the session token
///
/// The launcher has not held a session token since sign-in moved over HTTP - `crates/auth`
/// stakes the claim itself and `crates/launcher/src/http.rs` deliberately drops the token,
/// with a test asserting it never leaves that module. The owner, 2026-08-29: *"everyone installs
/// this differently, on a client machine you won't have access to the project or the
/// database."*
///
/// But the launcher is the only thing that knows the client's process id, because it is what
/// starts the client. So it needs *some* handle to say "that process is my launch". Handing
/// back the session token would undo a deliberate decision and give a client machine a
/// credential it has no other use for.
///
/// A launch id is a second random 32-byte value whose only power is
/// [`Store::bind_launch_pid`]. It cannot be used to authenticate, cannot bind a migration, and
/// buys nothing that stealing the *process id itself* would not - the pid is not a secret and
/// the binding is only meaningful because the **server** independently attributes a connection
/// to a process. Only its SHA-256 is stored, the same standing constraint the token follows.
pub struct StakedClaim {
    pub claim: LoginClaim,
    /// Give this to the launcher; never log it, never store it.
    pub launch_id: String,
}

/// Redacted, because a `{:?}` of a struct is how a secret reaches a log without anybody
/// deciding to put it there.
impl std::fmt::Debug for StakedClaim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StakedClaim")
            .field("claim", &self.claim)
            .field("launch_id", &"<redacted>")
            .finish()
    }
}

/// What a server can say about the connection it is trying to resolve.
///
/// Every field is optional because **today the game socket carries none of them**: the client
/// sends no token (measured - 8510 trials against 115 hello bodies and 74 seeds, zero hits,
/// with the character id passing as a positive control), and its identity block is
/// per-machine rather than per-launch (two distinct `0x0073` bodies across 56 archived
/// launches, one of them the smoke test's synthetic one). The two facts a server *can*
/// observe on its own are the peer address and, for a local peer, the process that owns it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClaimEvidence {
    /// A session token the connection presented, if any. Only its SHA-256 is ever compared or
    /// stored; the plain value does not leave this struct.
    pub token: Option<String>,
    /// The process id that owns the far end of this connection, as the operating system
    /// reports it - see [`crate::peerowner::owning_pid`].
    ///
    /// **This is the discriminator that works for two clients on one machine.** It is not
    /// something the client asserts: it comes from the OS's TCP table keyed by the accepted
    /// socket's peer endpoint, so a process cannot claim to be another one. It is `None` for
    /// any peer that is not on this machine.
    pub launch_pid: Option<u32>,
    /// The connection's source address as the server observed it, e.g. `"127.0.0.1"`.
    ///
    /// Never the sole discriminator - see the module docs' table, rule 3.
    pub peer: Option<String>,
}

impl ClaimEvidence {
    /// A connection that presented nothing and could not be attributed to a process.
    ///
    /// This is what a caller has when it has not been wired to [`crate::peerowner`] yet, and
    /// it can only ever reach rule 4 - one live claim, or nothing.
    pub fn none() -> Self {
        Self::default()
    }

    pub fn with_token(token: impl Into<String>) -> Self {
        Self { token: Some(token.into()), ..Self::default() }
    }

    pub fn with_launch_pid(pid: u32) -> Self {
        Self { launch_pid: Some(pid), ..Self::default() }
    }

    pub fn from_peer(mut self, peer: impl Into<String>) -> Self {
        self.peer = Some(peer.into());
        self
    }

    pub fn and_launch_pid(mut self, pid: Option<u32>) -> Self {
        self.launch_pid = pid;
        self
    }

    /// True when nothing here can tell one connection from another.
    pub fn is_empty(&self) -> bool {
        self.token.is_none() && self.launch_pid.is_none() && self.peer.is_none()
    }
}

/// **Which rule matched**, and therefore how much the answer is worth.
///
/// Carried out of the store rather than discarded because `CLAUDE.md`'s standing complaint is
/// about guards whose answers are thrown away. A caller is expected to put this in its log
/// line: `SoleLiveClaim` and `Token` are the same type of value and wildly different
/// evidence, and a run should never be ambiguous about which one it got.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedBy {
    /// The connection presented a session token that hashes to this claim's key.
    Token,
    /// The operating system attributes the far end of this connection to the process this
    /// claim was bound to. Same machine only.
    LaunchPid,
    /// Exactly one live claim was staked from this source address.
    PeerAddress,
    /// **Nothing identified this connection.** There was exactly one live claim in the whole
    /// database, so it is the only answer there is. This is the pre-existing single-player
    /// behaviour and is not a discriminator.
    SoleLiveClaim,
}

impl ResolvedBy {
    /// A phrase for a log line, in the caller's sentence.
    pub fn describe(self) -> &'static str {
        match self {
            ResolvedBy::Token => "the session token this connection presented",
            ResolvedBy::LaunchPid => {
                "the process the operating system says owns this connection (same machine)"
            }
            ResolvedBy::PeerAddress => "the only claim staked from this address",
            ResolvedBy::SoleLiveClaim => {
                "the only live claim in the database - NOTHING identified this connection, so \
                 this is not a discriminator"
            }
        }
    }
}

/// A claim, plus what it was matched by and what it can bind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedClaim {
    pub claim: LoginClaim,
    /// SHA-256 of the session token this claim was staked with.
    ///
    /// **This is what a migration should be bound to.** `Store::create_migration_bound_hash`
    /// takes exactly this. `Store::live_claim_token_hash` reads *the newest* claim instead,
    /// which is the same singleton mistake this module was rewritten to remove - with two
    /// players signed in it binds one player's migration to the other's token.
    ///
    /// `None` for a claim staked before this column existed. That is not an error; it means
    /// there is nothing to bind and the caller must mint an unbound migration and say so.
    pub token_hash: Option<String>,
    /// The client process this claim was bound to, if the launcher registered one.
    pub launch_pid: Option<u32>,
    /// The address the sign-in came from, if it was recorded.
    pub peer: Option<String>,
    pub how: ResolvedBy,
}

/// The answer [`Store::resolve_login_claim`] gives.
///
/// Deliberately **not** an `Option`. `CLAUDE.md`: *"A refusal that is reported to no one will
/// be ignored eventually."* `Ambiguous` and `NoClaim` both end in the caller using its
/// fallback account, but they are different events with different fixes - one means nobody
/// has signed in, the other means two people have and the server cannot tell their clients
/// apart - and collapsing them into `None` loses the only sentence worth logging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaimResolution {
    Resolved(ResolvedClaim),
    /// No live claim exists at all. Use the configured fallback account.
    NoClaim,
    /// More than one claim is live and nothing in the evidence picks one out.
    ///
    /// **Do not guess.** Use the configured fallback account and log this - serving the newest
    /// is the eviction bug this module exists to remove.
    Ambiguous {
        /// How many claims were live when the decision was made.
        live: usize,
        /// True when the caller presented no evidence whatsoever, which makes this the
        /// *expected* outcome rather than a surprise: an unwired caller cannot resolve two
        /// claims and never will. Distinguishing the two is the difference between "wire the
        /// peer-owner lookup" and "the lookup ran and did not help".
        evidence_was_empty: bool,
    },
}

impl ClaimResolution {
    /// The resolved claim, if there was one.
    ///
    /// A convenience for a caller that has **already handled** [`ClaimResolution::Ambiguous`]
    /// explicitly. Do not use it to collapse ambiguity into `None`; that throws away the only
    /// thing worth logging about the interesting case.
    pub fn resolved(&self) -> Option<&ResolvedClaim> {
        match self {
            ClaimResolution::Resolved(r) => Some(r),
            _ => None,
        }
    }

    /// The whole sentence a caller should log, whichever way it went.
    ///
    /// Written here rather than at the call site because there are three outcomes and the two
    /// uninteresting-looking ones are the ones that get summarised into silence.
    pub fn why(&self) -> String {
        match self {
            ClaimResolution::Resolved(r) => format!(
                "account {:?} (id {}) from a login claim, matched by {}",
                r.claim.account_name,
                r.claim.account_id,
                r.how.describe()
            ),
            // The wording is pinned by `login::server::tests`, which greps the log line for
            // "no launcher claim is live". That is a reasonable thing for a test to hold on
            // to - three of this project's answers came from reading a log after the fact -
            // so the phrase is kept rather than improved.
            ClaimResolution::NoClaim => {
                "no launcher claim is live - run maplecw-launcher and sign in".to_string()
            }
            ClaimResolution::Ambiguous { live, evidence_was_empty } => {
                let tail = if *evidence_was_empty {
                    "and this connection presented NO evidence at all, so no rule could have \
                     picked one. Wire store::peerowner::owning_pid into the accept path"
                } else {
                    "and the evidence presented did not pick one out"
                };
                format!(
                    "{live} login claims are live {tail}. REFUSING TO GUESS - serving the \
                     newest is how one player gets served as another. The fallback account \
                     is used instead"
                )
            }
        }
    }
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
/// # The schema migration, and why it is in two halves
///
/// The `CREATE TABLE IF NOT EXISTS` covers a database that has never seen this table. It does
/// **nothing at all** to one that has - which is the owner's - so everything added after the table
/// first shipped has to go through [`ensure_columns`], with the same `PRAGMA table_info` guard
/// `crate::migration::ensure_columns` and `Store::add_meso_column` carry, because
/// `ALTER TABLE ADD COLUMN` raises "duplicate column name" on the second open.
///
/// The unique index on `token_hash` is the other half of the migration and is what makes the
/// table per-launch rather than per-machine. `CREATE UNIQUE INDEX IF NOT EXISTS` *does* work
/// on an existing table, but it fails if the existing rows already violate it - so
/// [`ensure_columns`] de-duplicates first. On a database written by the old code that is a
/// no-op, because the old code kept at most one row.
///
/// # Why every entry point below calls this as well
///
/// Not tidiness, and not a substitute for the line in `Store::init` - both should exist. This
/// module cannot add that line itself (`db.rs` belongs to the coordinator this session), and
/// `CLAUDE.md`'s "Built is not wired" section is about subsystems that were finished and never
/// connected, which *"on screen look identical to not existing"*.
///
/// Here that failure would be worse than invisible. [`Store::resolve_login_claim`] is read
/// once per login connection; with no table it returns `Err`, and a login server that answers
/// an error instead of a reply freezes the client's entire UI - the "always answer" rule.
/// Ensuring the table at each entry point costs one no-op `CREATE TABLE IF NOT EXISTS` per
/// login (not per packet) and makes the unwired state impossible rather than fatal.
pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        -- ONE ROW PER LAUNCH, keyed by the sign-in token's SHA-256.
        --
        -- The previous version of this table held one global row: `stake_login_claim` ran
        -- DELETE and then INSERT, so a second sign-in evicted the first and BOTH login
        -- connections were served as the second account. Measured, 2026-08-29. Nothing here
        -- deletes another launch's row any more.
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
    ensure_columns(conn)?;
    Ok(())
}

/// Bring an existing `login_claims` up to the per-launch schema.
///
/// Three steps, all idempotent, all guarded:
///
/// 1. add `launch_pid` and `peer` if they are missing. Nullable, so every existing row gets
///    `NULL` - a statement of fact rather than a default being imposed, since nothing has ever
///    recorded either;
/// 2. add `token_hash` if it is missing. It is declared `NOT NULL` in the `CREATE TABLE`, but
///    `ALTER TABLE ADD COLUMN` cannot add a `NOT NULL` column without a default, so on the
///    upgrade path it arrives nullable. A `NULL` there means "staked before this column
///    existed" and is handled everywhere as `None`, never as a match;
/// 3. de-duplicate `token_hash` and create the unique index that makes it the key.
///
/// **Step 3 keeps the newest row per hash.** A database written by the old code has at most
/// one row, so this is a no-op there; it exists so that creating the index cannot fail on a
/// database somebody has been experimenting with. `NULL`s are left alone: SQLite treats every
/// `NULL` as distinct in a unique index, so pre-migration rows do not collide with each other.
///
/// An empty `PRAGMA table_info` means the table is not there yet - do nothing and let the
/// caller's own statement raise, which is the honest failure.
pub(crate) fn ensure_columns(conn: &Connection) -> Result<()> {
    let mut existing = std::collections::HashSet::new();
    {
        let mut stmt = conn.prepare("PRAGMA table_info(login_claims)")?;
        for name in stmt.query_map([], |row| row.get::<_, String>(1))? {
            existing.insert(name?);
        }
    }
    if existing.is_empty() {
        return Ok(());
    }
    if !existing.contains("token_hash") {
        conn.execute("ALTER TABLE login_claims ADD COLUMN token_hash TEXT", [])?;
    }
    if !existing.contains("launch_pid") {
        conn.execute("ALTER TABLE login_claims ADD COLUMN launch_pid INTEGER", [])?;
    }
    if !existing.contains("launch_hash") {
        conn.execute("ALTER TABLE login_claims ADD COLUMN launch_hash TEXT", [])?;
    }
    if !existing.contains("peer") {
        conn.execute("ALTER TABLE login_claims ADD COLUMN peer TEXT", [])?;
    }
    // Keep the newest row per hash so the unique index below can be created. NULLs are exempt
    // - SQLite counts each of them as distinct - which is what leaves pre-migration rows alone.
    conn.execute(
        "DELETE FROM login_claims
          WHERE token_hash IS NOT NULL
            AND id NOT IN (SELECT MAX(id) FROM login_claims
                            WHERE token_hash IS NOT NULL GROUP BY token_hash)",
        [],
    )?;
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_login_claims_token ON login_claims(token_hash)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_login_claims_pid ON login_claims(launch_pid)",
        [],
    )?;
    Ok(())
}

/// One live claim, as read from the database. Internal to the resolver.
struct LiveRow {
    claim: LoginClaim,
    token_hash: Option<String>,
    launch_pid: Option<u32>,
    peer: Option<String>,
}

impl LiveRow {
    fn into_resolved(self, how: ResolvedBy) -> ResolvedClaim {
        ResolvedClaim {
            claim: self.claim,
            token_hash: self.token_hash,
            launch_pid: self.launch_pid,
            peer: self.peer,
            how,
        }
    }
}

impl Store {
    /// **Stake a claim for one launch.** Does not touch anybody else's.
    ///
    /// `token` is the session token `Store::authenticate` just issued; only its SHA-256 is
    /// stored, never the token. Pass [`LOGIN_CLAIM_TTL_SECS`] for `ttl_secs` unless there is a
    /// reason not to; a `ttl_secs` of `0` or less produces a row that is already expired and
    /// will never be served, which is useful in tests and is otherwise a way of writing "no
    /// claim" the long way round.
    ///
    /// Re-staking with **the same token** refreshes that one claim in place - it is the same
    /// sign-in, and a launcher that retries must not accumulate rows. Any `launch_pid` already
    /// bound to it is kept, because the client it names has not changed.
    ///
    /// # What was removed here, and why the removal is the fix
    ///
    /// This used to run `DELETE FROM login_claims` first. That is what made two launcher
    /// sign-ins mutually exclusive and made both connections resolve to the second account.
    /// What is left is a `DELETE` of **expired** rows only, which evicts nothing that anybody
    /// could still be playing and keeps the table from growing without bound.
    ///
    /// # Refusals
    ///
    /// An account that does not exist, or exists and is **disabled**, is refused with
    /// [`StoreError::NoSuchAccount`] rather than staked. The alternative - staking anyway -
    /// would hand the caller back a `LoginClaim` describing a claim that
    /// [`Store::resolve_login_claim`] can never return, because that read joins to `accounts`
    /// and skips disabled ones. A return value that says "done" about something that will
    /// never happen is the shape `CLAUDE.md` keeps warning about; failing loudly at the
    /// launcher, where a person is watching, is the cheaper end of that trade.
    pub fn stake_login_claim(
        &self,
        account_id: i64,
        token: &str,
        ttl_secs: i64,
    ) -> Result<LoginClaim> {
        Ok(self.stake_login_claim_with(account_id, token, ttl_secs, None)?.claim)
    }

    /// [`Store::stake_login_claim`], recording the address the sign-in came from and returning
    /// the launch handle.
    ///
    /// `peer` is the source address of the **sign-in**, which for the launcher is the machine
    /// the client will be started on. It is rule 3 in the module docs' table: it separates two
    /// machines and is explicitly useless for two clients on one, which is why it is a
    /// tie-breaker and never the discriminator.
    ///
    /// A **fresh launch id** is minted on every call, including a re-stake of the same token:
    /// one press of Login is one launch, and the previous handle stops working. See
    /// [`StakedClaim`] for why it is not the session token.
    pub fn stake_login_claim_with(
        &self,
        account_id: i64,
        token: &str,
        ttl_secs: i64,
        peer: Option<&str>,
    ) -> Result<StakedClaim> {
        // Hash before taking the lock: nothing that holds the connection should be handling
        // the plain token at all.
        let token_hash = hash_token(token);
        let launch = new_token();
        let now = Store::now();
        // Saturating, so an absurd ttl cannot wrap the expiry into the past and produce a
        // claim that is dead the instant it is written.
        let expires_at = now.saturating_add(ttl_secs);

        let mut conn = self.conn();
        create_tables(&conn)?;
        // One transaction: the prune and the insert go together, so a failure between them
        // cannot leave the table half-migrated.
        let tx = conn.transaction()?;

        let account: Option<(String, i64)> = tx
            .query_row(
                "SELECT name, enabled FROM accounts WHERE id = ?1",
                rusqlite::params![account_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        // Both refusals drop the transaction unread, so a refused stake writes nothing and
        // leaves every claim that was already there intact.
        let Some((account_name, enabled)) = account else {
            return Err(StoreError::NoSuchAccount { name: format!("account id {account_id}") });
        };
        if enabled == 0 {
            return Err(StoreError::NoSuchAccount { name: format!("{account_name} (disabled)") });
        }

        // Expired rows only. THIS IS NOT THE OLD `DELETE FROM login_claims` - nothing live is
        // touched, so a second sign-in can no longer evict the first.
        tx.execute("DELETE FROM login_claims WHERE expires_at <= ?1", rusqlite::params![now])?;
        tx.execute(
            "INSERT INTO login_claims
                 (account_id, token_hash, created_at, expires_at, peer, launch_hash)
                  VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(token_hash) DO UPDATE SET
                 account_id  = excluded.account_id,
                 created_at  = excluded.created_at,
                 expires_at  = excluded.expires_at,
                 launch_hash = excluded.launch_hash,
                 -- Keep an address that was recorded before if this stake has none, and keep
                 -- launch_pid untouched: the same token is the same sign-in, so the client
                 -- process it was bound to has not changed.
                 peer        = COALESCE(excluded.peer, login_claims.peer)",
            rusqlite::params![account_id, token_hash, now, expires_at, peer, launch.hash],
        )?;
        tx.commit()?;

        Ok(StakedClaim {
            claim: LoginClaim { account_id, account_name, created_at: now, expires_at },
            launch_id: launch.token,
        })
    }

    /// **Bind a claim to the client process the launcher started.**
    ///
    /// This is what makes two launchers on one machine resolvable. The launcher spawns
    /// `MapleStory.exe` and therefore knows its pid; the login server can ask the operating
    /// system which process owns the far end of an accepted local connection
    /// ([`crate::peerowner::owning_pid`]) and match the two. The pid is never asserted by the
    /// client - it comes from the OS's TCP table on the server side - so a process cannot
    /// claim to be another one.
    ///
    /// Returns `true` if a live claim was bound. **`false` means the token named no live
    /// claim**, which a caller must report rather than ignore: it leaves the launch
    /// unidentifiable, and with a second player signed in that means both fall back.
    ///
    /// # Any other claim holding this pid is unbound first
    ///
    /// Windows reuses process ids. A stale binding on an exited process would make two claims
    /// match one pid, and rule 2 would decline - silently turning a working discriminator off.
    /// So this clears the pid from every other claim in the same statement pair. That un-binds
    /// a *pid*, not a claim: the other claim stays live and stays resolvable by its token, its
    /// address, or by being the only one.
    pub fn bind_launch_pid(&self, launch_id: &str, pid: u32) -> Result<bool> {
        self.bind_pid("launch_hash", &hash_token(launch_id), pid)
    }

    /// [`Store::bind_launch_pid`] for a caller that holds the **session token** rather than
    /// the launch handle - the auth service on a path where it still has one, and the tests.
    pub fn bind_launch_pid_by_token(&self, token: &str, pid: u32) -> Result<bool> {
        self.bind_pid("token_hash", &hash_token(token), pid)
    }

    /// The one statement pair behind both. `column` is a fixed identifier from this module,
    /// never anything a caller supplies, so it cannot be an injection point.
    fn bind_pid(&self, column: &str, hash: &str, pid: u32) -> Result<bool> {
        debug_assert!(matches!(column, "launch_hash" | "token_hash"));
        let now = Store::now();
        let mut conn = self.conn();
        create_tables(&conn)?;
        let tx = conn.transaction()?;
        tx.execute(
            &format!(
                "UPDATE login_claims SET launch_pid = NULL
                  WHERE launch_pid = ?1 AND ({column} IS NULL OR {column} IS NOT ?2)"
            ),
            rusqlite::params![pid, hash],
        )?;
        let changed = tx.execute(
            &format!(
                "UPDATE login_claims SET launch_pid = ?2
                  WHERE {column} = ?1 AND expires_at > ?3"
            ),
            rusqlite::params![hash, pid, now],
        )?;
        tx.commit()?;
        Ok(changed == 1)
    }

    /// **Which account this connection should be served as.** The read to make per connection.
    ///
    /// The rules, in strength order, are in the module docs. The short version: the first rule
    /// that identifies exactly one live claim wins, and if none does the answer is
    /// [`ClaimResolution::Ambiguous`] rather than a guess.
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
    /// never name an account that is not there - and cannot go stale if an account is renamed.
    ///
    /// **This does not consume anything.** The client reconnects within one launch; see the
    /// module docs.
    ///
    /// A caller must treat an `Err` here as "no claim, use the fallback", **not** as a reason
    /// to refuse the connection. An unanswered login freezes the client's whole UI.
    ///
    /// # A presented token that matches nothing resolves to `NoClaim`
    ///
    /// Not to the weaker rules. A connection that presents a token is asserting an identity,
    /// and a wrong assertion must not quietly collect somebody else's claim by falling through
    /// to "there was only one". Presenting nothing is still allowed to reach rule 4, so this
    /// is not a downgrade path: a junk token buys strictly less than silence, which is the
    /// right way round.
    pub fn resolve_login_claim(&self, evidence: &ClaimEvidence) -> Result<ClaimResolution> {
        let live = self.live_rows()?;
        if live.is_empty() {
            return Ok(ClaimResolution::NoClaim);
        }

        // Rule 1: a credential.
        if let Some(token) = evidence.token.as_deref() {
            let want = hash_token(token);
            return Ok(match live.into_iter().find(|r| r.token_hash.as_deref() == Some(&want)) {
                Some(row) => ClaimResolution::Resolved(row.into_resolved(ResolvedBy::Token)),
                None => ClaimResolution::NoClaim,
            });
        }

        // Rule 2: the process the OS attributes this connection to.
        if let Some(pid) = evidence.launch_pid {
            let mut matched = live.iter().filter(|r| r.launch_pid == Some(pid));
            if let (Some(_), None) = (matched.next(), matched.next()) {
                let row = live.into_iter().find(|r| r.launch_pid == Some(pid)).expect("just seen");
                return Ok(ClaimResolution::Resolved(row.into_resolved(ResolvedBy::LaunchPid)));
            }
        }

        // Rule 3: the address, but only when it picks out exactly one.
        if let Some(peer) = evidence.peer.as_deref() {
            let mut matched = live.iter().filter(|r| r.peer.as_deref() == Some(peer));
            if let (Some(_), None) = (matched.next(), matched.next()) {
                let row = live
                    .into_iter()
                    .find(|r| r.peer.as_deref() == Some(peer))
                    .expect("just seen");
                return Ok(ClaimResolution::Resolved(row.into_resolved(ResolvedBy::PeerAddress)));
            }
        }

        // Rule 4: there was only one answer to give.
        if live.len() == 1 {
            let row = live.into_iter().next().expect("length checked");
            return Ok(ClaimResolution::Resolved(row.into_resolved(ResolvedBy::SoleLiveClaim)));
        }

        Ok(ClaimResolution::Ambiguous {
            live: live.len(),
            evidence_was_empty: evidence.is_empty(),
        })
    }

    /// **The one live claim, or `None` if there is not exactly one.**
    ///
    /// Kept because `crates/login` and `db.rs` call it, and it is the right call for a server
    /// that has not been wired to [`ClaimEvidence`] yet. Its meaning has changed and the change
    /// is the safe direction: it used to return the **newest** of several claims, which is
    /// precisely how one player was served as another. With two live claims and nothing to
    /// tell connections apart there is no honest answer, so this returns `None` and the caller
    /// falls back to its configured account.
    ///
    /// Prefer [`Store::resolve_login_claim`], which says *why* it gave the answer it gave and
    /// can actually resolve two claims. This one cannot: `None` here is both "nobody signed
    /// in" and "two people did", and a log line built on it cannot tell the owner which.
    pub fn current_login_claim(&self) -> Result<Option<LoginClaim>> {
        Ok(match self.resolve_login_claim(&ClaimEvidence::none())? {
            ClaimResolution::Resolved(r) => Some(r.claim),
            ClaimResolution::NoClaim | ClaimResolution::Ambiguous { .. } => None,
        })
    }

    /// Every live claim, newest first. For a startup banner and for `--list`.
    ///
    /// A server that only ever asks `current_login_claim` cannot tell "nobody is signed in"
    /// from "two people are", and those need opposite messages on screen.
    pub fn live_login_claims(&self) -> Result<Vec<LoginClaim>> {
        Ok(self.live_rows()?.into_iter().map(|r| r.claim).collect())
    }

    /// The live rows, newest first. Ordering is for display; no rule below depends on it.
    fn live_rows(&self) -> Result<Vec<LiveRow>> {
        let now = Store::now();
        let conn = self.conn();
        create_tables(&conn)?;
        let mut stmt = conn.prepare(
            "SELECT c.account_id, a.name, c.created_at, c.expires_at,
                    c.token_hash, c.launch_pid, c.peer
               FROM login_claims c
               JOIN accounts a ON a.id = c.account_id
              WHERE c.expires_at > ?1 AND a.enabled = 1
              ORDER BY c.created_at DESC, c.id DESC",
        )?;
        let rows = stmt.query_map(rusqlite::params![now], |row| {
            Ok(LiveRow {
                claim: LoginClaim {
                    account_id: row.get(0)?,
                    account_name: row.get(1)?,
                    created_at: row.get(2)?,
                    expires_at: row.get(3)?,
                },
                token_hash: row.get(4)?,
                launch_pid: row.get::<_, Option<i64>>(5)?.map(|p| p as u32),
                peer: row.get(6)?,
            })
        })?;
        Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
    }

    /// **Drop every claim.** Returns how many rows went.
    ///
    /// What a launcher's "sign out" calls, and what `maplecw-useradd` calls after a
    /// destructive account change. Every connection then falls back to the configured default
    /// account. **This drops other people's launches too** - it is the sledgehammer, and
    /// [`Store::clear_login_claims_for`] is the scoped one.
    pub fn clear_login_claims(&self) -> Result<usize> {
        let conn = self.conn();
        create_tables(&conn)?;
        Ok(conn.execute("DELETE FROM login_claims", [])?)
    }

    /// Drop only this account's claims.
    ///
    /// Called by `Store::set_password`, for the same reason it already revokes sessions: a
    /// password is changed when someone has lost control of it, and a live claim is a standing
    /// instruction to serve a game connection as that account. Leaving it would mean the old
    /// password's last act outlives the password.
    ///
    /// Scoped rather than [`Self::clear_login_claims`] because clearing everything would also
    /// evict a *different* account that happens to be playing - one person changing their
    /// password should not drop another out of the game. Note the plural: an account can now
    /// legitimately have more than one live claim, one per launch, and all of them go.
    pub fn clear_login_claims_for(&self, account_id: i64) -> Result<usize> {
        let conn = self.conn();
        create_tables(&conn)?;
        Ok(conn.execute(
            "DELETE FROM login_claims WHERE account_id = ?1",
            rusqlite::params![account_id],
        )?)
    }

    /// Drop the claim a particular sign-in staked. What a launcher's "quit" would call.
    pub fn clear_login_claim_for_token(&self, token: &str) -> Result<usize> {
        let conn = self.conn();
        create_tables(&conn)?;
        Ok(conn.execute(
            "DELETE FROM login_claims WHERE token_hash = ?1",
            rusqlite::params![hash_token(token)],
        )?)
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
        assert_eq!(
            store.resolve_login_claim(&ClaimEvidence::none()).unwrap(),
            ClaimResolution::NoClaim
        );
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

    // ------------------------------------------------------------------------------------
    // THE EVICTION BUG. Everything in this block was measured failing against the old code.
    // ------------------------------------------------------------------------------------

    /// **The bug, stated as a test.**
    ///
    /// Two people sign in through the launcher. Against the old `DELETE FROM login_claims`
    /// this measured, through `auth::AuthService::login`:
    ///
    /// ```text
    /// after otter: claim=Some("otter")
    /// after owl:   claim=Some("owl")
    /// login connection 1 served as Some("owl")
    /// login connection 2 served as Some("owl")
    /// ```
    ///
    /// Otter's row was gone from the table. Both assertions matter: the row count catches the
    /// eviction directly, and the resolution catches the consequence.
    #[test]
    fn a_second_sign_in_does_not_evict_the_first() {
        let (store, wisp, other) = store_with_accounts();
        let wisp_token = login(&store, "wisp");
        let other_token = login(&store, "wisp_alt");

        store.stake_login_claim(wisp, &wisp_token, LOGIN_CLAIM_TTL_SECS).unwrap();
        store.stake_login_claim(other, &other_token, LOGIN_CLAIM_TTL_SECS).unwrap();

        assert_eq!(claim_rows(&store), 2, "the second sign-in deleted the first one's row");
        let live = store.live_login_claims().unwrap();
        assert_eq!(live.len(), 2);
        let names: Vec<&str> = live.iter().map(|c| c.account_name.as_str()).collect();
        assert!(names.contains(&"wisp") && names.contains(&"wisp_alt"), "{names:?}");
    }

    /// **Neither can be served as the other.** Each connection presents its own token and gets
    /// its own account - and this is the property the owner asked for, from one machine.
    #[test]
    fn two_launches_each_resolve_to_their_own_account() {
        let (store, wisp, other) = store_with_accounts();
        let wisp_token = login(&store, "wisp");
        let other_token = login(&store, "wisp_alt");
        // Same address, deliberately: "IP cannot be the sole discriminator".
        store.stake_login_claim_with(wisp, &wisp_token, LOGIN_CLAIM_TTL_SECS, Some("127.0.0.1")).unwrap();
        store.stake_login_claim_with(other, &other_token, LOGIN_CLAIM_TTL_SECS, Some("127.0.0.1")).unwrap();

        for (token, want) in [(&wisp_token, wisp), (&other_token, other)] {
            let r = store
                .resolve_login_claim(&ClaimEvidence::with_token(token.clone()).from_peer("127.0.0.1"))
                .unwrap();
            let resolved = r.resolved().expect("a presented token resolves");
            assert_eq!(resolved.claim.account_id, want, "{}", r.why());
            assert_eq!(resolved.how, ResolvedBy::Token);
        }
    }

    /// **The same, with the discriminator that actually exists today: the client's process.**
    ///
    /// Nothing on the game socket presents a token, so this is the rule that has to carry two
    /// clients on one box. Same address for both, different pid for each.
    #[test]
    fn two_launches_on_one_address_are_separated_by_their_client_process() {
        let (store, wisp, other) = store_with_accounts();
        let wisp_token = login(&store, "wisp");
        let other_token = login(&store, "wisp_alt");
        store.stake_login_claim_with(wisp, &wisp_token, LOGIN_CLAIM_TTL_SECS, Some("127.0.0.1")).unwrap();
        store.stake_login_claim_with(other, &other_token, LOGIN_CLAIM_TTL_SECS, Some("127.0.0.1")).unwrap();

        assert!(store.bind_launch_pid_by_token(&wisp_token, 4242).unwrap(), "wisp's launch was bound");
        assert!(store.bind_launch_pid_by_token(&other_token, 9999).unwrap(), "the other launch was bound");

        for (pid, want) in [(4242u32, wisp), (9999, other)] {
            let r = store
                .resolve_login_claim(&ClaimEvidence::with_launch_pid(pid).from_peer("127.0.0.1"))
                .unwrap();
            let resolved = r.resolved().unwrap_or_else(|| panic!("pid {pid}: {}", r.why()));
            assert_eq!(resolved.claim.account_id, want, "{}", r.why());
            assert_eq!(resolved.how, ResolvedBy::LaunchPid);
        }
    }

    /// **Two live claims and nothing to tell them apart is refused, not guessed.**
    ///
    /// This is the assertion the old code could never have passed: it answered with the newest
    /// row, and that answer is how one player is served as another. The fallback the caller
    /// then uses is a downgrade in usefulness and is not an impersonation of either player.
    #[test]
    fn two_live_claims_with_no_evidence_are_ambiguous_rather_than_the_newest() {
        let (store, wisp, other) = store_with_accounts();
        let wisp_token = login(&store, "wisp");
        let other_token = login(&store, "wisp_alt");
        store.stake_login_claim(wisp, &wisp_token, LOGIN_CLAIM_TTL_SECS).unwrap();
        store.stake_login_claim(other, &other_token, LOGIN_CLAIM_TTL_SECS).unwrap();

        let r = store.resolve_login_claim(&ClaimEvidence::none()).unwrap();
        assert_eq!(
            r,
            ClaimResolution::Ambiguous { live: 2, evidence_was_empty: true },
            "serving the newest here is the whole bug"
        );
        assert!(r.resolved().is_none());
        // And the legacy reader, which `crates/login` still calls, refuses too.
        assert!(
            store.current_login_claim().unwrap().is_none(),
            "current_login_claim must not resurrect newest-wins"
        );
        // The sentence a caller logs has to name the count and say what it did.
        let why = r.why();
        assert!(why.contains('2'), "{why}");
        assert!(why.to_lowercase().contains("fallback"), "{why}");
    }

    /// Evidence that is present but matches nothing still refuses when there are two claims -
    /// and says so differently, because "the lookup did not help" and "there was no lookup"
    /// need opposite work.
    #[test]
    fn ambiguity_says_whether_any_evidence_was_presented() {
        let (store, wisp, other) = store_with_accounts();
        let a = login(&store, "wisp");
        let b = login(&store, "wisp_alt");
        store.stake_login_claim_with(wisp, &a, LOGIN_CLAIM_TTL_SECS, Some("127.0.0.1")).unwrap();
        store.stake_login_claim_with(other, &b, LOGIN_CLAIM_TTL_SECS, Some("127.0.0.1")).unwrap();

        // A pid nobody registered, from the shared address: both rules decline.
        let r = store
            .resolve_login_claim(&ClaimEvidence::with_launch_pid(1).from_peer("127.0.0.1"))
            .unwrap();
        assert_eq!(r, ClaimResolution::Ambiguous { live: 2, evidence_was_empty: false });
        assert!(r.why().contains("did not pick one out"), "{}", r.why());
    }

    /// A *presented* token that matches no live claim resolves to nothing rather than falling
    /// through to "there was only one". Otherwise presenting a junk token would be a way of
    /// collecting somebody else's claim.
    #[test]
    fn a_token_that_matches_nothing_does_not_fall_through_to_the_sole_claim() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();

        // The control: presenting nothing DOES reach the sole claim, so the assertion below
        // is about the token and not about the store being empty.
        assert!(store.resolve_login_claim(&ClaimEvidence::none()).unwrap().resolved().is_some());
        assert_eq!(
            store
                .resolve_login_claim(&ClaimEvidence::with_token("a-token-nobody-issued"))
                .unwrap(),
            ClaimResolution::NoClaim
        );
    }

    /// The single-player case is unchanged: one claim, no evidence, served - and labelled
    /// honestly as the rule that it is.
    #[test]
    fn one_live_claim_is_served_to_an_unidentified_connection_and_says_it_is_not_a_discriminator() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();

        let r = store.resolve_login_claim(&ClaimEvidence::none()).unwrap();
        let resolved = r.resolved().expect("one claim, one answer");
        assert_eq!(resolved.claim.account_id, wisp);
        assert_eq!(resolved.how, ResolvedBy::SoleLiveClaim);
        assert!(
            r.why().to_lowercase().contains("not a discriminator"),
            "the log line must not overstate this: {}",
            r.why()
        );
    }

    /// Two machines, one claim each, no pid registered anywhere: the address separates them.
    /// This is rule 3 doing the job rule 2 cannot do off-box.
    #[test]
    fn two_claims_from_different_addresses_are_separated_by_the_address() {
        let (store, wisp, other) = store_with_accounts();
        let a = login(&store, "wisp");
        let b = login(&store, "wisp_alt");
        store.stake_login_claim_with(wisp, &a, LOGIN_CLAIM_TTL_SECS, Some("10.0.0.5")).unwrap();
        store.stake_login_claim_with(other, &b, LOGIN_CLAIM_TTL_SECS, Some("10.0.0.9")).unwrap();

        for (peer, want) in [("10.0.0.5", wisp), ("10.0.0.9", other)] {
            let r = store.resolve_login_claim(&ClaimEvidence::none().from_peer(peer)).unwrap();
            let resolved = r.resolved().unwrap_or_else(|| panic!("{peer}: {}", r.why()));
            assert_eq!(resolved.claim.account_id, want);
            assert_eq!(resolved.how, ResolvedBy::PeerAddress);
        }
    }

    /// **The owner's constraint, stated as a test.** Same address, two claims: the address must
    /// decline rather than pick one. If this ever resolves, IP has become the sole
    /// discriminator and two clients on one machine are impersonating each other again.
    #[test]
    fn the_address_alone_never_separates_two_claims_from_one_machine() {
        let (store, wisp, other) = store_with_accounts();
        let a = login(&store, "wisp");
        let b = login(&store, "wisp_alt");
        store.stake_login_claim_with(wisp, &a, LOGIN_CLAIM_TTL_SECS, Some("127.0.0.1")).unwrap();
        store.stake_login_claim_with(other, &b, LOGIN_CLAIM_TTL_SECS, Some("127.0.0.1")).unwrap();

        assert_eq!(
            store.resolve_login_claim(&ClaimEvidence::none().from_peer("127.0.0.1")).unwrap(),
            ClaimResolution::Ambiguous { live: 2, evidence_was_empty: false }
        );
    }

    // ------------------------------------------------------------------------------------
    // The launch handle. The launcher no longer holds a session token - `crates/auth` stakes
    // the claim and `launcher::http` drops the token, with a test pinning that - so this is
    // how the one thing that knows the client's pid gets to record it.
    // ------------------------------------------------------------------------------------

    /// The launcher's real path, end to end: sign in, get a launch id, start a client, bind
    /// its pid, and have the server resolve that pid to the right account. Two of them, on one
    /// address, which is the owner's machine.
    #[test]
    fn two_launch_handles_bind_two_client_processes_on_one_address() {
        let (store, wisp, other) = store_with_accounts();
        let a = login(&store, "wisp");
        let b = login(&store, "wisp_alt");
        let first = store
            .stake_login_claim_with(wisp, &a, LOGIN_CLAIM_TTL_SECS, Some("127.0.0.1"))
            .unwrap();
        let second = store
            .stake_login_claim_with(other, &b, LOGIN_CLAIM_TTL_SECS, Some("127.0.0.1"))
            .unwrap();
        assert_ne!(first.launch_id, second.launch_id, "each sign-in gets its own handle");

        assert!(store.bind_launch_pid(&first.launch_id, 1111).unwrap());
        assert!(store.bind_launch_pid(&second.launch_id, 2222).unwrap());

        for (pid, want) in [(1111u32, wisp), (2222, other)] {
            let r = store
                .resolve_login_claim(&ClaimEvidence::with_launch_pid(pid).from_peer("127.0.0.1"))
                .unwrap();
            let resolved = r.resolved().unwrap_or_else(|| panic!("pid {pid}: {}", r.why()));
            assert_eq!(resolved.claim.account_id, want, "{}", r.why());
            assert_eq!(resolved.how, ResolvedBy::LaunchPid);
        }
    }

    /// **The launch id is not a session token and cannot stand in for one.** If it could, the
    /// launcher would be carrying a game credential again and `login_claims` would have two
    /// keys of equal power.
    #[test]
    fn a_launch_id_cannot_resolve_a_claim_the_way_a_token_can() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        let staked = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();

        // The control: the real token does resolve, so a `NoClaim` below is about the launch
        // id and not about the claim being missing.
        assert_eq!(
            store
                .resolve_login_claim(&ClaimEvidence::with_token(token))
                .unwrap()
                .resolved()
                .unwrap()
                .how,
            ResolvedBy::Token
        );
        assert_eq!(
            store
                .resolve_login_claim(&ClaimEvidence::with_token(staked.launch_id))
                .unwrap(),
            ClaimResolution::NoClaim,
            "the launch handle must buy nothing on the resolve path"
        );
    }

    /// Neither the launch id nor the token is stored in the clear, and the assertion is
    /// against `hash_token` so "absent" cannot be satisfied by storing nothing at all.
    #[test]
    fn the_launch_id_is_stored_only_as_a_hash() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        let staked = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        let stored: String = store
            .conn()
            .query_row("SELECT launch_hash FROM login_claims", [], |r| r.get(0))
            .unwrap();
        assert!(!staked.launch_id.is_empty(), "a blank id would make this vacuous");
        assert_ne!(stored, staked.launch_id);
        assert_eq!(stored, hash_token(&staked.launch_id));
        // And a `{:?}` of the struct - the way a secret usually reaches a log - is redacted.
        let debug = format!("{staked:?}");
        assert!(!debug.contains(&staked.launch_id), "the launch id leaked into Debug: {debug}");
        assert!(debug.contains("redacted"), "{debug}");
    }

    /// A stale launch id stops working when the launcher signs in again. One press of Login is
    /// one launch; a handle from the previous press must not bind a process to the new claim.
    #[test]
    fn re_staking_invalidates_the_previous_launch_handle() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        let old = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        let new = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        assert_ne!(old.launch_id, new.launch_id);

        assert!(!store.bind_launch_pid(&old.launch_id, 5).unwrap(), "the old handle is spent");
        assert!(store.bind_launch_pid(&new.launch_id, 5).unwrap(), "and the new one works");
    }

    /// An unknown launch id binds nothing at all - it must not fall through to some other
    /// claim, and it must not be reported as success.
    #[test]
    fn an_unknown_launch_id_binds_nothing() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();
        assert!(!store.bind_launch_pid("not-a-launch-id", 99).unwrap());
        assert_eq!(
            store.resolve_login_claim(&ClaimEvidence::with_launch_pid(99)).unwrap(),
            ClaimResolution::Resolved(
                store
                    .resolve_login_claim(&ClaimEvidence::none())
                    .unwrap()
                    .resolved()
                    .unwrap()
                    .clone()
            ),
            "an unbound pid falls to the sole-claim rule, not to a phantom binding"
        );
    }

    /// Binding a pid to a token that names no live claim reports `false` rather than writing
    /// nothing quietly. The caller has to be able to say "the launch was not registered".
    #[test]
    fn binding_a_pid_to_no_live_claim_answers_false() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        assert!(!store.bind_launch_pid_by_token(&token, 100).unwrap(), "nothing staked yet");

        store.stake_login_claim(wisp, &token, -60).unwrap();
        assert!(!store.bind_launch_pid_by_token(&token, 100).unwrap(), "the claim is expired");

        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();
        assert!(store.bind_launch_pid_by_token(&token, 100).unwrap());
    }

    /// A recycled process id must not end up on two claims - two matches means rule 2 declines,
    /// which would silently turn the only working discriminator off.
    #[test]
    fn rebinding_a_recycled_pid_moves_it_rather_than_duplicating_it() {
        let (store, wisp, other) = store_with_accounts();
        let a = login(&store, "wisp");
        let b = login(&store, "wisp_alt");
        store.stake_login_claim(wisp, &a, LOGIN_CLAIM_TTL_SECS).unwrap();
        store.stake_login_claim(other, &b, LOGIN_CLAIM_TTL_SECS).unwrap();

        store.bind_launch_pid_by_token(&a, 7777).unwrap();
        // wisp's client exits; Windows hands 7777 to the other launch.
        store.bind_launch_pid_by_token(&b, 7777).unwrap();

        let r = store.resolve_login_claim(&ClaimEvidence::with_launch_pid(7777)).unwrap();
        let resolved = r.resolved().unwrap_or_else(|| panic!("{}", r.why()));
        assert_eq!(resolved.claim.account_id, other, "the newer binding owns the pid");
        // And wisp's claim is still live and still resolvable by its token - un-binding a pid
        // must not have removed a claim.
        assert_eq!(store.live_login_claims().unwrap().len(), 2);
        assert_eq!(
            store
                .resolve_login_claim(&ClaimEvidence::with_token(a))
                .unwrap()
                .resolved()
                .unwrap()
                .claim
                .account_id,
            wisp
        );
    }

    /// Re-staking the same token refreshes one row instead of adding one, and keeps the pid -
    /// the client process the sign-in named has not changed.
    #[test]
    fn re_staking_the_same_token_refreshes_the_row_and_keeps_its_pid() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();
        store.bind_launch_pid_by_token(&token, 555).unwrap();

        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();
        assert_eq!(claim_rows(&store), 1, "a retry must not accumulate rows");
        let r = store.resolve_login_claim(&ClaimEvidence::with_launch_pid(555)).unwrap();
        assert_eq!(r.resolved().expect("the pid survived the re-stake").claim.account_id, wisp);
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
        store.bind_launch_pid_by_token(&token, 31337).unwrap();

        for connection in 1..=3 {
            let live = store
                .current_login_claim()
                .unwrap()
                .unwrap_or_else(|| panic!("login connection {connection} lost the claim"));
            assert_eq!(live.account_id, wisp, "connection {connection} was served the wrong account");
            assert_eq!(live.account_name, "wisp");

            let by_pid = store
                .resolve_login_claim(&ClaimEvidence::with_launch_pid(31337))
                .unwrap();
            assert_eq!(by_pid.resolved().expect("still bound").claim.account_id, wisp);
        }
        assert_eq!(claim_rows(&store), 1, "reading must not consume the row");
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

    /// Staking prunes rows that have expired, so a machine used for months does not accumulate
    /// a claim per sign-in forever. **Only expired ones** - the live sibling must survive, or
    /// this is the eviction bug with a timer on it.
    #[test]
    fn staking_prunes_expired_rows_and_only_expired_rows() {
        let (store, wisp, other) = store_with_accounts();
        let dead = login(&store, "wisp");
        let live = login(&store, "wisp_alt");
        // The live one first: staking prunes before it inserts, so staking the dead row last
        // is the only way to have both on disk at once.
        store.stake_login_claim(other, &live, LOGIN_CLAIM_TTL_SECS).unwrap();
        store.stake_login_claim(wisp, &dead, -1).unwrap();
        assert_eq!(claim_rows(&store), 2, "one dead, one live");

        let third = login(&store, "wisp");
        store.stake_login_claim(wisp, &third, LOGIN_CLAIM_TTL_SECS).unwrap();
        assert_eq!(claim_rows(&store), 2, "the dead row went, the live one stayed");
        let names: Vec<String> =
            store.live_login_claims().unwrap().into_iter().map(|c| c.account_name).collect();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"wisp_alt".to_string()), "{names:?}");
    }

    /// Disabling an account kills its claims. Disabling is how this server locks somebody out,
    /// and a claim staked beforehand must not outlive it.
    #[test]
    fn a_claim_whose_account_was_disabled_is_not_returned() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();
        store.bind_launch_pid_by_token(&token, 21).unwrap();
        assert!(store.current_login_claim().unwrap().is_some());

        store.set_enabled("wisp", false).unwrap();
        assert_eq!(claim_rows(&store), 1, "the row is untouched...");
        assert!(
            store.current_login_claim().unwrap().is_none(),
            "...but a disabled account must not be served"
        );
        // Every rule, not just the weakest one: a token or a pid must not be a way past it.
        assert_eq!(
            store.resolve_login_claim(&ClaimEvidence::with_token(token.clone())).unwrap(),
            ClaimResolution::NoClaim
        );
        assert_eq!(
            store.resolve_login_claim(&ClaimEvidence::with_launch_pid(21)).unwrap(),
            ClaimResolution::NoClaim
        );

        // Re-enabling brings the same claim back - the row was never destroyed.
        store.set_enabled("wisp", true).unwrap();
        assert_eq!(store.current_login_claim().unwrap().unwrap().account_id, wisp);
    }

    /// Staking for a disabled account is refused rather than written. Otherwise the caller
    /// would get a `LoginClaim` back describing something the resolver can never return.
    #[test]
    fn staking_for_a_disabled_or_missing_account_is_refused_and_writes_nothing() {
        let (store, wisp, other) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();

        store.set_enabled("wisp_alt", false).unwrap();
        let other_token = "1111111111111111111111111111111111111111111111111111111111111111";
        assert!(matches!(
            store.stake_login_claim(other, other_token, LOGIN_CLAIM_TTL_SECS),
            Err(StoreError::NoSuchAccount { .. })
        ));
        assert!(matches!(
            store.stake_login_claim(9_999_999, other_token, LOGIN_CLAIM_TTL_SECS),
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

        // And the whole row, dumped as text, contains it nowhere else either - a second column
        // holding the plain token would pass every assertion above.
        let dump: String = store
            .conn()
            .query_row(
                "SELECT COALESCE(account_id,'')||'|'||COALESCE(token_hash,'')||'|'||
                        COALESCE(created_at,'')||'|'||COALESCE(expires_at,'')||'|'||
                        COALESCE(launch_pid,'')||'|'||COALESCE(peer,'')
                   FROM login_claims",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(!dump.contains(&token), "the token appears somewhere in the row: {dump}");
    }

    /// The resolved claim carries the hash a migration should be bound to, and it is the hash
    /// of *this* claim's token rather than of whichever is newest.
    #[test]
    fn a_resolved_claim_carries_its_own_token_hash_for_binding() {
        let (store, wisp, other) = store_with_accounts();
        let a = login(&store, "wisp");
        let b = login(&store, "wisp_alt");
        store.stake_login_claim(wisp, &a, LOGIN_CLAIM_TTL_SECS).unwrap();
        store.stake_login_claim(other, &b, LOGIN_CLAIM_TTL_SECS).unwrap();
        store.bind_launch_pid_by_token(&a, 11).unwrap();
        store.bind_launch_pid_by_token(&b, 22).unwrap();

        let first = store.resolve_login_claim(&ClaimEvidence::with_launch_pid(11)).unwrap();
        assert_eq!(first.resolved().unwrap().token_hash.as_deref(), Some(hash_token(&a).as_str()));
        let second = store.resolve_login_claim(&ClaimEvidence::with_launch_pid(22)).unwrap();
        assert_eq!(second.resolved().unwrap().token_hash.as_deref(), Some(hash_token(&b).as_str()));
        assert_ne!(
            first.resolved().unwrap().token_hash,
            second.resolved().unwrap().token_hash,
            "binding both migrations to one hash is the bug this field exists to stop"
        );
    }

    /// Clearing empties the table, and the answer goes with it.
    #[test]
    fn clearing_removes_every_claim() {
        let (store, wisp, other) = store_with_accounts();
        let a = login(&store, "wisp");
        let b = login(&store, "wisp_alt");
        store.stake_login_claim(wisp, &a, LOGIN_CLAIM_TTL_SECS).unwrap();
        store.stake_login_claim(other, &b, LOGIN_CLAIM_TTL_SECS).unwrap();
        assert_eq!(claim_rows(&store), 2);

        assert_eq!(store.clear_login_claims().unwrap(), 2, "both rows went");
        assert_eq!(claim_rows(&store), 0);
        assert!(
            store.current_login_claim().unwrap().is_none(),
            "the next connection falls back to the configured account"
        );
        // Clearing again is a no-op, not an error or a second phantom row.
        assert_eq!(store.clear_login_claims().unwrap(), 0);
    }

    /// Clearing one account's claims leaves another account's launch alone - and now takes
    /// *every* launch of that account, because there can be more than one.
    #[test]
    fn clearing_one_account_takes_all_its_launches_and_nobody_elses() {
        let (store, wisp, other) = store_with_accounts();
        let a = login(&store, "wisp");
        let a2 = login(&store, "wisp");
        let b = login(&store, "wisp_alt");
        store.stake_login_claim(wisp, &a, LOGIN_CLAIM_TTL_SECS).unwrap();
        store.stake_login_claim(wisp, &a2, LOGIN_CLAIM_TTL_SECS).unwrap();
        store.stake_login_claim(other, &b, LOGIN_CLAIM_TTL_SECS).unwrap();
        assert_eq!(claim_rows(&store), 3);

        assert_eq!(store.clear_login_claims_for(wisp).unwrap(), 2, "both of wisp's launches");
        assert_eq!(
            store.current_login_claim().unwrap().map(|c| c.account_id),
            Some(other),
            "the other account was playing and had nothing to do with it"
        );
    }

    /// One launch can be dropped without touching the other.
    #[test]
    fn clearing_one_token_drops_only_that_launch() {
        let (store, wisp, other) = store_with_accounts();
        let a = login(&store, "wisp");
        let b = login(&store, "wisp_alt");
        store.stake_login_claim(wisp, &a, LOGIN_CLAIM_TTL_SECS).unwrap();
        store.stake_login_claim(other, &b, LOGIN_CLAIM_TTL_SECS).unwrap();

        assert_eq!(store.clear_login_claim_for_token(&a).unwrap(), 1);
        assert_eq!(
            store.current_login_claim().unwrap().map(|c| c.account_id),
            Some(other),
            "the surviving launch is now the sole one and is served"
        );
        assert_eq!(store.clear_login_claim_for_token(&a).unwrap(), 0, "and again is a no-op");
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
            store.bind_launch_pid_by_token(&token, 8080).unwrap();
        }

        let again = Store::open(&path).expect("the schema runs on every open and must be a no-op");
        let live = again.current_login_claim().unwrap().expect("the claim persisted");
        assert_eq!(live.account_id, wisp);
        assert_eq!(live.account_name, "wisp");
        // And the pid binding survived, which is what makes the claim resolvable after a
        // server restart mid-session.
        assert!(again
            .resolve_login_claim(&ClaimEvidence::with_launch_pid(8080))
            .unwrap()
            .resolved()
            .is_some());
        drop(again);
        let _ = std::fs::remove_dir_all(&dir);
    }

    // ------------------------------------------------------------------------------------
    // The schema migration. The owner's maplecw.db has real characters on it and predates all of
    // this, so "upgrades in place without losing rows" has to be a test rather than a hope.
    // ------------------------------------------------------------------------------------

    /// Running the whole schema repeatedly must be a no-op. `ALTER TABLE ADD COLUMN` raises
    /// "duplicate column name" on the second call and `CREATE UNIQUE INDEX` fails on a
    /// violation, so both guards have to hold.
    #[test]
    fn the_schema_is_idempotent() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();
        {
            let conn = store.conn();
            for _ in 0..3 {
                create_tables(&conn).expect("the schema must survive being run again");
                ensure_columns(&conn).expect("ALTER TABLE ADD COLUMN is not idempotent");
            }
        }
        assert_eq!(store.current_login_claim().unwrap().unwrap().account_id, wisp);
    }

    /// **The pre-existing database.** A `login_claims` written before `launch_pid` and `peer`
    /// existed must gain them, keep its row, and stay resolvable - an `Err` on the read path
    /// would make the login server answer nothing, which freezes the client's whole UI.
    #[test]
    fn a_database_without_the_new_columns_is_upgraded_without_losing_rows() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();
        {
            let conn = store.conn();
            conn.execute("DROP INDEX IF EXISTS idx_login_claims_token", []).unwrap();
            conn.execute("DROP INDEX IF EXISTS idx_login_claims_pid", []).unwrap();
            conn.execute("ALTER TABLE login_claims DROP COLUMN launch_pid", []).unwrap();
            conn.execute("ALTER TABLE login_claims DROP COLUMN peer", []).unwrap();
            let n: i64 = conn
                .query_row("SELECT COUNT(*) FROM login_claims", [], |r| r.get(0))
                .unwrap();
            assert_eq!(n, 1, "the row must survive the downgrade for this test to mean anything");
        }
        // The read path ensures the columns itself, so the pre-existing row still resolves.
        let live = store.current_login_claim().unwrap().expect("the old row is still served");
        assert_eq!(live.account_id, wisp);
        assert_eq!(claim_rows(&store), 1, "the upgrade must not have dropped it");
        // And it can be bound to a pid afterwards, which is what an upgraded row needs.
        assert!(store.bind_launch_pid_by_token(&token, 4321).unwrap());
    }

    /// The unique index is created on a table that may already contain duplicates - somebody
    /// experimenting, or a half-applied older build. De-duplication keeps the newest row per
    /// hash so the index can be created at all, and it must not touch distinct hashes.
    #[test]
    fn duplicate_token_hashes_are_collapsed_to_the_newest_before_the_index_is_made() {
        let (store, wisp, other) = store_with_accounts();
        let a = login(&store, "wisp");
        let b = login(&store, "wisp_alt");
        store.stake_login_claim(wisp, &a, LOGIN_CLAIM_TTL_SECS).unwrap();
        store.stake_login_claim(other, &b, LOGIN_CLAIM_TTL_SECS).unwrap();
        {
            let conn = store.conn();
            conn.execute("DROP INDEX IF EXISTS idx_login_claims_token", []).unwrap();
            // Two more rows with a hash that already exists, as an old build could leave.
            for account in [wisp, other] {
                conn.execute(
                    "INSERT INTO login_claims (account_id, token_hash, created_at, expires_at)
                          SELECT ?1, token_hash, created_at, expires_at
                            FROM login_claims WHERE account_id = ?1 LIMIT 1",
                    rusqlite::params![account],
                )
                .unwrap();
            }
            let n: i64 = conn
                .query_row("SELECT COUNT(*) FROM login_claims", [], |r| r.get(0))
                .unwrap();
            assert_eq!(n, 4, "the duplicates must exist for this test to mean anything");
            ensure_columns(&conn).expect("the index must be creatable over duplicates");
        }
        assert_eq!(claim_rows(&store), 2, "one row per distinct token hash survived");
        assert_eq!(store.live_login_claims().unwrap().len(), 2);
    }

    /// A row staked before `token_hash` existed reads as `None` rather than matching anything.
    /// It is still servable as the sole live claim - it names a real account - but it can never
    /// be picked out by a token, and it must not bind a migration to a hash it does not have.
    #[test]
    fn a_row_with_no_token_hash_is_never_matched_by_a_token() {
        let (store, wisp, _) = store_with_accounts();
        let now = Store::now();
        {
            let conn = store.conn();
            create_tables(&conn).unwrap();
            // The only way to get a NULL in, since the CREATE TABLE declares it NOT NULL:
            // rebuild the table the way the pre-token schema had it.
            conn.execute_batch(
                "DROP TABLE login_claims;
                 CREATE TABLE login_claims (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
                    created_at INTEGER NOT NULL,
                    expires_at INTEGER NOT NULL);",
            )
            .unwrap();
            conn.execute(
                "INSERT INTO login_claims (account_id, created_at, expires_at) VALUES (?1, ?2, ?3)",
                rusqlite::params![wisp, now, now + LOGIN_CLAIM_TTL_SECS],
            )
            .unwrap();
            ensure_columns(&conn).expect("the upgrade must cope with a pre-token table");
        }

        let r = store.resolve_login_claim(&ClaimEvidence::none()).unwrap();
        let resolved = r.resolved().expect("it still names a real account");
        assert_eq!(resolved.claim.account_id, wisp);
        assert_eq!(resolved.token_hash, None, "there is nothing to bind a migration to");
        assert_eq!(
            store.resolve_login_claim(&ClaimEvidence::with_token("anything")).unwrap(),
            ClaimResolution::NoClaim,
            "a NULL hash must not be matched by a token"
        );
    }
}
