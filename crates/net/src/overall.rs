//! **An overall and a bottom cannot be worn at once**, and the server has to enforce it.
//!
//! The owner, 2026-08-29: *"when I wore an overall item, the server did not take off the bottoms
//! that I was wearing. Overalls and tops and bottoms are not compatible. Wearing an overall
//! should force the server to unequip the player's top and bottom unless the player does not
//! have sufficient inventory space."*
//!
//! # What the equip slots already do, and the one thing they do not
//!
//! A top and an overall both go to **slot 5**, so the ordinary swap in
//! `Store::equip_from_bag` already takes the top off: the incoming item displaces whatever
//! was there. That half needed no rule.
//!
//! The bottom is **slot 6**, a different slot, so nothing displaces it. Without this the
//! character wears an overall and a pair of trousers at the same time - which is what the owner
//! saw, and which the real game does not allow.
//!
//! # The ranges, checked against the data rather than remembered
//!
//! `gm-handbook/equips.txt`, counted:
//!
//! ```text
//! 1040000..1049999   274 items   coats / tops        -> slot 5
//! 1050000..1059999   178 items   longcoats, robes    -> slot 5, and forbids slot 6
//! 1060000..1069999   253 items   trousers / bottoms  -> slot 6
//! ```
//!
//! The first three of each were read to confirm the naming: `Blue Striped Undershirt`,
//! `Beige Plain Robe`, `Blue-Striped Boxers`. The ranges are the game's own item-id scheme -
//! `itemId / 10000` is the equip category - so this is a reading of the id, not a table that
//! can fall out of date with the WZ.
//!
//! # It is the id that decides, not the slot the client asked for
//!
//! The client sends the worn slot it wants. Trusting that to identify an overall would mean
//! trusting the client to tell us what it is wearing, and nothing on this socket is
//! authenticated. The item id is ours: it came out of the bag row the server owns.

/// Equip category, from the item id. `itemId / 10000`.
fn category(item_id: u32) -> u32 {
    item_id / 10_000
}

/// A coat or shirt: worn in slot 5, compatible with a bottom.
pub fn is_top(item_id: u32) -> bool {
    category(item_id) == 104
}

/// A longcoat, robe or dress: worn in slot 5, and **forbids a bottom**.
pub fn is_overall(item_id: u32) -> bool {
    category(item_id) == 105
}

/// Trousers or a skirt: worn in slot 6.
pub fn is_bottom(item_id: u32) -> bool {
    category(item_id) == 106
}

/// The worn slot a top or overall occupies.
pub const SLOT_TOP: u8 = 5;

/// The worn slot a bottom occupies.
pub const SLOT_BOTTOM: u8 = 6;

/// Which worn slot must be emptied before `item_id` can go on, if any.
///
/// **Both directions**, because the incompatibility is symmetric and only enforcing one of
/// them leaves the same picture on screen from the other order:
///
/// * putting on an **overall** must take off the **bottom** (slot 6);
/// * putting on a **bottom** must take off an **overall** (slot 5) - but not a plain top,
///   which is compatible.
///
/// Returns `None` when nothing has to come off, which is every other equip in the game. The
/// caller still has to check what is actually *in* that slot: this function knows the rule,
/// not the character.
pub fn conflicting_slot(item_id: u32) -> Option<u8> {
    if is_overall(item_id) {
        Some(SLOT_BOTTOM)
    } else if is_bottom(item_id) {
        // Only an OVERALL in slot 5 conflicts. A top there is fine, and the caller decides by
        // looking at the item that is actually worn - see `conflicts_with`.
        Some(SLOT_TOP)
    } else {
        None
    }
}

/// Does wearing `incoming` require taking off `worn`, given `worn` is in `slot`?
///
/// The pair test rather than the slot test, so a plain top in slot 5 is not removed to make
/// room for trousers.
pub fn conflicts_with(incoming: u32, worn: u32, slot: u8) -> bool {
    match slot {
        SLOT_BOTTOM => is_overall(incoming) && is_bottom(worn),
        SLOT_TOP => is_bottom(incoming) && is_overall(worn),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Real ids out of `gm-handbook/equips.txt`, named, so a range typo is visible.
    const BEIGE_PLAIN_ROBE: u32 = 1_050_000; // overall
    const BLUE_STRIPED_UNDERSHIRT: u32 = 1_040_000; // top
    const BLUE_STRIPED_BOXERS: u32 = 1_060_000; // bottom
    const SWORD: u32 = 1_302_000;

    #[test]
    fn the_three_categories_are_told_apart() {
        assert!(is_overall(BEIGE_PLAIN_ROBE));
        assert!(!is_top(BEIGE_PLAIN_ROBE));
        assert!(!is_bottom(BEIGE_PLAIN_ROBE));

        assert!(is_top(BLUE_STRIPED_UNDERSHIRT));
        assert!(!is_overall(BLUE_STRIPED_UNDERSHIRT));

        assert!(is_bottom(BLUE_STRIPED_BOXERS));
        assert!(!is_overall(BLUE_STRIPED_BOXERS));
    }

    #[test]
    fn the_range_boundaries_are_where_the_data_says() {
        assert!(is_top(1_049_999));
        assert!(is_overall(1_050_000));
        assert!(is_overall(1_059_999));
        assert!(is_bottom(1_060_000));
        assert!(is_bottom(1_069_999));
        assert!(!is_bottom(1_070_000));
    }

    #[test]
    fn an_overall_conflicts_with_a_worn_bottom() {
        assert_eq!(conflicting_slot(BEIGE_PLAIN_ROBE), Some(SLOT_BOTTOM));
        assert!(conflicts_with(BEIGE_PLAIN_ROBE, BLUE_STRIPED_BOXERS, SLOT_BOTTOM));
    }

    /// The other direction. Enforcing only the first leaves the same picture on screen from
    /// the other order - overall on, then trousers on top of it.
    #[test]
    fn a_bottom_conflicts_with_a_worn_overall() {
        assert_eq!(conflicting_slot(BLUE_STRIPED_BOXERS), Some(SLOT_TOP));
        assert!(conflicts_with(BLUE_STRIPED_BOXERS, BEIGE_PLAIN_ROBE, SLOT_TOP));
    }

    /// **The control that stops this over-reaching.** A plain top and a bottom are worn
    /// together by every character in the game; removing one to put on the other would be a
    /// far worse bug than the one this module fixes.
    #[test]
    fn a_top_and_a_bottom_do_not_conflict() {
        assert!(!conflicts_with(BLUE_STRIPED_BOXERS, BLUE_STRIPED_UNDERSHIRT, SLOT_TOP));
        assert!(!conflicts_with(BLUE_STRIPED_UNDERSHIRT, BLUE_STRIPED_BOXERS, SLOT_BOTTOM));
    }

    #[test]
    fn an_overall_replacing_an_overall_needs_nothing_taken_off_the_other_slot() {
        // Slot 5's ordinary swap handles it: the incoming robe displaces the worn one.
        assert!(!conflicts_with(BEIGE_PLAIN_ROBE, 1_050_001, SLOT_TOP));
    }

    #[test]
    fn everything_else_conflicts_with_nothing() {
        assert_eq!(conflicting_slot(SWORD), None);
        assert!(!conflicts_with(SWORD, BLUE_STRIPED_BOXERS, SLOT_BOTTOM));
        // A hat, a shoe, an earring.
        for id in [1_002_000_u32, 1_072_000, 1_032_000] {
            assert_eq!(conflicting_slot(id), None, "item {id}");
        }
    }
}
