//! **The ship from Ellinia Station to Orbis** - the timetable, the tickets, and each channel's
//! voyages. `session/boat.rs` puts it on the wire.
//!
//! The owner, 2026-09-26: *"Players can purchase two different types of tickets from Joel the
//! ticketing Usher: Ticket to Orbis (Basic) - 5000 mesos, Ticket to Orbis (Regular) 20000
//! mesos. The boat leaves Ellinia Station every 10 minutes ... The boat ride itself takes 5
//! minutes to complete. Players can board the boat from 5 minutes before departure time up
//! until 1 minute before departure time."* Then, for the Regular ticket: *"a private ride that
//! only lasts 1 minute. It will only consist of a single player instance, but they have free
//! access to the cabin and the boat, just to themselves with a single minute timer."* And:
//! *"Joel the ticket usher should display the two tickets available for purchase via NPC
//! dialogue and selection, not a shop window."*
//!
//! # The places, all [L] from `gm-handbook/`
//!
//! ```text
//!   10002090  Ellinia Station            Joel (322), Cherry (323)     declares a clock node
//!   10002091  Before Takeoff <To Orbis>  Purin (324)                  no clock node
//!   20000022  To Orbis                   in00/under00 -> 20000023     no clock node
//!   20000023  Cabin <To Orbis>           out00/out01  -> 20000022     no clock node
//!   20000010  Orbis Ticketing Booth                                   declares a clock node
//! ```
//!
//! The three ship fields have `returnMap` and `forcedReturn` both `10002090`
//! (`gm-handbook/returnmaps.txt`), which is where a login on one of them is sent.
//!
//! # The timetable is the wall clock, so every channel agrees about it
//!
//! A departure is every Unix second divisible by [`DEPARTURE_EVERY_S`] - `02:00`, `02:10`, ...
//! in UTC, which is the server time the station's own wall clock shows
//! (`crate::serverclock`). Nothing is scheduled: a voyage exists only once somebody boards it,
//! and it leaves when the tick notices its departure has passed.
//!
//! # A voyage is a field instance
//!
//! Every Basic passenger for one departure on one channel is in one [`Voyage`], and its id is
//! the `instance` half of `crate::fields::FieldKey` on the three ship fields - the same
//! mechanism the party quest uses, so who sees whom is decided by the key and nothing else.
//! Channels have separate `Fields`, and so separate voyages. A Regular ticket opens a voyage of
//! one, sailing at once.

/// Joel, the ticketing usher. Sells both tickets from a menu.
pub const JOEL: u32 = 322;
/// Cherry, who takes the ticket and lets the passenger on.
pub const CHERRY: u32 = 323;
/// Purin, in the waiting room: *"Anyone that wants to leave the ship and return to the
/// starting point, please come talk to me."* (`String.wz`, their own line.)
pub const PURIN: u32 = 324;

/// Ellinia Station, where Joel and Cherry stand.
pub const STATION: u32 = 10_002_090;
/// Before Takeoff <To Orbis> - the Basic passengers wait here for the departure.
pub const WAITING_ROOM: u32 = 10_002_091;
/// To Orbis - the ship's deck, where every voyage sails.
pub const DECK: u32 = 20_000_022;
/// Cabin <To Orbis> - below deck, reached by the deck's own portals.
pub const CABIN: u32 = 20_000_023;
/// Orbis Ticketing Booth - where every voyage ends.
pub const ORBIS: u32 = 20_000_010;

/// `Ticket to Orbis (Basic)`, [L] `gm-handbook/items.txt`.
pub const BASIC_TICKET: u32 = 4_031_082;
/// `Ticket to Orbis (Regular)`, [L] `gm-handbook/items.txt`.
pub const REGULAR_TICKET: u32 = 4_031_083;
/// The owner's price for the Basic ticket.
pub const BASIC_PRICE: u32 = 5_000;
/// The owner's price for the Regular ticket.
pub const REGULAR_PRICE: u32 = 20_000;

/// A ship leaves every ten minutes, on the ten.
pub const DEPARTURE_EVERY_S: i64 = 600;
/// The shared crossing takes five minutes.
pub const RIDE_S: i64 = 300;
/// Boarding opens five minutes before the departure...
pub const BOARDING_OPENS_S: i64 = 300;
/// ...and closes one minute before it. The last minute the ship is still in the station and
/// takes nobody new - the owner: *"There's intentionally a period ... that the boat technically is
/// still at station but it will not accept new passengers."*
pub const BOARDING_CLOSES_S: i64 = 60;
/// The Regular ticket's private crossing takes one minute.
pub const PRIVATE_RIDE_S: i64 = 60;

