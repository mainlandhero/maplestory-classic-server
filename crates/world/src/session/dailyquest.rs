//! **Quests that come back once a day** - Arwen and the Glass Shoe (10200).
//!
//! The owner, 2026-10-05: *"Arwen's Glass Shoes is repeatable every day, but the server should
//! not immediately offer the quest again."*
//!
//! # Why the client offered it straight away
//!
//! The client decides which quest an NPC offers from its own `Quest.wz`, before the server hears
//! anything (`tools/quest_patch.py`'s docs). 10200 carries `Check/0/interval = 0` - the only
//! quest in this client with an `interval` key at all - which the client reads as "repeatable,
//! zero minutes after the last turn-in". So the offer came back the moment the shoe was handed
//! over, and nothing the server sends can lengthen a number the client reads from its own data.
//!
//! # So the day belongs to the server
//!
//! * `tools/quest_patch.py` **deletes** `Check/0/interval` from 10200. Without it the quest is
//!   an ordinary one-time quest to the client: once complete, never offered.
//! * Here, once the UTC day the turn-in happened on is over, the completion is cleared
//!   (`store::Store::clear_daily_completion`) and the client is told the quest was never done
//!   (`net::quest::quest_forgotten(.., true)`, the same record a re-picked Community Board quest
//!   is cleared with). The offer comes back the next day and not before.
//!
//! That runs at field entry and on the first tick of each new UTC day, so a player online at
//! midnight sees it come back without changing maps. The day is UTC, the same day as every other
//! daily on this server (`store::dailyperks::today`).
//!
//! An unpatched client still offers it immediately; accepting it then gets Arwen saying to come
//! back tomorrow ([`Session::daily_quest_refusal`]) rather than silence.

use super::{Reply, Session};

/// The quests this server repeats once a UTC day. Nothing in `Quest.wz` names a daily cadence
/// for these (`research/second-job.md`'s key census: `dayByDay` 2, `interval` 1), so the list
/// is the server's.
pub(crate) const DAILY_QUESTS: [u32; 1] = [10_200];

/// What Arwen says to an accept made on the day the shoe was already returned.
pub(crate) const COME_BACK_TOMORROW: &str =
    "You already brought my glass shoe back today... Thank you. If that flaming monster takes it again, come and see me tomorrow.";

impl Session {
    /// Clear yesterday's (or older) completions of the daily quests and tell the client.
    /// Does nothing more than one day comparison until the UTC day changes.
    pub(super) fn refresh_daily_quests(&mut self) -> Vec<Reply> {
        let today = store::dailyperks::today();
        if today == self.daily_quest_day {
            return Vec::new();
        }
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        self.daily_quest_day = today;
        let day_start = store::dailyperks::utc_day_start(today);
        let mut out = Vec::new();
        for quest_id in DAILY_QUESTS {
            match self.store.clear_daily_completion(chr.id, quest_id, day_start) {
                Ok(true) => out.push(Reply {
                    opcode: net::quest::MESSAGE,
                    body: net::quest::quest_forgotten(quest_id, true),
                    what: format!(
                        "quest {quest_id}: a daily turned in before today (UTC) - its completion is cleared for {} ({}) and the client offers it again",
                        chr.name, chr.id
                    ),
                }),
                Ok(false) => {}
                Err(e) => crate::server::log(&format!(
                    "   quest {quest_id}: could not clear yesterday's completion for character {}: {e} - it stays done until the next try",
                    chr.id
                )),
            }
        }
        out
    }

    /// `Some(refusal)` when `quest_id` is a daily already turned in today; `None` otherwise.
    pub(super) fn daily_quest_refusal(&mut self, quest_id: u32, speaker: u32) -> Option<Vec<Reply>> {
        if !DAILY_QUESTS.contains(&quest_id) {
            return None;
        }
        let chr = self.claimed_character()?;
        let row = self.store.quest_row(chr.id, quest_id).ok().flatten()?;
        if row.state != store::QuestState::Complete {
            return None;
        }
        Some(self.citizenship_refusal(quest_id, speaker, COME_BACK_TOMORROW.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::fields::Fields;
    use std::sync::Arc;
    use store::Store;

    const ARWEN: u32 = 304;
    const QUEST: u32 = 10_200;

    fn session() -> (Session, Arc<Store>, u32) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut cfg = Config::default();
        cfg.fields.insert(100_000_000);
        cfg.quests.insert(QUEST, crate::config::Quest { start_npc: Some(ARWEN), end_npc: Some(ARWEN), ..Default::default() });
        let chr = net::opcode::Character { name: "Pebble".into(), map_id: 100_000_000, level: 30, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store.clone(), Arc::new(cfg), Arc::new(Fields::new()));
        s.claim_for_character(id);
        (s, store, id)
    }

    fn complete_at(store: &Store, id: u32, at: i64) {
        store
            .save_quest(id, &store::QuestRow { quest_id: QUEST, state: store::QuestState::Complete, progress: String::new(), started_at: at, completed_at: Some(at) })
            .unwrap();
    }

    fn forgotten(out: &[Reply]) -> bool {
        out.iter().any(|r| r.opcode == net::quest::MESSAGE && r.body == net::quest::quest_forgotten(QUEST, true))
    }

    /// **Done today: not offered, and an accept is answered.** Every effect: no record is
    /// sent, the row stays complete, and an accept from a client that still offers it gets
    /// Arwen's line instead of silence - and does not restart the quest.
    #[test]
    fn a_shoe_returned_today_is_not_offered_again_today() {
        let (mut s, store, id) = session();
        complete_at(&store, id, Store::unix_now());
        let out = s.on_field_entered();
        assert!(!forgotten(&out), "today's turn-in stays done");
        assert!(s.tick(1_000).iter().all(|r| r.body != net::quest::quest_forgotten(QUEST, true)));
        assert_eq!(store.quest_row(id, QUEST).unwrap().unwrap().state, store::QuestState::Complete);

        let said = s.record_quest_start(QUEST, ARWEN);
        assert!(said.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE), "answered: {said:?}");
        assert!(!said.iter().any(|r| r.body == net::quest::quest_accepted(QUEST)), "not accepted");
        assert_eq!(store.quest_row(id, QUEST).unwrap().unwrap().state, store::QuestState::Complete);
    }

    /// **Done yesterday: offered again, once.** The completion is cleared in the store and the
    /// client is told, the next field entry says nothing more, and the quest accepts afresh.
    #[test]
    fn a_shoe_returned_yesterday_is_offered_again() {
        let (mut s, store, id) = session();
        let yesterday = store::dailyperks::utc_day_start(store::dailyperks::today()) - 3_600;
        complete_at(&store, id, yesterday);
        let out = s.on_field_entered();
        assert!(forgotten(&out), "the client is told it was never done");
        assert_eq!(store.quest_row(id, QUEST).unwrap(), None);
        assert!(!forgotten(&s.on_field_entered()), "said once");
        let said = s.record_quest_start(QUEST, ARWEN);
        assert!(said.iter().any(|r| r.body == net::quest::quest_accepted(QUEST)));
        assert_eq!(store.quest_row(id, QUEST).unwrap().unwrap().state, store::QuestState::InProgress);
    }
}
