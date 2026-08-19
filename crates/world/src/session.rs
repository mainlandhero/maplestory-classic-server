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
/// It carries the migration seed, obfuscated with the `u32` immediately before its length,
/// using the same scheme the migration packet's tail uses in the other direction. The
/// prefix in front of that block is **not decoded**: `FUN_1415d10e0` writes 16 bytes, two
/// `u8`, a `u32` and the launch mode from `session+0x68` before it gets there, and that has
/// not been read carefully enough to index.
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
    pub fn handle(&mut self, _body: &[u8]) -> Vec<Reply> {
        Vec::new()
    }

    /// Claim the migration a seed refers to, and say what happened.
    ///
    /// Called by the server once it has recovered a seed from `0x007D`. Kept separate from
    /// [`Session::handle`] because the seed's position in that packet is not known yet, so
    /// the recovery is a search rather than a parse - and a search belongs where it can be
    /// logged, not buried in the state machine.
    pub fn claim(&mut self, seed: u32) -> String {
        match self.store.claim_migration(seed) {
            Ok(Some(claimed)) => {
                let wrong_channel = claimed.world_id != self.config.world_id
                    || claimed.channel_id != self.config.channel_id;
                let note = format!(
                    "claimed migration {seed:#010x}: character {} of account {} \
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
                "seed {seed:#010x} matches no unconsumed migration - either it is not the \
                 seed, or it was already claimed, or it expired"
            ),
            Err(e) => format!("seed {seed:#010x} could not be checked: {e}"),
        }
    }

    /// The migration this connection claimed, if any.
    pub fn claimed(&self) -> Option<&ClaimedMigration> {
        self.claimed.as_ref()
    }
}

/// Undo one aligned word of the client's obfuscation, in the direction the *client*
/// applies when it writes.
///
/// The migration packet's tail and `0x007D`'s payload use the same arithmetic, so this is
/// the forward transform from `crates/net`'s builder, restated here because the search
/// below needs it. Kept as a free function so it is testable on its own.
pub fn deobfuscate_word(raw: u32, key: u32, offset: u32) -> u32 {
    let t = (key ^ raw).wrapping_add(0x369F_144D).wrapping_add(key >> 7);
    (t ^ 0xAAAA_BBBB).wrapping_sub(offset.wrapping_mul(key))
}

/// Look for a migration seed in a `0x007D` body.
///
/// The layout of the prefix is unknown, so this walks every position where a
/// `(key, length, word)` triple could start and reports the candidates. It is a **search,
/// not a parse**, and it is honest about that: it returns every position that decodes to a
/// plausible seed rather than claiming to know where the field is.
///
/// A candidate is a position where the `u32` at `at + 4` is a length of at least 4 that
/// fits in the remaining body. That is exactly the constraint `FUN_1406e8460` enforces on
/// the read side, so it rules out most of the body without assuming anything else.
pub fn seed_candidates(body: &[u8]) -> Vec<(usize, u32)> {
    let word = |at: usize| -> Option<u32> {
        body.get(at..at + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    };
    let mut out = Vec::new();
    for at in 0..body.len().saturating_sub(11) {
        let (Some(key), Some(len), Some(raw)) = (word(at), word(at + 4), word(at + 8)) else {
            continue;
        };
        let after = at + 8;
        if len < 4 || len as usize > body.len() - after {
            continue;
        }
        out.push((at, deobfuscate_word(raw, key, 0)));
    }
    out
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
    fn a_valid_seed_is_claimed_once_and_remembered() {
        let (mut s, store, account_id, id) = session();
        let seed = store.create_migration(account_id, id, 0, 0).unwrap();

        let note = s.claim(seed);
        assert!(note.contains("claimed migration"), "{note}");
        assert!(!note.contains("WRONG CHANNEL"), "{note}");
        assert_eq!(s.claimed().unwrap().character_id, id);

        let mut other = Session::new(store, Arc::new(Config::default()));
        assert!(other.claim(seed).contains("matches no unconsumed migration"));
    }

    #[test]
    fn a_seed_for_another_channel_is_reported_rather_than_silently_accepted() {
        let (_, store, account_id, id) = session();
        let seed = store.create_migration(account_id, id, 0, 7).unwrap();
        let config = Config { channel_id: 0, ..Config::default() };
        let mut s = Session::new(store, Arc::new(config));
        assert!(s.claim(seed).contains("WRONG CHANNEL"));
    }

    #[test]
    fn an_unknown_seed_says_so_instead_of_failing() {
        let (mut s, _, _, _) = session();
        assert!(s.claim(0xDEADBEEF).contains("matches no unconsumed migration"));
    }

    /// The transform has to round-trip against the builder in `crates/net`, or the seed
    /// we look for is not the seed we sent.
    #[test]
    fn deobfuscate_undoes_the_migration_builder() {
        let addr = "127.0.0.1:8485".parse().unwrap();
        for seed in [1u32, 0xDEAD_BEEF, 0xFFFF_FFFF, 0x1234_5678] {
            let body = net::opcode::migrate(addr, 200, seed);
            let key = u32::from_le_bytes(body[47..51].try_into().unwrap());
            let raw = u32::from_le_bytes(body[55..59].try_into().unwrap());
            assert_eq!(deobfuscate_word(raw, key, 0), seed);
        }
    }

    /// The search must actually find a seed planted in a body-shaped buffer, at a
    /// position it was not told about.
    #[test]
    fn the_search_finds_a_planted_seed() {
        let seed = 0x1BAD_C0DE;
        let key = 0x5EED_5EEDu32;
        // A 26-byte prefix of the shape FUN_1415d10e0 writes, then key, length, payload.
        let mut body = vec![0x11u8; 26];
        body.extend_from_slice(&key.to_le_bytes());
        body.extend_from_slice(&4u32.to_le_bytes());
        // Invert the client's forward transform to get the raw word it would have written.
        let raw = {
            let t: u32 = (seed ^ 0xAAAA_BBBBu32)
                .wrapping_sub(0x369F_144D)
                .wrapping_sub(key >> 7);
            t ^ key
        };
        body.extend_from_slice(&raw.to_le_bytes());

        let found = seed_candidates(&body);
        assert!(
            found.iter().any(|&(at, value)| at == 26 && value == seed),
            "planted seed not among {found:?}"
        );
    }

    #[test]
    fn the_search_does_not_panic_on_short_or_empty_bodies() {
        for len in 0..16 {
            seed_candidates(&vec![0xAB; len]);
        }
    }
}
