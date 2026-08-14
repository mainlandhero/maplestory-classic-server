//! Password hashing.
//!
//! Rules this module exists to enforce:
//!
//! * Passwords are **never** stored in plain text, and never stored reversibly.
//! * Hashing is **argon2id** — memory-hard, so attacking a stolen database stays
//!   expensive even with a GPU.
//! * Every password gets a **unique random salt**, generated here from the OS CSPRNG
//!   and embedded in the PHC string alongside the parameters. Callers cannot supply or
//!   skip the salt.
//! * Verification is **constant-time** (`argon2`'s comparison), so timing does not leak
//!   whether a prefix matched.
//! * Nothing here logs, prints, or returns the plain-text password.

use argon2::Argon2;
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use rand::rngs::OsRng;

use crate::error::{Result, StoreError};

/// Shortest password accepted. Local server, but a trivially guessable password is
/// still worth refusing outright.
pub const MIN_PASSWORD_LEN: usize = 8;

/// Hash a password into a PHC string (`$argon2id$v=19$m=...,t=...,p=...$salt$hash`).
///
/// The salt is random per call, so hashing the same password twice yields different
/// strings — that is correct and expected.
pub fn hash_password(password: &str) -> Result<String> {
    if password.chars().count() < MIN_PASSWORD_LEN {
        return Err(StoreError::PasswordTooShort {
            min: MIN_PASSWORD_LEN,
        });
    }

    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| StoreError::Hash(e.to_string()))
}

/// Verify a password against a stored PHC string.
///
/// Returns `Ok(false)` for a wrong password and `Err` only when the stored hash itself
/// is unreadable — callers must not distinguish those two cases to the outside world.
pub fn verify_password(password: &str, stored_phc: &str) -> Result<bool> {
    let parsed =
        PasswordHash::new(stored_phc).map_err(|e| StoreError::Hash(e.to_string()))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_phc_argon2id_and_not_the_password() {
        let hash = hash_password("correct horse battery").unwrap();
        assert!(hash.starts_with("$argon2id$"), "got {hash}");
        assert!(
            !hash.contains("correct horse battery"),
            "the password must not appear in the hash"
        );
    }

    #[test]
    fn same_password_hashes_differently_each_time() {
        // Proves a fresh random salt is used; equal hashes would mean no salt.
        let a = hash_password("correct horse battery").unwrap();
        let b = hash_password("correct horse battery").unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn verifies_correct_and_rejects_wrong() {
        let hash = hash_password("correct horse battery").unwrap();
        assert!(verify_password("correct horse battery", &hash).unwrap());
        assert!(!verify_password("Correct horse battery", &hash).unwrap());
        assert!(!verify_password("", &hash).unwrap());
    }

    #[test]
    fn both_salts_verify_against_their_own_hash() {
        let pw = "another good password";
        let a = hash_password(pw).unwrap();
        let b = hash_password(pw).unwrap();
        assert!(verify_password(pw, &a).unwrap());
        assert!(verify_password(pw, &b).unwrap());
    }

    #[test]
    fn short_passwords_are_refused() {
        assert!(matches!(
            hash_password("short"),
            Err(StoreError::PasswordTooShort { .. })
        ));
    }

    #[test]
    fn corrupt_stored_hash_is_an_error_not_a_pass() {
        assert!(verify_password("whatever", "not-a-phc-string").is_err());
    }
}
