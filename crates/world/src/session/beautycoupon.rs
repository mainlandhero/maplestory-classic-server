//! `0x0165` - the Beauty Coupon dialog's Confirm. Apply the cosmetic, spend the coupon, redraw.
//!
//! The owner, 2026-09-12: *"I tried using the Ubel Hair Coupon but my hair did not change."* The
//! press arrived twice as `0x0165` and nothing answered it - `net::beautycoupon` has the
//! capture. The dialog and its preview are the client's; the decision is the server's.
//!
//! **No field re-entry** (the owner, 2026-09-12: *"switch immediately... without a reload"*), and
//! **no assumption that the client applies it either** (the owner, 2026-09-18: *"the player needs
//! to enter a different map to see the hair or face updated"* - the dialog previews, it does
//! not commit). The player's own screen is redrawn by a `0x007C StatChanged` carrying the
//! FACE or HAIR bit - [`look_stat_changed`] - and the OTHER clients are told through
//! `Session::broadcast_look_change`. `0x0138` applies nothing in this client
//! (`research/naked-character.md`), which is why neither half rides it.

use super::{Reply, Session};
use crate::cosmetics::Kind;

/// The `0x007C` that redraws the player's own avatar with a new hair or face.
///
/// The owner, 2026-09-18: *"the player needs to enter a different map to see the hair or face
/// updated on their character."* The earlier reading - that the Beauty dialog commits the
/// look on Confirm - was an assumption, and that report is its measurement. `0x0138` applies
/// nothing in this client, but the `StatChanged` handler `FUN_142d54780` does: at
/// `142d560d5` it tests the mask's FACE bit and at `142d56122` the HAIR bit, and for each it
/// calls `FUN_142ce51b0(user, id, 1)` for the new id and the old, then `FUN_142ce5e60(user)`,
/// then `FUN_141e755d0` once for any of the three look bits - the same pair of calls the
/// `0x0070` equip handler makes after a worn-slot change, which is the redraw every equip
/// on screen goes through (`research/equip-crash.md`, `research/beauty-2026-09-09.md` §8).
/// **[L]** for the chain; that it draws is plan step TO(c)'s reading.
///
/// **One packet.** The exclusive-request byte clears the `0x0165` latch and the mask carries
/// the change, so nothing is left latched and the client is not answered twice.
pub(super) fn look_stat_changed(kind: Kind, id: u32, excl: bool, why: &str) -> Reply {
    let change = net::stats::StatChange {
        excl_request_sent: excl,
        hair: (kind == Kind::Hair).then_some(id),
        face: (kind == Kind::Face).then_some(id),
        ..Default::default()
    };
    let what = match kind {
        Kind::Hair => "HAIR",
        Kind::Face => "FACE",
    };
    Reply {
        opcode: net::stats::STAT_CHANGED,
        body: change.build(),
        what: format!(
            "StatChanged: {what} bit -> {id}{}. {why}. The client's handler runs the equip \
             redraw pair (FUN_142ce51b0 x2, FUN_142ce5e60) for this bit, so the player's own \
             avatar is rebuilt in place - no SetField, no re-entry.",
            if excl { ", exclusive-request latch cleared" } else { "" }
        ),
    }
}

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
            "   beauty coupon: character {} used {} - {what} is now {cosmetic}; redrawn by 0x007C, broadcast to the field, no reload",
            chr.id, req.item_id
        ));
        // **No field re-entry** (the owner, 2026-09-12: *"switch immediately on screen without a
        // reload"*) **and no reliance on the dialog** (the owner, 2026-09-18: it previews, it does
        // not commit - the look only showed after a map change). The `0x007C` with the look
        // bit redraws the player and clears the `0x0165` latch in one packet; the OTHER
        // clients are told through `broadcast_look_change`; the coupon's inventory op follows.
        let mut out = vec![look_stat_changed(
            kind,
            cosmetic,
            true,
            &format!("Beauty coupon {} confirmed from Use slot {}", req.item_id, req.slot),
        )];
        out.extend(self.stack_change_replies(store::InventoryType::Use, req.slot, 0));
        self.broadcast_look_change(&chr);
        out
    }
}
