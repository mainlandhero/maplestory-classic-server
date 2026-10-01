//! The scrolling banner across the top of the screen, and the rest of `0x00AC`.
//!
//! Full working: `research/broadcast-banner.md`.
//!
//! # How the opcode was found, because "BroadcastMsg" in a table is not evidence
//!
//! `research/msexe-gamestage-opcodes.md` is an order-preserving alignment of mscw's 273
//! channel opcodes against two *other versions'* enums, anchored on one confirmed name. It
//! offers **two** candidates called `BroadcastMsg`: `0x00AB` on the newer alignment and
//! `0x00B5` on the older one. Both are wrong, and decompiling them says so out loud:
//!
//! * `0x00B5` (`FUN_142d92e30`, 33 bytes) reads one `u32` and makes two calls.
//! * `0x00AB` (`FUN_142d60c70`, 192 bytes) reads `u32, u32` and, when **neither is
//!   `999999999`**, a `u32` and two `u16`. That sentinel is the town-portal "no portal
//!   placed" marker, so mscw's `0x00AB` is **TownPortal** - the entry the newer alignment
//!   puts one slot *earlier*. The alignment is off by one here, which is exactly the decay
//!   that file warns about.
//!
//! One slot further on is `0x00AC`, `FUN_142d60d40`, **8292 bytes** - and its handlers are
//! adjacent in the image to the town-portal one, the way the enum entries are adjacent.
//! That function is this packet.
//!
//! # The body, from the listing and the decompiler, which agree
//!
//! ```text
//! u8  type              always read, at 142d60d72
//! u8  flag              types 4 and 26 ONLY, at 142d60e48
//! str message           unless the type is 13, 14, 27 or 29 - and for types 4 and 26,
//!                       only when `flag` is non-zero.                    142d60e64
//! str                   a second string, types 28 and 30 only            142d60ebd
//! ```
//!
//! The type is bounded at `0x1e`; anything above it falls to the same do-nothing target as
//! the unhandled cases. The "no string" set is a **bit mask**, not a comparison chain:
//! `(type < 0x1e) && ((0x28006000 >> type) & 1)`, and `0x28006000` has bits 13, 14, 27 and
//! 29 set. Reading that as a range would have been wrong in the quiet way this project keeps
//! paying for.
//!
//! `tools/reads.py 0x142d60d40 1` lists the same four reads at the same four addresses, in
//! the same order, so the field order here rests on two instruments rather than one.
//!
//! # Type 4 is the banner, and it owns a singleton the other types do not
//!
//! Most types end in the chat printer `FUN_1415eca30(text, kind)` that
//! `crate::notice::CHAT_NOTICE` already documents - type 0 posts with kind 9, type 2 with
//! kind 7, type 5 with kind 0xb. **Type 4 touches none of it.** It works on one global
//! object, `DAT_143acd9a0`, with a create-and-destroy lifecycle:
//!
//! ```text
//! case 4:
//!   if (DAT_143acd9a0) { FUN_142bf3f70(...); FUN_1422a00c0(...); }   // always reset first
//!   if (flag == 0 || message is null or empty) {
//!       if (DAT_143acd9a0) { FUN_142bf3f70(...); ... }               // tear it down
//!   } else {
//!       if (DAT_143acd9a0 == 0) FUN_142db92d0();                     // create it
//!       FUN_14229c3d0(DAT_143acd9a0, message, 0);                    // hand it the text
//!   }
//! ```
//!
//! A persistent singleton with a text setter and an explicit teardown is a **banner**, not a
//! chat line: a chat line needs neither. **[L]** for all of that - it is read straight out of
//! the function.
//!
//! **[I]** that the banner *scrolls*, and that it sits at the top. That comes from the
//! packet's shape matching the same family's scrolling-header type in other versions, and
//! from the owner's description of the feature they asked for. A run settles it in one line of chat,
//! and if it turns out to be a static banner or a popup, only this paragraph is wrong.
//!
//! # Re-sending the same text restarts the animation
//!
//! The `case 4` prologue resets the object **before** it looks at the flag, so sending the
//! banner again is not idempotent on screen even when the string is identical. Callers must
//! remember what they last sent and stay quiet - see `world::session::rates`.

use crate::PacketWriter;

/// The client's broadcast-message packet. See the module docs for how the number was found.
pub const BROADCAST_MSG: u16 = 0x00AC;

/// The type byte that drives the scrolling banner.
pub const BANNER: u8 = 4;

