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
    /// The two prices come from different places and swapping them is the easy mistake:
    /// a **buy** price is authored per row in `data/shops.txt`, a **sell** price is
    /// `ItemData::price`, which is the client's own `info/price` and is what the NPC *pays*.
    /// The owner established that direction: *"The prices client side most likely represents sell
    /// prices."* Only the buy price goes out on a row; the sell price is what `classic_sell`
    /// pays, and the client draws its own from the WZ.
    ///
    /// The owner's rule - no selling quest items - is enforced in the store (`sell_item`).
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
            rows.push(
                net::classicshop::ClassicShopRow::buy(
                    item.item_id,
                    u64::from(item.buy_price),
                    // **Never zero.** `ItemData::slot_max` is zero for 2495 of the 2785 rows in
                    // `gm-handbook/itemdata.txt`, and a zero cap makes every purchase of that row
                    // fail with no message at all - to the player or to us.
                    //
                    // **A star or bullet row sells ONE SET per purchase** (the owner, 2026-10-02:
                    // *"on purchase, the player should receive a full stack of that star consumable
                    // instead of just a singular 1"*), so its cap is 1, which the client draws as a
                    // plain yes/no instead of a quantity box (`research/classic-shop-rows.md` §3
                    // row 42, `== 1`). `classic_buy` hands over the full `slotMax`.
                    if net::bag::bundle_has_serial(item.item_id) {
                        1
                    } else {
                        i16::try_from(self.config.shops.max_per_purchase(item.item_id)).unwrap_or(100)
                    },
                )
                // **The recharge price rides on the same row.** The owner, 2026-09-06: stars
                // "should be able to recharge ... at general merchants". The client offers
                // Recharge for an id this counter lists with a non-zero unit price
                // (`research/classic-shop-rows.md` §3 row 41a), and every Grocer already lists
                // Subi, so this is one field on a row that already went out - not a new row.
                // On a non-rechargeable id the field is not on the wire and the value drops.
                .with_unit_price(self.unit_price_milli(item.item_id))
                // **The town-hall shops' grade locks.** `data/shops.txt` tags a row with the
                // grade it needs; the town is the shop's own. The client draws the row locked
                // and refuses it itself, and `classic_buy` refuses it too.
                // session/citizenship.rs.
                .with_citizenship(
                    Self::shop_row_citizenship(template, item.min_grade).map_or(0, |(t, _)| u32::from(t)),
                    Self::shop_row_citizenship(template, item.min_grade).map_or(0, |(_, g)| u32::from(g)),
                ),
            );
        }
        // **No Sell twins.** Until 2026-09-16 every stocked item went out a second time with
        // the classic row's "sell" byte set and the WZ `info/price` - a Shop2 habit, where the
        // price's sign chose the tab. The classic window files EVERY surviving row into the
        // Buy list (`research/classic-shop-rows.md` §5: only a Buy Back row skips the Buy-tab
        // classification), so on screen each item appeared twice, the twin at a tenth of the
        // price. The owner: *"there are duplicate items in the NPC shop, one being regular price,
        // another being 10 times cheaper. This is happening across multiple if not all NPC
        // shops."* The Sell panel is built by the client from the player's own inventory at
        // the client's own `info/price` (the screenshot prices a Green Skullcap no shop
        // stocks), and `classic_sell` answers the sell request from `ItemData::price` - so
        // the twins carried nothing the client used. Quest items still cannot be sold:
        // `Store::sell_item` refuses them.
        //
        // **No Buy Back rows.** This client's `UI/UIShop.img/Shop` has no `repurchaseInfo`
        // node and exactly two tabs, `TabBuy` and `TabSell`. See `classic_sell`.

        if rows.is_empty() {
            return None; // a zero-row shop is a different client arm, not an empty counter
        }

        // **Every star recharges at a general store**, stocked or not. The owner, 2026-10-02,
        // after a dropped Wolbi could not be recharged at a Grocer that sells only Subi: *"All
        // stars should be rechargeable at any general store."* The client offers Recharge only
        // for an id in its Recharge list, and that list is built from the rows we send - so
        // each star the counter does not stock goes out as a price-0 recharge-only row
        // (`ClassicShopRow::recharge_only`: in the Recharge list, in no Buy tab). Appended
        // AFTER the stocked rows so no Buy row's index moves. Stars only - `207xxxx` with a
        // unit price and a stack size; this client has no bullets in its item table.
        if shop.is_general_store() {
            let mut stars: Vec<(u32, u32)> = self
                .config
                .shops
                .item_data
                .iter()
                .filter(|(id, d)| **id / 10_000 == 207 && d.unit_price_milli > 0 && d.slot_max > 0)
                .filter(|(id, _)| !rows.iter().any(|r| r.item_id == **id))
                .map(|(id, d)| (*id, d.unit_price_milli))
                .collect();
            stars.sort_unstable();
            rows.extend(stars.into_iter().map(|(id, unit)| net::classicshop::ClassicShopRow::recharge_only(id, unit)));
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
             {} sell, {} buy-back, {} rechargeable with a unit price, {} of them recharge-only), \
             {} bytes{}{}",
            shop.npc,
            shop.role,
            rows.len(),
            rows.iter().filter(|r| !r.sell && !r.buy_back && r.price > 0).count(),
            rows.iter().filter(|r| r.sell).count(),
            rows.iter().filter(|r| r.buy_back).count(),
            rows.iter().filter(|r| r.unit_price().is_some_and(|p| p > 0.0)).count(),
            rows.iter().filter(|r| r.price == 0 && r.unit_price().is_some_and(|p| p > 0.0)).count(),
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
            net::classicshop::ClassicShopRequest::Recharge { inventory_slot } => {
                self.classic_recharge(inventory_slot)
            }
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
        // **A recharge-only row is not for sale.** The client never offers it (price 0 skips
        // every Buy tab), so a buy naming one is a forged body - and would be free stars.
        if row.price == 0 && !row.buy_back {
            return self.classic_refused(
                net::classicshop::RESULT_NOT_ENOUGH_MESOS,
                &format!("row {row_index} (item {}) is recharge-only, not for sale", row.item_id),
            );
        }
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
        // **A grade-locked row is sold to that town's citizens only.** The client refuses it
        // first; this is the refusal on the side of the socket that has to be believed.
        if row.citizenship_grade > 0 {
            let (town, grade) = (row.citizenship_town as u8, row.citizenship_grade as u8);
            if !self.may_buy_gated(chr.id, town, grade) {
                let mut out = self.classic_refused(
                    net::classicshop::RESULT_NOT_ENOUGH_MESOS,
                    &format!("item {} needs citizenship town {town} grade {grade}", row.item_id),
                );
                out.extend(self.notice(format!(
                    "Only citizens of {} of grade {grade} ({}) or higher may buy that.",
                    crate::citizenship::town(town).map_or("that town", |t| t.name),
                    crate::citizenship::grade_name(grade)
                )));
                return out;
            }
        }

        let cap = u16::try_from(row.max_per_purchase.max(1)).unwrap_or(1);
        // **A star or bullet is sold by the SET**: one purchase, the row's price, a full
        // `slotMax` stack (Subi 500, Ilbi 800) - whatever count the client names, because the
        // row went out with a cap of 1 (see `open_shop_for`). Everything else by the unit.
        let set = net::bag::bundle_has_serial(row.item_id).then(|| self.config.shops.max_stack(row.item_id));
        let qty = if set.is_some() { 1 } else { quantity.clamp(1, cap) };
        let cost = u64::from(qty).saturating_mul(row.price);
        let cost = u32::try_from(cost).unwrap_or(u32::MAX);
        let max_stack = self.config.shops.max_per_purchase(row.item_id);
        let item = if inv == store::InventoryType::Equip {
            store::Item::equip(row.item_id)
        } else {
            store::Item::bundle(row.item_id, set.unwrap_or(qty))
        };
        match self.store.buy_item(chr.id, inv, &item, max_stack, cost) {
            Ok(changed) => {
                let what = if row.buy_back {
                    format!("bought BACK {qty}x {} for {cost} mesos", row.item_id)
                } else {
                    match set {
                        Some(n) => format!("bought a SET of {n} x {} for {cost} mesos", row.item_id),
                        None => format!("bought {qty}x {} for {cost} mesos", row.item_id),
                    }
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
        // **The owner: "Please do not allow quest items to be sold."** Checked here against the
        // client's own `info/quest` flag, and again in the store against its generated list.
        // This was the Sell twin's job until 2026-09-16 - a quest item got no twin - but the
        // client's Sell panel never read the twins (it is the player's own bag at the WZ
        // price), so the twin gated nothing; this does. The refusal is answered, not dropped:
        // an unanswered shop request leaves the window waiting.
        if self.config.shops.item_data.get(&item_id).is_some_and(|d| !d.may_be_sold()) {
            return self.classic_refused(
                net::classicshop::RESULT_NOT_ENOUGH_MESOS,
                &format!("item {item_id} is a quest item and may not be sold"),
            );
        }
        // **A sale of something that is no longer there is a stale view, not a cheat.** The
        // owner, 2026-10-02: *"When users sell to shop too fast, sometimes their view does not
        // refresh fast enough and they try to sell the same thing again to which the server
        // refuses."* The Sell list is drawn from the bag and redraws on the `0x0070` that
        // follows a sale (plan step 29c, on screen) - but a second click can leave before that
        // lands. So when the slot is empty, holds a different item, or holds fewer than asked,
        // nothing is sold and the slot's REAL state goes back, which redraws the list, under
        // the silent type 16: no "not enough mesos" for a click that was merely early.
        let held = self.store.inventory_slot(chr.id, inv, slot).ok().flatten();
        let stale = match &held {
            None => Some("the slot is already empty".to_string()),
            Some(h) if h.item_id != item_id => Some(format!("the slot holds item {}, not {item_id}", h.item_id)),
            // An EMPTY star stack (`Store::spend_ammo`) is sold whole, whatever count is named.
            Some(h)
                if inv != store::InventoryType::Equip
                    && h.kind.quantity() < quantity
                    && !(h.kind.quantity() == 0 && net::bag::bundle_has_serial(h.item_id)) =>
            {
                Some(format!("the slot holds {}, not {quantity}", h.kind.quantity()))
            }
            Some(_) => None,
        };
        if let Some(why) = stale {
            let mut out = vec![Reply {
                opcode: net::classicshop::CLASSIC_SHOP_RESULT,
                body: net::classicshop::classic_shop_refused(net::classicshop::RESULT_ACKNOWLEDGED),
                what: format!(
                    "ClassicShopResult type 16 (acknowledged, nothing sold): a STALE sell of {quantity}x \
                     {item_id} from {inv:?} slot {slot} - {why}; the slot's real state follows so the \
                     Sell list redraws"
                ),
            }];
            out.extend(self.slot_resync(inv, slot, held.as_ref()));
            return out;
        }
        // The sell price is the client's own `Item.wz` price - the owner, 2026-08-19: *"The prices
        // client side most likely represents sell prices."* `data/shops.txt` holds only what
        // the NPC charges.
        let unit = self.config.shops.item_data.get(&item_id).map(|d| d.price).unwrap_or(0);
        match self.store.sell_item(chr.id, inv, slot, Some(quantity), unit) {
            Ok(_) => {
                // **Type 16, not type 0**: type 0 re-selects the tab of the last PURCHASE
                // (`net::classicshop::RESULT_ACKNOWLEDGED`), which threw the player back to it
                // after every sale. 16 only clears the window's latch.
                let mut out = vec![Reply {
                    opcode: net::classicshop::CLASSIC_SHOP_RESULT,
                    body: net::classicshop::classic_shop_refused(net::classicshop::RESULT_ACKNOWLEDGED),
                    what: format!("ClassicShopResult type 16 (acknowledged, no tab change): sold {quantity}x {item_id} from slot {slot}"),
                }];
                // **What is LEFT in the slot**, not "the slot is gone": until 2026-10-02 every
                // sale sent a REMOVE, so selling 5 of 100 potions emptied the slot on screen
                // while the server still held 95.
                let left = self.store.inventory_slot(chr.id, inv, slot).ok().flatten();
                out.extend(self.slot_resync(inv, slot, left.as_ref()));
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

    /// One bag slot as the server holds it, for the client to redraw: a REMOVE when it is
    /// empty, a quantity change for a stack, and for anything else a REMOVE and then the item
    /// itself, so a slot that changed under the client's feet is replaced rather than merged.
    fn slot_resync(&self, inv: store::InventoryType, slot: u16, held: Option<&store::Item>) -> Vec<Reply> {
        let removed = Reply {
            opcode: net::inventory::INVENTORY_OPERATION,
            body: net::inventory::inventory_removed(inv.as_u8() as i8, slot as i16),
            what: format!("InventoryOperation REMOVE: {inv:?} slot {slot}"),
        };
        match held {
            None => vec![removed],
            Some(item) if inv != store::InventoryType::Equip => vec![Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: net::inventory::inventory_quantity(inv.as_u8() as i8, slot as i16, item.kind.quantity()),
                what: format!("InventoryOperation QUANTITY: {inv:?} slot {slot} now holds {} of item {}", item.kind.quantity(), item.item_id),
            }],
            Some(item) => {
                let mut out = vec![removed];
                out.extend(self.inventory_added_replies(
                    inv,
                    &[store::InvItem { inv_type: inv, slot, item: item.clone() }],
                    "the slot as the server holds it",
                ));
                out
            }
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

    /// `info/unitPrice` in thousandths, or `0` for an item the table does not price.
    fn unit_price_milli(&self, item_id: u32) -> u32 {
        self.config.shops.item_data.get(&item_id).map(|d| d.unit_price_milli).unwrap_or(0)
    }

    /// `u8 2` - **recharge** the throwing stars or bullets in one Use-tab slot.
    ///
    /// The owner, 2026-09-06: *"they should be able to recharge stars at general merchants."* The
    /// client sends only the slot; everything else is the server's to work out, and every
    /// step that can refuse does so with a `0x055E`, because the window latched on send.
    ///
    /// **What it costs.** `ceil((slotMax - held) * unitPrice)` whole mesos, `unitPrice` being
    /// the item's own `info/unitPrice` (Subi 0.3 ... Hwabi 1.0, `ItemData::unit_price_milli`).
    /// The rounding direction is **[I]**: the client formats its own *"Recharge: %lld"* and
    /// the arithmetic behind that string has not been read, so the test plan asks for the
    /// number the window shows against the number the meso count moved by.
    ///
    /// **How it lands.** The top-up is a `buy_item` of exactly `slotMax - held` units at the
    /// total price, in one transaction: `place_into_bag` fills existing partial stacks before
    /// it opens a slot, and `need` is by construction what the stack has room for, so the
    /// units land in the slot the player pointed at and the `0x0070` reports that slot's new
    /// count. Pay-then-fail and fail-then-pay are both impossible for the same reason a
    /// purchase cannot half-happen.
    fn classic_recharge(&mut self, slot: u16) -> Vec<Reply> {
        use net::classicshop::RESULT_NOT_ENOUGH_MESOS as REFUSED;
        let Some(chr) = self.claimed_character() else {
            return self.classic_refused(REFUSED, "no character is claimed");
        };
        let Some((_, rows)) = self.open_shop.clone() else {
            return self.classic_refused(REFUSED, "no shop is open");
        };
        let Ok(bag) = self.store.bag_items(chr.id, store::InventoryType::Use) else {
            return self.classic_refused(REFUSED, "could not read the Use tab");
        };
        let Some(held) = bag.iter().find(|r| r.slot == slot) else {
            return self.classic_refused(REFUSED, &format!("recharge: Use slot {slot} is empty"));
        };
        let item_id = held.item.item_id;
        if !net::bag::bundle_has_serial(item_id) {
            return self.classic_refused(
                REFUSED,
                &format!("recharge: item {item_id} in Use slot {slot} is not a star or a bullet"),
            );
        }
        // The window offers Recharge only for an id this counter listed with a unit price, so
        // anything else is a drifted list or a forged body - refused, not priced from the
        // item table. A general store lists every star (recharge-only rows, `open_shop_for`);
        // any other counter recharges only a star it stocks.
        let Some(unit_milli) = rows
            .iter()
            .filter(|r| r.item_id == item_id)
            .map(|r| r.unit_price_milli)
            .find(|m| *m > 0)
        else {
            return self.classic_refused(
                REFUSED,
                &format!("recharge: this counter lists no rechargeable row for item {item_id}"),
            );
        };
        let slot_max = self.config.shops.item_data.get(&item_id).map(|d| d.slot_max).unwrap_or(0);
        if slot_max == 0 {
            return self.classic_refused(
                REFUSED,
                &format!("recharge: item {item_id} has no slotMax in the item table, so a full stack is unknown"),
            );
        }
        let have = held.item.kind.quantity();
        if have >= slot_max {
            return self.classic_refused(
                REFUSED,
                &format!("recharge: Use slot {slot} already holds {have} of {slot_max}"),
            );
        }
        let need = slot_max - have;
        let cost_milli = u64::from(need) * u64::from(unit_milli);
        let cost = u32::try_from(cost_milli.div_ceil(1000)).unwrap_or(u32::MAX);
        match self.store.buy_item(
            chr.id,
            store::InventoryType::Use,
            &store::Item::bundle(item_id, need),
            slot_max,
            cost,
        ) {
            Ok(changed) => {
                let mut out = vec![Reply {
                    opcode: net::classicshop::CLASSIC_SHOP_RESULT,
                    body: net::classicshop::classic_shop_success(item_id, 0),
                    what: format!(
                        "ClassicShopResult success: recharged item {item_id} in Use slot {slot} \
                         {have} -> {slot_max} (+{need}) for {cost} mesos ({need} x {} mesos, \
                         rounded up)",
                        f64::from(unit_milli) / 1000.0
                    ),
                }];
                // The result moves nothing by itself - same as a purchase.
                out.extend(self.inventory_added_replies(store::InventoryType::Use, &changed, "recharged"));
                out.extend(self.meso_reply(chr.id));
                out
            }
            Err(store::StoreError::NotEnoughMesos { .. }) => self.classic_refused(
                REFUSED,
                &format!("recharge: not enough mesos - {need} units at {} each is {cost}", f64::from(unit_milli) / 1000.0),
            ),
            Err(store::StoreError::BagFull { .. }) => self.classic_refused(
                net::classicshop::RESULT_INVENTORY_FULL,
                "recharge: the top-up did not fit the slot it was meant for",
            ),
            Err(e) => self.classic_refused(REFUSED, &format!("recharge failed: {e}")),
        }
    }

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
