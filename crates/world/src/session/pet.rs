//! Summoning a pet, and keeping it out across field changes.
//!
//! The owner, 2026-09-13: *"I tried summoning the Husky pet, but the pet does not come out."* The
//! double-click is `0x0147` (`u32 tick, u16 Cash-tab slot`); nothing handled it, so the
//! client sent it twice and got nothing. `net::pet` has the two packets; this is the
//! bookkeeping around them.
//!
//! # What a summon is here
//!
//! * The pet at that Cash-tab slot stands up beside the character: `0x0277` to this client
//!   and to everyone on the map, at the position the character last reported, on the
//!   foothold under it.
//! * The item in the Cash tab is re-sent at its slot with `active = 1` and the pet's serial,
//!   so the client can pair the two and draw the item as summoned. The record and every bag
//!   list build the same body through `Session::item_blob_with_cash_sn`, which asks
//!   [`Session::active_pet`] for the flag.
//! * A second double-click on the same pet puts it away; a double-click on a different pet
//!   swaps them, because this client accepts one pet (`net::pet::PET_INDEX`).
//! * On every field entry the pet is sent again, because the client rebuilds its pools on a
//!   `SetField` - the same reason the NPCs and the other players are re-sent.
//!
//! # The unlock
//!
//! The reference server ends this request with `chr.dispose()`, its "exclusive request done"
//! stat change. This codebase's established unlock for an item-window request is the empty
//! `0x0070` with `bExclRequestSent = 1` (`net::inventory::inventory_rejected`), so every path
//! out of here - summoned, put away, or refused - ends with one. **Always answer.**
//!
//! # Movement and commands
//!
//! * The client walks the pet itself and reports it as `0x0202`; [`Session::on_pet_move`]
//!   forwards the path to the map as `0x0278` so other players see it too.
//! * A chat line that is one of the pet's own command words makes the pet act and speak -
//!   [`Session::pet_command_replies`], reading `crate::petcommands`.
//!
//! # Built 2026-09-15, after the two-client run
//!
//! * The pet's own loot request `0x0205` reaches `on_pick_up` (it was UNKNOWN; "Husky does not
//!   loot").
//! * The four skill items and the Pet Name Tag, which ride `0x0116` with the pet's serial -
//!   [`Session::use_pet_skill_item`], [`Session::use_pet_name_tag`]; the state lives in
//!   `store::pets` and rides back in the pet's Cash item (`petSkill` mask, name).
//! * A hat in the pet-equip slot is re-announced to the map like any other worn change
//!   (`Session::on_inventory_move` -> `Session::broadcast_look_change`).
//! * The pet that was out at log-out is out at the next login ([`Session::restore_active_pet`]).
//!
//! # Not built, and said so
//!
//! Dyeing. Feeding, closeness and levelling exist since 2026-09-15 - `on_use_pet_food`,
//! `pet_hunger_tick`, `crate::petlevel` - and the pet's level picks its command band.
//! (**Show Pet Info** in Character Info
//! is the client's own: its window builds a pet list when it opens, so one opened before the
//! summon stays grey - `research/pets-loot-skills-name-relogin-2026-09-15.md` §6.)

use super::*;

/// The worn slot a pet's equip goes to, as the Deco tab numbers it: the Blue Top Hat went
/// `Deco slot 1 -> -114` (`world-ch0.log` 2026-09-15 02:59:25, `0x0107`), i.e. cash worn slot
/// 114 = body slot 14 plus the cash base. One pet, one slot. **[L]** The move handler
/// watches the whole worn set rather than this one slot (2026-09-18); the Character Info
/// window's pet cell reads it (`session::charinfo`).
pub(super) const PET_EQUIP_WORN_SLOT: u8 = 114;

/// One of this character's pets, as the session handles it: where its item is, what it is,
/// and **which one it is** - `pet_id` is the `pets` row its name and vitals live on, and the
/// low half of the serial the client pairs by. Two Huskies are two of these with two ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ActivePet {
    /// The Cash-tab slot it lives in, 1-based.
    pub slot: u16,
    pub item_id: u32,
    /// `store::pets` row id. The owner, 2026-09-16: *"The pets should in the background have
    /// different ids to identify them apart."*
    pub pet_id: u32,
}

impl Session {
    /// **`0x0198`, after every SetField: the long-range pickup boxes.** The client keeps them
    /// in two globals that are `(0,0,0,0)` until a server says otherwise, and consults them
    /// only for a pet whose item carries `wonderGrade 6` - which every pet's item does
    /// (`net::bag::PET_WONDER_GRADE_VACUUM`; the owner, 2026-09-17: the in-range vacuum is free).
    /// So this is the other half of that grade: without it every pet would sweep a box of no
    /// size and pick up nothing at all, which on screen is "the vacuum does not work".
    ///
    /// Sent whether or not a pet is out (36 bytes; the box is per client, not per pet), and
    /// on every SetField because the keymap and the SP pools ride the same way and for the
    /// same reason: a portal walk must not be a second way to lose it.
    pub(super) fn pet_pickup_range_reply(&self) -> Reply {
        let [l, t, r, b] = net::pet::PET_VACUUM_BOX;
        Reply {
            opcode: net::pet::PET_PICKUP_RANGE,
            body: net::pet::pet_pickup_range(net::pet::PET_VACUUM_BOX, net::pet::PET_VACUUM_BOX, &[]),
            what: format!(
                "PetPickupRange: the long-range box ({l},{t})..({r},{b}) around a pet whose item \
                 says wonderGrade 6 - every pet's does; the in-range vacuum is free. Without \
                 this the client's copy is (0,0,0,0) and nothing is picked up."
            ),
        }
    }

