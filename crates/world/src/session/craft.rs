//! The Crafting Journal: `0x02F6` in, `0x0398` out.
//!
//! The owner, 2026-09-21: *"We need to implement crafting in our server."* The decode is
//! `research/crafting-2026-09-21.md`, the wire is [`net::craft`], the recipes and the two
//! gates are [`crate::crafting`], and what a character has learnt is `store::crafting`.
//!
//! # One craft is TWO packets, and the item moves on the second
//!
//! The window asks to begin, runs the recipe's own animation, then asks to finish:
//!
//! ```text
//! -> 0x02F6 mode 0  profession, recipeKey, useAdditive, count
//! <- 0x0398 mode 4  result 0            the bar starts filling
//!    ... ProcessTimeMS of animation, entirely client-side ...
//! -> 0x02F6 mode 3
//! <- 0x0398 mode 7  result 0            + the inventory, meso and skill packets
//! ```
//!
//! **Nothing is taken on the begin.** A player who closes the client mid-animation keeps
//! their materials, which is the forgiving direction; taking on the begin would mean a
//! disconnect eats them. The begin still runs every check, because the client is owed a
//! refusal *before* it plays three seconds of animation.
//!
//! The checks run **again** on the complete, against the bag as it is then. Between the two
//! packets the player can drop, trade or move items - and **nothing authenticates**, so a
//! crafted `0x02F6 mode 3` is free. The second check is the one that decides.
//!
//! # A multi-item craft is N of these pairs
//!
//! `count` is the client's own loop counter: on a successful mode 7 it increments its
//! counter and, if more remain, sends another mode 0 by itself. So this handler only ever
//! makes **one** item per pair and never loops.

use super::{Reply, Session};

/// The craft this session has accepted and is waiting to finish.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PendingCraft {
    /// The recipe's own key, as the begin carried it.
    pub(super) key: u32,
    /// Whether the ADDITIVE box was ticked. Kept from the begin: the complete carries no
    /// fields at all, so this is the only record of it.
    pub(super) use_additive: bool,
}

impl Session {
    /// `0x02F6` - the crafting window's only packet.
    ///
    /// **Always answered.** Every arm below ends in a `0x0398`, including the modes this
    /// server does not model: an unanswered request leaves the client's request latch set
    /// (`FUN_142cc4430`), and a latched client never sends another craft.
    pub(super) fn on_craft_request(&mut self, body: &[u8]) -> Vec<Reply> {
        let request = match net::craft::CraftRequest::parse(body) {
            Ok(r) => r,
            Err(e) => {
                crate::server::log(&format!("   craft: unreadable 0x02F6 body ({e}); answered with a reset"));
                return vec![self.craft_reply(
                    net::craft::craft_cancelled(),
                    "the request body was too short to read".to_string(),
                )];
            }
        };
        match request {
            net::craft::CraftRequest::Begin { profession, recipe_key, use_additive, count } => {
                self.craft_begin(profession, recipe_key, use_additive, count)
            }
            net::craft::CraftRequest::Complete => self.craft_complete(),
            net::craft::CraftRequest::Cancel => {
                self.pending_craft = None;
                vec![self.craft_reply(net::craft::craft_cancelled(), "cancelled".to_string())]
            }
            net::craft::CraftRequest::Unknown(mode) => {
                // Mode 6 clears the latch and changes nothing - the honest answer to a mode
                // nobody has decoded, rather than a made-up success or failure.
                vec![self.craft_reply(
                    net::craft::craft_acknowledged(),
                    format!("0x02F6 mode {mode} is not modelled; the latch is cleared and nothing else"),
                )]
            }
        }
    }

    /// Mode 0: may this craft start?
    fn craft_begin(&mut self, profession: u32, recipe_key: u32, use_additive: bool, count: u32) -> Vec<Reply> {
        if self.pending_craft.is_some() {
            // The one code the client does NOT reset its window on - see `net::craft`.
            return vec![self.craft_begin_reply(
                net::craft::CraftResult::AlreadyCrafting,
                format!("recipe {recipe_key}: one is already running"),
            )];
        }
        let (result, why) = self.craft_check(recipe_key, profession, use_additive);
        if result != net::craft::CraftResult::Ok {
            return vec![self.craft_begin_reply(result, format!("recipe {recipe_key}: {why}"))];
        }
        self.pending_craft = Some(PendingCraft { key: recipe_key, use_additive });
        let asked = if count == net::craft::CRAFT_ALL { "all".to_string() } else { count.to_string() };
        vec![self.craft_begin_reply(
            net::craft::CraftResult::Ok,
            format!("recipe {recipe_key} accepted (profession {profession}, {asked} asked for, additive {use_additive}); nothing taken until the complete"),
        )]
    }

