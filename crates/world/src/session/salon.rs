//! The hair salon's two NPCs, on the wire. The rules and the lists are `crate::salon`.
//!
//! Denma the Owner (Signature Hair Coupon): the client's "pick a look" box, message type
//! `0x0a`, drawn with the player's own pool; the answer's index picks the style. Dr. Feeble
//! (Mystery Hair Coupon): a yes/no, then a random style from the VIP pool. A click without
//! the coupon gets a Say that links the coupon and names the Cash Shop. Applying a style is
//! the beauty-coupon sequence: write the look, spend the coupon, redraw the player with the
//! HAIR bit, tell the field.

use super::{Conversation, Reply, Session};
use crate::salon::{self, Service};

impl Session {
    /// The click on Denma or Dr. Feeble in Henesys Plastic Surgery. `None` for any other NPC
    /// or any other map, so the ordinary dialogue path has them.
    pub(super) fn open_salon_for(&mut self, template: u32) -> Option<Vec<Reply>> {
        let chr = self.claimed_character()?;
        let service = salon::service_for(template, chr.map_id)?;
        if self.held_count(chr.id, service.coupon()) == 0 {
            crate::server::log(&format!(
                "   salon: character {} clicked {template} without coupon {}; pointed at the Cash Shop",
                chr.id,
                service.coupon()
            ));
            self.conversation = None;
            return Some(vec![Reply {
                opcode: net::script::SCRIPT_MESSAGE,
                body: net::script::npc_say(template, &salon::no_coupon_line(service), false, false),
                what: format!("ScriptMessage Say from NPC {template}: no coupon {} in the Cash tab - the line links it and names the Cash Shop", service.coupon()),
            }]);
        }
        let pool = service.styles(chr.gender);
        Some(match service {
            Service::Signature => {
                self.conversation = Some(Conversation {
                    npc_template: template,
                    quest_id: None,
                    path: salon::CHOICE_PATH.to_string(),
                    sent: 0,
                    awaiting_yes_no: false,
                    sent_with_next: false,
                });
                vec![Reply {
                    opcode: net::script::SCRIPT_MESSAGE,
                    body: net::script::npc_avatar(template, salon::choice_prompt(), pool),
                    what: format!(
                        "ScriptMessage AVATAR (type 0x0a) from Denma: {} Henesys REG styles for a {} character, drawn on the player; the reply's index picks one",
                        pool.len(),
                        if chr.gender == 0 { "male" } else { "female" }
                    ),
                }]
            }
            Service::Mystery => {
                self.conversation = Some(Conversation {
                    npc_template: template,
                    quest_id: None,
                    path: salon::MYSTERY_PATH.to_string(),
                    sent: 0,
                    awaiting_yes_no: true,
                    sent_with_next: false,
                });
                vec![Reply {
                    opcode: net::script::SCRIPT_MESSAGE,
                    body: net::script::npc_ask(template, salon::mystery_prompt(), false),
                    what: "ScriptMessage YES/NO from Dr. Feeble: spend the Mystery Hair Coupon on a random Henesys VIP style?".to_string(),
                }]
            }
        })
    }

    /// Denma's avatar box came back. `None` when no salon choice is parked or the body is
    /// not a type-0x0a reply, so the other parsers see it.
    pub(super) fn salon_choice_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        if convo.path != salon::CHOICE_PATH {
            return None;
        }
        let reply = net::script::parse_avatar_reply(body)?;
        self.conversation = None;
        let chr = self.claimed_character()?;
        let Some(index) = reply.selection else {
            crate::server::log(&format!("   salon: character {} closed Denma's box; nothing spent", chr.id));
            return Some(Vec::new());
        };
        let pool = Service::Signature.styles(chr.gender);
        let Some(&base) = pool.get(usize::from(index)) else {
            crate::server::log(&format!("   salon: character {} answered index {index} of {}; refused, nothing spent", chr.id, pool.len()));
            return Some(Vec::new());
        };
        Some(self.apply_salon_style(Service::Signature, base))
    }

    /// Dr. Feeble's yes/no came back (`on_script_reply` routes by path). Yes spends the
    /// coupon on a random VIP style; anything else spends nothing.
    pub(super) fn salon_mystery_answer(&mut self, action: i8) -> Vec<Reply> {
        self.conversation = None;
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        if action != 1 {
            crate::server::log(&format!("   salon: character {} declined Dr. Feeble; nothing spent", chr.id));
            return Vec::new();
        }
        let pool = Service::Mystery.styles(chr.gender);
        let roll = self.rng.next();
        let Some(base) = salon::pick(pool, roll) else { return Vec::new() };
        self.apply_salon_style(Service::Mystery, base)
    }

    /// The style goes on, in the player's current colour; the coupon leaves the Cash tab;
    /// the player is redrawn and the field told. The coupon is checked again here - the box
    /// stays open as long as the player likes, and a coupon traded away meanwhile spends
    /// nothing and changes nothing.
    fn apply_salon_style(&mut self, service: Service, base: u32) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let coupon = service.coupon();
        let slot = self
            .store
            .bag_items(chr.id, store::InventoryType::Cash)
            .ok()
            .into_iter()
            .flatten()
            .find(|r| r.item.item_id == coupon)
            .map(|r| (r.slot, r.item.kind.quantity()));
        let Some((slot, held)) = slot else {
            crate::server::log(&format!("   salon: character {} has no coupon {coupon} any more; nothing changed", chr.id));
            return self.notice("The coupon is no longer in your Cash tab. Nothing was changed.".to_string());
        };
        let hair = salon::with_current_colour(base, chr.hair, &self.config);
        match self.store.set_character_look(chr.id, Some(hair), None) {
            Ok(true) => {}
            Ok(false) | Err(_) => {
                crate::server::log(&format!("   salon: could not write character {}'s hair; nothing spent", chr.id));
                return self.notice("That did not take. Nothing was used up.".to_string());
            }
        }
        // The effect first, then the cost - the Heena rule.
        let _ = self.store.remove_item(chr.id, store::InventoryType::Cash, slot, Some(1));
        let was = chr.hair;
        chr.hair = hair;
        crate::server::log(&format!(
            "   salon: character {} {} -> hair {hair} (base {base}, colour kept from {was}); coupon {coupon} used from Cash slot {slot}",
            chr.id,
            match service {
                Service::Signature => "chose at Denma",
                Service::Mystery => "rolled at Dr. Feeble",
            }
        ));
        let mut out = vec![super::beautycoupon::look_stat_changed(
            crate::cosmetics::Kind::Hair,
            hair,
            false,
            &format!("the salon: base {base} in the player's colour, coupon {coupon}"),
        )];
        out.extend(self.stack_change_replies(store::InventoryType::Cash, slot, held.saturating_sub(1)));
        self.broadcast_look_change(&chr);
        let name = self.config.item_names.get(&hair).or_else(|| self.config.item_names.get(&base)).cloned();
        out.extend(self.notice(match name {
            Some(n) => format!("Your hair is now {n}."),
            None => "Your hair has changed.".to_string(),
        }));
        out
    }
}
