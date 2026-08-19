//! One channel connection's protocol state, as a pure state machine.
//!
//! Bodies in, bodies out - no socket, no cipher, no clock, for the same reason
//! `login::session` is built that way: a client launch costs the owner a manual elevated run,
//! so anything that can be a unit test has to be one.
//!
//! # This stage is not decoded yet
//!
//! Everything the client sends on a channel connection is new. The only packet with a name
//! is `0x007D`, and that name was read out of the client rather than captured: the builder
//! is `FUN_1415d10e0`, and it writes the `u32` the migration packet handed over. So the
//! first job here is to log what arrives and pick the seed out of `0x007D` - which is also
//! the check on the whole `0x0011` decode.
//!
//! **Nothing here answers anything yet, deliberately.** An unanswered request freezes the
//! client's UI, so this is not a state to stay in - but answering a packet whose meaning is
//! unknown is worse than not answering, because a wrong reply moves the client into a state
//! nobody has read. Log first, then decode, then answer.

use std::sync::Arc;

use store::{ClaimedMigration, Store};

use crate::config::Config;

/// One packet to send, plus what it is - the label goes in the log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reply {
    pub opcode: u16,
    pub body: Vec<u8>,
    pub what: String,
}

impl Reply {
    /// Opcode then body - the packet as the framer wants it.
    pub fn packet(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(2 + self.body.len());
        out.extend_from_slice(&self.opcode.to_le_bytes());
        out.extend_from_slice(&self.body);
        out
    }
}

/// The client's hello on a channel connection, built by `FUN_1415d10e0`.
///
/// **It carries the character id, not the migration seed.** The seed was the design's
/// assumption and the capture disproved it - see [`migration_hello_character`] for the
/// measured layout. The bytes in front of the id are still undecoded: `FUN_1415d10e0`
/// writes two `u32` before it, and the tail is the same MAC and machine id `0x0073`
/// carries.
pub const CLIENT_MIGRATION_HELLO: u16 = 0x007D;

/// One channel connection.
pub struct Session {
    store: Arc<Store>,
    config: Arc<Config>,
    /// The migration this connection claimed, once it has claimed one.
    claimed: Option<ClaimedMigration>,
}

impl Session {
    pub fn new(store: Arc<Store>, config: Arc<Config>) -> Self {
        Session { store, config, claimed: None }
    }

    /// What the channel sends the moment the client connects: **nothing**.
    ///
    /// The login server sends the `0x0032` startup gate here, unprompted. That is right
    /// there and wrong here - it releases the login connection's startup loop, and this
    /// connection has no startup loop. It is also the most likely reason the first
    /// migrated connection was rejected with "The client is outdated".
    pub fn on_connect(&mut self) -> Vec<Reply> {
        Vec::new()
    }

    /// Handle one packet body, opcode included.
    ///
    /// Answers nothing unless [`Config::set_field_probe`] is on, and then exactly one
    /// packet: a `SetField` carrying the claimed character's record. A wrong reply moves
    /// the client into a state nobody has read, which is worse than silence - so this
    /// answers only the migration hello, and only the one opcode that is confirmed.
    pub fn handle(&mut self, body: &[u8]) -> Vec<Reply> {
        let opcode = match body.get(..2) {
            Some(b) => u16::from_le_bytes([b[0], b[1]]),
            None => return Vec::new(),
        };
        if opcode != CLIENT_MIGRATION_HELLO || !self.config.set_field_probe {
            return Vec::new();
        }
        // Always answer. An unanswered packet freezes the client's whole UI - every
        // button, including the quit prompt - and reads on screen as a crash. So a
        // character we cannot load falls back to the minimal record rather than silence.
        let (body, what) = match self.claimed_character() {
            Some(chr) => (
                net::opcode::set_field_with_character(
                    &chr,
                    self.config.world_id,
                    self.clock_base(),
                    self.config.channel_id,
                ),
                format!(
                    "SetField, characterData=1, presence[0] set so the character-stat block                      decodes, carrying map {} for character {} ({}). presence[0] is gate                      entry 7, settled in research/charrecord-presence-map.md; the map id                      sits at stat-block offset {}, settled in research/charstat-layout.md.                      Nothing here authenticates anybody.",
                    chr.map_id,
                    chr.id,
                    chr.name,
                    net::opcode::stat_block_map_id_at(chr.job),
                ),
            ),
            None => (
                net::opcode::set_field_minimal(self.clock_base(), self.config.channel_id),
                "SetField, characterData=1, MINIMAL record - the character could not be                  loaded, so this falls back to the all-flags-clear form. It is answered                  rather than dropped because an unanswered packet freezes the client's                  whole UI. It will NOT put the character on a map: with every presence                  flag clear the stat block never decodes, so there is no map id at all."
                    .to_string(),
            ),
        };
        vec![Reply { opcode: net::opcode::SET_FIELD, body, what }]
    }

    /// The character this connection claimed a migration for.
    ///
    /// `describe_hello` in `server.rs` claims the migration before `handle` runs, so by
    /// this point `claimed` is populated for a well-formed hello. The store has no
    /// lookup by character id alone, but a claim carries the account and world, and a
    /// character id is unique within those.
    fn claimed_character(&self) -> Option<net::opcode::Character> {
        let claimed = self.claimed.as_ref()?;
        self.store
            .characters_for(claimed.account_id, claimed.world_id)
            .ok()?
            .into_iter()
            .find(|c| c.id == claimed.character_id)
    }

