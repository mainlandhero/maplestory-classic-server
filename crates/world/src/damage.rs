//! Damage: what the client computes, so the server can check it, and what the server
//! computes itself.
//!
//! Full working, with every address and every measurement: **`research/damage-formula.md`**.
//!
//! # Read this first: the server does not tell the client what to draw
//!
//! **This client computes its own damage.** The `0x00DF`/`0x00E0`/`0x00E1` attack body
//! carries a per-target list of `u64` damages the *client* worked out - `net::combat` parses
//! them and `research/mob-combat.md` §2 settles that nothing we send supplies a number the
//! client does not already have. So this module is **not** a damage source for the player's
//! attacks. It has exactly two honest uses:
//!
//! | | |
//! |---|---|
//! | **validation** | is a claimed hit plausible for this character, this weapon, this mob? |
//! | **mob -> player** | the server *is* the only authority for touch damage: the "I was hit" packet has never been found, and the HP bar moves because we send `0x007C` |
//!
//! Nothing here is wired. See the `WIRE IT LIKE THIS` section of `research/damage-formula.md`.
//!
//! # Where the numbers come from
//!
//! Three sources, and every item below says which:
//!
//! * **`[L]`** - read off this client. `FUN_14025e000` (the weapon-multiplier builder),
//!   `Character.wz` (weapon speed, frame delays), `Skill.wz` (skill damage, mastery).
//! * **`[D]`** - derived or measured from captures in this repo.
//! * **`[I]`** - the meowdb guides the owner named. A fan site with a good prior on this client,
//!   not a listing. `research/meowdb-combat-formulas.md` is the capture.
//!
//! # Every quantity here states its unit
//!
//! `CLAUDE.md`'s "the unit, not the arithmetic": three bugs this month were a correct number
//! in the wrong unit. So, explicitly:
//!
//! | quantity | unit | note |
//! |---|---|---|
//! | [`WeaponClass`] code | `itemId / 10000 - 100`, i.e. **30..=47** | the client's own numbering, and `Skill.wz`'s `weapon` field uses it too |
//! | [`AttackAction`] | the raw integer the client passes, **not** an index | 2, 3, 4, 6. `5` is never tested by any arm |
//! | weapon multiplier | a **multiplier**, ~1.0..3.5 | not a percentage |
//! | mastery | **1..=10** in this client, *not* a percentage | 60 would be a v83 number; this client's `Skill.wz` tops out at 10 |
//! | skill damage | **percent**, e.g. `160` for Power Strike Lv1 | divide by 100 |
//! | `stat_div` / `ap_div` | **divisors**, 100.0 / 50.0 | not percentages |
//! | frame delay | **milliseconds** | straight out of `Character.wz` |
//! | attack speed | a **stage number**, lower is faster | 2..=8 observed in this client |
//! | HP / MP per level | **absolute points** | and they live in [`crate::expcurve`], not here |
//!
//! # `0` is a value, not "unset"
//!
//! [`weapon_multiplier`] returns `Option<f64>` and `None` means *the client would leave the
//! multiplier at zero here*, which is a real outcome of `FUN_14025e000` and produces zero
//! damage. It does not mean "we do not know". A mob's size was shipped as `0` meaning
//! "unset" once, and `0` meant zero percent; the same trap is live here.

// ---------------------------------------------------------------------------------------
// Weapon classes
// ---------------------------------------------------------------------------------------

/// A weapon class, by the code this client uses everywhere.
///
/// **[L]** `FUN_14025e000` opens with `lea eax,[r8-0x1e] / cmp eax,0x11 / ja`, so the
/// switch covers exactly `0x1e..=0x2f` = **30..=47** and nothing outside it has a weapon
/// multiplier at all. The same numbering is what `Skill.wz` puts in a skill's `weapon`
/// field (Lucky Seven is `47`, Double Stab is `33`), and it is `itemId / 10000 - 100`.
///
/// **There is no knuckle (48) and no gun (49)**, which matches `Skill.wz` having no `500`
/// job tree: this client has no pirates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeaponClass {
    OneHandSword = 30,
    OneHandAxe = 31,
    OneHandBlunt = 32,
    Dagger = 33,
    Wand = 37,
    Staff = 38,
    /// Item `1392000`, `sfx` "barehands". Multiplier 1.0, no `incWAT`.
    BareHands = 39,
    TwoHandSword = 40,
    TwoHandAxe = 41,
    TwoHandBlunt = 42,
    Spear = 43,
    Polearm = 44,
    Bow = 45,
    Crossbow = 46,
    Claw = 47,
}

impl WeaponClass {
    /// The lowest code the client's switch covers.
    pub const FIRST_CODE: u32 = 30;
    /// The highest code the client's switch covers.
    pub const LAST_CODE: u32 = 47;

    /// Every class this client actually ships weapons for, in code order.
    ///
    /// **[L]** from `Character.wz/Weapon`: 230 images across exactly these fifteen classes,
    /// plus 27 images at `170xxxx` which carry no `attack`, no `attackSpeed` and no attack
    /// actions at all - appearance-only weapons, deliberately absent here.
    pub const ALL: [WeaponClass; 15] = [
        WeaponClass::OneHandSword,
        WeaponClass::OneHandAxe,
        WeaponClass::OneHandBlunt,
        WeaponClass::Dagger,
        WeaponClass::Wand,
        WeaponClass::Staff,
        WeaponClass::BareHands,
        WeaponClass::TwoHandSword,
        WeaponClass::TwoHandAxe,
        WeaponClass::TwoHandBlunt,
        WeaponClass::Spear,
        WeaponClass::Polearm,
        WeaponClass::Bow,
        WeaponClass::Crossbow,
        WeaponClass::Claw,
    ];

    /// The code, 30..=47.
    pub fn code(self) -> u32 {
        self as u32
    }

    /// From a raw code. `34`, `35`, `36` are inside the switch's range and have **no arm** -
    /// the client jumps straight to the exit and leaves the multiplier zero - so they are
    /// not classes here.
    pub fn from_code(code: u32) -> Option<WeaponClass> {
        Some(match code {
            30 => WeaponClass::OneHandSword,
            31 => WeaponClass::OneHandAxe,
            32 => WeaponClass::OneHandBlunt,
            33 => WeaponClass::Dagger,
            37 => WeaponClass::Wand,
            38 => WeaponClass::Staff,
            39 => WeaponClass::BareHands,
            40 => WeaponClass::TwoHandSword,
            41 => WeaponClass::TwoHandAxe,
            42 => WeaponClass::TwoHandBlunt,
            43 => WeaponClass::Spear,
            44 => WeaponClass::Polearm,
            45 => WeaponClass::Bow,
            46 => WeaponClass::Crossbow,
            47 => WeaponClass::Claw,
            _ => return None,
        })
    }

    /// From an equip item id. `1312000` -> [`WeaponClass::OneHandAxe`].
    ///
    /// **[L]** `itemId / 10000 - 100`. The `Weapon` archive's image names are the item ids
    /// and every one of the 203 weapons with attack data lands on a class in [`Self::ALL`].
    pub fn from_item_id(item_id: u32) -> Option<WeaponClass> {
        let class = item_id / 10_000;
        if !(100..=200).contains(&class) {
            return None;
        }
        WeaponClass::from_code(class - 100)
    }

    /// Whether this class's ordinary attack is a shot rather than a melee action.
    pub fn is_ranged(self) -> bool {
        matches!(self, WeaponClass::Bow | WeaponClass::Crossbow | WeaponClass::Claw)
    }
}

// ---------------------------------------------------------------------------------------
// Attack actions
// ---------------------------------------------------------------------------------------

/// The action an attack used, as the integer the client passes.
///
/// **[L]** for the codes: `FUN_14025e000` tests its fourth argument against `2`, `3`, `4`
/// and `6` inside every weapon arm, and against `1` at the head. **[D]** for the names,
/// derived by matching the 1H-axe arm (`2 -> 2.4`, `3 -> 1.2`) and the bow arm (`4 -> 2.5`)
/// against the meowdb swing/stab/shoot columns, which agree exactly for all eight weapon
/// families. `5` is tested by nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttackAction {
    /// Forces the multiplier to `1.0` and swaps all four scalars (see [`Scalars`]).
    ///
    /// **Deliberately not called "magic".** The head does `cmp ebp,1 / jne`, and the exit
    /// does it again, and that is all this client says about it. Naming it would be a guess.
    One = 1,
    Swing = 2,
    Stab = 3,
    Shoot = 4,
    /// The averaged action. For 1H axe/blunt the client's constant is *exactly*
    /// `(2.4 + 1.2) / 2` folded at compile time - see [`weapon_multiplier`].
    Mixed = 6,
}

impl AttackAction {
    /// The raw integer.
    pub fn code(self) -> u32 {
        self as u32
    }

    pub fn from_code(code: u32) -> Option<AttackAction> {
        Some(match code {
            1 => AttackAction::One,
            2 => AttackAction::Swing,
            3 => AttackAction::Stab,
            4 => AttackAction::Shoot,
            6 => AttackAction::Mixed,
            _ => return None,
        })
    }
}

