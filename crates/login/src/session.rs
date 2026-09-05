//! The protocol, as a pure state machine.
//!
//! Bodies in, bodies out. No socket, no cipher, no clock - so every exchange that has been
//! measured against the real client is a unit test here, and a change can be checked
//! without a client launch. That matters more than it sounds: a launch costs the owner a manual
//! run from an elevated shell, and the only oracle for most of this is what they see on
//! screen.
//!
//! # The rule that governs everything below: always answer
//!
//! **An unanswered request freezes the client's entire UI.** Not the screen it is on - all
//! of it, including the OK button on the quit prompt. That is what "Check" did before the
//! name check was answered and what "Choose another world" did before the leave-world
//! request was, and both times it read as a crash. So no path here returns an error to the
//! caller in place of a reply: a database failure becomes a *refusal* the client can
//! render, and the reason travels in the reply's log label instead.

use std::sync::Arc;

use net::opcode::{
    account_info, check_name_result, create_character_failed, create_character_result,
    data_wz_up_to_date, delete_character_result, enter_creation_permitted, login_result,
    migrate, migrate_refused, world_list_end, world_list_entry, Character,
    CreateCharacterRequest, SelectCharacterRequest, ACCOUNT_INFO, CHARACTER_SLOTS,
    CHECK_NAME_RESULT, CLIENT_CHECK_NAME_REQUEST, CLIENT_CREATE_CHARACTER_REQUEST,
    CLIENT_DATA_WZ_REQUEST, CLIENT_DELETE_CHARACTER_REQUEST, CLIENT_ENTER_CREATION_REQUEST,
    CLIENT_LEAVE_WORLD_REQUEST, CLIENT_LOGIN_REQUEST, CLIENT_SELECT_CHARACTER_REQUEST,
    CLIENT_SELECT_WORLD,
    CREATE_CANNOT_PROCESS, CREATE_CHARACTER_RESULT, CREATE_INSUFFICIENT_SLOT, DATA_WZ_PATCH,
    DELETE_CHARACTER_RESULT, DELETE_FAILED, DELETE_OK, ENTER_CREATION_RESULT,
    LOGIN_RESULT, MIGRATE_COMMAND, NAME_ALREADY_USED, NAME_AVAILABLE, NAME_NOT_ALLOWED,
    WORLD_LIST,
};
use store::{Account, NameCheck, Store};

use crate::config::{Config, World};

/// The channel index to advertise in `LOGIN_RESULT`, and the sentence explaining it.
///
/// **This is deliberately not always the true channel**, and the reason is the one thing
/// that kept the Change Channel dialog empty for a week.
///
/// The client builds its channel list once, at login, from the pair carried in
/// `LOGIN_RESULT` - it never asks for the list again, which is why opening the dialog sends
/// nothing at all. `FUN_141b2c7c0` bails out of the rebuild when the incoming
/// `(world, channel)` equals the pair the singleton already holds, and a client that has
/// loaded no list holds `(0, 0)`. So a world with id `0` telling the truth about channel `0`
/// is indistinguishable from saying nothing, and the dialog stays at one row.
///
/// [`net::channel::priming_channel`] is that rule. `None` means the world cannot populate
/// the dialog at all - with a single channel the only index that passes the client's range
/// check is `0`, which is the pair it already holds - and the honest value is sent instead.
///
/// **Sending the wrong index here costs nothing.** `SetField`'s `0x01A0` handler
/// `FUN_142097f80` reads a `u32` at body offset 8 and passes it to the same setter, and
/// that field is already the real channel in `opcode::set_field_head`. The only window in
/// which the value is wrong is the character-select screen, which displays no channel.
///
/// Full working, with the addresses: `research/channel-select.md` section 9.
fn advertised_channel(world: &World) -> (u32, String) {
    let count = u32::from(world.channel_count());
    match net::channel::priming_channel(world.id, world.channel_id, count) {
        Some(c) if c == world.channel_id => (c, format!("channel {c}")),
        Some(c) => (
            c,
            format!(
                "channel {c} advertised instead of the true {}, so the client rebuilds its channel list - research/channel-select.md section 9",
                world.channel_id
            ),
        ),
        None => (
            world.channel_id,
            format!(
                "channel {} (the true value: this world advertises {count} channel(s), so no index can make the client rebuild its list and the dialog will stay at one row)",
                world.channel_id
            ),
        ),
    }
}

/// One packet to send, plus what it is - the label is written to the log, and the log is
/// the instrument that gets read when the screen does something unexpected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reply {
    pub opcode: u16,
    pub body: Vec<u8>,
    pub what: String,
    /// **Wait this long before putting it on the wire.**
    ///
    /// Zero for every reply but one. See [`CHARACTER_LIST_PAUSE_MS`].
    pub pause_ms: u64,
}

/// **How long to wait before the character list, and why there is a wait at all.**
///
/// 2026-09-02. The character-select avatars are not drawn on the first visit of a client
/// process. Go into the world, log out, come back, and the same three characters draw
/// correctly. Two things were eliminated before this:
///
/// * **not the client patches** - `RETURN IMMEDIATELY`, `FORCE rdx=0x0` and
///   `mode=2,create=on` are byte-identical to archived runs of 2026-08-21;
/// * **not the bytes.** The two `0x0010` bodies, one that drew and one that did not, are
///   identical - same 1219-byte length, same prefix. Whatever differs is client state.
///
/// What this server does that no real one could is answer instantly:
///
/// ```text
///   03:24:11.730  <- 0x0080  CLIENT_LOGIN_REQUEST
///   03:24:11.739  -> 0x0000  ACCOUNT_INFO
///   03:24:11.739  -> 0x000B  WORLD_LIST
///   03:24:11.739  -> 0x000B  WORLD_LIST end
///   03:24:11.739  -> 0x0010  LOGIN_RESULT, 3 characters
/// ```
///
/// Four packets in the same millisecond, on loopback. In the service this client shipped
/// against those are **three round trips** with a player picking a world in between, and the
/// character list arrives tens to hundreds of milliseconds after the world list. Here the
/// client can receive all four in one read and build the character-select stage inside the
/// same dispatch that built the world list.
///
/// # This is an experiment, and it is labelled one
///
/// **[I]**, not [L]. Nothing has been read out of the client that says the avatar build is
/// asynchronous or that it depends on the world-list stage being finished. What is [L] is
/// that the bytes are identical and the timing is not, so the difference has to be timing or
/// something downstream of it.
///
/// It fails honestly: if the avatars are still missing with this in, the race is not here and
/// the next step is the client, not the server. 400 ms is comfortably more than a frame at
/// any refresh rate and far less than a player would notice.
pub const CHARACTER_LIST_PAUSE_MS: u64 = 400;

impl Reply {
    fn new(opcode: u16, body: Vec<u8>, what: impl Into<String>) -> Self {
        Reply { opcode, body, what: what.into(), pause_ms: 0 }
    }

    /// The same reply, sent after a wait. See [`CHARACTER_LIST_PAUSE_MS`].
    fn after(mut self, pause_ms: u64) -> Self {
        self.pause_ms = pause_ms;
        self
    }

    /// Opcode then body - the packet as the framer wants it.
    pub fn packet(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(2 + self.body.len());
        out.extend_from_slice(&self.opcode.to_le_bytes());
        out.extend_from_slice(&self.body);
        out
    }
}

/// One client connection's worth of state.
pub struct Session {
    store: Arc<Store>,
    config: Arc<Config>,
    /// Whose characters this connection sees.
    ///
    /// Resolved from configuration at startup, **not** from anything the client sent -
    /// the game socket carries no credentials. See the crate docs.
    account: Account,
    /// Has the client asked to log in yet? Only used to decide whether the startup gate
    /// needs repeating - see [`Session::on_quiet`].
    seen_login_request: bool,
    /// The source address this connection arrived from, if the socket could report one.
    ///
    /// Recorded on the migration for the audit trail and for `store::PeerPolicy::Require`.
    /// **It is not a discriminator and must not be described as one** - the owner, 2026-08-29:
    /// *"the launcher needs to be able to potentially handle multiple connections from the
    /// same IP as well, IP cannot be the sole discriminator."* Two clients on one machine
    /// both present `127.0.0.1`.
    peer: Option<String>,
    /// `peer` as an address, when it parses as one. Read by `net::advertise` to decide which
    /// host goes into the migration packet.
    peer_ip: Option<std::net::IpAddr>,
    /// The server's own end of the accepted socket. On a `0.0.0.0` bind this is the specific
    /// interface the client reached, which is exactly the host a directly-connected client
    /// should be told to dial for its channel. `None` in every test-built session, where the
    /// listed host is used and nothing changes.
    local_ip: Option<std::net::IpAddr>,
    /// The SHA-256 of the session token behind **the login claim this connection resolved
    /// to**, if it resolved to one.
    ///
    /// This replaces a call to `Store::live_claim_token_hash`, and the replacement is the
    /// point. That reader takes the *newest* live claim: with two people signed in it binds
    /// one player's migration to the other player's token, so with `--bind-migrations` on the
    /// rightful player's channel connection is refused (`TokenMismatch`) and the other
    /// player's is not. That is the same singleton mistake `store::claims` was rewritten to
    /// remove, one layer down.
    ///
    /// `None` means this connection did not resolve to a claim - nobody signed in, or two
    /// people did and it could not be told apart - and the migration is minted **unbound**,
    /// which is the pre-existing behaviour and is logged as such.
    claim_token_hash: Option<String>,
    /// **The account to fall back to if the client presents a credential and it is refused.**
    ///
    /// The `--account` account, the same one [`crate::server::resolve_account`] falls back to.
    /// It has to be carried here because the refusal happens *mid-connection*: the account was
    /// already chosen at accept time by the weaker rules, and a refused credential has to be
    /// able to take it away again. Without this, presenting a wrong token would leave the
    /// connection exactly where presenting nothing leaves it, and the credential would be
    /// advisory - see the anti-downgrade rule in `store::claims`.
    ///
    /// `None` in the unit tests below, which never present a credential.
    fallback: Option<Account>,
    /// The process the operating system attributes this socket to, as
    /// `store::peerowner::owning_pid_of` reported it at accept time. Never asserted by the
    /// client. Used only to let the **same** client process re-present a spent token.
    launch_pid: Option<u32>,
    /// Lines for the server's log that are not attached to a reply.
    ///
    /// `Reply::what` is the only thing that reaches the log today, so a packet that produces
    /// no reply produces no line - and `0x0073` deliberately produces no reply. Drained by
    /// [`Session::take_notes`] after every `handle`. A `Vec` rather than a callback so this
    /// module stays pure and every sentence below is unit-testable without a socket.
    notes: Vec<String>,
}

impl Session {
    pub fn new(store: Arc<Store>, config: Arc<Config>, account: Account) -> Self {
        Session {
            store,
            config,
            account,
            seen_login_request: false,
            peer: None,
            peer_ip: None,
            local_ip: None,
            claim_token_hash: None,
            fallback: None,
            launch_pid: None,
            notes: Vec::new(),
        }
    }

    /// Record the address this connection came from.
    ///
    /// A builder rather than a fourth parameter to [`Session::new`], so the thirteen tests
    /// below that do not care about addressing keep compiling. A session with no peer mints
    /// a migration with no address recorded, which is exactly what an in-process test is.
    pub fn with_peer(mut self, peer: impl Into<String>) -> Self {
        let peer = peer.into();
        self.peer_ip = peer.parse().ok();
        self.peer = Some(peer);
        self
    }

    /// Record the address this connection was ACCEPTED on - the server's end of the socket.
    ///
    /// Under `--advertise auto` this is what a LAN or VPN client is told to dial for its
    /// channel: the interface it already reached, with the channel's port. `net::advertise`.
    pub fn with_local_addr(mut self, local: std::net::SocketAddr) -> Self {
        self.local_ip = Some(local.ip());
        self
    }

    /// Record which login claim this connection was resolved to, by its token's SHA-256.
    ///
    /// A builder for the same reason `with_peer` is: the tests below that do not care about
    /// binding keep compiling, and a session that was never told is one that mints unbound
    /// migrations - the old behaviour, stated rather than defaulted into.
    pub fn with_claim_token_hash(mut self, hash: Option<String>) -> Self {
        self.claim_token_hash = hash;
        self
    }

    /// The `--account` account, so a **refused** client token can take this connection back
    /// down to it. See [`Session::fallback`].
    pub fn with_fallback(mut self, fallback: Account) -> Self {
        self.fallback = Some(fallback);
        self
    }

    /// The OS-attributed owning process of this socket, from the accept path.
    pub fn with_launch_pid(mut self, pid: Option<u32>) -> Self {
        self.launch_pid = pid;
        self
    }

    pub fn account_name(&self) -> &str {
        &self.account.name
    }

    /// Take the log lines produced since the last call. The server drains these after every
    /// `handle`; nothing here writes to a log itself.
    pub fn take_notes(&mut self) -> Vec<String> {
        std::mem::take(&mut self.notes)
    }

    fn note(&mut self, line: impl Into<String>) {
        self.notes.push(line.into());
    }

    /// The client has gone quiet without asking to log in. Send the startup gate again.
    ///
    /// The gate releases a `recv` loop on the client's **UI thread**, so a client that
    /// missed it shows a blank, non-responding window - indistinguishable on screen from
    /// a server that is not running. The Python harness sent the gate twice, once at
    /// connect and once after four seconds of quiet, and every successful run used that.
    /// Reproduced here rather than reasoned away: the packet is idempotent (a varint zero
    /// means "nothing to patch" however often it arrives), and the alternative costs a
    /// manual client launch to discover.
    pub fn on_quiet(&mut self) -> Vec<Reply> {
        if self.seen_login_request {
            return Vec::new();
        }
        vec![Reply::new(DATA_WZ_PATCH, data_wz_up_to_date(), "startup gate, repeated after quiet")]
    }

