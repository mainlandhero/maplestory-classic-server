//! **Omok rooms**: the room, its balloon on the map, and the game's rules.
//!
//! The owner, 2026-10-04: *"I just tried opening up a minigame room for omok, it did not open,
//! and it did not advertise the minigame room in the map."* `net::minigame` owns the wire and
//! its working; this is the room table and the referee.
//!
//! # How a room lives
//!
//! * **Create** (mode 0, room type 3): the owner takes seat 0 and their window opens at once
//!   (mode 4 with one member); everyone on the map, the owner included, gets the balloon
//!   (`0x0233`), and the owner's cached `0x0224` carries it for players who arrive later.
//! * **Visit** - a click on the balloon is mode 3 with the room id as the ticket, the same mode
//!   a trade accept uses. The visitor takes seat 1: their window opens with both members, the
//!   owner's gets "a visitor sat down" (mode 3), the balloon says 2/2.
//! * **Ready / Start / moves / tie / undo / forfeit / expel**: [`Session::game_action`].
//! * **Leave**: the owner leaving closes the room for both; the visitor leaving frees the seat.
//!   Leaving mid-game forfeits it first.
//!
//! The room id is the owner's character id. A character is in at most one miniroom, so it
//! cannot collide with a trade ticket that is live at the same time - the busy check sees to
//! that.
//!
//! # What is decided here, not on the client
//!
//! Everything (`research/minigames-2026-09-09.md` §6): whose turn, an occupied square, the
//! double-three rule, five in a row, a full board. The client draws, and reports its own turn
//! clock running out.
//!
//! # Not kept
//!
//! The 20-byte win/draw/loss record is sent as zeros: which of its five `u32` is which is not
//! read off the client, and nothing stores a record yet. The `0x0224` copy a late arrival
//! gets is rebuilt when the OWNER's state changes (create, close), so its player count and
//! "game running" flag can lag; the live `0x0233` everyone on the map gets is always current.

use std::time::{Duration, Instant};

use super::{Reply, Session};

/// **Both clients report a turn clock running out**, about 20 ms apart (the owner's run of
/// 2026-10-04: 05:01:48.974 and .993). Before this guard the second report, arriving after the
/// first had already passed the turn, passed it straight back - and the next stone was refused
/// as out of turn. A report this soon after the clock last restarted is that duplicate. Every
/// real time-out is at least 10 s after a restart (Match Cards' clock; Omok's is 30 s).
const DUPLICATE_TIME_UP: Duration = Duration::from_secs(3);

const BOARD: usize = net::minigame::BOARD;

/// One seat of a game room.
#[derive(Debug, Clone)]
pub(crate) struct GameSeat {
    character_id: u32,
    name: String,
    look: Vec<u8>,
    account_id: u32,
    world: u8,
    /// Their record at this room's game, as of sitting down; updated when a game ends.
    record: net::minigame::Record,
}

/// A game in progress - the part both games share, and the board.
#[derive(Debug, Clone)]
pub(crate) struct Game {
    /// The seat that moves now.
    turn: usize,
    /// The seat that moved first this game.
    first: usize,
    /// When the clients' turn clock last restarted - every packet that restarts it on the
    /// client (the start, a stone, a card pair, a time-out, an undo) restarts this.
    clock: Instant,
    tie_asked_by: Option<usize>,
    undo_asked_by: Option<usize>,
    board: Board,
}

#[derive(Debug, Clone)]
enum Board {
    Omok {
        /// `cells[y][x]`: 0 empty, else the stone type.
        cells: [[u8; BOARD]; BOARD],
        /// The stone each seat plays: the first mover's is [`net::minigame::STONE_FIRST`].
        stones: [u8; 2],
        /// Every stone in order, `(x, y, seat)` - the client keeps the same stack and pops
        /// from it on an undo (`FUN_141E9B690`).
        moves: Vec<(usize, usize, usize)>,
    },
    Cards {
        /// The dealt face of every card in board order; `None` once its pair is found.
        faces: Vec<Option<u32>>,
        /// The first card of the pair being turned.
        turned: Option<usize>,
        /// Pairs found, per seat.
        pairs: [u32; 2],
    },
}

/// A fresh Match Cards deal: `count / 2` faces drawn from the client's 15 without repeats, each
/// twice, shuffled. `next` is a random source.
pub(crate) fn deal_cards(count: usize, mut next: impl FnMut() -> u64) -> Vec<u32> {
    let mut pool: Vec<u32> = (0..net::minigame::CARD_FACES).collect();
    for i in (1..pool.len()).rev() {
        pool.swap(i, (next() % (i as u64 + 1)) as usize);
    }
    let mut faces: Vec<u32> = pool.into_iter().take(count / 2).flat_map(|f| [f, f]).collect();
    for i in (1..faces.len()).rev() {
        faces.swap(i, (next() % (i as u64 + 1)) as usize);
    }
    faces
}

/// One open game room.
#[derive(Debug, Clone)]
pub(crate) struct GameRoom {
    /// The owner's character id - also what the balloon carries and a click sends back.
    id: u32,
    room_type: u32,
    title: String,
    password: Option<String>,
    /// Omok: the set, item id % 100. Match Cards: the board size, 0 / 1 / 2. Echoed to the
    /// room open and the balloon.
    spec: u8,
    map: crate::fields::FieldKey,
    seats: [Option<GameSeat>; 2],
    visitor_ready: bool,
    game: Option<Game>,
    /// Who moves first next game: the owner first, then the loser of the last game.
    first_next: usize,
    /// "Leave after this game" (modes 0x17 / 0x18).
    leave_after: [bool; 2],
}

impl GameRoom {
    fn seat_of(&self, character: u32) -> Option<usize> {
        self.seats.iter().position(|s| s.as_ref().is_some_and(|s| s.character_id == character))
    }

    fn members(&self) -> Vec<net::trade::RoomMember> {
        self.seats
            .iter()
            .enumerate()
            .filter_map(|(slot, s)| {
                let s = s.as_ref()?;
                Some(net::trade::RoomMember { slot: slot as u8, character_id: s.character_id, name: s.name.clone(), look: s.look.clone() })
            })
            .collect()
    }

    fn balloon(&self) -> Option<net::minigame::Balloon> {
        let owner = self.seats[0].as_ref()?;
        Some(net::minigame::Balloon {
            room_type: self.room_type,
            room_id: self.id,
            title: self.title.clone(),
            game_kind: u32::from(self.spec),
            private: self.password.is_some(),
            cur: self.seats.iter().filter(|s| s.is_some()).count() as u8,
            max: net::minigame::OMOK_CAPACITY,
            playing: self.game.is_some(),
            owner_name: owner.name.clone(),
            owner_account: owner.account_id,
            world: owner.world,
        })
    }

    /// Each member's record, in [`GameRoom::members`]'s order.
    fn records(&self) -> Vec<net::minigame::Record> {
        self.seats.iter().flatten().map(|s| s.record).collect()
    }

    fn occupant(&self, seat: usize) -> Option<u32> {
        self.seats[seat].as_ref().map(|s| s.character_id)
    }
}

// ---------------------------------------------------------------------------------------
// The rules, pure so they can be tested without a session
// ---------------------------------------------------------------------------------------

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];

fn at(board: &[[u8; BOARD]; BOARD], x: i32, y: i32) -> Option<u8> {
    if (0..BOARD as i32).contains(&x) && (0..BOARD as i32).contains(&y) {
        Some(board[y as usize][x as usize])
    } else {
        None
    }
}

/// How many `stone`s run through `(x, y)` along `(dx, dy)`, counting `(x, y)` itself.
fn run(board: &[[u8; BOARD]; BOARD], x: i32, y: i32, (dx, dy): (i32, i32), stone: u8) -> usize {
    let mut n = 1;
    for sign in [1, -1] {
        let (mut cx, mut cy) = (x + dx * sign, y + dy * sign);
        while at(board, cx, cy) == Some(stone) {
            n += 1;
            cx += dx * sign;
            cy += dy * sign;
        }
    }
    n
}

/// Five or more in a row through the stone just placed at `(x, y)`.
pub(crate) fn makes_five(board: &[[u8; BOARD]; BOARD], x: usize, y: usize, stone: u8) -> bool {
    DIRECTIONS.iter().any(|&d| run(board, x as i32, y as i32, d, stone) >= 5)
}

