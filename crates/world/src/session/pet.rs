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
//! # Not built, and said so
//!
//! Pet movement, feeding, naming and the rest of the `0x0278..0x027E` family. The client
//! moves the pet itself; whatever it sends about that is logged and not yet answered.

use super::*;

/// The pet this session has out, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ActivePet {
    /// The Cash-tab slot it lives in, 1-based.
    pub slot: u16,
    pub item_id: u32,
}

impl Session {
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
            self.bus().publish(self.subscriber, chr.map_id, gone.clone(), None);
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
        self.bus().publish(self.subscriber, chr.map_id, up.clone(), None);
        out.push(up);
        self.active_pet = Some(ActivePet { slot: req.slot, item_id: item.item_id });
        out.push(self.pet_item_refresh(&chr, req.slot, item.item_id, true));
        out.push(unlock);
        out
    }

    /// The pet again, after a `SetField` rebuilt the client's pools. Nothing when none is out.
    pub(super) fn pet_entry_replies(&self, chr: &net::opcode::Character) -> Vec<Reply> {
        let Some(active) = self.active_pet else { return Vec::new() };
        let pet = self.field_pet(chr, active.item_id);
        let up = Reply {
            opcode: net::pet::PET_ACTIVATED,
            body: net::pet::pet_activated(chr.id, &pet),
            what: format!(
                "PetActivated: {} ({}) back beside {} after the field entry, at ({}, {})",
                pet.name, pet.item_id, chr.name, pet.x, pet.y
            ),
        };
        self.bus().publish(self.subscriber, chr.map_id, up.clone(), None);
        vec![up]
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
            move_action: 0,
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
