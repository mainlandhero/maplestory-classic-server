//! **The Maple Administrator's three daily favours, driven through the real dispatcher.**
//!
//! `crate::dailyperks`'s own unit tests cover the table, the menu markup and the words;
//! `store::dailyperks`'s cover the UTC-midnight boundary. Neither of them proves that clicking
//! the NPC produces a menu, or that clicking a line produces a grant - and `CLAUDE.md` records
//! what happens to a subsystem that is fully decoded, implemented, tested and never connected:
//! *"on screen they look identical to not existing."*
//!
//! So every test here goes in through `Session::handle` with a real packet, opcode and all,
//! and asserts on **the database** rather than on the sentence that was sent. A test that
//! checks the reply and not the row is a test of the words.
//!
//! # Why the click arrives as `0x0151` and not `0x00F2`
//!
//! Both reach the same handler and this file uses the quest-request one because it carries the
//! NPC **template** in the packet, so no map placement has to be faked. The `0x00F2` path
//! resolves an *object* id through `config.npcs[map]` first; that mapping is `on_npc_click`'s
//! and is exercised by the taxi and shop features already.
//!
//! **`0x0151` is also the packet that would have been missed.** Quest 500005 names this NPC,
//! so the client sends the quest-shaped request rather than the click-shaped one whenever it
//! thinks that quest is offerable - the same fork that made Phil arrive as either packet.
//!
//! # What is NOT covered here, named rather than left implicit
//!
//! * **The screen.** Nobody has watched this menu draw. It is the same type-6 box the taxis
//!   are proven on (`research/fixtures/type6-menu-renders-and-taxi-rides-world.log`), which is
//!   why it was chosen, but that is a **[D]** for this NPC and not an **[L]**.
//! * **The UTC rollover against a real clock.** `store::dailyperks` tests the boundary
//!   arithmetic directly at 23:59:59Z and 00:00:00Z; here it is simulated by moving the stored
//!   claim day back one, which is the same state a real midnight produces.
//! * **The `max_mp` half of an AP refund**, and the skill-point ledger's arithmetic. Both
//!   belong to `!resetap` / `!resetsp` and have their own tests in `store::abilityspend` and
//!   `store::skillpoints`; this file asserts that the reset reached them at all.

use std::sync::Arc;

use net::opcode::Character;
use store::Store;
use world::config::Config;
use world::dailyperks::{Perk, ADMIN_TEMPLATE, LEAF_POINTS_PER_CLAIM, PERKS};
use world::session::Session;

// ---------------------------------------------------------------------------------------
// The harness
// ---------------------------------------------------------------------------------------

/// A curve with two priced levels, so "exactly the experience needed" has a number that is
/// not the one any other test uses.
const CURVE: &str = "1 | 15\n2 | 34\n";

fn config() -> Config {
    Config {
        // Without this `Session::handle` returns nothing for EVERY packet, and every
        // assertion below would fail identically whatever the feature did.
        set_field_probe: true,
        exp_curve: world::expcurve::ExpCurve::parse(CURVE),
        ..Config::default()
    }
}

/// A session with the migration claimed, which is what reaching a character needs.
fn session() -> (Session, Arc<Store>, i64, u32) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = Character { name: "Wanderer".to_string(), ..Default::default() };
    let id = store.create_character(account_id, 0, &chr).unwrap().id;
    assert!(id >= store::FIRST_CHARACTER_ID, "ids start at 200, never at 1");
    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut s = Session::new(store.clone(), Arc::new(config()));
    let note = s.claim_for_character(id);
    assert!(note.contains("claimed the migration"), "{note}");
    (s, store, account_id, id)
}

fn character(store: &Store, account_id: i64, id: u32) -> Character {
    store
        .characters_for(account_id, 0)
        .unwrap()
        .into_iter()
        .find(|c| c.id == id)
        .expect("the character exists")
}

/// A `0x0151` naming an NPC template. The 9-byte fixed head and nothing after it, which
/// `parse_quest_request` accepts and reports with both optionals `None`.
fn click(template: u32) -> Vec<u8> {
    let mut b = net::script::CLIENT_QUEST_REQUEST.to_le_bytes().to_vec();
    b.push(net::script::QUEST_ACTION_START);
    b.extend_from_slice(&0u32.to_le_bytes()); // quest id - never read on this branch
    b.extend_from_slice(&template.to_le_bytes());
    b
}

