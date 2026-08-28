//! Magic damage: this client's **second** damage path, and a validator, not a generator.
//!
//! Full working, with every address: **`research/magic-damage.md`**. The physical sibling is
//! [`crate::damage`] and this module leans on it deliberately - see *What is shared* below.
//!
//! # Read this first: nothing here has a caller, and that is not an oversight
//!
//! **The attack packet carries no skill id.** `research/mob-combat.md` §7 counted thirty
//! header fields in `0x00DF`, five of them non-zero, and explained none; the skill is not
//! among the ones that have been identified. So the server cannot tell Energy Bolt
//! (`2001002`, `mad` 90..130) from Magic Claw (`2001003`, `mad` 45..65) from an ordinary
//! swing - and for magic the skill *is* the whole formula: both `skill_magic_percent` and
//! `mastery` come from it.
//!
//! There is no neutral default to fall back on either. A physical plain swing is
//! `skill_damage_percent = 100`; the client pre-sets the **magic** local to **`0`**
//! (`research/magic-damage.md` §4.2), because a magician's plain swing is *physical* and goes
//! down the other path entirely. Guessing `100` here would invent a ceiling out of nothing.
//!
//! So: [`check_magic_hit`] exists, takes an `Option`, and every caller today would have to
//! pass `None`. **Wiring it is blocked on finding the skill field in the `0x00DF` header**
//! (`research/damage-formula.md` §11), and until then this module is decoded-and-unwired in
//! exactly the way `CLAUDE.md`'s "Built is not wired" section says to record loudly.
//!
//! Same rule as [`crate::damage`] when it *is* wired: **log only, never refuse.** An
//! unanswered or rejected packet freezes the client's UI.
//!
//! # The formula
//!
//! `FUN_14025ffd0`, `0x14025ffd0..0x140260a0a`. Every line below is **[L]** - read off the
//! listing - except where marked.
//!
//! ```text
//! M   = (min(mastery, 10) / 10 + 0.1) * 0.8      the SAME function the physical path calls
//! u   = rand32 * 2^-32                            half-open [0, 1)
//! Q   = uniform(INT * M, INT)                     ONE roll. Physical has two.
//! raw = (skillMagic / 100) * MagicTotal * (1 + Q / 100)
//! ```
//!
//! then, instruction for instruction the same tail as the physical path:
//! defence `* 100 / (MDEF + 100)` -> element -> level gap -> damage-up buff -> critical ->
//! `trunc(clamp(v, 1, 99999))`.
//!
//! # What is shared with the physical path, and what is genuinely different
//!
//! | | |
//! |---|---|
//! | [`crate::damage::mastery_factor`] | **shared, the same client function.** `FUN_14025e840`, called from the physical calculator at `0x14025f75a` and the magic one at `0x1402601cc` |
//! | [`crate::damage::after_defence`] | **shared**, with **MDEF** instead of PDEF as its input |
//! | [`crate::damage::level_gap_scale`] | **shared**, same `0.005` / `0.05` / `9` literals in both functions |
//! | [`crate::damage::finish`], [`crate::damage::DAMAGE_CAP`] | **shared**, the same `0x1869f` |
//! | element ladder | **shared** in the client (`FUN_14025e8d0`); it lives *here* only because `damage.rs` never needed it. See [`element_factor`] |
//! | critical | **shared shape**, and both write the flag as a byte at `hit_record - 7` |
//! | the second `uniform(0.8, 1.0)` roll | **DIFFERENT: magic does not have one.** See [`magic_window`] |
//! | [`crate::damage::Scalars`] (`roll_lo`/`roll_hi`/`stat_div`/`ap_div`) | **does not apply.** They are written by `FUN_14025e000`, which the magic path never calls |
//! | weapon multiplier, the 30..=47 table, Lucky Seven | **does not apply.** No weapon term at all |
//! | [`crate::damage::primary_and_secondary`] | **does not apply.** Magic reads INT (`statObj+0x3c`) and nothing else |
//! | attacker totals field | `+0x08` (**MagicTotal**), where physical uses `+0x00` and `+0x04` |
//!
//! # Every quantity here states its unit
//!
//! `CLAUDE.md`'s "the unit, not the arithmetic":
//!
//! | quantity | unit | note |
//! |---|---|---|
//! | `mastery` | **a level, 1..=10** | *not* a percentage. Identical to the physical path - `Skill.wz` gives Energy Bolt `mastery` 1..10 over its 20 levels |
//! | `skill_magic_percent` | **percent**, e.g. `90` for Energy Bolt Lv1 | the `mad` node. **[D]**, see [`MagicAttacker::skill_magic_percent`] |
//! | `magic_total` | **absolute points** | seeded `floor(INT/2)`, plus equipment `incMAD`, plus CTS 85 |
//! | `intelligence` | **absolute points**, total INT | the same number `floor(INT/2)` was taken from |
//! | crit rate | **percent**, an integer | `totals+0x20`. *Not* a fraction, and *not* a global |
//! | crit damage | **percent bonus**, an integer | `totals+0x24`; the multiplier is `(x + 100) / 100` |
//! | element code | **an index 0..=6**, not a factor | see [`element_factor`] |
//! | `attackCount` | **not here at all** | see [`MagicAttacker`] |
//!
//! # `0` is a value, not "unset"
//!
//! Three places in this module where that bites, all of them the shape of the mob-size bug:
//!
//! * `skill_magic_percent == 0` means **zero magic damage**, which is what the client's own
//!   pre-set says a non-skill produces on this path.
//! * `mastery == 0` means **no mastery skill**, `M = 0.08`, the widest possible spread.
//! * element code `1` is a factor of **0.00**, not "no element". "No element" is code `0`.

use crate::damage;

// ---------------------------------------------------------------------------------------
// MagicTotal
// ---------------------------------------------------------------------------------------

