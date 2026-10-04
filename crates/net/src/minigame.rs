//! **Omok and Match Cards rooms** - the game half of the miniroom protocol (`0x017E` out,
//! `0x0575` in), and the balloon that advertises a room over its owner's head (`0x0233`).
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
/// `roomType` for Match Cards - the `0x1860` class, `FUN_141C145F0` [L].
pub const ROOM_TYPE_MATCH_CARDS: u32 = 4;
/// Match Cards board sizes by the create's spec byte: `FUN_141C1B080` maps 0 / 1 / 2 to 12 /
/// 20 / 30 cards (4 / 5 / 6 wide) [L].
pub const CARD_COUNTS: [u8; 3] = [12, 20, 30];
/// Card faces the client has art for: `UI/Minigame.img/MatchCards/card/0` .. `/14`, picked by
/// formatting the dealt value with `"%d"` (`FUN_141C262D0`) [L]. 15 faces, 15 pairs at most.
pub const CARD_FACES: u32 = 15;
/// Both ways: a card turned. Out `u8 isFirst, u8 index`; in, see [`card_first`] /
/// [`card_second`].
pub const MC_CARD: u32 = 0x23;
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

/// One player's record: five `u32`, **20 bytes**, copied whole by `FUN_1402d1b50`.
pub const RECORD_LEN: usize = 20;

/// **A player's record at one game**, as the side panel draws it. `FUN_141C17290` formats
/// `+0x10` into the top box and `+0x04`, `+0x0C`, `+0x08` into rows at y `0xa9`, `0xba`,
/// `0xcb`; the panel's baked labels (`UI/Minigame.img/Common/score`) read **PTS** over
/// **W / L / D** [L]. So: `+0x04` wins, `+0x08` ties, `+0x0C` losses, `+0x10` points. `+0x00`
/// is not drawn; the v214 reference writes 1 there [I], and so does this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Record {
    pub wins: u32,
    pub ties: u32,
    pub losses: u32,
    pub points: u32,
}

impl Record {
    pub fn write(&self, w: &mut PacketWriter) {
        w.u32(1);
        w.u32(self.wins);
        w.u32(self.ties);
        w.u32(self.losses);
        w.u32(self.points);
    }
}

/// The room-open body for an Omok room. `my_slot` is the reader's own seat; `records[i]` is
/// `members[i]`'s.
pub fn room_open(my_slot: u8, members: &[RoomMember], records: &[Record], title: &str, spec: u8) -> Vec<u8> {
    room_open_game(ROOM_TYPE_OMOK, my_slot, members, records, title, spec)
}

/// The room-open body for either game: Omok (`FUN_141E99C40`) and Match Cards
/// (`FUN_141C1B080`) read the same tail. For Match Cards `spec` is the board size, 0 / 1 / 2.
pub fn room_open_game(room_type: u32, my_slot: u8, members: &[RoomMember], records: &[Record], title: &str, spec: u8) -> Vec<u8> {
    debug_assert_eq!(members.len(), records.len());
    let mut tail = PacketWriter::new();
    for (m, r) in members.iter().zip(records) {
        tail.u8(m.slot);
        r.write(&mut tail);
    }
    tail.u8(crate::trade::MEMBER_LIST_END);
    tail.str(title);
    tail.u8(spec); // Omok: room+0x1b28, the set (item id % 100). Match Cards: the board size
    crate::trade::room_open_with(room_type, my_slot, OMOK_CAPACITY, members, &tail.into_vec())
}

/// `0x0575` mode 3: **somebody sat down** - sent to the players already in the room.
///
/// `FUN_141C3D3E0` case 3: `u8 slot`, the avatar look (`vt+0x1C8`), `u32 id`, `str name`,
/// `u16`, then `vt+0x180`. For Omok that is `FUN_141E9A2B0`, and it **reads the 20-byte record**
/// through `FUN_1402d1b50` - `research/minigames-2026-09-09.md` §5.2 said it read nothing,
/// which is `reads.py`'s `.pdata` blind spot again. Without the 20 bytes the body is short.
pub fn visitor_entered(m: &RoomMember, record: &Record) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(3);
    w.u8(m.slot);
    w.bytes(&m.look);
    w.u32(m.character_id);
    w.str(&m.name);
    w.u16(0);
    record.write(&mut w);
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

/// **Match Cards starts: the deal.** `FUN_141C1CA70`: `u8` the seat that moves SECOND (the same
/// polarity as [`start`]), `u8 count`, then `count` `u32` faces in board order, read raw [L].
/// The client never shuffles; it shows every card for `count * 200 + 0x2cec` ms and turns them
/// down by itself.
pub fn deal(second_slot: u8, faces: &[u32]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(MG_START);
    w.u8(second_slot);
    w.u8(faces.len() as u8);
    for f in faces {
        w.u32(*f);
    }
    w.into_vec()
}

/// The opponent turned the first card of a pair - to the opponent only: the clicker's own
/// client turned it already (`FUN_141C16CD0`) [L].
pub fn card_first(index: u8) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(MC_CARD);
    w.u8(1);
    w.u8(index);
    w.into_vec()
}