/// A real type-6 `0x00F3`, built the way the client builds it at `141f7398e`/`141f73995`.
fn pick(selection: u32) -> Vec<u8> {
    let mut b = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
    b.extend_from_slice(&0u32.to_le_bytes()); // handle, a literal 0
    b.push(net::script::SCRIPT_TYPE_MENU);
    b.push(1); // accepted
    b.extend_from_slice(&selection.to_le_bytes());
    b
}

/// The six-byte cancel, `141f7397c`.
fn close_menu() -> Vec<u8> {
    let mut b = net::script::CLIENT_SCRIPT_REPLY.to_le_bytes().to_vec();
    b.extend_from_slice(&0u32.to_le_bytes());
    b.push(net::script::SCRIPT_TYPE_MENU);
    b.push(0);
    b
}

/// Open the menu and take a favour, in the two packets a player sends.
fn take(s: &mut Session, perk: Perk) -> Vec<world::Reply> {
    let menu = s.handle(&click(ADMIN_TEMPLATE));
    assert_eq!(menu.len(), 1, "the click must answer with exactly one box");
    s.handle(&pick(perk.selection()))
}

/// The concatenated `Reply::what` of a batch - the log line, which is where every decision
/// this feature makes is written down.
fn log_of(replies: &[world::Reply]) -> String {
    replies.iter().map(|r| r.what.as_str()).collect::<Vec<_>>().join(" | ")
}

/// Every reply is a real packet with a real opcode and a non-empty body. **Always answer** is
/// not satisfied by returning a `Reply` that decodes to nothing.
fn assert_well_formed(replies: &[world::Reply]) {
    assert!(!replies.is_empty(), "silence freezes the client's whole UI");
    for r in replies {
        assert_ne!(r.opcode, 0, "opcode 0 is not a packet");
        assert!(!r.body.is_empty(), "an empty body is not a well-formed reply: {}", r.what);
        assert!(!r.what.is_empty(), "an unlabelled reply is invisible in world.log");
    }
}

/// Move a claim back one UTC day - the state a real midnight leaves behind.
/// Move a claim back one UTC day, on **the row that perk actually spends** - the account for
/// Leaf Points, the character for the other two. Rewinding the character row for an
/// account-scoped perk silently does nothing and reads as "a new day did not reopen it".
fn rewind_claim(store: &Store, account_id: i64, character_id: u32, perk: Perk) {
    let today = store::today();
    store
        .release_daily_perk(
            perk.scope(),
            claim_row(perk, account_id, character_id),
            perk.store_key(),
            Some(today - 1),
        )
        .unwrap();
}

// ---------------------------------------------------------------------------------------
// The menu
// ---------------------------------------------------------------------------------------

/// **The click is answered, and with the menu.** This is the wiring test: everything else in
/// this feature could be right and this one branch missing, and on screen that is an NPC who
/// says nothing.
#[test]
fn clicking_the_administrator_opens_the_three_option_menu() {
    let (mut s, _store, _acct, _id) = session();
    let out = s.handle(&click(ADMIN_TEMPLATE));
    assert_well_formed(&out);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].opcode, net::script::SCRIPT_MESSAGE);
    // The body is a type-6 box from this template, decoded rather than trusted.
    assert_eq!(out[0].body, net::script::npc_menu(ADMIN_TEMPLATE, &world::dailyperks::menu_text([false; 3])));
    let text = world::dailyperks::menu_text([false; 3]);
    assert!(text.contains("Gain 1000 Leaf Points"));
    assert!(text.contains("Level up"));
    assert!(text.contains("Reset AP & SP"));
    assert_eq!(text.matches("#L").count(), 3, "exactly three options, no more");
}

/// **Another NPC is untouched.** The branch matches one template and must not swallow a click
/// meant for anybody else - a QoL NPC that ate every conversation would be a worse bug than no
/// QoL NPC.
#[test]
fn another_npc_does_not_get_the_daily_menu() {
    let (mut s, _store, _acct, _id) = session();
    let out = s.handle(&click(2100)); // Heena
    for r in &out {
        assert!(
            !r.what.contains("MENU (type 6) from the Maple Administrator"),
            "the daily menu answered a click on NPC 2100: {}",
            r.what
        );
    }
}

