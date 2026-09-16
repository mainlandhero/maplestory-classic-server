//! Shanks, the Southperry ferryman: 1000 mesos to Lith Harbor, free if you finished Mai.
//!
//! The owner, 2026-09-09: *"Shanks should ask the player for 1000 mesos for them to leave
//! Southperry and go to Lith Harbor, unless they have completed 'Mai's Final Training'
//! questline, to which Shanks will teleport the player for free."* And on the shape:
//!
//! > *"Shanks should say that going to Lith Harbor will cost the player 1000 mesos. If the
//! > player clicks 'Yes', then it checks whether or not the player has completed the quest.
//! > (If they haven't, the mesos is deducted and the teleport is executed) If they have,
//! > Shanks should have an additional dialogue that exclaims that the player actually
//! > finished Mai's training and they have heard about them, and they'll take them for free."*
//!
//! Labels are the project's: **[L]** read off this client's data, **[I]** policy.
//!
//! # Why this is not a row in `crate::taxi`
//!
//! It was, briefly. A taxi is a **menu** of stops and its opening quotes one fare; this is a
//! **yes/no** whose price is fixed before the answer and whose waiver is a *surprise after
//! it*. `taxi::Step` has no arm that says something extra on the way to the ride, so putting
//! Shanks there meant either quoting "free" up front - which the owner explicitly did not ask for -
//! or bolting a second text onto `Ride`. They also have exactly one destination, and
//! `taxi::destinations` derives stops from the other rows in a network, so a one-row network
//! would offer an **empty** menu: the same blank screen as the bug being fixed.
//!
//! One real thing came back with that detour and is kept: putting them in `Network::Ossyria`
//! made every Ossyria port offer Southperry as a stop - a ferry from El Nath to the tutorial
//! island - and `taxi`'s own per-network count test caught it immediately.
//!
//! # The waiver is its own box, and the boat leaves when that box is dismissed
//!
//! The owner, 2026-09-16: *"the dialogue that they'll waive it because you have finished Mai's
//! Final Training does not show. However, Shanks does correctly waive the fee and TP the
//! players."* The first build sent the free line as a Say **in the same batch as the
//! `SetField`**, under a comment claiming that order made it safe. It does not: field entry
//! runs the script-manager reset, so the box was built and torn down in the same frame - the
//! exact failure `taxi` records costing a run, and the reason `jobguide::Step::Ride` sends no
//! script at all. So the free crossing is now **two replies**: Yes puts the waiver on screen
//! as its own OK box ([`Step::Announce`], parked at [`ANNOUNCE_PATH`]) and takes nothing;
//! the boat sails when the player dismisses that box. A paid crossing still sails on Yes.
//!
//! **And the waiver is for Beginners only** - same message: *"please make this fee waiver
//! only work for Beginners."* A Swordsman who finished Mai's training pays the thousand like
//! anyone else; Shanks has heard of the *beginner* who did it.
//!
//! # This module is the decision and the words. It knows nothing about `Session`
//!
//! Same shape as `crate::jobguide` and `crate::taxi`: every branch below is a unit test
//! rather than a client run, because a run costs the owner a manual elevated launch.
//!
//! # Nothing here authenticates
//!
//! The channel socket carries no credentials. Passage is sold to whoever holds it.

use net::opcode::Character;
use store::Store;

use crate::config::Config;

/// Shanks' `Npc.wz` template. **[L]** `gm-handbook/npcs.txt`:
/// `60, 15, 3223, -13, 140, 3173, 3273, 0, Shanks` - and they appear on exactly one map.
pub const TEMPLATE: u32 = 15;

/// Southperry, where they stand. **[L]** `gm-handbook/maps.txt`: `60, Southperry`.
pub const HOME_MAP: u32 = 60;

/// Lith Harbor, where they sail. **[L]** `gm-handbook/maps.txt`: `10000000, Lith Harbor`.
pub const DESTINATION_MAP: u32 = 10_000_000;

/// The fare. **The owner's number.** Nothing in this client's data carries a ferry price.
pub const FARE_MESOS: u32 = 1_000;

/// *Mai's Final Training*, whose completion waives the fare.
///
/// **[L]** `gm-handbook/questlines.txt`: `1010  QuestInfo  name  Mai's Final Training`, whose
/// `parent` is `Mai's Training`. `1009` is *Mai's Training*, the earlier step, and is
/// deliberately **not** this id - the owner named the **Final** one.
pub const MAIS_FINAL_TRAINING: u32 = 1010;

