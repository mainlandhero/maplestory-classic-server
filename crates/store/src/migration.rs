//! The handoff from the login/character-select server to a channel.
//!
//! The two are separate processes and share nothing but this database, so a migration has
//! to be written down somewhere both can see. That is the whole job of this module.
//!
//! # What a migration is, and what it is not
//!
//! **It is not authentication.** The token that actually travels *in the packet* is a `u32` -
//! that is all the client's migration packet has room for - and a 32-bit value is not a
//! secret. What the table buys on its own is *single use*: a migration is claimed exactly
//! once, so a replayed handoff cannot put a second connection into the world as the same
//! character. That is a real property and worth having; it is not the same as proving who
//! is on the far end.
//!
//! # The hole this module was rebuilt to close
//!
//! The owner, 2026-08-29: *"this may lead to impersonation if two clients log in at the exact
//! same time."*
//!
//! It is worse than a race. [`Store::claim_migration_for_character`] keys on a character id
//! that the connecting client simply **asserts** - the channel server reads it out of the
//! client's `0x007D` hello and looks it up. Nothing checks that the connection has any right
//! to that character, so **any** connection to a channel port can claim **any** character's
//! pending migration by naming it. No race is required, and no second client is required.
//!
//! # Two things were measured before this design, and both constrain it
//!
//! **The seed does not come back.** 115 distinct `0x007D` bodies from the archived runs were
//! searched for every one of the 74 seeds this server has ever minted - plainly, in both
//! endiannesses, and under the XOR-with-a-replicated-byte form the decompiler note on
//! `FUN_1415deae0` predicts. 8510 trials, zero hits, with a positive control (the character
//! id, which is documented to sit at offset 8) succeeding on all 115 bodies. So the client
//! will not carry a secret back for us; anything that travels on the game socket has to be
//! put there by the hook.
//!
//! **The client's identity block is per-machine, not per-launch.** Across 56 archived
//! launches there are exactly **two** distinct `0x0073` bodies, and one of them is the smoke
//! test's synthetic `aabbccddeeff`. The real client sends a byte-identical block every time.
//! So there is no value the login server can see on its socket and the channel can recognise
//! on the other one - **no correlator exists that does not require changing the client.**
//!
//! Those two measurements together are why the fix below is a *credential*, and why a
//! credential is the only thing that can work. See the module's `IMPERSONATION` note on what
//! is still open.
//!
//! # The invariant that makes this safe to deploy incrementally
//!
//! > **A migration row that carries a `token_hash` can only ever be claimed by presenting a
//! > token that hashes to it. No mode, flag, fallback or older caller relaxes that.**
//!
//! The permissive path applies *only* to rows with `token_hash IS NULL` - migrations minted
//! by a caller that had no credential to bind. This is deliberate and it is the whole reason
//! there is no flag day: the moment the login server starts binding tokens, every new row is
//! protected automatically, and an attacker **cannot downgrade by simply omitting the
//! token**. A guard that can be skipped by not presenting the thing it checks is not a guard,
//! which is the failure `CLAUDE.md` records about `record_quest_forfeit`.
//!
//! [`Store::create_migration`] keeps its old signature and mints **unbound** rows. That is
//! not an oversight - see its doc block for which callers legitimately have no token.

use rand::RngCore;
use rusqlite::{Connection, OptionalExtension};

use crate::db::Store;
use crate::error::{Result, StoreError};
use crate::session::hash_token;

/// A migration the channel server has accepted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimedMigration {
    pub account_id: i64,
    pub character_id: u32,
    pub world_id: u32,
    pub channel_id: u32,
}

/// How long a minted migration stays claimable.
///
/// The client reconnects immediately - the measured gap between the login socket closing
/// and the new connection arriving was **under a millisecond** - so this is generous by
/// three orders of magnitude and exists only so an abandoned migration does not sit
/// claimable forever.
pub const MIGRATION_TTL_SECS: i64 = 60;

/// What a connection presents when it tries to claim a migration.
///
/// Built by the channel server from the connection itself. Both fields are optional because
/// **today the game socket carries neither**: the client sends no token (measured - the seed
/// does not come back), and the peer address is observable but is explicitly *not* the
/// discriminator.
#[derive(Debug, Clone, Default)]
pub struct MigrationEvidence {
    /// The plain session token the connection presented, if any. Only its SHA-256 is ever
    /// compared or stored; the plain value does not leave this struct.
    pub token: Option<String>,
    /// The connection's source address as the server observed it, e.g. `"127.0.0.1"`.
    ///
    /// **Not a discriminator.** The owner, 2026-08-29: *"The login MapleCW Launcher needs to be
    /// able to potentially handle multiple connections from the same IP as well, IP cannot
    /// be the sole discriminator."* Two clients on one machine both present `127.0.0.1`, and
    /// several players behind one NAT share one address. Recorded and reported; see
    /// [`PeerPolicy`].
    pub peer: Option<String>,
}

impl MigrationEvidence {
    /// A connection that presented nothing. This is what the channel server has today.
    pub fn none() -> Self {
        Self::default()
    }

    /// A connection presenting a session token.
    pub fn with_token(token: impl Into<String>) -> Self {
        Self { token: Some(token.into()), peer: None }
    }

    /// Record the observed source address alongside whatever else was presented.
    pub fn from_peer(mut self, peer: impl Into<String>) -> Self {
        self.peer = Some(peer.into());
        self
    }
}