/// Whether, along `d`, the stone at `(x, y)` is part of a **three that can become an open
/// four**: some empty point on the line, filled, makes exactly four in a row through `(x, y)`
/// with both ends empty (`.XXXX.`).
fn open_three_along(board: &[[u8; BOARD]; BOARD], x: i32, y: i32, (dx, dy): (i32, i32), stone: u8) -> bool {
    let mut b = *board;
    for k in -4..=4 {
        let (px, py) = (x + dx * k, y + dy * k);
        if k == 0 || at(&b, px, py) != Some(0) {
            continue;
        }
        b[py as usize][px as usize] = stone;
        // The run through the filled point, and whether it includes (x, y) and is open.
        let mut lo = 0;
        while at(&b, px - dx * (lo + 1), py - dy * (lo + 1)) == Some(stone) {
            lo += 1;
        }
        let mut hi = 0;
        while at(&b, px + dx * (hi + 1), py + dy * (hi + 1)) == Some(stone) {
            hi += 1;
        }
        let len = lo + hi + 1;
        let covers = (-lo..=hi).contains(&(k * -1));
        let open = at(&b, px - dx * (lo + 1), py - dy * (lo + 1)) == Some(0) && at(&b, px + dx * (hi + 1), py + dy * (hi + 1)) == Some(0);
        b[py as usize][px as usize] = 0;
        if len == 4 && covers && open {
            return true;
        }
    }
    false
}

/// **The double-three rule** ("You have double-3's."): a stone that would make two open threes
/// at once, in two directions, may not be placed - unless it wins outright. `board` already
/// holds the stone at `(x, y)`.
pub(crate) fn makes_double_three(board: &[[u8; BOARD]; BOARD], x: usize, y: usize, stone: u8) -> bool {
    DIRECTIONS.iter().filter(|&&d| open_three_along(board, x as i32, y as i32, d, stone)).count() >= 2
}

// ---------------------------------------------------------------------------------------
// The session side
// ---------------------------------------------------------------------------------------

/// A packet owed to one seat of a room.
struct Send {
    to: u32,
    body: Vec<u8>,
    what: String,
}

impl Session {
    fn with_games<T>(&self, f: impl FnOnce(&mut Vec<GameRoom>) -> T) -> T {
        f(&mut self.fields.trades().games)
    }

    /// The game room `character` sits in, as `(room id, seat)`.
    fn game_seat_of(&self, character: u32) -> Option<(u32, usize)> {
        self.with_games(|g| g.iter().find_map(|r| Some((r.id, r.seat_of(character)?))))
    }

    /// Whether this character is in a game room - for the busy check and the dispatcher.
    pub(super) fn in_game_room(&self, character: u32) -> bool {
        self.game_seat_of(character).is_some()
    }

    /// The balloon this character's own room puts over their head, for their `0x0224`.
    pub(super) fn own_balloon(&self, character: u32) -> Option<net::minigame::Balloon> {
        self.with_games(|g| g.iter().find(|r| r.id == character).and_then(GameRoom::balloon))
    }

    /// Whether `ticket` names an open game room - mode 3 is shared with a trade accept.
    pub(super) fn is_game_room(&self, ticket: u32) -> bool {
        self.with_games(|g| g.iter().any(|r| r.id == ticket))
    }

    /// Deliver: our own packets come back in the returned vector, the other seat's go through
    /// the bus.
    fn deliver_room(&self, me: u32, sends: Vec<Send>) -> Vec<Reply> {
        let mut out = Vec::new();
        for s in sends {
            let reply = Reply { opcode: net::trade::MINIROOM_RESULT, body: s.body, what: s.what };
            if s.to == me {
                out.push(reply);
            } else if !self.bus().publish_to_character_anywhere(s.to, reply) {
                crate::server::log(&format!("   omok: character {} was not reachable for a room packet.", s.to));
            }
        }
        out
    }

    /// `0x0233` to everyone on the room's map, the owner included.
    fn show_balloon(&self, map: crate::fields::FieldKey, owner: u32, b: Option<&net::minigame::Balloon>) {
        let what = match b {
            Some(b) => format!(
                "UserMiniRoomBalloon: character {owner}'s Omok room {:?}, {}/{}, {}{}",
                b.title,
                b.cur,
                b.max,
                if b.playing { "playing" } else { "waiting" },
                if b.private { ", private" } else { "" }
            ),
            None => format!("UserMiniRoomBalloon: character {owner}'s room is gone - balloon down"),
        };
        let n = self.bus().publish_to_map(
            map,
            Reply { opcode: net::minigame::USER_MINIROOM_BALLOON, body: net::minigame::balloon(owner, b), what },
        );
        crate::server::log(&format!("   omok: balloon for room {owner} sent to {n} player(s) on the map."));
    }

    /// Room `owner`'s balloon changed: their cached `0x0224` is rebuilt - here when it is our
    /// own, through [`crate::broadcast::Event::MiniRoomChanged`] when it is not.
    fn owner_spawn_changed(&mut self, owner: u32) {
        if self.claimed_character().is_some_and(|c| c.id == owner) {
            self.refresh_own_spawn();
        } else {
            self.bus().publish_event_to_character(owner, crate::broadcast::Event::MiniRoomChanged);
        }
    }

    /// `character`'s record at `game` (3 Omok, 4 Match Cards), as the panel draws it. A store
    /// error shows a fresh record rather than refusing the seat.
    fn record_of(&self, character: u32, game: u32) -> net::minigame::Record {
        match self.store.minigame_record(character, game) {
            Ok(r) => to_net(r),
            Err(e) => {
                crate::server::log(&format!("   minigame: character {character}'s record at game {game} did not load ({e}); showing a fresh one."));
                to_net(store::minigame::MiniGameRecord::default())
            }
        }
    }

    /// Rebuild this player's cached `0x0224`, which carries their balloon.
    pub(super) fn refresh_own_spawn(&mut self) {
        let Some(chr) = self.claimed_character() else { return };
        let spawn = self.presence(&chr).spawn;
        self.fields.bus().refresh_spawn(self.subscriber, spawn);
    }

    fn speaker_of(&self, _chr: &net::opcode::Character) -> (u32, u8) {
        let (account_id, world) = self.claimed.as_ref().map(|c| (c.account_id, c.world_id)).unwrap_or_default();
        (u32::try_from(account_id).unwrap_or(0), u8::try_from(world).unwrap_or(0))
    }

    /// **Mode 0, room type 3: open an Omok room.**
    pub(super) fn game_create(&mut self, chr: &net::opcode::Character, room_type: u32, title: String, password: Option<String>, spec: u8) -> Vec<Reply> {
        if let Some(why) = self.busy_for_trade() {
            crate::server::log(&format!("   omok: character {} is busy ({why}); create ignored.", chr.id));
            return Vec::new();
        }
        let (account_id, world) = self.speaker_of(chr);
        let map = self.field_of(chr);
        let room = GameRoom {
            id: chr.id,
            room_type,
            title: title.clone(),
            password,
            spec,
            map,
            seats: [
                Some(GameSeat {
                    character_id: chr.id,
                    name: chr.name.clone(),
                    look: net::opcode::avatar_look(chr),
                    account_id,
                    world,
                    record: self.record_of(chr.id, room_type),
                }),
                None,
            ],
            visitor_ready: false,
            game: None,
            first_next: 0,
            leave_after: [false; 2],
        };
        let open = net::minigame::room_open_game(room.room_type, 0, &room.members(), &room.records(), &room.title, room.spec);
        let balloon = room.balloon();
        let private = room.password.is_some();
        self.with_games(|g| {
            g.retain(|r| r.id != chr.id);
            g.push(room);
        });
        crate::server::log(&format!(
            "   minigame: character {} opened {} room {:?} (spec {spec}{}). Window open, balloon up.",
            chr.id,
            game_name(room_type),
            title,
            if private { ", private" } else { "" }
        ));
        self.show_balloon(map, chr.id, balloon.as_ref());
        self.refresh_own_spawn();
        vec![Reply {
            opcode: net::trade::MINIROOM_RESULT,
            what: format!("MiniroomResult mode 4: {} room {:?} opens for its owner, character {} ({} bytes)", game_name(room_type), title, chr.id, open.len()),
            body: open,
        }]
    }

