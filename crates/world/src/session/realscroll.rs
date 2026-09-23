//! `0x0125` - **the client's own scrolling window**, which is how a player actually scrolls.
//!
//! The owner, 2026-09-09: *"Just tried scrolling the topwear, it did not work."* It did not, and
//! the log said so in as many words:
//!
//! ```text
//! 00:59:09.274 <- 0x0125 UNKNOWN, 11 byte body 5696fc000d000100fbff00
//! 00:59:09.275 -> 0x007C StatChanged: UNLOCK ONLY. 0x0125 is not handled ...
//! ```
//!
//! The opcode had been **decoded in full** on 2026-09-09 - request, result, the four-way
//! result code, the field effect, all of it in `research/scrolling-2026-09-09.md` - and never
//! wired. `CLAUDE.md`'s "built is not wired", and the third instance found in one day.
//!
//! # This is a different feature from `!scroll`, and they share only the rules
//!
//! `!scroll` is the Maple Administrator's dialogue for two items this client's data does not
//! contain. **This is the ordinary game mechanic**: 208 real scrolls that already exist, each
//! with its own success rate, destroy rate and stat increments, dragged onto an equip in the
//! inventory window. They meet in `crate::scrolls`, which owns every rule and knows about
//! neither.
//!
//! # Always answer, and `result = 3` is what makes that possible
//!
//! The client latches `ctx+0x2330` when it builds this packet, so an unanswered scroll does
//! not merely fail - it kills the inventory, the ability buttons and the cash shop for the
//! rest of the session. Every path out of here sends something, and a refusal is a real
//! `0x0236` with `result = 3` rather than silence.
//!
//! **No `0x00B8`.** The research first concluded that an extra unlock packet was needed here;
//! that correction was itself corrected the same day - `0x007C` already clears the latch, and
//! it is confirmed on the owner's screen. `0x0070` carries `bExclRequestSent = 1` for the same
//! reason, so the reply pair unlocks twice over.

use super::{Reply, Session};
use crate::config::ScrollTemplate;
use crate::scrolls::{self, EquipBase, EquipState};
use net::upgrade::{
    item_upgrade_effect, ItemUpgradeRequest, ItemUpgradeResult, ITEM_UPGRADE_EFFECT,
};

/// Where the equip being scrolled lives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    /// A worn item. `dstSlot` was negative and this is its magnitude.
    Worn(u8),
    /// An equip sitting in the Equip tab. `dstSlot` was positive.
    Bagged(u16),
}

impl Target {
    /// **The sign of `dstSlot` is the whole discriminator.** Read unsigned, the owner's `-5` is
    /// 65531 and every worn-item scroll would refuse - and wearing it is the usual way to
    /// scroll something.
    fn from_dst_slot(dst_slot: i16) -> Option<Self> {
        match dst_slot {
            0 => None,
            n if n < 0 => u8::try_from(-i32::from(n)).ok().map(Target::Worn),
            n => Some(Target::Bagged(n as u16)),
        }
    }
}

impl Session {
    /// `0x0125` - put a real scroll on an equip.
    pub(super) fn on_item_upgrade(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(req) = net::upgrade::parse_item_upgrade(body) else {
            // Unreadable: still answer, or the UI freezes. There is no character id to put in
            // a `0x0236`, so this is the one path that uses the bare unlock.
            crate::server::log(&format!(
                "   scroll: a {} byte 0x0125 body (expected {}); answering with the unlock only",
                body.len(),
                net::upgrade::ITEM_UPGRADE_REQUEST_LEN
            ));
            return crate::mesodrop::unlock_unhandled_latching_request(
                net::upgrade::CLIENT_ITEM_UPGRADE,
            );
        };
        let Some(chr) = self.claimed_character() else {
            return crate::mesodrop::unlock_unhandled_latching_request(
                net::upgrade::CLIENT_ITEM_UPGRADE,
            );
        };
        self.upgrade_with(chr.id, self.field_of(&chr), &req)
    }

