//! **Item variance: an equip dropped by a mob rolls its stats around the clean template.**
//!
//! The owner, 2026-09-24, with a rules sheet: *"I want to introduce item variance for any items
//! dropped by mobs following these rules (including those dropped by party quests such as
//! Slime Shoes)."* The rules, as the sheet states them:
//!
//! 1. **Range = required level / 10.** An overall doubles it, because it replaces a top and a
//!    bottom. A level-0 item has range 0 and never varies.
//! 2. **Each stat rolls on its own**: down, unchanged or up with equal odds, then a
//!    fine-grained random decides how far toward that stat's cap.
//! 3. **Round, then floor at zero.**
//!
//! **Only a stat the clean template already has can roll** - a base of 0 stays 0, so an item
//! never gains a line it did not have. The cap per stat is the range times a multiplier:
//!
//! | stat | max swing |
//! |---|---|
//! | STR, DEX, INT, LUK | range / how many of those four the item has |
//! | WATK, MATK, Speed | range x 1/2 |
//! | ACC, Avoid | range x 1 |
//! | Jump | range x 1/4 |
//! | HP, MP, WDEF, MDEF | range x 5 |
//!
//! Mapped onto this client's fields: WATK is `incWAT` (bit 16 - this client's data has no
//! `incPAD` anywhere, see `EquipTemplate::fresh_stats`), MATK `incMAD`, WDEF / MDEF
//! `incPDD` / `incMDD`, Avoid `incEVA`. Critical rate and damage are not on the sheet and do
//! not move.
//!
//! The result is the item's **absolute** stat block (`ItemKind::Equip(Some(..))`), which is
//! what the packet carries and what a Chaos scroll already writes - a value below the
//! template is a shape the client has been drawing since Chaos shipped. The upgrade slots
//! are the template's, untouched.
//!
//! **The roll is the item's own base from then on** (`store::Item::rolled_base`, the owner
//! 2026-09-25: *"Can we make Innocence Scrolls keep a good base roll?"*): Innocence reverts a
//! rolled drop to its roll, and anything that never rolled to the template.

use crate::config::EquipTemplate;

/// An overall's item-id family (`105xxxx`): one piece that replaces a top and a bottom.
pub const OVERALL_FAMILY: u32 = 105;

/// Rule 1: the range, before any per-stat multiplier.
pub fn range(req_level: u16, item_id: u32) -> f64 {
    let r = f64::from(req_level) / 10.0;
    if item_id / 10_000 == OVERALL_FAMILY { r * 2.0 } else { r }
}

/// One stat's roll. `direction` is `0` down, `1` unchanged, `2` up; `fraction` is how far
/// toward the cap, in `0.0..=1.0`. Returns the new value - rounded, floored at zero, and
/// untouched when the base is zero.
pub fn roll_one(base: u16, cap: f64, direction: u64, fraction: f64) -> u16 {
    if base == 0 {
        return 0;
    }
    let delta = cap * fraction;
    let v = match direction % 3 {
        0 => f64::from(base) - delta,
        1 => f64::from(base),
        _ => f64::from(base) + delta,
    };
    v.round().clamp(0.0, f64::from(u16::MAX)) as u16
}

/// A fraction in `0.0..=1.0` from 53 random bits - the "fine-grained random".
fn fraction(bits: u64) -> f64 {
    const ONE: u64 = (1 << 53) - 1;
    (bits >> 11) as f64 / ONE as f64
}

/// Roll a fresh copy of `template` (item `item_id`). Two random draws per stat the item has
/// - direction, then distance - and none for a stat it does not.
pub fn roll(template: &EquipTemplate, item_id: u32, next: &mut dyn FnMut() -> u64) -> net::opcode::EquipStats {
    let mut out = template.fresh_stats();
    let range = range(template.req_level, item_id);
    let s = &mut out.stats;
    let primes = [s.inc_str, s.inc_dex, s.inc_int, s.inc_luk].iter().filter(|&&v| v > 0).count().max(1) as f64;
    let prime_cap = range / primes;
    let mut go = |v: &mut u16, cap: f64| {
        if *v == 0 {
            return;
        }
        let direction = next();
        let how_far = fraction(next());
        *v = roll_one(*v, cap, direction, how_far);
    };
    go(&mut s.inc_str, prime_cap);
    go(&mut s.inc_dex, prime_cap);
    go(&mut s.inc_int, prime_cap);
    go(&mut s.inc_luk, prime_cap);
    go(&mut s.inc_wat, range * 0.5);
    go(&mut s.inc_pad, range * 0.5);
    go(&mut s.inc_mad, range * 0.5);
    go(&mut s.inc_speed, range * 0.5);
    go(&mut s.inc_acc, range);
    go(&mut s.inc_eva, range);
    go(&mut s.inc_jump, range * 0.25);
    go(&mut s.inc_mhp, range * 5.0);
    go(&mut s.inc_mmp, range * 5.0);
    go(&mut s.inc_pdd, range * 5.0);
    go(&mut s.inc_mdd, range * 5.0);
    out
}

