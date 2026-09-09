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
//! # This module is the decision and the words. It knows nothing about `Session`
//!
//! Same shape as `crate::jobguide` and `crate::taxi`: every branch below is a unit test
//! rather than a client run, because a run costs the owner a manual elevated launch.
//!
//! # Nothing here authenticates
//!
//! The channel socket carries no credentials. Passage is sold to whoever holds it.

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
    /// **Every guard passed.** Charge `fare` (which may be zero) and warp to
    /// [`DESTINATION_MAP`]. `text` is the extra line for a waived fare, and `None` for a
    /// paid crossing - a player who paid gets the meso line and the new map, which is
    /// already two pieces of feedback.
    ///
    /// **Nothing that assumed a refusal may precede this.** It is the only arm that carries a
    /// destination, so a refusal cannot warp anybody by accident.
    Sail { text: Option<String>, fare: u32, balance: u32 },
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

/// The line Shanks adds when the fare is waived.
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
/// 3. **The money, last** - charging for a crossing we are about to refuse is the one
///    direction that takes something and gives nothing.
pub fn on_answer(store: &Store, config: &Config, character_id: u32, action: i8) -> Step {
    if action != 1 {
        return Step::Done {
            text: "Suit yourself. The tide'll wait a while yet.".to_string(),
            why: format!(
                "shanks: character {character_id} answered {action} (not Yes). NO FARE TAKEN, \
                 NO TELEPORT"
            ),
        };
    }
    if config.fields.is_empty() {
        return Step::Done {
            text: "The harbour's fogged in. Try me again later.".to_string(),
            why: "shanks: the field table is EMPTY, so the destination could not be checked \
                  and a bad id would kill the client. Regenerate gm-handbook/fields.txt with \
                  tools/dump_portals.py, or restart the server. NO FARE TAKEN"
                .to_string(),
        };
    }
    if !config.map_exists(DESTINATION_MAP) {
        return Step::Done {
            text: "The harbour's fogged in. Try me again later.".to_string(),
            why: format!(
                "shanks: map {DESTINATION_MAP} has no field image in this client, so sailing \
                 would strand the character. NO FARE TAKEN"
            ),
        };
    }

    let free = passage_is_free(store, character_id);
    let fare = if free { 0 } else { FARE_MESOS };
    // `add_mesos` reads and writes in ONE transaction and refuses to go below zero, so two
    // fast clicks cannot both be charged against the same balance. `add_mesos(0)` is a no-op
    // that still reports the balance, so the free crossing takes the same path as the paid
    // one and cannot drift from it.
    match store.add_mesos(character_id, -i64::from(fare)) {
        Ok(balance) => Step::Sail {
            text: free.then(free_passage_line),
            fare,
            balance,
        },
        Err(store::StoreError::NotEnoughMesos { have, .. }) => Step::Done {
            text: format!(
                "Passage is #b{FARE_MESOS} mesos#k and you're carrying #b{have}#k. \
                 No ticket, no crossing."
            ),
            why: format!(
                "shanks: character {character_id} has {have} mesos and the fare is \
                 {FARE_MESOS}. NO FARE TAKEN, NO TELEPORT"
            ),
        },
        Err(e) => Step::Done {
            text: "The harbour's fogged in. Try me again later.".to_string(),
            why: format!(
                "shanks: could not charge character {character_id}: {e}. NO FARE TAKEN, \
                 NO TELEPORT"
            ),
        },
    }
}

