//! `!scroll` - the join between [`crate::scrolls`] (the rules), [`crate::scrollnpc`] (the
//! wording) and the store.
//!
//! Three steps, each a separate reply from the client: pick a scroll, pick a worn item,
//! confirm. `crate::scrollnpc` explains why the choice rides in the conversation path.
//!
//! # Every effect hangs off the transition
//!
//! `CLAUDE.md`'s Heena rule, and this handler has four effects to keep together: the scroll
//! leaves the bag, the equip's stats change, its failed-slot count changes, and the daily pass
//! is spent. [`Session::apply_scroll`] refuses **before** any of them when the rules refuse,
//! and takes the scroll out of the bag only once the write to the equip has succeeded - so
//! there is no path that eats a scroll and changes nothing, or changes an item for free.

use super::{Reply, Session};
use crate::scrollnpc as npc;
use crate::scrolls::{self, Chance, EquipBase, EquipState, Scroll};

/// The daily-perk key for one scroll type. The owner: *"per scroll type per character for the daily
/// gate"*, so the key names the scroll and the scope is the character.
fn daily_key(scroll: Scroll) -> String {
    format!("scroll.{}", scroll.item_id())
}

impl Session {
    /// `!scroll` - step 1, the pick menu.
    pub(super) fn open_scroll_picker(&mut self) -> Vec<Reply> {
        let template = crate::dailyperks::ADMIN_TEMPLATE;
        let Some(chr) = self.claimed_character() else {
            return self.admin_says(
                template,
                "I cannot find your record just now.",
                "no character is claimed on this connection",
            );
        };
        let held = self.scrolls_held(chr.id);
        if held.is_empty() {
            return self.admin_says(
                template,
                &npc::nothing_to_use(),
                "the character is carrying none of the three scrolls",
            );
        }
        self.conversation = Some(super::Conversation {
            npc_template: template,
            quest_id: None,
            path: npc::PICK_PATH.to_string(),
            sent: 0,
            awaiting_yes_no: false,
            sent_with_next: false,
        });
        let text = npc::pick_menu(&held);
        vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_menu(template, &text),
            what: format!(
                "ScriptMessage MENU opened by {} for character {}: {} scroll type(s) held",
                npc::COMMAND_TYPED,
                chr.id,
                held.len()
            ),
        }]
    }

    /// Steps 1 and 2 answered. `None` means "not mine" - fall through to the other menus.
    pub(super) fn scroll_menu_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        if !npc::is_scroll_path(&convo.path) {
            return None;
        }
        let reply = net::script::parse_menu_reply(body)?;
        let template = convo.npc_template;
        self.conversation = None;
        // Closed rather than chosen: nothing decided, nothing said. `0x00F3` does not hold the
        // one-request latch, so silence is safe here and is measured.
        let Some(selection) = reply.selection else { return Some(Vec::new()) };
        let chr = self.claimed_character()?;

        if convo.path == npc::PICK_PATH {
            let held = self.scrolls_held(chr.id);
            let Some(&(scroll, _)) = held.get(selection as usize) else {
                return Some(self.admin_says(
                    template,
                    &npc::nothing_to_use(),
                    &format!("selection {selection} names no scroll among {} held", held.len()),
                ));
            };
            let worn = self.worn_list(chr.id);
            if worn.is_empty() {
                return Some(self.admin_says(
                    template,
                    &npc::nothing_equipped(),
                    "nothing is equipped, so there is nothing to scroll",
                ));
            }
            self.conversation = Some(super::Conversation {
                npc_template: template,
                quest_id: None,
                path: npc::equip_path(scroll),
                sent: 0,
                awaiting_yes_no: false,
                sent_with_next: false,
            });
            let text = npc::equip_menu(scroll, &worn);
            return Some(vec![Reply {
                opcode: net::script::SCRIPT_MESSAGE,
                body: net::script::npc_menu(template, &text),
                what: format!(
                    "ScriptMessage MENU: character {} chose {} and is picking from {} worn item(s)",
                    chr.id,
                    scroll.name(),
                    worn.len()
                ),
            }]);
        }

        // Step 2: a worn item was picked. Confirm.
        let scroll = npc::scroll_from_equip_path(&convo.path)?;
        let worn = self.worn_list(chr.id);
        let Some((equip_slot, _, item_name, _, _)) = worn.get(selection as usize).cloned() else {
            return Some(self.admin_says(
                template,
                &npc::nothing_equipped(),
                &format!("selection {selection} names no worn item among {}", worn.len()),
            ));
        };
        // The odds line needs to know whether the free pass is still there, and this must NOT
        // claim it - the claim happens on Yes. `daily_claim_day` only reads.
        let guaranteed = self
            .store
            .daily_claim_day(store::SCOPE_CHARACTER, i64::from(chr.id), &daily_key(scroll))
            .ok()
            .flatten()
            != Some(store::today());
        self.conversation = Some(super::Conversation {
            npc_template: template,
            quest_id: None,
            path: npc::confirm_path(scroll, equip_slot),
            sent: 0,
            awaiting_yes_no: true,
            sent_with_next: false,
        });
        let text = npc::confirm(scroll, &item_name, guaranteed);
        Some(vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_ask(template, &text, false),
            what: format!(
                "ScriptMessage YES/NO: character {} confirming {} on equip slot {equip_slot} \
                 ({item_name}), guaranteed={guaranteed}",
                chr.id,
                scroll.name()
            ),
        }])
    }

    /// Step 3, the confirm. Called when the parked conversation is one of ours.
    pub(super) fn scroll_confirm_answer(&mut self, action: i8) -> Vec<Reply> {
        let convo = match self.conversation.clone() {
            Some(c) => c,
            None => return Vec::new(),
        };
        let template = convo.npc_template;
        self.conversation = None;
        let Some((scroll, equip_slot)) = npc::choice_from_confirm_path(&convo.path) else {
            return Vec::new();
        };
        // Anything that is not Yes is a No. The client sends 0 for No and 1 for Yes; a closed
        // box is neither, and all three mean "do not do it".
        if action != net::script::SCRIPT_ACTION_YES {
            return self.admin_says(template, &npc::cancelled(), "the player answered No");
        }
        self.apply_scroll(template, scroll, equip_slot)
    }

    /// The transition. Nothing is spent until the write succeeds.
    fn apply_scroll(&mut self, template: u32, scroll: Scroll, equip_slot: u8) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        // Re-read everything: the confirm box may have been on screen for a while, and the
        // player could have unequipped the item or dropped the scroll in between.
        let Some(bag_slot) = self.scroll_bag_slot(chr.id, scroll) else {
            return self.admin_says(
                template,
                &npc::nothing_to_use(),
                "the scroll is no longer in the bag",
            );
        };
        let Ok(Some((item_id, stats, failed_slots))) = self.store.worn_item(chr.id, equip_slot)
        else {
            return self.admin_says(
                template,
                &npc::nothing_equipped(),
                "that equipment slot is empty now",
            );
        };
        let base = self.equip_base(item_id);
        // No stored stats means the item has never been modified - build the template's own,
        // which is what the wire edge does for the same row. Not zeros: `None` is "derive from
        // the WZ", and treating it as zero would strip a fresh item on its first scroll.
        let current = stats.unwrap_or_else(|| net::opcode::EquipStats::fresh(base.stats, base.tuc));
        let state = EquipState {
            remaining: current.options.remaining_enhancements,
            failed_slots,
            stats: current.stats,
        };

        // **Read the pass without claiming it**, then claim only on the path that uses it.
        let key = daily_key(scroll);
        let guaranteed = !scroll.always_succeeds()
            && self
                .store
                .daily_claim_day(store::SCOPE_CHARACTER, i64::from(chr.id), &key)
                .ok()
                .flatten()
                != Some(store::today());
        let chance = if guaranteed { Chance::Guaranteed } else { Chance::Rolled };

        let applied = match scrolls::apply(scroll, &base, &state, chance, self.next_roll()) {
            Ok(a) => a,
            // Refused: nothing is taken, nothing is written, and the player is told which.
            Err(refusal) => {
                return self.admin_says(
                    template,
                    &npc::refused(refusal),
                    &format!("{:?} refused: {}", scroll, refusal.line()),
                )
            }
        };

        let mut new_stats = current;
        new_stats.stats = applied.after.stats;
        new_stats.options.remaining_enhancements = applied.after.remaining;
        match self.store.set_worn_equip(chr.id, equip_slot, &new_stats, applied.after.failed_slots)
        {
            Ok(true) => {}
            // The slot emptied under us, or the write failed. Nothing is taken.
            _ => {
                return self.admin_says(
                    template,
                    &npc::nothing_equipped(),
                    "the equipment row would not take the write; nothing was consumed",
                )
            }
        }
        // Only now: the scroll leaves the bag, and the daily pass is spent.
        let removed = self.store.remove_item(chr.id, store::InventoryType::Etc, bag_slot, Some(1));
        if guaranteed {
            let _ = self.store.claim_daily_perk_now(
                store::SCOPE_CHARACTER,
                i64::from(chr.id),
                &key,
            );
        }

        let item_name = self.item_name(item_id);
        let mut out = self.admin_says(
            template,
            &npc::outcome(
                scroll,
                &item_name,
                applied.succeeded,
                applied.stat_change,
                applied.after.remaining,
            ),
            &format!(
                "{} on {item_name}: succeeded={}, slot_spent={}, remaining {} -> {}, failed {} -> {}, \
                 guaranteed={guaranteed}. Nothing authenticates.",
                scroll.name(),
                applied.succeeded,
                applied.slot_spent,
                state.remaining,
                applied.after.remaining,
                state.failed_slots,
                applied.after.failed_slots,
            ),
        );
        if removed.is_err() {
            crate::server::log("   scroll: the equip was written but the scroll would not leave the bag");
        }
        // Refresh the item on screen: the tooltip is the only place the player can see the
        // stats and the remaining slots, and it is drawn from what the client holds.
        let refreshed = store::Item {
            item_id,
            kind: store::ItemKind::Equip(Some(new_stats)),
            failed_slots: applied.after.failed_slots,
        };
        let blob = self.item_blob(&refreshed);
        out.push(Reply {
            opcode: net::inventory::INVENTORY_OPERATION,
            body: net::inventory::inventory_added(
                store::InventoryType::Equip.as_u8() as i8,
                -i16::from(equip_slot),
                &blob,
            ),
            what: format!(
                "InventoryOperation ADD: re-sending worn {item_id} at slot -{equip_slot} so the \
                 tooltip shows the new stats and {} enhancement slot(s)",
                applied.after.remaining
            ),
        });
        // The sound and the animation, for the scroller and for everyone standing there.
        out.push(self.publish_scroll_effect(
            chr.id,
            chr.map_id,
            applied.succeeded,
            scroll.item_id(),
            item_id,
        ));
        out
    }

    /// `0x0236` to the whole map, and the scroller's own copy handed back to be sent.
    ///
    /// The owner, 2026-09-09: *"whenever the scrolling via `!scroll` succeeds or fails, it should
    /// also broadcast the scroll success or scroll fail sound to everyone, just like regular
    /// scrolling."*
    ///
    /// **`Bus::publish` skips `from`**, deliberately and everywhere - it is what stops a
    /// level-up animation playing twice for the person who levelled. So a packet the subject
    /// must *also* see is two sends, not one, and this returns the subject's copy rather than
    /// pushing it, so a caller cannot forget it and leave the scroller watching everyone
    /// else's screen flash.
    ///
    /// The body is byte-identical on both. `research/scrolling-2026-09-09.md` §4: the handler
    /// appears once across all four user-pool tables, and the character id in the body is the
    /// scroller on every copy - the dispatcher looks that user up in its own pool and drops
    /// the packet if there is no such user, so it addresses the subject, not the recipient.
    ///
    /// **Nothing supersedes.** Two scrolls are two events, the same rule that keeps two swings
    /// from rendering as one hit.
    fn publish_scroll_effect(
        &mut self,
        character_id: u32,
        map: u32,
        succeeded: bool,
        scroll_item_id: u32,
        equip_item_id: u32,
    ) -> Reply {
        use net::upgrade::{item_upgrade_effect, ItemUpgradeResult, ITEM_UPGRADE_EFFECT};
        let result = ItemUpgradeResult::from_success(succeeded);
        let body = item_upgrade_effect(character_id, result, scroll_item_id, equip_item_id);
        debug_assert_eq!(body.len(), net::upgrade::ITEM_UPGRADE_EFFECT_LEN);
        let what = format!(
            "ItemUpgradeEffect: character {character_id} scrolled {equip_item_id} with \
             {scroll_item_id}, {result:?} - plays {} on every screen in map {map}. \
             Nothing authenticates.",
            if succeeded { "EnchantSuccess_Delay" } else { "EnchantFailure_Delay" }
        );
        self.bus().publish(
            self.subscriber,
            map,
            Reply { opcode: ITEM_UPGRADE_EFFECT, body: body.clone(), what: what.clone() },
            None,
        );
        Reply { opcode: ITEM_UPGRADE_EFFECT, body, what }
    }

    /// The three scroll ids the character is carrying, with counts, in a stable order.
    fn scrolls_held(&self, character_id: u32) -> Vec<(Scroll, u16)> {
        let mut out = Vec::new();
        let Ok(rows) = self.store.bag_items(character_id, store::InventoryType::Etc) else {
            return out;
        };
        for scroll in [Scroll::Innocence, Scroll::Chaos, Scroll::CleanSlate] {
            let n: u32 = rows
                .iter()
                .filter(|r| r.item.item_id == scroll.item_id())
                .map(|r| u32::from(r.item.kind.quantity()))
                .sum();
            if n > 0 {
                out.push((scroll, n.min(u32::from(u16::MAX)) as u16));
            }
        }
        out
    }

    /// The first bag slot holding this scroll.
    fn scroll_bag_slot(&self, character_id: u32, scroll: Scroll) -> Option<u16> {
        self.store
            .bag_items(character_id, store::InventoryType::Etc)
            .ok()?
            .iter()
            .find(|r| r.item.item_id == scroll.item_id())
            .map(|r| r.slot)
    }

    /// Worn items as the menu wants them: `(equip slot, item id, name, remaining, failed)`.
    ///
    /// The item id is carried so the menu can draw the `#i<itemId>#` icon beside each row.
    fn worn_list(&self, character_id: u32) -> Vec<(u8, u32, String, u8, u8)> {
        let Ok(worn) = self.store.equipped_items(character_id) else { return Vec::new() };
        worn.iter()
            .map(|e| {
                let (remaining, failed) = match self.store.worn_item(character_id, e.slot) {
                    Ok(Some((_, stats, failed))) => (
                        stats
                            .map(|s| s.options.remaining_enhancements)
                            .unwrap_or_else(|| self.equip_base(e.item_id).tuc),
                        failed,
                    ),
                    _ => (0, 0),
                };
                (e.slot, e.item_id, self.item_name(e.item_id), remaining, failed)
            })
            .collect()
    }

    /// The template's own stats and slot count.
    ///
    /// A missing row is `tuc = 0` and no stats, which makes Chaos refuse and Innocence a no-op
    /// rather than inventing numbers for an item `gm-handbook/equips.txt` does not describe.
    ///
    /// # This must go through `EquipTemplate::fresh_stats`, and the reason is a shipped bug
    ///
    /// It used to build the 17-field set by hand here, and wrote `inc_pad: t.inc_wat` -
    /// **the same mistake `EquipTemplate`'s own doc block warns about in as many words**.
    /// `incPAD` is bit 8 and `incWAT` is bit 16; this client's `Character.wz` carries
    /// `incWAT` on 202 equips and `incPAD` on **none**, so bit 8 must always be zero.
    ///
    /// The owner, 2026-09-09, scrolling a Wizet Secret Agent Suitcase: *"it lost 1 weapon attack
    /// on the item, but it also gave it 200 attack power"*. `Weapon Attack: +199 (200 -1)` was
    /// the Chaos roll working exactly as designed; `Attack Power: +200 (0 +200)` beside it was
    /// this line, copying the weapon's 200 into a stat the item has no base for. The scroll
    /// was innocent - the *base* it was handed was already wrong, before any roll.
    ///
    /// `fresh_stats` is the one function in the workspace that maps this template onto the
    /// wire's bit order, and it writes a literal `0` at index 8 with a comment saying why.
    /// Calling it means there is nothing here left to get out of order.
    fn equip_base(&self, item_id: u32) -> EquipBase {
        match self.config.equips.get(&item_id) {
            Some(t) => {
                let fresh = t.fresh_stats();
                EquipBase { tuc: fresh.options.remaining_enhancements, stats: fresh.stats }
            }
            None => EquipBase::default(),
        }
    }

    /// A roll for the scroll. Distinct per call so two scrolls in a row do not share one.
    fn next_roll(&mut self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.subsec_nanos()).unwrap_or(0);
        self.scroll_roll_counter = self.scroll_roll_counter.wrapping_add(1);
        u64::from(nanos)
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(self.scroll_roll_counter.wrapping_mul(1_442_695_040_888_963_407))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, EquipTemplate};
    use std::sync::Arc;
    use store::Store;

    /// The owner's Wizet Secret Agent Suitcase, as `gm-handbook/equips.txt` has it: the attack is
    /// **`incWAT`**, and there is no `incPAD` column because this client's WZ has no such
    /// property on any of its 1760 equips.
    const SUITCASE: u32 = 1_402_043;

    fn session_with_a_weapon(inc_wat: u16) -> Session {
        let mut config = Config::default();
        config.equips.insert(
            SUITCASE,
            EquipTemplate { tuc: 7, inc_wat, ..EquipTemplate::default() },
        );
        Session::new(Arc::new(Store::open_in_memory().unwrap()), Arc::new(config))
    }

    /// **The base a scroll is measured against must not invent an `incPAD`.**
    ///
    /// The owner, 2026-09-09: *"the chaos scroll did something completely unexpected. It lost 1
    /// weapon attack on the item, but it also gave it 200 attack power."* The tooltip read
    /// `Weapon Attack: +199 (200 -1)` - the roll working - and `Attack Power: +200 (0 +200)`
    /// beside it, which is bit 8 carrying the weapon's own attack because `equip_base` built
    /// the set by hand and wrote `inc_pad: t.inc_wat`.
    ///
    /// The two halves are asserted separately on purpose. `inc_wat` alone would still pass
    /// with the bug present, and `inc_pad` alone would pass on a template that has no attack
    /// at all - which is 1558 of the 1760 equips, so a fixture picked at random would have
    /// been silent about this.
    #[test]
    fn the_base_for_a_weapon_has_no_attack_power_only_weapon_attack() {
        let base = session_with_a_weapon(200).equip_base(SUITCASE);
        assert_eq!(base.stats.inc_wat, 200, "the weapon's attack is incWAT, bit 16");
        assert_eq!(
            base.stats.inc_pad, 0,
            "incPAD is bit 8 and this client's data never carries it; a non-zero here is the \
             +200 Attack Power line the owner saw"
        );
        assert_eq!(base.tuc, 7, "the slot count comes from the same template");
    }

    /// An item the handbook does not describe gives a base of nothing rather than a guess -
    /// which is what makes Chaos refuse instead of rolling against invented numbers.
    #[test]
    fn an_unknown_equip_has_no_base_at_all() {
        let base = session_with_a_weapon(200).equip_base(SUITCASE + 1);
        assert_eq!(base.tuc, 0);
        assert_eq!(base.stats, net::opcode::EquipStatSet::default());
    }
}
