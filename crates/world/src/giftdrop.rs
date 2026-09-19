//! **`!giftdrop` / `!giftall`** - compensation items, handed out through the Maple
//! Administrator's dialogue.
//!
//! The owner, 2026-09-18, with a screenshot of the modern client's Gift Drop window: *"Can you take
//! a look and see if a gift drop functionality exists in Classic Client so I can send items to
//! players as compensation if necessary?"* It does not: the window is later than this client
//! (no UI image, none of its strings), the client's own mailbox window has no way in that any
//! scan could find, and the Cash Shop locker was refused - *"the Cash Shop should not handle
//! items that are not Cash Items."* So: *"Can we do it via our usual MapleStory Administrator,
//! but this time it is via !giftdrop, and our usual show NPC chat dialogue."* Then: *"I want
//! all gifts to expire in 7 days if unclaimed. Additionally, there should also be a !giftall
//! command which gives all accounts (not character) an item. The player can claim it on any
//! character they want. The dialogue to claim should also have a cancel option just in case if
//! the player wants to change the character they want to claim it on."*
//!
//! Same bones as `crate::dailyperks`: the Administrator's portrait
//! ([`crate::dailyperks::ADMIN_TEMPLATE`]) on a type-6 menu, answers told from every other
//! menu's by the conversation path, nothing spawned. The rows live in `store::gifts`
//! (`store::GIFT_TTL_SECS` is the seven days); the session code is `session/giftdrop.rs`.
//!
//! # Three commands
//!
//! * **`!giftdrop <player> <itemId> [count] [message...]`** - a GM queues a gift for one
//!   character. On this channel now: their box opens at once. Otherwise: at their next field
//!   entry, on their first move (the same "the client is provably live" moment the pet
//!   re-summon uses, so a `0x055B` never lands with a `SetField`).
//! * **`!giftall <itemId> [count] [message...]`** - a GM queues one gift per **account** that
//!   exists at that moment; any character of the account can claim it, and the first to do so
//!   settles it for all of them. Every account's online character on this channel gets the box
//!   at once; the rest at their next field entry.
//! * **`!giftdrop`** with nothing after it - anyone: open the box for whatever is waiting,
//!   or be told there is nothing. Public on purpose, like `!tool`: it grants nothing by
//!   itself, and a claim is gated on a pending row that only a GM can create.
//!
//! # The box
//!
//! One gift at a time, oldest first, so the claim's selection number can only ever mean one
//! row. `#i<id># #t<id>#` draws the item's icon and name the way the Signature Style receipt
//! does (`crate::signaturestyle::receipt_text`, on screen). Three choices, fixed:
//! **Claim** (0) runs the quest room check (`crate::questroom`) first - a full tab gets the
//! same "make N spaces in your <Tab> tab" box the quests use and the gift stays queued;
//! **Refuse** (1) settles the row without giving anything; **Cancel** (2) leaves it queued,
//! which is how an account gift is carried to another character. Closing the box is a
//! Cancel. When more are waiting, the next opens after a Claim or a Refuse.

/// The chat word for one character, without its `!`.
pub const COMMAND: &str = "giftdrop";
/// As typed, for help text and log lines.
pub const COMMAND_TYPED: &str = "!giftdrop";
/// The chat word for every account, without its `!`.
pub const COMMAND_ALL: &str = "giftall";

/// The prefix every gift-drop conversation path starts with. Disjoint from the taxi's, the
/// instructor's, the daily perks', the job guide's, the scroll picker's and the package
/// chooser's; the tests beside those assert it.
pub const PATH_PREFIX: &str = "giftdrop.";

/// The path a live gift box is parked under: `giftdrop.<gift row id>`.
pub fn menu_path(gift_id: i64) -> String {
    format!("{PATH_PREFIX}{gift_id}")
}

/// The gift row a live gift box is about, or `None` for any other conversation.
pub fn gift_id_from_path(path: &str) -> Option<i64> {
    path.strip_prefix(PATH_PREFIX)?.parse().ok()
}

/// Selection 0 on the menu.
pub const SELECT_CLAIM: u32 = 0;
/// Selection 1 on the menu.
pub const SELECT_REFUSE: u32 = 1;
/// Selection 2 on the menu: leave it queued, for another time or another character.
pub const SELECT_CANCEL: u32 = 2;

