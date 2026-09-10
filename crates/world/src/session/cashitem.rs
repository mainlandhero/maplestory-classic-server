//! `0x0114` - Cash-tab items: the five slot coupons and the two reset scrolls.
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
        match req.item_id {
            AP_RESET_SCROLL => self.use_reset_scroll(req.item_id, req.slot, true),
            SP_RESET_SCROLL => self.use_reset_scroll(req.item_id, req.slot, false),
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

    /// The AP or SP reset scroll. `ap` picks which.
    ///
    /// **The effect is the existing `!resetap` / `!resetsp`**, deliberately: those two are
    /// already the reset this server does, they already send the right stat packets, and a
    /// second implementation here would be a second set of rules to keep in step. The owner's
    /// report is that the *items* do nothing, not that the reset is wrong.
    fn use_reset_scroll(&mut self, item_id: u32, slot: u16, ap: bool) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let mut out = if ap { self.gm_reset_ap() } else { self.gm_reset_sp() };
        if out.is_empty() {
            // The reset refused - and it says so through its own replies normally, so an
            // empty answer here means it could not even start. Keep the scroll.
            return self.cash_item_notice(
                "That could not be applied just now. Nothing was used up.".to_string(),
            );
        }
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
        let mut out = crate::mesodrop::unlock_unhandled_latching_request(
            net::cashitem::CLIENT_USE_CASH_ITEM,
        );
        out.push(Reply {
            opcode: net::notice::CHAT_NOTICE,
            body: net::notice::chat_notice(&line),
            what: format!("ChatNotice: {line}"),
        });
        out
    }
}
