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

    let ack = notice_text(&s.handle(&gm_chat("!exp 250"))[0]);
    assert!(ack.starts_with("TestCharD gains 250 experience: 0 -> 250"), "{ack}");
    // The ack must not overstate what the player will see. The number reaches the client in
    // the character record, which is built on a field entry and nowhere else.
    assert!(ack.contains("Change maps"), "{ack}");

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

/// **A drag out of the inventory window is a drop**, and dropping is not built.
///
/// Measured 2026-08-20: `dst == 0`, which is not a move to slot zero - slots are 1-based
/// and 0 is the hole that makes them so. It must still be answered, and the item must
/// still be in the bag afterwards, because there is nowhere on the ground to put it.
#[test]
fn a_drop_is_answered_and_the_item_stays_in_the_bag() {
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
