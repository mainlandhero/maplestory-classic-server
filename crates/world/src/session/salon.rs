//! The beauty shops' NPCs, on the wire. The rules and the lists are `crate::salon`.
//!
//! A salon's owner (Natalie, Don Giovanni) takes the two hair-style coupons and its
//! assistant (Brittany, Andre) the two hair-colour ones; a surgery's owner (Denma, Franz)
//! the two face coupons and its assistant (Dr. Feeble, Riza) the skin coupon. A click counts
//! the desk's coupons in the Cash tab: none held is a Say linking them and naming the Cash
//! Shop; otherwise a type-6 menu lists the held ones. The Signature line opens the client's
//! "pick a look" box (message type `0x0a`) with the desk's candidates; the Mystery line
//! spends the coupon on a roll at once. Applying a look is the beauty-coupon sequence: write
//! it, spend the coupon, redraw the player with the HAIR / FACE / SKIN bit, tell the field.

use super::{Conversation, Reply, Session};
use crate::salon::{self, Desk, Shop, Tier};

/// What a pick or a roll settled on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Look {
    Hair(u32),
    Face(u32),
    Skin(u8),
}

impl Session {
    /// The click on one of the eight beauty NPCs, in its own shop. `None` for any other NPC
    /// or any other map, so the ordinary dialogue path has them.
    pub(super) fn open_salon_for(&mut self, template: u32) -> Option<Vec<Reply>> {
        let chr = self.claimed_character()?;
        let (shop, desk) = salon::desk_for(template, chr.map_id)?;
        let (has_signature, has_mystery) = self.salon_coupons_held(chr.id, desk);
        let Some(menu) = salon::menu_text(desk, has_signature, has_mystery) else {
            crate::server::log(&format!(
                "   salon: character {} clicked {template} ({} {:?}) without coupon {:?} or {:?}; pointed at the Cash Shop",
                chr.id,
                shop.name(),
                desk,
                desk.coupon(Tier::Signature),
                desk.coupon(Tier::Mystery)
            ));
            self.conversation = None;
            return Some(vec![Reply {
                opcode: net::script::SCRIPT_MESSAGE,
                body: net::script::npc_say(template, &salon::no_coupon_line(desk), false, false),
                what: format!(
                    "ScriptMessage Say from NPC {template}: no coupon {:?} / {:?} in the Cash tab - the line links them and names the Cash Shop",
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
                shop.name(),
                desk,
                if has_signature { "listed" } else { "not held" },
                if has_mystery { "listed" } else { "not held" }
            ),
        }])
    }

    /// Whether the character holds the desk's Signature and Mystery coupons. A desk with no
    /// Mystery item (skins) never holds one.
    fn salon_coupons_held(&self, chr_id: u32, desk: Desk) -> (bool, bool) {
        let held = |tier| desk.coupon(tier).is_some_and(|c| self.held_count(chr_id, c) > 0);
        (held(Tier::Signature), held(Tier::Mystery))
    }

    /// The desk's Signature candidates for this character: the pick box's contents.
    fn salon_candidates(&self, shop: Shop, desk: Desk, chr: &net::opcode::Character) -> Vec<u32> {
        match desk {
            Desk::Styles => shop.styles(Tier::Signature, chr.gender).to_vec(),
            Desk::Colours => salon::colour_variants(chr.hair, &self.config),
            Desk::Faces => salon::faces(Tier::Signature, chr.gender).to_vec(),
            Desk::Skins => salon::skin_candidates(),
        }
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
        let Some((shop, desk)) = salon::desk_for(template, chr.map_id) else { return Some(Vec::new()) };
        let Some(selection) = reply.selection else {
            crate::server::log(&format!("   salon: character {} closed the coupon menu; nothing spent", chr.id));
            return Some(Vec::new());
        };
        // Counted again: the menu was built from a count too, but the bag may have moved.
        let (has_signature, has_mystery) = self.salon_coupons_held(chr.id, desk);
        let Some(tier) = salon::tier_of_selection(selection, has_signature, has_mystery) else {
            crate::server::log(&format!("   salon: character {} answered menu line {selection} it was not offered; refused, nothing spent", chr.id));
            return Some(Vec::new());
        };
        match tier {
            Tier::Signature => {
                let pool = self.salon_candidates(shop, desk, &chr);
                if pool.is_empty() {
                    crate::server::log(&format!("   salon: character {} has nothing to pick from at {desk:?} (hair {}, face {}); nothing offered", chr.id, chr.hair, chr.face));
                    return Some(self.notice("I have nothing to offer for that. Nothing was used up.".to_string()));
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
                    what: format!(
                        "ScriptMessage AVATAR (type 0x0a) from NPC {template}: {} {desk:?} candidates for a {} character ({}), drawn on the player; the reply's index picks one",
                        pool.len(),
                        if chr.gender == 0 { "male" } else { "female" },
                        match desk {
                            Desk::Styles => format!("{} REG styles", shop.name()),
                            Desk::Colours => format!("hair {} in its colours", salon::base_of(chr.hair)),
                            Desk::Faces => "the REG faces".to_string(),
                            Desk::Skins => "the skins this client draws; the client reads ids under 24000 as skins".to_string(),
                        }
                    ),
                }])
            }
            Tier::Mystery => {
                let roll = self.rng.next();
                let look = match desk {
                    Desk::Styles => {
                        let Some(base) = salon::pick(shop.styles(Tier::Mystery, chr.gender), roll) else { return Some(Vec::new()) };
                        Look::Hair(salon::with_current_colour(base, chr.hair, &self.config))
                    }
                    Desk::Colours => Look::Hair(salon::base_of(chr.hair) + (roll % salon::COLOURS.len() as u64) as u32),
                    Desk::Faces => {
                        let Some(base) = salon::pick(salon::faces(Tier::Mystery, chr.gender), roll) else { return Some(Vec::new()) };
                        Look::Face(salon::face_with_current_eye_colour(base, chr.face, &self.config))
                    }
                    Desk::Skins => {
                        let Some(s) = salon::pick(&salon::skin_candidates(), roll) else { return Some(Vec::new()) };
                        Look::Skin(s as u8)
                    }
                };
                Some(self.apply_salon_look(desk, tier, look))
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
        // The shop is the map the player is standing on; the box was built from the same.
        let Some((shop, desk)) = salon::desk_for(convo.npc_template, chr.map_id) else { return Some(Vec::new()) };
        let pool = self.salon_candidates(shop, desk, &chr);
        let Some(&picked) = pool.get(usize::from(index)) else {
            crate::server::log(&format!("   salon: character {} answered index {index} of {}; refused, nothing spent", chr.id, pool.len()));
            return Some(Vec::new());
        };
        let look = match desk {
            Desk::Styles => Look::Hair(salon::with_current_colour(picked, chr.hair, &self.config)),
            Desk::Colours => Look::Hair(picked),
            Desk::Faces => Look::Face(salon::face_with_current_eye_colour(picked, chr.face, &self.config)),
            Desk::Skins => Look::Skin(picked as u8),
        };
        Some(self.apply_salon_look(desk, Tier::Signature, look))
    }

    /// The look goes on; the desk's coupon for `tier` leaves the Cash tab; the player is
    /// redrawn and the field told. The coupon is checked again here - the box stays open as
    /// long as the player likes, and a coupon traded away meanwhile spends nothing and
    /// changes nothing.
    fn apply_salon_look(&mut self, desk: Desk, tier: Tier, look: Look) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let Some(coupon) = desk.coupon(tier) else { return Vec::new() };
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
        let written = match look {
            Look::Hair(h) => self.store.set_character_look(chr.id, Some(h), None),
            Look::Face(f) => self.store.set_character_look(chr.id, None, Some(f)),
            Look::Skin(s) => self.store.set_character_skin(chr.id, s),
        };
        match written {
            Ok(true) => {}
            Ok(false) | Err(_) => {
                crate::server::log(&format!("   salon: could not write character {}'s {look:?}; nothing spent", chr.id));
                return self.notice("That did not take. Nothing was used up.".to_string());
            }
        }
        // The effect first, then the cost - the Heena rule.
        let _ = self.store.remove_item(chr.id, store::InventoryType::Cash, slot, Some(1));
        let was = (chr.hair, chr.face, chr.skin);
        let stat = match look {
            Look::Hair(h) => {
                chr.hair = h;
                super::beautycoupon::look_stat_changed(crate::cosmetics::Kind::Hair, h, false, &format!("the salon: {desk:?} {tier:?}, coupon {coupon}"))
            }
            Look::Face(f) => {
                chr.face = f;
                super::beautycoupon::look_stat_changed(crate::cosmetics::Kind::Face, f, false, &format!("the surgery: {desk:?} {tier:?}, coupon {coupon}"))
            }
            Look::Skin(s) => {
                chr.skin = s;
                Reply {
                    opcode: net::stats::STAT_CHANGED,
                    body: net::stats::StatChange { skin: Some((s, 0)), ..Default::default() }.build(),
                    what: format!(
                        "StatChanged: SKIN bit -> {s} (u8 skin, u32 0). The surgery: {desk:?} {tier:?}, coupon {coupon}. \
                         The client's handler redraws the body for this bit as it does for HAIR and FACE."
                    ),
                }
            }
        };
        crate::server::log(&format!(
            "   salon: character {} {desk:?} {tier:?}: (hair, face, skin) {was:?} -> {look:?}; coupon {coupon} used from Cash slot {slot}",
            chr.id
        ));
        let mut out = vec![stat];
        out.extend(self.stack_change_replies(store::InventoryType::Cash, slot, held.saturating_sub(1)));
        self.broadcast_look_change(&chr);
        let notice = match look {
            Look::Hair(h) => {
                let name = self.config.item_names.get(&h).or_else(|| self.config.item_names.get(&salon::base_of(h))).cloned();
                match (desk, name) {
                    (Desk::Colours, Some(n)) => format!("Your hair is now {} {n}.", salon::colour_name(h)),
                    (Desk::Colours, None) => format!("Your hair is now {}.", salon::colour_name(h)),
                    (_, Some(n)) => format!("Your hair is now {n}."),
                    (_, None) => "Your hair has changed.".to_string(),
                }
            }
            Look::Face(f) => match self.config.item_names.get(&f).or_else(|| self.config.item_names.get(&(f - (f / 100 % 10) * 100))) {
                Some(n) => format!("Your face is now {n}."),
                None => "Your face has changed.".to_string(),
            },
            Look::Skin(s) => format!("Your skin is now {}.", salon::skin_name(s)),
        };
        out.extend(self.notice(notice));
        out
    }
}
