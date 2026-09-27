//! The ship from Ellinia Station to Orbis, on the wire. The timetable, the tickets and each
//! channel's voyages are `crate::boat`.
//!
//! * **Joel** sells both tickets from a menu (`0x055B` type 6), one line per ticket.
//! * **Cherry** takes a ticket. A Basic one inside the boarding window puts the passenger in
//!   the waiting room with everyone else on that departure; a Regular one puts them straight
//!   on their own ship.
//! * **The tick** sails every waiting voyage whose departure has come and lands every sailing
//!   voyage whose crossing is over - this session's own passenger directly, the rest through
//!   `Event::BoatWarp`, because only a passenger's own session can build their `SetField`.
//! * **Every field entry** on a ship field carries the countdown, type 2 as the party quest's
//!   is, so walking between the deck and the cabin re-sends the same time rather than
//!   starting a new one.
//!
//! Nothing here authenticates: a ride is sold to whoever holds the socket.

use super::{Conversation, Reply, Session};
use crate::boat::{self, Ticket};

impl Session {
    /// A click on Joel or Cherry at the station, or on Purin in the waiting room. `None` for
    /// any other NPC or map, so the ordinary click chain carries on.
    pub(super) fn open_boat_npc(&mut self, template: u32) -> Option<Vec<Reply>> {
        let chr = self.claimed_character()?;
        match (template, chr.map_id) {
            // Joel opens with their v96 introduction, and Next brings up the tickets.
            (boat::JOEL, boat::STATION) => {
                self.park_boat(template, boat::JOEL_INTRO_PATH, false, true);
                Some(vec![Reply {
                    opcode: net::script::SCRIPT_MESSAGE,
                    body: net::script::npc_say(template, &boat::joel_intro(), false, true),
                    what: "ScriptMessage Say from Joel: the station and its timetable; Next opens the tickets".to_string(),
                }])
            }
            (boat::CHERRY, boat::STATION) => Some(self.boat_menu(boat::CHERRY_PATH, template, &boat::cherry_menu(), "which ticket")),
            (boat::PURIN, boat::WAITING_ROOM) => Some(self.boat_ask(boat::PURIN_PATH, template, boat::PURIN_ASK, "back to Ellinia Station?")),
            _ => None,
        }
    }

    fn park_boat(&mut self, template: u32, path: &str, yes_no: bool, next: bool) {
        self.conversation = Some(Conversation {
            npc_template: template,
            quest_id: None,
            path: path.to_string(),
            sent: 0,
            awaiting_yes_no: yes_no,
            sent_with_next: next,
        });
    }

