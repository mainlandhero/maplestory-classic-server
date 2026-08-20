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
/// raw  4                    0x1408dcd11   <- a TAIL JMP, missed on the first pass
/// --- back in FUN_142784970 ---
/// u8   flags                0x142784a5b   <- A BITMASK. bit 1 is the balloon.
/// u8                        0x142784a69
/// u8                        0x142784a75
/// --- the trailing object, FUN_1408da090 ---
/// raw  4   MUST NOT BE 1    0x1408da0b6, then `CMP r8d,1 / JNE` exits
/// ```
///
/// **47 bytes plus the message.** See [`user_chat`].
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

/// The speaker object at `0x142784a53`: four strings and **25** raw bytes, none optional.
///
/// # It was 21 for one run, and the client died twice
///
/// `FUN_1408dcb80` does not `ret`. Its last instruction is `jmp 0x1406e9170` at
/// `0x1408dcd11` - a **tail call into `read_raw`**, with `r8d = 4` set at `0x1408dccee`. A
/// walker that only looks at `call` sees nine reads where there are ten, and the packet goes
/// out four bytes short.
///
/// **`tools/reads.py` was fixed to count tail jumps hours before this shipped, and this
/// analysis was never re-run against the fixed tool.** I told four agents to re-run anything
/// resting on a read count and did not do it here. The first crash was `0xE06D7363`, an
/// unhandled C++ exception from a body 23 bytes short; the second was `0xC0000005` from a
/// body 4 bytes short, and its stack named `0x1408da0bb` - `0x1408da0b6 + 5`, the trailing
/// object's first read, reached with the buffer already empty.
///
/// The two crashes are the same bug found twice, and the second one was avoidable.
///
/// **All empty and all zero.** What the fields mean is not established - the strings land in
/// out-pointers at `obj+8`, `+0x10`, `+0x38`, `+0x40` and the raws at `+0x18`, `+0x1c`,
/// `+0x20`, `+0x24`, `+0x28`, `+0x30` - and the shortest legal encoding is what keeps the
/// client on the path this was measured on. The one thing worth trying later is the
/// speaker's **name** in the first string; it is left empty because a name in a field that
/// turns out to be a title or a medal would render as one.
const SPEAKER_OBJECT_LEN: usize = 2 + 2 + 4 + 4 + 1 + 4 + 4 + 2 + 4 + 2 + 4;

/// The trailing object at `0x142784b05`, in its four-byte early-exit form.
///
/// `FUN_1408da090` reads a 4-byte block and then `CMP r8d,1 / JNE 0x1408da1eb` returns.
/// **Any value except 1 stops it there**, so zero costs four bytes and nothing else. With a
/// 1 it would go on to read a `u8`, a sub-object and a string. **[L]**
///
/// The handler branches on the same value again at `0x142785064`
/// (`cmp dword [r14+0x10], 1 / je`), but only **inside** the [`CHAT_FLAG_CHAT_WINDOW`]
/// block, so with that bit clear the value is read and never looked at.
const TRAILING_OBJECT_LEN: usize = 4;

/// What [`user_chat`] adds beyond the message text itself.
pub const USER_CHAT_OVERHEAD: usize =
    4 + 1 + 2 + SPEAKER_OBJECT_LEN + 3 + TRAILING_OBJECT_LEN;

/// Byte offset of the flag byte inside a [`user_chat`] body, given a message of `n` bytes.
///
/// `4` id + `1` leading flag + `2` length + `n` text + the speaker object.
pub const fn chat_flags_offset(text_len: usize) -> usize {
    4 + 1 + 2 + text_len + SPEAKER_OBJECT_LEN
}

/// The flag byte read at `0x142784a5b` - the first `u8` **after** the speaker object.
///
/// **It is a bitmask, and it was going out as `0`.** `FUN_142784970` stores it in
/// `[rbp+0x168]` (rbp is `lea`'d once at `0x142784980` and never written again, so the slot
/// is stable) and tests three separate bits of it:
///
/// | test | at | what it gates |
/// |---|---|---|
/// | `test al, 1` | `0x142784ece` | a chat post through `FUN_1415a8b80`; clear -> `je 0x142785537` |
/// | `test byte [rbp+0x168], 2` | `0x142785722` | **the balloon**; clear -> `je 0x142785b96`, past *both* `FUN_14158f5c0` call sites (`0x142785927` and `0x142785ac1`) |
/// | `test al, 4` | `0x142785537` | a post through `FUN_1415ed1c0(.., 0x1f)` |
///
/// All **[L]**, off `tools/listing.py 0x142784970`.
///
/// The sibling handler for `0x0226` does the same thing with the same field: in
/// `FUN_1427834b0` the byte arrives as the 5th argument (`[rbp+0x100]`, and
/// `FUN_1427847a0` passes its own first post-speaker `u8` there at `0x1427848d0`), and it is
/// tested `test al,1` at `0x142783aa0`, `test byte [rbp+0x100],2` at `0x142783dcf` - the
/// balloon - and `test al,4` at `0x142783af1`. Two handlers, one convention. **[L]**
pub const CHAT_FLAG_CHAT_WINDOW: u8 = 0x01;

