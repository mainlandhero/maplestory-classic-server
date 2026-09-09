//! Storage: one shared box per **account**, holding mesos and items.
//!
//! The owner, 2026-08-19: *"Storage is kind of like inventory, except all of the characters of a
//! particular account share this inventory. The storage stores mesos and items."* And:
//! *"Please do not allow untradeable items to be stored."*
//!
//! # Why this is not a column on `characters`
//!
//! The unit that owns a storage box is the **account**, and `characters` is keyed per
//! character. Two tables, both keyed on `account_id`:
//!
//! * `storage` - one row per account: its meso balance and how many slots it has.
//! * `storage_item` - one row per occupied slot, with exactly the item columns `inventory`
//!   uses, so an equip does not lose its per-item stats by being put away.
//!
//! The item column set, the row codec and the "an empty slot is the absence of a row" rule all
//! come from [`crate::inventory`] - one definition, so the two containers cannot drift and an
//! item can cross between them without being re-encoded.
//!
//! # One flat list, not six bags
//!
//! Storage in this game family is a single list that the UI groups by inventory tab, not six
//! separate containers, so there is one slot count rather than six. **Nothing in this client
//! has been read to confirm that**, and it is written down here rather than smoothed over: if
//! the storage dialog turns out to carry six counts, this table needs the same six-column
//! treatment `characters` has and that is a schema change, not a code change.
//!
//! # The untradeable rule is enforced HERE
//!
//! The owner's restriction is a property of the client's own data (`info/tradeBlock`), and it is
//! enforced at this crate's API rather than left to a caller - the same way password hashing
//! is (see the crate docs). **There is no function in this module that can put a trade-blocked
//! item into storage.** A caller that tries gets [`StoreError::ItemMayNotBeStored`] back and
//! nothing is written.
//!
//! That error is a **refusal a caller still has to answer**. An unanswered inventory packet
//! latches `player+0x2330` and kills the client's whole inventory UI for the session; see
//! `net::inventory::inventory_rejected`, which exists for exactly this case.
//!
//! # Nothing here authenticates
//!
//! `account_id` is the box's owner, not a credential that was checked. The channel connection
//! carries none at all - it is identified only by the migration row it claimed - so the
//! account id reaching this module came from that row and from nothing else.

use rusqlite::types::Value;
use rusqlite::{params_from_iter, Connection, OptionalExtension};

use crate::db::Store;
use crate::error::{Result, StoreError};
use crate::inventory::{
    self, check_slot, item_columns, item_from_row, item_values, InvItem, InventoryType, Item,
    ItemKind, ItemRules,
};

/// Slots a brand-new storage box has.
///
/// **Thirty, and it is the owner's number.** 2026-08-22: *"Same as inventory, the first 30 slots of
/// storage is provided as default."* That is the same figure as
/// [`net::opcode::DEFAULT_INVENTORY_SLOTS`], and the constant below is written in terms of it
/// rather than as a second literal `30`, so the two cannot drift apart while the sentence that
/// ties them together stays true.
///
/// # It used to be 4, and the way that nearly got "confirmed" is worth keeping
///
/// The old value was **[I]** and honestly marked as uncorroborated. Decoding the storage
/// dialog did not settle it either: what `research/storage.md` measures is the field's
/// *width*, a `u8`, which admits any of 0..=255.
///
/// **And the v214 reference tree contains a line that would have read as corroboration.**
/// `new Trunk(GameConstants.DEFAULT_TRUNK_SIZE); // Free first 4 storage slot` - a comment
/// naming exactly the old number, one line from where it would have been believed.
/// `GameConstants.DEFAULT_TRUNK_SIZE` is **56**. The comment contradicts its own constant, so
/// it corroborated nothing, and a search that stopped at the comment would have come back with
/// a confident four and a citation.
pub const DEFAULT_STORAGE_SLOTS: u16 = net::opcode::DEFAULT_INVENTORY_SLOTS;

/// The floor. A zero-slot box is a box that refuses everything, which reads on screen as
/// broken rather than as full.
pub const MIN_STORAGE_SLOTS: u16 = 1;

/// The ceiling. **[I]**, and no longer the same provenance as [`DEFAULT_STORAGE_SLOTS`] -
/// that one is the owner's number now, this one is still nobody's. It exists so a typo cannot ask
/// for 65535 slots, not because the client is known to stop anywhere.
///
/// One thing the decode *does* constrain: the slot count goes on the wire as a **`u8`**, so
/// anything above 255 could not be sent whatever this said.
pub const MAX_STORAGE_SLOTS: u16 = 100;

/// The two bounds the default has to satisfy, checked at **compile** time rather than in a
/// test - clippy is right that a comparison between two constants is not an assertion, and a
/// build that cannot produce the bad value beats a test that notices it afterwards.
///
/// The `u8` bound is the one with teeth: the slot count goes on the wire as a single byte, so
/// a default above 255 would be silently truncated by `open_storage`.
const _: () = assert!(DEFAULT_STORAGE_SLOTS <= MAX_STORAGE_SLOTS);
const _: () = assert!(DEFAULT_STORAGE_SLOTS <= u8::MAX as u16);

