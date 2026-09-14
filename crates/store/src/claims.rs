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
//! does not remove **anybody else's**.
//!
//! # One live claim per account, from 2026-09-13
//!
//! It does remove *your own*. The owner: *"shouldn't they be allowed to sign in multiple times?
//! Signing in again should invalidate previous claims on that account."*
//!
//! Read the two sentences together, because the difference between them is the whole fix and
//! it is one `WHERE` clause: **scoped by account**, a re-sign-in replaces the signer's own
//! earlier claim and leaves every other account's alone. The eviction bug above was
//! `DELETE FROM login_claims` with no predicate at all, which is what made `owl`'s sign-in
//! serve `otter`'s connection as `owl`. That test still passes.
//!
//! What it buys is rule 3. Two live claims staked from one address stop the address picking
//! one out, so *every* connection from that address falls to the fallback - and the second
//! claim was the same person retrying. Measured on the live server on 2026-09-13: six
//! launches from one address, the first three matched, the rest refused with *"4 login claims
//! are live and the evidence presented did not pick one out"*. The guard was right; there was
//! no honest answer to give, and the reason was four sign-ins by one account.
//!
//! What it costs is stated where it is enforced: the superseded launch's client token stops
//! matching a live row, so a client still holding it is resolved as
//! [`ClaimResolution::NoClaim`] at its next login connection. Signing in again while a client
//! of that account is running cuts that client off at its next Log Out or world change.
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
//! | 1a | a session token the connection presented | [`ResolvedBy::Token`] | a credential; nothing on the wire presents one |
//! | 1b | **a one-time client token the CLIENT carried in `0x0073`** | [`ResolvedBy::ClientToken`] | a credential, **on the wire**, once the hook fills the field |
//! | 2 | the OS-attributed owning process of the peer socket | [`ResolvedBy::LaunchPid`] | same machine only; separates two clients on one box |
//! | 3 | the source address, when it picks out exactly one claim | [`ResolvedBy::PeerAddress`] | separates two machines; useless for two clients on one |
//! | 4 | there is exactly one live claim in the whole database | [`ResolvedBy::SoleLiveClaim`] | not a discriminator at all - it is "there was only one answer" |
//!
//! and if none of them does, the answer is [`ClaimResolution::Ambiguous`], **not** a guess.
//!
//! # Rule 1b is the point of the whole file, and everything below 1b is fallback
//!
//! The owner, 2026-08-29: *"if the client itself has a way to carry an identity, I would like to
//! use that way more ... using the client to pass a session should be what we aim for instead
//! of inference."*
//!
//! Rules 2, 3 and 4 are all inference: the server looks at a socket it was handed and guesses
//! which launch opened it. Rule 1b is not. The launcher mints a token at sign-in, writes it
//! where the hook can read it, the hook writes it into the client's own session object, and
//! **the client transmits it itself, through its own cipher, in a packet it already builds**.
//! Nothing is forged and no packet is injected - see `research/client-session-args.md` section 5
//! for why that matters.
//!
//! Rules 2-4 are deliberately kept, and this is not indecision:
//!
//! * the field is **empty in all 72 captured `0x0073` bodies**, so until a client run shows a
//!   non-empty one, removing the weaker rules would remove the only thing that works;
//! * `0x0073` is sent **once per launch**. Measured over 57 archived runs: every run has
//!   exactly one, while 7 runs have two or three `0x0080` login requests, on *separate
//!   connections that carry no identity at all* (Log Out / Choose another world reconnect).
//!   The reconnect is carried by rule 2, and it always will be.
//!
//! # Presenting a WRONG credential buys less than presenting nothing
//!
//! The anti-downgrade rule, mirrored from `crate::migration`'s invariant:
//!
//! > A connection that presents a client token gets **that claim or none**. It never falls
//! > through to rules 2, 3 or 4.
//!
//! Presenting nothing still reaches the weaker rules, and that is not a hole - it is what a
//! stock client does and what every reconnect does. The property that matters is the other
//! direction: **an attacker cannot get past the credential by supplying a wrong one**, and
//! cannot improve on silence by guessing. `CLAUDE.md` on `record_quest_forfeit`: a guard that
//! can be skipped by not presenting the thing it checks is not a guard.
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

use rand::RngCore;
use rusqlite::{Connection, OptionalExtension};

use crate::db::Store;
use crate::error::{Result, StoreError};
use crate::session::{hash_token, new_token};

// ----------------------------------------------------------------------------------------
// THE CLIENT TOKEN: the one credential that travels on the wire.
// ----------------------------------------------------------------------------------------

/// **How many random bytes a client token carries. 16 - 128 bits.**
///
/// # Length is NOT the reason, and the first version of this comment said it was
///
/// This block originally read *"a compromise against an unmeasured client-side limit ... the
/// thing most likely to sink the whole mechanism"*, and picked 16 bytes to stay small. The
/// limit was measured the same day, on the client half (`grap_stub::identity`), and it is
/// enormous:
///
/// * **65535 bytes** is the hard ceiling - `w_str` writes the length prefix with a `u16`
///   truncation (`movzx r8d, cx`) and the body from a `u32`, so past that the stream desyncs;
/// * **~3996 bytes** is the largest frame the client sends without also uploading an
///   oversized-packet report. It still sends;
/// * **6550 bytes** is what it has actually put on this socket in an archived run
///   (`previous-runs/login-20260826-171724.log`, `0x0090`).
///
/// A 26-character token has roughly 150x margin, and **so would 64 hex characters**. So the
/// size worry was unfounded, and it is recorded rather than quietly deleted because it is the
/// live example of this file's own habit: a guess in a doc block reads exactly like a
/// measurement once it has been written down confidently.
///
/// # What the number is actually for
///
/// Entropy, and nothing else. 128 bits from the OS CSPRNG is not guessable over a socket by
/// any margin that matters. The 32-byte session token would be equally safe to carry and is
/// simply more than this needs - and reusing *it* would put a game credential in a plain-text
/// file beside the client, which is the thing `StakedClaim`'s launch id exists to avoid.
///
/// The binding constraint is the character SET, not the count: see [`CLIENT_TOKEN_CHARS`].
pub const CLIENT_TOKEN_BYTES: usize = 16;

/// **The number of characters the client carries: 26.**
///
/// # The constraint that binds is the character set, and it is a hard one
///
/// **The identity is a C string on both sides of the client**, established on the client half
/// (`grap_stub::identity::validate`): `FUN_142c50400` re-derives the length it sends with a
/// `strlen` loop rather than reading the string header, and substitutes an empty literal when
/// the field is null *or begins with a NUL*.
///
/// So an embedded NUL **truncates the credential with no error anywhere**, and a leading one
/// sends nothing at all - and both read on the wire as "the hook did not write the field",
/// which is exactly the outcome that cannot be told apart from the mechanism not working. The
/// hook refuses anything outside `0x21..=0x7e` for that reason. Uppercase base32 is
/// `A-Z` (`0x41..=0x5a`) and `2-7` (`0x32..=0x37`), comfortably inside it, with no NUL, no
/// space, no `=` padding, no `-` and no `_`.
/// `a_client_token_is_within_the_byte_set_the_client_can_carry` asserts that rather than
/// trusting this paragraph, because a character set read off a doc block is a claim.
///
/// # The case-folding worry that motivated base32 is CLOSED, and it was not why it stayed
///
/// This block used to argue that base32 defends against `FUN_142c9f2d0` (`_strupr`) mangling a
/// lower-case token in flight. **That is now measured and it does not happen.** The path from
/// `session+0x1b8` to the wire is four calls and then `w_str`'s two leaf writers, which call
/// nothing; corroborated on the wire by the 16-byte machine GUID that `w_str`'s own byte
/// writer emits into this same packet appearing **verbatim in all 72 captured bodies**.
///
/// The comparison is byte-exact and does not fold case, and it should stay that way: folding
/// would be defending against something that has been shown not to occur, at the cost of
/// shrinking the credential's effective space.
///
/// # What 26 buys
///
/// 16 bytes is 128 bits; base32 packs 5 bits per character, so 128/5 rounds up to 26. Hex
/// would need 32 for the same entropy. Both fit with enormous margin
/// ([`CLIENT_TOKEN_BYTES`]), so this is compactness for its own sake, not a fit constraint.
pub const CLIENT_TOKEN_CHARS: usize = 26;

/// RFC 4648 base32, uppercase, no padding.
const BASE32_ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

