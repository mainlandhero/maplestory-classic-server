//! The NPC shop counter: opening it, buying, selling, and the meso count.
//!
//! The rule that shapes all of it: **every transaction ends in a `0x055F`**. The client
//! latches after sending one and every further Buy or Sell click is a no-op until a
//! result or a fresh list clears it.

use super::*;

// `BUY_BACK_DEPTH` was 15, the client's own list length. Removed with the Buy Back feature:
// this client's shop window has no `repurchaseInfo` node and exactly two tabs.

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
    /// Open the classic counter for a shopkeeper, or `None` if this NPC does not keep one.
    ///
    /// # This used to be off by default, and the reason it was is now fixed
    ///
    /// It sent `0x0560`, the **Shop2** window. That window's art -
    /// `UI/UIWindow2.img/Shop2/backgrnd` - **is not in this client's WZ**: the ResMan COM call
    /// fails, `_com_issue_errorex` throws and the unwinder faults, *before a single row byte
    /// is read*. Two manual launches died on it, twelve rows and one correctly-formed row
    /// alike, which is why `--shop-rows 1` changed nothing. So the packet was disabled and a
    /// shopkeeper fell through to ordinary dialogue.
    ///
    /// **This client has two shop windows.** The classic one, `UI/UIShop.img/Shop`, is present
    /// in `UI_000.wz`, and it opens on `0x055D`. `research/classic-shop-opcode.md` found that
    /// and `research/classic-shop-rows.md` decoded the body down to the price, the gates and
    /// the result table. `net::classicshop` builds it against that file's own golden vector.
    ///
    /// So the gate is gone: shops are on. What killed the client was never our bytes.
    ///
    /// # The dialogue must be REPLACED, not followed
    ///
    /// `0x055D`'s handler has a modal guard - if the shop singleton is non-null the packet is
    /// **discarded silently**. A `0x055B` script box already on screen is exactly that case,
    /// which is why `on_npc_click` tries this branch *before* it builds a conversation.
    pub(super) fn open_shop_for(&mut self, template: u32, character_id: u32) -> Option<Vec<Reply>> {
        let index = *self.config.shop_by_template.get(&template)?;
        let shop = self.config.shops.shops.get(index)?;

        let mut rows = Vec::new();
        let mut skipped_free = 0usize;
        for item in &shop.items {
            // **A zero buy price is not a free item, it is a SELL row.** The client files a
            // row by the sign of its price, so a 0 here would silently appear in the Sell tab
            // offering to buy something at nothing. `data/shops.txt` has no such row today;
            // this makes a future transcription typo show up as a missing line rather than as
            // junk in the wrong tab.
            if item.buy_price == 0 {
                skipped_free += 1;
                continue;
            }
            rows.push(net::classicshop::ClassicShopRow::buy(
                item.item_id,
                u64::from(item.buy_price),
                // **Never zero.** `ItemData::slot_max` is zero for 2495 of the 2785 rows in
                // `gm-handbook/itemdata.txt`, and a zero cap makes every purchase of that row
                // fail with no message at all - to the player or to us.
                i16::try_from(self.config.shops.max_per_purchase(item.item_id)).unwrap_or(100),
            ));
        }
        for item in &shop.items {
            let Some(data) = self.config.shops.item_data.get(&item.item_id) else { continue };
            if !data.may_be_sold() {
                continue; // the owner: "Please do not allow quest items to be sold."
            }
            rows.push(net::classicshop::ClassicShopRow::sell(
                item.item_id,
                u64::from(data.price),
                i16::try_from(self.config.shops.max_per_purchase(item.item_id)).unwrap_or(100),
            ));
        }
        // **No Buy Back rows.** This client's `UI/UIShop.img/Shop` has no `repurchaseInfo`
        // node and exactly two tabs, `TabBuy` and `TabSell`. See `classic_sell`.

        if rows.is_empty() {
            return None; // a zero-row shop is a different client arm, not an empty counter
        }

        // **The blast-radius control survives the opcode change**, and is worth more here than
        // it was: `--shop-rows 1` tells a bad row apart from too many rows in one launch,
        // which is the scarcest thing on this project. A zero-row shop is a different client
        // arm entirely, so the cap can never take the list below one.
        let capped = match self.config.shop_rows {
            Some(n) if n < rows.len() => n.max(1),
            _ => rows.len(),
        };
        let dropped_by_cap = rows.len() - capped;
        rows.truncate(capped);

        let body = net::classicshop::classic_open_shop(template, &rows);
        let what = format!(
            "ClassicOpenShop 0x055D: {} ({}) for character {character_id} - {} rows ({} buy, \
             {} sell, {} buy-back), {} bytes{}{}",
            shop.npc,
            shop.role,
            rows.len(),
            rows.iter().filter(|r| !r.sell && !r.buy_back).count(),
            rows.iter().filter(|r| r.sell).count(),
            rows.iter().filter(|r| r.buy_back).count(),
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
        Some(vec![Reply { opcode: net::classicshop::CLASSIC_OPEN_SHOP, body, what }])
    }

    /// `0x00F5` - everything the classic counter sends.
    ///
    /// **Every arm but Close is answered with a `0x055E`.** `shopUI+0x4b0` latches when the
    /// window sends one of these and only a result clears it: an unanswered buy leaves the
    /// window alive but every further click a silent no-op, which reads on screen as the shop
    /// half-working. Closing and re-clicking the NPC recovers it - a fresh `0x055D` alone does
    /// not, because of the modal guard.
    pub(super) fn on_classic_shop_request(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(request) = net::classicshop::parse_classic_shop_request(body) else {
            // A body we cannot read is our problem, not a reason to wedge the counter.
            return self.classic_refused(
                net::classicshop::RESULT_NOT_ENOUGH_MESOS,
                &format!("unreadable 0x00F5 body {body:02x?} - clearing the latch anyway"),
            );
        };
        match request {
            net::classicshop::ClassicShopRequest::Close => {
                self.open_shop = None;
                Vec::new() // nothing is latched on close
            }
            net::classicshop::ClassicShopRequest::Recharge { inventory_slot } => self
                .classic_refused(
                    net::classicshop::RESULT_NOT_ENOUGH_MESOS,
                    &format!(
                        "recharge of slot {inventory_slot} - no row this server sends is \
                         rechargeable, so this should be unreachable"
                    ),
                ),
            net::classicshop::ClassicShopRequest::Buy { row_index, item_id, quantity } => {
                self.classic_buy(row_index, item_id, quantity)
            }
            net::classicshop::ClassicShopRequest::Sell { inventory_slot, item_id, quantity } => {
                self.classic_sell(inventory_slot, item_id, quantity)
            }
        }
    }

    /// Buy, or buy back when the row index names a flagged row.
    ///
    /// **The client's numbers are claims.** It checked `price * quantity` against its own meso
    /// count before sending, but that is its arithmetic on its balance, and nothing on this
    /// socket authenticates anybody. The quantity is clamped to the row's own cap and the
    /// price is the server's.
    fn classic_buy(&mut self, row_index: u16, item_id: u32, quantity: u16) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else {
            return self.classic_refused(
                net::classicshop::RESULT_NOT_ENOUGH_MESOS,
                "no character is claimed",
            );
        };
        let Some((_, rows)) = self.open_shop.clone() else {
            return self
                .classic_refused(net::classicshop::RESULT_NOT_ENOUGH_MESOS, "no shop is open");
        };
        // **The index is into the list we sent**, which is the only defensible reading of it.
        let Some(row) = rows.get(usize::from(row_index)).copied() else {
            return self.classic_refused(
                net::classicshop::RESULT_NOT_ENOUGH_MESOS,
                &format!("row {row_index} is not among the {} rows we sent", rows.len()),
            );
        };
        // The client also names the item. Disagreement means our list and its list have
        // drifted, and buying the wrong thing is worse than refusing.
        if row.item_id != item_id {
            return self.classic_refused(
                net::classicshop::RESULT_NOT_ENOUGH_MESOS,
                &format!("row {row_index} is item {} and the client asked for {item_id}", row.item_id),
            );
        }
        let Some(inv) = store::InventoryType::for_item(row.item_id) else {
            return self.classic_refused(
                net::classicshop::RESULT_NOT_ENOUGH_MESOS,
                &format!("item {} belongs to no inventory tab", row.item_id),
            );
        };

        let cap = u16::try_from(row.max_per_purchase.max(1)).unwrap_or(1);
        let qty = quantity.clamp(1, cap);
        let cost = u64::from(qty).saturating_mul(row.price);
        let cost = u32::try_from(cost).unwrap_or(u32::MAX);
        let max_stack = self.config.shops.max_per_purchase(row.item_id);
        let item = if inv == store::InventoryType::Equip {
            store::Item::equip(row.item_id)
        } else {
            store::Item::bundle(row.item_id, qty)
        };
        match self.store.buy_item(chr.id, inv, &item, max_stack, cost) {
            Ok(changed) => {
                let what = if row.buy_back {
                    format!("bought BACK {qty}x {} for {cost} mesos", row.item_id)
                } else {
                    format!("bought {qty}x {} for {cost} mesos", row.item_id)
                };
                let mut out = vec![Reply {
                    opcode: net::classicshop::CLASSIC_SHOP_RESULT,
                    body: net::classicshop::classic_shop_success(row.item_id, 0),
                    what: format!("ClassicShopResult success: {what}"),
                }];
                // **The result moves nothing by itself.** Without these two the window says
                // the purchase worked while the item and the mesos stay where they were.
                out.extend(self.inventory_added_replies(inv, &changed, "bought"));
                out.extend(self.meso_reply(chr.id));
                // No buy-back rows are ever sent, so `row.buy_back` cannot be set here.
                // Kept as a debug assertion rather than a branch, because the reason is a
                // property of this client's WZ and not of the protocol.
                debug_assert!(!row.buy_back, "this client cannot draw a Buy Back tab");
                out
            }
            Err(store::StoreError::BagFull { .. }) => {
                self.classic_refused(net::classicshop::RESULT_INVENTORY_FULL, "the bag is full")
            }
            Err(store::StoreError::NotEnoughMesos { .. }) => self
                .classic_refused(net::classicshop::RESULT_NOT_ENOUGH_MESOS, "not enough mesos"),
            Err(e) => self
                .classic_refused(net::classicshop::RESULT_NOT_ENOUGH_MESOS, &format!("buy failed: {e}")),
        }
    }

    /// Sell one inventory slot, and push what was sold onto the Buy Back ring.
    fn classic_sell(&mut self, slot: u16, item_id: u32, quantity: u16) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else {
            return self.classic_refused(
                net::classicshop::RESULT_NOT_ENOUGH_MESOS,
                "no character is claimed",
            );
        };
        let Some(inv) = store::InventoryType::for_item(item_id) else {
            return self.classic_refused(
                net::classicshop::RESULT_NOT_ENOUGH_MESOS,
                &format!("item {item_id} belongs to no inventory tab"),
            );
        };
        // The sell price is the client's own `Item.wz` price - the owner, 2026-08-19: *"The prices
        // client side most likely represents sell prices."* `data/shops.txt` holds only what
        // the NPC charges.
        let unit = self.config.shops.item_data.get(&item_id).map(|d| d.price).unwrap_or(0);
        match self.store.sell_item(chr.id, inv, slot, Some(quantity), unit) {
            Ok(_) => {
                let mut out = vec![Reply {
                    opcode: net::classicshop::CLASSIC_SHOP_RESULT,
                    body: net::classicshop::classic_shop_success(item_id, 0),
                    what: format!("ClassicShopResult success: sold {quantity}x {item_id} from slot {slot}"),
                }];
                out.push(Reply {
                    opcode: net::inventory::INVENTORY_OPERATION,
                    body: net::inventory::inventory_removed(inv.as_u8() as i8, slot as i16),
                    what: format!("InventoryOperation REMOVE: {inv:?} slot {slot}"),
                });
                out.extend(self.meso_reply(chr.id));
                // **NO Buy Back, and NO type-10 refresh. Both kill this client.**
                //
                // The owner, 2026-08-28: *"Selling an item to Lucy crashed the client."* The sale
                // itself worked - the success, the inventory remove and the meso change all
                // went out and were accepted. What killed it was the `0x055E` type 10 that
                // followed, and the client handed the packet straight back to us in a
                // 2075-byte `0x009E` before dying.
                //
                // **`UI/UIShop.img/Shop` has 16 nodes and `repurchaseInfo` is not one of
                // them.** Read with `wz-dump`, with `BtBuy` as the positive control. The
                // node list also carries exactly `TabBuy` and `TabSell` - **two** tabs. Type
                // 10's documented effect is *refill, then select the Buy Back tab*
                // (`research/classic-shop-rows.md` §8), so it reaches for art this client
                // does not ship, `_com_issue_errorex` throws, and the unwinder faults at
                // `0x140ce89d6`.
                //
                // That is the **same failure as Shop2**, one level down and for the same
                // reason - `research/npc-shop-crash2.md` records `0x140ce89d6` for that crash
                // too. The client's own art is the authority on what its windows have.
                //
                // If a refresh is ever needed, **type 35 selects tab 0**, which exists.
                out
            }
            Err(store::StoreError::ItemMayNotBeSold { .. }) => self.classic_refused(
                net::classicshop::RESULT_NOT_ENOUGH_MESOS,
                "that is a quest item and may not be sold - the owner's rule, enforced in the store",
            ),
            Err(e) => self
                .classic_refused(net::classicshop::RESULT_NOT_ENOUGH_MESOS, &format!("sell failed: {e}")),
        }
    }

    // **`classic_refresh` was removed on 2026-08-28, the day it first went out.**
    //
    // It sent `0x055E` type 10, whose documented effect is *refill, then select the Buy Back
    // tab*. `UI/UIShop.img/Shop` has no `repurchaseInfo` node - 16 nodes, `BtBuy` as the
    // positive control - and carries exactly `TabBuy` and `TabSell`. Selecting a tab whose
    // art is missing throws, and the unwinder faults at `0x140ce89d6`, the same address the
    // Shop2 crash produced for the same reason.
    //
    // **Type 35 is the survivor** if a refill is ever wanted: it selects tab 0.

    /// A `0x055E` refusal. **Never skip one**: the window latches on send.
    fn classic_refused(&self, result_type: u8, why: &str) -> Vec<Reply> {
        vec![Reply {
            opcode: net::classicshop::CLASSIC_SHOP_RESULT,
            body: net::classicshop::classic_shop_refused(result_type),
            what: format!("ClassicShopResult refusal (type {result_type}): {why}"),
        }]
    }

    /// `0x0104` - the **Shop2** window's request opcode.
    ///
    /// Nothing can open that window any more: its art is missing from this client, and
    /// [`Session::open_shop_for`] sends `0x055D` instead. This arm exists so that if the
    /// assumption is ever wrong the request is still answered rather than left hanging, which
    /// is the failure mode this project has paid for most often.
    pub(super) fn on_shop_request(&mut self, body: &[u8]) -> Vec<Reply> {
        if net::shop::parse_shop_request(body).is_none() {
            return Vec::new(); // no sub-op byte; nothing was latched by a body this short
        }
        vec![Reply {
            opcode: net::shop::SHOP_TRANSACTION_RESULT,
            body: net::shop::shop_result(net::shop::ShopResult::Busy),
            what: "ShopResult Busy: a 0x0104 arrived, but this server opens the CLASSIC \
                   counter (0x055D) and nothing should be able to open Shop2. Answered so the \
                   window is not left latched"
                .to_string(),
        }]
    }

    // **`shop_answer` was deleted with the Shop2 transaction path.** It built a `0x055F`
    // and, on a re-requesting result, a fresh `0x0560` - the packet whose art this client does
    // not have. Its replacement is `Session::classic_refused` plus `Session::classic_refresh`,
    // which are driven by what actually changed rather than by the result code.

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
