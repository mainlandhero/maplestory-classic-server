//! Lakelis and the "First Time Together" entry, on the wire. The rules are
//! `crate::firsttime`; the quest as the client holds it is
//! `research/first-time-together-pq.md`.
//!
//! A click on Lakelis in Kerning City opens a yes/no with their own line. Yes runs
//! `firsttime::check`; a refusal is a Say naming the rule that failed, and an acceptance
//! opens an instance and sends **every member** to stage 1 - the leader from here, the rest
//! through `Event::PartyQuestEnter`, because only a member's own session can build their
//! `SetField`.

use super::{Conversation, Reply, Session};
use crate::firsttime::{self};

impl Session {
    /// The click on Lakelis, in Kerning City. `None` for any other NPC or any other map, so
    /// the ordinary dialogue path keeps their elsewhere.
    pub(super) fn open_first_time_together(&mut self, template: u32) -> Option<Vec<Reply>> {
        let chr = self.claimed_character()?;
        if template != firsttime::LAKELIS || chr.map_id != firsttime::ENTRY_MAP {
            return None;
        }
        self.conversation = Some(Conversation {
            npc_template: template,
            quest_id: None,
            path: firsttime::ASK_PATH.to_string(),
            sent: 0,
            awaiting_yes_no: true,
            sent_with_next: false,
        });
        Some(vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_ask(template, firsttime::GREETING, false),
            what: format!(
                "ScriptMessage YES/NO from NPC {template} (Lakelis): take on First Time Together? \
                 The party gate runs on Yes"
            ),
        }])
    }

    /// Lakelis' yes/no came back. Anything but Yes leaves without a word, which is what them
    /// own `no` node in `Quest.wz` does.
    pub(super) fn first_time_together_answer(&mut self, action: i8) -> Vec<Reply> {
        self.conversation = None;
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        if action != net::script::SCRIPT_ACTION_YES {
            crate::server::log(&format!("   first time together: {} declined at Lakelis", chr.name));
            return Vec::new();
        }
        // The party registry is locked only for the read, because `check` calls back into
        // the store for every member and holding both locks is how a deadlock starts.
        let party = self.fields.parties().party_of(chr.id).cloned();
        // Who is playing on this channel, and which of them are standing in Kerning City.
        // Both come from the broadcast bus's presence table, which is the same thing that
        // decides who can see whom - so "here with you" means exactly what it looks like on
        // screen. A member on another channel is in neither set; see `Refusal::NotHere`.
        let members: Vec<u32> = party.as_ref().map(|p| p.members.clone()).unwrap_or_default();
        let online: std::collections::HashSet<u32> = self.bus().online_characters().into_iter().collect();
        let here: std::collections::HashSet<u32> =
            self.bus().characters_on(crate::fields::FieldKey::world(firsttime::ENTRY_MAP), &members).into_iter().collect();
        let store = self.store.clone();
        let gate = firsttime::check(chr.id, party.as_ref(), |id| {
            store.character_brief(id).ok().flatten().map(|b| firsttime::Candidate {
                character: id,
                name: b.name,
                level: u16::try_from(b.level).unwrap_or(u16::MAX),
                online: online.contains(&id),
                here: here.contains(&id),
            })
        });
        let members = match gate {
            Ok(members) => members,
            Err(why) => {
                crate::server::log(&format!(
                    "   first time together: {} ({}) refused at the gate: {why:?}",
                    chr.name, chr.id
                ));
                return vec![Reply {
                    opcode: net::script::SCRIPT_MESSAGE,
                    body: net::script::npc_say(firsttime::LAKELIS, &why.line(), false, false),
                    what: format!("ScriptMessage Say from Lakelis: entry refused - {why:?}"),
                }];
            }
        };
        let Some(party) = party else { return Vec::new() };
        let instance = self.fields.runs().open(
            party.id,
            members.iter().map(|m| m.character).collect(),
            store::Store::unix_now(),
        );
        crate::server::log(&format!(
            "   first time together: party {} enters as instance {} with {} member(s): {}",
            party.id,
            instance.id,
            members.len(),
            members.iter().map(|m| format!("{} ({})", m.name, m.character)).collect::<Vec<_>>().join(", ")
        ));
        // Everyone but the leader warps themselves: their own session owns their record and
        // their client's SetField. A member who is not reachable is logged and left behind
        // rather than silently counted as sent.
        for member in members.iter().filter(|m| m.character != chr.id) {
            let sent = self.bus().publish_event_to_character(
                member.character,
                crate::broadcast::Event::PartyQuestEnter {
                    map: firsttime::STAGE_1,
                    why: format!("First Time Together, instance {}", instance.id),
                },
            );
            if !sent {
                crate::server::log(&format!(
                    "   first time together: member {} ({}) is not reachable on this channel; they stay behind",
                    member.name, member.character
                ));
            }
        }
        let mut chr = chr;
        self.go_to_map(&mut chr, firsttime::STAGE_1, 0, format!("First Time Together, instance {}", instance.id))
    }

    /// The other members' side of the entry: warp, sent as an `Event` rather than as bytes
    /// because only this session can build this character's `SetField`.
    pub(super) fn enter_party_quest(&mut self, map: u32, why: &str) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        crate::server::log(&format!("   first time together: {} ({}) pulled in - {why}", chr.name, chr.id));
        let mut out = if why == firsttime::TOO_FEW_LEFT { self.notice(why.to_string()) } else { Vec::new() };
        if map == firsttime::EXIT_MAP && chr.map_id == firsttime::EXIT_MAP {
            // Already standing there: a second SetField of the same map is a visible reload
            // for nothing.
            return out;
        }
        out.extend(self.go_to_map(&mut chr, map, 0, why.to_string()));
        out
    }

    /// **Take `who` out of their run - and end the run if too few are left.** Every way out
    /// comes through here: leaving the party, Nella, a disconnect, a login onto a stage.
    ///
    /// The owner, 2026-09-23: *"A party of 1 should not be allowed to continue doing the party
    /// quest."* So when the departure leaves fewer than `MIN_PARTY` in the run, the run is
    /// closed and whoever is left is sent to the Exit with `TOO_FEW_LEFT` - this session
    /// directly if it is them, anyone else through `Event::PartyQuestEnter`. `close` is the
    /// test-and-set: two sessions noticing the same departure cannot both end it.
    ///
    /// Returns this session's own packets; the leaver's own warp is the caller's business.
    fn drop_from_run(&mut self, who: u32, why: &str) -> Vec<Reply> {
        let was = self.fields.runs().drop_member(who);
        let Some(was) = was else { return Vec::new() };
        let left = self.fields.runs().instance(was.id);
        let Some(left) = left else { return Vec::new() }; // nobody left; it is already gone
        if left.members.len() >= firsttime::MIN_PARTY {
            return Vec::new();
        }
        let closed = self.fields.runs().close(left.id);
        if !closed {
            return Vec::new();
        }
        crate::server::log(&format!(
            "   first time together: character {who} left instance {} ({why}); {} member(s) left, fewer than {} - the run ends and they go to the Exit",
            left.id,
            left.members.len(),
            firsttime::MIN_PARTY
        ));
        let me = self.claimed_character().map(|c| c.id);
        let mut out = Vec::new();
        for member in left.members {
            if Some(member) == me {
                out.extend(self.notice(firsttime::TOO_FEW_LEFT.to_string()));
                out.extend(self.leave_party_quest(firsttime::TOO_FEW_LEFT));
                continue;
            }
            self.bus().publish_event_to_character(
                member,
                crate::broadcast::Event::PartyQuestEnter { map: firsttime::EXIT_MAP, why: firsttime::TOO_FEW_LEFT.to_string() },
            );
        }
        out
    }

    /// **The countdown, on every field entry inside the quest.** Sent from the field-entry
    /// path rather than from the entry itself, so it survives a stage change and so the
    /// members pulled in by `Event::PartyQuestEnter` get it without a second message.
    ///
    /// Type 2 (`net::clock::clock_seconds`), not the map clock: the seven quest fields
    /// declare no `clock` node, and type 1 throws without one. Nothing is sent outside the
    /// quest or for a character with no run.
    pub(super) fn party_quest_clock(&mut self) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        if !firsttime::is_quest_map(chr.map_id) {
            return Vec::new();
        }
        let run = self.fields.runs().instance_of(chr.id);
        let Some(inst) = run else { return Vec::new() };
        let left = inst.remaining_s(store::Store::unix_now());
        vec![Reply {
            opcode: net::clock::FIELD_CLOCK,
            body: net::clock::clock_seconds(left),
            what: format!(
                "FieldClock type 2 to character {}: {left}s left of instance {} - the countdown \
                 builds its own widget, which is why it is safe on a map with no clock node",
                chr.id, inst.id
            ),
        }]
    }

    /// **The clock ran out.** Checked on the tick; the instance is taken out of the registry
    /// as it is handed over, so two members ticking at once cannot both expire the same run.
    /// Everyone still in it is sent to the Exit map - this session directly if it is one of
    /// them, the rest through `Event::PartyQuestEnter`.
    pub(super) fn party_quest_timer_tick(&mut self) -> Vec<Reply> {
        let over = self.fields.runs().take_expired(store::Store::unix_now());
        if over.is_empty() {
            return Vec::new();
        }
        let me = self.claimed_character().map(|c| c.id);
        let mut out = Vec::new();
        for inst in over {
            crate::server::log(&format!(
                "   first time together: instance {} ran out of time; {} member(s) go to the Exit",
                inst.id,
                inst.members.len()
            ));
            for member in &inst.members {
                if Some(*member) == me {
                    out.extend(self.notice("Time is up.".to_string()));
                    out.extend(self.leave_party_quest("the time limit ran out"));
                    continue;
                }
                self.bus().publish_event_to_character(
                    *member,
                    crate::broadcast::Event::PartyQuestEnter {
                        map: firsttime::EXIT_MAP,
                        why: "the time limit ran out".to_string(),
                    },
                );
            }
        }
        out
    }

    /// **Out to the Exit map**, and out of the run. Used by the timer, by Nella, and by a
    /// departure from the party - the owner, 2026-09-22: *"If anyone leaves the party, the person
    /// that left will also be immediately brought to the party exit."*
    pub(super) fn leave_party_quest(&mut self, why: &str) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let mut out = self.drop_from_run(chr.id, why);
        if !firsttime::is_quest_map(chr.map_id) || chr.map_id == firsttime::EXIT_MAP {
            // Not inside, or already standing at the Exit: dropping them from the run is the
            // whole effect. Warping someone who is in Kerning City would be a bug with a
            // very confusing screen.
            return out;
        }
        crate::server::log(&format!(
            "   first time together: {} ({}) leaves for the Exit map - {why}",
            chr.name, chr.id
        ));
        out.extend(self.go_to_map(&mut chr, firsttime::EXIT_MAP, 0, format!("First Time Together: {why}")));
        out
    }

    /// **Somebody left the party, so they leave the quest.** The owner, 2026-09-22: *"If anyone
    /// leaves the party, the person that left will also be immediately brought to the party
    /// exit."*
    ///
    /// Works for this connection and for anybody else: the registry is process-wide so the
    /// drop happens here either way, and the warp goes through `Event::PartyQuestEnter` when
    /// the leaver is somebody else, for the usual reason - only their session can build
    /// their `SetField`. Does nothing for a character who was not in a run.
    pub(super) fn eject_from_party_quest(&mut self, who: u32, why: &str) -> Vec<Reply> {
        let run = self.fields.runs().instance_of(who);
        if run.is_none() {
            return Vec::new();
        }
        if self.claimed_character().map(|c| c.id) == Some(who) {
            return self.leave_party_quest(why);
        }
        let out = self.drop_from_run(who, why);
        self.bus().publish_event_to_character(
            who,
            crate::broadcast::Event::PartyQuestEnter { map: firsttime::EXIT_MAP, why: why.to_string() },
        );
        crate::server::log(&format!("   first time together: character {who} is out of the quest - {why}"));
        out
    }

    /// Nella, in any of the seven fields. Inside the quest they offer the way out; on the
    /// Exit map they offer Kerning City. `None` for any other NPC or map.
    pub(super) fn open_nella(&mut self, template: u32) -> Option<Vec<Reply>> {
        let chr = self.claimed_character()?;
        if template != firsttime::NELLA || !firsttime::is_quest_map(chr.map_id) {
            return None;
        }
        let line = if chr.map_id == firsttime::EXIT_MAP { firsttime::NELLA_TOWN } else { firsttime::NELLA_LEAVE };
        self.conversation = Some(Conversation {
            npc_template: template,
            quest_id: None,
            path: firsttime::NELLA_PATH.to_string(),
            sent: 0,
            awaiting_yes_no: true,
            sent_with_next: false,
        });
        Some(vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_ask(template, line, false),
            what: format!("ScriptMessage YES/NO from NPC {template} (Nella) on map {}", chr.map_id),
        }])
    }

    /// Nella's yes/no. Yes on the Exit map goes to Kerning City; Yes anywhere else inside
    /// the quest goes to the Exit map. Anything but Yes leaves them where they stand.
    pub(super) fn nella_answer(&mut self, action: i8) -> Vec<Reply> {
        self.conversation = None;
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        if action != net::script::SCRIPT_ACTION_YES {
            return Vec::new();
        }
        if chr.map_id == firsttime::EXIT_MAP {
            crate::server::log(&format!("   first time together: {} ({}) leaves the Exit for Kerning City", chr.name, chr.id));
            // Already out of the run by the time they reach the Exit, but a member who
            // logged back in there may not be; dropping again is harmless.
            let mut out = self.drop_from_run(chr.id, "Nella sent them home");
            out.extend(self.go_to_map(&mut chr, firsttime::TOWN_MAP, 0, "First Time Together: Nella sends them home".to_string()));
            return out;
        }
        self.leave_party_quest("Nella showed them out")
    }

    /// **Cloto.** Every stage has its own rule: `cloto_stage_one`, `cloto_zone_stage` for
    /// 2-4, `cloto_stage_five`. The temporary click-to-clear is gone.
    ///
    /// `None` outside a run, so a GM who walked in with `!map` falls through to their ordinary
    /// dialogue.
    pub(super) fn open_cloto(&mut self, template: u32) -> Option<Vec<Reply>> {
        let chr = self.claimed_character()?;
        if template != firsttime::CLOTO {
            return None;
        }
        let inst = self.fields.runs().instance_of(chr.id);
        let inst = inst?;
        firsttime::next_stage(chr.map_id)?;
        self.conversation = None;
        if inst.cleared.contains(&chr.map_id) {
            return Some(vec![self.cloto_say(
                firsttime::CLOTO_ALREADY,
                false,
                format!("map {} already cleared by instance {}", chr.map_id, inst.id),
            )]);
        }
        if chr.map_id == firsttime::STAGE_1 {
            return Some(self.cloto_stage_one(&chr, &inst));
        }
        if let Some(stage) = firsttime::zone_stage(chr.map_id) {
            return Some(self.cloto_zone_stage(&chr, &inst, stage));
        }
        if chr.map_id == firsttime::STAGE_5 {
            return Some(self.cloto_stage_five(&chr, &inst));
        }
        // No stage map is left without its own rule; anything else is their ordinary dialogue.
        None
    }

    /// One line from Cloto. `next` puts a Next button on it.
    fn cloto_say(&self, line: &str, next: bool, what: String) -> Reply {
        Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(firsttime::CLOTO, line, false, next),
            what: format!("ScriptMessage Say from Cloto: {what}"),
        }
    }

    /// **Stage 1.** The owner, 2026-09-23. The leader gets a menu - take a question like everyone
    /// else, or hand in the Passes - and every other member goes straight to their mission.
    ///
    /// "Leader" is the party's leader now, from the party registry, so a crown that changed
    /// hands mid-run moves the menu with it.
    fn cloto_stage_one(&mut self, chr: &net::opcode::Character, inst: &firsttime::Instance) -> Vec<Reply> {
        let leads = self.fields.parties().party_of(chr.id).map(|p| p.leader) == Some(chr.id);
        if !leads {
            return self.cloto_mission(chr);
        }
        let required = firsttime::passes_required(inst.members.len());
        self.conversation = Some(Conversation {
            npc_template: firsttime::CLOTO,
            quest_id: None,
            path: firsttime::CLOTO_MENU_PATH.to_string(),
            sent: 0,
            awaiting_yes_no: false,
            sent_with_next: false,
        });
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_menu(firsttime::CLOTO, &firsttime::cloto_menu(required)),
            what: format!(
                "ScriptMessage MENU from Cloto to leader {} ({}): a question, or hand in {required} Pass(es) for instance {}",
                chr.name, chr.id, inst.id
            ),
        }]
    }

    /// **Stages 2, 3 and 4 - the ropes, the platforms and the barrels.** The owner, 2026-09-23: *"In a 2 person
    /// party, 2 people must hang from the 2 correct ropes then have the party leader talk to
    /// Cloto. The server randomly decides for this particular party instance that which of
    /// the 2 ropes are correct."* Three for a party of three or four; and stage 3 is the same
    /// with five platforms (*"The server randomly selects 2 of these platforms (or 3, in a
    /// 3-4 player party)"*).
    ///
    /// A member gets the intro. The leader gets the intro too while nobody is on a rope -
    /// it is the only way the leader reads it - and otherwise the verdict: the wrong count
    /// says so; the right count on the wrong ropes plays WRONG for the whole run, with no
    /// dialogue at all (the owner's rule); the right
    /// ropes clear the stage (which still requires everyone in the run to be on it).
    ///
    /// "On a rope" (or platform) is standing inside one of the client's own `area`
    /// rectangles (`firsttime::STAGE_2_ROPES`, `STAGE_3_PLATFORMS`), read from each
    /// member's last reported position on the bus.
    fn cloto_zone_stage(&mut self, chr: &net::opcode::Character, inst: &firsttime::Instance, stage: firsttime::ZoneStage) -> Vec<Reply> {
        let needed = usize::try_from(firsttime::passes_required(inst.members.len())).unwrap_or(3);
        let intro = |s: &Self| vec![s.cloto_say(&(stage.intro)(needed), false, format!("map {} intro to {}", stage.map, chr.name))];
        let leads = self.fields.parties().party_of(chr.id).map(|p| p.leader) == Some(chr.id);
        if !leads {
            return intro(self);
        }
        let key = self.field_of(chr);
        let at: Vec<(i16, i16)> = self.bus().positions_on(key, &inst.members).into_iter().map(|(_, p)| p).collect();
        let on = at.iter().filter(|&&p| stage.zones.iter().any(|r| r.contains(p))).count();
        if on == 0 {
            return intro(self);
        }
        let roll = self.rng.next();
        let answer = self.fields.runs().answer_for(chr.id, stage.map, needed, stage.zones.len(), roll);
        let Some(answer) = answer else { return Vec::new() };
        crate::server::log(&format!(
            "   first time together: {} ({}) asked Cloto on map {} with {on} of {needed} on the {}; the answer is {}",
            chr.name, chr.id, stage.map, stage.noun, stage.describe(&answer)
        ));
        match firsttime::check_zones(stage.zones, &at, &answer, needed) {
            firsttime::ZoneCheck::Count { on, needed } => {
                vec![self.cloto_say(&firsttime::cloto_zone_count(on, needed, &stage), false, format!("{on} on the {}, {needed} needed", stage.noun))]
            }
            firsttime::ZoneCheck::Wrong => {
                crate::server::log(&format!(
                    "   first time together: instance {} tried the wrong {} on map {} - the answer is {}",
                    inst.id, stage.noun, stage.map, stage.describe(&answer)
                ));
                // **The animation and nothing else.** The owner, 2026-09-23: *"If the combination is
                // incorrect, clicking on Cloto will only play the animation, no dialogue will
                // be generated for getting a combination wrong."*
                self.party_quest_effects(
                    key,
                    &[
                        (net::fieldeffect::screen(net::fieldeffect::SCREEN_PARTY_WRONG), "screen quest/party/wrong"),
                        (net::fieldeffect::sound(net::fieldeffect::SOUND_PARTY_FAILED, 100), "sound Party1/Failed"),
                    ],
                    &format!("wrong {}, instance {}", stage.noun, inst.id),
                )
            }
            firsttime::ZoneCheck::Right => self.cloto_clear(chr, inst, &format!("the right {}", stage.describe(&answer))),
        }
    }

    /// Field effects to this screen and to `key` on the bus - which is one run's copy of
    /// the map, so no other party sees them.
    fn party_quest_effects(&self, key: crate::fields::FieldKey, effects: &[(Vec<u8>, &str)], why: &str) -> Vec<Reply> {
        let mut out = Vec::new();
        for (body, what) in effects {
            let reply = Reply {
                opcode: net::fieldeffect::FIELD_EFFECT,
                body: body.clone(),
                what: format!("FieldEffect {what}: {why}"),
            };
            self.bus().publish(self.subscriber, key, reply.clone(), None);
            out.push(reply);
        }
        out
    }

    /// **The last stage.** The owner, 2026-09-23: *"The Party Leader should talk with Cloto once
    /// all 10 passes has been collected in exchange to clear the stage. Upon clearing the
    /// stage, each member of the party will immediately receive Companion's Magic Box as a
    /// reward."* A member gets the intro; the leader with ten Passes clears it (the whole run
    /// must be here, checked before a Pass is taken) and every member of the run is rewarded.
    fn cloto_stage_five(&mut self, chr: &net::opcode::Character, inst: &firsttime::Instance) -> Vec<Reply> {
        let leads = self.fields.parties().party_of(chr.id).map(|p| p.leader) == Some(chr.id);
        if !leads {
            return vec![self.cloto_say(&firsttime::cloto_stage5_intro(), false, format!("last stage intro to {}", chr.name))];
        }
        if let Some(wait) = self.cloto_waiting_for(chr, inst) {
            return vec![wait];
        }
        let held = self.held(chr.id, firsttime::PASS);
        if held < firsttime::STAGE_5_PASSES {
            return vec![self.cloto_say(&firsttime::cloto_stage5_short(held), false, format!("{} holds {held} of 10 Passes", chr.name))];
        }
        let mut out = self.take_items(chr.id, store::InventoryType::Etc, firsttime::PASS, firsttime::STAGE_5_PASSES);
        out.extend(self.cloto_clear(chr, inst, "10 Passes handed in on the last stage"));
        let cleared = self.fields.runs().is_cleared(chr.id, firsttime::STAGE_5);
        if !cleared {
            return out;
        }
        // **The reward, to every member of the run** - the leader here, the rest through
        // their own sessions. All of them are on this stage: `cloto_clear` refused otherwise.
        let why = format!("First Time Together cleared, instance {}", inst.id);
        for &member in &inst.members {
            if member == chr.id {
                out.extend(self.receive_party_quest_reward(firsttime::COMPANIONS_MAGIC_BOX, &why));
                continue;
            }
            let sent = self.bus().publish_event_to_character(
                member,
                crate::broadcast::Event::PartyQuestReward { item: firsttime::COMPANIONS_MAGIC_BOX, why: why.clone() },
            );
            if !sent {
                crate::server::log(&format!("   first time together: member {member} could not be reached for the reward"));
            }
        }
        out
    }

    /// A reward arriving in this character's bag.
    pub(super) fn receive_party_quest_reward(&mut self, item: u32, why: &str) -> Vec<Reply> {
        match self.give_item(item, 1, why) {
            Ok((line, replies)) => {
                crate::server::log(&format!("   first time together: reward - {line} ({why})"));
                replies
            }
            Err(e) => {
                crate::server::log(&format!("   first time together: reward {item} NOT given - {e} ({why})"));
                self.notice(format!("Your reward could not be given: {e}"))
            }
        }
    }

    /// **Open a Companion's Magic Box.** One line of `crate::magicbox`'s table, equal odds.
    ///
    /// The prize goes in **first** and the box comes out only if it did: a full tab refuses
    /// the prize, and then the player keeps the box and is told why - the same order the
    /// Leaf coupons use, so a refusal never costs anything. The slot must hold the box the
    /// packet names, or nothing happens.
    pub(super) fn open_magic_box(&mut self, slot: u16) -> Vec<Reply> {
        let op = net::cashitem::CLIENT_USE_CASH_ITEM;
        let Some(chr) = self.claimed_character() else { return crate::mesodrop::unlock_unhandled_latching_request(op) };
        let inv = store::InventoryType::Use;
        let holding = self
            .store
            .bag_items(chr.id, inv)
            .ok()
            .into_iter()
            .flatten()
            .find(|r| r.slot == slot)
            .map(|r| r.item.item_id);
        if holding != Some(crate::magicbox::BOX) {
            crate::server::log(&format!(
                "   magic box: character {} asked to open Use slot {slot}, which holds {holding:?}; nothing opened",
                chr.id
            ));
            return self.cash_item_notice_for(op, "That box is not where the client says it is. Nothing was used up.".to_string());
        }
        let roll = self.rng.next();
        let (item, qty) = crate::magicbox::roll(roll);
        let (line, mut out) = match self.give_item(item, qty, "Companion's Magic Box") {
            Ok(given) => given,
            Err(why) => {
                crate::server::log(&format!("   magic box: character {} rolled {qty} x {item} and could not take it: {why}; the box is kept", chr.id));
                return self.cash_item_notice_for(
                    op,
                    "There is no room for what is inside. Make room in your inventory and open it again - the box was kept.".to_string(),
                );
            }
        };
        let _ = self.store.remove_item(chr.id, inv, slot, Some(1));
        let left = self
            .store
            .bag_items(chr.id, inv)
            .ok()
            .into_iter()
            .flatten()
            .find(|r| r.slot == slot)
            .map(|r| r.item.kind.quantity())
            .unwrap_or(0);
        out.extend(self.stack_change_replies(inv, slot, left));
        let name = self.item_name(item);
        crate::server::log(&format!("   magic box: character {} opened a box from Use slot {slot} - {line}", chr.id));
        let said = if qty > 1 { format!("You received {qty} {name} from the Companion's Magic Box.") } else { format!("You received {name} from the Companion's Magic Box.") };
        out.extend(self.cash_item_notice_for(op, said));
        out
    }

    /// **The last stage fills all at once**, the first time anyone in the run walks in.
    /// `Fields::seed_all_now` has the reasoning; this only decides that it is this map, in a
    /// run, and stands the mobs up so the field-entry batch that follows carries them.
    pub(super) fn fill_last_stage(&mut self, chr: &net::opcode::Character) {
        if chr.map_id != firsttime::STAGE_5 {
            return;
        }
        let key = self.field_of(chr);
        if !key.is_instanced() {
            return;
        }
        if self.fields.seed_all_now(key, &self.config, self.clock_ms) {
            let up = self.fields.due_respawns(key, &self.config, self.clock_ms);
            crate::server::log(&format!(
                "   first time together: the last stage for field {key} filled at once - {} mob(s)",
                up.len()
            ));
        }
    }

    /// **A kill on the last stage.** The King Slime leaves a pair of Squishy Shoes for every
    /// member of the run on this field - each one only theirs to see and take - and breaks
    /// into twenty Slimes where it died. The owner, 2026-09-23. The Passes come from the ordinary
    /// drop table, which carries them at 100% (`data/drops.txt`).
    pub(super) fn party_quest_kill(&mut self, key: crate::fields::FieldKey, template: u32, died_at: Option<(i16, i16)>) -> Vec<Reply> {
        if template != firsttime::KING_SLIME || key.map != firsttime::STAGE_5 || !key.is_instanced() {
            return Vec::new();
        }
        let Some(me) = self.claimed_character().map(|c| c.id) else { return Vec::new() };
        let run = self.fields.runs().instance_of(me);
        let Some(run) = run else { return Vec::new() };
        let Some((x, y)) = died_at.or(self.last_position) else { return Vec::new() };
        let mut out = Vec::new();

        // The shoes. One each, owned by and shown to that member alone.
        let here = self.bus().characters_on(key, &run.members);
        let n = here.len() as i16;
        for (i, &member) in here.iter().enumerate() {
            let offset = (i as i16 - (n - 1) / 2) * crate::drops::DROP_STAGGER_PX;
            let landed = self.config.footholds.landing(key.map, x.saturating_add(offset), y);
            let (dx, dy) = landed.map(|l| (l.x, l.y)).unwrap_or((x, y));
            let now = self.clock_ms;
            let (_, enter) = self.fields.with_drops(key, |d| {
                d.personal_drop_from_mob(crate::drops::DropFromMob {
                    map_id: key,
                    owner_id: member,
                    item: store::Item::equip(firsttime::SLIME_SHOES),
                    inv_type: store::InventoryType::Equip,
                    meso: 0,
                    x: dx,
                    y: dy,
                    source_x: x,
                    source_y: y,
                    now_ms: now,
                    party_id: 0,
                    from_mob: true,
                })
            });
            if member == me {
                out.push(enter);
            } else {
                self.bus().publish_to_character(member, key, enter);
            }
        }

        // The Slimes, where it died. Summoned like a sack's mobs: never a spawn point, so a
        // kill books nothing and they do not come back.
        let landed = self.config.footholds.landing(key.map, x, y);
        let (sx, sy, fh) = match landed {
            Some(l) => (l.x, l.y, i16::try_from(l.foothold).unwrap_or(0)),
            None => (x, y, 0),
        };
        let hp = self.config.mob_templates.get(&firsttime::SLIME).map(|t| u64::from(t.max_hp)).unwrap_or(1);
        for _ in 0..firsttime::SLIMES_FROM_THE_KING {
            let live = self.fields.summon_mob(key, firsttime::SLIME, (sx, sy), fh, hp);
            let mut mob = live.as_seen();
            mob.forced_stat = self.forced_stat_for(mob.template_id);
            let spawn = Reply {
                opcode: net::mob::MOB_ENTER_FIELD,
                body: net::mob::mob_enter_field(&mob),
                what: format!("MobEnterField: a Slime out of the King Slime at ({sx}, {sy}), object id {}", mob.object_id),
            };
            self.bus().publish(self.subscriber, key, spawn.clone(), None);
            out.push(spawn);
            if self.fields.controllers().claim_one(key, mob.object_id, self.subscriber.get()) {
                out.push(Reply {
                    opcode: net::mobmove::MOB_CHANGE_CONTROLLER,
                    body: net::mobmove::mob_change_controller(&mob, net::mobmove::CONTROL_NORMAL),
                    what: format!("MobChangeController: King Slime's Slime {} to this client", mob.object_id),
                });
            }
        }
        crate::server::log(&format!(
            "   first time together: the King Slime died at ({x}, {y}) on field {key}; {} pair(s) of shoes, {} Slimes",
            here.len(),
            firsttime::SLIMES_FROM_THE_KING
        ));
        out
    }

    /// The leader's menu came back.
    pub(super) fn cloto_menu_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        if convo.path != firsttime::CLOTO_MENU_PATH {
            return None;
        }
        let reply = net::script::parse_menu_reply(body)?;
        self.conversation = None;
        let chr = self.claimed_character()?;
        let inst = self.fields.runs().instance_of(chr.id);
        let Some(inst) = inst else { return Some(Vec::new()) };
        match reply.selection {
            Some(firsttime::CLOTO_MENU_QUESTION) => Some(self.cloto_mission(&chr)),
            Some(firsttime::CLOTO_MENU_PASSES) => Some(self.cloto_hand_in(&chr, &inst)),
            _ => Some(Vec::new()),
        }
    }

    /// **A member's mission.** First visit: the intro, then (on Next) a question. Later
    /// visits: exactly the answer's number of Coupons buys a Pass; any other number repeats
    /// the question without the answer. One Pass per member per run.
    fn cloto_mission(&mut self, chr: &net::opcode::Character) -> Vec<Reply> {
        let inst = self.fields.runs().instance_of(chr.id);
        let Some(inst) = inst else { return Vec::new() };
        if inst.passed.contains(&chr.id) {
            return vec![self.cloto_say(firsttime::CLOTO_DONE, false, format!("{} already earned their Pass", chr.name))];
        }
        let Some(&(_, question)) = inst.questions.iter().find(|(c, _)| *c == chr.id) else {
            self.conversation = Some(Conversation {
                npc_template: firsttime::CLOTO,
                quest_id: None,
                path: firsttime::CLOTO_INTRO_PATH.to_string(),
                sent: 0,
                awaiting_yes_no: false,
                sent_with_next: true,
            });
            return vec![self.cloto_say(firsttime::CLOTO_STAGE1_INTRO, true, format!("stage 1 intro to {}; Next deals a question", chr.name))];
        };
        let answer = firsttime::QUESTIONS[question].1;
        let held = self.held(chr.id, firsttime::COUPON);
        if held != answer {
            crate::server::log(&format!(
                "   first time together: {} ({}) brought {held} coupon(s) for question {question} (answer {answer}); not the number",
                chr.name, chr.id
            ));
            return vec![self.cloto_say(&firsttime::cloto_wrong(question), false, format!("{} held {held}, needs {answer}", chr.name))];
        }
        // The Pass first, then the Coupons: a full bag must cost them nothing.
        let (line, mut out) = match self.give_item(firsttime::PASS, 1, "Cloto: a Pass for a mission completed") {
            Ok(given) => given,
            Err(why) => {
                crate::server::log(&format!("   first time together: {} ({}) earned a Pass and could not take it: {why}", chr.name, chr.id));
                return vec![self.cloto_say(firsttime::CLOTO_BAG_FULL, false, format!("{}'s bag is full", chr.name))];
            }
        };
        out.extend(self.take_items(chr.id, store::InventoryType::Etc, firsttime::COUPON, answer));
        let _ = self.fields.runs().mark_passed(chr.id);
        crate::server::log(&format!(
            "   first time together: {} ({}) answered question {question} with {answer} coupon(s) - {line}",
            chr.name, chr.id
        ));
        out.push(self.cloto_say(firsttime::CLOTO_RIGHT, false, format!("{} earned a Pass", chr.name)));
        out
    }

    /// The intro's Next: deal the question (or repeat the one already dealt).
    pub(super) fn cloto_intro_answer(&mut self, action: i8) -> Vec<Reply> {
        self.conversation = None;
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        if action != net::script::SCRIPT_ACTION_YES {
            return Vec::new();
        }
        let roll = self.rng.next();
        let dealt = self.fields.runs().deal_question(chr.id, roll);
        let Some(question) = dealt else { return Vec::new() };
        crate::server::log(&format!(
            "   first time together: {} ({}) was dealt question {question}: {:?} (answer {})",
            chr.name, chr.id, firsttime::QUESTIONS[question].0, firsttime::QUESTIONS[question].1
        ));
        vec![self.cloto_say(&firsttime::cloto_question(question), false, format!("question {question} to {}", chr.name))]
    }

    /// **The leader hands in the Passes.** Everyone in the run must be on the stage first,
    /// and that is checked before a Pass is taken, so a refusal costs nothing.
    fn cloto_hand_in(&mut self, chr: &net::opcode::Character, inst: &firsttime::Instance) -> Vec<Reply> {
        let required = firsttime::passes_required(inst.members.len());
        if let Some(wait) = self.cloto_waiting_for(chr, inst) {
            return vec![wait];
        }
        let held = self.held(chr.id, firsttime::PASS);
        if held < required {
            return vec![self.cloto_say(&firsttime::cloto_short(required, held), false, format!("{} holds {held} of {required} Passes", chr.name))];
        }
        let mut out = self.take_items(chr.id, store::InventoryType::Etc, firsttime::PASS, required);
        out.extend(self.cloto_clear(chr, inst, &format!("{required} Pass(es) handed in by the leader")));
        out
    }

    /// How many of `item` the character holds, across every stack.
    fn held(&self, chr_id: u32, item: u32) -> u32 {
        self.store
            .bag_items(chr_id, store::InventoryType::Etc)
            .unwrap_or_default()
            .iter()
            .filter(|r| r.item.item_id == item)
            .map(|r| u32::from(r.item.kind.quantity()))
            .sum()
    }

    /// **Everyone in the run on this stage?** The owner, 2026-09-23: *"Do not clear a stage unless
    /// everyone is on same map that the stage is about to be cleared of."* "On this stage" is
    /// this run's FIELD - `(map, instance)` - on the presence table, which is the same thing
    /// that decides who can see whom. A member who has disconnected is no longer in the run
    /// (`leave_party_quest_on_disconnect`), so they cannot hold it up. `Some` is Cloto's
    /// refusal naming who is missing.
    fn cloto_waiting_for(&self, chr: &net::opcode::Character, inst: &firsttime::Instance) -> Option<Reply> {
        let key = self.field_of(chr);
        let here: std::collections::HashSet<u32> = self.bus().characters_on(key, &inst.members).into_iter().collect();
        let missing: Vec<String> = inst
            .members
            .iter()
            .filter(|m| !here.contains(m))
            .map(|&m| self.store.character_brief(m).ok().flatten().map(|b| b.name).unwrap_or_else(|| format!("character {m}")))
            .collect();
        if missing.is_empty() {
            return None;
        }
        crate::server::log(&format!(
            "   first time together: {} ({}) asked Cloto to clear map {} for instance {}; refused, not here: {}",
            chr.name, chr.id, chr.map_id, inst.id, missing.join(", ")
        ));
        Some(self.cloto_say(
            &firsttime::cloto_waiting(&missing),
            false,
            format!("map {} NOT cleared, waiting for {}", chr.map_id, missing.join(", ")),
        ))
    }

    /// **Clear the stage for this run**: the three effects to this screen and to this run's
    /// field on the bus - `(map, instance)`, so another party on the same stage gets nothing
    /// and its gate stays shut - and the clear recorded on the run. Checks that everyone is
    /// here first, so every caller gets that rule.
    fn cloto_clear(&mut self, chr: &net::opcode::Character, inst: &firsttime::Instance, how: &str) -> Vec<Reply> {
        if let Some(wait) = self.cloto_waiting_for(chr, inst) {
            return vec![wait];
        }
        let fresh = self.fields.runs().clear_stage(chr.id, chr.map_id);
        if fresh != Some(true) {
            return vec![self.cloto_say(firsttime::CLOTO_ALREADY, false, format!("map {} already cleared", chr.map_id))];
        }
        let key = self.field_of(chr);
        let run = inst.id;
        crate::server::log(&format!(
            "   first time together: {} ({}) cleared map {} for instance {run} ({how}); effects to field {key} only",
            chr.name, chr.id, chr.map_id
        ));
        let mut out = self.party_quest_effects(
            key,
            &[
                (net::fieldeffect::screen(net::fieldeffect::SCREEN_PARTY_CLEAR), "screen quest/party/clear"),
                (net::fieldeffect::sound(net::fieldeffect::SOUND_PARTY_CLEAR, 100), "sound Party1/Clear"),
                (net::fieldeffect::object_state(net::fieldeffect::OBJECT_GATE), "object state gate - the portal opens"),
            ],
            &format!("stage {} cleared, instance {run}", chr.map_id),
        );
        out.push(self.cloto_say(firsttime::CLOTO_CLEARED, false, format!("map {} cleared for instance {run}", chr.map_id)));
        out
    }

    /// **A login never lands on a stage.** The owner, 2026-09-23: *"If anyone disconnects from the
    /// party quest mid-session, they should log-in onto the Exit map to be returned to Kerning
    /// City. Disconnected players should never be logged back into to any of the PQ stages."*
    ///
    /// Called on the login `SetField` path, before the record is read for it, so the saved
    /// map is rewritten first and the `SetField` carries the Exit. That path is also the
    /// arrival of a channel change, and the rule holds there too: runs belong to a channel
    /// (`Fields::runs`), so a stage on another channel is a stage with no run behind it.
    ///
    /// It keys on the **saved map, not on the run**, deliberately. A server restart or a
    /// crash forgets every run, and the saved map is the only thing that survives - which is
    /// exactly the case where a check on the run would find nothing and let them in.
    pub(super) fn keep_out_of_party_quest_on_login(&mut self) {
        let Some(chr) = self.claimed_character() else { return };
        if !firsttime::is_quest_map(chr.map_id) || chr.map_id == firsttime::EXIT_MAP {
            return;
        }
        // Its replies are for this session, which is mid-login and has no field yet; the
        // members left behind hear through the bus, which is what matters.
        let _ = self.drop_from_run(chr.id, "logged in on a stage");
        let result = self.store.set_character_map(chr.id, firsttime::EXIT_MAP);
        crate::server::log(&format!(
            "   first time together: {} ({}) logged in on stage map {}; sent to the Exit {} instead - {}",
            chr.name,
            chr.id,
            chr.map_id,
            firsttime::EXIT_MAP,
            match result {
                Ok(()) => "saved".to_string(),
                Err(e) => format!("THE SAVE FAILED ({e}), so this login lands where it was saved"),
            }
        ));
    }

    /// **A dropped connection leaves its run.** Without this the rest of the party could
    /// never clear another stage - Cloto waits for everyone in the run, and someone who is
    /// not connected can never arrive. Also on a channel change: the run is this channel's.
    pub(super) fn leave_party_quest_on_disconnect(&mut self) {
        let Some(chr) = self.claimed_character() else { return };
        let run = self.fields.runs().instance_of(chr.id);
        let Some(run) = run else { return };
        crate::server::log(&format!(
            "   first time together: {} ({}) disconnected on map {}; out of instance {} - they log in on the Exit",
            chr.name, chr.id, chr.map_id, run.id
        ));
        // A partner left alone is sent out through the bus. This session is closing, so its
        // own replies have nowhere to go - and the leaver cannot be among those left.
        let _ = self.drop_from_run(chr.id, "disconnected");
    }

    /// **A cleared stage's gate, for whoever arrives after the clear.** The gate object is
    /// part of the map and starts closed on every field entry, so a member who walks in
    /// later - or comes back through Nella's side of things - would see it shut over a portal
    /// that works. Sent once per entry: the object-state arm advances the object, so it must
    /// never be sent twice to one screen (`net::fieldeffect`).
    pub(super) fn party_quest_gate(&mut self) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let cleared = self.fields.runs().is_cleared(chr.id, chr.map_id);
        if !cleared {
            return Vec::new();
        }
        vec![Reply {
            opcode: net::fieldeffect::FIELD_EFFECT,
            body: net::fieldeffect::object_state(net::fieldeffect::OBJECT_GATE),
            what: format!(
                "FieldEffect object state gate to character {}: map {} was already cleared by this run",
                chr.id, chr.map_id
            ),
        }]
    }

    /// **`next00` - `PQ_01_nextstage_portal`.** Open when this run has cleared the stage,
    /// closed otherwise; `None` for any other portal or a character in no run, so the
    /// ordinary script-portal path answers those.
    ///
    /// One member at a time, the way the classic quest works: each player walks through
    /// themselves. The arrival is the next stage's `st00`, which is what each `next00` names
    /// as its `tn` [L].
    pub(super) fn party_quest_portal(&mut self, portal: &str) -> Option<Vec<Reply>> {
        let mut chr = self.claimed_character()?;
        if portal != firsttime::NEXT_PORTAL {
            return None;
        }
        let run = self.fields.runs().instance_of(chr.id);
        let run = run?;
        let next = firsttime::next_stage(chr.map_id)?;
        if !run.cleared.contains(&chr.map_id) {
            crate::server::log(&format!(
                "   first time together: {} ({}) pressed next00 on map {} before this run cleared it",
                chr.name, chr.id, chr.map_id
            ));
            let mut out = crate::mesodrop::unlock_unhandled_latching_request(
                net::portalscript::CLIENT_PORTAL_SCRIPT,
            );
            out.extend(self.notice(firsttime::PORTAL_CLOSED.to_string()));
            return Some(out);
        }
        let arrival = self
            .config
            .portal_index
            .get(&(next, firsttime::ARRIVAL_PORTAL.to_string()))
            .copied()
            .unwrap_or(0);
        let from = chr.map_id;
        Some(self.go_to_map(
            &mut chr,
            next,
            arrival,
            format!("First Time Together: next00 from cleared stage {from}"),
        ))
    }
}
