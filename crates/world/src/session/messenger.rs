//! Maple Chat - `0x01FD` in, `0x00A3` out. `net::messenger` has the bytes and the
//! measured-versus-read status of every mode.
//!
//! What is built: an **open with an invitee** opens the opener's window (mode 0, result 0)
//! and puts the invite dialog on the invitee's client (mode 6), wherever they are - through
//! the hub when they are on another channel. Everything else the client sends (the accept
//! from the dialog, the decline, a typed line) is logged with its bytes: it is the capture
//! the next step needs, and a reply guessed for it has killed this client before.
//!
//! The messenger id is minted per channel process with the channel in its high half, so two
//! channels cannot mint the same one; membership is not tracked yet - the accept has not
//! been captured, so there is nothing to track it with.

use super::*;

static NEXT_MESSENGER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

impl Session {
    /// `0x01FD`.
    pub(super) fn on_messenger(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(req) = net::messenger::parse_messenger(body) else {
            crate::server::log(&format!("   maple chat: 0x01FD did not parse ({} bytes: {:02x?}); nothing sent", body.len(), body));
            return Vec::new();
        };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        match req {
            net::messenger::MessengerRequest::Open { invite } => {
                let id = ((self.config.channel_id + 1) << 16) | NEXT_MESSENGER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let mut out = vec![Reply {
                    opcode: net::messenger::MESSENGER,
                    body: net::messenger::self_enter(id, 0),
                    what: format!("Messenger 0x00A3 mode 0 to character {}: messenger {id:#x} opened, result 0 - the client stores the id and opens the Maple Chat window", chr.id),
                }];
                if let Some(name) = invite {
                    out.extend(self.messenger_invite(id, &chr, &name));
                }
                out
            }
            net::messenger::MessengerRequest::Invite { name } => {
                // An invite from inside an open window. The client's id is not in the
                // request; without membership tracking the invite names a fresh id.
                let id = ((self.config.channel_id + 1) << 16) | NEXT_MESSENGER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                self.messenger_invite(id, &chr, &name)
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
                    "   maple chat: 0x01FD mode {mode} from character {} ({}) is NOT decoded - {} byte(s) after the mode: {rest:02x?}. THIS IS THE CAPTURE: if it followed a click on Accept in the invite dialog, it is the accept",
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
