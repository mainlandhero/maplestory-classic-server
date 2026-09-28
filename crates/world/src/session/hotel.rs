//! **The Sleepywood Hotel Receptionist** - a menu of the two saunas, paid for in mesos.
//!
//! The owner, 2026-09-28, with the reference page's remarks - *"Purchase Regular Sauna room for 499
//! mesos"*, *"Purchase VIP Sauna room for 999 mesos"*: *"It should be a NPC dialogue selection
//! for players to figure out which sauna they choose to enter and deduct mesos accordingly to
//! their choice."*
//!
//! [L] from `gm-handbook/`: the Hotel Receptionist is template 606, placed only in Sleepywood
//! Hotel (10005001); Regular Sauna is 10005002 and VIP Sauna 10005003, and each sauna's `out00`
//! leads back to the hotel's `st00`. The words below are this server's, not the client's.
//!
//! Nothing here authenticates: a room is sold to whoever holds the socket.

use super::{Conversation, Reply, Session};

/// The Hotel Receptionist.
pub(super) const RECEPTIONIST: u32 = 606;
/// Sleepywood Hotel, where they stand.
pub(super) const HOTEL: u32 = 10_005_001;
/// The conversation path their menu is parked under.
pub(super) const HOTEL_PATH: &str = "hotel.receptionist";

/// The two rooms, in menu order: `#L` line, name, map, price. The owner's prices.
pub(super) const ROOMS: [(u32, &str, u32, u32); 2] = [
    (0, "Regular Sauna", 10_005_002, 499),
    (1, "VIP Sauna", 10_005_003, 999),
];

fn menu() -> String {
    let mut text = "Welcome to the Sleepywood Hotel. If you're worn out from hunting, a stay in one of our \
                    saunas will have you rested in no time. Which room would you like?"
        .to_string();
    for (line, name, _, price) in ROOMS {
        text.push_str(&format!("\r\n#d#L{line}##b{name}#d - {price} mesos#l"));
    }
    text.push_str("#k");
    text
}

