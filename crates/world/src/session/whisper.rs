//! Whispers - `0x017B` in, `0x01B3` to the target wherever they are, and the sender's
//! result. `net::whisper` has the bytes.
//!
//! The target is found by name: in the hub's directory first (every channel's online
//! characters, `crate::link`), then this channel's bus through the store's id for the name.
//! Delivery is `Session::deliver_anywhere`, so a target on another channel gets it through
//! the hub. **The sender is always answered**: found or not, `0x0A` goes back, because the
//! client's own text for the miss - *'Could not find %s.'* - is that arm's.

use super::*;

impl Session {
    /// `0x017B`.
    pub(super) fn on_whisper(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(req) = net::whisper::parse_whisper(body) else {
            crate::server::log(&format!("   whisper: 0x017B did not parse ({} bytes); nothing sent", body.len()));
            return Vec::new();
        };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let Some(account_id) = self.claimed().map(|c| c.account_id) else { return Vec::new() };

        // Where is the target? The directory knows every channel; the store knows the id
        // for a name that is on this channel but not (yet) in the directory.
        let via_directory = crate::link::installed().and_then(|l| {
            let d = l.directory.lock().unwrap_or_else(|e| e.into_inner());
            d.find_by_name(&req.target).map(|(id, e)| (id, e.name.clone(), Some(e.channel)))
        });
        // **Without a hub, this channel still knows its own players**, and not asking it is
        // what made a find for somebody standing on the same map answer "not online on any
        // channel" (the owner, 2026-09-22). The store gives the id; the bus says whether they are
        // playing here.
        let target = via_directory.or_else(|| {
            self.store.character_id_by_name(&req.target).ok().flatten().map(|id| {
                let here = self.bus().character_online(id).then_some(self.config.channel_id);
                (id, req.target.clone(), here)
            })
        });

        match req.kind {
            net::whisper::kind::WHISPER => {
                let delivered = match &target {
                    Some((id, _, _)) if *id != chr.id => {
                        let packet = Reply {
                            opcode: net::whisper::WHISPER,
                            body: net::whisper::whisper_receive(
                                &chr.name,
                                chr.id,
                                u32::try_from(account_id).unwrap_or(0),
                                u8::try_from(self.config.channel_id).unwrap_or(0),
                                u8::try_from(self.config.world_id).unwrap_or(0),
                                &req.text,
                            ),
                            what: format!("Whisper 0x01B3 mode 0x12 to character {id}: {} whispers '{}'", chr.name, req.text),
                        };
                        self.deliver_anywhere(*id, packet)
                    }
                    _ => false,
                };
                crate::server::log(&format!(
                    "   whisper: {} -> '{}': '{}' - {}",
                    chr.name,
                    req.target,
                    req.text,
                    match (&target, delivered) {
                        (Some((id, name, ch)), true) => format!("delivered to {name} ({id}, channel {ch:?})"),
                        (Some((id, _, _)), false) => format!("character {id} exists but is online nowhere this process can reach"),
                        (None, _) => "no such character".to_string(),
                    }
                ));
                vec![Reply {
                    opcode: net::whisper::WHISPER,
                    body: net::whisper::whisper_sent(&req.target, delivered),
                    what: format!(
                        "Whisper 0x01B3 mode 0x0A to the sender: '{}' {} - the client draws {}",
                        req.target,
                        if delivered { "found" } else { "NOT found" },
                        if delivered { "'<target><< <text>'" } else { "'Could not find <target>.'" }
                    ),
                }]
            }
            // **/find, and the buddy window's location check (kind 0x44).**
            //
            // The owner, 2026-09-22: *"please do not send a message for that, as those information
            // should only show in the UI where 'Checking location' is."* So **nothing here
            // writes a chat line**: mode `0x09` is the arm that fills that status line, and
            // the client formats it itself as `"<name> - <place>"`. `net::whisper::place`.
            _ => {
                let (place, value, said) = match &target {
                    // On this channel: hand over the MAP ID and let the client name it, which
                    // is what it does with its own `streetName` lookup.
                    Some((id, _, Some(channel))) if *channel == self.config.channel_id => {
                        let map = self
                            .bus()
                            .everyone_here()
                            .into_iter()
                            .find_map(|(who, field)| (who == *id).then_some(field.map))
                            .unwrap_or(0);
                        (net::whisper::place::MAP, map, format!("map {map}"))
                    }
                    // Another channel: the client names the channel.
                    Some((_, _, Some(channel))) => {
                        (net::whisper::place::CHANNEL, *channel, format!("channel {}", channel + 1))
                    }
                    // Offline, or no such character. Answered - an unanswered find leaves
                    // "Checking location" on screen forever - but with no place, so the
                    // window stops waiting and the chat log stays clean.
                    _ => (net::whisper::place::NOWHERE, 0, "nowhere - not online".to_string()),
                };
                let name = target.as_ref().map(|(_, n, _)| n.clone()).unwrap_or_else(|| req.target.clone());
                // The buddy window asks with kind 0x44 (0x40 | /find) and reads the answer only
                // if it comes back with the same 0x40 bit - `net::whisper::mode::FOUND_IN_WINDOW`.
                let in_window = req.kind & 0x40 != 0;
                crate::server::log(&format!(
                    "   whisper: {} asked where '{}' is -> {said}",
                    chr.name, req.target
                ));
                vec![Reply {
                    opcode: net::whisper::WHISPER,
                    body: net::whisper::whisper_found(&name, place, value, in_window),
                    what: format!(
                        "Whisper 0x01B3 mode {:#04x}: '{name}' is at {said} - {}",
                        if in_window { net::whisper::mode::FOUND_IN_WINDOW } else { net::whisper::mode::FOUND },
                        if in_window { "into the buddy window's status line" } else { "a chat line, for /find" }
                    ),
                }]
            }
        }
    }
}