/// One occupied storage slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorageItem {
    /// **1-based**, the same rule the bag uses.
    pub slot: u16,
    pub item: Item,
}

/// An account's whole storage box.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageBox {
    pub mesos: u32,
    pub slots: u16,
    /// Occupied slots only, in slot order - stable between two identical loads.
    pub items: Vec<StorageItem>,
}

impl StorageBox {
    /// A box for an account that has never opened one. Not written until something is put in
    /// it: an empty row and no row are the same box, and keeping one representation means a
    /// storage NPC cannot create state just by being talked to.
    fn empty() -> Self {
        StorageBox { mesos: 0, slots: DEFAULT_STORAGE_SLOTS, items: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn free_slots(&self) -> u16 {
        self.slots.saturating_sub(self.items.len() as u16)
    }

    fn lowest_free(&self) -> Option<u16> {
        let taken: std::collections::HashSet<u16> = self.items.iter().map(|i| i.slot).collect();
        (1..=self.slots).find(|s| !taken.contains(s))
    }
}

/// Create the storage tables. Called from `Store::init`.
///
/// Both are new, so `CREATE TABLE IF NOT EXISTS` is enough - the same situation `quest.rs` is
/// in. The `ALTER` discipline is only needed for columns on tables that already exist in
/// The owner's database, which here is `characters.mesos` and `equipment`'s stat tail.
pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(&format!(
        r#"
        -- One row per account that has ever used storage. Absent means "the default box",
        -- which is the same thing as an empty one - so a storage NPC being talked to creates
        -- no state.
        CREATE TABLE IF NOT EXISTS storage (
            account_id INTEGER PRIMARY KEY REFERENCES accounts(id) ON DELETE CASCADE,
            mesos      INTEGER NOT NULL DEFAULT 0,
            slots      INTEGER NOT NULL DEFAULT {default_slots},
            CHECK (mesos >= 0),
            CHECK (slots >= 1)
        );

        -- One row per OCCUPIED storage slot. Exactly the item columns `inventory` uses, so an
        -- equip crossing between the two keeps its per-item stats.
        CREATE TABLE IF NOT EXISTS storage_item (
            account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
            -- 1-BASED, the same rule the bag uses.
            slot       INTEGER NOT NULL,
            item_id    INTEGER NOT NULL,
            kind       INTEGER NOT NULL,
            quantity   INTEGER NOT NULL DEFAULT 1,
            {stats}
            PRIMARY KEY (account_id, slot),
            CHECK (slot >= 1),
            CHECK (kind IN (1, 2)),
            CHECK (quantity >= 1),
            CHECK (kind <> 1 OR quantity = 1)
        );

        CREATE INDEX IF NOT EXISTS idx_storage_item_account ON storage_item(account_id);
        "#,
        default_slots = DEFAULT_STORAGE_SLOTS,
        stats = inventory::equip_stat_declarations()
    ))?;
    Ok(())
}

/// Read a whole box on a caller-supplied connection, so a transfer can read both containers
/// inside one transaction.
fn read_storage(conn: &Connection, account_id: i64) -> Result<StorageBox> {
    let header: Option<(i64, i64)> = conn
        .query_row(
            "SELECT mesos, slots FROM storage WHERE account_id = ?1",
            [account_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()?;
    let mut out = match header {
        Some((mesos, slots)) => StorageBox {
            mesos: mesos.max(0) as u32,
            slots: (slots.max(1) as u16).min(MAX_STORAGE_SLOTS),
            items: Vec::new(),
        },
        None => StorageBox::empty(),
    };

    let mut stmt = conn.prepare(&format!(
        "SELECT slot, {} FROM storage_item WHERE account_id = ?1 ORDER BY slot",
        item_columns().join(", ")
    ))?;
    let rows = stmt.query_map([account_id], |row| {
        Ok(StorageItem { slot: row.get::<_, i64>(0)? as u16, item: item_from_row(row, 1)? })
    })?;
    out.items = rows.collect::<rusqlite::Result<_>>()?;
    Ok(out)
}

/// Make sure the header row exists before it is updated. `INSERT OR IGNORE` rather than a
/// read-then-write, so two concurrent deposits cannot both decide it is missing.
fn ensure_header(conn: &Connection, account_id: i64) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO storage (account_id, mesos, slots) VALUES (?1, 0, ?2)",
        rusqlite::params![account_id, DEFAULT_STORAGE_SLOTS],
    )?;
    Ok(())
}

/// Insert-or-replace one storage slot. The mirror of `inventory::set_slot`.
fn set_storage_slot_row(conn: &Connection, account_id: i64, slot: u16, item: &Item) -> Result<()> {
    let columns = item_columns();
    let placeholders: Vec<String> = (3..3 + columns.len()).map(|i| format!("?{i}")).collect();
    let assignments: Vec<String> =
        columns.iter().enumerate().map(|(i, c)| format!("{c} = ?{}", i + 3)).collect();
    let sql = format!(
        "INSERT INTO storage_item (account_id, slot, {cols})
         VALUES (?1, ?2, {ph})
         ON CONFLICT(account_id, slot) DO UPDATE SET {set}",
        cols = columns.join(", "),
        ph = placeholders.join(", "),
        set = assignments.join(", "),
    );
    let mut values = vec![Value::Integer(account_id), Value::Integer(i64::from(slot))];
    values.extend(item_values(item));
    conn.execute(&sql, params_from_iter(values))?;
    Ok(())
}

/// Put an item into a box, stacking onto an existing slot first.
///
/// **The owner's restriction is checked here**, which is the one place every write path goes
/// through, rather than at each of them. Nothing is written when it fails.
fn place_into_storage(
    conn: &Connection,
    account_id: i64,
    boxed: &mut StorageBox,
    item: &Item,
    max_stack: u16,
) -> Result<Vec<StorageItem>> {
    if !ItemRules::may_be_stored(item.item_id) {
        return Err(StoreError::ItemMayNotBeStored { item_id: item.item_id });
    }
    ensure_header(conn, account_id)?;
    let mut changed = Vec::new();

    let mut remaining = match item.kind {
        ItemKind::Equip(_) => {
            let slot = boxed.lowest_free().ok_or(StoreError::StorageFull { slots: boxed.slots })?;
            set_storage_slot_row(conn, account_id, slot, item)?;
            let placed = StorageItem { slot, item: *item };
            boxed.items.push(placed);
            return Ok(vec![placed]);
        }
        ItemKind::Bundle { quantity } => quantity,
    };
    let cap = max_stack.max(1);

    if cap > 1 {
        let existing: Vec<(u16, u16)> = boxed
            .items
            .iter()
            .filter(|i| i.item.item_id == item.item_id && !i.item.kind.is_equip())
            .map(|i| (i.slot, i.item.kind.quantity()))
            .collect();
        for (slot, have) in existing {
            if remaining == 0 {
                break;
            }
            let room = cap.saturating_sub(have);
            if room == 0 {
                continue;
            }
            let take = room.min(remaining);
            remaining -= take;
            let topped = Item::bundle(item.item_id, have + take);
            set_storage_slot_row(conn, account_id, slot, &topped)?;
            if let Some(e) = boxed.items.iter_mut().find(|i| i.slot == slot) {
                e.item = topped;
            }
            changed.push(StorageItem { slot, item: topped });
        }
    }

    while remaining > 0 {
        // Nothing is committed on the way out: the caller's transaction rolls the whole
        // deposit back, so a box that cannot take the lot takes none of it.
        let slot = boxed.lowest_free().ok_or(StoreError::StorageFull { slots: boxed.slots })?;
        let take = cap.min(remaining);
        remaining -= take;
        let placed = Item::bundle(item.item_id, take);
        set_storage_slot_row(conn, account_id, slot, &placed)?;
        boxed.items.push(StorageItem { slot, item: placed });
        changed.push(StorageItem { slot, item: placed });
    }
    Ok(changed)
}

/// Take `count` out of a storage slot on a caller-supplied connection.
fn take_from_storage(
    conn: &Connection,
    account_id: i64,
    slot: u16,
    count: Option<u16>,
) -> Result<Item> {
    let item: Option<Item> = conn
        .query_row(
            &format!(
                "SELECT {} FROM storage_item WHERE account_id = ?1 AND slot = ?2",
                item_columns().join(", ")
            ),
            rusqlite::params![account_id, slot],
            |row| item_from_row(row, 0),
        )
        .optional()?;
    let Some(item) = item else { return Err(StoreError::SlotEmpty { slot }) };
    let have = item.kind.quantity();
    let want = count.unwrap_or(have).max(1);
    if want > have {
        return Err(StoreError::NotEnoughItems { slot, item_id: item.item_id, have, want });
    }
    if want == have {
        conn.execute(
            "DELETE FROM storage_item WHERE account_id = ?1 AND slot = ?2",
            rusqlite::params![account_id, slot],
        )?;
    } else {
        set_storage_slot_row(conn, account_id, slot, &Item::bundle(item.item_id, have - want))?;
    }
    Ok(match item.kind {
        ItemKind::Equip(_) => item,
        ItemKind::Bundle { .. } => Item::bundle(item.item_id, want),
    })
}

impl Store {
    /// The whole box: mesos, slot count and contents. **This is the open-storage call.**
    ///
    /// An account that has never used storage gets the default box rather than an error, and
    /// nothing is written by asking.
    pub fn storage(&self, account_id: i64) -> Result<StorageBox> {
        read_storage(&self.conn(), account_id)
    }

