//! **Improved HP/MP Recovery's item bonus** - the pure decision.
//!
//! The owner, 2026-08-30: *"I'm testing the new magician skills, one of the passive for 'Improved
//! MP Recovery' says it will increase MP recovery item recovery amount by 20%, it currently
//! does not do that."*
//!
//! Full working, every census and every named negative: **`research/item-recovery.md`**.
//!
//! Labels are the project's: **[L]** read off this client's own data or off a capture,
//! **[D]** derived from two or more [L] facts, **[I]** inferred - a policy nothing on this
//! machine can confirm.
//!
//! # Wired since 2026-09-01 (`cc5511f`) - this heading used to say the opposite
//!
//! `session/consume.rs` reads both skill levels and calls [`restored`] before it caps the
//! restore, exactly as `research/item-recovery.md` §7 asked. The heading above said *"THIS
//! MODULE IS NOT WIRED"* for five days after that landed, which is the mirror of the
//! `CLAUDE.md` failure it was written to prevent: a loud "unwired" that outlives the wiring
//! sends the next reader to re-do finished work. Corrected 2026-09-06 while answering
//! whether the passives work. **Not yet confirmed on a screen** - the 08-30 report that
//! found it missing is the last time a potion's bonus was watched.
//!
//! It is the **pure decision**, the same shape as [`crate::secondjob`]: given what the
//! character has learned and what the item restores, how much lands. It sends no packets,
//! touches no database and knows nothing about sessions.
//!
//! # They are right, and this is the third time this exact shape has been reported
//!
//! `crates/world/src/session/consume.rs` computes the restore with
//! `restores.hp_for(chr.max_hp)` / `restores.mp_for(chr.max_mp)` and **there is no skill
//! lookup anywhere on that path**. The two siblings are already documented as the same bug:
//! `session::buff::magic_guard_percent` (*"setting the bit buys an icon, and the arithmetic
//! that makes the buff mean something is the server's"*) and
//! `session::buff::held_weapon_defence` (the owner: *"Iron Body did not seem to reduce the damage
//! I take."*). This is the third.
//!
//! ## And it is measured, not only reasoned
//!
//! `previous-runs/world-20260830-221224.log`, the run from the day of the owner's report:
//!
//! ```text
//!   character 213 "Cobalt", job 200, max MP 237
//!   maplecw.db character_skills: (213, 2000000, 15)      <- level 15, y = 20%
//!   gm-handbook/consumables.txt: 2000003, 0, 200, 0, 0   <- 200 flat MP
//!
//!   02:0x:xx  used item 2000003 from Use slot 5 - +0 hp, +200 mp   x 11
//! ```
//!
//! **200, eleven times, where 20% of 200 is 40.** **[L]** - one file, one character, the
//! database read directly. The screen and the log agree, which is the combination
//! `CLAUDE.md` asks for before building on a user report.
//!
//! # The columns, and which is which
//!
//! Read out of `gm-handbook/skills.txt`, which `tools/dump_skills.py` generates from the
//! client's own `Skill.wz` and `String.wz`. **[L]**, every level of both skills:
//!
//! ```text
//!   1000000  Improved HP Recovery  job 100  maxLevel 15  type 50  psd absent
//!            x ABSENT at every level          y = 5..20
//!            "Regenerates HP every 10 seconds; increases HP recovery from items by N%"
//!
//!   2000000  Improved MP Recovery  job 200  maxLevel 15  type 50  psd absent
//!            x = 1 at every level             y = 5..20
//!            "Regenerates 1% of Max MP every 10 seconds; increases MP recovery from items by N%"
//! ```
//!
//! Three things there are worth more than the fix:
//!
//! * **`y` is the item bonus, and the tooltip says so in words.** Level 1's `y` is `5` and
//!   level 1's tooltip ends *"by 5%"*; level 15's `y` is `20` and it ends *"by 20%"*. Fifteen
//!   levels, fifteen agreements, zero counterexamples. **[L]** This is the client's own
//!   statement of the unit, which is what `CLAUDE.md`'s *"the unit, not the arithmetic"*
//!   section asks for when a field's meaning would otherwise come from a comment.
//! * **`y` is NOT `level + 4`.** It runs `5,6,7,…,17,18,` **`20`** - level 14 is `18` and
//!   level 15 jumps to `20`. **There is no level whose `y` is 19.** A formula would be wrong
//!   at the top level, which is precisely the level `!learn` grants and the only one the owner has
//!   ever held. [`ITEM_BONUS_PERCENT`] is therefore a **table**, and
//!   [`tests::the_table_is_the_generated_files_own_y_column`] asserts it against the file.
//! * **`1000000` carries no `x` at all.** Its twin does. The two skills are *not* the same
//!   shape and a reader who assumes they are will invent an HP regen number that is nowhere
//!   in this client's data - the tooltip does not state one either. Only the `y` half is
//!   symmetric, and this module implements only the `y` half.
//!
//! # The second bonus on the same skill, and why its TABLE lives here
//!
//! **`2000000`'s `x` - "Regenerates 1% of Max MP every 10 seconds" - is a second bonus on a
//! second server-computed number.** It was unwired for a day after the `y` half landed; it is
//! wired now, and the split of ownership is worth stating because it is not the obvious one:
//!
//! * **The column lives here** - [`MP_REGEN_PERCENT_OF_MAX`] and [`mp_regen_percent`]. Skill
//!   `2000000`'s WZ row is transcribed in exactly **one** file and checked by exactly **one**
//!   file-backed harness ([`tests::real_skills`]). Two transcriptions of one row is how one of
//!   them goes stale, and this module already owned the other half of that same row.
//! * **The composition lives in `crates/world/src/session/regen.rs`** - whether the percentage
//!   adds to the flat base, replaces it, or floors it, and on which clock. That is a question
//!   about idle regeneration rather than about this skill, and the working is in
//!   `research/mp-regen.md`.
//!
//! Nothing here knows what the base regeneration is, deliberately: this module answers *"what
//! does the skill say"*, and `regen.rs` answers *"what does the player get"*.
//!
//! Implementing one half and calling the skill "done" is exactly the failure `CLAUDE.md`'s
//! Heena-quest section describes - *a test that checks one of several effects gives false
//! confidence about the rest*.
//!
//! # Which restores it applies to, and which half of that is measured
//!
//! | question | answer | label |
//! |---|---|---|
//! | flat (`hp`/`mp`) or percentage (`hpR`/`mpR`)? | **both** | **[I]** - see below |
//! | HP skill on MP items? | **no.** Each skill bonuses its own pool only | **[L]** from the tooltip wording |
//! | applied before or after the missing-HP cap? | **before** | **[D]** |
//!
//! The tooltip says *"increases MP recovery from items"*. It names the **pool** (MP) and the
//! **source** (items) and it does not name a term, so the pool half is [L] - "Improved **MP**
//! Recovery … increases **MP** recovery" cannot reasonably bonus HP, and the HP twin's own
//! tooltip says HP in the same two places. **Seven of this client's 44 consumables restore
//! both pools** (Elixir `2000005` is `hpR 100 / mpR 100`), so this is not a hypothetical
//! distinction: a Magician drinking an Elixir gets the bonus on the MP half and not the HP
//! half.
//!
//! The flat-vs-percentage half is **[I]**, and it is [I] because *nothing in this client can
//! disagree with either reading*: **no item in `gm-handbook/consumables.txt` carries both a
//! flat and a percentage term on the same pool** - 0 of 44 for HP, 0 of 44 for MP, both
//! directions counted. So "bonus the total" and "bonus each term" produce identical numbers
//! on every item this client has, and the choice is unobservable. The total is bonused
//! because that is the number the item's own tooltip promises the player, and it is the one
//! `Restores::hp_for` already returns.
//!
//! # Rounding: floor, and the reason is not "it was easiest"
//!
//! [`boosted`] is `amount * (100 + percent) / 100` in `u64`, truncated. **[I]** - a policy.
//! Three reasons, in the order they mattered:
//!
//! 1. **It is what every other percentage in this repo already does.**
//!    `consumables::Restores::hp_for` is `max_hp * hp_percent / 100`;
//!    `session::combat::on_user_hit`'s Magic Guard split is
//!    `u64::from(applied) * u64::from(guard_percent) / 100`. A fourth rounding rule in a
//!    fourth place is how two of them end up disagreeing.
//! 2. **It can never pay more than the tooltip promises.** Round-half-up can, by one point.
//! 3. **A future reader who "simplifies" this to integer division gets the same answer.**
//!
//! The cost of that choice is bounded and was measured rather than assumed: **across every
//! flat amount this client's consumables carry (30, 50, 100, 150, 200, 300, 450, 800, 900,
//! 1500, 2000, 2500) and every level 1..=15, there is no pair where the bonus rounds away to
//! nothing.** The smallest case is `floor(30 * 105 / 100) = 31`. **[L]**
//! [`tests::no_real_item_at_any_level_gets_a_bonus_that_rounds_to_nothing`] is that
//! enumeration against the generated file, not a sample.
//!
//! ## The one place the choice is visible, and it is pinned
//!
//! A percentage restore floors **twice** - once in `Restores::mp_for`, once here:
//!
//! ```text
//!   max MP 237, an item with mpR 35, skill level 15 (y = 20)
//!     floor(237 * 35 / 100)      = 82      <- Restores::mp_for
//!     floor(82 * 120 / 100)      = 98      <- this module, and what the player gets
//!     floor(237 * 35 * 120/10000) = 99     <- what one fused multiply would give
//! ```
//!
//! One point, and this module takes the 98. It composes with `Restores` rather than
//! reimplementing it, so the base amount has exactly one definition.
//! [`tests::a_percentage_restore_floors_twice_and_that_is_deliberate`] pins it so the choice
//! cannot be changed by accident.
//!
//! # Why the table is hard-coded when `skilltable` already parses `y`
//!
//! Because **a missing `gm-handbook/` must not silently turn the bonus off.**
//! `SkillTable::load` answers an empty table for a missing file, by design, and a
//! `bonus_percent` that read `y` at runtime would then return `0` on every level - a working
//! server, a learned skill, and a passive that does nothing, which is the exact screen this
//! module exists to fix. `Config::map_exists` was fail-open in the same way for a day and
//! `session::consume::use_return_scroll` now shouts about it.
//!
//! So [`ITEM_BONUS_PERCENT`] is the runtime source and [`bonus_percent_from_wz`] exists only
//! so a test can prove the two agree. That is `CLAUDE.md`'s *"a constant that came from
//! reading data is a claim, not a fact - assert it against something that can disagree"*.

