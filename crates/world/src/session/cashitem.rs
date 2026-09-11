//! `0x0114` - Cash-tab items: the five slot coupons, the two reset scrolls, and the
//! Signature Style Collection (the box and the eight Outfit Set Coupons).
//!
//! The owner, 2026-09-09: *"I just tried using the Equip expansion coupon"*, and *"Using the AP and
//! SP reset cash items also does not perform the function."* Both are this opcode, which was
//! unhandled - the log answered all three of their attempts with the latch unlock and nothing
//! else. `net::cashitem` has the capture and the decode.
//!
//! # `notConsume` means the server decides, and it decides to consume
//!
//! Every item here carries `info/notConsume = 1`, which is the client saying *"I have not
//! taken this out of the bag; you do."* So each arm below removes it **only after** its effect
//! has succeeded - `CLAUDE.md`'s Heena rule - and a refusal leaves the item where it was.
//!
//! # What this does NOT do
//!
//! It does not run the client's `script` name (`cash_5680000`) as a script, because this
//! server has no script engine and the ids are a closed set of seven. When an eighth arrives
//! it will be a new arm, which is honest, rather than a half-built interpreter.

use super::{Reply, Session};
use crate::slotcoupons::SlotCoupon;

/// `5050100` AP Reset Scroll and `5051001` SP Reset Scroll.
///
/// **Two literals, not a range.** They are not adjacent - `5050100` and `5051001` - so
/// anything range-shaped would either miss one or sweep in the whole `505xxxx` family, which
/// holds other cash items entirely.
pub const AP_RESET_SCROLL: u32 = 5_050_100;
pub const SP_RESET_SCROLL: u32 = 5_051_001;

impl Session {
    /// `0x0114` - the player used something in their Cash tab.
    pub(super) fn on_use_cash_item(&mut self, body: &[u8]) -> Vec<Reply> {
        let unlock = || {
            crate::mesodrop::unlock_unhandled_latching_request(
                net::cashitem::CLIENT_USE_CASH_ITEM,
            )
        };
        let Some(req) = net::cashitem::parse_use_cash_item(body) else {
            crate::server::log(&format!(
                "   cash item: a {} byte 0x0114 body (expected {}); unlock only",
                body.len(),
                net::cashitem::USE_CASH_ITEM_BODY_LEN
            ));
            return unlock();
        };
        let Some(chr) = self.claimed_character() else { return unlock() };

        // **The slot must hold what the packet names.** Same rule as the summoning sack: the
        // client sends both, and trusting the id alone would let a crafted packet spend a
        // coupon out of a slot holding something else.
        let holding = self
            .store
            .bag_items(chr.id, store::InventoryType::Cash)
            .ok()
            .into_iter()
            .flatten()
            .find(|r| r.slot == req.slot)
            .map(|r| r.item.item_id);
        if holding != Some(req.item_id) {
            crate::server::log(&format!(
                "   cash item: character {} asked to use {} from Cash slot {}, which holds \
                 {:?}. Refused; nothing was consumed.",
                chr.id, req.item_id, req.slot, holding
            ));
            return unlock();
        }

        if let Some(coupon) = SlotCoupon::for_item(req.item_id) {
            return self.use_slot_coupon(coupon, req.item_id, req.slot);
        }
        if req.item_id == crate::signaturestyle::COLLECTION {
            return self.open_collection(req.slot);
        }
        if let Some(set) = crate::signaturestyle::set_for_coupon(req.item_id) {
            return self.open_outfit_set(set, req.slot);
        }
        match req.item_id {
            // Still answered here in case a build ever routes them this way; the capture
            // says the client uses 0x0116 - see `on_use_stat_reset_item`.
            AP_RESET_SCROLL => self.use_reset_scroll(
                net::cashitem::CLIENT_USE_CASH_ITEM, req.item_id, req.slot, true,
            ),
            SP_RESET_SCROLL => self.use_reset_scroll(
                net::cashitem::CLIENT_USE_CASH_ITEM, req.item_id, req.slot, false,
            ),
            _ => {
                crate::server::log(&format!(
                    "   cash item: {} is a Cash item this server has no arm for. Unlock only, \
                     and the item is KEPT.",
                    req.item_id
                ));
                unlock()
            }
        }
    }

