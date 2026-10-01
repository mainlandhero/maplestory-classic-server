//! Party and buddy chat - `0x0179` in, `0x01B1` to every recipient, on any channel.
//!
//! **Buddy chat, 2026-09-24.** The owner: *"apparently buddy chat does not work. Buddy chat sent by a
//! player with buddies should go to all online buddies that the player has added."* Kind 0 was
//! logged as "not built" and dropped - `Server Investigation/world-ch0.log` 04:32:28, Moth's
//! `'hewwo'` "went nowhere". It now goes to every **accepted** friend on the sender's list, the
//! server's own list (`store::friends`), not the recipient list the client sends - the same rule
//! party chat follows for its roster. Delivery is the same `deliver_anywhere`, so a buddy on
//! another channel is reached through the hub.
//!
//! The owner, 2026-09-14: *"my party member does not receive the message. Party chat works
//! differently than map all chat. This message should be broadcasted to all party members
//! across channels and maps as long as the client is online."*
//!
//! Maps: the line goes to the member wherever they stand. Channels: through the hub
//! (`crate::link`) - the party registry is the hub-serialised replica every channel holds,
//! so a member on another channel is on the roster, and `Session::deliver_anywhere` hands
//! their copy to the hub, which relays it to the channel that hosts them. A member who is
//! online nowhere this process can reach is logged as not told.

use super::*;

impl Session {
    /// `0x0179`. A party line goes to every other current member on this channel, on any
    /// map. The sender's own client draws its line itself, so nothing goes back to it - a
    /// copy would show the line twice. Buddy/guild/alliance kinds are not built: logged and
    /// answered with nothing, which is safe here because the send sets no latch (the 0x0179
    /// builder `FUN_1411afb40` is a plain send).
    pub(super) fn on_group_message(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(req) = net::groupmessage::parse_group_message(body) else {
            crate::server::log(&format!("   party chat: 0x0179 did not parse ({} bytes); nothing sent", body.len()));
            return Vec::new();
        };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let Some(account_id) = self.claimed().map(|c| c.account_id) else { return Vec::new() };
        let (label, members): (&str, Vec<u32>) = match req.kind {
            net::groupmessage::kind::PARTY => match self.fields.parties().party_of(chr.id) {
                Some(p) => ("party", p.members.iter().copied().filter(|&m| m != chr.id).collect()),
                None => {
                    crate::server::log(&format!("   party chat: {} is in no party; '{}' went nowhere", chr.name, req.text));
                    return Vec::new();
                }
            },
            // **Buddy chat**: every accepted friend, from the server's own list. A request that
            // is still waiting is not a friendship, so it does not receive the line.
            net::groupmessage::kind::BUDDY => (
                "buddy",
                self.store
                    .friends(chr.id)
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|f| f.state == store::friends::FriendState::Accepted)
                    .map(|f| f.friend_id)
                    .collect(),
            ),
            _ => {
                crate::server::log(&format!(
                    "   group chat: kind {} (2 guild, 3 alliance) is not built; '{}' from {} went nowhere",
                    req.kind, req.text, chr.name
                ));
                return Vec::new();
            }
        };
        if members.is_empty() {
            crate::server::log(&format!("   {label} chat: {} has nobody to send '{}' to", chr.name, req.text));
            return Vec::new();
        }
        let packet = net::groupmessage::group_message(
            req.kind,
            u32::try_from(account_id).unwrap_or(0),
            chr.id,
            u8::try_from(self.config.world_id).unwrap_or(0),
            &chr.name,
            &req.text,
        );
        let mut told = 0usize;
        for member in &members {
            let reply = Reply {
                opcode: net::groupmessage::GROUP_MESSAGE,
                body: packet.clone(),
                what: format!("GroupMessage 0x01B1 ({label}) to character {member}: {} says '{}'", chr.name, req.text),
            };
            if self.deliver_anywhere(*member, reply) {
                told += 1;
            } else {
                crate::server::log(&format!(
                    "   {label} chat: {member} is online nowhere this process can reach (offline, no hub, or between fields) and was NOT told '{}'",
                    req.text
                ));
            }
        }
        crate::server::log(&format!(
            "   {label} chat: {} -> {told} of {} recipient(s), here or via the hub: '{}' (the client listed {:?})",
            chr.name,
            members.len(),
            req.text,
            req.recipients
        ));
        Vec::new()
    }
}