use crate::consumables::Restores;
use crate::skilltable::SkillLevel;

// ---------------------------------------------------------------------------------------
// The numbers
// ---------------------------------------------------------------------------------------

/// Warrior **Improved HP Recovery**. Bonuses the **HP** half of a restore, and only that.
pub const IMPROVED_HP_RECOVERY: u32 = 1_000_000;

/// Magician **Improved MP Recovery**. Bonuses the **MP** half of a restore, and only that.
pub const IMPROVED_MP_RECOVERY: u32 = 2_000_000;

/// Both skills' ceiling. **[L]** - the `maxLevel` column, 15 on all 30 rows.
pub const MAX_LEVEL: u32 = 15;

/// The `y` column of both skills, indexed by `level - 1`. **PERCENT**, added to 100.
///
/// **[L]**, read off `gm-handbook/skills.txt` and corroborated by the client's own tooltip on
/// every one of the fifteen levels. `1000000` and `2000000` carry the identical fifteen
/// values, which is measured rather than assumed -
/// [`tests::the_table_is_the_generated_files_own_y_column`] checks both skills separately.
///
/// **This is not an arithmetic progression.** `level + 4` is right for levels 1..=14 and
/// **wrong for level 15**, which is the only level anyone here has ever held.
pub const ITEM_BONUS_PERCENT: [u32; MAX_LEVEL as usize] =
    [5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20];

