//! **The jump quests' NPCs** - the doors, the wardens, and what waits at the top.
//! `crate::jumpquest` has the tables and the owner's calls, and
//! `research/jump-quests-2026-10-04.md` has the evidence.
//!
//! * **Shane / the Mysterious Statue**: a menu of the courses, then a teleport to the chosen
//!   course's first step. Shane refuses a stranger to Sabitrama's errand by falling through
//!   to his own `d0`.
//! * **Jake**: a menu of the three tickets. The level is checked, then the room, then the
//!   price, and only then is the ticket handed over.
//! * **The Ticket Gate**: a menu of the tickets the player holds. One is taken, then they go
//!   to that floor's Area 1.
//! * **Louis / the Crumbling Statue / the Exit**: a yes/no, then the town's landing spot.
//! * **A goal** (a pile, a chest): first a reach check, then the roll and a room check. A box
//!   then says what was found, and **dismissing it** hands everything over and warps the
//!   player out.
//!
//! The grant waits for the dismissal so that a disconnect with the box open loses nothing and
//! gains nothing. The player is still at the top, and their next click rolls again.
//!
//! Nothing here authenticates: whoever holds the socket is the player.

use super::{Conversation, Reply, Session};
use crate::jumpquest as jq;

impl Session {
    fn jq_park(&mut self, template: u32, path: String, yes_no: bool) {
        self.conversation = Some(Conversation {
            npc_template: template,
            quest_id: None,
            path,
            sent: 0,
            awaiting_yes_no: yes_no,
            sent_with_next: false,
        });
    }

