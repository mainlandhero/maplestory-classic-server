//! **Using a Megaphone or a Super Megaphone** - `0x0116`, answered with one `0x00AC` line to
//! everyone on this channel (Megaphone) or on every channel (Super Megaphone). The packets and
//! their evidence are `net::megaphone`.
//!
//! The owner, 2026-09-29: *"Super megaphone messages should send to clients across all channels (with
//! a pink background), and display the whisper icon depending on client selection. Megaphone
//! messages should send to client on the same channel (without the pink background), and display
//! the whisper icon depending on client selection."*
//!
//! Every channel is reached the way whispers already are: `crate::link::Link::everyone` is the
//! hub's directory of who is online where, and `Session::deliver_anywhere` hands a packet to this
//! channel's bus or, for anyone elsewhere, to the hub. No new frame on the link.
//!
//! Nothing authenticates: a megaphone speaks for whoever holds the socket.

use super::{Reply, Session};
use net::megaphone::{self, Speaker};

impl Session {
    /// A megaphone used from Cash `slot`. The caller has already checked the slot holds it.
    /// **Always answers** with the unlock the `0x0116` builder latched on.
    pub(super) fn use_megaphone(&mut self, opcode: u16, body: &[u8]) -> Vec<Reply> {
        let mut out = crate::mesodrop::unlock_unhandled_latching_request(opcode);
        let Some(chr) = self.claimed_character() else { return out };
        let Some(req) = megaphone::parse_megaphone_use(body) else {
            crate::server::log(&format!("   megaphone: a {} byte 0x0116 body did not decode; unlock only, nothing used", body.len()));
            return out;
        };
        let text = req.text.trim();
        if text.is_empty() {
            return out; // the client's dialog does not send one; nothing to say, nothing used
        }
        let super_ = req.item_id == megaphone::SUPER_MEGAPHONE;

        // **Spend it first**, from the slot the client named. A megaphone that could not be taken
        // says nothing.
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
            crate::server::log(&format!("   megaphone: character {} could not spend {} from Cash slot {}; nothing said", chr.id, req.item_id, req.slot));
            return out;
        }
        out.extend(self.stack_change_replies(store::InventoryType::Cash, req.slot, held - 1));

        let (account_id, world) = self.claimed.as_ref().map(|c| (c.account_id, c.world_id)).unwrap_or_default();
        let who = Speaker {
            name: &chr.name,
            account_id: u32::try_from(account_id).unwrap_or(0),
            character_id: chr.id,
            world: u8::try_from(world).unwrap_or(0),
        };
        let channel = u8::try_from(self.config.channel_id).unwrap_or(0);
        let body = if super_ {
            megaphone::super_megaphone(&who, text, channel, req.whisper)
        } else {
            megaphone::megaphone(&who, text, channel, req.whisper)
        };
        let reply = Reply {
            opcode: net::broadcast::BROADCAST_MSG,
            body,
            what: format!(
                "BroadcastMsg {} from {} ({}) on channel {channel}, whisper icon {}: {:?}",
                if super_ { "type 3 SUPER MEGAPHONE (kind 0xd, pink) - every channel" } else { "type 8 MEGAPHONE (kind 0xf) - this channel" },
                chr.name,
                chr.id,
                if req.whisper { "on" } else { "off" },
                megaphone::line(&chr.name, text)
            ),
        };

        // Who hears it. The speaker always does, on this connection.
        let mut everyone: Vec<u32> = if super_ {
            match crate::link::current() {
                Some(link) => link.everyone().into_iter().map(|(id, _)| id).collect(),
                None => self.bus().online_characters(), // no hub: this channel is the world
            }
        } else {
            self.bus().online_characters()
        };
        everyone.sort_unstable();
        everyone.dedup();
        let mut heard = 1usize;
        for id in everyone.into_iter().filter(|&id| id != chr.id) {
            let sent = if super_ {
                self.deliver_anywhere(id, reply.clone())
            } else {
                self.bus().publish_to_character_anywhere(id, reply.clone())
            };
            heard += usize::from(sent);
        }
        crate::server::log(&format!(
            "   megaphone: {} ({}) used a {} - {heard} player(s) sent the line",
            chr.name,
            chr.id,
            if super_ { "Super Megaphone" } else { "Megaphone" }
        ));
        out.push(reply);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::Fields;
    use std::sync::Arc;
    use store::Store;

