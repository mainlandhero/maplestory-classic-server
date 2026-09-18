//! `0x01FC` - a double-click on another player: answer with their Character Info.
//!
//! The owner, 2026-09-18: *"When double clicking another player, a similar Character Info window
//! should show for as well for players that are not yourself."* The click sent `0x01FC` naming
//! character 214 and nothing answered it (`world-ch0.log` 15:27:47). The reply is `0x00A2`,
//! `net::charinfo`; `research/character-info-2026-09-18.md` is the decode.
//!
//! **Every request is answered.** Both client-side builders set the shared exclusive-request
//! latch, so a request left unanswered freezes some 35 other request senders until the next
//! field entry. A character that does not exist gets the four-byte refusal, which clears the
//! latch and shows nothing.
//!
//! The window draws the avatar from its own user pool when the character is on this map and
//! skips the avatar otherwise, so a character on another map or channel still gets a window
//! with its numbers. Fame is `characters.fame` (`store::fame`, since 2026-09-18 - it was a
//! literal 0 before that). Guild is `""`: there are no guilds.
//!
//! **The ITEM tab** (the "Item List" panel) is what the character is wearing: the hair, the
//! face, then every row of `equipment` - the regular slots and the cash covers above 100 -
//! as whole equip slots. The owner, 2026-09-18: *"This item list should include the hair, face,
//! equipment and cash shop cover items that the player is wearing."* and, shown a modern
//! client, *"Showing hair and face is absolutely do-able."*
//!
//! A hair or face IS an equip to this client's item lookup - `Character/<Type>/<id>.img`
//! with `id / 10000` = 3 -> Hair, 2 -> Face, the same path a cap takes - so the slot is an
//! ordinary equip body under the look id. What the classic data lacked was an `info/icon` on
//! those images (both the classic and the modern Hair/Face images carry only
//! `info/{islot,vslot,cash}`; the modern window draws those cells in UI code), so
//! `tools/backport_install.py` now renders one per hair and face from the part's own default
//! frame and writes it into the hybrid archives (`look_icons`). **Until that install is on
//! the client, a hair or face entry asks the widget for an icon that is not there** - the
//! config switch `charinfo_look_items` (default on) is the way to send equips only. [I] that
//! the widget draws a cash cover, a hair and a face the way it draws a cap; nothing about the
//! list has been on a screen yet. Skin is not listed: its image is `Character/0000200x.img`
//! at the tree root, which the installer does not build yet.

use super::{Reply, Session};

impl Session {
    pub(super) fn on_character_info_request(&mut self, body: &[u8]) -> Vec<Reply> {
        let refuse = |why: String| {
            crate::server::log(&format!("   character info: {why}; refused (4 bytes), the latch clears"));
            vec![Reply {
                opcode: net::charinfo::CHARACTER_INFO,
                body: net::charinfo::character_info_refused(),
                what: format!("CharacterInfo REFUSED: {why}. result != 0 - the client clears its request latch and shows nothing."),
            }]
        };
        let Some(req) = net::charinfo::parse_character_info_request(body) else {
            return refuse(format!("a {} byte 0x01FC body that does not parse", body.len()));
        };
        let id = if req.character_id != 0 {
            req.character_id
        } else {
            match self.store.character_id_by_name(&req.name) {
                Ok(Some(id)) => id,
                _ => return refuse(format!("no character named {:?}", req.name)),
            }
        };
        let Ok(Some(brief)) = self.store.character_brief(id) else {
            return refuse(format!("no character {id}"));
        };
        let worn = self.store.equipped_items(id).unwrap_or_default();
        // The pet the character has out, from the store - the other player's session owns the
        // live copy, but `active` is written on every summon and put-away, so the row is it.
        // The cell under the pet is the pet's EQUIP (the hat in worn slot 114), not the pet
        // item: the owner, 2026-09-18, "that slot is blank" with the pet item sent there. The hat's
        // body comes from its worn row, so a scrolled hat arrives scrolled (the owner, the same
        // day: "make sure that the Character Info shows all scrolled information").
        let pet = self.store.active_pet(id).ok().flatten().map(|row| {
            let state = self.store.pet_state(row.pet_id).unwrap_or_else(|_| store::PetState::fresh());
            let name = state
                .name
                .clone()
                .unwrap_or_else(|| self.config.item_names.get(&row.item_id).cloned().unwrap_or_default());
            let wear = worn.iter().find(|e| e.slot == super::pet::PET_EQUIP_WORN_SLOT).map(|e| {
                let stats = e.stats.unwrap_or_else(|| self.template_stats(e.item_id));
                (e.item_id, net::opcode::equipped_item(e.item_id, &stats))
            });
            net::charinfo::PetPanel {
                item_id: row.item_id,
                name,
                level: u32::from(state.level),
                closeness: state.closeness,
                fullness: u32::from(state.fullness),
                wear,
            }
        });
        let fame = self.store.fame(id).ok().flatten().unwrap_or(0);
        // The look first - hair, face - then everything worn, as the record would send it:
        // template stats for a row that never stored its own. Slot order, hat first.
        let mut items: Vec<Vec<u8>> = Vec::new();
        if self.config.charinfo_look_items {
            for look in [brief.hair, brief.face] {
                if look != 0 {
                    items.push(net::opcode::equipped_item(look, &net::opcode::EquipStats::default()));
                }
            }
        }
        items.extend(self.store.equipped_items(id).unwrap_or_default().iter().map(|e| {
            let stats = e.stats.unwrap_or_else(|| self.template_stats(e.item_id));
            net::opcode::equipped_item(e.item_id, &stats)
        }));
        let info = net::charinfo::CharacterInfo {
            character_id: id,
            name: brief.name.clone(),
            level: brief.level,
            job: brief.job,
            fame: fame as u32,
            guild: String::new(),
            pet: pet.clone(),
            show_pet_panel: req.pet_info,
            items,
        };
        crate::server::log(&format!(
            "   character info: {} ({id}) asked for by {:?}: level {}, job {}, fame {fame}, {} worn item(s) in the list, pet {}",
            brief.name,
            self.claimed.as_ref().map(|c| c.character_id),
            brief.level,
            brief.job,
            info.items.len(),
            pet.as_ref().map(|p| format!("{} ({})", p.name, p.item_id)).unwrap_or_else(|| "none".into())
        ));
        vec![Reply {
            opcode: net::charinfo::CHARACTER_INFO,
            body: net::charinfo::character_info(&info),
            what: format!(
                "CharacterInfo: {} ({id}) - level {}, job {}, fame {fame}, no guild, {} worn item(s) in the ITEM tab, pet {}{}. Opens the Character Info window for another player; clears the request latch.",
                brief.name,
                brief.level,
                brief.job,
                info.items.len(),
                pet.as_ref()
                    .map(|p| format!(
                        "{} lv {} closeness {} fullness {}, wearing {}",
                        p.name,
                        p.level,
                        p.closeness,
                        p.fullness,
                        p.wear.as_ref().map(|(hat, _)| hat.to_string()).unwrap_or_else(|| "nothing".into())
                    ))
                    .unwrap_or_else(|| "none".into()),
                if req.pet_info && pet.is_some() { ", pet panel open" } else { "" }
            ),
        }]
    }
}
