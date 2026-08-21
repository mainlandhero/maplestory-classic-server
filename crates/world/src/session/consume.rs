//! Using a consumable: `0x010E` in, HP/MP and a smaller stack out.
//!
//! The owner, 2026-08-21: *"Using a consumable item such as Roger's Apple does not recover HP. I
//! tried to consume Red Potion, but it did not recover 100 HP. (Or less if it will fill my
//! HP bar up to full)"* — the parenthesis is the cap, and it is implemented.
//!
//! **Every path answers.** There is exactly one `0x010E` in the whole capture even though
//! The owner used more than one item, which is the signature of a client-side request latch —
//! the same one `0x0107` and both ability-point requests have. A refusal that sends nothing
//! would cost not one potion but every later use for the rest of the session.

use super::*;

impl Session {
    /// `0x010E` — the player double-clicked something in the Use tab.
    pub(super) fn on_use_item(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Some(req) = net::useitem::parse_use_item(payload) else {
            return self.use_refused("the 0x010E body did not parse".to_string());
        };
        let Some(mut chr) = self.claimed_character() else {
            return self.use_refused("no character is claimed on this connection".to_string());
        };
        let Ok(slot) = u16::try_from(req.slot) else {
            return self.use_refused(format!("slot {} is not a bag slot", req.slot));
        };

        // **The bag is the server's, so the slot is checked rather than trusted.** The
        // client names the item it believes is there; if the two disagree the server's copy
        // wins, because it is the one that persisted.
        let inv = store::InventoryType::Use;
        let held = self
            .store
            .bag_items(chr.id, inv)
            .ok()
            .and_then(|rows| rows.into_iter().find(|i| i.slot == slot));
        let Some(row) = held else {
            return self.use_refused(format!("Use slot {slot} is empty"));
        };
        if row.item.item_id != req.item_id {
            return self.use_refused(format!(
                "Use slot {slot} holds {} and the client asked to use {}",
                row.item.item_id, req.item_id
            ));
        }
        let Some(restores) = self.config.consumables.get(req.item_id) else {
            // A real item that simply is not a potion - a scroll, a summoning sack, a return
            // scroll. Nothing here knows what those do, and pretending otherwise would take
            // the item away for no effect.
            return self.use_refused(format!(
                "item {} restores nothing this server knows about",
                req.item_id
            ));
        };
        if restores.is_nothing() {
            return self.use_refused(format!("item {} restores nothing", req.item_id));
        }

        // **Cap at what is actually missing.** The owner asked for this in the same sentence, and
        // it is also the only reading that cannot put a number above the maximum into the HP
        // field - which the client would then draw as a bar past its own end.
        let hp_gain = restores.hp_for(chr.max_hp).min(chr.max_hp.saturating_sub(chr.hp));
        let mp_gain = restores.mp_for(chr.max_mp).min(chr.max_mp.saturating_sub(chr.mp));
        chr.hp = chr.hp.saturating_add(hp_gain).min(chr.max_hp);
        chr.mp = chr.mp.saturating_add(mp_gain).min(chr.max_mp);

        // The item is consumed whether or not it healed anything: drinking a potion at full
        // health still drinks the potion, and refusing here would let a player at full HP
        // hold an infinite stack. That is a rule rather than an observation, and it is the
        // same one every version of this game has.
        // What is left AFTER this one is drunk. Taken from the row that was just read, not
        // re-queried: `remove_item` returns what it took, not what remains.
        let left = row.item.kind.quantity().saturating_sub(1);
        if let Err(e) = self.store.remove_item(chr.id, inv, slot, Some(1)) {
            return self.use_refused(format!("could not consume the item: {e}"));
        }
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.use_refused(format!("could not save your health: {e}"));
        }

        let mut out = vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange {
                hp: Some(chr.hp),
                mp: Some(chr.mp),
                ..Default::default()
            }
            .build(),
            what: format!(
                "StatChanged: used item {} from Use slot {slot} - +{hp_gain} hp (now {}/{}), +{mp_gain} mp (now {}/{}). Byte 0 also clears the client's request latch",
                req.item_id, chr.hp, chr.max_hp, chr.mp, chr.max_mp
            ),
        }];
        out.extend(self.stack_change_replies(inv, slot, left));
        // **Eating the thing can be the turn-in.** The owner, 2026-08-21: *"Once the user
        // consumes the apple, the quest would be completed."*
        out.extend(self.quests_completed_by_consuming(req.item_id));
        out
    }


    /// Finish any started quest whose completion is *consuming* this item.
    ///
    /// `Check.<state>.consumeitem` is an authored key, and it states a rule the WZ only
    /// implies: quest 1002's `Check.1.item.0` carries an **id and no count** while 214 other
    /// quests carry one, and `research/quest-scripts.md` marked "no count means must-not-hold"
    /// **[I]** and left it open. The owner's sentence settles it.
    ///
    /// **Only a quest that is actually started counts.** Eating an apple you were never asked
    /// for finishes nothing, and eating one twice cannot finish it twice - `complete_quest`
    /// returns `Ok(None)` for a quest with no in-progress row and
    /// [`Session::record_quest_complete`] already declines to play a fanfare for that.
    fn quests_completed_by_consuming(&mut self, item_id: u32) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let Ok(rows) = self.store.quest_rows(chr.id) else { return Vec::new() };
        let finished: Vec<u32> = rows
            .iter()
            // **In progress only.** `quest_rows` returns completed rows too, so without this
            // a second apple would re-complete a quest that is already finished - and play
            // its fanfare again.
            .filter(|r| r.state == store::QuestState::InProgress)
            .map(|r| r.quest_id)
            .filter(|id| {
                self.config
                    .quests
                    .get(id)
                    .and_then(|q| q.complete_on_consume)
                    .is_some_and(|want| want == item_id)
            })
            .collect();
        let mut out = Vec::new();
        for quest_id in finished {
            let next = self
                .config
                .quests
                .get(&quest_id)
                .and_then(|q| q.next_quest)
                .unwrap_or(quest_id);
            out.extend(self.record_quest_complete(quest_id, next));
        }
        out
    }

    /// Tell the client what the Use tab looks like now: one fewer, or the slot emptied.
    fn stack_change_replies(
        &self,
        inv: store::InventoryType,
        slot: u16,
        left: u16,
    ) -> Vec<Reply> {
        if left == 0 {
            return vec![Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: net::inventory::inventory_removed(inv.as_u8() as i8, slot as i16),
                what: format!("InventoryOperation REMOVE: {inv:?} slot {slot} is empty now"),
            }];
        }
        vec![Reply {
            opcode: net::inventory::INVENTORY_OPERATION,
            body: net::inventory::inventory_quantity(inv.as_u8() as i8, slot as i16, left),
            what: format!("InventoryOperation QUANTITY: {inv:?} slot {slot} down to {left}"),
        }]
    }

    /// A refusal that still answers, and says why in the log.
    ///
    /// The notice goes to the chat window so the refusal is visible on screen rather than
    /// only in `world.log` - a use that silently does nothing is indistinguishable from a
    /// handler that never ran, and telling those apart has cost launches here.
    fn use_refused(&self, why: String) -> Vec<Reply> {
        let mut out = self.notice(format!("That item could not be used: {why}."));
        out.push(Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange::default().build(),
            what: format!("StatChanged: EMPTY - 0x010E refused ({why}). Sent anyway because byte 0 clears the client's request latch, and a silent refusal is what kills the next use"),
        });
        out
    }
}
