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
//!
//! # How a channel server satisfies a binding without the client carrying anything
//!
//! The two measurements above say the client cannot help, so for a while `--bind-migrations`
//! was un-switchable: a bound row is refused by a channel presenting nothing, and that was
//! every channel server. Turning it on would have refused **every** character select and read
//! on screen as a total outage.
//!
//! [`Store::attest_channel_connection`] is the missing half. It takes the peer address of a
//! socket the server accepted and walks `address -> owning process -> the login claim
//! registered to that process -> that claim's token_hash`. Every link is read by the server -
//! the pid from the kernel's TCP table, the claim from this database - so none of it is
//! asserted by the connection. The result is an [`AttestedTokenHash`], and
//! [`MigrationEvidence::with_token_hash`] is the only thing that accepts one.
//!
//! **The invariant is not weakened by any of it.** A bound row still demands a credential that
//! hashes to its `token_hash`; all that changed is the *form* the credential may arrive in.
//! Presenting nothing is still refused, so omitting the token is still not a way past the
//! binding.
//!
//! Three things it is not, and all three have to be said out loud:
//!
//! * **not authentication.** It proves which *process* opened a socket, not who is at the
//!   keyboard. `CLAUDE.md`'s standing constraint is untouched.
//! * **same-machine only.** An off-box peer has no row in this machine's TCP table, so it
//!   cannot be attested and a bound migration will be refused for it. That refusal is
//!   deliberate - see [`Attestation::NotAttributable`] - and it is the reason binding stays
//!   off by default until a run shows an `ATTESTED` line.
//! * **not proof against a recycled process id.** See [`launchtime`], which measures the fact
//!   that would close it and says exactly what closing it costs.

use std::net::SocketAddr;

use rand::RngCore;
use rusqlite::{Connection, OptionalExtension};

use crate::claims::{ClaimEvidence, ClaimResolution, ResolvedBy};
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

/// **A `token_hash` the server derived from a fact the operating system supplied.**
///
/// The only way to obtain one is [`Store::attest_channel_connection`], whose single argument
/// is the peer address of a socket **this process accepted**. There is no constructor from a
/// `String`, no `From`, no public field and no deserialiser, so a value that arrived *on a
/// connection* cannot become one of these.
///
/// That is the whole reason this is a type rather than a second `Option<String>` beside
/// [`MigrationEvidence::token`]. `CLAUDE.md`: *"a comment describing a guarantee is not the
/// guarantee. Put it where it is enforced."* The enforcement here is the privacy of `hash`,
/// which the compiler checks, not this paragraph. The two `compile_fail` doctests on
/// [`MigrationEvidence::with_token_hash`] are what make that assertable rather than asserted.
///
/// It is deliberately **not** `Clone`: one attestation, one evidence. Nothing needs a second
/// copy and every copy is one more place a hash can be carried to.
pub struct AttestedTokenHash {
    hash: String,
    pid: u32,
}

impl AttestedTokenHash {
    /// The process the operating system attributed the connection to. Not a secret, and the
    /// number worth putting in a log line.
    pub fn pid(&self) -> u32 {
        self.pid
    }
}

/// Redacted, because a `{:?}` of a struct is how a value reaches a log without anybody
/// deciding to put it there - the same reasoning [`crate::claims::StakedClaim`] carries. A
/// stored hash is not a secret; there is still no reason for it to travel.
impl std::fmt::Debug for AttestedTokenHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttestedTokenHash")
            .field("pid", &self.pid)
            .field("hash", &"<redacted>")
            .finish()
    }
}

/// What [`Store::attest_channel_connection`] could establish about a connection.
///
/// **Every variant is a log line**, and the five that are not [`Attestation::Attested`] are
/// the ones that matter: they are the states in which a bound migration will be refused, and
/// `CLAUDE.md`'s standing complaint is about guards whose answers nobody reports. Deliberately
/// not an `Option`, for the reason [`ClaimOutcome`] is not one - collapsing five different
/// causes into `None` throws away the only sentence worth reading when the owner cannot get into
/// the world.
#[derive(Debug)]
pub enum Attestation {
    /// The OS attributes this connection to a process, and exactly one live login claim is
    /// bound to that process, and that claim carries a token hash. **This is the only variant
    /// that can satisfy a bound migration.**
    Attested(AttestedTokenHash),
    /// The peer could not be attributed to a process on this machine: it is off-box, the
    /// socket has already closed, the table could not be read, or this is not Windows.
    NotAttributable { peer: SocketAddr },
    /// Attributed to a process, but no live claim is bound to it - and more than one claim is
    /// live, so there is not even a single answer to fall back to. The launcher never
    /// registered this launch, or Windows recycled the pid.
    NoClaimForProcess { pid: u32, live: usize },
    /// Attributed to a process, and there is no live login claim at all. Nobody has signed in
    /// through the launcher.
    NoLiveClaim { pid: u32 },
    /// **A claim resolved, but not by the process rule.** Refused rather than used - see the
    /// safety argument on [`Store::attest_channel_connection`]. This is the variant that stops
    /// a sole live claim from being handed to a connection nothing identified.
    NotByProcess { pid: u32, how: ResolvedBy },
    /// The claim bound to this process predates the `token_hash` column, so there is nothing
    /// to attest. Signing in again mints one.
    ClaimHasNoTokenHash { pid: u32 },
    /// The claim table could not be read. Reported, never fatal: an unanswered packet freezes
    /// the client's whole UI.
    Unreadable { why: String },
}

impl Attestation {
    /// The attested hash, if there is one. **Do not use this to collapse the other five
    /// variants into "no evidence" silently** - log [`Attestation::why`] first.
    pub fn attested(&self) -> Option<&AttestedTokenHash> {
        match self {
            Attestation::Attested(a) => Some(a),
            _ => None,
        }
    }