/// The conversation path Joel's opening line (a Say with Next) is parked under.
pub const JOEL_INTRO_PATH: &str = "boat.joel.intro";
/// The conversation path Joel's menu is parked under.
pub const JOEL_PATH: &str = "boat.joel";
/// The conversation path Cherry's menu is parked under.
pub const CHERRY_PATH: &str = "boat.cherry";
/// The conversation path Cherry's "do you still wish to board?" is parked under, per ticket.
pub const CHERRY_BASIC_PATH: &str = "boat.cherry.basic";
/// The Regular ticket's.
pub const CHERRY_REGULAR_PATH: &str = "boat.cherry.regular";
/// The conversation path Purin's yes/no is parked under.
pub const PURIN_PATH: &str = "boat.purin";

/// Is `path` one of this module's conversations?
pub fn is_boat_path(path: &str) -> bool {
    path.starts_with("boat.")
}

/// One of the three fields a voyage owns.
pub fn is_ship_map(map: u32) -> bool {
    matches!(map, WAITING_ROOM | DECK | CABIN)
}

/// The two tickets. The `#L` number of each line in Joel's and Cherry's menus is
/// [`Ticket::line`], so the menu and the answer cannot disagree about what "1" meant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ticket {
    Basic,
    Regular,
}

impl Ticket {
    pub const ALL: [Ticket; 2] = [Ticket::Basic, Ticket::Regular];

    pub fn item(self) -> u32 {
        match self {
            Ticket::Basic => BASIC_TICKET,
            Ticket::Regular => REGULAR_TICKET,
        }
    }

    pub fn price(self) -> u32 {
        match self {
            Ticket::Basic => BASIC_PRICE,
            Ticket::Regular => REGULAR_PRICE,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Ticket::Basic => "Ticket to Orbis (Basic)",
            Ticket::Regular => "Ticket to Orbis (Regular)",
        }
    }

    pub fn line(self) -> u32 {
        match self {
            Ticket::Basic => 0,
            Ticket::Regular => 1,
        }
    }

    pub fn from_line(line: u32) -> Option<Ticket> {
        Ticket::ALL.into_iter().find(|t| t.line() == line)
    }
}

// ---------------------------------------------------------------------------------------
// The timetable
// ---------------------------------------------------------------------------------------

/// The next departure strictly after `now`. At exactly `02:00:00` the 02:00 ship is leaving
/// and the next one is 02:10.
pub fn next_departure(now: i64) -> i64 {
    (now.div_euclid(DEPARTURE_EVERY_S) + 1) * DEPARTURE_EVERY_S
}

/// Whether Cherry is letting Basic passengers on right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Boarding {
    /// Boarding for the ship leaving at `departs`.
    Open { departs: i64 },
    /// The last minute: the `departs` ship is in and takes nobody new.
    Closing { departs: i64 },
    /// Nothing is boarding; the `departs` ship opens at `opens`.
    NotYet { departs: i64, opens: i64 },
}

/// The boarding window at `now`: open from five minutes before a departure (inclusive) until
/// one minute before it (exclusive).
pub fn boarding(now: i64) -> Boarding {
    let departs = next_departure(now);
    let left = departs - now;
    if left <= BOARDING_CLOSES_S {
        Boarding::Closing { departs }
    } else if left <= BOARDING_OPENS_S {
        Boarding::Open { departs }
    } else {
        Boarding::NotYet { departs, opens: departs - BOARDING_OPENS_S }
    }
}

/// `HH:MM`, server time (UTC) - what the station's wall clock reads.
pub fn hh_mm(unix: i64) -> String {
    let (h, m, _) = crate::serverclock::hms_of(u64::try_from(unix).unwrap_or(0));
    format!("{h:02}:{m:02}")
}

// ---------------------------------------------------------------------------------------
// The voyages
// ---------------------------------------------------------------------------------------

/// Which kind of crossing a voyage is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ride {
    /// The Basic ticket: everyone for one departure on one channel, together.
    Shared,
    /// The Regular ticket: one passenger, their own ship.
    Private,
}