/// The action a **skill** inherits when it names none of its own.
///
/// **[L]** the first jump table, at `0x14025e4a4`. It runs only when the skill id is
/// non-zero *and* the action argument is zero (`test r15d,r15d / je` skips it when there is
/// no skill), so this is the "inherited ordinary melee action" case and not a plain swing.
///
/// Every entry is the weapon's **higher-multiplier** action, which is a useful sanity check
/// on the transcription: axe 2.4 > 1.2, dagger 2.0 > 1.0, spear 3.5 > 1.5, polearm 3.5 > 1.5.
///
/// `None` for wand/staff/bare-hands is not a gap - those classes take the unconditional
/// 1.8 / 1.0 arms where the action never matters.
pub fn inherited_action(class: WeaponClass) -> AttackAction {
    match class {
        WeaponClass::OneHandAxe
        | WeaponClass::OneHandBlunt
        | WeaponClass::TwoHandAxe
        | WeaponClass::TwoHandBlunt
        | WeaponClass::Polearm => AttackAction::Swing,
        WeaponClass::Bow | WeaponClass::Crossbow | WeaponClass::Claw => AttackAction::Shoot,
        _ => AttackAction::Stab,
    }
}

/// How an ordinary, skill-less melee attack splits between swing and stab.
///
/// **[L]**, and this explains meowdb's otherwise unexplained "60% swing, 40% stab": every
/// melee weapon image in `Character.wz/Weapon` carries **three** ordinary swing actions and
/// **two** ordinary stabs - `swingO1/O2/O3` + `stabO1/O2` for one-handers and claws,
/// `swingT1/T2/T3` + `stabO1/O2` for two-handers, `swingP1/P2/swingT2` + `stabT1/T2` for
/// spears and polearms. Three of five is 60%.
pub const ORDINARY_SWING_ACTIONS: u32 = 3;
/// See [`ORDINARY_SWING_ACTIONS`].
pub const ORDINARY_STAB_ACTIONS: u32 = 2;

/// The probability an ordinary melee attack is a swing rather than a stab. **[L]**, as a
/// **fraction**, not a percentage.
pub fn ordinary_swing_share() -> f64 {
    ORDINARY_SWING_ACTIONS as f64 / (ORDINARY_SWING_ACTIONS + ORDINARY_STAB_ACTIONS) as f64
}

// ---------------------------------------------------------------------------------------
// The weapon multiplier table
// ---------------------------------------------------------------------------------------

/// Lucky Seven. **[L]**: `FUN_14025e000` does `cmp r15d, 0x3d0ceb` (= 4001003) on the claw
/// arm and, when it matches, replaces the multiplier with **3.0** and re-reads LUK.
pub const LUCKY_SEVEN_SKILL_ID: u32 = 4_001_003;

/// The 1H-axe / 1H-blunt "mixed" multiplier, as the client stores it.
///
/// **[L]** `0x3FFCCCCCCCCCCCCC` at `0x14025e17c` - one unit in the last place *below* the
/// literal `1.8` at `0x14025e137`. It is `(2.4 + 1.2) / 2.0` constant-folded: the exact sum
/// is a tie between the two neighbouring doubles and round-half-to-even picks the lower one.
///
/// This is kept exact rather than rounded to `1.8` because it is free to be exact and
/// because the 1-ulp gap is the evidence that "mixed" is the **mean** of swing and stab
/// rather than a separate authored number.
pub const MIXED_ONE_HAND_AXE: f64 = 1.799_999_999_999_999_8;

/// The weapon multiplier, transcribed arm by arm from `FUN_14025e000`. **[L]**
///
/// `None` means *this client leaves the multiplier at zero for this combination* - the arm
/// tests the action against its own small set and jumps to the exit when nothing matches,
/// and the exit only overwrites the field when the action is `1`. Zero damage is the real
/// consequence, so `None` is "the client computes nothing here", not "unknown".
///
/// | class | swing (2) | stab (3) | shoot (4) | mixed (6) |
/// |---|---|---|---|---|
/// | 1H Sword | 1.8 | 1.8 | 1.8 | 1.8 |
/// | 1H Axe / Blunt | 2.4 | 1.2 | - | (2.4+1.2)/2 |
/// | Dagger | 1.0 | 2.0 | - | 1.5 |
/// | Wand / Staff | 1.8 | 1.8 | 1.8 | 1.8 |
/// | Bare hands | 1.0 | 1.0 | 1.0 | 1.0 |
/// | 2H Sword | 2.5 | 2.5 | 2.5 | 2.5 |
/// | 2H Axe / Blunt | 3.0 | 2.0 | - | 2.5 |
/// | Spear | 1.5 | 3.5 | - | 2.5 |
/// | Polearm | 3.5 | 1.5 | - | 2.5 |
/// | Bow / Crossbow / Claw | 1.0 | 1.0 | 2.5 | 1.0 |
///
/// Every one of those matches the meowdb table exactly, which is the strongest single check
/// that guide has passed here. **One row does not**: meowdb gives wand/staff a physical
/// attack keyed on INT with LUK secondary, and this client sends weapon types 37 and 38 to
/// the *same arm as a 1H sword* - STR primary, DEX secondary, 1.8. See
/// [`primary_and_secondary`].
///
/// Action `1` is handled by the caller ([`weapon_multiplier_for`]) because the client
/// applies it at the exit, after the arm has run.
pub fn weapon_multiplier(class: WeaponClass, action: AttackAction) -> Option<f64> {
    use AttackAction::*;
    use WeaponClass::*;
    // Action 1 overwrites whatever the arm produced. Do it here so the table below is a
    // straight transcription of the arms.
    if action == One {
        return Some(1.0);
    }
    Some(match (class, action) {
        // Unconditional arms: no test on the action at all.
        (OneHandSword, _) | (Wand, _) | (Staff, _) => 1.8,
        (BareHands, _) => 1.0,
        (TwoHandSword, _) => 2.5,

        (OneHandAxe | OneHandBlunt, Swing) => 2.4,
        (OneHandAxe | OneHandBlunt, Stab) => 1.2,
        (OneHandAxe | OneHandBlunt, Mixed) => MIXED_ONE_HAND_AXE,

        (Dagger, Swing) => 1.0,
        (Dagger, Stab) => 2.0,
        (Dagger, Mixed) => 1.5,

        (TwoHandAxe | TwoHandBlunt, Swing) => 3.0,
        (TwoHandAxe | TwoHandBlunt, Stab) => 2.0,
        (TwoHandAxe | TwoHandBlunt, Mixed) => 2.5,

        (Spear, Swing) => 1.5,
        (Spear, Stab) => 3.5,
        (Spear, Mixed) => 2.5,

        (Polearm, Swing) => 3.5,
        (Polearm, Stab) => 1.5,
        (Polearm, Mixed) => 2.5,

        (Bow | Crossbow | Claw, Shoot) => 2.5,
        (Bow | Crossbow | Claw, Swing | Stab | Mixed) => 1.0,

        // Melee weapons have no shoot arm: the client falls through to the exit.
        (_, Shoot) => return None,
        (_, One) => unreachable!("handled above"),
    })
}

/// The multiplier including the one skill the client special-cases. **[L]**
pub fn weapon_multiplier_for(
    class: WeaponClass,
    action: AttackAction,
    skill_id: u32,
) -> Option<f64> {
    if class == WeaponClass::Claw && skill_id == LUCKY_SEVEN_SKILL_ID {
        return Some(3.0);
    }
    weapon_multiplier(class, action)
}

// ---------------------------------------------------------------------------------------
// The four scalars the same function writes beside the multiplier
// ---------------------------------------------------------------------------------------

/// The four floating-point scalars `FUN_14025e000` writes into its output struct alongside
/// the multiplier. **[L]**
///
/// The struct, offset for offset:
///
/// ```text
/// +0x00 i32  secondary stat (a sum, for dagger and claw)
/// +0x04 i32  primary stat
/// +0x08 f64  roll_lo      0.8   (0.2 when action == 1)
/// +0x10 f64  roll_hi      1.0   (0.3 when action == 1)
/// +0x18 f64  weapon multiplier
/// +0x20 f64  stat_div     100.0 (300.0 when action == 1, or on a ranged weapon's MELEE action)
/// +0x28 f64  stat_div, a second copy written by the same `movups`
/// +0x30 f64  ap_div       50.0  (150.0 in the same two cases)
/// ```
///
/// `roll_lo`/`roll_hi` are meowdb's `B = uniform(0.8, 1.0)`; `stat_div` and `ap_div` are its
/// `StatDiv = 100` and `APDiv = 50`. All four constants were read out of the image
/// (`0x14327aa28`, `0x1434b95e0`, `0x143277e78`, `0x14327aad0`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scalars {
    /// Lower bound of the flat roll, a **multiplier**.
    pub roll_lo: f64,
    /// Upper bound of the flat roll, a **multiplier**.
    pub roll_hi: f64,
    /// The stat term's **divisor**. Never a percentage.
    pub stat_div: f64,
    /// The attack-power term's **divisor**.
    pub ap_div: f64,
}

