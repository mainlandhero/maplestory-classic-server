//! SQLite persistence for MapleCW.
//!
//! Everything is local and single-machine, so SQLite is the whole storage story.
//!
//! Credential handling is the point of this crate, and the rules are enforced here
//! rather than left to callers:
//!
//! * Passwords are hashed with **argon2id** and a **unique random salt** per password,
//!   stored as a PHC string. There is no API that stores a password any other way.
//! * Session tokens are 32 random bytes from the OS CSPRNG; the database keeps only
//!   their SHA-256, so a stolen database yields no usable sessions.
//! * Authentication does not reveal whether an account exists — a wrong name and a
//!   wrong password give the same answer and cost comparable time.
//! * Nothing in this crate logs or prints a password or a token.

pub mod character;
pub mod db;
pub mod error;
pub mod password;
pub mod session;

pub use character::{NameCheck, MAX_CHARACTER_NAME_LEN, MIN_CHARACTER_NAME_LEN};
pub use db::{Account, AuthOutcome, Store, SESSION_TTL_SECS};
pub use error::{Result, StoreError};
pub use password::{hash_password, verify_password, MIN_PASSWORD_LEN};
pub use session::{hash_token, new_token, NewSession};