/// What `MagicTotal` starts at before equipment and buffs: `floor(INT / 2)`. **[L]**
///
/// `FUN_14087c130` is the attacker-totals builder, and its second write is
/// `getter(statObj+0x3c); sar eax,1 -> totals[+0x08]` at `0x14087c1b4`. An arithmetic right
/// shift of a non-negative stat is a **floor**, so `INT 41` seeds `20`, not `21`, and the
/// guide's `MagicTotal = floor(TotalINT/2) + MATK` is read rather than inferred.
///
/// The two other seeds from the same run of writes are already in [`crate::damage`]:
/// [`crate::damage::wdef_from_strength`] is `floor(STR/4)` and
/// [`crate::damage::mdef_from_intelligence`] is `floor(INT/4)`.
///
/// **This is a seed, not the total.** Equipment `incMAD` and CTS bit 85 are added on top by
/// the client, and the server does not have either today - `research/magic-damage.md` §6
/// flags that `gm-handbook/equips.txt` has already been found missing two columns and that
/// nobody has checked whether it carries `incMAD` at all.
pub fn magic_total_seed(intelligence: u32) -> u32 {
    intelligence / 2
}

// ---------------------------------------------------------------------------------------
// The element ladder
// ---------------------------------------------------------------------------------------

/// The element factors, indexed by the client's own code. **[L]**
///
/// `FUN_14025e8d0` is a bare 7-entry jump table on an `int` (table at `0x14025e928`), and
/// anything above `6` falls through to `1.00`.
///
/// | code | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
/// |---|---|---|---|---|---|---|---|
/// | factor | 1.00 | 0.00 | 0.75 | 1.25 | 0.25 | 0.50 | 1.50 |
///
/// The code comes from `FUN_140474fa0(mob, attrA, attrB)` on the magic path. **Which mob
/// attribute produces which code is not established here** - the seven *values* are, and
/// their order is.
pub const ELEMENT_FACTORS: [f64; 7] = [1.00, 0.00, 0.75, 1.25, 0.25, 0.50, 1.50];

/// The factor for one element code, `>= 7` defaulting to `1.00`. **[L]**
///
/// **Code `1` is `0.00`, and that is a real outcome, not a missing entry.** With
/// [`crate::damage::finish`]'s floor of `1`, a fully-resisted cast still ends up as a drawn
/// `1` in this model. Whether this client draws `1` or a miss for an immune target has not
/// been observed, and the difference has never mattered because nothing is wired.
pub fn element_factor(code: u32) -> f64 {
    *ELEMENT_FACTORS.get(code as usize).unwrap_or(&1.00)
}

/// The smallest factor the ladder can produce: **0.00**, code 1. **[L]**
pub const MIN_ELEMENT_FACTOR: f64 = 0.00;

/// The largest factor the ladder can produce: **1.50**, code 6. **[L]**
///
/// # This is why a magic ceiling cannot skip the element step
///
/// [`crate::damage::max_plausible_hit`] skips defence, element and the level gap on the
/// grounds that "both only ever reduce". That is true of defence and of the level gap. It is
/// **not true of the element ladder**: code 6 multiplies by `1.5`. A ceiling that ignored the
/// element against an unknown target would sit a third *below* what the client can legally
/// draw, and would then report a legitimate cast as too high - which is the one failure mode
/// a validator must not have.
pub const MAX_ELEMENT_FACTOR: f64 = 1.50;

/// `(min, max)` element factor for a target whose code may not be known.
///
/// A known code gives a point; `None` gives the whole ladder, because an unknown target could
/// be immune (`0.00`) or weak (`1.50`).
pub fn element_bounds(code: Option<u32>) -> (f64, f64) {
    match code {
        Some(c) => {
            let f = element_factor(c);
            (f, f)
        }
        None => (MIN_ELEMENT_FACTOR, MAX_ELEMENT_FACTOR),
    }
}

// ---------------------------------------------------------------------------------------
// Effective magic defence
// ---------------------------------------------------------------------------------------

/// The mob's effective magic defence. **[L]** for the shape, from `FUN_14025e770`.
///
/// ```text
/// mdef = tempStat(mob, 5) + trunc((tempStat(mob, 6) / 100 + 1) * mobTemplate.MDDamage)
/// ```
///
/// clamped to `>= 0` with a `cmovs`. Index **5 is the flat MDD modifier** and index **6 is a
/// percentage** - that pairing is the whole reason this helper exists rather than passing
/// `MDDamage` straight to [`crate::damage::after_defence`].
///
/// `mdd_damage` is the template's `MDDamage` column. The physical twin is `FUN_14025e6c0`
/// with indices 3 and 4 and `PDDamage`.
///
/// # Two things this does not model, both named rather than smoothed over
///
/// * Both helpers short-circuit to a different function when `[mob + 0x4c0] > 0`. Not
///   identified.
/// * The **physical** helper has an extra early exit the magic one does not
///   (`cmp dword [mob+0x40], 0 / jle`, returning zero defence). It is absent from the magic
///   helper, so it does not belong here - but it is unexplained, and it is recorded in
///   `research/magic-damage.md` §2.5 as such.
pub fn effective_magic_defence(mdd_damage: u32, flat_bonus: i32, percent_bonus: i32) -> u32 {
    let scaled = ((percent_bonus as f64 / 100.0 + 1.0) * mdd_damage as f64).trunc();
    let total = flat_bonus as f64 + scaled;
    if !total.is_finite() || total < 0.0 {
        0
    } else if total > u32::MAX as f64 {
        u32::MAX
    } else {
        total as u32
    }
}

// ---------------------------------------------------------------------------------------
// Criticals - per character, not global
// ---------------------------------------------------------------------------------------

