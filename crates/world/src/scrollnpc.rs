//! `!scroll` - the Maple Administrator's dialogue for the two scrolls in [`crate::scrolls`].
//!
//! The owner, 2026-09-09: *"`!scroll` will be a new command that functions very similar to `!tool`,
//! shows up with NPC dialogue from MapleStory Administrator which implements functionality that
//! does not exist in the game yet."* And: *"The dialogue should also ask players to confirm
//! their choice, and select which equipment they have currently equipped to use the item scroll
//! on."*
//!
//! `crate::scrolls` owns the rules and knows nothing about dialogue; this owns the wording and
//! the steps and knows nothing about the rules. The session joins them.
//!
//! # The steps, and the state rides in the path
//!
//! ```text
//!   scroll.pick                          which item?  (only ones the player holds)
//!
//!   Scroll of Secrets
//!     scroll.mode                        Chaos, Innocence or Clean Slate?
//!     scroll.equip:secrets:<modeKey>     which worn item?
//!     scroll.confirm:secrets:<key>:<eq>  are you sure?   yes/no
//!
//!   Treasure Scroll
//!     scroll.equip:treasure              which worn item?   (asked FIRST - see below)
//!     scroll.real:<eq>                   which of your scrolls shall I guarantee?
//!     scroll.confirm:treasure:<eq>:<id>  are you sure?   yes/no
//! ```
//!
//! **The Treasure Scroll asks for the equip before the scroll**, which is the opposite order
//! to the other branch and is not an accident: the list of real scrolls it can offer is
//! filtered to the ones that *fit* the target, so the target has to be known first. Offering
//! all 208 and refusing 200 of them afterwards would be a menu that mostly disappoints.
//!
//! **The path carries the choice because `Conversation` has nowhere else to put it**, and the
//! alternative - a field per feature on a struct every dialogue shares - is how that struct
//! grows a tail nobody can reason about. It is also what makes each step's handler able to
//! refuse a reply that is not its own: `research/script-reply.md` records that a type-6 body
//! carries no speaker, so **the path is the only thing that says who asked**.
//!
//! Every prefix here begins `scroll.`, which no other dialogue uses, and a test says so.
//!
//! # The hover question, and what is actually known
//!
//! The owner, 2026-09-09: *"Pretty sure you can hide the effects on a hover text. Please
//! investigate that."* Three things were measured and one was not found, and the difference
//! matters:
//!
//! **Measured.** Every string in this client's `QuestData` was enumerated - 322 images, 3969
//! strings - and every `#<char>` code in them tallied. The codes this client's own content
//! uses are `#b #k #r #e #n #p #m #t #o #i #c #h #a #s #q #L #l`. **`#v` and `#z` appear zero
//! times**, and those are the two codes other MapleStory versions use for an icon that
//! carries a tooltip.
//!
//! **Measured.** `Etc/ScriptInfo.img` - the image `FUN_141e3c5d0` reaches beside the `#L%d#`
//! formatting, per `research/npc-click.md` §2.1 - is **13 bytes, an empty image** in this
//! client. `String.wz/ToolTipHelp.img` is real and populated but is keyed by fixed UI element
//! name (`Game/Button/Shop`), not by anything script text could address.
//!
//! **Measured, and it is the part that changes the design.** Even if an `#i` icon in a script
//! window turns out to be hoverable, the tooltip it would show is the *item's own*, and these
//! items are repurposed. `String.wz/Etc.img` gives them:
//!
//! ```text
//!   4031065  Scroll of Secrets "A mystical scroll written in a lost, ancient language."
//!   4031066  Treasure Scroll   "A map that shows where the jewels are hidden away."
//! ```
//!
//! Neither says what the scroll does here. A hover would have to be told our text, and there
//! is no code that takes text.
//!
//! **Not found, which is not the same as not there.** Two whole-`.text` scans for the markup
//! parser - one over byte-register comparisons, one widened to 16- and 32-bit registers
//! because the text is UTF-16 - ranked every 0x400 window by how many distinct ASCII letters
//! it tests. The top of both rankings is `printf`'s conversion specifiers (`cdiopsux`), not a
//! markup set. **Neither scan has a positive control**, and both share one blind spot: a
//! `switch` on the character compiles to a jump table, which shows up as at most two
//! comparisons. So "no hover code" is *not* established, only "not found by an instrument
//! that cannot see the most likely shape". `tools/find_switch_tables.py` is the instrument
//! that could, and it has not been pointed at this.
//!
//! What this module does in the meantime: every menu is one short line per row, and the
//! description of a choice is the first thing the *next* screen says. That is the nearest
//! thing to "on demand" the client is known to support.

