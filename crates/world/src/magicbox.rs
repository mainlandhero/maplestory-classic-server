//! **Companion's Magic Box** (`2430000`) - First Time Together's reward, opened by a
//! double-click.
//!
//! The owner, 2026-09-23, with the community table of what the box has been seen to give: *"By
//! double clicking the box, it should automatically generate one of these possible contents
//! for the player. For ETC items, let's make sure all ores given are 8 quantity, screw is the
//! only one given in 10 quantity. Use items should include all 60% or 10% Attack scrolls as a
//! possibility. Or it could be 100 Blue Potions, 100 Orange Potions, 100 White Potions, or 20
//! Elixirs. Equipment drops will remain the same."*
//!
//! # How it reaches the server
//!
//! `0x0114`, from the **Use** tab - the same route as the Leaf Point Exchange Coupons
//! (`crate::leafcoupons` has the dispatcher read), which the owner confirmed on screen the same
//! day. `FUN_140416d60`'s exclusions from that route all sit at `2432495` and above (or in
//! `202xxxx`), so `2430000` is not among them **[L]**.
//!
//! # The table, and three readings of the owner's words
//!
//! Every id below was resolved by exact name in `gm-handbook/items.txt` (one match each).
//!
//! * **"60% or 10% Attack scrolls"** is this client's *Intermediate* (success 60) and
//!   *Greater* (success 10) rows whose name ends `Attack Scroll` - 32 of them, including the
//!   six Magic Attack scrolls (gloves, wand, staff). The *Chaos* rows are also 10% but carry
//!   a 50% curse; they are a different scroll and are left out. `gm-handbook/scrolls.txt`,
//!   pinned by a test.
//! * **The four non-attack scrolls in the community table** (Hat Accuracy, Overall DEF,
//!   Bottomwear DEF, Overall STR - all *Greater*, 10%) are **kept**: "should include" read as
//!   adding to the table, not replacing it.
//! * **Odds are equal per line**, the plainest reading of "one of these possible contents".
//!   With 32 attack scrolls in a table of 61 lines that makes an attack scroll a little over
//!   half of all boxes - [`share`] states it, and it is the number to change if that is not
//!   what was meant.

/// The box.
pub const BOX: u32 = 2_430_000;

/// One possible content: `(item id, quantity)`.
pub type Prize = (u32, u16);

/// Ores, eight each, and the Screw, ten.
pub const ETC: [Prize; 14] = [
    (4_010_006, 8),  // Gold Ore
    (4_010_004, 8),  // Silver Ore
    (4_020_006, 8),  // Topaz Ore
    (4_010_003, 8),  // Adamantium Ore
    (4_020_001, 8),  // Amethyst Ore
    (4_020_002, 8),  // Aquamarine Ore
    (4_020_008, 8),  // Black Crystal Ore
    (4_020_007, 8),  // Diamond Ore
    (4_020_003, 8),  // Emerald Ore
    (4_020_000, 8),  // Garnet Ore
    (4_010_001, 8),  // Iron Ore
    (4_010_002, 8),  // Mithril Ore
    (4_020_005, 8),  // Sapphire Ore
    (4_003_000, 10), // Screw
];

/// Every 60% and 10% attack scroll - `Intermediate` and `Greater`, magic attack included.
pub const ATTACK_SCROLLS: [u32; 32] = [
    2_040_801, 2_040_802, // Gloves Attack
    2_040_805, 2_040_806, // Gloves Magic Attack
    2_043_001, 2_043_002, // One-Handed Sword
    2_043_101, 2_043_102, // One-Handed Axe
    2_043_201, 2_043_202, // One-Handed Blunt Weapon
    2_043_301, 2_043_302, // Dagger
    2_043_701, 2_043_702, // Wand Magic Attack
    2_043_801, 2_043_802, // Staff Magic Attack
    2_044_001, 2_044_002, // Two-handed Sword
    2_044_101, 2_044_102, // Two-handed Axe
    2_044_201, 2_044_202, // Two-handed Blunt Weapon
    2_044_301, 2_044_302, // Spear
    2_044_401, 2_044_402, // Polearm
    2_044_501, 2_044_502, // Bow
    2_044_601, 2_044_602, // Crossbow
    2_044_701, 2_044_702, // Claw
];

/// The rest of the Use lines: the community table's four scrolls, and the owner's potion bundles.
pub const USE_OTHER: [Prize; 8] = [
    (2_040_002, 1),   // Hat Accuracy Scroll: Greater
    (2_040_502, 1),   // Overall Armor DEF Scroll: Greater
    (2_040_602, 1),   // Bottomwear DEF Scroll: Greater
    (2_040_506, 1),   // Overall Armor STR Scroll: Greater
    (2_000_003, 100), // Blue Potion
    (2_000_001, 100), // Orange Potion
    (2_000_002, 100), // White Potion
    (2_000_004, 20),  // Elixir
];

/// "Equipment drops will remain the same" - the community table's seven, one each.
pub const EQUIPS: [Prize; 7] = [
    (1_002_083, 1), // Blue Bamboo Hat
    (1_002_081, 1), // Brown Bamboo Hat
    (1_002_082, 1), // Green Bamboo Hat
    (1_032_005, 1), // Red Cross Earrings
    (1_032_007, 1), // Emerald Earrings
    (1_032_008, 1), // Star Earrings
    (1_032_003, 1), // Yellow Square
];

