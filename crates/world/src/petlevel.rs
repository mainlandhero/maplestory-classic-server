//! Pet closeness and level: the table the owner pasted, and the two edges around a feed.
//!
//! The owner, 2026-09-15, with the wiki's **Pet Closeness** page: a level is reached at a total
//! closeness (level 2 at 1, level 3 at 3, ... level 30 at 30 000), and *"lowered closeness
//! will not cause pet level to decrease"*. Closeness moves by:
//!
//! * a feed: `+1` (the owner's number - `net::petfood::PET_FOOD_CLOSENESS`);
//! * a successful command: `+1..+3`, the entry's own `inc` (`crate::petcommands`);
//! * an overfeed - food on a pet already at 100 - `-1` **after the first one**;
//! * starvation - fullness reaching 0 - `-1`, and the pet goes home.
//!
//! The Henesys / Ludibrium Pet Parks are quests this server does not have; not modelled.

/// The total closeness at which each level begins, index `level - 1`. Level 1 starts at 0 and
/// level 30 at 30 000 - the wiki table, column "Total Closeness". **[R]**, the page the owner gave.
pub const LEVEL_STARTS_AT: [u32; 30] = [
    0, 1, 3, 6, 14, 31, 60, 108, 181, 287, //         1..10
    434, 632, 891, 1_224, 1_642, 2_161, 2_793, 3_557, 4_467, 5_542, // 11..20
    6_801, 8_263, 9_950, 11_882, 14_084, 16_578, 19_391, 22_548, 26_074, 30_000, // 21..30
];

pub const MAX_LEVEL: u8 = 30;

/// The level a total closeness has EARNED - the highest whose start it has reached.
pub fn level_for_closeness(closeness: u32) -> u8 {
    let reached = LEVEL_STARTS_AT.iter().filter(|&&start| closeness >= start).count();
    u8::try_from(reached.clamp(1, usize::from(MAX_LEVEL))).unwrap_or(1)
}

/// **The level after a closeness change: it can go up, never down.** `current` is what the
/// pet already holds; the wiki's rule is that a loss of closeness leaves the level alone.
pub fn level_after(current: u8, closeness: u32) -> u8 {
    current.max(level_for_closeness(closeness)).min(MAX_LEVEL)
}

/// One feed, applied to `(fullness, closeness)`. `overfeeds` is how many times this pet has
/// been fed while already full during the session - the first is free, every later one costs
/// a closeness. Returns the new `(fullness, closeness, overfeeds)`.
pub fn feed(fullness: u8, closeness: u32, overfeeds: u32) -> (u8, u32, u32) {
    use net::petfood::{PET_FOOD_CLOSENESS, PET_FOOD_FULLNESS, PET_FULLNESS_MAX};
    if fullness >= PET_FULLNESS_MAX {
        let overfeeds = overfeeds + 1;
        let closeness = if overfeeds > 1 { closeness.saturating_sub(1) } else { closeness };
        return (PET_FULLNESS_MAX, closeness, overfeeds);
    }
    (
        fullness.saturating_add(PET_FOOD_FULLNESS).min(PET_FULLNESS_MAX),
        closeness + u32::from(PET_FOOD_CLOSENESS),
        overfeeds,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The table's own spot checks: the multipliers column is derived, the starts are the data.
    #[test]
    fn the_table_is_the_wikis_and_the_level_is_the_highest_start_reached() {
        assert_eq!(LEVEL_STARTS_AT[0], 0);
        assert_eq!(LEVEL_STARTS_AT[1], 1, "level 2 at 1");
        assert_eq!(LEVEL_STARTS_AT[9], 287, "level 10 at 287");
        assert_eq!(LEVEL_STARTS_AT[29], 30_000, "level 30 at 30 000");
        assert!(LEVEL_STARTS_AT.windows(2).all(|w| w[0] < w[1]), "strictly rising");
        // "Closeness to next level" column, spot-checked: 6 -> 14 is 8, 287 -> 434 is 147.
        assert_eq!(LEVEL_STARTS_AT[4] - LEVEL_STARTS_AT[3], 8);
        assert_eq!(LEVEL_STARTS_AT[10] - LEVEL_STARTS_AT[9], 147);

        assert_eq!(level_for_closeness(0), 1);
        assert_eq!(level_for_closeness(1), 2);
        assert_eq!(level_for_closeness(2), 2);
        assert_eq!(level_for_closeness(3), 3);
        assert_eq!(level_for_closeness(286), 9);
        assert_eq!(level_for_closeness(287), 10);
        assert_eq!(level_for_closeness(29_999), 29);
        assert_eq!(level_for_closeness(30_000), 30);
        assert_eq!(level_for_closeness(u32::MAX), 30, "capped");
    }

    /// The wiki's sentence, as a function: a loss leaves the level where it was.
    #[test]
    fn a_lost_closeness_never_lowers_the_level() {
        assert_eq!(level_after(1, 1), 2, "earned");
        assert_eq!(level_after(5, 0), 5, "kept after losses");
        assert_eq!(level_after(30, 0), 30);
    }

    /// The owner's two numbers, the cap, and the overfeed rule with its one free pass.
    #[test]
    fn a_feed_is_plus_thirty_and_plus_one_until_full_then_overfeeding_costs_after_the_first() {
        assert_eq!(feed(50, 10, 0), (80, 11, 0));
        assert_eq!(feed(90, 10, 0), (100, 11, 0), "capped at 100, the closeness still earned");
        assert_eq!(feed(100, 11, 0), (100, 11, 1), "the first overfeed is free");
        assert_eq!(feed(100, 11, 1), (100, 10, 2), "the second costs one");
        assert_eq!(feed(100, 0, 1), (100, 0, 2), "and never below zero");
    }
}
