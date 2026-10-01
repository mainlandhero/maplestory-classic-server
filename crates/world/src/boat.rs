//! **The ships between Ellinia Station and Orbis**, both ways - the timetable, the tickets, and
//! each channel's voyages. `session/boat.rs` puts it on the wire.
//!
//! The owner, 2026-09-27: *"We'll need to implement the boat going from Orbis back to Ellinia as
//! well. Agatha will sell the tickets ... The boat ride will still be 10 minutes."* And, for
//! reaching the platform: *"Platform Usher on the right side of the Orbis Station map will
//! offer the players a choice to be teleported to the correct tunnel to the ship."* So the
//! route is a [`Route`], and everything below is written once for both.
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
//!   20000010  Orbis Ticketing Booth      Agatha (1000), Platform Usher (1001)   clock node
//!   20000011  Station Tunnel <To Ellinia>  west00 -> the booth, east00 -> 20000012
//!   20000012  Station<To Ellinia>        Rini (1004)          fieldType 2, shipObj, clock node
//!   20000013  Before Takeoff <To Ellinia>  Erin (1006)
//!   20000020  To Ellinia                 in00/under00 -> 20000021, shipObj (shipKind 1)
//!   20000021  Cabin <To Ellinia>         out00/out01  -> 20000020
//! ```
//!
//! The booth's `east00` has **no target** (`tm 999999999`) [L] - it is only the landing point
//! for the tunnel's `west00`. Nothing walks from the booth to the platform, which is why the
//! Platform Usher takes you. Their own line agrees: *"Orbis Station is huge. I'll take you to the
//! station platform, so talk to me."*
//!
//! The To Orbis ship fields have `returnMap` and `forcedReturn` both `10002090`; the To Ellinia
//! ones have both `20000010` (`gm-handbook/returnmaps.txt`). A login on a ship field goes to the
//! route's **departure station** instead - the owner, 2026-09-26: *"put the player back to the
//! respective station before their departure"* - which on the Orbis side is `20000012`, where
//! Rini stands, not the booth.
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
//! one: ten seconds in a waiting room of its own, then a one-minute crossing (2026-09-29).

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
/// Orbis Ticketing Booth - where the ship to Orbis lands, and where Agatha sells the tickets
/// back.
pub const ORBIS: u32 = 20_000_010;

/// Agatha, in the Orbis Ticketing Booth: *"You need to purchase a ticket to get on the ride to
/// Victoria Island."* (`String.wz`, their own line.) Sells the tickets to Ellinia.
pub const AGATHA: u32 = 1_000;
/// The Platform Usher, on the right of the booth. Takes you to the tunnel to the platform;
/// also this server's ferry to El Nath and Sleepywood (`crate::taxi`), which they still are.
pub const PLATFORM_USHER: u32 = 1_001;
/// Rini, on the platform: *"If you want to get on the ride to Victoria Island, please give me
/// the ticket for it."*
pub const RINI: u32 = 1_004;
/// Erin, in the waiting room: *"If you want to leave the ship and go back to the place of
/// takeoff, please come talk to me."*
pub const ERIN: u32 = 1_006;
/// Station Tunnel <To Ellinia>. Where the Platform Usher puts you; it walks to the platform.
pub const ORBIS_TUNNEL: u32 = 20_000_011;
/// Station<To Ellinia> - Rini's platform. `fieldType 2` with its own `shipObj`.
pub const ORBIS_STATION: u32 = 20_000_012;
/// Before Takeoff <To Ellinia>.
pub const ORBIS_WAITING_ROOM: u32 = 20_000_013;
/// To Ellinia - the deck.
pub const ELLINIA_DECK: u32 = 20_000_020;
/// Cabin <To Ellinia>.
pub const ELLINIA_CABIN: u32 = 20_000_021;

/// `Ticket to Orbis (Basic)`, [L] `gm-handbook/items.txt`.
pub const BASIC_TICKET: u32 = 4_031_082;
/// `Ticket to Orbis (Regular)`, [L] `gm-handbook/items.txt`.
pub const REGULAR_TICKET: u32 = 4_031_083;
/// `Ticket to Ellinia (Basic)`, [L] `gm-handbook/items.txt`.
pub const ELLINIA_BASIC_TICKET: u32 = 4_031_084;
/// `Ticket to Ellinia (Regular)`, [L] `gm-handbook/items.txt`.
pub const ELLINIA_REGULAR_TICKET: u32 = 4_031_085;
/// The owner's price for the Basic ticket. **The same both ways** [I]: the owner gave the To Orbis
/// prices and said of the way back only that it is the same ride.
pub const BASIC_PRICE: u32 = 5_000;
/// The owner's price for the Regular ticket, both ways.
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
/// **...after ten seconds in the waiting room**, a room of its own (the waiting room is keyed by
/// the voyage like the ship). The owner, 2026-09-29: *"The before travel should also last 10 seconds
/// before players get teleported to during the ride for 1 minute. This should happen in both
/// directions."*
pub const PRIVATE_WAIT_S: i64 = 10;

