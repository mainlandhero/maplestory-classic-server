//! **Which second- and third-job buffs ride which temporary-stat bit, and what goes in the
//! value slot.** The pure table; `session::buff::table_buff_level` is the one reader.
//!
//! The owner, 2026-09-07: *"take a look at all of the classes' 2nd and 3rd job skills, make sure
//! that most if not all of them are implemented correctly."* The audit itself is
//! `research/second-third-job-audit-2026-09-07.md`; this file is the part of its answer that
//! is a table.
//!
//! # Why a table, and why it is not the whole answer
//!
//! `session::buff::table_buff_level` already builds a buff from any row whose grants are flat
//! `indie*` columns - that is how Haste, Rage, Meditation and Iron Will work without a line of
//! per-skill code. **Twenty-nine more buffs in the census carry their effect in `x`, or in no
//! column at all**, because the effect is a *behaviour* the client switches on when it sees the
//! bit: a Booster is faster swings, Soul Arrow is free shots, a Charge is an element on the
//! blade. For those the generated table cannot say which bit; this file does, one row per
//! skill, each with the standing of the bit it names (`net::jobbuffs`).
//!
//! What this file does **not** do is the server's half of any of them. Power Guard's
//! reflection, Meso Guard's mesos, Holy Symbol's experience, Hyper Body's ceiling, Dragon
//! Blood's drain and Combo's orbs are arithmetic on numbers the server owns, and they live
//! beside the numbers they change (`session::combat`, `session::pools`, `session::buff`).
//! Granting the bit without that half is exactly the failure `session::buff::magic_guard_percent`
//! documents - *"setting the bit buys an icon"* - so every row here whose effect is the server's
//! names the function that does it, and the audit lists the ones that have none.
//!
//! # The value slot
//!
//! Three conventions, and the census is the reason there are three rather than one:
//!
//! * [`ValueFrom::X`] - the row's `x`, where the tooltip gives it a unit the client can use
//!   directly: Booster's *"by 2 stages"* (`-2`), Power Guard's *"return 20%"*, Holy Symbol's
//!   percent, Meso Guard's percent. **[L]** for the unit, from the tooltip beside the number.
//! * [`ValueFrom::Level`] - the skill level, for the flags: Soul Arrow, Shadow Partner, the
//!   Charges, Final Attack, Dragon Blood, Element Amplification. The client reads the skill's
//!   own table at that level for anything numeric it needs. **[I]** - it is the convention
//!   every reference server uses for these, and the one measured reader of a flag bit here
//!   (Dark Sight, `net::jobbuffs::CTS_DARK_SIGHT`) tests for non-zero.
//! * [`ValueFrom::One`] - Combo Attack starts at one orb-plus-one; `session::combat` raises it
//!   per hit and Coma/Panic put it back. **[I]**, same source.

use crate::firstjob::CastNumbers;

/// Where a flag buff's value comes from. See the module docs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueFrom {
    /// The row's `x`, as the WZ writes it (sign included).
    X,
    /// The skill level that was cast.
    Level,
    /// The literal `1`.
    One,
}

impl ValueFrom {
    /// Resolve against a row and the level cast. `None` when the row lacks the column it
    /// needs, which refuses the grant rather than sending a zero the client would read as a
    /// value.
    pub fn resolve(self, row: &CastNumbers, level: u32) -> Option<i16> {
        match self {
            ValueFrom::X => row.x.and_then(|x| i16::try_from(x).ok()),
            ValueFrom::Level => i16::try_from(level).ok(),
            ValueFrom::One => Some(1),
        }
    }
}

/// One buff whose bit the generated table cannot name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlagBuff {
    pub bit: u32,
    pub value: ValueFrom,
    /// `processtype 113` in the archive: no `time`, switched off by casting again or by the
    /// right-click. A toggle is held until then - `session::buff` gives it no expiry.
    pub toggle: bool,
}

const fn timed(bit: u32, value: ValueFrom) -> FlagBuff {
    FlagBuff { bit, value, toggle: false }
}
const fn toggle(bit: u32, value: ValueFrom) -> FlagBuff {
    FlagBuff { bit, value, toggle: true }
}

/// The ten weapon boosters: every second-job book but the Magician's has one per weapon.
pub const BOOSTERS: [u32; 10] = [
    1_101_002, 1_101_003, 1_201_002, 1_201_003, 1_301_002, 1_301_003, 3_101_001, 3_201_001,
    4_101_000, 4_201_000,
];
/// The eight Final Attack toggles.
pub const FINAL_ATTACKS: [u32; 8] =
    [1_101_000, 1_101_001, 1_201_000, 1_201_001, 1_301_000, 1_301_001, 3_101_000, 3_201_000];