use crate::scrolls::{Refusal, Scroll, SecretsMode};

/// What the player types.
pub const COMMAND: &str = "scroll";

/// The same string with its `!`, for a line a person reads.
pub const COMMAND_TYPED: &str = "!scroll";

/// Step 1: pick one of the two items.
pub const PICK_PATH: &str = "scroll.pick";

/// Step 2, Scroll of Secrets only: which of the three it acts as.
pub const MODE_PATH: &str = "scroll.mode";

/// Which worn item. Followed by `secrets:<modeKey>` or `treasure`.
pub const EQUIP_PATH_PREFIX: &str = "scroll.equip:";

/// Treasure Scroll only: which real scroll to guarantee. Followed by `:<equipSlot>`.
pub const REAL_PATH_PREFIX: &str = "scroll.real:";

/// The yes/no. Followed by `secrets:<key>:<slot>` or `treasure:<slot>:<realScrollId>`.
pub const CONFIRM_PATH_PREFIX: &str = "scroll.confirm:";

/// The `secrets` / `treasure` discriminator inside a path.
const SECRETS_TAG: &str = "secrets";
const TREASURE_TAG: &str = "treasure";

/// Does this conversation path belong to `!scroll`?
///
/// Checked before anything is decoded, exactly as the taxi's, the instructor's and the
/// Administrator's favour menu do.
pub fn is_scroll_path(path: &str) -> bool {
    path == PICK_PATH
        || path == MODE_PATH
        || path.starts_with(EQUIP_PATH_PREFIX)
        || path.starts_with(REAL_PATH_PREFIX)
        || path.starts_with(CONFIRM_PATH_PREFIX)
}

/// What the equip menu is being asked on behalf of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Branch {
    Secrets(SecretsMode),
    Treasure,
}

pub fn equip_path(branch: Branch) -> String {
    match branch {
        Branch::Secrets(mode) => format!("{EQUIP_PATH_PREFIX}{SECRETS_TAG}:{}", mode.key()),
        Branch::Treasure => format!("{EQUIP_PATH_PREFIX}{TREASURE_TAG}"),
    }
}

/// `scroll.equip:secrets:chaos` -> `Branch::Secrets(Chaos)`; `scroll.equip:treasure` ->
/// `Branch::Treasure`. Anything else is `None` rather than a default.
pub fn branch_from_equip_path(path: &str) -> Option<Branch> {
    let rest = path.strip_prefix(EQUIP_PATH_PREFIX)?;
    if rest == TREASURE_TAG {
        return Some(Branch::Treasure);
    }
    let key = rest.strip_prefix(SECRETS_TAG)?.strip_prefix(':')?;
    Some(Branch::Secrets(SecretsMode::from_key(key)?))
}

pub fn real_path(equip_slot: u8) -> String {
    format!("{REAL_PATH_PREFIX}{equip_slot}")
}

pub fn equip_slot_from_real_path(path: &str) -> Option<u8> {
    path.strip_prefix(REAL_PATH_PREFIX)?.parse().ok()
}

/// The confirmed action, which is everything the apply step needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confirmed {
    Secrets { mode: SecretsMode, equip_slot: u8 },
    Treasure { equip_slot: u8, real_scroll: u32 },
}

impl Confirmed {
    pub fn equip_slot(self) -> u8 {
        match self {
            Confirmed::Secrets { equip_slot, .. } => equip_slot,
            Confirmed::Treasure { equip_slot, .. } => equip_slot,
        }
    }