/// The `x` column of [`IMPROVED_MP_RECOVERY`], indexed by `level - 1`. **PERCENT OF MAX MP**,
/// per idle tick. `1` at every level - it is a table only so that the shape matches its `y`
/// sibling and a future WZ that varies it cannot be missed.
///
/// # The unit is the client's own word, and it is NOT the same unit as the twin's
///
/// The tooltip, verbatim from `String.wz`, identical on all fifteen levels: *"Regenerates 1%
/// of Max MP every 10 seconds; increases MP recovery from items by N%"*. **[L]** It names the
/// pool (Max MP), the proportion (`%`) and the clock (10 seconds) in one clause.
///
/// That the `1` is **this column** rather than a literal baked into the string is **[D]**, and
/// it has to be, because `x` never varies here so no correlation can be observed. The control
/// is the second-job twin, which is the same skill name on the same clock with a *varying*
/// `x`:
///
/// ```text
///   1110000 / 1210000  Improved MP Recovery   x = 3..22 over 20 levels
///     level  1  "Recover 3 additional MP every 10 sec."
///     level 20  "Recover 22 additional MP every 10 sec."     <- the number IS x, 20 for 20
/// ```
///
/// **And the two are different units on the same column.** `1110000`'s slot has no `%` and
/// says *"additional MP"*; `2000000`'s has a `%` and says *"of Max MP"*. Reading `2000000`'s
/// `x` as flat gives a 237-MP Magician **1** MP per tick where the client promises **2**, and
/// reading `1110000`'s as a percentage would give a Fighter 22% of their pool. This is
/// `CLAUDE.md`'s *"the unit, not the arithmetic"* with both readings live in one column of one
/// book - see `research/mp-regen.md` §2.
pub const MP_REGEN_PERCENT_OF_MAX: [u32; MAX_LEVEL as usize] = [1; MAX_LEVEL as usize];

// ---------------------------------------------------------------------------------------
// What the character has learned
// ---------------------------------------------------------------------------------------

/// The two learned levels, named so they cannot be crossed.
///
/// Two `u32`s in a positional pair would let a caller swap them and produce a Magician whose
/// MP potions are bonused by a Warrior skill they cannot have. Named fields make that
/// unwritable rather than merely unlikely. `0` means *not learned* and is the default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Learned {
    /// The character's level in [`IMPROVED_HP_RECOVERY`], or `0`.
    pub improved_hp_recovery: u32,
    /// The character's level in [`IMPROVED_MP_RECOVERY`], or `0`.
    pub improved_mp_recovery: u32,
}

impl Learned {
    /// Nobody has learned anything - the value for a character with no rows.
    pub fn none() -> Self {
        Learned::default()
    }
}

// ---------------------------------------------------------------------------------------
// The decision
// ---------------------------------------------------------------------------------------

/// The percent bonus `skill_id` at `level` adds, or `0`.
///
/// `0` for level `0` (not learned) and for **any id that is not one of the two**, which is
/// the guard that stops a caller bonusing MP with the Warrior skill.
///
/// # A level above the ceiling saturates rather than returning zero
///
/// Nothing in this server should produce a level above [`MAX_LEVEL`] - `skilltable` refuses
/// the raise - so a level of 16 means a bug somewhere else. It saturates at the top row
/// instead of returning `0`, because `0` would mean *"over-levelling the skill switches it
/// off"*, which is the same silent nothing this module exists to remove. The wrong answer is
/// one point of bonus; the other wrong answer is the bug back again.
pub fn bonus_percent(skill_id: u32, level: u32) -> u32 {
    if skill_id != IMPROVED_HP_RECOVERY && skill_id != IMPROVED_MP_RECOVERY {
        return 0;
    }
    if level == 0 {
        return 0;
    }
    let idx = (level.min(MAX_LEVEL) - 1) as usize;
    ITEM_BONUS_PERCENT[idx]
}

/// The same answer, read out of a loaded [`SkillLevel`] instead of the table above.
///
/// **Not the runtime path.** This exists so a test can prove [`ITEM_BONUS_PERCENT`] is the
/// generated file's own `y` column rather than somebody's memory of it; see the module
/// header for why the runtime path must not depend on a generated file.
///
/// `None` means the id is not one of the two, or the row carries no `y`. A **negative** `y`
/// yields `Some(0)` rather than a subtraction: no tooltip in this client describes a
/// negative item bonus, and quietly healing *less* than the item says would read on screen
/// as the item being broken rather than as the skill being wrong.
pub fn bonus_percent_from_wz(skill_id: u32, row: &SkillLevel) -> Option<u32> {
    if skill_id != IMPROVED_HP_RECOVERY && skill_id != IMPROVED_MP_RECOVERY {
        return None;
    }
    Some(u32::try_from(row.y?).unwrap_or(0))
}

/// The percent of **max MP** that [`IMPROVED_MP_RECOVERY`] regenerates per idle tick, or `0`.
///
/// `0` for level `0` (not learned) and for **any other skill id**, including
/// [`IMPROVED_HP_RECOVERY`] - which is the guard that matters most in this whole module.
///
/// # `1000000` must return 0 here, and that is a finding rather than an omission
///
/// The Warrior twin carries **no `x` column at any of its fifteen levels** and its tooltip
/// states no HP number either - just *"Regenerates HP every 10 seconds"*. **[L]**, enumerated
/// over the generated file by [`tests::only_the_magician_half_carries_an_x_column`]. So there
/// is **no HP regeneration amount in this client's data at all**, and a server that answers
/// one here is inventing it. The symmetry with `y` is a trap: `y` is identical on both skills,
/// `x` exists on only one of them.
///
/// Saturates at the top row for an impossible level, for the same reason
/// [`bonus_percent`] does: returning `0` would make over-levelling switch the skill off, which
/// is the silent nothing this module exists to remove.
pub fn mp_regen_percent(skill_id: u32, level: u32) -> u32 {
    if skill_id != IMPROVED_MP_RECOVERY {
        return 0;
    }
    if level == 0 {
        return 0;
    }
    MP_REGEN_PERCENT_OF_MAX[(level.min(MAX_LEVEL) - 1) as usize]
}

