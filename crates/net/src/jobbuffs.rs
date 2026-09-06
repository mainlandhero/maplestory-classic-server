//! The first-job stat buffs of the Warrior, Bowman and Thief branches.
//!
//! [`crate::buff`] models the beginner skill and the two Magician ones. This module is the
//! other three branches, and it exists as a separate file only because `buff.rs` is shared -
//! everything here re-uses [`TemporaryStat`], [`BuffLevel`], [`BuffDuration`] and
//! [`StatGrant`] from it and adds no wire format of its own.
//!
//! # The four skills, and only three of them are a player stat
//!
//! | skill | id | what it sets | bit |
//! |---|---|---|---|
//! | Iron Body | `1001000` | Weapon Def., **as a PERCENTAGE** | [`crate::buff::CTS_WEAPON_DEFENCE`] = 86 |
//! | Focus | `3001000` | Accuracy **and** Avoidability, flat | [`CTS_ACCURACY`] = 88, [`CTS_AVOIDABILITY`] = 89 |
//! | Dark Sight | `4001001` | invisibility, **and** a Speed penalty | [`CTS_DARK_SIGHT`] = 99, [`crate::buff::CTS_SPEED`] = 92 |
//! | Disorder | `4001000` | a debuff **on the mob** | none - see [`DISORDER`] |
//!
//! # The bit numbers came out of a name table this file re-derived, with its controls
//!
//! `research/first-job-buffs.md` §2. `.rdata` holds the plain-ASCII temporary-stat names;
//! nothing in data points at them, so they are recovered by pairing every rip-relative
//! `lea` in `.text` with the `mov dword [rsp+D], imm32` that writes the CTS index into the
//! *same* stack slot the surrounding block compares against `0x52` (82). That pairing is
//! structural rather than a proximity guess, and the run reports:
//!
//! ```text
//! 211 080 rip-relative lea sites -> 880 structural pairings
//! 408 distinct indices, 83..502, ZERO indices carrying two different names
//! controls: 92 Speed, 96 Booster, 97 MagicGuard, 98 IronWill, 100 PowerGuard   5/5
//! ```
//!
//! The five controls are the bits `research/magic-damage.md` §7.3 had already named by a
//! different route, and all five come back identical. The head of the table is
//! `83 WAT, 84 PAD, 85 MAD, 86 PDD, 87 MDD, 88 ACC, 89 EVA, 90 CRT, 91 CRD, 92 Speed` -
//! which is **the same order, on the same nine bits**, that `FUN_14087c130` reads them in
//! when it builds the attacker totals. Two instruments that share no code meeting on the
//! same nine indices is what promotes 86/87 out of the doubt recorded on
//! [`crate::buff::CTS_WEAPON_DEFENCE`] and gives 88/89 for free.
//!
//! # The one that is a unit trap, and it is not the one that was expected
//!
//! The brief guessed Iron Body would be "almost certainly weapon defence, so bit 86 may
//! already be right". **The bit is right and the unit is not.** Magic Armor's WZ key is
//! `indiePdd` - flat points, *"Weapon Def. +40"*. Iron Body's is **`indiePddR`** - a ratio,
//! *"Weapon Def. +5%"*. Those are two different WZ properties and they are two different
//! entries in the client's own Indie name array (`IndiePDD` at `0x14327ce88`, `IndiePDDR`
//! sixteen bytes-per-entry later at `0x14327cf18`).
//!
//! CTS 86 is a **flat add**: `FUN_14087c130` adds its value to `totals+0x0c`, the field
//! seeded with `floor(STR/4)`. So a level-1 Iron Body sent as `5` on bit 86 is *"+5 Weapon
//! Def."*, not *"+5%"* - and for a low-level character those two numbers are close enough
//! that the screen would not obviously say which one happened. That is exactly the shape
//! `CLAUDE.md`'s *"the unit, not the arithmetic"* records three times.
//!
//! This module therefore **resolves the percentage before it reaches the wire**, and
//! [`buff_level`] takes the character's Weapon Def. as a parameter to make that impossible
//! to skip. See [`iron_body_flat_pdd`].
//!
//! # Nothing here is confirmed on a client
//!
//! Every row is **[L] from `gm-handbook/skills.txt`** cross-checked against the client's own
//! per-level tooltip text, which states the same numbers in words. The bits are [L]/[D] as
//! marked on each constant. **No cast of any of these four has been watched on screen**, so
//! the same status applies as to the Magician pair: read, not measured.

use crate::buff::{BuffDuration, BuffLevel, StatGrant, CTS_SPEED, CTS_WEAPON_DEFENCE};

/// Iron Body, skill **1001000**, max level 20. Warrior. A timed buff granting a
/// **percentage** of Weapon Def. - see the module docs and [`iron_body_flat_pdd`].
pub const IRON_BODY: u32 = 1_001_000;

/// Focus, skill **3001000**, max level 20. Bowman. A timed buff granting **two** flat stats.
pub const FOCUS: u32 = 3_001_000;

/// Disorder, skill **4001000**, max level 20. Thief.
///
/// # This is not a player stat and this module cannot send it
///
/// Its WZ row is `type 1`, `damage 100`, `attackCount 1`, `mobCount 1`, `processtype 108`,
/// and its tooltip reads *"MP -5; **Enemy's** Attack Power -5; Weapon Def. -1 for 10 sec"*.
/// `x` is the enemy's attack-power reduction and `y` is the enemy's weapon-defence
/// reduction. **[L]** The same archive uses the same wording for Threaten (`1201004`),
/// Slow (`2101002`) and Amazon's Judgement (`3100001`), all of which reduce a stat on the
/// *mob*.
///
/// So Disorder is a 100%-damage single-target attack that leaves a **mob** temporary stat
/// behind, and a mob temporary stat needs a mob-directed packet. **This server has never
/// sent one.** Enumerating every `pub const … : u16 = 0x03xx` in `crates/net/src` gives
/// `0x03C6` MobEnterField, `0x03D1` MobLeaveField, `0x03D2` MobChangeController, `0x03D9`
/// MobMove, `0x03E4` MobCtrlAck and `0x03F0` MobHpChange - and nothing that carries a mob
/// stat. **[L]**
///
/// [`buff_level`] returns `None` for this id, deliberately and with a test that says so.
/// The numbers are in [`disorder_debuff`] because they are a reading of the WZ and worth
/// keeping; **there is no packet builder here and one should not be invented.**
pub const DISORDER: u32 = 4_001_000;

/// Dark Sight, skill **4001001**, max level 20. Thief. A **toggle** - see
/// [`BuffDuration::Toggle`] - that sets invisibility and, below master level, a Speed
/// penalty.
pub const DARK_SIGHT: u32 = 4_001_001;

/// The character-temporary-stat bit for **Accuracy**.
///
/// **[L]**, and by two routes that share no code:
///
/// * the name table this module re-derived says `88 -> ACC`, in a run whose five
///   independent controls all reproduce (module docs);
/// * `FUN_14087c130`, the attacker-totals builder, reads CTS 83..91 **in index order** and
///   adds them to the totals in order; bit 88 lands on `totals+0x18`, the field
///   `research/magic-damage.md` §3.1 identifies as *the accuracy line*.
///
/// The value setter was re-disassembled rather than copied: `FUN_140896b30` writes
/// `[rbx + 0x4b8]`, which is the offset `research/magic-damage.md` §7.3's table already
/// gives for bit 88. **[L]**
pub const CTS_ACCURACY: u32 = 88;

/// The character-temporary-stat bit for **Avoidability** (the client calls it `EVA`).
///
/// **[D]**, one notch weaker than [`CTS_ACCURACY`], and the difference is worth stating.
/// The name table says `89 -> EVA` and the totals builder puts bit 89 on `totals+0x1c` -
/// but §7.4's own table leaves that column's *consumption* blank, so unlike `totals+0x18`
/// nothing has been found that reads it back out and does something avoidance-shaped with
/// it. What is established is the name, the position in the in-order 83..91 run, and the
/// setter: `FUN_140897b40` writes `[rbx + 0x4f4]`, matching §7.3. **[L]** for those.
///
/// > **Blind spot, quoted so it can be acted on.** Nothing I found reads `secStat+0x4f4`
/// > and applies it to a dodge roll. The name and the ordering are what put Focus's
/// > `indieEva` here; a reader that consumes it would promote this to [L], and a run in
/// > which Focus moves Accuracy but not Avoidability would say the pair is off by one.
pub const CTS_AVOIDABILITY: u32 = 89;

