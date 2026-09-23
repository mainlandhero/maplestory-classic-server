//! Maple Chat (the client's *messenger*): `0x01FD` in, `0x00A3` out.
//!
//! The owner, 2026-09-15: *"I just invited Tester2 to a 'Maple Chat' session, please implement
//! that functionality, make sure the invite pops up on the other client, once accepted,
//! they should also have a chat session via the chat-hub server in a dedicated UI that the
//! client already implements."*
//!
//! # The request, captured (`world-ch0.log` 00:39:49)
//!
//! ```text
//! 0x01FD  00000000 01 0700 "Tester2"
//!         ^        ^  ^
//!         |        |  str the invitee's name
//!         |        u8 hasName
//!         u32 mode 0: open a Maple Chat. Builder FUN_141183010 [L]: it writes the literal
//!             0 as a 4-byte raw, then `u8 1 + str name` when a name was given, else `u8 0`.
//!             Sent only when the client holds no messenger id (global < 0) and has no
//!             window - i.e. this is "create one and invite".
//! ```
//!
//! Every `0x01FD` starts with a **`u32` mode**, read off all eight builders
//! (`research/msexe-send-opcodes.txt` lines 1305-1312; `tools/encodes.py` on each): **[L]**
//!
//! ```text
//! 0   open          u8 hasName, [str name]           FUN_141183010 (new), FUN_141182e70
//! 1   leave         u32 messengerId                  the inline one at 1411865d1, and
//!                                                    FUN_141183150, found by byte scan
//! 3   chat          str text                         FUN_141188700 (the chatinput box),
//!                                                    and FUN_141183200, found by byte scan
//! 5   invite        str name                         FUN_141183310 (resolves 'already in room')
//! 7   ?             u32                              FUN_14180b750
//! 8   decline       u32 messengerId, str name        FUN_1411838e0 - sent by the invitee's
//!                   client on its own when invites are blocked or the inviter is
//! ```
//!
//! Mode 7 IS the accept - captured 01:52:50: `07000000 01000100`, u32 mode 7 then the
//! messenger id the dialog was given. **Mode 3 is a typed line** and **mode 1 is the window
//! being closed**, both captured 2026-09-22.
//!
//! # There is no typing indicator, and here is how that was measured
//!
//! The owner, 2026-09-22: *"The problem is that there's no typing indicator when a user is typing
//! a message."* There is not, and no server can put one there. **Three instruments, three
//! directions, and the first version of this answer was not one of them.**
//!
//! It used to read: "the client has eight `0x01FD` builders
//! (`research/msexe-send-opcodes.txt` 1305-1312) and none of them says typing". That is a
//! **miss in the census**, and the census's own header says in capitals that a miss in it is
//! not evidence - it resolved 1881 of 1894 call sites and undercounts `0x017E` by 14. So the
//! claim was retired and re-measured:
//!
//! 1. **`python tools/builder_scan.py 0x1fd`** - a byte scan for every immediate load of the
//!    opcode into `edx` followed by a call to the constructor, which needs neither the census
//!    nor Ghidra. Its control re-finds **38 of 38** census sites for `0x017E` before it will
//!    report anything. It finds **12** call sites, four of which the census does not name:
//!    two are false pairings inside `0x017E` code (reported with their +89 distance, which is
//!    the tell), and **two are real builders the census missed** - `141183150` writes mode
//!    **1** and `141183200` mode **3**, second copies of the leave and the chat line. So the
//!    modes this client can send are `{0, 0, 1, 1, 3, 3, 5, 7, 8, 8}` and **none is
//!    "typing"**. Blind spot, named: an opcode reaching `edx` from a register or memory is
//!    invisible to this scan.
//! 2. **`UI_000.wz/MapleChat.img`** has 34 distinct node names. The window has a background,
//!    six buttons, a chat input, a scroll bar, a name tag, avatar offsets, a user-info grid
//!    and `layer:newchat_enabled`/`_disabled` - the *unread* lamp for a minimised window.
//!    There is **no typing state and no canvas for one**, and `CLAUDE.md`'s note that much of
//!    this client's wording is bitmaps rather than strings cuts the right way here: a baked
//!    indicator would still have to be a canvas in this image.
//! 3. **The string table** - all 6165 entries, decrypted with `tools/dump_stringids.py` -
//!    contains no "typing", "is writing" or "entering a message".
//!
//! A feature needs a packet to carry it, art to draw it and words to say it; this client has
//! none of the three. What it does have is mode 3, which carries the finished line, and that
//! works.
//!
//! # The reply: `0x00A3`, `FUN_141183ec0` (`research/msexe-gamestage-cases.txt`)
//!
//! `u32 messengerId, i32 mode` then a 9-entry table at `0x141184330` (read out of the PE): **[L]**
//!
//! ```text
//! 0  141183f35  SELF ENTER: i32 result. 0 -> the id is stored (141183f50) and the window
//!               (UI type 0x1b) opens; anything else -> nothing
//! 1  141183f7c  u32, then FUN_141183cc0 (a member leaves?)
//! 2  141183f92  FUN_141183cc0 with no read
//! 3  141183fb0  only for the current messenger: u8, str [, str] -> a line in the window
//! 4  14118400a  MEMBERS. **SIX records either way** - the "otherwise ONE record" this note
//!               used to claim was wrong, and sending one killed the client (see `members`).
//!               FUN_141184360 scans the window's six slots (0x143aca880, 0x20 apart,
//!               occupied when [slot+4] != 0) only to pick which LOOP to run, and both loops
//!               run six times: the all-empty one fills the slots, the other reads a record
//!               per slot while DIFFING the old id against the new, which is how the window
//!               announces who entered and who left. Each record,
//!               FUN_140425ed0: u32 -> [slot+0] (the position), u32 -> [slot+4] (the
//!               character id; 0 = empty, and the record ENDS there), str name, u8 (0 = no
//!               look, the record ends), then the avatar look FUN_1402ee8d0 reads - the
//!               same bytes `opcode::avatar_look` has put on screen since 2026-08-19.
//! 5  141184017  u32
//! 6  14118403c  INVITE: u8 flag, u32 inviterId, str inviterName -> FUN_1411838e0, which
//!               shows the dialog ('Chat invite from', 0x043C) when flag != 0, and when
//!               flag == 0 auto-declines with request mode 8 if FUN_142d01050(inviterId)
//!               says so or the client's option [+0x140] is off. flag 1 forces the dialog.
//! 7  14118409e  u32, then the same open as mode 0
//! 8  1411840c7  only for the current messenger: str -> a formatted line
//! ```
//!
//! **Nothing here has been on a screen.** This module builds modes 0 and 6; the run that
//! follows is what measures them, and the accept it produces is the next capture.
//!
//! # Nothing here authenticates
//!
//! As everywhere in this project, the channel socket carries no credentials.

