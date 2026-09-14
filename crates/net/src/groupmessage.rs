//! Party chat: `0x0179` in, `0x01B1` out.
//!
//! The owner, 2026-09-14: *"I also tried sending 'Hello' in party chat, my party member does not
//! receive the message. Party chat works differently than map all chat. This message should
//! be broadcasted to all party members across channels and maps as long as the client is
//! online."*
//!
//! Labels are the project's: **[L]** read off this client's listing or a capture, **[D]**
//! derived from two or more [L] facts, **[I]** inferred, **[R]** from the v214 reference
//! source, which scores 1 of 8 against held-out controls and names things only.
//!
//! # The request, captured
//!
//! `world-ch0.log` 23:40:10, the line the owner typed:
//!
//! ```text
//! 0x0179  01 01 d6000000 0500 48656c6c6f
//!         ^  ^  ^        ^    "Hello"
//!         |  |  |        u16 length + text
//!         |  |  u32 recipient character id (214, the other member)
//!         |  u8 recipient count
//!         u8 kind: 1 = party
//! ```
//!
//! `research/msexe-send-opcodes.txt` puts `0x0179` on exactly one builder, `FUN_1411afb40`.
//! The kind byte is the reference's `GroupMessageType` ordinal (`Buddy 0, Party 1, Guild 2,
//! Alliance 3`) **[R]**, and the client's own reply handler tests the same three values
//! against three enable flags (below), which is the check that makes the numbering **[D]**.
//! The recipient list is what the client *thinks* the party is; the server uses its own
//! roster and does not trust the list.
//!
//! # The reply: `CField::OnPacket` case `0x01B1` -> `FUN_1418463c0`
//!
//! Found by lining the reference's `CField::OnPacket` block up against
//! `research/msexe-field-cases.txt` - `GROUP_MESSAGE(737)` sits four after
//! `TRANSFER_FIELD_REQ_IGNORED(733)`, and this client's `0x01B1` handler reads exactly the
//! reference's `groupMessage` body **[L]**, all of it unconditional
//! (`tools/reads.py 0x1418463c0`, then `0x1408d6760` and `0x1408dcb80` for the tail):
//!
//! ```text
//! 141846402  u8   kind            1 = party. Gated on the client's own chat toggles:
//!                                 kind 1 -> [settings+0x144], 2 -> +0x148, 3 -> +0x14c;
//!                                 a disabled kind is dropped at 141847389 (no reads past
//!                                 the byte, so a dropped line cannot desync anything)
//! 141846449  u32  accountId
//! 141846454  u32  characterId
//! 141846463  str  name            the sender's
//! 141846470  str  text
//! 1408d6782  str  name            again - the reference's encodeChatInfo, byte for byte:
//! 1408dcba3  str  text
//! 1408dcbee  raw4 accountId
//! 1408dcbff  raw4 characterId
//! 1408dcc12  raw1 worldId
//! 1408dcc25  raw4 characterId
//! 1408dcc38  raw4 0
//! 1408dcc45  str  ""
//! 1408dcc9a  raw4 0
//! 1408dcca7  str  ""
//! 1408dcd11  raw4 0
//! ```
//!
//! What the client does with the sender's name is its own (`141846661 cmp esi, 1` picks
//! the party branch). **Nothing here has been on a screen yet**; the shape is the listing's.
//!
//! # Nothing here authenticates
//!
//! As everywhere in this project, the channel socket carries no credentials.

use crate::packet::{PacketReader, PacketWriter};

/// `0x0179` - the client sends a line to a group. See the module docs for the capture.
pub const CLIENT_GROUP_MESSAGE: u16 = 0x0179;

/// `0x01B1` - a group line arrives. `CField::OnPacket` case -> `FUN_1418463c0`. **[L]**
pub const GROUP_MESSAGE: u16 = 0x01B1;

/// The kind byte, first in both directions. **[D]** - see the module docs.
pub mod kind {
    pub const BUDDY: u8 = 0;
    pub const PARTY: u8 = 1;
    pub const GUILD: u8 = 2;
    pub const ALLIANCE: u8 = 3;
}

/// A parsed [`CLIENT_GROUP_MESSAGE`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupMessageRequest {
    pub kind: u8,
    /// Who the client thinks should hear it. Informational; the server uses its roster.
    pub recipients: Vec<u32>,
    pub text: String,
}

/// Parse the body after the opcode. `None` unless every field is present.
pub fn parse_group_message(body: &[u8]) -> Option<GroupMessageRequest> {
    let mut r = PacketReader::new(body);
    let kind = r.u8().ok()?;
    let count = r.u8().ok()?;
    let mut recipients = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        recipients.push(r.u32().ok()?);
    }
    let text = r.str().ok()?;
    Some(GroupMessageRequest { kind, recipients, text })
}

