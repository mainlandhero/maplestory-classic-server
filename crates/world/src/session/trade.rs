//! The trade invite, and the popup on the other player's screen.
//!
//! `net::trade` owns the wire format; this is the join. The owner, 2026-09-09: *"Tester2 also sent
//! Cobalt a trade request, but the trade request pop up never showed up on Cobalt's side."*
//!
//! # What works and what does not, said plainly
//!
//! **The popup works from here.** The invite arrives, and `0x0575` mode 5 goes to the invited
//! player with `type = 1`, which is the field that decides whether the client draws anything
//! at all - and a follow-up scan of all 31 writers of the balloon-kind field showed it is the
//! *only* field that can, because the other 30 writers all store a literal immediate and none
//! of those immediates is the trade popup's. So `type` is not one gate among several; it is
//! the gate. `crates/net/src/trade.rs`'s `invite` doc carries the counts.
//!
//! **The trade WINDOW works from 2026-09-22.** The owner: *"Tester2 just sent the owner a trade
//! request, but after the owner accepts it, the Trade window did not open."* Mode 4's payload was
//! the hole `research/trade-2026-09-09.md` §3 left: it runs through a virtual call on the
//! dialog the room type selects, so the body is a property of the object rather than of the
//! opcode. That call is read now - for a trade it is `FUN_141C423D0`, and its **only** packet
//! read is `FUN_1402ee8d0`, the avatar decoder `0x0224` already uses.
//! `net::trade::room_open` carries the whole body and its working.
//!
//! # A trade room lives here, not in the client
//!
//! The same shape as `session/messenger.rs`: a process-wide table of open rooms, keyed by the
//! **ticket**, which is the inviter's character id. The creator takes slot 0 when they open
//! the room, the accepter takes slot 1, and both then get a mode 4 listing both seats - each
//! with **its own** `mySlot`. One window each, at the same instant.
//!
//! # What is still not known
//!
//! The trailing virtual call in `FUN_141C3ED00` resolves to a method that reads nothing, but
//! that resolution is **[D]**: the slot arithmetic lands in a region shared with a second
//! vtable. If it is wrong the body is short, and a short body is how `0x02AD` killed a client
//! - so plan step 13 asks for `client-exit.log` if the client dies on Accept rather than
//! assuming it will not. Mode `0xB` (somebody entering a window that is already open) is also
//! undecoded, which is why **both** sides get a mode 4 rather than the creator getting an
//! enter notice.

use super::{Reply, Session};

/// One seat of an open trade room.
#[derive(Debug, Clone)]
pub(crate) struct Seat {
    character_id: u32,
    name: String,
    look: Vec<u8>,
    map_id: crate::fields::FieldKey,
    /// Pressed Trade. Once either side has, nothing more goes on the table.
    confirmed: bool,
}

/// Every trade room open on one channel, keyed by the ticket - the inviter's character id.
/// Lives on `crate::fields::Fields` (`Fields::trades`), one per channel, like the messenger
/// rooms.
///
/// A `Vec` rather than a map for the reason `session/messenger.rs` gives: there are never
/// many, and a vector keeps a log line's order stable.
#[derive(Debug, Default)]
pub(crate) struct Rooms(Vec<(u32, [Option<Seat>; 2])>);

