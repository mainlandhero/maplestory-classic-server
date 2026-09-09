//! `!scroll` - the Maple Administrator's dialogue for the three scrolls in [`crate::scrolls`].
//!
//! The owner, 2026-09-09: *"`!scroll` will be a new command that functions very similar to `!tool`,
//! shows up with NPC dialogue from MapleStory Administrator which implements functionality that
//! does not exist in the game yet."* And: *"The dialogue should also ask players to confirm
//! their choice, and select which equipment they have currently equipped to use the item scroll
//! on."*
//!
//! `crate::scrolls` owns the rules and knows nothing about dialogue; this owns the wording and
//! the three steps and knows nothing about the rules. The session joins them.
//!
//! # Three steps, and the state rides in the path
//!
//! ```text
//!   scroll.pick                     which scroll?      (only ones the player holds)
//!   scroll.equip:<scrollId>         which worn item?
//!   scroll.confirm:<scrollId>:<eq>  are you sure?      yes/no
//! ```
//!
//! **The path carries the choice because `Conversation` has nowhere else to put it**, and the
//! alternative - a field per feature on a struct every dialogue shares - is how that struct
//! grows a tail nobody can reason about. It is also what makes each step's handler able to
//! refuse a reply that is not its own: `research/script-reply.md` records that a type-6 body
//! carries no speaker, so **the path is the only thing that says who asked**.
//!
//! Every prefix here begins `scroll.`, which no other dialogue uses, and a test says so.

use crate::scrolls::{Refusal, Scroll};

/// What the player types.
pub const COMMAND: &str = "scroll";

/// The same string with its `!`, for a line a person reads.
pub const COMMAND_TYPED: &str = "!scroll";

/// Step 1: pick a scroll.
pub const PICK_PATH: &str = "scroll.pick";

/// Step 2: pick a worn item. Followed by `:<scrollItemId>`.
pub const EQUIP_PATH_PREFIX: &str = "scroll.equip:";

/// Step 3: confirm. Followed by `:<scrollItemId>:<equipSlot>`.
pub const CONFIRM_PATH_PREFIX: &str = "scroll.confirm:";

/// Does this conversation path belong to `!scroll`?
///
/// Checked before anything is decoded, exactly as the taxi's, the instructor's and the
/// Administrator's favour menu do.
pub fn is_scroll_path(path: &str) -> bool {
    path == PICK_PATH
        || path.starts_with(EQUIP_PATH_PREFIX)
        || path.starts_with(CONFIRM_PATH_PREFIX)
}

/// `scroll.equip:4031065` -> the scroll.
pub fn scroll_from_equip_path(path: &str) -> Option<Scroll> {
    Scroll::from_item_id(path.strip_prefix(EQUIP_PATH_PREFIX)?.parse().ok()?)
}

/// `scroll.confirm:4031065:5` -> `(scroll, equip slot)`.
pub fn choice_from_confirm_path(path: &str) -> Option<(Scroll, u8)> {
    let rest = path.strip_prefix(CONFIRM_PATH_PREFIX)?;
    let (id, slot) = rest.split_once(':')?;
    Some((Scroll::from_item_id(id.parse().ok()?)?, slot.parse().ok()?))
}

pub fn equip_path(scroll: Scroll) -> String {
    format!("{EQUIP_PATH_PREFIX}{}", scroll.item_id())
}

pub fn confirm_path(scroll: Scroll, equip_slot: u8) -> String {
    format!("{CONFIRM_PATH_PREFIX}{}:{equip_slot}", scroll.item_id())
}

/// What each scroll does, in the player's words. Shown on the pick menu and the confirm box,
/// because a player choosing between three scrolls they have never seen needs to be told.
pub fn describe(scroll: Scroll) -> &'static str {
    match scroll {
        Scroll::Innocence => {
            "Returns the item to its original state and gives back every enhancement slot."
        }
        Scroll::Chaos => {
            "Uses one enhancement slot and randomly raises or lowers one of the item's stats \
             by up to 5. The slot is used whether it works or not."
        }
        Scroll::CleanSlate => {
            "Gives back one enhancement slot that a failed scroll used up."
        }
    }
}

pub fn header() -> String {
    "I have three scrolls here that you will not find anywhere else in Maple World. \
     Which one shall I use?"
        .to_string()
}