    fn upgrade_with(
        &mut self,
        character_id: u32,
        map: crate::fields::FieldKey,
        req: &ItemUpgradeRequest,
    ) -> Vec<Reply> {
        // ---- the scroll -------------------------------------------------------------
        let Some(scroll) = self.bag_slot_item(character_id, store::InventoryType::Use, req.src_slot)
        else {
            return self.upgrade_refused(character_id, map, 0, 0, "no item in that Use slot");
        };
        let scroll_id = scroll.item_id;
        let Some(template) = self.config.scrolls.get(&scroll_id).copied() else {
            return self.upgrade_refused(
                character_id,
                map,
                scroll_id,
                0,
                "that item is not a scroll in this client's 0204.img",
            );
        };

        // ---- the equip --------------------------------------------------------------
        let Some(target) = Target::from_dst_slot(req.dst_slot) else {
            return self.upgrade_refused(character_id, map, scroll_id, 0, "dstSlot names no slot");
        };
        let Some((equip_id, stored, failed_slots)) = self.read_target(character_id, target) else {
            return self.upgrade_refused(
                character_id,
                map,
                scroll_id,
                0,
                "there is no equipment in that slot",
            );
        };

        // **The category check, and it is the client's own id scheme.** A hat scroll on a
        // weapon is refused rather than applied; `ScrollTemplate::category`'s derivation is
        // controlled against the names of all 208 scrolls.
        if !ScrollTemplate::fits(scroll_id, equip_id) {
            return self.upgrade_refused(
                character_id,
                map,
                scroll_id,
                equip_id,
                "that scroll was not made for that kind of equipment",
            );
        }

        let base = self.equip_template_base(equip_id);
        let current =
            stored.unwrap_or_else(|| net::opcode::EquipStats::fresh(base.stats, base.tuc));
        let state = EquipState {
            remaining: current.options.remaining_enhancements,
            failed_slots,
            stats: current.stats,
        };

        let applied = match scrolls::apply_real(
            &base,
            &state,
            template.success,
            template.cursed,
            &template.increments,
            self.next_scroll_roll(),
        ) {
            Ok(a) => a,
            Err(refusal) => {
                return self.upgrade_refused(
                    character_id,
                    map,
                    scroll_id,
                    equip_id,
                    refusal.line(),
                )
            }
        };

        // ---- write, in the order that cannot lose an item ---------------------------
        //
        // The equip first: if it will not take the write, nothing has been consumed and the
        // refusal is honest. Only then does the scroll leave the bag. `CLAUDE.md`'s Heena
        // rule - every effect hangs off the transition.
        let mut new_stats = current;
        new_stats.stats = applied.after.stats;
        new_stats.options.remaining_enhancements = applied.after.remaining;

        let wrote = if applied.destroyed {
            self.destroy_target(character_id, target)
        } else {
            self.write_target(character_id, target, equip_id, &new_stats, applied.after.failed_slots)
        };
        if !wrote {
            return self.upgrade_refused(
                character_id,
                map,
                scroll_id,
                equip_id,
                "the equipment would not take the write; nothing was consumed",
            );
        }
        let taken =
            self.store.remove_item(character_id, store::InventoryType::Use, req.src_slot, Some(1));
        if taken.is_err() {
            crate::server::log(
                "   scroll: the equip was written but the scroll would not leave the bag",
            );
        }

        // ---- tell the client --------------------------------------------------------
        let result = if applied.destroyed {
            ItemUpgradeResult::Destroyed
        } else {
            ItemUpgradeResult::from_success(applied.succeeded)
        };
        let mut out = vec![self.upgrade_effect(character_id, map, result, scroll_id, equip_id)];

        // The scroll's own slot: mode 1 with what is left, or mode 3 when it was the last one.
        out.push(self.scroll_slot_reply(character_id, req.src_slot, scroll_id));

        // And the equip itself.
        out.push(match target {
            Target::Worn(slot) if applied.destroyed => Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: net::inventory::inventory_removed(
                    store::InventoryType::Equip.as_u8() as i8,
                    -i16::from(slot),
                ),
                what: format!(
                    "InventoryOperation REMOVE: {equip_id} was DESTROYED by scroll {scroll_id} \
                     ({}% cursed). 0x0236 alone prints the message and leaves the item on \
                     screen, so this is what actually takes it away.",
                    template.cursed
                ),
            },
            Target::Bagged(slot) if applied.destroyed => Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: net::inventory::inventory_removed(
                    store::InventoryType::Equip.as_u8() as i8,
                    slot as i16,
                ),
                what: format!(
                    "InventoryOperation REMOVE: {equip_id} in Equip slot {slot} was DESTROYED \
                     by scroll {scroll_id}."
                ),
            },
            _ => {
                let refreshed = store::Item {
                    item_id: equip_id,
                    kind: store::ItemKind::Equip(Some(new_stats)),
                    failed_slots: applied.after.failed_slots,
                    pet_id: None,
                };
                let blob = self.item_blob(&refreshed);
                let pos = match target {
                    Target::Worn(slot) => -i16::from(slot),
                    Target::Bagged(slot) => slot as i16,
                };
                Reply {
                    opcode: net::inventory::INVENTORY_OPERATION,
                    body: net::inventory::inventory_added(
                        store::InventoryType::Equip.as_u8() as i8,
                        pos,
                        &blob,
                    ),
                    what: format!(
                        "InventoryOperation ADD: re-sending {equip_id} at position {pos} so the \
                         tooltip shows the new stats and {} enhancement slot(s)",
                        applied.after.remaining
                    ),
                }
            }
        });

        crate::server::log(&format!(
            "   scroll: character {character_id} used {scroll_id} ({}% / {}% cursed) on \
             {equip_id}: {result:?}, remaining {} -> {}, failed {} -> {}, changes {:?}. \
             Nothing authenticates.",
            template.success,
            template.cursed,
            state.remaining,
            applied.after.remaining,
            state.failed_slots,
            applied.after.failed_slots,
            applied.changes,
        ));
        out
    }

    /// `0x0236` to the map and back to the scroller. Same two-send rule as `!scroll`'s:
    /// `Bus::publish` skips the publisher, so the subject needs their own copy.
    fn upgrade_effect(
        &mut self,
        character_id: u32,
        map: crate::fields::FieldKey,
        result: ItemUpgradeResult,
        scroll_id: u32,
        equip_id: u32,
    ) -> Reply {
        let body = item_upgrade_effect(character_id, result, scroll_id, equip_id);
        debug_assert_eq!(body.len(), net::upgrade::ITEM_UPGRADE_EFFECT_LEN);
        let what = format!(
            "ItemUpgradeEffect: character {character_id} scrolled {equip_id} with {scroll_id}, \
             {result:?}. Broadcast to map {map}. Nothing authenticates."
        );
        self.bus().publish(
            self.subscriber,
            map,
            Reply { opcode: ITEM_UPGRADE_EFFECT, body: body.clone(), what: what.clone() },
            None,
        );
        Reply { opcode: ITEM_UPGRADE_EFFECT, body, what }
    }

    /// A refusal that is still an answer. `result = 3` is *"cannot be used here"*, and the
    /// client has its own string for it.
    ///
    /// **This is not silence and it is not a bare unlock.** `0x0236` reaches the client's own
    /// message path, so the player is told; and it is broadcast like any other, which costs
    /// nothing because the client drops a `0x0236` for a user it cannot find.
    fn upgrade_refused(
        &mut self,
        character_id: u32,
        map: crate::fields::FieldKey,
        scroll_id: u32,
        equip_id: u32,
        why: &str,
    ) -> Vec<Reply> {
        crate::server::log(&format!(
            "   scroll: REFUSED character {character_id}'s 0x0125 ({scroll_id} on {equip_id}): \
             {why}. Nothing was consumed."
        ));
        let mut reply =
            self.upgrade_effect(character_id, map, ItemUpgradeResult::CannotBeUsed, scroll_id, equip_id);
        reply.what = format!("{} REFUSED: {why}", reply.what);
        vec![reply]
    }

    /// What is in one bag slot, if anything.
    fn bag_slot_item(
        &self,
        character_id: u32,
        inv: store::InventoryType,
        slot: u16,
    ) -> Option<store::Item> {
        self.store
            .bag_items(character_id, inv)
            .ok()?
            .into_iter()
            .find(|r| r.slot == slot)
            .map(|r| r.item)
    }

    /// The equip's id, stored stats and failed-slot count, from wherever it lives.
    fn read_target(
        &self,
        character_id: u32,
        target: Target,
    ) -> Option<(u32, Option<net::opcode::EquipStats>, u8)> {
        match target {
            Target::Worn(slot) => self.store.worn_item(character_id, slot).ok().flatten(),
            Target::Bagged(slot) => {
                let item = self.bag_slot_item(character_id, store::InventoryType::Equip, slot)?;
                match item.kind {
                    store::ItemKind::Equip(stats) => {
                        Some((item.item_id, stats, item.failed_slots))
                    }
                    // A bundle in the Equip tab should be impossible; refuse rather than
                    // invent a stat block for it.
                    _ => None,
                }
            }
        }
    }

    fn write_target(
        &self,
        character_id: u32,
        target: Target,
        equip_id: u32,
        stats: &net::opcode::EquipStats,
        failed_slots: u8,
    ) -> bool {
        match target {
            Target::Worn(slot) => {
                matches!(self.store.set_worn_equip(character_id, slot, stats, failed_slots), Ok(true))
            }
            Target::Bagged(slot) => {
                let item = store::Item {
                    item_id: equip_id,
                    kind: store::ItemKind::Equip(Some(*stats)),
                    failed_slots,
                    pet_id: None,
                };
                self.store
                    .set_inventory_slot(character_id, store::InventoryType::Equip, slot, &item)
                    .is_ok()
            }
        }
    }

    fn destroy_target(&self, character_id: u32, target: Target) -> bool {
        match target {
            Target::Worn(slot) => {
                matches!(self.store.destroy_worn_equip(character_id, slot), Ok(true))
            }
            Target::Bagged(slot) => self
                .store
                .remove_item(character_id, store::InventoryType::Equip, slot, None)
                .is_ok(),
        }
    }

    /// The `0x0070` for a consumed Use-tab item's own slot: mode 1 with what is left, mode 3
    /// when the slot emptied. **The same rule the partial drop follows**, and for the same
    /// reason - a mode 3 on a stack of five would clear the slot while four are still there.
    ///
    /// Shared with `session::summonsack`, which consumes a sack the same way. It lives here
    /// because this is where it was first needed; the two callers must not grow separate
    /// copies, because a copy is where one of them quietly stops handling the stack case.
    pub(super) fn scroll_slot_reply(&self, character_id: u32, slot: u16, scroll_id: u32) -> Reply {
        let left = self
            .bag_slot_item(character_id, store::InventoryType::Use, slot)
            .filter(|i| i.item_id == scroll_id)
            .map(|i| i.kind.quantity())
            .unwrap_or(0);
        let inv = store::InventoryType::Use.as_u8() as i8;
        if left == 0 {
            Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: net::inventory::inventory_removed(inv, slot as i16),
                what: format!("InventoryOperation REMOVE: the last {scroll_id} left Use slot {slot}"),
            }
        } else {
            Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: net::inventory::inventory_quantity(inv, slot as i16, left),
                what: format!(
                    "InventoryOperation UPDATE QUANTITY: {left} x {scroll_id} left in Use slot \
                     {slot} after one was used"
                ),
            }
        }
    }

    /// The equip's template stats, through the one function that owns the bit order.
    ///
    /// Same reasoning as `session::scroll`'s `equip_base`, and deliberately the same call: a
    /// hand-built stat set here would be free to repeat the `inc_pad: t.inc_wat` bug that put
    /// `Attack Power: +200` on the owner's screen.
    fn equip_template_base(&self, item_id: u32) -> EquipBase {
        match self.config.equips.get(&item_id) {
            Some(t) => {
                let fresh = t.fresh_stats();
                EquipBase { tuc: fresh.options.remaining_enhancements, stats: fresh.stats }
            }
            None => EquipBase::default(),
        }
    }

    /// A roll for a real scroll. Shares the session counter with `!scroll` so two scrolls in
    /// one session never reuse a value.
    fn next_scroll_roll(&mut self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos =
            SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.subsec_nanos()).unwrap_or(0);
        self.scroll_roll_counter = self.scroll_roll_counter.wrapping_add(1);
        u64::from(nanos)
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(self.scroll_roll_counter.wrapping_mul(1_442_695_040_888_963_407))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The sign of `dstSlot` decides where to look**, and the owner's capture is the fixture.
    #[test]
    fn a_negative_destination_is_a_worn_slot_and_a_positive_one_is_a_bag_slot() {
        assert_eq!(Target::from_dst_slot(-5), Some(Target::Worn(5)), "the owner's topwear");
        assert_eq!(Target::from_dst_slot(-11), Some(Target::Worn(11)), "a weapon");
        assert_eq!(Target::from_dst_slot(7), Some(Target::Bagged(7)));
        // Zero names nothing - it is the drop destination in the *other* inventory packet,
        // and treating it as slot 0 would scroll whatever the 0-hole holds.
        assert_eq!(Target::from_dst_slot(0), None);
        // And the magnitude cannot overflow a u8 into a wrong slot.
        assert_eq!(Target::from_dst_slot(-300), None);
    }
}