/// What a migration row was minted with. See [`Store::migration_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationBinding {
    /// True when the row carries a `token_hash`, and therefore **cannot** be claimed by a
    /// connection that presents nothing. False is the impersonatable case.
    pub token_bound: bool,
    /// The address recorded at mint time, if the minting socket reported one.
    pub peer: Option<String>,
}

/// What to do when the claiming connection's address differs from the minting one's.
///
/// Defaults to [`PeerPolicy::Record`], and that default is a decision rather than timidity.
/// Refusing on a peer mismatch **can break a legitimate client**: a login connection that
/// arrives on `::1` and a channel connection that arrives on `127.0.0.1` are the same client
/// on a dual-stack machine and would not match as strings. The brief's instruction was to
/// keep the address as defence in depth *only if it costs nothing*, and a check that can lock
/// The owner out of their own server does not cost nothing.
///
/// So the answer is reported rather than ignored - [`ClaimOutcome::Claimed`] carries
/// `peer_mismatch` and the caller is expected to log it - and `Require` exists for a
/// deployment that knows its addressing is stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PeerPolicy {
    /// Record and report a mismatch; never refuse on it alone.
    #[default]
    Record,
    /// Refuse a claim whose address differs from the one recorded at mint time.
    Require,
}

/// Why a claim was refused. Every variant is an event worth a log line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// The row is bound to a token and the connection presented none.
    ///
    /// **This is the impersonation case.** It is also what an un-hooked or outdated client
    /// looks like, and the two are indistinguishable from here - which is why the channel
    /// server must say out loud which it is refusing rather than silently falling back.
    TokenMissing,
    /// The row is bound to a token and the connection presented a different one.
    TokenMismatch,
    /// [`PeerPolicy::Require`] was in force and the addresses differ.
    PeerMismatch { minted_at: String, presented: String },
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Refusal::TokenMissing => write!(
                f,
                "the migration is bound to a session token and this connection presented \
                 none - it is either an impersonation attempt or a client launched without \
                 the hook. It is REFUSED either way; a fallback here would undo the binding"
            ),
            Refusal::TokenMismatch => write!(
                f,
                "the migration is bound to a session token and this connection presented a \
                 different one"
            ),
            Refusal::PeerMismatch { minted_at, presented } => write!(
                f,
                "the migration was minted for {minted_at} and this connection came from \
                 {presented}, with PeerPolicy::Require in force"
            ),
        }
    }
}

/// The outcome of a claim attempt.
///
/// Deliberately **not** an `Option`. `CLAUDE.md`: *"A refusal that is reported to no one will
/// be ignored eventually"* - the three quest call sites captured a store refusal into a log
/// string and carried on, and the experience doubled beside it. A refusal here is a distinct
/// variant so a caller that only handles the happy path fails to compile rather than falling
/// through to "no migration", which is the exact shape that would silently restore the old
/// trust-the-asserted-id behaviour.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaimOutcome {
    Claimed {
        migration: ClaimedMigration,
        /// True when the address differs from the one recorded at mint time and
        /// [`PeerPolicy::Record`] let it through anyway. **Log this.**
        peer_mismatch: bool,
    },
    /// A live migration exists and this connection may not have it.
    Refused(Refusal),
    /// There is no unconsumed, unexpired migration to claim - or another connection won the
    /// race for it. Not an error and not a refusal.
    NoMigration,
}

impl ClaimOutcome {
    /// The migration, if one was actually claimed.
    ///
    /// A convenience for call sites that have already handled [`ClaimOutcome::Refused`]
    /// explicitly. **Do not use it to collapse a refusal into `None`** - that is the bug this
    /// enum exists to prevent.
    pub fn migration(&self) -> Option<&ClaimedMigration> {
        match self {
            ClaimOutcome::Claimed { migration, .. } => Some(migration),
            _ => None,
        }
    }
}

/// Add the credential columns to `migrations`.
///
/// **`CREATE TABLE IF NOT EXISTS` does nothing at all to a table that already exists**, and
/// `migrations` exists in the owner's database, so these columns cannot be declared in the
/// `CREATE TABLE` in `db.rs` - they would appear only in a database built from scratch. That
/// is the same reason `Store::add_meso_column` and friends are separate functions, and this
/// carries the same `PRAGMA table_info` guard because `ALTER TABLE ADD COLUMN` is not
/// idempotent: it raises "duplicate column name" on the second open.
///
/// Both columns are **nullable and every existing row gets NULL**. That is a statement of
/// fact rather than a default being imposed: nothing has ever recorded a credential here, so
/// there is no value that could be overwritten - and a NULL `token_hash` means exactly
/// "unbound", which is the state the invariant in the module docs is written around.
///
/// # Why every entry point calls this
///
/// The same belt-and-braces reasoning `crate::claims::create_tables` spells out, and for a
/// sharper reason. `db.rs` is not this session's to edit, and a module that is finished but
/// unwired *"on screen looks identical to not existing"*. Here the unwired state would be
/// worse than invisible: a missing column makes every claim return `Err`, and a channel that
/// answers an error instead of a reply freezes the client's entire UI. Calling this per
/// claim costs one no-op `PRAGMA` and makes the unwired state impossible rather than fatal.
pub(crate) fn ensure_columns(conn: &Connection) -> Result<()> {
    let mut existing = std::collections::HashSet::new();
    {
        let mut stmt = conn.prepare("PRAGMA table_info(migrations)")?;
        for name in stmt.query_map([], |row| row.get::<_, String>(1))? {
            existing.insert(name?);
        }
    }
    // An empty set means the table itself is not there yet - `Store::init` has not run. Do
    // nothing rather than ALTER a table that does not exist; the caller's own query will
    // raise, which is the honest failure.
    if existing.is_empty() {
        return Ok(());
    }
    if !existing.contains("token_hash") {
        conn.execute("ALTER TABLE migrations ADD COLUMN token_hash TEXT", [])?;
    }
    if !existing.contains("peer") {
        conn.execute("ALTER TABLE migrations ADD COLUMN peer TEXT", [])?;
    }
    Ok(())
}

