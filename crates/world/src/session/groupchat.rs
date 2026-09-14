//! Party chat - `0x0179` in, `0x01B1` to every other member on this channel.
//!
//! The owner, 2026-09-14: *"my party member does not receive the message. Party chat works
//! differently than map all chat. This message should be broadcasted to all party members
//! across channels and maps as long as the client is online."*
//!
//! Maps: yes - the line goes to the member wherever they stand
//! (`Bus::publish_to_character_anywhere`). **Channels: not yet, and the reason is
//! structural.** Each channel is its own process with its own `Fields`, and the party
//! registry lives in `Fields` - a party does not exist across channels today, so a member on
//! the other channel is not on this roster to begin with. Cross-channel delivery needs a
//! shared party registry and a cross-process mailbox first; until then a member who is not
//! on this channel is logged as not told, the same way an invite is.

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
        if req.kind != net::groupmessage::kind::PARTY {
            crate::server::log(&format!(
                "   party chat: kind {} (0 buddy, 2 guild, 3 alliance) is not built; '{}' from {} went nowhere",
                req.kind, req.text, chr.name
            ));
            return Vec::new();
        }
        let members: Vec<u32> = match self.fields.parties().party_of(chr.id) {
            Some(p) => p.members.iter().copied().filter(|&m| m != chr.id).collect(),
            None => {
                crate::server::log(&format!("   party chat: {} is in no party; '{}' went nowhere", chr.name, req.text));
                return Vec::new();
            }
        };
        let packet = net::groupmessage::group_message(
            net::groupmessage::kind::PARTY,
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
                what: format!("GroupMessage 0x01B1 (party) to character {member}: {} says '{}'", chr.name, req.text),
            };
            if self.bus().publish_to_character_anywhere(*member, reply) {
                told += 1;
            } else {
                crate::server::log(&format!(
                    "   party chat: member {member} is not on this channel (or is between fields) and was NOT told '{}'",
                    req.text
                ));
            }
        }
        crate::server::log(&format!(
            "   party chat: {} -> {told} of {} member(s) on this channel: '{}' (the client listed {:?})",
            chr.name,
            members.len(),
            req.text,
            req.recipients
        ));
        Vec::new()
    }
}