    /// A plain box whose OK closes it silently (`questroom::REFUSAL_PATH`).
    fn jq_say(&mut self, template: u32, text: &str, why: String) -> Vec<Reply> {
        self.jq_park(template, crate::questroom::REFUSAL_PATH.to_string(), false);
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(template, text, false, false),
            what: format!("ScriptMessage Say from NPC {template} (jump quest): {why}"),
        }]
    }

    fn jq_menu(&mut self, template: u32, path: &str, text: &str, why: &str) -> Vec<Reply> {
        self.jq_park(template, path.to_string(), false);
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_menu(template, text),
            what: format!("ScriptMessage MENU from NPC {template} (jump quest): {why}"),
        }]
    }

    fn jq_quest_in_progress(&self, character_id: u32, quest: u32) -> bool {
        matches!(self.store.quest_row(character_id, quest), Ok(Some(r)) if r.state == store::QuestState::InProgress)
    }

    /// Onto a named portal of `map` - a way out's landing spot - or a random spawn point when
    /// this `portals.txt` does not name it.
    fn jq_land(&mut self, chr: &mut net::opcode::Character, (map, portal): (u32, &str), why: String) -> Vec<Reply> {
        match self.config.portal_index.get(&(map, portal.to_string())).copied() {
            Some(idx) => self.go_to_map(chr, map, idx, format!("{why} (portal {portal})")),
            None => self.teleport(chr, map, why),
        }
    }

    /// **A click on a jump-quest NPC.** `None` for any other NPC, or for one of these standing
    /// somewhere it does not belong, so the ordinary click path answers it.
    pub(super) fn open_jump_quest_npc(&mut self, template: u32) -> Option<Vec<Reply>> {
        let chr = self.claimed_character()?;
        match template {
            jq::SHANE if chr.map_id == jq::ELLINIA => self.open_shane(&chr),
            jq::MYSTERIOUS_STATUE if chr.map_id == jq::SLEEPYWOOD => Some(self.open_statue()),
            jq::JAKE if chr.map_id == jq::TICKET_BOOTH => Some(self.open_jake()),
            jq::TICKET_GATE if chr.map_id == jq::TICKET_BOOTH => Some(self.open_ticket_gate(&chr)),
            _ => {
                let area = jq::area_of(chr.map_id)?;
                if template == area.warden() {
                    self.jq_park(template, jq::LEAVE_PATH.to_string(), true);
                    return Some(vec![Reply {
                        opcode: net::script::SCRIPT_MESSAGE,
                        body: net::script::npc_ask(template, area.leave_question(), false),
                        what: format!("ScriptMessage YES/NO from NPC {template}: leave the jump quest from map {}", chr.map_id),
                    }]);
                }
                let goal = jq::goal_for(template).filter(|g| g.map == chr.map_id)?;
                Some(self.open_goal(&chr, goal))
            }
        }
    }

    /// Shane: the menu for anyone who has taken Sabitrama's first errand. `None` - his own
    /// `d0` - for a stranger.
    fn open_shane(&mut self, chr: &net::opcode::Character) -> Option<Vec<Reply>> {
        if !matches!(self.store.quest_row(chr.id, jq::SHANE_KEY_QUEST), Ok(Some(_))) {
            crate::server::log(&format!(
                "   jump quest: Shane refuses {} ({}) - quest {} never taken, so his own line",
                chr.name,
                chr.id,
                jq::SHANE_KEY_QUEST
            ));
            return None;
        }
        let mut text = "Are you here at Sabitrama's request? Then go on in - I won't charge you. Which herb are you \
                        looking for?"
            .to_string();
        for c in jq::FOREST_COURSES {
            text.push_str(&format!(r"\n#L{}##b#t{}##k ({})#l", c.line, c.item, c.steps));
        }
        Some(self.jq_menu(jq::SHANE, jq::SHANE_PATH, &text, "which Forest of Patience course"))
    }

    fn open_statue(&mut self) -> Vec<Reply> {
        let mut text = "(A strange statue. It's hard to tell whether it's laughing or crying.) Laying a hand on it, \
                        I feel I could be pulled somewhere far away. Which flower am I looking for?"
            .to_string();
        for c in jq::DEEP_FOREST_COURSES {
            text.push_str(&format!(r"\n#L{}##b#t{}##k ({})#l", c.line, c.item, c.steps));
        }
        self.jq_menu(jq::MYSTERIOUS_STATUE, jq::STATUE_PATH, &text, "which Deep Forest of Patience course")
    }

    fn open_jake(&mut self) -> Vec<Reply> {
        let mut text = "Monsters that like the dark often hide inside the subway, so please be careful if you're \
                        thinking of going in! No one's allowed in without a ticket. Which one would you like?"
            .to_string();
        for t in jq::TICKETS {
            text.push_str(&format!(r"\n#L{}##b#t{}##k - Lv. {}+, {} mesos#l", t.line, t.item, t.min_level, t.price));
        }
        self.jq_menu(jq::JAKE, jq::JAKE_PATH, &text, "which Construction Site ticket")
    }

    fn open_ticket_gate(&mut self, chr: &net::opcode::Character) -> Vec<Reply> {
        let held: Vec<jq::Ticket> = jq::TICKETS.into_iter().filter(|t| self.held_count(chr.id, t.item) > 0).collect();
        if held.is_empty() {
            return self.jq_say(
                jq::TICKET_GATE,
                "You need a ticket to go through the gate. #bJake#k, right beside it, sells them.",
                "no Construction Site ticket held".to_string(),
            );
        }
        let mut text = "Which ticket will you use?".to_string();
        for t in held {
            text.push_str(&format!(r"\n#L{}##b#t{}##k#l", t.line, t.item));
        }
        self.jq_menu(jq::TICKET_GATE, jq::GATE_PATH, &text, "which held ticket to use")
    }

    /// **The top of a course.** Close enough, a roll, room for all of it - then the box. The
    /// grant itself waits for the box to be dismissed (`jump_quest_script_answer`).
    fn open_goal(&mut self, chr: &net::opcode::Character, goal: &'static jq::Goal) -> Vec<Reply> {
        let at = self
            .config
            .npcs
            .get(&chr.map_id)
            .and_then(|list| list.iter().find(|n| n.template_id == goal.npc))
            .map(|n| (n.x, n.cy));
        if let Some(npc) = at {
            if !self.last_position.is_some_and(|p| jq::within_reach(p, npc)) {
                crate::server::log(&format!(
                    "   jump quest: {} ({}) clicked NPC {} at {npc:?} from {:?} - out of reach, nothing given",
                    chr.name, chr.id, goal.npc, self.last_position
                ));
                return self.jq_say(
                    goal.npc,
                    &format!("You can't see inside the {} very well from here. Go a little closer.", goal.noun),
                    format!("out of reach from {:?}", self.last_position),
                );
            }
        }
        let owed = jq::quest_item_owed(self.jq_quest_in_progress(chr.id, goal.quest), self.held_count(chr.id, goal.item), goal.count);
        let prizes: Vec<(&str, crate::magicbox::Prize)> = if goal.prize {
            jq::SLOTS.iter().map(|s| (s.name, crate::magicbox::roll(s, self.rng.next()))).collect()
        } else {
            Vec::new()
        };
        if let Some(refusal) = self.jq_room_refusal(chr, goal, owed, &prizes.iter().map(|&(_, p)| p).collect::<Vec<_>>()) {
            return refusal;
        }
        let landing = jq::goal_landing(goal);
        let text = jq::found_text(goal, (owed > 0).then_some((goal.item, owed)), &prizes, jq::town_name(landing));
        let path = jq::found_path(goal.npc, &prizes.iter().map(|&(_, p)| p).collect::<Vec<_>>());
        crate::server::log(&format!(
            "   jump quest: {} ({}) reached NPC {} on map {} - quest {} item {} x{owed}, prizes {:?}; handed over when the box closes",
            chr.name, chr.id, goal.npc, chr.map_id, goal.quest, goal.item, prizes
        ));
        self.jq_park(goal.npc, path, false);
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(goal.npc, &text, false, false),
            what: format!("ScriptMessage Say from NPC {}: what the {} held", goal.npc, goal.noun),
        }]
    }

    /// The bag-full box when the quest item and the prizes do not all fit; `None` when they do.
    fn jq_room_refusal(
        &mut self,
        chr: &net::opcode::Character,
        goal: &jq::Goal,
        owed: u16,
        prizes: &[crate::magicbox::Prize],
    ) -> Option<Vec<Reply>> {
        let mut gives: Vec<(u32, u16, store::InventoryType)> = Vec::new();
        let quest_item = (owed > 0).then_some((goal.item, owed));
        for &(id, q) in quest_item.iter().chain(prizes) {
            if let Some(tab) = self.config.tab_for(id).or_else(|| store::InventoryType::for_item(id)) {
                gives.push((id, q, tab));
            }
        }
        let short = match self.store.bag(chr.id) {
            Ok(bag) => crate::questroom::shortfall(&bag, &gives, &[], |id| self.config.shops.max_stack(id)),
            Err(_) => Vec::new(),
        };
        if short.is_empty() {
            return None;
        }
        crate::server::log(&format!(
            "   jump quest: {} ({}) at NPC {} has no room for {gives:?}: {short:?}; nothing given, they stay",
            chr.name, chr.id, goal.npc
        ));
        let text = format!("{} Then come back and look again.", crate::questroom::refusal_text(&short));
        Some(self.jq_say(goal.npc, &text, "the bag is full".to_string()))
    }

    /// **The box at the top was dismissed** - OK or Close alike: hand over what it promised and
    /// what the quest is still owed, then warp out. A bag that filled in between refuses it all
    /// and leaves the player there.
    fn claim_goal(&mut self, npc: u32, prizes: Vec<crate::magicbox::Prize>) -> Vec<Reply> {
        let Some(goal) = jq::goal_for(npc) else { return Vec::new() };
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        if chr.map_id != goal.map {
            return Vec::new();
        }
        let owed = jq::quest_item_owed(self.jq_quest_in_progress(chr.id, goal.quest), self.held_count(chr.id, goal.item), goal.count);
        if let Some(refusal) = self.jq_room_refusal(&chr, goal, owed, &prizes) {
            return refusal;
        }
        let mut out = Vec::new();
        let mut lines = Vec::new();
        let mut gained = Vec::new();
        let quest_item = (owed > 0).then_some((goal.item, owed));
        for (i, &(id, q)) in quest_item.iter().chain(&prizes).enumerate() {
            match self.give_item(id, q, "jump quest") {
                Ok((line, replies)) => {
                    out.extend(replies);
                    out.push(self.item_chat_line(id, i64::from(q)));
                    lines.push(line);
                    if quest_item.is_none() || i > 0 {
                        gained.push((id, q));
                    }
                }
                Err(why) => lines.push(format!("{q} x {id} NOT given: {why}")),
            }
        }
        if !gained.is_empty() {
            crate::killstats::note_jump_quest(chr.id, &gained);
        }
        crate::server::log(&format!(
            "   jump quest: {} ({}) took what NPC {} held - {}",
            chr.name,
            chr.id,
            goal.npc,
            if lines.is_empty() { "nothing".to_string() } else { lines.join("; ") }
        ));
        out.extend(self.jq_land(&mut chr, jq::goal_landing(goal), format!("jump quest: done at NPC {}", goal.npc)));
        out
    }

    /// **A jump-quest menu came back.** `None` when none of theirs is parked.
    pub(super) fn jump_quest_menu_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        if ![jq::SHANE_PATH, jq::STATUE_PATH, jq::JAKE_PATH, jq::GATE_PATH].contains(&convo.path.as_str()) {
            return None;
        }
        let reply = net::script::parse_menu_reply(body)?;
        self.conversation = None;
        let Some(line) = reply.selection else { return Some(Vec::new()) };
        let mut chr = self.claimed_character()?;
        let out = match convo.path.as_str() {
            jq::SHANE_PATH if chr.map_id == jq::ELLINIA => match jq::FOREST_COURSES.iter().find(|c| c.line == line) {
                Some(c) => self.teleport(&mut chr, c.start_map, format!("Shane: the Forest of Patience, {}", c.steps)),
                None => Vec::new(),
            },
            jq::STATUE_PATH if chr.map_id == jq::SLEEPYWOOD => match jq::DEEP_FOREST_COURSES.iter().find(|c| c.line == line) {
                Some(c) => self.teleport(&mut chr, c.start_map, format!("the Mysterious Statue: the Deep Forest, {}", c.steps)),
                None => Vec::new(),
            },
            jq::JAKE_PATH if chr.map_id == jq::TICKET_BOOTH => match jq::TICKETS.into_iter().find(|t| t.line == line) {
                Some(t) => self.sell_ticket(&chr, t),
                None => Vec::new(),
            },
            jq::GATE_PATH if chr.map_id == jq::TICKET_BOOTH => match jq::TICKETS.into_iter().find(|t| t.line == line) {
                Some(t) => self.use_ticket(&mut chr, t),
                None => Vec::new(),
            },
            _ => Vec::new(),
        };
        Some(out)
    }

    /// Jake sells one ticket: the level, the room, then the price, then the ticket. A ticket the
    /// store will not take gives the price back.
    fn sell_ticket(&mut self, chr: &net::opcode::Character, t: jq::Ticket) -> Vec<Reply> {
        if chr.level < u32::from(t.min_level) {
            return self.jq_say(
                jq::JAKE,
                &format!("The #b{}#k is dangerous. I can only sell its ticket to someone at level {} or higher.", t.floor, t.min_level),
                format!("level {} is below {} for the {}", chr.level, t.min_level, t.floor),
            );
        }
        let tab = store::InventoryType::Etc;
        let short = match self.store.bag(chr.id) {
            Ok(bag) => crate::questroom::shortfall(&bag, &[(t.item, 1, tab)], &[], |id| self.config.shops.max_stack(id)),
            Err(_) => Vec::new(),
        };
        if !short.is_empty() {
            return self.jq_say(jq::JAKE, &crate::questroom::refusal_text(&short), "no room for the ticket".to_string());
        }
        match self.store.add_mesos(chr.id, -i64::from(t.price)) {
            Ok(balance) => match self.give_item(t.item, 1, "Jake's ticket") {
                Ok((line, replies)) => {
                    crate::server::log(&format!(
                        "   jump quest: {} ({}) bought the {} ticket for {} - {balance} mesos left; {line}",
                        chr.name, chr.id, t.floor, t.price
                    ));
                    let mut out = self.meso_reply(chr.id);
                    out.push(Reply {
                        opcode: net::message::MESSAGE,
                        body: net::message::meso_lost_line(t.price),
                        what: format!("Message: grey chat line, {} mesos for the {} ticket", t.price, t.floor),
                    });
                    out.extend(replies);
                    out.push(self.item_chat_line(t.item, 1));
                    out
                }
                Err(why) => {
                    let _ = self.store.add_mesos(chr.id, i64::from(t.price));
                    crate::server::log(&format!("   jump quest: the {} ticket could not be given to {} ({why}); price returned", t.floor, chr.id));
                    self.jq_say(jq::JAKE, "I can't hand you the ticket just now. Your mesos are back in your pocket.", why)
                }
            },
            Err(store::StoreError::NotEnoughMesos { have, .. }) => self.jq_say(
                jq::JAKE,
                &format!("The ticket to the #b{}#k is #b{} mesos#k, and you don't have enough.", t.floor, t.price),
                format!("{have} mesos is short of {}", t.price),
            ),
            Err(e) => self.jq_say(jq::JAKE, "I can't take your payment just now. Please try again in a moment.", format!("the charge failed - {e}")),
        }
    }

    /// The gate takes one ticket and opens onto its floor. Held is checked again here - the
    /// menu was built from the bag as it was.
    fn use_ticket(&mut self, chr: &mut net::opcode::Character, t: jq::Ticket) -> Vec<Reply> {
        if self.held_count(chr.id, t.item) == 0 {
            return self.jq_say(jq::TICKET_GATE, "You need a ticket to go through the gate.", format!("no {} ticket held any more", t.floor));
        }
        let mut out = self.take_items(chr.id, store::InventoryType::Etc, t.item, 1);
        out.push(self.item_chat_line(t.item, -1));
        out.extend(self.teleport(chr, t.area_one, format!("the Ticket Gate: the {}", t.floor)));
        out
    }

    /// **A jump-quest yes/no or found box was answered.** Called before the generic code drops a
    /// closed box, because a found box hands over on Close as well as OK. `None` when the
    /// conversation is not one of ours.
    pub(super) fn jump_quest_script_answer(&mut self, path: &str, action: i8) -> Option<Vec<Reply>> {
        if let Some((npc, prizes)) = jq::parse_found_path(path) {
            self.conversation = None;
            return Some(self.claim_goal(npc, prizes));
        }
        if path != jq::LEAVE_PATH {
            return None;
        }
        self.conversation = None;
        if action != net::script::SCRIPT_ACTION_YES {
            return Some(Vec::new());
        }
        let mut chr = self.claimed_character()?;
        let area = jq::area_of(chr.map_id)?;
        Some(self.jq_land(&mut chr, area.landing(), format!("jump quest: {} shows them out", area.warden())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::Fields;
    use std::sync::Arc;
    use store::Store;

    const OBJ: u32 = 970;

    /// A character on `map` with `npc` beside them, the courses' maps loaded, the three
    /// landings named, and the items handed out known to the config.
    fn standing(map: u32, npc: u32, at: (i16, i16), level: u32) -> (Arc<Store>, Session, u32) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let mut cfg = crate::config::Config::default();
        for m in (10_002_040..=10_002_044).chain(10_005_040..=10_005_046).chain(10_003_100..=10_003_109) {
            cfg.fields.insert(m);
        }
        for m in [jq::ELLINIA, jq::SLEEPYWOOD, jq::TICKET_BOOTH] {
            cfg.fields.insert(m);
        }
        for (i, (m, name)) in [jq::ELLINIA_LANDING, jq::SLEEPYWOOD_LANDING, jq::BOOTH_LANDING].into_iter().enumerate() {
            cfg.portal_index.insert((m, name.to_string()), 30 + i as u8);
        }
        let mut items: Vec<u32> = jq::GOALS.iter().map(|g| g.item).chain(jq::TICKETS.map(|t| t.item)).collect();
        for s in &jq::SLOTS {
            items.extend(s.prizes.iter().map(|p| p.0));
        }
        for id in items {
            cfg.item_names.insert(id, format!("item {id}"));
        }
        cfg.npcs.insert(
            map,
            vec![net::opcode::FieldNpc { object_id: OBJ, template_id: npc, x: at.0, cy: at.1, fh: 1, rx0: 0, rx1: 0, f: 0 }],
        );
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Jumper".to_string(), map_id: map, level, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.set_character_map(id, map).unwrap();
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store.clone(), Arc::new(cfg), Arc::new(Fields::new()));
        s.claim_for_character(id);
        s.last_position = Some(at);
        (store, s, id)
    }

    fn click() -> Vec<u8> {
        let mut b = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
        b.extend_from_slice(&OBJ.to_le_bytes());
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

    /// The answer to a box: `u32 handle, u8 type`, then - for a Say only - `u32 echo, str`, then
    /// `i8 action` (`net::script::parse_script_reply`).
    fn answer(kind: u8, action: i8) -> Vec<u8> {
        let mut b = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
        b.extend_from_slice(&0u32.to_le_bytes());
        b.push(kind);
        if kind == net::script::SCRIPT_TYPE_SAY {
            b.extend_from_slice(&0u32.to_le_bytes());
            b.extend_from_slice(&0u16.to_le_bytes());
        }
        b.push(action as u8);
        b
    }

    fn said(out: &[Reply]) -> String {
        out.iter().map(|r| String::from_utf8_lossy(&r.body).to_string()).collect()
    }

    fn map_of(s: &Session) -> u32 {
        s.claimed_character().unwrap().map_id
    }

    fn held(store: &Store, id: u32, item: u32) -> u32 {
        let tab = store::InventoryType::for_item(item).unwrap();
        store.bag_items(id, tab).unwrap().iter().filter(|r| r.item.item_id == item).map(|r| u32::from(r.item.kind.quantity())).sum()
    }

    /// **Shane**: a stranger hears his own refusal and stays; anyone who has taken Sabitrama's
    /// errand picks a course, free, and lands on its first step.
    #[test]
    fn shane_lets_sabitramas_helpers_choose_a_course_for_free() {
        let (store, mut s, id) = standing(jq::ELLINIA, jq::SHANE, (168, -2879), 30);
        store.set_mesos(id, 1_000).unwrap();
        let out = s.handle(&click());
        assert!(!said(&out).contains("#L0#"), "a stranger gets no menu: {}", said(&out));
        assert!(s.conversation.as_ref().map_or(true, |c| !c.path.starts_with(jq::PATH_PREFIX)));

        store.start_quest(id, jq::SHANE_KEY_QUEST).unwrap();
        for c in jq::FOREST_COURSES {
            s.conversation = None;
            let out = s.handle(&click());
            let menu = said(&out);
            assert!(menu.contains("#L0#") && menu.contains("#L1#") && menu.contains(c.steps), "{menu}");
            let _ = s.handle(&pick(c.line));
            assert_eq!(map_of(&s), c.start_map, "{}", c.steps);
            assert_eq!(store.mesos(id).unwrap(), 1_000, "free");
            let mut chr = s.claimed_character().unwrap();
            let _ = s.teleport(&mut chr, jq::ELLINIA, "back for the next".to_string());
        }
    }

    /// **The Statue** offers the three courses to anyone and sends each to its own start.
    #[test]
    fn the_mysterious_statue_sends_each_flower_to_its_own_course() {
        for c in jq::DEEP_FOREST_COURSES {
            let (_, mut s, _) = standing(jq::SLEEPYWOOD, jq::MYSTERIOUS_STATUE, (1061, 255), 10);
            let out = s.handle(&click());
            assert!(said(&out).contains(&format!("#L{}#", c.line)));
            let _ = s.handle(&pick(c.line));
            assert_eq!(map_of(&s), c.start_map, "{}", c.steps);
        }
    }

    /// **The top of a pile** with the quest in progress: the box names the quest item and one
    /// prize per slot; nothing moves until it is dismissed; then all of it is in the bag and
    /// the player is in town. John's count is handed over whole, topped up from what is held.
    #[test]
    fn a_pile_gives_the_quest_item_and_a_prize_from_each_slot_when_its_box_closes() {
        let goal = jq::goal_for(jq::PINK_PILE).unwrap();
        let (store, mut s, id) = standing(goal.map, goal.npc, (762, -2332), 50);
        store.start_quest(id, goal.quest).unwrap();
        store.add_item(id, store::InventoryType::Etc, &store::Item::bundle(goal.item, 4), 100).unwrap();
        let out = s.handle(&click());
        let box_text = said(&out);
        assert!(box_text.contains(&format!("#t{}##k x6", goal.item)), "4 held, 6 owed: {box_text}");
        assert!(box_text.contains("#bUse#k") && box_text.contains("#bScroll#k"), "{box_text}");
        assert_eq!(held(&store, id, goal.item), 4, "nothing moves while the box is open");
        let path = s.conversation.as_ref().unwrap().path.clone();
        let (_, prizes) = jq::parse_found_path(&path).unwrap();
        assert_eq!(prizes.len(), 2);

        let out = s.handle(&answer(0, net::script::SCRIPT_ACTION_CLOSED));
        assert_eq!(held(&store, id, goal.item), 10, "Close hands over as OK does");
        for (item, q) in prizes {
            assert!(held(&store, id, item) >= u32::from(q), "{item}");
        }
        assert_eq!(map_of(&s), jq::SLEEPYWOOD);
        assert!(out.iter().any(|r| r.what.starts_with("SetField") && r.what.contains("(portal forest00)")), "the Statue's landing");
        assert!(out.iter().any(|r| r.opcode == net::stats::USER_EFFECT_LOCAL), "chat lines for what was given");
        assert!(s.conversation.is_none());
    }

    /// Without the quest a pile still gives its prizes and no quest item; so does a chest (the
    /// owner, 2026-10-04: *"the chest also gives jump quest rewards in addition to the quest
    /// item"*). Either way the box closes onto the way out.
    #[test]
    fn without_the_quest_a_pile_gives_prizes_only_and_a_chest_nothing() {
        let goal = jq::goal_for(jq::HERB_PILE).unwrap();
        let (store, mut s, id) = standing(goal.map, goal.npc, (146, -3626), 30);
        let out = s.handle(&click());
        assert!(!said(&out).contains(&format!("#t{}#", goal.item)), "{}", said(&out));
        let _ = s.handle(&answer(0, net::script::SCRIPT_ACTION_YES));
        assert_eq!(held(&store, id, goal.item), 0);
        assert_eq!(map_of(&s), jq::ELLINIA);

        let chest = jq::goal_for(jq::CHEST_B2).unwrap();
        let (store, mut s, id) = standing(chest.map, chest.npc, (107, 547), 40);
        let out = s.handle(&click());
        let box_text = said(&out);
        assert!(box_text.contains("You open the treasure chest") && box_text.contains("#bUse#k") && box_text.contains("#bScroll#k"), "{box_text}");
        assert!(!box_text.contains(&format!("#t{}#", chest.item)), "no quest, no roll of cash: {box_text}");
        let _ = s.handle(&answer(0, net::script::SCRIPT_ACTION_YES));
        assert_eq!(map_of(&s), jq::TICKET_BOOTH);
        assert_eq!(held(&store, id, chest.item), 0);

        let (store, mut s, id) = standing(chest.map, chest.npc, (107, 547), 40);
        store.start_quest(id, chest.quest).unwrap();
        let out = s.handle(&click());
        assert!(said(&out).contains(&format!("#t{}#", chest.item)) && said(&out).contains("#bScroll#k"), "{}", said(&out));
        let path = s.conversation.as_ref().unwrap().path.clone();
        let (_, prizes) = jq::parse_found_path(&path).unwrap();
        let _ = s.handle(&answer(0, net::script::SCRIPT_ACTION_YES));
        assert_eq!(held(&store, id, chest.item), 1, "Shumi's Roll of Cash");
        for (item, q) in prizes {
            assert!(held(&store, id, item) >= u32::from(q), "the reward beside it: {item}");
        }
    }

    /// **A click from the bottom of the step reaches nothing**, and gives nothing.
    #[test]
    fn a_goal_cannot_be_taken_from_the_bottom_of_the_step() {
        let goal = jq::goal_for(jq::WHITE_PILE).unwrap();
        let (store, mut s, id) = standing(goal.map, goal.npc, (1009, -3355), 50);
        store.start_quest(id, goal.quest).unwrap();
        s.last_position = Some((995, 230));
        let out = s.handle(&click());
        assert!(said(&out).contains("Go a little closer"), "{}", said(&out));
        let _ = s.handle(&answer(0, net::script::SCRIPT_ACTION_YES));
        assert_eq!((held(&store, id, goal.item), map_of(&s)), (0, goal.map), "nothing given, nobody moved");
    }

    /// **The wardens** ask first; No leaves the player where they are, Yes lands them by the
    /// door they came in through.
    #[test]
    fn the_wardens_show_a_player_out_on_yes_only() {
        for (map, warden, town, portal) in [
            (10_002_043, jq::LOUIS, jq::ELLINIA, "herb"),
            (10_005_045, jq::CRUMBLING_STATUE, jq::SLEEPYWOOD, "forest00"),
            (10_003_104, jq::EXIT, jq::TICKET_BOOTH, "out01"),
        ] {
            let (_, mut s, _) = standing(map, warden, (0, 0), 30);
            let _ = s.handle(&click());
            let _ = s.handle(&answer(net::script::SCRIPT_TYPE_YES_NO, net::script::SCRIPT_ACTION_NO));
            assert_eq!(map_of(&s), map, "No stays");
            let _ = s.handle(&click());
            let out = s.handle(&answer(net::script::SCRIPT_TYPE_YES_NO, net::script::SCRIPT_ACTION_YES));
            assert_eq!(map_of(&s), town, "{warden}");
            assert!(out.iter().any(|r| r.what.starts_with("SetField") && r.what.contains(&format!("(portal {portal})"))), "{warden}");
        }
    }

    /// **Jake and the gate**: too low a level is refused, a short purse is refused, a sale
    /// charges and hands the ticket over; the gate takes it and opens onto that floor.
    #[test]
    fn jake_sells_a_ticket_and_the_gate_takes_it() {
        let (store, mut s, id) = standing(jq::TICKET_BOOTH, jq::JAKE, (71, 187), 35);
        store.set_mesos(id, 1_500).unwrap();
        let _ = s.handle(&click());
        let out = s.handle(&pick(2));
        assert!(said(&out).contains("level 40"), "{}", said(&out));
        let _ = s.handle(&click());
        let _ = s.handle(&pick(1));
        assert_eq!((store.mesos(id).unwrap(), held(&store, id, 4_031_037)), (300, 1), "B2 sold for 1 200");
        let _ = s.handle(&click());
        let out = s.handle(&pick(0));
        assert!(said(&out).contains("don't have enough"), "{}", said(&out));
        assert_eq!((store.mesos(id).unwrap(), held(&store, id, 4_031_036)), (300, 0));

        let mut cfg = (*s.config).clone();
        cfg.npcs.insert(
            jq::TICKET_BOOTH,
            vec![net::opcode::FieldNpc { object_id: OBJ, template_id: jq::TICKET_GATE, x: 272, cy: 187, fh: 1, rx0: 0, rx1: 0, f: 0 }],
        );
        s.config = Arc::new(cfg);
        let out = s.handle(&click());
        let menu = said(&out);
        assert!(menu.contains("#L1#") && !menu.contains("#L0#"), "only the held ticket: {menu}");
        let _ = s.handle(&pick(1));
        assert_eq!((map_of(&s), held(&store, id, 4_031_037)), (10_003_103, 0), "B2 Area 1, ticket taken");
    }
}
