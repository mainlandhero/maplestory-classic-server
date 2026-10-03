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
//! # Four slots, one prize from each (the owner, 2026-10-03)
//!
//! *"I need to change the PQ drops from 1 item from Companion's Magic Box to 1 per type of
//! item. It will always give 1 equip, 1 use, 1 scroll, and 1 etc item."* So a box is no longer
//! one roll over one table: it is **four** rolls, one per [`SLOTS`] entry, and every box hands
//! over four things. Inside a slot the odds are equal per line.
//!
//! * **Equip** - "Equipment drops will remain the same": the community table's seven.
//! * **Use** - the owner's list of the same day, potions and food, with their quantities. The
//!   2026-09-23 potion bundles (100 Blue, Orange, White, 20 Elixir) are in it unchanged.
//!   **Salad** and **Fried Chicken** were given with no quantity, so they are one each; every
//!   other food on that list is fifty. That is the reading to change if it was an omission.
//! * **Scroll** - the 2026-09-23 scrolls, split out of the old Use lines: every 60% and 10%
//!   attack scroll, and the community table's four non-attack scrolls.
//! * **Etc** - ores eight each, the Screw ten.
//!
//! Every id below the potions was resolved by exact name in `gm-handbook/items.txt` (one match
//! each); the 2026-10-03 Use ids are the owner's own, and the test that reads the handbook
//! checks each one is an item this client has.
//!
//! The 2026-09-23 readings still stand inside the scroll slot:
//!
//! * **"60% or 10% Attack scrolls"** is this client's *Intermediate* (success 60) and
//!   *Greater* (success 10) rows whose name ends `Attack Scroll` - 32 of them, including the
//!   six Magic Attack scrolls (gloves, wand, staff). The *Chaos* rows are also 10% but carry
//!   a 50% curse; they are a different scroll and are left out. `gm-handbook/scrolls.txt`,
//!   pinned by a test.
//! * **The four non-attack scrolls in the community table** (Hat Accuracy, Overall DEF,
//!   Bottomwear DEF, Overall STR - all *Greater*, 10%) are **kept**.

/// The box.
pub const BOX: u32 = 2_430_000;

/// One possible content: `(item id, quantity)`.
pub type Prize = (u32, u16);

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

/// The owner's Use list, 2026-10-03, in their order and quantities.
pub const USE: [Prize; 20] = [
    (2_000_001, 100), // Orange Potion
    (2_000_002, 100), // White Potion
    (2_000_003, 100), // Blue Potion
    (2_000_006, 100), // Mana Elixir
    (2_000_004, 20),  // Elixir
    (2_022_000, 20),  // Pure Water
    (2_012_002, 10),  // Sap of Ancient Tree
    (2_012_001, 10),  // Fairy's Honey
    (2_020_000, 1),   // Salad - no quantity given
    (2_020_001, 1),   // Fried Chicken - no quantity given
    (2_020_002, 50),  // Cake
    (2_020_003, 50),  // Pizza
    (2_020_004, 50),  // Hamburger
    (2_020_005, 50),  // Hot Dog
    (2_020_006, 50),  // Hot Dog Supreme
    (2_020_007, 50),  // Dried Squid
    (2_020_008, 50),  // Fat Sausage
    (2_020_009, 50),  // Orange Juice
    (2_020_010, 50),  // Grape Juice
    (2_020_011, 50),  // W Ramen
];

/// How many of [`SCROLLS`]' lines, from the front, are the 60% and 10% attack scrolls.
pub const ATTACK_SCROLLS: usize = 32;

/// Every 60% and 10% attack scroll - `Intermediate` and `Greater`, magic attack included -
/// then the community table's four other scrolls. One each.
pub const SCROLLS: [Prize; 36] = [
    (2_040_801, 1), (2_040_802, 1), // Gloves Attack
    (2_040_805, 1), (2_040_806, 1), // Gloves Magic Attack
    (2_043_001, 1), (2_043_002, 1), // One-Handed Sword
    (2_043_101, 1), (2_043_102, 1), // One-Handed Axe
    (2_043_201, 1), (2_043_202, 1), // One-Handed Blunt Weapon
    (2_043_301, 1), (2_043_302, 1), // Dagger
    (2_043_701, 1), (2_043_702, 1), // Wand Magic Attack
    (2_043_801, 1), (2_043_802, 1), // Staff Magic Attack
    (2_044_001, 1), (2_044_002, 1), // Two-handed Sword
    (2_044_101, 1), (2_044_102, 1), // Two-handed Axe
    (2_044_201, 1), (2_044_202, 1), // Two-handed Blunt Weapon
    (2_044_301, 1), (2_044_302, 1), // Spear
    (2_044_401, 1), (2_044_402, 1), // Polearm
    (2_044_501, 1), (2_044_502, 1), // Bow
    (2_044_601, 1), (2_044_602, 1), // Crossbow
    (2_044_701, 1), (2_044_702, 1), // Claw
    (2_040_002, 1), // Hat Accuracy Scroll: Greater
    (2_040_502, 1), // Overall Armor DEF Scroll: Greater
    (2_040_602, 1), // Bottomwear DEF Scroll: Greater
    (2_040_506, 1), // Overall Armor STR Scroll: Greater
];

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

/// One slot of the box: what the drops page calls it, and its lines.
pub struct Slot {
    pub name: &'static str,
    pub prizes: &'static [Prize],
}