/// Closing the box takes nothing and leaves no conversation behind for the next reply to walk
/// into. Silence is correct here and it is measured - `0x00F3` does not hold the client's
/// one-request latch (`research/script-reply.md` §5.1).
/// **The row a perk's claim lives on.** Leaf Points are account-scoped and the other two are
/// per character, so a test that hard-codes the character id reads the wrong row for one of the
/// three and reports `None` - which looks exactly like "nothing was claimed". Every assertion
/// below goes through here so the tests follow `Perk::scope` rather than restating it.
fn claim_row(perk: Perk, account_id: i64, character_id: u32) -> i64 {
    if perk.scope() == store::SCOPE_ACCOUNT {
        account_id
    } else {
        i64::from(character_id)
    }
}

#[test]
fn closing_the_menu_claims_nothing() {
    let (mut s, store, acct, id) = session();
    s.handle(&click(ADMIN_TEMPLATE));
    assert!(s.handle(&close_menu()).is_empty(), "a deliberate close needs no answer");
    for perk in PERKS {
        assert_eq!(
            store
                .daily_claim_day(perk.scope(), claim_row(perk, acct, id), perk.store_key())
                .unwrap(),
            None,
            "{perk:?} was claimed by closing the box"
        );
    }
    // And the favours still work afterwards.
    let out = take(&mut s, Perk::LeafPoints);
    assert!(log_of(&out).contains("LeafPoints PAID"), "{}", log_of(&out));
}

/// A selection the menu never offered - including the client's own `-2` - is a sentence, not
/// silence and not a claim.
#[test]
fn an_out_of_range_selection_is_answered_and_claims_nothing() {
    let (mut s, store, _acct, id) = session();
    for selection in [PERKS.len() as u32, 99, u32::MAX, u32::MAX - 1] {
        s.handle(&click(ADMIN_TEMPLATE));
        let out = s.handle(&pick(selection));
        assert_well_formed(&out);
        assert!(log_of(&out).contains("NOTHING CLAIMED"), "selection {selection}");
        for perk in PERKS {
            assert_eq!(
                store.daily_claim_day(perk.scope(), i64::from(id), perk.store_key()).unwrap(),
                None,
                "selection {selection} claimed {perk:?}"
            );
        }
    }
}

/// **A used option keeps its line and its number**, on the wire and not only in the unit test:
/// the menu that comes back after one claim still has three lines, and the perk that was used
/// is the one marked.
#[test]
fn the_menu_marks_what_has_been_used_without_moving_anything() {
    let (mut s, _store, _acct, _id) = session();
    take(&mut s, Perk::LevelUp);
    let out = s.handle(&click(ADMIN_TEMPLATE));
    assert_eq!(out.len(), 1);
    assert_eq!(
        out[0].body,
        net::script::npc_menu(ADMIN_TEMPLATE, &world::dailyperks::menu_text([false, true, false])),
        "line 1 is marked used and lines 0 and 2 are not"
    );
    assert!(out[0].what.contains("1 of 3 favours already used"), "{}", out[0].what);
}

// ---------------------------------------------------------------------------------------
// 1. Gain 1000 Leaf Points
// ---------------------------------------------------------------------------------------

/// **Every effect of the Leaf Points option**: the wallet moves by exactly 1000, the NX pot
/// beside it does not, the claim row is stamped with today, and the player is told.
#[test]
fn leaf_points_credits_the_wallet_once_and_says_so() {
    let (mut s, store, account_id, id) = session();
    let before = store.cash_wallet(account_id).unwrap();

    let out = take(&mut s, Perk::LeafPoints);
    assert_well_formed(&out);

    let after = store.cash_wallet(account_id).unwrap();
    assert_eq!(after.maple_points, before.maple_points + LEAF_POINTS_PER_CLAIM, "the LP pot");
    assert_eq!(after.nx, before.nx, "NX is a DIFFERENT pot and must not move");
    assert_eq!(
        store
            .daily_claim_day(
                Perk::LeafPoints.scope(),
                claim_row(Perk::LeafPoints, account_id, id),
                Perk::LeafPoints.store_key(),
            )
            .unwrap(),
        Some(store::today()),
        "the claim is stamped with today's UTC day"
    );
    let text = log_of(&out);
    assert!(text.contains("LeafPoints PAID"), "{text}");
    assert!(text.contains("1000 Leaf Points"), "the balance reaches the screen: {text}");
    // And the other two favours are untouched.
    for perk in [Perk::LevelUp, Perk::ResetApSp] {
        assert_eq!(
            store.daily_claim_day(perk.scope(), i64::from(id), perk.store_key()).unwrap(),
            None,
            "{perk:?} shares an allowance with Leaf Points"
        );
    }
}

