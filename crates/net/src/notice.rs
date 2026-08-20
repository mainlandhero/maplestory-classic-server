//! Talking back to the player: a chat line, and the answer to Log Out.
//!
//! Full working: `research/talking-back.md`.

use crate::PacketWriter;

/// A line in the client's chat window.
///
/// ```text
/// u8   force
/// str  text     u16 length, then bytes
/// ```
///
/// **Always send `force = 1`.** With `0`, only the *first* line after each field entry is
/// shown: `world->[0x28c8]` latches after a post and is reset by `FUN_142caa4e0`, the
/// routine that already runs on field entry. Empty text is a silent no-op. **[L]**
///
/// **How this was found rather than guessed.** The printer is
/// `FUN_1415eca30(char**, u16 type)` with 1133 call sites, and **type 7 is the chat
/// window** - the control being `FUN_14209ee50`, which posts string id `0x533` with type 7,
/// and that string decrypts to `[Welcome] Welcome to MapleStory!!`, the line already visible
/// in every capture. A `rel32` sweep of all 1133 sites gives 17 with type 7, and `0x00BB` is
/// the only minimal one. **The 17 is a lower bound**: 167 sites load the type from a
/// register, so the sweep cannot see them.
///
/// Colour and tab are **not** controllable through this packet, and what they render as was
/// not settled.
pub const CHAT_NOTICE: u16 = 0x00BB;

/// Build a [`CHAT_NOTICE`].
pub fn chat_notice(text: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(1); // force - see CHAT_NOTICE; 0 shows only the first line per field entry
    w.str(text);
    w.into_vec()
}

/// The answer to the client's Log Out request, `0x01BE`.
///
/// Body is **one non-empty string**; an empty one is a no-op. Verified three ways that
/// agree - the jump table at `0x142cbd9d0` (index `opcode - 0x70`), the listing, and the
/// decompiler - all showing exactly one read. **[L]**
///
/// It runs `FUN_142d3c670`, which outside the login subsystem's own startup is **the only
/// function in the image that constructs a login stage** - proved through three vtable
/// pointers whose slots all resolve to the login-stage `OnPacket`, `FUN_141B25F30`.
///
/// **The teardown is in place, not a reconnect.** `FUN_142d3c670` builds no `sockaddr`,
/// the connect helper's two callers are both unreachable from it, and a whole-image `htons`
/// scan finds nine sites, none in any channel handler. What is *not* settled is whether the
/// client closes the channel socket - the deciding function jumps into the Themida region.
/// A run answers it for free: send this, then see whether the next packet lands in
/// `login.log` or `world.log`.
pub const LOG_OUT_RESULT: u16 = 0x0106;

/// The client requesting Log Out. **Empty body**, and answering it is not optional.
///
/// **`0x01BE` poisons `SetField` until it is answered.** Its builder sets
/// `world->[0x33f4] = 1`, and `FUN_142097f80` - the `0x01A0` handler - tests that byte right
/// after reading its 8-byte FILETIME and returns to its epilogue if it is set. **Only
/// `0x0106` clears it.** So from the moment a player clicks Log Out, every `SetField` is
/// dropped in silence: no dialog, no fault, nothing in any log. **[L]**
///
/// That is the same failure class as the `player->[0x2330]` latch in `research/npc-click.md`
/// but with a far worse blast radius, and it has a consequence for testing:
/// **any observation made after a Log Out click, in the same session, is invalid.**
///
/// Inbound `0x0137` (empty body) makes the client *send* `0x01BE` and set the same latch, so
/// never send that without following it with `0x0106`.
pub const CLIENT_LOG_OUT: u16 = 0x01BE;

/// Build a [`LOG_OUT_RESULT`]. The string must not be empty.
pub fn log_out_result(message: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.str(if message.is_empty() { " " } else { message });
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_chat_notice_forces_itself_onto_the_screen() {
        let b = chat_notice("no such map");
        // force = 1. A zero here shows the line once per field entry and then never again,
        // which reads exactly like the feature being broken.
        assert_eq!(b[0], 1);
        assert_eq!(u16::from_le_bytes([b[1], b[2]]) as usize, "no such map".len());
        assert_eq!(&b[3..], b"no such map");
    }

    /// An empty log-out message is a no-op in the client, so it must never reach the wire.
    #[test]
    fn the_log_out_answer_is_never_an_empty_string() {
        for message in ["", "Returning to the login screen."] {
            let b = log_out_result(message);
            let len = u16::from_le_bytes([b[0], b[1]]) as usize;
            assert!(len > 0, "an empty string makes the client do nothing at all");
            assert_eq!(b.len(), 2 + len);
        }
    }
}