/// **The four slots, in the order a box hands them over.** One prize from each, every box.
pub const SLOTS: [Slot; 4] = [
    Slot { name: "equip", prizes: &EQUIPS },
    Slot { name: "use", prizes: &USE },
    Slot { name: "scroll", prizes: &SCROLLS },
    Slot { name: "etc", prizes: &ETC },
];

/// **One slot's prize** from one roll: a line of that slot, equal odds.
pub fn roll(slot: &Slot, r: u64) -> Prize {
    slot.prizes[usize::try_from(r % slot.prizes.len() as u64).unwrap_or(0)]
}

/// **What Lakelis says when a box is opened** (the owner, 2026-10-03: *"a relevant NPC chat
/// popup should open showing what they got in each slot. The NPC chat should be from
/// Lakelis."*). One line per slot that was given, icon and name, and the amount when it is
/// more than one. The breaks are the two characters `\` `n`, which is what this client's
/// renderer turns into a line break - a real newline byte is dropped (`crate::scrollnpc`).
pub fn lakelis_text(given: &[(&str, Prize)]) -> String {
    let mut text = String::from(r"You opened your #bCompanion's Magic Box#k! Here is what was inside, one from every slot:\n");
    for &(slot, (id, qty)) in given {
        let mut label = slot.to_string();
        if let Some(c) = label.get_mut(..1) {
            c.make_ascii_uppercase();
        }
        let amount = if qty > 1 { format!(" x{qty}") } else { String::new() };
        text.push_str(&format!(r"\n#b{label}#k: #i{id}# #t{id}#{amount}"));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The owner's quantities: every ore 8, the Screw 10, the Use list as written on
    /// 2026-10-03, every equip and scroll one.
    #[test]
    fn the_quantities_are_the_owners() {
        for (id, qty) in ETC {
            let want = if id == 4_003_000 { 10 } else { 8 };
            assert_eq!(qty, want, "{id}");
        }
        assert_eq!(USE[..8], [
            (2_000_001, 100),
            (2_000_002, 100),
            (2_000_003, 100),
            (2_000_006, 100),
            (2_000_004, 20),
            (2_022_000, 20),
            (2_012_002, 10),
            (2_012_001, 10),
        ]);
        assert_eq!(USE[8..10], [(2_020_000, 1), (2_020_001, 1)], "Salad and Fried Chicken: no quantity given");
        assert!(USE[10..].iter().all(|&(_, q)| q == 50));
        assert_eq!(USE[10..].iter().map(|&(id, _)| id).collect::<Vec<_>>(), (2_020_002..=2_020_011).collect::<Vec<_>>());
        assert!(EQUIPS.iter().all(|&(_, q)| q == 1));
        assert!(SCROLLS.iter().all(|&(_, q)| q == 1));
    }

    /// **One of each kind, every box.** Each slot holds only its own kind, so four rolls are
    /// four different tabs' worth - and every line of every slot is reachable, no id twice.
    #[test]
    fn every_slot_is_its_own_kind_and_every_line_is_reachable() {
        let kind = |id: u32| match id {
            1_000_000..=1_999_999 => "equip",
            2_040_000..=2_049_999 => "scroll",
            2_000_000..=2_999_999 => "use",
            _ => "etc",
        };
        assert_eq!(SLOTS.map(|s| s.name), ["equip", "use", "scroll", "etc"]);
        let mut seen = std::collections::HashSet::new();
        for slot in &SLOTS {
            for r in 0..slot.prizes.len() as u64 {
                let (id, _) = roll(slot, r);
                assert_eq!(kind(id), slot.name, "{id}");
                assert!(seen.insert(id), "{id} twice");
            }
            assert_eq!(roll(slot, u64::MAX), slot.prizes[(u64::MAX % slot.prizes.len() as u64) as usize]);
        }
        assert_eq!(seen.len(), 7 + 20 + 36 + 14);
    }

    /// Lakelis names every slot that was given, with the icon, the name and the amount - and
    /// the breaks are the escape the client renders, not newline bytes.
    #[test]
    fn lakelis_says_what_each_slot_gave() {
        let t = lakelis_text(&[("equip", EQUIPS[0]), ("use", USE[0]), ("scroll", SCROLLS[0]), ("etc", ETC[13])]);
        assert!(t.contains(r"\n#bEquip#k: #i1002083# #t1002083#\n"), "{t}");
        assert!(t.contains(r"#bUse#k: #i2000001# #t2000001# x100"), "{t}");
        assert!(t.contains(r"#bScroll#k: #i2040801# #t2040801#\n"), "{t}");
        assert!(t.ends_with(r"#bEtc#k: #i4003000# #t4003000# x10"), "{t}");
        assert!(!t.contains('\n') && !t.contains('\r'), "no real newline bytes");
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
        let mut have: Vec<u32> = SCROLLS[..ATTACK_SCROLLS].iter().map(|&(id, _)| id).collect();
        have.sort_unstable();
        assert_eq!(have, want);
    }

    /// Every id is the item its comment names - checked against `gm-handbook/items.txt`, so
    /// a typo in a seven-digit number fails here rather than handing out the wrong thing.
    /// The 2026-10-03 Use ids are checked to exist; their names are the owner's words and are
    /// not pinned to the client's spelling.
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
        for slot in &SLOTS {
            for &(id, _) in slot.prizes {
                assert!(name(id).is_some(), "{id} is not an item in this client");
            }
        }
    }
}
