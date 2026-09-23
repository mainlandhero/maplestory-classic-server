//! Whispers: `0x017B` in, `0x01B3` out.
//!
//! The owner, 2026-09-15: *"I just tried sending Tester2 a whisper message, can you see if the
//! opcode was captured and implement it please?"* It was - `world-ch0.log` 00:29:35:
//!
//! ```text
//! 0x017B  06 9465a11a 0700 "Tester2" 0d00 "Hello Whisper"
//!         ^  ^        ^             ^
//!         |  |        |             u16 length + the text
//!         |  |        u16 length + the target's name
//!         |  u32 tick
//!         u8 kind: 6 = whisper. The reference's handler names 5 (and 0x44) as /find, and
//!            for those the text is absent [R]; only 6 has been captured here.
//! ```
//!
//! # The reply: `CField::OnPacket` case `0x01B3` -> `FUN_1418486b0`
//!
//! `research/msexe-field-cases.txt` puts `0x01B3` two after the group message, exactly where
//! the reference puts `WHISPER(739)` after `GROUP_MESSAGE(737)`. The handler reads a mode
//! byte at `141848708` and jumps through a table indexed by `mode - 9` (`0x141a4a3e0`,
//! read out of the PE). The arms that matter, and every read in them **[L]**:
//!
//! ```text
//! 0x12  14184873d   a whisper ARRIVES
//!       141848740  u32   (0)
//!       141848752  str   the sender's name
//!       14184875b  u32   the sender's character id
//!       141848763  u8    the sender's channel
//!       141848771  u8    (0)
//!       141848783  str   the text
//!       1418488c4  FUN_1408d6760: the same chat-info block a group line carries (name,
//!                  text, account id, character id, world, character id, 0, "", 0, "", 0)
//!       14184898e  FUN_1408da090: raw4 hasItem; only a 1 reads on (u8, an item, a str)
//! 0x0A  14184929e   the SENDER's result
//!       1418492a1  u8    (a flag; 0)
//!       1418492b3  str   the target's name
//!       1418492bc  u8    found: non-zero draws string 0x061A '%s<< %s' - the echo of what
//!                  was sent - and zero draws 0x0087 'Could not find %s.'
//! 0x09  141849827   /find's answer: str, u8, u32 - the u8's meaning is not read here, so
//!                  this server answers /find with a chat line instead (session/whisper.rs)
//! ```
//!
//! The reference's `whisper()` writes a `u32 world` between the two bytes and the text;
//! this client reads none, and its `encodeByte(0)` after `hasItem` is not read either. The
//! listing wins.
//!
//! **Nothing here has been on a screen yet.** Every offset is the listing's.
//!
//! # Nothing here authenticates
//!
//! As everywhere in this project, the channel socket carries no credentials.

use crate::packet::{PacketReader, PacketWriter};

/// `0x017B` - the client whispers, or asks where someone is.
pub const CLIENT_WHISPER: u16 = 0x017B;

/// `0x01B3` - a whisper arrives, or the sender is told how theirs went.
pub const WHISPER: u16 = 0x01B3;

/// The request's kind byte.
pub mod kind {
    /// `/find name` **[R]** - not captured here.
    pub const FIND: u8 = 5;
    /// A whisper. Captured.
    pub const WHISPER: u8 = 6;
    /// `/find` from the buddy or party window **[R]**.
    pub const FIND_ALT: u8 = 0x44;
}

/// The reply's mode byte.
pub mod mode {
    /// **Where somebody is**: `str name, u8 place, u32 value`. See [`whisper_found`].
    pub const FOUND: u8 = 0x09;
    /// The sender's result: `u8 0, str target, u8 found`.
    pub const SENT: u8 = 0x0A;
    /// A whisper arrives - see the module docs for the body.
    pub const RECEIVE: u8 = 0x12;
}