impl Store {
    /// Mint a migration **with no credential bound to it**.
    ///
    /// The seed is random rather than derived from the character id. Deriving it would
    /// make the value predictable *and* make two migrations of the same character collide
    /// on the primary key, so the second would fail exactly when a player re-enters the
    /// world - which is the common case, not an edge one.
    ///
    /// # This mints an UNBOUND row, and that is on purpose
    ///
    /// A row with no `token_hash` is claimable by a connection that presents nothing - which
    /// is the pre-existing behaviour, hole included. Two callers legitimately have no
    /// credential to bind:
    ///
    /// * the **channel-change** path in the world server, which mints a migration for a
    ///   client that is already in the world and never touched a login connection;
    /// * every test that just needs a claimable row.
    ///
    /// The login server has a token and must use [`Store::create_migration_bound`]. The
    /// signature here is unchanged deliberately: roughly sixty call sites in files owned by
    /// other work call it, and breaking them to add a parameter that two thirds of them would
    /// pass `None` to would be churn, not safety. The invariant in the module docs is what
    /// makes leaving it safe - an unbound row is no weaker than it was, and a **bound** row
    /// cannot be claimed through this path's permissive evidence at all.
    pub fn create_migration(
        &self,
        account_id: i64,
        character_id: u32,
        world_id: u32,
        channel_id: u32,
    ) -> Result<u32> {
        self.create_migration_bound(account_id, character_id, world_id, channel_id, None, None)
    }

    /// Mint a migration **bound to a session token**, and optionally to an address.
    ///
    /// `token` is the plain session token the launcher was issued at sign-in. **Only its
    /// SHA-256 is stored** - the standing constraint in `CLAUDE.md`, and the same rule
    /// `login_claims.token_hash` already follows. The plain token does not survive this call.
    ///
    /// `peer` is recorded for the audit trail and for [`PeerPolicy::Require`]; it is **not**
    /// the discriminator, because two clients on one machine share an address.
    ///
    /// Passing `None` for `token` mints an unbound row and is exactly
    /// [`Store::create_migration`].
    pub fn create_migration_bound(
        &self,
        account_id: i64,
        character_id: u32,
        world_id: u32,
        channel_id: u32,
        token: Option<&str>,
        peer: Option<&str>,
    ) -> Result<u32> {
        let token_hash = token.map(hash_token);
        self.create_migration_bound_hash(
            account_id,
            character_id,
            world_id,
            channel_id,
            token_hash.as_deref(),
            peer,
        )
    }

    /// [`Store::create_migration_bound`] for a caller that already holds the **hash** and
    /// never had the plain token.
    ///
    /// That is the login server's situation: it binds a migration to the sign-in that staked
    /// the live login claim, and `login_claims` stores only the SHA-256 - the plain token was
    /// handed to the launcher and is not in the login server's process at all. Hashing again
    /// would be wrong and demanding the plain value would mean inventing a way to obtain one.
    ///
    /// **The binding is part of the INSERT, not a follow-up UPDATE.** Minting unbound and
    /// then binding would leave a window - however short - in which the row sits in the table
    /// claimable by a connection presenting nothing, which is precisely the hole this exists
    /// to close. The row is never visible in an unbound state.
    pub fn create_migration_bound_hash(
        &self,
        account_id: i64,
        character_id: u32,
        world_id: u32,
        channel_id: u32,
        token_hash: Option<&str>,
        peer: Option<&str>,
    ) -> Result<u32> {
        let conn = self.conn();
        ensure_columns(&conn)?;
        let now = Store::now();

        // Retry on collision rather than trusting 32 bits to be unique. Ten attempts is
        // far past the point where a collision means something else is wrong.
        for _ in 0..10 {
            let seed = random_seed();
            let inserted = conn.execute(
                "INSERT OR IGNORE INTO migrations
                     (seed, account_id, character_id, world_id, channel_id, created_at,
                      token_hash, peer)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                rusqlite::params![
                    seed, account_id, character_id, world_id, channel_id, now, token_hash, peer
                ],
            )?;
            if inserted == 1 {
                return Ok(seed);
            }
        }
        Err(StoreError::MigrationSeedExhausted { tries: 10 })
    }

    /// What a migration row is bound to, without revealing the binding itself.
    ///
    /// For logs, for tests, and for a channel server that wants to say *why* it refused.
    /// Returns the fact of a token binding rather than the hash: a stored hash is not a
    /// secret, but there is no reason for it to travel and every reason for a log line not
    /// to contain something that looks like one.
    ///
    /// `None` means there is no such row at all.
    pub fn migration_binding(&self, seed: u32) -> Result<Option<MigrationBinding>> {
        let conn = self.conn();
        ensure_columns(&conn)?;
        conn.query_row(
            "SELECT token_hash, peer FROM migrations WHERE seed = ?1",
            rusqlite::params![seed],
            |row| {
                Ok(MigrationBinding {
                    token_bound: row.get::<_, Option<String>>(0)?.is_some(),
                    peer: row.get(1)?,
                })
            },
        )
        .optional()
        .map_err(Into::into)
    }