    /// Which of the two repurposed items this consumes.
    pub fn item(self) -> Scroll {
        match self {
            Confirmed::Secrets { .. } => Scroll::Secrets,
            Confirmed::Treasure { .. } => Scroll::Treasure,
        }
    }
}

pub fn confirm_path(action: Confirmed) -> String {
    match action {
        Confirmed::Secrets { mode, equip_slot } => {
            format!("{CONFIRM_PATH_PREFIX}{SECRETS_TAG}:{}:{equip_slot}", mode.key())
        }
        Confirmed::Treasure { equip_slot, real_scroll } => {
            format!("{CONFIRM_PATH_PREFIX}{TREASURE_TAG}:{equip_slot}:{real_scroll}")
        }
    }
}

pub fn confirmed_from_path(path: &str) -> Option<Confirmed> {
    let rest = path.strip_prefix(CONFIRM_PATH_PREFIX)?;
    let (tag, rest) = rest.split_once(':')?;
    let (a, b) = rest.split_once(':')?;
    match tag {
        SECRETS_TAG => {
            Some(Confirmed::Secrets { mode: SecretsMode::from_key(a)?, equip_slot: b.parse().ok()? })
        }
        TREASURE_TAG => Some(Confirmed::Treasure {
            equip_slot: a.parse().ok()?,
            real_scroll: b.parse().ok()?,
        }),
        _ => None,
    }
}

/// What each item is, in the player's words.
pub fn describe_item(scroll: Scroll) -> &'static str {
    match scroll {
        Scroll::Secrets => {
            "Three scrolls in one. You choose which of them it becomes when you use it."
        }
        Scroll::Treasure => {
            "Makes one of the scrolls you are carrying succeed outright, whatever its odds. \
             It still uses one enhancement slot."
        }
    }
}

/// What each mode of the Scroll of Secrets does. Shown on the mode menu's next screen and on
/// the confirm box, because a player choosing between three effects needs to be told.
pub fn describe(mode: SecretsMode) -> &'static str {
    match mode {
        SecretsMode::Chaos => {
            "Uses one enhancement slot and randomly raises or lowers one of the item's stats \
             by up to 5. The slot is used whether it works or not."
        }
        SecretsMode::Innocence => {
            "Returns the item to its original state and gives back every enhancement slot. \
             Nothing you rolled onto it is kept."
        }
        SecretsMode::CleanSlate => {
            "Gives back one enhancement slot that a failed scroll used up."
        }
    }
}

pub fn header() -> String {
    "I have scrolls here that you will not find anywhere else in Maple World. \
     Which one shall I use?"
        .to_string()
}

/// # The shape of a menu is the client's, and it was measured rather than guessed
///
/// The owner, 2026-09-09, with a screenshot: *"the dialogue shown clips"* - the description ran
/// straight over the top of the selectable line above it.
///
/// The first version wrote a CRLF and then the description after each `#l`, which is plain
/// text following a link close. Every `#L` menu in this client's own `QuestData` was then
/// enumerated - 33 strings - and:
///
/// ```text
///   nothing at all follows the final #l          32 of 33   (the 33rd has a bare #k)
///   the separator immediately before a #L is     81 of 81   a bare newline, never a CRLF
///   one #l closes the whole menu                 30 of 33   (3 close each link)
/// ```
///
/// So text after `#l` is **unattested in 33 out of 33 cases**, and that is the clip. Every
/// menu below is therefore the last thing in its string, one row per line, one `#l` at the end.
const MENU_SEPARATOR: &str = "\n";

/// Assemble a menu the way this client's own content does: a lead paragraph, a blank line,
/// then the rows and nothing after them.
///
/// **One function so there is one shape.** The clip was in `pick_menu` and would have been in
/// each of the other three the moment they grew a description, because they were four
/// separate `format!`s that merely happened to agree.
fn menu(lead: &str, rows: &[String]) -> String {
    format!("{lead}\n\n{}#l", rows.join(MENU_SEPARATOR))
}

