//! **What a player has put up for sale in their store, held outside their bag until it sells or
//! the store closes.**
//!
//! The owner, 2026-10-04: *"I just tried to open a player store, but nothing happened."* The store
//! window (`world::session::playershop`) lists items the way the trade window does - they leave the
//! bag when they are listed, the client waiting on the inventory update that says so - so a line
//! here is a **move**: out of `inventory` and into `shop_escrow`, in one transaction, every
//! per-item column carried (a scrolled equip keeps its stats). A sale moves part or all of a line
//! into the buyer's bag and the price between the two wallets in ONE transaction; a closed store
//! puts every line back.
//!
//! **A table and not memory** for the reason `crate::tradeescrow` gives: a listed item held in
//! memory is an item a crash deletes. [`Store::return_shop_escrow`] gives back whatever a crash
//! left listed, at the owner's next login.
//!
//! A line sells in **bundles**: `per_bundle` units each, `bundles` of them left, `price` mesos a
//! bundle - the three numbers the store window's row carries (`net::playershop::Row`). An equip, a
//! pet or a star stack is one bundle of one: the whole item.

use rusqlite::types::Value;
use rusqlite::{params_from_iter, Connection};

use crate::db::Store;
use crate::error::{Result, StoreError};
use crate::inventory::{self, InvItem, InventoryType, Item, ItemKind};

/// **The sales fee: 3% of every sale, taken from the seller.** It is the client's own figure: the
/// store window tells its owner *"Please be aware that a 3% fee will be deducted from sales."*
/// (`0x1772`, formatted with a literal 3 at `FUN_140d98830`). Rounded down; nobody receives it.
pub const SALES_FEE_PERCENT: u64 = 3;

/// What the seller receives from a sale of `total` mesos.
pub fn proceeds_after_fee(total: u64) -> u64 {
    total - total * SALES_FEE_PERCENT / 100
}

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(&format!(
        r#"
        -- One row per line a store lists: the item (out of its owner's bag), the tab it came
        -- from (where it goes back to) and its terms. `line` only orders the rows; the store
        -- window addresses a line by its position in that order.
        CREATE TABLE IF NOT EXISTS shop_escrow (
            character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            line         INTEGER NOT NULL,
            inv_type     INTEGER NOT NULL,
            bundles      INTEGER NOT NULL,
            per_bundle   INTEGER NOT NULL,
            price        INTEGER NOT NULL,
            item_id      INTEGER NOT NULL,
            kind         INTEGER NOT NULL,
            quantity     INTEGER NOT NULL DEFAULT 1,
            {stats}
            PRIMARY KEY (character_id, line),
            CHECK (kind IN (1, 2)),
            CHECK (bundles >= 0),
            CHECK (per_bundle >= 1),
            CHECK (price >= 0),
            CHECK (quantity >= 0)
        );
        "#,
        stats = inventory::equip_stat_declarations(),
    ))?;
    inventory::add_equip_stat_columns(conn, "shop_escrow")?;
    allow_sold_out_lines(conn)?;
    conn.execute_batch(
        r#"
        -- A hired merchant: a store that stays on the map without its owner. Its shelf is the
        -- owner's `shop_escrow` lines; this row is where it stands and since when.
        CREATE TABLE IF NOT EXISTS hired_merchants (
            owner_id      INTEGER PRIMARY KEY REFERENCES characters(id) ON DELETE CASCADE,
            owner_account INTEGER NOT NULL,
            owner_name    TEXT    NOT NULL,
            title         TEXT    NOT NULL,
            permit        INTEGER NOT NULL,
            channel       INTEGER NOT NULL,
            map           INTEGER NOT NULL,
            instance      INTEGER NOT NULL,
            x             INTEGER NOT NULL,
            y             INTEGER NOT NULL,
            opened_at     INTEGER NOT NULL
        );
        "#,
    )?;
    Ok(())
}

