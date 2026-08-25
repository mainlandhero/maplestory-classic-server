//! The cash wallet and the cash locker: the two pieces of state a cash shop needs.
//!
//! The owner, 2026-08-22: *"Please build the Cash Shop, make sure the experience is functional."*
//!
//! # Why this is per ACCOUNT and not per character
//!
//! The same reason storage is (`crate::storage`, and `research/storage.md` §9): a wallet a
//! player tops up on one character and cannot spend on another is not a wallet. It is also
//! what this game family does. Unlike storage's ownership question, nothing here is inferred
//! from the wire - **no packet has been decoded yet that carries an owner either way** - so
//! this is a design decision, taken deliberately, and it is recorded rather than discovered.
//!
//! # Two currencies, and they are not interchangeable
//!
//! [`CashWallet`] holds **NX** and **maple points** separately because the client's shop UI
//! shows two balances and prices are quoted against one of them. Collapsing them into one
//! number would make a purchase that should have been refused succeed, which is the direction
//! that costs the player.
//!
//! # The locker is not the Cash inventory tab
//!
//! A bought item lands in the **locker**, and the player moves it into the Cash tab as a
//! separate action. They are different places with different capacities, and conflating them
//! would let a purchase fail because a bag was full - which is not how this shop works.
//! The locker reuses `inventory`'s item columns exactly, so an item crossing between the
//! locker and a bag keeps its per-item stats, the same property `storage_item` has.

use rusqlite::types::Value;
use rusqlite::{params_from_iter, Connection, OptionalExtension};

use crate::inventory::{item_columns, item_from_row, item_values, Item};
use crate::{Result, Store, StoreError};

/// What a brand-new account has to spend.
///
/// **Zero, and that is deliberate.** A test server that hands out currency by existing makes
/// every later "did the purchase deduct?" question unanswerable, because the balance moves for
/// reasons nobody is tracking. `!nx` grants it explicitly instead, so every credit has a cause.
pub const DEFAULT_NX: u32 = 0;

/// Slots in a cash locker.
///
/// **[I].** Nothing decoded says what the real limit is; this is a round number that is
/// comfortably larger than anything a test session will buy, named so that changing it is one
/// edit rather than a scatter of literals.
pub const LOCKER_SLOTS: u16 = 100;

/// An account's two balances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CashWallet {
    pub nx: u32,
    pub maple_points: u32,
}

/// One occupied locker slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LockerItem {
    /// **1-based**, the same rule the bag and storage use.
    pub slot: u16,
    pub item: Item,
}

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(&format!(
        r#"
        -- One row per account that has ever had a balance. Absent means "zero", which is the
        -- same thing as a zero row - so opening the shop creates no state.
        CREATE TABLE IF NOT EXISTS cash_wallet (
            account_id    INTEGER PRIMARY KEY REFERENCES accounts(id) ON DELETE CASCADE,
            nx            INTEGER NOT NULL DEFAULT 0,
            maple_points  INTEGER NOT NULL DEFAULT 0,
            CHECK (nx >= 0),
            CHECK (maple_points >= 0)
        );

        -- One row per OCCUPIED locker slot. Exactly the item columns `inventory` uses, so an
        -- item moving between the locker and the Cash tab keeps its per-item stats.
        CREATE TABLE IF NOT EXISTS cash_locker (
            account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
            -- 1-BASED, the same rule the bag and storage use.
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

        CREATE INDEX IF NOT EXISTS idx_cash_locker_account ON cash_locker(account_id);
        "#,
        // **The same declarations `storage_item` uses, from the same generator.** The equip
        // stat columns are NULLABLE - `item_values` writes NULL for a bundle - and declaring
        // them `NOT NULL` here made every purchase fail on a constraint. Sharing the
        // generator rather than re-typing the column list is what stops that recurring.
        stats = crate::inventory::equip_stat_declarations(),
    ))?;
    Ok(())
}

