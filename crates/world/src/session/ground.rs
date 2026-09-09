//! Items on the ground: putting one there, and taking it back.
//!
//! The packets are in `net::drops` and the field-side table is in `crate::drops`; this file
//! is only the two session handlers that join them to the bag.
//!
//! # What is measured and what is not
//!
//! Dropping is built out of packets read off the client: `0x0107` with **`dst == 0`** is the
//! request (the owner dragged a sword out of the window and the capture shows `01 0100 0000 0100`),
//! and `0x046E` puts the item on the floor. Picking up is **not**. The player's pick-up
//! request opcode could never be found statically - the chain runs into `.themida`, whose
//! `SizeOfRawData` is 0 - and **one walk over one drop named it: `0x032C`**, with the drop's
//! object id at body offset 13. Measured 2026-08-20.
//!
//! The handler used to **search** the body for a `u32` matching a live drop id, because the
//! layout was unknown; that is what made one run name the opcode, the field offset *and*
//! complete the pick-up instead of needing three. It reads offset 13 directly now.
//!
//! # Always answer
//!
//! Every path out of both handlers sends something. `0x0107` is the strict one: the client
//! latches `player+0x2330` when it sends, only an inbound handler clears it, and a refusal
//! that sends nothing kills every later inventory action for the rest of the session.

use super::*;

impl Session {
    /// `0x0107` with `dst == 0` - the player dragged an item out of the inventory window.
    ///
    /// Order matters on the way out: the `0x0070` **Remove** goes first and the `0x046E`
    /// second, because it is the inventory reply that clears the client's latch.
    ///
    /// **The item leaves the database before it reaches the floor**, and if the floor step
    /// could fail that would lose it. It cannot: `DropTable::drop_item` mints an id and
    /// records the drop infallibly. The reverse direction - pick-up - is the one that can
    /// fail half way, and that is why [`crate::drops::DropTable::restore`] exists.
    pub(super) fn on_drop_request(
        &mut self,
        m: &net::inventory::InventoryMove,
        chr: &net::opcode::Character,
    ) -> Vec<Reply> {
        use crate::drops::{whole_slot_is_leaving, DropFromBag, PARTIAL_STACK_REFUSAL};

        // `src < 0` with `dst == 0` is a drag straight off the character rather than out of
        // a bag slot. Never tested, and mode 3 on a negative slot makes the reply's length
        // depend on client state the server cannot see. Refuse it rather than send a packet
        // whose length is a guess.
        let Ok(slot) = u16::try_from(m.src) else {
            return self.refuse_drop(m, "dropping straight off the character has never been tested");
        };
        let Ok(inv) = store::InventoryType::from_wire(i16::from(m.inv_type)) else {
            return self.refuse_drop(m, "invType is not a bag");
        };

        // **Where it lands decides whether the test can discriminate.** The client's pick-up
        // sweep is a box of `x-0x19..x+0x19` by `y-0x32..y+0x0a` around the player, so a drop
        // more than about 25 pixels off is drawn and unreachable - which on screen is the
        // same picture as nothing having happened. Refused rather than guessed for exactly
        // that reason: a guess would make a broken run look like a working one.
        let Some((x, y)) = self.last_position else {
            return self.refuse_drop(
                m,
                "the server does not know where you are standing, and a drop it puts in the \
                 wrong place cannot be picked up. Walk a step first",
            );
        };
        // `last_position` is the last point of the movement path, which can be **mid-jump**.
        // An item left hanging in the air where the player happened to be is exactly as
        // uncollectable as one inside a wall, and it is the same 10-px pick-up box either
        // way. The fallback is the player's own position, so a player already standing on
        // the ground sees no change at all.
        let (x, y) = self.config.footholds.rest_at(chr.map_id, x, y, (x, y));

        let in_slot = self
            .store
            .bag_items(chr.id, inv)
            .ok()
            .and_then(|rows| rows.iter().find(|i| i.slot == slot).map(|i| i.item.kind.quantity()))
            .unwrap_or(0);
        if in_slot == 0 {
            return self.refuse_drop(m, "there is nothing in that slot");
        }
        // A partial stack needs `0x0070` mode 1, which is not built. Refusing keeps the whole
        // stack in the bag, which is the safe direction.
        if !whole_slot_is_leaving(net::drops::drop_count(m), in_slot) {
            return self.refuse_drop(m, PARTIAL_STACK_REFUSAL);
        }

        let item = match self.store.remove_item(chr.id, inv, slot, None) {
            Ok(i) => i,
            Err(e) => return self.refuse_drop(m, &format!("the store would not release it: {e}")),
        };
        let (map, now) = (chr.map_id, self.clock_ms);
        let placed = self.fields.with_drops(map, |d| {
            d.drop_item(DropFromBag {
                map_id: map,
                character_id: chr.id,
                inv_type: inv,
                slot,
                item,
                x,
                y,
                now_ms: now,
            })
        });
        // **The floor is shared.** The owner, 2026-09-05: *"If a player drops an item on the
        // ground, anyone in the map should be able to see it and pick it up."* The `0x046E`
        // goes to everyone else on the map through the bus; the dropper gets it directly,
        // after the `0x0070` that clears their own inventory latch. `drop_item` marked the
        // drop public, so `may_be_taken_by` lets anyone here take it.
        self.bus().publish(self.subscriber, map, placed.enter.clone(), None);
        vec![placed.removed, placed.enter]
    }