/// **A sold-out line stays on the shelf** (the owner, 2026-10-04: *"when an item is sold, it should
/// remain as a row ... to display the price it was sold at"*), which is `bundles = 0` - and the first
/// version of this table, which reached the live database the same day, said `CHECK (bundles >= 1)`.
/// SQLite cannot alter a CHECK, so a table carrying the old one is rebuilt: renamed aside, created
/// fresh, its rows copied by column name, the old one dropped - one transaction. Idempotent: a
/// table without the old CHECK is left alone.
fn allow_sold_out_lines(conn: &Connection) -> Result<()> {
    let sql: String = conn.query_row("SELECT sql FROM sqlite_master WHERE type = 'table' AND name = 'shop_escrow'", [], |r| r.get(0))?;
    if !sql.contains("bundles >= 1") {
        return Ok(());
    }
    let columns: Vec<String> = {
        let mut stmt = conn.prepare("SELECT name FROM pragma_table_info('shop_escrow')")?;
        let names = stmt.query_map([], |r| r.get::<_, String>(0))?.collect::<rusqlite::Result<Vec<_>>>()?;
        names
    };
    let list = columns.join(", ");
    let tx = conn.unchecked_transaction()?;
    tx.execute_batch(&format!(
        "ALTER TABLE shop_escrow RENAME TO shop_escrow_before_sold_out;
         {create};
         INSERT INTO shop_escrow ({list}) SELECT {list} FROM shop_escrow_before_sold_out;
         DROP TABLE shop_escrow_before_sold_out;",
        create = sql.replace("bundles >= 1", "bundles >= 0"),
    ))?;
    tx.commit()?;
    Ok(())
}

/// **How long a hired merchant stands: 24 hours from its setup.** The item says so - *"It will be
/// automatically removed 24 hours after setup and must be set up again."* (the owner's screenshot
/// of Mushroom House Elf, 2026-10-04).
pub const HIRED_MERCHANT_SECS: i64 = 24 * 60 * 60;

/// One hired merchant standing somewhere.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HiredMerchant {
    pub owner_id: u32,
    pub owner_account: u32,
    pub owner_name: String,
    pub title: String,
    /// The Hired Merchant item (`503xxxx`), which picks the figure.
    pub permit: u32,
    pub channel: u32,
    pub map: u32,
    pub instance: u32,
    pub x: i16,
    pub y: i16,
    /// Unix seconds.
    pub opened_at: i64,
}

impl HiredMerchant {
    /// Whether it has stood its [`HIRED_MERCHANT_SECS`] at `now` (unix seconds).
    pub fn expired(&self, now: i64) -> bool {
        now >= self.opened_at + HIRED_MERCHANT_SECS
    }
}

fn merchant_from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<HiredMerchant> {
    Ok(HiredMerchant {
        owner_id: r.get::<_, i64>(0)? as u32,
        owner_account: r.get::<_, i64>(1)? as u32,
        owner_name: r.get(2)?,
        title: r.get(3)?,
        permit: r.get::<_, i64>(4)? as u32,
        channel: r.get::<_, i64>(5)? as u32,
        map: r.get::<_, i64>(6)? as u32,
        instance: r.get::<_, i64>(7)? as u32,
        x: r.get::<_, i64>(8)? as i16,
        y: r.get::<_, i64>(9)? as i16,
        opened_at: r.get(10)?,
    })
}

const MERCHANT_COLUMNS: &str = "owner_id, owner_account, owner_name, title, permit, channel, map, instance, x, y, opened_at";

/// One line of a store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShopLine {
    /// The tab it came out of, and goes back into.
    pub inv_type: InventoryType,
    /// Bundles still for sale. **0 is a sold-out line**, kept so the window can show what sold and
    /// at what price: it is never sold from, never given back, and its `item` is a picture of what
    /// was sold rather than anything held.
    pub bundles: u16,
    /// Units in one bundle. 1 for an item that sells whole.
    pub per_bundle: u16,
    /// Mesos for one bundle.
    pub price: u64,
    /// What is left: `bundles * per_bundle` units of a stack, or the whole item.
    pub item: Item,
}

impl ShopLine {
    pub fn sold_out(&self) -> bool {
        self.bundles == 0
    }
}

