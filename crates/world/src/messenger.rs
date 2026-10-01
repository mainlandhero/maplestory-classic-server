//! Maple Chat rooms, as a **registry the world hub serialises** - the same shape as
//! [`crate::party`].
//!
//! The owner, 2026-09-24, from the deployed server: *"the Maple Chat invitation did not work, an
//! invite was attempted, but ... the client was not able to accept even when they tried to
//! accept it, the server replied back you are too busy"* - and then *"Maple Chat should work
//! cross channel, please use the hub code."*
//!
//! `Server Investigation/world-ch1.log` 04:32:58 and `world-ch0.log` 04:33:04 are the whole
//! story: Cobalt opened room `0x20001` on **channel 1**, the invite crossed to Moth on
//! **channel 0** through the hub, and Moth's Accept arrived at channel 0 - whose per-process
//! registry did not hold the room, so it answered mode 0 result 1 and the window stayed shut.
//! The invite used the hub; the room did not.
//!
//! # Shape
//!
//! Every channel holds a replica of [`Rooms`]. A change - open, enter, leave - is a
//! [`Request`] the hub applies to its own copy and echoes to **every** channel in one order
//! (`crate::link::Frame::MessengerRequest`), so every replica applies the same sequence and
//! agrees, room ids included. A channel that connects late gets
//! `crate::link::Frame::MessengerSnapshot` first. With no hub (the test suite, one channel
//! started by hand) the session applies the request to its own replica synchronously - the
//! same `apply`, the same effects.
//!
//! A chat line and an invite change nothing, so they are not requests: any channel reads its
//! replica for the room and its members and delivers through `Session::deliver_anywhere`.

use net::messenger::{Seat, SEATS};

/// The first room id. Non-zero, and never a small number - the messenger id is a `u32` the
/// client keeps and echoes, and `0` is what an unset id looks like on the wire.
pub const FIRST_ROOM_ID: u32 = 0x0001_0001;

/// One room: six seats.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Room {
    pub id: u32,
    pub seats: [Option<Seat>; SEATS],
}

impl Room {
    pub fn position_of(&self, character: u32) -> Option<usize> {
        self.seats.iter().position(|s| s.as_ref().is_some_and(|s| s.character_id == character))
    }

    /// Everyone seated.
    pub fn members(&self) -> Vec<u32> {
        self.seats.iter().flatten().map(|s| s.character_id).collect()
    }
}

/// A change to the registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    /// Open a room with the actor in seat 0. The id is the registry's next one - the same on
    /// every replica, because every replica applies the same sequence.
    Open { seat: Seat },
    /// The actor takes a free seat in `room` (the invite dialog's Accept).
    Enter { room: u32, seat: Seat },
    /// The actor leaves `room` (the window was closed).
    Leave { room: u32 },
    /// The actor went offline everywhere: leave whatever room they are in.
    Disconnect,
}

/// What an applied request did, for the actor's channel to turn into packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    Opened { room: u32, seats: [Option<Seat>; SEATS] },
    /// `others` are the members who were already seated and must be shown the new table.
    Joined { room: u32, position: usize, seats: [Option<Seat>; SEATS], others: Vec<u32> },
    /// `remaining` are told with the leaver's seat zeroed; empty means the room is gone.
    Left { room: u32, position: usize, seats: [Option<Seat>; SEATS], remaining: Vec<u32> },
    /// Nothing to do (a Disconnect for somebody in no room).
    Nothing,
}

/// Why a request changed nothing. Each carries the room it named, because the refusal the
/// client is sent (mode 0, result 1) echoes the id it asked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    NoSuchRoom(u32),
    Full(u32),
    NotInRoom(u32),
}

impl Refusal {
    pub fn room(self) -> u32 {
        match self {
            Refusal::NoSuchRoom(r) | Refusal::Full(r) | Refusal::NotInRoom(r) => r,
        }
    }
}

/// Every room the world has.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rooms {
    rooms: Vec<Room>,
    next_id: u32,
}

impl Default for Rooms {
    fn default() -> Self {
        Self::new()
    }
}

impl Rooms {
    pub fn new() -> Self {
        Rooms { rooms: Vec::new(), next_id: FIRST_ROOM_ID }
    }

    pub fn get(&self, id: u32) -> Option<&Room> {
        self.rooms.iter().find(|r| r.id == id)
    }

    /// The room `character` is seated in, if any.
    pub fn room_of(&self, character: u32) -> Option<&Room> {
        self.rooms.iter().find(|r| r.position_of(character).is_some())
    }

    pub fn len(&self) -> usize {
        self.rooms.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rooms.is_empty()
    }

    pub fn snapshot(&self) -> Vec<Room> {
        self.rooms.clone()
    }

    pub fn next_id(&self) -> u32 {
        self.next_id
    }

    /// Replace the whole registry with the hub's copy (a late-connecting channel).
    pub fn restore(&mut self, rooms: Vec<Room>, next_id: u32) {
        self.rooms = rooms;
        self.next_id = next_id.max(FIRST_ROOM_ID);
    }

