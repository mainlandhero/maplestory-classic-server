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
    /// What this side has put in so far.
    offer: Offer,
}

/// **What one side offers - by reference, never moved.** The owner, 2026-10-03: *"On tester I
/// tried putting in 3000 mesos, on [the other] I tried putting in 21 eggs."*
///
/// An offered item stays in its owner's bag; this remembers which slot and how many. Nothing
/// leaves a bag until the trade completes, in one step, so a crash, a dropped socket or a
/// cancel with things on the table loses nothing and has nothing to give back. The price is
/// that the bag can change under an offer - the completion has to check every line again.
#[derive(Debug, Clone, Default)]
struct Offer {
    items: Vec<Offered>,
    mesos: u64,
}

/// One line of an [`Offer`].
#[derive(Debug, Clone, Copy)]
struct Offered {
    /// 1..=9, the client's grid.
    trade_slot: u8,
    inv: store::InventoryType,
    bag_slot: u16,
    /// What the slot held when it was offered. **Unread until the completion is built** - it
    /// is what that step checks the slot against, since the bag can change under an offer.
    #[allow(dead_code)]
    item_id: u32,
    quantity: u16,
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
                    offer: Offer::default(),
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
                // **The ticket is the inviter's character id.** The client echoes it back in
                // the accept or decline, so it has to identify the pairing, and the inviter's
                // id does that with nothing to store. It is opaque to the client.
                let ticket = chr.id;
                let body = net::trade::invite(
                    net::trade::INVITE_TRADE,
                    chr.id,
                    &chr.name,
                    ticket,
                );
                debug_assert_eq!(body.len(), net::trade::invite_len(&chr.name));
                let sent = self.bus().publish_to_character(
                    target,
                    self.field_of(&chr),
                    Reply {
                        opcode: net::trade::MINIROOM_RESULT,
                        body,
                        what: format!(
                            "MiniroomResult mode 5: trade invite from {} ({}) to character \
                             {target}, {} bytes, type=1. Nothing authenticates - the target is \
                             not checked for being on this map or for wanting the invite.",
                            chr.name,
                            chr.id,
                            net::trade::invite_len(&chr.name)
                        ),
                    },
                );
                // **Say when it went nowhere.** A trade invite to somebody who is not on this
                // channel is the exact case that reads on screen as "the popup is broken",
                // and it is the same failure whether the packet was wrong or the recipient was
                // absent. The log has to separate them.
                crate::server::log(&format!(
                    "   trade: character {} invited character {target} - {}",
                    chr.id,
                    if sent {
                        "invite delivered".to_string()
                    } else {
                        format!(
                            "NOT DELIVERED: character {target} has no live session on map {}. \
                             The popup will not appear and that is not a packet fault.",
                            chr.map_id
                        )
                    }
                ));
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
            net::trade::Request::Leave => self.trade_leave(&chr, "closed the trade window"),
            // **The Trade button is in here.** Its sender has not been found in the client, so
            // the log names the sub-action and the body (the line above this one) for the run
            // that presses it. Nothing moves until it is decoded.
            net::trade::Request::TradeOther { sub } => crate::server::log(&format!(
                "   trade: character {} sent trade action {sub} (mode 0x10), which is not handled yet - nothing moved. 0 is an item and 1 is mesos; the Trade button is the likely sender of this one.",
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
            offer: Offer::default(),
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
// Putting things in, and leaving
// ---------------------------------------------------------------------------------------

impl Session {
    /// Send `body` (a `0x0575`) to another character on this channel. False when they are not
    /// here, which the caller logs.
    fn tell_trade_partner(&self, character: u32, body: Vec<u8>, what: String) -> bool {
        self.bus().publish_to_character_anywhere(character, Reply { opcode: net::trade::MINIROOM_RESULT, body, what })
    }

    /// A trade refusal: said in chat, so it is on screen, and logged. Nothing is put in.
    fn trade_refused(&self, chr: &net::opcode::Character, why: &str) -> Vec<Reply> {
        crate::server::log(&format!("   trade: character {} - {why}. Nothing was put in.", chr.id));
        self.notice(format!("{why}."))
    }

    /// **Mode `0x10` sub 0: an item into the trade grid.**
    ///
    /// Checked against the bag as it is now: the slot holds something, it may be traded
    /// (`store::ItemRules::trade_blocked`, the same list storage refuses), and the quantity is
    /// there - counting what this side already offered from the same slot, so one stack cannot
    /// be offered twice. Not the Cash tab: a cash item goes through the cash trade, which is
    /// another opcode (`0x02F8`). A throwing star or bullet goes whole, as it is recharged
    /// whole.
    ///
    /// Then BOTH windows are told - the putter's too, because the client never draws its own
    /// offer (`net::trade::put_item`) - with the seat each one reads: 0 for the putter, 1 for the
    /// partner - the partner FIRST, and if they cannot be reached the trade closes rather than
    /// leaving the two windows disagreeing ([`Session::trade_partner_unreachable`]). **The item
    /// stays in the bag** ([`Offer`]); nothing is sent to the inventory.
    fn trade_put_item(
        &mut self,
        chr: &net::opcode::Character,
        inv_type: u8,
        bag_slot: i16,
        quantity: u16,
        trade_slot: u8,
    ) -> Vec<Reply> {
        let Some((partner, already)) = self.with_rooms(|rooms| {
            let (room, seat) = rooms.seat_of(chr.id)?;
            let partner = rooms[room].1[1 - seat].as_ref()?.character_id;
            let mine = rooms[room].1[seat].as_ref()?.offer.items.clone();
            Some((partner, mine))
        }) else {
            return self.trade_refused(chr, "You are not trading with anyone");
        };
        if !(1..=net::trade::TRADE_SLOTS).contains(&trade_slot) || already.iter().any(|o| o.trade_slot == trade_slot) {
            return self.trade_refused(chr, &format!("Trade slot {trade_slot} is not free"));
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
        let Some(held) = self
            .store
            .bag_items(chr.id, inv)
            .ok()
            .and_then(|items| items.into_iter().find(|r| r.slot == bag_slot))
            .map(|r| r.item)
        else {
            return self.trade_refused(chr, &format!("Your {inv:?} slot {bag_slot} is empty"));
        };
        if store::ItemRules::trade_blocked(held.item_id) {
            return self.trade_refused(chr, "That item cannot be traded");
        }
        let have = held.kind.quantity();
        let offered: u16 = already.iter().filter(|o| o.inv == inv && o.bag_slot == bag_slot).map(|o| o.quantity).sum();
        let quantity = match held.kind {
            store::ItemKind::Equip(_) => 1,
            store::ItemKind::Bundle { .. } if net::bag::bundle_has_serial(held.item_id) => have,
            store::ItemKind::Bundle { .. } => quantity,
        };
        let rechargeable = net::bag::bundle_has_serial(held.item_id);
        if (quantity == 0 && !rechargeable) || u32::from(offered) + u32::from(quantity) > u32::from(have) || (offered > 0 && (held.kind.is_equip() || rechargeable)) {
            return self.trade_refused(
                chr,
                &format!("You have {have} of that and {offered} already in the trade, so {quantity} more cannot go in"),
            );
        }
        let mut shown = held;
        if let store::ItemKind::Bundle { .. } = shown.kind {
            shown.kind = store::ItemKind::Bundle { quantity };
        }
        let blob = self.item_blob(&shown);
        let line = Offered { trade_slot, inv, bag_slot, item_id: held.item_id, quantity };
        let recorded = self.with_rooms(|rooms| {
            let (room, seat) = rooms.seat_of(chr.id)?;
            let offer = &mut rooms[room].1[seat].as_mut()?.offer;
            if offer.items.iter().any(|o| o.trade_slot == trade_slot) {
                return None;
            }
            offer.items.push(line);
            Some(())
        });
        if recorded.is_none() {
            return self.trade_refused(chr, "The trade closed before that went in");
        }
        let delivered = self.tell_trade_partner(
            partner,
            net::trade::put_item(net::trade::SEAT_PARTNER, trade_slot, &blob),
            format!("MiniroomResult 0x10/0 to character {partner}: their partner put {quantity} x {} in trade slot {trade_slot} (seat 1)", held.item_id),
        );
        if !delivered {
            return self.trade_partner_unreachable(chr, partner);
        }
        crate::server::log(&format!(
            "   trade: character {} put {quantity} x {} from {inv:?} slot {bag_slot} in trade slot {trade_slot}; it stays in the bag until the trade completes. Partner {partner} was told.",
            chr.id,
            held.item_id,
        ));
        vec![Reply {
            opcode: net::trade::MINIROOM_RESULT,
            body: net::trade::put_item(net::trade::SEAT_SELF, trade_slot, &blob),
            what: format!("MiniroomResult 0x10/0 to character {}: own {quantity} x {} drawn in trade slot {trade_slot} (seat 0)", chr.id, held.item_id),
        }]
    }

    /// **Mode `0x10` sub 1: mesos on the table.** The amount is taken as the side's new total
    /// (see `net::trade::Request::PutMesos`), and it must be in the wallet now. Like an item,
    /// it stays there until the trade completes; both windows are told, each with its own seat.
    fn trade_put_mesos(&mut self, chr: &net::opcode::Character, amount: u64) -> Vec<Reply> {
        let wallet = self.store.mesos(chr.id).map(u64::from).unwrap_or(0);
        if amount > wallet {
            return self.trade_refused(chr, &format!("You have {wallet} mesos, not {amount}"));
        }
        let partner = self.with_rooms(|rooms| {
            let (room, seat) = rooms.seat_of(chr.id)?;
            let partner = rooms[room].1[1 - seat].as_ref()?.character_id;
            rooms[room].1[seat].as_mut()?.offer.mesos = amount;
            Some(partner)
        });
        let Some(partner) = partner else {
            return self.trade_refused(chr, "You are not trading with anyone");
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
            "   trade: character {} offers {amount} mesos (has {wallet}); they stay in the wallet until the trade completes. Partner {partner} was told.",
            chr.id,
        ));
        vec![Reply {
            opcode: net::trade::MINIROOM_RESULT,
            body: net::trade::put_mesos(net::trade::SEAT_SELF, amount),
            what: format!("MiniroomResult 0x10/1 to character {}: own offer of {amount} mesos (seat 0)", chr.id),
        }]
    }

    /// **An offer the partner could not be told about ends the trade.** The owner, 2026-10-03:
    /// *"When the player makes an offer, make sure that the counterparty of the trade window
    /// also gets updated of that offer."* Every offer is mirrored to the partner before the
    /// putter's own window draws it; when the partner has no mailbox on this channel any more
    /// (they left between the two packets), the two windows would disagree about what is on the
    /// table. So the room closes instead, and the putter's window with it ("Trade cancelled.",
    /// `0x0575` mode `0x0C` at the putter's own slot). Nothing was moved, so nothing is lost.
    fn trade_partner_unreachable(&mut self, chr: &net::opcode::Character, partner: u32) -> Vec<Reply> {
        let seat = self.with_rooms(|rooms| {
            let (room, seat) = rooms.seat_of(chr.id)?;
            rooms.remove(room);
            Some(seat)
        });
        crate::server::log(&format!(
            "   trade: character {}'s offer could NOT reach partner {partner} (no mailbox on this channel), so the windows would disagree - the room is closed and nothing moved.",
            chr.id
        ));
        let Some(seat) = seat else { return Vec::new() };
        vec![Reply {
            opcode: net::trade::MINIROOM_RESULT,
            body: net::trade::room_leave(seat as u8, net::trade::LEAVE_CANCELLED),
            what: format!("MiniroomResult mode 0x0C to character {} (slot {seat}): own trade window closes, reason {} - the partner could not be told", chr.id, net::trade::LEAVE_CANCELLED),
        }]
    }

    /// **The room closes because `chr` left it** - the window's own close (mode `0x0C`), a
    /// dropped connection or a channel change. The partner's window is closed with "Trade
    /// cancelled by the other character" (`0x0575` mode `0x0C` carrying the partner's OWN
    /// slot, which is the only slot that closes a window). The leaver's client closed its own
    /// window before it sent anything, so it is told nothing. Nothing was moved, so nothing is
    /// given back: both offers were references into bags that never changed.
    fn trade_leave(&mut self, chr: &net::opcode::Character, why: &str) {
        let left = self.with_rooms(|rooms| {
            let (room, seat) = rooms.seat_of(chr.id)?;
            let (ticket, seats) = rooms.remove(room);
            Some((ticket, seat, seats[1 - seat].clone()))
        });
        let Some((ticket, seat, partner)) = left else {
            crate::server::log(&format!("   trade: character {} {why}, but was in no trade room here.", chr.id));
            return;
        };
        let told = partner.as_ref().map(|p| {
            let partner_slot = (1 - seat) as u8;
            self.tell_trade_partner(
                p.character_id,
                net::trade::room_leave(partner_slot, net::trade::LEAVE_CANCELLED_BY_PARTNER),
                format!(
                    "MiniroomResult mode 0x0C to character {} (slot {partner_slot}): their trade window closes, reason {} - cancelled by the other character",
                    p.character_id,
                    net::trade::LEAVE_CANCELLED_BY_PARTNER
                ),
            )
        });
        crate::server::log(&format!(
            "   trade: character {} {why}; room {ticket} closed, nothing moved. {}",
            chr.id,
            match (partner, told) {
                (Some(p), Some(true)) => format!("Partner {} ({}) told: cancelled by the other character.", p.character_id, p.name),
                (Some(p), _) => format!("Partner {} is not on this channel and was not told.", p.character_id),
                (None, _) => "Nobody had accepted yet.".to_string(),
            }
        ));
    }

    /// From `Drop`: a trade does not outlive its player's connection.
    pub(super) fn leave_trade_on_disconnect(&mut self) {
        if let Some(chr) = self.claimed_character() {
            if self.fields.trades().seat_of(chr.id).is_some() {
                self.trade_leave(&chr, "left the channel");
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

    /// **The owner's run, 2026-10-03**: one side puts 3000 mesos in, the other 21 eggs. Each
    /// window is told about both, the putter as seat 0 and the partner as seat 1, the eggs as
    /// a whole item blob of 21 - and **nothing leaves either bag or wallet**.
    #[test]
    fn mesos_and_eggs_go_in_and_both_windows_are_told() {
        let (store, config, fields) = channel();
        let (mut host, host_id, mut guest, guest_id) = trading(&store, &config, &fields);
        store.set_mesos(host_id, 5000).unwrap();
        let egg = 4_031_013; // any Use-tab bundle will do; this is about the wire
        let egg = if store::InventoryType::for_item(egg) == Some(store::InventoryType::Use) { egg } else { 2_000_000 };
        store.add_item(guest_id, store::InventoryType::Use, &store::Item::bundle(egg, 30), 100).unwrap();
        let egg_slot = store.bag_items(guest_id, store::InventoryType::Use).unwrap()[0].slot;

        // Tester2: 3000 mesos, the captured body.
        let mine = host.handle(&miniroom(&[0x10, 0, 0, 0, 1, 0, 0, 0, 0xb8, 0x0b, 0, 0, 0, 0, 0, 0]));
        assert_eq!(results(&mine), vec![net::trade::put_mesos(net::trade::SEAT_SELF, 3000)], "the putter's own window");
        assert_eq!(results(&guest.collect_mail()), vec![net::trade::put_mesos(net::trade::SEAT_PARTNER, 3000)], "the partner's");
        assert_eq!(store.mesos(host_id).unwrap(), 5000, "still in the wallet");

        // Wisp: 21 eggs into trade slot 1.
        let mine = guest.handle(&miniroom(&eggs_body(1, egg_slot, 21)));
        let blob = guest.item_blob(&store::Item::bundle(egg, 21));
        assert_eq!(results(&mine), vec![net::trade::put_item(net::trade::SEAT_SELF, 1, &blob)]);
        assert_eq!(results(&host.collect_mail()), vec![net::trade::put_item(net::trade::SEAT_PARTNER, 1, &blob)]);
        assert_eq!(store.bag_items(guest_id, store::InventoryType::Use).unwrap()[0].item.kind.quantity(), 30, "still in the bag");
        assert!(mine.iter().all(|r| r.opcode != net::inventory::INVENTORY_OPERATION), "and the bag is not told otherwise");
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
        let mesos = |n: u64| {
            let mut b = u32s(&[net::trade::TRADE_ACTION, net::trade::TRADE_PUT_MESOS]);
            b.extend_from_slice(&n.to_le_bytes());
            b
        };
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
        to_host.extend(results(&host.handle(&miniroom(&mesos(3000)))));
        to_guest.extend(results(&guest.handle(&miniroom(&eggs_body(1, guest_slot, 21)))));
        to_host.extend(results(&host.handle(&miniroom(&eggs_body(1, host_slot, 5)))));
        to_host.extend(results(&host.handle(&miniroom(&mesos(2000)))));
        to_guest.extend(results(&guest.handle(&miniroom(&mesos(700)))));
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
    }

    /// A partner who cannot be told (no mailbox on this channel, between two packets) ends the
    /// trade: the putter's window closes with "Trade cancelled.", nothing is drawn as offered,
    /// and the room is gone - the two windows never disagree.
    #[test]
    fn an_offer_the_partner_cannot_hear_closes_the_trade() {
        let (store, config, fields) = channel();
        let (mut host, host_id, guest, _) = trading(&store, &config, &fields);
        store.set_mesos(host_id, 10_000).unwrap();
        fields.bus().part(guest.subscriber); // gone from the bus, room still open
        let mut b = u32s(&[net::trade::TRADE_ACTION, net::trade::TRADE_PUT_MESOS]);
        b.extend_from_slice(&3000u64.to_le_bytes());
        let out = results(&host.handle(&miniroom(&b)));
        assert_eq!(out, vec![net::trade::room_leave(0, net::trade::LEAVE_CANCELLED)], "own window closes, no offer drawn");
        assert!(fields.trades().is_empty(), "the room is gone");
        assert_eq!(store.mesos(host_id).unwrap(), 10_000, "nothing moved");
    }

    /// Refusals put nothing in and tell nobody but the player: more mesos than the wallet, a
    /// slot already used, the same stack twice past what it holds, an empty slot, the Cash tab.
    #[test]
    fn a_put_that_cannot_be_covered_is_refused() {
        let (store, config, fields) = channel();
        let (mut host, host_id, mut guest, _) = trading(&store, &config, &fields);
        store.set_mesos(host_id, 100).unwrap();
        store.add_item(host_id, store::InventoryType::Use, &store::Item::bundle(2_000_000, 30), 100).unwrap();
        let slot = store.bag_items(host_id, store::InventoryType::Use).unwrap()[0].slot;

        let mesos = |n: u64| {
            let mut b = u32s(&[net::trade::TRADE_ACTION, net::trade::TRADE_PUT_MESOS]);
            b.extend_from_slice(&n.to_le_bytes());
            b
        };
        assert!(results(&host.handle(&miniroom(&mesos(101)))).is_empty(), "more than the wallet");
        assert!(!results(&host.handle(&miniroom(&eggs_body(1, slot, 20)))).is_empty(), "20 of 30 go in");
        assert!(results(&host.handle(&miniroom(&eggs_body(1, slot, 1)))).is_empty(), "trade slot 1 is taken");
        assert!(results(&host.handle(&miniroom(&eggs_body(2, slot, 11)))).is_empty(), "20 + 11 is more than 30");
        assert!(!results(&host.handle(&miniroom(&eggs_body(2, slot, 10)))).is_empty(), "20 + 10 is exactly 30");
        assert!(results(&host.handle(&miniroom(&eggs_body(3, slot + 1, 1)))).is_empty(), "an empty slot");
        assert!(results(&host.handle(&miniroom(&eggs_body(10, slot, 1)))).is_empty(), "there is no trade slot 10");
        let mut cash = eggs_body(3, slot, 1);
        cash[8] = 5;
        assert!(results(&host.handle(&miniroom(&cash))).is_empty(), "the Cash tab");
        let told = results(&guest.collect_mail());
        assert_eq!(told.len(), 2, "the partner heard about the two that went in and nothing else: {told:?}");
    }

    /// Closing the window ends the room: the partner's window is closed with "cancelled by the
    /// other character" at THEIR slot, nothing moved, and a put afterwards finds no trade.
    /// Dropping the connection does the same.
    #[test]
    fn leaving_closes_the_partners_window_and_moves_nothing() {
        let (store, config, fields) = channel();
        let (mut host, host_id, mut guest, _) = trading(&store, &config, &fields);
        store.set_mesos(host_id, 5000).unwrap();
        let mut b = u32s(&[net::trade::TRADE_ACTION, net::trade::TRADE_PUT_MESOS]);
        b.extend_from_slice(&3000u64.to_le_bytes());
        host.handle(&miniroom(&b));
        guest.collect_mail();

        assert!(results(&host.handle(&miniroom(&u32s(&[net::trade::ROOM_LEAVE])))).is_empty(), "the leaver closed its own");
        assert_eq!(
            results(&guest.collect_mail()),
            vec![net::trade::room_leave(1, net::trade::LEAVE_CANCELLED_BY_PARTNER)],
            "the guest sits in slot 1"
        );
        assert_eq!(store.mesos(host_id).unwrap(), 5000);
        assert!(results(&host.handle(&miniroom(&b))).is_empty(), "no trade any more");

        // A new trade, and this time the guest's connection drops.
        let (store, config, fields) = channel();
        let (mut host, _, guest, _) = trading(&store, &config, &fields);
        drop(guest);
        assert_eq!(
            results(&host.collect_mail()),
            vec![net::trade::room_leave(0, net::trade::LEAVE_CANCELLED_BY_PARTNER)],
            "the host sits in slot 0"
        );
        assert!(fields.trades().0.is_empty(), "and the room is gone");
    }
}
