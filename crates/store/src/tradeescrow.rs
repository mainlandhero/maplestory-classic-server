//! **What a player has put into a trade window, held outside their bag until the trade ends.**
//!
//! The owner, 2026-10-03: *"I believe the vanilla behavior should be that the item disappears
//! from the player's inventory into the trade window until the conclusion of the trade request
//! (either canceled which then the item gets returned, or given to the counterparty when it is
//! successful)."* And it is not only the look: after one stack went in, the client let nothing
//! else be dragged, because a put waits for the inventory update that takes the item out of the
//! bag (`bExclRequestSent`), and none came.
//!
//! So a put **moves** the item: out of `inventory` and into `trade_escrow`, in one transaction,
//! with every per-item column carried (the same columns storage uses, so a scrolled equip keeps
//! its stats). Mesos move from `characters.mesos` into `trade_escrow_mesos` the same way.
//!
//! **Why a table and not memory**: an item in memory is an item a crash deletes. A row here
//! survives one, and [`Store::return_trade_escrow`] gives it back - at the end of every trade
//! that does not complete, and at the owner's next login for anything a crash left behind.

use rusqlite::types::Value;
use rusqlite::{params_from_iter, Connection, OptionalExtension};

use crate::db::Store;
use crate::error::{Result, StoreError};
use crate::inventory::{self, InvItem, InventoryType, Item};

/// The trade grid's size, `net::trade::TRADE_SLOTS`.
pub const TRADE_SLOTS: u8 = 9;

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(&format!(
        r#"
        -- One row per occupied trade slot: the item, out of its owner's bag, and the tab it
        -- came from (where it goes back to).
        CREATE TABLE IF NOT EXISTS trade_escrow (
            character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            trade_slot   INTEGER NOT NULL,
            inv_type     INTEGER NOT NULL,
            item_id      INTEGER NOT NULL,
            kind         INTEGER NOT NULL,
            quantity     INTEGER NOT NULL DEFAULT 1,
            {stats}
            PRIMARY KEY (character_id, trade_slot),
            CHECK (trade_slot BETWEEN 1 AND {slots}),
            CHECK (kind IN (1, 2)),
            CHECK (quantity >= 0)
        );

        -- The mesos one side has on the table.
        CREATE TABLE IF NOT EXISTS trade_escrow_mesos (
            character_id INTEGER PRIMARY KEY REFERENCES characters(id) ON DELETE CASCADE,
            mesos        INTEGER NOT NULL,
            CHECK (mesos >= 0)
        );
        "#,
        stats = inventory::equip_stat_declarations(),
        slots = TRADE_SLOTS,
    ))?;
    inventory::add_equip_stat_columns(conn, "trade_escrow")?;
    Ok(())
}

/// One line of a side's offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Escrowed {
    pub trade_slot: u8,
    /// The tab it came out of, and goes back into.
    pub inv_type: InventoryType,
    pub item: Item,
}

/// Everything given back by [`Store::return_trade_escrow`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Returned {
    /// The bag slots that changed, per tab, for the inventory packets.
    pub placed: Vec<(InventoryType, Vec<InvItem>)>,
    /// Mesos back in the wallet.
    pub mesos: u32,
    /// Lines that did not fit in the bag and are STILL held - given back at the next login.
    pub kept: usize,
}

fn escrow_mesos(conn: &Connection, character_id: u32) -> Result<u32> {
    let m: Option<i64> = conn
        .query_row("SELECT mesos FROM trade_escrow_mesos WHERE character_id = ?1", [i64::from(character_id)], |r| r.get(0))
        .optional()?;
    Ok(m.unwrap_or(0).clamp(0, i64::from(u32::MAX)) as u32)
}

fn read_escrow(conn: &Connection, character_id: u32) -> Result<Vec<Escrowed>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT trade_slot, inv_type, {} FROM trade_escrow WHERE character_id = ?1 ORDER BY trade_slot",
        inventory::item_columns().join(", ")
    ))?;
    let rows = stmt.query_map([i64::from(character_id)], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?, inventory::item_from_row(row, 2)?))
    })?;
    let mut out = Vec::new();
    for row in rows {
        let (slot, inv, item) = row?;
        out.push(Escrowed { trade_slot: slot as u8, inv_type: InventoryType::from_wire(inv as i16)?, item });
    }
    Ok(out)
}