    /// **Mode 3 for a game room**: a click on the balloon.
    pub(super) fn game_join(&mut self, chr: &net::opcode::Character, room_id: u32, password: Option<String>) -> Vec<Reply> {
        let notice = |code: u32, why: &str| {
            crate::server::log(&format!("   omok: character {} cannot enter room {room_id} - {why} (mode 4 notice {code:#x}).", chr.id));
            vec![Reply {
                opcode: net::trade::MINIROOM_RESULT,
                body: net::trade::room_notice(code),
                what: format!("MiniroomResult mode 4 notice {code:#x}: {why}"),
            }]
        };
        if self.in_game_room(chr.id) {
            return notice(net::trade::ROOM_NOTICE_FULL, "already in a room");
        }
        let (account_id, world) = self.speaker_of(chr);
        let map = self.field_of(chr);
        let Some(game) = self.with_games(|g| g.iter().find(|r| r.id == room_id).map(|r| r.room_type)) else {
            return notice(net::trade::ROOM_NOTICE_CLOSED, "the room is gone");
        };
        let me = GameSeat {
            character_id: chr.id,
            name: chr.name.clone(),
            look: net::opcode::avatar_look(chr),
            account_id,
            world,
            record: self.record_of(chr.id, game),
        };
        let joined = self.with_games(|g| {
            let Some(r) = g.iter_mut().find(|r| r.id == room_id) else { return Err((net::trade::ROOM_NOTICE_CLOSED, "the room is gone")) };
            if r.map != map {
                return Err((net::trade::ROOM_NOTICE_CLOSED, "the room is on another map"));
            }
            if r.seats[1].is_some() {
                return Err((net::trade::ROOM_NOTICE_FULL, "both seats are taken"));
            }
            if r.password.is_some() && r.password != password {
                return Err((net::minigame::ROOM_NOTICE_BAD_PASSWORD, "wrong password"));
            }
            r.seats[1] = Some(me);
            r.visitor_ready = false;
            r.leave_after = [false; 2];
            Ok(r.clone())
        });
        let room = match joined {
            Ok(r) => r,
            Err((code, why)) => return notice(code, why),
        };
        let members = room.members();
        let visitor = members.iter().find(|m| m.slot == 1).cloned().expect("just seated");
        let owner = room.id;
        crate::server::log(&format!("   omok: character {} sat down in room {owner} ({:?}).", chr.id, room.title));
        let out = self.deliver_room(
            chr.id,
            vec![
                Send {
                    to: owner,
                    body: net::minigame::visitor_entered(&visitor, &room.seats[1].as_ref().expect("just seated").record),
                    what: format!("MiniroomResult mode 3: {} sat down in the owner's Omok room", chr.name),
                },
                Send {
                    to: chr.id,
                    body: net::minigame::room_open_game(room.room_type, 1, &members, &room.records(), &room.title, room.spec),
                    what: format!("MiniroomResult mode 4: Omok room {:?} opens for the visitor, character {}", room.title, chr.id),
                },
            ],
        );
        self.show_balloon(room.map, owner, room.balloon().as_ref());
        self.owner_spawn_changed(owner);
        out
    }

    /// Mode 8 in a game room: the line to both seats.
    pub(super) fn game_chat(&mut self, chr: &net::opcode::Character, text: &str) -> Vec<Reply> {
        let Some((room_id, seat)) = self.game_seat_of(chr.id) else { return Vec::new() };
        let other = self.with_games(|g| g.iter().find(|r| r.id == room_id).and_then(|r| r.occupant(1 - seat)));
        let (account_id, world) = self.speaker_of(chr);
        let who = net::megaphone::Speaker { name: &chr.name, account_id, character_id: chr.id, world };
        let body = net::trade::chat(seat as u8, &who, other.unwrap_or(0), text);
        let mut sends = vec![Send { to: chr.id, body: body.clone(), what: format!("MiniroomResult mode 8: Omok room chat from {}: {text:?}", chr.name) }];
        if let Some(o) = other {
            sends.push(Send { to: o, body, what: format!("MiniroomResult mode 8: Omok room chat from {}: {text:?}", chr.name) });
        }
        self.deliver_room(chr.id, sends)
    }

    /// **Mode 0x0C**, or the connection going: out of the room. Mid-game it forfeits first.
    pub(super) fn game_leave(&mut self, chr: &net::opcode::Character, why: &str) -> Vec<Reply> {
        let Some((room_id, seat)) = self.game_seat_of(chr.id) else { return Vec::new() };
        let store = self.store.clone();
        let mut sends = self.with_games(|g| {
            let r = g.iter_mut().find(|r| r.id == room_id).expect("seated");
            let mut sends = Vec::new();
            if r.game.is_some() {
                sends.extend(finish(r, Some(1 - seat), &format!("{} left mid-game", chr.name), &store));
            }
            sends
        });
        // The leaver's window has already closed itself (`FUN_141E9C770` sends 0x0C, then
        // `vt+0x138`), so nothing more goes to them.
        sends.extend(self.remove_seat(room_id, seat, net::minigame::LEAVE_LEFT, false));
        crate::server::log(&format!("   omok: character {} {why} - left room {room_id} from seat {seat}.", chr.id));
        let out = self.deliver_room(chr.id, sends);
        self.owner_spawn_changed(room_id);
        out
    }

    /// Take seat `seat` out of room `room_id`. The owner's seat closes the room: the visitor is
    /// told "The room is closed." and the balloon comes down. The visitor's frees the seat.
    /// `tell_leaver` sends the leaver their own 0x0C with `reason` (an expel, a leave after the
    /// game) - not when their window closed itself.
    fn remove_seat(&mut self, room_id: u32, seat: usize, reason: u32, tell_leaver: bool) -> Vec<Send> {
        let Some(room) = self.with_games(|g| {
            let i = g.iter().position(|r| r.id == room_id)?;
            let r = &mut g[i];
            let before = r.clone();
            if seat == 0 {
                g.remove(i);
            } else {
                r.seats[1] = None;
                r.visitor_ready = false;
                r.leave_after[1] = false;
            }
            Some(before)
        }) else {
            return Vec::new();
        };
        let mut sends = Vec::new();
        if tell_leaver {
            if let Some(me) = room.occupant(seat) {
                sends.push(Send { to: me, body: net::trade::room_leave(seat as u8, reason), what: format!("MiniroomResult mode 0x0C: out of Omok room {room_id}, reason {reason}") });
            }
        }
        if seat == 0 {
            if let Some(v) = room.occupant(1) {
                sends.push(Send {
                    to: v,
                    body: net::trade::room_leave(1, net::minigame::LEAVE_ROOM_CLOSED),
                    what: format!("MiniroomResult mode 0x0C: Omok room {room_id} closed by its owner - \"The room is closed.\""),
                });
            }
            self.show_balloon(room.map, room_id, None);
        } else {
            sends.push(Send {
                to: room_id,
                body: net::trade::room_leave(1, reason),
                what: format!("MiniroomResult mode 0x0C: the visitor left Omok room {room_id} (reason {reason})"),
            });
            let after = self.with_games(|g| g.iter().find(|r| r.id == room_id).and_then(GameRoom::balloon));
            self.show_balloon(room.map, room_id, after.as_ref());
        }
        sends
    }

