#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),

    #[error("password hashing failed: {0}")]
    Hash(String),

    #[error("password must be at least {min} characters")]
    PasswordTooShort { min: usize },

    #[error("account {name:?} already exists")]
    AccountExists { name: String },

    #[error("no such account: {name:?}")]
    NoSuchAccount { name: String },

    #[error("character name {name:?} is already taken")]
    CharacterNameTaken { name: String },

    #[error("character name {name:?} is not allowed")]
    InvalidCharacterName { name: String },

    #[error("account name {name:?} is invalid: {reason}")]
    InvalidAccountName { name: String, reason: &'static str },
}

pub type Result<T> = std::result::Result<T, StoreError>;