/// **The Crimson Balrog invasion.** The owner, 2026-09-26: *"There's a 50% chance that any given
/// trip will be invaded by 2 Crimson Balrog with the server spawning the two monster and the
/// accompanying background boat that Crimson Balrog arrives on. The invasion happens at 1
/// minute into the 5 minute boat ride ... it does not happen on the 1 minute private rides."*
pub const INVASION_CHANCE_PERCENT: u64 = 50;
/// One minute into the crossing.
pub const INVASION_AFTER_S: i64 = 60;
/// `700005` Crimson Balrog [L] `gm-handbook/mobnames.txt` - level 100, 741,240 HP, a boss
/// (`gm-handbook/mobtemplates.txt`). The same template summon sack `2100007` lists twice.
pub const CRIMSON_BALROG: u32 = 700_005;
/// Where the Crimson Balrogs appear: at the Balrog's ship, [`Route::enemy_ship_at`]. The owner,
/// 2026-09-26: *"The Crimson Balrog should spawn where the flying ship of the invasion is.
/// Since Crimson Balrog can fly, spawning off of a foothold is not a concern."* So no
/// foothold - in the air, 40 px either side of the ship's point so the two do not stack into
/// one sprite. The 40 is [I].
pub const INVADER_SPREAD: i16 = 40;

// ---------------------------------------------------------------------------------------
// The two routes
// ---------------------------------------------------------------------------------------

/// **One direction of the crossing.** Everything that differs between the two ships is here;
/// the timetable, the prices, the boarding window and the invasion are the same both ways.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Route {
    /// [`Voyage::route`]. 0 to Orbis, 1 to Ellinia.
    pub id: u8,
    /// Where it goes, as the ticket names it: `Ticket to <to> (Basic)`.
    pub to: &'static str,
    /// The arrival, as the notice says it.
    pub arrival_name: &'static str,
    pub seller: u32,
    pub seller_name: &'static str,
    pub seller_map: u32,
    pub boarder: u32,
    /// The departure platform, where the boarder stands and the ship docks. A `fieldType 2`
    /// map with a `shipKind 0` `shipObj`, so it takes the station animations.
    pub station: u32,
    pub steward: u32,
    pub waiting_room: u32,
    pub deck: u32,
    pub cabin: u32,
    pub arrival: u32,
    pub basic_ticket: u32,
    pub regular_ticket: u32,
    /// The deck's own `shipObj` - the Balrog's ship, `ship/ossyria/97` - which
    /// `FUN_140d6b130` draws [L].
    pub enemy_ship_at: (i16, i16),
}

/// Ellinia Station to Orbis.
pub const TO_ORBIS: Route = Route {
    id: 0,
    to: "Orbis",
    arrival_name: "Orbis Station",
    seller: JOEL,
    seller_name: "Joel",
    seller_map: STATION,
    boarder: CHERRY,
    station: STATION,
    steward: PURIN,
    waiting_room: WAITING_ROOM,
    deck: DECK,
    cabin: CABIN,
    arrival: ORBIS,
    basic_ticket: BASIC_TICKET,
    regular_ticket: REGULAR_TICKET,
    // `Map0_000.wz/020000022.img` shipObj x 485, y -221 [L].
    enemy_ship_at: (485, -221),
};

/// Orbis to Ellinia Station.
pub const TO_ELLINIA: Route = Route {
    id: 1,
    to: "Ellinia",
    arrival_name: "Ellinia Station",
    seller: AGATHA,
    seller_name: "Agatha",
    seller_map: ORBIS,
    boarder: RINI,
    station: ORBIS_STATION,
    steward: ERIN,
    waiting_room: ORBIS_WAITING_ROOM,
    deck: ELLINIA_DECK,
    cabin: ELLINIA_CABIN,
    arrival: STATION,
    basic_ticket: ELLINIA_BASIC_TICKET,
    regular_ticket: ELLINIA_REGULAR_TICKET,
    // `Map0_000.wz/020000020.img` shipObj x -590, y -221, f 1 [L] - the Balrog's ship comes
    // from the left on this side.
    enemy_ship_at: (-590, -221),
};

pub const ROUTES: [Route; 2] = [TO_ORBIS, TO_ELLINIA];

impl Route {
    /// The ticket's item id on this route.
    pub fn item(&self, t: Ticket) -> u32 {
        match t {
            Ticket::Basic => self.basic_ticket,
            Ticket::Regular => self.regular_ticket,
        }
    }