    /// **Every in-room action** (`net::minigame::Action`).
    pub(super) fn game_action(&mut self, chr: &net::opcode::Character, action: net::minigame::Action) -> Vec<Reply> {
        use net::minigame::Action;
        let Some((room_id, seat)) = self.game_seat_of(chr.id) else {
            crate::server::log(&format!("   omok: character {} sent {action:?} with no room; ignored.", chr.id));
            return Vec::new();
        };
        let other = 1 - seat;
        let name = chr.name.clone();
        // Expel goes through `remove_seat`, which needs `self`.
        if action == Action::Expel {
            let can = self.with_games(|g| g.iter().find(|r| r.id == room_id).is_some_and(|r| seat == 0 && r.seats[1].is_some() && r.game.is_none()));
            if !can {
                crate::server::log(&format!("   omok: character {} asked to expel with nobody to expel, or mid-game; ignored.", chr.id));
                return Vec::new();
            }
            crate::server::log(&format!("   omok: character {} expelled the visitor of room {room_id}.", chr.id));
            let sends = self.remove_seat(room_id, 1, net::minigame::LEAVE_EXPELLED, true);
            let out = self.deliver_room(chr.id, sends);
            self.refresh_own_spawn();
            return out;
        }
        let seed = self.rng.next();
        let store = self.store.clone();
        let (sends, balloon, leavers) = self.with_games(|g| {
            let r = g.iter_mut().find(|r| r.id == room_id).expect("seated");
            let before = r.game.is_some();
            let sends = step(r, seat, other, &name, &action, seed, &store);
            let changed = before != r.game.is_some();
            let leavers: Vec<usize> = if before && r.game.is_none() { (0..2).filter(|&s| r.leave_after[s] && r.seats[s].is_some()).collect() } else { Vec::new() };
            (sends, changed.then(|| (r.map, r.balloon())), leavers)
        });
        let mut sends = sends;
        if let Some((map, b)) = &balloon {
            self.show_balloon(*map, room_id, b.as_ref());
        }
        // "Leave after this game": the owner's leaving closes the room for both.
        if leavers.contains(&0) {
            sends.extend(self.remove_seat(room_id, 0, net::minigame::LEAVE_AFTER_GAME, true));
        } else if leavers.contains(&1) {
            sends.extend(self.remove_seat(room_id, 1, net::minigame::LEAVE_AFTER_GAME, true));
        }
        let out = self.deliver_room(chr.id, sends);
        if balloon.is_some() || !leavers.is_empty() {
            self.owner_spawn_changed(room_id);
        }
        out
    }

    /// From `Drop`.
    pub(super) fn leave_game_on_disconnect(&mut self) {
        if let Some(chr) = self.claimed_character() {
            if self.in_game_room(chr.id) {
                let _ = self.game_leave(&chr, "left the channel");
            }
        }
    }
}

fn game_name(room_type: u32) -> &'static str {
    if room_type == net::minigame::ROOM_TYPE_MATCH_CARDS {
        "Match Cards"
    } else {
        "Omok"
    }
}

/// Both seats of `r`, each with `body`.
fn both(r: &GameRoom, body: Vec<u8>, what: &str) -> Vec<Send> {
    r.seats.iter().flatten().map(|s| Send { to: s.character_id, body: body.clone(), what: what.to_string() }).collect()
}

fn one(r: &GameRoom, seat: usize, body: Vec<u8>, what: &str) -> Vec<Send> {
    r.occupant(seat).map(|to| Send { to, body, what: what.to_string() }).into_iter().collect()
}

fn to_net(r: store::minigame::MiniGameRecord) -> net::minigame::Record {
    net::minigame::Record { wins: r.wins, ties: r.ties, losses: r.losses, points: r.points }
}

/// Whether `stone` may go on the empty `(x, y)`: anything that makes five, and anything else
/// that is not a double three.
fn legal(cells: &[[u8; BOARD]; BOARD], x: usize, y: usize, stone: u8) -> bool {
    let mut b = *cells;
    b[y][x] = stone;
    makes_five(&b, x, y, stone) || !makes_double_three(&b, x, y, stone)
}

/// **The stone the server plays for a player whose clock ran out**: a legal empty square
/// next to a stone already on the board (any legal one if none is), picked with `seed`. `None`
/// only when no square is legal.
pub(crate) fn auto_stone(cells: &[[u8; BOARD]; BOARD], stone: u8, seed: u64) -> Option<(usize, usize)> {
    let near = |x: usize, y: usize| {
        (-1i32..=1).any(|dy| (-1i32..=1).any(|dx| at(cells, x as i32 + dx, y as i32 + dy).is_some_and(|c| c != 0)))
    };
    let empty: Vec<(usize, usize)> = (0..BOARD).flat_map(|y| (0..BOARD).map(move |x| (x, y))).filter(|&(x, y)| cells[y][x] == 0 && legal(cells, x, y, stone)).collect();
    let close: Vec<(usize, usize)> = empty.iter().copied().filter(|&(x, y)| near(x, y)).collect();
    let pool = if close.is_empty() { &empty } else { &close };
    let mut state = seed;
    (!pool.is_empty()).then(|| pool[(crate::config::splitmix64(&mut state) % pool.len() as u64) as usize])
}

/// Put `seat`'s stone on the legal, empty `(x, y)`: to both clients (which hand the turn over
/// by themselves on it), then five in a row or a full board ends the game.
fn place(r: &mut GameRoom, seat: usize, x: usize, y: usize, who: &str, store: &store::Store) -> Vec<Send> {
    let g = r.game.as_mut().expect("a game");
    let Board::Omok { cells, stones, moves } = &mut g.board else { return Vec::new() };
    let stone = stones[seat];
    cells[y][x] = stone;
    let five = makes_five(cells, x, y, stone);
    moves.push((x, y, seat));
    let full = moves.len() == BOARD * BOARD;
    g.tie_asked_by = None;
    g.undo_asked_by = None;
    g.turn = 1 - seat;
    g.clock = Instant::now();
    let mut sends = both(r, net::minigame::stone(x as u32, y as u32, stone), &format!("MiniroomResult 0x1F: {who} put stone {stone} at ({x}, {y})"));
    if five {
        sends.extend(finish(r, Some(seat), &format!("{who} made five"), store));
    } else if full {
        sends.extend(finish(r, None, "the board is full", store));
    }
    sends
}

/// The game ends: both records counted and stored (per game - an Omok result never touches the
/// Match Cards record), the result to both with the records after it, and the next game's
/// first mover is the loser.
fn finish(r: &mut GameRoom, winner: Option<usize>, why: &str, store: &store::Store) -> Vec<Send> {
    let Some(game) = r.game.take() else { return Vec::new() };
    for seat in 0..2 {
        let outcome = match winner {
            None => store::minigame::Outcome::Tie,
            Some(w) if w == seat => store::minigame::Outcome::Win,
            Some(_) => store::minigame::Outcome::Loss,
        };
        let room_type = r.room_type;
        if let Some(s) = r.seats[seat].as_mut() {
            match store.record_minigame_result(s.character_id, room_type, outcome) {
                Ok(rec) => s.record = to_net(rec),
                Err(e) => crate::server::log(&format!("   minigame: {}'s {outcome:?} at game {room_type} was not stored ({e}).", s.name)),
            }
        }
    }
    let records = [r.seats[0].as_ref().map(|s| s.record).unwrap_or_default(), r.seats[1].as_ref().map(|s| s.record).unwrap_or_default()];
    r.visitor_ready = false;
    r.first_next = match winner {
        Some(w) => 1 - w,
        None => 1 - game.first,
    };
    let played = match &game.board {
        Board::Omok { moves, .. } => format!("{} stone(s)", moves.len()),
        Board::Cards { pairs, .. } => format!("{} - {} pairs", pairs[0], pairs[1]),
    };
    crate::server::log(&format!(
        "   minigame: room {} game over after {played} - {}. {}",
        r.id,
        match winner {
            Some(w) => format!("seat {w} wins"),
            None => "a draw".into(),
        },
        why
    ));
    match winner {
        Some(w) => both(r, net::minigame::result_win(w as u8, records), &format!("MiniroomResult 0x1D: seat {w} wins ({why}); records {records:?}")),
        None => both(r, net::minigame::result_draw(records), &format!("MiniroomResult 0x1D: a draw ({why}); records {records:?}")),
    }
}