/// The `0x055B` a step puts on screen.
///
/// # `Step::Sail`'s free line goes out, and a paid crossing says nothing here
///
/// **A script message must never travel with, or just before, a `SetField`** - field entry
/// runs the script-manager reset, so the dialog is built and torn down silently. `taxi`
/// records this costing a run. The free line is the exception the owner asked for, and it is safe
/// because the caller sends it and *then* warps, in that order, exactly as the arrival notice
/// is ordered.
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
        Step::Sail { text: Some(text), .. } => vec![crate::Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(TEMPLATE, text, false, false),
            what: format!(
                "ScriptMessage Say from Shanks ({TEMPLATE}): FREE passage - Mai's Final \
                 Training ({MAIS_FINAL_TRAINING}) is complete, so no fare was taken"
            ),
        }],
        Step::Sail { text: None, .. } => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store_with_character() -> (Store, u32) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Sailor".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        (store, id)
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
        store.add_mesos(chr, 5_000).unwrap();
        for action in [0i8, -1] {
            let step = on_answer(&store, &config(), chr, action);
            assert!(matches!(step, Step::Done { .. }), "{action}: {step:?}");
        }
        assert_eq!(store.mesos(chr).unwrap(), 5_000, "nothing was charged");
    }

    #[test]
    fn yes_without_the_quest_charges_the_fare_and_sails() {
        let (store, chr) = store_with_character();
        store.add_mesos(chr, 5_000).unwrap();
        let step = on_answer(&store, &config(), chr, 1);
        let Step::Sail { text, fare, balance } = step else { panic!("expected Sail: {step:?}") };
        assert_eq!(fare, FARE_MESOS);
        assert_eq!(balance, 4_000);
        assert_eq!(text, None, "a paying passenger gets no extra line");
        assert_eq!(store.mesos(chr).unwrap(), 4_000);
        assert!(script_replies(&Step::Sail { text: None, fare, balance }).is_empty());
    }

    /// The whole point of the feature.
    #[test]
    fn yes_with_mais_final_training_complete_is_free_and_says_so() {
        let (store, chr) = store_with_character();
        store.add_mesos(chr, 5_000).unwrap();
        store.start_quest(chr, MAIS_FINAL_TRAINING).unwrap();
        store.complete_quest(chr, MAIS_FINAL_TRAINING).unwrap();

        let step = on_answer(&store, &config(), chr, 1);
        let Step::Sail { text, fare, balance } = step.clone() else {
            panic!("expected Sail: {step:?}")
        };
        assert_eq!(fare, 0, "the crossing is free");
        assert_eq!(balance, 5_000, "and nothing left the purse");
        assert_eq!(store.mesos(chr).unwrap(), 5_000);
        let text = text.expect("the free crossing says something extra");
        assert!(text.contains("Mai's training"), "{text}");
        assert_eq!(script_replies(&step).len(), 1, "and it goes on screen");
    }

    /// Only the FINAL training waives it. `1009` is the earlier step and must not.
    #[test]
    fn the_earlier_mai_quest_does_not_waive_the_fare() {
        let (store, chr) = store_with_character();
        store.add_mesos(chr, 5_000).unwrap();
        store.start_quest(chr, 1009).unwrap();
        store.complete_quest(chr, 1009).unwrap();
        assert!(!passage_is_free(&store, chr));
        let Step::Sail { fare, .. } = on_answer(&store, &config(), chr, 1) else {
            panic!("expected Sail")
        };
        assert_eq!(fare, FARE_MESOS);
    }

    /// Started but not finished is not finished.
    #[test]
    fn an_unfinished_quest_does_not_waive_the_fare() {
        let (store, chr) = store_with_character();
        store.add_mesos(chr, 5_000).unwrap();
        store.start_quest(chr, MAIS_FINAL_TRAINING).unwrap();
        assert!(!passage_is_free(&store, chr));
    }

    #[test]
    fn a_player_who_cannot_pay_is_told_so_and_is_not_moved() {
        let (store, chr) = store_with_character();
        store.add_mesos(chr, 999).unwrap();
        let step = on_answer(&store, &config(), chr, 1);
        let Step::Done { text, .. } = step else { panic!("expected a refusal: {step:?}") };
        assert!(text.contains("999"), "the balance is the store's, not an assumption: {text}");
        assert_eq!(store.mesos(chr).unwrap(), 999, "and nothing was taken");
    }

    /// Exactly the fare is enough - an off-by-one here charges nobody or everybody.
    #[test]
    fn exactly_the_fare_is_enough() {
        let (store, chr) = store_with_character();
        store.add_mesos(chr, i64::from(FARE_MESOS)).unwrap();
        let Step::Sail { balance, .. } = on_answer(&store, &config(), chr, 1) else {
            panic!("expected Sail")
        };
        assert_eq!(balance, 0);
    }

    /// An empty field table must refuse rather than fail open - `map_exists` says yes to
    /// everything when it has nothing to check against.
    #[test]
    fn an_empty_field_table_refuses_and_charges_nothing() {
        let (store, chr) = store_with_character();
        store.add_mesos(chr, 5_000).unwrap();
        let step = on_answer(&store, &Config::default(), chr, 1);
        assert!(matches!(step, Step::Done { .. }), "{step:?}");
        assert_eq!(store.mesos(chr).unwrap(), 5_000);
    }

    /// The path must not be mistaken for another module's conversation.
    #[test]
    fn the_path_collides_with_nothing() {
        assert!(!crate::taxi::is_taxi_path(ASK_PATH));
        assert_ne!(ASK_PATH, crate::taxi::MENU_PATH);
        assert!(!ASK_PATH.starts_with(crate::taxi::PATH_PREFIX));
        assert!(ASK_PATH.contains('.'), "a quest path is \"\", \"0\" or \"0.yes\"");
    }
}
