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
            // The player's name for it and its learned skills come from `store::pets`;
            // `active` is this session's word - 1 while the pet is summoned - and since
            // 2026-09-15 `restore_active_pet` sets it from the store at claim time, so a pet
            // that was out at the last log-out is out in this record too. session/pet.rs.
            // **Per pet, by `Item::pet_id`.** Two Huskies are two rows with two numbers, so
            // each gets its own name, vitals and active byte; a pet with no number yet reads
            // as fresh under the item's own name.
            let (name, vitals) = (self.pet_name(item.pet_id, item.item_id), self.pet_vitals(item.pet_id));
            return net::bag::pet_item_with_state(
                item.item_id,
                &name,
                cash_sn,
                u8::from(item.pet_id.is_some_and(|id| self.pet_is_active(id))),
                &vitals,
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
        // **What the character is wearing, before and after.** Any move that changes the
        // worn set - a regular equip, a cash equip, the pet's hat - has to reach the other
        // players' copy of this character, and the cheapest true test of "changed" is the
        // store itself rather than the reply's shape. The owner, 2026-09-18: *"Whenever a client
        // is changing their equipment, either a cash equipment or a regular equipment, it is
        // not being immediately reflected on other clients."* Until then only the pet-equip
        // slot was watched (2026-09-15, the Blue Top Hat); the observers saw everything else
        // at the changer's next field entry, when a fresh `0x0224` carried the new look.
        // `Session::broadcast_look_change` is that fresh sighting on demand.
        let worn = |s: &Self| -> Vec<(u8, u32)> {
            s.claimed
                .as_ref()
                .and_then(|c| s.store.equipped_items(c.character_id).ok())
                .map(|w| w.into_iter().map(|e| (e.slot, e.item_id)).collect())
                .unwrap_or_default()
        };
        let before = worn(self);
        let out = self.on_inventory_move_inner(payload);
        if worn(self) != before {
            if let Some(chr) = self.claimed_character() {
                self.broadcast_look_change(&chr);
            }
        }
        out
    }

    fn on_inventory_move_inner(&mut self, payload: &[u8]) -> Vec<Reply> {
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
        let moving = self.store.inventory_slot(chr.id, inv, src).ok().flatten();
        match self.store.move_item(chr.id, inv, src, dst, count, max_stack) {
            // **A stack dropped on a stack of the same item fills it first.** The owner,
            // 2026-09-18: *"it should try to fill the stack first (any remaining after the
            // full stack will remain at the original position), if the resulting stack is
            // already full, then it will carry out the swap slots procedure."* The store
            // already did exactly that (`Store::move_item`); the REPLY was the bug: a merge
            // was answered with the same mode-2 as a swap, and mode 2 in the client is an
            // unconditional two-way exchange (`research/equip-crash.md`), so the screen
            // swapped the two stacks while the rows had merged them - and every later drag
            // on either slot moved the wrong thing. A merge is what the client draws from
            // two mode-1 counts (or a count and a mode-3 for a source poured out), the same
            // packets Consolidate Item uses.
            Ok(store::MoveOutcome::Merged { moved, remaining, destination }) => {
                let mut out = vec![Reply {
                    opcode: net::inventory::INVENTORY_OPERATION,
                    body: net::inventory::inventory_quantity(m.inv_type, m.dst, destination),
                    what: format!(
                        "InventoryOperation: MERGE {inv:?} slot {src} onto {dst} - {moved} crossed, the destination holds {destination} now. bExclRequestSent = 1 clears the +0x2330 latch."
                    ),
                }];
                out.push(if remaining == 0 {
                    Reply {
                        opcode: net::inventory::INVENTORY_OPERATION,
                        body: net::inventory::inventory_removed(m.inv_type, m.src),
                        what: format!("InventoryOperation: MERGE - {inv:?} slot {src} poured out entirely, removed."),
                    }
                } else {
                    Reply {
                        opcode: net::inventory::INVENTORY_OPERATION,
                        body: net::inventory::inventory_quantity(m.inv_type, m.src, remaining),
                        what: format!("InventoryOperation: MERGE - {inv:?} slot {src} keeps {remaining}, the destination was full before the rest."),
                    }
                });
                out
            }
            // Part of a stack into an empty slot: the source's new count, then the new stack
            // as an ADD - the client has nothing in `dst` to count.
            Ok(store::MoveOutcome::Split { moved, remaining }) => {
                let Some(item) = moving else {
                    return self.inventory_refused(&m, "split: the source row vanished between the read and the move");
                };
                let piece = store::Item { kind: store::ItemKind::Bundle { quantity: moved }, ..item };
                let blob = self.bag_item_blob(inv, dst, &piece);
                vec![
                    Reply {
                        opcode: net::inventory::INVENTORY_OPERATION,
                        body: net::inventory::inventory_quantity(m.inv_type, m.src, remaining),
                        what: format!(
                            "InventoryOperation: SPLIT {inv:?} slot {src} - {moved} of item {} go to empty slot {dst}, {remaining} stay. bExclRequestSent = 1 clears the +0x2330 latch.",
                            item.item_id
                        ),
                    },
                    Reply {
                        opcode: net::inventory::INVENTORY_OPERATION,
                        body: net::inventory::inventory_added(m.inv_type, m.dst, &blob),
                        what: format!(
                            "InventoryOperation ADD: the split-off stack of {moved} x item {} into {inv:?} slot {dst} - {} byte blob.",
                            item.item_id,
                            blob.len()
                        ),
                    },
                ]
            }
            Ok(outcome) => self.inventory_moved(&m, format!("{inv:?} bag: {outcome:?}")),
            Err(e) => self.inventory_refused(&m, &format!("move refused: {e}")),
        }
    }


    /// **Consolidate Item.** Every stack of the same item in the tab is poured into the
    /// earlier stacks of it, up to the item's stack limit; the store does the arithmetic
    /// (`Store::consolidate_bag`) and this sends one `0x0070` per changed slot.
    ///
    /// The owner, 2026-09-18, on clicking it and seeing nothing: *"This button should make sure
    /// that all items that can be stacked without violating their max stack size should be
    /// done."* and *"Consolidate items should also additionally move all items to take the
    /// first available slots in the inventory."* Until this the request fell to the generic
    /// latch unlock and did nothing. The slides go out as bag-to-bag mode-2 moves, after the
    /// quantities and removals, in ascending destination order, so each destination is empty
    /// on the client's side when the move lands.
    ///
    /// The stack limit is `config.shops.max_stack` - the one pick-ups, shops and quest
    /// rewards use to build stacks in the first place - so this merges exactly what those
    /// would have merged, no further. A tab with nothing to merge is still answered, with
    /// the nCount-0 `0x0070`: the request latches the client.
    pub(super) fn on_gather_items(&mut self, payload: &[u8]) -> Vec<Reply> {
        self.rearrange_tab(payload, false)
    }

    /// **Sort Items.** The consolidate, then the tab in order: biggest stack first, then name
    /// A to Z (`store::plan_sort` holds the comparator). The owner, 2026-09-18: *"Sort Items
    /// should sort by quantity, then name."* The swaps go out as the same bag-to-bag mode-2
    /// move a drag onto an occupied slot is answered with, which the client draws as a swap.
    pub(super) fn on_sort_items(&mut self, payload: &[u8]) -> Vec<Reply> {
        self.rearrange_tab(payload, true)
    }

    /// Consolidate Item and Sort Items share everything but the plan.
    fn rearrange_tab(&mut self, payload: &[u8], sort: bool) -> Vec<Reply> {
        let verb = if sort { "sort" } else { "consolidate" };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let Some(tab) = net::inventory::parse_gather_request(payload) else {
            return self.gather_answered_empty(verb, 0, "short body");
        };
        let inv = match store::InventoryType::from_wire(i16::from(tab)) {
            Ok(inv) => inv,
            Err(_) => return self.gather_answered_empty(verb, tab, "not an inventory tab"),
        };
        let shops = self.config.shops.clone();
        let cap = move |item_id: u32| shops.max_stack(item_id);
        let names = self.config.item_names.clone();
        let name_of = move |item_id: u32| names.get(&item_id).cloned().unwrap_or_default();
        let changes = if sort {
            self.store.sort_bag(chr.id, inv, &cap, &name_of)
        } else {
            self.store.consolidate_bag(chr.id, inv, &cap)
        };
        let changes = match changes {
            Ok(c) => c,
            Err(e) => return self.gather_answered_empty(verb, tab, &format!("store refused: {e}")),
        };
        if changes.is_empty() {
            return self.gather_answered_empty(verb, tab, "nothing to do");
        }
        crate::server::log(&format!(
            "   {verb}: character {} tab {inv:?} - {} change(s): {changes:?}",
            chr.id,
            changes.len()
        ));
        let inv_type = inv.as_u8() as i8;
        changes
            .iter()
            .map(|c| match *c {
                store::StackChange::Quantity { slot, quantity } => Reply {
                    opcode: net::inventory::INVENTORY_OPERATION,
                    body: net::inventory::inventory_quantity(inv_type, slot as i16, quantity),
                    what: format!(
                        "InventoryOperation: consolidate - {inv:?} slot {slot} now holds {quantity}. bExclRequestSent = 1 clears the +0x2330 latch the 0x0105 set."
                    ),
                },
                store::StackChange::Emptied { slot } => Reply {
                    opcode: net::inventory::INVENTORY_OPERATION,
                    body: net::inventory::inventory_removed(inv_type, slot as i16),
                    what: format!(
                        "InventoryOperation: consolidate - {inv:?} slot {slot} poured out entirely, removed. bExclRequestSent = 1 clears the +0x2330 latch the 0x0105 set."
                    ),
                },
                store::StackChange::Moved { from, to } => Reply {
                    opcode: net::inventory::INVENTORY_OPERATION,
                    body: net::inventory::inventory_move_result(inv_type, from as i16, to as i16),
                    what: format!(
                        "InventoryOperation: {verb} - {inv:?} slot {from} slides up to {to}, the first free slot. Bag-to-bag, no avatar tail."
                    ),
                },
                store::StackChange::Swapped { a, b } => Reply {
                    opcode: net::inventory::INVENTORY_OPERATION,
                    body: net::inventory::inventory_move_result(inv_type, a as i16, b as i16),
                    what: format!(
                        "InventoryOperation: {verb} - {inv:?} slots {a} and {b} change places (a mode-2 onto an occupied slot, which the client draws as a swap). Bag-to-bag, no avatar tail."
                    ),
                },
            })
            .collect()
    }

    /// The consolidate or sort that changed nothing - still a reply, because both latch.
    fn gather_answered_empty(&self, verb: &str, tab: u8, why: &str) -> Vec<Reply> {
        vec![Reply {
            opcode: net::inventory::INVENTORY_OPERATION,
            body: net::inventory::inventory_rejected(),
            what: format!(
                "InventoryOperation: {verb} tab {tab} - {why}; nCount 0, nothing moves, bExclRequestSent = 1 clears the +0x2330 latch the request set."
            ),
        }]
    }

    /// How many of the item in `slot` fit in one stack - `crate::shops::ShopTable::max_stack`,
    /// the one number every stack builder uses (pick-ups, shops, quest rewards, Consolidate).
    ///
    /// It used to read `info/slotMax` directly and call an absent one "does not stack",
    /// which was the safe direction for a lone drag; but pick-ups build 100-stacks of those
    /// same items (`slotMax` is absent on 187 Etc and 285 Use items), so a drag of two such
    /// stacks onto each other swapped where every other path merged. One rule now. An equip
    /// or a pet never stacks whatever the rule says (`Store::move_item` checks the kind).
    pub(super) fn max_stack(&self, chr: &net::opcode::Character, inv: store::InventoryType, slot: u16) -> u16 {
        let Ok(items) = self.store.bag_items(chr.id, inv) else { return 1 };
        let Some(row) = items.iter().find(|i| i.slot == slot) else { return 1 };
        if row.item.pet_id.is_some() {
            return 1;
        }
        self.config.shops.max_stack(row.item.item_id)
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
