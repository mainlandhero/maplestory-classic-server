//! Maple Chat - `0x01FD` in, `0x00A3` out. `net::messenger` has the bytes and the
//! measured-versus-read status of every mode.
//!
//! # What is built
//!
//! * **Open with an invitee** (mode 0): a room is made with the opener in seat 0; the opener
//!   gets mode 0 / result 0 (the window opens) and then mode 4 with all six seats, so their
//!   own avatar is drawn; the invitee gets mode 6, the dialog, wherever they are.
//! * **Accept** (mode 7, captured 01:52:50): the accepter takes a free seat; they get mode
//!   0 / result 0 and the six-seat mode 4; everyone already in the room gets the one-record
//!   mode 4 with the newcomer.
//! * **Invite from inside** (mode 5): mode 6 to the named character with the room's id.
//!
//! A typed line and a close have not been captured; they are logged with their bytes.
//!
//! # Rooms live in this channel process
//!
//! The registry is per process and the id carries the channel in its high half. A member
//! on another channel can be *invited* (the dialog crosses through the hub) but their
//! Accept arrives at their own channel, which does not hold the room - it answers with a
//! non-zero result (the window stays shut) and says so. Moving the registry into the hub,
//! the way parties were, is the next step once this shape is on a screen.

use super::*;
use std::sync::Mutex;

static NEXT_MESSENGER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

/// One member as the room remembers them - enough to draw them for a later joiner.
#[derive(Debug, Clone)]
struct Member {
    seat: net::messenger::Seat,
}

#[derive(Debug, Default)]
struct Room {
    seats: [Option<Member>; net::messenger::SEATS],
}

impl Room {
    fn seats_for_wire(&self) -> [Option<net::messenger::Seat>; net::messenger::SEATS] {
        let mut out: [Option<net::messenger::Seat>; net::messenger::SEATS] = Default::default();
        for (i, m) in self.seats.iter().enumerate() {
            out[i] = m.as_ref().map(|m| m.seat.clone());
        }
        out
    }
    fn position_of(&self, character: u32) -> Option<usize> {
        self.seats.iter().position(|m| m.as_ref().is_some_and(|m| m.seat.character_id == character))
    }
    fn free_seat(&self) -> Option<usize> {
        self.seats.iter().position(Option::is_none)
    }
    /// Everyone seated, the caller included.
    fn members(&self) -> Vec<u32> {
        self.seats.iter().flatten().map(|m| m.seat.character_id).collect()
    }
    fn others(&self, character: u32) -> Vec<u32> {
        self.seats
            .iter()
            .flatten()
            .map(|m| m.seat.character_id)
            .filter(|&id| id != character)
            .collect()
    }
}

static ROOMS: Mutex<Vec<(u32, Room)>> = Mutex::new(Vec::new());

fn with_room<T>(id: u32, f: impl FnOnce(&mut Room) -> T) -> Option<T> {
    let mut rooms = ROOMS.lock().unwrap_or_else(|e| e.into_inner());
    rooms.iter_mut().find(|(rid, _)| *rid == id).map(|(_, r)| f(r))
}

fn room_of(character: u32) -> Option<u32> {
    let rooms = ROOMS.lock().unwrap_or_else(|e| e.into_inner());
    rooms.iter().find(|(_, r)| r.position_of(character).is_some()).map(|(id, _)| *id)
}