/// One selectable row: `#L<n>#` then the item's icon, its name, and whatever trails it.
///
/// # The icon
///
/// The owner: *"I also want you to display the item icon as well as part of that line selection."*
/// `#i<itemId>#` is the icon, and it is attested **164 times** in this client's own quest text
/// - `"The mirror looks like this: #i4031000#."* - so the code itself is not in doubt.
///
/// **What is not attested is an icon INSIDE a `#L` region**: zero of the 33 menu strings put
/// one there, so whether the list widget makes the row tall enough for a 32px canvas is
/// **[I]**, and it is the same class of question as the clip above. One screen settles it.
fn row(index: usize, item_id: u32, name: &str, trailer: &str) -> String {
    format!("#L{index}##i{item_id}# #b{name}#k{trailer}")
}

/// Step 1: which of the two items. `held` is `(scroll, count)`.
///
/// **Only items in the bag are listed**: offering one the player does not have would be a menu
/// entry whose only outcome is a refusal, and a menu that can only disappoint is worse than a
/// shorter menu.
pub fn pick_menu(held: &[(Scroll, u16)]) -> String {
    let rows: Vec<String> = held
        .iter()
        .enumerate()
        .map(|(i, (scroll, count))| {
            row(i, scroll.item_id(), scroll.name(), &format!(" (x{count})"))
        })
        .collect();
    menu(&header(), &rows)
}