impl std::ops::Deref for Rooms {
    type Target = Vec<(u32, [Option<Seat>; 2])>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Rooms {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Rooms {
    /// The room `character` sits in: `(index into the table, their seat)`.
    fn seat_of(&self, character: u32) -> Option<(usize, usize)> {
        self.0.iter().enumerate().find_map(|(i, (_, seats))| {
            seats.iter().position(|s| s.as_ref().is_some_and(|s| s.character_id == character)).map(|seat| (i, seat))
        })
    }
}

impl Session {
    fn with_rooms<T>(&self, f: impl FnOnce(&mut Rooms) -> T) -> T {
        f(&mut self.fields.trades())
    }

    /// `0x017E` - every miniroom action. Today only the invite produces a packet.
    pub(super) fn on_miniroom(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(req) = net::trade::parse_request(body) else {
            crate::server::log(&format!(
                "   trade: 0x017E did not decode ({} byte body)",
                body.len()
            ));
            return Vec::new();
        };
        let Some(chr) = self.claimed_character() else {
            crate::server::log("   trade: 0x017E with no claimed character; ignored");
            return Vec::new();
        };

        match req {
            // The create arrives ~8 ms before the invite and carries no target, so there is
            // nothing to relay yet. Logged because a missing create is how the invite was
            // first mis-read as a single packet.
            net::trade::Request::Create { room_type } => {
                // **The room is remembered now, because the accept carries only a ticket.**
                // The creator takes slot 0. Nothing goes on the wire yet: mode 4 is what
                // opens a window, and a window with one seat has nothing to trade with.
                let seat = Seat {
                    character_id: chr.id,
                    name: chr.name.clone(),
                    look: net::opcode::avatar_look(&chr),
                    map_id: self.field_of(&chr),
                    confirmed: false,
                };
                self.with_rooms(|rooms| {
                    rooms.retain(|(ticket, _)| *ticket != chr.id);
                    rooms.push((chr.id, [Some(seat), None]));
                });
                crate::server::log(&format!(
                    "   trade: character {} opened a miniroom, roomType {room_type} (trade is {}) and took slot 0 of room {}. Nothing to send until somebody accepts.",
                    chr.id,
                    net::trade::ROOM_TYPE_TRADE,
                    chr.id
                ));
            }
            net::trade::Request::Invite { target } => {
                // **The target's own session decides** whether the popup goes up or the
                // inviter is told they are busy - only it knows whether it has a shop, a
                // conversation or storage open. `receive_trade_invite`. The ticket is the
                // inviter's character id: the client echoes it back in the accept or
                // decline, and it identifies the room with nothing to store.
                let field = self.field_of(&chr);
                let sent = self.bus().publish_event_to_character(
                    target,
                    crate::broadcast::Event::TradeInvite { from: chr.id, name: chr.name.clone(), ticket: chr.id, field },
                );
                crate::server::log(&format!(
                    "   trade: character {} invited character {target} - {}",
                    chr.id,
                    if sent {
                        "handed to their session, which answers for itself".to_string()
                    } else {
                        format!("NOT DELIVERED: character {target} is not on this channel. Told \"Unable to find the character.\"")
                    }
                ));
                if !sent {
                    self.with_rooms(|rooms| rooms.retain(|(t, _)| *t != chr.id));
                    return vec![Reply {
                        opcode: net::trade::MINIROOM_RESULT,
                        body: net::trade::invite_result(net::trade::INVITE_NOT_FOUND, ""),
                        what: format!("MiniroomResult mode 6 result 1 to character {}: \"Unable to find the character.\" (target {target})", chr.id),
                    }];
                }
            }
            // Both are answered with NOTHING on purpose - see the module docs. `0x017E` does
            // not latch, so this does not freeze anything; it just does not open a window.
            net::trade::Request::Accept { ticket } => return self.trade_accept(&chr, ticket),
            net::trade::Request::Decline { ticket, reason } => {
                // The room goes with the refusal: leaving it open would let a later accept of
                // the same ticket open a window nobody asked for.
                let dropped = self.with_rooms(|rooms| {
                    let before = rooms.len();
                    rooms.retain(|(t, _)| *t != ticket);
                    before != rooms.len()
                });
                crate::server::log(&format!(
                    "   trade: character {} DECLINED invite ticket {ticket}, reason {reason} (4 is an ordinary refusal, 0xB means a miniroom was already open - and the client sends 4 by itself when it auto-declines). Room {ticket} {}.",
                    chr.id,
                    if dropped { "dropped" } else { "was not open here" }
                ));
            }
            net::trade::Request::PutItem { inv_type, bag_slot, quantity, trade_slot } => {
                return self.trade_put_item(&chr, inv_type, bag_slot, quantity, trade_slot)
            }
            net::trade::Request::PutMesos { amount } => return self.trade_put_mesos(&chr, amount),
            net::trade::Request::Leave => return self.trade_leave(&chr, "closed the trade window"),
            net::trade::Request::Chat { text } => return self.trade_chat(&chr, &text),
            net::trade::Request::TradeConfirm { items } => return self.trade_confirm(&chr, &items),
            net::trade::Request::TradeVerify { items } => return self.trade_verify(&chr, &items),
            net::trade::Request::TradeOther { sub } => crate::server::log(&format!(
                "   trade: character {} sent trade action {sub} (mode 0x10), which is not handled - nothing moved. 0 item, 1 mesos, 2 the Trade button, 5 the partner's check.",
                chr.id
            )),
            net::trade::Request::Other { mode } => crate::server::log(&format!(
                "   trade: character {} sent miniroom mode {mode}, which is not handled. \
                 research/trade-2026-09-09.md section 1.1 tabulates all 24.",
                chr.id
            )),
        }
        Vec::new()
    }

    /// **Mode 3: somebody accepted, so both windows open.**
    ///
    /// The ticket is the inviter's character id ([`Session::on_miniroom`]'s invite arm sets
    /// it), so it is also the room's key. The accepter takes slot 1, and each side is sent a
    /// `0x0575` mode 4 listing **both** seats with its own `mySlot` - see `net::trade`.
    ///
    /// A ticket with no room is answered with nothing and logged: it means the inviter left,
    /// declined first, or changed channel, and inventing a window for it would put a trade
    /// partner on screen who is not there.
    fn trade_accept(&mut self, chr: &net::opcode::Character, ticket: u32) -> Vec<Reply> {
        let me = Seat {
            character_id: chr.id,
            name: chr.name.clone(),
            look: net::opcode::avatar_look(chr),
            map_id: self.field_of(&chr),
            confirmed: false,
        };
        let room = self.with_rooms(|rooms| {
            let slot = rooms.iter().position(|(t, _)| *t == ticket)?;
            if rooms[slot].1[1].is_some() {
                return None; // already full: a second accept of one ticket
            }
            rooms[slot].1[1] = Some(me);
            Some(rooms[slot].1.clone())
        });
        let Some(seats) = room else {
            crate::server::log(&format!(
                "   trade: character {} accepted ticket {ticket}, which this channel has no open room for (the inviter left, declined, or changed channel), or which already has two players. Nothing sent.",
                chr.id
            ));
            return Vec::new();
        };
        let members: Vec<net::trade::RoomMember> = seats
            .iter()
            .enumerate()
            .filter_map(|(i, seat)| {
                seat.as_ref().map(|s| net::trade::RoomMember {
                    slot: i as u8,
                    character_id: s.character_id,
                    name: s.name.clone(),
                    look: s.look.clone(),
                })
            })
            .collect();
        let names: Vec<&str> = members.iter().map(|m| m.name.as_str()).collect();
        // The inviter's copy, over the field bus - their session owns their socket.
        if let Some(host) = seats[0].as_ref() {
            let body = net::trade::room_open(0, net::trade::TRADE_CAPACITY, &members);
            debug_assert_eq!(body.len(), net::trade::room_open_len(&members));
            let sent = self.bus().publish_to_character(
                host.character_id,
                host.map_id,
                Reply {
                    opcode: net::trade::MINIROOM_RESULT,
                    body,
                    what: format!(
                        "MiniroomResult mode 4 to character {} (slot 0): the trade window, {} seat(s) - {names:?}",
                        host.character_id,
                        members.len()
                    ),
                },
            );
            if !sent {
                crate::server::log(&format!(
                    "   trade: the inviter (character {}) has no live session on map {}; their window will not open, and that is not a packet fault.",
                    host.character_id, host.map_id
                ));
            }
        }
        crate::server::log(&format!(
            "   trade: character {} ACCEPTED ticket {ticket} and took slot 1; mode 4 sent to both sides ({names:?}).",
            chr.id
        ));
        let body = net::trade::room_open(1, net::trade::TRADE_CAPACITY, &members);
        debug_assert_eq!(body.len(), net::trade::room_open_len(&members));
        vec![Reply {
            opcode: net::trade::MINIROOM_RESULT,
            body,
            what: format!(
                "MiniroomResult mode 4 to character {} (slot 1): the trade window, {} seat(s) - {names:?}",
                chr.id,
                members.len()
            ),
        }]
    }
}

// ---------------------------------------------------------------------------------------
// Putting things in, chatting, and leaving
// ---------------------------------------------------------------------------------------

impl Session {
    /// Send `body` (a `0x0575`) to another character on this channel. False when they are not
    /// here, which the caller logs.
    fn tell_trade_partner(&self, character: u32, body: Vec<u8>, what: String) -> bool {
        self.bus().publish_to_character_anywhere(character, Reply { opcode: net::trade::MINIROOM_RESULT, body, what })
    }

    /// The room `chr` sits in, as `(their seat, the partner's character id)`, when the window is
    /// open - both seats taken.
    fn trade_partner_of(&self, chr: u32) -> Option<(usize, u32)> {
        self.with_rooms(|rooms| {
            let (room, seat) = rooms.seat_of(chr)?;
            Some((seat, rooms[room].1[1 - seat].as_ref()?.character_id))
        })
    }

    /// A put refused: said in chat so it is on screen, logged, and **answered with an empty
    /// `0x007C`**. A drag into the trade window sets the client's exclusive-request latch and
    /// waits for the inventory update that takes the item out; a refusal that sends nothing
    /// leaves that latch set and every later drag dead - the owner's run of 2026-10-03, after
    /// one stack of arrows. Byte 0 of any `0x007C` clears it (`Session::use_refused` does the
    /// same).
    fn trade_refused(&self, chr: &net::opcode::Character, why: &str) -> Vec<Reply> {
        crate::server::log(&format!("   trade: character {} - {why}. Nothing was put in.", chr.id));
        let mut out = self.notice(format!("{why}."));
        out.push(Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange::default().build(),
            what: format!("StatChanged: EMPTY - a trade put refused ({why}); sent because byte 0 clears the client's request latch"),
        });
        out
    }

    /// **Mode `0x10` sub 0: an item into the trade grid - out of the bag.**
    ///
    /// The owner, 2026-10-03: *"the item disappears from the player's inventory into the trade
    /// window until the conclusion of the trade request"*. So the stack (or the part of it
    /// named) moves into `store::tradeescrow` in one transaction, and the client is sent the
    /// inventory update that takes it out - **which is also what releases its drag latch**:
    /// without it the first put was the last. A throwing star or bullet goes whole. Not the
    /// Cash tab (a cash item is the cash trade's, `0x02F8`), nothing on the `trade_blocked`
    /// list, and only from a bag slot.
    ///
    /// Then both windows draw it: the partner FIRST (seat 1), and if they cannot be reached the
    /// trade closes and the item comes back ([`Session::trade_partner_unreachable`]); then the
    /// putter (seat 0) - the client never draws its own offer.
    fn trade_put_item(
        &mut self,
        chr: &net::opcode::Character,
        inv_type: u8,
        bag_slot: i16,
        quantity: u16,
        trade_slot: u8,
    ) -> Vec<Reply> {
        let Some((_, partner)) = self.trade_partner_of(chr.id) else {
            return self.trade_refused(chr, "You are not trading with anyone");
        };
        if self.trade_pressed(chr.id) {
            return self.trade_refused(chr, "You have pressed Trade - your side can no longer change");
        }
        if !(1..=net::trade::TRADE_SLOTS).contains(&trade_slot) {
            return self.trade_refused(chr, &format!("There is no trade slot {trade_slot}"));
        }
        let Ok(inv) = store::InventoryType::from_wire(i16::from(inv_type)) else {
            return self.trade_refused(chr, &format!("There is no inventory tab {inv_type}"));
        };
        if inv == store::InventoryType::Cash {
            return self.trade_refused(chr, "Cash items cannot be traded here");
        }
        let Ok(bag_slot) = u16::try_from(bag_slot) else {
            return self.trade_refused(chr, "Only items in your inventory can be traded");
        };
        let Some(held) = self.store.inventory_slot(chr.id, inv, bag_slot).ok().flatten() else {
            return self.trade_refused(chr, &format!("Your {inv:?} slot {bag_slot} is empty"));
        };
        if store::ItemRules::trade_blocked(held.item_id) {
            return self.trade_refused(chr, "That item cannot be traded");
        }
        let have = held.kind.quantity();
        let count = match held.kind {
            store::ItemKind::Equip(_) => None,
            store::ItemKind::Bundle { .. } if net::bag::bundle_has_serial(held.item_id) => None,
            store::ItemKind::Bundle { .. } if quantity == 0 || quantity > have => {
                return self.trade_refused(chr, &format!("You have {have} of that, so {quantity} cannot go in"));
            }
            store::ItemKind::Bundle { .. } => Some(quantity).filter(|q| *q < have),
        };
        let (taken, left) = match self.store.escrow_trade_item(chr.id, inv, bag_slot, count, trade_slot) {
            Ok(moved) => moved,
            Err(store::StoreError::SlotOccupied { .. }) => return self.trade_refused(chr, &format!("Trade slot {trade_slot} is not free")),
            Err(e) => return self.trade_refused(chr, &format!("That could not be put in ({e})")),
        };
        let blob = self.item_blob(&taken);
        let n = taken.kind.quantity();
        let delivered = self.tell_trade_partner(
            partner,
            net::trade::put_item(net::trade::SEAT_PARTNER, trade_slot, &blob),
            format!("MiniroomResult 0x10/0 to character {partner}: their partner put {n} x {} in trade slot {trade_slot} (seat 1)", taken.item_id),
        );
        if !delivered {
            let mut out = self.stack_change_replies(inv, bag_slot, left);
            out.extend(self.trade_partner_unreachable(chr, partner));
            return out;
        }
        crate::server::log(&format!(
            "   trade: character {} put {n} x {} from {inv:?} slot {bag_slot} ({left} left there) into trade slot {trade_slot} - out of the bag and held until the trade ends. Partner {partner} was told.",
            chr.id, taken.item_id
        ));
        let mut out = self.stack_change_replies(inv, bag_slot, left);
        out.extend(self.trade_unconfirm_partner_of(chr.id, &chr.name));
        out.push(Reply {
            opcode: net::trade::MINIROOM_RESULT,
            body: net::trade::put_item(net::trade::SEAT_SELF, trade_slot, &blob),
            what: format!("MiniroomResult 0x10/0 to character {}: own {n} x {} drawn in trade slot {trade_slot} (seat 0)", chr.id, taken.item_id),
        });
        out
    }

    /// **Mode `0x10` sub 1: mesos on the table - out of the wallet.** The amount is taken as
    /// this side's new total (`net::trade::Request::PutMesos`): the difference moves between
    /// the wallet and `store::tradeescrow` in one transaction, and the new balance goes to the
    /// client (`0x007C`, which also clears the request latch). The 5% fee is taken from the
    /// RECEIVER when the trade completes (`store::tradeescrow::mesos_after_fee`), not here.
    fn trade_put_mesos(&mut self, chr: &net::opcode::Character, amount: u64) -> Vec<Reply> {
        let Some((_, partner)) = self.trade_partner_of(chr.id) else {
            return self.trade_refused(chr, "You are not trading with anyone");
        };
        if self.trade_pressed(chr.id) {
            return self.trade_refused(chr, "You have pressed Trade - your side can no longer change");
        }
        let Ok(total) = u32::try_from(amount) else {
            return self.trade_refused(chr, &format!("{amount} mesos is more than a wallet holds"));
        };
        let wallet = match self.store.escrow_trade_mesos(chr.id, total) {
            Ok(w) => w,
            Err(e) => return self.trade_refused(chr, &format!("{total} mesos could not be put in ({e})")),
        };
        let delivered = self.tell_trade_partner(
            partner,
            net::trade::put_mesos(net::trade::SEAT_PARTNER, amount),
            format!("MiniroomResult 0x10/1 to character {partner}: their partner offers {amount} mesos (seat 1)"),
        );
        if !delivered {
            return self.trade_partner_unreachable(chr, partner);
        }
        crate::server::log(&format!(
            "   trade: character {} has {total} mesos on the table now; the wallet holds {wallet}. Partner {partner} was told.",
            chr.id
        ));
        let mut out = self.meso_reply(chr.id);
        out.extend(self.trade_unconfirm_partner_of(chr.id, &chr.name));
        out.push(Reply {
            opcode: net::trade::MINIROOM_RESULT,
            body: net::trade::put_mesos(net::trade::SEAT_SELF, amount),
            what: format!("MiniroomResult 0x10/1 to character {}: own offer of {amount} mesos (seat 0)", chr.id),
        });
        out
    }

    /// **Mode 8: a line typed in the trade window's chat.** The owner, 2026-10-03: *"it does
    /// not work and the other client do not see my messages. I don't even see my own player's
    /// messages."* The client draws nothing of its own - the line comes back from the server,
    /// to BOTH windows, as `0x0575` mode 8 sub 0 (`net::trade::chat`), naming the speaker's
    /// absolute seat, which each window compares with its own to colour the line.
    fn trade_chat(&mut self, chr: &net::opcode::Character, text: &str) -> Vec<Reply> {
        let Some((seat, partner)) = self.trade_partner_of(chr.id) else {
            crate::server::log(&format!("   trade: character {} chatted in a trade window with no room here; nothing sent.", chr.id));
            return Vec::new();
        };
        let (account_id, world) = self.claimed.as_ref().map(|c| (c.account_id, c.world_id)).unwrap_or_default();
        let who = net::megaphone::Speaker {
            name: &chr.name,
            account_id: u32::try_from(account_id).unwrap_or(0),
            character_id: chr.id,
            world: u8::try_from(world).unwrap_or(0),
        };
        let body = net::trade::chat(seat as u8, &who, partner, text);
        self.tell_trade_partner(partner, body.clone(), format!("MiniroomResult mode 8: trade chat from {} (slot {seat}): {text:?}", chr.name));
        crate::server::log(&format!("   trade: character {} said {text:?} in the trade window (slot {seat}); sent to both.", chr.id));
        vec![Reply {
            opcode: net::trade::MINIROOM_RESULT,
            body,
            what: format!("MiniroomResult mode 8: own trade chat line (slot {seat}): {text:?}"),
        }]
    }

    /// **Everything this character had on the table, back** - items into the tabs they came
    /// from, mesos into the wallet - and the packets that show it. Lines that no longer fit
    /// stay held and come back at the next login (`return_trade_escrow_at_login`).
    fn trade_give_back(&mut self, character: u32, why: &str) -> Vec<Reply> {
        let returned = match self.store.return_trade_escrow(character, &|id| self.config.shops.max_stack(id)) {
            Ok(r) => r,
            Err(e) => {
                crate::server::log(&format!("   trade: could not give character {character} their offer back ({e}); it stays held for the next login."));
                return Vec::new();
            }
        };
        let mut out = Vec::new();
        for (inv, changed) in &returned.placed {
            out.extend(self.inventory_added_replies(*inv, changed, "back from the trade window"));
        }
        if returned.mesos > 0 {
            out.extend(self.meso_reply(character));
        }
        if !returned.placed.is_empty() || returned.mesos > 0 || returned.kept > 0 {
            crate::server::log(&format!(
                "   trade: character {character} {why}: {} line(s) and {} mesos given back{}.",
                returned.placed.len(),
                returned.mesos,
                if returned.kept > 0 { format!(", {} line(s) did NOT fit and stay held until the next login", returned.kept) } else { String::new() }
            ));
        }
        out
    }

    /// From the login, after the claim: anything a crash left on a trade table goes back into
    /// the bag before the record that draws it is built.
    pub(super) fn return_trade_escrow_at_login(&mut self) {
        if let Some(chr) = self.claimed_character() {
            let _ = self.trade_give_back(chr.id, "logged in with an offer still held from a trade that never ended");
        }
    }

    /// **An offer the partner could not be told about ends the trade.** The owner, 2026-10-03:
    /// *"When the player makes an offer, make sure that the counterparty of the trade window
    /// also gets updated of that offer."* Every offer goes to the partner before the putter's
    /// own window draws it; a partner with no mailbox on this channel (gone between two
    /// packets) would leave the two windows disagreeing. So the room closes, the putter's
    /// offer comes back, and the putter's window closes with "Trade cancelled." (mode `0x0C`
    /// at their own slot). The partner's offer, if any, is theirs to get back at login.
    fn trade_partner_unreachable(&mut self, chr: &net::opcode::Character, partner: u32) -> Vec<Reply> {
        let seat = self.with_rooms(|rooms| {
            let (room, seat) = rooms.seat_of(chr.id)?;
            rooms.remove(room);
            Some(seat)
        });
        crate::server::log(&format!(
            "   trade: character {}'s offer could NOT reach partner {partner} (no mailbox on this channel) - the room is closed and the offer given back.",
            chr.id
        ));
        let mut out = self.trade_give_back(chr.id, "lost their trade partner");
        if let Some(seat) = seat {
            out.push(Reply {
                opcode: net::trade::MINIROOM_RESULT,
                body: net::trade::room_leave(seat as u8, net::trade::LEAVE_CANCELLED),
                what: format!("MiniroomResult mode 0x0C to character {} (slot {seat}): own trade window closes, reason {} - the partner could not be told", chr.id, net::trade::LEAVE_CANCELLED),
            });
        }
        out
    }

    /// **The room closes because `chr` left it** - the window's own close (mode `0x0C`), a
    /// dropped connection or a channel change. Their own offer comes back now (the returned
    /// replies; the leaver's client closed its window itself before it sent anything). The
    /// partner is sent [`crate::broadcast::Event::TradeEnded`]: their own session gives their
    /// offer back and closes their window with "Trade cancelled by the other character".
    fn trade_leave(&mut self, chr: &net::opcode::Character, why: &str) -> Vec<Reply> {
        let left = self.with_rooms(|rooms| {
            let (room, seat) = rooms.seat_of(chr.id)?;
            let (ticket, seats) = rooms.remove(room);
            Some((ticket, seat, seats[1 - seat].clone()))
        });
        let Some((ticket, seat, partner)) = left else {
            crate::server::log(&format!("   trade: character {} {why}, but was in no trade room here.", chr.id));
            return self.trade_give_back(chr.id, why);
        };
        let told = partner.as_ref().map(|p| {
            self.bus().publish_event_to_character(
                p.character_id,
                crate::broadcast::Event::TradeEnded { slot: (1 - seat) as u8, reason: net::trade::LEAVE_CANCELLED_BY_PARTNER, received: None },
            )
        });
        crate::server::log(&format!(
            "   trade: character {} {why}; room {ticket} closed. {}",
            chr.id,
            match (partner, told) {
                (Some(p), Some(true)) => format!("Partner {} ({}) told: cancelled by the other character.", p.character_id, p.name),
                (Some(p), _) => format!("Partner {} is not on this channel; their offer comes back at their next login.", p.character_id),
                (None, _) => "Nobody had accepted yet.".to_string(),
            }
        ));
        self.trade_give_back(chr.id, why)
    }

    /// The trade ended on the partner's side ([`crate::broadcast::Event::TradeEnded`]). With
    /// `received` it COMPLETED and this is what the store already put in this bag and wallet,
    /// drawn before the window closes (the client words its success message from its own
    /// wallet having risen). Without, it was cancelled or failed: this side's offer comes back.
    /// Either way the window closes at `slot` - its own - with the message `reason` picks.
    pub(super) fn receive_trade_ended(&mut self, slot: u8, reason: u32, received: Option<store::tradeescrow::Received>) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let mut out = match received {
            Some(got) => self.trade_draw_received(chr.id, &got),
            None => self.trade_give_back(chr.id, "had the trade end on the other side"),
        };
        out.push(Reply {
            opcode: net::trade::MINIROOM_RESULT,
            body: net::trade::room_leave(slot, reason),
            what: format!("MiniroomResult mode 0x0C to character {} (slot {slot}): their trade window closes, reason {reason}", chr.id),
        });
        out
    }