/// The `Conversation::path` that marks Shanks' open question.
///
/// It must not collide with a quest path (`""`, `"0"`, `"0.yes"`), `crate::taxi`'s
/// `"taxi.menu"`, or `crate::jobguide`'s prefix. A test asserts the last two.
pub const ASK_PATH: &str = "shanks.ask";

/// The `Conversation::path` parked while the waiver box is on screen. The reply to that box
/// - OK or Close, either - is what sails the boat.
pub const ANNOUNCE_PATH: &str = "shanks.free";

/// The job the waiver is for. **[I]** the owner, 2026-09-16: Beginners only.
pub const BEGINNER_JOB: u16 = 0;

/// Is this click Shanks, standing where Shanks stands?
///
/// **The map is part of the question**, the same way `taxi::taxi_for` takes one: a template
/// that turns up on a second map later must not silently become a second ferry port.
pub fn is_shanks(template: u32, map: u32) -> bool {
    template == TEMPLATE && map == HOME_MAP
}

/// What Shanks does next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    /// Put the yes/no on screen and keep the conversation open.
    Ask { text: String },
    /// **The fare is waived, and Shanks says so on its own box first.** Nothing is charged
    /// and nobody moves yet; the conversation parks at [`ANNOUNCE_PATH`] and the boat sails
    /// when the box is dismissed ([`on_announce_dismissed`]). Sent alone, never beside a
    /// `SetField`, because that is what tore it down.
    Announce { text: String },
    /// **Every guard passed.** `fare` (which may be zero) has been taken and the character
    /// warps to [`DESTINATION_MAP`]. A paying passenger gets the meso line and the new map,
    /// which is already two pieces of feedback; a free one already had the waiver box.
    ///
    /// **Nothing that assumed a refusal may precede this.** It is the only arm that carries a
    /// destination, so a refusal cannot warp anybody by accident. And it sends no script.
    Sail { fare: u32, balance: u32 },
    /// Say `text` and drop the conversation. `why` is the log label.
    ///
    /// **A refusal is a reply** - `CLAUDE.md`'s *always answer*. "You do not have 1000 mesos"
    /// and "I cannot sail today" both live here and neither may be silence.
    Done { text: String, why: String },
}

/// The opening question. **Always quotes [`FARE_MESOS`]**, even for a player who will not pay
/// it.
///
/// The owner asked for the waiver to be a surprise *after* Yes, and that is a deliberate piece of
/// writing rather than an oversight: the fare is what the crossing costs, and Shanks having
/// heard of you is the reward. Quoting "free" here would spend the moment before it arrives.
pub fn opening() -> Step {
    Step::Ask {
        text: format!(
            "So you're after passage to #bLith Harbor#k? I can take you - the crossing is \
             #b{FARE_MESOS} mesos#k. Shall we set sail?"
        ),
    }
}

/// Has this character finished Mai's Final Training?
///
/// **A database error reads as "no"**, which charges full price. That is the safe direction:
/// a waiver that failed open would give the crossing away to everyone the first time the
/// quest table hiccupped, and a free ferry is invisible. Being charged 1000 mesos you should
/// have kept is visible and complainable.
pub fn passage_is_free(store: &Store, character_id: u32) -> bool {
    matches!(
        store.quest_row(character_id, MAIS_FINAL_TRAINING),
        Ok(Some(row)) if row.state == store::QuestState::Complete
    )
}

/// Whether the fare is waived for this character: **a Beginner** who finished Mai's Final
/// Training. Both halves are the rule; a test says each alone is not enough.
pub fn waiver_applies(store: &Store, chr: &Character) -> bool {
    chr.job == BEGINNER_JOB && passage_is_free(store, chr.id)
}

/// The line Shanks says when the fare is waived.
fn free_passage_line() -> String {
    "Hold on - you're the one who finished #bMai's training#k, aren't you? Word travels fast \
     on a small island. Put your money away; I'll not take a meso off you. Climb aboard!"
        .to_string()
}

/// The player answered the yes/no. `action` is the client's byte: `1` Yes, `0` No, `-1`
/// closed - `net::script::npc_ask` documents that this box is the one where those are
/// unambiguous.
///
/// The gates run in this order and the order matters:
///
/// 1. **Not Yes** -> nothing happens and nothing is charged.
/// 2. **The field table**, then the map. `Config::map_exists` is fail-open on an empty set,
///    so with no `gm-handbook/fields.txt` every id would pass; refusing out loud here is the
///    same guard `taxi::board` and `Session::gm_map` use, after the owner lost a session to a map
///    with no field image on 2026-08-20.
/// 3. **The waiver** -> the announcement box, nothing charged, nobody moved yet.
/// 4. **The money, last** - charging for a crossing we are about to refuse is the one
///    direction that takes something and gives nothing.
pub fn on_answer(store: &Store, config: &Config, chr: &Character, action: i8) -> Step {
    if action != 1 {
        return Step::Done {
            text: "Suit yourself. The tide'll wait a while yet.".to_string(),
            why: format!(
                "shanks: character {} answered {action} (not Yes). NO FARE TAKEN, \
                 NO TELEPORT",
                chr.id
            ),
        };
    }
    if let Some(refusal) = harbour_refusal(config) {
        return refusal;
    }
    if waiver_applies(store, chr) {
        return Step::Announce { text: free_passage_line() };
    }
    charge_and_sail(store, chr, FARE_MESOS)
}