/// Bit 1 of the flag byte: **draw the balloon over the character's head.**
///
/// See [`CHAT_FLAG_CHAT_WINDOW`] for the listing. With this bit clear the handler jumps
/// from `0x142785729` to `0x142785b96` and no `FUN_14158f5c0` call is reachable, so a
/// balloon is impossible however correct the rest of the body is.
pub const CHAT_FLAG_BALLOON: u8 = 0x02;

/// Bit 2 of the flag byte: a third posting route, `FUN_1415ed1c0(.., 0x1f)`.
///
/// What it renders is **not established** - only that the bit selects it. Not sent.
pub const CHAT_FLAG_BIT2: u8 = 0x04;

/// What the client itself puts in the equivalent byte of its **outbound** `0x00E7`.
///
/// `FUN_1418cd030` builds `0x00E7` as `u32 tick, str text, u8` and the `u8` is the
/// immediate `3`: `mov dl, 3` at `0x1418cd247`, then `call 0x1406ed840` (encode `u8`) at
/// `0x1418cd24e`. **[L]** The second `0x00E7` builder, at `0x141824a26` inside
/// `FUN_141824980`, encodes its `u8` from an argument (`movzx edx, sil` at `0x141824a4f`)
/// rather than a constant - which is what says the field is a flag byte and not a literal.
///
/// Every capture agrees: the three `0x00E7` bodies the owner sent on 2026-08-19 and the `!map 40`
/// of 2026-08-20 all end in `03`.
///
/// That the **inbound** byte uses the same encoding as the outbound one is **[D]**, not
/// **[L]** - nothing has read a server-shaped `0x0231` into this client. What is measured is
/// that inbound bit 1 is the balloon and that the client's own outbound constant has bit 1
/// set.
pub const CHAT_FLAGS_CLIENT_DEFAULT: u8 = CHAT_FLAG_CHAT_WINDOW | CHAT_FLAG_BALLOON;

/// Build a [`USER_CHAT`] with the flag byte the client's own sender uses.
///
/// **This still only renders if the handler runs at all.** `FUN_142784970` is reached
/// through `CField::OnPacket FUN_141820080` -> `FUN_1429b9300` -> `FUN_1429bafb0`, and that
/// last one drops `0x0231` in silence when `CUserPool::GetUser` returns null - which it does
/// both when the pool singleton `[0x143AC1B90]` is null (`0x1429b6ca7`) and when no `CUser`
/// carries the id we sent. See `research/user-chat-round2.md` for the watch that settles it.
pub fn user_chat(character_id: u32, text: &str) -> Vec<u8> {
    user_chat_with_flags(character_id, text, CHAT_FLAGS_CLIENT_DEFAULT)
}