/// The critical stage's two inputs, both **integer percentages**, both read per character.
///
/// **[L]** `research/magic-damage.md` §2.2 step 5, in both damage functions:
///
/// ```text
/// if trunc(u * 100) < totals[+0x20]:      # rate, percent
///     d *= (totals[+0x24] + 100) / 100    # damage bonus, percent
/// ```
///
/// # Why this is a parameter and not a constant
///
/// [`crate::damage::CRIT_RATE`] and [`crate::damage::CRIT_MULTIPLIER`] are module constants,
/// and their doc blocks now carry the limitation: the client reads both from the attacker's
/// **totals struct**, so they are one character's values, not the game's. A magician in
/// `+critical` gear moves both. [`CritStats::DEFAULT`] carries the two numbers those
/// constants imply and a test asserts the two representations agree, so a change to either
/// one cannot drift silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CritStats {
    /// `totals + 0x20`, **percent**. `0` means this character never crits.
    pub rate_percent: u32,
    /// `totals + 0x24`, **percent bonus**. `20` means a crit is `1.2x`.
    pub damage_percent: u32,
}

impl CritStats {
    /// The values [`crate::damage`]'s two constants imply. **[D]**, and only for the one
    /// character those constants were measured on.
    pub const DEFAULT: CritStats = CritStats { rate_percent: 5, damage_percent: 20 };

    /// A character who cannot crit at all - the honest choice when the fields are unread.
    pub const NONE: CritStats = CritStats { rate_percent: 0, damage_percent: 0 };
}

impl Default for CritStats {
    fn default() -> Self {
        CritStats::DEFAULT
    }
}

/// What a critical multiplies by: `(damage_percent + 100) / 100`. **[L]**
///
/// Note the unit: `damage_percent` is the **bonus**, so `20` gives `1.2`, not `0.2`.
pub fn crit_multiplier(damage_percent: u32) -> f64 {
    (damage_percent as f64 + 100.0) / 100.0
}

/// The multiplier a **ceiling** must allow for. **[L]** for the gate.
///
/// The whole critical stage sits inside `if trunc(u*100) < rate`, so a character with
/// `rate_percent == 0` can never take it and their ceiling must not include it. This is the
/// only place the crit *rate* affects a window at all - it moves nothing else, exactly as
/// `mastery` moves nothing at the top.
pub fn crit_ceiling_multiplier(crit: CritStats) -> f64 {
    if crit.rate_percent == 0 {
        1.0
    } else {
        crit_multiplier(crit.damage_percent)
    }
}

// ---------------------------------------------------------------------------------------
// The attacker
// ---------------------------------------------------------------------------------------

/// Everything about the attacker the magic formula needs. Four numbers, and that is all -
/// there is no weapon, no action and no secondary stat on this path.
///
/// # `attackCount` is deliberately absent
///
/// `Skill.wz` gives Energy Bolt `attackCount 1` and Magic Claw `attackCount 2`. That is a
/// count of **separate hits**, each of which the client computes and draws on its own, and
/// each of which goes through `FUN_14025ffd0` once. Folding it in here would double every
/// Magic Claw number and make its per-hit window wrong by exactly 2x. **A caller that wants a
/// per-cast total multiplies.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MagicAttacker {
    /// The attacker-totals field at `+0x08`: `floor(INT/2)` + equipment `incMAD` + CTS 85.
    /// **Absolute points.** [`magic_total_seed`] computes the first term.
    pub magic_total: u32,
    /// Total INT - base plus equipment plus buffs - the value at `statObj + 0x3c`. This is
    /// the stat `FUN_14025e000` never touches, which is how the magic function was found.
    pub intelligence: u32,
    /// `Skill.wz` `mastery` for the skill cast, **1..=10**, or `0` for none.
    pub mastery: u32,
    /// The skill's magic number, **in percent**. Energy Bolt Lv1 is `90`.
    ///
    /// # This is [D], not [L], and the blind spot has a name
    ///
    /// The client reads it from `[skillLevel + 0x5c]`. That it is `Skill.wz`'s `mad` node
    /// rests on four things - the local's pre-set of `0`, this build's magic skills carrying
    /// `mad` and no `damage`, the client's own tooltip calling `mad` the "Basic Attack"
    /// number, and it being the only per-level number Energy Bolt has that scales.
    /// **`research/magic-damage.md` §4.2 did not find the WZ loader that writes `+0x5c`**, and
    /// names what would hide it: a struct filled from a `{name, offset}` table, or by an
    /// inlined literal copy, leaves no `lea` of the string to find.
    ///
    /// So it is a **parameter**. Nothing in this module reads it from anywhere.
    pub skill_magic_percent: u32,
}

impl Default for MagicAttacker {
    /// **`skill_magic_percent` defaults to `0`, and that is the client's own choice.**
    ///
    /// `research/magic-damage.md` §4.2: the physical local is pre-set to `100.0` before the
    /// skill lookup - a percentage whose neutral value is a plain swing - and the magic local
    /// is pre-set to **`0`**, because a plain swing does no magic damage. So
    /// `MagicAttacker::default()` is a character casting *nothing*, and it produces zero,
    /// where [`crate::damage::Attacker::default`] is a character swinging.
    fn default() -> Self {
        MagicAttacker { magic_total: 0, intelligence: 0, mastery: 0, skill_magic_percent: 0 }
    }
}

// ---------------------------------------------------------------------------------------
// The magic damage window
// ---------------------------------------------------------------------------------------