/// **The second click the same day pays nothing.** `CLAUDE.md`'s Heena section: the guard was
/// asked and its answer ignored, and the payout ran anyway. This is that regression, for the
/// option where it would be cheapest to farm.
#[test]
fn leaf_points_refuses_the_second_time_the_same_day() {
    let (mut s, store, account_id, _id) = session();
    take(&mut s, Perk::LeafPoints);
    let banked = store.cash_wallet(account_id).unwrap().maple_points;

    for attempt in 0..5 {
        let out = take(&mut s, Perk::LeafPoints);
        assert_well_formed(&out); // a refusal is still a dialogue
        assert!(log_of(&out).contains("NOTHING PAID"), "attempt {attempt}: {}", log_of(&out));
        assert_eq!(
            store.cash_wallet(account_id).unwrap().maple_points,
            banked,
            "attempt {attempt} moved the wallet"
        );
    }
}

/// And after UTC midnight it pays again. The rollover itself is
/// `store::dailyperks::a_claim_at_the_last_second_of_a_day_reopens_one_second_later`; this is
/// the same state seen from the session's side.
#[test]
fn leaf_points_pays_again_the_next_utc_day() {
    let (mut s, store, account_id, id) = session();
    take(&mut s, Perk::LeafPoints);
    let after_one = store.cash_wallet(account_id).unwrap().maple_points;

    rewind_claim(&store, account_id, id, Perk::LeafPoints);
    let out = take(&mut s, Perk::LeafPoints);
    assert!(log_of(&out).contains("LeafPoints PAID"), "{}", log_of(&out));
    assert_eq!(
        store.cash_wallet(account_id).unwrap().maple_points,
        after_one + LEAF_POINTS_PER_CLAIM,
        "a new UTC day is a new allowance"
    );
}

// ---------------------------------------------------------------------------------------
// 2. Level up
// ---------------------------------------------------------------------------------------

/// **Exactly the experience needed, and no more.** The character arrives at the new level with
/// zero banked toward the one after it, which is what "exactly" means and is the thing a
/// generous implementation would get wrong invisibly.
#[test]
fn level_up_grants_precisely_the_shortfall_to_the_next_level() {
    let (mut s, store, account_id, id) = session();
    let mut chr = character(&store, account_id, id);
    chr.exp = 5; // level 1 costs 15, so the shortfall is 10
    store.save_character_progress(&chr).unwrap();
    let before = character(&store, account_id, id);

    let out = take(&mut s, Perk::LevelUp);
    assert_well_formed(&out);

    let after = character(&store, account_id, id);
    assert_eq!(after.level, before.level + 1, "exactly one level");
    assert_eq!(after.exp, 0, "and nothing left over - 5 banked plus 10 granted is exactly 15");
    assert_eq!(after.ap, before.ap + 5, "the level's ability points, from LevelGains");
    assert!(after.max_hp > before.max_hp, "and the level's HP");
    assert!(after.max_mp > before.max_mp, "and the level's MP");
    assert!(log_of(&out).contains("+10 exp"), "the log names the number: {}", log_of(&out));
    assert!(log_of(&out).contains("LevelUp PAID"), "{}", log_of(&out));
    assert_eq!(
        store
            .daily_claim_day(Perk::LevelUp.scope(), i64::from(id), Perk::LevelUp.store_key())
            .unwrap(),
        Some(store::today())
    );
}

/// A character with nothing banked pays the whole price, and still lands on zero.
#[test]
fn level_up_from_zero_experience_costs_the_whole_level() {
    let (mut s, store, account_id, id) = session();
    let out = take(&mut s, Perk::LevelUp);
    assert!(log_of(&out).contains("+15 exp"), "{}", log_of(&out));
    let after = character(&store, account_id, id);
    assert_eq!((after.level, after.exp), (2, 0));
}

/// **It does not level twice, and it does not overshoot.** One favour is one level, whatever
/// the curve says the next one costs.
#[test]
fn level_up_refuses_the_second_time_the_same_day() {
    let (mut s, store, account_id, id) = session();
    take(&mut s, Perk::LevelUp);
    let after_one = character(&store, account_id, id);

    let out = take(&mut s, Perk::LevelUp);
    assert_well_formed(&out);
    assert!(log_of(&out).contains("NOTHING PAID"), "{}", log_of(&out));
    let now = character(&store, account_id, id);
    assert_eq!((now.level, now.exp, now.ap), (after_one.level, after_one.exp, after_one.ap));
}