/// The same answer read out of a loaded [`SkillLevel`] instead of the table above.
///
/// **Not the runtime path** - it exists only so a test can prove [`MP_REGEN_PERCENT_OF_MAX`]
/// is the generated file's own `x` column. See the module header for why the runtime path must
/// not depend on a gitignored generated file.
///
/// `None` means the id is not [`IMPROVED_MP_RECOVERY`], or the row carries no `x` - which is
/// every row of [`IMPROVED_HP_RECOVERY`]. A **negative** `x` yields `Some(0)`: no tooltip in
/// this client describes a negative regeneration, and draining a player who learned a recovery
/// passive would read on screen as poison.
pub fn mp_regen_percent_from_wz(skill_id: u32, row: &SkillLevel) -> Option<u32> {
    if skill_id != IMPROVED_MP_RECOVERY {
        return None;
    }
    Some(u32::try_from(row.x?).unwrap_or(0))
}

/// `percent` percent of `max_mp`, **rounded down** - what the skill adds to one idle tick.
///
/// Floors, like every other percentage in this repo ([`boosted`], `Restores::mp_for`,
/// `session::combat::on_user_hit`'s Magic Guard split). The floor is load-bearing at the
/// bottom of the range and it is why `regen.rs` **adds** this to the flat base rather than
/// replacing it: a Magician whose max MP is under 100 gets `floor(< 1) = 0` from the skill, so
/// a replacement reading would regenerate them **nothing at all**. See `research/mp-regen.md`
/// §3.
///
/// `u64` throughout so a large pool cannot wrap; saturates at `u32::MAX`.
pub fn regen_of_max(max_mp: u32, percent: u32) -> u32 {
    if percent == 0 {
        return 0;
    }
    u32::try_from(u64::from(max_mp) * u64::from(percent) / 100).unwrap_or(u32::MAX)
}

/// The maximum the **client draws** when a max-pool passive is learned: `base` plus
/// `percent` percent of it, **rounded down**.
///
/// The owner, 2026-09-06, with a screenshot: *"Max HP Increase results in Cobalt having more HP on
/// client side, but natural regeneration does not regenerate that amount, which seems to be
/// meaning that the server thinks that Cobalt is at max HP already."* They are right on every
/// clause, and the screenshot is the measurement `research/magician-first-job.md` §8
/// experiment A had been waiting for:
///
/// ```text
///   maplecw.db      Cobalt  hp 358  max_hp 358   skill 1000001 at level 15 (mhpR 25)
///   world.log       "idle regen +0 hp ... 358/358 hp - HP is full"
///   the screen      HP 358 / 447
///
///   358 + 358 * 25 / 100 = 358 + 89.5 -> 447     (447.5 would be 448: the client FLOORS)
/// ```
///
/// **[L]**: the client applies the percent itself, on top of the maximum this server sends.
/// Two consequences, both enforced through this one function: the server must **never** fold
/// the percent into `max_hp`/`max_mp` on the wire (the client would apply it again), and it
/// must raise every ceiling it caps HP or MP against - regen, potions, Recovery, the level-up
/// refill, `!heal`, a quest's set-HP, the party bar - or it keeps a 358/447 character "full"
/// and, worse, a potion drunk at 400 would *lower* them to 358. `session::pools` is the one
/// place those ceilings come from.
///
/// Truncation rather than rounding is what 447 says; one data point at one level, so **[L]**
/// for 25% and **[D]** that the same expression holds at the other fourteen.
pub fn boosted_max(base: u32, percent: u32) -> u32 {
    if percent == 0 {
        return base;
    }
    let bonus = u64::from(base) * u64::from(percent) / 100;
    u32::try_from(u64::from(base) + bonus).unwrap_or(u32::MAX)
}

/// `amount` increased by `percent` percent, **rounded down**.
///
/// `u64` throughout: `u32::MAX * 120` overflows a `u32` and the wrap would hand a player a
/// tiny heal from a huge one. Saturates at `u32::MAX` rather than wrapping.
///
/// See the module header for why this floors. A `percent` of `0` returns `amount` unchanged,
/// exactly, with no rounding of any kind.
pub fn boosted(amount: u32, percent: u32) -> u32 {
    if percent == 0 {
        return amount;
    }
    let scaled = u64::from(amount) * (100 + u64::from(percent)) / 100;
    u32::try_from(scaled).unwrap_or(u32::MAX)
}

/// What one use of an item restores, with the bonus in and before any cap.
///
/// The base amounts and the bonuses are all carried, not just the totals, because the log
/// line at the call site should be able to say *why* a potion healed 240 - `CLAUDE.md`'s rule
/// that a disagreement worth measuring goes in the log rather than being reasoned about
/// later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Restored {
    /// What the item alone restores - `Restores::hp_for(max_hp)`.
    pub hp_base: u32,
    /// The HP half after [`IMPROVED_HP_RECOVERY`]. Equal to [`Self::hp_base`] when unlearned.
    pub hp: u32,
    /// The percent that produced it, for the log line. `0` when unlearned.
    pub hp_bonus_percent: u32,
    /// What the item alone restores - `Restores::mp_for(max_mp)`.
    pub mp_base: u32,
    /// The MP half after [`IMPROVED_MP_RECOVERY`].
    pub mp: u32,
    /// The percent that produced it, for the log line.
    pub mp_bonus_percent: u32,
}

impl Restored {
    /// Did either skill actually change a number? Use this to decide whether the log line is
    /// worth the extra words, never to decide whether to apply the bonus.
    pub fn any_bonus_applied(&self) -> bool {
        self.hp > self.hp_base || self.mp > self.mp_base
    }