    /// Two players on one channel; the first holds `n` of `item` in Cash slot 1.
    fn two_on_a_channel(item: u32, n: u16) -> (Arc<Store>, Session, Session, u32) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let fields = Arc::new(Fields::new());
        let config = Arc::new(crate::config::Config::default());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ss = Vec::new();
        for name in ["Shouter", "Listener"] {
            let chr = net::opcode::Character { name: name.to_string(), map_id: 100_000_000, level: 30, ..Default::default() };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
            s.claim_for_character(id);
            let _ = s.on_field_entered();
            ss.push((s, id));
        }
        let (mut a, id) = ss.remove(0);
        let (b, _) = ss.remove(0);
        store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(item, n), 100).unwrap();
        let _ = &mut a;
        (store, a, b, id)
    }

    fn use_body(item: u32, text: &str, whisper: u8) -> Vec<u8> {
        let mut b = 0x0fe8762eu32.to_le_bytes().to_vec();
        b.extend_from_slice(&1u16.to_le_bytes());
        b.extend_from_slice(&item.to_le_bytes());
        b.extend_from_slice(&(text.len() as u16).to_le_bytes());
        b.extend_from_slice(text.as_bytes());
        b.push(whisper);
        b
    }

    fn lines(out: &[Reply]) -> Vec<Vec<u8>> {
        out.iter().filter(|r| r.opcode == net::broadcast::BROADCAST_MSG).map(|r| r.body.clone()).collect()
    }

    /// The Super Megaphone: type 3 with the whisper choice, heard by the speaker and the other
    /// player, one spent of two. The Megaphone: type 8, whisper off as chosen. Both unlock.
    #[test]
    fn a_megaphone_is_spent_and_heard_by_the_channel() {
        for (item, kind, whisper) in [(megaphone::SUPER_MEGAPHONE, 3u8, 1u8), (megaphone::MEGAPHONE, 8u8, 0u8)] {
            let (store, mut a, mut b, id) = two_on_a_channel(item, 2);
            // Through the real dispatch: the opcode, then the body, as the client sends it.
            let mut packet = net::cashitem::CLIENT_USE_STAT_RESET_ITEM.to_le_bytes().to_vec();
            packet.extend(use_body(item, "Hello", whisper));
            let out = a.handle(&packet);
            let mine = lines(&out);
            assert_eq!(mine.len(), 1, "the speaker hears it once");
            assert_eq!(mine[0][0], kind);
            let expect = if kind == 3 {
                megaphone::super_megaphone(&Speaker { name: "Shouter", account_id: mine_account(&a), character_id: id, world: 0 }, "Hello", 0, whisper == 1)
            } else {
                megaphone::megaphone(&Speaker { name: "Shouter", account_id: mine_account(&a), character_id: id, world: 0 }, "Hello", 0, whisper == 1)
            };
            assert_eq!(mine[0], expect, "the whisper choice and the channel ride along");
            assert_eq!(lines(&b.tick(1_000)), vec![expect], "and the other player on the channel hears it");
            let left: u16 = store.bag_items(id, store::InventoryType::Cash).unwrap().iter().map(|r| r.item.kind.quantity()).sum();
            assert_eq!(left, 1, "one spent");
            assert!(out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED), "the unlock");
        }
    }

    /// Nothing held: nothing said, nothing spent - but still the unlock.
    #[test]
    fn no_megaphone_no_line() {
        let (_, mut a, mut b, _) = two_on_a_channel(megaphone::MEGAPHONE, 1);
        let out = a.use_megaphone(net::cashitem::CLIENT_USE_STAT_RESET_ITEM, &use_body(megaphone::SUPER_MEGAPHONE, "Hi", 1));
        assert!(lines(&out).is_empty() && lines(&b.tick(1_000)).is_empty());
        assert!(out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED));
    }

    fn mine_account(s: &Session) -> u32 {
        u32::try_from(s.claimed.as_ref().unwrap().account_id).unwrap()
    }
}