/// The box for one gift: the heading, the sender's message, the reward with its icon and
/// name, who it is for, when it expires, then the three choices. `remaining` is how many more
/// wait behind this one.
pub fn menu_text(message: &str, item_id: u32, count: u16, sender: &str, for_account: bool, days_left: i64, remaining: usize) -> String {
    let br = crate::dailyperks::LINE_BREAK;
    let mut out = String::from("#bGIFT DROP#k");
    out.push_str(br);
    if !message.trim().is_empty() {
        out.push_str(message.trim());
        out.push_str(br);
    }
    out.push_str(&format!("Reward: #i{item_id}# #t{item_id}# x{count}"));
    if !sender.is_empty() {
        out.push_str(&format!(" (from {sender})"));
    }
    out.push_str(br);
    if for_account {
        out.push_str("For your account: claim it on whichever character you like. ");
    }
    out.push_str(&match days_left {
        0 | 1 => "Expires within a day.".to_string(),
        d => format!("Expires in {d} days."),
    });
    if remaining > 0 {
        out.push_str(br);
        out.push_str(&format!("{remaining} more waiting after this one."));
    }
    out.push_str(br);
    out.push_str(br);
    out.push_str(&crate::dailyperks::menu_line(SELECT_CLAIM, "Claim"));
    out.push_str(br);
    out.push_str(&crate::dailyperks::menu_line(SELECT_REFUSE, "Refuse"));
    out.push_str(br);
    out.push_str(&crate::dailyperks::menu_line(SELECT_CANCEL, "Cancel (keep it for later or for another character)"));
    out
}

/// What they say when nothing is waiting - the modern window's own wording.
pub fn nothing_to_claim() -> String {
    "There's nothing to claim right now.".to_string()
}

/// After a claim: what went into the bag.
pub fn claimed_text(item_id: u32, count: u16) -> String {
    format!("Claimed: #i{item_id}# #t{item_id}# x{count} has been placed in your inventory.")
}

/// After a refuse.
pub fn refused_text(item_id: u32) -> String {
    format!("You refused #t{item_id}#. It will not be offered again.")
}

/// After a cancel.
pub fn cancelled_text() -> String {
    format!("Kept for later. Type {COMMAND_TYPED} on any of your characters to see it again.")
}

/// The line a player sees at login when something is waiting.
pub fn waiting_notice(n: usize) -> String {
    if n == 1 {
        format!("You have a gift waiting. Type {COMMAND_TYPED} to see it.")
    } else {
        format!("You have {n} gifts waiting. Type {COMMAND_TYPED} to see them.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The path round-trips its row id and is nobody else's.
    #[test]
    fn the_path_carries_the_row_and_is_disjoint_from_every_other_menu() {
        let p = menu_path(42);
        assert_eq!(gift_id_from_path(&p), Some(42));
        assert_eq!(gift_id_from_path("giftdrop."), None);
        assert_eq!(gift_id_from_path("dailyperk.menu"), None);
        assert!(!crate::dailyperks::is_menu_path(&p));
        assert!(!crate::taxi::is_taxi_path(&p));
        assert!(!crate::secondjob::is_menu_path(&p));
        assert!(!crate::jobguide::is_menu_path(&p));
        assert!(!crate::scrollnpc::is_scroll_path(&p));
        assert_ne!(p, crate::questroom::REFUSAL_PATH);
    }

    /// The text is the shape the client draws: heading, message, an icon+name reward line,
    /// the expiry, then `#L0# Claim#l`, `#L1# Refuse#l`, `#L2# Cancel...#l`, so the answer's
    /// number is fixed.
    #[test]
    fn the_box_lists_claim_refuse_and_cancel_in_that_order() {
        let t = menu_text("Sorry about the crash.", 1_302_000, 1, "Wisp", false, 7, 0);
        assert!(t.starts_with("#bGIFT DROP#k"));
        assert!(t.contains("Sorry about the crash."));
        assert!(t.contains("Reward: #i1302000# #t1302000# x1 (from the owner)"));
        assert!(t.contains("Expires in 7 days."), "{t}");
        assert!(!t.contains("For your account"));
        let claim = t.find("#L0# Claim#l").expect("claim");
        let refuse = t.find("#L1# Refuse#l").expect("refuse");
        let cancel = t.find("#L2# Cancel").expect("cancel");
        assert!(claim < refuse && refuse < cancel, "{t}");
        assert!(!t.contains("more waiting"));
        let more = menu_text("", 2_000_000, 50, "", true, 1, 2);
        assert!(more.contains("For your account: claim it on whichever character you like."));
        assert!(more.contains("Expires within a day."));
        assert!(more.contains("2 more waiting after this one."));
        assert!(!more.contains("(from"), "no sender, no parenthesis");
        assert_eq!(waiting_notice(1), "You have a gift waiting. Type !giftdrop to see it.");
        assert_eq!(waiting_notice(3), "You have 3 gifts waiting. Type !giftdrop to see them.");
    }
}