    /// **`0x0202`: the pet walked, so everyone on the map is told.** The owner, 2026-09-13:
    /// *"broadcast player pet movement so other people can see pets moving even if it is not
    /// their own."*
    ///
    /// The path block is forwarded byte for byte inside `0x0278` - `net::pet::pet_move_broadcast`
    /// - which is the same rule a remote character's move follows: the client that owns the pet
    /// has already decided where it walked.
    ///
    /// **It is published to the map and NOT returned to the sender.** The owner's client drew
    /// the walk itself; sending it back would fight its own simulation, exactly as
    /// `MOB_CHANGE_CONTROLLER` must not be doubled. The client expects no answer either - it is
    /// a report, like `0x00D9`.
    pub(super) fn on_pet_move(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        if self.active_pet.is_none() {
            return Vec::new();
        }
        // `Config::broadcast_pets` is on by default; off is the owner-local fallback.
        if !self.config.broadcast_pets {
            return Vec::new();
        }
        let Some(out) = net::pet::pet_move_broadcast(chr.id, body) else { return Vec::new() };
        let reply = Reply {
            opcode: net::pet::PET_MOVE,
            body: out,
            what: format!("PetMove: {}'s pet walked; the path forwarded to the map", chr.name),
        };
        self.bus().publish(self.subscriber, chr.map_id, reply, None);
        Vec::new()
    }

    /// **A chat line that is one of the pet's commands.** The owner, 2026-09-13: *"They will still
    /// happen as regular chat messages in the game, but if those messages match as one of the
    /// pet commands, then the pet should respond accordingly."*
    ///
    /// So this does not replace the chat line - `say_out_loud` has already built it - it adds
    /// the pet's answer beside it. Nothing when no pet is out, or when the text is not one of
    /// that pet's words, which is every ordinary sentence.
    ///
    /// **The pet is always level 1** here: this server keeps no closeness, so the level band the
    /// table picks is always the first. `crate::petcommands` records the `inc` the entry would
    /// have earned so that adding closeness later is a change in one place.
    pub(super) fn pet_command_replies(&mut self, text: &str) -> Vec<Reply> {
        let Some(active) = self.active_pet else { return Vec::new() };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let roll = self.rng.next();
        let st = self.pet_state(Some(active.pet_id));
        let Some(r) = self.config.pet_commands.respond(active.item_id, u32::from(st.level), text, roll)
        else {
            return Vec::new();
        };
        // A trick that lands earns the entry's closeness (the wiki's +1..+3), and the level
        // follows the table - up only. The Cash item goes out again so the panel agrees.
        let mut earned = Vec::new();
        if r.success && r.inc > 0 {
            let closeness = st.closeness + r.inc;
            let level = crate::petlevel::level_after(st.level, closeness);
            if self.store.set_pet_vitals(active.pet_id, level, closeness, st.fullness).is_ok() {
                crate::server::log(&format!(
                    "   pet: {:?} earned pet {} +{} closeness -> {closeness}, level {} -> {level}",
                    text.trim(), active.item_id, r.inc, st.level
                ));
                earned.push(self.pet_item_refresh(&chr, active, true));
                if level > st.level {
                    earned.extend(self.pet_level_up_replies(&chr, st.level, level));
                }
            }
        }
        let reply = Reply {
            opcode: net::pet::PET_ACTION,
            body: net::pet::pet_action(chr.id, r.index, r.success, &r.text),
            what: format!(
                "PetAction: {:?} -> pet {} interact {} {} ({}) says {:?}",
                text.trim(),
                active.item_id,
                r.index,
                if r.success { "succeeds" } else { "fails" },
                r.act,
                r.text
            ),
        };
        // The owner always gets its own pet's answer (the return below); the map gets it too
        // unless pets are owner-local. `Config::broadcast_pets`.
        if self.config.broadcast_pets {
            self.bus().publish(self.subscriber, chr.map_id, reply.clone(), None);
        }
        let mut out = vec![reply];
        out.extend(earned);
        out
    }