    /// The `token_hash` of the live login claim, if there is one.
    ///
    /// The login server needs this to bind a migration to the sign-in that authorised it, and
    /// [`crate::claims::LoginClaim`] deliberately does not carry it - `claims.rs` describes the
    /// column as *"an audit trail waiting for a reader"*. **This is the reader.**
    ///
    /// The liveness predicate is the same one `Store::current_login_claim` enforces, and it is
    /// repeated here rather than inferred: the claim must be unexpired, its account must still
    /// exist, and the account must still be **enabled**. Disabling an account is how this
    /// server locks somebody out, and a migration bound to a claim staked before that must not
    /// outlive it.
    ///
    /// `Ok(None)` means "no live claim", which is a legitimate state (nobody has used the
    /// launcher). It is **not** a reason to refuse a login - an unanswered login freezes the
    /// client's whole UI - it is a reason to mint an unbound migration and say so.
    pub fn live_claim_token_hash(&self) -> Result<Option<String>> {
        let conn = self.conn();
        let now = Store::now();
        conn.query_row(
            "SELECT c.token_hash
               FROM login_claims c
               JOIN accounts a ON a.id = c.account_id
              WHERE c.expires_at > ?1 AND a.enabled = 1
              ORDER BY c.created_at DESC, c.id DESC
              LIMIT 1",
            rusqlite::params![now],
            |row| row.get(0),
        )
        .optional()
        .map_err(Into::into)
    }

    /// Claim the pending migration for a character. `None` if there is no live one.
    ///
    /// Keyed on the character rather than the seed, because the capture on 2026-08-19
    /// showed the seed does **not** come back: `0x007D` carries the character id and no
    /// trace of the `u32` the migration packet handed over. That was re-checked on
    /// 2026-08-29 across 115 hello bodies and 74 seeds with a passing positive control - it
    /// is not there.
    ///
    /// # This is the impersonation path, and this signature cannot close it
    ///
    /// The character id is **asserted by whoever connects**. Any connection naming a
    /// character id gets that character's pending migration. This wrapper presents
    /// [`MigrationEvidence::none`], so by the module invariant it can only ever claim an
    /// **unbound** row - a bound one is refused rather than served. That is what keeps this
    /// signature from being a downgrade path.
    ///
    /// New callers should use [`Store::claim_migration_for_character_with`] and handle
    /// [`ClaimOutcome::Refused`] explicitly.
    pub fn claim_migration_for_character(&self, character_id: u32) -> Result<Option<ClaimedMigration>> {
        Ok(self
            .claim_migration_for_character_with(
                character_id,
                &MigrationEvidence::none(),
                PeerPolicy::Record,
            )?
            .migration()
            .cloned())
    }

    /// Claim the pending migration for a character, checking whatever the connection presented.
    ///
    /// Picks the newest live migration if somehow more than one exists, so a stale row
    /// cannot shadow a fresh entry.
    pub fn claim_migration_for_character_with(
        &self,
        character_id: u32,
        evidence: &MigrationEvidence,
        policy: PeerPolicy,
    ) -> Result<ClaimOutcome> {
        let now = Store::now();
        // The guard is scoped so it is dropped before `claim_migration_with` runs. std's
        // Mutex is not reentrant, so holding it across that call deadlocks the process -
        // which is exactly what happened the first time this was written.
        let seed: Option<u32> = {
            let conn = self.conn();
            ensure_columns(&conn)?;
            conn.query_row(
                "SELECT seed FROM migrations
                  WHERE character_id = ?1 AND consumed_at IS NULL AND created_at >= ?2
                  ORDER BY created_at DESC, rowid DESC LIMIT 1",
                rusqlite::params![character_id, now - MIGRATION_TTL_SECS],
                |row| row.get(0),
            )
            .optional()?
        };
        match seed {
            Some(seed) => self.claim_migration_with(seed, evidence, policy),
            None => Ok(ClaimOutcome::NoMigration),
        }
    }

    /// Claim a migration. Returns `None` if there is no unconsumed, unexpired one.
    ///
    /// Presents no credential, so by the module invariant it can only claim an **unbound**
    /// row. See [`Store::claim_migration_with`].
    pub fn claim_migration(&self, seed: u32) -> Result<Option<ClaimedMigration>> {
        Ok(self
            .claim_migration_with(seed, &MigrationEvidence::none(), PeerPolicy::Record)?
            .migration()
            .cloned())
    }

