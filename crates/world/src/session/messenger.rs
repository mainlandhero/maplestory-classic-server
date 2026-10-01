//! Maple Chat - `0x01FD` in, `0x00A3` out. `net::messenger` has the bytes and the
//! measured-versus-read status of every mode; [`crate::messenger`] has the rooms.
//!
//! # Rooms belong to the world now, not to a channel (2026-09-24)
//!
//! The owner, from the deployed server: *"the client was not able to accept even when they tried
//! to accept it, the server replied back you are too busy"*, then *"Maple Chat should work
//! cross channel, please use the hub code."* The registry used to be a `static` in this file,
//! so a room lived in the process of whoever opened it and an Accept from any other channel
//! found nothing and answered mode 0 result 1. It is now [`crate::messenger::Rooms`], one
//! replica per channel kept in step by the hub exactly the way parties are.
//!
//! * **Open, Enter and Leave change the registry**, so they are
//!   [`crate::messenger::Request`]s: sent to the hub, applied by every channel in hub order,
//!   and turned into packets by [`messenger_effect_packets`]. With no hub, the session applies
//!   the request to its own replica and does the same thing at once.
//! * **A chat line and an invite change nothing**, so any channel reads its replica for the
//!   room and the members and delivers through `Session::deliver_anywhere`.
//!
//! # Who sends what, so nothing is sent twice
//!
//! The ACTOR's own replies (mode 0 and the six-seat mode 4 that opens their window) are built
//! by the actor's channel. Everyone ELSE in the room is told by **the channel that hosts
//! them**, from the same echo - each channel publishes the new table to the members on its own
//! bus. Every member is hosted on exactly one channel, so every member is told exactly once,
//! and a member whose channel is not the actor's is still told. The same rule covers a channel
//! that dies: the hub echoes a `Disconnect` for each of its players, and the survivors'
//! channels redraw the room without them.

use super::*;