/// The player dismissed the waiver box - OK or Close, it makes no difference: the decision
/// to sail was the Yes before it, and the fare is nothing, so there is nothing a Close could
/// be protecting. The guards run again because the box is modal and the socket is not; if
/// the waiver somehow no longer applies, the fare is charged exactly as a Yes would.
pub fn on_announce_dismissed(store: &Store, config: &Config, chr: &Character) -> Step {
    if let Some(refusal) = harbour_refusal(config) {
        return refusal;
    }
    let fare = if waiver_applies(store, chr) { 0 } else { FARE_MESOS };
    charge_and_sail(store, chr, fare)
}

/// Gates 2 of `on_answer`: the field table, then the map. `None` means sail on.
fn harbour_refusal(config: &Config) -> Option<Step> {
    if config.fields.is_empty() {
        return Some(Step::Done {
            text: "The harbour's fogged in. Try me again later.".to_string(),
            why: "shanks: the field table is EMPTY, so the destination could not be checked \
                  and a bad id would kill the client. Regenerate gm-handbook/fields.txt with \
                  tools/dump_portals.py, or restart the server. NO FARE TAKEN"
                .to_string(),
        });
    }
    if !config.map_exists(DESTINATION_MAP) {
        return Some(Step::Done {
            text: "The harbour's fogged in. Try me again later.".to_string(),
            why: format!(
                "shanks: map {DESTINATION_MAP} has no field image in this client, so sailing \
                 would strand the character. NO FARE TAKEN"
            ),
        });
    }
    None
}

/// Take `fare` and sail, or say why not.
fn charge_and_sail(store: &Store, chr: &Character, fare: u32) -> Step {
    // `add_mesos` reads and writes in ONE transaction and refuses to go below zero, so two
    // fast clicks cannot both be charged against the same balance. `add_mesos(0)` is a no-op
    // that still reports the balance, so the free crossing takes the same path as the paid
    // one and cannot drift from it.
    match store.add_mesos(chr.id, -i64::from(fare)) {
        Ok(balance) => Step::Sail { fare, balance },
        Err(store::StoreError::NotEnoughMesos { have, .. }) => Step::Done {
            text: format!(
                "Passage is #b{FARE_MESOS} mesos#k and you're carrying #b{have}#k. \
                 No ticket, no crossing."
            ),
            why: format!(
                "shanks: character {} has {have} mesos and the fare is \
                 {FARE_MESOS}. NO FARE TAKEN, NO TELEPORT",
                chr.id
            ),
        },
        Err(e) => Step::Done {
            text: "The harbour's fogged in. Try me again later.".to_string(),
            why: format!(
                "shanks: could not charge character {}: {e}. NO FARE TAKEN, \
                 NO TELEPORT",
                chr.id
            ),
        },
    }
}

