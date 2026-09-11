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
    assert_eq!(s.fields.with_drops(net::opcode::START_MAP_ID, |d| d.len()), 1, "and it is on the floor");
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
    assert_eq!(s.fields.with_drops(net::opcode::START_MAP_ID, |d| d.len()), 1);
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
    assert_eq!(s.fields.with_drops(net::opcode::START_MAP_ID, |d| d.len()), 1);
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
    // Every `!` command is gated on the account's GM flag, and these helpers exist to
    // drive them. `maplecw` is the GM account on the owner's machine too.
    store.set_gm("maplecw", true).unwrap();
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
        set_field_probe: true,
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
    // Buy rows first (potion, quest item, subi), then the Sell rows for whatever may be sold.
    let mut at = net::classicshop::CLASSIC_HEAD_LEN;
    let mut priced = 0;
    for row in &rows {
        let len = row.wire_len();
        if net::bag::bundle_has_serial(row.item_id) {
            let bits = u64::from_le_bytes(body[at + len - 12..at + len - 4].try_into().unwrap());
            assert_eq!(f64::from_bits(bits), 0.3, "Subi's unitPrice, on the {} row", if row.sell { "Sell" } else { "Buy" });
            priced += 1;
        }
        at += len;
    }
    assert_eq!(at, body.len(), "walked every row by its own width");
    assert_eq!(priced, 2, "the Buy row and its Sell twin both carry it");
    assert!(out[0].what.contains("2 rechargeable with a unit price"), "{}", out[0].what);
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