    /// `0x0143` - **the player dropped mesos**, and now it actually happens.
    ///
    /// The owner, 2026-09-09: *"I still cannot drop mesos."* Correct: `crate::mesodrop` decoded the
    /// packet and then refused it. The 2026-09-08 work fixed the *freeze* a refused drop left
    /// behind and never made the drop happen, and a patch note of mine said otherwise.
    ///
    /// # Every effect hangs off the transition, not off the request
    ///
    /// `CLAUDE.md`'s Heena-quest rule, which cost repeatable experience: the payout sat outside
    /// the match on the store's answer, so a refusal was logged and then ignored. Here the
    /// order is **deduct first, place second**, and the drop is built only from the balance the
    /// store actually returned. Every refusal below returns immediately with the unlock, so
    /// there is no path that puts coins on the floor without them having left the character.
    ///
    /// # The signed amount is a duplication bug if it is read as a `u32`
    ///
    /// `net::dropmoney::DropMoney::amount` is an `i32` **on purpose**: `142d4cb5f` sign-extends
    /// it and the client's only check is `cmp rdi, rax / jle` against the player's money, which
    /// **a negative number passes**. Read as unsigned, `-1` is four billion mesos; handed to
    /// `add_mesos` as a negative delta it would *credit* the player. It is refused here, and a
    /// test pins that.
    pub(super) fn on_drop_money(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(req) = net::dropmoney::parse_drop_money(body) else {
            return crate::mesodrop::refuse(
                &format!(
                    "an unreadable {} byte body (expected {})",
                    body.len(),
                    net::dropmoney::DROP_MONEY_BODY_LEN
                ),
                None,
            );
        };
        let Some(chr) = self.claimed_character() else {
            return crate::mesodrop::refuse("no character is claimed on this connection", None);
        };
        // Negative and zero, both refused. See the doc block: negative is the dangerous one.
        if req.amount <= 0 {
            return crate::mesodrop::refuse(
                &format!("{} is not an amount that can be dropped", req.amount),
                Some(crate::mesodrop::NOT_A_POSITIVE_AMOUNT),
            );
        }
        let want = req.amount as u32;
        let balance = match self.store.mesos(chr.id) {
            Ok(v) => v,
            Err(e) => {
                return crate::mesodrop::refuse(&format!("the store would not read mesos: {e}"), None)
            }
        };
        if want > balance {
            return crate::mesodrop::refuse(
                &format!("asked for {want} with {balance} in hand"),
                Some(crate::mesodrop::NOT_ENOUGH),
            );
        }
        // Same guard as the item drop, and for the same measured reason: the client's pick-up
        // sweep is a small box around the player, so coins put down more than ~25 px away are
        // drawn and unreachable - which on screen looks exactly like nothing having happened.
        let Some((x, y)) = self.last_position else {
            return crate::mesodrop::refuse(
                "the server does not know where you are standing",
                Some(crate::mesodrop::WALK_FIRST),
            );
        };
        let (x, y) = self.config.footholds.rest_at(chr.map_id, x, y, (x, y));

        // **The transition.** Nothing below runs unless this succeeded, and the drop is built
        // from `left`, the balance the store returned, rather than from `balance - want`.
        let left = match self.store.add_mesos(chr.id, -i64::from(want)) {
            Ok(v) => v,
            Err(e) => {
                return crate::mesodrop::refuse(
                    &format!("the store would not take the mesos: {e}"),
                    None,
                )
            }
        };
        let (map, now) = (chr.map_id, self.clock_ms);
        let placed = self.fields.with_drops(map, |d| {
            d.drop_money(crate::drops::DropMoneyOnGround {
                map_id: map,
                character_id: chr.id,
                meso: want,
                x,
                y,
                now_ms: now,
            })
        });
        crate::server::log(&format!(
            "   mesos: character {} dropped {want} at ({x}, {y}) on map {map}, {left} left, \
             drop object {}. Nothing authenticates - the amount is checked against the stored \
             balance and nothing else.",
            chr.id, placed.object_id
        ));
        // **One packet does both jobs.** `excl_request` is what clears `player+0x2330`, and
        // the same `StatChanged` carries the new balance, so the meso counter and the latch
        // cannot disagree. A bag drop needs a `0x0070` here; mesos are not a bag slot.
        let balance_reply = Reply {
            opcode: net::combat::STAT_CHANGED,
            body: net::combat::stat_changed(&net::combat::StatChange {
                excl_request: true,
                meso: Some(u64::from(left)),
                ..Default::default()
            }),
            what: format!(
                "StatChanged: {want} mesos dropped, balance now {left}. excl_request = 1 \
                 clears +0x2330; without it every later inventory action, AP click and item \
                 drop is dropped before it is built."
            ),
        };
        // The floor is shared - everyone else on the map sees it through the bus, the dropper
        // gets it directly and after the packet that clears their latch.
        self.bus().publish(self.subscriber, map, placed.enter.clone(), None);
        vec![balance_reply, placed.enter]
    }

