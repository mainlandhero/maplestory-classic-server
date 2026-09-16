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
//! # Not built, and said so
//!
//! Feeding, naming, dyeing, closeness and levelling, and the Cash Shop pet-skill items that
//! would set the item body's `petSkill` mask (`net::bag::PET_SKILLS_LEARNED_AT_START`). A pet
//! is level 1 for as long as closeness does not exist, and that is what picks the command band.

use super::*;

/// The level every pet is at, because this server keeps no closeness yet. It selects which
/// band of an `interact` command answers. See `Session::pet_command_replies`.
const PET_LEVEL: u32 = 1;

/// The pet this session has out, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ActivePet {
    /// The Cash-tab slot it lives in, 1-based.
    pub slot: u16,
    pub item_id: u32,
}

impl Session {
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
        // Owner-local by default: broadcasting a pet MOVE to the map is what crashed a second
        // client on 2026-09-15 - the remote pet has no visual to apply the move to.
        // `Config::broadcast_pets`.
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
        let Some(r) = self.config.pet_commands.respond(active.item_id, PET_LEVEL, text, roll)
        else {
            return Vec::new();
        };
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
        // The owner always gets its own pet's answer (the return below); other players get it
        // only when pets are broadcast, which is off by default - a pet action drives the same
        // remote-pet object the move does, so it rides the same crash. `Config::broadcast_pets`.
        if self.config.broadcast_pets {
            self.bus().publish(self.subscriber, chr.map_id, reply.clone(), None);
        }
        vec![reply]
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
            out.push(gone);
            out.push(self.pet_item_refresh(&chr, active.slot, active.item_id, false));
            if active.slot == req.slot {
                out.push(unlock);
                return out;
            }
        }

        let pet = self.field_pet(&chr, item.item_id);
        let up = Reply {
            opcode: net::pet::PET_ACTIVATED,
            body: net::pet::pet_activated(chr.id, &pet),
            what: format!(
                "PetActivated: {} ({}) summoned beside {} at ({}, {}) fh {} - Cash slot {}",
                pet.name, pet.item_id, chr.name, pet.x, pet.y, pet.foothold, req.slot
            ),
        };
        // Owner-local by default. When pets are broadcast, the summon goes to the map and the
        // pet travels with the owner to whoever arrives after; when they are not - the default
        // since the 2026-09-15 crash - neither happens, so no other client is ever handed a
        // remote pet that would crash on its first move. `Config::broadcast_pets`.
        if self.config.broadcast_pets {
            self.bus().publish(self.subscriber, chr.map_id, up.clone(), None);
            self.bus().set_companions(self.subscriber, vec![up.clone()]);
        }
        out.push(up);
        self.active_pet = Some(ActivePet { slot: req.slot, item_id: item.item_id });
        out.push(self.pet_item_refresh(&chr, req.slot, item.item_id, true));
        out.push(unlock);
        out
    }

    /// **The packets that travel with this player** - `crate::broadcast::Presence::companions`.
    /// The summoned pet's `0x0277`, so a player arriving on the map after the summon gets
    /// the pet right behind the owner's spawn; nothing when no pet is out.
    pub(super) fn pet_companions(&self, chr: &net::opcode::Character) -> Vec<Reply> {
        // Off by default: an arriving player must not be handed a pet either, since it would
        // crash on the owner's first step. `Config::broadcast_pets`, and the 2026-09-15 crash.
        if !self.config.broadcast_pets {
            return Vec::new();
        }
        let Some(active) = self.active_pet else { return Vec::new() };
        let pet = self.field_pet(chr, active.item_id);
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
    pub(super) fn pet_entry_replies(&self, chr: &net::opcode::Character) -> Vec<Reply> {
        let Some(active) = self.active_pet else { return Vec::new() };
        let pet = self.field_pet(chr, active.item_id);
        vec![Reply {
            opcode: net::pet::PET_ACTIVATED,
            body: net::pet::pet_activated(chr.id, &pet),
            what: format!(
                "PetActivated: {} ({}) back beside {} after the field entry, at ({}, {})",
                pet.name, pet.item_id, chr.name, pet.x, pet.y
            ),
        }]
    }

    /// Whether `item_id` is the pet this session has out - the `active` byte of its body.
    pub(super) fn pet_is_active(&self, item_id: u32) -> bool {
        self.active_pet.is_some_and(|p| p.item_id == item_id)
    }

    /// The pet as `0x0277` describes it: the item's name, the pairing serial, and the spot
    /// under the character. With no position reported yet it stands at the origin, which the
    /// client's own physics then drops onto whatever is below.
    fn field_pet(&self, chr: &net::opcode::Character, item_id: u32) -> net::pet::FieldPet {
        let (x, y) = self.last_position.unwrap_or((0, 0));
        let landing = self.config.footholds.landing(chr.map_id, x, y);
        net::pet::FieldPet {
            item_id,
            name: self.config.item_names.get(&item_id).cloned().unwrap_or_default(),
            serial: net::pet::pet_serial(chr.id, item_id).get(),
            x,
            y: landing.as_ref().map_or(y, |l| l.y),
            move_action: self.config.pet_move_action.unwrap_or(0),
            foothold: landing.map_or(0, |l| u16::try_from(l.foothold).unwrap_or(0)),
        }
    }

    /// The Cash-tab item re-sent at its slot with its `active` byte as `active` says and the
    /// pairing serial set. An Add at an occupied slot replaces what the client holds there.
    fn pet_item_refresh(&self, chr: &net::opcode::Character, slot: u16, item_id: u32, active: bool) -> Reply {
        let name = self.config.item_names.get(&item_id).cloned().unwrap_or_default();
        let blob = net::bag::pet_item_with_state(
            item_id,
            &name,
            Some(net::pet::pet_serial(chr.id, item_id)),
            u8::from(active),
        );
        Reply {
            opcode: net::inventory::INVENTORY_OPERATION,
            body: net::inventory::inventory_added(store::InventoryType::Cash as i8, slot as i16, &blob),
            what: format!(
                "InventoryOperation: Cash slot {slot} re-sent as pet {item_id} with active={} and its serial",
                u8::from(active)
            ),
        }
    }
}