fn base32_encode(raw: &[u8]) -> String {
    let mut out = String::with_capacity(raw.len() * 8 / 5 + 1);
    let mut acc: u32 = 0;
    let mut bits: u32 = 0;
    for &b in raw {
        acc = (acc << 8) | u32::from(b);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            out.push(BASE32_ALPHABET[((acc >> bits) & 0x1f) as usize] as char);
        }
    }
    if bits > 0 {
        out.push(BASE32_ALPHABET[((acc << (5 - bits)) & 0x1f) as usize] as char);
    }
    out
}

/// Mint a client token: the plain value, and the SHA-256 that is the only thing stored.
///
/// Same hashing rule as every other secret in this crate - the standing constraint, and
/// [`hash_token`] is reused rather than reimplemented so there is one function to be wrong.
fn new_client_token() -> (String, String) {
    let mut raw = [0u8; CLIENT_TOKEN_BYTES];
    rand::rngs::OsRng.fill_bytes(&mut raw);
    let token = base32_encode(&raw);
    debug_assert_eq!(token.len(), CLIENT_TOKEN_CHARS);
    let hash = hash_token(&token);
    (token, hash)
}

/// Normalise an identity string that came off the wire before it is hashed or compared.
///
/// The client's string is length-prefixed, so it can legitimately carry a trailing NUL that
/// the sender never meant as data, and a hook that writes a fixed-size buffer will produce
/// exactly that. Trimming ASCII whitespace and NUL is the difference between "the credential
/// works" and "the credential never matches and nobody can see why" - and it costs nothing,
/// because [`CLIENT_TOKEN_CHARS`] of base32 contains neither.
///
/// **An empty result is not a credential.** Every `0x0073` this project has ever captured -
/// 72 of them - carries a zero-length identity, so "empty" is what a stock client sends and it
/// must keep buying the weaker rules. Only a NON-EMPTY identity is a presentation.
pub fn normalise_client_token(raw: &str) -> Option<String> {
    let trimmed = raw.trim_matches(|c: char| c.is_ascii_whitespace() || c == '\0');
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

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
    /// **The one-time credential the CLIENT is meant to carry**, [`CLIENT_TOKEN_CHARS`]
    /// characters of base32.
    ///
    /// Unlike [`Self::launch_id`], this one is not a handle for an API call - it is the thing
    /// the launcher writes where the hook can read it, so the hook can put it in the client's
    /// own session object and the client sends it in `0x0073`. That makes it the first value
    /// in this project that identifies a launch **on the wire** rather than by inference.
    ///
    /// **Valid for as long as its claim is**, not for a single presentation. It used to be
    /// one-time, and that was the bug the owner reported on 2026-09-08: the launcher hands the same
    /// token to every `Start Game`, and the second one is a different process. See
    /// [`Store::present_client_token`], which also states what the change costs.
    ///
    /// It authenticates the **login socket only**. `0x0073` has never appeared on a channel
    /// connection - 103 archived files, every one port 8484, including four whose names say
    /// "world" - so `CLAUDE.md`'s standing constraint *"the game socket carries no
    /// credentials"* is untouched by this. Say so when reporting progress.
    pub client_token: String,
    /// How many of **this account's** earlier live claims this sign-in replaced.
    ///
    /// Normally 0, and 1 when somebody signs in again without the previous launch having
    /// expired. Carried out rather than swallowed for the reason `CLAUDE.md` gives about
    /// guards whose answers nobody reads: a launch that silently invalidated a credential the
    /// player is still holding should say so in the log where they can see it.
    pub superseded: usize,
}

/// Redacted, because a `{:?}` of a struct is how a secret reaches a log without anybody
/// deciding to put it there.
impl std::fmt::Debug for StakedClaim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StakedClaim")
            .field("claim", &self.claim)
            .field("launch_id", &"<redacted>")
            .field("client_token", &"<redacted>")
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
    /// **The client token the connection carried in `0x0073`**, already normalised by
    /// [`normalise_client_token`] - so `Some("")` is not a state this can be in.
    ///
    /// This is the field that turns the ladder below from inference into a claim the client
    /// makes and the server checks. It is `None` for every client that has ever connected to
    /// this server: 72 of 72 captured `0x0073` bodies carry a zero-length identity, because
    /// its setter `FUN_142c503c0` has no callers. The hook is what fills it.
    pub client_token: Option<String>,
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

    /// Evidence carrying a client token, normalised on the way in.
    ///
    /// Normalising here rather than at the call site is deliberate: an empty or
    /// whitespace-only identity must land as `None`, because `None` reaches the weaker rules
    /// and `Some` is a *presentation* that refuses if it does not match. Getting that
    /// backwards would refuse every stock client on the first packet it sends.
    pub fn with_client_token(token: impl AsRef<str>) -> Self {
        Self { client_token: normalise_client_token(token.as_ref()), ..Self::default() }
    }

    pub fn and_client_token(mut self, token: Option<&str>) -> Self {
        self.client_token = token.and_then(normalise_client_token);
        self
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
        self.token.is_none()
            && self.client_token.is_none()
            && self.launch_pid.is_none()
            && self.peer.is_none()
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
    /// **The client itself carried a one-time client token in `0x0073` and it matched.**
    ///
    /// The only rule in this table that is not an inference about a credential-less socket.
    /// It still says nothing about *who is at the keyboard* - a token in a file beside the
    /// client is a launch credential, not a person - and it covers the **login** socket only.
    ClientToken,
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
            ResolvedBy::ClientToken => {
                "the one-time client token the CLIENT carried in 0x0073 - a credential on the \
                 wire, not an inference about the socket (login socket only)"
            }
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
    // The client token, added 2026-08-29. Three columns, all nullable, all arriving NULL on
    // The owner's existing maplecw.db - which is the honest state for a claim staked before the
    // credential existed: it has no client token, so no client token can ever match it, and it
    // stays resolvable by the weaker rules exactly as it was.
    if !existing.contains("client_token_hash") {
        conn.execute("ALTER TABLE login_claims ADD COLUMN client_token_hash TEXT", [])?;
    }
    // When it was spent. NULL means unspent. This is what makes it one-time.
    if !existing.contains("client_token_used_at") {
        conn.execute("ALTER TABLE login_claims ADD COLUMN client_token_used_at INTEGER", [])?;
    }
    // WHICH PROCESS spent it, as the operating system attributed the socket - never asserted
    // by the connection. See `judge_client_token` for the one thing this buys.
    if !existing.contains("client_token_used_pid") {
        conn.execute("ALTER TABLE login_claims ADD COLUMN client_token_used_pid INTEGER", [])?;
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
    // Unique for the same reason `token_hash` is: two claims sharing a client token would make
    // the credential ambiguous, and an ambiguous credential is not one. Every pre-existing row
    // holds NULL here and SQLite counts each NULL as distinct, so this cannot fail on upgrade.
    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_login_claims_client_token
             ON login_claims(client_token_hash)",
        [],
    )?;
    Ok(())
}

/// One live claim, as read from the database. Internal to the resolver.
struct LiveRow {
    /// The row's own id, so a write can name exactly the row a read judged.
    id: i64,
    claim: LoginClaim,
    token_hash: Option<String>,
    client_token_hash: Option<String>,
    client_token_used_at: Option<i64>,
    client_token_used_pid: Option<u32>,
    launch_pid: Option<u32>,
    peer: Option<String>,
}

/// **The one decision about a presented client token.**
///
/// One function, two callers - [`Store::resolve_login_claim`] reads it and
/// [`Store::present_client_token`] reads it and then writes. They cannot disagree, which is
/// the point: `CLAUDE.md` has a whole section on a guard that was asked and then ignored, and
/// two copies of a credential check is the same failure waiting to be written.
enum Judgement {
    /// Unspent and matching. Index into the live rows.
    Fresh(usize),
    /// Matching, already presented once, and **the operating system attributes this connection
    /// to the same process that presented it first**.
    ReplayBySameProcess(usize),
    /// Matching, already presented once, and this is a **different** process - or one this
    /// server cannot attribute.
    ///
    /// **Accepted since 2026-09-08.** It used to be a refusal, and the refusal was the bug
    /// The owner reported: see [`Store::present_client_token`]'s "the token is a session
    /// credential" section. The variant is kept rather than folded into
    /// [`Judgement::ReplayBySameProcess`] because the two are still different events and the
    /// log line has to say which one happened.
    ReplayByAnotherProcess(usize),
    /// It matches no live claim: never issued, or the claim expired, was cleared, or its
    /// account was disabled or deleted. **A refusal, and now the only one.**
    Unknown,
}

