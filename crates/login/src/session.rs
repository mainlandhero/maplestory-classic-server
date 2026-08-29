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
}

impl Reply {
    fn new(opcode: u16, body: Vec<u8>, what: impl Into<String>) -> Self {
        Reply { opcode, body, what: what.into() }
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
}

impl Session {
    pub fn new(store: Arc<Store>, config: Arc<Config>, account: Account) -> Self {
        Session { store, config, account, seen_login_request: false }
    }

    pub fn account_name(&self) -> &str {
        &self.account.name
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
        out.push(Reply::new(
            LOGIN_RESULT,
            login_result(world.id, channel, &characters),
            format!(
                "{cause}: login result, {} character(s): {}; {channel_note}",
                characters.len(),
                names.join(", ")
            ),
        ));
        out
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
        let Some(addr) = world.channel_address(channel) else {
            return refuse(format!(
                "REFUSED - world {} has no address for channel {channel}; the client would be sent nowhere",
                world.id
            ));
        };

        // The seed is minted here and claimed by the channel server out of the same
        // database. It is a u32 - all the packet has room for - so it identifies a pending
        // migration rather than proving anything. What it does buy is single use.
        let seed = match self.store.create_migration(self.account.id, id, world.id, channel) {
            Ok(seed) => seed,
            Err(e) => return refuse(format!("REFUSED - could not mint a migration: {e}")),
        };

        vec![Reply::new(
            MIGRATE_COMMAND,
            migrate(addr, id, seed),
            format!(
                "migrate {:?} (id {id}) to world {} channel {channel} at {addr}, seed {seed:#010x} - single use, NOT authentication",
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
/// request. Nothing here answers it - it is decoded only so the log can show what the
/// client thinks its identity is, which is the open question for multi-account support.
const CLIENT_SESSION_IDENTITY: u16 = 0x0073;

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
/// It is the only thing the client sends that could carry an account identity, and
/// **whether it can is unmeasured**. `-NXLDEBUG` routes launch arguments 3 onward into the
/// client config's six-slot session array at `+0x90`; if those arrive here, a launcher can
/// pass a single-use token and the server can stop serving every connection as one
/// configured account. If they do not, multi-account needs a different route entirely.
///
/// Decoding it costs nothing and makes any launch answer the question, rather than
/// spending a launch on it later. Measured so far: a first `u32` of `5` (the launch mode),
/// then a **zero-length** identity string, then a constant 20-byte tail that is a MAC
/// address and a machine id - recorded, never authorised on, since the client machine is
/// not fixed.
pub fn describe(opcode: u16, payload: &[u8]) -> Option<String> {
    if opcode == CLIENT_MIGRATION_HELLO {
        return Some(format!(
            "MIGRATION HELLO: the client reconnected after 0x0011 and sent {} bytes. The migration seed is in here, obfuscated with the u32 before its length -              layout not yet decoded, read the body hex. See docs/opcodes.md.",
            payload.len()
        ));
    }
    if opcode != CLIENT_SESSION_IDENTITY {
        return None;
    }
    let mode = payload
        .get(..4)
        .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))?;
    let identity = read_str(&payload[4..]).unwrap_or_default();
    let tail_at = 4 + 2 + identity.len();
    let tail = payload.get(tail_at..).unwrap_or(&[]);
    Some(format!(
        "session identity: mode={mode} identity={identity:?} ({} bytes) tail={} \
         - NOT used to pick the account; see docs/login-server.md",
        identity.len(),
        tail.iter().map(|b| format!("{b:02x}")).collect::<String>()
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

    #[test]
    fn describe_says_nothing_about_packets_it_does_not_decode() {
        assert!(describe(CLIENT_LOGIN_REQUEST, &[]).is_none());
        // And a truncated identity must not panic.
        assert!(describe(CLIENT_SESSION_IDENTITY, &[1, 2]).is_none());
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