    /// Mode 3: the animation finished - take the materials and hand the item over.
    fn craft_complete(&mut self) -> Vec<Reply> {
        let Some(pending) = self.pending_craft.take() else {
            return vec![self.craft_complete_reply(
                net::craft::CraftResult::TryAgain,
                "a complete arrived with no craft running".to_string(),
            )];
        };
        let Some(chr) = self.claimed_character() else {
            return vec![self.craft_complete_reply(
                net::craft::CraftResult::UnknownError,
                "no character is claimed".to_string(),
            )];
        };
        let Some(recipe) = self.config.recipes.get(&pending.key).cloned() else {
            return vec![self.craft_complete_reply(
                net::craft::CraftResult::TryAgain,
                format!("recipe {} vanished between the begin and the complete", pending.key),
            )];
        };
        // **The checks run again.** See the module docs: the bag can change between the two
        // packets, and the complete is the one that decides.
        let (result, why) = self.craft_check(recipe.key, u32::from(recipe.profession), pending.use_additive);
        if result != net::craft::CraftResult::Ok {
            return vec![self.craft_complete_reply(result, format!("recipe {}: {why}", recipe.key))];
        }

        let mut out = Vec::new();
        // Materials first, then mesos, then the item: if the item somehow cannot be placed
        // the player has lost the materials, so the room check above is the one that has to
        // be right. It is the quests' own `questroom::shortfall`.
        for &(item_id, per) in &recipe.ingredients {
            out.extend(self.take_for_craft(chr.id, item_id, per));
        }
        if pending.use_additive && recipe.additive.0 != 0 {
            out.extend(self.take_for_craft(chr.id, recipe.additive.0, recipe.additive.1));
        }
        if recipe.meso > 0 {
            let _ = self.store.add_mesos(chr.id, -(i64::from(recipe.meso)));
            out.extend(self.meso_reply(chr.id));
        }

        let (item_id, made) = recipe.result;
        let inv = match self.config.tab_for(item_id).or_else(|| store::InventoryType::for_item(item_id)) {
            Some(inv) => inv,
            None => {
                return vec![self.craft_complete_reply(
                    net::craft::CraftResult::UnknownError,
                    format!("recipe {} makes item {item_id}, which belongs in no tab", recipe.key),
                )]
            }
        };
        let item = if inv == store::InventoryType::Equip {
            store::Item::equip(item_id)
        } else {
            store::Item::bundle(item_id, made.min(u32::from(u16::MAX)) as u16)
        };
        let max_stack = self.config.shops.max_stack(item_id);
        match self.store.add_item(chr.id, inv, &item, max_stack) {
            Ok(placed) => out.extend(self.inventory_added_replies(inv, &placed, "crafted")),
            Err(e) => {
                // The materials are already gone, so this is worth saying out loud rather
                // than only in a result code.
                out.extend(self.notice(format!("Your craft could not be placed in your bag: {e}")));
                return vec![self.craft_complete_reply(
                    net::craft::CraftResult::NotEnoughSpace,
                    format!("recipe {}: {item_id} would not go into {inv:?}: {e}", recipe.key),
                )];
            }
        }

        out.extend(self.pay_mastery(recipe.profession, recipe.result_exp));
        out.push(self.craft_complete_reply(
            net::craft::CraftResult::Ok,
            format!(
                "recipe {} crafted: {made} x {item_id}, {} meso, {} mastery",
                recipe.key, recipe.meso, recipe.result_exp
            ),
        ));
        out
    }