use crate::packet::{PacketReader, PacketWriter};

/// `0x01FD` - the client opens, invites to, leaves or types in a Maple Chat.
pub const CLIENT_MESSENGER: u16 = 0x01FD;

/// `0x00A3` - a Maple Chat result.
pub const MESSENGER: u16 = 0x00A3;

/// The request's `u32` mode.
pub mod request {
    pub const OPEN: u32 = 0;
    /// Mode 1: the player closed their Maple Chat window. `u32 messengerId`. Captured
    /// 2026-09-22 14:29:08: `01000000 01000100`. Builder: the inline one at `1411865d1`,
    /// which writes the literal `1`.
    pub const LEAVE: u32 = 1;
    /// Mode 3: a typed line. `str text`. Captured 2026-09-22 14:27:36 twice:
    /// `03000000 0500 "hello"`. Builder `FUN_141188700`, which reads the `chatinput` box.
    pub const CHAT: u32 = 3;
    pub const INVITE: u32 = 5;
    /// The invite dialog's Accept: `u32 messengerId`. Captured 2026-09-15 01:52:50.
    pub const ENTER: u32 = 7;
    pub const DECLINE: u32 = 8;
}

/// The reply's `i32` mode.
pub mod result {
    pub const SELF_ENTER: i32 = 0;
    /// A line in the window: `u8 position, str, str`. See [`chat`].
    pub const CHAT: i32 = 3;
    pub const MEMBERS: i32 = 4;
    pub const INVITE: i32 = 6;
}

/// The window has six seats.
pub const SEATS: usize = 6;

