//! **Omok rooms** - the game half of the miniroom protocol (`0x017E` out, `0x0575` in), and
//! the balloon that advertises a room over its owner's head (`0x0233`).
//!
//! The owner, 2026-10-04: *"I just tried opening up a minigame room for omok, it did not open,
//! and it did not advertise the minigame room in the map."* The create had been arriving all
//! along - `00000000 03000000 0500 68656c6c6f 00 00`, room type 3 titled "hello" - and the
//! server only knew room type 1.
//!
//! Sources, all static against this client: `research/minigames-2026-09-09.md` (the moves),
//! `research/miniroom-rooms-2026-09-09.md` (the room open), and
//! `research/omok-room-2026-10-04.md` (the balloon, the visit, the leave reasons, and one
//! correction to the first two: Omok's `vt+0x180` does read - see [`visitor_entered`]).
//!
//! # The room-open, for a game
//!
//! Trade's base body ([`crate::trade::room_open_with`]) and then the game tail `FUN_141E99C40`
//! reads: `{u8 slot, raw 20 record}` per member, `0xFF`, `str title`, `u8 spec`. **The record
//! is read through `FUN_1402d1b50`, a `.pdata`-less thunk that `tools/reads.py` cannot see**,
//! so it is the bytes a read count would leave out.
//!
//! # Who decides what
//!
//! The server, everything: whose turn, whether a stone may go there, five in a row, a tie, an
//! undo. The client draws the board and counts its own turn clock down
//! (`research/minigames-2026-09-09.md` §6).

use crate::trade::RoomMember;
use crate::PacketWriter;

/// `roomType` for Omok - `FUN_141C3D980` builds the `0x1B40` class for it [L].
pub const ROOM_TYPE_OMOK: u32 = 3;
/// `roomType` for Match Cards. Not hosted: a create of it is refused.
pub const ROOM_TYPE_MATCH_CARDS: u32 = 4;
/// Two seats; the room's `capacity` byte and the balloon's max.
pub const OMOK_CAPACITY: u8 = 2;
/// The board is 15 x 15 (`FUN_141E95590`'s hit test) [L].
pub const BOARD: usize = 15;

/// `0x0233` **UserMiniRoomBalloon** - `FUN_1427956b0`, row `0x0d` of the `0x0226..0x0292`
/// dispatcher (`research/user-pool-tables.md`). The dispatcher reads the character id first
/// and looks the user up through `CUserPool::GetUser`, which reaches the local player too, so
/// the owner sees their own balloon from the same packet.
pub const USER_MINIROOM_BALLOON: u16 = 0x0233;

// Outbound game modes (client -> server), and inbound ones where the same number is used.
/// Out only: the room's owner sends `u8 1` from the room-open tail. Meaning not read off the
/// client; logged and otherwise ignored.
pub const MG_OWNER_OPEN: u32 = 0x0A;
/// Out: "will you accept a tie?". In: the opponent asked (`0x1F9`).
pub const MG_TIE_REQUEST: u32 = 0x11;
/// Out: the answer, `u8 1` yes. In: the tie was refused (`0x1FA`).
pub const MG_TIE_ANSWER: u32 = 0x12;
/// Out only: give up (`0x1F6`).
pub const MG_FORFEIT: u32 = 0x13;
/// Out: ask to take the last move back (`0x1FD`). In: the opponent asked (`0x1FC`).
pub const MG_UNDO_REQUEST: u32 = 0x15;
/// Out: the answer, `u8 1` yes. In: [`undo_denied`] / [`undo_accepted`].
pub const MG_UNDO_ANSWER: u32 = 0x16;
/// Out only: leave when this game ends (`0x1FF`) ...
pub const MG_LEAVE_AFTER: u32 = 0x17;
/// ... and changed my mind (`0x200`).
pub const MG_LEAVE_AFTER_CANCEL: u32 = 0x18;
/// Both ways: ready. **The client never sets its own flag** - it sends this and waits for the
/// echo (`FUN_141E95720` case `0x3f0`), so the echo goes to the sender too.
pub const MG_READY: u32 = 0x19;
/// Both ways: not ready.
pub const MG_UNREADY: u32 = 0x1A;
/// Out only: the owner expels the visitor (`0x1F7`).
pub const MG_EXPEL: u32 = 0x1B;
/// Out: the owner pressed Start. In: [`start`].
pub const MG_START: u32 = 0x1C;
/// In only: the result, [`result_draw`] / [`result_win`].
pub const MG_RESULT: u32 = 0x1D;
/// Out: **my turn clock ran out** (4 bytes). In: whose turn it is now (5 bytes) - never echo.
pub const MG_TURN: u32 = 0x1E;
/// Both ways: a stone. `u32 x, u32 y, u8 stone`, 13 bytes.
pub const OMOK_MOVE: u32 = 0x1F;
/// In only: the stone was refused.
pub const OMOK_BAD_MOVE: u32 = 0x20;
/// [`OMOK_BAD_MOVE`] reason for the double-three rule: "You have double-3's." Any other
/// reason draws "You can't put it there." [L]
pub const BAD_MOVE_DOUBLE_THREE: u32 = 0x22;
/// [`OMOK_BAD_MOVE`] reason for an occupied square.
pub const BAD_MOVE_OCCUPIED: u32 = 0;