    /// `Ticket to Orbis (Basic)` - the client's own item name, rebuilt.
    pub fn ticket_name(&self, t: Ticket) -> String {
        format!("Ticket to {} ({})", self.to, match t {
            Ticket::Basic => "Basic",
            Ticket::Regular => "Regular",
        })
    }

    /// Where the two Crimson Balrogs appear on this route's deck.
    pub fn invaders_at(&self) -> [(i16, i16); 2] {
        let (x, y) = self.enemy_ship_at;
        [(x - INVADER_SPREAD, y), (x + INVADER_SPREAD, y)]
    }

    /// One of this route's three ship fields.
    pub fn owns_ship_map(&self, map: u32) -> bool {
        map == self.waiting_room || map == self.deck || map == self.cabin
    }
}

/// The route by [`Voyage::route`].
pub fn route(id: u8) -> &'static Route {
    ROUTES.iter().find(|r| r.id == id).unwrap_or(&ROUTES[0])
}

/// The route a ship field belongs to.
pub fn route_of_ship_map(map: u32) -> Option<&'static Route> {
    ROUTES.iter().find(|r| r.owns_ship_map(map))
}

/// The route whose seller, boarder or steward this template is.
pub fn route_of_npc(template: u32) -> Option<&'static Route> {
    ROUTES.iter().find(|r| [r.seller, r.boarder, r.steward].contains(&template))
}

/// A departure platform - a map that shows the station ship.
pub fn is_station(map: u32) -> bool {
    ROUTES.iter().any(|r| r.station == map)
}

/// The seller's opening line (a Say with Next) - Joel, or Agatha. Every path below is shared by
/// both routes; the conversation's NPC template says which route it is ([`route_of_npc`]).
pub const SELLER_INTRO_PATH: &str = "boat.seller.intro";
/// The seller's ticket menu.
pub const SELLER_PATH: &str = "boat.seller";
/// The boarder's menu - Cherry, or Rini.
pub const BOARDER_PATH: &str = "boat.boarder";
/// The boarder's "do you still wish to board?", per ticket.
pub const BOARDER_BASIC_PATH: &str = "boat.boarder.basic";
/// The Regular ticket's.
pub const BOARDER_REGULAR_PATH: &str = "boat.boarder.regular";
/// The steward's yes/no in the waiting room - Purin, or Erin.
pub const STEWARD_PATH: &str = "boat.steward";
/// The Platform Usher's menu.
pub const USHER_PATH: &str = "boat.usher";
/// The Platform Usher's yes/no before the tunnel.
pub const USHER_ASK_PATH: &str = "boat.usher.ask";
/// Usher menu line: the platform to Victoria Island.
pub const USHER_TO_VICTORIA: u32 = 0;

/// Is `path` one of this module's conversations?
pub fn is_boat_path(path: &str) -> bool {
    path.starts_with("boat.")
}

/// One of the six fields a voyage owns - three per route.
pub fn is_ship_map(map: u32) -> bool {
    route_of_ship_map(map).is_some()
}

/// The two tickets, on either route ([`Route::item`]). The `#L` number of each line in the
/// seller's and the boarder's menus is [`Ticket::line`], so the menu and the answer cannot
/// disagree about what "1" meant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ticket {
    Basic,
    Regular,
}

impl Ticket {
    pub const ALL: [Ticket; 2] = [Ticket::Basic, Ticket::Regular];

    pub fn price(self) -> u32 {
        match self {
            Ticket::Basic => BASIC_PRICE,
            Ticket::Regular => REGULAR_PRICE,
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
// The ship at the station - what `net::ship` animates
// ---------------------------------------------------------------------------------------

/// **Is the ship in at the station?** The owner, 2026-09-26: it *"arrives at xx:x5 to the station
/// so players can board"* and is there *"up until the boat leaves"* - so from five minutes
/// before a departure (inclusive) until the departure, the closing minute included.
pub fn ship_docked(now: i64) -> bool {
    next_departure(now) - now <= BOARDING_OPENS_S
}

/// The instant the station last changed - the ship coming in at `:x5` or leaving at `:x0` -
/// and which way. `(docked, at)`: `docked` is [`ship_docked`] at `now`, `at` when it became so.
pub fn station_changed(now: i64) -> (bool, i64) {
    let departs = next_departure(now);
    if ship_docked(now) {
        (true, departs - BOARDING_OPENS_S)
    } else {
        (false, departs - DEPARTURE_EVERY_S)
    }
}

/// How late a change may still be announced. The tick runs every 500 ms, so a live channel
/// is always inside this; a channel nobody was ticking on when the ship moved stays quiet,
/// because everyone who arrives on the station afterwards is told on entry, and a second
/// animation on top of that one would restart the slide.
pub const ANNOUNCE_WITHIN_S: i64 = 5;

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
    /// [`Route::id`].
    pub route: u8,
    pub ride: Ride,
    pub phase: Phase,
    pub members: Vec<u32>,
    /// When the Crimson Balrogs come, if they do - rolled once, at the departure, and only for
    /// a shared ship. `None` is a quiet crossing.
    pub invasion_at: Option<i64>,
    /// Whether they have come. Set once, by [`Voyages::take_invasion`].
    pub invaded: bool,
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
    /// The last station change handled ([`station_changed`]'s `at`), so each one is
    /// announced once on this channel however many sessions tick past it.
    station_handled: i64,
}

impl Default for Voyages {
    /// Ids start at 1: 0 is `FieldKey`'s shared world and must never name a voyage.
    fn default() -> Self {
        Self { next_id: 1, live: Vec::new(), station_handled: 0 }
    }
}

impl Voyages {
    /// **The ship has just come in or just left - announce it?** `Some(docked)` exactly once
    /// per change, and only within [`ANNOUNCE_WITHIN_S`] of it; `None` otherwise. The
    /// test-and-set is here, under the registry's lock, for the same reason departures are.
    pub fn take_station_change(&mut self, now: i64) -> Option<bool> {
        let (docked, at) = station_changed(now);
        if at <= self.station_handled {
            return None;
        }
        self.station_handled = at;
        (now - at <= ANNOUNCE_WITHIN_S).then_some(docked)
    }

    fn new_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1).max(1);
        id
    }