/// Build a [`USER_CHAT`] with an explicit flag byte - see [`CHAT_FLAG_BALLOON`].
///
/// The knob exists because the flag is the one field in this body whose *value* changes what
/// appears on screen, and because a run costs the owner a manual launch: bisecting it should not
/// mean editing this file.
pub fn user_chat_with_flags(character_id: u32, text: &str, flags: u8) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character_id);
    w.u8(0); // 0x142784998 - the leading flag; only reaches `FUN_1415ed1c0`'s 0/0xa argument
    w.str(text);

    // The speaker object. Four empty strings and 25 zero bytes, in this exact order - see
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
    w.zeros(4); // 0x1408dcd11 - the tail jmp; see the note on SPEAKER_OBJECT_LEN

    w.u8(flags); // 0x142784a5b - the balloon lives in bit 1 of this byte
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
            "47 + the message; 20 and 43 are the two versions that killed the client"
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
    /// The tail-jmp read at `0x1408dcd11` is in this number. It was missing twice.
    #[test]
    fn the_speaker_object_is_the_size_the_listing_adds_up_to() {
        assert_eq!(SPEAKER_OBJECT_LEN, 33, "29 omits the tail-jmp raw 4 at 0x1408dcd11");
        assert_eq!(USER_CHAT_OVERHEAD, 47);
        let short = user_chat(1, "");
        let long = user_chat(1, "abcd");
        assert_eq!(long.len() - short.len(), 4, "only the message may vary");
    }

    /// The whole body, byte for byte, with the flag byte pinned at its exact offset.
    ///
    /// This is the test the previous two crashes did not have. It is not a length check:
    /// it names every byte, so a field that moves shows up as a diff rather than as a
    /// still-passing total.
    #[test]
    fn the_body_is_pinned_byte_for_byte() {
        let b = user_chat_with_flags(204, "hi", CHAT_FLAGS_CLIENT_DEFAULT);
        let want: Vec<u8> = [
            &[0xCC, 0x00, 0x00, 0x00][..], // u32 characterId = 204, read by the dispatcher
            &[0x00][..],                   // 0x142784998
            &[0x02, 0x00][..],             // the message length
            b"hi",                         // 0x1427849ac
            &[0x00, 0x00][..],             // 0x1408d6782  empty str
            &[0x00, 0x00][..],             // 0x1408dcba3  empty str
            &[0x00; 4][..],                // 0x1408dcbee
            &[0x00; 4][..],                // 0x1408dcbff
            &[0x00; 1][..],                // 0x1408dcc12
            &[0x00; 4][..],                // 0x1408dcc25
            &[0x00; 4][..],                // 0x1408dcc38
            &[0x00, 0x00][..],             // 0x1408dcc45  empty str
            &[0x00; 4][..],                // 0x1408dcc9a
            &[0x00, 0x00][..],             // 0x1408dcca7  empty str
            &[0x00; 4][..],                // 0x1408dcd11  the TAIL JMP read
            &[0x03][..],                   // 0x142784a5b  THE FLAG BYTE
            &[0x00][..],                   // 0x142784a69
            &[0x00][..],                   // 0x142784a75
            &[0x00; 4][..],                // 0x1408da090, stopped because it is not 1
        ]
        .concat();
        assert_eq!(b, want);
        assert_eq!(b.len(), USER_CHAT_OVERHEAD + 2);
    }

    /// The flag byte sits where `chat_flags_offset` says, whatever the message length.
    ///
    /// `0x142784a5b` reads it straight after the speaker object, so its offset moves with
    /// the text and with nothing else.
    #[test]
    fn the_flag_byte_is_where_the_offset_helper_says() {
        for text in ["", "hi", "Hello David", &"x".repeat(300)] {
            let b = user_chat_with_flags(7, text, 0x2A);
            let at = chat_flags_offset(text.len());
            assert_eq!(b[at], 0x2A, "flag byte for {:?} landed elsewhere", text.len());
            assert_eq!(at, b.len() - 3 - 4, "flag, two more u8s, then the trailing object");
        }
    }

    /// **The balloon bit must be set by default.**
    ///
    /// This is the whole finding of `research/user-chat-round2.md`: the byte went out as
    /// `0`, and `0x142785722` (`test byte [rbp+0x168], 2 / je 0x142785b96`) jumps past both
    /// `FUN_14158f5c0` call sites when bit 1 is clear. A zero here cannot draw a balloon no
    /// matter what else is right, so a regression to `0` is worth failing the build over.
    #[test]
    fn the_default_flags_ask_for_the_balloon() {
        assert_eq!(CHAT_FLAG_BALLOON, 0x02);
        assert_eq!(CHAT_FLAG_CHAT_WINDOW, 0x01);
        assert_eq!(CHAT_FLAG_BIT2, 0x04);
        assert_eq!(CHAT_FLAGS_CLIENT_DEFAULT, 0x03, "the client's own 0x00E7 constant");
        let b = user_chat(204, "Hello");
        let flags = b[chat_flags_offset("Hello".len())];
        assert_ne!(flags & CHAT_FLAG_BALLOON, 0, "no balloon bit, no balloon");
    }

    /// Changing the flag byte must not change the length - it is one byte, not a section.
    #[test]
    fn the_flag_byte_does_not_resize_the_body() {
        let a = user_chat_with_flags(204, "same", 0x00);
        let c = user_chat_with_flags(204, "same", 0xFF);
        assert_eq!(a.len(), c.len());
        assert_eq!(a.len(), USER_CHAT_OVERHEAD + 4);
        let at = chat_flags_offset(4);
        assert_eq!((a[at], c[at]), (0x00, 0xFF));
        assert_eq!(a[..at], c[..at]);
        assert_eq!(a[at + 1..], c[at + 1..]);
    }
}