    /// The whole sentence a caller should log, whichever way it went.
    ///
    /// Written here rather than at the call site because there are seven outcomes and six of
    /// them are the uninteresting-looking ones that get summarised into silence.
    pub fn why(&self) -> String {
        match self {
            Attestation::Attested(a) => format!(
                "ATTESTED: the operating system says process {} owns this connection, and that \
                 process is the client one live launcher sign-in registered. This connection \
                 may present that sign-in's credential, so a migration bound to it can be \
                 claimed. Nothing here authenticates the person: it proves which PROCESS \
                 opened the socket{}",
                a.pid,
                launchtime::describe(a.pid),
            ),
            Attestation::NotAttributable { peer } => format!(
                "NOT ATTESTED: {peer} could not be attributed to a process on this machine - it \
                 is off-box, already closed, or this is not Windows. This connection presents \
                 NO credential, so a BOUND migration will be REFUSED and an unbound one will \
                 not. That refusal is the design: falling back here would let any connection \
                 past the binding by simply being unattributable"
            ),
            Attestation::NoClaimForProcess { pid, live } => format!(
                "NOT ATTESTED: this connection is owned by process {pid}, and none of the \
                 {live} live login claims is registered to it. Either the launcher could not \
                 report the client's pid (an elevated launch may not return hProcess), or \
                 Windows recycled the pid. A bound migration will be REFUSED"
            ),
            Attestation::NoLiveClaim { pid } => format!(
                "NOT ATTESTED: this connection is owned by process {pid}, but no launcher \
                 sign-in is live at all. Run maplecw-launcher and sign in. A bound migration \
                 will be REFUSED; an unbound one is unaffected"
            ),
            Attestation::NotByProcess { pid, how } => format!(
                "NOT ATTESTED, ON PURPOSE: a login claim did resolve for process {pid}, but by \
                 {} - not by the process rule. REFUSED. Accepting it would hand that claim's \
                 credential to any connection the process rule could not identify, including \
                 one from another machine, and the binding would then buy nothing",
                how.describe()
            ),
            Attestation::ClaimHasNoTokenHash { pid } => format!(
                "NOT ATTESTED: process {pid} is registered to a live login claim, but that \
                 claim was staked before token hashes were recorded, so there is nothing to \
                 present. Sign in again through maplecw-launcher. A bound migration will be \
                 REFUSED"
            ),
            Attestation::Unreadable { why } => format!(
                "NOT ATTESTED: the login claim table could not be read ({why}). A bound \
                 migration will be REFUSED. This is a server fault, not an impersonation \
                 attempt, and it is reported rather than raised because an unanswered packet \
                 freezes the client's entire UI"
            ),
        }
    }
}

/// What a connection presents when it tries to claim a migration.
///
/// Built by the channel server from the connection itself. Every field is optional because
/// **the game socket carries no credential**: the client sends no token (measured - the seed
/// does not come back), and the peer address is observable but is explicitly *not* the
/// discriminator. The one credential a channel server can obtain today does not come off the
/// wire at all - see [`MigrationEvidence::with_token_hash`].
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
    /// A SHA-256 the **server** derived, never one a connection sent.
    ///
    /// **Private, and that privacy is the guard.** A public field here would be settable from
    /// any `String`, and the first `String` to hand at a channel server is a packet body.
    /// [`MigrationEvidence::with_token_hash`] is the only way in and it takes an
    /// [`AttestedTokenHash`], which only [`Store::attest_channel_connection`] can mint.
    token_hash: Option<String>,
}

impl MigrationEvidence {
    /// A connection that presented nothing.
    pub fn none() -> Self {
        Self::default()
    }

    /// A connection presenting a session token.
    pub fn with_token(token: impl Into<String>) -> Self {
        Self { token: Some(token.into()), peer: None, token_hash: None }
    }

    /// A connection the **server** attributed to a registered launch.
    ///
    /// This is what makes a bound migration claimable without the client carrying anything:
    /// the channel server goes peer address -> owning process (the OS's TCP table) -> the
    /// login claim registered to that process -> that claim's `token_hash`. Every step is a
    /// fact the server reads for itself, which is why it can stand in for a credential the
    /// client cannot send.
    ///
    /// # The footgun this signature exists to remove
    ///
    /// A hash is a `String`, and a channel server is knee-deep in `String`s that came off a
    /// socket. Taking an [`AttestedTokenHash`] instead means a hash from a packet body cannot
    /// reach this function at all - it is a type error, not a review comment. Both of these
    /// fail to compile, and that is the assertion:
    ///
    /// ```compile_fail
    /// // A hash that arrived on a connection cannot be made into an attestation.
    /// let from_the_wire = String::from("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    /// let forged = store::migration::AttestedTokenHash { hash: from_the_wire, pid: 0 };
    /// ```
    ///
    /// ```compile_fail
    /// // And the field it would be written to is not reachable either.
    /// let mut e = store::migration::MigrationEvidence::none();
    /// e.token_hash = Some(String::from("anything at all"));
    /// ```
    pub fn with_token_hash(attested: AttestedTokenHash) -> Self {
        Self { token: None, peer: None, token_hash: Some(attested.hash) }
    }

    /// Record the observed source address alongside whatever else was presented.
    pub fn from_peer(mut self, peer: impl Into<String>) -> Self {
        self.peer = Some(peer.into());
        self
    }