    /// What to send immediately after the greeting.
    ///
    /// The client hashes its `Data.wz` and blocks in `recv` on its **UI thread** until a
    /// handler sets `conn+0x150`, and only the patch handler does that. This client ships
    /// no `Data.wz` at all, so "nothing to patch" - a zigzag varint zero - is the correct
    /// and complete answer.
    ///
    /// Sent unprompted as well as in answer to the request, because that is what the
    /// harness did on every successful run: one at connect, one later. The request is
    /// documented as optional, and a startup gate that depends on an optional packet is
    /// not a gate worth taking a chance on.
    pub fn on_connect(&mut self) -> Vec<Reply> {
        vec![Reply::new(DATA_WZ_PATCH, data_wz_up_to_date(), "startup gate, unprompted")]
    }

    /// Handle one packet body from the client, opcode included.
    pub fn handle(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(opcode) = body.get(..2).map(|b| u16::from_le_bytes([b[0], b[1]])) else {
            return Vec::new();
        };
        let payload = &body[2..];

        match opcode {
            CLIENT_DATA_WZ_REQUEST => {
                vec![Reply::new(DATA_WZ_PATCH, data_wz_up_to_date(), "startup gate")]
            }
            // Both of these want the same four packets. The login request is the client
            // arriving; the leave-world request is it coming back from world select, and
            // it used to be answered once only - so picking a world a second time sat on
            // "Connecting..." forever.
            CLIENT_LOGIN_REQUEST => {
                self.seen_login_request = true;
                self.world_and_characters("login request")
            }
            // "Choose another world" and "Back" send the SAME empty 0x0082, so the server
            // cannot tell them apart - whatever makes Back go back one step is client-side.
            // What the server does control is which screen the reply lands on.
            //
            // The world-list TERMINATOR transitions the client to screen 2, WorldSelect:
            // `FUN_141b2fac0`'s end-of-list branch calls `FUN_141b3f050(stage, 2, 400)`, and
            // `docs/session.md` has the stage table read out of `FUN_141127730`, which
            // registers each id against a screen name - 2 is WorldSelect, 4 is CharSelect.
            //
            // So the terminator was already putting the client on WorldSelect and the
            // LOGIN RESULT that followed was yanking it straight to CharSelect. Leaving the
            // login result off is the whole change.
            CLIENT_LEAVE_WORLD_REQUEST => self.world_list_only("leave world"),

            // Picking a world on the WorldSelect screen. Unanswered this leaves the client
            // on "Connecting to server..." forever - it was the first thing to freeze once
            // that screen became reachable.
            //
            // The answer is the character list for the chosen world, which is exactly what a
            // login request already gets. The reference's handleSelectWorld ends its success
            // path in `selectWorldResult`, the same wire shape as our LOGIN_RESULT.
            //
            // We do NOT read the chosen world or channel out of the body yet - see
            // net::opcode::CLIENT_SELECT_WORLD. Every account here has one world, so the
            // answer is the same whichever row was clicked; that stops being true the moment
            // the channel swap needs the choice.
            CLIENT_SELECT_WORLD => self.world_and_characters("select world"),
            CLIENT_ENTER_CREATION_REQUEST => vec![Reply::new(
                ENTER_CREATION_RESULT,
                enter_creation_permitted(),
                "creation screen permitted",
            )],
            // **The identity the client carries.** Answered with nothing, deliberately - see
            // `Session::on_session_identity`. It is handled BEFORE the login request that
            // follows it, and the ordering is not luck: across 57 archived runs `0x0073` is
            // followed by `0x0080` three log lines later, every time, minimum equal to maximum.
            // The two arrive in the same read and are framed in order, so the account this
            // handler chooses is the account the character list is built from.
            CLIENT_SESSION_IDENTITY => self.on_session_identity(payload),
            CLIENT_CHECK_NAME_REQUEST => self.check_name(payload),
            CLIENT_CREATE_CHARACTER_REQUEST => self.create_character(payload),
            CLIENT_DELETE_CHARACTER_REQUEST => self.delete_character(payload),
            CLIENT_SELECT_CHARACTER_REQUEST => self.select_character(payload),
            _ => Vec::new(),
        }
    }

    /// The account name, the world, and the character list - in that order.
    ///
    /// Order is not cosmetic. The account name is drawn on the login screen the client is
    /// still showing; the world entry is what enables the Login button *and* fills the list
    /// the login result searches; the login result names a world that must already be in
    /// that list or the client stops without saying why.
    /// The world list and nothing after it, which leaves the client on **WorldSelect**.
    ///
    /// Answering is not optional - an unanswered packet freezes the client's whole UI - but
    /// the *login result* is, and it is what drives CharSelect. Three packets go out.
    fn world_list_only(&mut self, cause: &str) -> Vec<Reply> {
        self.world_head(cause)
    }

    /// What the client's login screen displays - **the account's own masked email**.
    ///
    /// The client cannot compute this: `research/` established that the name on the login
    /// screen comes from the server or the field stays blank. It used to come from
    /// `--display-name`, a hard-coded `wisp****@example.com` that `config.rs` described as
    /// standing in *"until there is something real to show"* - because the accounts table held
    /// no email. It holds one now, so the masked address is derived from the account this
    /// connection is actually being served as, and it changes with the claim.
    ///
    /// **The fallback is not decoration.** An account with no email - which is every account
    /// created before the column existed - would otherwise put an empty string in the field,
    /// and the client draws that as a blank line where a person expects to see themselves. So
    /// `--display-name` remains, as the answer for an account that cannot supply one.
    fn display_name(&self) -> String {
        self.account
            .masked_email()
            .unwrap_or_else(|| self.config.display_name.clone())
    }

    /// Account info, the world entry, and the end-of-list terminator.
    fn world_head(&mut self, cause: &str) -> Vec<Reply> {
        let world = &self.config.world;
        vec![
            Reply::new(
                ACCOUNT_INFO,
                account_info(&self.account.name, &self.display_name()),
                format!("{cause}: account info"),
            ),
            Reply::new(
                WORLD_LIST,
                world_list_entry(world.id as u8, &world.name, world.channel_count()),
                format!("{cause}: world {}", world.name),
            ),
            Reply::new(WORLD_LIST, world_list_end(), format!("{cause}: end of worlds")),
        ]
    }

    fn world_and_characters(&mut self, cause: &str) -> Vec<Reply> {
        let world = &self.config.world;
        let mut out = vec![
            Reply::new(
                ACCOUNT_INFO,
                account_info(&self.account.name, &self.display_name()),
                format!("{cause}: account info"),
            ),
            Reply::new(
                WORLD_LIST,
                world_list_entry(world.id as u8, &world.name, world.channel_count()),
                format!("{cause}: world {}", world.name),
            ),
            Reply::new(WORLD_LIST, world_list_end(), format!("{cause}: end of worlds")),
        ];

        let (channel, channel_note) = advertised_channel(world);

        let characters = match self.store.characters_for(self.account.id, world.id) {
            Ok(c) => c,
            Err(e) => {
                // Send an empty list rather than nothing. A character select screen with
                // no characters is recoverable; a client blocked on a reply is not.
                out.push(Reply::new(
                    LOGIN_RESULT,
                    login_result(world.id, channel, &[]),
                    format!("{cause}: EMPTY LIST - could not read characters: {e}"),
                ));
                return out;
            }
        };

        let names: Vec<&str> = characters.iter().map(|c| c.name.as_str()).collect();
        out.push(
            Reply::new(
                LOGIN_RESULT,
                login_result(world.id, channel, &characters),
                format!(
                    "{cause}: login result, {} character(s): {}; {channel_note}",
                    characters.len(),
                    names.join(", ")
                ),
            )
            // The only paused reply in this server. See `CHARACTER_LIST_PAUSE_MS`: the
            // avatars are not drawn on a client's first visit to character select, the bytes
            // are identical to a visit that DOES draw them, and this is the one thing this
            // server does that no real one could - answer four packets in one millisecond.
            .after(CHARACTER_LIST_PAUSE_MS),
        );
        out
    }

    /// **The client's session identity, `0x0073` - the credential the client carries itself.**
    ///
    /// The owner, 2026-08-29: *"using the client to pass a session should be what we aim for instead
    /// of inference."* This is the server half of that. The launcher mints a one-time token at
    /// sign-in, writes it where the hook can read it, the hook writes it into the client's own
    /// session object (`session+0x1b8`, whose setter `FUN_142c503c0` exists and has no
    /// callers), and the client builds, encrypts and sends `0x0073` itself. Nothing is forged.
    ///
    /// # Why this returns no reply, and why that is not a violation of "always answer"
    ///
    /// `CLAUDE.md`'s rule is that a packet the client **blocks on** must be answered, because
    /// an unanswered one freezes its entire UI. `0x0073` is not one: in 57 archived runs the
    /// client sends `0x0073` and then `0x0080` in the same millisecond, having received nothing
    /// in between. It has never been answered and has never blocked.
    ///
    /// Starting to answer it would be the *other* expensive mistake this project has recorded -
    /// answering with a packet the client did not expect leaves a latch set and kills the
    /// feature for the whole session. So the refusal is not expressed as a reply to `0x0073`;
    /// it is expressed in **which account's character list goes back in the answer to `0x0080`
    /// one millisecond later**, which the client does block on and which is always sent.
    ///
    /// # What each outcome does
    ///
    /// | identity | account served |
    /// |---|---|
    /// | empty (every capture to date, and every reconnect) | unchanged - whatever the weaker rules chose at accept time |
    /// | a valid unspent token | **the claim that minted it**, and the token is spent |
    /// | a token this same client process already spent | the same claim, nothing spent again |
    /// | a spent token from another process, or one nobody issued | **the `--account` fallback** - a presented credential gets its claim or none |
    fn on_session_identity(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(parsed) = SessionIdentity::parse(payload) else {
            // A body that does not parse is not a presentation - it buys exactly what silence
            // buys, which is the weaker rules. Truncating deliberately therefore gains an
            // attacker nothing over sending nothing at all.
            self.note(format!(
                "0x0073 IDENTITY: the {} byte body did not parse as (u32 mode, u16-prefixed \
                 string, tail). Treated as NO credential - the account stands as the weaker \
                 rules chose it. Nothing is refused on a malformed body",
                payload.len()
            ));
            return Vec::new();
        };

        // E1: the length, unconditionally, whatever happens next. Without this line a failed
        // E2 is indistinguishable from "the server ignored the field".
        //
        // `identity_bytes` is the WIRE byte count, which is the number the hook's own log line
        // tells the reader to compare against ("login.log should now show ... identity
        // length=N"). See the field's doc block for why the decoded length is a different
        // number. They differ only when what arrived is not UTF-8, and then the mismatch is
        // itself the finding, so it is printed rather than hidden.
        let decoded_note = if parsed.identity.len() == parsed.identity_bytes {
            String::new()
        } else {
            format!(
                " (those {} wire bytes are NOT valid UTF-8 and decoded to {} - so whatever \
                 reached the field is not a MapleCW token)",
                parsed.identity_bytes,
                parsed.identity.len()
            )
        };
        self.note(format!(
            "0x0073 IDENTITY: mode={} identity length={}{decoded_note} (expected {} for a \
             MapleCW client token; every capture before 2026-08-29 was 0) tail={} bytes",
            parsed.mode,
            parsed.identity_bytes,
            store::claims::CLIENT_TOKEN_CHARS,
            parsed.tail.len()
        ));

        let outcome = match self.store.present_client_token(&parsed.identity, self.launch_pid) {
            Ok(outcome) => outcome,
            Err(e) => {
                // A database failure must not become a refusal: that would drop a legitimate
                // player to the fallback because a table would not read. Leave the account
                // alone and say so - the same trade `resolve_account` makes on its own Err.
                self.note(format!(
                    "0x0073 IDENTITY: the client token lookup FAILED: {e}. The account is left \
                     as the weaker rules chose it - a database error must not be a refusal"
                ));
                return Vec::new();
            }
        };
        self.note(format!("0x0073 IDENTITY: {}", outcome.why()));

        match outcome {
            store::claims::ClientTokenOutcome::NotPresented => {}
            store::claims::ClientTokenOutcome::Accepted { claim, .. } => {
                // Re-resolve to the account the credential names. `get_account` rather than
                // trusting the claim's copy, so a renamed or since-disabled account cannot be
                // served off a stale string.
                match self.store.get_account(&claim.claim.account_name) {
                    Ok(Some(account)) if account.enabled => {
                        if account.id != self.account.id {
                            self.note(format!(
                                "0x0073 IDENTITY: account CHANGED {:?} -> {:?} by the client's \
                                 own credential. The character list about to be sent is {:?}'s",
                                self.account.name, account.name, account.name
                            ));
                        }
                        self.account = account;
                        // The migration this connection later mints binds to THIS claim.
                        self.claim_token_hash = claim.token_hash.clone();
                    }
                    _ => self.note(format!(
                        "0x0073 IDENTITY: the token named account {:?}, which is gone or \
                         disabled. Serving {:?} unchanged",
                        claim.claim.account_name, self.account.name
                    )),
                }
            }
            refused => {
                debug_assert!(refused.is_refusal());
                // THE ANTI-DOWNGRADE RULE, at the one place it can be broken. A wrong
                // credential must not leave this connection where a missing one would - if it
                // did, presenting junk would be as good as presenting nothing and the
                // credential would be worth nothing. `store::migration`'s invariant, mirrored.
                match self.fallback.clone() {
                    Some(fallback) => {
                        // The wording branches, the assignment does not. Saying "DOWNGRADING
                        // from X to X" when the weaker rules had already landed on the
                        // fallback reads as a bug in the log rather than as a refusal, and
                        // this line is the one the owner reads off a run.
                        let same = fallback.id == self.account.id;
                        self.note(if same {
                            format!(
                                "0x0073 IDENTITY: DOWNGRADING to the --account fallback {:?} - \
                                 which is what this connection was ALREADY being served as, so \
                                 nothing on screen changes. The refusal still happened",
                                fallback.name
                            )
                        } else {
                            format!(
                                "0x0073 IDENTITY: DOWNGRADING this connection from {:?} to the \
                                 --account fallback {:?}. A connection that presents a \
                                 credential gets THAT claim or none; it must not keep what the \
                                 weaker rules gave it, or a wrong token would be as good as no \
                                 token",
                                self.account.name, fallback.name
                            )
                        });
                        self.account = fallback;
                        self.claim_token_hash = None;
                    }
                    None => self.note(
                        "0x0073 IDENTITY: refused, but this session was never given a fallback \
                         account, so there is nothing to downgrade to. The account is UNCHANGED \
                         - wire Session::with_fallback in crates/login/src/server.rs"
                            .to_string(),
                    ),
                }
            }
        }
        Vec::new()
    }