/// The White Knight's three charges. One bit between them, so casting one replaces another.
pub const CHARGES: [u32; 3] = [1_211_001, 1_211_002, 1_211_003];

pub const POWER_GUARD: [u32; 2] = [1_111_005, 1_211_005];
pub const SPELL_BOOSTER: [u32; 2] = [2_111_005, 2_211_005];
pub const ELEMENT_AMPLIFICATION: [u32; 2] = [2_111_000, 2_211_000];
pub const SOUL_ARROW: [u32; 2] = [3_101_003, 3_201_003];
pub const INVINCIBLE: u32 = 2_301_002;
pub const HOLY_SYMBOL: u32 = 2_311_002;
pub const SHADOW_PARTNER: u32 = 4_111_001;
pub const COMBO_ATTACK: u32 = 1_111_000;
pub const DRAGON_BLOOD: u32 = 1_311_004;
pub const MESO_GUARD: u32 = 4_211_000;
pub const HYPER_BODY: u32 = 1_311_005;
pub const BLESS: u32 = 2_301_003;
pub const HEAL: u32 = 2_301_001;
pub const DRAIN: u32 = 4_101_002;
/// The two Crusader/White Knight attacks that spend the Combo orbs.
pub const COMBO_FINISHERS: [u32; 2] = [1_111_001, 1_111_002];
/// The three copies of MP Eater, one per Magician book.
pub const MP_EATER: [u32; 3] = [2_100_000, 2_200_000, 2_300_000];
/// The two third-job warrior "Improved MP Recovery" passives - flat MP every regen tick.
pub const IMPROVED_MP_RECOVERY_3RD: [u32; 2] = [1_110_000, 1_210_000];

/// The bit and value convention for a skill whose effect is not an `indie*` column.
///
/// `None` for everything else - including every buff the `indie*` mapping already covers,
/// so the two sources cannot disagree about one skill.
pub fn flag_buff(skill_id: u32) -> Option<FlagBuff> {
    use net::jobbuffs::*;
    Some(match skill_id {
        id if BOOSTERS.contains(&id) => timed(CTS_BOOSTER, ValueFrom::X),
        id if SPELL_BOOSTER.contains(&id) => timed(CTS_SPELL_BOOSTER, ValueFrom::X),
        id if POWER_GUARD.contains(&id) => timed(CTS_POWER_GUARD, ValueFrom::X),
        INVINCIBLE => timed(CTS_INVINCIBLE, ValueFrom::X),
        HOLY_SYMBOL => timed(CTS_HOLY_SYMBOL, ValueFrom::X),
        id if SOUL_ARROW.contains(&id) => timed(CTS_SOUL_ARROW, ValueFrom::Level),
        SHADOW_PARTNER => timed(CTS_SHADOW_PARTNER, ValueFrom::Level),
        id if CHARGES.contains(&id) => timed(CTS_WEAPON_CHARGE, ValueFrom::Level),
        id if FINAL_ATTACKS.contains(&id) => toggle(CTS_FINAL_ATTACK, ValueFrom::Level),
        COMBO_ATTACK => toggle(CTS_COMBO, ValueFrom::One),
        DRAGON_BLOOD => toggle(CTS_DRAGON_BLOOD, ValueFrom::Level),
        id if ELEMENT_AMPLIFICATION.contains(&id) => toggle(CTS_ELEMENT_AMP, ValueFrom::Level),
        MESO_GUARD => toggle(CTS_MESO_GUARD, ValueFrom::X),
        _ => return None,
    })
}