/// Whether `item` sells only whole - one bundle of one. An equip, a pet, and a star or bullet
/// stack (one item with its own serial, never split - `place_into_bag`).
pub fn sells_whole(item: &Item) -> bool {
    matches!(item.kind, ItemKind::Equip(_)) || item.pet_id.is_some() || net::bag::bundle_has_serial(item.item_id)
}

/// A finished sale.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Sale {
    /// What arrived in the buyer's bag, per tab, for the inventory packets.
    pub placed: Vec<(InventoryType, Vec<InvItem>)>,
    /// What the buyer paid.
    pub paid: u64,
    /// What reached the seller, after the fee.
    pub proceeds: u64,
    /// The item and the count that changed hands, for the log.
    pub item_id: u32,
    pub units: u32,
}

/// Everything given back by [`Store::return_shop_escrow`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Returned {
    /// The bag slots that changed, per tab, for the inventory packets.
    pub placed: Vec<(InventoryType, Vec<InvItem>)>,
    /// Lines that did not fit in the bag and are STILL held - given back at the next login.
    pub kept: usize,
}

fn read_lines(conn: &Connection, character_id: u32) -> Result<Vec<(i64, ShopLine)>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT line, inv_type, bundles, per_bundle, price, {} FROM shop_escrow WHERE character_id = ?1 ORDER BY line",
        inventory::item_columns().join(", ")
    ))?;
    let rows = stmt.query_map([i64::from(character_id)], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, i64>(3)?,
            row.get::<_, i64>(4)?,
            inventory::item_from_row(row, 5)?,
        ))
    })?;
    let mut out = Vec::new();
    for row in rows {
        let (line, inv, bundles, per_bundle, price, item) = row?;
        out.push((
            line,
            ShopLine {
                inv_type: InventoryType::from_wire(inv as i16)?,
                bundles: bundles.clamp(0, i64::from(u16::MAX)) as u16,
                per_bundle: per_bundle.clamp(0, i64::from(u16::MAX)) as u16,
                price: price.max(0) as u64,
                item,
            },
        ));
    }
    Ok(out)
}

fn line_at(conn: &Connection, character_id: u32, index: u16) -> Result<(i64, ShopLine)> {
    read_lines(conn, character_id)?
        .into_iter()
        .nth(usize::from(index))
        .ok_or(StoreError::ShopLineGone { index })
}

fn delete_line(conn: &Connection, character_id: u32, line: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM shop_escrow WHERE character_id = ?1 AND line = ?2",
        rusqlite::params![i64::from(character_id), line],
    )?;
    Ok(())
}