/// Judge a presented client token against the live rows.
///
/// `pid` is what the **server** derived from the accepted socket via the OS TCP table
/// (`crate::peerowner::owning_pid`), never anything the connection asserted. `None` means the
/// peer is not on this machine and cannot be attributed.
///
/// # `rows` is the whole of the expiry check, and that is now the whole of the gate
///
/// Every row here came from [`Store::live_rows`], whose `WHERE` is
/// `c.expires_at > now AND a.enabled = 1` over a `JOIN` to `accounts`. So a token whose claim
/// has expired, been cleared by [`Store::clear_login_claims`] or
/// [`Store::clear_login_claims_for`], or whose account has been disabled or deleted, cannot
/// match **any** row and lands in [`Judgement::Unknown`]. Since the pid no longer decides
/// acceptance, that query is the only thing standing between a leaked token and an account -
/// which is why `a_spent_token_is_refused_once_its_claim_has_expired` and its siblings assert
/// the refusal rather than only the acceptance.
fn judge_client_token(rows: &[LiveRow], presented: &str, pid: Option<u32>) -> Judgement {
    let want = hash_token(presented);
    let Some(index) = rows.iter().position(|r| r.client_token_hash.as_deref() == Some(&want))
    else {
        return Judgement::Unknown;
    };
    let row = &rows[index];
    if row.client_token_used_at.is_none() {
        return Judgement::Fresh(index);
    }
    match (row.client_token_used_pid, pid) {
        (Some(first_by), Some(now)) if first_by == now => Judgement::ReplayBySameProcess(index),
        _ => Judgement::ReplayByAnotherProcess(index),
    }
}

