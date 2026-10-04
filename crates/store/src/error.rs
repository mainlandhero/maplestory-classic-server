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

    /// Ten random 32-bit seeds in a row already existed. That is not bad luck at any
    /// plausible table size, so it is reported rather than retried forever.
    #[error("could not mint a unique migration seed after {tries} tries")]
    MigrationSeedExhausted { tries: u32 },

    // -----------------------------------------------------------------------------------
    // Inventory and storage. Every one of these is a REFUSAL a caller has to answer, not a
    // failure it can drop: an unanswered `0x0107` latches `player+0x2330` and kills the
    // client's whole inventory UI for the rest of the session. See
    // `net::inventory::inventory_rejected`, which exists for exactly this.
    // -----------------------------------------------------------------------------------
    /// The owner: *"Please do not allow untradeable items to be stored."*
    ///
    /// Enforced in this crate rather than by the caller, the same way password hashing is:
    /// there is no API here that puts a trade-blocked item into storage. The flag is the
    /// client's own `info/tradeBlock`; see [`crate::inventory::ItemRules`].
    #[error("item {item_id} is untradeable and may not be put in storage")]
    ItemMayNotBeStored { item_id: u32 },

    /// The owner: *"Please do not allow quest items to be sold."*
    ///
    /// Enforced at [`crate::db::Store::sell_item`] rather than by the caller, for the same
    /// reason as [`StoreError::ItemMayNotBeStored`]. The flag is the client's own `info/quest`.
    #[error("item {item_id} is a quest item and may not be sold")]
    ItemMayNotBeSold { item_id: u32 },

    /// A slot number outside `1..=slots`. **Slot 0 does not exist** - the client's slot
    /// arrays are 1-based and index 0 is an unused hole; see
    /// `net::opcode::PRESENCE_INVENTORY_SIZE`.
    #[error("slot {slot} is outside 1..={slots}")]
    SlotOutOfRange { slot: u16, slots: u16 },

    /// The move's destination already holds something that cannot merge with the source.
    #[error("slot {slot} is already occupied")]
    SlotOccupied { slot: u16 },

    /// Nothing is in the slot the caller named.
    #[error("slot {slot} is empty")]
    SlotEmpty { slot: u16 },

    /// Every slot of the target inventory is taken.
    #[error("inventory {inv_type} is full ({slots} slots)")]
    BagFull { inv_type: u8, slots: u16 },

    /// Every storage slot is taken.
    #[error("storage is full ({slots} slots)")]
    StorageFull { slots: u16 },

    /// Asked to take more out of a stack than it holds. Refused rather than clamped: a
    /// clamp here would silently hand the client a different number from the one it asked
    /// for, and the client is the thing drawing the count.
    #[error("slot {slot} holds {have} of item {item_id}, not {want}")]
    NotEnoughItems { slot: u16, item_id: u32, have: u16, want: u16 },

    /// A spend that would take a balance below zero. Mesos are unsigned everywhere in this
    /// crate; there is no API that can produce a negative balance.
    #[error("balance is {have} mesos, not {want}")]
    NotEnoughMesos { have: u32, want: u32 },

    /// Asked to wear something that is not an equip. Refused rather than coerced: the
    /// equipped list is what dresses the avatar, and a consumable in it is a record the client
    /// has to decode.
    #[error("item {item_id} is not an equip and cannot be worn")]
    NotAnEquip { item_id: u32 },

    /// A byte that is not one of the six inventories. The client numbers them 1..=6 and
    /// `0x0107` carries the number as an `i8`, so a bad one arrives from the wire rather
    /// than from our own code.
    #[error("{value} is not one of the six inventories (1..=6)")]
    InvalidInventoryType { value: i16 },

    // -----------------------------------------------------------------------------------
    // Player stores (`crate::playershop`).
    // -----------------------------------------------------------------------------------
    /// The store already lists as many lines as its window holds.
    #[error("the store already lists {lines} items")]
    ShopFull { lines: u16 },

    /// A row number with no line behind it - sold out, or taken back, since the window drew it.
    #[error("the store has no item at row {index}")]
    ShopLineGone { index: u16 },

    /// More bundles asked for than the line still has.
    #[error("the store has {have} bundle(s) of that, not {want}")]
    ShopStock { have: u32, want: u32 },

    /// A payment that would take a wallet past what it can hold.
    #[error("a wallet of {have} mesos cannot take {incoming} more")]
    WalletCap { have: u32, incoming: u64 },
}

pub type Result<T> = std::result::Result<T, StoreError>;
