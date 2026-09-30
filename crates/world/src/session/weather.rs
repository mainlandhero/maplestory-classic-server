//! **Using a weather item** - Sprinkled Chocolate and the rest of `Item/Cash/0512.img`. One is
//! spent and everyone on the map gets `0x01B7`: the item's own falling effect with the player's
//! message, for thirty seconds, after which the client fades it out. `net::weather` has the
//! packet and the evidence.
//!
//! The owner, 2026-09-30: *"display my chosen message with the particular item effect as an
//! atmospheric effect for everyone present in the map for 30 seconds then gradually fade out."*
//!
//! **One at a time per map.** A second item while one is running is refused and kept - the
//! client draws one weather object per field, and a second `0x01B7` would cut the first short.
//! Someone who walks in while one is running gets the rest of it on field entry.
//!
//! Nothing authenticates: the effect is cast by whoever holds the socket.

use super::{Reply, Session};

impl Session {
    /// A weather item used from Cash `slot`. The caller has already checked the slot holds it.
    /// **Always answers** with the unlock the `0x0116` builder latched on.
    pub(super) fn use_weather_item(&mut self, opcode: u16, body: &[u8]) -> Vec<Reply> {
        self.use_weather_item_at(opcode, body, store::Store::unix_now())
    }

    pub(super) fn use_weather_item_at(&mut self, opcode: u16, body: &[u8], now: i64) -> Vec<Reply> {
        let mut out = crate::mesodrop::unlock_unhandled_latching_request(opcode);
        let Some(chr) = self.claimed_character() else { return out };
        let Some(req) = net::weather::parse_weather_use(body) else {
            crate::server::log(&format!("   weather: a {} byte 0x0116 body did not decode; unlock only, nothing used", body.len()));
            return out;
        };
        let map = self.field_of(&chr);
        let running = self.fields.weather().get(&map).filter(|w| w.2 > now).map(|w| w.2 - now);
        if let Some(left) = running {
            out.extend(self.notice(format!("Another effect is already showing on this map. Try again in {left} seconds.")));
            return out;
        }
        let held = self
            .store
            .bag_items(chr.id, store::InventoryType::Cash)
            .ok()
            .into_iter()
            .flatten()
            .find(|r| r.slot == req.slot && r.item.item_id == req.item_id)
            .map(|r| r.item.kind.quantity())
            .unwrap_or(0);
        if held == 0 || self.store.remove_item(chr.id, store::InventoryType::Cash, req.slot, Some(1)).is_err() {
            crate::server::log(&format!("   weather: character {} could not spend {} from Cash slot {}; nothing shown", chr.id, req.item_id, req.slot));
            return out;
        }
        out.extend(self.stack_change_replies(store::InventoryType::Cash, req.slot, held - 1));

        let seconds = net::weather::SECONDS;
        self.fields.weather().insert(map, (req.item_id, req.text.clone(), now + i64::from(seconds)));
        let reply = weather_reply(req.item_id, &req.text, seconds, &format!("{} ({}) used it on {map}", chr.name, chr.id));
        self.bus().publish(self.subscriber, map, reply.clone(), None);
        out.push(reply);
        crate::server::log(&format!("   weather: {} ({}) used {} on {map} for {seconds}s: {:?}", chr.name, chr.id, req.item_id, req.text));
        out
    }

    /// **On field entry**: the rest of an effect still running here. Called from
    /// `on_field_entered`.
    pub(super) fn weather_on_entry(&mut self) -> Vec<Reply> {
        self.weather_on_entry_at(store::Store::unix_now())
    }

    pub(super) fn weather_on_entry_at(&mut self, now: i64) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let map = self.field_of(&chr);
        let running = self.fields.weather().get(&map).filter(|w| w.2 > now).cloned();
        let Some((item, text, until)) = running else { return Vec::new() };
        let left = u32::try_from(until - now).unwrap_or(0);
        vec![weather_reply(item, &text, left, &format!("the rest of the effect already on {map}, for character {}", chr.id))]
    }
}

