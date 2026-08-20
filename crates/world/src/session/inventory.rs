//! The bag: moving an item, taking one off, putting one on, and what the character wears.
//!
//! The rule that shapes all of it: **every `0x0107` is answered with a `0x0070`, including
//! the refusals**. The client latches `player+0x2330` when it sends, only an inbound
//! handler clears it, and a refusal that sends nothing kills every later inventory action
//! for the rest of the session. That is not a theory; it happened on 2026-08-19.

use super::*;

impl Session {
    /// One item as the client's own factory reads it: a leading type byte and the body.
    ///
    /// **There is exactly one of these, on purpose.** The same match existed twice - once
    /// for `!item` and once for a shop purchase - and the two copies had already drifted in
    /// how they named the bundle's quantity binding. A third copy is how one of them would
    /// have kept `hasCashSN` at zero while the other did not, and that byte changes the
    /// body's length.
    ///
    /// A `None` in the stored stats means "derive from the `Character.wz` template", which
    /// is what every row written before those columns existed says. [`Session::template_stats`]
    /// is the single place that resolves it, so a worn item and a bagged one cannot disagree
    /// about what the same item id is worth.
    pub(super) fn item_blob(&self, item: &store::Item) -> Vec<u8> {
        match item.kind {
            store::ItemKind::Equip(stored) => net::opcode::equipped_item(
                item.item_id,
                &stored.unwrap_or_else(|| self.template_stats(item.item_id)),
            ),
            store::ItemKind::Bundle { quantity } => net::bag::bundle_item(
                item.item_id,
                quantity,
                0,
                &[0u8; net::bag::BUNDLE_OWNER_LEN],
            ),
        }
    }


    /// Each worn item with the stats its `Character.wz` template gives it.
    ///
    /// **The stats belong to the item template, not to the character**, so they are resolved
    /// here rather than persisted: `crates/store` keeps `(slot, itemId)` and nothing else,
    /// and a stat column in the database would be a second source of truth for a value the
    /// client already has its own copy of.
    ///
    /// An item with no template row goes out bare. That is the behaviour confirmed on screen
    /// on 2026-08-19 - the character was dressed, the items simply had no stats - so a
    /// missing or stale `gm-handbook/equips.txt` degrades to something known to work rather
    /// than to something untested.
    pub(super) fn dressed(&self, chr: &net::opcode::Character) -> Vec<(u8, u32, net::opcode::EquipStats)> {
        chr.equips
            .iter()
            .map(|&(slot, item_id)| (slot, item_id, self.template_stats(item_id)))
            .collect()
    }


    /// One `0x0070` Add per slot a purchase touched.
    pub(super) fn inventory_added_replies(
        &self,
        inv: store::InventoryType,
        changed: &[store::InvItem],
    ) -> Vec<Reply> {
        changed
            .iter()
            .map(|row| {
                let blob = self.item_blob(&row.item);
                Reply {
                    opcode: net::inventory::INVENTORY_OPERATION,
                    body: net::inventory::inventory_added(
                        inv.as_u8() as i8,
                        row.slot as i16,
                        &blob,
                    ),
                    what: format!(
                        "InventoryOperation ADD: item {} into {inv:?} slot {}",
                        row.item.item_id, row.slot
                    ),
                }
            })
            .collect()
    }


