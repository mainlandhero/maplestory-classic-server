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

use super::{Reply, Session};

const BOARD: usize = net::minigame::BOARD;

/// One seat of a game room.
#[derive(Debug, Clone)]
pub(crate) struct GameSeat {
    character_id: u32,
    name: String,
    look: Vec<u8>,
    account_id: u32,
    world: u8,
}

/// A game in progress.
#[derive(Debug, Clone)]
pub(crate) struct Omok {
    /// `board[y][x]`: 0 empty, else the stone type.
    board: [[u8; BOARD]; BOARD],
    /// The stone each seat plays: the first mover's is [`net::minigame::STONE_FIRST`].
    stones: [u8; 2],
    /// The seat that moves now.
    turn: usize,
    /// Every stone in order, `(x, y, seat)` - the client keeps the same stack and pops from it
    /// on an undo (`FUN_141E9B690`).
    moves: Vec<(usize, usize, usize)>,
    tie_asked_by: Option<usize>,
    undo_asked_by: Option<usize>,
}

/// One open game room.
#[derive(Debug, Clone)]
pub(crate) struct GameRoom {
    /// The owner's character id - also what the balloon carries and a click sends back.
    id: u32,
    room_type: u32,
    title: String,
    password: Option<String>,
    /// The Omok set, item id % 100 - echoed to the room open and the balloon.
    spec: u8,
    map: crate::fields::FieldKey,
    seats: [Option<GameSeat>; 2],
    visitor_ready: bool,
    game: Option<Omok>,
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
        if room_type != net::minigame::ROOM_TYPE_OMOK {
            crate::server::log(&format!("   omok: character {} asked for a Match Cards room (type {room_type}); not hosted.", chr.id));
            return vec![Reply {
                opcode: net::message::MESSAGE,
                body: net::message::chat_line_system("Match Cards is not available yet."),
                what: "Message chat line: Match Cards rooms are not hosted".into(),
            }];
        }
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
                Some(GameSeat { character_id: chr.id, name: chr.name.clone(), look: net::opcode::avatar_look(chr), account_id, world }),
                None,
            ],
            visitor_ready: false,
            game: None,
            first_next: 0,
            leave_after: [false; 2],
        };
        let open = net::minigame::room_open(0, &room.members(), &room.title, room.spec);
        let balloon = room.balloon();
        let private = room.password.is_some();
        self.with_games(|g| {
            g.retain(|r| r.id != chr.id);
            g.push(room);
        });
        crate::server::log(&format!(
            "   omok: character {} opened Omok room {:?} (set {spec}{}). Window open, balloon up.",
            chr.id,
            title,
            if private { ", private" } else { "" }
        ));
        self.show_balloon(map, chr.id, balloon.as_ref());
        self.refresh_own_spawn();
        vec![Reply {
            opcode: net::trade::MINIROOM_RESULT,
            what: format!("MiniroomResult mode 4: Omok room {:?} opens for its owner, character {} ({} bytes)", title, chr.id, open.len()),
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
        let me = GameSeat { character_id: chr.id, name: chr.name.clone(), look: net::opcode::avatar_look(chr), account_id, world };
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
                    body: net::minigame::visitor_entered(&visitor),
                    what: format!("MiniroomResult mode 3: {} sat down in the owner's Omok room", chr.name),
                },
                Send {
                    to: chr.id,
                    body: net::minigame::room_open(1, &members, &room.title, room.spec),
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
        let mut sends = self.with_games(|g| {
            let r = g.iter_mut().find(|r| r.id == room_id).expect("seated");
            let mut sends = Vec::new();
            if r.game.is_some() {
                sends.extend(finish(r, Some(1 - seat), &format!("{} left mid-game", chr.name)));
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
        let (sends, balloon, leavers) = self.with_games(|g| {
            let r = g.iter_mut().find(|r| r.id == room_id).expect("seated");
            let before = r.game.is_some();
            let sends = step(r, seat, other, &name, &action);
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

/// Both seats of `r`, each with `body`.
fn both(r: &GameRoom, body: Vec<u8>, what: &str) -> Vec<Send> {
    r.seats.iter().flatten().map(|s| Send { to: s.character_id, body: body.clone(), what: what.to_string() }).collect()
}

fn one(r: &GameRoom, seat: usize, body: Vec<u8>, what: &str) -> Vec<Send> {
    r.occupant(seat).map(|to| Send { to, body, what: what.to_string() }).into_iter().collect()
}

/// The game ends: the result to both, and the next game's first mover is the loser.
fn finish(r: &mut GameRoom, winner: Option<usize>, why: &str) -> Vec<Send> {
    let Some(game) = r.game.take() else { return Vec::new() };
    r.visitor_ready = false;
    let first = if game.stones[0] == net::minigame::STONE_FIRST { 0 } else { 1 };
    r.first_next = match winner {
        Some(w) => 1 - w,
        None => 1 - first,
    };
    crate::server::log(&format!(
        "   omok: room {} game over after {} stone(s) - {}. {}",
        r.id,
        game.moves.len(),
        match winner {
            Some(w) => format!("seat {w} wins"),
            None => "a draw".into(),
        },
        why
    ));
    match winner {
        Some(w) => both(r, net::minigame::result_win(w as u8), &format!("MiniroomResult 0x1D: seat {w} wins ({why})")),
        None => both(r, net::minigame::result_draw(), &format!("MiniroomResult 0x1D: a draw ({why})")),
    }
}

/// One action from `seat` in room `r`.
fn step(r: &mut GameRoom, seat: usize, other: usize, name: &str, action: &net::minigame::Action) -> Vec<Send> {
    use net::minigame::Action;
    let playing = r.game.is_some();
    match *action {
        Action::OwnerOpen => {
            crate::server::log(&format!("   omok: room {} owner sent mode 0x0A (u8 1) from the room open - noted.", r.id));
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
                crate::server::log(&format!("   omok: room {} start refused - owner {}, visitor {}, ready {}.", r.id, seat == 0, r.seats[1].is_some(), r.visitor_ready));
                return Vec::new();
            }
            let first = r.first_next;
            let second = 1 - first;
            let mut stones = [0u8; 2];
            stones[first] = net::minigame::STONE_FIRST;
            stones[second] = net::minigame::STONE_SECOND;
            r.game = Some(Omok { board: [[0; BOARD]; BOARD], stones, turn: first, moves: Vec::new(), tie_asked_by: None, undo_asked_by: None });
            r.visitor_ready = false;
            crate::server::log(&format!("   omok: room {} game starts; seat {first} moves first.", r.id));
            both(r, net::minigame::start(second as u8), &format!("MiniroomResult 0x1C: the game starts, seat {first} first (the byte names seat {second}, who moves second)"))
        }
        Action::Move { x, y, .. } => {
            let Some(g) = r.game.as_mut() else { return Vec::new() };
            if g.turn != seat {
                crate::server::log(&format!("   omok: room {} seat {seat} moved out of turn; ignored.", r.id));
                return Vec::new();
            }
            let (xu, yu) = (x as usize, y as usize);
            if xu >= BOARD || yu >= BOARD || g.board[yu][xu] != 0 {
                return one(r, seat, net::minigame::bad_move(net::minigame::BAD_MOVE_OCCUPIED), &format!("MiniroomResult 0x20: ({x}, {y}) cannot take a stone"));
            }
            let stone = g.stones[seat];
            g.board[yu][xu] = stone;
            let five = makes_five(&g.board, xu, yu, stone);
            if !five && makes_double_three(&g.board, xu, yu, stone) {
                g.board[yu][xu] = 0;
                return one(r, seat, net::minigame::bad_move(net::minigame::BAD_MOVE_DOUBLE_THREE), &format!("MiniroomResult 0x20: ({x}, {y}) is a double three"));
            }
            g.moves.push((xu, yu, seat));
            g.tie_asked_by = None;
            g.undo_asked_by = None;
            let full = g.moves.len() == BOARD * BOARD;
            g.turn = other;
            let mut sends = both(r, net::minigame::stone(x, y, stone), &format!("MiniroomResult 0x1F: {name} put stone {stone} at ({x}, {y})"));
            if five {
                sends.extend(finish(r, Some(seat), &format!("{name} made five")));
            } else if full {
                sends.extend(finish(r, None, "the board is full"));
            }
            sends
        }
        Action::TimeUp => {
            let Some(g) = r.game.as_mut() else { return Vec::new() };
            if g.turn != seat {
                return Vec::new();
            }
            g.turn = other;
            both(r, net::minigame::turn(other as u8), &format!("MiniroomResult 0x1E: {name}'s clock ran out; seat {other}'s turn"))
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
                finish(r, None, "a tie was agreed")
            } else {
                one(r, other, net::minigame::tie_refused(), &format!("MiniroomResult 0x12: {name} refused the tie"))
            }
        }
        Action::Forfeit => {
            if !playing {
                return Vec::new();
            }
            finish(r, Some(other), &format!("{name} gave up"))
        }
        Action::UndoRequest => {
            let Some(g) = r.game.as_mut() else { return Vec::new() };
            if !g.moves.iter().any(|m| m.2 == seat) {
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
            // Off the top until the asker's last stone is gone: one stone if it was the last
            // move, two if the answerer has moved since.
            let mut count = 0u8;
            while let Some((x, y, s)) = g.moves.pop() {
                g.board[y][x] = 0;
                count += 1;
                if s == other {
                    break;
                }
            }
            g.turn = other;
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
        assert!(entered[0].ends_with(&[0u8; 22]), "the u16 then the 20-byte record");
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
        assert_eq!(results(&out), vec![net::minigame::stone(4, 7, 1), net::minigame::result_win(0)]);
        assert_eq!(results(&guest.collect_mail()), vec![net::minigame::stone(4, 7, 1), net::minigame::result_win(0)]);
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
            vec![net::minigame::result_win(1), net::trade::room_leave(1, net::minigame::LEAVE_ROOM_CLOSED)],
            "the visitor wins, then is told the room is closed"
        );
        assert_eq!(of(&mail, net::minigame::USER_MINIROOM_BALLOON).last().unwrap(), &net::minigame::balloon(host_id, None), "the balloon comes down");
        assert!(!guest.in_game_room(guest_id) && !host.in_game_room(host_id));
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