/// Build a [`GROUP_MESSAGE`] body in the client's read order. `text` is folded to ASCII
/// like a chat notice: the same printer, the same font.
pub fn group_message(kind: u8, account_id: u32, character_id: u32, world_id: u8, name: &str, text: &str) -> Vec<u8> {
    let text = crate::notice::ascii_fold(text);
    let mut w = PacketWriter::new();
    w.u8(kind); //                  141846402
    w.u32(account_id); //           141846449
    w.u32(character_id); //         141846454
    w.str(name); //                 141846463
    w.str(&text); //                141846470
    // FUN_1408d6760 -> FUN_1408dcb80: the chat-info block, every read unconditional.
    w.str(name); //                 1408d6782
    w.str(&text); //                1408dcba3
    w.u32(account_id); //           1408dcbee  raw4
    w.u32(character_id); //         1408dcbff  raw4
    w.u8(world_id); //              1408dcc12  raw1
    w.u32(character_id); //         1408dcc25  raw4
    w.u32(0); //                    1408dcc38  raw4
    w.str(""); //                   1408dcc45
    w.u32(0); //                    1408dcc9a  raw4
    w.str(""); //                   1408dcca7
    w.u32(0); //                    1408dcd11  raw4 (tail jmp)
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The owner's own line, byte for byte out of `world-ch0.log`.
    #[test]
    fn the_captured_hello_parses_as_a_party_line_to_one_member() {
        let body = [0x01, 0x01, 0xd6, 0x00, 0x00, 0x00, 0x05, 0x00, b'H', b'e', b'l', b'l', b'o'];
        assert_eq!(
            parse_group_message(&body),
            Some(GroupMessageRequest { kind: kind::PARTY, recipients: vec![214], text: "Hello".into() })
        );
        assert_eq!(parse_group_message(&body[..12]), None, "a short text is refused, not padded");
        assert_eq!(parse_group_message(&[1]), None);
        assert_eq!(parse_group_message(&[]), None);
    }

    /// The reply, field by field at the offsets the reads above land on, and the whole
    /// length: the handler reads every byte and nothing else, so a body one byte off in
    /// either direction is the failure `CLAUDE.md` records twice.
    #[test]
    fn the_reply_is_the_handlers_read_order_and_nothing_else() {
        let b = group_message(kind::PARTY, 7, 214, 0, "Cobalt", "Hello");
        let mut at = 0;
        let u8_at = |b: &[u8], at: &mut usize| { let v = b[*at]; *at += 1; v };
        let u32_at = |b: &[u8], at: &mut usize| { let v = u32::from_le_bytes(b[*at..*at + 4].try_into().unwrap()); *at += 4; v };
        let str_at = |b: &[u8], at: &mut usize| {
            let n = u16::from_le_bytes([b[*at], b[*at + 1]]) as usize;
            let s = String::from_utf8(b[*at + 2..*at + 2 + n].to_vec()).unwrap();
            *at += 2 + n;
            s
        };
        assert_eq!(u8_at(&b, &mut at), 1, "kind");
        assert_eq!(u32_at(&b, &mut at), 7, "accountId");
        assert_eq!(u32_at(&b, &mut at), 214, "characterId");
        assert_eq!(str_at(&b, &mut at), "Cobalt");
        assert_eq!(str_at(&b, &mut at), "Hello");
        assert_eq!(str_at(&b, &mut at), "Cobalt", "chat info: name again");
        assert_eq!(str_at(&b, &mut at), "Hello");
        assert_eq!(u32_at(&b, &mut at), 7);
        assert_eq!(u32_at(&b, &mut at), 214);
        assert_eq!(u8_at(&b, &mut at), 0, "worldId");
        assert_eq!(u32_at(&b, &mut at), 214);
        assert_eq!(u32_at(&b, &mut at), 0);
        assert_eq!(str_at(&b, &mut at), "");
        assert_eq!(u32_at(&b, &mut at), 0);
        assert_eq!(str_at(&b, &mut at), "");
        assert_eq!(u32_at(&b, &mut at), 0);
        assert_eq!(at, b.len(), "every byte accounted for");
        // The text goes through the chat fold: the font has no glyph for an umlaut.
        let f = group_message(kind::PARTY, 7, 214, 0, "Cobalt", "Übel");
        assert_eq!(f, group_message(kind::PARTY, 7, 214, 0, "Cobalt", "Ubel"));
    }
}