/// The stone the first mover plays: drawn as "Mushroom" and moves first (`141e997d7`).
pub const STONE_FIRST: u8 = 1;
/// The second mover's stone, "Slime".
pub const STONE_SECOND: u8 = 2;

/// [`MG_RESULT`] kind for a draw ("It's a tie."). Any other kind reads a winner slot; only
/// `== 1` is branched on, so a win is sent as 0.
pub const RESULT_DRAW: u8 = 1;
pub const RESULT_WIN: u8 = 0;

// Leave reasons for a game room - `0x0575` mode 0x0C, read by the Omok class's `vt+0x190`
// (`FUN_141E9A5E0`) when the named slot is the reader's own [L].
/// "You have left the room." (1 and 5 both.)
pub const LEAVE_LEFT: u32 = 1;
/// The game ended and the player had asked to leave after it: "The game has ended. The room
/// will automatically close in 10 sec." - the window closes itself later.
pub const LEAVE_AFTER_GAME: u32 = 3;
/// "The room is closed." - the owner left.
pub const LEAVE_ROOM_CLOSED: u32 = 4;
/// "You have been expelled."
pub const LEAVE_EXPELLED: u32 = 6;

/// Mode 4 notice `0x14`: "The password is incorrect." (`FUN_141C3D980`, string `0x205`) [L].
pub const ROOM_NOTICE_BAD_PASSWORD: u32 = 0x14;

/// One player's record in the room: five `u32`, **20 bytes**, copied whole by
/// `FUN_1402d1b50`. Four are drawn in the record panel (`+0x10, +0x04, +0x0C, +0x08`) and which
/// is which is not read off the client; nothing keeps a record yet, so it is zeros.
pub const RECORD_LEN: usize = 20;

/// The room-open body for a game room. `my_slot` is the reader's own seat.
pub fn room_open(my_slot: u8, members: &[RoomMember], title: &str, spec: u8) -> Vec<u8> {
    let mut tail = PacketWriter::new();
    for m in members {
        tail.u8(m.slot);
        tail.zeros(RECORD_LEN);
    }
    tail.u8(crate::trade::MEMBER_LIST_END);
    tail.str(title);
    tail.u8(spec); // FUN_141E99C40 stores it at room+0x1b28; the Omok set, item id % 100
    crate::trade::room_open_with(ROOM_TYPE_OMOK, my_slot, OMOK_CAPACITY, members, &tail.into_vec())
}

