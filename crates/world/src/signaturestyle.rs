//! **The Signature Style Collection** - what the box and each Outfit Set Coupon hand out.
//!
//! The owner, 2026-09-10: *"Signature Style Collection is a cash item that when used, allows the
//! player to select one of the following items: Frieren Outfit Set, Fern Outfit Set, Stark
//! Outfit Set, Übel Outfit Set, Himmel Outfit Set, Aura Outfit Set, Lügner Outfit Set, or
//! Linie Outfit Set"* - and then the same evening: *"Can we modify the Signature Style
//! Collection coupon to be instead of obtaining 1 at random rates, we give them all of the
//! sets for 8000 LP."* So the box hands out **all eight** set coupons, and each set coupon
//! hands out its set: the equips, plus a hair coupon per hairstyle and a face coupon.
//!
//! # This module is the rule and nothing else
//!
//! It touches no store and no packet. Every id below was read out of the modern client's
//! `String.wz` by `tools/backport_signature_style.py` and is in
//! `backport/signature-style/manifest.md`; the assets themselves are installed in
//! `client-patched/Data` by `tools/backport_install.py`. A test checks every id here against
//! the hybrid `gm-handbook/items.txt`, so a set that names an item the client cannot draw
//! fails in the suite rather than in a player's bag.
//!
//! # Why the contents are here and not in the WZ
//!
//! In the modern client every layer is `spec/script = cash_NNN` with `notConsume 1`: the
//! client asks the server to run a script and the server decides. No package table names
//! these ids. The owner's listing is that script, and it differs from the modern game in two
//! ways they chose: one Frieren set carrying all three hairstyles and all three outfits
//! (the modern client has three Frieren coupons and "selector" coupons), and a box that
//! gives everything rather than one at random.

/// The box. Cash item, sold in the Special tab.
pub const COLLECTION: u32 = 5_222_221;

/// One set: the coupon that opens it and what comes out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutfitSet {
    /// The Outfit Set Coupon, a Cash item.
    pub coupon: u32,
    pub name: &'static str,
    /// Hair coupons (`Use` tab items whose `spec/cosmetic` names the hairstyle). Aura, Linie
    /// and Lügner have none: their hair is a hat, listed under `equips`.
    pub hair_coupons: &'static [u32],
    /// The face coupon.
    pub face_coupon: u32,
    /// Equips, in the order the owner listed them.
    pub equips: &'static [u32],
}

/// The eight sets, in the order the box hands them out.
pub const SETS: [OutfitSet; 8] = [
    OutfitSet {
        coupon: 5_681_543,
        name: "Frieren",
        hair_coupons: &[2_543_137, 2_543_138, 2_543_139], // Hair, Hair (Ringlets), Hair (Sleep)
        face_coupon: 2_897_007,
        // Clothes, Winter Clothes, Sleep Clothes, Shoes, Earrings, Staff
        equips: &[1_054_555, 1_054_556, 1_054_557, 1_074_234, 1_032_360, 1_703_722],
    },
    OutfitSet {
        coupon: 5_681_547,
        name: "Fern",
        hair_coupons: &[2_543_140],
        face_coupon: 2_897_008,
        // Clothes, Winter Clothes, Shoes, Staff
        equips: &[1_054_558, 1_054_559, 1_074_235, 1_703_723],
    },
    OutfitSet {
        coupon: 5_681_549,
        name: "Stark",
        hair_coupons: &[2_543_141],
        face_coupon: 2_897_009,
        // Clothes, Winter Clothes, Shoes, Winter Shoes, Gloves, Winter Gloves, Axe
        equips: &[1_054_560, 1_054_595, 1_074_236, 1_074_263, 1_082_877, 1_082_882, 1_703_724],
    },
    OutfitSet {
        coupon: 5_681_548,
        name: "\u{dc}bel",
        hair_coupons: &[2_543_143],
        face_coupon: 2_897_011,
        // Clothes, Shoes, Gloves, Staff
        equips: &[1_054_562, 1_074_238, 1_082_878, 1_703_726],
    },
    OutfitSet {
        coupon: 5_681_546,
        name: "Himmel",
        hair_coupons: &[2_543_142],
        face_coupon: 2_897_010,
        // Clothes, Shoes, Sword, Himmel's Blessing (cape)
        equips: &[1_054_561, 1_074_237, 1_703_725, 1_103_918],
    },
    OutfitSet {
        coupon: 5_681_550,
        name: "Aura",
        hair_coupons: &[],
        face_coupon: 2_897_012,
        // Hair (Hat), Clothes, Shoes, Gloves, Scales of Obedience
        equips: &[1_006_910, 1_054_563, 1_074_239, 1_082_879, 1_703_727],
    },
    OutfitSet {
        coupon: 5_681_552,
        name: "L\u{fc}gner",
        hair_coupons: &[],
        face_coupon: 2_897_014,
        // Hair (Hat), Clothes, Shoes
        equips: &[1_006_912, 1_054_565, 1_074_241],
    },
    OutfitSet {
        coupon: 5_681_551,
        name: "Linie",
        hair_coupons: &[],
        face_coupon: 2_897_013,
        // Hair (Hat), Clothes, Shoes
        equips: &[1_006_911, 1_054_564, 1_074_240],
    },
];