    /// Answer the name check truthfully. This is the first thing a real server does that
    /// the harness could not: the harness replied "available" to every name, including
    /// names that were already taken.
    fn check_name(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(name) = read_str(payload) else {
            return Vec::new();
        };

        let (code, verdict) = match self.store.check_character_name(&name) {
            Ok(NameCheck::Available) => (NAME_AVAILABLE, "available".to_string()),
            Ok(NameCheck::AlreadyUsed) => (NAME_ALREADY_USED, "already taken".to_string()),
            Ok(NameCheck::NotAllowed) => (NAME_NOT_ALLOWED, "not allowed".to_string()),
            // A name we cannot check is a name we will not hand out.
            Err(e) => (NAME_ALREADY_USED, format!("REFUSED - name check failed: {e}")),
        };
        vec![Reply::new(
            CHECK_NAME_RESULT,
            check_name_result(&name, code),
            format!("name check {name:?}: {verdict}"),
        )]
    }

    /// Create the character, persist it, and send back what was stored.
    ///
    /// What goes back is read out of the *stored* row rather than out of the request, so
    /// the screen after creation shows what the next launch will show. Sending the request
    /// back instead would hide exactly the bug this crate exists to fix.
    fn create_character(&mut self, payload: &[u8]) -> Vec<Reply> {
        let world = &self.config.world;
        let Some(request) = CreateCharacterRequest::parse(payload) else {
            return vec![Reply::new(
                CREATE_CHARACTER_RESULT,
                create_character_failed(CREATE_CANNOT_PROCESS),
                "create refused: the request did not parse".to_string(),
            )];
        };

        let refuse = |code: u8, why: String| {
            vec![Reply::new(CREATE_CHARACTER_RESULT, create_character_failed(code), why)]
        };

        match self.store.character_count(self.account.id, world.id) {
            Ok(n) if n >= CHARACTER_SLOTS => {
                return refuse(
                    CREATE_INSUFFICIENT_SLOT,
                    format!("create refused: {n} characters already, {CHARACTER_SLOTS} slots"),
                )
            }
            Ok(_) => {}
            Err(e) => {
                return refuse(
                    CREATE_CANNOT_PROCESS,
                    format!("create refused: could not count characters: {e}"),
                )
            }
        }

        // The id argument is discarded: the database assigns the real one, and it has to
        // be distinct per character or the client silently drops the second.
        let wanted = request.character(0);
        match self.store.create_character(self.account.id, world.id, &wanted) {
            Ok(stored) => {
                let what = format!(
                    "created {:?} as id {} with {} equipped item(s)",
                    stored.name,
                    stored.id,
                    stored.equips.len()
                );
                vec![Reply::new(
                    CREATE_CHARACTER_RESULT,
                    create_character_result(world.id, &stored),
                    what,
                )]
            }
            Err(e) => refuse(CREATE_CANNOT_PROCESS, format!("create refused: {e}")),
        }
    }

    /// Delete a character, if this account owns it.
    ///
    /// The request is one `u32` and nothing else - no password, no confirmation token. The
    /// confirmation is a client-side dialog, so **the server cannot tell a confirmed delete
    /// from a forged one** and the only protection that means anything is the ownership
    /// check, which lives inside the SQL statement rather than in a prior read.
    ///
    /// Refusals must use [`DELETE_FAILED`] specifically: every other non-zero code falls
    /// through the client's switch to the branch that removes the character from the list
    /// anyway, which would show a delete that did not happen.
    fn delete_character(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(id) = payload
            .get(..4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        else {
            return vec![Reply::new(
                DELETE_CHARACTER_RESULT,
                delete_character_result(0, DELETE_FAILED),
                "delete refused: the request did not parse".to_string(),
            )];
        };

        // Look the name up before deleting, so the log says what went, not just an id.
        let name = self
            .store
            .characters_for(self.account.id, self.config.world.id)
            .ok()
            .and_then(|cs| cs.into_iter().find(|c| c.id == id).map(|c| c.name))
            .unwrap_or_else(|| "unknown".to_string());

        match self.store.delete_character(self.account.id, id) {
            Ok(true) => vec![Reply::new(
                DELETE_CHARACTER_RESULT,
                delete_character_result(id, DELETE_OK),
                format!("deleted {name:?} (id {id})"),
            )],
            // Not an error: the account does not own it, or it is already gone. Either way
            // the answer is the same, and it does not say which - a delete that reports
            // "no such character" differently from "not yours" is an ownership oracle.
            Ok(false) => vec![Reply::new(
                DELETE_CHARACTER_RESULT,
                delete_character_result(id, DELETE_FAILED),
                format!("delete refused: id {id} is not on this account"),
            )],
            Err(e) => vec![Reply::new(
                DELETE_CHARACTER_RESULT,
                delete_character_result(id, DELETE_FAILED),
                format!("delete refused: {e}"),
            )],
        }
    }

    /// Answer the select-character request: send the client to the game server.
    ///
    /// `0x0011` is the migration packet - identified statically, never captured, so this is
    /// the first thing here built entirely from a decode. See `docs/opcodes.md`.
    ///
    /// # Two client-side preconditions this has to respect
    ///
    /// The client looks the character id up in its own map (`FUN_14108cae0`) and **skips
    /// the entire action block on a miss** - no dialog, no reconnect, no clue. So an id
    /// that was not in the login result produces a silent nothing, which would read as "the
    /// migration packet is wrong". Refusing here instead makes that case say so out loud.
    ///
    /// And the address is the *advertise* address, not the bind address: it goes straight
    /// into the client's `sockaddr_in`, so it has to be reachable from the client machine.
    fn select_character(&mut self, payload: &[u8]) -> Vec<Reply> {
        let refuse = |why: String| {
            vec![Reply::new(
                MIGRATE_COMMAND,
                migrate_refused("Could not enter the world."),
                why,
            )]
        };

        let Some(request) = SelectCharacterRequest::parse(payload) else {
            return refuse("REFUSED - select-character body has no character id".to_string());
        };
        let id = request.character_id;

        let characters = match self.store.characters_for(self.account.id, self.config.world.id) {
            Ok(cs) => cs,
            Err(e) => return refuse(format!("REFUSED - could not read the character list: {e}")),
        };
        let Some(chosen) = characters.into_iter().find(|c| c.id == id) else {
            return refuse(format!("REFUSED - id {id} is not on this account"));
        };

        let world = &self.config.world;
        let channel = world.channel_id;
        let Some(listed) = world.channel_address(channel) else {
            return refuse(format!(
                "REFUSED - world {} has no address for channel {channel}; the client would be sent nowhere",
                world.id
            ));
        };
        // The host in that entry is only what --channels SAID. What the client is told is
        // decided here, for this connection, from the two ends of its socket. `net::advertise`.
        let (addr, advertised_as) =
            self.config.advertise.address_for(listed, self.peer_ip, self.local_ip);

        // The seed is minted here and claimed by the channel server out of the same
        // database. It is a u32 - all the packet has room for - so it identifies a pending
        // migration rather than proving anything. What it does buy is single use.
        //
        // **The row is bound to the sign-in that staked the live login claim.** Only the
        // claim's SHA-256 is read and re-stored; the plain token is not in this process. A
        // bound row can never be claimed by a connection that presents nothing - see the
        // invariant in `store::migration`. That is what stops a channel connection from
        // getting this character by merely asserting its id.
        //
        // `None` here means nobody has used the launcher, so there is no credential to bind
        // and the row is minted unbound - the old behaviour, hole included. It is logged as
        // such rather than silently accepted, because an unbound migration is exactly the
        // thing that is still impersonatable and the log is where that has to be visible.
        // **Off by default, and that is the honest state rather than an oversight.** A bound
        // row can only be claimed by a connection presenting a matching token, and the
        // channel server presents nothing today - so binding unconditionally would refuse
        // every migration and lock the player out of the world entirely. See
        // `Config::bind_migrations` for the measurement behind that.
        let (token_hash, binding) = if !self.config.bind_migrations {
            (
                None,
                "UNBOUND - migration binding is off (--bind-migrations). Any channel \
                 connection that names this character id can claim it. Binding needs the \
                 hook to present the session token on the channel socket; without that, \
                 turning it on refuses every migration"
                    .to_string(),
            )
        } else {
            // **This connection's claim, not the newest one.** `Store::live_claim_token_hash`
            // reads whichever claim is newest, which with two people signed in binds one
            // player's migration to the other's token - and then refuses the rightful
            // player's channel connection while accepting the other's. The hash is carried in
            // from `resolve_account`, which already decided which launch this is.
            //
            // There is no `Err` arm any more because there is no read here to fail: the
            // decision was made once, at accept time, where a failure could still be logged
            // against a connection rather than against a character select.
            match self.claim_token_hash.clone() {
                Some(hash) => (
                    Some(hash),
                    "BOUND to the login claim THIS CONNECTION resolved to".to_string(),
                ),
                None => (
                    None,
                    "UNBOUND - this connection did not resolve to a login claim (nobody is \
                     signed in, or more than one launch is and this connection could not be \
                     attributed to one). Any channel connection that names this character id \
                     can claim it. Sign in through maplecw-launcher first"
                        .to_string(),
                ),
            }
        };

        // The hash goes in with the INSERT. This server never holds the plain token - the
        // launcher was handed that at sign-in - so `create_migration_bound_hash` is the right
        // entry point, and binding in the INSERT means the row is never briefly visible in an
        // unbound state.
        let seed = match self.store.create_migration_bound_hash(
            self.account.id,
            id,
            world.id,
            channel,
            token_hash.as_deref(),
            self.peer.as_deref(),
        ) {
            Ok(seed) => seed,
            Err(e) => return refuse(format!("REFUSED - could not mint a migration: {e}")),
        };

        vec![Reply::new(
            MIGRATE_COMMAND,
            migrate(addr, id, seed),
            format!(
                "migrate {:?} (id {id}) to world {} channel {channel} at {addr}, seed {seed:#010x} - single use, NOT authentication. Migration {binding}. Advertised as {advertised_as}",
                chosen.name, world.id
            ),
        )]
    }
}

/// Read a `u16`-length-prefixed string from the front of a payload.
fn read_str(payload: &[u8]) -> Option<String> {
    let len = payload.get(..2).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize)?;
    let bytes = payload.get(2..2 + len)?;
    Some(String::from_utf8_lossy(bytes).into_owned())
}

/// The client's session identity, `0x0073`. Built by `FUN_141b21ea0` alongside the login
/// request.
///
/// **This is now the credential path**, not just a curiosity in the log. See
/// [`Session::on_session_identity`]. It is still never *answered* - the client does not block
/// on it and never has.
const CLIENT_SESSION_IDENTITY: u16 = 0x0073;

/// A decoded `0x0073` body.
///
/// The field sequence is from `research/msexe-packet-fields.txt`, which lists a builder's
/// encoder calls in order - `u32` launch mode, then `w_str` (a `u16` length prefix and that
/// many bytes) for the identity, then a raw tail that is a MAC address, a machine id and a
/// tick. Only the middle field is a credential; the tail is per-machine and is recorded, never
/// authorised on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionIdentity {
    /// The launch mode. `5` in all 72 captured bodies - `-NXLDEBUG`, the only mode this
    /// project has ever launched.
    pub mode: u32,
    /// The identity string, exactly as it came off the wire. **Not normalised** - the caller
    /// hands it to `store::claims::normalise_client_token`, which is the one place that
    /// decides what counts as "presented nothing".
    pub identity: String,
    /// **The `u16` length prefix as it arrived: the number of BYTES on the wire.**
    ///
    /// Not the same number as `identity.len()`, and the difference is the unit trap this
    /// project keeps paying for. `identity` is decoded with `from_utf8_lossy`, so a byte
    /// sequence that is not UTF-8 becomes replacement characters at **three bytes each** - and
    /// the decoded length would then be larger than what the client actually sent.
    ///
    /// This is the number the log prints, because the hook's own line
    /// (`IDENTITY wrote N bytes ... login.log should now show ... identity length=N`) is
    /// counting the marker file's bytes, and the whole point of that sentence is that the two
    /// can be compared. Comparing a wire byte count against a decoded string length would be
    /// right for every ASCII token and wrong exactly when something has gone wrong.
    pub identity_bytes: usize,
    /// Everything after the string: the 16-byte machine GUID and a tick, per the field list.
    pub tail: Vec<u8>,
}