/// Where a voyage is in its life.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// In the waiting room, until `departs`.
    Waiting { departs: i64 },
    /// On the ship, until `arrives`.
    Sailing { arrives: i64 },
}

/// One ship's crossing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Voyage {
    /// The field instance. Never 0, which is `FieldKey`'s shared world.
    pub id: u32,
    pub ride: Ride,
    pub phase: Phase,
    pub members: Vec<u32>,
}

impl Voyage {
    /// Seconds until whatever comes next - the departure while waiting, the arrival while
    /// sailing. Saturating at zero. **This is the number on the countdown.**
    pub fn remaining_s(&self, now: i64) -> u32 {
        let until = match self.phase {
            Phase::Waiting { departs } => departs,
            Phase::Sailing { arrives } => arrives,
        };
        u32::try_from((until - now).max(0)).unwrap_or(0)
    }
}

/// **This channel's voyages.** Owned by `crate::fields::Fields`, beside the party-quest runs,
/// so it is exactly as wide as a channel.
#[derive(Debug)]
pub struct Voyages {
    next_id: u32,
    live: Vec<Voyage>,
}

impl Default for Voyages {
    /// Ids start at 1: 0 is `FieldKey`'s shared world and must never name a voyage.
    fn default() -> Self {
        Self { next_id: 1, live: Vec::new() }
    }
}

impl Voyages {
    fn new_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1).max(1);
        id
    }

    /// **A Basic passenger boards the ship leaving at `departs`.** Everyone boarding the same
    /// departure lands in the same voyage; the first one opens it. A character is on one
    /// voyage at a time, so any other they were on is left first.
    pub fn board_shared(&mut self, character: u32, departs: i64) -> Voyage {
        self.drop_member(character);
        let waiting = Phase::Waiting { departs };
        let at = match self.live.iter().position(|v| v.ride == Ride::Shared && v.phase == waiting) {
            Some(at) => at,
            None => {
                let id = self.new_id();
                self.live.push(Voyage { id, ride: Ride::Shared, phase: waiting, members: Vec::new() });
                self.live.len() - 1
            }
        };
        self.live[at].members.push(character);
        self.live[at].clone()
    }

    /// **A Regular passenger's own ship**, sailing from `now` for [`PRIVATE_RIDE_S`].
    pub fn board_private(&mut self, character: u32, now: i64) -> Voyage {
        self.drop_member(character);
        let id = self.new_id();
        let voyage = Voyage {
            id,
            ride: Ride::Private,
            phase: Phase::Sailing { arrives: now + PRIVATE_RIDE_S },
            members: vec![character],
        };
        self.live.push(voyage.clone());
        voyage
    }

    /// The voyage `character` is on, if any.
    pub fn voyage_of(&self, character: u32) -> Option<Voyage> {
        self.live.iter().find(|v| v.members.contains(&character)).cloned()
    }

    /// **Every waiting voyage whose departure has come**, switched to sailing as it is handed
    /// over - the test-and-set, so two sessions ticking at once cannot both sail the same
    /// ship and warp its passengers twice.
    pub fn take_departures(&mut self, now: i64) -> Vec<Voyage> {
        let mut out = Vec::new();
        for v in &mut self.live {
            if let Phase::Waiting { departs } = v.phase {
                if now >= departs {
                    v.phase = Phase::Sailing { arrives: departs + RIDE_S };
                    out.push(v.clone());
                }
            }
        }
        out
    }

    /// **Every sailing voyage that has arrived**, removed as it is handed over, for the same
    /// reason as [`Voyages::take_departures`].
    pub fn take_arrivals(&mut self, now: i64) -> Vec<Voyage> {
        let arrived = |v: &Voyage| matches!(v.phase, Phase::Sailing { arrives } if now >= arrives);
        let (over, still): (Vec<_>, Vec<_>) = self.live.drain(..).partition(arrived);
        self.live = still;
        over
    }

    /// Take `character` off their voyage. A voyage with nobody left is forgotten.
    pub fn drop_member(&mut self, character: u32) -> Option<Voyage> {
        let at = self.live.iter().position(|v| v.members.contains(&character))?;
        let was = self.live[at].clone();
        self.live[at].members.retain(|&m| m != character);
        if self.live[at].members.is_empty() {
            self.live.remove(at);
        }
        Some(was)
    }
}

// ---------------------------------------------------------------------------------------
// The words. ASCII only: `PacketWriter::str` sends one byte per char.
// ---------------------------------------------------------------------------------------