/// The `[min, max]` window a single magic hit can land in, **before** defence, element, level
/// gap and criticals. Both ends are **raw damage points**, un-truncated. **[L]**
///
/// ```text
/// M   = (mastery / 10 + 0.1) * 0.8
/// MIN = (skillMagic / 100) * MagicTotal * (1 + INT * M / 100)
/// MAX = (skillMagic / 100) * MagicTotal * (1 + INT     / 100)
/// ```
///
/// # The one difference that changes the shape of the answer
///
/// **Magic has ONE uniform roll. Physical has two.** [`crate::damage::physical_window`]
/// carries an independent `uniform(roll_lo, roll_hi)` on top of the mastery roll, so its
/// bottom is `0.8x` its top even when every stat is zero. There is no second draw in
/// `FUN_14025ffd0` - `mastery` is the *only* thing that sets the bottom of this window, and
/// with `INT == 0` the window collapses to a single point. That is the cheapest test that
/// tells the two shapes apart, and it is in this module's tests.
///
/// # The top end is a supremum, not a value
///
/// `u = rand32 * 2^-32` is half-open `[0, 1)` (`research/magic-damage.md` §2.4), so `MAX` is
/// approached and never reached: the largest `Q` the client can draw is
/// `lo + (1 - 2^-32) * (hi - lo)`. Returning the inclusive bound is the **safe** direction for
/// a ceiling - one ulp too generous never rejects a legitimate cast - and it is called out
/// because using it as a *generator* would be one ulp wrong.
///
/// # No `Option`, unlike the physical window
///
/// [`crate::damage::physical_window`] returns `None` for the combinations where the client
/// leaves the weapon multiplier at zero. Magic has no weapon term, so there is no such case:
/// a zero here comes from a zero input and means zero.
pub fn magic_window(attacker: &MagicAttacker) -> (f64, f64) {
    let s = attacker.skill_magic_percent as f64 / 100.0;
    let magic_total = attacker.magic_total as f64;
    let int = attacker.intelligence as f64;
    // The SAME function the physical path calls - FUN_14025e840, one implementation.
    let m = damage::mastery_factor(attacker.mastery);

    // The client orders the pair before rolling between them ("order the pair, then
    // lo + u*(hi-lo)" at 0x140260550..). M never exceeds 0.88, so INT*M <= INT for every
    // non-negative INT and the ordering is a no-op here - it is kept because the client does
    // it and because it is what makes this a range rather than a signed span.
    let a = int * m;
    let q_lo = a.min(int);
    let q_hi = a.max(int);

    let lo = s * magic_total * (1.0 + q_lo / 100.0);
    let hi = s * magic_total * (1.0 + q_hi / 100.0);
    (lo, hi)
}

// ---------------------------------------------------------------------------------------
// The target, and the whole tail
// ---------------------------------------------------------------------------------------

/// What the post-processing tail needs to know about the mob.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MagicTarget {
    /// Effective **magic** defence, absolute. [`effective_magic_defence`] builds it from the
    /// template's `MDDamage`.
    pub magic_defence: u32,
    /// The element code, `0..=6`. `None` when it is not known, which widens the window to the
    /// whole ladder - see [`element_bounds`].
    pub element_code: Option<u32>,
    /// The mob's level, for [`crate::damage::level_gap_scale`].
    pub level: u32,
}

impl Default for MagicTarget {
    /// An unarmoured, unknown-element, level-1 mob: the widest honest target.
    fn default() -> Self {
        MagicTarget { magic_defence: 0, element_code: None, level: 1 }
    }
}

/// The `[min, max]` **drawn number** for one magic hit, through the whole tail. Both ends are
/// already `trunc(clamp(v, 1, 99999))`, so they are what the client would print.
///
/// Order of operations, all **[L]** and all shared with the physical path except the element
/// step's input:
///
/// ```text
/// raw          magic_window
/// defence      * 100 / (MDEF + 100)         crate::damage::after_defence
/// element      * elementFactor(code)        this module
/// level gap    * level_gap_scale            crate::damage::level_gap_scale
/// critical     * (critDamage + 100) / 100   top end only, and only if the rate is non-zero
/// finish       trunc(clamp(v, 1, 99999))    crate::damage::finish
/// ```
///
/// # One step is deliberately not modelled, and it is the one that would break a ceiling
///
/// Between the level gap and the critical the client applies a **damage-up buff**:
/// `if getter(totals+0x1b8) > 0 { d += d * FUN_14025e950(...) }`. That term **increases**
/// damage, and nothing here models it, so the top of this window is a ceiling only for a
/// character who does not have that buff. No such buff is granted by this server today -
/// `net::buff::buff_level` returns `None` for everything except Nimble Feet - but this is the
/// assumption that would silently go stale first.
pub fn magic_hit_window(
    attacker: &MagicAttacker,
    target: &MagicTarget,
    player_level: u32,
    crit: CritStats,
) -> (u64, u64) {
    let (raw_lo, raw_hi) = magic_window(attacker);
    let (element_lo, element_hi) = element_bounds(target.element_code);
    let gap = damage::level_gap_scale(player_level, target.level);

    let lo = damage::after_defence(raw_lo, target.magic_defence) * element_lo * gap;
    let hi = damage::after_defence(raw_hi, target.magic_defence)
        * element_hi
        * gap
        * crit_ceiling_multiplier(crit);

    (damage::finish(lo), damage::finish(hi))
}

/// The largest number one magic hit can draw when the **target is not known**.
///
/// The counterpart of [`crate::damage::max_plausible_hit`], and it differs from it in one
/// way that matters: it multiplies by [`MAX_ELEMENT_FACTOR`]. Defence and the level gap are
/// skipped because they only ever reduce; the element ladder is **not** skipped, because
/// `1.50` is on it.
///
/// `attackCount` is not applied - see [`MagicAttacker`].
pub fn magic_ceiling(attacker: &MagicAttacker, crit: CritStats) -> u64 {
    let (_, hi) = magic_window(attacker);
    damage::finish(hi * MAX_ELEMENT_FACTOR * crit_ceiling_multiplier(crit))
}

