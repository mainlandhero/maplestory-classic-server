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
//! **The trade WINDOW does not.** Accepting needs `0x0575` mode 4 carrying a payload whose
//! per-member portion is dispatched through a virtual call on the open dialog - the body is a
//! property of the object, not of the opcode, and `research/trade-2026-09-09.md` §3 names that
//! as undecoded. So Accept and Decline are recorded and answered with nothing rather than with
//! a guess. **A guessed body killed a client on this same day** (`0x02AD`, 12 bytes where the
//! handler read 13), and this opcode does not latch, so silence here is safe where invention
//! is not.

use super::{Reply, Session};

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
                crate::server::log(&format!(
                    "   trade: character {} opened a miniroom, roomType {room_type} \
                     (trade is {}). Nothing to send until the invite.",
                    chr.id,
                    net::trade::ROOM_TYPE_TRADE
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
                    chr.map_id,
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
            net::trade::Request::Accept { ticket } => crate::server::log(&format!(
                "   trade: character {} ACCEPTED invite ticket {ticket}. No trade window is \
                 sent: 0x0575 mode 4's payload is dispatched through a virtual call on the \
                 open dialog and is undecoded. research/trade-2026-09-09.md section 3.",
                chr.id
            )),
            net::trade::Request::Decline { ticket, reason } => crate::server::log(&format!(
                "   trade: character {} DECLINED invite ticket {ticket}, reason {reason} \
                 (4 is an ordinary refusal, 0xB means a miniroom was already open - and the \
                 client sends 4 by itself when it auto-declines).",
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
}