    /// Move an item, and **write it down**.
    ///
    /// **Answering this is not optional.** `FUN_142cc5b00` sets `player->[0x2330]` to 1 the
    /// moment it sends `0x0107`, and its own gate 2 at `142cc5b5d` refuses every later
    /// request while that latch is set. Only an inbound handler clears it, and for this
    /// packet that means the `bExclRequestSent` byte of our `0x0070`. So an unanswered
    /// `0x0107` does not fail one drag - it silently kills every inventory action for the
    /// rest of the session. Same class as Log Out and `world->[0x33f4]`.
    ///
    /// **Every path here answers, including every refusal.** A refusal is
    /// [`net::inventory::inventory_rejected`], which moves nothing and still clears the
    /// latch; answering with a chat notice and no `0x0070` is what killed the whole
    /// inventory UI on 2026-08-19.
    ///
    /// # This is goal I
    ///
    /// Until 2026-08-19 the unequip moved an item on screen and nowhere else: the record's
    /// equipped list is built from the `equipment` rows, so the next `SetField` - a portal, a
    /// `!map`, a relog - put the item straight back on. The owner: *"items taken off should
    /// persist as is during transitions from map to map."*
    ///
    /// The store now has somewhere to put it. `Store::unequip_to_bag` deletes the
    /// `equipment` row and inserts the `inventory` row **in one transaction**, so there is no
    /// instant in which the item is in both places or neither - which was the whole reason
    /// the old code refused to touch the database at all. Per-item stats travel with it, so
    /// a scrolled item does not come back flattened.
    pub(super) fn on_inventory_move(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(m) = net::inventory::parse_inventory_move(payload) else {
            return Vec::new();
        };
        let Some(chr) = self.claimed_character() else {
            return self.inventory_refused(&m, "no character is claimed on this connection");
        };

        if let Some(equip_slot) = m.equipped_slot() {
            let dst = u16::try_from(m.dst).ok();
            return match self.store.unequip_to_bag(chr.id, equip_slot, dst) {
                Ok(row) => self.inventory_moved(
                    &m,
                    format!(
                        "unequipped slot {equip_slot} into Equip bag slot {} - item {}, STORED, so the next SetField will not re-dress it",
                        row.slot, row.item.item_id
                    ),
                ),
                Err(e) => self.inventory_refused(&m, &format!("unequip refused: {e}")),
            };
        }

        // An equip: out of a bag slot, into a negative worn slot.
        if m.inv_type == net::inventory::INV_EQUIP && m.src > 0 && m.dst < 0 {
            let Ok(worn) = u8::try_from(-i32::from(m.dst)) else {
                return self.inventory_refused(&m, "the worn slot does not fit in a u8");
            };
            let Ok(src) = u16::try_from(m.src) else {
                return self.inventory_refused(&m, "the bag slot does not fit in a u16");
            };
            return match self.store.equip_from_bag(chr.id, src, worn) {
                Ok(item_id) => self.inventory_moved(
                    &m,
                    format!("equipped item {item_id} from Equip bag slot {src} into slot {worn}"),
                ),
                Err(e) => self.inventory_refused(&m, &format!("equip refused: {e}")),
            };
        }

        // **`dst == 0` is a drop**, measured on 2026-08-20: the owner dragged a sword out of the
        // inventory window and the client sent `01 0100 0000 0100` - invType 1, src 1,
        // dst 0, count 1. It is not a move to slot zero; slots are 1-based and slot 0 is the
        // hole that makes them so.
        //
        // Refusing is deliberate and temporary. The item stays in the bag, which is the safe
        // direction: `Store::remove_item` would take it out and there is nowhere to put it -
        // no drop pool, no `DropEnterField`, no pickup. Losing an item is worse than one
        // that will not leave. `crates/net/src/drops.rs` is where that lands.
        if m.dst == 0 {
            let mut out = self.inventory_refused(
                &m,
                "dst 0 is a DROP, and dropping is not built yet - the item is still in your bag",
            );
            out.extend(self.notice(
                "Dropping is not built yet - the item is still in your bag.".to_string(),
            ));
            return out;
        }

        // Bag to bag. The client sends -1 for `count` when the item is not a bundle.
        let Ok(inv) = store::InventoryType::from_wire(i16::from(m.inv_type)) else {
            return self.inventory_refused(&m, &format!("invType {} is not a bag", m.inv_type));
        };
        let (Ok(src), Ok(dst)) = (u16::try_from(m.src), u16::try_from(m.dst)) else {
            return self.inventory_refused(&m, "a bag-to-bag move needs two positive slots");
        };
        let count = (m.count >= 0).then_some(m.count as u16);
        let max_stack = self.max_stack(&chr, inv, src);
        match self.store.move_item(chr.id, inv, src, dst, count, max_stack) {
            Ok(outcome) => self.inventory_moved(&m, format!("{inv:?} bag: {outcome:?}")),
            Err(e) => self.inventory_refused(&m, &format!("move refused: {e}")),
        }
    }


    /// How many of the item in `slot` fit in one stack, from `info/slotMax`.
    ///
    /// **`0` and `1` both mean "does not stack"**, and `0` is what every equip has because
    /// the property is simply absent - 290 of 2785 items carry one. An unknown item also
    /// lands here, and treating it as non-stacking is the safe direction: the worst case is
    /// a merge that does not happen, against a merge that silently destroys the overflow.
    pub(super) fn max_stack(&self, chr: &net::opcode::Character, inv: store::InventoryType, slot: u16) -> u16 {
        let Ok(items) = self.store.bag_items(chr.id, inv) else { return 1 };
        let Some(row) = items.iter().find(|i| i.slot == slot) else { return 1 };
        self.config
            .shops
            .item_data
            .get(&row.item.item_id)
            .map(|d| d.slot_max.max(1))
            .unwrap_or(1)
    }


    /// The `0x0070` that says a move happened.
    pub(super) fn inventory_moved(&self, m: &net::inventory::InventoryMove, why: String) -> Vec<Reply> {
        vec![Reply {
            opcode: net::inventory::INVENTORY_OPERATION,
            body: net::inventory::inventory_move_result(m.inv_type, m.src, m.dst),
            what: format!(
                "InventoryOperation: move invType {} slot {} -> {}. {why}. The first byte is 1, which clears the client's +0x2330 request latch; a 0 there would block every later inventory action.",
                m.inv_type, m.src, m.dst
            ),
        }]
    }


    /// The `0x0070` that says nothing happened - and it is **still a reply**.
    ///
    /// `nCount` is 0, so the client skips the entry loop entirely and moves no item, but
    /// `bExclRequestSent` is 1 and that is what unlocks the UI. A refusal that sends nothing
    /// is not a refusal; it is a dead inventory for the rest of the session.
    pub(super) fn inventory_refused(&self, m: &net::inventory::InventoryMove, why: &str) -> Vec<Reply> {
        vec![Reply {
            opcode: net::inventory::INVENTORY_OPERATION,
            body: net::inventory::inventory_rejected(),
            what: format!(
                "InventoryOperation: REFUSING invType {} slot {} -> {} with nCount 0 - {why}. Nothing moves, but bExclRequestSent = 1 clears the +0x2330 latch.",
                m.inv_type, m.src, m.dst
            ),
        }]
    }
}