// **The wording follows the v96 scripts the owner pasted, 2026-09-26** - Joel `1032007` and
// Cherry `1032008` - changed only where this server's rules differ: a ship every 10 minutes,
// not 15; boarding opens 5 minutes before, not 10; and the tickets are paid for, so Joel's
// page about the flights having become free is not here and a ticket menu is. Cherry's "the
// ride schedule is available through the guide at the ticketing booth" names a guide this
// station does not have, so the next ship's time is said instead.

/// Joel's opening line, a Say with Next. The v96 text with this server's timetable.
pub fn joel_intro() -> String {
    format!(
        "Hi there! I'm Joel, and I work in this station. Are you thinking of leaving Victoria \
         Island for other places? This station is where you'll find the ship that heads to \
         #bOrbis Station#k of Ossyria leaving #bat the top of the hour, and every {} minutes \
         afterwards#k.",
        DEPARTURE_EVERY_S / 60
    )
}

/// Joel's menu, after Next: both tickets, their prices, and what each one buys.
pub fn joel_menu() -> String {
    format!(
        "To get on board you'll need a ticket, and I sell them right here. Which one would you like?\r\n\
         #d#L{}#{} - {} mesos#l\r\n\
         #L{}#{} - {} mesos#l#k\r\n\r\n\
         With a #bBasic#k ticket you ride the next ship with the other passengers, and the flight \
         takes {} minutes. A #bRegular#k ticket is a private ship, just for you, that takes off as \
         soon as you board and arrives in {} minute.",
        Ticket::Basic.line(),
        Ticket::Basic.name(),
        thousands(BASIC_PRICE),
        Ticket::Regular.line(),
        Ticket::Regular.name(),
        thousands(REGULAR_PRICE),
        RIDE_S / 60,
        PRIVATE_RIDE_S / 60,
    )
}

/// Joel, after a sale - ending on their own v96 line about Cherry.
pub fn joel_sold(ticket: Ticket) -> String {
    format!(
        "Here is your #b{}#k. If you are thinking of going to Orbis, please go talk to #bCherry#k on the right.",
        ticket.name()
    )
}

/// Joel, when the purse is short.
pub fn joel_short(ticket: Ticket) -> String {
    format!("I'm sorry, but the #b{}#k costs #b{} mesos#k, and you don't have enough.", ticket.name(), thousands(ticket.price()))
}

/// Joel, when the Etc tab is full.
pub const JOEL_BAG_FULL: &str = "Please make some room in your #bEtc#k inventory first.";

/// Cherry's menu: which ticket. Their opening is their own `String.wz` line.
pub fn cherry_menu() -> String {
    format!(
        "If you want to get on board the ship that heads to Orbis Station, please give me the ticket.\r\n\
         #d#L{}#I have a #b{}#d.#l\r\n\
         #L{}#I have a #b{}#d.#l#k",
        Ticket::Basic.line(),
        Ticket::Basic.name(),
        Ticket::Regular.line(),
        Ticket::Regular.name(),
    )
}

/// Cherry, to somebody without the ticket they chose.
pub fn cherry_no_ticket(ticket: Ticket) -> String {
    format!("You don't have a #b{}#k. You can buy one from #bJoel#k.", ticket.name())
}

/// Cherry's yes/no before a Basic passenger boards - the v96 line.
pub const CHERRY_ASK_BASIC: &str = "This will not be a short flight, so you need to take care of some things, I suggest you do \
     that first before getting on board. Do you still wish to board the ship?";

/// Cherry's yes/no before a Regular passenger boards. Not a v96 line - v96 had no private ship.
pub const CHERRY_ASK_REGULAR: &str = "Your private ship will take off as soon as you are on board, and the flight takes only \
     #b1 minute#k. Do you wish to board the ship?";

/// Cherry, to a No - the v96 line.
pub const CHERRY_DECLINED: &str = "You must have some business to take care of here, right?";