/// The `u8` in a [`mode::FOUND`] reply: what `value` means, and what the client writes.
///
/// Decoded 2026-09-22 off `FUN_1418486b0` case 9, whose four arms each resolve a different
/// kind of place and then format it with string `0x03EB`, **`"%s - %s"`** - which is exactly
/// the shape of *"the owner - Checking location"* in the buddy window. **[L]**
pub mod place {
    /// `value` is a **map id**; the client looks its street and map name up itself. **[L]**
    /// (`FUN_141815360`, then the `streetName` property.)
    pub const MAP: u8 = 1;
    /// String `0x0DE2` *"Cash Shop"*; `value` unused. **[L]**
    pub const CASH_SHOP: u8 = 2;
    /// `value` is a **channel**, resolved by `FUN_142cb92f0`. **[L]**
    pub const CHANNEL: u8 = 3;
    /// String `0x0DE3` *"Maple Auction"*. **[L]**
    pub const MAPLE_AUCTION: u8 = 5;
    /// **No place.** Not an arm of the switch, so the client formats an empty second half
    /// rather than drawing anything - which is how this server answers a find for somebody
    /// who is not online without putting a line in the chat log. **[I]** on what that looks
    /// like on screen; it is deliberate, and plan step 12 asks.
    pub const NOWHERE: u8 = 0;
}

/// A parsed [`CLIENT_WHISPER`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhisperRequest {
    pub kind: u8,
    pub tick: u32,
    pub target: String,
    /// Present for [`kind::WHISPER`]; empty for a find.
    pub text: String,
}

/// Parse the body after the opcode.
pub fn parse_whisper(body: &[u8]) -> Option<WhisperRequest> {
    let mut r = PacketReader::new(body);
    let kind = r.u8().ok()?;
    let tick = r.u32().ok()?;
    let target = r.str().ok()?;
    let text = if kind == kind::WHISPER { r.str().ok()? } else { String::new() };
    Some(WhisperRequest { kind, tick, target, text })
}

/// **A whisper arrives** at the target: mode `0x12`. `text` is folded to ASCII like every
/// chat line - the font has no glyph for an umlaut.
pub fn whisper_receive(from_name: &str, from_id: u32, from_account: u32, from_channel: u8, world_id: u8, text: &str) -> Vec<u8> {
    let text = crate::notice::ascii_fold(text);
    let mut w = PacketWriter::new();
    w.u8(mode::RECEIVE);
    w.u32(0); //                 141848740
    w.str(from_name); //         141848752
    w.u32(from_id); //           14184875b
    w.u8(from_channel); //       141848763
    w.u8(0); //                  141848771
    w.str(&text); //             141848783
    // FUN_1408d6760 -> FUN_1408dcb80: the chat-info block, every read unconditional.
    w.str(from_name); //         1408d6782
    w.str(&text); //             1408dcba3
    w.u32(from_account); //      1408dcbee
    w.u32(from_id); //           1408dcbff
    w.u8(world_id); //           1408dcc12
    w.u32(from_id); //           1408dcc25
    w.u32(0); //                 1408dcc38
    w.str(""); //                1408dcc45
    w.u32(0); //                 1408dcc9a
    w.str(""); //                1408dcca7
    w.u32(0); //                 1408dcd11
    w.u32(0); //                 1408da0b6  hasItem: 0, nothing follows
    w.into_vec()
}

/// **Where somebody is**: mode `0x09`, the answer the buddy window's location check wants.
///
/// The owner, 2026-09-22: *"It keeps saying the owner is not online on any channel when the owner is right
/// there. Also please do not send a message for that, as those information should only show
/// in the UI where 'Checking location' is."*
///
/// Both halves of that are this packet. The window sends `0x017B` kind `0x44` and waits; this
/// server used to answer with a **chat line it wrote itself**, which is why the wording was
/// wrong *and* why it appeared in the log instead of the window. Mode `0x09` is the arm that
/// fills the status line: `str name, u8 place, u32 value`, formatted with `"%s - %s"`.
///
/// ```
/// use net::whisper::{whisper_found, place};
/// let b = whisper_found("Wisp", place::MAP, 104040000);
/// assert_eq!(b[0], 0x09);
/// assert_eq!(&b[1..3], &4u16.to_le_bytes());
/// assert_eq!(&b[3..7], b"Wisp");
/// assert_eq!(b[7], place::MAP);
/// assert_eq!(&b[8..12], &104040000u32.to_le_bytes());
/// ```
pub fn whisper_found(name: &str, place: u8, value: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(mode::FOUND);
    w.str(name); //              141849833
    w.u8(place); //              14184983c
    w.u32(value); //             141849848
    w.into_vec()
}