impl Store {
    /// **Bag -> a new line of this character's store, in one transaction.** `bundles` bundles of
    /// `per_bundle` units from `slot`, at `price` mesos a bundle; an item that [`sells_whole`]
    /// goes as one bundle of one whatever the counts say. Returns the line and how many are left
    /// in the bag slot, which is what the client's inventory packet says.
    ///
    /// Refuses, writing nothing, when the store already has `max_lines` lines, when the slot holds
    /// fewer units than asked for, or when a count is zero.
    #[allow(clippy::too_many_arguments)]
    pub fn list_shop_item(
        &self,
        character_id: u32,
        inv_type: InventoryType,
        slot: u16,
        bundles: u16,
        per_bundle: u16,
        price: u64,
        max_lines: u16,
    ) -> Result<(ShopLine, u16)> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let lines = read_lines(&tx, character_id)?;
        if lines.len() >= usize::from(max_lines) {
            return Err(StoreError::ShopFull { lines: lines.len() as u16 });
        }
        let Some(peek) = inventory::read_slot(&tx, character_id, inv_type, slot)? else {
            return Err(StoreError::SlotEmpty { slot });
        };
        let whole = sells_whole(&peek);
        let (bundles, per_bundle, count) = if whole {
            (1, 1, None)
        } else {
            let units = u32::from(bundles) * u32::from(per_bundle);
            let have = peek.kind.quantity();
            if units == 0 || units > u32::from(have) {
                return Err(StoreError::NotEnoughItems {
                    slot,
                    item_id: peek.item_id,
                    have,
                    want: units.min(u32::from(u16::MAX)) as u16,
                });
            }
            (bundles, per_bundle, Some(units as u16))
        };
        let taken = inventory::take_from_bag(&tx, character_id, inv_type, slot, count)?;
        let left = inventory::read_slot(&tx, character_id, inv_type, slot)?.map_or(0, |i| i.kind.quantity());
        let next = lines.last().map_or(1, |(l, _)| l + 1);
        let columns = inventory::item_columns();
        let placeholders: Vec<String> = (7..7 + columns.len()).map(|i| format!("?{i}")).collect();
        let mut values = vec![
            Value::Integer(i64::from(character_id)),
            Value::Integer(next),
            Value::Integer(i64::from(inv_type.as_u8())),
            Value::Integer(i64::from(bundles)),
            Value::Integer(i64::from(per_bundle)),
            Value::Integer(price.min(i64::MAX as u64) as i64),
        ];
        values.extend(inventory::item_values(&taken));
        tx.execute(
            &format!(
                "INSERT INTO shop_escrow (character_id, line, inv_type, bundles, per_bundle, price, {}) VALUES (?1, ?2, ?3, ?4, ?5, ?6, {})",
                columns.join(", "),
                placeholders.join(", ")
            ),
            params_from_iter(values),
        )?;
        tx.commit()?;
        Ok((ShopLine { inv_type, bundles, per_bundle, price, item: taken }, left))
    }

    /// This character's store, in the order its window draws it - sold-out lines included.
    pub fn shop_lines(&self, character_id: u32) -> Result<Vec<ShopLine>> {
        Ok(read_lines(&self.conn(), character_id)?.into_iter().map(|(_, l)| l).collect())
    }

    /// Whether anything on this character's shelf is still for sale.
    pub fn shop_has_stock(&self, character_id: u32) -> Result<bool> {
        Ok(read_lines(&self.conn(), character_id)?.iter().any(|(_, l)| !l.sold_out()))
    }

    /// **The line at row `index`, back into the bag** - the owner taking an item off the shelf.
    /// One transaction: if it does not fit, nothing moves.
    pub fn unlist_shop_item(
        &self,
        character_id: u32,
        index: u16,
        max_stack: &dyn Fn(u32) -> u16,
    ) -> Result<(InventoryType, Vec<InvItem>)> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let (line, l) = line_at(&tx, character_id, index)?;
        // A sold-out line is only a record: taking it down returns nothing.
        let changed = if l.sold_out() {
            Vec::new()
        } else {
            let mut bag = inventory::read_bag(&tx, character_id)?;
            inventory::place_into_bag(&tx, character_id, &mut bag, l.inv_type, &l.item, max_stack(l.item.item_id))?
        };
        delete_line(&tx, character_id, line)?;
        tx.commit()?;
        Ok((l.inv_type, changed))
    }

    /// **`buyer` buys `bundles` bundles of the line at row `index` of `seller`'s store.** One
    /// transaction: the units leave the line (the line goes when it is empty), land in the
    /// buyer's bag, the price leaves the buyer's wallet, and the price less
    /// [`SALES_FEE_PERCENT`] reaches the seller's.
    ///
    /// `expected_total`, when given, is the total the buyer's window computed and sent: a line
    /// that changed under it (sold out, taken back, the rows shifted) is refused rather than sold
    /// at a price nobody agreed to. Nothing moves on any refusal.
    pub fn buy_from_shop(
        &self,
        seller: u32,
        buyer: u32,
        index: u16,
        bundles: u16,
        expected_total: Option<u64>,
        max_stack: &dyn Fn(u32) -> u16,
    ) -> Result<Sale> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let (line, mut l) = line_at(&tx, seller, index)?;
        if bundles == 0 || bundles > l.bundles {
            return Err(StoreError::ShopStock { have: u32::from(l.bundles), want: u32::from(bundles) });
        }
        let total = u64::from(bundles) * l.price;
        if expected_total.is_some_and(|t| t != total) {
            return Err(StoreError::ShopLineGone { index });
        }
        let whole = sells_whole(&l.item);
        let units = if whole { 1 } else { u32::from(bundles) * u32::from(l.per_bundle) };
        let bought = if whole { l.item } else { Item::bundle(l.item.item_id, units as u16) };

        let paid = u32::try_from(total).map_err(|_| StoreError::NotEnoughMesos { have: 0, want: u32::MAX })?;
        inventory::adjust_mesos(&tx, buyer, -i64::from(paid))?;
        let proceeds = proceeds_after_fee(total);
        let seller_has = inventory::adjust_mesos(&tx, seller, 0)?;
        if u64::from(seller_has) + proceeds > u64::from(u32::MAX) {
            return Err(StoreError::WalletCap { have: seller_has, incoming: proceeds });
        }
        inventory::adjust_mesos(&tx, seller, proceeds as i64)?;

        let mut bag = inventory::read_bag(&tx, buyer)?;
        let changed = inventory::place_into_bag(&tx, buyer, &mut bag, l.inv_type, &bought, max_stack(bought.item_id))?;

        l.bundles -= bundles;
        // A line that sells out STAYS, at 0 bundles, so the window draws it SOLD OUT with its price.
        // A stack keeps one bundle's worth as its picture; a whole item keeps itself - neither is
        // held, because a sold-out line is never given back.
        let left = if l.bundles == 0 {
            if whole { u32::from(l.item.kind.quantity()) } else { u32::from(l.per_bundle) }
        } else {
            u32::from(l.bundles) * u32::from(l.per_bundle)
        };
        tx.execute(
            "UPDATE shop_escrow SET bundles = ?3, quantity = ?4 WHERE character_id = ?1 AND line = ?2",
            rusqlite::params![i64::from(seller), line, i64::from(l.bundles), i64::from(left)],
        )?;
        tx.commit()?;
        Ok(Sale { placed: vec![(l.inv_type, changed)], paid: total, proceeds, item_id: bought.item_id, units })
    }

    /// **A hired merchant is set up** (or its row replaced): from here it stands without its owner
    /// until it is closed or [`HiredMerchant::expired`].
    pub fn open_hired_merchant(&self, m: &HiredMerchant) -> Result<()> {
        self.conn().execute(
            &format!("INSERT OR REPLACE INTO hired_merchants ({MERCHANT_COLUMNS}) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)"),
            rusqlite::params![
                i64::from(m.owner_id),
                i64::from(m.owner_account),
                m.owner_name,
                m.title,
                i64::from(m.permit),
                i64::from(m.channel),
                i64::from(m.map),
                i64::from(m.instance),
                i64::from(m.x),
                i64::from(m.y),
                m.opened_at
            ],
        )?;
        Ok(())
    }

    /// This character's hired merchant, wherever it stands - expired or not; the caller decides.
    pub fn hired_merchant(&self, owner_id: u32) -> Result<Option<HiredMerchant>> {
        use rusqlite::OptionalExtension;
        Ok(self
            .conn()
            .query_row(&format!("SELECT {MERCHANT_COLUMNS} FROM hired_merchants WHERE owner_id = ?1"), [i64::from(owner_id)], merchant_from_row)
            .optional()?)
    }

    /// Every hired merchant standing on `channel`.
    pub fn hired_merchants_on_channel(&self, channel: u32) -> Result<Vec<HiredMerchant>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(&format!("SELECT {MERCHANT_COLUMNS} FROM hired_merchants WHERE channel = ?1 ORDER BY owner_id"))?;
        let rows = stmt.query_map([i64::from(channel)], merchant_from_row)?;
        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }

    /// The hired merchant is gone - closed by its owner or expired. Its shelf is NOT touched: the
    /// caller gives it back ([`Store::return_shop_escrow`]) now or at the owner's next login.
    pub fn close_hired_merchant(&self, owner_id: u32) -> Result<bool> {
        Ok(self.conn().execute("DELETE FROM hired_merchants WHERE owner_id = ?1", [i64::from(owner_id)])? > 0)
    }

    /// **Every line of this character's store, back into the bag** - the store closing, the owner
    /// leaving the channel, or what a crash left listed. One transaction per line, so one that does
    /// not fit stays held (and is counted in [`Returned::kept`]) without stopping the rest.
    pub fn return_shop_escrow(&self, character_id: u32, max_stack: &dyn Fn(u32) -> u16) -> Result<Returned> {
        let mut conn = self.conn();
        let mut out = Returned::default();
        for (line, l) in read_lines(&conn, character_id)? {
            if l.sold_out() {
                delete_line(&conn, character_id, line)?; // a record of a sale, not an item
                continue;
            }
            let tx = conn.transaction()?;
            let mut bag = inventory::read_bag(&tx, character_id)?;
            match inventory::place_into_bag(&tx, character_id, &mut bag, l.inv_type, &l.item, max_stack(l.item.item_id)) {
                Ok(changed) => {
                    delete_line(&tx, character_id, line)?;
                    tx.commit()?;
                    out.placed.push((l.inv_type, changed));
                }
                Err(StoreError::BagFull { .. }) => {
                    drop(tx); // rolled back: the line stays held
                    out.kept += 1;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn character(store: &Store, account: &str, name: &str) -> u32 {
        let account = store.create_account(account, "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: name.into(), ..Default::default() };
        store.create_character(account, 0, &chr).unwrap().id
    }

    /// A merchant row is written, found by owner and by channel, expires at 24 hours, and goes.
    #[test]
    fn a_hired_merchant_row_stands_for_a_day() {
        let store = Store::open_in_memory().unwrap();
        let id = character(&store, "seller", "Wisp");
        let m = HiredMerchant {
            owner_id: id,
            owner_account: 7,
            owner_name: "Wisp".into(),
            title: "Elf".into(),
            permit: 5_030_000,
            channel: 1,
            map: 910_000_001,
            instance: 0,
            x: -120,
            y: 34,
            opened_at: 1_000,
        };
        store.open_hired_merchant(&m).unwrap();
        assert_eq!(store.hired_merchant(id).unwrap(), Some(m.clone()));
        assert_eq!(store.hired_merchants_on_channel(1).unwrap(), vec![m.clone()]);
        assert!(store.hired_merchants_on_channel(0).unwrap().is_empty());
        assert!(!m.expired(1_000 + HIRED_MERCHANT_SECS - 1));
        assert!(m.expired(1_000 + HIRED_MERCHANT_SECS));
        assert!(store.close_hired_merchant(id).unwrap());
        assert_eq!(store.hired_merchant(id).unwrap(), None);
    }

    /// A database made by the first version - `CHECK (bundles >= 1)` - is rebuilt on open, its
    /// lines kept, and a sold-out line can then be written.
    #[test]
    fn the_old_bundles_check_is_rebuilt_and_its_lines_kept() {
        let store = Store::open_in_memory().unwrap();
        let id = character(&store, "seller", "Wisp");
        store.add_item(id, InventoryType::Etc, &Item::bundle(4_000_000, 10), 100).unwrap();
        let slot = store.bag_items(id, InventoryType::Etc).unwrap()[0].slot;
        store.list_shop_item(id, InventoryType::Etc, slot, 1, 10, 7, 16).unwrap();
        {
            let conn = store.conn();
            let sql: String = conn.query_row("SELECT sql FROM sqlite_master WHERE name = 'shop_escrow'", [], |r| r.get(0)).unwrap();
            let cols: Vec<String> = {
                let mut st = conn.prepare("SELECT name FROM pragma_table_info('shop_escrow')").unwrap();
                let v = st.query_map([], |r| r.get::<_, String>(0)).unwrap().collect::<rusqlite::Result<Vec<_>>>().unwrap();
                v
            };
            let list = cols.join(", ");
            conn.execute_batch(&format!(
                "ALTER TABLE shop_escrow RENAME TO t; {}; INSERT INTO shop_escrow ({list}) SELECT {list} FROM t; DROP TABLE t;",
                sql.replace("bundles >= 0", "bundles >= 1")
            ))
            .unwrap();
            allow_sold_out_lines(&conn).unwrap();
            allow_sold_out_lines(&conn).unwrap();
            let after: String = conn.query_row("SELECT sql FROM sqlite_master WHERE name = 'shop_escrow'", [], |r| r.get(0)).unwrap();
            assert!(after.contains("bundles >= 0") && !after.contains("bundles >= 1"));
        }
        assert_eq!(store.shop_lines(id).unwrap()[0].price, 7, "the line survived the rebuild");
        let buyer = character(&store, "buyer", "Pebble");
        store.set_mesos(buyer, 100).unwrap();
        store.buy_from_shop(id, buyer, 0, 1, None, &|_| 100).unwrap();
        assert!(store.shop_lines(id).unwrap()[0].sold_out());
    }

    #[test]
    fn the_fee_is_three_percent_rounded_down() {
        assert_eq!(proceeds_after_fee(1000), 970);
        assert_eq!(proceeds_after_fee(33), 33, "3% of 33 rounds down to nothing");
        assert_eq!(proceeds_after_fee(34), 33);
        assert_eq!(proceeds_after_fee(0), 0);
    }

    /// Part of a stack is listed in bundles; a buyer takes two bundles; the units, the price
    /// and the fee all land where they should, and the line keeps what is left.
    #[test]
    fn a_sale_moves_the_units_and_the_price_less_the_fee() {
        let store = Store::open_in_memory().unwrap();
        let seller = character(&store, "seller", "Wisp");
        let buyer = character(&store, "buyer", "Pebble");
        store.add_item(seller, InventoryType::Use, &Item::bundle(2_000_000, 100), 100).unwrap();
        store.set_mesos(buyer, 10_000).unwrap();
        let slot = store.bag_items(seller, InventoryType::Use).unwrap()[0].slot;

        let (line, left) = store.list_shop_item(seller, InventoryType::Use, slot, 5, 10, 300, 16).unwrap();
        assert_eq!((line.bundles, line.per_bundle, line.price, left), (5, 10, 300, 50));
        assert_eq!(line.item, Item::bundle(2_000_000, 50), "five bundles of ten left the bag");

        let sale = store.buy_from_shop(seller, buyer, 0, 2, Some(600), &|_| 100).unwrap();
        assert_eq!((sale.paid, sale.proceeds, sale.units), (600, 582, 20));
        assert_eq!(store.mesos(buyer).unwrap(), 9_400);
        assert_eq!(store.mesos(seller).unwrap(), 582);
        assert_eq!(store.bag_items(buyer, InventoryType::Use).unwrap()[0].item, Item::bundle(2_000_000, 20));
        let lines = store.shop_lines(seller).unwrap();
        assert_eq!((lines[0].bundles, lines[0].item.kind.quantity()), (3, 30), "three bundles of ten remain");

        // A price that is not the line's, more bundles than are left, a row that is not there.
        assert!(matches!(store.buy_from_shop(seller, buyer, 0, 1, Some(1), &|_| 100), Err(StoreError::ShopLineGone { .. })));
        assert!(matches!(store.buy_from_shop(seller, buyer, 0, 4, None, &|_| 100), Err(StoreError::ShopStock { .. })));
        assert!(matches!(store.buy_from_shop(seller, buyer, 1, 1, None, &|_| 100), Err(StoreError::ShopLineGone { .. })));

        // The last three bundles: the line stays, SOLD OUT, with its price - and nothing of it is
        // held: a close gives nothing back, and the record goes.
        store.buy_from_shop(seller, buyer, 0, 3, None, &|_| 100).unwrap();
        let lines = store.shop_lines(seller).unwrap();
        assert_eq!((lines.len(), lines[0].bundles, lines[0].price), (1, 0, 300));
        assert!(!store.shop_has_stock(seller).unwrap());
        assert!(matches!(store.buy_from_shop(seller, buyer, 0, 1, None, &|_| 100), Err(StoreError::ShopStock { .. })));
        assert_eq!(store.bag_items(buyer, InventoryType::Use).unwrap()[0].item, Item::bundle(2_000_000, 50));
        let back = store.return_shop_escrow(seller, &|_| 100).unwrap();
        assert!(back.placed.is_empty() && store.shop_lines(seller).unwrap().is_empty());
        assert_eq!(store.bag_items(seller, InventoryType::Use).unwrap()[0].item, Item::bundle(2_000_000, 50), "the 50 never listed, and nothing duplicated back");
    }

    /// A buyer who cannot pay, or whose bag cannot take it, moves nothing - not the item, not
    /// either wallet.
    #[test]
    fn a_refused_sale_moves_nothing() {
        let store = Store::open_in_memory().unwrap();
        let seller = character(&store, "seller", "Wisp");
        let buyer = character(&store, "buyer", "Pebble");
        store.add_item(seller, InventoryType::Equip, &Item::equip(1_302_000), 1).unwrap();
        let slot = store.bag_items(seller, InventoryType::Equip).unwrap()[0].slot;
        let (line, left) = store.list_shop_item(seller, InventoryType::Equip, slot, 7, 7, 5_000, 16).unwrap();
        assert_eq!((line.bundles, line.per_bundle, left), (1, 1, 0), "an equip sells whole, whatever the counts said");

        store.set_mesos(buyer, 4_999).unwrap();
        assert!(matches!(store.buy_from_shop(seller, buyer, 0, 1, None, &|_| 1), Err(StoreError::NotEnoughMesos { .. })));
        assert_eq!(store.shop_lines(seller).unwrap().len(), 1);
        assert_eq!((store.mesos(buyer).unwrap(), store.mesos(seller).unwrap()), (4_999, 0));

        store.set_mesos(buyer, 5_000).unwrap();
        let free = store.inventory_slots(buyer, InventoryType::Equip).unwrap();
        for _ in 0..free {
            store.add_item(buyer, InventoryType::Equip, &Item::equip(1_302_001), 1).unwrap();
        }
        assert!(matches!(store.buy_from_shop(seller, buyer, 0, 1, None, &|_| 1), Err(StoreError::BagFull { .. })));
        assert_eq!(store.shop_lines(seller).unwrap().len(), 1);
        assert_eq!((store.mesos(buyer).unwrap(), store.mesos(seller).unwrap()), (5_000, 0));
    }

    /// Lines come back to the bag whole - on a take-back, and on close - and a full store
    /// refuses another.
    #[test]
    fn lines_come_back_and_a_full_store_refuses() {
        let store = Store::open_in_memory().unwrap();
        let id = character(&store, "seller", "Wisp");
        store.add_item(id, InventoryType::Etc, &Item::bundle(4_000_000, 30), 100).unwrap();
        let slot = store.bag_items(id, InventoryType::Etc).unwrap()[0].slot;
        store.list_shop_item(id, InventoryType::Etc, slot, 1, 10, 1, 2).unwrap();
        store.list_shop_item(id, InventoryType::Etc, slot, 1, 10, 2, 2).unwrap();
        assert!(matches!(store.list_shop_item(id, InventoryType::Etc, slot, 1, 10, 3, 2), Err(StoreError::ShopFull { lines: 2 })));
        assert!(matches!(store.list_shop_item(id, InventoryType::Etc, slot, 2, 10, 3, 3), Err(StoreError::NotEnoughItems { .. })), "20 asked, 10 left");

        let (inv, changed) = store.unlist_shop_item(id, 0, &|_| 100).unwrap();
        assert_eq!((inv, changed[0].item), (InventoryType::Etc, Item::bundle(4_000_000, 20)));
        assert_eq!(store.shop_lines(id).unwrap()[0].price, 2, "row 1 is row 0 now");

        let back = store.return_shop_escrow(id, &|_| 100).unwrap();
        assert_eq!((back.placed.len(), back.kept), (1, 0));
        assert!(store.shop_lines(id).unwrap().is_empty());
        assert_eq!(store.bag_items(id, InventoryType::Etc).unwrap()[0].item, Item::bundle(4_000_000, 30), "all thirty home");
    }
}