    /// Whether `character` has pressed Trade. Their own side is then fixed - the client
    /// refuses their puts itself (`FUN_14214B5D0` and `FUN_14214BC10` bail on `room+0x500`, the
    /// flag the button sets), and the server agrees.
    fn trade_pressed(&self, character: u32) -> bool {
        self.with_rooms(|rooms| {
            rooms.seat_of(character).is_some_and(|(room, seat)| rooms[room].1[seat].as_ref().is_some_and(|s| s.confirmed))
        })
    }

    /// **`character`'s side just changed, so a press by their partner no longer counts.** The
    /// owner, 2026-10-03: *"If the other party does not press trade, but instead modifies
    /// items/mesos in the trade window, it will cancel the confirmation of the trade for the
    /// other side, because since trade contents have been changed, the player who originally
    /// accepted needs to reconfirm the contents of the trade."* The partner is told so in red
    /// and has to press Trade again.
    ///
    /// **Whether this client lets them press again is not measured.** Nothing found clears the
    /// client's own pressed flag (`room+0x500`) once the button sets it, nor the partner-ready
    /// flag (`room+0x504`) our 0x10/2 sets - a `[reg+disp]` write scan of the trade class, whose
    /// blind spot is a store through a pointer. If the button stays greyed, the presser can
    /// still cancel (everything comes back) - plan step 43 (h) is the measurement.
    fn trade_unconfirm_partner_of(&mut self, character: u32, changer: &str) -> Vec<Reply> {
        let cleared = self.with_rooms(|rooms| {
            let (room, seat) = rooms.seat_of(character)?;
            let partner = rooms[room].1[1 - seat].as_mut()?;
            std::mem::replace(&mut partner.confirmed, false).then_some(partner.character_id)
        });
        if let Some(partner) = cleared {
            let text = format!("{changer} changed the trade. Press Trade again to accept the new contents.");
            self.bus().publish_to_character_anywhere(
                partner,
                Reply {
                    opcode: net::message::MESSAGE,
                    body: net::message::chat_line_system(&text),
                    what: format!("Message chat line (system, category 11): {text:?} - their Trade press no longer counts"),
                },
            );
            crate::server::log(&format!("   trade: character {character} changed their side, so partner {partner}'s Trade press is cancelled; they have to press again."));
        }
        Vec::new()
    }

