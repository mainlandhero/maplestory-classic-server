//! The world session's tests, moved wholesale out of `session.rs`.
//!
//! They stay in one file on purpose. Splitting them per topic would mean either
//! duplicating the fixtures or inventing a shared test-support module, and the
//! fixtures are the part most likely to drift.

use super::*;

fn session() -> (Session, Arc<Store>, i64, u32) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
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
    let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut item_names = std::collections::HashMap::new();
    item_names.insert(1302000u32, "Sword".to_string());
    item_names.insert(2000000u32, "Red Potion".to_string());
    // `set_field_probe` is the master switch: with it clear, `Session::handle` returns
    // nothing for EVERY packet. It is off in `Config::default()` and it is the same trap
    // that cost a client launch on 2026-08-20 - a bare launcher line without
    // `-SetFieldProbe` left the character on the select screen.
    let config = Config { item_names, set_field_probe: true, ..Config::default() };
    let mut s = Session::new(store.clone(), Arc::new(config));
    assert!(s.claim_for_character(id).contains("claimed the migration"));
    (s, store, id)
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

/// `!exp` awards experience, persists it, and says so.
///
/// The persistence half is the point. Experience that is announced but not written down
/// would look identical on the ack line and be gone at the next field entry, which is the
/// exact shape of the unequip bug that goal I existed to fix.
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
    // **Three slots, not one stack of three**, and that is correct here: this config has
    // no `item_data`, so `info/slotMax` is unknown, and an unknown stack size is treated
    // as 1. That is the safe direction - a merge that does not happen, rather than one
    // that silently destroys the overflow - and it is what a server with no
    // `gm-handbook/itemdata.txt` will do on a real run.
    assert_eq!(rows.len(), 3, "unknown slotMax means one per slot");
    assert!(rows.iter().all(|r| r.item.kind.quantity() == 1));
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
    assert_eq!(s.fields.with_drops(net::opcode::START_MAP_ID, |d| d.len()), 1, "and it is on the floor");
}

