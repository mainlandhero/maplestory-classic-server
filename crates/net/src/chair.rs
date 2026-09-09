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
/// # Body: FOUR fields, and the fourth is why a client died
///
/// `u32 characterId, u32 chairId, u32 second, u8 show` - **13 bytes**. The pool consumes the
/// character id; `tools/reads.py` counts **three** reads in `FUN_1429d4fd0`, the first two
/// `u32` (`0x1406e8f00` is a bare `jmp` to `Decode4`) and a `u8` at `0x1429d50e8` [L].
///
/// The first version of the builder sent only three `u32`, because the body had been read off
/// the first 26 instructions and the third read is 260 bytes further in. Tester2's client
/// faulted `0xc0000005` **five milliseconds** after one went out. `CLAUDE.md` records this
/// exact mistake - a read walk that came back short - killing the client twice before.
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
    // **The byte that was missing, and its absence killed a client.** The first version of
    // this builder stopped after three `u32`, because the body was read off the first 26
    // instructions of `FUN_1429d4fd0`. `tools/reads.py` counts **three** reads in that
    // function - `u32`, `u32`, then a `u8` at `0x1429d50e8` - and the third is 260 bytes
    // further in. A 12-byte body made the client's `Decode1` underflow, which its own
    // decoders answer with a C++ throw; Tester2 faulted `0xc0000005` five milliseconds after
    // one went out, with "30 C++ throw(s) seen before this" in the fault line.
    //
    // It is **not** conditional - `reads.py` marks it `gated?` but the branch at
    // `0x1429d50d5` jumps *to* the read and the fall-through reaches it too - and it gates
    // whether the chair object is built:
    //
    // ```text
    // 1429d50e8  Decode1 -> al
    // 1429d50ed  test al, al / je skip
    // 1429d5100  call 0x141712040      ; CONSTRUCT the chair object
    // ```
    //
    // So it is 1 for a seat and 0 for a release, and the release also takes the earlier
    // `test chairId / jne` branch into `[vtable+0xc0]`, which is the clear.
    w.bool(chair_id.is_some());
    w.into_vec()
}

/// **`0x0252` - the MAP chair, and the packet three earlier attempts were missing.**
///
/// The owner, repeatedly: *"I still cannot sit in map chairs."* `research/map-chair-seat-2026-09-09.md`.
///
/// # Why nothing else could ever have worked
///
/// The client has two remote dispatchers and they look up the target user differently, which
/// is the whole answer [L]:
///
/// ```text
/// 0x0226..0x0292  FUN_1429bafb0 -> GetUser 0x1429b6c90
///                   1429b6ca9  mov rcx,[rcx+0x10]   THE LOCAL USER, checked FIRST
///                   1429b6cb7  cmp eax,ebx          ...against the id we sent
///                   1429b6cbb  mov rax,[rdi+0x10]   and returned
///                   1429b6cca  (only then the hash bucket walk at +0xf8/+0x100)
///
/// 0x0293..0x02C4  FUN_1429bb720 - where 0x02AD lives
///                   1429bb74c  mov r8,[rbx+0xf8]    STRAIGHT to the hash. No +0x10.
/// ```
///
/// The local player is not in the remote hash, so **`0x02AD` is structurally incapable of
/// addressing the player who sent the request** - it is not that the body was wrong. And
/// `0x0318`'s local dispatcher has no seat-index path at all; sending it was **refuted on
/// screen**, four replies and four retries with no seating.
///
/// # The body, counted rather than eyeballed
///
/// `tools/reads.py 0x1428341c0 6` reports **exactly two** reads, and the dispatcher head
/// consumes the `u32` before them [L]:
///
/// ```text
/// u32 characterId    1429bb08d, in the head; fed straight to GetUser at 1429bb099
/// u8  bSit           1428341e2
/// u16 seatIndex      1428341f7  ONLY IF bSit != 0  (1428341f2 test al,al / je)
/// ```
///
/// **7 bytes seated, 5 released.** A length test pins both, because a chair packet one field
/// short is exactly how `0x02AD` killed Tester2's client on this same day.
///
/// **The release is `bSit = 0`, NOT `0xFFFF`.** `1428341ea` presets `r8d = -1` and the `u16`
/// is `movzx`-widened at `1428341fc`, so a `0xFFFF` sent in the field arrives as `0x0000FFFF`
/// and is not the sentinel. Omitting the field is the only way to say "not seated".
///
/// # One gate that makes this do nothing, silently
///
/// `1428341d3 call [rax+0x58]` runs **before any read**, and a non-zero return exits the
/// handler having consumed nothing. It is not decoded. If a run shows the packet going out
/// and the player still standing, that gate is the first suspect and it is not a body fault.
pub fn user_sit_result(character_id: u32, seat: Option<u16>) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character_id);
    w.bool(seat.is_some());
    if let Some(index) = seat {
        w.u16(index);
    }
    w.into_vec()
}

/// Server -> client. Seats a character on a **map** chair, addressed by character id.
///
/// Pinned by index arithmetic with two controls [L]: `1429bb100 lea eax,[rsi-0x226]` and
/// `cmp eax,0x50`, so `index = opcode - 0x226`; entry 44 is `0x226 + 44 = 0x252` and its stub
/// `1429bb344` calls `FUN_1428341c0`. Index 11 of that same table is `0x0231 USER_CHAT`,
/// which `crates/net/src/userchat.rs` decoded independently on 2026-08-30 and records as
/// index 11 - a control this file did not have to take on trust.
pub const USER_SIT_RESULT: u16 = 0x0252;