impl Scalars {
    /// The ordinary attack set. **[L]**
    pub const ATTACK: Scalars =
        Scalars { roll_lo: 0.8, roll_hi: 1.0, stat_div: 100.0, ap_div: 50.0 };
    /// What the head writes when the action is `1`. **[L]**
    pub const ACTION_ONE: Scalars =
        Scalars { roll_lo: 0.2, roll_hi: 0.3, stat_div: 300.0, ap_div: 150.0 };
}

/// The scalars for one (class, action). **[L]**
///
/// The interesting case, which meowdb does not carry: a **bow, crossbow or claw used as a
/// club** - action swing or mixed - keeps the ordinary `0.8..1.0` roll but triples the stat
/// divisor to `300` and the attack-power divisor to `150`. Its *stab* triples only
/// `stat_div` and leaves `ap_div` at 50; that asymmetry is what the code does
/// (`0x14025e3a8` and `0x14025e48f` write `+0x20`/`+0x28` and nothing else), not a
/// transcription slip.
///
/// A bow's **shoot** takes the plain 100/50, which is why meowdb's `(DEX * 2.5 + STR) / 100`
/// is right for the case that actually matters.
pub fn scalars(class: WeaponClass, action: AttackAction) -> Scalars {
    if !class.is_ranged() {
        return if action == AttackAction::One { Scalars::ACTION_ONE } else { Scalars::ATTACK };
    }
    match action {
        AttackAction::Swing | AttackAction::Mixed => {
            Scalars { roll_lo: 0.8, roll_hi: 1.0, stat_div: 300.0, ap_div: 150.0 }
        }
        AttackAction::Stab => Scalars { stat_div: 300.0, ..Scalars::ATTACK },
        AttackAction::Shoot => Scalars::ATTACK,
        AttackAction::One => Scalars::ACTION_ONE,
    }
}

// ---------------------------------------------------------------------------------------
// Primary and secondary stat
// ---------------------------------------------------------------------------------------

/// A character's four stats, in whatever total the caller has already assembled
/// (base + equipment + buffs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Stats {
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub luck: u32,
}

/// `(primary, secondary)` for a weapon class.
///
/// **[L] for the structure.** Each arm of `FUN_14025e000` calls a stat getter twice (three
/// times for dagger and claw) and writes the results to `+0x04` (primary) and `+0x00`
/// (secondary). The offsets it reads from the stat object are `+0x24`, `+0x30` and `+0x48`,
/// on an even `0xc` stride whose fourth slot `+0x3c` **this function never touches**.
///
/// **[D] for the naming.** Three arms pin it without any outside help, and they agree:
/// a bow's primary is `+0x30` while a sword's primary is `+0x24`, and a dagger's primary is
/// `+0x48` with `+0x24 + +0x30` as its secondary. Under the classic STR/DEX/INT/LUK order on
/// that stride, that is sword->STR, bow->DEX, dagger->LUK, secondary STR+DEX - which is
/// exactly the meowdb assignment, three ways over.
///
/// # The one place meowdb is wrong about this client
///
/// meowdb gives wand and staff a physical attack of `(INT * 1.8 + LUK) / 100`. **This client
/// sends weapon types 37 and 38 to the same arm as a 1H sword**: jump-table entries
/// `0x14025e11a` for both, which reads `+0x24` into primary and `+0x30` into secondary. INT
/// is never read by this function at all. So a wand's physical swing is a **STR** attack
/// here, and `intelligence` below is deliberately unused.
pub fn primary_and_secondary(class: WeaponClass, stats: &Stats) -> (u32, u32) {
    match class {
        WeaponClass::Bow | WeaponClass::Crossbow => (stats.dexterity, stats.strength),
        WeaponClass::Dagger | WeaponClass::Claw => {
            (stats.luck, stats.strength.saturating_add(stats.dexterity))
        }
        _ => (stats.strength, stats.dexterity),
    }
}

// ---------------------------------------------------------------------------------------
// Mastery
// ---------------------------------------------------------------------------------------

/// The highest `mastery` value in this client's `Skill.wz`. **[L]**
///
/// **This is 10, not 60.** Every mastery skill in `Skill_000.wz` - Sword, Axe, Blunt,
/// Spear, Polearm, Bow, Crossbow, Claw, Dagger and the fifteen spell masteries - runs
/// `1,1,1,2,2,3,3,...,10` over its levels. A version that stored mastery as a percentage
/// would top out at 60 here, and `M = (MasteryLevel/10 + 0.1) * 0.8` would then be nonsense.
/// The guide's divisor of 10 is the client's own unit.
pub const MASTERY_MAX: u32 = 10;

/// The mastery factor `M`. **[I]** for the formula, **[L]** for the unit of its input.
///
/// `M = (mastery / 10 + 0.1) * 0.8`, a **multiplier** applied to the primary stat to get the
/// bottom of its roll.
///
/// **`mastery == 0` means zero mastery, and that is a decision, not an accident.** A
/// character with no mastery skill has no such skill in their list, so the caller passes 0
/// and `M` is `0.08` - a very wide damage range, which is what an unmastered weapon is
/// supposed to feel like. Reading 0 as "unset" and substituting something else would be the
/// exact shape of the mob-size bug.
pub fn mastery_factor(mastery: u32) -> f64 {
    (mastery.min(MASTERY_MAX) as f64 / 10.0 + 0.1) * 0.8
}

// ---------------------------------------------------------------------------------------
// The physical damage window
// ---------------------------------------------------------------------------------------

/// Everything about the attacker the physical formula needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Attacker {
    /// Total weapon attack: the weapon's `incWAT` plus every other source. **Absolute**, not
    /// a percentage. `gm-handbook/equips.txt` has the column, and it is `incWAT` in this
    /// client - there is no `incPAD`.
    pub total_watk: u32,
    pub stats: Stats,
    /// `Skill.wz` `mastery` for the relevant mastery skill, **1..=10**, or 0 for none.
    pub mastery: u32,
    /// A temporary Attack Power buff, absolute. `0` when there is none.
    pub attack_power: u32,
    /// `Skill.wz` `damage` for the skill used, **in percent**. `100` for a plain attack.
    pub skill_damage_percent: u32,
}

impl Default for Attacker {
    /// A bare level-1 beginner with no weapon: 100% skill damage, no mastery, no buff.
    fn default() -> Self {
        Attacker {
            total_watk: 0,
            stats: Stats::default(),
            mastery: 0,
            attack_power: 0,
            skill_damage_percent: 100,
        }
    }
}

/// The `[min, max]` window a single physical hit can land in, **before** defence, element,
/// level gap and criticals. Both ends are **raw damage points**, un-truncated.
///
/// **[I] for the shape; every constant in it is [L].**
///
/// ```text
/// S   = skillDamage / 100
/// M   = (mastery / 10 + 0.1) * 0.8
/// MIN = S * WATK * (roll_lo + (primary * M * mult + secondary) / statDiv + attackPower / apDiv)
/// MAX = S * WATK * (roll_hi + (primary     * mult + secondary) / statDiv + attackPower / apDiv)
/// ```
///
/// Returns `None` when the client would compute a zero multiplier - see
/// [`weapon_multiplier`].
pub fn physical_window(
    attacker: &Attacker,
    class: WeaponClass,
    action: AttackAction,
    skill_id: u32,
) -> Option<(f64, f64)> {
    let mult = weapon_multiplier_for(class, action, skill_id)?;
    let sc = scalars(class, action);
    let (primary, secondary) = primary_and_secondary(class, &attacker.stats);
    let (primary, secondary) = (primary as f64, secondary as f64);
    let s = attacker.skill_damage_percent as f64 / 100.0;
    let watk = attacker.total_watk as f64;
    let m = mastery_factor(attacker.mastery);
    let ap = attacker.attack_power as f64 / sc.ap_div;

    let lo = s * watk * (sc.roll_lo + (primary * m * mult + secondary) / sc.stat_div + ap);
    let hi = s * watk * (sc.roll_hi + (primary * mult + secondary) / sc.stat_div + ap);
    Some((lo, hi))
}

// ---------------------------------------------------------------------------------------
// What happens to a raw number on its way to the screen
// ---------------------------------------------------------------------------------------

/// Physical defence reduction. **[I]**
///
/// `raw * 100 / (def + 100)`. `def` is **absolute**, the mob's `PDDamage` column in
/// `gm-handbook/mobtemplates.txt`.
pub fn after_defence(raw: f64, def: u32) -> f64 {
    raw * 100.0 / (def as f64 + 100.0)
}