/// `0x0575` mode 3: **somebody sat down** - sent to the players already in the room.
///
/// `FUN_141C3D3E0` case 3: `u8 slot`, the avatar look (`vt+0x1C8`), `u32 id`, `str name`,
/// `u16`, then `vt+0x180`. For Omok that is `FUN_141E9A2B0`, and it **reads the 20-byte record**
/// through `FUN_1402d1b50` - `research/minigames-2026-09-09.md` §5.2 said it read nothing,
/// which is `reads.py`'s `.pdata` blind spot again. Without the 20 bytes the body is short.
pub fn visitor_entered(m: &RoomMember) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(3);
    w.u8(m.slot);
    w.bytes(&m.look);
    w.u32(m.character_id);
    w.str(&m.name);
    w.u16(0);
    w.zeros(RECORD_LEN);
    w.into_vec()
}

fn mode(m: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(m);
    w.into_vec()
}

fn mode_u8(m: u32, v: u8) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(m);
    w.u8(v);
    w.into_vec()
}

/// Ready, echoed to both seats.
pub fn ready(on: bool) -> Vec<u8> {
    mode(if on { MG_READY } else { MG_UNREADY })
}

/// **The game starts.** The byte names the player who moves SECOND: a reader whose own slot
/// it is takes stone 2 and waits; the other takes stone 1 and moves (`141e99a98`) [L]. The
/// opposite polarity from [`turn`] - `research/minigames-2026-09-09.md` trap 2.
pub fn start(second_slot: u8) -> Vec<u8> {
    mode_u8(MG_START, second_slot)
}

/// Whose turn it is now - the slot that moves. 5 bytes.
pub fn turn(slot: u8) -> Vec<u8> {
    mode_u8(MG_TURN, slot)
}

/// A stone on the board, to both seats. The client hands the turn over by itself on this.
pub fn stone(x: u32, y: u32, stone: u8) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(OMOK_MOVE);
    w.u32(x);
    w.u32(y);
    w.u8(stone);
    w.into_vec()
}

/// The stone was refused - to the player who placed it.
pub fn bad_move(reason: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(OMOK_BAD_MOVE);
    w.u32(reason);
    w.into_vec()
}

pub fn result_draw() -> Vec<u8> {
    mode_u8(MG_RESULT, RESULT_DRAW)
}

pub fn result_win(winner_slot: u8) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(MG_RESULT);
    w.u8(RESULT_WIN);
    w.u8(winner_slot);
    w.into_vec()
}

/// The opponent asks for a tie.
pub fn tie_request() -> Vec<u8> {
    mode(MG_TIE_REQUEST)
}

/// The tie was refused - to the player who asked.
pub fn tie_refused() -> Vec<u8> {
    mode(MG_TIE_ANSWER)
}

/// The opponent asks to take back their last move.
pub fn undo_request() -> Vec<u8> {
    mode(MG_UNDO_REQUEST)
}

/// The undo was refused - to the player who asked. 5 bytes.
pub fn undo_denied() -> Vec<u8> {
    mode_u8(MG_UNDO_ANSWER, 0)
}

/// The undo was granted: take `count` stones off the top of the move stack, and `turn_slot`
/// moves next (`FUN_141E9B690`) [L]. 7 bytes, to both seats.
pub fn undo_accepted(count: u8, turn_slot: u8) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(MG_UNDO_ANSWER);
    w.u8(1);
    w.u8(count);
    w.u8(turn_slot);
    w.into_vec()
}

/// What the balloon over a room owner's head says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Balloon {
    /// [`ROOM_TYPE_OMOK`]. Draws `icon_omok`; 0 would remove the balloon.
    pub room_type: u32,
    /// The room's id. A click on the balloon sends it back as mode 3's ticket
    /// (`FUN_1428b7600` via `FUN_14276f790`, `mov eax,[rcx+0x1120]`) [L].
    pub room_id: u32,
    pub title: String,
    /// `user+0x113c`. Picks the Match Cards icon; for Omok the set, item id % 100.
    pub game_kind: u32,
    /// `user+0x1130`: draws `icon_private`, and a click asks for the password first
    /// (`FUN_14276f8d0`) [L].
    pub private: bool,
    /// `number/cur/%d`.
    pub cur: u8,
    /// `number/max/%d`.
    pub max: u8,
    /// `status2` while a game runs, `status1` otherwise.
    pub playing: bool,
    /// The owner, for the chat-info block the balloon carries.
    pub owner_name: String,
    pub owner_account: u32,
    pub world: u8,
}

