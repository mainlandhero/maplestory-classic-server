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
        let store = self.store.clone();
        let gate = firsttime::check(chr.id, party.as_ref(), |id| {
            store.character_brief(id).ok().flatten().map(|b| firsttime::Candidate {
                character: id,
                name: b.name,
                level: u16::try_from(b.level).unwrap_or(u16::MAX),
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
        let instance = firsttime::open(party.id, members.iter().map(|m| m.character).collect());
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
}
