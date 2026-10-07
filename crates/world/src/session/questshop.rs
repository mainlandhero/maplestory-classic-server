//! **Shops a quest opens** - an NPC who asks "look at my wares?" only of a character who has
//! finished their quest.
//!
//! The owner, 2026-10-05: *"Can you make it so that talking to her gives a "Yes" or "No" prompt
//! for asking if the players wants to look at her wares. This shop should only be offered to
//! those who have completed the Arwen's shoes quest."* Then: *"Can we also do Jane's shop in
//! Lith Harbor please? She should have a quest that enables her shop too."*
//!
//! | NPC | quest that opens it | stock (`data/shops.txt`) |
//! |---|---|---|
//! | Arwen the Fairy (304), Ellinia | 10200 Arwen and the Glass Shoe | Moon Rock, Star Rock, Black Feather |
//! | Jane (103), Lith Harbor | 10002 Jane and the Mushroom | White Potion, Unagi, Pure Water, Watermelon |
//!
//! Classic World's Arwen and Jane sell these (meowdb npcs 304 and 103). This server had no shop
//! for either, so Moon Rock and Black Feather had no source at all and quests 10005 and 10114
//! could not be finished. The stock is in `data/shops.txt` like every other counter - but a
//! shopkeeper there opens the counter on click, and these two must not, so their click is
//! answered here first:
//!
//! * **Has finished the quest** (complete now, or - for Arwen's daily - completed on an earlier
//!   day and cleared by the daily reset; `store::Store::has_completed_quest`): a yes/no. Yes
//!   opens the counter; No ends the conversation.
//! * **Has not**: no shop and no prompt - the click falls through to the NPC's ordinary lines.
//!
//! **Which quest opens Jane's is the server's choice**, not a measurement: nothing public names
//! one. 10002 is her first quest, and her own `d1` line - *"Thanks for the materials. If you
//! need something, please let me know"* - is the one the client shows after it.
//!
//! The client sends a click on an NPC with a quest on offer as `0x0151`, not `0x00F2`
//! (`session/npc.rs` `on_quest_request`, Phil's note), so while one of the NPC's quests is on
//! offer the click opens that quest and this prompt is not reached. It is reached once nothing
//! is on offer from them.

use super::{Conversation, Reply, Session};

/// One shop a quest opens.
#[derive(Debug, Clone, Copy)]
pub(crate) struct QuestShop {
    pub npc_template: u32,
    pub npc_name: &'static str,
    pub quest: u32,
    pub ask: &'static str,
}

/// Arwen the Fairy, Ellinia.
pub(crate) const ARWEN_TEMPLATE: u32 = 304;
/// "Arwen and the Glass Shoe".
pub(crate) const GLASS_SHOE_QUEST: u32 = 10_200;
/// Jane, Lith Harbor.
pub(crate) const JANE_TEMPLATE: u32 = 103;
/// "Jane and the Mushroom".
pub(crate) const JANE_MUSHROOM_QUEST: u32 = 10_002;

/// Every shop a quest opens.
pub(crate) const QUEST_SHOPS: [QuestShop; 2] = [
    QuestShop {
        npc_template: ARWEN_TEMPLATE,
        npc_name: "Arwen",
        quest: GLASS_SHOE_QUEST,
        ask: "Oh, it's you - the one who brought back my glass shoe. Fairies don't usually trade with humans, but for you I'll make an exception.\r\nWould you like to look at my wares?",
    },
    QuestShop {
        npc_template: JANE_TEMPLATE,
        npc_name: "Jane",
        quest: JANE_MUSHROOM_QUEST,
        ask: "Ah, it's you! Thanks to the mushroom caps you brought me, I've been brewing again. I make my own medicine for the journey I'll take one day.\r\nWould you like to look at my wares?",
    },
];

/// The conversation path the yes/no is answered on; the conversation's NPC says whose.
pub(crate) const ASK_PATH: &str = "questshop.wares";

/// The quest-opened shop `template` keeps, if any.
pub(crate) fn quest_shop(template: u32) -> Option<&'static QuestShop> {
    QUEST_SHOPS.iter().find(|s| s.npc_template == template)
}