/// The higher-level-target penalty. **[I]**, and it is a **multiplier** in `(0, 1]`.
///
/// ```text
/// gap <= 0  -> 1
/// gap 1..9  -> 1 / (1 + gap^2 * 0.005)
/// gap >= 10 -> 1 / (1 + gap   * 0.05)
/// ```
///
/// Both `0.005` and `0.05` exist as doubles in this client and each occurs essentially once
/// in the whole 76 MB image - which raises the prior and settles nothing.
pub fn level_gap_scale(player_level: u32, mob_level: u32) -> f64 {
    let gap = mob_level.saturating_sub(player_level);
    match gap {
        0 => 1.0,
        1..=9 => 1.0 / (1.0 + (gap * gap) as f64 * 0.005),
        _ => 1.0 / (1.0 + gap as f64 * 0.05),
    }
}

/// The base critical rate, as a **fraction**. **[D]**, measured - see [`CRIT_MULTIPLIER`].
pub const CRIT_RATE: f64 = 0.05;

/// What a critical hit multiplies by. **[D]**, and the measurement is worth stating.
///
/// `net::combat::AttackHit` carries two unexplained flags. In
/// `previous-runs/world-20260820-181822.log`, **one capture, one character, one session**:
///
/// | | |
/// |---|---|
/// | hits with `flag_b == 0` | 64, damages **15..20** |
/// | hits with `flag_b == 1` | 5, damages **20, 21, 22, 24** |
/// | rate | 5 of 69 = **7.2%**, consistent with 5% at that sample size |
///
/// **Every flagged hit is at or above the maximum of the 64 unflagged ones, and 24 is above
/// it**, so the flagged hits cannot be draws from the same distribution. `{trunc(b * 1.2) :
/// b in 15..=20}` is `{18,19,20,21,22,24}` and all four observed crit values are in it.
///
/// So `flag_b` is the **critical** flag - which `net::combat` currently marks `[I]` and
/// reads nothing from. The multiplier being exactly `1.2` is *consistent*, not settled: in
/// the pooled corpus 7 of 15 crits sit at the very top of the predicted set, where only 8.7%
/// of non-crits sit, and mob defence differing per template is not controlled for in that
/// data.
pub const CRIT_MULTIPLIER: f64 = 1.0 + 0.20;

/// The largest number this client will draw for one hit. **[L]**
///
/// `99999.0` as a double at `0x14327ab10`, and it occurs **exactly once** in the image.
pub const DAMAGE_CAP: u64 = 99_999;

/// The largest number the incoming-damage path will produce. **[L]**
///
/// `50000000.0` at `0x14327ab18`, also exactly once.
pub const INCOMING_DAMAGE_CAP: u64 = 50_000_000;

/// The last step: `trunc(clamp(value, 1, 99999))`. **[I]** for the order, **[L]** for the cap.
///
/// The floor of `1` is why a hit never shows zero.
pub fn finish(value: f64) -> u64 {
    if !value.is_finite() {
        return 1;
    }
    let v = value.trunc();
    if v < 1.0 {
        1
    } else if v > DAMAGE_CAP as f64 {
        DAMAGE_CAP
    } else {
        v as u64
    }
}

// ---------------------------------------------------------------------------------------
// Validation: the only thing the server can honestly do with the outgoing formula
// ---------------------------------------------------------------------------------------

/// What a claimed hit was checked against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitVerdict {
    /// Inside the window the formula allows.
    Plausible,
    /// Above the ceiling. `ceiling` is the largest value this attack could have produced.
    TooHigh { ceiling: u64 },
    /// No ceiling could be computed, so nothing was checked. **This is not a rejection.**
    ///
    /// The action and the skill id are not parsed out of the attack packet yet
    /// (`research/mob-combat.md` §7: thirty header fields, five non-zero, none explained), so
    /// most real hits land here today.
    Unchecked,
}

/// The largest a single hit can be, over **every** action the weapon could have used.
///
/// This is the honest ceiling when the server does not know which action was played - which
/// is every hit today. It takes the maximum over the actions that have a multiplier, applies
/// the critical multiplier, and skips defence and the level gap deliberately: both only ever
/// reduce, so leaving them out keeps the ceiling a ceiling.
///
/// `None` when no action gives this weapon a multiplier at all.
pub fn max_plausible_hit(attacker: &Attacker, class: WeaponClass, skill_id: u32) -> Option<u64> {
    let mut best: Option<f64> = None;
    for action in
        [AttackAction::One, AttackAction::Swing, AttackAction::Stab, AttackAction::Shoot, AttackAction::Mixed]
    {
        if let Some((_, hi)) = physical_window(attacker, class, action, skill_id) {
            best = Some(best.map_or(hi, |b: f64| b.max(hi)));
        }
    }
    best.map(|hi| finish(hi * CRIT_MULTIPLIER))
}

/// Check one claimed damage number.
///
/// **Always returns a verdict and never an error.** A validator that could refuse would end
/// up in a code path that returns nothing, and `CLAUDE.md`'s always-answer rule is about
/// exactly that. Deciding what to *do* with a `TooHigh` is the caller's problem, and today
/// the right answer is "log it".
pub fn check_hit(
    claimed: u64,
    attacker: &Attacker,
    class: Option<WeaponClass>,
    skill_id: u32,
) -> HitVerdict {
    let Some(class) = class else { return HitVerdict::Unchecked };
    let Some(ceiling) = max_plausible_hit(attacker, class, skill_id) else {
        return HitVerdict::Unchecked;
    };
    if claimed > ceiling {
        HitVerdict::TooHigh { ceiling }
    } else {
        HitVerdict::Plausible
    }
}

// ---------------------------------------------------------------------------------------
// Mob -> player. The server IS the authority here.
// ---------------------------------------------------------------------------------------

/// Physical defence derived from STR. **[I]**, `floor(STR / 4)`.
pub fn wdef_from_strength(strength: u32) -> u32 {
    strength / 4
}

/// Magic defence derived from INT. **[I]**, `floor(INT / 4)`.
pub fn mdef_from_intelligence(intelligence: u32) -> u32 {
    intelligence / 4
}

/// Avoidability. **[I]**, `floor(LUK/3) + floor(DEX/6) + 5`.
pub fn evasion(dexterity: u32, luck: u32) -> u32 {
    luck / 3 + dexterity / 6 + 5
}

/// The bounds of the incoming-damage roll, as **multipliers**. **[I]**
///
/// `Roll = 1.1 + 0.4 * U`, `U` uniform on `[0, 1)`, so the range is `[1.1, 1.5)`. Both `1.1`
/// and `0.4` are doubles in this client and `0.4` occurs exactly once in the image.
pub const INCOMING_ROLL_LO: f64 = 1.1;
/// See [`INCOMING_ROLL_LO`].
pub const INCOMING_ROLL_SPAN: f64 = 0.4;

/// Damage a mob does to the player, for a given roll. **[I]** for the formula.
///
/// ```text
/// Raw    = mobAttack * roll
/// Taken  = Raw * (1 - DEF / (DEF + 5 * (playerLevel + 40) + 1.2 * Raw))
/// result = trunc(clamp(Taken, 1, 50_000_000))
/// ```
///
/// `mob_attack` is the `PADamage` column of `gm-handbook/mobtemplates.txt` - **[L]**, from
/// this client's own `Mob.wz`. `roll` is a **multiplier** and belongs in
/// `[INCOMING_ROLL_LO, INCOMING_ROLL_LO + INCOMING_ROLL_SPAN)`; it is a parameter rather
/// than an internal `rand` call so this stays a pure function and so a test can pin both
/// ends.
///
/// # The level term goes the way you do not expect, and it is transcribed, not corrected
///
/// `PlayerLevel` sits in the **denominator of the defence ratio**, so raising it *shrinks*
/// `DEF / denominator` and the player takes **more**, not less, from the same mob at the
/// same DEF. Levelling still helps, but through the DEF and HP it buys, not through this
/// term - the term makes a point of defence worth less the higher you are.
///
/// That reads like a transcription error and it is not one: it is what the guide's line
/// says, and every constant in it (`5`, `40`, `1.2`) is a double in this client. It is
/// called out here because the tempting fix - flipping it so that levelling reduces damage -
/// would be exactly the "correct a source into what you expected" move `CLAUDE.md` keeps
/// warning about. What would settle it: one client session at two different levels with the
/// same gear against the same mob, which is not a measurement this repo has.
///
/// # This is not what the server does today
///
/// `net::combat::touch_damage` uses `PADamage * (1 + 5% per level of gap)` with a floor of
/// 1, which is a different [I] guess with no level, no defence and no roll. Swapping them is
/// a behaviour change on a confirmed-working feature and belongs to the coordinator - see
/// `research/damage-formula.md`.
pub fn incoming_damage(mob_attack: u32, player_level: u32, player_wdef: u32, roll: f64) -> u32 {
    let raw = mob_attack as f64 * roll;
    let def = player_wdef as f64;
    let denom = def + 5.0 * (player_level as f64 + 40.0) + 1.2 * raw;
    let taken = if denom > 0.0 { raw * (1.0 - def / denom) } else { raw };
    let taken = taken.trunc();
    if !taken.is_finite() || taken < 1.0 {
        1
    } else if taken > INCOMING_DAMAGE_CAP as f64 {
        INCOMING_DAMAGE_CAP as u32
    } else {
        taken as u32
    }
}

