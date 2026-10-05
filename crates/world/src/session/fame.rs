//! **The fame arrows on another player's Character Info** - `0x0144` in, `0x0087` out.
//!
//! The owner, 2026-09-18: *"Both the fame and the defame functionality should be available once
//! per character. A single character is allowed to fame another character once per day
//! (reset at midnight UTC), they are not allowed to fame the same character twice in a week,
//! resets on Monday midnight UTC."*
//!
//! The rules and the row live in `store::fame`; this file turns its outcome into the client's
//! own messages (`net::fame` names them), tells the target, and moves the target's fame on
//! their own screen. Nothing here is a guess about the wire: the request is the click in the
//! log, the reply is the one handler that references every fame message string.

use super::{Reply, Session};

impl Session {
    pub(super) fn on_give_fame(&mut self, body: &[u8]) -> Vec<Reply> {
        let refused = |mode: u8, why: String| {
            crate::server::log(&format!("   fame: {why}; result mode {mode}"));
            vec![Reply {
                opcode: net::fame::GIVE_FAME_RESULT,
                body: net::fame::fame_refused(mode),
                what: format!("GiveFameResult mode {mode}: {why}."),
            }]
        };
        let Some(me) = self.claimed_character() else { return Vec::new() };
        let Some(req) = net::fame::parse_give_fame(body) else {
            return refused(net::fame::result::NO_SUCH_USER, format!("a {} byte 0x0144 body that does not parse", body.len()));
        };
        let verb = if req.raise { "raise" } else { "drop" };
        let now = store::Store::unix_now();
        let outcome = match self.store.give_fame(me.id, req.target, req.raise, now) {
            Ok(o) => o,
            Err(e) => return refused(net::fame::result::NO_SUCH_USER, format!("store refused: {e}")),
        };
        match outcome {
            store::FameOutcome::NoSuchTarget => {
                refused(net::fame::result::NO_SUCH_USER, format!("{} ({}) tried to {verb} the fame of {}, who is nobody, or themself", me.name, me.id, req.target))
            }
            store::FameOutcome::AlreadyToday => {
                refused(net::fame::result::ALREADY_TODAY, format!("{} ({}) already gave fame today (UTC); {verb} on {} refused", me.name, me.id, req.target))
            }
            store::FameOutcome::SameTargetThisWeek => refused(
                net::fame::result::SAME_TARGET_THIS_WEEK,
                format!("{} ({}) already gave {} fame this week (Monday 00:00 UTC); {verb} refused - the client's text says month", me.name, me.id, req.target),
            ),
            store::FameOutcome::Given { target_name, fame } => {
                crate::server::log(&format!(
                    "   fame: {} ({}) {verb}s the fame of {target_name} ({}) -> {fame}",
                    me.name, me.id, req.target
                ));
                // The target hears who did it, and their own stat moves - wherever they are:
                // a person-addressed packet, so it may cross a map, like a whisper.
                let told = self.deliver_anywhere(
                    req.target,
                    Reply {
                        opcode: net::fame::GIVE_FAME_RESULT,
                        body: net::fame::fame_received(&me.name, req.raise),
                        what: format!("GiveFameResult mode 5: '{}' has {verb}ped your level of fame (the target's copy).", me.name),
                    },
                );
                let _ = self.deliver_anywhere(
                    req.target,
                    Reply {
                        opcode: net::combat::STAT_CHANGED,
                        body: net::combat::stat_changed(&net::combat::StatChange {
                            fame: Some(fame as u32),
                            ..Default::default()
                        }),
                        what: format!("StatChanged: fame = {fame} - the target's own record, so their own window agrees with the giver's."),
                    },
                );
                if !told {
                    crate::server::log(&format!("   fame: {target_name} ({}) is not online anywhere this process can reach; they will see it at their next login", req.target));
                }
                vec![Reply {
                    opcode: net::fame::GIVE_FAME_RESULT,
                    body: net::fame::fame_given(&target_name, req.raise, fame),
                    what: format!(
                        "GiveFameResult mode 0: {} {verb}d '{target_name}''s level of fame; the open Character Info window shows {fame}.",
                        me.name
                    ),
                }]
            }
        }
    }
}