    /// What actually lands, **capped at what is missing**, as `(hp_gain, mp_gain)`.
    ///
    /// The cap is here rather than left to the caller for the reason `CLAUDE.md`'s Heena
    /// section gives: an effect that hangs off a separate step is the one that gets missed.
    /// An item healing 240 on a character missing 3 heals 3, and telling the client otherwise
    /// puts a number above the maximum into the HP field, which the client draws as a bar
    /// past its own end.
    ///
    /// The bonus is applied **before** this, which is the only order that lets the bonus
    /// matter at all: capping first and then multiplying would let a full-health drink
    /// overshoot.
    pub fn capped(&self, hp: u32, max_hp: u32, mp: u32, max_mp: u32) -> (u32, u32) {
        (self.hp.min(max_hp.saturating_sub(hp)), self.mp.min(max_mp.saturating_sub(mp)))
    }
}

/// **The decision.** What `restores` gives this character, with both passives folded in.
///
/// Neither skill can touch the other's pool: [`Learned::improved_hp_recovery`] reaches only
/// [`Restored::hp`] and [`Learned::improved_mp_recovery`] only [`Restored::mp`]. That is the
/// tooltip's own reading and there is a test in both directions.
///
/// The result is **uncapped**. Call [`Restored::capped`] with the character's current and
/// maximum pools; see its docs for why the order is not negotiable.
pub fn restored(restores: &Restores, max_hp: u32, max_mp: u32, learned: Learned) -> Restored {
    let hp_base = restores.hp_for(max_hp);
    let mp_base = restores.mp_for(max_mp);
    let hp_bonus_percent = bonus_percent(IMPROVED_HP_RECOVERY, learned.improved_hp_recovery);
    let mp_bonus_percent = bonus_percent(IMPROVED_MP_RECOVERY, learned.improved_mp_recovery);
    Restored {
        hp_base,
        hp: boosted(hp_base, hp_bonus_percent),
        hp_bonus_percent,
        mp_base,
        mp: boosted(mp_base, mp_bonus_percent),
        mp_bonus_percent,
    }
}