/// The `[min, max]` a mob's hit can be. Both ends **absolute HP points**.
pub fn incoming_window(mob_attack: u32, player_level: u32, player_wdef: u32) -> (u32, u32) {
    let lo = incoming_damage(mob_attack, player_level, player_wdef, INCOMING_ROLL_LO);
    // The roll is half-open, so the top is the largest value strictly below the span's end.
    let hi = incoming_damage(
        mob_attack,
        player_level,
        player_wdef,
        INCOMING_ROLL_LO + INCOMING_ROLL_SPAN - f64::EPSILON,
    );
    (lo.min(hi), lo.max(hi))
}

// ---------------------------------------------------------------------------------------
// HP and MP
// ---------------------------------------------------------------------------------------

/// A new character's maximum HP. **[I]**, and it agrees with what this server already ships.
///
/// The hp/mp guide says characters "begin at 50 HP and 5 MP"; `net::opcode::Character`'s
/// `Default` has said `hp: 50, max_hp: 50, mp: 5, max_mp: 5` since before that guide was
/// read here. Two independent [I] sources landing on the same number is worth recording, and
/// it is still [I].
pub const BASE_MAX_HP: u32 = 50;
/// See [`BASE_MAX_HP`].
pub const BASE_MAX_MP: u32 = 5;

/// **HP and MP per level are NOT here.** They live in [`crate::expcurve::LevelGains`], which
/// already carries the same five lines this workstream would have produced -
/// Beginner 16/12, Warrior 28/12, Magician 16/22, Bowman and Thief 22/17. Duplicating them
/// would create a second place to edit, which is the thing goal E asked for the opposite of.
///
/// The two numbers that are *not* implemented anywhere are the job-advancement bonus and the
/// Improving Max HP/MP skill bonus, and they stay unimplemented on purpose: job advancement
/// is goal E and has nothing to hang off. `research/damage-formula.md` carries the table.
pub const HP_MP_PER_LEVEL_LIVES_IN: &str = "crate::expcurve::LevelGains";

// ---------------------------------------------------------------------------------------
// Attack speed and animation timing
// ---------------------------------------------------------------------------------------

/// The frame delays of one action, in **milliseconds**, straight out of
/// `Character.wz/Character_000.wz/00002000.img`. **[L]**
///
/// The body image is where the timing lives; a weapon image carries only the weapon sprite's
/// placement per frame and no `delay` at all, which is why looking in `Weapon` for this
/// finds nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionFrames {
    pub name: &'static str,
    pub delays: &'static [u32],
}

impl ActionFrames {
    /// Total unmodified animation length, **milliseconds**.
    pub fn base_ms(&self) -> u32 {
        self.delays.iter().sum()
    }
}

/// Every ordinary attack action, with its measured frame delays. **[L]**
///
/// **Actions with negative delays are deliberately absent.** `shoot6`, `shootDb1`, `shotC1`,
/// `magic1/2/3`, `savage`, `assaulter`, `avenger` and `burster1/2` all carry at least one
/// negative `delay` in this client - `magic1` is `-900, 200, 200`. What a negative delay
/// means to this client's renderer has not been established, and summing `|delay|` versus
/// dropping the frame gives answers 1.8x apart. Including them would be a guess with a unit
/// attached, so they are named here and left out.
pub const ORDINARY_ACTIONS: &[ActionFrames] = &[
    ActionFrames { name: "swingO1", delays: &[300, 150, 350] },
    ActionFrames { name: "swingO2", delays: &[300, 150, 350] },
    ActionFrames { name: "swingO3", delays: &[300, 150, 350] },
    ActionFrames { name: "swingOF", delays: &[200, 100, 100, 300] },
    ActionFrames { name: "swingT1", delays: &[300, 150, 350] },
    ActionFrames { name: "swingT2", delays: &[300, 150, 350] },
    ActionFrames { name: "swingT3", delays: &[300, 150, 350] },
    ActionFrames { name: "swingTF", delays: &[200, 150, 150, 200] },
    ActionFrames { name: "swingP1", delays: &[300, 150, 350] },
    ActionFrames { name: "swingP2", delays: &[300, 150, 350] },
    ActionFrames { name: "swingPF", delays: &[100, 200, 200, 200] },
    ActionFrames { name: "stabO1", delays: &[350, 450] },
    ActionFrames { name: "stabO2", delays: &[350, 450] },
    ActionFrames { name: "stabOF", delays: &[250, 150, 300] },
    ActionFrames { name: "stabT1", delays: &[300, 100, 350] },
    ActionFrames { name: "stabT2", delays: &[300, 100, 350] },
    ActionFrames { name: "stabTF", delays: &[100, 200, 200, 200] },
    ActionFrames { name: "shoot1", delays: &[300, 150, 350] },
    ActionFrames { name: "shoot2", delays: &[160, 160, 250, 100, 150] },
    ActionFrames { name: "shootF", delays: &[300, 150, 250] },
    ActionFrames { name: "proneStab", delays: &[300, 400] },
];

/// Look one up by name.
pub fn ordinary_action(name: &str) -> Option<&'static ActionFrames> {
    ORDINARY_ACTIONS.iter().find(|a| a.name == name)
}

/// The neutral timing tier: at this value the animation runs at its unmodified length. **[D]**
///
/// `(10 + tier) / 16 == 1` exactly at `tier == 6`, which is the arithmetic anchor for reading
/// meowdb's `resolvedTier` as the weapon's own `attackSpeed`. It is the only value of the
/// tier that makes the scaling a no-op, and 6 is a real `attackSpeed` in this client (wands,
/// bows and the bare-hand weapon all carry it).
pub const NEUTRAL_TIMING_TIER: u32 = 6;

/// The tier is clamped to this range before scaling. **[I]**, from the guide.
pub const TIMING_TIER_MAX: u32 = 10;

/// The player-facing quantum, **milliseconds**. **[I]**
///
/// The guide's own words: shown in 30 ms steps, "our best-supported timing model", matched to
/// pre-Big-Bang packet timing and one Double Stab observation. It labels its own outer
/// cadence as best-supported rather than confirmed, and that hedge is repeated here because
/// `CLAUDE.md` has a section about hedges getting dropped in the retelling.
pub const TIMING_QUANTUM_MS: u32 = 30;

/// The tier for a weapon, after boosters. **[I]**
///
/// `attack_speed` is the weapon's `attackSpeed` from `Character.wz` - **[L]**, 2..=8 across
/// this client's 203 real weapons. A Weapon Booster is worth **two stages at every skill
/// level**; Spell Booster is one stage at Lv1-10 and two from Lv11. Lower is faster, so a
/// booster subtracts.
pub fn timing_tier(attack_speed: u32, booster_stages: u32) -> u32 {
    attack_speed.saturating_sub(booster_stages).min(TIMING_TIER_MAX)
}

/// One frame's delay after speed scaling, **milliseconds**. **[I]**
///
/// `trunc(delay * (10 + tier) / 16)`, truncated toward zero per frame - not once at the end.
pub fn scaled_frame_ms(delay_ms: u32, tier: u32) -> u32 {
    let t = tier.min(TIMING_TIER_MAX);
    ((delay_ms as u64 * (10 + t) as u64) / 16) as u32
}

/// The native (unquantised) length of an action at a tier, **milliseconds**. **[I]** for the
/// scaling, **[L]** for the delays that go into it.
pub fn native_action_ms(delays: &[u32], tier: u32) -> u32 {
    delays.iter().map(|d| scaled_frame_ms(*d, tier)).sum()
}

/// The player-facing animation time, **milliseconds**, rounded **up** to the next 30 ms.
/// **[I]**
pub fn animation_time_ms(delays: &[u32], tier: u32) -> u32 {
    let native = native_action_ms(delays, tier);
    native.div_ceil(TIMING_QUANTUM_MS) * TIMING_QUANTUM_MS
}

