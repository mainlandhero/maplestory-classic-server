//! **A quest's mesos, both ways** - `Act.0.money` on accepting, `Act.1.money` on turning in.
//!
//! A positive amount is a payout at the Quest rate (the owner, 2026-09-28). A negative one is a
//! **cost**, taken at face value - the owner, 2026-10-02: *"Quests that take away money should
//! properly take away mesos and complain if the player does not have enough mesos"*. The client
//! ships one: quest 10303, Nella's 1,000-meso commission on accept.
//!
//! # The cost is taken BEFORE the quest moves, and given back if it then does not
//!
//! [`Session::take_quest_cost`] charges first, through `Store::add_mesos`, which reads and
//! writes in one transaction and refuses to go below zero - so the purse check and the charge
//! are one step and cannot disagree. A short purse is refused with the NPC's own box (parked
//! under `questroom::REFUSAL_PATH`, like a full bag) and **nothing is written**: the quest stays
//! where it was. If the quest then fails to move after all - a database error, a repeat click
//! that `start_quest`/`complete_quest` turned away - [`Session::refund_quest_cost`] hands it
//! back. The lines on screen ([`Session::quest_cost_replies`]) go out only once the quest has
//! actually moved, so a refund never needs a second line to take one back.

use super::{Conversation, Reply, Session};

impl Session {
    /// `Act.<state>.money`, signed: 0 = on accepting, 1 = on turning in.
    fn quest_money(&self, quest_id: u32, state: u8) -> i32 {
        self.config.quests.get(&quest_id).map_or(0, |q| if state == 0 { q.start_money } else { q.complete_money })
    }

    /// Charge a quest's cost, if it has one. `Ok(mesos taken)` (0 for a quest that costs
    /// nothing); `Err(the refusal)` when the purse is short or the store fails - nothing taken.
    /// `at` is `"accepted"` or `"completed"`, for the words and the log.
    pub(super) fn take_quest_cost(&mut self, quest_id: u32, state: u8, speaker: u32, at: &str) -> Result<u32, Vec<Reply>> {
        let money = self.quest_money(quest_id, state);
        if money >= 0 {
            return Ok(0);
        }
        let cost = money.unsigned_abs();
        let Some(chr) = self.claimed_character() else { return Ok(0) };
        match self.store.add_mesos(chr.id, -i64::from(cost)) {
            Ok(_) => Ok(cost),
            Err(store::StoreError::NotEnoughMesos { have, .. }) => {
                let verb = if state == 0 { "accept" } else { "complete" };
                Err(self.quest_money_refusal(
                    quest_id,
                    speaker,
                    format!("You need #b{cost} mesos#k to {verb} this quest, and you only have #b{have}#k. Come back when you have enough."),
                    &format!("NOT {at} - it costs {cost} mesos and character {} has {have}. Nothing written, nothing taken", chr.id),
                ))
            }
            Err(e) => Err(self.notice(format!("Quest {quest_id}'s cost of {cost} mesos could not be taken: {e}"))),
        }
    }

    /// Give back a cost [`Session::take_quest_cost`] took, because the quest did not move.
    pub(super) fn refund_quest_cost(&mut self, quest_id: u32, charged: u32) {
        if charged == 0 {
            return;
        }
        let Some(chr) = self.claimed_character() else { return };
        let outcome = match self.store.add_mesos(chr.id, i64::from(charged)) {
            Ok(balance) => format!("refunded, balance {balance}"),
            Err(e) => format!("REFUND FAILED: {e}"),
        };
        crate::server::log(&format!(
            "   quest {quest_id}: did not move after its {charged}-meso cost was taken - {outcome}"
        ));
    }

    /// The balance and `You have lost mesos (-n)` for a cost that was taken, once the quest
    /// has moved. Nothing for 0.
    pub(super) fn quest_cost_replies(&mut self, quest_id: u32, charged: u32) -> Vec<Reply> {
        if charged == 0 {
            return Vec::new();
        }
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let mut out = self.meso_reply(chr.id);
        out.push(Reply {
            opcode: net::message::MESSAGE,
            body: net::message::meso_lost_line(charged),
            what: format!("Message: quest {quest_id} took {charged} mesos, as a grey chat line"),
        });
        out
    }

