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
//! request opcode cannot be found statically - the chain runs into the Themida VM - so
//! [`Session::on_pick_up`] accepts the whole unclaimed range `0x0329..0x032E` and reports
//! which one arrived. One walk over one drop names it in `world.log`.
//!
//! The body layout of that request is unknown too, which is why the handler **searches** the
//! body for a `u32` matching a live drop id instead of reading a fixed offset. That is only
//! safe because the ids start at 20 000 000 and nothing else mints them - see
//! `crate::drops::FIRST_DROP_OBJECT_ID`. It makes one run name the opcode, the field offset
//! *and* complete the pick-up, instead of needing three.
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
        self.drops.drop_item(DropFromBag {
            map_id: chr.map_id,
            character_id: chr.id,
            inv_type: inv,
            slot,
            item,
            x,
            y,
            now_ms: self.clock_ms,
        })
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


    /// One of `0x0329..0x032E` arrived, and one of them is the pick-up request.
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

        let found = (0..payload.len().saturating_sub(3)).find_map(|at| {
            let id = u32::from_le_bytes([
                payload[at],
                payload[at + 1],
                payload[at + 2],
                payload[at + 3],
            ]);
            self.drops.get(id).map(|_| (at, id))
        });

        let Some((at, object_id)) = found else {
            // Not the pick-up, or the pick-up for a drop that is already gone. Either way,
            // answer: a silent reply to an unknown opcode is how the UI latches.
            return self.notice(format!(
                "0x{opcode:04X}: {} byte body, and none of it names a drop on this field \
                 ({} live). If an item is lying here and you just walked over it, this IS \
                 the pick-up request and the id is encoded some other way.",
                payload.len(),
                self.drops.len()
            ));
        };

        let outcome = self.drops.take(object_id, chr.id, self.clock_ms);
        // The log line is the deliverable. It is written to be greppable on one line,
        // because the run that produces it is read by eye.
        let mut out = vec![Reply {
            opcode: net::notice::CHAT_NOTICE,
            body: net::notice::chat_notice(&format!(
                "PICK-UP: 0x{opcode:04X} carried drop {object_id} at body offset {at}."
            )),
            what: format!(
                "*** THE PICK-UP REQUEST IS 0x{opcode:04X}, drop object id at body offset \
                 {at} of {} *** - {}",
                payload.len(),
                outcome.what()
            ),
        }];

        if let Some(drop) = outcome.taken() {
            // **Mesos are not an item and must never reach a bag.** The first real pick-up,
            // 2026-08-20, was a meso drop, and this code put `item id 0, quantity 0` through
            // `add_item`: nothing was placed, the player got nothing, and the client latched
            // and would not swing again. Whether the missing credit is what latched it is
            // being established separately - but crediting them is right regardless.
            if drop.is_meso() {
                let amount = drop.meso;
                let credited = self.store.add_mesos(chr.id, i64::from(amount));
                out.extend(outcome.replies());
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
                return out;
            }

            let max_stack = self
                .config
                .shops
                .item_data
                .get(&drop.item.item_id)
                .map(|d| d.slot_max.max(1))
                .unwrap_or(1);
            let inv = drop.inv_type;
            match self.store.add_item(chr.id, inv, &drop.item, max_stack) {
                Ok(placed) => {
                    out.extend(self.inventory_added_replies(inv, &placed, "picked up"));
                    // Only now is the drop really gone. The leave packet goes last, after
                    // the bag write it depends on has succeeded.
                    out.extend(outcome.replies());
                }
                Err(e) => {
                    // Put it back on the floor and send NO leave. A leave for a drop that is
                    // still in the table is how an item disappears from the world entirely.
                    self.drops.restore(*drop);
                    out.extend(self.notice(format!("Your bag would not take it: {e}")));
                }
            }
            return out;
        }

        out.extend(outcome.replies());
        if let Some(line) = outcome.notice() {
            out.extend(self.notice(line));
        }
        out
    }
}