    /// `0x0116` - the player double-clicked an AP or SP Reset Scroll.
    ///
    /// The owner, 2026-09-10: *"it did not work and it did not take the item."* The two reset arms
    /// below existed and were reachable only from `0x0114`, and the client sends these two
    /// items through `0x0116` - `net::cashitem::CLIENT_USE_STAT_RESET_ITEM` has the capture.
    /// Same body, same slot check, same arms; only the opcode in the unlock differs.
    pub(super) fn on_use_stat_reset_item(&mut self, body: &[u8]) -> Vec<Reply> {
        let opcode = net::cashitem::CLIENT_USE_STAT_RESET_ITEM;
        let unlock = || crate::mesodrop::unlock_unhandled_latching_request(opcode);
        let Some(req) = net::cashitem::parse_use_cash_item(body) else {
            crate::server::log(&format!(
                "   reset scroll: a {} byte 0x0116 body (expected {}); unlock only",
                body.len(),
                net::cashitem::USE_CASH_ITEM_BODY_LEN
            ));
            return unlock();
        };
        let Some(chr) = self.claimed_character() else { return unlock() };
        let holding = self
            .store
            .bag_items(chr.id, store::InventoryType::Cash)
            .ok()
            .into_iter()
            .flatten()
            .find(|r| r.slot == req.slot)
            .map(|r| r.item.item_id);
        if holding != Some(req.item_id) {
            crate::server::log(&format!(
                "   reset scroll: character {} asked to use {} from Cash slot {}, which holds \
                 {:?}. Refused; nothing was consumed.",
                chr.id, req.item_id, req.slot, holding
            ));
            return unlock();
        }
        match req.item_id {
            AP_RESET_SCROLL => self.use_reset_scroll(opcode, req.item_id, req.slot, true),
            SP_RESET_SCROLL => self.use_reset_scroll(opcode, req.item_id, req.slot, false),
            other => {
                crate::server::log(&format!(
                    "   reset scroll: {other} arrived on 0x0116, which this server only knows \
                     for the two reset scrolls. Unlock only, and the item is KEPT."
                ));
                unlock()
            }
        }
    }