/// The miniroom block - the same eight fields and chat-info block in `0x0233` and in
/// `0x0224`'s optional block at `1429cee29` (`research/user-enter-field.md` §2.3) [L]:
///
/// ```text
/// raw 4  room type       -> user+0x1118   0 = no balloon, and nothing else is read
/// u32    room id         -> user+0x1120
/// raw 4  canEnter        -> user+0x111c   a shop's; 0 here
/// str    title           -> user+0x1128
/// u32    game kind       -> user+0x113c
/// u8     private         -> user+0x1130
/// u8     players in      -> user+0x1138
/// u8     capacity        -> user+0x1134
/// u8     game running    -> user+0x1140
/// chat-info block        FUN_1408d6760, posted as a "[Miniroom]" line (category 0x1f)
/// ```
///
/// The chat-info read is skipped when the user's `+0x6ac` flag is set (`0x140f8abc0`); it is
/// always sent, because trailing bytes the client does not read are harmless and a block it
/// does read and does not get is not.
pub fn miniroom_block(w: &mut PacketWriter, b: Option<&Balloon>) {
    let Some(b) = b else {
        w.zeros(4);
        return;
    };
    w.u32(b.room_type);
    w.u32(b.room_id);
    w.u32(0);
    w.str(&b.title);
    w.u32(b.game_kind);
    w.u8(u8::from(b.private));
    w.u8(b.cur);
    w.u8(b.max);
    w.u8(u8::from(b.playing));
    let who = crate::megaphone::Speaker { name: &b.owner_name, account_id: b.owner_account, character_id: b.room_id, world: b.world };
    crate::megaphone::chat_info(w, &who, &crate::megaphone::line(&b.owner_name, &b.title));
}

/// `0x0233` for `character`: their balloon, or none (`None` takes it down).
pub fn balloon(character: u32, b: Option<&Balloon>) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character);
    miniroom_block(&mut w, b);
    w.into_vec()
}

/// What a client asked of a game room.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    OwnerOpen,
    TieRequest,
    TieAnswer { yes: bool },
    Forfeit,
    UndoRequest,
    UndoAnswer { yes: bool },
    LeaveAfterGame { on: bool },
    Ready { on: bool },
    Expel,
    Start,
    TimeUp,
    Move { x: u32, y: u32, stone: u8 },
}