    /// **A Basic passenger boards `route`'s ship leaving at `departs`.** Everyone boarding the
    /// same departure of the same route lands in the same voyage; the first one opens it. A
    /// character is on one voyage at a time, so any other they were on is left first.
    pub fn board_shared(&mut self, character: u32, route: u8, departs: i64) -> Voyage {
        self.drop_member(character);
        let waiting = Phase::Waiting { departs };
        let at = match self.live.iter().position(|v| v.ride == Ride::Shared && v.route == route && v.phase == waiting) {
            Some(at) => at,
            None => {
                let id = self.new_id();
                self.live.push(Voyage { id, route, ride: Ride::Shared, phase: waiting, members: Vec::new(), invasion_at: None, invaded: false });
                self.live.len() - 1
            }
        };
        self.live[at].members.push(character);
        self.live[at].clone()
    }

    /// **A Regular passenger's own ship** on `route`: [`PRIVATE_WAIT_S`] in its own waiting
    /// room, then [`PRIVATE_RIDE_S`] sailing - the tick's `take_departures` moves it on, as it
    /// does a shared ship.
    pub fn board_private(&mut self, character: u32, route: u8, now: i64) -> Voyage {
        self.drop_member(character);
        let id = self.new_id();
        let voyage = Voyage {
            id,
            route,
            ride: Ride::Private,
            phase: Phase::Waiting { departs: now + PRIVATE_WAIT_S },
            members: vec![character],
            // The owner: the invasion "does not happen on the 1 minute private rides."
            invasion_at: None,
            invaded: false,
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
    /// ship and warp its passengers twice. `roll` decides each one's invasion, once:
    /// [`INVASION_CHANCE_PERCENT`] of its values mod 100 are an invasion.
    pub fn take_departures(&mut self, now: i64, mut roll: impl FnMut() -> u64) -> Vec<Voyage> {
        let mut out = Vec::new();
        for v in &mut self.live {
            if let Phase::Waiting { departs } = v.phase {
                if now >= departs {
                    let ride = if v.ride == Ride::Private { PRIVATE_RIDE_S } else { RIDE_S };
                    v.phase = Phase::Sailing { arrives: departs + ride };
                    if v.ride == Ride::Shared && roll() % 100 < INVASION_CHANCE_PERCENT {
                        v.invasion_at = Some(departs + INVASION_AFTER_S);
                    }
                    out.push(v.clone());
                }
            }
        }
        out
    }

    /// **Is it time for this voyage's Balrogs?** `true` exactly once - the first call at or
    /// after `invasion_at` while the ship is still sailing - so however many passengers tick
    /// past the minute, one of them spawns the two.
    pub fn take_invasion(&mut self, voyage: u32, now: i64) -> bool {
        let Some(v) = self.live.iter_mut().find(|v| v.id == voyage) else { return false };
        let due = matches!(v.phase, Phase::Sailing { arrives } if now < arrives) && v.invasion_at.is_some_and(|at| now >= at);
        if !due || v.invaded {
            return false;
        }
        v.invaded = true;
        true
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

// **The wording follows the v96 scripts the owner pasted** - Joel `1032007`, Cherry `1032008` and
// Purin on 2026-09-26; Agatha, Rini, Erin and Isa on 2026-09-27 - changed only where this
// server's rules differ: a ship every 10 minutes, not 15; boarding opens 5 minutes before, not
// 10; and the tickets are paid for, so Joel's page about the flights having become free is not
// here and a ticket menu is. "The ride schedule is available through the guide at the ticketing
// booth" names a guide neither station has, so the next ship's time is said instead. Rini's and
// Erin's v96 lines are word for word Cherry's and Purin's, so each is written once. Isa is not
// in this client; the Platform Usher speaks their lines.

/// The seller's opening line, a Say with Next.
pub fn seller_intro(r: &Route) -> String {
    let every = DEPARTURE_EVERY_S / 60;
    if r.id == TO_ELLINIA.id {
        // Agatha's v96 answer for "Victoria Island", without their menu of six destinations,
        // of which only one exists here.
        return format!(
            "Hello, I'm the information guide for Orbis Station. Are you trying to go to Victoria \
             Island? Oh, it's a beautiful island with an abundance of beautiful forests. The ship \
             that goes to Victoria #bleaves at the top of the hour, and every {every} minutes \
             afterwards#k."
        );
    }
    format!(
        "Hi there! I'm Joel, and I work in this station. Are you thinking of leaving Victoria \
         Island for other places? This station is where you'll find the ship that heads to \
         #bOrbis Station#k of Ossyria leaving #bat the top of the hour, and every {every} minutes \
         afterwards#k."
    )
}

/// The seller's menu, after Next: both tickets, their prices, and what each one buys.
pub fn seller_menu(r: &Route) -> String {
    format!(
        "To get on board you'll need a ticket, and I sell them right here. Which one would you like?\r\n\
         #d#L{}#{} - {} mesos#l\r\n\
         #L{}#{} - {} mesos#l#k\r\n\r\n\
         With a #bBasic#k ticket you ride the next ship with the other passengers, and the flight \
         takes {} minutes. A #bRegular#k ticket is a private ship, just for you, that takes off {} \
         seconds after you board and arrives in {} minute.",
        Ticket::Basic.line(),
        r.ticket_name(Ticket::Basic),
        thousands(BASIC_PRICE),
        Ticket::Regular.line(),
        r.ticket_name(Ticket::Regular),
        thousands(REGULAR_PRICE),
        RIDE_S / 60,
        PRIVATE_WAIT_S,
        PRIVATE_RIDE_S / 60,
    )
}

/// The seller, after a sale - Joel's own v96 line about Cherry, or Isa's about themself spoken
/// of the Platform Usher.
pub fn seller_sold(r: &Route, ticket: Ticket) -> String {
    let next = if r.id == TO_ELLINIA.id {
        "Talk to the #bPlatform Usher#k on the right if you would like to take the airship to \
         Victoria. The Platform Usher will guide you to the Station to Victoria."
    } else {
        "If you are thinking of going to Orbis, please go talk to #bCherry#k on the right."
    };
    format!("Here is your #b{}#k. {next}", r.ticket_name(ticket))
}

/// The seller, when the purse is short.
pub fn seller_short(r: &Route, ticket: Ticket) -> String {
    format!(
        "I'm sorry, but the #b{}#k costs #b{} mesos#k, and you don't have enough.",
        r.ticket_name(ticket),
        thousands(ticket.price())
    )
}

/// The seller, when the Etc tab is full.
pub const SELLER_BAG_FULL: &str = "Please make some room in your #bEtc#k inventory first.";

/// The boarder's menu: which ticket. Their opening is their own `String.wz` line.
pub fn boarder_menu(r: &Route) -> String {
    let opening = if r.id == TO_ELLINIA.id {
        "If you want to get on the ride to Victoria Island, please give me the ticket for it."
    } else {
        "If you want to get on board the ship that heads to Orbis Station, please give me the ticket."
    };
    format!(
        "{opening}\r\n\
         #d#L{}#I have a #b{}#d.#l\r\n\
         #L{}#I have a #b{}#d.#l#k",
        Ticket::Basic.line(),
        r.ticket_name(Ticket::Basic),
        Ticket::Regular.line(),
        r.ticket_name(Ticket::Regular),
    )
}

/// The boarder, to somebody without the ticket they chose.
pub fn boarder_no_ticket(r: &Route, ticket: Ticket) -> String {
    format!("You don't have a #b{}#k. You can buy one from #b{}#k.", r.ticket_name(ticket), r.seller_name)
}

/// The boarder's yes/no before a Basic passenger boards - the v96 line.
pub const BOARD_ASK_BASIC: &str = "This will not be a short flight, so you need to take care of some things, I suggest you do \
     that first before getting on board. Do you still wish to board the ship?";

/// The boarder's yes/no before a Regular passenger boards. Not a v96 line - v96 had no
/// private ship.
pub const BOARD_ASK_REGULAR: &str = "Your private ship will take off #b10 seconds#k after you are on board, and the flight \
     takes only #b1 minute#k. Do you wish to board the ship?";

/// The boarder, to a No - the v96 line.
pub const BOARD_DECLINED: &str = "You must have some business to take care of here, right?";

/// The boarder, to a Basic passenger outside the boarding window - the v96 lines with this
/// server's minutes. `None` while it is open.
pub fn not_boarding(now: i64) -> Option<String> {
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

/// The steward's question in the waiting room - the v96 line, then the one fact v96 did not
/// have: the ticket is spent.
pub const STEWARD_ASK: &str = "We're just about to take off. Are you sure you want to get off the ship? You may do so, but \
     then you'll have to wait until the next available flight. Do you still wish to get off board? \
     Your ticket will #rnot#k be returned.";

/// The steward, to a No - the v96 line.
pub const STEWARD_STAY: &str = "You'll get to your destination in a short while. Talk to other passengers and share your \
     stories to them, and you'll be there before you know it.";

/// The notice at the end of every crossing.
pub fn arrived(r: &Route) -> String {
    format!("The ship has arrived at {}.", r.arrival_name)
}

/// The Platform Usher's menu - Isa's v96 opening, with the one platform this client has, and
/// their ferry as the other line.
pub fn usher_menu() -> String {
    format!(
        "There are many Platforms at the Orbis Station. You must find the correct Platform for your \
         destination. Which Platform would you like to go to?\r\n\
         #d#L{USHER_TO_VICTORIA}##bPlatform to Board a Ship to Victoria Island#d#l#k"
    )
}

/// The Platform Usher's yes/no - Isa's v96 line. The tunnel's `west00` does lead back to the
/// booth [L], so "you can always get back" is true here.
pub const USHER_ASK: &str = "Even if you've entered a wrong Tunnel, you can always get back to where I am, via the Portal, \
     so don't worry. Would you like to go to the #bPlatform to the Ship that heads to Victoria Island#k?";

/// The Platform Usher, to a No - Isa's v96 line.
pub const USHER_DECLINED: &str = "Please make sure you know where you are going and then go to the platform through me. The \
     ride is on schedule so you better not miss it!";

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
        assert_eq!(not_boarding(d - 200), None);
        assert!(not_boarding(d - 30).unwrap().contains("00:20"), "the next one after the closing ship");
        assert!(not_boarding(d - 30).unwrap().starts_with("This ship is getting ready for takeoff"));
        let early = not_boarding(d - 400).unwrap();
        assert!(early.contains("begin boarding 5 minutes before") && early.contains("stop boarding 1 minute before"), "{early}");
        assert!(early.contains("00:10"), "{early}");
    }

    /// Every Basic passenger for one departure of one route shares a voyage; the next
    /// departure is another, and so is the same departure the other way.
    #[test]
    fn one_departure_is_one_voyage_per_route() {
        let mut v = Voyages::default();
        let d = MIDNIGHT + 600;
        let a = v.board_shared(200, TO_ORBIS.id, d);
        let b = v.board_shared(201, TO_ORBIS.id, d);
        assert_eq!(a.id, b.id, "one ship for one departure");
        assert_ne!(a.id, 0, "0 is the shared world");
        assert_eq!(v.voyage_of(200).unwrap().members, vec![200, 201]);
        let later = v.board_shared(202, TO_ORBIS.id, d + 600);
        assert_ne!(later.id, a.id, "the next departure is its own ship");
        let back = v.board_shared(204, TO_ELLINIA.id, d);
        assert_ne!(back.id, a.id, "the same minute the other way is the other ship");
        assert_eq!(back.route, TO_ELLINIA.id);
        let private = v.board_private(203, TO_ORBIS.id, d - 200);
        assert!(private.id != a.id && private.id != later.id && private.id != back.id, "and a private ship is nobody else's");
        assert_eq!(private.members, vec![203]);
        assert_eq!(private.remaining_s(d - 200), 10, "ten seconds in the waiting room first");
    }

    /// The two routes, read against the map data they were typed from: every map a route
    /// names is its own and no other route's, and the NPCs find their route.
    #[test]
    fn the_two_routes_are_mirror_images_and_share_nothing() {
        assert_eq!((TO_ORBIS.arrival, TO_ELLINIA.arrival), (ORBIS, STATION), "each lands where the other's seller is, or near it");
        assert_eq!(TO_ELLINIA.seller_map, TO_ORBIS.arrival, "Agatha sells in the booth the ship to Orbis lands in");
        for r in ROUTES {
            for map in [r.waiting_room, r.deck, r.cabin] {
                assert_eq!(route_of_ship_map(map).map(|x| x.id), Some(r.id), "{map}");
            }
            assert!(!is_ship_map(r.station) && is_station(r.station), "the platform is not a ship field");
            assert!(!is_ship_map(r.arrival));
            for npc in [r.seller, r.boarder, r.steward] {
                assert_eq!(route_of_npc(npc).map(|x| x.id), Some(r.id), "{npc}");
            }
            assert_eq!(route(r.id).id, r.id);
        }
        assert_eq!(TO_ORBIS.ticket_name(Ticket::Basic), "Ticket to Orbis (Basic)");
        assert_eq!(TO_ELLINIA.ticket_name(Ticket::Regular), "Ticket to Ellinia (Regular)");
        assert_eq!((TO_ELLINIA.item(Ticket::Basic), TO_ELLINIA.item(Ticket::Regular)), (4_031_084, 4_031_085));
        assert_eq!(TO_ELLINIA.invaders_at(), [(-630, -221), (-550, -221)], "at the To Ellinia deck's own shipObj");
        assert_eq!(TO_ORBIS.invaders_at(), [(445, -221), (525, -221)]);
        assert!(!is_station(ORBIS) && !is_station(ORBIS_TUNNEL), "the booth and the tunnel are not platforms");
    }

    /// The route names in the ticket text are the client's own item names - checked against
    /// the generated table when it is on disk.
    #[test]
    fn the_ticket_names_are_the_clients() {
        let Ok(items) = std::fs::read_to_string("../../gm-handbook/items.txt") else { return };
        for r in ROUTES {
            for t in Ticket::ALL {
                let row = format!("{}, {}", r.item(t), r.ticket_name(t));
                assert!(items.lines().any(|l| l == row), "{row} is not in items.txt");
            }
        }
    }

    /// Departure turns a waiting voyage into a sailing one exactly once, with five minutes on
    /// it; arrival hands it over exactly once and forgets it.
    #[test]
    fn a_voyage_departs_once_and_arrives_once() {
        let mut v = Voyages::default();
        let d = MIDNIGHT + 600;
        let ship = v.board_shared(200, TO_ORBIS.id, d);
        assert_eq!(ship.remaining_s(d - 250), 250, "the waiting room counts to the departure");
        assert!(v.take_departures(d - 1, || 0).is_empty(), "not before");
        let gone = v.take_departures(d, || 99);
        assert_eq!(gone.len(), 1);
        assert_eq!(gone[0].phase, Phase::Sailing { arrives: d + 300 });
        assert_eq!(gone[0].remaining_s(d), 300, "five minutes on the new clock");
        assert!(v.take_departures(d + 1, || 0).is_empty(), "a second tick sails nothing");
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
        let first = v.board_shared(200, TO_ORBIS.id, d);
        let _ = v.take_departures(d, || 0);
        let late = v.board_shared(201, TO_ORBIS.id, d);
        assert_ne!(late.id, first.id);
        assert_eq!(v.voyage_of(200).unwrap().members, vec![200]);
    }

    #[test]
    fn leaving_a_voyage_forgets_an_empty_one_and_boarding_again_moves_you() {
        let mut v = Voyages::default();
        let d = MIDNIGHT + 600;
        let a = v.board_shared(200, TO_ORBIS.id, d);
        let _ = v.board_shared(201, TO_ORBIS.id, d);
        assert_eq!(v.drop_member(200).map(|x| x.id), Some(a.id));
        assert_eq!(v.voyage_of(201).unwrap().members, vec![201]);
        let _ = v.board_private(201, TO_ORBIS.id, d);
        assert!(v.take_departures(d, || 0).is_empty(), "201 left the shared ship, which was then empty and gone");
    }

    #[test]
    fn the_menus_number_their_lines_as_the_answers_read_them() {
        for r in ROUTES {
            for t in Ticket::ALL {
                assert_eq!(Ticket::from_line(t.line()), Some(t));
                assert!(seller_menu(&r).contains(&format!("#L{}#{}", t.line(), r.ticket_name(t))), "{}", seller_menu(&r));
                assert!(boarder_menu(&r).contains(&format!("#L{}#I have a #b{}", t.line(), r.ticket_name(t))), "{}", boarder_menu(&r));
            }
            assert!(seller_menu(&r).contains("5,000 mesos") && seller_menu(&r).contains("20,000 mesos"), "{}", seller_menu(&r));
            assert!(seller_intro(&r).contains("every 10 minutes"), "{}", seller_intro(&r));
            assert!(boarder_no_ticket(&r, Ticket::Basic).contains(r.seller_name));
        }
        assert!(seller_intro(&TO_ELLINIA).starts_with("Hello, I'm the information guide for Orbis Station"));
        assert!(seller_sold(&TO_ELLINIA, Ticket::Basic).contains("Platform Usher"));
        assert!(seller_sold(&TO_ORBIS, Ticket::Basic).contains("Cherry"));
        assert!(boarder_menu(&TO_ELLINIA).starts_with("If you want to get on the ride to Victoria Island"));
        assert!(usher_menu().contains(&format!("#L{USHER_TO_VICTORIA}#")) && !usher_menu().contains("ferry"), "the platform, and no ferry (2026-09-29)");
        assert_eq!(Ticket::from_line(2), None);
        let mut words = vec![
            not_boarding(MIDNIGHT + 100).unwrap(),
            not_boarding(MIDNIGHT + 570).unwrap(),
            BOARD_ASK_BASIC.to_string(),
            BOARD_ASK_REGULAR.to_string(),
            BOARD_DECLINED.to_string(),
            STEWARD_ASK.to_string(),
            STEWARD_STAY.to_string(),
            usher_menu(),
            USHER_ASK.to_string(),
            USHER_DECLINED.to_string(),
        ];
        for r in ROUTES {
            words.extend([seller_intro(&r), seller_menu(&r), seller_sold(&r, Ticket::Basic), boarder_menu(&r), arrived(&r)]);
        }
        for text in words {
            assert!(text.is_ascii(), "one byte per char on the wire: {text}");
        }
    }

    /// The ship is in from `:x5:00` until the departure, closing minute included, and out
    /// from the departure until `:x5:00`. Each change is announced once, and only while fresh.
    #[test]
    fn the_ship_is_in_from_x5_until_it_leaves_and_each_change_is_announced_once() {
        let d = MIDNIGHT + 600; // 00:10
        assert!(!ship_docked(d - 301), "00:04:59 - out");
        assert!(ship_docked(d - 300), "00:05:00 - in");
        assert!(ship_docked(d - 30), "00:09:30 - in, though boarding has closed");
        assert!(!ship_docked(d), "00:10:00 - gone");
        assert_eq!(station_changed(d - 200), (true, d - 300));
        assert_eq!(station_changed(d + 10), (false, d));

        let mut v = Voyages::default();
        assert_eq!(v.take_station_change(d - 298), Some(true), "two seconds after it came in");
        assert_eq!(v.take_station_change(d - 297), None, "once");
        assert_eq!(v.take_station_change(d + 1), Some(false), "and it leaves");
        assert_eq!(v.take_station_change(d + 2), None);
        // A channel nobody ticked on: the next change it sees is stale, and it stays quiet
        // rather than restarting the slide for people the entry packet already told.
        assert_eq!(v.take_station_change(d + 300 + 60), None, "a minute late - too late");
        assert_eq!(v.take_station_change(d + 600 + 1), Some(false), "the next one is fresh again");
    }

    /// Half the shared crossings are invaded, a minute in, once; a private ship never is.
    #[test]
    fn half_the_shared_crossings_are_invaded_a_minute_in_and_never_a_private_one() {
        let d = MIDNIGHT + 600;
        let mut v = Voyages::default();
        let quiet = v.board_shared(200, TO_ORBIS.id, d);
        let loud = v.board_shared(201, TO_ELLINIA.id, d + 600);
        let solo = v.board_private(202, TO_ORBIS.id, d);
        let _ = v.take_departures(d, || 50); // 50 of 100: not under 50, quiet
        let _ = v.take_departures(d + 600, || 49); // under 50: invaded
        assert_eq!(v.voyage_of(200).unwrap().invasion_at, None);
        assert_eq!(v.voyage_of(201).unwrap().invasion_at, Some(d + 660), "one minute into the crossing, either way");
        assert_eq!(v.voyage_of(202).unwrap().invasion_at, None, "the private ride never rolls");

        assert!(!v.take_invasion(loud.id, d + 659), "not before the minute");
        assert!(v.take_invasion(loud.id, d + 660));
        assert!(!v.take_invasion(loud.id, d + 661), "once");
        assert!(v.voyage_of(201).unwrap().invaded);
        assert!(!v.take_invasion(quiet.id, d + 60), "a quiet crossing stays quiet");
        assert!(!v.take_invasion(solo.id, d + 60));

        // Over many rolls, exactly the values under 50 invade: the chance is the constant.
        let mut n = 0u64;
        let invaded = (0..1000)
            .filter(|_| {
                let mut w = Voyages::default();
                let _ = w.board_shared(1, TO_ORBIS.id, d);
                n += 1;
                let _ = w.take_departures(d, || n.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 7);
                w.voyage_of(1).unwrap().invasion_at.is_some()
            })
            .count();
        assert!((400..600).contains(&invaded), "{invaded} of 1000");
    }

    #[test]
    fn the_paths_are_this_modules_and_nobody_elses() {
        for p in [SELLER_INTRO_PATH, SELLER_PATH, BOARDER_PATH, BOARDER_BASIC_PATH, BOARDER_REGULAR_PATH, STEWARD_PATH, USHER_PATH, USHER_ASK_PATH] {
            assert!(is_boat_path(p));
        }
        for other in [crate::taxi::MENU_PATH, crate::firsttime::ASK_PATH, crate::firsttime::NELLA_PATH] {
            assert!(!is_boat_path(other), "{other}");
        }
    }
}