/// The level cap is respected, **and the refusal does not cost the day**. A player who is told
/// "you are at the cap" and then finds their reset gone too has been charged for nothing.
#[test]
fn a_character_at_the_level_cap_is_refused_without_spending_the_day() {
    let (mut s, store, account_id, id) = session();
    let mut chr = character(&store, account_id, id);
    chr.level = world::expcurve::MAX_LEVEL;
    store.save_character_progress(&chr).unwrap();

    let out = take(&mut s, Perk::LevelUp);
    assert_well_formed(&out);
    assert!(log_of(&out).contains("NOTHING CLAIMED"), "{}", log_of(&out));
    assert_eq!(
        store
            .daily_claim_day(Perk::LevelUp.scope(), i64::from(id), Perk::LevelUp.store_key())
            .unwrap(),
        None,
        "a refusal before the claim must leave the day intact"
    );
    assert_eq!(character(&store, account_id, id).level, world::expcurve::MAX_LEVEL);
}

/// A level with no row in the curve has no price, so there is nothing to grant - and again the
/// day survives. With the shipped `data/exp-curve.txt` this is unreachable; with a truncated
/// one it is the difference between a sentence and a client that looks frozen.
#[test]
fn a_level_the_curve_does_not_price_is_refused_without_spending_the_day() {
    let (mut s, store, account_id, id) = session();
    let mut chr = character(&store, account_id, id);
    chr.level = 3; // CURVE prices 1 and 2 only
    store.save_character_progress(&chr).unwrap();

    let out = take(&mut s, Perk::LevelUp);
    assert_well_formed(&out);
    assert!(log_of(&out).contains("NOTHING CLAIMED"), "{}", log_of(&out));
    assert_eq!(
        store
            .daily_claim_day(Perk::LevelUp.scope(), i64::from(id), Perk::LevelUp.store_key())
            .unwrap(),
        None
    );
    assert_eq!(character(&store, account_id, id).level, 3, "and the level did not move");
}

// ---------------------------------------------------------------------------------------
// 3. Reset AP & SP
// ---------------------------------------------------------------------------------------

/// **Every effect of the reset, and the conservation law under it.** Points spent on the four
/// stats, points spent on max HP, and learned skills all come back - and the total is
/// conserved rather than invented: the AP refunded is exactly what the stats and the ledger
/// gave up.
#[test]
fn reset_returns_the_stats_the_hp_ledger_and_the_skills() {
    let (mut s, store, account_id, id) = session();
    let mut chr = character(&store, account_id, id);
    chr.strength = 24;
    chr.dexterity = 10;
    chr.intelligence = 4;
    chr.luck = 4;
    chr.ap = 3; // some already unspent, which must survive
    store.save_character_progress(&chr).unwrap();
    // One point into max HP, recorded the only moment it is distinguishable from a level-up.
    store.record_ap_spend(id, 1, 0).unwrap();
    let before = character(&store, account_id, id);
    store.set_skill_level(id, 1_000_000, 3).unwrap();
    assert_eq!(store.skills(id).unwrap().len(), 1);

    let out = take(&mut s, Perk::ResetApSp);
    assert_well_formed(&out);

    let after = character(&store, account_id, id);
    // (24-4) + (10-4) + 0 + 0 = 26 from the stats, plus 1 from the HP ledger.
    let expected_refund = 26 + 1;
    assert_eq!(after.ap, before.ap + expected_refund, "points in equals points out");
    assert_eq!(
        (after.strength, after.dexterity, after.intelligence, after.luck),
        (4, 4, 4, 4),
        "every stat back to the floor"
    );
    assert_eq!(
        after.max_hp,
        before.max_hp - net::abilityup::policy::MAX_HP_PER_AP,
        "the HP that point bought is removed with the constant that granted it"
    );
    assert_eq!(store.ap_spend(id).unwrap().total(), 0, "and the ledger is cleared");
    assert!(store.skills(id).unwrap().is_empty(), "the skill is unlearned");
    assert!(log_of(&out).contains("ResetApSp PAID"), "{}", log_of(&out));
}