/// Step 2 for the Scroll of Secrets: which of the three it becomes.
///
/// Each row says its own success rate, because that is the difference between the three that
/// the player is actually choosing on, and there is no later screen that would show all three
/// side by side.
pub fn mode_menu(guaranteed: [bool; 3]) -> String {
    let rows: Vec<String> = SecretsMode::ALL
        .iter()
        .zip(guaranteed)
        .enumerate()
        .map(|(i, (mode, free))| {
            let odds = if mode.always_succeeds() {
                "always works".to_string()
            } else if free {
                "guaranteed today".to_string()
            } else {
                format!("{}%", crate::scrolls::ROLLED_SUCCESS_PCT)
            };
            row(i, crate::scrolls::SCROLL_OF_SECRETS, mode.name(), &format!(" - {odds}"))
        })
        .collect();
    menu(
        "A #bScroll of Secrets#k is whichever of these you need it to be. Which shall it be?",
        &rows,
    )
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

/// The worn-item menu. `worn` is `(equip slot, item id, item name, remaining slots, failed)`.
///
/// **This is where the chosen effect is described.** It is the lead paragraph, plain text
/// above the menu, which is the one position for non-link text that this client's own content
/// attests. The menu before it is a list of names, and the moment one is chosen this says what
/// it does - so nothing is lost by keeping the lists short.
pub fn equip_menu(branch: Branch, worn: &[(u8, u32, String, u8, u8)]) -> String {
    let lead = match branch {
        Branch::Secrets(mode) => {
            format!("#b{}#k. {}\n\nWhich of the things you are wearing shall I use it on?",
                    mode.name(), describe(mode))
        }
        Branch::Treasure => {
            "#bTreasure Scroll#k. I will make one of your own scrolls succeed outright.\n\n\
             Which of the things you are wearing is it for?"
                .to_string()
        }
    };
    let rows: Vec<String> = worn
        .iter()
        .enumerate()
        .map(|(i, (_, item_id, name, remaining, failed))| {
            row(i, *item_id, name, &format!(" - {remaining} slot(s) left, {failed} failed"))
        })
        .collect();
    menu(&lead, &rows)
}

/// Nothing in the bag fits the chosen equip. Names the item, because "no scroll fits" without
/// saying what it had to fit is a refusal the player cannot act on.
pub fn no_scroll_fits(item_name: &str) -> String {
    format!(
        "None of the scrolls you are carrying were made for a #b{item_name}#k. \
         Bring me one that was, and I will make it work."
    )
}

/// Treasure Scroll step 3: which of the player's real scrolls to guarantee.
///
/// `offered` is `(scroll item id, name, normal success rate, count held)`, already filtered to
/// the ones that fit the chosen equip.
///
/// **The normal rate is shown on every row**, and it is the whole point of the screen: a 10%
/// scroll and a 100% scroll cost the same Treasure Scroll, so a player who cannot see the
/// difference cannot spend it well.
pub fn real_scroll_menu(item_name: &str, offered: &[(u32, String, u16, u16)]) -> String {
    let rows: Vec<String> = offered
        .iter()
        .enumerate()
        .map(|(i, (id, name, success, count))| {
            row(i, *id, name, &format!(" - normally {success}% (x{count})"))
        })
        .collect();
    menu(
        &format!(
            "These are the scrolls you carry that fit your #b{item_name}#k. \
             Whichever you choose will succeed.\n\nWhich one?"
        ),
        &rows,
    )
}

/// The confirm box. Deliberately restates BOTH what is being used and what it is being used
/// on: the owner asked for a confirmation, and a confirmation that does not name what it is
/// confirming is a button.
pub fn confirm(action: Confirmed, item_name: &str, real_name: &str, guaranteed: bool) -> String {
    match action {
        Confirmed::Secrets { mode, .. } => {
            let odds = if mode.always_succeeds() {
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
                "Use a #bScroll of Secrets#k as a #b{}#k on your #b{item_name}#k?\n\n{}\n{odds}\n\n\
                 The scroll is used up either way.",
                mode.name(),
                describe(mode)
            )
        }
        Confirmed::Treasure { .. } => format!(
            "Use your #bTreasure Scroll#k to guarantee a #b{real_name}#k on your \
             #b{item_name}#k?\n\nIt will succeed, and one enhancement slot will be used.\n\n\
             Both scrolls are used up."
        ),
    }
}

pub fn cancelled() -> String {
    "As you like. Come back when you have made up your mind.".to_string()
}

pub fn refused(refusal: Refusal) -> String {
    format!("{} Nothing has been used up.", refusal.line())
}

/// The outcome line. `changes` is every `(name, delta)` the scroll produced.
pub fn outcome(
    action: Confirmed,
    item_name: &str,
    real_name: &str,
    succeeded: bool,
    changes: &[(&str, i32)],
    remaining: u8,
) -> String {
    let head = match action {
        Confirmed::Secrets { mode: SecretsMode::Innocence, .. } => {
            format!("Your #b{item_name}#k is as it was the day it was made.")
        }
        Confirmed::Secrets { mode: SecretsMode::Chaos, .. } if succeeded => match changes.first() {
            Some((stat, d)) if *d > 0 => {
                format!("It worked. #b{item_name}#k gained {d} {stat}.")
            }
            Some((stat, d)) if *d < 0 => {
                format!("It worked, after a fashion. #b{item_name}#k lost {} {stat}.", -d)
            }
            // A roll of zero, or an item with no stats to move. It is still a success and the
            // slot is still gone, so it must not read as a failure.
            _ => format!("It worked, but nothing about #b{item_name}#k changed."),
        },
        Confirmed::Secrets { mode: SecretsMode::Chaos, .. } => {
            format!("It failed, and the slot is gone. #b{item_name}#k is unchanged otherwise.")
        }
        Confirmed::Secrets { mode: SecretsMode::CleanSlate, .. } if succeeded => {
            format!("One of the slots you lost is back. #b{item_name}#k can be scrolled again.")
        }
        Confirmed::Secrets { mode: SecretsMode::CleanSlate, .. } => {
            format!("It failed. #b{item_name}#k keeps the slots it has.")
        }
        // The Treasure Scroll cannot fail, so there is no failure arm to write. A scroll that
        // granted nothing is still a success and still cost a slot, and must say so.
        Confirmed::Treasure { .. } => {
            if changes.is_empty() {
                format!("The #b{real_name}#k took, but it had nothing to give #b{item_name}#k.")
            } else {
                let list: Vec<String> = changes
                    .iter()
                    .map(|(stat, d)| format!("{}{d} {stat}", if *d > 0 { "+" } else { "" }))
                    .collect();
                format!(
                    "The #b{real_name}#k took hold. #b{item_name}#k gained {}.",
                    list.join(", ")
                )
            }
        }
    };
    format!("{head}\n\nEnhancement slots remaining: #b{remaining}#k.")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Written as an escape rather than typed, because a literal CR in this file is invisible
    /// in a diff and this repo has already lost one to a heredoc halving its backslashes.
    const CARRIAGE_RETURN: char = '\r';

    fn worn() -> Vec<(u8, u32, String, u8, u8)> {
        vec![
            (5, 1_322_999, "Wizet Secret Agent Suitcase".to_string(), 6, 1),
            (11, 1_040_002, "Grey T-Shirt".to_string(), 7, 0),
        ]
    }

    /// Real rows, so the fixture and `gm-handbook/scrolls.txt` cannot drift:
    /// `2043200` is *Lesser*, 100%; `2043202` is *Greater*, 10%. Both are One-Handed Blunt
    /// Weapon scrolls, which is the category of the owner's `1322999` suitcase.
    fn offered() -> Vec<(u32, String, u16, u16)> {
        vec![
            (2_043_200, "One-Handed Blunt Weapon Attack Scroll: Lesser".to_string(), 100, 2),
            (2_043_202, "One-Handed Blunt Weapon Attack Scroll: Greater".to_string(), 10, 1),
        ]
    }

    fn every_menu() -> Vec<String> {
        vec![
            pick_menu(&[(Scroll::Secrets, 3), (Scroll::Treasure, 1)]),
            mode_menu([true, false, true]),
            equip_menu(Branch::Secrets(SecretsMode::Chaos), &worn()),
            equip_menu(Branch::Treasure, &worn()),
            real_scroll_menu("Wizet Secret Agent Suitcase", &offered()),
        ]
    }

    /// **The path prefix cannot be confused with any other dialogue's.** A type-6 reply carries
    /// no speaker, so if two features shared a prefix each would answer the other's menu.
    #[test]
    fn the_scroll_paths_are_disjoint_from_every_other_dialogue() {
        let paths = [
            PICK_PATH.to_string(),
            MODE_PATH.to_string(),
            equip_path(Branch::Secrets(SecretsMode::Chaos)),
            equip_path(Branch::Treasure),
            real_path(5),
            confirm_path(Confirmed::Secrets { mode: SecretsMode::Chaos, equip_slot: 5 }),
            confirm_path(Confirmed::Treasure { equip_slot: 5, real_scroll: 2_043_200 }),
        ];
        for p in &paths {
            assert!(is_scroll_path(p), "{p}");
            assert!(!crate::dailyperks::is_menu_path(p), "{p}");
            assert!(!crate::secondjob::is_menu_path(p), "{p}");
        }
        // And the control: this predicate is not simply true.
        assert!(!is_scroll_path(crate::dailyperks::MENU_PATH));
        assert!(!is_scroll_path("shanks.ask"));
        assert!(!is_scroll_path(""));
    }

    /// The state really does survive the round trip through the path - every branch of it.
    #[test]
    fn every_choice_round_trips_through_the_path() {
        for mode in SecretsMode::ALL {
            assert_eq!(
                branch_from_equip_path(&equip_path(Branch::Secrets(mode))),
                Some(Branch::Secrets(mode))
            );
            for slot in [1u8, 5, 11, 255] {
                let c = Confirmed::Secrets { mode, equip_slot: slot };
                assert_eq!(confirmed_from_path(&confirm_path(c)), Some(c));
            }
        }
        assert_eq!(branch_from_equip_path(&equip_path(Branch::Treasure)), Some(Branch::Treasure));
        for slot in [1u8, 5, 255] {
            assert_eq!(equip_slot_from_real_path(&real_path(slot)), Some(slot));
            let c = Confirmed::Treasure { equip_slot: slot, real_scroll: 2_043_200 };
            assert_eq!(confirmed_from_path(&confirm_path(c)), Some(c));
        }
        // Garbage decodes to nothing rather than to a default. A `Secrets` path that names no
        // mode is the dangerous one: a default would silently run a Chaos.
        assert_eq!(branch_from_equip_path("scroll.equip:secrets:nonsense"), None);
        assert_eq!(branch_from_equip_path("scroll.equip:"), None);
        assert_eq!(confirmed_from_path("scroll.confirm:secrets:chaos"), None);
        assert_eq!(confirmed_from_path("scroll.confirm:treasure:5:x"), None);
        assert_eq!(confirmed_from_path("scroll.confirm:other:5:5"), None);
    }

    /// **The clip the owner photographed, as an assertion, over every menu this module builds.**
    ///
    /// 33 of 33 `#L` menus in this client's own `QuestData` end at the final `#l`, so plain
    /// text after one is unattested - and on screen it clips. Only one of these five menus was
    /// photographed; all five are held to it.
    #[test]
    fn nothing_follows_the_final_link_close_in_any_menu() {
        for text in every_menu() {
            assert!(text.ends_with("#l"), "the menu must be the last thing: {text:?}");
            assert_eq!(text.matches("#l").count(), 1, "one close for the whole menu: {text:?}");
            assert!(
                !text.contains(CARRIAGE_RETURN),
                "a CR is not the attested separator: {text:?}"
            );
        }
    }

    /// Every row is numbered from zero and carries its own icon, on every menu.
    ///
    /// The numbering matches what `parse_menu_reply` returns - an off-by-one uses the wrong
    /// scroll, which is unrecoverable for a player - and the icon must be the row's own item,
    /// because an icon showing something else reads as a working feature.
    #[test]
    fn every_menu_row_is_numbered_from_zero_and_carries_its_own_icon() {
        for text in every_menu() {
            assert!(text.contains("#L0##i"), "row 0 has an icon: {text:?}");
            assert!(text.contains("#L1##i"), "row 1 has an icon: {text:?}");
            assert!(!text.contains("#L3#"), "none of these fixtures has four rows: {text:?}");
        }
        // The ids themselves, spot-checked where getting them wrong would be invisible.
        let p = pick_menu(&[(Scroll::Secrets, 3), (Scroll::Treasure, 1)]);
        assert!(p.contains("#L0##i4031065# #bScroll of Secrets#k (x3)"), "{p}");
        assert!(p.contains("#L1##i4031066# #bTreasure Scroll#k (x1)"), "{p}");
        let e = equip_menu(Branch::Treasure, &worn());
        assert!(e.contains("#L0##i1322999# #bWizet Secret Agent Suitcase#k"), "{e}");
        let r = real_scroll_menu("Suitcase", &offered());
        assert!(r.contains("#L0##i2043200#"), "{r}");
    }

    /// The mode menu is the one screen where all three effects are compared, so it must say
    /// each one's odds - and it must not claim a guarantee that the day has already spent.
    #[test]
    fn the_mode_menu_states_each_modes_odds() {
        let fresh = mode_menu([true, true, true]);
        assert!(fresh.contains("Chaos Scroll#k - guaranteed today"), "{fresh}");
        assert!(fresh.contains("Innocence Scroll#k - always works"), "{fresh}");
        let spent = mode_menu([false, true, false]);
        assert!(spent.contains("Chaos Scroll#k - 60%"), "{spent}");
        assert!(spent.contains("Clean Slate Scroll#k - 60%"), "{spent}");
        // Innocence has no daily pass at all, so its row must never move.
        assert!(spent.contains("Innocence Scroll#k - always works"), "{spent}");
    }

    /// The description did not vanish with the clip fix - it moved one screen later, and if it
    /// ever stops being said there the mode menu becomes three unexplained names.
    #[test]
    fn the_equip_menu_still_explains_the_chosen_effect() {
        for mode in SecretsMode::ALL {
            let text = equip_menu(Branch::Secrets(mode), &worn());
            assert!(text.contains(describe(mode)), "{mode:?}: {text}");
        }
    }

    /// The confirm box must name what is being used AND what it is used on, and must state the
    /// odds - a confirmation that does not say what it is confirming is just a button.
    #[test]
    fn the_confirm_box_names_both_and_states_the_odds() {
        let chaos = Confirmed::Secrets { mode: SecretsMode::Chaos, equip_slot: 5 };
        let g = confirm(chaos, "Maple Sword", "", true);
        assert!(g.contains("Chaos Scroll") && g.contains("Maple Sword"), "{g}");
        assert!(g.contains("guaranteed"), "{g}");
        let r = confirm(chaos, "Maple Sword", "", false);
        assert!(r.contains("60%"), "{r}");
        // Innocence never rolls, so it must not offer odds at all.
        let inno = Confirmed::Secrets { mode: SecretsMode::Innocence, equip_slot: 5 };
        let i = confirm(inno, "Maple Sword", "", false);
        assert!(i.contains("always works"), "{i}");
        assert!(!i.contains('%'), "{i}");
        // The Treasure Scroll names the real scroll too - it is spending two items and must
        // say which two.
        let t = Confirmed::Treasure { equip_slot: 5, real_scroll: 2_043_200 };
        let text = confirm(t, "Maple Sword", "Greater Attack Scroll", false);
        assert!(text.contains("Treasure Scroll"), "{text}");
        assert!(text.contains("Greater Attack Scroll"), "{text}");
        assert!(text.contains("Maple Sword"), "{text}");
        assert!(text.contains("Both scrolls are used up"), "{text}");
    }

    /// A Chaos that rolled zero is a SUCCESS that changed nothing. The player still paid a
    /// slot, so wording it as a failure would be a lie about what it cost them.
    #[test]
    fn a_zero_roll_chaos_does_not_read_as_a_failure() {
        let chaos = Confirmed::Secrets { mode: SecretsMode::Chaos, equip_slot: 5 };
        let t = outcome(chaos, "Maple Sword", "", true, &[], 4);
        assert!(t.contains("worked"), "{t}");
        assert!(!t.to_lowercase().contains("failed"), "{t}");
        let f = outcome(chaos, "Maple Sword", "", false, &[], 4);
        assert!(f.contains("failed"), "{f}");
    }

    /// A real scroll can grant several stats at once, and the outcome must list all of them -
    /// reporting only the first would understate what the player just spent two items on.
    #[test]
    fn a_treasure_scroll_outcome_lists_every_stat_it_granted() {
        let t = Confirmed::Treasure { equip_slot: 5, real_scroll: 2_040_800 };
        let text = outcome(t, "Work Gloves", "Gloves Attack Scroll", true,
                           &[("Attack Power", 2), ("Accuracy", 1)], 5);
        assert!(text.contains("+2 Attack Power"), "{text}");
        assert!(text.contains("+1 Accuracy"), "{text}");
        assert!(text.contains("Gloves Attack Scroll"), "{text}");
        // A scroll that granted nothing is still a success and still cost a slot.
        let none = outcome(t, "Work Gloves", "Gloves Attack Scroll", true, &[], 5);
        assert!(!none.to_lowercase().contains("failed"), "{none}");
        assert!(none.contains("nothing to give"), "{none}");
    }

    /// Every outcome states the slots left, because that is the number the player is deciding
    /// on next and the client's own tooltip is the only other place it appears.
    #[test]
    fn every_outcome_reports_the_remaining_slots() {
        let mut actions: Vec<Confirmed> = SecretsMode::ALL
            .iter()
            .map(|&mode| Confirmed::Secrets { mode, equip_slot: 5 })
            .collect();
        actions.push(Confirmed::Treasure { equip_slot: 5, real_scroll: 2_043_200 });
        for action in actions {
            for ok in [true, false] {
                let t = outcome(action, "Maple Sword", "Scroll", ok, &[("STR", 2)], 3);
                assert!(t.contains("Enhancement slots remaining: #b3#k"), "{t}");
            }
        }
    }
}