/// Every skill id this table names. For the audit's own census, and for the test that
/// checks each one against the generated file.
pub fn all_flag_buffs() -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    v.extend(BOOSTERS);
    v.extend(SPELL_BOOSTER);
    v.extend(POWER_GUARD);
    v.push(INVINCIBLE);
    v.push(HOLY_SYMBOL);
    v.extend(SOUL_ARROW);
    v.push(SHADOW_PARTNER);
    v.extend(CHARGES);
    v.extend(FINAL_ATTACKS);
    v.push(COMBO_ATTACK);
    v.push(DRAGON_BLOOD);
    v.extend(ELEMENT_AMPLIFICATION);
    v.push(MESO_GUARD);
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::firstjob::CombatTable;
    use std::path::Path;

    fn table() -> Option<CombatTable> {
        let p = Path::new("../../gm-handbook/skills.txt");
        if !p.exists() {
            return None; // generated, gitignored - python tools/dump_skills.py
        }
        let t = CombatTable::load(p);
        assert!(t.header_ok() && t.problems() == 0, "{}", t.banner());
        Some(t)
    }

    #[test]
    fn every_named_skill_is_in_the_clients_table_and_its_value_resolves_at_every_level() {
        let Some(t) = table() else { return };
        for id in all_flag_buffs() {
            let fb = flag_buff(id).unwrap();
            let sk = t.get(id).unwrap_or_else(|| panic!("{id} is not in skills.txt"));
            assert!(sk.levels_loaded() > 0, "{id} has no level rows");
            for row in sk.levels() {
                assert!(
                    fb.value.resolve(row, row.level).is_some(),
                    "{id} ({}) level {}: value {:?} does not resolve - the column is missing",
                    sk.name,
                    row.level,
                    fb.value
                );
            }
        }
    }

    /// The toggle flag was set from the archive's `processtype 113` correlation, and the
    /// generated file must agree with it skill by skill - a timed buff marked toggle would be
    /// held forever, and a toggle marked timed would expire after zero seconds.
    #[test]
    fn toggles_are_exactly_the_processtype_113_rows_with_no_time() {
        let Some(t) = table() else { return };
        for id in all_flag_buffs() {
            let fb = flag_buff(id).unwrap();
            let sk = t.get(id).unwrap();
            let has_time = sk.levels().any(|l| l.time_seconds.is_some_and(|s| s > 0));
            assert_eq!(
                fb.toggle, !has_time,
                "{id} ({}): toggle={} but time present={has_time}",
                sk.name, fb.toggle
            );
            if fb.toggle {
                assert_eq!(sk.processtype, Some(113), "{id} ({}) is a toggle", sk.name);
            }
        }
    }

    /// `x` carries a different meaning on every skill; the rows that read it as the value
    /// are the ones whose tooltip gives it a wire-ready unit. Pin those units here so a
    /// regenerated table that moves them is loud.
    #[test]
    fn the_x_values_are_what_the_tooltips_say() {
        let Some(t) = table() else { return };
        let x1 = |id: u32| t.level(id, 1).and_then(|l| l.x).unwrap();
        for id in BOOSTERS {
            assert_eq!(x1(id), -2, "booster {id}: 'by 2 stages'");
        }
        assert_eq!(x1(POWER_GUARD[0]), 20, "Power Guard L1 'return 20%'");
        assert_eq!(x1(INVINCIBLE), 10, "Invincible L1 'Physical damage -10%'");
        assert_eq!(x1(HOLY_SYMBOL), 5, "Holy Symbol L1 '5%'");
        assert_eq!(x1(MESO_GUARD), 30, "Meso Guard L1 'Blocks 30%'");
        assert_eq!(x1(SPELL_BOOSTER[0]), -1, "Spell Booster L1 'by 1 stage'");
    }

    #[test]
    fn the_indie_buffs_are_deliberately_not_in_this_table() {
        for id in [1_101_004, 1_301_004, 2_101_000, 2_201_000, 4_101_001, 4_201_001, HYPER_BODY, BLESS] {
            assert_eq!(flag_buff(id), None, "{id} rides an indie column, not a flag bit");
        }
        assert_eq!(flag_buff(net::buff::MAGIC_GUARD), None, "first job stays in net::buff");
    }

    #[test]
    fn the_value_conventions_resolve_as_documented() {
        let row = CastNumbers {
            level: 7,
            mp_con: None,
            hp_con: None,
            damage_percent: None,
            mad_percent: None,
            attack_count: None,
            mob_count: None,
            bullet_count: None,
            bullet_consume: None,
            time_seconds: None,
            cooltime_seconds: None,
            indie_speed: None,
            indie_jump: None,
            indie_pad: None,
            indie_mad: None,
            indie_pdd: None,
            indie_mdd: None,
            max_hp_percent: None,
            max_mp_percent: None,
            no_bullet_consume: false,
            item_con: None,
            item_con_no: None,
            money_con: None,
            x: Some(-2),
            y: None,
            prop: None,
            indie_acc: None,
            indie_eva: None,
            indie_mhp_r: None,
            fix_damage: None,
            tooltip_damage_percent: None,
            mastery: None,
            dot: None,
            dot_time_seconds: None,
            dot_interval_seconds: None,
            area: None,
        };
        assert_eq!(ValueFrom::X.resolve(&row, 7), Some(-2), "sign kept");
        assert_eq!(ValueFrom::Level.resolve(&row, 7), Some(7));
        assert_eq!(ValueFrom::One.resolve(&row, 7), Some(1));
        let no_x = CastNumbers { x: None, ..row };
        assert_eq!(ValueFrom::X.resolve(&no_x, 7), None, "no column, no grant");
    }
}
