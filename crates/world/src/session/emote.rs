//! **Emotes, seen by everyone on the map.**
//!
//! The owner, 2026-09-29: *"I just tried playing the Queasy emote as the owner, can we make sure that the
//! emotes are relayed to other clients in the same map as well please?"* The client sends
//! `0x00EA` after drawing the face on itself, and the server had never answered it - so only the
//! player who pressed the key saw it. Now it goes to everyone else on the field as `0x02A6`,
//! the remote twin whose handler calls the same face-setter with the same three values
//! (`net::userpool::USER_EMOTION_REMOTE`).
//!
//! Nothing comes back to the sender: its own client has already drawn the face.
//!
//! # Effect items - Shadow Style, the same shape
//!
//! The owner, 2026-09-29: *"I just tried turning on Shadow Style ... Upon double click, the server
//! should change the effect from OFF to ON and start animating the effect to the client and
//! other characters on the same map."* `0x00EC` carries the item (or `0` for off) and, like the
//! emote, the client has already applied it to itself. So the server checks it, remembers it,
//! puts it in this character's `0x0224` for later arrivals, and tells everyone else on the map
//! with `0x02A8` (`net::userpool::USER_EFFECT_ITEM_REMOTE`).
//!
//! # Saved between logins - and the one thing the server cannot do
//!
//! The owner, 2026-09-30: *"The effect should persist and should be saved between logins."* It is
//! saved on every switch (`store::effectitem`) and restored at claim, so the character arrives
//! with it on for **everyone else** - their `0x0224` carries it.
//!
//! **The player's own client starts with it OFF after a login or a channel change, and no
//! packet can switch it on there.** Measured statically, 2026-09-30: the local character's
//! effect is `context+0x2394` (`FUN_142CE2DC0`), which the character-data field entry clears
//! (`FUN_142CAD420`, from `0x14209EE50`); its only other writers are the setter itself and two
//! item-removal paths that clear it; `0x02A8` naming one's own id is dropped, because the remote
//! range's dispatcher (`FUN_1429BB720`) looks only in the remote-user hash. So the owner sees it
//! again after one double-click - which sends the saved item, is the "no change" case here,
//! and moves nothing for anyone else.

use super::{Reply, Session};

impl Session {
    /// `0x00EA` - relay the emote to the rest of the field, unchanged. A body of the wrong
    /// length is not this packet and is not relayed.
    pub(super) fn on_emotion(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let Some(relay) = net::userpool::user_emotion_remote(chr.id, body) else {
            crate::server::log(&format!("   emote: character {} sent a {}-byte 0x00EA; not relayed", chr.id, body.len()));
            return Vec::new();
        };
        let emotion = u32::from_le_bytes([body[0], body[1], body[2], body[3]]);
        let reply = Reply {
            opcode: net::userpool::USER_EMOTION_REMOTE,
            body: relay,
            what: format!("UserEmotion 0x02A6: {} makes face {emotion} (the sender's own 0x00EA, relayed)", chr.name),
        };
        self.bus().publish(self.subscriber, self.field_of(&chr), reply, None);
        Vec::new()
    }
}

impl Session {
    /// `0x00EC` - an effect item on or off. The item must be in the effect range and in this
    /// character's Cash tab; `0` is always allowed (off). A refused switch is logged and
    /// relayed to nobody - the sender's own client may already show it, which is the client's
    /// own doing and harmless to anyone else.
    pub(super) fn on_effect_item(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let Some(item) = net::userpool::parse_effect_item(body) else { return Vec::new() };
        if item != 0 && (!net::userpool::EFFECT_ITEMS.contains(&item) || self.held_count(chr.id, item) == 0) {
            crate::server::log(&format!("   effect item: {} asked for {item}, which is not an effect item they hold - not switched", chr.name));
            return Vec::new();
        }
        if item == self.active_effect_item {
            return Vec::new();
        }
        self.active_effect_item = item;
        if let Err(e) = self.store.set_effect_item(chr.id, item) {
            crate::server::log(&format!("   effect item: {}'s switch to {item} NOT saved: {e}", chr.name));
        }
        // A later arrival sees it from the spawn; the ones already here from the relay.
        let spawn = self.presence(&chr).spawn;
        self.bus().refresh_spawn(self.subscriber, spawn);
        let reply = Reply {
            opcode: net::userpool::USER_EFFECT_ITEM_REMOTE,
            body: net::userpool::user_effect_item_remote(chr.id, item),
            what: if item == 0 {
                format!("UserEffectItem 0x02A8: {} switched their effect item OFF", chr.name)
            } else {
                format!("UserEffectItem 0x02A8: {} switched effect item {item} ON", chr.name)
            },
        };
        self.bus().publish(self.subscriber, self.field_of(&chr), reply, None);
        Vec::new()
    }
}