/// A parsed [`CLIENT_MESSENGER`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessengerRequest {
    /// Mode 0. `invite` is the name given with the open, if any.
    Open { invite: Option<String> },
    /// Mode 1: the window was closed.
    Leave { messenger_id: u32 },
    /// Mode 3: a typed line.
    Chat { text: String },
    /// Mode 5.
    Invite { name: String },
    /// Mode 7: the invite dialog's Accept.
    Enter { messenger_id: u32 },
    /// Mode 8: the invitee's client declined on its own.
    Decline { messenger_id: u32, name: String },
    /// A mode this server has not decoded; the bytes after the mode.
    Other { mode: u32, rest: Vec<u8> },
}

/// Parse the body after the opcode.
pub fn parse_messenger(body: &[u8]) -> Option<MessengerRequest> {
    let mut r = PacketReader::new(body);
    let mode = r.u32().ok()?;
    Some(match mode {
        request::OPEN => {
            let has_name = r.u8().ok()? != 0;
            let invite = if has_name { Some(r.str().ok()?) } else { None };
            MessengerRequest::Open { invite }
        }
        request::LEAVE => MessengerRequest::Leave { messenger_id: r.u32().ok()? },
        request::CHAT => MessengerRequest::Chat { text: r.str().ok()? },
        request::INVITE => MessengerRequest::Invite { name: r.str().ok()? },
        request::ENTER => MessengerRequest::Enter { messenger_id: r.u32().ok()? },
        request::DECLINE => MessengerRequest::Decline { messenger_id: r.u32().ok()?, name: r.str().ok()? },
        other => MessengerRequest::Other { mode: other, rest: body.get(4..).unwrap_or(&[]).to_vec() },
    })
}

/// Mode 0 to the opener: `u32 messengerId, i32 0, i32 result`. `0` opens the window.
pub fn self_enter(messenger_id: u32, result: i32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(messenger_id); //     141183eed
    w.i32(result::SELF_ENTER); //141183f02
    w.i32(result); //           141183f42
    w.into_vec()
}

/// Mode 6 to the invitee: `u32 messengerId, i32 6, u8 1, u32 inviterId, str inviterName`.
/// The flag is 1 so the dialog is shown rather than auto-declined.
pub fn invite(messenger_id: u32, inviter_id: u32, inviter_name: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(messenger_id); //     141183eed
    w.i32(result::INVITE); //   141183f02
    w.u8(1); //                 14118403f  flag: show the dialog
    w.u32(inviter_id); //       14118404d
    w.str(inviter_name); //     14118405b
    w.into_vec()
}

/// Mode 3 - **one line in every window**: `u32 messengerId, i32 3, u8 position, str, str`.
///
/// Decompiled 2026-09-22 rather than guessed. `FUN_141183ec0` case 3 runs only when the id
/// matches the client's current messenger, then `FUN_140426220` reads **`u8`, `str`, `str`**
/// into a `{u32, String, String}` and `FUN_141183b20` appends it to the window's line list.
///
/// The `u8` is the **speaker's seat**, not a flag: `FUN_141183b20` uses it to index the six
/// slots at `DAT_143aca880` (`0x20` apart) and asks `FUN_142d01050` whether that slot's
/// character is blocked, dropping the line if so. A value of 6 or more falls back to a
/// scratch slot, so it must be a real seat for the block check to mean anything.
///
/// # This is sent to the speaker as well
///
/// `FUN_141188700` builds the request from the `chatinput` box and **does not** call
/// `FUN_141183b20` - the client never draws its own line. So a server that sends the line
/// only to the others leaves the speaker looking at an empty window, which is half of what
/// The owner reported on 2026-09-22.
pub fn chat(messenger_id: u32, position: u8, name: &str, text: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(messenger_id);
    w.i32(result::CHAT);
    w.u8(position); //  141183b20  indexes the six slots for the block check
    w.str(name); //     140426220  first string
    w.str(text); //     140426220  second string
    w.into_vec()
}

/// One seat of a Maple Chat, as mode 4 sends it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seat {
    pub character_id: u32,
    pub name: String,
    /// `opcode::avatar_look` of the member.
    pub look: Vec<u8>,
}

fn write_seat(w: &mut PacketWriter, position: u32, seat: Option<&Seat>) {
    w.u32(position); //          140425f27 -> [slot+0]
    match seat {
        None => w.u32(0), //     140425f31 -> [slot+4] = 0: the record ends
        Some(s) => {
            w.u32(s.character_id); //140425f31
            w.str(&s.name); //   140425f49
            w.u8(1); //          140425f8b  a look follows
            w.bytes(&s.look) //  140426186  FUN_1402ee8d0
        }
    };
}