    /// **The one hash this connection is presenting**, whichever form it arrived in.
    ///
    /// Both public constructors are associated functions that set exactly one credential, so
    /// the mixed case is not reachable through this module's API. It is still handled rather
    /// than `unreachable!()`d, and handled in the safe direction: two credentials that
    /// disagree present *nothing*, so a mixed evidence can never be stronger than either half
    /// of it. A `panic!` here would be worse than a refusal - it would drop the connection,
    /// and an unanswered packet freezes the client's entire UI.
    fn presented_hash(&self) -> Option<String> {
        match (self.token.as_deref(), self.token_hash.as_deref()) {
            (Some(token), None) => Some(hash_token(token)),
            (None, Some(hash)) => Some(hash.to_string()),
            (None, None) => None,
            (Some(token), Some(hash)) => (hash_token(token) == hash).then(|| hash.to_string()),
        }
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

    /// **What this channel connection may present, derived from facts the client cannot
    /// assert.**
    ///
    /// `peer address -> owning process -> the login claim registered to that process -> that
    /// claim's token_hash`. Every link is read by the server: the pid comes from the kernel's
    /// TCP table keyed by the socket this process accepted ([`crate::peerowner`]), and the
    /// claim comes from this database. Nothing in the chain is supplied by the connection, so
    /// the result can stand in for a credential the client is measurably unable to send.
    ///
    /// # This is what unblocks `--bind-migrations`, and what it is not
    ///
    /// It is **not authentication** and must never be reported as if it were. It proves which
    /// *process* opened a socket, not who is at the keyboard, and it is same-machine only. The
    /// standing constraint in `CLAUDE.md` is untouched: the game socket carries no credentials.
    ///
    /// What it does buy is exactly the missing half of the migration binding. The login server
    /// binds a migration to the sign-in that authorised it; until now no channel server could
    /// satisfy that binding, so turning `--bind-migrations` on would have refused **every**
    /// character select. With this, a same-machine client that the launcher registered can.
    ///
    /// # Only [`ResolvedBy::LaunchPid`] is accepted, and that is the whole safety argument
    ///
    /// [`Store::resolve_login_claim`] is a *ladder*: when the process rule finds nothing it
    /// falls through to the address, and then to "there was only one live claim"
    /// ([`ResolvedBy::SoleLiveClaim`]). Those weaker rules are right for deciding **which
    /// account to serve a credential-less login connection as** - a deliberate degrade with a
    /// fallback behind it. They are catastrophically wrong here, because here the answer is
    /// **used as a credential**: with one live claim, `SoleLiveClaim` would hand that claim's
    /// hash to *any* connection, including one from another machine that could not be
    /// attributed at all, and a bound migration would then be claimable by anybody. The
    /// binding would buy nothing while looking like it worked.
    ///
    /// So the resolution is accepted only when it came from the process rule, and anything
    /// else becomes [`Attestation::NotByProcess`].
    /// `a_sole_live_claim_is_never_an_attestation` is the test, and removing this one guard is
    /// the injection it was watched to fail against.
    ///
    /// # Infallible on purpose
    ///
    /// A `Result` here would be handled as `.unwrap_or(nothing)` at some call site eventually,
    /// and the sentence explaining the failure would go with it. A read failure is
    /// [`Attestation::Unreadable`], which carries the message and is refused like any other
    /// non-attestation. The caller always has something to log and always answers the client.
    pub fn attest_channel_connection(&self, peer: SocketAddr) -> Attestation {
        // The ONLY input is the address of a socket this process accepted. There is no
        // overload taking a pid and no overload taking a hash, so nothing a connection sends
        // can steer this - which is the property `with_token_hash` needs to be safe.
        match crate::peerowner::owning_pid(peer) {
            Some(pid) => self.attest_process(pid),
            None => Attestation::NotAttributable { peer },
        }
    }

    /// The claim half of [`Store::attest_channel_connection`], split out **only** so the tests
    /// can exercise it without opening real sockets.
    ///
    /// **Deliberately private.** A `pub` version would take a `u32`, and a `u32` is exactly
    /// the kind of value a packet body is full of. The public entry point takes a
    /// `SocketAddr` so the pid can only ever come from the OS.
    fn attest_process(&self, pid: u32) -> Attestation {
        let resolved = match self.resolve_login_claim(&ClaimEvidence::with_launch_pid(pid)) {
            Ok(r) => r,
            Err(e) => return Attestation::Unreadable { why: e.to_string() },
        };
        match resolved {
            // THE GUARD. See the doc block above; without the `how` test this arm swallows
            // SoleLiveClaim and the binding stops meaning anything.
            ClaimResolution::Resolved(r) if r.how == ResolvedBy::LaunchPid => match r.token_hash {
                Some(hash) => Attestation::Attested(AttestedTokenHash { hash, pid }),
                None => Attestation::ClaimHasNoTokenHash { pid },
            },
            ClaimResolution::Resolved(r) => Attestation::NotByProcess { pid, how: r.how },
            ClaimResolution::NoClaim => Attestation::NoLiveClaim { pid },
            ClaimResolution::Ambiguous { live, .. } => {
                Attestation::NoClaimForProcess { pid, live }
            }
        }
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
        //
        // `presented_hash` is the ONLY widening this gained when server-side attestation was
        // added, and it widens the *form* the credential arrives in, never the test: a plain
        // token is hashed, an attested hash is used as-is, and both are compared against the
        // stored hash by equality. Presenting nothing is still `TokenMissing`, so an attacker
        // still cannot get past a binding by omitting the credential - which is the property
        // the module docs call un-downgradable.
        if let Some(expected) = token_hash.as_deref() {
            match evidence.presented_hash().as_deref() {
                None => return Ok(ClaimOutcome::Refused(Refusal::TokenMissing)),
                // Compared as hashes, so the plain token is never held beside the stored one.
                Some(presented) if presented != expected => {
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

/// **When the process behind an attributed pid started.** Observation, never a refusal.
///
/// # Why this is here and why it does not refuse anything
///
/// Windows reuses process ids and a login claim lives twelve hours
/// ([`crate::claims::LOGIN_CLAIM_TTL_SECS`]). If the client exits and Windows hands its pid to
/// another process, that process's connections are attributed to the dead launch's claim -
/// by [`Store::attest_process`] here, and equally by the **login** server's rule 2, which has
/// shipped with that property since per-launch claims existed. This module does not widen
/// that hole and cannot close it either, because closing it needs a value nobody records.
///
/// The check that would close it is: store the client process's creation time beside
/// `login_claims.launch_pid` when the launcher registers the launch, and refuse an
/// attribution whose process was created at a different time. That is one nullable column,
/// one write in `Store::bind_pid` and one comparison here - roughly fifteen lines, all of them
/// in `crates/store/src/claims.rs`. It is genuinely cheap; it is simply not in this file.
///
/// **What is NOT a sound check, and was tried first:** comparing the process's creation time
/// against the claim's `created_at`. A recycled-pid attacker has to drive the login server to
/// mint the migration anyway, so their process is necessarily *also* older than the migration
/// row - the comparison passes for both. Bounding it instead ("within N seconds of the
/// stake") refuses a legitimate client whose claim was re-staked while it was running, and
/// `CLAUDE.md` is explicit that a check which can lock the owner out of their own server does not
/// cost nothing. There is no sound refusal available without the stored value, so this
/// **reports** rather than guessing, and [`Attestation::why`] puts the number in `world.log`
/// where a later investigation can read it.
pub mod launchtime {
    /// The 100-nanosecond FILETIME at which `pid`'s process was created.
    ///
    /// `None` is ordinary: the process has gone, or it runs at an integrity level this one
    /// may not query. Never an error.
    pub fn started_at(pid: u32) -> Option<u64> {
        platform::started_at(pid)
    }

    /// 100-nanosecond FILETIME ticks between 1601-01-01 and the Unix epoch. The same constant
    /// `world::session`'s clock base uses, and it is a claim from a header like any other -
    /// `the_epoch_constant_puts_this_process_in_the_present` is what makes it falsifiable.
    const FILETIME_1970: u64 = 116_444_736_000_000_000;

    /// Unix seconds for a FILETIME, or `None` if it predates 1970 (which would mean the value
    /// is not a creation time at all).
    pub fn unix_seconds(filetime: u64) -> Option<i64> {
        filetime
            .checked_sub(FILETIME_1970)
            .map(|since_epoch| (since_epoch / 10_000_000) as i64)
    }

    /// A clause to append to a log sentence, or an empty string when nothing could be read.
    ///
    /// The **age** is the number worth printing. A claim staked eleven hours ago whose
    /// registered process has been alive for forty seconds is the signature of a recycled pid
    /// (or of a relaunch), and that is invisible from the pid alone.
    pub fn describe(pid: u32) -> String {
        let Some(ft) = started_at(pid) else {
            return String::new();
        };
        let Some(started) = unix_seconds(ft) else {
            return String::new();
        };
        let now = crate::db::Store::now();
        format!(
            ". That process started at unix {started}, {} seconds ago - compare it against how \
             long ago the sign-in was if a recycled process id is suspected",
            now - started
        )
    }

    /// **Prove the reader works, on this machine, right now.**
    ///
    /// Three controls, and the middle one is the only one that can catch a wrong field:
    ///
    /// * a **positive** - this process has a creation time, and it is in the past;
    /// * a **direction** control - a process started *now* must have a creation time strictly
    ///   later than this one's. Reading the wrong `FILETIME` out of `GetProcessTimes` (exit,
    ///   kernel or user time) produces a plausible-looking number that fails exactly here, and
    ///   nowhere else. This is the same shape as `peerowner`'s `curl` test, and for the same
    ///   reason: a same-subject positive cannot see a swapped field;
    /// * a **negative** - pid 0 is the idle process and can never be opened, so "it returns
    ///   None" can be told from "it returns None for everything".
    ///
    /// Returns `Err` naming the numbers it actually saw, so a failure is a measurement.
    pub fn self_test() -> std::result::Result<String, String> {
        let me = std::process::id();
        let mine = started_at(me).ok_or_else(|| {
            format!("this process ({me}) has no creation time, so the reader is not working")
        })?;
        let mine_unix = unix_seconds(mine)
            .ok_or_else(|| format!("this process's creation time {mine} predates 1970"))?;
        let now = crate::db::Store::now();
        if mine_unix > now {
            return Err(format!(
                "this process claims to have started at unix {mine_unix}, which is after now \
                 ({now}) - the field being read is not a creation time"
            ));
        }

        // The direction control. `Child` is held until after the read, so the pid cannot be
        // recycled between the spawn and the query even if the child has already exited.
        let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".into());
        let cmd = std::path::PathBuf::from(&system_root).join("System32").join("cmd.exe");
        if !cmd.is_file() {
            return Err(format!(
                "no {} - the direction control cannot run, so a swapped FILETIME field would \
                 be invisible",
                cmd.display()
            ));
        }
        let mut child = std::process::Command::new(&cmd)
            .args(["/c", "exit"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|e| format!("could not start {}: {e}", cmd.display()))?;
        let child_pid = child.id();
        let theirs = started_at(child_pid);
        let _ = child.kill();
        let _ = child.wait();

        let theirs = theirs.ok_or_else(|| {
            format!("the child process {child_pid} has no creation time, although it was just \
                     started and its handle is still open")
        })?;
        if theirs <= mine {
            return Err(format!(
                "the child started at {theirs} and this process at {mine}, but the child was \
                 started second - GetProcessTimes' first out-parameter is not the creation \
                 time, or the two halves of the FILETIME are swapped"
            ));
        }

        if started_at(0).is_some() {
            return Err(
                "pid 0 reported a creation time, so this reader answers for anything and its \
                 negatives mean nothing"
                    .to_string(),
            );
        }
        Ok(format!(
            "process start times are readable: this process ({me}) started at unix {mine_unix}"
        ))
    }

    #[cfg(windows)]
    mod platform {
        use std::ffi::c_void;

        /// The narrowest right that can read a creation time. `PROCESS_QUERY_INFORMATION`
        /// would work too and asks for more than is needed.
        const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;

        /// `FILETIME`. Two `DWORD`s, and **not** in a `pshpack` region - unlike every
        /// `MINIDUMP_*` structure, which is the mistake `CLAUDE.md` records. The assertion in
        /// `the_filetime_struct_is_the_documented_size` is what makes that a fact here rather
        /// than a claim read out of a header.
        #[repr(C)]
        #[derive(Default, Clone, Copy)]
        struct FileTime {
            low: u32,
            high: u32,
        }

        impl FileTime {
            fn as_u64(self) -> u64 {
                ((self.high as u64) << 32) | self.low as u64
            }
        }

        #[link(name = "kernel32")]
        extern "system" {
            fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut c_void;
            fn CloseHandle(handle: *mut c_void) -> i32;
            fn GetProcessTimes(
                process: *mut c_void,
                creation: *mut FileTime,
                exit: *mut FileTime,
                kernel: *mut FileTime,
                user: *mut FileTime,
            ) -> i32;
        }

        pub(super) fn started_at(pid: u32) -> Option<u64> {
            // SAFETY: a null return is the documented failure and is checked before use.
            let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };
            if handle.is_null() {
                return None;
            }
            let (mut creation, mut exit, mut kernel, mut user) = (
                FileTime::default(),
                FileTime::default(),
                FileTime::default(),
                FileTime::default(),
            );
            // SAFETY: `handle` is non-null and owned here, and all four out-parameters point
            // at live locals of exactly the type the API writes.
            let ok = unsafe {
                GetProcessTimes(handle, &mut creation, &mut exit, &mut kernel, &mut user)
            };
            // SAFETY: closed exactly once, on every path, before the value is used.
            unsafe { CloseHandle(handle) };
            (ok != 0).then(|| creation.as_u64())
        }

        #[cfg(test)]
        mod layout {
            use super::*;

            #[test]
            fn the_filetime_struct_is_the_documented_size() {
                assert_eq!(std::mem::size_of::<FileTime>(), 8);
                assert_eq!(std::mem::align_of::<FileTime>(), 4);
            }

            /// The halves are combined high-then-low. Getting this backwards produces a number
            /// that is still "a big u64" and still increases, so only an explicit case catches
            /// it.
            #[test]
            fn the_two_halves_combine_high_first() {
                let ft = FileTime { low: 0x9ABC_DEF0, high: 0x1234_5678 };
                assert_eq!(ft.as_u64(), 0x1234_5678_9ABC_DEF0);
            }
        }
    }

    #[cfg(not(windows))]
    mod platform {
        /// Not Windows, so there is no `GetProcessTimes`. `None` is the honest answer and is
        /// the same answer a vanished process gives, so callers already handle it. This
        /// project only runs on Windows; the arm exists so the crate compiles elsewhere rather
        /// than as a claim that anything has been tested there.
        pub(super) fn started_at(_pid: u32) -> Option<u64> {
            None
        }
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

    // ---------------------------------------------------------------------------------
    // SERVER-SIDE ATTESTATION. This is what makes `--bind-migrations` switchable: the
    // channel server derives the credential the client cannot send. Every test below is
    // paired with an injection listed in the report - the guard it covers was removed, the
    // test was watched to fail, and the guard put back.
    // ---------------------------------------------------------------------------------

    /// A store with an account, a character, a live launcher sign-in, and that sign-in's
    /// client process registered - which is the whole production shape in five lines.
    ///
    /// Returns the character id, the pid, and the claim's `token_hash`. The hash is read back
    /// through the resolver rather than computed from the token, so the test mints its
    /// migration with exactly the value `crates/login` would.
    fn store_with_a_registered_launch(pid: u32) -> (Store, i64, u32, String) {
        let store = Store::open_in_memory().unwrap();
        let account_id = store.create_account("wisp", "correct horse battery").unwrap();
        let id = store
            .create_character(account_id, 0, &seeded_character("Wanderer"))
            .unwrap()
            .id;
        let crate::db::AuthOutcome::Ok { token, .. } =
            store.authenticate("wisp", "correct horse battery").unwrap()
        else {
            panic!("the test account must authenticate");
        };
        store
            .stake_login_claim_with(account_id, &token, 3600, Some("127.0.0.1"))
            .unwrap();
        assert!(
            store.bind_launch_pid_by_token(&token, pid).unwrap(),
            "the launch must register, or every assertion below is about an unbound claim"
        );
        (store, account_id, id, hash_token(&token))
    }

    /// **The headline.** A migration bound to a launcher sign-in is claimed by the client
    /// process that sign-in started, presenting nothing on the wire.
    ///
    /// The control is the line above the assertion: the same row refuses
    /// `MigrationEvidence::none()`. Without it a passing test would be indistinguishable from
    /// the row never having been bound at all.
    #[test]
    fn an_attested_process_claims_the_migration_bound_to_its_sign_in() {
        const PID: u32 = 4242;
        let (store, account_id, id, claim_hash) = store_with_a_registered_launch(PID);
        let seed = store
            .create_migration_bound_hash(account_id, id, 0, 0, Some(&claim_hash), Some("127.0.0.1"))
            .unwrap();

        // THE CONTROL. This row really is bound, so the pass below is about the attestation.
        assert_eq!(
            store
                .claim_migration_with(seed, &MigrationEvidence::none(), PeerPolicy::Record)
                .unwrap(),
            ClaimOutcome::Refused(Refusal::TokenMissing),
            "if this claims, the row was never bound and the next assertion means nothing"
        );

        let attestation = store.attest_process(PID);
        let attested = attestation
            .attested()
            .unwrap_or_else(|| panic!("the registered launch must attest: {}", attestation.why()));
        assert_eq!(attested.pid(), PID);

        let Attestation::Attested(attested) = store.attest_process(PID) else {
            unreachable!("just checked")
        };
        match store
            .claim_migration_with(
                seed,
                &MigrationEvidence::with_token_hash(attested).from_peer("127.0.0.1"),
                PeerPolicy::Record,
            )
            .unwrap()
        {
            ClaimOutcome::Claimed { migration, peer_mismatch } => {
                assert_eq!(migration.character_id, id);
                assert!(!peer_mismatch);
            }
            other => panic!("the attested client must get its own character, got {other:?}"),
        }
    }

    /// **The guard that carries the whole design, stated as a test.**
    ///
    /// `Store::resolve_login_claim` is a ladder that falls through to "there was only one live
    /// claim". That is the right answer for *which account to serve a login connection as*,
    /// and a catastrophic one here: it would hand the only sign-in's credential to any
    /// connection the process rule could not identify - including one from another machine -
    /// and a bound migration would be claimable by anybody while the binding looked like it
    /// worked.
    ///
    /// One live claim, and a pid nobody registered.
    #[test]
    fn a_sole_live_claim_is_never_an_attestation() {
        let (store, _, _, _) = store_with_a_registered_launch(4242);

        // The control: the ladder really does resolve this connection, by rule 4. So the
        // refusal below is the `how` guard doing its job and not an empty claim table.
        let ladder = store
            .resolve_login_claim(&crate::claims::ClaimEvidence::with_launch_pid(9999))
            .unwrap();
        assert_eq!(
            ladder.resolved().expect("rule 4 answers with one live claim").how,
            ResolvedBy::SoleLiveClaim,
            "if this is not SoleLiveClaim the test is no longer pointed at the hole"
        );

        match store.attest_process(9999) {
            Attestation::NotByProcess { pid, how } => {
                assert_eq!(pid, 9999);
                assert_eq!(how, ResolvedBy::SoleLiveClaim);
            }
            other => panic!(
                "a claim resolved by anything but the process rule must NOT attest, got {other:?}"
            ),
        }
    }

    /// The same hole from the other end: an unregistered process must not be able to claim a
    /// bound migration, even when its sign-in is the only one in the database.
    #[test]
    fn an_unregistered_process_cannot_claim_the_only_bound_migration() {
        let (store, account_id, id, claim_hash) = store_with_a_registered_launch(4242);
        let seed = store
            .create_migration_bound_hash(account_id, id, 0, 0, Some(&claim_hash), None)
            .unwrap();

        let attestation = store.attest_process(9999);
        assert!(attestation.attested().is_none(), "{}", attestation.why());
        let evidence = match attestation.attested() {
            Some(_) => unreachable!(),
            None => MigrationEvidence::none(),
        };
        assert_eq!(
            store.claim_migration_with(seed, &evidence, PeerPolicy::Record).unwrap(),
            ClaimOutcome::Refused(Refusal::TokenMissing)
        );
        // And the rightful client still gets in afterwards - a refusal must not burn the row.
        let Attestation::Attested(a) = store.attest_process(4242) else {
            panic!("{}", store.attest_process(4242).why())
        };
        assert!(store
            .claim_migration_with(seed, &MigrationEvidence::with_token_hash(a), PeerPolicy::Record)
            .unwrap()
            .migration()
            .is_some());
    }

    /// **The owner's two-clients-on-one-machine case, with attestation doing the separating.**
    ///
    /// Two accounts, two sign-ins, two client processes, one address. Each process must get
    /// its own character and neither may get the other's by naming it.
    #[test]
    fn two_registered_launches_on_one_address_cannot_claim_each_others_characters() {
        let store = Store::open_in_memory().unwrap();
        let a = store.create_account("otter", "correct horse battery").unwrap();
        let b = store.create_account("owl", "correct horse battery").unwrap();
        let a_chr = store.create_character(a, 0, &seeded_character("OtterChr")).unwrap().id;
        let b_chr = store.create_character(b, 0, &seeded_character("OwlChr")).unwrap().id;

        let mut hashes = Vec::new();
        for (account, name, pid) in [(a, "otter", 111u32), (b, "owl", 222)] {
            let crate::db::AuthOutcome::Ok { token, .. } =
                store.authenticate(name, "correct horse battery").unwrap()
            else {
                panic!("{name} must authenticate");
            };
            store.stake_login_claim_with(account, &token, 3600, Some("127.0.0.1")).unwrap();
            assert!(store.bind_launch_pid_by_token(&token, pid).unwrap());
            hashes.push(hash_token(&token));
        }

        // Same address for both, deliberately: "IP cannot be the sole discriminator".
        store
            .create_migration_bound_hash(a, a_chr, 0, 0, Some(&hashes[0]), Some("127.0.0.1"))
            .unwrap();
        store
            .create_migration_bound_hash(b, b_chr, 0, 0, Some(&hashes[1]), Some("127.0.0.1"))
            .unwrap();

        // Owl's client process asserts Otter's character id.
        let Attestation::Attested(owl) = store.attest_process(222) else {
            panic!("{}", store.attest_process(222).why())
        };
        assert_eq!(
            store
                .claim_migration_for_character_with(
                    a_chr,
                    &MigrationEvidence::with_token_hash(owl).from_peer("127.0.0.1"),
                    PeerPolicy::Record,
                )
                .unwrap(),
            ClaimOutcome::Refused(Refusal::TokenMismatch),
            "asserting another account's character id must not be enough, even from the same \
             machine and the same address"
        );

        // And each of them still gets their own.
        for (pid, chr) in [(111u32, a_chr), (222, b_chr)] {
            let Attestation::Attested(who) = store.attest_process(pid) else {
                panic!("{}", store.attest_process(pid).why())
            };
            assert!(
                store
                    .claim_migration_for_character_with(
                        chr,
                        &MigrationEvidence::with_token_hash(who).from_peer("127.0.0.1"),
                        PeerPolicy::Record,
                    )
                    .unwrap()
                    .migration()
                    .is_some(),
                "process {pid} must still get character {chr}"
            );
        }
    }

    /// Every non-attesting state has to be distinguishable, because they need opposite work:
    /// "sign in" is not "the launcher could not report the pid" is not "this peer is off-box".
    /// A single `None` would collapse all three.
    #[test]
    fn every_attestation_outcome_says_which_one_it_is_and_names_the_subject() {
        // No claim at all.
        let store = Store::open_in_memory().unwrap();
        store.create_account("wisp", "correct horse battery").unwrap();
        match store.attest_process(4242) {
            Attestation::NoLiveClaim { pid } => assert_eq!(pid, 4242),
            other => panic!("{other:?}"),
        }
        assert!(store.attest_process(4242).why().contains("4242"));
        assert!(store.attest_process(4242).why().contains("REFUSED"));

        // Registered, so the happy path names the pid too.
        let (registered, _, _, _) = store_with_a_registered_launch(7);
        let why = registered.attest_process(7).why();
        assert!(why.contains("ATTESTED"), "{why}");
        assert!(why.contains('7'), "{why}");

        // Two claims live, neither registered to this pid: ambiguity, not a guess.
        let two = Store::open_in_memory().unwrap();
        for name in ["otter", "owl"] {
            let account = two.create_account(name, "correct horse battery").unwrap();
            let crate::db::AuthOutcome::Ok { token, .. } =
                two.authenticate(name, "correct horse battery").unwrap()
            else {
                panic!()
            };
            two.stake_login_claim(account, &token, 3600).unwrap();
        }
        match two.attest_process(31337) {
            Attestation::NoClaimForProcess { pid, live } => {
                assert_eq!(pid, 31337);
                assert_eq!(live, 2);
            }
            other => panic!("{other:?}"),
        }
        assert!(two.attest_process(31337).why().contains("hProcess"));

        // Off-box: not attributable, and the sentence has to say the refusal is deliberate.
        let peer: SocketAddr = "203.0.113.7:54321".parse().unwrap();
        let why = registered.attest_channel_connection(peer).why();
        assert!(why.contains("203.0.113.7"), "{why}");
        assert!(why.contains("REFUSED"), "{why}");
    }

    /// A claim staked before the `token_hash` column existed has nothing to attest, and that
    /// is a different sentence from "not registered". Both refuse; only one is fixed by
    /// signing in again.
    ///
    /// The column is `NOT NULL` in the `CREATE TABLE` and nullable on the upgrade path -
    /// `ALTER TABLE ADD COLUMN` cannot add a `NOT NULL` column without a default - so the row
    /// is put back the way an older database really holds it rather than by writing a NULL a
    /// fresh schema would reject. `claims::ensure_columns` re-adds the column and the index on
    /// the next read, which is the path a real upgrade takes.
    #[test]
    fn a_claim_with_no_token_hash_attests_nothing_rather_than_attesting_an_empty_string() {
        let (store, _, _, _) = store_with_a_registered_launch(4242);
        {
            let conn = store.conn();
            // The index has to go first: SQLite refuses to drop an indexed column.
            conn.execute("DROP INDEX IF EXISTS idx_login_claims_token", []).unwrap();
            conn.execute("ALTER TABLE login_claims DROP COLUMN token_hash", []).unwrap();
            let n: i64 = conn
                .query_row("SELECT COUNT(*) FROM login_claims", [], |r| r.get(0))
                .unwrap();
            assert_eq!(n, 1, "the claim must survive the downgrade or this test proves nothing");
        }
        match store.attest_process(4242) {
            Attestation::ClaimHasNoTokenHash { pid } => assert_eq!(pid, 4242),
            other => panic!("{other:?}"),
        }
    }

    /// The plain-token path is untouched by the hash path. Both forms of the same credential
    /// must satisfy the same row, or `crates/login`'s tests are testing a second mechanism.
    #[test]
    fn a_plain_token_and_an_attested_hash_satisfy_the_same_binding() {
        let (store, account_id, id, claim_hash) = store_with_a_registered_launch(4242);
        let by_hash = store
            .create_migration_bound_hash(account_id, id, 0, 0, Some(&claim_hash), None)
            .unwrap();
        let by_token = store
            .create_migration_bound(account_id, id, 0, 0, Some(TOKEN), None)
            .unwrap();

        let Attestation::Attested(a) = store.attest_process(4242) else { panic!() };
        assert!(store
            .claim_migration_with(by_hash, &MigrationEvidence::with_token_hash(a), PeerPolicy::Record)
            .unwrap()
            .migration()
            .is_some());
        assert!(store
            .claim_migration_with(by_token, &MigrationEvidence::with_token(TOKEN), PeerPolicy::Record)
            .unwrap()
            .migration()
            .is_some());
    }

    /// Two credentials that disagree must present **nothing**, so a mixed evidence can never
    /// be stronger than either half. Not reachable through the public constructors - both are
    /// associated functions that set exactly one - which is why it is asserted here rather
    /// than assumed.
    #[test]
    fn disagreeing_credentials_present_nothing_rather_than_the_stronger_one() {
        let mut mixed = MigrationEvidence::with_token(TOKEN);
        mixed.token_hash = Some(hash_token(OTHER));
        assert_eq!(mixed.presented_hash(), None);

        // Agreeing is fine, and is the control: without it the assertion above would pass on
        // a `presented_hash` that simply always returns None.
        let mut agreeing = MigrationEvidence::with_token(TOKEN);
        agreeing.token_hash = Some(hash_token(TOKEN));
        assert_eq!(agreeing.presented_hash(), Some(hash_token(TOKEN)));
    }

    /// An attested hash must never reach a log through a `{:?}`.
    #[test]
    fn an_attested_hash_is_redacted_in_debug() {
        let (store, _, _, claim_hash) = store_with_a_registered_launch(4242);
        let Attestation::Attested(a) = store.attest_process(4242) else { panic!() };
        let debug = format!("{a:?}");
        assert!(!debug.contains(&claim_hash), "the hash must not be printable: {debug}");
        assert!(debug.contains("4242"), "the pid is not a secret and is worth logging: {debug}");
        // And the same through the enum, which is what a caller actually holds.
        let whole = format!("{:?}", store.attest_process(4242));
        assert!(!whole.contains(&claim_hash), "{whole}");
    }

    /// **The whole path, over a real socket, with the connection opened by a different
    /// process.**
    ///
    /// Every other test here hands `attest_process` a pid directly, which proves the claim
    /// half and says nothing about the half that matters most: whether an *accepted socket*
    /// resolves to the process on the far end of it. That is exactly the blind spot
    /// `peerowner`'s own docs name - a lookup that matched the table's remote columns instead
    /// of its local ones would attribute every connection to this server, and no launch would
    /// ever attest while the code looked right.
    ///
    /// So this opens a listener, has **`curl.exe` (a different process) connect to it**,
    /// registers that child's pid as the launch, and then runs the production entry point on
    /// the address the accept produced. It is the same instrument
    /// `a_connection_from_another_process_is_attributed_to_that_process` uses, carried one
    /// step further: through the claim, through the hash, to a migration actually claimed.
    ///
    /// **The negative control is a second connection from this process.** One claim is live,
    /// so the resolution ladder answers `SoleLiveClaim` for it - and it must still not attest.
    /// Without that half, a pass here would be consistent with "everything attests".
    #[cfg(windows)]
    #[test]
    fn a_real_connection_from_a_registered_process_claims_a_bound_migration() {
        let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".into());
        let curl = std::path::PathBuf::from(&system_root).join("System32").join("curl.exe");
        assert!(
            curl.is_file(),
            "no {} - the socket-to-process half of this feature would go unverified",
            curl.display()
        );

        let store = Store::open_in_memory().unwrap();
        let account_id = store.create_account("wisp", "correct horse battery").unwrap();
        let character = store
            .create_character(account_id, 0, &seeded_character("Wanderer"))
            .unwrap()
            .id;
        let crate::db::AuthOutcome::Ok { token, .. } =
            store.authenticate("wisp", "correct horse battery").unwrap()
        else {
            panic!("the test account must authenticate");
        };
        store.stake_login_claim(account_id, &token, 3600).unwrap();

        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("a loopback listener");
        let port = listener.local_addr().expect("an address").port();
        let mut child = std::process::Command::new(&curl)
            .args(["-s", "--max-time", "10", &format!("http://127.0.0.1:{port}/")])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("curl.exe must start");
        let child_pid = child.id();

        // The launcher's half: it started the client, so it knows the pid and registers it.
        assert!(
            store.bind_launch_pid_by_token(&token, child_pid).unwrap(),
            "the launch must register or nothing below is testing attestation"
        );

        // The login server's half: a migration bound to the sign-in that authorised it.
        let seed = store
            .create_migration_bound_hash(
                account_id,
                character,
                0,
                0,
                Some(&hash_token(&token)),
                Some("127.0.0.1"),
            )
            .unwrap();

        // The channel server's half, from here down. `accepted` is held so the TCP row stays.
        let accepted = listener.accept().expect("curl must connect").0;
        let peer = accepted.peer_addr().expect("the accepted socket has a peer");

        // THE NEGATIVE CONTROL, taken first so a panic below still tears the child down.
        let mine = std::net::TcpStream::connect(("127.0.0.1", port)).expect("a second connection");
        let mine_accepted = listener.accept().expect("accept our own").0;
        let mine_peer = mine_accepted.peer_addr().expect("a peer");
        let unregistered = store.attest_channel_connection(mine_peer);

        let attestation = store.attest_channel_connection(peer);

        drop(accepted);
        drop(mine_accepted);
        drop(mine);
        let _ = child.kill();
        let _ = child.wait();

        assert!(
            matches!(unregistered, Attestation::NotByProcess { .. }),
            "a connection from an UNREGISTERED process ({}) must not attest, even with one \
             live claim - got {}",
            std::process::id(),
            unregistered.why()
        );

        let Attestation::Attested(hash) = attestation else {
            panic!(
                "the registered client process {child_pid} opened this socket and must attest: \
                 {}",
                attestation.why()
            );
        };
        assert_eq!(
            hash.pid(),
            child_pid,
            "the attribution must name the process that opened the socket, not this one ({})",
            std::process::id()
        );

        match store
            .claim_migration_with(
                seed,
                &MigrationEvidence::with_token_hash(hash).from_peer("127.0.0.1"),
                PeerPolicy::Record,
            )
            .unwrap()
        {
            ClaimOutcome::Claimed { migration, .. } => assert_eq!(migration.character_id, character),
            other => panic!("the real client connection must get its character, got {other:?}"),
        }
    }

    /// **The instrument, checked before it is believed.** A reader that has never produced a
    /// positive is exactly the shape `CLAUDE.md` keeps warning about, and a swapped
    /// `GetProcessTimes` out-parameter produces a plausible number rather than an error.
    #[cfg_attr(not(windows), ignore = "no GetProcessTimes off Windows")]
    #[test]
    fn process_start_times_are_readable_and_ordered() {
        match launchtime::self_test() {
            Ok(what) => assert!(what.contains("readable"), "{what}"),
            Err(why) => panic!("{why}"),
        }
    }

    /// The epoch constant is a claim from a header like any other. This is the call that can
    /// disagree with it: converted, this process must sit within a day of now.
    #[cfg_attr(not(windows), ignore = "no GetProcessTimes off Windows")]
    #[test]
    fn the_epoch_constant_puts_this_process_in_the_present() {
        let ft = launchtime::started_at(std::process::id()).expect("this process exists");
        let started = launchtime::unix_seconds(ft).expect("after 1970");
        let now = Store::now();
        assert!(
            started <= now && now - started < 86_400,
            "this process started at unix {started} and now is {now} - the FILETIME epoch \
             offset is wrong"
        );
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
