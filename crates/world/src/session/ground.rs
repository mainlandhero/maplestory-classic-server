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
        use crate::drops::{whole_slot_is_leaving, DropFromBag};

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
        // **`(x, y)` is where the player IS; `rest` is where the drop comes to land.** Both
        // are needed: the snap keeps the item inside the client's pick-up box, and the
        // un-snapped point is what the coins fall FROM. See `crate::drops::arc_from`.
        let (rest_x, rest_y) = self.config.footholds.rest_at(chr.map_id, x, y, (x, y));

        let in_slot = self
            .store
            .bag_items(chr.id, inv)
            .ok()
            .and_then(|rows| rows.iter().find(|i| i.slot == slot).map(|i| i.item.kind.quantity()))
            .unwrap_or(0);
        if in_slot == 0 {
            return self.refuse_drop(m, "there is nothing in that slot");
        }
        // **Dropping part of a stack.** The owner, 2026-09-09: *"I see that partial drop is not
        // implemented, I also need this implemented please."*
        //
        // This used to refuse, and the refusal's own words were the reason: *"needs a 0x0070
        // mode 1 UpdateQuantity, which net::inventory does not build"*. `inventory_quantity`
        // HAS been built since - `net/src/inventory.rs`, mode read at `142d521fe`, tagged
        // [L] - and the guard was never revisited. `CLAUDE.md`'s "built is not wired", found
        // by reading the refusal rather than the module it named.
        //
        // `drop_count` turns the `-1` a non-bundle carries into 1, so `requested` is always
        // at least one. Asking for more than is there takes what is there rather than
        // refusing: the client draws the number and a stack can shrink between the drag
        // starting and the packet arriving.
        let requested = net::drops::drop_count(m).min(in_slot);
        let whole = whole_slot_is_leaving(requested, in_slot);
        let remaining_in_slot = if whole { None } else { Some(in_slot - requested) };

        // `None` takes the slot, `Some(n)` takes n. **Both go through the store first**, so a
        // store that refuses leaves nothing on the floor - the same order the meso drop uses.
        let take = if whole { None } else { Some(requested) };
        let item = match self.store.remove_item(chr.id, inv, slot, take) {
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
                remaining_in_slot,
                x: rest_x,
                y: rest_y,
                from_x: x,
                from_y: y,
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
        // **The client's own cap, enforced here too.** The owner, 2026-09-09: *"The client restricts
        // dropping of mesos to 10k, we should also mimic that on the server side."* Its dialog
        // says "You may only enter a number equal to or lower than 10000".
        //
        // That box is a UI rule; this is a packet. The amount is an `i32` the client chose, and
        // the field has already fooled this server once - see the doc block above on the sign.
        // A limit enforced only in a dialog is enforced only against people using the dialog.
        if want > crate::mesodrop::MAX_DROP {
            return crate::mesodrop::refuse(
                &format!("asked to drop {want}, over the {} cap", crate::mesodrop::MAX_DROP),
                Some(crate::mesodrop::TOO_MUCH_AT_ONCE),
            );
        }
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
        let (rest_x, rest_y) = self.config.footholds.rest_at(chr.map_id, x, y, (x, y));

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
                x: rest_x,
                y: rest_y,
                from_x: x,
                from_y: y,
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


    /// **A picked-up (or expired-on-click) drop leaves EVERY screen it was on, not just the
    /// picker's.**
    ///
    /// The owner, 2026-09-14: *"when one person picks up the drops, all other people that see the
    /// drops also see it being picked up by that person. There should be no duplicates of
    /// drops, multiple people cannot pick up the same drop."*
    ///
    /// The no-duplicate half is already true: [`crate::drops::DropTable::take`] removes the
    /// drop from the shared table *before* it returns `Taken`, under the field lock, so a
    /// second player's pick-up of the same id gets `Unknown` and is refused. This is the other
    /// half - the visual one. The leave packet (`0x046F` leaveType 2) carries the picker's
    /// character id, which is what every client uses to animate the drop flying into that
    /// player, so the SAME bytes are what the whole field needs; the drop's own `take` doc
    /// says as much ("Broadcast this to the field INCLUDING the picker").
    ///
    /// The picker's copy goes back to the caller in `out`; this returns those same replies for
    /// the caller to push, and publishes each to the rest of the field over the bus (which
    /// excludes the publisher, so the picker is not told twice). A member who never saw the
    /// drop - a solo drop belongs to one owner - holds no such object id and its client
    /// ignores the leave, exactly as it ignores a movement packet for an id its pool never
    /// had. So publishing to the whole field is safe and needs no per-drop audience list.
    fn take_leaves_to_field(&self, map: u32, outcome: &crate::drops::PickUp) -> Vec<Reply> {
        let leaves = outcome.replies();
        for leave in &leaves {
            self.bus().publish(self.subscriber, map, leave.clone(), None);
        }
        leaves
    }

    /// The picker's cut of a meso pick-up, having mailed every other party member on the map
    /// their share. `(picker's amount, a note for the log line)`.
    ///
    /// The rule is the EXP rule (`Session::party_exp_split`): the picker keeps **70%**, every
    /// other member standing on the same map receives the **party share of the whole**
    /// (`RateKind::Party`, 30% until `!setrates` changes it) - a copy each, not a division.
    /// Only for a drop a mob left (`from_mob`); anything else, or no party, or nobody else on
    /// the map, is the whole amount to the picker. Integer arithmetic, so the picker's 70%
    /// rounds down and a 1-meso drop pays the picker 0 and the members 0 - and a member's
    /// zero share is not mailed, since it would draw nothing.
    fn party_meso_split(&self, total: u32, from_mob: bool, picker: u32, map: u32) -> (u32, String) {
        if !from_mob || total == 0 {
            return (total, String::new());
        }
        let members = match self.fields.parties().party_of(picker) {
            Some(p) if p.members.len() >= 2 => p.members.clone(),
            _ => return (total, String::new()),
        };
        let others: Vec<u32> = members.into_iter().filter(|&m| m != picker).collect();
        let eligible = self.bus().characters_on(map, &others);
        if eligible.is_empty() {
            return (total, " (party, alone on the map: all of it)".to_string());
        }
        let share = self.rate(store::rates::RateKind::Party);
        let each = u32::try_from(share.share_of(u64::from(total))).unwrap_or(u32::MAX);
        let mine = total * 70 / 100;
        let mut paid = 0usize;
        if each > 0 {
            for member in &eligible {
                if self.bus().send_to_character(*member, crate::broadcast::Event::PartyMesos { amount: each, picker }) {
                    paid += 1;
                } else {
                    crate::server::log(&format!(
                        "   mesos: party member {member} was on map {map} at the split and gone by delivery; their {each} share was not paid"
                    ));
                }
            }
        }
        crate::server::log(&format!(
            "   mesos: party pick-up on map {map} of {total} from a mob - picker {picker} keeps {mine} (70%, white), {paid} member(s) each receive {each} ({} of the whole, yellow 'Spotting Small Change')",
            share.as_percent()
        ));
        (mine, format!(" (70% of {total}; {paid} party member(s) mailed {each} each)"))
    }

    /// **Receive a party share of mesos somebody else picked up** - `Event::PartyMesos` over
    /// the bus. Credits this character's purse, sends the balance (`0x007C`, the only way the
    /// client learns it) and the client's own yellow line for the event,
    /// `net::message::meso_party_share`. A share too large for that line's `u16` falls back
    /// to the white `You have gained mesos` line rather than drawing nothing.
    pub(super) fn receive_party_mesos(&mut self, amount: u32, picker: u32) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        if amount == 0 {
            return Vec::new();
        }
        let total = match self.store.add_mesos(chr.id, i64::from(amount)) {
            Ok(total) => total,
            Err(e) => return self.notice(format!("Could not credit your party share of mesos: {e}")),
        };
        let mut out = vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange { meso: Some(u64::from(total)), ..Default::default() }.build(),
            what: format!("StatChanged: +{amount} mesos -> {total}, a party share of character {picker}'s pick-up. Bit 18."),
        }];
        out.push(match net::message::meso_party_share(amount) {
            Some(body) => Reply {
                opcode: net::message::MESSAGE,
                body,
                what: format!("Message: party share +{amount} mesos as smallChange - 'Spotting Small Change (+{amount})', YELLOW, message area; no white line"),
            },
            None => Reply {
                opcode: net::message::MESSAGE,
                body: net::message::meso_gained(amount.min(i32::MAX as u32) as i32),
                what: format!("Message: party share +{amount} mesos is past the smallChange u16 ({}); the plain WHITE line instead", net::message::PARTY_SHARE_LINE_MAX),
            },
        });
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
        let by_character = crate::drops::pick_up_object_id(payload)
            .filter(|id| self.fields.with_drops(map, |d| d.get(*id).is_some()));
        // **A pet's request, if a pet is out and the player's shape named nothing.** The
        // reference's pet request carries the drop id four bytes later than the player's
        // (`net::drops::PET_PICK_UP_OBJECT_ID_AT`, [R]); this client's builder is in .themida
        // like the player's, so the first capture of a pet reaching a drop is what settles the
        // offset and the opcode - the log line below names both.
        let by_pet = match (by_character, self.active_pet) {
            (None, Some(_)) => net::drops::pet_pick_up_object_id(payload)
                .filter(|id| self.fields.with_drops(map, |d| d.get(*id).is_some())),
            _ => None,
        };
        if let Some(id) = by_pet {
            crate::server::log(&format!(
                "   pet pick-up: 0x{opcode:04X} names live drop {id} at the pet offset - this opcode is the pet's request"
            ));
        }
        let found = by_character.or(by_pet);

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
        // **Under "Party Leader" pick-up rights the roster the drop resolves against is the
        // leader alone** - the killer still takes it through the owner rule. Under "All" it
        // is every current member. `crate::party::Party::pickup_rights`.
        let party_members: Vec<u32> = match party_id.filter(|&p| p != 0) {
            Some(p) => self
                .fields
                .parties()
                .party(p)
                .map(|party| {
                    if party.pickup_rights == crate::party::PICKUP_LEADER_ONLY {
                        vec![party.leader]
                    } else {
                        party.members.clone()
                    }
                })
                .unwrap_or_default(),
            None => Vec::new(),
        };
        let outcome = match by_pet {
            Some(_) => self.fields.with_drops(map, |d| {
                d.take_by_pet(object_id, chr.id, net::pet::PET_INDEX, now, &party_members)
            }),
            None => self.fields.with_drops(map, |d| d.take(object_id, chr.id, now, &party_members)),
        };
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
                // **A party splits a mob's mesos the way it splits its EXP.** The owner,
                // 2026-09-14: the picker keeps 70%, every other member on the map gets a
                // copy of the party share - "provided that the mesos is from mob death";
                // mesos a player dropped are 100% to whoever picks them up, party or not.
                // `from_mob` is the drop's own record of that (set only by the kill path).
                // The members' shares cross the bus as facts and are credited by their own
                // sessions; this one credits the picker's cut and nothing else.
                let (amount, split_note) = self.party_meso_split(drop.meso, drop.from_mob, chr.id, map);
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
                                "StatChanged: +{amount} mesos -> {total}{split_note}. Bit 18, and the ONLY                                  way this client is ever told a meso balance - the SetField                                  stat block has no meso field at all."
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
                out.extend(self.take_leaves_to_field(map, &outcome));
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
                    // the bag write it depends on has succeeded - and it goes to the whole
                    // field, not just this picker.
                    out.extend(self.take_leaves_to_field(map, &outcome));
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
        if outcome.is_silent_refusal() {
            // A pet asked for a drop no mob dropped: the unlock, and nothing on screen.
            out.push(Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: net::inventory::inventory_rejected(),
                what: format!("InventoryOperation: refusing the pet's pick-up with nCount 0 - {}.", outcome.what()),
            });
            return out;
        }
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