    /// The 8 bytes the client stores as a server clock base, stamping its own tick beside
    /// them. A Windows `FILETIME` is the shape the reference server sends; nothing has been
    /// measured about what this client does with the value, so a plausible one is sent
    /// rather than zero.
    fn clock_base(&self) -> u64 {
        const FILETIME_1970: u64 = 116_444_736_000_000_000;
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        FILETIME_1970 + secs * 10_000_000
    }

    /// Claim the pending migration for a character, and say what happened.
    ///
    /// **The seed does not come back.** It was the design's assumption that the `u32` handed
    /// over in `0x0011` would return in `0x007D`; the capture says otherwise - it is absent
    /// from the body both plainly and under the obfuscated-block search - and what the
    /// client sends instead is the character id.
    ///
    /// So single use is carried entirely by the database row, which is where it always
    /// actually lived: a `u32` on the wire was never a secret, and this only removes the
    /// pretence that it was. Nothing here authenticates anybody.
    pub fn claim_for_character(&mut self, character_id: u32) -> String {
        match self.store.claim_migration_for_character(character_id) {
            Ok(Some(claimed)) => {
                let wrong_channel = claimed.world_id != self.config.world_id
                    || claimed.channel_id != self.config.channel_id;
                let note = format!(
                    "claimed the migration for character {} of account {} \
                     (world {} channel {})",
                    claimed.character_id, claimed.account_id, claimed.world_id, claimed.channel_id
                );
                self.claimed = Some(claimed);
                if wrong_channel {
                    format!(
                        "{note} - WRONG CHANNEL: this is world {} channel {}",
                        self.config.world_id, self.config.channel_id
                    )
                } else {
                    note
                }
            }
            Ok(None) => format!(
                "character {character_id} has no unconsumed migration - it was never \
                 minted, or already claimed, or it expired"
            ),
            Err(e) => format!("character {character_id} could not be checked: {e}"),
        }
    }

    /// The migration this connection claimed, if any.
    pub fn claimed(&self) -> Option<&ClaimedMigration> {
        self.claimed.as_ref()
    }
}

/// Where the character id sits in a `0x007D` body.
///
/// **Measured from a real capture, 2026-08-19**, decrypted with AES once the channel's
/// cipher was settled:
///
/// ```text
/// u32  0
/// u32  0
/// u32  characterId      <- 204, TestCharD
/// u8[6] MAC
/// u32  machine id
/// ...                    the same trailing identity block 0x0073 carries
/// ```
const HELLO_CHARACTER_AT: usize = 8;

/// The character id out of a `0x007D` body, or `None` if it is too short to hold one.
pub fn migration_hello_character(payload: &[u8]) -> Option<u32> {
    payload
        .get(HELLO_CHARACTER_AT..HELLO_CHARACTER_AT + 4)
        .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session() -> (Session, Arc<Store>, i64, u32) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Wanderer".to_string(), ..Default::default() };
        let id = store.create_character(account_id, 0, &chr).unwrap().id;
        let s = Session::new(store.clone(), Arc::new(Config::default()));
        (s, store, account_id, id)
    }

    /// The channel must not send the login server's startup gate. That packet is what a
    /// login connection needs and what a game connection almost certainly rejected.
    #[test]
    fn a_channel_says_nothing_on_connect() {
        let (mut s, _, _, _) = session();
        assert!(s.on_connect().is_empty());
    }

    #[test]
    fn a_pending_migration_is_claimed_once_and_remembered() {
        let (mut s, store, account_id, id) = session();
        store.create_migration(account_id, id, 0, 0).unwrap();

        let note = s.claim_for_character(id);
        assert!(note.contains("claimed the migration"), "{note}");
        assert!(!note.contains("WRONG CHANNEL"), "{note}");
        assert_eq!(s.claimed().unwrap().character_id, id);

        let mut other = Session::new(store, Arc::new(Config::default()));
        assert!(other.claim_for_character(id).contains("no unconsumed migration"));
    }

    #[test]
    fn a_migration_for_another_channel_is_reported_rather_than_silently_accepted() {
        let (_, store, account_id, id) = session();
        store.create_migration(account_id, id, 0, 7).unwrap();
        let config = Config { channel_id: 0, ..Config::default() };
        let mut s = Session::new(store, Arc::new(config));
        assert!(s.claim_for_character(id).contains("WRONG CHANNEL"));
    }

    #[test]
    fn a_character_with_no_migration_says_so_instead_of_failing() {
        let (mut s, _, _, _) = session();
        assert!(s.claim_for_character(999).contains("no unconsumed migration"));
    }

    /// The offset came from a real capture; this is that capture.
    #[test]
    fn the_character_id_is_read_out_of_a_real_0x007d_body() {
        let body = hex(
            "0000000000000000cc000000aabbccddeeffdeadbeef00000000764d0000230000\
             00020000007d29595a7929595a3fc073dd1e000000",
        );
        assert_eq!(migration_hello_character(&body), Some(204));
    }

    #[test]
    fn a_short_hello_yields_no_character_rather_than_panicking() {
        for len in 0..12 {
            migration_hello_character(&vec![0u8; len]);
        }
        assert_eq!(migration_hello_character(&[0u8; 11]), None);
        assert_eq!(migration_hello_character(&[0u8; 12]), Some(0));
    }

    fn hex(s: &str) -> Vec<u8> {
        let clean: String = s.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        (0..clean.len() / 2)
            .map(|i| u8::from_str_radix(&clean[i * 2..i * 2 + 2], 16).unwrap())
            .collect()
    }
}
