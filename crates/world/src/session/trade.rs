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

use std::sync::Mutex;

use super::{Reply, Session};

/// One seat of an open trade room.
#[derive(Debug, Clone)]
struct Seat {
    character_id: u32,
    name: String,
    look: Vec<u8>,
    map_id: crate::fields::FieldKey,
}

/// Every trade room open on this channel, keyed by the ticket - the inviter's character id.
///
/// A `Mutex<Vec<..>>` rather than a map for the reason `session/messenger.rs` gives: there are
/// never many, and a vector keeps a log line's order stable.
static ROOMS: Mutex<Vec<(u32, [Option<Seat>; 2])>> = Mutex::new(Vec::new());

fn with_rooms<T>(f: impl FnOnce(&mut Vec<(u32, [Option<Seat>; 2])>) -> T) -> T {
    let mut rooms = ROOMS.lock().unwrap_or_else(|e| e.into_inner());
    f(&mut rooms)
}

impl Session {
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
                };
                with_rooms(|rooms| {
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
                let dropped = with_rooms(|rooms| {
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
        };
        let room = with_rooms(|rooms| {
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
