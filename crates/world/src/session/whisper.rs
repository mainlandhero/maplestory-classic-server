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
        let target = via_directory.or_else(|| {
            self.store.character_id_by_name(&req.target).ok().flatten().map(|id| (id, req.target.clone(), None))
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
            // /find: answered in words rather than with mode 0x09, whose u8 field's meaning
            // has not been read. `net::whisper`.
            _ => match &target {
                Some((_, name, Some(channel))) => {
                    let where_ = if *channel == self.config.channel_id {
                        "this channel".to_string()
                    } else {
                        format!("channel {}", channel + 1)
                    };
                    self.notice(format!("{name} is on {where_}."))
                }
                Some((_, name, None)) => self.notice(format!("{name} is not online on any channel.")),
                None => vec![Reply {
                    opcode: net::whisper::WHISPER,
                    body: net::whisper::whisper_sent(&req.target, false),
                    what: format!("Whisper 0x01B3 mode 0x0A: /find '{}' - no such character; 'Could not find'", req.target),
                }],
            },
        }
    }
}