/// Cherry, to a Basic passenger outside the boarding window - the v96 lines with this
/// server's minutes. `None` while it is open.
pub fn cherry_not_boarding(now: i64) -> Option<String> {
    match boarding(now) {
        Boarding::Open { .. } => None,
        Boarding::Closing { departs } => Some(format!(
            "This ship is getting ready for takeoff. I'm sorry, but you'll have to get on the next \
             ride. The next ship leaves at #b{}#k.",
            hh_mm(departs + DEPARTURE_EVERY_S),
        )),
        Boarding::NotYet { departs, .. } => Some(format!(
            "We will begin boarding {} minutes before the takeoff. Please be patient and wait for a \
             few minutes. Be aware that the ship will take off right on time, and we stop boarding \
             {} minute before that, so please make sure to be here on time. The next ship leaves \
             at #b{}#k.",
            BOARDING_OPENS_S / 60,
            BOARDING_CLOSES_S / 60,
            hh_mm(departs),
        )),
    }
}

/// Purin's question in the waiting room - the v96 line, then the one fact v96 did not have: the ticket is spent.
pub const PURIN_ASK: &str = "We're just about to take off. Are you sure you want to get off the ship? You may do so, but \
     then you'll have to wait until the next available flight. Do you still wish to get off board? \
     Your ticket will #rnot#k be returned.";

/// Purin, to a No - the v96 line.
pub const PURIN_STAY: &str = "You'll get to your destination in a short while. Talk to other passengers and share your \
     stories to them, and you'll be there before you know it.";

/// The notice at the end of every crossing.
pub const ARRIVED: &str = "The ship has arrived at Orbis Station.";