impl Session {
    /// The effect item to start this connection with: the saved one, if it is still an effect
    /// item this character holds. One that is not (sold, expired, moved to storage) is
    /// forgotten rather than shown.
    pub(super) fn restored_effect_item(&self, character_id: u32) -> u32 {
        let item = self.store.effect_item(character_id).unwrap_or(0);
        if item == 0 {
            return 0;
        }
        if net::userpool::EFFECT_ITEMS.contains(&item) && self.held_count(character_id, item) > 0 {
            return item;
        }
        let _ = self.store.set_effect_item(character_id, 0);
        crate::server::log(&format!("   effect item: character {character_id}'s saved {item} is no longer held - forgotten"));
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::Fields;
    use std::sync::Arc;
    use store::Store;

    const MAP: u32 = 10_000_000;

    fn two_on(maps: [u32; 3]) -> (Vec<(Session, u32)>, Arc<Fields>) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let fields = Arc::new(Fields::new());
        let mut cfg = crate::config::Config::default();
        for m in maps {
            cfg.fields.insert(m);
        }
        let cfg = Arc::new(cfg);
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut out = Vec::new();
        for (i, map) in maps.iter().enumerate() {
            let chr = net::opcode::Character { name: format!("Face{i}"), map_id: *map, level: 10, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, *map).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), cfg.clone(), fields.clone());
            s.claim_for_character(id);
            let _ = s.on_field_entered();
            out.push((s, id));
        }
        (out, fields)
    }

    fn emote(body: &[u8]) -> Vec<u8> {
        let mut b = net::userpool::CLIENT_EMOTION.to_le_bytes().to_vec();
        b.extend_from_slice(body);
        b
    }

    /// The owner's Queasy, byte for byte from the capture: the player on the same map gets
    /// `charId` + those nine bytes as `0x02A6`; the sender gets nothing back; a player on
    /// another map gets nothing.
    #[test]
    fn an_emote_reaches_the_rest_of_the_map_and_nobody_else() {
        let (mut ss, _) = two_on([MAP, MAP, MAP + 1]);
        let queasy = [0x08, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0x00];
        let sender = ss[0].1;
        let out = ss[0].0.handle(&emote(&queasy));
        assert!(!out.iter().any(|r| r.opcode == net::userpool::USER_EMOTION_REMOTE), "nothing back: {out:?}");
        let want = [&sender.to_le_bytes()[..], &queasy].concat();
        let seen = ss[1].0.tick(1_000);
        assert!(seen.iter().any(|r| r.opcode == net::userpool::USER_EMOTION_REMOTE && r.body == want), "{seen:?}");
        let elsewhere = ss[2].0.tick(1_000);
        assert!(!elsewhere.iter().any(|r| r.opcode == net::userpool::USER_EMOTION_REMOTE), "another map");
    }

    const SHADOW_STYLE: u32 = 5_010_005;

    fn effect_item(item: u32) -> Vec<u8> {
        let mut b = net::userpool::CLIENT_EFFECT_ITEM.to_le_bytes().to_vec();
        b.extend_from_slice(&item.to_le_bytes());
        b.extend_from_slice(&5u32.to_le_bytes());
        b
    }

    fn effect_seen(s: &mut Session) -> Vec<Vec<u8>> {
        s.tick(1_000).into_iter().filter(|r| r.opcode == net::userpool::USER_EFFECT_ITEM_REMOTE).map(|r| r.body).collect()
    }

    /// Shadow Style, as the owner's client sent it: ON reaches the rest of the map as `0x02A8`
    /// (charId, 5010005) and goes into the sender's `0x0224` for whoever arrives later; OFF
    /// (item 0) reaches them as `0x02A8` (charId, 0). An item they do not hold, or one outside
    /// the effect range, switches nothing.
    #[test]
    fn shadow_style_is_seen_by_the_map_and_by_later_arrivals() {
        let (mut ss, fields) = two_on([MAP, MAP, MAP + 1]);
        let (sender, store) = (ss[0].1, ss[0].0.store.clone());
        let _ = ss[0].0.handle(&effect_item(SHADOW_STYLE));
        assert!(effect_seen(&mut ss[1].0).is_empty(), "not held: nothing");
        store.add_item(sender, store::InventoryType::Cash, &store::Item::bundle(SHADOW_STYLE, 1), 1).unwrap();
        let _ = ss[0].0.handle(&effect_item(2_000_000));
        assert!(effect_seen(&mut ss[1].0).is_empty(), "not an effect item");

        let out = ss[0].0.handle(&effect_item(SHADOW_STYLE));
        assert!(out.is_empty(), "nothing back - the client drew it itself");
        let on = net::userpool::user_effect_item_remote(sender, SHADOW_STYLE);
        assert_eq!(effect_seen(&mut ss[1].0), vec![on]);
        assert!(effect_seen(&mut ss[2].0).is_empty(), "another map");
        // **A later arrival**: the player on the other map walks in and is handed the sender's
        // 0x0224 - which must carry Shadow Style at offset 395.
        let _ = fields;
        let mut late = ss[2].0.claimed_character().unwrap();
        let _ = ss[2].0.go_to_map(&mut late, MAP, 0, "walks in after the switch".to_string());
        let arrived = ss[2].0.on_field_entered();
        let spawn = arrived
            .iter()
            .find(|r| r.opcode == net::userpool::USER_ENTER_FIELD && r.body[4..8] == sender.to_le_bytes())
            .expect("the sender's 0x0224");
        assert!(spawn.body.windows(4).any(|w| w == SHADOW_STYLE.to_le_bytes()), "effect item in the spawn");
        assert_eq!(ss[0].0.remote_at().active_effect_item, SHADOW_STYLE);

        let _ = ss[0].0.handle(&effect_item(0));
        assert_eq!(effect_seen(&mut ss[1].0), vec![net::userpool::user_effect_item_remote(sender, 0)], "off");
        assert_eq!(ss[0].0.remote_at().active_effect_item, 0);
    }

    /// **Saved between logins** (the owner, 2026-09-30): switched on, the character's next
    /// connection starts with it on - in its `0x0224` for everyone else. Switched off, it
    /// starts off. And a saved item no longer held is forgotten.
    #[test]
    fn the_effect_item_survives_a_relog_and_is_forgotten_once_it_is_gone() {
        let (mut ss, _) = two_on([MAP, MAP, MAP]);
        let (id, store) = (ss[0].1, ss[0].0.store.clone());
        store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(SHADOW_STYLE, 1), 1).unwrap();
        let _ = ss[0].0.handle(&effect_item(SHADOW_STYLE));
        assert_eq!(store.effect_item(id).unwrap(), SHADOW_STYLE, "saved");

        let relog = |store: &Arc<Store>, id: u32| {
            let account = store.get_account("maplecw").unwrap().unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), ss_config(), Arc::new(Fields::new()));
            s.claim_for_character(id);
            s
        };
        let again = relog(&store, id);
        assert_eq!(again.remote_at().active_effect_item, SHADOW_STYLE, "back on after a relog");

        let _ = ss[0].0.handle(&effect_item(0));
        assert_eq!(relog(&store, id).remote_at().active_effect_item, 0, "off stays off");

        let _ = ss[0].0.handle(&effect_item(SHADOW_STYLE));
        let slot = store.bag_items(id, store::InventoryType::Cash).unwrap()[0].slot;
        store.remove_item(id, store::InventoryType::Cash, slot, None).unwrap();
        assert_eq!(relog(&store, id).remote_at().active_effect_item, 0, "no longer held: forgotten");
        assert_eq!(store.effect_item(id).unwrap(), 0);
    }

    fn ss_config() -> Arc<crate::config::Config> {
        let mut cfg = crate::config::Config::default();
        cfg.fields.insert(MAP);
        Arc::new(cfg)
    }

    /// A body of the wrong shape is not relayed.
    #[test]
    fn a_malformed_emote_is_not_relayed() {
        let (mut ss, _) = two_on([MAP, MAP, MAP]);
        let _ = ss[0].0.handle(&emote(&[8, 0, 0, 0]));
        assert!(!ss[1].0.tick(1_000).iter().any(|r| r.opcode == net::userpool::USER_EMOTION_REMOTE));
        assert_eq!(net::userpool::user_emotion_remote(1, &[0; 10]), None);
    }
}
