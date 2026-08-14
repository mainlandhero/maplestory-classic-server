//! Session tokens.
//!
//! A token is 32 random bytes from the OS CSPRNG, shown to the client as hex. The
//! database stores only the **SHA-256 of the token**, never the token itself — so a
//! leaked database does not hand an attacker live sessions.
//!
//! SHA-256 (rather than argon2) is right here: the token is already high-entropy
//! random, so there is nothing to brute-force and no need to be slow.

use rand::RngCore;
use sha2::{Digest, Sha256};

/// Token length in bytes before hex encoding.
pub const TOKEN_BYTES: usize = 32;

/// A freshly minted token. The plain text exists only long enough to hand to the
/// client; only [`Self::hash`] is persisted.
#[derive(Debug, Clone)]
pub struct NewSession {
    /// Hex token to give the client. Never store this.
    pub token: String,
    /// SHA-256 of `token`, hex encoded. This is what goes in the database.
    pub hash: String,
}

/// Generate a new random session token.
pub fn new_token() -> NewSession {
    let mut raw = [0u8; TOKEN_BYTES];
    rand::rngs::OsRng.fill_bytes(&mut raw);
    let token = hex::encode(raw);
    let hash = hash_token(&token);
    NewSession { token, hash }
}

/// Hash a token for storage or lookup.
pub fn hash_token(token: &str) -> String {
    let mut h = Sha256::new();
    h.update(token.as_bytes());
    hex::encode(h.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_are_unique_and_long() {
        let a = new_token();
        let b = new_token();
        assert_ne!(a.token, b.token);
        assert_eq!(a.token.len(), TOKEN_BYTES * 2);
    }

    #[test]
    fn stored_hash_does_not_reveal_the_token() {
        let s = new_token();
        assert_ne!(s.hash, s.token);
        assert!(!s.hash.contains(&s.token));
    }

    #[test]
    fn hashing_is_deterministic_for_lookup() {
        let s = new_token();
        assert_eq!(hash_token(&s.token), s.hash);
    }
}