impl Session {
    /// A click on the Hotel Receptionist in the hotel. `None` for anyone else, anywhere else.
    pub(super) fn open_hotel_receptionist(&mut self, template: u32) -> Option<Vec<Reply>> {
        let chr = self.claimed_character()?;
        if template != RECEPTIONIST || chr.map_id != HOTEL {
            return None;
        }
        self.conversation = Some(Conversation {
            npc_template: template,
            quest_id: None,
            path: HOTEL_PATH.to_string(),
            sent: 0,
            awaiting_yes_no: false,
            sent_with_next: false,
        });
        Some(vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_menu(template, &menu()),
            what: "ScriptMessage MENU from the Hotel Receptionist: Regular or VIP Sauna".to_string(),
        }])
    }

    /// Their menu came back: take the price, then the room. `None` when their menu is not parked.
    pub(super) fn hotel_menu_answer(&mut self, body: &[u8]) -> Option<Vec<Reply>> {
        let convo = self.conversation.clone()?;
        if convo.path != HOTEL_PATH {
            return None;
        }
        let reply = net::script::parse_menu_reply(body)?;
        self.conversation = None;
        let Some((_, name, map, price)) = reply.selection.and_then(|s| ROOMS.into_iter().find(|r| r.0 == s)) else {
            return Some(Vec::new());
        };
        let mut chr = self.claimed_character()?;
        if chr.map_id != HOTEL {
            return Some(Vec::new());
        }
        let say = |line: String, what: String| Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(RECEPTIONIST, &line, false, false),
            what: format!("ScriptMessage Say from the Hotel Receptionist: {what}"),
        };
        // **The mesos first**, one store write that refuses a short purse; only a charge that
        // went through moves anyone.
        match self.store.add_mesos(chr.id, -i64::from(price)) {
            Ok(balance) => {
                crate::server::log(&format!(
                    "   hotel: {} ({}) paid {price} for the {name} - {balance} mesos left",
                    chr.name, chr.id
                ));
                let mut out = self.meso_reply(chr.id);
                out.push(Reply {
                    opcode: net::message::MESSAGE,
                    body: net::message::meso_lost_line(price),
                    what: format!("Message: grey chat line, {price} mesos for the {name}"),
                });
                out.extend(self.teleport(&mut chr, map, format!("the Hotel Receptionist: the {name}")));
                Some(out)
            }
            Err(store::StoreError::NotEnoughMesos { have, .. }) => Some(vec![say(
                format!("I'm sorry, but the #b{name}#k is #b{price} mesos#k, and you don't have enough."),
                format!("{have} mesos is short of {price} for the {name}"),
            )]),
            Err(e) => {
                crate::server::log(&format!("   hotel: could not charge {} for the {name}: {e}", chr.id));
                Some(vec![say("I can't take your payment just now. Please try again in a moment.".to_string(), format!("the charge failed - {e}"))])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::Fields;
    use std::sync::Arc;
    use store::Store;

    fn guest(mesos: u32) -> (Arc<Store>, Session, u32) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let mut cfg = crate::config::Config::default();
        for m in [HOTEL, ROOMS[0].2, ROOMS[1].2] {
            cfg.fields.insert(m);
        }
        cfg.npcs.insert(
            HOTEL,
            vec![net::opcode::FieldNpc { object_id: 970, template_id: RECEPTIONIST, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 }],
        );
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Sleeper".to_string(), map_id: HOTEL, level: 30, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.set_character_map(id, HOTEL).unwrap();
        store.set_mesos(id, mesos).unwrap();
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store.clone(), Arc::new(cfg), Arc::new(Fields::new()));
        s.claim_for_character(id);
        (store, s, id)
    }

    fn click() -> Vec<u8> {
        let mut b = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
        b.extend_from_slice(&970u32.to_le_bytes());
        b.extend_from_slice(&0i16.to_le_bytes());
        b.extend_from_slice(&0i16.to_le_bytes());
        b.extend_from_slice(&u32::MAX.to_le_bytes());
        b
    }

    fn pick(line: u32) -> Vec<u8> {
        let mut b = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
        b.extend_from_slice(&0u32.to_le_bytes());
        b.push(net::script::SCRIPT_TYPE_MENU);
        b.push(1);
        b.extend_from_slice(&line.to_le_bytes());
        b
    }

    fn map_of(s: &Session) -> u32 {
        s.claimed_character().unwrap().map_id
    }

    /// Each choice charges its own price and goes to its own sauna, with the fare said in the
    /// chat log; a short purse is refused and nobody moves or pays.
    #[test]
    fn the_receptionist_sells_each_sauna_at_its_price() {
        for (line, name, map, price) in ROOMS {
            let (store, mut s, id) = guest(1_000);
            let out = s.handle(&click());
            let said: String = out.iter().map(|r| String::from_utf8_lossy(&r.body).to_string()).collect();
            assert!(said.contains(&format!("#L{line}#")) && said.contains(name), "{said}");
            let out = s.handle(&pick(line));
            assert_eq!(store.mesos(id).unwrap(), 1_000 - price, "{name}");
            assert_eq!(map_of(&s), map, "{name}");
            assert!(out.iter().any(|r| r.opcode == net::message::MESSAGE && r.body == net::message::meso_lost_line(price)));
        }
        let (store, mut s, id) = guest(998);
        let _ = s.handle(&click());
        let out = s.handle(&pick(1));
        let said: String = out.iter().map(|r| String::from_utf8_lossy(&r.body).to_string()).collect();
        assert!(said.contains("don't have enough"), "{said}");
        assert_eq!((store.mesos(id).unwrap(), map_of(&s)), (998, HOTEL), "998 is short of 999: nothing taken, nobody moved");
        assert!(menu().is_ascii());
    }
}