    /// Every reason a craft can be refused, in the order the client itself checks them.
    ///
    /// Returns the code **and** a sentence for the log, because a bare `3` in `world-ch0.log`
    /// says nothing about which of a recipe's five ingredients was short.
    fn craft_check(&self, recipe_key: u32, profession: u32, use_additive: bool) -> (net::craft::CraftResult, String) {
        use net::craft::CraftResult;
        if self.config.recipes.is_empty() {
            return (CraftResult::Unavailable, "this server has no recipe table - run tools/dump_craftrecipe.py".to_string());
        }
        let Some(chr) = self.claimed_character() else {
            return (CraftResult::UnknownError, "no character is claimed".to_string());
        };
        let Some(recipe) = self.config.recipes.get(&recipe_key) else {
            return (CraftResult::TryAgain, "no such recipe".to_string());
        };
        if u32::from(recipe.profession) != profession {
            return (
                CraftResult::TryAgain,
                format!("asked for under profession {profession} but it belongs to {}", recipe.profession),
            );
        }
        let (level, _) = self.store.profession(chr.id, recipe.profession).unwrap_or((0, 0));
        if level == 0 {
            return (
                CraftResult::SkillTooLow,
                format!("{} has not been learnt", crate::crafting::profession_name(recipe.profession)),
            );
        }
        if level < recipe.craft_level {
            return (
                CraftResult::SkillTooLow,
                format!("{} is level {level}, the recipe wants {}", crate::crafting::profession_name(recipe.profession), recipe.craft_level),
            );
        }
        if let Some(why) = self.station_shortfall(recipe.profession) {
            return (CraftResult::NotNearTools, why);
        }
        // Materials. The additive only counts when the box was ticked - an untouched
        // additive is not a requirement.
        let mut wanted: Vec<(u32, u32)> = recipe.ingredients.clone();
        if use_additive && recipe.additive.0 != 0 {
            wanted.push(recipe.additive);
        }
        for (item_id, per) in &wanted {
            let held = self.held_count(chr.id, *item_id);
            if held < *per {
                return (
                    CraftResult::NotEnoughMaterials,
                    format!("wants {per} x {item_id} and you hold {held}"),
                );
            }
        }
        if recipe.meso > 0 {
            let mesos = self.store.mesos(chr.id).unwrap_or(0);
            if mesos < recipe.meso {
                return (
                    CraftResult::NotEnoughMesos,
                    format!("costs {} meso and you hold {mesos}", recipe.meso),
                );
            }
        }
        // Room, by the quests' own reckoning: what comes in, minus what goes out.
        let gives = self.craft_room_rows(recipe, use_additive);
        if let Ok(bag) = self.store.bag(chr.id) {
            let short = crate::questroom::shortfall(&bag, &gives.0, &gives.1, |id| self.config.shops.max_stack(id));
            if let Some(s) = short.first() {
                return (
                    CraftResult::NotEnoughSpace,
                    format!("the {} tab is {} slot(s) short", net::bag::BAG_TAB_NAMES[s.tab.index()], s.slots),
                );
            }
        }
        (CraftResult::Ok, String::new())
    }

    /// The `(gives, takes)` a craft makes, in [`crate::questroom::shortfall`]'s shape.
    fn craft_room_rows(
        &self,
        recipe: &crate::crafting::Recipe,
        use_additive: bool,
    ) -> (Vec<(u32, u16, store::InventoryType)>, Vec<(u32, u16, store::InventoryType)>) {
        let tab = |id: u32| self.config.tab_for(id).or_else(|| store::InventoryType::for_item(id));
        let mut gives = Vec::new();
        let mut takes = Vec::new();
        if let Some(t) = tab(recipe.result.0) {
            gives.push((recipe.result.0, recipe.result.1.min(u32::from(u16::MAX)) as u16, t));
        }
        for (id, per) in &recipe.ingredients {
            if let Some(t) = tab(*id) {
                takes.push((*id, (*per).min(u32::from(u16::MAX)) as u16, t));
            }
        }
        if use_additive && recipe.additive.0 != 0 {
            if let Some(t) = tab(recipe.additive.0) {
                takes.push((recipe.additive.0, recipe.additive.1.min(u32::from(u16::MAX)) as u16, t));
            }
        }
        (gives, takes)
    }

    /// Why the player is not close enough to a station, or `None` when they are - or when
    /// this server cannot tell.
    ///
    /// **A second opinion, not the gate.** The client refuses to send at all when its own
    /// test fails (`FUN_1401d2f70`), so this only catches a crafted packet. When the session
    /// has no position yet - nobody has moved since the field entry - it does not refuse:
    /// a player who crafts at a station without taking a step is not a cheat, and guessing
    /// here would break the ordinary case to catch the rare one.
    fn station_shortfall(&self, profession: u8) -> Option<String> {
        let chr = self.claimed_character()?;
        let (x, y) = self.last_position?;
        let template = crate::crafting::station_template(profession)?;
        let npcs = self.config.npcs.get(&chr.map_id)?;
        let mut nearest: Option<i32> = None;
        for npc in npcs.iter().filter(|n| n.template_id == template) {
            if crate::crafting::within_station_range(i32::from(x), i32::from(y), i32::from(npc.x), i32::from(npc.cy)) {
                return None;
            }
            let d = (i32::from(npc.x) - i32::from(x)).abs();
            nearest = Some(nearest.map_or(d, |best: i32| best.min(d)));
        }
        Some(match nearest {
            Some(d) => format!("the nearest {} station on this map is {d} px away", crate::crafting::profession_name(profession)),
            None => format!("this map has no {} station", crate::crafting::profession_name(profession)),
        })
    }