    /// `0x0147`: a double-click on a pet in the Cash tab.
    pub(super) fn on_pet_activate(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let unlock = Reply {
            opcode: net::inventory::INVENTORY_OPERATION,
            body: net::inventory::inventory_rejected(),
            what: "InventoryOperation: no change; bExclRequestSent so the pet request is closed".to_string(),
        };
        let Some(req) = net::pet::parse_pet_activate(body) else {
            crate::server::log(&format!(
                "   pet: 0x0147 body of {} bytes is not `u32 tick, u16 slot` - unlocking and doing nothing",
                body.len()
            ));
            return vec![unlock];
        };
        let at_slot = self
            .store
            .bag(chr.id)
            .ok()
            .and_then(|bag| bag.items_in(store::InventoryType::Cash).find(|i| i.slot == req.slot).map(|i| i.item));
        let Some(item) = at_slot.filter(|i| net::inventory::is_pet(i.item_id)) else {
            crate::server::log(&format!(
                "   pet: Cash slot {} holds {} - not a pet; unlocking and doing nothing",
                req.slot,
                at_slot.map(|i| format!("item {}", i.item_id)).unwrap_or_else(|| "nothing".to_string())
            ));
            return vec![unlock];
        };

        // **Which pet.** The row carries its number from `add_item`; a row that somehow has
        // none is numbered now, so a pet never acts under another pet's identity.
        let pet_id = match item.pet_id {
            Some(id) => id,
            None => match self.store.pet_id_at(chr.id, store::InventoryType::Cash, req.slot) {
                Ok(Some(id)) => id,
                _ => {
                    crate::server::log(&format!(
                        "   pet: Cash slot {} holds pet {} with no pet id and none could be assigned; unlocking",
                        req.slot, item.item_id
                    ));
                    return vec![unlock];
                }
            },
        };
        let next = ActivePet { slot: req.slot, item_id: item.item_id, pet_id };

        let mut out = Vec::new();
        if let Some(active) = self.active_pet.take() {
            // Put the current one away first - either this is the toggle, or a swap.
            let gone = Reply {
                opcode: net::pet::PET_ACTIVATED,
                body: net::pet::pet_deactivated(chr.id),
                what: format!("PetActivated: pet {} (Cash slot {}) put away for {}", active.item_id, active.slot, chr.name),
            };
            // The owner always gets the put-away (below). Other players were only told the pet
            // was there if `broadcast_pets` is on, so only then do they need the removal; the
            // companion list is cleared regardless, since it is free and keeps arrivals clean.
            if self.config.broadcast_pets {
                self.bus().publish(self.subscriber, chr.map_id, gone.clone(), None);
            }
            self.bus().set_companions(self.subscriber, Vec::new());
            // Put away in the store too, or the next login would summon it again.
            let _ = self.store.set_pet_active(chr.id, active.pet_id, false);
            self.pet_hunger_due_ms = None;
            out.push(gone);
            out.push(self.pet_item_refresh(&chr, active, false));
            if active.pet_id == pet_id {
                out.push(unlock);
                return out;
            }
        }

        let pet = self.field_pet(&chr, next);
        let up = Reply {
            opcode: net::pet::PET_ACTIVATED,
            body: net::pet::pet_activated(chr.id, &pet),
            what: format!(
                "PetActivated: {} ({}) summoned beside {} at ({}, {}) fh {} - Cash slot {}",
                pet.name, pet.item_id, chr.name, pet.x, pet.y, pet.foothold, req.slot
            ),
        };
        // The summon goes to the map and the pet travels with the owner to whoever arrives
        // after; with pets owner-local (`Config::broadcast_pets` off) neither happens.
        if self.config.broadcast_pets {
            self.bus().publish(self.subscriber, chr.map_id, up.clone(), None);
            self.bus().set_companions(self.subscriber, vec![up.clone()]);
        }
        out.push(up);
        self.active_pet = Some(next);
        self.pet_hunger_due_ms = Some(self.clock_ms + net::petfood::PET_HUNGER_INTERVAL_MS);
        self.pet_overfeeds = 0;
        // Remembered across a re-login: `restore_active_pet` reads this at the next claim.
        let _ = self.store.set_pet_active(chr.id, pet_id, true);
        out.push(self.pet_item_refresh(&chr, next, true));
        out.push(unlock);
        out
    }

    /// **The packets that travel with this player** - `crate::broadcast::Presence::companions`.
    /// The summoned pet's `0x0277`, so a player arriving on the map after the summon gets
    /// the pet right behind the owner's spawn; nothing when no pet is out.
    pub(super) fn pet_companions(&self, chr: &net::opcode::Character) -> Vec<Reply> {
        // Owner-local pets travel with nobody. `Config::broadcast_pets`.
        if !self.config.broadcast_pets {
            return Vec::new();
        }
        let Some(active) = self.active_pet else { return Vec::new() };
        let pet = self.field_pet(chr, active);
        vec![Reply {
            opcode: net::pet::PET_ACTIVATED,
            body: net::pet::pet_activated(chr.id, &pet),
            what: format!(
                "PetActivated: {} ({}) beside {} - sent to a player who arrived after the summon",
                pet.name, pet.item_id, chr.name
            ),
        }]
    }

    /// The pet again, after a `SetField` rebuilt the client's pools. Nothing when none is out.
    ///
    /// **For the owner only.** Everyone else on the map already got it: `announce_field_entry`
    /// runs first and its `Presence` carries the pet as a companion, so `Bus::enter_field`
    /// posts the owner's spawn and then the pet to the field. Publishing it here as well sent
    /// every other player the pet twice per field entry.
    pub(super) fn pet_entry_replies(&mut self, chr: &net::opcode::Character) -> Vec<Reply> {
        let Some(active) = self.active_pet else { return Vec::new() };
        let pet = self.field_pet(chr, active);
        // The rest of the job is done once the client is demonstrably live - see
        // `pet_settle_replies`. Armed here, on every field entry that has a pet out.
        self.pet_settle_pending = true;
        vec![
            Reply {
                opcode: net::pet::PET_ACTIVATED,
                body: net::pet::pet_activated(chr.id, &pet),
                what: format!(
                    "PetActivated: {} ({}) back beside {} after the field entry, at ({}, {})",
                    pet.name, pet.item_id, chr.name, pet.x, pet.y
                ),
            },
            // **The post-summon item write, without which the pet spawns sad and inert.** The owner,
            // 2026-09-17: *"whenever the pet first spawn in on either login field load or map
            // change, it appears sad and non-functional ... until it is re-summoned ... or
            // feeding the pet."* The `0x0277` above carries neither fullness nor the pet's
            // learned skills nor its wonderGrade (`pet_activated` sends `wonderGrade 0`), so a
            // pet summoned by that alone reads as hungry and never vacuums. `CPet` takes its
            // real state from the Cash item, and it re-reads it on a `0x0070` Add to the pet's
            // slot *after* it is active - which is exactly what makes a re-summon
            // (`on_pet_activate`) and a feed (`on_use_pet_food`) both fix it. The item is in the
            // SetField bag restore too, but that write lands before the pet is active and does
            // not trigger the re-read. So the field entry sends the same refresh those two do.
            self.pet_item_refresh(chr, active, true),
        ]
    }

