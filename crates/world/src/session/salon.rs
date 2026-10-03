//! The beauty shops' NPCs, on the wire. The rules and the lists are `crate::salon`.
//!
//! A salon's owner (Natalie, Don Giovanni) takes the two hair-style coupons and its
//! assistant (Brittany, Andre) the two hair-colour ones; a surgery's owner (Denma, Franz)
//! the two face coupons and its assistant (Dr. Feeble, Riza) the skin coupon and the
//! Signature Eye Color Coupon (2026-10-02 - eye colour had no desk at all). A click counts
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
        let has_second = self.salon_second_held(chr.id, desk);
        let Some(menu) = salon::menu_text(desk, has_signature, has_mystery, has_second) else {
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

    /// Whether the character holds the Signature coupon of the desk's [`Desk::second`] line.
    fn salon_second_held(&self, chr_id: u32, desk: Desk) -> bool {
        desk.second()
            .and_then(|d| d.coupon(Tier::Signature))
            .is_some_and(|c| self.held_count(chr_id, c) > 0)
    }

    /// The desk's Signature candidates for this character: the pick box's contents.
    fn salon_candidates(&self, shop: Shop, desk: Desk, chr: &net::opcode::Character) -> Vec<u32> {
        match desk {
            Desk::Styles => shop.styles(Tier::Signature, chr.gender).to_vec(),
            Desk::Colours => salon::colour_variants(chr.hair, &self.config),
            Desk::Faces => salon::faces(Tier::Signature, chr.gender).to_vec(),
            Desk::Skins => salon::skin_candidates(),
            Desk::EyeColours => salon::eye_colour_variants(chr.face, &self.config),
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
        // The third line: the second desk's Signature pick (the surgery assistant's eye colours).
        if selection == salon::MENU_SECOND {
            let Some(second) = desk.second().filter(|_| self.salon_second_held(chr.id, desk)) else {
                crate::server::log(&format!("   salon: character {} answered menu line {selection} it was not offered; refused, nothing spent", chr.id));
                return Some(Vec::new());
            };
            let pool = self.salon_candidates(shop, second, &chr);
            if pool.is_empty() {
                crate::server::log(&format!("   salon: character {} has nothing to pick from at {second:?} (face {}); nothing offered", chr.id, chr.face));
                return Some(self.notice("I have nothing to offer for that. Nothing was used up.".to_string()));
            }
            self.conversation = Some(Conversation {
                npc_template: template,
                quest_id: None,
                path: salon::EYE_CHOICE_PATH.to_string(),
                sent: 0,
                awaiting_yes_no: false,
                sent_with_next: false,
            });
            return Some(vec![Reply {
                opcode: net::script::SCRIPT_MESSAGE,
                body: net::script::npc_avatar(
                    template,
                    second.coupon(Tier::Signature).unwrap_or(0),
                    salon::choice_prompt(second),
                    &pool,
                ),
                what: format!(
                    "ScriptMessage AVATAR (type 0x0a) from NPC {template}: face {} in {} eye colour(s) this client draws; the reply's index picks one",
                    chr.face,
                    pool.len()
                ),
            }]);
        }
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
                    // The coupon types the box: without it a colour box rewrites every
                    // candidate to the current colour (net::script::SCRIPT_TYPE_AVATAR).
                    body: net::script::npc_avatar(
                        template,
                        desk.coupon(Tier::Signature).unwrap_or(0),
                        salon::choice_prompt(desk),
                        &pool,
                    ),
                    what: format!(
                        "ScriptMessage AVATAR (type 0x0a) from NPC {template}: {} {desk:?} candidates for a {} character ({}), drawn on the player; the reply's index picks one",
                        pool.len(),
                        if chr.gender == 0 { "male" } else { "female" },
                        match desk {
                            Desk::Styles => format!("{} REG styles", shop.name()),
                            Desk::Colours => format!("hair {} in its colours", salon::base_of(chr.hair)),
                            Desk::Faces => "the REG faces".to_string(),
                            Desk::Skins => "the skins this client draws; the client reads ids under 24000 as skins".to_string(),
                            Desk::EyeColours => format!("face {} in its eye colours", chr.face),
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
                    // No Mystery eye colour item exists, so tier_of_selection never gets here.
                    Desk::EyeColours => return Some(Vec::new()),
                };
                Some(self.apply_salon_look(desk, tier, look))
            }
        }
    }

    /// The pick-a-look box came back. `None` when no salon choice is parked or the body is
    /// not a type-0x0a reply, so the other parsers see it.
    pub(super) fn salon_choice_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        let eyes = convo.path == salon::EYE_CHOICE_PATH;
        if convo.path != salon::CHOICE_PATH && !eyes {
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
        // The eye-colour box was opened from the assistant's second line, not its own desk.
        let desk = if eyes { desk.second().unwrap_or(desk) } else { desk };
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
            Desk::EyeColours => Look::Face(picked),
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use store::Store;

    use crate::config::Config;
    use crate::salon;
    use crate::session::{Reply, Session};

    fn npc_click(object_id: u32) -> Vec<u8> {
        let mut b = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
        b.extend_from_slice(&object_id.to_le_bytes());
        b.extend_from_slice(&0i16.to_le_bytes());
        b.extend_from_slice(&0i16.to_le_bytes());
        b.extend_from_slice(&u32::MAX.to_le_bytes());
        b
    }

    fn menu_reply(selection: u32) -> Vec<u8> {
        let mut b = 0u32.to_le_bytes().to_vec();
        b.push(net::script::SCRIPT_TYPE_MENU);
        b.push(1);
        b.extend_from_slice(&selection.to_le_bytes());
        b
    }

    fn avatar_pick(index: u8) -> Vec<u8> {
        let mut b = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&[net::script::SCRIPT_TYPE_AVATAR, 1, 0, 0]);
        b.extend_from_slice(&0u32.to_le_bytes());
        b.push(index);
        b
    }

    /// The coupon a type-0x0a pick box names - its first body field.
    fn coupon_in(body: &[u8]) -> u32 {
        let b = &body[net::script::SCRIPT_HEAD_LEN..];
        u32::from_le_bytes(b[..4].try_into().unwrap())
    }

    /// The candidate ids in a type-0x0a pick box.
    fn ids_in(body: &[u8]) -> Vec<u32> {
        let b = &body[net::script::SCRIPT_HEAD_LEN..];
        let text_len = u16::from_le_bytes([b[4], b[5]]) as usize;
        let at = 4 + 2 + text_len + 1;
        (0..b[at] as usize).map(|k| u32::from_le_bytes(b[at + 1 + k * 4..at + 5 + k * 4].try_into().unwrap())).collect()
    }

    /// A female character on `map` with `hair` and `face`, two NPCs on the map, every
    /// collaboration hair colour and face eye colour known to the tables.
    fn at(map: u32, owner: u32, assistant: u32, hair: u32, face: u32) -> (Session, Arc<Store>, u32) {
        let mut npcs = std::collections::HashMap::new();
        npcs.insert(
            map,
            vec![
                net::opcode::FieldNpc { object_id: 1000, template_id: owner, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 },
                net::opcode::FieldNpc { object_id: 1001, template_id: assistant, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 },
            ],
        );
        let mut hair_ids = std::collections::HashSet::new();
        let mut face_ids = std::collections::HashSet::new();
        for style in 0..7 {
            for c in 0..8 {
                hair_ids.insert(42_540 + style * 10 + c);
            }
        }
        for style in 22_035..=22_042 {
            for c in 0..9 {
                face_ids.insert(style + c * 100);
            }
        }
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Frieren".into(), gender: 1, hair, face, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.set_character_map(id, map).unwrap();
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::new(store.clone(), Arc::new(Config { npcs, hair_ids, face_ids, ..Config::default() }));
        s.claim_for_character(id);
        (s, store, id)
    }

    fn look(store: &Store, id: u32) -> (u32, u32) {
        let c = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
        (c.hair, c.face)
    }

    fn give(store: &Store, id: u32, coupon: u32) {
        store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(coupon, 1), 1).unwrap();
    }

    fn names(out: &[Reply]) -> Vec<String> {
        out.iter().map(|r| r.what.clone()).collect()
    }

    /// **Eye colour at the surgery assistant, for a collaboration face.** The owner, 2026-10-02:
    /// *"the collaboration hairs and eyes are not able to change colors ... but we do have those
    /// files"*. Frieren Face 22035 at Dr. Feeble with the Signature Eye Color Coupon: the menu's
    /// third line, then all nine eye colours 22035..22835; index 4 is 22435 - the face kept,
    /// the eye colour changed - the coupon gone, one 0x007C with the FACE bit. The skin coupon's
    /// line is unaffected, and an assistant with neither coupon names both.
    #[test]
    fn the_surgery_assistant_changes_a_collaboration_faces_eye_colour() {
        let (mut s, store, id) = at(salon::HENESYS_SURGERY_MAP, salon::DENMA, salon::DR_FEEBLE, 42_540, 22_035);

        let out = s.handle(&npc_click(1001));
        let text = String::from_utf8_lossy(&out[0].body).to_string();
        assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_SAY, "{:?}", names(&out));
        assert!(text.contains("#i5153000#") && text.contains("#i5152100#"), "both coupons named: {text}");

        give(&store, id, salon::SIGNATURE_EYE_COLOR_COUPON);
        let out = s.handle(&npc_click(1001));
        assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_MENU, "{:?}", names(&out));
        let text = String::from_utf8_lossy(&out[0].body).to_string();
        assert!(text.contains("#L2##i5152100#") && !text.contains("#L0#"), "{text}");

        let out = s.on_script_reply(&menu_reply(salon::MENU_SECOND));
        assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_AVATAR, "{:?}", names(&out));
        assert_eq!(coupon_in(&out[0].body), salon::SIGNATURE_EYE_COLOR_COUPON, "types the box as eye colour (0xd)");
        let all: Vec<u32> = (0..9).map(|c| 22_035 + c * 100).collect();
        assert_eq!(ids_in(&out[0].body), all, "every eye colour of Frieren Face");

        let out = s.handle(&avatar_pick(4));
        assert_eq!(look(&store, id), (42_540, 22_435), "the face kept, eye colour 4");
        let stat: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::stats::STAT_CHANGED).collect();
        assert_eq!(stat.len(), 1, "{:?}", names(&out));
        assert!(stat[0].what.contains("FACE bit -> 22435"), "{}", stat[0].what);
        assert!(
            store.bag_items(id, store::InventoryType::Cash).unwrap().iter().all(|r| r.item.item_id != salon::SIGNATURE_EYE_COLOR_COUPON),
            "the coupon is spent"
        );

        // Without the coupon the third line is refused, and nothing changes.
        s.handle(&npc_click(1001));
        assert!(s.on_script_reply(&menu_reply(salon::MENU_SECOND)).is_empty());
        assert_eq!(look(&store, id), (42_540, 22_435));
    }

    /// **Hair colour for a collaboration hair** - already offered by the salon assistant; pinned
    /// here because the owner reported it with the eyes. Frieren Hair 42540 at Brittany with the
    /// Signature Color Coupon: all eight colours 42540..42547; index 5 is blue, 42545.
    #[test]
    fn the_salon_assistant_changes_a_collaboration_hairs_colour() {
        let (mut s, store, id) = at(salon::HENESYS_SALON_MAP, salon::NATALIE, salon::BRITTANY, 42_540, 22_035);
        give(&store, id, salon::SIGNATURE_COLOR_COUPON);
        let out = s.handle(&npc_click(1001));
        assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_MENU, "{:?}", names(&out));
        let out = s.on_script_reply(&menu_reply(salon::MENU_SIGNATURE));
        assert_eq!(ids_in(&out[0].body), (42_540..=42_547).collect::<Vec<_>>());
        assert_eq!(coupon_in(&out[0].body), salon::SIGNATURE_COLOR_COUPON, "types the box as hair colour (0x17), not style");
        s.handle(&avatar_pick(5));
        assert_eq!(look(&store, id), (42_545, 22_035), "Frieren Hair in blue");
    }
}