    /// A positive `Act.<state>.money`, at the Quest rate - the same multiplier quest EXP gets.
    /// **Every quest**: the owner, 2026-09-28, first for the Community Board and then *"for all
    /// quests"*. Called on a recorded transition only, so a repeat click pays nothing.
    pub(super) fn pay_quest_mesos(&mut self, quest_id: u32, state: u8) -> Vec<Reply> {
        let money = self.quest_money(quest_id, state);
        if money <= 0 {
            return Vec::new();
        }
        let base = money.unsigned_abs();
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let rate = self.rate(store::rates::RateKind::Quest);
        let paid = u32::try_from(rate.apply(u64::from(base))).unwrap_or(u32::MAX).min(i32::MAX as u32);
        if let Err(e) = self.store.add_mesos(chr.id, i64::from(paid)) {
            return self.notice(format!("Quest {quest_id}'s {paid} mesos could not be paid: {e}"));
        }
        let mut out = self.meso_reply(chr.id);
        // In the chat box, not the pick-up area (the owner, 2026-10-02).
        out.push(Reply {
            opcode: net::message::MESSAGE,
            body: net::message::meso_gained_line(paid),
            what: format!("Message: quest {quest_id} paid {paid} mesos ({base} at {rate}x), as a grey chat line"),
        });
        out
    }

    /// The NPC says the purse is short, and the quest stays where it was - the same box a full
    /// bag gets, parked under `questroom::REFUSAL_PATH` so the closing line is skipped and the
    /// OK is answered silently. No NPC on record: the words go to chat instead.
    fn quest_money_refusal(&mut self, quest_id: u32, speaker: u32, text: String, why: &str) -> Vec<Reply> {
        crate::server::log(&format!("   quest {quest_id}: {why}"));
        if speaker == 0 {
            return self.notice(text);
        }
        self.conversation = Some(Conversation {
            npc_template: speaker,
            quest_id: None,
            path: crate::questroom::REFUSAL_PATH.to_string(),
            sent: 0,
            awaiting_yes_no: false,
            sent_with_next: false,
        });
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(speaker, &text, false, false),
            what: format!("ScriptMessage Say from NPC template {speaker}: quest {quest_id} {why}"),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, Quest};
    use crate::fields::Fields;
    use std::sync::Arc;
    use store::Store;

    /// Nella's commission: 1,000 to accept (10303's real `Act.0.money`), 292 back on turn-in.
    const COMMISSION: u32 = 10_303;
    /// A quest that costs 500 to turn in.
    const TOLL: u32 = 990_001;
    const NELLA: u32 = 406;
    const MAP: u32 = 10_003_000;

