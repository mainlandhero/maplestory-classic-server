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
//! # The reply the client waits for
//!
//! The owner, same session: *"I also cannot get out of the chair, the server won't let me."* Theirs
//! client sent **eight** `0x00DA` requests and stayed seated. The client seats itself but will
//! not stand until the server says so, and the packet that says so is [`USER_SIT`] = `0x0318`.
//! `research/chairs-2026-09-08.md` §12.

use crate::packet::{PacketReader, PacketWriter};

/// Client -> server: "I sat on this Set Up chair." Carries the item id and inventory slot.
pub const CLIENT_CHAIR_SIT: u16 = 0x00DB;

/// Client -> server: "I want to stand up" (`u16` chair id, `0xFFFF` for none).
pub const CLIENT_CHAIR_CANCEL: u16 = 0x00DA;

/// The `u16` the client sends for *no chair*. `FUN_142cd3a60` writes it as an immediate [L].
pub const NO_CHAIR: u16 = 0xFFFF;

/// Server -> client. Seats or releases **the local player**, and it is the reply the client
/// blocks on.
///
/// # Why a reply is required at all
///
/// The client seats itself - `FUN_1428d72e0` builds the chair object and its caller is the
/// `0x00DB` builder - so sitting works against a server that does nothing. **Standing does
/// not.** `FUN_142cd3a60`'s tail re-arms the `ctx+0x2330` latch, stamps a retry timer and
/// returns without touching the chair, and its caller only asks `IsSitting` then calls it. So
/// the client asks and waits, which is why eight `0x00DA` retries produced no stand [L].
///
/// # How the opcode was pinned
///
/// `CField::OnPacket` routes `0x0224 ..= 0x039F` into the user pool (`lea eax,[r9-0x224] /
/// cmp eax,0x17b / ja`), which hands `0x2C5..` to the **local** user's `FUN_14289a3a0`. Its
/// jump table is `0x14289d660` with `index = opcode - 0x2C5`, and the chair arm is
/// **index 83** -> `0x2C5 + 83` = **`0x0318`**.
///
/// *Controls, both already in this repo and neither about chairs:* index `0x50` of the same
/// table is `0x0315`, the revive dialog (`crate::revive`), and index `0xC` is `0x02D1`, the
/// level-up effect (`research/level-up.md`). The arithmetic is checked against two answers
/// that were established independently.
pub const USER_SIT: u16 = 0x0318;

/// Server -> client. **The remote half**: whose chair changed, named by character id.
///
/// The owner, 2026-09-08: *"the server needs to relay that action to all of the players in the map
/// so other players can see you sitting in a specific chair ID as well."* [`USER_SIT`] cannot
/// do that - it carries no character id and the local dispatcher applies it to
/// `ctx->localUser`. This one can.
///
/// # How it was pinned
///
/// `CUserPool::OnPacket` splits its range five ways [L]: `0x0224`, `0x0225`,
/// `0x0226..=0x0292`, `0x0293..=0x02C4` and `0x02C5..=0x039E` (the local user). The fourth
/// goes to `FUN_1429bb720`, which decodes a **character id**, looks the user up, and then
/// dispatches on a byte-indexed table:
///
/// ```text
/// 1429bbb10  add   esi, 0xfffffd6d          ; index = opcode - 0x293
/// 1429bbb16  cmp   esi, 0x30 / ja default
/// 1429bbb22  movzx eax, byte [0x1429bbd10 + index]     ; byte table
/// 1429bbb2b  mov   ecx, dword [0x1429bbcd0 + eax*4]    ; jump table
/// ```
///
/// Dword slot 2 is `FUN_1429d4fd0`, which constructs a chair object, and the byte table maps
/// that slot from exactly one opcode: **`0x02AD`** [L]. Its neighbour `0x02AE` reaches
/// `FUN_1429d5290`, unexamined.
///
/// # And it is what a MAP chair needs
///
/// `0x0318` was tried for map chairs and **refuted on a screen**: the reply went out four
/// times and the client retried four times without sitting. The local dispatcher has no
/// seat-index path at all - one chair arm in 218, and the only `SetSeat` call in it is the
/// release. This is the remaining chair path.
///
/// Body: `u32 characterId, u32 chairId, u32` - the pool takes the id, then the handler reads
/// two more (`0x1406e8f00` is a bare `jmp` to `Decode4`) [L].
pub const USER_SIT_REMOTE: u16 = 0x02AD;

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

/// `0x0318`: seat the local player on `chair_id`, or release them with `None`.
///
/// The handler decodes two `u32` and branches three ways [L]:
///
/// ```text
/// chairId != 0                 -> construct the chair object          SIT
/// chairId == 0 && second != 0  -> set a cooldown at +0x4898 and RETURN - NO release
/// chairId == 0 && second == 0  -> call [vtable+0xb8] with 0            RELEASE
/// ```
///
/// **The middle arm is the trap.** A release built as "chair id zero" with anything non-zero
/// in the second field sets a timer and returns, leaving the player seated - and it would look
/// exactly like the bug this fixes. `release()` sends both fields zero for that reason.
pub fn user_sit(chair_id: Option<u32>) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(chair_id.unwrap_or(0));
    // Always zero. For a seat it is a duration the client only reads on the release arm; for a
    // release it MUST be zero or the handler takes the cooldown branch and never stands up.
    w.u32(0);
    w.into_vec()
}

/// `0x02AD`: tell the map that `character_id` sat on `chair_id`, or stood with `None`.
///
/// Same three-`u32` shape as [`user_sit`] with the character id in front. The release is
/// zeros for the same reason - see [`user_sit`] on the middle arm.
pub fn user_sit_remote(character_id: u32, chair_id: Option<u32>) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character_id);
    w.u32(chair_id.unwrap_or(0));
    w.u32(0);
    w.into_vec()
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

    /// The middle arm of the handler is a trap: `chairId == 0` with a NON-zero second field
    /// sets a cooldown and returns **without** releasing. A release must be two zero words.
    #[test]
    fn a_release_is_two_zero_words_because_the_middle_arm_is_a_trap() {
        // chairId == 0 with a NON-zero second field sets a cooldown and returns without
        // releasing. If this ever regresses, the player stays stuck exactly as before.
        assert_eq!(user_sit(None), vec![0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn the_remote_form_puts_the_character_first_and_is_three_words() {
        let b = user_sit_remote(213, Some(3_010_005));
        assert_eq!(b.len(), 12);
        assert_eq!(u32::from_le_bytes(b[0..4].try_into().unwrap()), 213);
        assert_eq!(u32::from_le_bytes(b[4..8].try_into().unwrap()), 3_010_005);
        assert_eq!(u32::from_le_bytes(b[8..12].try_into().unwrap()), 0);
        assert_eq!(user_sit_remote(213, None), vec![213, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn a_seat_carries_the_chair_id_first() {
        let b = user_sit(Some(3_010_005));
        assert_eq!(u32::from_le_bytes(b[0..4].try_into().unwrap()), 3_010_005);
        assert_eq!(b.len(), 8);
    }

    /// Both must stay in the latching list. If either is ever removed from it, the handler
    /// stops sending the unlock and the client can no longer ask to stand at all.
    #[test]
    fn both_chair_opcodes_latch_the_exclusive_request() {
        assert!(crate::dropmoney::latches_the_exclusive_request(CLIENT_CHAIR_SIT));
        assert!(crate::dropmoney::latches_the_exclusive_request(CLIENT_CHAIR_CANCEL));
    }
}