    /// Refuse a drop: the mandatory `0x0070`, **and** a line saying why.
    ///
    /// The `0x0070` alone is enough to keep the client alive - it is what clears the
    /// `player+0x2330` latch - but it is silent on screen. An item that will not leave the
    /// bag and gives no reason is indistinguishable from a frozen inventory, which is the
    /// symptom this project has spent runs chasing before. The notice is the difference
    /// between "it refused, and here is what to do" and "nothing happened".
    fn refuse_drop(&self, m: &net::inventory::InventoryMove, why: &str) -> Vec<Reply> {
        let mut out = crate::drops::drop_refused(m, why);
        out.extend(self.notice(format!("Cannot drop that: {why}.")));
        out
    }


    /// `0x032C` - the player walked over a drop.
    ///
    /// **This handler exists to be read in a log.** Until a run names the opcode it reports
    /// what came in, whether a live drop id was found in the body, and at what byte offset.
    /// The `what` strings are the deliverable as much as the packets are.
    ///
    /// The body is searched rather than parsed. Every `u32` in it, at every byte offset, is
    /// tested against the live drop ids - unaligned included, because nothing says the field
    /// is aligned. A false positive would need the body to contain a number in the
    /// 20 000 000+ range that happens to equal a drop currently on this field, which is why
    /// the ids are minted up there in the first place.
    pub(super) fn on_pick_up(&mut self, opcode: u16, payload: &[u8]) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else {
            return self.notice(format!(
                "0x{opcode:04X} arrived with no character claimed on this connection."
            ));
        };