/// The second card of a pair, to both seats (the clicker's client waits for it).
/// `result` (`FUN_141C1C4B0`), against the room's capacity of 2 [L]:
/// below it, no match and the turn passes - the value is the seat that moved; at or above it, a
/// match scored by seat `result - 2`, who keeps the turn. A miss turns both cards face down by
/// itself 900 ms later (`room+0x17c4` / `+0x17bc`, the tick in `FUN_141C1BF70`).
pub fn card_second(second: u8, first: u8, result: u8) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(MC_CARD);
    w.u8(0);
    w.u8(second);
    w.u8(first);
    w.u8(result);
    w.into_vec()
}

/// [`card_second`]'s `result` for a miss by `seat`.
pub fn card_miss(seat: u8) -> u8 {
    seat
}

/// [`card_second`]'s `result` for a pair found by `seat`.
pub fn card_match(seat: u8) -> u8 {
    OMOK_CAPACITY + seat
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

/// **The game is over: a draw**, then both seats' records AFTER it, seat 0 first.
///
/// `FUN_141E9BB10` (Omok) and `FUN_141C1C7E0` (Match Cards) end by reading seat 0's and seat
/// 1's 20-byte records through `FUN_1402d1b50` [L] - the `.pdata`-less thunk again, so
/// `research/minigames-2026-09-09.md` §3.6 counted 5 / 6 bytes where the body is 45 / 46.
pub fn result_draw(records: [Record; 2]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(MG_RESULT);
    w.u8(RESULT_DRAW);
    records[0].write(&mut w);
    records[1].write(&mut w);
    w.into_vec()
}

/// **The game is over: `winner_slot` won**, then both records, as [`result_draw`].
pub fn result_win(winner_slot: u8, records: [Record; 2]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(MG_RESULT);
    w.u8(RESULT_WIN);
    w.u8(winner_slot);
    records[0].write(&mut w);
    records[1].write(&mut w);
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
    /// Match Cards: a card turned, the first or second of a pair.
    Card { first: bool, index: u8 },
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
        MC_CARD => Action::Card { first: c.u8().ok()? == 1, index: c.u8().ok()? },
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
        let body = room_open(0, &[member(0)], &[Record::default()], "hello", 0);
        let base = crate::trade::room_open_len(&[member(0)]);
        // B is the room type, at bytes 8..12.
        assert_eq!(&body[8..12], &ROOM_TYPE_OMOK.to_le_bytes());
        // slot, 20 record bytes, terminator, title, spec.
        assert_eq!(body.len(), base + 1 + RECORD_LEN + 1 + 2 + 5 + 1);
        assert_eq!(body[base], 0, "the record's slot");
        assert_eq!(&body[base + 1..base + 5], &[1, 0, 0, 0], "the record's first u32");
        assert_eq!(body[base + 21], 0xFF);
        assert_eq!(&body[base + 22..base + 24], &[5, 0]);
        assert_eq!(&body[base + 24..base + 29], b"hello");
    }

    #[test]
    fn a_visitor_carries_their_record() {
        let body = visitor_entered(&member(1), &Record { wins: 3, ties: 2, losses: 1, points: 2020 });
        assert_eq!(body.len(), 4 + 1 + 195 + 4 + 2 + 4 + 2 + RECORD_LEN);
        assert_eq!(&body[..5], &[3, 0, 0, 0, 1]);
        assert_eq!(&body[body.len() - 20..], &[1, 0, 0, 0, 3, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0xE4, 7, 0, 0], "1, W, D, L, PTS");
    }

    #[test]
    fn the_game_bodies_have_their_lengths() {
        assert_eq!(stone(7, 7, STONE_FIRST), vec![0x1F, 0, 0, 0, 7, 0, 0, 0, 7, 0, 0, 0, 1]);
        assert_eq!(bad_move(BAD_MOVE_DOUBLE_THREE).len(), 8);
        assert_eq!(start(1), vec![0x1C, 0, 0, 0, 1]);
        assert_eq!(turn(0), vec![0x1E, 0, 0, 0, 0]);
        let both = [Record::default(); 2];
        assert_eq!(result_draw(both).len(), 5 + 2 * RECORD_LEN);
        assert_eq!(result_win(1, both).len(), 6 + 2 * RECORD_LEN);
        assert_eq!(&result_win(1, both)[..6], &[0x1D, 0, 0, 0, 0, 1]);
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
        assert_eq!(parse_action(MC_CARD, &mut crate::PacketReader::new(&[1, 4])), Some(Action::Card { first: true, index: 4 }));
        assert_eq!(parse_action(0x24, &mut crate::PacketReader::new(&[])), None);
    }

    #[test]
    fn the_match_cards_bodies_have_their_lengths() {
        assert_eq!(deal(1, &[0; 12]).len(), 54, "research/minigames-2026-09-09.md: 54 / 86 / 126");
        assert_eq!(deal(1, &[0; 30]).len(), 126);
        assert_eq!(card_first(3), vec![0x23, 0, 0, 0, 1, 3]);
        assert_eq!(card_second(5, 3, card_match(1)), vec![0x23, 0, 0, 0, 0, 5, 3, 3]);
        assert_eq!(card_miss(0), 0);
        let open = room_open_game(ROOM_TYPE_MATCH_CARDS, 0, &[member(0)], &[Record::default()], "cards", 2);
        assert_eq!(&open[8..12], &4u32.to_le_bytes());
        assert_eq!(*open.last().unwrap(), 2, "the board size closes the tail");
    }
}