/// What [`Store::present_client_token`] did with the identity a connection carried.
///
/// A separate type from [`ClaimResolution`] on purpose, and not merely for tidiness: adding a
/// variant to `ClaimResolution` would break `crate::migration`'s exhaustive match on it, and a
/// credential mechanism is not worth reshaping a neighbouring module's control flow for. It
/// also lets the four outcomes carry the four *different* sentences a log needs - which is the
/// same reasoning that made `ClaimResolution` three variants instead of an `Option`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presentation {
    /// **The first time this token has been seen.** The first-use markers - the time and the
    /// OS-attributed process - were written by this call.
    First,
    /// Presented again, by the process the operating system attributed the first use to.
    SameProcess,
    /// Presented again, by a **different** process, or by a peer this server cannot attribute.
    ///
    /// **This is an acceptance since 2026-09-08**, and it is the whole of the owner's Change 1: a
    /// second `Start Game` is a second process, and refusing it was the reported bug. The
    /// first use is carried along so the log can say who got here first and when.
    AnotherProcess { first_used_at: i64, first_used_pid: Option<u32> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientTokenOutcome {
    /// **Accepted.** The claim is the account this connection should be served as.
    ///
    /// [`Presentation`] says which of the three ways it got here. All three are acceptances;
    /// the distinction is for the log and for the audit columns, not for the decision.
    Accepted { claim: Box<ResolvedClaim>, presentation: Presentation },
    /// It matches no live claim: never issued, **expired**, cleared, disabled, deleted, or
    /// from a previous sign-in. Refused - and this is now the ONLY refusal, which is why
    /// [`Store::live_rows`]'s predicate is the whole security boundary.
    Unknown,
    /// **Nothing was presented.** The identity field was empty or absent, which is what every
    /// client that has ever connected to this server sends and what every reconnect sends.
    ///
    /// **This is not a refusal.** The caller keeps whatever the weaker rules gave it.
    NotPresented,
}

impl ClientTokenOutcome {
    /// True when the caller must discard whatever the weaker rules decided.
    ///
    /// The anti-downgrade rule as a function, so no call site has to re-derive it - and so
    /// that adding an outcome later cannot silently default to "carry on".
    pub fn is_refusal(&self) -> bool {
        matches!(self, ClientTokenOutcome::Unknown)
    }

    /// The whole sentence to log. Written here for the same reason [`ClaimResolution::why`] is.
    pub fn why(&self) -> String {
        match self {
            ClientTokenOutcome::Accepted { claim, presentation: Presentation::First } => format!(
                "ACCEPTED - the client carried a valid token in 0x0073, for the FIRST time. \
                 Serving account {:?} (id {}). This connection is identified by a CREDENTIAL, \
                 not by inference. It authenticates the LOGIN socket only. The token stays \
                 valid for the rest of the claim's life; the first use is recorded for audit",
                claim.claim.account_name, claim.claim.account_id
            ),
            ClientTokenOutcome::Accepted { claim, presentation: Presentation::SameProcess } => {
                format!(
                    "ACCEPTED as a REPEAT BY THE SAME CLIENT PROCESS - the operating system \
                     attributes this socket to the process that first presented this token. \
                     Serving account {:?} (id {})",
                    claim.claim.account_name, claim.claim.account_id
                )
            }
            ClientTokenOutcome::Accepted {
                claim,
                presentation: Presentation::AnotherProcess { first_used_at, first_used_pid },
            } => format!(
                "ACCEPTED as a REPEAT BY A DIFFERENT PROCESS - the token was first presented at \
                 {first_used_at} by pid {}. Since 2026-09-08 the token is honoured for as long \
                 as its login claim is live (the owner: \"the server should honor that same token \
                 until its expiry\"), so a second Start Game is served rather than refused. \
                 EXPIRY IS NOW THE WHOLE GATE: anyone holding this token can be served as \
                 account {:?} (id {}) until the claim runs out. Serving it",
                first_used_pid.map(|p| p.to_string()).unwrap_or_else(|| "<unattributed>".into()),
                claim.claim.account_name,
                claim.claim.account_id
            ),
            ClientTokenOutcome::Unknown => "REFUSED - the client token matches no LIVE claim. \
                 The claim EXPIRED, or was cleared, or its account was disabled or deleted, or \
                 the token was never issued, or Login was pressed again since the launcher \
                 wrote it. Serving the FALLBACK account: a connection that presents a \
                 credential gets THAT claim or none, and never falls through to the weaker \
                 rules. Sign in again through maplecw-launcher"
                .to_string(),
            ClientTokenOutcome::NotPresented => "no client token was carried - the 0x0073 \
                 identity was empty, which is what every capture to date shows and what every \
                 reconnect sends. The account stands as the weaker rules decided it"
                .to_string(),
        }
    }
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
    /// **Stake a claim for one launch.** Does not touch anybody else's - but it does replace
    /// **this account's** own earlier live claims, one per account being the rule since
    /// 2026-09-13. See the module header, and the comment at the `DELETE` that enforces it for
    /// what a superseded launch loses. The count comes back in [`StakedClaim::superseded`].
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
        // A FRESH client token per stake, and the used markers reset with it. Same rule as the
        // launch id: one press of Login is one launch, and the previous launch's credential
        // stops working. Re-staking after a client has already spent its token is therefore
        // the supported way to re-arm one.
        let (client_token, client_token_hash) = new_client_token();
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

        // **This account's own earlier claims, and nobody else's.** The owner, 2026-09-13:
        // *"shouldn't they be allowed to sign in multiple times? Signing in again should
        // invalidate previous claims on that account."*
        //
        // It is scoped by `account_id`, which is the entire difference between this and the
        // eviction bug in the module header: `otter` signing in leaves `owl`'s claim exactly
        // where it was, and the regression test for that still passes. What it removes is the
        // second live row for ONE account, which was never useful and was actively harmful -
        // with two of them staked from one address, rule 3 stops picking one out and every
        // connection from that address falls to the fallback. Measured on the live server on
        // 2026-09-13: six launches from one address, the first three matched by address and
        // the rest refused with *"4 login claims are live and the evidence presented did not
        // pick one out"*. Nothing was wrong with the guard; there was simply no longer an
        // honest answer, and the reason there wasn't is that the same person had signed in
        // four times.
        //
        // `token_hash <> ?2` keeps the re-stake path intact: staking the same token again is
        // the same sign-in refreshing itself and is handled by the upsert below.
        //
        // **What this costs.** The superseded launch stops working, in every sense: its client
        // token no longer matches a live row, so a client still holding it is resolved as
        // `NoClaim` at its next login connection rather than served, and a migration bound to
        // that claim's hash cannot be re-derived. That is the meaning of "signing in again
        // invalidates the previous claim" and not a side effect of it - but it does mean a
        // second sign-in while a client of the same account is running will cut that client
        // off at its next Log Out or world change, which is rule 2's territory.
        let superseded = tx.execute(
            "DELETE FROM login_claims WHERE account_id = ?1 AND token_hash <> ?2",
            rusqlite::params![account_id, token_hash],
        )?;
        tx.execute(
            "INSERT INTO login_claims
                 (account_id, token_hash, created_at, expires_at, peer, launch_hash,
                  client_token_hash, client_token_used_at, client_token_used_pid)
                  VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, NULL)
             ON CONFLICT(token_hash) DO UPDATE SET
                 account_id  = excluded.account_id,
                 created_at  = excluded.created_at,
                 expires_at  = excluded.expires_at,
                 launch_hash = excluded.launch_hash,
                 -- The client token is REPLACED and its used markers CLEARED. A re-stake is a
                 -- new launch: the old credential must stop working, and the new one must not
                 -- arrive already spent - which it would if these two were left alone.
                 client_token_hash     = excluded.client_token_hash,
                 client_token_used_at  = NULL,
                 client_token_used_pid = NULL,
                 -- Keep an address that was recorded before if this stake has none, and keep
                 -- launch_pid untouched: the same token is the same sign-in, so the client
                 -- process it was bound to has not changed.
                 peer        = COALESCE(excluded.peer, login_claims.peer)",
            rusqlite::params![
                account_id,
                token_hash,
                now,
                expires_at,
                peer,
                launch.hash,
                client_token_hash
            ],
        )?;
        tx.commit()?;

        Ok(StakedClaim {
            claim: LoginClaim { account_id, account_name, created_at: now, expires_at },
            launch_id: launch.token,
            client_token,
            superseded,
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

        // Rule 1a: a session token. Nothing on the wire presents one.
        if let Some(token) = evidence.token.as_deref() {
            let want = hash_token(token);
            return Ok(match live.into_iter().find(|r| r.token_hash.as_deref() == Some(&want)) {
                Some(row) => ClaimResolution::Resolved(row.into_resolved(ResolvedBy::Token)),
                None => ClaimResolution::NoClaim,
            });
        }

        // Rule 1b: THE CLIENT TOKEN, carried in 0x0073 by the client's own code.
        //
        // `return` in every arm, deliberately: a presented credential gets its claim or none.
        // Falling through to rules 2-4 on a mismatch would make the credential worthless -
        // an attacker would get past it by presenting a wrong one, which is strictly easier
        // than presenting nothing. This mirrors `crate::migration`'s invariant.
        //
        // **This does not spend the token.** `resolve_login_claim` is a read - it is called
        // per connection and `live_login_claims` shares its query - and a read that consumes
        // is exactly the shape `reading_the_claim_twice_serves_the_same_account` exists to
        // forbid. `Store::present_client_token` is the one that writes.
        if let Some(presented) = evidence.client_token.as_deref() {
            return Ok(
                match judge_client_token(&live, presented, evidence.launch_pid) {
                    // All three matching arms resolve. The pid stopped deciding acceptance on
                    // 2026-09-08 - see `Judgement::ReplayByAnotherProcess` - so the only
                    // refusal left is a token that matches no LIVE row, and liveness is
                    // `live_rows`'s `expires_at > now AND a.enabled = 1`.
                    Judgement::Fresh(i)
                    | Judgement::ReplayBySameProcess(i)
                    | Judgement::ReplayByAnotherProcess(i) => {
                        let row = live.into_iter().nth(i).expect("index came from this slice");
                        ClaimResolution::Resolved(row.into_resolved(ResolvedBy::ClientToken))
                    }
                    Judgement::Unknown => ClaimResolution::NoClaim,
                },
            );
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
                    c.token_hash, c.launch_pid, c.peer, c.id,
                    c.client_token_hash, c.client_token_used_at, c.client_token_used_pid
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
                id: row.get(7)?,
                client_token_hash: row.get(8)?,
                client_token_used_at: row.get(9)?,
                client_token_used_pid: row.get::<_, Option<i64>>(10)?.map(|p| p as u32),
            })
        })?;
        Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
    }

    /// **Present the client token a connection carried in `0x0073`, and spend it.**
    ///
    /// The one entry point the login server calls. It resolves *and* consumes in one place,
    /// because the alternative - resolve here, consume there - is two guards where one of them
    /// eventually gets skipped, which is the failure `CLAUDE.md` records about the quest
    /// payout that sat outside the match on the store's answer.
    ///
    /// `pid` is what the operating system says owns the far end of this socket
    /// (`crate::peerowner::owning_pid_of`), not anything the connection asserted. Pass `None`
    /// for an off-box peer; the effect is that a replay from it cannot be honoured.
    ///
    /// # THE TOKEN IS A SESSION CREDENTIAL WITH THE CLAIM'S EXPIRY, AND IT USED NOT TO BE
    ///
    /// The owner, 2026-09-08, relaying a player: *"the launcher says the session is valid for 12
    /// hours on Login, but after the initial launch, if they try clicking Start Game again,
    /// the client will say the session is invalid."*
    ///
    /// That was this function. The launcher keeps the token its sign-in returned
    /// (`launcher::session::SignIn::Ok`) and hands the same one to every `Start Game`; the
    /// token was **spent on first presentation**, and a second `Start Game` is a second
    /// process, so it judged `ReplayByAnother` and was refused. The twelve hours the launcher
    /// promises is [`LOGIN_CLAIM_TTL_SECS`], the **claim's** lifetime, and the token's
    /// lifetime was one presentation.
    ///
    /// The owner: *"the launcher should keep that token, the server should honor that same token
    /// until its expiry, as long as that token is still valid."* So it does. Every match is an
    /// acceptance:
    ///
    /// | who presents it | answer |
    /// |---|---|
    /// | nobody has yet | [`Presentation::First`] - accepted, and the first use is recorded |
    /// | the same client process | [`Presentation::SameProcess`] - accepted |
    /// | a different process, or an unattributable peer | [`Presentation::AnotherProcess`] - **accepted** |
    /// | a token matching no live claim | [`ClientTokenOutcome::Unknown`] - refused |
    ///
    /// # What this costs, stated plainly rather than buried
    ///
    /// **The token used to be useless once spent; it is now good for the rest of the claim's
    /// life - up to twelve hours.** It sits in plain text in a marker file beside the client
    /// until the hook deletes it (`launcher::client::write_identity_marker`), so anyone who
    /// can read that file, or who captures the identity string off the wire, can be served as
    /// that account on the LOGIN socket for the remainder of the claim. Before this change the
    /// same theft bought one presentation and only if it beat the real client to it.
    ///
    /// What has NOT changed: it is still login-socket only (`0x0073` has never appeared on a
    /// channel connection - 103 archived files), it still cannot be used to authenticate
    /// anything else, and it still dies with the claim. [`Store::clear_login_claims_for`] -
    /// which `Store::set_password` already calls - revokes it immediately, and that is now the
    /// meaningful kill switch rather than a formality.
    ///
    /// # The audit columns are still written, and they no longer refuse
    ///
    /// First use - time, and the OS-attributed process - is recorded exactly as before. It has
    /// simply stopped being a guard. `CLAUDE.md`: *"A refusal that is reported to no one will
    /// be ignored eventually"*; the mirror of that is that a **record** which no longer
    /// refuses must not go on describing itself as one, which is why
    /// [`ClientTokenOutcome::AlreadyUsed`] was removed rather than left unreachable.
    ///
    /// The pid still comes from the kernel's TCP table keyed by the socket this server
    /// accepted, so a process cannot claim to be another one - it is just no longer the thing
    /// that decides.
    pub fn present_client_token(
        &self,
        presented: &str,
        pid: Option<u32>,
    ) -> Result<ClientTokenOutcome> {
        let Some(presented) = normalise_client_token(presented) else {
            return Ok(ClientTokenOutcome::NotPresented);
        };
        let live = self.live_rows()?;
        Ok(match judge_client_token(&live, &presented, pid) {
            Judgement::Unknown => ClientTokenOutcome::Unknown,
            Judgement::ReplayBySameProcess(index) => {
                let row = live.into_iter().nth(index).expect("index came from this slice");
                ClientTokenOutcome::Accepted {
                    claim: Box::new(row.into_resolved(ResolvedBy::ClientToken)),
                    presentation: Presentation::SameProcess,
                }
            }
            Judgement::ReplayByAnotherProcess(index) => {
                let row = live.into_iter().nth(index).expect("index came from this slice");
                let presentation = Presentation::AnotherProcess {
                    first_used_at: row.client_token_used_at.unwrap_or_default(),
                    first_used_pid: row.client_token_used_pid,
                };
                ClientTokenOutcome::Accepted {
                    claim: Box::new(row.into_resolved(ResolvedBy::ClientToken)),
                    presentation,
                }
            }
            Judgement::Fresh(index) => {
                let row = live.into_iter().nth(index).expect("index came from this slice");
                let id = row.id;
                // **Record** the first use. The predicate still repeats
                // `client_token_used_at IS NULL`, so two connections racing a token nobody has
                // presented cannot both be told they were first - but the loser is no longer
                // refused, it is simply a repeat. The write is an audit trail now, not a
                // consumption, and the `WHERE` is what keeps the recorded pid the pid of the
                // connection that actually got there first.
                let now = Store::now();
                let conn = self.conn();
                let recorded = conn.execute(
                    "UPDATE login_claims
                        SET client_token_used_at = ?2, client_token_used_pid = ?3
                      WHERE id = ?1 AND client_token_used_at IS NULL",
                    rusqlite::params![id, now, pid],
                )?;
                drop(conn);
                let claim = Box::new(row.into_resolved(ResolvedBy::ClientToken));
                if recorded == 1 {
                    ClientTokenOutcome::Accepted { claim, presentation: Presentation::First }
                } else {
                    // The other connection won the race between the read and the write. Both
                    // are accepted either way; re-judging is what makes the LOG say which of
                    // the two this was, and re-reading the row is what makes the recorded
                    // first-use figures in it belong to the connection that really was first.
                    match self.live_rows()?.into_iter().find(|r| r.id == id) {
                        Some(row) => {
                            let presentation = if row.client_token_used_pid == pid && pid.is_some()
                            {
                                Presentation::SameProcess
                            } else {
                                Presentation::AnotherProcess {
                                    first_used_at: row.client_token_used_at.unwrap_or_default(),
                                    first_used_pid: row.client_token_used_pid,
                                }
                            };
                            ClientTokenOutcome::Accepted {
                                claim: Box::new(row.into_resolved(ResolvedBy::ClientToken)),
                                presentation,
                            }
                        }
                        // The claim stopped being live between the two reads - it expired, or
                        // was cleared. Expiry is the gate, so this is a refusal.
                        None => ClientTokenOutcome::Unknown,
                    }
                }
            }
        })
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

    /// **Signing in again replaces your own claim, and only your own.**
    ///
    /// The owner, 2026-09-13: *"shouldn't they be allowed to sign in multiple times? Signing in
    /// again should invalidate previous claims on that account."* The other account's row in
    /// this test is the guard on the fix: it is the eviction bug in the module header, and it
    /// must survive a re-sign-in by somebody else.
    #[test]
    fn signing_in_again_replaces_your_own_claim_and_nobody_elses() {
        let (store, wisp, other) = store_with_accounts();
        let other_token = login(&store, "wisp_alt");
        store.stake_login_claim(other, &other_token, LOGIN_CLAIM_TTL_SECS).unwrap();

        let first = login(&store, "wisp");
        let staked = store.stake_login_claim_with(wisp, &first, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        assert_eq!(staked.superseded, 0, "there was nothing of wisp's to replace");

        let second = login(&store, "wisp");
        let staked = store.stake_login_claim_with(wisp, &second, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        assert_eq!(staked.superseded, 1, "the first sign-in's claim");

        assert_eq!(claim_rows(&store), 2, "one for wisp, one for wisp_alt");
        let live = store.live_login_claims().unwrap();
        assert_eq!(live.iter().filter(|c| c.account_id == wisp).count(), 1);
        assert_eq!(
            live.iter().filter(|c| c.account_id == other).count(),
            1,
            "somebody else's claim was evicted - this is the bug the module exists to fix"
        );

        // The replaced launch is gone in the sense that matters: its token no longer resolves.
        let r = store.resolve_login_claim(&ClaimEvidence::with_token(first)).unwrap();
        assert_eq!(r.resolved(), None, "the superseded token still works: {}", r.why());
        let r = store.resolve_login_claim(&ClaimEvidence::with_token(second)).unwrap();
        assert_eq!(r.resolved().expect("the newest sign-in").claim.account_id, wisp);
    }

    /// **The failure this was reported from**, in the shape the live server logged it.
    ///
    /// 2026-09-13: one person signed in repeatedly from one address, and from the fourth
    /// connection on the login server refused every one of them - *"4 login claims are live
    /// and the evidence presented did not pick one out"*. Rule 3 needs the address to name
    /// exactly one claim, and their own retries were the other ones.
    #[test]
    fn repeated_sign_ins_from_one_address_stay_resolvable_by_address() {
        let (store, wisp, _other) = store_with_accounts();
        let peer = "108.27.246.100";

        let mut last = String::new();
        for _ in 0..4 {
            last = login(&store, "wisp");
            store.stake_login_claim_with(wisp, &last, LOGIN_CLAIM_TTL_SECS, Some(peer)).unwrap();
        }

        let r = store.resolve_login_claim(&ClaimEvidence::default().from_peer(peer)).unwrap();
        let resolved = r.resolved().expect("four sign-ins by one account is still one claim");
        assert_eq!(resolved.how, ResolvedBy::PeerAddress, "{}", r.why());
        assert_eq!(resolved.claim.account_id, wisp);
        assert_eq!(resolved.token_hash.as_deref(), Some(hash_token(&last).as_str()));
    }

    /// Two accounts behind one address are still ambiguous, and must stay that way.
    ///
    /// The fix above is scoped by account precisely so it cannot collapse this case: two
    /// different people on one NAT have two honest answers and the address picks neither.
    #[test]
    fn two_accounts_behind_one_address_are_still_refused_rather_than_guessed() {
        let (store, wisp, other) = store_with_accounts();
        let peer = "108.27.246.100";
        let a = login(&store, "wisp");
        let b = login(&store, "wisp_alt");
        store.stake_login_claim_with(wisp, &a, LOGIN_CLAIM_TTL_SECS, Some(peer)).unwrap();
        store.stake_login_claim_with(other, &b, LOGIN_CLAIM_TTL_SECS, Some(peer)).unwrap();

        let r = store.resolve_login_claim(&ClaimEvidence::default().from_peer(peer)).unwrap();
        assert!(matches!(r, ClaimResolution::Ambiguous { live: 2, .. }), "{}", r.why());
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
        let b = login(&store, "wisp_alt");
        store.stake_login_claim(wisp, &a, LOGIN_CLAIM_TTL_SECS).unwrap();
        store.stake_login_claim(other, &b, LOGIN_CLAIM_TTL_SECS).unwrap();
        // A second row for `wisp`, written directly rather than staked. Staking cannot produce
        // one any more - one live claim per account since 2026-09-13 - but a database written
        // before that change can hold several, and the live server's did: four live claims,
        // more than one of them the same person. Clearing by account must still take them all.
        store
            .conn()
            .execute(
                "INSERT INTO login_claims (account_id, token_hash, created_at, expires_at)
                      VALUES (?1, 'a-hash-from-an-older-build', ?2, ?3)",
                rusqlite::params![wisp, Store::now(), Store::now() + LOGIN_CLAIM_TTL_SECS],
            )
            .unwrap();
        assert_eq!(claim_rows(&store), 3);

        assert_eq!(store.clear_login_claims_for(wisp).unwrap(), 2, "both of wisp's rows");
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

    // ------------------------------------------------------------------------------------
    // THE CLIENT TOKEN. Rule 1b - the first credential in this project that travels on the
    // wire rather than being inferred from a socket.
    // ------------------------------------------------------------------------------------

    /// **The encoder, against RFC 4648's own vectors.**
    ///
    /// `CLAUDE.md`: *"A constant that came from reading a header is a claim, not a fact."* The
    /// same goes for an alphabet typed out of a spec. These are the published test vectors,
    /// with the `=` padding dropped because this encoder emits none - so the assertion is
    /// against something that can disagree, not against the code's own behaviour.
    #[test]
    fn the_base32_encoder_matches_the_rfc4648_vectors() {
        for (input, want) in [
            ("", ""),
            ("f", "MY"),
            ("fo", "MZXQ"),
            ("foo", "MZXW6"),
            ("foob", "MZXW6YQ"),
            ("fooba", "MZXW6YTB"),
            ("foobar", "MZXW6YTBOI"),
        ] {
            assert_eq!(base32_encode(input.as_bytes()), want, "input {input:?}");
        }
    }

    /// The length constant is the thing most likely to sink this - the client-side cap on the
    /// `0x0073` identity string is unmeasured. Pin what the server will emit, in characters,
    /// and pin the alphabet: uppercase `A-Z2-7` only, so an upper-casing transform anywhere on
    /// the path cannot break the comparison.
    #[test]
    fn a_client_token_is_26_uppercase_base32_characters() {
        for _ in 0..64 {
            let (token, hash) = new_client_token();
            assert_eq!(token.len(), CLIENT_TOKEN_CHARS, "{token:?}");
            assert_eq!(token.len(), 26, "the constant itself, spelled out");
            assert!(
                token.bytes().all(|b| b.is_ascii_uppercase() || (b'2'..=b'7').contains(&b)),
                "a character outside A-Z2-7 got out: {token:?}"
            );
            assert_eq!(token.to_uppercase(), token, "upper-casing must be a no-op");
            assert_eq!(hash, hash_token(&token));
            assert_eq!(normalise_client_token(&token).as_deref(), Some(token.as_str()));
        }
    }

    /// **The cross-crate constraint, asserted rather than described.**
    ///
    /// The hook (`grap_stub::identity::validate`) refuses any identity byte outside
    /// `0x21..=0x7e`, and it refuses for a real reason: the client re-derives the identity's
    /// length with a `strlen` loop, so an embedded NUL truncates the credential silently and a
    /// leading one sends nothing - both indistinguishable on the wire from the hook never
    /// having run.
    ///
    /// The two crates cannot share the constant (`store` does not depend on `grap-stub`, and
    /// should not), so the coupling is a **test on the mint** instead of a comment claiming it
    /// holds. If anybody widens the alphabet, this fails here rather than as an empty identity
    /// in a client run that costs the owner a manual launch.
    #[test]
    fn a_client_token_is_within_the_byte_set_the_client_can_carry() {
        for _ in 0..256 {
            let (token, _) = new_client_token();
            assert!(!token.is_empty(), "a leading NUL / empty identity sends nothing at all");
            for (at, byte) in token.bytes().enumerate() {
                assert!(
                    (0x21..=0x7e).contains(&byte),
                    "byte {at} of {token:?} is {byte:#04x}; grap_stub::identity::validate \
                     refuses anything outside 0x21..=0x7e"
                );
                assert_ne!(byte, 0, "a NUL truncates the credential with no error anywhere");
                assert!(!byte.is_ascii_whitespace(), "normalise_client_token would trim it");
            }
            // And the server's own normaliser is a no-op on it, so what the hook writes and
            // what the server hashes are the same string.
            assert_eq!(normalise_client_token(&token).as_deref(), Some(token.as_str()));
        }
    }

    /// Two mints are two different tokens. A blank or constant token would make every
    /// assertion in this block pass while the credential proved nothing.
    #[test]
    fn two_stakes_mint_two_different_client_tokens() {
        let (store, wisp, other) = store_with_accounts();
        let a = login(&store, "wisp");
        let b = login(&store, "wisp_alt");
        let first = store.stake_login_claim_with(wisp, &a, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        let second = store.stake_login_claim_with(other, &b, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        assert_ne!(first.client_token, second.client_token);
        assert_ne!(first.client_token, first.launch_id, "and it is not the launch handle");
    }

    /// **The raw client token is never in the database**, and what IS there is its SHA-256 -
    /// the standing constraint, asserted rather than trusted. The whole row is dumped as text
    /// so a second column holding the plain value could not pass.
    #[test]
    fn the_client_token_is_stored_only_as_a_hash() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        let staked = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        assert!(!staked.client_token.is_empty(), "a blank token makes this vacuous");

        let stored: String = store
            .conn()
            .query_row("SELECT client_token_hash FROM login_claims", [], |r| r.get(0))
            .unwrap();
        assert_ne!(stored, staked.client_token);
        assert_eq!(stored, hash_token(&staked.client_token));

        let dump: String = store
            .conn()
            .query_row(
                "SELECT COALESCE(account_id,'')||'|'||COALESCE(token_hash,'')||'|'||
                        COALESCE(created_at,'')||'|'||COALESCE(expires_at,'')||'|'||
                        COALESCE(launch_pid,'')||'|'||COALESCE(peer,'')||'|'||
                        COALESCE(launch_hash,'')||'|'||COALESCE(client_token_hash,'')||'|'||
                        COALESCE(client_token_used_at,'')||'|'||COALESCE(client_token_used_pid,'')
                   FROM login_claims",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(!dump.contains(&staked.client_token), "the token is somewhere in the row: {dump}");
        // And `{:?}` - the way a secret usually reaches a log - is redacted.
        let debug = format!("{staked:?}");
        assert!(!debug.contains(&staked.client_token), "it leaked into Debug: {debug}");
    }

    /// **The headline.** Two launches on one address, each carrying its own client token in
    /// `0x0073`, each served its own account - by a credential rather than by inference.
    #[test]
    fn two_client_tokens_resolve_two_launches_on_one_address() {
        let (store, wisp, other) = store_with_accounts();
        let a = login(&store, "wisp");
        let b = login(&store, "wisp_alt");
        let first =
            store.stake_login_claim_with(wisp, &a, LOGIN_CLAIM_TTL_SECS, Some("127.0.0.1")).unwrap();
        let second = store
            .stake_login_claim_with(other, &b, LOGIN_CLAIM_TTL_SECS, Some("127.0.0.1"))
            .unwrap();

        for (token, want) in [(&first.client_token, wisp), (&second.client_token, other)] {
            let r = store
                .resolve_login_claim(
                    &ClaimEvidence::with_client_token(token).from_peer("127.0.0.1"),
                )
                .unwrap();
            let resolved = r.resolved().unwrap_or_else(|| panic!("{}", r.why()));
            assert_eq!(resolved.claim.account_id, want, "{}", r.why());
            assert_eq!(resolved.how, ResolvedBy::ClientToken);
        }
    }

    /// **The anti-downgrade rule.** A presented-and-wrong client token gets nothing - it must
    /// not fall through to "there was only one claim", or an attacker would get past the
    /// credential by supplying a wrong one, which is easier than supplying none.
    ///
    /// The control is the point: presenting nothing DOES reach the sole claim on the same
    /// store, so the refusal below is about the token and not about an empty table.
    #[test]
    fn a_wrong_client_token_refuses_rather_than_falling_through_to_the_sole_claim() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();

        assert_eq!(
            store
                .resolve_login_claim(&ClaimEvidence::none())
                .unwrap()
                .resolved()
                .expect("THE CONTROL: silence still reaches the sole claim")
                .claim
                .account_id,
            wisp
        );
        assert_eq!(
            store
                .resolve_login_claim(&ClaimEvidence::with_client_token("AAAAAAAAAAAAAAAAAAAAAAAAAA"))
                .unwrap(),
            ClaimResolution::NoClaim,
            "a wrong credential must buy strictly LESS than presenting nothing"
        );
        // And it cannot be laundered through the weaker rules by adding them to the evidence.
        assert_eq!(
            store
                .resolve_login_claim(
                    &ClaimEvidence::with_client_token("AAAAAAAAAAAAAAAAAAAAAAAAAA")
                        .from_peer("127.0.0.1")
                        .and_launch_pid(Some(4242))
                )
                .unwrap(),
            ClaimResolution::NoClaim
        );
    }

    /// **The compatibility hinge.** All 72 captured `0x0073` bodies carry a zero-length
    /// identity, because the setter has no callers. An empty identity must therefore be
    /// "presented nothing" and reach the weaker rules - if it counted as a presentation, every
    /// stock client would be refused on the first packet it sends and every player would drop
    /// to the fallback account.
    #[test]
    fn an_empty_identity_is_not_a_presentation() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();

        for empty in ["", "   ", "\0", "\0\0  \t"] {
            assert_eq!(normalise_client_token(empty), None, "{empty:?}");
            let ev = ClaimEvidence::with_client_token(empty);
            assert!(ev.client_token.is_none(), "{empty:?} must land as None");
            assert!(ev.is_empty(), "and must not count as evidence");
            let r = store.resolve_login_claim(&ev).unwrap();
            assert_eq!(
                r.resolved().unwrap_or_else(|| panic!("{empty:?}: {}", r.why())).how,
                ResolvedBy::SoleLiveClaim,
                "an empty identity must reach the weaker rules, not refuse"
            );
            assert_eq!(
                store.present_client_token(empty, Some(1)).unwrap(),
                ClientTokenOutcome::NotPresented
            );
        }
        // A token with a trailing NUL - what a fixed-size hook buffer produces - is the SAME
        // credential, not a different one.
        let staked = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        let padded = format!("{}\0\0", staked.client_token);
        assert_eq!(
            normalise_client_token(&padded).as_deref(),
            Some(staked.client_token.as_str())
        );
        assert_eq!(
            store
                .resolve_login_claim(&ClaimEvidence::with_client_token(&padded))
                .unwrap()
                .resolved()
                .expect("a NUL-padded token is the same token")
                .how,
            ResolvedBy::ClientToken
        );
    }

    /// **THE BUG WISP REPORTED, AS A TEST.** One sign-in, the same token, two different
    /// processes - both accepted - and then refused once the claim stops being live.
    ///
    /// This test used to be `presenting_the_token_spends_it_and_another_process_is_refused` and
    /// asserted the opposite. The behaviour it pinned is what a player hit: the launcher hands
    /// the token it got at sign-in to **every** `Start Game`, and the second `Start Game` is a
    /// second process, so it was `ReplayByAnother` and refused - while the launcher's own
    /// screen said the session was good for twelve hours.
    ///
    /// **Both halves matter and the second one is the load-bearing one.** A test that proved
    /// only the acceptance would pass just as happily against a version with no expiry check
    /// at all, and expiry is now the only gate there is.
    #[test]
    fn the_same_token_is_accepted_from_two_processes_and_refused_once_the_claim_is_gone() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        let staked = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();

        // The first Start Game.
        let first = store.present_client_token(&staked.client_token, Some(1111)).unwrap();
        match &first {
            ClientTokenOutcome::Accepted { claim, presentation } => {
                assert_eq!(claim.claim.account_id, wisp);
                assert_eq!(claim.how, ResolvedBy::ClientToken);
                assert_eq!(*presentation, Presentation::First);
            }
            other => panic!("the first presentation must be accepted: {other:?}"),
        }
        assert!(!first.is_refusal());

        // The first use is still RECORDED - it just no longer refuses. `CLAUDE.md`: an effect
        // that quietly stops happening is how an audit trail becomes fiction.
        let (used_at, used_pid): (Option<i64>, Option<i64>) = store
            .conn()
            .query_row(
                "SELECT client_token_used_at, client_token_used_pid FROM login_claims",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert!(used_at.is_some(), "first use was not recorded");
        assert_eq!(used_pid, Some(1111));

        // **The second Start Game.** A different pid, the same token, and it is served.
        let second = store.present_client_token(&staked.client_token, Some(2222)).unwrap();
        assert!(!second.is_refusal(), "this refusal is the reported bug: {second:?}");
        match &second {
            ClientTokenOutcome::Accepted {
                claim,
                presentation: Presentation::AnotherProcess { first_used_pid, .. },
            } => {
                assert_eq!(claim.claim.account_id, wisp);
                assert_eq!(*first_used_pid, Some(1111), "the audit trail names who was first");
            }
            other => panic!("a second process must now be served: {other:?}"),
        }
        assert!(second.why().contains("DIFFERENT PROCESS"), "{}", second.why());
        // The recorded first use is NOT overwritten by the second presentation.
        let used_pid: Option<i64> = store
            .conn()
            .query_row("SELECT client_token_used_pid FROM login_claims", [], |r| r.get(0))
            .unwrap();
        assert_eq!(used_pid, Some(1111), "a repeat must not rewrite who was first");

        // The read path agrees - the resolver and the presenter share `judge_client_token`, so
        // they cannot disagree, and this asserts that rather than trusting it.
        assert_eq!(
            store
                .resolve_login_claim(
                    &ClaimEvidence::with_client_token(&staked.client_token)
                        .and_launch_pid(Some(2222))
                )
                .unwrap()
                .resolved()
                .expect("the second process resolves too")
                .how,
            ResolvedBy::ClientToken
        );

        // The claim is untouched by any of it.
        assert_eq!(claim_rows(&store), 1);
        assert_eq!(store.current_login_claim().unwrap().unwrap().account_id, wisp);

        // ---- AND NOW THE REFUSAL, which is the half that proves the gate exists. ----
        assert_eq!(store.clear_login_claims().unwrap(), 1);
        for pid in [Some(1111), Some(2222), None] {
            assert_eq!(
                store.present_client_token(&staked.client_token, pid).unwrap(),
                ClientTokenOutcome::Unknown,
                "a cleared claim's token must be refused, from any process ({pid:?})"
            );
            assert_eq!(
                store
                    .resolve_login_claim(
                        &ClaimEvidence::with_client_token(&staked.client_token)
                            .and_launch_pid(pid)
                    )
                    .unwrap(),
                ClaimResolution::NoClaim,
                "and the read path must refuse it too ({pid:?})"
            );
        }
    }

    /// **The refusal, on the other three ways a claim stops being live.** Clearing is covered
    /// above; this is expiry, the scoped clear, and disabling the account.
    ///
    /// Every one of them goes through `live_rows`, and `live_rows` is now the whole of the
    /// boundary between a leaked token and an account. So it gets asserted per route rather
    /// than once, and each route gets a positive control immediately before it - otherwise a
    /// query that returned nothing at all would pass every line of this.
    #[test]
    fn a_token_is_refused_once_its_claim_stops_being_live_by_every_route() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");

        // 1. EXPIRY - the route the whole change now rests on.
        let dead = store.stake_login_claim_with(wisp, &token, -1, None).unwrap();
        assert_eq!(
            store.present_client_token(&dead.client_token, Some(1)).unwrap(),
            ClientTokenOutcome::Unknown,
            "an expired claim's token is not a credential"
        );

        // 2. clear_login_claims_for - what `set_password` calls, and now the kill switch.
        let scoped = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        assert!(
            !store.present_client_token(&scoped.client_token, Some(1)).unwrap().is_refusal(),
            "positive control: it works while the claim is live"
        );
        assert_eq!(store.clear_login_claims_for(wisp).unwrap(), 1);
        assert_eq!(
            store.present_client_token(&scoped.client_token, Some(1)).unwrap(),
            ClientTokenOutcome::Unknown
        );

        // 3. the account being disabled.
        let live = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        assert!(
            !store.present_client_token(&live.client_token, Some(1)).unwrap().is_refusal(),
            "positive control"
        );
        store.set_enabled("wisp", false).unwrap();
        assert_eq!(
            store.present_client_token(&live.client_token, Some(1)).unwrap(),
            ClientTokenOutcome::Unknown,
            "disabling an account must not be bypassable with a client token"
        );

        // 4. the account being deleted, through ON DELETE CASCADE and the join.
        store.set_enabled("wisp", true).unwrap();
        let live = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        assert!(!store.present_client_token(&live.client_token, Some(1)).unwrap().is_refusal());
        store.conn().execute("DELETE FROM accounts WHERE id = ?1", rusqlite::params![wisp]).unwrap();
        assert_eq!(
            store.present_client_token(&live.client_token, Some(1)).unwrap(),
            ClientTokenOutcome::Unknown
        );
    }

    /// **The reconnect insurance.** The client opens a second login connection in one launch
    /// ("Log Out", "Choose another world"). Measured over 57 archived runs it sends no
    /// `0x0073` on that connection - but nobody has ever seen a run with a NON-EMPTY identity,
    /// so if it ever does, the same client process must not be refused. Refusing it would read
    /// on screen as "my characters vanished when I logged out".
    #[test]
    fn the_same_client_process_may_present_a_spent_token_again() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        let staked = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();

        assert!(!store
            .present_client_token(&staked.client_token, Some(4242))
            .unwrap()
            .is_refusal());
        let again = store.present_client_token(&staked.client_token, Some(4242)).unwrap();
        match &again {
            ClientTokenOutcome::Accepted { claim, presentation } => {
                assert_eq!(claim.claim.account_id, wisp);
                assert_eq!(*presentation, Presentation::SameProcess);
            }
            other => panic!("the same process must not be refused: {other:?}"),
        }
        assert!(again.why().contains("SAME CLIENT PROCESS"), "{}", again.why());
    }

    /// An off-box peer, which this server cannot attribute to a process, is **also** served
    /// now. It used to be refused, and the refusal was deliberate: a spent token was one-time
    /// and an unattributable replay could not be shown to be the same client.
    ///
    /// That reasoning went with the one-time property. Expiry is the gate, the pid is an audit
    /// column, and refusing an off-box client would refuse everybody who is not on the owner's box -
    /// which is the deployment shape `docs/launcher.md` is aiming at.
    #[test]
    fn an_unattributable_peer_may_present_the_token_again() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        let staked = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();

        assert!(!store.present_client_token(&staked.client_token, None).unwrap().is_refusal());
        let again = store.present_client_token(&staked.client_token, None).unwrap();
        assert!(!again.is_refusal(), "an off-box client must not be locked out: {again:?}");
        assert!(
            matches!(
                again,
                ClientTokenOutcome::Accepted {
                    presentation: Presentation::AnotherProcess { first_used_pid: None, .. },
                    ..
                }
            ),
            "{again:?}"
        );
        // ...and expiry still refuses it, which is the only thing that does.
        store.clear_login_claims().unwrap();
        assert_eq!(
            store.present_client_token(&staked.client_token, None).unwrap(),
            ClientTokenOutcome::Unknown
        );
    }

    /// A token nobody issued is refused and says so differently from a spent one - the two
    /// have different fixes and collapsing them loses the only sentence worth logging.
    #[test]
    fn an_unissued_client_token_is_unknown_rather_than_spent() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();
        let out = store.present_client_token("ZZZZZZZZZZZZZZZZZZZZZZZZZZ", Some(1)).unwrap();
        assert_eq!(out, ClientTokenOutcome::Unknown);
        assert!(out.is_refusal());
        assert!(out.why().contains("matches no LIVE claim"), "{}", out.why());
    }

    /// Re-staking is a new launch: a fresh token, the old one dead, and the spent markers
    /// cleared so the new one does not arrive already used. That last clause is the whole
    /// reason the `ON CONFLICT` sets them to NULL explicitly.
    #[test]
    fn re_staking_mints_a_fresh_client_token_and_clears_the_spent_marker() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        let old = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        assert!(!store.present_client_token(&old.client_token, Some(7)).unwrap().is_refusal());

        let new = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        assert_ne!(old.client_token, new.client_token);
        assert_eq!(claim_rows(&store), 1, "a re-stake refreshes one row");

        // The old one is dead...
        assert_eq!(
            store.present_client_token(&old.client_token, Some(7)).unwrap(),
            ClientTokenOutcome::Unknown,
            "the previous launch's credential must stop working"
        );
        // ...and the new one is fresh, not inherited-as-spent.
        let out = store.present_client_token(&new.client_token, Some(7)).unwrap();
        assert!(
            matches!(
                out,
                ClientTokenOutcome::Accepted { presentation: Presentation::First, .. }
            ),
            "a re-stake must hand back a token whose first use has not been recorded: {out:?}"
        );
    }

    /// Expiry and disabling apply to the credential exactly as they do to every weaker rule.
    /// A credential that outlives the claim it belongs to would be a way past `set_enabled`.
    #[test]
    fn a_client_token_cannot_outlive_its_claim() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");

        let dead = store.stake_login_claim_with(wisp, &token, -60, None).unwrap();
        assert_eq!(
            store.present_client_token(&dead.client_token, Some(1)).unwrap(),
            ClientTokenOutcome::Unknown,
            "an expired claim's token is not a credential"
        );

        let live = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        // The control: it works while the claim is live, so the refusals here are about state
        // and not about the token being wrong.
        assert!(!store.present_client_token(&live.client_token, Some(1)).unwrap().is_refusal());

        let again = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        store.set_enabled("wisp", false).unwrap();
        assert_eq!(
            store.present_client_token(&again.client_token, Some(1)).unwrap(),
            ClientTokenOutcome::Unknown,
            "disabling an account must not be bypassable with a client token"
        );
        assert_eq!(
            store
                .resolve_login_claim(&ClaimEvidence::with_client_token(&again.client_token))
                .unwrap(),
            ClaimResolution::NoClaim
        );
    }

    /// **The owner's `maplecw.db`.** A `login_claims` written before these columns existed must gain
    /// them, keep its rows, and stay resolvable - and its rows must read as "no client token"
    /// rather than matching one. An `Err` on the read path would make the login server answer
    /// nothing, which freezes the client's whole UI.
    #[test]
    fn a_database_without_the_client_token_columns_is_upgraded_without_losing_rows() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        store.stake_login_claim(wisp, &token, LOGIN_CLAIM_TTL_SECS).unwrap();
        {
            let conn = store.conn();
            conn.execute("DROP INDEX IF EXISTS idx_login_claims_client_token", []).unwrap();
            for column in ["client_token_hash", "client_token_used_at", "client_token_used_pid"] {
                conn.execute(&format!("ALTER TABLE login_claims DROP COLUMN {column}"), [])
                    .unwrap();
            }
            let n: i64 =
                conn.query_row("SELECT COUNT(*) FROM login_claims", [], |r| r.get(0)).unwrap();
            assert_eq!(n, 1, "the row must survive the downgrade or this proves nothing");
        }

        // The read path ensures the columns itself.
        let live = store.current_login_claim().unwrap().expect("the old row is still served");
        assert_eq!(live.account_id, wisp);
        assert_eq!(claim_rows(&store), 1);
        // A pre-upgrade row has no client token, so none can match it - and presenting one is
        // still a refusal rather than a fall-through to the sole claim.
        assert_eq!(
            store.present_client_token("AAAAAAAAAAAAAAAAAAAAAAAAAA", Some(1)).unwrap(),
            ClientTokenOutcome::Unknown
        );
        assert_eq!(
            store
                .resolve_login_claim(&ClaimEvidence::with_client_token("AAAAAAAAAAAAAAAAAAAAAAAAAA"))
                .unwrap(),
            ClaimResolution::NoClaim
        );
        // And a fresh stake on the upgraded table mints a working one.
        let staked = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        assert!(!store.present_client_token(&staked.client_token, Some(9)).unwrap().is_refusal());
    }

    /// The credential outranks the weaker rules: with a client token presented, a pid bound to
    /// a DIFFERENT claim must not win. Otherwise rule 1b would be advisory.
    #[test]
    fn the_client_token_outranks_the_process_and_the_address() {
        let (store, wisp, other) = store_with_accounts();
        let a = login(&store, "wisp");
        let b = login(&store, "wisp_alt");
        let first =
            store.stake_login_claim_with(wisp, &a, LOGIN_CLAIM_TTL_SECS, Some("10.0.0.1")).unwrap();
        store.stake_login_claim_with(other, &b, LOGIN_CLAIM_TTL_SECS, Some("10.0.0.2")).unwrap();
        store.bind_launch_pid_by_token(&b, 5555).unwrap();

        // The control: that pid alone resolves to the OTHER account.
        assert_eq!(
            store
                .resolve_login_claim(&ClaimEvidence::with_launch_pid(5555))
                .unwrap()
                .resolved()
                .unwrap()
                .claim
                .account_id,
            other
        );
        // With wisp's client token presented, wisp wins - by credential, over both weaker rules.
        let r = store
            .resolve_login_claim(
                &ClaimEvidence::with_client_token(&first.client_token)
                    .from_peer("10.0.0.2")
                    .and_launch_pid(Some(5555)),
            )
            .unwrap();
        let resolved = r.resolved().unwrap_or_else(|| panic!("{}", r.why()));
        assert_eq!(resolved.claim.account_id, wisp, "{}", r.why());
        assert_eq!(resolved.how, ResolvedBy::ClientToken);
    }

    /// A resolved-by-client-token claim still carries the `token_hash` a migration binds to.
    /// Without this the credential would identify the account and then fail to protect the
    /// hand-off to the channel, which is the half that is still open.
    #[test]
    fn a_claim_resolved_by_client_token_still_carries_its_binding_hash() {
        let (store, wisp, _) = store_with_accounts();
        let token = login(&store, "wisp");
        let staked = store.stake_login_claim_with(wisp, &token, LOGIN_CLAIM_TTL_SECS, None).unwrap();
        let out = store.present_client_token(&staked.client_token, Some(3)).unwrap();
        let ClientTokenOutcome::Accepted { claim, .. } = out else { panic!("{out:?}") };
        assert_eq!(claim.token_hash.as_deref(), Some(hash_token(&token).as_str()));
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
