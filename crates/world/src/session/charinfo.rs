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
//! with its numbers. Fame is `0`: the store does not keep one, and the character record has
//! sent `0` since the field was decoded. Guild is `""`: there are no guilds.

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
        // The pet the character has out, from the store - the other player's session owns the
        // live copy, but `active` is written on every summon and put-away, so the row is it.
        let pet = self.store.active_pet(id).ok().flatten().map(|row| {
            let state = self.store.pet_state(row.pet_id).unwrap_or_else(|_| store::PetState::fresh());
            let name = state
                .name
                .clone()
                .unwrap_or_else(|| self.config.item_names.get(&row.item_id).cloned().unwrap_or_default());
            let vitals = net::bag::PetVitals {
                level: state.level,
                closeness: u16::try_from(state.closeness).unwrap_or(u16::MAX),
                fullness: state.fullness,
                skills: state.skills,
            };
            net::charinfo::PetPanel {
                item_id: row.item_id,
                name: name.clone(),
                level: u32::from(state.level),
                closeness: state.closeness,
                fullness: u32::from(state.fullness),
                item: net::bag::pet_item_with_state(row.item_id, &name, Some(net::pet::pet_serial(id, row.pet_id)), 1, &vitals),
            }
        });
        let info = net::charinfo::CharacterInfo {
            character_id: id,
            name: brief.name.clone(),
            level: brief.level,
            job: brief.job,
            fame: 0,
            guild: String::new(),
            pet: pet.clone(),
            show_pet_panel: req.pet_info,
        };
        crate::server::log(&format!(
            "   character info: {} ({id}) asked for by {:?}: level {}, job {}, pet {}",
            brief.name,
            self.claimed.as_ref().map(|c| c.character_id),
            brief.level,
            brief.job,
            pet.as_ref().map(|p| format!("{} ({})", p.name, p.item_id)).unwrap_or_else(|| "none".into())
        ));
        vec![Reply {
            opcode: net::charinfo::CHARACTER_INFO,
            body: net::charinfo::character_info(&info),
            what: format!(
                "CharacterInfo: {} ({id}) - level {}, job {}, fame 0, no guild, pet {}{}. Opens the Character Info window for another player; clears the request latch.",
                brief.name,
                brief.level,
                brief.job,
                pet.as_ref().map(|p| format!("{} lv {} closeness {} fullness {}", p.name, p.level, p.closeness, p.fullness)).unwrap_or_else(|| "none".into()),
                if req.pet_info && pet.is_some() { ", pet panel open" } else { "" }
            ),
        }]
    }
}