/// The character-temporary-stat bit for **Weapon Attack** (the client's `PAD`). Rage's
/// `indiePad` lands here.
///
/// **[D]**, the same standing as [`CTS_AVOIDABILITY`] and for the same reasons. The
/// re-derived name table (module docs: 408 names, five controls) says `84 -> PAD`, one
/// before `85 MAD`, and the whole run `83 WAT, 84 PAD, 85 MAD, 86 PDD, 87 MDD, 88 ACC,
/// 89 EVA` is the client's own stat order with the two bits already promoted to [L]
/// sitting where the table puts them.
///
/// The decoder is safe to hand it: in `FUN_140a165f0`'s census
/// (`research/msexe-secondarystat-140a165f0.txt`) bit 84's block is the standard
/// 87-instruction shape, `reads=u32,u16,u32,u32` - byte-identical in form to bit 92's, which
/// a client has run. No extras, so a two-stat Rage packet is the Magic Armor shape.
///
/// > **Blind spot.** No reader has been traced that takes bit 84's `secStat` slot into a
/// > damage roll. `research/magic-damage.md` §7.3 names `totals+0x00` the physical
/// > multiplier under the OLDER 323-name table's "83 = PAD", which this table disagrees
/// > with by one - so that row corroborates nothing either way. One run decides it: Rage's
/// > number appearing on the stat window's attack line says 84; the icon drawing while the
/// > attack line stays put says the pair is off by one, and 83 is the next candidate.
pub const CTS_WEAPON_ATTACK: u32 = 84;

/// The character-temporary-stat bit for **Magic Attack** (`MAD`). Name table `85 -> MAD`,
/// standard block shape, no consumer traced - exactly [`CTS_WEAPON_ATTACK`]'s standing.
/// Nothing this server grants uses it today; it is here so the `indie*` column mapping in
/// `world::session::buff` is complete rather than because a skill needs it yet.
pub const CTS_MAGIC_ATTACK: u32 = 85;

/// The character-temporary-stat bit for **Jump**. Haste's `indieJump` lands here, beside
/// its `indieSpeed` on [`crate::buff::CTS_SPEED`].
///
/// **[D]**: name table `93 -> Jump`, one after `92 Speed` (which is [L] - the client's own
/// refusal string reads it), and the census gives bit 93 the same standard block as 92.
/// A run in which Haste raises the jump height as well as the walk says 93; a walk that
/// speeds up under an icon while the jump does not says this half is wrong.
pub const CTS_JUMP: u32 = 93;

/// The character-temporary-stat bit for **Dark Sight**'s invisibility.
///
/// **[L]**, and it does not rest on the name table alone. `FUN_14276e0e0` is forty-two
/// bytes long and does exactly one thing:
///
/// ```asm
/// 14276e0ea  mov  edx, [rax + 0x664]     ; the checksum of bit 99's value triple
/// 14276e0f0  lea  rcx, [rax + 0x65c]     ; bit 99's value
/// 14276e0f7  call 0x1401ba9d0            ; the de-obfuscating getter
/// 14276e100  setne cl                    ; -> value != 0
/// ```
///
/// That is an `IsDarkSight()` predicate, and `0x65c` is the offset §7.3's table gives for
/// bit 99. The name table independently says `99 -> DarkSight`. **Two routes, one offset.**
///
/// # The value is read as a boolean at the one reader that was disassembled
///
/// Hence [`DARK_SIGHT_ON`] is `1`: the smallest value that satisfies the only consumption
/// anyone has measured. **[D]**
///
/// > **Blind spot.** A whole-image scan for `[reg + 0x65c]` operands returns 56 sites, ten
/// > of which are the `lea rcx, [X + 0x65c]` getter idiom, and **one** of those ten was
/// > disassembled. If one of the other nine multiplies the value by something, `1` is a
/// > value in the wrong unit rather than a flag. The scan is also an exact-displacement
/// > match, so a wider load at a lower displacement that covers `0x65c` would not appear
/// > in it at all.
///
/// # Bit 99 is the only one of these five that makes the decoder read extra bytes
///
/// `research/msexe-secondarystat-140a165f0.txt` censuses the `0x007D` decoder by read
/// shape - 407 standard `u32,u16,u32,u32` blocks plus 67 conditional extras. Bits 86, 87,
/// 88, 89, 92 and 97 appear **once** each. Bit 99 appears **three** times: the standard
/// block, a two-`u32` extras block at `0x140a46032`, and a 19-byte block at `0x140a463f0`
/// that reads nothing. The extras block is gated on the same bit:
///
/// ```asm
/// 140a46025  mov  edx, 0x63              ; 99
/// 140a46032  call 0x1402bf6d0            ; is bit 99 set?
/// 140a46046  READ u32  -> setter 0x140896680  ; writes secStat+0x6a4
/// 140a46062  READ u32  -> setter 0x140896810  ; writes secStat+0x6b0
/// ```
///
/// **[L]** So a Dark Sight `0x007D` consumes **eight more bytes** than a Nimble Feet one.
/// They come out of [`crate::buff::TAIL_LEN`]'s zero padding, which is 64 bytes and every
/// field in it is fixed width - see [`DARK_SIGHT_EXTRA_BYTES`] for the budget.
pub const CTS_DARK_SIGHT: u32 = 99;

/// What to put in bit 99's value slot. See [`CTS_DARK_SIGHT`]: the one measured reader
/// tests it for non-zero.
pub const DARK_SIGHT_ON: i16 = 1;

/// Extra body bytes the decoder consumes when bit 99 is set, on top of the mask and the
/// per-stat entries: two `u32`s. **[L]**, from the gated block at `0x140a46032`.
///
/// The tail is 64 zero bytes and both extras are fixed-width, so they read as `0, 0` and
/// the remaining 56 bytes of slack are untouched. This constant exists so that a future
/// attempt to bisect [`crate::buff::TAIL_LEN`] downwards - which the module docs there
/// invite - knows that a Dark Sight packet needs eight more than the others.
pub const DARK_SIGHT_EXTRA_BYTES: usize = 8;

/// Master level of [`IRON_BODY`], from the table's own length rather than a second literal.
pub const IRON_BODY_MAX_LEVEL: u32 = IRON_BODY_PDD_PERCENT.len() as u32;

/// Master level of [`FOCUS`].
pub const FOCUS_MAX_LEVEL: u32 = FOCUS_ACC.len() as u32;

/// Master level of [`DARK_SIGHT`].
pub const DARK_SIGHT_MAX_LEVEL: u32 = DARK_SIGHT_MP.len() as u32;

/// Master level of [`DISORDER`].
pub const DISORDER_MAX_LEVEL: u32 = DISORDER_ATTACK_DROP.len() as u32;

// ---------------------------------------------------------------------------------------
// Iron Body 1001000
// ---------------------------------------------------------------------------------------

/// `mpCon` for [`IRON_BODY`]: **15 at every level**, so it is a scalar rather than an array.
///
/// This is the WZ's own shape - `mpCon` sits on the skill's `common` node, not on any
/// `level` node - and writing it as a 20-long array of fifteens would invent a per-level
/// structure the data does not have.
const IRON_BODY_MP: u16 = 15;

/// `time` for [`IRON_BODY`] levels 1..=20, in **SECONDS**.
///
/// **Identical to Magic Armor's `time` column, including the break in the last row**:
/// levels 1..=19 are exactly `300 + 15*(lv-1)` and level 20 is **600**, where that line
/// predicts 585. Two skills sharing a column is not a reason to share an array - they are
/// two WZ properties - but it is a reason to keep the same test, and there is one.
const IRON_BODY_SECONDS: [u32; 20] = [
    300, 315, 330, 345, 360, 375, 390, 405, 420, 435, 450, 465, 480, 495, 510, 525, 540, 555,
    570, 600,
];