/// Every refusal path is a `0x055E`, because the window latched on send: a potion, a star this
/// counter does not stock (Wolbi, when Lucy lists only Subi), a stack already full, and an
/// empty slot. **None of them moves an item or a meso.**
#[test]
fn recharging_anything_but_a_stocked_star_with_room_is_refused_with_a_result() {
    let (mut s, store, id) = recharge_session(500, 1_000);
    store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_000_000, 3), 200).unwrap(); // slot 2
    store.add_item(id, store::InventoryType::Use, &store::Item::bundle(2_070_001, 10), 500).unwrap(); // slot 3
    s.open_shop_for(21, id).unwrap();
    for (slot, why) in [(1u16, "a full stack"), (2, "a potion"), (3, "a star this counter does not list"), (7, "an empty slot")] {
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
    // Two buy rows - a quest item is perfectly buyable - and one sell row, because the
    // quest item gets none. The owner's rule is about SELLING, not stocking.
    let rows = u16::from_le_bytes([out[0].body[19], out[0].body[20]]);
    assert_eq!(rows, 3, "rowCount is a u16, at the end of the 21-byte head");
    assert_eq!(
        out[0].body.len(),
        net::classicshop::CLASSIC_HEAD_LEN + 3 * net::classicshop::CLASSIC_ROW_LEN
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
    let rows = &out[0].body[net::classicshop::CLASSIC_HEAD_LEN..];

    // **The tab is a FLAG on the classic row, not the price's sign.** Shop2 filed a row by
    // `cmp dword [rbx+0x58],0 / jg`; the classic window carries an explicit sell byte as the
    // second-to-last of the row's 157. Same rule, different encoding - and reading it the old
    // way here would have put every row in the Buy tab and passed.
    let mut sell_rows = Vec::new();
    let mut buy_rows = Vec::new();
    for i in 0..3usize {
        let at = i * net::classicshop::CLASSIC_ROW_LEN;
        let item_id = u32::from_le_bytes(rows[at + 8..at + 12].try_into().unwrap());
        let sell = rows[at + net::classicshop::CLASSIC_ROW_LEN - 2];
        if sell == 0 { buy_rows.push(item_id) } else { sell_rows.push(item_id) }
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
    assert!(out[0].what.contains("not among the 3 rows"), "{}", out[0].what);
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
        set_field_probe: true,
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

/// The whole `207xxxx` family counts - Hwabi as much as Subi - and the last star empties the
/// slot with a REMOVE rather than a count of zero.
#[test]
fn every_star_in_the_family_counts_and_the_last_one_empties_the_slot() {
    let Some((store, mut s, id)) = rogue_with(CLAW, HWABI, 1) else { return };
    let out = s.handle(&swing_packet(net::combat::USER_SHOOT_ATTACK, 0));
    assert_eq!(arrows_in_slot_1(&store, id), 0);
    let op = out
        .iter()
        .find(|r| r.opcode == net::inventory::INVENTORY_OPERATION)
        .expect("the emptied slot must be reported");
    assert_eq!(op.body[7], net::inventory::MODE_REMOVE);
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
        set_field_probe: true,
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
        set_field_probe: true,
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
        let config = Config { set_field_probe: true, npcs, ..Config::default() };
        let mut s = Session::new(store.clone(), Arc::new(config));
        s.claim_for_character(made.id);
        (s, store, made.id)
    };
    let job_of = |store: &Arc<Store>, id: u32| {
        store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().job
    };

    // ---- eligible: level 10 and STR at the minimum ----
    let (mut s, store, id) = build(crate::jobs::LEVEL_MINIMUM, crate::jobs::STAT_MINIMUM);
    let out = s.handle(&npc_click(1000));
    assert_eq!(job_of(&store, id), 100, "the job must persist, not just be announced");
    let stat = out
        .iter()
        .find(|r| r.opcode == net::stats::STAT_CHANGED)
        .expect("the client must be told, or it draws the old job forever");
    assert!(stat.what.contains("job 0 -> 100"), "{}", stat.what);
    assert!(stat.what.contains("skill points"), "and the SP, or the + button stays grey: {}", stat.what);
    assert!(
        out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE),
        "and a sentence, or nothing on screen says what happened"
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
    assert_eq!(job_of(&store, id), 100);
    let out = s.handle(&npc_click(1000));
    assert!(
        !out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
        "a second click must not re-advance or re-grant SP"
    );
    assert_eq!(job_of(&store, id), 100, "and the job is unchanged");
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
            set_field_probe: true,
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
        set_field_probe: true,
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
    assert_eq!(out[0].body[0], net::classicshop::RESULT_SUCCESS, "{}", out[0].what);
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
        set_field_probe: true,
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
        set_field_probe: true,
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
        set_field_probe: true,
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
fn setrates_stores_the_rate_and_announces_it_at_once() {
    let (mut s, store, _) = gm_session();
    let out = s.handle(&gm_chat("!setrates 2 1 1 1 30"));

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
    s.handle(&gm_chat("!setrates 2 1 1 1 30"));
    let out = s.handle(&gm_chat("!setrates 2 3 1 1 30"));

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
    s.handle(&gm_chat("!setrates 2 1 1 1 30"));
    let out = s.handle(&gm_chat("!setrates 1 1 1 1 30"));

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
    let out = s.handle(&gm_chat("!setrates 1 1 1 1 30"));
    assert!(banners(&out).is_empty(), "{out:?}");
    assert!(notice_text(&out[0]).contains("already"), "{out:?}");
}

/// One rate ending while another is still running puts BOTH on the banner: the ending and
/// the survivor.
#[test]
fn an_ending_and_a_survivor_share_the_banner() {
    let (mut s, _, _) = gm_session();
    s.handle(&gm_chat("!setrates 2 1 1 1 30"));
    s.handle(&gm_chat("!setrates 2 3 1 1 30"));
    let out = s.handle(&gm_chat("!setrates 1 3 1 1 30"));

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
    let map = net::opcode::START_MAP_ID;

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
        "!rates must not touch set_at either - that would restart the banner cycle"
    );
}

/// `!droprate` announces itself like the other two, and `!drop rate` is the same command.
#[test]
fn the_drop_rate_announces_through_setrates() {
    let (mut s, store, _) = gm_session();
    let out = s.handle(&gm_chat("!setrates 1 1 4 1 30"));
    assert_eq!(store.rates().unwrap().drop.rate.per_cent(), 400);
    assert_eq!(
        banners(&out),
        vec![Some("[Event] The Server's Drop rate has been set to 4x".to_string())]
    );

}

/// All three at once share one banner, in a fixed order.
#[test]
fn three_events_share_one_banner() {
    let (mut s, _, _) = gm_session();
    s.handle(&gm_chat("!setrates 2 1 1 1 30"));
    s.handle(&gm_chat("!setrates 2 3 1 1 30"));
    let out = s.handle(&gm_chat("!setrates 2 3 4 1 30"));
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

    s.handle(&gm_chat("!setrates 1 1 10 1 30"));
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


/// `!setrates` sets all three, on one anchor, with one banner naming all of them.
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
        "one timestamp, or the banner cycle is re-anchored per rate"
    );
    let banner = banners(&out);
    assert_eq!(banner.len(), 1, "one banner, not three: {out:?}");
    let text = banner[0].clone().unwrap();
    for want in ["EXP rate has been set to 2x", "Meso rate has been set to 3x", "Drop rate has been set to 5x"] {
        assert!(text.contains(want), "{want} missing from {text}");
    }
}

/// `!setrates 1 1 1` is the one-command way to end everything.
#[test]
fn setrates_all_ones_ends_every_event() {
    let (mut s, store, _) = gm_session();
    s.handle(&gm_chat("!setrates 2 3 5 1 30"));
    let out = s.handle(&gm_chat("!setrates 1 1 1 1 30"));
    assert!(store.rates().unwrap().all_normal());
    let text = banners(&out)[0].clone().unwrap();
    for want in ["The EXP rate-up event has ended.", "The Meso rate-up event has ended.", "The Drop rate-up event has ended."] {
        assert!(text.contains(want), "{want} missing from {text}");
    }
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
    assert_eq!(rows[0].chance_bp, crate::droptables::BASIS_POINTS, "100%, not the scraped 40%");
    // **The global table is no longer empty**, and this used to assert that it was. The owner put
    // the three scrolls in it on 2026-09-09. The concern behind the old assertion still
    // stands - an extra item on the floor during the step that teaches picking things up -
    // so this bounds it instead of forbidding it: every global row must be 1 basis point, so
    // a tutorial Sentinel drops something extra about once in 3 333 kills.
    for row in table.global() {
        assert_eq!(
            row.chance_bp, 1,
            "a global row above 1bp would land on the tutorial floor: {row:?}"
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
    let map = s.claimed_character().unwrap().map_id;

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
    assert!(
        out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE),
        "and the reason, which is the half that already worked"
    );
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
    let map = s.claimed_character().unwrap().map_id;
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
        set_field_probe: true,
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
    assert_eq!(grant.body.len(), 1 + net::cashshop::CASH_ITEM_RECORD_LEN + 4 + 1);

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
        set_field_probe: true,
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
            set_field_probe: true,
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
    assert!(out[0].what.contains("overall and a bottom cannot be worn together"), "{}", out[0].what);
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
    s.on_inventory_move(&inventory_move(net::inventory::INV_EQUIP, trousers, -6, -1));

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
    let config = Config { set_field_probe: true, ..Config::default() };
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
        set_field_probe: true,
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
        set_field_probe: true,
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
        set_field_probe: true,
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
        set_field_probe: true,
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

/// **The whole second advancement, walked the way a player walks it.**
///
/// The owner, 2026-08-31: *"the user goes to their job instructor, receive a letter from their
/// instructor, then go to the 2nd job trainer to talk, then get teleported to the special mob
/// map ... get 20 marbles for proof, receive the recommendation letter, go back to the job
/// instructor to receive their 2nd job advancement"*. That is this client's chain exactly,
/// with one correction: **it is 30 marbles, not 20** - `Check.1.item.0.count` of quest 20002
/// is `30` and the same on all four branches. **[L]**
///
/// The test walks the four legs that involve a map or an item, and asserts an effect at each
/// one rather than just the last: a chain where the middle silently does nothing still ends
/// with the right job, and that is precisely the failure this project keeps finding.
#[test]
fn the_second_advancement_walks_the_client_s_own_chain() {
    let b = &crate::secondjob::BRANCHES[0]; // Warrior
    let field = b.test_field;

    let mut npcs = std::collections::HashMap::new();
    let npc = |template: u32| net::opcode::FieldNpc {
        object_id: 1000, template_id: template, x: 0, cy: 0, fh: 1, rx0: 0, rx1: 0, f: 0,
    };
    npcs.insert(b.examiner_map_id, vec![npc(b.examiner_npc)]);
    npcs.insert(field.map_id, vec![npc(field.warden_npc)]);
    npcs.insert(b.instructor_map_id, vec![npc(b.instructor_npc)]);

    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = net::opcode::Character { name: "Thirty".to_string(), ..Default::default() };
    let mut made = store.create_character(account_id, 0, &chr).unwrap();
    made.level = crate::secondjob::LEVEL_MINIMUM;
    made.job = b.from_job; // already a Swordsman - the chain's own Check.0.job.0
    made.map_id = b.examiner_map_id;
    store.save_character_progress(&made).unwrap();
    store.set_character_map(made.id, b.examiner_map_id).unwrap();
    store.create_migration(account_id, made.id, 0, 0).unwrap();
    let config = Config { set_field_probe: true, npcs, ..Config::default() };
    let mut s = Session::new(store.clone(), Arc::new(config));
    s.claim_for_character(made.id);

    let map_of = |store: &Arc<Store>, id: u32| {
        store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().map_id
    };
    let job_of = |store: &Arc<Store>, id: u32| {
        store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().job
    };
    let held = |s: &Session, item: u32| s.held_count(made.id, item);

    // ---- leg 1: the examiner puts you inside the hidden field ----------------------------
    let out = s.handle(&npc_click(1000));
    assert_eq!(
        map_of(&store, made.id),
        field.map_id,
        "the examiner's whole job is to warp you in - that map has no portal to walk through"
    );
    assert!(
        out.iter().any(|r| r.opcode == net::notice::CHAT_NOTICE),
        "and says so first: a script box sent with a SetField is torn down by field entry"
    );

    // ---- leg 2: the warden is the only door out ------------------------------------------
    s.handle(&npc_click(1000)); // now the warden, because the map changed
    assert_eq!(
        map_of(&store, made.id),
        b.examiner_map_id,
        "and it is the client's OWN returnMap for the field, not a number we chose"
    );

    // ---- leg 3: thirty marbles buy the proof ---------------------------------------------
    // Short of thirty, the examiner sends you back in and takes nothing.
    let etc = store::InventoryType::Etc;
    store
        .add_item(made.id, etc, &store::Item::bundle(b.chain.marble_item, 29), 200)
        .unwrap();
    s.handle(&npc_click(1000));
    assert_eq!(map_of(&store, made.id), field.map_id, "29 is not 30");
    assert_eq!(held(&s, b.chain.marble_item), 29, "and nothing was taken for a failed test");
    assert_eq!(held(&s, b.chain.proof_item), 0, "and nothing was given");

    // Back out, and top up to exactly thirty.
    s.handle(&npc_click(1000)); // the warden again
    store.add_item(made.id, etc, &store::Item::bundle(b.chain.marble_item, 1), 200).unwrap();
    assert_eq!(held(&s, b.chain.marble_item), 30);

    let out = s.handle(&npc_click(1000));
    assert_eq!(
        map_of(&store, made.id),
        b.examiner_map_id,
        "a PASS does not warp - re-entering a test that is finished would strand the player"
    );
    assert_eq!(held(&s, b.chain.marble_item), 0, "the thirty are handed over");
    assert_eq!(held(&s, b.chain.proof_item), 1, "and the proof comes back");
    assert!(
        out.iter().any(|r| r.opcode == net::script::SCRIPT_MESSAGE),
        "and the examiner says so"
    );

    // ---- leg 4: the instructor, and only the instructor, advances -------------------------
    store.set_character_map(made.id, b.instructor_map_id).unwrap();
    s.claim_for_character(made.id); // re-read the character at its new map
    let out = s.handle(&npc_click(1000));
    let menu = out
        .iter()
        .find(|r| r.opcode == net::script::SCRIPT_MESSAGE)
        .expect("the instructor offers the choice");
    assert!(menu.what.contains("MENU"), "and it is a type-6 list, not a Say: {}", menu.what);
    assert_eq!(job_of(&store, made.id), b.from_job, "nothing has changed YET");

    // Picking position 0 is this branch's first choice, and the job that comes out is that
    // one rather than "the first job id in the file" - the two are only the same if the menu
    // and the decoder agree, which is the bug this asserts against.
    let want = b.choices[0].job;
    let out = s.on_script_reply(&menu_reply(Some(0)));
    assert_eq!(job_of(&store, made.id), want, "the job must PERSIST, not just be announced");
    let stat = out
        .iter()
        .find(|r| r.opcode == net::stats::STAT_CHANGED)
        .expect("the client must be told, or it draws the old job forever");
    assert!(stat.what.contains(&format!("job {} -> {want}", b.from_job)), "{}", stat.what);
    assert_eq!(held(&s, b.chain.proof_item), 0, "and the proof is spent, not left in the bag");
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
    let config = Config { set_field_probe: true, npcs, ..Config::default() };
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
    let out = s.drops_from_kill(b.test_field.mobs[1], 2000, None, 204, b.test_field.map_id);
    assert_eq!(out.len(), 1, "exactly the marble, and exactly one of it: {out:?}");
    assert!(
        out[0].what.contains(&b.chain.marble_item.to_string()),
        "and it is THIS branch's marble: {}",
        out[0].what
    );

    // The same mob on Precipice of Darkness, which is where it also lives. **[L]**
    let out = s.drops_from_kill(b.test_field.mobs[1], 2001, None, 204, 10006160);
    assert!(out.is_empty(), "an ordinary field must not pay a test's proof: {out:?}");

    // And a scraped 6% row cannot smuggle one out either: the filter runs on the ROLL, so a
    // hit on the wrong map is removed however it got there.
    let drops = crate::droptables::DropTables::parse(&format!(
        "{} | {} | 100 | 1 | 1 | 1 | Dark Marble at a certain rate\n",
        b.test_field.mobs[1], b.chain.marble_item
    ));
    s.config = Arc::new(Config { drops, ..(*s.config).clone() });
    let out = s.drops_from_kill(b.test_field.mobs[1], 2002, None, 204, 10006160);
    assert!(out.is_empty(), "a 100% table row is still refused off the field: {out:?}");
    // On the field, the table row and the guarantee do not stack.
    let out = s.drops_from_kill(b.test_field.mobs[1], 2003, None, 204, b.test_field.map_id);
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
        let config = Config { set_field_probe: true, npcs, ..Config::default() };
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
            let config = Config { set_field_probe: true, npcs, ..Config::default() };
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
        Arc::new(Config { set_field_probe: true, send_mobs: true, mobs, ..Config::default() });
    let fields = Arc::new(crate::fields::Fields::new());
    fields.seed(SHARED_MAP, &config, 0);
    fields.due_respawns(SHARED_MAP, &config, 999_999);
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
    let alive = fields.mob_count(SHARED_MAP);
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
    // Kill the one mob so its point is booked to refill while both players stand there.
    let victim = fields.mobs_on(SHARED_MAP)[0].spawn.object_id;
    fields.hurt(SHARED_MAP, victim, 999, 200, &config, 0);
    assert_eq!(fields.mob_count(SHARED_MAP), 0);

    let (mut ticker, _) = join_channel(&store, &config, &fields, account, "Ticker");
    let (mut watcher, _) = join_channel(&store, &config, &fields, account, "Watcher");
    ticker.on_field_entered();
    watcher.on_field_entered();
    let _ = ticker.tick(1_000);
    let _ = watcher.tick(1_000);

    let spawned = ticker.tick(1_000 + crate::config::DEFAULT_RESPAWN_MS);
    assert_eq!(count_of(&spawned, net::mob::MOB_ENTER_FIELD), 1, "the refill: {spawned:?}");
    assert_eq!(
        count_of(&spawned, net::mobmove::MOB_CHANGE_CONTROLLER),
        1,
        "claimed by whichever session ticked first"
    );

    let seen = watcher.tick(2_000 + crate::config::DEFAULT_RESPAWN_MS);
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
    assert_eq!(fields.mobs_on(SHARED_MAP)[0].spawn.object_id, 2000, "the fixture's mob");

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
        fields.mob_position(SHARED_MAP, 2000),
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
        fields.mob_position(SHARED_MAP, 2000),
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
        set_field_probe: true,
        send_mobs: true,
        mobs,
        drops: crate::droptables::DropTables::parse("2 | 4000001 | 100 | 1 | 1 | 9 | Shell\n"),
        ..Config::default()
    });
    let fields = Arc::new(crate::fields::Fields::new());
    fields.seed(SHARED_MAP, &config, 0);
    fields.due_respawns(SHARED_MAP, &config, 999_999);
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
        fields.controllers().controller_of(SHARED_MAP, 2002),
        Some(owner.subscriber.get()),
        "the control: the owner walked in first and holds it"
    );

    let out = other.handle(&melee_packet());
    let grants: Vec<&Reply> =
        out.iter().filter(|r| r.opcode == net::mobmove::MOB_CHANGE_CONTROLLER).collect();
    assert_eq!(grants.len(), 1, "the attacker is handed the mob: {out:?}");
    assert_eq!(grants[0].body[0], net::mobmove::CONTROL_NORMAL, "granted, not released");
    assert_eq!(
        fields.controllers().controller_of(SHARED_MAP, 2002),
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
        fields.controllers().controller_of(SHARED_MAP, 2002),
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
    fields.hurt(SHARED_MAP, 2002, 90, helper_id, &config, 0);
    assert_eq!(fields.mob_hp(SHARED_MAP, 2002), Some(10));

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
        fields.with_drops(SHARED_MAP, |d| d.on_field(SHARED_MAP).map(|x| x.owner_id).collect());
    assert_eq!(owners, vec![helper_id], "killer {killer_id} owns nothing here");
}

/// **An expired drop fades on the OWNER's screen, not on whichever session happened to tick.**
///
/// The drop table is shared by every connection on the channel and every one of them sweeps,
/// so before this the first to tick removed the drop and took the `0x046F` - the owner went on
/// drawing an item that no longer existed, and a bystander was handed a leave packet for an
/// object its pool never held. Neither is visible from one screen.
#[test]
fn an_expired_drop_fades_for_its_owner_and_not_for_the_session_that_swept_it() {
    let (store, config, fields, account) = shared_channel(1, 30);
    let (mut sweeper, _) = join_channel(&store, &config, &fields, account, "Sweeper");
    let (mut owner, owner_id) = join_channel(&store, &config, &fields, account, "Owner");
    sweeper.on_field_entered();
    owner.on_field_entered();
    let _ = sweeper.handle(&NO_PACKET);
    let _ = owner.handle(&NO_PACKET);

    let (drop_id, _) = fields.with_drops(SHARED_MAP, |d| {
        d.drop_from_mob(crate::drops::DropFromMob {
            map_id: SHARED_MAP,
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
        "the sweeping session was never shown this drop and must not be told it faded: {swept:?}"
    );
    assert_eq!(fields.with_drops(SHARED_MAP, |d| d.len()), 0, "but it IS gone from the floor");

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

    fields.with_drops(SHARED_MAP, |d| {
        d.drop_from_mob(crate::drops::DropFromMob {
            map_id: SHARED_MAP,
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
    let alive = fields.mob_count(SHARED_MAP);
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
    assert_eq!(fields.mob_count(SHARED_MAP), alive, "and the mobs themselves are untouched");
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

    let object_id = fields.mobs_on(SHARED_MAP)[0].spawn.object_id;
    let spawn_x = fields.mobs_on(SHARED_MAP)[0].spawn.x;
    let walked_to = (spawn_x + 300, 395);
    fields.note_position(SHARED_MAP, object_id, walked_to);

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
        &fields.mobs_on(SHARED_MAP)[0].as_seen(),
        net::mobmove::CONTROL_NORMAL,
    );
    assert_eq!(grants[0].body, expected_here, "granted where it is standing");

    // The control that gives the assertion above its teeth: the spawn-point body is a
    // DIFFERENT packet, so this test would fail if the handover sent that instead.
    let spawn_body = net::mobmove::mob_change_controller(
        &fields.mobs_on(SHARED_MAP)[0].spawn,
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
    let alive = fields.mob_count(SHARED_MAP);
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
    let alive = fields.mob_count(SHARED_MAP);
    let (mut only, _) = join_channel(&store, &config, &fields, account, "Only");
    only.on_field_entered();
    assert_eq!(fields.controllers().held_by(only.subscriber.get()), alive);

    only.on_log_out();
    assert_eq!(fields.controllers().held_by(only.subscriber.get()), 0, "nobody to hand them to");
    assert_eq!(fields.mob_count(SHARED_MAP), alive, "the mobs are still alive on the map");

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
    let alive = fields.mob_count(SHARED_MAP);
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
    let config = Arc::new(Config { set_field_probe: true, ..Config::default() });

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
        // `set_field_probe`, or `handle` answers nothing and the `!setrates` below is dropped
        // on the floor - which reads exactly like the rate not applying.
        let config = Config {
            set_field_probe: true,
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
        set_field_probe: true,
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
        set_field_probe: true,
        send_mobs: true,
        mobs,
        mob_templates,
        firstjob: crate::firstjob::CombatTable::load(table),
        ..Config::default()
    });
    let fields = Arc::new(crate::fields::Fields::new());
    fields.seed(SHARED_MAP, &config, 0);
    fields.due_respawns(SHARED_MAP, &config, 999_999);
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
    assert_eq!(fields.mob_hp(SHARED_MAP, 2002), Some(500), "the control");
    let out = s.on_user_hit(&drake_hit(200, 2002));
    assert_eq!(hp_of(&s), 1000 - 160, "20% of the hit never lands");
    assert_eq!(fields.mob_hp(SHARED_MAP, 2002), Some(460), "and the 40 goes to the mob");
    assert!(
        out.iter().any(|r| r.what.starts_with("mob 2002 took 40")),
        "the mob's bar is told through deal_to_mob: {out:?}"
    );
    // A hit from a mob that is not on this field reflects onto nothing and is otherwise the
    // same hit - the reflected share is still not taken from the player.
    let before = hp_of(&s);
    s.on_user_hit(&drake_hit(200, 7777));
    assert_eq!(hp_of(&s), before - 160);
    assert_eq!(fields.mob_hp(SHARED_MAP, 2002), Some(460));
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

    assert_eq!(fields.mobs_on(SHARED_MAP)[0].spawn.x, 100, "the spawn point, a decoy");
    assert_eq!(fields.mob_site(SHARED_MAP, 2000), Some((456, 395)));

    controller.last_position = Some((1500, 395));
    let out = controller.deal_to_mob(SHARED_MAP, 2000, 9_999, chr_id);
    assert!(
        out.iter().any(|r| r.opcode == net::drops::DROP_ENTER_FIELD),
        "the kill must actually have dropped something, or every number below is vacuous: {out:?}"
    );

    let xs: Vec<i16> =
        fields.with_drops(SHARED_MAP, |d| d.on_field(SHARED_MAP).map(|x| x.x).collect());
    assert_eq!(xs.len(), 1);
    assert_eq!(
        xs[0], 456,
        "the end of the path - not 424 (the head), 100 (the spawn point) or 1500 (the player)"
    );

    // The arc's ORIGIN too. An item flying out of empty space 32 px behind the corpse is
    // exactly as wrong on screen as one landing there, and it is a separate field.
    let src = fields.with_drops(SHARED_MAP, |d| {
        d.on_field(SHARED_MAP).map(|x| (x.source_x, x.source_y)).next().unwrap()
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
        set_field_probe: true,
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
    let config = Config { portals, portal_index, set_field_probe: true, ..Config::default() };
    let mut s = Session::new(store, Arc::new(config));
    assert!(s.claim_for_character(id).contains("claimed the migration"));
    (s, id)
}

/// **The owner's own press, byte for byte, walks through the door.** `world.log` 2026-09-10
/// 11:27:53.035: `0x014A`, body `000400696e30333a0303f4`. Yesterday this was logged UNKNOWN
/// and answered with nothing, with the destination sitting in the table the whole time.
#[test]
fn wisps_script_portal_press_reaches_ellinia_station() {
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

    // The database moved it: out of the locker, into the Cash tab at slot 3.
    assert!(store.cash_locker(account).unwrap().is_empty(), "the locker row is gone");
    let bag = store.bag_items(id, cash).unwrap();
    assert_eq!(bag.len(), 1);
    assert_eq!((bag[0].slot, bag[0].item.item_id), (3, 5680004));
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

/// **Entering the shop lists what the locker holds**, one `0x0C` per row between the stage
/// packet and the wallet - so an item bought and left in the Cash Inventory is still there
/// on the next visit. New 2026-09-10; before it only the current visit's purchase appeared.
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
    let listed: Vec<&Reply> = out
        .iter()
        .filter(|r| r.opcode == net::cashshop::CASH_SHOP_RESULT
            && r.body[0] == net::cashshop::RESULT_ITEM_TO_LOCKER)
        .collect();
    assert_eq!(listed.len(), 2, "one 0x0C per locker row: {ops:?}");
    // Each carries the serial the move request will echo: (account << 32) | slot.
    for (r, slot) in listed.iter().zip([1u64, 2]) {
        let serial = u64::from_le_bytes(r.body[1..9].try_into().unwrap());
        assert_eq!(serial, (account as u64) << 32 | slot, "{}", r.what);
        assert!(r.what.contains("LOCKER LISTING"), "{}", r.what);
    }
    // And an empty locker lists nothing extra.
    let (mut s2, _store2, _) = cash_shop_session();
    let out2 = s2.on_cash_shop_request(&[0u8; 5]);
    assert!(out2.iter().all(|r| r.opcode != net::cashshop::CASH_SHOP_RESULT), "{:?}", out2.iter().map(|r| r.opcode).collect::<Vec<_>>());
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
    let config = Config { item_names: names, set_field_probe: true, ..Config::default() };
    let mut s = Session::new(store.clone(), Arc::new(config));
    assert!(s.claim_for_character(id).contains("claimed the migration"));
    let before = store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();

    let out = s.handle(&gm_chat("!hair 42540"));
    assert!(notice_text(&out[0]).contains("hair is now 42540 (Frieren Hair)"), "{}", notice_text(&out[0]));
    assert!(out.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "the re-entry SetField: {out:?}");
    let out = s.handle(&gm_chat("!face 22035"));
    assert!(notice_text(&out[0]).contains("face is now 22035"), "{}", notice_text(&out[0]));
    let after = store.characters_for(account, 0).unwrap();
    let me = after.iter().find(|c| c.id == id).unwrap();
    assert_eq!((me.hair, me.face), (42540, 22035), "persisted");
    assert_eq!(me.map_id, before.map_id, "re-entry is the SAME map");

    // Controls: outside the id space, an id with no name, and a face id given to !hair.
    for (cmd, why) in [("!hair 1302000", "outside the hair id space"), ("!face 42540", "outside the face id space"), ("!hair 42541", "no name")] {
        let out = s.handle(&gm_chat(cmd));
        assert!(notice_text(&out[0]).contains("REFUSED"), "{cmd} should refuse ({why}): {}", notice_text(&out[0]));
        assert!(!out.iter().any(|r| r.opcode == net::opcode::SET_FIELD), "{cmd}: a refusal must not warp");
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

    // Open Frieren's.
    let frieren = crate::signaturestyle::set_for_coupon(5_681_543).unwrap();
    let fslot = store.bag(id).unwrap().items_in(store::InventoryType::Cash).find(|i| i.item.item_id == 5_681_543).unwrap().slot;
    let out = s.on_use_cash_item(&use_cash_item_body(fslot, 5_681_543));
    assert!(out.iter().any(|r| r.what.contains("10 item(s) given")), "{:?}", out.iter().map(|r| &r.what).collect::<Vec<_>>());
    let bag = store.bag(id).unwrap();
    let use_tab: Vec<u32> = bag.items_in(store::InventoryType::Use).map(|i| i.item.item_id).collect();
    let equip_tab: Vec<u32> = bag.items_in(store::InventoryType::Equip).map(|i| i.item.item_id).collect();
    assert_eq!(use_tab.len(), 4, "three hair coupons and a face coupon: {use_tab:?}");
    for c in frieren.hair_coupons.iter().chain([&frieren.face_coupon]) {
        assert!(use_tab.contains(c), "{c} missing");
    }
    assert_eq!(equip_tab.len(), 6, "{equip_tab:?}");
    for e in frieren.equips {
        assert!(equip_tab.contains(e), "{e} missing");
    }
    assert_eq!(bag.items_in(store::InventoryType::Cash).count(), 7, "the Frieren coupon is gone, seven remain");
    // Every hand-out is told to the client: one InventoryOperation per row placed.
    assert!(out.iter().filter(|r| r.opcode == net::inventory::INVENTORY_OPERATION).count() >= 10);
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