/// **The second reset the same day changes nothing**, and that is a stronger claim than it
/// looks: `!resetap` is idempotent by construction, so a broken gate here would be invisible
/// on the stats. What it would not be invisible on is the *skills* - so this one re-learns a
/// skill between the two attempts and checks it is still there.
#[test]
fn reset_refuses_the_second_time_the_same_day() {
    let (mut s, store, account_id, id) = session();
    store.set_skill_level(id, 1_000_000, 3).unwrap();
    take(&mut s, Perk::ResetApSp);
    assert!(store.skills(id).unwrap().is_empty());

    // Spend again, the way a player would between two clicks.
    let mut chr = character(&store, account_id, id);
    chr.strength = 30;
    store.save_character_progress(&chr).unwrap();
    store.set_skill_level(id, 1_000_000, 2).unwrap();

    let out = take(&mut s, Perk::ResetApSp);
    assert_well_formed(&out);
    assert!(log_of(&out).contains("NOTHING PAID"), "{}", log_of(&out));
    let after = character(&store, account_id, id);
    assert_eq!(after.strength, 30, "the second reset did not run");
    assert_eq!(store.skills(id).unwrap().len(), 1, "and the skill is still learned");
}

/// Nothing spent means nothing to give back, **and the day is not spent either**.
#[test]
fn a_character_with_nothing_to_reset_is_refused_without_spending_the_day() {
    let (mut s, store, account_id, id) = session();
    let mut chr = character(&store, account_id, id);
    // The creation roll is 12/5/4/4, which is already above the floor - so "nothing spent"
    // has to be set up explicitly rather than assumed of a fresh character.
    chr.strength = 4;
    chr.dexterity = 4;
    chr.intelligence = 4;
    chr.luck = 4;
    store.save_character_progress(&chr).unwrap();
    assert_eq!(store.skills(id).unwrap().len(), 0);
    assert_eq!(store.ap_spend(id).unwrap().total(), 0);

    let out = take(&mut s, Perk::ResetApSp);
    assert_well_formed(&out);
    assert!(log_of(&out).contains("NOTHING CLAIMED"), "{}", log_of(&out));
    assert_eq!(
        store
            .daily_claim_day(Perk::ResetApSp.scope(), i64::from(id), Perk::ResetApSp.store_key())
            .unwrap(),
        None,
        "a no-op must not eat a day"
    );
}

// ---------------------------------------------------------------------------------------
// The three together
// ---------------------------------------------------------------------------------------

/// **All three in one day, then none of them.** Three options that shared a gate would be one
/// option, and three that shared nothing would be no gate at all.
#[test]
fn all_three_can_be_taken_once_each_in_a_day_and_no_more() {
    let (mut s, store, account_id, id) = session();
    let mut chr = character(&store, account_id, id);
    chr.strength = 20; // something to reset
    store.save_character_progress(&chr).unwrap();

    for perk in PERKS {
        let out = take(&mut s, perk);
        assert_well_formed(&out);
        assert!(log_of(&out).contains("PAID:"), "{perk:?} was refused on its first use");
    }
    let banked = store.cash_wallet(account_id).unwrap().maple_points;
    let after_all = character(&store, account_id, id);
    assert_eq!(banked, LEAF_POINTS_PER_CLAIM);
    assert_eq!(after_all.level, 2);

    // The menu now shows all three used, and every one refuses.
    let menu = s.handle(&click(ADMIN_TEMPLATE));
    assert_eq!(
        menu[0].body,
        net::script::npc_menu(ADMIN_TEMPLATE, &world::dailyperks::menu_text([true; 3]))
    );
    // **A second click refuses, and this deliberately does not pin which refusal.** There are
    // two shapes - "you already had this today" and the pre-claim "there is nothing to give
    // back" - and which one a spent `Reset AP & SP` produces depends on an ordering decision
    // (`grant_daily_perk` step 0) that is about the *sentence*, not about the gate. Pinning it
    // here would make a wording improvement look like a regression. What must hold, and what is
    // asserted, is that no favour ran twice and that the refusal was said out loud.
    for perk in PERKS {
        let out = take(&mut s, perk);
        assert_well_formed(&out);
        let log = log_of(&out);
        assert!(!log.contains("PAID:"), "{perk:?} paid twice in one day: {log}");
        assert!(
            log.contains("NOTHING PAID") || log.contains("NOTHING CLAIMED"),
            "{perk:?} neither paid nor refused out loud: {log}"
        );
    }
    assert_eq!(store.cash_wallet(account_id).unwrap().maple_points, banked);
    assert_eq!(character(&store, account_id, id).level, after_all.level);
}