/// `indiePddR` for [`IRON_BODY`] levels 1..=20: **a PERCENTAGE of Weapon Def.**
///
/// **[L]** twice: `gm-handbook/skills.txt` reads `1001000/level/<lv>/indiePddR`, and the
/// client's own tooltip for each level says *"Weapon Def. +5% for 300 sec"* with the same
/// number and a `%` sign in it.
///
/// The last row breaks its own run the same way Magic Armor's does: 1..=19 are `4 + lv`
/// exactly, which predicts **24** at level 20; the WZ says **25**.
const IRON_BODY_PDD_PERCENT: [i16; 20] = [
    5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 25,
];

// ---------------------------------------------------------------------------------------
// Focus 3001000
// ---------------------------------------------------------------------------------------

/// `mpCon` for [`FOCUS`] levels 1..=20.
const FOCUS_MP: [u16; 20] =
    [8, 8, 8, 8, 8, 10, 10, 10, 10, 10, 13, 13, 13, 13, 13, 16, 16, 16, 16, 16];

/// `time` for [`FOCUS`] levels 1..=20, in **SECONDS**.
///
/// This column is **not** a straight line and does not look like one at a glance: it steps
/// by 10 within each five-level band and by **20** across a band boundary (110 -> 130,
/// 170 -> 195, 235 -> 260). A fitted curve would be wrong at three levels out of twenty
/// rather than one.
const FOCUS_SECONDS: [u32; 20] = [
    70, 80, 90, 100, 110, 130, 140, 150, 160, 170, 195, 205, 215, 225, 235, 260, 270, 280,
    290, 300,
];

/// `indieAcc` for [`FOCUS`] levels 1..=20: **flat Accuracy points**, `"Accuracy +1"`. **[L]**
const FOCUS_ACC: [i16; 20] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
];

/// `indieEva` for [`FOCUS`] levels 1..=20: **flat Avoidability points**, `"Evasion +5"`.
///
/// **The two columns are not equal**, unlike Magic Armor's `indiePdd`/`indieMdd` pair -
/// evasion runs four ahead of accuracy at every level and then breaks by one more at level
/// 20 (23 -> 25 where accuracy goes 19 -> 20). That is the whole reason this skill is a
/// two-array skill rather than one array used twice, and there is a test that would fail if
/// anyone collapsed them.
const FOCUS_EVA: [i16; 20] = [
    5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 25,
];

// ---------------------------------------------------------------------------------------
// Dark Sight 4001001
// ---------------------------------------------------------------------------------------

/// `mpCon` for [`DARK_SIGHT`] levels 1..=20. It goes **down** with level, 50 -> 30.
///
/// Levels 1..=19 are `51 - lv`; level 20 is **30**, where that line predicts 31.
const DARK_SIGHT_MP: [u16; 20] = [
    50, 49, 48, 47, 46, 45, 44, 43, 42, 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 30,
];

/// `speed` for [`DARK_SIGHT`] levels 1..=20 - the size of the **movement-speed penalty**,
/// and `None` at master level because the node is **absent** there.
///
/// # The WZ stores a magnitude; the sign is the skill's, and only the tooltip says which
///
/// Ten skills in this archive carry a `speed` node and it is a positive integer on all of
/// them. What it *means* is not in the data: **[L]**
///
/// ```text
/// 1002    Nimble Feet         speed 10   "speed +10 for 30 sec"          <- own speed, UP
/// 4001001 Dark Sight          speed 20   "Speed -20 while active"        <- own speed, DOWN
/// 1201004 Threaten            speed 35   "Enemy Speed -35"               <- the MOB's
/// 2101002 Slow                speed 30   "Enemy Speed -30"               <- the MOB's
/// ```
///
/// Nimble Feet is the control: the same property name, the same client, and the opposite
/// sign, established by the tooltip and by a confirmed cast. So Dark Sight's `20` reaches
/// [`crate::buff::CTS_SPEED`] as **`-20`**. **[D]** - the magnitude is [L], the negation is
/// read off the tooltip.
///
/// # Level 20 has no `speed` node at all, and that is the data
///
/// `gm-handbook/skills.txt` leaves the cell empty on level 20 only, and the level-20
/// tooltip drops the clause entirely: *"MP -30; Disappear into the shadows; regular
/// movement speed while active"*. **[L]** `CLAUDE.md` records a mob's hit box collapsing
/// because an absent property was sent as `0`, and `0` on the Speed bit would be a granted
/// stat worth nothing rather than an ungranted one - so this is an `Option` and level 20
/// grants **one** stat where every other level grants two.
const DARK_SIGHT_SPEED_PENALTY: [Option<i16>; 20] = [
    Some(20),
    Some(19),
    Some(18),
    Some(17),
    Some(16),
    Some(15),
    Some(14),
    Some(13),
    Some(12),
    Some(11),
    Some(10),
    Some(9),
    Some(8),
    Some(7),
    Some(6),
    Some(5),
    Some(4),
    Some(3),
    Some(2),
    None,
];

// ---------------------------------------------------------------------------------------
// Disorder 4001000  - data only, see DISORDER
// ---------------------------------------------------------------------------------------

/// `mpCon` for [`DISORDER`] levels 1..=20.
const DISORDER_MP: [u16; 20] =
    [5, 5, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10, 10];

/// `time` for [`DISORDER`] levels 1..=20, in **SECONDS**. This is how long the debuff sits
/// on the **mob**, not on the caster.
const DISORDER_SECONDS: [u32; 20] = [
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 30,
];

/// `x` for [`DISORDER`] levels 1..=20: how far the **enemy's** attack power drops.
const DISORDER_ATTACK_DROP: [i16; 20] = [
    5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 25,
];

/// `y` for [`DISORDER`] levels 1..=20: how far the **enemy's** weapon defence drops.
const DISORDER_DEFENCE_DROP: [i16; 20] =
    [1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5];

/// `damage` for [`DISORDER`], every level: **100** percent of a normal attack, one hit, one
/// target. Constant across all 20 levels in the WZ, so it is a scalar here.
pub const DISORDER_DAMAGE_PERCENT: i16 = 100;

// The tables are indexed together, so a length mismatch would silently read a neighbour.
// These refuse to compile instead - the same guard `buff.rs` uses.
const _: () = assert!(IRON_BODY_SECONDS.len() == IRON_BODY_PDD_PERCENT.len());
const _: () = assert!(FOCUS_MP.len() == FOCUS_SECONDS.len());
const _: () = assert!(FOCUS_MP.len() == FOCUS_ACC.len());
const _: () = assert!(FOCUS_MP.len() == FOCUS_EVA.len());
const _: () = assert!(DARK_SIGHT_MP.len() == DARK_SIGHT_SPEED_PENALTY.len());
const _: () = assert!(DISORDER_MP.len() == DISORDER_SECONDS.len());
const _: () = assert!(DISORDER_MP.len() == DISORDER_ATTACK_DROP.len());
const _: () = assert!(DISORDER_MP.len() == DISORDER_DEFENCE_DROP.len());

/// [`IRON_BODY`]'s `indiePddR` at `level`: **a percentage**, not defence points.
///
/// Exposed so that a caller reporting the skill (a tooltip, a `!buff` echo, a test) can say
/// *"+15%"* without going through [`iron_body_flat_pdd`] and un-resolving it.
pub fn iron_body_percent(level: u32) -> Option<i16> {
    let i = usize::try_from(level.checked_sub(1)?).ok()?;
    IRON_BODY_PDD_PERCENT.get(i).copied()
}

/// Turn Iron Body's percentage into the flat Weapon Def. points that CTS 86 actually adds.
///
/// # This is the whole of the unit fix, in one place
///
/// `percent` is `indiePddR`; `weapon_defence` is the character's Weapon Def. **before** the
/// buff - the same total the client is about to add the granted value to, which
/// `research/magic-damage.md` §7.4 identifies as `totals+0x0c`, seeded with `floor(STR/4)`
/// and topped up from equipment.
///
/// **The truncation is [I].** No client code has been decoded that resolves an `indiePddR`,
/// because the evidence says the client never sees the percentage at all - it sees whatever
/// number arrives on bit 86. Integer truncation is chosen because it is what the client's
/// own `/100` idiom does everywhere else it has been read (`research/magic-damage.md` §7.3,
/// `imul` + `sar edx,5`, and §1.5's `stat_div`), not because a tooltip says so.
///
/// A non-positive `weapon_defence` yields `0`, which grants the icon and no points. That is
/// deliberate rather than an error: **an unanswered or refused cast freezes the client's
/// UI**, so the failure mode of a strange input has to be a weak buff, never no reply.
pub fn iron_body_flat_pdd(percent: i16, weapon_defence: i16) -> i16 {
    let wdef = i32::from(weapon_defence).max(0);
    let points = wdef * i32::from(percent) / 100;
    i16::try_from(points).unwrap_or(i16::MAX)
}

