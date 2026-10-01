//! The ships between Ellinia Station and Orbis, on the wire, both ways. The timetable, the
//! tickets, the two routes and each channel's voyages are `crate::boat`.
//!
//! * **The seller** - Joel in Ellinia Station, Agatha in the Orbis Ticketing Booth - opens with
//!   a Say and Next brings up a menu of the route's two tickets (`0x055B` type 6).
//! * **The Platform Usher**, in the booth, takes you to the tunnel to Rini's platform - and
//!   nothing else: their ferry was removed on 2026-09-29 (the owner: *"in Orbis, the ferry to El Nath
//!   or Sleepywood should NOT exist"*).
//! * **The boarder** - Cherry, or Rini - takes a ticket. A Basic one inside the boarding window
//!   puts the passenger in the waiting room with everyone else on that departure; a Regular
//!   one puts them in a waiting room of their own for ten seconds, then on their own ship.
//! * **The tick** sails every waiting voyage whose departure has come and lands every sailing
//!   voyage whose crossing is over - this session's own passenger directly, the rest through
//!   `Event::BoatWarp`, because only a passenger's own session can build their `SetField`.
//! * **Every field entry** on a ship field carries the countdown, type 2 as the party quest's
//!   is, so walking between the deck and the cabin re-sends the same time rather than
//!   starting a new one.
//!
//! Nothing here authenticates: a ride is sold to whoever holds the socket.

use super::{Conversation, Reply, Session};
use crate::boat::{self, Route, Ticket};

impl Session {
    /// A click on a seller, a boarder or a steward where they stand, or on the Platform Usher
    /// in the booth. `None` for any other NPC or map, so the ordinary click chain carries on.
    pub(super) fn open_boat_npc(&mut self, template: u32) -> Option<Vec<Reply>> {
        let chr = self.claimed_character()?;
        if template == boat::PLATFORM_USHER && chr.map_id == boat::ORBIS {
            return Some(self.boat_menu(boat::USHER_PATH, template, &boat::usher_menu(), "which platform"));
        }
        let r = boat::route_of_npc(template)?;
        if template == r.seller && chr.map_id == r.seller_map {
            // The seller opens with the v96 introduction, and Next brings up the tickets.
            self.park_boat(template, boat::SELLER_INTRO_PATH, false, true);
            return Some(vec![Reply {
                opcode: net::script::SCRIPT_MESSAGE,
                body: net::script::npc_say(template, &boat::seller_intro(r), false, true),
                what: format!("ScriptMessage Say from {}: the ship to {} and its timetable; Next opens the tickets", r.seller_name, r.to),
            }]);
        }
        if template == r.boarder && chr.map_id == r.station {
            return Some(self.boat_menu(boat::BOARDER_PATH, template, &boat::boarder_menu(r), "which ticket"));
        }
        if template == r.steward && chr.map_id == r.waiting_room {
            return Some(self.boat_ask(boat::STEWARD_PATH, template, boat::STEWARD_ASK, "get off the ship?"));
        }
        None
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

    fn boat_say(&self, template: u32, line: &str, what: String) -> Reply {
        Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(template, line, false, false),
            what: format!("ScriptMessage Say from NPC {template}: {what}"),
        }
    }