    /// The items `character` really has on the table, as sorted item ids.
    fn trade_offered_ids(&self, character: u32) -> Vec<u32> {
        let mut ids: Vec<u32> =
            self.store.trade_escrow(character).map(|(lines, _)| lines.iter().map(|l| l.item.item_id).collect()).unwrap_or_default();
        ids.sort_unstable();
        ids
    }

    /// **Mode `0x10` sub 2: the Trade button.** The owner, 2026-10-03: *"when I click both trade
    /// button, the other player does not have an indication that the trade has been accepted
    /// by the player. When both players clicked the trade, the trade does not happen."*
    ///
    /// The first press: the presser's list (`(itemId, checksum)` per item it put in) is checked
    /// against what is really on the table, the presser's side is fixed, and the partner is sent
    /// `0x0575` 0x10/2 - their client marks this side ready (the indicator) and answers by
    /// itself with 0x10/5, its view of this side's items ([`Session::trade_verify`]).
    /// The second press completes it ([`Session::trade_complete`]).
    fn trade_confirm(&mut self, chr: &net::opcode::Character, claimed: &[(u32, u32)]) -> Vec<Reply> {
        let state = self.with_rooms(|rooms| {
            let (room, seat) = rooms.seat_of(chr.id)?;
            let partner = rooms[room].1[1 - seat].as_ref()?;
            let (partner, partner_ready) = (partner.character_id, partner.confirmed);
            let me = rooms[room].1[seat].as_mut()?;
            let again = std::mem::replace(&mut me.confirmed, true);
            Some((seat, partner, partner_ready, again))
        });
        let Some((seat, partner, partner_ready, again)) = state else {
            crate::server::log(&format!("   trade: character {} pressed Trade with no open trade here; nothing done.", chr.id));
            return Vec::new();
        };
        if again {
            return Vec::new();
        }
        let mut listed: Vec<u32> = claimed.iter().map(|&(id, _)| id).collect();
        listed.sort_unstable();
        let offered = self.trade_offered_ids(chr.id);
        if listed != offered {
            return self.trade_fail(chr, seat, partner, net::trade::LEAVE_PROBLEM, &format!("their client listed {listed:?} but the table holds {offered:?}"));
        }
        if partner_ready {
            return self.trade_complete(chr, seat, partner);
        }
        let told = self.tell_trade_partner(
            partner,
            net::trade::partner_confirmed(),
            format!("MiniroomResult 0x10/2 to character {partner}: their partner ({}) pressed Trade", chr.name),
        );
        if !told {
            return self.trade_partner_unreachable(chr, partner);
        }
        crate::server::log(&format!(
            "   trade: character {} pressed Trade ({} item(s) on their side, as listed); their side is fixed and partner {partner} is shown it. Waiting for them.",
            chr.id,
            offered.len()
        ));
        Vec::new()
    }

    /// **Mode `0x10` sub 5: the partner's client checking what it was shown.** Sent by itself
    /// on our 0x10/2: every item it draws on the OTHER side. If that is not what the other side
    /// really has on the table, the two screens disagree and the trade is stopped - "There was
    /// a problem trading the item." for both.
    fn trade_verify(&mut self, chr: &net::opcode::Character, seen: &[(u32, u32)]) -> Vec<Reply> {
        let Some((seat, partner)) = self.trade_partner_of(chr.id) else {
            return Vec::new(); // the trade already ended; a late check has nothing to check
        };
        let mut shown: Vec<u32> = seen.iter().map(|&(id, _)| id).collect();
        shown.sort_unstable();
        let offered = self.trade_offered_ids(partner);
        if shown != offered {
            return self.trade_fail(
                chr,
                seat,
                partner,
                net::trade::LEAVE_PROBLEM,
                &format!("their window shows the partner offering {shown:?} but the table holds {offered:?}"),
            );
        }
        crate::server::log(&format!(
            "   trade: character {}'s window shows partner {partner}'s {} item(s) as they are - checked.",
            chr.id,
            shown.len()
        ));
        Vec::new()
    }