impl SessionIdentity {
    /// `None` when the body is too short to hold the fields, or the length prefix runs past
    /// the end. Both are "no credential", never a refusal - see the caller.
    pub fn parse(payload: &[u8]) -> Option<Self> {
        let mode = payload.get(..4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))?;
        let rest = &payload[4..];
        let len = rest.get(..2).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize)?;
        let bytes = rest.get(2..2 + len)?;
        Some(SessionIdentity {
            mode,
            // Lossy on purpose: a byte sequence that is not UTF-8 is not our token, and it must
            // still produce a value that can be logged and refused rather than an error.
            identity: String::from_utf8_lossy(bytes).into_owned(),
            identity_bytes: len,
            tail: rest[2 + len..].to_vec(),
        })
    }
}

/// **The hand-off the migration packet is supposed to produce**, built by `FUN_1415d10e0`.
///
/// The `u32` seed in a [`MIGRATE_COMMAND`] is stashed at `DAT_143ac80b0` (XORed with a
/// replicated random byte, unmasked with `DAT_143ac80b8`) and written into this packet by
/// `FUN_1415deae0` when the client opens the new connection. So `0x007D` is where a
/// server-supplied token would come back - and it is the check on the whole `0x0011`
/// decode: the seed comes home or it does not.
///
/// The layout is **not** decoded. `FUN_1415d10e0` writes 16 bytes, two `u8`, a `u32`, and
/// the launch mode from `session+0x68` before it reaches the key/length/payload block, and
/// that prefix has not been read carefully enough to index. The body hex in the log is the
/// instrument until a real one is captured.
const CLIENT_MIGRATION_HELLO: u16 = 0x007D;

/// A human-readable note about a packet we do not answer, or `None` if there is nothing
/// worth saying. Pure, so the interesting decode is testable without a socket.
///
/// # Why `0x0073` in particular
///
/// It is the packet the client already builds that has a place for a session identity, and
/// **the identity has been empty in every one of the 72 bodies ever captured** - not because
/// the field does not exist, but because its setter `FUN_142c503c0` has zero callers
/// (`research/client-session-args.md` section 3). The hook fills it; this decodes it.
///
/// This function is the **raw** decode and stays raw on purpose: it says what arrived, in
/// bytes, before anything decides what it means. The decision and its consequences are
/// [`Session::on_session_identity`], and both lines go to the log. When E2 is run, this is the
/// line that says whether the client carried anything at all, and that one says what the
/// server did about it - two separate claims, either of which can come back false.
pub fn describe(opcode: u16, payload: &[u8]) -> Option<String> {
    if opcode == CLIENT_MIGRATION_HELLO {
        return Some(format!(
            "MIGRATION HELLO: the client reconnected after 0x0011 and sent {} bytes. The seed is NOT in here - measured 2026-08-29 across 115 distinct hello bodies against all 74 seeds this server has minted, plain and under the XOR-with-a-replicated-byte form, 8510 trials and zero hits, with the character id at offset 8 as a passing positive control. What IS in here is the character id, and it is asserted rather than proved. See docs/opcodes.md.",
            payload.len()
        ));
    }
    if opcode != CLIENT_SESSION_IDENTITY {
        return None;
    }
    let Some(parsed) = SessionIdentity::parse(payload) else {
        return Some(format!(
            "0x0073 session identity: MALFORMED - {} byte body is too short for (u32 mode, \
             u16-prefixed string, tail), or its length prefix runs past the end. Full body is \
             above",
            payload.len()
        ));
    };
    let shape = if parsed.identity.is_empty() {
        "EMPTY - this is what all 72 captures before 2026-08-29 carried, and what the stock \
         client sends. The hook has NOT written the field"
            .to_string()
    } else if parsed.identity.len() == store::claims::CLIENT_TOKEN_CHARS
        && parsed
            .identity
            .bytes()
            .all(|b| b.is_ascii_uppercase() || (b'2'..=b'7').contains(&b))
    {
        format!(
            "{} characters of uppercase base32 - THE SHAPE OF A MAPLECW CLIENT TOKEN. Whether \
             it MATCHES a live claim is the next log line",
            parsed.identity.len()
        )
    } else {
        format!(
            "{} characters, but NOT the shape of a MapleCW client token ({} characters of \
             A-Z2-7). Either the hook wrote something else, or the client transformed it in \
             transit - compare it against what the launcher wrote",
            parsed.identity.len(),
            store::claims::CLIENT_TOKEN_CHARS
        )
    };
    Some(format!(
        // The opcode is IN the line, not just above it. `login.log` interleaves connections,
        // and the one thing anybody will do with this feature is `grep 0x0073 login.log` -
        // which silently missed the EMPTY case while this line began with "session".
        "0x0073 session identity: mode={} identity={:?} ({} bytes) tail={} - {shape}",
        parsed.mode,
        parsed.identity,
        parsed.identity_bytes,
        parsed.tail.iter().map(|b| format!("{b:02x}")).collect::<String>()
    ))
}

/// Build a request body the way the client does: opcode, then payload.
pub fn request(opcode: u16, payload: &[u8]) -> Vec<u8> {
    let mut out = opcode.to_le_bytes().to_vec();
    out.extend_from_slice(payload);
    out
}

