//! The player's own chat: the balloon over the head and the line in the chat log.
//!
//! Full working: `research/user-chat.md`.

use crate::PacketWriter;

/// A player said something. **Inbound `0x0231`.**
///
/// ```text
/// u32  characterId          read by the dispatcher, before its switch
/// u8   flag                 0
/// str  text                 the message
/// --- the speaker object, FUN_1408d6760, ALWAYS present ---
/// str                       0x1408d6782
/// str                       0x1408dcba3   } FUN_1408dcb80,
/// raw  4                    0x1408dcbee   } also always present
/// raw  4                    0x1408dcbff
/// raw  1                    0x1408dcc12
/// raw  4                    0x1408dcc25
/// raw  4                    0x1408dcc38
/// str                       0x1408dcc45
/// raw  4                    0x1408dcc9a
/// str                       0x1408dcca7
/// --- back in FUN_142784970 ---
/// u8                        0x142784a5b
/// u8                        0x142784a69
/// u8                        0x142784a75
/// --- the trailing object, FUN_1408da090 ---
/// raw  4   MUST NOT BE 1    0x1408da0b6, then `CMP r8d,1 / JNE` exits
/// ```
///
/// **43 bytes plus the message.** See [`user_chat`].
///
/// **The client renders nothing locally.** Typing in the chat box sends `0x00E7` and stops;
/// the balloon and the chat-log line both come from this packet coming back.
///
/// # How the opcode was found
///
/// `FUN_142784970` is the only *packet-decoding* function in the image that both posts to
/// chat-window type 7 (one of the 17 sites found by the `rel32` sweep that
/// `notice::CHAT_NOTICE` already rests on) and calls the balloon factory `FUN_14158f5c0`
/// (one of 18). Two independently derived lists; the intersection is two functions and only
/// this one reads a packet. **[L]**
///
/// Its dispatcher `FUN_1429bafb0` normalises with `LEA EAX,[RSI-0x226]` / `CMP EAX,0x50` at
/// `0x1429bb100` and jumps through a table at `0x1429bb5d0`; **index 11 is `0x0231`**. The
/// dispatcher reads the leading `u32` itself, before the switch, so that field is shared by
/// every opcode in the range. **[L]**
///
/// # This body was wrong once, and the way it was wrong is the point
///
/// The first version sent `u32, u8, str, u8, u8` - **20 bytes** - and killed the client with
/// `0xE06D7363`, an unhandled C++ exception. Two instrument failures stacked:
///
/// 1. **Direct-only read counting.** `FUN_142784970` calls two helpers that read, and a scan
///    that only looks for calls to the eight primitives *inside* the function sees neither.
///    This is the third time that mistake has been made here - `research/mob-spawn.md`
///    records it for `FUN_141cc9410`.
/// 2. **Scanning bytes for `0xE8` instead of disassembling.** `44 0f b6 e8` (`MOVZX R13D,AL`)
///    at `0x142784a6e` contains an `0xE8`; a scanner that treats it as a CALL and skips five
///    bytes lands mid-instruction and **silently loses the read at `0x142784a75`**.
///
/// The crash log named the exact chain, and all three return addresses are exact rather than
/// heuristic: `0x142784a58` (the call at `...a53` plus 5), `0x1408d680f` (`...80a` plus 5),
/// `0x1408dcba8` (`...ba3` plus 5). The client read the message, then took our two trailing
/// zeros as an empty string's length, then had nothing left for the next string and threw.
///
/// `tools/reads.py` exists so this cannot happen again: it disassembles, walks helpers, and
/// carries a positive control in its docstring.
pub const USER_CHAT: u16 = 0x0231;

/// The other chat form, at index 0 of the same table. **Not sent.**
///
/// `FUN_1427847a0` reads `u8, str, str, <the same speaker object>, u8, u8, u8` - two strings
/// where `0x0231` has one, which is the shape of a whisper. It reaches the balloon and the
/// chat window through `FUN_1427834b0`. Untested, so unsent, and **it is not the simpler
/// option**: it carries the identical `FUN_1408d6760` object.
pub const USER_CHAT_TWO_STRINGS: u16 = 0x0226;

