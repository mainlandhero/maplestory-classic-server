//! `0x0165` - the Beauty Coupon dialog's Confirm. Apply the cosmetic, spend the coupon, redraw.
//!
//! The owner, 2026-09-12: *"I tried using the Ubel Hair Coupon but my hair did not change."* The
//! press arrived twice as `0x0165` and nothing answered it - `net::beautycoupon` has the
//! capture. The dialog and its preview are the client's; the decision is the server's.
//!
//! **No field re-entry** (the owner, 2026-09-12: *"switch immediately... without a reload"*). The
//! client's own dialog previews the look and commits it on Confirm, so the player's own
//! screen is already right; the server persists it, spends the coupon, and tells the OTHER
//! clients through `Session::broadcast_look_change`. `0x0138` applies nothing in this client
//! (`research/naked-character.md`), which is why the remote update rides the user-pool packet.

use super::{Reply, Session};

impl Session {
    pub(super) fn on_beauty_coupon_confirm(&mut self, body: &[u8]) -> Vec<Reply> {
        let opcode = net::beautycoupon::CLIENT_BEAUTY_COUPON_CONFIRM;
        let unlock = || crate::mesodrop::unlock_unhandled_latching_request(opcode);
        let Some(req) = net::beautycoupon::parse_beauty_coupon_confirm(body) else {
            crate::server::log(&format!(
                "   beauty coupon: a {} byte 0x0165 body that does not parse; unlock only",
                body.len()
            ));
            return unlock();
        };
        let Some(mut chr) = self.claimed_character() else { return unlock() };

        // The slot must hold the coupon the packet names - the same rule every item use here
        // follows, because nothing on this socket is authenticated.
        let holding = self
            .store
            .bag_items(chr.id, store::InventoryType::Use)
            .ok()
            .into_iter()
            .flatten()
            .find(|r| r.slot == req.slot)
            .map(|r| r.item.item_id);
        if holding != Some(req.item_id) {
            crate::server::log(&format!(
                "   beauty coupon: character {} confirmed {} from Use slot {}, which holds {:?}. \
                 Refused; nothing changed.",
                chr.id, req.item_id, req.slot, holding
            ));
            return unlock();
        }
        let Some((kind, cosmetic)) = crate::cosmetics::for_coupon(req.item_id) else {
            crate::server::log(&format!(
                "   beauty coupon: {} is not a coupon this server knows (world::cosmetics); \
                 unlock only, and the item is KEPT",
                req.item_id
            ));
            return unlock();
        };
        let (hair, face) = match kind {
            crate::cosmetics::Kind::Hair => (Some(cosmetic), None),
            crate::cosmetics::Kind::Face => (None, Some(cosmetic)),
        };
        match self.store.set_character_look(chr.id, hair, face) {
            Ok(true) => {}
            Ok(false) | Err(_) => {
                crate::server::log(&format!(
                    "   beauty coupon: could not write character {}'s look; nothing consumed",
                    chr.id
                ));
                return unlock();
            }
        }
        // Only now does the coupon leave the bag - the Heena rule: the effect first, then
        // the cost, and never the cost alone.
        let _ = self.store.remove_item(chr.id, store::InventoryType::Use, req.slot, Some(1));
        match kind {
            crate::cosmetics::Kind::Hair => chr.hair = cosmetic,
            crate::cosmetics::Kind::Face => chr.face = cosmetic,
        }
        let what = match kind {
            crate::cosmetics::Kind::Hair => "hairstyle",
            crate::cosmetics::Kind::Face => "face",
        };
        crate::server::log(&format!(
            "   beauty coupon: character {} used {} - {what} is now {cosmetic}; broadcasting to the field, no reload",
            chr.id, req.item_id
        ));
        // **No field re-entry.** The owner, 2026-09-12: *"The hair should just switch immediately
        // on screen without a reload."* The client's own Beauty dialog previewed the look and
        // committed it on Confirm, so the player's screen is already right; a `SetField` here
        // was a visible reload for nothing. The OTHER clients are told through
        // `broadcast_look_change`, and the coupon is answered with its inventory op plus the
        // exclusive-request unlock, so nothing is left latched.
        let mut out = crate::mesodrop::unlock_unhandled_latching_request(opcode);
        out.extend(self.stack_change_replies(store::InventoryType::Use, req.slot, 0));
        self.broadcast_look_change(&chr);
        out
    }
}