/// The item a mob drop should carry: a rolled copy of an equip with a template, the item
/// unchanged otherwise (a bundle, a cash equip, or an id with no template - nothing to roll
/// around, so the wire edge derives it as before).
pub fn for_mob_drop(equips: &std::collections::HashMap<u32, EquipTemplate>, item: store::Item, next: &mut dyn FnMut() -> u64) -> store::Item {
    if item.kind != store::ItemKind::Equip(None) {
        return item;
    }
    match equips.get(&item.item_id) {
        Some(t) if !t.cash => {
            let rolled = roll(t, item.item_id, next);
            // The roll is also the item's own base: an Innocence reverts to it, not to the
            // template (`store::Item::rolled_base`, the owner 2026-09-24).
            store::Item { kind: store::ItemKind::Equip(Some(rolled)), rolled_base: Some(rolled.stats), ..item }
        }
        _ => item,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A counter-based stand-in for the session's generator, so every test is reproducible.
    fn seq(seed: u64) -> impl FnMut() -> u64 {
        let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
        move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        }
    }

    /// The sheet's own examples: a Lv 60 hat has range 6, a Lv 120 coat 12, and an overall
    /// doubles.
    #[test]
    fn the_range_is_the_level_over_ten_and_an_overall_doubles_it() {
        assert_eq!(range(60, 1_002_000), 6.0);
        assert_eq!(range(120, 1_040_000), 12.0);
        assert_eq!(range(15, 1_050_000), 3.0, "Beige Plain Robe is an overall");
        assert_eq!(range(0, 1_302_000), 0.0);
    }

    #[test]
    fn one_stat_goes_down_stays_or_goes_up_and_is_rounded_and_floored() {
        assert_eq!(roll_one(10, 4.0, 0, 1.0), 6);
        assert_eq!(roll_one(10, 4.0, 1, 1.0), 10, "unchanged ignores the distance");
        assert_eq!(roll_one(10, 4.0, 2, 1.0), 14);
        assert_eq!(roll_one(10, 4.0, 2, 0.4), 12, "11.6 rounds to 12");
        assert_eq!(roll_one(3, 30.0, 0, 1.0), 0, "floored at zero, never wrapped");
        assert_eq!(roll_one(0, 30.0, 2, 1.0), 0, "a stat the item does not have stays absent");
    }

    fn template() -> EquipTemplate {
        // Squishy Shoes, as gm-handbook/equips.txt has them: Lv 28, +1 to all four primes,
        // 18 WDEF, 7 MDEF, 5 slots.
        EquipTemplate { tuc: 5, inc_str: 1, inc_dex: 1, inc_int: 1, inc_luk: 1, inc_pdd: 18, inc_mdd: 7, req_level: 28, ..Default::default() }
    }

    /// Every stat stays inside its cap over many rolls, every direction happens, and nothing
    /// the template lacks ever appears. Range 2.8: primes share it four ways (0.7 each),
    /// WDEF and MDEF get 14.
    #[test]
    fn many_rolls_stay_inside_every_cap_and_add_no_new_lines() {
        let t = template();
        let mut next = seq(7);
        let (mut lower, mut same, mut higher) = (0, 0, 0);
        for _ in 0..3_000 {
            let s = roll(&t, 1_072_128, &mut next).stats;
            for v in [s.inc_str, s.inc_dex, s.inc_int, s.inc_luk] {
                assert!(v <= 2, "1 + 0.7 rounds to at most 2: {v}");
            }
            assert!((4..=32).contains(&s.inc_pdd), "18 +/- 14: {}", s.inc_pdd);
            assert!(s.inc_mdd <= 21, "7 + 14, floored at 0 below: {}", s.inc_mdd);
            assert_eq!((s.inc_wat, s.inc_mad, s.inc_mhp, s.inc_acc, s.inc_jump, s.inc_speed), (0, 0, 0, 0, 0, 0));
            match s.inc_pdd.cmp(&18) {
                std::cmp::Ordering::Less => lower += 1,
                std::cmp::Ordering::Equal => same += 1,
                std::cmp::Ordering::Greater => higher += 1,
            }
        }
        // Equal thirds by direction; "unchanged" also collects the rolls that moved less
        // than half a point, so it is a little over a third.
        for (what, n) in [("lower", lower), ("same", same), ("higher", higher)] {
            assert!((800..=1_300).contains(&n), "{what}: {n} of 3000");
        }
    }

    /// The per-stat multipliers, one stat at a time at a round range of 10 (Lv 100), each
    /// driven to its cap upward.
    #[test]
    fn each_stat_class_has_the_sheets_multiplier() {
        let up = |t: EquipTemplate| {
            // direction 2 (up), then the largest fraction, for every draw.
            let mut n = 0u64;
            let mut next = move || {
                n += 1;
                if n % 2 == 1 { 2 } else { u64::MAX }
            };
            roll(&t, 1_002_000, &mut next).stats
        };
        let base = EquipTemplate { req_level: 100, ..Default::default() };
        assert_eq!(up(EquipTemplate { inc_wat: 50, ..base }).inc_wat, 55, "WATK: range x 1/2");
        assert_eq!(up(EquipTemplate { inc_mad: 50, ..base }).inc_mad, 55, "MATK: range x 1/2");
        assert_eq!(up(EquipTemplate { inc_speed: 10, ..base }).inc_speed, 15, "Speed: range x 1/2");
        assert_eq!(up(EquipTemplate { inc_acc: 10, ..base }).inc_acc, 20, "ACC: range x 1");
        assert_eq!(up(EquipTemplate { inc_eva: 10, ..base }).inc_eva, 20, "Avoid: range x 1");
        assert_eq!(up(EquipTemplate { inc_jump: 10, ..base }).inc_jump, 13, "Jump: range x 1/4 = 2.5, rounds to 3");
        assert_eq!(up(EquipTemplate { inc_mhp: 10, ..base }).inc_mhp, 60, "HP: range x 5");
        assert_eq!(up(EquipTemplate { inc_mmp: 10, ..base }).inc_mmp, 60, "MP: range x 5");
        assert_eq!(up(EquipTemplate { inc_pdd: 10, ..base }).inc_pdd, 60, "WDEF: range x 5");
        assert_eq!(up(EquipTemplate { inc_mdd: 10, ..base }).inc_mdd, 60, "MDEF: range x 5");
        assert_eq!(up(EquipTemplate { inc_str: 10, ..base }).inc_str, 20, "one prime: the whole range");
        let two = up(EquipTemplate { inc_str: 10, inc_luk: 10, ..base });
        assert_eq!((two.inc_str, two.inc_luk), (15, 15), "two primes share it: range / 2 each");
        assert_eq!(up(EquipTemplate { inc_crt: 10, ..base }).inc_crt, 10, "crit is not on the sheet");
        assert_eq!(up(EquipTemplate { tuc: 7, inc_wat: 50, ..base }).inc_pad, 0, "incPAD is never invented");
        assert_eq!(roll(&EquipTemplate { tuc: 7, ..base }, 1_002_000, &mut seq(1)), EquipTemplate { tuc: 7, ..base }.fresh_stats(), "no stats, nothing moves - the slots are the template's");
    }

    #[test]
    fn a_level_zero_item_never_varies() {
        let sword = EquipTemplate { tuc: 7, inc_wat: 17, ..Default::default() };
        let mut next = seq(3);
        for _ in 0..200 {
            assert_eq!(roll(&sword, 1_302_000, &mut next), sword.fresh_stats());
        }
    }

    #[test]
    fn only_a_fresh_non_cash_equip_with_a_template_is_rolled() {
        let mut equips = std::collections::HashMap::new();
        equips.insert(1_072_128, template());
        equips.insert(1_702_000, EquipTemplate { cash: true, inc_wat: 5, req_level: 50, ..Default::default() });
        let mut next = seq(9);
        let shoes = for_mob_drop(&equips, store::Item::equip(1_072_128), &mut next);
        let store::ItemKind::Equip(Some(rolled)) = shoes.kind else { panic!("rolled: {shoes:?}") };
        assert_eq!(shoes.rolled_base, Some(rolled.stats), "the roll is remembered as the item's own base");
        assert_eq!(for_mob_drop(&equips, store::Item::equip(1_702_000), &mut next), store::Item::equip(1_702_000), "cash");
        assert_eq!(for_mob_drop(&equips, store::Item::equip(1_999_999), &mut next), store::Item::equip(1_999_999), "no template");
        assert_eq!(for_mob_drop(&equips, store::Item::bundle(4_001_002, 1), &mut next), store::Item::bundle(4_001_002, 1));
    }
}
