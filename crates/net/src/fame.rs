//! **Fame**: the up and down arrows on another player's Character Info - `0x0144` in,
//! `0x0087` out.
//!
//! The owner, 2026-09-18: *"I just opened Tester2 Character info and attempted to fame them."*
//! `world-ch0.log` 18:12:49: `<- 0x0144, 5 byte body d6000000 01`, unanswered.
//!
//! The reply opcode was found without a guess and without Ghidra (the project was held):
//! `tools/dump_stringids.py` names the fame messages (`0x00FA` *"raised '%s's fame"*,
//! `0x00FB` *"dropped"*, `0x00FC`, `0x0100`..`0x0105`), a scan of the channel-stage handlers
//! for those immediates hits exactly one function, `FUN_142d58c60`, and
//! `research/msexe-gamestage-cases.txt` has it as case **`0x0087`**. Its read walk is
//! `u8 mode; [str, u8, u32]; [str, u8]` and its jump table (`0x142d58ff0`, six entries)
//! maps each mode to one message. **[L]** for every field and mode below.
//!
//! Neither packet touches the `[world+0x2330]` request latch: `0x0144` is not in
//! `dropmoney::LATCHING_REQUESTS`, so a refusal costs nothing but the message.

use crate::packet::{PacketReader, PacketWriter};

/// The client giving fame. `u32 targetCharacterId, u8 raise` (1 = the up arrow, 0 = down),
/// builder `FUN_142d4cd50` (`research/msexe-packet-fields.txt`). The client asks *"Do you
/// want to increase %s's Fame?"* (`0x00FE`) before sending, and refuses to send for yourself
/// (`0x00FD`).
pub const CLIENT_GIVE_FAME: u16 = 0x0144;

/// The result, `u8 mode` first. Modes, from the handler's jump table:
///
/// | mode | body after it | the client draws |
/// |---|---|---|
/// | 0 | `str targetName, u8 raise, u32 newFame` | `0x00FA` / `0x00FB`: *"raised / dropped '%s''s level of fame"*, and the open Character Info window's FAME line is set to `newFame` |
/// | 1 | - | `0x00FC` *"The user name is incorrectly entered."* |
/// | 2 | - | `0x0100` *"Users under level %d are unable to raise or lower fame."* with `%d` = 10 (an immediate in the handler) |
/// | 3 | - | `0x0101` *"You can't raise or drop a level of fame anymore for today."* |
/// | 4 | - | `0x0102` *"You can't raise or drop a level of fame of that character anymore for this month."* - the client's text says month; the server's rule is a week |
/// | 5 | `str giverName, u8 raise` | `0x0103` / `0x0104`: *"'%s' has raised / dropped '%s''s level of fame."* - sent to the TARGET |
/// | >5 | - | `0x0105` *"...neither been raised or dropped due to an unexpected error."* |
pub const GIVE_FAME_RESULT: u16 = 0x0087;

pub mod result {
    pub const GIVEN: u8 = 0;
    pub const NO_SUCH_USER: u8 = 1;
    pub const LEVEL_TOO_LOW: u8 = 2;
    pub const ALREADY_TODAY: u8 = 3;
    pub const SAME_TARGET_THIS_WEEK: u8 = 4;
    pub const RECEIVED: u8 = 5;
}

/// A parsed [`CLIENT_GIVE_FAME`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GiveFameRequest {
    pub target: u32,
    /// `true` for the up arrow.
    pub raise: bool,
}

pub fn parse_give_fame(body: &[u8]) -> Option<GiveFameRequest> {
    let mut r = PacketReader::new(body);
    let target = r.u32().ok()?;
    let raise = r.u8().ok()? != 0;
    Some(GiveFameRequest { target, raise })
}

/// The body a builder writes, for tests.
pub fn give_fame_request(target: u32, raise: bool) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(target);
    w.u8(u8::from(raise));
    w.into_vec()
}

/// Mode 0, to the giver: it went through, and the window shows `fame` now.
pub fn fame_given(target_name: &str, raise: bool, fame: i32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(result::GIVEN);
    w.str(target_name);
    w.u8(u8::from(raise));
    w.i32(fame);
    w.into_vec()
}

/// Modes 1..4 (and anything else the client reads as "unexpected error"): one byte.
pub fn fame_refused(mode: u8) -> Vec<u8> {
    vec![mode]
}

/// Mode 5, to the target: `giver` raised or dropped your fame.
pub fn fame_received(giver_name: &str, raise: bool) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(result::RECEIVED);
    w.str(giver_name);
    w.u8(u8::from(raise));
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The owner's click, byte for byte: `d6000000 01` is Tester2 (214), up.
    #[test]
    fn the_click_from_the_log_parses() {
        let req = parse_give_fame(&[0xd6, 0, 0, 0, 1]).unwrap();
        assert_eq!(req, GiveFameRequest { target: 214, raise: true });
        assert_eq!(parse_give_fame(&give_fame_request(214, false)).unwrap().raise, false);
        assert!(parse_give_fame(&[0xd6, 0, 0, 0]).is_none(), "short");
    }

    /// The three bodies against the handler's read order: `u8 mode; [str, u8, u32]; [str, u8]`.
    #[test]
    fn the_result_bodies_follow_the_handlers_reads() {
        let b = fame_given("Tester2", true, 1);
        assert_eq!(b[0], result::GIVEN);
        assert_eq!(&b[1..3], &7u16.to_le_bytes(), "str = u16 length");
        assert_eq!(&b[3..10], b"Tester2");
        assert_eq!(b[10], 1, "raise");
        assert_eq!(&b[11..15], &1i32.to_le_bytes());
        assert_eq!(b.len(), 15);
        assert_eq!(&fame_given("X", false, -3)[5..], &(-3i32).to_le_bytes(), "a defame below zero is signed");
        assert_eq!(fame_refused(result::ALREADY_TODAY), vec![3]);
        assert_eq!(fame_refused(result::SAME_TARGET_THIS_WEEK), vec![4]);
        let b = fame_received("Wisp", false);
        assert_eq!(b, vec![5, 4, 0, b'W', b'i', b's', b'p', 0]);
    }
}