impl Store {
    /// **Bag -> trade slot `trade_slot`, in one transaction.** `count` of the stack in `slot`
    /// (`None` the whole slot; a star or bullet stack always goes whole). Returns what went in
    /// and how many are left in the bag slot, which is what the client's inventory packet says.
    ///
    /// Refuses, writing nothing, when that trade slot is already taken.
    pub fn escrow_trade_item(
        &self,
        character_id: u32,
        inv_type: InventoryType,
        slot: u16,
        count: Option<u16>,
        trade_slot: u8,
    ) -> Result<(Item, u16)> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let taken_slot: Option<i64> = tx
            .query_row(
                "SELECT 1 FROM trade_escrow WHERE character_id = ?1 AND trade_slot = ?2",
                rusqlite::params![i64::from(character_id), i64::from(trade_slot)],
                |r| r.get(0),
            )
            .optional()?;
        if taken_slot.is_some() || !(1..=TRADE_SLOTS).contains(&trade_slot) {
            return Err(StoreError::SlotOccupied { slot: u16::from(trade_slot) });
        }
        let Some(peek) = inventory::read_slot(&tx, character_id, inv_type, slot)? else {
            return Err(StoreError::SlotEmpty { slot });
        };
        let count = if net::bag::bundle_has_serial(peek.item_id) { None } else { count };
        let taken = inventory::take_from_bag(&tx, character_id, inv_type, slot, count)?;
        let left = inventory::read_slot(&tx, character_id, inv_type, slot)?.map_or(0, |i| i.kind.quantity());
        let columns = inventory::item_columns();
        let placeholders: Vec<String> = (4..4 + columns.len()).map(|i| format!("?{i}")).collect();
        let mut values = vec![
            Value::Integer(i64::from(character_id)),
            Value::Integer(i64::from(trade_slot)),
            Value::Integer(i64::from(inv_type.as_u8())),
        ];
        values.extend(inventory::item_values(&taken));
        tx.execute(
            &format!(
                "INSERT INTO trade_escrow (character_id, trade_slot, inv_type, {}) VALUES (?1, ?2, ?3, {})",
                columns.join(", "),
                placeholders.join(", ")
            ),
            params_from_iter(values),
        )?;
        tx.commit()?;
        Ok((taken, left))
    }

    /// **Set this side's mesos on the table to `total`**, moving the difference between the
    /// wallet and the escrow in one transaction. Returns the new wallet. Refuses, moving
    /// nothing, when the wallet cannot cover the increase.
    pub fn escrow_trade_mesos(&self, character_id: u32, total: u32) -> Result<u32> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let held = escrow_mesos(&tx, character_id)?;
        let wallet = inventory::adjust_mesos(&tx, character_id, i64::from(held) - i64::from(total))?;
        if total == 0 {
            tx.execute("DELETE FROM trade_escrow_mesos WHERE character_id = ?1", [i64::from(character_id)])?;
        } else {
            tx.execute(
                "INSERT INTO trade_escrow_mesos (character_id, mesos) VALUES (?1, ?2)
                 ON CONFLICT(character_id) DO UPDATE SET mesos = excluded.mesos",
                rusqlite::params![i64::from(character_id), i64::from(total)],
            )?;
        }
        tx.commit()?;
        Ok(wallet)
    }

    /// This side's offer: the items by trade slot, and the mesos.
    pub fn trade_escrow(&self, character_id: u32) -> Result<(Vec<Escrowed>, u32)> {
        let conn = self.conn();
        Ok((read_escrow(&conn, character_id)?, escrow_mesos(&conn, character_id)?))
    }

    /// **Give back everything this character has on a trade table** - a cancelled trade, a
    /// dropped connection, or what a crash left behind. Each item goes back into the tab it
    /// came from (stacking where it can, as any item entering the bag does); the mesos go
    /// back to the wallet. One transaction per item, so one that does not fit stays held
    /// (and is counted in [`Returned::kept`]) without stopping the rest.
    ///
    /// `max_stack` is the item's `info/slotMax`, which this crate does not know.
    pub fn return_trade_escrow(&self, character_id: u32, max_stack: &dyn Fn(u32) -> u16) -> Result<Returned> {
        let mut conn = self.conn();
        let mut out = Returned::default();
        for line in read_escrow(&conn, character_id)? {
            let tx = conn.transaction()?;
            let mut bag = inventory::read_bag(&tx, character_id)?;
            match inventory::place_into_bag(&tx, character_id, &mut bag, line.inv_type, &line.item, max_stack(line.item.item_id)) {
                Ok(changed) => {
                    tx.execute(
                        "DELETE FROM trade_escrow WHERE character_id = ?1 AND trade_slot = ?2",
                        rusqlite::params![i64::from(character_id), i64::from(line.trade_slot)],
                    )?;
                    tx.commit()?;
                    out.placed.push((line.inv_type, changed));
                }
                Err(StoreError::BagFull { .. }) => {
                    drop(tx); // rolled back: the line stays held
                    out.kept += 1;
                }
                Err(e) => return Err(e),
            }
        }
        let tx = conn.transaction()?;
        let mesos = escrow_mesos(&tx, character_id)?;
        if mesos > 0 {
            inventory::adjust_mesos(&tx, character_id, i64::from(mesos))?;
            tx.execute("DELETE FROM trade_escrow_mesos WHERE character_id = ?1", [i64::from(character_id)])?;
        }
        tx.commit()?;
        out.mesos = mesos;
        Ok(out)
    }
}

