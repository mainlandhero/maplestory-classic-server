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
//! * **A goal** (a pile, a chest): first a reach check (within 250 px), then the roll and a
//!   room check. A box
//!   then says what was found, and **dismissing it** hands everything over and warps the
//!   player out.
//!
//! The grant waits for the dismissal so that a disconnect with the box open loses nothing and
//! gains nothing. The player is still at the top, and their next click rolls again.
//!
//! **The pity timer** (the owner, 2026-10-04). A door starts an hour per player, counted while
//! they are on that course (`store::jumpquest`). Past the hour, a yellow reminder comes every
//! five minutes, and `!skipjq` takes them out with the course's quest item and nothing else.
//! Leaving the course any other way ends the run.
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

    /// A door sent the player onto a course: a fresh hour - **only for a quest entry**. The
    /// owner, 2026-10-04: *"The timer will only be active for quest entries, it should not be
    /// active when the player is completing additional attempts for just the jump quest
    /// reward."* So the course's quest must be in progress and still want its item; any
    /// other entry starts no timer and clears a leftover one.
    fn jq_start_run(&mut self, character_id: u32, map: u32, why: &str) {
        let Some(goal) = jq::course_goal(map) else { return };
        let owed = jq::quest_item_owed(self.jq_quest_in_progress(character_id, goal.quest), self.held_count(character_id, goal.item), goal.count);
        if owed == 0 {
            let _ = self.store.end_jump_quest(character_id);
            crate::server::log(&format!(
                "   jump quest: character {character_id} enters the course to NPC {} ({why}) for the reward only - quest {} is not in progress or already has its item, so no pity timer",
                goal.npc, goal.quest
            ));
            self.jq_clock = jq::PityClock { next_check_ms: self.jq_clock.next_check_ms, ..jq::PityClock::default() };
            return;
        }
        match self.store.start_jump_quest(character_id, goal.npc) {
            Ok(()) => crate::server::log(&format!(
                "   jump quest: character {character_id} starts the course to NPC {} ({why}) - the pity timer is at 0 of {} s",
                goal.npc,
                jq::PITY_SECS
            )),
            Err(e) => crate::server::log(&format!("   jump quest: could not start the pity timer for {character_id}: {e}")),
        }
        self.jq_clock = jq::PityClock { last_ms: Some(self.clock_ms), ..jq::PityClock::default() };
    }

    /// **The pity timer on screen**, on every field entry on a course with a running hour. The
    /// owner, 2026-10-04: *"The timer is not visible, it should be a timer similar to the one in
    /// the First Time Together party quest."*
    ///
    /// The same widget as the party quest's: `0x01BC` type 2, the seconds left. It builds its
    /// own widget, so it is safe on these maps, which declare no `clock` node
    /// (`net::clock::clock_seconds`). It is sent on every entry - the door's warp, each step's
    /// portal, a log in or a channel change back onto the course, the way back from the Cash
    /// Shop - because a map change drops the widget. The client counts it down from there.
    ///
    /// Past the hour it shows zero, which is when `!skipjq` opens. A reward-only run has no
    /// row, so it gets no clock.
    pub(super) fn jump_quest_clock(&mut self) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let Some(goal) = jq::course_goal(chr.map_id) else { return Vec::new() };
        let run = match self.store.jump_quest_run(chr.id) {
            Ok(Some(r)) if r.goal_npc == goal.npc => r,
            _ => return Vec::new(),
        };
        let spent = run.spent_secs + self.jq_clock.pending_ms / 1_000;
        let left = u32::try_from(jq::PITY_SECS.saturating_sub(spent)).unwrap_or(0);
        vec![Reply {
            opcode: net::clock::FIELD_CLOCK,
            body: net::clock::clock_seconds(left),
            what: format!(
                "FieldClock type 2 to character {}: {left}s left of the jump quest's pity hour (course to NPC {})",
                chr.id, goal.npc
            ),
        }]
    }

    /// **What this connection counted and had not written yet**, written now - the connection
    /// is closing. The owner, 2026-10-04: *"The timer should also be kept should the player
    /// logout or otherwise disconnect"*. The row is kept either way; this saves the last few
    /// seconds of it.
    pub(super) fn flush_jump_quest_time_on_disconnect(&mut self) {
        let mut ms = self.jq_clock.pending_ms;
        if let Some(last) = self.jq_clock.last_ms {
            ms += self.clock_ms.saturating_sub(last);
        }
        self.jq_clock.pending_ms = 0;
        self.jq_clock.last_ms = None;
        if ms < 1_000 {
            return;
        }
        let Some(chr) = self.claimed_character() else { return };
        if jq::area_of(chr.map_id).is_some() {
            let _ = self.store.add_jump_quest_time(chr.id, ms / 1_000);
        }
    }

    /// The run is over: finished, shown out by a warden, or skipped - or, as a stale row, its
    /// player has been seen outside the course (`jump_quest_pity_tick`). Nothing else ends it.
    fn jq_end_run(&mut self, character_id: u32, why: &str) {
        if let Ok(true) = self.store.end_jump_quest(character_id) {
            crate::server::log(&format!("   jump quest: character {character_id}'s pity timer ends - {why}"));
        }
        self.jq_clock = jq::PityClock { next_check_ms: self.jq_clock.next_check_ms, ..jq::PityClock::default() };
    }

    /// **The pity timer's tick.** Once a second: time on a course is gathered and written every
    /// ten seconds, and a reminder goes out each time the total earns one. Time in the Cash Shop
    /// is not counted.
    ///
    /// **A row is removed here only when its player is outside its course** (the owner: *"does
    /// not destroy stale rows unless the player they are tracking are no longer within the jump
    /// quest area"*):
    /// * off every course - a return scroll, a death, a GM warp, or a log in that landed
    ///   elsewhere;
    /// * on another course than the run's, which only a GM warp reaches.
    ///
    /// Moving between the steps of the same course keeps it, because the run is named by the
    /// course's goal and every step shares that. A disconnect is not seen here at all: the row
    /// waits, and a log in back on the course carries on from it.
    pub(super) fn jump_quest_pity_tick(&mut self, now_ms: u64) -> Vec<Reply> {
        if now_ms < self.jq_clock.next_check_ms {
            return Vec::new();
        }
        self.jq_clock.next_check_ms = now_ms + jq::CHECK_MS;
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        if jq::area_of(chr.map_id).is_none() {
            if !self.jq_clock.cleared {
                self.jq_end_run(chr.id, &format!("off the course, on map {}", chr.map_id));
                self.jq_clock.cleared = true;
            }
            self.jq_clock.last_ms = None;
            return Vec::new();
        }
        if self.in_cash_shop {
            self.jq_clock.last_ms = None;
            return Vec::new();
        }
        self.jq_clock.cleared = false;
        let Some(last) = self.jq_clock.last_ms.replace(now_ms) else { return Vec::new() };
        self.jq_clock.pending_ms += now_ms.saturating_sub(last);
        if self.jq_clock.pending_ms < jq::FLUSH_MS {
            return Vec::new();
        }
        let secs = self.jq_clock.pending_ms / 1_000;
        self.jq_clock.pending_ms -= secs * 1_000;
        let Ok(Some(run)) = self.store.add_jump_quest_time(chr.id, secs) else { return Vec::new() };
        // A run for another course: they left that one (only a GM warp gets here).
        if jq::course_goal(chr.map_id).map(|g| g.npc) != Some(run.goal_npc) {
            self.jq_end_run(chr.id, &format!("on another course's map {}", chr.map_id));
            return Vec::new();
        }
        let due = jq::notices_due(run.spent_secs);
        if due <= run.notices {
            return Vec::new();
        }
        let _ = self.store.set_jump_quest_notices(chr.id, due);
        crate::server::log(&format!(
            "   jump quest: {} ({}) has spent {} s on the course to NPC {} - reminder {due}, !{} is open",
            chr.name,
            chr.id,
            run.spent_secs,
            run.goal_npc,
            jq::SKIP_COMMAND
        ));
        self.notice(jq::reminder_text(run.spent_secs))
    }

    /// **`!skipjq`.** Past the hour on the course they are on: the quest item that course's
    /// quest still wants (nothing else), out to the town, the run over. Before it, how long is
    /// left. Always answers.
    pub(super) fn skip_jump_quest(&mut self) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let Some(goal) = jq::course_goal(chr.map_id) else {
            return self.notice(format!("!{} only works on a jump quest.", jq::SKIP_COMMAND));
        };
        let run = match self.store.jump_quest_run(chr.id) {
            Ok(Some(r)) if r.goal_npc == goal.npc => r,
            _ => {
                return self.notice(
                    concat!(
                        "There is no jump quest timer running for you here. It starts when Shane, the Mysterious Statue or ",
                        "the Ticket Gate sends you in while you are on this course's quest."
                    )
                    .to_string(),
                )
            }
        };
        let spent = run.spent_secs + self.jq_clock.pending_ms / 1_000;
        if spent < jq::PITY_SECS {
            return self.notice(jq::not_yet_text(spent));
        }
        let owed = jq::quest_item_owed(self.jq_quest_in_progress(chr.id, goal.quest), self.held_count(chr.id, goal.item), goal.count);
        let mut out = Vec::new();
        if owed > 0 {
            let tab = self.config.tab_for(goal.item).or_else(|| store::InventoryType::for_item(goal.item)).unwrap_or(store::InventoryType::Etc);
            let short = match self.store.bag(chr.id) {
                Ok(bag) => crate::questroom::shortfall(&bag, &[(goal.item, owed, tab)], &[], |id| self.config.shops.max_stack(id)),
                Err(_) => Vec::new(),
            };
            if !short.is_empty() {
                return self.notice(format!("{} Then type !{} again.", crate::questroom::refusal_text(&short), jq::SKIP_COMMAND));
            }
            match self.give_item(goal.item, owed, "jump quest skipped") {
                Ok((_, replies)) => {
                    out.extend(replies);
                    out.push(self.item_chat_line(goal.item, i64::from(owed)));
                }
                Err(why) => {
                    crate::server::log(&format!("   jump quest: !skipjq could not give {owed} x {} to {}: {why}", goal.item, chr.id));
                    return self.notice(format!("The quest item could not be given ({why}). You are still on the jump quest."));
                }
            }
        }
        crate::server::log(&format!(
            "   jump quest: {} ({}) skips the course to NPC {} after {spent} s - quest item {} x{owed}, no prizes",
            chr.name, chr.id, goal.npc, goal.item
        ));
        self.jq_end_run(chr.id, "skipped with !skipjq");
        self.conversation = None;
        out.extend(self.jq_land(&mut chr, jq::goal_landing(goal), format!("jump quest: !{} from NPC {}'s course", jq::SKIP_COMMAND, goal.npc)));
        out.extend(self.notice(if owed > 0 {
            "You left the jump quest. You were given its quest item; the other rewards stay at the top.".to_string()
        } else {
            "You left the jump quest. The other rewards stay at the top.".to_string()
        }));
        out
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
        self.jq_end_run(chr.id, &format!("finished at NPC {}", goal.npc));
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
                Some(c) => {
                    let out = self.teleport(&mut chr, c.start_map, format!("Shane: the Forest of Patience, {}", c.steps));
                    self.jq_start_run(chr.id, c.start_map, "Shane");
                    out
                }
                None => Vec::new(),
            },
            jq::STATUE_PATH if chr.map_id == jq::SLEEPYWOOD => match jq::DEEP_FOREST_COURSES.iter().find(|c| c.line == line) {
                Some(c) => {
                    let out = self.teleport(&mut chr, c.start_map, format!("the Mysterious Statue: the Deep Forest, {}", c.steps));
                    self.jq_start_run(chr.id, c.start_map, "the Mysterious Statue");
                    out
                }
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
        self.jq_start_run(chr.id, t.area_one, "the Ticket Gate");
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
        self.jq_end_run(chr.id, &format!("NPC {} showed them out", area.warden()));
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

        // The owner's 250 px: 260 px along the same platform is still too far, 240 px is not.
        s.last_position = Some((1009 + 260, -3355));
        let out = s.handle(&click());
        assert!(said(&out).contains("Go a little closer"), "260 px: {}", said(&out));
        let _ = s.handle(&answer(0, net::script::SCRIPT_ACTION_YES));
        s.last_position = Some((1009 + 240, -3355));
        let out = s.handle(&click());
        assert!(said(&out).contains("You search the pile of white flowers"), "240 px: {}", said(&out));
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
            let (store, mut s, id) = standing(map, warden, (0, 0), 30);
            let goal = jq::course_goal(map).unwrap().npc;
            store.start_jump_quest(id, goal).unwrap();
            let _ = s.handle(&click());
            let _ = s.handle(&answer(net::script::SCRIPT_TYPE_YES_NO, net::script::SCRIPT_ACTION_NO));
            assert_eq!(map_of(&s), map, "No stays");
            assert!(store.jump_quest_run(id).unwrap().is_some(), "No keeps the hour running");
            let _ = s.handle(&click());
            let out = s.handle(&answer(net::script::SCRIPT_TYPE_YES_NO, net::script::SCRIPT_ACTION_YES));
            assert_eq!(map_of(&s), town, "{warden}");
            assert!(out.iter().any(|r| r.what.starts_with("SetField") && r.what.contains(&format!("(portal {portal})"))), "{warden}");
            assert_eq!(store.jump_quest_run(id).unwrap(), None, "{warden} stops the timer");
            let base = s.clock_ms;
            let _ = s.tick(base + 1_000);
            let _ = s.tick(base + 60_000);
            assert_eq!(store.jump_quest_run(id).unwrap(), None, "and nothing is counted in town");
        }
    }

    /// **The hour carries across the steps of one course.** The owner, 2026-10-04: *"when the
    /// player transitions from one stage of the jump quest to the next (but still for the same
    /// quest...) the timer is persisted throughout those maps"*. Deep Forest step 3 to step 4
    /// through the `in00` portal (a map change like any other): the same run, the time adding
    /// up across both, and the reminder still due on step 4.
    #[test]
    fn the_hour_carries_across_the_steps_of_one_course() {
        let (store, mut s, id) = standing(10_005_042, jq::CRUMBLING_STATUE, (0, 0), 50);
        store.start_jump_quest(id, jq::BLUE_PILE).unwrap();
        let base = s.clock_ms;
        let _ = s.tick(base + 1_000);
        let _ = s.tick(base + 1_800_000 + 1_000);
        assert_eq!(store.jump_quest_run(id).unwrap().unwrap().spent_secs, 1_800, "half an hour on step 3");
        let mut chr = s.claimed_character().unwrap();
        let _ = s.go_to_map(&mut chr, 10_005_043, 0, "in00 to step 4".to_string());
        let _ = s.tick(base + 1_802_000);
        let reminder = notices(&s.tick(base + 3_611_000));
        let run = store.jump_quest_run(id).unwrap().unwrap();
        assert_eq!((run.goal_npc, run.spent_secs), (jq::BLUE_PILE, 3_610), "one run, both steps' time");
        assert_eq!(reminder.len(), 1, "the hour is up on step 4: {reminder:?}");
    }

    /// **Off the course, the server stops keeping track** - including a log in that lands in
    /// town with a run left over from a log out on a course, and a GM warp onto another course.
    #[test]
    fn a_run_is_dropped_off_the_course_and_on_another_course() {
        let (store, mut s, id) = standing(jq::SLEEPYWOOD, jq::MYSTERIOUS_STATUE, (1061, 255), 50);
        store.start_jump_quest(id, jq::WHITE_PILE).unwrap();
        let _ = s.tick(s.clock_ms + 1_000);
        assert_eq!(store.jump_quest_run(id).unwrap(), None, "logged in off the course: the leftover run is gone");

        let (store, mut s, id) = standing(10_005_044, jq::CRUMBLING_STATUE, (0, 0), 50);
        store.start_jump_quest(id, jq::WHITE_PILE).unwrap();
        let base = s.clock_ms;
        let _ = s.tick(base + 1_000);
        let mut chr = s.claimed_character().unwrap();
        let _ = s.go_to_map(&mut chr, 10_003_100, 0, "a GM warp to B1".to_string());
        let _ = s.tick(base + 2_000);
        let _ = s.tick(base + 15_000);
        assert_eq!(store.jump_quest_run(id).unwrap(), None, "another course is not this one");
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
        assert_eq!(store.jump_quest_run(id).unwrap(), None, "not on Shumi's quest: a reward run, no timer");
    }

    fn chat(text: &str) -> Vec<u8> {
        let mut b = net::opcode::CLIENT_CHAT.to_le_bytes().to_vec();
        b.extend_from_slice(&[0u8; 4]);
        b.extend_from_slice(&(text.len() as u16).to_le_bytes());
        b.extend_from_slice(text.as_bytes());
        b.push(3);
        b
    }

    fn notices(out: &[Reply]) -> Vec<String> {
        out.iter()
            .filter(|r| r.opcode == net::notice::CHAT_NOTICE)
            .map(|r| {
                let len = u16::from_le_bytes([r.body[1], r.body[2]]) as usize;
                String::from_utf8_lossy(&r.body[3..3 + len]).to_string()
            })
            .collect()
    }

    /// **The pity timer.** A door starts the hour; nothing is said before it; at the hour one
    /// yellow reminder, then one every five minutes and not more often.
    #[test]
    fn the_hour_starts_at_the_door_and_the_reminders_come_every_five_minutes_after() {
        let (store, mut s, id) = standing(jq::SLEEPYWOOD, jq::MYSTERIOUS_STATUE, (1061, 255), 50);
        store.start_quest(id, 10_007).unwrap();
        let _ = s.handle(&click());
        let _ = s.handle(&pick(1));
        assert_eq!(map_of(&s), 10_005_042);
        let run = store.jump_quest_run(id).unwrap().unwrap();
        assert_eq!((run.goal_npc, run.spent_secs), (jq::BLUE_PILE, 0), "the blue course's hour");
        let entry = s.on_field_entered();
        assert!(
            entry.iter().any(|r| r.opcode == net::clock::FIELD_CLOCK && r.body == net::clock::clock_seconds(3_600)),
            "the field entry after the door's warp shows 60:00"
        );

        let base = s.clock_ms;
        assert!(notices(&s.tick(base + 1_000)).is_empty());
        assert!(notices(&s.tick(base + 3_599_000)).is_empty(), "59:59 - nothing yet");
        let at_hour = notices(&s.tick(base + 3_610_000));
        assert_eq!(at_hour.len(), 1, "{at_hour:?}");
        assert!(at_hour[0].contains("over an hour") && at_hour[0].contains("!skipjq"), "{at_hour:?}");
        assert!(notices(&s.tick(base + 3_610_000 + 200_000)).is_empty(), "three minutes later: quiet");
        assert_eq!(notices(&s.tick(base + 3_610_000 + 300_000)).len(), 1, "five minutes after the hour");
        assert_eq!(store.jump_quest_run(id).unwrap().unwrap().notices, 2, "kept, so a reconnect does not repeat one");
    }

    /// **Only a quest entry starts the hour.** The owner, 2026-10-04: *"it should not be active
    /// when the player is completing additional attempts for just the jump quest reward."* No
    /// quest, a finished quest, or a quest whose item is already held: no timer, and a leftover
    /// run is cleared. On the quest and still short of the item: the hour starts.
    #[test]
    fn only_an_entry_on_the_courses_quest_starts_the_hour() {
        let goal = jq::goal_for(jq::BLUE_PILE).unwrap();
        let enter = |s: &mut Session| {
            let mut chr = s.claimed_character().unwrap();
            let _ = s.teleport(&mut chr, jq::SLEEPYWOOD, "back to the statue".to_string());
            let _ = s.handle(&click());
            let _ = s.handle(&pick(1));
        };
        let (store, mut s, id) = standing(jq::SLEEPYWOOD, jq::MYSTERIOUS_STATUE, (1061, 255), 50);
        store.start_jump_quest(id, jq::CHEST_B3).unwrap();
        enter(&mut s);
        assert_eq!(store.jump_quest_run(id).unwrap(), None, "no quest: a reward run, and the leftover is gone");

        store.start_quest(id, goal.quest).unwrap();
        store.add_item(id, store::InventoryType::Etc, &store::Item::bundle(goal.item, 20), 100).unwrap();
        enter(&mut s);
        assert_eq!(store.jump_quest_run(id).unwrap(), None, "the 20 Blue Violas are already held");

        let tab_rows = store.bag_items(id, store::InventoryType::Etc).unwrap();
        store.remove_item(id, store::InventoryType::Etc, tab_rows[0].slot, Some(5)).unwrap();
        enter(&mut s);
        assert_eq!(store.jump_quest_run(id).unwrap().map(|r| r.goal_npc), Some(goal.npc), "15 of 20: a quest entry");

        store.complete_quest(id, goal.quest).unwrap();
        enter(&mut s);
        assert_eq!(store.jump_quest_run(id).unwrap(), None, "a finished quest: reward runs only");
    }

    /// **The hour is on screen**: entering a course field with a running hour sends the party
    /// quest's countdown with the seconds left; a reward-only run (no row) and a field off the
    /// course get none.
    #[test]
    fn entering_a_course_field_shows_the_countdown_with_the_time_left() {
        let clock = |out: &[Reply]| -> Vec<Vec<u8>> {
            out.iter().filter(|r| r.opcode == net::clock::FIELD_CLOCK).map(|r| r.body.clone()).collect()
        };
        let (store, mut s, id) = standing(10_005_043, jq::CRUMBLING_STATUE, (0, 0), 50);
        assert!(clock(&s.on_field_entered()).is_empty(), "no run, no clock");

        store.start_jump_quest(id, jq::BLUE_PILE).unwrap();
        store.add_jump_quest_time(id, 600).unwrap();
        assert_eq!(clock(&s.on_field_entered()), vec![net::clock::clock_seconds(3_000)], "50 minutes left");

        store.add_jump_quest_time(id, 3_500).unwrap();
        assert_eq!(clock(&s.on_field_entered()), vec![net::clock::clock_seconds(0)], "past the hour: zero, !skipjq open");

        let mut chr = s.claimed_character().unwrap();
        let _ = s.teleport(&mut chr, jq::SLEEPYWOOD, "out".to_string());
        assert!(clock(&s.on_field_entered()).is_empty(), "off the course");
    }

    /// **A disconnect keeps the hour**, the seconds not yet written included.
    #[test]
    fn a_disconnect_keeps_the_time_counted_so_far() {
        let (store, mut s, id) = standing(10_003_101, jq::EXIT, (0, 0), 30);
        store.start_jump_quest(id, jq::CHEST_B1).unwrap();
        let base = s.clock_ms;
        let _ = s.tick(base + 1_000);
        let _ = s.tick(base + 8_000);
        assert_eq!(store.jump_quest_run(id).unwrap().unwrap().spent_secs, 0, "7 s gathered, not yet written");
        let account = s.claimed().unwrap().account_id;
        let config = s.config.clone();
        drop(s);
        assert_eq!(store.jump_quest_run(id).unwrap().unwrap().spent_secs, 7, "written as the connection closed, and kept");

        // **Back in, on the course: the hour carries on from where it was.** The owner,
        // 2026-10-04: *"if a player disconnected and later logged back in to continue working on
        // the jump quest, I would like to have them keep the previous time left."*
        store.create_migration(account, id, 0, 0).unwrap();
        let mut back = Session::joining(store.clone(), config, Arc::new(Fields::new()));
        back.claim_for_character(id);
        let base = back.clock_ms;
        let _ = back.tick(base + 1_000);
        let _ = back.tick(base + 11_000);
        assert_eq!(store.jump_quest_run(id).unwrap().unwrap().spent_secs, 17, "7 before the disconnect, 10 after");
        let left = notices(&back.handle(&chat("!skipjq")));
        assert!(left.iter().any(|n| n.contains("60 more minutes")), "3 583 s left rounds up to 60: {left:?}");
    }

    /// **`!skipjq`**: refused before the hour with what is left; after it, the quest item the
    /// quest still wants and NOTHING else, out to town, the run over. Off a course it says so.
    #[test]
    fn skipjq_after_the_hour_gives_the_quest_item_only_and_leaves() {
        let goal = jq::goal_for(jq::PINK_PILE).unwrap();
        let (store, mut s, id) = standing(10_005_040, jq::CRUMBLING_STATUE, (0, 0), 50);
        store.start_quest(id, goal.quest).unwrap();
        store.start_jump_quest(id, goal.npc).unwrap();

        let early = notices(&s.handle(&chat("!skipjq")));
        assert!(early.iter().any(|n| n.contains("60 more minutes")), "{early:?}");
        assert_eq!(map_of(&s), 10_005_040, "still on the course");

        store.add_jump_quest_time(id, jq::PITY_SECS).unwrap();
        let out = s.handle(&chat("!skipjq"));
        assert_eq!(held(&store, id, goal.item), 10, "John's ten Pink Violas");
        let use_items = store.bag_items(id, store::InventoryType::Use).unwrap();
        assert!(use_items.is_empty(), "no consumable or scroll: {use_items:?}");
        assert_eq!(map_of(&s), jq::SLEEPYWOOD);
        assert!(out.iter().any(|r| r.what.starts_with("SetField") && r.what.contains("(portal forest00)")));
        assert_eq!(store.jump_quest_run(id).unwrap(), None, "the run is over");

        let off = notices(&s.handle(&chat("!skipjq")));
        assert!(off.iter().any(|n| n.contains("only works on a jump quest")), "{off:?}");
    }

    /// Leaving a course any other way - a return scroll, a GM warp - ends the run, so the next
    /// visit starts a fresh hour rather than inheriting this one.
    #[test]
    fn leaving_the_course_any_other_way_ends_the_run() {
        let (store, mut s, id) = standing(10_003_101, jq::EXIT, (0, 0), 30);
        store.start_jump_quest(id, jq::CHEST_B1).unwrap();
        let base = s.clock_ms;
        let _ = s.tick(base + 1_000);
        let _ = s.tick(base + 20_000);
        assert_eq!(store.jump_quest_run(id).unwrap().unwrap().spent_secs, 19, "time on the course counts");
        let mut chr = s.claimed_character().unwrap();
        let _ = s.teleport(&mut chr, jq::ELLINIA, "a return scroll".to_string());
        let _ = s.tick(base + 22_000);
        assert_eq!(store.jump_quest_run(id).unwrap(), None);
    }
}