fn read_wallet(conn: &Connection, account_id: i64) -> Result<CashWallet> {
    let row: Option<(i64, i64)> = conn
        .query_row(
            "SELECT nx, maple_points FROM cash_wallet WHERE account_id = ?1",
            [account_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?;
    Ok(match row {
        Some((nx, mp)) => {
            CashWallet { nx: nx.max(0) as u32, maple_points: mp.max(0) as u32 }
        }
        None => CashWallet { nx: DEFAULT_NX, maple_points: 0 },
    })
}

fn read_locker(conn: &Connection, account_id: i64) -> Result<Vec<LockerItem>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT slot, {} FROM cash_locker WHERE account_id = ?1 ORDER BY slot",
        item_columns().join(", ")
    ))?;
    let rows = stmt.query_map([account_id], |row| {
        Ok(LockerItem { slot: row.get::<_, i64>(0)? as u16, item: item_from_row(row, 1)? })
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// Insert one occupied locker row. Shared by the purchase and by the rollback so the two
/// cannot drift on the column list - the same reason `create_tables` borrows
/// `inventory::equip_stat_declarations` rather than re-typing it.
fn insert_locker_row(
    conn: &Connection,
    account_id: i64,
    slot: u16,
    item: &Item,
) -> rusqlite::Result<usize> {
    let cols = item_columns();
    let placeholders: Vec<String> = (3..3 + cols.len()).map(|i| format!("?{i}")).collect();
    let sql = format!(
        "INSERT INTO cash_locker (account_id, slot, {}) VALUES (?1, ?2, {})",
        cols.join(", "),
        placeholders.join(", ")
    );
    let mut values = vec![Value::Integer(account_id), Value::Integer(i64::from(slot))];
    values.extend(item_values(item));
    conn.execute(&sql, params_from_iter(values))
}

fn lowest_free(locker: &[LockerItem]) -> Option<u16> {
    let taken: std::collections::HashSet<u16> = locker.iter().map(|i| i.slot).collect();
    (1..=LOCKER_SLOTS).find(|s| !taken.contains(s))
}

impl Store {
    /// The account's balances. An account that has never had one reads as zero.
    pub fn cash_wallet(&self, account_id: i64) -> Result<CashWallet> {
        read_wallet(&self.conn(), account_id)
    }

    /// Add to (or, with a negative delta, take from) the NX balance.
    ///
    /// Returns the new balance. Refuses rather than clamping when the account cannot afford
    /// it: a silent clamp to zero is how a purchase succeeds for free.
    pub fn add_nx(&self, account_id: i64, delta: i64) -> Result<u32> {
        let conn = self.conn();
        let have = read_wallet(&conn, account_id)?.nx;
        let next = i64::from(have) + delta;
        if next < 0 {
            return Err(StoreError::NotEnoughMesos {
                have,
                want: delta.unsigned_abs().min(u64::from(u32::MAX)) as u32,
            });
        }
        conn.execute(
            "INSERT INTO cash_wallet (account_id, nx, maple_points) VALUES (?1, ?2, 0)
             ON CONFLICT(account_id) DO UPDATE SET nx = ?2",
            rusqlite::params![account_id, next],
        )?;
        Ok(next as u32)
    }

    /// The account's locker, occupied slots only, in slot order.
    pub fn cash_locker(&self, account_id: i64) -> Result<Vec<LockerItem>> {
        read_locker(&self.conn(), account_id)
    }

    /// **Buy: take the price and place the item, or do neither.**
    ///
    /// One transaction, for the reason `crate::storage::store_item` is one: an item that is
    /// briefly bought-but-not-placed is an item a crash loses, and a price taken without an
    /// item is worse.
    ///
    /// The order inside it is the rule `CLAUDE.md` records under the Heena quest - **every
    /// effect hangs off the transition**. The balance check, the debit and the placement are
    /// all inside the same `tx`, and any failure rolls the whole thing back rather than
    /// leaving one half done.
    pub fn buy_cash_item(&self, account_id: i64, item: &Item, price: u32) -> Result<LockerItem> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;

        let wallet = read_wallet(&tx, account_id)?;
        if wallet.nx < price {
            return Err(StoreError::NotEnoughMesos { have: wallet.nx, want: price });
        }
        let mut locker = read_locker(&tx, account_id)?;
        let slot = lowest_free(&locker).ok_or(StoreError::StorageFull { slots: LOCKER_SLOTS })?;

        tx.execute(
            "INSERT INTO cash_wallet (account_id, nx, maple_points) VALUES (?1, ?2, 0)
             ON CONFLICT(account_id) DO UPDATE SET nx = ?2",
            rusqlite::params![account_id, i64::from(wallet.nx - price)],
        )?;

        insert_locker_row(&tx, account_id, slot, item)?;

        // Read it back before committing. An INSERT that collided would leave the balance
        // debited and no item, which is the one outcome this function must never produce.
        locker = read_locker(&tx, account_id)?;
        let placed = locker
            .iter()
            .find(|l| l.slot == slot)
            .copied()
            .ok_or(StoreError::SlotEmpty { slot })?;
        tx.commit()?;
        Ok(placed)
    }

    /// Put an item into the lowest free locker slot **without charging for it**.
    ///
    /// This is the rollback half of moving an item out of the locker and into a bag. Those
    /// are two stores with two transactions, so the move cannot be one - and an item that
    /// leaves the locker and then fails to reach the bag would simply cease to exist.
    /// `CLAUDE.md`'s Heena rule from the other side: the caller must be able to undo the
    /// half that did happen.
    ///
    /// It does **not** restore the original slot number. The slot is not an identity - the
    /// locker is a bag of items in slot order - and demanding the old one back could fail
    /// where the lowest free one succeeds, which is the wrong direction for a rollback.
    pub fn put_cash_item(&self, account_id: i64, item: &Item) -> Result<LockerItem> {
        let conn = self.conn();
        let locker = read_locker(&conn, account_id)?;
        let slot = lowest_free(&locker).ok_or(StoreError::StorageFull { slots: LOCKER_SLOTS })?;
        insert_locker_row(&conn, account_id, slot, item)?;
        read_locker(&conn, account_id)?
            .into_iter()
            .find(|l| l.slot == slot)
            .ok_or(StoreError::SlotEmpty { slot })
    }

    /// Take an item out of the locker. The caller places it in a bag.
    pub fn take_cash_item(&self, account_id: i64, slot: u16) -> Result<Item> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let locker = read_locker(&tx, account_id)?;
        let found = locker
            .iter()
            .find(|l| l.slot == slot)
            .copied()
            .ok_or(StoreError::SlotEmpty { slot })?;
        tx.execute(
            "DELETE FROM cash_locker WHERE account_id = ?1 AND slot = ?2",
            rusqlite::params![account_id, slot],
        )?;
        tx.commit()?;
        Ok(found.item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> (Store, i64) {
        let st = Store::open_in_memory().unwrap();
        let account = st.create_account("maplecw", "correct horse battery").unwrap();
        (st, account)
    }

    /// A new account has nothing, and asking creates no row.
    #[test]
    fn a_new_account_has_an_empty_wallet_and_an_empty_locker() {
        let (st, account) = store();
        assert_eq!(st.cash_wallet(account).unwrap(), CashWallet { nx: 0, maple_points: 0 });
        assert!(st.cash_locker(account).unwrap().is_empty());
    }

    /// **A purchase takes the price and places the item, and a refused one does neither.**
    ///
    /// Both halves asserted. A test that only checked the item would pass while the balance
    /// silently never moved - the shape of the repeated-quest bug.
    #[test]
    fn buying_debits_the_wallet_and_fills_a_locker_slot() {
        let (st, account) = store();
        st.add_nx(account, 5_000).unwrap();

        let placed = st.buy_cash_item(account, &Item::equip(1302000), 1_200).unwrap();
        assert_eq!(placed.slot, 1);
        assert_eq!(placed.item.item_id, 1302000);
        assert_eq!(st.cash_wallet(account).unwrap().nx, 3_800, "the price came out");
        assert_eq!(st.cash_locker(account).unwrap().len(), 1);

        let second = st.buy_cash_item(account, &Item::bundle(5000000, 1), 800).unwrap();
        assert_eq!(second.slot, 2, "the next free slot");
        assert_eq!(st.cash_wallet(account).unwrap().nx, 3_000);
    }

    /// **An unaffordable purchase changes nothing at all.**
    #[test]
    fn a_purchase_that_cannot_be_afforded_leaves_both_sides_untouched() {
        let (st, account) = store();
        st.add_nx(account, 500).unwrap();

        let e = st.buy_cash_item(account, &Item::equip(1302000), 1_200).unwrap_err();
        assert!(matches!(e, StoreError::NotEnoughMesos { have: 500, want: 1_200 }), "{e:?}");
        assert_eq!(st.cash_wallet(account).unwrap().nx, 500, "not debited");
        assert!(st.cash_locker(account).unwrap().is_empty(), "and nothing placed");
    }

    /// NX cannot be spent below zero, and the refusal does not clamp.
    #[test]
    fn nx_cannot_go_negative() {
        let (st, account) = store();
        st.add_nx(account, 100).unwrap();
        assert!(st.add_nx(account, -200).is_err());
        assert_eq!(st.cash_wallet(account).unwrap().nx, 100, "a refused debit is not a clamp");
        assert_eq!(st.add_nx(account, -100).unwrap(), 0, "and exactly enough is allowed");
    }

    /// An item taken out of the locker really leaves it, and taking twice fails.
    #[test]
    fn taking_from_the_locker_empties_the_slot() {
        let (st, account) = store();
        st.add_nx(account, 5_000).unwrap();
        st.buy_cash_item(account, &Item::equip(1302000), 100).unwrap();

        let item = st.take_cash_item(account, 1).unwrap();
        assert_eq!(item.item_id, 1302000);
        assert!(st.cash_locker(account).unwrap().is_empty());
        assert!(st.take_cash_item(account, 1).is_err(), "and it is gone, not duplicable");
    }

    /// **An item taken out and put back is not lost, and is not charged for twice.**
    ///
    /// This is the rollback path for moving a cash item into a bag, and the thing it has to
    /// guarantee is that the undo costs nothing: `put_cash_item` must not touch the wallet.
    /// Both halves are asserted, because a test that only counted the item would pass while
    /// the balance quietly moved.
    #[test]
    fn an_item_can_be_put_back_without_paying_for_it_again() {
        let (st, account) = store();
        st.add_nx(account, 1_000).unwrap();
        st.buy_cash_item(account, &Item::equip(1302000), 700).unwrap();
        assert_eq!(st.cash_wallet(account).unwrap().nx, 300);

        let item = st.take_cash_item(account, 1).unwrap();
        assert!(st.cash_locker(account).unwrap().is_empty());

        let back = st.put_cash_item(account, &item).unwrap();
        assert_eq!(back.item.item_id, 1302000);
        assert_eq!(st.cash_locker(account).unwrap().len(), 1);
        assert_eq!(st.cash_wallet(account).unwrap().nx, 300, "the undo is free");
    }

    /// **The wallet is shared by every character on the account**, like storage.
    #[test]
    fn the_wallet_is_per_account() {
        let (st, account) = store();
        let other = st.create_account("second", "correct horse battery").unwrap();
        st.add_nx(account, 1_000).unwrap();
        assert_eq!(st.cash_wallet(account).unwrap().nx, 1_000);
        assert_eq!(st.cash_wallet(other).unwrap().nx, 0, "and one account never sees another's");
    }

    /// Deleting an account takes its wallet and locker with it.
    #[test]
    fn deleting_an_account_takes_its_cash() {
        let (st, account) = store();
        st.add_nx(account, 1_000).unwrap();
        st.buy_cash_item(account, &Item::equip(1302000), 100).unwrap();
        st.conn().execute("DELETE FROM accounts WHERE id = ?1", [account]).unwrap();
        assert!(st.cash_locker(account).unwrap().is_empty());
        assert_eq!(st.cash_wallet(account).unwrap().nx, 0);
    }
}