    /// **The pet is put away and summoned again, with its item re-sent, on the first move
    /// after a field entry** - once per entry, nothing at all otherwise.
    ///
    /// The owner, 2026-09-18: *"Pets still do not function upon initial login or map change. While
    /// they no longer look sad/droopy upon initial spawn, the vacuum functionality does not
    /// work until the pet is re-summoned or fed at least once with pet food. Can we have the
    /// vacuum functionality always be present when the pet is summoned please?"*
    ///
    /// What the two working cases have in common is WHEN they land: a re-summon
    /// (`on_pet_activate`) and a feed (`on_use_pet_food`) both write the pet's item to a pet
    /// the client has finished building, on a field it has finished loading. The field-entry
    /// batch (`pet_entry_replies`) sends the same summon and the same item write, and it fixed
    /// the sad face but not the vacuum - so whatever `CPet` reads the vacuum grade from, it
    /// reads it in a state the SetField batch is too early for. [I] on the mechanism; [L] on
    /// the two sequences that work and the one that does not, all three the owner's screen.
    ///
    /// So the sequence that works is sent again, at a moment that is provably after the
    /// field is live: the client's own first `0x00D9` move after the entry. It is the
    /// re-summon a skill item uses (`resummon_for_owner`: put-away, summon - one blink) and
    /// then the item write, for the owner only; the map already has the pet from the entry
    /// and is told nothing. Idempotent: the flag is cleared before anything is sent, and a
    /// session with no pet out sends nothing.
    pub(super) fn pet_settle_replies(&mut self) -> Vec<Reply> {
        if !self.pet_settle_pending {
            return Vec::new();
        }
        self.pet_settle_pending = false;
        let Some(active) = self.active_pet else { return Vec::new() };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let mut out = self.resummon_for_owner(&chr, active);
        out.push(self.pet_item_refresh(&chr, active, true));
        out
    }

    /// Whether the pet numbered `pet_id` is the one this session has out - the `active` byte
    /// of its body. Per pet: the other Husky's byte stays 0.
    pub(super) fn pet_is_active(&self, pet_id: u32) -> bool {
        self.active_pet.is_some_and(|p| p.pet_id == pet_id)
    }