/// The stat-granting levels of the three castable skills in this module.
///
/// # Why this signature has a third parameter that two of the three ignore
///
/// [`IRON_BODY`]'s WZ value is a **percentage** and [`crate::buff::CTS_WEAPON_DEFENCE`] is a
/// **flat add** (module docs). The percentage has to be resolved against the character's
/// Weapon Def. before it can go on the wire, and this crate has no access to a character.
///
/// Making `weapon_defence` a required argument rather than an `Option` or a second function
/// is the point: a caller that has to pass a number is a caller that has read why. The one
/// bug this module can produce silently is Iron Body's `5` shipped as five defence points
/// instead of five percent, and on a low-level Warrior those two are close enough that the
/// stat window would not obviously say which happened.
///
/// [`FOCUS`] and [`DARK_SIGHT`] ignore the parameter entirely, and there is a test that
/// pins that - so a caller that has only a placeholder to pass can still cast those two
/// correctly.
///
/// [`DISORDER`] returns `None`. It is a mob debuff and this server has no packet for it;
/// see the constant's own documentation.
pub fn buff_level(skill_id: u32, level: u32, weapon_defence: i16) -> Option<BuffLevel> {
    match skill_id {
        IRON_BODY => iron_body_level(level, weapon_defence),
        FOCUS => focus_level(level),
        DARK_SIGHT => dark_sight_level(level),
        // Not an oversight; see `DISORDER`. A mob temporary stat is not a `0x007D`.
        DISORDER => None,
        _ => None,
    }
}

/// Levels 1..=20 of [`IRON_BODY`]. One stat, timed, and the value is **resolved** here.
fn iron_body_level(level: u32, weapon_defence: i16) -> Option<BuffLevel> {
    let i = usize::try_from(level.checked_sub(1)?).ok()?;
    let seconds = *IRON_BODY_SECONDS.get(i)?;
    let percent = IRON_BODY_PDD_PERCENT[i];
    let duration = BuffDuration::Seconds(seconds);
    Some(BuffLevel {
        mp_cost: IRON_BODY_MP,
        seconds: duration.seconds(),
        // No `cooltime` node at any level. Not a placeholder.
        cooldown_seconds: 0,
        bit: CTS_WEAPON_DEFENCE,
        value: iron_body_flat_pdd(percent, weapon_defence),
        second: None,
        duration,
    })
}

/// Levels 1..=20 of [`FOCUS`]. **Two** stats, both flat, timed.
fn focus_level(level: u32) -> Option<BuffLevel> {
    let i = usize::try_from(level.checked_sub(1)?).ok()?;
    let seconds = *FOCUS_SECONDS.get(i)?;
    let duration = BuffDuration::Seconds(seconds);
    Some(BuffLevel {
        mp_cost: FOCUS_MP[i],
        seconds: duration.seconds(),
        cooldown_seconds: 0,
        bit: CTS_ACCURACY,
        value: FOCUS_ACC[i],
        second: Some(StatGrant { bit: CTS_AVOIDABILITY, value: FOCUS_EVA[i] }),
        duration,
    })
}

/// Levels 1..=20 of [`DARK_SIGHT`]. A **toggle**; two stats below master level, one at it.
///
/// The first stat is the invisibility flag - see [`CTS_DARK_SIGHT`] and [`DARK_SIGHT_ON`].
/// The second is the Speed penalty, and it is `None` at level 20 because the WZ node is
/// absent there rather than zero.
///
/// Putting the flag on bit 99 and the penalty on bit 92 keeps each field meaning exactly
/// one thing. The alternative - sending the `speed` magnitude on bit 99 and letting it
/// double as "non-zero, therefore on" - would work at 19 levels and then need an invented
/// number at level 20, which is precisely how an absent property turns into a fake value.
fn dark_sight_level(level: u32) -> Option<BuffLevel> {
    let i = usize::try_from(level.checked_sub(1)?).ok()?;
    let penalty = *DARK_SIGHT_SPEED_PENALTY.get(i)?;
    let duration = BuffDuration::Toggle;
    Some(BuffLevel {
        mp_cost: DARK_SIGHT_MP[i],
        seconds: duration.seconds(),
        cooldown_seconds: 0,
        bit: CTS_DARK_SIGHT,
        value: DARK_SIGHT_ON,
        second: penalty.map(|p| StatGrant { bit: CTS_SPEED, value: -p }),
        duration,
    })
}

/// What one level of [`DISORDER`] does **to a mob**. Data only - there is no wire format
/// here and there should not be one until a mob-stat packet is decoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisorderDebuff {
    pub mp_cost: u16,
    /// `Skill.wz`'s `time`, in **SECONDS**. Nothing in this module converts it, because
    /// `BuffLevel::granted_by` is the one place that multiplies by 1000 and this value
    /// never goes near it.
    pub seconds: u32,
    /// `x` - how far the enemy's attack power drops.
    pub attack_drop: i16,
    /// `y` - how far the enemy's weapon defence drops.
    pub defence_drop: i16,
    /// `damage` - percent of a normal attack the cast also deals. Always
    /// [`DISORDER_DAMAGE_PERCENT`].
    pub damage_percent: i16,
}

/// One level of [`DISORDER`], 1..=20.
pub fn disorder_debuff(level: u32) -> Option<DisorderDebuff> {
    let i = usize::try_from(level.checked_sub(1)?).ok()?;
    let seconds = *DISORDER_SECONDS.get(i)?;
    Some(DisorderDebuff {
        mp_cost: DISORDER_MP[i],
        seconds,
        attack_drop: DISORDER_ATTACK_DROP[i],
        defence_drop: DISORDER_DEFENCE_DROP[i],
        damage_percent: DISORDER_DAMAGE_PERCENT,
    })
}

/// The nine `type 50` first-job passives, and whether this server has to do anything.
///
/// # The client applies passives itself, and that is measured rather than assumed
///
/// `research/first-job-buffs.md` §5. Three separate client functions resolve a passive from
/// the client's **own** skill record and its **own** `Skill.wz`, with no packet in the
/// path. The chain is always the same three calls -
/// `FUN_1407b3df0(skillId) -> level`, `FUN_14079fe90(record, level) -> level node`, then the
/// de-obfuscating getter on a per-level field:
///
/// | client function | reads | the WZ column it lands on |
/// |---|---|---|
/// | `FUN_1407b4c10` | `3000001` The Eye of Amazon, `4000001` Keen Eyes | **`range`** - added to a per-weapon base of 200/300/380/400/450 px |
/// | `FUN_1407e49b0` | `3000000` Critical Shot | **`crtX` and `crdX`** - two fields read, two written to the caller's two out-pointers |
/// | `FUN_1407e4250` | the job's Mastery skill | switched on by id, and **it is called from `FUN_14087c130`, the attacker-totals builder itself** |
///
/// The column names are the corroboration rather than decoration: Eye of Amazon's only
/// per-level column **is** `range` and the client adds the value to a range; Critical Shot's
/// only per-level columns **are** `crtX` and `crdX`, and the client reads exactly two
/// fields. **[L]**
///
/// So for the seven passives that change a client-computed number, **the server does
/// nothing but send the skill level it already sends**. That is a result, not a gap.
///
/// # The two that are not client-computed, and why they are different
///
/// `1000001` Max HP Increase (`mhpR`) and `2000001` Max MP Increase (`mmpR`) change numbers
/// the **server** owns: `research/user-hit.md` establishes that the client computes damage
/// and never writes HP, and the max values arrive in this server's own stat packet. A
/// percentage the client folds in locally on top of a max the server already sent would be
/// visible as a mismatch, and one the server folds in would not - so the two readings are
/// distinguishable and neither is safe to assume.
/// `research/magician-first-job.md` §8 experiment A is that measurement and it is still
/// unrun. **[D]** that these two are the exception; **not established** which way they go.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassiveOwner {
    /// The client resolves it from the skill level the server already sent. Send nothing.
    Client,
    /// It changes a number this server owns, and which side applies the percentage has not
    /// been measured. See the enum's docs.
    Unmeasured,
}