/// The pick menu, over the scrolls the player is actually carrying.
///
/// `held` is `(scroll, count)`. **Only scrolls in the bag are listed**: offering one the player
/// does not have would be a menu entry whose only outcome is a refusal, and a menu that can
/// only disappoint is worse than a shorter menu.
pub fn pick_menu(held: &[(Scroll, u16)]) -> String {
    let mut s = header();
    s.push_str("\r\n");
    for (i, (scroll, count)) in held.iter().enumerate() {
        s.push_str(&format!(
            "\r\n#L{i}##b{}#k (x{count})#l\r\n{}",
            scroll.name(),
            describe(*scroll)
        ));
    }
    s
}

pub fn nothing_to_use() -> String {
    "You are not carrying any of my scrolls. They turn up in the wider world, rarely - \
     keep hunting and one will find you."
        .to_string()
}

pub fn nothing_equipped() -> String {
    "You are not wearing anything I could use that on. Put on the item first, then come back."
        .to_string()
}

/// The worn-item menu. `worn` is `(equip slot, item name, remaining slots, failed slots)`.
pub fn equip_menu(scroll: Scroll, worn: &[(u8, String, u8, u8)]) -> String {
    let mut s = format!(
        "#b{}#k. {}\r\n\r\nWhich of the things you are wearing shall I use it on?",
        scroll.name(),
        describe(scroll)
    );
    for (i, (_, name, remaining, failed)) in worn.iter().enumerate() {
        s.push_str(&format!(
            "\r\n#L{i}##b{name}#k - {remaining} enhancement slot(s) left, {failed} failed#l"
        ));
    }
    s
}

/// The confirm box. Deliberately restates BOTH the scroll and the item: the owner asked for a
/// confirmation, and a confirmation that does not name what it is confirming is a button.
pub fn confirm(scroll: Scroll, item_name: &str, guaranteed: bool) -> String {
    let odds = if scroll.always_succeeds() {
        "This one always works.".to_string()
    } else if guaranteed {
        "Your first use of this scroll today is guaranteed to work.".to_string()
    } else {
        format!(
            "You have already used one today, so this one has a {}% chance.",
            crate::scrolls::ROLLED_SUCCESS_PCT
        )
    };
    format!(
        "Use #b{}#k on your #b{item_name}#k?\r\n\r\n{}\r\n{odds}\r\n\r\n\
         The scroll is used up either way.",
        scroll.name(),
        describe(scroll)
    )
}

pub fn cancelled() -> String {
    "As you like. Come back when you have made up your mind.".to_string()
}

pub fn refused(refusal: Refusal) -> String {
    format!("{} Nothing has been used up.", refusal.line())
}

