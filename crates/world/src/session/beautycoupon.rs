//! `0x0165` - the Beauty Coupon dialog's Confirm. Apply the cosmetic, spend the coupon, redraw.
//!
//! The owner, 2026-09-12: *"I tried using the Ubel Hair Coupon but my hair did not change."* The
//! press arrived twice as `0x0165` and nothing answered it - `net::beautycoupon` has the
//! capture. The dialog and its preview are the client's; the decision is the server's.
//!
//! **The redraw is a field re-entry**, the same way `!hair` and `!face` do it: `0x0138`
//! applies nothing in this client (`net::opcode::USER_AVATAR_MODIFIED`), so the look is
//! rebuilt by a `SetField` of the map the player is standing on. That re-entry also re-sends
//! the bag, which is how the spent coupon leaves the screen.

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
            "   beauty coupon: character {} used {} - {what} is now {cosmetic}; re-entering map {} to redraw",
            chr.id, req.item_id, chr.map_id
        ));
        let map = chr.map_id;
        let mut out = self.stack_change_replies(store::InventoryType::Use, req.slot, 0);
        out.extend(self.go_to_map(
            &mut chr,
            map,
            0,
            format!("beauty coupon {}: {what} -> {cosmetic}, re-entry so the look is rebuilt", req.item_id),
        ));
        out
    }
}