/// The pick-up reads the drop id at **offset 13**, the offset one run measured.
///
/// It used to search every byte offset, because the layout was unknown. It is known now,
/// and the builder can never be read to confirm it further - it lives in `.themida`, whose
/// `SizeOfRawData` is zero.
#[test]
fn the_pick_up_handler_reads_the_drop_id_at_the_measured_offset() {
    let (mut s, store, id) = gm_session();
    let map = net::opcode::START_MAP_ID;
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
    let chr = net::opcode::Character { name: "TestCharD".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    store.create_migration(account_id, id, 0, 0).unwrap();
    let config = Config {
        set_field_probe: true,
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

    assert_eq!(out.len(), 1, "exactly one reply, and never zero");
    assert_eq!(out[0].opcode, net::opcode::MIGRATE_COMMAND);
    // The address must be the TARGET channel's, not this one's. Getting that backwards
    // sends the client to the channel it is already on.
    assert_eq!(&out[0].body[4..8], &[127, 0, 0, 1]);
    assert_eq!(&out[0].body[8..10], &8486u16.to_le_bytes(), "channel 1's port");
    assert!(out[0].what.contains("NOT authentication"), "{}", out[0].what);

    // And a migration was minted for the target, so the other channel can claim it.
    let mut other = Session::new(store, Arc::new(Config { channel_id: 1, ..Config::default() }));
    assert!(other.claim_for_character(id).contains("claimed the migration"));
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
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
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
    let shop = crate::shops::Shop {
        npc: "Lucy".to_string(),
        role: "Grocer".to_string(),
        map_label: "Maple Road".to_string(),
        items: vec![plain(2000000, 50, 5, false), plain(4031507, 20, 1, true)],
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
    let table = crate::shops::ShopTable { shops: vec![shop], item_data, problems: Vec::new() };

    let mut npcs = std::collections::HashMap::new();
    npcs.insert(
        net::opcode::START_MAP_ID,
        vec![net::opcode::FieldNpc {
            object_id: 1000,
            template_id: 21,
            x: 0,
            cy: 0,
            fh: 1,
            rx0: 0,
            rx1: 0,
            f: 0,
        }],
    );
    let mut shop_by_template = std::collections::HashMap::new();
    shop_by_template.insert(21u32, 0usize);

    let config = Config {
        set_field_probe: true,
        // The shop is OFF in `Config::default()` because `0x0560` kills the real client -
        // its window needs a WZ image this client does not ship. These tests are about the
        // packet's contents and the transaction rules, which stay correct and stay worth
        // pinning; they turn it on explicitly so the default cannot silently gut them into
        // passing against a server that sends nothing.
        send_shop: true,
        shops: table,
        shop_by_template,
        npcs,
        ..Config::default()
    };
    let mut s = Session::new(store.clone(), Arc::new(config));
    assert!(s.claim_for_character(id).contains("claimed the migration"));
    (s, store, id)
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
    assert_eq!(out[0].opcode, net::shop::OPEN_SHOP);
    assert_eq!(
        u32::from_le_bytes(out[0].body[0..4].try_into().unwrap()),
        21,
        "the NPC template the click named"
    );
    // Two buy rows - a quest item is perfectly buyable - and one sell row, because the
    // quest item gets none. The owner's rule is about SELLING, not stocking.
    assert_eq!(u16::from_le_bytes([out[0].body[4], out[0].body[5]]), 3, "rowCount is a u16");
    assert_eq!(
        out[0].body.len(),
        net::shop::OPEN_SHOP_FIXED_LEN + 3 * net::shop::SHOP_ROW_LEN
    );
    assert!(out[0].what.contains("2 buy, 1 sell"), "{}", out[0].what);
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

    let out = s.award_experience(15, "a test", true);

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

/// Experience short of a level is banked and levels nobody.
#[test]
fn experience_short_of_a_level_is_just_banked() {
    let (mut s, store, id) = gm_session();
    let curve = crate::expcurve::ExpCurve::parse("1 | 15
");
    s.config = Arc::new(Config { exp_curve: curve, ..(*s.config).clone() });

    let out = s.award_experience(14, "a test", true);
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
    assert!(s.award_experience(0, "a worthless mob", true).is_empty());
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

    let out = s.drops_from_kill(2, 2000, None, 204, net::opcode::START_MAP_ID);

    assert_eq!(out.len(), 3, "mesos, the shell, and the event item: {out:?}");
    assert!(
        out.iter().all(|r| r.opcode == net::drops::DROP_ENTER_FIELD),
        "a mob drop sends ONLY 0x046E - a 0x0070 would refuse an inventory request the          player never made"
    );
    assert_eq!(s.fields.with_drops(net::opcode::START_MAP_ID, |d| d.len()), 3, "and all three are on the floor");
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
    let map = net::opcode::START_MAP_ID;
    let mut mobs = std::collections::HashMap::new();
    mobs.insert(map, vec![net::mob::FieldMob::new(2000, 2, 100, 395, 1, 30)]);
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

    s.drops_from_kill(2, 2000, None, 204, net::opcode::START_MAP_ID);
    let d = s.fields.with_drops(net::opcode::START_MAP_ID, |d| d.on_field(net::opcode::START_MAP_ID).cloned().collect::<Vec<_>>()).into_iter().next().unwrap();
    assert_eq!(d.x, 777);
}

/// **A killed mob comes back.** The owner: *"The mobs that I kill also do not respawn."*
///
/// The delay is the WZ's own `mobTime`; a spawn point with none uses the field rate, and
/// reading that `0` as "never" would empty a map after one pass.
#[test]
fn a_dead_mob_respawns_when_its_timer_is_due() {
    let (mut s, _, _) = gm_session();
    let map = net::opcode::START_MAP_ID;
    let mut mobs = std::collections::HashMap::new();
    mobs.insert(map, vec![net::mob::FieldMob::new(2000, 2, 500, 395, 1, 30)]);
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

    assert_eq!(s.drops_from_kill(999_999, 2000, None, 204, 1).len(), 1);
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

    let out = s.drops_from_kill(2, 2000, None, 204, 1);
    assert!(!out.is_empty(), "it must answer");
    assert!(out.iter().all(|r| r.opcode != net::drops::DROP_ENTER_FIELD), "and drop nothing");
    assert_eq!(s.fields.with_drops(net::opcode::START_MAP_ID, |d| d.len()), 0);
}

/// With no drop table at all, a kill is silent - not a panic and not a notice.
#[test]
fn a_kill_with_no_table_drops_nothing_quietly() {
    let (mut s, _, _) = gm_session();
    s.last_position = Some((1, 1));
    assert!(s.drops_from_kill(2, 2000, None, 204, 1).is_empty());
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
fn with_the_shop_off_a_shopkeeper_falls_through_to_dialogue() {
    let (mut s, _, _) = shop_session();
    s.config = Arc::new(Config { send_shop: false, ..(*s.config).clone() });

    let out = s.handle(&npc_click(1000));
    assert!(!out.is_empty(), "a click must always be answered");
    assert!(
        out.iter().all(|r| r.opcode != net::shop::OPEN_SHOP),
        "the packet that kills the client must not go out"
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
    assert_eq!(u16::from_le_bytes([body[4], body[5]]), 1, "one row");
    assert_eq!(body.len(), net::shop::OPEN_SHOP_FIXED_LEN + net::shop::SHOP_ROW_LEN);
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
    assert_eq!(u16::from_le_bytes([out[0].body[4], out[0].body[5]]), 1, "clamped up to one");
}

/// **A quest item gets no sell row, so the player is never offered the option.**
///
/// The owner: *"Please do not allow quest items to be sold."* That is about selling, not
/// stocking - an NPC may perfectly well sell you a quest item, and this one does. The
/// store refuses the transaction too; this is the same rule one layer earlier, where the
/// row never appears in the Sell tab at all.
///
/// The tab a row lands in is its price's **sign**: `140d23ac5 cmp dword [rbx+0x58],0 /
/// jg` files positive into the Buy tab and the rest into Sell.
#[test]
fn a_quest_item_is_never_given_a_sell_row() {
    let (mut s, _, _) = shop_session();
    let out = s.handle(&npc_click(1000));
    let rows = &out[0].body[net::shop::OPEN_SHOP_FIXED_LEN - 1..];

    let mut sell_rows = Vec::new();
    let mut buy_rows = Vec::new();
    for i in 0..3usize {
        let at = i * net::shop::SHOP_ROW_LEN;
        let item_id = u32::from_le_bytes(rows[at + 4..at + 8].try_into().unwrap());
        let price = i32::from_le_bytes(rows[at + 8..at + 12].try_into().unwrap());
        if price > 0 { buy_rows.push(item_id) } else { sell_rows.push(item_id) }
    }
    assert_eq!(buy_rows, vec![2000000, 4031507], "both are stocked");
    assert_eq!(sell_rows, vec![2000000], "the quest item is not buyable back");
}

/// A purchase charges the SERVER's price, adds the item, and says so three ways.
#[test]
fn buying_charges_mesos_and_delivers_the_item() {
    let (mut s, store, id) = shop_session();
    store.set_mesos(id, 1000).unwrap();
    s.handle(&npc_click(1000));

    // sub-op 1: u32 rowKey, u16 quantity, u16 slot. Row 0 is the potion's buy row.
    let mut body = net::shop::CLIENT_SHOP_REQUEST.to_le_bytes().to_vec();
    body.push(net::shop::SHOP_REQ_TRANSACTION);
    body.extend_from_slice(&0u32.to_le_bytes());
    body.extend_from_slice(&3u16.to_le_bytes());
    body.extend_from_slice(&0u16.to_le_bytes());
    let out = s.handle(&body);

    assert_eq!(out[0].opcode, net::shop::SHOP_TRANSACTION_RESULT);
    assert_eq!(out[0].body[1], net::shop::ShopResult::Success.code());
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

    let mut body = net::shop::CLIENT_SHOP_REQUEST.to_le_bytes().to_vec();
    body.push(net::shop::SHOP_REQ_TRANSACTION);
    body.extend_from_slice(&0u32.to_le_bytes());
    body.extend_from_slice(&1u16.to_le_bytes());
    body.extend_from_slice(&0u16.to_le_bytes());
    let out = s.handle(&body);

    assert_eq!(out[0].body[1], net::shop::ShopResult::NotEnoughMesos.code());
    assert_eq!(store.mesos(id).unwrap(), 10, "nothing was charged");
    assert!(store.bag(id).unwrap().is_empty(), "and nothing was delivered");
}

/// A row key we never sent is refused **and** the list is re-sent, because that result
/// code makes the client re-request it.
#[test]
fn an_unknown_row_key_is_refused_and_the_list_is_re_sent() {
    let (mut s, _, _) = shop_session();
    s.handle(&npc_click(1000));

    let mut body = net::shop::CLIENT_SHOP_REQUEST.to_le_bytes().to_vec();
    body.push(net::shop::SHOP_REQ_TRANSACTION);
    body.extend_from_slice(&99u32.to_le_bytes());
    body.extend_from_slice(&1u16.to_le_bytes());
    body.extend_from_slice(&0u16.to_le_bytes());
    let out = s.handle(&body);

    assert_eq!(out[0].body[1], net::shop::ShopResult::UnknownItem.code());
    assert!(net::shop::ShopResult::UnknownItem.rerequests());
    assert_eq!(out[1].opcode, net::shop::OPEN_SHOP, "a re-requesting code owes a fresh list");
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
        set_field_probe: true,
        fields: Config::load_fields(path),
        ..Config::default()
    };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
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
        set_field_probe: true,
        npcs: [(40u32, npcs)].into_iter().collect(),
        npc_strings: strings,
        ..Config::default()
    };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
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
        set_field_probe: true,
        npcs: [(40u32, npcs)].into_iter().collect(),
        npc_strings: strings,
        ..Config::default()
    };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
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

    let plain = record_of(Config { set_field_probe: true, ..Config::default() });
    assert_eq!(
        read(&plain),
        vec![net::opcode::DEFAULT_INVENTORY_SLOTS; net::opcode::INVENTORY_COUNT],
        "a new character reached the wire with no bag"
    );
    assert_eq!(plain[net::opcode::PRESENCE_INVENTORY_SIZE], 1);

    let forced = record_of(Config {
        set_field_probe: true,
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
        set_field_probe: true,
        npcs: [(40u32, npcs)].into_iter().collect(),
        npc_strings: strings,
        ..Config::default()
    };
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
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

fn hex(s: &str) -> Vec<u8> {
    let clean: String = s.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    (0..clean.len() / 2)
        .map(|i| u8::from_str_radix(&clean[i * 2..i * 2 + 2], 16).unwrap())
        .collect()
}


// ---------------------------------------------------------------------------------------
// The server's EXP and meso rates, and the banner that announces them.
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

/// `!exprate 2` stores the rate and puts the banner up **immediately**, with the owner's wording.
#[test]
fn the_exp_rate_command_sets_the_rate_and_announces_it() {
    let (mut s, store, _) = gm_session();
    let out = s.handle(&gm_chat("!exprate 2"));

    assert_eq!(store.rates().unwrap().exp.rate.per_cent(), 200, "stored as hundredths");
    assert_eq!(
        banners(&out),
        vec![Some("[Event] The Server's EXP rate has been set to 2x".to_string())],
        "the change announces itself rather than waiting for the next tick"
    );
    assert!(notice_text(&out[0]).contains("2x"), "and the GM who typed it is told: {out:?}");
}

/// Both rates share one banner, because one banner is all the client has.
#[test]
fn two_rates_produce_one_banner_carrying_both() {
    let (mut s, _, _) = gm_session();
    s.handle(&gm_chat("!exprate 2"));
    let out = s.handle(&gm_chat("!mesorate 3"));

    assert_eq!(
        banners(&out),
        vec![Some(
            "[Event] The Server's EXP rate has been set to 2x [Event] The Server's Meso rate has been set to 3x"
                .to_string()
        )]
    );
}

/// Back to 1x, and the END is announced rather than the banner simply vanishing.
///
/// The owner, 2026-08-20: *"When either EXP or Meso is set back to 1x again, you should also
/// immediately display a scrolling notice."*
#[test]
fn returning_to_normal_announces_the_end() {
    let (mut s, store, _) = gm_session();
    s.handle(&gm_chat("!exprate 2"));
    let out = s.handle(&gm_chat("!exprate 1"));

    assert_eq!(store.rates().unwrap().exp.rate, store::rates::Rate::NORMAL);
    assert_eq!(
        banners(&out),
        vec![Some("[Event] The EXP rate-up event has ended.".to_string())]
    );
}

/// `!exprate 1` on a server that was never running an event announces nothing.
///
/// The failure this guards against is noisy rather than broken: three "event has ended"
/// banners on a fresh server, for events nobody ran.
#[test]
fn ending_an_event_that_never_started_says_nothing() {
    let (mut s, _, _) = gm_session();
    let out = s.handle(&gm_chat("!exprate 1"));
    assert!(banners(&out).is_empty(), "{out:?}");
    assert!(notice_text(&out[0]).contains("already"), "{out:?}");
}

/// One rate ending while another is still running puts BOTH on the banner: the ending and
/// the survivor.
#[test]
fn an_ending_and_a_survivor_share_the_banner() {
    let (mut s, _, _) = gm_session();
    s.handle(&gm_chat("!exprate 2"));
    s.handle(&gm_chat("!mesorate 3"));
    let out = s.handle(&gm_chat("!exprate 1"));

    assert_eq!(
        banners(&out),
        vec![Some(
            "[Event] The EXP rate-up event has ended. [Event] The Server's Meso rate has been set to 3x".to_string()
        )]
    );
}

/// The banner is sent when the answer CHANGES and not otherwise. Re-sending it restarts the
/// scroll on screen, so a tick that has nothing new to say must say nothing.
#[test]
fn a_tick_with_nothing_new_sends_no_banner() {
    let (mut s, _, _) = gm_session();
    s.handle(&gm_chat("!exprate 2"));
    for tick in 1..=6u64 {
        let out = s.tick(tick * 500);
        assert!(
            banners(&out).is_empty(),
            "tick {tick} re-sent a banner that was already on screen: {out:?}"
        );
    }
}

/// `!meso rate 2` - the two-word spelling the owner used - is the same command.
#[test]
fn the_two_word_spellings_work() {
    let (mut s, store, _) = gm_session();
    s.handle(&gm_chat("!meso rate 2"));
    assert_eq!(store.rates().unwrap().meso.rate.per_cent(), 200);
    s.handle(&gm_chat("!exp rate 1.5"));
    assert_eq!(store.rates().unwrap().exp.rate.per_cent(), 150);
}

/// A bad argument is refused **and changes nothing**. The dangerous failure here is a
/// command that says something plausible and leaves the rate half-set.
#[test]
fn a_bad_multiplier_changes_nothing() {
    let (mut s, store, _) = gm_session();
    s.handle(&gm_chat("!exprate 2"));
    for bad in ["fast", "0", "1000", "1.234", "-2"] {
        let out = s.handle(&gm_chat(&format!("!exprate {bad}")));
        assert!(banners(&out).is_empty(), "{bad} moved the banner: {out:?}");
        assert_eq!(store.rates().unwrap().exp.rate.per_cent(), 200, "{bad} changed the rate");
    }
}

/// With no argument a setter reports, because "the multiplier is applied" and "the
/// multiplier was never stored" look identical from inside the game. It reports through the
/// same path as `!rates`, so the two can never disagree.
#[test]
fn the_rate_commands_report_when_given_nothing() {
    let (mut s, _, _) = gm_session();
    s.handle(&gm_chat("!exprate 2"));
    let out = s.handle(&gm_chat("!exprate"));
    let said = notice_text(&out[0]);
    assert!(said.contains("EXP 2x"), "{said}");
    assert!(said.contains("Meso 1x"), "{said}");
    assert!(said.contains("Drop 1x"), "{said}");
    assert!(banners(&out).is_empty(), "reporting is not a change");
}

/// Setting a rate to what it already is does not restart the five-minute cycle.
#[test]
fn setting_the_same_rate_again_is_a_no_op() {
    let (mut s, _, _) = gm_session();
    s.handle(&gm_chat("!exprate 2"));
    let out = s.handle(&gm_chat("!exprate 2"));
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
    s.handle(&gm_chat("!exprate 2"));
    assert_eq!(s.exp_for_kill(2).0, 30);
    s.handle(&gm_chat("!exprate 1.5"));
    assert_eq!(s.exp_for_kill(2).0, 22, "truncated, not rounded");
    assert!(s.exp_for_kill(2).1.contains("1.5x"), "and the log says why");
}

/// `!exp` is NOT multiplied. It means "give me exactly this much".
#[test]
fn the_exp_command_is_not_multiplied() {
    let (mut s, store, id) = gm_session();
    s.handle(&gm_chat("!exprate 10"));
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
    let map = net::opcode::START_MAP_ID;

    s.handle(&gm_chat("!mesorate 3"));
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
    assert!(quiet.contains("No event is running"), "{quiet}");

    s.handle(&gm_chat("!droprate 2.5"));
    let loud = notice_text(&s.handle(&gm_chat("!rates"))[0]);
    assert!(loud.contains("Drop 2.5x"), "{loud}");
    assert!(!loud.contains("No event is running"), "{loud}");
}

/// `!rates` changes nothing - no banner, no stored rate.
#[test]
fn the_rates_command_is_read_only() {
    let (mut s, store, _) = gm_session();
    s.handle(&gm_chat("!exprate 2"));
    let before = store.rates().unwrap();
    let out = s.handle(&gm_chat("!rates"));
    assert!(banners(&out).is_empty(), "reporting is not a change: {out:?}");
    assert_eq!(
        store.rates().unwrap(),
        before,
        "!rates must not touch set_at either - that would restart the banner cycle"
    );
}

/// `!droprate` announces itself like the other two, and `!drop rate` is the same command.
#[test]
fn the_drop_rate_command_sets_and_announces() {
    let (mut s, store, _) = gm_session();
    let out = s.handle(&gm_chat("!droprate 4"));
    assert_eq!(store.rates().unwrap().drop.rate.per_cent(), 400);
    assert_eq!(
        banners(&out),
        vec![Some("[Event] The Server's Drop rate has been set to 4x".to_string())]
    );

    s.handle(&gm_chat("!drop rate 2"));
    assert_eq!(store.rates().unwrap().drop.rate.per_cent(), 200, "the two-word spelling");
}

/// All three at once share one banner, in a fixed order.
#[test]
fn three_events_share_one_banner() {
    let (mut s, _, _) = gm_session();
    s.handle(&gm_chat("!exprate 2"));
    s.handle(&gm_chat("!mesorate 3"));
    let out = s.handle(&gm_chat("!droprate 4"));
    assert_eq!(
        banners(&out),
        vec![Some(
            "[Event] The Server's EXP rate has been set to 2x [Event] The Server's Meso rate has been set to 3x [Event] The Server's Drop rate has been set to 4x".to_string()
        )]
    );
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
    let map = net::opcode::START_MAP_ID;

    // 5000 % 10000 = 5000, which is above 10% (1000 bp) and below 100%.
    let entry = &s.config.drops.for_mob(2)[0];
    assert!(!entry.hits_at(5_000, store::rates::Rate::NORMAL), "1x must miss this roll");
    assert!(
        entry.hits_at(5_000, store::rates::Rate::from_per_cent(1_000)),
        "10x makes a 10% row certain"
    );

    s.handle(&gm_chat("!droprate 10"));
    s.drops_from_kill(2, 2000, Some((500, 395)), 204, map);
    assert_eq!(
        s.fields.with_drops(map, |d| d.len()),
        1,
        "at 10x a 10% row drops every time"
    );
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

/// A quest that DOES have completion lines speaks them, and chains nothing.
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
    assert!(said.iter().any(|w| w.contains("on path \"1\"")), "{said:?}");
    assert!(said.iter().any(|w| w.contains("quest 1001 completed")), "{said:?}");
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