    /// **Both pressed Trade: everything changes hands.** One transaction in the store
    /// (`Store::complete_trade`): each side's escrow into the other's bag, mesos less the 5% fee
    /// (the window's own notice), both escrows emptied. Each side is shown what arrived and its
    /// window closes with "Trade successful." (reason 9) - this side here, the partner through
    /// [`crate::broadcast::Event::TradeEnded`] carrying what landed in their bag.
    ///
    /// If a bag cannot take its side, or a wallet would pass the cap, NOTHING moves and the
    /// trade fails: "Trade unsuccessful." for both, and each offer goes back to its owner.
    fn trade_complete(&mut self, chr: &net::opcode::Character, seat: usize, partner: u32) -> Vec<Reply> {
        let config = self.config.clone();
        let max_stack = move |id: u32| config.shops.max_stack(id);
        match self.store.complete_trade(chr.id, partner, &max_stack) {
            Ok((mine, theirs)) => {
                self.with_rooms(|rooms| {
                    if let Some((room, _)) = rooms.seat_of(chr.id) {
                        rooms.remove(room);
                    }
                });
                crate::server::log(&format!(
                    "   trade: COMPLETED between {} and {partner}. {} received {} line(s) and {} mesos; {partner} received {} line(s) and {} mesos (mesos after the 5% fee).",
                    chr.id,
                    chr.id,
                    mine.placed.len(),
                    mine.mesos,
                    theirs.placed.len(),
                    theirs.mesos
                ));
                self.bus().publish_event_to_character(
                    partner,
                    crate::broadcast::Event::TradeEnded { slot: (1 - seat) as u8, reason: net::trade::LEAVE_TRADE_DONE, received: Some(theirs) },
                );
                let mut out = self.trade_draw_received(chr.id, &mine);
                out.push(Reply {
                    opcode: net::trade::MINIROOM_RESULT,
                    body: net::trade::room_leave(seat as u8, net::trade::LEAVE_TRADE_DONE),
                    what: format!("MiniroomResult mode 0x0C to character {} (slot {seat}): trade window closes, reason 9 - Trade successful", chr.id),
                });
                out
            }
            Err(e) => self.trade_fail(chr, seat, partner, net::trade::LEAVE_UNSUCCESSFUL, &format!("the exchange could not be made ({e})")),
        }
    }

    /// **The trade stops without completing**: the room closes, each offer goes back to its
    /// owner, and both windows close with `reason`'s message.
    fn trade_fail(&mut self, chr: &net::opcode::Character, seat: usize, partner: u32, reason: u32, why: &str) -> Vec<Reply> {
        self.with_rooms(|rooms| {
            if let Some((room, _)) = rooms.seat_of(chr.id) {
                rooms.remove(room);
            }
        });
        crate::server::log(&format!(
            "   trade: STOPPED between {} and {partner} - {why}. Both offers given back, both windows closed with reason {reason}.",
            chr.id
        ));
        self.bus().publish_event_to_character(partner, crate::broadcast::Event::TradeEnded { slot: (1 - seat) as u8, reason, received: None });
        let mut out = self.trade_give_back(chr.id, "had the trade stopped");
        out.push(Reply {
            opcode: net::trade::MINIROOM_RESULT,
            body: net::trade::room_leave(seat as u8, reason),
            what: format!("MiniroomResult mode 0x0C to character {} (slot {seat}): trade window closes, reason {reason} - {why}", chr.id),
        });
        out
    }

    /// The packets that show what a completed trade put in this bag and wallet - the wallet
    /// always, since the client words its success message from its own balance.
    fn trade_draw_received(&mut self, character: u32, got: &store::tradeescrow::Received) -> Vec<Reply> {
        let mut out = Vec::new();
        for (inv, changed) in &got.placed {
            out.extend(self.inventory_added_replies(*inv, changed, "received in a trade"));
        }
        out.extend(self.meso_reply(character));
        out
    }