    /// What is in one storage slot, if anything.
    pub fn storage_slot(&self, account_id: i64, slot: u16) -> Result<Option<Item>> {
        Ok(self.storage(account_id)?.items.into_iter().find(|i| i.slot == slot).map(|i| i.item))
    }

    /// Put an item straight into storage without taking it from anywhere.
    ///
    /// For a GM command, a fixture or a load. The play path is [`Store::store_item`], which
    /// takes it out of a character's bag in the same transaction.
    ///
    /// Refuses a trade-blocked item - see the module docs.
    pub fn storage_deposit(
        &self,
        account_id: i64,
        item: &Item,
        max_stack: u16,
    ) -> Result<Vec<StorageItem>> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let mut boxed = read_storage(&tx, account_id)?;
        let changed = place_into_storage(&tx, account_id, &mut boxed, item, max_stack)?;
        tx.commit()?;
        Ok(changed)
    }

    /// Take an item out of storage without putting it anywhere. `None` takes the whole slot.
    pub fn storage_withdraw(
        &self,
        account_id: i64,
        slot: u16,
        count: Option<u16>,
    ) -> Result<Item> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let taken = take_from_storage(&tx, account_id, slot, count)?;
        tx.commit()?;
        Ok(taken)
    }

    /// **Bag -> storage, in one transaction.** The deposit a storage NPC performs.
    ///
    /// Returns the storage slots that changed, so the caller can redraw the box without a
    /// second read. Either both halves happen or neither does: an item that is briefly in both
    /// places is an item a crash duplicates.
    ///
    /// Refuses, and writes nothing, when the item is trade-blocked.
    pub fn store_item(
        &self,
        account_id: i64,
        character_id: u32,
        inv_type: InventoryType,
        slot: u16,
        count: Option<u16>,
        max_stack: u16,
    ) -> Result<Vec<StorageItem>> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;

        // The restriction is checked BEFORE anything is taken out of the bag. It would be
        // checked anyway by place_into_storage and the transaction would roll back, but doing
        // it first means the refusal never depends on a rollback working.
        let Some(peek) = inventory::read_slot(&tx, character_id, inv_type, slot)? else {
            return Err(StoreError::SlotEmpty { slot });
        };
        if !ItemRules::may_be_stored(peek.item_id) {
            return Err(StoreError::ItemMayNotBeStored { item_id: peek.item_id });
        }

        let mut boxed = read_storage(&tx, account_id)?;
        let taken = inventory::take_from_bag(&tx, character_id, inv_type, slot, count)?;
        let changed = place_into_storage(&tx, account_id, &mut boxed, &taken, max_stack)?;
        tx.commit()?;
        Ok(changed)
    }

    /// **Storage -> bag, in one transaction.** The withdrawal a storage NPC performs.
    ///
    /// `inv_type` is the bag it lands in. It is a parameter rather than derived from the item
    /// id because only the equip case of [`InventoryType::for_item`] is corroborated in this
    /// client; the caller has the item table and can be sure.
    pub fn take_item(
        &self,
        account_id: i64,
        character_id: u32,
        storage_slot: u16,
        count: Option<u16>,
        inv_type: InventoryType,
        max_stack: u16,
    ) -> Result<Vec<InvItem>> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let mut bag = inventory::read_bag(&tx, character_id)?;
        let taken = take_from_storage(&tx, account_id, storage_slot, count)?;
        let changed =
            inventory::place_into_bag(&tx, character_id, &mut bag, inv_type, &taken, max_stack)?;
        tx.commit()?;
        Ok(changed)
    }

    /// **Organize Item.** Repack the box so the occupied slots run 1..n with no gaps.
    ///
    /// The owner, 2026-08-22: *"I tried hitting the 'Organize Item' 3 times, but it did not perform
    /// anything."* It did not, because the session answered mode 6 with the unchanged box and
    /// a log line calling that *"a legal no-op"*. It is legal, and it is not what the button
    /// says it does.
    ///
    /// # The order, and why it is this one
    ///
    /// **Inventory type ascending, then item id ascending, then the old slot.** The first two
    /// are what this game family does - items group into their tabs and sort by id inside a
    /// tab. The third is only a tie-break, and it exists so the result is a **pure function of
    /// the box**: two sorts of the same contents produce the same layout, and an item that
    /// appears twice keeps its relative order rather than swapping on every click.
    ///
    /// # Why every row is rewritten rather than moved
    ///
    /// `(account_id, slot)` is the primary key, so shuffling in place can collide with a row
    /// that has not moved yet. Deleting the account's rows and re-inserting them inside one
    /// transaction has no ordering hazard at all, and the box is small - it cannot exceed
    /// [`MAX_STORAGE_SLOTS`].
    ///
    /// # It must not lose an item
    ///
    /// The count is asserted before the commit. A sort that silently dropped a row would be
    /// the worst possible bug in this file - the player would see the box tidy itself and
    /// come back shorter - so the transaction rolls back rather than commits if the numbers
    /// disagree.
    pub fn sort_storage(&self, account_id: i64) -> Result<StorageBox> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let mut boxed = read_storage(&tx, account_id)?;
        let before = boxed.items.len();
        if before == 0 {
            return Ok(boxed);
        }

        boxed.items.sort_by_key(|it| {
            let inv = InventoryType::for_item(it.item.item_id).map_or(u8::MAX, |i| i.as_u8());
            (inv, it.item.item_id, it.slot)
        });
        for (i, it) in boxed.items.iter_mut().enumerate() {
            it.slot = i as u16 + 1;
        }

        tx.execute("DELETE FROM storage_item WHERE account_id = ?1", [account_id])?;
        for it in &boxed.items {
            set_storage_slot_row(&tx, account_id, it.slot, &it.item)?;
        }

        // Read it back rather than trusting the writes. `set_storage_slot_row` is an upsert,
        // and an upsert that collided would leave the box short without erroring.
        let after = read_storage(&tx, account_id)?;
        if after.items.len() != before {
            return Err(StoreError::SlotOutOfRange {
                slot: after.items.len() as u16,
                slots: before as u16,
            });
        }
        tx.commit()?;
        Ok(after)
    }

    /// The mesos in the box. Separate from a character's own purse: storage holds its own.
    pub fn storage_mesos(&self, account_id: i64) -> Result<u32> {
        Ok(self.storage(account_id)?.mesos)
    }

    pub fn set_storage_mesos(&self, account_id: i64, mesos: u32) -> Result<()> {
        let conn = self.conn();
        ensure_header(&conn, account_id)?;
        conn.execute(
            "UPDATE storage SET mesos = ?2 WHERE account_id = ?1",
            rusqlite::params![account_id, i64::from(mesos)],
        )?;
        Ok(())
    }

    /// Add or withdraw mesos and return the new balance. A withdrawal that would go below zero
    /// is refused and changes nothing; read and write are one transaction, so two concurrent
    /// withdrawals cannot both see the same balance.
    pub fn add_storage_mesos(&self, account_id: i64, delta: i64) -> Result<u32> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        ensure_header(&tx, account_id)?;
        let have: i64 =
            tx.query_row("SELECT mesos FROM storage WHERE account_id = ?1", [account_id], |r| {
                r.get(0)
            })?;
        let next = have + delta;
        if next < 0 {
            return Err(StoreError::NotEnoughMesos {
                have: have.max(0) as u32,
                want: delta.unsigned_abs().min(u64::from(u32::MAX)) as u32,
            });
        }
        let next = next.min(i64::from(u32::MAX));
        tx.execute(
            "UPDATE storage SET mesos = ?2 WHERE account_id = ?1",
            rusqlite::params![account_id, next],
        )?;
        tx.commit()?;
        Ok(next as u32)
    }

    /// Move mesos between a character's purse and the account's box.
    ///
    /// Positive deposits, negative withdraws, and both halves are one transaction - mesos that
    /// leave one side and do not arrive at the other are the same duplication bug as an item
    /// in two containers.
    pub fn move_storage_mesos(
        &self,
        account_id: i64,
        character_id: u32,
        amount: i64,
    ) -> Result<(u32, u32)> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        ensure_header(&tx, account_id)?;
        let purse: i64 = tx
            .query_row("SELECT mesos FROM characters WHERE id = ?1", [i64::from(character_id)], |r| {
                r.get(0)
            })
            .optional()?
            .unwrap_or(0);
        let boxed: i64 =
            tx.query_row("SELECT mesos FROM storage WHERE account_id = ?1", [account_id], |r| {
                r.get(0)
            })?;
        let (next_purse, next_box) = (purse - amount, boxed + amount);
        if next_purse < 0 {
            return Err(StoreError::NotEnoughMesos {
                have: purse.max(0) as u32,
                want: amount.unsigned_abs().min(u64::from(u32::MAX)) as u32,
            });
        }
        if next_box < 0 {
            return Err(StoreError::NotEnoughMesos {
                have: boxed.max(0) as u32,
                want: amount.unsigned_abs().min(u64::from(u32::MAX)) as u32,
            });
        }
        let (next_purse, next_box) =
            (next_purse.min(i64::from(u32::MAX)), next_box.min(i64::from(u32::MAX)));
        tx.execute(
            "UPDATE characters SET mesos = ?2 WHERE id = ?1",
            rusqlite::params![i64::from(character_id), next_purse],
        )?;
        tx.execute(
            "UPDATE storage SET mesos = ?2 WHERE account_id = ?1",
            rusqlite::params![account_id, next_box],
        )?;
        tx.commit()?;
        Ok((next_purse as u32, next_box as u32))
    }

    /// Resize a storage box. Returns the count actually stored, clamped to
    /// [`MIN_STORAGE_SLOTS`]`..=`[`MAX_STORAGE_SLOTS`].
    ///
    /// A shrink that would strand an item is refused rather than truncating - the row would
    /// still be in the table but past the end of the box, so nothing would draw it and the
    /// player would have lost it with nothing to see. Same rule as
    /// `Store::set_inventory_slots`.
    pub fn set_storage_slots(&self, account_id: i64, slots: u16) -> Result<u16> {
        let slots = slots.clamp(MIN_STORAGE_SLOTS, MAX_STORAGE_SLOTS);
        let conn = self.conn();
        let highest: Option<i64> = conn.query_row(
            "SELECT MAX(slot) FROM storage_item WHERE account_id = ?1",
            [account_id],
            |row| row.get(0),
        )?;
        if let Some(highest) = highest {
            if highest as u16 > slots {
                return Err(StoreError::SlotOutOfRange { slot: highest as u16, slots });
            }
        }
        ensure_header(&conn, account_id)?;
        conn.execute(
            "UPDATE storage SET slots = ?2 WHERE account_id = ?1",
            rusqlite::params![account_id, slots],
        )?;
        Ok(slots)
    }

    /// Put an item in a named storage slot, replacing whatever was there.
    ///
    /// The blunt instrument, and it still refuses a trade-blocked item.
    pub fn set_storage_slot(&self, account_id: i64, slot: u16, item: &Item) -> Result<()> {
        if !ItemRules::may_be_stored(item.item_id) {
            return Err(StoreError::ItemMayNotBeStored { item_id: item.item_id });
        }
        let boxed = self.storage(account_id)?;
        check_slot(slot, boxed.slots)?;
        let conn = self.conn();
        ensure_header(&conn, account_id)?;
        set_storage_slot_row(&conn, account_id, slot, item)
    }
}