/// The set a coupon opens, if it is one.
pub fn set_for_coupon(item_id: u32) -> Option<&'static OutfitSet> {
    SETS.iter().find(|s| s.coupon == item_id)
}

/// Everything one set hands out, as `(item id, inventory tab)`, in the order it is given.
///
/// Use-tab first (the coupons), then the equips - so a bag that is full refuses before a
/// single row is written, and the caller can count per tab.
pub fn set_contents(set: &OutfitSet) -> Vec<(u32, store::InventoryType)> {
    let mut out: Vec<(u32, store::InventoryType)> = Vec::new();
    for &c in set.hair_coupons {
        out.push((c, store::InventoryType::Use));
    }
    out.push((set.face_coupon, store::InventoryType::Use));
    for &e in set.equips {
        out.push((e, store::InventoryType::Equip));
    }
    out
}

/// How many slots of each tab a hand-out needs, indexed by `InventoryType::index()`.
pub fn slots_needed(items: &[(u32, store::InventoryType)]) -> [u16; net::opcode::INVENTORY_COUNT] {
    let mut out = [0u16; net::opcode::INVENTORY_COUNT];
    for (_, t) in items {
        out[t.index()] += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The owner's three spelled-out sets, item for item, against their listing.
    #[test]
    fn the_three_sets_wisp_listed_match_his_listing() {
        let frieren = set_for_coupon(5_681_543).unwrap();
        assert_eq!(frieren.hair_coupons.len() + 1 + frieren.equips.len(), 10, "ten items");
        assert!(frieren.equips.contains(&1_032_360), "Frieren's Earrings");
        assert!(frieren.equips.contains(&1_703_722), "Frieren's Staff");
        let uebel = set_for_coupon(5_681_548).unwrap();
        assert_eq!(uebel.hair_coupons.len() + 1 + uebel.equips.len(), 6, "six items");
        let stark = set_for_coupon(5_681_549).unwrap();
        assert_eq!(stark.hair_coupons.len() + 1 + stark.equips.len(), 9, "nine items");
        assert!(stark.equips.contains(&1_082_882), "Stark's Winter Gloves");
        assert!(stark.equips.contains(&1_074_263), "Stark's Winter Shoes");
    }

    /// No id appears twice across the eight sets, and the eight coupons are distinct - a
    /// copied line would hand somebody two of one thing and none of another.
    #[test]
    fn every_id_is_handed_out_by_exactly_one_set() {
        let mut seen = std::collections::HashSet::new();
        for set in &SETS {
            assert!(seen.insert(set.coupon), "coupon {} twice", set.coupon);
            for (id, _) in set_contents(set) {
                assert!(seen.insert(id), "{id} is in two sets");
            }
        }
        assert_eq!(set_for_coupon(COLLECTION), None, "the box is not a set");
        assert_eq!(set_for_coupon(5_681_544), None, "the Ringlets coupon is not sold or opened here");
    }

    /// **Every id here has a name in the hybrid client** - `gm-handbook/items.txt`, generated
    /// from the installed `String.wz`. Skipped when the handbook is absent; when it is
    /// present, an id with no name is one the client cannot draw and this fails.
    #[test]
    fn every_id_has_a_name_in_the_hybrid_client() {
        let path = std::path::Path::new("../../gm-handbook/items.txt");
        let Ok(text) = std::fs::read_to_string(path) else { return };
        let names: std::collections::HashMap<u32, String> = text
            .lines()
            .filter(|l| !l.starts_with('#'))
            .filter_map(|l| {
                let (id, name) = l.split_once(", ")?;
                Some((id.trim().parse().ok()?, name.to_string()))
            })
            .collect();
        if !names.contains_key(&COLLECTION) {
            return; // the handbook predates the backport; nothing to check against
        }
        for set in &SETS {
            let coupon_name = names.get(&set.coupon).unwrap_or_else(|| panic!("{} unnamed", set.coupon));
            assert!(coupon_name.starts_with(set.name), "{coupon_name} is not {}'s coupon", set.name);
            for (id, _) in set_contents(set) {
                let n = names.get(&id).unwrap_or_else(|| panic!("{} ({}) has no name", id, set.name));
                assert!(
                    n.starts_with(set.name),
                    "{id} {n:?} does not belong to {}",
                    set.name
                );
            }
        }
    }

    #[test]
    fn slot_accounting_counts_per_tab() {
        let frieren = set_for_coupon(5_681_543).unwrap();
        assert_eq!(slots_needed(&set_contents(frieren)), [6, 4, 0, 0, 0, 0], "Equip, Use, Set Up, Etc, Cash, Deco - the listing's tabs; the hand-out re-tabs cash equips to Deco");
        let aura = set_for_coupon(5_681_550).unwrap();
        assert_eq!(slots_needed(&set_contents(aura)), [5, 1, 0, 0, 0, 0]);
    }
}