/// How many lines the table has.
pub const LINES: usize = ETC.len() + ATTACK_SCROLLS.len() + USE_OTHER.len() + EQUIPS.len();

/// The `n`th line, `0..LINES`.
pub fn line(n: usize) -> Prize {
    let n = n % LINES;
    if n < ETC.len() {
        return ETC[n];
    }
    let n = n - ETC.len();
    if n < ATTACK_SCROLLS.len() {
        return (ATTACK_SCROLLS[n], 1);
    }
    let n = n - ATTACK_SCROLLS.len();
    if n < USE_OTHER.len() {
        return USE_OTHER[n];
    }
    EQUIPS[n - USE_OTHER.len()]
}

/// **Open one box**: one line, equal odds, from one roll.
pub fn roll(r: u64) -> Prize {
    line(usize::try_from(r % LINES as u64).unwrap_or(0))
}

/// The chance, out of [`LINES`], that a box gives something in `ids` - for saying the odds
/// out loud.
pub fn share(ids: &[u32]) -> usize {
    (0..LINES).filter(|&n| ids.contains(&line(n).0)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The owner's quantities: every ore 8, the Screw 10, the potions 100 and the Elixir 20,
    /// everything else one.
    #[test]
    fn the_quantities_are_wisps() {
        for (id, qty) in ETC {
            let want = if id == 4_003_000 { 10 } else { 8 };
            assert_eq!(qty, want, "{id}");
        }
        assert_eq!(USE_OTHER[4..], [(2_000_003, 100), (2_000_001, 100), (2_000_002, 100), (2_000_004, 20)]);
        assert!(EQUIPS.iter().all(|&(_, q)| q == 1));
        assert!(USE_OTHER[..4].iter().all(|&(_, q)| q == 1));
    }

    /// Every roll lands on a line, every line is reachable, and no id appears twice.
    #[test]
    fn every_line_is_reachable_and_distinct() {
        assert_eq!(LINES, 61);
        let mut seen = std::collections::HashSet::new();
        for r in 0..LINES as u64 {
            assert!(seen.insert(roll(r).0), "{:?} twice", roll(r));
        }
        assert_eq!(seen.len(), LINES);
        assert_eq!(roll(u64::MAX), line((u64::MAX % LINES as u64) as usize));
        assert_eq!(share(&ATTACK_SCROLLS), 32, "32 of 61 - a little over half");
    }

    /// **The attack scrolls are exactly the client's 60% and 10% ones.** Read against
    /// `gm-handbook/scrolls.txt`: every row named `... Attack Scroll: Intermediate` (60) or
    /// `... Attack Scroll: Greater` (10, uncursed) is in the table, and nothing else is.
    /// Skipped when the generated handbook is absent.
    #[test]
    fn the_attack_scrolls_are_the_clients_sixty_and_ten_percent_rows() {
        let Ok(text) = std::fs::read_to_string("../../gm-handbook/scrolls.txt") else { return };
        let mut want: Vec<u32> = text
            .lines()
            .filter(|l| !l.starts_with('#'))
            .filter_map(|l| {
                let cols: Vec<&str> = l.split(", ").collect();
                let name = *cols.last()?;
                let success: u32 = cols.get(1)?.parse().ok()?;
                let cursed: u32 = cols.get(2)?.parse().ok()?;
                let hit = name.contains("Attack Scroll")
                    && ((success == 60 && name.ends_with(": Intermediate")) || (success == 10 && cursed == 0 && name.ends_with(": Greater")));
                hit.then(|| cols[0].parse().ok()).flatten()
            })
            .collect();
        want.sort_unstable();
        let mut have = ATTACK_SCROLLS.to_vec();
        have.sort_unstable();
        assert_eq!(have, want);
    }

    /// Every id is the item its comment names - checked against `gm-handbook/items.txt`, so
    /// a typo in a seven-digit number fails here rather than handing out the wrong thing.
    #[test]
    fn every_id_is_the_item_the_table_names() {
        let Ok(text) = std::fs::read_to_string("../../gm-handbook/items.txt") else { return };
        let name = |id: u32| {
            text.lines().find_map(|l| l.strip_prefix(&format!("{id}, ")).map(str::to_string))
        };
        let expect: [(u32, &str); 12] = [
            (4_010_006, "Gold Ore"),
            (4_020_008, "Black Crystal Ore"),
            (4_003_000, "Screw"),
            (2_040_002, "Hat Accuracy Scroll: Greater"),
            (2_040_506, "Overall Armor STR Scroll: Greater"),
            (2_000_003, "Blue Potion"),
            (2_000_001, "Orange Potion"),
            (2_000_002, "White Potion"),
            (2_000_004, "Elixir"),
            (1_002_082, "Green Bamboo Hat"),
            (1_032_003, "Yellow Square"),
            (BOX, "Companion's Magic Box"),
        ];
        for (id, want) in expect {
            assert_eq!(name(id).as_deref(), Some(want), "{id}");
        }
        for n in 0..LINES {
            let (id, _) = line(n);
            assert!(name(id).is_some(), "{id} is not an item in this client");
        }
    }
}