impl Session {
    /// `0x01FD`.
    pub(super) fn on_messenger(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(req) = net::messenger::parse_messenger(body) else {
            crate::server::log(&format!("   maple chat: 0x01FD did not parse ({} bytes: {:02x?}); nothing sent", body.len(), body));
            return Vec::new();
        };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let me = net::messenger::Seat { character_id: chr.id, name: chr.name.clone(), look: net::opcode::avatar_look(&chr) };
        match req {
            net::messenger::MessengerRequest::Open { invite } => {
                let id = ((self.config.channel_id + 1) << 16) | NEXT_MESSENGER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let mut room = Room::default();
                room.seats[0] = Some(Member { seat: me });
                let seats = room.seats_for_wire();
                ROOMS.lock().unwrap_or_else(|e| e.into_inner()).push((id, room));
                let mut out = vec![
                    Reply {
                        opcode: net::messenger::MESSENGER,
                        body: net::messenger::self_enter(id, 0),
                        what: format!("Messenger 0x00A3 mode 0 to character {}: messenger {id:#x} opened, result 0 - the client stores the id and opens the Maple Chat window", chr.id),
                    },
                    Reply {
                        opcode: net::messenger::MESSENGER,
                        body: net::messenger::members(id, &seats),
                        what: format!("Messenger 0x00A3 mode 4 to character {}: all six seats of {id:#x} - seat 0 is {} with their look, the rest empty. Draws the opener's own avatar", chr.id, chr.name),
                    },
                ];
                if let Some(name) = invite {
                    out.extend(self.messenger_invite(id, &chr, &name));
                }
                out
            }
            net::messenger::MessengerRequest::Enter { messenger_id } => {
                let joined = with_room(messenger_id, |room| {
                    if let Some(pos) = room.position_of(chr.id) {
                        return Some((pos, room.seats_for_wire(), Vec::new()));
                    }
                    let pos = room.free_seat()?;
                    room.seats[pos] = Some(Member { seat: me.clone() });
                    Some((pos, room.seats_for_wire(), room.others(chr.id)))
                })
                .flatten();
                let Some((pos, seats, others)) = joined else {
                    crate::server::log(&format!(
                        "   maple chat: character {} ({}) accepted messenger {messenger_id:#x}, which this channel does not hold (another channel's room, gone, or full) - answered result 1, the window stays shut",
                        chr.id, chr.name
                    ));
                    return vec![Reply {
                        opcode: net::messenger::MESSENGER,
                        body: net::messenger::self_enter(messenger_id, 1),
                        what: format!("Messenger 0x00A3 mode 0 to character {}: messenger {messenger_id:#x} result 1 - not here; nothing opens", chr.id),
                    }];
                };
                // **Everyone already seated is sent the whole table, not just the newcomer.**
                // A one-record mode 4 is refused by the client and kills it - measured, and
                // written up on `net::messenger::members`. The table already holds the
                // newcomer, because the seat was taken above.
                for other in &others {
                    let told = self.deliver_anywhere(
                        *other,
                        Reply {
                            opcode: net::messenger::MESSENGER,
                            body: net::messenger::members(messenger_id, &seats),
                            what: format!("Messenger 0x00A3 mode 4 to character {other}: {} joined {messenger_id:#x} in seat {pos}; all six seats, because a one-record mode 4 is rejected with 0x009E and crashes the client", chr.name),
                        },
                    );
                    if !told {
                        crate::server::log(&format!("   maple chat: member {other} of {messenger_id:#x} is online nowhere this process can reach; not told of the join"));
                    }
                }
                crate::server::log(&format!("   maple chat: {} ({}) joined messenger {messenger_id:#x} in seat {pos}; {} other(s) told", chr.name, chr.id, others.len()));
                vec![
                    Reply {
                        opcode: net::messenger::MESSENGER,
                        body: net::messenger::self_enter(messenger_id, 0),
                        what: format!("Messenger 0x00A3 mode 0 to character {}: joined {messenger_id:#x}, result 0 - the window opens", chr.id),
                    },
                    Reply {
                        opcode: net::messenger::MESSENGER,
                        body: net::messenger::members(messenger_id, &seats),
                        what: format!("Messenger 0x00A3 mode 4 to character {}: all six seats of {messenger_id:#x} for the freshly opened window", chr.id),
                    },
                ]
            }
            net::messenger::MessengerRequest::Chat { text } => {
                let Some(id) = room_of(chr.id) else {
                    crate::server::log(&format!("   maple chat: {} typed a line but is in no room this channel holds; nothing sent", chr.name));
                    return Vec::new();
                };
                let Some((pos, everyone)) = with_room(id, |room| {
                    room.position_of(chr.id).map(|pos| (pos, room.members()))
                })
                .flatten() else {
                    return Vec::new();
                };
                let body = net::messenger::chat(id, pos as u8, &chr.name, &text);
                // **Everyone, the speaker included.** The client's own send builder does not
                // draw the line locally, so leaving the speaker out shows them an empty
                // window. `net::messenger::chat`.
                for member in &everyone {
                    if *member == chr.id {
                        continue;
                    }
                    if !self.deliver_anywhere(*member, Reply {
                        opcode: net::messenger::MESSENGER,
                        body: body.clone(),
                        what: format!("Messenger 0x00A3 mode 3 to character {member}: {} said {text:?} in {id:#x} from seat {pos}", chr.name),
                    }) {
                        crate::server::log(&format!("   maple chat: member {member} of {id:#x} is online nowhere this process can reach; not given the line"));
                    }
                }
                crate::server::log(&format!("   maple chat: {} ({}) said {text:?} in {id:#x} from seat {pos}; {} other(s) told, and the speaker echoed", chr.name, chr.id, everyone.len() - 1));
                vec![Reply {
                    opcode: net::messenger::MESSENGER,
                    body,
                    what: format!("Messenger 0x00A3 mode 3 to character {}: their own line {text:?} echoed back - the client does not draw it itself", chr.id),
                }]
            }
            net::messenger::MessengerRequest::Leave { messenger_id } => {
                let left = with_room(messenger_id, |room| {
                    let pos = room.position_of(chr.id)?;
                    room.seats[pos] = None;
                    Some((pos, room.seats_for_wire(), room.members()))
                })
                .flatten();
                let Some((pos, seats, remaining)) = left else {
                    crate::server::log(&format!("   maple chat: character {} ({}) closed messenger {messenger_id:#x}, which this channel does not hold or which they were not in", chr.id, chr.name));
                    return Vec::new();
                };
                // **The whole table again, with the seat now zero.** `FUN_141184360`'s
                // occupied-window loop reads a record per slot and compares the id it had
                // against the one arriving, so a zeroed seat is how a departure is announced
                // and drawn. There is no "member left" result to send instead: results 1 and
                // 2 both run `FUN_141183cc0`, which clears the reader's OWN messenger id and
                // wipes all six of its slots - sending that to the people still in the room
                // would shut their windows.
                for member in &remaining {
                    if !self.deliver_anywhere(*member, Reply {
                        opcode: net::messenger::MESSENGER,
                        body: net::messenger::members(messenger_id, &seats),
                        what: format!("Messenger 0x00A3 mode 4 to character {member}: {} left {messenger_id:#x} from seat {pos}; all six seats with that one zeroed, which is how the window drops them", chr.name),
                    }) {
                        crate::server::log(&format!("   maple chat: member {member} of {messenger_id:#x} is online nowhere this process can reach; not told of the departure"));
                    }
                }
                if remaining.is_empty() {
                    ROOMS.lock().unwrap_or_else(|e| e.into_inner()).retain(|(rid, _)| *rid != messenger_id);
                    crate::server::log(&format!("   maple chat: {} ({}) left {messenger_id:#x} from seat {pos}; the room is empty and is forgotten", chr.name, chr.id));
                } else {
                    crate::server::log(&format!("   maple chat: {} ({}) left {messenger_id:#x} from seat {pos}; {} remaining told", chr.name, chr.id, remaining.len()));
                }
                // The leaver's own client closed its window before sending this.
                Vec::new()
            }
            net::messenger::MessengerRequest::Invite { name } => match room_of(chr.id) {
                Some(id) => self.messenger_invite(id, &chr, &name),
                None => {
                    crate::server::log(&format!("   maple chat: {} invites '{name}' but is in no room this channel holds; nothing sent", chr.name));
                    Vec::new()
                }
            },
            net::messenger::MessengerRequest::Decline { messenger_id, name } => {
                crate::server::log(&format!(
                    "   maple chat: character {} ({}) DECLINED messenger {messenger_id:#x} from '{name}' (the client sent mode 8 on its own - blocked, or invites are off). Nothing is sent back yet",
                    chr.id, chr.name
                ));
                Vec::new()
            }
            net::messenger::MessengerRequest::Other { mode, rest } => {
                crate::server::log(&format!(
                    "   maple chat: 0x01FD mode {mode} from character {} ({}) is NOT decoded - {} byte(s) after the mode: {rest:02x?}. THIS IS THE CAPTURE: a typed line or a closed window, whichever you just did",
                    chr.id, chr.name, rest.len()
                ));
                Vec::new()
            }
        }
    }

