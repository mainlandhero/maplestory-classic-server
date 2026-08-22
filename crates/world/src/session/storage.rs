//! Mr. Kim's storage box, on the wire.
//!
//! The owner, 2026-08-22: *"Mr. Kim the storage keeper does not open the storage UI."*
//!
//! # Built is not wired, for days
//!
//! `crates/store/src/storage.rs` has had `storage`, `storage_slot`, `storage_deposit`,
//! `storage_withdraw`, `store_item` and `take_item` since before this file existed, backed by
//! real `storage` and `storage_item` tables. **Every one of them worked and none of them had
//! a caller.** `grep -rn storage crates/world crates/net` found nothing at all. On screen that
//! is indistinguishable from a feature nobody has started, which is exactly the failure mode
//! `CLAUDE.md`'s "Built is not wired" section exists for.
//!
//! What was missing was two opcodes, and `research/storage.md` found them without a client
//! run: `0x0572` out and `0x00F6` in.
//!
//! # Always answer, and here the latch has a name
//!
//! Every `0x00F6` sets `dlg+0x334` in the client, and **only an inbound `0x0572` clears it**.
//! A refusal that sends nothing leaves the storage window open with every button dead until
//! the player closes it by hand - so every path out of [`Session::on_storage_request`] emits
//! a `0x0572`, including the ones that change nothing.
//!
//! # Items move now, and the fee is real
//!
//! The owner, 2026-08-22, after the first run that ever opened the window: *"I tried to store an
//! item with Mr. Kim. The item did not move to storage, and it did not charge the 100 meso
//! fee that it said it was going to charge."* Both sentences were the same missing arm -
//! `PutIn` and `TakeOut` fell through to "not implemented yet, here is the unchanged box".
//! The **fee text is the client's own**, read out of `Npc.wz` before anything is sent, so an
//! unbuilt deposit reads on screen as a fee that was promised and then not taken.
//!
//! # The box is per ACCOUNT, and that is the one thing the client cannot confirm
//!
//! Nothing on the wire in either direction carries an owner - not the open packet, not any
//! request. `crates/store` keys on `account_id`, and `research/storage.md` §9 keeps that on
//! The owner's own instruction plus one weak reading (a refusal string names the *account*). It is
//! marked **[I]** there and it is marked [I] here: two characters on one account sharing a box
//! is the behaviour this ships, and one launch with two characters would settle it.

use super::*;

impl Session {
    /// A storage keeper was clicked. `None` if `template` is not one.
    ///
    /// Sits beside `open_shop_for` in `on_npc_click` and before the conversation fallback,
    /// for the same reason: a keeper opens a window instead of talking, and Mr. Kim has no
    /// `d0` line to fall back to anyway - which is precisely why clicking them did nothing
    /// visible rather than doing something wrong.
    pub(super) fn open_storage_for(&mut self, template: u32) -> Option<Vec<Reply>> {
        let fee = net::storage::storage_fee(template)?;
        let claimed = self.claimed()?;
        let account_id = claimed.account_id;
        let boxx = match self.store.storage(account_id) {
            Ok(b) => b,
            Err(e) => {
                return Some(self.notice(format!("Your storage could not be opened: {e}")));
            }
        };
        let (slots, mesos, count) = (boxx.slots, boxx.mesos, boxx.items.len());
        let per_type = self.storage_blobs(&boxx);
        // The fee is the keeper's, and a put-in request does not name them. Remember it here
        // or the deposit has to guess.
        self.open_storage = Some(template);
        Some(vec![Reply {
            opcode: net::storage::STORAGE_RESULT,
            body: net::storage::open_storage(
                template,
                slots.min(u16::from(u8::MAX)) as u8,
                u64::from(mesos),
                &per_type,
            ),
            what: format!(
                "StorageResult OPEN at NPC template {template} (deposit fee {fee}, withdrawing is free on all ten keepers) for account {account_id}: {count} item(s), {mesos} mesos, {slots} slots. The template id, NOT the object id - the client reads Npc.wz from it to find the fee"
            ),
        }])
    }