#[cfg(test)]
mod sort_tests {
    use super::*;

    fn store() -> (crate::Store, i64) {
        let st = crate::Store::open_in_memory().unwrap();
        let account = st.create_account("maplecw", "correct horse battery").unwrap();
        (st, account)
    }

    /// **A sparse box is packed to the front, grouped by tab and ordered by id.**
    ///
    /// The owner, 2026-08-22: *"I tried hitting the 'Organize Item' 3 times, but it did not perform
    /// anything."*
    #[test]
    fn organize_packs_the_box_by_tab_then_id() {
        let (st, account) = store();
        // Deliberately out of order and full of gaps.
        st.set_storage_slot(account, 9, &Item::bundle(2000000, 3)).unwrap();
        st.set_storage_slot(account, 2, &Item::equip(1302000)).unwrap();
        st.set_storage_slot(account, 7, &Item::bundle(4000001, 5)).unwrap();
        st.set_storage_slot(account, 4, &Item::equip(1040001)).unwrap();

        let sorted = st.sort_storage(account).unwrap();
        let layout: Vec<(u16, u32)> =
            sorted.items.iter().map(|i| (i.slot, i.item.item_id)).collect();
        assert_eq!(
            layout,
            vec![(1, 1040001), (2, 1302000), (3, 2000000), (4, 4000001)],
            "packed 1..n, equips first by id, then Use, then Etc"
        );

        // And it survives a re-read: the rows really moved.
        assert_eq!(st.storage(account).unwrap().items, sorted.items);
    }

