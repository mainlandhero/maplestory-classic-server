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
        // `ENTRY_MIN_PARTY`, not `MIN_PARTY`: the TEMPORARY solo test (firsttime.rs).
        let gate = firsttime::check_min(firsttime::ENTRY_MIN_PARTY, chr.id, party.as_ref(), |id| {
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
        self.go_to_map(&mut chr, map, 0, why.to_string())
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
        let _ = self.fields.runs().drop_member(chr.id);
        if !firsttime::is_quest_map(chr.map_id) || chr.map_id == firsttime::EXIT_MAP {
            // Not inside, or already standing at the Exit: dropping them from the run is the
            // whole effect. Warping someone who is in Kerning City would be a bug with a
            // very confusing screen.
            return Vec::new();
        }
        crate::server::log(&format!(
            "   first time together: {} ({}) leaves for the Exit map - {why}",
            chr.name, chr.id
        ));
        self.go_to_map(&mut chr, firsttime::EXIT_MAP, 0, format!("First Time Together: {why}"))
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
        let _ = self.fields.runs().drop_member(who);
        self.bus().publish_event_to_character(
            who,
            crate::broadcast::Event::PartyQuestEnter { map: firsttime::EXIT_MAP, why: why.to_string() },
        );
        crate::server::log(&format!("   first time together: character {who} is out of the quest - {why}"));
        Vec::new()
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
            let _ = self.fields.runs().drop_member(chr.id);
            return self.go_to_map(&mut chr, firsttime::TOWN_MAP, 0, "First Time Together: Nella sends them home".to_string());
        }
        self.leave_party_quest("Nella showed them out")
    }

    /// **Cloto clears the stage they stand on - TEMPORARY, for the instancing test.** The owner,
    /// 2026-09-23: clicking the stage NPC should *"send the "stage clear" opcode and enable
    /// the portal to go to the next stage"*, and *"the PQ stage clears should be per
    /// instance, and never shared."*
    ///
    /// The clear is recorded on this run's instance (`firsttime::Runs::clear_stage`), and the three
    /// effects go to this screen directly and to **this field key** on the bus - which is
    /// `(map, instance)`, so another party on the same stage gets nothing and its gate stays
    /// shut. `None` outside a run, so a GM who walked in with `!map` falls through to them
    /// ordinary dialogue.
    pub(super) fn open_cloto(&mut self, template: u32) -> Option<Vec<Reply>> {
        let chr = self.claimed_character()?;
        if template != firsttime::CLOTO {
            return None;
        }
        let fresh = self.fields.runs().clear_stage(chr.id, chr.map_id);
        let fresh = fresh?;
        self.conversation = None;
        let run = self.fields.runs().instance_of(chr.id).map(|i| i.id).unwrap_or(0);
        let say = |line: &str, what: String| Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(template, line, false, false),
            what,
        };
        if !fresh {
            return Some(vec![say(
                firsttime::CLOTO_ALREADY,
                format!("ScriptMessage Say from Cloto: map {} already cleared by instance {run}", chr.map_id),
            )]);
        }
        let key = self.field_of(&chr);
        crate::server::log(&format!(
            "   first time together: {} ({}) cleared map {} for instance {run} (TEMPORARY: Cloto clears on click); \
             effects to field {key} only",
            chr.name, chr.id, chr.map_id
        ));
        let effects = [
            (net::fieldeffect::screen(net::fieldeffect::SCREEN_PARTY_CLEAR), "screen quest/party/clear"),
            (net::fieldeffect::sound(net::fieldeffect::SOUND_PARTY_CLEAR, 100), "sound Party1/Clear"),
            (net::fieldeffect::object_state(net::fieldeffect::OBJECT_GATE), "object state gate - the portal opens"),
        ];
        let mut out = Vec::new();
        for (body, what) in effects {
            let reply = Reply {
                opcode: net::fieldeffect::FIELD_EFFECT,
                body,
                what: format!("FieldEffect {what}: stage {} cleared, instance {run}", chr.map_id),
            };
            self.bus().publish(self.subscriber, key, reply.clone(), None);
            out.push(reply);
        }
        out.push(say(
            firsttime::CLOTO_CLEARED,
            format!("ScriptMessage Say from Cloto: map {} cleared for instance {run}", chr.map_id),
        ));
        Some(out)
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