    /// **What this player is doing that a trade request cannot interrupt**, or `None` when they
    /// are free. The owner, 2026-10-03: a player *"in a state where they cannot accept a trade
    /// request (such as in NPC shop, in dialogue, or in storage)"*. Plus the Cash Shop, and a
    /// trade window already open - the client itself declines with `0xB` in that case.
    fn busy_for_trade(&self) -> Option<&'static str> {
        if self.open_shop.is_some() {
            return Some("in an NPC shop");
        }
        if self.open_storage.is_some() {
            return Some("in storage");
        }
        if self.conversation.is_some() {
            return Some("in an NPC conversation");
        }
        if self.in_cash_shop {
            return Some("in the Cash Shop");
        }
        let me = self.claimed_character()?.id;
        let trading = {
            let rooms = self.fields.trades();
            rooms.seat_of(me).is_some_and(|(room, _)| rooms[room].1.iter().all(Option::is_some))
        };
        trading.then_some("in another trade")
    }

    /// **A trade request reached this player** ([`crate::broadcast::Event::TradeInvite`]).
    ///
    /// The owner, 2026-10-03: *"When a player sends a trade request, normally there should be a red
    /// message in chat box saying that trade request has been sent, or that the player is busy
    /// taking care of other things."* So exactly one of two things happens, and the inviter is
    /// told which, in red (chat category 11, the one the client's own invite results use):
    ///
    /// * **busy** - no popup here; the inviter gets the client's own `0x0575` mode 6 result 2,
    ///   *"'<name>' is doing something else right now."*, and the room is dropped so a stale
    ///   accept cannot open it;
    /// * **free** - the popup (mode 5) here, and the inviter gets *"You have sent a trade
    ///   request to '<name>'."* The client has no string of its own for that line, so the
    ///   server words it, in the client's quoting style.
    ///
    /// A player on another map is "Unable to find the character.", mode 6 result 1: the
    /// popup was always same-map only.
    pub(super) fn receive_trade_invite(&mut self, from: u32, inviter: &str, ticket: u32, field: crate::fields::FieldKey) -> Vec<Reply> {
        let Some(me) = self.claimed_character() else { return Vec::new() };
        let tell_inviter = |s: &Session, body: Vec<u8>, what: String| {
            s.bus().publish_to_character_anywhere(from, Reply { opcode: net::trade::MINIROOM_RESULT, body, what })
        };
        let drop_room = |s: &Session| s.with_rooms(|rooms| rooms.retain(|(t, _)| *t != ticket));
        if self.field_of(&me) != field {
            drop_room(self);
            tell_inviter(
                self,
                net::trade::invite_result(net::trade::INVITE_NOT_FOUND, ""),
                format!("MiniroomResult mode 6 result 1 to character {from}: \"Unable to find the character.\" - {} is on another map", me.name),
            );
            crate::server::log(&format!("   trade: invite from {from} to {} - on another map; inviter told the character was not found.", me.id));
            return Vec::new();
        }
        if let Some(why) = self.busy_for_trade() {
            drop_room(self);
            tell_inviter(
                self,
                net::trade::invite_result(net::trade::INVITE_BUSY, &me.name),
                format!("MiniroomResult mode 6 result 2 to character {from}: \"'{}' is doing something else right now.\" ({why})", me.name),
            );
            crate::server::log(&format!(
                "   trade: invite from {from} ({inviter}) to {} - BUSY, {why}: no popup, the inviter is told \"'{}' is doing something else right now.\"",
                me.id, me.name
            ));
            return Vec::new();
        }
        let text = format!("You have sent a trade request to '{}'.", me.name);
        self.bus().publish_to_character_anywhere(
            from,
            Reply {
                opcode: net::message::MESSAGE,
                body: net::message::chat_line_system(&text),
                what: format!("Message chat line (system, category 11): {text:?} - the trade invite reached its target"),
            },
        );
        crate::server::log(&format!("   trade: invite from {from} ({inviter}) to {} - popup shown, the inviter told it was sent.", me.id));
        let body = net::trade::invite(net::trade::INVITE_TRADE, from, inviter, ticket);
        debug_assert_eq!(body.len(), net::trade::invite_len(inviter));
        vec![Reply {
            opcode: net::trade::MINIROOM_RESULT,
            what: format!(
                "MiniroomResult mode 5: trade invite from {inviter} ({from}) to character {}, {} bytes, type=1. Nothing authenticates.",
                me.id,
                body.len()
            ),
            body,
        }]
    }

    /// From `Drop`: a trade does not outlive its player's connection.
    pub(super) fn leave_trade_on_disconnect(&mut self) {
        if let Some(chr) = self.claimed_character() {
            if self.fields.trades().seat_of(chr.id).is_some() {
                let _ = self.trade_leave(&chr, "left the channel");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    const MAP: u32 = 104_040_000;

    fn channel() -> (Arc<store::Store>, Arc<crate::config::Config>, Arc<crate::fields::Fields>) {
        let store = Arc::new(store::Store::open_in_memory().unwrap());
        let config = Arc::new(crate::config::Config::default());
        (store, config, Arc::new(crate::fields::Fields::new()))
    }

    fn join(
        store: &Arc<store::Store>,
        config: &Arc<crate::config::Config>,
        fields: &Arc<crate::fields::Fields>,
        name: &str,
    ) -> (Session, u32) {
        let account = store.create_account(&name.to_lowercase(), "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: name.to_string(), map_id: MAP, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
        s.claim_for_character(id);
        s.on_field_entered();
        s.collect_mail();
        (s, id)
    }

    fn miniroom(body: &[u8]) -> Vec<u8> {
        let mut b = net::trade::CLIENT_MINIROOM.to_le_bytes().to_vec();
        b.extend_from_slice(body);
        b
    }

    fn u32s(v: &[u32]) -> Vec<u8> {
        v.iter().flat_map(|x| x.to_le_bytes()).collect()
    }

    /// Two players with a trade window open between them: `(host, host id, guest, guest id)`.
    fn trading(store: &Arc<store::Store>, config: &Arc<crate::config::Config>, fields: &Arc<crate::fields::Fields>) -> (Session, u32, Session, u32) {
        let (mut host, host_id) = join(store, config, fields, "Tester2");
        let (mut guest, guest_id) = join(store, config, fields, "Wisp");
        host.handle(&miniroom(&u32s(&[0, net::trade::ROOM_TYPE_TRADE])));
        host.handle(&miniroom(&u32s(&[5, guest_id])));
        guest.collect_mail();
        let mut accept = u32s(&[3, host_id]);
        accept.extend_from_slice(&[0, 0]);
        guest.handle(&miniroom(&accept));
        host.collect_mail();
        (host, host_id, guest, guest_id)
    }

    fn results(out: &[Reply]) -> Vec<Vec<u8>> {
        out.iter().filter(|r| r.opcode == net::trade::MINIROOM_RESULT).map(|r| r.body.clone()).collect()
    }

    fn eggs_body(trade_slot: u8, bag_slot: u16, quantity: u16) -> Vec<u8> {
        let mut b = u32s(&[net::trade::TRADE_ACTION, net::trade::TRADE_PUT_ITEM]);
        b.push(2); // Use
        b.extend_from_slice(&bag_slot.to_le_bytes());
        b.extend_from_slice(&quantity.to_le_bytes());
        b.push(trade_slot);
        b
    }

    fn mesos_body(n: u64) -> Vec<u8> {
        let mut b = u32s(&[net::trade::TRADE_ACTION, net::trade::TRADE_PUT_MESOS]);
        b.extend_from_slice(&n.to_le_bytes());
        b
    }

    fn has(out: &[Reply], opcode: u16) -> bool {
        out.iter().any(|r| r.opcode == opcode)
    }

    /// **The owner's runs, 2026-10-03**: mesos on one side, a part-stack on the other. Each put
    /// LEAVES the bag or wallet (the vanilla behaviour the owner described), the putter is sent
    /// the update that shows it - which is also what releases the client's drag latch, so a
    /// second put works - and both windows draw it, the putter as seat 0 and the partner as
    /// seat 1.
    #[test]
    fn a_put_leaves_the_bag_and_both_windows_draw_it() {
        let (store, config, fields) = channel();
        let (mut host, host_id, mut guest, guest_id) = trading(&store, &config, &fields);
        store.set_mesos(host_id, 5000).unwrap();
        store.add_item(guest_id, store::InventoryType::Use, &store::Item::bundle(2_060_000, 1000), 1000).unwrap();
        let arrows = store.bag_items(guest_id, store::InventoryType::Use).unwrap()[0].slot;

        // Tester2: 3000 mesos, the captured body.
        let mine = host.handle(&miniroom(&[0x10, 0, 0, 0, 1, 0, 0, 0, 0xb8, 0x0b, 0, 0, 0, 0, 0, 0]));
        assert_eq!(results(&mine), vec![net::trade::put_mesos(net::trade::SEAT_SELF, 3000)], "the putter's own window");
        assert!(has(&mine, net::stats::STAT_CHANGED), "the new balance, which also clears the latch");
        assert_eq!(store.mesos(host_id).unwrap(), 2000, "out of the wallet");
        assert_eq!(results(&guest.collect_mail()), vec![net::trade::put_mesos(net::trade::SEAT_PARTNER, 3000)], "the partner's");

        // The other side: 400 of 1000 arrows, then the other 600 - the second put works.
        let mine = guest.handle(&miniroom(&eggs_body(1, arrows, 400)));
        let blob = guest.item_blob(&store::Item::bundle(2_060_000, 400));
        assert_eq!(results(&mine), vec![net::trade::put_item(net::trade::SEAT_SELF, 1, &blob)]);
        assert!(has(&mine, net::inventory::INVENTORY_OPERATION), "the bag is told: 600 left");
        assert_eq!(store.bag_items(guest_id, store::InventoryType::Use).unwrap()[0].item.kind.quantity(), 600);
        assert_eq!(results(&host.collect_mail()), vec![net::trade::put_item(net::trade::SEAT_PARTNER, 1, &blob)]);
        let mine = guest.handle(&miniroom(&eggs_body(2, arrows, 600)));
        assert_eq!(results(&mine).len(), 1, "a second put goes in");
        assert!(store.bag_items(guest_id, store::InventoryType::Use).unwrap().is_empty(), "the slot is empty now");
        assert_eq!(store.trade_escrow(guest_id).unwrap().0.len(), 2);
    }

    /// **The counterparty sees every offer** (the owner, 2026-10-03), as the exact mirror of
    /// what the putter's own window draws: the same packet with the seat flipped, in the order
    /// the offers were made - a sequence of puts from both sides, a meso change included.
    #[test]
    fn every_offer_reaches_the_counterparty_as_a_mirror() {
        let (store, config, fields) = channel();
        let (mut host, host_id, mut guest, guest_id) = trading(&store, &config, &fields);
        store.set_mesos(host_id, 10_000).unwrap();
        store.set_mesos(guest_id, 10_000).unwrap();
        store.add_item(host_id, store::InventoryType::Use, &store::Item::bundle(2_000_000, 30), 100).unwrap();
        store.add_item(guest_id, store::InventoryType::Use, &store::Item::bundle(2_000_001, 30), 100).unwrap();
        let host_slot = store.bag_items(host_id, store::InventoryType::Use).unwrap()[0].slot;
        let guest_slot = store.bag_items(guest_id, store::InventoryType::Use).unwrap()[0].slot;
        // Flip the seat byte (offset 8 in both put bodies): what the putter draws as its own
        // (0) the partner must draw as the other side (1), and nothing else may differ.
        let mirrored = |own: &[Vec<u8>]| -> Vec<Vec<u8>> {
            own.iter()
                .map(|b| {
                    let mut m = b.clone();
                    assert_eq!(m[8], net::trade::SEAT_SELF);
                    m[8] = net::trade::SEAT_PARTNER;
                    m
                })
                .collect()
        };

        // Everything each client was sent, in order: a `handle` also drains that player's
        // mailbox, so the partner's offers arrive interleaved with their own; the last
        // `collect_mail` is what the 100 ms tick would deliver.
        let mut to_host = Vec::new();
        let mut to_guest = Vec::new();
        to_host.extend(results(&host.handle(&miniroom(&mesos_body(3000)))));
        to_guest.extend(results(&guest.handle(&miniroom(&eggs_body(1, guest_slot, 21)))));
        to_host.extend(results(&host.handle(&miniroom(&eggs_body(1, host_slot, 5)))));
        to_host.extend(results(&host.handle(&miniroom(&mesos_body(2000)))));
        to_guest.extend(results(&guest.handle(&miniroom(&mesos_body(700)))));
        to_host.extend(results(&host.collect_mail()));
        to_guest.extend(results(&guest.collect_mail()));

        let split = |all: &[Vec<u8>]| -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
            all.iter().cloned().partition(|b| b[8] == net::trade::SEAT_SELF)
        };
        let (host_own, host_heard) = split(&to_host);
        let (guest_own, guest_heard) = split(&to_guest);
        assert_eq!((host_own.len(), guest_own.len()), (3, 2), "every put drew in the putter's own window");
        assert_eq!(guest_heard, mirrored(&host_own), "the guest saw each of the host's offers, in order");
        assert_eq!(host_heard, mirrored(&guest_own), "and the host each of the guest's");
        assert_eq!(store.mesos(host_id).unwrap(), 8000, "a lower total gave 1000 back");
    }

    /// A partner who cannot be told (no mailbox on this channel, between two packets) ends the
    /// trade: the putter's window closes with "Trade cancelled.", their offer comes back, and
    /// the room is gone - the two windows never disagree.
    #[test]
    fn an_offer_the_partner_cannot_hear_closes_the_trade() {
        let (store, config, fields) = channel();
        let (mut host, host_id, guest, _) = trading(&store, &config, &fields);
        store.set_mesos(host_id, 10_000).unwrap();
        fields.bus().part(guest.subscriber); // gone from the bus, room still open
        let out = host.handle(&miniroom(&mesos_body(3000)));
        assert_eq!(results(&out), vec![net::trade::room_leave(0, net::trade::LEAVE_CANCELLED)], "own window closes, no offer drawn");
        assert!(fields.trades().is_empty(), "the room is gone");
        assert_eq!(store.mesos(host_id).unwrap(), 10_000, "the mesos came back");
        assert_eq!(store.trade_escrow(host_id).unwrap(), (Vec::new(), 0));
    }

    /// Refusals put nothing in, tell nobody but the player - and EVERY one still answers with
    /// a `0x007C`, or the client's drag latch stays set and the window goes dead.
    #[test]
    fn a_put_that_cannot_be_covered_is_refused_and_still_answered() {
        let (store, config, fields) = channel();
        let (mut host, host_id, mut guest, _) = trading(&store, &config, &fields);
        store.set_mesos(host_id, 100).unwrap();
        store.add_item(host_id, store::InventoryType::Use, &store::Item::bundle(2_000_000, 30), 100).unwrap();
        let slot = store.bag_items(host_id, store::InventoryType::Use).unwrap()[0].slot;
        let mut refused = |body: Vec<u8>, why: &str| {
            let out = host.handle(&miniroom(&body));
            assert!(results(&out).is_empty(), "{why}: nothing drawn");
            assert!(has(&out, net::stats::STAT_CHANGED), "{why}: the latch is released");
        };
        refused(mesos_body(101), "more than the wallet");
        refused(eggs_body(10, slot, 1), "there is no trade slot 10");
        refused(eggs_body(3, slot + 1, 1), "an empty slot");
        refused(eggs_body(3, slot, 31), "more than the stack");
        let mut cash = eggs_body(3, slot, 1);
        cash[8] = 5;
        refused(cash, "the Cash tab");
        assert!(!results(&host.handle(&miniroom(&eggs_body(1, slot, 20)))).is_empty(), "20 of 30 go in");
        let out = host.handle(&miniroom(&eggs_body(1, slot, 1)));
        assert!(results(&out).is_empty() && has(&out, net::stats::STAT_CHANGED), "trade slot 1 is taken");
        assert_eq!(store.bag_items(host_id, store::InventoryType::Use).unwrap()[0].item.kind.quantity(), 10);
        assert_eq!(results(&guest.collect_mail()).len(), 1, "the partner heard about the one that went in");
    }

    /// Closing the window ends the room: BOTH offers come back to their owners, the partner's
    /// window is closed with "cancelled by the other character" at THEIR slot, and a put
    /// afterwards finds no trade. Dropping the connection does the same.
    #[test]
    fn leaving_gives_both_offers_back_and_closes_the_partners_window() {
        let (store, config, fields) = channel();
        let (mut host, host_id, mut guest, guest_id) = trading(&store, &config, &fields);
        store.set_mesos(host_id, 5000).unwrap();
        store.add_item(guest_id, store::InventoryType::Use, &store::Item::bundle(2_060_000, 1000), 1000).unwrap();
        let arrows = store.bag_items(guest_id, store::InventoryType::Use).unwrap()[0].slot;
        host.handle(&miniroom(&mesos_body(3000)));
        guest.handle(&miniroom(&eggs_body(1, arrows, 1000)));
        host.collect_mail();
        guest.collect_mail();

        let mine = host.handle(&miniroom(&u32s(&[net::trade::ROOM_LEAVE])));
        assert!(results(&mine).is_empty(), "the leaver closed its own window");
        assert!(has(&mine, net::stats::STAT_CHANGED), "and is shown the mesos back");
        assert_eq!(store.mesos(host_id).unwrap(), 5000);
        let theirs = guest.collect_mail();
        assert_eq!(results(&theirs), vec![net::trade::room_leave(1, net::trade::LEAVE_CANCELLED_BY_PARTNER)], "the guest sits in slot 1");
        assert!(has(&theirs, net::inventory::INVENTORY_OPERATION), "the arrows are drawn back");
        // All 1000 back. (This test's config has no item data, so the stack size is the
        // default and they land as several stacks; a real server uses the arrows' own.)
        let back: u32 = store.bag_items(guest_id, store::InventoryType::Use).unwrap().iter().map(|r| u32::from(r.item.kind.quantity())).sum();
        assert_eq!(back, 1000);
        let out = host.handle(&miniroom(&mesos_body(1)));
        assert!(results(&out).is_empty(), "no trade any more");

        // A new trade, and this time the guest's connection drops with an offer on the table.
        let (store, config, fields) = channel();
        let (mut host, _, mut guest, guest_id) = trading(&store, &config, &fields);
        store.set_mesos(guest_id, 900).unwrap();
        guest.handle(&miniroom(&mesos_body(900)));
        host.collect_mail();
        drop(guest);
        assert_eq!(results(&host.collect_mail()), vec![net::trade::room_leave(0, net::trade::LEAVE_CANCELLED_BY_PARTNER)], "the host sits in slot 0");
        assert!(fields.trades().0.is_empty(), "and the room is gone");
        assert_eq!(store.mesos(guest_id).unwrap(), 900, "the leaver's mesos are back in the database");
    }

    /// What a crash left on a trade table is back in the bag at the next login, before the
    /// record that draws the bag is built.
    #[test]
    fn a_crash_leftover_comes_back_at_login() {
        let (store, config, fields) = channel();
        let (s, id) = join(&store, &config, &fields, "Wisp");
        let account = s.claimed.as_ref().unwrap().account_id;
        drop(s);
        store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_000_000, 7), 100).unwrap();
        let slot = store.bag_items(id, store::InventoryType::Use).unwrap()[0].slot;
        store.escrow_trade_item(id, store::InventoryType::Use, slot, None, 1).unwrap();
        assert!(store.bag_items(id, store::InventoryType::Use).unwrap().is_empty());
        store.create_migration(account, id, 0, 0).unwrap();
        let mut again = Session::joining(store.clone(), config.clone(), fields.clone());
        again.claim_for_character(id);
        assert_eq!(store.bag_items(id, store::InventoryType::Use).unwrap()[0].item, store::Item::bundle(2_000_000, 7));
        assert_eq!(store.trade_escrow(id).unwrap(), (Vec::new(), 0));
    }

    /// **The invite says how it went, in red, to the inviter** (the owner, 2026-10-03): a free
    /// target gets the popup and the inviter "You have sent a trade request to 'Wisp'."; a
    /// target in a shop, storage, a conversation or another trade gets nothing and the inviter
    /// the client's own "'Wisp' is doing something else right now."
    #[test]
    fn the_inviter_is_told_sent_or_busy() {
        let (store, config, fields) = channel();
        let (mut host, _host_id) = join(&store, &config, &fields, "Tester2");
        let (mut guest, guest_id) = join(&store, &config, &fields, "Wisp");
        host.handle(&miniroom(&u32s(&[0, net::trade::ROOM_TYPE_TRADE])));
        host.handle(&miniroom(&u32s(&[5, guest_id])));
        let popup = results(&guest.collect_mail());
        assert_eq!(popup.len(), 1, "the popup");
        assert_eq!(&popup[0][0..4], &[5, 0, 0, 0]);
        let told = host.collect_mail();
        let sent = told.iter().find(|r| r.opcode == net::message::MESSAGE).expect("a chat line");
        assert_eq!(sent.body, net::message::chat_line_system("You have sent a trade request to 'Wisp'."));

        // Busy: Wisp has an NPC shop open.
        guest.open_shop = Some((1012000, Vec::new()));
        host.handle(&miniroom(&u32s(&[0, net::trade::ROOM_TYPE_TRADE])));
        host.handle(&miniroom(&u32s(&[5, guest_id])));
        assert!(results(&guest.collect_mail()).is_empty(), "no popup over the shop");
        assert_eq!(results(&host.collect_mail()), vec![net::trade::invite_result(net::trade::INVITE_BUSY, "Wisp")]);
        assert!(fields.trades().is_empty(), "the room was dropped");
    }

    /// **Trade chat** (the owner, 2026-10-03: *"the other client do not see my messages. I don't
    /// even see my own player's messages"*): a line goes back to BOTH windows, naming the
    /// speaker's own seat.
    #[test]
    fn a_chat_line_reaches_both_windows() {
        let (store, config, fields) = channel();
        let (mut host, host_id, mut guest, guest_id) = trading(&store, &config, &fields);
        let hello = [8u8, 0, 0, 0, 0x0a, 0x0c, 0x47, 0x04, 5, 0, b'h', b'e', b'l', b'l', b'o'];
        let mine = results(&host.handle(&miniroom(&hello)));
        let who = net::megaphone::Speaker { name: "Tester2", account_id: u32::try_from(host.claimed.as_ref().unwrap().account_id).unwrap(), character_id: host_id, world: 0 };
        let want = net::trade::chat(0, &who, guest_id, "hello");
        assert_eq!(mine, vec![want.clone()], "the speaker sees their own line");
        assert_eq!(results(&guest.collect_mail()), vec![want], "and so does the partner, with the speaker's seat");
    }

    /// The Trade button's body: `u8 count` then `(itemId, checksum)` per item.
    fn confirm_body(items: &[u32]) -> Vec<u8> {
        let mut b = u32s(&[net::trade::TRADE_ACTION, net::trade::TRADE_CONFIRM]);
        b.push(items.len() as u8);
        for &id in items {
            b.extend_from_slice(&id.to_le_bytes());
            b.extend_from_slice(&0xdead_beefu32.to_le_bytes());
        }
        b
    }

    /// **Both press Trade and it happens** (the owner, 2026-10-03: *"When both players clicked the
    /// trade, the trade does not happen"*), and the first press shows on the other screen.
    ///
    /// Tester2 offers 2000 mesos, Wisp 1000 arrows. Tester2 presses: Wisp's window is told
    /// (0x10/2, the indicator). Wisp presses: everything crosses - the arrows to Tester2, the
    /// mesos to Wisp **less 5%** (1900) - and both windows close with "Trade successful.", each
    /// at its own slot, after the packets that show what arrived.
    #[test]
    fn both_presses_complete_the_trade_with_the_fee() {
        let (store, config, fields) = channel();
        let (mut host, host_id, mut guest, guest_id) = trading(&store, &config, &fields);
        store.set_mesos(host_id, 5000).unwrap();
        store.set_mesos(guest_id, 100).unwrap();
        store.add_item(guest_id, store::InventoryType::Use, &store::Item::bundle(2_060_000, 1000), 1000).unwrap();
        let arrows = store.bag_items(guest_id, store::InventoryType::Use).unwrap()[0].slot;
        host.handle(&miniroom(&mesos_body(2000)));
        guest.handle(&miniroom(&eggs_body(1, arrows, 1000)));
        host.collect_mail();
        guest.collect_mail();

        // Tester2 presses Trade: nothing for them, the indicator for Wisp.
        assert!(results(&host.handle(&miniroom(&confirm_body(&[])))).is_empty());
        assert_eq!(results(&guest.collect_mail()), vec![net::trade::partner_confirmed()], "Wisp sees Tester2 is ready");
        // Wisp's client checks by itself what it sees on Tester2's side: nothing, which is right.
        let mut verify = u32s(&[net::trade::TRADE_ACTION, net::trade::TRADE_VERIFY]);
        verify.push(0);
        assert!(results(&guest.handle(&miniroom(&verify))).is_empty(), "a matching check stops nothing");

        // Wisp presses Trade: it completes.
        let mine = guest.handle(&miniroom(&confirm_body(&[2_060_000])));
        assert_eq!(results(&mine), vec![net::trade::room_leave(1, net::trade::LEAVE_TRADE_DONE)], "Wisp's window: Trade successful");
        assert!(has(&mine, net::stats::STAT_CHANGED), "Wisp is shown the new balance first");
        let theirs = host.collect_mail();
        assert_eq!(results(&theirs), vec![net::trade::room_leave(0, net::trade::LEAVE_TRADE_DONE)], "Tester2's window too");
        assert!(has(&theirs, net::inventory::INVENTORY_OPERATION), "Tester2 is shown the arrows");

        assert_eq!(store.mesos(guest_id).unwrap(), 100 + 1900, "2000 less the 5% fee");
        assert_eq!(store.mesos(host_id).unwrap(), 3000);
        let got: u32 = store.bag_items(host_id, store::InventoryType::Use).unwrap().iter().map(|r| u32::from(r.item.kind.quantity())).sum();
        assert_eq!(got, 1000, "the arrows crossed");
        assert!(store.bag_items(guest_id, store::InventoryType::Use).unwrap().is_empty());
        assert_eq!(store.trade_escrow(host_id).unwrap(), (Vec::new(), 0));
        assert_eq!(store.trade_escrow(guest_id).unwrap(), (Vec::new(), 0));
        assert!(fields.trades().is_empty(), "the room is gone");
    }

    /// A press whose list does not match the table, or a partner's check that does not, stops
    /// the trade: "There was a problem trading the item." for both, both offers back. And a
    /// player who pressed cannot change their own side.
    #[test]
    fn a_mismatch_stops_the_trade_and_a_press_locks_the_table() {
        let (store, config, fields) = channel();
        let (mut host, host_id, mut guest, guest_id) = trading(&store, &config, &fields);
        store.set_mesos(guest_id, 500).unwrap();
        store.add_item(host_id, store::InventoryType::Use, &store::Item::bundle(2_000_000, 10), 100).unwrap();
        let slot = store.bag_items(host_id, store::InventoryType::Use).unwrap()[0].slot;
        host.handle(&miniroom(&eggs_body(1, slot, 10)));
        guest.collect_mail();

        // Tester2 presses (honestly); now Tester2's own side is fixed.
        host.handle(&miniroom(&confirm_body(&[2_000_000])));
        guest.collect_mail();
        let out = host.handle(&miniroom(&mesos_body(0)));
        assert!(results(&out).is_empty() && has(&out, net::stats::STAT_CHANGED), "the presser's side is fixed, and still answered");
        assert_eq!(store.mesos(guest_id).unwrap(), 500);

        // Wisp's client claims to see something else on Tester2's side.
        let mut verify = u32s(&[net::trade::TRADE_ACTION, net::trade::TRADE_VERIFY]);
        verify.push(1);
        verify.extend_from_slice(&2_000_001u32.to_le_bytes());
        verify.extend_from_slice(&0u32.to_le_bytes());
        let mine = guest.handle(&miniroom(&verify));
        assert_eq!(results(&mine), vec![net::trade::room_leave(1, net::trade::LEAVE_PROBLEM)]);
        let theirs = host.collect_mail();
        assert_eq!(results(&theirs), vec![net::trade::room_leave(0, net::trade::LEAVE_PROBLEM)]);
        assert_eq!(store.bag_items(host_id, store::InventoryType::Use).unwrap()[0].item.kind.quantity(), 10, "back in the bag");
        assert!(fields.trades().is_empty());
    }

    /// **A change after a press cancels that press** (the owner, 2026-10-03). Tester2 presses;
    /// Wisp adds mesos instead of pressing - Tester2 is told in red to press again, and Wisp's
    /// own press then does NOT complete the trade (Tester2's no longer counts) but shows Tester2
    /// that Wisp is ready. Tester2's second press completes it, with the new contents.
    #[test]
    fn a_change_after_a_press_cancels_that_press() {
        let (store, config, fields) = channel();
        let (mut host, host_id, mut guest, guest_id) = trading(&store, &config, &fields);
        store.set_mesos(host_id, 1000).unwrap();
        store.set_mesos(guest_id, 1000).unwrap();
        host.handle(&miniroom(&mesos_body(100)));
        guest.collect_mail();

        host.handle(&miniroom(&confirm_body(&[])));
        assert_eq!(results(&guest.collect_mail()), vec![net::trade::partner_confirmed()]);
        guest.handle(&miniroom(&mesos_body(500)));
        let told = host.collect_mail();
        let line = told.iter().find(|r| r.opcode == net::message::MESSAGE).expect("a red line for the presser");
        assert_eq!(line.body, net::message::chat_line_system("Wisp changed the trade. Press Trade again to accept the new contents."));

        // Wisp presses: not complete, because Tester2's press was cancelled.
        let out = guest.handle(&miniroom(&confirm_body(&[])));
        assert!(results(&out).is_empty(), "nothing closes yet");
        assert_eq!(results(&host.collect_mail()), vec![net::trade::partner_confirmed()], "Tester2 sees Wisp is ready");
        assert_eq!(store.mesos(guest_id).unwrap(), 500, "nothing crossed");

        // Tester2 presses again: done, with the new contents - 500 from Wisp arrives as 475.
        let out = host.handle(&miniroom(&confirm_body(&[])));
        assert_eq!(results(&out), vec![net::trade::room_leave(0, net::trade::LEAVE_TRADE_DONE)]);
        assert_eq!(store.mesos(host_id).unwrap(), 900 + 475);
        assert_eq!(store.mesos(guest_id).unwrap(), 500 + 95);
    }
}
