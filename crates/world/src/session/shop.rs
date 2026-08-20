//! The NPC shop counter: opening it, buying, selling, and the meso count.
//!
//! The rule that shapes all of it: **every transaction ends in a `0x055F`**. The client
//! latches after sending one and every further Buy or Sell click is a no-op until a
//! result or a fresh list clears it.

use super::*;

impl Session {

    /// Build and send this NPC's shop, or `None` if it has no shop.
    ///
    /// # The Sell tab is made of negative prices
    ///
    /// `140d23ac5 cmp dword [rbx+0x58],0 / jg` files each row into the Buy tab
    /// (`shopUI+0x338`) or the Sell tab (`+0x340`), and the Sell tab is then intersected
    /// with what the player is actually carrying. **A shop that sends no negative-price rows
    /// has an empty Sell tab** - there is no separate "here is what I buy" packet.
    /// `net::shop::ShopRow::sell` does the negation; do not negate twice.
    ///
    /// The two prices come from different places and swapping them is the easy mistake:
    /// a **buy** price is authored per row in `data/shops.txt`, a **sell** price is
    /// `ItemData::price`, which is the client's own `info/price` and is what the NPC *pays*.
    /// The owner established that direction: *"The prices client side most likely represents sell
    /// prices."*
    ///
    /// The owner's rule is enforced here as well as in the store: a quest item gets no sell row,
    /// so it cannot be offered in the first place.
    ///
    /// **An empty row list is not an empty shop.** `140d22656` takes a different arm
    /// entirely for `rowCount == 0` - a dialog box, and a `0x0104` back - so a shop that
    /// resolves to nothing falls through to the dialogue path instead.
    pub(super) fn open_shop_for(&mut self, template: u32, character_id: u32) -> Option<Vec<Reply>> {
        let index = *self.config.shop_by_template.get(&template)?;
        let shop = self.config.shops.shops.get(index)?;

        let mut rows = Vec::new();
        let mut skipped_free = 0usize;
        for item in &shop.items {
            // **A zero buy price is not a free item, it is a SELL row.** The client files a
            // row by the sign of its price, and `> 0` is the buy test - so a 0 here would
            // silently appear in the Sell tab offering to buy something the player has, at
            // nothing. `data/shops.txt` currently has no such row; this is here so that a
            // future transcription typo shows up as a missing line rather than as junk in
            // the wrong tab.
            if item.buy_price == 0 {
                skipped_free += 1;
                continue;
            }
            rows.push(net::shop::ShopRow::buy(
                rows.len() as u32,
                item.item_id,
                item.buy_price,
                self.config.shops.max_per_purchase(item.item_id),
            ));
        }
        for item in &shop.items {
            let Some(data) = self.config.shops.item_data.get(&item.item_id) else { continue };
            if !data.may_be_sold() {
                continue; // the owner: "Please do not allow quest items to be sold."
            }
            rows.push(net::shop::ShopRow::sell(rows.len() as u32, item.item_id, data.price));
        }
        if rows.is_empty() {
            return None; // a zero-row shop is a different client arm, not an empty counter
        }

        // **A blast-radius control, exactly like `--mob-limit`.** On 2026-08-20 Lucy's
        // counter went out with twelve rows and the client threw a C++ exception ten
        // milliseconds later, then faulted. A fault can come from a row being wrong or from
        // twelve rows at once, and those look identical on screen. `--shop-rows 1` makes
        // them distinguishable in one launch, which is the scarcest thing on this project.
        //
        // Applied AFTER both loops rather than inside them, so `--shop-rows 1` leaves one
        // *buy* row - the buy direction is the one with a straight-line trace behind it,
        // and the sell direction has never been on a wire in either direction.
        //
        // A zero-row shop is a different client arm entirely (`140d22656 test edi,edi`), so
        // the cap can never take the list below one.
        let capped = match self.config.shop_rows {
            Some(n) if n < rows.len() => n.max(1),
            _ => rows.len(),
        };
        let dropped_by_cap = rows.len() - capped;
        rows.truncate(capped);

        let body = net::shop::open_shop(template, &rows);
        let what = format!(
            "OpenShop: {} ({}) for character {character_id} - {} rows ({} buy, {} sell), {} bytes{}{}",
            shop.npc,
            shop.role,
            rows.len(),
            rows.iter().filter(|r| r.is_buy_row()).count(),
            rows.iter().filter(|r| !r.is_buy_row()).count(),
            body.len(),
            if skipped_free > 0 {
                format!(" - {skipped_free} row(s) DROPPED for a zero buy price")
            } else {
                String::new()
            },
            if dropped_by_cap > 0 {
                format!(" - {dropped_by_cap} row(s) HELD BACK by --shop-rows")
            } else {
                String::new()
            }
        );
        self.open_shop = Some((template, rows));
        Some(vec![Reply { opcode: net::shop::OPEN_SHOP, body, what }])
    }