fn weather_reply(item: u32, text: &str, seconds: u32, why: &str) -> Reply {
    Reply {
        opcode: net::weather::BLOW_WEATHER,
        body: net::weather::blow_weather(item, text, seconds),
        what: format!("BlowWeather 0x01B7: item {item} for {seconds}s with {text:?} - {why}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::Fields;
    use std::sync::Arc;
    use store::Store;

    const CHOCOLATE: u32 = 5_120_005;

    /// Three players: two on one map, one elsewhere. The first holds `n` Sprinkled Chocolates.
    fn three(n: u16) -> (Arc<Store>, Vec<Session>, u32) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let fields = Arc::new(Fields::new());
        let config = Arc::new(crate::config::Config::default());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ss = Vec::new();
        let mut first = 0;
        for (name, map) in [("Sprinkler", 100_000_000), ("Onlooker", 100_000_000), ("Elsewhere", 101_000_000)] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: map, level: 30, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.set_character_map(id, map).unwrap();
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            let _ = s.on_field_entered();
            if first == 0 {
                first = id;
            }
            ss.push(s);
        }
        store.add_item(first, store::InventoryType::Cash, &store::Item::bundle(CHOCOLATE, n), 100).unwrap();
        (store, ss, first)
    }

    fn use_packet(item: u32, text: &str) -> Vec<u8> {
        let mut b = net::cashitem::CLIENT_USE_STAT_RESET_ITEM.to_le_bytes().to_vec();
        b.extend_from_slice(&0x0ff07590u32.to_le_bytes());
        b.extend_from_slice(&1u16.to_le_bytes());
        b.extend_from_slice(&item.to_le_bytes());
        b.extend_from_slice(&(text.len() as u16).to_le_bytes());
        b.extend_from_slice(text.as_bytes());
        b
    }

    fn effects(out: &[Reply]) -> Vec<Vec<u8>> {
        out.iter().filter(|r| r.opcode == net::weather::BLOW_WEATHER).map(|r| r.body.clone()).collect()
    }

    /// Through the real dispatch: one spent, the effect for thirty seconds on the user's screen
    /// and on the other player's on that map, not on the third's; a second while it runs is
    /// refused and kept; someone arriving mid-effect gets the remainder.
    #[test]
    fn a_weather_item_shows_on_the_map_for_thirty_seconds() {
        let (store, mut ss, id) = three(2);
        let text = "Sprinkler's Chocolatey Message: hi";
        let out = ss[0].handle(&use_packet(CHOCOLATE, text));
        let expect = net::weather::blow_weather(CHOCOLATE, text, 30);
        assert_eq!(effects(&out), vec![expect.clone()], "the user sees it");
        assert!(out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED), "the unlock");
        assert_eq!(effects(&ss[1].tick(1_000)), vec![expect], "so does the other player on the map");
        assert!(effects(&ss[2].tick(1_000)).is_empty(), "nobody on another map");
        let held = || -> u16 { store.bag_items(id, store::InventoryType::Cash).unwrap().iter().map(|r| r.item.kind.quantity()).sum() };
        assert_eq!(held(), 1, "one spent");

        // A second while it runs: refused, kept, nothing new on screen.
        let out = ss[0].handle(&use_packet(CHOCOLATE, "again"));
        assert!(effects(&out).is_empty());
        assert_eq!(held(), 1, "kept");
        assert!(out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED), "still the unlock");

        // Arriving mid-effect: the rest of it.
        let now = Store::unix_now();
        let late = ss[2].weather_on_entry_at(now); // still on the other map: nothing
        assert!(effects(&late).is_empty());
        let mut chr = ss[2].claimed_character().unwrap();
        let _ = ss[2].go_to_map(&mut chr, 100_000_000, 0, "walks in".to_string());
        let entry = ss[2].on_field_entered();
        assert_eq!(effects(&entry).len(), 1, "the real field entry carries it");
        let late = ss[2].weather_on_entry_at(now + 10);
        assert_eq!(effects(&late), vec![net::weather::blow_weather(CHOCOLATE, text, 20)], "twenty seconds left");
        assert!(effects(&ss[2].weather_on_entry_at(now + 31)).is_empty(), "and nothing once it has run out");
    }
}