/// One action from `seat` in room `r`. `seed` is fresh randomness, for a deal.
fn step(r: &mut GameRoom, seat: usize, other: usize, name: &str, action: &net::minigame::Action, seed: u64, store: &store::Store) -> Vec<Send> {
    use net::minigame::Action;
    let playing = r.game.is_some();
    match *action {
        Action::OwnerOpen => {
            crate::server::log(&format!("   minigame: room {} owner sent mode 0x0A (u8 1) from the room open - noted.", r.id));
            Vec::new()
        }
        Action::Ready { on } => {
            if seat != 1 || playing {
                return Vec::new();
            }
            r.visitor_ready = on;
            both(r, net::minigame::ready(on), &format!("MiniroomResult {}: {name} is {}", if on { "0x19" } else { "0x1A" }, if on { "ready" } else { "not ready" }))
        }
        Action::LeaveAfterGame { on } => {
            r.leave_after[seat] = on;
            Vec::new()
        }
        Action::Start => {
            if seat != 0 || playing || r.seats[1].is_none() || !r.visitor_ready {
                crate::server::log(&format!("   minigame: room {} start refused - owner {}, visitor {}, ready {}.", r.id, seat == 0, r.seats[1].is_some(), r.visitor_ready));
                return Vec::new();
            }
            let first = r.first_next;
            let second = 1 - first;
            r.visitor_ready = false;
            if r.room_type == net::minigame::ROOM_TYPE_MATCH_CARDS {
                let count = net::minigame::CARD_COUNTS.get(usize::from(r.spec)).copied().unwrap_or(net::minigame::CARD_COUNTS[0]);
                let mut state = seed;
                let faces = deal_cards(usize::from(count), || crate::config::splitmix64(&mut state));
                let body = net::minigame::deal(second as u8, &faces);
                r.game = Some(Game {
                    turn: first,
                    first,
                    clock: Instant::now(),
                    tie_asked_by: None,
                    undo_asked_by: None,
                    board: Board::Cards { faces: faces.into_iter().map(Some).collect(), turned: None, pairs: [0; 2] },
                });
                crate::server::log(&format!("   minigame: room {} Match Cards starts with {count} cards; seat {first} turns first.", r.id));
                return both(r, body, &format!("MiniroomResult 0x1C: Match Cards dealt, {count} cards, seat {first} first"));
            }
            let mut stones = [0u8; 2];
            stones[first] = net::minigame::STONE_FIRST;
            stones[second] = net::minigame::STONE_SECOND;
            r.game = Some(Game {
                turn: first,
                first,
                clock: Instant::now(),
                tie_asked_by: None,
                undo_asked_by: None,
                board: Board::Omok { cells: [[0; BOARD]; BOARD], stones, moves: Vec::new() },
            });
            crate::server::log(&format!("   minigame: room {} Omok starts; seat {first} moves first.", r.id));
            both(r, net::minigame::start(second as u8), &format!("MiniroomResult 0x1C: the game starts, seat {first} first (the byte names seat {second}, who moves second)"))
        }
        Action::Move { x, y, .. } => {
            let Some(g) = r.game.as_mut() else { return Vec::new() };
            if g.turn != seat {
                crate::server::log(&format!("   omok: room {} seat {seat} moved out of turn; ignored.", r.id));
                return Vec::new();
            }
            let Board::Omok { cells, stones, .. } = &g.board else { return Vec::new() };
            let (xu, yu) = (x as usize, y as usize);
            if xu >= BOARD || yu >= BOARD || cells[yu][xu] != 0 {
                return one(r, seat, net::minigame::bad_move(net::minigame::BAD_MOVE_OCCUPIED), &format!("MiniroomResult 0x20: ({x}, {y}) cannot take a stone"));
            }
            if !legal(cells, xu, yu, stones[seat]) {
                return one(r, seat, net::minigame::bad_move(net::minigame::BAD_MOVE_DOUBLE_THREE), &format!("MiniroomResult 0x20: ({x}, {y}) is a double three"));
            }
            place(r, seat, xu, yu, name, store)
        }
        Action::Card { first, index } => {
            let Some(g) = r.game.as_mut() else { return Vec::new() };
            if g.turn != seat {
                crate::server::log(&format!("   cards: room {} seat {seat} turned a card out of turn; ignored.", r.id));
                return Vec::new();
            }
            let Board::Cards { faces, turned, pairs } = &mut g.board else { return Vec::new() };
            let i = usize::from(index);
            if faces.get(i).copied().flatten().is_none() {
                crate::server::log(&format!("   cards: room {} seat {seat} turned card {i}, which is gone or off the board; ignored.", r.id));
                return Vec::new();
            }
            if first {
                *turned = Some(i);
                // The clicker's client turned it already; only the opponent needs telling.
                return one(r, other, net::minigame::card_first(index), &format!("MiniroomResult 0x23: {name} turned card {i}"));
            }
            let Some(a) = turned.take().filter(|&a| a != i) else {
                crate::server::log(&format!("   cards: room {} seat {seat} sent a second card {i} with no first; ignored.", r.id));
                return Vec::new();
            };
            g.tie_asked_by = None;
            g.clock = Instant::now();
            if faces[a] == faces[i] {
                faces[a] = None;
                faces[i] = None;
                pairs[seat] += 1;
                let done = faces.iter().all(Option::is_none);
                let score = *pairs;
                let mut sends = both(
                    r,
                    net::minigame::card_second(index, a as u8, net::minigame::card_match(seat as u8)),
                    &format!("MiniroomResult 0x23: {name} matched cards {a} and {i} ({} - {})", score[0], score[1]),
                );
                if done {
                    let winner = match score[0].cmp(&score[1]) {
                        std::cmp::Ordering::Greater => Some(0),
                        std::cmp::Ordering::Less => Some(1),
                        std::cmp::Ordering::Equal => None,
                    };
                    sends.extend(finish(r, winner, &format!("every pair found, {} - {}", score[0], score[1]), store));
                }
                sends
            } else {
                g.turn = other;
                both(
                    r,
                    net::minigame::card_second(index, a as u8, net::minigame::card_miss(seat as u8)),
                    &format!("MiniroomResult 0x23: {name} missed with cards {a} and {i}; seat {other}'s turn"),
                )
            }
        }
        Action::TimeUp => {
            let Some(g) = r.game.as_mut() else { return Vec::new() };
            // Both clients report it: the first report counts, whoever sends it, and the
            // second is a duplicate (`DUPLICATE_TIME_UP`).
            if g.clock.elapsed() < DUPLICATE_TIME_UP {
                crate::server::log(&format!("   minigame: room {} seat {seat}'s time-up report came {} ms after the clock restarted - the other client's copy; ignored.", r.id, g.clock.elapsed().as_millis()));
                return Vec::new();
            }
            let late = g.turn;
            let next = 1 - late;
            let late_name = r.seats[late].as_ref().map(|s| s.name.clone()).unwrap_or_default();
            let g = r.game.as_mut().expect("checked");
            match &mut g.board {
                // The owner, 2026-10-04: "Whenever the user times out on a turn, the server should
                // automatically make a move for them and skip that timed out user's turn." A
                // stone for them, which also hands the turn over on both clients.
                Board::Omok { cells, stones, .. } => {
                    if let Some((x, y)) = auto_stone(cells, stones[late], seed) {
                        crate::server::log(&format!("   omok: room {} {late_name}'s clock ran out - the server plays ({x}, {y}) for them.", r.id));
                        return place(r, late, x, y, &format!("{late_name} (timed out; placed by the server)"), store);
                    }
                }
                Board::Cards { turned, .. } => *turned = None,
            }
            // Match Cards (or a board with no legal square left): the turn passes. A first
            // card already turned goes face down again on 0x1E.
            g.turn = next;
            g.clock = Instant::now();
            both(r, net::minigame::turn(next as u8), &format!("MiniroomResult 0x1E: {late_name}'s clock ran out; seat {next}'s turn"))
        }
        Action::TieRequest => {
            let Some(g) = r.game.as_mut() else { return Vec::new() };
            g.tie_asked_by = Some(seat);
            one(r, other, net::minigame::tie_request(), &format!("MiniroomResult 0x11: {name} asks for a tie"))
        }
        Action::TieAnswer { yes } => {
            let Some(g) = r.game.as_mut() else { return Vec::new() };
            if g.tie_asked_by != Some(other) {
                return Vec::new();
            }
            g.tie_asked_by = None;
            if yes {
                finish(r, None, "a tie was agreed", store)
            } else {
                one(r, other, net::minigame::tie_refused(), &format!("MiniroomResult 0x12: {name} refused the tie"))
            }
        }
        Action::Forfeit => {
            if !playing {
                return Vec::new();
            }
            finish(r, Some(other), &format!("{name} gave up"), store)
        }
        Action::UndoRequest => {
            let Some(g) = r.game.as_mut() else { return Vec::new() };
            let Board::Omok { moves, .. } = &g.board else { return Vec::new() };
            if !moves.iter().any(|m| m.2 == seat) {
                return Vec::new();
            }
            g.undo_asked_by = Some(seat);
            one(r, other, net::minigame::undo_request(), &format!("MiniroomResult 0x15: {name} asks to take a move back"))
        }
        Action::UndoAnswer { yes } => {
            let Some(g) = r.game.as_mut() else { return Vec::new() };
            if g.undo_asked_by != Some(other) {
                return Vec::new();
            }
            g.undo_asked_by = None;
            if !yes {
                return one(r, other, net::minigame::undo_denied(), &format!("MiniroomResult 0x16: {name} refused the take-back"));
            }
            let Board::Omok { cells, moves, .. } = &mut g.board else { return Vec::new() };
            // Off the top until the asker's last stone is gone: one stone if it was the last
            // move, two if the answerer has moved since.
            let mut count = 0u8;
            while let Some((x, y, s)) = moves.pop() {
                cells[y][x] = 0;
                count += 1;
                if s == other {
                    break;
                }
            }
            g.turn = other;
            g.clock = Instant::now();
            both(r, net::minigame::undo_accepted(count, other as u8), &format!("MiniroomResult 0x16: take-back granted, {count} stone(s) off, seat {other}'s turn"))
        }
        Action::Expel => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    const MAP: u32 = 104_040_000;

    fn channel() -> (Arc<store::Store>, Arc<crate::config::Config>, Arc<crate::fields::Fields>) {
        let store = Arc::new(store::Store::open_in_memory().unwrap());
        (store, Arc::new(crate::config::Config::default()), Arc::new(crate::fields::Fields::new()))
    }

    fn join(store: &Arc<store::Store>, config: &Arc<crate::config::Config>, fields: &Arc<crate::fields::Fields>, name: &str) -> (Session, u32) {
        let account = store.create_account(&name.to_lowercase(), "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: name.to_string(), map_id: MAP, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
        s.claim_for_character(id);
        s.on_field_entered();
        s.collect_mail();
        (s, id)
    }

    fn miniroom(body: &[u8]) -> Vec<u8> {
        let mut b = net::trade::CLIENT_MINIROOM.to_le_bytes().to_vec();
        b.extend_from_slice(body);
        b
    }

    fn mode(m: u32) -> Vec<u8> {
        miniroom(&m.to_le_bytes())
    }

    /// The 2026-10-04 create, byte for byte: Omok, "hello", public, set 0.
    fn create() -> Vec<u8> {
        miniroom(&[0, 0, 0, 0, 3, 0, 0, 0, 5, 0, b'h', b'e', b'l', b'l', b'o', 0, 0])
    }

    fn visit(room: u32) -> Vec<u8> {
        let mut b = 3u32.to_le_bytes().to_vec();
        b.extend_from_slice(&room.to_le_bytes());
        b.extend_from_slice(&[0, 0]);
        miniroom(&b)
    }

    fn mv(x: u32, y: u32) -> Vec<u8> {
        let mut b = net::minigame::OMOK_MOVE.to_le_bytes().to_vec();
        b.extend_from_slice(&x.to_le_bytes());
        b.extend_from_slice(&y.to_le_bytes());
        b.push(1);
        miniroom(&b)
    }

    fn of(out: &[Reply], opcode: u16) -> Vec<Vec<u8>> {
        out.iter().filter(|r| r.opcode == opcode).map(|r| r.body.clone()).collect()
    }

    fn results(out: &[Reply]) -> Vec<Vec<u8>> {
        of(out, net::trade::MINIROOM_RESULT)
    }

    /// A first game's records: `winner` at 1 win, the other at 1 loss.
    fn after_first_game(winner: usize) -> [net::minigame::Record; 2] {
        let start = store::minigame::POINTS_START;
        let step = store::minigame::POINTS_STEP;
        let won = net::minigame::Record { wins: 1, ties: 0, losses: 0, points: start + step };
        let lost = net::minigame::Record { wins: 0, ties: 0, losses: 1, points: start - step };
        if winner == 0 {
            [won, lost]
        } else {
            [lost, won]
        }
    }

    /// Owner and visitor seated, the visitor ready and the game started.
    fn playing(store: &Arc<store::Store>, config: &Arc<crate::config::Config>, fields: &Arc<crate::fields::Fields>) -> (Session, u32, Session, u32) {
        let (mut host, host_id) = join(store, config, fields, "Tester2");
        let (mut guest, guest_id) = join(store, config, fields, "Wisp");
        host.handle(&create());
        guest.handle(&visit(host_id));
        guest.handle(&mode(net::minigame::MG_READY));
        host.handle(&mode(net::minigame::MG_START));
        guest.collect_mail();
        host.collect_mail();
        (host, host_id, guest, guest_id)
    }

    #[test]
    fn a_create_opens_the_window_and_puts_up_the_balloon() {
        let (store, config, fields) = channel();
        let (mut host, host_id) = join(&store, &config, &fields, "Tester2");
        let (mut other, _) = join(&store, &config, &fields, "Wisp");
        let out = host.handle(&create());
        let opens = results(&out);
        assert_eq!(opens.len(), 1, "the owner's window: {opens:?}");
        assert_eq!(&opens[0][..12], &[4, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0], "mode 4, A 0, B 3 = Omok");
        assert!(opens[0].ends_with(&[0xFF, 5, 0, b'h', b'e', b'l', b'l', b'o', 0]), "the game tail: records, title, set");
        // The balloon reaches the owner AND everyone else on the map.
        let mine = of(&out, net::minigame::USER_MINIROOM_BALLOON);
        assert_eq!(mine.len(), 1);
        assert_eq!(&mine[0][..12], &[&host_id.to_le_bytes()[..], &[3, 0, 0, 0], &host_id.to_le_bytes()[..]].concat());
        assert_eq!(of(&other.collect_mail(), net::minigame::USER_MINIROOM_BALLOON).len(), 1, "the other player on the map sees it");
        // A late arrival's copy of the owner carries the room.
        let spawn = host.presence(&host.claimed_character().unwrap()).spawn.body;
        assert!(spawn.windows(5).any(|w| w == b"hello"), "the 0x0224 carries the balloon");
    }

    #[test]
    fn a_visitor_sits_down_and_both_windows_know() {
        let (store, config, fields) = channel();
        let (mut host, host_id) = join(&store, &config, &fields, "Tester2");
        let (mut guest, _) = join(&store, &config, &fields, "Wisp");
        host.handle(&create());
        host.collect_mail();
        let out = guest.handle(&visit(host_id));
        let opens = results(&out);
        assert_eq!(opens.len(), 1);
        assert_eq!(&opens[0][8..18], &[3, 0, 0, 0, 0, 0, 0, 0, 2, 1], "Omok, room+0x308, capacity 2, my slot 1");
        let host_mail = host.collect_mail();
        let entered = results(&host_mail);
        assert_eq!(entered.len(), 1);
        assert_eq!(&entered[0][..5], &[3, 0, 0, 0, 1], "mode 3, seat 1");
        let mut w = net::PacketWriter::new();
        w.u16(0);
        net::minigame::Record { points: store::minigame::POINTS_START, ..Default::default() }.write(&mut w);
        assert!(entered[0].ends_with(&w.into_vec()), "the u16 then the visitor's 20-byte record: 1, 0 W, 0 D, 0 L, 2000 PTS");
        assert_eq!(of(&host_mail, net::minigame::USER_MINIROOM_BALLOON).len(), 1, "the balloon now says 2/2");
    }

    #[test]
    fn a_private_room_wants_its_password() {
        let (store, config, fields) = channel();
        let (mut host, host_id) = join(&store, &config, &fields, "Tester2");
        let (mut guest, _) = join(&store, &config, &fields, "Wisp");
        host.handle(&miniroom(&[0, 0, 0, 0, 3, 0, 0, 0, 1, 0, b'r', 1, 2, 0, b'p', b'w', 3]));
        let wrong = results(&guest.handle(&visit(host_id)));
        assert_eq!(wrong, vec![net::trade::room_notice(net::minigame::ROOM_NOTICE_BAD_PASSWORD)]);
        let mut b = 3u32.to_le_bytes().to_vec();
        b.extend_from_slice(&host_id.to_le_bytes());
        b.extend_from_slice(&[1, 2, 0, b'p', b'w', 0]);
        let right = results(&guest.handle(&miniroom(&b)));
        assert_eq!(&right[0][..4], &[4, 0, 0, 0], "the window opens");
    }

    #[test]
    fn ready_is_echoed_to_both_and_start_names_the_second_player() {
        let (store, config, fields) = channel();
        let (mut host, host_id) = join(&store, &config, &fields, "Tester2");
        let (mut guest, _) = join(&store, &config, &fields, "Wisp");
        host.handle(&create());
        guest.handle(&visit(host_id));
        host.collect_mail();
        assert!(results(&host.handle(&mode(net::minigame::MG_START))).is_empty(), "not before the visitor is ready");
        let out = guest.handle(&mode(net::minigame::MG_READY));
        assert_eq!(results(&out), vec![net::minigame::ready(true)], "the sender's own button waits for this");
        assert_eq!(results(&host.collect_mail()), vec![net::minigame::ready(true)]);
        let out = host.handle(&mode(net::minigame::MG_START));
        assert_eq!(results(&out), vec![net::minigame::start(1)], "the owner moves first, so the byte names seat 1");
        assert_eq!(results(&guest.collect_mail()), vec![net::minigame::start(1)]);
    }

    #[test]
    fn five_in_a_row_wins_and_turns_are_enforced() {
        let (store, config, fields) = channel();
        let (mut host, _, mut guest, _) = playing(&store, &config, &fields);
        assert!(results(&guest.handle(&mv(0, 0))).is_empty(), "not the visitor's turn");
        for i in 0..4 {
            assert_eq!(results(&host.handle(&mv(i, 7))), vec![net::minigame::stone(i, 7, 1)]);
            guest.collect_mail();
            assert_eq!(results(&guest.handle(&mv(i, 9))), vec![net::minigame::stone(i, 9, 2)]);
            host.collect_mail();
        }
        // An occupied square is refused to the mover only.
        assert_eq!(results(&host.handle(&mv(0, 7))), vec![net::minigame::bad_move(net::minigame::BAD_MOVE_OCCUPIED)]);
        let out = host.handle(&mv(4, 7));
        let result = net::minigame::result_win(0, after_first_game(0));
        assert_eq!(results(&out), vec![net::minigame::stone(4, 7, 1), result.clone()], "the result carries both records after the game");
        assert_eq!(results(&guest.collect_mail()), vec![net::minigame::stone(4, 7, 1), result]);
    }

    #[test]
    fn an_undo_takes_the_askers_stone_back() {
        let (store, config, fields) = channel();
        let (mut host, _, mut guest, _) = playing(&store, &config, &fields);
        host.handle(&mv(7, 7));
        guest.collect_mail();
        guest.handle(&mv(8, 8));
        host.collect_mail();
        // The owner asks; the visitor has moved since, so two stones come off.
        assert!(results(&host.handle(&mode(net::minigame::MG_UNDO_REQUEST))).is_empty());
        assert_eq!(results(&guest.collect_mail()), vec![net::minigame::undo_request()]);
        let out = guest.handle(&miniroom(&[0x16, 0, 0, 0, 1]));
        assert_eq!(results(&out), vec![net::minigame::undo_accepted(2, 0)]);
        assert_eq!(results(&host.collect_mail()), vec![net::minigame::undo_accepted(2, 0)]);
        assert_eq!(results(&host.handle(&mv(7, 7))), vec![net::minigame::stone(7, 7, 1)], "the square is free again and it is the owner's turn");
    }

    #[test]
    fn leaving_mid_game_forfeits_and_the_owner_leaving_closes_the_room() {
        let (store, config, fields) = channel();
        let (mut host, host_id, mut guest, guest_id) = playing(&store, &config, &fields);
        host.handle(&mode(net::trade::ROOM_LEAVE));
        let mail = guest.collect_mail();
        assert_eq!(
            results(&mail),
            vec![net::minigame::result_win(1, after_first_game(1)), net::trade::room_leave(1, net::minigame::LEAVE_ROOM_CLOSED)],
            "the visitor wins, then is told the room is closed"
        );
        assert_eq!(of(&mail, net::minigame::USER_MINIROOM_BALLOON).last().unwrap(), &net::minigame::balloon(host_id, None), "the balloon comes down");
        assert!(!guest.in_game_room(guest_id) && !host.in_game_room(host_id));
    }

    #[test]
    fn a_deal_is_pairs_of_the_clients_faces() {
        let mut state = 7u64;
        for count in [12usize, 20, 30] {
            let faces = deal_cards(count, || crate::config::splitmix64(&mut state));
            assert_eq!(faces.len(), count);
            let mut sorted = faces.clone();
            sorted.sort_unstable();
            for pair in sorted.chunks(2) {
                assert_eq!(pair[0], pair[1], "every face twice: {faces:?}");
            }
            assert!(faces.iter().all(|&f| f < net::minigame::CARD_FACES), "only faces the client has art for");
        }
    }

    fn card(first: bool, index: u8) -> Vec<u8> {
        let mut b = net::minigame::MC_CARD.to_le_bytes().to_vec();
        b.extend_from_slice(&[u8::from(first), index]);
        miniroom(&b)
    }

    /// Match Cards seated, ready and dealt (12 cards): `(owner, visitor, faces)`.
    fn dealt(store: &Arc<store::Store>, config: &Arc<crate::config::Config>, fields: &Arc<crate::fields::Fields>) -> (Session, Session, Vec<u32>) {
        let (mut host, host_id) = join(store, config, fields, "Tester2");
        let (mut guest, _) = join(store, config, fields, "Wisp");
        // Match Cards, "cards", no password, board size 0 = 12 cards.
        let open = results(&host.handle(&miniroom(&[0, 0, 0, 0, 4, 0, 0, 0, 5, 0, b'c', b'a', b'r', b'd', b's', 0, 0])));
        assert_eq!(&open[0][8..12], &4u32.to_le_bytes(), "a Match Cards window");
        guest.handle(&visit(host_id));
        guest.handle(&mode(net::minigame::MG_READY));
        let out = host.handle(&mode(net::minigame::MG_START));
        let deal = results(&out).into_iter().find(|b| b[..4] == [0x1C, 0, 0, 0]).expect("the deal");
        assert_eq!(deal.len(), 54, "u32 mode, u8 second, u8 count, 12 faces");
        assert_eq!(deal[4], 1, "the owner turns first, so the byte names seat 1");
        let faces: Vec<u32> = deal[6..].chunks(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
        assert_eq!(results(&guest.collect_mail()).last(), Some(&deal), "both get the same deal");
        host.collect_mail();
        (host, guest, faces)
    }

    #[test]
    fn a_miss_passes_the_turn_and_a_match_keeps_it() {
        let (store, config, fields) = channel();
        let (mut host, mut guest, faces) = dealt(&store, &config, &fields);
        let partner = |i: usize| (0..faces.len()).find(|&j| j != i && faces[j] == faces[i]).unwrap();
        let other = |i: usize| (0..faces.len()).find(|&j| faces[j] != faces[i]).unwrap();
        assert!(results(&guest.handle(&card(true, 0))).is_empty(), "not the visitor's turn");
        // The owner's first card goes to the visitor only; their own client turned it.
        assert!(results(&host.handle(&card(true, 0))).is_empty());
        assert_eq!(results(&guest.collect_mail()), vec![net::minigame::card_first(0)]);
        // A miss: both see it, the turn passes.
        let miss = other(0) as u8;
        let want = net::minigame::card_second(miss, 0, net::minigame::card_miss(0));
        assert_eq!(results(&host.handle(&card(false, miss))), vec![want.clone()]);
        assert_eq!(results(&guest.collect_mail()), vec![want]);
        assert!(results(&host.handle(&card(true, 1))).is_empty() && guest.collect_mail().is_empty(), "the owner's turn is over");
        // The visitor finds a pair and keeps the turn.
        let (a, b) = (2usize, partner(2));
        guest.handle(&card(true, a as u8));
        let want = net::minigame::card_second(b as u8, a as u8, net::minigame::card_match(1));
        assert_eq!(results(&guest.handle(&card(false, b as u8))), vec![want.clone()]);
        assert_eq!(results(&host.collect_mail()), vec![net::minigame::card_first(a as u8), want]);
        assert!(results(&guest.handle(&card(true, a as u8))).is_empty() && host.collect_mail().is_empty(), "a found card cannot be turned again");
    }

    #[test]
    fn clearing_the_board_ends_the_game_for_whoever_found_more() {
        let (store, config, fields) = channel();
        let (mut host, mut guest, faces) = dealt(&store, &config, &fields);
        // The owner finds every pair.
        let mut done = vec![false; faces.len()];
        let mut last = Vec::new();
        for i in 0..faces.len() {
            if done[i] {
                continue;
            }
            let j = (i + 1..faces.len()).find(|&j| faces[j] == faces[i]).unwrap();
            done[i] = true;
            done[j] = true;
            host.handle(&card(true, i as u8));
            last = results(&host.handle(&card(false, j as u8)));
        }
        let result = net::minigame::result_win(0, after_first_game(0));
        assert_eq!(last.last(), Some(&result), "six pairs to none");
        assert_eq!(results(&guest.collect_mail()).last(), Some(&result));
    }

    /// The owner, 2026-10-04: the record is kept per game. A won Omok game shows in the next
    /// Omok room's panel and not in a Match Cards room's.
    #[test]
    fn the_record_is_stored_and_kept_per_game() {
        let (store, config, fields) = channel();
        let (mut host, host_id, mut guest, guest_id) = playing(&store, &config, &fields);
        host.handle(&mode(net::minigame::MG_FORFEIT));
        guest.collect_mail();
        assert_eq!(store.minigame_record(guest_id, 3).unwrap().wins, 1, "the forfeit is the visitor's win");
        assert_eq!(store.minigame_record(host_id, 3).unwrap().losses, 1);
        assert_eq!(store.minigame_record(guest_id, 4).unwrap(), store::minigame::MiniGameRecord::default(), "Match Cards untouched");
        host.handle(&mode(net::trade::ROOM_LEAVE));
        guest.collect_mail();
        // The visitor opens a new Omok room: its panel shows the win.
        let open = results(&guest.handle(&create()));
        let won = after_first_game(1)[1];
        let mut w = net::PacketWriter::new();
        won.write(&mut w);
        let bytes = w.into_vec();
        assert!(open[0].windows(20).any(|x| x == bytes.as_slice()), "the Omok room opens with 1 win");
        guest.handle(&mode(net::trade::ROOM_LEAVE));
        // And a Match Cards room with a fresh record.
        let open = results(&guest.handle(&miniroom(&[0, 0, 0, 0, 4, 0, 0, 0, 1, 0, b'c', 0, 0])));
        assert!(!open[0].windows(20).any(|x| x == bytes.as_slice()), "not in Match Cards");
    }

    /// Move the room's turn clock into the past, as a real time-out would find it.
    fn age_clock(fields: &Arc<crate::fields::Fields>, by: Duration) {
        for r in fields.trades().games.iter_mut() {
            if let Some(g) = r.game.as_mut() {
                g.clock -= by;
            }
        }
    }

    /// The owner's run, 2026-10-04 05:01:48: both clients report the time-out ~20 ms apart.
    /// One stone is placed for the player who ran out, the duplicate does nothing, and the
    /// other player's next stone is accepted - before this, the second report passed the turn
    /// back and that stone was refused as out of turn.
    #[test]
    fn a_time_out_plays_a_stone_once_and_hands_the_turn_over() {
        let (store, config, fields) = channel();
        let (mut host, _, mut guest, _) = playing(&store, &config, &fields);
        host.handle(&mv(7, 7));
        guest.collect_mail();
        // The visitor's turn; their clock runs out on both screens.
        age_clock(&fields, Duration::from_secs(31));
        let first = results(&guest.handle(&mode(net::minigame::MG_TURN)));
        let second = results(&host.handle(&mode(net::minigame::MG_TURN)));
        assert_eq!(first.len(), 1, "one stone for the visitor: {first:?}");
        assert_eq!(&first[0][..4], &[0x1F, 0, 0, 0]);
        assert_eq!(*first[0].last().unwrap(), net::minigame::STONE_SECOND, "the visitor's stone, not the owner's");
        let (x, y) = (u32::from_le_bytes(first[0][4..8].try_into().unwrap()), u32::from_le_bytes(first[0][8..12].try_into().unwrap()));
        assert!(x.abs_diff(7) <= 1 && y.abs_diff(7) <= 1 && (x, y) != (7, 7), "next to the stone on the board: ({x}, {y})");
        assert_eq!(second, vec![first[0].clone()], "the owner gets the same stone, and their duplicate report adds nothing");
        guest.collect_mail();
        assert_eq!(results(&host.handle(&mv(0, 0))), vec![net::minigame::stone(0, 0, 1)], "and it is the owner's turn");
    }

    #[test]
    fn an_auto_stone_is_legal_and_wins_when_it_can() {
        let mut b = [[0u8; BOARD]; BOARD];
        // The only empty squares next to anything are a double three for stone 1.
        for (x, y) in [(5, 7), (6, 7), (7, 5), (7, 6)] {
            b[y][x] = 1;
        }
        for seed in 0..50 {
            let (x, y) = auto_stone(&b, 1, seed).unwrap();
            assert_ne!((x, y), (7, 7), "never the double three");
            assert_eq!(b[y][x], 0);
        }
        assert_eq!(auto_stone(&[[1; BOARD]; BOARD], 1, 0), None, "a full board has no square");
    }

    #[test]
    fn the_owner_can_expel_the_visitor() {
        let (store, config, fields) = channel();
        let (mut host, host_id) = join(&store, &config, &fields, "Tester2");
        let (mut guest, guest_id) = join(&store, &config, &fields, "Wisp");
        host.handle(&create());
        guest.handle(&visit(host_id));
        host.collect_mail();
        guest.collect_mail();
        let out = host.handle(&mode(net::minigame::MG_EXPEL));
        assert_eq!(results(&out), vec![net::trade::room_leave(1, net::minigame::LEAVE_EXPELLED)]);
        assert_eq!(results(&guest.collect_mail()), vec![net::trade::room_leave(1, net::minigame::LEAVE_EXPELLED)], "You have been expelled.");
        assert!(!guest.in_game_room(guest_id) && host.in_game_room(host_id));
    }


    fn board(stones: &[(usize, usize)], stone: u8) -> [[u8; BOARD]; BOARD] {
        let mut b = [[0; BOARD]; BOARD];
        for &(x, y) in stones {
            b[y][x] = stone;
        }
        b
    }

    #[test]
    fn five_in_a_row_wins_in_every_direction() {
        assert!(makes_five(&board(&[(3, 7), (4, 7), (5, 7), (6, 7), (7, 7)], 1), 5, 7, 1));
        assert!(makes_five(&board(&[(2, 2), (3, 3), (4, 4), (5, 5), (6, 6)], 1), 6, 6, 1));
        assert!(makes_five(&board(&[(6, 2), (5, 3), (4, 4), (3, 5), (2, 6)], 1), 2, 6, 1));
        assert!(!makes_five(&board(&[(3, 7), (4, 7), (5, 7), (6, 7)], 1), 6, 7, 1), "four is not five");
        assert!(makes_five(&board(&[(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)], 1), 0, 5, 1), "six counts too");
    }

    #[test]
    fn a_double_three_is_refused_and_a_single_is_not() {
        // Two open twos crossing at (7, 7): the stone there makes two open threes.
        let mut b = board(&[(5, 7), (6, 7), (7, 5), (7, 6)], 1);
        b[7][7] = 1;
        assert!(makes_double_three(&b, 7, 7, 1));
        // One open three only.
        let mut b = board(&[(5, 7), (6, 7)], 1);
        b[7][7] = 1;
        assert!(!makes_double_three(&b, 7, 7, 1));
        // A three closed at one end by the other colour is not open.
        let mut b = board(&[(5, 7), (6, 7), (7, 5), (7, 6)], 1);
        b[7][4] = 2;
        b[7][8] = 2;
        b[7][7] = 1;
        assert!(!makes_double_three(&b, 7, 7, 1), "the row is blocked on both sides of the gap that would make it open");
    }
}
