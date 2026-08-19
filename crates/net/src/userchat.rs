//! The player's own chat: the balloon over the head and the line in the chat log.
//!
//! Full working: `research/user-chat.md`.

use crate::PacketWriter;

/// A player said something. **Inbound `0x0231`.**
///
/// ```text
/// u32  characterId
/// u8   flag         0 - see below
/// str  text         u16 length, then bytes
/// u8   tail A       0
/// u8   tail B       0
/// ```
///
/// **The client renders nothing locally.** Typing in the chat box sends `0x00E7` and stops
/// there; the balloon and the chat-log line both come from this packet coming back. The owner,
/// 2026-08-19: three messages typed, nothing on screen, and the capture shows why - the
/// server read the `0x00E7`, matched it against `!map`, and answered nothing.
///
/// # How the opcode was found, and it was found rather than guessed
///
/// `FUN_142784970` is the only function in the image that **both** posts to the chat window
/// and builds a chat balloon: it is one of the 17 call sites that pass window type 7 to the
/// string printer `FUN_1415eca30` (the same sweep `notice::CHAT_NOTICE` rests on) **and**
/// one of the 18 functions that call the balloon factory `FUN_14158f5c0`. Two independently
/// derived lists, and the intersection is two functions. **[L]**
///
/// Its one caller `FUN_1429bafb0` dispatches `0x226..0x276` through a jump table at
/// `0x1429bb5d0`, read out of the exe: **index 11 - opcode `0x0231` - is the case that calls
/// it**. The caller reads the leading `u32` itself, before the switch, so that field is
/// shared by every opcode in the range. **[L]**
///
/// The neighbour at index 0, `0x0226`, goes to `FUN_1427847a0`, which reads
/// `u8, str, str, u8, u8, u8` - **two** strings. A form that carries a name as well as a
/// message is what a whisper looks like, but nothing has tested it, so that is **[I]** and
/// this module does not send it.
///
/// # The three bytes we send as zero
///
/// The `u8` before the text and the two after it are read unconditionally as far as the
/// listing shows, and **nothing is known about what they mean**. In this game family the
/// leading one is an admin/GM flag and one of the trailing ones suppresses the chat-log line
/// so only the balloon shows - which is why they are all `0`: that is the value that asks
/// for the ordinary case of both.
///
/// **Sending all three is the safe direction.** A client that reads fewer bytes than arrive
/// simply never looks at the rest; one that reads more than arrive throws on underrun. So a
/// field that turns out to be gated off costs a wasted byte, and a field left out would cost
/// the session.
pub const USER_CHAT: u16 = 0x0231;

/// The other chat form, which takes a name and a message. **Not sent** - see [`USER_CHAT`].
pub const USER_CHAT_TWO_STRINGS: u16 = 0x0226;

/// What [`user_chat`] adds beyond the text and its length prefix.
pub const USER_CHAT_OVERHEAD: usize = 4 + 1 + 2 + 1 + 1;

/// Build a [`USER_CHAT`].
pub fn user_chat(character_id: u32, text: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character_id);
    w.u8(0); // the flag before the text - an admin marker in this game family
    w.str(text);
    w.u8(0); // tail A
    w.u8(0); // tail B - one of these suppresses the chat-log line; 0 asks for both
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_body_is_the_five_fields_the_client_reads_in_order() {
        let b = user_chat(204, "Hello");
        assert_eq!(b.len(), USER_CHAT_OVERHEAD + "Hello".len());
        assert_eq!(u32::from_le_bytes([b[0], b[1], b[2], b[3]]), 204);
        assert_eq!(b[4], 0);
        assert_eq!(u16::from_le_bytes([b[5], b[6]]) as usize, "Hello".len());
        assert_eq!(&b[7..12], b"Hello");
        // The two trailing bytes are not optional: FUN_142784970 reads them whatever they
        // are, and a client that reads past the end of a body throws.
        assert_eq!(&b[12..], &[0, 0]);
    }

    #[test]
    fn an_empty_message_still_carries_every_field() {
        let b = user_chat(200, "");
        assert_eq!(b.len(), USER_CHAT_OVERHEAD);
        assert_eq!(u16::from_le_bytes([b[5], b[6]]), 0);
    }
}