/// The table's bound, checked at COMPILE time rather than in a test.
///
/// `1429bb106 cmp eax,0x50 / ja` sends anything past index 0x50 to the default arm, which
/// answers nothing. A test asserting this would be a constant expression - it can never fail
/// at run time - so it is a build failure instead, which is the only form that means anything.
const _: () = assert!(USER_SIT_RESULT >= 0x0226 && USER_SIT_RESULT - 0x0226 <= 0x50);

/// Byte count of a seated [`user_sit_result`].
pub const USER_SIT_RESULT_SEATED_LEN: usize = 7;

/// Byte count of a released [`user_sit_result`].
pub const USER_SIT_RESULT_RELEASED_LEN: usize = 5;

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

    /// **13 bytes, not 12.** The twelve-byte version killed Tester2's client on 2026-09-09;
    /// `tools/reads.py` counts three reads in `FUN_1429d4fd0` and the third is a `u8`.
    /// If this length assertion ever fails downward, the packet is a client-killer again.
    #[test]
    fn the_remote_form_is_three_words_and_the_show_byte() {
        let b = user_sit_remote(213, Some(3_010_005));
        assert_eq!(b.len(), 13, "u32 char, u32 chair, u32, u8 - reads.py counts three reads");
        assert_eq!(u32::from_le_bytes(b[0..4].try_into().unwrap()), 213);
        assert_eq!(u32::from_le_bytes(b[4..8].try_into().unwrap()), 3_010_005);
        assert_eq!(u32::from_le_bytes(b[8..12].try_into().unwrap()), 0);
        assert_eq!(b[12], 1, "show = 1 builds the chair object at 1429d5100");

        let r = user_sit_remote(213, None);
        assert_eq!(r.len(), 13);
        assert_eq!(r[12], 0, "show = 0 skips the construction; the clear is the chairId branch");
        assert_eq!(u32::from_le_bytes(r[4..8].try_into().unwrap()), 0);
    }

    #[test]
    fn a_seat_carries_the_chair_id_first() {
        let b = user_sit(Some(3_010_005));
        assert_eq!(u32::from_le_bytes(b[0..4].try_into().unwrap()), 3_010_005);
        assert_eq!(b.len(), 8);
    }

    /// **7 seated, 5 released.** `reads.py 0x1428341c0 6` counts two reads and the head takes
    /// the `u32`. A chair packet one field short is how `0x02AD` killed a client on
    /// 2026-09-09; if this ever fails downward it is a client-killer again.
    #[test]
    fn the_map_seat_is_seven_bytes_seated_and_five_released() {
        let seated = user_sit_result(213, Some(24));
        assert_eq!(seated.len(), USER_SIT_RESULT_SEATED_LEN, "u32 + u8 + u16");
        assert_eq!(u32::from_le_bytes(seated[0..4].try_into().unwrap()), 213);
        assert_eq!(seated[4], 1, "bSit, read at 1428341e2");
        assert_eq!(u16::from_le_bytes(seated[5..7].try_into().unwrap()), 24);

        let up = user_sit_result(213, None);
        assert_eq!(up.len(), USER_SIT_RESULT_RELEASED_LEN, "the u16 is gated on bSit");
        assert_eq!(u32::from_le_bytes(up[0..4].try_into().unwrap()), 213);
        assert_eq!(up[4], 0, "bSit = 0 is the release");
    }

    /// **The release must be the ABSENT field, never `0xFFFF` in it.** `1428341ea` presets
    /// `r8d = -1` and `1428341fc` is a `movzx`, so a `0xFFFF` written into the `u16` reaches
    /// `SetChair` as `0x0000FFFF` - a seat index of 65535, not the sentinel. `0x00DA` uses
    /// `0xFFFF` for exactly this meaning, so the two conventions are opposite and mixing them
    /// up is the easy mistake.
    #[test]
    fn the_release_omits_the_field_rather_than_sending_ffff() {
        let up = user_sit_result(213, None);
        assert_eq!(up.len(), 5, "no seat index at all");
        assert!(!up.windows(2).any(|w| w == [0xff, 0xff]), "0xFFFF must not appear");
        // The control: 0xFFFF really is representable in this builder, so its absence above
        // is a property of the release form and not of the writer.
        assert!(user_sit_result(213, Some(0xffff)).windows(2).any(|w| w == [0xff, 0xff]));
    }

    /// Index arithmetic, with the control the decode itself used.
    #[test]
    fn the_opcode_is_index_forty_four_of_the_0x226_table() {
        assert_eq!(USER_SIT_RESULT, 0x0252);
        assert_eq!(USER_SIT_RESULT - 0x0226, 44, "1429bb100 lea eax,[rsi-0x226]");
        // Index 11 of the SAME table is USER_CHAT, decoded independently in userchat.rs.
        assert_eq!(0x0226 + 11, crate::userchat::USER_CHAT);
    }

    /// Both must stay in the latching list. If either is ever removed from it, the handler
    /// stops sending the unlock and the client can no longer ask to stand at all.
    #[test]
    fn both_chair_opcodes_latch_the_exclusive_request() {
        assert!(crate::dropmoney::latches_the_exclusive_request(CLIENT_CHAIR_SIT));
        assert!(crate::dropmoney::latches_the_exclusive_request(CLIENT_CHAIR_CANCEL));
    }
}