    /// `0x00F6` - take out, put in, sort, move mesos, close.
    ///
    /// **Every branch answers.** See the module docs: the client latches on send and only a
    /// `0x0572` releases it.
    pub(super) fn on_storage_request(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(claimed) = self.claimed() else { return Vec::new() };
        let account_id = claimed.account_id;
        let Some(req) = net::storage::StorageRequest::parse(body) else {
            // A body we cannot read is our problem, not a reason to wedge the window.
            return self.storage_refusal(
                net::storage::RESULT_TRUNK_REFRESH,
                account_id,
                format!("unreadable 0x00F6 body {body:02x?} - answered anyway to clear the latch"),
            );
        };
        match req {
            net::storage::StorageRequest::Close => {
                self.open_storage = None;
                Vec::new()
            }
            net::storage::StorageRequest::Sort => self.storage_refusal(
                net::storage::RESULT_TRUNK_REFRESH,
                account_id,
                "sort: the box is re-sent unchanged, which is a legal no-op".to_string(),
            ),
            net::storage::StorageRequest::Mesos { amount } => {
                self.storage_mesos(account_id, amount)
            }
            net::storage::StorageRequest::PutIn { bag_slot, item_id, count } => {
                self.storage_put_in(account_id, bag_slot, item_id, count)
            }
            net::storage::StorageRequest::TakeOut { inv_type, position, count } => {
                self.storage_take_out(account_id, inv_type, position, count)
            }
        }
    }

    /// Mode 5 - **bag to box, and the keeper takes their fee.**
    ///
    /// The owner, 2026-08-22: *"I tried to store an item with Mr. Kim. The item did not move to
    /// storage, and it did not charge the 100 meso fee that it said it was going to charge."*
    /// Both halves of that were one missing arm: the request parsed, and the answer was the
    /// unchanged box. The fee is the client's own text, read out of `Npc.wz` - it announces
    /// what the server is *going to* do, so an unimplemented deposit reads on screen as a fee
    /// that was promised and not taken.
    ///
    /// # Every effect hangs off the transition
    ///
    /// The order here is the one `CLAUDE.md`'s repeated-quest bug bought: the store is asked
    /// **first**, and the fee, the `0x0070` and the refreshed box are all reached only
    /// through its `Ok`. The two pre-checks above it - a free slot, and enough mesos - exist
    /// so the *refusal mode* can be specific (17 storage full, 16 cannot afford the fee)
    /// rather than a bare refresh; they are not what protects the money.
    ///
    /// The fee is charged **after** a successful move, and it cannot fail after the pre-check
    /// except on a database error, which is logged rather than swallowed. Charging first and
    /// then failing to move would take mesos for nothing, which is the direction that costs
    /// the player.
    fn storage_put_in(
        &mut self,
        account_id: i64,
        bag_slot: u16,
        item_id: u32,
        count: u16,
    ) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let refresh = net::storage::RESULT_TRUNK_REFRESH;

        // **The window must be open, because the fee lives on the keeper.** This is not
        // defensive noise: `storage_fee` is per template and Mr. Thalj charges 150.
        let Some(template) = self.open_storage else {
            return self.storage_refusal(
                refresh,
                account_id,
                format!("put-in of item {item_id} with no storage window open - refusing rather than inventing a deposit fee"),
            );
        };
        let fee = net::storage::storage_fee(template).unwrap_or(0);

        // Which bag is it in? The request carries the slot and the item id but not the type.
        // Deriving the type from the item id alone is the thing `store::take_item`'s doc
        // block warns about - only the equip case of `for_item` is corroborated - so the
        // bags are asked instead, and the slot must really hold that item id.
        let Some(inv) = self.bag_holding(chr.id, bag_slot, item_id) else {
            return self.storage_refusal(
                refresh,
                account_id,
                format!("no bag has item {item_id} in slot {bag_slot} - nothing moved"),
            );
        };

        match self.store.storage(account_id) {
            Ok(boxx) if boxx.free_slots() == 0 => {
                return self.storage_refusal(
                    net::storage::RESULT_STORAGE_FULL,
                    account_id,
                    format!("storage is full ({} slots)", boxx.slots),
                )
            }
            Ok(_) => {}
            Err(e) => {
                return self.storage_refusal(
                    refresh,
                    account_id,
                    format!("the box could not be read: {e}"),
                )
            }
        }
        let purse = self.store.mesos(chr.id).unwrap_or(0);
        if purse < fee {
            return self.storage_refusal(
                net::storage::RESULT_NOT_ENOUGH_FEE,
                account_id,
                format!("the deposit fee at template {template} is {fee} and the purse holds {purse}"),
            );
        }