impl Session {
    /// **`Act.1.pop` - a quest's fame, paid on turning it in.** The owner, 2026-10-04: *"Quests
    /// that award fame currently do not give fame"* - nothing read the key. Called on a recorded
    /// completion only, beside the mesos, so a repeat click pays nothing.
    ///
    /// Not a gift (`Store::add_fame`, no `fame_log` row, the day's own gift untouched) and not
    /// scaled by any rate. Two replies: the stat, so Character Info agrees, and the client's own
    /// `You have gained fame. (+n)` - `Message` kind 5, which posts to the chat log
    /// (`net::message::fame`, [L]).
    pub(super) fn pay_quest_fame(&mut self, quest_id: u32) -> Vec<Reply> {
        let delta = self.config.quests.get(&quest_id).map_or(0, |q| q.complete_fame);
        if delta == 0 {
            return Vec::new();
        }
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let fame = match self.store.add_fame(chr.id, delta) {
            Ok(Some(f)) => f,
            Ok(None) => return Vec::new(),
            Err(e) => return self.notice(format!("Quest {quest_id}'s {delta} fame could not be given: {e}")),
        };
        vec![
            Reply {
                opcode: net::combat::STAT_CHANGED,
                body: net::combat::stat_changed(&net::combat::StatChange {
                    excl_request: true,
                    fame: Some(fame as u32),
                    ..Default::default()
                }),
                what: format!("StatChanged: fame = {fame} after quest {quest_id}'s Act.1.pop {delta:+}"),
            },
            Reply {
                opcode: net::message::MESSAGE,
                body: net::message::fame(delta),
                what: format!("Message kind 5: 'You have gained fame. (+{delta})' for quest {quest_id}, in the chat log"),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, Quest};
    use crate::fields::Fields;
    use std::sync::Arc;
    use store::Store;

    /// 10205's real `Act.1.pop`.
    const FAMOUS: u32 = 10_205;
    const NPC: u32 = 406;
    const MAP: u32 = 10_003_000;

    fn quest_request(action: u8, quest_id: u32) -> Vec<u8> {
        let mut b = net::script::CLIENT_QUEST_REQUEST.to_le_bytes().to_vec();
        b.push(action);
        b.extend_from_slice(&quest_id.to_le_bytes());
        b.extend_from_slice(&NPC.to_le_bytes());
        b.extend_from_slice(&[0u8; 8]);
        b
    }

    /// **Turning in a fame quest raises fame once**: the stat, the chat line, and nothing on a
    /// repeat click. Three effects, all three checked (the Heena lesson).
    #[test]
    fn a_quest_that_awards_fame_pays_it_once() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let mut cfg = Config::default();
        cfg.fields.insert(MAP);
        cfg.quests.insert(FAMOUS, Quest { start_npc: Some(NPC), end_npc: Some(NPC), complete_fame: 3, ..Quest::default() });
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Wisp".to_string(), map_id: MAP, level: 10, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store.clone(), Arc::new(cfg), Arc::new(Fields::new()));
        s.claim_for_character(id);

        let accepted = s.handle(&quest_request(1, FAMOUS));
        assert!(!accepted.iter().any(|r| r.body == net::message::fame(3)), "nothing on accepting");
        let out = s.handle(&quest_request(2, FAMOUS));
        assert_eq!(store.fame(id).unwrap(), Some(3));
        assert!(out.iter().any(|r| r.opcode == net::message::MESSAGE && r.body == net::message::fame(3)), "{out:?}");
        let stat = net::combat::stat_changed(&net::combat::StatChange { excl_request: true, fame: Some(3), ..Default::default() });
        assert!(out.iter().any(|r| r.opcode == net::combat::STAT_CHANGED && r.body == stat), "{out:?}");

        let again = s.handle(&quest_request(2, FAMOUS));
        assert_eq!(store.fame(id).unwrap(), Some(3), "a repeat turn-in pays nothing");
        assert!(!again.iter().any(|r| r.body == net::message::fame(3)));
    }
}