    /// The client's shop request, `0x0104`.
    ///
    /// **A transaction must be answered.** `FUN_140d2c060` sets `shopUI+0x14d8 = 1` after
    /// sending sub-op 1 and returns immediately at `140d2c0a6` while it is set, so every
    /// further Buy or Sell click is a silent no-op until it clears. It is **not** the
    /// session-long latch `0x0107` has: `tools/fieldrefs.py 0x14d8` enumerates four writers,
    /// and a fresh `OPEN_SHOP` clears it too. Dead until the next result or list, not
    /// forever - a real difference, and worth not overstating.
    pub(super) fn on_shop_request(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(request) = net::shop::parse_shop_request(body) else {
            // No sub-op byte at all. Nothing was latched by a body this short.
            return Vec::new();
        };
        match request {
            net::shop::ShopRequest::Reopen { npc_template_id } => {
                // Re-send the SAME rows, so the row keys the client still holds stay valid.
                let Some((template, rows)) = self.open_shop.clone() else { return Vec::new() };
                if template != npc_template_id {
                    return Vec::new();
                }
                vec![Reply {
                    opcode: net::shop::OPEN_SHOP,
                    body: net::shop::open_shop(template, &rows),
                    what: format!("OpenShop: re-sent for {template}, same {} rows", rows.len()),
                }]
            }
            net::shop::ShopRequest::Close => {
                self.open_shop = None;
                Vec::new() // nothing is owed; the client closed its own UI
            }
            net::shop::ShopRequest::Other { .. } => Vec::new(),
            net::shop::ShopRequest::Transaction(t) => self.on_shop_transaction(t),
        }
    }


    /// Buy or sell one row. **Every path here ends in a `0x055F`.**
    ///
    /// Three rules this encodes, all of them easy to lose:
    ///
    /// 1. **The client's numbers are claims.** It computed `|price| * quantity` against its
    ///    own meso count before sending, but that is its arithmetic and its balance. The
    ///    quantity is clamped to the row's own maximum and the price is the server's.
    /// 2. **A result that re-requests owes a fresh list.** `ShopResult::rerequests` is the
    ///    client's own tail test at `140d22f51`; five of the fourteen codes set it.
    /// 3. **Success still owes the bag.** The client does not move an item on a bare
    ///    success - the `0x0070` does that, and the meso count needs `0x007C`.
    pub(super) fn on_shop_transaction(&mut self, t: net::shop::ShopTransaction) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else {
            return self.shop_answer(net::shop::ShopResult::Busy, "no character is claimed");
        };
        let Some((template, rows)) = self.open_shop.clone() else {
            return self.shop_answer(net::shop::ShopResult::Busy, "no shop is open");
        };
        let Some(row) = rows.iter().find(|r| r.row_key == t.row_key).copied() else {
            return self.shop_answer(
                net::shop::ShopResult::UnknownItem,
                &format!("row key {} is not in the {} rows we sent", t.row_key, rows.len()),
            );
        };
        let Some(inv) = store::InventoryType::for_item(row.item_id) else {
            return self.shop_answer(
                net::shop::ShopResult::UnknownItem,
                &format!("item {} belongs to no inventory tab", row.item_id),
            );
        };
        let _ = template;