/// **Leaf Points are per ACCOUNT; the other two are per character.** The owner, 2026-09-08: *"Make
/// leaf point claim per account."* A second character on the same account is refused the Leaf
/// Points a first character already took, and the wallet stays at one grant - but that same
/// second character still has its own level-up, because an account-wide allowance there would
/// mean levelling one character spent the other five's turn.
///
/// Both halves are in one test deliberately. Asserting only the refusal would pass against a
/// version that made every perk account-scoped, which is the coupling this scope must not have.
#[test]
fn leaf_points_are_one_per_account_while_the_other_perks_stay_per_character() {
    let (mut s, store, account_id, id) = session();
    take(&mut s, Perk::LeafPoints);
    assert_eq!(store.cash_wallet(account_id).unwrap().maple_points, LEAF_POINTS_PER_CLAIM);

    let other = store
        .create_character(
            account_id,
            0,
            &Character { name: "Second".to_string(), ..Default::default() },
        )
        .unwrap()
        .id;
    store.create_migration(account_id, other, 0, 0).unwrap();
    let mut s2 = Session::new(store.clone(), Arc::new(config()));
    assert!(s2.claim_for_character(other).contains("claimed the migration"));

    // The SECOND character is refused the Leaf Points the first one already took.
    let out = take(&mut s2, Perk::LeafPoints);
    assert!(log_of(&out).contains("NOTHING PAID"), "{}", log_of(&out));
    assert_eq!(
        store.cash_wallet(account_id).unwrap().maple_points,
        LEAF_POINTS_PER_CLAIM,
        "one grant for the account, not one per character - the wallet must not have moved"
    );
    // The first character is refused too, so the refusal is about the ACCOUNT's spent day and
    // not about either character in particular.
    let refused = take(&mut s, Perk::LeafPoints);
    assert!(log_of(&refused).contains("NOTHING PAID"), "{}", log_of(&refused));

    // **But the second character still has its own level-up.** Without this, the test above
    // would pass against a version that scoped all three to the account.
    let levelled = take(&mut s2, Perk::LevelUp);
    assert!(
        log_of(&levelled).contains("LevelUp PAID"),
        "a per-character perk must survive the account-scoped one being spent: {}",
        log_of(&levelled)
    );
    assert_ne!(id, other, "two characters, one Leaf Point allowance between them");
}

/// A relog is not a new day. The gate is a database row, not session state - which is the
/// difference between a daily allowance and a per-connection one.
#[test]
fn a_new_session_does_not_reopen_a_claim() {
    let (mut s, store, account_id, id) = session();
    take(&mut s, Perk::LeafPoints);
    let banked = store.cash_wallet(account_id).unwrap().maple_points;

    store.create_migration(account_id, id, 0, 0).unwrap();
    let mut fresh = Session::new(store.clone(), Arc::new(config()));
    assert!(fresh.claim_for_character(id).contains("claimed the migration"));
    let out = take(&mut fresh, Perk::LeafPoints);
    assert!(log_of(&out).contains("NOTHING PAID"), "{}", log_of(&out));
    assert_eq!(store.cash_wallet(account_id).unwrap().maple_points, banked);
}

/// **Every path answers.** `CLAUDE.md`: an unanswered packet freezes the client's entire UI,
/// including the quit prompt, and reads on screen as a crash. This walks the whole feature -
/// menu, each grant, each refusal, an out-of-range pick - and asserts that the only silent
/// outcome is the one that is measured to be safe (the player's own Close).
#[test]
fn no_path_through_this_feature_is_silent_except_a_deliberate_close() {
    let (mut s, store, account_id, id) = session();
    let mut chr = character(&store, account_id, id);
    chr.strength = 20;
    store.save_character_progress(&chr).unwrap();

    // Open, take, re-open, be refused - twice round, for each option.
    for _round in 0..2 {
        for perk in PERKS {
            assert_well_formed(&s.handle(&click(ADMIN_TEMPLATE)));
            assert_well_formed(&s.handle(&pick(perk.selection())));
        }
    }
    // A pick with no menu open falls through to the ordinary script path, which is silent by
    // design and is safe: the dialog is already gone and `0x00F3` holds no latch.
    assert!(s.handle(&pick(0)).is_empty());
    // And the deliberate close.
    s.handle(&click(ADMIN_TEMPLATE));
    assert!(s.handle(&close_menu()).is_empty());
}
