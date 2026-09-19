//! The hair salons' NPCs, on the wire. The rules and the lists are `crate::salon`.
//!
//! The owner (Natalie in Henesys, Don Giovanni in Kerning City) takes the two hair-style
//! coupons; the assistant (Brittany, Andre) the two hair-colour ones. A click counts the
//! desk's coupons in the Cash tab: neither held is a Say linking both and naming the Cash
//! Shop; otherwise a type-6 menu lists the held ones. The Signature line opens the client's
//! "pick a look" box (message type `0x0a`) with the salon's REG styles for the player's
//! gender, or the player's own style in every colour; the Mystery line spends the coupon on
//! a roll at once. Applying a look is the beauty-coupon sequence: write it, spend the
//! coupon, redraw the player with the HAIR bit, tell the field.

use super::{Conversation, Reply, Session};
use crate::salon::{self, Desk, Tier};

impl Session {
    /// The click on one of the four salon NPCs, in its own salon. `None` for any other NPC or
    /// any other map, so the ordinary dialogue path has them.
    pub(super) fn open_salon_for(&mut self, template: u32) -> Option<Vec<Reply>> {
        let chr = self.claimed_character()?;
        let (salon, desk) = salon::desk_for(template, chr.map_id)?;
        let has_signature = self.held_count(chr.id, desk.coupon(Tier::Signature)) > 0;
        let has_mystery = self.held_count(chr.id, desk.coupon(Tier::Mystery)) > 0;
        let Some(menu) = salon::menu_text(desk, has_signature, has_mystery) else {
            crate::server::log(&format!(
                "   salon: character {} clicked {template} ({} {:?}) without coupon {} or {}; pointed at the Cash Shop",
                chr.id,
                salon.name(),
                desk,
                desk.coupon(Tier::Signature),
                desk.coupon(Tier::Mystery)
            ));
            self.conversation = None;
            return Some(vec![Reply {
                opcode: net::script::SCRIPT_MESSAGE,
                body: net::script::npc_say(template, &salon::no_coupon_line(desk), false, false),
                what: format!(
                    "ScriptMessage Say from NPC {template}: neither coupon {} nor {} in the Cash tab - the line links both and names the Cash Shop",
                    desk.coupon(Tier::Signature),
                    desk.coupon(Tier::Mystery)
                ),
            }]);
        };
        self.conversation = Some(Conversation {
            npc_template: template,
            quest_id: None,
            path: salon::MENU_PATH.to_string(),
            sent: 0,
            awaiting_yes_no: false,
            sent_with_next: false,
        });
        Some(vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_menu(template, &menu),
            what: format!(
                "ScriptMessage MENU from NPC {template} ({} {:?}): Signature coupon {}, Mystery coupon {}; #L0 Signature, #L1 Mystery",
                salon.name(),
                desk,
                if has_signature { "listed" } else { "not held" },
                if has_mystery { "listed" } else { "not held" }
            ),
        }])
    }

    /// The coupon menu came back. `None` when no salon menu is parked or the body is not a
    /// menu reply, so the other parsers see it. Signature opens the pick box; Mystery rolls
    /// and applies at once.
    pub(super) fn salon_menu_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        if convo.path != salon::MENU_PATH {
            return None;
        }
        let reply = net::script::parse_menu_reply(body)?;
        self.conversation = None;
        let chr = self.claimed_character()?;
        let template = convo.npc_template;
        let Some((salon, desk)) = salon::desk_for(template, chr.map_id) else { return Some(Vec::new()) };
        let Some(selection) = reply.selection else {
            crate::server::log(&format!("   salon: character {} closed the coupon menu; nothing spent", chr.id));
            return Some(Vec::new());
        };
        // Counted again: the menu was built from a count too, but the bag may have moved.
        let has_signature = self.held_count(chr.id, desk.coupon(Tier::Signature)) > 0;
        let has_mystery = self.held_count(chr.id, desk.coupon(Tier::Mystery)) > 0;
        let Some(tier) = salon::tier_of_selection(selection, has_signature, has_mystery) else {
            crate::server::log(&format!("   salon: character {} answered menu line {selection} it was not offered; refused, nothing spent", chr.id));
            return Some(Vec::new());
        };
        match tier {
            Tier::Signature => {
                let pool: Vec<u32> = match desk {
                    Desk::Styles => salon.styles(Tier::Signature, chr.gender).to_vec(),
                    Desk::Colours => salon::colour_variants(chr.hair, &self.config),
                };
                if pool.is_empty() {
                    crate::server::log(&format!("   salon: character {} hair {} has no colour variants in the hair table; nothing offered", chr.id, chr.hair));
                    return Some(self.notice("I don't have any colours for that style. Nothing was used up.".to_string()));
                }
                self.conversation = Some(Conversation {
                    npc_template: template,
                    quest_id: None,
                    path: salon::CHOICE_PATH.to_string(),
                    sent: 0,
                    awaiting_yes_no: false,
                    sent_with_next: false,
                });
                Some(vec![Reply {
                    opcode: net::script::SCRIPT_MESSAGE,
                    body: net::script::npc_avatar(template, salon::choice_prompt(desk), &pool),
                    what: match desk {
                        Desk::Styles => format!(
                            "ScriptMessage AVATAR (type 0x0a) from NPC {template}: {} {} REG styles for a {} character, drawn on the player; the reply's index picks one",
                            pool.len(),
                            salon.name(),
                            if chr.gender == 0 { "male" } else { "female" }
                        ),
                        Desk::Colours => format!(
                            "ScriptMessage AVATAR (type 0x0a) from NPC {template}: hair {} in its {} colours, drawn on the player; the reply's index picks one",
                            salon::base_of(chr.hair),
                            pool.len()
                        ),
                    },
                }])
            }
            Tier::Mystery => {
                let roll = self.rng.next();
                let hair = match desk {
                    Desk::Styles => {
                        let Some(base) = salon::pick(salon.styles(Tier::Mystery, chr.gender), roll) else { return Some(Vec::new()) };
                        salon::with_current_colour(base, chr.hair, &self.config)
                    }
                    Desk::Colours => salon::base_of(chr.hair) + (roll % salon::COLOURS.len() as u64) as u32,
                };
                Some(self.apply_salon_look(desk, tier, hair))
            }
        }
    }

    /// The pick-a-look box came back. `None` when no salon choice is parked or the body is
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
            crate::server::log(&format!("   salon: character {} closed the pick-a-look box; nothing spent", chr.id));
            return Some(Vec::new());
        };
        // The salon is the map the player is standing on; the box was built from the same.
        let Some((salon, desk)) = salon::desk_for(convo.npc_template, chr.map_id) else { return Some(Vec::new()) };
        let pool: Vec<u32> = match desk {
            Desk::Styles => salon.styles(Tier::Signature, chr.gender).to_vec(),
            Desk::Colours => salon::colour_variants(chr.hair, &self.config),
        };
        let Some(&picked) = pool.get(usize::from(index)) else {
            crate::server::log(&format!("   salon: character {} answered index {index} of {}; refused, nothing spent", chr.id, pool.len()));
            return Some(Vec::new());
        };
        let hair = match desk {
            Desk::Styles => salon::with_current_colour(picked, chr.hair, &self.config),
            Desk::Colours => picked,
        };
        Some(self.apply_salon_look(desk, Tier::Signature, hair))
    }

    /// The look goes on; the desk's coupon for `tier` leaves the Cash tab; the player is
    /// redrawn and the field told. The coupon is checked again here - the box stays open as
    /// long as the player likes, and a coupon traded away meanwhile spends nothing and
    /// changes nothing.
    fn apply_salon_look(&mut self, desk: Desk, tier: Tier, hair: u32) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let coupon = desk.coupon(tier);
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
            "   salon: character {} {:?} {:?}: hair {was} -> {hair}; coupon {coupon} used from Cash slot {slot}",
            chr.id, desk, tier
        ));
        let mut out = vec![super::beautycoupon::look_stat_changed(
            crate::cosmetics::Kind::Hair,
            hair,
            false,
            &format!("the salon: {desk:?} {tier:?}, coupon {coupon}"),
        )];
        out.extend(self.stack_change_replies(store::InventoryType::Cash, slot, held.saturating_sub(1)));
        self.broadcast_look_change(&chr);
        let base = salon::base_of(hair);
        let name = self.config.item_names.get(&hair).or_else(|| self.config.item_names.get(&base)).cloned();
        out.extend(self.notice(match (desk, name) {
            (Desk::Colours, Some(n)) => format!("Your hair is now {} {n}.", salon::colour_name(hair)),
            (Desk::Colours, None) => format!("Your hair is now {}.", salon::colour_name(hair)),
            (Desk::Styles, Some(n)) => format!("Your hair is now {n}."),
            (Desk::Styles, None) => "Your hair has changed.".to_string(),
        }));
        out
    }
}