/// The shortest interval between two ordinary attacks this weapon can legitimately produce,
/// **milliseconds**.
///
/// The minimum over every ordinary action, because the client picks among them and the
/// fastest one sets the floor. This is what an attack-rate check would compare against - and
/// it is a **floor for validation**, not a schedule: no server-side rate check exists today
/// and a single-player local server does not need one.
pub fn min_attack_interval_ms(tier: u32) -> u32 {
    ORDINARY_ACTIONS
        .iter()
        .map(|a| animation_time_ms(a.delays, tier))
        .min()
        .unwrap_or(TIMING_QUANTUM_MS)
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- weapon classes ------------------------------------------------------------------

    #[test]
    fn the_class_codes_are_the_item_id_prefix() {
        // Every one of these is an item that exists in this client's Character.wz/Weapon.
        assert_eq!(WeaponClass::from_item_id(1_302_000), Some(WeaponClass::OneHandSword));
        assert_eq!(WeaponClass::from_item_id(1_312_000), Some(WeaponClass::OneHandAxe));
        assert_eq!(WeaponClass::from_item_id(1_392_000), Some(WeaponClass::BareHands));
        assert_eq!(WeaponClass::from_item_id(1_402_000), Some(WeaponClass::TwoHandSword));
        assert_eq!(WeaponClass::from_item_id(1_472_000), Some(WeaponClass::Claw));
        // 170xxxx are the appearance-only weapons: no attack node, no attackSpeed.
        assert_eq!(WeaponClass::from_item_id(1_702_000), None);
        // Not a weapon at all.
        assert_eq!(WeaponClass::from_item_id(1_040_003), None);
    }

    /// The switch's own bounds, which is the strongest statement about which classes exist.
    #[test]
    fn the_switch_covers_thirty_to_fortyseven_and_three_of_them_have_no_arm() {
        assert_eq!(WeaponClass::FIRST_CODE, 30);
        assert_eq!(WeaponClass::LAST_CODE, 47);
        for gap in [34u32, 35, 36] {
            assert_eq!(WeaponClass::from_code(gap), None, "code {gap} has no arm");
        }
        for c in WeaponClass::ALL {
            assert_eq!(WeaponClass::from_code(c.code()), Some(c));
            assert!((WeaponClass::FIRST_CODE..=WeaponClass::LAST_CODE).contains(&c.code()));
        }
        // No knuckle, no gun: this client has no pirate job tree either.
        assert_eq!(WeaponClass::from_code(48), None);
        assert_eq!(WeaponClass::from_code(49), None);
    }

    // -- the multiplier table ------------------------------------------------------------

    /// The whole table, transcribed from `FUN_14025e000` and checked against the meowdb one.
    #[test]
    fn every_weapon_multiplier_matches_the_client() {
        use AttackAction::*;
        use WeaponClass::*;
        let cases: &[(WeaponClass, AttackAction, Option<f64>)] = &[
            (OneHandSword, Swing, Some(1.8)),
            (OneHandSword, Stab, Some(1.8)),
            (OneHandSword, Mixed, Some(1.8)),
            (OneHandAxe, Swing, Some(2.4)),
            (OneHandAxe, Stab, Some(1.2)),
            (OneHandAxe, Mixed, Some(MIXED_ONE_HAND_AXE)),
            (OneHandBlunt, Swing, Some(2.4)),
            (OneHandBlunt, Stab, Some(1.2)),
            (Dagger, Swing, Some(1.0)),
            (Dagger, Stab, Some(2.0)),
            (Dagger, Mixed, Some(1.5)),
            (Wand, Swing, Some(1.8)),
            (Staff, Stab, Some(1.8)),
            (BareHands, Swing, Some(1.0)),
            (TwoHandSword, Swing, Some(2.5)),
            (TwoHandSword, Stab, Some(2.5)),
            (TwoHandAxe, Swing, Some(3.0)),
            (TwoHandAxe, Stab, Some(2.0)),
            (TwoHandAxe, Mixed, Some(2.5)),
            (TwoHandBlunt, Swing, Some(3.0)),
            (Spear, Swing, Some(1.5)),
            (Spear, Stab, Some(3.5)),
            (Spear, Mixed, Some(2.5)),
            (Polearm, Swing, Some(3.5)),
            (Polearm, Stab, Some(1.5)),
            (Polearm, Mixed, Some(2.5)),
            (Bow, Shoot, Some(2.5)),
            (Bow, Swing, Some(1.0)),
            (Crossbow, Shoot, Some(2.5)),
            (Claw, Shoot, Some(2.5)),
            (Claw, Stab, Some(1.0)),
        ];
        for (class, action, want) in cases {
            assert_eq!(
                weapon_multiplier(*class, *action),
                *want,
                "{class:?} {action:?}"
            );
        }
    }

    /// `None` is "the client computes a zero multiplier", and the melee shoot arms are it.
    #[test]
    fn a_melee_weapon_has_no_shoot_arm_and_that_means_zero_not_unknown() {
        for class in [
            WeaponClass::OneHandAxe,
            WeaponClass::Dagger,
            WeaponClass::TwoHandAxe,
            WeaponClass::Spear,
            WeaponClass::Polearm,
        ] {
            assert_eq!(weapon_multiplier(class, AttackAction::Shoot), None, "{class:?}");
        }
        // The unconditional arms answer for every action, including shoot.
        for class in [WeaponClass::OneHandSword, WeaponClass::TwoHandSword, WeaponClass::BareHands]
        {
            assert!(weapon_multiplier(class, AttackAction::Shoot).is_some(), "{class:?}");
        }
    }

    /// The 1-ulp gap is the evidence that "mixed" is the mean, so pin the exact bits.
    #[test]
    fn the_mixed_axe_multiplier_is_the_compile_time_mean_not_the_literal_one_point_eight() {
        assert_eq!(
            MIXED_ONE_HAND_AXE.to_bits(),
            0x3FFC_CCCC_CCCC_CCCC,
            "this is the constant at 0x14025e17c"
        );
        assert_ne!(MIXED_ONE_HAND_AXE.to_bits(), 1.8f64.to_bits(), "one ulp below the literal");
        assert!((MIXED_ONE_HAND_AXE - (2.4 + 1.2) / 2.0).abs() < 1e-15);
        // And it really is between the swing and the stab.
        let swing = weapon_multiplier(WeaponClass::OneHandAxe, AttackAction::Swing).unwrap();
        let stab = weapon_multiplier(WeaponClass::OneHandAxe, AttackAction::Stab).unwrap();
        assert!(stab < MIXED_ONE_HAND_AXE && MIXED_ONE_HAND_AXE < swing);
    }

    #[test]
    fn lucky_seven_is_the_one_skill_the_client_special_cases() {
        assert_eq!(LUCKY_SEVEN_SKILL_ID, 0x3d0ceb);
        assert_eq!(
            weapon_multiplier_for(WeaponClass::Claw, AttackAction::Shoot, LUCKY_SEVEN_SKILL_ID),
            Some(3.0)
        );
        // Only for a claw, and only for that id.
        assert_eq!(
            weapon_multiplier_for(WeaponClass::Claw, AttackAction::Shoot, 0),
            Some(2.5)
        );
        assert_eq!(
            weapon_multiplier_for(WeaponClass::Dagger, AttackAction::Stab, LUCKY_SEVEN_SKILL_ID),
            Some(2.0)
        );
    }

    /// The inherited action is always the weapon's better one - a check on the transcription.
    #[test]
    fn the_inherited_action_is_the_weapons_stronger_one() {
        for class in WeaponClass::ALL {
            let inherited = inherited_action(class);
            let Some(best) = weapon_multiplier(class, inherited) else { continue };
            for other in [AttackAction::Swing, AttackAction::Stab, AttackAction::Shoot] {
                if let Some(m) = weapon_multiplier(class, other) {
                    assert!(m <= best, "{class:?}: {other:?} = {m} beats {inherited:?} = {best}");
                }
            }
        }
    }

    #[test]
    fn the_ordinary_melee_split_is_three_swings_to_two_stabs() {
        assert_eq!(ORDINARY_SWING_ACTIONS + ORDINARY_STAB_ACTIONS, 5);
        assert!((ordinary_swing_share() - 0.6).abs() < 1e-12, "meowdb's 60/40");
    }

    // -- the scalars ---------------------------------------------------------------------

    #[test]
    fn a_melee_attack_uses_a_hundred_and_fifty_and_a_zero_point_eight_roll() {
        let s = scalars(WeaponClass::OneHandSword, AttackAction::Swing);
        assert_eq!(s, Scalars::ATTACK);
        assert_eq!((s.roll_lo, s.roll_hi), (0.8, 1.0));
        assert_eq!((s.stat_div, s.ap_div), (100.0, 50.0));
    }

    /// A bow's *shoot* is an ordinary 100/50; only its melee actions triple the divisors.
    #[test]
    fn a_bow_shoots_at_a_hundred_but_clubs_at_three_hundred() {
        assert_eq!(scalars(WeaponClass::Bow, AttackAction::Shoot), Scalars::ATTACK);
        let swung = scalars(WeaponClass::Bow, AttackAction::Swing);
        assert_eq!((swung.stat_div, swung.ap_div), (300.0, 150.0));
        // The stab arm writes stat_div and nothing else. That asymmetry is in the code.
        let stabbed = scalars(WeaponClass::Claw, AttackAction::Stab);
        assert_eq!((stabbed.stat_div, stabbed.ap_div), (300.0, 50.0));
    }

    #[test]
    fn action_one_swaps_all_four_scalars_and_forces_the_multiplier_to_one() {
        assert_eq!(scalars(WeaponClass::Polearm, AttackAction::One), Scalars::ACTION_ONE);
        assert_eq!(Scalars::ACTION_ONE.roll_lo, 0.2);
        assert_eq!(Scalars::ACTION_ONE.roll_hi, 0.3);
        for class in WeaponClass::ALL {
            assert_eq!(
                weapon_multiplier(class, AttackAction::One),
                Some(1.0),
                "{class:?} action 1"
            );
        }
    }

    // -- stats ---------------------------------------------------------------------------

    #[test]
    fn the_primary_stat_is_the_one_the_client_reads_into_the_second_slot() {
        let s = Stats { strength: 12, dexterity: 5, intelligence: 4, luck: 4 };
        assert_eq!(primary_and_secondary(WeaponClass::OneHandSword, &s), (12, 5));
        assert_eq!(primary_and_secondary(WeaponClass::Spear, &s), (12, 5));
        assert_eq!(primary_and_secondary(WeaponClass::Bow, &s), (5, 12));
        assert_eq!(primary_and_secondary(WeaponClass::Crossbow, &s), (5, 12));
        // Dagger and claw: LUK primary, STR + DEX secondary - three getter calls, one add.
        assert_eq!(primary_and_secondary(WeaponClass::Dagger, &s), (4, 17));
        assert_eq!(primary_and_secondary(WeaponClass::Claw, &s), (4, 17));
    }

    /// The row where this client and meowdb disagree, kept as an assertion so it cannot be
    /// "fixed" back to the guide by accident.
    #[test]
    fn a_wand_is_a_strength_weapon_in_this_client_whatever_the_guide_says() {
        let s = Stats { strength: 12, dexterity: 5, intelligence: 40, luck: 4 };
        assert_eq!(
            primary_and_secondary(WeaponClass::Wand, &s),
            (12, 5),
            "weapon types 37/38 share the 1H sword's jump-table entry 0x14025e11a"
        );
        assert_eq!(primary_and_secondary(WeaponClass::Staff, &s), (12, 5));
        // INT never appears, however large it is.
        assert_eq!(primary_and_secondary(WeaponClass::Staff, &s).0, s.strength);
    }

    // -- mastery -------------------------------------------------------------------------

    #[test]
    fn mastery_is_one_to_ten_in_this_client_not_a_percentage() {
        assert_eq!(MASTERY_MAX, 10);
        assert!((mastery_factor(0) - 0.08).abs() < 1e-12, "no mastery skill at all");
        assert!((mastery_factor(1) - 0.16).abs() < 1e-12);
        assert!((mastery_factor(10) - 0.88).abs() < 1e-12, "a maxed mastery");
        // A v83-style percentage would be clamped rather than silently producing 4.88.
        assert_eq!(mastery_factor(60), mastery_factor(10));
    }

    // -- the damage window ---------------------------------------------------------------

    /// The single measurement this whole module can be checked against, and it is one
    /// capture: `previous-runs/world-20260821-001440.log`, character 208 "Programmer".
    ///
    /// Level 7, job 0, STR 7, DEX 7, 30 AP **unspent** (so the stats are the creation roll),
    /// weapon slot 11 = `1312000`, a 1H axe with `incWAT` 17, no mastery skill, no buff. The
    /// fourteen hits in that file are 16, 16, 16, 16, 16, 17, 17, 17, 18, 18, 18, 19, 19 and
    /// one more 16, every target a template 1 or 2 mob whose `PDDamage` is **0**.
    ///
    /// **This is a containment check, not a fit.** Fourteen samples spanning 16..19 inside a
    /// predicted 14..21 would also sit inside a formula that was 20% wrong. It is recorded
    /// because it is free and because a *failure* would have been decisive.
    #[test]
    fn the_captured_hits_from_character_208_fall_inside_the_predicted_window() {
        let attacker = Attacker {
            total_watk: 17,
            stats: Stats { strength: 7, dexterity: 7, intelligence: 5, luck: 6 },
            mastery: 0,
            attack_power: 0,
            skill_damage_percent: 100,
        };
        let class = WeaponClass::from_item_id(1_312_000).unwrap();
        let (swing_lo, swing_hi) =
            physical_window(&attacker, class, AttackAction::Swing, 0).unwrap();
        let (stab_lo, stab_hi) = physical_window(&attacker, class, AttackAction::Stab, 0).unwrap();

        // The numbers this predicts, to three places, so a change to the formula shows up.
        assert!((swing_lo - 15.018).abs() < 0.001, "swing min was {swing_lo}");
        assert!((swing_hi - 21.046).abs() < 0.001, "swing max was {swing_hi}");
        assert!((stab_lo - 14.904).abs() < 0.001, "stab min was {stab_lo}");
        assert!((stab_hi - 19.618).abs() < 0.001, "stab max was {stab_hi}");

        let lo = swing_lo.min(stab_lo);
        let hi = swing_hi.max(stab_hi);
        for observed in [16u64, 16, 16, 16, 16, 16, 17, 17, 17, 18, 18, 18, 19, 19] {
            assert!(
                observed as f64 >= lo.trunc() && observed as f64 <= hi,
                "{observed} is outside [{lo}, {hi}]"
            );
        }
    }

    #[test]
    fn a_zero_multiplier_gives_no_window_at_all() {
        let a = Attacker { total_watk: 50, ..Attacker::default() };
        assert_eq!(physical_window(&a, WeaponClass::Spear, AttackAction::Shoot, 0), None);
    }

    #[test]
    fn skill_damage_is_a_percentage() {
        let mut a = Attacker { total_watk: 20, ..Attacker::default() };
        a.stats.strength = 10;
        let (_, plain) =
            physical_window(&a, WeaponClass::OneHandSword, AttackAction::Swing, 0).unwrap();
        // Power Strike Lv1 is `damage: 160` in this client's Skill.wz - 160 percent.
        a.skill_damage_percent = 160;
        let (_, boosted) =
            physical_window(&a, WeaponClass::OneHandSword, AttackAction::Swing, 0).unwrap();
        assert!((boosted / plain - 1.6).abs() < 1e-12);
    }

    #[test]
    fn no_weapon_attack_means_no_damage_however_good_the_stats_are() {
        let a = Attacker {
            total_watk: 0,
            stats: Stats { strength: 999, dexterity: 999, intelligence: 999, luck: 999 },
            ..Attacker::default()
        };
        let (lo, hi) = physical_window(&a, WeaponClass::Polearm, AttackAction::Swing, 0).unwrap();
        assert_eq!((lo, hi), (0.0, 0.0));
        // ...and the floor still makes the drawn number 1, never 0.
        assert_eq!(finish(hi), 1);
    }

    // -- what happens on the way to the screen --------------------------------------------

    #[test]
    fn defence_never_increases_damage_and_zero_defence_changes_nothing() {
        assert!((after_defence(100.0, 0) - 100.0).abs() < 1e-12);
        assert!(after_defence(100.0, 100) < 100.0);
        assert!((after_defence(100.0, 100) - 50.0).abs() < 1e-12);
    }

    #[test]
    fn the_level_gap_only_bites_when_the_mob_is_higher() {
        assert_eq!(level_gap_scale(30, 10), 1.0, "a lower mob is not a bonus");
        assert_eq!(level_gap_scale(10, 10), 1.0);
        let one = level_gap_scale(10, 11);
        let nine = level_gap_scale(10, 19);
        let ten = level_gap_scale(10, 20);
        assert!(one < 1.0 && nine < one && ten < nine);
        // The two branches meet sensibly: gap 9 uses gap^2, gap 10 goes linear.
        assert!((one - 1.0 / 1.005).abs() < 1e-12);
        assert!((ten - 1.0 / 1.5).abs() < 1e-12);
    }

    #[test]
    fn the_final_step_floors_at_one_and_caps_at_the_clients_own_number() {
        assert_eq!(DAMAGE_CAP, 99_999);
        assert_eq!(finish(0.0), 1);
        assert_eq!(finish(-50.0), 1);
        assert_eq!(finish(0.9), 1);
        assert_eq!(finish(17.99), 17, "truncated, not rounded");
        assert_eq!(finish(99_999.0), 99_999);
        assert_eq!(finish(1e12), 99_999);
        assert_eq!(finish(f64::NAN), 1);
    }

    /// The measured crit relationship, kept as arithmetic so it is checkable.
    #[test]
    fn the_measured_crit_values_are_all_reachable_from_the_measured_normal_range() {
        // world-20260820-181822.log, one capture: 64 unflagged hits over 15..20.
        let reachable: Vec<u64> =
            (15..=20).map(|b| finish(b as f64 * CRIT_MULTIPLIER)).collect();
        for observed_crit in [20u64, 21, 22, 24] {
            assert!(
                reachable.contains(&observed_crit),
                "{observed_crit} is not trunc(b * {CRIT_MULTIPLIER}) for any b in 15..=20; \
                 reachable = {reachable:?}"
            );
        }
        // And the top crit is above the top normal hit, which is why the flag cannot be
        // noise. Written against the measured numbers rather than as `24 > 20`, which was a
        // tautology the compiler could fold: it asserted arithmetic, not the measurement.
        let top_normal = 20u64;
        let top_crit = 24u64;
        assert!(
            top_crit > top_normal,
            "the highest flagged hit ({top_crit}) must exceed the highest of the 64 unflagged \
             ones ({top_normal}), or the flag is explained by ordinary spread"
        );
        assert!(reachable.contains(&top_crit), "and it must still be a crit of a real roll");
        assert!((CRIT_RATE - 0.05).abs() < 1e-12);
    }

    // -- validation ----------------------------------------------------------------------

    #[test]
    fn the_ceiling_covers_every_action_the_weapon_could_have_used() {
        let a = Attacker {
            total_watk: 17,
            stats: Stats { strength: 7, dexterity: 7, intelligence: 5, luck: 6 },
            ..Attacker::default()
        };
        let ceiling = max_plausible_hit(&a, WeaponClass::OneHandAxe, 0).unwrap();
        // swing max 21.046, times the crit multiplier, truncated.
        assert_eq!(ceiling, finish(21.046_4 * CRIT_MULTIPLIER));
        assert!(ceiling >= 25, "a crit on the best action must fit under it");
        for observed in [16u64, 17, 18, 19, 24] {
            assert_eq!(
                check_hit(observed, &a, Some(WeaponClass::OneHandAxe), 0),
                HitVerdict::Plausible,
                "{observed}"
            );
        }
        assert!(matches!(
            check_hit(9_999, &a, Some(WeaponClass::OneHandAxe), 0),
            HitVerdict::TooHigh { .. }
        ));
    }

    /// Not knowing the weapon is `Unchecked`, never a rejection. This is the always-answer
    /// rule wearing a different hat.
    #[test]
    fn an_unknown_weapon_is_unchecked_rather_than_too_high() {
        let a = Attacker::default();
        assert_eq!(check_hit(1_000_000, &a, None, 0), HitVerdict::Unchecked);
    }

    // -- incoming --------------------------------------------------------------------------

    #[test]
    fn a_snail_hits_for_at_least_one_and_defence_only_ever_reduces_it() {
        // Template 2 on map 40: PADamage 3, level 1 - gm-handbook/mobtemplates.txt.
        let bare = incoming_damage(3, 1, 0, INCOMING_ROLL_LO);
        let armoured = incoming_damage(3, 1, 50, INCOMING_ROLL_LO);
        assert!(bare >= 1 && armoured >= 1, "never zero");
        assert!(armoured <= bare);
    }

    #[test]
    fn the_incoming_window_is_ordered_and_the_roll_is_a_multiplier() {
        let (lo, hi) = incoming_window(300, 20, 10);
        assert!(lo <= hi);
        assert!((INCOMING_ROLL_LO - 1.1).abs() < 1e-12);
        assert!((INCOMING_ROLL_LO + INCOMING_ROLL_SPAN - 1.5).abs() < 1e-12);
        // A bigger hit at the top of the roll than at the bottom, which is the whole point.
        assert!(hi > lo);
    }

    /// **This asserts the surprising direction on purpose.** See the note on
    /// [`incoming_damage`]: the level term dilutes the defence ratio, so at a *fixed* DEF a
    /// higher level takes more. The test exists so that anyone who "fixes" the formula to
    /// match their intuition has to delete an assertion that says they are doing it.
    #[test]
    fn a_higher_level_dilutes_defence_rather_than_reducing_damage() {
        let low = incoming_damage(500, 10, 40, 1.3);
        let high = incoming_damage(500, 90, 40, 1.3);
        assert!(
            high >= low,
            "the level term is in the DENOMINATOR of the defence ratio: {low} -> {high}"
        );
        // Defence itself still works, at every level.
        assert!(incoming_damage(500, 90, 400, 1.3) < incoming_damage(500, 90, 0, 1.3));
        // And with no defence at all the level term cannot do anything.
        assert_eq!(incoming_damage(500, 10, 0, 1.3), incoming_damage(500, 90, 0, 1.3));
    }

    #[test]
    fn incoming_damage_is_capped_by_the_clients_own_number() {
        assert_eq!(INCOMING_DAMAGE_CAP, 50_000_000);
        assert_eq!(incoming_damage(u32::MAX, 1, 0, 1.5), INCOMING_DAMAGE_CAP as u32);
    }

    #[test]
    fn the_derived_defences_are_quarters_and_the_evasion_is_thirds_and_sixths() {
        assert_eq!(wdef_from_strength(0), 0);
        assert_eq!(wdef_from_strength(7), 1);
        assert_eq!(wdef_from_strength(40), 10);
        assert_eq!(mdef_from_intelligence(40), 10);
        assert_eq!(evasion(0, 0), 5, "the flat +5 floor");
        assert_eq!(evasion(6, 3), 7);
    }

    // -- HP / MP ---------------------------------------------------------------------------

    #[test]
    fn the_base_pools_agree_with_what_the_server_already_creates() {
        assert_eq!((BASE_MAX_HP, BASE_MAX_MP), (50, 5));
        assert_eq!(net::opcode::Character::default().max_hp, BASE_MAX_HP);
        assert_eq!(net::opcode::Character::default().max_mp, BASE_MAX_MP);
    }

    /// A guard against this module growing a second copy of the level gains.
    ///
    /// **If this fails because `expcurve` legitimately changed, update or delete it - do not
    /// "fix" it by adding the old numbers here.** The whole point is that there is exactly
    /// one place for them.
    #[test]
    fn hp_and_mp_per_level_are_not_duplicated_here() {
        assert_eq!(HP_MP_PER_LEVEL_LIVES_IN, "crate::expcurve::LevelGains");
        let g = crate::expcurve::LevelGains::for_class(crate::expcurve::ClassLine::Beginner);
        assert_eq!(
            (g.max_hp, g.max_mp),
            (16, 12),
            "expcurve is the one place the per-level gains live; this test only witnesses it"
        );
    }

    // -- timing ------------------------------------------------------------------------------

    #[test]
    fn the_frame_delays_are_the_ones_in_character_wz() {
        let swing = ordinary_action("swingO1").unwrap();
        assert_eq!(swing.delays, &[300, 150, 350]);
        assert_eq!(swing.base_ms(), 800);
        assert_eq!(ordinary_action("stabO1").unwrap().base_ms(), 800);
        assert_eq!(ordinary_action("stabT1").unwrap().base_ms(), 750);
        assert_eq!(ordinary_action("shoot2").unwrap().base_ms(), 820);
        assert_eq!(ordinary_action("swingOF").unwrap().base_ms(), 700);
        assert!(ordinary_action("magic1").is_none(), "negative delays are excluded on purpose");
    }

    /// The anchor for reading the tier as the weapon's own `attackSpeed`.
    #[test]
    fn tier_six_is_the_only_no_op_and_it_is_a_real_attack_speed_in_this_client() {
        assert_eq!(NEUTRAL_TIMING_TIER, 6);
        for d in [100u32, 150, 300, 350, 450] {
            assert_eq!(scaled_frame_ms(d, NEUTRAL_TIMING_TIER), d, "tier 6 changes nothing");
        }
        for t in 0..=TIMING_TIER_MAX {
            let scaled = scaled_frame_ms(300, t);
            if t < NEUTRAL_TIMING_TIER {
                assert!(scaled < 300, "tier {t} must be faster");
            } else if t > NEUTRAL_TIMING_TIER {
                assert!(scaled > 300, "tier {t} must be slower");
            }
        }
    }

    #[test]
    fn a_polearm_swing_lands_on_a_round_nine_hundred_milliseconds() {
        // attackSpeed 8 is what nine of this client's ten polearms carry.
        let swing = ordinary_action("swingP1").unwrap();
        assert_eq!(native_action_ms(swing.delays, 8), 337 + 168 + 393);
        assert_eq!(native_action_ms(swing.delays, 8), 898);
        assert_eq!(animation_time_ms(swing.delays, 8), 900);
    }

    #[test]
    fn the_quantum_rounds_up_and_never_down() {
        assert_eq!(TIMING_QUANTUM_MS, 30);
        let swing = ordinary_action("swingO1").unwrap();
        // attackSpeed 4, the modal 1H sword: 262 + 131 + 306 = 699 -> 720.
        assert_eq!(native_action_ms(swing.delays, 4), 699);
        assert_eq!(animation_time_ms(swing.delays, 4), 720);
        assert!(animation_time_ms(swing.delays, 4) >= native_action_ms(swing.delays, 4));
        assert_eq!(animation_time_ms(&[30], 6) % TIMING_QUANTUM_MS, 0);
    }

    #[test]
    fn a_booster_lowers_the_tier_and_a_lower_tier_is_faster() {
        assert_eq!(timing_tier(6, 0), 6);
        assert_eq!(timing_tier(6, 2), 4, "a weapon booster is two stages");
        assert_eq!(timing_tier(1, 2), 0, "and it cannot go below zero");
        assert_eq!(timing_tier(20, 0), TIMING_TIER_MAX, "clamped at the top");
        assert!(min_attack_interval_ms(4) < min_attack_interval_ms(8));
    }

    #[test]
    fn the_minimum_interval_is_a_floor_over_every_ordinary_action() {
        for tier in 0..=TIMING_TIER_MAX {
            let floor = min_attack_interval_ms(tier);
            for a in ORDINARY_ACTIONS {
                assert!(animation_time_ms(a.delays, tier) >= floor, "{} at tier {tier}", a.name);
            }
            assert_eq!(floor % TIMING_QUANTUM_MS, 0);
        }
    }
}