/// A `Character` as the wire would carry it, for tests and for callers that want to seed
/// a database without going through a create request.
pub fn seed_character(name: &str) -> Character {
    Character { name: name.to_string(), ..Character::default() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::World;

    fn session() -> Session {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let id = store.create_account("maplecw", "correct horse battery").unwrap();
        let account = store.get_account("maplecw").unwrap().unwrap();
        assert_eq!(account.id, id);
        Session::new(store, Arc::new(Config::default()), account)
    }

    /// A session whose account carries an email, so the login screen has something real.
    fn session_with_email(email: &str) -> Session {
        let store = Arc::new(Store::open_in_memory().unwrap());
        store.create_account("maplecw", "correct horse battery").unwrap();
        store.set_email("maplecw", Some(email)).unwrap();
        let account = store.get_account("maplecw").unwrap().unwrap();
        Session::new(store, Arc::new(Config::default()), account)
    }

    /// The login screen shows the ACCOUNT's masked email, not a configured constant.
    ///
    /// `--display-name` used to be the only answer, and `config.rs` said it stood in "until
    /// there is something real to show". This is that.
    #[test]
    fn the_login_screen_shows_the_masked_email_of_the_account_being_served() {
        let s = session_with_email("wispplayer@example.com");
        assert_eq!(s.display_name(), "wisp****@example.com");
    }

    #[test]
    fn the_masked_email_reaches_the_account_info_packet() {
        // Not just the helper - the bytes. `account_info` writes the display name as the last
        // string in the body, and that is the field the client draws.
        let mut s = session_with_email("wispplayer@example.com");
        let replies = s.world_head("test");
        let info = replies.iter().find(|r| r.opcode == ACCOUNT_INFO).expect("account info");
        let body = info.packet();
        let needle = "wisp****@example.com".as_bytes();
        assert!(
            body.windows(needle.len()).any(|w| w == needle),
            "the masked address is not in the packet the client reads"
        );
        assert!(
            !body.windows(21).any(|w| w == "wispplayer@example.com".as_bytes()),
            "the FULL address must never reach the client"
        );
    }

    /// An account with no email must not blank the field - the client draws an empty string
    /// as a blank line where a person expects to see themselves.
    #[test]
    fn an_account_without_an_email_falls_back_to_the_configured_display_name() {
        let s = session();
        assert_eq!(s.display_name(), Config::default().display_name);
        assert!(!s.display_name().is_empty(), "a blank field is the failure this avoids");
    }

    /// The account is resolved per connection now, so the screen has to follow it. If this
    /// ever regresses, the launcher would sign in as one account and the login screen would
    /// name another - and the two would be reported as the claim not working.
    #[test]
    fn the_screen_follows_the_account_not_the_configuration() {
        let a = session_with_email("wispplayer@example.com");
        let b = session_with_email("someone.else@example.com");
        assert_ne!(a.display_name(), b.display_name());
        assert_eq!(b.display_name(), "some****@example.com");
    }

    fn opcodes(replies: &[Reply]) -> Vec<u16> {
        replies.iter().map(|r| r.opcode).collect()
    }

    /// The whole point of [`advertised_channel`]: a world whose id is `0` and whose real
    /// channel is `0` must **not** say so, because that is exactly the pair a client with no
    /// channel list already holds, and the rebuild bails on a match.
    #[test]
    fn a_two_channel_world_at_zero_advertises_channel_one_instead_of_the_truth() {
        let world = World {
            id: 0,
            name: "Scania".to_string(),
            channels: vec!["127.0.0.1:8485".parse().unwrap(), "127.0.0.1:8486".parse().unwrap()],
            channel_id: 0,
        };
        let (channel, why) = advertised_channel(&world);
        assert_eq!(channel, 1, "channel 0 is the idle pair and would leave the dialog empty");
        assert!(why.contains("instead of the true 0"), "{why}");
    }

    /// One channel and there is no move: index 0 is the only one the client's range check
    /// accepts and it is the idle pair. The honest value goes out and the label says why,
    /// because a silent fallback here looks identical to the bug it is working around.
    #[test]
    fn a_one_channel_world_sends_the_true_channel_and_says_the_dialog_cannot_populate() {
        let world = World::default();
        assert_eq!(world.channels.len(), 1, "this test is about the one-channel case");
        let (channel, why) = advertised_channel(&world);
        assert_eq!(channel, world.channel_id);
        assert!(why.contains("stay at one row"), "{why}");
    }

    /// A non-zero channel is already unequal to the idle pair, so it travels untouched.
    /// Nothing here is allowed to invent a channel the client would then try to enter.
    #[test]
    fn a_channel_that_is_not_the_idle_pair_is_sent_unchanged() {
        let world = World {
            id: 0,
            name: "Scania".to_string(),
            channels: vec!["127.0.0.1:8485".parse().unwrap(), "127.0.0.1:8486".parse().unwrap()],
            channel_id: 1,
        };
        assert_eq!(advertised_channel(&world).0, 1);
    }

    /// The advertised channel reaches the wire. `LOGIN_RESULT`'s channel field is at body
    /// offset 16 - u8 result, an empty u16-prefixed string, u8, an 8-byte FILETIME, then the
    /// u32 world id - and this asserts the bytes rather than the call.
    #[test]
    fn the_login_result_on_the_wire_carries_the_advertised_channel() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        store.create_account("maplecw", "correct horse battery").unwrap();
        let account = store.get_account("maplecw").unwrap().unwrap();
        let mut config = Config::default();
        config.world.channels =
            vec!["127.0.0.1:8485".parse().unwrap(), "127.0.0.1:8486".parse().unwrap()];
        config.world.channel_id = 0;
        let mut s = Session::new(store, Arc::new(config), account);

        let replies = s.world_and_characters("test");
        let result = replies
            .iter()
            .find(|r| r.opcode == LOGIN_RESULT)
            .expect("a login result is always sent");
        assert_eq!(&result.body[12..16], &0u32.to_le_bytes(), "the world id, for orientation");
        assert_eq!(
            &result.body[16..20],
            &1u32.to_le_bytes(),
            "the priming channel, not the true 0 - research/channel-select.md section 9"
        );
        assert!(result.what.contains("instead of the true 0"), "{}", result.what);
    }

    /// **The character list is the only reply that waits, and it really does wait.**
    ///
    /// Two halves, and the second is the one worth having. Pinning that `LOGIN_RESULT` has a
    /// pause is easy; pinning that **nothing else does** is what stops a later change from
    /// putting latency on the whole login path while this test still passed.
    ///
    /// The three replies ahead of it must stay instant: the client is blocked on its `recv`
    /// for the account info and the world list, and delaying those would be a real cost for
    /// an experiment that is about the gap between them and the list.
    #[test]
    fn only_the_character_list_is_delayed_and_nothing_ahead_of_it_is() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        store.create_account("maplecw", "correct horse battery").unwrap();
        let account = store.get_account("maplecw").unwrap().unwrap();
        let mut s = Session::new(store, Arc::new(Config::default()), account);

        let replies = s.world_and_characters("test");
        let paused: Vec<&Reply> = replies.iter().filter(|r| r.pause_ms > 0).collect();
        assert_eq!(paused.len(), 1, "exactly one reply waits: {:?}", replies);
        assert_eq!(paused[0].opcode, LOGIN_RESULT, "and it is the character list");
        assert_eq!(paused[0].pause_ms, CHARACTER_LIST_PAUSE_MS);

        // The control that gives that its meaning: the replies AHEAD of it are instant, and
        // there are three of them.
        let instant: Vec<&Reply> = replies.iter().filter(|r| r.pause_ms == 0).collect();
        assert_eq!(instant.len(), 3, "account info and both world-list rows stay instant");
        assert!(
            instant.iter().all(|r| r.opcode != LOGIN_RESULT),
            "no unpaused character list slipped through: {instant:?}"
        );

        // And the pause is long enough to be a gap rather than jitter. Loopback delivered all
        // four of these in the SAME MILLISECOND in the capture this exists to explain, so a
        // value that could be lost in scheduling noise would test nothing.
        assert!(
            CHARACTER_LIST_PAUSE_MS >= 100,
            "a pause smaller than scheduling noise is not an experiment"
        );
    }

    /// An empty list is still answered, and **it waits too**.
    ///
    /// The error path builds its own `LOGIN_RESULT` rather than falling through to the one
    /// below it, so it is a second place to forget. A character select that draws nothing is
    /// exactly the screen this pause exists for.
    #[test]
    fn the_empty_character_list_is_delayed_the_same_way() {
        // `characters_for` failing is hard to force, so this asserts the shape instead: every
        // LOGIN_RESULT this function can emit carries the pause.
        let store = Arc::new(Store::open_in_memory().unwrap());
        store.create_account("maplecw", "correct horse battery").unwrap();
        let account = store.get_account("maplecw").unwrap().unwrap();
        let mut s = Session::new(store, Arc::new(Config::default()), account);
        for r in s.world_and_characters("test").iter().filter(|r| r.opcode == LOGIN_RESULT) {
            assert_eq!(r.pause_ms, CHARACTER_LIST_PAUSE_MS, "{}", r.what);
        }
    }

    /// The name-check body: a length-prefixed name.
    fn name_request(name: &str) -> Vec<u8> {
        let mut payload = (name.len() as u16).to_le_bytes().to_vec();
        payload.extend_from_slice(name.as_bytes());
        request(CLIENT_CHECK_NAME_REQUEST, &payload)
    }

    /// A create request, in the layout measured off the wire.
    fn create_request(name: &str, hair: u32, items: &[(u32, u32)]) -> Vec<u8> {
        let mut p = Vec::new();
        p.extend_from_slice(&(name.len() as u16).to_le_bytes());
        p.extend_from_slice(name.as_bytes());
        p.extend_from_slice(&0u32.to_le_bytes()); // discarded
        p.extend_from_slice(&0u32.to_le_bytes()); // discarded
        p.extend_from_slice(&0u32.to_le_bytes()); // race
        p.extend_from_slice(&0u16.to_le_bytes()); // subJob
        for stat in [12u32, 5, 4, 4] {
            p.extend_from_slice(&stat.to_le_bytes());
        }
        p.extend_from_slice(&1u32.to_le_bytes()); // gender
        p.extend_from_slice(&2u32.to_le_bytes()); // skin
        p.extend_from_slice(&hair.to_le_bytes());
        p.extend_from_slice(&(items.len() as u32).to_le_bytes());
        for (category, item) in items {
            p.extend_from_slice(&category.to_le_bytes());
            p.extend_from_slice(&item.to_le_bytes());
        }
        request(CLIENT_CREATE_CHARACTER_REQUEST, &p)
    }

    const STYLE: [(u32, u32); 6] =
        [(1, 21002), (2, 30030), (3, 1040002), (4, 1060002), (5, 1072001), (6, 1302000)];

    #[test]
    fn the_login_request_is_answered_with_four_packets_in_order() {
        let mut s = session();
        let replies = s.handle(&request(CLIENT_LOGIN_REQUEST, &[]));
        assert_eq!(
            opcodes(&replies),
            vec![ACCOUNT_INFO, WORLD_LIST, WORLD_LIST, LOGIN_RESULT]
        );
    }

    #[test]
    fn leaving_the_world_is_answered_the_same_way_every_time() {
        // It used to be a one-shot, so picking a world a second time hung on
        // "Connecting..." forever. Nothing about this reply is stateful.
        let mut s = session();
        let first = s.handle(&request(CLIENT_LEAVE_WORLD_REQUEST, &[]));
        let second = s.handle(&request(CLIENT_LEAVE_WORLD_REQUEST, &[]));
        assert_eq!(opcodes(&first), vec![ACCOUNT_INFO, WORLD_LIST, WORLD_LIST]);
        assert_eq!(first.iter().map(|r| &r.body).collect::<Vec<_>>(),
                   second.iter().map(|r| &r.body).collect::<Vec<_>>());
    }

    /// Picking a world must be answered, or the client sits on "Connecting to server..."
    /// forever. That is the UI-freeze rule on the screen it was first observed on.
    #[test]
    fn selecting_a_world_is_answered_with_the_character_list() {
        let mut s = session();
        let replies = s.handle(&request(CLIENT_SELECT_WORLD, &[]));
        assert!(!replies.is_empty(), "silence here freezes the client's whole UI");
        assert_eq!(
            opcodes(&replies),
            vec![ACCOUNT_INFO, WORLD_LIST, WORLD_LIST, LOGIN_RESULT],
            "the answer is the character list for the chosen world"
        );
    }

    /// The real 171-byte body, from the first run that ever reached WorldSelect. A handler
    /// tested only against an empty body proves nothing about the one the client sends.
    #[test]
    fn the_captured_world_select_body_is_answered() {
        let mut s = session();
        let body = hex_body(
            "00000000020000007f0000012f00414d442052797a656e2037203737303058             8382d436f72652050726f636573736f72"
        );
        let replies = s.handle(&request(CLIENT_SELECT_WORLD, &body));
        assert!(!replies.is_empty(), "a real body must be answered too");
        assert!(opcodes(&replies).contains(&LOGIN_RESULT));

        // And a truncated one must not panic - it comes off a socket.
        for n in 0..8 {
            assert!(!s.handle(&request(CLIENT_SELECT_WORLD, &body[..n])).is_empty());
        }
    }

    fn hex_body(h: &str) -> Vec<u8> {
        let h: String = h.chars().filter(|c| !c.is_whitespace()).collect();
        (0..h.len() / 2)
            .map(|i| u8::from_str_radix(&h[i * 2..i * 2 + 2], 16).expect("hex"))
            .collect()
    }

    /// Leaving the world must **not** send a login result, and that is the whole point.
    ///
    /// The world-list terminator transitions the client to screen 2, WorldSelect
    /// (`FUN_141b2fac0`'s end-of-list branch calls `FUN_141b3f050(stage, 2, 400)`; the stage
    /// table in `docs/session.md` is read out of `FUN_141127730` and registers 2 as
    /// WorldSelect, 4 as CharSelect). A login result after it drags the client to CharSelect,
    /// which is why "Choose another world" always landed back on the character screen.
    ///
    /// The login REQUEST still sends one - that path is meant to reach CharSelect.
    #[test]
    fn leaving_the_world_sends_no_login_result_so_the_client_stays_on_world_select() {
        let mut s = session();
        let leave = s.handle(&request(CLIENT_LEAVE_WORLD_REQUEST, &[]));
        assert!(
            !opcodes(&leave).contains(&LOGIN_RESULT),
            "a login result here pulls the client off WorldSelect and onto CharSelect"
        );
        assert!(!leave.is_empty(), "still answered - silence freezes the client's whole UI");
        assert_eq!(*opcodes(&leave).last().expect("a reply"), WORLD_LIST,
                   "the terminator must be the last thing the client sees");

        let login = s.handle(&request(CLIENT_LOGIN_REQUEST, &[]));
        assert!(opcodes(&login).contains(&LOGIN_RESULT), "the login path still reaches CharSelect");
    }

    #[test]
    fn every_request_the_client_blocks_on_gets_an_answer() {
        // The UI-freeze rule, as a test. Each of these is a button the player can press.
        let mut s = session();
        for (opcode, payload) in [
            (CLIENT_LOGIN_REQUEST, vec![]),
            (CLIENT_LEAVE_WORLD_REQUEST, vec![]),
            (CLIENT_ENTER_CREATION_REQUEST, vec![0x01, 0x00, 0x2e]),
            (CLIENT_CHECK_NAME_REQUEST, name_request("Hello")[2..].to_vec()),
            (CLIENT_SELECT_CHARACTER_REQUEST, select_payload(999)),
        ] {
            let replies = s.handle(&request(opcode, &payload));
            assert!(!replies.is_empty(), "opcode 0x{opcode:04X} went unanswered");
        }
    }

    #[test]
    fn a_created_character_is_in_the_next_login_result() {
        // The whole point of the crate, in one test.
        let mut s = session();
        s.handle(&create_request("Hello", 30030, &STYLE));

        let replies = s.handle(&request(CLIENT_LOGIN_REQUEST, &[]));
        let result = replies.last().unwrap();
        assert_eq!(result.opcode, LOGIN_RESULT);
        assert!(result.what.contains("Hello"), "{}", result.what);
        assert!(result.what.contains("1 character"), "{}", result.what);
    }

    #[test]
    fn a_created_character_survives_a_new_session_on_the_same_store() {
        // A relaunch is a new connection against the same database. This is the thing
        // that was never true before: the harness generated the list at launch, so a
        // character created in one run was gone in the next.
        let store = Arc::new(Store::open_in_memory().unwrap());
        store.create_account("maplecw", "correct horse battery").unwrap();
        let account = store.get_account("maplecw").unwrap().unwrap();
        let config = Arc::new(Config::default());

        let mut first = Session::new(store.clone(), config.clone(), account.clone());
        first.handle(&create_request("Hello", 30030, &STYLE));

        let mut second = Session::new(store, config, account);
        let replies = second.handle(&request(CLIENT_LOGIN_REQUEST, &[]));
        assert!(replies.last().unwrap().what.contains("Hello"));
    }

    #[test]
    fn the_reply_carries_the_style_that_was_asked_for() {
        // The bug this replaces: a canned reply built from Character::default() sent the
        // character back naked and with the wrong hair.
        let mut s = session();
        let replies = s.handle(&create_request("Dressed", 30030, &STYLE));
        assert_eq!(replies.len(), 1);
        assert_eq!(replies[0].opcode, CREATE_CHARACTER_RESULT);
        assert!(replies[0].what.contains("4 equipped item(s)"), "{}", replies[0].what);

        let stored = s.store.characters_for(s.account.id, 0).unwrap();
        assert_eq!(stored[0].hair, 30030);
        assert_eq!(stored[0].face, 21002);
        assert_eq!(stored[0].gender, 1);
        assert_eq!(stored[0].skin, 2);
        // Equipment slots, not creation categories: top 5, bottom 6, shoes 7, weapon 11.
        assert_eq!(
            stored[0].equips,
            vec![(5, 1040002), (6, 1060002), (7, 1072001), (11, 1302000)]
        );
    }

    #[test]
    fn two_characters_get_two_different_ids() {
        // Sending id 200 twice made the client drop the second character silently.
        let mut s = session();
        s.handle(&create_request("Alpha", 30030, &STYLE));
        s.handle(&create_request("Bravo", 30020, &STYLE));
        let stored = s.store.characters_for(s.account.id, 0).unwrap();
        assert_eq!(stored.len(), 2);
        assert_ne!(stored[0].id, stored[1].id);
    }

    #[test]
    fn the_name_check_tells_the_truth_about_a_taken_name() {
        let mut s = session();
        let free = s.handle(&name_request("Hello"));
        assert_eq!(free[0].body.last(), Some(&NAME_AVAILABLE));

        s.handle(&create_request("Hello", 30030, &STYLE));

        let taken = s.handle(&name_request("Hello"));
        assert_eq!(taken[0].opcode, CHECK_NAME_RESULT);
        assert_eq!(taken[0].body.last(), Some(&NAME_ALREADY_USED));
    }

    #[test]
    fn the_name_check_answers_about_the_name_that_was_asked() {
        // The client compares the name we send back; a reply about some other name is a
        // reply it will not match.
        let mut s = session();
        let replies = s.handle(&name_request("Marbles"));
        assert_eq!(replies[0].body, check_name_result("Marbles", NAME_AVAILABLE));
    }

    #[test]
    fn a_name_the_client_could_not_carry_is_refused() {
        let mut s = session();
        let replies = s.handle(&name_request("waytoolongforthirteen"));
        assert_eq!(replies[0].body.last(), Some(&NAME_NOT_ALLOWED));
    }

    #[test]
    fn creating_a_duplicate_name_is_refused_rather_than_stored_twice() {
        let mut s = session();
        s.handle(&create_request("Hello", 30030, &STYLE));
        let replies = s.handle(&create_request("Hello", 30020, &STYLE));

        assert_eq!(replies[0].opcode, CREATE_CHARACTER_RESULT);
        assert_ne!(replies[0].body[0], 0, "a refusal must not carry the success code");
        assert_eq!(s.store.characters_for(s.account.id, 0).unwrap().len(), 1);
    }

    #[test]
    fn a_full_account_is_refused_with_the_slot_code() {
        let mut s = session();
        for name in ["Alpha", "Bravo", "Charlie"] {
            s.handle(&create_request(name, 30030, &STYLE));
        }
        assert_eq!(s.store.characters_for(s.account.id, 0).unwrap().len(), 3);

        let replies = s.handle(&create_request("Delta", 30030, &STYLE));
        assert_eq!(replies[0].body, vec![CREATE_INSUFFICIENT_SLOT]);
        assert_eq!(s.store.characters_for(s.account.id, 0).unwrap().len(), 3);
    }

    #[test]
    fn a_create_request_that_does_not_parse_is_still_answered() {
        let mut s = session();
        let replies = s.handle(&request(CLIENT_CREATE_CHARACTER_REQUEST, &[0xFF, 0xFF]));
        assert_eq!(replies.len(), 1);
        assert_eq!(replies[0].opcode, CREATE_CHARACTER_RESULT);
        assert_ne!(replies[0].body[0], 0);
    }

    #[test]
    fn the_startup_gate_goes_out_unprompted_and_on_request() {
        let mut s = session();
        assert_eq!(opcodes(&s.on_connect()), vec![DATA_WZ_PATCH]);
        assert_eq!(opcodes(&s.handle(&request(CLIENT_DATA_WZ_REQUEST, &[]))), vec![DATA_WZ_PATCH]);
    }

    #[test]
    fn a_quiet_client_is_sent_the_startup_gate_again_until_it_logs_in() {
        let mut s = session();
        assert_eq!(opcodes(&s.on_quiet()), vec![DATA_WZ_PATCH]);

        s.handle(&request(CLIENT_LOGIN_REQUEST, &[]));
        // Once it is logged in, quiet is just quiet - repeating the gate then would put
        // an unexpected packet into a client that has moved on.
        assert!(s.on_quiet().is_empty());
    }

    #[test]
    fn an_unknown_opcode_is_ignored_rather_than_answered_with_noise() {
        // The client's opening burst carries version and environment reports and log
        // uploads that want no reply. Answering them with something would put a random
        // opcode into its dispatcher.
        let mut s = session();
        for opcode in [0x0070u16, 0x0071, 0x008F, 0x0090, 0x0091, 0x00C0, 0x0073, 0x007A] {
            assert!(s.handle(&request(opcode, &[0; 8])).is_empty(), "0x{opcode:04X}");
        }
    }

    // ------------------------------------------------------------------------------------
    // 0x0073, THE CLIENT'S OWN CREDENTIAL. The owner, 2026-08-29: "using the client to pass a
    // session should be what we aim for instead of inference."
    //
    // Every test here is about the LOGIN socket. `0x0073` has never appeared on a channel
    // connection - 103 archived files, every one port 8484 - so none of this touches
    // CLAUDE.md's standing constraint that the game socket carries no credentials.
    // ------------------------------------------------------------------------------------

    /// A `0x0073` body in the layout measured off the wire: `u32` mode, `u16`-prefixed
    /// identity, then the machine tail. The tail bytes are the real ones from
    /// `previous-runs/login-20260829-094630.log`.
    fn identity_request(mode: u32, identity: &str) -> Vec<u8> {
        let mut p = mode.to_le_bytes().to_vec();
        p.extend_from_slice(&(identity.len() as u16).to_le_bytes());
        p.extend_from_slice(identity.as_bytes());
        p.extend_from_slice(&hex_body("d843ae4c5617b6ae9cd200000000764d00000000"));
        request(CLIENT_SESSION_IDENTITY, &p)
    }

    /// Two accounts, a character on each, and a live claim for `claimed`. The session starts
    /// out serving `served_as` - which is what the weaker rules would have chosen at accept
    /// time - with the other account as the `--account` fallback.
    ///
    /// Returns the session and the client token the claim minted.
    fn session_with_claim(served_as: &str, claimed: &str) -> (Session, String) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        for name in ["maplecw", "second_one"] {
            store.create_account(name, "correct horse battery").unwrap();
        }
        let claimed_id = store.get_account(claimed).unwrap().unwrap().id;
        let session_token = match store.authenticate(claimed, "correct horse battery").unwrap() {
            store::AuthOutcome::Ok { token, .. } => token,
            other => panic!("the test account must authenticate: {other:?}"),
        };
        let staked = store
            .stake_login_claim_with(
                claimed_id,
                &session_token,
                store::LOGIN_CLAIM_TTL_SECS,
                Some("127.0.0.1"),
            )
            .unwrap();

        // One character per account, differently named, so "which list came back" is legible.
        let config = Arc::new(Config::default());
        for (name, character) in [("maplecw", "AlphaChar"), ("second_one", "BetaChar")] {
            let account = store.get_account(name).unwrap().unwrap();
            let mut s = Session::new(store.clone(), config.clone(), account);
            s.handle(&create_request(character, 30030, &STYLE));
        }

        let start = store.get_account(served_as).unwrap().unwrap();
        let fallback = store.get_account("maplecw").unwrap().unwrap();
        let session = Session::new(store, config, start)
            .with_fallback(fallback)
            .with_launch_pid(Some(4242));
        (session, staked.client_token)
    }

    /// The names in the character list the login request would produce.
    fn character_list(s: &mut Session) -> String {
        let replies = s.handle(&request(CLIENT_LOGIN_REQUEST, &[]));
        replies
            .iter()
            .find(|r| r.opcode == LOGIN_RESULT)
            .expect("a login result is always sent")
            .what
            .clone()
    }

    /// **The headline, end to end through the state machine.**
    ///
    /// The connection starts being served as the wrong account - which is what an unattributed
    /// connection gets today - then the client carries its one-time token in `0x0073`, and the
    /// character list that goes out one packet later is the RIGHT account's.
    ///
    /// This is the whole of E1: the server's half of the transaction, proved without a client.
    #[test]
    fn a_client_token_in_0x0073_changes_the_account_before_the_character_list_goes_out() {
        let (mut s, client_token) = session_with_claim("maplecw", "second_one");

        // The control: without the token this connection is serving the WRONG account, so the
        // assertion below is about the credential and not about the fixture.
        let before = character_list(&mut s);
        assert!(before.contains("AlphaChar"), "{before}");
        assert!(!before.contains("BetaChar"), "{before}");

        let replies = s.handle(&identity_request(5, &client_token));
        assert!(replies.is_empty(), "0x0073 must not be answered - the client does not block");
        let notes = s.take_notes().join("\n");
        assert!(notes.contains("ACCEPTED and SPENT"), "{notes}");
        assert!(notes.contains("account CHANGED"), "{notes}");

        let after = character_list(&mut s);
        assert!(after.contains("BetaChar"), "the credential did not change the list: {after}");
        assert!(!after.contains("AlphaChar"), "{after}");
        assert_eq!(s.account_name(), "second_one");
        // And the migration this connection mints now binds to THAT claim rather than to none.
        assert!(s.claim_token_hash.is_some(), "an accepted credential must carry its binding");
    }

    /// **The anti-downgrade rule at the place it can be broken.** A wrong token must take the
    /// connection down to the `--account` fallback - not leave it where presenting nothing
    /// would leave it, or a wrong token would be as good as no token.
    #[test]
    fn a_wrong_client_token_downgrades_the_connection_to_the_fallback() {
        let (mut s, _) = session_with_claim("second_one", "second_one");
        assert!(character_list(&mut s).contains("BetaChar"), "the control");

        let replies = s.handle(&identity_request(5, "AAAAAAAAAAAAAAAAAAAAAAAAAA"));
        assert!(replies.is_empty());
        let notes = s.take_notes().join("\n");
        assert!(notes.contains("REFUSED"), "{notes}");
        assert!(notes.contains("DOWNGRADING"), "{notes}");

        assert_eq!(s.account_name(), "maplecw", "a refused credential must not keep the account");
        assert!(character_list(&mut s).contains("AlphaChar"));
        assert!(s.claim_token_hash.is_none(), "and it must not keep the binding either");
    }

    /// **The compatibility hinge, on the login server's own path.** Every `0x0073` captured
    /// before today carries a zero-length identity. If empty counted as a presentation, every
    /// stock client would be downgraded to the fallback the moment it sent this packet - and
    /// on screen that is "my characters vanished", not "the credential is missing".
    #[test]
    fn an_empty_identity_leaves_the_account_exactly_where_it_was() {
        let (mut s, _) = session_with_claim("second_one", "second_one");
        let replies = s.handle(&identity_request(5, ""));
        assert!(replies.is_empty());

        let notes = s.take_notes().join("\n");
        assert!(notes.contains("identity length=0"), "E1 must log the length: {notes}");
        assert!(notes.contains("no client token was carried"), "{notes}");
        assert!(!notes.contains("DOWNGRADING"), "an empty identity is NOT a refusal: {notes}");

        assert_eq!(s.account_name(), "second_one");
        assert!(character_list(&mut s).contains("BetaChar"));
    }

    /// **The real captured body**, byte for byte, from `login-20260829-094630.log`. A handler
    /// tested only against bodies this file builds proves nothing about the one the client
    /// actually sends - and this is the exact shape 57 archived runs carry.
    #[test]
    fn the_captured_0x0073_body_is_handled_as_no_credential() {
        let (mut s, _) = session_with_claim("second_one", "second_one");
        let body = hex_body("050000000000d843ae4c5617b6ae9cd200000000764d00000000");
        assert_eq!(body.len(), 26, "the captured body is 26 bytes");

        let note = describe(CLIENT_SESSION_IDENTITY, &body).expect("0x0073 is described");
        assert!(note.contains("mode=5"), "{note}");
        assert!(note.contains("EMPTY"), "{note}");
        assert!(note.contains("d843ae4c5617b6ae"), "the machine tail must still be logged: {note}");

        assert!(s.handle(&request(CLIENT_SESSION_IDENTITY, &body)).is_empty());
        assert_eq!(s.account_name(), "second_one", "the stock client must be unaffected");
        assert!(!s.take_notes().join("\n").contains("DOWNGRADING"));
    }

    /// A body that does not parse is "no credential", never a refusal: truncating deliberately
    /// must gain an attacker nothing over sending nothing. And it must not panic - it comes
    /// off a socket.
    ///
    /// **The predicate is `SessionIdentity::parse`, not the byte count**, and that distinction
    /// is what the first version of this test got wrong. Chopping the machine tail off leaves a
    /// body that parses perfectly and carries a COMPLETE identity string - so it is a
    /// presentation, and a wrong one, and it is *supposed* to be refused. Only a body that
    /// cannot be read as (mode, string, tail) is "nothing was presented". Asserting on the
    /// length instead of on the parse made a correct refusal look like a bug.
    #[test]
    fn a_malformed_identity_body_is_not_a_refusal_and_does_not_panic() {
        let full = identity_request(5, "AAAAAAAAAAAAAAAAAAAAAAAAAA");
        let mut unparsed = 0;
        let mut parsed = 0;
        // From 2, because a packet shorter than its own opcode never reaches a handler at all -
        // `handle` returns before dispatch, which `a_truncated_packet_does_not_panic` covers.
        for n in 2..full.len() {
            let (mut s, _) = session_with_claim("second_one", "second_one");
            assert!(s.handle(&full[..n]).is_empty(), "truncated to {n} bytes");
            let notes = s.take_notes().join("\n");
            // `full` carries the two opcode bytes; the payload the handler sees starts at 2.
            if SessionIdentity::parse(&full[2..n]).is_none() {
                unparsed += 1;
                assert!(
                    !notes.contains("DOWNGRADING"),
                    "a body that does not parse must not be a refusal ({n} bytes): {notes}"
                );
                assert!(notes.contains("did not parse"), "{n} bytes: {notes}");
                assert_eq!(s.account_name(), "second_one", "{n} bytes");
            } else {
                parsed += 1;
                // It parsed and carried a complete wrong token: a presentation, and refused.
                assert!(notes.contains("DOWNGRADING"), "{n} bytes: {notes}");
            }
        }
        // Both branches have to be exercised, or this test is only checking one of them and
        // saying nothing about the other.
        assert!(unparsed > 0 && parsed > 0, "{unparsed} unparsed, {parsed} parsed");
        // A length prefix that runs past the end of the body.
        let (mut s, _) = session_with_claim("second_one", "second_one");
        let mut lying = 5u32.to_le_bytes().to_vec();
        lying.extend_from_slice(&999u16.to_le_bytes());
        lying.extend_from_slice(b"short");
        assert!(s.handle(&request(CLIENT_SESSION_IDENTITY, &lying)).is_empty());
        assert!(s.take_notes().join("\n").contains("did not parse"));
        assert_eq!(s.account_name(), "second_one");
    }

    /// **One-time use, seen from the login server.** Presenting the token spends it; the same
    /// session presenting it again is the same client process and is accepted as a replay.
    #[test]
    fn the_same_connection_may_present_its_token_twice() {
        let (mut s, client_token) = session_with_claim("maplecw", "second_one");
        s.handle(&identity_request(5, &client_token));
        assert_eq!(s.account_name(), "second_one");
        s.take_notes();

        s.handle(&identity_request(5, &client_token));
        let notes = s.take_notes().join("\n");
        assert!(notes.contains("REPLAY BY THE SAME CLIENT PROCESS"), "{notes}");
        assert_eq!(s.account_name(), "second_one", "the same client must not be downgraded");
    }

    /// A different client process replaying a spent token is downgraded. This is the
    /// one-time-use property where it matters - and the pid it turns on comes from the
    /// operating system, never from the connection.
    #[test]
    fn another_process_replaying_a_spent_token_is_downgraded() {
        let (mut first, client_token) = session_with_claim("maplecw", "second_one");
        first.handle(&identity_request(5, &client_token));
        assert_eq!(first.account_name(), "second_one", "the control: it worked once");

        // A second connection, same store, a DIFFERENT owning process.
        let mut thief = Session::new(
            first.store.clone(),
            first.config.clone(),
            first.store.get_account("maplecw").unwrap().unwrap(),
        )
        .with_fallback(first.store.get_account("maplecw").unwrap().unwrap())
        .with_launch_pid(Some(9999));
        thief.handle(&identity_request(5, &client_token));
        let notes = thief.take_notes().join("\n");
        assert!(notes.contains("SPENT"), "{notes}");
        assert!(notes.contains("DOWNGRADING"), "{notes}");
        // The thief was ALREADY being served the fallback, so nothing on screen changes -
        // and the line has to say that rather than "DOWNGRADING from maplecw to maplecw",
        // which reads as a bug in the log instead of as a refusal.
        assert!(notes.contains("ALREADY being served as"), "{notes}");
        assert_eq!(thief.account_name(), "maplecw");
    }

    /// **E1's own requirement**: every `0x0073`, whatever it carries, produces a log line that
    /// names the length. Without it a failed E2 is indistinguishable from "the server ignored
    /// the field", and the owner would have spent a launch to learn nothing.
    #[test]
    fn every_identity_produces_a_log_line_naming_the_length() {
        for (label, identity) in [
            ("empty", ""),
            ("short", "AB"),
            ("token shaped", "AAAAAAAAAAAAAAAAAAAAAAAAAA"),
            ("long", "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
            ("lower case hex", "0123456789abcdef0123456789abcdef"),
        ] {
            let (mut s, _) = session_with_claim("second_one", "second_one");
            s.handle(&identity_request(5, identity));
            let notes = s.take_notes().join("\n");
            assert!(
                notes.contains(&format!("identity length={}", identity.len())),
                "{label}: {notes}"
            );
            // And the raw decode says the SHAPE, so a hook that writes the wrong thing is
            // legible without a second launch.
            let raw = describe(CLIENT_SESSION_IDENTITY, &identity_request(5, identity)[2..])
                .expect("described");
            assert!(raw.contains("session identity:"), "{label}: {raw}");
            // Grepping login.log for `0x0073` has to find EVERY line about it. This one
            // began with "session" and was silently missed for the empty-identity case,
            // which is the case the whole feature has to be readable in.
            assert!(raw.contains("0x0073"), "{label}: not greppable by opcode: {raw}");
        }
    }

    /// **The unit.** The hook logs `IDENTITY wrote N bytes` and tells the reader in the same
    /// sentence that `login.log should now show ... identity length=N`. That comparison is the
    /// whole readout of the one-launch experiment, so the two numbers have to be the same
    /// unit - **wire bytes**, not decoded characters.
    ///
    /// They agree for anything ASCII, which is why this needs a non-UTF-8 body to mean
    /// anything: `from_utf8_lossy` turns each bad byte into a three-byte replacement
    /// character, so a decoded length would report 6 where the client sent 2 - and the owner would
    /// read that as the hook having written something it did not.
    #[test]
    fn the_logged_length_is_wire_bytes_and_not_decoded_characters() {
        let mut payload = 5u32.to_le_bytes().to_vec();
        payload.extend_from_slice(&2u16.to_le_bytes());
        payload.extend_from_slice(&[0xFF, 0xFE]); // not valid UTF-8
        payload.extend_from_slice(&hex_body("d843ae4c5617b6ae9cd200000000764d00000000"));

        let parsed = SessionIdentity::parse(&payload).expect("it still parses");
        assert_eq!(parsed.identity_bytes, 2, "the client sent two bytes");
        assert_eq!(parsed.identity.len(), 6, "and they decode to two replacement characters");

        let (mut s, _) = session_with_claim("second_one", "second_one");
        s.handle(&request(CLIENT_SESSION_IDENTITY, &payload));
        let notes = s.take_notes().join("\n");
        assert!(notes.contains("identity length=2"), "must report WIRE bytes: {notes}");
        assert!(!notes.contains("identity length=6"), "must not report decoded bytes: {notes}");
        // And the divergence is itself reported, because it means what arrived is not a token.
        assert!(notes.contains("NOT valid UTF-8"), "{notes}");

        // The raw decode uses the same unit.
        let raw = describe(CLIENT_SESSION_IDENTITY, &payload).unwrap();
        assert!(raw.contains("(2 bytes)"), "{raw}");
    }

    /// The shape check has to discriminate, or it is decoration. A real token is recognised;
    /// something the right length that is not base32 is not.
    #[test]
    fn the_raw_decode_tells_a_client_token_from_something_else_the_same_length() {
        let good = describe(CLIENT_SESSION_IDENTITY, &identity_request(5, "MZXW6YTBOIMZXW6YTBOIMZXW6Y")[2..])
            .unwrap();
        assert!(good.contains("THE SHAPE OF A MAPLECW CLIENT TOKEN"), "{good}");

        // 26 characters, but lower case - which is what a hex token would look like, and what
        // an upper-casing transform would NOT produce.
        let bad = describe(CLIENT_SESSION_IDENTITY, &identity_request(5, "mzxw6ytboimzxw6ytboimzxw6y")[2..])
            .unwrap();
        assert!(bad.contains("NOT the shape"), "{bad}");
    }

    /// A session that was never given a fallback must say so rather than silently keeping the
    /// account. `CLAUDE.md`: built is not wired, and an unwired refusal looks identical to no
    /// refusal at all.
    #[test]
    fn a_refusal_with_no_fallback_configured_says_so_loudly() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        store.create_account("maplecw", "correct horse battery").unwrap();
        let account = store.get_account("maplecw").unwrap().unwrap();
        let mut s = Session::new(store, Arc::new(Config::default()), account);
        s.handle(&identity_request(5, "AAAAAAAAAAAAAAAAAAAAAAAAAA"));
        let notes = s.take_notes().join("\n");
        assert!(notes.contains("never given a fallback"), "{notes}");
        assert!(notes.contains("with_fallback"), "the fix has to be in the line: {notes}");
    }

    #[test]
    fn a_truncated_packet_does_not_panic() {
        let mut s = session();
        assert!(s.handle(&[]).is_empty());
        assert!(s.handle(&[0x80]).is_empty());
    }

    #[test]
    fn one_account_never_sees_another_accounts_characters() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        store.create_account("maplecw", "correct horse battery").unwrap();
        store.create_account("someone_else", "correct horse battery").unwrap();
        let config = Arc::new(Config::default());

        let mine = store.get_account("maplecw").unwrap().unwrap();
        let theirs = store.get_account("someone_else").unwrap().unwrap();

        let mut a = Session::new(store.clone(), config.clone(), mine);
        a.handle(&create_request("Hello", 30030, &STYLE));

        let mut b = Session::new(store, config, theirs);
        let replies = b.handle(&request(CLIENT_LOGIN_REQUEST, &[]));
        assert!(replies.last().unwrap().what.contains("0 character"));
    }

    fn delete_request(id: u32) -> Vec<u8> {
        request(CLIENT_DELETE_CHARACTER_REQUEST, &id.to_le_bytes())
    }

    #[test]
    fn deleting_removes_the_character_and_frees_its_name() {
        let mut s = session();
        s.handle(&create_request("Doomed", 30030, &STYLE));
        let id = s.store.characters_for(s.account.id, 0).unwrap()[0].id;

        let replies = s.handle(&delete_request(id));
        assert_eq!(replies.len(), 1);
        assert_eq!(replies[0].opcode, DELETE_CHARACTER_RESULT);
        assert_eq!(replies[0].body, delete_character_result(id, DELETE_OK));

        assert_eq!(s.store.characters_for(s.account.id, 0).unwrap().len(), 0);
        // The slot and the name both come back.
        assert_eq!(s.handle(&name_request("Doomed"))[0].body.last(), Some(&NAME_AVAILABLE));
    }

    #[test]
    fn deleting_frees_a_slot_so_creation_is_possible_again() {
        let mut s = session();
        for name in ["Alpha", "Bravo", "Charlie"] {
            s.handle(&create_request(name, 30030, &STYLE));
        }
        assert_eq!(s.handle(&create_request("Delta", 30030, &STYLE))[0].body,
                   vec![CREATE_INSUFFICIENT_SLOT]);

        let id = s.store.characters_for(s.account.id, 0).unwrap()[0].id;
        s.handle(&delete_request(id));

        let replies = s.handle(&create_request("Delta", 30030, &STYLE));
        assert_eq!(replies[0].body[0], 0, "a freed slot should accept a new character");
    }

    #[test]
    fn a_refusal_never_uses_a_code_that_deletes_anyway() {
        // The trap in the client's switch: every non-zero code it does not name falls
        // through to the branch that removes the character from the list. Only 6 refuses.
        // So a refusal that used, say, 1 would show a delete that did not happen.
        let mut s = session();
        s.handle(&create_request("Safe", 30030, &STYLE));
        let mine = s.store.characters_for(s.account.id, 0).unwrap()[0].id;

        for body in [delete_request(mine + 999), request(CLIENT_DELETE_CHARACTER_REQUEST, &[1, 2])] {
            let replies = s.handle(&body);
            assert_eq!(replies.len(), 1, "a delete must always be answered");
            assert_eq!(
                replies[0].body.last(),
                Some(&DELETE_FAILED),
                "refusals must use DELETE_FAILED, not any non-zero code"
            );
        }
        assert_eq!(s.store.characters_for(s.account.id, 0).unwrap().len(), 1);
    }

    #[test]
    fn one_account_cannot_delete_another_accounts_character() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        store.create_account("maplecw", "correct horse battery").unwrap();
        store.create_account("someone_else", "correct horse battery").unwrap();
        let config = Arc::new(Config::default());
        let mine = store.get_account("maplecw").unwrap().unwrap();
        let theirs = store.get_account("someone_else").unwrap().unwrap();

        let mut owner = Session::new(store.clone(), config.clone(), mine.clone());
        owner.handle(&create_request("Mine", 30030, &STYLE));
        let id = store.characters_for(mine.id, 0).unwrap()[0].id;

        let mut thief = Session::new(store.clone(), config, theirs);
        let replies = thief.handle(&delete_request(id));
        assert_eq!(replies[0].body.last(), Some(&DELETE_FAILED));
        assert_eq!(store.characters_for(mine.id, 0).unwrap().len(), 1);
    }

    #[test]
    fn the_session_identity_is_decoded_for_the_log() {
        // The body measured on the wire: mode 5, an empty identity string, then the
        // 20-byte machine tail.
        let mut payload = 5u32.to_le_bytes().to_vec();
        payload.extend_from_slice(&0u16.to_le_bytes());
        payload.extend_from_slice(&[0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);

        let note = describe(CLIENT_SESSION_IDENTITY, &payload).expect("0x0073 is decoded");
        assert!(note.contains("mode=5"), "{note}");
        assert!(note.contains(r#"identity="""#), "{note}");
        assert!(note.contains("aabbccddeeff"), "{note}");
    }

    #[test]
    fn a_launcher_token_in_the_identity_would_be_visible_in_the_log() {
        // The whole point of decoding it: if the session array at +0x90 ever reaches us,
        // the token shows up as the identity string instead of an empty one.
        let token = "abc123";
        let mut payload = 5u32.to_le_bytes().to_vec();
        payload.extend_from_slice(&(token.len() as u16).to_le_bytes());
        payload.extend_from_slice(token.as_bytes());

        let note = describe(CLIENT_SESSION_IDENTITY, &payload).unwrap();
        assert!(note.contains(r#"identity="abc123""#), "{note}");
    }

    /// **This assertion changed on 2026-08-29 and the change is the point.**
    ///
    /// A truncated `0x0073` used to return `None` - the same answer as a packet this function
    /// does not decode at all. That was fine while the identity was a curiosity. It is not fine
    /// now: `0x0073` carries a credential, and "the client sent a malformed identity" and "the
    /// client sent a packet we do not decode" are different events with different fixes, which
    /// a shared `None` cannot tell apart. Silence on a malformed credential is exactly the
    /// shape `CLAUDE.md` warns about - an instrument that answers the same way for two
    /// different causes.
    ///
    /// So a malformed identity now says MALFORMED, and the "does not panic" half - which is
    /// what this test was really for - is asserted directly.
    #[test]
    fn describe_says_nothing_about_packets_it_does_not_decode() {
        assert!(describe(CLIENT_LOGIN_REQUEST, &[]).is_none());
        // A truncated identity must not panic, and must not be silent either.
        for n in 0..8 {
            let note = describe(CLIENT_SESSION_IDENTITY, &vec![1u8; n])
                .unwrap_or_else(|| panic!("a {n}-byte 0x0073 must still produce a line"));
            assert!(note.contains("MALFORMED"), "{n} bytes: {note}");
        }
    }

    /// The select-character body, as the client sends it: a leading `u32`, the PIC as a
    /// length-prefixed string, then the id.
    fn select_payload(character_id: u32) -> Vec<u8> {
        let mut body = 0u32.to_le_bytes().to_vec();
        body.extend_from_slice(&1u16.to_le_bytes());
        body.push(b'.');
        body.extend_from_slice(&character_id.to_le_bytes());
        body.extend_from_slice(&[0]);
        body
    }

    #[test]
    fn selecting_a_character_migrates_it_to_the_advertise_address() {
        let mut s = session();
        s.handle(&create_request("Wanderer", 30030, &STYLE));
        let id = s.store.characters_for(s.account.id, 0).unwrap()[0].id;

        let replies = s.handle(&request(CLIENT_SELECT_CHARACTER_REQUEST, &select_payload(id)));
        assert_eq!(replies.len(), 1);
        assert_eq!(replies[0].opcode, MIGRATE_COMMAND);

        // The seed is minted in the store, so read it back out of the packet and check the
        // channel server would be able to claim exactly that migration.
        let body = &replies[0].body;
        let key = u32::from_le_bytes(body[47..51].try_into().unwrap());
        let raw = u32::from_le_bytes(body[55..59].try_into().unwrap());
        let seed = {
            let t = (key ^ raw).wrapping_add(0x369F_144D).wrapping_add(key >> 7);
            t ^ 0xAAAA_BBBBu32
        };
        assert_eq!(*body, migrate(s.config.world.channel_address(0).unwrap(), id, seed));
        let claimed = s.store.claim_migration(seed).unwrap().expect("the seed was minted");
        assert_eq!(claimed.character_id, id);
    }

    /// **The host is decided per connection under `--advertise auto`.** The listed channel is
    /// `127.0.0.1:8485`; a LAN client that reached this server at `192.168.1.20` is told
    /// `192.168.1.20:8485`, because loopback would send it back to itself. `net::advertise`.
    #[test]
    fn a_lan_client_is_migrated_to_the_address_it_reached_the_server_on() {
        let base = session();
        let mut s = Session::new(base.store.clone(), base.config.clone(), base.account.clone())
            .with_peer("192.168.1.77")
            .with_local_addr("192.168.1.20:8484".parse().unwrap());
        s.handle(&create_request("Wanderer", 30030, &STYLE));
        let id = s.store.characters_for(s.account.id, 0).unwrap()[0].id;

        let replies = s.handle(&request(CLIENT_SELECT_CHARACTER_REQUEST, &select_payload(id)));
        let seed = seed_from(&replies[0].body);
        assert_eq!(replies[0].body, migrate("192.168.1.20:8485".parse().unwrap(), id, seed));
        assert!(replies[0].what.contains("Advertised as 192.168.1.20"), "{}", replies[0].what);
    }

    /// `--advertise <ip>` overrides everything, a loopback peer included.
    #[test]
    fn a_fixed_advertise_address_is_used_for_every_client() {
        let base = session();
        let fixed = net::advertise::Mode::Fixed("203.0.113.9".parse().unwrap());
        let config = Config {
            advertise: Arc::new(net::advertise::Advertiser::new(fixed)),
            ..(*base.config).clone()
        };
        let mut s = Session::new(base.store.clone(), Arc::new(config), base.account.clone())
            .with_peer("127.0.0.1")
            .with_local_addr("127.0.0.1:8484".parse().unwrap());
        s.handle(&create_request("Wanderer", 30030, &STYLE));
        let id = s.store.characters_for(s.account.id, 0).unwrap()[0].id;

        let replies = s.handle(&request(CLIENT_SELECT_CHARACTER_REQUEST, &select_payload(id)));
        let seed = seed_from(&replies[0].body);
        assert_eq!(replies[0].body, migrate("203.0.113.9:8485".parse().unwrap(), id, seed));
    }

    /// The seed the client was handed, dug back out of a `MIGRATE_COMMAND` body.
    fn seed_from(body: &[u8]) -> u32 {
        let key = u32::from_le_bytes(body[47..51].try_into().unwrap());
        let raw = u32::from_le_bytes(body[55..59].try_into().unwrap());
        let t = (key ^ raw).wrapping_add(0x369F_144D).wrapping_add(key >> 7);
        t ^ 0xAAAA_BBBBu32
    }

    /// Select a character and return the seed the client was told to migrate with.
    fn migrate_and_get_seed(s: &mut Session, id: u32) -> u32 {
        let replies = s.handle(&request(CLIENT_SELECT_CHARACTER_REQUEST, &select_payload(id)));
        assert_eq!(replies[0].opcode, MIGRATE_COMMAND);
        seed_from(&replies[0].body)
    }

    /// **The fix, end to end on the login side.** With a launcher sign-in staked, the
    /// migration is bound to that sign-in's token, and a channel connection presenting
    /// nothing cannot claim it - which is what the world server does today.
    /// A session with migration binding switched on.
    fn binding_session() -> Session {
        let base = session();
        let config = Config { bind_migrations: true, ..(*base.config).clone() };
        Session::new(base.store.clone(), Arc::new(config), base.account.clone())
    }

    #[test]
    fn a_migration_minted_under_a_live_claim_is_bound_to_it() {
        let mut s = binding_session();
        s.handle(&create_request("Wanderer", 30030, &STYLE));
        let id = s.store.characters_for(s.account.id, 0).unwrap()[0].id;

        let token = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        s.store.stake_login_claim(s.account.id, token, store::LOGIN_CLAIM_TTL_SECS).unwrap();
        // What `server::resolve_account` does once per connection: decide which claim this
        // connection is, and hand the session that claim's token hash.
        s = s.with_claim_token_hash(Some(store::hash_token(token)));

        let seed = migrate_and_get_seed(&mut s, id);

        // Presenting nothing is refused, not served.
        assert_eq!(
            s.store
                .claim_migration_with(
                    seed,
                    &store::migration::MigrationEvidence::none(),
                    store::migration::PeerPolicy::Record,
                )
                .unwrap(),
            store::migration::ClaimOutcome::Refused(store::migration::Refusal::TokenMissing),
            "a bound migration must not be claimable by a connection that proves nothing"
        );
        // And the real launcher token still gets the character in.
        assert!(s
            .store
            .claim_migration_with(
                seed,
                &store::migration::MigrationEvidence::with_token(token),
                store::migration::PeerPolicy::Record,
            )
            .unwrap()
            .migration()
            .is_some());
    }

    /// **The default is the old behaviour, unchanged.** A live claim with the switch off
    /// still mints an unbound migration - because a bound one would be refused by today's
    /// channel server and the player would simply never enter the world.
    #[test]
    fn binding_is_off_by_default_even_with_a_live_claim() {
        let mut s = session();
        s.handle(&create_request("Wanderer", 30030, &STYLE));
        let id = s.store.characters_for(s.account.id, 0).unwrap()[0].id;
        s.store.stake_login_claim(s.account.id, "abc", 3600).unwrap();

        let seed = migrate_and_get_seed(&mut s, id);
        assert!(
            !s.store.migration_binding(seed).unwrap().unwrap().token_bound,
            "binding on by default would refuse every migration on today's channel server"
        );
        assert!(
            s.store.claim_migration(seed).unwrap().is_some(),
            "the credential-less channel server must still be able to claim it"
        );
    }

    /// With no launcher sign-in there is nothing to bind to. The migration is minted
    /// **unbound** and behaves exactly as it always did - the honest degradation, and the
    /// reason the reply's log line says which of the two happened.
    #[test]
    fn a_migration_minted_with_no_claim_is_unbound_and_says_so() {
        let mut s = binding_session();
        s.handle(&create_request("Wanderer", 30030, &STYLE));
        let id = s.store.characters_for(s.account.id, 0).unwrap()[0].id;

        let replies = s.handle(&request(CLIENT_SELECT_CHARACTER_REQUEST, &select_payload(id)));
        assert!(
            replies[0].what.contains("UNBOUND"),
            "an unbound migration is the impersonatable case and the log must name it: {}",
            replies[0].what
        );
        assert!(s.store.claim_migration(seed_from(&replies[0].body)).unwrap().is_some());
    }

    /// The reply must still be a `MIGRATE_COMMAND` in every branch. An unanswered
    /// select-character freezes the client's entire UI, including the quit prompt, so no
    /// binding failure may turn into silence.
    #[test]
    fn every_binding_outcome_still_answers_the_client() {
        for stake in [false, true] {
            let mut s = session();
            s.handle(&create_request("Wanderer", 30030, &STYLE));
            let id = s.store.characters_for(s.account.id, 0).unwrap()[0].id;
            if stake {
                s.store.stake_login_claim(s.account.id, "abc", 3600).unwrap();
            }
            let replies = s.handle(&request(CLIENT_SELECT_CHARACTER_REQUEST, &select_payload(id)));
            assert_eq!(replies.len(), 1, "stake={stake}");
            assert_eq!(replies[0].opcode, MIGRATE_COMMAND, "stake={stake}");
        }
    }

    /// The address is recorded when the socket had one, and its absence is not an error.
    #[test]
    fn the_peer_address_is_recorded_on_the_migration_when_there_is_one() {
        let base = session();
        let mut s = Session::new(base.store.clone(), base.config.clone(), base.account.clone())
            .with_peer("203.0.113.7");
        s.handle(&create_request("Wanderer", 30030, &STYLE));
        let id = s.store.characters_for(s.account.id, 0).unwrap()[0].id;
        let seed = migrate_and_get_seed(&mut s, id);

        let binding = s.store.migration_binding(seed).unwrap().expect("the row was minted");
        assert_eq!(binding.peer.as_deref(), Some("203.0.113.7"));
        assert!(!binding.token_bound, "no claim was staked, so nothing to bind to");
    }

    /// A world that advertises a channel it has no address for would send the client
    /// nowhere, so the login server refuses rather than building a packet with a hole.
    #[test]
    fn a_channel_with_no_address_is_refused_rather_than_migrated_to() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        store.create_account("maplecw", "correct horse battery").unwrap();
        let account = store.get_account("maplecw").unwrap().unwrap();
        let world = World { channel_id: 3, ..World::default() };
        let mut s = Session::new(store, Arc::new(Config { world, ..Config::default() }), account);
        s.handle(&create_request("Wanderer", 30030, &STYLE));
        let id = s.store.characters_for(s.account.id, 0).unwrap()[0].id;

        let replies = s.handle(&request(CLIENT_SELECT_CHARACTER_REQUEST, &select_payload(id)));
        assert_eq!(replies[0].body[0], net::opcode::MIGRATE_REFUSED);
        assert!(replies[0].what.contains("no address for channel 3"), "{}", replies[0].what);
    }

    /// A migration is single use: the channel claims it once, and a replay gets nothing.
    #[test]
    fn a_migration_cannot_be_claimed_twice() {
        let mut s = session();
        s.handle(&create_request("Wanderer", 30030, &STYLE));
        let id = s.store.characters_for(s.account.id, 0).unwrap()[0].id;
        let body = s
            .handle(&request(CLIENT_SELECT_CHARACTER_REQUEST, &select_payload(id)))
            .remove(0)
            .body;
        let key = u32::from_le_bytes(body[47..51].try_into().unwrap());
        let raw = u32::from_le_bytes(body[55..59].try_into().unwrap());
        let seed = {
            let t = (key ^ raw).wrapping_add(0x369F_144D).wrapping_add(key >> 7);
            t ^ 0xAAAA_BBBBu32
        };
        assert!(s.store.claim_migration(seed).unwrap().is_some());
        assert!(s.store.claim_migration(seed).unwrap().is_none());
    }

    /// The address in the packet is the one the *client* must reach, and it is the
    /// **channel's**, not this server's. Getting it backwards sends the client to the
    /// login server, which is what produced "The client is outdated" on 2026-08-19.
    #[test]
    fn the_migration_carries_the_channel_address_rather_than_bind() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        store.create_account("maplecw", "correct horse battery").unwrap();
        let account = store.get_account("maplecw").unwrap().unwrap();
        let config = Config {
            bind: "0.0.0.0:8484".parse().unwrap(),
            world: World {
                channels: vec!["192.168.1.50:8485".parse().unwrap()],
                ..World::default()
            },
            ..Config::default()
        };
        let mut s = Session::new(store, Arc::new(config), account);
        s.handle(&create_request("Wanderer", 30030, &STYLE));
        let id = s.store.characters_for(s.account.id, 0).unwrap()[0].id;

        let body = s.handle(&request(CLIENT_SELECT_CHARACTER_REQUEST, &select_payload(id)))
            .remove(0)
            .body;
        assert_eq!(&body[4..8], &[192, 168, 1, 50]);
        assert_eq!(&body[8..10], &8485u16.to_le_bytes(), "the channel's port, not login's");
    }

    /// An id the login result never sent makes the client skip its whole action block in
    /// silence, so the server has to be the one that says something.
    #[test]
    fn selecting_a_character_that_is_not_on_the_account_is_refused_out_loud() {
        let mut s = session();
        s.handle(&create_request("Wanderer", 30030, &STYLE));

        let replies = s.handle(&request(CLIENT_SELECT_CHARACTER_REQUEST, &select_payload(9999)));
        assert_eq!(replies.len(), 1, "a refusal is still an answer");
        assert_eq!(replies[0].opcode, MIGRATE_COMMAND);
        assert_eq!(replies[0].body[0], net::opcode::MIGRATE_REFUSED);
        assert!(replies[0].what.contains("not on this account"), "{}", replies[0].what);
    }

    #[test]
    fn one_account_cannot_migrate_into_another_accounts_character() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        store.create_account("otter", "correct horse battery").unwrap();
        store.create_account("owl", "correct horse battery").unwrap();
        let otter = store.get_account("otter").unwrap().unwrap();
        let owl = store.get_account("owl").unwrap().unwrap();

        let config = Arc::new(Config::default());
        let mut a = Session::new(store.clone(), config.clone(), otter);
        a.handle(&create_request("AlicesChar", 30030, &STYLE));
        let id = a.store.characters_for(a.account.id, 0).unwrap()[0].id;

        let mut b = Session::new(store, config, owl);
        let replies = b.handle(&request(CLIENT_SELECT_CHARACTER_REQUEST, &select_payload(id)));
        assert_eq!(replies[0].body[0], net::opcode::MIGRATE_REFUSED);
    }

    /// The refusal must carry the byte the handler reads before it looks at the code, or
    /// the client faults instead of showing the dialog.
    #[test]
    fn a_migration_refusal_is_long_enough_for_the_reads_before_the_gate() {
        let mut s = session();
        let body = s.handle(&request(CLIENT_SELECT_CHARACTER_REQUEST, &select_payload(1)))
            .remove(0)
            .body;
        let message_len = u16::from_le_bytes([body[1], body[2]]) as usize;
        assert_eq!(body.len(), 1 + 2 + message_len + 1);
    }

    #[test]
    fn a_reply_packet_is_the_opcode_then_the_body() {
        let r = Reply::new(0x0015, vec![0xAA, 0xBB], "x");
        assert_eq!(r.packet(), vec![0x15, 0x00, 0xAA, 0xBB]);
    }
}
