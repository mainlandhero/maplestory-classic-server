//! `!scroll` - the join between [`crate::scrolls`] (the rules), [`crate::scrollnpc`] (the
//! wording) and the store.
//!
//! Four screens, each a separate reply from the client. `crate::scrollnpc` draws the tree and
//! explains why the choice rides in the conversation path.
//!
//! # Every effect hangs off the transition
//!
//! `CLAUDE.md`'s Heena rule, and this handler has five effects to keep together: the
//! repurposed scroll leaves the bag, a real scroll leaves it too on the Treasure path, the
//! equip's stats change, its failed-slot count changes, and the daily pass is spent.
//! [`Session::apply_scroll`] refuses **before** any of them when the rules refuse, and takes
//! nothing out of the bag until the write to the equip has succeeded - so there is no path
//! that eats a scroll and changes nothing, or changes an item for free.
//!
//! # Everything is re-read after the confirm
//!
//! The yes/no box can sit on screen for as long as the player likes, and they can unequip the
//! target or drop the scroll while it does. So the apply step re-reads the bag, the worn slot
//! and the real scroll rather than trusting anything the menu step saw, and every one of those
//! re-reads has a refusal behind it.

use super::{Reply, Session};
use crate::config::ScrollTemplate;
use crate::scrollnpc::{self as npc, Branch, Confirmed};
use crate::scrolls::{self, Chance, EquipBase, EquipState, Scroll, SecretsMode};

/// The daily-perk key for one **mode** of the Scroll of Secrets.
///
/// The owner: *"per scroll type per character for the daily gate"*, and after 2026-09-09 the three
/// types live inside one item - so the key names the mode, not the item id. The scope is the
/// character.
///
/// **This changed shape when Event Trophy was dropped.** Claims written under the old
/// `scroll.<itemId>` keys are simply never read again, which hands every character one fresh
/// guaranteed use per mode. That is the harmless direction, and it is the reason the key is
/// built from [`SecretsMode::key`] - a stable word - rather than from the enum's discriminant,
/// which would silently re-map every claim if the variants were ever reordered.
fn daily_key(mode: SecretsMode) -> String {
    format!("scroll.secrets.{}", mode.key())
}