    /// **A Say or yes/no answered, on any of this module's conversations.** `None` when the
    /// parked conversation is not a boat one, so the caller carries on. The route comes from
    /// the NPC the conversation is with.
    pub(super) fn boat_script_answer(&mut self, action: i8, now: i64) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        let path = convo.path.as_str();
        if !boat::is_boat_path(path) || path == boat::SELLER_PATH || path == boat::BOARDER_PATH || path == boat::USHER_PATH {
            return None; // not a boat conversation, or a menu - `boat_menu_answer` has those
        }
        self.conversation = None;
        let yes = action == net::script::SCRIPT_ACTION_YES;
        if path == boat::USHER_ASK_PATH {
            return Some(self.usher_answer(yes));
        }
        let r = boat::route_of_npc(convo.npc_template)?;
        Some(match path {
            boat::SELLER_INTRO_PATH if yes => {
                self.boat_menu(boat::SELLER_PATH, r.seller, &boat::seller_menu(r), &format!("the two tickets to {}", r.to))
            }
            boat::BOARDER_BASIC_PATH | boat::BOARDER_REGULAR_PATH if yes => {
                let ticket = if path == boat::BOARDER_BASIC_PATH { Ticket::Basic } else { Ticket::Regular };
                self.board_ship(r, ticket, now)
            }
            boat::BOARDER_BASIC_PATH | boat::BOARDER_REGULAR_PATH if action == net::script::SCRIPT_ACTION_NO => {
                vec![self.boat_say(r.boarder, boat::BOARD_DECLINED, "declined".to_string())]
            }
            boat::STEWARD_PATH => self.steward_answer(r, action),
            _ => Vec::new(),
        })
    }

    /// A seller's, a boarder's or the Platform Usher's menu came back. `None` when none of
    /// them is parked, so the other menu answers get their turn.
    pub(super) fn boat_menu_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        let path = convo.path.as_str();
        if path != boat::SELLER_PATH && path != boat::BOARDER_PATH && path != boat::USHER_PATH {
            return None;
        }
        let reply = net::script::parse_menu_reply(body)?;
        self.conversation = None;
        if path == boat::USHER_PATH {
            return Some(match reply.selection {
                Some(boat::USHER_TO_VICTORIA) => {
                    self.boat_ask(boat::USHER_ASK_PATH, boat::PLATFORM_USHER, boat::USHER_ASK, "to the platform to Victoria Island?")
                }
                _ => Vec::new(),
            });
        }
        let r = boat::route_of_npc(convo.npc_template)?;
        let Some(ticket) = reply.selection.and_then(Ticket::from_line) else { return Some(Vec::new()) };
        if path == boat::SELLER_PATH {
            return Some(self.buy_ticket(r, ticket));
        }
        Some(self.boarder_choice(r, ticket, store::Store::unix_now()))
    }

    /// **The Platform Usher's yes/no.** Yes teleports to the tunnel - the owner, 2026-09-27: *"offer
    /// the players a choice to be teleported to the correct tunnel to the ship"* - which walks
    /// on to Rini's platform, and back to the booth through its `west00`.
    fn usher_answer(&mut self, yes: bool) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        if !yes {
            return vec![self.boat_say(boat::PLATFORM_USHER, boat::USHER_DECLINED, "declined".to_string())];
        }
        if chr.map_id != boat::ORBIS {
            return Vec::new();
        }
        self.teleport(&mut chr, boat::ORBIS_TUNNEL, "the Platform Usher: to the tunnel to the ship to Victoria Island".to_string())
    }

    /// **The boarder, once the ticket is named.** Every refusal is said here, before they ask
    /// - no ticket, or (Basic) the ship is not boarding - so a Yes is never followed by a No.
    /// The Yes re-checks both anyway ([`Session::board_ship`]): the window can close while the
    /// question is on screen.
    pub(super) fn boarder_choice(&mut self, r: &Route, ticket: Ticket, now: i64) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        if self.held_count(chr.id, r.item(ticket)) == 0 {
            return vec![self.boat_say(r.boarder, &boat::boarder_no_ticket(r, ticket), format!("no {}", r.ticket_name(ticket)))];
        }
        match ticket {
            Ticket::Basic => match boat::not_boarding(now) {
                Some(line) => vec![self.boat_say(r.boarder, &line, format!("not boarding: {:?}", boat::boarding(now)))],
                None => self.boat_ask(boat::BOARDER_BASIC_PATH, r.boarder, boat::BOARD_ASK_BASIC, "board the Basic ship?"),
            },
            Ticket::Regular => self.boat_ask(boat::BOARDER_REGULAR_PATH, r.boarder, boat::BOARD_ASK_REGULAR, "board a private ship?"),
        }
    }

    /// **The seller sells a ticket**: the mesos and the ticket change hands in one
    /// transaction (`Store::buy_item`), so a full bag never costs the fare and a short purse
    /// never gets the ticket.
    pub(super) fn buy_ticket(&mut self, r: &Route, ticket: Ticket) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let (item_id, name) = (r.item(ticket), r.ticket_name(ticket));
        let item = store::Item::bundle(item_id, 1);
        let max_stack = self.config.shops.max_stack(item_id);
        match self.store.buy_item(chr.id, store::InventoryType::Etc, &item, max_stack, ticket.price()) {
            Ok(changed) => {
                crate::server::log(&format!(
                    "   boat: {} ({}) bought a {name} from {} for {} mesos",
                    chr.name,
                    chr.id,
                    r.seller_name,
                    ticket.price()
                ));
                let mut out = self.inventory_added_replies(store::InventoryType::Etc, &changed, "bought from the ticket seller");
                out.extend(self.meso_reply(chr.id));
                out.push(Reply {
                    opcode: net::message::MESSAGE,
                    body: net::message::meso_lost_line(ticket.price()),
                    what: format!("Message: grey chat line, {} mesos for the {name}", ticket.price()),
                });
                out.push(self.item_chat_line(item_id, 1));
                out.push(self.boat_say(r.seller, &boat::seller_sold(r, ticket), format!("sold a {name}")));
                out
            }
            Err(store::StoreError::NotEnoughMesos { have, .. }) => {
                vec![self.boat_say(r.seller, &boat::seller_short(r, ticket), format!("{have} mesos is short of {}", ticket.price()))]
            }
            Err(store::StoreError::BagFull { .. }) => {
                vec![self.boat_say(r.seller, boat::SELLER_BAG_FULL, "the Etc tab is full".to_string())]
            }
            Err(e) => {
                crate::server::log(&format!("   boat: {} could not sell {} a {name}: {e}", r.seller_name, chr.id));
                vec![self.boat_say(r.seller, "I can't sell you a ticket just now. Please try again in a moment.", format!("the sale failed - {e}"))]
            }
        }
    }

    /// **The boarder takes the ticket.** The ticket goes only once every refusal has been ruled
    /// out, so nobody loses one to a closed gate.
    pub(super) fn board_ship(&mut self, r: &Route, ticket: Ticket, now: i64) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        if chr.map_id != r.station {
            return Vec::new();
        }
        if self.held_count(chr.id, r.item(ticket)) == 0 {
            return vec![self.boat_say(r.boarder, &boat::boarder_no_ticket(r, ticket), format!("no {}", r.ticket_name(ticket)))];
        }
        let departs = match (ticket, boat::boarding(now)) {
            (Ticket::Basic, boat::Boarding::Open { departs }) => Some(departs),
            (Ticket::Basic, _) => {
                let line = boat::not_boarding(now).unwrap_or_default();
                return vec![self.boat_say(r.boarder, &line, format!("not boarding: {:?}", boat::boarding(now)))];
            }
            (Ticket::Regular, _) => None,
        };
        let mut out = self.take_items(chr.id, store::InventoryType::Etc, r.item(ticket), 1);
        out.push(self.item_chat_line(r.item(ticket), -1));
        // Both kinds wait in the waiting room: a shared ship until its departure, a private
        // one for `PRIVATE_WAIT_S` in a room of its own (the owner, 2026-09-29).
        let voyage = match departs {
            Some(departs) => self.fields.voyages().board_shared(chr.id, r.id, departs),
            None => self.fields.voyages().board_private(chr.id, r.id, now),
        };
        let map = r.waiting_room;
        crate::server::log(&format!(
            "   boat: {} ({}) boarded the ship to {} with a {} - voyage {} ({:?}, {:?}), {} aboard",
            chr.name,
            chr.id,
            r.to,
            r.ticket_name(ticket),
            voyage.id,
            voyage.ride,
            voyage.phase,
            voyage.members.len()
        ));
        out.extend(self.teleport(&mut chr, map, format!("the ship to {}, voyage {}", r.to, voyage.id)));
        out
    }

    /// The steward's yes/no, from the v96 script. Yes leaves the voyage and goes back to the
    /// departure platform before the ship leaves - the ticket is spent; No gets the line and
    /// they stay aboard.
    fn steward_answer(&mut self, r: &Route, action: i8) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        if action == net::script::SCRIPT_ACTION_NO {
            return vec![self.boat_say(r.steward, boat::STEWARD_STAY, "they stay aboard".to_string())];
        }
        if action != net::script::SCRIPT_ACTION_YES || chr.map_id != r.waiting_room {
            return Vec::new();
        }
        let _ = self.fields.voyages().drop_member(chr.id);
        crate::server::log(&format!("   boat: {} ({}) got off the ship to {} before it left", chr.name, chr.id, r.to));
        self.teleport(&mut chr, r.station, format!("off the ship to {} before it left", r.to))
    }

    /// **On every field entry.** On a ship field with a voyage: the countdown. Anywhere else:
    /// off whatever voyage they were on - a return scroll, a death or a GM warp all end the
    /// crossing, and a stale membership would put them back in that ship's instance later.
    /// And on a departure platform, the ship.
    pub(super) fn boat_field_entry(&mut self) -> Vec<Reply> {
        self.boat_field_entry_at(store::Store::unix_now())
    }

    pub(super) fn boat_field_entry_at(&mut self, now: i64) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        if !boat::is_ship_map(chr.map_id) {
            let _ = self.fields.voyages().drop_member(chr.id);
            if boat::is_station(chr.map_id) {
                return vec![station_ship_on_entry(chr.id, chr.map_id, now)];
            }
            return Vec::new();
        }
        let voyage = self.fields.voyages().voyage_of(chr.id);
        let Some(voyage) = voyage else { return Vec::new() };
        let left = voyage.remaining_s(now);
        // **An invaded deck shows the Balrog's ship** to whoever arrives on it after the
        // minute - up from the cabin, or a controller handing over. The Balrogs themselves come
        // with the ordinary field entry, like any mob alive on the field.
        let enemy = (chr.map_id == boat::route(voyage.route).deck && voyage.invaded).then(|| Reply {
            opcode: net::ship::CONTI_STATE,
            body: net::ship::deck_invaded(),
            what: format!("ContiState 0x01C0 to character {}: voyage {} is invaded - the Balrog's ship is alongside (state 3, flag 1)", chr.id, voyage.id),
        });
        vec![Reply {
            opcode: net::clock::FIELD_CLOCK,
            body: net::clock::clock_seconds(left),
            what: format!(
                "FieldClock type 2 to character {}: {left}s left on voyage {} ({:?}, {:?}) - the ship fields declare no clock node, and type 2 builds its own widget",
                chr.id, voyage.id, voyage.ride, voyage.phase
            ),
        }]
        .into_iter()
        .chain(enemy)
        .collect()
    }

    /// The ships' clock, on the session tick.
    pub(super) fn boat_tick(&mut self) -> Vec<Reply> {
        self.boat_tick_at(store::Store::unix_now())
    }

    /// Sail what is due and land what has arrived. Both takes are test-and-set in the
    /// registry, so whichever session ticks first moves everyone and the rest see nothing.
    pub(super) fn boat_tick_at(&mut self, now: i64) -> Vec<Reply> {
        // **The ship coming in at `:x5` or leaving at `:x0`, to everyone on each platform** -
        // once per channel, by whichever session ticks first. Both routes keep one timetable,
        // so one change moves both ships. `publish_to_map` leaves nobody out, so a ticker
        // standing on a platform hears it through its own mailbox.
        let change = self.fields.voyages().take_station_change(now);
        if let Some(docked) = change {
            let (body, what) = if docked {
                (net::ship::ship_arrives(), "the ship ARRIVES (type 12, state 6)")
            } else {
                (net::ship::ship_leaves(), "the ship LEAVES (type 8, state 2)")
            };
            for r in boat::ROUTES {
                let n = self.bus().publish_to_map(
                    crate::fields::FieldKey::world(r.station),
                    Reply {
                        opcode: net::ship::CONTI_MOVE,
                        body: body.clone(),
                        what: format!("ContiMove 0x01BF: {what} at the platform to {}, {}", r.to, boat::hh_mm(now)),
                    },
                );
                crate::server::log(&format!("   boat: {what} at the platform to {} - told {n} player(s) there", r.to));
            }
        }
        let fields = self.fields.clone();
        let rng = &mut self.rng;
        let departing = fields.voyages().take_departures(now, || rng.next());
        let arriving = self.fields.voyages().take_arrivals(now);
        let mut out = Vec::new();
        let moves = departing.into_iter().map(|v| (boat::route(v.route).deck, "departs", v));
        let moves: Vec<_> = moves.chain(arriving.into_iter().map(|v| (boat::route(v.route).arrival, "arrives", v))).collect();
        for (map, what, voyage) in moves {
            crate::server::log(&format!(
                "   boat: voyage {} ({:?}) to {} {what} with {} aboard",
                voyage.id,
                voyage.ride,
                boat::route(voyage.route).to,
                voyage.members.len()
            ));
            for member in &voyage.members {
                if self.claimed.as_ref().map(|c| c.character_id) == Some(*member) {
                    out.extend(self.boat_warp(voyage.id, map));
                } else if !self.bus().publish_event_to_character(*member, crate::broadcast::Event::BoatWarp { voyage: voyage.id, map }) {
                    crate::server::log(&format!("   boat: passenger {member} of voyage {} is not reachable on this channel; not moved to {map}", voyage.id));
                }
            }
            if what == "departs" && voyage.invasion_at.is_some() {
                crate::server::log(&format!("   boat: voyage {} will be INVADED by two Crimson Balrogs a minute in", voyage.id));
            }
        }
        out.extend(self.boat_invasion_at(now));
        out
    }

    /// **The Crimson Balrogs, a minute into an invaded crossing.** Done by a passenger's own
    /// session, and only one standing on the deck: the mobs need a controller, and the
    /// controller has to be a client that can see them (`crate::mobshare`). `take_invasion`
    /// is the test-and-set, so two passengers on the deck cannot both summon a pair. If every
    /// passenger is below in the cabin at the minute, the first to step back onto the deck
    /// brings them.
    fn boat_invasion_at(&mut self, now: i64) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let voyage = self.fields.voyages().voyage_of(chr.id);
        let Some(voyage) = voyage else { return Vec::new() };
        let r = boat::route(voyage.route);
        if chr.map_id != r.deck {
            return Vec::new();
        }
        let invade = self.fields.voyages().take_invasion(voyage.id, now);
        if !invade {
            return Vec::new();
        }
        let map = crate::fields::FieldKey::instanced(r.deck, voyage.id);
        crate::server::log(&format!(
            "   boat: voyage {} to {} is INVADED - the Balrog's ship and two Crimson Balrogs, by {} ({})",
            voyage.id, r.to, chr.name, chr.id
        ));
        // The ship first, so the Balrogs come out of something.
        let ship = Reply {
            opcode: net::ship::CONTI_MOVE,
            body: net::ship::enemy_ship_arrives(),
            what: format!("ContiMove 0x01BF: the Crimson Balrog's ship comes alongside voyage {} (type 10, state 4)", voyage.id),
        };
        self.bus().publish(self.subscriber, map, ship.clone(), None);
        let mut out = vec![ship];
        for at in r.invaders_at() {
            out.extend(self.spawn_invader(map, at));
        }
        out
    }

    /// One Crimson Balrog at `at` on the deck `map`, for everyone there, controlled by this
    /// client. The summon sack's path (`session/summonsack.rs`) without the sack, and without
    /// its summoning effect: that effect leaves a mob untargetable until a `0x03E8` follows,
    /// and nothing here needs one. **No foothold**: the Balrog flies (the owner), so it is put in
    /// the air where the Balrog's ship is, `fh` 0 - as the sack does when there is no floor.
    fn spawn_invader(&mut self, map: crate::fields::FieldKey, at: (i16, i16)) -> Vec<Reply> {
        let hp = self.config.mob_templates.get(&boat::CRIMSON_BALROG).map(|t| u64::from(t.max_hp)).unwrap_or(1);
        let live = self.fields.summon_mob(map, boat::CRIMSON_BALROG, at, 0, hp);
        let mut mob = live.as_seen();
        mob.forced_stat = self.forced_stat_for(mob.template_id);
        let spawn = Reply {
            opcode: net::mob::MOB_ENTER_FIELD,
            body: net::mob::mob_enter_field(&mob),
            what: format!(
                "MobEnterField: Crimson Balrog {} INVADES {map} at {at:?}, object id {}, hp {hp}",
                mob.template_id, mob.object_id
            ),
        };
        self.bus().publish(self.subscriber, map, spawn.clone(), None);
        let mut out = vec![spawn];
        if self.fields.controllers().claim_one(map, mob.object_id, self.subscriber.get()) {
            out.push(Reply {
                opcode: net::mobmove::MOB_CHANGE_CONTROLLER,
                body: net::mobmove::mob_change_controller(&mob, net::mobmove::CONTROL_NORMAL),
                what: format!("MobChangeController: invading Balrog {} to this client, which claimed it", mob.object_id),
            });
        }
        out
    }

    /// **Move this passenger** for `voyage`: to a route's deck from its waiting room, or to its
    /// arrival from any of its ship fields. Anyone no longer where the voyage left them - they
    /// got off through the steward, or read a return scroll - stays where they are.
    pub(super) fn boat_warp(&mut self, voyage: u32, map: u32) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let departing = boat::ROUTES.iter().find(|r| r.deck == map);
        let arriving = boat::ROUTES.iter().find(|r| r.arrival == map);
        let aboard = match (departing, arriving) {
            (Some(r), _) => chr.map_id == r.waiting_room && self.fields.voyages().voyage_of(chr.id).map(|v| v.id) == Some(voyage),
            // The arrival has already taken the voyage out of the registry. The waiting room
            // counts too: a passenger whose departure warp never reached them (their client
            // was between a SetField and its field entry, with no presence to deliver to)
            // still arrives rather than waiting for a ship that has gone.
            (None, Some(r)) => r.owns_ship_map(chr.map_id),
            (None, None) => false,
        };
        if !aboard {
            crate::server::log(&format!(
                "   boat: {} ({}) is on map {}, not aboard voyage {voyage} - not moved to {map}",
                chr.name, chr.id, chr.map_id
            ));
            return Vec::new();
        }
        let mut out = match (departing, arriving) {
            (None, Some(r)) => self.notice(boat::arrived(r)),
            _ => Vec::new(),
        };
        out.extend(self.teleport(&mut chr, map, format!("the ship, voyage {voyage}")));
        out
    }

    /// **A login never lands on a ship.** A voyage does not survive a disconnect, a channel
    /// change or a restart, so the saved map is rewritten to the route's departure platform
    /// before the login's `SetField` reads it - the owner, 2026-09-26: *"put the player back to the
    /// respective station before their departure."*
    pub(super) fn keep_off_ship_on_login(&mut self) {
        let Some(chr) = self.claimed_character() else { return };
        let Some(r) = boat::route_of_ship_map(chr.map_id) else { return };
        let _ = self.fields.voyages().drop_member(chr.id);
        let result = self.store.set_character_map(chr.id, r.station);
        crate::server::log(&format!(
            "   boat: {} ({}) logged in on ship map {}; sent to the platform to {} ({}) instead - {}",
            chr.name,
            chr.id,
            chr.map_id,
            r.to,
            r.station,
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

/// **The ship, for someone arriving on a departure platform**: the arrive animation from `:x5`
/// until the departure, the leave animation at any other time - the owner, 2026-09-26. `0x01C0`,
/// whose state-1 and state-2 arms call the same two routines as the live `0x01BF`
/// announcements. Both platforms are `fieldType 2` with a `shipKind 0` `shipObj` [L].
fn station_ship_on_entry(chr_id: u32, map: u32, now: i64) -> Reply {
    let docked = boat::ship_docked(now);
    Reply {
        opcode: net::ship::CONTI_STATE,
        body: net::ship::station_state(docked),
        what: format!(
            "ContiState 0x01C0 to character {chr_id} on {map}: the ship {} at {} (next departure {})",
            if docked { "ARRIVES - it is in for boarding" } else { "LEAVES - it is not in" },
            boat::hh_mm(now),
            boat::hh_mm(boat::next_departure(now)),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::{FieldKey, Fields};
    use std::sync::Arc;
    use store::Store;

    /// The route most tests ride. The Orbis side has its own tests below.
    const O: &Route = &boat::TO_ORBIS;
    const E: &Route = &boat::TO_ELLINIA;

    const JOEL_OBJECT: u32 = 950;
    const CHERRY_OBJECT: u32 = 951;
    const PURIN_OBJECT: u32 = 952;
    const AGATHA_OBJECT: u32 = 960;
    const USHER_OBJECT: u32 = 961;
    const RINI_OBJECT: u32 = 962;
    const ERIN_OBJECT: u32 = 963;

    /// One channel - one `Fields`, so one bus and one voyage registry - with both routes'
    /// maps and NPCs loaded, and `names` standing in Ellinia Station with `mesos` each.
    fn station(names: &[&str], mesos: u32) -> (Arc<Store>, Arc<Fields>, Vec<(Session, u32)>) {
        station_at(boat::STATION, names, mesos)
    }

    /// [`station`], standing on `start` instead.
    fn station_at(start: u32, names: &[&str], mesos: u32) -> (Arc<Store>, Arc<Fields>, Vec<(Session, u32)>) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let fields = Arc::new(Fields::new());
        let mut cfg = crate::config::Config::default();
        for r in boat::ROUTES {
            for m in [r.seller_map, r.station, r.waiting_room, r.deck, r.cabin, r.arrival] {
                cfg.fields.insert(m);
            }
        }
        cfg.fields.insert(boat::ORBIS_TUNNEL);
        let npc = |object_id, template_id| net::opcode::FieldNpc { object_id, template_id, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 };
        cfg.npcs.insert(boat::STATION, vec![npc(JOEL_OBJECT, boat::JOEL), npc(CHERRY_OBJECT, boat::CHERRY)]);
        cfg.npcs.insert(boat::WAITING_ROOM, vec![npc(PURIN_OBJECT, boat::PURIN)]);
        cfg.npcs.insert(boat::ORBIS, vec![npc(AGATHA_OBJECT, boat::AGATHA), npc(USHER_OBJECT, boat::PLATFORM_USHER)]);
        cfg.npcs.insert(boat::ORBIS_STATION, vec![npc(RINI_OBJECT, boat::RINI)]);
        cfg.npcs.insert(boat::ORBIS_WAITING_ROOM, vec![npc(ERIN_OBJECT, boat::ERIN)]);
        for r in boat::ROUTES {
            for t in Ticket::ALL {
                cfg.item_names.insert(r.item(t), r.ticket_name(t));
            }
        }
        let config = Arc::new(cfg);
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut out = Vec::new();
        for name in names {
            let chr = net::opcode::Character { name: name.to_string(), map_id: start, level: 30, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, start).unwrap();
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
            let _ = s.buy_ticket(O, Ticket::Basic);
        }
        let now = Store::unix_now();
        let departs = boat::next_departure(now) + boat::DEPARTURE_EVERY_S;
        let open = departs - 200;
        let (ann, owl, cat) = (ss[0].1, ss[1].1, ss[2].1);

        // Outside the window: refused, and the ticket stays.
        let out = ss[0].0.board_ship(O, Ticket::Basic, departs - 30);
        assert!(said(&out).contains("getting ready for takeoff"), "{}", said(&out));
        let out = ss[0].0.board_ship(O, Ticket::Basic, departs - 400);
        assert!(said(&out).contains("We will begin boarding"), "{}", said(&out));
        assert_eq!(ss[0].0.held_count(ann, boat::BASIC_TICKET), 1, "a refusal takes no ticket");
        assert_eq!(map_of(&ss[0].0), boat::STATION);

        // Inside it: the ticket goes, with its chat line, and they wait.
        let out = ss[0].0.board_ship(O, Ticket::Basic, open);
        assert!(out.iter().any(|r| r.opcode == net::stats::USER_EFFECT_LOCAL && r.body == net::message::item_lost_in_chat(boat::BASIC_TICKET, 1)));
        assert_eq!(ss[0].0.held_count(ann, boat::BASIC_TICKET), 0);
        let _ = ss[1].0.board_ship(O, Ticket::Basic, open + 100);
        let _ = ss[2].0.board_ship(O, Ticket::Basic, open + 600); // the next departure
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

    /// The Regular ticket: ten seconds in a waiting room of its own, then a ship of one for a
    /// minute, whose deck is nobody else's. The owner, 2026-09-29: *"The before travel should also
    /// last 10 seconds ... This should happen in both directions."*
    #[test]
    fn a_regular_ticket_waits_ten_seconds_alone_then_sails_a_private_minute() {
        let (_, fields, mut ss) = station(&["Solo", "Other"], 50_000);
        let _ = ss[0].0.buy_ticket(O, Ticket::Regular);
        let _ = ss[1].0.buy_ticket(O, Ticket::Regular);
        let now = Store::unix_now();
        // Any time at all - in the last minute before a Basic departure too. The departure
        // AFTER the next one, so this ship always lands after the first one's is done.
        let closing = boat::next_departure(now) + boat::DEPARTURE_EVERY_S - 30;
        assert!(matches!(boat::boarding(closing), boat::Boarding::Closing { .. }));
        let _ = ss[0].0.board_ship(O, Ticket::Regular, now);
        let _ = ss[1].0.board_ship(O, Ticket::Regular, closing);
        assert_eq!((map_of(&ss[0].0), map_of(&ss[1].0)), (boat::WAITING_ROOM, boat::WAITING_ROOM), "the waiting room first");
        assert_ne!(ss[0].0.field(), ss[1].0.field(), "each their own waiting room");
        let entry = ss[0].0.boat_field_entry_at(now);
        assert_eq!(clock_of(&entry), Some(boat::PRIVATE_WAIT_S as u32), "a ten-second countdown");
        assert_eq!(ss[0].0.held_count(ss[0].1, boat::REGULAR_TICKET), 0);

        let t = now + boat::PRIVATE_WAIT_S;
        let _ = ss[0].0.boat_tick_at(t - 1);
        assert_eq!(map_of(&ss[0].0), boat::WAITING_ROOM, "not yet");
        let _ = ss[0].0.boat_tick_at(t);
        assert_eq!(map_of(&ss[0].0), boat::DECK, "ten seconds, then the deck");
        assert_ne!(ss[0].0.field(), ss[1].0.field(), "each their own ship");
        let entry = ss[0].0.boat_field_entry_at(t);
        assert_eq!(clock_of(&entry), Some(boat::PRIVATE_RIDE_S as u32), "a one-minute countdown");

        let _ = ss[0].0.boat_tick_at(t + boat::PRIVATE_RIDE_S - 1);
        assert_eq!(map_of(&ss[0].0), boat::DECK, "not yet");
        let _ = ss[0].0.boat_tick_at(t + boat::PRIVATE_RIDE_S);
        assert_eq!(map_of(&ss[0].0), boat::ORBIS);
        assert!(fields.voyages().voyage_of(ss[1].1).is_some(), "the other ship is still out");

        // The other direction, the same shape: Orbis platform -> its waiting room -> its deck.
        let e = &boat::TO_ELLINIA;
        let (_, _, mut back) = station_at(e.station, &["Home"], 50_000);
        let (s, id) = &mut back[0];
        let _ = s.buy_ticket(e, Ticket::Regular);
        let _ = s.board_ship(e, Ticket::Regular, now);
        assert_eq!(map_of(s), e.waiting_room, "Erin's waiting room first");
        let _ = s.boat_tick_at(now + boat::PRIVATE_WAIT_S);
        assert_eq!(map_of(s), e.deck);
        let _ = s.boat_tick_at(now + boat::PRIVATE_WAIT_S + boat::PRIVATE_RIDE_S);
        assert_eq!(map_of(s), e.arrival);
        assert_eq!(s.held_count(*id, e.item(Ticket::Regular)), 0);
    }

    /// **Cherry asks before they board anyone**, as their v96 script does. Refusals come before
    /// the question; No keeps the passenger and the ticket with their v96 line; Yes boards.
    #[test]
    fn cherry_asks_first_and_no_keeps_you_and_your_ticket() {
        let (_, _, mut ss) = station(&["Rider"], 50_000);
        let (s, id) = &mut ss[0];
        let id = *id;
        let _ = s.buy_ticket(O, Ticket::Basic);
        let _ = s.buy_ticket(O, Ticket::Regular);
        let departs = boat::next_departure(Store::unix_now()) + boat::DEPARTURE_EVERY_S;

        // Basic, closing: refused outright, no question parked.
        let out = s.boarder_choice(O, Ticket::Basic, departs - 30);
        assert!(said(&out).contains("getting ready for takeoff"), "{}", said(&out));
        assert!(s.conversation.is_none(), "nothing to answer");
        // Basic, too early: the v96 wait-for-boarding line.
        let out = s.boarder_choice(O, Ticket::Basic, departs - 400);
        assert!(said(&out).contains("We will begin boarding 5 minutes before the takeoff"), "{}", said(&out));

        // Basic, open: their question, then No.
        let out = s.boarder_choice(O, Ticket::Basic, departs - 200);
        assert!(said(&out).contains("This will not be a short flight"), "{}", said(&out));
        let out = s.boat_script_answer(net::script::SCRIPT_ACTION_NO, departs - 199).unwrap();
        assert!(said(&out).contains("You must have some business"), "{}", said(&out));
        assert_eq!((map_of(s), s.held_count(id, boat::BASIC_TICKET)), (boat::STATION, 1), "No: nobody moved, nothing taken");

        // ...and Yes.
        let _ = s.boarder_choice(O, Ticket::Basic, departs - 200);
        let _ = s.boat_script_answer(net::script::SCRIPT_ACTION_YES, departs - 199).unwrap();
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
        assert_eq!((map_of(s), s.held_count(id, boat::REGULAR_TICKET)), (boat::WAITING_ROOM, 0), "Yes: aboard - its own waiting room first");
    }

    /// **The station ship.** Arriving on the station between `:x5` and the departure gets the
    /// arrive animation, any other time the leave one; the ship coming in and leaving is
    /// announced to everyone standing there once, by one session. The waiting room is the
    /// control: nobody there is told anything about the station's ship.
    #[test]
    fn the_station_ship_arrives_at_x5_and_leaves_at_departure_for_everyone_there() {
        let (_, _, mut ss) = station(&["Watcher", "Ticker", "Waiter"], 10_000);
        let d = boat::next_departure(Store::unix_now()) + boat::DEPARTURE_EVERY_S;
        let ship = |out: &[Reply], op: u16| -> Vec<Vec<u8>> { out.iter().filter(|r| r.opcode == op).map(|r| r.body.clone()).collect() };

        // Entry: in at :x5:00 and in the closing minute; out a second before :x5 and at :x0.
        for (at, docked) in [(d - 300, true), (d - 30, true), (d - 301, false), (d, false)] {
            let out = ss[0].0.boat_field_entry_at(at);
            assert_eq!(ship(&out, net::ship::CONTI_STATE), vec![net::ship::station_state(docked)], "entering at d{:+}", at - d);
        }

        // Waiter is in the waiting room: the control.
        let _ = ss[2].0.buy_ticket(O, Ticket::Basic);
        let _ = ss[2].0.board_ship(O, Ticket::Basic, d - 200);
        let _ = ss[2].0.on_field_entered();
        assert_eq!(map_of(&ss[2].0), boat::WAITING_ROOM);
        assert!(ship(&ss[2].0.boat_field_entry_at(d - 200), net::ship::CONTI_STATE).is_empty(), "no station ship in the waiting room");

        // The ship comes in: one session's tick tells both people on the station, once.
        let _ = ss[1].0.boat_tick_at(d - 299);
        let _ = ss[0].0.boat_tick_at(d - 299); // a second ticker: nothing more
        let heard = |s: &mut Session| s.tick(1_000).into_iter().filter(|r| r.opcode == net::ship::CONTI_MOVE).map(|r| r.body).collect::<Vec<_>>();
        assert_eq!(heard(&mut ss[0].0), vec![net::ship::ship_arrives()], "the watcher sees it come in, once");
        assert_eq!(heard(&mut ss[1].0), vec![net::ship::ship_arrives()], "and so does the ticker, through its own mailbox");
        assert!(heard(&mut ss[2].0).is_empty(), "not the waiting room");

        // And leaves.
        let _ = ss[0].0.boat_tick_at(d + 1);
        assert_eq!(heard(&mut ss[1].0), vec![net::ship::ship_leaves()]);
        assert_eq!(heard(&mut ss[0].0), vec![net::ship::ship_leaves()]);
    }

    /// **The Crimson Balrog invasion**, a minute into an invaded crossing: one passenger on the
    /// deck brings the Balrog's ship and two Balrogs at its position, both deck passengers see
    /// all three, the one controlling client gets both, and the cabin sees nothing. Someone
    /// coming up afterwards is shown the ship and the Balrogs on entry. A quiet crossing on
    /// the same tick is the control.
    #[test]
    fn an_invaded_crossing_gets_the_balrog_ship_and_two_crimson_balrogs_a_minute_in() {
        let (_, fields, mut ss) = station(&["Deckhand", "Lookout", "Stowaway", "Quietone"], 10_000);
        let d = boat::next_departure(Store::unix_now()) + boat::DEPARTURE_EVERY_S;
        let ids: Vec<u32> = ss.iter().map(|(_, id)| *id).collect();
        for id in &ids[..3] {
            let _ = fields.voyages().board_shared(*id, O.id, d);
        }
        let _ = fields.voyages().board_shared(ids[3], O.id, d + boat::DEPARTURE_EVERY_S);
        // Sail the first ship with an invading roll, the way the tick would, and put everyone
        // where the test wants them: two on the deck, one in the cabin.
        let sailed = fields.voyages().take_departures(d, || 0);
        assert_eq!(sailed.len(), 1);
        assert_eq!(sailed[0].invasion_at, Some(d + boat::INVASION_AFTER_S));
        for (n, map) in [(0, boat::DECK), (1, boat::DECK), (2, boat::CABIN)] {
            let mut chr = ss[n].0.claimed_character().unwrap();
            let _ = ss[n].0.go_to_map(&mut chr, map, 0, "aboard".to_string());
            let _ = ss[n].0.on_field_entered();
        }
        let deck = ss[0].0.field();
        assert_eq!(deck, FieldKey::instanced(boat::DECK, sailed[0].id));

        let ops = |out: &[Reply], op: u16| out.iter().filter(|r| r.opcode == op).count();
        // Not before the minute.
        let out = ss[0].0.boat_tick_at(d + boat::INVASION_AFTER_S - 1);
        assert_eq!(ops(&out, net::mob::MOB_ENTER_FIELD), 0, "not yet");

        // The minute: the ship, then two Balrogs, controlled here.
        let out = ss[0].0.boat_tick_at(d + boat::INVASION_AFTER_S);
        assert_eq!(
            out.iter().filter(|r| r.opcode == net::ship::CONTI_MOVE).map(|r| r.body.clone()).collect::<Vec<_>>(),
            vec![net::ship::enemy_ship_arrives()]
        );
        assert_eq!(ops(&out, net::mob::MOB_ENTER_FIELD), 2, "two Crimson Balrogs");
        assert_eq!(ops(&out, net::mobmove::MOB_CHANGE_CONTROLLER), 2, "and this client runs both");
        let mobs = fields.mobs_on(deck);
        assert_eq!(mobs.len(), 2);
        let mut at: Vec<(i16, i16)> = mobs.iter().map(|m| (m.spawn.x, m.spawn.y)).collect();
        at.sort();
        assert_eq!(at, O.invaders_at().to_vec(), "at the Balrog's ship, in the air");
        assert!(mobs.iter().all(|m| m.spawn.template_id == boat::CRIMSON_BALROG && m.spawn.fh == 0));
        assert!(ss[1].0.boat_tick_at(d + boat::INVASION_AFTER_S + 1).iter().all(|r| r.opcode != net::mob::MOB_ENTER_FIELD), "once");

        // The other deck passenger sees all three; the cabin sees nothing.
        let lookout = ss[1].0.tick(1_000);
        assert_eq!(ops(&lookout, net::ship::CONTI_MOVE), 1);
        assert_eq!(ops(&lookout, net::mob::MOB_ENTER_FIELD), 2);
        let cabin = ss[2].0.tick(1_000);
        assert_eq!(ops(&cabin, net::ship::CONTI_MOVE) + ops(&cabin, net::mob::MOB_ENTER_FIELD), 0, "not below deck");
        // Quietone's crossing was not invaded: their ship has nothing to take.
        let quiet = fields.voyages().voyage_of(ids[3]).unwrap();
        assert!(!fields.voyages().take_invasion(quiet.id, d + 600 + boat::INVASION_AFTER_S));

        // Up from the cabin: the ship on entry, and the Balrogs with the field.
        let mut chr = ss[2].0.claimed_character().unwrap();
        let _ = ss[2].0.go_to_map(&mut chr, boat::DECK, 0, "up to the deck".to_string());
        let entry = ss[2].0.boat_field_entry_at(d + 120);
        assert_eq!(
            entry.iter().filter(|r| r.opcode == net::ship::CONTI_STATE).map(|r| r.body.clone()).collect::<Vec<_>>(),
            vec![net::ship::deck_invaded()]
        );
        let entry = ss[2].0.on_field_entered();
        assert_eq!(ops(&entry, net::mob::MOB_ENTER_FIELD), 2, "the Balrogs are on the field for a latecomer");
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
        let _ = s.board_ship(O, Ticket::Basic, departs - 100);
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
            let _ = fields.voyages().board_private(id, O.id, Store::unix_now());
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

    /// Agatha's two boxes: their introduction, then Next to the tickets to Ellinia.
    fn agatha_tickets(s: &mut Session) -> Vec<Reply> {
        let out = s.handle(&click(AGATHA_OBJECT));
        assert!(said(&out).contains("information guide for Orbis Station"), "their introduction first: {}", said(&out));
        s.handle(&yes())
    }

    /// **The way back, end to end.** Agatha sells a Ticket to Ellinia from their menu; the
    /// Platform Usher takes you to the tunnel after Isa's question (No leaves you in the booth
    /// with their line); Rini boards you inside the window; Erin's No keeps you; the departure
    /// sails to To Ellinia and the arrival lands in Ellinia Station, where Joel is. The Orbis
    /// ship's voyages are never the Ellinia ship's, even for the same minute.
    #[test]
    fn the_ship_back_to_ellinia_runs_from_agatha_to_ellinia_station() {
        let (store, fields, mut ss) = station_at(boat::ORBIS, &["Homebound", "Outbound"], 26_000);
        let (home, out_id) = (ss[0].1, ss[1].1);

        // Agatha: introduction, Next, the Ellinia tickets, a sale.
        let out = agatha_tickets(&mut ss[0].0);
        assert!(said(&out).contains("#L0#Ticket to Ellinia (Basic)"), "{}", said(&out));
        let out = ss[0].0.handle(&pick(Ticket::Basic.line()));
        assert!(said(&out).contains("#bPlatform Usher#k"), "{}", said(&out));
        assert_eq!(ss[0].0.held_count(home, boat::ELLINIA_BASIC_TICKET), 1);
        assert_eq!(ss[0].0.held_count(home, boat::BASIC_TICKET), 0, "a ticket to Ellinia, not to Orbis");
        assert_eq!(store.mesos(home).unwrap(), 21_000);

        // The Platform Usher: No first, then Yes to the tunnel.
        let out = ss[0].0.handle(&click(USHER_OBJECT));
        assert!(said(&out).contains("Platform to Board a Ship to Victoria Island"), "{}", said(&out));
        let out = ss[0].0.handle(&pick(boat::USHER_TO_VICTORIA));
        assert!(said(&out).contains("Even if you've entered a wrong Tunnel"), "{}", said(&out));
        let out = ss[0].0.handle(&no());
        assert!(said(&out).contains("The ride is on schedule"), "{}", said(&out));
        assert_eq!(map_of(&ss[0].0), boat::ORBIS, "No: still in the booth");
        let _ = ss[0].0.handle(&click(USHER_OBJECT));
        let _ = ss[0].0.handle(&pick(boat::USHER_TO_VICTORIA));
        let _ = ss[0].0.handle(&yes());
        assert_eq!(map_of(&ss[0].0), boat::ORBIS_TUNNEL, "Yes: the tunnel to the platform");

        // The tunnel walks to Rini's platform (a portal walk, done here directly).
        let mut chr = ss[0].0.claimed_character().unwrap();
        let _ = ss[0].0.go_to_map(&mut chr, boat::ORBIS_STATION, 0, "through the tunnel".to_string());
        let _ = ss[0].0.on_field_entered();
        let out = ss[0].0.handle(&click(RINI_OBJECT));
        assert!(said(&out).contains("If you want to get on the ride to Victoria Island"), "their own line: {}", said(&out));
        let d = boat::next_departure(Store::unix_now()) + boat::DEPARTURE_EVERY_S;
        let out = ss[0].0.boarder_choice(E, Ticket::Basic, d - 400);
        assert!(said(&out).contains("We will begin boarding"), "{}", said(&out));
        let out = ss[0].0.boarder_choice(E, Ticket::Basic, d - 200);
        assert!(said(&out).contains("This will not be a short flight"), "{}", said(&out));
        let _ = ss[0].0.boat_script_answer(net::script::SCRIPT_ACTION_YES, d - 199).unwrap();
        assert_eq!(map_of(&ss[0].0), boat::ORBIS_WAITING_ROOM);
        let _ = ss[0].0.on_field_entered();

        // A passenger to Orbis on the same departure is on another ship.
        let _ = fields.voyages().board_shared(out_id, O.id, d);
        // One registry read per statement: two guards in one expression deadlock.
        let outbound = fields.voyages().voyage_of(out_id).unwrap().id;
        let homebound = fields.voyages().voyage_of(home).unwrap().id;
        assert_ne!(outbound, homebound);

        // Erin: No keeps you aboard.
        let out = ss[0].0.handle(&click(ERIN_OBJECT));
        assert!(said(&out).contains("Are you sure you want to get off the ship?"), "{}", said(&out));
        let out = ss[0].0.handle(&no());
        assert!(said(&out).contains("You'll get to your destination"), "{}", said(&out));
        assert_eq!(map_of(&ss[0].0), boat::ORBIS_WAITING_ROOM);

        // The departure: to the To Ellinia deck, with the countdown.
        let _ = ss[0].0.boat_tick_at(d);
        assert_eq!(map_of(&ss[0].0), boat::ELLINIA_DECK);
        assert_eq!(ss[0].0.field(), FieldKey::instanced(boat::ELLINIA_DECK, homebound));
        let entry = ss[0].0.on_field_entered();
        assert!(clock_of(&entry).is_some(), "the countdown on the deck");
        // The cabin keeps the voyage.
        let mut chr = ss[0].0.claimed_character().unwrap();
        let _ = ss[0].0.go_to_map(&mut chr, boat::ELLINIA_CABIN, 2, "below".to_string());
        assert!(ss[0].0.field().is_instanced());

        // The arrival: Ellinia Station, with the notice.
        let out = ss[0].0.boat_tick_at(d + boat::RIDE_S);
        assert!(out.iter().any(|r| String::from_utf8_lossy(&r.body).contains("The ship has arrived at Ellinia Station.")));
        assert_eq!(map_of(&ss[0].0), boat::STATION);
        assert_eq!(fields.voyages().voyage_of(home), None);
    }

    /// **Erin's Yes** puts you back on Rini's platform - the departure station, not the booth -
    /// and a login saved on any To Ellinia ship field lands there too.
    #[test]
    fn getting_off_or_logging_in_on_the_way_back_lands_on_the_orbis_platform() {
        let (store, fields, mut ss) = station_at(boat::ORBIS_STATION, &["Erinsays"], 10_000);
        let (s, id) = &mut ss[0];
        let id = *id;
        let _ = s.give_item(boat::ELLINIA_BASIC_TICKET, 1, "test").unwrap();
        let d = boat::next_departure(Store::unix_now()) + boat::DEPARTURE_EVERY_S;
        let _ = s.board_ship(E, Ticket::Basic, d - 100);
        assert_eq!(map_of(s), boat::ORBIS_WAITING_ROOM);
        let _ = s.on_field_entered();
        let _ = s.handle(&click(ERIN_OBJECT));
        let _ = s.handle(&yes());
        assert_eq!(map_of(s), boat::ORBIS_STATION, "back to Rini's platform");
        assert_eq!(fields.voyages().voyage_of(id), None);

        for map in [boat::ORBIS_WAITING_ROOM, boat::ELLINIA_DECK, boat::ELLINIA_CABIN] {
            store.set_character_map(id, map).unwrap();
            let account = s.claimed.as_ref().unwrap().account_id;
            store.create_migration(account, id, 0, 0).unwrap();
            let mut again = Session::joining(store.clone(), s.config.clone(), fields.clone());
            again.claim_for_character(id);
            let out = again.handle(&crate::session::CLIENT_MIGRATION_HELLO.to_le_bytes());
            let set = out.iter().find(|r| r.opcode == net::opcode::SET_FIELD).expect("the login SetField");
            assert!(set.what.contains(&format!("carrying map {} ", boat::ORBIS_STATION)), "saved on {map}: {}", set.what);
        }
    }

    /// **The Platform Usher runs no ferry** (the owner, 2026-09-29). Their menu is the platform and
    /// nothing else; the old ferry line's number answers nothing, costs nothing and moves
    /// nobody.
    #[test]
    fn the_platform_usher_runs_no_ferry() {
        let (store, _, mut ss) = station_at(boat::ORBIS, &["Ferryrider"], 10_000);
        let (s, id) = &mut ss[0];
        let out = s.handle(&click(USHER_OBJECT));
        assert!(said(&out).contains("Platform to Board") && !said(&out).contains("ferry"), "{}", said(&out));
        let out = s.handle(&pick(1));
        assert!(out.is_empty(), "{out:?}");
        assert_ne!(s.conversation.as_ref().map(|c| c.path.clone()), Some(crate::taxi::MENU_PATH.to_string()), "no taxi menu");
        assert_eq!((map_of(s), store.mesos(*id).unwrap()), (boat::ORBIS, 10_000));
    }

    /// Both platforms show the station ship, on entry and when it moves; the booth does not.
    #[test]
    fn both_platforms_see_the_ship_come_and_go() {
        let (_, _, mut ss) = station_at(boat::ORBIS_STATION, &["Platformer", "Boothsitter"], 0);
        let mut chr = ss[1].0.claimed_character().unwrap();
        let _ = ss[1].0.go_to_map(&mut chr, boat::ORBIS, 0, "the booth".to_string());
        let _ = ss[1].0.on_field_entered();
        let d = boat::next_departure(Store::unix_now()) + boat::DEPARTURE_EVERY_S;
        let states = |out: &[Reply]| out.iter().filter(|r| r.opcode == net::ship::CONTI_STATE).map(|r| r.body.clone()).collect::<Vec<_>>();
        assert_eq!(states(&ss[0].0.boat_field_entry_at(d - 100)), vec![net::ship::station_state(true)]);
        assert!(states(&ss[1].0.boat_field_entry_at(d - 100)).is_empty(), "the booth has no ship");
        let _ = ss[1].0.boat_tick_at(d - 299);
        let heard = |s: &mut Session| s.tick(1_000).into_iter().filter(|r| r.opcode == net::ship::CONTI_MOVE).count();
        assert_eq!(heard(&mut ss[0].0), 1, "the Orbis platform sees it come in");
        assert_eq!(heard(&mut ss[1].0), 0, "the booth does not");
    }
}