    /// **Sorting twice is the same as sorting once**, so a second click cannot reshuffle.
    ///
    /// The owner clicked it three times. A sort whose tie-break depended on read order would move
    /// items every click, which on screen is indistinguishable from it being broken.
    #[test]
    fn sorting_is_idempotent_even_with_duplicates() {
        let (st, account) = store();
        st.set_storage_slot(account, 5, &Item::equip(1302000)).unwrap();
        st.set_storage_slot(account, 1, &Item::equip(1302000)).unwrap();
        st.set_storage_slot(account, 8, &Item::equip(1302000)).unwrap();

        let once = st.sort_storage(account).unwrap();
        let twice = st.sort_storage(account).unwrap();
        let thrice = st.sort_storage(account).unwrap();
        assert_eq!(once.items, twice.items);
        assert_eq!(twice.items, thrice.items);
        assert_eq!(once.items.iter().map(|i| i.slot).collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    /// An empty box sorts to an empty box rather than erroring, and the header is untouched.
    #[test]
    fn an_empty_box_sorts_to_nothing() {
        let (st, account) = store();
        let sorted = st.sort_storage(account).unwrap();
        assert!(sorted.is_empty());
        assert_eq!(sorted.slots, DEFAULT_STORAGE_SLOTS);
    }

    /// **Nothing is lost, and the mesos are not touched.**
    #[test]
    fn sorting_loses_no_item_and_no_mesos() {
        let (st, account) = store();
        st.set_storage_mesos(account, 4_321).unwrap();
        for slot in [3u16, 11, 19, 27, 30] {
            st.set_storage_slot(account, slot, &Item::bundle(2000000 + u32::from(slot), 1))
                .unwrap();
        }
        let sorted = st.sort_storage(account).unwrap();
        assert_eq!(sorted.items.len(), 5, "every item survived");
        assert_eq!(sorted.mesos, 4_321, "and the balance is untouched");
        assert_eq!(sorted.items.iter().map(|i| i.slot).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use net::opcode::{Character, EquipOptions, EquipStatSet, EquipStats};

    /// An item this client marks `tradeBlock`. Taken from the generated list rather than
    /// written down, so the test cannot outlive the data it is about.
    fn untradeable() -> u32 {
        *ItemRules::TRADE_BLOCKED.first().expect("the client ships 39 trade-blocked items")
    }

    fn store_with_two_characters() -> (Store, i64, u32, u32) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let first = store
            .create_character(account, 0, &Character { name: "Alpha".into(), ..Character::default() })
            .unwrap()
            .id;
        let second = store
            .create_character(account, 0, &Character { name: "Bravo".into(), ..Character::default() })
            .unwrap()
            .id;
        (store, account, first, second)
    }

    /// **The owner's restriction.** A trade-blocked item cannot reach storage by any path here, and
    /// the caller gets a typed error rather than a silent no-op.
    #[test]
    fn an_untradeable_item_cannot_be_stored_by_any_route() {
        let (store, account, chr, _) = store_with_two_characters();
        let blocked = untradeable();
        assert!(!ItemRules::may_be_stored(blocked), "the generated list must know this id");

        // Straight in.
        assert!(matches!(
            store.storage_deposit(account, &Item::equip(blocked), 1),
            Err(StoreError::ItemMayNotBeStored { item_id }) if item_id == blocked
        ));
        // In a named slot.
        assert!(matches!(
            store.set_storage_slot(account, 1, &Item::equip(blocked)),
            Err(StoreError::ItemMayNotBeStored { .. })
        ));
        // And out of a bag - the path a storage NPC actually uses.
        store
            .set_inventory_slot(chr, InventoryType::Equip, 1, &Item::equip(blocked))
            .unwrap();
        assert!(matches!(
            store.store_item(account, chr, InventoryType::Equip, 1, None, 1),
            Err(StoreError::ItemMayNotBeStored { .. })
        ));

        // Nothing was written, and - the part that matters - the item is still in the bag.
        assert!(store.storage(account).unwrap().is_empty());
        assert!(
            store.inventory_slot(chr, InventoryType::Equip, 1).unwrap().is_some(),
            "a refused deposit must not have taken the item out of the bag"
        );
    }

    /// A tradeable item goes in and comes back out, and every character on the account sees it.
    #[test]
    fn storage_is_shared_by_every_character_on_the_account() {
        let (store, account, first, second) = store_with_two_characters();
        store.set_storage_slots(account, 20).unwrap();
        store
            .set_inventory_slot(first, InventoryType::Use, 1, &Item::bundle(2000000, 30))
            .unwrap();

        let deposited =
            store.store_item(account, first, InventoryType::Use, 1, None, 100).unwrap();
        assert_eq!(deposited.len(), 1);
        assert_eq!(deposited[0].slot, 1);
        assert!(store.inventory_slot(first, InventoryType::Use, 1).unwrap().is_none());

        // The OTHER character takes it out. Same box, no per-character partition.
        let out = store
            .take_item(account, second, 1, None, InventoryType::Use, 100)
            .unwrap();
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].item.item_id, 2000000);
        assert_eq!(out[0].item.kind.quantity(), 30);
        assert!(store.storage(account).unwrap().is_empty());
        assert_eq!(store.bag(second).unwrap().items.len(), 1);
    }

    /// An equip keeps its per-item stats across storage. Without the shared column set this is
    /// where a scrolled item would quietly become a plain one.
    #[test]
    fn an_equip_keeps_its_stats_across_storage() {
        let (store, account, chr, _) = store_with_two_characters();
        let scrolled = EquipStats {
            stats: EquipStatSet { inc_wat: 12, inc_luk: 3, ..EquipStatSet::default() },
            options: EquipOptions { remaining_enhancements: 2, ..EquipOptions::default() },
            ..EquipStats::default()
        };
        let item = Item { item_id: 1302000, kind: ItemKind::Equip(Some(scrolled)), failed_slots: 0 };
        store.set_inventory_slot(chr, InventoryType::Equip, 1, &item).unwrap();

        store.store_item(account, chr, InventoryType::Equip, 1, None, 1).unwrap();
        assert_eq!(store.storage(account).unwrap().items[0].item, item);

        let back = store.take_item(account, chr, 1, None, InventoryType::Equip, 1).unwrap();
        assert_eq!(back[0].item, item, "the scrolls survived both crossings");
    }

    /// A deposit that does not fit takes nothing and leaves the bag alone.
    #[test]
    fn a_deposit_that_does_not_fit_leaves_both_sides_untouched() {
        let (store, account, chr, _) = store_with_two_characters();
        store.set_storage_slots(account, 1).unwrap();
        store.storage_deposit(account, &Item::equip(1040002), 1).unwrap();
        store
            .set_inventory_slot(chr, InventoryType::Etc, 1, &Item::bundle(4000019, 5))
            .unwrap();

        let before_bag = store.bag(chr).unwrap();
        let before_box = store.storage(account).unwrap();
        assert!(matches!(
            store.store_item(account, chr, InventoryType::Etc, 1, None, 1),
            Err(StoreError::StorageFull { slots: 1 })
        ));
        assert_eq!(store.bag(chr).unwrap(), before_bag, "the bag was not touched");
        assert_eq!(store.storage(account).unwrap(), before_box, "nor was the box");
    }

    /// A box nobody has opened reads as the default and writes nothing.
    #[test]
    fn an_unused_box_is_the_default_and_asking_creates_no_row() {
        let (store, account, _, _) = store_with_two_characters();
        let boxed = store.storage(account).unwrap();
        assert_eq!(boxed.slots, DEFAULT_STORAGE_SLOTS);
        assert_eq!(boxed.mesos, 0);
        assert!(boxed.is_empty());
        let rows: i64 = store
            .conn()
            .query_row("SELECT COUNT(*) FROM storage", [], |r| r.get(0))
            .unwrap();
        assert_eq!(rows, 0, "reading a box must not create one");
    }

    /// **Storage starts with the same thirty slots the inventory does.**
    ///
    /// The owner, 2026-08-22: *"Same as inventory, the first 30 slots of storage is provided as
    /// default."* Asserted against `net::opcode::DEFAULT_INVENTORY_SLOTS` rather than against
    /// a literal `30`, because the constant is defined in terms of it and a test that repeated
    /// the literal would pass even if the two drifted apart - which is the whole thing the
    /// definition is there to prevent. The literal is checked once, separately, so that
    /// "they agree" cannot be satisfied by both being wrong.
    #[test]
    fn a_new_box_has_the_same_slots_as_a_new_inventory() {
        assert_eq!(DEFAULT_STORAGE_SLOTS, net::opcode::DEFAULT_INVENTORY_SLOTS);
        assert_eq!(DEFAULT_STORAGE_SLOTS, 30, "the owner's number");
        let (store, account, _, _) = store_with_two_characters();
        let boxed = store.storage(account).unwrap();
        assert_eq!(boxed.slots, 30, "and a brand-new box actually gets them");
    }


    #[test]
    fn storage_mesos_move_between_the_purse_and_the_box_without_being_created() {
        let (store, account, chr, _) = store_with_two_characters();
        store.set_mesos(chr, 1000).unwrap();
        let (purse, boxed) = store.move_storage_mesos(account, chr, 400).unwrap();
        assert_eq!((purse, boxed), (600, 400));
        assert_eq!(store.mesos(chr).unwrap(), 600);
        assert_eq!(store.storage_mesos(account).unwrap(), 400);

        // And back.
        let (purse, boxed) = store.move_storage_mesos(account, chr, -400).unwrap();
        assert_eq!((purse, boxed), (1000, 0));

        // Overdrawing either side is refused and changes nothing.
        assert!(matches!(
            store.move_storage_mesos(account, chr, 1001),
            Err(StoreError::NotEnoughMesos { have: 1000, .. })
        ));
        assert!(matches!(
            store.move_storage_mesos(account, chr, -1),
            Err(StoreError::NotEnoughMesos { have: 0, .. })
        ));
        assert_eq!((store.mesos(chr).unwrap(), store.storage_mesos(account).unwrap()), (1000, 0));
    }

    #[test]
    fn a_bundle_stacks_in_storage_and_a_shrink_that_would_strand_it_is_refused() {
        let (store, account, _, _) = store_with_two_characters();
        store.set_storage_slots(account, 10).unwrap();
        store.storage_deposit(account, &Item::bundle(2000000, 60), 100).unwrap();
        store.storage_deposit(account, &Item::bundle(2000000, 60), 100).unwrap();
        let boxed = store.storage(account).unwrap();
        let counts: Vec<u16> = boxed.items.iter().map(|i| i.item.kind.quantity()).collect();
        assert_eq!(counts, vec![100, 20]);

        assert!(matches!(
            store.set_storage_slots(account, 1),
            Err(StoreError::SlotOutOfRange { slot: 2, slots: 1 })
        ));
        assert_eq!(store.storage(account).unwrap().slots, 10);
    }

    /// Deleting the account takes its box with it.
    #[test]
    fn deleting_an_account_takes_its_storage() {
        let (store, account, _, _) = store_with_two_characters();
        store.storage_deposit(account, &Item::bundle(2000000, 1), 100).unwrap();
        let count = |t: &str| -> i64 {
            store
                .conn()
                .query_row(&format!("SELECT COUNT(*) FROM {t}"), [], |r| r.get(0))
                .unwrap()
        };
        assert_eq!((count("storage"), count("storage_item")), (1, 1));
        store.conn().execute("DELETE FROM accounts WHERE id = ?1", [account]).unwrap();
        assert_eq!((count("storage"), count("storage_item")), (0, 0));
    }

    /// Two accounts do not share a box.
    #[test]
    fn one_account_never_sees_anothers_storage() {
        let (store, account, _, _) = store_with_two_characters();
        let other = store.create_account("someone_else", "correct horse battery").unwrap();
        store.storage_deposit(account, &Item::bundle(2000000, 5), 100).unwrap();
        assert!(store.storage(other).unwrap().is_empty());
        assert_eq!(store.storage(account).unwrap().items.len(), 1);
    }
}