    /// Put the invite dialog on `name`'s client, wherever they are.
    fn messenger_invite(&self, messenger_id: u32, from: &net::opcode::Character, name: &str) -> Vec<Reply> {
        let via_directory = crate::link::installed().and_then(|l| {
            let d = l.directory.lock().unwrap_or_else(|e| e.into_inner());
            d.find_by_name(name).map(|(id, e)| (id, Some(e.channel)))
        });
        let target = via_directory.or_else(|| self.store.character_id_by_name(name).ok().flatten().map(|id| (id, None)));
        let delivered = match target {
            Some((id, _)) if id != from.id => self.deliver_anywhere(
                id,
                Reply {
                    opcode: net::messenger::MESSENGER,
                    body: net::messenger::invite(messenger_id, from.id, &from.name),
                    what: format!("Messenger 0x00A3 mode 6 to character {id}: '{}' invites them to Maple Chat {messenger_id:#x} - the dialog 'Chat invite from {}'", from.name, from.name),
                },
            ),
            _ => false,
        };
        crate::server::log(&format!(
            "   maple chat: {} invites '{name}' to messenger {messenger_id:#x} - {}",
            from.name,
            match (target, delivered) {
                (Some((id, ch)), true) => format!("delivered to {id} (channel {ch:?})"),
                (Some((id, _)), false) => format!("character {id} exists but is online nowhere this process can reach"),
                (None, _) => "no such character".to_string(),
            }
        ));
        Vec::new()
    }
}