/// **Type 5: one line in the chat log, in the client's own system colour.**
///
/// The owner, 2026-09-22, about *"Tester2 is now your friend."*: *"if it has to be a chat message
/// we send, can we send it as a red system message?"*
///
/// It can, and the colour is not chosen - it is inherited. `case 5` of `FUN_142d60d40` is two
/// instructions long:
///
/// ```text
/// case 5:
///   FUN_1415eca30(&text, 0xb);
///   break;
/// ```
///
/// `FUN_1415eca30(char**, u16 kind)` is the chat printer [`crate::notice::CHAT_NOTICE`]
/// documents, and **kind `0xb` is the kind the client's own friend sentences use** - `0x00A7`
/// sub-op `0x32` *"%s has declined the friend request."* ends in `FUN_1415eca30(.., 0xb)`, and
/// that line is on the owner's screen in the colour they are asking for. So a line sent this way is
/// drawn by the same printer, with the same kind, as the client's own.
///
/// Both **[L]**, out of `research/msexe-broadcast-0xac.txt` and
/// `research/msexe-friendpopup-0x1a.c`. What is **[I]** is the word "red": nobody has measured
/// the RGB. What is measured is *"the same as the decline line"*, which is the thing that was
/// actually asked for.
///
/// **No flag byte.** The `u8` after the type is read for types 4 and 26 only; every other type
/// reads the string straight after the type, so a flag here would be eaten as the string's
/// length.
pub const SYSTEM_LINE: u8 = 5;

/// **Type 0: the blue `[Notice]` line in the chat log.** `case 0` of `FUN_142d60d40` formats
/// the text after string `0x536`, **`[Notice]`**, and posts it with `FUN_1415eca30(.., 9)` -
/// chat kind **9**, whose colour constant is **`0xFF60CEFF`**, light blue
/// (`research/message-subcases.md` §3's table). **[L]** for the read, the prefix and the kind;
/// what it looks like on screen is unmeasured. No flag byte - types 4 and 26 only.
pub const NOTICE: u8 = 0;

/// A blue `[Notice]` line - see [`NOTICE`].
pub fn notice(text: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(NOTICE);
    w.str(text);
    w.into_vec()
}

/// Show `text` in the banner.
///
/// An empty string is sent as [`clear_banner`] instead, because that is what the client does
/// with it anyway - `case 4` tests `*message == '\0'` and takes the teardown branch.
pub fn banner(text: &str) -> Vec<u8> {
    if text.is_empty() {
        return clear_banner();
    }
    let mut w = PacketWriter::new();
    w.u8(BANNER);
    w.u8(1); // flag: there is a message after this
    w.str(text);
    w.into_vec()
}

/// Take the banner off the screen.
///
/// Two bytes and **no string**: with the flag at 0 the client never reads one, so appending
/// an empty string here would leave two bytes in the buffer that nothing consumes.
pub fn clear_banner() -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(BANNER);
    w.u8(0);
    w.into_vec()
}

/// One line in the chat log, in the client's own system colour - see [`SYSTEM_LINE`].
///
/// The text is ASCII-folded for the reason [`crate::notice::chat_notice`] gives: this client's
/// chat font has no glyph at `0xDC`, and an accented name came back as a box on screen.
///
/// ```
/// use net::broadcast::{system_line, SYSTEM_LINE};
/// let b = system_line("Tester2 is now your friend.");
/// assert_eq!(b[0], SYSTEM_LINE);
/// assert_eq!(u16::from_le_bytes([b[1], b[2]]), 27);
/// assert_eq!(&b[3..], b"Tester2 is now your friend.");
/// ```
pub fn system_line(text: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(SYSTEM_LINE);
    w.str(&crate::notice::ascii_fold(text));
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_is_type_flag_string() {
        let b = banner("hello");
        assert_eq!(b[0], BANNER);
        assert_eq!(b[1], 1);
        assert_eq!(u16::from_le_bytes([b[2], b[3]]), 5);
        assert_eq!(&b[4..], b"hello");
        assert_eq!(b.len(), 2 + 2 + 5);
    }

    #[test]
    fn clearing_sends_no_string() {
        // The client reads the flag and stops. A third and fourth byte would be left in the
        // buffer for whatever the framer does next.
        assert_eq!(clear_banner(), vec![BANNER, 0]);
    }

    #[test]
    fn an_empty_message_is_a_clear() {
        assert_eq!(banner(""), clear_banner());
    }

    /// **Type 5 has no flag byte**, unlike the banner - the client reads the string straight
    /// after the type, so a flag would be eaten as its length prefix.
    #[test]
    fn a_system_line_is_type_then_string_with_no_flag() {
        let b = system_line("hi");
        assert_eq!(b, vec![SYSTEM_LINE, 2, 0, b'h', b'i']);
        // Folded, because the chat font has no glyph for the raw byte.
        assert_eq!(&system_line("\u{00dc}bel")[3..], b"Ubel");
    }
}