/// What an applied request means on the wire: the actor's own replies, and the table every
/// other member must be shown. Pure, so the hub path and the no-hub path cannot differ.
pub(crate) fn messenger_effect_packets(
    actor: u32,
    outcome: &Result<crate::messenger::Effect, crate::messenger::Refusal>,
) -> (Vec<Reply>, Vec<(u32, Reply)>) {
    use crate::messenger::Effect;
    match outcome {
        Ok(Effect::Opened { room, seats }) => (
            vec![
                Reply {
                    opcode: net::messenger::MESSENGER,
                    body: net::messenger::self_enter(*room, 0),
                    what: format!("Messenger 0x00A3 mode 0 to character {actor}: messenger {room:#x} opened, result 0 - the client stores the id and opens the Maple Chat window"),
                },
                Reply {
                    opcode: net::messenger::MESSENGER,
                    body: net::messenger::members(*room, seats),
                    what: format!("Messenger 0x00A3 mode 4 to character {actor}: all six seats of {room:#x} - seat 0 is the opener with their look, the rest empty"),
                },
            ],
            Vec::new(),
        ),
        Ok(Effect::Joined { room, position, seats, others }) => (
            vec![
                Reply {
                    opcode: net::messenger::MESSENGER,
                    body: net::messenger::self_enter(*room, 0),
                    what: format!("Messenger 0x00A3 mode 0 to character {actor}: joined {room:#x} in seat {position}, result 0 - the window opens"),
                },
                Reply {
                    opcode: net::messenger::MESSENGER,
                    body: net::messenger::members(*room, seats),
                    what: format!("Messenger 0x00A3 mode 4 to character {actor}: all six seats of {room:#x} for the freshly opened window"),
                },
            ],
            // **The whole table, not a one-record mode 4** - that form is refused with 0x009E
            // and kills the client (`net::messenger::members`).
            others
                .iter()
                .map(|&m| {
                    (m, Reply {
                        opcode: net::messenger::MESSENGER,
                        body: net::messenger::members(*room, seats),
                        what: format!("Messenger 0x00A3 mode 4 to character {m}: character {actor} joined {room:#x} in seat {position}; all six seats"),
                    })
                })
                .collect(),
        ),
        // The leaver's own client closed its window before it sent the request, so it is told
        // nothing. The others get the table with the seat zeroed - that is how the window
        // drops someone; results 1 and 2 would shut THEIR windows instead.
        Ok(Effect::Left { room, position, seats, remaining }) => (
            Vec::new(),
            remaining
                .iter()
                .map(|&m| {
                    (m, Reply {
                        opcode: net::messenger::MESSENGER,
                        body: net::messenger::members(*room, seats),
                        what: format!("Messenger 0x00A3 mode 4 to character {m}: character {actor} left {room:#x} from seat {position}; all six seats with that one zeroed"),
                    })
                })
                .collect(),
        ),
        Ok(Effect::Nothing) => (Vec::new(), Vec::new()),
        // A leave for a room the actor is not in: their window is already shut. Nothing.
        Err(crate::messenger::Refusal::NotInRoom(_)) => (Vec::new(), Vec::new()),
        // Enter refused: the room is gone or full. Result 1 keeps the window shut - the one
        // case where "not here" is the true answer now that every channel knows every room.
        Err(refusal) => (
            vec![Reply {
                opcode: net::messenger::MESSENGER,
                body: net::messenger::self_enter(refusal.room(), 1),
                what: format!("Messenger 0x00A3 mode 0 to character {actor}: messenger {:#x} result 1 - {refusal:?}; nothing opens", refusal.room()),
            }],
            Vec::new(),
        ),
    }
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
                // The invite has to name the room, and with the hub up the room's id is only
                // known once the echo comes back - so it is remembered and sent then.
                if let Some(name) = invite {
                    self.pending_messenger_invite = Some(name);
                }
                self.run_messenger_request(chr.id, crate::messenger::Request::Open { seat: me })
            }
            net::messenger::MessengerRequest::Enter { messenger_id } => {
                self.run_messenger_request(chr.id, crate::messenger::Request::Enter { room: messenger_id, seat: me })
            }
            net::messenger::MessengerRequest::Leave { messenger_id } => {
                self.run_messenger_request(chr.id, crate::messenger::Request::Leave { room: messenger_id })
            }
            net::messenger::MessengerRequest::Chat { text } => {
                let found = self.fields.messengers().room_of(chr.id).map(|r| (r.id, r.position_of(chr.id), r.members()));
                let Some((id, Some(pos), everyone)) = found else {
                    crate::server::log(&format!("   maple chat: {} typed a line but is in no room; nothing sent", chr.name));
                    return Vec::new();
                };
                let body = net::messenger::chat(id, pos as u8, &chr.name, &text);
                for member in everyone.iter().filter(|&&m| m != chr.id) {
                    if !self.deliver_anywhere(*member, Reply {
                        opcode: net::messenger::MESSENGER,
                        body: body.clone(),
                        what: format!("Messenger 0x00A3 mode 3 to character {member}: {} said {text:?} in {id:#x} from seat {pos}", chr.name),
                    }) {
                        crate::server::log(&format!("   maple chat: member {member} of {id:#x} is online nowhere this process can reach; not given the line"));
                    }
                }
                crate::server::log(&format!("   maple chat: {} said {text:?} in {id:#x} from seat {pos}; {} other(s), and the speaker echoed", chr.name, everyone.len() - 1));
                // **Everyone, the speaker included** - the client does not draw its own line.
                vec![Reply {
                    opcode: net::messenger::MESSENGER,
                    body,
                    what: format!("Messenger 0x00A3 mode 3 to character {}: their own line {text:?} echoed back - the client does not draw it itself", chr.id),
                }]
            }
            net::messenger::MessengerRequest::Invite { name } => {
                let room = self.fields.messengers().room_of(chr.id).map(|r| r.id);
                match room {
                    Some(id) => self.messenger_invite(id, &chr, &name),
                    None => {
                        crate::server::log(&format!("   maple chat: {} invites '{name}' but is in no room; nothing sent", chr.name));
                        Vec::new()
                    }
                }
            }
            net::messenger::MessengerRequest::Decline { messenger_id, name } => {
                crate::server::log(&format!(
                    "   maple chat: character {} ({}) DECLINED messenger {messenger_id:#x} from '{name}' (the client sent mode 8 on its own - blocked, or invites are off). Nothing is sent back yet",
                    chr.id, chr.name
                ));
                Vec::new()
            }
            net::messenger::MessengerRequest::Other { mode, rest } => {
                crate::server::log(&format!(
                    "   maple chat: 0x01FD mode {mode} from character {} ({}) is NOT decoded - {} byte(s) after the mode: {rest:02x?}",
                    chr.id, chr.name, rest.len()
                ));
                Vec::new()
            }
        }
    }

    /// Send a room change through the hub, or apply it here when there is no hub. With the
    /// hub the actor's replies come back through [`Session::collect_messenger_outcomes`].
    pub(super) fn run_messenger_request(&mut self, actor: u32, request: crate::messenger::Request) -> Vec<Reply> {
        let described = format!("{request:?}");
        if let Some(link) = crate::link::current() {
            if link.send(&crate::link::Frame::MessengerRequest { actor, request: request.clone() }) {
                crate::server::log(&format!("   maple chat: {described} by {actor} sent to the hub; answered on its echo"));
                return Vec::new();
            }
        }
        let outcome = self.fields.messengers().apply(actor, request);
        crate::server::log(&format!("   maple chat: {described} by {actor} applied here (no hub) -> {outcome:?}"));
        let (mine, others) = messenger_effect_packets(actor, &outcome);
        for (member, reply) in others {
            if !self.deliver_anywhere(member, reply) {
                crate::server::log(&format!("   maple chat: member {member} is online nowhere this process can reach; not told"));
            }
        }
        self.after_messenger_outcome(&outcome, mine)
    }

    /// The actor's own replies for an outcome, plus the invite an Open was carrying.
    pub(super) fn after_messenger_outcome(
        &mut self,
        outcome: &Result<crate::messenger::Effect, crate::messenger::Refusal>,
        mut mine: Vec<Reply>,
    ) -> Vec<Reply> {
        if let Ok(crate::messenger::Effect::Opened { room, .. }) = outcome {
            if let (Some(name), Some(chr)) = (self.pending_messenger_invite.take(), self.claimed_character()) {
                mine.extend(self.messenger_invite(*room, &chr, &name));
            }
        }
        mine
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
