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
//!
//! # The backported scrolls, and the Lucky Day Scroll (2026-10-01)
//!
//! `crate::scrolls::BACKPORTED` - Pure Clean Slate, Chaos and Innocence from the modern client -
//! arrive here like any scroll and apply `!scroll`'s rules at their own tooltip rate. They are
//! not in `0204.img`'s scroll table (`gm-handbook/scrolls.txt`), so they are recognised first.
//!
//! The Lucky Day Scroll arrives on `0x0126` instead ([`net::upgrade::CLIENT_ITEM_ENHANCER`]) and
//! marks the item ([`net::opcode::ATTRIBUTE_LUCKY_DAY`]); **the next scroll on a marked item
//! succeeds whatever its rate and clears the mark** - a real scroll at 100% (so it cannot
//! destroy), a backported one guaranteed, and `!scroll`'s Scroll of Secrets too.

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
        let backported = scrolls::backported(scroll_id);
        let template = match (backported, self.config.scrolls.get(&scroll_id).copied()) {
            (Some(_), _) => None,
            (None, Some(t)) => Some(t),
            (None, None) => {
                return self.upgrade_refused(
                    character_id,
                    map,
                    scroll_id,
                    0,
                    "that item is not a scroll in this client's 0204.img",
                )
            }
        };

        // ---- the equip --------------------------------------------------------------
        let Some(target) = Target::from_dst_slot(req.dst_slot) else {
            return self.upgrade_refused(character_id, map, scroll_id, 0, "dstSlot names no slot");
        };
        let Some((equip_id, stored, failed_slots, rolled_base)) = self.read_target(character_id, target) else {
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
        // A backported scroll goes on any equip but a pet's - the client's own predicate excludes
        // `1800000..1899999` for exactly these ids (`0x14041754d`). [L]
        if backported.is_some() && equip_id / 100_000 == 18 {
            return self.upgrade_refused(
                character_id,
                map,
                scroll_id,
                equip_id,
                "that scroll cannot be used on pet equipment",
            );
        }
        if backported.is_none() && !ScrollTemplate::fits(scroll_id, equip_id) {
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

        // **A Lucky Day mark makes this one succeed**, whatever its rate - and is spent below.
        let lucky = current.options.attribute & net::opcode::ATTRIBUTE_LUCKY_DAY != 0;
        let (success_pct, cursed_pct) = match (backported, template) {
            (Some((_, pct)), _) => (pct as u16, 0),
            (None, Some(t)) => (t.success, t.cursed),
            (None, None) => (0, 0),
        };
        let roll = self.next_scroll_roll();
        let outcome = match (backported, template) {
            (Some((mode, pct)), _) => {
                // Innocence reverts a mob drop to its ROLL, as `!scroll`'s does.
                let base = match (mode, rolled_base) {
                    (scrolls::SecretsMode::Innocence, Some(rolled)) => EquipBase { tuc: base.tuc, stats: rolled },
                    _ => base,
                };
                let chance = if lucky { scrolls::Chance::Guaranteed } else { scrolls::Chance::Percent(pct) };
                scrolls::apply(mode, &base, &state, chance, roll)
            }
            (None, Some(t)) => scrolls::apply_real(
                &base,
                &state,
                if lucky { 100 } else { t.success },
                t.cursed,
                &t.increments,
                roll,
            ),
            (None, None) => Err(scrolls::Refusal::NoScrollFits),
        };
        let applied = match outcome {
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
        new_stats.options.attribute &= !net::opcode::ATTRIBUTE_LUCKY_DAY;

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
        // **The server-wide tally for the drops page** (the owner, 2026-10-03), by the scroll's
        // listed rate. A Lucky Day use is guaranteed whatever its rate, so it is left out.
        if !lucky {
            if let Err(e) = self.store.record_scroll_use(success_pct, applied.succeeded, applied.destroyed) {
                crate::server::log(&format!("   scroll: the use could not be counted for the drops page: {e}"));
            }
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
                    cursed_pct
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
            _ => self.equip_refresh(target, equip_id, new_stats, applied.after.failed_slots),
        });

        crate::server::log(&format!(
            "   scroll: character {character_id} used {scroll_id} ({}% / {}% cursed{}) on \
             {equip_id}: {result:?}, remaining {} -> {}, failed {} -> {}, changes {:?}. \
             Nothing authenticates.",
            success_pct,
            cursed_pct,
            if lucky { ", LUCKY DAY: guaranteed, mark spent" } else { "" },
            state.remaining,
            applied.after.remaining,
            state.failed_slots,
            applied.after.failed_slots,
            applied.changes,
        ));
        out
    }

    /// The `0x0070` that redraws one equip where it is - the tooltip is the only place the
    /// player sees its stats, its slots and (perhaps) its Lucky Day mark.
    fn equip_refresh(&self, target: Target, equip_id: u32, stats: net::opcode::EquipStats, failed_slots: u8) -> Reply {
        let refreshed = store::Item {
            item_id: equip_id,
            kind: store::ItemKind::Equip(Some(stats)),
            failed_slots,
            pet_id: None,
            rolled_base: None, // server-only; this copy is only drawn
        };
        let blob = self.item_blob(&refreshed);
        let pos = match target {
            Target::Worn(slot) => -i16::from(slot),
            Target::Bagged(slot) => slot as i16,
        };
        Reply {
            opcode: net::inventory::INVENTORY_OPERATION,
            body: net::inventory::inventory_added(store::InventoryType::Equip.as_u8() as i8, pos, &blob),
            what: format!(
                "InventoryOperation ADD: re-sending {equip_id} at position {pos} so the tooltip shows \
                 the new stats, {} enhancement slot(s), attribute {:#06x}",
                stats.options.remaining_enhancements, stats.options.attribute
            ),
        }
    }

    /// `0x0126` - **a Lucky Day Scroll dragged onto an equip** (the owner, 2026-10-01). Marks the
    /// item; the next scroll on it succeeds (`upgrade_with`). Answered on every path - the
    /// client latched `+0x2330` when it sent this.
    pub(super) fn on_item_enhancer(&mut self, body: &[u8]) -> Vec<Reply> {
        let (Some(req), Some(chr)) = (net::upgrade::parse_item_upgrade(body), self.claimed_character()) else {
            return crate::mesodrop::unlock_unhandled_latching_request(net::upgrade::CLIENT_ITEM_ENHANCER);
        };
        let (character_id, map) = (chr.id, self.field_of(&chr));
        let Some(scroll) = self.bag_slot_item(character_id, store::InventoryType::Use, req.src_slot) else {
            return self.upgrade_refused(character_id, map, 0, 0, "no item in that Use slot");
        };
        if scroll.item_id != scrolls::LUCKY_DAY {
            return self.upgrade_refused(
                character_id,
                map,
                scroll.item_id,
                0,
                "the only enhancer scroll this server knows is the Lucky Day Scroll (2530000)",
            );
        }
        let Some(target) = Target::from_dst_slot(req.dst_slot) else {
            return self.upgrade_refused(character_id, map, scrolls::LUCKY_DAY, 0, "dstSlot names no slot");
        };
        let Some((equip_id, stored, failed_slots, _)) = self.read_target(character_id, target) else {
            return self.upgrade_refused(character_id, map, scrolls::LUCKY_DAY, 0, "there is no equipment in that slot");
        };
        let base = self.equip_template_base(equip_id);
        let mut stats = stored.unwrap_or_else(|| net::opcode::EquipStats::fresh(base.stats, base.tuc));
        // Bits 8 and 9 together are the client's own "already has one" test (`0x1417e9a30`).
        if stats.options.attribute & (net::opcode::ATTRIBUTE_LUCKY_DAY | 1 << 8) != 0 {
            return self.upgrade_refused(
                character_id,
                map,
                scrolls::LUCKY_DAY,
                equip_id,
                "that item already has a Lucky Day or Protection Scroll on it",
            );
        }
        stats.options.attribute |= net::opcode::ATTRIBUTE_LUCKY_DAY;
        if !self.write_target(character_id, target, equip_id, &stats, failed_slots) {
            return self.upgrade_refused(
                character_id,
                map,
                scrolls::LUCKY_DAY,
                equip_id,
                "the equipment would not take the write; nothing was consumed",
            );
        }
        if self.store.remove_item(character_id, store::InventoryType::Use, req.src_slot, Some(1)).is_err() {
            crate::server::log("   lucky day: the item was marked but the scroll would not leave the bag");
        }
        crate::server::log(&format!(
            "   lucky day: character {character_id} marked {equip_id} at {target:?} - its next scroll \
             succeeds. Nothing authenticates."
        ));
        vec![
            self.upgrade_effect(character_id, map, ItemUpgradeResult::Succeeded, scrolls::LUCKY_DAY, equip_id),
            self.scroll_slot_reply(character_id, req.src_slot, scrolls::LUCKY_DAY),
            self.equip_refresh(target, equip_id, stats, failed_slots),
        ]
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

    /// The equip's id, stored stats, failed-slot count and rolled base, from wherever it lives.
    #[allow(clippy::type_complexity)]
    fn read_target(
        &self,
        character_id: u32,
        target: Target,
    ) -> Option<(u32, Option<net::opcode::EquipStats>, u8, Option<net::opcode::EquipStatSet>)> {
        match target {
            Target::Worn(slot) => self.store.worn_item(character_id, slot).ok().flatten(),
            Target::Bagged(slot) => {
                let item = self.bag_slot_item(character_id, store::InventoryType::Equip, slot)?;
                match item.kind {
                    store::ItemKind::Equip(stats) => {
                        Some((item.item_id, stats, item.failed_slots, item.rolled_base))
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
                // The rolled base is the ROW's and a scroll does not change it - carried over,
                // or a scrolled drop would lose what Innocence reverts to (`Item::rolled_base`).
                let rolled_base = self.bag_slot_item(character_id, store::InventoryType::Equip, slot).and_then(|i| i.rolled_base);
                let item = store::Item {
                    item_id: equip_id,
                    kind: store::ItemKind::Equip(Some(*stats)),
                    failed_slots,
                    pet_id: None,
                    rolled_base,
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

    use crate::config::{Config, EquipTemplate};
    use std::sync::Arc;
    use store::Store;

    const HAT: u32 = 1_002_999;
    /// A hat scroll at 10% (category 100, `ScrollTemplate::fits`), +1 DEF.
    const HAT_SCROLL_10: u32 = 2_040_002;
    const PET_HAT: u32 = 1_802_000;

    /// A claimed character wearing a 7-slot hat, with `use_items` in the Use tab.
    fn wearing_a_hat(use_items: &[(u32, u16)]) -> (Arc<Store>, Session, u32) {
        let mut config = Config::default();
        config.equips.insert(HAT, EquipTemplate { tuc: 7, inc_pdd: 10, ..EquipTemplate::default() });
        config.equips.insert(PET_HAT, EquipTemplate { tuc: 0, ..EquipTemplate::default() });
        config.scrolls.insert(
            HAT_SCROLL_10,
            ScrollTemplate { success: 10, cursed: 0, increments: net::opcode::EquipStatSet { inc_pdd: 1, ..Default::default() } },
        );
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let id = store.create_character(account, 0, &net::opcode::Character { name: "Lucky".into(), ..Default::default() }).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::new(store.clone(), Arc::new(config));
        assert!(s.claim_for_character(id).contains("claimed the migration"));
        let _ = store.unequip_to_bag(id, 1, Some(20));
        store.set_inventory_slot(id, store::InventoryType::Equip, 10, &store::Item::equip(HAT)).unwrap();
        store.equip_from_bag(id, 10, 1).unwrap();
        for &(item, n) in use_items {
            store.add_item(id, store::InventoryType::Use, &store::Item::bundle(item, n), 100).unwrap();
        }
        (store, s, id)
    }

    fn use_slot(store: &Store, id: u32, item: u32) -> u16 {
        store.bag_items(id, store::InventoryType::Use).unwrap().iter().find(|r| r.item.item_id == item).map(|r| r.slot).unwrap()
    }

    fn held(store: &Store, id: u32, item: u32) -> u16 {
        store.bag_items(id, store::InventoryType::Use).unwrap().iter().filter(|r| r.item.item_id == item).map(|r| r.item.kind.quantity()).sum()
    }

    /// `u32 tick, u16 src, u16 dstInvType, i16 dst, u8` - the body both `0x0125` and `0x0126` carry.
    fn drag(src: u16, dst: i16) -> Vec<u8> {
        let mut b = 0u32.to_le_bytes().to_vec();
        b.extend_from_slice(&src.to_le_bytes());
        b.extend_from_slice(&1u16.to_le_bytes());
        b.extend_from_slice(&dst.to_le_bytes());
        b.push(0);
        b
    }

    /// The `0x0236`'s result byte: `u32 charId`, then the result.
    fn effect_result(out: &[Reply]) -> u8 {
        let e = out.iter().find(|r| r.opcode == ITEM_UPGRADE_EFFECT).expect("a 0x0236");
        e.body[4]
    }

    fn hat_stats(store: &Store, id: u32) -> net::opcode::EquipStats {
        store.worn_item(id, 1).unwrap().unwrap().1.expect("stored stats")
    }

    /// **A Lucky Day Scroll makes the next scroll succeed** (the owner, 2026-10-01: *"automatically
    /// succeed without respecting its success percentage"*). Seven rounds of mark-then-scroll with
    /// a 10% scroll: every one succeeds (seven 10% successes in a row by chance is one in ten
    /// million), each takes one Lucky Day and one scroll, a second mark is refused for free, and
    /// the mark is spent every time.
    #[test]
    fn a_lucky_day_mark_makes_the_next_scroll_succeed_and_is_spent() {
        let (store, mut s, id) = wearing_a_hat(&[(scrolls::LUCKY_DAY, 12), (HAT_SCROLL_10, 12)]);
        for round in 1..=7u8 {
            let out = s.on_item_enhancer(&drag(use_slot(&store, id, scrolls::LUCKY_DAY), -1));
            assert_eq!(effect_result(&out), ItemUpgradeResult::Succeeded as u8, "round {round}: {out:?}");
            assert!(hat_stats(&store, id).options.attribute & net::opcode::ATTRIBUTE_LUCKY_DAY != 0, "marked");
            let again = s.on_item_enhancer(&drag(use_slot(&store, id, scrolls::LUCKY_DAY), -1));
            assert_eq!(effect_result(&again), ItemUpgradeResult::CannotBeUsed as u8, "one mark at a time");

            let out = s.on_item_upgrade(&drag(use_slot(&store, id, HAT_SCROLL_10), -1));
            assert_eq!(effect_result(&out), ItemUpgradeResult::Succeeded as u8, "round {round}: a 10% scroll, guaranteed");
            let hat = hat_stats(&store, id);
            assert_eq!(hat.stats.inc_pdd, 10 + u16::from(round));
            assert_eq!(hat.options.remaining_enhancements, 7 - round);
            assert_eq!(hat.options.attribute & net::opcode::ATTRIBUTE_LUCKY_DAY, 0, "the mark is spent");
        }
        assert_eq!(held(&store, id, scrolls::LUCKY_DAY), 5, "one Lucky Day per mark; the refused second ones took nothing");
        assert_eq!(held(&store, id, HAT_SCROLL_10), 5);
        // **None of the seven reaches the drops page's scroll tally** - guaranteed is not luck.
        assert!(store.scroll_stats().unwrap().is_empty(), "Lucky Day uses are not counted");
    }

    /// **An unmarked scroll is counted at its listed rate**, whichever way it went - the drops
    /// page's server-wide scroll luck (the owner, 2026-10-03). A refused one is not a use.
    #[test]
    fn an_ordinary_scroll_use_is_counted_for_the_drops_page() {
        let (store, mut s, id) = wearing_a_hat(&[(HAT_SCROLL_10, 3)]);
        let mut won = 0;
        for _ in 0..3 {
            let out = s.on_item_upgrade(&drag(use_slot(&store, id, HAT_SCROLL_10), -1));
            won += u64::from(effect_result(&out) == ItemUpgradeResult::Succeeded as u8);
        }
        assert_eq!(
            store.scroll_stats().unwrap(),
            vec![store::scrollstats::ScrollRate { success_pct: 10, uses: 3, successes: won, destroyed: 0 }]
        );
        let pet_hat = store.add_item(id, store::InventoryType::Equip, &store::Item::equip(PET_HAT), 1).unwrap()[0].slot;
        store.add_item(id, store::InventoryType::Use, &store::Item::bundle(HAT_SCROLL_10, 1), 100).unwrap();
        let _ = s.on_item_upgrade(&drag(use_slot(&store, id, HAT_SCROLL_10), pet_hat as i16));
        assert_eq!(store.scroll_stats().unwrap()[0].uses, 3, "a refused scroll is not a use");
    }

    /// **A Lucky Day mark means the next scroll cannot destroy the item.** The owner, 2026-10-01:
    /// *"The scroll will not cause destruction"*. A 10% scroll that destroys on EVERY failure
    /// (cursed 100), five times on a marked hat: the hat survives and gains each time. Without
    /// the mark the same scroll would take the hat nine times in ten.
    #[test]
    fn a_lucky_day_mark_protects_the_item_from_a_cursed_scroll() {
        const CURSED_10: u32 = 2_040_099;
        let (store, mut s, id) = wearing_a_hat(&[(scrolls::LUCKY_DAY, 5), (CURSED_10, 5)]);
        let mut cfg = (*s.config).clone();
        cfg.scrolls.insert(
            CURSED_10,
            ScrollTemplate { success: 10, cursed: 100, increments: net::opcode::EquipStatSet { inc_pdd: 1, ..Default::default() } },
        );
        s.config = Arc::new(cfg);
        for round in 1..=5u8 {
            let _ = s.on_item_enhancer(&drag(use_slot(&store, id, scrolls::LUCKY_DAY), -1));
            let out = s.on_item_upgrade(&drag(use_slot(&store, id, CURSED_10), -1));
            assert_eq!(effect_result(&out), ItemUpgradeResult::Succeeded as u8, "round {round}: {out:?}");
            assert_eq!(hat_stats(&store, id).stats.inc_pdd, 10 + u16::from(round), "round {round}: still worn, and gained");
        }
    }

    /// **The backported scrolls**: a Chaos (60%) on a marked hat cannot fail; a Pure Clean Slate
    /// with nothing to restore is refused and takes nothing; Innocence (2049190) on a marked hat
    /// puts it back to the template; and none of them goes on pet equipment.
    #[test]
    fn the_backported_scrolls_apply_the_scroll_rules_at_their_own_rate() {
        const CHAOS: u32 = 2_049_100;
        const CLEAN_20: u32 = 2_049_003;
        const INNOCENCE: u32 = 2_049_190;
        let (store, mut s, id) = wearing_a_hat(&[(scrolls::LUCKY_DAY, 2), (CHAOS, 1), (CLEAN_20, 1), (INNOCENCE, 1)]);
        let _ = s.on_item_enhancer(&drag(use_slot(&store, id, scrolls::LUCKY_DAY), -1));
        let out = s.on_item_upgrade(&drag(use_slot(&store, id, CHAOS), -1));
        assert_eq!(effect_result(&out), ItemUpgradeResult::Succeeded as u8, "{out:?}");
        assert_eq!(hat_stats(&store, id).options.remaining_enhancements, 6, "Chaos spends a slot");
        assert_eq!(held(&store, id, CHAOS), 0);

        let out = s.on_item_upgrade(&drag(use_slot(&store, id, CLEAN_20), -1));
        assert_eq!(effect_result(&out), ItemUpgradeResult::CannotBeUsed as u8, "no failed slot to restore");
        assert_eq!(held(&store, id, CLEAN_20), 1, "and it is still in the bag");

        let _ = s.on_item_enhancer(&drag(use_slot(&store, id, scrolls::LUCKY_DAY), -1));
        let out = s.on_item_upgrade(&drag(use_slot(&store, id, INNOCENCE), -1));
        assert_eq!(effect_result(&out), ItemUpgradeResult::Succeeded as u8);
        let hat = hat_stats(&store, id);
        assert_eq!((hat.stats.inc_pdd, hat.options.remaining_enhancements), (10, 7), "the template again");

        // A pet's hat, in the Equip tab: refused before anything is spent.
        let pet_hat = store.add_item(id, store::InventoryType::Equip, &store::Item::equip(PET_HAT), 1).unwrap()[0].slot;
        let out = s.on_item_upgrade(&drag(use_slot(&store, id, CLEAN_20), pet_hat as i16));
        assert_eq!(effect_result(&out), ItemUpgradeResult::CannotBeUsed as u8);
        assert_eq!(held(&store, id, CLEAN_20), 1);
    }
}