        let map = chr.map_id;
        // **Read the object id where it is.** It was searched for, across every byte offset,
        // because the layout was unknown; one run measured it at offset 13 and the search can
        // go. The body is `u8 0 | u32 tick | u32 0 | i16 x | i16 y | u32 dropObjectId | ...`,
        // and the builder can never be read - it lives in `.themida`, whose `SizeOfRawData`
        // is zero, so those bytes are not on disk at all. `research/pick-up-latch.md` §3.
        let found = crate::drops::pick_up_object_id(payload)
            .filter(|id| self.fields.with_drops(map, |d| d.get(*id).is_some()));

        let Some(object_id) = found else {
            // No drop by that id on this field: it expired, someone else took it, or this is
            // one of the five sibling opcodes that is not the pick-up. **Answer anyway** -
            // the sweep tests the same exclusive-request gate the inventory does, so silence
            // here risks closing every later pick-up.
            let live = self.fields.with_drops(map, |d| d.len());
            let mut out = vec![Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: net::inventory::inventory_rejected(),
                what: format!("InventoryOperation: 0x{opcode:04X} named no live drop ({live} on this field); clearing the exclusive-request gate so later pick-ups still work."),
            }];
            out.extend(self.notice(format!(
                "Nothing to pick up there ({live} on this field)."
            )));
            return out;
        };

        let now = self.clock_ms;
        // **Resolve the drop's party roster before taking it.** A party drop may be taken by
        // any *current* member (or the killer, via owner); a member who has since left is no
        // longer on this list and is refused, which is the owner's *"unless they were the killer"*.
        // The two locks are independent, so reading the drop's party id, then the party, then
        // taking, never holds one across the other.
        let party_id = self.fields.with_drops(map, |d| d.get(object_id).map(|dr| dr.party_id));
        let party_members: Vec<u32> = match party_id.filter(|&p| p != 0) {
            Some(p) => self.fields.parties().party(p).map(|party| party.members.clone()).unwrap_or_default(),
            None => Vec::new(),
        };
        let outcome = self.fields.with_drops(map, |d| d.take(object_id, chr.id, now, &party_members));
        // The log line is the deliverable. It is written to be greppable on one line,
        // because the run that produces it is read by eye.
        let mut out: Vec<Reply> = Vec::new();
        let _ = opcode;

        if let Some(drop) = outcome.taken() {
            // **Mesos are not an item and must never reach a bag.** The first real pick-up,
            // 2026-08-20, was a meso drop, and this code put `item id 0, quantity 0` through
            // `add_item`: nothing was placed, the player got nothing, and the client latched
            // and would not swing again. Whether the missing credit is what latched it is
            // being established separately - but crediting them is right regardless.
            if drop.is_meso() {
                let amount = drop.meso;
                let credited = self.store.add_mesos(chr.id, i64::from(amount));
                // **The stat change goes FIRST, and it replaces the `0x0070` an item would
                // send** - mesos are not an inventory slot. Then the leave. The order is
                // from `research/pick-up-latch.md` §4; it was the other way round here, and
                // the first real pick-up sent no stat change at all.
                match credited {
                    Ok(total) => {
                        out.push(Reply {
                            opcode: net::stats::STAT_CHANGED,
                            body: net::stats::StatChange {
                                meso: Some(u64::from(total)),
                                ..Default::default()
                            }
                            .build(),
                            what: format!(
                                "StatChanged: +{amount} mesos -> {total}. Bit 18, and the ONLY                                  way this client is ever told a meso balance - the SetField                                  stat block has no meso field at all."
                            ),
                        });
                    }
                    Err(e) => out.extend(self.notice(format!("Could not credit mesos: {e}"))),
                }
                out.push(Reply {
                    opcode: net::message::MESSAGE,
                    body: net::message::meso_gained(amount.min(i32::MAX as u32) as i32),
                    what: format!("Message: +{amount} mesos, screen message area"),
                });
                out.extend(outcome.replies());
                return out;
            }

            // NOT `slot_max.max(1)`: `info/slotMax` is absent - and therefore 0 - on 187 Etc
            // items including Garnet Ore, and reading that as one-per-slot is why they would
            // not stack. `max_stack` is the one place that rule lives.
            let max_stack = self.config.shops.max_stack(drop.item.item_id);
            let inv = drop.inv_type;
            match self.store.add_item(chr.id, inv, &drop.item, max_stack) {
                Ok(placed) => {
                    out.extend(self.inventory_added_replies(inv, &placed, "picked up"));
                    // The client composes "<item> x<n> earned." from its own string table.
                    // A zero count would make it format a string it never built, so the
                    // builder refuses one - see net::message::item_gained.
                    let picked = drop.quantity().max(1);
                    out.push(Reply {
                        opcode: net::message::MESSAGE,
                        body: net::message::item_gained(drop.item_id(), u32::from(picked)),
                        what: format!(
                            "Message: picked up {} x{picked}, screen message area",
                            drop.item_id()
                        ),
                    });
                    // Only now is the drop really gone. The leave packet goes last, after
                    // the bag write it depends on has succeeded.
                    out.extend(outcome.replies());
                }
                Err(e) => {
                    // Put it back on the floor and send NO leave. A leave for a drop that is
                    // still in the table is how an item disappears from the world entirely.
                    self.fields.with_drops(map, |d| d.restore(*drop));
                    // **And the `0x0070`, which this branch used to omit.** Every other exit
                    // from this handler sends one; this one sent a chat line alone, and a
                    // chat line does not clear `player+0x2330`. Measured 2026-08-22: the owner's
                    // equip bag filled, the sixth pick-up came back "inventory 1 is full",
                    // and in the following four minutes the client sent **zero** further
                    // 0x032C over 56 drops - it had stopped asking, for mesos too. On screen
                    // that is "I cannot pick anything up any more", which is why the report
                    // named the equip bag: the full bag is what triggered the branch, not
                    // what blocked the later pick-ups.
                    out.push(Reply {
                        opcode: net::inventory::INVENTORY_OPERATION,
                        body: net::inventory::inventory_rejected(),
                        what: format!(
                            "InventoryOperation: the bag refused the item ({e}) - nCount 0,                              bExclRequestSent 1. WITHOUT this the client never sends another                              pick-up for the rest of the session, whatever the item is."
                        ),
                    });
                    out.extend(self.notice(format!("Your bag would not take it: {e}")));
                }
            }
            return out;
        }

        // **A refusal owes a `0x0070`, even though nothing left a bag.**
        //
        // The pick-up sweep tests the same exclusive-request gate the inventory does -
        // `FUN_142cc42d0` on `[ctx+0x2330]`, checked in the sweep's own first basic block at
        // `0x14179ca24` - so a refusal that sends only a chat line risks closing every LATER
        // pick-up. `inventory_rejected()` is seven bytes whose whole job is to clear that
        // gate. `research/pick-up-latch.md` §4.
        //
        // And **never a `0x046F` for a drop that is still on the floor**: telling the client
        // to remove something it can still see is how an item disappears from the world.
        // `outcome.replies()` is empty for the three refusals, which is what makes that safe.
        out.extend(outcome.replies());
        if let Some(line) = outcome.notice() {
            out.push(Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: net::inventory::inventory_rejected(),
                what: format!(
                    "InventoryOperation: refusing the pick-up with nCount 0 - {line}. The                      bExclRequestSent byte is what keeps later pick-ups working."
                ),
            });
            out.extend(self.notice(line));
        }
        out
    }
}