/// Mode 4: **six records, one per seat, always** - empty seats as `u32 position, u32 0`.
///
/// # There is no one-record form, and assuming there was crashed the client
///
/// This module used to carry a `member_joined` that sent the newcomer's single record to
/// the members already seated, on the reasoning that a window with somebody in it only
/// needs the new arrival. **The client refuses it.** Measured, `world-ch0.log` 2026-09-18:
///
/// ```text
/// 01:38:09.581 -> [Wisp#215] 0x00A3 mode 4: Tester2 joined in seat 1, ONE record
/// 01:38:09.590 <- [Wisp#215] 0x009E CLIENT_PACKET_REJECTED, 262 byte body
/// 01:38:12.695    ch0 #3 ended: forcibly closed by the remote host (os error 10054)
/// ```
///
/// The rejected packet inside that `0x009E` is byte-for-byte the one we sent, and the
/// client's copy of it is **262 bytes** where ours was 86 - it kept reading past the end of
/// the packet into zero padding, looking for the five records that were not there, and threw
/// the whole thing out. Nine milliseconds, and the process was gone three seconds later.
///
/// The six-record form was never rejected by either client in the same capture, so that is
/// the only mode-4 shape known to be accepted and every join now sends it to everybody.
pub fn members(messenger_id: u32, seats: &[Option<Seat>; SEATS]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(messenger_id);
    w.i32(result::MEMBERS);
    for (i, seat) in seats.iter().enumerate() {
        write_seat(&mut w, i as u32, seat.as_ref());
    }
    w.into_vec()
}


#[cfg(test)]
mod tests {
    use super::*;

    /// The accept, the log's own bytes: mode 7 and the id the invite carried.
    #[test]
    fn the_captured_accept_parses() {
        assert_eq!(parse_messenger(&[7, 0, 0, 0, 1, 0, 1, 0]), Some(MessengerRequest::Enter { messenger_id: 0x10001 }));
    }

    /// Six records for an empty window, an empty seat ending after its zero id; one record
    /// for a newcomer, with the look after the flag byte.
    #[test]
    fn the_member_records_follow_the_readers_shape() {
        let wisp = Seat { character_id: 215, name: "Wisp".into(), look: vec![0xAA; 20] };
        let mut seats: [Option<Seat>; SEATS] = Default::default();
        seats[0] = Some(wisp.clone());
        let b = members(0x10001, &seats);
        assert_eq!(&b[..8], &[1, 0, 1, 0, 4, 0, 0, 0]);
        let mut at = 8;
        assert_eq!(&b[at..at + 4], &0u32.to_le_bytes(), "seat 0");
        assert_eq!(&b[at + 4..at + 8], &215u32.to_le_bytes());
        assert_eq!(&b[at + 8..at + 10], &4u16.to_le_bytes());
        assert_eq!(&b[at + 10..at + 14], b"Wisp");
        assert_eq!(b[at + 14], 1, "a look follows");
        assert_eq!(&b[at + 15..at + 35], &[0xAA; 20]);
        at += 35;
        for pos in 1..6u32 {
            assert_eq!(&b[at..at + 4], &pos.to_le_bytes());
            assert_eq!(&b[at + 4..at + 8], &[0, 0, 0, 0], "empty: the record ends at the zero id");
            at += 8;
        }
        assert_eq!(at, b.len());
    }

    /// **The whole table is the only mode-4 shape**, and its length is fixed by the seats
    /// rather than by who just arrived: a reader that kept going past a short packet is what
    /// the client did with the old one-record form before rejecting it. See `members`.
    #[test]
    fn mode_four_is_always_six_records_however_many_are_seated() {
        let wisp = Seat { character_id: 215, name: "Wisp".into(), look: vec![0xAA; 20] };
        let mut seats: [Option<Seat>; SEATS] = Default::default();
        let empty = members(0x10001, &seats).len();
        assert_eq!(empty, 8 + SEATS * 8, "six empty records and the header");
        seats[0] = Some(wisp.clone());
        seats[1] = Some(Seat { character_id: 214, name: "Tester2".into(), look: vec![0xBB; 20] });
        let two = members(0x10001, &seats);
        // Two occupied records are longer than the empty pair they replace, and the other
        // four are still there - the count never changes.
        let mut at = 8;
        for pos in 0..SEATS as u32 {
            assert_eq!(&two[at..at + 4], &pos.to_le_bytes(), "record {pos} is present");
            let id = u32::from_le_bytes(two[at + 4..at + 8].try_into().unwrap());
            at += 8;
            if id != 0 {
                let n = u16::from_le_bytes(two[at..at + 2].try_into().unwrap()) as usize;
                at += 2 + n + 1 + 20;
            }
        }
        assert_eq!(at, two.len(), "the six records account for every byte");
    }