/// The `0x055B` a step puts on screen.
///
/// # `Step::Sail` sends no script, ever
///
/// **A script message must never travel with, or just before, a `SetField`** - field entry
/// runs the script-manager reset, so the dialog is built and torn down silently. `taxi`
/// records this costing a run, and the free line was lost to it here for a week (see the
/// module doc). The waiver is `Step::Announce`, a box of its own with nothing after it.
pub fn script_replies(step: &Step) -> Vec<crate::Reply> {
    match step {
        Step::Ask { text } => vec![crate::Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_ask(TEMPLATE, text, false),
            what: format!("ScriptMessage YES/NO from Shanks ({TEMPLATE}): {text:?}"),
        }],
        Step::Done { text, why } => vec![crate::Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(TEMPLATE, text, false, false),
            what: format!("ScriptMessage Say from Shanks ({TEMPLATE}): {why}"),
        }],
        Step::Announce { text } => vec![crate::Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(TEMPLATE, text, false, false),
            what: format!(
                "ScriptMessage Say from Shanks ({TEMPLATE}): FREE passage - a Beginner with \
                 Mai's Final Training ({MAIS_FINAL_TRAINING}) complete. Nothing charged, nobody \
                 moved: the boat sails when this box is dismissed"
            ),
        }],
        Step::Sail { .. } => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store_with_character() -> (Store, Character) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = Character { name: "Sailor".to_string(), ..Default::default() };
        let chr = store.create_character(account, 0, &chr).unwrap();
        assert_eq!(chr.job, BEGINNER_JOB, "a new character is a Beginner");
        (store, chr)
    }

    fn finish_mai(store: &Store, chr: &Character) {
        store.start_quest(chr.id, MAIS_FINAL_TRAINING).unwrap();
        store.complete_quest(chr.id, MAIS_FINAL_TRAINING).unwrap();
    }

    /// A config whose field table contains Lith Harbor, so the map guard passes.
    fn config() -> Config {
        let mut c = Config::default();
        c.fields.insert(DESTINATION_MAP);
        c
    }

    #[test]
    fn he_is_only_shanks_where_shanks_stands() {
        assert!(is_shanks(TEMPLATE, HOME_MAP));
        assert!(!is_shanks(TEMPLATE, DESTINATION_MAP), "not a ferry port in Lith Harbor");
        assert!(!is_shanks(200, HOME_MAP), "a cab is not Shanks");
    }

    /// The owner asked for the price up front and the waiver as a surprise. The opening must quote
    /// the fare **even for a player who will not pay it**, so this asserts the words rather
    /// than the number alone.
    #[test]
    fn the_opening_always_quotes_the_fare() {
        let Step::Ask { text } = opening() else { panic!("Shanks opens with a question") };
        assert!(text.contains("1000 mesos"), "{text}");
        assert!(text.contains("Lith Harbor"), "{text}");
        assert!(!text.to_lowercase().contains("free"), "the waiver is not announced: {text}");
    }

    #[test]
    fn saying_no_costs_nothing_and_goes_nowhere() {
        let (store, chr) = store_with_character();
        store.add_mesos(chr.id, 5_000).unwrap();
        for action in [0i8, -1] {
            let step = on_answer(&store, &config(), &chr, action);
            assert!(matches!(step, Step::Done { .. }), "{action}: {step:?}");
        }
        assert_eq!(store.mesos(chr.id).unwrap(), 5_000, "nothing was charged");
    }

    #[test]
    fn yes_without_the_quest_charges_the_fare_and_sails() {
        let (store, chr) = store_with_character();
        store.add_mesos(chr.id, 5_000).unwrap();
        let step = on_answer(&store, &config(), &chr, 1);
        let Step::Sail { fare, balance } = step else { panic!("expected Sail: {step:?}") };
        assert_eq!(fare, FARE_MESOS);
        assert_eq!(balance, 4_000);
        assert_eq!(store.mesos(chr.id).unwrap(), 4_000);
        assert!(script_replies(&Step::Sail { fare, balance }).is_empty(), "a paying passenger gets no box");
    }

    /// **The whole point of the feature, in the two replies it takes now.** Yes puts the
    /// waiver on its own box and moves nobody; dismissing the box sails for nothing.
    #[test]
    fn a_beginner_with_mais_final_training_is_told_of_the_waiver_and_then_sails_free() {
        let (store, chr) = store_with_character();
        store.add_mesos(chr.id, 5_000).unwrap();
        finish_mai(&store, &chr);
        assert!(waiver_applies(&store, &chr));

        let step = on_answer(&store, &config(), &chr, 1);
        let Step::Announce { text } = step.clone() else { panic!("expected Announce: {step:?}") };
        assert!(text.contains("Mai's training"), "{text}");
        assert_eq!(store.mesos(chr.id).unwrap(), 5_000, "nothing left the purse on Yes");
        let out = script_replies(&step);
        assert_eq!(out.len(), 1, "the waiver goes on screen, on its own");
        assert_eq!(out[0].body, net::script::npc_say(TEMPLATE, &text, false, false));

        let step = on_announce_dismissed(&store, &config(), &chr);
        let Step::Sail { fare, balance } = step.clone() else { panic!("expected Sail: {step:?}") };
        assert_eq!(fare, 0, "the crossing is free");
        assert_eq!(balance, 5_000, "and nothing left the purse");
        assert_eq!(store.mesos(chr.id).unwrap(), 5_000);
        assert!(script_replies(&step).is_empty(), "NO script beside the SetField - that is what tore the line down");
    }

    /// **Beginners only.** A Swordsman who finished Mai's training pays like anyone else,
    /// and there is no waiver box on the way.
    #[test]
    fn a_non_beginner_who_finished_mai_pays_the_fare() {
        for job in [100u16, 200, 300, 400] {
            let (store, mut chr) = store_with_character();
            store.add_mesos(chr.id, 5_000).unwrap();
            finish_mai(&store, &chr);
            assert!(passage_is_free(&store, chr.id), "the quest half holds");
            chr.job = job;
            assert!(!waiver_applies(&store, &chr), "job {job} gets no waiver");
            let step = on_answer(&store, &config(), &chr, 1);
            let Step::Sail { fare, balance } = step else { panic!("job {job}: expected Sail: {step:?}") };
            assert_eq!(fare, FARE_MESOS, "job {job}");
            assert_eq!(balance, 4_000, "job {job}");
        }
    }

    /// Only the FINAL training waives it. `1009` is the earlier step and must not.
    #[test]
    fn the_earlier_mai_quest_does_not_waive_the_fare() {
        let (store, chr) = store_with_character();
        store.add_mesos(chr.id, 5_000).unwrap();
        store.start_quest(chr.id, 1009).unwrap();
        store.complete_quest(chr.id, 1009).unwrap();
        assert!(!passage_is_free(&store, chr.id));
        assert!(!waiver_applies(&store, &chr));
        let Step::Sail { fare, .. } = on_answer(&store, &config(), &chr, 1) else {
            panic!("expected Sail")
        };
        assert_eq!(fare, FARE_MESOS);
    }

    /// Started but not finished is not finished.
    #[test]
    fn an_unfinished_quest_does_not_waive_the_fare() {
        let (store, chr) = store_with_character();
        store.add_mesos(chr.id, 5_000).unwrap();
        store.start_quest(chr.id, MAIS_FINAL_TRAINING).unwrap();
        assert!(!passage_is_free(&store, chr.id));
        assert!(!waiver_applies(&store, &chr));
    }

    #[test]
    fn a_player_who_cannot_pay_is_told_so_and_is_not_moved() {
        let (store, chr) = store_with_character();
        store.add_mesos(chr.id, 999).unwrap();
        let step = on_answer(&store, &config(), &chr, 1);
        let Step::Done { text, .. } = step else { panic!("expected a refusal: {step:?}") };
        assert!(text.contains("999"), "the balance is the store's, not an assumption: {text}");
        assert_eq!(store.mesos(chr.id).unwrap(), 999, "and nothing was taken");
    }

    /// Exactly the fare is enough - an off-by-one here charges nobody or everybody.
    #[test]
    fn exactly_the_fare_is_enough() {
        let (store, chr) = store_with_character();
        store.add_mesos(chr.id, i64::from(FARE_MESOS)).unwrap();
        let Step::Sail { balance, .. } = on_answer(&store, &config(), &chr, 1) else {
            panic!("expected Sail")
        };
        assert_eq!(balance, 0);
    }

    /// An empty field table must refuse rather than fail open - `map_exists` says yes to
    /// everything when it has nothing to check against. On the Yes and on the dismissal both.
    #[test]
    fn an_empty_field_table_refuses_and_charges_nothing() {
        let (store, chr) = store_with_character();
        store.add_mesos(chr.id, 5_000).unwrap();
        let step = on_answer(&store, &Config::default(), &chr, 1);
        assert!(matches!(step, Step::Done { .. }), "{step:?}");
        finish_mai(&store, &chr);
        let step = on_announce_dismissed(&store, &Config::default(), &chr);
        assert!(matches!(step, Step::Done { .. }), "{step:?}");
        assert_eq!(store.mesos(chr.id).unwrap(), 5_000);
    }

    /// The paths must not be mistaken for another module's conversation, or for each other.
    #[test]
    fn the_path_collides_with_nothing() {
        assert_ne!(ASK_PATH, ANNOUNCE_PATH);
        for p in [ASK_PATH, ANNOUNCE_PATH] {
            assert!(!crate::taxi::is_taxi_path(p));
            assert_ne!(p, crate::taxi::MENU_PATH);
            assert_ne!(p, crate::jobs::ASK_PATH, "the instructor's yes/no is its own path");
            assert!(!crate::jobguide::is_menu_path(p));
            assert!(!crate::scrollnpc::is_scroll_path(p));
            assert!(!p.starts_with(crate::taxi::PATH_PREFIX));
            assert!(p.contains('.'), "a quest path is \"\", \"0\" or \"0.yes\"");
        }
        assert!(!crate::taxi::is_taxi_path(crate::jobs::ASK_PATH));
        assert!(!crate::jobguide::is_menu_path(crate::jobs::ASK_PATH));
        assert!(!crate::scrollnpc::is_scroll_path(crate::jobs::ASK_PATH));
    }
}
