//! **The character-select stat sheet shows equipment totals**, the way the in-game window does.
//!
//! The owner, 2026-09-10, with both windows side by side: *"I also feel like the stat screen on
//! character select should reflect all equipment bonuses like our current character stat
//! window."* Their in-game window read STR 1026 / DEX 1006 / INT 1073 / LUK 1003 and HP 517;
//! the select screen read 27 / 5 / 74 / 4 and 414 - the bare row.
//!
//! # Why the two screens differ, and why this is the login server's job
//!
//! In the field the client is handed every worn item's stat block inside the character
//! record and **does the addition itself** - the world server never sums anything. At
//! character select the client is handed a stat block and a *look*, and a look carries item
//! ids only, so there is nothing for it to add. The only place the totals can come from is
//! the stat block the login server writes, so the login server does the sum the client
//! would have done.
//!
//! # Matching the client's arithmetic exactly
//!
//! The client adds what the record carries, and the world fills the record with the **stored**
//! stat block where one exists and the **template's** fresh stats where none does
//! (`EquipTemplate::fresh_stats`, the same path `!scroll` and the record use). So this does
//! the same: stored first, template second, nothing when neither exists. A worn item whose
//! template is missing contributes nothing rather than guessing, and the sheet is then short
//! by that item's base - which the log line says.
//!
//! **Display only.** The world's `SetField` still carries the bare row; the client adds the
//! equipment there itself and would double it if the base were inflated. This copy never
//! reaches a database.

use std::collections::HashMap;

use net::opcode::Character;
use store::inventory::EquippedItem;
use world::config::EquipTemplate;

/// What the sheet gained over the bare row, for the log.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Bonus {
    pub str: u32,
    pub dex: u32,
    pub int: u32,
    pub luk: u32,
    pub max_hp: u32,
    pub max_mp: u32,
    /// Worn items that had no stored block and no template - their base is missing from the
    /// sheet. Zero on the owner's data; non-zero means `gm-handbook/equips.txt` did not load.
    pub unresolved: u32,
}

/// Sum every worn item's bonus the way the field client would.
pub fn bonus(worn: &[EquippedItem], templates: &HashMap<u32, EquipTemplate>) -> Bonus {
    let mut b = Bonus::default();
    for item in worn {
        let stats = match item.stats {
            Some(s) => s.stats,
            None => match templates.get(&item.item_id) {
                Some(t) => t.fresh_stats().stats,
                None => {
                    b.unresolved += 1;
                    continue;
                }
            },
        };
        b.str += u32::from(stats.inc_str);
        b.dex += u32::from(stats.inc_dex);
        b.int += u32::from(stats.inc_int);
        b.luk += u32::from(stats.inc_luk);
        b.max_hp += u32::from(stats.inc_mhp);
        b.max_mp += u32::from(stats.inc_mmp);
    }
    b
}

/// The character as the select sheet should show it: the bare row plus [`bonus`].
///
/// Saturating into the `u16` stat fields - a +999 on each of five items already reaches
/// 5,000, and a scrolled-up character must not wrap to a small number.
pub fn for_select(chr: &Character, b: Bonus) -> Character {
    let add16 = |base: u16, extra: u32| u16::try_from(u32::from(base) + extra).unwrap_or(u16::MAX);
    let add32 = |base: u32, extra: u32| base.saturating_add(extra);
    Character {
        strength: add16(chr.strength, b.str),
        dexterity: add16(chr.dexterity, b.dex),
        intelligence: add16(chr.intelligence, b.int),
        luck: add16(chr.luck, b.luk),
        max_hp: add32(chr.max_hp, b.max_hp),
        max_mp: add32(chr.max_mp, b.max_mp),
        ..chr.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use net::opcode::{EquipStatSet, EquipStats};

    fn worn(slot: u8, item_id: u32, stats: Option<EquipStats>) -> EquippedItem {
        EquippedItem { slot, item_id, stats }
    }

    fn scrolled(inc_str: u16, inc_dex: u16, inc_mhp: u16) -> EquipStats {
        EquipStats {
            stats: EquipStatSet { inc_str, inc_dex, inc_mhp, ..EquipStatSet::default() },
            ..EquipStats::default()
        }
    }

    /// **Stored block first, template second, nothing when neither exists** - the world's own
    /// order for filling the record, so the sheet and the in-game window agree.
    #[test]
    fn stored_stats_win_and_templates_fill_the_rest() {
        let mut templates = HashMap::new();
        templates.insert(1040021, EquipTemplate { inc_dex: 2, inc_pdd: 45, tuc: 7, ..Default::default() });
        let items = [
            worn(5, 1040021, Some(scrolled(999, 999, 0))), // scrolled: the stored block, NOT +2
            worn(6, 1062999, Some(scrolled(0, 0, 103))),   // stored, HP only
            worn(7, 1040021, None),                         // unscrolled: the template's +2 DEX
            worn(11, 1322999, None),                        // no block, no template
        ];
        let b = bonus(&items, &templates);
        assert_eq!(b.str, 999);
        assert_eq!(b.dex, 999 + 2, "the stored block replaces the template, it does not add to it");
        assert_eq!(b.max_hp, 103);
        assert_eq!(b.unresolved, 1, "the suitcase had nothing to say and is counted, not guessed");
    }

    /// The owner's numbers: base 27/5/74/4, HP 414, plus what their scrolls added.
    #[test]
    fn the_sheet_is_base_plus_bonus_and_never_wraps() {
        let cobalt = Character {
            strength: 27,
            dexterity: 5,
            intelligence: 74,
            luck: 4,
            max_hp: 414,
            max_mp: 283,
            ..Default::default()
        };
        let shown = for_select(
            &cobalt,
            Bonus { str: 999, dex: 1001, int: 999, luk: 999, max_hp: 103, ..Default::default() },
        );
        assert_eq!((shown.strength, shown.dexterity, shown.intelligence, shown.luck), (1026, 1006, 1073, 1003));
        assert_eq!(shown.max_hp, 517, "the in-game window's 517/517");
        assert_eq!(shown.max_mp, 283, "untouched when nothing adds to it");
        // The bare row is not modified: this is a copy for the wire.
        assert_eq!(cobalt.strength, 27);
        // Saturation, not wraparound.
        let big = for_select(&cobalt, Bonus { str: 70_000, ..Default::default() });
        assert_eq!(big.strength, u16::MAX);
    }

    /// Nothing worn, nothing added - a fresh character's sheet is the bare row.
    #[test]
    fn no_equipment_means_no_change() {
        let chr = Character { strength: 12, ..Default::default() };
        let b = bonus(&[], &HashMap::new());
        assert_eq!(b, Bonus::default());
        assert_eq!(for_select(&chr, b).strength, 12);
    }
}