        let max_stack = self.config.shops.max_stack(item_id);
        let moved = self.store.store_item(
            account_id,
            chr.id,
            inv,
            bag_slot,
            (count > 0).then_some(count),
            max_stack,
        );
        let placed = match moved {
            Ok(rows) => rows,
            Err(store::StoreError::ItemMayNotBeStored { item_id }) => {
                return self.storage_refusal(
                    net::storage::RESULT_TRUNK_REFRESH,
                    account_id,
                    format!("item {item_id} may not be stored"),
                )
            }
            Err(store::StoreError::StorageFull { slots }) => {
                return self.storage_refusal(
                    net::storage::RESULT_STORAGE_FULL,
                    account_id,
                    format!("storage is full ({slots} slots)"),
                )
            }
            Err(e) => {
                return self.storage_refusal(
                    refresh,
                    account_id,
                    format!("the deposit failed and nothing moved: {e}"),
                )
            }
        };

        // Only now. The item is in the box and out of the bag, in one transaction.
        let mut out = self.bag_slot_replies(chr.id, inv, bag_slot, "stored");
        match self.store.add_mesos(chr.id, -i64::from(fee)) {
            Ok(_) => out.extend(self.meso_reply(chr.id)),
            Err(e) => out.extend(self.notice(format!(
                "The item was stored but the {fee} meso fee could not be taken: {e}"
            ))),
        }
        let into: Vec<u16> = placed.iter().map(|r| r.slot).collect();
        out.extend(self.storage_refusal(
            net::storage::RESULT_PUT_OK,
            account_id,
            format!(
                "stored item {item_id} from {inv:?} slot {bag_slot} into storage slot(s) {into:?}, fee {fee} at template {template}"
            ),
        ));
        out
    }

    /// Mode 4 - **box to bag.** Free on all ten keepers.
    ///
    /// # `position` is not a slot, and this is the only place that can go wrong quietly
    ///
    /// The client hands back the 0-based position **within its inventory type, in the order
    /// the server last sent the box**. Reading it as a storage slot would take the wrong item
    /// whenever the box is not densely packed from slot 1 - and it would succeed, silently,
    /// with the wrong item. [`Self::storage_order`] is the one function that decides that
    /// ordering, and [`Self::storage_blobs`] builds the wire from the same array, so the
    /// two cannot drift.
    fn storage_take_out(
        &mut self,
        account_id: i64,
        inv_type: u8,
        position: u8,
        count: u16,
    ) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let refresh = net::storage::RESULT_TRUNK_REFRESH;

        let Ok(inv) = store::InventoryType::from_wire(i16::from(inv_type)) else {
            return self.storage_refusal(
                refresh,
                account_id,
                format!("take-out named inventory type {inv_type}, which is not a bag"),
            );
        };
        let boxx = match self.store.storage(account_id) {
            Ok(b) => b,
            Err(e) => {
                return self.storage_refusal(
                    refresh,
                    account_id,
                    format!("the box could not be read: {e}"),
                )
            }
        };
        let order = Self::storage_order(&boxx);
        let idx = inv.index();
        let Some(row) = order.get(idx).and_then(|list| list.get(usize::from(position))) else {
            return self.storage_refusal(
                refresh,
                account_id,
                format!("no item at position {position} of {inv:?} - the box holds {} there", order.get(idx).map_or(0, Vec::len)),
            );
        };
        let (slot, item_id) = (row.slot, row.item.item_id);

        let max_stack = self.config.shops.max_stack(item_id);
        let taken = self.store.take_item(
            account_id,
            chr.id,
            slot,
            (count > 0).then_some(count),
            inv,
            max_stack,
        );
        let placed = match taken {
            Ok(rows) => rows,
            Err(store::StoreError::BagFull { slots, .. }) => {
                return self.storage_refusal(
                    net::storage::RESULT_INVENTORY_FULL,
                    account_id,
                    format!("{inv:?} is full ({slots} slots) - the item stays in the box"),
                )
            }
            Err(e) => {
                return self.storage_refusal(
                    refresh,
                    account_id,
                    format!("the withdrawal failed and nothing moved: {e}"),
                )
            }
        };

        let mut out = self.inventory_added_replies(inv, &placed, "taken out of storage");
        out.extend(self.storage_refusal(
            net::storage::RESULT_PUT_OK,
            account_id,
            format!(
                "took item {item_id} out of storage slot {slot} (position {position} of {inv:?}) into bag slot(s) {:?}",
                placed.iter().map(|r| r.slot).collect::<Vec<_>>()
            ),
        ));
        out
    }

    /// Which bag holds `item_id` at `bag_slot`, if any.
    ///
    /// Asked of the bags rather than derived from the item id: `InventoryType::for_item` is
    /// corroborated for equips only, and a wrong answer here would hand `store_item` a slot
    /// in the wrong bag - which either finds nothing, or finds a **different item** at the
    /// same slot number and stores that instead. The item id is checked, not just the slot,
    /// for exactly that reason.
    fn bag_holding(
        &self,
        character_id: u32,
        bag_slot: u16,
        item_id: u32,
    ) -> Option<store::InventoryType> {
        store::InventoryType::ALL.iter().copied().find(|inv| {
            self.store.bag_items(character_id, *inv).is_ok_and(|rows| {
                rows.iter().any(|r| r.slot == bag_slot && r.item.item_id == item_id)
            })
        })
    }

    /// The `0x0070` for a bag slot **after** something left it: gone, or merely smaller.
    ///
    /// A whole-slot deposit needs a Remove and a partial one needs a Quantity, and the
    /// difference is not visible from the request - `count` is what the player asked for, not
    /// what happened. So the slot is read back. Sending the wrong one of these two leaves a
    /// ghost item in the client's grid that only a relog clears.
    fn bag_slot_replies(
        &self,
        character_id: u32,
        inv: store::InventoryType,
        slot: u16,
        why: &str,
    ) -> Vec<Reply> {
        let left = self
            .store
            .bag_items(character_id, inv)
            .ok()
            .and_then(|rows| rows.iter().find(|r| r.slot == slot).map(|r| r.item.kind.quantity()));
        match left {
            Some(q) if q > 0 => vec![Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: net::inventory::inventory_quantity(inv.as_u8() as i8, slot as i16, q),
                what: format!(
                    "InventoryOperation QUANTITY: {inv:?} slot {slot} now holds {q} - {why}."
                ),
            }],
            _ => vec![Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: net::inventory::inventory_removed(inv.as_u8() as i8, slot as i16),
                what: format!("InventoryOperation REMOVE: {inv:?} slot {slot} is empty - {why}."),
            }],
        }
    }

    /// The box grouped by inventory type, **in the order the wire uses**.
    ///
    /// One function, two callers: the encoder below and the take-out resolver above. That is
    /// deliberate - `research/storage.md` §11.5 lists this as the first of "the three numbers
    /// that must not drift", because a take-out index that means a different item than the
    /// one the client is pointing at fails by moving the **wrong item**, with no error
    /// anywhere.
    fn storage_order(boxx: &store::StorageBox) -> [Vec<store::StorageItem>; net::storage::INVENTORY_TYPES]
    {
        let mut out: [Vec<store::StorageItem>; net::storage::INVENTORY_TYPES] = Default::default();
        // `StorageBox::items` is documented as "occupied slots only, in slot order", which is
        // what makes the position stable between two sends.
        for it in &boxx.items {
            let Some(inv) = store::InventoryType::for_item(it.item.item_id) else { continue };
            let idx = inv.index();
            if idx < out.len() {
                out[idx].push(*it);
            }
        }
        out
    }

    /// Move mesos between the purse and the box.
    ///
    /// # The two sign conventions are opposite, and this is the only place they meet
    ///
    /// **On the wire** (`0x00F6` mode 7) a *positive* amount **withdraws** from the box into
    /// the purse. **In the store** `move_storage_mesos` takes a positive amount to mean
    /// **deposit** - *"Positive deposits, negative withdraws"*, straight from its doc block.
    ///
    /// So the wire value is **negated** on the way in. Getting this backwards would not
    /// error, would not crash, and would not look wrong in any log: it would quietly move
    /// money the other way, which is the worst shape a bug can have. There is a test.
    ///
    /// The store does both halves in one transaction, which is the property that matters -
    /// mesos that leave one side and never arrive at the other are the same duplication bug
    /// as an item in two containers.
    fn storage_mesos(&mut self, account_id: i64, amount: i64) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let direction = if amount >= 0 { "withdraw" } else { "deposit" };
        match self.store.move_storage_mesos(account_id, chr.id, -amount) {
            Ok((purse, boxed)) => {
                let mut out = self.storage_refusal(
                    net::storage::RESULT_TRUNK_REFRESH,
                    account_id,
                    format!(
                        "{direction} {}: purse now {purse}, box now {boxed}. The wire's sign is INVERTED from the store's - positive on 0x00F6 means withdraw",
                        amount.abs()
                    ),
                );
                // The stat block has no meso field at all, so `0x007C` bit 18 is the only way
                // this client is ever told a balance.
                out.extend(self.meso_reply(chr.id));
                out
            }
            Err(store::StoreError::NotEnoughMesos { have, want }) => self.storage_refusal(
                net::storage::RESULT_NOT_ENOUGH_MESOS,
                account_id,
                format!("tried to {direction} {want} with {have} available"),
            ),
            Err(e) => self.storage_refusal(
                net::storage::RESULT_TRUNK_REFRESH,
                account_id,
                format!("meso move failed: {e}"),
            ),
        }
    }

    /// Send the box as it currently stands, with `mode` on the front.
    ///
    /// Used for refusals and for no-ops alike: in both cases the right answer is "here is the
    /// box, unchanged", and the mode byte is what tells the client which message to show.
    fn storage_refusal(&mut self, mode: u8, account_id: i64, why: String) -> Vec<Reply> {
        // A box that cannot be read still has to produce a packet, or the latch never
        // clears. An empty one is the honest fallback: it says "nothing in here" rather than
        // inventing contents, and the `what` line below names the mode so the log says which
        // path produced it.
        let boxx = match self.store.storage(account_id) {
            Ok(b) => b,
            Err(_) => store::StorageBox { mesos: 0, slots: 0, items: Vec::new() },
        };
        let (slots, mesos) = (boxx.slots, boxx.mesos);
        let per_type = self.storage_blobs(&boxx);
        vec![Reply {
            opcode: net::storage::STORAGE_RESULT,
            body: net::storage::storage_refresh(
                mode,
                slots.min(u16::from(u8::MAX)) as u8,
                u64::from(mesos),
                &per_type,
            ),
            what: format!("StorageResult mode {mode}: {why}. Every 0x00F6 is answered - the client latches dlg+0x334 on send and only a 0x0572 clears it"),
        }]
    }

    /// The six per-type item lists, encoded with the **bag's** encoders.
    ///
    /// Storage and the bag are decoded by the same client function, so there is exactly one
    /// item encoding in this server and this is not it - `equipped_item` and `bundle_item`
    /// already emit the leading type byte.
    fn storage_blobs(
        &self,
        boxx: &store::StorageBox,
    ) -> [Vec<Vec<u8>>; net::storage::INVENTORY_TYPES] {
        let mut out: [Vec<Vec<u8>>; net::storage::INVENTORY_TYPES] = Default::default();
        for (idx, list) in Self::storage_order(boxx).iter().enumerate() {
            for it in list {
                out[idx].push(match it.item.kind {
                    store::ItemKind::Equip(stats) => {
                        let stats = stats.unwrap_or_else(|| self.template_stats(it.item.item_id));
                        net::opcode::equipped_item(it.item.item_id, &stats)
                    }
                    store::ItemKind::Bundle { quantity } => net::bag::bundle_item(
                        it.item.item_id,
                        quantity,
                        0,
                        &[0u8; net::bag::BUNDLE_OWNER_LEN],
                    ),
                });
            }
        }
        out
    }
}