/// **The trade fee: 5% of mesos received.** The owner, 2026-10-03, quoting the trade window's own
/// notice: *"Please be aware that a 5% fee will be deducted from meso transactions."* Taken
/// from the RECEIVING side, rounded down, which is what the client's completion message
/// reports - *"Received %lld mesos after fees"* (`0x01CF`), the amount it computes from its own
/// wallet rising (`net::trade::LEAVE_TRADE_DONE`).
pub const TRADE_FEE_PERCENT: u64 = 5;

/// What arrives from `offered` mesos after the fee.
pub fn mesos_after_fee(offered: u32) -> u32 {
    let offered = u64::from(offered);
    (offered - offered * TRADE_FEE_PERCENT / 100) as u32
}

/// What one side of a completed trade received.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Received {
    /// The bag slots that changed, per tab, for the inventory packets.
    pub placed: Vec<(InventoryType, Vec<InvItem>)>,
    /// Mesos into the wallet, after the fee.
    pub mesos: u32,
    /// The new wallet.
    pub wallet: u32,
}

impl Store {
    /// **The trade completes: each side's offer goes to the other, in ONE transaction.**
    /// Items land in the receiver's bag in the tab they came out of; mesos land less the
    /// [`TRADE_FEE_PERCENT`] fee, which nobody receives. Both escrows are emptied.
    ///
    /// All or nothing: if either bag cannot take what is coming, or a wallet would pass the
    /// meso cap, nothing moves and the escrows stay as they were, for the caller to return
    /// (`Store::return_trade_escrow`) and say why. **Not wired yet** - the Trade button's packet
    /// has not been captured (`world::session::trade`).
    pub fn complete_trade(
        &self,
        a: u32,
        b: u32,
        max_stack: &dyn Fn(u32) -> u16,
    ) -> Result<(Received, Received)> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let (a_items, a_mesos) = (read_escrow(&tx, a)?, escrow_mesos(&tx, a)?);
        let (b_items, b_mesos) = (read_escrow(&tx, b)?, escrow_mesos(&tx, b)?);
        let give = |to: u32, items: &[Escrowed], mesos: u32| -> Result<Received> {
            let mut bag = inventory::read_bag(&tx, to)?;
            let mut got = Received::default();
            for line in items {
                let changed = inventory::place_into_bag(&tx, to, &mut bag, line.inv_type, &line.item, max_stack(line.item.item_id))?;
                got.placed.push((line.inv_type, changed));
            }
            got.mesos = mesos_after_fee(mesos);
            let before = inventory::adjust_mesos(&tx, to, 0)?;
            if u64::from(before) + u64::from(got.mesos) > u64::from(u32::MAX) {
                return Err(StoreError::NotEnoughMesos { have: before, want: got.mesos });
            }
            got.wallet = inventory::adjust_mesos(&tx, to, i64::from(got.mesos))?;
            Ok(got)
        };
        let to_b = give(b, &a_items, a_mesos)?;
        let to_a = give(a, &b_items, b_mesos)?;
        for id in [a, b] {
            tx.execute("DELETE FROM trade_escrow WHERE character_id = ?1", [i64::from(id)])?;
            tx.execute("DELETE FROM trade_escrow_mesos WHERE character_id = ?1", [i64::from(id)])?;
        }
        tx.commit()?;
        Ok((to_a, to_b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 5%, rounded down, taken from what arrives.
    #[test]
    fn the_fee_is_five_percent_of_what_is_received() {
        assert_eq!(mesos_after_fee(3000), 2850);
        assert_eq!(mesos_after_fee(19), 19, "5% of 19 rounds down to nothing");
        assert_eq!(mesos_after_fee(20), 19);
        assert_eq!(mesos_after_fee(0), 0);
        assert_eq!(mesos_after_fee(u32::MAX), u32::MAX - (u64::from(u32::MAX) * 5 / 100) as u32);
    }

    /// Both offers cross in one step: items into the other bag, mesos less the fee, and both
    /// escrows empty. When a bag cannot take its side, NOTHING moves.
    #[test]
    fn a_completed_trade_swaps_both_offers_with_the_fee() {
        let store = Store::open_in_memory().unwrap();
        let a = character(&store);
        let account = store.create_account("other", "correct horse battery").unwrap();
        let b = store.create_character(account, 0, &net::opcode::Character { name: "Pebble".into(), ..Default::default() }).unwrap().id;
        store.set_mesos(a, 5000).unwrap();
        store.add_item(b, InventoryType::Use, &Item::bundle(2_060_000, 1000), 1000).unwrap();
        let slot = store.bag_items(b, InventoryType::Use).unwrap()[0].slot;
        store.escrow_trade_mesos(a, 3000).unwrap();
        store.escrow_trade_item(b, InventoryType::Use, slot, None, 1).unwrap();

        let (to_a, to_b) = store.complete_trade(a, b, &|_| 1000).unwrap();
        assert_eq!((to_b.mesos, to_b.wallet), (2850, 2850), "3000 less 150");
        assert_eq!(to_a.mesos, 0);
        assert_eq!(store.mesos(a).unwrap(), 2000);
        assert_eq!(store.bag_items(a, InventoryType::Use).unwrap()[0].item, Item::bundle(2_060_000, 1000));
        assert!(store.bag_items(b, InventoryType::Use).unwrap().is_empty());
        assert_eq!(store.trade_escrow(a).unwrap(), (Vec::new(), 0));
        assert_eq!(store.trade_escrow(b).unwrap(), (Vec::new(), 0));

        // A bag with no room: a's Use tab filled with single potions, and b offers one more
        // kind of item. Nothing may move - not even a's mesos to b.
        let free = store.inventory_slots(a, InventoryType::Use).unwrap() - 1;
        for i in 0..u32::from(free) {
            store.add_item(a, InventoryType::Use, &Item::bundle(2_001_000 + i, 1), 1).unwrap();
        }
        store.add_item(b, InventoryType::Use, &Item::bundle(2_000_000, 5), 100).unwrap();
        let slot = store.bag_items(b, InventoryType::Use).unwrap()[0].slot;
        store.escrow_trade_item(b, InventoryType::Use, slot, None, 1).unwrap();
        store.escrow_trade_mesos(a, 1000).unwrap();
        assert!(store.complete_trade(a, b, &|_| 1).is_err(), "no room on one side");
        assert_eq!(store.trade_escrow(b).unwrap().0.len(), 1, "nothing moved");
        assert_eq!(store.trade_escrow(a).unwrap().1, 1000);
        assert_eq!(store.mesos(b).unwrap(), 2850, "b's wallet untouched");
    }

    fn character(store: &Store) -> u32 {
        let account = store.create_account("trader", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Wisp".into(), ..Default::default() };
        store.create_character(account, 0, &chr).unwrap().id
    }

    /// Part of a stack goes in and the rest stays; a cancel puts it back on the stack and the
    /// mesos back in the wallet, and nothing is left held.
    #[test]
    fn a_put_leaves_the_bag_and_a_return_restores_it() {
        let store = Store::open_in_memory().unwrap();
        let id = character(&store);
        store.set_mesos(id, 5000).unwrap();
        store.add_item(id, InventoryType::Use, &Item::bundle(2_060_000, 1000), 1000).unwrap();
        let slot = store.bag_items(id, InventoryType::Use).unwrap()[0].slot;

        let (taken, left) = store.escrow_trade_item(id, InventoryType::Use, slot, Some(400), 1).unwrap();
        assert_eq!((taken, left), (Item::bundle(2_060_000, 400), 600));
        assert_eq!(store.escrow_trade_mesos(id, 3000).unwrap(), 2000, "3000 out of the wallet");
        assert_eq!(store.escrow_trade_mesos(id, 1000).unwrap(), 4000, "a lower total gives the difference back");
        let (lines, mesos) = store.trade_escrow(id).unwrap();
        assert_eq!((lines.len(), mesos), (1, 1000));
        assert!(store.escrow_trade_item(id, InventoryType::Use, slot, Some(1), 1).is_err(), "trade slot 1 is taken");
        assert!(store.escrow_trade_mesos(id, 6000).is_err(), "more than the wallet and the table together");

        let back = store.return_trade_escrow(id, &|_| 1000).unwrap();
        assert_eq!((back.mesos, back.kept), (1000, 0));
        assert_eq!(store.mesos(id).unwrap(), 5000);
        assert_eq!(store.bag_items(id, InventoryType::Use).unwrap()[0].item, Item::bundle(2_060_000, 1000), "back on its stack");
        assert_eq!(store.trade_escrow(id).unwrap(), (Vec::new(), 0), "nothing held");
    }

    /// A whole stack, an equip with its own stats, and a star stack named with a smaller
    /// count: all three leave the bag whole and come back whole.
    #[test]
    fn whole_slots_and_stars_go_in_whole_and_come_back_whole() {
        let store = Store::open_in_memory().unwrap();
        let id = character(&store);
        let mut sword = Item::equip(1_302_000);
        sword.failed_slots = 2;
        store.add_item(id, InventoryType::Equip, &sword, 1).unwrap();
        store.add_item(id, InventoryType::Use, &Item::bundle(2_070_000, 480), 500).unwrap();
        let eq = store.bag_items(id, InventoryType::Equip).unwrap()[0].slot;
        let st = store.bag_items(id, InventoryType::Use).unwrap()[0].slot;

        assert_eq!(store.escrow_trade_item(id, InventoryType::Equip, eq, None, 2).unwrap(), (sword, 0));
        assert_eq!(store.escrow_trade_item(id, InventoryType::Use, st, Some(1), 3).unwrap(), (Item::bundle(2_070_000, 480), 0), "stars whole");
        assert!(store.bag_items(id, InventoryType::Equip).unwrap().is_empty());
        assert!(store.bag_items(id, InventoryType::Use).unwrap().is_empty());

        let back = store.return_trade_escrow(id, &|_| 500).unwrap();
        assert_eq!(back.placed.len(), 2);
        assert_eq!(store.bag_items(id, InventoryType::Equip).unwrap()[0].item, sword, "its failed slots travelled");
        assert_eq!(store.bag_items(id, InventoryType::Use).unwrap()[0].item.kind.quantity(), 480);
    }
}
