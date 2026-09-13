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
        self.item_blob_with_cash_sn(item, None)
    }

    /// [`Session::item_blob`] with the item's own cash serial set - **only** for a body the
    /// client reads by itself. The cash shop's `0x19` erases that serial from its locker
    /// map; without it the moved item stays drawn in the Cash Inventory (2026-09-11 03:47).
    /// A bundle grows by eight bytes with it, so it must never reach the character record
    /// or a bag list.
    pub(super) fn item_blob_with_cash_sn(
        &self,
        item: &store::Item,
        cash_sn: Option<std::num::NonZeroU64>,
    ) -> Vec<u8> {
        // **A pet is item type 3**, whatever the store calls it (a bundle of one in the Cash
        // tab). The owner, 2026-09-13: the eleven pets, permanent, never revived. The body is
        // `net::bag::pet_item_with_cash_sn`; its name is the item's until the player renames it.
        if net::inventory::is_pet(item.item_id) {
            let name = self.config.item_names.get(&item.item_id).cloned().unwrap_or_default();
            // `active` is this session's word: 1 while the pet is summoned, 0 otherwise -
            // and 0 again on the next login, which is what puts it away. session/pet.rs.
            return net::bag::pet_item_with_state(
                item.item_id,
                &name,
                cash_sn,
                u8::from(self.pet_is_active(item.item_id)),
            );
        }
        match item.kind {
            store::ItemKind::Equip(stored) => net::opcode::equipped_item_with_cash_sn(
                item.item_id,
                &stored.unwrap_or_else(|| self.template_stats(item.item_id)),
                cash_sn,
            ),
            store::ItemKind::Bundle { quantity } => net::bag::bundle_item_with_cash_sn(
                item.item_id,
                quantity,
                0,
                &[0u8; net::bag::BUNDLE_OWNER_LEN],
                cash_sn,
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


    /// One `0x0070` Add per slot an arrival touched.
    ///
    /// A stack that overflows into a second slot is two entries: the client draws what it
    /// is told rather than working it out. `why` goes in the log line, so a run says whether
    /// an item arrived from a purchase, from `!item`, or off the floor - which matters most
    /// for the floor, where reading the log IS the experiment.
    pub(super) fn inventory_added_replies(
        &self,
        inv: store::InventoryType,
        changed: &[store::InvItem],
        why: &str,
    ) -> Vec<Reply> {
        changed
            .iter()
            .map(|row| {
                let blob = self.bag_item_blob(inv, row.slot, &row.item);
                Reply {
                    opcode: net::inventory::INVENTORY_OPERATION,
                    body: net::inventory::inventory_added(
                        inv.as_u8() as i8,
                        row.slot as i16,
                        &blob,
                    ),
                    what: format!(
                        "InventoryOperation ADD: item {} into {inv:?} slot {} - {} byte blob. {why}.",
                        row.item.item_id,
                        row.slot,
                        blob.len()
                    ),
                }
            })
            .collect()
    }


    /// Field-entry restore only: `0x0070` **mode 5**, which stores the item without the
    /// quest re-check that pops a collection tooltip on every map change.
    ///
    /// The owner, 2026-08-30: *"whenever I change the map, I see the popup for my collection quest
    /// as a tooltip every time ... This should only pop up when the amount in my inventory
    /// changes."*
    ///
    /// # Why the restore cannot simply be skipped
    ///
    /// `142d9b6fd cmp r15d, r14d / je` skips the whole hint block when the count before the
    /// packet equals the count after. The hint fires, so **before is 0** - the client's Etc
    /// bag is genuinely EMPTY when the restore arrives, on every SetField. "Send the bag only
    /// on the first field entry" would therefore leave the tab blank after every map change.
    /// That alternative is dead, and it died without spending a client run. **[L]**
    ///
    /// # Why mode 5 rather than a flag
    ///
    /// There is no suppressing flag. The unnamed header byte is read in exactly one place in
    /// the 11 648-byte handler, gating a single fixed UI message id. What exists instead is
    /// the mode: the jump table at `0x142d546bc` puts mode 5 at
    /// `0x142d531e4..0x142d532c7`, and that case reads the same item blob and calls
    /// `FUN_1402e4c20` - mode 0's exact store - and nothing else. `FUN_142d9b200`, the quest
    /// hook that draws the hint, has six call sites and **none of them is inside that
    /// range**. The unconditional refresh after the entry loop still runs. **[L]**
    ///
    /// # Nothing has ever sent a mode 5 on this wire
    ///
    /// So the failure to watch for is **the Etc tab looking empty**, which is worse than the
    /// tooltip. That is why this is wired for Etc alone at first: Use, Set Up and Cash stay
    /// on mode 0 and are the control inside the same run.
    pub(super) fn inventory_restored_replies(
        &self,
        inv: store::InventoryType,
        changed: &[store::InvItem],
        why: &str,
    ) -> Vec<Reply> {
        changed
            .iter()
            .map(|row| {
                let blob = self.bag_item_blob(inv, row.slot, &row.item);
                Reply {
                    opcode: net::inventory::INVENTORY_OPERATION,
                    body: net::inventory::inventory_set_quiet(
                        inv.as_u8() as i8,
                        row.slot as i16,
                        &blob,
                    ),
                    what: format!(
                        "InventoryOperation mode 5 SET (quiet): item {} into {inv:?} slot {} - {} byte blob. {why}. NOBODY HAS SENT A MODE 5 ON THIS WIRE - if the tab is empty on screen, this is why.",
                        row.item.item_id,
                        row.slot,
                        blob.len()
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

        // **The tab the item lives in.** Equip for ordinary gear; Deco for cash equips, whose
        // worn slots are 101 and up (`-101`.. on the wire). The owner, 2026-09-12: *"none of the
        // outfit items work"* - the client sent every one from the Deco tab and this handler
        // knew only the Equip tab, so they fell through to the bag-to-bag branch and were
        // refused as "needs two positive slots". The client's own rule already names both
        // tabs (`net::inventory::move_changes_the_avatar`).
        let tab = if m.inv_type == net::inventory::INV_DECO {
            store::InventoryType::Deco
        } else {
            store::InventoryType::Equip
        };

        if let Some(equip_slot) = m.equipped_slot() {
            let dst = u16::try_from(m.dst).ok();
            return match self.store.unequip_to_tab(chr.id, equip_slot, tab, dst) {
                Ok(row) => self.inventory_moved(
                    &m,
                    format!(
                        "unequipped slot {equip_slot} into {tab:?} bag slot {} - item {}, STORED, so the next SetField will not re-dress it",
                        row.slot, row.item.item_id
                    ),
                ),
                Err(e) => self.inventory_refused(&m, &format!("unequip refused: {e}")),
            };
        }

        // An equip: out of a bag slot, into a negative worn slot - on either tab.
        if m.is_equip() {
            let Ok(worn) = u8::try_from(-i32::from(m.dst)) else {
                return self.inventory_refused(&m, "the worn slot does not fit in a u8");
            };
            let Ok(src) = u16::try_from(m.src) else {
                return self.inventory_refused(&m, "the bag slot does not fit in a u16");
            };
            // **Equipping over a worn item swaps, and the client does the swap itself.**
            //
            // The owner, 2026-08-21: *"Equipping another equipment (while having a current
            // equipment in the same slot) should swap the current equipment with the one
            // being replaced."* This used to refuse, because `Store::equip_from_bag`
            // returned `SlotOccupied`.
            //
            // Mode 2 in `FUN_142d51930` is an **unconditional two-way exchange** - there is
            // no `newPos < 0` arm and no `oldPos < 0` arm, it is one path: **[L]**
            //
            //   142d52566  GetItem(invType, newPos) -> DEST
            //   142d5257b  GetItem(invType, oldPos) -> SRC
            //   142d52c13  SetItem(invType, oldPos, DEST)   <- the displaced item, into the bag
            //   142d52c65  SetItem(invType, newPos, SRC)
            //
            // So the reply is unchanged: **one entry, the same 14 bytes**, and a second entry
            // would be wrong rather than merely redundant - it would move the item that had
            // just arrived. `research/equip-crash.md`. The store mirrors `142d52c13` exactly:
            // the displaced item goes into `src`, in the same transaction.
            // **An overall and a bottom cannot be worn together.** The owner, 2026-08-29: *"when I
            // wore an overall item, the server did not take off the bottoms that I was
            // wearing... unless the player does not have sufficient inventory space."*
            //
            // Slot 5's ordinary swap already removes a worn TOP, because a top and an overall
            // share that slot. The bottom is slot 6 and nothing displaces it, which is what
            // The owner saw. `net::overall` holds the rule and the item-id ranges it reads.
            //
            // **Done BEFORE the equip, and a failure aborts the whole thing.** The alternative
            // - equip first, then try to free the other slot - can leave the character wearing
            // an illegal pair when the bag is full, and a half-applied equip is exactly the
            // state `CLAUDE.md` records as the most expensive kind of bug here.
            let mut freed: Option<String> = None;
            // The id comes out of the BAG ROW the server owns, never from the slot the client
            // asked for: trusting that would be trusting the client to say what it is wearing,
            // and nothing on this socket is authenticated.
            let incoming = self
                .store
                .bag(chr.id)
                .ok()
                .and_then(|b| {
                    b.items
                        .iter()
                        .find(|i| i.inv_type == tab && i.slot == src)
                        .map(|i| i.item.item_id)
                })
                .unwrap_or(0);
            // **A female character cannot wear a male top, and the client agrees.**
            //
            // The owner, 2026-09-09: *"Cobalt does not seem to be wearing a top, but does have a
            // top equipped when logging into the game."* The server was sending it correctly -
            // `avatar_look` puts slot 5 / `1040021` on the wire and the client has the art -
            // and the client still refused to draw it, because `1040021` is a MALE top on a
            // female character. `net::equipgender` has the derivation from the client's own
            // `MakeCharInfo.img`.
            //
            // The client was right; the bug was that this let their put it on. Checked here
            // rather than in the store, because the store has the item and the slot but not
            // the character's gender, and passing it in would put a rendering rule into the
            // layer that only knows about rows.
            if !net::equipgender::may_wear(incoming, chr.gender) {
                return self.inventory_refused(
                    &m,
                    &format!(
                        "item {incoming} is not made for this character's gender, and the                          client will not draw it even if the server allows it - which is how                          a character ends up looking undressed on the character-select screen"
                    ),
                );
            }
            // On the Deco tab the same pair is 105/106: a cash overall over a cash bottom.
            let slot_base: u8 = if tab == store::InventoryType::Deco { 100 } else { 0 };
            if let Some(other) = net::overall::conflicting_slot(incoming).map(|o| o + slot_base) {
                if let Some(worn_there) = self
                    .store
                    .equipped_items(chr.id)
                    .unwrap_or_default()
                    .into_iter()
                    .find(|e| e.slot == other)
                {
                    if net::overall::conflicts_with(incoming, worn_there.item_id, other - slot_base) {
                        // `None` picks the lowest free bag slot, and returns
                        // `StoreError::BagFull` when there is not one - which is exactly the
                        // "unless the player does not have sufficient inventory space" case.
                        match self.store.unequip_to_tab(chr.id, other, tab, None) {
                            Ok(moved) => {
                                freed = Some(format!(
                                    "took off item {} from slot {other} into bag slot {} first \
                                     - an overall and a bottom cannot be worn together",
                                    moved.item.item_id, moved.slot
                                ));
                            }
                            Err(e) => {
                                return self.inventory_refused(
                                    &m,
                                    &format!(
                                        "an overall cannot be worn over a bottom, and the bottom \
                                         could not be taken off: {e}. Free an Equip slot first - \
                                         nothing was changed."
                                    ),
                                );
                            }
                        }
                    }
                }
            }

            return match self.store.equip_from_tab(chr.id, tab, src, worn) {
                Ok(done) => self.inventory_moved(
                    &m,
                    format!(
                        "{}{}",
                        match done.displaced {
                            Some(off) => format!(
                                "equipped item {} from {tab:?} bag slot {src} into slot {worn}, SWAPPING item {off} back into bag slot {src}",
                                done.equipped
                            ),
                            None => format!(
                                "equipped item {} from {tab:?} bag slot {src} into slot {worn}{}",
                                done.equipped,
                                if worn > 100 { " - a CASH equip; the record cannot carry worn slots above 31 yet, so it will not draw on the character or survive a relog as worn" } else { "" }
                            ),
                        },
                        match &freed {
                            Some(note) => format!(" ({note})"),
                            None => String::new(),
                        }
                    ),
                ),
                Err(e) => self.inventory_refused(&m, &format!("equip refused: {e}")),
            };
        }

        // **`dst == 0` is a drop**, measured on 2026-08-20: the owner dragged a sword out of the
        // inventory window and the client sent `01 0100 0000 0100` - invType 1, src 1,
        // dst 0, count 1. It is not a move to slot zero; slots are 1-based and slot 0 is the
        // hole that makes them so.
        //
        // Wired 2026-08-20. Every early return below is still a `0x0070`, so no path here is
        // silent - see this module's header for what silence costs.
        if m.dst == 0 {
            return self.on_drop_request(&m, &chr);
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
