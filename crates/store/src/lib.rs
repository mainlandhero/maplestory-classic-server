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
//!
//! Two more rules are enforced here rather than left to callers, for the same reason:
//!
//! * **A trade-blocked item cannot be put in storage.** There is no API in [`storage`] that
//!   can do it; a caller that tries gets [`StoreError::ItemMayNotBeStored`] back. The flag is
//!   the client's own `info/tradeBlock`, baked into [`inventory::ItemRules`] by
//!   `tools/gen_item_rules.py`.
//! * **A meso balance cannot go negative**, and every read-modify-write of one is a single
//!   transaction, so two concurrent spends cannot both see the same balance.

pub mod abilityspend;
pub mod cash;
pub mod character;
pub mod claims;
pub mod codes;
pub mod db;
pub mod inventory;
pub mod migration;
pub mod error;
pub mod password;
pub mod quest;
pub mod rates;
pub mod session;
pub mod skills;
pub mod skillpoints;
pub mod storage;

pub use abilityspend::ApSpend;
pub use character::{NameCheck, MAX_CHARACTER_NAME_LEN, MIN_CHARACTER_NAME_LEN};
pub use claims::{LoginClaim, LOGIN_CLAIM_TTL_SECS};
pub use codes::{NewCode, CODE_ALPHABET, CODE_CHARS, INVITE_TTL_SECS, RECOVERY_TTL_SECS};
pub use inventory::{
    Bag, Equipped, EquippedItem, InvItem, InventoryType, Item, ItemKind, ItemRules, MoveOutcome,
};
pub use quest::{QuestRow, QuestState};
pub use skillpoints::{
    balance, Refunded, SkillUp, SpendOutcome, SpendRefusal, SpendRow, CLIENT_COMPUTED_TIER,
    MAX_POOL_TIER,
};
pub use storage::{
    StorageBox, StorageItem, DEFAULT_STORAGE_SLOTS, MAX_STORAGE_SLOTS, MIN_STORAGE_SLOTS,
};
pub use db::{Account, AuthOutcome, Store, FIRST_CHARACTER_ID, SESSION_TTL_SECS};
pub use error::{Result, StoreError};
pub use migration::{ClaimedMigration, MIGRATION_TTL_SECS};
pub use password::{hash_password, verify_password, MIN_PASSWORD_LEN};
pub use session::{hash_token, new_token, NewSession};
