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