/// The game modes of `0x017E`, after the mode word. `None` for any other mode.
pub fn parse_action(mode: u32, c: &mut crate::PacketReader) -> Option<Action> {
    Some(match mode {
        MG_OWNER_OPEN => Action::OwnerOpen,
        MG_TIE_REQUEST => Action::TieRequest,
        MG_TIE_ANSWER => Action::TieAnswer { yes: c.u8().ok()? == 1 },
        MG_FORFEIT => Action::Forfeit,
        MG_UNDO_REQUEST => Action::UndoRequest,
        MG_UNDO_ANSWER => Action::UndoAnswer { yes: c.u8().ok()? == 1 },
        MG_LEAVE_AFTER => Action::LeaveAfterGame { on: true },
        MG_LEAVE_AFTER_CANCEL => Action::LeaveAfterGame { on: false },
        MG_READY => Action::Ready { on: true },
        MG_UNREADY => Action::Ready { on: false },
        MG_EXPEL => Action::Expel,
        MG_START => Action::Start,
        MG_TURN => Action::TimeUp,
        OMOK_MOVE => Action::Move { x: c.u32().ok()?, y: c.u32().ok()?, stone: c.u8().ok()? },
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn member(slot: u8) -> RoomMember {
        RoomMember { slot, character_id: 215, name: "Wisp".into(), look: vec![0xAA; 195] }
    }

    #[test]
    fn the_room_open_ends_with_the_game_tail() {
        let body = room_open(0, &[member(0)], "hello", 0);
        let base = crate::trade::room_open_len(&[member(0)]);
        // B is the room type, at bytes 8..12.
        assert_eq!(&body[8..12], &ROOM_TYPE_OMOK.to_le_bytes());
        // slot, 20 record bytes, terminator, title, spec.
        assert_eq!(body.len(), base + 1 + RECORD_LEN + 1 + 2 + 5 + 1);
        assert_eq!(body[base], 0, "the record's slot");
        assert_eq!(body[base + 21], 0xFF);
        assert_eq!(&body[base + 22..base + 24], &[5, 0]);
        assert_eq!(&body[base + 24..base + 29], b"hello");
    }

    #[test]
    fn a_visitor_carries_their_record() {
        let body = visitor_entered(&member(1));
        assert_eq!(body.len(), 4 + 1 + 195 + 4 + 2 + 4 + 2 + RECORD_LEN);
        assert_eq!(&body[..5], &[3, 0, 0, 0, 1]);
    }

    #[test]
    fn the_game_bodies_have_their_lengths() {
        assert_eq!(stone(7, 7, STONE_FIRST), vec![0x1F, 0, 0, 0, 7, 0, 0, 0, 7, 0, 0, 0, 1]);
        assert_eq!(bad_move(BAD_MOVE_DOUBLE_THREE).len(), 8);
        assert_eq!(start(1), vec![0x1C, 0, 0, 0, 1]);
        assert_eq!(turn(0), vec![0x1E, 0, 0, 0, 0]);
        assert_eq!(result_draw(), vec![0x1D, 0, 0, 0, 1]);
        assert_eq!(result_win(1), vec![0x1D, 0, 0, 0, 0, 1]);
        assert_eq!(undo_accepted(2, 0), vec![0x16, 0, 0, 0, 1, 2, 0]);
        assert_eq!(undo_denied(), vec![0x16, 0, 0, 0, 0]);
        assert_eq!(ready(true), vec![0x19, 0, 0, 0]);
    }

    #[test]
    fn the_balloon_reads_as_the_client_reads_it() {
        let b = Balloon {
            room_type: ROOM_TYPE_OMOK,
            room_id: 215,
            title: "hello".into(),
            game_kind: 0,
            private: false,
            cur: 1,
            max: 2,
            playing: false,
            owner_name: "Wisp".into(),
            owner_account: 7,
            world: 0,
        };
        let body = balloon(215, Some(&b));
        let mut r = crate::PacketReader::new(&body);
        assert_eq!(r.u32().unwrap(), 215, "the dispatcher's character id");
        assert_eq!(r.u32().unwrap(), ROOM_TYPE_OMOK);
        assert_eq!(r.u32().unwrap(), 215, "room id");
        assert_eq!(r.u32().unwrap(), 0);
        assert_eq!(r.str().unwrap(), "hello");
        assert_eq!(r.u32().unwrap(), 0);
        assert_eq!([r.u8().unwrap(), r.u8().unwrap(), r.u8().unwrap(), r.u8().unwrap()], [0, 1, 2, 0]);
        assert_eq!(r.str().unwrap(), "Wisp", "the chat-info block follows");
        // Taken down: the id and four zero bytes, nothing else read.
        assert_eq!(balloon(215, None), vec![215, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn the_actions_parse() {
        let mv = [7u8, 0, 0, 0, 8, 0, 0, 0, 1];
        assert_eq!(parse_action(OMOK_MOVE, &mut crate::PacketReader::new(&mv)), Some(Action::Move { x: 7, y: 8, stone: 1 }));
        assert_eq!(parse_action(MG_TIE_ANSWER, &mut crate::PacketReader::new(&[1])), Some(Action::TieAnswer { yes: true }));
        assert_eq!(parse_action(MG_TURN, &mut crate::PacketReader::new(&[])), Some(Action::TimeUp));
        assert_eq!(parse_action(0x23, &mut crate::PacketReader::new(&[])), None, "Match Cards is not hosted");
    }
}