/// Check one claimed magic damage number.
///
/// **Always returns a verdict and never an error**, and the `Option` is the whole point:
/// today every call site would pass `None`, because the attack packet carries no skill id and
/// [`MagicAttacker::skill_magic_percent`] therefore has no defensible value. See this
/// module's header.
///
/// Reuses [`crate::damage::HitVerdict`] rather than defining a parallel enum, so a caller
/// that one day checks both paths has one type to match on.
pub fn check_magic_hit(
    claimed: u64,
    attacker: Option<&MagicAttacker>,
    crit: CritStats,
) -> damage::HitVerdict {
    let Some(attacker) = attacker else { return damage::HitVerdict::Unchecked };
    let ceiling = magic_ceiling(attacker, crit);
    if claimed > ceiling {
        damage::HitVerdict::TooHigh { ceiling }
    } else {
        damage::HitVerdict::Plausible
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `research/magic-damage.md` §5's worked example, as a struct: a Magician with **INT 40**
    /// and no MATK gear casting **Energy Bolt Lv1** (`mad 90`, `mastery 1`).
    fn energy_bolt_lv1() -> MagicAttacker {
        MagicAttacker {
            magic_total: magic_total_seed(40),
            intelligence: 40,
            mastery: 1,
            skill_magic_percent: 90,
        }
    }

    // -- MagicTotal ----------------------------------------------------------------------

    /// `sar eax,1` is a floor, and the odd case is the one that could disagree.
    #[test]
    fn magic_total_is_seeded_with_the_floor_of_half_int_not_the_round() {
        assert_eq!(magic_total_seed(40), 20, "the worked example's number");
        assert_eq!(magic_total_seed(41), 20, "floor, not round-half-up");
        assert_eq!(magic_total_seed(1), 0, "INT 1 seeds nothing at all");
        assert_eq!(magic_total_seed(0), 0);
    }

    /// The three seeds come from one run of writes in one function, so they are checked
    /// against each other: INT feeds MagicTotal at half and magic defence at a quarter.
    #[test]
    fn the_int_seeds_agree_with_the_ones_already_in_damage_rs() {
        for int in [0u32, 1, 3, 4, 7, 40, 41, 999] {
            assert_eq!(
                damage::mdef_from_intelligence(int),
                magic_total_seed(int) / 2,
                "INT {int}: floor(INT/4) must be floor(floor(INT/2)/2)"
            );
        }
        // And STR feeds neither of them.
        assert_eq!(damage::wdef_from_strength(40), 10);
    }

    // -- the element ladder ----------------------------------------------------------------

    #[test]
    fn the_element_ladder_is_the_clients_seven_values_and_anything_above_six_is_neutral() {
        let table = [(0u32, 1.00), (1, 0.00), (2, 0.75), (3, 1.25), (4, 0.25), (5, 0.50), (6, 1.50)];
        for (code, want) in table {
            assert!((element_factor(code) - want).abs() < 1e-12, "code {code}");
        }
        // The jump table covers 0..=6 and the default arm is 1.00.
        for code in [7u32, 8, 100, u32::MAX] {
            assert!((element_factor(code) - 1.00).abs() < 1e-12, "code {code} falls through");
        }
        // Code 1 is zero damage, not "no element". Code 0 is "no element".
        assert_eq!(element_factor(1), 0.0);
        assert_eq!(element_factor(0), 1.0);
    }

    /// The bounds are declared as constants; assert they are actually the table's extremes,
    /// including the `> 6` default. If a value were transcribed wrongly this fires.
    #[test]
    fn the_declared_element_bounds_are_the_tables_own_extremes() {
        let mut lo = element_factor(7); // the default arm counts too
        let mut hi = lo;
        for f in ELEMENT_FACTORS {
            lo = lo.min(f);
            hi = hi.max(f);
        }
        assert!((lo - MIN_ELEMENT_FACTOR).abs() < 1e-12, "min was {lo}");
        assert!((hi - MAX_ELEMENT_FACTOR).abs() < 1e-12, "max was {hi}");
    }

    /// The reason a magic ceiling cannot copy `max_plausible_hit`'s "skip it, it only
    /// reduces".
    #[test]
    fn the_element_step_can_increase_damage_which_defence_and_the_level_gap_cannot() {
        // Written against the table through the function rather than as
        // `MAX_ELEMENT_FACTOR > 1.0`, which the compiler folds to `true` and which asserts
        // arithmetic instead of the transcription. This can come back false.
        let neutral = element_factor(0);
        let weakest = element_factor(6);
        assert!(
            weakest > neutral,
            "code 6 ({weakest}) must exceed the neutral code 0 ({neutral}), or an \
             element-blind ceiling would be safe and MAX_ELEMENT_FACTOR pointless"
        );
        // The two steps that really do only reduce, for contrast.
        for def in [0u32, 1, 50, 10_000] {
            assert!(damage::after_defence(100.0, def) <= 100.0 + 1e-12, "def {def}");
        }
        for mob_level in [1u32, 10, 11, 20, 60] {
            assert!(damage::level_gap_scale(10, mob_level) <= 1.0, "mob level {mob_level}");
        }
        // And an unknown element widens the window in both directions.
        assert_eq!(element_bounds(None), (MIN_ELEMENT_FACTOR, MAX_ELEMENT_FACTOR));
        assert_eq!(element_bounds(Some(3)), (1.25, 1.25));
    }

    // -- magic defence ---------------------------------------------------------------------

    /// Index 5 is flat and index 6 is a **percentage**. Getting that pair backwards is
    /// exactly the class of bug `CLAUDE.md`'s unit section is about, so the two are separated
    /// by an input that would make them look the same if they were swapped.
    #[test]
    fn the_mdd_modifiers_are_one_flat_and_one_percent() {
        // No modifiers at all: the template's own number, unchanged.
        assert_eq!(effective_magic_defence(80, 0, 0), 80);
        // A +50 flat and a +50 percent do very different things to the same 80.
        assert_eq!(effective_magic_defence(80, 50, 0), 130);
        assert_eq!(effective_magic_defence(80, 0, 50), 120);
        // The percent scales the template and is truncated before the flat is added.
        assert_eq!(effective_magic_defence(7, 0, 50), 10, "trunc(1.5 * 7) = 10");
        // The cmovs floor: a debuff cannot make defence negative.
        assert_eq!(effective_magic_defence(10, -100, 0), 0);
        assert_eq!(effective_magic_defence(10, 0, -200), 0);
        // A snail: MDDamage 0, so nothing to scale.
        assert_eq!(effective_magic_defence(0, 0, 300), 0);
    }

    // -- criticals -------------------------------------------------------------------------

    /// The two representations of the same measurement must agree. This is a cross-module
    /// check: editing either `damage.rs`'s constants or `CritStats::DEFAULT` alone fails it.
    #[test]
    fn the_default_crit_stats_agree_with_the_constants_in_damage_rs() {
        assert!(
            (crit_multiplier(CritStats::DEFAULT.damage_percent) - damage::CRIT_MULTIPLIER).abs()
                < 1e-12,
            "damage_percent is a BONUS: 20 must become 1.2, not 0.2"
        );
        assert!(
            (CritStats::DEFAULT.rate_percent as f64 / 100.0 - damage::CRIT_RATE).abs() < 1e-12,
            "rate is a percent here and a fraction there"
        );
    }

    /// The critical stage is gated on the rate, so a character who cannot crit gets no
    /// headroom for one. Nothing else in the model reads the rate.
    #[test]
    fn a_zero_crit_rate_removes_the_crit_headroom_entirely() {
        let a = energy_bolt_lv1();
        let with_crit = magic_ceiling(&a, CritStats::DEFAULT);
        let no_crit = magic_ceiling(&a, CritStats::NONE);
        assert!(no_crit < with_crit, "{no_crit} vs {with_crit}");
        assert_eq!(crit_ceiling_multiplier(CritStats::NONE), 1.0);
        // A huge crit damage with a zero rate still buys nothing.
        let unreachable = CritStats { rate_percent: 0, damage_percent: 900 };
        assert_eq!(magic_ceiling(&a, unreachable), no_crit);
    }

    // -- the window ------------------------------------------------------------------------

    /// `research/magic-damage.md` §5, to three places, so any change to the formula shows up
    /// as a number rather than as a passing test.
    #[test]
    fn the_worked_example_from_the_research_file_reproduces_exactly() {
        let a = energy_bolt_lv1();
        assert_eq!(a.magic_total, 20, "floor(40/2), no MATK gear");
        assert!((damage::mastery_factor(1) - 0.16).abs() < 1e-12, "M at mastery 1");

        let (lo, hi) = magic_window(&a);
        assert!((lo - 19.152).abs() < 0.001, "min was {lo}, §5 says 19.15");
        assert!((hi - 25.200).abs() < 0.001, "max was {hi}, §5 says 25.20");

        // Through the tail against §5's target: a template-2 snail, MDD 0, level 1, no
        // element, cast by a level-10 character. §5: "final 19 .. 25".
        let target = MagicTarget { magic_defence: 0, element_code: Some(0), level: 1 };
        assert_eq!(magic_hit_window(&a, &target, 10, CritStats::NONE), (19, 25));
        // "...and a crit multiplies by (100 + critDamage) / 100": 25.2 * 1.2 = 30.24.
        assert_eq!(magic_hit_window(&a, &target, 10, CritStats::DEFAULT), (19, 30));
    }

    /// **The cheapest test that tells magic apart from physical**, and the one that would
    /// catch a second roll being copied across from `damage.rs`.
    ///
    /// With the stat at zero, physical still spans `roll_lo..roll_hi` = `0.8..1.0`; magic
    /// collapses to a point, because `Q = uniform(0*M, 0)` is `[0, 0]` and there is no other
    /// draw in `FUN_14025ffd0`.
    #[test]
    fn magic_has_one_roll_where_physical_has_two() {
        let magic = MagicAttacker {
            magic_total: 20,
            intelligence: 0,
            mastery: 10,
            skill_magic_percent: 100,
        };
        let (mlo, mhi) = magic_window(&magic);
        assert!((mlo - mhi).abs() < 1e-12, "magic collapsed to a point: {mlo} .. {mhi}");
        assert!((mhi - 20.0).abs() < 1e-12, "(100/100) * 20 * (1 + 0)");

        let physical = damage::Attacker {
            total_watk: 20,
            stats: damage::Stats::default(),
            mastery: 10,
            attack_power: 0,
            skill_damage_percent: 100,
        };
        let (plo, phi) = damage::physical_window(
            &physical,
            damage::WeaponClass::OneHandSword,
            damage::AttackAction::Swing,
            0,
        )
        .unwrap();
        assert!((plo / phi - 0.8).abs() < 1e-12, "physical keeps its uniform(0.8, 1.0): {plo}/{phi}");
    }

    /// The client's own tooltip: *"The higher the Mastery, the lower the damage variation."*
    /// Mastery moves the **bottom** of the window and nothing else, so the spread must shrink
    /// monotonically while the top stays put.
    #[test]
    fn higher_mastery_narrows_the_spread_and_never_moves_the_top() {
        let base = energy_bolt_lv1();
        let (_, top_at_one) = magic_window(&MagicAttacker { mastery: 1, ..base });
        let mut previous_spread = f64::INFINITY;
        for mastery in 0..=damage::MASTERY_MAX {
            let (lo, hi) = magic_window(&MagicAttacker { mastery, ..base });
            assert!(
                (hi - top_at_one).abs() < 1e-12,
                "mastery {mastery} moved the top to {hi}; mastery is a floor, not a cap"
            );
            let spread = hi - lo;
            assert!(spread < previous_spread, "mastery {mastery} did not narrow: {spread}");
            previous_spread = spread;
        }
        // §5's two named cases, as drawn numbers: 6 wide at mastery 1, 1 wide at mastery 10.
        let target = MagicTarget { magic_defence: 0, element_code: Some(0), level: 1 };
        let one = magic_hit_window(&MagicAttacker { mastery: 1, ..base }, &target, 10, CritStats::NONE);
        let ten =
            magic_hit_window(&MagicAttacker { mastery: 10, ..base }, &target, 10, CritStats::NONE);
        assert_eq!(one, (19, 25));
        assert_eq!(ten, (24, 25), "§5: raw 24.34 .. 25.20");
        assert!(ten.1 - ten.0 < one.1 - one.0);
    }

    /// `attackCount` must not be folded in anywhere.
    ///
    /// Magic Claw Lv1 is `mad 45, attackCount 2`; Energy Bolt Lv1 is `mad 90, attackCount 1`.
    /// Their **per-cast totals coincide**, which is exactly why this is worth pinning: if the
    /// count were folded in, the two windows would come out equal and look correct. They must
    /// differ by a factor of two, and only a caller's multiplication makes the totals meet.
    #[test]
    fn the_window_is_per_hit_and_attack_count_belongs_to_the_caller() {
        let bolt = energy_bolt_lv1();
        let claw = MagicAttacker { skill_magic_percent: 45, ..bolt };

        let (bolt_lo, bolt_hi) = magic_window(&bolt);
        let (claw_lo, claw_hi) = magic_window(&claw);

        assert!((claw_lo * 2.0 - bolt_lo).abs() < 1e-12, "claw min {claw_lo} is not half {bolt_lo}");
        assert!((claw_hi * 2.0 - bolt_hi).abs() < 1e-12, "claw max {claw_hi} is not half {bolt_hi}");
        assert!((claw_hi - 12.6).abs() < 1e-12, "one Magic Claw hit is 12.6, not 25.2");

        // The struct has nowhere to put a count, which is the enforcement rather than the
        // comment: this is the whole of what the formula reads.
        let MagicAttacker { magic_total: _, intelligence: _, mastery: _, skill_magic_percent: _ } =
            claw;
    }

    /// `skill_magic_percent` is a percentage, and its neutral value is **0**, not 100 - the
    /// client pre-sets the two locals differently and that difference is the whole reason a
    /// magic ceiling cannot be built without the skill id.
    #[test]
    fn the_magic_default_is_zero_where_the_physical_default_is_a_hundred() {
        assert_eq!(MagicAttacker::default().skill_magic_percent, 0);
        assert_eq!(damage::Attacker::default().skill_damage_percent, 100);

        // And it really is a percentage: doubling it doubles both ends.
        let a = energy_bolt_lv1();
        let (lo, hi) = magic_window(&a);
        let (lo2, hi2) = magic_window(&MagicAttacker { skill_magic_percent: 180, ..a });
        assert!((lo2 / lo - 2.0).abs() < 1e-12);
        assert!((hi2 / hi - 2.0).abs() < 1e-12);

        // A character casting nothing does no magic damage, and finish still floors the
        // drawn number at 1.
        let nothing = MagicAttacker { magic_total: 999, intelligence: 999, ..Default::default() };
        assert_eq!(magic_window(&nothing), (0.0, 0.0));
        assert_eq!(
            magic_hit_window(&nothing, &MagicTarget::default(), 1, CritStats::DEFAULT),
            (1, 1)
        );
    }

    /// The property that would catch a mis-ordered `Q` pair, a sign slip, or a tail step
    /// applied to only one end: over a wide sweep, the window must never be inverted, and the
    /// drawn numbers must stay inside the client's own clamp.
    #[test]
    fn the_window_is_never_inverted_and_is_always_inside_the_clients_clamp() {
        let mut checked = 0u32;
        for magic_total in [0u32, 1, 20, 300, 50_000, u32::MAX / 4] {
            for intelligence in [0u32, 1, 40, 500, 30_000] {
                for mastery in [0u32, 1, 5, 10, 60] {
                    for skill in [0u32, 45, 90, 130, 1_000] {
                        let a = MagicAttacker {
                            magic_total,
                            intelligence,
                            mastery,
                            skill_magic_percent: skill,
                        };
                        let (lo, hi) = magic_window(&a);
                        assert!(lo <= hi, "raw window inverted: {lo} .. {hi} for {a:?}");

                        for element in [None, Some(0), Some(1), Some(6), Some(99)] {
                            for defence in [0u32, 100, 9_999] {
                                for mob_level in [1u32, 10, 90] {
                                    let t = MagicTarget {
                                        magic_defence: defence,
                                        element_code: element,
                                        level: mob_level,
                                    };
                                    let (dlo, dhi) =
                                        magic_hit_window(&a, &t, 10, CritStats::DEFAULT);
                                    assert!(dlo <= dhi, "drawn window inverted: {dlo} .. {dhi}");
                                    assert!(
                                        (1..=damage::DAMAGE_CAP).contains(&dlo)
                                            && (1..=damage::DAMAGE_CAP).contains(&dhi),
                                        "outside 1..=99999: {dlo} .. {dhi}"
                                    );
                                    checked += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(checked, 6 * 5 * 5 * 5 * 5 * 3 * 3, "the sweep must actually have run");
    }

    /// Both ends of the clamp, reached rather than asserted about.
    #[test]
    fn both_ends_of_the_clamp_are_reachable() {
        let target = MagicTarget { magic_defence: 0, element_code: Some(0), level: 1 };

        // A huge cast is capped at the client's own 99999, not left to overflow.
        let huge = MagicAttacker {
            magic_total: 1_000_000,
            intelligence: 999,
            mastery: 10,
            skill_magic_percent: 1_000,
        };
        assert_eq!(magic_hit_window(&huge, &target, 10, CritStats::DEFAULT), (99_999, 99_999));
        assert_eq!(magic_ceiling(&huge, CritStats::DEFAULT), damage::DAMAGE_CAP);

        // A cast that arithmetically produces less than one still draws 1.
        let tiny = MagicAttacker {
            magic_total: 1,
            intelligence: 0,
            mastery: 0,
            skill_magic_percent: 10,
        };
        let (lo, hi) = magic_window(&tiny);
        assert!(hi < 1.0, "raw was {lo} .. {hi}, which must be below the floor for this test");
        assert_eq!(magic_hit_window(&tiny, &target, 10, CritStats::NONE), (1, 1));

        // An immune target (element code 1, factor 0.00) is the other way to get there.
        let immune = MagicTarget { element_code: Some(1), ..target };
        assert_eq!(magic_hit_window(&energy_bolt_lv1(), &immune, 10, CritStats::NONE), (1, 1));
    }

    // -- validation --------------------------------------------------------------------------

    /// The ceiling has to cover the worst *target*, not just the best roll - and the element
    /// ladder is the part a copy of `max_plausible_hit` would drop.
    #[test]
    fn the_ceiling_covers_every_target_the_cast_could_have_hit() {
        let a = energy_bolt_lv1();
        let ceiling = magic_ceiling(&a, CritStats::DEFAULT);

        // 25.2 raw, x1.5 weakest element, x1.2 crit = 45.36 -> 45.
        assert_eq!(ceiling, 45);

        // Nothing any legal target can produce may exceed it.
        for element in [None, Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)] {
            for defence in [0u32, 5, 500] {
                for mob_level in [1u32, 10, 40] {
                    let t = MagicTarget {
                        magic_defence: defence,
                        element_code: element,
                        level: mob_level,
                    };
                    let (_, hi) = magic_hit_window(&a, &t, 10, CritStats::DEFAULT);
                    assert!(hi <= ceiling, "{hi} > ceiling {ceiling} for {t:?}");
                }
            }
        }

        // An element-blind ceiling - the shape max_plausible_hit uses - would be too low.
        let element_blind = damage::finish(magic_window(&a).1 * damage::CRIT_MULTIPLIER);
        assert!(element_blind < ceiling, "{element_blind} would reject a legal weak-element cast");
    }

    #[test]
    fn a_claimed_hit_is_checked_against_the_ceiling_and_never_refused_outright() {
        let a = energy_bolt_lv1();
        for claimed in [1u64, 19, 25, 30, 45] {
            assert_eq!(
                check_magic_hit(claimed, Some(&a), CritStats::DEFAULT),
                damage::HitVerdict::Plausible,
                "{claimed}"
            );
        }
        assert_eq!(
            check_magic_hit(46, Some(&a), CritStats::DEFAULT),
            damage::HitVerdict::TooHigh { ceiling: 45 }
        );
    }

    /// The state every real call site is in today: no skill id in the attack packet, so no
    /// attacker can be built, so the answer is `Unchecked` - never a rejection.
    #[test]
    fn without_the_skill_id_there_is_no_attacker_and_the_verdict_is_unchecked() {
        assert_eq!(
            check_magic_hit(1_000_000, None, CritStats::DEFAULT),
            damage::HitVerdict::Unchecked
        );
    }
}
#[cfg(test)]
mod cobalt {
    use super::*;

    /// **The real observation, 2026-08-28: Magic Claw at level 7 dealt exactly 1.**
    ///
    /// The owner: *"I added all of the points into Magic Claw, but the skill only deals 1 damage,
    /// which is definitely not correct."* It is correct, and this test is why - the character
    /// is a **Rogue wearing a Magician's job id**. `!job 200` changed the number; it did not
    /// move a single ability point.
    ///
    /// ```text
    /// Cobalt, level 12, job 200:  STR 4  DEX 24  INT 6  LUK 36
    /// Magic Claw level 7:         mad 51   mastery 3   attackCount 2
    /// ```
    ///
    /// `MagicTotal` seeds from `floor(INT/2)` = **3**, and a Rogue's equipment carries no
    /// `incMAD`. So the whole window lands between 1 and 2 and `finish`'s truncation makes it
    /// **1** before the mob's magic defence has even been applied.
    ///
    /// This is the anchor that stops the next reader "fixing" a formula that is right. If this
    /// test ever fails, either the formula changed or the client's own data did.
    #[test]
    fn josiahs_magic_claw_predicts_the_1_he_saw() {
        let attacker = MagicAttacker {
            magic_total: magic_total_seed(6), // no wand, no incMAD
            intelligence: 6,
            mastery: 3,
            skill_magic_percent: 51,
        };
        assert_eq!(magic_total_seed(6), 3, "floor(INT/2)");

        let (lo, hi) = magic_window(&attacker);
        assert!(lo > 1.0 && hi < 2.5, "the RAW window is {lo}..{hi} - between 1 and 2");

        // Even against a defenceless, same-level target the whole window is 1..2.
        //
        // **The low end is 1 for a reason worth knowing**, and it is not the truncation: with
        // `element_code: None` the element bound spans 0.00..1.50, because an unknown target
        // might be immune. So the LOW end of any unknown-element window is 1 by construction,
        // and only the high end carries information here.
        let soft = MagicTarget { magic_defence: 0, element_code: None, level: 12 };
        let (win_lo, win_hi) = magic_hit_window(&attacker, &soft, 12, CritStats::NONE);
        assert_eq!(
            (win_lo, win_hi),
            (1, 2),
            "even the CEILING is 2 against a defenceless target - which is why every hit the owner              saw read 1 once a real mob's magic defence was applied"
        );

        // **And the skill is not the problem: INT is.** A Magician who had actually spent
        // their points hits for real, which makes this a diagnosis rather than an excuse.
        // Compared on the CEILING, since the floor is 1 whenever the element is unknown.
        let magician =
            MagicAttacker { magic_total: magic_total_seed(60), intelligence: 60, ..attacker };
        let (_, real_hi) = magic_hit_window(&magician, &soft, 12, CritStats::NONE);
        assert!(real_hi > 10 * win_hi, "60 INT ceilings at {real_hi} against Cobalt's {win_hi}");
    }
}