impl Session {
    /// A click on an NPC whose shop a quest opens: `Some` (the yes/no) for a character who has
    /// finished it, `None` for anyone else - and the caller must not open the counter on `None`.
    pub(super) fn quest_shop_click(&mut self, template: u32) -> Option<Vec<Reply>> {
        let shop = quest_shop(template)?;
        let chr = self.claimed_character()?;
        match self.store.has_completed_quest(chr.id, shop.quest) {
            Ok(true) => {}
            Ok(false) => return None,
            Err(e) => {
                crate::server::log(&format!(
                    "   quest shop: could not read quest {} for {} ({}): {e} - {}'s shop not offered",
                    shop.quest, chr.name, chr.id, shop.npc_name
                ));
                return None;
            }
        }
        self.conversation = Some(Conversation {
            npc_template: template,
            quest_id: None,
            path: ASK_PATH.to_string(),
            sent: 0,
            awaiting_yes_no: true,
            sent_with_next: false,
        });
        Some(vec![Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_ask(template, shop.ask, false),
            what: format!(
                "ScriptMessage YES/NO from NPC {template} ({}): look at the wares? - {} ({}) finished quest {}",
                shop.npc_name, chr.name, chr.id, shop.quest
            ),
        }])
    }

    /// The yes/no. Yes opens the counter; anything else ends the conversation.
    pub(super) fn quest_shop_answer(&mut self, template: u32, action: i8) -> Vec<Reply> {
        self.conversation = None;
        if action != net::script::SCRIPT_ACTION_YES {
            return Vec::new();
        }
        let Some(shop) = quest_shop(template) else { return Vec::new() };
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        // The same gate as the prompt: a Yes is not a way round it.
        if !self.store.has_completed_quest(chr.id, shop.quest).unwrap_or(false) {
            return Vec::new();
        }
        match self.open_shop_for(template, chr.id) {
            Some(replies) => replies,
            None => self.notice(format!("{} has nothing to sell right now - the shop did not load (data/shops.txt).", shop.npc_name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::fields::Fields;
    use std::sync::Arc;
    use store::Store;

    /// Object ids on the test map: Arwen, Jane.
    const ARWEN_OBJECT: u32 = 1000;
    const JANE_OBJECT: u32 = 1001;

    fn session() -> (Session, Arc<Store>, u32) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Pebble".into(), level: 30, ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let item = |item_id, buy_price| crate::shops::ShopItem {
            item_id,
            name: format!("item {item_id}"),
            buy_price,
            sell_price: buy_price,
            min_grade: None,
            quest_item: false,
            trade_blocked: false,
        };
        let shop = |npc: &str, items| crate::shops::Shop {
            npc: npc.into(),
            role: "Material Merchant".into(),
            map_label: String::new(),
            items,
        };
        let arwen = shop("Arwen the Fairy", vec![item(4_010_107, 2_000), item(4_020_109, 4_000), item(4_031_033, 1_000)]);
        let jane = shop("Jane", vec![item(2_000_002, 350), item(2_022_003, 1_000), item(2_022_000, 1_120), item(2_010_006, 1_920)]);
        let mut item_data = std::collections::HashMap::new();
        for id in [4_010_107u32, 4_020_109, 4_031_033, 2_000_002, 2_022_003, 2_022_000, 2_010_006] {
            item_data.insert(id, crate::shops::ItemData { price: 100, slot_max: 100, ..Default::default() });
        }
        let npc = |object_id, template_id| net::opcode::FieldNpc { object_id, template_id, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 };
        let mut cfg = Config::default();
        cfg.shops = crate::shops::ShopTable { shops: vec![arwen, jane], item_data, problems: Vec::new() };
        cfg.shop_by_template.insert(ARWEN_TEMPLATE, 0);
        cfg.shop_by_template.insert(JANE_TEMPLATE, 1);
        cfg.npcs.insert(net::opcode::START_MAP_ID, vec![npc(ARWEN_OBJECT, ARWEN_TEMPLATE), npc(JANE_OBJECT, JANE_TEMPLATE)]);
        cfg.fields.insert(net::opcode::START_MAP_ID);
        let mut s = Session::joining(store.clone(), Arc::new(cfg), Arc::new(Fields::new()));
        s.claim_for_character(id);
        (s, store, id)
    }

    fn click(s: &mut Session, object: u32) -> Vec<Reply> {
        let mut body = object.to_le_bytes().to_vec();
        body.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
        s.on_npc_click(&body)
    }

    fn answer(s: &mut Session, action: i8) -> Vec<Reply> {
        let mut body = 0u32.to_le_bytes().to_vec();
        body.push(net::script::SCRIPT_TYPE_YES_NO);
        body.push(action as u8);
        s.on_script_reply(&body)
    }

    fn is_shop(out: &[Reply]) -> bool {
        out.iter().any(|r| r.opcode == net::classicshop::CLASSIC_OPEN_SHOP)
    }

    fn is_ask(out: &[Reply], template: u32) -> bool {
        let ask = quest_shop(template).unwrap().ask;
        out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE && r.body == net::script::npc_ask(template, ask, false))
    }

    fn finish(store: &Store, id: u32, quest: u32) {
        assert!(store.start_quest(id, quest).unwrap());
        store.complete_quest(id, quest).unwrap();
    }

    fn stock(s: &Session) -> Vec<u32> {
        s.open_shop.clone().expect("a counter is open").1.iter().map(|r| r.item_id).collect()
    }

    /// **Not finished: no counter and no question** - neither NPC opens on click the way a
    /// shopkeeper does, an accepted quest is not a finished one, and a stray Yes opens nothing.
    #[test]
    fn a_quest_shop_stays_shut_to_a_stranger() {
        let (mut s, store, id) = session();
        for (object, template) in [(ARWEN_OBJECT, ARWEN_TEMPLATE), (JANE_OBJECT, JANE_TEMPLATE)] {
            let out = click(&mut s, object);
            assert!(!is_shop(&out) && !is_ask(&out, template), "{template}: {out:?}");
        }
        assert!(store.start_quest(id, GLASS_SHOE_QUEST).unwrap());
        let out = click(&mut s, ARWEN_OBJECT);
        assert!(!is_shop(&out) && !is_ask(&out, ARWEN_TEMPLATE), "accepted is not finished: {out:?}");
        s.conversation = Some(Conversation {
            npc_template: JANE_TEMPLATE,
            quest_id: None,
            path: ASK_PATH.to_string(),
            sent: 0,
            awaiting_yes_no: true,
            sent_with_next: false,
        });
        assert!(!is_shop(&answer(&mut s, net::script::SCRIPT_ACTION_YES)), "a Yes is not a way round it");
    }

    /// **Arwen: asked first, Yes opens her counter, No does not** - and that still holds the day
    /// after, when the daily reset has cleared the completion. Her quest opens only hers.
    #[test]
    fn arwen_asks_whoever_returned_the_shoe_and_yes_opens_her_counter() {
        let (mut s, store, id) = session();
        finish(&store, id, GLASS_SHOE_QUEST);

        let out = click(&mut s, ARWEN_OBJECT);
        assert!(is_ask(&out, ARWEN_TEMPLATE) && !is_shop(&out), "asked, not opened: {out:?}");
        assert!(answer(&mut s, net::script::SCRIPT_ACTION_NO).is_empty(), "No: nothing");
        assert!(s.conversation.is_none());

        assert!(is_ask(&click(&mut s, ARWEN_OBJECT), ARWEN_TEMPLATE));
        assert!(is_shop(&answer(&mut s, net::script::SCRIPT_ACTION_YES)));
        assert_eq!(stock(&s), vec![4_010_107, 4_020_109, 4_031_033]);

        let out = click(&mut s, JANE_OBJECT);
        assert!(!is_ask(&out, JANE_TEMPLATE) && !is_shop(&out), "Arwen's quest does not open Jane's");

        let tomorrow = store::dailyperks::utc_day_start(store::dailyperks::today() + 1);
        assert!(store.clear_daily_completion(id, GLASS_SHOE_QUEST, tomorrow).unwrap());
        assert!(is_ask(&click(&mut s, ARWEN_OBJECT), ARWEN_TEMPLATE), "the turn-in is remembered across the reset");
    }

    /// **Jane: "Jane and the Mushroom" opens her counter**, with her own question and her own stock.
    #[test]
    fn jane_asks_whoever_brought_the_mushroom_caps_and_yes_opens_her_counter() {
        let (mut s, store, id) = session();
        finish(&store, id, JANE_MUSHROOM_QUEST);
        let out = click(&mut s, JANE_OBJECT);
        assert!(is_ask(&out, JANE_TEMPLATE) && !is_shop(&out), "{out:?}");
        assert!(is_shop(&answer(&mut s, net::script::SCRIPT_ACTION_YES)));
        assert_eq!(stock(&s), vec![2_000_002, 2_022_003, 2_022_000, 2_010_006]);
    }
}