/// The speaker object at `0x142784a53`: four strings and 21 raw bytes, none of it optional.
///
/// **All empty and all zero.** What the fields mean is not established - the strings land in
/// out-pointers at `obj+8`, `+0x10`, `+0x38`, `+0x40` and the raws at `+0x18`, `+0x1c`,
/// `+0x20`, `+0x24`, `+0x28`, `+0x30` - and the shortest legal encoding is what keeps the
/// client on the path this was measured on. The one thing worth trying later is the
/// speaker's **name** in the first string; it is left empty because a name in a field that
/// turns out to be a title or a medal would render as one.
const SPEAKER_OBJECT_LEN: usize = 2 + 2 + 4 + 4 + 1 + 4 + 4 + 2 + 4 + 2;

/// The trailing object at `0x142784b05`, in its four-byte early-exit form.
///
/// `FUN_1408da090` reads a 4-byte block and then `CMP r8d,1 / JNE 0x1408da1eb` returns.
/// **Any value except 1 stops it there**, so zero costs four bytes and nothing else. With a
/// 1 it would go on to read a `u8`, a sub-object and a string. **[L]**
const TRAILING_OBJECT_LEN: usize = 4;

/// What [`user_chat`] adds beyond the message text itself.
pub const USER_CHAT_OVERHEAD: usize =
    4 + 1 + 2 + SPEAKER_OBJECT_LEN + 3 + TRAILING_OBJECT_LEN;

/// Build a [`USER_CHAT`].
pub fn user_chat(character_id: u32, text: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character_id);
    w.u8(0); // the flag before the text
    w.str(text);

    // The speaker object. Four empty strings and 21 zero bytes, in this exact order - see
    // SPEAKER_OBJECT_LEN. None of it is optional; the client reads every field.
    w.str(""); // 0x1408d6782
    w.str(""); // 0x1408dcba3
    w.zeros(4); // 0x1408dcbee
    w.zeros(4); // 0x1408dcbff
    w.zeros(1); // 0x1408dcc12
    w.zeros(4); // 0x1408dcc25
    w.zeros(4); // 0x1408dcc38
    w.str(""); // 0x1408dcc45
    w.zeros(4); // 0x1408dcc9a
    w.str(""); // 0x1408dcca7

    w.u8(0); // 0x142784a5b
    w.u8(0); // 0x142784a69
    w.u8(0); // 0x142784a75

    // The trailing object, stopped at its first field. Not 1, on purpose.
    w.zeros(4);
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_body_is_every_field_the_client_reads() {
        let b = user_chat(204, "Hello David");
        assert_eq!(
            b.len(),
            USER_CHAT_OVERHEAD + "Hello David".len(),
            "43 + the message; 20 bytes is the version that killed the client"
        );
        assert_eq!(u32::from_le_bytes([b[0], b[1], b[2], b[3]]), 204);
        assert_eq!(b[4], 0);
        assert_eq!(u16::from_le_bytes([b[5], b[6]]) as usize, "Hello David".len());
        assert_eq!(&b[7..18], b"Hello David");
    }

    /// The four bytes at the end must never be 1.
    ///
    /// `FUN_1408da090` reads them and `CMP r8d,1 / JNE` returns. A 1 would send it on to a
    /// `u8`, a sub-object and a string that are not in this body, and reading past the end
    /// is what `0xE06D7363` was.
    #[test]
    fn the_trailing_object_is_stopped_at_its_first_field() {
        let b = user_chat(204, "hi");
        let tail = &b[b.len() - 4..];
        assert_ne!(u32::from_le_bytes(tail.try_into().unwrap()), 1);
    }

    /// Every field is present even when the message is empty - none of them is optional.
    #[test]
    fn an_empty_message_still_carries_every_field() {
        assert_eq!(user_chat(200, "").len(), USER_CHAT_OVERHEAD);
    }

    /// The four strings inside the speaker object are length-prefixed and empty, so the
    /// object is exactly the size the listing adds up to.
    #[test]
    fn the_speaker_object_is_the_size_the_listing_adds_up_to() {
        assert_eq!(SPEAKER_OBJECT_LEN, 29);
        let short = user_chat(1, "");
        let long = user_chat(1, "abcd");
        assert_eq!(long.len() - short.len(), 4, "only the message may vary");
    }
}