/// The outcome line. `stat_change` is the `(name, delta)` Chaos produced, if any.
pub fn outcome(
    scroll: Scroll,
    item_name: &str,
    succeeded: bool,
    stat_change: Option<(&str, i32)>,
    remaining: u8,
) -> String {
    let head = match (scroll, succeeded) {
        (Scroll::Innocence, _) => {
            format!("Your #b{item_name}#k is as it was the day it was made.")
        }
        (Scroll::Chaos, true) => match stat_change {
            Some((stat, d)) if d > 0 => {
                format!("It worked. #b{item_name}#k gained {d} {stat}.")
            }
            Some((stat, d)) if d < 0 => {
                format!("It worked, after a fashion. #b{item_name}#k lost {} {stat}.", -d)
            }
            // A roll of zero, or an item with no stats to move. It is still a success and the
            // slot is still gone, so it must not read as a failure.
            _ => format!("It worked, but nothing about #b{item_name}#k changed."),
        },
        (Scroll::Chaos, false) => {
            format!("It failed, and the slot is gone. #b{item_name}#k is unchanged otherwise.")
        }
        (Scroll::CleanSlate, true) => {
            format!("One of the slots you lost is back. #b{item_name}#k can be scrolled again.")
        }
        (Scroll::CleanSlate, false) => {
            format!("It failed. #b{item_name}#k keeps the slots it has.")
        }
    };
    format!("{head}\r\n\r\nEnhancement slots remaining: #b{remaining}#k.")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The path prefix cannot be confused with any other dialogue's.** A type-6 reply carries
    /// no speaker, so if two features shared a prefix each would answer the other's menu.
    #[test]
    fn the_scroll_paths_are_disjoint_from_every_other_dialogue() {
        for p in [PICK_PATH, &equip_path(Scroll::Chaos), &confirm_path(Scroll::Chaos, 5)] {
            assert!(is_scroll_path(p), "{p}");
            assert!(!crate::dailyperks::is_menu_path(p), "{p}");
            assert!(!crate::secondjob::is_menu_path(p), "{p}");
        }
        // And the control: this predicate is not simply true.
        assert!(!is_scroll_path(crate::dailyperks::MENU_PATH));
        assert!(!is_scroll_path("shanks.ask"));
        assert!(!is_scroll_path(""));
    }

    /// The state really does survive the round trip through the path.
    #[test]
    fn the_choice_round_trips_through_the_path() {
        for scroll in [Scroll::Innocence, Scroll::Chaos, Scroll::CleanSlate] {
            assert_eq!(scroll_from_equip_path(&equip_path(scroll)), Some(scroll));
            for slot in [1u8, 5, 11, 255] {
                assert_eq!(
                    choice_from_confirm_path(&confirm_path(scroll, slot)),
                    Some((scroll, slot))
                );
            }
        }
        // Garbage decodes to nothing rather than to a default.
        assert_eq!(scroll_from_equip_path("scroll.equip:999"), None);
        assert_eq!(choice_from_confirm_path("scroll.confirm:4031065"), None);
        assert_eq!(choice_from_confirm_path("scroll.confirm:4031065:x"), None);
    }

    /// A menu entry per scroll held, numbered from zero, matching what `parse_menu_reply`
    /// returns - an off-by-one here uses the wrong scroll, which is unrecoverable for a player.
    #[test]
    fn the_pick_menu_numbers_from_zero_and_lists_only_what_is_held() {
        let text = pick_menu(&[(Scroll::Chaos, 3), (Scroll::CleanSlate, 1)]);
        assert!(text.contains("#L0#"), "{text}");
        assert!(text.contains("#L1#"), "{text}");
        assert!(!text.contains("#L2#"), "only two are held: {text}");
        assert!(text.contains("Scroll of Secrets"));
        assert!(text.contains("(x3)"));
        assert!(!text.contains("Event Trophy"), "not held, so not offered");
    }

    /// The confirm box must name the scroll AND the item, and must state the odds - a
    /// confirmation that does not say what it is confirming is just a button.
    #[test]
    fn the_confirm_box_names_both_and_states_the_odds() {
        let g = confirm(Scroll::Chaos, "Maple Sword", true);
        assert!(g.contains("Scroll of Secrets") && g.contains("Maple Sword"), "{g}");
        assert!(g.contains("guaranteed"), "{g}");
        let r = confirm(Scroll::Chaos, "Maple Sword", false);
        assert!(r.contains("60%"), "{r}");
        // Innocence never rolls, so it must not offer odds at all.
        let i = confirm(Scroll::Innocence, "Maple Sword", false);
        assert!(i.contains("always works"), "{i}");
        assert!(!i.contains('%'), "{i}");
    }

    /// A Chaos that rolled zero is a SUCCESS that changed nothing. The player still paid a
    /// slot, so wording it as a failure would be a lie about what it cost them.
    #[test]
    fn a_zero_roll_chaos_does_not_read_as_a_failure() {
        let t = outcome(Scroll::Chaos, "Maple Sword", true, None, 4);
        assert!(t.contains("worked"), "{t}");
        assert!(!t.to_lowercase().contains("failed"), "{t}");
        let f = outcome(Scroll::Chaos, "Maple Sword", false, None, 4);
        assert!(f.contains("failed"), "{f}");
    }

    /// Every outcome states the slots left, because that is the number the player is deciding
    /// on next and the client's own tooltip is the only other place it appears.
    #[test]
    fn every_outcome_reports_the_remaining_slots() {
        for scroll in [Scroll::Innocence, Scroll::Chaos, Scroll::CleanSlate] {
            for ok in [true, false] {
                let t = outcome(scroll, "Maple Sword", ok, Some(("STR", 2)), 3);
                assert!(t.contains("Enhancement slots remaining: #b3#k"), "{t}");
            }
        }
    }
}