    /// Apply one request for `actor`. Deterministic: the same sequence gives the same state
    /// and the same ids on every replica.
    pub fn apply(&mut self, actor: u32, request: Request) -> Result<Effect, Refusal> {
        match request {
            Request::Open { seat } => {
                // A player is in one room at a time: opening a new one leaves the old one
                // first, silently, rather than leaving a ghost seat behind.
                self.remove_everywhere(actor);
                let id = self.next_id;
                self.next_id = self.next_id.wrapping_add(1).max(FIRST_ROOM_ID);
                let mut room = Room { id, ..Room::default() };
                room.seats[0] = Some(seat);
                let seats = room.seats.clone();
                self.rooms.push(room);
                Ok(Effect::Opened { room: id, seats })
            }
            Request::Enter { room, seat } => {
                let r = self.rooms.iter_mut().find(|r| r.id == room).ok_or(Refusal::NoSuchRoom(room))?;
                if let Some(position) = r.position_of(actor) {
                    return Ok(Effect::Joined { room, position, seats: r.seats.clone(), others: Vec::new() });
                }
                let position = r.seats.iter().position(Option::is_none).ok_or(Refusal::Full(room))?;
                let others = r.members();
                r.seats[position] = Some(seat);
                let seats = r.seats.clone();
                Ok(Effect::Joined { room, position, seats, others })
            }
            Request::Leave { room } => self.leave(actor, room),
            Request::Disconnect => match self.room_of(actor).map(|r| r.id) {
                Some(room) => self.leave(actor, room),
                None => Ok(Effect::Nothing),
            },
        }
    }

    fn leave(&mut self, actor: u32, room: u32) -> Result<Effect, Refusal> {
        // A leave that finds nothing is `NotInRoom` whether the room is gone or the actor was
        // never in it: either way the leaver's window is already shut and nothing is sent.
        let r = self.rooms.iter_mut().find(|r| r.id == room).ok_or(Refusal::NotInRoom(room))?;
        let position = r.position_of(actor).ok_or(Refusal::NotInRoom(room))?;
        r.seats[position] = None;
        let seats = r.seats.clone();
        let remaining = r.members();
        if remaining.is_empty() {
            self.rooms.retain(|r| r.id != room);
        }
        Ok(Effect::Left { room, position, seats, remaining })
    }

    fn remove_everywhere(&mut self, actor: u32) {
        for r in &mut self.rooms {
            if let Some(p) = r.position_of(actor) {
                r.seats[p] = None;
            }
        }
        self.rooms.retain(|r| !r.members().is_empty());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seat(id: u32, name: &str) -> Seat {
        Seat { character_id: id, name: name.to_string(), look: vec![id as u8] }
    }

    /// **The deployed server's failure, as the registry now sees it.** Two replicas - channel
    /// 0's and channel 1's - applying the same hub-ordered sequence agree on the room, its id
    /// and its seats, so an Accept on either channel finds it.
    #[test]
    fn two_replicas_fed_the_same_sequence_agree_on_the_room() {
        let (mut ch0, mut ch1) = (Rooms::new(), Rooms::new());
        let seq = vec![
            (213, Request::Open { seat: seat(213, "Cobalt") }),
            (219, Request::Enter { room: FIRST_ROOM_ID, seat: seat(219, "Moth") }),
        ];
        let mut effects = Vec::new();
        for (actor, r) in seq {
            effects.push(ch0.apply(actor, r.clone()));
            ch1.apply(actor, r).unwrap();
        }
        assert_eq!(ch0, ch1, "every replica is the same registry");
        match &effects[1] {
            Ok(Effect::Joined { room, position, others, .. }) => {
                assert_eq!((*room, *position, others.as_slice()), (FIRST_ROOM_ID, 1, &[213][..]));
            }
            e => panic!("{e:?}"),
        }
        assert_eq!(ch1.room_of(219).unwrap().id, FIRST_ROOM_ID, "channel 1 knows Moth's room");
    }

    #[test]
    fn a_full_room_a_missing_room_and_the_last_to_leave() {
        let mut r = Rooms::new();
        r.apply(1, Request::Open { seat: seat(1, "a") }).unwrap();
        for id in 2..=6 {
            r.apply(id, Request::Enter { room: FIRST_ROOM_ID, seat: seat(id, "x") }).unwrap();
        }
        assert_eq!(r.apply(7, Request::Enter { room: FIRST_ROOM_ID, seat: seat(7, "x") }), Err(Refusal::Full(FIRST_ROOM_ID)));
        assert_eq!(r.apply(7, Request::Enter { room: 999, seat: seat(7, "x") }), Err(Refusal::NoSuchRoom(999)));
        // Re-entering is idempotent: the same seat, nobody re-told.
        assert!(matches!(
            r.apply(3, Request::Enter { room: FIRST_ROOM_ID, seat: seat(3, "x") }),
            Ok(Effect::Joined { position: 2, ref others, .. }) if others.is_empty()
        ));
        for id in 1..=5 {
            r.apply(id, Request::Leave { room: FIRST_ROOM_ID }).unwrap();
        }
        assert!(matches!(r.apply(6, Request::Disconnect), Ok(Effect::Left { ref remaining, .. }) if remaining.is_empty()));
        assert!(r.is_empty(), "the last one out forgets the room");
        assert_eq!(r.apply(6, Request::Disconnect), Ok(Effect::Nothing));
    }
}
