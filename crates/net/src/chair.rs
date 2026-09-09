//! Sitting on a chair: `0x00DB` to sit, `0x00DA` to stand.
//!
//! The owner, 2026-09-08: *"my character just sat in a chair, but the idle recovery did not adjust
//! to match the chair's recovery stats."* Both opcodes were decoded from the capture of that
//! session; `research/chairs-2026-09-08.md` has the working.
//!
//! # The capture, and why the decode is checkable
//!
//! ```text
//! 02:54:41.004 <- 0x00DB  25 bytes
//! 804a5d05 d5ed2d00 04000000 00 ffffffff fd000000 2c050000
//!  tick     3010005  slot 4
//! ```
//!
//! `0x2DEDD5` is **3010005**, which `gm-handbook/items.txt` names *Red Chair* - the chair the owner
//! said they sat on - and the `4` behind it is the slot it occupies in their Set Up tab, which is
//! where the screenshot shows it. Two independent values agreeing with a screen [L].
//!
//! `0x00DA` is two bytes, `ffff`, and arrived 22 s later when they stood up. The client's builder
//! `FUN_142cd3a60` writes `Encode2(0xFFFF)` as an immediate [L], so the `u16` is a chair id
//! with `-1` meaning *none*.
//!
//! # BOTH opcodes latch, so neither may be answered with silence
//!
//! `0x00DA` and `0x00DB` are both in [`crate::dropmoney::LATCHING_REQUESTS`]. Their client-side
//! builders set the `ctx+0x2330` exclusive-request latch, and `FUN_142cd3a60` refuses to send
//! at all while it is set - `cmp dword ptr [rcx + 0x2330], 0 / jne return` [L]. So an
//! unanswered sit would make the client unable to even ask to stand, on top of freezing every
//! later inventory action. The handler must keep sending the unlock.
//!
//! # What is NOT decoded here, stated plainly
//!
//! The owner, same session: *"I also cannot get out of the chair, the server won't let me."* Theirs
//! client sent **eight** `0x00DA` requests and stayed seated, so the client leaves a chair only
//! when the server tells it to, and **the packet that does so has not been found**. This module
//! therefore tracks the state and drives recovery; it does not claim to seat or unseat anybody
//! on screen. `research/chairs-2026-09-08.md` §4 records what was eliminated.

use crate::packet::PacketReader;

/// Client -> server: "I sat on this Set Up chair." Carries the item id and inventory slot.
pub const CLIENT_CHAIR_SIT: u16 = 0x00DB;

/// Client -> server: "I want to stand up" (`u16` chair id, `0xFFFF` for none).
pub const CLIENT_CHAIR_CANCEL: u16 = 0x00DA;

/// The `u16` the client sends for *no chair*. `FUN_142cd3a60` writes it as an immediate [L].
pub const NO_CHAIR: u16 = 0xFFFF;

/// Where the item id sits in a `0x00DB` body: after the leading tick.
const SIT_ITEM_ID_AT: usize = 4;

/// A sit request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sit {
    /// The chair's item id, e.g. `3010005` for the Red Chair.
    pub item_id: u32,
    /// Its slot in the Set Up bag. Read because it is checkable against a screen, not because
    /// anything needs it yet.
    pub slot: u32,
}

/// Decode a `0x00DB` body - everything after the two opcode bytes.
///
/// The leading `u32` is a tick and the four values after the slot are not decoded: every
/// capture has had them constant and nothing here needs them. They are recorded in the
/// research file rather than guessed at here.
pub fn parse_sit(body: &[u8]) -> Option<Sit> {
    let mut c = PacketReader::new(body);
    c.skip(SIT_ITEM_ID_AT).ok()?;
    let item_id = c.u32().ok()?;
    let slot = c.u32().ok()?;
    Some(Sit { item_id, slot })
}

/// Decode a `0x00DA` body. `None` if it does not decode; `Some(None)` means *stand up*.
#[allow(clippy::option_option)]
pub fn parse_cancel(body: &[u8]) -> Option<Option<u16>> {
    let mut c = PacketReader::new(body);
    let id = c.u16().ok()?;
    Some(if id == NO_CHAIR { None } else { Some(id) })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The owner's captured sit, byte for byte.
    const SAT_ON_THE_RED_CHAIR: &[u8] = &[
        0x80, 0x4a, 0x5d, 0x05, 0xd5, 0xed, 0x2d, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0xff,
        0xff, 0xff, 0xff, 0xfd, 0x00, 0x00, 0x00, 0x2c, 0x05, 0x00, 0x00,
    ];

    #[test]
    fn the_capture_decodes_to_the_red_chair_in_the_slot_the_screenshot_shows() {
        assert_eq!(SAT_ON_THE_RED_CHAIR.len(), 25, "the captured body was 25 bytes");
        assert_eq!(
            parse_sit(SAT_ON_THE_RED_CHAIR),
            Some(Sit { item_id: 3_010_005, slot: 4 })
        );
    }

    #[test]
    fn a_truncated_sit_is_refused_rather_than_half_read() {
        for n in 0..12 {
            assert_eq!(parse_sit(&SAT_ON_THE_RED_CHAIR[..n]), None, "{n}-byte prefix");
        }
    }

    /// The captured stand-up, and the thing most likely to be got backwards: `0xFFFF` is not a
    /// chair id, it is the absence of one.
    #[test]
    fn ffff_means_stand_up_and_anything_else_is_a_chair() {
        assert_eq!(parse_cancel(&[0xff, 0xff]), Some(None));
        assert_eq!(parse_cancel(&[0x01, 0x00]), Some(Some(1)));
        assert_eq!(parse_cancel(&[0xff]), None);
        assert_eq!(parse_cancel(&[]), None);
    }

    /// Both must stay in the latching list. If either is ever removed from it, the handler
    /// stops sending the unlock and the client can no longer ask to stand at all.
    #[test]
    fn both_chair_opcodes_latch_the_exclusive_request() {
        assert!(crate::dropmoney::latches_the_exclusive_request(CLIENT_CHAIR_SIT));
        assert!(crate::dropmoney::latches_the_exclusive_request(CLIENT_CHAIR_CANCEL));
    }
}