    /// Take `count` of an item for a craft, across as many slots as it takes.
    ///
    /// Modelled on `Session::take_quest_item` and deliberately not shared with it: that one
    /// ends with a chat line about a quest, and a craft has already told the player what it
    /// is doing.
    fn take_for_craft(&mut self, character_id: u32, item_id: u32, count: u32) -> Vec<Reply> {
        let Some(inv) = self.config.tab_for(item_id).or_else(|| store::InventoryType::for_item(item_id)) else {
            return Vec::new();
        };
        let mut left = count.min(u32::from(u16::MAX)) as u16;
        let mut out = Vec::new();
        let Ok(rows) = self.store.bag_items(character_id, inv) else { return out };
        for row in rows {
            if left == 0 {
                break;
            }
            if row.item.item_id != item_id {
                continue;
            }
            let take = row.item.kind.quantity().min(left);
            if self.store.remove_item(character_id, inv, row.slot, Some(take)).is_err() {
                continue;
            }
            left -= take;
            let remaining = row.item.kind.quantity() - take;
            out.push(Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: if remaining == 0 {
                    net::inventory::inventory_removed(inv.as_u8() as i8, row.slot as i16)
                } else {
                    net::inventory::inventory_quantity(inv.as_u8() as i8, row.slot as i16, remaining)
                },
                what: format!(
                    "InventoryOperation: {take} x {item_id} out of {inv:?} slot {} - a crafting material",
                    row.slot
                ),
            });
        }
        out
    }

    /// **`Act.1.skill.<n>` - what a turn-in teaches.** This is how the six Crafting Journal
    /// tabs unlock.
    ///
    /// The owner, 2026-09-21: *"After these quest completions, they should unlock the appropriate
    /// crafting menu within the client"*, naming the six starter quests. The client asks for
    /// nothing else: `FUN_1410e8140` greys a tab when its skill's level is below 1, so the
    /// row in `character_crafting` IS the unlock.
    ///
    /// Each of this client's 18 rows is `(9200x000, mastery)` - `1` on the starter quest,
    /// `20` and `50` on its two follow-ups. The skill is learnt at level 1 if it is new
    /// (never reset if it is not), and the mastery is paid through the same path a craft
    /// uses, cap and all.
    pub(super) fn grant_quest_skills(&mut self, quest_id: u32) -> Vec<Reply> {
        let Some(grants) = self.config.quests.get(&quest_id).map(|q| q.complete_skills.clone()) else {
            return Vec::new();
        };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let mut out = Vec::new();
        for (skill_id, exp) in grants {
            let Some(profession) = net::craft::profession_of_skill(skill_id) else {
                // Not a profession. Nothing in this client's data reaches here, and a silent
                // skip is how a new shape would go unnoticed for a month.
                crate::server::log(&format!(
                    "   quest {quest_id}: Act.1.skill grants {skill_id}, which is not one of the six crafting professions - nothing done"
                ));
                continue;
            };
            let name = crate::crafting::profession_name(profession);
            match self.store.learn_profession(chr.id, profession) {
                Ok(true) => {
                    crate::server::log(&format!("   quest {quest_id}: {name} learnt at level 1 (skill {skill_id})"));
                    out.push(self.craft_skill_reply(profession, 1, 0));
                    out.extend(self.notice(format!(
                        "You have learnt {name}. Its tab is now open in the Crafting Journal."
                    )));
                }
                Ok(false) => {}
                Err(e) => {
                    out.extend(self.notice(format!("Quest {quest_id} could not teach you {name}: {e}")));
                    continue;
                }
            }
            out.extend(self.pay_mastery(profession, exp));
        }
        out
    }

    /// **Every crafting quest this character has ALREADY finished, made to count.**
    ///
    /// The owner, 2026-09-21, with the Woodcrafting tab still showing *"Vicious in Henesys is
    /// looking for an apprentice"* on a level-12 character: *"The UI shows this even after
    /// the user has completed the pre-requisite quest."* It would: `Act.1.skill` was read by
    /// nothing until today, so every turn-in before that granted an item, some experience and
    /// no profession - and a completed quest is never turned in again, so the grant could
    /// never arrive on its own.
    ///
    /// This runs once per session, at the claim, **before the login `SetField`** - so the
    /// character record itself carries the skill and the tab is open on the first screen
    /// rather than after a packet nobody would connect to it.
    ///
    /// **The mastery is NOT replayed**, only the learning. `learn_profession` is
    /// `INSERT OR IGNORE`, so this is idempotent however many times it runs; paying the
    /// quest's 1, 20 or 50 mastery again would not be, and a login that quietly adds mastery
    /// is a farm. A profession backfilled this way starts at level 1 with an empty bar; the
    /// exp those old turn-ins would have paid is gone, and `!craft` is how to put it back.
    pub(super) fn reconcile_crafting_quests(&mut self) {
        let Some(claimed) = self.claimed.as_ref() else { return };
        let character_id = claimed.character_id;
        // Only the quests that grant something - 18 of this client's 322, all crafting.
        let granting: Vec<(u32, Vec<(u32, u32)>)> = self
            .config
            .quests
            .iter()
            .filter(|(_, q)| !q.complete_skills.is_empty())
            .map(|(id, q)| (*id, q.complete_skills.clone()))
            .collect();
        for (quest_id, grants) in granting {
            let complete = self
                .store
                .quest_row(character_id, quest_id)
                .ok()
                .flatten()
                .is_some_and(|r| r.state == store::QuestState::Complete);
            if !complete {
                continue;
            }
            for (skill_id, _) in grants {
                let Some(profession) = net::craft::profession_of_skill(skill_id) else { continue };
                if self.store.learn_profession(character_id, profession).unwrap_or(false) {
                    crate::server::log(&format!(
                        "   crafting: character {character_id} finished quest {quest_id} before this server granted professions - {} learnt at level 1 now (its mastery is not replayed)",
                        crate::crafting::profession_name(profession)
                    ));
                }
            }
        }
    }

    /// Pay the mastery a craft earns, level it if it can, and tell the client.
    ///
    /// The client reads the level and the mastery out of **one** `u32` on the skill record
    /// (`net::craft::packed_mastery`), so the whole of "the bar moved" is a `0x0081`.
    pub(super) fn pay_mastery(&mut self, profession: u8, gain: u32) -> Vec<Reply> {
        if gain == 0 {
            return Vec::new();
        }
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let cap = crate::crafting::mastery_level_cap(u32::from(chr.level));
        let Ok(got) = self.store.add_mastery_exp(chr.id, profession, gain, cap, crate::crafting::mastery_exp_needed)
        else {
            return Vec::new();
        };
        if got.level == 0 {
            return Vec::new(); // not learnt; nothing to show
        }
        let name = crate::crafting::profession_name(profession);
        let mut out = vec![self.craft_skill_reply(profession, got.level, got.exp)];
        out.extend(self.notice(crate::crafting::mastery_line(name, gain)));
        if got.levels_gained > 0 {
            out.extend(self.notice(format!("{name} is now level {}.", got.level)));
        } else if got.wasted > 0 {
            // The bar parks at 100% and the rest is discarded - the client says nothing
            // about it, so this server does.
            out.extend(self.notice(format!(
                "{name} is capped at level {} until your character reaches level {}.",
                got.level,
                (got.level + 1) * 5
            )));
        }
        out
    }

    /// The `0x0081` that carries one profession's packed level.
    pub(super) fn craft_skill_reply_for_gm(&self, profession: u8, level: u32, exp: u32) -> Reply {
        self.craft_skill_reply(profession, level, exp)
    }

    fn craft_skill_reply(&self, profession: u8, level: u32, exp: u32) -> Reply {
        let id = net::craft::profession_skill(profession).unwrap_or(0);
        let change = net::skills::SkillChange::Learn(net::skills::Skill::at_level(
            id,
            net::craft::packed_mastery(level, exp),
        ));
        Reply {
            opcode: net::skills::CHANGE_SKILL_RECORD_RESULT,
            // `show_effect` false, for the reason `session/skills.rs` gives: this client's
            // "a skill has been activated" line is not something the real game shows.
            body: net::skills::change_skill_record_result(true, false, &[change]),
            what: format!(
                "ChangeSkillRecordResult: {} ({id}) now level {level}, mastery {exp} - packed as {:#010x}",
                crate::crafting::profession_name(profession),
                net::craft::packed_mastery(level, exp)
            ),
        }
    }

    fn craft_begin_reply(&self, result: net::craft::CraftResult, why: String) -> Reply {
        self.craft_reply(net::craft::craft_begin_result(result), format!("begin {result:?} - {why}"))
    }

    fn craft_complete_reply(&self, result: net::craft::CraftResult, why: String) -> Reply {
        self.craft_reply(net::craft::craft_complete_result(result), format!("complete {result:?} - {why}"))
    }

    fn craft_reply(&self, body: Vec<u8>, why: String) -> Reply {
        Reply { opcode: net::craft::CRAFT_RESULT, body, what: format!("CraftResult: {why}") }
    }
}
