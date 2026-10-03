//! The world session's tests, moved wholesale out of `session.rs`.
//!
//! They stay in one file on purpose. Splitting them per topic would mean either
//! duplicating the fixtures or inventing a shared test-support module, and the
//! fixtures are the part most likely to drift.

use super::*;

fn session() -> (Session, Arc<Store>, i64, u32) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Wanderer".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    let s = Session::new(store.clone(), Arc::new(Config::default()));
    (s, store, account_id, id)
}

/// A session with the migration already claimed, which is what every quest test needs.
fn claimed_session() -> (Session, Arc<Store>, u32) {
    let (mut s, store, account_id, id) = session();
    store.create_migration(account_id, id, 0, 0).unwrap();
    let note = s.claim_for_character(id);
    assert!(note.contains("claimed the migration"), "{note}");
    (s, store, id)
}

/// The `0x00F3` body: u32 handle, u8 messageType, u32 echo, a u16-prefixed string, u8
/// action. Built here rather than hand-hexed so a change to the parser breaks this too.
fn script_reply(action: i8) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&0u32.to_le_bytes()); // handle
    b.push(0); //                                messageType
    b.extend_from_slice(&0u32.to_le_bytes()); // echo
    b.extend_from_slice(&0u16.to_le_bytes()); // empty text
    b.push(action as u8);
    b
}

/// Pressing Yes writes the quest down. **This is the half that was missing**: the
/// dialogue already answered the `yes` branch, so the conversation looked finished on
/// screen while nothing had ever called `start_quest`.
#[test]
fn saying_yes_to_a_quest_stores_it_and_sends_the_quest_record() {
    let (mut s, store, id) = claimed_session();
    let convo = Conversation {
        npc_template: 2100,
        quest_id: Some(1000),
        path: "0".to_string(),
        sent: 0,
        awaiting_yes_no: true,
        sent_with_next: false,
    };

    let out = s.accept_quest(&convo);
    assert_eq!(out.len(), 1, "one quest record");
    assert_eq!(out[0].opcode, net::quest::MESSAGE);
    assert_eq!(out[0].body, net::quest::quest_accepted(1000));
    assert!(out[0].what.contains("and stored"), "{}", out[0].what);

    let rows = store.quest_rows(id).unwrap();
    assert_eq!(rows.len(), 1, "the quest is in quest_state");
    assert_eq!(rows[0].quest_id, 1000);
    assert_eq!(rows[0].state, store::QuestState::InProgress);
}

/// The ordering decision, pinned. `Config::default()` has no quest text at all, so
/// `has_branch` is false and the conversation ends immediately - and the acceptance must
/// still have been recorded, because a quest whose `yes` path has no line is still a
/// quest the player accepted.
#[test]
fn a_yes_with_no_branch_text_still_records_the_acceptance() {
    let (mut s, store, id) = claimed_session();
    s.conversation = Some(Conversation {
        npc_template: 2100,
        quest_id: Some(1000),
        path: "0".to_string(),
        sent: 0,
        awaiting_yes_no: true,
        sent_with_next: false,
    });
    assert!(s.config.quests.is_empty(), "this test is about the no-branch-text case");

    let out = s.on_script_reply(&script_reply(net::script::SCRIPT_ACTION_YES));
    assert_eq!(out.len(), 1, "the quest record, and no dialogue line");
    assert_eq!(out[0].opcode, net::quest::MESSAGE);
    assert_eq!(store.quest_rows(id).unwrap().len(), 1, "recorded despite the empty branch");
    assert!(s.conversation.is_none(), "the conversation is over");
}

/// No means no: nothing is written and nothing is sent.
#[test]
fn saying_no_to_a_quest_records_nothing() {
    let (mut s, store, id) = claimed_session();
    s.conversation = Some(Conversation {
        npc_template: 2100,
        quest_id: Some(1000),
        path: "0".to_string(),
        sent: 0,
        awaiting_yes_no: true,
        sent_with_next: false,
    });

    assert!(s.on_script_reply(&script_reply(net::script::SCRIPT_ACTION_NO)).is_empty());
    assert!(store.quest_rows(id).unwrap().is_empty());
}

/// The other half: an accepted quest has to come back on the next field entry, or the
/// journal is empty every time the player walks through a portal. This asserts the
/// **bytes** in the record, not that a function was called.
#[test]
fn the_field_entry_setfield_carries_the_quest_journal() {
    let (mut s, store, id) = claimed_session();
    store.start_quest(id, 1000).unwrap();

    let mut chr = s.claimed_character().expect("the claim resolves to a character");
    let replies = s.go_to_map(&mut chr, 40, 0, "a test portal walk".to_string());
    let sf = replies
        .iter()
        .find(|r| r.opcode == net::opcode::SET_FIELD)
        .expect("a field entry always sends a SetField");

    let expected = net::quest::started_quest_block(&[net::quest::StartedQuest {
        quest_id: 1000,
        progress: String::new(),
    }]);
    assert!(
        sf.body.windows(expected.len()).any(|w| w == expected.as_slice()),
        "the started-quest block is not in the record"
    );
    assert!(sf.what.contains("1 started / 0 completed"), "{}", sf.what);
}

/// Every slot of every tab emptied, so a test can fill exactly what it means to.
fn empty_bag(store: &Store, id: u32) {
    for tab in store::InventoryType::ALL {
        for row in store.bag_items(id, tab).unwrap() {
            store.remove_item(id, tab, row.slot, None).unwrap();
        }
    }
}

/// A one-reward quest whose turn-in has a line of its own, so the closing line is what
/// would be said if the refusal did not stop it.
fn hat_quest(end_npc: u32) -> crate::config::Quest {
    let mut say = std::collections::HashMap::new();
    say.insert("1".to_string(), vec!["Here is your hat.".to_string()]);
    crate::config::Quest {
        name: "Lucas' Reply".to_string(),
        start_npc: Some(2001),
        end_npc: Some(end_npc),
        complete_rewards: vec![
            crate::config::RewardItem { id: 1_002_005, count: 1, prop: 0, gender: None },
            crate::config::RewardItem { id: 4_031_002, count: -1, prop: 0, gender: None },
        ],
        complete_items: vec![(1_002_005, 1), (4_031_002, -1)],
        complete_exp: 10,
        say,
        ..Default::default()
    }
}

/// **Mint's report, 2026-09-17: "quest continues to complete despite this happening."**
/// Quest 1008 into a full Equip tab wrote the completion, paid the EXP, took the letter and
/// said "could not give you item 1002005" in yellow. Now nothing moves: the NPC's own box
/// says which tab and how many, the row stays in progress, the letter stays, the EXP is
/// unpaid, and the closing line is NOT spoken over it. Free a slot and the same click
/// completes it.
#[test]
fn a_turn_in_into_a_full_tab_is_refused_at_the_npc_and_the_quest_stays_in_progress() {
    let (mut s, store, id) = claimed_session();
    let mut quests = std::collections::HashMap::new();
    quests.insert(1008u32, hat_quest(2000));
    s.config = Arc::new(Config { quests, ..(*s.config).clone() });
    let exp_now = |store: &Arc<Store>| store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().exp;
    empty_bag(&store, id);
    for i in 0..30u32 {
        store.add_item(id, store::InventoryType::Equip, &store::Item::equip(1_302_000 + i % 3), 1).unwrap();
    }
    store.add_item(id, store::InventoryType::Etc, &store::Item::bundle(4_031_002, 1), 1).unwrap();
    store.start_quest(id, 1008).unwrap();
    let exp_before = exp_now(&store);

    let out = s.handle(&quest_request(2, 1008, 2000));
    assert!(!out.iter().any(|r| r.opcode == net::quest::MESSAGE), "no completion record: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert_eq!(store.quest_row(id, 1008).unwrap().unwrap().state, store::QuestState::InProgress, "still in progress");
    assert_eq!(exp_now(&store), exp_before, "no EXP paid");
    let bag = store.bag(id).unwrap();
    assert_eq!(bag.items_in(store::InventoryType::Etc).filter(|i| i.item.item_id == 4_031_002).count(), 1, "the letter was not taken");
    assert_eq!(bag.items_in(store::InventoryType::Equip).count(), 30, "nothing was placed");
    let boxes: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).collect();
    assert_eq!(boxes.len(), 1, "exactly one box, the refusal - not the quest's closing line: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    let text = String::from_utf8_lossy(&boxes[0].body).into_owned();
    assert!(text.contains("Your bag is full"), "{text}");
    assert!(text.contains("Please make 1 space in your Equip tab."), "{text}");
    assert!(boxes[0].what.contains("NPC template 2000"), "{}", boxes[0].what);
    assert!(!out.iter().any(|r| r.what.contains("Here is your hat")), "the closing line is not said over a refusal");
    assert_eq!(s.conversation.as_ref().map(|c| c.path.as_str()), Some(crate::questroom::REFUSAL_PATH), "parked under its own path");

    // OK on the box: silence, conversation cleared, still nothing moved.
    let closed = s.on_script_reply(&script_reply(net::script::SCRIPT_ACTION_YES));
    assert!(closed.is_empty(), "{:?}", closed.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(s.conversation.is_none());
    assert_eq!(store.quest_row(id, 1008).unwrap().unwrap().state, store::QuestState::InProgress);

    // One slot freed: the same click completes it, hat in, letter out, EXP paid, line said.
    store.remove_item(id, store::InventoryType::Equip, 30, None).unwrap();
    let out = s.handle(&quest_request(2, 1008, 2000));
    assert!(out.iter().any(|r| r.opcode == net::quest::MESSAGE), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert_eq!(store.quest_row(id, 1008).unwrap().unwrap().state, store::QuestState::Complete);
    let bag = store.bag(id).unwrap();
    assert_eq!(bag.items_in(store::InventoryType::Equip).filter(|i| i.item.item_id == 1_002_005).count(), 1, "the hat");
    assert_eq!(bag.items_in(store::InventoryType::Etc).filter(|i| i.item.item_id == 4_031_002).count(), 0, "the letter went back");
    assert_eq!(exp_now(&store) - exp_before, 10);
    // The closing line itself is the client's (it drew `Say.1` before sending the turn-in), so
    // what matters is that nothing contradicts it: no refusal box this time.
    assert!(!out.iter().any(|r| r.what.contains("bag is full") || r.what.contains("Your bag")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(!out.iter().any(|r| r.what.contains("on path \"1\"")), "and Say.1 is not repeated: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
}

/// **A stack that still has room is room.** The hat quest's twin with a potion reward: a
/// full Use tab whose last stack is 96 of 100 takes a gift of 4 and refuses a gift of 5 -
/// counted the way the store places it, not by free rows.
#[test]
fn a_turn_in_that_tops_up_a_stack_is_not_refused_for_a_full_tab() {
    let (mut s, store, id) = claimed_session();
    let potion_quest = |s: &mut Session, count: i32| {
        let mut q = hat_quest(2000);
        q.complete_rewards = vec![crate::config::RewardItem { id: 2_000_000, count, prop: 0, gender: None }];
        q.complete_items = vec![(2_000_000, count)];
        let mut quests = std::collections::HashMap::new();
        quests.insert(1009u32, q);
        s.config = Arc::new(Config { quests, ..(*s.config).clone() });
    };
    empty_bag(&store, id);
    for _ in 0..29 {
        store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_000_001, 100), 100).unwrap();
    }
    store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_000_000, 96), 100).unwrap();
    let max = s.config.shops.max_stack(2_000_000);
    assert!(max >= 100, "the test config's Red Potion stacks to {max}");

    potion_quest(&mut s, 5);
    store.start_quest(id, 1009).unwrap();
    let out = s.record_quest_complete(1009, 1009);
    assert!(!out.iter().any(|r| r.opcode == net::quest::MESSAGE), "96 + 5 needs a 31st slot: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(out.iter().any(|r| r.what.contains("Use short 1")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());

    potion_quest(&mut s, 4);
    s.conversation = None;
    let out = s.record_quest_complete(1009, 1009);
    assert!(out.iter().any(|r| r.opcode == net::quest::MESSAGE), "96 + 4 tops the stack up: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    let stack = store.bag(id).unwrap().items_in(store::InventoryType::Use).find(|i| i.item.item_id == 2_000_000).unwrap().item.kind.quantity();
    assert_eq!(stack, 100);
}

/// **Accepting has the same rule.** Sera hands over their mirror on accept (quest 1001, Etc);
/// with the Etc tab full the quest is not started and Sera says so.
#[test]
fn accepting_a_quest_whose_start_item_has_no_room_is_refused_and_not_started() {
    let (mut s, store, id) = claimed_session();
    let mut quests = std::collections::HashMap::new();
    quests.insert(
        1001u32,
        crate::config::Quest {
            name: "Sera".to_string(),
            start_npc: Some(2),
            start_items: vec![(4_031_000, 1)],
            start_rewards: vec![crate::config::RewardItem { id: 4_031_000, count: 1, prop: 0, gender: None }],
            ..Default::default()
        },
    );
    s.config = Arc::new(Config { quests, ..(*s.config).clone() });
    empty_bag(&store, id);
    for i in 0..30u32 {
        store.add_item(id, store::InventoryType::Etc, &store::Item::bundle(4_000_000 + i, 1), 1).unwrap();
    }
    let out = s.record_quest_start(1001, 2);
    assert!(store.quest_row(id, 1001).unwrap().is_none(), "not started");
    assert!(!out.iter().any(|r| r.opcode == net::quest::MESSAGE));
    let say = out.iter().find(|r| r.opcode == net::script::SCRIPT_MESSAGE).expect("Sera's box");
    let text = String::from_utf8_lossy(&say.body).into_owned();
    assert!(text.contains("Please make 1 space in your Etc tab."), "{text}");
    assert!(say.what.contains("NPC template 2"), "{}", say.what);
    assert_eq!(store.bag(id).unwrap().items_in(store::InventoryType::Etc).count(), 30, "nothing given");
}

/// A character with no quests still sends both blocks. They are three bytes each and the
/// record has no length prefix, so "send nothing when there is nothing" desynchronises
/// everything after it.
#[test]
fn an_empty_journal_still_costs_two_blocks() {
    let (s, _store, _id) = claimed_session();
    let (book, note) = s.quest_book(s.claimed().unwrap().character_id);
    assert!(book.started.is_empty());
    assert!(book.completed.is_empty());
    assert!(note.contains("0 started / 0 completed"), "{note}");
    assert_eq!(book.started_block().len(), net::quest::EMPTY_QUEST_BLOCK_LEN);
    assert_eq!(book.completed_block().len(), net::quest::EMPTY_QUEST_BLOCK_LEN);
}

/// A session whose character is wearing four items, which is what a real one wears.
fn dressed_session() -> (Session, Arc<Store>, u32) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character {
        name: "Wanderer".to_string(),
        equips: vec![(5, 1040002), (6, 1060002), (7, 1072001), (11, 1302000)],
        ..Default::default()
    };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store.clone(), Arc::new(Config::default()));
    assert!(s.claim_for_character(id).contains("claimed the migration"));
    (s, store, id)
}

/// The `0x0107` body: u32 tick, i8 invType, i16 src, i16 dst, i16 count.
fn inventory_move(inv_type: i8, src: i16, dst: i16, count: i16) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&0u32.to_le_bytes());
    b.push(inv_type as u8);
    b.extend_from_slice(&src.to_le_bytes());
    b.extend_from_slice(&dst.to_le_bytes());
    b.extend_from_slice(&count.to_le_bytes());
    b
}

/// **Goal I, end to end.** The owner: *"items taken off should persist as is during
/// transitions from map to map."*
///
/// Take the hat off, then walk a portal, and read the record that comes back: the hat
/// must be gone from the equipped list and present in the Equip tab. Before the store
/// had somewhere to put it this test could not be written - the unequip touched no row,
/// so the next `SetField` re-dressed the character and the item came back on.
#[test]
fn an_unequipped_item_is_still_off_after_a_map_change() {
    let (mut s, store, id) = dressed_session();

    let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, -5, 1, -1));
    assert_eq!(out.len(), 1, "exactly one reply, and it is never zero");
    assert_eq!(out[0].opcode, net::inventory::INVENTORY_OPERATION);
    assert_eq!(out[0].body[0], 1, "bExclRequestSent - without it the UI locks up");
    assert!(out[0].what.contains("STORED"), "{}", out[0].what);

    // The database, not the reply.
    let worn: Vec<u8> = store.equipped_items(id).unwrap().iter().map(|e| e.slot).collect();
    assert_eq!(worn, vec![6, 7, 11], "slot 5 is off");
    let bagged: Vec<(u16, u32)> = store
        .bag(id)
        .unwrap()
        .items_in(store::InventoryType::Equip)
        .map(|i| (i.slot, i.item.item_id))
        .collect();
    assert_eq!(bagged, vec![(1, 1040002)], "and it is in the Equip tab, slot 1");

    // The wire, on the next field entry.
    let mut chr = s.claimed_character().expect("the claim resolves");
    assert!(!chr.equips.iter().any(|&(slot, _)| slot == 5), "not worn any more");
    assert_eq!(chr.equip_bag.len(), 1, "and the record will carry it");

    let replies = s.go_to_map(&mut chr, 40, 0, "a test portal walk".to_string());
    let sf = replies
        .iter()
        .find(|r| r.opcode == net::opcode::SET_FIELD)
        .expect("a field entry always sends a SetField");

    let expected = net::bag::equipped_tail(
        &[net::bag::BagEquip::plain(1, 1040002)],
        net::opcode::DEFAULT_INVENTORY_SLOTS,
    );
    assert!(
        sf.body.windows(expected.len()).any(|w| w == expected.as_slice()),
        "the Equip tab's list is not in the record"
    );
    // And it costs what one bag equip costs, rather than merely appearing somewhere. A
    // 125-byte item body is mostly zeros, so "the ten empty bytes are absent" is NOT a
    // usable check - the pattern occurs inside any item. The length delta is exact.
    let mut bare = chr.clone();
    bare.equip_bag.clear();
    let without = s.go_to_map(&mut bare, 40, 0, "the same walk, empty bag".to_string());
    let without = &without
        .iter()
        .find(|r| r.opcode == net::opcode::SET_FIELD)
        .expect("a SetField")
        .body;
    assert_eq!(
        sf.body.len() - without.len(),
        2 + net::opcode::EQUIPPED_ITEM_LEN,
        "one u16 position plus one type-1 item body"
    );
}

/// Putting it back on is the same transaction in reverse, and it also persists.
#[test]
fn equipping_from_the_bag_moves_the_row_back() {
    let (mut s, store, id) = dressed_session();
    s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, -5, 1, -1));

    let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, 1, -5, -1));
    assert_eq!(out[0].body[0], 1, "still answered");
    assert!(out[0].what.contains("equipped item 1040002"), "{}", out[0].what);

    let worn: Vec<u8> = store.equipped_items(id).unwrap().iter().map(|e| e.slot).collect();
    assert_eq!(worn, vec![5, 6, 7, 11], "back on");
    assert_eq!(store.bag(id).unwrap().items.len(), 0, "and out of the bag");
}

/// A move the store refuses is still answered, and with the byte that unlocks the UI.
///
/// **This is the failure that cost a whole session on 2026-08-19**: a refusal sent as a
/// chat notice left `player+0x2330` latched, and every later inventory action was dropped
/// by the client before it was built.
#[test]
fn a_refused_move_still_clears_the_request_latch() {
    let (mut s, store, id) = dressed_session();

    // Slot 9 is empty, so this cannot succeed.
    let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, -9, 1, -1));
    assert_eq!(out.len(), 1, "a refusal is a packet");
    assert_eq!(out[0].opcode, net::inventory::INVENTORY_OPERATION);
    assert_eq!(out[0].body, net::inventory::inventory_rejected());
    assert_eq!(out[0].body[0], 1, "bExclRequestSent, even on a refusal");
    assert!(out[0].what.contains("REFUSING"), "{}", out[0].what);

    assert_eq!(store.equipped_items(id).unwrap().len(), 4, "nothing moved");
    assert!(store.bag(id).unwrap().is_empty());
}

/// A connection with no claimed migration is answered too, rather than dropped.
#[test]
fn an_unclaimed_connection_is_refused_rather_than_ignored() {
    let (mut s, _, _, _) = session();
    let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, -5, 1, -1));
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].body, net::inventory::inventory_rejected());
}

/// A claimed session whose config knows two item names, which `!item` needs to accept an
/// id at all.
fn gm_session() -> (Session, Arc<Store>, u32) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut item_names = std::collections::HashMap::new();
    item_names.insert(1302000u32, "Sword".to_string());
    item_names.insert(2000000u32, "Red Potion".to_string());
    // `Config::answer_packets` is the master switch: with it clear, `Session::handle`
    // returns nothing for EVERY packet. It was OFF in `Config::default()` until 2026-09-14,
    // which is the trap that cost a client launch on 2026-08-20 - a bare launcher line
    // without `-SetFieldProbe` left the character on the select screen. It is on now, so
    // no test has to remember it, and the sixty `set_field_probe: true` lines are gone.
    let config = Config { item_names, ..Config::default() };
    let mut s = Session::new(store.clone(), Arc::new(config));
    assert!(s.claim_for_character(id).contains("claimed the migration"));
    (s, store, id)
}

/// **The channel answers on a bare `Config::default()`, and only `answer_packets: false`
/// silences it.** This is the guard on the 2026-09-14 default flip.
///
/// It is deliberately not a test of the CLI. The trap was never in the argument parser - it
/// was that the off state was reachable by *forgetting* something, and a test that pins the
/// parser would still pass if someone set the field back to `false` in `Config::default()`.
/// So this asserts the behaviour: a default session replies, and the one caller who asks for
/// silence by name gets it.
///
/// Sixty-one call sites passed `set_field_probe: true` and not one wanted the default. The
/// only thing it ever produced was a launch spent on a client stuck at "Connecting..."
/// (2026-08-20), read as a server bug for the whole run.
#[test]
fn default_config_answers_packets_and_silent_channel_does_not() {
    let (mut s, _store, _id) = gm_session();
    let spoken = s.handle(&gm_chat("!help"));
    assert!(
        !spoken.is_empty(),
        "a session built from Config::default() must answer - answer_packets is the default \
         since 2026-09-14, and a channel that answers nothing looks exactly like a server \
         that is not running"
    );

    let (mut quiet, _store, _id) = gm_session();
    quiet.config = Arc::new(Config { answer_packets: false, ..(*quiet.config).clone() });
    assert!(
        quiet.handle(&gm_chat("!help")).is_empty(),
        "--silent-channel still has to silence the channel; it is the control that \
         eliminates the channel as a variable"
    );
}

/// The `0x00E7` body the client sends when a line is typed: u32 tick, the text, u8 tab.
fn gm_chat(text: &str) -> Vec<u8> {
    let mut b = net::opcode::CLIENT_CHAT.to_le_bytes().to_vec();
    b.extend_from_slice(&[0u8; 4]);
    b.extend_from_slice(&(text.len() as u16).to_le_bytes());
    b.extend_from_slice(text.as_bytes());
    b.push(3);
    b
}

/// The text of a `0x00BB` chat notice.
fn notice_text(r: &Reply) -> String {
    assert_eq!(r.opcode, net::notice::CHAT_NOTICE);
    let len = u16::from_le_bytes([r.body[1], r.body[2]]) as usize;
    String::from_utf8(r.body[3..3 + len].to_vec()).unwrap()
}

/// `!item` puts an equip in the Equip tab, says so, and tells the client to draw it.
#[test]
fn the_item_command_adds_an_equip_and_announces_it() {
    let (mut s, store, id) = gm_session();
    let out = s.handle(&gm_chat("!item 1302000"));

    let ack = notice_text(&out[0]);
    assert!(ack.starts_with("Giving TestCharD 1x Sword (1302000)"), "{ack}");
    assert!(ack.contains("Equip tab"), "{ack}");

    // The database, not the reply.
    let bagged: Vec<(u16, u32)> = store
        .bag(id)
        .unwrap()
        .items_in(store::InventoryType::Equip)
        .map(|i| (i.slot, i.item.item_id))
        .collect();
    assert_eq!(bagged, vec![(1, 1302000)], "slot 1 of the Equip tab");

    // And the wire: one mode-0 Add carrying a full type-1 item body.
    let add = out.iter().find(|r| r.opcode == net::inventory::INVENTORY_OPERATION).unwrap();
    assert_eq!(add.body[7], net::inventory::MODE_ADD);
    assert_eq!(add.body[8] as i8, net::inventory::INV_EQUIP);
    assert_eq!(i16::from_le_bytes([add.body[9], add.body[10]]), 1);
    assert_eq!(
        add.body[net::inventory::INVENTORY_ADD_HEAD_LEN],
        net::opcode::EQUIPPED_ITEM_TYPE,
        "the blob's own type byte"
    );
}

#[test]
fn the_exp_command_awards_and_persists_experience() {
    let (mut s, store, id) = gm_session();
    let before = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
    assert_eq!(before.exp, 0, "a fresh character has earned nothing");

    let out = s.handle(&gm_chat("!exp 250"));
    let ack = notice_text(&out[0]);
    assert!(ack.starts_with("TestCharD gains 250 experience: 0 -> 250"), "{ack}");

    // And the live update, so the bar moves without a map change. Bit 16 carries the new
    // TOTAL - a packet that sent the award instead would make the client draw the gain
    // twice as large on the second award and nothing would say so.
    let stat = out.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).expect("a 0x007C");
    assert_eq!(stat.body, net::stats::StatChange::exp(250).build());
    assert_ne!(stat.body, net::stats::StatChange::exp(0).build(), "and it is not the old total");

    let after = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
    assert_eq!(after.exp, 250, "the database, not the reply");

    // It adds rather than sets, so a second award moves it again.
    s.handle(&gm_chat("!exp 50"));
    let twice = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
    assert_eq!(twice.exp, 300);
}

/// A bad argument is refused with a reply, never with silence.
///
/// **Always answer.** An unanswered packet freezes the client's whole UI - every button,
/// including the quit prompt - and reads on screen as a crash.
#[test]
fn the_exp_command_refuses_nonsense_but_still_answers() {
    let (mut s, store, id) = gm_session();
    let out = s.handle(&gm_chat("!exp lots"));
    assert!(!out.is_empty(), "a refusal is still a reply");
    assert!(notice_text(&out[0]).contains("is not an amount"), "{}", notice_text(&out[0]));

    let unchanged = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
    assert_eq!(unchanged.exp, 0, "and nothing was written");
}

/// A `2xxxxxx` id lands in the Use tab, because the tab comes from the id.
#[test]
fn the_item_command_picks_the_tab_from_the_id() {
    let (mut s, store, id) = gm_session();
    let out = s.handle(&gm_chat("!item 2000000 3"));
    assert!(notice_text(&out[0]).contains("3x Red Potion"), "{}", notice_text(&out[0]));

    let rows: Vec<_> =
        store.bag(id).unwrap().items_in(store::InventoryType::Use).cloned().collect();
    // **One stack of three.** This test used to assert three slots, on the reasoning that an
    // unknown `info/slotMax` should be treated as 1 because that is "the safe direction".
    // It is not safe, it is just wrong: `slotMax` is *absent* on 2495 of this client's 2785
    // items, and while 1760 of those are equips - which really do not stack - 187 Etc, 285
    // Use and 263 Cash items are not. The owner, 2026-08-20: *"The items that I get such as Garnet
    // Ore should stack in my Etc inventory."*
    //
    // `ShopTable::max_stack` is the one place that decides, and it already said `100` for a
    // non-equip. Three callers had each written `slot_max.max(1)` instead of calling it.
    assert_eq!(rows.len(), 1, "an absent slotMax stacks: it means unspecified, not one");
    assert_eq!(rows[0].item.kind.quantity(), 3);
    assert!(store.bag(id).unwrap().items_in(store::InventoryType::Equip).next().is_none());
}

/// An id this client cannot draw is refused, and nothing is written.
///
/// The client has to render whatever arrives; an item body for an id with no `Item.wz`
/// entry is the shape of thing that has faulted it before.
#[test]
fn the_item_command_refuses_an_id_the_client_does_not_have() {
    let (mut s, store, id) = gm_session();
    let out = s.handle(&gm_chat("!item 9999999"));
    assert_eq!(out.len(), 1, "a refusal and nothing else");
    assert!(notice_text(&out[0]).contains("REFUSED"), "{}", notice_text(&out[0]));
    assert!(store.bag(id).unwrap().is_empty());
}

/// Every GM command acknowledges itself, including the ones that are not commands.
#[test]
fn an_unknown_command_lists_the_real_ones() {
    let (mut s, _, _) = gm_session();
    let out = s.handle(&gm_chat("!banana"));
    let text = notice_text(&out[0]);
    assert!(text.contains("!banana is not a command"), "{text}");
    assert!(text.contains("!item"), "the list must name the commands: {text}");

    assert!(notice_text(&s.handle(&gm_chat("!help"))[0]).contains("!map"));
}

/// **A drag out of the inventory window is a drop**, and a drop with no known position is
/// refused rather than guessed.
///
/// Measured 2026-08-20: `dst == 0`, which is not a move to slot zero - slots are 1-based
/// and 0 is the hole that makes them so.
///
/// Dropping IS built now. What this pins is the fallback: the client's pick-up sweep is a
/// box around the *player*, so an item put down at a guessed position is drawn and cannot be
/// reached - and on screen that is the same picture as nothing happening. Refusing keeps the
/// item, and the reply still clears the `+0x2330` latch.
#[test]
fn a_drop_with_no_known_position_is_answered_and_the_item_stays_in_the_bag() {
    let (mut s, store, id) = gm_session();
    s.handle(&gm_chat("!item 1302000"));

    let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, 1, 0, 1));
    let op = &out[0];
    assert_eq!(op.opcode, net::inventory::INVENTORY_OPERATION);
    assert_eq!(op.body, net::inventory::inventory_rejected());
    assert_eq!(op.body[0], 1, "bExclRequestSent - the UI must not latch");
    assert!(op.what.contains("DROP"), "{}", op.what);
    assert!(out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE), "and it says so");

    let bagged: Vec<u32> = store
        .bag(id)
        .unwrap()
        .items_in(store::InventoryType::Equip)
        .map(|i| i.item.item_id)
        .collect();
    assert_eq!(bagged, vec![1302000], "losing an item is worse than one that will not leave");
}

/// Once the server knows where the character stands, a drop leaves the bag and lands on the
/// floor - and the two packets go out in the order the client needs.
///
/// The `0x0070` Remove **first**: it is the inventory reply that clears `player+0x2330`, and
/// an out-of-order pair would leave the latch set for the rest of the session.
#[test]
fn a_drop_leaves_the_bag_and_lands_on_the_floor() {
    let (mut s, store, id) = gm_session();
    s.handle(&gm_chat("!item 1302000"));
    // The position comes from an attack, which is the only coordinate pair this server
    // reads from the client today.
    s.last_position = Some((520, 395));

    let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, 1, 0, 1));

    assert_eq!(out[0].opcode, net::inventory::INVENTORY_OPERATION, "the Remove goes first");
    assert_eq!(out[0].body[7], net::inventory::MODE_REMOVE);
    assert_eq!(out[1].opcode, net::drops::DROP_ENTER_FIELD, "then the item on the ground");

    let bagged: Vec<u32> = store
        .bag(id)
        .unwrap()
        .items_in(store::InventoryType::Equip)
        .map(|i| i.item.item_id)
        .collect();
    assert!(bagged.is_empty(), "the sword really left the bag");
    assert_eq!(s.fields.with_drops(crate::fields::FieldKey::world(net::opcode::START_MAP_ID), |d| d.len()), 1, "and it is on the floor");
}

/// The Use tab's wire number, from the store's own enum rather than a literal - `net::inventory`
/// exports `INV_EQUIP` and `INV_DECO` only, and a hand-written `2` here would be a claim.
fn use_tab() -> i8 {
    store::InventoryType::Use.as_u8() as i8
}

/// **Dropping 2 of a stack of 5 leaves 3 in the bag and puts 2 on the floor.**
///
/// The owner, 2026-09-09: *"I see that partial drop is not implemented, I also need this
/// implemented please."* It used to refuse, and the refusal said why: mode 1 UpdateQuantity
/// *"which net::inventory does not build"*. It had been built since and the guard was never
/// revisited.
///
/// **The mode is the whole test.** A mode 3 REMOVE here would clear the slot on screen while
/// the store still held three - the two ends then disagree about a slot, and the player reads
/// it as having lost three items. So this asserts the mode, the number the client is told,
/// and the number the store kept, because any one of the three alone can be right while the
/// others are wrong.
#[test]
fn dropping_part_of_a_stack_sends_mode_1_and_keeps_the_rest() {
    let (mut s, store, id) = gm_session();
    s.handle(&gm_chat("!item 2000000 5"));
    s.last_position = Some((520, 395));

    // count 2 out of the 5 sitting in Use slot 1.
    let out = s.on_inventory_move(&inventory_move(use_tab(), 1, 0, 2));

    assert_eq!(out[0].opcode, net::inventory::INVENTORY_OPERATION, "the 0x0070 goes first");
    assert_eq!(
        out[0].body[7],
        net::inventory::MODE_QUANTITY,
        "mode 1, NOT mode 3 - the slot is not empty: {}",
        out[0].what
    );
    assert_eq!(out[0].body[0], 1, "bExclRequestSent - the UI must not latch");
    assert_eq!(
        out[0].body,
        net::inventory::inventory_quantity(use_tab(), 1, 3),
        "the client must be told THREE are left"
    );
    assert_eq!(out[1].opcode, net::drops::DROP_ENTER_FIELD, "then the item on the ground");

    // And the store agrees with what the client was told.
    let left: Vec<u16> = store
        .bag(id)
        .unwrap()
        .items_in(store::InventoryType::Use)
        .map(|i| i.item.kind.quantity())
        .collect();
    assert_eq!(left, vec![3], "three stay in the bag");
    assert_eq!(s.fields.with_drops(crate::fields::FieldKey::world(net::opcode::START_MAP_ID), |d| d.len()), 1);
}

/// Dropping the **whole** stack still sends mode 3, because the slot really is empty.
///
/// The control for the test above: if the partial path had simply replaced the whole-slot
/// path, that one would still pass and every full drop would leave a slot drawing 0.
#[test]
fn dropping_a_whole_stack_still_sends_mode_3() {
    let (mut s, store, id) = gm_session();
    s.handle(&gm_chat("!item 2000000 5"));
    s.last_position = Some((520, 395));

    let out = s.on_inventory_move(&inventory_move(use_tab(), 1, 0, 5));

    assert_eq!(out[0].body[7], net::inventory::MODE_REMOVE, "{}", out[0].what);
    assert_eq!(
        store.bag(id).unwrap().items_in(store::InventoryType::Use).count(),
        0,
        "the slot really is empty"
    );
}

/// **Asking for more than is there takes what is there**, rather than refusing.
///
/// A stack can shrink between the drag starting and the packet arriving, and refusing on that
/// race would look to the player like the drop silently failed. The slot must end empty and
/// the reply must be mode 3, not a mode 1 claiming a negative remainder.
#[test]
fn asking_to_drop_more_than_the_slot_holds_drops_what_is_there() {
    let (mut s, store, id) = gm_session();
    s.handle(&gm_chat("!item 2000000 3"));
    s.last_position = Some((520, 395));

    let out = s.on_inventory_move(&inventory_move(use_tab(), 1, 0, 99));

    assert_eq!(out[0].body[7], net::inventory::MODE_REMOVE, "{}", out[0].what);
    assert_eq!(
        store.bag(id).unwrap().items_in(store::InventoryType::Use).count(),
        0,
        "the whole slot left"
    );
    assert_eq!(s.fields.with_drops(crate::fields::FieldKey::world(net::opcode::START_MAP_ID), |d| d.len()), 1);
}

/// The pick-up reads the drop id at **offset 13**, the offset one run measured.
///
/// It used to search every byte offset, because the layout was unknown. It is known now,
/// and the builder can never be read to confirm it further - it lives in `.themida`, whose
/// `SizeOfRawData` is zero.
#[test]
fn the_pick_up_handler_reads_the_drop_id_at_the_measured_offset() {
    let (mut s, store, id) = gm_session();
    let map = crate::fields::FieldKey::world(net::opcode::START_MAP_ID);
    s.handle(&gm_chat("!item 1302000"));
    s.last_position = Some((520, 395));
    s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, 1, 0, 1));
    let object_id =
        s.fields.with_drops(map, |d| d.on_field(map).map(|x| x.object_id).next().unwrap());

    // A body shaped like the real one: the id at offset 13, junk everywhere else.
    let mut body = crate::drops::CLIENT_DROP_PICK_UP.to_le_bytes().to_vec();
    body.extend_from_slice(&[0u8; crate::drops::PICK_UP_OBJECT_ID_AT]);
    body.extend_from_slice(&object_id.to_le_bytes());
    body.extend_from_slice(&[0u8; 17]);

    let out = s.handle(&body);
    // The pick-up reports itself in the screen message area, not the chat log.
    assert!(
        out.iter().any(|r| r.opcode == net::message::MESSAGE),
        "a pick-up should post to the screen message area: {out:?}"
    );
    assert!(
        out.iter().all(|r| r.opcode != net::notice::CHAT_NOTICE),
        "and nothing about it belongs in the chat log"
    );

    let bagged: Vec<u32> = store
        .bag(id)
        .unwrap()
        .items_in(store::InventoryType::Equip)
        .map(|i| i.item.item_id)
        .collect();
    assert_eq!(bagged, vec![1302000], "the sword came back");
    assert_eq!(s.fields.with_drops(map, |d| d.len()), 0, "and left the floor");
}

/// A pick-up naming no live drop is still answered - **with the packet that clears the gate**.
///
/// The client's pick-up sweep tests the same exclusive-request gate the inventory does
/// (`research/pick-up-latch.md` §4), so a refusal that sends only a chat line risks closing
/// every LATER pick-up. And it must never send a `0x046F` for a drop still on the floor:
/// telling the client to remove something it can still see is how an item disappears.
#[test]
fn a_pick_up_that_names_no_drop_clears_the_gate_and_removes_nothing() {
    let (mut s, _, _) = gm_session();
    let mut body = crate::drops::CLIENT_DROP_PICK_UP.to_le_bytes().to_vec();
    body.extend_from_slice(&[0xAA; 34]);

    let out = s.handle(&body);
    assert!(!out.is_empty(), "a packet with no answer freezes the whole UI");
    assert_eq!(
        out[0].opcode,
        net::inventory::INVENTORY_OPERATION,
        "the gate-clearing reply goes first"
    );
    assert_eq!(out[0].body, net::inventory::inventory_rejected());
    assert!(
        out.iter().all(|r| r.opcode != net::drops::DROP_LEAVE_FIELD),
        "never remove a drop that is still on the floor"
    );
}

/// A `0x00D2` body in the **preamble-first** shape: the 14-byte integrity block that
/// begins with the client's literal `100`, then the target channel.
fn change_channel(target: u8) -> Vec<u8> {
    let mut b = net::channel::CLIENT_CHANGE_CHANNEL.to_le_bytes().to_vec();
    b.extend_from_slice(&100u32.to_le_bytes());
    b.extend_from_slice(&[0u8; 10]);
    b.push(target);
    b.extend_from_slice(&0u32.to_le_bytes());
    b
}

/// A session that knows where both channels listen, which is what answering a channel
/// change needs.
fn two_channel_session() -> (Session, Arc<Store>, u32) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let config = Config {
        channel_id: 0,
        channels: vec!["127.0.0.1:8485".parse().unwrap(), "127.0.0.1:8486".parse().unwrap()],
        ..Config::default()
    };
    let mut s = Session::new(store.clone(), Arc::new(config));
    assert!(s.claim_for_character(id).contains("claimed the migration"));
    (s, store, id)
}

/// **Change Channel must be answered, and now it is.**
///
/// `net::opcode::CHANNEL_ENABLED` went to `1` in the same change, which is what makes the
/// button reachable at all. Draw and click read the same predicate in the client, so
/// there was no way to make the row look enabled without making it send - and an
/// unanswered packet freezes the whole UI.
#[test]
fn a_channel_change_is_answered_with_a_migration_for_the_target() {
    let (mut s, store, id) = two_channel_session();
    let out = s.handle(&change_channel(1));

    // **This used to assert exactly one `0x0011`, and that reply never arrived.**
    // `0x0011` is a login-stage opcode, below the channel switch's `0x70` floor, so a
    // channel connection cannot dispatch it. The owner, 2026-08-21: *"the transfer did not go
    // through, but I lost all ability to attack"* - `0x00D2` latches on send and only an
    // inbound handler clears it, so an undispatchable reply strands the player mid-migration.
    //
    // The body is measured; the opcode is the one field that cannot be read statically. So
    // the button sends every candidate, and the hook log names the one that dispatched.
    // ONE packet, and 0x001A is measured, not guessed: the sweep of 2026-08-21 found it.
    // The hook log writes a dispatch line per opcode on handler RETURN, and 0x001A took
    // 354 ms where 0x0019 took 64 us - the long one is the socket teardown and reconnect.
    assert_eq!(out.len(), 1, "one migrate reply, and never zero");
    assert_eq!(out[0].opcode, 0x001A);

    for r in &out {
        // 1 + 4 + 2 + 64. The padding is load-bearing: an over-read throws in the client,
        // so a wrong guess has to be inert rather than fatal.
        assert_eq!(r.body.len(), 71, "{:#06x}", r.opcode);
        assert_eq!(r.body[0], 1, "ok");
        // The address must be the TARGET channel's, not this one's. Getting that backwards
        // sends the client to the channel it is already on.
        assert_eq!(&r.body[1..5], &[127, 0, 0, 1], "network order, straight into sin_addr");
        assert_eq!(&r.body[5..7], &8486u16.to_le_bytes(), "channel 1's port, LITTLE endian");
        assert!(r.body[7..].iter().all(|b| *b == 0), "the tail is padding");
        assert!(r.what.contains("NOT authentication"), "{}", r.what);
    }

    // And a migration was minted for the target, so the other channel can claim it.
    let mut other = Session::new(store, Arc::new(Config { channel_id: 1, ..Config::default() }));
    assert!(other.claim_for_character(id).contains("claimed the migration"));
}

/// **The host in the answer is decided per connection, not copied from `--channels`.**
///
/// A LAN client that reached this channel at `192.168.1.20` is told `192.168.1.20` for the
/// target channel too - the listed `127.0.0.1` would send it back to itself. The port is
/// the target channel's. `net::advertise`.
#[test]
fn a_channel_change_advertises_the_host_this_client_reached_not_the_listed_one() {
    let (s, _, _) = two_channel_session();
    let mut s = s
        .with_peer_addr("192.168.1.77:51000".parse().unwrap())
        .with_local_addr("192.168.1.20:8485".parse().unwrap());
    let out = s.handle(&change_channel(1));
    assert_eq!(out.len(), 1);
    assert_eq!(&out[0].body[1..5], &[192, 168, 1, 20], "the interface the client reached");
    assert_eq!(&out[0].body[5..7], &8486u16.to_le_bytes(), "but channel 1's port");
    assert!(out[0].what.contains("Advertised as 192.168.1.20"), "{}", out[0].what);
}

/// **A channel migration is claimed by CHANNEL, because the packet carries no character.**
///
/// Measured 2026-08-21: `0x001A`'s body is `u8 ok, u32 ip, u16 port` - seven bytes, no
/// character id. The client's `0x007D` on the new channel then reported id **32513**, which
/// is `01 7f 00 00` read straight back out of our own body. Channel 1 refused it, answered
/// with the MINIMAL SetField, and the client faulted three seconds later.
///
/// A login migration is different - `0x0011` carries the id and the hello echoes it - so the
/// by-id path stays first and this only runs when it fails.
#[test]
fn a_channel_migration_is_claimed_by_channel_when_the_hello_names_nobody() {
    let (mut s, store, id) = two_channel_session();
    s.handle(&change_channel(1));

    // Channel 1, and the hello reports the bogus id the real client actually sent.
    let mut other = Session::new(store, Arc::new(Config { channel_id: 1, ..Config::default() }));
    let note = other.claim_for_character(32513);
    assert!(note.contains("by CHANNEL"), "{note}");
    assert!(note.contains("32513"), "the note must name the id it did not believe: {note}");
    assert_eq!(other.claimed().expect("claimed").character_id, id, "the real character");
}

/// Ambiguity is refused rather than guessed, and a claimed migration is not claimable twice.
#[test]
fn two_pending_migrations_to_one_channel_are_refused_rather_than_guessed() {
    let (mut s, store, _) = two_channel_session();
    s.handle(&change_channel(1));

    // A second, independent migration to the same channel.
    let other_account = store.create_account("second", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Second".to_string(), ..Default::default() };
    let second = store.create_character(other_account, 0, &chr).unwrap().id;
    store.create_migration(other_account, second, 0, 1).unwrap();

    let mut ch1 = Session::new(store.clone(), Arc::new(Config { channel_id: 1, ..Config::default() }));
    let note = ch1.claim_for_character(32513);
    assert!(note.contains("no single pending one"), "ambiguity must refuse: {note}");
    assert!(ch1.claimed().is_none());
}

/// A channel with no address is one nobody can enter, so it is refused **with a packet**.
#[test]
fn a_channel_with_no_address_is_refused_rather_than_ignored() {
    let (mut s, _, _) = two_channel_session();
    let out = s.handle(&change_channel(7));
    assert_eq!(out.len(), 1, "silence here would freeze the client's whole UI");
    assert_eq!(out[0].opcode, net::opcode::MIGRATE_COMMAND);
    assert!(out[0].what.contains("no address for channel 7"), "{}", out[0].what);
}

/// Asking for the channel you are already on is refused too - and still answered.
#[test]
fn changing_to_the_current_channel_is_refused_and_still_answered() {
    let (mut s, _, _) = two_channel_session();
    let out = s.handle(&change_channel(0));
    assert_eq!(out.len(), 1);
    assert!(out[0].what.contains("already on"), "{}", out[0].what);
}

/// A session standing next to a shopkeeper, with one buyable row and one quest item.
fn shop_session() -> (Session, Arc<Store>, u32) {
    shop_session_with(Vec::new(), Vec::new())
}

/// [`shop_session`] with `extra` rows APPENDED to Lucy's counter and `extra_data` added to
/// the item table. Appended, so the row indices the older shop tests buy by do not move;
/// the recharge tests stock a star this way.
fn shop_session_with(
    extra: Vec<crate::shops::ShopItem>,
    extra_data: Vec<(u32, crate::shops::ItemData)>,
) -> (Session, Arc<Store>, u32) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();

    let plain = |item_id, buy, sell, quest| crate::shops::ShopItem {
        item_id,
        name: format!("item {item_id}"),
        buy_price: buy,
        sell_price: sell,
        min_grade: None,
        quest_item: quest,
        trade_blocked: false,
    };
    let mut items = vec![plain(2000000, 50, 5, false), plain(4031507, 20, 1, true)];
    items.extend(extra);
    let shop = crate::shops::Shop {
        npc: "Lucy".to_string(),
        role: "Grocer".to_string(),
        map_label: "Maple Road".to_string(),
        items,
    };
    let mut item_data = std::collections::HashMap::new();
    item_data.insert(
        2000000u32,
        crate::shops::ItemData { price: 5, slot_max: 200, ..Default::default() },
    );
    item_data.insert(
        4031507u32,
        crate::shops::ItemData { price: 1, quest: true, ..Default::default() },
    );
    for (item_id, data) in extra_data {
        item_data.insert(item_id, data);
    }
    let table = crate::shops::ShopTable { shops: vec![shop], item_data, problems: Vec::new() };

    let mut npcs = std::collections::HashMap::new();
    npcs.insert(
        net::opcode::START_MAP_ID,
        vec![
            net::opcode::FieldNpc {
                object_id: 1000,
                template_id: 21,
                x: 0,
                cy: 0,
                fh: 1,
                rx0: 0,
                rx1: 0,
                f: 0,
            },
            // A second NPC on the same map that keeps NO shop, so the fall-through to
            // dialogue can be tested without turning a flag off.
            net::opcode::FieldNpc {
                object_id: 1001,
                template_id: 22,
                x: 0,
                cy: 0,
                fh: 1,
                rx0: 0,
                rx1: 0,
                f: 0,
            },
        ],
    );
    let mut shop_by_template = std::collections::HashMap::new();
    shop_by_template.insert(21u32, 0usize);

    let config = Config {
        // `send_shop` is gone: it existed because `0x0560` killed the client, and the real
        // cause was that this client has TWO shop windows and that was the one whose art it
        // does not ship. The classic `0x055D` counter goes out by default now.
        shops: table,
        shop_by_template,
        npcs,
        ..Config::default()
    };
    let mut s = Session::new(store.clone(), Arc::new(config));
    assert!(s.claim_for_character(id).contains("claimed the migration"));
    (s, store, id)
}

/// A classic-shop BUY, `0x00F5`: u8 0, u16 rowIndex, u32 itemId, u16 quantity.
fn classic_buy(row_index: u16, item_id: u32, quantity: u16) -> Vec<u8> {
    let mut b = net::classicshop::CLIENT_CLASSIC_SHOP_REQUEST.to_le_bytes().to_vec();
    b.push(0);
    b.extend_from_slice(&row_index.to_le_bytes());
    b.extend_from_slice(&item_id.to_le_bytes());
    b.extend_from_slice(&quantity.to_le_bytes());
    b
}

/// A classic-shop SELL, `0x00F5`: u8 1, u16 slot, u32 itemId, u16 quantity.
fn classic_sell(slot: u16, item_id: u32, quantity: u16) -> Vec<u8> {
    let mut b = net::classicshop::CLIENT_CLASSIC_SHOP_REQUEST.to_le_bytes().to_vec();
    b.push(1);
    b.extend_from_slice(&slot.to_le_bytes());
    b.extend_from_slice(&item_id.to_le_bytes());
    b.extend_from_slice(&quantity.to_le_bytes());
    b
}

/// A classic-shop RECHARGE, `0x00F5`: u8 2, u16 slot. The slot is all the client sends.
fn classic_recharge(slot: u16) -> Vec<u8> {
    let mut b = net::classicshop::CLIENT_CLASSIC_SHOP_REQUEST.to_le_bytes().to_vec();
    b.push(2);
    b.extend_from_slice(&slot.to_le_bytes());
    b
}

// ---------------------------------------------------------------------------------------
// Recharging stars. The owner, 2026-09-06: "they should be able to recharge stars at general
// merchants." Lucy is a Grocer and every Grocer in data/shops.txt lists Subi at 500.
// ---------------------------------------------------------------------------------------

/// `shop_session` with Lucy stocking **Subi Throwing Stars** (unitPrice 0.3, slotMax 500)
/// beside the Red Potion, the player holding `held` Subi in Use slot 1 and `mesos` mesos.
fn recharge_session(held: u16, mesos: u32) -> (Session, Arc<Store>, u32) {
    let subi = crate::shops::ShopItem {
        item_id: 2_070_000,
        name: "Subi Throwing Stars".to_string(),
        buy_price: 500,
        sell_price: 250,
        min_grade: None,
        quest_item: false,
        trade_blocked: false,
    };
    let subi_data = crate::shops::ItemData { price: 250, slot_max: 500, unit_price_milli: 300, ..Default::default() };
    let (s, store, id) = shop_session_with(vec![subi], vec![(2_070_000, subi_data)]);
    store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_070_000, held), 500).unwrap();
    store.set_mesos(id, mesos).unwrap();
    (s, store, id)
}

fn use_slot(store: &Arc<Store>, id: u32, slot: u16) -> u16 {
    store
        .bag_items(id, store::InventoryType::Use)
        .unwrap()
        .iter()
        .find(|r| r.slot == slot)
        .map(|r| r.item.kind.quantity())
        .unwrap_or(0)
}

/// The Grocer's Subi rows go out with the unit price in the eight bytes the client reads as
/// the recharge double - both the Buy row and its Sell twin - and the potion row is unchanged.
#[test]
fn a_grocers_star_rows_carry_the_recharge_price_and_the_potion_row_does_not() {
    let (mut s, _store, id) = recharge_session(480, 1_000);
    let out = s.open_shop_for(21, id).expect("Lucy keeps a shop");
    let body = &out[0].body;
    let (_, rows) = s.open_shop.clone().expect("the rows we sent are kept for the buy");
    // Buy rows only (potion, quest item, subi): no Sell twins since 2026-09-16.
    let mut at = net::classicshop::CLASSIC_HEAD_LEN;
    let mut priced = 0;
    for row in &rows {
        let len = row.wire_len();
        assert!(!row.sell, "no row is a Sell twin: {}", row.item_id);
        if net::bag::bundle_has_serial(row.item_id) {
            let bits = u64::from_le_bytes(body[at + len - 12..at + len - 4].try_into().unwrap());
            assert_eq!(f64::from_bits(bits), 0.3, "Subi's unitPrice, on the Buy row");
            priced += 1;
        }
        at += len;
    }
    assert_eq!(at, body.len(), "walked every row by its own width");
    assert_eq!(priced, 1, "the one Buy row carries it");
    assert!(out[0].what.contains("1 rechargeable with a unit price"), "{}", out[0].what);
}

#[test]
fn recharging_tops_the_stack_to_slotmax_and_charges_ceil_of_units_times_unit_price() {
    // 480 of 500: 20 units x 0.3 = 6.0 mesos exactly.
    let (mut s, store, id) = recharge_session(480, 1_000);
    s.open_shop_for(21, id).unwrap();
    let out = s.handle(&classic_recharge(1));
    assert_eq!(out[0].opcode, net::classicshop::CLASSIC_SHOP_RESULT);
    assert_eq!(out[0].body[0], net::classicshop::RESULT_SUCCESS, "{}", out[0].what);
    assert_eq!(use_slot(&store, id, 1), 500, "the stack is full");
    assert_eq!(store.mesos(id).unwrap(), 994);
    assert!(out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "the result moves nothing by itself");
    assert!(out.iter().any(|r| r.opcode == net::combat::STAT_CHANGED), "and the meso count is told");

    // 483 of 500: 17 x 0.3 = 5.1, rounded UP to 6 whole mesos - never a fraction, never free.
    let (mut s, store, id) = recharge_session(483, 1_000);
    s.open_shop_for(21, id).unwrap();
    let out = s.handle(&classic_recharge(1));
    assert_eq!(out[0].body[0], net::classicshop::RESULT_SUCCESS, "{}", out[0].what);
    assert_eq!(use_slot(&store, id, 1), 500);
    assert_eq!(store.mesos(id).unwrap(), 994, "5.1 mesos is 6, not 5");
}

#[test]
fn a_recharge_the_player_cannot_afford_is_refused_and_moves_nothing() {
    let (mut s, store, id) = recharge_session(100, 5); // 400 x 0.3 = 120 mesos wanted, 5 held
    s.open_shop_for(21, id).unwrap();
    let out = s.handle(&classic_recharge(1));
    assert_eq!(out.len(), 1, "a refusal and nothing else: {out:?}");
    assert_eq!(out[0].body, vec![net::classicshop::RESULT_NOT_ENOUGH_MESOS]);
    assert_eq!(use_slot(&store, id, 1), 100);
    assert_eq!(store.mesos(id).unwrap(), 5);
}

/// **Buying stars hands over a full set.** The owner, 2026-10-02: *"on purchase, the player should
/// receive a full stack of that star consumable instead of just a singular 1."* Lucy's Subi row
/// goes out with a per-purchase cap of 1 (the client's yes/no, no quantity box), and one
/// purchase - even one naming 3 - is one set of slotMax 500 at the row's 500 mesos, landing in
/// a fresh slot because slot 1 is already full.
#[test]
fn buying_stars_hands_over_a_full_set_at_the_rows_price() {
    let (mut s, store, id) = recharge_session(500, 2_000);
    s.open_shop_for(21, id).unwrap();
    let (_, rows) = s.open_shop.clone().unwrap();
    let subi = rows.iter().position(|r| r.item_id == 2_070_000).expect("Lucy stocks Subi") as u16;
    assert_eq!(rows[usize::from(subi)].max_per_purchase, 1, "a yes/no, not a quantity box");
    assert!(rows.iter().filter(|r| r.item_id == 2_000_000).all(|r| r.max_per_purchase > 1), "a potion keeps its box");

    let out = s.handle(&classic_buy(subi, 2_070_000, 3));
    assert_eq!(out[0].body[0], net::classicshop::RESULT_SUCCESS, "{}", out[0].what);
    assert!(out[0].what.contains("SET of 500"), "{}", out[0].what);
    assert_eq!(use_slot(&store, id, 1), 500, "the full stack is untouched");
    assert_eq!(use_slot(&store, id, 2), 500, "a whole set, not 1 and not 3");
    assert_eq!(store.mesos(id).unwrap(), 1_500, "one set at the row's price, not three");
}

/// `recharge_session` with **Wolbi** (unitPrice 0.4, slotMax 500) in the item table but not on
/// Lucy's shelf, and `wolbi` of them in Use slot 2 - the dropped stack from the owner's
/// screenshot, 2026-10-02.
fn dropped_wolbi_session(wolbi: u16, mesos: u32) -> (Session, Arc<Store>, u32) {
    let subi = crate::shops::ShopItem {
        item_id: 2_070_000,
        name: "Subi Throwing Stars".to_string(),
        buy_price: 500,
        sell_price: 250,
        min_grade: None,
        quest_item: false,
        trade_blocked: false,
    };
    let (s, store, id) = shop_session_with(
        vec![subi],
        vec![
            (2_070_000, crate::shops::ItemData { price: 250, slot_max: 500, unit_price_milli: 300, ..Default::default() }),
            (2_070_001, crate::shops::ItemData { price: 500, slot_max: 500, unit_price_milli: 400, ..Default::default() }),
        ],
    );
    store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_000_000, 1), 200).unwrap(); // slot 1
    store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_070_001, wolbi), 500).unwrap(); // slot 2
    store.set_mesos(id, mesos).unwrap();
    (s, store, id)
}

/// **A general store recharges a star it does not sell.** The owner, 2026-10-02: *"All stars
/// should be rechargeable at any general store"*, after 2 dropped Wolbi could not be topped up
/// at a Grocer that stocks only Subi. Wolbi goes out as a recharge-only row - price 0, so the
/// client files it in the Recharge list and no Buy tab - AFTER every stocked row, and the
/// recharge tops it up at Wolbi's own 0.4.
#[test]
fn a_general_store_recharges_a_dropped_star_it_does_not_stock() {
    let (mut s, store, id) = dropped_wolbi_session(2, 1_000);
    let out = s.open_shop_for(21, id).expect("Lucy keeps a shop");
    let (_, rows) = s.open_shop.clone().unwrap();
    let ids: Vec<u32> = rows.iter().map(|r| r.item_id).collect();
    assert_eq!(ids, vec![2_000_000, 4_031_507, 2_070_000, 2_070_001], "stocked rows first, the extra star last");
    let wolbi = rows[3];
    assert_eq!(wolbi.price, 0, "recharge-only: price 0 keeps it out of every Buy tab");
    assert_eq!(wolbi.unit_price(), Some(0.4));
    assert_eq!(rows[2].price, 500, "Subi is still sold at its shelf price");
    assert!(out[0].what.contains("3 buy,"), "{}", out[0].what);
    assert!(out[0].what.contains("2 rechargeable with a unit price, 1 of them recharge-only"), "{}", out[0].what);
    let walked: usize = net::classicshop::CLASSIC_HEAD_LEN + rows.iter().map(|r| r.wire_len()).sum::<usize>();
    assert_eq!(walked, out[0].body.len(), "every row on the wire at its own width");

    // 2 of 500: 498 x 0.4 = 199.2 mesos, rounded up to 200.
    let out = s.handle(&classic_recharge(2));
    assert_eq!(out[0].body[0], net::classicshop::RESULT_SUCCESS, "{}", out[0].what);
    assert_eq!(use_slot(&store, id, 2), 500, "the dropped stack is full");
    assert_eq!(store.mesos(id).unwrap(), 800);
}

/// **A recharge-only row cannot be bought** - the client never offers it, so a buy naming its
/// index is a forged body, and answering it would hand out free stars.
#[test]
fn buying_a_recharge_only_row_is_refused_and_moves_nothing() {
    let (mut s, store, id) = dropped_wolbi_session(2, 1_000);
    s.open_shop_for(21, id).unwrap();
    let out = s.handle(&classic_buy(3, 2_070_001, 500));
    assert_eq!(out.len(), 1, "one refusal, nothing else - {out:?}");
    assert_eq!(out[0].body, vec![net::classicshop::RESULT_NOT_ENOUGH_MESOS], "{}", out[0].what);
    assert_eq!(use_slot(&store, id, 2), 2);
    assert_eq!(use_slot(&store, id, 3), 0, "no new stack either");
    assert_eq!(store.mesos(id).unwrap(), 1_000);
}

/// Every refusal path is a `0x055E`, because the window latched on send: a potion, a star with
/// no unit price in the item table (Wolbi is absent from this fixture's, so no row lists it), a
/// stack already full, and an empty slot. **None of them moves an item or a meso.**
#[test]
fn recharging_anything_but_a_stocked_star_with_room_is_refused_with_a_result() {
    let (mut s, store, id) = recharge_session(500, 1_000);
    store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_000_000, 3), 200).unwrap(); // slot 2
    store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_070_001, 10), 500).unwrap(); // slot 3
    s.open_shop_for(21, id).unwrap();
    for (slot, why) in [(1u16, "a full stack"), (2, "a potion"), (3, "a star no row lists"), (7, "an empty slot")] {
        let out = s.handle(&classic_recharge(slot));
        assert_eq!(out.len(), 1, "{why}: one refusal, nothing else - {out:?}");
        assert_eq!(out[0].opcode, net::classicshop::CLASSIC_SHOP_RESULT, "{why}");
        assert_eq!(out[0].body, vec![net::classicshop::RESULT_NOT_ENOUGH_MESOS], "{why}: {}", out[0].what);
    }
    assert_eq!(use_slot(&store, id, 1), 500);
    assert_eq!(use_slot(&store, id, 3), 10);
    assert_eq!(store.mesos(id).unwrap(), 1_000);
}

/// The `0x00F2` body: u32 npcObjectId, i16 x, i16 y, u32 tail.
fn npc_click(object_id: u32) -> Vec<u8> {
    let mut b = net::script::CLIENT_NPC_CLICK.to_le_bytes().to_vec();
    b.extend_from_slice(&object_id.to_le_bytes());
    b.extend_from_slice(&0i16.to_le_bytes());
    b.extend_from_slice(&0i16.to_le_bytes());
    b.extend_from_slice(&u32::MAX.to_le_bytes());
    b
}

/// **Goal F's last mile.** Clicking a shopkeeper opens the shop rather than saying a line.
///
/// The owner, 2026-08-20: *"when I tried to click on Lucy (NPC ID 21), it does not open them
/// shop."* The packet had been decoded; nothing joined the shop's NPC *name* onto the
/// template id the click carries.
#[test]
fn clicking_a_shopkeeper_opens_the_shop() {
    let (mut s, _, _) = shop_session();
    let out = s.handle(&npc_click(1000));

    assert_eq!(out.len(), 1, "one packet: the shop");
    // **The CLASSIC counter, 0x055D.** This asserted 0x0560 until 2026-08-28, and that
    // packet's art is not in this client's WZ - it killed the client twice, before a single
    // row byte was read. The test passed the whole time, because a builder can be perfectly
    // correct about a window that does not exist.
    assert_eq!(out[0].opcode, net::classicshop::CLASSIC_OPEN_SHOP);
    assert_ne!(out[0].opcode, net::shop::OPEN_SHOP, "0x0560 must never go out again");
    assert_eq!(
        u32::from_le_bytes(out[0].body[0..4].try_into().unwrap()),
        21,
        "the NPC template the click named"
    );
    // Two buy rows - a quest item is perfectly buyable - and NO sell twins (2026-09-16: the
    // classic window filed them into the Buy list too, so every item drew twice).
    let rows = u16::from_le_bytes([out[0].body[19], out[0].body[20]]);
    assert_eq!(rows, 2, "rowCount is a u16, at the end of the 21-byte head");
    assert_eq!(
        out[0].body.len(),
        net::classicshop::CLASSIC_HEAD_LEN + 2 * net::classicshop::CLASSIC_ROW_LEN
    );
    assert!(out[0].what.contains("2 buy, 0 sell"), "{}", out[0].what);
}

/// **Getting hit subtracts HP and tells the client.** The owner: *"Getting hit by the mob does
/// not subtract my HP."*
///
/// The client computes the damage and does not apply it, so the server must - and must send
/// back the NEW HP, not a delta, because `0x007C` bit 10 is absolute.
#[test]
fn being_hit_subtracts_hp_and_answers_with_the_new_value() {
    let (mut s, store, id) = gm_session();
    // A real 147-byte body: snail template 2, object id 2004, damage 1.
    let hex = "00000000ffffffff0100000002002100b663ed0a0000000000000000000001000000010000000100000001000000d4070000d407000001000000000000000000000000000000000000490300008b010000000000000000000000000000ffffffff00000000ffffffff000000000000000002000000000000000000000000000000000000000100000000000000000000000000";
    let mut packet = net::userhit::CLIENT_USER_HIT.to_le_bytes().to_vec();
    packet.extend((0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()));

    let before = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().hp;
    let out = s.handle(&packet);

    let stat = out.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).expect("a 0x007C");
    assert_eq!(stat.body, net::stats::StatChange::hp_only(before - 1).build());
    let after = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
    assert_eq!(after.hp, before - 1, "the database, not just the reply");
}

/// **`!heal` exists because death does not**, so a character at zero HP is not stuck.
#[test]
fn heal_restores_and_says_so() {
    let (mut s, store, id) = gm_session();
    let mut chr = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
    chr.hp = 1;
    store.save_character_progress(&chr).unwrap();

    let out = s.handle(&gm_chat("!heal"));
    assert!(notice_text(&out[0]).contains("restored"), "{}", notice_text(&out[0]));
    assert!(out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED), "the client must be told");

    let after = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
    assert_eq!(after.hp, after.max_hp);
}

/// **A kill awards the mob's own EXP, and enough of it levels the character.**
///
/// Goal D, end to end: the EXP per kill is the client's own `mobtemplates.txt` value, the
/// curve is `data/exp-curve.txt`, and the `0x007C` carries the new level so the client plays
/// its own level-up effect without a separate packet.
#[test]
fn enough_experience_levels_the_character_and_says_so() {
    let (mut s, store, id) = gm_session();
    let curve = crate::expcurve::ExpCurve::parse("1 | 15
2 | 34
");
    s.config = Arc::new(Config { exp_curve: curve, ..(*s.config).clone() });

    let out = s.award_experience(15, "a test", true, false);

    let stat = out.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).expect("a 0x007C");
    assert!(stat.what.contains("LEVEL 1 -> 2"), "{}", stat.what);
    // **No chat line.** The owner asked for level-ups and pick-ups to stop going to the chat
    // log; the level-up animation comes from the `0x007C` itself, and the EXP gain goes to
    // the screen message area as `0x0089` type 3.
    assert!(
        out.iter().all(|r| r.opcode != net::notice::CHAT_NOTICE),
        "nothing about a level-up belongs in the chat log"
    );
    assert!(
        out.iter().any(|r| r.opcode == net::message::MESSAGE),
        "the EXP gain goes to the screen message area"
    );

    // The database, not the reply.
    let after = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
    assert_eq!((after.level, after.exp), (2, 0), "levelled, and the remainder is nothing");
    assert_eq!(after.ap, 5, "five ability points a level");
    assert_eq!(after.hp, after.max_hp, "a level-up restores");
}

/// **A level-up sends the skill points with it**, after the level. The owner, 2026-10-02: *"skill
/// points are not available immediately for use upon level up, a map change and or a cash shop or
/// change channel has to be performed"*. A level-30 Magician (job 200) reaching 31 is owed 64 in
/// the first pool; the control is an EXP gain short of a level, which sends no pool at all, and
/// a beginner, who has no pool to send.
#[test]
fn a_level_up_sends_the_new_skill_points_at_once() {
    let (mut s, store, _id) = gm_session();
    let curve = crate::expcurve::ExpCurve::parse("30 | 100
31 | 200
");
    s.config = Arc::new(Config { exp_curve: curve, ..(*s.config).clone() });
    let mut chr = s.claimed_character().unwrap();
    chr.job = 200;
    chr.level = 30;
    chr.exp = 0;
    store.save_character_progress(&chr).unwrap();

    let short = s.award_experience(50, "a test", true, false);
    assert!(!short.iter().any(|r| r.what.contains("skill points now")), "no level, no pool");

    let out = s.award_experience(50, "a test", true, false);
    let level = out.iter().position(|r| r.what.contains("LEVEL 30 -> 31")).expect("the level-up 0x007C");
    let pool = out.iter().position(|r| r.what.contains("skill points now")).expect("the pools, at once");
    assert!(pool > level, "after the level they were computed for");
    let owed = crate::skillpoints::first_job_entitlement(31, s.first_job_book_points(200));
    assert!(out[pool].what.contains(&format!("tier 1 = {owed}")), "{}", out[pool].what);
    assert_eq!(out[pool].opcode, net::stats::STAT_CHANGED);

    // A beginner levels with no pool to send.
    chr.job = 0;
    chr.level = 30;
    chr.exp = 0;
    store.save_character_progress(&chr).unwrap();
    let out = s.award_experience(100, "a test", true, false);
    assert!(out.iter().any(|r| r.what.contains("LEVEL 30 -> 31")));
    assert!(!out.iter().any(|r| r.what.contains("skill points now")), "a beginner has no pool");
}

/// Experience short of a level is banked and levels nobody.
#[test]
fn experience_short_of_a_level_is_just_banked() {
    let (mut s, store, id) = gm_session();
    let curve = crate::expcurve::ExpCurve::parse("1 | 15
");
    s.config = Arc::new(Config { exp_curve: curve, ..(*s.config).clone() });

    let out = s.award_experience(14, "a test", true, false);
    assert!(!out
        .iter()
        .filter(|r| r.opcode == net::notice::CHAT_NOTICE)
        .any(|r| notice_text(r).contains("Level up!")));

    let after = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
    assert_eq!((after.level, after.exp, after.ap), (1, 14, 0));
}

/// An award of zero sends nothing at all - a `0x007C` that changes nothing is a packet the
/// client parses for no reason, and most mobs in this game are worth some EXP but some are
/// worth none.
#[test]
fn an_award_of_zero_sends_no_packet() {
    let (mut s, _, _) = gm_session();
    assert!(s.award_experience(0, "a worthless mob", true, false).is_empty());
}

/// **A kill counts toward a started quest.** The owner: *"I accepted Sam's suggestion which
/// requires Snail kills, but the quest is not progressing even when I kill snails."*
///
/// The count is three zero-padded characters per mob slot - an integer renders as nothing.
#[test]
fn a_kill_advances_a_started_quest_and_the_count_is_a_string() {
    let (mut s, store, id) = gm_session();
    // Sam's Suggestion in the real data: quest 1006 wants ten of template 2.
    let reqs = net::quest::QuestRequirementTable::parse(
        "1006	1	mob	0	2	10
",
    );
    s.config = Arc::new(Config { quest_reqs: reqs, ..(*s.config).clone() });
    assert!(store.start_quest(id, 1006).unwrap());

    let out = s.credit_kill_to_quests(2, id);
    assert_eq!(out.len(), 1, "one started quest wants this mob: {out:?}");
    assert_eq!(out[0].opcode, net::quest::MESSAGE);
    assert_eq!(
        store.quest_row(id, 1006).unwrap().unwrap().progress,
        "001",
        "three zero-padded characters, not an integer"
    );

    // And it keeps counting.
    s.credit_kill_to_quests(2, id);
    assert_eq!(store.quest_row(id, 1006).unwrap().unwrap().progress, "002");
}

/// A kill counts toward **nothing** if the quest was never accepted.
#[test]
fn a_kill_credits_no_quest_the_player_has_not_started() {
    let (mut s, _, id) = gm_session();
    let reqs = net::quest::QuestRequirementTable::parse("1006	1	mob	0	2	10
");
    s.config = Arc::new(Config { quest_reqs: reqs, ..(*s.config).clone() });

    assert!(s.credit_kill_to_quests(2, id).is_empty());
}

/// A mob no quest asks about produces no packet at all.
#[test]
fn a_kill_of_an_unwanted_mob_sends_nothing() {
    let (mut s, _, id) = gm_session();
    let reqs = net::quest::QuestRequirementTable::parse("1006	1	mob	0	2	10
");
    s.config = Arc::new(Config { quest_reqs: reqs, ..(*s.config).clone() });
    assert!(s.credit_kill_to_quests(9999, id).is_empty());
}

/// **Killing a mob puts its drops on the floor** - the mob's own table, then the global one.
///
/// This is what the owner asked for: *"when a mob dies, it checks for its mob specific drop table
/// and any global drop table"*. The global row is the event hook, and it fires for a mob
/// that has no table of its own.
#[test]
fn a_kill_drops_the_mobs_own_table_and_the_global_one() {
    let (mut s, _, _) = gm_session();
    let drops = crate::droptables::DropTables::parse(
        "2 | 0       | 100 | 7 | 7 | 1 | mesos
         2 | 4000001 | 100 | 1 | 1 | 9 | Snail Shell
         * | 2022000 | 100 | 1 | 1 | 0 | Event Candy
",
    );
    s.config = Arc::new(Config { drops, ..(*s.config).clone() });
    // A drop needs somewhere to land; without it the server declines rather than guessing.
    s.last_position = Some((520, 395));

    let out = s.drops_from_kill(2, 2000, None, 204, crate::fields::FieldKey::world(net::opcode::START_MAP_ID));

    assert_eq!(out.len(), 3, "mesos, the shell, and the event item: {out:?}");
    assert!(
        out.iter().all(|r| r.opcode == net::drops::DROP_ENTER_FIELD),
        "a mob drop sends ONLY 0x046E - a 0x0070 would refuse an inventory request the          player never made"
    );
    assert_eq!(s.fields.with_drops(crate::fields::FieldKey::world(net::opcode::START_MAP_ID), |d| d.len()), 3, "and all three are on the floor");
}

/// **Drops fall where the mob died, not at the player's feet**, and several are staggered.
///
/// The owner, with a screenshot of the live server: *"they should drop from the killed mob's
/// position, not from the player character position"* and *"the items that drop should also
/// be slightly staggered from each other"*.
#[test]
fn drops_land_on_the_mob_and_are_staggered_apart() {
    let (mut s, _, _) = gm_session();
    let drops = crate::droptables::DropTables::parse(
        "2 | 4000001 | 100 | 1 | 1 | 9 | Shell
         2 | 2000000 | 100 | 1 | 1 | 4 | Potion
         2 | 1302000 | 100 | 1 | 1 | 3 | Sword
",
    );
    let map = crate::fields::FieldKey::world(net::opcode::START_MAP_ID);
    let mut mobs = std::collections::HashMap::new();
    mobs.insert(map.map, vec![net::mob::FieldMob::new(2000, 2, 100, 395, 1, 30)]);
    s.config = Arc::new(Config { drops, mobs, send_mobs: true, ..(*s.config).clone() });

    // Bring the field up and let the mob wander away from its spawn point.
    let cfg = s.config.clone();
    s.fields.seed(map, &cfg, 0);
    s.fields.due_respawns(map, &cfg, 999_999);
    s.fields.note_position(map, 2000, (500, 395));

    s.last_position = Some((1000, 395)); // the player, far away
    let out = s.drops_from_kill(2, 2000, Some((500, 395)), 204, map);
    assert_eq!(out.len(), 3);

    let xs: Vec<i16> = s
        .fields
        .with_drops(map, |d| d.on_field(map).map(|x| x.x).collect::<Vec<_>>());
    assert!(
        xs.iter().all(|x| (*x - 500).abs() <= 2 * crate::drops::DROP_STAGGER_PX),
        "every drop should be near the MOB at 500, not the player at 1000: {xs:?}"
    );
    let mut sorted = xs.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), 3, "three drops must not stack on one pixel: {xs:?}");
}

/// A mob that never moved has no reported position, so its drop falls at the player - the
/// old behaviour, kept as a fallback rather than a refusal.
#[test]
fn a_mob_that_never_moved_drops_at_the_player() {
    let (mut s, _, _) = gm_session();
    let drops = crate::droptables::DropTables::parse("2 | 4000001 | 100 | 1 | 1 | 9 | Shell
");
    s.config = Arc::new(Config { drops, ..(*s.config).clone() });
    s.last_position = Some((777, 395));

    s.drops_from_kill(2, 2000, None, 204, crate::fields::FieldKey::world(net::opcode::START_MAP_ID));
    let d = s.fields.with_drops(crate::fields::FieldKey::world(net::opcode::START_MAP_ID), |d| d.on_field(crate::fields::FieldKey::world(net::opcode::START_MAP_ID)).cloned().collect::<Vec<_>>()).into_iter().next().unwrap();
    assert_eq!(d.x, 777);
}

/// **Item variance, through a real kill.** The owner, 2026-09-24: *"I want to introduce item
/// variance for any items dropped by mobs following these rules."* A Lv 60 hat (range 6:
/// STR and DEX share it, 3 each; WDEF gets 30) dropped sixty times lands with its stats
/// rolled, inside every cap, on no line the template lacks, and not all the same. The
/// control: the Lv 0 starter sword on the same mob comes out exactly as its template.
#[test]
fn a_mobs_equip_drop_comes_out_with_rolled_stats() {
    let (mut s, _, _) = gm_session();
    let drops = crate::droptables::DropTables::parse("2 | 1002999 | 100 | 1 | 1 | 1 | Hat
2 | 1302000 | 100 | 1 | 1 | 1 | Sword
");
    let mut equips = s.config.equips.clone();
    let hat = crate::config::EquipTemplate { tuc: 7, inc_str: 2, inc_dex: 2, inc_pdd: 40, req_level: 60, ..Default::default() };
    let sword = crate::config::EquipTemplate { tuc: 7, inc_wat: 17, ..Default::default() };
    equips.insert(1_002_999, hat);
    equips.insert(1_302_000, sword);
    s.config = Arc::new(Config { drops, equips, ..(*s.config).clone() });
    s.last_position = Some((500, 395));
    let map = crate::fields::FieldKey::world(net::opcode::START_MAP_ID);

    for kill in 0..60 {
        s.drops_from_kill(2, 2000 + kill, None, 204, map);
    }
    let mut hats = Vec::new();
    let floor: Vec<crate::drops::LiveDrop> = s.fields.with_drops(map, |d| d.on_field(map).copied().collect());
    for d in &floor {
        let store::ItemKind::Equip(Some(stats)) = d.item.kind else { panic!("an unrolled equip on the floor: {d:?}") };
        match d.item_id() {
            1_002_999 => hats.push(stats),
            1_302_000 => assert_eq!(stats, sword.fresh_stats(), "Lv 0: range 0, nothing moves"),
            other => panic!("{other}"),
        }
    }
    assert_eq!(hats.len(), 60);
    for h in &hats {
        let st = h.stats;
        assert!(st.inc_str <= 5 && st.inc_dex <= 5, "2 + 3 at most: {st:?}");
        assert!((10..=70).contains(&st.inc_pdd), "40 +/- 30: {st:?}");
        assert_eq!((st.inc_int, st.inc_luk, st.inc_wat, st.inc_mhp), (0, 0, 0, 0), "no new lines: {st:?}");
        assert_eq!(h.options.remaining_enhancements, 7, "the slots are the template's");
    }
    let mut wdef: Vec<u16> = hats.iter().map(|h| h.stats.inc_pdd).collect();
    wdef.sort_unstable();
    wdef.dedup();
    assert!(wdef.len() > 10, "sixty hats, and they vary: {wdef:?}");
}

/// **A character comes back in at the spawn point nearest where they left**, and a teleport
/// lands on a random one. The owner, 2026-09-26: *"spawn the player to the closest spawn point where
/// they last were before they disconnect, change channel, go into cash shop ... If the player
/// does log off, the server should store which spawn point"*, and *"if a player is teleported
/// into a map, the server will choose a random spawn point."*
///
/// A map with two spawn points - 0 at x 0 and 3 at x 1000 - and a door (5, at x 1100) that is
/// not one. Claims, each an effect: the log off stores `(map, 3)` for a player standing at 950;
/// the next login's record carries portal 3; a map that changed since ignores it; going into the
/// Cash Shop records the new nearest; and teleports land only on 0 or 3, both of them.
#[test]
fn a_returning_character_comes_back_at_the_nearest_spawn_and_a_teleport_at_a_random_one() {
    const MAP: u32 = 104_040_000;
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Returner".to_string(), map_id: MAP, ..Default::default() };
    let id = store.create_character(account, 0, &chr).unwrap().id;
    store.set_character_map(id, MAP).unwrap();
    let mut config = Config::default();
    config.spawn_points.insert(MAP, vec![0, 3]);
    for (idx, x) in [(0u8, 0i16), (3, 1000), (5, 1100)] {
        config.portal_positions.insert((MAP, idx), (x, 0));
    }
    let config = Arc::new(config);
    let session = |store: &Arc<Store>| {
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::new(store.clone(), config.clone());
        assert!(s.claim_for_character(id).contains("claimed the migration"));
        s
    };

    // Logs off standing near the far spawn point.
    let mut s = session(&store);
    assert_eq!(s.claimed_character().unwrap().portal, 0, "nothing recorded yet: the default");
    s.last_position = Some((950, 0));
    drop(s);
    assert_eq!(store.spawn_point(id).unwrap(), Some((MAP, 3)), "the log off stored the nearest");

    // Comes back: the record every re-entry reads carries it.
    let mut s = session(&store);
    assert_eq!(s.claimed_character().unwrap().portal, 3, "back in at spawn point 3, not the origin");

    // Goes into the Cash Shop from beside spawn point 0: that is the new one.
    s.last_position = Some((40, 0));
    let _ = s.on_cash_shop_request(&[]);
    assert_eq!(store.spawn_point(id).unwrap(), Some((MAP, 0)));

    // Teleports: only spawn points, and both of them.
    let mut landed = std::collections::BTreeSet::new();
    for _ in 0..60 {
        let mut chr = s.claimed_character().unwrap();
        let out = s.teleport(&mut chr, MAP, "a test warp".to_string());
        assert!(out.iter().any(|r| r.opcode == net::opcode::SET_FIELD));
        landed.insert(chr.portal);
    }
    assert_eq!(landed, [0u8, 3].into_iter().collect(), "never the door, and not always the same spawn");

    // A stored point on another map is not applied here.
    store.set_spawn_point(id, 999, 3).unwrap();
    assert_eq!(s.claimed_character().unwrap().portal, 0, "a portal index means nothing on another map");
}

/// **A killed mob comes back.** The owner: *"The mobs that I kill also do not respawn."*
///
/// The delay is the WZ's own `mobTime`; a spawn point with none uses the field rate, and
/// reading that `0` as "never" would empty a map after one pass.
#[test]
fn a_dead_mob_respawns_when_its_timer_is_due() {
    let (mut s, _, _) = gm_session();
    let map = crate::fields::FieldKey::world(net::opcode::START_MAP_ID);
    let mut mobs = std::collections::HashMap::new();
    mobs.insert(map.map, vec![net::mob::FieldMob::new(2000, 2, 500, 395, 1, 30)]);
    s.config = Arc::new(Config { mobs, send_mobs: true, ..(*s.config).clone() });
    let cfg = s.config.clone();

    // **A field starts empty.** Seeding books the spawn points; nothing is alive yet.
    s.fields.seed(map, &cfg, 0);
    assert_eq!(s.fields.mob_count(map), 0, "no mobs until the timer fires");
    assert!(s.spawn_due_mobs(map, crate::config::DEFAULT_RESPAWN_MS - 1).is_empty());

    // And then it fills in.
    let out = s.spawn_due_mobs(map, crate::config::DEFAULT_RESPAWN_MS);
    assert_eq!(out.len(), 2, "MobEnterField then MobChangeController: {out:?}");
    assert_eq!(out[0].opcode, net::mob::MOB_ENTER_FIELD);
    assert_eq!(out[1].opcode, net::mobmove::MOB_CHANGE_CONTROLLER);
    assert_eq!(s.fields.mob_hp(map, 2000), Some(30), "alive at full HP");
    // Once, not forever.
    assert!(s.spawn_due_mobs(map, 9_999_999).is_empty());
}

/// `mobTime` of -1 means the spawn point never refills, and one in this client says so.
#[test]
fn a_spawn_point_marked_never_is_not_rescheduled() {
    assert_eq!(crate::config::respawn_delay_ms(crate::config::MOB_TIME_NEVER), None);
    assert_eq!(crate::config::respawn_delay_ms(0), Some(crate::config::DEFAULT_RESPAWN_MS));
    assert_eq!(crate::config::respawn_delay_ms(30), Some(30_000));
}

/// A mob with no table of its own still rolls the **global** table. That is the whole point
/// of an event drop.
#[test]
fn an_unknown_mob_still_rolls_the_global_table() {
    let (mut s, _, _) = gm_session();
    let drops = crate::droptables::DropTables::parse("* | 2022000 | 100 | 1 | 1 | 0 | Candy
");
    s.config = Arc::new(Config { drops, ..(*s.config).clone() });
    s.last_position = Some((520, 395));

    assert_eq!(s.drops_from_kill(999_999, 2000, None, 204, crate::fields::FieldKey::world(1)).len(), 1);
}

/// **With no known position a kill drops nothing, and says so.**
///
/// An item put where the player cannot reach is indistinguishable on screen from no drop at
/// all, so guessing would make the next run unreadable. The reply is still a real reply -
/// silence is what freezes the UI.
#[test]
fn a_kill_with_no_known_position_drops_nothing_but_still_answers() {
    let (mut s, _, _) = gm_session();
    let drops = crate::droptables::DropTables::parse("2 | 4000001 | 100 | 1 | 1 | 9 | Shell
");
    s.config = Arc::new(Config { drops, ..(*s.config).clone() });
    s.last_position = None;

    let out = s.drops_from_kill(2, 2000, None, 204, crate::fields::FieldKey::world(1));
    assert!(!out.is_empty(), "it must answer");
    assert!(out.iter().all(|r| r.opcode != net::drops::DROP_ENTER_FIELD), "and drop nothing");
    assert_eq!(s.fields.with_drops(crate::fields::FieldKey::world(net::opcode::START_MAP_ID), |d| d.len()), 0);
}

/// With no drop table at all, a kill is silent - not a panic and not a notice.
#[test]
fn a_kill_with_no_table_drops_nothing_quietly() {
    let (mut s, _, _) = gm_session();
    s.last_position = Some((1, 1));
    assert!(s.drops_from_kill(2, 2000, None, 204, crate::fields::FieldKey::world(1)).is_empty());
}

/// With the shop off, a shopkeeper **talks** instead of ending the session.
///
/// `0x0560` kills this client: the window it builds loads `UI/UIWindow2.img/Shop2/backgrnd`
/// and that image is not in this client's WZ, so the resource call throws and the unwinder
/// faults - before a single row byte is read. Two of the owner's manual launches died on it.
///
/// So the default must be a click that does something harmless and useful. A shopkeeper who
/// says a line is worth more than one who ends the session, and this pins that the fallback
/// is a real reply rather than silence - an unanswered click freezes the whole UI.
#[test]
fn an_npc_with_no_shop_falls_through_to_dialogue() {
    // **The `send_shop` gate is gone**, and this test used to turn it off. It existed because
    // the shop packet killed the client, and the fix was to not send it. The real cause was
    // that we were sending the wrong one of this client's two shop windows; the classic one
    // has its art and goes out by default now.
    //
    // What is still worth pinning is the fall-through itself: an NPC this server has no shop
    // for must SAY something. An unanswered click is not the hazard here - the 2026-08-19
    // capture shows the UI stays live with `0x00F2` unanswered - but a shopkeeper who does
    // nothing visible is indistinguishable from a click that never arrived.
    let (mut s, _, _) = shop_session();

    // Object 1001 is on the same map and its template keeps no shop.
    let out = s.handle(&npc_click(1001));
    assert!(
        out.iter().all(|r| r.opcode != net::classicshop::CLASSIC_OPEN_SHOP),
        "no shop for this NPC"
    );
    assert!(
        out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE),
        "it should say a line instead: {:?}",
        out.iter().map(|r| r.opcode).collect::<Vec<_>>()
    );
}

/// `--shop-rows 1` sends exactly one row, and it is a **buy** row.
///
/// The bisect instrument for the 2026-08-20 crash: twelve rows went out, the client threw
/// ten milliseconds later and faulted. "A row is wrong" and "twelve rows at once" look
/// identical on screen, and this is what tells them apart in one launch.
#[test]
fn the_shop_row_cap_keeps_one_buy_row() {
    let (mut s, _, _) = shop_session();
    // Mutated in place rather than by building a second Session: the migration is
    // single-use, so a fresh one claims nothing and every reply comes back empty.
    s.config = Arc::new(Config { shop_rows: Some(1), ..(*s.config).clone() });

    let out = s.handle(&npc_click(1000));
    let body = &out[0].body;
    assert_eq!(u16::from_le_bytes([body[19], body[20]]), 1, "one row");
    assert_eq!(body.len(), net::classicshop::CLASSIC_HEAD_LEN + net::classicshop::CLASSIC_ROW_LEN);
    // The surviving row must be the BUY one. The sell direction has never been on a wire in
    // either direction, so capping to the untested half would waste the launch.
    assert!(out[0].what.contains("1 buy, 0 sell"), "{}", out[0].what);
    assert!(out[0].what.contains("HELD BACK"), "the log must say rows were held: {}", out[0].what);
}

/// The cap can never produce a zero-row shop, because that is a different client arm.
///
/// `140d22656 test edi,edi` sends a `rowCount == 0` shop down a path that builds a dialog
/// box and never creates a counter at all - so a cap of 0 would test something other than
/// the shop.
#[test]
fn the_shop_row_cap_cannot_empty_the_counter() {
    let (mut s, _, _) = shop_session();
    s.config = Arc::new(Config { shop_rows: Some(0), ..(*s.config).clone() });

    let out = s.handle(&npc_click(1000));
    assert_eq!(u16::from_le_bytes([out[0].body[19], out[0].body[20]]), 1, "clamped up to one");
}

/// **Every row is a Buy row, each item once, and a quest item still cannot be sold.**
///
/// The owner, 2026-09-16: *"there are duplicate items in the NPC shop, one being regular price,
/// another being 10 times cheaper."* The classic window files every surviving row into the
/// Buy list whatever its sell byte says (`research/classic-shop-rows.md` §5), so the Sell
/// twins this server used to send were the duplicates. They are gone; the sell byte on every
/// row is 0; and the owner's older rule - *"Please do not allow quest items to be sold"* - is the
/// store's, which refuses the sell request.
#[test]
fn every_shop_row_is_a_buy_row_and_a_quest_item_still_cannot_be_sold() {
    let (mut s, store, id) = shop_session();
    let out = s.handle(&npc_click(1000));
    let rows = &out[0].body[net::classicshop::CLASSIC_HEAD_LEN..];
    let count = u16::from_le_bytes([out[0].body[19], out[0].body[20]]) as usize;
    let mut ids = Vec::new();
    for i in 0..count {
        let at = i * net::classicshop::CLASSIC_ROW_LEN;
        ids.push(u32::from_le_bytes(rows[at + 8..at + 12].try_into().unwrap()));
        assert_eq!(rows[at + net::classicshop::CLASSIC_ROW_LEN - 2], 0, "row {i}: the sell byte is 0");
    }
    assert_eq!(ids, vec![2000000, 4031507], "each stocked item exactly once");

    // The quest item in the bag: a sell request is refused and it stays.
    store.add_item(id, store::InventoryType::Etc, &store::Item::bundle(4031507, 1), 1).unwrap();
    let out = s.handle(&classic_sell(1, 4031507, 1));
    assert!(out.iter().any(|r| r.opcode == net::classicshop::CLASSIC_SHOP_RESULT), "answered: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert_eq!(store.bag(id).unwrap().items_in(store::InventoryType::Etc).count(), 1, "still in the bag");
}

/// A purchase charges the SERVER's price, adds the item, and says so three ways.
#[test]
fn buying_charges_mesos_and_delivers_the_item() {
    let (mut s, store, id) = shop_session();
    store.set_mesos(id, 1000).unwrap();
    s.handle(&npc_click(1000));

    // The CLASSIC request, 0x00F5: u8 0, u16 rowIndex, u32 itemId, u16 quantity.
    // Row 0 is the potion's buy row.
    let out = s.handle(&classic_buy(0, 2000000, 3));

    assert_eq!(out[0].opcode, net::classicshop::CLASSIC_SHOP_RESULT);
    assert_eq!(out[0].body[0], net::classicshop::RESULT_SUCCESS);
    assert!(out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "the bag");
    assert!(out.iter().any(|r| r.opcode == net::combat::STAT_CHANGED), "the meso count");

    assert_eq!(store.mesos(id).unwrap(), 1000 - 3 * 50, "3 at the AUTHORED buy price");
    let held: u16 = store
        .bag(id)
        .unwrap()
        .items_in(store::InventoryType::Use)
        .map(|i| i.item.kind.quantity())
        .sum();
    assert_eq!(held, 3);
}

/// Too little money is refused with the client's own code, and nothing moves.
#[test]
fn a_purchase_beyond_the_purse_is_refused_by_code() {
    let (mut s, store, id) = shop_session();
    store.set_mesos(id, 10).unwrap();
    s.handle(&npc_click(1000));

    let out = s.handle(&classic_buy(0, 2000000, 1));

    assert_eq!(out[0].body[0], net::classicshop::RESULT_NOT_ENOUGH_MESOS);
    assert_eq!(out.len(), 1, "a refusal moves nothing and owes no list");
    assert_eq!(store.mesos(id).unwrap(), 10, "nothing was charged");
    assert!(store.bag(id).unwrap().is_empty(), "and nothing was delivered");
}

/// A row index we never sent is refused - **and it is still answered**, because the window
/// latches on send and only a result clears it.
#[test]
fn an_unknown_row_index_is_refused_and_still_answered() {
    let (mut s, _, _) = shop_session();
    s.handle(&npc_click(1000));

    let out = s.handle(&classic_buy(99, 2000000, 1));
    assert_eq!(out.len(), 1, "one result, and it must exist");
    assert_eq!(out[0].opcode, net::classicshop::CLASSIC_SHOP_RESULT);
    assert_eq!(out[0].body[0], net::classicshop::RESULT_NOT_ENOUGH_MESOS);
    assert!(out[0].what.contains("not among the 2 rows"), "{}", out[0].what);
}

/// **The item id in the request is checked against the row.** A client whose list has
/// drifted from ours would otherwise buy whatever our row 0 happens to be.
#[test]
fn a_row_index_naming_the_wrong_item_is_refused() {
    let (mut s, store, id) = shop_session();
    s.handle(&npc_click(1000));

    let before = store.mesos(id).unwrap();
    let out = s.handle(&classic_buy(0, 4031507, 1));
    assert_eq!(out[0].body[0], net::classicshop::RESULT_NOT_ENOUGH_MESOS);
    assert!(out[0].what.contains("the client asked for"), "{}", out[0].what);
    assert_eq!(store.mesos(id).unwrap(), before, "nothing was charged");
}

/// Current MP straight out of the database, by character id. (`mp_of` is taken.)
// ---------------------------------------------------------------------------------------
// Arrows. The owner, 2026-09-06: "regular attacks or skills using bows/crossbows should consume
// arrows from the use tab depending on the attack amount. for example double shot should
// consume 2 arrows."
// ---------------------------------------------------------------------------------------

/// A level-1 archer holding `weapon` with `arrows` x `count` in Use slot 1, and the skills of
/// the Bowman book granted. `None` when the generated skill table is absent (gitignored).
fn archer_with(weapon: u32, arrows: u32, count: u16) -> Option<(Arc<Store>, Session, u32)> {
    first_job_with(300, weapon, &[3_001_001, 3_001_002, 3_001_003], arrows, count, 1000)
}

/// A level-1 rogue holding `weapon` with `stars` x `count` in Use slot 1, and the Rogue book
/// (Double Stab, Lucky Seven) granted at level 1.
fn rogue_with(weapon: u32, stars: u32, count: u16) -> Option<(Arc<Store>, Session, u32)> {
    first_job_with(400, weapon, &[4_001_002, 4_001_003], stars, count, 800)
}

/// A level-1 character of `job` holding `weapon`, with `skills` at level 1, `count` of
/// `ammo` in Use slot 1 when `count > 0`, **50/50 HP and 200/200 MP**, claimed. `None` when
/// the generated skill table is absent (gitignored) - every caller returns early then.
fn first_job_with(
    job: u16,
    weapon: u32,
    skills: &[u32],
    ammo: u32,
    count: u16,
    max_stack: u16,
) -> Option<(Arc<Store>, Session, u32)> {
    let table = std::path::Path::new("../../gm-handbook/skills.txt");
    if !table.exists() {
        return None; // python tools/dump_skills.py
    }
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Robin".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.job = job;
    made.hp = 50;
    made.max_hp = 50;
    made.mp = 200;
    made.max_mp = 200;
    store.save_character_progress(&made).unwrap();
    for skill in skills {
        store.set_skill_level(made.id, *skill, 1).unwrap();
    }
    // The weapon goes into the Equip bag and then onto the weapon slot (11), the way the
    // client's drag does it; nothing here auto-equips.
    store.add_item(made.id, store::InventoryType::Equip, &store::Item::equip(weapon), 1).unwrap();
    store.equip_from_bag(made.id, 1, 11).unwrap();
    if count > 0 {
        store
            .add_item(made.id, store::InventoryType::Use, &store::Item::bundle(ammo, count), max_stack)
            .unwrap();
    }
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let config = Config {
        firstjob: crate::firstjob::CombatTable::load(table),
        ..Config::default()
    };
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(made.id);
    Some((store, s, made.id))
}

/// The captured swing from `an_attack_skill_costs_mp_and_a_potion_does_not_undo_it`, with
/// `skill` and level 1 patched into body offsets 2 and 6, behind `opcode`.
fn swing_packet(opcode: u16, skill: u32) -> Vec<u8> {
    const SWING: &str = concat!(
        "0000000000000000000000000000000000050000009fae34080104000000e6a81f08f7018b0100000000f7018b010000",
        "0000000000000000000000000000000000000000000000000100000001000000000a0055736572204d656c6565890100",
        "000000000000000000000000000000000000000000000000000080e8da8f00",
    );
    let hex: String = SWING.chars().filter(|c| !c.is_whitespace()).collect();
    let mut payload: Vec<u8> =
        (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect();
    payload[2..6].copy_from_slice(&skill.to_le_bytes());
    payload[6] = if skill == 0 { 0 } else { 1 };
    assert!(net::attack::parse(opcode, &payload).is_ok(), "the patched body must still parse");
    let mut body = opcode.to_le_bytes().to_vec();
    body.extend_from_slice(&payload);
    body
}

// ---------------------------------------------------------------------------------------
// The damage guard (crate::damageguard), on a real character. The owner, 2026-10-02: a hit more
// than 25% over what the character could deal is CAPPED to that and the attacker logged.
// ---------------------------------------------------------------------------------------

/// The session's config with the CLIENT's real equipment templates and item data - what the
/// live server loads from `gm-handbook/`. `None` when the handbook is absent (gitignored).
fn with_real_items(s: &mut Session) -> Option<()> {
    let equips = std::path::Path::new("../../gm-handbook/equips.txt");
    let items = std::path::Path::new("../../gm-handbook/itemdata.txt");
    if !equips.exists() || !items.exists() {
        return None;
    }
    let mut config = (*s.config).clone();
    config.equips = Config::load_equips(equips);
    config.shops.item_data = crate::shops::load_item_data(items);
    s.config = Arc::new(config);
    Some(())
}

/// One target taking `damage` in a single hit, `critical` as the client flags it.
fn one_hit(damage: u64, critical: bool) -> net::combat::AttackTarget {
    net::combat::AttackTarget {
        object_id: 2000,
        second: 0,
        hits: vec![net::combat::AttackHit { flag_a: false, flag_b: critical, damage }],
    }
}

/// **The positive control: an absurd claim is capped, an honest one is untouched.** A level-1
/// warrior with a sword swings: the swing is priced (not unchecked), 1 point and the ceiling
/// itself pass as they are, and a million comes back as exactly the limit - the ceiling plus
/// 25% - not the million and not zero. A critical hit is allowed more than an ordinary one.
#[test]
fn the_damage_guard_caps_an_impossible_hit_and_passes_an_honest_one() {
    let Some((_store, mut s, _id)) = first_job_with(100, 1_302_000, &[1_001_004, 1_001_005], 0, 0, 1) else { return };
    if with_real_items(&mut s).is_none() {
        return;
    }
    let pricing = s.price_swing(net::combat::USER_MELEE_ATTACK, &swing_packet(net::combat::USER_MELEE_ATTACK, 0)[2..]);
    let crate::session::damageguard::Pricing::Physical { attacker, class, .. } = &pricing else {
        panic!("a sword swing must be priced, got {pricing:?}");
    };
    let ceiling = crate::damageguard::physical_ceiling(attacker, *class, 0, false).unwrap();
    assert!(ceiling > 0, "{pricing:?}");
    let limit = crate::damageguard::limit(ceiling);

    assert_eq!(s.guarded_damage(&pricing, &one_hit(1, false)), 1, "an honest hit");
    assert_eq!(s.guarded_damage(&pricing, &one_hit(ceiling, false)), ceiling, "the ceiling itself");
    assert_eq!(s.guarded_damage(&pricing, &one_hit(limit, false)), limit, "25% over is still allowed");
    assert_eq!(s.guarded_damage(&pricing, &one_hit(1_000_000, false)), limit, "CAPPED, not dropped");
    let crit_limit = s.guarded_damage(&pricing, &one_hit(1_000_000, true));
    assert!(crit_limit > limit, "a critical may reach more: {crit_limit} vs {limit}");
}

/// **`--damage-guard log` measures and caps nothing.**
#[test]
fn the_damage_guard_in_log_mode_applies_the_hit_whole() {
    let Some((_store, mut s, _id)) = first_job_with(100, 1_302_000, &[1_001_004], 0, 0, 1) else { return };
    if with_real_items(&mut s).is_none() {
        return;
    }
    Arc::get_mut(&mut s.config).expect("sole owner").damage_guard = crate::damageguard::Mode::Log;
    let pricing = s.price_swing(net::combat::USER_MELEE_ATTACK, &swing_packet(net::combat::USER_MELEE_ATTACK, 0)[2..]);
    assert_eq!(s.guarded_damage(&pricing, &one_hit(1_000_000, false)), 1_000_000, "log mode applies it");
}

/// **Every thief and archer skill is priced** (the owner, 2026-10-02: *"Why can we not create
/// formulas for thief and archer skills?"*). Lucky Seven and Avenger by the claw formula, Arrow
/// Bomb by its tooltip's percent, Three Snails at its fixed 15 and Shadow Meso at mesos x 8 -
/// and a million on each comes back capped, not whole.
#[test]
fn the_damage_guard_prices_every_thief_and_archer_skill() {
    use crate::session::damageguard::Pricing;
    let priced = |s: &Session, opcode: u16, skill: u32| s.price_swing(opcode, &swing_packet(opcode, skill)[2..]);

    let Some((_store, mut thief, _id)) = rogue_with(CLAW, HWABI, 100) else { return };
    if with_real_items(&mut thief).is_none() {
        return;
    }
    for skill in [4_001_003u32, 4_111_004] {
        let p = priced(&thief, net::combat::USER_SHOOT_ATTACK, skill);
        assert!(matches!(p, Pricing::Physical { .. }), "skill {skill}: {p:?}");
        assert!(thief.guarded_damage(&p, &one_hit(1_000_000, false)) < 1_000_000, "skill {skill} is capped");
    }
    let meso = priced(&thief, net::combat::USER_SHOOT_ATTACK, crate::damageguard::SHADOW_MESO);
    let Pricing::Fixed { ceiling, .. } = meso else { panic!("Shadow Meso: {meso:?}") };
    assert_eq!(ceiling, 200 * 8, "level 1: 200 mesos x 8");
    assert_eq!(thief.guarded_damage(&meso, &one_hit(1_600, false)), 1_600);
    assert_eq!(thief.guarded_damage(&meso, &one_hit(1_000_000, false)), crate::damageguard::limit(1_600));

    let snails = priced(&thief, net::combat::USER_SHOOT_ATTACK, 1_000);
    let Pricing::Fixed { ceiling, .. } = snails else { panic!("Three Snails: {snails:?}") };
    assert_eq!(ceiling, 15, "level 1's fixdamage");
    assert_eq!(thief.guarded_damage(&snails, &one_hit(15, false)), 15);
    assert_eq!(thief.guarded_damage(&snails, &one_hit(9_999, false)), crate::damageguard::limit(15));

    let Some((_store, mut archer, _id)) = archer_with(BOW, BOW_ARROWS, 100) else { return };
    if with_real_items(&mut archer).is_none() {
        return;
    }
    for skill in [3_101_004u32, 3_001_003] {
        let p = priced(&archer, net::combat::USER_SHOOT_ATTACK, skill);
        assert!(matches!(p, Pricing::Physical { .. }), "skill {skill}: {p:?}");
        assert!(archer.guarded_damage(&p, &one_hit(1_000_000, false)) < 1_000_000, "skill {skill} is capped");
    }
    let Pricing::Physical { attacker, .. } = priced(&archer, net::combat::USER_SHOOT_ATTACK, 3_101_004) else { unreachable!() };
    assert_eq!(attacker.skill_damage_percent, 80, "Arrow Bomb level 1: the tooltip's 80%, not the column's 0");
}

/// **The star's attack is in the ceiling.** The same thief, the same plain throw: holding Hwabi
/// (attack 29) raises the limit over holding nothing to throw.
#[test]
fn the_damage_guard_counts_the_stars_attack() {
    let Some((store, mut s, id)) = rogue_with(CLAW, HWABI, 100) else { return };
    if with_real_items(&mut s).is_none() {
        return;
    }
    let throw = swing_packet(net::combat::USER_SHOOT_ATTACK, 0);
    let ceiling = |s: &Session| match s.price_swing(net::combat::USER_SHOOT_ATTACK, &throw[2..]) {
        crate::session::damageguard::Pricing::Physical { attacker, class, .. } => {
            crate::damageguard::physical_ceiling(&attacker, class, 0, false).unwrap()
        }
        other => panic!("{other:?}"),
    };
    let with_hwabi = ceiling(&s);
    store.set_inventory_slot(id, store::InventoryType::Use, 1, &store::Item::bundle(HWABI, 0)).unwrap();
    let empty = ceiling(&s);
    assert!(with_hwabi > empty, "Hwabi's 29 attack must raise the ceiling: {with_hwabi} vs {empty}");
}

/// **A weapon whose numbers are unknown is never priced** - the failure found writing these
/// tests: with no templates loaded the sword read 0 attack and a million was capped to 2.
/// Without `gm-handbook/equips.txt` the swing is unchecked and applied whole.
#[test]
fn the_damage_guard_does_not_price_a_weapon_with_no_known_attack() {
    let Some((_store, s, _id)) = first_job_with(100, 1_302_000, &[1_001_004], 0, 0, 1) else { return };
    let pricing = s.price_swing(net::combat::USER_MELEE_ATTACK, &swing_packet(net::combat::USER_MELEE_ATTACK, 0)[2..]);
    assert!(matches!(pricing, crate::session::damageguard::Pricing::Unchecked(_)), "{pricing:?}");
    assert_eq!(s.guarded_damage(&pricing, &one_hit(1_000_000, false)), 1_000_000);
}

/// **Disorder puts a status on the mob it damaged** (the owner, 2026-10-02: *"Still no debuff status
/// shown on mobs hit by disorder."*). Level 1 from the client's own table: attack -5, weapon
/// defence -1, 10 s. The attacker gets exactly `net::mobstat`'s bytes for that mob; the server
/// remembers the attack cut for its touch-damage fallback, a second cast refreshes rather than
/// stacks, and after 10 s the cut is gone. A skill that is not a debuff sends nothing.
#[test]
fn disorder_puts_its_attack_and_defence_cut_on_the_mob_it_hit() {
    let Some((_store, mut s, _id)) = rogue_with(CLAW, HWABI, 10) else { return };
    let map = crate::fields::FieldKey::world(1);
    let out = s.debuff_mobs(map, crate::session::mobdebuff::DISORDER, 1, &[2042]);
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!(out[0].opcode, net::mobstat::MOB_STAT_SET);
    let status = |index, value| net::mobstat::MobStatus { index, value, reason: 4_001_000, duration_ms: 10_000 };
    assert_eq!(
        out[0].body,
        net::mobstat::mob_stat_set(2042, &[status(net::mobstat::PAD, -5), status(net::mobstat::PDR, -1)])
    );
    assert_eq!(s.mob_attack_cut(map, 2042), 5);
    assert_eq!(s.mob_attack_cut(map, 2043), 0, "only the mob that was hit");

    s.debuff_mobs(map, crate::session::mobdebuff::DISORDER, 1, &[2042]);
    assert_eq!(s.mob_attack_cut(map, 2042), 5, "refreshed, not stacked to 10");

    s.clock_ms += 10_001;
    assert_eq!(s.mob_attack_cut(map, 2042), 0, "expired after 10 s");
    assert!(s.debuff_mobs(map, 4_001_003, 1, &[2042]).is_empty(), "Lucky Seven debuffs nothing");
    assert!(s.debuff_mobs(map, crate::session::mobdebuff::DISORDER, 1, &[]).is_empty(), "a miss debuffs nothing");
}

/// How many arrows sit in Use slot 1, or 0 when the slot is empty.
fn arrows_in_slot_1(store: &Arc<Store>, id: u32) -> u16 {
    store
        .bag_items(id, store::InventoryType::Use)
        .unwrap()
        .iter()
        .find(|r| r.slot == 1)
        .map(|r| r.item.kind.quantity())
        .unwrap_or(0)
}

const BOW: u32 = 1_452_000;
const CROSSBOW: u32 = 1_462_000;
const BOW_ARROWS: u32 = 2_060_000;
const CROSSBOW_ARROWS: u32 = 2_061_000;

#[test]
fn a_plain_shot_with_a_bow_takes_one_arrow_and_tells_the_client() {
    let Some((store, mut s, id)) = archer_with(BOW, BOW_ARROWS, 50) else { return };
    let out = s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 0));
    assert_eq!(arrows_in_slot_1(&store, id), 49);
    let qty = out
        .iter()
        .find(|r| r.opcode == net::inventory::INVENTORY_OPERATION)
        .expect("a 0x0070 must carry the new stack size, or the client's count drifts");
    assert_eq!(qty.body[7], net::inventory::MODE_QUANTITY, "mode 1: the stack shrank, it is not empty");
}

#[test]
fn double_shot_takes_two_arrows_arrow_blow_one() {
    let Some((store, mut s, id)) = archer_with(BOW, BOW_ARROWS, 50) else { return };
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 3_001_002));
    assert_eq!(arrows_in_slot_1(&store, id), 48, "Double Shot: bulletConsume 2");
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 3_001_001));
    assert_eq!(arrows_in_slot_1(&store, id), 47, "Arrow Blow: bulletConsume 1");
}

#[test]
fn power_knockback_and_a_melee_swing_with_a_bow_take_no_arrow() {
    let Some((store, mut s, id)) = archer_with(BOW, BOW_ARROWS, 50) else { return };
    // Power Knockback is the bow swung as a club: no bullet column at any level.
    s.handle(&swing_packet(net::combat::USER_MELEE_ATTACK, 3_001_003));
    assert_eq!(arrows_in_slot_1(&store, id), 50);
    // And a plain melee swing while holding a bow fires nothing either.
    s.handle(&swing_packet(net::combat::USER_MELEE_ATTACK, 0));
    assert_eq!(arrows_in_slot_1(&store, id), 50);
}

#[test]
fn a_crossbow_takes_crossbow_arrows_and_ignores_bow_arrows() {
    let Some((store, mut s, id)) = archer_with(CROSSBOW, BOW_ARROWS, 50) else { return };
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 0));
    assert_eq!(arrows_in_slot_1(&store, id), 50, "2060xxx are for bows; a crossbow leaves them alone");
    let Some((store, mut s, id)) = archer_with(CROSSBOW, CROSSBOW_ARROWS, 50) else { return };
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 0));
    assert_eq!(arrows_in_slot_1(&store, id), 49);
}

#[test]
fn the_last_arrow_empties_the_slot_with_a_remove_and_a_quiver_short_of_two_is_not_refused() {
    let Some((store, mut s, id)) = archer_with(BOW, BOW_ARROWS, 1) else { return };
    // Double Shot wants 2; there is 1. The swing is not refused, the one is taken, and the
    // slot is reported EMPTY (mode 3), not "down to 0".
    let out = s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 3_001_002));
    assert_eq!(arrows_in_slot_1(&store, id), 0);
    let op = out
        .iter()
        .find(|r| r.opcode == net::inventory::INVENTORY_OPERATION)
        .expect("the emptied slot must be reported");
    assert_eq!(op.body[7], net::inventory::MODE_REMOVE);
    // With no arrows at all the swing still goes through and nothing is sent about a bag.
    let out = s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 0));
    assert!(!out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION));
}

#[test]
fn a_sword_never_costs_an_arrow_however_the_packet_is_labelled() {
    let Some((store, mut s, id)) = archer_with(1_302_000, BOW_ARROWS, 50) else { return };
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 0));
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 3_001_002));
    assert_eq!(arrows_in_slot_1(&store, id), 50);
}

// ---------------------------------------------------------------------------------------
// Stars. The owner, 2026-09-06: "Thief skills/basic attack should consume stars similar to bowman
// with arrows." A claw draws from the 207 family; Lucky Seven throws two.
// ---------------------------------------------------------------------------------------

const CLAW: u32 = 1_472_000; // Garnier
const DAGGER: u32 = 1_332_000; // the Rogue's other weapon - Double Stab wants it
const SUBI: u32 = 2_070_000;
const HWABI: u32 = 2_070_007;

#[test]
fn a_plain_throw_with_a_claw_takes_one_star_and_tells_the_client() {
    let Some((store, mut s, id)) = rogue_with(CLAW, SUBI, 50) else { return };
    let out = s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 0));
    assert_eq!(arrows_in_slot_1(&store, id), 49);
    let qty = out
        .iter()
        .find(|r| r.opcode == net::inventory::INVENTORY_OPERATION)
        .expect("a 0x0070 must carry the new stack size, or the client's count drifts");
    assert_eq!(qty.body[7], net::inventory::MODE_QUANTITY);
}

/// Lucky Seven: `bulletCount 2`, no `bulletConsume` column - one star per projectile, which
/// is the owner's "depending on the attack amount" rule and **[I]** until a client run counts it.
#[test]
fn lucky_seven_takes_two_stars_one_per_projectile() {
    let Some((store, mut s, id)) = rogue_with(CLAW, SUBI, 50) else { return };
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 4_001_003));
    assert_eq!(arrows_in_slot_1(&store, id), 48, "Lucky Seven: bulletCount 2, charged 2");
}

#[test]
fn double_stab_with_a_dagger_takes_no_star_and_a_claw_never_touches_arrows() {
    // A dagger is not a throwing weapon: neither its skill nor a mislabelled shot costs a star.
    let Some((store, mut s, id)) = rogue_with(DAGGER, SUBI, 50) else { return };
    s.handle(&swing_packet(net::combat::USER_MELEE_ATTACK, 4_001_002));
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 0));
    assert_eq!(arrows_in_slot_1(&store, id), 50);
    // And a claw draws from 207xxxx only - a quiver of bow arrows is not ammunition for it.
    let Some((store, mut s, id)) = rogue_with(CLAW, BOW_ARROWS, 50) else { return };
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 0));
    assert_eq!(arrows_in_slot_1(&store, id), 50);
}

/// The whole `207xxxx` family counts - Hwabi as much as Subi - and **the last star leaves an
/// EMPTY STACK, not an empty slot.** The owner, 2026-10-02: *"When stars reach 0, it should remain
/// in the player's inventory because they should be able to recharge them at any general
/// store."* Until then this test asserted a REMOVE. Now the row is still in Use slot 1 at 0,
/// the client is told quantity 0 (no REMOVE), and the next throw takes nothing from it.
#[test]
fn every_star_in_the_family_counts_and_the_last_one_leaves_an_empty_stack() {
    let Some((store, mut s, id)) = rogue_with(CLAW, HWABI, 1) else { return };
    let out = s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 0));
    let row = store.inventory_slot(id, store::InventoryType::Use, 1).unwrap().expect("the stack is still there");
    assert_eq!((row.item_id, row.kind.quantity()), (HWABI, 0), "Hwabi at 0, in its slot");
    let ops: Vec<_> = out.iter().filter(|r| r.opcode == net::inventory::INVENTORY_OPERATION).collect();
    assert_eq!(ops.len(), 1, "{ops:?}");
    assert_eq!(ops[0].body, net::inventory::inventory_quantity(store::InventoryType::Use.as_u8() as i8, 1, 0), "a count of 0, not a REMOVE");

    let again = s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 0));
    assert!(!again.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "nothing left to take");
    assert_eq!(store.inventory_slot(id, store::InventoryType::Use, 1).unwrap().map(|i| i.kind.quantity()), Some(0));
}

/// **An empty star stack recharges at a general store**, the reason it is kept. Lucy stocks
/// Subi; 0 of 500 tops up to 500 at 0.3 each = 150 mesos, into the same slot.
#[test]
fn an_empty_star_stack_recharges_to_full() {
    let (mut s, store, id) = recharge_session(1, 1_000);
    store.set_inventory_slot(id, store::InventoryType::Use, 1, &store::Item::bundle(2_070_000, 0)).unwrap();
    s.open_shop_for(21, id).unwrap();
    let out = s.handle(&classic_recharge(1));
    assert_eq!(out[0].body[0], net::classicshop::RESULT_SUCCESS, "{}", out[0].what);
    assert_eq!(use_slot(&store, id, 1), 500);
    assert_eq!(store.mesos(id).unwrap(), 850);
}

/// **An empty star stack can still be sold** - the one way to get rid of it - whatever count the
/// client names, and it is not mistaken for a stale click.
#[test]
fn an_empty_star_stack_can_be_sold_away() {
    let (mut s, store, id) = recharge_session(1, 1_000);
    store.set_inventory_slot(id, store::InventoryType::Use, 1, &store::Item::bundle(2_070_000, 0)).unwrap();
    s.open_shop_for(21, id).unwrap();
    let out = s.handle(&classic_sell(1, 2_070_000, 1));
    assert!(!out[0].what.contains("STALE"), "{}", out[0].what);
    assert_eq!(store.inventory_slot(id, store::InventoryType::Use, 1).unwrap(), None, "the slot is clear");
    assert!(out.iter().any(|r| r.body == net::inventory::inventory_removed(store::InventoryType::Use.as_u8() as i8, 1)));
}

// ---------------------------------------------------------------------------------------
// The Warrior's one HP-costing skill. `Obligation::deduct_hp` had said so since 08-28 and
// `on_attack` never read it - found by the 2026-09-06 audit.
// ---------------------------------------------------------------------------------------

fn stored_hp_mp(store: &Arc<Store>, id: u32) -> (u32, u32) {
    let c = store
        .characters_for(1, 0)
        .unwrap()
        .into_iter()
        .find(|c| c.id == id)
        .expect("the character is in the store");
    (c.hp, c.mp)
}

#[test]
fn slash_blast_costs_hp_as_well_as_mp_and_power_strike_costs_only_mp() {
    let Some((store, mut s, id)) = first_job_with(100, 1_302_000, &[1_001_001, 1_001_002], 0, 0, 1) else {
        return;
    };
    let out = s.handle(&swing_packet(net::combat::USER_MELEE_ATTACK, 1_001_002));
    assert_eq!(stored_hp_mp(&store, id), (47, 196), "Slash Blast level 1: hpCon 3, mpCon 4");
    assert!(
        out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
        "the 0x007C carries the new HP and MP, or the client keeps its own idea of both"
    );
    s.handle(&swing_packet(net::combat::USER_MELEE_ATTACK, 1_001_001));
    assert_eq!(stored_hp_mp(&store, id), (47, 192), "Power Strike has no hpCon; only the MP moved");
}

/// A skill's own cost floors HP at 1 - it never kills its caster, and the log says SHORT.
#[test]
fn a_skills_own_hp_cost_never_kills_its_caster() {
    let Some((store, mut s, id)) = first_job_with(100, 1_302_000, &[1_001_002], 0, 0, 1) else { return };
    let mut c = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
    c.hp = 2;
    store.save_character_progress(&c).unwrap();
    s.handle(&swing_packet(net::combat::USER_MELEE_ATTACK, 1_001_002));
    assert_eq!(stored_hp_mp(&store, id).0, 1, "2 - 3 floors at 1, not 0");
}

fn stored_mp(store: &Arc<Store>, id: u32) -> u32 {
    store
        .characters_for(1, 0)
        .unwrap()
        .into_iter()
        .find(|c| c.id == id)
        .expect("the character is in the store")
        .mp
}

/// **A skill point is charged, and it does not come back on its own.**
///
/// The farming loop this closes: spend three points into Power Strike, relog, and have the
/// three points back **and** keep the skill. `!resetsp`'s own chat line documented it - *"the
/// points come back on their own"* - which is how a loop gets written down instead of fixed.
///
/// Four things, because the Heena lesson is that a test of one effect passes while the others
/// are wrong: the level, the ledger, the balance the client is told, and that `!resetsp`
/// refunds rather than just erasing.
#[test]
fn a_skill_point_is_charged_and_only_a_forget_gives_it_back() {
    let path = std::path::Path::new("../../gm-handbook/skills.txt");
    if !path.exists() {
        return;
    }
    const MAGIC_CLAW: u32 = 2001003;
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Mage".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.job = 200;
    made.level = 30;
    store.save_character_progress(&made).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let config = Config {
        skills: crate::skilltable::SkillTable::load(path),
        ..Config::default()
    };
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(made.id);

    let owed = crate::skillpoints::entitlement(crate::skillpoints::Tier::First, 30);
    assert!(owed >= 3, "level 30 must have earned points for this to test anything");
    let ask = |count: u32| {
        let mut b = net::skills::CLIENT_USER_SKILL_UP_REQUEST.to_le_bytes().to_vec();
        b.extend_from_slice(&0x1187_0e94u32.to_le_bytes());
        b.extend_from_slice(&MAGIC_CLAW.to_le_bytes());
        b.extend_from_slice(&count.to_le_bytes());
        b
    };

    let out = s.handle(&ask(3));
    assert_eq!(store.skill_level(made.id, MAGIC_CLAW).unwrap_or(0), 3, "three levels");
    assert_eq!(store.skill_points_spent(made.id, 1).unwrap(), 3, "and three points charged");
    // The client is told the new balance, or the database is right and the window is wrong.
    let pool = out
        .iter()
        .find(|r| r.opcode == net::stats::STAT_CHANGED && r.what.contains("skill points now"))
        .expect("the pool must be re-sent - the client never decrements one itself");
    assert!(pool.what.contains(&format!("tier 1 = {}", owed - 3)), "{}", pool.what);

    // **The loop it closes**: the points do not return by themselves.
    assert_eq!(store.skill_points_spent(made.id, 1).unwrap(), 3, "still charged");

    // `!resetsp` refunds, in one transaction with the forget.
    let out = s.handle(&gm_chat("!resetsp"));
    assert_eq!(store.skill_points_spent(made.id, 1).unwrap(), 0, "the refund happened");
    assert_eq!(store.skill_level(made.id, MAGIC_CLAW).unwrap_or(0), 0, "and the skill is gone");
    // One line in chat since 2026-09-06 (the owner); the refund count is asserted from the store
    // above and printed to the log, not to the player.
    assert_eq!(notice_text(&out[0]), "Skill Point successfully reset for Mage");
    assert!(
        out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
        "and the refilled pool reaches the screen"
    );
}

/// **Past 30 the first-job pool keeps growing until the book can be maxed, and an SP reset
/// refunds to that total.** The owner, 2026-10-02: *"Allow continuous accumulation of skill
/// points for 1st job beyond level 30 until all skills can be maxed in first job"*, and *"Make
/// sure that SP reset are aware of this change too so resets give characters the correct
/// amount of SP"*.
///
/// A level-50 Magician whose job is already second (210) - the case that matters, because both
/// pools grow. Every effect: tier 1 is 106 (covers the 105-point book) and tier 2 is the classic
/// 61; spending charges tier 1; `!resetsp` (the same function the SP Reset Scroll runs) gives
/// all 106 back. A Thief's book costs 110, so its pool is 112.
#[test]
fn the_first_job_pool_grows_past_thirty_and_a_reset_refunds_to_it() {
    let path = std::path::Path::new("../../gm-handbook/skills.txt");
    if !path.exists() {
        return;
    }
    const MAGIC_CLAW: u32 = 2001003;
    for (job, want_first) in [(210u16, 106u32), (410, 112)] {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        store.set_gm("maplecw", true).unwrap();
        let chr = net::opcode::Character { name: "Grown".to_string(), ..Default::default() };
        let mut made = store.create_character(account_id, 0, &chr).unwrap();
        made.job = job;
        made.level = 50;
        store.save_character_progress(&made).unwrap();
        store.create_migration(account_id, made.id, 0, 0).unwrap();
        let config = Config { skills: crate::skilltable::SkillTable::load(path), ..Config::default() };
        let mut s = Session::new(store.clone(), Arc::new(config));
        s.claim_for_character(made.id);

        let pools = s.skill_point_reply(&made);
        assert!(pools[0].what.contains(&format!("tier 1 = {want_first}")), "{job}: {}", pools[0].what);
        let second = crate::skillpoints::entitlement(crate::skillpoints::Tier::Second, 50);
        assert!(pools[0].what.contains(&format!("tier 2 = {second}")), "{job}: the second pool is unchanged: {}", pools[0].what);
        if job != 210 {
            continue;
        }
        let mut b = net::skills::CLIENT_USER_SKILL_UP_REQUEST.to_le_bytes().to_vec();
        b.extend_from_slice(&0x1187_0e94u32.to_le_bytes());
        b.extend_from_slice(&MAGIC_CLAW.to_le_bytes());
        b.extend_from_slice(&20u32.to_le_bytes());
        let out = s.handle(&b);
        assert_eq!(store.skill_level(made.id, MAGIC_CLAW).unwrap_or(0), 20);
        let pool = out.iter().find(|r| r.what.contains("skill points now")).expect("the pool re-sent");
        assert!(pool.what.contains(&format!("tier 1 = {}", want_first - 20)), "{}", pool.what);

        let out = s.handle(&gm_chat("!resetsp"));
        assert_eq!(store.skill_points_spent(made.id, 1).unwrap(), 0, "refunded");
        let pool = out.iter().find(|r| r.what.contains("skill points now")).expect("the refilled pool");
        assert!(pool.what.contains(&format!("tier 1 = {want_first}")), "the reset gives the grown pool back: {}", pool.what);
    }
}

/// **`!learn` grants levels without spending a point**, which is the whole reason it exists.
///
/// If it charged the pool, `!learn` on a Magician book would want far more points than a
/// level-30 character has earned and the command would silently stop being free - which is why
/// the ledger is a stored counter rather than `SUM(level)`.
#[test]
fn learn_grants_levels_without_charging_the_pool() {
    let path = std::path::Path::new("../../gm-handbook/skills.txt");
    if !path.exists() {
        return;
    }
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Mage".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.job = 200;
    made.level = 30;
    store.save_character_progress(&made).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let config = Config {
        skills: crate::skilltable::SkillTable::load(path),
        ..Config::default()
    };
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(made.id);

    s.handle(&gm_chat("!learn"));
    let levels: u32 = store.skills(made.id).unwrap().iter().map(|s| s.level).sum();
    assert!(levels > 60, "the whole Magician book is more levels than a pool could buy: {levels}");
    assert_eq!(store.skill_points_spent(made.id, 1).unwrap(), 0, "and it charged nothing");
}

/// **Clicking an instructor advances the job**, and every refusal is still a sentence.
///
/// `world::jobs::advancement_for` was correct from the day it was written and had **no caller
/// outside tests** - `!job` skips it deliberately - so the four instructors just said a line.
/// `CLAUDE.md`'s "built is not wired", found by asking what has no caller.
///
/// Four effects are asserted, because this is the shape the Heena quest got wrong: the guard
/// was asked and its answer ignored, because the payout sat outside the match. Here: the
/// database row, the `0x007C`, the sentence, and that a **refusal changes nothing**.
#[test]
fn clicking_an_instructor_advances_the_job() {
    // Dances with Balrog is template 511, one map inside Perion - `jobs::FIRST_JOBS`.
    const BALROG: u32 = 511;
    let build = |level: u32, strength: u16| {
        let mut npcs = std::collections::HashMap::new();
        npcs.insert(
            net::opcode::START_MAP_ID,
            vec![net::opcode::FieldNpc {
                object_id: 1000, template_id: BALROG, x: 0, cy: 0, fh: 1,
                rx0: 0, rx1: 0, f: 0,
            }],
        );
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
        let chr = net::opcode::Character { name: "Rookie".to_string(), ..Default::default() };
        let mut made = store.create_character(account_id, 0, &chr).unwrap();
        made.level = level;
        made.strength = strength;
        made.job = 0;
        store.save_character_progress(&made).unwrap();
        store.create_migration(account_id, made.id, 0, 0).unwrap();
        // The starter sword has to be a real item to `give_item`, which refuses an id the
        // client cannot draw. `Config::default()` knows no names, so name the one gift.
        let mut item_names = std::collections::HashMap::new();
        item_names.insert(1_302_016, "Beginner's Long Sword".to_string());
        let config = Config { npcs, item_names, ..Config::default() };
        let mut s = Session::new(store.clone(), Arc::new(config));
        s.claim_for_character(made.id);
        (s, store, made.id)
    };
    let job_of = |store: &Arc<Store>, id: u32| {
        store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().job
    };
    let equips_of = |store: &Arc<Store>, id: u32| -> Vec<u32> {
        store.bag_items(id, store::InventoryType::Equip).unwrap().into_iter().map(|i| i.item.item_id).collect()
    };

    // ---- eligible: level 10 and STR at the minimum. THE CLICK ASKS; only Yes advances ----
    // The owner, 2026-09-15: "the moment you click on the first job instructors, you simply
    // become that job. There should be a yes or no dialogue (including the requirement)".
    let (mut s, store, id) = build(crate::jobs::LEVEL_MINIMUM, crate::jobs::STAT_MINIMUM);
    let out = s.handle(&npc_click(1000));
    assert_eq!(job_of(&store, id), 0, "the click alone changes nothing");
    assert!(!out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED), "no job packet before an answer");
    let ask = out.iter().find(|r| r.opcode == net::script::SCRIPT_MESSAGE).expect("the question");
    assert!(ask.what.contains("AskYesNo"), "{}", ask.what);
    assert!(ask.what.contains("Swordsman") && ask.what.contains("Level 10") && ask.what.contains("STR 35"), "the requirements are in the box: {}", ask.what);
    assert!(ask.what.contains("cannot be undone"), "{}", ask.what);
    // No: a sentence, nothing else, and the job is untouched. Closing the box: nothing.
    let out = s.on_script_reply(&script_reply(net::script::SCRIPT_ACTION_NO));
    assert_eq!(job_of(&store, id), 0);
    assert!(out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE) && !out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED));
    let _ = s.handle(&npc_click(1000));
    let out = s.on_script_reply(&script_reply(net::script::SCRIPT_ACTION_CLOSED));
    assert_eq!(job_of(&store, id), 0);
    assert!(!out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED));
    assert!(equips_of(&store, id).is_empty(), "no gift before the Yes");
    // Yes: the job persists, the packet goes, the sentence is said.
    let _ = s.handle(&npc_click(1000));
    let out = s.on_script_reply(&script_reply(net::script::SCRIPT_ACTION_YES));
    assert_eq!(job_of(&store, id), 100, "the job must persist, not just be announced");
    let stat = out
        .iter()
        .find(|r| r.opcode == net::stats::STAT_CHANGED)
        .expect("the client must be told, or it draws the old job forever");
    assert!(stat.what.contains("job 0 -> 100"), "{}", stat.what);
    assert!(stat.what.contains("skill points"), "and the SP, or the + button stays grey: {}", stat.what);
    let said = out.iter().find(|r| r.opcode == net::script::SCRIPT_MESSAGE).expect("a sentence, or nothing on screen says what happened");
    // **And the Beginner's set.** The owner, 2026-09-16: the instructors hand out the Beginner's
    // equipment free with the advancement. Dances with Balrog: the Long Sword, in the Equip
    // tab, announced in their sentence and on the grey chat line a quest reward uses.
    assert_eq!(equips_of(&store, id), vec![1_302_016], "the Beginner's Long Sword is in the bag");
    assert!(said.what.contains("Beginner's Long Sword"), "the gift is named: {}", said.what);
    assert!(
        out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION),
        "the Add row that draws it: {:?}",
        out.iter().map(|r| &r.what).collect::<Vec<_>>()
    );
    assert!(
        out.iter().any(|r| r.opcode == net::stats::USER_EFFECT_LOCAL && r.what.contains("1302016")),
        "the grey chat line: {:?}",
        out.iter().map(|r| &r.what).collect::<Vec<_>>()
    );

    // ---- too low a level: a sentence, and NOTHING else ----
    let (mut s, store, id) = build(crate::jobs::LEVEL_MINIMUM - 1, crate::jobs::STAT_MINIMUM);
    let out = s.handle(&npc_click(1000));
    assert_eq!(job_of(&store, id), 0, "a refused advancement must change nothing");
    assert!(
        !out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
        "and must not send a job packet"
    );
    assert!(
        out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE),
        "but it is still ANSWERED - a silent click is the frozen-UI failure"
    );

    // ---- at level, short on the stat: the sentence names the STAT, not the level ----
    let (mut s, store, id) = build(crate::jobs::LEVEL_MINIMUM, 0);
    let out = s.handle(&npc_click(1000));
    assert_eq!(job_of(&store, id), 0);
    assert!(out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE));

    // ---- already advanced: one-way, so this is a refusal rather than a re-offer ----
    let (mut s, store, id) = build(crate::jobs::LEVEL_MINIMUM, crate::jobs::STAT_MINIMUM);
    s.handle(&npc_click(1000));
    s.on_script_reply(&script_reply(net::script::SCRIPT_ACTION_YES));
    assert_eq!(job_of(&store, id), 100);
    let out = s.handle(&npc_click(1000));
    assert!(
        !out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
        "a second click must not re-advance or re-grant SP"
    );
    assert_eq!(job_of(&store, id), 100, "and the job is unchanged");
    assert_eq!(equips_of(&store, id), vec![1_302_016], "and the gift is not handed out twice");
}

/// **Iron Body actually reduces the damage taken.**
///
/// The owner, 2026-08-28: *"Iron Body did not seem to reduce the damage I take."* It could not have.
/// `incoming_damage_for` summed equipment `inc_pdd` and stopped, so a buff that set CTS bit 86,
/// drew its icon and cost MP was invisible to the one calculation it exists to change.
///
/// **This is the third time in this file.** Magic Guard set its bit and moved no HP until the
/// split was written; Nimble Feet's grant was decoded and never wired. Setting the bit buys the
/// icon - the arithmetic is always the server's. So the assertion is on the DAMAGE, not on the
/// packet: a test that checked bit 86 went out would have passed the whole time.
#[test]
fn iron_body_reduces_the_damage_taken() {
    let skills = std::path::Path::new("../../gm-handbook/skills.txt");
    if !skills.exists() {
        return;
    }
    const IRON_BODY: u32 = net::jobbuffs::IRON_BODY;
    // Template 3, so `incoming_damage_for` has a real PADamage to work from.
    let mob_attack: std::collections::HashMap<u32, u32> =
        [(3u32, 40u32)].into_iter().collect();

    let build = |buffed: bool| {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
        let chr = net::opcode::Character { name: "Tank".to_string(), ..Default::default() };
        let mut made = store.create_character(account_id, 0, &chr).unwrap();
        made.job = 100;
        made.mp = 200;
        made.max_mp = 200;
        made.hp = 5000;
        made.max_hp = 5000;
        // **Real STR, because Iron Body's value is a PERCENTAGE of Weapon Def.**
        // `indiePddR` is 25 at level 20, and `iron_body_flat_pdd` resolves it as
        // `wdef * 25 / 100`. A character with no defence gets 25% of nothing - which is a
        // property of the reading, not a bug in the wiring, and it is very likely what the owner
        // saw. Whether the WZ means +25% or +25 flat is [I]; `research/first-job-buffs.md`
        // §3.2 says so and the stat window is the discriminator.
        made.strength = 100;
        store.save_character_progress(&made).unwrap();
        store.set_skill_level(made.id, IRON_BODY, 20).unwrap();
        store.create_migration(account_id, made.id, 0, 0).unwrap();
        let config = Config {
            mob_attack: mob_attack.clone(),
            ..Config::default()
        };
        let mut s = Session::new(store.clone(), Arc::new(config));
        s.claim_for_character(made.id);
        if buffed {
            let out = s.handle(&cast(IRON_BODY, 20));
            assert!(
                out.iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_SET),
                "Iron Body must actually be granted, or this test compares nothing"
            );
        }
        (s, store, made.id)
    };

    // The buff must add weapon defence to the model the incoming hit uses.
    let (bare, _, _) = build(false);
    let (armoured, _, _) = build(true);
    let bare_wdef = bare.held_weapon_defence();
    let armoured_wdef = armoured.held_weapon_defence();
    assert_eq!(bare_wdef, 0, "nothing held, nothing added");
    assert!(
        armoured_wdef > 0,
        "Iron Body must contribute weapon defence, or it cannot reduce anything"
    );

    // And that has to show up as less damage. `incoming_damage` is monotonic in wdef, so
    // comparing the WINDOWS avoids the roll making this flaky.
    let hurt = |wdef: u32| crate::damage::incoming_window(40, 1, wdef);
    let (bare_lo, bare_hi) = hurt(bare_wdef);
    let (buff_lo, buff_hi) = hurt(armoured_wdef);
    assert!(buff_lo <= bare_lo && buff_hi < bare_hi, "buffed {buff_hi} must be under {bare_hi}");
}

/// **An attack skill costs MP, and a potion afterwards does not hand it back.**
///
/// The owner, 2026-08-28: *"Using the Red Potion when my MP is depleted incorrectly recovered my
/// MP?"* It did not. Red Potion's `spec` is `hp 100, mp 0` and the handler added `+0 mp` -
/// `world.log` says so in as many words. The **client** had been spending MP locally on every
/// Power Strike while the **server** never did, so `chr.mp` was still full; the potion's
/// `0x007C` carries the MP field like every stat change, and the client believed it.
///
/// The bug was a stale number, and it surfaced through an unrelated packet. So this test
/// asserts the *whole* shape rather than the deduction alone: the cast spends, the total is
/// what the WZ says, and the potion afterwards reports the LOWERED total. A test of only the
/// first would pass while the symptom the owner saw was untouched.
#[test]
fn an_attack_skill_costs_mp_and_a_potion_does_not_undo_it() {
    let skills = std::path::Path::new("../../gm-handbook/skills.txt");
    if !skills.exists() {
        return; // generated, gitignored - python tools/dump_skills.py
    }
    const POWER_STRIKE: u32 = 1_001_001;

    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Fighter".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.job = 100;
    made.mp = 100;
    made.max_mp = 100;
    store.save_character_progress(&made).unwrap();
    store.set_skill_level(made.id, POWER_STRIKE, 5).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let config = Config {
        firstjob: crate::firstjob::CombatTable::load(skills),
        ..Config::default()
    };
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(made.id);

    // What the client's own data says a level-5 Power Strike costs.
    let cost = s
        .config
        .firstjob
        .level(POWER_STRIKE, 5)
        .and_then(|l| l.mp_con)
        .expect("Power Strike has an mpCon");
    assert!(cost > 0, "a zero cost would make this test vacuous");

    // **A REAL captured swing**, from `research/fixtures/melee-collector-runs-once-per-swing`,
    // with the skill id and level patched into the two fields
    // `research/attack-skill-id.md` measured: the `u32` at body offset 2 and the `u8` at 6.
    // A hand-built header will not do - the parser checks the length and the trailer, and a
    // body it rejects would make this test pass for the wrong reason.
    const SWING: &str = concat!(
        "0000000000000000000000000000000000050000009fae34080104000000e6a81f08f7018b0100000000f7018b010000",
        "0000000000000000000000000000000000000000000000000100000001000000000a0055736572204d656c6565890100",
        "000000000000000000000000000000000000000000000000000080e8da8f00",
    );
    let mut payload: Vec<u8> = {
        let hex: String = SWING.chars().filter(|c| !c.is_whitespace()).collect();
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect()
    };
    payload[2..6].copy_from_slice(&POWER_STRIKE.to_le_bytes());
    payload[6] = 5;
    // It must parse, or the deduction below is testing the early return.
    assert!(
        net::attack::parse(net::combat::USER_MELEE_ATTACK, &payload).is_ok(),
        "the captured body must still parse after patching"
    );
    let mut body = net::combat::USER_MELEE_ATTACK.to_le_bytes().to_vec();
    body.extend_from_slice(&payload);

    let out = s.handle(&body);
    let mp_now = stored_mp(&store, made.id);
    assert_eq!(mp_now, 100 - cost, "the cast must spend exactly mpCon");
    assert!(
        out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
        "and the client must be told, or its own count and ours drift again"
    );

    // **The symptom, not just the cause.** A Red Potion is hp 100 / mp 0; the stat change it
    // sends must carry the LOWERED MP, not the total the server used to be holding.
    store
        .add_item(made.id, store::InventoryType::Use, &store::Item::bundle(2_000_000, 1), 200)
        .unwrap();
    let before = stored_mp(&store, made.id);
    let out = s.on_use_item(&net::useitem::use_item(0, 1, 2_000_000, 1));
    let after = stored_mp(&store, made.id);
    assert_eq!(after, before, "a Red Potion restores no MP: its spec is hp 100, mp 0");
    for r in out.iter().filter(|r| r.opcode == net::stats::STAT_CHANGED) {
        assert!(
            !r.what.contains(&format!("{}/{}", 100, 100)),
            "the potion must not report full MP: {}",
            r.what
        );
    }
}

/// The Use-tab slot holding item 2000000 after `shop_session` bought some.
fn potion_slot(store: &Arc<Store>, id: u32) -> u16 {
    store.bag(id).unwrap().items_in(store::InventoryType::Use).next().expect("potions in the Use tab").slot
}

/// **A sale clicked twice before the list redrew sells once, and the second click redraws it.**
/// The owner, 2026-10-02: *"When users sell to shop too fast, sometimes their view does not refresh
/// fast enough and they try to sell the same thing again to which the server refuses ... can we
/// automatically refresh their view"*. The second sell names a slot that is now empty: nothing
/// is sold, nothing is paid, the latch is cleared with the SILENT type 16 (not "not enough
/// mesos"), and a REMOVE for that slot goes back so the Sell list redraws. Every effect named.
#[test]
fn a_second_sell_of_an_emptied_slot_sells_nothing_and_redraws_the_slot() {
    let (mut s, store, id) = shop_session();
    store.set_mesos(id, 1000).unwrap();
    s.handle(&npc_click(1000));
    s.handle(&classic_buy(0, 2000000, 3));
    let slot = potion_slot(&store, id);
    s.handle(&classic_sell(slot, 2000000, 3));
    let paid_once = store.mesos(id).unwrap();

    let out = s.handle(&classic_sell(slot, 2000000, 3));
    assert_eq!(out[0].opcode, net::classicshop::CLASSIC_SHOP_RESULT);
    assert_eq!(out[0].body, vec![net::classicshop::RESULT_ACKNOWLEDGED], "silent, not a refusal message: {}", out[0].what);
    assert!(out[0].what.contains("STALE"), "{}", out[0].what);
    assert_eq!(out.len(), 2, "the result and the slot, nothing else - no mesos: {out:?}");
    assert_eq!(out[1].body, net::inventory::inventory_removed(store::InventoryType::Use.as_u8() as i8, slot as i16));
    assert_eq!(store.mesos(id).unwrap(), paid_once, "paid once");
}

/// **A stale sell of a slot that now holds something else** - another item, or fewer than the
/// click asked for - sells nothing and sends the slot's real contents.
#[test]
fn a_stale_sell_of_a_changed_slot_sells_nothing_and_sends_what_is_there() {
    let (mut s, store, id) = shop_session();
    store.set_mesos(id, 1000).unwrap();
    s.handle(&npc_click(1000));
    s.handle(&classic_buy(0, 2000000, 3));
    let slot = potion_slot(&store, id);
    let use_tab = store::InventoryType::Use.as_u8() as i8;

    // Fewer than asked: 3 held, 5 asked.
    let out = s.handle(&classic_sell(slot, 2000000, 5));
    assert_eq!(out[0].body, vec![net::classicshop::RESULT_ACKNOWLEDGED], "{}", out[0].what);
    assert_eq!(out[1].body, net::inventory::inventory_quantity(use_tab, slot as i16, 3), "the real count");
    // Another item: the client thinks the slot holds 2000001.
    let out = s.handle(&classic_sell(slot, 2000001, 1));
    assert_eq!(out[0].body, vec![net::classicshop::RESULT_ACKNOWLEDGED], "{}", out[0].what);
    assert_eq!(use_slot(&store, id, slot), 3, "nothing left the bag");
    assert_eq!(store.mesos(id).unwrap(), 1000 - 3 * 50, "and nothing was paid");
}

/// **Selling part of a stack leaves the rest on screen.** Until 2026-10-02 every sale sent a REMOVE,
/// so selling 1 of 3 potions emptied the slot on screen while the server kept 2.
#[test]
fn selling_part_of_a_stack_sends_the_count_left_not_a_remove() {
    let (mut s, store, id) = shop_session();
    store.set_mesos(id, 1000).unwrap();
    s.handle(&npc_click(1000));
    s.handle(&classic_buy(0, 2000000, 3));
    let slot = potion_slot(&store, id);
    let out = s.handle(&classic_sell(slot, 2000000, 1));
    assert_eq!(out[0].body[0], net::classicshop::RESULT_ACKNOWLEDGED);
    let use_tab = store::InventoryType::Use.as_u8() as i8;
    assert!(out.iter().any(|r| r.body == net::inventory::inventory_quantity(use_tab, slot as i16, 2)), "2 left: {out:?}");
    assert!(!out.iter().any(|r| r.body == net::inventory::inventory_removed(use_tab, slot as i16)), "no REMOVE");
    assert_eq!(use_slot(&store, id, slot), 2);
}

/// **Selling works, and sends NO buy-back row and NO type-10 refresh.**
///
/// The owner, 2026-08-28: *"Selling an item to Lucy crashed the client."* The sale itself was fine -
/// `world.log` shows the success, the inventory remove and the meso change all going out and
/// being accepted. What killed it was the `0x055E` **type 10** that followed, and the client
/// handed the packet straight back in a 2075-byte `0x009E` before it died.
///
/// **`UI/UIShop.img/Shop` has 16 nodes and `repurchaseInfo` is not one of them** - read with
/// `wz-dump`, with `BtBuy` as the positive control - and it carries exactly `TabBuy` and
/// `TabSell`. Type 10 *selects the Buy Back tab*, so it reached for art this client does not
/// ship. Same failure as Shop2, same fault address, one level down.
///
/// This test is the inverted form of the one it replaces, which asserted the tab filled. That
/// test passed while the feature killed the client, because it checked our bytes rather than
/// what the client could draw with them.
#[test]
fn selling_never_sends_a_buy_back_row_or_a_list_refresh() {
    let (mut s, store, id) = shop_session();
    store.set_mesos(id, 1000).unwrap();
    s.handle(&npc_click(1000));
    s.handle(&classic_buy(0, 2000000, 3));

    let slot = store
        .bag(id)
        .unwrap()
        .items_in(store::InventoryType::Use)
        .next()
        .expect("three potions are in the Use tab")
        .slot;
    let before = store.mesos(id).unwrap();

    let out = s.handle(&classic_sell(slot, 2000000, 3));

    // The sale still happens, and all three of its effects go out.
    assert_eq!(out[0].body[0], net::classicshop::RESULT_ACKNOWLEDGED, "a sale must not re-select the last purchase's tab: {}", out[0].what);
    assert!(out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "the bag");
    assert!(out.iter().any(|r| r.opcode == net::combat::STAT_CHANGED), "the meso count");
    assert!(store.mesos(id).unwrap() > before, "selling pays");

    // **And nothing selects a tab that has no art.**
    assert!(
        !out.iter().any(|r| r.opcode == net::classicshop::CLASSIC_SHOP_RESULT
            && r.body[0] == net::classicshop::RESULT_REFRESH_LIST),
        "type 10 selects the Buy Back tab, whose art this client does not have"
    );

    // Re-opening the counter must not smuggle a buy-back row in either.
    let out = s.handle(&npc_click(1000));
    let body = &out[0].body;
    let rows = u16::from_le_bytes([body[19], body[20]]) as usize;
    for i in 0..rows {
        let at = net::classicshop::CLASSIC_HEAD_LEN + i * net::classicshop::CLASSIC_ROW_LEN;
        assert_eq!(
            body[at + net::classicshop::CLASSIC_ROW_LEN - 1],
            0,
            "row {i} carries the Buy Back flag"
        );
    }
    // The length identity proves it independently: a buy-back row is 158, not 157.
    assert_eq!(
        body.len(),
        net::classicshop::CLASSIC_HEAD_LEN + rows * net::classicshop::CLASSIC_ROW_LEN,
        "every row must be the ordinary 157 bytes"
    );
}

/// **Every `0x00F5` arm is answered except Close, which latches nothing.**
///
/// `shopUI+0x4b0` latches when the window sends a request and only a result clears it. An
/// unanswered buy leaves the counter alive but every further click a silent no-op - the shop
/// reads as half-working, and another `0x055D` will not fix it because of the modal guard.
#[test]
fn every_classic_shop_request_is_answered_except_close() {
    let (mut s, _, _) = shop_session();
    s.handle(&npc_click(1000));

    // A body with no sub-op byte at all, a nonsense sub-op, and a recharge nothing can reach.
    for body in [
        vec![],
        vec![0xEE],
        vec![2, 0, 0],
        vec![0], // a buy with no fields
    ] {
        let mut packet = net::classicshop::CLIENT_CLASSIC_SHOP_REQUEST.to_le_bytes().to_vec();
        packet.extend_from_slice(&body);
        let out = s.handle(&packet);
        assert!(
            out.iter().any(|r| r.opcode == net::classicshop::CLASSIC_SHOP_RESULT),
            "body {body:02x?} must still clear the latch"
        );
    }

    // Close is the exception: nothing is latched, and the state goes.
    let mut packet = net::classicshop::CLIENT_CLASSIC_SHOP_REQUEST.to_le_bytes().to_vec();
    packet.push(3);
    let out = s.handle(&packet);
    assert!(out.is_empty(), "close owes nothing: {out:?}");

    // With the counter closed, a buy is refused rather than acted on.
    let out = s.handle(&classic_buy(0, 2000000, 1));
    assert!(out[0].what.contains("no shop is open"), "{}", out[0].what);
}

/// **Every one of the 39 authored shops builds a packet whose length is exactly accounted
/// for**, against the real `data/shops.txt`.
///
/// This is the check that a client run cannot cheaply give. The classic counter parses rows in
/// a loop, so a row one byte wide in either direction desynchronises everything after it -
/// `research/classic-shop-rows.md` §4 calls that the one failure here that crashes rather than
/// disappoints. Deriving the expected length from the row *contents* rather than from the
/// writer's cursor is what makes this a check and not a restatement.
///
/// Flora the Fairy is asserted by name: they are the NPC the owner clicked on 2026-08-28, and the
/// reason they got a dialogue box was that shops were off, not that their join was missing.
#[test]
fn every_authored_shop_builds_a_length_correct_packet() {
    let shops = std::path::Path::new("../../data/shops.txt");
    let items = std::path::Path::new("../../gm-handbook/items.txt");
    let strings = std::path::Path::new("../../gm-handbook/npcstrings.txt");
    let itemdata = std::path::Path::new("../../gm-handbook/itemdata.txt");
    if !shops.exists() || !items.exists() || !strings.exists() {
        return; // two of the three are generated and gitignored
    }
    let table = crate::shops::ShopTable::load(shops, items, itemdata);
    let npc_strings = crate::config::load_npc_strings(strings);
    let (by_template, _) = crate::shops::resolve_npc_templates(&table, &npc_strings);
    assert!(by_template.len() >= 39, "only {} shops resolved", by_template.len());

    let mut flora = false;
    for (&template, &index) in &by_template {
        let shop = &table.shops[index];
        let rows: Vec<net::classicshop::ClassicShopRow> = shop
            .items
            .iter()
            .filter(|i| i.buy_price > 0)
            .map(|i| {
                net::classicshop::ClassicShopRow::buy(
                    i.item_id,
                    u64::from(i.buy_price),
                    i16::try_from(table.max_per_purchase(i.item_id)).unwrap_or(100),
                )
            })
            .collect();
        if rows.is_empty() {
            continue;
        }
        let body = net::classicshop::classic_open_shop(template, &rows);
        let want: usize =
            net::classicshop::CLASSIC_HEAD_LEN + rows.iter().map(|r| r.wire_len()).sum::<usize>();
        assert_eq!(body.len(), want, "{} ({}) row width", shop.npc, shop.role);
        assert_eq!(
            u16::from_le_bytes([body[19], body[20]]) as usize,
            rows.len(),
            "{}: rowCount must match what was written",
            shop.npc
        );
        // Every row's price must have survived as a non-zero u64, and every cap must be
        // non-zero - the two silent killers.
        // **Walk by each row's own width, not by a fixed stride.** The stride version is why
        // the rechargeable-row bug survived this test: it asserted our packet against our own
        // constant, so a row the CLIENT reads as 6 bytes wider than we wrote it looked
        // perfectly self-consistent. `wire_len()` is the same function the builder uses, so
        // this still cannot catch a wrong constant - but it does catch the offsets sliding,
        // which is what actually killed the client.
        let mut at = net::classicshop::CLASSIC_HEAD_LEN;
        for (i, row) in rows.iter().enumerate() {
            let price = u64::from_le_bytes(body[at + 36..at + 44].try_into().unwrap());
            assert_eq!(price, row.price, "{}: row {i} price", shop.npc);
            assert_ne!(price, 0, "{}: row {i} is free, which files it in the Sell tab", shop.npc);
            // The cap sits four bytes from the END of the row, so it is measured against
            // THIS row's width - a rechargeable row is longer than the constant.
            let cap = i16::from_le_bytes(
                body[at + row.wire_len() - 4..at + row.wire_len() - 2].try_into().unwrap(),
            );
            assert!(cap > 0, "{}: row {i} cap is 0 - every purchase would fail silently", shop.npc);
            at += row.wire_len();
        }
        if shop.npc.contains("Flora") {
            flora = true;
            assert!(!rows.is_empty(), "Flora the Fairy must actually stock something");
        }
    }
    assert!(flora, "Flora the Fairy is the NPC that started this; they must be in the join");
}

/// **The join, against the real data.** All 39 authored shops must resolve to a template,
/// and Lucy must be 21 - the id the owner read off their own screen.
#[test]
fn every_authored_shop_resolves_to_an_npc_template() {
    let shops = std::path::Path::new("../../data/shops.txt");
    let strings = std::path::Path::new("../../gm-handbook/npcstrings.txt");
    if !shops.exists() || !strings.exists() {
        return; // npcstrings is generated and gitignored
    }
    let table = crate::shops::ShopTable::load(
        shops,
        std::path::Path::new("../../gm-handbook/items.txt"),
        std::path::Path::new("../../gm-handbook/itemdata.txt"),
    );
    let npc_strings = crate::config::load_npc_strings(strings);
    let (by_template, problems) = crate::shops::resolve_npc_templates(&table, &npc_strings);

    assert_eq!(table.shops.len(), 39, "the authored shop count");
    assert_eq!(by_template.get(&21).copied(), table.shops.iter().position(|s| s.npc == "Lucy"));
    assert!(
        !problems.iter().any(|p| p.contains("can never open")),
        "every shop must resolve: {problems:?}"
    );
}

/// The channel must not send the login server's startup gate. That packet is what a
/// login connection needs and what a game connection almost certainly rejected.
#[test]
fn a_channel_says_nothing_on_connect() {
    let (mut s, _, _, _) = session();
    assert!(s.on_connect().is_empty());
}

#[test]
fn a_pending_migration_is_claimed_once_and_remembered() {
    let (mut s, store, account_id, id) = session();
    store.create_migration(account_id, id, 0, 0).unwrap();

    let note = s.claim_for_character(id);
    assert!(note.contains("claimed the migration"), "{note}");
    assert!(!note.contains("WRONG CHANNEL"), "{note}");
    assert_eq!(s.claimed().unwrap().character_id, id);

    let mut other = Session::new(store, Arc::new(Config::default()));
    assert!(other.claim_for_character(id).contains("no unconsumed migration"));
}

#[test]
fn a_migration_for_another_channel_is_reported_rather_than_silently_accepted() {
    let (_, store, account_id, id) = session();
    store.create_migration(account_id, id, 0, 7).unwrap();
    let config = Config { channel_id: 0, ..Config::default() };
    let mut s = Session::new(store, Arc::new(config));
    assert!(s.claim_for_character(id).contains("WRONG CHANNEL"));
}

#[test]
fn a_character_with_no_migration_says_so_instead_of_failing() {
    let (mut s, _, _, _) = session();
    assert!(s.claim_for_character(999).contains("no unconsumed migration"));
}

/// The real 34 bytes the client sent when the owner walked into map 1's right-hand portal,
/// copied out of `research/fixtures/character-on-map1-playable-world.log`. A parser
/// tested against invented bytes proves only that it agrees with itself.
#[test]
fn the_captured_portal_request_parses() {
    let body = hex("64000000ad1500000000000000000000ffffffff05006f7574303053046d01000000");
    assert_eq!(body.len(), 34, "the capture is 34 bytes");

    let r = parse_transfer_field(&body).expect("the captured body parses");
    assert_eq!(r.target_field, None, "0xFFFFFFFF means 'resolve the portal name'");
    assert_eq!(r.portal_name, "out00", "map 1's portal 4, from the WZ");
    assert_eq!(r.position, Some((1107, 365)), "y is exactly the portal's own y");

}

/// The real `0x0151` the owner's client sent when they clicked Heena on map 1, from
/// `research/fixtures/npcs-visible-quests-clicked-world.log`. A parser tested against
/// invented bytes proves only that it agrees with itself.
///
/// This is also the test that pins the retraction: field 1 is a **quest id**, not the
/// object id we assigned. Every map's first NPC gets object id 1000 and the client
/// answered 1000/1002/1003/1005 for four NPCs in the same order across two sessions
/// with opposite visit orders, so it cannot be reading our numbering back.
#[test]
fn a_clicked_npc_is_answered_with_something_to_say() {
    let body = hex("01e8030000010000000c046d0100000000");
    assert_eq!(body.len(), 17, "the capture is 17 bytes");
    let req = net::script::parse_quest_request(&body).expect("the captured body parses");
    assert_eq!(req.action, 1);
    assert_eq!(req.quest_id, 1000, "field 1 is a quest id, not our object id");
    assert_eq!(req.npc_template_id, 1, "field 2 is the template we sent in 0x044F");

    let (mut s, store, account_id, id) = session();
    store.create_migration(account_id, id, 0, 0).unwrap();
    s.claim_for_character(id);

    let replies = s.on_quest_request(&body);
    assert!(!replies.is_empty(), "an unanswered request freezes the client's whole UI");
    // Action 1 is Accept, so this capture also records the quest - see
    // `Session::record_quest_start`. The Say is selected by opcode, not by position.
    assert!(
        replies.iter().any(|r| r.opcode == net::quest::MESSAGE),
        "an Accept must write the quest down as well as answer it"
    );

    // The speaker must be the template the client named. It is by construction a real
    // Npc.wz id, which is what keeps this safe - 0 is not one.
    let said = &script_message(&replies).body;
    assert_eq!(
        u32::from_le_bytes(said[5..9].try_into().unwrap()),
        req.npc_template_id
    );
    assert_ne!(req.npc_template_id, 0);

    // Type 0 is Say. Anything else indexes a different entry of the 71-entry table and
    // reads a different body, and there is no resync point.
    assert_eq!(said[10], net::script::SCRIPT_TYPE_SAY);

    // A short body must not panic - these come off a socket.
    for n in 0..body.len() {
        let _ = s.on_quest_request(&body[..n]);
    }
}

/// The real 12 bytes the owner's client sent when they clicked Robin on map 40, from
/// `research/fixtures/dressed-in-world-npc-click-00f2-world.log`. Answering only
/// `0x0151` left every quest-less NPC silent, which is what that run measured.
#[test]
fn clicking_a_questless_npc_is_answered_as_its_template_not_its_object_id() {
    let body = hex("e803000001001301ffffffff");
    let click = net::script::parse_npc_click(&body).expect("the captured body parses");
    assert_eq!(click.npc_object_id, 1000, "the object id WE assigned, [npc+0x190]");
    assert_eq!((click.char_x, click.char_y), (1, 275), "the CHARACTER's position");

    // Map 40's NPCs, exactly as gm-handbook/npcs.txt has them: object ids 1000 and 1001
    // for templates 8 and 9. The lookup has to invert the numbering config::load_npcs
    // does, and it is per-map because that numbering restarts on every field.
    let npcs = vec![
        net::opcode::FieldNpc {
            object_id: 1000, template_id: 8, x: 69, cy: 275, fh: 30,
            rx0: 19, rx1: 119, f: 0,
        },
        net::opcode::FieldNpc {
            object_id: 1001, template_id: 9, x: 1602, cy: 215, fh: 59,
            rx0: 1552, rx1: 1652, f: 0,
        },
    ];
    let config = Config {
        npcs: [(40u32, npcs)].into_iter().collect(),
        ..Config::default()
    };

    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character {
        name: "TestCharD".to_string(), map_id: 40, ..Default::default()
    };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);

    let replies = s.on_npc_click(&body);
    assert_eq!(replies.len(), 1);
    assert_eq!(replies[0].opcode, net::script::SCRIPT_MESSAGE);

    // The speaker must be the TEMPLATE (8), never the object id (1000). An object id in
    // that field does not fault - the loader result is null-checked - it just draws a
    // box with no portrait, which is the kind of failure a run cannot explain.
    let said = &replies[0].body;
    assert_eq!(u32::from_le_bytes(said[5..9].try_into().unwrap()), 8);
    assert_ne!(u32::from_le_bytes(said[5..9].try_into().unwrap()), 1000);
    assert_eq!(said[10], net::script::SCRIPT_TYPE_SAY);

    // An object id that is not on this map has no template to speak as. Answering with
    // a made-up one buys nothing, and this request does not block: the capture shows
    // the UI stayed live with 0x00F2 unanswered.
    let mut unknown = body.clone();
    unknown[0] = 0xFF;
    assert!(s.on_npc_click(&unknown).is_empty());

    // Short bodies come off a socket and must not panic.
    for n in 0..body.len() {
        let _ = s.on_npc_click(&body[..n]);
    }
}

/// Accepting a quest answers with the **yes branch**, not the opening again.
///
/// The owner, 2026-08-19: *"Clicking 'Accept' starts the 'You must be the new traveler'
/// conversation again. That portion is incorrect, as [it] exists and gets handled on
/// client side."* By the time `0x0151` arrives the client has already shown the opening
/// and the user has pressed a button; action 1 is that press.
#[test]
fn accepting_a_quest_answers_with_the_yes_branch() {
    let path = std::path::Path::new("../../gm-handbook/questlines.txt");
    if !path.exists() {
        return; // generated data, gitignored
    }
    let config = Config { quests: crate::config::load_quests(path), ..Config::default() };
    let want = config.quests[&1000].say["0.yes"][0].clone();
    assert!(want.contains("hill to the east"), "{want}");

    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);

    // The real 0x0151 the owner's client sent on pressing Accept: action 1, quest 1000.
    let replies = s.on_quest_request(&hex("01e8030000010000000c046d0100000000"));
    // Two things now: the quest record, then the Say. The record is FIRST so the journal
    // is right before the NPC's follow-up line is drawn.
    assert_eq!(replies.len(), 2, "the quest record and the yes branch");
    assert_eq!(replies[0].opcode, net::quest::MESSAGE, "the record leads");
    assert_eq!(replies[0].body, net::quest::quest_accepted(1000));

    // **And it is in the database**, which is the half that was missing until
    // 2026-08-20: the dialogue was always right and nothing was written down.
    let rows = s.store.quest_rows(id).unwrap();
    assert_eq!(rows.len(), 1, "the accept reached quest_state");
    assert_eq!(rows[0].quest_id, 1000);

    let (message_type, text, _) = script_text(&script_message(&replies).body);
    assert_eq!(text, want, "Accept must answer with the yes branch");

    // And it must be a plain Say, NOT another Accept/Decline prompt - the user has
    // already answered, and asking again is the loop the owner hit.
    assert_eq!(message_type, net::script::SCRIPT_TYPE_SAY);

    // The branch is one line, so OK ends the conversation rather than repeating it.
    let done = s.on_script_reply(&reply_bytes(&text, message_type, 1));
    assert!(done.is_empty(), "the conversation must end, not loop");
}

/// **Roger's quest opens**, from the authored script overlay rather than their idle line.
///
/// The owner, 2026-08-21: *"Roger's Apple quest doesn't start as expected. All I see is 'Hey,
/// nice weather isn't it', which is just their normal text instead of the quest text."*
///
/// Quest 1002 opens with `Check.0.startscript q1002s` and has **no `Say."0"`** - and the
/// script body is not in the client at all (all 205 archives and 10021 images enumerated;
/// `research/quest-scripts.md`). So the opening is authored in `data/quest-scripts.txt` and
/// merged over the generated table.
///
/// This drives the **exact 13-byte body from `world.log` at 06:10:04.854** - action 4,
/// quest 1002, NPC 3 - and asserts the two things that were wrong on screen: that the reply
/// is the quest's opening rather than the fall-through line, and that the last line of it
/// carries an Accept/Decline box rather than a plain OK.
#[test]
fn rogers_script_quest_opens_from_the_authored_overlay() {
    let generated = std::path::Path::new("../../gm-handbook/questlines.txt");
    let authored = std::path::Path::new("../../data/quest-scripts.txt");
    if !generated.exists() {
        return; // generated data, gitignored
    }
    let mut quests = crate::config::load_quests(generated);
    assert!(
        !quests[&1002].say.contains_key("0"),
        "the client ships no opening for 1002 - if it ever does, this overlay is redundant"
    );
    // **Assert what this test is about, not how many quests the file happens to hold.**
    // The first version pinned the return at 1 and broke the moment the job-advancement
    // quests were authored into the same file - a test failing on someone else's correct
    // work. The overlay's size is content and will keep growing; what must hold is that
    // 1002 got an opening out of it.
    assert!(crate::config::overlay_quests(&mut quests, authored) >= 1, "{authored:?} loaded");
    assert!(
        quests[&1002].say.contains_key("0"),
        "the overlay did not reach quest 1002 - check {authored:?} and its TAB characters"
    );

    let config = Config { quests, ..Config::default() };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Roger".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);

    // The capture, byte for byte: 04 ea030000 03000000 44ff 1301
    let replies = s.on_quest_request(&hex("04ea0300000300000044ff1301"));
    let (ty, first, _) = script_text(&script_message(&replies).body);
    assert!(
        first.starts_with("You'll die when your HP reaches 0"),
        "the overlay's opening, not the fall-through. got: {first}"
    );
    assert!(
        !first.contains("nice weather"),
        "'Hey, nice weather, isn't it?' is String.wz/Npc.img/3/d0 - the exact signature of          the overlay not having loaded"
    );
    assert_eq!(ty, net::script::SCRIPT_TYPE_SAY, "the first of two lines pages normally");

    // Page to the last line. It must become the quest Accept/Decline box, because a plain
    // Say draws OK and there is then nothing for the player to accept.
    let next = s.on_script_reply(&reply_bytes(&first, ty, 1));
    let (last_ty, last, _) = script_text(&script_message(&next).body);
    assert_eq!(last, "Want to see how that works? I'll give you one of my apples to try it with. Shall we?");
    assert_eq!(
        last_ty,
        net::script::SCRIPT_TYPE_QUEST_YES_NO,
        "the last line of a branchable conversation is a 0x10 box, not a Say"
    );
}

/// A multi-line branch still pages, so the machine is not special-cased to one line.
#[test]
fn a_multi_line_path_still_pages_in_order() {
    let mut quests = std::collections::HashMap::new();
    quests.insert(
        42u32,
        crate::config::Quest {
            name: "Test".into(),
            say: [("0.yes".to_string(), vec!["one".to_string(), "two".to_string()])]
                .into_iter()
                .collect(),
            ..Default::default()
        },
    );
    let config = Config { quests, ..Config::default() };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);

    let mut body = vec![1u8]; // action 1, accept
    body.extend_from_slice(&42u32.to_le_bytes());
    body.extend_from_slice(&1u32.to_le_bytes());
    let replies = s.on_quest_request(&body);
    let said = script_message(&replies);
    let (ty, text, after) = script_text(&said.body);
    assert_eq!(text, "one");
    assert_eq!(said.body[after + 1], 1, "next must be set - there is a line 2");

    let next = s.on_script_reply(&reply_bytes(&text, ty, 1));
    assert_eq!(script_text(&script_message(&next).body).1, "two");
}


/// A 0x00F3 whose action is -1 - the user closed the box - ends the conversation and
/// sends nothing. An unanswered 0x00F3 costs only a dead conversation: it is not one of
/// the 37 setters of the player->[0x2330] latch.
#[test]
fn closing_a_box_ends_the_conversation_silently() {
    let path = std::path::Path::new("../../gm-handbook/questlines.txt");
    if !path.exists() {
        return;
    }
    let config = Config { quests: crate::config::load_quests(path), ..Config::default() };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);
    s.on_quest_request(&hex("01e8030000010000000c046d0100000000"));

    assert!(s.on_script_reply(&reply_bytes("anything", 0, -1i8 as u8)).is_empty());
    // And a second reply with no conversation open must not panic or answer.
    assert!(s.on_script_reply(&reply_bytes("anything", 0, 1)).is_empty());
    // Short bodies come off a socket.
    for n in 0..12 {
        let _ = s.on_script_reply(&vec![0u8; n]);
    }
}

/// **Turning a quest in plays the fanfare**, and failing to turn one in does not.
///
/// The owner, 2026-08-21: *"Quest finish still does not trigger the SFX for quest finish (this is
/// a different SFX than quest completion)."* Nothing sent one. `0x02D1` effect 15 is
/// `QuestClear`, pinned to `Sound/Game.img/QuestClear` through its only two readers -
/// `research/quest-complete-effect.md`.
///
/// The negative half is the point of the test: a "completion" of a quest the character never
/// started must stay silent, or the sound stops meaning anything.
#[test]
fn completing_a_quest_plays_the_clear_fanfare_and_a_non_completion_does_not() {
    let path = std::path::Path::new("../../gm-handbook/questlines.txt");
    if !path.exists() {
        return; // generated data, gitignored
    }
    let config = Config { quests: crate::config::load_quests(path), ..Config::default() };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);

    // **Filter on the EFFECT byte, not the opcode.** `0x02D1` used to carry only the
    // QuestClear fanfare; since 2026-08-21 it also carries effect 8, the item line that puts
    // a quest reward in the chat log. Counting by opcode made this test fail the moment a
    // second effect started sharing it - which is the test doing its job, and the fix is to
    // ask the question the test is actually about.
    let fanfares = |rs: &[Reply]| -> Vec<Vec<u8>> {
        rs.iter()
            .filter(|r| r.opcode == net::questeffect::USER_EFFECT_LOCAL)
            .filter(|r| r.body.first() == Some(&net::questeffect::EFFECT_QUEST_CLEAR))
            .map(|r| r.body.clone())
            .collect()
    };

    // Never started, so "completing" it records nothing. The client is still answered - the
    // always-answer rule - but there must be no fanfare.
    let unstarted = s.on_quest_request(&hex("02e80300000200000043ffe501ffffffff"));
    assert!(!unstarted.is_empty(), "every 0x0151 is answered");
    assert!(fanfares(&unstarted).is_empty(), "no row was completed, so no sound");

    // Now start it and turn it in for real.
    s.on_quest_request(&hex("01e8030000010000000c046d0100000000"));
    let done = s.on_quest_request(&hex("02e80300000200000043ffe501ffffffff"));
    let played = fanfares(&done);
    assert_eq!(played.len(), 1, "exactly one fanfare per turn-in");
    assert_eq!(played[0], vec![net::questeffect::EFFECT_QUEST_CLEAR], "one body byte, 0x0F");

    // It lands AFTER the journal row and after the rewards, so the sound plays on a book
    // that already reads complete.
    let record = done.iter().position(|r| r.opcode == net::quest::MESSAGE).expect("the row");
    let effect = done
        .iter()
        .position(|r| {
            r.opcode == net::questeffect::USER_EFFECT_LOCAL
                && r.body.first() == Some(&net::questeffect::EFFECT_QUEST_CLEAR)
        })
        .expect("the fanfare");
    assert!(effect > record, "the fanfare must not precede the journal row");
}

/// Equipping over a worn item swaps, and the reply stays **one entry, 14 bytes**.
///
/// Mode 2 in `FUN_142d51930` is an unconditional two-way exchange - `142d52c13` writes the
/// displaced item into `oldPos` on its own. So the swap needs no second entry, and adding
/// one would move the item that had just arrived. `research/equip-crash.md`.
///
/// The byte-length assertion is the load-bearing half. This project has shipped a short
/// packet twice and killed the client both times, and the fatal equip and a known-good one
/// differ by exactly two bytes - `newPos` - so length is the thing worth pinning.
#[test]
fn equipping_over_a_worn_item_swaps_and_still_sends_one_entry() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character {
        name: "Swapper".to_string(),
        equips: vec![(5, 1040002)],
        ..Default::default()
    };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    store
        .set_inventory_slot(id, store::InventoryType::Equip, 4, &store::Item::equip(1040001))
        .unwrap();

    let mut s = Session::new(store, Arc::new(Config::default()));
    s.claim_for_character(id);

    // invType 1, src 4, dst -5, count -1 - the shape of the real 0x0107.
    let mut body = vec![0u8; 4];
    body.push(1);
    body.extend_from_slice(&4i16.to_le_bytes());
    body.extend_from_slice(&(-5i16).to_le_bytes());
    body.extend_from_slice(&(-1i16).to_le_bytes());
    let replies = s.on_inventory_move(&body);

    let op = replies
        .iter()
        .find(|r| r.opcode == net::inventory::INVENTORY_OPERATION)
        .expect("every 0x0107 is answered with a 0x0070, including the refusals");
    assert_eq!(
        op.body.len(),
        net::inventory::INVENTORY_MOVE_RESULT_LEN,
        "one entry plus the avatarChanged tail - a swap adds no bytes"
    );
    assert_eq!(op.body, net::inventory::inventory_move_result(1, 4, -5));
    assert!(op.what.contains("SWAPPING"), "the log line names both halves: {}", op.what);

    // The database mirrors 142d52c13: the coat that came off is in the slot the new one
    // vacated, and the new one is worn.
    let worn = s.store.equipped_items(id).unwrap();
    assert_eq!(worn.iter().find(|e| e.slot == 5).unwrap().item_id, 1040001);
    let back = s.store.inventory_slot(id, store::InventoryType::Equip, 4).unwrap().unwrap();
    assert_eq!(back.item_id, 1040002);
}

/// A claimed session whose character has `ap` unspent ability points.
///
/// The starter stats are 4/4/4/4, so a single point into STR must read back as 5.
fn session_with_ap(ap: u16) -> (Session, i64, u32) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Spender".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.ap = ap;
    store.save_character_progress(&made).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(Config::default()));
    s.claim_for_character(made.id);
    (s, account_id, made.id)
}

/// Read a character back out of the store, so a test asserts on what was persisted rather
/// than on what the handler happened to hold.
fn reload(s: &Session, account: i64, id: u32) -> net::opcode::Character {
    s.store
        .characters_for(account, 0)
        .unwrap()
        .into_iter()
        .find(|c| c.id == id)
        .expect("the character is still there")
}

/// Ability points: a single `+` click is `0x0138`, and every path answers.
///
/// The owner tried allocating AP three times across two runs and nothing happened. The opcode
/// was written up as `0x0139`, which is only the **bulk** request - a plain `+` click sends
/// `0x0138`, a different opcode with a different body. `research/ap-allocation.md`.
///
/// The load-bearing assertion is **byte 0 of every reply**. Both builders latch
/// `ctx+0x2330` on send and only a `0x007C` clears it, so an unanswered - or wrongly
/// answered - AP request does not lose one point, it kills the stat window for the whole
/// session. Same failure class as `0x0107`.
#[test]
fn a_single_ability_point_click_raises_the_stat_and_always_answers() {
    let (mut s, acct, id) = session_with_ap(7);
    // Read the roll rather than assuming it: character creation rolls the four stats to a
    // total of 25, so the starter STR is not a constant this test gets to know.
    let before = reload(&s, acct, id);

    // 0x0138: u32 tick, u32 statMask. 0x40 is STR.
    let mut body = 0u32.to_le_bytes().to_vec();
    body.extend_from_slice(&net::abilityup::stat_bits::STR.to_le_bytes());
    let replies = s.on_ability_up(&body);

    assert_eq!(replies.len(), 1);
    assert_eq!(replies[0].opcode, net::stats::STAT_CHANGED);
    assert_eq!(replies[0].body[0], 1, "byte 0 is what clears the ctx+0x2330 latch");

    let chr = reload(&s, acct, id);
    assert_eq!(chr.strength, before.strength + 1, "one point of STR");
    assert_eq!(chr.ap, 6, "and one point is gone");
}

/// The client does **not** stop an over-spend, so the server must - and still answer.
///
/// The gate at `142d4bc94` looks like a "do you have the points" check and is not: both
/// positive tests jump *to* the send, so the final comparison is only reachable when both
/// operands are non-positive.
#[test]
fn an_overspend_is_refused_and_the_refusal_is_still_a_stat_change() {
    let (mut s, acct, id) = session_with_ap(2);
    let before = reload(&s, acct, id);

    // 0x0139 bulk: tick, count, then (mask, amount). Ask for 30 with 2 available - the
    // exact shape of the capture that identified this opcode.
    let mut body = 0u32.to_le_bytes().to_vec();
    body.extend_from_slice(&1u32.to_le_bytes());
    body.extend_from_slice(&net::abilityup::stat_bits::STR.to_le_bytes());
    body.extend_from_slice(&30u32.to_le_bytes());
    let replies = s.on_ability_mass_up(&body);

    assert_eq!(replies.len(), 1, "a refusal is still exactly one 0x007C");
    assert_eq!(replies[0].opcode, net::stats::STAT_CHANGED);
    assert_eq!(replies[0].body[0], 1, "the latch is cleared even when refusing");
    assert!(replies[0].what.contains("REFUSED"), "{}", replies[0].what);

    let chr = reload(&s, acct, id);
    assert_eq!((chr.strength, chr.ap), (before.strength, 2), "nothing moved");
}

/// A body that does not parse still gets answered, and short bodies come off a socket.
#[test]
fn an_unreadable_ability_request_still_clears_the_latch() {
    let (mut s, _, _) = session_with_ap(5);
    for n in 0..16 {
        let short = vec![0u8; n];
        for replies in [s.on_ability_up(&short), s.on_ability_mass_up(&short)] {
            assert_eq!(replies.len(), 1, "len {n} must still be answered");
            assert_eq!(replies[0].opcode, net::stats::STAT_CHANGED);
            assert_eq!(replies[0].body[0], 1, "len {n}: byte 0 clears the latch");
        }
    }

    // A mask this client's switch refuses is a refusal, not a silent drop.
    let mut bogus = 0u32.to_le_bytes().to_vec();
    bogus.extend_from_slice(&0x0000_0001u32.to_le_bytes()); // not one of the six
    let replies = s.on_ability_up(&bogus);
    assert!(replies[0].what.contains("REFUSED"), "{}", replies[0].what);
}

/// `0x800` and `0x2000` are **MAX** hp and mp, and current hp/mp must not move.
///
/// The button says "HP". The bit is not current HP, and the request cannot name current HP
/// at all - `0x400` is that, and no arm of the switch accepts it. `CLAUDE.md`'s "the unit,
/// not the arithmetic": three bugs this month were a correct number in the wrong field.
#[test]
fn ability_points_into_hp_raise_the_maximum_not_the_current_value() {
    let (mut s, acct, id) = session_with_ap(3);
    let before = reload(&s, acct, id);

    let mut body = 0u32.to_le_bytes().to_vec();
    body.extend_from_slice(&net::abilityup::stat_bits::MAX_HP.to_le_bytes());
    s.on_ability_up(&body);

    let after = reload(&s, acct, id);
    assert_eq!(
        after.max_hp,
        before.max_hp + net::abilityup::policy::MAX_HP_PER_AP,
        "the MAXIMUM went up"
    );
    assert_eq!(after.hp, before.hp, "and the current value did not");
    assert_eq!(after.ap, before.ap - 1);
}

/// **Giving up a quest removes the row and answers**, driven by the real captured packet.
///
/// The owner: *"I tried to forfeit the quest to start that portion over, but I cannot talk to
/// Sera again."* The forfeit is `0x0151` **action 3** with a **5-byte** body, and
/// `parse_quest_request` needs a 9-byte head - so it returned `None`, and `None` meant
/// silence. Both `0x01ED` and `0x01A5` went unanswered in the same runs and looked like far
/// better candidates; neither is the forfeit. `research/quest-forfeit.md`.
///
/// The client cannot clear its own journal - the started-map erase has exactly two callers
/// and the forfeit path reaches neither - so the row only ever disappears when the server
/// answers. That is why the reply is asserted here and not just the database.
#[test]
fn giving_up_a_quest_forgets_the_row_and_still_answers() {
    let path = std::path::Path::new("../../gm-handbook/questlines.txt");
    if !path.exists() {
        return; // generated data, gitignored
    }
    let config = Config { quests: crate::config::load_quests(path), ..Config::default() };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Quitter".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);

    s.on_quest_request(&hex("01e8030000010000000c046d0100000000"));
    assert_eq!(s.store.quest_rows(id).unwrap().len(), 1, "accepted first");

    // The exact 5 bytes from research/fixtures/quest-forfeit-0151-action3-on-the-wire:
    // action 3, quest 1000.
    let replies = s.on_quest_request(&hex("03e8030000"));
    assert_eq!(replies.len(), 1, "a forfeit is answered - silence is what broke it");
    assert_eq!(replies[0].opcode, net::quest::MESSAGE);
    assert_eq!(replies[0].body, net::questforfeit::forfeit_reply(1000, false));
    assert!(s.store.quest_rows(id).unwrap().is_empty(), "and the row is gone");

    // Forfeiting something never started still answers, because the thing being repaired is
    // the client's journal and a disagreement is exactly the case that matters.
    let again = s.on_quest_request(&hex("03e8030000"));
    assert_eq!(again.len(), 1, "still answered with no row to remove");
}

/// A forfeit must not be mistaken for a start, or a start for a forfeit.
///
/// Five bytes beginning `03` is a forfeit; anything else with a 9-byte head is an ordinary
/// request. The lengths are what separate them, so short bodies off a socket are checked
/// here too - `on_quest_request` is reachable from the wire.
#[test]
fn only_a_five_byte_action_three_is_read_as_a_forfeit() {
    assert!(net::questforfeit::is_forfeit(&hex("03e8030000")));
    // Action 3 was never a documented tag: the census listed six actions and there are nine
    // builder sites across four functions, carrying tags 0, 3 and 7 as well.
    assert!(!net::questforfeit::is_forfeit(&hex("01e8030000010000000c046d0100000000")));
    assert!(!net::questforfeit::is_forfeit(&hex("02e90300000100000043ffe501ffffffff")));

    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Shorty".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(Config::default()));
    s.claim_for_character(id);
    for n in 0..18 {
        let _ = s.on_quest_request(&vec![3u8; n]);
    }
}

/// A session holding `count` Red Potions in Use slot 1, hurt down to `hp`.
fn session_with_potions(count: u16, hp: u32) -> (Session, i64, u32) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Drinker".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    // **A fresh character has 50 max HP**, so a 100-point potion would always cap and the
    // flat amount could never be measured. Raised deliberately, and the test that DOES
    // measure the cap sets its own hp relative to whatever this is.
    made.max_hp = 500;
    made.hp = hp;
    store.save_character_progress(&made).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    store
        .set_inventory_slot(
            made.id,
            store::InventoryType::Use,
            1,
            &store::Item::bundle(2_000_000, count),
        )
        .unwrap();
    // The real table's numbers: Red Potion 100 flat, Roger's Apple 30.
    let config = Config {
        consumables: crate::consumables::Consumables::parse(
            "2000000, 100, 0, 0, 0\n2010000, 30, 0, 0, 0\n",
        ),
        ..Config::default()
    };
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(made.id);
    (s, account_id, made.id)
}

/// The captured packet heals, and takes one out of the stack.
///
/// The owner, 2026-08-21: *"I tried to consume Red Potion, but it did not recover 100 HP."* It
/// did not, because `0x010E` went unanswered - there is exactly one in the whole capture,
/// which is the request-latch signature.
#[test]
fn drinking_a_red_potion_heals_a_hundred_and_takes_one_from_the_stack() {
    let (mut s, acct, id) = session_with_potions(2, 1);
    let before = reload(&s, acct, id);
    assert!(before.max_hp >= 101, "the cap must not be what this test measures");

    // The real body: tick, slot 1, item 2000000, tail 1.
    let replies = s.on_use_item(&net::useitem::use_item(0x0f14_e1f7, 1, 2_000_000, 1));

    let stat = replies
        .iter()
        .find(|r| r.opcode == net::stats::STAT_CHANGED)
        .expect("every 0x010E is answered");
    assert_eq!(stat.body[0], 1, "byte 0 clears the client's request latch");

    let after = reload(&s, acct, id);
    assert_eq!(after.hp, 101, "1 + 100");

    // And the Use tab says one left, WITHOUT emptying the slot.
    let op = replies
        .iter()
        .find(|r| r.opcode == net::inventory::INVENTORY_OPERATION)
        .expect("the client is told the stack shrank");
    assert_eq!(
        op.body,
        net::inventory::inventory_quantity(store::InventoryType::Use.as_u8() as i8, 1, 1),
        "mode 1 UpdateQuantity - mode 3 would empty the slot on screen while the server kept one"
    );
    let left = s.store.inventory_slot(id, store::InventoryType::Use, 1).unwrap().unwrap();
    assert_eq!(left.kind.quantity(), 1);
}

/// **A pet's Auto HP drinks the potion, through the real dispatcher.** The owner, 2026-09-25:
/// *"the pet attempts to drink the potion for the player, but the client never actually
/// performs the restoration ... potions are never consumed and clients never recover."*
/// `0x0206` had no handler; the deployed server logged 25 as UNKNOWN and answered none.
///
/// The body is the deployed capture's shape (`u8 pet, u32 tick, u16 slot, u32 item, u32 1`).
/// Claims, all effects named: HP rises by the potion, the stack shrinks on screen and in the
/// bag, and the latch is cleared (byte 0 of the StatChanged). A pet offering a non-potion is
/// refused - and still answered, because the builder set the latch either way.
#[test]
fn a_pets_auto_hp_drinks_the_potion_and_answers_the_latch() {
    let (mut s, acct, id) = session_with_potions(2, 1);
    assert!(reload(&s, acct, id).max_hp >= 101, "the cap must not be what this test measures");

    let mut packet = net::useitem::CLIENT_PET_USE_ITEM.to_le_bytes().to_vec();
    packet.extend(net::useitem::pet_use_item(0, 0x1e5e_1dfb, 1, 2_000_000));
    let replies = s.handle(&packet);
    let stat = replies.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).expect("0x0206 is answered");
    assert_eq!(stat.body[0], 1, "byte 0 clears the latch the pet's builder set");
    assert_eq!(reload(&s, acct, id).hp, 101, "1 + 100: the owner recovered");
    let op = replies.iter().find(|r| r.opcode == net::inventory::INVENTORY_OPERATION).expect("the stack shrank on screen");
    assert_eq!(op.body, net::inventory::inventory_quantity(store::InventoryType::Use.as_u8() as i8, 1, 1));
    assert_eq!(s.store.inventory_slot(id, store::InventoryType::Use, 1).unwrap().unwrap().kind.quantity(), 1, "and in the bag");

    // A non-potion (a Return Scroll's id) is refused, nothing is taken, and it still answers.
    let mut scroll = net::useitem::CLIENT_PET_USE_ITEM.to_le_bytes().to_vec();
    scroll.extend(net::useitem::pet_use_item(0, 0x1e5e_1dfc, 1, 2_030_002));
    let refused = s.handle(&scroll);
    let stat = refused.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).expect("a refusal answers too");
    assert_eq!(stat.body[0], 1);
    assert!(!refused.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "nothing taken");
    assert_eq!(s.store.inventory_slot(id, store::InventoryType::Use, 1).unwrap().unwrap().kind.quantity(), 1);
}


/// **Sound is kept for the account; the pet's auto-potion thresholds for the character.**
/// The capture's own HP/MP warning pair (`flHP=7`, `flMP=3`) and a sound volume, sent through
/// the real dispatcher. A second character on the same account logs in: its record carries the
/// sound and NOT the first character's thresholds - the owner, 2026-10-02: *"other characters may
/// want to use different auto potion setup."* Then it sets its own, and the first keeps 7/3.
#[test]
fn sound_is_per_account_and_the_auto_potion_thresholds_are_per_character() {
    let (mut s, store, id) = gm_session();
    let mut pair = net::clientsettings::CLIENT_OPTIONS_CHANGED.to_le_bytes().to_vec();
    pair.extend(net::clientsettings::options_changed(1, &[(0x10, 7), (0x11, 3)]));
    assert!(s.handle(&pair).is_empty(), "nothing is expected back");
    let mut sound = net::clientsettings::CLIENT_OPTIONS_CHANGED.to_le_bytes().to_vec();
    sound.extend(net::clientsettings::options_changed(0, &[(0, 30), (1, 1), (0x41, 5)]));
    s.handle(&sound);
    let account = s.claimed.as_ref().unwrap().account_id;
    assert_eq!(
        store.client_settings(account).unwrap(),
        vec![(0, 0, 30), (0, 1, 1)],
        "the account keeps the sound; COUNT (0x41) is not a key and is dropped"
    );
    assert_eq!(store.character_settings(id).unwrap(), vec![(1, 0x10, 7), (1, 0x11, 3)], "the character keeps flHP/flMP");

    let (b28, b32) = s.option_records();
    assert_eq!(b28, vec![(101_563, "flHP=7;flMP=3".to_string())]);
    assert_eq!(b32, vec![(368, "vBG1=30;mBG1=1".to_string())]);

    // A second character on the same account: the sound, and none of the first one's thresholds.
    let other = store
        .create_character(account, 0, &net::opcode::Character { name: "Second".into(), ..Default::default() })
        .unwrap()
        .id;
    assert_ne!(other, id);
    store.create_migration(account, other, 0, 0).unwrap();
    let mut again = Session::new(store.clone(), s.config.clone());
    again.claim_for_character(other);
    let out = again.handle(&crate::session::CLIENT_MIGRATION_HELLO.to_le_bytes());
    let set = out.iter().find(|r| r.opcode == net::opcode::SET_FIELD).expect("the login SetField");
    assert!(!set.what.contains("flHP"), "the first character's threshold leaked: {}", set.what);
    let block = net::clientsettings::shared_quest_ex_block(&b32);
    assert!(set.body.windows(block.len()).any(|w| w == &block[..]), "block #32 - the sound - is in the record");

    let mut own = net::clientsettings::CLIENT_OPTIONS_CHANGED.to_le_bytes().to_vec();
    own.extend(net::clientsettings::options_changed(1, &[(0x10, 12)]));
    again.handle(&own);
    assert_eq!(again.option_records().0, vec![(101_563, "flHP=12".to_string())]);
    assert_eq!(s.option_records().0, vec![(101_563, "flHP=7;flMP=3".to_string())], "the first is untouched");
}

/// **Nobody's threshold resets on the upgrade.** Before 2026-10-02 `flHP` was an account row; a
/// character that has not set its own still gets it, and its own row wins once it does.
#[test]
fn an_account_threshold_from_before_the_split_is_the_fallback_until_the_character_sets_one() {
    let (mut s, store, id) = gm_session();
    let account = s.claimed.as_ref().unwrap().account_id;
    store.save_client_settings(account, 1, &[(0x10, 5), (0x11, 9)]).unwrap();
    assert_eq!(s.option_records().0, vec![(101_563, "flHP=5;flMP=9".to_string())]);
    let mut own = net::clientsettings::CLIENT_OPTIONS_CHANGED.to_le_bytes().to_vec();
    own.extend(net::clientsettings::options_changed(1, &[(0x10, 2)]));
    s.handle(&own);
    assert_eq!(s.option_records().0, vec![(101_563, "flHP=2;flMP=9".to_string())]);
    assert_eq!(store.client_settings(account).unwrap(), vec![(1, 0x10, 5), (1, 0x11, 9)], "the account row is not rewritten");
    assert_eq!(store.character_settings(id).unwrap(), vec![(1, 0x10, 2)]);
}

#[test]
fn a_malformed_body_is_not_stored_and_answers_nothing() {
    let (mut s, store, _id) = gm_session();
    let mut bad = net::clientsettings::CLIENT_OPTIONS_CHANGED.to_le_bytes().to_vec();
    bad.extend(&net::clientsettings::options_changed(1, &[(0x10, 7)])[..12]);
    assert!(s.handle(&bad).is_empty());
    let account = s.claimed.as_ref().unwrap().account_id;
    assert!(store.client_settings(account).unwrap().is_empty());
}

/// `--system-options off` drops block #32 and nothing else: the game options (the HP/MP warning)
/// still ride in block #28.
#[test]
fn system_options_off_drops_only_block_32() {
    let (mut s, store, id) = gm_session();
    let account = s.claimed.as_ref().unwrap().account_id;
    store.save_client_settings(account, 0, &[(0, 30)]).unwrap();
    store.save_client_settings(account, 1, &[(0x10, 7)]).unwrap();
    let (on, _) = s.quest_book(id);
    assert_eq!(on.shared_ex, vec![(368, "vBG1=30".to_string())]);
    Arc::get_mut(&mut s.config).expect("sole owner").system_options = false;
    let (off, _) = s.quest_book(id);
    assert!(off.shared_ex.is_empty(), "block #32 is not sent");
    assert!(off.ex.contains(&(101_563, "flHP=7".to_string())), "block #28 still is");
}

/// **A dead character's pet drinks nothing.** The owner, 2026-10-01: *"The server says I was dead,
/// gave me the revive in town window, but my pet still auto potioned me."* The deployed log has
/// the `0x0206` in the same millisecond as the `0x007C` that set hp 0, and the server healed the
/// corpse to 100.
///
/// Every effect named: HP stays 0, nothing leaves the bag or the screen's stack, and the latch
/// is still cleared. The double-click (`0x010E`) is the same walk and is checked too. The
/// control is the test above: the same potion at 1 HP heals.
#[test]
fn a_dead_characters_potions_are_refused_and_still_answer_the_latch() {
    let (mut s, acct, id) = session_with_potions(2, 0);

    let mut pet = net::useitem::CLIENT_PET_USE_ITEM.to_le_bytes().to_vec();
    pet.extend(net::useitem::pet_use_item(0, 0x1e5e_1dfb, 1, 2_000_000));
    let mut own = net::useitem::CLIENT_USE_ITEM.to_le_bytes().to_vec();
    own.extend(net::useitem::use_item(0x1e5e_1dfc, 1, 2_000_000, 1));
    for (what, packet) in [("pet", pet), ("double-click", own)] {
        let replies = s.handle(&packet);
        let stat = replies.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).expect("answered");
        assert_eq!(stat.body[0], 1, "{what}: byte 0 clears the latch");
        assert_eq!(stat.body, net::stats::StatChange::default().build(), "{what}: an EMPTY change - no HP field");
        assert!(!replies.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "{what}: nothing taken on screen");
        assert_eq!(reload(&s, acct, id).hp, 0, "{what}: still dead - a potion is not a revive");
        assert_eq!(s.store.inventory_slot(id, store::InventoryType::Use, 1).unwrap().unwrap().kind.quantity(), 2, "{what}: nor from the bag");
    }
}

/// **REVIVE IN TOWN from a character the server thinks alive goes nowhere new - never map 0.**
///
/// 2026-10-01: a pet's potion had healed the corpse, so the town button's `0x00D1` (target 0,
/// empty name) reached the portal path and the server sent and SAVED map 0. Every login after
/// crashed the client. The potion is refused now, but the warp is guarded on its own: any
/// destination without a field image is replaced by the map they are on.
#[test]
fn a_transfer_to_a_map_with_no_field_stays_where_it_is() {
    let path = std::path::Path::new("../../gm-handbook/fields.txt");
    if !path.exists() {
        return; // generated data, gitignored
    }
    let (mut s, store, id) = gm_session();
    {
        let cfg = Arc::get_mut(&mut s.config).expect("sole owner in this test");
        cfg.fields = Config::load_fields(path);
    }
    assert!(!s.config.map_exists(0), "the precondition: this client has no map 0");
    assert!(s.config.map_exists(crate::dailyperks::HENESYS));
    let mut chr = s.claimed_character().unwrap();
    chr.hp = 100;
    chr.max_hp = 200;
    store.save_character_progress(&chr).unwrap();
    store.set_character_map(id, crate::dailyperks::HENESYS).unwrap();

    let mut body = vec![0u8; 16];
    body.extend_from_slice(&0u32.to_le_bytes());
    body.extend_from_slice(&0u16.to_le_bytes());
    let out = s.on_transfer_field(&body);
    assert!(out.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "always answered");
    let after = reload(&s, 1, id);
    assert_eq!(after.map_id, crate::dailyperks::HENESYS, "re-sent where they stand, not map 0");
    assert_eq!(after.hp, 100, "and it is not a revive: no HP change");
}

/// **A character already saved on map 0 logs in to a town, through the real login.**
///
/// The owner, 2026-10-01: *"I'm in a state where whenever I log into the server, my client
/// crashes."* The record said map 0. The control is a character saved on Henesys, who stays.
#[test]
fn a_login_saved_on_a_missing_map_lands_in_a_town() {
    let path = std::path::Path::new("../../gm-handbook/fields.txt");
    if !path.exists() {
        return; // generated data, gitignored
    }
    for (saved, want) in [(0, crate::dailyperks::HENESYS), (crate::dailyperks::HENESYS, crate::dailyperks::HENESYS)] {
        let config = Arc::new(Config { fields: Config::load_fields(path), ..Config::default() });
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Lost".to_string(), ..Default::default() };
        let id = store.create_character(account_id, 0, &chr).unwrap().id;
        store.set_character_map(id, saved).unwrap();
        store.create_migration(account_id, id, 0, 0).unwrap();
        let mut s = Session::new(store.clone(), config);
        s.claim_for_character(id);
        let out = s.handle(&crate::session::CLIENT_MIGRATION_HELLO.to_le_bytes());
        let set = out.iter().find(|r| r.opcode == net::opcode::SET_FIELD).expect("the login SetField");
        assert!(set.what.contains(&format!("carrying map {want} ")), "saved on {saved}: {}", set.what);
        assert_eq!(reload(&s, account_id, id).map_id, want, "saved on {saved}: and the record is repaired");
    }
}

/// **A potion draws no number over the player's head**, and this is a rule about the game
/// rather than about the packet.
///
/// The owner, 2026-08-21: *"Potion recovery should not trigger the recovery number, that's only
/// for idle regeneration standing or sitting in a chair in the Set-up tab or sitting on a
/// chair in a map."*
///
/// `0x007C`'s second optional trailer - `u8 flag`, then `u32 hpRecovery, u32 mpRecovery` - is
/// what draws it, and it was briefly sent here on the reasoning that a potion recovers and
/// the field is called recovery. That is arguing from the encoding outwards. The field is the
/// **regeneration indicator**, and which events may raise it is not something the byte layout
/// can tell you.
///
/// So this test asserts an **absence**, which is worth stating plainly: the flag byte must be
/// `0`, and the body must be byte-identical to the same change built without a trailer. It
/// exists because "a recovery is a recovery" is a persuasive-sounding reason to put it back.
#[test]
fn a_potion_sends_no_recovery_trailer_and_therefore_no_floating_number() {
    let (mut s, acct, id) = session_with_potions(2, 1);
    let before = reload(&s, acct, id);
    assert!(before.max_hp >= 101, "the cap must not be what this test measures");

    let replies = s.on_use_item(&net::useitem::use_item(0x0f14_e1f7, 1, 2_000_000, 1));
    let stat = replies
        .iter()
        .find(|r| r.opcode == net::stats::STAT_CHANGED)
        .expect("every 0x010E is answered");

    let after = reload(&s, acct, id);
    let want = net::stats::StatChange {
        hp: Some(after.hp),
        mp: Some(after.mp),
        ..Default::default()
    }
    .build();
    assert_eq!(stat.body, want, "the potion's 0x007C carries no recovery trailer");

    // The trailer is the last field, so its flag is the last byte. Naming it separately from
    // the whole-body compare means a failure says WHICH of the two things went wrong.
    assert_eq!(*stat.body.last().unwrap(), 0, "the recovery flag byte must be 0");

    // And the control, so this cannot pass by the packet being empty or the heal not
    // happening: idle regeneration on the same session DOES carry one.
    assert_ne!(
        stat.body,
        net::stats::StatChange {
            hp: Some(after.hp),
            mp: Some(after.mp),
            recovery: Some((100, 0)),
            ..Default::default()
        }
        .build(),
        "sanity: the two bodies are distinguishable"
    );
}

/// *"(Or less if it will fill my HP bar up to full)"* - the owner asked for the cap in the same
/// sentence, and it is also the only reading that cannot put a number above the maximum into
/// the HP field.
#[test]
fn a_potion_never_heals_past_the_maximum() {
    let (mut s, acct, id) = session_with_potions(1, 1);
    let max = reload(&s, acct, id).max_hp;
    // Hurt to three below full, then drink a 100-point potion.
    let mut chr = reload(&s, acct, id);
    chr.hp = max - 3;
    s.store.save_character_progress(&chr).unwrap();

    s.on_use_item(&net::useitem::use_item(0, 1, 2_000_000, 1));
    assert_eq!(reload(&s, acct, id).hp, max, "capped at full, not max + 97");

    // The potion is gone even though it only healed 3: drinking it still drinks it.
    assert!(s.store.inventory_slot(id, store::InventoryType::Use, 1).unwrap().is_none());
}

/// The last one out of a stack empties the slot, and that is a different packet.
#[test]
fn the_last_potion_empties_the_slot_with_a_remove_not_a_quantity() {
    let (mut s, _, id) = session_with_potions(1, 1);
    let replies = s.on_use_item(&net::useitem::use_item(0, 1, 2_000_000, 1));
    let op = replies
        .iter()
        .find(|r| r.opcode == net::inventory::INVENTORY_OPERATION)
        .expect("the slot change is reported");
    assert_eq!(
        op.body,
        net::inventory::inventory_removed(store::InventoryType::Use.as_u8() as i8, 1)
    );
    assert!(s.store.inventory_slot(id, store::InventoryType::Use, 1).unwrap().is_none());
}

/// **Every refusal still answers**, or the next use never leaves the client.
#[test]
fn every_refused_use_still_sends_a_stat_change() {
    let (mut s, acct, id) = session_with_potions(1, 1);
    let hp_before = reload(&s, acct, id).hp;

    let answered = |rs: &[Reply]| rs.iter().any(|r| r.opcode == net::stats::STAT_CHANGED);

    // A slot that holds something else than the client claims.
    let wrong = s.on_use_item(&net::useitem::use_item(0, 1, 2_010_000, 1));
    assert!(answered(&wrong), "id mismatch must still be answered");

    // An empty slot.
    let empty = s.on_use_item(&net::useitem::use_item(0, 7, 2_000_000, 1));
    assert!(answered(&empty), "empty slot must still be answered");

    // Every truncation, straight off a socket.
    let body = net::useitem::use_item(0, 1, 2_000_000, 1);
    for n in 0..body.len() {
        assert!(answered(&s.on_use_item(&body[..n])), "len {n} must still be answered");
    }

    // And none of that healed anything or took an item.
    assert_eq!(reload(&s, acct, id).hp, hp_before);
    assert!(s.store.inventory_slot(id, store::InventoryType::Use, 1).unwrap().is_some());
}

/// An item the table does not know is refused rather than silently eaten.
#[test]
fn an_item_that_restores_nothing_is_not_consumed() {
    let (mut s, _, id) = session_with_potions(1, 1);
    s.store
        .set_inventory_slot(id, store::InventoryType::Use, 2, &store::Item::bundle(2_040_000, 1))
        .unwrap();
    let replies = s.on_use_item(&net::useitem::use_item(0, 2, 2_040_000, 1));
    assert!(replies.iter().any(|r| r.opcode == net::stats::STAT_CHANGED), "still answered");
    assert!(
        s.store.inventory_slot(id, store::InventoryType::Use, 2).unwrap().is_some(),
        "a scroll is not drunk"
    );
}

/// **A snail hits for more than 1 now, and the server is the one deciding.**
///
/// The owner, 2026-08-21: *"all mobs should not only just deal 1 damage to the player."*
///
/// `on_user_hit` used to apply the client's own number verbatim, and every one of the twelve
/// `0x00E5` captures in that run said `1`. The snail's `PADamage` is **3** in the client's
/// own `Mob.wz`, and `damage::incoming_damage` over that character gives 3 or 4 - so the two
/// disagree and the server's number is the one that moves the bar.
///
/// **The value 3 appears at no offset in any of the twelve bodies**, checked as a u32 across
/// all 144 offsets. So this is not the server misreading a field that holds the real number
/// somewhere else - the client is not sending a 3 at all. That mattered, because
/// `net::userhit::UserHit::damage`'s own doc says its offset is undiscriminated: every
/// capture carries 1 and 1 also sits at six other offsets.
#[test]
fn a_mob_with_an_attack_column_overrides_the_damage_the_client_claimed() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Bitten".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.level = 7;
    made.max_hp = 146;
    made.hp = 146;
    store.save_character_progress(&made).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();

    // Template 2 is the snail, PADamage 3 - the row from the client's own data.
    let config = Config {
        mob_attack: [(2u32, 3u32)].into_iter().collect(),
        ..Config::default()
    };
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(made.id);

    // The captured body, verbatim: attack index -1, template 2, and the client's damage of 1.
    let body = hex(
        "00000000ffffffff0100000002002100431e140f0000000000000000000001000000010000000100000001000000d3070000d307000001000000000000000000000000000000000000de0100008b010000000000000000000000000000ffffffff00000000ffffffff000000000000000002000000000000000000000000000000000000000100000000000000000000000000",
    );
    let replies = s.on_user_hit(&body);
    assert!(!replies.is_empty(), "being hit is answered with the new HP");

    let after = reload(&s, account_id, made.id);
    let lost = 146 - after.hp;
    let (lo, hi) = crate::damage::incoming_window(3, 7, 0);
    assert!(
        (lo..=hi).contains(&lost),
        "the snail should take {lo}..={hi}, not the 1 the client claimed - took {lost}"
    );
    assert!(lost > 1, "and above all, more than 1");

    // **And the server draws NO damage number of its own.** Inverted 2026-08-29.
    //
    // It used to send one, because the client's was a stub: 224 captured hits across mob
    // templates whose PADamage runs 3 to 287 all reported 1, from a `max(damage, 1)` floor
    // that every mob call site fed 0. The owner, 2026-08-27: *"When I died to the Drakes, I still
    // visually took 1 damage, but it wiped out my whole HP bar."*
    //
    // `MobForcedStat` fixed the cause rather than the symptom - the client is told the mob's
    // attack power and computes a real number - and the owner confirmed on 2026-08-29 that the two
    // numbers agree, so ours became the duplicate. This asserts the absence, because a second
    // number reappearing is a screen regression nothing else here would catch.
    assert!(
        !replies.iter().any(|r| r.opcode == net::stats::USER_EFFECT_LOCAL),
        "the client draws its own damage number now; a second one is the bug"
    );

    // The log line carries BOTH numbers, because the disagreement is the measurement.
    let what = &replies[0].what;
    assert!(what.contains("CLIENT claimed 1"), "{what}");
    assert!(what.contains("overrode"), "{what}");
}

/// A mob with no attack column keeps the client's number rather than healing to zero.
///
/// The safe direction: a template we have no data for should still hurt.
#[test]
fn a_mob_we_have_no_attack_data_for_keeps_the_clients_number() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Bitten2".to_string(), ..Default::default() };
    let made = store.create_character(account_id, 0, &chr).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    // Config::default() has an empty mob_attack map.
    let mut s = Session::new(store, Arc::new(Config::default()));
    s.claim_for_character(made.id);

    let before = reload(&s, account_id, made.id).hp;
    let body = hex(
        "00000000ffffffff0100000002002100431e140f0000000000000000000001000000010000000100000001000000d3070000d307000001000000000000000000000000000000000000de0100008b010000000000000000000000000000ffffffff00000000ffffffff000000000000000002000000000000000000000000000000000000000100000000000000000000000000",
    );
    let replies = s.on_user_hit(&body);
    assert_eq!(reload(&s, account_id, made.id).hp, before - 1, "the client's 1 stands");
    assert!(replies[0].what.contains("no template to check it against"), "{}", replies[0].what);
}

/// **Roger's whole quest, end to end**, over the real generated data plus the overlay.
///
/// The owner, 2026-08-21: *"When accepting their quest, it should automatically lower the user's
/// health to 25/50, and give users a Roger's Apple to recover their HP with."* and *"Once
/// the user consumes the apple, the quest would be completed."*
///
/// Both halves are authored keys - the WZ has no `Act.<state>.hp` anywhere in its 17 Act
/// shapes, and `Check.1.item.0` for 1002 carries an id with no count, which is the shape
/// `research/quest-scripts.md` marked [I] and left open. The owner's two sentences settle it.
#[test]
fn rogers_apple_drops_your_health_on_accept_and_completes_when_you_eat_it() {
    let generated = std::path::Path::new("../../gm-handbook/questlines.txt");
    let authored = std::path::Path::new("../../data/quest-scripts.txt");
    if !generated.exists() {
        return; // generated data, gitignored
    }
    let mut quests = crate::config::load_quests(generated);
    crate::config::overlay_quests(&mut quests, authored);
    assert_eq!(quests[&1002].set_hp.get(&0), Some(&25), "Act.0.hp came out of the overlay");
    assert_eq!(quests[&1002].complete_on_consume, Some(2_010_000), "Check.1.consumeitem");

    let config = Config {
        quests,
        consumables: crate::consumables::Consumables::parse("2010000, 30, 0, 0, 0\n"),
        ..Config::default()
    };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Apple".to_string(), ..Default::default() };
    let made = store.create_character(account_id, 0, &chr).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(made.id);

    let full = reload(&s, account_id, made.id);
    assert_eq!(full.hp, full.max_hp, "starts at full health");
    assert_eq!(full.max_hp, 50, "a fresh character, so 25 really is 25/50");

    // Accept.
    let accept = s.on_quest_request(&hex("01ea0300000300000044ff130100000000"));
    assert!(!accept.is_empty());
    let hurt = reload(&s, account_id, made.id);
    assert_eq!(hurt.hp, 25, "25/50, which is what the apple is for");
    let apple = s
        .store
        .bag_items(made.id, store::InventoryType::Use)
        .unwrap()
        .into_iter()
        .find(|i| i.item.item_id == 2_010_000)
        .expect("Roger hands over the apple");
    assert_eq!(s.store.quest_rows(made.id).unwrap().len(), 1, "and the quest is started");

    // Eat it. The quest finishes because the apple is gone, not because an NPC was clicked.
    let eaten = s.on_use_item(&net::useitem::use_item(0, apple.slot as i16, 2_010_000, 1));
    // The apple restores 30 from 25/50, which is 55 - so it caps at 50. That IS the
    // behaviour the owner asked for in the same breath as the potion: *"or less if it will fill
    // my HP bar up to full"*. A fresh character cannot show the uncapped 30.
    assert_eq!(reload(&s, account_id, made.id).hp, 50, "healed to full, not to 55");
    assert!(
        eaten.iter().any(|r| r.opcode == net::questeffect::USER_EFFECT_LOCAL),
        "and the turn-in plays the QuestClear fanfare"
    );
    let book = s.store.quest_book(made.id).unwrap();
    assert!(book.started.is_empty(), "no longer in progress");
    assert!(book.completed.iter().any(|q| q.quest_id == 1002), "1002 is recorded complete");
}

/// Eating something no started quest asked for finishes nothing.
#[test]
fn eating_an_apple_you_were_never_asked_for_completes_no_quest() {
    let mut quests = std::collections::HashMap::new();
    quests.insert(
        1002u32,
        crate::config::Quest { complete_on_consume: Some(2_010_000), ..Default::default() },
    );
    let config = Config {
        quests,
        consumables: crate::consumables::Consumables::parse("2010000, 30, 0, 0, 0\n"),
        ..Config::default()
    };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Greedy".to_string(), ..Default::default() };
    let made = store.create_character(account_id, 0, &chr).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    store
        .set_inventory_slot(
            made.id,
            store::InventoryType::Use,
            1,
            &store::Item::bundle(2_010_000, 1),
        )
        .unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(made.id);

    let eaten = s.on_use_item(&net::useitem::use_item(0, 1, 2_010_000, 1));
    assert!(
        !eaten.iter().any(|r| r.opcode == net::questeffect::USER_EFFECT_LOCAL),
        "no fanfare for a quest that was never started"
    );
    assert!(s.store.quest_book(made.id).unwrap().completed.is_empty());
}

/// The `0x055B` in a reply list, which is no longer always the first thing in it.
///
/// Accepting a quest now sends the `0x0089` quest record **and** the Say, and the record
/// goes first so the journal is right before the NPC's follow-up line is drawn. Selecting
/// by opcode rather than by index keeps these tests about what they are about.
fn script_message(replies: &[Reply]) -> &Reply {
    replies
        .iter()
        .find(|r| r.opcode == net::script::SCRIPT_MESSAGE)
        .expect("a quest request is always answered with something to say")
}

/// Pull the text out of a script-message body. The shared head is 14 bytes; a Say then
/// has a `u32 echo` before its string and a yes/no box does not.
fn script_text(body: &[u8]) -> (u8, String, usize) {
    let message_type = body[10];
    let at = if message_type == net::script::SCRIPT_TYPE_SAY { 18 } else { 14 };
    let len = u16::from_le_bytes([body[at], body[at + 1]]) as usize;
    let text = String::from_utf8(body[at + 2..at + 2 + len].to_vec()).unwrap();
    (message_type, text, at + 2 + len)
}

/// The client's 0x00F3, in the shape the captures show: the box's own text echoed back.
fn reply_bytes(text: &str, message_type: u8, action: u8) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&0u32.to_le_bytes()); // handle
    b.push(message_type);
    b.extend_from_slice(&0u32.to_le_bytes()); // echo
    b.extend_from_slice(&(text.len() as u16).to_le_bytes());
    b.extend_from_slice(text.as_bytes());
    b.push(action);
    b
}

/// A refused `!map` says why on screen now, and Log Out is answered.
///
/// The Log Out half is the one that matters beyond politeness: until `0x0106` clears
/// `world->[0x33f4]`, the client drops every `SetField` in silence.
#[test]
fn a_bad_map_says_why_and_log_out_is_answered() {
    let path = std::path::Path::new("../../gm-handbook/fields.txt");
    if !path.exists() {
        return; // generated data, gitignored
    }
    let config = Config {
        fields: Config::load_fields(path),
        ..Config::default()
    };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);

    fn chat(text: &str) -> Vec<u8> {
        let mut b = net::opcode::CLIENT_CHAT.to_le_bytes().to_vec();
        b.extend_from_slice(&[0u8; 4]);
        b.extend_from_slice(&(text.len() as u16).to_le_bytes());
        b.extend_from_slice(text.as_bytes());
        b.push(3);
        b
    }

    // A map with no field image is refused, and the refusal reaches the screen.
    let replies = s.handle(&chat("!map 104040000"));
    assert_eq!(replies.len(), 1);
    assert_eq!(replies[0].opcode, net::notice::CHAT_NOTICE);
    assert_eq!(replies[0].body[0], 1, "force must be 1 or only the first line shows");
    let len = u16::from_le_bytes([replies[0].body[1], replies[0].body[2]]) as usize;
    let text = String::from_utf8(replies[0].body[3..3 + len].to_vec()).unwrap();
    assert!(text.contains("104040000"), "the notice must name the id: {text}");

    // So is a non-numeric one, rather than being swallowed.
    assert_eq!(s.handle(&chat("!map banana"))[0].opcode, net::notice::CHAT_NOTICE);

    // A real map still warps - and now says so first. The owner asked for every GM command to
    // acknowledge itself, because a command that silently works and one that silently
    // does nothing look identical on screen.
    let warp = s.handle(&chat("!map 40"));
    assert_eq!(warp[0].opcode, net::notice::CHAT_NOTICE, "the acknowledgement leads");
    let len = u16::from_le_bytes([warp[0].body[1], warp[0].body[2]]) as usize;
    let ack = String::from_utf8(warp[0].body[3..3 + len].to_vec()).unwrap();
    assert!(ack.starts_with("Teleporting "), "{ack}");
    assert!(ack.contains("map 40"), "{ack}");
    assert!(
        warp.iter().any(|r| r.opcode == net::opcode::SET_FIELD),
        "the warp itself still happens"
    );

    // **Anything that is not a command is said out loud.** The client draws nothing for
    // its own chat, so a server that answers nothing is a player typing into a void -
    // which is exactly what the owner got on 2026-08-19 from "Hello", "Hello2", "Hello3".
    let said = s.handle(&chat("Hello"));
    assert_eq!(said.len(), 1, "chat was swallowed");
    assert_eq!(said[0].opcode, net::userchat::USER_CHAT);
    let body = &said[0].body;
    assert_eq!(
        u32::from_le_bytes([body[0], body[1], body[2], body[3]]),
        id,
        "the balloon has to be attached to the speaker"
    );
    let len = u16::from_le_bytes([body[5], body[6]]) as usize;
    assert_eq!(&body[7..7 + len], b"Hello");
    assert_eq!(body.len(), net::userchat::USER_CHAT_OVERHEAD + len,
               "both trailing bytes must be there - the client reads past the text");

    // An unknown command says so rather than vanishing, and is NOT spoken aloud.
    let unknown = s.handle(&chat("!nope"));
    assert_eq!(unknown.len(), 1);
    assert_eq!(unknown[0].opcode, net::notice::CHAT_NOTICE);

    // And an empty line is dropped: a balloon with no text is worse than none.
    assert!(s.handle(&chat("")).is_empty());

    // Log out is answered, with a non-empty message - an empty one is a client no-op.
    let mut body = net::notice::CLIENT_LOG_OUT.to_le_bytes().to_vec();
    body.truncate(2);
    let out = s.handle(&body);
    assert_eq!(out.len(), 1, "an unanswered log out makes every later SetField vanish");
    assert_eq!(out[0].opcode, net::notice::LOG_OUT_RESULT);
    assert!(u16::from_le_bytes([out[0].body[0], out[0].body[1]]) > 0);
}

/// Idle chatter: the only unsolicited packet the server sends.
///
/// Pins the three things a client run cannot easily show - the cadence window, that the
/// lines advance in order and wrap, and that a stalled connection does not burst.
#[test]
fn npcs_chatter_in_order_on_the_clients_own_cadence() {
    let npcs = vec![
        net::opcode::FieldNpc {
            object_id: 1000, template_id: 8, x: 69, cy: 275, fh: 30,
            rx0: 19, rx1: 119, f: 0,
        },
        net::opcode::FieldNpc {
            object_id: 1001, template_id: 9, x: 1602, cy: 215, fh: 59,
            rx0: 1552, rx1: 1652, f: 0,
        },
    ];
    let mut strings = std::collections::HashMap::new();
    strings.insert(
        8u32,
        crate::config::NpcStrings {
            name: "Robin".into(),
            info: (0..4).map(|i| format!("line {i}")).collect(),
            ..Default::default()
        },
    );
    // Template 9 has no info lines at all - it must never speak, and must not panic on
    // the modulo either.
    strings.insert(9u32, crate::config::NpcStrings::default());

    let config = Config {
        npcs: [(40u32, npcs)].into_iter().collect(),
        npc_strings: strings.into(),
        ..Config::default()
    };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character {
        name: "TestCharD".to_string(), map_id: 40, ..Default::default()
    };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);
    s.on_field_entered();

    // Nothing is due before the minimum wait, ever.
    assert!(s.tick(0).is_empty());
    assert!(s.tick(CHATTER_MIN_MS - 1).is_empty(), "3s is the floor");

    // Drive it and collect the lines Robin says, and when.
    let mut said = Vec::new();
    let mut gaps = Vec::new();
    let mut last = 0u64;
    for now in (0..120_000).step_by(250) {
        for reply in s.tick(now) {
            let body = &reply.body;
            assert_eq!(body.len(), net::npcchat::NPC_CHAT_LEN);
            let who = u32::from_le_bytes(body[..4].try_into().unwrap());
            assert_eq!(who, 1000, "only the NPC with lines may speak");
            assert_eq!(body[4] as i8, net::npcchat::NPC_CHAT_NO_ANIMATION);
            said.push(body[5]);
            if last > 0 {
                gaps.push(now - last);
            }
            last = now;
        }
    }

    assert!(said.len() > 10, "only {} lines in two minutes", said.len());
    // In order, wrapping at the line count. This is the half that is ours.
    for (i, idx) in said.iter().enumerate() {
        assert_eq!(*idx as usize, i % 4, "line {i} out of order");
    }
    // And the cadence is the client's own window. The tick granularity can only make a
    // gap look longer, never shorter, so the floor is the strict check.
    assert!(
        gaps.iter().all(|g| *g >= CHATTER_MIN_MS),
        "a gap below the 3s floor: {:?}",
        gaps.iter().filter(|g| **g < CHATTER_MIN_MS).collect::<Vec<_>>()
    );
    let ceiling = CHATTER_MIN_MS + CHATTER_SPREAD_MS + 500;
    assert!(gaps.iter().all(|g| *g <= ceiling), "a gap above 9s: {gaps:?}");
    // Randomised, not fixed - a constant interval would be a regression to what the owner
    // asked to move away from.
    assert!(gaps.iter().collect::<std::collections::HashSet<_>>().len() > 2, "{gaps:?}");
}

/// Entering a field late must not make the whole map speak at once.
///
/// `handle` takes bytes and not time, so a field entry has no clock of its own. Before
/// the session carried one, `reset_chatter` scheduled from zero and every NPC on a map
/// entered after the first ten seconds was already overdue.
#[test]
fn entering_a_field_late_does_not_make_everyone_speak_at_once() {
    let npcs = (0..3)
        .map(|i| net::opcode::FieldNpc {
            object_id: 1000 + i, template_id: 8, x: 0, cy: 0, fh: 1,
            rx0: 0, rx1: 0, f: 0,
        })
        .collect::<Vec<_>>();
    let mut strings = std::collections::HashMap::new();
    strings.insert(
        8u32,
        crate::config::NpcStrings {
            info: (0..4).map(|i| format!("line {i}")).collect(),
            ..Default::default()
        },
    );
    let config = Config {
        npcs: [(40u32, npcs)].into_iter().collect(),
        npc_strings: strings.into(),
        ..Config::default()
    };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character {
        name: "TestCharD".to_string(), map_id: 40, ..Default::default()
    };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);

    // Half a minute of ticks, then walk into the field - a portal walk, or a !map.
    //
    // **Inclusive of 30_000 on purpose.** `on_field_entered` schedules from the
    // session's own clock, which is the last tick it was given - so a range ending at
    // 29_500 would put the floor at 32_500 while the assertion below measured it from
    // 30_000. That is a real 500ms window, and with three NPCs each drawing from a
    // 6000ms spread it made this test fail about one run in six, on a seed that comes
    // from a heap address and so changes with what else the suite ran.
    for now in (0..=30_000).step_by(500) {
        s.tick(now);
    }
    s.on_field_entered();

    // Nothing may be due before the 3s floor measured from NOW, not from zero.
    for now in (30_000..30_000 + CHATTER_MIN_MS).step_by(250) {
        assert!(
            s.tick(now).is_empty(),
            "an NPC spoke {}ms after a late field entry",
            now - 30_000
        );
    }
    // And when they do start, they do not all go at once - three NPCs each drawing an
    // independent delay from a 6000ms window colliding exactly is the thing to notice.
    let mut first_tick_counts = Vec::new();
    for now in (30_000 + CHATTER_MIN_MS..50_000).step_by(250) {
        let n = s.tick(now).len();
        if n > 0 {
            first_tick_counts.push(n);
        }
    }
    assert!(!first_tick_counts.is_empty(), "nobody ever spoke");
    assert!(
        first_tick_counts.iter().any(|&n| n < 3),
        "every tick spoke for all three: {first_tick_counts:?}"
    );
}

/// The record carries a bag, and `--inventory-slots` can change what is in it.
///
/// The default and the override are checked in the SAME test on purpose: 24 is also the
/// number a client could plausibly have defaulted to on its own, so only a value that
/// could not have come from anywhere else proves the field is being read. That is the
/// same argument the flag's doc makes for spending a client launch at 32 rather than 24.
#[test]
fn the_record_sizes_the_bag_and_the_override_reaches_it() {
    fn record_of(config: Config) -> Vec<u8> {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character {
            name: "TestCharD".to_string(), map_id: 1, ..Default::default()
        };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::new(store, Arc::new(config));
        s.claim_for_character(id);
        let chr = s.claimed_character().expect("the claimed character");
        net::opcode::character_record_for_set_field(&chr, 0)
    }

    // Where the six u16 live: after the 100-byte presence array, the eleven head bytes,
    // the stat block and the four string flags.
    let sizes_at = net::opcode::STAT_BLOCK_AT + net::opcode::stat_block_len(0) + 4;
    let read = |record: &[u8]| -> Vec<u16> {
        (0..net::opcode::INVENTORY_COUNT)
            .map(|i| {
                let at = sizes_at + i * 2;
                u16::from_le_bytes([record[at], record[at + 1]])
            })
            .collect()
    };

    let plain = record_of(Config::default());
    assert_eq!(
        read(&plain),
        net::opcode::default_inventory_slots().to_vec(),
        "a new character reached the wire with no bag (five at 30, Deco at its 150)"
    );
    assert_eq!(plain[net::opcode::PRESENCE_INVENTORY_SIZE], 1);

    let forced = record_of(Config {
        inventory_slots: Some(32),
        ..Config::default()
    });
    assert_eq!(read(&forced), vec![32u16; net::opcode::INVENTORY_COUNT]);
    // And nothing else moved: same length, and the only differing bytes are the twelve.
    assert_eq!(plain.len(), forced.len());
    let differing: Vec<usize> =
        (0..plain.len()).filter(|&i| plain[i] != forced[i]).collect();
    assert!(
        differing.iter().all(|&i| (sizes_at..sizes_at + 12).contains(&i)),
        "the override changed bytes outside the bag: {differing:?}"
    );
    assert!(!differing.is_empty(), "the override changed nothing at all");
}

/// A connection that stalls must not emit a backlog when it comes back.
#[test]
fn a_late_tick_does_not_burst() {
    let npcs = vec![net::opcode::FieldNpc {
        object_id: 1000, template_id: 8, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0,
    }];
    let mut strings = std::collections::HashMap::new();
    strings.insert(
        8u32,
        crate::config::NpcStrings {
            info: (0..4).map(|i| format!("line {i}")).collect(),
            ..Default::default()
        },
    );
    let config = Config {
        npcs: [(40u32, npcs)].into_iter().collect(),
        npc_strings: strings.into(),
        ..Config::default()
    };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character {
        name: "TestCharD".to_string(), map_id: 40, ..Default::default()
    };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);
    s.on_field_entered();

    // Ten minutes with no ticks at all, then one. Exactly one line, not sixty.
    assert_eq!(s.tick(600_000).len(), 1);
    assert!(s.tick(600_001).is_empty(), "the next one waits the full interval");
}

/// `!map <id>` typed into the chat box, from the real captured chat body.
#[test]
fn the_gm_map_command_moves_the_character() {
    // The exact shape the client sends: u32 tick, u16 length, text, u8 tab.
    fn chat(text: &str) -> Vec<u8> {
        let mut b = vec![0u8; 4];
        b.extend_from_slice(&(text.len() as u16).to_le_bytes());
        b.extend_from_slice(text.as_bytes());
        b.push(3); // the All tab
        b
    }
    assert_eq!(net::opcode::parse_chat(&chat("Hello")).as_deref(), Some("Hello"));
    assert_eq!(net::opcode::parse_chat(&chat("!map 40")).as_deref(), Some("!map 40"));

    // The real 12 bytes the owner sent, so the parser is tested against the client and not
    // only against its own encoder.
    let real = [0xe7, 0x5b, 0x64, 0x05, 0x05, 0x00, b'H', b'e', b'l', b'l', b'o', 0x03];
    assert_eq!(net::opcode::parse_chat(&real).as_deref(), Some("Hello"));

    // Short bodies come off a socket and must not panic.
    for n in 0..6 {
        assert_eq!(net::opcode::parse_chat(&real[..n]), None, "{n} bytes");
    }
}

/// A map that does not exist must not move the character anywhere.
///
/// Being stranded is the mild failure. `research/map1-exists.md` found that a map with no
/// `String.wz` name entry sends the client down a branch containing a **non-returning**
/// `E_POINTER` call, and that 12 ids are named-but-absent while 6 are present-but-unnamed
/// - so "it has a name" is not the same question as "it has a field".
#[test]
fn the_map_command_refuses_a_map_that_does_not_exist() {
    fn chat(text: &str) -> Vec<u8> {
        let mut body = net::opcode::CLIENT_CHAT.to_le_bytes().to_vec();
        body.extend_from_slice(&[0u8; 4]);
        body.extend_from_slice(&(text.len() as u16).to_le_bytes());
        body.extend_from_slice(text.as_bytes());
        body.push(3);
        body
    }

    let (mut s, _st, _id, _acct) = session();
    // With a field list loaded, only ids in it are allowed.
    let cfg = crate::config::Config {
        fields: [1u32, 10, 40].into_iter().collect(),
        ..(*s.config).clone()
    };
    s.config = std::sync::Arc::new(cfg);
    assert!(s.handle(&chat("!map 999999999")).is_empty(), "a nonexistent map moves nobody");
    assert!(s.handle(&chat("!map 0")).is_empty(), "0 is not a map");

    // And an EMPTY list must not refuse everything - that would fail closed on a missing
    // generated file rather than on a real problem.
    let cfg = crate::config::Config { fields: Default::default(), ..(*s.config).clone() };
    assert!(cfg.map_exists(999_999_999), "an unknown table allows, it does not refuse");
    assert!(cfg.map_exists(1));
}

/// Only `/map` with a number is a command; ordinary chat must stay ordinary.
#[test]
fn ordinary_chat_is_not_a_command() {
    let (mut s, _store, _id, _acct) = session();
    for text in ["Hello", "/map", "/map abc", "/mapabc 1", "map 40", "/warp 40"] {
        let mut b = vec![0u8; 4];
        b.extend_from_slice(&(text.len() as u16).to_le_bytes());
        b.extend_from_slice(text.as_bytes());
        b.push(3);
        let mut body = net::opcode::CLIENT_CHAT.to_le_bytes().to_vec();
        body.extend_from_slice(&b);
        assert!(s.handle(&body).is_empty(), "{text:?} should not move anybody");
    }
}

/// The portal table is generated from the client's `Map.wz`, so the loader has to cope
/// with what a generator emits: a comment header, blank lines, and a fourth column it
/// does not use. Rows are the real ones for maps 1 and 10 - the exact two-way route that
/// stranded a character on map 10 when the table was a hand-typed stub.
#[test]
fn the_portal_table_loads_and_is_keyed_on_the_source_map() {
    let dir = std::env::temp_dir().join("maplecw-portal-test");
    std::fs::create_dir_all(&dir).expect("a temp dir");
    let path = dir.join("portals.txt");
    std::fs::write(
        &path,
        "# map, index, portal, target map, target portal

         1, 0, sp, 0, 
1, 4, out00, 10, in00
         10, 0, sp, 0, 
10, 1, in00, 1, out00
10, 2, out00, 20, in00
         not, a, valid, row, here
",
    )
    .expect("write");

    let (links, index) = crate::config::Config::load_portals(&path);
    assert_eq!(
        links.get(&(1, "out00".to_string())),
        Some(&(10, "in00".to_string())),
        "map 1's out00 leads to map 10's in00"
    );
    assert_eq!(links.get(&(10, "in00".to_string())), Some(&(1, "out00".to_string())));
    assert_eq!(links.get(&(1, "in00".to_string())), None, "keyed on the SOURCE map");

    // The arrival lookup is the other direction, and spawn points must be in it even
    // though they lead nowhere - `sp` is a perfectly good place to arrive.
    assert_eq!(index.get(&(10, "in00".to_string())), Some(&1), "map 10's in00 is index 1");
    assert_eq!(index.get(&(1, "sp".to_string())), Some(&0), "spawns are indexed too");
    assert_eq!(index.get(&(1, "out00".to_string())), Some(&4));

    let (l, i) = crate::config::Config::load_portals(std::path::Path::new("no-such-file"));
    assert!(l.is_empty() && i.is_empty(), "a missing file is empty, not a panic");
}

/// The client omits BOTH coordinates when the portal name is empty, so the body is 31
/// bytes and x/y are not at a fixed offset from the end.
#[test]
fn a_nameless_portal_request_has_no_coordinates() {
    let mut body = vec![0u8; 31];
    body[16..20].copy_from_slice(&10u32.to_le_bytes()); // an explicit target field
    // length prefix at 20 stays 0 -> empty name
    let r = parse_transfer_field(&body).expect("a 31-byte body parses");
    assert_eq!(r.target_field, Some(10));
    assert_eq!(r.portal_name, "");
    assert_eq!(r.position, None, "no coordinates when the name is empty");
}

/// Too short to hold the fixed part is None, not a panic - the body comes off a socket.
#[test]
fn a_truncated_portal_request_is_rejected_not_panicked_on() {
    for n in 0..20 {
        assert_eq!(parse_transfer_field(&vec![0u8; n]), None, "{n} bytes should not parse");
    }
}

/// The offset came from a real capture; this is that capture.
#[test]
fn the_character_id_is_read_out_of_a_real_0x007d_body() {
    let body = hex(
        "0000000000000000cc000000aabbccddeeffdeadbeef00000000764d0000230000\
         00020000007d29595a7929595a3fc073dd1e000000",
    );
    assert_eq!(migration_hello_character(&body), Some(204));
}

#[test]
fn a_short_hello_yields_no_character_rather_than_panicking() {
    for len in 0..12 {
        migration_hello_character(&vec![0u8; len]);
    }
    assert_eq!(migration_hello_character(&[0u8; 11]), None);
    assert_eq!(migration_hello_character(&[0u8; 12]), Some(0));
}

/// The pet item as the bag holds it - WITH its `pet_id`, which is what the blob builder keys
/// the name, vitals and active byte on. A bare `Item::bundle` would read as an un-numbered pet.
fn bag_pet(store: &Arc<Store>, character_id: u32, item_id: u32) -> store::Item {
    store
        .bag_items(character_id, store::InventoryType::Cash)
        .unwrap()
        .into_iter()
        .find(|r| r.item.item_id == item_id)
        .map(|r| r.item)
        .unwrap_or_else(|| panic!("no pet {item_id} in character {character_id}'s Cash tab"))
}

/// The `pets` row id of `item_id` in this character's Cash tab - the first one, when there
/// are two. Tests that talk about "the Husky" mean this.
fn pet_of(store: &Arc<Store>, character_id: u32, item_id: u32) -> u32 {
    store
        .bag_items(character_id, store::InventoryType::Cash)
        .unwrap()
        .into_iter()
        .find(|r| r.item.item_id == item_id)
        .and_then(|r| r.item.pet_id)
        .unwrap_or_else(|| panic!("no numbered pet {item_id} in character {character_id}'s Cash tab"))
}

fn hex(s: &str) -> Vec<u8> {
    let clean: String = s.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    (0..clean.len() / 2)
        .map(|i| u8::from_str_radix(&clean[i * 2..i * 2 + 2], 16).unwrap())
        .collect()
}


// ---------------------------------------------------------------------------------------
// The server's EXP and meso rates, and the banner - which since 2026-10-03 is `!announce`'s
// alone: a rate change scrolls nothing (the owner: "Players can check the EXP rates using the
// public !rate command anyways").
// ---------------------------------------------------------------------------------------

/// The text inside a `0x00AC` type-4 banner, or `None` if the packet is a teardown.
fn banner_of(r: &Reply) -> Option<String> {
    assert_eq!(r.opcode, net::broadcast::BROADCAST_MSG);
    assert_eq!(r.body[0], net::broadcast::BANNER, "type 4 is the banner");
    if r.body[1] == 0 {
        assert_eq!(r.body.len(), 2, "a teardown carries NO string - the client reads none");
        return None;
    }
    let len = u16::from_le_bytes([r.body[2], r.body[3]]) as usize;
    Some(String::from_utf8(r.body[4..4 + len].to_vec()).unwrap())
}

fn banners(out: &[Reply]) -> Vec<Option<String>> {
    out.iter()
        .filter(|r| r.opcode == net::broadcast::BROADCAST_MSG)
        .map(banner_of)
        .collect()
}

/// `!setrates 2 ...` stores the rate, tells the GM, and **scrolls nothing** - not at once and
/// not on later ticks. Since 2026-10-03 the banner is `!announce`'s alone.
#[test]
fn setrates_stores_the_rate_and_scrolls_nothing() {
    let (mut s, store, _) = gm_session();
    let out = s.handle(&gm_chat("!setrates 2 1 1 1 30"));

    assert_eq!(store.rates().unwrap().exp.rate.per_cent(), 200, "stored as hundredths");
    assert!(banners(&out).is_empty(), "{out:?}");
    assert!(notice_text(&out[0]).contains("2x"), "and the GM who typed it is told: {out:?}");
    for tick in 1..=6u64 {
        assert!(banners(&s.tick(tick * 500)).is_empty(), "tick {tick}");
    }
}

/// **`!announce <message>` scrolls exactly that**, and a bare `!announce` takes it down.
#[test]
fn announce_scrolls_the_gms_text_and_a_bare_one_clears_it() {
    let (mut s, store, _) = gm_session();
    let out = s.handle(&gm_chat("!announce Double EXP all weekend!"));
    assert_eq!(banners(&out), vec![Some("Double EXP all weekend!".to_string())]);
    assert_eq!(store.announcement().unwrap().as_deref(), Some("Double EXP all weekend!"), "stored, so a restart keeps it");
    assert!(banners(&s.tick(500)).is_empty(), "sent once - a re-send restarts the scroll");

    let out = s.handle(&gm_chat("!announce"));
    assert_eq!(banners(&out), vec![None], "taken down");
    assert_eq!(store.announcement().unwrap(), None);
}

/// Back to 1x scrolls nothing either - the old "event has ended" line went with the cycle.
#[test]
fn returning_to_normal_scrolls_nothing() {
    let (mut s, store, _) = gm_session();
    s.handle(&gm_chat("!setrates 2 1 1 1 30"));
    let out = s.handle(&gm_chat("!setrates 1 1 1 1 30"));

    assert_eq!(store.rates().unwrap().exp.rate, store::rates::Rate::NORMAL);
    assert!(banners(&out).is_empty(), "{out:?}");
}

/// `!exprate 1` on a server that was never running an event announces nothing.
///
/// The failure this guards against is noisy rather than broken: three "event has ended"
/// banners on a fresh server, for events nobody ran.
#[test]
fn ending_an_event_that_never_started_says_nothing() {
    let (mut s, _, _) = gm_session();
    let out = s.handle(&gm_chat("!setrates 1 1 1 1 30"));
    assert!(banners(&out).is_empty(), "{out:?}");
    assert!(notice_text(&out[0]).contains("already"), "{out:?}");
}

/// A bare `!announce` with nothing set explains itself and scrolls nothing; `!rate` is
/// `!rates`, the public command the owner pointed players at.
#[test]
fn a_bare_announce_with_nothing_set_scrolls_nothing_and_rate_is_rates() {
    let (mut s, _, _) = gm_session();
    let out = s.handle(&gm_chat("!announce"));
    assert!(banners(&out).is_empty(), "{out:?}");
    assert!(notice_text(&out[0]).contains("!announce <message>"), "{out:?}");
    let rate = notice_text(&s.handle(&gm_chat("!rate"))[0]);
    assert_eq!(rate, notice_text(&s.handle(&gm_chat("!rates"))[0]));
}

/// The banner is sent when the answer CHANGES and not otherwise. Re-sending it restarts the
/// scroll on screen, so a tick that has nothing new to say must say nothing.
#[test]
fn a_tick_with_nothing_new_sends_no_banner() {
    let (mut s, _, _) = gm_session();
    s.handle(&gm_chat("!setrates 2 1 1 1 30"));
    for tick in 1..=6u64 {
        let out = s.tick(tick * 500);
        assert!(
            banners(&out).is_empty(),
            "tick {tick} re-sent a banner that was already on screen: {out:?}"
        );
    }
}

/// A bad argument is refused **and changes nothing**. The dangerous failure here is a
/// command that says something plausible and leaves the rate half-set.
#[test]
fn a_bad_multiplier_changes_nothing() {
    let (mut s, store, _) = gm_session();
    s.handle(&gm_chat("!setrates 2 1 1 1 30"));
    for bad in ["fast", "0", "1000", "1.234", "-2"] {
        let out = s.handle(&gm_chat(&format!("!setrates {bad} 1 1 1 30")));
        assert!(banners(&out).is_empty(), "{bad} moved the banner: {out:?}");
        assert_eq!(store.rates().unwrap().exp.rate.per_cent(), 200, "{bad} changed the rate");
    }
}

/// Setting a rate to what it already is does not restart the five-minute cycle.
#[test]
fn setting_the_same_rate_again_is_a_no_op() {
    let (mut s, _, _) = gm_session();
    s.handle(&gm_chat("!setrates 2 1 1 1 30"));
    let out = s.handle(&gm_chat("!setrates 2 1 1 1 30"));
    assert!(banners(&out).is_empty(), "it would have restarted the scroll: {out:?}");
    assert!(notice_text(&out[0]).contains("already"), "{out:?}");
}

/// The EXP rate multiplies what a kill is worth.
#[test]
fn the_exp_rate_multiplies_a_kill() {
    let (mut s, _, _) = gm_session();
    let mut mob_exp = std::collections::HashMap::new();
    mob_exp.insert(2u32, 15u32);
    s.config = Arc::new(Config { mob_exp, ..(*s.config).clone() });

    assert_eq!(s.exp_for_kill(2).0, 15, "1x by default");
    s.handle(&gm_chat("!setrates 2 1 1 1 30"));
    assert_eq!(s.exp_for_kill(2).0, 30);
    s.handle(&gm_chat("!setrates 1.5 1 1 1 30"));
    assert_eq!(s.exp_for_kill(2).0, 22, "truncated, not rounded");
    assert!(s.exp_for_kill(2).1.contains("1.5x"), "and the log says why");
}

/// **The EXP coupon actually multiplies a kill**, which it did not until 2026-09-09.
///
/// The owner: *"Once the EXP buff is applied, either the 2x or the 3x EXP buff, are we sure that
/// the EXP gained is actually properly being modified?"* No - `Restores::exp_percent` was
/// parsed, stored and read by nothing, so the coupon was consumed and changed no number.
///
/// This drives the real thing: drink the real `2450001` out of the real generated table, then
/// kill, and count the experience the character actually banked.
#[test]
fn an_exp_coupon_triples_a_kill_and_expires() {
    let path = std::path::Path::new("../../gm-handbook/consumables.txt");
    if !path.exists() {
        return; // generated, gitignored
    }
    let (mut s, store, id) = gm_session();
    let mut mob_exp = std::collections::HashMap::new();
    mob_exp.insert(2u32, 15u32);
    s.config = Arc::new(Config {
        mob_exp,
        consumables: crate::consumables::Consumables::load(path),
        ..(*s.config).clone()
    });
    let exp_now = |store: &Arc<Store>| {
        store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().exp
    };

    // The control: without a coupon a kill is worth its face value, so the assertion below
    // is about the coupon and not about the kill.
    let before = exp_now(&store);
    s.award_experience(s.exp_for_kill(2).0, "a kill", true, false);
    assert_eq!(exp_now(&store) - before, 15, "1x with no coupon");

    // Drink a real 3x coupon. Seeded straight into the bag rather than through `!item`,
    // which refuses any id the test config's (empty) item table does not name.
    store
        .add_item(id, store::InventoryType::Use, &store::Item::bundle(2_450_001, 1), 100)
        .unwrap();
    let out = s.on_use_item(&net::useitem::use_item(0, 1, 2_450_001, 0));
    assert!(
        out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE
            && notice_text(r).contains("3x experience")),
        "the player must be told it started: {out:?}"
    );

    let before = exp_now(&store);
    let (amount, note) = s.with_exp_coupon(s.exp_for_kill(2).0);
    assert_eq!(amount, 45, "15 tripled");
    assert!(note.contains("3x EXP coupon"), "and the log says so: {note}");
    s.award_experience(amount, "a kill", true, false);
    assert_eq!(exp_now(&store) - before, 45, "the character really banked triple");

    // **It ends.** A coupon that never expires is a permanent rate change nobody asked for.
    s.clock_ms += 900_000;
    let (amount, note) = s.with_exp_coupon(s.exp_for_kill(2).0);
    assert_eq!(amount, 15, "back to face value after fifteen minutes");
    assert!(note.is_empty());
}

/// **The coupon and the potion both put an icon top-right, and the coupon's comes down when
/// it stops multiplying.** The owner, 2026-09-16: *"the EXP coupon effects are not applying the
/// appropriate buff icon on the top right of player's screen ... make sure 2x and 3x coupons
/// have the proper buff durations applied"* and *"make sure that Magic Potions and other
/// similar potions are applying the buff icons as well."*
///
/// Two defects, one packet. The potion's `0x007D` carried its item id as a POSITIVE reason,
/// which the client reads as a skill id, and there is no skill 2002001 to draw. The coupon
/// sent no `0x007D` at all. Now: the potion's entry names `-2002001`; the coupon sends CTS
/// 163 `ExpBuffRate` worth its percent for its duration with `-2450001` as the reason; the
/// tick at the coupon's expiry resets 163 in the same breath as the multiplier ends.
#[test]
fn an_exp_coupon_and_a_magic_potion_send_their_stat_with_the_item_as_a_negative_reason() {
    let (mut s, store, id) = gm_session();
    // The two rows as `gm-handbook/consumables.txt` carries them: Magic Potion `mad 10` for
    // ten minutes, the 3x coupon `expBuff 300` for fifteen.
    s.config = Arc::new(Config {
        consumables: crate::consumables::Consumables::parse(
            "2002001, 0, 0, 0, 0, 600000, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
2450001, 0, 0, 0, 0, 900000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 300, 0, 0
",
        ),
        ..(*s.config).clone()
    });
    let potion = store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_002_001, 1), 100).unwrap()[0].slot;
    let coupon = store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_450_001, 1), 100).unwrap()[0].slot;

    let out = s.on_use_item(&net::useitem::use_item(0, potion as i16, 2_002_001, 0));
    let set = first_stat_set(&out).expect("the potion grants a stat");
    let (bits, entries) = stat_set(set);
    assert_eq!(bits, vec![net::buff::CTS_MAGIC_ATTACK]);
    assert_eq!(entries, vec![(10, net::buff::item_reason(2_002_001), 600_000)]);
    assert_eq!(entries[0].1 as i32, -2_002_001, "the reason is the item, negated - a positive one is a skill id");

    let out = s.on_use_item(&net::useitem::use_item(0, coupon as i16, 2_450_001, 0));
    let set = first_stat_set(&out).expect("the coupon now grants a stat, for the icon");
    let (bits, entries) = stat_set(set);
    assert_eq!(bits, vec![net::buff::CTS_EXP_BUFF_RATE], "bit 163, ExpBuffRate, and nothing else");
    assert_eq!(entries, vec![(300, net::buff::item_reason(2_450_001), 900_000)]);
    assert!(s.holds(net::buff::CTS_EXP_BUFF_RATE), "held, so the tick can take it down");
    assert_eq!(s.with_exp_coupon(100).0, 300, "and the multiplier is running");
    assert!(out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE && notice_text(r).contains("3x experience")));

    // Ten minutes on: the potion's 0x007E, and only the potion's - the coupon has five
    // minutes left and is still multiplying.
    s.clock_ms += 600_000;
    let out = s.buff_tick(s.clock_ms);
    let reset = out.iter().find(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET).expect("the potion's icon comes down");
    let cleared = net::buff::bits_in_mask(&reset.body[3..3 + net::buff::MASK_LEN]);
    assert_eq!(cleared, vec![net::buff::CTS_MAGIC_ATTACK]);
    assert!(s.holds(net::buff::CTS_EXP_BUFF_RATE), "the coupon is on its own clock");
    assert_eq!(s.with_exp_coupon(100).0, 300);

    // Fifteen: one 0x007E clearing 163, the multiplier gone in the same instant.
    s.clock_ms += 300_000;
    let out = s.buff_tick(s.clock_ms);
    let reset = out.iter().find(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET).expect("the coupon's icon comes down");
    let cleared = net::buff::bits_in_mask(&reset.body[3..3 + net::buff::MASK_LEN]);
    assert_eq!(cleared, vec![net::buff::CTS_EXP_BUFF_RATE]);
    assert!(!s.holds(net::buff::CTS_EXP_BUFF_RATE));
    assert_eq!(s.with_exp_coupon(100).0, 100, "face value again");
}

/// **A second coupon replaces the first**, rather than stacking or being refused.
///
/// Stacking would make two 2x coupons a 4x, which no version of this game does; refusing
/// would eat the item, which is the worst of the three.
#[test]
fn a_second_exp_coupon_replaces_rather_than_stacks() {
    let path = std::path::Path::new("../../gm-handbook/consumables.txt");
    if !path.exists() {
        return;
    }
    let (mut s, store, id) = gm_session();
    s.config = Arc::new(Config {
        consumables: crate::consumables::Consumables::load(path),
        ..(*s.config).clone()
    });
    for item in [2_450_000u32, 2_450_001] {
        store
            .add_item(id, store::InventoryType::Use, &store::Item::bundle(item, 1), 100)
            .unwrap();
    }
    // The 2x first, then the 3x on top of it.
    s.on_use_item(&net::useitem::use_item(0, 1, 2_450_000, 0));
    assert_eq!(s.with_exp_coupon(100).0, 200, "the 2x is running");
    s.on_use_item(&net::useitem::use_item(0, 2, 2_450_001, 0));
    assert_eq!(s.with_exp_coupon(100).0, 300, "the 3x replaced it - NOT 600");
}

/// `!exp` is NOT multiplied. It means "give me exactly this much".
#[test]
fn the_exp_command_is_not_multiplied() {
    let (mut s, store, id) = gm_session();
    s.handle(&gm_chat("!setrates 10 1 1 1 30"));
    let exp_now = |store: &Arc<Store>| {
        store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().exp
    };
    let before = exp_now(&store);
    s.handle(&gm_chat("!exp 100"));
    let after = exp_now(&store);
    assert_eq!(after - before, 100, "a 10x rate must not touch a debugging command");
}

/// The meso rate multiplies the pile that lands on the floor.
#[test]
fn the_meso_rate_multiplies_a_drop() {
    let (mut s, _, _) = gm_session();
    let drops = crate::droptables::DropTables::parse(
        "2 | 0 | 100 | 10 | 10 | 1 | mesos
",
    );
    s.config = Arc::new(Config { drops, ..(*s.config).clone() });
    s.last_position = Some((520, 395));
    let map = crate::fields::FieldKey::world(net::opcode::START_MAP_ID);

    s.handle(&gm_chat("!setrates 1 3 1 1 30"));
    s.drops_from_kill(2, 2000, Some((500, 395)), 204, map);

    let mesos: Vec<u32> =
        s.fields.with_drops(map, |d| d.on_field(map).map(|x| x.meso).collect::<Vec<_>>());
    assert_eq!(mesos, vec![30], "10 mesos at 3x, on the floor rather than at pick-up time");
}


/// `!rates` lists all three and says so plainly when nothing is running.
#[test]
fn the_rates_command_lists_all_three() {
    let (mut s, _, _) = gm_session();
    let quiet = notice_text(&s.handle(&gm_chat("!rates"))[0]);
    assert!(quiet.contains("EXP 1x"), "{quiet}");
    assert!(quiet.contains("Meso 1x"), "{quiet}");
    assert!(quiet.contains("Drop 1x"), "{quiet}");
    // The two fields added 2026-09-06 - the owner: "!rates should also additionally show both of
    // these new rates". The party share prints as a percent, not a multiplier.
    assert!(quiet.contains("Quest EXP 1x"), "{quiet}");
    assert!(quiet.contains("Party EXP 30%"), "{quiet}");
    assert!(quiet.contains("No event is running"), "a 30% share is not an event: {quiet}");
    // And it is one ordinary chat notice, the same builder as every reply that draws.
    let out = s.handle(&gm_chat("!rates"));
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].opcode, net::notice::CHAT_NOTICE);
    assert_eq!(out[0].body[0], 1, "force = 1, like every notice");
    assert_eq!(u16::from_le_bytes([out[0].body[1], out[0].body[2]]) as usize, out[0].body.len() - 3);

    s.handle(&gm_chat("!setrates 1 1 2.5 1 30"));
    let loud = notice_text(&s.handle(&gm_chat("!rates"))[0]);
    assert!(loud.contains("Drop 2.5x"), "{loud}");
    assert!(!loud.contains("No event is running"), "{loud}");
}

/// `!rates` changes nothing - no banner, no stored rate.
#[test]
fn the_rates_command_is_read_only() {
    let (mut s, store, _) = gm_session();
    s.handle(&gm_chat("!setrates 2 1 1 1 30"));
    let before = store.rates().unwrap();
    let out = s.handle(&gm_chat("!rates"));
    assert!(banners(&out).is_empty(), "reporting is not a change: {out:?}");
    assert_eq!(
        store.rates().unwrap(),
        before,
        "!rates must not touch set_at either"
    );
}

/// The drop rate is set through `!setrates` like the other two, and scrolls nothing.
#[test]
fn the_drop_rate_is_set_through_setrates() {
    let (mut s, store, _) = gm_session();
    let out = s.handle(&gm_chat("!setrates 1 1 4 1 30"));
    assert_eq!(store.rates().unwrap().drop.rate.per_cent(), 400);
    assert!(banners(&out).is_empty(), "{out:?}");

}

/// **An `!announce` survives rate changes**: three events in a row leave the GM's text alone.
#[test]
fn rate_events_leave_the_announcement_alone() {
    let (mut s, _, _) = gm_session();
    s.handle(&gm_chat("!announce Welcome to MapleCW"));
    for rates in ["!setrates 2 1 1 1 30", "!setrates 2 3 1 1 30", "!setrates 2 3 4 1 30"] {
        let out = s.handle(&gm_chat(rates));
        assert!(banners(&out).is_empty(), "{rates}: {out:?}");
    }
    assert!(banners(&s.tick(500)).is_empty(), "still the same text on screen");
}

/// The drop rate scales the CHANCE, and a rate high enough makes an unlikely row certain.
#[test]
fn the_drop_rate_multiplies_the_chance() {
    let (mut s, _, _) = gm_session();
    // A 10% row. At 1x the sample roll below misses it; at 10x it cannot miss.
    let drops = crate::droptables::DropTables::parse("2 | 4000001 | 10 | 1 | 1 | 9 | Shell
");
    s.config = Arc::new(Config { drops, ..(*s.config).clone() });
    s.last_position = Some((520, 395));
    let map = crate::fields::FieldKey::world(net::opcode::START_MAP_ID);

    // 500 000 of 1 000 000 is 50%: above the 10% row (100 000 ppm) and below 100%.
    let entry = &s.config.drops.for_mob(2)[0];
    assert!(!entry.hits_at(500_000, store::rates::Rate::NORMAL), "1x must miss this roll");
    assert!(
        entry.hits_at(500_000, store::rates::Rate::from_per_cent(1_000)),
        "10x makes a 10% row certain"
    );

    s.handle(&gm_chat("!setrates 1 1 10 1 30"));
    s.drops_from_kill(2, 2000, Some((500, 395)), 204, map);
    assert_eq!(
        s.fields.with_drops(map, |d| d.len()),
        1,
        "at 10x a 10% row drops every time"
    );
}

/// **Every kill is counted for the drop-table page**, in memory, with what fell. A template no
/// other test uses, so the shared in-memory counter cannot be disturbed by a parallel test.
#[test]
fn a_kill_and_its_drop_are_counted_for_the_drop_page() {
    const TEMPLATE: u32 = 424_242;
    let (mut s, _, _) = gm_session();
    let drops = crate::droptables::DropTables::parse(&format!("{TEMPLATE} | 4000001 | 100 | 1 | 1 | 0 | Shell\n"));
    s.config = Arc::new(Config { drops, ..(*s.config).clone() });
    let map = crate::fields::FieldKey::world(net::opcode::START_MAP_ID);
    let before = crate::killstats::pending_kills(TEMPLATE);
    s.drops_from_kill(TEMPLATE, 2000, Some((500, 395)), 204, map);
    s.drops_from_kill(TEMPLATE, 2001, Some((500, 395)), 204, map);
    assert_eq!(crate::killstats::pending_kills(TEMPLATE), before + 2);
}

/// A row disabled with a chance of 0 stays disabled at every rate.
///
/// `Rate::apply` floors at 1 so that a 0.5x event cannot zero a 1-exp mob. Applied to a
/// chance that would turn "never" into "1 in 10000", which is a disabled row coming back to
/// life quietly.
#[test]
fn a_zero_chance_row_stays_dead_at_every_rate() {
    let drops = crate::droptables::DropTables::parse("2 | 4000001 | 0 | 1 | 1 | 9 | Shell
");
    let entry = &drops.for_mob(2)[0];
    for per_cent in [100u32, 1_000, 10_000] {
        assert!(
            !entry.hits_at(0, store::rates::Rate::from_per_cent(per_cent)),
            "{per_cent} hundredths revived a disabled row"
        );
    }
}


/// **`!map 45` must be refused, not obeyed.** The owner tried it on 2026-08-20 and the client
/// died; the guard is innocent, and this pins it so nobody has to re-establish that.
///
/// 45 has no field image in this client - `gm-handbook/fields.txt` lists 40 and jumps to the
/// next real map - so `map_exists` is false and the command answers with a refusal.
#[test]
fn the_map_command_refuses_an_id_with_no_field_image() {
    let (mut s, _, _) = gm_session();
    // A loaded field table. Without one `map_exists` is fail-open by design, which is a
    // different branch and has its own test below.
    let fields = [1u32, 40, 104000000].into_iter().collect();
    s.config = Arc::new(Config { fields, ..(*s.config).clone() });
    let before = s.claimed_character().unwrap().map_id;
    for missing in ["45", "999999", "4294967295"] {
        let out = s.handle(&gm_chat(&format!("!map {missing}")));
        let said = notice_text(&out[0]);
        assert!(said.contains("REFUSED"), "!map {missing}: {said}");
        assert_eq!(out.len(), 1, "a refusal is one notice and no field change: {out:?}");
        assert_eq!(
            s.claimed_character().unwrap().map_id,
            before,
            "!map {missing} moved the character"
        );
    }
}

/// **An empty field table refuses too, and says why.**
///
/// `map_exists` is fail-open on an empty table so that a missing generated file does not turn
/// every warp into a refusal. That silently removes the guard, and `gm-handbook/` is generated
/// and gitignored, so it can genuinely be missing. The command must not answer as though it
/// checked something it could not check.
#[test]
fn an_empty_field_table_refuses_and_names_the_reason() {
    let (mut s, _, _) = gm_session();
    assert!(s.config.fields.is_empty(), "the default config has no field table");
    let out = s.handle(&gm_chat("!map 40"));
    let said = notice_text(&out[0]);
    assert!(said.contains("REFUSED"), "{said}");
    assert!(said.contains("field table is empty"), "{said}");
    assert!(said.contains("dump_portals.py"), "it has to say how to fix it: {said}");
}

/// And the shapes that are not numbers at all still answer rather than going silent.
#[test]
fn the_map_command_answers_rubbish_rather_than_ignoring_it() {
    let (mut s, _, _) = gm_session();
    for rubbish in ["", "forty", "-1", "4.5", "40 40"] {
        let out = s.handle(&gm_chat(&format!("!map {rubbish}")));
        assert!(!out.is_empty(), "!map {rubbish:?} answered nothing at all");
        assert_eq!(out[0].opcode, net::notice::CHAT_NOTICE, "!map {rubbish:?}");
    }
}

/// The killer of a mob they did all the damage to gets a WHITE line.
#[test]
fn a_solo_kill_pays_in_full_and_in_white() {
    let (mut s, _, _) = gm_session();
    let shares = vec![crate::fields::DamageShare {
        character: 204,
        dealt: 45,
        total: 45,
        majority: true,
    }];
    let out = s.award_kill_experience(10, "a kill", 204, &shares);
    let msg = out.iter().find(|r| r.opcode == net::message::MESSAGE).expect("an EXP line");
    assert_eq!(msg.body[1], 1, "white = 1 for majority damage");
    assert!(msg.what.contains("WHITE"), "{}", msg.what);
}

/// A player who did not deal the majority gets a fraction, and a YELLOW line.
#[test]
fn a_minority_share_is_a_fraction_and_yellow() {
    let (mut s, _, _) = gm_session();
    let shares = vec![
        crate::fields::DamageShare { character: 999, dealt: 30, total: 45, majority: true },
        crate::fields::DamageShare { character: 204, dealt: 15, total: 45, majority: false },
    ];
    let out = s.award_kill_experience(30, "a kill", 204, &shares);
    let msg = out.iter().find(|r| r.opcode == net::message::MESSAGE).expect("an EXP line");
    assert_eq!(msg.body[1], 0, "white = 0 for a share");
    assert!(msg.what.contains("+10 exp"), "15 of 45 of 30 exp is 10: {}", msg.what);
}


/// **A completion must never replay the quest's opening.** The regression the owner hit on
/// 2026-08-20: they clicked Sera holding quest 1000 and they recited Heena's tutorial.
///
/// Quest 1000 has no `Say.1` - only `1.stop.npc` - so the old fallback chain reached `"0"`.
#[test]
fn completing_a_quest_never_replays_its_opening() {
    let (mut s, _, _) = gm_session();
    let mut q1000 = crate::config::Quest {
        name: "Borrowing Sera's Mirror".to_string(),
        start_npc: Some(1),
        end_npc: Some(2),
        next_quest: Some(1001),
        ..Default::default()
    };
    q1000.say.insert("0".to_string(), vec!["You must be the new traveler.".to_string()]);
    q1000.say.insert("1.stop.npc".to_string(), vec!["Haven't met up with Sera yet?".to_string()]);
    let mut q1001 = crate::config::Quest {
        name: "Bringing a Mirror to Heena".to_string(),
        start_npc: Some(2),
        ..Default::default()
    };
    q1001.say.insert("0".to_string(), vec!["How am I going to hang all these up?".to_string()]);
    let mut quests = std::collections::HashMap::new();
    quests.insert(1000u32, q1000);
    quests.insert(1001u32, q1001);
    s.config = Arc::new(Config { quests, ..(*s.config).clone() });
    s.handle(&quest_request(net::script::QUEST_ACTION_START, 1000, 1));

    let out = s.handle(&quest_request(net::script::QUEST_ACTION_COMPLETE, 1000, 2));
    let said: Vec<String> = out.iter().map(|r| r.what.clone()).collect();
    assert!(
        !said.iter().any(|w| w.contains("new traveler")),
        "Sera replayed Heena's opening: {said:?}"
    );
    assert!(
        said.iter().any(|w| w.contains("quest 1001")),
        "completing 1000 must chain into 1001: {said:?}"
    );
    assert!(
        said.iter().any(|w| w.contains("quest 1000 completed")),
        "and 1000 must be recorded complete: {said:?}"
    );
}

/// **A quest with its own completion lines does NOT repeat them** - the client drew `Say.1`
/// before it sent the turn-in (`FUN_141f0e4c0`, 2026-10-02). It chains nothing, says its `1.yes`
/// when it has one, and otherwise sends the record alone.
#[test]
fn a_quest_with_its_own_completion_lines_uses_them() {
    let (mut s, _, _) = gm_session();
    let mut q = crate::config::Quest {
        name: "Bringing a Mirror to Heena".to_string(),
        start_npc: Some(2),
        next_quest: None,
        ..Default::default()
    };
    q.say.insert("0".to_string(), vec!["How am I going to hang all these up?".to_string()]);
    q.say.insert("1".to_string(), vec!["Oh wow! You brought Sera's mirror!".to_string()]);
    let mut quests = std::collections::HashMap::new();
    quests.insert(1001u32, q);
    s.config = Arc::new(Config { quests, ..(*s.config).clone() });
    s.handle(&quest_request(net::script::QUEST_ACTION_START, 1001, 2));

    let out = s.handle(&quest_request(net::script::QUEST_ACTION_COMPLETE, 1001, 1));
    let said: Vec<String> = out.iter().map(|r| r.what.clone()).collect();
    assert!(!said.iter().any(|w| w.contains("on path \"1\"")), "the client already said Say.1: {said:?}");
    assert!(!out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE), "no yes branch, so no box: {said:?}");
    assert!(said.iter().any(|w| w.contains("quest 1001 completed")), "{said:?}");

    // With a `1.yes`, that is the answer - once.
    let mut q = s.config.quests[&1001].clone();
    q.say.insert("1.yes".to_string(), vec!["Thank you!".to_string()]);
    let mut quests = std::collections::HashMap::new();
    quests.insert(1002u32, q);
    s.config = Arc::new(Config { quests, ..(*s.config).clone() });
    s.handle(&quest_request(net::script::QUEST_ACTION_START, 1002, 2));
    let out = s.handle(&quest_request(net::script::QUEST_ACTION_COMPLETE, 1002, 1));
    let boxes: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).collect();
    assert_eq!(boxes.len(), 1, "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(boxes[0].what.contains("on path \"1.yes\""), "{}", boxes[0].what);
}

/// `0x0151` body: u8 action, u32 questId, u32 npcTemplateId, then a tail we do not read.
fn quest_request(action: u8, quest_id: u32, npc: u32) -> Vec<u8> {
    let mut b = net::script::CLIENT_QUEST_REQUEST.to_le_bytes().to_vec();
    b.push(action);
    b.extend_from_slice(&quest_id.to_le_bytes());
    b.extend_from_slice(&npc.to_le_bytes());
    b.extend_from_slice(&[0u8; 8]);
    b
}


/// **A full Equip tab must not stop an Etc pick-up.** The owner, 2026-08-20: *"Even when my Equip
/// tab is full, I should still be allowed to pick up items that belong to other tabs since
/// they each have 30 slots and can be expanded independently."*
///
/// Every tab has its own slot column and its own capacity check, so this is a regression
/// guard rather than a fix - if it ever fails, the per-tab accounting has been collapsed.
#[test]
fn a_full_equip_tab_does_not_block_the_other_bags() {
    let (s, store, id) = gm_session();
    let equip_slots = store.inventory_slots(id, store::InventoryType::Equip).unwrap();
    for _ in 0..equip_slots {
        store
            .add_item(id, store::InventoryType::Equip, &store::Item::equip(1302000), 1)
            .expect("filling the equip tab");
    }
    assert!(
        store.free_slot(id, store::InventoryType::Equip).unwrap().is_none(),
        "the equip tab must actually be full for this test to mean anything"
    );

    for (inv, item) in [
        (store::InventoryType::Etc, store::Item::bundle(4000001, 1)),
        (store::InventoryType::Use, store::Item::bundle(2000000, 1)),
    ] {
        let placed = store.add_item(id, inv, &item, 100);
        assert!(placed.is_ok(), "a full Equip tab blocked {inv:?}: {placed:?}");
    }
    let _ = s;
}

/// **The bag and the meso balance are re-sent on field entry.** They were never lost - the
/// client was simply never told, because the record cannot carry the non-equip bags and the
/// stat block has no meso field.
#[test]
fn entering_a_field_restores_the_bag_and_the_mesos() {
    let (mut s, store, id) = gm_session();
    store.add_item(id, store::InventoryType::Etc, &store::Item::bundle(4000001, 3), 100).unwrap();
    store.add_mesos(id, 1234).unwrap();

    let out = s.on_field_entered();
    let adds: Vec<&Reply> = out
        .iter()
        .filter(|r| r.opcode == net::inventory::INVENTORY_OPERATION && r.what.contains("restored"))
        .collect();
    assert_eq!(adds.len(), 1, "the Etc stack should be re-sent: {out:?}");

    let meso = out
        .iter()
        .find(|r| r.what.contains("mesos restored"))
        .expect("a meso balance should be sent");
    assert_eq!(meso.opcode, net::stats::STAT_CHANGED);
    assert!(meso.what.contains("1234"), "{}", meso.what);
}

/// Equips are NOT re-sent: they ride the character record, and a second copy would double
/// every item in the tab.
#[test]
fn the_equip_tab_is_not_restored_twice() {
    let (mut s, store, id) = gm_session();
    store.add_item(id, store::InventoryType::Equip, &store::Item::equip(1302000), 1).unwrap();
    let out = s.on_field_entered();
    assert!(
        !out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION
            && r.what.contains("restored")),
        "an equip was re-sent on top of the record: {out:?}"
    );
}


/// **Accepting a quest hands over what `Act.0.item` promises.** The owner, 2026-08-20: Sera talked
/// and gave nothing, and quest 1001 cannot be completed without the mirror it was supposed to
/// hand over.
#[test]
fn accepting_a_quest_hands_over_its_act_items() {
    let (mut s, store, id) = gm_session();
    let mut q = crate::config::Quest {
        name: "Bringing a Mirror to Heena".to_string(),
        start_npc: Some(2),
        start_items: vec![(4031000, 1)],
        ..Default::default()
    };
    q.say.insert("0".to_string(), vec!["Fine, fine, here's the mirror.".to_string()]);
    let mut quests = std::collections::HashMap::new();
    quests.insert(1001u32, q);
    s.config = Arc::new(Config { quests, ..(*s.config).clone() });

    s.handle(&quest_request(net::script::QUEST_ACTION_START, 1001, 2));

    let etc = store.bag_items(id, store::InventoryType::Etc).unwrap();
    assert_eq!(etc.len(), 1, "the mirror should be in the Etc bag: {etc:?}");
    assert_eq!(etc[0].item.item_id, 4031000);
}

/// A negative count is a TAKE, and taking is not wired - it must not be mistaken for a give.
#[test]
fn a_negative_act_count_gives_nothing() {
    let (mut s, store, id) = gm_session();
    let mut q = crate::config::Quest { start_npc: Some(2), start_items: vec![(4031000, -1)], ..Default::default() };
    q.say.insert("0".to_string(), vec!["...".to_string()]);
    let mut quests = std::collections::HashMap::new();
    quests.insert(1001u32, q);
    s.config = Arc::new(Config { quests, ..(*s.config).clone() });

    s.handle(&quest_request(net::script::QUEST_ACTION_START, 1001, 2));
    assert!(store.bag_items(id, store::InventoryType::Etc).unwrap().is_empty());
}

/// The real `questlines.txt` parses into the grant, so the fixture above is not the only
/// thing being tested.
#[test]
fn the_real_quest_file_carries_seras_mirror() {
    let path = std::path::Path::new("../../gm-handbook/questlines.txt");
    if !path.exists() {
        return; // gm-handbook is generated and gitignored; skip rather than fail a fresh clone
    }
    let quests = crate::config::load_quests(path);
    let q = quests.get(&1001).expect("quest 1001 is in this client");
    assert_eq!(q.start_items, vec![(4031000, 1)], "Act.0.item.0 for quest 1001");
    assert_eq!(quests.get(&1000).and_then(|q| q.next_quest), Some(1001));
}


/// **`slotMax` of 0 means "unspecified", not "one per slot".** The rule that decides whether
/// Garnet Ore stacks, pinned in the one place it lives.
#[test]
fn an_absent_slot_max_is_unspecified_not_one() {
    let shops = crate::ShopTable::default();
    // Nothing in item_data at all - the state a server with no gm-handbook/ is in.
    assert_eq!(shops.max_stack(4020000), 100, "an Etc item with no slotMax stacks");
    assert_eq!(shops.max_stack(2000000), 100, "so does a Use item");
    assert_eq!(shops.max_stack(1302000), 1, "an equip does NOT - the one case 0 really is 1");
}


/// `!setrates` sets all three, on one timestamp, and scrolls nothing.
#[test]
fn setrates_sets_all_three_on_one_timestamp() {
    let (mut s, store, _) = gm_session();
    let out = s.handle(&gm_chat("!setrates 2 3 5 1 30"));
    let r = store.rates().unwrap();
    assert_eq!(r.exp.rate.per_cent(), 200);
    assert_eq!(r.meso.rate.per_cent(), 300);
    assert_eq!(r.drop.rate.per_cent(), 500);
    assert_eq!(
        r.exp.set_at, r.drop.set_at,
        "one timestamp for everything that moved"
    );
    assert!(banners(&out).is_empty(), "{out:?}");
}

/// `!setrates 1 1 1` is the one-command way to end everything.
#[test]
fn setrates_all_ones_ends_every_event() {
    let (mut s, store, _) = gm_session();
    s.handle(&gm_chat("!setrates 2 3 5 1 30"));
    let out = s.handle(&gm_chat("!setrates 1 1 1 1 30"));
    assert!(store.rates().unwrap().all_normal());
    assert!(banners(&out).is_empty(), "{out:?}");
}

/// Below 1x is refused, and **nothing is written** - not even the values that were valid.
#[test]
fn setrates_refuses_below_one_and_writes_nothing() {
    let (mut s, store, _) = gm_session();
    s.handle(&gm_chat("!setrates 2 2 2 1 30"));
    for bad in ["0.5 3 5 1 30", "2 0.99 5 1 30", "2 3 0 1 30", "2 3 5 0.5 30"] {
        let out = s.handle(&gm_chat(&format!("!setrates {bad}")));
        assert!(banners(&out).is_empty(), "{bad} moved the banner: {out:?}");
        let r = store.rates().unwrap();
        assert_eq!(
            (r.exp.rate.per_cent(), r.meso.rate.per_cent(), r.drop.rate.per_cent()),
            (200, 200, 200),
            "{bad} applied part of itself"
        );
    }
}

/// Exactly 1 is fine; it is only *below* 1 that this command refuses.
#[test]
fn setrates_accepts_one_and_fractions_above_it() {
    let (mut s, store, _) = gm_session();
    s.handle(&gm_chat("!setrates 1 1.5 2 1 30"));
    let r = store.rates().unwrap();
    assert_eq!(
        (r.exp.rate.per_cent(), r.meso.rate.per_cent(), r.drop.rate.per_cent()),
        (100, 150, 200)
    );
}

/// The wrong number of arguments says what it wanted rather than guessing.
#[test]
fn setrates_wants_exactly_five() {
    let (mut s, store, _) = gm_session();
    for bad in ["", "2", "2 3", "2 3 5", "2 3 5 7", "2 3 5 7 30 1"] {
        let out = s.handle(&gm_chat(&format!("!setrates {bad}")));
        assert!(notice_text(&out[0]).contains("wants 5 fields"), "{}", notice_text(&out[0]));
        assert!(store.rates().unwrap().all_normal(), "{bad:?} changed something");
    }
}


/// **`!setrates` refuses below 1x in every position, and writes nothing when it does.** The owner,
/// 2026-08-20: *"only accept 1 or above"* - and since the per-kind setters were removed on
/// 2026-09-06 this is the one place the floor lives, so it is checked for each of the three.
#[test]
fn setrates_refuses_below_one_in_every_position() {
    let (mut s, store, _) = gm_session();
    for position in 0..4 {
        for bad in ["0.5", "0.99", "0.01"] {
            let mut words = ["1", "1", "1", "1", "30"];
            words[position] = bad;
            let cmd = format!("!setrates {}", words.join(" "));
            let out = s.handle(&gm_chat(&cmd));
            let said = notice_text(&out[0]);
            assert!(said.contains("below 1x"), "{cmd}: {said}");
            assert!(banners(&out).is_empty(), "{cmd} moved the banner");
        }
    }
    assert!(store.rates().unwrap().all_normal(), "a refused rate must write nothing");
}

/// 1 and above still work on every setter - the floor is inclusive.
#[test]
fn setrates_accepts_one_and_above_on_every_kind() {
    let (mut s, store, _) = gm_session();
    s.handle(&gm_chat("!setrates 1.5 2 1 1 30"));
    let r = store.rates().unwrap();
    assert_eq!(
        (r.exp.rate.per_cent(), r.meso.rate.per_cent(), r.drop.rate.per_cent()),
        (150, 200, 100)
    );
}

/// The floor is a rule about what may be TYPED, not about what a rate can be.
///
/// `store::rates::Rate` still represents fractions, because the drop rate multiplies a chance
/// and that arithmetic has no business knowing what a chat command accepts. If this ever
/// fails, the policy has leaked into the type.
#[test]
fn the_rate_type_still_represents_fractions() {
    let half = store::rates::Rate::from_per_cent(50);
    assert_eq!(half.apply(100), 50);
    assert!(half < store::rates::Rate::NORMAL);
}


/// **The Tutorial Jr. Sentinel always drops its Shellpiece and nothing else.** The owner,
/// 2026-08-21: *"It's the only mob that has this exception."*
///
/// A tutorial kill that hands you the item the tutorial then asks you to have. Anything under
/// 100% is a tutorial that sometimes cannot be finished, and a meso row would be one more
/// thing on the floor during the step that teaches picking things up.
#[test]
fn the_tutorial_sentinel_always_drops_its_shellpiece() {
    let path = std::path::Path::new("../../data/drops.txt");
    let table = crate::droptables::DropTables::load(path);
    if table.total_entries() == 0 {
        return; // no data file in this checkout
    }
    let rows = table.for_mob(1);
    assert_eq!(rows.len(), 1, "template 1 drops exactly one thing: {rows:?}");
    assert_eq!(rows[0].item_id, 4000000, "Jr. Sentinel Shellpiece");
    assert_eq!(rows[0].chance_ppm, crate::droptables::PER_MILLION, "100%, not the scraped 40%");
    // **The global table is no longer empty**, and this used to assert that it was. The owner put
    // the three scrolls in it on 2026-09-09. The concern behind the old assertion still
    // stands - an extra item on the floor during the step that teaches picking things up -
    // so this bounds it instead of forbidding it: every global row must be exactly the scroll
    // rate. That was 1 basis point until `c0f1d87` raised it to 50 (0.5%); the owner, 2026-09-25:
    // *"Accept the new drop rate."* Two rows at 0.5% put an extra scroll on the tutorial floor
    // about once in 100 kills. A row above the rate the scroll NPC quotes still fails here.
    // Since 2026-10-01 the global rows are the four backported scrolls, 1 in 500 at most.
    for row in table.global() {
        assert!(
            crate::scrolls::GLOBAL_SCROLLS.contains(&(row.item_id, row.chance_ppm)),
            "a global row that is not one of the owner's four would land on the tutorial floor: {row:?}"
        );
    }

    // And it really does hit on every roll, not just at a value that looks like 100.
    for roll in [0u64, 1, 4_999, 9_999, u64::MAX] {
        assert!(rows[0].hits(roll), "missed at roll {roll}");
    }
}

/// **A quest can be turned in twice, and the second time pays out again.**
///
/// The owner, 2026-08-21: *"I was able to complete the Heena quest multiple times, this is not
/// okay."*
///
/// `store::complete_quest` was never the problem - it guards on `state = InProgress` and
/// correctly refuses. What ignored the refusal was the caller: `apply_quest_completion_rewards`
/// sat *outside* the match, so a repeat turn-in re-paid the EXP and re-ran the item rows,
/// while the journal row and the fanfare - which ARE gated - stayed correct. That is why the
/// existing turn-in test passed throughout: it counted fanfares, and the fanfare was the one
/// effect that was right.
///
/// Quest 1001's `Act.1` is `exp 2` and `item 4031000 count -1`, so the visible symptom is
/// two experience per click, for as many clicks as the player likes.
#[test]
fn turning_a_quest_in_twice_pays_out_only_once() {
    let path = std::path::Path::new("../../gm-handbook/questlines.txt");
    if !path.exists() {
        return; // generated data, gitignored
    }
    let config = Config { quests: crate::config::load_quests(path), ..Config::default() };
    // The reward this test is about, read from the data rather than hard-coded - if the
    // dump changes, the test should follow it or fail loudly, not quietly measure nothing.
    assert_eq!(config.quests[&1001].complete_exp, 2, "quest 1001 Act.1.exp");

    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "TwiceOver".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);

    let exp_now = |s: &Session| {
        s.store.characters_for(account_id, 0).unwrap().into_iter()
            .find(|c| c.id == id).unwrap().exp
    };
    let before = exp_now(&s);

    // Accept 1001 from Heena, then turn it in. Action 1 = accept, action 2 = complete.
    s.on_quest_request(&hex("01e9030000010000000c046d0100000000"));
    let first = s.on_quest_request(&hex("02e90300000200000043ffe501ffffffff"));
    assert!(!first.is_empty(), "every 0x0151 is answered");
    let after_first = exp_now(&s);
    assert_eq!(after_first, before + 2, "the real turn-in pays Act.1.exp");

    // Click the NPC again with the exact same packet. The journal already says complete.
    let second = s.on_quest_request(&hex("02e90300000200000043ffe501ffffffff"));
    assert!(!second.is_empty(), "still answered - an unanswered packet freezes the UI");
    assert_eq!(
        exp_now(&s),
        after_first,
        "a second turn-in must pay NOTHING - complete_quest already refused it"
    );

    // And nothing on the wire should claim otherwise. No journal record, no fanfare, and
    // no experience line: a repeat click is a conversation, not an event.
    assert!(
        !second.iter().any(|r| r.opcode == net::quest::MESSAGE),
        "no journal record for a quest that did not change state"
    );
    assert!(
        !second
            .iter()
            .any(|r| r.opcode == net::questeffect::USER_EFFECT_LOCAL
                && r.body.first() == Some(&net::questeffect::EFFECT_QUEST_CLEAR)),
        "no fanfare"
    );
}

/// **The same hole on the accept side, which is how the mirror was farmable.**
///
/// `record_quest_start` called `grant_quest_start_items` and `apply_quest_hp` outside the
/// match on `start_quest`, so clicking Accept on a quest already held - or already finished -
/// handed the items over again and re-applied the HP.
///
/// Quest 1001's `Act.0` gives Sera's Mirror. Roger's 1002 gives an apple *and* sets HP to 25,
/// so the same bug lets a player refill on apples and re-cripple themselves at will.
#[test]
fn accepting_a_quest_twice_grants_its_items_only_once() {
    let path = std::path::Path::new("../../gm-handbook/questlines.txt");
    if !path.exists() {
        return; // generated data, gitignored
    }
    let config = Config { quests: crate::config::load_quests(path), ..Config::default() };
    assert_eq!(
        config.quests[&1001].start_items.iter().find(|(i, _)| *i == 4031000).map(|(_, c)| *c),
        Some(1),
        "quest 1001 Act.0 gives Sera's Mirror"
    );

    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "TwoMirrors".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);

    let mirrors = |s: &Session| -> i64 {
        s.store
            .bag(id)
            .unwrap()
            .items_in(store::InventoryType::Etc)
            .filter(|i| i.item.item_id == 4031000)
            .map(|i| i64::from(i.item.kind.quantity()))
            .sum()
    };

    s.on_quest_request(&hex("01e9030000010000000c046d0100000000"));
    assert_eq!(mirrors(&s), 1, "the accept hands the mirror over");

    s.on_quest_request(&hex("01e9030000010000000c046d0100000000"));
    assert_eq!(mirrors(&s), 1, "accepting again must NOT hand over a second one");
}

/// **The whole loop, end to end: finish it, give it up, take it again.**
///
/// The owner, 2026-08-21: *"I was able to complete the Heena quest multiple times, this is not
/// okay."* Three separate holes made that possible and any one of them is enough on its own,
/// which is why this test exercises the sequence rather than the pieces:
///
/// 1. `record_quest_complete` paid `Act.1` outside the match on `complete_quest`, so a repeat
///    turn-in re-paid the experience.
/// 2. `record_quest_start` granted `Act.0` outside the match on `start_quest`, so a repeat
///    accept re-handed the items.
/// 3. `forget_quest` deleted a row of any state, so give-up on a *finished* quest put the
///    character back to never having touched it - and then 1 and 2 were not even needed.
///
/// The store was the authority for all three and was answering correctly the whole time. What
/// was wrong is that the callers asked and then did the work anyway.
#[test]
fn a_finished_quest_cannot_be_farmed_by_giving_it_up_and_taking_it_again() {
    let path = std::path::Path::new("../../gm-handbook/questlines.txt");
    if !path.exists() {
        return; // generated data, gitignored
    }
    let config = Config { quests: crate::config::load_quests(path), ..Config::default() };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "NoFarming".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);

    let exp_now = |s: &Session| {
        s.store.characters_for(account_id, 0).unwrap().into_iter()
            .find(|c| c.id == id).unwrap().exp
    };
    let mirrors = |s: &Session| -> i64 {
        s.store.bag(id).unwrap()
            .items_in(store::InventoryType::Etc)
            .filter(|i| i.item.item_id == 4031000)
            .map(|i| i64::from(i.item.kind.quantity()))
            .sum()
    };

    // Accept 1001, which hands over Sera's Mirror, then turn it in: +2 EXP, mirror taken.
    s.on_quest_request(&hex("01e9030000010000000c046d0100000000"));
    assert_eq!(mirrors(&s), 1, "Act.0 hands the mirror over");
    s.on_quest_request(&hex("02e90300000200000043ffe501ffffffff"));
    assert_eq!(exp_now(&s), 2, "Act.1 pays two experience");
    assert_eq!(mirrors(&s), 0, "and Heena keeps the mirror");

    // Now the loop. Give it up - action 3, a five-byte body - and take it again.
    for round in 0..3 {
        s.on_quest_request(&hex("03e9030000"));
        assert_eq!(
            s.store.quest_row(id, 1001).unwrap().map(|r| r.state),
            Some(store::QuestState::Complete),
            "round {round}: a completed quest is not given up"
        );
        s.on_quest_request(&hex("01e9030000010000000c046d0100000000"));
        s.on_quest_request(&hex("02e90300000200000043ffe501ffffffff"));
        assert_eq!(exp_now(&s), 2, "round {round}: still two experience, never four");
        assert_eq!(mirrors(&s), 0, "round {round}: and no second mirror");
    }

    // The journal must still read finished, not started - which is the other half of why
    // re-sending `quest_accepted` for a completed quest is wrong.
    let book = s.store.quest_book(id).unwrap();
    assert_eq!(book.completed.len(), 1, "one completion");
    assert!(book.started.is_empty(), "and nothing back in the started list");
}

/// Opcode then body, the way the framer hands it to `Session::handle`.
fn skill_packet(body_hex: &str) -> Vec<u8> {
    let mut p = net::skills::CLIENT_USER_SKILL_UP_REQUEST.to_le_bytes().to_vec();
    p.extend_from_slice(&hex(body_hex));
    p
}

/// **Bulk skill points: the request carries a count and it used to be ignored.**
///
/// The owner, 2026-08-21: *"I just tried to bulk add 3 points into Three Snails, but it only went
/// up 1 point."* The capture says the client asked for three -
/// `<- 0x013B 940e8711 e8030000 03000000` is tick, skill 1000, count **3** - and the handler
/// did `level + 1`.
///
/// The body is the real one from `world.log`, not a hand-built one, so this test would have
/// failed on the day the bug shipped.
#[test]
fn a_bulk_skill_request_spends_every_point_it_asks_for() {
    let (mut s, _store, id) = gm_session();

    // The owner's own packet: tick 0x1187_0e94, skill 1000, count 3.
    let out = s.handle(&skill_packet("940e8711e803000003000000"));
    assert!(
        out.iter().any(|r| r.opcode == net::skills::CHANGE_SKILL_RECORD_RESULT),
        "always answered - the reply is what clears the latch"
    );
    assert_eq!(
        s.store.skill_level(id, 1000).unwrap_or(0),
        3,
        "three points asked for, three granted"
    );
}

/// And it clamps, because nothing on this socket authenticates.
///
/// `SkillUpRequest::count`'s doc says the client computes `min(sp, maxLevel - level)` and
/// clamps before sending - *"but the server must clamp again: nothing here authenticates, and
/// nothing stops a crafted body carrying any number at all."* The cap is `masterLevel` and
/// the length of the `level` table in this client's own `Skill.wz`, both of which say 3.
#[test]
fn a_skill_request_is_clamped_to_the_level_table_and_then_refused() {
    let (mut s, _store, id) = gm_session();

    // A body that asks for four billion points.
    let greedy = skill_packet("940e8711e8030000ffffffff");
    let out = s.handle(&greedy);
    assert!(!out.is_empty(), "still answered");
    assert_eq!(
        s.store.skill_level(id, 1000).unwrap_or(0),
        net::skills::BEGINNER_SKILL_MAX_LEVEL,
        "clamped to the maximum the client's own Skill.wz describes, not to the count"
    );

    // Already at the top: refused, and the level does not move.
    let again = s.handle(&greedy);
    assert!(!again.is_empty(), "a refusal is still a reply - the latch has to clear");
    assert_eq!(
        s.store.skill_level(id, 1000).unwrap_or(0),
        net::skills::BEGINNER_SKILL_MAX_LEVEL,
        "and nothing moved"
    );

    // A count of zero is the client asking for nothing. Answer, change nothing.
    let (mut s2, _store2, id2) = gm_session();
    let zero = s2.handle(&skill_packet("940e8711e803000000000000"));
    assert!(!zero.is_empty(), "answered");
    assert_eq!(s2.store.skill_level(id2, 1000).unwrap_or(0), 0, "and nothing was granted");
}

/// **Dying opens the revive dialog, and it opens exactly once.**
///
/// The owner, 2026-08-21: *"My HP hit 0, I see the tombstone on my character, but I do not see the
/// revive confirmation."* The tombstone was always working - `hp = 0` disables the player
/// through ~65 client sites - and the dialog needs `0x0315`, which the client never sends
/// itself.
///
/// Two things are asserted that a "does it send the packet" test would miss:
///
/// * **Order.** The client gates `0x0315` on its own copy of the HP the server just wrote, so
///   a `0x0315` that overtakes the `0x007C` is dropped in silence. The `0x007C` must come
///   first in the batch.
/// * **Once.** A dead character can still be hit. Gating on `hp == 0` rather than on the
///   transition would re-open the dialog on every subsequent hit.
#[test]
fn dying_opens_the_revive_dialog_once_and_after_the_stat_change() {
    let (mut s, store, id) = gm_session();
    let mut chr = s.claimed_character().unwrap();
    // One HP. `gm_session`'s config carries no `mob_attack` table, so `incoming_damage_for`
    // returns None and the server keeps the client's number - which in this captured body is
    // 1. That fallback is the documented behaviour for an unknown template, and using it here
    // keeps the test about the DEATH TRANSITION rather than about the damage formula, which
    // has its own tests.
    chr.hp = 1;
    chr.max_hp = 200;
    store.save_character_progress(&chr).unwrap();

    // A snail hit. The server computes its own damage, which for template 2 exceeds 3.
    // The same captured 0x00E5 the other hit tests use: attack index -1, template 2, and the
    // client's damage of 1, which the server overrides from the template's PADamage of 3.
    let body = hex(
        "00000000ffffffff0100000002002100431e140f0000000000000000000001000000010000000100000001000000d3070000d307000001000000000000000000000000000000000000de0100008b010000000000000000000000000000ffffffff00000000ffffffff000000000000000002000000000000000000000000000000000000000100000000000000000000000000",
    );
    let killing = s.on_user_hit(&body);
    let stat = killing
        .iter()
        .position(|r| r.opcode == net::stats::STAT_CHANGED)
        .expect("the bar update");
    let dialog = killing
        .iter()
        .position(|r| r.opcode == net::revive::SHOW_REVIVE_DIALOG)
        .expect("death opens the revive dialog");
    assert!(
        dialog > stat,
        "0x0315 must FOLLOW the 0x007C - the client drops it while it still thinks HP is positive"
    );
    assert_eq!(killing[dialog].body, net::revive::show_revive_dialog());
    assert_eq!(reload(&s, 1, id).hp, 0, "and the character is dead");

    // Hit again while dead. The bar still updates; the dialog must not re-open.
    let again = s.on_user_hit(&body);
    assert!(
        !again.iter().any(|r| r.opcode == net::revive::SHOW_REVIVE_DIALOG),
        "the dialog opens on the TRANSITION, not on the state - a dead character can still be hit"
    );
}

/// **Reviving: town, 50 HP, and the experience penalty.**
///
/// The owner: *"Reviving a character should warp them to the nearest town, start at 50 HP, and
/// reduce their EXP by 10% unless they are level 10 or below."*
///
/// The transfer request is the real one the town button sends - `targetField = 0`, empty
/// portal name - which is why the branch is on the server's own HP and not on the packet:
/// read as an ordinary portal walk, `0` is a perfectly good map id and the character would be
/// warped to **map 0**, which this client has no field image for.
#[test]
fn reviving_warps_to_town_restores_fifty_hp_and_charges_the_penalty() {
    let path = std::path::Path::new("../../gm-handbook/returnmaps.txt");
    if !path.exists() {
        return; // generated data, gitignored
    }
    let (mut s, store, id) = gm_session();
    {
        let cfg = Arc::get_mut(&mut s.config).expect("sole owner in this test");
        cfg.revive_maps = crate::config::Config::load_revive_maps(path);
    }
    // Map 40 returns to 60, Southperry - not a neighbouring screen, which is the point of
    // using the table rather than walking portals.
    let want_town = s.config.revive_field(40).expect("map 40 has a revive destination");
    assert_eq!(want_town, 60, "Snail Hunting Ground I -> Southperry");

    let mut chr = s.claimed_character().unwrap();
    chr.map_id = 40;
    chr.level = 20;
    chr.exp = 1_000;
    chr.hp = 0;
    chr.max_hp = 200;
    store.save_character_progress(&chr).unwrap();
    // save_character_progress does NOT persist the map - that is set_character_map's column.
    store.set_character_map(id, 40).unwrap();

    // The town button: 25 bytes, target 0, empty portal name.
    let mut body = net::opcode::SET_FIELD.to_le_bytes().to_vec(); // any 2-byte head
    body.clear();
    body.extend_from_slice(&[0u8; 16]); // the client integrity block, unread
    body.extend_from_slice(&0u32.to_le_bytes()); // targetField = 0
    body.extend_from_slice(&0u16.to_le_bytes()); // empty portal name
    assert_eq!(body.len(), 22, "the revive shape: no position follows an empty name");

    let out = s.on_transfer_field(&body);
    assert!(
        out.iter().any(|r| r.opcode == net::opcode::SET_FIELD),
        "always answered"
    );

    let after = reload(&s, 1, id);
    assert_eq!(after.map_id, 60, "warped to town, NOT to map 0");
    assert_eq!(after.hp, 50, "back at 50 HP");
    assert_eq!(after.exp, 900, "10% of 1000 at level 20");

    // And the 0x007C after the SetField, without which ~65 client action gates stay shut.
    let stat = out
        .iter()
        .position(|r| r.opcode == net::stats::STAT_CHANGED)
        .expect("the HP has to be restored in the client's own copy too");
    let field = out
        .iter()
        .position(|r| r.opcode == net::opcode::SET_FIELD)
        .expect("the SetField");
    assert!(stat > field, "the stat change goes AFTER the SetField");
}

/// A level-10 character pays nothing, and still revives.
#[test]
fn a_low_level_character_revives_without_an_experience_penalty() {
    let path = std::path::Path::new("../../gm-handbook/returnmaps.txt");
    if !path.exists() {
        return;
    }
    let (mut s, store, id) = gm_session();
    {
        let cfg = Arc::get_mut(&mut s.config).expect("sole owner in this test");
        cfg.revive_maps = crate::config::Config::load_revive_maps(path);
    }
    let mut chr = s.claimed_character().unwrap();
    chr.map_id = 40;
    chr.level = 10;
    chr.exp = 500;
    chr.hp = 0;
    store.save_character_progress(&chr).unwrap();
    store.set_character_map(id, 40).unwrap();

    let mut body = vec![0u8; 16];
    body.extend_from_slice(&0u32.to_le_bytes());
    body.extend_from_slice(&0u16.to_le_bytes());
    s.on_transfer_field(&body);

    let after = reload(&s, 1, id);
    assert_eq!(after.exp, 500, "'level 10 or below' is inclusive");
    assert_eq!(after.map_id, 60, "and they still get to town");
}

/// **Logging in dead must still offer the way out.**
///
/// The owner, 2026-08-22: *"My character 'Idiot' has 0 HP from last time, and I don't see any
/// revive dialogue when I login because I immediately spawned in dead."*
///
/// `on_user_hit` opens the dialog on the **transition** alive -> dead, which is correct there
/// and is exactly what stops a dead character being re-prompted on every further hit. Logging
/// in dead is not a transition, so nothing fired and the character was stranded with `!heal`
/// as the only escape. Death has to be recoverable from both directions or it is a trap.
///
/// This asserts the ordering too: the `0x007C` restating `hp = 0` must precede the `0x0315`,
/// because the dialog's handler tests the client's own HP copy and drops the packet silently
/// if it is still positive.
#[test]
fn entering_a_field_already_dead_opens_the_revive_dialog() {
    let (mut s, store, _id) = gm_session();
    let mut chr = s.claimed_character().unwrap();
    chr.hp = 0;
    chr.max_hp = 194;
    store.save_character_progress(&chr).unwrap();

    let out = s.on_field_entered();
    let stat = out
        .iter()
        .position(|r| {
            r.opcode == net::stats::STAT_CHANGED
                && r.body == net::stats::StatChange::hp_only(0).build()
        })
        .expect("the hp = 0 restatement");
    let dialog = out
        .iter()
        .position(|r| r.opcode == net::revive::SHOW_REVIVE_DIALOG)
        .expect("a character who logs in dead is offered the dialog");
    assert!(stat < dialog, "the 0x007C must precede the 0x0315 or it is dropped silently");
    assert_eq!(out[dialog].body, net::revive::show_revive_dialog());
}

/// And a living character entering a field is offered nothing.
///
/// The control for the test above: if `on_field_entered` sent the dialog unconditionally this
/// would pass anyway, and every map change would pop a revive box.
#[test]
fn entering_a_field_alive_offers_no_revive_dialog() {
    let (mut s, store, _id) = gm_session();
    let mut chr = s.claimed_character().unwrap();
    chr.hp = 50;
    chr.max_hp = 194;
    store.save_character_progress(&chr).unwrap();

    let out = s.on_field_entered();
    assert!(
        !out.iter().any(|r| r.opcode == net::revive::SHOW_REVIVE_DIALOG),
        "a living character must not be asked whether to revive"
    );
}

/// **Clicking Mr. Kim opens the storage box.**
///
/// The owner, 2026-08-22: *"Mr. Kim the storage keeper does not open the storage UI."* They did not,
/// because nothing had ever sent a packet - `crates/store` has had the whole storage layer for
/// days with no caller anywhere.
///
/// The packet must carry the **template** id, not the object id: the client loads `Npc.wz`
/// from it to find the deposit fee, so an object id charges the wrong fee or none.
#[test]
fn a_storage_keeper_opens_a_box_instead_of_talking() {
    let (mut s, _store, _id) = gm_session();

    let out = s.open_storage_for(105).expect("105 is Mr. Kim, a storage keeper");
    let open = out
        .iter()
        .find(|r| r.opcode == net::storage::STORAGE_RESULT)
        .expect("a 0x0572 goes out");
    assert_eq!(open.body[0], net::storage::RESULT_OPEN, "mode 24");
    assert_eq!(
        u32::from_le_bytes(open.body[1..5].try_into().unwrap()),
        105,
        "the TEMPLATE id - an object id here would charge the wrong fee"
    );
    // An empty box with every gate on: 1 mode + 4 template + 115 block.
    assert_eq!(open.body.len(), 120);

    // And an NPC that is not a keeper is left alone, or every NPC would open a box.
    assert!(s.open_storage_for(2).is_none(), "Sera is not a storage keeper");
}

/// **The wire's meso sign is the opposite of the store's, and getting it backwards would
/// quietly move money the wrong way.**
///
/// `0x00F6` mode 7 carries one signed `i64` where **positive withdraws** from the box.
/// `store::move_storage_mesos` takes positive to mean **deposit**. The session negates
/// between them, and nothing about an inverted version would error, crash or look wrong in a
/// log - it would just move the money the other way.
#[test]
fn depositing_and_withdrawing_mesos_move_the_money_the_right_way() {
    let (mut s, store, id) = gm_session();
    store.set_mesos(id, 5_000).unwrap();
    let account_id = 1i64;

    // Negative on the wire = DEPOSIT into the box.
    let mut body = vec![7u8];
    body.extend_from_slice(&(-2_000i64).to_le_bytes());
    let out = s.on_storage_request(&body);
    assert!(
        out.iter().any(|r| r.opcode == net::storage::STORAGE_RESULT),
        "every 0x00F6 is answered - the client latches until a 0x0572 arrives"
    );
    assert_eq!(store.mesos(id).unwrap(), 3_000, "2000 left the purse");
    assert_eq!(store.storage_mesos(account_id).unwrap(), 2_000, "and arrived in the box");

    // Positive on the wire = WITHDRAW from the box.
    let mut body = vec![7u8];
    body.extend_from_slice(&500i64.to_le_bytes());
    s.on_storage_request(&body);
    assert_eq!(store.mesos(id).unwrap(), 3_500, "500 came back to the purse");
    assert_eq!(store.storage_mesos(account_id).unwrap(), 1_500, "and left the box");
}

/// Over-withdrawing is refused with the mode that says so, and moves nothing.
#[test]
fn a_meso_move_that_cannot_be_afforded_is_refused_and_still_answered() {
    let (mut s, store, id) = gm_session();
    store.set_mesos(id, 100).unwrap();

    let mut body = vec![7u8];
    body.extend_from_slice(&(-9_999i64).to_le_bytes()); // deposit more than the purse holds
    let out = s.on_storage_request(&body);
    let reply = out
        .iter()
        .find(|r| r.opcode == net::storage::STORAGE_RESULT)
        .expect("refusals are answered too, or the window locks up");
    assert_eq!(reply.body[0], net::storage::RESULT_NOT_ENOUGH_MESOS);
    assert_eq!(store.mesos(id).unwrap(), 100, "and nothing moved");
    assert_eq!(store.storage_mesos(1).unwrap(), 0);
}

/// **A pick-up the bag refuses must still send the `0x0070`, or nothing is ever picked up
/// again.**
///
/// The owner, 2026-08-22: *"when my equip slots are full, I should be able to get more items in my
/// other inventory where I still have slots, such as Use, ETC, or mesos. Currently I'm not
/// able to do that."*
///
/// The equip bag being full is what *triggered* it; it is not what blocked the later
/// pick-ups. `world.log` of the 12:21 run has six `0x032C` requests, the sixth answered with
/// `"inventory 1 is full (30 slots)"` **and a chat line alone**, and then **zero** further
/// `0x032C` across the next four minutes and 56 drops. The client had stopped asking:
/// `player+0x2330` latches on send and only an inbound `0x0070` clears it.
///
/// This test asserts the packet, not the notice, because the notice was there the whole time.
#[test]
fn a_pick_up_the_bag_refuses_still_clears_the_clients_latch() {
    let (mut s, store, id) = gm_session();
    let map = crate::fields::FieldKey::world(s.claimed_character().unwrap().map_id);

    // Fill the equip bag to its last slot.
    let slots = net::opcode::DEFAULT_INVENTORY_SLOTS;
    for _ in 0..slots {
        store.add_item(id, store::InventoryType::Equip, &store::Item::equip(1302000), 1).unwrap();
    }

    let object_id = s.fields.with_drops(map, |d| d.next_object_id());
    s.fields.with_drops(map, |d| {
        d.drop_item(crate::drops::DropFromBag {
            map_id: map,
            character_id: id,
            inv_type: store::InventoryType::Equip,
            slot: 1,
            item: store::Item::equip(1302000),
            remaining_in_slot: None,
            x: 0,
            y: 0,
            from_x: 0,
            from_y: 0,
            now_ms: 0,
        })
    });

    let out = s.on_pick_up(0x032C, &pick_up_body(object_id));
    assert!(
        out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION),
        "a refused pick-up owes a 0x0070 - without it the client never asks again, for ANY \
         item, mesos included. Replies were: {:?}",
        out.iter().map(|r| format!("0x{:04X}", r.opcode)).collect::<Vec<_>>()
    );
    // The reason used to be a yellow chat line; since 2026-09-16 it is the client's own
    // "You can't get anymore items." - 0x0089 sub-mode -1 - in the message area.
    assert!(
        out.iter().any(|r| r.opcode == net::message::MESSAGE && r.body == net::message::inventory_full()),
        "and the reason, as the client's own inventory-full line"
    );
    assert!(out.iter().all(|r| r.opcode != net::notice::CHAT_NOTICE), "and nothing in the chat log");
    assert_eq!(
        s.fields.with_drops(map, |d| d.len()),
        1,
        "and the item is still on the floor, not destroyed"
    );
}

/// **A full equip bag does not block a Use pick-up.** The other half of the same report.
///
/// This is what the server was always going to do - `add_item` is given the drop's own
/// inventory type - so the test exists to say the failure was never here. With the latch
/// fixed, it is the behaviour on screen too.
#[test]
fn a_full_equip_bag_does_not_stop_a_use_item_being_picked_up() {
    let (mut s, store, id) = gm_session();
    let map = crate::fields::FieldKey::world(s.claimed_character().unwrap().map_id);
    let slots = net::opcode::DEFAULT_INVENTORY_SLOTS;
    for _ in 0..slots {
        store.add_item(id, store::InventoryType::Equip, &store::Item::equip(1302000), 1).unwrap();
    }

    let object_id = s.fields.with_drops(map, |d| d.next_object_id());
    s.fields.with_drops(map, |d| {
        d.drop_item(crate::drops::DropFromBag {
            map_id: map,
            character_id: id,
            inv_type: store::InventoryType::Use,
            slot: 1,
            item: store::Item::bundle(2000000, 3),
            remaining_in_slot: None,
            x: 0,
            y: 0,
            from_x: 0,
            from_y: 0,
            now_ms: 0,
        })
    });

    s.on_pick_up(0x032C, &pick_up_body(object_id));
    let use_bag = store.bag_items(id, store::InventoryType::Use).unwrap();
    assert_eq!(use_bag.len(), 1, "the potion landed in the Use bag");
    assert_eq!(use_bag[0].item.item_id, 2000000);
    assert_eq!(s.fields.with_drops(map, |d| d.len()), 0, "and left the floor");
}

/// **A player's ground drop can be picked up by somebody else, end to end.**
///
/// The owner, 2026-09-23: *"users dropping items publicly in the field, but nobody except
/// themselves were able to pick up what was dropped on the ground"* - and then the rule:
/// *"as long as it is not untradeable, it should remain on the ground until drop expiry and
/// available for anyone to pick up."*
///
/// Through the real handlers, both halves: the owner's `0x0070` drop, the `0x046E` the bus hands
/// Tester2, and Tester2's `0x032C` naming it. The table-level test
/// (`drops::a_player_ground_drop_is_public_and_has_no_owner_lock`) already passed before this
/// was reported, so this asks the question at the level the report was made at.
#[test]
fn another_player_can_pick_up_a_tradeable_item_wisp_dropped() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let (mut wisp, wisp_id) = join_channel(&store, &config, &fields, account, "Wisp");
    let other = store.create_account("player", "correct horse battery").unwrap();
    let (mut tester, tester_id) = join_channel(&store, &config, &fields, other, "Tester2");
    wisp.on_field_entered();
    tester.on_field_entered();
    wisp.collect_mail();
    tester.collect_mail();

    store.add_item(wisp_id, store::InventoryType::Equip, &store::Item::equip(1_302_000), 1).unwrap();
    wisp.last_position = Some((520, 395));
    tester.last_position = Some((520, 395));
    let out = wisp.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, 1, 0, 1));
    let enter = out
        .iter()
        .find(|r| r.opcode == net::drops::DROP_ENTER_FIELD)
        .expect("the owner's drop lands");
    let object_id = u32::from_le_bytes([enter.body[2], enter.body[3], enter.body[4], enter.body[5]]);

    // **Tester2 sees it**, and is told it belongs to everyone.
    let seen = tester.collect_mail();
    let theirs = seen
        .iter()
        .find(|r| r.opcode == net::drops::DROP_ENTER_FIELD)
        .unwrap_or_else(|| panic!("Tester2 never saw the drop: {:?}", seen.iter().map(|r| &r.what).collect::<Vec<_>>()));
    assert_eq!(
        theirs.body[27],
        net::drops::OWN_TYPE_EVERYONE,
        "a public drop must say so in ownType - OWN_TYPE_USER tells every other client it is the owner's"
    );

    // **And takes it.**
    let got = tester.on_pick_up(0x032C, &pick_up_body(object_id));
    assert!(
        got.iter().any(|r| r.opcode == net::drops::DROP_LEAVE_FIELD),
        "{:?}",
        got.iter().map(|r| &r.what).collect::<Vec<_>>()
    );
    let bagged: Vec<u32> = store
        .bag(tester_id)
        .unwrap()
        .items_in(store::InventoryType::Equip)
        .map(|i| i.item.item_id)
        .collect();
    assert_eq!(bagged, vec![1_302_000], "the sword is in Tester2's bag");
    assert_eq!(
        wisp.fields.with_drops(crate::fields::FieldKey::world(SHARED_MAP), |d| d.len()),
        0,
        "and off the floor"
    );
}

/// **A player who arrives AFTER the drop is shown it too.** The field-entry re-send used to
/// filter every drop on owner/party alone, so a public drop was invisible to anyone who walked
/// in later - the other half of the owner's 2026-09-23 report.
#[test]
fn a_public_drop_is_resent_to_somebody_who_enters_the_map_later() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let (mut wisp, wisp_id) = join_channel(&store, &config, &fields, account, "Wisp");
    wisp.on_field_entered();
    wisp.collect_mail();
    store.add_item(wisp_id, store::InventoryType::Equip, &store::Item::equip(1_302_000), 1).unwrap();
    wisp.last_position = Some((520, 395));
    let out = wisp.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, 1, 0, 1));
    let enter = out.iter().find(|r| r.opcode == net::drops::DROP_ENTER_FIELD).unwrap();
    let object_id = u32::from_le_bytes([enter.body[2], enter.body[3], enter.body[4], enter.body[5]]);

    // Tester2 logs in only now, onto the same map.
    let other = store.create_account("player", "correct horse battery").unwrap();
    let (mut tester, _) = join_channel(&store, &config, &fields, other, "Tester2");
    let entry = tester.on_field_entered();
    let resent = entry
        .iter()
        .find(|r| r.opcode == net::drops::DROP_ENTER_FIELD && r.body[2..6] == object_id.to_le_bytes())
        .unwrap_or_else(|| panic!("the late arrival was never shown the owner's drop: {:?}", entry.iter().map(|r| &r.what).collect::<Vec<_>>()));
    assert_eq!(resent.body[1], net::drops::ENTER_INSTANT, "already lying there, pickable");
    assert_eq!(resent.body[27], net::drops::OWN_TYPE_EVERYONE);
}

/// **An untradeable item a player drops is drawn landing, then fades - for everyone - and
/// nobody gets it.**
///
/// The owner, 2026-09-23: *"Untradeable items when dropped should just disappear, there should be
/// an animation for it on client side and also broadcasted to other clients as well."*
#[test]
fn an_untradeable_item_dropped_fades_for_everyone_and_nobody_gets_it() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let (mut wisp, wisp_id) = join_channel(&store, &config, &fields, account, "Wisp");
    let other = store.create_account("player", "correct horse battery").unwrap();
    let (mut tester, tester_id) = join_channel(&store, &config, &fields, other, "Tester2");
    wisp.on_field_entered();
    tester.on_field_entered();
    wisp.collect_mail();
    tester.collect_mail();

    const BLOCKED: u32 = 1_302_016;
    assert!(store::ItemRules::trade_blocked(BLOCKED), "positive control: really untradeable");
    store.add_item(wisp_id, store::InventoryType::Equip, &store::Item::equip(BLOCKED), 1).unwrap();
    wisp.last_position = Some((520, 395));
    let out = wisp.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, 1, 0, 1));
    let enter = out.iter().find(|r| r.opcode == net::drops::DROP_ENTER_FIELD).expect("drawn landing");
    let object_id = u32::from_le_bytes([enter.body[2], enter.body[3], enter.body[4], enter.body[5]]);
    assert!(store.bag(wisp_id).unwrap().items_in(store::InventoryType::Equip).next().is_none(), "it left the bag");
    // **The client's own disappearing animation**: enter type 3, not the ordinary arc. The owner:
    // *"There should be a separate animation that client should be able to animate where the
    // drop fades out."*
    assert_eq!(enter.body[1], net::drops::ENTER_DISAPPEARING, "{}", enter.what);

    // Tester2 is sent the SAME animation - it is broadcast, not local to the dropper.
    let theirs = tester.collect_mail();
    let seen = theirs.iter().find(|r| r.opcode == net::drops::DROP_ENTER_FIELD).expect("Tester2 sees it go");
    assert_eq!(seen.body[1], net::drops::ENTER_DISAPPEARING);

    // Somebody walking in during that second is NOT sent it - it is on its way out.
    assert!(!tester
        .on_field_entered()
        .iter()
        .any(|r| r.opcode == net::drops::DROP_ENTER_FIELD && r.body[2..6] == object_id.to_le_bytes()));

    // Nobody may take it, the dropper included, and nobody is told off for trying.
    for who in [&mut tester, &mut wisp] {
        let got = who.on_pick_up(0x032C, &pick_up_body(object_id));
        assert!(!got.iter().any(|r| r.opcode == net::drops::DROP_LEAVE_FIELD && r.body[0] == 2), "not picked up");
        assert!(!got.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE), "no chat line for a disposal");
    }
    assert!(store.bag(tester_id).unwrap().items_in(store::InventoryType::Equip).next().is_none());

    // After VANISH_MS the fade goes to the field - Tester2's screen too.
    wisp.tick(crate::drops::VANISH_MS + 1);
    let faded = tester.collect_mail();
    assert!(
        faded.iter().any(|r| r.opcode == net::drops::DROP_LEAVE_FIELD && r.body[0] == 0),
        "{:?}",
        faded.iter().map(|r| &r.what).collect::<Vec<_>>()
    );
    assert_eq!(wisp.fields.with_drops(crate::fields::FieldKey::world(SHARED_MAP), |d| d.len()), 0);
}

/// **An empty star stack drops and is picked up like any item.** The owner, 2026-10-02: *"A star
/// that has 0 ammo is still allowed to be dropped on the ground and picked up."* Dropping it is
/// not refused as "nothing in that slot": the slot is cleared (mode 3) and the stack lies on
/// the floor at 0. Picking it up puts it back in a slot of its own at 0 - not nowhere - with no
/// "x1 earned" line for a count it does not have.
#[test]
fn an_empty_star_stack_drops_and_is_picked_up_at_zero() {
    let (mut s, store, id) = gm_session();
    s.last_position = Some((520, 395));
    store.set_inventory_slot(id, store::InventoryType::Use, 1, &store::Item::bundle(2_070_006, 0)).unwrap();
    let map = crate::fields::FieldKey::world(s.claimed_character().unwrap().map_id);
    let object_id = s.fields.with_drops(map, |d| d.next_object_id());

    let out = s.on_inventory_move(&inventory_move(use_tab(), 1, 0, 1));
    assert_eq!(out[0].body, net::inventory::inventory_removed(use_tab(), 1), "the slot is cleared: {}", out[0].what);
    assert!(out.iter().any(|r| r.opcode == net::drops::DROP_ENTER_FIELD), "and the stack is on the floor");
    assert_eq!(store.inventory_slot(id, store::InventoryType::Use, 1).unwrap(), None);

    let got = s.on_pick_up(0x032C, &pick_up_body(object_id));
    let row = store.bag_items(id, store::InventoryType::Use).unwrap();
    assert_eq!(row.len(), 1, "it came back: {got:?}");
    assert_eq!((row[0].item.item_id, row[0].item.kind.quantity()), (2_070_006, 0), "Ilbi, still empty");
    assert!(got.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "the bag is told");
    assert!(!got.iter().any(|r| r.opcode == net::message::MESSAGE), "no 'x1 earned' for an empty stack");
    assert_eq!(s.fields.with_drops(map, |d| d.len()), 0, "and it left the floor");
}

/// A `0x032C` body with the drop's object id where the client puts it: **offset 13**.
fn pick_up_body(object_id: u32) -> Vec<u8> {
    let mut b = vec![0u8; 13];
    b.extend_from_slice(&object_id.to_le_bytes());
    b.extend_from_slice(&[0u8; 4]);
    b
}

/// **Storing an item moves it, and the keeper takes the fee they advertised.**
///
/// The owner, 2026-08-22: *"I tried to store an item with Mr. Kim. The item did not move to
/// storage, and it did not charge the 100 meso fee that it said it was going to charge."*
/// Both sentences are one missing arm. The fee text is the **client's**, out of `Npc.wz`, so
/// an unbuilt deposit shows a promise the server then does not keep.
///
/// Three effects, and the test says something about all three - the rule the repeated-quest
/// bug bought, where a turn-in test counted fanfares and missed the doubled experience beside
/// it: the bag loses it, the box gains it, and the purse pays 100.
#[test]
fn storing_an_item_moves_it_and_charges_the_keepers_fee() {
    let (mut s, store, id) = gm_session();
    let account_id = 1i64;
    store.set_mesos(id, 5_000).unwrap();
    store.add_item(id, store::InventoryType::Equip, &store::Item::equip(1302000), 1).unwrap();
    s.open_storage_for(105).expect("Mr. Kim");

    let mut body = vec![5u8];
    body.extend_from_slice(&1u16.to_le_bytes()); // bag slot 1
    body.extend_from_slice(&1302000u32.to_le_bytes());
    body.extend_from_slice(&1u16.to_le_bytes());
    let out = s.on_storage_request(&body);

    let reply = out
        .iter()
        .find(|r| r.opcode == net::storage::STORAGE_RESULT)
        .expect("every 0x00F6 is answered");
    assert_eq!(reply.body[0], net::storage::RESULT_PUT_OK, "mode 13 rebuilds both grids");
    assert!(
        out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION),
        "and the bag grid is told, or the item is still drawn in a slot it left"
    );

    assert!(store.bag_items(id, store::InventoryType::Equip).unwrap().is_empty(), "left the bag");
    let boxx = store.storage(account_id).unwrap();
    assert_eq!(boxx.items.len(), 1, "arrived in the box");
    assert_eq!(boxx.items[0].item.item_id, 1302000);
    assert_eq!(store.mesos(id).unwrap(), 4_900, "Mr. Kim's fee is 100, and they took it");
}

/// A deposit that cannot pay the fee is refused with the mode that says so, and **moves
/// nothing**. The refusal is a transition, so no effect may hang off the request.
#[test]
fn a_deposit_that_cannot_pay_the_fee_moves_nothing() {
    let (mut s, store, id) = gm_session();
    store.set_mesos(id, 50).unwrap();
    store.add_item(id, store::InventoryType::Equip, &store::Item::equip(1302000), 1).unwrap();
    s.open_storage_for(105).expect("Mr. Kim");

    let mut body = vec![5u8];
    body.extend_from_slice(&1u16.to_le_bytes());
    body.extend_from_slice(&1302000u32.to_le_bytes());
    body.extend_from_slice(&1u16.to_le_bytes());
    let out = s.on_storage_request(&body);

    let reply = out
        .iter()
        .find(|r| r.opcode == net::storage::STORAGE_RESULT)
        .expect("refusals are answered too");
    assert_eq!(reply.body[0], net::storage::RESULT_NOT_ENOUGH_FEE, "mode 16 names the fee");
    assert_eq!(store.mesos(id).unwrap(), 50, "the purse is untouched");
    assert_eq!(store.bag_items(id, store::InventoryType::Equip).unwrap().len(), 1, "so is the bag");
    assert!(store.storage(1).unwrap().is_empty(), "and the box");
}

/// **A put-in with no window open is refused rather than charged a guessed fee.**
///
/// Nine of the ten keepers charge 100 and Mr. Thalj charges 150, so a default would be right
/// nine times in ten and quietly wrong once. `Close` drops the keeper, and after it a deposit
/// has no fee to name.
#[test]
fn closing_the_window_forgets_the_keeper_and_a_later_deposit_is_refused() {
    let (mut s, store, id) = gm_session();
    store.set_mesos(id, 5_000).unwrap();
    store.add_item(id, store::InventoryType::Equip, &store::Item::equip(1302000), 1).unwrap();
    s.open_storage_for(105).expect("Mr. Kim");
    assert_eq!(s.open_storage, Some(105));

    s.on_storage_request(&[8u8]); // close
    assert_eq!(s.open_storage, None, "the keeper is forgotten");

    let mut body = vec![5u8];
    body.extend_from_slice(&1u16.to_le_bytes());
    body.extend_from_slice(&1302000u32.to_le_bytes());
    body.extend_from_slice(&1u16.to_le_bytes());
    let out = s.on_storage_request(&body);
    assert!(
        out.iter().any(|r| r.opcode == net::storage::STORAGE_RESULT),
        "still answered - the latch does not care why we refused"
    );
    assert_eq!(store.mesos(id).unwrap(), 5_000, "no fee was invented");
    assert_eq!(store.bag_items(id, store::InventoryType::Equip).unwrap().len(), 1, "and nothing moved");
}

/// **The take-out index is positional, not a storage slot, and a sparse box is where the
/// difference shows.**
///
/// `research/storage.md` §11.5 lists this first among "the three numbers that must not
/// drift". Two equips go into slots 1 and 2, slot 1 is emptied by hand, and the client then
/// asks for **position 0 of type 1** - which is the item in storage slot **2**. A server that
/// read the index as a slot would look at slot 0, find nothing, and refuse; one that used a
/// different ordering than it sent would hand back a different item with no error anywhere.
#[test]
fn a_take_out_index_is_a_position_in_the_list_that_was_sent_not_a_slot() {
    let (mut s, store, id) = gm_session();
    let account_id = 1i64;
    store.set_storage_slot(account_id, 1, &store::Item::equip(1302000)).unwrap();
    store.set_storage_slot(account_id, 2, &store::Item::equip(1332000)).unwrap();
    store.storage_withdraw(account_id, 1, None).unwrap(); // slot 1 is now empty, 2 is not

    let out = s.on_storage_request(&[4u8, 1, 0, 1, 0]); // type 1 equip, position 0, count 1
    let reply = out
        .iter()
        .find(|r| r.opcode == net::storage::STORAGE_RESULT)
        .expect("every 0x00F6 is answered");
    assert_eq!(reply.body[0], net::storage::RESULT_PUT_OK, "mode 13");

    let bag = store.bag_items(id, store::InventoryType::Equip).unwrap();
    assert_eq!(bag.len(), 1, "one item came out");
    assert_eq!(
        bag[0].item.item_id, 1332000,
        "position 0 is the FIRST SURVIVING row, which lives in storage slot 2 - reading the \
         index as a slot would have found slot 0 and refused"
    );
    assert!(store.storage(account_id).unwrap().is_empty(), "and it left the box");
}

/// The `0x013C` body the client sends: `u32 skillId, u32 level`, then a tail we do not read.
fn skill_use_body(skill_id: u32, level: u32) -> Vec<u8> {
    let mut b = Vec::with_capacity(51);
    b.extend_from_slice(&skill_id.to_le_bytes());
    b.extend_from_slice(&level.to_le_bytes());
    b.resize(51, 0);
    b
}

/// The character's MP as the database holds it. `claimed_character` re-reads every call, so
/// this is the stored value and not a copy taken before the handler ran.
fn mp_of(s: &Session) -> u32 {
    s.claimed_character().expect("a character is claimed").mp
}

/// A session whose character owns Nimble Feet at level 3 and has MP to spend.
fn session_with_nimble_feet() -> (Session, Arc<Store>, u32) {
    let (s, store, id) = gm_session();
    store.set_skill_level(id, net::buff::NIMBLE_FEET, 3).unwrap();
    let mut chr = s.claimed_character().unwrap();
    // A fresh character has **5** max MP and level 3 costs 10, so without this every one of
    // these tests would pass or fail on the MP gate rather than on what it is about.
    chr.max_mp = 125;
    chr.mp = 125;
    store.save_character_progress(&chr).unwrap();
    (s, store, id)
}

/// **Pressing Nimble Feet grants the buff.**
///
/// The owner, twice: *"Nimble Feet still does not give me a buff despite me activating the
/// skill."* The request was arriving and being dropped - one `0x013C`, 51 bytes, skill 1002
/// level 3, logged as UNKNOWN.
///
/// Three effects, and this asserts all three: the MP is spent, the `0x007C` goes out so the
/// bar moves, and the `0x007D` carries the documented body. A test that counted only the
/// `0x007D` would pass while the MP silently never left, which is the shape of the
/// repeated-quest bug.
#[test]
fn casting_nimble_feet_spends_mp_and_sends_the_temporary_stat() {
    let (mut s, _store, _id) = session_with_nimble_feet();
    let before = s.claimed_character().unwrap().mp;

    let out = s.on_skill_use(&skill_use_body(net::buff::NIMBLE_FEET, 3));

    let set = out
        .iter()
        .find(|r| r.opcode == net::buff::TEMPORARY_STAT_SET)
        .expect("0x007D goes out");
    // NOT a literal: net::buff::TAIL_LEN is slack around a length nobody has derived, and
    // it has already changed once - 18 bytes threw an unhandled C++ exception in the client.
    // The head is pinned byte for byte below; the total follows the constant.
    assert_eq!(set.body.len(), net::buff::temporary_stat_set_len(1));
    assert_eq!(&set.body[8..12], &[0x08, 0, 0, 0], "CTS bit 92, Speed");
    assert_eq!(
        &set.body[124..134],
        &[0x0a, 0x00, 0xea, 0x03, 0x00, 0x00, 0x30, 0x75, 0x00, 0x00],
        "speed 10, reason 1002, 30000 MILLISECONDS"
    );
    assert!(
        out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
        "and the MP bar is told, or it silently disagrees with the database"
    );
    assert_eq!(mp_of(&s), before - 10, "level 3 costs 10 mp");
}

/// A skill the character does not own casts nothing and costs nothing.
///
/// The client sends the level **it** believes it has, and nothing on this socket
/// authenticates anybody.
#[test]
fn a_skill_the_character_does_not_have_is_refused_and_costs_nothing() {
    let (mut s, store, _id) = gm_session();
    let mut chr = s.claimed_character().unwrap();
    chr.max_mp = 125;
    chr.mp = 125;
    store.save_character_progress(&chr).unwrap();
    let before = chr.mp;

    let out = s.on_skill_use(&skill_use_body(net::buff::NIMBLE_FEET, 3));
    assert!(
        !out.iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_SET),
        "no buff for a skill nobody has"
    );
    assert!(out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE), "and it says why");
    assert_eq!(mp_of(&s), before, "and no MP was spent");
}

/// **The cooldown refuses the second cast and stays free.**
///
/// `Skill.wz` puts `cooltime` at 180 s. The refusal names the seconds left, because a silent
/// one would be indistinguishable on screen from the buff being broken - which is the exact
/// thing this run is trying to tell apart.
#[test]
fn a_second_cast_inside_the_cooldown_is_refused_and_costs_nothing() {
    let (mut s, _store, _id) = session_with_nimble_feet();
    s.clock_ms = 1_000;
    s.on_skill_use(&skill_use_body(net::buff::NIMBLE_FEET, 3));
    let after_first = mp_of(&s);

    s.clock_ms = 60_000; // a minute later: buff over, cooldown not
    let out = s.on_skill_use(&skill_use_body(net::buff::NIMBLE_FEET, 3));
    assert!(!out.iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_SET));
    assert_eq!(mp_of(&s), after_first, "a refused cast is free");
    let why = out.iter().find(|r| r.opcode == net::notice::CHAT_NOTICE).map(notice_text);
    assert!(
        why.as_deref().is_some_and(|w| w.contains("cooldown")),
        "and names the cooldown: {why:?}"
    );

    // And past it, the cast works again.
    s.clock_ms = 1_000 + 180_000;
    let out = s.on_skill_use(&skill_use_body(net::buff::NIMBLE_FEET, 3));
    assert!(out.iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_SET), "180 s later it casts");
}

/// Not enough MP refuses, and refuses before anything is spent or stamped.
#[test]
fn a_cast_without_the_mp_is_refused_before_the_cooldown_is_stamped() {
    let (mut s, store, _id) = session_with_nimble_feet();
    let mut chr = s.claimed_character().unwrap();
    chr.mp = 3; // level 3 costs 10
    store.save_character_progress(&chr).unwrap();

    let out = s.on_skill_use(&skill_use_body(net::buff::NIMBLE_FEET, 3));
    assert!(!out.iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_SET));
    assert_eq!(mp_of(&s), 3, "nothing was spent");

    // The cooldown must NOT have been stamped by the refusal, or one mistimed press would
    // lock the skill out for three minutes.
    chr.mp = chr.max_mp;
    store.save_character_progress(&chr).unwrap();
    let out = s.on_skill_use(&skill_use_body(net::buff::NIMBLE_FEET, 3));
    assert!(
        out.iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_SET),
        "with MP it casts immediately - the refusal stamped no cooldown"
    );
}

/// **The expiry sends `0x007E`, because the client will not remove the stat itself.**
///
/// This assertion has been inverted once, and the inversion is the point. The tick was
/// changed to send nothing on the reading that the client's own `tExpire` would drop the
/// stat. The owner, 2026-08-22: *"after the expiry, the buff did not go away. (It just kept
/// flashing, but the temporary stats were still there)"* - and the client sent nothing at
/// the thirty-second mark, so `tExpire` drives the flashing and nothing else.
///
/// Both halves are asserted: the packet, and that the table really expired. A test that
/// only checked the packet would also pass on a tick that never cleared its own state and
/// re-sent the reset for ever.
#[test]
fn the_expiry_sends_the_reset_because_the_client_will_not_self_expire() {
    let (mut s, _store, _id) = session_with_nimble_feet();
    s.clock_ms = 1_000;
    s.on_skill_use(&skill_use_body(net::buff::NIMBLE_FEET, 3));
    assert_eq!(s.buffs.len(), 1, "held");

    let resets = |out: &[Reply]| {
        out.iter().filter(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET).count()
    };
    assert_eq!(resets(&s.buff_tick(20_000)), 0, "still running");
    assert_eq!(resets(&s.buff_tick(30_999)), 0, "and at 29.999 s");
    assert_eq!(s.buffs.len(), 1);

    let out = s.buff_tick(31_000);
    assert_eq!(resets(&out), 1, "30 s after the cast at 1 s");
    let reset = out.iter().find(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET).unwrap();
    assert_eq!(reset.body.len(), net::buff::TEMPORARY_STAT_RESET_LEN);
    assert!(reset.body.len() > net::buff::RESET_KNOWN_TOO_SHORT, "127 threw");
    assert_eq!(reset.body[3 + 8], 0x08, "bit 92");

    assert!(s.buffs.is_empty(), "and the table dropped it");
    assert_eq!(resets(&s.buff_tick(40_000)), 0, "not a second time");
}

/// A `0x013F` body: `u32 skillId`, five bytes, then the 124-byte mask.
fn skill_cancel_body(skill_id: u32, bits: &[u32]) -> Vec<u8> {
    let mut b = vec![0u8; net::buff::CLIENT_SKILL_CANCEL_LEN];
    b[0..4].copy_from_slice(&skill_id.to_le_bytes());
    b[net::buff::CANCEL_MASK_OFFSET..].copy_from_slice(&net::buff::stat_mask(bits));
    b
}

/// **Right-clicking a buff icon cancels it, and the request is answered even when it cannot.**
///
/// The owner, 2026-08-22: *"I also tried to pre-emptively kill the buff by right clicking on the
/// icon, it also did not dismiss the buff."* Fourteen `0x013F` bodies arrived in three
/// seconds, one every ~180 ms - a retry loop - and every one was dropped.
#[test]
fn right_clicking_a_buff_icon_cancels_it() {
    let (mut s, _store, _id) = session_with_nimble_feet();
    s.clock_ms = 1_000;
    s.on_skill_use(&skill_use_body(net::buff::NIMBLE_FEET, 3));

    let out = s.on_skill_cancel(&skill_cancel_body(net::buff::NIMBLE_FEET, &[net::buff::CTS_SPEED]));
    let reset = out
        .iter()
        .find(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET)
        .expect("the right-click is answered with a 0x007E");
    assert_eq!(reset.body[3 + 8], 0x08, "the bit the client pointed at");
    assert!(s.buffs.is_empty(), "and the server stops holding it");

    // A second right-click has nothing to cancel - and still says something, because silence
    // here is exactly what the retry loop looked like.
    let out = s.on_skill_cancel(&skill_cancel_body(net::buff::NIMBLE_FEET, &[net::buff::CTS_SPEED]));
    assert!(!out.iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET));
    assert!(out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE));
}

/// **A cancel for a bit we never granted clears nothing.**
///
/// The mask is what the client points at, and honouring a bit the server does not hold would
/// tell it to drop a stat that came from somewhere else.
#[test]
fn a_cancel_for_a_bit_we_do_not_hold_is_refused_and_leaves_the_buff_alone() {
    let (mut s, _store, _id) = session_with_nimble_feet();
    s.clock_ms = 1_000;
    s.on_skill_use(&skill_use_body(net::buff::NIMBLE_FEET, 3));

    let out = s.on_skill_cancel(&skill_cancel_body(net::buff::NIMBLE_FEET, &[200]));
    assert!(!out.iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET));
    assert_eq!(s.buffs.len(), 1, "the Speed buff is untouched");

    // An unreadable body is answered with a line rather than dropped.
    let out = s.on_skill_cancel(&[0u8; 8]);
    assert!(out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE));
}

/// **"Organize Item" organises.**
///
/// The owner, 2026-08-22: *"I tried hitting the 'Organize Item' 3 times, but it did not perform
/// anything."* Mode 6 answered with the unchanged box and a log line calling that a legal
/// no-op. It was legal; it was not what the button says it does.
///
/// Three clicks is what they actually did, so three is what this asserts - a sort whose
/// tie-break depended on read order would reshuffle every click, which on screen looks
/// exactly like the old broken version.
#[test]
fn organize_item_repacks_the_box_and_repeats_do_not_reshuffle() {
    let (mut s, store, _id) = gm_session();
    let account_id = 1i64;
    store.set_storage_slot(account_id, 9, &store::Item::bundle(2000000, 3)).unwrap();
    store.set_storage_slot(account_id, 2, &store::Item::equip(1302000)).unwrap();
    store.set_storage_slot(account_id, 7, &store::Item::equip(1040001)).unwrap();

    let out = s.on_storage_request(&[6u8]);
    let reply = out
        .iter()
        .find(|r| r.opcode == net::storage::STORAGE_RESULT)
        .expect("every 0x00F6 is answered");
    assert_eq!(reply.body[0], net::storage::RESULT_TRUNK_REFRESH, "mode 15 recalculates scroll");

    let after = store.storage(account_id).unwrap();
    assert_eq!(
        after.items.iter().map(|i| (i.slot, i.item.item_id)).collect::<Vec<_>>(),
        vec![(1, 1040001), (2, 1302000), (3, 2000000)],
        "packed 1..n, equips by id first, then Use"
    );

    s.on_storage_request(&[6u8]);
    s.on_storage_request(&[6u8]);
    assert_eq!(store.storage(account_id).unwrap().items, after.items, "clicks 2 and 3 change nothing");
}

/// **The Cash Shop button sends the player into the shop.**
///
/// This assertion has been replaced once. It used to demand a `0x0070` - a borrowed packet
/// whose only job was to clear `[ctx+0x2330]` so the button would fire more than once. `0x01A3`
/// clears that latch itself (`FUN_142CBE8F0` at `0x142CBE918`), so sending both would be a
/// packet whose whole purpose is already served, telling the client an inventory operation
/// completed when none did.
///
/// Both halves are asserted: the stage packet, and the wallet - which is the **only** packet
/// that carries a balance, so without it the shop has nothing to spend.
#[test]
fn the_cash_shop_button_sends_the_stage_packet_and_the_wallet() {
    let (mut s, store, _id) = gm_session();
    store.add_nx(1, 25_000).unwrap();
    let body = [0xe9, 0x29, 0xba, 0x05, 0x00]; // the real capture

    let out = s.on_cash_shop_request(&body);
    let stage = out
        .iter()
        .find(|r| r.opcode == net::cashshop::SET_CASH_SHOP)
        .expect("0x01A3 goes out");
    assert!(
        !out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION),
        "and NO 0x0070 - 0x01A3 clears ctx+0x2330 by itself"
    );

    let wallet = out
        .iter()
        .find(|r| r.opcode == net::cashshop::CASH_SHOP_WALLET)
        .expect("the balance is the only thing that makes the shop usable");
    assert_eq!(u32::from_le_bytes(wallet.body[0..4].try_into().unwrap()), 25_000);

    // The head is the FILETIME alone. Borrowing SetField's head would shift the character
    // record by 25 bytes and decode garbage, and net/cashshop.rs pins that byte for byte;
    // here it is enough that the stage body is a different shape from a SetField body.
    let field = out.iter().find(|r| r.opcode == net::opcode::SET_FIELD);
    assert!(field.is_none(), "entering the shop is NOT a SetField");
    assert!(stage.body.len() > net::cashshop::CASH_SHOP_MARGIN, "record plus margin");

    // **An unreadable body still enters the shop.** The latch was set by the client's builder
    // before we ever saw the body.
    for bad in [vec![], vec![1u8, 2, 3], vec![0u8; 9]] {
        let out = s.on_cash_shop_request(&bad);
        assert!(
            out.iter().any(|r| r.opcode == net::cashshop::SET_CASH_SHOP),
            "body {bad:02x?} must still be answered"
        );
    }
}

/// **`0x03E0` is answered with the balance**, and it is the packet that clears the client's
/// own 60-second query latch.
#[test]
fn the_balance_query_is_answered() {
    let (mut s, store, _id) = gm_session();
    store.add_nx(1, 777).unwrap();
    let out = s.on_cash_shop_query();
    let wallet = out
        .iter()
        .find(|r| r.opcode == net::cashshop::CASH_SHOP_WALLET)
        .expect("0x05AD");
    assert_eq!(u32::from_le_bytes(wallet.body[0..4].try_into().unwrap()), 777);
    assert_eq!(wallet.body.len(), net::cashshop::CASH_SHOP_WALLET_LEN);
}

/// **`0x00D1` is two requests and the LENGTH is the only thing that separates them.**
///
/// A portal walk sends 35 bytes; the Cash Shop's Exit button sends none. Getting this
/// backwards would make every portal in the game try to leave a cash shop, so it is asserted
/// from the dispatcher rather than from the handler - the split lives there.
#[test]
fn an_empty_transfer_field_leaves_the_cash_shop_and_a_full_one_is_a_portal() {
    let (mut s, _store, _id) = gm_session();

    // Empty body: the Exit button. A SetField comes back for the map they are already on.
    let mut empty = 0x00D1u16.to_le_bytes().to_vec();
    let out = s.handle(&empty);
    assert!(
        out.iter().any(|r| r.opcode == net::opcode::SET_FIELD),
        "an empty 0x00D1 is the Exit button"
    );

    // A portal-shaped body must NOT take the exit path. It carries a target and a name, so it
    // reaches the transfer handler and is answered on its own terms.
    empty.extend_from_slice(&[0u8; 35]);
    let out = s.handle(&empty);
    assert!(
        !out.is_empty(),
        "a 35-byte 0x00D1 is a portal walk and is still answered"
    );
}


/// A session whose config carries three real sale rows, so a purchase can be priced.
///
/// The first two are the same item id at two counts and two prices - `130200000` is one
/// Megaphone for 100 NX, `130200001` is eleven for 1000 - because that pair is the reason the
/// shop's key is the SN and not the item id. **All three rows are copied verbatim from
/// `gm-handbook/commodity.txt`**, including the third, which really does have `onSale = 0` -
/// a fixture that quietly contradicts the data it stands in for is worse than no fixture.
///
/// Worth knowing while reading these: the 21 rows priced at 0 NX are **exactly** the 21 rows
/// that are not on sale, so nothing buyable in this client is free.
fn cash_shop_session() -> (Session, Arc<Store>, u32) {
    let dir = std::env::temp_dir().join(format!("maplecw-shop-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("commodity.txt");
    std::fs::write(
        &path,
        "# sn, itemId, count, price, ...\n\
         130200000, 5070000, 1, 100, 100, , 0, 2, 1, , 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 302, Megaphone\n\
         130200001, 5070000, 11, 1000, 1100, 1, 0, 2, 1, , 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 302, Megaphone\n\
         92000000, 5000054, 1, 0, 0, , 0, 2, 0, , 100, 0, 0, 0, 0, , , , 0, 0, -1, 20, -80, Snail\n",
    )
    .unwrap();

    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut item_names = std::collections::HashMap::new();
    item_names.insert(5070000u32, "Megaphone".to_string());
    let config = Config {
        item_names,
        commodity: crate::commodity::CommodityTable::load(&path),
        ..Config::default()
    };
    let mut s = Session::new(store.clone(), Arc::new(config));
    assert!(s.claim_for_character(id).contains("claimed the migration"));
    (s, store, id)
}

/// **The real buy payload, byte for byte off the wire.** `u8, u32, u8, u8, u32 SN, u32`, with
/// the serial at offset 7 - captured three times on 2026-08-26, each resolving to the item
/// The owner said they had clicked. Building the shape the client really sends is what makes these
/// tests exercise the same path a run does.
fn buy_body(sn: u32) -> Vec<u8> {
    let mut body = net::cashshop::CLIENT_CASH_SHOP_ACTION.to_le_bytes().to_vec();
    body.push(net::cashshop::ACTION_BUY);
    body.extend_from_slice(&[0x01, 0x02, 0, 0, 0, 0, 0]); // the constant head, as captured
    body.extend_from_slice(&sn.to_le_bytes()); // payload offset 7
    body.extend_from_slice(&[0u8; 4]);
    assert_eq!(body.len(), 2 + 1 + net::cashshop::BUY_PAYLOAD_LEN, "the captured shape");
    body
}

/// **Every `0x03E1` that latches is answered, with the refusal its family needs.**
///
/// This assertion has been replaced once. It used to demand `0x1A` for every sub-op, which
/// was right for the buy and wrong for the rest: `research/cash-shop-actions.md` found that
/// `0x0A`/`0x0B`/`0x1C` are a **queue** of 32-byte records and `0x1A` empties the queue
/// vector, so refusing the first of five queued deletes with it would silently discard four.
///
/// Three things are pinned: that a latching sub-op is always answered, that the answer is
/// never the arm which **ejects** the player, and that the queue family gets `0x3D` instead.
#[test]
fn every_cash_shop_action_is_answered_with_the_refusal_its_family_needs() {
    let (mut s, store, _id) = cash_shop_session();
    store.add_maple_points(1, 50_000).unwrap();

    let action = |sub: u8, tail: &[u8]| {
        let mut b = net::cashshop::CLIENT_CASH_SHOP_ACTION.to_le_bytes().to_vec();
        b.push(sub);
        b.extend_from_slice(tail);
        b
    };

    // A buy that CANNOT go through - no such sale row, and a body with no serial at all -
    // still gets the refusal measured on a client.
    for body in [buy_body(999_999), action(net::cashshop::ACTION_BUY, &[])] {
        let r = s
            .handle(&body)
            .into_iter()
            .find(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT)
            .unwrap_or_else(|| panic!("buy {body:02x?} went unanswered"));
        assert_eq!(r.body[0], net::cashshop::RESULT_CANCEL_AND_STAY, "the measured one");
        assert_eq!(r.body.len(), net::cashshop::CASH_SHOP_REFUSAL_LEN, "u8 reason");
    }

    // The QUEUE family: 0x3D, and a u16 reason rather than a u8.
    for sub in net::cashshop::ACTION_ON_SERIAL {
        let r = s
            .handle(&action(sub, &[0u8; 8]))
            .into_iter()
            .find(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT)
            .unwrap_or_else(|| panic!("queued sub-op 0x{sub:02X} went unanswered"));
        assert_eq!(
            r.body[0],
            net::cashshop::RESULT_QUEUE_REFUSED,
            "sub-op 0x{sub:02X} is queued - 0x1A would empty the queue"
        );
        assert_eq!(r.body.len(), net::cashshop::CASH_SHOP_QUEUE_REFUSAL_LEN);
    }

    // The gift and anything unknown: 0x1A, because it is the only arm that clears
    // [stage+0x120], and a gift sets that to a value nothing else compares against.
    for sub in [net::cashshop::ACTION_GIFT, 0x77] {
        let r = s
            .handle(&action(sub, &[0u8; 4]))
            .into_iter()
            .find(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT)
            .unwrap_or_else(|| panic!("sub-op 0x{sub:02X} went unanswered"));
        assert_eq!(r.body[0], net::cashshop::RESULT_CANCEL_AND_STAY);
    }

    // **The one deliberate silence**, and it is deliberate: 0x2B does not set the in-flight
    // latch, so nothing blocks - and the only refusal available would discard the queue.
    assert!(
        s.handle(&action(net::cashshop::ACTION_NO_LATCH, &[0u8; 4])).is_empty(),
        "0x2B is the documented exception to always-answer"
    );

    // And NOTHING in any family ejects the player, or carries a wallet on a REFUSAL - a
    // wallet is only ever legal behind a 0x19, because before one it re-triggers the buy.
    for sub in [0x02u8, 0x0A, 0x03, 0x77] {
        let out = s.handle(&action(sub, &[0u8; 8]));
        assert!(
            !out.iter().any(|r| r.opcode == net::cashshop::CASH_SHOP_WALLET),
            "sub-op 0x{sub:02X}: a refusal must not carry a wallet"
        );
        for r in out.iter().filter(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT) {
            assert_ne!(r.body[0], net::cashshop::RESULT_MESSAGE_AND_EJECT);
        }
    }
}

/// **The reason byte says which thing went wrong, and an affordable buy goes through.**
///
/// The last block is the one that changed on 2026-08-27. It used to assert that a purchase
/// this server *could* make was refused anyway and nothing debited, because no packet was
/// known that reported success without an error message on screen. `0x05AE 0x19` is that
/// packet; it was missed because a known list was searched instead of the space enumerated.
#[test]
fn a_buy_is_priced_against_the_real_sale_row_and_then_completes() {
    let (mut s, store, id) = cash_shop_session();

    // Nothing in the wallet: "You don't have enough cash."
    let out = s.handle(&buy_body(130200000));
    let r = out.iter().find(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT).expect("answered");
    assert_eq!(r.body[1], net::cashshop::reason::NOT_ENOUGH_CASH);
    assert!(r.what.contains("SN 130200000"), "the log names the row: {}", r.what);
    assert!(r.what.contains("OFFSET 7"), "and where the serial was: {}", r.what);
    assert!(store.cash_locker(1).unwrap().is_empty(), "and nothing was placed");

    // onSale = 0: "sold out", whatever the balance is.
    store.add_maple_points(1, 50_000).unwrap();
    let out = s.handle(&buy_body(92000000));
    let r = out.iter().find(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT).expect("answered");
    assert_eq!(r.body[1], net::cashshop::reason::SOLD_OUT);

    // No serial anywhere in the payload: also sold out, and the log shows what it read.
    let mut junk = net::cashshop::CLIENT_CASH_SHOP_ACTION.to_le_bytes().to_vec();
    junk.push(net::cashshop::ACTION_BUY);
    junk.extend_from_slice(&[7u8; 12]);
    let out = s.handle(&junk);
    let r = out.iter().find(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT).expect("answered");
    assert_eq!(r.body[1], net::cashshop::reason::SOLD_OUT);
    assert!(r.what.contains("NO commodity serial"), "{}", r.what);

    // **Affordable: the item arrives in the CASH INVENTORY and the price comes out.**
    let out = s.handle(&buy_body(130200000));
    let grant = out
        .iter()
        .find(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT)
        .expect("a result comes back");
    assert_eq!(
        grant.body[0],
        net::cashshop::RESULT_ITEM_TO_LOCKER,
        "0x0C, the CASH INVENTORY. 0x19 is the reply to a locker->bag MOVE, and using it here          is what made the owner's coupon land in the Item Inventory"
    );
    // The record carries the item itself after its trailing flag - the form the locker panel
    // draws. 5070000 is a bundle, so the bag's own type-2 body follows, verbatim.
    let item_body = net::bag::bundle_item(5070000, 1, 0, &[0u8; net::bag::BUNDLE_OWNER_LEN]);
    assert_eq!(grant.body[1 + 70], 1, "trailing flag 1: a GW_ItemSlot follows");
    assert_eq!(&grant.body[1 + 71..1 + 71 + item_body.len()], &item_body[..], "the bundle body");
    assert_eq!(grant.body.len(), 1 + net::cashshop::CASH_ITEM_RECORD_LEN + item_body.len() + 4 + 1);

    assert_eq!(store.cash_wallet(1).unwrap().maple_points, 49_900, "100 LP came out");
    let locker = store.cash_locker(1).unwrap();
    assert_eq!(locker.len(), 1, "and it is in the LOCKER");
    assert_eq!(locker[0].item.item_id, 5070000);
    assert!(
        store.inventory_slot(id, store::InventoryType::Cash, 1).unwrap().is_none(),
        "and NOT in the bag - that was the bug"
    );

    // The record must name the item and carry a usable serial, or every later move fails.
    let rec = &grant.body[1..1 + net::cashshop::CASH_ITEM_RECORD_LEN];
    assert_eq!(u32::from_le_bytes(rec[16..20].try_into().unwrap()), 5070000, "nItemID at +16");
    assert_eq!(u32::from_le_bytes(rec[20..24].try_into().unwrap()), 130200000, "the SN at +20");
    let serial = u64::from_le_bytes(rec[0..8].try_into().unwrap());
    assert_ne!(serial, 0, "a zero serial is not a usable map key");
    assert_ne!(serial, u64::MAX, "and -1 is dropped by FUN_140D75850");

    // **The wallet goes SECOND, and here it is load-bearing in a new way.** It clears
    // [stage+0x74], then sees the buy's [stage+0x120] == 1 and re-enters the buy builder on
    // its COMPLETION path - which is what shows "You have successfully made the purchase."
    let grant_at = out.iter().position(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT).unwrap();
    let wallet_at = out
        .iter()
        .position(|r| r.opcode == net::cashshop::CASH_SHOP_WALLET)
        .expect("the debited balance follows");
    assert!(wallet_at > grant_at, "0x05AD must come AFTER 0x0C, never before");
    assert_eq!(
        u32::from_le_bytes(out[wallet_at].body[4..8].try_into().unwrap()),
        49_900,
        "and it carries the DEBITED balance"
    );
}

/// **A purchase that cannot be placed changes nothing** - and needs no compensating undo,
/// because `buy_cash_item` checks, debits and places in ONE transaction.
#[test]
fn a_purchase_that_cannot_be_placed_leaves_the_wallet_alone() {
    let (mut s, store, _id) = cash_shop_session();
    store.add_maple_points(1, 50_000).unwrap();

    // Fill every locker slot.
    for n in 0..store::cash::LOCKER_SLOTS {
        store.put_cash_item(1, &store::Item::bundle(5_072_000 + u32::from(n), 1)).unwrap();
    }
    let before = store.cash_wallet(1).unwrap().maple_points;

    let out = s.handle(&buy_body(130200000));
    let r = out.iter().find(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT).expect("answered");
    assert_ne!(r.body[0], net::cashshop::RESULT_ITEM_TO_LOCKER, "a refusal, not a grant");
    assert_eq!(store.cash_wallet(1).unwrap().maple_points, before, "not a leaf point taken");
}

/// **A Magician can put points in Magic Claw, and a beginner cannot.**
///
/// The owner, 2026-08-27: *"I want to verify that all Magician 1st job skills are working first."*
/// The handler used to refuse every id outside the three beginner skills, and its comment said
/// why: *"what a job may learn is Skill.wz data nobody has read."* It has been read now.
///
/// Two things are asserted that a single-direction test would miss: that the ceiling is the
/// **skill's own** 20 rather than the beginner constant 3, and that the gate still refuses the
/// same skill for a character who has not advanced.
#[test]
fn a_magician_may_raise_magic_claw_and_a_beginner_may_not() {
    let path = std::path::Path::new("../../gm-handbook/skills.txt");
    if !path.exists() {
        return; // generated, gitignored - python tools/dump_skills.py
    }
    const MAGIC_CLAW: u32 = 2001003;

    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Mage".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let config = Config {
        skills: crate::skilltable::SkillTable::load(path),
        ..Config::default()
    };
    let mut s = Session::new(store.clone(), Arc::new(config));
    assert!(s.claim_for_character(id).contains("claimed the migration"));

    // Still a beginner: refused, and the refusal names the book rather than shrugging.
    // The real 0x013B shape: u32 tick, u32 skillId, u32 count.
    let ask = |count: u32| {
        let mut b = net::skills::CLIENT_USER_SKILL_UP_REQUEST.to_le_bytes().to_vec();
        b.extend_from_slice(&0x1187_0e94u32.to_le_bytes());
        b.extend_from_slice(&MAGIC_CLAW.to_le_bytes());
        b.extend_from_slice(&count.to_le_bytes());
        b
    };
    let out = s.handle(&ask(1));
    assert!(
        out[0].what.contains("job book 200"),
        "a beginner must be told WHY: {}",
        out[0].what
    );
    assert_eq!(store.skill_level(id, MAGIC_CLAW).unwrap_or(0), 0, "and nothing was granted");

    // Advance, then it is allowed - and the ceiling is Magic Claw's own 20, not the
    // beginner constant 3. A bulk request for 99 must clamp to 20, not to 3.
    let mut chr = s.claimed_character().unwrap();
    chr.job = 200;
    // **Level 30, because skill points are a real balance now.** Before the ledger a level-0
    // character could raise anything; `entitlement(First, level)` gates it, and a character
    // who has earned nothing is correctly refused. That is the feature, not a broken test.
    chr.level = 30;
    store.save_character_progress(&chr).unwrap();
    s.handle(&ask(99));
    assert_eq!(
        store.skill_level(id, MAGIC_CLAW).unwrap_or(0),
        20,
        "Magic Claw stops at 20 - clamping to the beginner 3 would look like a refused click"
    );
}

/// **A job advancement carries the skill points with it, in one packet.**
///
/// The owner, 2026-08-27: *"Once I became a Magician (or any job at level 10), I should immediately
/// get 1 skill point for 1st job. Currently I get none."* They got none because the packet
/// carried the job bit alone, so the pool was empty and the client greys the `+` button when
/// the pool reads zero - with nothing on screen to say why.
///
/// The **tier** is the assertion that matters. `FUN_1402CB030` returns 0 for any pool key
/// above 10, so putting the job id `200` in that byte would read an empty pool and look
/// exactly like the bug being fixed.
#[test]
fn a_job_advancement_carries_its_skill_points() {
    let (mut s, store, id) = gm_session();
    let mut chr = s.claimed_character().unwrap();
    chr.level = 10;
    store.save_character_progress(&chr).unwrap();

    let out = s.handle(&gm_chat("!job 200"));
    let stat = out
        .iter()
        .find(|r| r.opcode == net::stats::STAT_CHANGED)
        .expect("the job change goes out");
    assert!(stat.what.contains("tier 1 = 1"), "one point, tier 1: {}", stat.what);
    assert!(!stat.what.contains("tier 1 = 200"), "the TIER, never the job id");

    // Level 11 owes four - the owner's own worked example - and it is one packet, not two.
    let mut chr = s.claimed_character().unwrap();
    chr.level = 11;
    store.save_character_progress(&chr).unwrap();
    let out = s.handle(&gm_chat("!job 200"));
    let stat = out.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).expect("sent");
    assert!(stat.what.contains("tier 1 = 4"), "1 + 3 for the level: {}", stat.what);
    assert_eq!(
        out.iter().filter(|r| r.opcode == net::stats::STAT_CHANGED).count(),
        1,
        "ONE packet - the client reads the SP encoding from the job in this same body"
    );

    // A beginner is owed nothing, and the packet must still be legal.
    let mut chr = s.claimed_character().unwrap();
    chr.level = 5;
    store.save_character_progress(&chr).unwrap();
    let out = s.handle(&gm_chat("!job 0"));
    assert!(
        out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
        "a level-5 beginner still gets a job packet, just an empty pool"
    );
    let _ = id;
}

/// **A point into an attack skill warns when the stat behind it is missing.**
///
/// The owner, 2026-08-28: *"I added all of the points into Magic Claw, but the skill only deals 1
/// damage, which is definitely not correct."* It was correct - they were a Rogue with **6 INT**
/// wearing a Magician's job id, and `magic::cobalt` pins that the formula predicts exactly the
/// 1 they saw. What was missing was anything on screen saying so.
///
/// Both directions are asserted. A warning that fired on every point would be noise, and noise
/// is how a warning stops being read.
#[test]
fn spending_a_point_on_an_attack_skill_warns_when_the_stat_is_missing() {
    let path = std::path::Path::new("../../gm-handbook/skills.txt");
    if !path.exists() {
        return;
    }
    const MAGIC_CLAW: u32 = 2001003;
    let ask = |count: u32| {
        let mut b = net::skills::CLIENT_USER_SKILL_UP_REQUEST.to_le_bytes().to_vec();
        b.extend_from_slice(&0x1187_0e94u32.to_le_bytes());
        b.extend_from_slice(&MAGIC_CLAW.to_le_bytes());
        b.extend_from_slice(&count.to_le_bytes());
        b
    };
    let build = |int: u16| {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
        let chr = net::opcode::Character { name: "Mage".to_string(), ..Default::default() };
        let mut made = store.create_character(account_id, 0, &chr).unwrap();
        made.job = 200;
        made.level = 30;
        made.intelligence = int;
        store.save_character_progress(&made).unwrap();
        store.create_migration(account_id, made.id, 0, 0).unwrap();
        let config = Config {
            skills: crate::skilltable::SkillTable::load(path),
            ..Config::default()
        };
        let mut s = Session::new(store, Arc::new(config));
        s.claim_for_character(made.id);
        s
    };

    // Cobalt's INT. The point is still granted - the stat can be raised afterwards, so a
    // refusal would be wrong as well as annoying - but they are told.
    let mut s = build(6);
    let out = s.handle(&ask(1));
    let warned = out.iter().any(|r| {
        r.opcode == net::notice::CHAT_NOTICE && notice_text(r).contains("INT")
    });
    assert!(warned, "a 6-INT Magician must be told why Magic Claw will hit for 1");

    // A real Magician gets no chat line at all.
    let mut s = build(60);
    let out = s.handle(&ask(1));
    assert!(
        !out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE),
        "60 INT must NOT be nagged - a warning on every point is noise"
    );
}

/// **`!resetap` conserves the total, in both directions and twice over.**
///
/// The invariant is the whole design: nothing here knows the per-level AP award, so the reset
/// refunds the difference from a fresh character's stats rather than recomputing a number it
/// would have to invent. Points in equals points out, and a second run is a no-op.
#[test]
fn resetap_gives_back_exactly_what_was_spent() {
    let (mut s, store, _id) = gm_session();
    let floor = 4u16; // crate::session::gm::AP_RESET_FLOOR
    let mut chr = s.claimed_character().unwrap();
    // Cobalt's real spread from the 2026-08-28 run, with STR taken one BELOW the floor so the
    // "not corrected upward" rule still has a case. At the old floor of 12/5/4/4 a STR of 4
    // was under it; at 4/4/4/4 it is exactly on it, and the test would have stopped checking
    // the thing it was written for without failing.
    chr.strength = 3;
    chr.dexterity = 24;
    chr.intelligence = 6;
    chr.luck = 36;
    chr.ap = 10;
    store.save_character_progress(&chr).unwrap();
    let before_total = 3 + 24 + 6 + 36 + 10;

    let out = s.handle(&gm_chat("!resetap"));
    let after = s.claimed_character().unwrap();
    let after_total = after.strength + after.dexterity + after.intelligence + after.luck + after.ap;
    assert_eq!(after_total, before_total, "not one point created or destroyed");
    assert_eq!(after.dexterity, floor, "DEX back to the floor");
    assert_eq!(after.luck, floor, "LUK back to the floor");
    assert_eq!(after.intelligence, floor, "INT back to the floor");
    // **STR was BELOW the floor and must be left alone**, not topped up - a reset that hands
    // out free stats is worse than one that occasionally refunds nothing.
    assert_eq!(after.strength, 3, "a stat under the floor is not corrected upward");
    assert!(after.ap > 10, "and the difference is in the pool: {}", after.ap);

    // One packet, carrying all five fields - the stat window reads them together.
    let stat = out.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).expect("sent");
    assert!(stat.what.contains("ability reset"), "{}", stat.what);
    assert_eq!(out.iter().filter(|r| r.opcode == net::stats::STAT_CHANGED).count(), 1);

    // **Idempotent.** Running it again refunds nothing.
    let ap_now = after.ap;
    s.handle(&gm_chat("!resetap"));
    assert_eq!(s.claimed_character().unwrap().ap, ap_now, "a second reset is a no-op");
}

/// The floor is **4/4/4/4**, not the 12/5/4/4 a character is created with.
///
/// The owner, 2026-08-29: *"the character should only have 4, 4, 4, 4 in STR, DEX, INT and LUK,
/// that represents the lowest amount of AP available for characters to increase from."* A
/// reset that stopped at 12 STR would strand eight points in a stat the player may not want,
/// which is the situation the command exists to get out of.
#[test]
fn resetap_floors_every_stat_at_four_not_at_the_creation_spread() {
    let (mut s, store, _id) = gm_session();
    let mut chr = s.claimed_character().unwrap();
    chr.strength = 30;
    chr.dexterity = 30;
    chr.intelligence = 30;
    chr.luck = 30;
    chr.ap = 0;
    store.save_character_progress(&chr).unwrap();

    s.handle(&gm_chat("!resetap"));
    let after = s.claimed_character().unwrap();
    assert_eq!(
        (after.strength, after.dexterity, after.intelligence, after.luck),
        (4, 4, 4, 4),
        "the creation spread 12/5/4/4 would leave 8 STR and 1 DEX stranded"
    );
    assert_eq!(after.ap, (30 - 4) * 4, "and every point above the floor is in the pool");
}

/// Points spent on HP and MP come back too, and the max values come down with them.
#[test]
fn resetap_refunds_ability_points_spent_on_hp_and_mp() {
    let (mut s, store, _id) = gm_session();
    let chr = s.claimed_character().unwrap();
    let (base_hp, base_mp, base_ap) = (chr.max_hp, chr.max_mp, chr.ap);

    // Spend as the ability-up handler would: raise the maxima AND record the points.
    //
    // The four stats are put ON the floor first, so this test measures the HP/MP refund alone.
    // Left at the creation spread of 12/5/4/4 they contribute nine points of their own, and
    // the assertion below would be checking two things at once while naming one.
    let mut spent = chr.clone();
    spent.strength = 4;
    spent.dexterity = 4;
    spent.intelligence = 4;
    spent.luck = 4;
    spent.max_hp += 3 * net::abilityup::policy::MAX_HP_PER_AP;
    spent.max_mp += 2 * net::abilityup::policy::MAX_MP_PER_AP;
    store.save_character_progress(&spent).unwrap();
    store.record_ap_spend(spent.id, 3, 2).unwrap();

    s.handle(&gm_chat("!resetap"));
    let after = s.claimed_character().unwrap();
    assert_eq!(after.max_hp, base_hp, "the HP those points bought is gone");
    assert_eq!(after.max_mp, base_mp, "and the MP");
    assert_eq!(after.ap, base_ap + 5, "all five points are back in the pool");
    // And the counters were cleared, so a second run cannot hand them out again.
    assert_eq!(store.ap_spend(after.id).unwrap().total(), 0);
    s.handle(&gm_chat("!resetap"));
    assert_eq!(s.claimed_character().unwrap().ap, base_ap + 5, "a second reset refunds nothing");
}

/// Current HP is never left above maximum by the refund - the bar would draw past its end.
#[test]
fn resetap_pulls_current_hp_down_with_the_maximum() {
    let (mut s, store, _id) = gm_session();
    let mut chr = s.claimed_character().unwrap();
    chr.max_hp += 5 * net::abilityup::policy::MAX_HP_PER_AP;
    chr.hp = chr.max_hp; // full, on the inflated bar
    store.save_character_progress(&chr).unwrap();
    store.record_ap_spend(chr.id, 5, 0).unwrap();

    s.handle(&gm_chat("!resetap"));
    let after = s.claimed_character().unwrap();
    assert!(after.hp <= after.max_hp, "{} hp on a {} bar", after.hp, after.max_hp);
}

/// **The gate, and the control for it.** The owner, 2026-08-29: *"make GM commands only available
/// to accounts with GM status."*
///
/// Both halves matter. Without the second, a gate that refused everything would pass.
#[test]
fn gm_commands_are_refused_without_gm_status_and_allowed_with_it() {
    let (mut s, store, _id) = gm_session();

    // The helper grants it, so take it away and watch the same command stop working.
    store.set_gm("maplecw", false).unwrap();
    let refused = s.handle(&gm_chat("!heal"));
    assert!(
        refused.iter().all(|r| r.opcode != net::stats::STAT_CHANGED),
        "!heal must not heal without GM status"
    );
    // And it is SAID rather than refused: to an account that cannot run commands, "!heal" is
    // just text someone typed.
    let said = refused
        .iter()
        .find(|r| r.opcode == net::userchat::USER_CHAT)
        .map(|r| r.what.clone())
        .expect("a non-GM's ! line goes out as ordinary chat");

    // The positive control: the identical command, with the flag back on.
    store.set_gm("maplecw", true).unwrap();
    let allowed = s.handle(&gm_chat("!heal"));
    assert!(
        allowed.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
        "with GM status !heal must work - otherwise the refusal above proves nothing. \
         Refusal said: {said}"
    );
}

/// **The GM gate must not touch ordinary chat.** A non-GM who cannot speak would be a far
/// worse regression than the one the gate prevents, and it is one misplaced line away: the
/// gate sits inside `on_chat`, and moving it above the `!` check would silence everybody.
///
/// Nothing else pinned this, so it is pinned here.
#[test]
fn a_non_gm_can_still_talk() {
    let (mut s, store, _id) = gm_session();
    store.set_gm("maplecw", false).unwrap();

    let out = s.handle(&gm_chat("Hello"));
    assert!(
        out.iter().any(|r| r.opcode == net::userchat::USER_CHAT),
        "plain chat must still be said out loud: {:?}",
        out.iter().map(|r| &r.what).collect::<Vec<_>>()
    );
    assert!(
        !out.iter().any(|r| r.what.contains("does not have GM status")),
        "and it must not be answered as a refused command"
    );
}

/// **`!help` and `!rates` are the two public commands; everything else a player types with a
/// bang is chat.** The owner, 2026-08-29: *"All commands should have this gate for now until
/// otherwise specified"*; the owner, 2026-09-06: `!rates` is public and `!help` shows a player
/// only what they may run. So the two are answered with a notice and nothing else, and
/// `!item` / `!map` are still said out loud with no notice, no stat change and no item.
#[test]
fn help_and_rates_are_public_and_everything_else_is_said_out_loud() {
    let (mut s, store, _id) = gm_session();
    store.set_gm("maplecw", false).unwrap();
    for command in ["!help", "!rates"] {
        let out = s.handle(&gm_chat(command));
        assert_eq!(out.len(), 1, "{command}: one notice and nothing else: {out:?}");
        assert_eq!(out[0].opcode, net::notice::CHAT_NOTICE, "{command}");
    }
    let help = notice_text(&s.handle(&gm_chat("!help"))[0]);
    assert!(help.contains("!rates") && !help.contains("!item"), "a player's help: {help}");
    for command in ["!item 1302000", "!map 1"] {
        let out = s.handle(&gm_chat(command));
        // Said out loud, and nothing else. No notice, no stat change, no item.
        assert!(
            out.iter().any(|r| r.opcode == net::userchat::USER_CHAT),
            "{command} should have been said out loud: {:?}",
            out.iter().map(|r| &r.what).collect::<Vec<_>>()
        );
        assert!(
            out.iter().all(|r| r.opcode != net::stats::STAT_CHANGED
                && r.opcode != net::notice::CHAT_NOTICE),
            "{command} did something: {:?}",
            out.iter().map(|r| &r.what).collect::<Vec<_>>()
        );
    }
}

/// Put one equip in the bag and hand back the slot it landed in.
fn bag_an_equip(store: &Store, id: u32, item_id: u32) -> i16 {
    let placed = store
        .add_item(id, store::InventoryType::Equip, &store::Item::equip(item_id), 1)
        .expect("the equip goes in the bag");
    placed.first().expect("a slot").slot as i16
}

/// **An overall takes the bottom off.** The owner, 2026-08-29: *"when I wore an overall item, the
/// server did not take off the bottoms that I was wearing."*
///
/// `dressed_session` starts wearing a top in slot 5 and a bottom in slot 6. Slot 5's ordinary
/// swap already removed the top, because a top and an overall share that slot - the bottom
/// was the half nothing displaced, and it is what the owner saw.
#[test]
fn equipping_an_overall_takes_off_the_bottom() {
    let (mut s, store, id) = dressed_session();
    let robe = bag_an_equip(&store, id, 1_050_000);

    let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, robe, -5, -1));
    assert_eq!(out[0].body[0], 1, "still answered - a silent refusal locks the UI");

    let worn = store.equipped_items(id).unwrap();
    assert!(
        worn.iter().any(|e| e.slot == 5 && e.item_id == 1_050_000),
        "the robe should be on: {worn:?}"
    );
    assert!(
        !worn.iter().any(|e| e.slot == 6),
        "the bottom must have come off: {worn:?}"
    );
    // Into the bag, not destroyed.
    assert!(
        store.bag(id).unwrap().items.iter().any(|i| i.item.item_id == 1_060_002),
        "the bottom must be back in the bag"
    );
    assert!(out[1].what.contains("overall and a bottom cannot be worn together"), "{}", out[1].what);

    // **And the CLIENT is told the bottom came off** (the owner, 2026-10-01: *"it does happen in
    // the backend, I just need to change maps for it to show up"*). First the bottom, -6 into
    // the bag slot the store chose; then the robe's own swap, unchanged.
    let bag_slot = store
        .bag(id)
        .unwrap()
        .items
        .iter()
        .find(|i| i.item.item_id == 1_060_002)
        .map(|i| i.slot as i16)
        .unwrap();
    assert_eq!(out.len(), 2, "{out:?}");
    assert_eq!(out[0].body, net::inventory::inventory_move_result(net::inventory::INV_EQUIP, -6, bag_slot));
    assert_eq!(out[1].body, net::inventory::inventory_move_result(net::inventory::INV_EQUIP, robe, -5));
}

/// **The control, and it is the important half.** A plain top and a bottom are worn together
/// by every character in the game; taking the trousers off to put a shirt on would be a much
/// worse bug than the one being fixed.
#[test]
fn equipping_a_plain_top_leaves_the_bottom_alone() {
    let (mut s, store, id) = dressed_session();
    let shirt = bag_an_equip(&store, id, 1_040_000);

    s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, shirt, -5, -1));

    let worn = store.equipped_items(id).unwrap();
    assert!(
        worn.iter().any(|e| e.slot == 6 && e.item_id == 1_060_002),
        "a shirt does not take your trousers off: {worn:?}"
    );
}

/// The other direction. Enforcing only the first leaves the identical picture on screen from
/// the other order - robe on, then trousers over it.
#[test]
fn equipping_a_bottom_takes_off_a_worn_overall() {
    let (mut s, store, id) = dressed_session();
    let robe = bag_an_equip(&store, id, 1_050_000);
    s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, robe, -5, -1));
    assert!(store.equipped_items(id).unwrap().iter().any(|e| e.slot == 5 && e.item_id == 1_050_000));

    let trousers = bag_an_equip(&store, id, 1_060_000);
    let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, trousers, -6, -1));
    let robe_slot = store.bag(id).unwrap().items.iter().find(|i| i.item.item_id == 1_050_000).map(|i| i.slot as i16).unwrap();
    assert_eq!(out[0].body, net::inventory::inventory_move_result(net::inventory::INV_EQUIP, -5, robe_slot), "the robe's take-off reaches the client");

    let worn = store.equipped_items(id).unwrap();
    assert!(
        worn.iter().any(|e| e.slot == 6 && e.item_id == 1_060_000),
        "the trousers should be on: {worn:?}"
    );
    assert!(
        !worn.iter().any(|e| e.slot == 5 && e.item_id == 1_050_000),
        "and the robe must have come off: {worn:?}"
    );
}

/// A `0x013C`: `u32 skillId, u32 level`.
fn cast(skill_id: u32, level: u32) -> Vec<u8> {
    let mut b = net::buff::CLIENT_SKILL_USE.to_le_bytes().to_vec();
    b.extend_from_slice(&skill_id.to_le_bytes());
    b.extend_from_slice(&level.to_le_bytes());
    b
}

/// A `0x013F` naming exactly one CTS bit - the inverse of `net::buff::bits_in_mask`.
fn cancel(skill_id: u32, bit: u32) -> Vec<u8> {
    let mut b = net::buff::CLIENT_SKILL_CANCEL.to_le_bytes().to_vec();
    b.extend_from_slice(&skill_id.to_le_bytes());
    b.extend_from_slice(&[0u8; net::buff::CANCEL_MASK_OFFSET - 4]);
    let mut mask = [0u8; net::buff::MASK_LEN];
    let word = (bit / 32) as usize;
    let within = bit % 32;
    let value: u32 = 1 << (31 - within);
    mask[word * 4..word * 4 + 4].copy_from_slice(&value.to_le_bytes());
    // The helper has to agree with the parser, or this test proves nothing about the wire.
    assert_eq!(net::buff::bits_in_mask(&mask), vec![bit]);
    b.extend_from_slice(&mask);
    b
}

/// A job-`job` character holding `skill_id` at `level`, claimed and on a field.
fn buffed_session(job: u16, skill_id: u32, level: u32) -> (Session, Arc<Store>, u32) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Caster".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.job = job;
    made.mp = 200;
    made.max_mp = 200;
    store.save_character_progress(&made).unwrap();
    store.set_skill_level(made.id, skill_id, level).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let config = Config::default();
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(made.id);
    (s, store, made.id)
}

/// **The three first-job buffs of the other branches now cast at all.**
///
/// Before this, `on_skill_use` looked in `net::buff` alone - three skills - and answered every
/// other cast with "this server does not grant that yet". Iron Body, Focus and Dark Sight are
/// half of what a Warrior, a Bowman and a Thief have to show for a first job.
///
/// Their CTS bits came from **re-deriving** the index-to-name table on 2026-08-28: the earlier
/// pass produced 323 names, this one 408, and the 85 it had been missing include every bit
/// needed here. The old absence was a property of the search.
#[test]
fn iron_body_focus_and_dark_sight_all_cast() {
    for (job, skill, level, bits) in [
        (100u16, net::jobbuffs::IRON_BODY, 20u32, 1usize),
        (300, net::jobbuffs::FOCUS, 20, 2),
        (400, net::jobbuffs::DARK_SIGHT, 1, 2),
    ] {
        let (mut s, _store, _id) = buffed_session(job, skill, level);
        let out = s.handle(&cast(skill, level));
        let granted = out
            .iter()
            .find(|r| r.opcode == net::buff::TEMPORARY_STAT_SET)
            .unwrap_or_else(|| panic!("skill {skill} must grant a temporary stat"));
        assert!(!granted.body.is_empty());
        // The MP is spent in the same exchange - every effect hangs off the one transition.
        assert!(
            out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
            "skill {skill} must also spend MP"
        );
        // How many stats it grants is the thing a single-stat call site would get wrong.
        let level_row = net::jobbuffs::buff_level(skill, level, 100).expect("in the table");
        assert_eq!(
            level_row.all_granted_by(skill).len(),
            bits,
            "skill {skill} grants {bits} stat(s)"
        );
    }
}

/// **Cancelling Dark Sight clears the Speed penalty too, not just the invisibility.**
///
/// `research/first-job-buffs.md` §7 item 3: the right-click names the bit the player clicked.
/// If the server cleared only that bit, the half left behind for Dark Sight is the
/// **drawback**: a cancelled buff and a character permanently slow for no visible reason,
/// which nobody reports as a buff bug. Focus has the same shape with Accuracy and
/// Avoidability.
///
/// This is the mirror of `grant_buff_with_tail`'s `all_granted_by` note: the same mistake, in
/// the other direction, on the other path.
#[test]
fn cancelling_a_two_stat_buff_clears_both_of_its_bits() {
    let (mut s, _store, _id) = buffed_session(400, net::jobbuffs::DARK_SIGHT, 1);
    s.handle(&cast(net::jobbuffs::DARK_SIGHT, 1));

    // The client right-clicks the icon, which names ONE bit: invisibility.
    let out = s.handle(&cancel(net::jobbuffs::DARK_SIGHT, net::jobbuffs::CTS_DARK_SIGHT));
    let reset = out
        .iter()
        .find(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET)
        .expect("the cancel must be answered");
    let cleared = net::buff::bits_in_mask(&reset.body[3..3 + net::buff::MASK_LEN]);
    assert!(
        cleared.contains(&net::jobbuffs::CTS_DARK_SIGHT),
        "the bit that was clicked: {cleared:?}"
    );
    assert!(
        cleared.contains(&net::buff::CTS_SPEED),
        "and the Speed penalty it came with, or the player stays slow forever: {cleared:?}"
    );

    // Nothing is left held, so a second right-click has nothing to do.
    let out = s.handle(&cancel(net::jobbuffs::DARK_SIGHT, net::jobbuffs::CTS_DARK_SIGHT));
    assert!(
        !out.iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_RESET),
        "both bits were already released"
    );
}

/// **Disorder is answered and grants nothing**, because it is a debuff on the mob.
///
/// `jobbuffs::buff_level` returns `None` for it deliberately, and this asserts the refusal is
/// a *chat line* rather than silence. An unanswered request is the frozen-UI failure this
/// project has paid for repeatedly - and `0x013C` is the one request that does not latch, so
/// the honest answer here is words on screen.
#[test]
fn disorder_is_answered_even_though_it_grants_no_stat() {
    let (mut s, _store, _id) = buffed_session(400, net::jobbuffs::DISORDER, 20);
    let out = s.handle(&cast(net::jobbuffs::DISORDER, 20));
    assert!(!out.is_empty(), "an unanswered 0x013C is how a UI freezes");
    assert!(
        !out.iter().any(|r| r.opcode == net::buff::TEMPORARY_STAT_SET),
        "Disorder sets no stat on the player"
    );
    assert!(notice_text(&out[0]).contains("Disorder"), "{}", notice_text(&out[0]));
}

/// **The Cash Shop silences idle chatter, and only the chatter.**
///
/// The owner, 2026-08-26: *"we should fix NPC idle chatter when player is in cash shop."* About
/// forty `0x0453`s per visit were going out addressed to NPC object ids on a field the client
/// had put away.
///
/// Three states are asserted, not one. A test that only checked "silent while in the shop"
/// would pass just as happily if the gate never cleared and the field went quiet for the rest
/// of the session - which is the more annoying bug of the two, and the one that would take a
/// client run to notice.
///
/// The fourth assertion is the one `CLAUDE.md`'s Heena rule asks for: the gate must **not**
/// take the rest of `tick` with it. Drop sweeps, respawns and buff expiry are properties of
/// the world rather than of what is on screen, and a gate placed one line too early would stop
/// all three while looking exactly like this test passing.
#[test]
fn the_cash_shop_silences_idle_chatter_and_the_field_gets_it_back() {
    let npcs = vec![net::opcode::FieldNpc {
        object_id: 1000, template_id: 8, x: 69, cy: 275, fh: 30,
        rx0: 19, rx1: 119, f: 0,
    }];
    let mut strings = std::collections::HashMap::new();
    strings.insert(
        8u32,
        crate::config::NpcStrings {
            name: "Robin".into(),
            info: (0..4).map(|i| format!("line {i}")).collect(),
            ..Default::default()
        },
    );
    let config = Config {
        npcs: [(40u32, npcs)].into_iter().collect(),
        npc_strings: strings.into(),
        ..Config::default()
    };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character {
        name: "Shopper".to_string(), map_id: 40, ..Default::default()
    };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(id);
    s.on_field_entered();

    // How many balloons a 60-second stretch produces, starting from `from`.
    let balloons = |s: &mut Session, from: u64| {
        let mut n = 0;
        for now in (from..from + 60_000).step_by(250) {
            n += s.tick(now).iter().filter(|r| r.opcode == net::npcchat::NPC_CHAT).count();
        }
        n
    };

    // 1. On the field, Robin talks.
    assert!(balloons(&mut s, 0) > 0, "an NPC with four lines must speak on the field");

    // 2. The Cash Shop button, and then silence. `0x00D5` with the real body shape.
    let mut open = net::cashshop::CLIENT_CASH_SHOP_REQUEST.to_le_bytes().to_vec();
    open.extend_from_slice(&[0u8; 5]);
    let out = s.handle(&open);
    assert!(
        out.iter().any(|r| r.opcode == net::cashshop::SET_CASH_SHOP),
        "the shop must actually open, or this test is measuring nothing"
    );
    assert_eq!(balloons(&mut s, 60_000), 0, "no balloons while the field is not being drawn");

    // 3. Exit - `0x00D1` with an EMPTY body is the button, not a portal - and Robin is back.
    let out = s.handle(&super::CLIENT_TRANSFER_FIELD.to_le_bytes());
    assert!(
        !out.is_empty(),
        "the Exit button must be answered; an unanswered one freezes the whole UI"
    );
    s.on_field_entered();
    assert!(balloons(&mut s, 130_000) > 0, "leaving the shop must give the field its voice back");

    // 4. The gate is chatter-only. With the shop open again, a tick still carries the rest of
    //    the world - here, the regeneration that `regen_tick` produces for a damaged
    //    character. If this ever fails, the `in_cash_shop` return moved too far up `tick`.
    let mut hurt = s.claimed_character().unwrap();
    hurt.hp = 1;
    store.save_character_progress(&hurt).unwrap();
    s.handle(&open);
    let mut healed = false;
    for now in (200_000..320_000).step_by(250) {
        if s.tick(now).iter().any(|r| r.opcode == net::stats::STAT_CHANGED) {
            healed = true;
            break;
        }
    }
    assert!(healed, "the Cash Shop must not stop regeneration - it gates the chatter only");
}

/// **`!learn` puts a whole branch on the bar, and all four of its effects are asserted.**
///
/// The owner, 2026-08-28: *"I need all 1st job skills of all branches to have their damage
/// calculation ready and their skills available to test next session."* Reaching 24 skills
/// through `0x013B` means levelling four times inside one run that costs a manual launch.
///
/// `CLAUDE.md`'s Heena rule: *"when a handler produces N effects, the test has to say something
/// about N of them"* - the turn-in test counted fanfares, the one effect that was correctly
/// gated, and passed on every run while the experience doubled beside it. This handler produces
/// **four**: the database rows, the `0x0081` the client draws from, the chat report, and the
/// low-stat warning. All four are below.
#[test]
fn learn_grants_a_whole_job_book_at_once() {
    let path = std::path::Path::new("../../gm-handbook/skills.txt");
    if !path.exists() {
        return; // generated, gitignored - python tools/dump_skills.py
    }
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Ranger".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.job = 300;
    made.dexterity = 60; // a real Bowman, so the warning must stay silent
    store.save_character_progress(&made).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let config = Config {
        skills: crate::skilltable::SkillTable::load(path),
        ..Config::default()
    };
    let mut s = Session::new(store.clone(), Arc::new(config));
    assert!(s.claim_for_character(made.id).contains("claimed the migration"));

    let out = s.handle(&gm_chat("!learn 5"));

    // 1. The database. Six skills is the Bowman first-job book: 3000000, 3000001, 3001000,
    //    3001001, 3001002, 3001003. The count is asserted against the table rather than
    //    hard-coded, so regenerating `skills.txt` cannot silently make this test wrong.
    let expected: Vec<u32> =
        s.config.skills.book(300).iter().map(|sk| sk.id).collect();
    assert_eq!(expected.len(), 6, "the Bowman first-job book is six skills: {expected:?}");
    let learned = store.skills(made.id).unwrap();
    assert_eq!(learned.len(), 6, "every skill in the book must persist");
    for sk in &learned {
        assert!(expected.contains(&sk.id), "{} is not in the Bowman book", sk.id);
        assert_eq!(sk.level, 5, "skill {} should be level 5", sk.id);
    }

    // 2. The client is told, in ONE packet. Six separate `0x0081`s would each clear the latch
    //    and five of them would be answering nothing.
    let records: Vec<_> =
        out.iter().filter(|r| r.opcode == net::skills::CHANGE_SKILL_RECORD_RESULT).collect();
    assert_eq!(records.len(), 1, "one skill record, not one per skill");
    assert!(records[0].what.contains("6 skill(s)"), "{}", records[0].what);

    // 3. The chat report names them, or nothing on screen says what just happened.
    assert!(notice_text(&out[0]).contains("Arrow Blow"), "{}", notice_text(&out[0]));

    // 4. A real Bowman is NOT nagged. The warning fires on the stat, not on the command.
    assert!(
        !notice_text(&out[0]).contains("WARNING"),
        "60 DEX must not be warned: {}",
        notice_text(&out[0])
    );
}

/// **`!learn` clamps to each skill's own ceiling, not to one constant.**
///
/// The Magician book runs to 15 *and* 20 - Magic Guard stops at 15, Magic Claw at 20 - so a
/// single clamp is wrong for half the book whichever number it is. This is the same mistake
/// `on_skill_up` made with `BEGINNER_SKILL_MAX_LEVEL`, which is 3 and right only for the three
/// beginner skills.
///
/// The low-stat warning's other direction is asserted here too: this character has the default
/// INT, so it must fire.
#[test]
fn learn_clamps_each_skill_to_its_own_maximum() {
    let path = std::path::Path::new("../../gm-handbook/skills.txt");
    if !path.exists() {
        return;
    }
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Cobalt".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.job = 200;
    made.level = 30;
    store.save_character_progress(&made).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let config = Config {
        skills: crate::skilltable::SkillTable::load(path),
        ..Config::default()
    };
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(made.id);

    // Ask for 99 - far above every ceiling in the book.
    let out = s.handle(&gm_chat("!learn 99"));
    let levels: std::collections::HashMap<u32, u32> =
        store.skills(made.id).unwrap().iter().map(|sk| (sk.id, sk.level)).collect();
    assert_eq!(levels.get(&2001000), Some(&15), "Magic Guard stops at 15");
    assert_eq!(levels.get(&2001003), Some(&20), "Magic Claw stops at 20");
    for (id, level) in &levels {
        let ceiling = s.config.skills.max_level(*id).expect("in the table");
        assert_eq!(*level, ceiling, "skill {id} must sit on its own ceiling");
    }

    // The warning fires: default INT with a Magician job id is exactly Cobalt's situation.
    assert!(
        out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE
            && notice_text(r).contains("INT")),
        "a Magician with no INT must be told the attack skills will hit for 1"
    );
}

/// **`!learn` refuses another branch's skill by asking the same predicate `0x013B` asks.**
///
/// Two copies of "what may this job have" is how one of them ends up wrong. This asserts the
/// bulk command and the single-point handler agree, rather than asserting a second list.
#[test]
fn learn_refuses_a_skill_from_another_branch() {
    let path = std::path::Path::new("../../gm-handbook/skills.txt");
    if !path.exists() {
        return;
    }
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Ranger".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.job = 300;
    store.save_character_progress(&made).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let config = Config {
        skills: crate::skilltable::SkillTable::load(path),
        ..Config::default()
    };
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(made.id);

    // Magic Claw, on a Bowman.
    let out = s.handle(&gm_chat("!learn 2001003 5"));
    assert!(store.skills(made.id).unwrap().is_empty(), "nothing may be granted");
    assert!(
        !out.iter().any(|r| r.opcode == net::skills::CHANGE_SKILL_RECORD_RESULT),
        "and no empty change list is sent"
    );
    let said = notice_text(&out[0]);
    assert!(said.contains("job book 200"), "the refusal must name the book: {said}");

    // The single-point handler refuses it too. Same predicate, so this is a check that the
    // two paths cannot drift, not a duplicate of the assertion above.
    assert!(!s.config.skills.may_learn(300, 2001003));
}

/// **`!resetsp` forgets every skill and tells the client**, or the client keeps drawing them.
#[test]
fn resetsp_forgets_every_skill_and_says_so() {
    let (mut s, store, id) = gm_session();
    store.set_skill_level(id, 1002, 3).unwrap();
    store.set_skill_level(id, 1000, 1).unwrap();
    assert_eq!(store.skills(id).unwrap().len(), 2);

    let out = s.handle(&gm_chat("!resetsp"));
    assert!(store.skills(id).unwrap().is_empty(), "every skill is gone from the database");

    // The client is told, and with the FORGET encoding - a Learn at level 0 does not reach
    // the client's erase arm.
    let reply = out
        .iter()
        .find(|r| r.opcode == net::skills::CHANGE_SKILL_RECORD_RESULT)
        .expect("the client must be told, or it keeps drawing the old levels");
    assert!(reply.what.contains("forgot 2"), "{}", reply.what);

    // With nothing learned it says so rather than sending an empty change list.
    let out = s.handle(&gm_chat("!resetsp"));
    assert!(notice_text(&out[0]).contains("no skills to forget"), "{}", notice_text(&out[0]));
    assert!(
        !out.iter().any(|r| r.opcode == net::skills::CHANGE_SKILL_RECORD_RESULT),
        "and sends no empty change list"
    );
}

/// **Magic Guard sends part of a hit to MP, and that is the server's arithmetic.**
///
/// The client computes the split and then **never writes HP** - only `0x007C` moves either
/// bar - so setting CTS bit 97 alone would buy an icon and change nothing about how much a hit
/// hurt. Four things are asserted, because a test of one would pass while the others were
/// wrong: the total taken, the MP share, the HP share, and that **both travel in one packet**.
#[test]
fn magic_guard_splits_incoming_damage_between_hp_and_mp() {
    // Template 45 is the Drake, PADamage 287 - a hit big enough that a percentage of it is
    // not lost to integer division, which a snail's 3 would be.
    let hit_body = || {
        hex("00000000ffffffff0100000002002100431e140f0000000000000000000001000000010000000100000001000000d3070000d307000001000000000000000000000000000000000000de0100008b010000000000000000000000000000ffffffff00000000ffffffff000000000000000002000000000000000000000000000000000000000100000000000000000000000000")
    };
    let build = || {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
        let chr = net::opcode::Character { name: "Mage".to_string(), ..Default::default() };
        let mut made = store.create_character(account_id, 0, &chr).unwrap();
        made.level = 7;
        made.job = 200;
        made.max_hp = 5000;
        made.hp = 5000;
        made.max_mp = 5000;
        made.mp = 5000;
        store.save_character_progress(&made).unwrap();
        store.create_migration(account_id, made.id, 0, 0).unwrap();
        // A big attack column so the server's own number is large enough that a percentage of
        // it survives integer division.
        let config =
            Config { mob_attack: [(2u32, 300u32)].into_iter().collect(), ..Config::default() };
        let mut s = Session::new(store.clone(), Arc::new(config));
        s.claim_for_character(made.id);
        (s, store, made.id)
    };

    // **The control first.** Without the buff the whole hit lands on HP, so a pass below
    // cannot come from the split quietly doing nothing.
    let (mut s, _store, _id) = build();
    s.on_user_hit(&hit_body());
    let plain = s.claimed_character().unwrap();
    let plain_loss = 5000 - plain.hp;
    assert!(plain_loss > 10, "the control has to actually hurt: took {plain_loss}");
    assert_eq!(plain.mp, 5000, "and MP must not move without Magic Guard");

    // Now with it up, taking the same hit from the same state.
    let (mut s, store, id) = build();
    store.set_skill_level(id, net::buff::MAGIC_GUARD, 1).unwrap();
    let level = net::buff::buff_level(net::buff::MAGIC_GUARD, 1).expect("Magic Guard level 1");
    s.grant_buff_with_tail(net::buff::MAGIC_GUARD, level, 0, net::buff::TAIL_LEN);
    let mut chr = s.claimed_character().unwrap();
    chr.hp = 5000;
    chr.mp = 5000;
    store.save_character_progress(&chr).unwrap();

    let out = s.on_user_hit(&hit_body());
    let after = s.claimed_character().unwrap();
    let to_mp = 5000 - after.mp;
    let to_hp = 5000 - after.hp;
    let percent = u32::try_from(level.value).unwrap();
    let total = to_hp + to_mp;
    assert!(to_mp > 0, "Magic Guard must send SOME of it to MP - level 1 is {percent}%");
    // **The invariant is checked WITHIN one hit, not across two.** `incoming_damage` rolls a
    // window, so two hits legitimately differ - the first version of this test compared the
    // totals of two separate hits and failed on 429 vs 389, which is the damage model working
    // rather than the split being wrong.
    assert_eq!(to_mp, total * percent / 100, "the same arithmetic the client does");
    assert!(to_hp > 0, "and the rest still lands on HP at {percent}%");
    assert!(total > 10, "the hit has to be big enough to divide: took {total}");

    // **One packet.** Two would let the client draw the HP bar against a stale MP value, and
    // the revive dialog gates on the HP this very packet sets.
    let stats: Vec<_> = out.iter().filter(|r| r.opcode == net::stats::STAT_CHANGED).collect();
    assert_eq!(stats.len(), 1, "HP and MP travel together");
    assert!(stats[0].what.contains("MAGIC GUARD"), "and the log says so: {}", stats[0].what);
}

/// An unreadable body still gets a `0x0572`, because the client's latch is cleared by nothing
/// else. This is the always-answer rule with a different field name.
#[test]
fn an_unreadable_storage_request_is_answered_rather_than_dropped() {
    let (mut s, _store, _id) = gm_session();
    for body in [vec![], vec![4u8], vec![99u8, 1, 2, 3], vec![7u8, 1, 2]] {
        let out = s.on_storage_request(&body);
        assert!(
            out.iter().any(|r| r.opcode == net::storage::STORAGE_RESULT),
            "body {body:02x?} must still be answered"
        );
    }
}

/// The `0x00F3` body the client sends when a **type-6 menu** line is clicked.
///
/// `net::script::parse_menu_reply` owns the shape: `u32 0, u8 6, u8 1, u32 selection` for a
/// pick and `u32 0, u8 6, u8 0` for a Close. Written out here rather than built through the
/// decoder, so a decoder that changed shape would fail this test instead of agreeing with it.
fn menu_reply(selection: Option<u32>) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&0u32.to_le_bytes()); // handle
    b.push(net::script::SCRIPT_TYPE_MENU);
    match selection {
        Some(n) => {
            b.push(1);
            b.extend_from_slice(&n.to_le_bytes());
        }
        None => b.push(0),
    }
    b
}

/// **The whole second advancement, walked the way a player walks it** - the Magician's, which
/// is the one the owner played on 2026-09-26: *"Upon accepting the quest, it should teleport me into
/// the test map with monsters that drop marbles, and the quest should automatically be set to in
/// progress with a completion requirement of 30 dark marbles"*; re-entry by regular talk while
/// the test is under way, "not ready yet" and "nothing more to teach" otherwise; and *"when
/// leaving the test area, the player should be placed right next to the spawn point at Magician
/// Job Instructor"*.
///
/// Every leg asserts its effect, not just the last one: a chain whose middle silently does
/// nothing still ends with the right job, which is the failure this project keeps finding. The
/// quest's opening is three `Say.0` lines with no `yes` branch - 20102's real shape - and the
/// deployed failure was exactly that: the third line ended on OK and nothing started.
#[test]
fn the_second_advancement_walks_the_client_s_own_chain() {
    let b = crate::secondjob::BRANCHES.iter().find(|b| b.from_job == 200).unwrap(); // Magician
    let field = b.test_field;
    let test_quest = b.chain.quests[2];

    let mut npcs = std::collections::HashMap::new();
    let npc = |template: u32| net::opcode::FieldNpc {
        object_id: 1000, template_id: template, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0,
    };
    npcs.insert(b.examiner_map_id, vec![npc(b.examiner_npc)]);
    npcs.insert(field.map_id, vec![npc(field.warden_npc)]);
    npcs.insert(b.instructor_map_id, vec![npc(b.instructor_npc)]);
    let mut quests = std::collections::HashMap::new();
    let mut say = std::collections::HashMap::new();
    say.insert("0".to_string(), vec!["So Grendel sent you.".to_string(), "There is a place near here.".to_string(), "Bring me #b30#k of them.".to_string()]);
    quests.insert(test_quest, crate::config::Quest { name: "Test of Qualification".into(), start_npc: Some(b.examiner_npc), say, ..Default::default() });
    let mut portal_index = std::collections::HashMap::new();
    portal_index.insert((b.examiner_map_id, crate::secondjob::EXAMINER_SPAWN_PORTAL.to_string()), 32u8);

    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Thirty".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.level = crate::secondjob::LEVEL_MINIMUM;
    made.job = b.from_job; // already a Magician - the chain's own Check.0.job.0
    made.map_id = b.examiner_map_id;
    store.save_character_progress(&made).unwrap();
    store.set_character_map(made.id, b.examiner_map_id).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let config = Config { npcs, quests, portal_index, ..Config::default() };
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(made.id);

    let map_of = |store: &Arc<Store>, id: u32| {
        store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().map_id
    };
    let job_of = |store: &Arc<Store>, id: u32| {
        store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().job
    };
    let held = |s: &Session, item: u32| s.held_count(made.id, item);
    let said = |out: &[Reply]| -> String {
        out.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).map(|r| String::from_utf8_lossy(&r.body).to_string()).collect()
    };
    let yes = net::script::SCRIPT_ACTION_YES;

    // ---- leg 0: before the quest, a regular talk warps nobody ------------------------------
    let out = s.handle(&npc_click(1000));
    assert!(said(&out).contains("not ready yet") && said(&out).contains(b.instructor_name), "{}", said(&out));
    assert_eq!(map_of(&store, made.id), b.examiner_map_id, "no quest, no warp");

    // ---- leg 1: accept the Test of Qualification (action 4, the client's own request) -------
    let out = s.handle(&quest_request(net::script::QUEST_ACTION_OPENING_SCRIPT, test_quest, b.examiner_npc));
    assert!(out.iter().any(|r| r.what.contains("line 1 of 3")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    let out = s.on_script_reply(&script_reply(1));
    assert!(out.iter().any(|r| r.what.contains("line 2 of 3")));
    let out = s.on_script_reply(&script_reply(1));
    let last = out.iter().find(|r| r.opcode == net::script::SCRIPT_MESSAGE).unwrap();
    assert!(last.what.contains("yes/no prompt") && last.what.contains("line 3 of 3"), "the last line is Accept/Decline: {}", last.what);
    assert!(store.quest_row(made.id, test_quest).unwrap().is_none(), "nothing starts before Accept");
    let out = s.on_script_reply(&script_reply(yes));
    assert_eq!(
        store.quest_row(made.id, test_quest).unwrap().map(|r| r.state),
        Some(store::QuestState::InProgress),
        "accepted: in progress, so the journal shows the 30 Dark Marbles"
    );
    assert!(out.iter().any(|r| r.opcode == net::quest::MESSAGE), "the quest record goes to the client");
    assert_eq!(map_of(&store, made.id), field.map_id, "and in they go");
    assert!(out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE), "said as a notice: a box would die with the SetField");

    // ---- leg 2: the warden lets them out BESIDE the instructor ------------------------------
    let out = s.handle(&npc_click(1000)); // the warden, because the map changed
    assert_eq!(map_of(&store, made.id), b.examiner_map_id, "the client's own returnMap");
    assert!(out.iter().any(|r| r.what.contains("at portal 32 (job00")), "job00, not portal 0: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());

    // ---- leg 3: short of marbles, a regular talk offers re-entry; No stays, Yes goes --------
    let etc = store::InventoryType::Etc;
    store.add_item(made.id, etc, &store::Item::bundle(b.chain.marble_item, 29), 200).unwrap();
    let out = s.handle(&npc_click(1000));
    assert!(out.iter().any(|r| r.what.contains("AskYesNo") && r.what.contains("back into the test area")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    s.on_script_reply(&script_reply(0));
    assert_eq!(map_of(&store, made.id), b.examiner_map_id, "No: they stay");
    s.handle(&npc_click(1000));
    s.on_script_reply(&script_reply(yes));
    assert_eq!(map_of(&store, made.id), field.map_id, "Yes: back in");
    assert_eq!(held(&s, b.chain.marble_item), 29, "and nothing was taken");
    s.handle(&npc_click(1000)); // the warden again

    // ---- leg 4: all thirty in hand - the talk points at the quest, and warps nobody --------
    store.add_item(made.id, etc, &store::Item::bundle(b.chain.marble_item, 1), 200).unwrap();
    let out = s.handle(&npc_click(1000));
    assert!(said(&out).contains("Test of Qualification"), "{}", said(&out));
    assert_eq!(map_of(&store, made.id), b.examiner_map_id);
    assert_eq!(held(&s, b.chain.marble_item), 30, "a talk takes nothing - the quest's turn-in does");

    // ---- leg 5: the instructor, and only the instructor, advances ---------------------------
    // The proof is what quest 20103's start hands over once the test is turned in.
    store.add_item(made.id, etc, &store::Item::bundle(b.chain.proof_item, 1), 200).unwrap();
    store.set_character_map(made.id, b.instructor_map_id).unwrap();
    s.claim_for_character(made.id); // re-read the character at its new map
    let out = s.handle(&npc_click(1000));
    let menu = out.iter().find(|r| r.opcode == net::script::SCRIPT_MESSAGE).expect("the instructor offers the choice");
    assert!(menu.what.contains("MENU"), "and it is a type-6 list, not a Say: {}", menu.what);
    assert_eq!(job_of(&store, made.id), b.from_job, "nothing has changed YET");
    let want = b.choices[0].job;
    let out = s.on_script_reply(&menu_reply(Some(0)));
    assert_eq!(job_of(&store, made.id), want, "the job must PERSIST, not just be announced");
    let stat = out.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).expect("the client must be told");
    assert!(stat.what.contains(&format!("job {} -> {want}", b.from_job)), "{}", stat.what);
    assert_eq!(held(&s, b.chain.proof_item), 0, "and the proof is spent, not left in the bag");

    // ---- leg 6: a second job has nothing more to learn from the examiner -------------------
    store.set_character_map(made.id, b.examiner_map_id).unwrap();
    s.claim_for_character(made.id);
    let out = s.handle(&npc_click(1000));
    assert!(said(&out).contains("nothing more to teach"), "{}", said(&out));
}

/// **Phil's guide is a menu the cursor can pick from, and the pick rides.**
///
/// The owner, 2026-09-15: *"Phil's dialogue to allow Beginners to choose a location to job advance
/// to does not work. The selection is fundamentally broken and cannot be selected by the
/// cursor."* It was a chain of yes/no boxes drawn to look like a list. This walks the fixed
/// path the way a player does: click Phil, get ONE type-6 box with a `#L` line per first job,
/// answer it with the client's own 10-byte pick, and land on the chosen instructor's map with
/// the job unchanged - Phil routes, the instructor advances. A Close sends nothing and leaves
/// the player where they stand.
#[test]
fn phils_job_guide_is_a_selectable_menu_and_the_pick_rides() {
    let phil = crate::jobguide::PHIL_TEMPLATE;
    let mut npcs = std::collections::HashMap::new();
    npcs.insert(
        crate::jobguide::PHIL_MAP,
        vec![net::opcode::FieldNpc { object_id: 1000, template_id: phil, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 }],
    );
    let fields = crate::jobs::FIRST_JOBS.iter().map(|j| j.map_id).chain([crate::jobguide::PHIL_MAP]).collect();

    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "LevelTen".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.level = crate::jobs::LEVEL_MINIMUM;
    made.job = 0;
    store.save_character_progress(&made).unwrap();
    store.set_character_map(made.id, crate::jobguide::PHIL_MAP).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let mut s = Session::new(store.clone(), Arc::new(Config { npcs, fields, ..Config::default() }));
    s.claim_for_character(made.id);
    let row = |store: &Arc<Store>| store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == made.id).unwrap();

    // ---- the click: one menu, parked -----------------------------------------------------
    let out = s.handle(&npc_click(1000));
    assert_eq!(out.len(), 1, "one box, not a chain: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert_eq!(out[0].opcode, net::script::SCRIPT_MESSAGE);
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_MENU, "type 6 - the list the taxi measured on screen");
    assert_eq!(out[0].body, net::script::npc_menu(phil, &crate::jobguide::menu_text()));
    let convo = s.conversation.clone().expect("the menu is parked");
    assert_eq!(convo.path, crate::jobguide::MENU_PATH);
    assert!(!convo.awaiting_yes_no, "a menu is not a yes/no box");

    // ---- Close: silence, and nobody moved --------------------------------------------------
    assert!(s.on_script_reply(&menu_reply(None)).is_empty());
    assert!(s.conversation.is_none());
    assert_eq!(row(&store).map_id, crate::jobguide::PHIL_MAP);

    // ---- the pick: line 2, the third first job, rides to ITS instructor's map -------------
    s.handle(&npc_click(1000));
    let dest = crate::jobguide::destination(2).unwrap();
    let out = s.on_script_reply(&menu_reply(Some(2)));
    assert!(out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE), "the arrival line, before the SetField");
    assert!(out.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "and the SetField itself: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(!out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE), "no script box beside a SetField - field entry tears it down");
    assert_eq!(row(&store).map_id, dest.map_id, "{}'s map, not the town", dest.npc_name);
    assert_eq!(row(&store).job, 0, "Phil routes; only the instructor advances");
    assert!(s.conversation.is_none());
}

/// **Shanks' waiver is a box of its own, and the boat leaves when it is dismissed.**
///
/// The owner, 2026-09-16: *"the dialogue that they'll waive it because you have finished Mai's
/// Final Training does not show. However, Shanks does correctly waive the fee and TP the
/// players."* The line shipped in the same batch as the `SetField` and field entry tore it
/// down. This pins the fix at the packet level: the Yes reply carries the Say and **no
/// SetField**; the dismissal carries the SetField and **no script**. And the waiver is for
/// Beginners only - the same character as a Swordsman pays and gets no box.
/// **The hair salons: the owner's coupon menu and pick-a-look box, the assistant's colours,
/// the Cash Shop pointer without a coupon, the colour kept on a style change and the style
/// kept on a colour change, and each salon its own lists.**
///
/// The owner, 2026-09-18: *"the Hair Salon Owners will take both the VIP mystery (random) or REG
/// Signature Coupons. The player get to choose which one they want to use if they have
/// both. The dialogue should list out all available coupons for use, or ask the player to
/// purchase one from the Cash Shop if there isn't one detected in the inventory. The Hair
/// Salon assistants will now deal with Hair Color."* A female character with blonde hair
/// (31000 + 3) at Natalie: no coupon is a Say linking both coupons and the Cash Shop; with
/// the Signature coupon only, a one-line menu; picking it opens the six Henesys REG female
/// styles in a type-0x0a box; index 4 is Rose 31230 and they end up in 31233 - Rose in blue,
/// the coupon gone, one 0x007C with the HAIR bit, the notice names the style. With both
/// coupons the menu has two lines, and the Mystery line rolls a Henesys VIP female style at
/// once. At Brittany the Signature Color coupon opens Rose in eight colours and index 7 is
/// brown 31237; the Mystery Color coupon rolls one of the eight. A male gets the male pools.
/// In Kerning, Don Giovanni offers the Kerning lists; Natalie standing there is no salon.
#[test]
fn the_salon_owner_does_styles_the_assistant_does_colours_and_each_keeps_the_other() {
    let mut npcs = std::collections::HashMap::new();
    npcs.insert(
        crate::salon::HENESYS_SALON_MAP,
        vec![
            net::opcode::FieldNpc { object_id: 1000, template_id: crate::salon::NATALIE, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 },
            net::opcode::FieldNpc { object_id: 1001, template_id: crate::salon::BRITTANY, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 },
        ],
    );
    npcs.insert(
        crate::salon::KERNING_SALON_MAP,
        vec![
            net::opcode::FieldNpc { object_id: 1000, template_id: crate::salon::DON_GIOVANNI, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 },
            net::opcode::FieldNpc { object_id: 1001, template_id: crate::salon::ANDRE, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 },
            net::opcode::FieldNpc { object_id: 1002, template_id: crate::salon::NATALIE, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 },
        ],
    );
    let mut hair_ids = std::collections::HashSet::new();
    for base in crate::salon::HENESYS_REG_FEMALE
        .iter()
        .chain(crate::salon::HENESYS_VIP_FEMALE)
        .chain(crate::salon::HENESYS_REG_MALE)
        .chain(crate::salon::KERNING_REG_FEMALE)
        .chain(crate::salon::KERNING_VIP_FEMALE)
        .chain(&[31_000])
    {
        for c in 0..8 {
            hair_ids.insert(base + c);
        }
    }
    let mut item_names = std::collections::HashMap::new();
    item_names.insert(31_233u32, "Rose".to_string());
    item_names.insert(31_237u32, "Rose".to_string());
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Blondie".to_string(), gender: 1, hair: 31_003, ..Default::default() };
    let made = store.create_character(account_id, 0, &chr).unwrap();
    store.set_character_map(made.id, crate::salon::HENESYS_SALON_MAP).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let mut s = Session::new(store.clone(), Arc::new(Config { npcs, hair_ids, item_names, ..Config::default() }));
    s.claim_for_character(made.id);
    let names = |out: &[Reply]| out.iter().map(|r| r.what.clone()).collect::<Vec<_>>();
    let hair_now = |store: &Arc<Store>| store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == made.id).unwrap().hair;
    let styles_in = |body: &[u8]| -> Vec<u32> {
        let b = &body[net::script::SCRIPT_HEAD_LEN..];
        let text_len = u16::from_le_bytes([b[4], b[5]]) as usize;
        let count_at = 4 + 2 + text_len + 1;
        (0..b[count_at] as usize).map(|k| u32::from_le_bytes(b[count_at + 1 + k * 4..count_at + 5 + k * 4].try_into().unwrap())).collect()
    };
    let text_of = |body: &[u8]| String::from_utf8_lossy(body).to_string();
    let avatar_pick = |index: u8| {
        let mut reply = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
        reply.extend_from_slice(&0u32.to_le_bytes());
        reply.extend_from_slice(&[net::script::SCRIPT_TYPE_AVATAR, 1, 0, 0]);
        reply.extend_from_slice(&0u32.to_le_bytes());
        reply.push(index);
        reply
    };
    let avatar_cancel = || {
        let mut cancel = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
        cancel.extend_from_slice(&0u32.to_le_bytes());
        cancel.extend_from_slice(&[net::script::SCRIPT_TYPE_AVATAR, 0]);
        cancel
    };
    let give = |who: u32, coupon: u32| store.add_item(who, store::InventoryType::Cash, &store::Item::bundle(coupon, 1), 1).unwrap();

    // 1. Natalie, no coupon: a Say that links BOTH style coupons and names the Cash Shop;
    //    nothing parked.
    let out = s.handle(&npc_click(1000));
    assert_eq!(out.len(), 1, "{:?}", names(&out));
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_SAY);
    let text = text_of(&out[0].body);
    assert!(text.contains("#i5150100#") && text.contains("#i5150000#") && text.contains("Cash Shop"), "{text}");
    assert!(s.conversation.is_none());
    assert_eq!(hair_now(&store), 31_003);

    // 2. With the Signature coupon only: a menu with the one line; picking it opens the
    //    avatar box with the six REG female bases.
    give(made.id, crate::salon::SIGNATURE_HAIR_COUPON);
    let out = s.handle(&npc_click(1000));
    assert_eq!(out.len(), 1, "{:?}", names(&out));
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_MENU);
    let text = text_of(&out[0].body);
    assert!(text.contains("#L0##i5150100#") && !text.contains("#L1#"), "{text}");
    assert_eq!(s.conversation.as_ref().unwrap().path, crate::salon::MENU_PATH);
    let out = s.on_script_reply(&menu_reply(Some(1)));
    assert!(out.is_empty(), "the Mystery line was not offered: {:?}", names(&out));
    assert!(s.conversation.is_none());
    let _ = s.handle(&npc_click(1000));
    let out = s.on_script_reply(&menu_reply(Some(0)));
    assert_eq!(out.len(), 1, "{:?}", names(&out));
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_AVATAR);
    assert_eq!(styles_in(&out[0].body), crate::salon::HENESYS_REG_FEMALE);
    assert_eq!(s.conversation.as_ref().unwrap().path, crate::salon::CHOICE_PATH);

    // 3. They pick index 4 (Rose, 31230): Rose in their blonde, the coupon spent, one HAIR
    //    redraw, the notice with the name.
    let out = s.handle(&avatar_pick(4));
    assert_eq!(hair_now(&store), 31_233, "Rose, blonde: base 31230 + colour 3");
    let stat: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::stats::STAT_CHANGED).collect();
    assert_eq!(stat.len(), 1, "{:?}", names(&out));
    assert!(stat[0].what.contains("HAIR bit -> 31233"), "{}", stat[0].what);
    assert!(out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "the coupon slot change: {:?}", names(&out));
    assert_eq!(s.held_count(made.id, crate::salon::SIGNATURE_HAIR_COUPON), 0, "spent");
    let notice = out.iter().find(|r| r.opcode == net::notice::CHAT_NOTICE).expect("a line");
    assert!(text_of(&notice.body).contains("Rose"));
    assert!(s.conversation.is_none());

    // 4. A cancel of the box, and a Close of the menu, spend nothing.
    give(made.id, crate::salon::SIGNATURE_HAIR_COUPON);
    let _ = s.handle(&npc_click(1000));
    let _ = s.on_script_reply(&menu_reply(Some(0)));
    let out = s.handle(&avatar_cancel());
    assert!(out.is_empty(), "{:?}", names(&out));
    let _ = s.handle(&npc_click(1000));
    let out = s.on_script_reply(&menu_reply(None));
    assert!(out.is_empty(), "{:?}", names(&out));
    assert_eq!(hair_now(&store), 31_233);
    assert_eq!(s.held_count(made.id, crate::salon::SIGNATURE_HAIR_COUPON), 1, "kept");

    // 5. Both coupons: two menu lines; the Mystery line rolls a Henesys VIP female style
    //    at once, in blonde, and spends only the Mystery coupon.
    give(made.id, crate::salon::MYSTERY_HAIR_COUPON);
    let out = s.handle(&npc_click(1000));
    let text = text_of(&out[0].body);
    assert!(text.contains("#L0##i5150100#") && text.contains("#L1##i5150000#"), "{text}");
    let out = s.on_script_reply(&menu_reply(Some(1)));
    let hair = hair_now(&store);
    assert!(crate::salon::HENESYS_VIP_FEMALE.contains(&(hair - 3)) && hair % 10 == 3, "a VIP female base in blonde, got {hair}");
    assert!(out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED && r.what.contains(&format!("HAIR bit -> {hair}"))), "{:?}", names(&out));
    assert_eq!(s.held_count(made.id, crate::salon::MYSTERY_HAIR_COUPON), 0, "spent");
    assert_eq!(s.held_count(made.id, crate::salon::SIGNATURE_HAIR_COUPON), 1, "the other one kept");
    assert!(s.conversation.is_none());

    // 6. Brittany: no colour coupon is a Say linking both colour coupons (the style coupon
    //    they hold counts for nothing here); the Signature Color coupon opens their current
    //    style in its eight colours; index 7 is brown, the style kept.
    let out = s.handle(&npc_click(1001));
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_SAY, "{:?}", names(&out));
    let text = text_of(&out[0].body);
    assert!(text.contains("#i5151100#") && text.contains("#i5151000#") && text.contains("Cash Shop"), "{text}");
    store.set_character_look(made.id, Some(31_233), None).unwrap();
    give(made.id, crate::salon::SIGNATURE_COLOR_COUPON);
    let out = s.handle(&npc_click(1001));
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_MENU, "{:?}", names(&out));
    assert!(text_of(&out[0].body).contains("#L0##i5151100#"));
    let out = s.on_script_reply(&menu_reply(Some(0)));
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_AVATAR, "{:?}", names(&out));
    assert_eq!(styles_in(&out[0].body), (0..8).map(|c| 31_230 + c).collect::<Vec<u32>>());
    let out = s.handle(&avatar_pick(7));
    assert_eq!(hair_now(&store), 31_237, "Rose, brown");
    assert!(out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED && r.what.contains("HAIR bit -> 31237")), "{:?}", names(&out));
    assert_eq!(s.held_count(made.id, crate::salon::SIGNATURE_COLOR_COUPON), 0, "spent");
    let notice = out.iter().find(|r| r.opcode == net::notice::CHAT_NOTICE).expect("a line");
    assert!(text_of(&notice.body).contains("Brown"), "{}", text_of(&notice.body));

    // 7. The Mystery Color coupon rolls one of the eight of the same style.
    give(made.id, crate::salon::MYSTERY_COLOR_COUPON);
    let _ = s.handle(&npc_click(1001));
    let _ = s.on_script_reply(&menu_reply(Some(1)));
    let hair = hair_now(&store);
    assert_eq!(hair / 10, 3_123, "still Rose, got {hair}");
    assert_eq!(s.held_count(made.id, crate::salon::MYSTERY_COLOR_COUPON), 0, "spent");

    // 8. A male at Natalie gets the male REG pool.
    let boy = net::opcode::Character { name: "Barber".to_string(), gender: 0, hair: 30_000, ..Default::default() };
    let him = store.create_character(account_id, 0, &boy).unwrap();
    store.set_character_map(him.id, crate::salon::HENESYS_SALON_MAP).unwrap();
    give(him.id, crate::salon::SIGNATURE_HAIR_COUPON);
    store.create_migration(account_id, him.id, 0, 0).unwrap();
    let mut t = Session::new(store.clone(), s.config.clone());
    t.claim_for_character(him.id);
    let _ = t.handle(&npc_click(1000));
    let out = t.on_script_reply(&menu_reply(Some(0)));
    assert_eq!(styles_in(&out[0].body), crate::salon::HENESYS_REG_MALE);

    // 9. Kerning City: Don Giovanni offers the Kerning REG list and rolls from the Kerning
    //    VIP list; Natalie standing there is not a salon NPC.
    store.set_character_look(made.id, Some(31_233), None).unwrap();
    store.set_character_map(made.id, crate::salon::KERNING_SALON_MAP).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let mut k = Session::new(store.clone(), s.config.clone());
    k.claim_for_character(made.id);
    let _ = k.handle(&npc_click(1000));
    let out = k.on_script_reply(&menu_reply(Some(0)));
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_AVATAR, "{:?}", names(&out));
    assert_eq!(styles_in(&out[0].body), crate::salon::KERNING_REG_FEMALE);
    let _ = k.handle(&avatar_cancel());
    give(made.id, crate::salon::MYSTERY_HAIR_COUPON);
    let _ = k.handle(&npc_click(1000));
    let _ = k.on_script_reply(&menu_reply(Some(1)));
    let hair = hair_now(&store);
    assert!(crate::salon::KERNING_VIP_FEMALE.contains(&(hair - 3)) && hair % 10 == 3, "a Kerning VIP female base in blonde, got {hair}");
    assert_eq!(crate::salon::desk_for(crate::salon::NATALIE, crate::salon::KERNING_SALON_MAP), None);
    let out = k.handle(&npc_click(1002));
    assert!(out.iter().all(|r| !r.what.contains("MENU") && !r.what.contains("AVATAR")), "{:?}", names(&out));

    assert!(s.config.face_ids.is_empty(), "the salon test runs without a face table");
}

/// **The plastic surgeries: the owner's face coupons and the assistant's skin coupon.**
///
/// The owner, 2026-09-18: *"The Plastic Surgeon Owners at Henesys and Kerning will handle the REG
/// (Signature) or VIP (Mystery) face coupons. The assistants will take the Skin Signature or
/// Skin Mystery coupons."* This client has surgeries in Henesys and Orbis (Kerning has none)
/// and one skin coupon, 5153000. A female with eye colour 4 (21005 + 400) at Denma: no coupon
/// is a Say linking both face coupons; the Signature Face coupon's line opens the six REG
/// female faces; index 2 is Look of Death 21009 and they end up in 21409 - the eye colour
/// kept - the coupon gone, one 0x007C with the FACE bit; the Mystery line rolls a VIP female
/// face. At Dr. Feeble the skin coupon opens the seven skins as plain numbers (the client
/// reads ids under 24000 as skins) and index 6 is Pink, skin 6, one 0x007C with the SKIN bit;
/// their no-coupon line names only 5153000. Franz and Riza in Orbis do the same; Denma in
/// Orbis is nobody.
#[test]
fn the_surgery_owner_does_faces_and_the_assistant_does_skins() {
    let mut npcs = std::collections::HashMap::new();
    npcs.insert(
        crate::salon::HENESYS_SURGERY_MAP,
        vec![
            net::opcode::FieldNpc { object_id: 1000, template_id: crate::salon::DENMA, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 },
            net::opcode::FieldNpc { object_id: 1001, template_id: crate::salon::DR_FEEBLE, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 },
        ],
    );
    npcs.insert(
        crate::salon::ORBIS_SURGERY_MAP,
        vec![
            net::opcode::FieldNpc { object_id: 1000, template_id: crate::salon::FRANZ, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 },
            net::opcode::FieldNpc { object_id: 1001, template_id: crate::salon::RIZA, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 },
            net::opcode::FieldNpc { object_id: 1002, template_id: crate::salon::DENMA, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 },
        ],
    );
    let mut face_ids = std::collections::HashSet::new();
    for base in crate::salon::FACE_REG_FEMALE.iter().chain(crate::salon::FACE_VIP_FEMALE).chain(crate::salon::FACE_REG_MALE) {
        for c in 0..9 {
            face_ids.insert(base + c * 100);
        }
    }
    let mut item_names = std::collections::HashMap::new();
    item_names.insert(21_009u32, "Look of Death".to_string());
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Patient".to_string(), gender: 1, face: 21_405, skin: 0, ..Default::default() };
    let made = store.create_character(account_id, 0, &chr).unwrap();
    store.set_character_map(made.id, crate::salon::HENESYS_SURGERY_MAP).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let mut s = Session::new(store.clone(), Arc::new(Config { npcs, face_ids, item_names, ..Config::default() }));
    s.claim_for_character(made.id);
    let names = |out: &[Reply]| out.iter().map(|r| r.what.clone()).collect::<Vec<_>>();
    let look_now = |store: &Arc<Store>| {
        let c = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == made.id).unwrap();
        (c.face, c.skin)
    };
    let ids_in = |body: &[u8]| -> Vec<u32> {
        let b = &body[net::script::SCRIPT_HEAD_LEN..];
        let text_len = u16::from_le_bytes([b[4], b[5]]) as usize;
        let count_at = 4 + 2 + text_len + 1;
        (0..b[count_at] as usize).map(|k| u32::from_le_bytes(b[count_at + 1 + k * 4..count_at + 5 + k * 4].try_into().unwrap())).collect()
    };
    let text_of = |body: &[u8]| String::from_utf8_lossy(body).to_string();
    let avatar_pick = |index: u8| {
        let mut reply = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
        reply.extend_from_slice(&0u32.to_le_bytes());
        reply.extend_from_slice(&[net::script::SCRIPT_TYPE_AVATAR, 1, 0, 0]);
        reply.extend_from_slice(&0u32.to_le_bytes());
        reply.push(index);
        reply
    };
    let give = |who: u32, coupon: u32| store.add_item(who, store::InventoryType::Cash, &store::Item::bundle(coupon, 1), 1).unwrap();

    // 1. Denma, no coupon: both face coupons linked, the Cash Shop named.
    let out = s.handle(&npc_click(1000));
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_SAY, "{:?}", names(&out));
    let text = text_of(&out[0].body);
    assert!(text.contains("#i5152200#") && text.contains("#i5152000#") && text.contains("Cash Shop"), "{text}");

    // 2. The Signature Face coupon: menu, then the six REG female faces; index 2 is Look of
    //    Death 21009, worn at their eye colour 4 as 21409.
    give(made.id, crate::salon::SIGNATURE_FACE_COUPON);
    let out = s.handle(&npc_click(1000));
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_MENU, "{:?}", names(&out));
    assert!(text_of(&out[0].body).contains("#L0##i5152200#"));
    let out = s.on_script_reply(&menu_reply(Some(0)));
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_AVATAR, "{:?}", names(&out));
    assert_eq!(ids_in(&out[0].body), crate::salon::FACE_REG_FEMALE);
    let out = s.handle(&avatar_pick(2));
    assert_eq!(look_now(&store), (21_409, 0), "Look of Death at eye colour 4");
    let stat: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::stats::STAT_CHANGED).collect();
    assert_eq!(stat.len(), 1, "{:?}", names(&out));
    assert!(stat[0].what.contains("FACE bit -> 21409"), "{}", stat[0].what);
    assert!(out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "{:?}", names(&out));
    assert_eq!(s.held_count(made.id, crate::salon::SIGNATURE_FACE_COUPON), 0, "spent");
    let notice = out.iter().find(|r| r.opcode == net::notice::CHAT_NOTICE).expect("a line");
    assert!(text_of(&notice.body).contains("Look of Death"), "{}", text_of(&notice.body));

    // 3. The Mystery Face coupon rolls a VIP female face at once, eye colour kept.
    give(made.id, crate::salon::MYSTERY_FACE_COUPON);
    let _ = s.handle(&npc_click(1000));
    let out = s.on_script_reply(&menu_reply(Some(1)));
    let (face, _) = look_now(&store);
    assert!(crate::salon::FACE_VIP_FEMALE.contains(&(face - 400)) && (face / 100) % 10 == 4, "a VIP female face at colour 4, got {face}");
    assert!(out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED && r.what.contains(&format!("FACE bit -> {face}"))), "{:?}", names(&out));
    assert_eq!(s.held_count(made.id, crate::salon::MYSTERY_FACE_COUPON), 0, "spent");

    // 4. Dr. Feeble: no skin coupon names 5153000 only; with one, the seven skins as plain
    //    numbers; index 6 is Pink, one 0x007C with the SKIN bit.
    let out = s.handle(&npc_click(1001));
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_SAY, "{:?}", names(&out));
    let text = text_of(&out[0].body);
    assert!(text.contains("#i5153000#") && text.contains("Cash Shop"), "{text}");
    give(made.id, crate::salon::SIGNATURE_SKIN_COUPON);
    let out = s.handle(&npc_click(1001));
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_MENU, "{:?}", names(&out));
    let text = text_of(&out[0].body);
    assert!(text.contains("#L0##i5153000#") && !text.contains("#L1#"), "{text}");
    let out = s.on_script_reply(&menu_reply(Some(0)));
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_AVATAR, "{:?}", names(&out));
    assert_eq!(ids_in(&out[0].body), vec![0, 1, 2, 3, 4, 5, 6]);
    let out = s.handle(&avatar_pick(6));
    assert_eq!(look_now(&store), (face, 6), "Pink, the face untouched");
    let stat: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::stats::STAT_CHANGED).collect();
    assert_eq!(stat.len(), 1, "{:?}", names(&out));
    assert!(stat[0].what.contains("SKIN bit -> 6"), "{}", stat[0].what);
    assert_eq!(stat[0].body[7..12], [6, 0, 0, 0, 0], "u8 skin 6, u32 0");
    assert_eq!(s.held_count(made.id, crate::salon::SIGNATURE_SKIN_COUPON), 0, "spent");
    let notice = out.iter().find(|r| r.opcode == net::notice::CHAT_NOTICE).expect("a line");
    assert!(text_of(&notice.body).contains("Pink"), "{}", text_of(&notice.body));
    // A Mystery line was never offered, so answering it spends nothing.
    give(made.id, crate::salon::SIGNATURE_SKIN_COUPON);
    let _ = s.handle(&npc_click(1001));
    let out = s.on_script_reply(&menu_reply(Some(1)));
    assert!(out.is_empty(), "{:?}", names(&out));
    assert_eq!(s.held_count(made.id, crate::salon::SIGNATURE_SKIN_COUPON), 1, "kept");

    // 5. Orbis: Franz offers the same REG faces (male pool for a male), Riza the same skins;
    //    Denma standing there is not a surgeon.
    let boy = net::opcode::Character { name: "Orbisman".to_string(), gender: 0, face: 20_000, ..Default::default() };
    let him = store.create_character(account_id, 0, &boy).unwrap();
    store.set_character_map(him.id, crate::salon::ORBIS_SURGERY_MAP).unwrap();
    give(him.id, crate::salon::SIGNATURE_FACE_COUPON);
    give(him.id, crate::salon::SIGNATURE_SKIN_COUPON);
    store.create_migration(account_id, him.id, 0, 0).unwrap();
    let mut t = Session::new(store.clone(), s.config.clone());
    t.claim_for_character(him.id);
    let _ = t.handle(&npc_click(1000));
    let out = t.on_script_reply(&menu_reply(Some(0)));
    assert_eq!(ids_in(&out[0].body), crate::salon::FACE_REG_MALE, "{:?}", names(&out));
    let _ = t.handle(&avatar_pick(0));
    let _ = t.handle(&npc_click(1001));
    let out = t.on_script_reply(&menu_reply(Some(0)));
    assert_eq!(ids_in(&out[0].body), vec![0, 1, 2, 3, 4, 5, 6], "{:?}", names(&out));
    assert_eq!(crate::salon::desk_for(crate::salon::DENMA, crate::salon::ORBIS_SURGERY_MAP), None);
    let out = t.handle(&npc_click(1002));
    assert!(out.iter().all(|r| !r.what.contains("MENU") && !r.what.contains("AVATAR")), "{:?}", names(&out));
}

#[test]
fn shanks_waiver_is_its_own_box_and_the_boat_sails_on_dismissal() {
    let mut npcs = std::collections::HashMap::new();
    npcs.insert(
        crate::shanks::HOME_MAP,
        vec![net::opcode::FieldNpc { object_id: 1000, template_id: crate::shanks::TEMPLATE, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0 }],
    );
    let fields = [crate::shanks::HOME_MAP, crate::shanks::DESTINATION_MAP].into_iter().collect();

    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Sailor".to_string(), ..Default::default() };
    let made = store.create_character(account_id, 0, &chr).unwrap();
    store.set_character_map(made.id, crate::shanks::HOME_MAP).unwrap();
    store.add_mesos(made.id, 5_000).unwrap();
    store.start_quest(made.id, crate::shanks::MAIS_FINAL_TRAINING).unwrap();
    store.complete_quest(made.id, crate::shanks::MAIS_FINAL_TRAINING).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let mut s = Session::new(store.clone(), Arc::new(Config { npcs, fields, ..Config::default() }));
    s.claim_for_character(made.id);
    let row = |store: &Arc<Store>| store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == made.id).unwrap();
    let names = |out: &[Reply]| out.iter().map(|r| r.what.clone()).collect::<Vec<_>>();

    // ---- click: the yes/no that quotes the fare ------------------------------------------
    let out = s.handle(&npc_click(1000));
    assert_eq!(out.len(), 1, "{:?}", names(&out));
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_YES_NO);
    assert_eq!(s.conversation.as_ref().unwrap().path, crate::shanks::ASK_PATH);

    // ---- Yes: the waiver box ALONE - no SetField in this batch ---------------------------
    let out = s.on_script_reply(&script_reply(net::script::SCRIPT_ACTION_YES));
    assert_eq!(out.len(), 1, "the box and nothing else: {:?}", names(&out));
    assert_eq!(out[0].opcode, net::script::SCRIPT_MESSAGE);
    assert_eq!(out[0].body[10], net::script::SCRIPT_TYPE_SAY);
    assert!(out[0].what.contains("FREE"), "{}", out[0].what);
    assert_eq!(row(&store).map_id, crate::shanks::HOME_MAP, "nobody moved yet");
    assert_eq!(store.mesos(made.id).unwrap(), 5_000);
    let convo = s.conversation.clone().expect("parked for the dismissal");
    assert_eq!(convo.path, crate::shanks::ANNOUNCE_PATH);
    assert!(!convo.awaiting_yes_no);

    // ---- dismiss (Esc, the harder case): the SetField, no script, no fare line -----------
    let out = s.on_script_reply(&script_reply(net::script::SCRIPT_ACTION_CLOSED));
    assert!(out.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "{:?}", names(&out));
    assert!(!out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE), "no script beside a SetField: {:?}", names(&out));
    assert!(!out.iter().any(|r| r.opcode == net::message::MESSAGE), "no 'lost mesos (-0)' line: {:?}", names(&out));
    assert_eq!(row(&store).map_id, crate::shanks::DESTINATION_MAP);
    assert_eq!(store.mesos(made.id).unwrap(), 5_000, "free");
    assert!(s.conversation.is_none());

    // ---- the same player as a Swordsman: no box, pays, sails on the Yes -----------------
    let mut adv = row(&store);
    adv.job = 100;
    store.save_character_progress(&adv).unwrap();
    store.set_character_map(made.id, crate::shanks::HOME_MAP).unwrap();
    s.handle(&npc_click(1000));
    let out = s.on_script_reply(&script_reply(net::script::SCRIPT_ACTION_YES));
    assert!(!out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE), "no waiver box for a Swordsman: {:?}", names(&out));
    assert!(out.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "{:?}", names(&out));
    assert!(out.iter().any(|r| r.opcode == net::message::MESSAGE && r.what.contains("1000")), "the grey fare line: {:?}", names(&out));
    assert_eq!(store.mesos(made.id).unwrap(), 4_000, "a first-job character pays");
    assert_eq!(row(&store).map_id, crate::shanks::DESTINATION_MAP);
}

/// **Without the proof the instructor refuses, and refusing changes nothing.**
///
/// The mirror of the walk above, and the reason it is a separate test: a chain that can be
/// skipped is not a chain. `secondjob::REQUIRE_PROOF_ITEM` is the gate and this is the check
/// that its answer is actually used - `CLAUDE.md`'s Heena section is exactly this shape.
#[test]
fn a_level_thirty_character_cannot_skip_the_test() {
    let b = &crate::secondjob::BRANCHES[0];
    let mut npcs = std::collections::HashMap::new();
    npcs.insert(
        net::opcode::START_MAP_ID,
        vec![net::opcode::FieldNpc {
            object_id: 1000, template_id: b.instructor_npc, x: 0, cy: 0, fh: 1,
            rx0: 0, rx1: 0, f: 0,
        }],
    );
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Impatient".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.level = 60; // well past the minimum: it is the proof that is missing, not the level
    made.job = b.from_job;
    store.save_character_progress(&made).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let config = Config { npcs, ..Config::default() };
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(made.id);

    let out = s.handle(&npc_click(1000));
    let said = out
        .iter()
        .find(|r| r.opcode == net::script::SCRIPT_MESSAGE)
        .expect("ALWAYS ANSWER - a silent click is the frozen-UI failure");
    assert!(
        said.what.contains("Say") || !said.what.contains("MENU"),
        "and it is a sentence, not the choice box: {}",
        said.what
    );
    assert!(
        !out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
        "a refusal must not send a job packet"
    );
    let job = store.characters_for(1, 0).unwrap().into_iter()
        .find(|c| c.id == made.id).unwrap().job;
    assert_eq!(job, b.from_job, "and must not change the row");

    // And a reply arriving anyway - the socket carries no credentials, so it can - is refused
    // by the same gate rather than granted on the strength of the packet.
    let out = s.on_script_reply(&menu_reply(Some(0)));
    let job = store.characters_for(1, 0).unwrap().into_iter()
        .find(|c| c.id == made.id).unwrap().job;
    assert_eq!(job, b.from_job, "an unsolicited menu answer grants nothing: {out:?}");
}

/// **The Dark Marble drops in its own field and nowhere else.**
///
/// Three of the eight test mobs also spawn on ordinary maps, so the map half of the rule is
/// not decoration - `800011` shares *Precipice of Darkness* with the mobs an ordinary player
/// grinds at that level.
#[test]
fn a_dark_marble_drops_in_the_test_field_and_nowhere_else() {
    let b = &crate::secondjob::BRANCHES[2]; // Bowman: the branch with the real leak
    let (mut s, _, _) = gm_session();
    s.last_position = Some((520, 395));

    // In its own field: the marble is there, once, without any drop table saying so.
    let out = s.drops_from_kill(b.test_field.mobs[1], 2000, None, 204, crate::fields::FieldKey::world(b.test_field.map_id));
    assert_eq!(out.len(), 1, "exactly the marble, and exactly one of it: {out:?}");
    assert!(
        out[0].what.contains(&b.chain.marble_item.to_string()),
        "and it is THIS branch's marble: {}",
        out[0].what
    );

    // The same mob on Precipice of Darkness, which is where it also lives. **[L]**
    let out = s.drops_from_kill(b.test_field.mobs[1], 2001, None, 204, crate::fields::FieldKey::world(10006160));
    assert!(out.is_empty(), "an ordinary field must not pay a test's proof: {out:?}");

    // And a scraped 6% row cannot smuggle one out either: the filter runs on the ROLL, so a
    // hit on the wrong map is removed however it got there.
    let drops = crate::droptables::DropTables::parse(&format!(
        "{} | {} | 100 | 1 | 1 | 1 | Dark Marble at a certain rate\n",
        b.test_field.mobs[1], b.chain.marble_item
    ));
    s.config = Arc::new(Config { drops, ..(*s.config).clone() });
    let out = s.drops_from_kill(b.test_field.mobs[1], 2002, None, 204, crate::fields::FieldKey::world(10006160));
    assert!(out.is_empty(), "a 100% table row is still refused off the field: {out:?}");
    // On the field, the table row and the guarantee do not stack.
    let out = s.drops_from_kill(b.test_field.mobs[1], 2003, None, 204, crate::fields::FieldKey::world(b.test_field.map_id));
    assert_eq!(out.len(), 1, "one marble, not two: {out:?}");
}

/// **The third advancement, on the click, with every refusal beside it.**
///
/// The owner, 2026-08-31, choosing between the options this client leaves open: *"No test - level
/// 70 and click."* It ships no third-job quest, no hidden field, no test mobs and no test
/// items - `research/third-job.md` §3 enumerates all four absences - so level and job is not a
/// shortcut past a chain, it is the only gate there was ever going to be.
///
/// Four effects are asserted, because this is the shape the Heena quest got wrong: the
/// database row, the `0x007C`, the sentence, and that **a refusal changes nothing**.
#[test]
fn clicking_a_third_job_instructor_advances_at_seventy() {
    // Tylus, template 1104, in Chief's Residence. They serve the three Warrior second jobs.
    const TYLUS: u32 = 1104;
    let build = |level: u32, job: u16| {
        let mut npcs = std::collections::HashMap::new();
        npcs.insert(
            net::opcode::START_MAP_ID,
            vec![net::opcode::FieldNpc {
                object_id: 1000, template_id: TYLUS, x: 0, cy: 0, fh: 1,
                rx0: 0, rx1: 0, f: 0,
            }],
        );
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Veteran".to_string(), ..Default::default() };
        let mut made = store.create_character(account_id, 0, &chr).unwrap();
        made.level = level;
        made.job = job;
        store.save_character_progress(&made).unwrap();
        store.create_migration(account_id, made.id, 0, 0).unwrap();
        let config = Config { npcs, ..Config::default() };
        let mut s = Session::new(store.clone(), Arc::new(config));
        s.claim_for_character(made.id);
        (s, store, made.id)
    };
    let job_of = |store: &Arc<Store>, id: u32| {
        store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().job
    };

    // ---- a level-70 Fighter becomes a Crusader, and only a Crusader --------------------
    let (mut s, store, id) = build(crate::thirdjob::LEVEL_MINIMUM, 110);
    let out = s.handle(&npc_click(1000));
    assert_eq!(job_of(&store, id), 111, "the job must PERSIST, not just be announced");
    let stat = out
        .iter()
        .find(|r| r.opcode == net::stats::STAT_CHANGED)
        .expect("the client must be told, or it draws the old job forever");
    assert!(stat.what.contains("job 110 -> 111"), "{}", stat.what);
    // **The third pool must be in the packet.** A job change with no tier-3 row greys the
    // `+` button with nothing on screen to say why - the failure `skillpoints` was written
    // to prevent, one tier up.
    assert!(stat.what.contains("tier 3"), "the third SP pool is missing: {}", stat.what);
    assert!(
        out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE),
        "and a sentence, or nothing on screen says what happened"
    );
    // No box in between: there is exactly one destination, so nothing was asked.
    assert!(
        !out.iter().any(|r| r.what.contains("MENU")),
        "one destination means no menu: {out:?}"
    );

    // ---- one level short: a sentence, and NOTHING else ---------------------------------
    let (mut s, store, id) = build(crate::thirdjob::LEVEL_MINIMUM - 1, 110);
    let out = s.handle(&npc_click(1000));
    assert_eq!(job_of(&store, id), 110, "a refused advancement must change nothing");
    assert!(
        !out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
        "and must not send a job packet"
    );
    let said = out
        .iter()
        .find(|r| r.opcode == net::script::SCRIPT_MESSAGE)
        .expect("but it is still ANSWERED - a silent click is the frozen-UI failure");
    assert!(said.what.contains("70") || said.body.len() > 10, "{}", said.what);

    // ---- the wrong branch, and a first-job character -----------------------------------
    for (level, job) in [(70u32, 210u16), (70, 100), (70, 0), (99, 111)] {
        let (mut s, store, id) = build(level, job);
        let out = s.handle(&npc_click(1000));
        assert_eq!(job_of(&store, id), job, "level {level} job {job} must not change");
        assert!(
            !out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
            "level {level} job {job} must not send a job packet"
        );
        assert!(
            out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE),
            "level {level} job {job} must still be ANSWERED"
        );
    }

    // ---- and every branch pairs with its own instructor --------------------------------
    for m in crate::thirdjob::MASTERS {
        for t in m.serves {
            let mut npcs = std::collections::HashMap::new();
            npcs.insert(
                net::opcode::START_MAP_ID,
                vec![net::opcode::FieldNpc {
                    object_id: 1000, template_id: m.npc, x: 0, cy: 0, fh: 1,
                    rx0: 0, rx1: 0, f: 0,
                }],
            );
            let store = Arc::new(Store::open_in_memory().unwrap());
            let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
            let name = format!("Third{}", t.from_job);
            let chr = net::opcode::Character { name, ..Default::default() };
            let mut made = store.create_character(account_id, 0, &chr).unwrap();
            made.level = 70;
            made.job = t.from_job;
            store.save_character_progress(&made).unwrap();
            store.create_migration(account_id, made.id, 0, 0).unwrap();
            let config = Config { npcs, ..Config::default() };
            let mut s = Session::new(store.clone(), Arc::new(config));
            s.claim_for_character(made.id);
            s.handle(&npc_click(1000));
            assert_eq!(
                job_of(&store, made.id),
                t.job,
                "{} must advance job {} into {}",
                m.name,
                t.from_job,
                t.job
            );
        }
    }
}

/// **Improved MP Recovery finally does what its own tooltip says.**
///
/// The owner, 2026-08-30: *"one of the passive for 'Improved MP Recovery' says it will increase MP
/// recovery item recovery amount by 20%, it currently does not do that."*
///
/// This reproduces the exact case out of the archive rather than a made-up one. Character 213
/// holds `2000000` at level 15 in `maplecw.db`, and `previous-runs/world-20260830-221224.log`
/// - the same day as the report - shows `used item 2000003 ... +200 mp` eleven times. `2000003`
/// is 200 flat MP and `y` at level 15 is **20**, so the 40 that never arrived is what this
/// asserts.
///
/// **The level-15 value is 20, not 19.** `y` runs 5, 6, 7 ... 18 and then jumps: `level + 4`
/// is right for fourteen levels and wrong for the only one `!learn` grants. That is why the
/// module carries a table rather than a formula, and why this test uses the top level.
#[test]
fn improved_mp_recovery_adds_its_percent_to_a_potion() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Wizard".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.job = 200;
    made.max_mp = 5_000;
    made.mp = 0;
    made.max_hp = 5_000;
    made.hp = 5_000;
    store.save_character_progress(&made).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    store
        .set_inventory_slot(
            made.id,
            store::InventoryType::Use,
            1,
            &store::Item::bundle(2_000_003, 10),
        )
        .unwrap();
    // 2000003 is 200 flat MP - the item from the archived log.
    let config = Config {
        consumables: crate::consumables::Consumables::parse("2000003, 0, 200, 0, 0\n"),
        ..Config::default()
    };
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(made.id);

    let mp_now = |s: &Session| s.claimed_character().unwrap().mp;

    // ---- THE CONTROL: no skill, so the plain 200 -------------------------------------
    s.on_use_item(&net::useitem::use_item(1, 1, 2_000_003, 1));
    assert_eq!(mp_now(&s), 200, "without the skill it is the item's own number");

    // ---- level 15: 200 + 20% = 240 ---------------------------------------------------
    store.set_skill_level(made.id, crate::itemrecovery::IMPROVED_MP_RECOVERY, 15).unwrap();
    s.claim_for_character(made.id);
    let before = mp_now(&s);
    let out = s.on_use_item(&net::useitem::use_item(2, 1, 2_000_003, 1));
    assert_eq!(
        mp_now(&s) - before,
        240,
        "level 15 is +20%, so 200 becomes 240 - this is the 40 the owner never got"
    );

    // The run has to be able to SEE that the bonus fired, or a capture cannot tell
    // "applied 0%" from "not wired at all".
    let stat = out
        .iter()
        .find(|r| r.opcode == net::stats::STAT_CHANGED)
        .expect("a potion sends a StatChanged");
    assert!(stat.what.contains("Improved Recovery"), "{}", stat.what);
    assert!(stat.what.contains("+20%"), "and it names the percent: {}", stat.what);

    // ---- level 1 is +5%, so the table is being read rather than a constant applied ----
    store.set_skill_level(made.id, crate::itemrecovery::IMPROVED_MP_RECOVERY, 1).unwrap();
    s.claim_for_character(made.id);
    let before = mp_now(&s);
    s.on_use_item(&net::useitem::use_item(3, 1, 2_000_003, 1));
    assert_eq!(mp_now(&s) - before, 210, "level 1 is +5%");

    // ---- and the HP twin must NOT touch MP -------------------------------------------
    store.set_skill_level(made.id, crate::itemrecovery::IMPROVED_MP_RECOVERY, 0).unwrap();
    store.set_skill_level(made.id, crate::itemrecovery::IMPROVED_HP_RECOVERY, 15).unwrap();
    s.claim_for_character(made.id);
    let before = mp_now(&s);
    s.on_use_item(&net::useitem::use_item(4, 1, 2_000_003, 1));
    assert_eq!(
        mp_now(&s) - before,
        200,
        "Improved HP Recovery bonuses HP, and an MP potion is not its business"
    );
}

// =========================================================================================
// One mob, one simulation, two screens - `crate::mobshare`, wired 2026-09-01
//
// The owner: *"All clients need to see other clients damages to mobs, but the drops can remain per
// client. If multiple clients hit the mob, the one who dealt the most damage (without counting
// over-damage) will see the drops."*
//
// **Nothing below has ever been on a wire between two players.** Two clients have never been
// connected to this server at once, so each of these proves the server does what it was told
// to do and none of them proves the client likes it.
// =========================================================================================

/// An opcode nothing dispatches, so `handle` returns the mailbox and nothing else.
const NO_PACKET: [u8; 2] = [0xFF, 0xFE];

/// The map these tests share.
const SHARED_MAP: u32 = 104_040_000;

/// A channel whose one map has `points` spawn points, all of them already alive.
///
/// The mobs are brought up **before** anybody walks in, because that is the case the claim
/// split is about: a field that already has monsters on it when a second player arrives.
fn shared_channel(
    points: u32,
    hp: u64,
) -> (Arc<Store>, Arc<Config>, Arc<crate::fields::Fields>, i64) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account = store.create_account("maplecw", "correct horse battery").unwrap();
    let mut mobs = std::collections::HashMap::new();
    mobs.insert(
        SHARED_MAP,
        (0..points)
            .map(|i| net::mob::FieldMob::new(2000 + i, 2, 100 + 100 * i as i16, 395, 1, hp))
            .collect::<Vec<_>>(),
    );
    let config =
        Arc::new(Config { send_mobs: true, mobs, ..Config::default() });
    let fields = Arc::new(crate::fields::Fields::new());
    fields.seed(crate::fields::FieldKey::world(SHARED_MAP), &config, 0);
    fields.due_respawns(crate::fields::FieldKey::world(SHARED_MAP), &config, 999_999);
    (store, config, fields, account)
}

/// Another connection on the same channel, playing a new character on [`SHARED_MAP`].
fn join_channel(
    store: &Arc<Store>,
    config: &Arc<Config>,
    fields: &Arc<crate::fields::Fields>,
    account: i64,
    name: &str,
) -> (Session, u32) {
    let chr =
        net::opcode::Character { name: name.to_string(), map_id: SHARED_MAP, ..Default::default() };
    let id = store.create_character(account, 0, &chr).unwrap().id;
    store.create_migration(account, id, 0, 0).unwrap();
    let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
    s.claim_for_character(id);
    (s, id)
}

fn count_of(out: &[Reply], opcode: u16) -> usize {
    out.iter().filter(|r| r.opcode == opcode).count()
}

fn unhex_body(s: &str) -> Vec<u8> {
    (0..s.len() / 2)
        .map(|i| u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).expect("fixture hex"))
        .collect()
}

/// **The bug this whole module exists to remove.** `on_field_entered` used to push a
/// `MOB_CHANGE_CONTROLLER` for every mob to every arriving session, so two players on one map
/// were two clients each rolling their own wander for the same monster - the client builds the
/// path out of its own random source, one call per element (`research/mob-behaviour.md` §5.1),
/// so the two screens diverge on the first step and never reconverge.
///
/// **Both halves are asserted**, because a claim that returned nothing to anybody would pass
/// the second half on its own - and that is a different bug: monsters that never move at all.
#[test]
fn only_the_first_arrival_is_granted_control_and_the_second_is_a_spectator() {
    let (store, config, fields, account) = shared_channel(4, 30);
    let alive = fields.mob_count(crate::fields::FieldKey::world(SHARED_MAP));
    assert!(alive >= 2, "the fixture needs several mobs, not {alive}");

    let (mut first, _) = join_channel(&store, &config, &fields, account, "Wanderer");
    let (mut second, _) = join_channel(&store, &config, &fields, account, "Stranger");

    let a = first.on_field_entered();
    assert_eq!(count_of(&a, net::mob::MOB_ENTER_FIELD), alive, "every mob is drawn for the first");
    assert_eq!(
        count_of(&a, net::mobmove::MOB_CHANGE_CONTROLLER),
        alive,
        "and an empty map's mobs are all claimed by whoever arrives first"
    );

    let b = second.on_field_entered();
    assert_eq!(
        count_of(&b, net::mob::MOB_ENTER_FIELD),
        alive,
        "the second player must SEE every mob - only the simulation is exclusive"
    );
    assert_eq!(
        count_of(&b, net::mobmove::MOB_CHANGE_CONTROLLER),
        0,
        "and must be granted NONE of them: two grants are two independent wanders"
    );

    assert_eq!(fields.controllers().held_by(first.subscriber.get()), alive);
    assert_eq!(fields.controllers().held_by(second.subscriber.get()), 0);
    assert_eq!(fields.controllers().len(), alive, "one controller per mob, not two");
}

/// **A respawn used to be unicast.** `Fields::due_respawns` drains `field.pending`, so
/// whichever session ticked first took the new mobs and the other player was never told they
/// exist - and no later packet would have healed it.
///
/// Four effects, not one: the ticking session gets the spawn *and* the grant; the other
/// session gets the spawn *and not* the grant.
#[test]
fn a_respawned_mob_reaches_the_other_player_but_its_grant_does_not() {
    let (store, config, fields, account) = shared_channel(1, 30);
    // Kill the one mob so the field's next wave refills it while both players stand there.
    // `shared_channel` ran the field to 999 999, so that wave is at 1 000 000 (`crate::fields`).
    let wave = 1_000_000;
    let victim = fields.mobs_on(crate::fields::FieldKey::world(SHARED_MAP))[0].spawn.object_id;
    fields.hurt(crate::fields::FieldKey::world(SHARED_MAP), victim, 999, 200, &config, 0);
    assert_eq!(fields.mob_count(crate::fields::FieldKey::world(SHARED_MAP)), 0);

    let (mut ticker, _) = join_channel(&store, &config, &fields, account, "Ticker");
    let (mut watcher, _) = join_channel(&store, &config, &fields, account, "Watcher");
    ticker.on_field_entered();
    watcher.on_field_entered();
    let _ = ticker.tick(wave - 1);
    let _ = watcher.tick(wave - 1);

    let spawned = ticker.tick(wave);
    assert_eq!(count_of(&spawned, net::mob::MOB_ENTER_FIELD), 1, "the refill: {spawned:?}");
    assert_eq!(
        count_of(&spawned, net::mobmove::MOB_CHANGE_CONTROLLER),
        1,
        "claimed by whichever session ticked first"
    );

    let seen = watcher.tick(wave + 1_000);
    assert_eq!(
        count_of(&seen, net::mob::MOB_ENTER_FIELD),
        1,
        "the other player has to learn the mob exists: {seen:?}"
    );
    assert_eq!(
        count_of(&seen, net::mobmove::MOB_CHANGE_CONTROLLER),
        0,
        "but must not be granted it as well - claim_one is a test-and-set"
    );
    assert_eq!(fields.controllers().held_by(ticker.subscriber.get()), 1);
    assert_eq!(fields.controllers().held_by(watcher.subscriber.get()), 0);
}

/// Object 2000, four path elements, 174 bytes - a **real captured `0x02FF`**, byte-identical
/// to `net::mobmove::tests::CAPTURED_FOUR_ELEMENTS`, and the mob a second decoder
/// independently placed at `(424, 395)`.
const CAPTURED_MOB_MOVE_2000: &str = "d0070000010000ff0000000000000000000000000000000000010000\
00ccddff00ccddff005087d93c000000000100000000a8018b0100000000040000a6018b01d5ff000023000000000\
00000035a000000a3018b01000000002300000000000000025a000000c2018b012b00000023000000000000000\
2f1020000c8018b012b00000025000000000000000293000000000faa8ebe57f5c299250000000000000000000000\
0300000000010000";

/// **Only the controller may move a mob, and its report reaches the other screen.**
///
/// Two claims in one test, and the second is the positive control for the first: an identical
/// body sent by the controller must be believed, or "the non-controller was refused" would be
/// indistinguishable from "this handler stopped working".
///
/// Five effects, because `CLAUDE.md` says a test that checks one of several gives false
/// confidence about the rest: the refusal, the position that must not move, the `0x03E4` ack,
/// the `0x03D9` rebroadcast, and that the mover never receives its own rebroadcast.
#[test]
fn a_non_controllers_mob_move_is_refused_and_the_controllers_is_rebroadcast() {
    let (store, config, fields, account) = shared_channel(1, 30);
    assert_eq!(fields.mobs_on(crate::fields::FieldKey::world(SHARED_MAP))[0].spawn.object_id, 2000, "the fixture's mob");

    let (mut controller, _) = join_channel(&store, &config, &fields, account, "Controller");
    let (mut spectator, _) = join_channel(&store, &config, &fields, account, "Spectator");
    controller.on_field_entered();
    spectator.on_field_entered();
    let _ = controller.handle(&NO_PACKET);
    let _ = spectator.handle(&NO_PACKET);

    let mut packet = net::mobmove::MOB_MOVE_REQUEST.to_le_bytes().to_vec();
    packet.extend_from_slice(&unhex_body(CAPTURED_MOB_MOVE_2000));

    // (1) The spectator does not own this simulation.
    let refused = spectator.handle(&packet);
    assert!(refused.is_empty(), "no ack for a mob this connection does not control: {refused:?}");
    assert_eq!(
        fields.mob_position(crate::fields::FieldKey::world(SHARED_MAP), 2000),
        None,
        "and the position a drop lands on must not have been written"
    );
    assert!(
        controller.handle(&NO_PACKET).is_empty(),
        "nor may a refused report be rebroadcast to anybody"
    );

    // (2) The controller's identical report is believed - the control.
    let ack = controller.handle(&packet);
    assert_eq!(count_of(&ack, net::mobmove::MOB_CTRL_ACK), 1, "the ack: {ack:?}");
    assert_eq!(
        fields.mob_position(crate::fields::FieldKey::world(SHARED_MAP), 2000),
        Some((456, 395)),
        "the END of the reported path. (424, 395) is its HEAD - where the mob was when the \
         walk began - and storing that is the drop-placement bug crate::dropsite measures. \
         The head is still a real coordinate, so the two independent decoders that agreed on \
         it were not wrong; it just is not where the mob is now"
    );
    assert_eq!(
        count_of(&ack, net::mobmove::MOB_MOVE),
        0,
        "the mover must never be sent 0x03D9 - it overwrites the state the controller owns"
    );

    // (3) ...and it is what makes the mob walk on the other screen at all.
    let mail = spectator.handle(&NO_PACKET);
    let moves: Vec<&Reply> = mail.iter().filter(|r| r.opcode == net::mobmove::MOB_MOVE).collect();
    assert_eq!(moves.len(), 1, "one rebroadcast: {mail:?}");
    assert_eq!(
        u32::from_le_bytes(moves[0].body[0..4].try_into().unwrap()),
        2000,
        "addressed to the mob that moved"
    );
}

/// `previous-runs/world-20260820-121055.log` 16:10:28.598, `0x00DF`, 229 bytes: a plain swing
/// that connected - mob **2002**, one hit of 19, not critical. The same body `net::attack`,
/// `crate::remoteattack` and `session::multiplayer`'s tests all use.
const MELEE_2002_FOR_19: &str = "0001000000000000000000000000000001050000009fae340801040000003b80680a70028b010000000070028b0100000000000000000000000000000000000000000000000000000100000001000000000a0055736572204d656c65658901000000000000000000000000000000010000000000000000000000d20700000200000001000013000000000000000000000736028b0136028b0135027b01890100000000000001000002000000000001012302710148028b01000000007e6c3c6600000000030000000000d5c057820100000092e9bc2707000000bc6509e5000080e8da8f00";

fn melee_packet() -> Vec<u8> {
    let mut p = net::combat::USER_MELEE_ATTACK.to_le_bytes().to_vec();
    p.extend_from_slice(&unhex_body(MELEE_2002_FOR_19));
    p
}

/// A channel whose single mob is object **2002**, which is what the captured swing targets.
fn channel_with_mob_2002(hp: u64) -> (Arc<Store>, Arc<Config>, Arc<crate::fields::Fields>, i64) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account = store.create_account("maplecw", "correct horse battery").unwrap();
    let mut mobs = std::collections::HashMap::new();
    mobs.insert(SHARED_MAP, vec![net::mob::FieldMob::new(2002, 2, 400, 395, 1, hp)]);
    let config = Arc::new(Config {
        send_mobs: true,
        mobs,
        drops: crate::droptables::DropTables::parse("2 | 4000001 | 100 | 1 | 1 | 9 | Shell\n"),
        ..Config::default()
    });
    let fields = Arc::new(crate::fields::Fields::new());
    fields.seed(crate::fields::FieldKey::world(SHARED_MAP), &config, 0);
    fields.due_respawns(crate::fields::FieldKey::world(SHARED_MAP), &config, 999_999);
    (store, config, fields, account)
}

/// **Hitting a mob takes it, and the old holder is TOLD - in that order.**
///
/// This assertion has been written three ways in one evening and the history is the point:
///
///   1. no handover at all - a non-controller's hits did not flinch the mob;
///   2. handover with no release - the flinch worked and **mobs teleported**, because two
///      clients simulated one mob;
///   3. handover WITH a release, which is this - and it only became possible when the owner
///      pushed back on `CONTROL_RELEASE` being a despawn. It is not: the zero branch releases
///      and a live mob never reaches the erase behind it (`net::mobmove::CONTROL_RELEASE`).
///
/// **Order is the fix, not a detail.** Granting first leaves both clients past their run
/// gate, both rolling independent wanders, both sending `0x02FF` - which is what step 2
/// looked like on screen.
#[test]
fn hitting_a_mob_takes_control_and_releases_the_old_holder() {
    let (store, config, fields, account) = channel_with_mob_2002(500);
    let (mut owner, _) = join_channel(&store, &config, &fields, account, "Owner");
    let (mut other, _) = join_channel(&store, &config, &fields, account, "Other");
    owner.on_field_entered();
    other.on_field_entered();
    let _ = owner.tick(1);

    assert_eq!(
        fields.controllers().controller_of(crate::fields::FieldKey::world(SHARED_MAP), 2002),
        Some(owner.subscriber.get()),
        "the control: the owner walked in first and holds it"
    );

    let out = other.handle(&melee_packet());
    let grants: Vec<&Reply> =
        out.iter().filter(|r| r.opcode == net::mobmove::MOB_CHANGE_CONTROLLER).collect();
    assert_eq!(grants.len(), 1, "the attacker is handed the mob: {out:?}");
    assert_eq!(grants[0].body[0], net::mobmove::CONTROL_NORMAL, "granted, not released");
    assert_eq!(
        fields.controllers().controller_of(crate::fields::FieldKey::world(SHARED_MAP), 2002),
        Some(other.subscriber.get()),
        "one holder at every instant, and it is the attacker"
    );

    // **The old holder is told, and its absence is what made mobs teleport.**
    let told = owner.tick(2);
    let released: Vec<&Reply> = told
        .iter()
        .filter(|r| {
            r.opcode == net::mobmove::MOB_CHANGE_CONTROLLER
                && r.body.first() == Some(&net::mobmove::CONTROL_RELEASE)
        })
        .collect();
    assert_eq!(released.len(), 1, "the previous holder gets a release: {told:?}");
    assert_eq!(
        u32::from_le_bytes(released[0].body[1..5].try_into().unwrap()),
        2002,
        "and it names the mob it lost"
    );
    assert_eq!(released[0].body.len(), 5, "a release is five bytes and carries nothing else");

    // Already ours: no second grant, and nobody is released again.
    let again = other.handle(&melee_packet());
    assert_eq!(count_of(&again, net::mobmove::MOB_CHANGE_CONTROLLER), 0, "{again:?}");
    assert_eq!(count_of(&owner.tick(3), net::mobmove::MOB_CHANGE_CONTROLLER), 0);
}

/// **The owner's first sentence, end to end**: *"All clients need to see other clients damages to
/// mobs."*
///
/// The health bar and the death, on a screen that did not swing - and **byte-identical to what
/// the attacker got**, which is the property that matters most here. `0x03F0`'s hp field is a
/// **percentage**, not an absolute (`net::combat::hp_percent`, `research/mob-hp-bar.md`); the
/// absolute went out once and drew a 45-HP snail at 27%. A second call site that looked
/// `max_hp` up its own way is exactly how that comes back, so this compares the bodies rather
/// than re-deriving the number beside them.
#[test]
fn a_mobs_damage_and_death_reach_the_other_players_screen_unchanged() {
    let (store, config, fields, account) = channel_with_mob_2002(30);
    let (mut attacker, _) = join_channel(&store, &config, &fields, account, "Attacker");
    let (mut watcher, _) = join_channel(&store, &config, &fields, account, "Watcher");
    attacker.on_field_entered();
    watcher.on_field_entered();
    let _ = attacker.handle(&NO_PACKET);
    let _ = watcher.handle(&NO_PACKET);

    // (1) A hit that wounds: 30 - 19 = 11, so a bar update and no death.
    let swing = attacker.handle(&melee_packet());
    let hp: Vec<&Reply> = swing.iter().filter(|r| r.opcode == net::combat::MOB_HP_CHANGE).collect();
    assert_eq!(hp.len(), 1, "the attacker's own bar update: {swing:?}");

    let mail = watcher.handle(&NO_PACKET);
    let seen: Vec<&Reply> =
        mail.iter().filter(|r| r.opcode == net::combat::MOB_HP_CHANGE).collect();
    assert_eq!(seen.len(), 1, "the watcher's bar must move too: {mail:?}");
    assert_eq!(seen[0].body, hp[0].body, "the SAME body, not a second computation of it");
    assert_eq!(
        u32::from_le_bytes(seen[0].body[4..8].try_into().unwrap()),
        net::combat::hp_percent(11, 30),
        "a PERCENTAGE - 11 of 30 - read out of net::combat rather than restated here"
    );

    // (2) A hit that kills: the death, and no bar update for an object being torn down.
    let kill = attacker.handle(&melee_packet());
    let died: Vec<&Reply> =
        kill.iter().filter(|r| r.opcode == net::combat::MOB_LEAVE_FIELD).collect();
    assert_eq!(died.len(), 1, "the attacker sees it die: {kill:?}");

    let mail = watcher.handle(&NO_PACKET);
    let seen: Vec<&Reply> =
        mail.iter().filter(|r| r.opcode == net::combat::MOB_LEAVE_FIELD).collect();
    assert_eq!(seen.len(), 1, "and so does everybody else on the map: {mail:?}");
    assert_eq!(seen[0].body, died[0].body);
    assert_eq!(
        count_of(&mail, net::mobmove::MOB_CHANGE_CONTROLLER),
        0,
        "a grant must never ride along with a broadcast"
    );
    assert_eq!(
        fields.controllers().controller_of(crate::fields::FieldKey::world(SHARED_MAP), 2002),
        None,
        "and the dead mob's entry is forgotten at the death, not left for the next reconcile"
    );
}

/// **The case the owner named.** *"If multiple clients hit the mob, the one who dealt the most
/// damage (without counting over-damage) will see the drops."*
///
/// The helper does 90 of 100; the attacker lands the killing 10. The loot is the helper's, on
/// the helper's own connection, and the killer gets none of it.
///
/// Both directions are asserted. "The killer got no drop" alone would pass against a version
/// that dropped nothing at all, which is the failure that looks identical on screen -
/// `session/combat.rs`'s own sentence about an item nobody can see.
#[test]
fn the_drops_go_to_the_top_damager_and_not_to_whoever_landed_the_last_hit() {
    let (store, config, fields, account) = channel_with_mob_2002(100);
    let (mut killer, killer_id) = join_channel(&store, &config, &fields, account, "Finisher");
    let (mut helper, helper_id) = join_channel(&store, &config, &fields, account, "Helper");
    killer.on_field_entered();
    helper.on_field_entered();
    let _ = killer.handle(&NO_PACKET);
    let _ = helper.handle(&NO_PACKET);

    // 90 of the 100, credited to the helper. `LiveMob::credit` caps at what landed, so this is
    // the same ranking the EXP split and its white/yellow line already pay out on.
    fields.hurt(crate::fields::FieldKey::world(SHARED_MAP), 2002, 90, helper_id, &config, 0);
    assert_eq!(fields.mob_hp(crate::fields::FieldKey::world(SHARED_MAP), 2002), Some(10));

    let kill = killer.handle(&melee_packet());
    assert!(
        kill.iter().any(|r| r.opcode == net::combat::MOB_LEAVE_FIELD),
        "the swing has to actually kill it: {kill:?}"
    );
    assert_eq!(
        count_of(&kill, net::drops::DROP_ENTER_FIELD),
        0,
        "the killer did 10 of 100 and must not see the loot: {kill:?}"
    );

    let mail = helper.handle(&NO_PACKET);
    let loot: Vec<&Reply> =
        mail.iter().filter(|r| r.opcode == net::drops::DROP_ENTER_FIELD).collect();
    assert_eq!(loot.len(), 1, "the top damager sees it, on their own connection: {mail:?}");
    assert_eq!(
        u32::from_le_bytes(loot[0].body[23..27].try_into().unwrap()),
        helper_id,
        "and the packet names them as the owner"
    );

    // The floor agrees with the packet, which is what `LiveDrop::may_be_taken_by` enforces - a
    // drop the helper can see and the killer can take would be the same bug wearing a hat.
    let owners: Vec<u32> =
        fields.with_drops(crate::fields::FieldKey::world(SHARED_MAP), |d| d.on_field(crate::fields::FieldKey::world(SHARED_MAP)).map(|x| x.owner_id).collect());
    assert_eq!(owners, vec![helper_id], "killer {killer_id} owns nothing here");
}

/// **An expired drop fades on the OWNER's screen whichever session happened to tick - and on
/// every other screen on the map.**
///
/// The drop table is shared by every connection on the channel and every one of them sweeps,
/// so once the first to tick removed the drop and took the `0x046F` for itself: the owner went
/// on drawing an item that no longer existed. Then the fade went to the owner alone, and a
/// party member or a bystander who had been shown the drop kept drawing it (the owner, 2026-09-18).
/// Now the fade goes to the drop's whole map: the owner reads it from their mailbox, and so
/// does the sweeper on ITS next tick (`collect_mail` runs before the sweep, so never the same
/// tick). A client that was never shown the id ignores the leave, as it ignores a movement
/// packet for a mob it never held - the pick-up leave has gone to the whole field on that
/// reasoning since 2026-09-14.
#[test]
fn an_expired_drop_fades_for_its_owner_and_for_the_session_that_swept_it() {
    let (store, config, fields, account) = shared_channel(1, 30);
    let (mut sweeper, _) = join_channel(&store, &config, &fields, account, "Sweeper");
    let (mut owner, owner_id) = join_channel(&store, &config, &fields, account, "Owner");
    sweeper.on_field_entered();
    owner.on_field_entered();
    let _ = sweeper.handle(&NO_PACKET);
    let _ = owner.handle(&NO_PACKET);

    let (drop_id, _) = fields.with_drops(crate::fields::FieldKey::world(SHARED_MAP), |d| {
        d.drop_from_mob(crate::drops::DropFromMob {
            from_mob: true,
            map_id: crate::fields::FieldKey::world(SHARED_MAP),
            owner_id,
            item: store::Item::bundle(4_000_001, 1),
            inv_type: store::InventoryType::Etc,
            meso: 0,
            x: 400,
            y: 395,
            source_x: 400,
            source_y: 395,
            now_ms: 0,
            party_id: 0,
        })
    });

    // Well past `DROP_LIFETIME_MS`, and it is the *other* connection that gets there first.
    let swept = sweeper.tick(crate::drops::DROP_LIFETIME_MS + 1_000);
    assert_eq!(
        count_of(&swept, net::drops::DROP_LEAVE_FIELD),
        0,
        "the sweep posts to mailboxes, and this tick's mail was collected before it ran: {swept:?}"
    );
    assert_eq!(fields.with_drops(crate::fields::FieldKey::world(SHARED_MAP), |d| d.len()), 0, "but it IS gone from the floor");
    let next = sweeper.tick(crate::drops::DROP_LIFETIME_MS + 1_500);
    assert_eq!(
        count_of(&next, net::drops::DROP_LEAVE_FIELD),
        1,
        "the sweeper is on the map, so it is told too - its client ignores an id it never held: {next:?}"
    );

    let mail = owner.tick(crate::drops::DROP_LIFETIME_MS + 2_000);
    let fades: Vec<&Reply> =
        mail.iter().filter(|r| r.opcode == net::drops::DROP_LEAVE_FIELD).collect();
    assert_eq!(fades.len(), 1, "the owner's screen is the one holding the icon: {mail:?}");
    assert_eq!(
        u32::from_le_bytes(fades[0].body[0..4].try_into().unwrap()),
        drop_id,
        "and it names the drop that expired"
    );
    assert_eq!(fades[0].body[4], net::drops::leave_type::FADE);
}

/// **Walking in must not show somebody else's loot.** The floor is re-sent on every field
/// entry because the client's drop pool is destroyed and rebuilt empty by each `SetField` -
/// and that re-send used to be unfiltered, which is a leak the moment drops are owner-scoped.
/// The client reads `ownType` into `drop+0x70` and never tests it again (**[L]**), so who is
/// sent the `0x046E` is the only thing that decides who can pick the item up.
#[test]
fn walking_into_a_field_does_not_re_send_another_players_drops() {
    let (store, config, fields, account) = shared_channel(1, 30);
    let (mut owner, owner_id) = join_channel(&store, &config, &fields, account, "Owner");
    let (mut passer_by, _) = join_channel(&store, &config, &fields, account, "PasserBy");
    owner.on_field_entered();
    passer_by.on_field_entered();

    fields.with_drops(crate::fields::FieldKey::world(SHARED_MAP), |d| {
        d.drop_from_mob(crate::drops::DropFromMob {
            from_mob: true,
            map_id: crate::fields::FieldKey::world(SHARED_MAP),
            owner_id,
            item: store::Item::bundle(4_000_001, 1),
            inv_type: store::InventoryType::Etc,
            meso: 0,
            x: 400,
            y: 395,
            source_x: 400,
            source_y: 395,
            now_ms: 0,
            party_id: 0,
        })
    });

    let theirs = passer_by.on_field_entered();
    assert_eq!(
        count_of(&theirs, net::drops::DROP_ENTER_FIELD),
        0,
        "a bystander sees none of it: {theirs:?}"
    );

    // The control, and it is the half that matters: the owner must still get it back, or this
    // would pass against a field entry that had simply stopped re-sending the floor at all.
    let mine = owner.on_field_entered();
    let back: Vec<&Reply> =
        mine.iter().filter(|r| r.opcode == net::drops::DROP_ENTER_FIELD).collect();
    assert_eq!(back.len(), 1, "the owner's own: {mine:?}");
    assert_eq!(back[0].body[1], net::drops::ENTER_INSTANT, "already lying there, no second arc");
}

/// **A hair change reaches the other clients on the map, with no field reload for anyone.**
///
/// The owner, 2026-09-12: *"The moment any hair or face change happens, it should also show up on
/// other clients."* One player uses an Übel Hair Coupon; the other, standing on the same map,
/// gets a fresh `USER_ENTER_FIELD` carrying the new look, and the user who changed gets no
/// `SetField`.
#[test]
fn a_hair_change_is_broadcast_to_the_other_clients_without_a_reload() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let (mut me, my_id) = join_channel(&store, &config, &fields, account, "Stylist");
    me.on_field_entered();
    let (mut them, _) = join_channel(&store, &config, &fields, account, "Bystander");
    them.on_field_entered();
    me.collect_mail(); // drain the bystander's own entry

    let slot = store
        .add_item(my_id, store::InventoryType::Use, &store::Item::bundle(2_543_143, 1), 1)
        .unwrap()[0]
        .slot;
    let mut body = slot.to_le_bytes().to_vec();
    body.extend_from_slice(&2_543_143u32.to_le_bytes());
    let out = me.on_beauty_coupon_confirm(&body);
    assert!(!out.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "no reload for the changer");

    let seen = them.collect_mail();
    // The owner, 2026-09-18: a bare re-sent 0x0224 drew nothing (the pool ignores an id it has),
    // the leave + enter drew it WITH a blink and a pet respawn, and 0x0138 walks the summoned
    // map. So the observer gets ONE 0x02AE naming the changer: the pool decodes the look into
    // its copy and rebuilds the avatar in place.
    let ops: Vec<u16> = seen.iter().map(|r| r.opcode).collect();
    assert_eq!(ops, vec![net::lookupdate::USER_LOOK_UPDATE_REMOTE], "one 0x02AE and nothing else: {ops:x?}");
    let chr = me.claimed_character().unwrap();
    assert_eq!(seen[0].body.len(), net::lookupdate::user_look_update_remote_len(&chr), "the exact length - one short faulted a client on the chair relay");
    assert_eq!(u32::from_le_bytes(seen[0].body[..4].try_into().unwrap()), my_id, "it names the changer");
    assert_eq!(seen[0].body[4], 1, "flag bit 0: a look follows");
    assert_eq!(&seen[0].body[5..5 + 195 + 5 * chr.equips.len()], &net::opcode::avatar_look(&chr)[..], "then the compact look, byte for byte");
    assert!(seen[0].body[5..].windows(4).any(|w| u32::from_le_bytes(w.try_into().unwrap()) == 42_604), "the look carries the new hair id - Ubel Hair in its default Green");
}

/// **`--look-reenter` is the fallback**: the leave, the enter with the new look, for that one
/// character - what a fresh sighting gets. Works on any client, at the price of the blink and
/// the pet respawn the owner measured; it is what to run if `0x02AE` is refuted on screen.
#[test]
fn with_look_reenter_a_hair_change_is_a_leave_then_an_enter_for_the_other_clients() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let config = Arc::new(Config { look_change_reenter: true, ..(*config).clone() });
    let (mut me, my_id) = join_channel(&store, &config, &fields, account, "Stylist");
    me.on_field_entered();
    let (mut them, _) = join_channel(&store, &config, &fields, account, "Bystander");
    them.on_field_entered();
    me.collect_mail();

    let slot = store.add_item(my_id, store::InventoryType::Use, &store::Item::bundle(2_543_143, 1), 1).unwrap()[0].slot;
    let mut body = slot.to_le_bytes().to_vec();
    body.extend_from_slice(&2_543_143u32.to_le_bytes());
    me.on_beauty_coupon_confirm(&body);

    let seen = them.collect_mail();
    let ops: Vec<u16> = seen.iter().map(|r| r.opcode).collect();
    assert_eq!(ops, vec![net::userpool::USER_LEAVE_FIELD, net::userpool::USER_ENTER_FIELD], "leave then enter, no 0x02AE: {ops:x?}");
    assert_eq!(u32::from_le_bytes(seen[0].body[..4].try_into().unwrap()), my_id, "the leave names the changer, nobody else");
    assert!(seen[1].body.windows(4).any(|w| u32::from_le_bytes(w.try_into().unwrap()) == 42_604), "the enter carries the new hair - Ubel Hair in its default Green");
}

/// **Putting on or taking off any equip reaches the other client at once.** The owner,
/// 2026-09-18: *"Whenever a client is changing their equipment, either a cash equipment or a
/// regular equipment, it is not being immediately reflected on other clients."* Only the
/// pet-hat slot was watched. Now any change to the worn set sends the observers one in-place
/// `0x02AE` with the new look and nothing else; a move that leaves the worn set as it was
/// (bag to bag) sends them nothing.
#[test]
fn a_worn_change_is_re_announced_to_the_map() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let (mut me, my_id) = join_channel(&store, &config, &fields, account, "Dresser");
    me.on_field_entered();
    let (mut them, _) = join_channel(&store, &config, &fields, account, "Bystander");
    them.on_field_entered();
    me.collect_mail();

    let sword = store.add_item(my_id, store::InventoryType::Equip, &store::Item::equip(1_302_000), 1).unwrap()[0].slot;
    let coat = store.add_item(my_id, store::InventoryType::Deco, &store::Item::equip(1_054_562), 1).unwrap()[0].slot;
    let potion = store.add_item(my_id, store::InventoryType::Use, &store::Item::bundle(2_000_000, 3), 3).unwrap()[0].slot;

    let announced = |them: &mut Session, what: &str| {
        let seen = them.collect_mail();
        let ops: Vec<u16> = seen.iter().map(|r| r.opcode).collect();
        assert_eq!(ops, vec![net::lookupdate::USER_LOOK_UPDATE_REMOTE], "{what}: one in-place 0x02AE and nothing else: {ops:x?}");
        assert_eq!(u32::from_le_bytes(seen[0].body[..4].try_into().unwrap()), my_id, "{what}: it names the changer");
        seen.into_iter().next().unwrap().body
    };
    let wears = |body: &[u8], slot: u8, id: u32| body.windows(5).any(|w| w[0] == slot && w[1..5] == id.to_le_bytes());

    // A regular equip on: worn slot 11, the weapon.
    let out = me.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, sword as i16, -11, -1));
    assert!(!out[0].what.contains("REFUSING"), "{}", out[0].what);
    let body = announced(&mut them, "sword on");
    assert!(wears(&body, 11, 1_302_000), "the look carries the sword");

    // A cash equip on: worn slot 105, the cash overall - it is what is DRAWN, in slot 5.
    let out = me.on_inventory_move(&inventory_move(net::inventory::INV_DECO, coat as i16, -105, -1));
    assert!(!out[0].what.contains("REFUSING"), "{}", out[0].what);
    let body = announced(&mut them, "coat on");
    assert!(wears(&body, 5, 1_054_562), "the look carries the coat");

    // Off again: the sword back to its bag slot.
    let out = me.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, -11, sword as i16, -1));
    assert!(!out[0].what.contains("REFUSING"), "{}", out[0].what);
    let body = announced(&mut them, "sword off");
    assert!(!wears(&body, 11, 1_302_000), "the look no longer carries the sword");

    // The control: a bag-to-bag move changes what is worn not at all, so the field hears nothing.
    let out = me.on_inventory_move(&inventory_move(store::InventoryType::Use as i8, potion as i16, potion as i16 + 1, -1));
    assert!(!out[0].what.contains("REFUSING"), "{}", out[0].what);
    assert!(them.collect_mail().is_empty(), "a bag shuffle is nobody else's business");
}

/// The captured double-click on Tester2, retargeted: `u32 tick, u32 id, str "", u8 petInfo`.
fn character_info_request(id: u32, pet_info: bool) -> Vec<u8> {
    let mut b = net::charinfo::CLIENT_CHARACTER_INFO_REQUEST.to_le_bytes().to_vec();
    b.extend_from_slice(&0x2c10b8u32.to_le_bytes());
    b.extend_from_slice(&id.to_le_bytes());
    b.extend_from_slice(&[0, 0]);
    b.push(u8::from(pet_info));
    b
}

/// **Double-clicking another player opens their Character Info.** The owner, 2026-09-18: *"When
/// double clicking another player, a similar Character Info window should show for as well for
/// players that are not yourself. I just tried double clicking on Tester2."* The click is
/// `0x01FC` naming the character; the answer is one `0x00A2` with their name, level, job, and
/// - when they have a pet out - the pet's name and vitals plus the pet item, with the panel
/// flag echoed. A character that does not exist gets the four-byte refusal, because both
/// client-side builders latch and an unanswered request freezes ~35 other senders.
#[test]
fn double_clicking_another_player_answers_with_their_character_info() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let mut item_names = std::collections::HashMap::new();
    item_names.insert(5_000_006u32, "Husky".to_string());
    let config = Arc::new(Config { item_names, ..(*config).clone() });
    let (mut me, _my_id) = join_channel(&store, &config, &fields, account, "Wisp");
    me.on_field_entered();
    let (mut them, their_id) = join_channel(&store, &config, &fields, account, "Tester2");
    them.on_field_entered();
    me.collect_mail(); // Tester2's arrival, so the click's answer is the only reply below
    let mut chr8 = them.claimed_character().unwrap();
    chr8.level = 8;
    store.save_character_progress(&chr8).unwrap();

    // No pet: the 60-byte body plus the name, level 8, job 0, pet item id 0, panel closed -
    // plus the two look entries (hair, face) the ITEM tab lists since 2026-09-18 evening.
    let out = me.handle(&character_info_request(their_id, false));
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!(out[0].opcode, net::charinfo::CHARACTER_INFO);
    let b = &out[0].body;
    let look_slot = net::opcode::equipped_item(chr8.hair, &net::opcode::EquipStats::default()).len();
    assert_eq!(b.len(), net::charinfo::CHARACTER_INFO_BASE_LEN + "Tester2".len() + 2 * look_slot);
    assert_eq!(&b[..4], &[0, 0, 0, 0], "result 0: show it");
    assert_eq!(u32::from_le_bytes(b[4..8].try_into().unwrap()), their_id);
    assert_eq!(&b[10..17], b"Tester2");
    assert_eq!(u32::from_le_bytes(b[17..21].try_into().unwrap()), 8, "level");
    assert_eq!(u32::from_le_bytes(b[31..35].try_into().unwrap()), 0, "no pet");

    // Tester2 summons a Husky; the same click now carries the pet, and petInfo opens the panel.
    store.add_item(their_id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
    them.last_position = Some((0, 0));
    them.on_pet_activate(&hex("509a18140100"));
    let pet_id = store.active_pet(their_id).unwrap().expect("the Husky is out").pet_id;
    store.set_pet_vitals(pet_id, 3, 250, 90).unwrap();
    me.collect_mail(); // the summon, broadcast to the map
    let out = me.handle(&character_info_request(their_id, true));
    let out: Vec<Reply> = out.into_iter().filter(|r| r.opcode == net::charinfo::CHARACTER_INFO).collect();
    assert_eq!(out.len(), 1, "{out:?}");
    let b = &out[0].body;
    let at = 31;
    assert_eq!(u32::from_le_bytes(b[at..at + 4].try_into().unwrap()), 5_000_006, "the pet item id");
    assert_eq!(&b[at + 6..at + 11], b"Husky", "the pet's name beside TYPE");
    let nums: Vec<u32> = (0..3).map(|i| u32::from_le_bytes(b[at + 11 + i * 4..at + 15 + i * 4].try_into().unwrap())).collect();
    assert_eq!(nums, vec![3, 250, 90], "level, closeness, fullness - what the panel prints");
    let flag = at + 11 + 20;
    assert_eq!(&b[flag - 4..flag + 1], &[0, 0, 0, 0, 0], "no hat: wear id 0 and no equip slot");
    assert_eq!(*b.last().unwrap(), 1, "the panel opens with the window, as asked");
    assert!(out[0].what.contains("Husky lv 3 closeness 250 fullness 90, wearing nothing"), "{}", out[0].what);

    // The Blue Top Hat on the pet (Deco slot -> worn 114): the cell under the pet is the hat.
    // The owner, 2026-09-18: "that slot is blank" while the pet item was sent there.
    let hat_slot = store.add_item(their_id, store::InventoryType::Deco, &store::Item::equip(1_802_006), 1).unwrap()[0].slot;
    them.on_inventory_move(&inventory_move(net::inventory::INV_DECO, hat_slot as i16, -114, -1));
    me.collect_mail();
    let out = me.handle(&character_info_request(their_id, true));
    let out: Vec<Reply> = out.into_iter().filter(|r| r.opcode == net::charinfo::CHARACTER_INFO).collect();
    let b = &out[0].body;
    assert_eq!(u32::from_le_bytes(b[flag - 4..flag].try_into().unwrap()), 1_802_006, "the wear item id");
    assert_eq!(b[flag], 1, "an equip slot follows");
    assert_eq!(b[flag + 1], 1, "an equip body, type byte 1");
    assert_eq!(u32::from_le_bytes(b[flag + 2..flag + 6].try_into().unwrap()), 1_802_006, "the hat");
    assert!(out[0].what.contains("wearing 1802006"), "{}", out[0].what);

    // The owner, 2026-09-18: "The pet equip can be scrolled, make sure that the Character Info shows
    // all scrolled information to other clients as well instead of just the base item." The
    // worn row keeps its own stats once a scroll writes them, and the cell is built from the
    // row, so a +5 STR hat with one enhancement left arrives as exactly that.
    let mut scrolled = net::opcode::EquipStats::default();
    scrolled.stats.inc_str = 5;
    scrolled.options.remaining_enhancements = 1;
    assert!(store.set_worn_equip(their_id, 114, &scrolled, 1).unwrap(), "the hat row took the scroll");
    let out = me.handle(&character_info_request(their_id, true));
    let out: Vec<Reply> = out.into_iter().filter(|r| r.opcode == net::charinfo::CHARACTER_INFO).collect();
    let b = &out[0].body;
    let expect = net::opcode::equipped_item(1_802_006, &scrolled);
    assert_eq!(&b[flag + 1..flag + 1 + expect.len()], &expect[..], "the hat's slot carries the scrolled stats, byte for byte");
    let base = net::opcode::equipped_item(1_802_006, &net::opcode::EquipStats::default());
    assert_ne!(expect, base, "and that is not the base item");

    // A character that does not exist: the refusal, four bytes, still an answer.
    let out = me.handle(&character_info_request(999_999, false));
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].opcode, net::charinfo::CHARACTER_INFO);
    assert_eq!(out[0].body, vec![1, 0, 0, 0]);

    // By name (id 0), which the other builder sends.
    let mut by_name = net::charinfo::CLIENT_CHARACTER_INFO_REQUEST.to_le_bytes().to_vec();
    by_name.extend_from_slice(&0u32.to_le_bytes());
    by_name.extend_from_slice(&0u32.to_le_bytes());
    by_name.extend_from_slice(&7u16.to_le_bytes());
    by_name.extend_from_slice(b"Tester2");
    by_name.push(0);
    let out = me.handle(&by_name);
    assert_eq!(u32::from_le_bytes(out[0].body[4..8].try_into().unwrap()), their_id, "resolved by name");
}

// ---------------------------------------------------------------------------------------
// Gift Drops - session/giftdrop.rs, crate::giftdrop, store::gifts.

/// **A GM queues a gift; the player on the same channel gets the Administrator's box at once;
/// Claim puts the item in the bag and settles the row; the box for a second gift follows.**
/// The owner, 2026-09-18: *"Can we do it via our usual MapleStory Administrator, but this time it
/// is via !giftdrop, and our usual show NPC chat dialogue."*
#[test]
fn a_gift_drop_is_queued_by_a_gm_offered_at_once_and_claimed_into_the_bag() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let mut item_names = std::collections::HashMap::new();
    item_names.insert(1_302_000u32, "Sword".to_string());
    item_names.insert(2_000_000u32, "Red Potion".to_string());
    let config = Arc::new(Config { item_names, ..(*config).clone() });
    let (mut gm, _gm_id) = join_channel(&store, &config, &fields, account, "Wisp");
    store.set_gm("maplecw", true).unwrap();
    gm.on_field_entered();
    // Tester2 is on a second, non-GM account: the "said out loud" control at the end needs it.
    let player_account = store.create_account("player", "correct horse battery").unwrap();
    let (mut them, their_id) = join_channel(&store, &config, &fields, player_account, "Tester2");
    them.on_field_entered();
    gm.collect_mail();
    them.collect_mail();

    // Nothing waiting: the public word says so, in their box.
    let out = them.handle(&gm_chat("!giftdrop"));
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].opcode, net::script::SCRIPT_MESSAGE);
    assert!(String::from_utf8_lossy(&out[0].body).contains("nothing to claim"), "{}", out[0].what);

    // The GM queues two. The ack names the row, and the target is on this channel.
    let out = gm.handle(&gm_chat("!giftdrop Tester2 1302000 1 Sorry about the crash"));
    let ack = notice_text(&out[0]);
    assert!(ack.contains("Gift #1 queued for Tester2: 1x Sword (1302000) - \"Sorry about the crash\"; expires in 7 days"), "{ack}");
    assert!(ack.contains("on this channel"), "{ack}");
    gm.handle(&gm_chat("!giftdrop Tester2 2000000 10"));
    let now = store::Store::unix_now();
    assert_eq!(store.pending_gifts(their_id, player_account, now).unwrap().len(), 2);

    // Tester2's session drains the event: the notice, then the box for the OLDEST gift, with
    // the sword's icon and name, the message, "1 more waiting", Claim = 0, Refuse = 1.
    let seen = them.collect_mail();
    let boxes: Vec<&Reply> = seen.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).collect();
    assert_eq!(boxes.len(), 1, "one box: {:?}", seen.iter().map(|r| &r.what).collect::<Vec<_>>());
    let text = String::from_utf8_lossy(&boxes[0].body).into_owned();
    assert!(text.contains("GIFT DROP") && text.contains("Sorry about the crash") && text.contains("#i1302000# #t1302000# x1"), "{text}");
    assert!(text.contains("1 more waiting") && text.contains("#L0# Claim#l") && text.contains("#L1# Refuse#l") && text.contains("#L2# Cancel"), "{text}");
    assert!(text.contains("Expires in 7 days."), "{text}");
    assert!(seen.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE && notice_text(r).contains("2 gifts waiting")), "the notice");
    assert_eq!(them.conversation.as_ref().map(|c| c.path.clone()), Some("giftdrop.1".to_string()));

    // Claim: the row settles, the sword is in the Equip tab, the bag op goes out, they say so,
    // and the second gift's box opens behind it.
    let out = them.on_script_reply(&menu_reply(Some(0)));
    assert!(out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    let says: Vec<String> = out.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).map(|r| String::from_utf8_lossy(&r.body).into_owned()).collect();
    assert!(says[0].contains("Claimed: #i1302000# #t1302000# x1"), "{}", says[0]);
    assert!(says[1].contains("#i2000000# #t2000000# x10") && !says[1].contains("more waiting"), "the next box: {}", says[1]);
    let bag = store.bag(their_id).unwrap();
    assert_eq!(bag.items_in(store::InventoryType::Equip).filter(|i| i.item.item_id == 1_302_000).count(), 1);
    assert_eq!(store.pending_gifts(their_id, player_account, now).unwrap().len(), 1);
    assert_eq!(them.conversation.as_ref().map(|c| c.path.clone()), Some("giftdrop.2".to_string()));

    // Cancel on the second box: nothing settles, nothing given, they say it is kept.
    let out = them.on_script_reply(&menu_reply(Some(2)));
    let says: Vec<String> = out.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).map(|r| String::from_utf8_lossy(&r.body).into_owned()).collect();
    assert_eq!(says.len(), 1, "{says:?}");
    assert!(says[0].contains("Kept for later"), "{}", says[0]);
    assert!(!out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION));
    assert_eq!(store.pending_gifts(their_id, player_account, now).unwrap().len(), 1, "still queued after a cancel");
    assert!(them.conversation.is_none());

    // A stale click on the first box gives nothing: the path names row 1, which is settled.
    them.conversation = Some(Conversation { npc_template: crate::dailyperks::ADMIN_TEMPLATE, quest_id: None, path: "giftdrop.1".into(), sent: 0, awaiting_yes_no: false, sent_with_next: false });
    let out = them.on_script_reply(&menu_reply(Some(0)));
    assert!(!out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "nothing given twice");
    assert_eq!(store.bag(their_id).unwrap().items_in(store::InventoryType::Equip).count(), 1);

    // Refuse the potions: settled without giving, nothing more waiting, no further box.
    them.handle(&gm_chat("!giftdrop"));
    let out = them.on_script_reply(&menu_reply(Some(1)));
    let says: Vec<String> = out.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).map(|r| String::from_utf8_lossy(&r.body).into_owned()).collect();
    assert_eq!(says.len(), 1, "{says:?}");
    assert!(says[0].contains("You refused #t2000000#"));
    assert!(store.pending_gifts(their_id, player_account, now).unwrap().is_empty());
    assert!(store.bag(their_id).unwrap().items_in(store::InventoryType::Use).next().is_none(), "nothing given on a refuse");

    // A non-GM with arguments is said out loud, like every GM word; the bare word still works.
    let out = them.handle(&gm_chat("!giftdrop Wisp 1302000"));
    assert!(out.iter().any(|r| r.opcode == net::userchat::USER_CHAT), "said out loud: {:?}", out.iter().map(|r| r.opcode).collect::<Vec<_>>());
    assert_eq!(store.pending_gifts(_gm_id, account, now).unwrap().len(), 0, "and nothing queued");
}

/// **`!giftall` is one gift per account, offered on every screen on the channel, claimable on
/// any character of the account, once; the second character sees it gone.** The owner: *"gives all
/// accounts (not character) an item. The player can claim it on any character they want."*
#[test]
fn giftall_queues_one_gift_per_account_that_any_of_its_characters_can_claim_once() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let mut item_names = std::collections::HashMap::new();
    item_names.insert(2_000_000u32, "Red Potion".to_string());
    let config = Arc::new(Config { item_names, ..(*config).clone() });
    let (mut gm, gm_id) = join_channel(&store, &config, &fields, account, "Wisp");
    store.set_gm("maplecw", true).unwrap();
    gm.on_field_entered();
    let player_account = store.create_account("player", "correct horse battery").unwrap();
    let (mut main, main_id) = join_channel(&store, &config, &fields, player_account, "Tester2");
    main.on_field_entered();
    // The same account's second character, not online.
    let alt = net::opcode::Character { name: "Tester3".to_string(), map_id: SHARED_MAP, ..Default::default() };
    let alt_id = store.create_character(player_account, 0, &alt).unwrap().id;
    gm.collect_mail();
    main.collect_mail();

    let out = gm.handle(&gm_chat("!giftall 2000000 5 Thanks for testing"));
    let ack = notice_text(&out[0]);
    assert!(ack.contains("Gift queued for 2 account(s)") && ack.contains("5x Red Potion") && ack.contains("expires in 7 days"), "{ack}");
    let now = store::Store::unix_now();
    // Both of the player account's characters see the same row; the GM's account has its own.
    let seen_by_main = store.pending_gifts(main_id, player_account, now).unwrap();
    assert_eq!(seen_by_main.len(), 1);
    assert_eq!(store.pending_gifts(alt_id, player_account, now).unwrap(), seen_by_main);
    assert_eq!(store.pending_gifts(gm_id, account, now).unwrap().len(), 1);

    // Tester2's screen: the box, marked as the account's, at once.
    let seen = main.collect_mail();
    let box_text = seen.iter().find(|r| r.opcode == net::script::SCRIPT_MESSAGE).map(|r| String::from_utf8_lossy(&r.body).into_owned()).expect("the box");
    assert!(box_text.contains("For your account: claim it on whichever character you like."), "{box_text}");
    // The GM's own screen got one too - the GM's account is an account, and a session drains
    // its own events right after the reply to the packet that queued them.
    assert!(out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE), "the GM's own box: {:?}", out.iter().map(|r| r.opcode).collect::<Vec<_>>());

    // Tester2 cancels (to claim on Tester3), logs the alt in: the alt is offered it on the
    // first move and claims it; Tester2 then has nothing.
    main.on_script_reply(&menu_reply(Some(2)));
    store.create_migration(player_account, alt_id, 0, 0).unwrap();
    let mut alt_s = Session::joining(store.clone(), config.clone(), fields.clone());
    assert!(alt_s.claim_for_character(alt_id).contains("claimed"), "the alt's claim");
    alt_s.on_field_entered();
    let a_move = || { let mut b = net::usermove::CLIENT_USER_MOVE.to_le_bytes().to_vec(); b.extend_from_slice(&[0u8; 8]); b };
    let out = alt_s.handle(&a_move());
    assert!(out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE), "offered to the alt on its first move");
    let out = alt_s.on_script_reply(&menu_reply(Some(0)));
    assert!(out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "claimed on the alt");
    assert_eq!(store.bag(alt_id).unwrap().items_in(store::InventoryType::Use).filter(|i| i.item.item_id == 2_000_000).count(), 1);
    assert!(store.pending_gifts(main_id, player_account, now).unwrap().is_empty(), "settled for the whole account");
    let out = main.handle(&gm_chat("!giftdrop"));
    assert!(String::from_utf8_lossy(&out[0].body).contains("nothing to claim"), "Tester2 cannot claim it again");
    assert_eq!(store.bag(main_id).unwrap().items_in(store::InventoryType::Use).count(), 0);
}

/// **A full tab keeps the gift queued** - the quests' own "make N spaces" box, no settle, no
/// item - and a gift queued while the player was away is offered on the first move after
/// their next field entry, never with the SetField.
#[test]
fn a_gift_drop_into_a_full_tab_waits_and_an_offline_gift_is_offered_on_the_first_move() {
    let (mut s, store, id) = claimed_session();
    let mut names = std::collections::HashMap::new();
    names.insert(1_302_000u32, "Sword".to_string());
    s.config = Arc::new(Config { item_names: names, ..(*s.config).clone() });
    empty_bag(&store, id);
    for i in 0..30u32 {
        store.add_item(id, store::InventoryType::Equip, &store::Item::equip(1_302_000 + i % 3), 1).unwrap();
    }
    // Queued "offline": straight into the store, as another channel's GM would. Dated now,
    // so the seven-day expiry has not passed.
    let now = store::Store::unix_now();
    let account = s.claimed.as_ref().unwrap().account_id;
    store.queue_gift(store::GiftTarget::Character(id), 1_302_000, 1, "", "Wisp", now).unwrap();

    // Field entry arms it and sends no box; the first move sends the notice and the box.
    let entered = s.on_field_entered();
    assert!(!entered.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE), "no 0x055B with the entry");
    let a_move = || { let mut b = net::usermove::CLIENT_USER_MOVE.to_le_bytes().to_vec(); b.extend_from_slice(&[0u8; 8]); b };
    let out = s.handle(&a_move());
    assert!(out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE && notice_text(r).contains("a gift waiting")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE), "the box on the first move");
    assert!(s.handle(&a_move()).iter().all(|r| r.opcode != net::script::SCRIPT_MESSAGE), "once");

    // Claim with the Equip tab full: their refusal names the tab, the row stays pending.
    let out = s.on_script_reply(&menu_reply(Some(0)));
    let say = out.iter().find(|r| r.opcode == net::script::SCRIPT_MESSAGE).expect("the refusal box");
    let text = String::from_utf8_lossy(&say.body);
    assert!(text.contains("Please make 1 space in your Equip tab."), "{text}");
    assert!(!out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION));
    assert_eq!(store.pending_gifts(id, account, now).unwrap().len(), 1, "still queued");

    // One slot freed: the same gift claims.
    store.remove_item(id, store::InventoryType::Equip, 30, None).unwrap();
    s.handle(&gm_chat("!giftdrop"));
    let out = s.on_script_reply(&menu_reply(Some(0)));
    assert!(out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION));
    assert!(store.pending_gifts(id, account, now).unwrap().is_empty());

    // An expired gift is not offered: eight days old, the entry arms nothing.
    store.queue_gift(store::GiftTarget::Character(id), 1_302_000, 1, "", "Wisp", now - 8 * 86_400).unwrap();
    s.on_field_entered();
    assert!(s.handle(&a_move()).iter().all(|r| r.opcode != net::script::SCRIPT_MESSAGE), "expired: no box");
    let out = s.handle(&gm_chat("!giftdrop"));
    assert!(String::from_utf8_lossy(&out[0].body).contains("nothing to claim"));
}

/// **A late joiner is told where people ARE, not where they were when they arrived.**
///
/// The owner, 2026-09-03: *"the positioning is off if someone joins the map later since they don't
/// know where existing clients are."* The spawn packet is built once at field entry and the
/// bus hands that same body to every later arrival, so a player who walked across the map
/// still appears at the origin - which is where they were standing before their first step.
///
/// The assertion is on the POSITION BYTES of the packet the joiner actually receives, not on
/// `last_position`: the session knowing where somebody is was never the problem.
#[test]
fn someone_joining_late_is_told_where_the_others_are_now() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let (mut early, _) = join_channel(&store, &config, &fields, account, "Early");
    early.on_field_entered();

    // Early walks. This is the move packet's own path, not a test back door.
    let walked = (1337i16, 395i16);
    early.note_own_position(walked.0, walked.1, None);

    let (mut late, _) = join_channel(&store, &config, &fields, account, "Late");
    let seen = late.on_field_entered();
    let spawn: Vec<&Reply> =
        seen.iter().filter(|r| r.opcode == net::userpool::USER_ENTER_FIELD).collect();
    assert_eq!(spawn.len(), 1, "the joiner is told about exactly one other player: {seen:?}");

    // `Early` has an empty equip list here, so the only shift is the name.
    let at = net::userpool::USER_ENTER_FIELD_POS_AT + "Early".len();
    let body = &spawn[0].body;
    assert_eq!(
        (
            i16::from_le_bytes(body[at..at + 2].try_into().unwrap()),
            i16::from_le_bytes(body[at + 2..at + 4].try_into().unwrap()),
        ),
        walked,
        "the joiner must be told where Early is NOW, not where it entered"
    );

    // The control, and it is what makes the assertion above mean something: the ORIGIN is
    // what this used to send, so a test that happened to pass on a stale body would read
    // (0, 0) here.
    assert_ne!(walked, (0, 0), "the walked-to position must differ from the origin");
}

/// **A controller that leaves hands its mobs to somebody still there, and that somebody is
/// told without having to move.**
///
/// The owner, 2026-09-01: *"If the person who is controlling the movement of the mob leaves the map,
/// then the mob should not disappear. That's a jarring experience. The control of the mob
/// should be handed over to another client who is still present in the map."*
///
/// The mobs never did disappear - a release leaves the mob alive, and the assertion on
/// `mob_count` below is what says so. The real symptom was worse and quieter: **nobody was
/// told**, so every monster on that map stood perfectly still on the remaining screens with no
/// error and no log line. This test's predecessor missed it by calling `on_field_entered` a
/// second time to make the grants appear - and a player standing still never does that, which
/// is the entire case.
///
/// So the grants are collected from `tick`, the idle path, with no field entry anywhere after
/// the departure.
#[test]
fn a_departing_controller_hands_its_mobs_to_whoever_is_left() {
    let (store, config, fields, account) = shared_channel(4, 30);
    let alive = fields.mob_count(crate::fields::FieldKey::world(SHARED_MAP));
    let (mut leaver, _) = join_channel(&store, &config, &fields, account, "Leaver");
    let (mut stayer, _) = join_channel(&store, &config, &fields, account, "Stayer");
    leaver.on_field_entered();
    let spectating = stayer.on_field_entered();
    assert_eq!(
        count_of(&spectating, net::mobmove::MOB_CHANGE_CONTROLLER),
        0,
        "the control: the leaver got there first, so the stayer drives nothing yet"
    );
    assert_eq!(fields.controllers().held_by(leaver.subscriber.get()), alive);

    let out = leaver.on_log_out();
    assert!(!out.is_empty(), "log out is answered, and that is not optional");
    assert_eq!(
        count_of(&out, net::mobmove::MOB_CHANGE_CONTROLLER),
        0,
        "nothing goes to the LEAVER - level 0 is the only revoke and it DESPAWNS the mob"
    );
    assert_eq!(fields.controllers().held_by(leaver.subscriber.get()), 0, "the claims are gone");
    assert_eq!(fields.mob_count(crate::fields::FieldKey::world(SHARED_MAP)), alive, "and the mobs themselves are untouched");
    assert_eq!(
        fields.controllers().held_by(stayer.subscriber.get()),
        alive,
        "...because they were handed over rather than dropped on the floor"
    );

    // The half that used to be missing. No `on_field_entered` - the stayer has not moved, has
    // not walked a portal and has sent nothing. A tick is what an idle client's socket does.
    let arrived = stayer.tick(1_000);
    let grants: Vec<&Reply> =
        arrived.iter().filter(|r| r.opcode == net::mobmove::MOB_CHANGE_CONTROLLER).collect();
    assert_eq!(
        grants.len(),
        alive,
        "a standing client must be told it now drives them, or they freeze: {arrived:?}"
    );
    assert!(
        grants.iter().all(|g| g.body[2] != net::mobmove::CONTROL_RELEASE),
        "and told with a level that MOVES the mob, not one that despawns it"
    );
}

/// **The handover carries the mob's current position, not its spawn point.**
///
/// The heir's client resumes the wander from whatever coordinates the grant names. Send the
/// spawn point and every handed-over monster teleports across the map on the remaining
/// screens - which is precisely the jarring thing this whole path exists to avoid, arriving
/// by a different door.
#[test]
fn a_handed_over_mob_is_granted_where_it_is_standing() {
    let (store, config, fields, account) = shared_channel(1, 30);
    let (mut leaver, _) = join_channel(&store, &config, &fields, account, "Leaver");
    let (mut stayer, _) = join_channel(&store, &config, &fields, account, "Stayer");
    leaver.on_field_entered();
    stayer.on_field_entered();

    let object_id = fields.mobs_on(crate::fields::FieldKey::world(SHARED_MAP))[0].spawn.object_id;
    let spawn_x = fields.mobs_on(crate::fields::FieldKey::world(SHARED_MAP))[0].spawn.x;
    let walked_to = (spawn_x + 300, 395);
    fields.note_position(crate::fields::FieldKey::world(SHARED_MAP), object_id, walked_to);

    leaver.on_log_out();
    let grants: Vec<Reply> = stayer
        .tick(1_000)
        .into_iter()
        .filter(|r| r.opcode == net::mobmove::MOB_CHANGE_CONTROLLER)
        .collect();
    assert_eq!(grants.len(), 1, "one mob, one grant: {grants:?}");

    // The same builder the field-entry grant uses, fed the mob as it stands. Comparing whole
    // bodies rather than picking an offset out: an offset would have to be re-derived here and
    // that is a second claim about the packet layout to get wrong.
    let expected_here = net::mobmove::mob_change_controller(
        &fields.mobs_on(crate::fields::FieldKey::world(SHARED_MAP))[0].as_seen(),
        net::mobmove::CONTROL_NORMAL,
    );
    assert_eq!(grants[0].body, expected_here, "granted where it is standing");

    // The control that gives the assertion above its teeth: the spawn-point body is a
    // DIFFERENT packet, so this test would fail if the handover sent that instead.
    let spawn_body = net::mobmove::mob_change_controller(
        &fields.mobs_on(crate::fields::FieldKey::world(SHARED_MAP))[0].spawn,
        net::mobmove::CONTROL_NORMAL,
    );
    assert_ne!(expected_here, spawn_body, "the mob really has moved away from its spawn point");
}

/// **A client that crashes hands its mobs over too.**
///
/// The exit nobody takes deliberately: the socket drops, the process is killed. It goes
/// through no log out and no portal, so `Drop` is the only thing left - and `Drop` used to
/// call `release_all`, which is correct about ownership and says nothing to anybody.
///
/// This is the departure where a silent handover matters most, because the person who left is
/// the one person who cannot see the result.
#[test]
fn a_crashed_connection_hands_its_mobs_over_rather_than_stranding_them() {
    let (store, config, fields, account) = shared_channel(3, 30);
    let alive = fields.mob_count(crate::fields::FieldKey::world(SHARED_MAP));
    let (mut stayer, _) = join_channel(&store, &config, &fields, account, "Stayer");

    {
        let (mut doomed, _) = join_channel(&store, &config, &fields, account, "Doomed");
        doomed.on_field_entered();
        assert_eq!(
            fields.controllers().held_by(doomed.subscriber.get()),
            alive,
            "the control: it really is driving them when the socket dies"
        );
        stayer.on_field_entered();
        // and `doomed` is dropped here - no log out, no channel change, no portal walk.
    }

    assert_eq!(
        fields.controllers().held_by(stayer.subscriber.get()),
        alive,
        "a crash must not strand the mobs on a session id that will never exist again"
    );
    let arrived = stayer.tick(1_000);
    assert_eq!(
        count_of(&arrived, net::mobmove::MOB_CHANGE_CONTROLLER),
        alive,
        "and the survivor is told, without moving: {arrived:?}"
    );
}

/// **The last player out releases rather than handing over, and the field still works.**
///
/// The degenerate case, and the one that would break quietly: `successor_on` returns `None`,
/// so there is nobody to send a grant to. If that path forgot to free the claims instead, the
/// mobs would stay owned by a departed session and the next arrival would be granted nothing -
/// an empty, motionless map that looks exactly like a spawn failure.
#[test]
fn the_last_player_out_frees_the_mobs_for_the_next_arrival() {
    let (store, config, fields, account) = shared_channel(2, 30);
    let alive = fields.mob_count(crate::fields::FieldKey::world(SHARED_MAP));
    let (mut only, _) = join_channel(&store, &config, &fields, account, "Only");
    only.on_field_entered();
    assert_eq!(fields.controllers().held_by(only.subscriber.get()), alive);

    only.on_log_out();
    assert_eq!(fields.controllers().held_by(only.subscriber.get()), 0, "nobody to hand them to");
    assert_eq!(fields.mob_count(crate::fields::FieldKey::world(SHARED_MAP)), alive, "the mobs are still alive on the map");

    let (mut next, _) = join_channel(&store, &config, &fields, account, "Next");
    let welcome = next.on_field_entered();
    assert_eq!(
        count_of(&welcome, net::mobmove::MOB_CHANGE_CONTROLLER),
        alive,
        "the next arrival takes all of them: {welcome:?}"
    );
}

/// **Re-entering a field re-grants what this connection already controls.**
///
/// The `SetField` destroys the client's mob pool, so every grant it was holding is void. A
/// registry that was only additive would answer "you already control these" and send nothing,
/// and every monster on that screen would stand still for the rest of the session - which is
/// the bug the registry was added to fix, arriving from the other direction.
#[test]
fn coming_back_to_a_map_this_connection_controls_re_sends_every_grant() {
    let (store, config, fields, account) = shared_channel(4, 30);
    let alive = fields.mob_count(crate::fields::FieldKey::world(SHARED_MAP));
    let (mut only, _) = join_channel(&store, &config, &fields, account, "Solo");

    let first = only.on_field_entered();
    assert_eq!(count_of(&first, net::mobmove::MOB_CHANGE_CONTROLLER), alive);

    let again = only.on_field_entered();
    assert_eq!(
        count_of(&again, net::mobmove::MOB_CHANGE_CONTROLLER),
        alive,
        "a second SetField needs the grants again, not a registry saying 'already yours'"
    );
    assert_eq!(
        fields.controllers().held_by(only.subscriber.get()),
        alive,
        "and still exactly one holder"
    );
}


// ---------------------------------------------------------------- account codes, 2026-09-05

/// A code as the two commands print it: eight alphabet characters as `XXXX-XXXX`.
fn code_in(text: &str) -> Option<String> {
    text.split_whitespace()
        .map(|t| t.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '-'))
        .find(|t| {
            t.len() == 9
                && t.as_bytes()[4] == b'-'
                && t.chars().filter(|c| *c != '-').all(|c| store::CODE_ALPHABET.contains(c))
        })
        .map(str::to_string)
}

/// **`!registrationcode` mints a code the launcher can redeem, shows it to the GM, and logs
/// none of it.** The owner, 2026-09-05. The code is a credential: it belongs on the GM's screen
/// and not in world.log, so `Reply::what` carries a redaction rather than the text.
#[test]
fn a_gm_mints_a_registration_code_that_redeems_once_and_is_not_logged() {
    let (mut s, store, _) = gm_session();
    let out = s.handle(&gm_chat("!registrationcode"));
    assert_eq!(out.len(), 1, "one chat notice");
    assert_eq!(out[0].opcode, net::notice::CHAT_NOTICE);
    let text = notice_text(&out[0]);
    let code = code_in(&text).unwrap_or_else(|| panic!("no code in {text:?}"));
    assert!(!out[0].what.contains(&code), "the code must not reach world.log: {}", out[0].what);
    assert!(out[0].what.contains("not logged"), "{}", out[0].what);
    assert!(store.redeem_invite_code(&code).unwrap(), "the launcher could redeem it");
    assert!(!store.redeem_invite_code(&code).unwrap(), "exactly once");
}

/// `!recoverycode` takes the email or the username, refuses a blank, and says when nobody
/// matches - a code for an account that does not exist would be a promise nothing can keep.
#[test]
fn a_gm_mints_a_recovery_code_by_email_or_name_and_is_told_when_neither_exists() {
    let (mut s, store, _) = gm_session();
    store.set_email("maplecw", Some("gm@example.test")).unwrap();
    let account = store.get_account("maplecw").unwrap().unwrap();

    let by_email = notice_text(&s.handle(&gm_chat("!recoverycode gm@example.test"))[0]);
    let code = code_in(&by_email).unwrap_or_else(|| panic!("no code in {by_email:?}"));
    assert!(store.redeem_recovery_code_for(&code, account.id).unwrap(), "minted for that account");

    let by_name = notice_text(&s.handle(&gm_chat("!recoverycode maplecw"))[0]);
    assert!(code_in(&by_name).is_some(), "{by_name}");

    let missing = notice_text(&s.handle(&gm_chat("!recoverycode nobody@example.test"))[0]);
    assert!(missing.contains("no account"), "{missing}");
    assert!(code_in(&missing).is_none(), "and no code was shown: {missing}");

    let blank = notice_text(&s.handle(&gm_chat("!recoverycode"))[0]);
    assert!(blank.contains("needs"), "{blank}");
}

/// The gate every `!` command has: a non-GM typing these says them out loud and mints nothing.
#[test]
fn a_non_gm_typing_the_code_commands_mints_nothing() {
    let (mut s, store, _) = gm_session();
    store.set_gm("maplecw", false).unwrap();
    let out = s.handle(&gm_chat("!registrationcode"));
    assert_eq!(out[0].opcode, net::userchat::USER_CHAT, "said, not obeyed");
    assert_eq!(store.live_code_counts().unwrap(), (0, 0));
}

// ---------------------------------------------------------------- the channel holds the address, 2026-09-05

/// **The off-box half of "only the launcher's client enters."** A migration minted for a
/// login connection from one address cannot be claimed by a channel connection from another;
/// the same address claims it. `PeerPolicy::Require` is the world server's default now.
#[test]
fn a_channel_refuses_a_migration_claimed_from_a_different_address_and_accepts_the_same_one() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Roamer".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration_bound_hash(account_id, id, 0, 0, None, Some("192.168.1.5")).unwrap();
    let config = Arc::new(Config::default());

    let mut stranger = Session::new(store.clone(), config.clone())
        .with_peer_addr("192.168.1.9:50000".parse().unwrap());
    let note = stranger.claim_for_character(id);
    assert!(note.contains("REFUSED"), "{note}");
    assert!(note.contains("192.168.1.5") && note.contains("192.168.1.9"), "names both addresses: {note}");
    assert!(stranger.claimed().is_none());

    let mut owner = Session::new(store, config).with_peer_addr("192.168.1.5:50001".parse().unwrap());
    let note = owner.claim_for_character(id);
    assert!(note.contains("claimed the migration"), "the refusal did not consume it: {note}");
    assert!(owner.claimed().is_some());
}

/// **The Quest EXP rate multiplies a turn-in, and the kill rate does not.** The owner, 2026-09-06:
/// *"it should also now affect quest exp obtained from quest completion"* - the fourth
/// `!setrates` field. Quest 1000 pays nothing; **1001** (Sera's, auto-started when 1000
/// completes, turned in to Heena) pays `Act.1.exp = 2`. At `!setrates 5 1 1 1 30` it still
/// pays 2 (the kill rate is not the quest rate), and at `!setrates 1 1 1 2 30` it pays 4, with
/// the reason line saying so.
#[test]
fn the_quest_exp_rate_multiplies_a_turn_in_and_the_kill_rate_does_not() {
    let path = std::path::Path::new("../../gm-handbook/questlines.txt");
    if !path.exists() {
        return; // generated data, gitignored
    }
    // `0x0151`: u8 op, u32 questId, u32 npc, i16 x, i16 y, u32 -1. The 1000 bodies are the
    // captured ones the fanfare test uses; 1001's completion is the same shape with its own
    // id (0x3e9) and its end NPC, Heena (1).
    let turn_in = |s: &mut Session| -> (u64, String) {
        s.on_quest_request(&hex("01e8030000010000000c046d0100000000")); // start 1000 at Heena
        s.on_quest_request(&hex("02e80300000200000043ffe501ffffffff")); // complete 1000 at Sera -> 1001 starts
        let done = s.on_quest_request(&hex("02e90300000100000043ffe501ffffffff")); // complete 1001 at Heena
        let exp = done
            .iter()
            .find(|r| r.opcode == net::stats::STAT_CHANGED && r.what.contains("exp from quest 1001"))
            .unwrap_or_else(|| panic!("a quest EXP line: {:?}", done.iter().map(|r| &r.what).collect::<Vec<_>>()));
        let gained: u64 = exp.what.trim_start_matches("StatChanged: +").split(' ').next().unwrap().parse().unwrap();
        (gained, exp.what.clone())
    };
    let fresh = || {
        // `answer_packets` (the default), or `handle` answers nothing and `!setrates` is lost
        // on the floor - which reads exactly like the rate not applying.
        let config = Config {
            quests: crate::config::load_quests(path),
            ..Config::default()
        };
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        store.set_gm("maplecw", true).unwrap();
        let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
        let id = store.create_character(account_id, 0, &chr).unwrap().id;
        store.create_migration(account_id, id, 0, 0).unwrap();
        let mut s = Session::new(store, Arc::new(config));
        s.claim_for_character(id);
        s
    };

    let mut s = fresh();
    let (base, _) = turn_in(&mut s);
    assert!(base > 0, "the positive control: quest 1000 pays something");

    let mut s = fresh();
    s.handle(&gm_chat("!setrates 5 1 1 1 30"));
    let (at_kill_rate, why) = turn_in(&mut s);
    assert_eq!(at_kill_rate, base, "the KILL rate does not touch a turn-in: {why}");

    let mut s = fresh();
    s.handle(&gm_chat("!setrates 1 1 1 2 30"));
    let (doubled, why) = turn_in(&mut s);
    assert_eq!(doubled, base * 2, "the QUEST rate does: {why}");
    assert!(why.contains(&format!("quest 1001 ({base} at 2x)")), "and the reason says so: {why}");
}

// ---------------------------------------------------------------------------------------
// The second- and third-job audit, 2026-09-07.
// research/second-third-job-audit-2026-09-07.md
// ---------------------------------------------------------------------------------------

/// A level-70 character of `job` with `skills` at the given levels, `etc` items in the Etc
/// tab, `mesos` in the purse, 1000/1000 HP and MP, claimed. `None` without the generated
/// table. `tweak` edits the config before the session takes it.
fn adv_session_with(
    job: u16,
    skills: &[(u32, u32)],
    etc: &[(u32, u16)],
    mesos: u32,
    tweak: impl FnOnce(&mut Config),
) -> Option<(Arc<Store>, Session, u32)> {
    let table = std::path::Path::new("../../gm-handbook/skills.txt");
    if !table.exists() {
        return None; // python tools/dump_skills.py
    }
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Advanced".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.job = job;
    made.level = 70;
    made.hp = 1000;
    made.max_hp = 1000;
    made.mp = 1000;
    made.max_mp = 1000;
    store.save_character_progress(&made).unwrap();
    for (skill, level) in skills {
        store.set_skill_level(made.id, *skill, *level).unwrap();
    }
    for (item, n) in etc {
        store
            .add_item(made.id, store::InventoryType::Etc, &store::Item::bundle(*item, *n), 100)
            .unwrap();
    }
    if mesos > 0 {
        store.set_mesos(made.id, mesos).unwrap();
    }
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let mut config = Config {
        firstjob: crate::firstjob::CombatTable::load(table),
        ..Config::default()
    };
    tweak(&mut config);
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(made.id);
    Some((store, s, made.id))
}

fn adv_session(job: u16, skills: &[(u32, u32)], etc: &[(u32, u16)], mesos: u32) -> Option<(Arc<Store>, Session, u32)> {
    adv_session_with(job, skills, etc, mesos, |_| {})
}

fn hp_of(s: &Session) -> u32 {
    s.claimed_character().expect("a character is claimed").hp
}

/// The CTS bits a `0x007D` sets, and its entries as `(value, reason, duration_ms)`, in the
/// order the body carries them - ascending bit, whatever the caller passed.
fn stat_set(reply: &Reply) -> (Vec<u32>, Vec<(i16, u32, u32)>) {
    let bits = net::buff::bits_in_mask(&reply.body[..net::buff::MASK_LEN]);
    let entries = (0..bits.len())
        .map(|i| {
            let at = net::buff::MASK_LEN + i * 10;
            (
                i16::from_le_bytes([reply.body[at], reply.body[at + 1]]),
                u32::from_le_bytes(reply.body[at + 2..at + 6].try_into().unwrap()),
                u32::from_le_bytes(reply.body[at + 6..at + 10].try_into().unwrap()),
            )
        })
        .collect();
    (bits, entries)
}

fn first_stat_set(out: &[Reply]) -> Option<&Reply> {
    out.iter().find(|r| r.opcode == net::buff::TEMPORARY_STAT_SET)
}

/// **A toggle is held until it is cancelled; a timed buff still expires on schedule.**
///
/// Before 2026-09-07 `grant_buff_with_tail` recorded a toggle's expiry as `now + 0`, and
/// `buff_tick` took Magic Guard off again on the next pass of the session loop. The one
/// Magic Guard test grants and hits at the same instant, so it never saw a tick.
#[test]
fn a_toggle_survives_the_buff_tick_and_a_timed_buff_does_not() {
    let Some((_store, mut s, _id)) =
        adv_session(210, &[(net::buff::MAGIC_GUARD, 1), (2_101_000, 1)], &[], 0)
    else {
        return;
    };
    s.on_skill_use(&skill_use_body(net::buff::MAGIC_GUARD, 1));
    assert!(s.holds(net::buff::CTS_MAGIC_GUARD), "the cast landed");
    let out = s.buff_tick(3_600_000);
    assert!(s.holds(net::buff::CTS_MAGIC_GUARD), "a toggle has no expiry");
    assert_eq!(count_of(&out, net::buff::TEMPORARY_STAT_RESET), 0, "and no 0x007E: {out:?}");
    // The control: Meditation L1 is 100 s and must still go.
    s.on_skill_use(&skill_use_body(2_101_000, 1));
    assert!(s.holds(net::jobbuffs::CTS_MAGIC_ATTACK));
    assert!(s.buff_tick(99_999).is_empty());
    let out = s.buff_tick(100_000);
    assert!(!s.holds(net::jobbuffs::CTS_MAGIC_ATTACK), "expired on schedule");
    assert_eq!(count_of(&out, net::buff::TEMPORARY_STAT_RESET), 1);
    assert!(s.holds(net::buff::CTS_MAGIC_GUARD), "the toggle is untouched by a neighbour expiring");
}

/// **A Booster costs HP as well as MP**, and the buff tables never carried an HP cost.
#[test]
fn a_booster_costs_hp_and_mp_and_rides_bit_96_at_minus_two() {
    let Some((_store, mut s, _id)) = adv_session(110, &[(1_101_002, 1)], &[], 0) else { return };
    let out = s.on_skill_use(&skill_use_body(1_101_002, 1));
    let set = first_stat_set(&out).expect("Sword Booster grants a stat");
    let (bits, entries) = stat_set(set);
    assert_eq!(bits, vec![net::jobbuffs::CTS_BOOSTER]);
    assert_eq!(entries[0].0, -2, "x = -2, 'by 2 stages', sign kept");
    assert_eq!(entries[0].1, 1_101_002, "the reason is the skill");
    assert_eq!(entries[0].2, 100_000, "100 s, in MILLISECONDS");
    assert_eq!(mp_of(&s), 1000 - 30, "mpCon 30");
    assert_eq!(hp_of(&s), 1000 - 30, "hpCon 30");
    let stat = out.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).expect("0x007C");
    assert!(stat.what.contains("hpCon"), "{}", stat.what);
}

/// **A skill the server grants nothing for still costs what the client's table says.**
///
/// Teleport: the client moves the character itself. The server used to answer with a chat
/// line and keep the MP - which the next `0x007C` handed straight back.
#[test]
fn a_skill_the_server_grants_nothing_for_still_costs_what_the_table_says() {
    let Some((_store, mut s, _id)) = adv_session(210, &[(2_101_001, 1)], &[], 0) else { return };
    let out = s.on_skill_use(&skill_use_body(2_101_001, 1));
    assert_eq!(mp_of(&s), 1000 - 55, "Teleport L1 mpCon 55");
    assert_eq!(count_of(&out, net::stats::STAT_CHANGED), 1, "the bar is told");
    assert_eq!(count_of(&out, net::buff::TEMPORARY_STAT_SET), 0, "nothing to grant");
    assert_eq!(count_of(&out, net::notice::CHAT_NOTICE), 0, "and no chat line on every Teleport");
}

/// **Spell Booster takes its Magic Rock, and is refused before anything is spent without one.**
#[test]
fn spell_booster_takes_a_magic_rock_and_is_refused_without_one() {
    let Some((_store, mut s, id)) = adv_session(211, &[(2_111_005, 1)], &[(4_006_000, 2)], 0) else {
        return;
    };
    let out = s.on_skill_use(&skill_use_body(2_111_005, 1));
    let (bits, entries) = stat_set(first_stat_set(&out).expect("Spell Booster grants bit 109"));
    assert_eq!(bits, vec![net::jobbuffs::CTS_SPELL_BOOSTER]);
    assert_eq!(entries[0].0, -1, "x = -1 at level 1");
    assert_eq!(count_of(&out, net::inventory::INVENTORY_OPERATION), 1, "the rock stack is told");
    assert!(s.has_items(id, store::InventoryType::Etc, 4_006_000, 1), "one of two left");
    assert!(!s.has_items(id, store::InventoryType::Etc, 4_006_000, 2));
    assert_eq!(mp_of(&s), 1000 - 60);

    let Some((_store, mut s, _id)) = adv_session(211, &[(2_111_005, 1)], &[], 0) else { return };
    let out = s.on_skill_use(&skill_use_body(2_111_005, 1));
    assert!(first_stat_set(&out).is_none(), "no rock, no buff");
    assert_eq!(mp_of(&s), 1000, "and nothing spent");
    assert_eq!(count_of(&out, net::notice::CHAT_NOTICE), 1, "and it says why");
}

/// **Hyper Body raises the server's own HP ceiling while it is held**, the same lesson Max HP
/// Increase taught on 2026-09-06 with a different lifetime.
#[test]
fn hyper_body_raises_the_servers_hp_ceiling_only_while_held() {
    let Some((_store, mut s, _id)) = adv_session(131, &[(1_311_005, 1)], &[], 0) else { return };
    let chr = s.claimed_character().unwrap();
    assert_eq!(s.pools(&chr).max_hp, 1000, "the control");
    let out = s.on_skill_use(&skill_use_body(1_311_005, 1));
    let (bits, entries) = stat_set(first_stat_set(&out).expect("0x007D"));
    assert_eq!(bits, vec![net::jobbuffs::CTS_MAX_HP]);
    assert_eq!(entries[0].0, 10, "indieMhpR 10 = +10%");
    assert_eq!(s.pools(&chr).max_hp, 1100, "the server's ceiling follows the icon");
    let out = s.buff_tick(100_000);
    assert_eq!(count_of(&out, net::buff::TEMPORARY_STAT_RESET), 1);
    assert_eq!(s.pools(&chr).max_hp, 1000, "and comes back down with it");
}

#[test]
fn bless_grants_accuracy_and_avoidability_together() {
    let Some((_store, mut s, _id)) = adv_session(230, &[(2_301_003, 1)], &[], 0) else { return };
    let out = s.on_skill_use(&skill_use_body(2_301_003, 1));
    let (bits, entries) = stat_set(first_stat_set(&out).expect("0x007D"));
    assert_eq!(bits, vec![net::jobbuffs::CTS_ACCURACY, net::jobbuffs::CTS_AVOIDABILITY]);
    assert_eq!((entries[0].0, entries[1].0), (1, 1), "indieAcc 1, indieEva 1 at level 1");
    assert_eq!(entries[0].2, 100_000);
    assert_eq!(entries[0].2, entries[1].2, "one cast, one duration");
}

/// **Soul Arrow stops every arrow**, plain shot and skill alike.
#[test]
fn soul_arrow_stops_the_arrows_for_every_shot() {
    let Some((store, mut s, id)) =
        first_job_with(310, BOW, &[3_101_003, 3_001_001], BOW_ARROWS, 50, 1000)
    else {
        return;
    };
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 0));
    assert_eq!(arrows_in_slot_1(&store, id), 49, "the control: a plain shot takes one");
    let out = s.on_skill_use(&skill_use_body(3_101_003, 1));
    assert!(first_stat_set(&out).is_some(), "Soul Arrow grants bit 104: {out:?}");
    assert!(s.holds(net::jobbuffs::CTS_SOUL_ARROW));
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 0));
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 3_001_001));
    assert_eq!(arrows_in_slot_1(&store, id), 49, "nothing taken while it is held");
}

/// **Outside the first-job book the row itself decides the arrow count.** Strafe fires 3 and
/// names no consume column; Arrow Rain says 8; the hidden Arrow Bomb hit says none; Avenger
/// says 4 for a single throw.
#[test]
fn third_job_shots_take_what_their_own_row_says() {
    let Some((store, mut s, id)) =
        first_job_with(311, BOW, &[3_111_003, 3_111_002, 3_101_005], BOW_ARROWS, 50, 1000)
    else {
        return;
    };
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 3_111_003));
    assert_eq!(arrows_in_slot_1(&store, id), 47, "Strafe L1 fires 3 - one per projectile");
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 3_111_002));
    assert_eq!(arrows_in_slot_1(&store, id), 39, "Arrow Rain L1 bulletConsume 8");
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 3_101_005));
    assert_eq!(arrows_in_slot_1(&store, id), 39, "noBulletConsume");

    let Some((store, mut s, id)) = first_job_with(411, CLAW, &[4_111_004], SUBI, 50, 800) else {
        return;
    };
    s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 4_111_004));
    assert_eq!(arrows_in_slot_1(&store, id), 46, "Avenger: bulletConsume 4 for one throw");
}

/// **Element Amplification raises every MP cost but its own.** Fire Arrow L1 is 14; at 120%
/// that is 16, floored.
#[test]
fn element_amplification_raises_every_mp_cost_but_its_own() {
    let Some((_store, mut s, _id)) = adv_session(211, &[(2_111_000, 1), (2_101_003, 1)], &[], 0) else {
        return;
    };
    let out = s.on_skill_use(&skill_use_body(2_111_000, 1));
    assert!(first_stat_set(&out).is_some(), "the toggle is granted: {out:?}");
    assert_eq!(mp_of(&s), 1000 - 25, "its own cast is not amplified");
    s.handle(&swing_packet(net::combat::USER_MAGIC_ATTACK, 2_101_003));
    assert_eq!(mp_of(&s), 1000 - 25 - 16, "14 at 120% = 16");
}

#[test]
fn holy_symbol_adds_its_percent_to_a_kills_experience() {
    let Some((_store, mut s, _id)) = adv_session_with(231, &[(2_311_002, 1)], &[], 0, |c| {
        c.mob_exp.insert(2, 100);
    }) else {
        return;
    };
    assert_eq!(s.exp_for_kill(2).0, 100, "the control");
    let out = s.on_skill_use(&skill_use_body(2_311_002, 1));
    assert!(first_stat_set(&out).is_some(), "{out:?}");
    assert_eq!(s.exp_for_kill(2).0, 105, "x = 5 at level 1");
    assert!(s.exp_for_kill(2).1.contains("Holy Symbol"), "{}", s.exp_for_kill(2).1);
}

/// **Dragon Blood** is the toggle's flag plus its `indiePad`, and a drain on its own clock
/// that floors at 1.
#[test]
fn dragon_blood_drains_on_its_own_clock_and_never_kills() {
    let Some((store, mut s, _id)) = adv_session(131, &[(1_311_004, 1)], &[], 0) else { return };
    let out = s.on_skill_use(&skill_use_body(1_311_004, 1));
    let (bits, entries) = stat_set(first_stat_set(&out).expect("0x007D"));
    assert_eq!(bits, vec![net::jobbuffs::CTS_WEAPON_ATTACK, net::jobbuffs::CTS_DRAGON_BLOOD]);
    assert_eq!(entries[0].0, 30, "indiePad 30");
    assert_eq!(entries[1].0, 1, "the flag carries the level");
    assert_eq!(entries[0].2, 0, "a toggle sends no duration");
    assert_eq!(mp_of(&s), 1000 - 50);
    assert!(s.dragon_blood_tick(2_999).is_empty(), "y = 3 s");
    let out = s.dragon_blood_tick(3_000);
    assert_eq!(count_of(&out, net::stats::STAT_CHANGED), 1);
    assert_eq!(hp_of(&s), 1000 - 40, "x = 40");
    assert!(s.dragon_blood_tick(5_999).is_empty());
    let mut chr = s.claimed_character().unwrap();
    chr.hp = 20;
    store.save_character_progress(&chr).unwrap();
    s.dragon_blood_tick(6_000);
    assert_eq!(hp_of(&s), 1, "floored at 1 - the toggle never kills its holder");
    assert!(s.dragon_blood_tick(9_000).is_empty(), "and nothing left to drain");
    s.buffs.clear();
    assert!(s.dragon_blood_tick(12_000).is_empty(), "a cancel stops the clock");
}

/// **Heal** restores `x`% of the drawn ceiling, capped there, and Bless's `x` adds to the rate.
#[test]
fn heal_restores_a_percent_of_the_ceiling_and_bless_raises_the_percent() {
    let Some((store, mut s, _id)) = adv_session(230, &[(2_301_001, 1), (2_301_003, 1)], &[], 0) else {
        return;
    };
    let mut chr = s.claimed_character().unwrap();
    chr.hp = 100;
    store.save_character_progress(&chr).unwrap();
    let out = s.on_skill_use(&skill_use_body(2_301_001, 1));
    assert_eq!(hp_of(&s), 500, "x = 40% of 1000, on top of 100");
    assert_eq!(mp_of(&s), 1000 - 12);
    assert_eq!(count_of(&out, net::stats::USER_EFFECT_LOCAL), 1, "the blue number");
    s.on_skill_use(&skill_use_body(2_301_001, 1));
    s.on_skill_use(&skill_use_body(2_301_001, 1));
    assert_eq!(hp_of(&s), 1000, "capped at the ceiling");
    s.on_skill_use(&skill_use_body(2_301_003, 1));
    let mut chr = s.claimed_character().unwrap();
    chr.hp = 100;
    store.save_character_progress(&chr).unwrap();
    s.on_skill_use(&skill_use_body(2_301_001, 1));
    assert_eq!(hp_of(&s), 510, "Bless L1 adds 1%: 41% of 1000");
}

// ---- the four that need something to hit -------------------------------------------

/// A channel with mob 2002 (template 2) at `hp`, the skill table loaded, template 2 given
/// `mob_max_mp`, for the effects that hang off a landed swing.
fn adv_channel(hp: u64, mob_max_mp: u32) -> Option<(Arc<Store>, Arc<Config>, Arc<crate::fields::Fields>, i64)> {
    let table = std::path::Path::new("../../gm-handbook/skills.txt");
    if !table.exists() {
        return None;
    }
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account = store.create_account("maplecw", "correct horse battery").unwrap();
    let mut mobs = std::collections::HashMap::new();
    mobs.insert(SHARED_MAP, vec![net::mob::FieldMob::new(2002, 2, 400, 395, 1, hp)]);
    let mut mob_templates = std::collections::HashMap::new();
    mob_templates.insert(
        2u32,
        crate::config::MobTemplate { max_hp: 30, max_mp: mob_max_mp, ..Default::default() },
    );
    let config = Arc::new(Config {
        send_mobs: true,
        mobs,
        mob_templates,
        firstjob: crate::firstjob::CombatTable::load(table),
        ..Config::default()
    });
    let fields = Arc::new(crate::fields::Fields::new());
    fields.seed(crate::fields::FieldKey::world(SHARED_MAP), &config, 0);
    fields.due_respawns(crate::fields::FieldKey::world(SHARED_MAP), &config, 999_999);
    Some((store, config, fields, account))
}

/// Join [`adv_channel`] as a level-70 `job` with `skills`, 1000/1000, standing on the map.
fn adv_join(
    store: &Arc<Store>,
    config: &Arc<Config>,
    fields: &Arc<crate::fields::Fields>,
    account: i64,
    job: u16,
    skills: &[(u32, u32)],
) -> (Session, u32) {
    let chr = net::opcode::Character { name: "Advanced".to_string(), map_id: SHARED_MAP, ..Default::default() };
    let mut made = store.create_character(account, 0, &chr).unwrap();
    made.job = job;
    made.level = 70;
    made.hp = 1000;
    made.max_hp = 1000;
    made.mp = 1000;
    made.max_mp = 1000;
    store.save_character_progress(&made).unwrap();
    for (skill, level) in skills {
        store.set_skill_level(made.id, *skill, *level).unwrap();
    }
    store.create_migration(account, made.id, 0, 0).unwrap();
    let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
    s.claim_for_character(made.id);
    s.on_field_entered();
    let _ = s.handle(&NO_PACKET);
    (s, made.id)
}

/// The captured swing on mob 2002 with `skill` at level 1 patched in, behind `opcode`.
fn skilled_swing(opcode: u16, skill: u32) -> Vec<u8> {
    let mut body = unhex_body(MELEE_2002_FOR_19);
    body[2..6].copy_from_slice(&skill.to_le_bytes());
    body[6] = if skill == 0 { 0 } else { 1 };
    let mut p = opcode.to_le_bytes().to_vec();
    p.extend_from_slice(&body);
    p
}

/// **Combo Attack gains an orb per landed swing, up to `y`, and Coma spends them.** The
/// value starts at 1 and the client draws `value - 1` orbs; level 1's `y` is 3.
#[test]
fn combo_attack_gains_an_orb_per_landed_swing_up_to_y_and_a_finisher_spends_them() {
    let Some((store, config, fields, account)) = adv_channel(1_000_000, 0) else { return };
    let (mut s, _id) = adv_join(&store, &config, &fields, account, 111, &[(1_111_000, 1), (1_111_002, 1)]);
    let out = s.on_skill_use(&skill_use_body(1_111_000, 1));
    let (bits, entries) = stat_set(first_stat_set(&out).expect("Combo grants bit 107"));
    assert_eq!(bits, vec![net::jobbuffs::CTS_COMBO]);
    assert_eq!(entries[0].0, 1, "one orb-plus-one to start");
    assert_eq!(entries[0].2, 0, "a toggle");
    for expect in [2i16, 3, 4] {
        let out = s.handle(&melee_packet());
        let set = first_stat_set(&out).unwrap_or_else(|| panic!("orb {expect}: {out:?}"));
        assert_eq!(stat_set(set).1[0].0, expect);
    }
    let out = s.handle(&melee_packet());
    assert!(first_stat_set(&out).is_none(), "capped at y + 1 = 4: {out:?}");
    assert_eq!(s.held_value(net::jobbuffs::CTS_COMBO), 4);
    let out = s.handle(&skilled_swing(net::combat::USER_MELEE_ATTACK, 1_111_002));
    assert_eq!(stat_set(first_stat_set(&out).expect("Coma resets")).1[0].0, 1);
    assert_eq!(s.held_value(net::jobbuffs::CTS_COMBO), 1);
}

/// **Drain heals `x`% of the damage dealt when its `prop`% chance lands.** Level 30: 12%
/// and 15%; the captured swing does 19, so a proc is exactly 2.
#[test]
fn drain_heals_a_share_of_the_damage_when_its_chance_lands() {
    let Some((store, config, fields, account)) = adv_channel(1_000_000, 0) else { return };
    let (mut s, _id) = adv_join(&store, &config, &fields, account, 410, &[(4_101_002, 30)]);
    let mut chr = s.claimed_character().unwrap();
    chr.hp = 500;
    store.save_character_progress(&chr).unwrap();
    s.rng = Xorshift(0x9E37_79B9_7F4A_7C15);
    let mut procs = 0;
    for _ in 0..300 {
        let before = hp_of(&s);
        let out = s.handle(&skilled_swing(net::combat::USER_MELEE_ATTACK, 4_101_002));
        let after = hp_of(&s);
        if after > before {
            procs += 1;
            assert_eq!(after - before, 2, "19 * 15 / 100: {out:?}");
        }
    }
    assert!(procs > 0, "12% over 300 swings and not one proc - the roll is not running");
    assert!(procs < 300, "and not every swing - prop is 12, not 100");
}

/// **MP Eater absorbs `x`% of the mob's max MP on a landed magic hit, then waits `cooltime`.**
/// Level 20: 50% chance, 50% of the mob's 200.
#[test]
fn mp_eater_absorbs_a_share_of_the_mobs_max_mp_and_then_waits_five_seconds() {
    let Some((store, config, fields, account)) = adv_channel(1_000_000, 200) else { return };
    let (mut s, _id) = adv_join(&store, &config, &fields, account, 210, &[(2_100_000, 20)]);
    let mut chr = s.claimed_character().unwrap();
    chr.mp = 100;
    store.save_character_progress(&chr).unwrap();
    s.rng = Xorshift(42);
    let mut first = None;
    for i in 0..40u64 {
        s.clock_ms = i * 100;
        let before = mp_of(&s);
        s.handle(&skilled_swing(net::combat::USER_MAGIC_ATTACK, 0));
        if mp_of(&s) > before {
            assert_eq!(mp_of(&s) - before, 100, "50% of the mob's 200");
            first = Some(i);
            break;
        }
    }
    let first = first.expect("50% over 40 swings and not one proc");
    for i in first + 1..first + 50 {
        s.clock_ms = i * 100;
        let before = mp_of(&s);
        s.handle(&skilled_swing(net::combat::USER_MAGIC_ATTACK, 0));
        assert_eq!(mp_of(&s), before, "inside the 5 s cooltime at {} ms", i * 100);
    }
    // A melee swing never feeds it, whatever the dice say.
    s.clock_ms = 100_000;
    let before = mp_of(&s);
    for _ in 0..40 {
        s.handle(&melee_packet());
    }
    assert_eq!(mp_of(&s), before, "melee is not a magic attack");
}

/// The Drake hit from `magic_guard_splits_incoming_damage_between_hp_and_mp`, with the damage
/// set to `damage` and the mob object id to `mob`. Offsets are `net::userhit`'s own
/// (`DAMAGE_AT 8`, `MOB_OBJECT_ID_AT 46`); the parse below is what pins them. A claim above
/// the floor is the CLIENT's own arithmetic and the server applies it verbatim, which is what
/// makes the guard arithmetic below exact rather than a window.
fn drake_hit(damage: u32, mob: u32) -> Vec<u8> {
    let mut body = hex("00000000ffffffff0100000002002100431e140f0000000000000000000001000000010000000100000001000000d3070000d307000001000000000000000000000000000000000000de0100008b010000000000000000000000000000ffffffff00000000ffffffff000000000000000002000000000000000000000000000000000000000100000000000000000000000000");
    body[8..12].copy_from_slice(&damage.to_le_bytes());
    body[46..50].copy_from_slice(&mob.to_le_bytes());
    let hit = net::userhit::parse_user_hit(&body).expect("the fixture parses");
    assert_eq!(hit.damage, damage, "DAMAGE_AT");
    assert_eq!(hit.mob_object_id, mob, "MOB_OBJECT_ID_AT");
    body
}

/// **Invincible and Meso Guard take their shares off a hit, in that order, before Magic
/// Guard.** Level 1: Invincible 10%; Meso Guard blocks 30% at 50% of the blocked amount in
/// mesos, and blocks nothing when the purse cannot pay.
#[test]
fn invincible_and_meso_guard_take_their_shares_off_a_hit() {
    let Some((_st, mut s, _id)) = adv_session(230, &[], &[], 0) else { return };
    s.on_user_hit(&drake_hit(200, 1));
    assert_eq!(hp_of(&s), 800, "the control: the client's 200 is applied verbatim");

    let Some((_st, mut s, _id)) = adv_session(230, &[(2_301_002, 1)], &[], 0) else { return };
    s.on_skill_use(&skill_use_body(2_301_002, 1));
    assert!(s.holds(net::jobbuffs::CTS_INVINCIBLE));
    s.on_user_hit(&drake_hit(200, 1));
    assert_eq!(hp_of(&s), 1000 - 180, "Invincible L1: 10% ignored");

    let Some((store, mut s, id)) = adv_session(421, &[(4_211_000, 1)], &[], 10_000) else { return };
    s.on_skill_use(&skill_use_body(4_211_000, 1));
    assert!(s.holds(net::jobbuffs::CTS_MESO_GUARD));
    let out = s.on_user_hit(&drake_hit(200, 1));
    assert_eq!(hp_of(&s), 1000 - 140, "30% of 200 blocked");
    assert_eq!(store.mesos(id).unwrap(), 10_000 - 30, "at 50% of the 60 blocked");
    assert_eq!(count_of(&out, net::stats::STAT_CHANGED), 1, "HP and mesos in one packet");

    let Some((store, mut s, id)) = adv_session(421, &[(4_211_000, 1)], &[], 20) else { return };
    s.on_skill_use(&skill_use_body(4_211_000, 1));
    s.on_user_hit(&drake_hit(200, 1));
    assert_eq!(hp_of(&s), 800, "20 mesos cannot pay 30: nothing blocked");
    assert_eq!(store.mesos(id).unwrap(), 20, "and nothing charged");
}

/// **Power Guard sends its share back to the mob that hit us**, through the same body a
/// swing uses. Level 1: 20%.
#[test]
fn power_guard_reflects_its_share_onto_the_mob() {
    let Some((store, config, fields, account)) = adv_channel(500, 0) else { return };
    let (mut s, _id) = adv_join(&store, &config, &fields, account, 121, &[(1_211_005, 1)]);
    s.on_skill_use(&skill_use_body(1_211_005, 1));
    assert!(s.holds(net::jobbuffs::CTS_POWER_GUARD));
    assert_eq!(fields.mob_hp(crate::fields::FieldKey::world(SHARED_MAP), 2002), Some(500), "the control");
    let out = s.on_user_hit(&drake_hit(200, 2002));
    assert_eq!(hp_of(&s), 1000 - 160, "20% of the hit never lands");
    assert_eq!(fields.mob_hp(crate::fields::FieldKey::world(SHARED_MAP), 2002), Some(460), "and the 40 goes to the mob");
    assert!(
        out.iter().any(|r| r.what.starts_with("mob 2002 took 40")),
        "the mob's bar is told through deal_to_mob: {out:?}"
    );
    // A hit from a mob that is not on this field reflects onto nothing and is otherwise the
    // same hit - the reflected share is still not taken from the player.
    let before = hp_of(&s);
    s.on_user_hit(&drake_hit(200, 7777));
    assert_eq!(hp_of(&s), before - 160);
    assert_eq!(fields.mob_hp(crate::fields::FieldKey::world(SHARED_MAP), 2002), Some(460));
}

// ---------------------------------------------------------------------------------------
// 0x0143, the meso drop. See crates/world/src/mesodrop.rs and crates/net/src/dropmoney.rs.
// ---------------------------------------------------------------------------------------

/// **Mesos actually leave the character and land on the floor.** 2026-09-09.
///
/// The owner: *"I still cannot drop mesos."* They were right - this handler used to refuse every
/// drop. It asserts all three effects rather than one, which is `CLAUDE.md`'s rule from the
/// quest turn-in that counted fanfares while the experience doubled beside it: the balance
/// moved, an object reached the floor, and the reply carries the byte that frees the UI.
#[test]
fn a_meso_drop_leaves_the_character_and_reaches_the_floor() {
    let (mut s, store, id) = claimed_session();
    store.set_mesos(id, 5_000).unwrap();
    s.last_position = Some((520, 395));

    let out = s.on_drop_money(&net::dropmoney::drop_money_request(0x101b_5775, 10));

    // 1. THE BALANCE MOVED, and by exactly the amount asked for.
    assert_eq!(store.mesos(id).unwrap(), 4_990, "10 mesos left the character");
    // 2. SOMETHING REACHED THE FLOOR.
    let enter = out
        .iter()
        .find(|r| r.opcode == net::drops::DROP_ENTER_FIELD)
        .expect("a meso drop must put an object on the ground");
    assert_eq!(enter.body[0], net::drops::DROP_TYPE_MESO, "it is money, not an item");
    // 3. THE UI IS FREED, and the new balance rides the same packet.
    assert_eq!(out[0].opcode, net::combat::STAT_CHANGED, "the unlock leads");
    assert_eq!(out[0].body[0], 1, "bExclRequestSent");
    assert!(out[0].what.contains("balance now 4990"), "{}", out[0].what);
    // A meso drop touches no bag slot, so it must not answer with a bag packet.
    assert!(!out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION));
}

/// **Consolidate Item merges the stacks and slides everything up, and every step is a
/// `0x0070` the client has drawn before.** The owner, 2026-09-18, clicking it on the Use tab:
/// *"This button should make sure that all items that can be stacked without violating their
/// max stack size should be done."* and *"Consolidate items should also additionally move all
/// items to take the first available slots in the inventory. Such as that blue potion should
/// be consolidated upwards to be below the red potion."*
///
/// Their screen, plus a second red stack: red 54 in slot 1, orange 21 in 2, red 60 in 5, blue
/// 100 in 11. After: red 100, orange 21, red 14, blue 100 in slots 1..4. The wire is checked
/// packet by packet - a mode 1 for each changed count, then the two slides as bag-to-bag
/// mode 2s in ascending destination order - and the rows agree with it. Every packet leads
/// with `bExclRequestSent = 1`, because `0x0105` latches. Dispatched through `handle`, so the
/// arm is covered and the request can no longer fall to the generic unlock.
#[test]
fn consolidate_item_merges_the_stacks_then_slides_them_up_and_answers_with_0x0070s() {
    let (mut s, store, id) = claimed_session();
    let usable = store::InventoryType::Use;
    for (slot, item) in [
        (1u16, store::Item::bundle(2_000_000, 54)),
        (2, store::Item::bundle(2_010_000, 21)),
        (5, store::Item::bundle(2_000_000, 60)),
        (11, store::Item::bundle(2_000_002, 100)),
    ] {
        store.set_inventory_slot(id, usable, slot, &item).unwrap();
    }
    let mut packet = net::inventory::CLIENT_GATHER_ITEMS.to_le_bytes().to_vec();
    packet.extend_from_slice(&[0x3a, 0x1b, 0x27, 0x00, usable.as_u8()]); // tick, invType

    let out = s.handle(&packet);
    let ops: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::inventory::INVENTORY_OPERATION).collect();
    assert_eq!(ops.len(), out.len(), "nothing but 0x0070s: {out:?}");
    assert!(ops.iter().all(|r| r.body[0] == 1), "every one clears the latch");
    // mode 1: slot 1 -> 100, slot 5 -> 14; then mode 2: 5 -> 3, 11 -> 4.
    let entries: Vec<(u8, i16, i16)> = ops
        .iter()
        .map(|r| {
            let mode = r.body[7];
            let pos = i16::from_le_bytes([r.body[9], r.body[10]]);
            let tail = i16::from_le_bytes([r.body[11], r.body[12]]);
            (mode, pos, tail)
        })
        .collect();
    assert_eq!(
        entries,
        vec![
            (net::inventory::MODE_QUANTITY, 1, 100),
            (net::inventory::MODE_QUANTITY, 5, 14),
            (net::inventory::MODE_MOVE, 5, 3),
            (net::inventory::MODE_MOVE, 11, 4),
        ],
        "{:?}",
        ops.iter().map(|r| &r.what).collect::<Vec<_>>()
    );
    assert!(ops.iter().all(|r| r.body[8] == usable.as_u8()), "all on the Use tab");
    assert!(ops[2].body.len() == 13 && ops[3].body.len() == 13, "bag-to-bag: header 7 + entry 6, no avatar tail");
    let rows: Vec<(u16, u32, u16)> = store
        .bag_items(id, usable)
        .unwrap()
        .into_iter()
        .map(|i| (i.slot, i.item.item_id, i.item.kind.quantity()))
        .collect();
    assert_eq!(rows, vec![(1, 2_000_000, 100), (2, 2_010_000, 21), (3, 2_000_000, 14), (4, 2_000_002, 100)]);

    // A second click finds nothing to do and still answers - with the nCount-0 unlock.
    let again = s.handle(&packet);
    assert_eq!(again.len(), 1, "{again:?}");
    assert_eq!(again[0].opcode, net::inventory::INVENTORY_OPERATION);
    assert_eq!(again[0].body.len(), net::inventory::INVENTORY_REJECTED_LEN, "nCount 0");
    assert_eq!(again[0].body[0], 1, "and the latch still clears");
}

/// **Sort Items: the consolidate, then the swaps that put the biggest stack first and break
/// ties by name - each one a mode-2 onto an occupied slot, which is what a drag-swap is
/// answered with.** The owner, 2026-09-18: *"Sort Items should sort by quantity, then name."*
/// `Config::default()` has no item names, so every name is empty and the tie-break falls to
/// the item id; the store test covers the names. Through `handle`, so the arm is covered.
#[test]
fn sort_items_consolidates_then_swaps_the_tab_into_order_and_answers_with_0x0070s() {
    let (mut s, store, id) = claimed_session();
    let usable = store::InventoryType::Use;
    for (slot, item) in [
        (1u16, store::Item::bundle(2_000_000, 54)),
        (2, store::Item::bundle(2_010_000, 21)),
        (5, store::Item::bundle(2_000_000, 60)),
        (11, store::Item::bundle(2_000_002, 100)),
    ] {
        store.set_inventory_slot(id, usable, slot, &item).unwrap();
    }
    let mut packet = net::inventory::CLIENT_SORT_ITEMS.to_le_bytes().to_vec();
    packet.extend_from_slice(&[0x5c, 0xc5, 0x4b, 0x00, usable.as_u8()]);

    let out = s.handle(&packet);
    assert!(out.iter().all(|r| r.opcode == net::inventory::INVENTORY_OPERATION && r.body[0] == 1), "{out:?}");
    let entries: Vec<(u8, i16, i16)> = out
        .iter()
        .map(|r| (r.body[7], i16::from_le_bytes([r.body[9], r.body[10]]), i16::from_le_bytes([r.body[11], r.body[12]])))
        .collect();
    // The consolidate's four, then the swaps: after the slide the tab is red 100, orange 21,
    // red 14, blue 100; wanted is red 100 (id 2000000 before 2000002 with no names), blue
    // 100, orange 21, red 14 - one swap of slots 3 and 4... then 2 and 3.
    assert_eq!(entries[..4], [(1, 1, 100), (1, 5, 14), (2, 5, 3), (2, 11, 4)]);
    assert!(entries[4..].iter().all(|e| e.0 == net::inventory::MODE_MOVE), "{entries:?}");
    let rows: Vec<(u16, u32, u16)> = store
        .bag_items(id, usable)
        .unwrap()
        .into_iter()
        .map(|i| (i.slot, i.item.item_id, i.item.kind.quantity()))
        .collect();
    assert_eq!(rows, vec![(1, 2_000_000, 100), (2, 2_000_002, 100), (3, 2_010_000, 21), (4, 2_000_000, 14)]);

    // Sorted already: the nCount-0 unlock and nothing else.
    let again = s.handle(&packet);
    assert_eq!(again.len(), 1, "{again:?}");
    assert_eq!(again[0].body.len(), net::inventory::INVENTORY_REJECTED_LEN);
    assert_eq!(again[0].body[0], 1);
}

/// **A stack dragged onto a stack of the same item fills it first, and is drawn that way;
/// a full destination swaps; part of a stack into an empty slot is an ADD.** The owner,
/// 2026-09-18: *"it should try to fill the stack first (any remaining after the full stack
/// will remain at the original position), if the resulting stack is already full, then it
/// will carry out the swap slots procedure."*
///
/// The store already merged; the reply was one mode-2, which the client draws as an
/// unconditional exchange - so the screen swapped while the rows merged. Now: 40 onto 70 at
/// cap 100 answers two mode-1s (70 -> 100, 40 -> 10); the same drag again, destination full,
/// answers one mode-2 (the swap); 5 onto 5 answers a mode-1 and a mode-3; and 4 of 10 into an
/// empty slot answers a mode-1 (6 stay) and a mode-0 ADD of the 4. The rows agree with each
/// reply, and every reply leads with the latch byte.
#[test]
fn a_stack_dropped_on_its_own_kind_fills_it_first_then_swaps_when_full() {
    let (mut s, store, id) = claimed_session();
    let usable = store::InventoryType::Use;
    let qty = |slot: u16| store.inventory_slot(id, usable, slot).unwrap().map(|i| i.kind.quantity());
    let entries = |out: &[Reply]| -> Vec<(u8, i16, i16, usize)> {
        out.iter()
            .map(|r| {
                assert_eq!(r.opcode, net::inventory::INVENTORY_OPERATION, "{r:?}");
                assert_eq!(r.body[0], 1, "bExclRequestSent");
                let tail = if r.body.len() >= 13 { i16::from_le_bytes([r.body[11], r.body[12]]) } else { -1 };
                (r.body[7], i16::from_le_bytes([r.body[9], r.body[10]]), tail, r.body.len())
            })
            .collect()
    };
    store.set_inventory_slot(id, usable, 1, &store::Item::bundle(2_000_000, 40)).unwrap();
    store.set_inventory_slot(id, usable, 2, &store::Item::bundle(2_000_000, 70)).unwrap();

    // 1. Fill first: 40 onto 70 -> 100 in slot 2, 10 left in slot 1.
    let out = s.on_inventory_move(&inventory_move(2, 1, 2, -1));
    assert_eq!(
        entries(&out),
        vec![(net::inventory::MODE_QUANTITY, 2, 100, 13), (net::inventory::MODE_QUANTITY, 1, 10, 13)],
        "{:?}",
        out.iter().map(|r| &r.what).collect::<Vec<_>>()
    );
    assert_eq!((qty(1), qty(2)), (Some(10), Some(100)));

    // 2. The destination is full now: the same drag swaps, one mode-2, both slots still held.
    let out = s.on_inventory_move(&inventory_move(2, 1, 2, -1));
    assert_eq!(entries(&out), vec![(net::inventory::MODE_MOVE, 1, 2, 13)]);
    assert_eq!((qty(1), qty(2)), (Some(100), Some(10)));

    // 3. A source poured out entirely: 10 onto a stack with room -> mode 1 + mode 3.
    let out = s.on_inventory_move(&inventory_move(2, 2, 1, -1));
    assert_eq!(entries(&out), vec![(net::inventory::MODE_MOVE, 2, 1, 13)], "100 is full, so this one swaps back first");
    store.set_inventory_slot(id, usable, 3, &store::Item::bundle(2_000_000, 5)).unwrap();
    let out = s.on_inventory_move(&inventory_move(2, 3, 1, -1)); // 5 onto the 10 in slot 1
    let e = entries(&out);
    assert_eq!(e[0], (net::inventory::MODE_QUANTITY, 1, 15, 13));
    assert_eq!((e[1].0, e[1].1, e[1].3), (net::inventory::MODE_REMOVE, 3, net::inventory::INVENTORY_REMOVE_LEN));
    assert_eq!((qty(1), qty(3)), (Some(15), None));

    // 4. Part of a stack into an EMPTY slot: 4 of the 15 -> slot 5. The source's new count,
    //    then an ADD of the new stack (the client has nothing in slot 5 to re-count).
    let out = s.on_inventory_move(&inventory_move(2, 1, 5, 4));
    assert_eq!(out.len(), 2, "{out:?}");
    assert_eq!(&entries(&out[..1])[0], &(net::inventory::MODE_QUANTITY, 1, 11, 13));
    assert_eq!(out[1].body[7], net::inventory::MODE_ADD, "{}", out[1].what);
    assert_eq!(i16::from_le_bytes([out[1].body[9], out[1].body[10]]), 5, "into slot 5");
    assert_eq!((qty(1), qty(5)), (Some(11), Some(4)));
}

/// **The fame arrows: the giver gets mode 0 with the new number, the target gets mode 5 and
/// their own stat, a second gift the same day gets mode 3, and the window shows the fame.**
/// The owner, 2026-09-18. The week rule and the calendar are the store's tests (`store::fame`);
/// this is the wire. **And the ITEM tab lists what they wear**: Tester2 puts on a hat and the
/// window's item count is 1 with one whole equip slot behind it.
#[test]
fn fame_reaches_both_players_once_a_day_and_the_window_lists_what_they_wear() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let (mut me, my_id) = join_channel(&store, &config, &fields, account, "Wisp");
    me.on_field_entered();
    let (mut them, their_id) = join_channel(&store, &config, &fields, account, "Tester2");
    them.on_field_entered();
    me.collect_mail();
    them.collect_mail();

    let mut packet = net::fame::CLIENT_GIVE_FAME.to_le_bytes().to_vec();
    packet.extend_from_slice(&net::fame::give_fame_request(their_id, true));
    let out = me.handle(&packet);
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!(out[0].opcode, net::fame::GIVE_FAME_RESULT);
    assert_eq!(out[0].body, net::fame::fame_given("Tester2", true, 1), "mode 0, Tester2, up, fame 1");
    assert_eq!(store.fame(their_id).unwrap(), Some(1));

    let mail = them.tick(1_000);
    let received: Vec<&Reply> = mail.iter().filter(|r| r.opcode == net::fame::GIVE_FAME_RESULT).collect();
    assert_eq!(received.len(), 1, "{mail:?}");
    assert_eq!(received[0].body, net::fame::fame_received("Wisp", true), "mode 5: 'the owner' has raised your fame");
    let stat: Vec<&Reply> = mail.iter().filter(|r| r.opcode == net::combat::STAT_CHANGED && r.what.contains("fame = 1")).collect();
    assert_eq!(stat.len(), 1, "the target's own fame stat moves: {mail:?}");

    // The same day, anyone: refused with the client's "not anymore for today" message.
    let mut again = net::fame::CLIENT_GIVE_FAME.to_le_bytes().to_vec();
    again.extend_from_slice(&net::fame::give_fame_request(their_id, false));
    let out = me.handle(&again);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].body, net::fame::fame_refused(net::fame::result::ALREADY_TODAY));
    assert_eq!(store.fame(their_id).unwrap(), Some(1), "a refusal moves nothing");
    // Yourself: the client refuses before sending; the server refuses too.
    let mut selfie = net::fame::CLIENT_GIVE_FAME.to_le_bytes().to_vec();
    selfie.extend_from_slice(&net::fame::give_fame_request(my_id, true));
    assert_eq!(me.handle(&selfie)[0].body, net::fame::fame_refused(net::fame::result::NO_SUCH_USER));
    // The other way is a different giver: Tester2 may fame the owner today.
    let mut back = net::fame::CLIENT_GIVE_FAME.to_le_bytes().to_vec();
    back.extend_from_slice(&net::fame::give_fame_request(my_id, false));
    assert_eq!(them.handle(&back)[0].body, net::fame::fame_given("Wisp", false, -1), "a defame goes below zero");

    // The window: fame 1, and once Tester2 wears a hat, the ITEM tab is hair, face, hat -
    // the look first, as equip slots under the look ids, then what is worn.
    store.set_character_look(their_id, Some(30_030), Some(20_000)).unwrap();
    store.set_inventory_slot(their_id, store::InventoryType::Equip, 1, &store::Item::equip(1_002_357)).unwrap();
    store.equip_from_bag(their_id, 1, 1).unwrap();
    let out: Vec<Reply> = me.handle(&character_info_request(their_id, false)).into_iter().filter(|r| r.opcode == net::charinfo::CHARACTER_INFO).collect();
    assert_eq!(out.len(), 1);
    let b = &out[0].body;
    assert_eq!(u32::from_le_bytes(b[25..29].try_into().unwrap()), 1, "fame 1 in the window");
    let plain = net::opcode::EquipStats::default();
    let entries: Vec<Vec<u8>> = [30_030, 20_000, 1_002_357].iter().map(|id| net::opcode::equipped_item(*id, &plain)).collect();
    let total: usize = entries.iter().map(|e| e.len()).sum();
    let base = net::charinfo::CHARACTER_INFO_BASE_LEN + "Tester2".len();
    assert_eq!(b.len(), base + total, "three whole equip slots behind the count");
    // The count sits 9 bytes before the end of the base: count, record count, flag.
    let count_at = base - 9;
    assert_eq!(u32::from_le_bytes(b[count_at..count_at + 4].try_into().unwrap()), 3, "hair, face, hat");
    let mut at = count_at + 4;
    for (what, e) in ["hair 30030", "face 20000", "the hat"].iter().zip(&entries) {
        assert_eq!(&b[at..at + e.len()], &e[..], "{what}, type byte first");
        at += e.len();
    }
    // Equips only, for a client without the rendered hair/face icons.
    let mut plain_config = (*me.config).clone();
    plain_config.charinfo_look_items = false;
    me.config = Arc::new(plain_config);
    let out: Vec<Reply> = me.handle(&character_info_request(their_id, false)).into_iter().filter(|r| r.opcode == net::charinfo::CHARACTER_INFO).collect();
    assert_eq!(u32::from_le_bytes(out[0].body[count_at..count_at + 4].try_into().unwrap()), 1, "--no-look-items: the hat alone");
}

/// **A negative amount must not credit the player.**
///
/// `0x0143`'s amount is a signed `i32` and the client's own check (`cmp rdi, rax / jle`)
/// passes a negative. Read as a `u32` it is four billion; handed to `add_mesos` as a
/// negative delta it would *pay* the dropper. This is the test that says it does neither.
#[test]
fn a_negative_meso_drop_is_refused_and_pays_nothing() {
    let (mut s, store, id) = claimed_session();
    store.set_mesos(id, 7).unwrap();
    s.last_position = Some((520, 395));

    for amount in [-1i32, i32::MIN, 0] {
        let out = s.on_drop_money(&net::dropmoney::drop_money_request(1, amount));
        assert_eq!(store.mesos(id).unwrap(), 7, "amount {amount} moved the balance");
        assert!(
            !out.iter().any(|r| r.opcode == net::drops::DROP_ENTER_FIELD),
            "amount {amount} reached the floor"
        );
        assert_eq!(out[0].body[0], 1, "amount {amount} left the UI latched");
    }
}

/// Dropping more than you hold is refused, and the balance is untouched.
#[test]
fn a_meso_drop_larger_than_the_balance_is_refused() {
    let (mut s, store, id) = claimed_session();
    store.set_mesos(id, 100).unwrap();
    s.last_position = Some((520, 395));

    let out = s.on_drop_money(&net::dropmoney::drop_money_request(1, 101));
    assert_eq!(store.mesos(id).unwrap(), 100);
    assert!(!out.iter().any(|r| r.opcode == net::drops::DROP_ENTER_FIELD));
    assert!(out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE), "and says why");

    // The control: one less is accepted, so the refusal above is the boundary and not a
    // handler that refuses everything.
    let ok = s.on_drop_money(&net::dropmoney::drop_money_request(1, 100));
    assert_eq!(store.mesos(id).unwrap(), 0, "the whole balance may be dropped");
    assert!(ok.iter().any(|r| r.opcode == net::drops::DROP_ENTER_FIELD));
}

/// **The client's own 10 000 cap, enforced on the packet.**
///
/// The owner, 2026-09-09, with a screenshot of the client's dialog: *"The client restricts dropping
/// of mesos to 10k, we should also mimic that on the server side."*
///
/// The dialog is a UI rule and `0x0143` is a packet. This field has already fooled this server
/// once - it is signed, and the client's own check passes a negative - so "the client would
/// never send that" is not a guarantee the server may rely on.
///
/// **The boundary is asserted from both sides.** A test that only refuses 10 001 would pass on
/// a handler that refuses everything, and one that only accepts 10 000 would pass on a handler
/// with no cap at all.
#[test]
fn a_meso_drop_over_the_clients_own_cap_is_refused() {
    let (mut s, store, id) = claimed_session();
    store.set_mesos(id, 50_000).unwrap();
    s.last_position = Some((520, 395));

    let cap = crate::mesodrop::MAX_DROP as i32;
    let out = s.on_drop_money(&net::dropmoney::drop_money_request(1, cap + 1));
    assert_eq!(store.mesos(id).unwrap(), 50_000, "nothing left the balance");
    assert!(!out.iter().any(|r| r.opcode == net::drops::DROP_ENTER_FIELD), "nor the floor");
    assert_eq!(out[0].body[0], 1, "and the UI must not be left latched");
    assert!(
        out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE),
        "a refusal nobody is told about is the failure this module exists for"
    );

    // The control: exactly the cap is accepted, so the refusal is a boundary and not a
    // handler that says no to everything.
    let ok = s.on_drop_money(&net::dropmoney::drop_money_request(1, cap));
    assert_eq!(store.mesos(id).unwrap(), 40_000, "10000 really left");
    assert!(ok.iter().any(|r| r.opcode == net::drops::DROP_ENTER_FIELD));
}

/// Every refusal still answers. An unanswered `0x0143` is the freeze this whole module
/// exists for, and a malformed body must not become a silent return.
#[test]
fn every_meso_refusal_still_clears_the_latch() {
    let (mut s, store, id) = claimed_session();
    store.set_mesos(id, 50).unwrap();

    // No position: the drop would be placed where it cannot be picked up.
    s.last_position = None;
    let no_pos = s.on_drop_money(&net::dropmoney::drop_money_request(1, 10));
    assert_eq!(no_pos[0].body[0], 1);
    assert_eq!(store.mesos(id).unwrap(), 50, "a refused drop costs nothing");

    // An unreadable body, answered before anyone has an opinion about the bytes.
    s.last_position = Some((520, 395));
    for junk in [vec![], vec![0u8; 3], vec![0u8; 40]] {
        let out = s.on_drop_money(&junk);
        assert_eq!(out[0].opcode, net::combat::STAT_CHANGED, "len {}", junk.len());
        assert_eq!(out[0].body[0], 1, "len {}", junk.len());
    }
    assert_eq!(store.mesos(id).unwrap(), 50);
}


/// The other half of the patch: the fall-through that must never let a latching opcode go
/// unanswered again. `0x0143` reaches it if the specific arm is ever removed, and `0x01FD`
/// and `0x02F6` - both seen unhandled in the archive - reach it today.
#[test]
fn the_latching_fall_through_would_have_caught_every_freeze_in_the_archive() {
    for op in [0x0143u16, 0x01FD, 0x02F6] {
        assert!(net::dropmoney::latches_the_exclusive_request(op), "{op:#06X}");
        let out = crate::mesodrop::unlock_unhandled_latching_request(op);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].body[0], 1);
    }
    // Every opcode the dispatch already answers specifically is matched by an earlier arm,
    // so this predicate being true for them changes nothing - but the ones it is FALSE for
    // are the control that says it is not simply "everything".
    for op in [0x00D9u16, 0x02FF, 0x0070, 0x00E7, 0x013F] {
        assert!(!net::dropmoney::latches_the_exclusive_request(op), "{op:#06X}");
    }
}

/// **A drop lands where the mob FINISHED its path, not where it started.**
///
/// The owner, 2026-09-08: *"the mob drops from a moving mob seems to be dropping from an awkward
/// location not related to the current mob location mid-movement."* The server stored the
/// `0x02FF` path HEAD as the mob's position, and the head is where the walk began - a median
/// of 41 px behind the mob, and more than 25 px behind it 62.8% of the time, measured over
/// 642 431 deduplicated reports (`crate::dropsite`, `tools/mobmove_lag.py`).
///
/// This is the only test that crosses every seam: the move report, the stored position, the
/// kill, and the bytes of the drop that reaches the field. Four numbers are in play and only
/// one is right - 100 is the spawn point, 424 the path head, 1500 the player, 456 the mob -
/// so each wrong answer is named in its own assertion rather than left to a bare equality.
#[test]
fn a_drop_lands_where_the_mob_finished_its_path_not_where_it_started() {
    let (store, config, fields, account) = shared_channel(1, 30);
    let drops = crate::droptables::DropTables::parse("2 | 4000001 | 100 | 1 | 1 | 9 | Shell\n");
    let config = Arc::new(Config { drops, ..(*config).clone() });
    let (mut controller, chr_id) = join_channel(&store, &config, &fields, account, "PathWalker");
    controller.on_field_entered();
    let _ = controller.handle(&NO_PACKET);

    let mut packet = net::mobmove::MOB_MOVE_REQUEST.to_le_bytes().to_vec();
    packet.extend_from_slice(&unhex_body(CAPTURED_MOB_MOVE_2000));
    let ack = controller.handle(&packet);
    assert_eq!(count_of(&ack, net::mobmove::MOB_CTRL_ACK), 1, "the report was believed");

    assert_eq!(fields.mobs_on(crate::fields::FieldKey::world(SHARED_MAP))[0].spawn.x, 100, "the spawn point, a decoy");
    assert_eq!(fields.mob_site(crate::fields::FieldKey::world(SHARED_MAP), 2000), Some((456, 395)));

    controller.last_position = Some((1500, 395));
    let out = controller.deal_to_mob(crate::fields::FieldKey::world(SHARED_MAP), 2000, 9_999, chr_id);
    assert!(
        out.iter().any(|r| r.opcode == net::drops::DROP_ENTER_FIELD),
        "the kill must actually have dropped something, or every number below is vacuous: {out:?}"
    );

    let xs: Vec<i16> =
        fields.with_drops(crate::fields::FieldKey::world(SHARED_MAP), |d| d.on_field(crate::fields::FieldKey::world(SHARED_MAP)).map(|x| x.x).collect());
    assert_eq!(xs.len(), 1);
    assert_eq!(
        xs[0], 456,
        "the end of the path - not 424 (the head), 100 (the spawn point) or 1500 (the player)"
    );

    // The arc's ORIGIN too. An item flying out of empty space 32 px behind the corpse is
    // exactly as wrong on screen as one landing there, and it is a separate field.
    let src = fields.with_drops(crate::fields::FieldKey::world(SHARED_MAP), |d| {
        d.on_field(crate::fields::FieldKey::world(SHARED_MAP)).map(|x| (x.source_x, x.source_y)).next().unwrap()
    });
    assert_eq!(src, (456, 395), "an arc starting behind the corpse is the bug on screen");
}

/// **The field clock goes out on a map that declares one, with UTC.** The owner,
/// 2026-09-09, in Ellinia Station: the clock sat at 00:00 because nothing ever sent
/// `0x01BC`. This pins the send, its shape - type 1, hour, minute, second - and that the
/// time is UTC - the owner, 2026-09-10: "this needs to read the UTC time" - rather than a
/// constant or the machine's zone.
#[test]
fn entering_a_map_with_a_clock_node_sends_the_local_time() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "TestCharD".to_string(), map_id: 40, ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.set_character_map(id, 40).unwrap();
    store.create_migration(account_id, id, 0, 0).unwrap();
    let config = Config {
        clocks: [40u32].into_iter().collect(),
        ..Config::default()
    };
    let mut s = Session::new(store.clone(), Arc::new(config));
    assert!(s.claim_for_character(id).contains("claimed the migration"));

    // Sample the wall clock on both sides of the call so a second boundary cannot fake a
    // mismatch; the body must agree with one of the two.
    let before = crate::serverclock::utc_hms();
    let out = s.on_field_entered();
    let after = crate::serverclock::utc_hms();

    let clocks: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::clock::FIELD_CLOCK).collect();
    assert_eq!(clocks.len(), 1, "exactly one clock per entry: {out:?}");
    let body = &clocks[0].body;
    assert_eq!(body.len(), net::clock::CLOCK_HMS_LEN);
    assert_eq!(body[0], net::clock::CLOCK_TYPE_HMS, "type 1 is the wall clock");
    let sent = (body[1], body[2], body[3]);
    assert!(
        sent == before || sent == after,
        "sent {sent:?}, but the wall clock read {before:?} before and {after:?} after"
    );
    assert!(clocks[0].what.contains("FieldClock"), "{}", clocks[0].what);
}

/// **The control: no clock node, no packet.** The client's type-1 arm fetches the widget
/// with no null check and the fetch throws on a map that built none, so an unconditional
/// send would be a crash on every ordinary map. A map missing from `Config::clocks` must
/// get nothing.
#[test]
fn entering_a_map_without_a_clock_node_sends_no_clock() {
    let (mut s, _store, _id) = gm_session();
    // `gm_session`'s config has an empty clock table, so whatever map the character is on
    // is one without a clock.
    let out = s.on_field_entered();
    assert!(
        !out.iter().any(|r| r.opcode == net::clock::FIELD_CLOCK),
        "a clock went out on a map with no clock node: {out:?}"
    );
}

/// A session standing in Ellinia with the station door in its portal table - the shape the
/// real config has after `load_portals` folds `world::scriptportals` in.
fn session_in_ellinia_with_the_station_door() -> (Session, u32) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character {
        name: "Cobalt".to_string(),
        map_id: 10_002_000,
        ..Default::default()
    };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.set_character_map(id, 10_002_000).unwrap();
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut portals = std::collections::HashMap::new();
    portals.insert((10_002_000u32, "in03".to_string()), (10_002_090u32, "out00".to_string()));
    let mut portal_index = std::collections::HashMap::new();
    portal_index.insert((10_002_090u32, "out00".to_string()), 2u8);
    let config = Config { portals, portal_index, ..Config::default() };
    let mut s = Session::new(store, Arc::new(config));
    assert!(s.claim_for_character(id).contains("claimed the migration"));
    (s, id)
}

/// **The owner's own press, byte for byte, walks through the door.** `world.log` 2026-09-10
/// 11:27:53.035: `0x014A`, body `000400696e30333a0303f4`. Yesterday this was logged UNKNOWN
/// and answered with nothing, with the destination sitting in the table the whole time.
#[test]
fn remys_script_portal_press_reaches_ellinia_station() {
    let (mut s, _id) = session_in_ellinia_with_the_station_door();
    let body = [0x00, 0x04, 0x00, b'i', b'n', b'0', b'3', 0x3a, 0x03, 0x03, 0xf4];
    let out = s.on_portal_script(&body);
    let warp = out
        .iter()
        .find(|r| r.opcode == net::opcode::SET_FIELD)
        .unwrap_or_else(|| panic!("no SetField for the station door: {out:?}"));
    assert!(
        warp.what.contains("map 10002090 portal 2"),
        "it must land on the station's out00, index 2: {}",
        warp.what
    );
    assert_eq!(
        out.iter().filter(|r| r.opcode == net::opcode::SET_FIELD).count(),
        1,
        "one warp, not a re-send plus a warp"
    );
}

/// **The control: a script portal the server cannot resolve is answered, and not with a
/// warp.** `rand_ola` and the PQ portals are real script portals with no destination here;
/// re-entering the map under the player would be worse than the unlock, and silence is
/// against the standing rule.
#[test]
fn an_unresolved_script_portal_is_answered_without_a_warp() {
    let (mut s, _id) = session_in_ellinia_with_the_station_door();
    let mut body = vec![0x00, 0x08, 0x00];
    body.extend_from_slice(b"rand_ola");
    body.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    let out = s.on_portal_script(&body);
    assert!(!out.is_empty(), "an unanswered request is the one thing this must never be");
    assert!(
        !out.iter().any(|r| r.opcode == net::opcode::SET_FIELD),
        "an unknown script portal must not warp anywhere: {out:?}"
    );
    // And a body too short to parse gets the same treatment rather than a panic.
    let out = s.on_portal_script(&[0x00, 0x04, 0x00, b'i']);
    assert!(!out.is_empty());
    assert!(!out.iter().any(|r| r.opcode == net::opcode::SET_FIELD));
}

/// **The walked door and the script portal resolve through ONE path.** The same table entry,
/// reached by `0x00D1` and by `0x014A`, must produce the same destination and arrival - if
/// the two handlers ever drift, a door works on foot and not by script, or the reverse.
#[test]
fn the_walked_door_and_the_script_portal_agree() {
    let (mut s, _id) = session_in_ellinia_with_the_station_door();
    // 0x00D1: 16 bytes of integrity block, u32 target = -1, u16 len, name, u16 x, u16 y.
    let mut walk = vec![0u8; 16];
    walk.extend_from_slice(&u32::MAX.to_le_bytes());
    walk.extend_from_slice(&4u16.to_le_bytes());
    walk.extend_from_slice(b"in03");
    walk.extend_from_slice(&[0x3a, 0x03, 0x03, 0xf4]);
    let by_walk = s.on_transfer_field(&walk);
    let (mut s2, _id) = session_in_ellinia_with_the_station_door();
    let by_script =
        s2.on_portal_script(&[0x00, 0x04, 0x00, b'i', b'n', b'0', b'3', 0x3a, 0x03, 0x03, 0xf4]);
    let pick = |out: &[Reply]| {
        out.iter()
            .find(|r| r.opcode == net::opcode::SET_FIELD)
            .map(|r| r.what.clone())
            .expect("a SetField")
    };
    let (a, b) = (pick(&by_walk), pick(&by_script));
    assert!(a.contains("map 10002090 portal 2"), "{a}");
    assert!(b.contains("map 10002090 portal 2"), "{b}");
}

/// **The gender gate, through a real move packet, for a normal slot AND a cash-equip slot.**
///
/// The owner, 2026-09-10: *"There needs to be a server side fix to prevent users from equipping
/// the opposite gendered equipment. This needs to happen for both normal item equipments
/// and cash equipments."* The gate has existed since 2026-09-09 but nothing drove it
/// through `0x0107`; this does, and it drives the cash-slot form (`dst <= -101`) too, because
/// the gate sits before any slot arithmetic and a regression that moved it below would
/// let the cash form through unnoticed.
#[test]
fn a_female_top_is_refused_on_a_male_character_in_both_slot_forms() {
    let (mut s, store, id) = gm_session();
    // TestCharD is gender 0 (male) - `Character::default()`, and every character in
    // maplecw.db. 1041001 is a FEMALE shirt in the client's own MakeCharInfo list.
    store.add_item(id, store::InventoryType::Equip, &store::Item::equip(1041001), 1).unwrap();
    for dst in [-5i16, -105] {
        let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, 1, dst, -1));
        assert_eq!(out.len(), 1, "always answered, once: {out:?}");
        assert_eq!(out[0].opcode, net::inventory::INVENTORY_OPERATION);
        assert!(out[0].what.contains("REFUSING"), "dst {dst}: {}", out[0].what);
        assert!(out[0].what.contains("not made for"), "dst {dst}: {}", out[0].what);
        assert_eq!(out[0].body[0], 1, "the refusal still clears the exclusive-request latch");
        assert!(store.equipped_items(id).unwrap().is_empty(), "nothing went on for dst {dst}");
    }
    // The control: the MALE top Cobalt wears is legal on a male, and goes on.
    store.add_item(id, store::InventoryType::Equip, &store::Item::equip(1040021), 1).unwrap();
    let slot = store
        .bag(id)
        .unwrap()
        .items
        .iter()
        .find(|i| i.item.item_id == 1040021)
        .map(|i| i.slot)
        .expect("the Blue Sergeant is in the bag");
    let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, slot as i16, -5, -1));
    assert!(!out[0].what.contains("REFUSING"), "{}", out[0].what);
    assert!(store.equipped_items(id).unwrap().iter().any(|e| e.item_id == 1040021 && e.slot == 5));
}

/// The `0x03E1 0x0A` body the client builds: sub-op, `u64` serial, `u32` item, `u8` tab,
/// `u16` destination slot - `research/cash-shop-actions.md` §3.2.
fn locker_to_bag(serial: u64, item_id: u32, tab: u8, slot: u16) -> Vec<u8> {
    let mut b = net::cashshop::CLIENT_CASH_SHOP_ACTION.to_le_bytes().to_vec();
    b.push(net::cashshop::ACTION_MOVE_LOCKER_TO_BAG);
    b.extend_from_slice(&serial.to_le_bytes());
    b.extend_from_slice(&item_id.to_le_bytes());
    b.push(tab);
    b.extend_from_slice(&slot.to_le_bytes());
    b
}

/// **A locker item moves into the Cash tab at the slot the client chose, and the reply is
/// the `0x19` the client reads as "place at N and forget the serial".** The owner, 2026-09-10:
/// *"The player can choose to move the coupon out of the Cash Inventory into the regular
/// inventory in the Cash Tab."* Refused by this server until today; never attempted in any
/// archived run.
#[test]
fn a_locker_item_moves_into_the_cash_tab_where_the_client_asked() {
    let (mut s, store, id) = cash_shop_session();
    let account = 1i64;
    let placed = store.put_cash_item(account, &store::Item::bundle(5680004, 1)).unwrap();
    let serial = (account as u64) << 32 | u64::from(placed.slot);
    let cash = store::InventoryType::Cash;

    let out = s.handle(&locker_to_bag(serial, 5680004, cash.as_u8(), 3));
    assert_eq!(out.len(), 1, "one reply and no wallet: {out:?}");
    assert_eq!(out[0].opcode, net::cashshop::CASH_SHOP_RESULT);
    assert_eq!(out[0].body[0], net::cashshop::RESULT_ITEM_GRANTED, "0x19, the MOVE reply");
    assert_eq!(out[0].body[1], 1, "bRelease must be non-zero or the shop stays blocked");
    assert_eq!(&out[0].body[2..4], &3u16.to_le_bytes(), "the slot the client picked, echoed");
    assert!(out[0].body.len() > 4 + 10, "an item body follows, then the trailing flag");
    assert_eq!(*out[0].body.last().unwrap(), 0, "bEffect");
    assert!(out[0].what.contains("MOVED"), "{}", out[0].what);
    // **The item body carries the serial in its own +0x38** - hasCashSN 1, then the u64.
    // The 0x19 handler erases `[item+0x38]` from the locker map, not the request's serial;
    // with the flag at 0 the coupon stayed drawn in the Cash Inventory (2026-09-11 03:47).
    let item = &out[0].body[4..out[0].body.len() - 1];
    assert_eq!(item[0], net::bag::BUNDLE_ITEM_TYPE);
    assert_eq!(&item[1..5], &5680004u32.to_le_bytes());
    assert_eq!(item[5], 1, "140303787  hasCashSN");
    assert_eq!(&item[6..14], &serial.to_le_bytes(), "14030379d  raw[8] -> +0x38: the locker serial");
    assert_eq!(item.len(), net::bag::BUNDLE_ITEM_LEN + 8, "a bundle grows by eight with the serial");

    // The database moved it: out of the locker, into the Cash tab at slot 3.
    assert!(store.cash_locker(account).unwrap().is_empty(), "the locker row is gone");
    let bag = store.bag_items(id, cash).unwrap();
    assert_eq!(bag.len(), 1);
    assert_eq!((bag[0].slot, bag[0].item.item_id), (3, 5680004));
}

/// The `0x03E1 0x1C` body: the sub-op and the locker serial, nothing else.
fn cash_delete(serial: u64) -> Vec<u8> {
    let mut b = net::cashshop::CLIENT_CASH_SHOP_ACTION.to_le_bytes().to_vec();
    b.push(net::cashshop::ACTION_DELETE);
    b.extend_from_slice(&serial.to_le_bytes());
    b
}

/// **Deleting a locker item removes it, says so, and leaves the shop usable.**
///
/// The owner, 2026-09-24: *"I just tried deleting an item in Cash Shop, but this is currently
/// unhandled."* The body is the one their client sent, `1c 01000000 01000000` - account 1,
/// locker slot 1.
#[test]
fn deleting_a_locker_item_removes_it_and_releases_the_shop() {
    let (mut s, store, _id) = cash_shop_session();
    let account = 1i64;
    let placed = store.put_cash_item(account, &store::Item::bundle(5680004, 1)).unwrap();
    let serial = (account as u64) << 32 | u64::from(placed.slot);
    assert_eq!(&cash_delete(serial)[2..], &[0x1c, 1, 0, 0, 0, 1, 0, 0, 0], "the captured body");

    let out = s.handle(&cash_delete(serial));
    // **Two replies, in this order.** 0x3C erases the item and prints "The cash item has been
    // deleted." but does NOT clear the in-flight latch; the wallet does. Without it the shop
    // would refuse every request after one delete.
    assert_eq!(out.len(), 2, "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert_eq!(out[0].opcode, net::cashshop::CASH_SHOP_RESULT);
    assert_eq!(out[0].body, net::cashshop::cash_item_deleted(serial), "0x3C + the serial echoed");
    assert_eq!(out[1].opcode, net::cashshop::CASH_SHOP_WALLET, "then the latch-clearing wallet");
    assert!(store.cash_locker(account).unwrap().is_empty(), "the row is really gone");

    // **The same serial again: nothing is there, so it is refused - with 0x3D, which clears
    // the latch and keeps anything else queued - and nothing else is touched.**
    store.put_cash_item(account, &store::Item::bundle(5070000, 1)).unwrap(); // lands in slot 1 again
    let other = s.handle(&cash_delete((2u64 << 32) | 1));
    assert_eq!(other[0].body[0], net::cashshop::RESULT_QUEUE_REFUSED, "another account's serial: {}", other[0].what);
    assert_eq!(store.cash_locker(account).unwrap().len(), 1, "and this account's item is untouched");
    let short = s.handle(&{
        let mut b = cash_delete(serial);
        b.pop();
        b
    });
    assert_eq!(short[0].body[0], net::cashshop::RESULT_QUEUE_REFUSED, "a 7-byte serial is refused, not guessed");
    assert_eq!(store.cash_locker(account).unwrap().len(), 1);
}

/// The `0x03E1 0x0B` body the client builds: sub-op, `u64` serial, `u32` item, `u8` tab,
/// **`u32`** slot - captured 2026-09-11 03:54:41.
fn bag_to_locker(serial: u64, item_id: u32, tab: u8, slot: u32) -> Vec<u8> {
    let mut b = net::cashshop::CLIENT_CASH_SHOP_ACTION.to_le_bytes().to_vec();
    b.push(net::cashshop::ACTION_MOVE_BAG_TO_LOCKER);
    b.extend_from_slice(&serial.to_le_bytes());
    b.extend_from_slice(&item_id.to_le_bytes());
    b.push(tab);
    b.extend_from_slice(&slot.to_le_bytes());
    b
}

/// **A bag item moves back into the locker: the `0x1B` zero form carries a record keyed on
/// the serial the client sent, and a `0x04` reload re-keys it canonically.** The owner,
/// 2026-09-11: *"it does not work in the reverse direction."* The round trip is the test:
/// out through `0x0A` (the body gives the item a serial), back through `0x0B` echoing it.
#[test]
fn a_bag_item_moves_back_into_the_locker_and_the_reload_rekeys_it() {
    let (mut s, store, id) = cash_shop_session();
    let account = 1i64;
    let cash = store::InventoryType::Cash;
    let placed = store.put_cash_item(account, &store::Item::bundle(5150000, 1)).unwrap();
    let serial = (account as u64) << 32 | u64::from(placed.slot);
    let out = s.handle(&locker_to_bag(serial, 5150000, cash.as_u8(), 6));
    assert_eq!(out[0].body[0], net::cashshop::RESULT_ITEM_GRANTED);
    assert!(store.cash_locker(account).unwrap().is_empty());

    // Back: the client names the item by the serial its +0x38 holds, and the Cash slot.
    let out = s.handle(&bag_to_locker(serial, 5150000, cash.as_u8(), 6));
    assert_eq!(out.len(), 2, "the 0x1B and the reload: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    let moved = &out[0];
    assert_eq!(moved.opcode, net::cashshop::CASH_SHOP_RESULT);
    assert_eq!(&moved.body[..2], &[net::cashshop::RESULT_MOVED_TO_LOCKER, 0], "0x1B, zero form: a record");
    let rec = &moved.body[2..];
    assert_eq!(u64::from_le_bytes(rec[..8].try_into().unwrap()), serial, "the REQUEST's serial - the bag lookup key");
    assert_eq!(u32::from_le_bytes(rec[16..20].try_into().unwrap()), 5150000);
    assert_eq!(rec[70], 1, "an item follows, so the row draws");
    let body = net::bag::bundle_item_with_cash_sn(5150000, 1, 0, &[0u8; net::bag::BUNDLE_OWNER_LEN], std::num::NonZeroU64::new(serial));
    assert_eq!(&rec[71..], &body[..], "the bundle body, serial in its +0x38");
    assert!(moved.what.contains("0x1B MOVED"), "{}", moved.what);
    let reload = &out[1];
    assert_eq!(reload.body[0], net::cashshop::RESULT_LOAD_LOCKER, "then the reload");
    assert_eq!(u16::from_le_bytes([reload.body[2], reload.body[3]]), 1, "one row");
    let canonical = u64::from_le_bytes(reload.body[4..12].try_into().unwrap());
    assert_eq!(canonical >> 32, account as u64);

    // The database: out of the Cash tab, back in the locker.
    assert!(store.inventory_slot(id, cash, 6).unwrap().is_none(), "the Cash slot is empty");
    let locker = store.cash_locker(account).unwrap();
    assert_eq!(locker.len(), 1);
    assert_eq!(locker[0].item.item_id, 5150000);
    assert_eq!(canonical & 0xFFFF_FFFF, u64::from(locker[0].slot), "the reload keys the row on its real slot");

    // And every check that fails moves nothing: wrong slot, wrong item, wrong tab, serial 0.
    let placed = store.put_cash_item(account, &store::Item::bundle(5680004, 1)).unwrap();
    let serial2 = (account as u64) << 32 | u64::from(placed.slot);
    s.handle(&locker_to_bag(serial2, 5680004, cash.as_u8(), 2));
    for (what, req) in [
        ("an empty slot", bag_to_locker(serial2, 5680004, cash.as_u8(), 3)),
        ("the wrong item id", bag_to_locker(serial2, 5150000, cash.as_u8(), 2)),
        ("the wrong tab", bag_to_locker(serial2, 5680004, store::InventoryType::Etc.as_u8(), 2)),
        ("serial 0", bag_to_locker(0, 5680004, cash.as_u8(), 2)),
        ("a slot beyond u16", bag_to_locker(serial2, 5680004, cash.as_u8(), 0x1_0002)),
    ] {
        let out = s.handle(&req);
        assert_eq!(out.len(), 1, "{what}: one refusal");
        assert_eq!(out[0].body[0], net::cashshop::RESULT_QUEUE_REFUSED, "{what}: 0x3D, not 0x1A");
        assert!(out[0].what.contains("nothing moved"), "{what}: {}", out[0].what);
        assert!(store.inventory_slot(id, cash, 2).unwrap().is_some(), "{what}: still in the bag");
        assert_eq!(store.cash_locker(account).unwrap().len(), 1, "{what}: locker unchanged");
    }
}

/// **Every check the client made is re-made here, and a failed one moves nothing.** Wrong
/// account in the serial, a slot already occupied, an item id that does not match the locker
/// row, the wrong tab, slot 0 - each is the queue refusal (`0x3D`, not `0x1A`, which would
/// empty the client's queue), and afterwards the locker still holds the item and the tab is
/// still empty.
#[test]
fn a_locker_move_that_fails_a_check_is_refused_and_moves_nothing() {
    let (mut s, store, id) = cash_shop_session();
    let account = 1i64;
    let placed = store.put_cash_item(account, &store::Item::bundle(5680004, 1)).unwrap();
    let serial = (account as u64) << 32 | u64::from(placed.slot);
    let cash = store::InventoryType::Cash;
    // Occupy slot 4 so the "destination must be empty" check has something to refuse.
    store.set_inventory_slot(id, cash, 4, &store::Item::bundle(5070000, 1)).unwrap();

    let bad = [
        ("another account's serial", locker_to_bag((2u64 << 32) | 1, 5680004, cash.as_u8(), 3)),
        ("an empty locker slot", locker_to_bag((account as u64) << 32 | 9, 5680004, cash.as_u8(), 3)),
        ("the wrong item id", locker_to_bag(serial, 5070000, cash.as_u8(), 3)),
        ("the wrong tab", locker_to_bag(serial, 5680004, store::InventoryType::Etc.as_u8(), 3)),
        ("slot 0", locker_to_bag(serial, 5680004, cash.as_u8(), 0)),
        ("an occupied slot", locker_to_bag(serial, 5680004, cash.as_u8(), 4)),
        ("a short body", {
            let mut b = net::cashshop::CLIENT_CASH_SHOP_ACTION.to_le_bytes().to_vec();
            b.push(net::cashshop::ACTION_MOVE_LOCKER_TO_BAG);
            b.extend_from_slice(&[0u8; 8]);
            b
        }),
    ];
    for (why, body) in bad {
        let out = s.handle(&body);
        assert_eq!(out.len(), 1, "{why}: answered once: {out:?}");
        assert_eq!(out[0].opcode, net::cashshop::CASH_SHOP_RESULT, "{why}");
        assert_eq!(out[0].body[0], net::cashshop::RESULT_QUEUE_REFUSED, "{why}: 0x3D keeps the queue");
        assert_eq!(out[0].body.len(), net::cashshop::CASH_SHOP_QUEUE_REFUSAL_LEN, "{why}");
        assert!(out[0].what.contains("nothing moved"), "{why}: {}", out[0].what);
        assert_eq!(store.cash_locker(account).unwrap().len(), 1, "{why}: the locker still has it");
        assert_eq!(store.bag_items(id, cash).unwrap().len(), 1, "{why}: only the pre-placed item");
    }
}

/// **Entering the shop lists what the locker holds as ONE `0x04` reload**, between the
/// stage packet and the wallet - so an item bought and left in the Cash Inventory is still
/// there on the next visit. New 2026-09-10; before it only the current visit's purchase
/// appeared.
///
/// **And never as `0x0C`.** That was the first version, one per row, and the owner saw the shop
/// open onto "You have successfully made the purchase" two runs in a row: `0x0C` is the buy
/// reply and carries that dialog. This pins the sub-op, the flag byte, and the absence of
/// any `0x0C` on entry.
#[test]
fn entering_the_shop_lists_the_lockers_stored_rows() {
    let (mut s, store, _id) = cash_shop_session();
    let account = 1i64;
    store.put_cash_item(account, &store::Item::bundle(5680004, 1)).unwrap();
    store.put_cash_item(account, &store::Item::bundle(5680002, 1)).unwrap();

    let out = s.on_cash_shop_request(&[0u8; 5]);
    let ops: Vec<u16> = out.iter().map(|r| r.opcode).collect();
    assert_eq!(ops[0], net::cashshop::SET_CASH_SHOP, "the stage first");
    assert_eq!(*ops.last().unwrap(), net::cashshop::CASH_SHOP_WALLET, "the wallet last");
    // The beauty coupons' preview lists (the owner, 2026-09-26): once, inside the stage, before the
    // wallet - and exactly the salon's lists.
    let preview: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::cashshop::CASH_SHOP_BEAUTY_PREVIEW).collect();
    assert_eq!(preview.len(), 1, "{ops:?}");
    assert_eq!(&preview[0].body[9..], &net::cashshop::beauty_preview(0, &crate::salon::cash_shop_previews())[9..], "the stamp aside, the salon's lists");
    assert!(
        !out.iter().any(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT
            && r.body[0] == net::cashshop::RESULT_ITEM_TO_LOCKER),
        "0x0C on entry is the 'purchase successful' dialog the owner saw: {ops:?}"
    );
    let reloads: Vec<&Reply> = out
        .iter()
        .filter(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT
            && r.body[0] == net::cashshop::RESULT_LOAD_LOCKER)
        .collect();
    assert_eq!(reloads.len(), 1, "exactly one reload: {ops:?}");
    let r = reloads[0];
    assert_eq!(r.body[1], 0, "flag 0: no over-limit warning, no dialog");
    assert_eq!(u16::from_le_bytes([r.body[2], r.body[3]]), 2, "both rows");
    // Each record carries the serial the move request will echo, (account << 32) | slot, and
    // - after its trailing flag - the item itself, the bag's own type-2 body, which is what
    // the panel's row widget draws. Walk them by that body's length.
    let mut at = 4;
    for (slot, item_id) in [(1u64, 5680004u32), (2, 5680002)] {
        let serial = u64::from_le_bytes(r.body[at..at + 8].try_into().unwrap());
        assert_eq!(serial, (account as u64) << 32 | slot, "{}", r.what);
        assert_eq!(u32::from_le_bytes(r.body[at + 16..at + 20].try_into().unwrap()), item_id);
        assert_eq!(r.body[at + 70], 1, "trailing flag 1: a GW_ItemSlot follows");
        let body = net::bag::bundle_item(item_id, 1, 0, &[0u8; net::bag::BUNDLE_OWNER_LEN]);
        at += net::cashshop::CASH_ITEM_RECORD_LEN;
        assert_eq!(&r.body[at..at + body.len()], &body[..], "slot {slot}: the bundle body");
        at += body.len();
    }
    assert_eq!(r.body.len(), at + 8, "then the four trailing u16s and nothing else");
    assert!(r.what.contains("LOCKER RELOAD"), "{}", r.what);
    // An empty locker still reloads - count 0 - so a stale panel is cleared, and still no 0x0C.
    let (mut s2, _store2, _) = cash_shop_session();
    let out2 = s2.on_cash_shop_request(&[0u8; 5]);
    let empty: Vec<&Reply> = out2.iter().filter(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT).collect();
    assert_eq!(empty.len(), 1);
    assert_eq!(&empty[0].body[..4], &[net::cashshop::RESULT_LOAD_LOCKER, 0, 0, 0]);
}

/// **`!hair` and `!face` change the record and re-enter the map**, which is the only way a
/// look reaches the screen in this client. Built for the first hybrid-asset test: hair
/// 42540 and face 22035 are the backported Frieren ids, present in the name table the
/// hybrid data generates. The gates - id space, a name - are exercised as controls.
#[test]
fn hair_and_face_commands_persist_and_re_enter_the_map() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account = store.create_account("maplecw", "correct horse battery").unwrap();
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "TestCharD".to_string(), map_id: 40, ..Default::default() };
    let id = store.create_character(account, 0, &chr).unwrap().id;
    store.set_character_map(id, 40).unwrap();
    store.create_migration(account, id, 0, 0).unwrap();
    let mut names = std::collections::HashMap::new();
    names.insert(42540u32, "Frieren Hair".to_string());
    names.insert(22035u32, "Frieren Face".to_string());
    names.insert(1302000u32, "Sword".to_string());
    let config = Config { item_names: names, ..Config::default() };
    let mut s = Session::new(store.clone(), Arc::new(config));
    assert!(s.claim_for_character(id).contains("claimed the migration"));
    let before = store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();

    let out = s.handle(&gm_chat("!hair 42540"));
    assert!(notice_text(&out[0]).contains("hair is now 42540 (Frieren Hair)"), "{}", notice_text(&out[0]));
    // Since 2026-09-18: a 0x007C with the HAIR bit and the id, not a re-entry (the same-map
    // SetField was a visible reload onto the spawn point).
    assert!(!out.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "no re-entry: {out:?}");
    let sc = out.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).expect("the look StatChanged");
    assert_eq!(stat_changed_mask(&sc.body), net::stats::bits::HAIR, "{:?}", sc.body);
    assert_eq!(stat_changed_first_u32(&sc.body), 42540);
    assert_eq!(sc.body[0], 0, "a GM command is not an exclusive request; nothing to unlock");
    let out = s.handle(&gm_chat("!face 22035"));
    assert!(notice_text(&out[0]).contains("face is now 22035"), "{}", notice_text(&out[0]));
    let sc = out.iter().find(|r| r.opcode == net::stats::STAT_CHANGED).expect("the look StatChanged");
    assert_eq!(stat_changed_mask(&sc.body), net::stats::bits::FACE);
    assert_eq!(stat_changed_first_u32(&sc.body), 22035);
    let after = store.characters_for(account, 0).unwrap();
    let me = after.iter().find(|c| c.id == id).unwrap();
    assert_eq!((me.hair, me.face), (42540, 22035), "persisted");
    assert_eq!(me.map_id, before.map_id, "still on the same map, never moved");

    // Controls: outside the id space, an id with no name, and a face id given to !hair.
    for (cmd, why) in [("!hair 1302000", "outside the hair id space"), ("!face 42540", "outside the face id space"), ("!hair 42541", "no name")] {
        let out = s.handle(&gm_chat(cmd));
        assert!(notice_text(&out[0]).contains("REFUSED"), "{cmd} should refuse ({why}): {}", notice_text(&out[0]));
        assert!(!out.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "{cmd}: a refusal must not warp");
        assert!(!out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED), "{cmd}: a refusal redraws nothing");
    }
    let me = store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
    assert_eq!((me.hair, me.face), (42540, 22035), "the refusals changed nothing");
}

/// The `0x0114` body: u32 tick, u16 slot, u32 itemId - the capture of 2026-09-09.
fn use_cash_item_body(slot: u16, item_id: u32) -> Vec<u8> {
    let mut b = 0u32.to_le_bytes().to_vec();
    b.extend_from_slice(&slot.to_le_bytes());
    b.extend_from_slice(&item_id.to_le_bytes());
    b
}

/// **A summoned mob is handed to the client that summoned it.** The owner, 2026-09-12, beside a
/// Balrog from a sack: *"does not have AI and does not have movement and does not use
/// skills."* The sack spawned it (0x03C6) and granted nobody control (0x03D2); no client
/// runs a mob it does not control. Field entry and the respawn tick both grant, the sack
/// did not. The 0x03C6 is map-wide; the 0x03D2 is this connection's alone.
#[test]
fn a_sack_summons_a_mob_and_grants_this_client_control_of_it() {
    let (mut s, store, id) = gm_session();
    let mut cfg = (*s.config).clone();
    cfg.summon_sacks.insert(2_100_006, crate::config::SummonSack { mobs: vec![800_023] });
    // Balrog: summonType 0 - Effect/Summon.img/0, the 2.5 s summoning circle.
    cfg.mob_templates.insert(800_023, crate::config::MobTemplate { max_hp: 325_400, summon_type: 0, ..Default::default() });
    s.config = Arc::new(cfg);
    let slot = store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_100_006, 1), 1).unwrap()[0].slot;
    s.last_position = Some((520, 395));

    let mut body = Vec::new();
    body.extend_from_slice(&0u32.to_le_bytes());
    body.extend_from_slice(&slot.to_le_bytes());
    body.extend_from_slice(&2_100_006u32.to_le_bytes());
    let out = s.on_summon_sack(&body);
    let ops: Vec<u16> = out.iter().map(|r| r.opcode).collect();
    let spawn = out.iter().position(|r| r.opcode == net::mob::MOB_ENTER_FIELD).expect("the spawn");
    let grant = out.iter().position(|r| r.opcode == net::mobmove::MOB_CHANGE_CONTROLLER).expect("the grant: {ops:?}");
    assert!(spawn < grant, "grant after the spawn, as the client requires: {ops:?}");
    let object_id = u32::from_le_bytes(out[spawn].body[1..5].try_into().unwrap()); // after sealedInsteadDead
    assert_eq!(u32::from_le_bytes(out[grant].body[1..5].try_into().unwrap()), object_id, "the same mob");
    assert_eq!(out[grant].body[0], net::mobmove::CONTROL_NORMAL);
    assert!(out[grant].what.contains("summoned"), "{}", out[grant].what);
    // The registry agrees, and a second claim by anyone is refused.
    assert!(!s.fields.controllers().claim_one(crate::fields::FieldKey::world(s.claimed_character().unwrap().map_id), object_id, 99));

    // **The summoning animation, then the reset that makes the mob hittable.** The owner,
    // 2026-09-12: "it is also missing the summon effect that is played for all players."
    // appear_type is the template's summonType (0 -> Effect/Summon.img/0), which suspends the
    // mob (mob+0x504 = 1); the 0x03E8 is due when the 2500 ms animation ends, not before.
    // (The body's appear byte sits after the forced-stat block, whose length depends on the
    // template, so the log line - built from the same FieldMob - is the stable readout.)
    assert!(out[spawn].what.contains("appear 0 (Effect/Summon.img/0; SUSPENDED until the 0x03E8 in 2500 ms"), "{}", out[spawn].what);
    assert!(out.iter().all(|r| r.opcode != net::mobmove::MOB_SUSPEND_RESET), "not yet");
    let early = s.tick(2_400);
    assert!(early.iter().all(|r| r.opcode != net::mobmove::MOB_SUSPEND_RESET), "still playing at 2400 ms");
    let due = s.tick(2_500);
    let reset = due.iter().find(|r| r.opcode == net::mobmove::MOB_SUSPEND_RESET).expect("the reset at 2500 ms");
    assert_eq!(&reset.body[..], &[object_id.to_le_bytes().as_slice(), &[1u8]].concat()[..], "u32 objectId, u8 1");
    assert!(s.tick(9_000).iter().all(|r| r.opcode != net::mobmove::MOB_SUSPEND_RESET), "sent once");
    // A template with no effect entry (summonType past the image) is a plain -2 spawn.
    let mut cfg = (*s.config).clone();
    cfg.mob_templates.insert(800_023, crate::config::MobTemplate { max_hp: 325_400, summon_type: 99, ..Default::default() });
    s.config = Arc::new(cfg);
    let slot = store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_100_006, 1), 1).unwrap()[0].slot;
    body[4..6].copy_from_slice(&slot.to_le_bytes());
    let out = s.on_summon_sack(&body);
    let spawn = out.iter().find(|r| r.opcode == net::mob::MOB_ENTER_FIELD).unwrap();
    assert!(spawn.what.contains("appear -2 (no summon effect"), "{}", spawn.what);
    assert!(s.pending_suspend_resets.is_empty(), "nothing to reset for a plain spawn");
}

/// **A Leaf Point Exchange Coupon credits the ACCOUNT's Leaf Points and is used up.** The owner,
/// 2026-09-17. The client sends a Use-tab scripted consumable on `0x0114` (the dispatcher
/// read in `crate::leafcoupons`), so the coupon is found in the Use tab, not the Cash tab;
/// the wallet moves before the coupon leaves; the notice names the amount and the balance;
/// two coupons of different sizes add up; and a coupon the slot does not hold is refused with
/// nothing consumed and nothing credited.
#[test]
fn a_leaf_point_coupon_credits_the_account_and_is_used_up() {
    let (mut s, store, id) = gm_session();
    let account = s.claimed().unwrap().account_id;
    assert_eq!(store.cash_wallet(account).unwrap().maple_points, 0);
    let slot_1k = store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_430_004, 1), 1).unwrap()[0].slot;
    let slot_100k = store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_430_008, 1), 1).unwrap()[0].slot;
    let use_tab = |store: &Arc<Store>| -> Vec<u32> {
        store.bag(id).unwrap().items_in(store::InventoryType::Use).map(|i| i.item.item_id).collect()
    };

    // The 1,000: credited, gone, said.
    let out = s.on_use_cash_item(&use_cash_item_body(slot_1k, 2_430_004));
    assert_eq!(store.cash_wallet(account).unwrap().maple_points, 1_000, "the account's Leaf Points");
    assert_eq!(use_tab(&store), vec![2_430_008], "the 1,000 coupon is used up; the other stays");
    let notice = out.iter().find(|r| r.opcode == net::notice::CHAT_NOTICE).expect("the line");
    assert!(notice.what.contains("1,000 Leaf Points") && notice.what.contains("now have 1,000"), "{}", notice.what);
    assert!(out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "the slot is cleared on screen");

    // Then the 100,000, on top.
    s.on_use_cash_item(&use_cash_item_body(slot_100k, 2_430_008));
    assert_eq!(store.cash_wallet(account).unwrap().maple_points, 101_000);
    assert!(use_tab(&store).is_empty());

    // A coupon the slot does not hold: refused, answered, nothing moves.
    let out = s.on_use_cash_item(&use_cash_item_body(slot_1k, 2_430_007));
    assert!(out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE), "answered: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert_eq!(store.cash_wallet(account).unwrap().maple_points, 101_000, "nothing credited");
    assert!(!out.iter().any(|r| r.what.contains("received")), "and not said to be");
}

/// **The box hands out all eight Outfit Set Coupons, and a set coupon hands out its set.**
/// The owner, 2026-09-10: *"instead of obtaining 1 at random rates, we give them all of the
/// sets."* The box is consumed, the eight land in the Cash tab, and opening Frieren's puts
/// three hair coupons and a face coupon in the Use tab and six equips in the Equip tab.
#[test]
fn the_collection_gives_every_set_and_a_set_coupon_gives_its_contents() {
    let (mut s, store, id) = gm_session();
    let slot = store
        .add_item(id, store::InventoryType::Cash, &store::Item::bundle(crate::signaturestyle::COLLECTION, 1), 1)
        .unwrap()[0]
        .slot;

    let out = s.on_use_cash_item(&use_cash_item_body(slot, crate::signaturestyle::COLLECTION));
    assert!(out.iter().any(|r| r.what.contains("8 item(s) given")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    let cash: Vec<u32> = store.bag(id).unwrap().items_in(store::InventoryType::Cash).map(|i| i.item.item_id).collect();
    assert_eq!(cash.len(), 8, "the box is gone and eight coupons are in: {cash:?}");
    assert!(!cash.contains(&crate::signaturestyle::COLLECTION));
    for set in &crate::signaturestyle::SETS {
        assert!(cash.contains(&set.coupon), "{} coupon missing", set.name);
    }

    // Open Frieren's: it asks which version first (Nexon ships three), and the coupon stays
    // until a version is chosen. Choose the normal one.
    let frieren = &crate::signaturestyle::FRIEREN_VERSIONS[0];
    let fslot = store.bag(id).unwrap().items_in(store::InventoryType::Cash).find(|i| i.item.item_id == 5_681_543).unwrap().slot;
    let out = s.on_use_cash_item(&use_cash_item_body(fslot, 5_681_543));
    assert!(out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE && r.what.contains("which Frieren")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert_eq!(store.bag(id).unwrap().items_in(store::InventoryType::Cash).count(), 8, "nothing consumed by opening the menu");
    let out = s.on_script_reply(&menu_reply(Some(0)));
    assert!(out.iter().any(|r| r.what.contains("7 item(s) given")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    let bag = store.bag(id).unwrap();
    let use_tab: Vec<u32> = bag.items_in(store::InventoryType::Use).map(|i| i.item.item_id).collect();
    let equip_tab: Vec<u32> = bag.items_in(store::InventoryType::Equip).map(|i| i.item.item_id).collect();
    assert_eq!(use_tab.len(), 2, "one hair coupon and the face coupon: {use_tab:?}");
    for c in frieren.hair_coupons.iter().chain([&frieren.face_coupon]) {
        assert!(use_tab.contains(c), "{c} missing");
    }
    assert_eq!(equip_tab.len(), 5, "{equip_tab:?}");
    for e in frieren.equips {
        assert!(equip_tab.contains(e), "{e} missing");
    }
    assert_eq!(bag.items_in(store::InventoryType::Cash).count(), 7, "the Frieren coupon is gone, seven remain");
    // Every hand-out is told to the client: one InventoryOperation per row placed.
    assert!(out.iter().filter(|r| r.opcode == net::inventory::INVENTORY_OPERATION).count() >= 7);
}

/// **A cash equip goes to the Deco tab, the chat says "Ubel", and a stray one in the Equip
/// tab is moved on field entry.** The owner, 2026-09-11: *"The chat does not handle the accented
/// character well ... It says I also got the outfit, but I see nothing in my decoration
/// inventory."* The four went to the Equip tab by leading digit; the client keeps cash
/// equips (ItemInfo `cash`) in tab 6.
#[test]
fn a_cash_equip_lands_in_the_deco_tab_and_the_notice_is_ascii() {
    let (mut s, store, id) = gm_session();
    let uebel = crate::signaturestyle::set_for_coupon(5_681_548).unwrap();
    // The config flags the set's equips as cash, as the regenerated equips.txt does.
    let mut equips = std::collections::HashMap::new();
    for &e in uebel.equips {
        equips.insert(e, crate::config::EquipTemplate { cash: true, ..Default::default() });
    }
    let mut cfg = (*s.config).clone();
    cfg.equips = equips;
    s.config = Arc::new(cfg);

    let slot = store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_681_548, 1), 1).unwrap()[0].slot;
    let out = s.on_use_cash_item(&use_cash_item_body(slot, 5_681_548));
    let bag = store.bag(id).unwrap();
    let deco: Vec<u32> = bag.items_in(store::InventoryType::Deco).map(|i| i.item.item_id).collect();
    assert_eq!(deco.len(), uebel.equips.len(), "every equip of the set is in Deco: {deco:?}");
    assert_eq!(bag.items_in(store::InventoryType::Equip).count(), 0, "and none in Equip");
    // The Add packets name tab 6, and carry a serial (Deco is a cash tab the shop scans).
    let adds: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::inventory::INVENTORY_OPERATION && r.what.contains("Deco")).collect();
    assert_eq!(adds.len(), uebel.equips.len(), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    for r in &adds {
        assert_eq!(r.body[8], 6, "invType 6, the Deco tab (after u8,u8,u32,u8,mode): {}", r.what);
        let item_at = 11; // u8, u8, u32, u8, mode, invType, i16 pos
        assert_eq!(r.body[item_at], net::opcode::EQUIPPED_ITEM_TYPE);
        assert_eq!(r.body[item_at + 5], 1, "hasCashSN");
        let sn = u64::from_le_bytes(r.body[item_at + 6..item_at + 14].try_into().unwrap());
        assert_eq!((sn >> 16) & 0xFF, 6, "the tab is in the serial: {sn:#x}");
    }
    let notice = out.iter().rev().find(|r| r.opcode == net::notice::CHAT_NOTICE).unwrap();
    let text = notice_text(notice);
    assert!(text.starts_with("Ubel Outfit Set: you received"), "{text}");
    assert!(text.is_ascii());

    // A stray: the same equip placed in the Equip tab (as every one was before today) is
    // moved to Deco on the next field entry, and the restore then sends it as Deco.
    store.set_inventory_slot(id, store::InventoryType::Equip, 5, &store::Item::equip(uebel.equips[0])).unwrap();
    let out = s.restore_bag_and_mesos();
    let bag = store.bag(id).unwrap();
    assert_eq!(bag.items_in(store::InventoryType::Equip).count(), 0, "moved out of Equip");
    assert_eq!(bag.items_in(store::InventoryType::Deco).count(), uebel.equips.len() + 1, "and into Deco");
    let restored = out.iter().filter(|r| r.opcode == net::inventory::INVENTORY_OPERATION && r.what.contains("Deco")).count();
    assert_eq!(restored, uebel.equips.len() + 1, "the Deco tab is restored on field entry");
}

/// **The Quest Helper counts its items again after the map change.** The owner, 2026-09-18: *"when
/// players enter a new map, the progress in quest helper completely zeroes out. But when you
/// pick up an item ... it will return back to normal."* The quiet restore skips the client's
/// quest hook; each in-progress quest's own record, unchanged, is re-sent AFTER the bag so the
/// helper recounts. A completed quest and a never-taken one send nothing.
#[test]
fn the_bag_restore_is_followed_by_every_in_progress_quests_record_so_the_helper_recounts() {
    let (mut s, store, id) = claimed_session();
    store.start_quest(id, 1008).unwrap();
    store.start_quest(id, 1010).unwrap();
    store.set_quest_progress(id, 1010, "003").unwrap();
    store.start_quest(id, 1002).unwrap();
    store.complete_quest(id, 1002).unwrap();

    let out = s.restore_bag_and_mesos();
    let last_bag = out.iter().rposition(|r| r.opcode == net::inventory::INVENTORY_OPERATION).unwrap_or(0);
    let records: Vec<(usize, &Reply)> = out.iter().enumerate().filter(|(_, r)| r.opcode == net::quest::MESSAGE).collect();
    assert_eq!(records.len(), 2, "one record per in-progress quest, none for the finished one: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(records.iter().all(|(i, _)| *i > last_bag), "after the last 0x0070 of the restore, never before it");
    let mut got: Vec<(u32, u8, Vec<u8>)> = records
        .iter()
        .map(|(_, r)| (u32::from_le_bytes(r.body[1..5].try_into().unwrap()), r.body[5], r.body[6..].to_vec()))
        .collect();
    got.sort();
    assert_eq!(got[0], (1008, net::quest::QUEST_STATE_IN_PROGRESS, vec![0, 0]), "1008: in progress, empty progress string");
    assert_eq!(got[1], (1010, net::quest::QUEST_STATE_IN_PROGRESS, b"\x03\x00003".to_vec()), "1010: its own progress string, unchanged");
}

/// **The Frieren coupon, on a live-shaped bag.** The owner, 2026-09-16: *"The coupon also should
/// not be used if the user does not have enough inventory space to use the coupon. The coupon
/// should remain in the player's inventory."* Live, the set's five equips are cash equips
/// and go to the Deco tab (150 slots), and the choice of version is made in a menu before
/// anything is handed out. Deco at 146 of 150: the choice is refused whole, the coupon stays
/// in its Cash slot, the Use tab (which had room for the two coupons) gets nothing, and the
/// refusal names the Deco tab and the shortfall.
#[test]
fn frierens_coupon_is_kept_when_the_deco_tab_cannot_take_the_set() {
    let (mut s, store, id) = gm_session();
    let version = &crate::signaturestyle::FRIEREN_VERSIONS[0];
    let mut equips = std::collections::HashMap::new();
    for &e in version.equips {
        equips.insert(e, crate::config::EquipTemplate { cash: true, ..Default::default() });
    }
    let mut cfg = (*s.config).clone();
    cfg.equips = equips;
    s.config = Arc::new(cfg);
    assert_eq!(store.inventory_slots(id, store::InventoryType::Deco).unwrap(), 150, "the live Deco size");
    for _ in 0..146 {
        store.add_item(id, store::InventoryType::Deco, &store::Item::equip(1_802_006), 1).unwrap();
    }
    let coupon = crate::signaturestyle::FRIEREN_COUPON;
    let slot = store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(coupon, 1), 1).unwrap()[0].slot;
    let before = store.bag(id).unwrap();

    s.on_use_cash_item(&use_cash_item_body(slot, coupon));
    let out = s.on_script_reply(&menu_reply(Some(0)));
    let notice = out.iter().find(|r| r.opcode == net::notice::CHAT_NOTICE).expect("a refusal notice");
    let text = notice_text(notice);
    assert!(text.contains("Deco tab needs 5 free slot(s)") && text.contains("has 4"), "{text}");
    assert!(text.contains("Nothing was used up"), "{text}");
    assert!(!out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "no row placed: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(!out.iter().any(|r| r.what.contains("receipt")), "no receipt for nothing");
    let after = store.bag(id).unwrap();
    assert_eq!(after.items, before.items, "the bag is byte-for-byte what it was");
    assert!(after.items_in(store::InventoryType::Cash).any(|i| i.slot == slot && i.item.item_id == coupon), "the coupon is in its slot");
    assert_eq!(after.items_in(store::InventoryType::Use).count(), 0, "the Use tab got nothing either");
    assert!(s.conversation.is_none(), "the menu is closed");
}

/// **All or nothing.** A set that needs more Equip slots than are free is refused before a
/// single row is written, the coupon is kept, and the player is told which tab and by how
/// much. A half-opened set is the failure this exists to prevent.
#[test]
fn a_set_that_does_not_fit_is_refused_whole_and_the_coupon_is_kept() {
    let (mut s, store, id) = gm_session();
    // Stark needs 7 Equip slots. Leave 3.
    let slots = store.inventory_slots(id, store::InventoryType::Equip).unwrap();
    for i in 0..(slots - 3) {
        store.add_item(id, store::InventoryType::Equip, &store::Item::equip(1_302_000 + u32::from(i % 3)), 1).unwrap();
    }
    let slot = store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_681_549, 1), 1).unwrap()[0].slot;
    let before = store.bag(id).unwrap().items.len();

    let out = s.on_use_cash_item(&use_cash_item_body(slot, 5_681_549));
    let notice = out.iter().find(|r| r.opcode == net::notice::CHAT_NOTICE).expect("a refusal notice");
    let text = notice_text(notice);
    assert!(text.contains("Equip tab needs 7 free slot(s)") && text.contains("has 3"), "{text}");
    assert!(text.contains("Nothing was used up"), "{text}");
    assert_eq!(store.bag(id).unwrap().items.len(), before, "nothing moved");
    assert!(store.bag(id).unwrap().items_in(store::InventoryType::Cash).any(|i| i.item.item_id == 5_681_549), "the coupon is kept");
    // And the latch is still cleared, so the Cash tab is not frozen by the refusal.
    assert!(out.iter().any(|r| r.opcode == net::combat::STAT_CHANGED), "the unlock");
}

/// **The reset scrolls arrive on `0x0116`, are applied, and are consumed - with the latch
/// cleared.** The owner, 2026-09-10: *"I just tried using the AP Reset Scroll and the SP Reset
/// Scroll, it did not work and it did not take the item."* Both presses were `0x0116`, an
/// opcode nothing answered; the arms had been written against `0x0114`.
#[test]
fn the_reset_scrolls_on_0x0116_reset_and_are_consumed() {
    let (mut s, store, id) = gm_session();
    let cash = store::InventoryType::Cash;
    let ap_slot = store
        .add_item(id, cash, &store::Item::bundle(crate::session::cashitem::AP_RESET_SCROLL, 1), 1)
        .unwrap()[0]
        .slot;
    let sp_slot = store
        .add_item(id, cash, &store::Item::bundle(crate::session::cashitem::SP_RESET_SCROLL, 1), 1)
        .unwrap()[0]
        .slot;

    for (slot, item) in [
        (ap_slot, crate::session::cashitem::AP_RESET_SCROLL),
        (sp_slot, crate::session::cashitem::SP_RESET_SCROLL),
    ] {
        let out = s.on_use_stat_reset_item(&use_cash_item_body(slot, item));
        assert!(!out.is_empty(), "{item}: answered");
        // The unlock for THIS opcode is in the answer, or the inventory stays frozen.
        assert!(
            out.iter().any(|r| r.opcode == net::combat::STAT_CHANGED && r.what.contains("0x0116")),
            "{item}: the 0x0116 latch must be cleared: {:?}",
            out.iter().map(|r| &r.what).collect::<Vec<_>>()
        );
        // The reset ran: its own ack is in the replies.
        assert!(
            out.iter().any(|r| r.what.contains("reset") || r.what.contains("!reset")),
            "{item}: {:?}",
            out.iter().map(|r| &r.what).collect::<Vec<_>>()
        );
        // And the scroll is gone.
        assert!(
            store.bag(id).unwrap().items_in(cash).all(|i| i.item.item_id != item),
            "{item}: the scroll was not consumed"
        );
    }

    // The control: a slot that does not hold what the packet names is refused, kept, and
    // still answered with the unlock.
    let out = s.on_use_stat_reset_item(&use_cash_item_body(ap_slot, crate::session::cashitem::AP_RESET_SCROLL));
    assert_eq!(out.len(), 1, "unlock only: {out:?}");
    assert_eq!(out[0].opcode, net::combat::STAT_CHANGED);
}

/// **Entering the shop refills the character's bag and stamps the character id on each
/// locker record.** Both came out of the two measuring runs of 2026-09-10: the shop's Item
/// Inventory showed none of the character's Cash-tab items because nothing restored the
/// bag after SetCashShop the way field entry does after SetField; and the locker rows were
/// placed on the grid and drew nothing, so the one record field the draw could still key
/// on is filled.
#[test]
fn entering_the_shop_restores_the_bag_and_stamps_the_character_on_locker_rows() {
    let (mut s, store, id) = cash_shop_session();
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_150_000, 3), 3).unwrap();
    store.put_cash_item(1, &store::Item::bundle(5_681_548, 1)).unwrap();

    let out = s.on_cash_shop_request(&[0u8; 5]);
    let ops: Vec<u16> = out.iter().map(|r| r.opcode).collect();
    assert_eq!(ops[0], net::cashshop::SET_CASH_SHOP);
    // The Cash-tab stack goes out again, quietly, between the stage and the locker reload.
    let restore = out.iter().position(|r| r.opcode == net::inventory::INVENTORY_OPERATION && r.what.contains("5150000"))
        .expect("the Cash tab is restored inside the shop");
    let reload = out.iter().position(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT && r.what.contains("LOCKER RELOAD"))
        .expect("the locker reload");
    assert!(restore < reload, "bag first, then the locker: {ops:?}");
    assert!(out[restore].what.contains("mode 5"), "{}", out[restore].what);
    // **The Cash-tab body carries a serial** - hasCashSN 1 and a non-zero u64 keyed on the
    // character and the slot - because the shop's double-click builder sends nothing for an
    // item whose +0x38 is 0 (run 6, 2026-09-11: "nothing moves back"). An Etc-tab body does
    // not: a serial lengthens a bundle and nothing outside the Cash tab needs one.
    let cash_body = &out[restore].body;
    let item_at = cash_body.len() - net::bag::BUNDLE_ITEM_LEN - 8;
    assert_eq!(cash_body[item_at], net::bag::BUNDLE_ITEM_TYPE);
    assert_eq!(&cash_body[item_at + 1..item_at + 5], &5_150_000u32.to_le_bytes());
    assert_eq!(cash_body[item_at + 5], 1, "hasCashSN");
    let bag_sn = u64::from_le_bytes(cash_body[item_at + 6..item_at + 14].try_into().unwrap());
    assert_eq!(bag_sn, ((0x4000_0000 | u64::from(id)) << 32) | (5 << 16) | 1, "the bag serial: mark | character, then the tab and the slot (the stack landed in Cash slot 1)");
    assert_ne!(bag_sn >> 32, 1, "never an account id's high dword - a locker serial's space");
    store.add_item(id, store::InventoryType::Etc, &store::Item::bundle(4_000_000, 5), 2).unwrap();
    let out2 = s.on_cash_shop_request(&[0u8; 5]);
    let etc = out2.iter().find(|r| r.opcode == net::inventory::INVENTORY_OPERATION && r.what.contains("4000000")).expect("Etc restored");
    let etc_at = etc.body.len() - net::bag::BUNDLE_ITEM_LEN;
    assert_eq!(etc.body[etc_at], net::bag::BUNDLE_ITEM_TYPE);
    assert_eq!(etc.body[etc_at + 5], 0, "an Etc body has no serial and the plain length");
    // The locker record carries the character id at wire +12.
    let body = &out[reload].body;
    let rec = &body[4..4 + net::cashshop::CASH_ITEM_RECORD_LEN];
    assert_eq!(u32::from_le_bytes(rec[12..16].try_into().unwrap()), id, "dwCharacterID");
    assert_eq!(u32::from_le_bytes(rec[16..20].try_into().unwrap()), 5_681_548, "nItemID");
}

/// **A cash equip goes on from the Deco tab and comes off into it.** The owner, 2026-09-12:
/// *"none of the outfit items work"* - the client sent `invType 6, slot 1 -> -105` and the
/// server refused it as a bag-to-bag move. Worn slot 105 is the cash overall.
#[test]
fn a_cash_equip_moves_between_the_deco_tab_and_its_worn_slot() {
    let (mut s, store, id) = gm_session();
    let deco = store::InventoryType::Deco;
    let slot = store.add_item(id, deco, &store::Item::equip(1054562), 1).unwrap()[0].slot;

    let out = s.on_inventory_move(&inventory_move(net::inventory::INV_DECO, slot as i16, -105, -1));
    assert_eq!(out.len(), 1);
    assert!(!out[0].what.contains("REFUSING"), "{}", out[0].what);
    assert!(out[0].what.contains("Deco bag slot") && out[0].what.contains("into slot 105"), "{}", out[0].what);
    assert!(store.bag(id).unwrap().items_in(deco).next().is_none(), "the Deco tab slot is empty");
    let worn = store.equipped_items(id).unwrap();
    assert!(worn.iter().any(|e| e.slot == 105 && e.item_id == 1054562), "{worn:?}");

    let out = s.on_inventory_move(&inventory_move(net::inventory::INV_DECO, -105, slot as i16, -1));
    assert!(!out[0].what.contains("REFUSING"), "{}", out[0].what);
    assert!(out[0].what.contains("Deco bag slot"), "{}", out[0].what);
    assert!(store.equipped_items(id).unwrap().iter().all(|e| e.slot != 105));
    let back: Vec<(u16, u32)> = store.bag(id).unwrap().items_in(deco).map(|i| (i.slot, i.item.item_id)).collect();
    assert_eq!(back, vec![(slot, 1054562)]);
}

/// The `u32` mask of a `0x007C` body: `u8 excl, u8 secondary, u8 context, u32 mask, ...`.
fn stat_changed_mask(body: &[u8]) -> u32 {
    u32::from_le_bytes(body[3..7].try_into().unwrap())
}

/// The first value after the mask, for a body whose mask names exactly one `u32` field.
fn stat_changed_first_u32(body: &[u8]) -> u32 {
    u32::from_le_bytes(body[7..11].try_into().unwrap())
}

/// **The Beauty Coupon's Confirm applies the cosmetic, spends the coupon and redraws the
/// player where they stand** - a `0x007C` carrying the HAIR (or FACE) bit and the id, with
/// the exclusive-request byte set so the `0x0165` latch clears in the same packet. The owner,
/// 2026-09-18: *"the player needs to enter a different map to see the hair or face updated"*
/// - the dialog only previews, so the empty unlock this used to send drew nothing. A slot
/// that does not hold the coupon changes nothing and is still answered with the empty unlock.
#[test]
fn a_beauty_coupon_confirm_changes_the_hair_and_spends_the_coupon() {
    let (mut s, store, id) = gm_session();
    let use_tab = store::InventoryType::Use;
    let slot = store.add_item(id, use_tab, &store::Item::bundle(2_543_143, 1), 1).unwrap()[0].slot;
    let before = s.claimed_character().unwrap().hair;
    assert_ne!(before, 42_604);

    let mut body = slot.to_le_bytes().to_vec();
    body.extend_from_slice(&2_543_143u32.to_le_bytes());
    body.extend_from_slice(&[0, 0]);
    let out = s.on_beauty_coupon_confirm(&body);
    assert!(
        !out.iter().any(|r| r.opcode == net::opcode::SET_FIELD),
        "no reload: the client applied the look itself; {:?}",
        out.iter().map(|r| r.opcode).collect::<Vec<_>>()
    );
    let scs: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::combat::STAT_CHANGED).collect();
    assert_eq!(scs.len(), 1, "ONE StatChanged - the redraw and the unlock are the same packet: {out:?}");
    assert_eq!(scs[0].body[0], 1, "exclusive-request byte set, so the 0x0165 latch clears");
    assert_eq!(stat_changed_mask(&scs[0].body), net::stats::bits::HAIR, "{:?}", scs[0].body);
    assert_eq!(stat_changed_first_u32(&scs[0].body), 42_604, "the new hair rides the packet");
    assert_eq!(s.claimed_character().unwrap().hair, 42_604, "Übel Hair in its default Green");
    assert!(store.bag(id).unwrap().items_in(use_tab).all(|i| i.item.item_id != 2_543_143), "the coupon is spent");

    // The control: the same confirm again - the slot is empty now - is refused with the
    // unlock alone (empty mask, nothing redrawn), and the hair stays.
    let out = s.on_beauty_coupon_confirm(&body);
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!(out[0].opcode, net::combat::STAT_CHANGED);
    assert_eq!(stat_changed_mask(&out[0].body), 0, "a refusal redraws nothing");
    assert_eq!(out[0].body[0], 1, "but still unlocks");
    assert_eq!(s.claimed_character().unwrap().hair, 42_604);

    // And a face coupon writes the face, with the FACE bit.
    let fslot = store.add_item(id, use_tab, &store::Item::bundle(2_890_911, 1), 1).unwrap()[0].slot;
    let mut body = fslot.to_le_bytes().to_vec();
    body.extend_from_slice(&2_890_911u32.to_le_bytes());
    let out = s.on_beauty_coupon_confirm(&body);
    let sc = out.iter().find(|r| r.opcode == net::combat::STAT_CHANGED).unwrap();
    assert_eq!(stat_changed_mask(&sc.body), net::stats::bits::FACE);
    assert_eq!(stat_changed_first_u32(&sc.body), 22_639);
    assert_eq!(s.claimed_character().unwrap().face, 22_639, "Übel Face in its default Violet");
}

// ---------------------------------------------------------------------------------------
// The client's one-way reports - session/reports.rs.

/// A dispatcher-enabled session; the reports need no claimed character.
fn report_session() -> Session {
    let (_, store, _, _) = session();
    Session::new(store, Arc::new(Config::default()))
}

/// **Every report is answered with nothing, and nothing panics on the bodies the archive
/// actually carried.** `0x013D` in particular must not be answered (research/buffs.md sec 2):
/// a reply here would be a regression the screen cannot show, only the log.
#[test]
fn every_client_report_is_answered_with_nothing() {
    let mut s = report_session();
    let bodies: &[(u16, &[u8])] = &[
        (0x013D, &[0, 1, 0, 0, 0, 0xdf, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0x29, 0, 0, 0, 0, 0, 0, 0]),
        (0x02F4, &[7, 0, 0, 0, 0x5a, 3, 0, 0, 0xab, 0xff, 0xff, 0xff]),
        (0x01ED, &[0x20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        (0x01A5, &[0, 0, 0, 0, 0, 0, 0, 0]),
        (0x02DE, &[1]),
        (0x0184, &[8, 0, 0, 0, 4, 0, 4, 0, 4, 0, 0, 0]),
        (0x0194, &[1]),
        (0x00B8, &[0]),
        (0x0420, &[0x11, 0, b'2', b'0', b'2', b'6']),
        (0x0422, &[4, 0, 0, 0, 2, 0, b'1', b'2']),
        (0x0422, &[]),           // truncated: must not panic
        (0x0422, &[2, 0, 0, 0]), // reason without a string: must not panic
        (0x0425, &[0x64, 0, 0, 0, 1, 0, 0, 0]),
        (0x0425, &[]),
        (0x0426, &[0xff; 20]),
        (0x01C1, &[0xd5, 0, 0, 0, 6, 0, b'J', b'o', b's', b'i', b'a', b'h']),
        (0x01B9, &[]),
        (0x0226, &[0x83, 2, 0, 0, 9, 0xf1, 0x78, 0x14, 0, 0]),
    ];
    for (op, body) in bodies {
        assert!(net::names::is_client_report(*op), "0x{op:04X} is not listed as a report");
        let mut packet = op.to_le_bytes().to_vec();
        packet.extend_from_slice(body);
        let out = s.handle(&packet);
        assert!(out.is_empty(), "0x{op:04X} must be answered with NOTHING, got {:?}", out.iter().map(|r| r.opcode).collect::<Vec<_>>());
    }
}

/// A report has a name, so `grep UNKNOWN` over a run finds only the genuinely new; and the
/// three undecoded ones keep logging in full, because their bytes are the only evidence.
#[test]
fn reports_are_named_and_the_undecoded_ones_stay_whole() {
    for op in [0x013Du16, 0x02F4, 0x01ED, 0x01A5, 0x02DE, 0x0184, 0x0194, 0x00B8, 0x0420, 0x0422, 0x0425, 0x0426, 0x01C1, 0x01B9, 0x0226] {
        let label = net::names::label(op);
        assert!(!label.contains("UNKNOWN"), "{label}");
    }
    for op in [0x01C1u16, 0x01B9, 0x0226] {
        assert!(net::names::label(op).contains("UNDECODED"));
        assert!(net::names::never_truncate(op), "0x{op:04X} must log whole");
    }
    // And a real request is not swallowed by the report arm: the beauty coupon still latches.
    assert!(!net::names::is_client_report(net::beautycoupon::CLIENT_BEAUTY_COUPON_CONFIRM));
    assert!(net::dropmoney::latches_the_exclusive_request(net::beautycoupon::CLIENT_BEAUTY_COUPON_CONFIRM));
}

// ---------------------------------------------------------------------------------------
// The package receipt: the Administrator's "You have received the following items" box.

/// **Opening a set coupon ends with the receipt**: a Say from NPC 9010000 whose text carries
/// one `#i<id># #t<id>#` line per item actually handed out, the OK on it is answered the way
/// every last box is (silently, conversation cleared), and nothing else on that reply path
/// claims the receipt's conversation. The owner, 2026-09-12: *"open a NPC dialogue from
/// 'MapleStory Administrator' ... list out the items ... one per line along with the
/// appropriate item icon."*
#[test]
fn opening_a_package_ends_with_the_administrators_receipt_and_ok_closes_it() {
    let (mut s, store, id) = gm_session();
    // Übel's: a fixed set (Frieren's now asks which version first - its own test).
    let frieren = crate::signaturestyle::set_for_coupon(5_681_548).unwrap();
    let slot = store
        .add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_681_548, 1), 1)
        .unwrap()[0]
        .slot;
    let out = s.on_use_cash_item(&use_cash_item_body(slot, 5_681_548));

    let receipt = out.last().expect("a reply");
    assert_eq!(receipt.opcode, net::script::SCRIPT_MESSAGE, "{}", receipt.what);
    assert!(receipt.what.contains("receipt"), "{}", receipt.what);
    let text = String::from_utf8_lossy(&receipt.body).into_owned();
    assert!(text.contains("You have received the following items:"), "{text:?}");
    let given: Vec<u32> = frieren.hair_coupons.iter().chain([&frieren.face_coupon]).chain(frieren.equips).copied().collect();
    // The line break is the LITERAL two characters backslash-n (scrollnpc::LINE_BREAK's
    // lesson: a real CR LF draws as nothing).
    for item in &given {
        assert!(text.contains(&format!(r"\n#i{item}# #t{item}#")), "no icon+name line for {item}: {text:?}");
    }
    assert_eq!(text.matches(r"\n#i").count(), given.len(), "one line per item, no more");
    assert!(!text.contains('\n') && !text.contains('\r'), "no real newline bytes: {text:?}");
    // The speaker is the Administrator, and the box is a plain OK (no Next, no Yes/No).
    assert!(receipt.body.windows(4).any(|w| w == crate::signaturestyle::ADMINISTRATOR_NPC.to_le_bytes()));
    let convo = s.conversation.as_ref().expect("the receipt is parked");
    assert_eq!(convo.path, crate::signaturestyle::RECEIPT_PATH);
    assert!(!convo.awaiting_yes_no && !convo.sent_with_next);

    // OK closes it: nothing sent (the established rule for the last box), conversation gone.
    let closed = s.on_script_reply(&script_reply(net::script::SCRIPT_ACTION_YES));
    assert!(closed.is_empty(), "{:?}", closed.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(s.conversation.is_none());

}

/// The receipt's path is claimed by none of the four other features that answer `0x00F3`
/// by path, in every direction - the same guarantee each of them already carries.
#[test]
fn the_receipt_path_cannot_be_confused_with_any_other_menu() {
    let p = crate::signaturestyle::RECEIPT_PATH;
    assert!(!crate::scrollnpc::is_scroll_path(p));
    assert!(!crate::taxi::is_taxi_path(p));
    assert!(!crate::secondjob::is_menu_path(p));
    assert!(!crate::dailyperks::is_menu_path(p));
    assert!(!crate::jobguide::is_menu_path(p));
    assert_ne!(p, crate::shanks::ASK_PATH);
    // And the text is exactly the shape the client draws: heading, then icon + name per line.
    let t = crate::signaturestyle::receipt_text(&[1_703_726, 2_543_137]);
    assert_eq!(t, r"You have received the following items:\n#i1703726# #t1703726#\n#i2543137# #t2543137#");
    assert_eq!(crate::signaturestyle::receipt_text(&[]), "You have received the following items:");
    // The chooser's path is disjoint too, and its slot round-trips.
    let c = crate::signaturestyle::frieren_chooser_path(3);
    assert!(!crate::scrollnpc::is_scroll_path(&c));
    assert!(!crate::taxi::is_taxi_path(&c));
    assert!(!crate::secondjob::is_menu_path(&c));
    assert!(!crate::dailyperks::is_menu_path(&c));
    assert!(!crate::jobguide::is_menu_path(&c));
    assert_eq!(crate::signaturestyle::slot_from_frieren_chooser_path("package.frieren:12"), Some(12));
    assert_eq!(crate::signaturestyle::slot_from_frieren_chooser_path("package.receipt"), None);
    // And the menu is the scroll NPC's measured grammar: literal line breaks, `#L<n>#` rows.
    let m = crate::signaturestyle::frieren_menu_text();
    assert!(m.starts_with(r"Which version of the Frieren Outfit Set would you like?\n\n#L0##i2543137# #bFrieren Outfit Set#k#l\n#L1##i2543138#"), "{m:?}");
    assert!(m.ends_with("#bFrieren (Sleep) Outfit Set#k#l"), "{m:?}");
    assert!(!m.contains('\n'));
}

/// **Frieren's Cash-Shop coupon asks which version; the choice is what spends it.** Nexon
/// ships the set as normal / Ringlets / Sleep, and the page's "Clothes Selector Coupon, your
/// choice of Clothes / Winter Clothes" is, by the owner's rule, both. The owner, 2026-09-12.
#[test]
fn frierens_coupon_asks_which_version_and_the_choice_spends_it() {
    let (mut s, store, id) = gm_session();
    let coupon = crate::signaturestyle::FRIEREN_COUPON;
    let slot = store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(coupon, 1), 1).unwrap()[0].slot;

    // Opening: the latch is released, a MENU from the Administrator opens, nothing consumed.
    let out = s.on_use_cash_item(&use_cash_item_body(slot, coupon));
    let unlock = crate::mesodrop::unlock_unhandled_latching_request(net::cashitem::CLIENT_USE_CASH_ITEM);
    assert_eq!(out[0].opcode, unlock[0].opcode, "the 0x0114 latch is released first");
    assert_eq!(out[0].body, unlock[0].body);
    let menu = out.iter().find(|r| r.opcode == net::script::SCRIPT_MESSAGE).expect("a menu");
    assert!(menu.what.contains("which Frieren"), "{}", menu.what);
    let text = String::from_utf8_lossy(&menu.body).into_owned();
    for (i, v) in crate::signaturestyle::FRIEREN_VERSIONS.iter().enumerate() {
        assert!(text.contains(&format!("#L{i}##i{}#", v.hair_coupons[0])), "row {i} with its hair icon: {text:?}");
    }
    assert_eq!(store.bag(id).unwrap().items_in(store::InventoryType::Cash).count(), 1, "the coupon stays while the menu is open");
    assert_eq!(s.conversation.as_ref().unwrap().path, crate::signaturestyle::frieren_chooser_path(slot));

    // Closing the menu keeps the coupon and says nothing.
    let closed = s.on_script_reply(&menu_reply(None));
    assert!(closed.is_empty());
    assert!(s.conversation.is_none());
    assert_eq!(store.bag(id).unwrap().items_in(store::InventoryType::Cash).count(), 1);

    // Open again, choose Ringlets: that hair, the face, both clothes, shoes, earrings, staff -
    // not the normal or the sleep hair. The coupon is spent by the choice; the receipt closes.
    s.on_use_cash_item(&use_cash_item_body(slot, coupon));
    let out = s.on_script_reply(&menu_reply(Some(1)));
    assert!(out.iter().any(|r| r.what.contains("Frieren (Ringlets) Outfit Set") && r.what.contains("7 item(s) given")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    let bag = store.bag(id).unwrap();
    let mut use_tab: Vec<u32> = bag.items_in(store::InventoryType::Use).map(|i| i.item.item_id).collect();
    use_tab.sort_unstable();
    assert_eq!(use_tab, vec![2_543_138, 2_890_907], "{use_tab:?}");
    let equips: Vec<u32> = bag.items_in(store::InventoryType::Equip).chain(bag.items_in(store::InventoryType::Deco)).map(|i| i.item.item_id).collect();
    for e in [1_054_555u32, 1_054_556, 1_074_234, 1_032_360, 1_703_722] {
        assert!(equips.contains(&e), "{e} missing from {equips:?}");
    }
    assert_eq!(bag.items_in(store::InventoryType::Cash).count(), 0, "the coupon is spent by the choice");
    assert_eq!(out.last().unwrap().opcode, net::script::SCRIPT_MESSAGE, "the receipt closes it");
    assert_eq!(s.conversation.as_ref().unwrap().path, crate::signaturestyle::RECEIPT_PATH);

    // Sleep is the short set: sleep hair, face, sleep clothes, earrings - four items.
    let (mut s3, store3, id3) = gm_session();
    let slot3 = store3.add_item(id3, store::InventoryType::Cash, &store::Item::bundle(coupon, 1), 1).unwrap()[0].slot;
    s3.on_use_cash_item(&use_cash_item_body(slot3, coupon));
    let out = s3.on_script_reply(&menu_reply(Some(2)));
    assert!(out.iter().any(|r| r.what.contains("Frieren (Sleep) Outfit Set") && r.what.contains("4 item(s) given")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());

    // A stale menu whose slot no longer holds the coupon spends nothing and hands out nothing.
    let (mut s4, store4, id4) = gm_session();
    let slot4 = store4.add_item(id4, store::InventoryType::Cash, &store::Item::bundle(coupon, 1), 1).unwrap()[0].slot;
    s4.on_use_cash_item(&use_cash_item_body(slot4, coupon));
    store4.remove_item(id4, store::InventoryType::Cash, slot4, Some(1)).unwrap();
    let out = s4.on_script_reply(&menu_reply(Some(0)));
    assert!(out.iter().any(|r| r.what.contains("no longer in that slot") || String::from_utf8_lossy(&r.body).contains("no longer")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(store4.bag(id4).unwrap().items_in(store::InventoryType::Use).next().is_none(), "nothing handed out");

    // The two variant coupons, held directly, resolve to their version without a menu.
    assert_eq!(crate::signaturestyle::set_for_coupon(5_681_544).unwrap().name, "Frieren (Ringlets)");
    assert_eq!(crate::signaturestyle::set_for_coupon(5_681_545).unwrap().name, "Frieren (Sleep)");
}

/// **A cash equip survives a relog.** The owner, 2026-09-12: *"I last had Cobalt wear the entire
/// Ubel outfit, but upon a fresh login, I do not see those cash items equipped anymore."*
/// The store had them the whole time (worn slots 105/107/108/111); the SetField record sent
/// those slot numbers raw in the look, which the client discards, and sent no second
/// equipped block at all, so the equip window had nothing either. This asserts the bytes of
/// the next field entry: the look draws the cash item at the base slot with the ordinary
/// one behind it (that look is what character select and other players see), and the
/// SetField record opens presence[44] with a block carrying the cash item at base slot 5.
#[test]
fn a_cash_equip_worn_at_105_is_drawn_and_listed_on_the_next_field_entry() {
    let (mut s, store, id) = claimed_session();
    let coat = 1040002u32; // an ordinary top, worn at 5
    let cash_coat = 1054562u32; // Ubel's Overall, worn OVER it at 105
    store.set_inventory_slot(id, store::InventoryType::Equip, 1, &store::Item::equip(coat)).unwrap();
    store.equip_from_bag(id, 1, 5).unwrap();
    store.set_inventory_slot(id, store::InventoryType::Deco, 1, &store::Item::equip(cash_coat)).unwrap();
    store.equip_from_tab(id, store::InventoryType::Deco, 1, 105).unwrap();

    let mut chr = s.claimed_character().expect("the claim resolves");
    assert!(chr.equips.contains(&(105, cash_coat)), "the store keeps it at 105: {:?}", chr.equips);
    assert!(chr.equips.contains(&(5, coat)), "and the coat it covers is still worn");

    let replies = s.go_to_map(&mut chr, 40, 0, "a relog, as far as the record is concerned".to_string());
    let sf = &replies.iter().find(|r| r.opcode == net::opcode::SET_FIELD).expect("a SetField").body;

    // The SetField record carries no compact look - the client dresses the local player
    // from the worn lists it decodes - so the look half of the fix is asserted on the
    // shared builder the character list and UserEnterField use: cash at the base slot,
    // the covered coat in the second map, no raw 105 anywhere.
    let (drawn, covered) = net::opcode::look_maps(&chr.equips);
    assert!(drawn.contains(&(5, cash_coat)), "{drawn:?}");
    assert!(covered.contains(&(5, coat)), "{covered:?}");
    let look = net::opcode::avatar_look(&chr);
    let mut raw_105 = vec![105u8];
    raw_105.extend_from_slice(&cash_coat.to_le_bytes());
    assert!(!look.windows(5).any(|w| w == raw_105.as_slice()), "the raw worn slot must not reach the look");

    // The second equipped block, with the cash coat at BASE slot 5 and the real stats.
    let dressed = s.dressed(&chr);
    let block = net::opcode::cash_equipped_block(&dressed);
    assert_eq!(block.len(), net::opcode::CASH_EQUIPPED_BLOCK_OVERHEAD + net::opcode::EQUIPPED_ENTRY_LEN);
    assert!(sf.windows(block.len()).any(|w| w == block.as_slice()), "the presence[44] block is not in the record");

    // And it is what the block costs, exactly - so the byte is set and nothing else moved.
    let mut bare = chr.clone();
    bare.equips.retain(|&(slot, _)| !net::opcode::CASH_EQUIP_SLOTS.contains(&slot));
    let without = &s.go_to_map(&mut bare, 40, 0, "the same walk, nothing cash".to_string())
        .iter().find(|r| r.opcode == net::opcode::SET_FIELD).expect("a SetField").body.clone();
    assert_eq!(sf.len() - without.len(), block.len(), "one block with one entry, and nothing else moved");
}

/// **A scrolled worn item keeps its scroll across a map change.** The owner, 2026-09-18: *"I just
/// scrolled an item in a map. When I change maps, those scrolled stats disappear ... those
/// scrolled stats should persist always."* The record's dresser asked the template for every
/// worn item; it reads the worn row's own stats first now, the way the bag restore and the
/// Character Info list already did.
#[test]
fn a_scrolled_worn_item_keeps_its_scroll_in_the_field_entry_record() {
    let (mut s, store, id) = gm_session();
    let slot = store.add_item(id, store::InventoryType::Equip, &store::Item::equip(1_040_002), 1).unwrap()[0].slot;
    let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, slot as i16, -5, -1));
    assert!(!out[0].what.contains("REFUSING"), "{}", out[0].what);
    let template = s.template_stats(1_040_002);

    // The scroll, as store::set_worn_equip records it: +70 HP, one enhancement left.
    let mut scrolled = template;
    scrolled.stats.inc_mhp += 70;
    scrolled.options.remaining_enhancements = 1;
    assert!(store.set_worn_equip(id, 5, &scrolled, 0).unwrap());

    // The record dresser - what every SetField, cash-shop return and remote look is built from.
    let chr = s.claimed_character().unwrap();
    let worn = s.dressed(&chr).into_iter().find(|(slot, _, _)| *slot == 5).expect("the shirt is worn");
    assert_eq!(worn.1, 1_040_002);
    assert_eq!(worn.2, scrolled, "the scrolled stats, not the template's");
    assert_ne!(worn.2, template);

    // And the map change itself carries them: the SetField record holds the scrolled entry.
    let entry = net::opcode::equipped_item(1_040_002, &scrolled);
    let sf = s.go_to_map(&mut chr.clone(), chr.map_id, 0, "same map".to_string()).into_iter().find(|r| r.opcode == net::opcode::SET_FIELD).expect("a SetField");
    assert!(sf.body.windows(entry.len()).any(|w| w == entry.as_slice()), "the record carries the scrolled shirt");
    let bare = net::opcode::equipped_item(1_040_002, &template);
    assert!(!sf.body.windows(bare.len()).any(|w| w == bare.as_slice()), "and not the template's copy");
}

/// **A saved key layout comes back on the next field entry.** The owner, 2026-09-12: *"Saving
/// keyboard layout still does not work. I tried putting both Power Strike on control and Slash
/// Blast on shift. It did not survive a re-login."* The rows were in the database; the
/// restore was switched off behind an unmeasured factory table. Now the SetField is followed
/// by a 0x05F1 with the READ gate and all 89 slots, the saved two on top of the factory 41.
#[test]
fn a_saved_key_layout_is_restored_right_after_the_setfield() {
    let (mut s, store, id) = claimed_session();
    // The owner's delta from world.log 23:56:57: LCtrl -> Power Strike, LShift -> Slash Blast,
    // '.' -> basic 52 (the attack the client moved off LCtrl).
    let body: Vec<u8> = {
        let mut b = vec![0u8, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 3];
        for (key, kind, action) in [(0x1Du8, 1u8, 1_001_002u32), (0x2A, 1, 1_001_001), (0x34, 5, 52)] {
            b.push(key);
            b.push(kind);
            b.extend_from_slice(&action.to_le_bytes());
        }
        b
    };
    s.on_keymap_change(&body);
    assert_eq!(store.keymap(id).unwrap().len(), 3, "the save half, as before");

    let mut chr = s.claimed_character().expect("the claim resolves");
    let replies = s.go_to_map(&mut chr, 40, 0, "a relog, as far as the keymap is concerned".to_string());
    let sf = replies.iter().position(|r| r.opcode == net::opcode::SET_FIELD).expect("a SetField");
    let km = replies.iter().position(|r| r.opcode == net::keymap::KEYMAP_INIT).expect("a FuncKeyMappedInit");
    assert!(km > sf, "the keymap rides AFTER the SetField that builds the stage it belongs to");
    let b = &replies[km].body;
    assert_eq!(b[0], 0, "READ gate");
    assert_eq!(b.len(), net::keymap::KEYMAP_INIT_LEN, "four gated tables and the quickslot gate - one table was rejected on screen");
    let slot = |code: usize| (b[1 + code * 5], u32::from_le_bytes(b[2 + code * 5..6 + code * 5].try_into().unwrap()));
    assert_eq!(slot(0x1D), (1, 1_001_002), "Power Strike on LCtrl");
    assert_eq!(slot(0x2A), (1, 1_001_001), "Slash Blast on LShift");
    assert_eq!(slot(0x34), (5, 52));
    assert_eq!(slot(0x10), (4, 8), "and Q is still the factory menu, not a zero");
}

/// **Key layouts and the pet's potions are per CHARACTER, not per account.** The owner, 2026-10-02:
/// *"Can you also make sure that keymapping are also saved for character?"* Two characters on
/// one account: the first binds LCtrl to Power Strike and picks potions 2000001 / 2000003 for
/// the pet (keymap options A and B, the ids the live log shows); the second's field entry
/// carries none of it, and its own choice leaves the first's alone.
#[test]
fn two_characters_on_one_account_keep_their_own_keys_and_pet_potions() {
    let (mut s, store, id) = claimed_session();
    let mut delta = vec![0u8, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 1, 0x1D, 1];
    delta.extend_from_slice(&1_001_002u32.to_le_bytes());
    s.on_keymap_change(&delta);
    s.on_keymap_change(&[1, 0x81, 0x84, 0x1e, 0x00]); // option A = 2000001
    s.on_keymap_change(&[2, 0x83, 0x84, 0x1e, 0x00]); // option B = 2000003

    let account = s.claimed.as_ref().unwrap().account_id;
    let other = store
        .create_character(account, 0, &net::opcode::Character { name: "Second".into(), ..Default::default() })
        .unwrap()
        .id;
    assert_ne!(other, id);
    store.create_migration(account, other, 0, 0).unwrap();
    let mut again = Session::new(store.clone(), s.config.clone());
    again.claim_for_character(other);

    let theirs = again.keymap_replies();
    assert!(!theirs.iter().any(|r| r.opcode == net::keymap::KEYMAP_OPT_A || r.opcode == net::keymap::KEYMAP_OPT_B), "no pet potions leaked: {theirs:?}");
    if let Some(km) = theirs.iter().find(|r| r.opcode == net::keymap::KEYMAP_INIT) {
        let b = &km.body;
        assert_ne!(u32::from_le_bytes(b[2 + 0x1D * 5..6 + 0x1D * 5].try_into().unwrap()), 1_001_002, "Power Strike leaked onto LCtrl");
    }

    again.on_keymap_change(&[1, 0x80, 0x84, 0x1e, 0x00]); // the second picks 2000000
    let opt_a = |sess: &Session| {
        sess.keymap_replies().into_iter().find(|r| r.opcode == net::keymap::KEYMAP_OPT_A).map(|r| r.body)
    };
    assert_eq!(opt_a(&again), Some(net::keymap::keymap_opt(2_000_000)));
    assert_eq!(opt_a(&s), Some(net::keymap::keymap_opt(2_000_001)), "the first keeps its own");
    let mine = s.keymap_replies();
    let km = mine.iter().find(|r| r.opcode == net::keymap::KEYMAP_INIT).expect("the first's layout");
    assert_eq!(u32::from_le_bytes(km.body[2 + 0x1D * 5..6 + 0x1D * 5].try_into().unwrap()), 1_001_002);
}

/// **A controller binding survives a map change, and does not land on the keyboard.** The owner,
/// 2026-09-14: *"Whenever there are customization to keybindings in the controller settings,
/// it is not getting saved properly, and when clients switch maps, their controller settings
/// are completely screwed up."* The delta's second byte is the table - 3 for the controller -
/// and the 0x05F1 at the next SetField now sends that table READ, the controller's own
/// factory with the binding on top, instead of a keep gate the client reads as "reset to
/// keyboard preset 0". The body here is the 17-byte capture from world-ch0.log 00:56:30.
#[test]
fn a_controller_binding_is_stored_under_table_3_and_restored_there_after_a_map_change() {
    let (mut s, store, id) = claimed_session();
    let controller = [0x00u8, 0x03, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x08, 0x01, 0xe8, 0x03, 0x00, 0x00];
    s.on_keymap_change(&controller);
    // And a keyboard binding on the SAME slot number, so the two tables can be told apart.
    let mut keyboard = vec![0u8, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 1, 0x08, 1];
    keyboard.extend_from_slice(&1_001_002u32.to_le_bytes());
    s.on_keymap_change(&keyboard);
    let rows = store.keymap(id).unwrap();
    assert_eq!(rows.len(), 2, "two rows, one per table: {rows:?}");
    assert_eq!((rows[0].preset, rows[0].key, rows[0].action), (0, 8, 1_001_002));
    assert_eq!((rows[1].preset, rows[1].key, rows[1].action), (3, 8, 1000));

    let mut chr = s.claimed_character().expect("the claim resolves");
    let replies = s.go_to_map(&mut chr, 40, 0, "a map change".to_string());
    let km = replies.iter().find(|r| r.opcode == net::keymap::KEYMAP_INIT).expect("a FuncKeyMappedInit");
    let b = &km.body;
    let table = 1 + net::keymap::SLOT_COUNT * 5;
    assert_eq!(b.len(), net::keymap::KEYMAP_INIT_LEN);
    for t in 0..net::keymap::PRESET_COUNT {
        assert_eq!(b[t * table], 0, "table {t} READ - the keep gate is the bug");
    }
    let slot = |t: usize, code: usize| {
        let at = t * table + 1 + code * 5;
        (b[at], u32::from_le_bytes(b[at + 1..at + 5].try_into().unwrap()))
    };
    assert_eq!(slot(3, 8), (1, 1000), "the controller's button 8");
    assert_eq!(slot(0, 8), (1, 1_001_002), "the keyboard's scan code 8, separately");
    assert_eq!(slot(3, 0), (5, 53), "controller button 0 is the controller factory's, not keyboard preset 0's");
    assert_eq!(slot(3, 0x10), (0, 0), "and no Q on the controller");
    assert_eq!(slot(0, 0x10), (4, 8), "Q is still the keyboard's menu");
}

/// **A job-advanced character's skill points survive a SetField.** seedling, 2026-09-14:
/// job-advanced to Bowman (300) at level 12, received the 7 SP in the advancement `0x007C`,
/// then walked through a portal and they were gone. The stat block in every record carries an
/// EMPTY SP table (`character_stat_block` pushes one zero byte on the extended branch), so the
/// new field read every pool as 0; nothing re-sent the real balance. Now a SetField is
/// followed by the pool packet, exactly as it follows a skill-up.
#[test]
fn a_job_advanced_characters_skill_points_are_resent_after_a_setfield() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let base = net::opcode::Character { name: "purr".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &base).unwrap();
    // purr's shape: first job, over the level-10 minimum, nothing spent.
    made.job = 300;
    made.level = 12;
    store.save_character_progress(&made).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let mut s = Session::new(store.clone(), Arc::new(Config::default()));
    s.claim_for_character(made.id);

    let mut chr = s.claimed_character().expect("claimed");
    let out = s.go_to_map(&mut chr, 40, 0, "a portal walk".to_string());
    let sf = out.iter().position(|r| r.opcode == net::opcode::SET_FIELD).expect("a SetField");
    let sp = out
        .iter()
        .position(|r| r.opcode == net::stats::STAT_CHANGED && r.what.contains("skill points now"))
        .expect("the SP pool packet must ride after the SetField");
    assert!(sp > sf, "the pool packet comes AFTER the SetField that blanked it");
    let owed = crate::skillpoints::entitlement(crate::skillpoints::Tier::First, 12);
    assert_eq!(owed, 7, "level 12 first job is owed 7 - seedling's number");
    assert!(out[sp].what.contains(&format!("tier 1 = {owed}")), "{}", out[sp].what);
}

/// **A beginner is NOT handed first-job points by the SetField refresh.** `pool_entitlement`
/// returns a level's worth for any tier, so without the reached-tier gate a level-12 beginner
/// would get 7 first-job SP they never advanced into.
#[test]
fn a_beginner_gets_no_skill_point_packet_after_a_setfield() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let base = net::opcode::Character { name: "greenhorn".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &base).unwrap();
    made.job = 0;
    made.level = 12; // over the minimum, so entitlement(First, 12) would be 7 if it leaked
    store.save_character_progress(&made).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let mut s = Session::new(store.clone(), Arc::new(Config::default()));
    s.claim_for_character(made.id);

    let mut chr = s.claimed_character().expect("claimed");
    let out = s.go_to_map(&mut chr, 40, 0, "a portal walk".to_string());
    assert!(out.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "the SetField still goes");
    assert!(
        !out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED && r.what.contains("skill points now")),
        "a beginner has no pools, so no SP packet - the points are the client's own budget"
    );
}

/// **An accept with no `yes` branch says nothing.** The owner, 2026-09-13: *"Nina's quest dialogue
/// seems to be repeated when accepting their 'What Sen wants to eat' quest. They say the same
/// two dialogues before and after I click 'Accept'."* Quest 1003 has `Say.0` (two lines the
/// client shows on its own before the button) and no `Say.0.yes`; the server was answering
/// the accept with `Say.0` again. Now: the quest record, and no box at all - not the opening,
/// and not Nina's own greeting either.
#[test]
fn accepting_a_quest_with_no_yes_branch_sends_the_record_and_no_dialogue() {
    let path = std::path::Path::new("../../gm-handbook/questlines.txt");
    if !path.exists() {
        return; // generated data, gitignored
    }
    let config = Config { quests: crate::config::load_quests(path), ..Config::default() };
    let nina = &config.quests[&1003];
    assert_eq!(nina.say["0"].len(), 2, "the two lines the owner saw twice");
    assert!(!nina.say.contains_key("0.yes"), "the premise: no yes branch to answer with");

    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);

    // world.log 04:06:17.183: action 1, quest 1003, NPC 4 (Nina).
    let replies = s.on_quest_request(&hex("01eb030000040000008d00d70000000000"));
    assert_eq!(replies.len(), 1, "the quest record and nothing else: {:?}", replies.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert_eq!(replies[0].opcode, net::quest::MESSAGE);
    assert_eq!(replies[0].body, net::quest::quest_accepted(1003));
    assert!(!replies.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE), "no Say - the client showed the opening itself");
    assert!(s.conversation.is_none(), "nothing left open to answer an OK with");
    let rows = s.store.quest_rows(id).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].quest_id, 1003);

    // The control: Heena's quest 1000 HAS a yes branch and still gets it.
    let heena = s.on_quest_request(&hex("01e8030000010000000c046d0100000000"));
    assert_eq!(heena.len(), 2, "record + the yes branch, unchanged");
    assert!(heena[1].what.contains("0.yes"), "{}", heena[1].what);
}

/// **Three Snails throws a shell, and without one it is refused with a red line.** The owner,
/// 2026-09-13: *"Three Snails is a skill that takes 1 Red Snail Shell to cast. If the user does
/// not have red snail shells in their inventory, the skill should output a red error text in
/// chat saying you do not have enough Red Snail Shell to cast this skill. Casting it should
/// decrease the client's Red Snail Shell inventory count by 1."* Level 3 throws 4000004; the
/// rows come from the client's own Skill.wz.
#[test]
fn three_snails_throws_a_red_snail_shell_and_is_refused_in_red_without_one() {
    let skills = std::path::Path::new("../../gm-handbook/skills.txt");
    if !skills.exists() {
        return; // generated, gitignored
    }
    const THREE_SNAILS: u32 = 1_000;
    const RED_SNAIL_SHELL: u32 = 4_000_004;
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Beginner".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.mp = 50;
    made.max_mp = 50;
    store.save_character_progress(&made).unwrap();
    store.set_skill_level(made.id, THREE_SNAILS, 3).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let mut config = Config {
        firstjob: crate::firstjob::CombatTable::load(skills),
        ..Config::default()
    };
    config.item_names.insert(RED_SNAIL_SHELL, "Red Snail Shell".to_string());
    let row = config.firstjob.level(THREE_SNAILS, 3).expect("Three Snails level 3");
    assert_eq!(row.item_con, Some(RED_SNAIL_SHELL), "the premise, from Skill.wz");
    assert_eq!(row.item_con_no.unwrap_or(1), 1);
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(made.id);
    let shells = |store: &Arc<Store>| -> u16 {
        store.bag_items(made.id, store::InventoryType::Etc).unwrap().iter()
            .filter(|r| r.item.item_id == RED_SNAIL_SHELL).map(|r| r.item.kind.quantity()).sum()
    };

    // Two shells in the Etc tab.
    store.add_item(made.id, store::InventoryType::Etc, &store::Item::bundle(RED_SNAIL_SHELL, 2), 2).unwrap();
    assert_eq!(shells(&store), 2);

    // First cast: one shell gone, the client told (an InventoryOperation), MP still spent.
    let out = s.handle(&swing_packet(net::combat::USER_MELEE_ATTACK, THREE_SNAILS));
    assert_eq!(shells(&store), 1, "one shell thrown");
    assert!(out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "the count change reaches the client: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED), "and the MP is still charged");
    assert!(!out.iter().any(|r| r.opcode == net::message::MESSAGE), "no complaint with a shell in hand");

    // Second cast: the last shell, and the stack disappears.
    s.handle(&swing_packet(net::combat::USER_MELEE_ATTACK, THREE_SNAILS));
    assert_eq!(shells(&store), 0);

    // Third cast: refused. The red system line, and NOTHING else - no MP, no inventory op.
    let mp_before = s.claimed_character().unwrap().mp;
    let out = s.handle(&swing_packet(net::combat::USER_MELEE_ATTACK, THREE_SNAILS));
    assert_eq!(out.len(), 1, "the line alone: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert_eq!(out[0].opcode, net::message::MESSAGE);
    assert_eq!(out[0].body[0], net::message::kind::CHAT_LINE_SYSTEM, "the client's own red system category");
    let text = String::from_utf8_lossy(&out[0].body[3..]).into_owned();
    assert!(text.contains("not have enough Red Snail Shell"), "{text}");
    assert_eq!(s.claimed_character().unwrap().mp, mp_before, "a refused cast costs no MP");

    // The control: an ordinary swing with no skill is untouched by any of this.
    let plain = s.handle(&swing_packet(net::combat::USER_MELEE_ATTACK, 0));
    assert!(!plain.iter().any(|r| r.opcode == net::message::MESSAGE));
}

/// **A quiz turn-in finalises a quiz the CLIENT conducted; it does not re-ask.** The owner,
/// 2026-09-13, with five timestamped screenshots of quest 1016: the client drew the offer,
/// the question and a "Yes, that's correct!" box entirely on its own - `world.log` had no
/// inbound quest/script packet for the 41 s those boxes were up - and only then sent the
/// turn-in, which the server answered by asking the same question again. The client has the
/// `#L` choices and `stop.0.answer` in `Quest.wz` and grades them itself, sending the turn-in
/// only on a right answer. So the server records the completion - the record, the exp, the
/// fanfare - and says nothing. Quest 1013 is the shape: `Say.1.0` is the question with `ask`.
#[test]
fn a_quiz_turn_in_completes_silently_because_the_client_conducts_the_quiz() {
    let path = std::path::Path::new("../../gm-handbook/questlines.txt");
    if !path.exists() {
        return; // generated data, gitignored
    }
    let config = Config { quests: crate::config::load_quests(path), ..Config::default() };
    let rain = &config.quests[&1013];
    assert!(rain.say["1"][0].contains("#L0#"), "the premise: the question is a menu the client renders itself");
    assert_eq!(rain.say["1.ask"], vec!["1"], "and `ask` marks the completion path as a quiz");

    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Pupil".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    store.start_quest(id, 1013).unwrap();
    let mut s = Session::new(store, Arc::new(config));
    s.claim_for_character(id);

    // world.log 17:20:55: action 2 (complete), quest 1013's shape, NPC 19 (Rain). The client
    // has already run the quiz; this is the turn-in it sends on the right answer.
    let out = s.on_quest_request(&hex("02f503000013000000caff1201ffffffff"));

    // The completion record rides with the turn-in - no second question, no closing box.
    assert!(out.iter().any(|r| r.opcode == net::quest::MESSAGE), "the turn-in records the completion: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(!out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE), "the server must NOT re-ask or repeat the closing line: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(s.store.quest_rows(id).unwrap().iter().any(|r| r.quest_id == 1013 && r.state == store::QuestState::Complete), "completed by the turn-in");
    assert!(s.conversation.is_none(), "the conversation ends on the turn-in");

    // A repeat click on a finished quest is silent - no payout, no box.
    let again = s.on_quest_request(&hex("02f503000013000000caff1201ffffffff"));
    assert!(!again.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE), "a finished quiz says nothing on a second click: {:?}", again.iter().map(|r| &r.what).collect::<Vec<_>>());
}

/// **A double-click on the Husky summons it; a second puts it away.** The owner, 2026-09-13:
/// *"I tried summoning the Husky pet, but the pet does not come out."* `world.log` 18:02:29 -
/// `0x0147`, `u32 tick, u16 slot 1`, twice, unanswered. Now the click is answered with the
/// `0x0277` the local user's vtable slot `+0x98` decodes, the Cash-tab item re-sent with
/// `active = 1` and the pairing serial, and the empty `0x0070` that closes the request; the
/// same click again puts the pet away; a slot with nothing on it gets the unlock alone.
#[test]
fn a_double_click_on_the_husky_summons_it_and_a_second_puts_it_away() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Wisp".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    // The Husky, in Cash slot 1 - where the click said it was.
    let placed = store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
    assert_eq!(placed[0].slot, 1);
    let mut item_names = std::collections::HashMap::new();
    item_names.insert(5_000_006u32, "Husky".to_string());
    let config = Config { item_names, ..Config::default() };
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(id);
    s.last_position = Some((300, -50));

    // The captured body: tick 0x14189a50, slot 1.
    let out = s.on_pet_activate(&hex("509a18140100"));
    let up = out.iter().find(|r| r.opcode == net::pet::PET_ACTIVATED).expect("a PetActivated");
    assert_eq!(&up.body[0..4], &id.to_le_bytes(), "the user pool's charId");
    assert_eq!(up.body[8], 1, "activated");
    assert_eq!(&up.body[10..14], &5_000_006u32.to_le_bytes(), "the Husky");
    assert_eq!(&up.body[16..21], b"Husky");
    assert_eq!(&up.body[29..31], &300i16.to_le_bytes(), "beside the character");
    assert!((s.active_pet_item() == Some(5_000_006)));
    let add = out
        .iter()
        .find(|r| r.opcode == net::inventory::INVENTORY_OPERATION && r.what.contains("active=1"))
        .expect("the Cash-tab item re-sent as summoned");
    let at = add.body.windows(5).position(|w| w[0] == net::bag::PET_ITEM_TYPE && w[1..5] == 5_000_006u32.to_le_bytes()).expect("a pet body in the Add");
    // type 1, itemId 4, hasCashSN 1 + serial 8, dateExpire 8, u32 4, u8 1 = 27 to the tail;
    // then name 13, level 1, closeness 2, fullness 1, dateDead 8, attr 2, skill 2, life 4, attribute 2 = 35.
    assert_eq!(add.body[at + 27 + 35], 1, "the active byte");
    assert_eq!(&add.body[at + 6..at + 14], &net::pet::pet_serial(id, pet_of(&store, id, 5_000_006)).get().to_le_bytes(), "the pairing serial on the item");
    // The owner, 2026-09-18: the re-sent pet item was the one cell in the Cash tab marked NEW on
    // every field entry. It is a re-send of an item already there, so it is mode 5 like the
    // rest of the restored bag - mode 0 is what earns the mark.
    assert_eq!(add.body[7], net::inventory::MODE_SET_QUIET, "the pet re-send is quiet: no NEW mark");
    let last = out.last().unwrap();
    assert_eq!((last.opcode, last.body[0]), (net::inventory::INVENTORY_OPERATION, 1), "the request is closed last");

    // A field entry sends the pet again, because the client rebuilt its pools.
    let chr = s.claimed_character().unwrap();
    let again = s.pet_entry_replies(&chr);
    // Three since 2026-09-21: the item, the `0x0277`, the item again. The FIRST write is what
    // `CPet` is built from - without it the client caches closeness 0 and the second write
    // makes it print "Closeness has increased (+N)" on every map change. The SECOND is the
    // re-read without which the pet spawns sad and inert (2026-09-17) and never vacuums
    // (2026-09-18). Both are on the owner's screen, in opposite directions.
    assert_eq!(again.len(), 3, "the item, the 0x0277, the item again");
    assert_eq!(again[0].opcode, net::inventory::INVENTORY_OPERATION, "the item BEFORE the summon");
    assert_eq!(again[1].opcode, net::pet::PET_ACTIVATED);
    assert_eq!(again[2].opcode, net::inventory::INVENTORY_OPERATION, "the item refresh CPet re-reads its state from");
    assert!(again[0].what.contains("active=1") && again[2].what.contains("active=1"), "and both say the pet is out");
    assert_eq!(again[0].body, again[2].body, "the two writes must be the same bytes - a difference is what the client prints");
    assert_eq!(again[2].body[7], net::inventory::MODE_SET_QUIET, "quiet on field entry too - the cell is not new");
    // And the record's own pet body now says active - the bag's row, which carries the pet
    // id the active byte is keyed on since 2026-09-16.
    let blob = s.item_blob(&bag_pet(&store, id, 5_000_006));
    assert_eq!(blob[1 + 18 + 35], 1, "the bag body agrees the pet is out");

    // The same click again: put away - activated 0, then the reason byte the OWNER's handler
    // reads (nine bytes without it killed the client, 2026-09-15 23:34) - and the item back to 0.
    let out = s.on_pet_activate(&hex("f29d18140100"));
    let down = out.iter().find(|r| r.opcode == net::pet::PET_ACTIVATED).expect("a PetActivated");
    assert_eq!(down.body.len(), 10);
    assert_eq!(down.body[8], 0, "activated = 0");
    assert_eq!(down.body[9], net::pet::PET_REMOVE_REASON_NONE, "the reason: a plain removal");
    assert!(!(s.active_pet_item() == Some(5_000_006)));
    assert!(out.iter().any(|r| r.what.contains("active=0")));
    assert!(s.pet_entry_replies(&chr).is_empty(), "nothing to re-send once it is away");

    // A slot with nothing on it: the unlock and nothing else - always answer.
    let out = s.on_pet_activate(&hex("509a18140700"));
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].opcode, out[0].body[0]), (net::inventory::INVENTORY_OPERATION, 1));
    assert!(!(s.active_pet_item() == Some(5_000_006)));
}

/// `--pet-move-action` reaches the wire and nothing else moves. The default body carries
/// `moveAction` 0 at offset 33 (after charId 4, petIdx 4, activated 1, init 1, itemId 4, the
/// 2+5 string, serial 8, x 2, y 2); with the lever set to 30 that one byte is 30 and the
/// foothold that follows it is unchanged. One variable, so the run that uses it is a
/// measurement. `Config::pet_move_action` says why 30.
#[test]
fn the_pet_move_action_lever_changes_exactly_one_byte_of_the_summon() {
    fn summon(lever: Option<u8>) -> Vec<u8> {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Wisp".to_string(), ..Default::default() };
        let id = store.create_character(account_id, 0, &chr).unwrap().id;
        store.create_migration(account_id, id, 0, 0).unwrap();
        store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
        let mut item_names = std::collections::HashMap::new();
        item_names.insert(5_000_006u32, "Husky".to_string());
        let config = Config { item_names, pet_move_action: lever, ..Config::default() };
        let mut s = Session::new(store.clone(), Arc::new(config));
        s.claim_for_character(id);
        s.last_position = Some((300, -50));
        let out = s.on_pet_activate(&hex("509a18140100"));
        out.iter().find(|r| r.opcode == net::pet::PET_ACTIVATED).expect("a PetActivated").body.clone()
    }
    let plain = summon(None);
    let flown = summon(Some(30));
    assert_eq!(plain[33], 0, "the default moveAction");
    assert_eq!(flown[33], 30, "the lever");
    assert_eq!(plain.len(), flown.len());
    let differing: Vec<usize> = (0..plain.len()).filter(|&i| plain[i] != flown[i]).collect();
    assert_eq!(differing, vec![33], "exactly one byte differs, and it is moveAction");
}

/// **A pet takes a mob's drop and nothing else.** The owner, 2026-09-13: *"turn every pet into a
/// vacuum pet ... provided that they are from a mob death drop ... offload most of the pet
/// driven operations on the client."* The client decides what the pet reaches for (the pet
/// image's `sweepForDrop`/`longRange`, patched in by the installer) and asks with a pet-shaped
/// request; the server's half is here. A mob's drop goes out with `canBePickedUpByPet` set and
/// a pet's request for it is answered with the type-5 leave and the bag write, no chat line. A
/// player's own ground drop goes out with the byte clear, and a pet's request for it gets the
/// unlock alone and the drop stays. With no pet out, a pet-shaped body names nothing.
/// (The test itself is `a_summoned_pet_picks_up_a_mob_drop_but_not_a_players_own_drop`, below
/// the four 2026-09-15 pet tests and their two helpers.)

/// The Cash slot the store put `item_id` in - `add_item` picks the slot itself (its last
/// argument is the stack size), so a test reads it back rather than assuming it.
fn cash_slot_of(store: &Store, id: u32, item_id: u32) -> u16 {
    store
        .bag_items(id, store::InventoryType::Cash)
        .unwrap()
        .into_iter()
        .find(|r| r.item.item_id == item_id)
        .map(|r| r.slot)
        .unwrap_or_else(|| panic!("item {item_id} is not in the Cash tab"))
}

/// A `0x0116` for a pet item: tick, the item's slot, the item, the pet's serial, an optional name.
fn use_pet_item_body(slot: u16, item_id: u32, owner: u32, pet_id: u32, name: Option<&str>) -> Vec<u8> {
    let mut body = net::cashitem::CLIENT_USE_STAT_RESET_ITEM.to_le_bytes().to_vec();
    body.extend_from_slice(&hex("f98e4e20"));
    body.extend_from_slice(&slot.to_le_bytes());
    body.extend_from_slice(&item_id.to_le_bytes());
    body.extend_from_slice(&net::pet::pet_serial(owner, pet_id).get().to_le_bytes());
    if let Some(n) = name {
        body.extend_from_slice(&(n.len() as u16).to_le_bytes());
        body.extend_from_slice(n.as_bytes());
    }
    body
}

/// A `0x0112`: tick, the Use slot, the food.
fn use_pet_food_body(slot: u16, item_id: u32) -> Vec<u8> {
    let mut body = net::petfood::CLIENT_USE_PET_FOOD.to_le_bytes().to_vec();
    body.extend_from_slice(&0x2050_8e0au32.to_le_bytes());
    body.extend_from_slice(&slot.to_le_bytes());
    body.extend_from_slice(&item_id.to_le_bytes());
    body
}

/// The pet's `(level, closeness, fullness)` as the CLIENT will read them - out of the Cash
/// item body the session builds, not out of the store - so the wire is what is asserted.
fn pet_vitals_on_the_wire(s: &Session, id: u32) -> (u8, u16, u8) {
    // The bag's own row, so the item carries its pet id - a bare `Item::bundle` would read as
    // a pet that has never been numbered.
    let pet = s.store.bag_items(id, store::InventoryType::Cash).unwrap().into_iter().find(|r| r.item.item_id == 5_000_006).unwrap().item;
    let pet_id = pet.pet_id.expect("numbered");
    let b = s.item_blob_with_cash_sn(&pet, Some(net::pet::pet_serial(id, pet_id)));
    // 1 type + 4 id + 1 hasSN + 8 sn + 8 expire + 4 + 1 + 13 name = 40, then level, closeness, fullness.
    (b[40], u16::from_le_bytes([b[41], b[42]]), b[43])
}

/// **Pet Food: +30 fullness, +1 closeness, the food used up, the level from the table.** The owner,
/// 2026-09-15: *"Using a pet food should recover the current active pet's fullness by 30 and
/// their closeness by 1."* The request is `0x0112` (`net::petfood`). A pet fed at 100 is an
/// overfeed: the first is free, the second costs a closeness (the wiki's rule).
#[test]
fn pet_food_restores_thirty_earns_one_closeness_and_overfeeding_costs_after_the_first() {
    let (mut s, store, id) = gm_session();
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
    store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_120_000, 5), 100).unwrap();
    let food_slot = store.bag_items(id, store::InventoryType::Use).unwrap().iter().find(|r| r.item.item_id == 2_120_000).unwrap().slot;

    // No pet out: refused, food kept, the request still unlocked.
    let out = s.handle(&use_pet_food_body(food_slot, 2_120_000));
    assert!(out.iter().any(|r| r.what.contains("Summon a pet")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED), "the 0x0112 unlock");

    s.last_position = Some((300, -50));
    s.on_pet_activate(&hex("509a18140100"));
    // Make it hungry first so the +30 is visible.
    store.set_pet_vitals(pet_of(&store, id, 5_000_006), 1, 0, 50).unwrap();

    let out = s.handle(&use_pet_food_body(food_slot, 2_120_000));
    let st = store.pet_state(pet_of(&store, id, 5_000_006)).unwrap();
    assert_eq!((st.fullness, st.closeness, st.level), (80, 1, 2), "+30, +1, and closeness 1 is level 2 in the table");
    assert_eq!(pet_vitals_on_the_wire(&s, id), (2, 1, 80), "the Cash item the client reads says so");
    assert!(out.iter().any(|r| r.what.contains("re-sent as pet 5000006")), "the item goes out again");
    assert!(out.iter().any(|r| r.what.contains("QUANTITY") && r.what.contains("down to 4")), "one food used: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    // The eating animation: 0x027E type 2, success, food id 0 - and, this feed having taken
    // the pet from level 1 to 2, the LevelUp flash as UserEffect 9 / subtype 0. The id is 0
    // because a pet-food id there is the client's switch for the auto-feed balloon "Yum, yum!
    // <food> x<count-1> left!" (the owner, 2026-09-18: hand feeds must not show it; it was also one
    // short, since the client assumes the bag has not been decremented yet).
    let ate = out.iter().find(|r| r.opcode == net::pet::PET_ACTION_COMMAND).expect("the pet eats");
    assert_eq!(&ate.body[0..4], &id.to_le_bytes());
    assert_eq!((ate.body[8], ate.body[9]), (net::pet::PET_ACTION_FOOD, 1), "type 2, success");
    assert_eq!(&ate.body[10..14], &net::pet::PET_FOOD_NONE.to_le_bytes(), "no food id: the animation, not the auto-feed balloon");
    assert!(!(2_120_000..2_130_000).contains(&net::pet::PET_FOOD_NONE), "the id must be outside the pet-food range the handler keeps");
    let flash = out.iter().find(|r| r.opcode == net::stats::USER_EFFECT_LOCAL).expect("the level-up flash");
    assert_eq!(flash.body, vec![net::pet::USER_EFFECT_PET, net::pet::PET_EFFECT_LEVEL_UP, 0, 0, 0, 0], "effect 9, subtype 0, petIdx 0");
    // The next feed (80 -> 100, closeness 2, still level 2) eats without a flash.
    let out = s.handle(&use_pet_food_body(food_slot, 2_120_000));
    assert!(out.iter().any(|r| r.opcode == net::pet::PET_ACTION_COMMAND));
    assert!(!out.iter().any(|r| r.opcode == net::stats::USER_EFFECT_LOCAL), "no level gained, no flash");
    // (that feed is the "fill it" step below - the count of foods eaten is unchanged)

    // Filled (above), then overfeed twice: free, then -1.
    assert_eq!(store.pet_state(pet_of(&store, id, 5_000_006)).unwrap().fullness, 100);
    s.handle(&use_pet_food_body(food_slot, 2_120_000)); // overfeed #1: free
    assert_eq!(store.pet_state(pet_of(&store, id, 5_000_006)).unwrap().closeness, 2);
    s.handle(&use_pet_food_body(food_slot, 2_120_000)); // overfeed #2: -1
    let st = store.pet_state(pet_of(&store, id, 5_000_006)).unwrap();
    assert_eq!((st.fullness, st.closeness, st.level), (100, 1, 2), "closeness down one, the level kept");
    assert_eq!(store.bag_items(id, store::InventoryType::Use).unwrap().iter().find(|r| r.slot == food_slot).map(|r| match r.item.kind { store::ItemKind::Bundle { quantity } => quantity, _ => 0 }), Some(1), "four foods eaten");
}

/// **Every five minutes out, one fullness; at zero the pet goes home.** The owner, 2026-09-15:
/// *"Pets should decrease their fullness by 1 every 5 minutes."* Starvation is the wiki's:
/// `-1` closeness, put away everywhere, and the next login leaves it in the bag.
#[test]
fn a_summoned_pet_loses_a_fullness_every_five_minutes_and_goes_home_at_zero() {
    let (mut s, store, id) = gm_session();
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
    s.last_position = Some((300, -50));
    s.tick(10_000);
    s.on_pet_activate(&hex("509a18140100"));
    store.set_pet_vitals(pet_of(&store, id, 5_000_006), 3, 10, 2).unwrap();
    let five = net::petfood::PET_HUNGER_INTERVAL_MS;

    // Not yet.
    assert!(s.tick(10_000 + five - 1).iter().all(|r| !r.what.contains("re-sent as pet")));
    assert_eq!(store.pet_state(pet_of(&store, id, 5_000_006)).unwrap().fullness, 2);
    // Five minutes: 2 -> 1, the item re-sent, the pet still out.
    let out = s.tick(10_000 + five);
    assert_eq!(store.pet_state(pet_of(&store, id, 5_000_006)).unwrap().fullness, 1);
    assert!(out.iter().any(|r| r.what.contains("re-sent as pet 5000006")));
    assert!((s.active_pet_item() == Some(5_000_006)));
    // Ten: 1 -> 0, starved - closeness 10 -> 9, level kept, sent home, remembered as away.
    let out = s.tick(10_000 + 2 * five);
    let st = store.pet_state(pet_of(&store, id, 5_000_006)).unwrap();
    assert_eq!((st.fullness, st.closeness, st.level, st.active), (0, 9, 3, false));
    assert!(!(s.active_pet_item() == Some(5_000_006)), "put away on this session");
    let gone = out.iter().find(|r| r.opcode == net::pet::PET_ACTIVATED).expect("the put-away");
    assert_eq!((gone.body[8], gone.body.len()), (0, 10), "activated 0, with the owner's reason byte");
    assert!(out.iter().any(|r| r.what.contains("starving")), "and the player is told");
    assert_eq!(store.active_pet(id).unwrap(), None, "the next login leaves it in the bag");
    // Nothing more happens while it is away.
    assert!(s.tick(10_000 + 3 * five).iter().all(|r| r.opcode != net::pet::PET_ACTIVATED));
}

/// **A trick that lands earns the entry's closeness, and the pet's own level picks its band.**
/// The wiki: +1..+3 per successful command. The table's `inc` was recorded on 2026-09-13 and
/// unused until now.
#[test]
fn a_successful_pet_command_earns_closeness_and_levels_the_pet_up() {
    let (mut s, store, id) = gm_session();
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
    let mut cfg = (*s.config).clone();
    // sit: always succeeds, inc 3 (column 5 of the table line). bad: always fails, inc 1.
    cfg.pet_commands = crate::petcommands::PetCommands::parse(
        "5000006\t0\tsit\t100\t1\t9\t3\ts\trest0\tBark bark!\n\
         5000006\t4\tbad|no\t0\t1\t9\t1\tf\tangry\tHeh... heh...\n",
    );
    s.config = std::sync::Arc::new(cfg);
    s.last_position = Some((300, -50));
    s.on_pet_activate(&hex("509a18140100"));

    let out = s.handle(&gm_chat("sit"));
    assert!(out.iter().any(|r| r.opcode == net::pet::PET_ACTION));
    let st = store.pet_state(pet_of(&store, id, 5_000_006)).unwrap();
    assert_eq!((st.closeness, st.level), (3, 3), "+3 closeness, and 3 is level 3 in the table");
    assert!(out.iter().any(|r| r.what.contains("re-sent as pet 5000006")), "the panel's numbers go out");
    assert!(out.iter().any(|r| r.opcode == net::stats::USER_EFFECT_LOCAL && r.body[0] == net::pet::USER_EFFECT_PET), "a level gained by a trick flashes too");

    // A failed trick earns nothing.
    s.handle(&gm_chat("bad"));
    assert_eq!(store.pet_state(pet_of(&store, id, 5_000_006)).unwrap().closeness, 3);
}

/// **The pet's own loot request is `0x0205`, and it takes the drop.** The owner, 2026-09-15:
/// *"Husky also currently does not loot items on the ground."* Seven `0x0205`s in that run,
/// all logged UNKNOWN. The body is the capture's shape - `u32 petIdx, u8, u32 tick, u32, i16 x,
/// i16 y, u32 dropId, u32 crc, u32 itemId` - with this test's drop id at byte 17, where the
/// capture had `0x01312d00` = 20 000 000, this server's first drop id.
#[test]
fn the_pets_0x0205_loot_request_takes_a_mob_drop() {
    let (mut s, store, id) = gm_session();
    let map = crate::fields::FieldKey::world(net::opcode::START_MAP_ID);
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
    s.last_position = Some((520, 395));
    let (mob_drop, _) = s.fields.with_drops(map, |d| {
        d.drop_from_mob(crate::drops::DropFromMob {
            from_mob: true,
            map_id: map,
            owner_id: id,
            item: store::Item::bundle(4_000_019, 1),
            inv_type: store::InventoryType::Etc,
            meso: 0,
            x: 540,
            y: 395,
            source_x: 540,
            source_y: 380,
            now_ms: 1_000,
            party_id: 0,
        })
    });
    s.on_pet_activate(&hex("509a18140100"));
    assert!((s.active_pet_item() == Some(5_000_006)));

    // world-ch0.log 02:56:09.012, with the drop id swapped in at byte 17.
    let mut body = net::pet::CLIENT_PET_PICK_UP.to_le_bytes().to_vec();
    body.extend_from_slice(&hex("000000000001f14d200100000069ffd700"));
    body.extend_from_slice(&mob_drop.to_le_bytes());
    body.extend_from_slice(&hex("9667e331464b4c00"));
    assert_eq!(body.len(), 2 + 29, "the captured length");
    let out = s.handle(&body);
    let leave = out.iter().find(|r| r.opcode == net::drops::DROP_LEAVE_FIELD).expect("the drop leaves");
    assert_eq!(leave.body[4], net::drops::leave_type::PET_PICKUP, "type 5 - it flies into the pet");
    assert_eq!(s.fields.with_drops(map, |d| d.len()), 0, "and it is gone from the floor");
    assert!(
        out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION),
        "and the item reaches the bag: {:?}",
        out.iter().map(|r| &r.what).collect::<Vec<_>>()
    );
}

/// **A skill item teaches the pet and is used up.** The owner, 2026-09-15: Auto HP / MP / Move
/// "doesn't work" - they arrived on `0x0116` with the pet's serial and were kept as "not a
/// reset scroll". Now: the bit lands in `store::pets`, the skill item is gone, the pet's Cash
/// item goes back out carrying the mask, and - the pet being out - it is put away and
/// re-summoned for the owner so `CPet::Init` re-reads the item.
#[test]
fn a_pet_skill_item_sets_the_bit_and_is_used_up() {
    let (mut s, store, id) = gm_session();
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_190_000, 1), 2).unwrap();
    s.last_position = Some((300, -50));
    s.on_pet_activate(&hex("509a18140100"));

    // tick, the skill item's slot, Auto HP Potion Skill, the Husky's serial.
    let skill_slot = cash_slot_of(&store, id, 5_190_000);
    let out = s.handle(&use_pet_item_body(skill_slot, 5_190_000, id, pet_of(&store, id, 5_000_006), None));

    let state = store.pet_state(pet_of(&store, id, 5_000_006)).unwrap();
    assert_eq!(state.skills, net::bag::PET_SKILLS_LEARNED_AT_START | net::bag::PET_SKILL_AUTO_HP, "Item Pouch and Auto HP");
    let cash = store.bag_items(id, store::InventoryType::Cash).unwrap();
    assert!(cash.iter().all(|r| r.item.item_id != 5_190_000), "the skill item is used up: {cash:?}");
    assert!(out.iter().any(|r| r.what.contains("re-sent as pet 5000006")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    // The mask is in the item body the client reads: petSkill at 54 when the serial rides.
    let pet = bag_pet(&store, id, 5_000_006);
    let blob = s.item_blob_with_cash_sn(&pet, Some(net::pet::pet_serial(id, pet_of(&store, id, 5_000_006))));
    assert_eq!(&blob[54..56], &(net::bag::PET_SKILLS_LEARNED_AT_START | net::bag::PET_SKILL_AUTO_HP).to_le_bytes());
    let summons: Vec<u8> = out.iter().filter(|r| r.opcode == net::pet::PET_ACTIVATED).map(|r| r.body[8]).collect();
    assert_eq!(summons, vec![0, 1], "put away, then back out, for the owner - a fresh CPet::Init");
    assert!((s.active_pet_item() == Some(5_000_006)), "and it is still out");

    // A second skill adds to the mask rather than replacing it.
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_190_002, 1), 1).unwrap();
    let slot = cash_slot_of(&store, id, 5_190_002);
    s.handle(&use_pet_item_body(slot, 5_190_002, id, pet_of(&store, id, 5_000_006), None));
    assert_eq!(
        store.pet_state(pet_of(&store, id, 5_000_006)).unwrap().skills,
        net::bag::PET_SKILLS_LEARNED_AT_START | net::bag::PET_SKILL_AUTO_HP | net::bag::PET_SKILL_AUTO_MOVE,
        "Auto Move is bought, not born with - the owner, 2026-09-16 evening"
    );
}

/// **The serial the field-entry restore sends for a pet is the one the skill item comes back
/// with - and the old bag serial still names the pet.** The live server, 2026-09-18 02:28:
/// Cobalt used Auto HP on the Husky in Cash slot 1 and the `0x0116` carried
/// `0x400000D5_00050001` - the generic BAG serial (mark, character 213, Cash, slot 1) the
/// restore had put on the pet item - and the answer was "That skill needs a pet to learn
/// it". Moth's worked because a summon had re-sent their pet with the PET serial. Now the
/// restore sends the pet serial too (`bag_item_blob`), and `pet_named_by` reads the bag
/// serial form as well, for a client that still holds one.
#[test]
fn a_pet_restored_at_field_entry_is_named_by_the_serial_it_was_sent_with_and_by_the_old_bag_serial() {
    let (mut s, store, id) = gm_session();
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_190_000, 1), 2).unwrap();
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_190_001, 1), 2).unwrap();
    let pet_id = pet_of(&store, id, 5_000_006);
    let pet_slot = cash_slot_of(&store, id, 5_000_006);

    // 1. What the restore sends: the blob `bag_item_blob` builds for the pet's own slot
    //    carries the PET serial, not the bag one. Bytes 6..14 of a type-3 body.
    let pet = bag_pet(&store, id, 5_000_006);
    let blob = s.bag_item_blob(store::InventoryType::Cash, pet_slot, &pet);
    let sent = u64::from_le_bytes(blob[6..14].try_into().unwrap());
    assert_eq!(sent, net::pet::pet_serial(id, pet_id).get(), "the restore's serial is the pet serial");

    // 2. The skill item, with exactly that serial, teaches the pet - no summon in between.
    let skill_slot = cash_slot_of(&store, id, 5_190_000);
    let out = s.handle(&use_pet_item_body(skill_slot, 5_190_000, id, pet_id, None));
    assert!(out.iter().all(|r| !r.what.contains("needs a pet")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert_eq!(store.pet_state(pet_id).unwrap().skills & net::bag::PET_SKILL_AUTO_HP, net::bag::PET_SKILL_AUTO_HP);

    // 3. Cobalt's packet shape: the OLD bag serial for the pet's slot (mark | character in the
    //    high dword, Cash << 16 | slot in the low) still names the pet.
    let bag_serial = ((0x4000_0000u64 | u64::from(id)) << 32) | (5u64 << 16) | u64::from(pet_slot);
    let mut body = hex("f98e4e20");
    let mp_slot = cash_slot_of(&store, id, 5_190_001);
    body.extend_from_slice(&mp_slot.to_le_bytes());
    body.extend_from_slice(&5_190_001u32.to_le_bytes());
    body.extend_from_slice(&bag_serial.to_le_bytes());
    let mut packet = 0x0116u16.to_le_bytes().to_vec();
    packet.extend_from_slice(&body);
    let out = s.handle(&packet);
    assert!(out.iter().all(|r| !r.what.contains("needs a pet")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert_eq!(store.pet_state(pet_id).unwrap().skills & net::bag::PET_SKILL_AUTO_MP, net::bag::PET_SKILL_AUTO_MP, "Auto MP learned through the bag serial");
    // A bag serial for a slot with no pet in it names nobody.
    let empty = ((0x4000_0000u64 | u64::from(id)) << 32) | (5u64 << 16) | 40;
    assert!(s.pet_named_by(&s.claimed_character().unwrap(), Some(empty)).is_none());
}

/// **A pet's re-send names the slot the pet is in NOW, not the one it was summoned from.**
/// The live server, 2026-09-18 02:28 (Cobalt): summoned from Cash slot 3, slid to slot 1 by
/// Consolidate, and every later re-send wrote a pet item over slot 3 while the restore put
/// the pet in slot 1 - the client, handed a pet where it had a coupon, printed "Closeness
/// has increased (+1)" on every map change. The owner: *"Pet closeness increase should only be
/// sent to the client if it is actually being increased."* The server never sends that line;
/// the client derives it from a pet item landing where none was.
#[test]
fn a_pet_re_send_after_the_pet_changed_slots_names_the_new_slot() {
    let (mut s, store, id) = gm_session();
    let cash = store::InventoryType::Cash;
    store.set_inventory_slot(id, cash, 1, &store::Item::bundle(5_150_000, 1)).unwrap();
    store.set_inventory_slot(id, cash, 2, &store::Item::bundle(5_150_000, 1)).unwrap();
    store.add_item(id, cash, &store::Item::bundle(5_000_006, 1), 1).unwrap(); // numbered, lands in slot 3
    assert_eq!(cash_slot_of(&store, id, 5_000_006), 3);
    s.last_position = Some((300, -50));
    s.on_pet_activate(&hex("509a18140300"));
    assert_eq!(s.active_pet.map(|p| p.slot), Some(3), "summoned from slot 3");

    // Sort Items on the Cash tab: the two coupons merge into one stack of 2 and lead, the
    // Husky slides up behind them - out of slot 3 either way.
    let mut packet = net::inventory::CLIENT_SORT_ITEMS.to_le_bytes().to_vec();
    packet.extend_from_slice(&[0x5c, 0xc5, 0x4b, 0x00, cash.as_u8()]);
    let _ = s.handle(&packet);
    let now = cash_slot_of(&store, id, 5_000_006);
    assert_ne!(now, 3, "the pet moved");
    assert_eq!(s.active_pet.map(|p| p.slot), Some(3), "the summon's number is stale - by design the send-time lookup covers it");

    // A field entry re-sends the pet item: it must name the slot the pet is in, never slot 3.
    let out = s.on_field_entered();
    let re_sends: Vec<&Reply> = out.iter().filter(|r| r.what.contains("re-sent as pet 5000006")).collect();
    assert!(!re_sends.is_empty(), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    for r in &re_sends {
        let slot = i16::from_le_bytes([r.body[9], r.body[10]]);
        assert_eq!(slot, now as i16, "the re-send names the slot the pet is in now: {}", r.what);
    }
}

/// **The in-range vacuum is free: every pet's item is `wonderGrade 6`, and the `0x0198` box
/// rides every SetField.** The owner, 2026-09-17: *"vacuuming loot within a certain range of the pet
/// (Petite Luna) should be free. Auto move ... should be a skill ... Expanded auto move
/// (longRange) ... should also remain a skill."* So the box is not gated on any purchase: a
/// fresh Husky reads 6 at the wonderGrade field (byte 69 with the serial riding, 61 + 8), and
/// buying Expanded Auto Move sets its skill bit for the pet's *movement* without touching the
/// grade. The box's own bytes ride `0x0198` after every SetField, because the client's copy is
/// `(0,0,0,0)` until told.
#[test]
fn the_vacuum_is_free_and_the_box_rides_every_set_field() {
    let (mut s, store, id) = gm_session();
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_190_003, 1), 2).unwrap();
    s.last_position = Some((300, -50));
    s.on_pet_activate(&hex("509a18140100"));
    let pet_id = pet_of(&store, id, 5_000_006);
    let serial = Some(net::pet::pet_serial(id, pet_id));

    // A fresh pet: Item Pouch only, but already wonderGrade 6 - the vacuum is free.
    let pet = bag_pet(&store, id, 5_000_006);
    let blob = s.item_blob_with_cash_sn(&pet, serial);
    assert_eq!(&blob[54..56], &net::bag::PET_SKILL_ITEM_POUCH.to_le_bytes(), "born with Item Pouch alone");
    assert_eq!(&blob[69..71], &net::bag::PET_WONDER_GRADE_VACUUM.to_le_bytes(), "wonderGrade 6 from the start: the free Petite Luna vacuum");

    // Buying Expanded Auto Move sets the movement bit; the grade is unchanged (still 6).
    let slot = cash_slot_of(&store, id, 5_190_003);
    let out = s.handle(&use_pet_item_body(slot, 5_190_003, id, pet_id, None));
    assert_eq!(
        store.pet_state(pet_id).unwrap().skills,
        net::bag::PET_SKILL_ITEM_POUCH | net::bag::PET_SKILL_EXPANDED_AUTO_MOVE
    );
    let pet = bag_pet(&store, id, 5_000_006);
    let blob = s.item_blob_with_cash_sn(&pet, serial);
    assert_eq!(&blob[69..71], &net::bag::PET_WONDER_GRADE_VACUUM.to_le_bytes(), "still 6 - the movement skill does not change the vacuum grade");
    let summons: Vec<u8> = out.iter().filter(|r| r.opcode == net::pet::PET_ACTIVATED).map(|r| r.body[8]).collect();
    assert_eq!(summons, vec![0, 1], "put away and back out, so CPet::Init re-reads the learned skill");

    // The box: after every SetField, 36 bytes, the near box first. A warp is one SetField;
    // the field table has to know the map or !map refuses rather than strand the character.
    let mut fields = std::collections::HashSet::new();
    fields.insert(40u32);
    s.config = Arc::new(Config { fields, ..(*s.config).clone() });
    let field = s.handle(&gm_chat("!map 40"));
    assert!(field.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "the warp happens: {:?}", field.iter().map(|r| &r.what).collect::<Vec<_>>());
    let range = field.iter().find(|r| r.opcode == net::pet::PET_PICKUP_RANGE).expect("0x0198 after SetField");
    assert_eq!(range.body.len(), 36);
    assert_eq!(&range.body[..16], &net::pet::pet_pickup_range(net::pet::PET_VACUUM_BOX, net::pet::PET_VACUUM_BOX, &[])[..16]);
    assert_eq!(&range.body[32..36], &[0, 0, 0, 0], "no item selects the far box");
}

/// **A Pet Name Tag renames the pet everywhere it is.** The owner, 2026-09-15: *"I also tried to
/// rename Husky into Dummy using the Pet Name Tag."* The tag's `0x0116` carries the serial and
/// the name (`world-ch0.log` 02:57:46). Now the name is stored, the tag is used up, the Cash item
/// goes back out under the new name, `0x027B` renames the pet on screen, and the next `0x0277`
/// - a field entry, a later arrival - already says "Dummy".
#[test]
fn a_pet_name_tag_renames_the_pet_and_the_name_sticks() {
    let (mut s, store, id) = gm_session();
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_170_000, 1), 1).unwrap();
    s.last_position = Some((300, -50));
    s.on_pet_activate(&hex("509a18140100"));

    let tag_slot = cash_slot_of(&store, id, 5_170_000);
    let out = s.handle(&use_pet_item_body(tag_slot, 5_170_000, id, pet_of(&store, id, 5_000_006), Some("Dummy")));

    assert_eq!(store.pet_state(pet_of(&store, id, 5_000_006)).unwrap().name.as_deref(), Some("Dummy"));
    assert!(
        store.bag_items(id, store::InventoryType::Cash).unwrap().iter().all(|r| r.item.item_id != 5_170_000),
        "the tag is used up"
    );
    let renamed = out.iter().find(|r| r.opcode == net::pet::PET_NAME_CHANGED).expect("0x027B to the owner");
    assert_eq!(&renamed.body[0..4], &id.to_le_bytes());
    assert_eq!(&renamed.body[8..], &hex("050044756d6d79"), "u16 length, then Dummy");
    // The re-sent Cash item and the next summon both carry it.
    let refreshed = out.iter().find(|r| r.what.contains("re-sent as pet 5000006")).expect("the Cash item again");
    assert!(refreshed.body.windows(5).any(|w| w == b"Dummy"), "the item body carries the name");
    let chr = s.claimed_character().unwrap();
    let again = s.pet_entry_replies(&chr);
    assert!(again[0].body.windows(5).any(|w| w == b"Dummy"), "the field pet is Dummy now");
    assert!(!again[0].body.windows(5).any(|w| w == b"Husky"));

    // Too long is cut to the wire's 12 bytes, not refused.
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_170_000, 1), 1).unwrap();
    let tag_slot = cash_slot_of(&store, id, 5_170_000);
    s.handle(&use_pet_item_body(tag_slot, 5_170_000, id, pet_of(&store, id, 5_000_006), Some("ThisNameIsFarTooLong")));
    assert_eq!(store.pet_state(pet_of(&store, id, 5_000_006)).unwrap().name.as_deref(), Some("ThisNameIsFa"));
}

/// **A pet that was out at log-out is out at the next login.** The owner, 2026-09-15: *"Pets that
/// were spawned from before does not survive a re-login, pets that were previously summoned by
/// user should keep their state."* A second session on the same store claims the same character:
/// the pet is active before the record is built (its Cash item says so), and the first field
/// entry re-sends the `0x0277`. Put away, and the next login leaves it in the bag.
#[test]
fn a_pet_that_was_out_is_out_again_after_a_relogin() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Wisp".to_string(), ..Default::default() };
    let id = store.create_character(account, 0, &chr).unwrap().id;
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();

    // Session one: summon, then the connection goes away.
    store.create_migration(account, id, 0, 0).unwrap();
    let mut first = Session::new(store.clone(), Arc::new(Config::default()));
    first.claim_for_character(id);
    first.last_position = Some((300, -50));
    first.on_pet_activate(&hex("509a18140100"));
    assert!((first.active_pet_item() == Some(5_000_006)));
    assert_eq!(store.active_pet(id).unwrap().map(|p| p.item_id), Some(5_000_006), "remembered");
    drop(first);

    // Session two, same character: out before anything is sent.
    store.create_migration(account, id, 0, 0).unwrap();
    let mut second = Session::new(store.clone(), Arc::new(Config::default()));
    assert!(second.claim_for_character(id).contains("claimed the migration"));
    assert!((second.active_pet_item() == Some(5_000_006)), "restored at claim time, before the login SetField");
    let pet = bag_pet(&store, id, 5_000_006);
    let blob = second.item_blob_with_cash_sn(&pet, Some(net::pet::pet_serial(id, pet_of(&store, id, 5_000_006))));
    assert_eq!(blob[1 + 4 + 1 + 8 + 8 + 4 + 1 + 13 + 1 + 2 + 1 + 8 + 2 + 2 + 4 + 2], 1, "the Cash item's active byte is 1 in the record");
    let entered = second.on_field_entered();
    let summon = entered.iter().find(|r| r.opcode == net::pet::PET_ACTIVATED).expect("the pet is re-summoned on the first field entry");
    assert_eq!(summon.body[8], 1, "activated");

    // Put away in session two: session three finds it in the bag.
    second.on_pet_activate(&hex("f29d18140100"));
    assert_eq!(store.active_pet(id).unwrap(), None);
    store.create_migration(account, id, 0, 0).unwrap();
    let mut third = Session::new(store.clone(), Arc::new(Config::default()));
    third.claim_for_character(id);
    assert!(!(third.active_pet_item() == Some(5_000_006)));
    assert!(third.on_field_entered().iter().all(|r| r.opcode != net::pet::PET_ACTIVATED));
}

/// **Two Huskies are two pets.** The owner, 2026-09-16: *"Two Husky should not share the same
/// name. The pets should in the background have different ids to identify them apart."*
///
/// Same item id in two Cash slots. The name tag used on the second names the second only;
/// the summon from slot 2 is the second (its serial carries its own id, its Cash item is the
/// one with the active byte); a hunger tick moves the second's fullness only; a new session
/// re-summons the second, by id, with its name - and the first is still "Husky" at 100.
#[test]
fn two_huskies_have_two_names_and_the_right_one_comes_back() {
    const HUSKY: u32 = 5_000_006;
    const ACTIVE: usize = 62;
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Wisp".to_string(), ..Default::default() };
    let id = store.create_character(account, 0, &chr).unwrap().id;
    let a = store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(HUSKY, 1), 1).unwrap()[0].item.pet_id.unwrap();
    let b = store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(HUSKY, 1), 1).unwrap()[0].item.pet_id.unwrap();
    assert_ne!(a, b, "two numbers");
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_170_000, 1), 1).unwrap(); // a name tag, slot 3
    let mut names = std::collections::HashMap::new();
    names.insert(HUSKY, "Husky".to_string());
    names.insert(5_170_000, "Pet Name Tag".to_string());
    let config = Arc::new(Config { item_names: names, ..Config::default() });
    let row = |slot: u16| store.bag_items(id, store::InventoryType::Cash).unwrap().into_iter().find(|r| r.slot == slot).unwrap().item;
    let name_on_wire = |s: &Session, slot: u16| {
        let item = row(slot);
        let blob = s.item_blob_with_cash_sn(&item, Some(net::pet::pet_serial(id, item.pet_id.unwrap())));
        let name = &blob[27..40];
        let end = name.iter().position(|&c| c == 0).unwrap_or(13);
        (String::from_utf8_lossy(&name[..end]).into_owned(), blob[ACTIVE])
    };
    let summon = |slot: u16| { let mut b = 0x1418_9a50u32.to_le_bytes().to_vec(); b.extend_from_slice(&slot.to_le_bytes()); b };

    store.create_migration(account, id, 0, 0).unwrap();
    let mut s = Session::new(store.clone(), config.clone());
    s.claim_for_character(id);
    s.last_position = Some((300, -50));
    assert_eq!(name_on_wire(&s, 1), ("Husky".to_string(), 0));
    assert_eq!(name_on_wire(&s, 2), ("Husky".to_string(), 0));

    // Summon the SECOND: its serial is its own id, its item is the one marked active.
    let out = s.on_pet_activate(&summon(2));
    let up = out.iter().find(|r| r.opcode == net::pet::PET_ACTIVATED).expect("the summon");
    assert_eq!(&up.body[4 + 4 + 1 + 1 + 4 + 2 + 5..][..8], &net::pet::pet_serial(id, b).get().to_le_bytes(), "the field pet carries pet b's serial");
    assert_eq!(store.active_pet(id).unwrap().map(|p| p.pet_id), Some(b));
    assert_eq!(name_on_wire(&s, 1).1, 0, "slot 1's Husky is not the one out");
    assert_eq!(name_on_wire(&s, 2).1, 1, "slot 2's Husky is");

    // The name tag used on pet b names pet b only.
    s.handle(&use_pet_item_body(3, 5_170_000, id, b, Some("Dummy")));
    assert_eq!(store.pet_state(b).unwrap().name.as_deref(), Some("Dummy"));
    assert_eq!(store.pet_state(a).unwrap().name, None, "the other Husky is still called Husky");
    assert_eq!(name_on_wire(&s, 1).0, "Husky");
    assert_eq!(name_on_wire(&s, 2).0, "Dummy");

    // Hunger moves pet b's fullness and nobody else's.
    s.pet_hunger_tick(net::petfood::PET_HUNGER_INTERVAL_MS);
    assert_eq!((store.pet_state(a).unwrap().fullness, store.pet_state(b).unwrap().fullness), (100, 99));

    // Swap slots 1 and 2: the numbers travel, the active one is still b, now in slot 1.
    s.on_inventory_move(&inventory_move(5, 1, 2, 1));
    assert_eq!(row(1).pet_id, Some(b));
    assert_eq!(row(2).pet_id, Some(a));
    drop(s);

    // A new session re-summons pet b BY ID - in slot 1 now - under its name; a is untouched.
    store.create_migration(account, id, 0, 0).unwrap();
    let mut again = Session::new(store.clone(), config);
    again.claim_for_character(id);
    assert_eq!(name_on_wire(&again, 1), ("Dummy".to_string(), 1));
    assert_eq!(name_on_wire(&again, 2), ("Husky".to_string(), 0));
    let entered = again.on_field_entered();
    let summons: Vec<&Reply> = entered.iter().filter(|r| r.opcode == net::pet::PET_ACTIVATED).collect();
    assert_eq!(summons.len(), 1);
    assert!(summons[0].what.contains("Dummy"), "{}", summons[0].what);
    assert_eq!(&summons[0].body[4 + 4 + 1 + 1 + 4 + 2 + 5..][..8], &net::pet::pet_serial(id, b).get().to_le_bytes());
}

/// **The pet is settled on the first move after a field entry: put away, summoned, item
/// re-sent - once.** The owner, 2026-09-18: *"the vacuum functionality does not work until the pet
/// is re-summoned or fed at least once ... Can we have the vacuum functionality always be
/// present when the pet is summoned please?"* The field-entry batch already sends a summon
/// and the item; what a re-summon and a feed have that it lacks is landing on a pet the
/// client has finished building. The client's first move after the entry is the earliest
/// packet that proves it has, so that is when the working sequence goes out - and only then.
#[test]
fn the_pet_is_resummoned_and_its_item_resent_on_the_first_move_after_a_field_entry() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Wisp".to_string(), ..Default::default() };
    let id = store.create_character(account, 0, &chr).unwrap().id;
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
    store.create_migration(account, id, 0, 0).unwrap();
    let mut s = Session::new(store.clone(), Arc::new(Config::default()));
    s.claim_for_character(id);
    s.last_position = Some((300, -50));
    let a_move = || { let mut b = net::usermove::CLIENT_USER_MOVE.to_le_bytes().to_vec(); b.extend_from_slice(&[0u8; 8]); b };
    let pet_replies = |out: &[Reply]| out.iter().filter(|r| r.opcode == net::pet::PET_ACTIVATED || r.opcode == net::inventory::INVENTORY_OPERATION).map(|r| (r.opcode, r.body[8])).collect::<Vec<_>>();

    // No pet out: a move sends nothing.
    assert!(pet_replies(&s.handle(&a_move())).is_empty());

    // Summon, then a field entry (a portal, a re-login): the entry's own summon + item write...
    s.on_pet_activate(&hex("509a18140100"));
    assert!(s.pet_is_active(pet_of(&store, id, 5_000_006)));
    let entered = s.on_field_entered();
    assert!(entered.iter().any(|r| r.opcode == net::pet::PET_ACTIVATED), "the entry summons");
    // ...and then, on the FIRST move, the settle: put away, summoned, item re-sent - the
    // sequence a re-summon and a feed use.
    let out = s.handle(&a_move());
    let kinds: Vec<(u16, u8)> = pet_replies(&out);
    assert_eq!(kinds.len(), 3, "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert_eq!((kinds[0].0, kinds[0].1), (net::pet::PET_ACTIVATED, 0), "put away");
    assert_eq!((kinds[1].0, kinds[1].1), (net::pet::PET_ACTIVATED, 1), "summoned again");
    assert_eq!(kinds[2].0, net::inventory::INVENTORY_OPERATION, "and the item, so CPet re-reads it");
    assert!(out.iter().any(|r| r.what.contains("re-sent as pet 5000006")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    // The second move: nothing. The settle is once per entry.
    assert!(pet_replies(&s.handle(&a_move())).is_empty(), "once");
    // A later field entry arms it again.
    s.on_field_entered();
    assert_eq!(pet_replies(&s.handle(&a_move())).len(), 3, "re-armed by the next entry");
    // A pet put away before the first move: the entry armed it, and it sends nothing.
    s.on_field_entered();
    s.on_pet_activate(&hex("509a18140100"));
    assert!(s.active_pet_item().is_none());
    assert!(pet_replies(&s.handle(&a_move())).is_empty(), "no pet out, nothing to settle");
}

/// **Two pets, two sets of vitals, one out at a time, and all of it survives a re-login.**
///
/// The owner, 2026-09-16: *"double check and make sure that pet fullness and closeness is
/// persisted through logins. A player should be able to have multiple pets with varying
/// amounts of fullness and closeness. Summoning a new pet onto the field should unsummon the
/// old pet. Only 1 active pet at a time."* This walks it with the session's own writes - the
/// five-minute hunger tick - rather than poking the store, so a write that stopped reaching
/// the store would fail here. The vitals are read back off the Cash item blob the next
/// session builds, which is the record the client draws from.
///
/// **Two pets of the SAME species are one pet to this server** - the store keys on
/// `(character, item id)` and the client-facing serial is `(character << 32) | item id` -
/// and this test says so rather than leaving it implicit.
#[test]
fn two_pets_keep_their_own_vitals_across_a_relogin_and_only_one_is_out() {
    const HUSKY: u32 = 5_000_006;
    const OTHER: u32 = 5_000_001;
    // Offsets into `net::bag::pet_item_with_state` with a cash serial: 27 bytes of head, 13 of
    // name, then u8 level, u16 closeness, u8 fullness; active is at 62.
    const LEVEL: usize = 40;
    const CLOSENESS: usize = 41;
    const FULLNESS: usize = 43;
    const ACTIVE: usize = 62;
    let tick = net::petfood::PET_HUNGER_INTERVAL_MS;

    let store = Arc::new(Store::open_in_memory().unwrap());
    let account = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Wisp".to_string(), ..Default::default() };
    let id = store.create_character(account, 0, &chr).unwrap().id;
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(HUSKY, 1), 1).unwrap(); // slot 1
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(OTHER, 1), 1).unwrap(); // slot 2
    let blob = |s: &Session, item: u32| s.item_blob_with_cash_sn(&bag_pet(&store, id, item), Some(net::pet::pet_serial(id, pet_of(&store, id, item))));
    let vitals = |s: &Session, item: u32| { let b = blob(s, item); (b[LEVEL], u16::from_le_bytes([b[CLOSENESS], b[CLOSENESS + 1]]), b[FULLNESS], b[ACTIVE]) };
    let summon = |slot: u16| { let mut b = 0x1418_9a50u32.to_le_bytes().to_vec(); b.extend_from_slice(&slot.to_le_bytes()); b };

    // ---- session one: the Husky out, one hunger tick; then the other pet swaps it out ----
    store.create_migration(account, id, 0, 0).unwrap();
    let mut first = Session::new(store.clone(), Arc::new(Config::default()));
    first.claim_for_character(id);
    first.last_position = Some((300, -50));
    first.on_pet_activate(&summon(1));
    assert!((first.active_pet_item() == Some(HUSKY)));
    first.pet_hunger_tick(tick); // the summon armed the timer; this is the five-minute hunger
    assert_eq!(vitals(&first, HUSKY), (1, 0, 99, 1), "Husky: level 1, closeness 0, fullness 99, out");
    assert_eq!(vitals(&first, OTHER), (1, 0, 100, 0), "the other pet is untouched and in the bag");

    // Summoning the other pet puts the Husky away in the same reply: ONE pet out.
    let out = first.on_pet_activate(&summon(2));
    let pets: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::pet::PET_ACTIVATED).collect();
    assert_eq!(pets.len(), 2, "the Husky's put-away and the other's summon: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(pets[0].what.contains("put away") && pets[0].what.contains(&HUSKY.to_string()), "{}", pets[0].what);
    assert!(pets[1].what.contains("summoned") && pets[1].what.contains(&OTHER.to_string()), "{}", pets[1].what);
    assert!((first.active_pet_item() == Some(OTHER)) && !(first.active_pet_item() == Some(HUSKY)));
    assert_eq!(store.active_pet(id).unwrap().map(|p| p.item_id), Some(OTHER), "the store agrees: one active row");
    assert!(!store.pet_state(pet_of(&store, id, HUSKY)).unwrap().active);
    // Two ticks on the other pet: its own fullness moves, the Husky's does not.
    first.pet_hunger_tick(3 * tick);
    first.pet_hunger_tick(4 * tick);
    assert_eq!(vitals(&first, OTHER), (1, 0, 98, 1));
    assert_eq!(vitals(&first, HUSKY), (1, 0, 99, 0));
    drop(first);

    // ---- session two: both sets of vitals come back, and only the other pet is out ----
    store.create_migration(account, id, 0, 0).unwrap();
    let mut second = Session::new(store.clone(), Arc::new(Config::default()));
    second.claim_for_character(id);
    assert_eq!(vitals(&second, HUSKY), (1, 0, 99, 0), "the Husky's own fullness, in the bag");
    assert_eq!(vitals(&second, OTHER), (1, 0, 98, 1), "the other pet's own fullness, and it is the one out");
    let entered = second.on_field_entered();
    let summons: Vec<&Reply> = entered.iter().filter(|r| r.opcode == net::pet::PET_ACTIVATED).collect();
    assert_eq!(summons.len(), 1, "exactly one pet is re-summoned on login");
    assert!(summons[0].what.contains(&OTHER.to_string()), "{}", summons[0].what);
    // Closeness persists the same way: a trick that lands writes it, and the next session
    // reads it back on the right pet.
    let st = store.pet_state(pet_of(&store, id, OTHER)).unwrap();
    store.set_pet_vitals(pet_of(&store, id, OTHER), st.level, 5, st.fullness).unwrap();
    store.create_migration(account, id, 0, 0).unwrap();
    let mut third = Session::new(store.clone(), Arc::new(Config::default()));
    third.claim_for_character(id);
    assert_eq!(vitals(&third, OTHER), (1, 5, 98, 1));
    assert_eq!(vitals(&third, HUSKY), (1, 0, 99, 0), "and the Husky did not gain it");
}

#[test]
fn a_summoned_pet_picks_up_a_mob_drop_but_not_a_players_own_drop() {
    let (mut s, store, id) = gm_session();
    let map = crate::fields::FieldKey::world(net::opcode::START_MAP_ID);
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
    s.last_position = Some((520, 395));

    // A mob's drop on the floor: the enter packet says a pet may take it.
    let (mob_drop, enter) = s.fields.with_drops(map, |d| {
        d.drop_from_mob(crate::drops::DropFromMob {
            from_mob: true,
            map_id: map,
            owner_id: id,
            item: store::Item::bundle(4_000_019, 1),
            inv_type: store::InventoryType::Etc,
            meso: 0,
            x: 540,
            y: 395,
            source_x: 540,
            source_y: 380,
            now_ms: 1_000,
            party_id: 0,
        })
    });
    assert_eq!(enter.body[102], 1, "canBePickedUpByPet for a mob's drop");

    // A pet-shaped request (the reference's shape: id at byte 17) on a sibling opcode.
    let pet_request = |object_id: u32| {
        let mut body = 0x032Du16.to_le_bytes().to_vec();
        body.extend_from_slice(&[0u8; net::drops::PET_PICK_UP_OBJECT_ID_AT]);
        body.extend_from_slice(&object_id.to_le_bytes());
        body.extend_from_slice(&[0u8; 4]);
        body
    };

    // No pet out: the pet shape names nothing, the gate is cleared, the drop stays.
    let out = s.handle(&pet_request(mob_drop));
    assert_eq!(out[0].opcode, net::inventory::INVENTORY_OPERATION);
    assert!(out.iter().all(|r| r.opcode != net::drops::DROP_LEAVE_FIELD));
    assert_eq!(s.fields.with_drops(map, |d| d.len()), 1);

    // Summon the Husky, then the same request: taken by the pet.
    s.on_pet_activate(&hex("509a18140100"));
    assert!((s.active_pet_item() == Some(5_000_006)));
    let out = s.handle(&pet_request(mob_drop));
    let leave = out.iter().find(|r| r.opcode == net::drops::DROP_LEAVE_FIELD).expect("a leave");
    assert_eq!(leave.body[4], net::drops::leave_type::PET_PICKUP, "type 5 - it flies into the pet");
    assert_eq!(&leave.body[5..9], &id.to_le_bytes());
    assert_eq!(&leave.body[9..13], &net::pet::PET_INDEX.to_le_bytes());
    assert!(out.iter().any(|r| r.opcode == net::message::MESSAGE), "the earned line");
    assert!(out.iter().all(|r| r.opcode != net::notice::CHAT_NOTICE), "nothing in the chat log");
    let etc: Vec<u32> = store.bag(id).unwrap().items_in(store::InventoryType::Etc).map(|i| i.item.item_id).collect();
    assert_eq!(etc, vec![4_000_019], "in the bag");
    assert_eq!(s.fields.with_drops(map, |d| d.len()), 0, "off the floor");


    // A player's own ground drop: the byte is clear, and the pet is refused without a word.
    s.handle(&gm_chat("!item 1302000"));
    let out = s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, 1, 0, 1));
    let enter = out.iter().find(|r| r.opcode == net::drops::DROP_ENTER_FIELD).expect("the ground drop");
    assert_eq!(enter.body[102], 0, "a player's drop is not for pets");
    let own_drop = s.fields.with_drops(map, |d| d.on_field(map).map(|x| x.object_id).next().unwrap());
    let out = s.handle(&pet_request(own_drop));
    assert_eq!(out.len(), 1, "the unlock and nothing else: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert_eq!((out[0].opcode, out[0].body[0]), (net::inventory::INVENTORY_OPERATION, 1));
    assert_eq!(s.fields.with_drops(map, |d| d.len()), 1, "still on the floor");
    // And the player can still pick their own drop up by hand.
    let mut by_hand = crate::drops::CLIENT_DROP_PICK_UP.to_le_bytes().to_vec();
    by_hand.extend_from_slice(&[0u8; crate::drops::PICK_UP_OBJECT_ID_AT]);
    by_hand.extend_from_slice(&own_drop.to_le_bytes());
    by_hand.extend_from_slice(&[0u8; 17]);
    let out = s.handle(&by_hand);
    let leave = out.iter().find(|r| r.opcode == net::drops::DROP_LEAVE_FIELD).expect("a leave");
    assert_eq!(leave.body[4], net::drops::leave_type::CHAR_PICKUP);

    // **A full tab: the pet is refused in silence, the player in a sentence.** The owner,
    // 2026-09-16: the pet's retries were filling the chat log with "inventory 1 is full".
    // Fill the Equip tab with a distinct equip per slot (a stack would not fill it), drop
    // one more from a mob, and ask for it both ways. Both get the 0x0070 unlock - the
    // client stops asking for the rest of the session without it - and only the hand gets
    // the yellow line.
    let mut filled = 0u32;
    while store.add_item(id, store::InventoryType::Equip, &store::Item::equip(1_302_000 + filled), 1).is_ok() {
        filled += 1;
        assert!(filled < 1_000, "the Equip tab never fills");
    }
    assert!(filled >= 1, "the tab took at least one equip before refusing");
    let mob_equip = |s: &mut Session| {
        s.fields.with_drops(map, |d| {
            d.drop_from_mob(crate::drops::DropFromMob {
                from_mob: true,
                map_id: map,
                owner_id: id,
                item: store::Item::equip(1_302_016),
                inv_type: store::InventoryType::Equip,
                meso: 0,
                x: 540,
                y: 395,
                source_x: 540,
                source_y: 380,
                now_ms: 2_000,
                party_id: 0,
            })
        }).0
    };
    let full_drop = mob_equip(&mut s);
    let out = s.handle(&pet_request(full_drop));
    assert!(out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "the unlock");
    assert!(out.iter().all(|r| r.opcode != net::notice::CHAT_NOTICE), "nothing in the chat log: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    // The client's own "You can't get anymore items." - a 3-byte 0x0089 the client throttles
    // to one line per two seconds itself, so the pet's retries cannot spam it.
    let full = out.iter().find(|r| r.opcode == net::message::MESSAGE).expect("the client's inventory-full line");
    assert_eq!(full.body, net::message::inventory_full());
    assert!(out.iter().all(|r| r.opcode != net::drops::DROP_LEAVE_FIELD), "and the drop stays on the floor");
    assert_eq!(s.fields.with_drops(map, |d| d.len()), 1, "the one drop on the floor is the refused one");
    let mut by_hand_full = crate::drops::CLIENT_DROP_PICK_UP.to_le_bytes().to_vec();
    by_hand_full.extend_from_slice(&[0u8; crate::drops::PICK_UP_OBJECT_ID_AT]);
    by_hand_full.extend_from_slice(&full_drop.to_le_bytes());
    by_hand_full.extend_from_slice(&[0u8; 17]);
    let out = s.handle(&by_hand_full);
    assert!(out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION), "the unlock");
    assert!(out.iter().all(|r| r.opcode != net::notice::CHAT_NOTICE), "our own yellow line is gone for good: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    let line = out.iter().find(|r| r.opcode == net::message::MESSAGE).expect("the player's own click gets the client's line");
    assert_eq!(line.body, net::message::inventory_full());
    assert!(line.what.contains("full"), "{}", line.what);
    assert_eq!(s.fields.with_drops(map, |d| d.len()), 1, "still on the floor");
}

/// **A pet walks and answers to its name.** The owner, 2026-09-13: *"broadcast player pet movement
/// so other people can see pets moving even if it is not their own"* and *"if those messages
/// match as one of the pet commands, then the pet should respond accordingly."*
///
/// The move is forwarded to the map and NOT echoed to the owner, whose client drew the walk
/// itself. The chat line still goes out as chat, with the pet's answer beside it; a sentence
/// that merely contains a command word is only chat.
#[test]
fn a_summoned_pet_walks_for_the_map_and_answers_its_command_words() {
    let (mut s, store, id) = gm_session();
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
    let mut cfg = (*s.config).clone();
    cfg.pet_commands = crate::petcommands::PetCommands::parse(
        "5000006\t0\tsit\t100\t1\t9\t1\ts\trest0\tBark bark!\n\
         5000006\t4\tbad|no\t0\t1\t9\t1\tf\tangry\tHeh... heh...\n",
    );
    s.config = std::sync::Arc::new(cfg);
    s.last_position = Some((300, -50));

    // Nothing answers until a pet is out.
    let body = hex("000000000000000000360112010000000001000036011201000000002a0000000000000004fe010000");
    assert!(s.on_pet_move(&body).is_empty());
    assert!(s.pet_command_replies("sit").is_empty(), "no pet, no trick");

    s.on_pet_activate(&hex("509a18140100"));
    assert!((s.active_pet_item() == Some(5_000_006)));

    // The move: nothing back to the owner, one PetMove on the map with the path untouched.
    assert!(s.on_pet_move(&body).is_empty(), "the owner's own client already drew it");

    // The command: the chat line AND the pet's answer, both out.
    let out = s.handle(&gm_chat("sit"));
    assert!(out.iter().any(|r| r.opcode == net::userchat::USER_CHAT), "it is still chat: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    let act = out.iter().find(|r| r.opcode == net::pet::PET_ACTION).expect("the pet answers");
    assert_eq!(&act.body[0..4], &id.to_le_bytes());
    assert_eq!(act.body[8], 0, "interact entry 0");
    assert_eq!(act.body[9], 1, "prob 100 always succeeds");
    assert!(String::from_utf8_lossy(&act.body).contains("Bark bark!"), "{}", act.what);

    // A word whose entry always fails still acts, with the fail line.
    let out = s.handle(&gm_chat("no"));
    let act = out.iter().find(|r| r.opcode == net::pet::PET_ACTION).expect("a failed trick is still a trick");
    assert_eq!((act.body[8], act.body[9]), (4, 0), "entry 4, failed");
    assert!(String::from_utf8_lossy(&act.body).contains("Heh"), "{}", act.what);

    // An ordinary sentence is only chat, even when it contains a command word.
    let out = s.handle(&gm_chat("sit down over there"));
    assert!(out.iter().any(|r| r.opcode == net::userchat::USER_CHAT));
    assert!(out.iter().all(|r| r.opcode != net::pet::PET_ACTION), "a sentence is not a command");

    // Put the pet away and the words go quiet again.
    s.on_pet_activate(&hex("f29d18140100"));
    assert!(!(s.active_pet_item() == Some(5_000_006)));
    let out = s.handle(&gm_chat("sit"));
    assert!(out.iter().all(|r| r.opcode != net::pet::PET_ACTION));
}

/// **The Wooden Boxes stand, break in four hits, drop, and come back.** The owner, 2026-09-13:
/// *"the items come out of breakable wooden boxes which we do not spawn right now. We need to
/// spawn them and provide the drops for the Wooden Box."* The shape is the client's reactor
/// pool (`net::reactor`): `0x0484` on entry, `0x0478` per hit, drops on the breaking hit,
/// `0x0485` + `0x0484` when `reactorTime` runs out.
#[test]
fn a_wooden_box_stands_on_entry_breaks_on_the_fourth_hit_drops_and_comes_back() {
    let (mut s, _store, _id) = claimed_session();
    let map = crate::fields::FieldKey::world(net::opcode::START_MAP_ID);
    let mut reactors = std::collections::HashMap::new();
    reactors.insert(map.map, vec![crate::config::ReactorSpawn {
        object_id: crate::config::REACTOR_OBJECT_ID_BASE,
        template_id: 1,
        x: 610,
        y: 259,
        respawn_s: 2,
        flip: false,
        break_at: 4,
        name: String::new(),
    }]);
    let reactor_drops = crate::droptables::DropTables::parse(
        "1 | 4031003 | 100 | 1 | 1 | 2 | Rusty Screw
         1 | 2010001 | 100 | 1 | 1 | 2 | Apple
",
    );
    s.config = Arc::new(Config { reactors, reactor_drops, ..(*s.config).clone() });
    let cfg = s.config.clone();
    s.fields.seed(map, &cfg, 0);

    // Field entry: the box, state 0, at its WZ position.
    let entry = s.on_field_entered();
    let boxes: Vec<&Reply> = entry.iter().filter(|r| r.opcode == net::reactor::REACTOR_ENTER_FIELD).collect();
    assert_eq!(boxes.len(), 1, "{:?}", entry.iter().map(|r| &r.what).collect::<Vec<_>>());
    let b = &boxes[0].body;
    assert_eq!(u32::from_le_bytes(b[1..5].try_into().unwrap()), crate::config::REACTOR_OBJECT_ID_BASE);
    assert_eq!(u32::from_le_bytes(b[5..9].try_into().unwrap()), 1, "Reactor.wz 0000001");
    assert_eq!(b[9], 0, "fresh");

    // The client's hit: u32 objectId, u32 hitOption, u16 delay, u32 skillId, behind 0x032F.
    let hit = |delay: u16| {
        let mut p = net::reactor::CLIENT_REACTOR_HIT.to_le_bytes().to_vec();
        p.extend_from_slice(&crate::config::REACTOR_OBJECT_ID_BASE.to_le_bytes());
        p.extend_from_slice(&0u32.to_le_bytes());
        p.extend_from_slice(&delay.to_le_bytes());
        p.extend_from_slice(&0u32.to_le_bytes());
        p
    };
    for expect_state in 1..=3u8 {
        let out = s.handle(&hit(150));
        assert_eq!(out.len(), 1, "one change of state and nothing else: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
        assert_eq!(out[0].opcode, net::reactor::REACTOR_CHANGE_STATE);
        assert_eq!(out[0].body[4], expect_state);
        assert_eq!(u16::from_le_bytes(out[0].body[9..11].try_into().unwrap()), 150, "the delay echoed");
    }
    // The fourth hit breaks it: state 4, then the two drops at the box.
    let out = s.handle(&hit(150));
    assert_eq!(out[0].opcode, net::reactor::REACTOR_CHANGE_STATE);
    assert_eq!(out[0].body[4], 4, "the broken state");
    let drops: Vec<&Reply> = out.iter().filter(|r| r.opcode == net::drops::DROP_ENTER_FIELD).collect();
    assert_eq!(drops.len(), 2, "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    assert!(s.fields.reactors_on(map).is_empty(), "a broken box is not standing");

    // A fifth hit hits nothing: no reactor by that id is standing.
    assert!(s.handle(&hit(150)).is_empty());

    // Not back before its time; back after it, as leave + enter with the same id.
    assert!(s.spawn_due_reactors(map, 1_999).is_empty());
    let back = s.spawn_due_reactors(map, 2_000);
    assert_eq!(back.iter().map(|r| r.opcode).collect::<Vec<_>>(), vec![net::reactor::REACTOR_LEAVE_FIELD, net::reactor::REACTOR_ENTER_FIELD]);
    assert_eq!(back[1].body[9], 0, "fresh again");
    assert_eq!(s.fields.reactors_on(map).len(), 1);

    // And the real table: 19 boxes on the six Amherst maps, every reactor the client ships.
    let path = std::path::Path::new("../../gm-handbook/reactors.txt");
    if path.exists() {
        let table = crate::config::Config::load_reactors(path);
        let boxes: usize = [1000u32, 1010, 1011, 1012, 1013, 1014].iter().map(|m| table.get(m).map(Vec::len).unwrap_or(0)).sum();
        assert_eq!(boxes, 19, "the fan site's count and the WZ's agree");
        assert!(table.values().flatten().all(|r| r.break_at >= 1 && r.respawn_s >= 1));
        assert_eq!(table[&1010][0].break_at, 4, "Reactor.wz 0000001: events on states 0..3");
    }
}

/// **A `prop`-marked reward is one draw from the pool, not the whole pool.** The owner, 2026-09-13:
/// *"When I finished 'Please bring this letter to Lucas', Maria gave me one of every single
/// Headband item when it's suppose to be choose 1 randomly from the pool."* Quest 1008 (Lucas's
/// Reply) takes the letter back and offers seven headbands at `prop 1` each.
#[test]
fn lucas_reply_gives_one_headband_from_the_pool_and_takes_the_letter() {
    let path = std::path::Path::new("../../gm-handbook/questlines.txt");
    if !path.exists() {
        return; // generated data, gitignored
    }
    let quests = crate::config::load_quests(path);
    let q = &quests[&1008];
    let pool: Vec<u32> = q.complete_rewards.iter().filter(|r| r.prop > 0).map(|r| r.id).collect();
    assert_eq!(pool, vec![1002000, 1002001, 1002003, 1002004, 1002005, 1002006, 1002007], "the seven hats, prop 1");
    assert_eq!(q.complete_rewards.iter().filter(|r| r.prop == 0).map(|r| (r.id, r.count)).collect::<Vec<_>>(), vec![(4031002, -1)], "the letter back, unconditional");
    // The rule, on the data: every roll gives the letter-take and exactly one hat; every hat
    // is reachable.
    let mut seen = std::collections::BTreeSet::new();
    for roll in 0..70u64 {
        let chosen = crate::config::choose_rewards(&q.complete_rewards, 0, roll);
        assert_eq!(chosen.len(), 2, "{chosen:?}");
        assert_eq!(chosen[0].id, 4031002);
        assert!(pool.contains(&chosen[1].id));
        seen.insert(chosen[1].id);
    }
    assert_eq!(seen.len(), 7, "all seven hats come up across the rolls");

    // Through the turn-in itself: the letter in the bag, the quest started, then completed.
    let (mut s, store, id) = claimed_session();
    s.config = Arc::new(Config { quests, ..(*s.config).clone() });
    store.add_item(id, store::InventoryType::Etc, &store::Item::bundle(4031002, 1), 1).unwrap();
    store.start_quest(id, 1008).unwrap();
    let out = s.record_quest_complete(1008, 1008);
    assert!(out.iter().any(|r| r.opcode == net::quest::MESSAGE), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    let bag = store.bag(id).unwrap();
    let hats: Vec<u32> = bag.items_in(store::InventoryType::Equip).map(|i| i.item.item_id).filter(|i| pool.contains(i)).collect();
    assert_eq!(hats.len(), 1, "ONE headband, not seven: {hats:?}");
    assert_eq!(bag.items_in(store::InventoryType::Etc).filter(|i| i.item.item_id == 4031002).count(), 0, "the letter went back");
}

/// The gender half of the same rule: a `gender 0` reward is for male characters, `gender 1`
/// for female, absent or 2 for anyone.
#[test]
fn a_gendered_reward_goes_only_to_the_gender_it_names() {
    use crate::config::{choose_rewards, RewardItem};
    let rewards = vec![
        RewardItem { id: 1, count: 1, prop: 0, gender: Some(0) },
        RewardItem { id: 2, count: 1, prop: 0, gender: Some(1) },
        RewardItem { id: 3, count: 1, prop: 0, gender: None },
        RewardItem { id: 4, count: 1, prop: 0, gender: Some(2) },
        RewardItem { id: 5, count: 1, prop: 3, gender: Some(0) },
        RewardItem { id: 6, count: 1, prop: 3, gender: Some(1) },
    ];
    let male: Vec<u32> = choose_rewards(&rewards, 0, 0).iter().map(|r| r.id).collect();
    assert_eq!(male, vec![1, 3, 4, 5]);
    let female: Vec<u32> = choose_rewards(&rewards, 1, 0).iter().map(|r| r.id).collect();
    assert_eq!(female, vec![2, 3, 4, 6]);
    // A pool with nothing eligible draws nothing rather than panicking.
    assert!(choose_rewards(&[RewardItem { id: 9, count: 1, prop: 1, gender: Some(1) }], 0, 5).is_empty());
}

/// **A pet is bought, sits in the locker with a type-3 body, and moves to the Cash tab as one.**
/// The owner, 2026-09-13: the eleven pets in the shop, permanent, never revived. Until today a pet
/// was refused at purchase; the body is `net::bag::pet_item_with_cash_sn`.
#[test]
fn a_pet_is_bought_as_a_type_3_item_that_never_dies() {
    let dir = std::env::temp_dir().join(format!("maplecw-pets-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("commodity.txt");
    std::fs::write(
        &path,
        "# sn, itemId, count, price, ...\n\
         160000003, 5000000, 1, 100, 100, , 0, 2, 1, , 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 600, Brown Kitty\n",
    )
    .unwrap();
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Owner".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    store.add_maple_points(account_id, 50_000).unwrap();
    let mut item_names = std::collections::HashMap::new();
    item_names.insert(5000000u32, "Brown Kitty".to_string());
    let config = Config {
        item_names,
        commodity: crate::commodity::CommodityTable::load(&path),
        ..Config::default()
    };
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(id);

    let out = s.handle(&buy_body(160000003));
    let bought = out.iter().find(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT).expect("a cash shop result");
    assert!(bought.what.contains("BOUGHT"), "not refused any more: {}", bought.what);
    // The locker record carries the pet body: type 3, the name, and a dateDead that is NOT the
    // ITEM_NEVER_EXPIRES sentinel - the client reads that sentinel here as "this pet is a doll".
    let body = &bought.body;
    let at = body.windows(1 + 4).position(|w| w[0] == net::bag::PET_ITEM_TYPE && w[1..5] == 5000000u32.to_le_bytes()).expect("a type-3 body for item 5000000 in the record");
    let pet = &body[at..];
    // The record's own header carries the serial, so the body inside it has hasCashSN 0 and
    // the name sits at 19 - the same place as in the bag blob.
    assert_eq!(pet[5], 0, "no serial inside the record's body");
    assert_eq!(&pet[19..30], b"Brown Kitty", "named after the item");
    assert_eq!(&pet[36..44], &net::bag::PET_DATE_DEAD.to_le_bytes(), "dateDead: alive, and below the sentinel");
    // The blob the bag will get for it is the same body: never expiring, and not yet dead.
    let item = store::Item::bundle(5000000, 1);
    let blob = s.item_blob(&item);
    assert_eq!(blob[0], net::bag::PET_ITEM_TYPE);
    assert_eq!(blob.len(), net::bag::PET_ITEM_LEN);
    assert_eq!(&blob[36..44], &net::bag::PET_DATE_DEAD.to_le_bytes(), "dateDead: alive, and below the sentinel");
    let _ = std::fs::remove_dir_all(&dir);
}

/// **The repeat-dialogue audit, over every quest the client ships.** The owner, 2026-09-13: *"Please
/// audit all of the questline and make sure repeat dialogue is no longer a concern."*
///
/// The rule the three fixes of 2026-09-13 converge on: **the client shows a quest's opening
/// (`Say.0`) itself, so the server must never send a `Say.0` line of THAT quest in answer to
/// its Accept (action 1) or its turn-in (action 2)**, and one request never opens two boxes at
/// once. What the server may send: the `0.yes` branch on Accept; the `Say.1` lines (or a quiz
/// menu) on turn-in; the NEXT quest's opening when a turn-in chains and the finished quest has
/// nothing of its own to say (1000 -> 1001, on screen 2026-08-20). Action 4, the opening script,
/// is the one place the server speaks `Say.0` - the client has no local text for a scripted
/// quest (27 captures, all quest 1002, all fine).
#[test]
fn no_quest_answers_its_accept_or_turn_in_with_its_own_opening_lines() {
    let path = std::path::Path::new("../../gm-handbook/questlines.txt");
    if !path.exists() {
        return; // generated data, gitignored
    }
    let quests = crate::config::load_quests(path);
    let config = Arc::new(Config { quests: quests.clone(), ..Config::default() });
    let request = |action: u8, quest: u32, npc: u32| -> Vec<u8> {
        let mut b = vec![action];
        b.extend_from_slice(&quest.to_le_bytes());
        b.extend_from_slice(&npc.to_le_bytes());
        b.extend_from_slice(&[0x0a, 0x01, 0x12, 0x01]); // x, y as captured
        b.extend_from_slice(if action == 2 { &[0xff; 4] } else { &[0; 4] });
        b
    };
    let own_opening = |replies: &[Reply], quest: u32| -> Option<String> {
        replies.iter().find(|r| {
            r.opcode == net::script::SCRIPT_MESSAGE
                && r.what.contains(&format!("for quest {quest},"))
                && r.what.contains("on path \"0\"")
        }).map(|r| r.what.clone())
    };
    let boxes = |replies: &[Reply]| replies.iter().filter(|r| r.opcode == net::script::SCRIPT_MESSAGE).count();

    let mut audited = 0;
    let mut board_skipped = 0;
    let mut accepted_silently = 0;
    let mut accepted_with_yes = 0;
    let mut turned_in_with_yes = 0;
    let mut turned_in_after_client_line = 0;
    let mut turned_in_with_quiz = 0;
    let mut turned_in_chained = 0;
    let mut turned_in_silently = 0;
    let mut ids: Vec<u32> = quests.keys().copied().collect();
    ids.sort_unstable();
    // One session for the lot: an account costs an argon2id hash, and 316 of them took three
    // minutes. Each quest is its own row, so one character can accept them all in turn.
    let (mut s, store, id2) = claimed_session();
    s.config = config.clone();
    for qid in ids {
        let q = &quests[&qid];
        if q.say.is_empty() {
            continue;
        }
        // The audit is about what is SAID, not about room: 316 quests' rewards on one
        // character fill a tab, and since 2026-09-17 a full tab is a refusal box instead of
        // the quest's line. Each quest starts with an empty bag.
        empty_bag(&store, id2);
        // **The Community Board's 71 quests are gated on the day's posting** (2026-09-28), so
        // an accept outside it is a refusal box by design - `session/citizenship.rs` tests
        // exactly that. The 14 citizenship story quests are still audited: the character is
        // made a top-grade citizen of that quest's town first, so the gate lets them through.
        if crate::citizenship::board_group_of(qid).is_some() {
            board_skipped += 1;
            continue;
        }
        if let Some((town, _)) = q.citizenship_check {
            let standing = |t: u8, state: u8| store::citizenship::TownStanding { town: t, state, grade: 10, contribution: 10_000, certified_grade: 10 };
            let other = if town == 1 { 2 } else { 1 };
            store.set_citizenship(id2, &[standing(town, store::citizenship::STATE_ACTIVE), standing(other, store::citizenship::STATE_FROZEN)]).unwrap();
        }
        audited += 1;
        let npc = q.start_npc.unwrap_or(1);

        // Accept.
        s.conversation = None;
        let out = s.on_quest_request(&request(1, qid, npc));
        assert!(own_opening(&out, qid).is_none(), "quest {qid}: Accept answered with its own opening: {:?}", own_opening(&out, qid));
        assert!(boxes(&out) <= 1, "quest {qid}: Accept opened {} boxes at once", boxes(&out));
        if q.say.contains_key("0.yes") {
            assert!(out.iter().any(|r| r.what.contains("on path \"0.yes\"")), "quest {qid}: has a yes branch and did not say it: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
            accepted_with_yes += 1;
        } else {
            assert_eq!(boxes(&out), 0, "quest {qid}: no yes branch, yet a box: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
            accepted_silently += 1;
        }

        // Turn-in. The accept above started it (or the store has it started).
        s.conversation = None;
        let _ = s.store.start_quest(id2, qid);
        let end_npc = q.end_npc.unwrap_or(npc);
        let out = s.on_quest_request(&request(2, qid, end_npc));
        assert!(own_opening(&out, qid).is_none(), "quest {qid}: turn-in answered with its own opening: {:?}", own_opening(&out, qid));
        assert!(boxes(&out) <= 1, "quest {qid}: turn-in opened {} boxes at once", boxes(&out));
        // **And never with its own `Say.1`**: the client drew it before sending the turn-in.
        assert!(
            !out.iter().any(|r| r.what.contains(&format!("for quest {qid},")) && r.what.contains("on path \"1\"")),
            "quest {qid}: turn-in repeated Say.1, which the client already showed: {:?}",
            out.iter().map(|r| &r.what).collect::<Vec<_>>()
        );
        let is_quiz = q.say.contains_key("1.ask");
        if is_quiz {
            // The client conducts the quiz and sends the turn-in on a right answer; the server
            // records the completion and says nothing, so it never re-asks or repeats the line.
            assert!(out.iter().any(|r| r.opcode == net::quest::MESSAGE), "quest {qid}: a quiz turn-in must record the completion: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
            assert_eq!(boxes(&out), 0, "quest {qid}: a quiz turn-in must not open a box - the client drew the quiz: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
            turned_in_with_quiz += 1;
        } else if q.say.contains_key("1") && q.say.contains_key("1.yes") {
            assert!(out.iter().any(|r| r.what.contains(&format!("for quest {qid}, line 1 of")) && r.what.contains("on path \"1.yes\"")), "quest {qid}: has 1.yes and did not say it: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
            turned_in_with_yes += 1;
        } else if q.say.contains_key("1") {
            assert_eq!(boxes(&out), 0, "quest {qid}: the client said Say.1 and there is no 1.yes, yet a box: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
            turned_in_after_client_line += 1;
        } else if let Some(next) = q.next_quest.filter(|n| quests.get(n).is_some_and(|nq| nq.say.contains_key("0"))) {
            // The chain: the NEXT quest's opening, spoken because the finished one has nothing.
            // The next quest is accepted in the same breath, so the client will not offer it a
            // second time - on screen 2026-08-20 for 1000 -> 1001.
            assert!(out.iter().any(|r| r.what.contains(&format!("for quest {next},")) && r.what.contains("on path \"0\"")), "quest {qid}: should chain to {next}: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
            assert!(s.store.quest_row(id2, next).unwrap().is_some(), "quest {qid}: chained to {next} without starting it");
            turned_in_chained += 1;
        } else {
            assert_eq!(boxes(&out), 0, "quest {qid}: nothing to say, yet a box: {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
            turned_in_silently += 1;
        }
    }
    eprintln!(
        "audited {audited} quests: accept -> {accepted_with_yes} spoke the yes branch, {accepted_silently} sent the record alone; turn-in -> {turned_in_with_yes} spoke 1.yes, {turned_in_after_client_line} sent the record after the client's own Say.1, {turned_in_with_quiz} completed a quiz silently, {turned_in_chained} chained, {turned_in_silently} sent the record alone"
    );
    assert_eq!(board_skipped, 71, "the four Community Board groups");
    assert!(audited > 230, "the client ships 322 quests, 71 of them board quests; {audited} audited");
    assert_eq!(turned_in_with_quiz, 11, "the quiz nodes on path 1: Rain's seven, Stan, I'm Bored 1, Flying Medicine, Animal Fossils (the other seven ask nodes are openings, which the client shows itself)");
}


// ==========================================================================================
// Crafting - `0x02F6` in, `0x0398` out. `session/craft.rs`, `research/crafting-2026-09-21.md`
// ==========================================================================================

/// The Smithing recipe quest 80011 asks for: five `4010000` into one `4010100`, 100 mesos,
/// 3 mastery. Key `1000` is `(level 1 + profession 0 * 10) * 1000 + index 0` - the client's
/// own formula, which is the one thing a craft request carries.
fn smithing_recipe() -> crate::crafting::Recipe {
    crate::crafting::Recipe {
        key: 1_000,
        profession: 0,
        craft_level: 1,
        process_time_ms: 3_000,
        meso: 100,
        additive: (0, 0),
        ingredients: vec![(4_010_000, 5)],
        result: (4_010_100, 1),
        result_exp: 3,
    }
}

/// A claimed session that knows one recipe, with the materials and the mesos for it.
fn crafting_session() -> (Session, Arc<Store>, u32) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    store.set_gm("maplecw", true).unwrap();
    let chr = net::opcode::Character { name: "Wisp".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    let mut recipes = crate::crafting::Recipes::new();
    recipes.insert(1_000, smithing_recipe());
    let config = Arc::new(Config { recipes, ..Config::default() });
    let mut s = Session::new(store.clone(), config);
    store.create_migration(account_id, id, 0, 0).unwrap();
    s.claim_for_character(id);
    store
        .add_item(id, store::InventoryType::Etc, &store::Item::bundle(4_010_000, 5), 200)
        .unwrap();
    store.set_mesos(id, 1_000).unwrap();
    (s, store, id)
}

/// The `0x02F6` body for "begin this craft".
fn craft_begin(profession: u32, key: u32, additive: bool, count: u32) -> Vec<u8> {
    let mut b = net::craft::CLIENT_CRAFT_REQUEST.to_le_bytes().to_vec();
    b.extend_from_slice(&0u32.to_le_bytes());
    b.extend_from_slice(&profession.to_le_bytes());
    b.extend_from_slice(&key.to_le_bytes());
    b.push(u8::from(additive));
    b.extend_from_slice(&count.to_le_bytes());
    b
}

/// The `0x02F6` body for a mode with no fields: 3 completes, 1 cancels.
fn craft_mode(mode: u32) -> Vec<u8> {
    let mut b = net::craft::CLIENT_CRAFT_REQUEST.to_le_bytes().to_vec();
    b.extend_from_slice(&mode.to_le_bytes());
    b
}

/// The `(mode, result)` of a `0x0398`. A mode-5 or mode-6 answer has no result.
fn craft_answer(out: &[Reply]) -> (u32, Option<u32>) {
    let r = out
        .iter()
        .find(|r| r.opcode == net::craft::CRAFT_RESULT)
        .unwrap_or_else(|| panic!("no 0x0398 in {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>()));
    let mode = u32::from_le_bytes([r.body[0], r.body[1], r.body[2], r.body[3]]);
    let result = (r.body.len() >= 8)
        .then(|| u32::from_le_bytes([r.body[4], r.body[5], r.body[6], r.body[7]]));
    (mode, result)
}

/// **A craft is two packets, and NOTHING is taken on the first.** The begin only decides
/// whether the client may play its animation; the complete is what moves items.
#[test]
fn a_craft_takes_its_materials_on_the_complete_and_never_on_the_begin() {
    let (mut s, store, id) = crafting_session();
    store.learn_profession(id, 0).unwrap();

    let out = s.handle(&craft_begin(0, 1_000, false, 1));
    assert_eq!(craft_answer(&out), (4, Some(0)), "mode 4 result 0 - the bar may start");
    assert!(
        !out.iter().any(|r| r.opcode == net::inventory::INVENTORY_OPERATION),
        "the begin moves nothing: {:?}",
        out.iter().map(|r| &r.what).collect::<Vec<_>>()
    );
    assert_eq!(store.mesos(id).unwrap(), 1_000, "and charges nothing");
    assert_eq!(
        store.bag(id).unwrap().items_in(store::InventoryType::Etc).count(),
        1,
        "the five ore are untouched"
    );

    let out = s.handle(&craft_mode(3));
    assert_eq!(craft_answer(&out), (7, Some(0)), "mode 7 result 0 - it was made");
    assert_eq!(store.mesos(id).unwrap(), 900, "100 mesos paid");
    let etc: Vec<(u32, u16)> = store
        .bag(id)
        .unwrap()
        .items_in(store::InventoryType::Etc)
        .map(|i| (i.item.item_id, i.item.kind.quantity()))
        .collect();
    assert_eq!(etc, vec![(4_010_100, 1)], "the ore is gone and the plate is here: {etc:?}");
    // The mastery, and the ONE packet that carries it: the level and the exp share a u32.
    assert_eq!(store.profession(id, 0).unwrap(), (1, 3));
    let skill = out
        .iter()
        .find(|r| r.opcode == net::skills::CHANGE_SKILL_RECORD_RESULT)
        .expect("a 0x0081 for the mastery");
    // clearLatch, showEffect, pad, count=1, then id / packed level / masterLevel / expiry
    assert_eq!(&skill.body[0..5], &[1, 0, 0, 1, 0]);
    assert_eq!(u32::from_le_bytes([skill.body[5], skill.body[6], skill.body[7], skill.body[8]]), 92_000_000);
    assert_eq!(
        u32::from_le_bytes([skill.body[9], skill.body[10], skill.body[11], skill.body[12]]),
        net::craft::packed_mastery(1, 3),
        "level in the top byte, mastery below - NOT a bare 1"
    );
    assert!(
        out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE && notice_text(r).contains("Smithing's mastery increased. (+3)")),
        "{:?}",
        out.iter().map(|r| &r.what).collect::<Vec<_>>()
    );
    // The record the next field entry builds carries the same packed number, which is how
    // the tab is still open after a map change.
    let skills = store.skills(id).unwrap();
    assert_eq!(skills.len(), 1);
    assert_eq!((skills[0].id, skills[0].level), (92_000_000, net::craft::packed_mastery(1, 3)));
}

/// Every refusal the client has a sentence for, and the one code that must NOT reset its
/// window. A refused begin leaves the materials where they are.
#[test]
fn the_refusals_use_the_clients_own_result_codes() {
    let (mut s, store, id) = crafting_session();

    // Not learnt: "Your skill level is not high enough to craft this item."
    assert_eq!(craft_answer(&s.handle(&craft_begin(0, 1_000, false, 1))), (4, Some(6)));
    store.learn_profession(id, 0).unwrap();

    // A key nobody has: "An error occurred. Please try again."
    assert_eq!(craft_answer(&s.handle(&craft_begin(0, 4_242, false, 1))), (4, Some(2)));
    // The right key under the wrong tab is the same answer.
    assert_eq!(craft_answer(&s.handle(&craft_begin(3, 1_000, false, 1))), (4, Some(2)));

    // No mesos: "You do not have enough mesos to craft this item."
    store.set_mesos(id, 10).unwrap();
    assert_eq!(craft_answer(&s.handle(&craft_begin(0, 1_000, false, 1))), (4, Some(7)));
    store.set_mesos(id, 1_000).unwrap();

    // Short of ore: "You do not have enough materials to craft this item."
    let slot = store.bag_items(id, store::InventoryType::Etc).unwrap()[0].slot;
    store.remove_item(id, store::InventoryType::Etc, slot, Some(3)).unwrap();
    assert_eq!(craft_answer(&s.handle(&craft_begin(0, 1_000, false, 1))), (4, Some(3)));
    store
        .add_item(id, store::InventoryType::Etc, &store::Item::bundle(4_010_000, 3), 200)
        .unwrap();

    // A complete with no begin behind it makes nothing: the pair is the whole gate, and
    // nothing authenticates, so a hand-built mode 3 is free to send.
    let out = s.handle(&craft_mode(3));
    assert_eq!(craft_answer(&out), (7, Some(2)));
    assert!(store.bag(id).unwrap().items_in(store::InventoryType::Etc).all(|i| i.item.item_id == 4_010_000));

    // A second begin while one is running is code 5 - the ONE code the client does not
    // reset its window on.
    assert_eq!(craft_answer(&s.handle(&craft_begin(0, 1_000, false, 1))), (4, Some(0)));
    assert_eq!(craft_answer(&s.handle(&craft_begin(0, 1_000, false, 1))), (4, Some(5)));
    // Cancel puts it back to idle, and then the complete has nothing to finish.
    assert_eq!(craft_answer(&s.handle(&craft_mode(1))), (5, None));
    assert_eq!(craft_answer(&s.handle(&craft_mode(3))), (7, Some(2)));
    assert_eq!(store.mesos(id).unwrap(), 1_000, "not one meso moved across all of that");

    // A mode nobody has decoded is acknowledged rather than dropped: an unanswered 0x02F6
    // leaves the client's request latch set and it never crafts again.
    assert_eq!(craft_answer(&s.handle(&craft_mode(9))), (6, None));
}

/// **The character level caps the profession**, and the overflow is discarded rather than
/// banked - the client's own sentence is *"To level up %s further, your character must be
/// level %d or higher"* at `(level + 1) * 5`.
#[test]
fn mastery_parks_at_the_character_level_cap() {
    let (mut s, store, id) = crafting_session();
    store.learn_profession(id, 0).unwrap();
    // A level-1 character: the cap is profession level 1, so the bar fills and stops.
    store.set_profession(id, 0, 1, 48).unwrap();
    store
        .add_item(id, store::InventoryType::Etc, &store::Item::bundle(4_010_000, 5), 200)
        .unwrap();
    s.handle(&craft_begin(0, 1_000, false, 1));
    let out = s.handle(&craft_mode(3));
    assert_eq!(craft_answer(&out), (7, Some(0)), "the craft still happens");
    assert_eq!(
        store.profession(id, 0).unwrap(),
        (1, crate::crafting::mastery_exp_needed(1)),
        "parked at 100% of level 1"
    );
    assert!(
        out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE && notice_text(r).contains("capped at level 1 until your character reaches level 10")),
        "the cap is said out loud: {:?}",
        out.iter().map(|r| &r.what).collect::<Vec<_>>()
    );
}

/// **The quest turn-in is what opens the tab.** The owner, 2026-09-21: *"After these quest
/// completions, they should unlock the appropriate crafting menu within the client."*
#[test]
fn the_starter_quest_learns_the_profession_and_pays_its_mastery() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Wisp".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    // Quest 80008, as `gm-handbook/questlines.txt` has it: Silas Irons teaches Smithing and
    // pays one mastery.
    let mut quests = std::collections::HashMap::new();
    quests.insert(
        80_008u32,
        crate::config::Quest {
            name: "Silas Irons in Need of an Apprentice".to_string(),
            complete_exp: 171,
            complete_skills: vec![(92_000_000, 1)],
            ..Default::default()
        },
    );
    let config = Arc::new(Config { quests, ..Config::default() });
    let mut s = Session::new(store.clone(), config);
    store.create_migration(account_id, id, 0, 0).unwrap();
    s.claim_for_character(id);
    store.start_quest(id, 80_008).unwrap();

    let out = s.record_quest_complete(80_008, 0);
    assert_eq!(store.profession(id, 0).unwrap(), (1, 1), "learnt at level 1, one mastery paid");
    assert!(
        out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE && notice_text(r).contains("You have learnt Smithing")),
        "{:?}",
        out.iter().map(|r| &r.what).collect::<Vec<_>>()
    );
    // The skill the client unlocks the tab on, packed.
    let learnt: Vec<(u32, u32)> = store.skills(id).unwrap().iter().map(|k| (k.id, k.level)).collect();
    assert_eq!(learnt, vec![(92_000_000, net::craft::packed_mastery(1, 1))]);

    // **A second turn-in does not reset a levelled profession.** The store's guard is asked
    // and its answer is used - the Heena rule.
    store.set_profession(id, 0, 6, 500).unwrap();
    store.start_quest(id, 80_008).unwrap();
    s.record_quest_complete(80_008, 0);
    assert_eq!(store.profession(id, 0).unwrap().0, 6, "still level 6");
}

/// `!craft` opens a tab without the quest, lists what is open, and closes one again.
#[test]
fn the_craft_command_opens_and_closes_a_tab() {
    let (mut s, store, id) = crafting_session();
    let out = s.handle(&gm_chat("!craft tailoring 4 100"));
    assert!(notice_text(out.last().unwrap()).contains("Set to level 4 (mastery 100): Tailoring"), "{}", notice_text(out.last().unwrap()));
    assert_eq!(store.profession(id, 2).unwrap(), (4, 100));
    let skill = out.iter().find(|r| r.opcode == net::skills::CHANGE_SKILL_RECORD_RESULT).expect("a 0x0081");
    assert_eq!(
        u32::from_le_bytes([skill.body[9], skill.body[10], skill.body[11], skill.body[12]]),
        net::craft::packed_mastery(4, 100)
    );

    let out = s.handle(&gm_chat("!craft"));
    assert!(notice_text(&out[0]).contains("Tailoring level 4 (100/521)"), "{}", notice_text(&out[0]));

    let out = s.handle(&gm_chat("!craft all 10"));
    assert!(notice_text(out.last().unwrap()).contains("Arcforge"), "{}", notice_text(out.last().unwrap()));
    assert_eq!(store.crafting(id).unwrap().len(), 6);

    let out = s.handle(&gm_chat("!craft 2 0"));
    assert!(notice_text(out.last().unwrap()).contains("Closed: Tailoring"), "{}", notice_text(out.last().unwrap()));
    assert_eq!(store.profession(id, 2).unwrap(), (0, 0));
    assert!(out.iter().any(|r| r.opcode == net::skills::CHANGE_SKILL_RECORD_RESULT), "the client is told it is gone");

    let out = s.handle(&gm_chat("!craft baking 3"));
    assert!(notice_text(&out[0]).contains("is not a profession"), "{}", notice_text(&out[0]));
}


/// **A crafting quest finished BEFORE this server read `Act.1.skill` still opens its tab.**
///
/// The owner, 2026-09-21, on a level-12 character whose Woodcrafting tab still read *"Vicious in
/// Henesys is looking for an apprentice"*: *"the UI shows this even after the user has
/// completed the pre-requisite quest."* The claim reconciles it, before the login `SetField`,
/// so the record itself carries the skill.
#[test]
fn a_crafting_quest_completed_before_today_is_backfilled_at_the_claim() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Wisp".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    let mut quests = std::collections::HashMap::new();
    quests.insert(
        80_017u32,
        crate::config::Quest {
            name: "Vicious in Need of an Apprentice".to_string(),
            complete_skills: vec![(92_030_000, 1)],
            ..Default::default()
        },
    );
    // An in-progress one must NOT count - only a finished quest teaches anything.
    quests.insert(
        80_008u32,
        crate::config::Quest {
            name: "Silas Irons in Need of an Apprentice".to_string(),
            complete_skills: vec![(92_000_000, 1)],
            ..Default::default()
        },
    );
    let config = Arc::new(Config { quests, ..Config::default() });
    // The world as it was: the quest is finished, and nothing ever granted the profession.
    store.start_quest(id, 80_017).unwrap();
    store.complete_quest(id, 80_017).unwrap();
    store.start_quest(id, 80_008).unwrap();
    assert!(store.crafting(id).unwrap().is_empty());

    let mut s = Session::new(store.clone(), config);
    store.create_migration(account_id, id, 0, 0).unwrap();
    s.claim_for_character(id);

    assert_eq!(store.profession(id, 3).unwrap(), (1, 0), "Woodcrafting learnt, bar empty");
    assert_eq!(store.profession(id, 0).unwrap(), (0, 0), "the unfinished quest taught nothing");
    // The record the client builds its tabs from carries it, so the tab is open on the first
    // screen rather than after a packet.
    let skills = store.skills(id).unwrap();
    assert_eq!(skills.iter().map(|k| (k.id, k.level)).collect::<Vec<_>>(), vec![(92_030_000, net::craft::packed_mastery(1, 0))]);

    // **Idempotent**: a second login neither re-learns nor re-pays. A login that quietly
    // added mastery would be a farm.
    store.set_profession(id, 3, 5, 300).unwrap();
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s2 = Session::new(store.clone(), s.config.clone());
    s2.claim_for_character(id);
    assert_eq!(store.profession(id, 3).unwrap(), (5, 300));
}


/// **The pet's item goes out BEFORE the field-entry summon as well as after it, so no
/// "Closeness has increased (+N)" line is invented on a map change.**
///
/// The owner, 2026-09-21: *"Whenever I change maps, if the pet has some sort of closeness, a
/// message of +1 closeness still erroneously show up bottom right on the screen, despite not
/// actually adding any closeness."* The number is the closeness itself - Lucy's was 1, read
/// out of the `0x0070` body in `world-ch0.log` - and the client is reporting a rise from 0:
/// `FUN_141ec4f60` prints string `0x1AC` with the difference between the pet's cached
/// closeness and the one it re-reads from the Cash item. A field entry clears the client's
/// bag, so the pet was being built with no item to read.
///
/// **Both writes matter and they fix different bugs**: the one before the summon is what the
/// pet is built from (this bug), the one after is the re-read that makes the vacuum work
/// (2026-09-18, on the owner's screen).
#[test]
fn the_field_entry_sends_the_pets_item_before_the_summon_as_well_as_after() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Wisp".to_string(), ..Default::default() };
    let id = store.create_character(account, 0, &chr).unwrap().id;
    store.add_item(id, store::InventoryType::Cash, &store::Item::bundle(5_000_006, 1), 1).unwrap();
    store.create_migration(account, id, 0, 0).unwrap();
    let mut s = Session::new(store.clone(), Arc::new(Config::default()));
    s.claim_for_character(id);
    s.on_pet_activate(&hex("509a18140100"));
    let pet_id = pet_of(&store, id, 5_000_006);
    // A pet with closeness to report - the case the owner saw. Level 2, closeness 1, like Lucy.
    store.set_pet_vitals(pet_id, 2, 1, 100).unwrap();

    let entered = s.on_field_entered();
    let pet_batch: Vec<(u16, &str)> = entered
        .iter()
        .filter(|r| r.opcode == net::pet::PET_ACTIVATED || r.what.contains("re-sent as pet"))
        .map(|r| (r.opcode, r.what.as_str()))
        .collect();
    assert_eq!(pet_batch.len(), 3, "item, summon, item: {pet_batch:?}");
    assert_eq!(pet_batch[0].0, net::inventory::INVENTORY_OPERATION, "the item is FIRST, so CPet is built knowing its closeness");
    assert_eq!(pet_batch[1].0, net::pet::PET_ACTIVATED);
    assert_eq!(pet_batch[2].0, net::inventory::INVENTORY_OPERATION, "and again after, which is the vacuum re-read");

    // Both carry the same closeness. A difference between them is exactly what the client
    // would print, so this is the assertion that matters rather than the order alone.
    let closeness_of = |r: &Reply| -> u16 {
        // The blob starts at the item type byte; `pet_item_with_state` puts closeness 40
        // bytes in (1 + 4 + 1 + 8 + 8 + 4 + 1 + 13 + 1), and the mode-5 header is 11 bytes
        // with a cash serial present.
        let blob = r.body.windows(5).position(|w| w == [3, 0x46, 0x4b, 0x4c, 0x00]).expect("the pet blob");
        let at = blob + 1 + 4 + 1 + 8 + 8 + 4 + 1 + 13 + 1;
        u16::from_le_bytes([r.body[at], r.body[at + 1]])
    };
    let first = entered.iter().find(|r| r.what.contains("re-sent as pet")).unwrap();
    let last = entered.iter().filter(|r| r.what.contains("re-sent as pet")).last().unwrap();
    assert_eq!(closeness_of(first), 1, "the closeness the store holds");
    assert_eq!(closeness_of(last), closeness_of(first), "the two writes must not disagree");
}


// ==========================================================================================
// The friend list - `0x0193` in, `0x00A7` out. `session/friends.rs`,
// `research/friends-2026-09-21.md`
// ==========================================================================================

/// The `0x0193` body the client's add button builds: `u8 1, str name, str group, str, u8`.
/// Byte for byte the shape captured at `world-ch0.log` 01:35:06.649.
fn friend_add_body(name: &str, group: &str) -> Vec<u8> {
    let mut b = net::friends::CLIENT_FRIEND_REQUEST.to_le_bytes().to_vec();
    b.push(1);
    b.extend_from_slice(&(name.len() as u16).to_le_bytes());
    b.extend_from_slice(name.as_bytes());
    b.extend_from_slice(&(group.len() as u16).to_le_bytes());
    b.extend_from_slice(group.as_bytes());
    b.extend_from_slice(&0u16.to_le_bytes());
    b.push(1);
    b
}

/// The `{id, name}` rows out of a `0x00A7` sub-op 0x19.
fn friend_rows(out: &[Reply]) -> Vec<(u32, String)> {
    let r = out
        .iter()
        .find(|r| r.opcode == net::friends::FRIEND_RESULT && r.body[0] == 0x19)
        .unwrap_or_else(|| panic!("no friend list in {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>()));
    let count = u32::from_le_bytes([r.body[1], r.body[2], r.body[3], r.body[4]]) as usize;
    let mut at = 5;
    let mut rows = Vec::new();
    for _ in 0..count {
        let id = u32::from_le_bytes([r.body[at], r.body[at + 1], r.body[at + 2], r.body[at + 3]]);
        let len = u16::from_le_bytes([r.body[at + 4], r.body[at + 5]]) as usize;
        rows.push((id, String::from_utf8(r.body[at + 6..at + 6 + len].to_vec()).unwrap()));
        at += 6 + len;
    }
    rows
}

/// **The rows the window actually draws**: `(id, name, flag)` out of a `0x00A7` sub-op `0x15`,
/// read at the offsets `FUN_142dec8f0` reads them at.
fn friend_records(out: &[Reply]) -> Vec<(u32, String, u8)> {
    let r = out
        .iter()
        .find(|r| r.opcode == net::friends::FRIEND_RESULT && r.body[0] == 0x15)
        .unwrap_or_else(|| panic!("no friend records in {:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>()));
    let count = u32::from_le_bytes([r.body[1], r.body[2], r.body[3], r.body[4]]) as usize;
    let entry = net::friends::FRIEND_ENTRY_LEN;
    assert_eq!(r.body.len(), 5 + count * entry, "the client reads count * 0x149 in one call");
    (0..count)
        .map(|i| {
            let rec = &r.body[5 + i * entry..5 + (i + 1) * entry];
            let id = u32::from_le_bytes([rec[0], rec[1], rec[2], rec[3]]);
            let name = rec[4..0x11].split(|b| *b == 0).next().unwrap();
            (id, String::from_utf8(name.to_vec()).unwrap(), rec[0x11])
        })
        .collect()
}

/// The sentence-only replies are one byte, and this is which one.
fn friend_notices(out: &[Reply]) -> Vec<u8> {
    out.iter()
        .filter(|r| r.opcode == net::friends::FRIEND_RESULT && r.body.len() == 1)
        .map(|r| r.body[0])
        .collect()
}

/// Two sessions of two characters on one channel, both claimed and in the field.
fn two_friends() -> (Session, u32, Session, u32, Arc<Store>) {
    let (store, config, fields, account) = shared_channel(0, 30);
    let (mut wisp, wisp_id) = join_channel(&store, &config, &fields, account, "Wisp");
    let other = store.create_account("player", "correct horse battery").unwrap();
    let (mut tester, tester_id) = join_channel(&store, &config, &fields, other, "Tester2");
    wisp.on_field_entered();
    tester.on_field_entered();
    wisp.collect_mail();
    tester.collect_mail();
    (wisp, wisp_id, tester, tester_id, store)
}

/// **The whole thing the owner reported.** Tester2 adds the owner: Tester2 is told the request went,
/// The owner is told on their own screen, and the rows are written - two directed ones, so each
/// side's list says something different about the same friendship.
#[test]
fn a_friend_request_is_answered_recorded_and_said_out_loud_on_both_screens() {
    let (mut wisp, wisp_id, mut tester, tester_id, store) = two_friends();

    let out = tester.handle(&friend_add_body("Wisp", "Default Group"));
    // The client's own line, sub-op 0x1B, carrying the name it will print.
    let sent = out
        .iter()
        .find(|r| r.opcode == net::friends::FRIEND_RESULT && r.body[0] == 0x1B)
        .expect("the \"request sent\" reply");
    assert_eq!(&sent.body[1..3], &4u16.to_le_bytes());
    assert_eq!(&sent.body[3..7], b"Wisp");
    // **Tester2's window stays empty while they wait**, and that is deliberate: the flag byte
    // a record carries has no "I asked them" value, and the one value that would show the row
    // - `FLAG_REQUEST` - is what makes the client raise a balloon. Only the answer puts the owner
    // in this list.
    assert_eq!(friend_rows(&out), vec![]);
    assert_eq!(friend_records(&out), vec![]);
    assert_eq!(
        store.friends(tester_id).unwrap()[0].state,
        store::friends::FriendState::Requested
    );
    assert_eq!(
        store.friends(wisp_id).unwrap()[0].state,
        store::friends::FriendState::Pending,
        "The owner's row is the one that owes an answer"
    );

    // **The owner's screen: the client's own popup**, not a chat line. The owner, 2026-09-22: *"there
    // should not be any chat commands ... one similar pop up just like the party
    // invitation"*. Sub-op 0x1A, balloon kind 0x0E, "Friend request from Tester2".
    let seen = wisp.collect_mail();
    assert_eq!(friend_records(&seen), vec![], "a request is an invitation, not a row");
    let popup = seen
        .iter()
        .find(|r| r.opcode == net::friends::FRIEND_RESULT && r.body[0] == 0x1A)
        .expect("the friend-request balloon");
    assert_eq!(popup.body[1], 0, "flag 0 raises balloon kind 0x0E, whose Yes is sub-op 2");
    // The u32 the balloon keeps and echoes back, then the one it looks up locally.
    assert_eq!(&popup.body[2..6], &tester_id.to_le_bytes());
    assert_eq!(&popup.body[6..10], &tester_id.to_le_bytes());
    assert_eq!(&popup.body[10..12], &7u16.to_le_bytes());
    assert_eq!(&popup.body[12..19], b"Tester2");
    // **And then the 329 bytes whose absence killed the client on 2026-09-22.** The arm reads
    // seven fields - 28 bytes, which is exactly what this packet used to be - and then a whole
    // record, so a body that ends here is refused with `0x009E` reason `0x26`.
    assert_eq!(
        popup.body.len(),
        31 + net::friends::FRIEND_ENTRY_LEN,
        "the popup must carry a friend record: {}",
        popup.what
    );
    let record = &popup.body[31..];
    assert_eq!(&record[0..4], &tester_id.to_le_bytes());
    assert_eq!(&record[4..11], b"Tester2");
    assert_eq!(record[0x11], net::friends::FLAG_REQUEST, "this byte is what pops the balloon");
    assert!(
        !seen.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE && notice_text(r).contains("!friend")),
        "no chat command is offered"
    );
    // The same field entry twice does not raise it again - the request is still pending, and
    // a balloon per map change is not what the client does.
    assert!(!wisp.on_field_entered().iter().any(|r| r.opcode == net::friends::FRIEND_RESULT && r.body[0] == 0x1A));

    // **The owner presses Yes**: 0x0193 sub-op 2, echoing what the balloon held.
    let mut yes = net::friends::CLIENT_FRIEND_REQUEST.to_le_bytes().to_vec();
    yes.push(net::friends::REQUEST_ACCEPT);
    yes.extend_from_slice(&tester_id.to_le_bytes());
    let out = wisp.handle(&yes);
    // **The system colour, not a plain chat line.** The owner: *"can we send it as a red system
    // message?"* `0x00AC` type 5 ends in the same printer and the same kind (0xb) as the
    // client's own "%s has declined the friend request."
    let said = out
        .iter()
        .find(|r| r.opcode == net::broadcast::BROADCAST_MSG)
        .unwrap_or_else(|| panic!("{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>()));
    assert_eq!(said.body[0], net::broadcast::SYSTEM_LINE);
    assert_eq!(&said.body[3..], b"Tester2 is now your friend.");
    for who in [wisp_id, tester_id] {
        assert_eq!(store.friends(who).unwrap()[0].state, store::friends::FriendState::Accepted);
    }
    // Tester2's window gains the owner without them doing anything - and the record says they are
    // online, on this channel, because they are.
    let told = tester.collect_mail();
    assert_eq!(friend_rows(&told), vec![(wisp_id, "Wisp".to_string())]);
    assert_eq!(
        friend_records(&told),
        vec![(wisp_id, "Wisp".to_string(), net::friends::FLAG_ONLINE)]
    );

    // And the list survives a map change, because the client rebuilds its friend manager
    // with its pools.
    let again = wisp.on_field_entered();
    assert_eq!(friend_rows(&again), vec![(tester_id, "Tester2".to_string())]);
    assert_eq!(
        friend_records(&again),
        vec![(tester_id, "Tester2".to_string(), net::friends::FLAG_ONLINE)]
    );
    // **The name cache goes out BEFORE the records**, so a cache that names a row cannot be
    // older than the row. (It is a preference: `0x19` clears its own map, not the record
    // array - the first version of this test said otherwise.)
    let order: Vec<u8> = again
        .iter()
        .filter(|r| r.opcode == net::friends::FRIEND_RESULT)
        .map(|r| r.body[0])
        .collect();
    assert_eq!(order, vec![0x19, 0x15]);
}

/// **A friend logging in tells the other side, and logging out tells them again.**
///
/// The owner, 2026-09-22: *"when Tester2 logs in after the owner, the owner was not informed of the fact that
/// Tester2 has logged in"*, and *"once the owner logs off, the buddy list also remains showing the owner
/// is still online."* Both were the same gap - the list was only re-sent on the friend's own
/// field entry.
#[test]
fn a_friend_logging_in_and_out_reaches_the_other_sides_window() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let (mut wisp, wisp_id) = join_channel(&store, &config, &fields, account, "Wisp");
    let other = store.create_account("player", "correct horse battery").unwrap();
    let (mut tester, tester_id) = join_channel(&store, &config, &fields, other, "Tester2");
    wisp.on_field_entered();
    tester.on_field_entered();
    // Friends already, so the presence notices have somewhere to go.
    store.request_friend(tester_id, wisp_id, "Default Group").unwrap();
    store.answer_friend_request(wisp_id, tester_id, true).unwrap();
    wisp.collect_mail();
    tester.collect_mail();

    // **Tester2 logs out.** The owner is told, quietly - the row greys, no line is said.
    drop(tester);
    let seen = wisp.collect_mail();
    let off = seen
        .iter()
        .find(|r| r.opcode == net::friends::FRIEND_RESULT && r.body[0] == 0x2D)
        .unwrap_or_else(|| panic!("{:?}", seen.iter().map(|r| &r.what).collect::<Vec<_>>()));
    assert_eq!(&off.body[1..5], &tester_id.to_le_bytes());
    assert_eq!(off.body[9] as u32, net::friends::STATUS_OFFLINE);
    assert_eq!(off.body[15], 0, "a logout is not announced out loud");
    // And the list came WITH it, with the row marked offline - that is what greys it.
    assert_eq!(
        friend_records(&seen),
        vec![(tester_id, "Tester2".to_string(), net::friends::FLAG_OFFLINE)]
    );
    // The list goes FIRST: sent after the 0x2D it would overwrite the status word the client
    // compares against, and the next login would announce nothing.
    let order: Vec<u8> = seen
        .iter()
        .filter(|r| r.opcode == net::friends::FRIEND_RESULT)
        .map(|r| r.body[0])
        .collect();
    assert_eq!(order, vec![0x19, 0x15, 0x2D]);

    // **Tester2 logs back in** - the same character, claimed by a new session.
    store.create_migration(other, tester_id, 0, 0).unwrap();
    let mut tester = Session::joining(store.clone(), config.clone(), fields.clone());
    tester.claim_for_character(tester_id);
    tester.on_field_entered();
    let seen = wisp.collect_mail();
    let on = seen
        .iter()
        .find(|r| r.opcode == net::friends::FRIEND_RESULT && r.body[0] == 0x2D)
        .unwrap_or_else(|| panic!("{:?}", seen.iter().map(|r| &r.what).collect::<Vec<_>>()));
    assert_eq!(on.body[9] as u32, net::friends::STATUS_ONLINE);
    assert_eq!(&on.body[10..14], &0u32.to_le_bytes(), "channel 0");
    assert_eq!(on.body[14], 0, "matched on the character id, so no detail block");
    assert_eq!(on.body[15], 1, "and this is the one that says it");
    assert_eq!(
        friend_records(&seen),
        vec![(tester_id, "Tester2".to_string(), net::friends::FLAG_ONLINE)]
    );
}

/// **The buddy window's "Checking location" is answered into the window, not the chat.**
///
/// The owner, 2026-09-23: *"the current location of the player is reflecting in chat, but it should
/// be where it says 'Tester2 - Checking location'. Instead of that, it should say 'Tester2 -
/// Kerning City' or 'Tester2 - Channel 2'."* The window asks with kind `0x44` (`0x40 | /find`)
/// and the client only fills the window when the answer carries the same bit - mode `0x48`,
/// not `0x09`. The bytes are the ones Tester2's client actually sent (`world-ch0.log`
/// 2026-09-22 00:58:14).
#[test]
fn the_buddy_windows_location_check_is_answered_into_the_window() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let (mut wisp, _) = join_channel(&store, &config, &fields, account, "Wisp");
    let other = store.create_account("player", "correct horse battery").unwrap();
    let (mut tester, _) = join_channel(&store, &config, &fields, other, "Tester2");
    wisp.on_field_entered();
    tester.on_field_entered();

    // 0x017B  44 <tick> 0400 "Wisp"
    let mut ask = net::whisper::CLIENT_WHISPER.to_le_bytes().to_vec();
    ask.extend_from_slice(&[0x44, 0x1f, 0x6b, 0x14, 0x12, 0x04, 0x00]);
    ask.extend_from_slice(b"Wisp");
    let out = tester.handle(&ask);
    let answer = out
        .iter()
        .find(|r| r.opcode == net::whisper::WHISPER)
        .unwrap_or_else(|| panic!("{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>()));
    assert_eq!(answer.body[0], net::whisper::mode::FOUND_IN_WINDOW, "{}", answer.what);
    assert_eq!(&answer.body[3..7], b"Wisp");
    assert_eq!(answer.body[7], net::whisper::place::MAP, "same channel: the map, which the client names");
    assert_eq!(&answer.body[8..12], &SHARED_MAP.to_le_bytes());
    assert!(!out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE), "and nothing in the chat log");

    // A /find typed in chat (kind 5) still answers in chat - mode 0x09.
    ask[2] = 0x05;
    let out = tester.handle(&ask);
    let answer = out.iter().find(|r| r.opcode == net::whisper::WHISPER).unwrap();
    assert_eq!(answer.body[0], net::whisper::mode::FOUND);
}

/// **Job and level are in the record where the row builder reads them**, so they show for an
/// offline friend too. The owner, 2026-09-23: *"The buddy list still does not have Job and level of
/// the buddy character."* `FUN_1411be0a0` reads `rec+0x139` (LV) and hands `rec+0x13D` to the
/// job-name lookup `FUN_1402b0250(job, subJob)`.
#[test]
fn a_friend_record_carries_level_and_job_where_the_row_builder_reads_them() {
    let (mut wisp, wisp_id, tester, tester_id, store) = two_friends();
    store.request_friend(tester_id, wisp_id, "Default Group").unwrap();
    store.answer_friend_request(wisp_id, tester_id, true).unwrap();
    let brief = store.character_brief(tester_id).unwrap().unwrap();
    drop(tester); // offline: the columns must still be filled
    wisp.collect_mail();

    let out = wisp.on_field_entered();
    let list = out.iter().find(|r| r.opcode == net::friends::FRIEND_RESULT && r.body[0] == 0x15).unwrap();
    let rec = &list.body[5..5 + net::friends::FRIEND_ENTRY_LEN];
    assert_eq!(rec[0x11], net::friends::FLAG_OFFLINE);
    assert_eq!(&rec[0x139..0x13D], &brief.level.to_le_bytes(), "LV");
    assert_eq!(&rec[0x13D..0x141], &brief.job.to_le_bytes(), "JOB");
    assert_eq!(&rec[0x141..0x145], &[0u8; 4], "subJob");
}

/// **Buddy chat reaches every accepted buddy, and nobody else.**
///
/// The owner, 2026-09-24: *"Buddy chat sent by a player with buddies should go to all online buddies
/// that the player has added."* On the deployed server it was logged "kind 0 ... is not built"
/// and went nowhere (`Server Investigation/world-ch0.log` 04:32:28, Moth's `'hewwo'`). The body
/// here is that capture's shape: `u8 kind 0, u16 count, u32 recipient, str text`.
#[test]
fn buddy_chat_reaches_accepted_buddies_only() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let (mut moth, moth_id) = join_channel(&store, &config, &fields, account, "Moth");
    let a2 = store.create_account("buddy", "correct horse battery").unwrap();
    let (mut cobalt, cobalt_id) = join_channel(&store, &config, &fields, a2, "Cobalt");
    let a3 = store.create_account("pending", "correct horse battery").unwrap();
    let (mut asker, asker_id) = join_channel(&store, &config, &fields, a3, "Asker");
    for s in [&mut moth, &mut cobalt, &mut asker] {
        s.on_field_entered();
        s.collect_mail();
    }
    // Cobalt is an accepted buddy; Asker has only ASKED, which is not a friendship.
    store.request_friend(cobalt_id, moth_id, "Default Group").unwrap();
    store.answer_friend_request(moth_id, cobalt_id, true).unwrap();
    store.request_friend(asker_id, moth_id, "Default Group").unwrap();
    for s in [&mut moth, &mut cobalt, &mut asker] {
        s.collect_mail();
    }

    let mut line = net::groupmessage::CLIENT_GROUP_MESSAGE.to_le_bytes().to_vec();
    line.push(net::groupmessage::kind::BUDDY);
    line.extend_from_slice(&1u16.to_le_bytes());
    line.extend_from_slice(&cobalt_id.to_le_bytes());
    line.extend_from_slice(&5u16.to_le_bytes());
    line.extend_from_slice(b"hewwo");
    let out = moth.handle(&line);
    assert!(!out.iter().any(|r| r.opcode == net::groupmessage::GROUP_MESSAGE), "the sender's client draws its own line");

    let heard = cobalt.collect_mail();
    let got = heard
        .iter()
        .find(|r| r.opcode == net::groupmessage::GROUP_MESSAGE)
        .unwrap_or_else(|| panic!("Cobalt hears it: {:?}", heard.iter().map(|r| &r.what).collect::<Vec<_>>()));
    assert_eq!(got.body[0], net::groupmessage::kind::BUDDY, "drawn as a BUDDY line, not a party one");
    assert!(got.body.windows(5).any(|w| w == b"hewwo"));
    assert!(
        !asker.collect_mail().iter().any(|r| r.opcode == net::groupmessage::GROUP_MESSAGE),
        "a request that was never accepted is not a buddy"
    );
}

/// **The group list is a REPORT, and answering it is an infinite loop.**
///
/// Measured on 2026-09-22, on the owner's screen and in their log: `world-ch0.log` reached **42 MB**
/// in one sitting, **32 566** round trips of `0x0193` sub-op `0x14` -> `0x00A7` `0x19` + `0x15`
/// -> sub-op `0x14`, one per millisecond. Every list reply ends in a window refresh and a
/// refreshed window hands its group names back, so answering that with a list closes the ring.
/// The owner: *"the owner's client started lagging a lot"*, *"opening the buddy list crashes/freezes"*.
#[test]
fn the_clients_group_names_are_a_report_and_are_never_answered() {
    let (mut wisp, _wisp_id, mut tester, _tester_id, _store) = two_friends();
    tester.handle(&friend_add_body("Wisp", "Default Group"));
    wisp.collect_mail();

    // The body the owner's client actually sent, 32 566 times: sub-op 0x14, one group.
    let mut groups = net::friends::CLIENT_FRIEND_REQUEST.to_le_bytes().to_vec();
    groups.push(net::friends::REQUEST_GROUPS);
    groups.extend_from_slice(&1u32.to_le_bytes());
    groups.push(1);
    groups.extend_from_slice(&13u16.to_le_bytes());
    groups.extend_from_slice(b"Default Group");
    let out = wisp.handle(&groups);
    assert!(
        out.is_empty(),
        "a group report must be answered with NOTHING - anything the window refreshes on loops: {:?}",
        out.iter().map(|r| &r.what).collect::<Vec<_>>()
    );
    // And specifically not with a list, which is what caused it.
    assert!(!out.iter().any(|r| r.opcode == net::friends::FRIEND_RESULT));
}

/// **A friend request nobody answers is cancelled, and both sides are told.**
///
/// The owner, 2026-09-22: *"it should have a timeout if not accepted within a certain amount of
/// time."* The clock starts when the balloon is raised, not when the request was made.
#[test]
fn an_unanswered_friend_request_times_out_and_tells_both_sides() {
    let (mut wisp, wisp_id, mut tester, tester_id, store) = two_friends();
    tester.handle(&friend_add_body("Wisp", "Default Group"));
    let seen = wisp.collect_mail();
    assert!(seen.iter().any(|r| r.opcode == net::friends::FRIEND_RESULT && r.body[0] == 0x1A));
    tester.collect_mail();

    // One tick short of the timeout changes nothing: the offer is still on screen.
    let nearly = crate::session::friends::FRIEND_REQUEST_TIMEOUT_MS - 1;
    assert!(!wisp
        .tick(nearly)
        .iter()
        .any(|r| r.opcode == net::friends::FRIEND_RESULT && r.body[0] == 0x2A));
    assert_eq!(store.friends(wisp_id).unwrap()[0].state, store::friends::FriendState::Pending);

    // And one tick past it cancels. 0x2A is "The request to add a Friend has been canceled."
    let out = wisp.tick(crate::session::friends::FRIEND_REQUEST_TIMEOUT_MS);
    assert!(
        out.iter().any(|r| r.opcode == net::friends::FRIEND_RESULT && r.body == vec![0x2A]),
        "{:?}",
        out.iter().map(|r| &r.what).collect::<Vec<_>>()
    );
    assert_eq!(friend_records(&out), vec![], "and the list is re-sent, empty");
    assert!(store.friends(wisp_id).unwrap().is_empty(), "both rows go, not just one");
    assert!(store.friends(tester_id).unwrap().is_empty());
    // The asker is told too, or they wait forever on a request that no longer exists.
    assert!(tester
        .collect_mail()
        .iter()
        .any(|r| r.opcode == net::friends::FRIEND_RESULT && r.body == vec![0x2A]));

    // **It does not fire twice**, and a tick long after changes nothing more.
    let out = wisp.tick(crate::session::friends::FRIEND_REQUEST_TIMEOUT_MS * 10);
    assert!(!out.iter().any(|r| r.opcode == net::friends::FRIEND_RESULT));
}

/// **An answered request never times out**, even though the tick that would have fired it runs.
#[test]
fn answering_a_friend_request_stops_its_timeout() {
    let (mut wisp, wisp_id, mut tester, tester_id, store) = two_friends();
    tester.handle(&friend_add_body("Wisp", "Default Group"));
    wisp.collect_mail();

    let mut yes = net::friends::CLIENT_FRIEND_REQUEST.to_le_bytes().to_vec();
    yes.push(net::friends::REQUEST_ACCEPT);
    yes.extend_from_slice(&tester_id.to_le_bytes());
    wisp.handle(&yes);
    tester.collect_mail();

    // Well past the timeout: the friendship is untouched and nothing is cancelled.
    let out = wisp.tick(crate::session::friends::FRIEND_REQUEST_TIMEOUT_MS * 3);
    assert!(!out.iter().any(|r| r.opcode == net::friends::FRIEND_RESULT && r.body == vec![0x2A]));
    for who in [wisp_id, tester_id] {
        assert_eq!(store.friends(who).unwrap()[0].state, store::friends::FriendState::Accepted);
    }
}

/// **The popup's No (sub-op 6) refuses, and the asker gets the client's own sentence.**
#[test]
fn the_friend_popups_no_refuses_and_tells_the_asker() {
    let (mut wisp, wisp_id, mut tester, tester_id, store) = two_friends();
    tester.handle(&friend_add_body("Wisp", "Default Group"));
    assert!(wisp.collect_mail().iter().any(|r| r.opcode == net::friends::FRIEND_RESULT && r.body[0] == 0x1A));

    let mut no = net::friends::CLIENT_FRIEND_REQUEST.to_le_bytes().to_vec();
    no.push(net::friends::REQUEST_REFUSE);
    no.extend_from_slice(&tester_id.to_le_bytes());
    no.push(0);
    let out = wisp.handle(&no);
    assert!(friend_rows(&out).is_empty(), "the owner's list is empty again");
    assert!(store.friends(wisp_id).unwrap().is_empty());
    assert!(store.friends(tester_id).unwrap().is_empty(), "the asker's row goes too");
    // "%s has declined the friend request." - sub-op 0x32, carrying the refuser's name.
    let told = tester.collect_mail();
    let declined = told
        .iter()
        .find(|r| r.opcode == net::friends::FRIEND_RESULT && r.body[0] == 0x32)
        .expect("the decline sentence");
    assert_eq!(&declined.body[1..3], &4u16.to_le_bytes());
    assert_eq!(&declined.body[3..7], b"Wisp");

    // An answer with nothing waiting settles nothing and is still answered.
    let out = wisp.handle(&no);
    assert!(out.iter().any(|r| r.opcode == net::friends::FRIEND_RESULT));
    assert!(store.friends(wisp_id).unwrap().is_empty());
}

/// Every refusal is the client's own sentence, and **nothing is written on any of them.**
#[test]
fn every_friend_refusal_is_the_clients_own_sentence_and_writes_nothing() {
    let (mut wisp, wisp_id, mut tester, tester_id, store) = two_friends();

    // Nobody by that name -> 0x23 "That character is not registered."
    let out = tester.handle(&friend_add_body("Nobody", "Default Group"));
    assert_eq!(friend_notices(&out), vec![0x23]);
    assert!(store.friends(tester_id).unwrap().is_empty());

    // Yourself -> 0x21 "You can't enter yourself as your buddy."
    assert_eq!(friend_notices(&tester.handle(&friend_add_body("Tester2", "G"))), vec![0x21]);
    assert!(store.friends(tester_id).unwrap().is_empty());

    // Asked once: the second ask is 0x1F "Account buddy request already sent."
    tester.handle(&friend_add_body("Wisp", "Default Group"));
    assert_eq!(friend_notices(&tester.handle(&friend_add_body("Wisp", "G"))), vec![0x1F]);
    assert_eq!(store.friends(tester_id).unwrap().len(), 1, "still one row");

    // The owner, who owes the answer, asking back is NOT a refusal: it settles both rows.
    let out = wisp.handle(&friend_add_body("Tester2", "Default Group"));
    assert!(friend_notices(&out).is_empty(), "{:?}", friend_notices(&out));
    assert_eq!(store.friends(wisp_id).unwrap()[0].state, store::friends::FriendState::Accepted);

    // Now already friends -> 0x1E "That character is already registered as your buddy."
    assert_eq!(friend_notices(&tester.handle(&friend_add_body("Wisp", "G"))), vec![0x1E]);
}

/// **An undecoded sub-op changes nothing and is still answered.** Which of the client's
/// `2`/`3`, `4`/`5`, `6`/`7` pairs is accept or refuse is not known; guessing would accept
/// requests the player refused.
#[test]
fn an_undecoded_friend_sub_op_is_answered_and_changes_nothing() {
    let (mut wisp, wisp_id, mut tester, _tester_id, store) = two_friends();
    tester.handle(&friend_add_body("Wisp", "Default Group"));
    wisp.collect_mail();

    // 2, 3, 6 and 7 are the popup's own buttons and are handled; these are the friend
    // WINDOW's other operations, which are not built.
    for sub_op in [4u8, 5, 0x0B, 0x12, 0x13] {
        let mut b = net::friends::CLIENT_FRIEND_REQUEST.to_le_bytes().to_vec();
        b.push(sub_op);
        b.extend_from_slice(&_tester_id.to_le_bytes());
        b.extend_from_slice(&0u16.to_le_bytes()); // a string / flag, whichever the shape wants
        let out = wisp.handle(&b);
        // Answered - the friend window is an exclusive-request window and an unanswered
        // request latches it.
        assert!(
            out.iter().any(|r| r.opcode == net::friends::FRIEND_RESULT),
            "sub-op {sub_op} went unanswered"
        );
        assert_eq!(
            store.friends(wisp_id).unwrap()[0].state,
            store::friends::FriendState::Pending,
            "sub-op {sub_op} must not settle anything"
        );
    }

    // The group names the client hands back are logged and stored as nothing - and NOT
    // answered, which has its own test beside this one: a list reply here loops.
    let mut groups = net::friends::CLIENT_FRIEND_REQUEST.to_le_bytes().to_vec();
    groups.push(0x14);
    groups.extend_from_slice(&1u32.to_le_bytes());
    groups.push(0);
    groups.extend_from_slice(&13u16.to_le_bytes());
    groups.extend_from_slice(b"Default Group");
    assert!(wisp.handle(&groups).is_empty());
    assert_eq!(store.friends(wisp_id).unwrap().len(), 1);
}


// ==========================================================================================
// The trade window - `0x0575` mode 4. `session/trade.rs`, `research/trade-2026-09-09.md`
// ==========================================================================================

/// The `0x017E` bodies the client builds, in the shapes captured in `world-ch0.log`.
fn miniroom(mode: u32, rest: &[u32]) -> Vec<u8> {
    let mut b = net::trade::CLIENT_MINIROOM.to_le_bytes().to_vec();
    b.extend_from_slice(&mode.to_le_bytes());
    for v in rest {
        b.extend_from_slice(&v.to_le_bytes());
    }
    if mode == 3 {
        b.extend_from_slice(&[0, 0]); // the accept's two trailing bytes
    }
    b
}

/// The `(mySlot, members)` of a `0x0575` mode 4, read back off the wire the way the client
/// reads it: capacity, mySlot, then members until a byte with bit 7 set.
fn room_open_members(r: &Reply) -> (u8, u8, Vec<(u8, u32, String)>) {
    assert_eq!(r.opcode, net::trade::MINIROOM_RESULT);
    assert_eq!(u32::from_le_bytes([r.body[0], r.body[1], r.body[2], r.body[3]]), 4, "mode 4");
    assert_eq!(u32::from_le_bytes([r.body[4], r.body[5], r.body[6], r.body[7]]), 0, "A = 0 opens a room");
    assert_eq!(u32::from_le_bytes([r.body[8], r.body[9], r.body[10], r.body[11]]), 1, "roomType 1 = trade");
    let capacity = r.body[16];
    let my_slot = r.body[17];
    let mut at = 18;
    let mut members = Vec::new();
    while r.body[at] & 0x80 == 0 {
        let slot = r.body[at];
        at += 1;
        // The avatar look is `opcode::avatar_look`: 195 bytes plus five per worn item, and
        // these characters wear nothing.
        at += 195;
        let id = u32::from_le_bytes([r.body[at], r.body[at + 1], r.body[at + 2], r.body[at + 3]]);
        at += 4;
        let len = u16::from_le_bytes([r.body[at], r.body[at + 1]]) as usize;
        at += 2;
        let name = String::from_utf8(r.body[at..at + len].to_vec()).unwrap();
        at += len + 2; // the name, then the u16 nobody has identified
        members.push((slot, id, name));
    }
    assert_eq!(r.body[at], net::trade::MEMBER_LIST_END, "the list ends on a negative byte");
    assert_eq!(at + 1, r.body.len(), "and nothing follows it");
    (capacity, my_slot, members)
}

/// **The bug the owner reported.** Tester2 opens a trade and invites the owner; the owner accepts; **both**
/// get a mode 4 that opens the window, each with its own `mySlot` and both seats listed.
#[test]
fn accepting_a_trade_opens_a_window_on_both_screens() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let (mut host, host_id) = join_channel(&store, &config, &fields, account, "Tester2");
    let other = store.create_account("player", "correct horse battery").unwrap();
    let (mut guest, guest_id) = join_channel(&store, &config, &fields, other, "Wisp");
    host.on_field_entered();
    guest.on_field_entered();
    host.collect_mail();
    guest.collect_mail();

    // Create, then invite - the two packets the client sends 8 ms apart.
    assert!(host.handle(&miniroom(0, &[net::trade::ROOM_TYPE_TRADE])).is_empty(), "the create sends nothing");
    host.handle(&miniroom(5, &[guest_id]));
    let invite = guest
        .collect_mail()
        .into_iter()
        .find(|r| r.opcode == net::trade::MINIROOM_RESULT)
        .expect("the invite popup");
    assert_eq!(u32::from_le_bytes([invite.body[0], invite.body[1], invite.body[2], invite.body[3]]), 5);
    // The ticket the client will echo back is the inviter's character id.
    let ticket = u32::from_le_bytes(invite.body[invite.body.len() - 4..].try_into().unwrap());
    assert_eq!(ticket, host_id);

    // Accept: the accepter's own reply, and the host's copy over the bus.
    let mine = guest.handle(&miniroom(3, &[ticket]));
    let (capacity, my_slot, members) = room_open_members(
        mine.iter().find(|r| r.opcode == net::trade::MINIROOM_RESULT).expect("my trade window"),
    );
    assert_eq!((capacity, my_slot), (net::trade::TRADE_CAPACITY, 1), "the accepter is slot 1");
    assert_eq!(
        members,
        vec![(0, host_id, "Tester2".to_string()), (1, guest_id, "Wisp".to_string())]
    );

    let theirs = host.collect_mail();
    let (capacity, my_slot, members) = room_open_members(
        theirs.iter().find(|r| r.opcode == net::trade::MINIROOM_RESULT).expect("the host's trade window"),
    );
    assert_eq!((capacity, my_slot), (net::trade::TRADE_CAPACITY, 0), "the host is slot 0");
    assert_eq!(
        members,
        vec![(0, host_id, "Tester2".to_string()), (1, guest_id, "Wisp".to_string())],
        "both sides are told about both seats"
    );

    // A second accept of the same ticket finds the room full and sends nothing, rather than
    // opening a third window into a two-seat room.
    assert!(guest.handle(&miniroom(3, &[ticket])).is_empty());
}

/// A ticket nobody opened a room for is answered with nothing - and a declined room is gone,
/// so accepting it afterwards is that same case.
#[test]
fn a_trade_accept_without_a_room_opens_nothing() {
    let (store, config, fields, account) = shared_channel(0, 30);
    let (mut host, host_id) = join_channel(&store, &config, &fields, account, "Tester2");
    let other = store.create_account("player", "correct horse battery").unwrap();
    let (mut guest, guest_id) = join_channel(&store, &config, &fields, other, "Wisp");
    host.on_field_entered();
    guest.on_field_entered();
    host.collect_mail();
    guest.collect_mail();

    // No create: the accept has nothing to join.
    assert!(guest.handle(&miniroom(3, &[host_id])).is_empty());

    // Create, invite, decline - then the same ticket accepted is still nothing.
    host.handle(&miniroom(0, &[net::trade::ROOM_TYPE_TRADE]));
    host.handle(&miniroom(5, &[guest_id]));
    guest.collect_mail();
    guest.handle(&miniroom(6, &[host_id, 4]));
    assert!(guest.handle(&miniroom(3, &[host_id])).is_empty(), "a declined room is gone");
    assert!(host.collect_mail().iter().all(|r| r.opcode != net::trade::MINIROOM_RESULT));
}