impl Session {
    /// `!scroll` - step 1, which of the two items.
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
                "the character is carrying neither of the two scrolls",
            );
        }
        let text = npc::pick_menu(&held);
        self.park(template, npc::PICK_PATH.to_string(), false);
        vec![self.menu_reply(
            template,
            text,
            format!(
                "ScriptMessage MENU opened by {} for character {}: {} scroll type(s) held",
                npc::COMMAND_TYPED,
                chr.id,
                held.len()
            ),
        )]
    }

    /// Park a conversation on a path. One place, so a step cannot forget a field.
    fn park(&mut self, template: u32, path: String, awaiting_yes_no: bool) {
        self.conversation = Some(super::Conversation {
            npc_template: template,
            quest_id: None,
            path,
            sent: 0,
            awaiting_yes_no,
            sent_with_next: false,
        });
    }

    fn menu_reply(&self, template: u32, text: String, what: String) -> Reply {
        Reply { opcode: net::script::SCRIPT_MESSAGE, body: net::script::npc_menu(template, &text), what }
    }

    fn ask_reply(&self, template: u32, text: String, what: String) -> Reply {
        Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_ask(template, &text, false),
            what,
        }
    }

    /// Every menu step answered. `None` means "not mine" - fall through to the other menus.
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
        let sel = selection as usize;

        // Step 1 -> the mode menu (Secrets) or straight to the equip menu (Treasure).
        if convo.path == npc::PICK_PATH {
            let held = self.scrolls_held(chr.id);
            let Some(&(scroll, _)) = held.get(sel) else {
                return Some(self.admin_says(
                    template,
                    &npc::nothing_to_use(),
                    &format!("selection {sel} names no scroll among {} held", held.len()),
                ));
            };
            return Some(match scroll {
                Scroll::Secrets => {
                    let free = self.free_modes_today(chr.id);
                    let text = npc::mode_menu(free);
                    self.park(template, npc::MODE_PATH.to_string(), false);
                    vec![self.menu_reply(
                        template,
                        text,
                        format!(
                            "ScriptMessage MENU: character {} is choosing what a Scroll of \
                             Secrets becomes; free today = {free:?}",
                            chr.id
                        ),
                    )]
                }
                Scroll::Treasure => self.open_equip_menu(template, chr.id, Branch::Treasure),
            });
        }

        // Step 2, Secrets only -> the equip menu.
        if convo.path == npc::MODE_PATH {
            let Some(&mode) = SecretsMode::ALL.get(sel) else {
                return Some(self.admin_says(
                    template,
                    &npc::nothing_to_use(),
                    &format!("selection {sel} names none of the three modes"),
                ));
            };
            return Some(self.open_equip_menu(template, chr.id, Branch::Secrets(mode)));
        }

        // Step 3 -> the confirm (Secrets) or the real-scroll menu (Treasure).
        if let Some(branch) = npc::branch_from_equip_path(&convo.path) {
            let worn = self.worn_list(chr.id);
            let Some((equip_slot, equip_id, item_name, _, _)) = worn.get(sel).cloned() else {
                return Some(self.admin_says(
                    template,
                    &npc::nothing_equipped(),
                    &format!("selection {sel} names no worn item among {}", worn.len()),
                ));
            };
            return Some(match branch {
                Branch::Secrets(mode) => {
                    let action = Confirmed::Secrets { mode, equip_slot };
                    let guaranteed = self.mode_is_free_today(chr.id, mode);
                    let text = npc::confirm(action, &item_name, "", guaranteed);
                    self.park(template, npc::confirm_path(action), true);
                    vec![self.ask_reply(
                        template,
                        text,
                        format!(
                            "ScriptMessage YES/NO: character {} confirming {} on equip slot \
                             {equip_slot} ({item_name}), guaranteed={guaranteed}",
                            chr.id,
                            mode.name()
                        ),
                    )]
                }
                Branch::Treasure => {
                    let offered = self.real_scrolls_that_fit(chr.id, equip_id);
                    let secrets = self.secrets_held(chr.id);
                    if offered.is_empty() && secrets == 0 {
                        return Some(self.admin_says(
                            template,
                            &npc::no_scroll_fits(&item_name),
                            &format!(
                                "no real scroll in the bag has category {} for equip {equip_id}",
                                ScrollTemplate::equip_category(equip_id)
                            ),
                        ));
                    }
                    let text = npc::real_scroll_menu(&item_name, &offered, secrets);
                    self.park(template, npc::real_path(equip_slot), false);
                    vec![self.menu_reply(
                        template,
                        text,
                        format!(
                            "ScriptMessage MENU: character {} is choosing which of {} scroll(s) \
                             the Treasure Scroll guarantees on {item_name} (Scrolls of Secrets \
                             held: {secrets})",
                            chr.id,
                            offered.len()
                        ),
                    )]
                }
            });
        }

        // Step 4, Treasure only -> the confirm.
        let equip_slot = npc::equip_slot_from_real_path(&convo.path)?;
        let Ok(Some((equip_id, _, _, _))) = self.store.worn_item(chr.id, equip_slot) else {
            return Some(self.admin_says(
                template,
                &npc::nothing_equipped(),
                "that equipment slot is empty now",
            ));
        };
        let offered = self.real_scrolls_that_fit(chr.id, equip_id);
        // Past the real scrolls: the Scroll of Secrets rows, while one is in the bag.
        let secrets_mode = sel
            .checked_sub(offered.len())
            .filter(|_| self.secrets_held(chr.id) > 0)
            .and_then(|j| npc::TREASURE_SECRETS_MODES.get(j).copied());
        if let Some(mode) = secrets_mode {
            let action = Confirmed::TreasureSecrets { mode, equip_slot };
            let item_name = self.item_name(equip_id);
            let text = npc::confirm(action, &item_name, "", true);
            self.park(template, npc::confirm_path(action), true);
            return Some(vec![self.ask_reply(
                template,
                text,
                format!(
                    "ScriptMessage YES/NO: character {} confirming a Treasure Scroll guaranteeing \
                     a Scroll of Secrets as {} on equip slot {equip_slot} ({item_name})",
                    chr.id,
                    mode.name()
                ),
            )]);
        }
        let Some((real_scroll, real_name, _, _)) = offered.get(sel).cloned() else {
            let item_name = self.item_name(equip_id);
            return Some(self.admin_says(
                template,
                &npc::no_scroll_fits(&item_name),
                &format!("selection {sel} names no scroll among {} offered", offered.len()),
            ));
        };
        let action = Confirmed::Treasure { equip_slot, real_scroll };
        let item_name = self.item_name(equip_id);
        let text = npc::confirm(action, &item_name, &real_name, false);
        self.park(template, npc::confirm_path(action), true);
        Some(vec![self.ask_reply(
            template,
            text,
            format!(
                "ScriptMessage YES/NO: character {} confirming a Treasure Scroll guaranteeing \
                 {real_scroll} ({real_name}) on equip slot {equip_slot} ({item_name})",
                chr.id
            ),
        )])
    }

    /// The worn-item menu, from either branch. Shared so the "nothing equipped" refusal cannot
    /// exist on one path and not the other.
    fn open_equip_menu(&mut self, template: u32, character_id: u32, branch: Branch) -> Vec<Reply> {
        let worn = self.worn_list(character_id);
        if worn.is_empty() {
            return self.admin_says(
                template,
                &npc::nothing_equipped(),
                "nothing is equipped, so there is nothing to scroll",
            );
        }
        let text = npc::equip_menu(branch, &worn);
        self.park(template, npc::equip_path(branch), false);
        vec![self.menu_reply(
            template,
            text,
            format!(
                "ScriptMessage MENU: character {character_id} on {branch:?} is picking from \
                 {} worn item(s)",
                worn.len()
            ),
        )]
    }

    /// The confirm answered. Called when the parked conversation is one of ours.
    pub(super) fn scroll_confirm_answer(&mut self, action_byte: i8) -> Vec<Reply> {
        let convo = match self.conversation.clone() {
            Some(c) => c,
            None => return Vec::new(),
        };
        let template = convo.npc_template;
        self.conversation = None;
        // "Keep scrolling?" - Yes is `!scroll` again; anything else closes, and silence is
        // safe on `0x00F3`.
        if convo.path == npc::AGAIN_PATH {
            if action_byte == net::script::SCRIPT_ACTION_YES {
                return self.open_scroll_picker();
            }
            return Vec::new();
        }
        let Some(action) = npc::confirmed_from_path(&convo.path) else {
            return Vec::new();
        };
        // Anything that is not Yes is a No. The client sends 0 for No and 1 for Yes; a closed
        // box is neither, and all three mean "do not do it".
        if action_byte != net::script::SCRIPT_ACTION_YES {
            return self.admin_says(template, &npc::cancelled(), "the player answered No");
        }
        self.apply_scroll(template, action)
    }

    /// The transition. Nothing is spent until the write succeeds.
    fn apply_scroll(&mut self, template: u32, action: Confirmed) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let equip_slot = action.equip_slot();
        let used_item = action.item();

        // Re-read everything. The confirm box may have been on screen for a while.
        let Some(bag_slot) = self.bag_slot_of(chr.id, used_item.item_id()) else {
            return self.admin_says(
                template,
                &npc::nothing_to_use(),
                "the scroll is no longer in the bag",
            );
        };
        let Ok(Some((item_id, stats, failed_slots, rolled_base))) = self.store.worn_item(chr.id, equip_slot)
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
        // The Lucky Day Scroll's mark (`net::opcode::ATTRIBUTE_LUCKY_DAY`): the next scroll on this
        // item succeeds - `!scroll`'s included - and spends it.
        let lucky = current.options.attribute & net::opcode::ATTRIBUTE_LUCKY_DAY != 0;
        let state = EquipState {
            remaining: current.options.remaining_enhancements,
            failed_slots,
            stats: current.stats,
        };

        // The Treasure path spends a second item, so it has a second thing to re-read and a
        // second way to refuse. Both are resolved before anything is applied.
        let mut second_item: Option<(store::InventoryType, u16)> = None;
        let mut real_name = String::new();
        let applied = match action {
            Confirmed::Secrets { mode, .. } => {
                // **Read the pass without claiming it**, then claim only on the path that
                // uses it. `daily_claim_day` only reads.
                // **A Lucky Day mark guarantees it first** (2026-10-01), so the day's free use
                // is kept for later; the mark is spent below either way.
                let guaranteed = !lucky && !mode.always_succeeds() && self.mode_is_free_today(chr.id, mode);
                let chance = if guaranteed || lucky { Chance::Guaranteed } else { Chance::Rolled };
                // **Innocence keeps a mob drop's roll.** The owner, 2026-09-24: *"Can we make
                // Innocence Scrolls keep a good base roll?"* An item that rolled when it
                // dropped (`world::variance`) reverts to THAT, not the template; anything that
                // never rolled has no `rolled_base` and reverts to the template as before.
                // Chaos still reads the template: which stats exist is the item's, not the roll's.
                let base = match (mode, rolled_base) {
                    (SecretsMode::Innocence, Some(rolled)) => EquipBase { tuc: base.tuc, stats: rolled },
                    _ => base,
                };
                match scrolls::apply(mode, &base, &state, chance, self.next_roll()) {
                    Ok(a) => {
                        if guaranteed {
                            let _ = self.store.claim_daily_perk_now(
                                store::SCOPE_CHARACTER,
                                i64::from(chr.id),
                                &daily_key(mode),
                            );
                        }
                        a
                    }
                    Err(refusal) => {
                        return self.admin_says(
                            template,
                            &npc::refused(refusal),
                            &format!("{mode:?} refused: {}", refusal.line()),
                        )
                    }
                }
            }
            Confirmed::Treasure { real_scroll, .. } => {
                // It must still fit, still be in the bag, and still be a scroll this client
                // has. Each is a separate way for the confirm to have gone stale.
                if !ScrollTemplate::fits(real_scroll, item_id) {
                    return self.admin_says(
                        template,
                        &npc::no_scroll_fits(&self.item_name(item_id)),
                        "the chosen scroll does not fit the equipment in that slot",
                    );
                }
                let Some(scroll_template) = self.config.scrolls.get(&real_scroll).copied() else {
                    return self.admin_says(
                        template,
                        &npc::refused(scrolls::Refusal::NoScrollFits),
                        &format!("{real_scroll} is in no scroll table; nothing was consumed"),
                    );
                };
                let Some(slot) = self.bag_slot_of(chr.id, real_scroll) else {
                    return self.admin_says(
                        template,
                        &npc::refused(scrolls::Refusal::NoScrollFits),
                        "the chosen scroll is no longer in the bag",
                    );
                };
                second_item = Some((store::InventoryType::Use, slot));
                real_name = self.item_name(real_scroll);
                match scrolls::apply_treasure(&base, &state, &scroll_template.increments) {
                    Ok(a) => a,
                    Err(refusal) => {
                        return self.admin_says(
                            template,
                            &npc::refused(refusal),
                            &format!("Treasure Scroll refused: {}", refusal.line()),
                        )
                    }
                }
            }
            // A Scroll of Secrets that cannot fail. Its own rules (Chaos needs a slot, Clean
            // Slate a failure) still refuse; no daily pass is read or claimed.
            Confirmed::TreasureSecrets { mode, .. } => {
                let Some(slot) = self.bag_slot_of(chr.id, scrolls::SCROLL_OF_SECRETS) else {
                    return self.admin_says(
                        template,
                        &npc::nothing_to_use(),
                        "the Scroll of Secrets is no longer in the bag",
                    );
                };
                second_item = Some((store::InventoryType::Etc, slot));
                match scrolls::apply(mode, &base, &state, Chance::Guaranteed, self.next_roll()) {
                    Ok(a) => a,
                    Err(refusal) => {
                        return self.admin_says(
                            template,
                            &npc::refused(refusal),
                            &format!("Treasure Scroll on a {mode:?} refused: {}", refusal.line()),
                        )
                    }
                }
            }
        };

        let mut new_stats = current;
        new_stats.stats = applied.after.stats;
        new_stats.options.remaining_enhancements = applied.after.remaining;
        new_stats.options.attribute &= !net::opcode::ATTRIBUTE_LUCKY_DAY;
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
        // Only now do the scrolls leave the bag.
        let removed =
            self.store.remove_item(chr.id, store::InventoryType::Etc, bag_slot, Some(1));
        let removed_real = second_item.map(|(tab, slot)| self.store.remove_item(chr.id, tab, slot, Some(1)));

        let item_name = self.item_name(item_id);
        let changes: Vec<(&str, i32)> = applied.changes.clone();
        let result = npc::outcome(
            action,
            &item_name,
            &real_name,
            applied.succeeded,
            &changes,
            applied.after.remaining,
        );
        // **More scrolls in the bag: ask to go on** rather than make them type `!scroll`
        // again (the owner, 2026-10-01). The bag is read after the removal above.
        let left = self.scrolls_held(chr.id);
        let what = format!(
            "{} on {item_name}: succeeded={}, slot_spent={}, remaining {} -> {}, failed {} -> {},              changes={:?}. Nothing authenticates. | {result:?}",
            used_item.name(),
            applied.succeeded,
            applied.slot_spent,
            state.remaining,
            applied.after.remaining,
            state.failed_slots,
            applied.after.failed_slots,
            applied.changes,
        );
        let mut out = if left.is_empty() {
            vec![Reply {
                opcode: net::script::SCRIPT_MESSAGE,
                body: net::script::npc_say(template, &result, false, false),
                what: format!("ScriptMessage Say: the result - {what}"),
            }]
        } else {
            self.park(template, npc::AGAIN_PATH.to_string(), true);
            let text = npc::again(&result, &left);
            vec![self.ask_reply(template, text, format!("ScriptMessage YES/NO: the result, and keep scrolling? - {what}"))]
        };
        if removed.is_err() {
            crate::server::log("   scroll: the equip was written but the scroll would not leave the bag");
        }
        if matches!(removed_real, Some(Err(_))) {
            crate::server::log("   scroll: the equip was written but the REAL scroll would not leave the bag");
        }
        // Refresh the item on screen: the tooltip is the only place the player can see the
        // stats and the remaining slots, and it is drawn from what the client holds.
        let refreshed = store::Item {
            item_id,
            kind: store::ItemKind::Equip(Some(new_stats)),
            failed_slots: applied.after.failed_slots,
            pet_id: None,
            rolled_base: None, // server-only; this copy is only drawn
        };
        let blob = self.item_blob(&refreshed);
        out.push(Reply {
            opcode: net::inventory::INVENTORY_OPERATION,
            // Type 6 for a worn cash equip (the pet's at 114): `net::inventory::worn_slot_tab`.
            body: net::inventory::inventory_added(
                net::inventory::worn_slot_tab(equip_slot),
                -i16::from(equip_slot),
                &blob,
            ),
            what: format!(
                "InventoryOperation ADD: re-sending worn {item_id} at slot -{equip_slot} so the \
                 tooltip shows the new stats and {} enhancement slot(s)",
                applied.after.remaining
            ),
        });
        // The sound and the animation, for the scroller and for everyone standing there. The
        // scroll id it names is the one the player recognises: the real scroll on the Treasure
        // path, the repurposed item otherwise.
        let named_scroll = match action {
            Confirmed::Treasure { real_scroll, .. } => real_scroll,
            Confirmed::TreasureSecrets { .. } => scrolls::SCROLL_OF_SECRETS,
            Confirmed::Secrets { .. } => used_item.item_id(),
        };
        out.push(self.publish_scroll_effect(
            chr.id,
            self.field_of(&chr),
            applied.succeeded,
            named_scroll,
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
        map: crate::fields::FieldKey,
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

    /// Which of the two repurposed scrolls the character is carrying, with counts, in a stable
    /// order.
    fn scrolls_held(&self, character_id: u32) -> Vec<(Scroll, u16)> {
        let mut out = Vec::new();
        let Ok(rows) = self.store.bag_items(character_id, store::InventoryType::Etc) else {
            return out;
        };
        for scroll in [Scroll::Secrets, Scroll::Treasure] {
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

    /// How many Scrolls of Secrets the bag holds - the Treasure Scroll's menu offers them.
    fn secrets_held(&self, character_id: u32) -> u16 {
        self.scrolls_held(character_id)
            .into_iter()
            .find(|(scroll, _)| *scroll == Scroll::Secrets)
            .map_or(0, |(_, n)| n)
    }

    /// The first bag slot holding this item. Etc for the repurposed scrolls, Use for the real
    /// ones - `2040000`-`2048000` are consumables and live in the Use tab.
    fn bag_slot_of(&self, character_id: u32, item_id: u32) -> Option<u16> {
        let tab = if scrolls::REPURPOSED.contains(&item_id) {
            store::InventoryType::Etc
        } else {
            store::InventoryType::Use
        };
        self.store
            .bag_items(character_id, tab)
            .ok()?
            .iter()
            .find(|r| r.item.item_id == item_id)
            .map(|r| r.slot)
    }

    /// The real scrolls in the Use tab that fit `equip_item_id`, as
    /// `(item id, name, normal success rate, count held)`, in ascending id order.
    ///
    /// **Filtered by category before it is shown**, not refused afterwards - see the module
    /// docs on why the Treasure branch asks for the equipment first.
    fn real_scrolls_that_fit(
        &self,
        character_id: u32,
        equip_item_id: u32,
    ) -> Vec<(u32, String, u16, u16)> {
        let Ok(rows) = self.store.bag_items(character_id, store::InventoryType::Use) else {
            return Vec::new();
        };
        let mut counts: std::collections::BTreeMap<u32, u32> = std::collections::BTreeMap::new();
        for r in &rows {
            let id = r.item.item_id;
            if !self.config.scrolls.contains_key(&id) {
                continue;
            }
            if !ScrollTemplate::fits(id, equip_item_id) {
                continue;
            }
            *counts.entry(id).or_default() += u32::from(r.item.kind.quantity());
        }
        counts
            .into_iter()
            .filter(|&(_, n)| n > 0)
            .map(|(id, n)| {
                let success = self.config.scrolls.get(&id).map_or(0, |t| t.success);
                (id, self.item_name(id), success, n.min(u32::from(u16::MAX)) as u16)
            })
            .collect()
    }

    /// Is this mode's guaranteed use still unspent today?
    fn mode_is_free_today(&self, character_id: u32, mode: SecretsMode) -> bool {
        if mode.always_succeeds() {
            // Innocence has no daily pass; saying "free" for it would light up a row that
            // never changes and imply the day can spend it.
            return false;
        }
        self.store
            .daily_claim_day(store::SCOPE_CHARACTER, i64::from(character_id), &daily_key(mode))
            .ok()
            .flatten()
            != Some(store::today())
    }

    /// The three modes' free-today flags, in [`SecretsMode::ALL`] order.
    fn free_modes_today(&self, character_id: u32) -> [bool; 3] {
        let mut out = [false; 3];
        for (i, mode) in SecretsMode::ALL.into_iter().enumerate() {
            out[i] = self.mode_is_free_today(character_id, mode);
        }
        out
    }

    /// Worn items as the menu wants them: `(equip slot, item id, name, remaining, failed)`.
    ///
    /// The item id is carried so the menu can draw the `#i<itemId>#` icon beside each row, and
    /// so the Treasure branch can filter scrolls by the target's category.
    fn worn_list(&self, character_id: u32) -> Vec<(u8, u32, String, u8, u8)> {
        let Ok(worn) = self.store.equipped_items(character_id) else { return Vec::new() };
        worn.iter()
            .map(|e| {
                let (remaining, failed) = match self.store.worn_item(character_id, e.slot) {
                    Ok(Some((_, stats, failed, _))) => (
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

    /// The owner's Wizet Secret Agent Suitcase, and this is its real row rather than a made-up one:
    ///
    /// ```text
    /// 1322999, 7, 0,0,0,0,0,0,0,0, 200, 0,0,0,0,0,0,0, …   gm-handbook/equips.txt
    ///          ^tuc                     ^incWAT
    /// ```
    ///
    /// `tuc 7` and `incWAT 200` are what the screenshot shows - `Weapon Attack: +199 (200 -1)`
    /// and `Remaining Enhancements: 6` after one Chaos spent a slot - so the fixture and the
    /// screen are the same numbers. There is no `incPAD` column in that file at all, because
    /// this client's WZ has no such property on any of its 1760 equips.
    const SUITCASE: u32 = 1_322_999;
    const SUITCASE_WAT: u16 = 200;
    const SUITCASE_TUC: u16 = 7;

    fn session_with_a_weapon(inc_wat: u16) -> Session {
        let mut config = Config::default();
        config.equips.insert(
            SUITCASE,
            EquipTemplate { tuc: SUITCASE_TUC, inc_wat, ..EquipTemplate::default() },
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
        let base = session_with_a_weapon(SUITCASE_WAT).equip_base(SUITCASE);
        assert_eq!(base.stats.inc_wat, SUITCASE_WAT, "the weapon's attack is incWAT, bit 16");
        assert_eq!(
            base.stats.inc_pad, 0,
            "incPAD is bit 8 and this client's data never carries it; a non-zero here is the \
             +200 Attack Power line the owner saw"
        );
        assert_eq!(
            u16::from(base.tuc),
            SUITCASE_TUC,
            "the slot count comes from the same template - the screenshot shows 6 left after \
             one Chaos spent one"
        );
    }

    /// An item the handbook does not describe gives a base of nothing rather than a guess -
    /// which is what makes Chaos refuse instead of rolling against invented numbers.
    #[test]
    fn an_unknown_equip_has_no_base_at_all() {
        let base = session_with_a_weapon(SUITCASE_WAT).equip_base(SUITCASE + 1);
        assert_eq!(base.tuc, 0);
        assert_eq!(base.stats, net::opcode::EquipStatSet::default());
    }

    /// **Innocence must never look like it has a daily pass to spend.**
    ///
    /// Its patch note is a flat `(100%)` where the other two say *"100% first time of the day,
    /// otherwise 60%"*. If it reported free-today, the mode menu would say so and the apply
    /// step would claim a pass for a mode that could not have used one - silently costing the
    /// player their guaranteed Chaos.
    #[test]
    fn innocence_never_reports_a_free_use_and_the_other_two_start_free() {
        let s = session_with_a_weapon(SUITCASE_WAT);
        let free = s.free_modes_today(200);
        assert!(free[0], "Chaos starts the day free");
        assert!(!free[1], "Innocence has no daily pass at all");
        assert!(free[2], "Clean Slate starts the day free");
        assert_eq!(SecretsMode::ALL[1], SecretsMode::Innocence, "the order the flags are in");
    }

    /// **Innocence keeps a mob drop's roll.** The owner, 2026-09-24: *"Can we make Innocence
    /// Scrolls keep a good base roll?"*
    ///
    /// A worn Lv 60 hat whose template is 40 WDEF / 2 STR, which dropped rolled at 60 / 4 and
    /// has since been Chaos'd to 65 / 3: Innocence takes it to **60 / 4** - its roll - with every
    /// slot back, and the rolled base is still there for next time. The control is the same
    /// hat with no rolled base (any item that did not come from a mob): Innocence takes it to
    /// the template's 40 / 2, exactly as before.
    #[test]
    fn innocence_reverts_a_rolled_drop_to_its_roll_and_anything_else_to_the_template() {
        const HAT: u32 = 1_002_999;
        let mut config = Config::default();
        config.equips.insert(HAT, EquipTemplate { tuc: 7, inc_str: 2, inc_pdd: 40, req_level: 60, ..EquipTemplate::default() });
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let id = store.create_character(account, 0, &net::opcode::Character { name: "Hatter".into(), ..Default::default() }).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::new(store.clone(), Arc::new(config));
        assert!(s.claim_for_character(id).contains("claimed the migration"));

        let template = s.config.equips[&HAT].fresh_stats();
        let rolled = net::opcode::EquipStatSet { inc_str: 4, inc_pdd: 60, ..Default::default() };
        let mut chaosed = template;
        chaosed.stats = net::opcode::EquipStatSet { inc_str: 3, inc_pdd: 65, ..Default::default() };
        chaosed.options.remaining_enhancements = 5;

        let innocence = |s: &mut Session, rolled_base: Option<net::opcode::EquipStatSet>| {
            let _ = store.unequip_to_bag(id, 1, Some(20));
            let hat = store::Item { item_id: HAT, kind: store::ItemKind::Equip(Some(chaosed)), failed_slots: 0, pet_id: None, rolled_base };
            store.set_inventory_slot(id, store::InventoryType::Equip, 10, &hat).unwrap();
            store.equip_from_bag(id, 10, 1).unwrap();
            store.add_item(id, store::InventoryType::Etc, &store::Item::bundle(scrolls::SCROLL_OF_SECRETS, 1), 100).unwrap();
            let _ = s.apply_scroll(9_010_000, Confirmed::Secrets { mode: SecretsMode::Innocence, equip_slot: 1 });
            store.worn_item(id, 1).unwrap().expect("still worn")
        };

        let (_, stats, _, kept) = innocence(&mut s, Some(rolled));
        let stats = stats.expect("stored stats");
        assert_eq!(stats.stats, rolled, "back to its ROLL (60 WDEF, 4 STR), not the template's 40 / 2");
        assert_eq!(stats.options.remaining_enhancements, 7, "every slot back");
        assert_eq!(kept, Some(rolled), "and the roll is still the base for the next Innocence");

        let (_, stats, _, kept) = innocence(&mut s, None);
        assert_eq!(stats.expect("stored stats").stats, template.stats, "no roll: the template, as before");
        assert_eq!(kept, None);
    }

    /// A worn hat with 7 slots on a claimed character, and `secrets` / `treasure` of each
    /// repurposed scroll in the Etc tab.
    fn hatter(secrets: u16, treasure: u16) -> (Arc<Store>, Session, u32) {
        const HAT: u32 = 1_002_999;
        let mut config = Config::default();
        config.equips.insert(HAT, EquipTemplate { tuc: 7, inc_str: 2, inc_pdd: 40, ..EquipTemplate::default() });
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let id = store.create_character(account, 0, &net::opcode::Character { name: "Hatter".into(), ..Default::default() }).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::new(store.clone(), Arc::new(config));
        assert!(s.claim_for_character(id).contains("claimed the migration"));
        let _ = store.unequip_to_bag(id, 1, Some(20));
        store.set_inventory_slot(id, store::InventoryType::Equip, 10, &store::Item::equip(HAT)).unwrap();
        store.equip_from_bag(id, 10, 1).unwrap();
        for (item, n) in [(scrolls::SCROLL_OF_SECRETS, secrets), (scrolls::TREASURE_SCROLL, treasure)] {
            if n > 0 {
                store.add_item(id, store::InventoryType::Etc, &store::Item::bundle(item, n), 100).unwrap();
            }
        }
        (store, s, id)
    }

    /// **A Treasure Scroll guarantees a Scroll of Secrets** (the owner, 2026-10-01). Chaos is
    /// rolled at 60% once today's free use is spent; with the Treasure Scroll it cannot fail,
    /// takes one of each scroll, and claims no daily pass. Run many times so a 60% roll would
    /// have failed at least once.
    #[test]
    fn a_treasure_scroll_makes_a_scroll_of_secrets_succeed_and_spends_both() {
        let (store, mut s, id) = hatter(40, 40);
        let _ = store.claim_daily_perk_now(store::SCOPE_CHARACTER, i64::from(id), &daily_key(SecretsMode::Chaos));
        for n in 1..=20u16 {
            let _ = s.apply_scroll(9_010_000, Confirmed::TreasureSecrets { mode: SecretsMode::Chaos, equip_slot: 1 });
            let (_, stats, failed, _) = store.worn_item(id, 1).unwrap().unwrap();
            if n <= 7 {
                assert_eq!(stats.unwrap().options.remaining_enhancements, 7 - n as u8, "a slot per Chaos, as always");
                assert_eq!(failed, 0, "and never a failure");
            }
        }
        let held = s.scrolls_held(id);
        assert_eq!(held, vec![(Scroll::Secrets, 33), (Scroll::Treasure, 33)], "7 applied; the 13 refused (no slots) took nothing");
        assert!(!s.mode_is_free_today(id, SecretsMode::Chaos), "the pass was spent before, and stays spent");
        assert!(s.mode_is_free_today(id, SecretsMode::CleanSlate), "Clean Slate's pass untouched");
    }

    /// **After a result, "keep scrolling?"** while either scroll is left - Yes is the picker
    /// again, No closes without a word. With none left, the plain result.
    #[test]
    fn the_result_asks_to_keep_scrolling_and_yes_opens_the_picker() {
        let (_store, mut s, _id) = hatter(2, 0);
        let out = s.apply_scroll(9_010_000, Confirmed::Secrets { mode: SecretsMode::Innocence, equip_slot: 1 });
        assert!(out[0].what.contains("keep scrolling?"), "{:?}", out[0].what);
        assert_eq!(s.conversation.as_ref().map(|c| c.path.as_str()), Some(npc::AGAIN_PATH));
        let out = s.scroll_confirm_answer(net::script::SCRIPT_ACTION_YES);
        assert!(out[0].what.contains("MENU opened"), "{:?}", out[0].what);
        assert_eq!(s.conversation.as_ref().map(|c| c.path.as_str()), Some(npc::PICK_PATH));

        s.conversation = None;
        let out = s.apply_scroll(9_010_000, Confirmed::Secrets { mode: SecretsMode::Innocence, equip_slot: 1 });
        assert!(out[0].what.contains("Say: the result"), "the last one: no question {:?}", out[0].what);
        assert!(s.conversation.is_none());

        let (_store, mut s, _id) = hatter(3, 0);
        let _ = s.apply_scroll(9_010_000, Confirmed::Secrets { mode: SecretsMode::Innocence, equip_slot: 1 });
        assert!(s.scroll_confirm_answer(net::script::SCRIPT_ACTION_NO).is_empty(), "No: closed, nothing said");
        assert!(s.conversation.is_none());
    }

    /// The daily key names the mode, not the item - all three modes now live in one item, so
    /// a key built from the item id would give the three of them one shared pass.
    #[test]
    fn each_mode_has_its_own_daily_key() {
        let keys: Vec<String> = SecretsMode::ALL.into_iter().map(daily_key).collect();
        assert_eq!(keys.len(), 3);
        assert!(keys.iter().all(|k| k.starts_with("scroll.secrets.")), "{keys:?}");
        let mut sorted = keys.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 3, "three distinct keys, not one shared pass: {keys:?}");
    }
}