    /// Claim a migration, checking whatever the connection presented.
    ///
    /// # Two properties, and the order of the statements is what gives them
    ///
    /// **Single use survives a race.** The consume is inside the `UPDATE`'s `WHERE`, so two
    /// connections racing the same seed cannot both win: SQLite reports one row changed to
    /// exactly one of them. That was correct before this change and is untouched.
    ///
    /// **A refused claim does not consume the migration.** The credential is checked *before*
    /// the `UPDATE` and a refusal returns without running it. This matters more than it
    /// looks: if a wrong-token attempt burned the row, an attacker who cannot impersonate
    /// could still **deny** the real player their character by racing every login with a
    /// junk token. Turning an impersonation bug into a denial-of-service bug is not a fix.
    pub fn claim_migration_with(
        &self,
        seed: u32,
        evidence: &MigrationEvidence,
        policy: PeerPolicy,
    ) -> Result<ClaimOutcome> {
        let conn = self.conn();
        ensure_columns(&conn)?;
        let now = Store::now();
        let live_since = now - MIGRATION_TTL_SECS;

        // Read the credential the row was minted with, under the same liveness predicate the
        // consume will use.
        let bound: Option<(Option<String>, Option<String>)> = conn
            .query_row(
                "SELECT token_hash, peer FROM migrations
                  WHERE seed = ?1 AND consumed_at IS NULL AND created_at >= ?2",
                rusqlite::params![seed, live_since],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        let Some((token_hash, minted_peer)) = bound else {
            return Ok(ClaimOutcome::NoMigration);
        };

        // THE INVARIANT. A bound row demands its token, and there is no branch below this
        // that can be reached without satisfying it.
        if let Some(expected) = token_hash.as_deref() {
            match evidence.token.as_deref() {
                None => return Ok(ClaimOutcome::Refused(Refusal::TokenMissing)),
                // Compared as hashes, so the plain token is never held beside the stored one.
                Some(presented) if hash_token(presented) != expected => {
                    return Ok(ClaimOutcome::Refused(Refusal::TokenMismatch))
                }
                Some(_) => {}
            }
        }

        // The address is advisory unless the caller asked otherwise; see `PeerPolicy`.
        let peer_mismatch = match (minted_peer.as_deref(), evidence.peer.as_deref()) {
            (Some(a), Some(b)) => a != b,
            // One side unknown is not a mismatch - it is an absence of evidence, and the two
            // read identically only if you forget which is which.
            _ => false,
        };
        if peer_mismatch && policy == PeerPolicy::Require {
            return Ok(ClaimOutcome::Refused(Refusal::PeerMismatch {
                minted_at: minted_peer.unwrap_or_default(),
                presented: evidence.peer.clone().unwrap_or_default(),
            }));
        }

        let changed = conn.execute(
            "UPDATE migrations SET consumed_at = ?2
              WHERE seed = ?1 AND consumed_at IS NULL AND created_at >= ?3",
            rusqlite::params![seed, now, live_since],
        )?;
        if changed == 0 {
            // Another connection won the race between the read above and here. Not a
            // refusal: nobody was turned away, the migration simply is not ours.
            return Ok(ClaimOutcome::NoMigration);
        }
        let migration = conn
            .query_row(
                "SELECT account_id, character_id, world_id, channel_id
                   FROM migrations WHERE seed = ?1",
                rusqlite::params![seed],
                |row| {
                    Ok(ClaimedMigration {
                        account_id: row.get(0)?,
                        character_id: row.get::<_, i64>(1)? as u32,
                        world_id: row.get::<_, i64>(2)? as u32,
                        channel_id: row.get::<_, i64>(3)? as u32,
                    })
                },
            )
            .optional()?;
        Ok(match migration {
            Some(migration) => ClaimOutcome::Claimed { migration, peer_mismatch },
            None => ClaimOutcome::NoMigration,
        })
    }

    /// Claim the **one** migration pending for this world and channel, whoever it is for.
    ///
    /// `None` when there is no pending migration, and - deliberately - also when there is
    /// more than one. Ambiguity is refused rather than guessed.
    ///
    /// # Why this exists, and it is not a shortcut
    ///
    /// A **channel** migration cannot be claimed by character id, because the packet that
    /// causes it does not carry one. Measured 2026-08-21: the channel-migrate reply
    /// (`0x001A`) is `u8 ok, u32 ip, u16 port` and nothing else - seven bytes, no character.
    /// The client's `0x007D` hello on the new channel then reported character id **32513**,
    /// which is `01 7f 00 00` read straight back out of our own body. So the id in the hello
    /// is not the character's; there is nothing in that flow that is.
    ///
    /// The **login** migration is different: `0x0011` carries the character id, the hello
    /// echoes it, and [`Store::claim_migration_for_character`] works. This is the fallback
    /// for the other case, and the caller only reaches it after that one has failed.
    ///
    /// **The ambiguity refusal is load-bearing and is untouched.** With two simultaneous
    /// pending migrations to the same channel this returns `None` rather than handing one
    /// player's character to another's connection - denial rather than impersonation, which
    /// is the right way round.
    pub fn claim_sole_migration_for_channel(
        &self,
        world_id: u32,
        channel_id: u32,
    ) -> Result<Option<ClaimedMigration>> {
        Ok(self
            .claim_sole_migration_for_channel_with(
                world_id,
                channel_id,
                &MigrationEvidence::none(),
                PeerPolicy::Record,
            )?
            .migration()
            .cloned())
    }

    /// [`Store::claim_sole_migration_for_channel`], checking whatever the connection presented.
    pub fn claim_sole_migration_for_channel_with(
        &self,
        world_id: u32,
        channel_id: u32,
        evidence: &MigrationEvidence,
        policy: PeerPolicy,
    ) -> Result<ClaimOutcome> {
        let now = Store::now();
        let seeds: Vec<u32> = {
            let conn = self.conn();
            ensure_columns(&conn)?;
            // LIMIT 2: enough to tell "exactly one" from "more than one", and no more.
            let mut stmt = conn.prepare(
                "SELECT seed FROM migrations
                  WHERE world_id = ?1 AND channel_id = ?2
                    AND consumed_at IS NULL AND created_at >= ?3
                  ORDER BY created_at DESC, rowid DESC LIMIT 2",
            )?;
            let rows = stmt.query_map(
                rusqlite::params![world_id, channel_id, now - MIGRATION_TTL_SECS],
                |row| row.get::<_, u32>(0),
            )?;
            rows.collect::<std::result::Result<Vec<u32>, _>>()?
        };
        match seeds.as_slice() {
            [seed] => self.claim_migration_with(*seed, evidence, policy),
            _ => Ok(ClaimOutcome::NoMigration),
        }
    }

    /// Drop consumed and expired migrations. Returns how many went.
    pub fn purge_migrations(&self) -> Result<usize> {
        let conn = self.conn();
        let cutoff = Store::now() - MIGRATION_TTL_SECS;
        Ok(conn.execute(
            "DELETE FROM migrations WHERE consumed_at IS NOT NULL OR created_at < ?1",
            rusqlite::params![cutoff],
        )?)
    }
}

/// A non-zero random `u32`.
///
/// Zero is excluded so that "the client sent no seed" and "the client sent seed 0" are
/// distinguishable in a log - an all-zero field is what an unwritten buffer looks like.
fn random_seed() -> u32 {
    let mut rng = rand::rngs::OsRng;
    loop {
        let seed = rng.next_u32();
        if seed != 0 {
            return seed;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use net::opcode::Character;

    fn seeded_character(name: &str) -> Character {
        Character { name: name.to_string(), ..Character::default() }
    }

    fn store_with_character() -> (Store, i64, u32) {
        let store = Store::open_in_memory().unwrap();
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let id = store
            .create_character(account_id, 0, &seeded_character("Wanderer"))
            .unwrap()
            .id;
        (store, account_id, id)
    }

    #[test]
    fn a_minted_migration_can_be_claimed_once() {
        let (store, account_id, id) = store_with_character();
        let seed = store.create_migration(account_id, id, 0, 0).unwrap();

        let claimed = store.claim_migration(seed).unwrap().expect("first claim wins");
        assert_eq!(claimed.character_id, id);
        assert_eq!(claimed.account_id, account_id);

        assert_eq!(
            store.claim_migration(seed).unwrap(),
            None,
            "a replayed migration must not put a second connection into the world"
        );
    }

    #[test]
    fn an_unminted_seed_is_never_claimable() {
        let (store, _, _) = store_with_character();
        assert_eq!(store.claim_migration(0xDEADBEEF).unwrap(), None);
    }

    #[test]
    fn seeds_are_never_zero_so_an_empty_field_is_distinguishable() {
        let (store, account_id, id) = store_with_character();
        for _ in 0..32 {
            assert_ne!(store.create_migration(account_id, id, 0, 0).unwrap(), 0);
        }
    }

    /// Deriving the seed from the character id would collide the second time a player
    /// enters the world, which is the ordinary case.
    #[test]
    fn the_same_character_can_migrate_more_than_once() {
        let (store, account_id, id) = store_with_character();
        let first = store.create_migration(account_id, id, 0, 0).unwrap();
        let second = store.create_migration(account_id, id, 0, 0).unwrap();
        assert_ne!(first, second);
        assert!(store.claim_migration(first).unwrap().is_some());
        assert!(store.claim_migration(second).unwrap().is_some());
    }

    /// Characters created before the start map existed carry map_id 0. Reopening the
    /// store must repair them, or three characters that already exist spawn nowhere.
    #[test]
    fn reopening_repairs_characters_that_were_stored_with_no_map() {
        let store = Store::open_in_memory().unwrap();
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let id = store
            .create_character(account_id, 0, &seeded_character("Wanderer"))
            .unwrap()
            .id;

        // Put it back the way an older build would have left it.
        store
            .conn()
            .execute("UPDATE characters SET map_id = 0 WHERE id = ?1", [id])
            .unwrap();

        // The repair runs in the schema batch, so it happens on the next open. Run the
        // same statement to prove the statement itself is the fix.
        let changed = store
            .conn()
            .execute("UPDATE characters SET map_id = 1 WHERE map_id = 0", [])
            .unwrap();
        assert_eq!(changed, 1);
        let back = store.characters_for(account_id, 0).unwrap();
        assert_eq!(back[0].map_id, 1);
    }

    /// And it must not move a character that is somewhere real.
    #[test]
    fn the_repair_leaves_a_character_on_a_real_map_alone() {
        let store = Store::open_in_memory().unwrap();
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut chr = seeded_character("Traveller");
        chr.map_id = 104040000;
        let id = store.create_character(account_id, 0, &chr).unwrap().id;

        let changed = store
            .conn()
            .execute("UPDATE characters SET map_id = 1 WHERE map_id = 0", [])
            .unwrap();
        assert_eq!(changed, 0, "nothing should have needed repair");
        let back = store.characters_for(account_id, 0).unwrap();
        assert_eq!(back.iter().find(|c| c.id == id).unwrap().map_id, 104040000);
    }

    #[test]
    fn purging_clears_claimed_migrations() {
        let (store, account_id, id) = store_with_character();
        let seed = store.create_migration(account_id, id, 0, 0).unwrap();
        store.claim_migration(seed).unwrap().unwrap();
        assert_eq!(store.purge_migrations().unwrap(), 1);
        assert_eq!(store.claim_migration(seed).unwrap(), None);
    }

    /// A deleted character must not leave a claimable migration behind.
    #[test]
    fn deleting_the_character_removes_its_migrations() {
        let (store, account_id, id) = store_with_character();
        let seed = store.create_migration(account_id, id, 0, 0).unwrap();
        assert!(store.delete_character(account_id, id).unwrap());
        assert_eq!(store.claim_migration(seed).unwrap(), None);
    }

    // ---------------------------------------------------------------------------------
    // The credential. Every test below is paired with an injection listed in the report:
    // the guard it covers was removed, the test was watched to fail, and the guard put back.
    // ---------------------------------------------------------------------------------

    const TOKEN: &str = "8d4c9f1e2b3a4d5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5d6";
    const OTHER: &str = "1111111111111111111111111111111111111111111111111111111111111111";

    #[test]
    fn a_bound_migration_is_claimed_by_the_token_it_was_minted_with() {
        let (store, account_id, id) = store_with_character();
        let seed = store
            .create_migration_bound(account_id, id, 0, 0, Some(TOKEN), Some("127.0.0.1"))
            .unwrap();
        let outcome = store
            .claim_migration_with(
                seed,
                &MigrationEvidence::with_token(TOKEN).from_peer("127.0.0.1"),
                PeerPolicy::Record,
            )
            .unwrap();
        match outcome {
            ClaimOutcome::Claimed { migration, peer_mismatch } => {
                assert_eq!(migration.character_id, id);
                assert!(!peer_mismatch);
            }
            other => panic!("the right token must win, got {other:?}"),
        }
    }

    /// **The negative.** A connection presenting somebody else's token does not get the
    /// character.
    #[test]
    fn a_bound_migration_refuses_a_different_token() {
        let (store, account_id, id) = store_with_character();
        let seed = store.create_migration_bound(account_id, id, 0, 0, Some(TOKEN), None).unwrap();
        assert_eq!(
            store
                .claim_migration_with(seed, &MigrationEvidence::with_token(OTHER), PeerPolicy::Record)
                .unwrap(),
            ClaimOutcome::Refused(Refusal::TokenMismatch)
        );
    }

    /// **The anti-downgrade property.** Presenting nothing must not be a way past a binding.
    /// If this ever returns `Claimed`, the whole design is worthless: an attacker would just
    /// omit the token.
    #[test]
    fn a_bound_migration_cannot_be_claimed_by_presenting_nothing() {
        let (store, account_id, id) = store_with_character();
        let seed = store.create_migration_bound(account_id, id, 0, 0, Some(TOKEN), None).unwrap();
        assert_eq!(
            store
                .claim_migration_with(seed, &MigrationEvidence::none(), PeerPolicy::Record)
                .unwrap(),
            ClaimOutcome::Refused(Refusal::TokenMissing)
        );
    }

    /// The old signature is the one the channel server still calls. It presents no evidence,
    /// so it must be **structurally incapable** of claiming a bound row - otherwise it is a
    /// downgrade path with a friendly name.
    #[test]
    fn the_credential_less_legacy_api_cannot_claim_a_bound_migration() {
        let (store, account_id, id) = store_with_character();
        let seed = store.create_migration_bound(account_id, id, 0, 0, Some(TOKEN), None).unwrap();
        assert_eq!(store.claim_migration(seed).unwrap(), None);
        assert_eq!(store.claim_migration_for_character(id).unwrap(), None);
        assert_eq!(store.claim_sole_migration_for_channel(0, 0).unwrap(), None);
    }

    /// A refused claim must leave the migration claimable by its rightful owner. Otherwise
    /// an attacker who cannot impersonate can still deny - and turning impersonation into
    /// denial of service is not a fix.
    #[test]
    fn a_refused_claim_does_not_consume_the_migration() {
        let (store, account_id, id) = store_with_character();
        let seed = store.create_migration_bound(account_id, id, 0, 0, Some(TOKEN), None).unwrap();

        for _ in 0..5 {
            assert!(matches!(
                store
                    .claim_migration_with(seed, &MigrationEvidence::with_token(OTHER), PeerPolicy::Record)
                    .unwrap(),
                ClaimOutcome::Refused(_)
            ));
        }
        assert!(
            store
                .claim_migration_with(seed, &MigrationEvidence::with_token(TOKEN), PeerPolicy::Record)
                .unwrap()
                .migration()
                .is_some(),
            "five wrong-token attempts must not have burned the real player's migration"
        );
    }

    /// The owner's scenario, stated as a test: two characters on two accounts, both mid-migration.
    /// The one holding B's token must not be able to name A's character and get it.
    #[test]
    fn one_account_cannot_claim_another_accounts_character_by_naming_it() {
        let store = Store::open_in_memory().unwrap();
        let a = store.create_account("otter", "correct horse battery").unwrap();
        let b = store.create_account("owl", "correct horse battery").unwrap();
        let a_chr = store.create_character(a, 0, &seeded_character("OtterChr")).unwrap().id;
        let b_chr = store.create_character(b, 0, &seeded_character("OwlChr")).unwrap().id;

        store.create_migration_bound(a, a_chr, 0, 0, Some(TOKEN), Some("127.0.0.1")).unwrap();
        store.create_migration_bound(b, b_chr, 0, 0, Some(OTHER), Some("127.0.0.1")).unwrap();

        // Owl's connection asserts Otter's character id. Same machine, same address.
        let outcome = store
            .claim_migration_for_character_with(
                a_chr,
                &MigrationEvidence::with_token(OTHER).from_peer("127.0.0.1"),
                PeerPolicy::Record,
            )
            .unwrap();
        assert_eq!(
            outcome,
            ClaimOutcome::Refused(Refusal::TokenMismatch),
            "asserting another account's character id must not be enough"
        );

        // And Otter is still able to enter the world.
        assert!(store
            .claim_migration_for_character_with(
                a_chr,
                &MigrationEvidence::with_token(TOKEN).from_peer("127.0.0.1"),
                PeerPolicy::Record,
            )
            .unwrap()
            .migration()
            .is_some());
    }

    /// An unbound row keeps its old behaviour exactly. This is the compatibility guarantee
    /// that lets the change ship without a flag day, and it is also the honest statement of
    /// what is NOT yet protected.
    #[test]
    fn an_unbound_migration_is_still_claimable_with_no_evidence() {
        let (store, account_id, id) = store_with_character();
        let seed = store.create_migration(account_id, id, 0, 0).unwrap();
        assert!(store
            .claim_migration_with(seed, &MigrationEvidence::none(), PeerPolicy::Record)
            .unwrap()
            .migration()
            .is_some());
    }

    /// The address is recorded and reported, never fatal by default - a dual-stack client
    /// can legitimately arrive as ::1 on one socket and 127.0.0.1 on the other.
    #[test]
    fn a_peer_mismatch_is_reported_but_not_refused_by_default() {
        let (store, account_id, id) = store_with_character();
        let seed = store.create_migration_bound(account_id, id, 0, 0, None, Some("::1")).unwrap();
        match store
            .claim_migration_with(
                seed,
                &MigrationEvidence::none().from_peer("127.0.0.1"),
                PeerPolicy::Record,
            )
            .unwrap()
        {
            ClaimOutcome::Claimed { peer_mismatch, .. } => {
                assert!(peer_mismatch, "the mismatch has to reach the caller to be logged")
            }
            other => panic!("Record must not refuse, got {other:?}"),
        }
    }

    /// ...and `Require` does refuse, so the policy is a real choice rather than a comment.
    #[test]
    fn peer_policy_require_refuses_a_mismatch() {
        let (store, account_id, id) = store_with_character();
        let seed = store.create_migration_bound(account_id, id, 0, 0, None, Some("::1")).unwrap();
        assert!(matches!(
            store
                .claim_migration_with(
                    seed,
                    &MigrationEvidence::none().from_peer("10.0.0.9"),
                    PeerPolicy::Require,
                )
                .unwrap(),
            ClaimOutcome::Refused(Refusal::PeerMismatch { .. })
        ));
    }

    /// The plain token must never reach the database. `CLAUDE.md`'s standing constraint.
    #[test]
    fn the_plain_token_is_never_stored() {
        let (store, account_id, id) = store_with_character();
        let seed = store.create_migration_bound(account_id, id, 0, 0, Some(TOKEN), None).unwrap();
        let stored: String = store
            .conn()
            .query_row(
                "SELECT token_hash FROM migrations WHERE seed = ?1",
                rusqlite::params![seed],
                |r| r.get(0),
            )
            .unwrap();
        assert_ne!(stored, TOKEN);
        assert_eq!(stored, hash_token(TOKEN));
    }

    /// The columns are ALTERed onto a table that already exists, and the schema runs on every
    /// open. Adding them twice raises "duplicate column name", so the guard has to hold.
    #[test]
    fn adding_the_credential_columns_is_idempotent() {
        let (store, account_id, id) = store_with_character();
        let conn = store.conn();
        for _ in 0..3 {
            ensure_columns(&conn).expect("ALTER TABLE ADD COLUMN is not idempotent");
        }
        drop(conn);
        // And the table still works afterwards.
        let seed = store.create_migration_bound(account_id, id, 0, 0, Some(TOKEN), None).unwrap();
        assert!(store
            .claim_migration_with(seed, &MigrationEvidence::with_token(TOKEN), PeerPolicy::Record)
            .unwrap()
            .migration()
            .is_some());
    }

    /// A database that predates the columns - which is the owner's - must gain them and keep every
    /// row it had. This is the "do not silently break an existing maplecw.db" check.
    #[test]
    fn a_database_without_the_columns_is_upgraded_without_losing_rows() {
        let (store, account_id, id) = store_with_character();
        let seed = store.create_migration(account_id, id, 0, 0).unwrap();
        {
            let conn = store.conn();
            conn.execute("ALTER TABLE migrations DROP COLUMN token_hash", []).unwrap();
            conn.execute("ALTER TABLE migrations DROP COLUMN peer", []).unwrap();
            let n: i64 = conn
                .query_row("SELECT COUNT(*) FROM migrations", [], |r| r.get(0))
                .unwrap();
            assert_eq!(n, 1, "the row must survive the downgrade for this test to mean anything");
        }
        // The claim path ensures the columns itself, so the pre-existing row is still
        // claimable rather than raising - an Err here would freeze the client's whole UI.
        assert!(store
            .claim_migration_with(seed, &MigrationEvidence::none(), PeerPolicy::Record)
            .unwrap()
            .migration()
            .is_some());
    }

    /// The login server binds to the sign-in that staked the claim. That read has to exist
    /// and has to respect the same liveness the claim itself does.
    #[test]
    fn the_live_claims_token_hash_is_readable_and_expires() {
        let (store, account_id, _) = store_with_character();
        assert_eq!(store.live_claim_token_hash().unwrap(), None, "no claim staked yet");

        store.stake_login_claim(account_id, TOKEN, 3600).unwrap();
        assert_eq!(store.live_claim_token_hash().unwrap(), Some(hash_token(TOKEN)));

        // A claim with a non-positive ttl is already expired, and must not be readable.
        store.stake_login_claim(account_id, TOKEN, -1).unwrap();
        assert_eq!(store.live_claim_token_hash().unwrap(), None);
    }

    /// A disabled account's claim must not keep minting bound migrations in its name.
    #[test]
    fn a_disabled_accounts_claim_stops_being_readable() {
        let (store, account_id, _) = store_with_character();
        store.stake_login_claim(account_id, TOKEN, 3600).unwrap();
        assert!(store.live_claim_token_hash().unwrap().is_some());
        store
            .conn()
            .execute("UPDATE accounts SET enabled = 0 WHERE id = ?1", [account_id])
            .unwrap();
        assert_eq!(store.live_claim_token_hash().unwrap(), None);
    }
}