    fn session(mesos: u32) -> (Arc<Store>, Session, u32) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let mut cfg = Config::default();
        cfg.fields.insert(MAP);
        cfg.quests.insert(COMMISSION, Quest { start_npc: Some(NELLA), end_npc: Some(NELLA), start_money: -1_000, complete_money: 292, ..Quest::default() });
        cfg.quests.insert(TOLL, Quest { start_npc: Some(NELLA), end_npc: Some(NELLA), complete_money: -500, ..Quest::default() });
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Wisp".to_string(), map_id: MAP, level: 10, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.set_mesos(id, mesos).unwrap();
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store.clone(), Arc::new(cfg), Arc::new(Fields::new()));
        s.claim_for_character(id);
        (store, s, id)
    }

    fn quest_request(action: u8, quest_id: u32) -> Vec<u8> {
        let mut b = net::script::CLIENT_QUEST_REQUEST.to_le_bytes().to_vec();
        b.push(action);
        b.extend_from_slice(&quest_id.to_le_bytes());
        b.extend_from_slice(&NELLA.to_le_bytes());
        b.extend_from_slice(&[0u8; 8]);
        b
    }

    fn state(store: &Store, id: u32, quest: u32) -> Option<store::QuestState> {
        store.quest_row(id, quest).unwrap().map(|r| r.state)
    }

    fn says(out: &[Reply], words: &str) -> bool {
        out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE && r.what.contains(words))
    }

    /// **Short of the commission: Nella says so, the quest is not accepted, nothing is taken.**
    #[test]
    fn a_quest_that_costs_mesos_to_accept_is_refused_when_the_purse_is_short() {
        let (store, mut s, id) = session(999);
        let out = s.handle(&quest_request(1, COMMISSION));
        assert!(says(&out, "costs 1000 mesos and character"), "{out:?}");
        let say = out.iter().find(|r| r.opcode == net::script::SCRIPT_MESSAGE).unwrap();
        assert!(String::from_utf8_lossy(&say.body).contains("You need #b1000 mesos#k to accept this quest, and you only have #b999#k"));
        assert_eq!(state(&store, id, COMMISSION), None, "not accepted");
        assert_eq!(store.mesos(id).unwrap(), 999, "nothing taken");
        assert!(!out.iter().any(|r| r.opcode == net::quest::MESSAGE), "no quest record went out: {out:?}");
        assert_eq!(s.conversation.as_ref().map(|c| c.path.as_str()), Some(crate::questroom::REFUSAL_PATH));
    }

    /// **Enough: the 1,000 is taken once, said in the chat box, and a second Accept takes
    /// nothing.** The turn-in then pays its 292 at the (default 1x) rate.
    #[test]
    fn a_quest_that_costs_mesos_to_accept_takes_them_once() {
        let (store, mut s, id) = session(1_500);
        let out = s.handle(&quest_request(1, COMMISSION));
        assert_eq!(state(&store, id, COMMISSION), Some(store::QuestState::InProgress));
        assert_eq!(store.mesos(id).unwrap(), 500);
        assert!(out.iter().any(|r| r.body == net::message::meso_lost_line(1_000)), "{out:?}");
        let again = s.handle(&quest_request(1, COMMISSION));
        assert_eq!(store.mesos(id).unwrap(), 500, "a repeat accept costs nothing");
        assert!(!again.iter().any(|r| r.body == net::message::meso_lost_line(1_000)));

        let out = s.handle(&quest_request(2, COMMISSION));
        assert_eq!(state(&store, id, COMMISSION), Some(store::QuestState::Complete));
        assert_eq!(store.mesos(id).unwrap(), 792);
        assert!(out.iter().any(|r| r.body == net::message::meso_gained_line(292)), "{out:?}");
    }

    /// **A turn-in that takes part of a stack tells the window the new count, not "removed".**
    /// The owner, 2026-10-03: handing in an Etc quest cleared the whole stack from the window
    /// while the rest stayed in the bag, until a map change. Two stacks, 5 and 20, and a quest
    /// that wants 10: the first goes (REMOVE), the second shows 15 (QUANTITY) - and so does the bag.
    #[test]
    fn a_turn_in_that_takes_part_of_a_stack_shows_what_is_left() {
        const SHELL: u32 = 4_000_000;
        const SHELLS: u32 = 990_002;
        let (store, mut s, id) = session(0);
        let mut cfg = (*s.config).clone();
        cfg.quests.insert(SHELLS, Quest {
            start_npc: Some(NELLA),
            end_npc: Some(NELLA),
            complete_rewards: vec![crate::config::RewardItem { id: SHELL, count: -10, prop: 0, gender: None }],
            ..Quest::default()
        });
        s.config = Arc::new(cfg);
        let etc = store::InventoryType::Etc;
        store.set_inventory_slot(id, etc, 1, &store::Item::bundle(SHELL, 5)).unwrap();
        store.set_inventory_slot(id, etc, 2, &store::Item::bundle(SHELL, 20)).unwrap();
        let _ = s.handle(&quest_request(1, SHELLS));
        let out = s.handle(&quest_request(2, SHELLS));
        let ops: Vec<&Vec<u8>> = out.iter().filter(|r| r.opcode == net::inventory::INVENTORY_OPERATION).map(|r| &r.body).collect();
        assert_eq!(ops, vec![&net::inventory::inventory_removed(4, 1), &net::inventory::inventory_quantity(4, 2, 15)], "{out:?}");
        let left: Vec<(u16, u16)> = store.bag_items(id, etc).unwrap().iter().map(|r| (r.slot, r.item.kind.quantity())).collect();
        assert_eq!(left, vec![(2, 15)]);
    }

    /// **A turn-in that costs mesos: refused while short, the quest stays in progress; paid
    /// once when the purse covers it.**
    #[test]
    fn a_quest_that_costs_mesos_to_turn_in_is_refused_when_short_and_charged_once_when_not() {
        let (store, mut s, id) = session(499);
        let _ = s.handle(&quest_request(1, TOLL));
        let out = s.handle(&quest_request(2, TOLL));
        let say = out.iter().find(|r| r.opcode == net::script::SCRIPT_MESSAGE).expect("Nella says so");
        assert!(String::from_utf8_lossy(&say.body).contains("You need #b500 mesos#k to complete this quest"));
        assert_eq!(state(&store, id, TOLL), Some(store::QuestState::InProgress), "still in progress");
        assert_eq!(store.mesos(id).unwrap(), 499);

        store.set_mesos(id, 600).unwrap();
        s.conversation = None;
        let out = s.handle(&quest_request(2, TOLL));
        assert_eq!(state(&store, id, TOLL), Some(store::QuestState::Complete));
        assert_eq!(store.mesos(id).unwrap(), 100);
        assert!(out.iter().any(|r| r.body == net::message::meso_lost_line(500)), "{out:?}");
        let _ = s.handle(&quest_request(2, TOLL));
        assert_eq!(store.mesos(id).unwrap(), 100, "a repeat turn-in takes nothing");
    }
}