/// Every `type 50` first-job passive, with who applies it.
///
/// A function rather than a table so that an unlisted id returns `None` instead of matching
/// a neighbour, which is the same reason `buff.rs` gives one table function per skill.
pub fn passive_owner(skill_id: u32) -> Option<PassiveOwner> {
    match skill_id {
        1_000_000 // Improved HP Recovery - `y`, a percent of HP restored by items
        | 1_000_002 // Precise Strikes - `accX`, `crtX`
        | 2_000_000 // Improved MP Recovery - `x` percent of max MP per tick, `y`
        | 3_000_000 // Critical Shot - `crtX`, `crdX`; FUN_1407e49b0 reads both
        | 3_000_001 // The Eye of Amazon - `range`; FUN_1407b4c10 adds it
        | 4_000_000 // Nimble Body - `accX`, `evaX`
        | 4_000_001 // Keen Eyes - `range`; FUN_1407b4c10 adds it
        => Some(PassiveOwner::Client),
        1_000_001 | 2_000_001 => Some(PassiveOwner::Unmeasured),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buff::{
        bits_in_mask, stat_mask, temporary_stat_set, temporary_stat_set_len, CTS_MAGIC_GUARD,
        MASK_LEN, MAX_CTS_BIT, TAIL_LEN,
    };

    /// A Weapon Def. big enough that Iron Body's percentages come out as whole numbers, so
    /// a test that is checking the *table* is not also checking the rounding.
    const WDEF_200: i16 = 200;

    /// **The bit numbers, with their confidence, and where each lands in the 124-byte mask.**
    ///
    /// Asserted as numbers *and* as mask bytes because the two can disagree: the arithmetic
    /// is big-endian inside a little-endian word, and the "obvious" `1 << (i & 31)` sets a
    /// different stat that the client would grant without complaint.
    #[test]
    fn the_new_bits_land_where_the_client_reads_them() {
        assert_eq!(CTS_ACCURACY, 88, "[L] - name table + totals+0x18, setter [rbx+0x4b8]");
        assert_eq!(CTS_AVOIDABILITY, 89, "[D] - name table + totals+0x1c, setter [rbx+0x4f4]");
        assert_eq!(CTS_DARK_SIGHT, 99, "[L] - FUN_14276e0e0 reads secStat+0x65c as a bool");

        // 88 and 89 share word 2, bytes 8..11: 1 << (31-24) | 1 << (31-25) = 0x80 | 0x40.
        let focus = stat_mask(&[CTS_ACCURACY, CTS_AVOIDABILITY]);
        assert_eq!(&focus[8..12], &[0xc0, 0x00, 0x00, 0x00], "bytes 8..11 read c0 00 00 00");
        assert_eq!(focus.iter().filter(|b| **b != 0).count(), 1, "both bits, one byte");
        assert_eq!(bits_in_mask(&focus), vec![CTS_ACCURACY, CTS_AVOIDABILITY]);

        // Bit 99: word 99>>5 = 3 is bytes 12..15, and 1 << (31 - (99 & 31)) = 1 << 28.
        let dark = stat_mask(&[CTS_DARK_SIGHT]);
        assert_eq!(&dark[12..16], &[0x00, 0x00, 0x00, 0x10], "bytes 12..15 read 00 00 00 10");
        assert_eq!(dark.iter().filter(|b| **b != 0).count(), 1, "exactly one byte is set");
        assert_ne!(
            (1u32 << (CTS_DARK_SIGHT & 31)).to_le_bytes(),
            [0x00, 0x00, 0x00, 0x10],
            "1 << (i & 31) is a DIFFERENT stat"
        );

        // Bit 99 is not bit 97 - they share a word and differ by two shifts, which is
        // exactly the kind of neighbour a wrong endianness would land on.
        assert_ne!(CTS_DARK_SIGHT, CTS_MAGIC_GUARD);
        assert_ne!(dark, stat_mask(&[CTS_MAGIC_GUARD]));

        for bit in [CTS_ACCURACY, CTS_AVOIDABILITY, CTS_DARK_SIGHT] {
            assert!(bit < MAX_CTS_BIT, "bit {bit} is inside the client's own limit");
            assert_eq!(bits_in_mask(&stat_mask(&[bit])), vec![bit]);
        }
    }

    /// **Every level of Iron Body, against `gm-handbook/skills.txt`.**
    ///
    /// The expected rows are transcribed from that file, **not** read back out of the
    /// constants. A test that loops over the array it is checking agrees with the code by
    /// construction and can never disagree with the WZ.
    #[test]
    fn iron_body_matches_the_wz_at_every_one_of_its_twenty_levels() {
        // level, mpCon, time (SECONDS), indiePddR (PERCENT)
        let wz: [(u32, u16, u32, i16); 20] = [
            (1, 15, 300, 5),
            (2, 15, 315, 6),
            (3, 15, 330, 7),
            (4, 15, 345, 8),
            (5, 15, 360, 9),
            (6, 15, 375, 10),
            (7, 15, 390, 11),
            (8, 15, 405, 12),
            (9, 15, 420, 13),
            (10, 15, 435, 14),
            (11, 15, 450, 15),
            (12, 15, 465, 16),
            (13, 15, 480, 17),
            (14, 15, 495, 18),
            (15, 15, 510, 19),
            (16, 15, 525, 20),
            (17, 15, 540, 21),
            (18, 15, 555, 22),
            (19, 15, 570, 23),
            (20, 15, 600, 25),
        ];
        assert_eq!(wz.len() as u32, IRON_BODY_MAX_LEVEL, "master level is 20");

        for (level, mp_cost, seconds, percent) in wz {
            assert_eq!(iron_body_percent(level), Some(percent), "level {level} indiePddR");

            let l = buff_level(IRON_BODY, level, WDEF_200)
                .unwrap_or_else(|| panic!("level {level}"));
            assert_eq!(l.mp_cost, mp_cost, "level {level} mpCon");
            assert_eq!(l.seconds, seconds, "level {level} time, in SECONDS");
            assert_eq!(l.duration, BuffDuration::Seconds(seconds));
            assert!(!l.duration.is_toggle(), "Iron Body is timed");
            assert_eq!(l.cooldown_seconds, 0, "no `cooltime` node at any level");
            assert_eq!(l.bit, CTS_WEAPON_DEFENCE, "level {level} lands on PDD");
            assert_eq!(l.second, None, "Iron Body grants one stat");
            assert_eq!(l.stat_count(), 1);

            // and the value on the wire is RESOLVED, not the percentage
            assert_eq!(l.value, 2 * percent, "{percent}% of 200 is {} points", 2 * percent);
        }

        assert!(buff_level(IRON_BODY, 0, WDEF_200).is_none(), "levels are 1-based");
        assert!(buff_level(IRON_BODY, 21, WDEF_200).is_none(), "master level is 20");
        assert!(buff_level(IRON_BODY, u32::MAX, WDEF_200).is_none());
        assert_eq!(iron_body_percent(0), None);
        assert_eq!(iron_body_percent(21), None);
    }

    /// **The value Iron Body puts on the wire is not the number in the WZ**, and this is the
    /// test that says so out loud.
    ///
    /// `CLAUDE.md`: *"When a number reaches the client and draws wrongly, suspect the unit
    /// before the arithmetic."* Sending `5` for a level-1 Iron Body would be *"+5 Weapon
    /// Def."* where the tooltip promises *"+5%"* - and on a Warrior with 100 W.Def those two
    /// are the same number, which is why this needs a test rather than a comment.
    #[test]
    fn iron_bodys_percentage_is_resolved_and_the_raw_percent_never_reaches_the_wire() {
        // 100 W.Def: the coincidence. 5% of 100 IS 5, so this level cannot discriminate.
        assert_eq!(buff_level(IRON_BODY, 1, 100).unwrap().value, 5);
        assert_eq!(iron_body_percent(1), Some(5));

        // 400 W.Def: now they part company, and by a factor of four.
        let l = buff_level(IRON_BODY, 1, 400).unwrap();
        assert_eq!(l.value, 20, "5% of 400");
        assert_ne!(l.value, iron_body_percent(1).unwrap(), "NOT the raw percent");

        // The resolution truncates, and it truncates the way the client's own /100 does.
        assert_eq!(iron_body_flat_pdd(5, 39), 1, "195/100 -> 1, not 2");
        assert_eq!(iron_body_flat_pdd(25, 3), 0, "75/100 -> 0");
        assert_eq!(iron_body_flat_pdd(25, 4), 1);

        // Degenerate inputs give a weak buff, never a refusal - an unanswered cast freezes
        // the client's whole UI.
        assert_eq!(iron_body_flat_pdd(5, 0), 0);
        assert_eq!(iron_body_flat_pdd(5, -30), 0, "negative W.Def clamps to zero");
        assert_eq!(iron_body_flat_pdd(25, i16::MAX), (i32::from(i16::MAX) * 25 / 100) as i16);
        assert!(buff_level(IRON_BODY, 20, 0).is_some(), "still answers with no W.Def");
    }

    /// **Every level of Focus, against `gm-handbook/skills.txt`**, all four columns.
    #[test]
    fn focus_matches_the_wz_at_every_one_of_its_twenty_levels() {
        // level, mpCon, time (SECONDS), indieAcc, indieEva
        let wz: [(u32, u16, u32, i16, i16); 20] = [
            (1, 8, 70, 1, 5),
            (2, 8, 80, 2, 6),
            (3, 8, 90, 3, 7),
            (4, 8, 100, 4, 8),
            (5, 8, 110, 5, 9),
            (6, 10, 130, 6, 10),
            (7, 10, 140, 7, 11),
            (8, 10, 150, 8, 12),
            (9, 10, 160, 9, 13),
            (10, 10, 170, 10, 14),
            (11, 13, 195, 11, 15),
            (12, 13, 205, 12, 16),
            (13, 13, 215, 13, 17),
            (14, 13, 225, 14, 18),
            (15, 13, 235, 15, 19),
            (16, 16, 260, 16, 20),
            (17, 16, 270, 17, 21),
            (18, 16, 280, 18, 22),
            (19, 16, 290, 19, 23),
            (20, 16, 300, 20, 25),
        ];
        assert_eq!(wz.len() as u32, FOCUS_MAX_LEVEL, "master level is 20");

        for (level, mp_cost, seconds, acc, eva) in wz {
            let l = buff_level(FOCUS, level, 0).unwrap_or_else(|| panic!("level {level}"));
            assert_eq!(l.mp_cost, mp_cost, "level {level} mpCon");
            assert_eq!(l.seconds, seconds, "level {level} time, in SECONDS");
            assert_eq!(l.duration, BuffDuration::Seconds(seconds));
            assert_eq!(l.cooldown_seconds, 0, "no `cooltime` node at any level");

            assert_eq!(l.bit, CTS_ACCURACY, "level {level} indieAcc bit");
            assert_eq!(l.value, acc, "level {level} indieAcc, FLAT accuracy points");
            assert_eq!(
                l.second,
                Some(StatGrant { bit: CTS_AVOIDABILITY, value: eva }),
                "level {level} indieEva"
            );
            assert_eq!(l.stat_count(), 2);
        }

        assert!(buff_level(FOCUS, 0, 0).is_none(), "levels are 1-based");
        assert!(buff_level(FOCUS, 21, 0).is_none(), "master level is 20");
    }

    /// **Focus's two columns are not the same column**, unlike Magic Armor's pair.
    ///
    /// `buff.rs` keeps `indiePdd` and `indieMdd` as two arrays that happen to agree, with a
    /// test that would notice a future divergence. Focus is the case that already diverges,
    /// so the assertion runs the other way: if anyone "simplifies" this to one array, every
    /// level is wrong by four and nothing on screen says which stat is short.
    #[test]
    fn focuss_accuracy_and_evasion_columns_differ_at_every_level() {
        for level in 1..=FOCUS_MAX_LEVEL {
            let l = buff_level(FOCUS, level, 0).unwrap();
            let second = l.second.expect("Focus always grants two");
            assert_ne!(l.value, second.value, "level {level}: acc and eva are not equal");
            assert_eq!(second.value - l.value, if level == 20 { 5 } else { 4 });
        }
        // ...and the gap itself changes in the last row, so even "eva = acc + 4" is wrong.
        assert_eq!(buff_level(FOCUS, 19, 0).unwrap().second.unwrap().value, 23);
        assert_eq!(buff_level(FOCUS, 20, 0).unwrap().second.unwrap().value, 25, "not 24");
    }

    /// **Every level of Dark Sight**, including the master level that grants one stat.
    #[test]
    fn dark_sight_matches_the_wz_at_every_one_of_its_twenty_levels() {
        // level, mpCon, speed penalty magnitude (None = the WZ node is ABSENT)
        let wz: [(u32, u16, Option<i16>); 20] = [
            (1, 50, Some(20)),
            (2, 49, Some(19)),
            (3, 48, Some(18)),
            (4, 47, Some(17)),
            (5, 46, Some(16)),
            (6, 45, Some(15)),
            (7, 44, Some(14)),
            (8, 43, Some(13)),
            (9, 42, Some(12)),
            (10, 41, Some(11)),
            (11, 40, Some(10)),
            (12, 39, Some(9)),
            (13, 38, Some(8)),
            (14, 37, Some(7)),
            (15, 36, Some(6)),
            (16, 35, Some(5)),
            (17, 34, Some(4)),
            (18, 33, Some(3)),
            (19, 32, Some(2)),
            (20, 30, None),
        ];
        assert_eq!(wz.len() as u32, DARK_SIGHT_MAX_LEVEL, "master level is 20");

        for (level, mp_cost, penalty) in wz {
            let l = buff_level(DARK_SIGHT, level, 0).unwrap_or_else(|| panic!("{level}"));
            assert_eq!(l.mp_cost, mp_cost, "level {level} mpCon");

            // A toggle, stated twice: the enum says so and `seconds` is derived from it.
            assert_eq!(l.duration, BuffDuration::Toggle, "processtype 113, no `time` node");
            assert!(l.duration.is_toggle());
            assert_eq!(l.seconds, 0, "0 here means ABSENT, not zero seconds");
            assert_eq!(l.cooldown_seconds, 0);

            assert_eq!(l.bit, CTS_DARK_SIGHT, "level {level} invisibility bit");
            assert_eq!(l.value, DARK_SIGHT_ON, "a flag, not a magnitude");
            assert_ne!(l.value, 0, "zero would read as `not in dark sight`");

            match penalty {
                Some(p) => {
                    assert_eq!(
                        l.second,
                        Some(StatGrant { bit: CTS_SPEED, value: -p }),
                        "level {level}: the WZ stores {p} and the tooltip says -{p}"
                    );
                    assert_eq!(l.stat_count(), 2);
                }
                None => {
                    assert_eq!(l.second, None, "level 20 has NO `speed` node");
                    assert_eq!(l.stat_count(), 1);
                }
            }
        }

        assert!(buff_level(DARK_SIGHT, 0, 0).is_none(), "levels are 1-based");
        assert!(buff_level(DARK_SIGHT, 21, 0).is_none(), "master level is 20");
    }

    /// **The Speed penalty is negative, and Nimble Feet is the control that says so.**
    ///
    /// Both skills carry the same WZ property, `speed`, as a positive magnitude. Nimble
    /// Feet's tooltip says *"+10"* and its cast is confirmed on a client; Dark Sight's says
    /// *"-20"*. So the sign is not in the data and the two must land on the same bit with
    /// opposite signs, which is what this pins.
    #[test]
    fn dark_sight_slows_where_nimble_feet_hastens_and_both_use_bit_92() {
        let nimble = crate::buff::buff_level(crate::buff::NIMBLE_FEET, 3).unwrap();
        assert_eq!(nimble.bit, CTS_SPEED);
        assert!(nimble.value > 0, "Nimble Feet is +10, the confirmed control");

        let dark = buff_level(DARK_SIGHT, 1, 0).unwrap();
        let speed = dark.second.expect("levels 1..=19 carry the penalty");
        assert_eq!(speed.bit, CTS_SPEED, "the same bit the control uses");
        assert!(speed.value < 0, "and the opposite sign");
        assert_eq!(speed.value, -20);

        // The penalty shrinks as the skill improves, and vanishes entirely at master level.
        let mut previous = i16::MIN;
        for level in 1..=19u32 {
            let v = buff_level(DARK_SIGHT, level, 0).unwrap().second.unwrap().value;
            assert!(v > previous, "level {level}: the penalty gets smaller, not larger");
            previous = v;
        }
        assert_eq!(buff_level(DARK_SIGHT, 20, 0).unwrap().second, None);
    }

    /// **Iron Body level 20 on the wire, byte for byte.** One stat, 600 000 ms.
    #[test]
    fn iron_body_at_master_level_is_the_documented_body() {
        let level = buff_level(IRON_BODY, 20, WDEF_200).unwrap();
        let body = temporary_stat_set(&level.all_granted_by(IRON_BODY));

        assert_eq!(body.len(), 198, "124 mask + 10 stat + 64 tail");
        assert_eq!(body.len(), temporary_stat_set_len(level.stat_count()));
        assert_eq!(&body[0..8], &[0u8; 8], "the mask is zero before the defence word");
        assert_eq!(&body[8..12], &[0x00, 0x02, 0x00, 0x00], "bit 86 alone");
        assert_eq!(&body[12..124], &[0u8; 112], "and zero after it");
        assert_eq!(
            &body[124..134],
            &[0x32, 0x00, 0x28, 0x46, 0x0f, 0x00, 0xc0, 0x27, 0x09, 0x00],
            "50 points (25% of 200), reason 1001000, duration 600000 MILLISECONDS"
        );
        assert_eq!(u32::from_le_bytes([body[130], body[131], body[132], body[133]]), 600_000);
        assert_eq!(&body[134..], &[0u8; TAIL_LEN], "the tail is all zero");
    }

    /// **Focus level 1 on the wire, byte for byte.** Two stats, ascending, 70 000 ms.
    #[test]
    fn focus_at_level_one_is_the_documented_body() {
        let level = buff_level(FOCUS, 1, 0).unwrap();
        let body = temporary_stat_set(&level.all_granted_by(FOCUS));

        assert_eq!(body.len(), 208, "124 mask + 2 x 10 stat + 64 tail");
        assert_eq!(&body[8..12], &[0xc0, 0x00, 0x00, 0x00], "bits 88 and 89");
        assert_eq!(body.iter().take(MASK_LEN).filter(|b| **b != 0).count(), 1);

        // Bit 88 first, then 89 - the order the client walks the mask in.
        assert_eq!(
            &body[124..134],
            &[0x01, 0x00, 0xa8, 0xca, 0x2d, 0x00, 0x70, 0x11, 0x01, 0x00],
            "accuracy 1, reason 3001000, duration 70000 MILLISECONDS"
        );
        assert_eq!(
            &body[134..144],
            &[0x05, 0x00, 0xa8, 0xca, 0x2d, 0x00, 0x70, 0x11, 0x01, 0x00],
            "avoidability 5, same reason, same duration"
        );
        assert_eq!(&body[144..], &[0u8; TAIL_LEN], "the tail is all zero");
    }

    /// **Dark Sight level 1 on the wire, byte for byte** - and the negative Speed value.
    ///
    /// The speed entry is `0xffec` as an `i16`, which is `-20`. A build that silently sent
    /// `+20` would make the player *faster* in Dark Sight, and the icon would look right.
    #[test]
    fn dark_sight_at_level_one_is_the_documented_body() {
        let level = buff_level(DARK_SIGHT, 1, 0).unwrap();
        let body = temporary_stat_set(&level.all_granted_by(DARK_SIGHT));

        assert_eq!(body.len(), 208, "124 mask + 2 x 10 stat + 64 tail");
        assert_eq!(&body[8..12], &[0x08, 0x00, 0x00, 0x00], "bit 92, word 2");
        assert_eq!(&body[12..16], &[0x00, 0x00, 0x00, 0x10], "bit 99, word 3");
        assert_eq!(bits_in_mask(&body[..MASK_LEN]), vec![CTS_SPEED, CTS_DARK_SIGHT]);

        // 92 before 99, whatever order `all_granted_by` produced them in.
        assert_eq!(
            &body[124..134],
            &[0xec, 0xff, 0xe9, 0x0c, 0x3d, 0x00, 0x00, 0x00, 0x00, 0x00],
            "speed -20, reason 4001001, duration 0 - a toggle has no time node"
        );
        assert_eq!(i16::from_le_bytes([body[124], body[125]]), -20, "NEGATIVE");
        assert_eq!(
            &body[134..144],
            &[0x01, 0x00, 0xe9, 0x0c, 0x3d, 0x00, 0x00, 0x00, 0x00, 0x00],
            "dark sight on, same reason, same zero duration"
        );
        assert_eq!(&body[144..], &[0u8; TAIL_LEN], "the tail is all zero");

        // The decoder reads eight more bytes for bit 99, and the tail covers them.
        assert!(
            body.len() >= MASK_LEN + 2 * 10 + DARK_SIGHT_EXTRA_BYTES,
            "the extras at 0x140a46032 must land inside the body"
        );
        const { assert!(TAIL_LEN > DARK_SIGHT_EXTRA_BYTES) };
    }

    /// **Master level on the wire is one stat, not two with a zero.**
    ///
    /// `CLAUDE.md` records a mob's hit box collapsing because an absent property was sent as
    /// `0`. Here the absent property is Dark Sight's `speed` at level 20, and the two
    /// encodings are ten bytes apart on the wire.
    #[test]
    fn dark_sight_at_master_level_omits_the_speed_stat_entirely() {
        let level = buff_level(DARK_SIGHT, 20, 0).unwrap();
        let body = temporary_stat_set(&level.all_granted_by(DARK_SIGHT));

        assert_eq!(body.len(), 198, "one stat, not two");
        assert_eq!(bits_in_mask(&body[..MASK_LEN]), vec![CTS_DARK_SIGHT], "no bit 92");
        assert_eq!(&body[124..134], &[0x01, 0x00, 0xe9, 0x0c, 0x3d, 0x00, 0, 0, 0, 0]);

        // The nineteenth level is the contrast, and it is a different length.
        let nineteen = buff_level(DARK_SIGHT, 19, 0).unwrap();
        assert_eq!(temporary_stat_set(&nineteen.all_granted_by(DARK_SIGHT)).len(), 208);
    }

    /// **Seconds reach the wire as milliseconds exactly once - not zero times, not twice.**
    ///
    /// The same assertion `buff.rs` makes about its own skills, repeated here because this
    /// module adds two more timed skills and `granted_by` is documented as the only place
    /// that multiplies. Both wrong answers are asserted against by name.
    #[test]
    fn the_seconds_to_milliseconds_conversion_happens_exactly_once() {
        for (skill, max) in [(IRON_BODY, IRON_BODY_MAX_LEVEL), (FOCUS, FOCUS_MAX_LEVEL)] {
            for level in 1..=max {
                let l = buff_level(skill, level, WDEF_200).unwrap();
                let stats = l.all_granted_by(skill);
                assert!(l.seconds > 0, "{skill} level {level} is a timed buff");

                for s in &stats {
                    assert_eq!(s.duration_ms, l.seconds * 1000, "{skill} level {level}");
                    assert_ne!(s.duration_ms, l.seconds, "not converted at all");
                    assert_ne!(s.duration_ms, l.seconds * 1_000_000, "converted twice");
                    assert_eq!(s.duration_ms % 1000, 0, "a whole number of seconds");
                    assert_eq!(s.reason, skill);
                }
                assert!(stats.iter().all(|s| s.duration_ms == stats[0].duration_ms));
                assert_eq!(stats.len(), l.stat_count());
            }
        }

        // The toggle converts nothing, because there is nothing to convert - and its
        // *second* stat inherits that zero rather than acquiring a duration of its own.
        for level in 1..=DARK_SIGHT_MAX_LEVEL {
            let l = buff_level(DARK_SIGHT, level, 0).unwrap();
            for s in l.all_granted_by(DARK_SIGHT) {
                assert_eq!(s.duration_ms, 0, "level {level}: no `time` node");
                assert_eq!(s.reason, DARK_SIGHT);
            }
        }

        // And no value is ever touched by the conversion.
        assert_eq!(buff_level(FOCUS, 1, 0).unwrap().value, 1, "not 1000");
        assert_eq!(buff_level(IRON_BODY, 1, 100).unwrap().value, 5, "not 5000");
    }

    /// **`granted_by` alone drops the second stat, on all three skills that have one.**
    ///
    /// `buff.rs` documents that hazard for Magic Armor and pins it with a test. This module
    /// adds two more two-stat casts, and for Dark Sight the dropped half is the Speed
    /// penalty - which on screen is a Dark Sight that makes you no slower, i.e. a working
    /// skill with a missing drawback, the least likely thing anyone reports as a bug.
    #[test]
    fn granted_by_alone_drops_the_second_stat_and_the_length_is_the_tell() {
        for (skill, level, second_bit) in [
            (FOCUS, 1u32, CTS_AVOIDABILITY),
            (DARK_SIGHT, 1, CTS_SPEED),
        ] {
            let l = buff_level(skill, level, 0).unwrap();
            let truncated = temporary_stat_set(&[l.granted_by(skill)]);
            let whole = temporary_stat_set(&l.all_granted_by(skill));

            assert_eq!(truncated.len(), 198, "{skill}: one stat");
            assert_eq!(whole.len(), 208, "{skill}: two");
            assert!(
                !bits_in_mask(&truncated[..MASK_LEN]).contains(&second_bit),
                "{skill}: `granted_by` loses bit {second_bit}"
            );
            assert!(bits_in_mask(&whole[..MASK_LEN]).contains(&second_bit));
        }

        // For the one-stat cases the two calls are identical, so a caller that uses
        // `all_granted_by` everywhere is never wrong.
        for (skill, level) in [(IRON_BODY, 1u32), (DARK_SIGHT, 20)] {
            let l = buff_level(skill, level, WDEF_200).unwrap();
            assert_eq!(l.all_granted_by(skill), vec![l.granted_by(skill)], "{skill}");
        }
    }

    /// **Disorder is data, not a packet**, and `buff_level` says so by returning `None`.
    #[test]
    fn disorder_is_a_mob_debuff_and_this_module_will_not_send_it() {
        for level in [0u32, 1, 10, 20, 21, u32::MAX] {
            assert!(
                buff_level(DISORDER, level, 200).is_none(),
                "Disorder level {level} is not a character temporary stat"
            );
        }

        // level, mpCon, time (SECONDS), x (enemy PAD drop), y (enemy PDD drop)
        let wz: [(u32, u16, u32, i16, i16); 6] = [
            (1, 5, 10, 5, 1),
            (5, 5, 14, 9, 2),
            (10, 7, 19, 14, 3),
            (15, 9, 24, 19, 4),
            (19, 10, 28, 23, 4),
            (20, 10, 30, 25, 5),
        ];
        for (level, mp, seconds, pad, pdd) in wz {
            let d = disorder_debuff(level).unwrap_or_else(|| panic!("level {level}"));
            assert_eq!(d.mp_cost, mp, "level {level} mpCon");
            assert_eq!(d.seconds, seconds, "level {level} time, in SECONDS");
            assert_eq!(d.attack_drop, pad, "level {level} x");
            assert_eq!(d.defence_drop, pdd, "level {level} y");
            assert_eq!(d.damage_percent, 100, "damage is 100 at every level");
        }
        assert_eq!(DISORDER_MAX_LEVEL, 20);
        assert!(disorder_debuff(0).is_none(), "levels are 1-based");
        assert!(disorder_debuff(21).is_none());
    }

    /// **An unmodelled skill returns `None` without disturbing the three that are modelled**,
    /// including the ids most likely to be typed by mistake.
    #[test]
    fn one_skill_returning_none_does_not_stop_the_others() {
        for (id, why) in [
            (1_000_000u32, "Improved HP Recovery is a passive"),
            (1_000_001, "Max HP Increase is a passive; mhpR is a PERCENT"),
            (1_000_002, "Precise Strikes is a passive"),
            (1_001_001, "Power Strike is an ATTACK"),
            (1_001_002, "Slash Blast is an ATTACK"),
            (3_000_000, "Critical Shot is a passive"),
            (3_000_001, "The Eye of Amazon is a passive"),
            (3_001_001, "Arrow Blow is an ATTACK"),
            (4_000_000, "Nimble Body is a passive"),
            (4_000_001, "Keen Eyes is a passive"),
            (4_001_000, "Disorder debuffs a MOB"),
            (4_001_002, "Double Stab is an ATTACK"),
            (2_001_000, "Magic Guard belongs to net::buff, not here"),
            (2_001_001, "Magic Armor belongs to net::buff, not here"),
            (1002, "Nimble Feet belongs to net::buff, not here"),
            (0, "not a skill"),
            (u32::MAX, "not a skill"),
        ] {
            for level in [0u32, 1, 3, 20] {
                assert!(buff_level(id, level, 200).is_none(), "skill {id}: {why}");
            }
        }

        // ...and all three castable skills still answer, at their first and last levels.
        for (id, max) in [
            (IRON_BODY, IRON_BODY_MAX_LEVEL),
            (FOCUS, FOCUS_MAX_LEVEL),
            (DARK_SIGHT, DARK_SIGHT_MAX_LEVEL),
        ] {
            assert!(buff_level(id, 1, 200).is_some(), "skill {id} level 1");
            assert!(buff_level(id, max, 200).is_some(), "skill {id} level {max}");
            assert!(buff_level(id, max + 1, 200).is_none(), "skill {id} past master");
        }

        assert_eq!(IRON_BODY, 1_001_000);
        assert_eq!(FOCUS, 3_001_000);
        assert_eq!(DISORDER, 4_001_000);
        assert_eq!(DARK_SIGHT, 4_001_001);
    }

    /// **Only Iron Body reads the Weapon Def. parameter.**
    ///
    /// If that ever stops being true the parameter becomes a trap of its own - a caller
    /// passing a placeholder for Focus would silently change the buff.
    #[test]
    fn only_iron_body_reads_the_weapon_defence_parameter() {
        for wdef in [0i16, 1, 100, 5000, -1, i16::MAX] {
            for (skill, max) in [(FOCUS, FOCUS_MAX_LEVEL), (DARK_SIGHT, DARK_SIGHT_MAX_LEVEL)] {
                for level in 1..=max {
                    assert_eq!(
                        buff_level(skill, level, wdef),
                        buff_level(skill, level, 0),
                        "skill {skill} level {level} must ignore weapon_defence {wdef}"
                    );
                }
            }
        }
        // ...and Iron Body does not ignore it.
        assert_ne!(buff_level(IRON_BODY, 20, 0), buff_level(IRON_BODY, 20, 400));
    }

    /// **The nine passives, and which two are not settled.**
    ///
    /// Seven are `Client` because three client functions were disassembled that resolve a
    /// passive from the client's own record with no packet in the path; the two max-pool
    /// skills are `Unmeasured` because the number they change is one this server owns.
    #[test]
    fn the_passives_are_the_clients_job_except_the_two_that_change_a_server_owned_number() {
        for id in [1_000_000u32, 1_000_002, 2_000_000, 3_000_000, 3_000_001, 4_000_000, 4_000_001]
        {
            assert_eq!(passive_owner(id), Some(PassiveOwner::Client), "skill {id}");
            assert!(buff_level(id, 1, 200).is_none(), "and it is not a buff packet");
        }
        for id in [1_000_001u32, 2_000_001] {
            assert_eq!(passive_owner(id), Some(PassiveOwner::Unmeasured), "skill {id}");
        }
        // Nine, and nine only - a castable skill is not a passive.
        for id in [IRON_BODY, FOCUS, DISORDER, DARK_SIGHT, 1002, 2_001_000, 0, u32::MAX] {
            assert_eq!(passive_owner(id), None, "skill {id} is not a type 50 passive");
        }
    }
}