fn thousands(n: u32) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2025-09-10 00:00:00 UTC, a departure. `serverclock`'s own pinned instant.
    const MIDNIGHT: i64 = 1_757_462_400;

    #[test]
    fn departures_are_on_every_ten_minutes_of_the_clock() {
        assert_eq!(hh_mm(MIDNIGHT), "00:00");
        assert_eq!(next_departure(MIDNIGHT - 1), MIDNIGHT);
        assert_eq!(next_departure(MIDNIGHT), MIDNIGHT + 600, "at 00:00:00 the 00:00 ship is leaving");
        assert_eq!(next_departure(MIDNIGHT + 1), MIDNIGHT + 600);
        assert_eq!(hh_mm(next_departure(MIDNIGHT + 2 * 3600 + 5)), "02:10");
        assert_eq!(hh_mm(next_departure(MIDNIGHT + 2 * 3600 - 5)), "02:00");
    }

    /// The owner's window, at every edge: open from 5:00 before (inclusive) to 1:00 before
    /// (exclusive), and closed in the last minute though the ship is still in.
    #[test]
    fn boarding_is_open_from_five_minutes_before_until_one_minute_before() {
        let d = MIDNIGHT + 600; // 00:10
        assert_eq!(boarding(d - 301), Boarding::NotYet { departs: d, opens: d - 300 }, "00:04:59");
        assert_eq!(boarding(d - 300), Boarding::Open { departs: d }, "00:05:00 opens");
        assert_eq!(boarding(d - 61), Boarding::Open { departs: d }, "00:08:59 still open");
        assert_eq!(boarding(d - 60), Boarding::Closing { departs: d }, "00:09:00 closed");
        assert_eq!(boarding(d - 1), Boarding::Closing { departs: d }, "00:09:59 still in, still closed");
        assert_eq!(boarding(d), Boarding::NotYet { departs: d + 600, opens: d + 300 }, "00:10:00 gone");
        assert_eq!(cherry_not_boarding(d - 200), None);
        assert!(cherry_not_boarding(d - 30).unwrap().contains("00:20"), "the next one after the closing ship");
        assert!(cherry_not_boarding(d - 30).unwrap().starts_with("This ship is getting ready for takeoff"));
        let early = cherry_not_boarding(d - 400).unwrap();
        assert!(early.contains("begin boarding 5 minutes before") && early.contains("stop boarding 1 minute before"), "{early}");
        assert!(early.contains("00:10"), "{early}");
    }

    /// Every Basic passenger for one departure shares a voyage; the next departure is another.
    #[test]
    fn one_departure_is_one_voyage_and_the_next_is_another() {
        let mut v = Voyages::default();
        let d = MIDNIGHT + 600;
        let a = v.board_shared(200, d);
        let b = v.board_shared(201, d);
        assert_eq!(a.id, b.id, "one ship for one departure");
        assert_ne!(a.id, 0, "0 is the shared world");
        assert_eq!(v.voyage_of(200).unwrap().members, vec![200, 201]);
        let later = v.board_shared(202, d + 600);
        assert_ne!(later.id, a.id, "the next departure is its own ship");
        let private = v.board_private(203, d - 200);
        assert!(private.id != a.id && private.id != later.id, "and a private ship is nobody else's");
        assert_eq!(private.members, vec![203]);
        assert_eq!(private.remaining_s(d - 200), 60, "one minute");
    }

    /// Departure turns a waiting voyage into a sailing one exactly once, with five minutes on
    /// it; arrival hands it over exactly once and forgets it.
    #[test]
    fn a_voyage_departs_once_and_arrives_once() {
        let mut v = Voyages::default();
        let d = MIDNIGHT + 600;
        let ship = v.board_shared(200, d);
        assert_eq!(ship.remaining_s(d - 250), 250, "the waiting room counts to the departure");
        assert!(v.take_departures(d - 1).is_empty(), "not before");
        let gone = v.take_departures(d);
        assert_eq!(gone.len(), 1);
        assert_eq!(gone[0].phase, Phase::Sailing { arrives: d + 300 });
        assert_eq!(gone[0].remaining_s(d), 300, "five minutes on the new clock");
        assert!(v.take_departures(d + 1).is_empty(), "a second tick sails nothing");
        assert!(v.take_arrivals(d + 299).is_empty());
        let there = v.take_arrivals(d + 300);
        assert_eq!(there.len(), 1);
        assert_eq!(there[0].members, vec![200]);
        assert!(v.take_arrivals(d + 301).is_empty(), "and never twice");
        assert_eq!(v.voyage_of(200), None);
    }

    /// A ship that has sailed takes nobody; boarding the same departure after it left would be
    /// a second ship, not a stowaway on the first.
    #[test]
    fn a_sailing_ship_takes_no_new_passengers() {
        let mut v = Voyages::default();
        let d = MIDNIGHT + 600;
        let first = v.board_shared(200, d);
        let _ = v.take_departures(d);
        let late = v.board_shared(201, d);
        assert_ne!(late.id, first.id);
        assert_eq!(v.voyage_of(200).unwrap().members, vec![200]);
    }

    #[test]
    fn leaving_a_voyage_forgets_an_empty_one_and_boarding_again_moves_you() {
        let mut v = Voyages::default();
        let d = MIDNIGHT + 600;
        let a = v.board_shared(200, d);
        let _ = v.board_shared(201, d);
        assert_eq!(v.drop_member(200).map(|x| x.id), Some(a.id));
        assert_eq!(v.voyage_of(201).unwrap().members, vec![201]);
        let _ = v.board_private(201, d);
        assert!(v.take_departures(d).is_empty(), "201 left the shared ship, which was then empty and gone");
    }

    #[test]
    fn the_menus_number_their_lines_as_the_answers_read_them() {
        for t in Ticket::ALL {
            assert_eq!(Ticket::from_line(t.line()), Some(t));
            assert!(joel_menu().contains(&format!("#L{}#{}", t.line(), t.name())), "{}", joel_menu());
            assert!(cherry_menu().contains(&format!("#L{}#I have a #b{}", t.line(), t.name())), "{}", cherry_menu());
        }
        assert_eq!(Ticket::from_line(2), None);
        assert!(joel_menu().contains("5,000 mesos") && joel_menu().contains("20,000 mesos"), "{}", joel_menu());
        assert!(joel_intro().contains("every 10 minutes"), "{}", joel_intro());
        let words = [
            joel_intro(),
            joel_menu(),
            joel_sold(Ticket::Basic),
            cherry_menu(),
            cherry_not_boarding(MIDNIGHT + 100).unwrap(),
            cherry_not_boarding(MIDNIGHT + 570).unwrap(),
            CHERRY_ASK_BASIC.to_string(),
            CHERRY_ASK_REGULAR.to_string(),
            CHERRY_DECLINED.to_string(),
            PURIN_ASK.to_string(),
            PURIN_STAY.to_string(),
        ];
        for text in words {
            assert!(text.is_ascii(), "one byte per char on the wire: {text}");
        }
    }

    #[test]
    fn the_paths_are_this_modules_and_nobody_elses() {
        for p in [JOEL_INTRO_PATH, JOEL_PATH, CHERRY_PATH, CHERRY_BASIC_PATH, CHERRY_REGULAR_PATH, PURIN_PATH] {
            assert!(is_boat_path(p));
        }
        for other in [crate::taxi::MENU_PATH, crate::firsttime::ASK_PATH, crate::firsttime::NELLA_PATH] {
            assert!(!is_boat_path(other), "{other}");
        }
    }
}