    /// The owner's open-and-invite, the log's own hex.
    #[test]
    fn the_captured_open_parses_with_its_invitee() {
        let hex = "0000000001070054657374657232";
        let b: Vec<u8> = (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect();
        assert_eq!(b.len(), 14, "the log said 14 bytes");
        assert_eq!(parse_messenger(&b), Some(MessengerRequest::Open { invite: Some("Tester2".into()) }));
        assert_eq!(parse_messenger(&[0, 0, 0, 0, 0]), Some(MessengerRequest::Open { invite: None }));
        assert_eq!(parse_messenger(&[5, 0, 0, 0, 1, 0, b'X']), Some(MessengerRequest::Invite { name: "X".into() }));
        assert_eq!(
            parse_messenger(&[8, 0, 0, 0, 7, 0, 0, 0, 1, 0, b'X']),
            Some(MessengerRequest::Decline { messenger_id: 7, name: "X".into() })
        );
        // The two captures of 2026-09-22, as hex from world-ch0.log rather than rebuilt.
        let chat: Vec<u8> = (0.."03000000050068656c6c6f".len()).step_by(2)
            .map(|i| u8::from_str_radix(&"03000000050068656c6c6f"[i..i + 2], 16).unwrap()).collect();
        assert_eq!(chat.len(), 11, "the log said 11 bytes");
        assert_eq!(parse_messenger(&chat), Some(MessengerRequest::Chat { text: "hello".into() }));
        let leave: Vec<u8> = (0.."0100000001000100".len()).step_by(2)
            .map(|i| u8::from_str_radix(&"0100000001000100"[i..i + 2], 16).unwrap()).collect();
        assert_eq!(leave.len(), 8, "the log said 8 bytes");
        assert_eq!(parse_messenger(&leave), Some(MessengerRequest::Leave { messenger_id: 0x10001 }));
        assert_eq!(parse_messenger(&[0, 0, 0]), None);
        // A mode with no builder in the client still cannot panic the server.
        assert_eq!(parse_messenger(&[9, 0, 0, 0, 1]), Some(MessengerRequest::Other { mode: 9, rest: vec![1] }));
    }

    /// **A chat line is `u8 position, str, str`** - the shape `FUN_140426220` reads, with the
    /// seat first because `FUN_141183b20` indexes the six slots with it.
    #[test]
    fn a_chat_line_carries_the_speakers_seat_then_two_strings() {
        let b = chat(0x10001, 1, "Wisp", "hello");
        assert_eq!(&b[..4], &0x10001u32.to_le_bytes());
        assert_eq!(&b[4..8], &3i32.to_le_bytes(), "mode 3");
        assert_eq!(b[8], 1, "the speaker's seat, for the block check");
        assert_eq!(&b[9..11], &4u16.to_le_bytes());
        assert_eq!(&b[11..15], b"Wisp");
        assert_eq!(&b[15..17], &5u16.to_le_bytes());
        assert_eq!(&b[17..], b"hello");
        assert_eq!(b.len(), 22, "every byte accounted for: no trailing field");
    }

    /// The two replies in the handler's read order: header, then the arm's fields.
    #[test]
    fn the_replies_are_the_handlers_read_order() {
        assert_eq!(self_enter(0x10001, 0), [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let i = invite(0x10001, 215, "Wisp");
        assert_eq!(&i[..4], &0x10001u32.to_le_bytes());
        assert_eq!(&i[4..8], &6i32.to_le_bytes(), "mode 6");
        assert_eq!(i[8], 1, "flag: show the dialog, never auto-decline");
        assert_eq!(&i[9..13], &215u32.to_le_bytes());
        assert_eq!(&i[13..15], &4u16.to_le_bytes());
        assert_eq!(&i[15..], b"Wisp");
    }
}