// ---------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consumables::Consumables;
    use crate::skilltable::SkillTable;
    use std::path::Path;

    const REAL_SKILLS: &str = "../../gm-handbook/skills.txt";
    const REAL_CONSUMABLES: &str = "../../gm-handbook/consumables.txt";

    /// The real skill table, or `None` on a clean checkout where `gm-handbook/` has not been
    /// generated.
    ///
    /// **The positive control is inside this helper, not in the callers**, so that a file
    /// which exists but has stopped containing these two skills fails loudly instead of
    /// letting every test below pass vacuously. `CLAUDE.md`: a check that silently skips
    /// looks exactly like one that passed.
    fn real_skills() -> Option<SkillTable> {
        let path = Path::new(REAL_SKILLS);
        if !path.exists() {
            return None;
        }
        let t = SkillTable::load(path);
        assert!(t.len() > 100, "the whole archive loaded: {}", t.banner());
        for id in [IMPROVED_HP_RECOVERY, IMPROVED_MP_RECOVERY] {
            let s = t
                .get(id)
                .unwrap_or_else(|| panic!("skill {id} is missing from {REAL_SKILLS}"));
            assert_eq!(s.max_level, MAX_LEVEL, "skill {id} ceiling");
            assert_eq!(s.levels_loaded(), MAX_LEVEL as usize, "skill {id} level rows");
        }
        Some(t)
    }

    /// The real consumables table, with its own positive control.
    fn real_consumables() -> Option<Consumables> {
        let path = Path::new(REAL_CONSUMABLES);
        if !path.exists() {
            return None;
        }
        let c = Consumables::load(path);
        assert!(!c.is_empty(), "{REAL_CONSUMABLES} exists but loaded nothing: {}", c.banner());
        // The Red Potion is the row every other test in this repo anchors on.
        assert_eq!(c.get(2_000_000).map(|r| r.hp), Some(100), "Red Potion is 100 flat HP");
        Some(c)
    }

    /// Every row of the real table, as `(itemId, Restores)`.
    ///
    /// `Consumables` exposes no iterator, so this sweeps an id range - and the sweep is
    /// **checked against `Consumables::len`**, which is the whole point. The first version of
    /// this helper swept `2_000_000 ..= 2_000_000 + u16::MAX` and silently missed the seven
    /// `2210xxx` items, which is `CLAUDE.md`'s *"enumerate before you filter"* in miniature:
    /// a range that looks generous is still a filter, and only a count can tell you it was
    /// too small.
    ///
    /// **It happened a second time on 2026-09-09**, and the count caught it again: the table
    /// grew buff columns, so items that buff without restoring joined it - among them
    /// `2450001`, the 3x EXP Coupon, which is past the old `2_400_000` end. The range is now
    /// the whole Use-tab id space, because any narrower bound is a guess about which items
    /// the client's own data happens to contain.
    fn every_restoring_item(c: &Consumables) -> Vec<(u32, Restores)> {
        let all: Vec<(u32, Restores)> =
            (2_000_000u32..=2_999_999).filter_map(|id| c.get(id).map(|r| (id, r))).collect();
        assert_eq!(all.len(), c.len(), "the id sweep missed rows the table holds");
        assert!(all.len() > 20, "the sweep found only {} items", all.len());
        all
    }

    // -----------------------------------------------------------------------------------
    // The table is the client's, not a memory of MapleStory
    // -----------------------------------------------------------------------------------

    /// **[`ITEM_BONUS_PERCENT`] is the generated file's own `y` column**, for both skills,
    /// at every level - checked through [`bonus_percent_from_wz`], which is a different code
    /// path from the one under test.
    #[test]
    fn the_table_is_the_generated_files_own_y_column() {
        let Some(t) = real_skills() else { return };
        for id in [IMPROVED_HP_RECOVERY, IMPROVED_MP_RECOVERY] {
            let s = t.get(id).expect("the control above already proved it is there");
            for level in 1..=MAX_LEVEL {
                let row = s.level(level).unwrap_or_else(|| panic!("{id} level {level}"));
                assert_eq!(
                    bonus_percent_from_wz(id, row),
                    Some(bonus_percent(id, level)),
                    "skill {id} level {level}: the hard-coded table disagrees with the WZ `y`"
                );
            }
        }
    }

    /// **`y` is not `level + 4`**, and this is the assertion that would have caught a formula.
    ///
    /// Level 14 is 18 and level 15 is **20**. Nothing in the archive has a `y` of 19.
    #[test]
    fn the_top_level_breaks_the_arithmetic_progression() {
        assert_eq!(bonus_percent(IMPROVED_MP_RECOVERY, 14), 18);
        assert_eq!(bonus_percent(IMPROVED_MP_RECOVERY, 15), 20, "not 19");
        assert!(!ITEM_BONUS_PERCENT.contains(&19), "no level of either skill gives 19%");
        // And every level below the top *is* level + 4, which is what makes the exception
        // easy to miss.
        for level in 1..MAX_LEVEL {
            assert_eq!(bonus_percent(IMPROVED_HP_RECOVERY, level), level + 4, "level {level}");
        }
    }

    /// **`1000000` carries no `x` and `2000000` carries `x = 1`.** They are not the same
    /// shape, and a reader who assumes they are invents an HP regen number this client does
    /// not have.
    #[test]
    fn only_the_magician_half_carries_an_x_column() {
        let Some(t) = real_skills() else { return };
        for level in 1..=MAX_LEVEL {
            assert_eq!(
                t.get(IMPROVED_HP_RECOVERY).unwrap().level(level).unwrap().x,
                None,
                "Improved HP Recovery level {level} has no `x`"
            );
            assert_eq!(
                t.get(IMPROVED_MP_RECOVERY).unwrap().level(level).unwrap().x,
                Some(1),
                "Improved MP Recovery level {level} regenerates 1% of Max MP"
            );
        }
    }

    /// **[`MP_REGEN_PERCENT_OF_MAX`] is the generated file's own `x` column**, checked through
    /// [`mp_regen_percent_from_wz`] - a different code path from the one under test, exactly
    /// as its `y` sibling above is.
    ///
    /// Verified by mutation, not merely by passing: changing the table to `[2; 15]` fails with
    /// *"level 1: the hard-coded x disagrees with the WZ"*.
    #[test]
    fn the_x_table_is_the_generated_files_own_x_column() {
        let Some(t) = real_skills() else { return };
        let s = t.get(IMPROVED_MP_RECOVERY).expect("the control above already proved it");
        for level in 1..=MAX_LEVEL {
            let row = s.level(level).unwrap_or_else(|| panic!("level {level}"));
            assert_eq!(
                mp_regen_percent_from_wz(IMPROVED_MP_RECOVERY, row),
                Some(mp_regen_percent(IMPROVED_MP_RECOVERY, level)),
                "level {level}: the hard-coded x disagrees with the WZ"
            );
        }
    }

    /// **The Warrior twin yields no regeneration percentage, by either route.**
    ///
    /// This is the assertion that stops an HP regen number being invented. `1000000` has no
    /// `x` at any level (the test above pins that against the file); both entry points must
    /// therefore answer "nothing" rather than falling through to the MP table because the two
    /// skills share a `y`.
    #[test]
    fn the_warrior_twin_yields_no_regen_percentage_by_either_route() {
        for level in 0..=30 {
            assert_eq!(
                mp_regen_percent(IMPROVED_HP_RECOVERY, level),
                0,
                "Improved HP Recovery level {level} must not produce a regen percent"
            );
        }
        // And no other id can either - including the second-job skills of the same NAME
        // (1110000 / 1210000), whose `x` is a FLAT MP amount and would be a unit error here.
        for id in [0, 1, 1_000_001, 2_000_001, 1_110_000, 1_210_000, 2_100_000] {
            for level in 0..=30 {
                assert_eq!(mp_regen_percent(id, level), 0, "skill {id} level {level}");
            }
        }

        let Some(t) = real_skills() else { return };
        for level in 1..=MAX_LEVEL {
            let row = t.get(IMPROVED_HP_RECOVERY).unwrap().level(level).unwrap();
            assert_eq!(
                mp_regen_percent_from_wz(IMPROVED_HP_RECOVERY, row),
                None,
                "level {level}: the HP skill must not answer a regen percent from the WZ either"
            );
        }
    }

    /// **The floor bites at the bottom of the range, and that is why `regen.rs` adds.**
    ///
    /// 1% of a pool under 100 is zero. A fresh Magician's max MP is far below that, so a
    /// "replace the base" reading would regenerate them *nothing*; adding to the flat base
    /// cannot. The numbers are pinned here rather than left to integer division to decide
    /// quietly.
    #[test]
    fn one_percent_of_a_small_pool_floors_to_nothing() {
        assert_eq!(regen_of_max(5, 1), 0, "a fresh Magician's 5 max MP");
        assert_eq!(regen_of_max(99, 1), 0, "still nothing at 99");
        assert_eq!(regen_of_max(100, 1), 1, "and one point at exactly 100");
        assert_eq!(regen_of_max(237, 1), 2, "floor(2.37) - Cobalt's actual pool");
        assert_eq!(regen_of_max(1000, 1), 10, "at 1000 max MP it finally equals the flat base");
        assert_eq!(regen_of_max(237, 0), 0, "unlearned is nothing, not a rounding");
        assert_eq!(regen_of_max(u32::MAX, 1), u32::MAX / 100, "no wrap at the top");
    }

    // -----------------------------------------------------------------------------------
    // The guard: neither skill touches the other's pool
    // -----------------------------------------------------------------------------------

    /// **The Magician's skill does not heal HP**, and the Warrior's does not restore MP.
    /// Asserted in both directions, because a one-directional test passes on a function that
    /// bonuses everything.
    #[test]
    fn each_skill_bonuses_its_own_pool_and_only_its_own() {
        // Elixir-shaped: 100 flat to each pool, so a leak in either direction is visible.
        let both = Consumables::parse("9000000, 100, 100, 0, 0\n").get(9_000_000).unwrap();

        let magician = restored(
            &both,
            1000,
            1000,
            Learned { improved_hp_recovery: 0, improved_mp_recovery: 15 },
        );
        assert_eq!(magician.hp, 100, "the MP skill must not touch HP");
        assert_eq!(magician.mp, 120, "and it must touch MP");

        let warrior = restored(
            &both,
            1000,
            1000,
            Learned { improved_hp_recovery: 15, improved_mp_recovery: 0 },
        );
        assert_eq!(warrior.hp, 120);
        assert_eq!(warrior.mp, 100, "the HP skill must not touch MP");

        // And a character who somehow holds both gets both, independently.
        let both_learned = restored(
            &both,
            1000,
            1000,
            Learned { improved_hp_recovery: 1, improved_mp_recovery: 15 },
        );
        assert_eq!(both_learned.hp, 105, "level 1 is 5%");
        assert_eq!(both_learned.mp, 120, "level 15 is 20%");
    }

    /// A skill id that is not one of the two adds nothing, however it is asked.
    #[test]
    fn no_other_skill_id_can_produce_a_bonus() {
        for id in [0, 1, 1000, 1001, 1_000_001, 2_000_001, 2_001_003, 3_000_000, 4_000_001] {
            for level in 0..=30 {
                assert_eq!(bonus_percent(id, level), 0, "skill {id} level {level}");
            }
        }
    }

    /// An unlearned skill changes nothing at all - not "changes it by 0%", *nothing*, so
    /// `hp == hp_base` exactly and [`Restored::any_bonus_applied`] is false.
    #[test]
    fn an_unlearned_skill_leaves_the_amount_byte_for_byte_alone() {
        let potion = Consumables::parse("2000000, 100, 0, 0, 0\n").get(2_000_000).unwrap();
        let r = restored(&potion, 500, 500, Learned::none());
        assert_eq!(r.hp, 100);
        assert_eq!(r.hp_base, 100);
        assert_eq!(r.hp_bonus_percent, 0);
        assert!(!r.any_bonus_applied());
        assert_eq!(bonus_percent(IMPROVED_MP_RECOVERY, 0), 0, "level 0 is not learned");
    }

    // -----------------------------------------------------------------------------------
    // The owner's own case, end to end
    // -----------------------------------------------------------------------------------

    /// **The run that reported the bug, with the fix in.**
    ///
    /// `previous-runs/world-20260830-221224.log`: character 213, `2000000` at level 15 in
    /// `maplecw.db`, item `2000003` (200 flat MP), MP 7 of 237. The log recorded `+200 mp`
    /// eleven times. With the bonus applied it is **240**, capped by the 230 that were
    /// missing.
    #[test]
    fn josiahs_blue_potion_at_level_fifteen() {
        let elixir = Consumables::parse("2000003, 0, 200, 0, 0\n").get(2_000_003).unwrap();
        let r = restored(
            &elixir,
            342,
            237,
            Learned { improved_hp_recovery: 0, improved_mp_recovery: 15 },
        );
        assert_eq!(r.mp_base, 200, "what the run actually gave them");
        assert_eq!(r.mp, 240, "20% of 200 is 40");
        assert_eq!(r.mp_bonus_percent, 20);
        assert!(r.any_bonus_applied());

        // They were at 7/237, so 230 were missing and the cap binds.
        let (hp_gain, mp_gain) = r.capped(202, 342, 7, 237);
        assert_eq!(mp_gain, 230, "capped at what is missing, not at 240");
        assert_eq!(hp_gain, 0, "a blue potion is not a red one");

        // At full MP the same drink restores nothing - and still consumes the item, which is
        // the caller's rule and not this module's.
        assert_eq!(r.capped(202, 342, 237, 237).1, 0);
    }

    /// The cap is applied **after** the bonus, and this is the test that says so: a
    /// character missing exactly 220 MP gets 220, which is more than the unbonused 200 and
    /// less than the bonused 240. Cap-then-bonus and bonus-then-cap disagree here.
    #[test]
    fn the_bonus_is_applied_before_the_cap_and_that_is_visible() {
        let elixir = Consumables::parse("2000003, 0, 200, 0, 0\n").get(2_000_003).unwrap();
        let r = restored(&elixir, 500, 500, Learned { improved_mp_recovery: 15, ..Learned::none() });
        let (_, mp_gain) = r.capped(500, 500, 280, 500);
        assert_eq!(mp_gain, 220, "220 missing: more than 200, less than 240");
    }

    // -----------------------------------------------------------------------------------
    // Rounding, stated and pinned
    // -----------------------------------------------------------------------------------

    /// **Floor.** `10 MP` at level 1 is `10.5` and lands on `10`; at level 15 it is `12`.
    /// That first case is the off-by-one this module could most easily have got wrong, and
    /// it is written down rather than left to integer division to decide quietly.
    #[test]
    fn the_bonus_rounds_down() {
        assert_eq!(boosted(10, 5), 10, "floor(10.5) - level 1 adds nothing to a 10-point item");
        assert_eq!(boosted(10, 20), 12, "floor(12.0)");
        assert_eq!(boosted(3, 20), 3, "floor(3.6)");
        assert_eq!(boosted(5, 20), 6, "floor(6.0)");
        assert_eq!(boosted(0, 20), 0, "nothing boosted is nothing");
        assert_eq!(boosted(999, 0), 999, "0% is the identity, not a rounding");
    }

    /// A percentage restore floors twice, once in `Restores` and once here. **98, not 99.**
    /// Pinned so that composing with `Restores` rather than fusing the multiply stays a
    /// decision instead of becoming an accident.
    #[test]
    fn a_percentage_restore_floors_twice_and_that_is_deliberate() {
        let potion = Consumables::parse("9000001, 0, 0, 0, 35\n").get(9_000_001).unwrap();
        assert_eq!(potion.mp_for(237), 82, "floor(237 * 35 / 100)");
        let r = restored(&potion, 237, 237, Learned { improved_mp_recovery: 15, ..Learned::none() });
        assert_eq!(r.mp, 98, "floor(82 * 120 / 100); a fused multiply would give 99");
    }

    /// A percentage restore scales with the maximum **after** the bonus too - the bonus is a
    /// percentage of the restore, not a second percentage of the maximum.
    #[test]
    fn the_bonus_is_a_percent_of_the_restore_not_of_the_maximum() {
        let half = Consumables::parse("9000002, 0, 0, 50, 0\n").get(9_000_002).unwrap();
        let r = restored(&half, 1000, 1000, Learned { improved_hp_recovery: 15, ..Learned::none() });
        assert_eq!(r.hp_base, 500, "50% of 1000");
        assert_eq!(r.hp, 600, "500 + 20% of 500, NOT 700 = 70% of 1000");
    }

    /// Nothing overflows and nothing wraps, at the top of the range.
    #[test]
    fn a_huge_amount_saturates_instead_of_wrapping() {
        assert_eq!(boosted(u32::MAX, 20), u32::MAX);
        assert!(boosted(u32::MAX / 2, 20) > u32::MAX / 2);
    }

    /// A level above the ceiling saturates at level 15 rather than switching the skill off.
    #[test]
    fn an_impossible_level_saturates_rather_than_returning_zero() {
        assert_eq!(bonus_percent(IMPROVED_MP_RECOVERY, 16), 20);
        assert_eq!(bonus_percent(IMPROVED_MP_RECOVERY, 999), 20);
    }

    // -----------------------------------------------------------------------------------
    // Against the whole generated consumables table, not a sample
    // -----------------------------------------------------------------------------------

    /// **Enumerate, do not sample.** Every item this client can restore with, at every level
    /// of the skill, gets a bonus of at least one point. If a future WZ adds an item small
    /// enough that a low level rounds away to nothing, this names it rather than letting it
    /// be discovered on screen.
    #[test]
    fn no_real_item_at_any_level_gets_a_bonus_that_rounds_to_nothing() {
        let Some(c) = real_consumables() else { return };
        let Some(_) = real_skills() else { return };

        let mut checked = 0usize;
        let mut dead: Vec<String> = Vec::new();
        // A large maximum so a percentage restore is a large number too; the point of this
        // test is the smallest *flat* amounts, which do not depend on the maximum.
        const MAX_POOL: u32 = 3000;
        for (item, r) in every_restoring_item(&c) {
            if r.is_nothing() {
                continue;
            }
            for level in 1..=MAX_LEVEL {
                let full = restored(
                    &r,
                    MAX_POOL,
                    MAX_POOL,
                    Learned { improved_hp_recovery: level, improved_mp_recovery: level },
                );
                if full.hp_base > 0 {
                    checked += 1;
                    if full.hp == full.hp_base {
                        dead.push(format!("item {item} HP {} at level {level}", full.hp_base));
                    }
                }
                if full.mp_base > 0 {
                    checked += 1;
                    if full.mp == full.mp_base {
                        dead.push(format!("item {item} MP {} at level {level}", full.mp_base));
                    }
                }
            }
        }
        // The instrument speaks before its silence is believed.
        assert!(checked > 100, "only {checked} (item, level) pairs were examined");
        assert!(dead.is_empty(), "the bonus rounds away to nothing for: {dead:?}");
    }

    /// **No item in this client carries both a flat and a percentage term on the same pool**,
    /// which is why "bonus the total" and "bonus each term" are indistinguishable here and
    /// why that choice is labelled **[I]** in the module header rather than **[L]**.
    ///
    /// If this ever fails, the module header's [I] has become measurable and should be
    /// re-decided rather than re-asserted.
    #[test]
    fn nothing_in_this_client_can_tell_the_two_bonus_orders_apart() {
        let Some(c) = real_consumables() else { return };
        let mixed: Vec<u32> = every_restoring_item(&c)
            .into_iter()
            .filter(|(_, r)| (r.hp > 0 && r.hp_percent > 0) || (r.mp > 0 && r.mp_percent > 0))
            .map(|(id, _)| id)
            .collect();
        assert!(mixed.is_empty(), "these carry a flat AND a percent on one pool: {mixed:?}");
    }

    /// Seven of this client's consumables restore **both** pools, so the pool guard is not
    /// hypothetical. Elixir `2000005` is the named one.
    #[test]
    fn seven_real_items_restore_both_pools_so_the_pool_guard_matters() {
        let Some(c) = real_consumables() else { return };
        let both: Vec<u32> = every_restoring_item(&c)
            .into_iter()
            .filter(|(_, r)| {
                (r.hp > 0 || r.hp_percent > 0) && (r.mp > 0 || r.mp_percent > 0)
            })
            .map(|(id, _)| id)
            .collect();
        assert!(both.contains(&2_000_005), "Elixir restores both pools: {both:?}");
        assert_eq!(both.len(), 7, "{both:?}");

        // And a Magician drinking one gets the MP half bonused and the HP half not.
        let elixir = c.get(2_000_005).unwrap();
        let r = restored(&elixir, 1000, 1000, Learned { improved_mp_recovery: 15, ..Learned::none() });
        assert_eq!(r.hp, 1000, "hpR 100 of 1000, unbonused");
        assert_eq!(r.mp, 1200, "mpR 100 of 1000, +20% - and the cap will take it back to 1000");
        assert_eq!(r.capped(0, 1000, 0, 1000), (1000, 1000), "a full restore was already full");
    }
}
