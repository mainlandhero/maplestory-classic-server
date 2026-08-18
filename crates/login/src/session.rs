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
    world_list_end, world_list_entry, Character, CreateCharacterRequest, ACCOUNT_INFO, CHARACTER_SLOTS,
    CHECK_NAME_RESULT, CLIENT_CHECK_NAME_REQUEST, CLIENT_CREATE_CHARACTER_REQUEST,
    CLIENT_DATA_WZ_REQUEST, CLIENT_DELETE_CHARACTER_REQUEST, CLIENT_ENTER_CREATION_REQUEST,
    CLIENT_LEAVE_WORLD_REQUEST, CLIENT_LOGIN_REQUEST, CREATE_CANNOT_PROCESS,
    CREATE_CHARACTER_RESULT, CREATE_INSUFFICIENT_SLOT, DATA_WZ_PATCH,
    DELETE_CHARACTER_RESULT, DELETE_FAILED, DELETE_OK, ENTER_CREATION_RESULT,
    LOGIN_RESULT, NAME_ALREADY_USED, NAME_AVAILABLE, NAME_NOT_ALLOWED, WORLD_LIST,
};
use store::{Account, NameCheck, Store};

use crate::config::Config;

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
            CLIENT_LEAVE_WORLD_REQUEST => self.world_and_characters("leave world"),
            CLIENT_ENTER_CREATION_REQUEST => vec![Reply::new(
                ENTER_CREATION_RESULT,
                enter_creation_permitted(),
                "creation screen permitted",
            )],
            CLIENT_CHECK_NAME_REQUEST => self.check_name(payload),
            CLIENT_CREATE_CHARACTER_REQUEST => self.create_character(payload),
            CLIENT_DELETE_CHARACTER_REQUEST => self.delete_character(payload),
            _ => Vec::new(),
        }
    }

    /// The account name, the world, and the character list - in that order.
    ///
    /// Order is not cosmetic. The account name is drawn on the login screen the client is
    /// still showing; the world entry is what enables the Login button *and* fills the list
    /// the login result searches; the login result names a world that must already be in
    /// that list or the client stops without saying why.
    fn world_and_characters(&mut self, cause: &str) -> Vec<Reply> {
        let world = &self.config.world;
        let mut out = vec![
            Reply::new(
                ACCOUNT_INFO,
                account_info(&self.account.name, &self.config.display_name),
                format!("{cause}: account info"),
            ),
            Reply::new(
                WORLD_LIST,
                world_list_entry(world.id as u8, &world.name, world.channels),
                format!("{cause}: world {}", world.name),
            ),
            Reply::new(WORLD_LIST, world_list_end(), format!("{cause}: end of worlds")),
        ];

        let characters = match self.store.characters_for(self.account.id, world.id) {
            Ok(c) => c,
            Err(e) => {
                // Send an empty list rather than nothing. A character select screen with
                // no characters is recoverable; a client blocked on a reply is not.
                out.push(Reply::new(
                    LOGIN_RESULT,
                    login_result(world.id, world.channel_id, &[]),
                    format!("{cause}: EMPTY LIST - could not read characters: {e}"),
                ));
                return out;
            }
        };

        let names: Vec<&str> = characters.iter().map(|c| c.name.as_str()).collect();
        out.push(Reply::new(
            LOGIN_RESULT,
            login_result(world.id, world.channel_id, &characters),
            format!("{cause}: login result, {} character(s): {}", characters.len(), names.join(", ")),
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

    fn session() -> Session {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let id = store.create_account("maplecw", "correct horse battery").unwrap();
        let account = store.get_account("maplecw").unwrap().unwrap();
        assert_eq!(account.id, id);
        Session::new(store, Arc::new(Config::default()), account)
    }

    fn opcodes(replies: &[Reply]) -> Vec<u16> {
        replies.iter().map(|r| r.opcode).collect()
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
        assert_eq!(opcodes(&first), vec![ACCOUNT_INFO, WORLD_LIST, WORLD_LIST, LOGIN_RESULT]);
        assert_eq!(first.iter().map(|r| &r.body).collect::<Vec<_>>(),
                   second.iter().map(|r| &r.body).collect::<Vec<_>>());
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

    #[test]
    fn a_reply_packet_is_the_opcode_then_the_body() {
        let r = Reply::new(0x0015, vec![0xAA, 0xBB], "x");
        assert_eq!(r.packet(), vec![0x15, 0x00, 0xAA, 0xBB]);
    }
}