    /// The item id of the pet that is out, for a caller that only wants to know what species
    /// is on the field. Tests, today.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) fn active_pet_item(&self) -> Option<u32> {
        self.active_pet.map(|p| p.item_id)
    }

    /// What the store knows about one pet: its name, its learned skills and whether it was
    /// out. Defaults when it has never been touched, and for `None` - a pet item that has not
    /// been numbered. `store::pets`.
    pub(super) fn pet_state(&self, pet_id: Option<u32>) -> store::PetState {
        pet_id
            .and_then(|id| self.store.pet_state(id).ok())
            .unwrap_or_else(store::PetState::fresh)
    }

    /// The four moving fields of the pet's Cash item, from the store.
    pub(super) fn pet_vitals(&self, pet_id: Option<u32>) -> net::bag::PetVitals {
        let st = self.pet_state(pet_id);
        net::bag::PetVitals {
            level: st.level,
            closeness: u16::try_from(st.closeness).unwrap_or(u16::MAX),
            fullness: st.fullness,
            skills: st.skills,
        }
    }

    /// **`0x0112`: Pet Food on the pet that is out.** The owner, 2026-09-15: *"Using a pet food
    /// should recover the current active pet's fullness by 30 and their closeness by 1."*
    /// `net::petfood` has the packet (never captured - the opcode is read off the client's
    /// item-use switch) and the numbers; `crate::petlevel::feed` has the overfeed rule.
    ///
    /// The numbers reach the screen as the pet's Cash item, re-sent (Show Pet Info and the
    /// tooltip read it there). The **eating animation** is `0x027E` type 2 with the food's id
    /// (`net::pet::pet_ate`), to the owner and the map - the owner, 2026-09-16: *"I do want the
    /// eating animation to play for the client and other players."* A level gained on the way
    /// adds the pet level-up flash, [`Session::pet_level_up_replies`].
    ///
    /// Refusals answer with the opcode's unlock and a notice: no pet out, the slot does not
    /// hold that food, or the item is not a pet food at all.
    pub(super) fn on_use_pet_food(&mut self, body: &[u8]) -> Vec<Reply> {
        let opcode = net::petfood::CLIENT_USE_PET_FOOD;
        let unlock = || crate::mesodrop::unlock_unhandled_latching_request(opcode);
        let Some(chr) = self.claimed_character() else { return unlock() };
        let Some(req) = net::petfood::parse_use_pet_food(body) else {
            crate::server::log(&format!("   pet food: a {} byte 0x0112 body did not decode; unlock only", body.len()));
            return unlock();
        };
        let holding = self
            .store
            .bag_items(chr.id, store::InventoryType::Use)
            .ok()
            .into_iter()
            .flatten()
            .find(|r| r.slot == req.slot)
            .map(|r| (r.item.item_id, match r.item.kind { store::ItemKind::Bundle { quantity } => quantity, _ => 1 }));
        let Some((item_id, held)) = holding.filter(|(id, _)| *id == req.item_id && net::petfood::is_pet_food(*id)) else {
            crate::server::log(&format!(
                "   pet food: character {} asked to feed {} from Use slot {}, which holds {:?}. Refused",
                chr.id, req.item_id, req.slot, holding
            ));
            return unlock();
        };
        let Some(active) = self.active_pet else {
            let mut out = self.notice("Summon a pet before feeding it.".to_string());
            out.extend(unlock());
            return out;
        };
        let st = self.pet_state(Some(active.pet_id));
        let (fullness, closeness, overfeeds) = crate::petlevel::feed(st.fullness, st.closeness, self.pet_overfeeds);
        self.pet_overfeeds = overfeeds;
        let level = crate::petlevel::level_after(st.level, closeness);
        if let Err(e) = self.store.set_pet_vitals(active.pet_id, level, closeness, fullness) {
            crate::server::log(&format!("   pet food: storing the vitals failed: {e}; the food is kept"));
            return unlock();
        }
        let _ = self.store.remove_item(chr.id, store::InventoryType::Use, req.slot, Some(1));
        crate::server::log(&format!(
            "   pet food: character {} fed pet {} with {item_id}: fullness {} -> {fullness}, closeness {} -> {closeness}, level {} -> {level}{}",
            chr.id, active.item_id, st.fullness, st.closeness, st.level,
            if overfeeds > 0 { format!(" (overfeed #{overfeeds})") } else { String::new() }
        ));
        let mut out = unlock();
        out.extend(self.stack_change_replies(store::InventoryType::Use, req.slot, held.saturating_sub(1)));
        out.push(self.pet_item_refresh(&chr, active, true));
        // The pet eats, on every screen it is on. The food id is 0 on purpose: a real id makes
        // the client draw the auto-feed balloon "Yum, yum! <food> x<count-1> left!", and this
        // is a hand feed (the owner, 2026-09-18; net::pet::pet_ate has the listing).
        let ate = Reply {
            opcode: net::pet::PET_ACTION_COMMAND,
            body: net::pet::pet_ate(chr.id, net::pet::PET_FOOD_NONE),
            what: format!("PetActionCommand: {}'s pet eats ({item_id}, sent as food id 0: the animation without the auto-feed balloon) - type 2", chr.name),
        };
        if self.config.broadcast_pets {
            self.bus().publish(self.subscriber, chr.map_id, ate.clone(), None);
        }
        out.push(ate);
        if level > st.level {
            out.extend(self.pet_level_up_replies(&chr, st.level, level));
        }
        out
    }

    /// **The pet levelled up: the flash, for the owner and the map.** The owner, 2026-09-16: *"When
    /// closeness levels up, it should also play an animation to the client and other players in
    /// the map."* `UserEffect` arm 9 with subtype 0 - `Effect/PetEff.img/Basic/LevelUp` - as
    /// `0x02D1` to the owner and `0x02AF` to everyone else (`net::pet::pet_level_up_local` /
    /// `_remote`). The returned replies are the owner's; the map's is published from here.
    fn pet_level_up_replies(&self, chr: &net::opcode::Character, from: u8, to: u8) -> Vec<Reply> {
        crate::server::log(&format!("   pet: character {}'s pet levelled {from} -> {to}; the LevelUp effect goes out", chr.id));
        if self.config.broadcast_pets {
            self.bus().publish(
                self.subscriber,
                chr.map_id,
                Reply {
                    opcode: net::stats::USER_EFFECT_REMOTE,
                    body: net::pet::pet_level_up_remote(chr.id),
                    what: format!("UserEffectRemote: {}'s pet level {from} -> {to} - effect 9 (pet), subtype 0 (LevelUp)", chr.name),
                },
                None,
            );
        }
        vec![Reply {
            opcode: net::stats::USER_EFFECT_LOCAL,
            body: net::pet::pet_level_up_local(),
            what: format!("UserEffectLocal: your pet's level {from} -> {to} - effect 9 (pet), subtype 0 (Effect/PetEff.img/Basic/LevelUp)"),
        }]
    }

    /// **Every five minutes a summoned pet loses one fullness; at zero it goes home.** The owner,
    /// 2026-09-15: *"Pets should decrease their fullness by 1 every 5 minutes."* The wiki's
    /// starvation rule rides with it: `-1` closeness, and the pet is put away - for the owner
    /// with a notice, for the map, and in the store, so the next login leaves it in the bag.
    ///
    /// `now_ms` is the session clock: the timer runs while the pet is out on this connection
    /// and is re-armed by every summon. A pet that is put away and re-summoned starts a fresh
    /// five minutes; that is a simplification, said out loud.
    pub(super) fn pet_hunger_tick(&mut self, now_ms: u64) -> Vec<Reply> {
        let Some(active) = self.active_pet else { return Vec::new() };
        let Some(due) = self.pet_hunger_due_ms else {
            self.pet_hunger_due_ms = Some(now_ms + net::petfood::PET_HUNGER_INTERVAL_MS);
            return Vec::new();
        };
        if now_ms < due {
            return Vec::new();
        }
        self.pet_hunger_due_ms = Some(now_ms + net::petfood::PET_HUNGER_INTERVAL_MS);
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let st = self.pet_state(Some(active.pet_id));
        let fullness = st.fullness.saturating_sub(1);
        if fullness > 0 {
            let _ = self.store.set_pet_vitals(active.pet_id, st.level, st.closeness, fullness);
            crate::server::log(&format!(
                "   pet: character {}'s pet {} is hungrier: fullness {} -> {fullness}",
                chr.id, active.item_id, st.fullness
            ));
            return vec![self.pet_item_refresh(&chr, active, true)];
        }
        // Starved: one closeness gone, and home it goes.
        let closeness = st.closeness.saturating_sub(1);
        let _ = self.store.set_pet_vitals(active.pet_id, st.level, closeness, 0);
        let _ = self.store.set_pet_active(chr.id, active.pet_id, false);
        self.active_pet = None;
        self.pet_hunger_due_ms = None;
        crate::server::log(&format!(
            "   pet: character {}'s pet {} STARVED - fullness 0, closeness {} -> {closeness}; sent home",
            chr.id, active.item_id, st.closeness
        ));
        let gone = Reply {
            opcode: net::pet::PET_ACTIVATED,
            body: net::pet::pet_deactivated(chr.id),
            what: format!("PetActivated: {} went home hungry (fullness 0)", self.pet_name(Some(active.pet_id), active.item_id)),
        };
        if self.config.broadcast_pets {
            self.bus().publish(self.subscriber, chr.map_id, gone.clone(), None);
        }
        self.bus().set_companions(self.subscriber, Vec::new());
        let mut out = self.notice(format!("{} is starving and went back home.", self.pet_name(Some(active.pet_id), active.item_id)));
        out.push(gone);
        out.push(self.pet_item_refresh(&chr, active, false));
        out
    }

    /// The name the pet goes by: the player's, from a Pet Name Tag, or the item's own.
    pub(super) fn pet_name(&self, pet_id: Option<u32>, item_id: u32) -> String {
        self.pet_state(pet_id)
            .name
            .unwrap_or_else(|| self.config.item_names.get(&item_id).cloned().unwrap_or_default())
    }

    /// **A pet that was out when the player left is out again when they come back.** The owner,
    /// 2026-09-15: *"pets that were previously summoned by user should keep their state."*
    ///
    /// Called once, at claim time, before the login `SetField` is built - so the Cash item in
    /// that record already carries `active = 1`, the owner's first field entry re-sends the
    /// `0x0277` (`pet_entry_replies`), and everyone already there is handed it as a companion
    /// (`pet_companions`). Nothing is sent from here; this only sets the session's word.
    ///
    /// A stored pet whose item is no longer in the Cash tab is put away in the store rather
    /// than summoned from nowhere.
    pub(super) fn restore_active_pet(&mut self) {
        let Some(character_id) = self.claimed.as_ref().map(|c| c.character_id) else { return };
        let Ok(Some(row)) = self.store.active_pet(character_id) else { return };
        let (pet_id, item_id) = (row.pet_id, row.item_id);
        // **By number, not by species**: with two Huskies in the bag, the one that was out is
        // the one whose row carries this id.
        let slot = self
            .store
            .bag_items(character_id, store::InventoryType::Cash)
            .ok()
            .into_iter()
            .flatten()
            .find(|r| r.item.pet_id == Some(pet_id))
            .map(|r| r.slot);
        match slot {
            Some(slot) => {
                self.active_pet = Some(ActivePet { slot, item_id, pet_id });
                crate::server::log(&format!(
                    "   pet: character {character_id} had pet {item_id} #{pet_id} (Cash slot {slot}) out \
                     when they last left; it comes back out on this login"
                ));
            }
            None => {
                let _ = self.store.set_pet_active(character_id, pet_id, false);
                crate::server::log(&format!(
                    "   pet: character {character_id}'s stored active pet {item_id} #{pet_id} is no longer \
                     in the Cash tab; put away in the store rather than summoned from nowhere"
                ));
            }
        }
    }

    /// The pet an item is meant for: the one the request's serial names (`net::pet::pet_serial`
    /// - the **pet id** is its low half, the character its high), or the pet that is out when
    /// no serial came. `None` when it is not in the Cash tab. A name tag used on one Husky
    /// names that Husky.
    pub(super) fn pet_named_by(&self, chr: &net::opcode::Character, serial: Option<u64>) -> Option<ActivePet> {
        // A client may hand back the generic BAG serial (mark, character, tab, slot) for a
        // pet its field-entry restore sent before 2026-09-18, or the pet serial (character,
        // pet id) every path sends now. Both name one pet.
        if let Some((tab, slot)) = serial.and_then(|sn| super::cashshop::bag_serial_slot(sn, chr.id)) {
            return self
                .store
                .bag_items(chr.id, tab)
                .ok()
                .into_iter()
                .flatten()
                .find(|r| r.slot == slot && net::inventory::is_pet(r.item.item_id))
                .and_then(|r| r.item.pet_id.map(|pet_id| ActivePet { slot: r.slot, item_id: r.item.item_id, pet_id }));
        }
        let pet_id = match serial {
            Some(sn) if (sn >> 32) as u32 == chr.id => (sn & 0xFFFF_FFFF) as u32,
            Some(_) => return None,
            None => self.active_pet?.pet_id,
        };
        self.store
            .bag_items(chr.id, store::InventoryType::Cash)
            .ok()
            .into_iter()
            .flatten()
            .find(|r| r.item.pet_id == Some(pet_id) && net::inventory::is_pet(r.item.item_id))
            .map(|r| ActivePet { slot: r.slot, item_id: r.item.item_id, pet_id })
    }

    /// **A pet skill item** - Auto HP, Auto MP, Auto Move, Expanded Auto Move - used on a pet.
    /// The owner, 2026-09-15: *"the owner just tried to add the Auto HP, Auto MP and Auto Move skill onto
    /// Husky, it doesn't work."* They arrived on `0x0116` with the pet's serial and were answered
    /// as "not a reset scroll" - unlock only, item kept (`world-ch0.log` 02:56:50..57).
    ///
    /// The skill is a bit in the pet ITEM's `petSkill` mask (`net::bag::pet_item_with_state`),
    /// so learning is: OR the bit into the store, use the skill item up, and re-send the pet's
    /// Cash item with the new mask. **[I]**: whether the client's auto-HP/auto-loot logic reads
    /// the refreshed item live or only at `CPet::Init` is unmeasured, so when the pet is out it
    /// is put away and summoned again for the owner in the same reply - one summon animation,
    /// and a pet that has certainly re-read its item. The plan step names the falsifier.
    pub(super) fn use_pet_skill_item(&mut self, opcode: u16, req: &net::cashitem::UseCashItem) -> Vec<Reply> {
        let unlock = || crate::mesodrop::unlock_unhandled_latching_request(opcode);
        let Some(chr) = self.claimed_character() else { return unlock() };
        let Some(bit) = net::bag::pet_skill_bit_for_item(req.item_id) else { return unlock() };
        let Some(pet) = self.pet_named_by(&chr, req.pet_serial) else {
            crate::server::log(&format!(
                "   pet skill: character {} used {} for a pet this server cannot find (serial {:?}); kept",
                chr.id, req.item_id, req.pet_serial
            ));
            return self.cash_item_notice_for(opcode, "That skill needs a pet to learn it. Nothing was used up.".to_string());
        };
        let pet_item = pet.item_id;
        let skills = match self.store.learn_pet_skill(pet.pet_id, bit) {
            Ok(m) => m,
            Err(e) => {
                crate::server::log(&format!("   pet skill: storing the skill failed: {e}; the item is kept"));
                return unlock();
            }
        };
        let _ = self.store.remove_item(chr.id, store::InventoryType::Cash, req.slot, Some(1));
        crate::server::log(&format!(
            "   pet skill: character {} taught pet {pet_item} skill bit {bit:#06x} with item {} - mask now {skills:#06x}; the item is used up",
            chr.id, req.item_id
        ));
        let mut out = unlock();
        out.extend(self.stack_change_replies(store::InventoryType::Cash, req.slot, 0));
        let out_now = self.pet_is_active(pet.pet_id);
        out.push(self.pet_item_refresh(&chr, pet, out_now));
        if out_now {
            out.extend(self.resummon_for_owner(&chr, pet));
        }
        out
    }

    /// **A Pet Name Tag.** The owner, 2026-09-15: *"I also tried to rename Husky into Dummy using
    /// the Pet Name Tag."* `0x0116` with the pet's serial and the name; answered as a reset
    /// scroll, so nothing happened. The name is stored per pet (`store::pets`), the pet's Cash
    /// item is re-sent carrying it, the tag is used up, and `0x027B` renames the pet on every
    /// screen it is on - the owner's and the map's (`net::pet::PET_NAME_CHANGED`).
    ///
    /// The wire field is 13 bytes with a terminator, so the name is cut to 12 bytes on a
    /// character boundary rather than refused - a long name losing its tail is what the
    /// client's own field would do.
    pub(super) fn use_pet_name_tag(&mut self, opcode: u16, req: &net::cashitem::UseCashItem) -> Vec<Reply> {
        let unlock = || crate::mesodrop::unlock_unhandled_latching_request(opcode);
        let Some(chr) = self.claimed_character() else { return unlock() };
        let mut name = req.text.as_deref().map(str::trim).unwrap_or_default().to_string();
        if name.is_empty() {
            return self.cash_item_notice_for(opcode, "Type a name for the pet first. Nothing was used up.".to_string());
        }
        while name.len() > net::bag::PET_NAME_LEN - 1 {
            name.pop();
        }
        let Some(pet) = self.pet_named_by(&chr, req.pet_serial) else {
            return self.cash_item_notice_for(opcode, "That tag needs a pet to name. Nothing was used up.".to_string());
        };
        let pet_item = pet.item_id;
        if let Err(e) = self.store.set_pet_name(pet.pet_id, &name) {
            crate::server::log(&format!("   pet name: storing {name:?} failed: {e}; the tag is kept"));
            return unlock();
        }
        let _ = self.store.remove_item(chr.id, store::InventoryType::Cash, req.slot, Some(1));
        crate::server::log(&format!("   pet name: character {} named pet {pet_item} {name:?}; the tag is used up", chr.id));
        let mut out = unlock();
        out.extend(self.stack_change_replies(store::InventoryType::Cash, req.slot, 0));
        let out_now = self.pet_is_active(pet.pet_id);
        out.push(self.pet_item_refresh(&chr, pet, out_now));
        if out_now {
            let renamed = Reply {
                opcode: net::pet::PET_NAME_CHANGED,
                body: net::pet::pet_name_changed(chr.id, &name),
                what: format!("PetNameChanged: {}'s pet is now called {name:?}", chr.name),
            };
            if self.config.broadcast_pets {
                self.bus().publish(self.subscriber, chr.map_id, renamed.clone(), None);
                // Whoever arrives next is handed the pet under its new name.
                let pet = self.field_pet(&chr, pet);
                self.bus().set_companions(
                    self.subscriber,
                    vec![Reply {
                        opcode: net::pet::PET_ACTIVATED,
                        body: net::pet::pet_activated(chr.id, &pet),
                        what: format!("PetActivated: {} beside {} - for a later arrival", pet.name, chr.name),
                    }],
                );
            }
            out.push(renamed);
        }
        out
    }

    /// Put the owner's pet away and summon it again **on the owner's screen only** - a fresh
    /// `CPet::Init` that has read the pet's Cash item as it is now. The map is not told: the
    /// remote pet has no item to re-read.
    fn resummon_for_owner(&self, chr: &net::opcode::Character, which: ActivePet) -> Vec<Reply> {
        let pet = self.field_pet(chr, which);
        vec![
            Reply {
                opcode: net::pet::PET_ACTIVATED,
                body: net::pet::pet_deactivated(chr.id),
                what: format!("PetActivated: {} put away for a moment so it re-reads its item", pet.name),
            },
            Reply {
                opcode: net::pet::PET_ACTIVATED,
                body: net::pet::pet_activated(chr.id, &pet),
                what: format!("PetActivated: {} back beside {} with its item as it is now", pet.name, chr.name),
            },
        ]
    }

    /// The pet as `0x0277` describes it: its name, the pairing serial, and the spot under the
    /// character. With no position reported yet it stands at the origin, which the client's
    /// own physics then drops onto whatever is below.
    fn field_pet(&self, chr: &net::opcode::Character, which: ActivePet) -> net::pet::FieldPet {
        let (x, y) = self.last_position.unwrap_or((0, 0));
        let landing = self.config.footholds.landing(chr.map_id, x, y);
        net::pet::FieldPet {
            item_id: which.item_id,
            name: self.pet_name(Some(which.pet_id), which.item_id),
            serial: net::pet::pet_serial(chr.id, which.pet_id).get(),
            x,
            y: landing.as_ref().map_or(y, |l| l.y),
            move_action: self.config.pet_move_action.unwrap_or(0),
            foothold: landing.map_or(0, |l| u16::try_from(l.foothold).unwrap_or(0)),
        }
    }

    /// The Cash-tab item re-sent at its slot with its `active` byte as `active` says and the
    /// pairing serial set. An Add at an occupied slot replaces what the client holds there.
    /// **Mode 5, not mode 0.** Every call here re-sends an item that is already in the slot,
    /// and a mode-0 add is what the client marks as NEW - the highlighted cell in the Cash
    /// tab. The owner, 2026-09-18: *"Whenever I join the map for the first time, I get a new item
    /// notification in my cash tab of my current summoned pet. If the item is not new, please
    /// do not highlight it."* The rest of the bag is restored on field entry with mode 5 and
    /// carries no mark; this one packet was the odd one out, sent as mode 0 since 2026-09-15.
    /// Both modes go through the same reader and the same store
    /// (`net::inventory::MODE_SET_QUIET` has the listing); mode 0 only adds the quest hook and
    /// the before/after slot map that the NEW mark and the quick slot come from. **[I]** that
    /// `CPet` re-reads the item's active byte from a mode-5 store the same as from a mode-0
    /// one - it reads the bag row, not the packet; plan step 8 has the reading.
    fn pet_item_refresh(&self, chr: &net::opcode::Character, which: ActivePet, active: bool) -> Reply {
        let ActivePet { slot, item_id, pet_id } = which;
        let vitals = self.pet_vitals(Some(pet_id));
        let name = self.pet_name(Some(pet_id), item_id);
        let blob = net::bag::pet_item_with_state(
            item_id,
            &name,
            Some(net::pet::pet_serial(chr.id, pet_id)),
            u8::from(active),
            &vitals,
        );
        Reply {
            opcode: net::inventory::INVENTORY_OPERATION,
            body: net::inventory::inventory_set_quiet(store::InventoryType::Cash as i8, slot as i16, &blob),
            what: format!(
                "InventoryOperation mode 5 (quiet, no NEW mark): Cash slot {slot} re-sent as pet {item_id} #{pet_id} with active={} and its serial",
                u8::from(active)
            ),
        }
    }
}