/// **The sender's result**: mode `0x0A`. `found` draws `'%s<< %s'` (the echo) when true and
/// `'Could not find %s.'` when false.
pub fn whisper_sent(target: &str, found: bool) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(mode::SENT);
    w.u8(0); //                  1418492a1
    w.str(target); //            1418492b3
    w.bool(found); //            1418492bc
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The owner's whisper, the log's own hex.
    #[test]
    fn the_captured_whisper_parses() {
        let hex = "069465a11a0700546573746572320d0048656c6c6f2057686973706572";
        let b: Vec<u8> = (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect();
        assert_eq!(b.len(), 29, "the log said 29 bytes");
        assert_eq!(
            parse_whisper(&b),
            Some(WhisperRequest { kind: kind::WHISPER, tick: 0x1aa16594, target: "Tester2".into(), text: "Hello Whisper".into() })
        );
        assert_eq!(parse_whisper(&b[..28]), None, "a short text is refused");
        // A find carries no text and still parses.
        let mut f = vec![kind::FIND, 0, 0, 0, 0, 7, 0];
        f.extend_from_slice(b"Tester2");
        assert_eq!(parse_whisper(&f).map(|r| (r.kind, r.target, r.text)), Some((5, "Tester2".into(), String::new())));
    }

    /// Both replies, walked field by field in the handler's read order, every byte accounted
    /// for - a body one byte off is the failure `CLAUDE.md` records twice.
    #[test]
    fn both_replies_are_the_handlers_read_order_and_nothing_else() {
        let b = whisper_receive("Wisp", 215, 1, 0, 0, "Hello Whisper");
        let mut at = 0usize;
        let u8_at = |b: &[u8], at: &mut usize| { let v = b[*at]; *at += 1; v };
        let u32_at = |b: &[u8], at: &mut usize| { let v = u32::from_le_bytes(b[*at..*at + 4].try_into().unwrap()); *at += 4; v };
        let str_at = |b: &[u8], at: &mut usize| {
            let n = u16::from_le_bytes([b[*at], b[*at + 1]]) as usize;
            let s = String::from_utf8(b[*at + 2..*at + 2 + n].to_vec()).unwrap();
            *at += 2 + n;
            s
        };
        assert_eq!(u8_at(&b, &mut at), mode::RECEIVE);
        assert_eq!(u32_at(&b, &mut at), 0);
        assert_eq!(str_at(&b, &mut at), "Wisp");
        assert_eq!(u32_at(&b, &mut at), 215);
        assert_eq!(u8_at(&b, &mut at), 0, "channel");
        assert_eq!(u8_at(&b, &mut at), 0);
        assert_eq!(str_at(&b, &mut at), "Hello Whisper");
        assert_eq!(str_at(&b, &mut at), "Wisp", "chat info");
        assert_eq!(str_at(&b, &mut at), "Hello Whisper");
        assert_eq!(u32_at(&b, &mut at), 1, "account");
        assert_eq!(u32_at(&b, &mut at), 215);
        assert_eq!(u8_at(&b, &mut at), 0, "world");
        assert_eq!(u32_at(&b, &mut at), 215);
        assert_eq!(u32_at(&b, &mut at), 0);
        assert_eq!(str_at(&b, &mut at), "");
        assert_eq!(u32_at(&b, &mut at), 0);
        assert_eq!(str_at(&b, &mut at), "");
        assert_eq!(u32_at(&b, &mut at), 0);
        assert_eq!(u32_at(&b, &mut at), 0, "hasItem 0");
        assert_eq!(at, b.len(), "every byte accounted for");

        let s = whisper_sent("Tester2", true);
        assert_eq!(&s[..2], &[mode::SENT, 0]);
        assert_eq!(&s[2..4], &7u16.to_le_bytes());
        assert_eq!(&s[4..11], b"Tester2");
        assert_eq!(s[11], 1, "found -> '%s<< %s'");
        assert_eq!(s.len(), 12);
        assert_eq!(*whisper_sent("Nobody", false).last().unwrap(), 0, "not found -> 'Could not find %s.'");
    }
}