        if row.is_buy_row() {
            let qty = t.quantity.clamp(1, row.max_per_purchase.max(1));
            let unit = row.price.max(0) as u32;
            let cost = u32::from(qty).saturating_mul(unit);
            let max_stack = self.config.shops.max_per_purchase(row.item_id);
            let item = if inv == store::InventoryType::Equip {
                store::Item::equip(row.item_id)
            } else {
                store::Item::bundle(row.item_id, qty)
            };
            match self.store.buy_item(chr.id, inv, &item, max_stack, cost) {
                Ok(changed) => {
                    let mut out = self.shop_answer(
                        net::shop::ShopResult::Success,
                        &format!("bought {qty}x {} for {cost} mesos", row.item_id),
                    );
                    out.extend(self.inventory_added_replies(inv, &changed, "bought"));
                    out.extend(self.meso_reply(chr.id));
                    out
                }
                Err(store::StoreError::BagFull { .. }) => {
                    self.shop_answer(net::shop::ShopResult::InventoryFull, "the bag is full")
                }
                Err(store::StoreError::NotEnoughMesos { .. }) => self
                    .shop_answer(net::shop::ShopResult::NotEnoughMesos, "not enough mesos"),
                Err(e) => self.shop_answer(net::shop::ShopResult::Busy, &format!("buy failed: {e}")),
            }
        } else {
            let unit = row.price.unsigned_abs();
            match self.store.sell_item(chr.id, inv, t.slot, Some(t.quantity), unit) {
                Ok(_) => {
                    let mut out = self.shop_answer(
                        net::shop::ShopResult::Success,
                        &format!("sold {}x {} from slot {}", t.quantity, row.item_id, t.slot),
                    );
                    out.push(Reply {
                        opcode: net::inventory::INVENTORY_OPERATION,
                        body: net::inventory::inventory_removed(inv.as_u8() as i8, t.slot as i16),
                        what: format!("InventoryOperation REMOVE: {inv:?} slot {}", t.slot),
                    });
                    out.extend(self.meso_reply(chr.id));
                    out
                }
                Err(store::StoreError::ItemMayNotBeSold { .. }) => self.shop_answer(
                    net::shop::ShopResult::UnknownItem,
                    "that is a quest item and may not be sold - the owner's rule, enforced in the store",
                ),
                Err(e) => {
                    self.shop_answer(net::shop::ShopResult::UnknownItem, &format!("sell failed: {e}"))
                }
            }
        }
    }


    /// A `0x055F`, plus the fresh list the result may owe.
    pub(super) fn shop_answer(&self, result: net::shop::ShopResult, why: &str) -> Vec<Reply> {
        let mut out = vec![Reply {
            opcode: net::shop::SHOP_TRANSACTION_RESULT,
            body: net::shop::shop_result(result),
            what: format!("ShopResult {result:?} (code {}): {why}", result.code()),
        }];
        if result.rerequests() {
            if let Some((template, rows)) = self.open_shop.as_ref() {
                out.push(Reply {
                    opcode: net::shop::OPEN_SHOP,
                    body: net::shop::open_shop(*template, rows),
                    what: format!(
                        "OpenShop: re-sent because ShopResult {result:?} makes the client re-request"
                    ),
                });
            }
        }
        out
    }


    /// Tell the client its new meso count.
    ///
    /// `quiet` is `false` on purpose: `142d56242` only fires the meso-gain effect when that
    /// byte is zero, and a purchase is exactly when a player expects to see the number move.
    pub(super) fn meso_reply(&self, character_id: u32) -> Vec<Reply> {
        let Ok(mesos) = self.store.mesos(character_id) else { return Vec::new() };
        let change = net::combat::StatChange {
            excl_request: true,
            meso: Some(u64::from(mesos)),
            ..Default::default()
        };
        vec![Reply {
            opcode: net::combat::STAT_CHANGED,
            body: net::combat::stat_changed(&change),
            what: format!("StatChanged: mesos now {mesos}"),
        }]
    }
}