    fn boat_menu(&mut self, path: &str, template: u32, text: &str, what: &str) -> Vec<Reply> {
        self.park_boat(template, path, false, false);
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_menu(template, text),
            what: format!("ScriptMessage MENU from NPC {template}: {what}"),
        }]
    }

    fn boat_ask(&mut self, path: &str, template: u32, text: &str, what: &str) -> Vec<Reply> {
        self.park_boat(template, path, true, false);
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_ask(template, text, false),
            what: format!("ScriptMessage YES/NO from NPC {template}: {what}"),
        }]
    }

    /// Joel's Next: the tickets.
    pub(super) fn joel_intro_answer(&mut self, action: i8) -> Vec<Reply> {
        self.conversation = None;
        if action != net::script::SCRIPT_ACTION_YES {
            return Vec::new();
        }
        self.boat_menu(boat::JOEL_PATH, boat::JOEL, &boat::joel_menu(), "the two tickets to Orbis")
    }

    /// Joel's or Cherry's menu came back. `None` when neither is parked, so the other menu
    /// answers get their turn.
    pub(super) fn boat_menu_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        if convo.path != boat::JOEL_PATH && convo.path != boat::CHERRY_PATH {
            return None;
        }
        let reply = net::script::parse_menu_reply(body)?;
        self.conversation = None;
        let Some(ticket) = reply.selection.and_then(Ticket::from_line) else { return Some(Vec::new()) };
        if convo.path == boat::JOEL_PATH {
            return Some(self.buy_ticket(ticket));
        }
        Some(self.cherry_choice(ticket, store::Store::unix_now()))
    }

    /// **Cherry, once the ticket is named.** Every refusal is said here, before they ask -
    /// no ticket, or (Basic) the ship is not boarding - so a Yes is never followed by a No.
    /// The Yes re-checks both anyway ([`Session::board_ship`]): the window can close while the
    /// question is on screen.
    pub(super) fn cherry_choice(&mut self, ticket: Ticket, now: i64) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        if self.held_count(chr.id, ticket.item()) == 0 {
            return vec![self.boat_say(boat::CHERRY, &boat::cherry_no_ticket(ticket), format!("no {}", ticket.name()))];
        }
        match ticket {
            Ticket::Basic => match boat::cherry_not_boarding(now) {
                Some(line) => vec![self.boat_say(boat::CHERRY, &line, format!("not boarding: {:?}", boat::boarding(now)))],
                None => self.boat_ask(boat::CHERRY_BASIC_PATH, boat::CHERRY, boat::CHERRY_ASK_BASIC, "board the Basic ship?"),
            },
            Ticket::Regular => self.boat_ask(boat::CHERRY_REGULAR_PATH, boat::CHERRY, boat::CHERRY_ASK_REGULAR, "board a private ship?"),
        }
    }

    /// Cherry's yes/no. `None` when it is not theirs. Yes boards; No gets their v96 line.
    pub(super) fn cherry_board_answer(&mut self, path: &str, action: i8, now: i64) -> Option<Vec<Reply>> {
        let ticket = match path {
            boat::CHERRY_BASIC_PATH => Ticket::Basic,
            boat::CHERRY_REGULAR_PATH => Ticket::Regular,
            _ => return None,
        };
        self.conversation = None;
        if action != net::script::SCRIPT_ACTION_YES {
            return Some(vec![self.boat_say(boat::CHERRY, boat::CHERRY_DECLINED, "declined".to_string())]);
        }
        Some(self.board_ship(ticket, now))
    }

    fn boat_say(&self, template: u32, line: &str, what: String) -> Reply {
        Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(template, line, false, false),
            what: format!("ScriptMessage Say from NPC {template}: {what}"),
        }
    }

    /// **Joel sells a ticket**: the mesos and the ticket change hands in one transaction
    /// (`Store::buy_item`), so a full bag never costs the fare and a short purse never gets
    /// the ticket.
    pub(super) fn buy_ticket(&mut self, ticket: Ticket) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let item = store::Item::bundle(ticket.item(), 1);
        let max_stack = self.config.shops.max_stack(ticket.item());
        match self.store.buy_item(chr.id, store::InventoryType::Etc, &item, max_stack, ticket.price()) {
            Ok(changed) => {
                crate::server::log(&format!(
                    "   boat: {} ({}) bought a {} from Joel for {} mesos",
                    chr.name,
                    chr.id,
                    ticket.name(),
                    ticket.price()
                ));
                let mut out = self.inventory_added_replies(store::InventoryType::Etc, &changed, "bought from Joel");
                out.extend(self.meso_reply(chr.id));
                out.push(Reply {
                    opcode: net::message::MESSAGE,
                    body: net::message::meso_lost_line(ticket.price()),
                    what: format!("Message: grey chat line, {} mesos for the {}", ticket.price(), ticket.name()),
                });
                out.push(self.item_chat_line(ticket.item(), 1));
                out.push(self.boat_say(boat::JOEL, &boat::joel_sold(ticket), format!("sold a {}", ticket.name())));
                out
            }
            Err(store::StoreError::NotEnoughMesos { have, .. }) => {
                vec![self.boat_say(boat::JOEL, &boat::joel_short(ticket), format!("{have} mesos is short of {}", ticket.price()))]
            }
            Err(store::StoreError::BagFull { .. }) => {
                vec![self.boat_say(boat::JOEL, boat::JOEL_BAG_FULL, "the Etc tab is full".to_string())]
            }
            Err(e) => {
                crate::server::log(&format!("   boat: Joel could not sell {} a {}: {e}", chr.id, ticket.name()));
                vec![self.boat_say(boat::JOEL, "I can't sell you a ticket just now. Please try again in a moment.", format!("the sale failed - {e}"))]
            }
        }
    }

    /// **Cherry takes the ticket.** The ticket goes only once every refusal has been ruled
    /// out, so nobody loses one to a closed gate.
    pub(super) fn board_ship(&mut self, ticket: Ticket, now: i64) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        if chr.map_id != boat::STATION {
            return Vec::new();
        }
        if self.held_count(chr.id, ticket.item()) == 0 {
            return vec![self.boat_say(boat::CHERRY, &boat::cherry_no_ticket(ticket), format!("no {}", ticket.name()))];
        }
        let departs = match (ticket, boat::boarding(now)) {
            (Ticket::Basic, boat::Boarding::Open { departs }) => Some(departs),
            (Ticket::Basic, _) => {
                let line = boat::cherry_not_boarding(now).unwrap_or_default();
                return vec![self.boat_say(boat::CHERRY, &line, format!("not boarding: {:?}", boat::boarding(now)))];
            }
            (Ticket::Regular, _) => None,
        };
        let mut out = self.take_items(chr.id, store::InventoryType::Etc, ticket.item(), 1);
        out.push(self.item_chat_line(ticket.item(), -1));
        let (voyage, map) = match departs {
            Some(departs) => (self.fields.voyages().board_shared(chr.id, departs), boat::WAITING_ROOM),
            None => (self.fields.voyages().board_private(chr.id, now), boat::DECK),
        };
        crate::server::log(&format!(
            "   boat: {} ({}) gave Cherry a {} - voyage {} ({:?}, {:?}), {} aboard",
            chr.name,
            chr.id,
            ticket.name(),
            voyage.id,
            voyage.ride,
            voyage.phase,
            voyage.members.len()
        ));
        out.extend(self.teleport(&mut chr, map, format!("the ship to Orbis, voyage {}", voyage.id)));
        out
    }

    /// Purin's yes/no, from their v96 script. Yes leaves the voyage and goes back to the station
    /// before the ship departs - the ticket is spent; No gets their line and they stay aboard.
    pub(super) fn purin_answer(&mut self, action: i8) -> Vec<Reply> {
        self.conversation = None;
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        if action == net::script::SCRIPT_ACTION_NO {
            return vec![self.boat_say(boat::PURIN, boat::PURIN_STAY, "they stay aboard".to_string())];
        }
        if action != net::script::SCRIPT_ACTION_YES || chr.map_id != boat::WAITING_ROOM {
            return Vec::new();
        }
        let _ = self.fields.voyages().drop_member(chr.id);
        crate::server::log(&format!("   boat: {} ({}) left the waiting room through Purin", chr.name, chr.id));
        self.teleport(&mut chr, boat::STATION, "Purin: back to Ellinia Station".to_string())
    }

    /// **On every field entry.** On a ship field with a voyage: the countdown. Anywhere else:
    /// off whatever voyage they were on - a return scroll, a death or a GM warp all end the
    /// crossing, and a stale membership would put them back in that ship's instance later.
    pub(super) fn boat_field_entry(&mut self) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        if !boat::is_ship_map(chr.map_id) {
            let _ = self.fields.voyages().drop_member(chr.id);
            return Vec::new();
        }
        let voyage = self.fields.voyages().voyage_of(chr.id);
        let Some(voyage) = voyage else { return Vec::new() };
        let left = voyage.remaining_s(store::Store::unix_now());
        vec![Reply {
            opcode: net::clock::FIELD_CLOCK,
            body: net::clock::clock_seconds(left),
            what: format!(
                "FieldClock type 2 to character {}: {left}s left on voyage {} ({:?}, {:?}) - the ship fields declare no clock node, and type 2 builds its own widget",
                chr.id, voyage.id, voyage.ride, voyage.phase
            ),
        }]
    }

    /// The ships' clock, on the session tick.
    pub(super) fn boat_tick(&mut self) -> Vec<Reply> {
        self.boat_tick_at(store::Store::unix_now())
    }

    /// Sail what is due and land what has arrived. Both takes are test-and-set in the
    /// registry, so whichever session ticks first moves everyone and the rest see nothing.
    pub(super) fn boat_tick_at(&mut self, now: i64) -> Vec<Reply> {
        let departing = self.fields.voyages().take_departures(now);
        let arriving = self.fields.voyages().take_arrivals(now);
        let mut out = Vec::new();
        for (voyages, map, what) in [(departing, boat::DECK, "departs"), (arriving, boat::ORBIS, "arrives")] {
            for voyage in voyages {
                crate::server::log(&format!(
                    "   boat: voyage {} ({:?}) {what} with {} aboard",
                    voyage.id,
                    voyage.ride,
                    voyage.members.len()
                ));
                for member in voyage.members {
                    if self.claimed.as_ref().map(|c| c.character_id) == Some(member) {
                        out.extend(self.boat_warp(voyage.id, map));
                    } else if !self.bus().publish_event_to_character(member, crate::broadcast::Event::BoatWarp { voyage: voyage.id, map }) {
                        crate::server::log(&format!("   boat: passenger {member} of voyage {} is not reachable on this channel; not moved to {map}", voyage.id));
                    }
                }
            }
        }
        out
    }

    /// **Move this passenger** for `voyage`: to the deck from the waiting room, or to Orbis
    /// from the deck or the cabin. Anyone no longer where the voyage left them - they took
    /// Purin's way out, or read a return scroll - stays where they are.
    pub(super) fn boat_warp(&mut self, voyage: u32, map: u32) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let aboard = match map {
            boat::DECK => chr.map_id == boat::WAITING_ROOM && self.fields.voyages().voyage_of(chr.id).map(|v| v.id) == Some(voyage),
            // The arrival has already taken the voyage out of the registry. The waiting room
            // counts too: a passenger whose departure warp never reached them (their client
            // was between a SetField and its field entry, with no presence to deliver to)
            // still arrives rather than waiting for a ship that has gone.
            boat::ORBIS => boat::is_ship_map(chr.map_id),
            _ => false,
        };
        if !aboard {
            crate::server::log(&format!(
                "   boat: {} ({}) is on map {}, not aboard voyage {voyage} - not moved to {map}",
                chr.name, chr.id, chr.map_id
            ));
            return Vec::new();
        }
        let mut out = if map == boat::ORBIS { self.notice(boat::ARRIVED.to_string()) } else { Vec::new() };
        out.extend(self.teleport(&mut chr, map, format!("the ship to Orbis, voyage {voyage}")));
        out
    }

    /// **A login never lands on a ship.** The three ship fields' `forcedReturn` is Ellinia
    /// Station [L], and a voyage does not survive a disconnect, a channel change or a restart,
    /// so the saved map is rewritten before the login's `SetField` reads it.
    pub(super) fn keep_off_ship_on_login(&mut self) {
        let Some(chr) = self.claimed_character() else { return };
        if !boat::is_ship_map(chr.map_id) {
            return;
        }
        let _ = self.fields.voyages().drop_member(chr.id);
        let result = self.store.set_character_map(chr.id, boat::STATION);
        crate::server::log(&format!(
            "   boat: {} ({}) logged in on ship map {}; sent to Ellinia Station instead - {}",
            chr.name,
            chr.id,
            chr.map_id,
            match result {
                Ok(()) => "saved".to_string(),
                Err(e) => format!("THE SAVE FAILED ({e}), so this login lands where it was saved"),
            }
        ));
    }

    /// A dropped connection leaves its voyage.
    pub(super) fn leave_ship_on_disconnect(&mut self) {
        let Some(chr) = self.claimed_character() else { return };
        let _ = self.fields.voyages().drop_member(chr.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::{FieldKey, Fields};
    use std::sync::Arc;
    use store::Store;

    const JOEL_OBJECT: u32 = 950;
    const CHERRY_OBJECT: u32 = 951;
    const PURIN_OBJECT: u32 = 952;

    /// One channel - one `Fields`, so one bus and one voyage registry - with the station, the
    /// waiting room, the ship and Orbis loaded, and `names` standing in the station with
    /// `mesos` each.
    fn station(names: &[&str], mesos: u32) -> (Arc<Store>, Arc<Fields>, Vec<(Session, u32)>) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let fields = Arc::new(Fields::new());
        let mut cfg = crate::config::Config::default();
        for m in [boat::STATION, boat::WAITING_ROOM, boat::DECK, boat::CABIN, boat::ORBIS] {
            cfg.fields.insert(m);
        }
        let npc = |object_id, template_id| net::opcode::FieldNpc { object_id, template_id, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 };
        cfg.npcs.insert(boat::STATION, vec![npc(JOEL_OBJECT, boat::JOEL), npc(CHERRY_OBJECT, boat::CHERRY)]);
        cfg.npcs.insert(boat::WAITING_ROOM, vec![npc(PURIN_OBJECT, boat::PURIN)]);
        for t in Ticket::ALL {
            cfg.item_names.insert(t.item(), t.name().into());
        }
        let config = Arc::new(cfg);
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut out = Vec::new();
        for name in names {
            let chr = net::opcode::Character { name: name.to_string(), map_id: boat::STATION, level: 30, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, boat::STATION).unwrap();
            store.set_mesos(id, mesos).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            s.on_field_entered();
            out.push((s, id));
        }
        (store, fields, out)
    }

    fn click(object_id: u32) -> Vec<u8> {
        let mut b = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
        b.extend_from_slice(&object_id.to_le_bytes());
        b.extend_from_slice(&0i16.to_le_bytes());
        b.extend_from_slice(&0i16.to_le_bytes());
        b.extend_from_slice(&u32::MAX.to_le_bytes());
        b
    }

    fn pick(line: u32) -> Vec<u8> {
        let mut b = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
        b.extend_from_slice(&0u32.to_le_bytes());
        b.push(net::script::SCRIPT_TYPE_MENU);
        b.push(1);
        b.extend_from_slice(&line.to_le_bytes());
        b
    }

    /// A Say / yes-no answer: Yes (and Next, which the client sends as the same 1), or No.
    fn answer(action: i8) -> Vec<u8> {
        let mut b = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
        b.extend_from_slice(&0u32.to_le_bytes());
        b.push(0);
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&0u16.to_le_bytes());
        b.push(action as u8);
        b
    }

    fn yes() -> Vec<u8> {
        answer(net::script::SCRIPT_ACTION_YES)
    }

    fn no() -> Vec<u8> {
        answer(net::script::SCRIPT_ACTION_NO)
    }

    /// Joel's two boxes: their introduction, then Next to the tickets.
    fn joel_tickets(s: &mut Session) -> Vec<Reply> {
        let out = s.handle(&click(JOEL_OBJECT));
        assert!(said(&out).contains("Hi there! I'm Joel"), "their introduction first: {}", said(&out));
        assert!(said(&out).contains("every 10 minutes afterwards"), "{}", said(&out));
        s.handle(&yes())
    }

    fn said(out: &[Reply]) -> String {
        out.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).map(|r| String::from_utf8_lossy(&r.body).to_string()).collect()
    }

    fn map_of(s: &Session) -> u32 {
        s.claimed_character().unwrap().map_id
    }

    fn clock_of(out: &[Reply]) -> Option<u32> {
        let r = out.iter().find(|r| r.opcode == net::clock::FIELD_CLOCK)?;
        assert_eq!(r.body[0], net::clock::CLOCK_TYPE_SECONDS, "type 2: the ship fields have no clock node");
        Some(u32::from_le_bytes(r.body[1..5].try_into().unwrap()))
    }

    /// Joel's menu sells either ticket for the owner's price, with the fare and the ticket each
    /// said in the chat log; a short purse gets a refusal and keeps both.
    #[test]
    fn joel_sells_both_tickets_from_a_menu_and_refuses_a_short_purse() {
        let (store, _, mut ss) = station(&["Buyer"], 26_000);
        let (s, id) = &mut ss[0];
        let id = *id;

        let out = joel_tickets(s);
        assert!(said(&out).contains("#L0#Ticket to Orbis (Basic)"), "a menu, not a shop: {}", said(&out));
        assert!(!out.iter().any(|r| r.opcode == net::classicshop::CLASSIC_OPEN_SHOP), "no shop window");

        let out = s.handle(&pick(Ticket::Basic.line()));
        assert!(said(&out).contains("please go talk to #bCherry#k on the right"), "{}", said(&out));
        assert_eq!(s.held_count(id, boat::BASIC_TICKET), 1);
        assert_eq!(store.mesos(id).unwrap(), 21_000, "5,000 for the Basic");
        assert!(out.iter().any(|r| r.opcode == net::message::MESSAGE && r.body == net::message::meso_lost_line(boat::BASIC_PRICE)));
        assert!(out.iter().any(|r| r.opcode == net::stats::USER_EFFECT_LOCAL && r.body == net::message::item_gained_in_chat(boat::BASIC_TICKET, 1)));
        assert!(out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "the ticket reaches the bag on screen");

        let _ = joel_tickets(s);
        let _ = s.handle(&pick(Ticket::Regular.line()));
        assert_eq!(s.held_count(id, boat::REGULAR_TICKET), 1);
        assert_eq!(store.mesos(id).unwrap(), 1_000, "20,000 for the Regular");

        // Closing the introduction sells nothing and opens nothing.
        let _ = s.handle(&click(JOEL_OBJECT));
        let out = s.handle(&answer(net::script::SCRIPT_ACTION_CLOSED));
        assert!(said(&out).is_empty(), "{}", said(&out));

        let _ = joel_tickets(s);
        let out = s.handle(&pick(Ticket::Regular.line()));
        assert!(said(&out).contains("don't have enough"), "{}", said(&out));
        assert_eq!(s.held_count(id, boat::REGULAR_TICKET), 1, "no second ticket");
        assert_eq!(store.mesos(id).unwrap(), 1_000, "and nothing taken");
    }

    /// The whole Basic crossing. Two passengers board one departure into one waiting room and
    /// a third boards the next departure, apart; the departure moves the two to one deck with
    /// five minutes on it; the cabin keeps the voyage and the clock; the arrival lands both in
    /// Orbis.
    #[test]
    fn basic_passengers_for_one_departure_wait_sail_and_arrive_together() {
        let (_, fields, mut ss) = station(&["Acorn", "Birch", "Cedar"], 10_000);
        for (s, _) in ss.iter_mut() {
            let _ = s.buy_ticket(Ticket::Basic);
        }
        let now = Store::unix_now();
        let departs = boat::next_departure(now) + boat::DEPARTURE_EVERY_S;
        let open = departs - 200;
        let (ann, owl, cat) = (ss[0].1, ss[1].1, ss[2].1);

        // Outside the window: refused, and the ticket stays.
        let out = ss[0].0.board_ship(Ticket::Basic, departs - 30);
        assert!(said(&out).contains("getting ready for takeoff"), "{}", said(&out));
        let out = ss[0].0.board_ship(Ticket::Basic, departs - 400);
        assert!(said(&out).contains("We will begin boarding"), "{}", said(&out));
        assert_eq!(ss[0].0.held_count(ann, boat::BASIC_TICKET), 1, "a refusal takes no ticket");
        assert_eq!(map_of(&ss[0].0), boat::STATION);

        // Inside it: the ticket goes, with its chat line, and they wait.
        let out = ss[0].0.board_ship(Ticket::Basic, open);
        assert!(out.iter().any(|r| r.opcode == net::stats::USER_EFFECT_LOCAL && r.body == net::message::item_lost_in_chat(boat::BASIC_TICKET, 1)));
        assert_eq!(ss[0].0.held_count(ann, boat::BASIC_TICKET), 0);
        let _ = ss[1].0.board_ship(Ticket::Basic, open + 100);
        let _ = ss[2].0.board_ship(Ticket::Basic, open + 600); // the next departure
        for (s, _) in ss.iter_mut() {
            assert_eq!(map_of(s), boat::WAITING_ROOM);
            // The client's 0x00DC after every SetField: it is what puts them back on the bus.
            let _ = s.on_field_entered();
        }
        let key = |s: &Session| s.field();
        assert_eq!(key(&ss[0].0), key(&ss[1].0), "one departure, one waiting room");
        assert!(key(&ss[0].0).is_instanced());
        assert_ne!(key(&ss[0].0), key(&ss[2].0), "the next departure waits apart");

        // Field entry carries the countdown to the departure.
        let entry = ss[0].0.on_field_entered();
        let left = clock_of(&entry).expect("a countdown in the waiting room");
        let expect = u32::try_from(departs - Store::unix_now()).unwrap();
        assert!(left.abs_diff(expect) <= 1, "{left}s against {expect}s");

        // Departure: one tick moves both - Ann directly, Owl through the bus - and not Cat.
        let _ = ss[0].0.boat_tick_at(departs);
        let _ = ss[1].0.tick(1_000);
        let _ = ss[2].0.tick(1_000);
        assert_eq!((map_of(&ss[0].0), map_of(&ss[1].0), map_of(&ss[2].0)), (boat::DECK, boat::DECK, boat::WAITING_ROOM));
        assert_eq!(key(&ss[0].0), key(&ss[1].0), "one deck");
        assert!(ss[1].0.boat_tick_at(departs).is_empty(), "a second tick sails nothing again");
        let entry = ss[1].0.on_field_entered();
        let left = clock_of(&entry).expect("a countdown on the deck");
        let expect = u32::try_from(departs + boat::RIDE_S - Store::unix_now()).unwrap();
        assert!(left.abs_diff(expect) <= 1, "five minutes from the departure: {left}s against {expect}s");

        // The cabin is the same voyage, and its clock is the same clock.
        let mut chr = ss[1].0.claimed_character().unwrap();
        let _ = ss[1].0.go_to_map(&mut chr, boat::CABIN, 2, "down to the cabin".to_string());
        assert_eq!(key(&ss[1].0), FieldKey::instanced(boat::CABIN, key(&ss[0].0).instance), "the cabin keeps the voyage");
        let entry = ss[1].0.on_field_entered();
        assert!(clock_of(&entry).unwrap().abs_diff(expect) <= 1, "the timer is not lost below deck");

        // Arrival: both to Orbis, off the voyage.
        let _ = ss[0].0.on_field_entered();
        let _ = ss[1].0.boat_tick_at(departs + boat::RIDE_S);
        let _ = ss[0].0.tick(1_000);
        assert_eq!((map_of(&ss[0].0), map_of(&ss[1].0)), (boat::ORBIS, boat::ORBIS));
        assert_eq!(fields.voyages().voyage_of(ann), None);
        assert_eq!(fields.voyages().voyage_of(owl), None);
        assert!(fields.voyages().voyage_of(cat).is_some(), "the next ship has not left");
    }

    /// The Regular ticket: straight onto a ship of one, sailing at once for one minute, whose
    /// deck is nobody else's.
    #[test]
    fn a_regular_ticket_is_a_private_one_minute_ship() {
        let (_, fields, mut ss) = station(&["Solo", "Other"], 50_000);
        let _ = ss[0].0.buy_ticket(Ticket::Regular);
        let _ = ss[1].0.buy_ticket(Ticket::Regular);
        let now = Store::unix_now();
        // Any time at all - in the last minute before a Basic departure too. The departure
        // AFTER the next one, so this ship always lands after the first one's minute is up:
        // the next departure itself can be under 30 s away.
        let closing = boat::next_departure(now) + boat::DEPARTURE_EVERY_S - 30;
        assert!(matches!(boat::boarding(closing), boat::Boarding::Closing { .. }));
        let _ = ss[0].0.board_ship(Ticket::Regular, now);
        let _ = ss[1].0.board_ship(Ticket::Regular, closing);
        assert_eq!((map_of(&ss[0].0), map_of(&ss[1].0)), (boat::DECK, boat::DECK), "no waiting room");
        assert_ne!(ss[0].0.field(), ss[1].0.field(), "each their own ship");
        let entry = ss[0].0.on_field_entered();
        assert!(clock_of(&entry).unwrap() <= 60, "one minute");
        assert_eq!(ss[0].0.held_count(ss[0].1, boat::REGULAR_TICKET), 0);

        let _ = ss[0].0.boat_tick_at(now + boat::PRIVATE_RIDE_S - 1);
        assert_eq!(map_of(&ss[0].0), boat::DECK, "not yet");
        let _ = ss[0].0.boat_tick_at(now + boat::PRIVATE_RIDE_S);
        assert_eq!(map_of(&ss[0].0), boat::ORBIS);
        assert!(fields.voyages().voyage_of(ss[1].1).is_some(), "the other ship is still out");
    }

    /// **Cherry asks before they board anyone**, as their v96 script does. Refusals come before
    /// the question; No keeps the passenger and the ticket with their v96 line; Yes boards.
    #[test]
    fn cherry_asks_first_and_no_keeps_you_and_your_ticket() {
        let (_, _, mut ss) = station(&["Rider"], 50_000);
        let (s, id) = &mut ss[0];
        let id = *id;
        let _ = s.buy_ticket(Ticket::Basic);
        let _ = s.buy_ticket(Ticket::Regular);
        let departs = boat::next_departure(Store::unix_now()) + boat::DEPARTURE_EVERY_S;

        // Basic, closing: refused outright, no question parked.
        let out = s.cherry_choice(Ticket::Basic, departs - 30);
        assert!(said(&out).contains("getting ready for takeoff"), "{}", said(&out));
        assert!(s.conversation.is_none(), "nothing to answer");
        // Basic, too early: the v96 wait-for-boarding line.
        let out = s.cherry_choice(Ticket::Basic, departs - 400);
        assert!(said(&out).contains("We will begin boarding 5 minutes before the takeoff"), "{}", said(&out));

        // Basic, open: their question, then No.
        let out = s.cherry_choice(Ticket::Basic, departs - 200);
        assert!(said(&out).contains("This will not be a short flight"), "{}", said(&out));
        let out = s.cherry_board_answer(boat::CHERRY_BASIC_PATH, net::script::SCRIPT_ACTION_NO, departs - 199).unwrap();
        assert!(said(&out).contains("You must have some business"), "{}", said(&out));
        assert_eq!((map_of(s), s.held_count(id, boat::BASIC_TICKET)), (boat::STATION, 1), "No: nobody moved, nothing taken");

        // ...and Yes.
        let _ = s.cherry_choice(Ticket::Basic, departs - 200);
        let _ = s.cherry_board_answer(boat::CHERRY_BASIC_PATH, net::script::SCRIPT_ACTION_YES, departs - 199).unwrap();
        assert_eq!((map_of(s), s.held_count(id, boat::BASIC_TICKET)), (boat::WAITING_ROOM, 0));

        // Regular, through the real packets - it does not depend on the clock. No, then Yes.
        let mut chr = s.claimed_character().unwrap();
        let _ = s.go_to_map(&mut chr, boat::STATION, 0, "back to the station".to_string());
        let _ = s.on_field_entered();
        let _ = s.handle(&click(CHERRY_OBJECT));
        let out = s.handle(&pick(Ticket::Regular.line()));
        assert!(said(&out).contains("private ship will take off"), "{}", said(&out));
        let out = s.handle(&no());
        assert!(said(&out).contains("You must have some business"), "{}", said(&out));
        assert_eq!((map_of(s), s.held_count(id, boat::REGULAR_TICKET)), (boat::STATION, 1));
        let _ = s.handle(&click(CHERRY_OBJECT));
        let _ = s.handle(&pick(Ticket::Regular.line()));
        let _ = s.handle(&yes());
        assert_eq!((map_of(s), s.held_count(id, boat::REGULAR_TICKET)), (boat::DECK, 0), "Yes: aboard");
    }

    /// Cherry without the ticket refuses; Purin takes a waiting passenger back to the station
    /// and off the voyage, so its departure leaves them behind.
    #[test]
    fn no_ticket_no_ride_and_purin_is_the_way_back() {
        let (_, fields, mut ss) = station(&["Late"], 0);
        let (s, id) = &mut ss[0];
        let id = *id;
        let _ = s.handle(&click(CHERRY_OBJECT));
        let out = s.handle(&pick(Ticket::Basic.line()));
        assert!(said(&out).contains("You don't have a #bTicket to Orbis (Basic)"), "{}", said(&out));
        assert_eq!(map_of(s), boat::STATION);

        let _ = s.give_item(boat::BASIC_TICKET, 1, "test").unwrap();
        let departs = boat::next_departure(Store::unix_now()) + boat::DEPARTURE_EVERY_S;
        let _ = s.board_ship(Ticket::Basic, departs - 100);
        assert_eq!(map_of(s), boat::WAITING_ROOM);
        // No first: their v96 line, and they stay aboard the same voyage.
        let out = s.handle(&click(PURIN_OBJECT));
        assert!(said(&out).contains("Are you sure you want to get off the ship?"), "{}", said(&out));
        assert!(said(&out).contains("not#k be returned"), "{}", said(&out));
        let out = s.handle(&no());
        assert!(said(&out).contains("You'll get to your destination in a short while"), "{}", said(&out));
        assert_eq!(map_of(s), boat::WAITING_ROOM, "No: still aboard");
        assert!(fields.voyages().voyage_of(id).is_some(), "and still on the voyage");
        // Then Yes: off the ship, before it leaves.
        let _ = s.handle(&click(PURIN_OBJECT));
        let _ = s.handle(&yes());
        assert_eq!(map_of(s), boat::STATION);
        assert_eq!(fields.voyages().voyage_of(id), None);
        let _ = s.boat_tick_at(departs);
        assert_eq!(map_of(s), boat::STATION, "the ship left without them");
    }

    /// A login saved on any ship field lands in Ellinia Station - through the real login
    /// path, the client's migration hello - and a disconnect leaves the voyage. The station
    /// itself is the control: a login there stays there.
    #[test]
    fn a_login_on_a_ship_lands_in_the_station() {
        for map in [boat::WAITING_ROOM, boat::DECK, boat::CABIN, boat::STATION] {
            let (store, fields, mut ss) = station(&["Sailor"], 0);
            let (s, id) = &mut ss[0];
            let id = *id;
            let _ = fields.voyages().board_private(id, Store::unix_now());
            store.set_character_map(id, map).unwrap();
            s.leave_ship_on_disconnect();
            assert_eq!(fields.voyages().voyage_of(id), None, "a disconnect leaves the voyage");

            let account = s.claimed.as_ref().unwrap().account_id;
            store.create_migration(account, id, 0, 0).unwrap();
            let mut again = Session::joining(store.clone(), s.config.clone(), fields.clone());
            again.claim_for_character(id);
            let out = again.handle(&crate::session::CLIENT_MIGRATION_HELLO.to_le_bytes());
            let set = out.iter().find(|r| r.opcode == net::opcode::SET_FIELD).expect("the login SetField");
            assert!(set.what.contains(&format!("carrying map {} ", boat::STATION)), "saved on {map}: {}", set.what);
            assert_eq!(map_of(&again), boat::STATION, "saved on {map}, and the record says so too");
        }
    }
}