    /// One of the five 5-slot coupons.
    fn use_slot_coupon(&mut self, coupon: SlotCoupon, item_id: u32, slot: u16) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        // Storage belongs to the ACCOUNT, not the character, which is why it needs the
        // claim rather than the character record.
        let account = self.claimed().map(|c| c.account_id);
        let current = match coupon {
            SlotCoupon::Storage => match account {
                Some(a) => self.store.storage(a).map(|b| b.slots).unwrap_or(0),
                None => return Vec::new(),
            },
            SlotCoupon::Tab(inv) => self.store.inventory_slots(chr.id, inv).unwrap_or(0),
        };
        // **A refusal that reaches the player.** At the ceiling the coupon is kept, which is
        // the only reading that does not silently spend it for nothing.
        let Some(widened) = crate::slotcoupons::widened(current) else {
            return self.cash_item_notice(format!(
                "Your {} is already at the maximum of {} slots.",
                coupon.what(),
                crate::slotcoupons::MAX_SLOTS
            ));
        };
        let wrote = match coupon {
            SlotCoupon::Storage => {
                account.is_some_and(|a| self.store.set_storage_slots(a, widened).is_ok())
            }
            SlotCoupon::Tab(inv) => {
                self.store.set_inventory_slots(chr.id, inv, widened).is_ok()
            }
        };
        if !wrote {
            return self
                .cash_item_notice("That could not be applied just now. Nothing was used up.".to_string());
        }
        // Only now does the coupon leave the bag.
        let _ = self.store.remove_item(chr.id, store::InventoryType::Cash, slot, Some(1));
        crate::server::log(&format!(
            "   cash item: character {} used {item_id} - {} {current} -> {widened} slots",
            chr.id,
            coupon.what()
        ));
        // **The tab widens on screen now**, which it did not when this shipped an hour ago.
        //
        // That version wrote the row and told the player "change maps or relog to see them",
        // because nothing here could change the count live - the client draws it from the
        // character record it got at field entry. `0x007B` InventoryGrow was decoded while
        // chasing the station clock: `u8 invType, u8 slots`, and the handler resizes
        // `charData + 0x5d0 + invType*8` to `slots + 1`. See `net::inventory::inventory_grow`.
        //
        // **Storage has no such packet and still needs a relog**, so the line says so for
        // that one and not for the others - a caveat printed where it does not apply teaches
        // players to ignore it.
        let mut out = Vec::new();
        let line = match coupon {
            SlotCoupon::Tab(inv) => {
                let slots = u8::try_from(widened).unwrap_or(u8::MAX);
                out.push(Reply {
                    opcode: net::inventory::INVENTORY_GROW,
                    body: net::inventory::inventory_grow(inv.as_u8(), slots),
                    what: format!(
                        "InventoryGrow: the {:?} tab is now {widened} slots. The client adds                          the 1-based hole itself, so this carries the logical count.",
                        inv
                    ),
                });
                format!("Your {} now holds {widened} slots.", coupon.what())
            }
            SlotCoupon::Storage => format!(
                "Your storage now holds {widened} slots. Reopen it to see them.",
            ),
        };
        out.extend(self.cash_item_notice(line));
        out.extend(self.stack_change_replies(store::InventoryType::Cash, slot, 0));
        out
    }

    /// **The Signature Style Collection box: all eight Outfit Set Coupons.** The owner,
    /// 2026-09-10: *"instead of obtaining 1 at random rates, we give them all of the sets"*.
    ///
    /// The eight coupons go into the Cash tab - the same tab the box sits in - and the box's
    /// own slot frees up as it is consumed, so seven free Cash slots are enough. That is
    /// checked BEFORE anything is written: an all-or-nothing hand-out is the only reading in
    /// which "the box did nothing" and "the box gave me half" cannot be confused.
    fn open_collection(&mut self, slot: u16) -> Vec<Reply> {
        let wares: Vec<(u32, store::InventoryType)> = crate::signaturestyle::SETS
            .iter()
            .map(|s| (s.coupon, store::InventoryType::Cash))
            .collect();
        self.hand_out(
            crate::signaturestyle::COLLECTION,
            slot,
            &wares,
            "Signature Style Collection",
            "all eight Outfit Set Coupons",
        )
    }

    /// **One Outfit Set Coupon: its hair coupons, its face coupon, and its equips.** The
    /// contents are `crate::signaturestyle`, which is the owner's listing; the modern client keeps
    /// this rule in a server script the WZ only names.
    fn open_outfit_set(&mut self, set: &crate::signaturestyle::OutfitSet, slot: u16) -> Vec<Reply> {
        let wares = crate::signaturestyle::set_contents(set);
        self.hand_out(set.coupon, slot, &wares, &format!("{} Outfit Set", set.name), "its outfit, hair and face")
    }

    /// Consume one Cash-tab item at `slot` and hand out `wares`, all or nothing.
    ///
    /// Room is counted first, per tab, and the source item's own Cash slot counts as free
    /// because it is removed in the same hand-out. A refusal names the tab and how many
    /// slots it is short, and consumes nothing. A store error part-way is logged loudly - it
    /// cannot be undone here, and the log line is how it would be found.
    fn hand_out(
        &mut self,
        item_id: u32,
        slot: u16,
        wares: &[(u32, store::InventoryType)],
        label: &str,
        contents: &str,
    ) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let Ok(bag) = self.store.bag(chr.id) else {
            return self.cash_item_notice("That could not be applied just now. Nothing was used up.".to_string());
        };
        // **The tab is the config's, not the listing's.** A cash equip goes to the Deco tab
        // (`Config::tab_for`); the Übel set's four went into the Equip tab on 2026-09-11 and
        // the Deco tab the owner opened was empty.
        let wares: Vec<(u32, store::InventoryType)> =
            wares.iter().map(|&(id, tab)| (id, self.config.tab_for(id).unwrap_or(tab))).collect();
        let wares = &wares[..];
        let need = crate::signaturestyle::slots_needed(wares);
        for tab in store::InventoryType::ALL {
            let used = bag.items_in(tab).count() as u16;
            // The source item's own Cash slot counts as free: it is removed in this hand-out.
            let reclaim = u16::from(tab == store::InventoryType::Cash);
            let have = bag.slots[tab.index()].saturating_sub(used).saturating_add(reclaim);
            let need = need[tab.index()];
            if need > have {
                return self.cash_item_notice(format!(
                    "Your {} tab needs {need} free slot(s) for the {label} and has {have}. Nothing was used up.",
                    net::bag::BAG_TAB_NAMES[tab.index()],
                ));
            }
        }

        // Only now does the source leave the bag - first, so its Cash slot is free for what
        // follows, and because a hand-out that fails after this point is a logged fault,
        // not a player keeping the box.
        if self.store.remove_item(chr.id, store::InventoryType::Cash, slot, Some(1)).is_err() {
            return self.cash_item_notice("That could not be applied just now. Nothing was used up.".to_string());
        }
        let mut out = crate::mesodrop::unlock_unhandled_latching_request(net::cashitem::CLIENT_USE_CASH_ITEM);
        out.extend(self.stack_change_replies(store::InventoryType::Cash, slot, 0));
        let mut given = Vec::new();
        for &(id, tab) in wares {
            let item = if matches!(tab, store::InventoryType::Equip | store::InventoryType::Deco) {
                store::Item::equip(id)
            } else {
                store::Item::bundle(id, 1)
            };
            match self.store.add_item(chr.id, tab, &item, 1) {
                Ok(rows) => {
                    out.extend(self.inventory_added_replies(tab, &rows, label));
                    given.push(id);
                }
                Err(e) => {
                    // The room was counted; this is a store fault, and it must be loud.
                    crate::server::log(&format!(
                        "   cash item: {label}: FAILED to hand out {id} into {tab:?} after {} of {}                          ({e}) - the room was counted free before the source was consumed.                          Given so far: {given:?}",
                        given.len(),
                        wares.len()
                    ));
                }
            }
        }
        crate::server::log(&format!(
            "   cash item: character {} opened {item_id} ({label}) -> {} item(s): {given:?}",
            chr.id,
            given.len()
        ));
        out.push(Reply {
            opcode: net::notice::CHAT_NOTICE,
            body: net::notice::chat_notice(&format!("{label}: you received {contents} ({} items).", given.len())),
            what: format!("ChatNotice: {label} opened, {} item(s) given", given.len()),
        });
        out
    }

    /// The AP or SP reset scroll. `ap` picks which.
    ///
    /// **The effect is the existing `!resetap` / `!resetsp`**, deliberately: those two are
    /// already the reset this server does, they already send the right stat packets, and a
    /// second implementation here would be a second set of rules to keep in step. The owner's
    /// report is that the *items* do nothing, not that the reset is wrong.
    fn use_reset_scroll(&mut self, opcode: u16, item_id: u32, slot: u16, ap: bool) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let mut out = if ap { self.gm_reset_ap() } else { self.gm_reset_sp() };
        if out.is_empty() {
            // The reset refused - and it says so through its own replies normally, so an
            // empty answer here means it could not even start. Keep the scroll.
            return self.cash_item_notice_for(
                opcode,
                "That could not be applied just now. Nothing was used up.".to_string(),
            );
        }
        // **The latch, too.** The reset's own replies are stat packets and a chat line; none
        // of them is the unlock this opcode's builder is waiting for, and without it the
        // inventory stays frozen after a successful reset.
        out.extend(crate::mesodrop::unlock_unhandled_latching_request(opcode));
        let _ = self.store.remove_item(chr.id, store::InventoryType::Cash, slot, Some(1));
        crate::server::log(&format!(
            "   cash item: character {} used {item_id} - {} reset",
            chr.id,
            if ap { "AP" } else { "SP" }
        ));
        out.extend(self.stack_change_replies(store::InventoryType::Cash, slot, 0));
        out
    }

    /// A notice plus the unlock. **Both**, because this opcode latches: a chat line alone
    /// would tell the player what happened and still freeze their inventory.
    fn cash_item_notice(&mut self, line: String) -> Vec<Reply> {
        self.cash_item_notice_for(net::cashitem::CLIENT_USE_CASH_ITEM, line)
    }

    /// The same, naming the opcode whose latch the unlock clears - `0x0114` or `0x0116`.
    fn cash_item_notice_for(&mut self, opcode: u16, line: String) -> Vec<Reply> {
        let mut out = crate::mesodrop::unlock_unhandled_latching_request(opcode);
        out.push(Reply {
            opcode: net::notice::CHAT_NOTICE,
            body: net::notice::chat_notice(&line),
            what: format!("ChatNotice: {line}"),
        });
        out
    }
}
