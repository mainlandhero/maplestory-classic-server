//! **All 24 first-job skills**: what kind each one is, what weapon it needs, and what this
//! server has to *do* about it.
//!
//! The owner, 2026-08-28: *"I need all 1st job skills of all branches to have their damage
//! calculation ready and their skills available to test next session."*
//!
//! [`crate::skilltable`] answers *"may this job learn it, and what does level `n` cost"*.
//! This module answers the question after that one: **when a cast of skill `N` arrives on the
//! socket, what is the server obliged to do?** Deduct MP, deduct HP, take an arrow, set a
//! temporary stat, or nothing at all — and, for seven of the 24, **nobody knows yet**, which
//! is written down here as [`StatDuty::PassiveUnknown`] rather than guessed.
//!
//! No client run and no Ghidra pass was spent on this file. Every number is read out of
//! `gm-handbook/skills.txt`, which `tools/dump_skills.py` generates from the client's own
//! `Skill.wz` and `String.wz`. Labels are the project's: **[L]** read off this client's own
//! data, **[D]** derived from two or more [L] facts, **[I]** inferred.
//!
//! # There are 24, and there is no Pirate branch
//!
//! `world::jobs::FIRST_JOBS` is 100/200/300/400. The archive has exactly six skills under
//! each of those four books and nothing under a 5xx book. **[L]**
//!
//! # The classification rule, and the column that decides it
//!
//! **`damage` or `mad` decides "attack". `mpCon` decides "buff". Everything else is passive.**
//! [`classify`] is that rule and [`ColumnEvidence`] is its input.
//!
//! The obvious candidate — the `type` column — **cannot** be the decider, and finding that out
//! cost one census:
//!
//! ```text
//!   type present   71 skills          type EMPTY   105 skills
//!                                       of which 36 carry `damage`
//!                                       and       10 carry `mad`
//! ```
//!
//! **[L], archive-wide.** One of our own 24 has an empty `type` cell: **Power Knockback
//! (3001003)**. A classifier keyed on `type` would have returned "unknown" for a skill whose
//! `damage` column runs 105..180 and whose tooltip says *"Damage 105%"*. This is
//! `CLAUDE.md`'s *enumerate before you filter*: the census over all 176 skills is what broke
//! the assumption, not a look at the six rows the question was about.
//!
//! The two columns the rule *does* use were checked the same way:
//!
//! | claim | census | counterexamples |
//! |---|---|---|
//! | no skill carries both `damage` and `mad` | all 176 | **0** |
//! | every `type 10` (buff) carries `mpCon` | 20 skills | **0** |
//! | no `type 50` (passive) carries `mpCon` | 23 skills | **0** |
//! | no `psd 1` skill carries `mpCon` | 31 skills | **0** |
//! | every `processtype 113` skill has no `time` at any level | 16 skills | **0** |
//!
//! All **[L]**. The last one is the toggle correlation `research/magician-first-job.md` §4
//! already recorded; it is repeated here because [`StatDuty::Toggle`] rests on it and it now
//! covers a second branch's skill (Dark Sight) as well as Magic Guard.
//!
//! ## Where two columns disagree, which is a finding rather than a nuisance
//!
//! **`type 50` and `psd 1` are not the same set.** Among the 24:
//!
//! * **nine** skills are `type 50`;
//! * **seven** of those carry `psd = 1`;
//! * **Improved HP Recovery (1000000)** and **Improved MP Recovery (2000000)** are `type 50`
//!   with **no `psd`**. **[L]**
//!
//! Archive-wide exactly four skills are `type 50` without `psd`, and they are those two plus
//! both copies of MP Eater (2200000, 2300000). **[L]** Every one of the four is a *recovery
//! over time* skill rather than a flat stat bonus, so the split is not noise — but what `psd`
//! actually switches on in the client has **not** been decoded here, and no Ghidra pass was
//! made. Treating `psd` as "passive" and `type 50` as "passive" gives the same answer for 20
//! of the 24 and different answers for those two; [`classify`] uses neither, so it is
//! unaffected, and [`ColumnEvidence::type_column_agrees`] reports the disagreement rather than
//! hiding it.
//!
//! It runs the other way too, outside the 24: **`psd 1` co-occurs with `damage` on two
//! second-job skills** (both Mortal Blows, 3110000 and 3210000). **[L]** So "`psd` implies the
//! skill does nothing on a cast" is false in general, and is only asserted here for these 24.
//!
//! # The weapon gate is real, and it is the reason two branches need a shopping list
//!
//! `Skill.wz`'s `weapon`..`weapon4` columns carry the client's own 30..=47 weapon numbering —
//! the same numbering [`crate::damage::WeaponClass`] models, and `research/damage-formula.md`
//! §1.3 states it as `itemId / 10000 - 100`. **[L]** Verified against the file rather than
//! remembered:
//!
//! | skill | `weapon` cells | class | in plain words |
//! |---|---|---|---|
//! | Arrow Blow, Double Shot, Power Knockback | `45`, `46` | Bow, Crossbow | **a bow (`145xxxx`) or a crossbow (`146xxxx`)** |
//! | Double Stab | `33` | Dagger | **a dagger (`133xxxx`)** |
//! | Lucky Seven | `47` | Claw | **a claw (`147xxxx`)** |
//! | the other 19 | *(empty)* | — | ungated |
//!
//! **[L] for all four rows**, and the negative is [L] as well rather than "I did not see one":
//! the loader here reads all four `weapon` columns of every row of all 24 skills, and 19 of
//! the 24 have all four empty at every level. The six Magician skills carry no weapon column,
//! and **no skill anywhere in the archive gates on 37/38/39** (wand, staff, bare hands) — the
//! codes that appear are `30 31 32 33 40 41 42 43 44 45 46 47` only. **[L]**
//!
//! So the Warrior's two attacks and the Rogue's Disorder are ungated, and **Power Strike will
//! fire with bare hands**, while the Archer's three attacks and both of the Rogue's weapon
//! attacks will not. The client owns that refusal — see *What is not established* — so the
//! shopping list matters for a test run whether or not the server ever checks it.
//!
//! # Every number here carries its unit
//!
//! `CLAUDE.md`'s *"the unit, not the arithmetic"*, and [`crate::skilltable`]'s module header
//! documents four of these already. Repeated because [`CastNumbers`] is a second reader of the
//! same file and a second reader is a second chance to get a unit wrong:
//!
//! | column | unit | trap |
//! |---|---|---|
//! | `time`, `cooltime` | **SECONDS** | the wire wants milliseconds. **This file does not convert.** |
//! | `damage` | **PERCENT**, and **per hit** | Double Stab's `attackCount` is 2, so `80` is 2 x 80% |
//! | `mad` | **PERCENT**, and **per hit** | Magic Claw's `attackCount` is 2, so `45` is 2 x 45% |
//! | `mpCon`, `hpCon` | **flat points** | Slash Blast is the only one of the 24 with an `hpCon` |
//! | `attackCount` | hits per cast | |
//! | `mobCount` | targets per cast | |
//! | `bulletCount` | **projectiles fired**, which is *not* the number consumed | Avenger fires 1 and consumes 4. **[L]** |
//! | `bulletConsume` | **items taken from the bullet slot per cast** | absent is not zero — see [`BulletDuty`] |
//! | `x`, `y`, `z` | **skill-specific, and the WZ never says which** | carried by [`crate::skilltable`], deliberately not interpreted here |
//!
//! # An empty cell is not a zero, and `0` is a value
//!
//! Every per-level number in [`CastNumbers`] is an [`Option`], matching
//! [`crate::skilltable::SkillLevel`]. Four places in these 24 where the difference is
//! load-bearing:
//!
//! * **Magic Guard and Dark Sight carry no `time` at any level.** Toggles, not zero-second
//!   buffs.
//! * **Lucky Seven carries `bulletCount 2` and no `bulletConsume`.** That is not "consumes
//!   zero" and it is not "consumes two" — see [`BulletDuty::ProjectilesNoConsumeColumn`].
//! * **Dark Sight's `speed` is present at levels 1..=19 and absent at level 20**, where the
//!   tooltip says *"regular movement speed"*. A `u32` defaulting to `0` would send
//!   "speed 0", which is what collapsed every mob's hit box the last time this rule was
//!   broken.
//! * **Power Strike has no `hpCon`; Slash Blast has one.** Two neighbouring skills in the same
//!   book, and reading absent as zero would make them look identical.
//!
//! # Do the passives need a packet at all?
//!
//! **Not settled, and the archived logs cannot settle it — but the question is now much
//! smaller than it was.**
//!
//! ## What the logs can and cannot say
//!
//! All 167 files in `previous-runs/` were grepped for the skill-change packet. The instrument
//! speaks: `0x0081 ChangeSkillRecordResult` appears in **six** runs, **three** raises in the
//! newest of them. **Every raised id is 1000, 1001 or 1002** — the beginner book, none of
//! which is `type 50` and none of which carries `psd`. **[L]**
//!
//! So no archived run has ever had a passive at level >= 1, and "the logs show nothing" here
//! is a property of what was tested, not of the client. That is `CLAUDE.md`'s *"nothing new
//! arrived is a different claim from this thing did not arrive"*, and only the second claim
//! is worth making: **this thing has never arrived, because nobody has ever sent it.**
//!
//! ## Seven of the nine cannot be the server's job, and that is [D]
//!
//! What each passive changes, and who owns that number today:
//!
//! | passive | changes | who authors it |
//! |---|---|---|
//! | Precise Strikes, Critical Shot | `accX`, `crtX`, `crdX` — accuracy, critical rate, critical damage | **the client.** `net::combat::AttackHit::damage` is documented *"The client computed this"*, and `AttackHit::flag_b` is the critical flag arriving **inbound** on `0x00DF`/`0x00E1`. **[L]** |
//! | Nimble Body | `accX`, `evaX` | same path |
//! | The Eye of Amazon, Keen Eyes | `range`, in client pixels | the client picks its own targets and sends them |
//! | Improved HP/MP Recovery | regen per tick, plus a percent bonus to recovery **from items** | the tick period is **not in the WZ at all** — it is only in the tooltip prose. **[L]** |
//! | **Max HP Increase, Max MP Increase** | `mhpR`, `mmpR` — **percent** of the maximum | **the client, on top of the server's number** - measured 2026-09-06, Cobalt at 358 base drawing 447. So the server never folds it into the stat packet, and instead raises every ceiling it caps HP/MP against (`session::pools`) or it thinks a 358/447 character is full |
//!
//! `crates/world/src/session/combat.rs` applies `target.total_damage()` — the client's number —
//! and never computes an outgoing hit, a miss or a critical. **[L]** So for seven of the nine
//! there is **no server-side quantity to apply the bonus to**. Either the client applies them
//! or nothing does; a server packet has nowhere to land. **[D]** from those two [L] facts.
//!
//! That does *not* prove the client reads `Skill.wz` for them. It proves the server cannot be
//! the answer, which is a different and weaker statement — and it is deliberately weaker,
//! because `research/magician-first-job.md` §3 already refused to promote *"the property names
//! exist in the client image as UTF-16 literals"* into a read, and so does this file.
//!
//! ## The one sentence the owner can act on
//!
//! > **Run `!learn 1000001 15` (Max HP Increase, "Max HP +25%") on a job-100 character, log
//! > in, and compare the Max HP drawn in the stat window against the `max_hp` this server put
//! > in its own stat packet in `world.log`: identical means the client does *not* apply
//! > passives and the server must fold every `psd` percentage in itself, and 1.25x means the
//! > client applies them locally and a server that also applied them would double-count.**
//!
//! This is `research/magician-first-job.md` §8's experiment **A**, unchanged, and it is
//! reproduced rather than cited because it is now the *only* remaining half of the question:
//! its two outcomes decide Max HP Increase and Max MP Increase, and the [D] above has already
//! decided the other seven as far as the server is concerned. Both outcomes are actionable and
//! they point opposite ways.
//!
//! # What this file does NOT establish
//!
//! * **Which `TemporaryStat` bit Iron Body, Focus and Dark Sight use.** `net::buff` knows
//!   `CTS_SPEED`, `CTS_MAGIC_GUARD` and the Magic Armor pair; nothing here adds a bit, and
//!   [`StatDuty::Timed`] deliberately carries no bit number.
//! * **Iron Body's unit problem.** Magic Armor's `indiePdd` is **flat** (+40 W.Def) and Iron
//!   Body's `indiePddR` is a **percent** (+5% W.Def). `net::buff::CTS_WEAPON_DEFENCE` is one
//!   bit, itself only **[D]** with a named blind spot. Whether that bit takes a flat value or
//!   a percentage — or whether the percentage needs a different bit entirely — is **unknown**,
//!   and it is exactly the shape of the three unit bugs `CLAUDE.md` records.
//! * **Whether the server should enforce the weapon gate.** The client refuses the cast
//!   itself (nobody here has watched it do so); a server that also refused would be a second
//!   refusal, and `CLAUDE.md`'s *always answer* rule makes a wrong refusal expensive.
//! * **How many damage lines Lucky Seven produces.** `attackCount 1`, `bulletCount 2`, and a
//!   tooltip that says only *"Damage 60%"*. Three sources, none of which settles it.
//! * **Whether Disorder deals damage.** It carries `damage 100` at every one of its 20 levels
//!   — a *constant*, unlike every other attack in the 24 — and its tooltip mentions no damage
//!   at all, only the debuff. `100` may be the neutral multiplier or it may be inert.
//! * **Power Knockback's `time 1500`.** Constant at all 15 levels. The file's header says
//!   `time` is SECONDS, which would be 25 minutes for a knockback, and no tooltip corroborates
//!   any duration. The unit is **unresolved** and [`CastNumbers::time_seconds`] carries it raw.
//! * **What `type`, `processtype`, `psd` and `prop` mean.** Emitted raw. The only correlation
//!   strong enough to act on is `processtype 113` <-> toggle, 16 cases, 0 counterexamples,
//!   and it is labelled **[D]**.
//! * **Anything about second job.** The 24 ids are hard-coded; nothing here generalises.

use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr;

use crate::damage::WeaponClass;

// ---------------------------------------------------------------------------------------
// What kind of skill it is
// ---------------------------------------------------------------------------------------

/// What a first-job skill *is*, derived from the columns rather than from memory.
///
/// See the module header for the census behind each arm. [`classify`] is the rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    /// Carries `damage` or `mad`. Ten of the 24.
    Attack,
    /// Carries neither, but carries `mpCon`, so it is cast. Five of the 24.
    Buff,
    /// Carries neither and costs nothing to have. Nine of the 24.
    Passive,
}

impl Kind {
    /// What the `type` column *would* say, for the 71 skills that have one.
    ///
    /// `None` for an empty cell — **Power Knockback's is empty** — and for a `type` outside the
    /// four values these 24 use. This exists so the disagreement between `type` and the rule
    /// can be asserted mechanically; it is **not** the classifier.
    pub fn from_wz_type(wz_type: u32) -> Option<Kind> {
        Some(match wz_type {
            // 1 is melee, 2 is a projectile. Both are attacks and the split is recorded in
            // `FirstJobSkill::wz_type` rather than folded away here.
            1 | 2 => Kind::Attack,
            10 => Kind::Buff,
            50 => Kind::Passive,
            _ => return None,
        })
    }
}

/// The five columns [`classify`] reads, plus the three it only reports on.
///
/// Built from the file by [`SkillCombat::evidence`], so a test can re-derive every
/// classification in [`FIRST_JOB_SKILLS`] instead of trusting the literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnEvidence {
    /// `damage` present on at least one level row. **Percent, per hit.**
    pub has_damage: bool,
    /// `mad` present on at least one level row. **Percent, per hit.**
    pub has_mad: bool,
    /// `mpCon` present on at least one level row. **Flat MP points.**
    pub has_mp_con: bool,
    /// The `type` column. `None` means the cell is **empty**, which is the case for 105 of the
    /// 176 skills in the archive and for one of these 24.
    pub wz_type: Option<u32>,
    /// `psd = 1`. Seven of the 24; see the module header for why it is not the same set as
    /// `type 50`.
    pub psd: bool,
    /// The `processtype` column. `None` means empty. `113` is the toggle correlation.
    ///
    /// **SIGNED, and that is measured rather than defensive.** Parsed as `u32` this reader
    /// dropped **330 rows across 12 skills** — every one of them a `processtype` of **`-1`**.
    /// The whole column was then swept: `-1` is the only negative value it takes, and `type`,
    /// `psd`, `weapon`..`weapon4`, `mpCon`, `hpCon`, `damage`, `mad`, `attackCount`,
    /// `mobCount`, `bulletCount`, `bulletConsume`, `time` and `cooltime` carry **no** negatives
    /// and **no** non-numeric cells anywhere in the archive. **[L]**
    ///
    /// [`crate::skilltable`] does not read this column at all, so it never met the problem —
    /// which is why the drop count is a finding rather than a bug this file introduced. It is
    /// the same shape as that module's `x` being signed because Booster carries `-1`/`-2`.
    pub processtype: Option<i32>,
}

impl ColumnEvidence {
    /// **The rule, as a method**, so a caller cannot accidentally use a different one.
    pub fn kind(&self) -> Kind {
        classify(self)
    }

    /// Whether the `type` column agrees with [`classify`]. `None` when the cell is empty and
    /// so has no opinion.
    ///
    /// A `Some(false)` from any of the 24 is a finding, not a bug in this function — it means
    /// two of the client's own columns describe the same skill differently, and the test that
    /// calls this says which.
    pub fn type_column_agrees(&self) -> Option<bool> {
        Some(Kind::from_wz_type(self.wz_type?)? == self.kind())
    }

    /// Whether `psd` agrees with [`classify`]: `psd = 1` should mean [`Kind::Passive`].
    ///
    /// **Only asserted for these 24.** Archive-wide it is false — both Mortal Blows carry
    /// `psd 1` *and* `damage`. **[L]**
    pub fn psd_agrees(&self) -> bool {
        !self.psd || self.kind() == Kind::Passive
    }
}

/// **The classification rule.** `damage` or `mad` -> attack; else `mpCon` -> buff; else passive.
///
/// The deciding column is named in the return, not inferred by the caller:
///
/// * `damage` / `mad` decide **attack**, and they are the only columns present on all ten
///   attacks among the 24. `type` is empty on one of them (Power Knockback) and archive-wide
///   it is empty on 36 of the 46 skills that carry `damage`. **[L]**
/// * `mpCon` decides **buff** among what is left, because `mpCon` present <-> the skill is
///   cast: all 20 `type 10` skills carry one and none of the 23 `type 50` skills does, with
///   **zero** counterexamples in either direction. **[L]**
/// * Everything else is **passive**, as the residual.
///
/// The residual is why this function is only claimed over the 24: archive-wide there are 13
/// skills with no `damage`, no `mad`, no `mpCon` and no `type`, and this would call all of
/// them passive without evidence. Nothing here looks at them.
pub fn classify(e: &ColumnEvidence) -> Kind {
    if e.has_damage || e.has_mad {
        Kind::Attack
    } else if e.has_mp_con {
        Kind::Buff
    } else {
        Kind::Passive
    }
}

// ---------------------------------------------------------------------------------------
// What the server owes on a cast
// ---------------------------------------------------------------------------------------

/// Which damage path a cast uses — **not** an instruction to compute a number.
///
/// `research/mob-combat.md` and `net::combat::AttackHit::damage` (*"The client computed
/// this"*) establish that the client authors the damage and the server applies it. **[L]**
/// `crate::damage` and `crate::magic` are **validators**, and both of their module headers say
/// so in the same words: *log only, never refuse*. So this enum says which validator applies,
/// and a caller that reads it as "generate a number here" has the direction backwards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageDuty {
    /// No `damage` and no `mad` column at any level.
    None,
    /// `damage`, a **percent, per hit**. Validate with [`crate::damage`].
    PhysicalPercent,
    /// `mad`, a **percent, per hit**. Validate with [`crate::magic`].
    MagicPercent,
}

/// What a cast takes out of the bullet slot.
///
/// **Absent is not zero, and `bulletCount` is not `bulletConsume`.** The archive proves they
/// are independent numbers rather than a default: Avenger (4111004) has `bulletCount 1` and
/// `bulletConsume 4`; Strafe and Holy Arrow have `bulletCount 3` and no `bulletConsume` at
/// all. **[L]** So there is no rule "consume equals count" to fall back on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulletDuty {
    /// Neither column. Nineteen of the 24.
    None,
    /// `bulletConsume` present: take exactly this many per cast. Arrow Blow `1`, Double Shot
    /// `2`. **[L]**
    Consume(u32),
    /// **UNKNOWN.** `bulletCount` present with **no** `bulletConsume`: the skill fires this
    /// many projectiles and the archive does not say how many items that costs.
    ///
    /// Two of the 24 are here, for different reasons:
    ///
    /// * **Energy Bolt** (`bulletCount 1`), which has no weapon gate and is a spell — there is
    ///   probably no item at all, but "probably" is the whole point of this arm.
    /// * **Lucky Seven** (`bulletCount 2`), which is gated on a Claw and in this game is thrown
    ///   with stars. A server that guessed `2` and a server that guessed `1` would both be
    ///   making up a number the client's data does not carry.
    ProjectilesNoConsumeColumn(u32),
}

/// What a cast does to somebody's stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatDuty {
    /// Nothing. The attack skills with no rider.
    None,
    /// A **timed** grant on the caster: `time` SECONDS, `processtype 6`. Iron Body, Magic
    /// Armor, Focus.
    ///
    /// **Carries no CTS bit on purpose.** `net::buff` owns the mask and only three bits in it
    /// have ever been written down. Putting a number here would be inventing one.
    Timed,
    /// A **toggle** on the caster: `processtype 113`, and **no `time` at any level**. Magic
    /// Guard, Dark Sight.
    ///
    /// **[D]**, 16 skills in the archive share the pattern and every one of their descriptions
    /// says the effect is switched off by casting it again; 0 counterexamples. The off-path is
    /// `0x013F`, which `world::session::buff` already handles.
    Toggle,
    /// A **debuff on the struck mob**, not on the caster: Disorder's `time` 10..30 SECONDS,
    /// which its own tooltip states as *"for 10 sec"*. **[L]**
    MobDebuff,
    /// **UNKNOWN.** `type 50` and/or `psd`. Whether the server sends anything at all is the
    /// open question in the module header, and for seven of the nine the answer is [D]
    /// "it cannot be the server".
    PassiveUnknown,
}

/// **Everything the server owes on one cast — a set, not a choice.**
///
/// It is deliberately not a single enum. `CLAUDE.md`: *"A test that checks one of several
/// effects gives false confidence about the rest"* — the Heena quest paid out twice because
/// the payout and the item grant were gated separately and one was missed. Arrow Blow needs
/// **three** things (MP, an arrow, the damage path) and an enum would have made a caller pick
/// one of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Obligation {
    /// Deduct `mpCon` **flat MP points** for the cast level. True for all 15 castable skills
    /// among the 24 and false for all nine passives — `mpCon` present <-> castable, 0
    /// counterexamples archive-wide. **[L]**
    pub deduct_mp: bool,
    /// Deduct `hpCon` **flat HP points**. **Slash Blast is the only one of the 24**, 3..8 HP.
    /// **[L]**
    pub deduct_hp: bool,
    pub damage: DamageDuty,
    pub bullets: BulletDuty,
    pub stat: StatDuty,
}

impl Obligation {
    /// Whether any part of this obligation is an admitted unknown.
    ///
    /// True for the nine passives ([`StatDuty::PassiveUnknown`]) and for the two skills whose
    /// bullet cost is not in the data ([`BulletDuty::ProjectilesNoConsumeColumn`]).
    pub fn has_unknown(&self) -> bool {
        matches!(self.stat, StatDuty::PassiveUnknown)
            || matches!(self.bullets, BulletDuty::ProjectilesNoConsumeColumn(_))
    }

    /// Whether a cast of this skill needs a `mpCon` lookup, i.e. whether the player pays for it.
    pub fn is_castable(&self) -> bool {
        self.deduct_mp || self.deduct_hp
    }
}

// ---------------------------------------------------------------------------------------
// The 24
// ---------------------------------------------------------------------------------------

/// One first-job skill's constant facts. Per-level numbers live in [`CastNumbers`].
///
/// Every field here is constant across that skill's level rows, and the tests assert that
/// against the generated file rather than taking it on trust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirstJobSkill {
    pub id: u32,
    /// The skill book: `100`, `200`, `300` or `400`.
    pub job: u16,
    pub name: &'static str,
    /// The highest level the client's own data describes.
    pub max_level: u32,
    /// The `type` column. **`None` means the cell is empty** — Power Knockback.
    pub wz_type: Option<u32>,
    /// The `processtype` column. `None` means empty.
    pub processtype: Option<i32>,
    /// `psd = 1`.
    pub psd: bool,
    /// Derived by [`classify`] from the columns; the tests re-derive it from the file.
    pub kind: Kind,
    /// The classes the client's `weapon`..`weapon4` columns name. **Empty means ungated**, and
    /// that is measured — all four columns are read on every row of all 24 skills.
    pub weapon_gate: &'static [WeaponClass],
    /// The gate in plain words, including the item-id prefix the owner would type at `!item`.
    /// Every class in [`FirstJobSkill::weapon_gate`] is named in here, and a test checks it.
    pub weapon_note: &'static str,
    pub obligation: Obligation,
    /// What is unresolved about this specific skill, in one sentence. Empty when nothing is.
    pub note: &'static str,
}

const UNGATED: &[WeaponClass] = &[];
const BOW_OR_CROSSBOW: &[WeaponClass] = &[WeaponClass::Bow, WeaponClass::Crossbow];
const DAGGER: &[WeaponClass] = &[WeaponClass::Dagger];
const CLAW: &[WeaponClass] = &[WeaponClass::Claw];

const NO_GATE: &str = "ungated: no weapon column at any level";
const BOW_WORDS: &str = "a Bow (145xxxx) or a Crossbow (146xxxx), with arrows in the bullet slot";
const DAGGER_WORDS: &str = "a Dagger (133xxxx)";
const CLAW_WORDS: &str = "a Claw (147xxxx)";

/// A passive's obligation: nothing known, and the module header says why that is a result.
const PASSIVE: Obligation = Obligation {
    deduct_mp: false,
    deduct_hp: false,
    damage: DamageDuty::None,
    bullets: BulletDuty::None,
    stat: StatDuty::PassiveUnknown,
};

/// **All 24, in job order.** The order is `world::jobs::FIRST_JOBS`' — Warrior, Magician,
/// Archer, Rogue — and within a book, ascending id, which is the order `Skill.wz` lists them.
pub const FIRST_JOB_SKILLS: [FirstJobSkill; 24] = [
    // ---- 100 Warrior -----------------------------------------------------------------
    FirstJobSkill {
        id: 1_000_000,
        job: 100,
        name: "Improved HP Recovery",
        max_level: 15,
        wz_type: Some(50),
        processtype: None,
        psd: false,
        kind: Kind::Passive,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: PASSIVE,
        note: "type 50 with NO psd - one of only four in the archive. `y` is a percent bonus \
               to HP recovered from items; the 10-second tick period is in the tooltip prose \
               and in no numeric field of the WZ.",
    },
    FirstJobSkill {
        id: 1_000_001,
        job: 100,
        name: "Max HP Increase",
        max_level: 15,
        wz_type: Some(50),
        processtype: None,
        psd: true,
        kind: Kind::Passive,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: PASSIVE,
        note: "`mhpR` is a PERCENT of maximum HP (10..25), never an amount. One of only two \
               passives among the 24 that changes a number this server authors - it is the \
               probe in the module header's experiment.",
    },
    FirstJobSkill {
        id: 1_000_002,
        job: 100,
        name: "Precise Strikes",
        max_level: 15,
        wz_type: Some(50),
        processtype: None,
        psd: true,
        kind: Kind::Passive,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: PASSIVE,
        note: "`accX` 5..20 accuracy points and `crtX` 1..5 percent critical rate. The critical \
               flag arrives INBOUND from the client, so the server has nothing to apply this to.",
    },
    FirstJobSkill {
        id: 1_001_000,
        job: 100,
        name: "Iron Body",
        max_level: 20,
        wz_type: Some(10),
        processtype: Some(6),
        psd: false,
        kind: Kind::Buff,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: Obligation {
            deduct_mp: true,
            deduct_hp: false,
            damage: DamageDuty::None,
            bullets: BulletDuty::None,
            stat: StatDuty::Timed,
        },
        note: "UNIT TRAP: `indiePddR` is a PERCENT weapon defence (5..25), where Magic Armor's \
               `indiePdd` is FLAT (+40..120). `net::buff` has one weapon-defence bit and it is \
               only [D]. `mpCon` is a flat 15 at every level.",
    },
    FirstJobSkill {
        id: 1_001_001,
        job: 100,
        name: "Power Strike",
        max_level: 20,
        wz_type: Some(1),
        processtype: Some(108),
        psd: false,
        kind: Kind::Attack,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: Obligation {
            deduct_mp: true,
            deduct_hp: false,
            damage: DamageDuty::PhysicalPercent,
            bullets: BulletDuty::None,
            stat: StatDuty::None,
        },
        note: "Ungated, so it fires with bare hands - the cheapest attack in the whole set to \
               put on screen. `areaAttack 1`, one hit, one target.",
    },
    FirstJobSkill {
        id: 1_001_002,
        job: 100,
        name: "Slash Blast",
        max_level: 20,
        wz_type: Some(1),
        processtype: Some(108),
        psd: false,
        kind: Kind::Attack,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: Obligation {
            deduct_mp: true,
            deduct_hp: true,
            damage: DamageDuty::PhysicalPercent,
            bullets: BulletDuty::None,
            stat: StatDuty::None,
        },
        note: "THE ONLY ONE OF THE 24 WITH AN `hpCon` (3..8 flat HP). Four targets a cast. \
               Power Strike has no `hpCon` at all, and absent is not zero.",
    },
    // ---- 200 Magician ----------------------------------------------------------------
    FirstJobSkill {
        id: 2_000_000,
        job: 200,
        name: "Improved MP Recovery",
        max_level: 15,
        wz_type: Some(50),
        processtype: None,
        psd: false,
        kind: Kind::Passive,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: PASSIVE,
        note: "type 50 with NO psd, the Magician twin of 1000000. `x` is 1 percent of max MP \
               per tick at every level; `y` is the item bonus.",
    },
    FirstJobSkill {
        id: 2_000_001,
        job: 200,
        name: "Max MP Increase",
        max_level: 15,
        wz_type: Some(50),
        processtype: None,
        psd: true,
        kind: Kind::Passive,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: PASSIVE,
        note: "`mmpR` is a PERCENT of maximum MP (10..25). The second of the two passives that \
               changes a number this server authors.",
    },
    FirstJobSkill {
        id: 2_001_000,
        job: 200,
        name: "Magic Guard",
        max_level: 15,
        wz_type: Some(10),
        processtype: Some(113),
        psd: false,
        kind: Kind::Buff,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: Obligation {
            deduct_mp: true,
            deduct_hp: false,
            damage: DamageDuty::None,
            bullets: BulletDuty::None,
            stat: StatDuty::Toggle,
        },
        note: "ALREADY MODELLED in `net::buff::buff_level`, CTS bit 97 [L]. No `time` at any of \
               its 15 levels - a toggle, not a zero-second buff.",
    },
    FirstJobSkill {
        id: 2_001_001,
        job: 200,
        name: "Magic Armor",
        max_level: 20,
        wz_type: Some(10),
        processtype: Some(6),
        psd: false,
        kind: Kind::Buff,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: Obligation {
            deduct_mp: true,
            deduct_hp: false,
            damage: DamageDuty::None,
            bullets: BulletDuty::None,
            stat: StatDuty::Timed,
        },
        note: "ALREADY MODELLED in `net::buff::buff_level`. Two stats, `indiePdd` and \
               `indieMdd`, both FLAT and equal at every level; the CTS pair 86/87 is [D] with a \
               named blind spot.",
    },
    FirstJobSkill {
        id: 2_001_002,
        job: 200,
        name: "Energy Bolt",
        max_level: 20,
        wz_type: Some(2),
        processtype: Some(116),
        psd: false,
        kind: Kind::Attack,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: Obligation {
            deduct_mp: true,
            deduct_hp: false,
            damage: DamageDuty::MagicPercent,
            bullets: BulletDuty::ProjectilesNoConsumeColumn(1),
            stat: StatDuty::None,
        },
        note: "`bulletCount 1` with no `bulletConsume` and no weapon gate: a spell, so probably \
               no item is taken - but `probably` is why the bullet duty is the unknown arm. \
               `range 350` comes from the skill's `common` node, not from any level.",
    },
    FirstJobSkill {
        id: 2_001_003,
        job: 200,
        name: "Magic Claw",
        max_level: 20,
        wz_type: Some(1),
        processtype: Some(2),
        psd: false,
        kind: Kind::Attack,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: Obligation {
            deduct_mp: true,
            deduct_hp: false,
            damage: DamageDuty::MagicPercent,
            bullets: BulletDuty::None,
            stat: StatDuty::None,
        },
        note: "`attackCount 2`, so `mad` 45..65 is PER HIT and a cast is twice that. \
               `magicDamage 1` with `type 1` - the only skill in the 24 that carries it.",
    },
    // ---- 300 Archer ------------------------------------------------------------------
    FirstJobSkill {
        id: 3_000_000,
        job: 300,
        name: "Critical Shot",
        max_level: 15,
        wz_type: Some(50),
        processtype: None,
        psd: true,
        kind: Kind::Passive,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: PASSIVE,
        note: "`crtX` 5..20 percent critical rate and `crdX` 1..15 percent critical damage. \
               The critical flag arrives inbound on the attack packet, so the server has no \
               roll to modify.",
    },
    FirstJobSkill {
        id: 3_000_001,
        job: 300,
        name: "The Eye of Amazon",
        max_level: 15,
        wz_type: Some(50),
        processtype: None,
        psd: true,
        kind: Kind::Passive,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: PASSIVE,
        note: "`range` +50..120 CLIENT PIXELS, and the tooltip scopes it to bows and crossbows \
               although the skill itself carries no weapon column. The client picks its own \
               targets, so this is its number to use.",
    },
    FirstJobSkill {
        id: 3_001_000,
        job: 300,
        name: "Focus",
        max_level: 20,
        wz_type: Some(10),
        processtype: Some(6),
        psd: false,
        kind: Kind::Buff,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: Obligation {
            deduct_mp: true,
            deduct_hp: false,
            damage: DamageDuty::None,
            bullets: BulletDuty::None,
            stat: StatDuty::Timed,
        },
        note: "TWO stats - `indieAcc` 1..20 and `indieEva` 5..25, both FLAT points - so it has \
               the same shape as Magic Armor and needs `BuffLevel::all_granted_by`, not \
               `granted_by`. Neither CTS bit is known. `time` 70..300 SECONDS.",
    },
    FirstJobSkill {
        id: 3_001_001,
        job: 300,
        name: "Arrow Blow",
        max_level: 20,
        wz_type: Some(2),
        processtype: Some(115),
        psd: false,
        kind: Kind::Attack,
        weapon_gate: BOW_OR_CROSSBOW,
        weapon_note: BOW_WORDS,
        obligation: Obligation {
            deduct_mp: true,
            deduct_hp: false,
            damage: DamageDuty::PhysicalPercent,
            bullets: BulletDuty::Consume(1),
            stat: StatDuty::None,
        },
        note: "`bulletConsume 1` - the only fully-specified bullet cost in the 24 alongside \
               Double Shot. Needs 2060xxx arrows for a bow or 2061xxx for a crossbow.",
    },
    FirstJobSkill {
        id: 3_001_002,
        job: 300,
        name: "Double Shot",
        max_level: 20,
        wz_type: Some(2),
        processtype: Some(115),
        psd: false,
        kind: Kind::Attack,
        weapon_gate: BOW_OR_CROSSBOW,
        weapon_note: BOW_WORDS,
        obligation: Obligation {
            deduct_mp: true,
            deduct_hp: false,
            damage: DamageDuty::PhysicalPercent,
            bullets: BulletDuty::Consume(2),
            stat: StatDuty::None,
        },
        note: "`bulletConsume 2`, `bulletCount 2`, `mobCount 2` but `attackCount 1`. The \
               tooltip says only `Damage 80%`, so how many lines a cast draws is not settled \
               by this data.",
    },
    FirstJobSkill {
        id: 3_001_003,
        job: 300,
        name: "Power Knockback",
        max_level: 15,
        wz_type: None,
        processtype: Some(108),
        psd: false,
        kind: Kind::Attack,
        weapon_gate: BOW_OR_CROSSBOW,
        weapon_note: BOW_WORDS,
        obligation: Obligation {
            deduct_mp: true,
            deduct_hp: false,
            damage: DamageDuty::PhysicalPercent,
            bullets: BulletDuty::None,
            stat: StatDuty::None,
        },
        note: "THE `type` CELL IS EMPTY - a classifier keyed on `type` returns nothing for a \
               skill whose `damage` runs 105..180. `time 1500` is constant at all 15 levels \
               and its unit is UNRESOLVED (the header says seconds, which would be 25 min). \
               `prop 100` is [I] a proc chance. Gated on a bow but carries NO bullet column, \
               unlike its two siblings. `mpCon` DESCENDS 12 -> 8 as the level rises.",
    },
    // ---- 400 Rogue -------------------------------------------------------------------
    FirstJobSkill {
        id: 4_000_000,
        job: 400,
        name: "Nimble Body",
        max_level: 15,
        wz_type: Some(50),
        processtype: None,
        psd: true,
        kind: Kind::Passive,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: PASSIVE,
        note: "`accX` and `evaX`, both 1..15 flat points and equal at every level.",
    },
    FirstJobSkill {
        id: 4_000_001,
        job: 400,
        name: "Keen Eyes",
        max_level: 15,
        wz_type: Some(50),
        processtype: None,
        psd: true,
        kind: Kind::Passive,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: PASSIVE,
        note: "`range` +50..200 CLIENT PIXELS for throwing weapons. The Rogue twin of The Eye \
               of Amazon, and it climbs further.",
    },
    FirstJobSkill {
        id: 4_001_000,
        job: 400,
        name: "Disorder",
        max_level: 20,
        wz_type: Some(1),
        processtype: Some(108),
        psd: false,
        kind: Kind::Attack,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: Obligation {
            deduct_mp: true,
            deduct_hp: false,
            damage: DamageDuty::PhysicalPercent,
            bullets: BulletDuty::None,
            stat: StatDuty::MobDebuff,
        },
        note: "`damage 100` is CONSTANT at all 20 levels - unlike every other attack here - and \
               the tooltip mentions no damage at all. Whether a cast deals damage is UNKNOWN. \
               `time` 10..30 SECONDS is the debuff on the MOB: `x` 5..25 is the enemy's attack \
               power lost and `y` 1..5 its weapon defence.",
    },
    FirstJobSkill {
        id: 4_001_001,
        job: 400,
        name: "Dark Sight",
        max_level: 20,
        wz_type: Some(10),
        processtype: Some(113),
        psd: false,
        kind: Kind::Buff,
        weapon_gate: UNGATED,
        weapon_note: NO_GATE,
        obligation: Obligation {
            deduct_mp: true,
            deduct_hp: false,
            damage: DamageDuty::None,
            bullets: BulletDuty::None,
            stat: StatDuty::Toggle,
        },
        note: "The second toggle in the 24 and the second `processtype 113` skill. `mpCon` \
               DESCENDS 50 -> 30 and `speed` DESCENDS 20 -> 2 and is then ABSENT at level 20, \
               where the tooltip says `regular movement speed` - absent, not zero.",
    },
    FirstJobSkill {
        id: 4_001_002,
        job: 400,
        name: "Double Stab",
        max_level: 20,
        wz_type: Some(1),
        processtype: Some(108),
        psd: false,
        kind: Kind::Attack,
        weapon_gate: DAGGER,
        weapon_note: DAGGER_WORDS,
        obligation: Obligation {
            deduct_mp: true,
            deduct_hp: false,
            damage: DamageDuty::PhysicalPercent,
            bullets: BulletDuty::None,
            stat: StatDuty::None,
        },
        note: "`attackCount 2`, so `damage` 80..160 is PER HIT. Gated on a Dagger only - not \
               on a Claw, so it cannot be tested with the Lucky Seven weapon.",
    },
    FirstJobSkill {
        id: 4_001_003,
        job: 400,
        name: "Lucky Seven",
        max_level: 20,
        wz_type: Some(2),
        processtype: Some(115),
        psd: false,
        kind: Kind::Attack,
        weapon_gate: CLAW,
        weapon_note: CLAW_WORDS,
        obligation: Obligation {
            deduct_mp: true,
            deduct_hp: false,
            damage: DamageDuty::PhysicalPercent,
            bullets: BulletDuty::ProjectilesNoConsumeColumn(2),
            stat: StatDuty::None,
        },
        note: "`bulletCount 2` with NO `bulletConsume`, so the star cost is UNKNOWN. \
               `crate::damage::weapon_multiplier_for` already special-cases this skill at 3.0 \
               for a Claw [L] - the one place the repo's damage table names a skill id.",
    },
];

/// The 24 ids, in the same order as [`FIRST_JOB_SKILLS`].
pub fn first_job_skill_ids() -> Vec<u32> {
    FIRST_JOB_SKILLS.iter().map(|s| s.id).collect()
}

/// One of the 24, by id. `None` for anything else, including second-job ids.
pub fn first_job_skill(id: u32) -> Option<&'static FirstJobSkill> {
    FIRST_JOB_SKILLS.iter().find(|s| s.id == id)
}

/// The six skills of one book. Empty for a job that is not `100`/`200`/`300`/`400`.
pub fn book(job: u16) -> impl Iterator<Item = &'static FirstJobSkill> {
    FIRST_JOB_SKILLS.iter().filter(move |s| s.job == job)
}

/// **What the server must do when a cast of `skill_id` arrives.**
///
/// `None` means this is not one of the 24 — which is a different answer from "nothing to do",
/// and the caller must not merge them. A second-job skill and a passive both produce no MP
/// deduction, and only one of them is a skill this module has anything to say about.
pub fn server_obligation(skill_id: u32) -> Option<Obligation> {
    first_job_skill(skill_id).map(|s| s.obligation)
}

// ---------------------------------------------------------------------------------------
// The per-level numbers, read from the same generated file
// ---------------------------------------------------------------------------------------

/// The numbers one **level** of one skill costs and carries, for the combat path.
///
/// # Why this is a second reader of a file [`crate::skilltable`] already reads
///
/// [`crate::skilltable::SkillLevel`] carries `mpCon`, `time`, `cooltime`, `mad`,
/// `attackCount` and `mobCount`. It does **not** carry `hpCon`, `damage`, `bulletCount`,
/// `bulletConsume` or the four `weapon` columns, and a caster needs all ten in one place
/// rather than a join it can get wrong.
///
/// The six shared numbers are deliberately re-read rather than referenced, and
/// `combat_table_agrees_with_skilltable_on_every_shared_column` asserts the two readers agree
/// on every level of all 24 skills. That is `CLAUDE.md`'s *count the same event in two logs*:
/// two independent readers of one file that must produce the same number, so a column shift
/// on either side is loud instead of silent.
///
/// **Every field is an [`Option`] because an empty cell means absent from the WZ node**, which
/// is not the same as zero. There is no `Default`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastNumbers {
    /// As the client numbers it: `1` is the first level, never `0`.
    pub level: u32,
    /// `mpCon` — **flat MP points**.
    pub mp_con: Option<u32>,
    /// `hpCon` — **flat HP points**. Present on exactly one of the 24 (Slash Blast, 3..8).
    pub hp_con: Option<u32>,
    /// `damage` — a **PERCENT, per hit**. Never present on the same skill as
    /// [`CastNumbers::mad_percent`]: 0 skills in the archive carry both. **[L]**
    pub damage_percent: Option<u32>,
    /// `mad` — a **PERCENT, per hit**. Magic Claw's `45` at `attackCount 2` is 2 x 45%.
    pub mad_percent: Option<u32>,
    /// `attackCount` — **hits per cast**.
    pub attack_count: Option<u32>,
    /// `mobCount` — **targets per cast**.
    pub mob_count: Option<u32>,
    /// `bulletCount` — **projectiles fired per cast**. *Not* the number of items consumed.
    pub bullet_count: Option<u32>,
    /// `bulletConsume` — **items taken from the bullet slot per cast**. Absent is not zero;
    /// see [`BulletDuty`].
    pub bullet_consume: Option<u32>,
    /// `time` — **SECONDS.** The wire wants milliseconds and **this file does not convert**;
    /// `net::buff::BuffLevel::granted_by` is the one place that multiplies.
    pub time_seconds: Option<u32>,
    /// `cooltime` — **SECONDS.** None of the 24 carries one.
    pub cooltime_seconds: Option<u32>,
    /// `indieSpeed` — a **flat** Speed grant. Haste: 10 at level 1, 30 at 20.
    ///
    /// The six `indie_*` fields are the temporary stats a buff row carries as flat, signed
    /// points; `world::session::buff` maps each onto its CTS bit. **Signed** because Rage's
    /// `indiePdd` is `-10..-40`. `None` where the column is absent from the generated file
    /// (an older `skills.txt`) or empty on the row. The `*R` percent columns - `indiePddR`,
    /// which is Iron Body - are deliberately NOT here: a percent is not a grant until
    /// something resolves it against a base, and `net::jobbuffs::iron_body_flat_pdd` is the
    /// one place that does.
    pub indie_speed: Option<i32>,
    /// `indieJump` — flat Jump. Haste: 1 at level 1, 10 at 20.
    pub indie_jump: Option<i32>,
    /// `indiePad` — flat Weapon Attack. Rage: 10..40.
    pub indie_pad: Option<i32>,
    /// `indieMad` — flat Magic Attack. No Warrior, Archer or Rogue skill carries it.
    pub indie_mad: Option<i32>,
    /// `indiePdd` — flat Weapon Defence. Iron Will: +20..+50; Rage: **-10..-40**.
    pub indie_pdd: Option<i32>,
    /// `indieMdd` — flat Magic Defence.
    pub indie_mdd: Option<i32>,
    /// `mhpR` — Max HP Increase's **percent of maximum HP**, 10..25. **The client applies
    /// it on top of the maximum this server sends** (measured 2026-09-06: server 358, screen
    /// 447 = 358 + ⌊358 × 25 / 100⌋), so the server must never fold it into `max_hp` and
    /// must raise every ceiling it caps HP against by the same amount - `session::pools`.
    pub max_hp_percent: Option<u32>,
    /// `mmpR` — Max MP Increase's percent of maximum MP. Same handling as
    /// [`CastNumbers::max_hp_percent`]; **[D]** by symmetry, the MP twin has not been watched.
    pub max_mp_percent: Option<u32>,
    /// `noBulletConsume` — present, and `1`, on exactly three skills in the whole archive, all
    /// of them hidden third-job archer hits (`3101005`, `3111006`, `3211006`): the shot takes
    /// no arrow whatever `bulletCount` says. **[L]**
    pub no_bullet_consume: bool,
    /// `itemCon` / `itemConNo` — an item the cast consumes, and how many. Magic Rock
    /// `4006000` for Spell Booster, Mystic Door and Meso Saver; Summoning Rock `4006001` for
    /// Shadow Partner and the four summons. **[L]**
    pub item_con: Option<u32>,
    pub item_con_no: Option<u32>,
    /// `moneyCon` — mesos per cast. Shadow Meso alone carries it, 200..500. **[L]**
    pub money_con: Option<u32>,
    /// `x` / `y` — **skill-specific, and the WZ never says which.** Booster's `x` is an attack
    /// speed stage (`-2`), Power Guard's a percent reflected, Dragon Blood's an HP drain,
    /// Hyper Body's nothing at all. Read them only beside the skill id that gives them a
    /// meaning; `crate::advbuffs` is where those meanings are written down.
    pub x: Option<i32>,
    pub y: Option<i32>,
    /// `prop` — a percent chance. Drain's absorb, MP Eater's proc, Mortal Blow's kill.
    pub prop: Option<u32>,
    /// `indieAcc` / `indieEva` — Bless's two flat grants, on CTS 88 and 89.
    pub indie_acc: Option<i32>,
    pub indie_eva: Option<i32>,
    /// `indieMhpR` — Hyper Body's **percent** of max HP. Unlike `mhpR` (a passive the client
    /// applies to its own drawing) this rides a temporary stat, so the server has to raise its
    /// ceilings only **while the stat is held** — `session::pools` reads it off the held buff.
    pub indie_mhp_r: Option<u32>,
    /// `fixdamage` - a hit of exactly this many points, whatever the stats. Three Snails' 15.
    pub fix_damage: Option<u32>,
    /// The **tooltip's** `damage N%`, for a skill whose `damage` column says 0 although it
    /// deals damage. Arrow Bomb is the one: `damage` 0 at every level, tooltip *"damage 80%"*
    /// ... *"damage 140%"*. **[L]**, the client's own text; `None` when it states no percent.
    pub tooltip_damage_percent: Option<u32>,
}

/// The `N` in the first `damage N%` of a tooltip, case-insensitive. `"MP -14; Stun chance 30%
/// for 2 sec; damage 80%"` gives 80; the stun's `30%` is not preceded by "damage".
pub fn tooltip_damage_percent(tooltip: &str) -> Option<u32> {
    let lower = tooltip.to_ascii_lowercase();
    let mut from = 0;
    while let Some(at) = lower[from..].find("damage ") {
        let start = from + at + "damage ".len();
        let digits: String = lower[start..].chars().take_while(|c| c.is_ascii_digit()).collect();
        if !digits.is_empty() && lower[start + digits.len()..].starts_with('%') {
            return digits.parse().ok();
        }
        from = start;
    }
    None
}

/// One skill's rows, plus the columns that are constant across them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillCombat {
    pub id: u32,
    pub job: u16,
    pub name: String,
    pub max_level: u32,
    /// `type`. `None` for an empty cell.
    pub wz_type: Option<u32>,
    /// `processtype`. `None` for an empty cell.
    pub processtype: Option<i32>,
    /// `psd == 1`.
    pub psd: bool,
    /// The skill carries an `lt`/`rb` rectangle on at least one level.
    ///
    /// **This is what makes a buff a PARTY buff.** The owner, 2026-09-06: *"party buffs should
    /// apply to everyone in the party who is in the same map."* The archive has no column
    /// that says "party" in words; what the party buffs have and the self buffs lack is the
    /// rectangle the client draws the cast's area from - Rage and both Hastes carry `ltX
    /// -250, rbX 250` (widening at higher levels) plus `processtype 17`; Iron Body, Focus,
    /// Magic Armor and **Iron Will** carry no rectangle and `processtype 6`, the timed
    /// self-grant. Iron Will being self-only in *this* client's data is worth knowing before
    /// anyone reports it as a bug: the tooltip says only "Weapon Def. +N", no party wording.
    /// **[L]** for the columns, **[I]** that the rectangle is the party discriminator - it
    /// is the reading every reference server makes, and it is one row in a table to change.
    pub party_rect: bool,
    /// The raw `weapon`..`weapon4` codes present, in column order. **Empty means ungated.**
    weapon_codes: Vec<u32>,
    /// Indexed by `level - 1`. Private so the off-by-one lives in [`SkillCombat::level`] only.
    levels: Vec<Option<CastNumbers>>,
}

impl SkillCombat {
    /// The numbers for one level, as the client numbers them — `1` is the first.
    pub fn level(&self, level: u32) -> Option<&CastNumbers> {
        if level == 0 {
            return None;
        }
        self.levels.get(level as usize - 1)?.as_ref()
    }

    /// Every level that loaded, ascending. Skips any that did not.
    pub fn levels(&self) -> impl Iterator<Item = &CastNumbers> {
        self.levels.iter().flatten()
    }

    /// How many level rows actually loaded.
    pub fn levels_loaded(&self) -> usize {
        self.levels.iter().flatten().count()
    }

    /// The raw 30..=47 codes from `weapon`..`weapon4`. **Empty means ungated.**
    pub fn weapon_codes(&self) -> &[u32] {
        &self.weapon_codes
    }

    /// The gate as [`WeaponClass`]es.
    ///
    /// A code with no class — the client's own switch covers 30..=33, 37..=47 and nothing else
    /// — is **dropped and counted**, so it cannot become a silently narrower gate. The count
    /// is what the caller checks; see [`SkillCombat::unmappable_weapon_codes`].
    pub fn weapon_gate(&self) -> Vec<WeaponClass> {
        self.weapon_codes.iter().filter_map(|c| WeaponClass::from_code(*c)).collect()
    }

    /// How many `weapon` codes did not map to a [`WeaponClass`]. `0` for all 24.
    pub fn unmappable_weapon_codes(&self) -> usize {
        self.weapon_codes.iter().filter(|c| WeaponClass::from_code(**c).is_none()).count()
    }

    /// The columns [`classify`] reads, taken over **all** of this skill's level rows.
    ///
    /// "Present" means present on at least one level: `damage`, `mad` and `mpCon` are all
    /// per-level columns, and a skill whose level 1 happened to omit one would otherwise be
    /// classified from a single row.
    pub fn evidence(&self) -> ColumnEvidence {
        ColumnEvidence {
            has_damage: self.levels().any(|l| l.damage_percent.is_some()),
            has_mad: self.levels().any(|l| l.mad_percent.is_some()),
            has_mp_con: self.levels().any(|l| l.mp_con.is_some()),
            wz_type: self.wz_type,
            psd: self.psd,
            processtype: self.processtype,
        }
    }

    /// [`classify`] applied to this skill's own columns.
    pub fn kind(&self) -> Kind {
        self.evidence().kind()
    }
}

/// `skillId -> SkillCombat`, loaded from `gm-handbook/skills.txt`.
///
/// A missing file gives an empty table and a banner that says so, never an error — the same
/// degradation [`crate::skilltable::SkillTable`] makes, and for the same reason: the server
/// then behaves exactly as it did before this existed.
#[derive(Debug, Clone, Default)]
pub struct CombatTable {
    by_id: BTreeMap<u32, SkillCombat>,
    source: String,
    problems: usize,
    level_rows: usize,
    header_ok: bool,
}

/// The columns this module needs, **by name**.
///
/// [`crate::skilltable`] hard-codes indices. This resolves them from the generated file's own
/// header line instead, because five of these columns are ones that file does not read and a
/// wrong index would be invisible: `bulletConsume` decoded from the `bulletCount` column would
/// produce a plausible number for every archer skill.
///
/// The resolved indices are asserted against the documented literals in the tests, so this is
/// belt *and* braces rather than either one.
const WANTED: [&str; 19] = [
    "skillId",
    "job",
    "level",
    "maxLevel",
    "name",
    "type",
    "psd",
    "processtype",
    "weapon",
    "weapon2",
    "weapon3",
    "weapon4",
    "mpCon",
    "hpCon",
    "bulletConsume",
    "time",
    "cooltime",
    "damage",
    "mad",
];

/// The rest, kept separate only so [`WANTED`] stays under a readable width.
const WANTED_MORE: [&str; 3] = ["attackCount", "mobCount", "bulletCount"];

/// Columns read **when present**, and silently `None` when the header lacks them.
///
/// Unlike [`WANTED`], a missing one does not refuse the whole file: they were added to the
/// generator on 2026-09-06 for the second-job buffs, and a `skills.txt` generated before
/// that - or the hand-written header in this module's own tests - must still load its
/// combat numbers. The cost of that leniency is bounded: a missing column reads as "the
/// skill grants no such stat", which turns a party buff into a chat notice rather than into
/// a wrong number.
const OPTIONAL: [&str; 21] = [
    "indieSpeed", "indieJump", "indiePad", "indieMad", "indiePdd", "indieMdd", "ltX", "mhpR", "mmpR",
    // Added 2026-09-07 for the second- and third-job audit. Same leniency, same bounded cost.
    "noBulletConsume", "itemCon", "itemConNo", "moneyCon", "x", "y", "prop", "indieAcc", "indieEva",
    "indieMhpR",
    // Added 2026-10-02 for the damage guard (`crate::damageguard`): Three Snails' fixed damage,
    // and the tooltip - the only place Arrow Bomb states its damage percent.
    "fixdamage", "tooltip",
];

/// Resolve the [`OPTIONAL`] columns that this header actually has.
fn optional_columns(header: &str) -> BTreeMap<&'static str, usize> {
    let names: Vec<&str> =
        header.trim_start_matches('#').split(',').map(str::trim).collect();
    OPTIONAL
        .iter()
        .filter_map(|want| names.iter().position(|n| n == want).map(|i| (*want, i)))
        .collect()
}

/// A bound on the level vector one row may ask for.
///
/// **An allocation guard, not a claim about the game** — the highest level in the client's data
/// is 30. Same reasoning as `skilltable::MAX_SKILL_LEVEL`.
const LEVEL_ALLOCATION_CAP: u32 = 255;

/// Read one cell. **Empty means absent, not zero.**
///
/// Quotes are stripped: 1 499 numeric cells in the WZ archive are quoted strings, and although
/// `tools/dump_skills.py` already unquotes them, a change on the generator side must not turn
/// every one of those cells into a dropped row.
///
/// `Err(())` means present and unreadable, which is a different outcome from absent and is what
/// makes the caller drop the whole row.
fn cell<T: FromStr>(raw: &str) -> Result<Option<T>, ()> {
    let s = raw.trim().trim_matches('"').trim();
    if s.is_empty() {
        return Ok(None);
    }
    s.parse::<T>().map(Some).map_err(|_| ())
}

/// Resolve every wanted column name to its index in the file's own header line.
///
/// `None` if the header is missing or does not name all of them — in which case the table
/// loads nothing and says so, rather than reading columns by position and hoping.
fn resolve_columns(header: &str) -> Option<BTreeMap<&'static str, usize>> {
    let names: Vec<&str> =
        header.trim_start_matches('#').split(',').map(str::trim).collect();
    let mut out = BTreeMap::new();
    for want in WANTED.iter().chain(WANTED_MORE.iter()) {
        let idx = names.iter().position(|n| n == want)?;
        out.insert(*want, idx);
    }
    Some(out)
}

impl CombatTable {
    /// Load `gm-handbook/skills.txt`.
    pub fn load(path: &Path) -> Self {
        let mut out =
            CombatTable { source: path.display().to_string(), ..CombatTable::default() };
        let Ok(text) = std::fs::read_to_string(path) else { return out };

        // The header first: without it nothing is read at all. A file whose columns moved is a
        // file this module must refuse, not one it should read by position.
        let Some(header) = text
            .lines()
            .find(|l| l.trim_start().starts_with('#') && l.contains("skillId"))
        else {
            return out;
        };
        let Some(col) = resolve_columns(header) else { return out };
        let opt = optional_columns(header);
        out.header_ok = true;
        // **The row width comes from the header itself**, not from a literal, so a generator
        // that adds a column does not turn every row into a dropped one.
        let n_columns = header.trim_start().trim_start_matches('#').split(',').count();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split(',').map(str::trim).collect();
            // A row with the wrong column count is dropped WHOLE. A half-read row would put a
            // wrong maximum or a wrong bullet cost on a skill with no second check behind it.
            if f.len() != n_columns {
                out.problems += 1;
                continue;
            }
            let at = |name: &str| f[col[name]];
            let (Ok(id), Ok(job), Ok(max_level), Ok(level)) = (
                at("skillId").parse::<u32>(),
                at("job").parse::<u16>(),
                at("maxLevel").parse::<u32>(),
                at("level").parse::<u32>(),
            ) else {
                out.problems += 1;
                continue;
            };
            if level == 0 || level > LEVEL_ALLOCATION_CAP {
                out.problems += 1;
                continue;
            }
            let Ok(row) = parse_cast(&f, &col, &opt, level) else {
                out.problems += 1;
                continue;
            };
            // A rectangle on any level marks the whole skill; see `SkillCombat::party_rect`.
            let has_rect = opt.get("ltX").is_some_and(|i| !f[*i].trim().is_empty());
            let (Ok(wz_type), Ok(psd_raw), Ok(processtype)) = (
                cell::<u32>(at("type")),
                cell::<u32>(at("psd")),
                cell::<i32>(at("processtype")),
            ) else {
                out.problems += 1;
                continue;
            };
            let mut weapon_codes = Vec::new();
            let mut weapon_bad = false;
            for w in ["weapon", "weapon2", "weapon3", "weapon4"] {
                match cell::<u32>(at(w)) {
                    Ok(Some(code)) => weapon_codes.push(code),
                    Ok(None) => {}
                    Err(()) => weapon_bad = true,
                }
            }
            if weapon_bad {
                out.problems += 1;
                continue;
            }

            let skill = out.by_id.entry(id).or_insert_with(|| SkillCombat {
                id,
                job,
                name: at("name").to_string(),
                max_level,
                wz_type,
                processtype,
                // `psd` is `1` or absent in this archive. Any other value is treated as set
                // and the raw number is not kept, because nothing here reads it as a number.
                psd: psd_raw.is_some(),
                party_rect: false,
                weapon_codes: weapon_codes.clone(),
                levels: Vec::new(),
            });
            // **The per-skill columns must not move between rows.** They do not in the file as
            // generated today; a disagreement is counted rather than silently taking the first,
            // because a weapon gate that changed at level 11 would be a real change in meaning.
            if skill.weapon_codes != weapon_codes
                || skill.wz_type != wz_type
                || skill.processtype != processtype
                || skill.psd != psd_raw.is_some()
                || skill.max_level != max_level
            {
                out.problems += 1;
                continue;
            }

            let idx = level as usize - 1;
            if skill.levels.len() <= idx {
                skill.levels.resize(idx + 1, None);
            }
            if skill.levels[idx].is_some() {
                out.problems += 1; // a duplicate row for a level already loaded
                continue;
            }
            skill.levels[idx] = Some(row);
            skill.party_rect |= has_rect;
            out.level_rows += 1;
        }
        out
    }

    pub fn get(&self, id: u32) -> Option<&SkillCombat> {
        self.by_id.get(&id)
    }

    /// The numbers for one level of one skill.
    pub fn level(&self, skill_id: u32, level: u32) -> Option<&CastNumbers> {
        self.get(skill_id)?.level(level)
    }

    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }

    /// How many skill-**level** rows loaded.
    pub fn level_rows(&self) -> usize {
        self.level_rows
    }

    /// Whether the file's header line named every column this module needs.
    pub fn header_ok(&self) -> bool {
        self.header_ok
    }

    /// How many rows were dropped.
    pub fn problems(&self) -> usize {
        self.problems
    }

    /// **How many of the 24 loaded**, which is the number that decides whether this server can
    /// answer a first-job cast at all.
    pub fn first_job_skills_loaded(&self) -> usize {
        FIRST_JOB_SKILLS.iter().filter(|s| self.get(s.id).is_some()).count()
    }

    /// Printed at start-up in **both** directions — a banner that is silent when things are
    /// fine cannot be told apart from one that is not being printed.
    pub fn banner(&self) -> String {
        let from = if self.source.is_empty() { "(no file)" } else { self.source.as_str() };
        if !self.header_ok {
            return format!(
                "maplecw-world: first-job combat: NO USABLE HEADER in {from}. Nothing loaded, \
                 so no first-job skill has a cast cost, a bullet cost or a weapon gate. \
                 Regenerate with: python tools/dump_skills.py"
            );
        }
        format!(
            "maplecw-world: first-job combat: {}/24 first-job skills, {} skills, {} \
             skill-levels from {from}{}",
            self.first_job_skills_loaded(),
            self.by_id.len(),
            self.level_rows,
            if self.problems > 0 {
                format!(" ({} unreadable line(s))", self.problems)
            } else {
                String::new()
            }
        )
    }
}

/// The per-level half of one row. `Err(())` if any cell is present and unreadable.
fn parse_cast(
    f: &[&str],
    col: &BTreeMap<&'static str, usize>,
    opt: &BTreeMap<&'static str, usize>,
    level: u32,
) -> Result<CastNumbers, ()> {
    let at = |name: &str| f[col[name]];
    // An optional column the header lacks reads as an empty cell, never as an error.
    let at_opt = |name: &str| -> Result<Option<i32>, ()> {
        match opt.get(name) {
            Some(i) => cell(f[*i]),
            None => Ok(None),
        }
    };
    Ok(CastNumbers {
        level,
        mp_con: cell(at("mpCon"))?,
        hp_con: cell(at("hpCon"))?,
        damage_percent: cell(at("damage"))?,
        mad_percent: cell(at("mad"))?,
        attack_count: cell(at("attackCount"))?,
        mob_count: cell(at("mobCount"))?,
        bullet_count: cell(at("bulletCount"))?,
        bullet_consume: cell(at("bulletConsume"))?,
        time_seconds: cell(at("time"))?,
        cooltime_seconds: cell(at("cooltime"))?,
        indie_speed: at_opt("indieSpeed")?,
        indie_jump: at_opt("indieJump")?,
        indie_pad: at_opt("indiePad")?,
        indie_mad: at_opt("indieMad")?,
        indie_pdd: at_opt("indiePdd")?,
        indie_mdd: at_opt("indieMdd")?,
        max_hp_percent: at_opt("mhpR")?.map(|v| u32::try_from(v).unwrap_or(0)),
        max_mp_percent: at_opt("mmpR")?.map(|v| u32::try_from(v).unwrap_or(0)),
        no_bullet_consume: at_opt("noBulletConsume")?.is_some_and(|v| v != 0),
        item_con: at_opt("itemCon")?.map(|v| u32::try_from(v).unwrap_or(0)),
        item_con_no: at_opt("itemConNo")?.map(|v| u32::try_from(v).unwrap_or(0)),
        money_con: at_opt("moneyCon")?.map(|v| u32::try_from(v).unwrap_or(0)),
        x: at_opt("x")?,
        y: at_opt("y")?,
        prop: at_opt("prop")?.map(|v| u32::try_from(v).unwrap_or(0)),
        indie_acc: at_opt("indieAcc")?,
        indie_eva: at_opt("indieEva")?,
        indie_mhp_r: at_opt("indieMhpR")?.map(|v| u32::try_from(v).unwrap_or(0)),
        fix_damage: at_opt("fixdamage")?.map(|v| u32::try_from(v).unwrap_or(0)),
        tooltip_damage_percent: opt.get("tooltip").and_then(|i| f.get(*i)).and_then(|t| tooltip_damage_percent(t)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skilltable::SkillTable;

    const REAL: &str = "../../gm-handbook/skills.txt";

    /// The real table, or `None` on a clean checkout where `gm-handbook/` has not been
    /// generated. Every test that uses it asserts a **positive control first**.
    fn real_table() -> Option<CombatTable> {
        let path = Path::new(REAL);
        if !path.exists() {
            return None;
        }
        let t = CombatTable::load(path);
        assert!(t.header_ok(), "the generated file has a header naming every wanted column");
        assert_eq!(t.problems(), 0, "every row parses: {}", t.banner());
        Some(t)
    }

    /// **The second-job buffs read their grants and their party rectangle from the table.**
    ///
    /// The owner, 2026-09-06. Positive control first - a first-job row still carries its combat
    /// numbers through the widened parser - then Haste, Rage and Iron Will as the file has
    /// them. **Iron Will's `party_rect` being false is a finding, not a bug**: in this
    /// client's data it has no `lt`/`rb` and `processtype 6`, the self-buff shape that Iron
    /// Body and Focus have. And a percent column is not a flat grant: Iron Body's `indiePddR`
    /// must not surface as `indie_pdd`, or it would be sent as 5 flat W.Def.
    #[test]
    fn the_second_job_buffs_read_their_grants_and_rectangles_from_the_table() {
        let Some(t) = real_table() else { return };
        assert_eq!(t.level(1_001_002, 1).and_then(|l| l.hp_con), Some(3), "positive control: Slash Blast");

        let haste = t.get(4_101_001).expect("Haste (Assassin)");
        assert!(haste.party_rect, "Haste carries lt/rb");
        let l1 = haste.level(1).unwrap();
        assert_eq!(
            (l1.indie_speed, l1.indie_jump, l1.time_seconds, l1.mp_con),
            (Some(10), Some(1), Some(100), Some(15))
        );
        let l20 = haste.level(20).unwrap();
        assert_eq!((l20.indie_speed, l20.indie_jump, l20.time_seconds), (Some(30), Some(10), Some(300)));
        assert!(t.get(4_201_001).is_some_and(|s| s.party_rect), "and the Bandit's twin");

        let rage = t.get(1_101_004).expect("Rage");
        assert!(rage.party_rect);
        let l1 = rage.level(1).unwrap();
        assert_eq!(
            (l1.indie_pad, l1.indie_pdd, l1.time_seconds),
            (Some(10), Some(-10), Some(150)),
            "attack up, defence DOWN - the sign is in the data"
        );

        let iron_will = t.get(1_301_004).expect("Iron Will");
        assert!(!iron_will.party_rect, "no rectangle in this client's data: self only");
        assert_eq!(iron_will.level(1).unwrap().indie_pdd, Some(20));

        let iron_body = t.level(1_001_000, 1).unwrap();
        assert_eq!((iron_body.indie_pdd, iron_body.indie_speed), (None, None), "indiePddR is a percent, not a grant");
        assert!(!t.get(1_001_000).unwrap().party_rect, "a self buff");

        // The two max-pool passives, as percents - 15 levels of 10..25 on each.
        assert_eq!(t.level(1_000_001, 1).unwrap().max_hp_percent, Some(10), "Max HP Increase L1");
        assert_eq!(t.level(1_000_001, 15).unwrap().max_hp_percent, Some(25), "Max HP Increase L15 - Cobalt's");
        assert_eq!(t.level(2_000_001, 15).unwrap().max_mp_percent, Some(25), "Max MP Increase L15");
        assert_eq!(t.level(1_000_001, 15).unwrap().max_mp_percent, None, "HP skill carries no mmpR");
        assert_eq!(t.level(1_001_002, 1).unwrap().max_hp_percent, None, "an attack carries neither");
    }

    /// **The static table is the client's own book, not a memory of MapleStory.**
    ///
    /// Positive control first: if `gm-handbook/skills.txt` exists at all, all 24 ids must be in
    /// it with the job, name and ceiling written here.
    #[test]
    fn the_twenty_four_are_the_clients_own_book() {
        let Some(t) = real_table() else { return };
        assert!(t.len() > 100, "the whole archive loaded: {} skills", t.len());
        assert_eq!(t.first_job_skills_loaded(), 24, "{}", t.banner());

        for s in &FIRST_JOB_SKILLS {
            let f = t.get(s.id).unwrap_or_else(|| panic!("{} ({}) is missing", s.name, s.id));
            assert_eq!(f.name, s.name, "{}", s.id);
            assert_eq!(f.job, s.job, "{}", s.name);
            assert_eq!(f.max_level, s.max_level, "{}", s.name);
            assert_eq!(f.wz_type, s.wz_type, "{} `type` column", s.name);
            assert_eq!(f.processtype, s.processtype, "{} `processtype` column", s.name);
            assert_eq!(f.psd, s.psd, "{} `psd` column", s.name);
            // Every level the ceiling promises actually loaded. A `max_level` of 20 with 19
            // rows behind it would refuse a cast at the top level while `+` still allowed it.
            assert_eq!(f.levels_loaded(), s.max_level as usize, "{} level rows", s.name);
            assert!(f.level(0).is_none(), "{} has no level 0", s.name);
            assert!(f.level(s.max_level + 1).is_none(), "{} stops at its ceiling", s.name);
        }

        // **The enumeration, not a lookup of a remembered list.** Exactly six skills in the
        // whole archive belong to each of the four first-job books, and they are these.
        for job in [100u16, 200, 300, 400] {
            let from_file: Vec<u32> =
                t.by_id.values().filter(|s| s.job == job).map(|s| s.id).collect();
            let from_here: Vec<u32> = book(job).map(|s| s.id).collect();
            assert_eq!(from_file, from_here, "book {job}");
            assert_eq!(from_here.len(), 6, "book {job} has six skills");
        }
        // And there is no fifth branch: no pirate book.
        assert!(t.by_id.values().all(|s| s.job / 100 != 5), "this client has no Pirate branch");
    }

    /// **The classification is re-derived from the file, never trusted from the literal.**
    ///
    /// This is the test that would break if `Kind` were written from a memory of the game
    /// rather than from the columns.
    #[test]
    fn every_kind_is_reproduced_from_the_columns() {
        let Some(t) = real_table() else { return };
        let (mut attacks, mut buffs, mut passives) = (0usize, 0usize, 0usize);
        for s in &FIRST_JOB_SKILLS {
            let f = t.get(s.id).unwrap();
            assert_eq!(f.kind(), s.kind, "{} classified from its own columns", s.name);
            match s.kind {
                Kind::Attack => attacks += 1,
                Kind::Buff => buffs += 1,
                Kind::Passive => passives += 1,
            }
            // The rule's own inputs, stated per skill so a change in the file is legible.
            let e = f.evidence();
            match s.kind {
                Kind::Attack => assert!(
                    e.has_damage || e.has_mad,
                    "{} is an attack because it carries damage or mad",
                    s.name
                ),
                Kind::Buff => assert!(
                    !e.has_damage && !e.has_mad && e.has_mp_con,
                    "{} is a buff because it costs MP and deals none",
                    s.name
                ),
                Kind::Passive => assert!(
                    !e.has_damage && !e.has_mad && !e.has_mp_con,
                    "{} is passive because it carries none of the three",
                    s.name
                ),
            }
            // No skill carries both damage columns - asserted per skill as well as archive-wide
            // below, because this is what makes `DamageDuty` a two-way choice.
            assert!(!(e.has_damage && e.has_mad), "{} carries damage AND mad", s.name);
        }
        assert_eq!((attacks, buffs, passives), (10, 5, 9), "10 attacks, 5 buffs, 9 passives");

        // Archive-wide, so the rule rests on a census rather than on the 24 agreeing with
        // themselves. These are the numbers the module header quotes.
        let both = t
            .by_id
            .values()
            .filter(|s| {
                s.levels().any(|l| l.damage_percent.is_some())
                    && s.levels().any(|l| l.mad_percent.is_some())
            })
            .count();
        assert_eq!(both, 0, "no skill in the archive carries both `damage` and `mad`");
        for s in t.by_id.values() {
            let has_mp = s.levels().any(|l| l.mp_con.is_some());
            match s.wz_type {
                Some(10) => assert!(has_mp, "{} is type 10 and must carry mpCon", s.name),
                Some(50) => assert!(!has_mp, "{} is type 50 and must not carry mpCon", s.name),
                _ => {}
            }
        }
    }

    /// **Where two of the client's own columns disagree, and the disagreement is the finding.**
    ///
    /// `CLAUDE.md`: a table row written from a quick read is a claim. This one is asserted, in
    /// both directions, so that "type 50 means psd" cannot quietly become true later.
    #[test]
    fn type_fifty_and_psd_are_not_the_same_set() {
        let Some(t) = real_table() else { return };

        // Among the 24: nine are type 50 and seven carry psd.
        let type50: Vec<u32> =
            FIRST_JOB_SKILLS.iter().filter(|s| s.wz_type == Some(50)).map(|s| s.id).collect();
        let psd: Vec<u32> = FIRST_JOB_SKILLS.iter().filter(|s| s.psd).map(|s| s.id).collect();
        assert_eq!(type50.len(), 9, "nine of the 24 are type 50");
        assert_eq!(psd.len(), 7, "seven of the 24 carry psd");

        // The two that differ, named rather than counted.
        for id in [1_000_000u32, 2_000_000] {
            let f = t.get(id).unwrap();
            assert_eq!(f.wz_type, Some(50), "{} is type 50", f.name);
            assert!(!f.psd, "{} carries NO psd - the disagreement", f.name);
            assert_eq!(f.kind(), Kind::Passive, "and the rule calls it passive anyway");
        }
        // Every other type-50 skill among the 24 does carry psd, so the exception is exactly
        // two and not "some".
        for s in FIRST_JOB_SKILLS.iter().filter(|s| s.wz_type == Some(50)) {
            if s.id != 1_000_000 && s.id != 2_000_000 {
                assert!(s.psd, "{} is type 50 and carries psd", s.name);
            }
        }

        // Archive-wide there are exactly four type-50 skills without psd, and the other two are
        // both copies of MP Eater. A positive control that the sweep can see anything at all.
        let no_psd: Vec<u32> = t
            .by_id
            .values()
            .filter(|s| s.wz_type == Some(50) && !s.psd)
            .map(|s| s.id)
            .collect();
        assert_eq!(no_psd, vec![1_000_000, 2_000_000, 2_200_000, 2_300_000]);

        // And `psd` is NOT "does nothing": archive-wide it co-occurs with `damage` on the two
        // Mortal Blows, which is why `psd_agrees` is only claimed for these 24.
        let psd_and_damage: Vec<u32> = t
            .by_id
            .values()
            .filter(|s| s.psd && s.levels().any(|l| l.damage_percent.is_some()))
            .map(|s| s.id)
            .collect();
        assert_eq!(psd_and_damage, vec![3_110_000, 3_210_000], "both Mortal Blows");
        for s in &FIRST_JOB_SKILLS {
            assert!(t.get(s.id).unwrap().evidence().psd_agrees(), "{}", s.name);
        }
    }

    /// **`type` cannot be the classifier, and Power Knockback is why.**
    #[test]
    fn the_type_column_is_empty_on_one_of_the_twenty_four() {
        let Some(t) = real_table() else { return };
        let pk = t.get(3_001_003).expect("Power Knockback");
        assert_eq!(pk.wz_type, None, "its `type` cell is EMPTY");
        assert_eq!(pk.evidence().type_column_agrees(), None, "so `type` has no opinion");
        assert_eq!(pk.kind(), Kind::Attack, "while its `damage` column runs 105..180");
        assert_eq!(pk.level(1).unwrap().damage_percent, Some(105));
        assert_eq!(pk.level(15).unwrap().damage_percent, Some(180));

        // Every OTHER one of the 24 has a type, and it agrees with the rule. So the rule is not
        // merely different from `type` - it reproduces it wherever `type` speaks.
        for s in &FIRST_JOB_SKILLS {
            if s.id == 3_001_003 {
                continue;
            }
            assert_eq!(
                t.get(s.id).unwrap().evidence().type_column_agrees(),
                Some(true),
                "{} - the rule and the `type` column agree",
                s.name
            );
        }

        // Archive-wide: `type` is empty on 105 of 176 skills, and 36 of those carry `damage`.
        // This is the census the module header quotes, and it is the reason `type` was rejected.
        let empty_type = t.by_id.values().filter(|s| s.wz_type.is_none()).count();
        assert_eq!(empty_type, 105, "of {} skills", t.len());
        let empty_type_with_damage = t
            .by_id
            .values()
            .filter(|s| s.wz_type.is_none() && s.levels().any(|l| l.damage_percent.is_some()))
            .count();
        assert_eq!(empty_type_with_damage, 36);
    }

    /// **`processtype` is signed, and this is the control that proves it.**
    ///
    /// Written as an assertion rather than a comment because it is exactly the failure the
    /// module header describes: parsed as `u32`, this reader silently dropped 330 rows and 12
    /// whole skills and still reported `24/24 first-job skills` in its banner, because none of
    /// the 24 is affected. A reader that is short, clean and confident is the thing `CLAUDE.md`
    /// keeps warning about, and only the archive-wide row count catches it.
    #[test]
    fn processtype_is_signed_and_twelve_skills_carry_minus_one() {
        let Some(t) = real_table() else { return };
        // The row totals, which are what a truncated parse moves. `real_table` has already
        // asserted `problems == 0`; these say the reader saw everything there was to see.
        assert_eq!(t.len(), 176, "every skill in the archive loaded");
        assert_eq!(t.level_rows(), 4164, "every skill-level row loaded");

        let negative: Vec<u32> = t
            .by_id
            .values()
            .filter(|s| s.processtype.is_some_and(|p| p < 0))
            .map(|s| s.id)
            .collect();
        assert_eq!(
            negative,
            vec![
                1_101_005, 1_201_005, 1_301_005, 2_101_005, 2_111_001, 3_101_005, 3_111_002,
                3_111_006, 3_211_002, 3_211_006, 4_211_001, 4_211_004
            ],
            "the twelve skills whose processtype is -1"
        );
        assert!(
            t.by_id.values().all(|s| s.processtype.is_none_or(|p| p >= -1)),
            "-1 is the only negative the column takes"
        );
        // None of the 24 is among them, which is why the banner could not have caught this.
        for s in &FIRST_JOB_SKILLS {
            assert!(s.processtype.is_none_or(|p| p >= 0), "{}", s.name);
        }
    }

    /// **The weapon gate, read off the file rather than remembered.**
    #[test]
    fn the_weapon_gate_is_what_the_file_says() {
        let Some(t) = real_table() else { return };

        // Positive control: the sweep can see a gate at all.
        assert_eq!(t.get(3_001_001).unwrap().weapon_codes(), [45, 46]);

        for s in &FIRST_JOB_SKILLS {
            let f = t.get(s.id).unwrap();
            assert_eq!(f.unmappable_weapon_codes(), 0, "{} weapon codes all map", s.name);
            assert_eq!(f.weapon_gate(), s.weapon_gate, "{} weapon gate", s.name);
            // The plain-English note actually names every class in the gate, so it cannot go
            // stale while the gate changes underneath it.
            for c in s.weapon_gate {
                assert!(
                    s.weapon_note.contains(&format!("{c:?}")),
                    "{}: note {:?} does not name {c:?}",
                    s.name,
                    s.weapon_note
                );
            }
            if s.weapon_gate.is_empty() {
                assert_eq!(s.weapon_note, NO_GATE, "{}", s.name);
            }
        }

        // The five that are gated, and the nineteen that are not. Counted, so a gate that
        // silently disappeared would fail here rather than pass quietly.
        let gated: Vec<u32> =
            FIRST_JOB_SKILLS.iter().filter(|s| !s.weapon_gate.is_empty()).map(|s| s.id).collect();
        assert_eq!(gated, vec![3_001_001, 3_001_002, 3_001_003, 4_001_002, 4_001_003]);
        assert_eq!(
            t.get(4_001_002).unwrap().weapon_gate(),
            vec![WeaponClass::Dagger],
            "Double Stab wants a Dagger, code 33"
        );
        assert_eq!(
            t.get(4_001_003).unwrap().weapon_gate(),
            vec![WeaponClass::Claw],
            "Lucky Seven wants a Claw, code 47"
        );

        // The six Magician skills carry no weapon column at all - the negative, measured over
        // all four columns on every row rather than looked for on level 1.
        for s in book(200) {
            assert!(t.get(s.id).unwrap().weapon_codes().is_empty(), "{}", s.name);
        }

        // Archive-wide: no skill anywhere gates on 37, 38 or 39 - wand, staff or bare hands.
        let codes: std::collections::BTreeSet<u32> =
            t.by_id.values().flat_map(|s| s.weapon_codes().iter().copied()).collect();
        assert!(codes.contains(&45) && codes.contains(&33), "the sweep sees real codes");
        for absent in [34u32, 35, 36, 37, 38, 39] {
            assert!(!codes.contains(&absent), "no skill gates on weapon code {absent}");
        }
    }

    /// **Two independent readers of one file must produce the same number.**
    ///
    /// `crate::skilltable` hard-codes column indices; this module resolves them by name. If
    /// either drifts, this fails. It is the check `CLAUDE.md` calls *count the same event in
    /// two logs*, applied to a column instead of a packet.
    #[test]
    fn combat_table_agrees_with_skilltable_on_every_shared_column() {
        let Some(t) = real_table() else { return };
        let s = SkillTable::load(Path::new(REAL));
        // `SkillTable::problems` is private, so the banner is the check it exposes.
        assert!(!s.banner().contains("unreadable"), "{}", s.banner());
        assert_eq!(s.level_rows(), t.level_rows(), "both readers see the same row count");
        assert!(s.level_rows() > 4000, "positive control: {} rows", s.level_rows());

        let mut compared = 0usize;
        for f in &FIRST_JOB_SKILLS {
            let mine = t.get(f.id).unwrap();
            let theirs = s.get(f.id).unwrap();
            assert_eq!(mine.max_level, theirs.max_level, "{}", f.name);
            for lv in 1..=f.max_level {
                let a = mine.level(lv).unwrap();
                let b = theirs.level(lv).unwrap();
                assert_eq!(a.mp_con, b.mp_con, "{} lv{lv} mpCon", f.name);
                assert_eq!(a.mad_percent, b.mad, "{} lv{lv} mad", f.name);
                assert_eq!(a.attack_count, b.attack_count, "{} lv{lv} attackCount", f.name);
                assert_eq!(a.mob_count, b.mob_count, "{} lv{lv} mobCount", f.name);
                assert_eq!(a.time_seconds, b.time_seconds, "{} lv{lv} time", f.name);
                assert_eq!(a.cooltime_seconds, b.cooltime_seconds, "{} lv{lv} cooltime", f.name);
                compared += 1;
            }
        }
        // 105 + 105 + 105 + 110: the four books' ceilings summed. A literal, so a skill whose
        // `max_level` moved cannot quietly shrink what this test covers.
        assert_eq!(compared, 425, "every level of all 24 was compared");
        assert_eq!(
            FIRST_JOB_SKILLS.iter().map(|s| s.max_level).sum::<u32>(),
            425,
            "and the static table agrees on the total"
        );
    }

    /// **The per-level numbers at both ends of every attack ladder**, in the units the field
    /// names claim.
    #[test]
    fn the_attack_numbers_are_what_the_client_ships() {
        let Some(t) = real_table() else { return };

        // (id, name, lv1 damage-or-mad, top damage-or-mad, attackCount, mobCount)
        let rows: [(u32, &str, u32, u32, u32, u32); 10] = [
            (1_001_001, "Power Strike", 160, 260, 1, 1),
            (1_001_002, "Slash Blast", 70, 130, 1, 4),
            (2_001_002, "Energy Bolt", 90, 130, 1, 1),
            (2_001_003, "Magic Claw", 45, 65, 2, 1),
            (3_001_001, "Arrow Blow", 160, 240, 1, 1),
            (3_001_002, "Double Shot", 80, 120, 1, 2),
            (3_001_003, "Power Knockback", 105, 180, 1, 2),
            (4_001_000, "Disorder", 100, 100, 1, 1),
            (4_001_002, "Double Stab", 80, 160, 2, 1),
            (4_001_003, "Lucky Seven", 60, 140, 1, 1),
        ];
        for (id, name, lo, hi, hits, targets) in rows {
            let f = t.get(id).unwrap_or_else(|| panic!("{name}"));
            let top = f.max_level;
            let pct = |l: &CastNumbers| l.damage_percent.or(l.mad_percent);
            assert_eq!(pct(f.level(1).unwrap()), Some(lo), "{name} level 1");
            assert_eq!(pct(f.level(top).unwrap()), Some(hi), "{name} level {top}");
            assert_eq!(f.level(1).unwrap().attack_count, Some(hits), "{name} attackCount");
            assert_eq!(f.level(1).unwrap().mob_count, Some(targets), "{name} mobCount");
            // Every attack costs MP at every level - the obligation says `deduct_mp`, and this
            // is the number it would deduct.
            for l in f.levels() {
                assert!(l.mp_con.is_some(), "{name} level {} has an mpCon", l.level);
            }
        }

        // The two hit-count traps, stated as assertions rather than as comments: applying the
        // percent once for these two halves the damage.
        assert_eq!(t.get(2_001_003).unwrap().level(20).unwrap().attack_count, Some(2));
        assert_eq!(t.get(4_001_002).unwrap().level(20).unwrap().attack_count, Some(2));

        // Disorder's `damage` is the only one that does not move across its ladder - which is
        // the observation the module header refuses to interpret.
        let dis = t.get(4_001_000).unwrap();
        assert!(
            dis.levels().all(|l| l.damage_percent == Some(100)),
            "Disorder carries damage 100 at every level"
        );
    }

    /// **An empty cell is not a zero**, on the four rows of these 24 where it is load-bearing.
    #[test]
    fn an_empty_cell_is_absent_and_not_a_zero() {
        let Some(t) = real_table() else { return };

        // Positive control: `hpCon` IS read when it is there.
        let slash = t.get(1_001_002).unwrap();
        assert_eq!(slash.level(1).unwrap().hp_con, Some(3), "\"HP -3\"");
        assert_eq!(slash.level(20).unwrap().hp_con, Some(8), "\"HP -8\"");
        // ... and absent on its sibling, which is a different fact from zero.
        let strike = t.get(1_001_001).unwrap();
        for l in strike.levels() {
            assert_eq!(l.hp_con, None, "Power Strike level {} has NO hpCon", l.level);
        }
        // Slash Blast is the only one of the 24 with an `hpCon` at all.
        let with_hp: Vec<u32> = FIRST_JOB_SKILLS
            .iter()
            .filter(|s| t.get(s.id).unwrap().levels().any(|l| l.hp_con.is_some()))
            .map(|s| s.id)
            .collect();
        assert_eq!(with_hp, vec![1_001_002]);

        // The two toggles carry no `time` at any level - the absence IS the data.
        for id in [2_001_000u32, 4_001_001] {
            let f = t.get(id).unwrap();
            for l in f.levels() {
                assert_eq!(l.time_seconds, None, "{} level {} is a toggle", f.name, l.level);
                assert_eq!(l.cooltime_seconds, None, "{} has no cooldown", f.name);
            }
            assert_eq!(f.processtype, Some(113), "{} is processtype 113", f.name);
        }
        // ... and archive-wide, all 16 `processtype 113` skills behave that way, which is what
        // `StatDuty::Toggle` rests on.
        let p113: Vec<&SkillCombat> =
            t.by_id.values().filter(|s| s.processtype == Some(113)).collect();
        assert_eq!(p113.len(), 16, "the toggle census");
        for s in &p113 {
            assert!(s.levels().all(|l| l.time_seconds.is_none()), "{} has a time", s.name);
        }

        // Lucky Seven fires two projectiles and the archive does not say what that costs.
        let seven = t.get(4_001_003).unwrap();
        for l in seven.levels() {
            assert_eq!(l.bullet_count, Some(2), "Lucky Seven level {}", l.level);
            assert_eq!(l.bullet_consume, None, "and NO bulletConsume - absent, not zero");
        }
        // Where it IS specified, it is read.
        assert_eq!(t.get(3_001_001).unwrap().level(1).unwrap().bullet_consume, Some(1));
        assert_eq!(t.get(3_001_002).unwrap().level(1).unwrap().bullet_consume, Some(2));
        // And `bulletConsume` is not a copy of `bulletCount`: Avenger fires one and takes four.
        let avenger = t.get(4_111_004).expect("Avenger, the counterexample");
        assert_eq!(avenger.level(1).unwrap().bullet_count, Some(1));
        assert_eq!(avenger.level(1).unwrap().bullet_consume, Some(4));
    }

    /// **The buffs' durations and the descending ladders**, both of which read as bugs if the
    /// unit or the direction is assumed.
    #[test]
    fn the_buffs_carry_seconds_and_two_ladders_descend() {
        let Some(t) = real_table() else { return };

        // Timed: three of the five, all `processtype 6`, all in SECONDS.
        for (id, name, lo, hi) in [
            (1_001_000u32, "Iron Body", 300u32, 600u32),
            (2_001_001, "Magic Armor", 300, 600),
            (3_001_000, "Focus", 70, 300),
        ] {
            let f = t.get(id).unwrap();
            assert_eq!(f.processtype, Some(6), "{name}");
            assert_eq!(f.level(1).unwrap().time_seconds, Some(lo), "{name} level 1");
            assert_eq!(f.level(f.max_level).unwrap().time_seconds, Some(hi), "{name} top");
            // The unit trap as an assertion: these are seconds, so none of them is a ms count.
            for l in f.levels() {
                assert!(l.time_seconds.unwrap() < 1000, "{name} level {} is seconds", l.level);
            }
        }

        // Iron Body's MP cost is a flat 15 at every level - it does not climb like the others.
        let iron = t.get(1_001_000).unwrap();
        assert!(iron.levels().all(|l| l.mp_con == Some(15)), "Iron Body costs 15 MP throughout");

        // Two ladders run DOWNWARDS, which a "higher level costs more" assumption gets wrong.
        let dark = t.get(4_001_001).unwrap();
        assert_eq!(dark.level(1).unwrap().mp_con, Some(50), "\"MP -50\"");
        assert_eq!(dark.level(20).unwrap().mp_con, Some(30), "\"MP -30\" - it goes DOWN");
        let pk = t.get(3_001_003).unwrap();
        assert_eq!(pk.level(1).unwrap().mp_con, Some(12));
        assert_eq!(pk.level(15).unwrap().mp_con, Some(8), "Power Knockback's cost falls too");

        // Power Knockback's `time` is constant and unexplained - recorded, not interpreted.
        assert!(
            pk.levels().all(|l| l.time_seconds == Some(1500)),
            "time 1500 at all 15 levels, unit unresolved"
        );

        // None of the 24 has a cooldown, unlike Nimble Feet's 180 s - so a cooldown map keyed
        // on these skills would never fire, and that is the data rather than a gap.
        for s in &FIRST_JOB_SKILLS {
            for l in t.get(s.id).unwrap().levels() {
                assert_eq!(l.cooltime_seconds, None, "{} level {}", s.name, l.level);
            }
        }
        assert_eq!(
            t.get(1002).unwrap().level(1).unwrap().cooltime_seconds,
            Some(180),
            "positive control: the reader CAN see a cooltime - Nimble Feet has one"
        );
    }

    /// **The obligations, checked against the columns they claim to come from.**
    ///
    /// `CLAUDE.md`: *"When a handler produces N effects, the test has to say something about N
    /// of them."* Each of the five fields is asserted for every one of the 24.
    #[test]
    fn every_obligation_matches_the_columns_it_came_from() {
        let Some(t) = real_table() else { return };
        for s in &FIRST_JOB_SKILLS {
            let f = t.get(s.id).unwrap();
            let o = s.obligation;

            // 1. MP: `mpCon` present at any level <-> the server deducts.
            let has_mp = f.levels().any(|l| l.mp_con.is_some());
            assert_eq!(o.deduct_mp, has_mp, "{} deduct_mp", s.name);
            // 2. HP: same rule for `hpCon`.
            let has_hp = f.levels().any(|l| l.hp_con.is_some());
            assert_eq!(o.deduct_hp, has_hp, "{} deduct_hp", s.name);
            // 3. Damage path: `damage` -> physical, `mad` -> magic, neither -> none.
            let expect = if f.levels().any(|l| l.damage_percent.is_some()) {
                DamageDuty::PhysicalPercent
            } else if f.levels().any(|l| l.mad_percent.is_some()) {
                DamageDuty::MagicPercent
            } else {
                DamageDuty::None
            };
            assert_eq!(o.damage, expect, "{} damage duty", s.name);
            // 4. Bullets: consume, or projectiles-with-no-consume-column, or neither.
            let l1 = f.level(1).unwrap();
            let expect = match (l1.bullet_consume, l1.bullet_count) {
                (Some(n), _) => BulletDuty::Consume(n),
                (None, Some(n)) => BulletDuty::ProjectilesNoConsumeColumn(n),
                (None, None) => BulletDuty::None,
            };
            assert_eq!(o.bullets, expect, "{} bullet duty", s.name);
            // 5. Stat: the kind and `processtype` between them decide it.
            let expect = match s.kind {
                Kind::Passive => StatDuty::PassiveUnknown,
                Kind::Buff if f.processtype == Some(113) => StatDuty::Toggle,
                Kind::Buff => StatDuty::Timed,
                // Disorder is the one attack that carries a `time`, and it is the mob's.
                Kind::Attack if f.levels().any(|l| l.time_seconds.is_some()) => {
                    if s.id == 4_001_000 {
                        StatDuty::MobDebuff
                    } else {
                        StatDuty::None
                    }
                }
                Kind::Attack => StatDuty::None,
            };
            assert_eq!(o.stat, expect, "{} stat duty", s.name);

            // A passive costs nothing to have and does nothing on a cast, for all nine.
            if s.kind == Kind::Passive {
                assert!(!o.is_castable(), "{} is not cast", s.name);
                assert_eq!(o.damage, DamageDuty::None, "{}", s.name);
                assert_eq!(o.bullets, BulletDuty::None, "{}", s.name);
            } else {
                assert!(o.deduct_mp, "{} is cast and costs MP", s.name);
            }
        }

        // `server_obligation` and the table cannot drift apart, and a non-first-job id is
        // `None` rather than a default.
        for s in &FIRST_JOB_SKILLS {
            assert_eq!(server_obligation(s.id), Some(s.obligation), "{}", s.name);
        }
        assert_eq!(server_obligation(1002), None, "Nimble Feet is not a first-job skill");
        assert_eq!(server_obligation(2_101_003), None, "nor is a second-job one");
        assert_eq!(server_obligation(0), None);
        assert_eq!(first_job_skill(9_999_999), None);
    }

    /// **Which obligations are admitted unknowns**, counted so that a later change that quietly
    /// resolves one has to say so here.
    #[test]
    fn the_unknowns_are_exactly_eleven_and_they_are_named() {
        let unknown: Vec<u32> = FIRST_JOB_SKILLS
            .iter()
            .filter(|s| s.obligation.has_unknown())
            .map(|s| s.id)
            .collect();
        // In `FIRST_JOB_SKILLS` order - job order, then ascending id within a book.
        assert_eq!(
            unknown,
            vec![
                // the nine passives: does the client apply them itself?
                1_000_000, 1_000_001, 1_000_002,
                // Energy Bolt's bullet cost is not in the data
                2_000_000, 2_000_001, 2_001_002, 3_000_000, 3_000_001, 4_000_000, 4_000_001,
                // nor is Lucky Seven's
                4_001_003,
            ],
            "nine passives plus Energy Bolt and Lucky Seven"
        );
        assert_eq!(unknown.len(), 11);

        // Every skill carries a note, and every unknown's note says so in words as well as in
        // the type - a comment describing a guarantee is not the guarantee, but a missing one
        // is a guarantee nobody wrote down.
        for s in &FIRST_JOB_SKILLS {
            assert!(!s.note.is_empty(), "{} has a note", s.name);
        }
    }

    /// **Which of the five buffs this server can actually grant today** — asked of
    /// `net::buff::buff_level` rather than pinned to a literal.
    ///
    /// `CLAUDE.md`: *"A test that pins what the code already does is not a check."* So this
    /// calls the real function. It is **meant to fail** the day somebody models Iron Body,
    /// Focus or Dark Sight, because that is the day `STATUS.md` and the test plan need
    /// changing, and a literal here would have let the change land silently.
    #[test]
    fn three_of_the_five_buffs_have_no_buff_level_today() {
        let buffs: Vec<u32> =
            FIRST_JOB_SKILLS.iter().filter(|s| s.kind == Kind::Buff).map(|s| s.id).collect();
        assert_eq!(buffs, vec![1_001_000, 2_001_000, 2_001_001, 3_001_000, 4_001_001]);

        // Positive control: `buff_level` can say yes. Nimble Feet is the one skill in the repo
        // whose numbers were confirmed on a client, and it is a beginner skill, not one of ours.
        assert!(net::buff::buff_level(1002, 1).is_some(), "the instrument answers");

        let unmodelled: Vec<u32> = buffs
            .iter()
            .copied()
            .filter(|id| net::buff::buff_level(*id, 1).is_none())
            .collect();
        assert_eq!(
            unmodelled,
            vec![1_001_000, 3_001_000, 4_001_001],
            "Iron Body, Focus and Dark Sight have no BuffLevel and no CTS bit. If this fails, \
             one of them has been modelled - update STATUS.md and both copies of the test plan."
        );
        // And the ten attacks: `buff_level` is not the path for them, so it must refuse all ten
        // rather than answering with something a caster would then send as a buff.
        for s in FIRST_JOB_SKILLS.iter().filter(|s| s.kind == Kind::Attack) {
            assert!(net::buff::buff_level(s.id, 1).is_none(), "{} is not a buff", s.name);
        }
        // Nor are the nine passives castable at all.
        for s in FIRST_JOB_SKILLS.iter().filter(|s| s.kind == Kind::Passive) {
            assert!(net::buff::buff_level(s.id, 1).is_none(), "{} is not cast", s.name);
        }
    }

    /// A missing file leaves the server exactly as it was, and says so.
    #[test]
    fn a_missing_file_degrades_to_saying_nothing() {
        let t = CombatTable::load(Path::new("no/such/skills.txt"));
        assert!(t.is_empty());
        assert_eq!(t.level_rows(), 0);
        assert!(!t.header_ok());
        assert_eq!(t.first_job_skills_loaded(), 0);
        assert_eq!(t.level(2_001_003, 1), None);
        assert!(t.banner().contains("NO USABLE HEADER"), "{}", t.banner());
        // The static table still answers - the classification does not depend on the file
        // being present, only its verification does.
        assert_eq!(server_obligation(1_001_001).map(|o| o.deduct_mp), Some(true));
        assert_eq!(first_job_skill_ids().len(), 24);
    }

    /// **The parser's own failure modes, on rows written here rather than found.**
    ///
    /// The checks above can only see cells the generator happens to produce today. Each row
    /// below states a rule this module claims, including the one `crate::skilltable` cannot
    /// state: **columns are resolved by name, so a header whose columns moved still reads
    /// correctly, and a header that lost one reads nothing at all.**
    #[test]
    fn columns_are_resolved_by_name_and_a_bad_cell_drops_its_row() {
        use std::io::Write;

        // A deliberately REORDERED header with an extra column in the middle. A reader that
        // hard-coded indices would decode every one of these cells as the wrong field.
        let header = "# name, skillId, junk, level, maxLevel, job, weapon, weapon2, weapon3, \
                      weapon4, type, psd, processtype, bulletCount, bulletConsume, mpCon, \
                      hpCon, damage, mad, attackCount, mobCount, time, cooltime";
        let names: Vec<&str> = header.trim_start_matches('#').split(',').map(str::trim).collect();
        let row = |pairs: &[(&str, &str)]| -> String {
            let mut f = vec![String::new(); names.len()];
            for (k, v) in pairs {
                let i = names.iter().position(|n| n == k).unwrap_or_else(|| panic!("{k}"));
                f[i] = (*v).to_string();
            }
            f.join(",")
        };

        let dir = std::env::temp_dir().join("maplecw-firstjob-test");
        std::fs::create_dir_all(&dir).unwrap();
        // Named per process: other agents share this machine and `cargo test -p world` can be
        // running in two of them at once.
        let path = dir.join(format!("skills-{}.txt", std::process::id()));
        let mut file = std::fs::File::create(&path).unwrap();
        let lines = [
            header.to_string(),
            "# a second comment line, skipped".to_string(),
            // The control: a good row in the shuffled layout.
            row(&[
                ("skillId", "700"),
                ("job", "300"),
                ("level", "1"),
                ("maxLevel", "2"),
                ("name", "Control"),
                ("type", "2"),
                ("processtype", "115"),
                ("weapon", "45"),
                ("weapon2", "46"),
                ("mpCon", "6"),
                ("damage", "160"),
                ("bulletCount", "1"),
                ("bulletConsume", "1"),
                ("attackCount", "1"),
                ("mobCount", "1"),
            ]),
            // Quoted cells still parse - 1 499 of them are quoted in the real archive.
            row(&[
                ("skillId", "700"),
                ("job", "300"),
                ("level", "2"),
                ("maxLevel", "2"),
                ("name", "Control"),
                ("type", "2"),
                ("processtype", "115"),
                ("weapon", "45"),
                ("weapon2", "46"),
                ("mpCon", "\"7\""),
                ("damage", "\"164\""),
                ("bulletConsume", "1"),
                ("bulletCount", "1"),
            ]),
            // A present-but-unreadable cell drops the WHOLE row rather than becoming `None`.
            row(&[
                ("skillId", "701"),
                ("job", "300"),
                ("level", "1"),
                ("maxLevel", "1"),
                ("name", "BadCell"),
                ("bulletConsume", "not-a-number"),
            ]),
            // An unreadable WEAPON cell drops the row too - a gate is not something to
            // half-read, and dropping the code silently would widen the gate.
            row(&[
                ("skillId", "702"),
                ("job", "300"),
                ("level", "1"),
                ("maxLevel", "1"),
                ("name", "BadWeapon"),
                ("weapon", "forty-five"),
            ]),
            // There is no level 0.
            row(&[
                ("skillId", "703"),
                ("job", "300"),
                ("level", "0"),
                ("maxLevel", "1"),
                ("name", "LevelZero"),
            ]),
            // A level above the allocation guard is refused rather than allocated for.
            row(&[
                ("skillId", "704"),
                ("job", "300"),
                ("level", "4000000000"),
                ("maxLevel", "1"),
                ("name", "Absurd"),
            ]),
            // A second row of skill 700 whose weapon gate DISAGREES with the first is counted,
            // not silently merged.
            row(&[
                ("skillId", "700"),
                ("job", "300"),
                ("level", "3"),
                ("maxLevel", "2"),
                ("name", "Control"),
                ("weapon", "33"),
            ]),
            // Wrong column count.
            "705,300,1,1".to_string(),
        ];
        writeln!(file, "{}", lines.join("\n")).unwrap();
        drop(file);

        let t = CombatTable::load(&path);

        // Positive control first: the shuffled header was resolved by name and the good rows
        // loaded with every field in the right place.
        assert!(t.header_ok());
        let s = t.get(700).expect("the control row loaded");
        assert_eq!(s.name, "Control");
        assert_eq!(s.job, 300);
        assert_eq!(s.wz_type, Some(2));
        assert_eq!(s.processtype, Some(115));
        assert!(!s.psd);
        assert_eq!(s.weapon_codes(), [45, 46]);
        assert_eq!(s.weapon_gate(), vec![WeaponClass::Bow, WeaponClass::Crossbow]);
        assert_eq!(s.levels_loaded(), 2);
        assert_eq!(s.level(1).unwrap().mp_con, Some(6));
        assert_eq!(s.level(1).unwrap().damage_percent, Some(160));
        assert_eq!(s.level(1).unwrap().bullet_consume, Some(1));
        assert_eq!(s.level(2).unwrap().mp_con, Some(7), "a quoted cell parses");
        assert_eq!(s.level(2).unwrap().damage_percent, Some(164), "so does a quoted damage");
        // An empty cell is absent, not zero.
        assert_eq!(s.level(1).unwrap().hp_con, None);
        assert_eq!(s.level(1).unwrap().time_seconds, None);
        assert_eq!(s.level(2).unwrap().attack_count, None);
        // And the rule reads the shuffled row correctly too.
        assert_eq!(s.kind(), Kind::Attack);

        // Each bad row was dropped whole, and counted.
        assert_eq!(t.get(701), None, "one unreadable cell drops the whole row");
        assert_eq!(t.get(702), None, "an unreadable weapon code drops the row");
        assert_eq!(t.get(703), None, "there is no level 0");
        assert_eq!(t.get(704), None, "and no level 4 000 000 000");
        assert_eq!(t.get(705), None, "nor a short row");
        assert_eq!(s.level(3), None, "the disagreeing weapon row was not merged in");
        assert_eq!(t.problems(), 6, "and the banner reports them: {}", t.banner());
        assert!(t.banner().contains("6 unreadable line(s)"), "{}", t.banner());
        assert!(t.banner().contains("0/24"), "{}", t.banner());

        std::fs::remove_file(&path).ok();
    }

    /// **A header missing one wanted column loads nothing at all**, rather than reading the
    /// remaining columns by position.
    #[test]
    fn a_header_without_every_wanted_column_refuses_to_read() {
        use std::io::Write;
        let dir = std::env::temp_dir().join("maplecw-firstjob-test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(format!("noheader-{}.txt", std::process::id()));
        let mut file = std::fs::File::create(&path).unwrap();
        // `bulletConsume` is absent from the header. Everything else is there.
        writeln!(
            file,
            "# skillId, job, level, maxLevel, name, type, psd, processtype, weapon, weapon2, \
             weapon3, weapon4, mpCon, hpCon, time, cooltime, damage, mad, attackCount, \
             mobCount, bulletCount"
        )
        .unwrap();
        writeln!(file, "700,300,1,1,Control,2,,115,45,46,,,6,,,,160,,1,1,1").unwrap();
        drop(file);

        let t = CombatTable::load(&path);
        assert!(!t.header_ok(), "the header does not name bulletConsume");
        assert!(t.is_empty(), "so nothing is read by position");
        assert!(t.banner().contains("NO USABLE HEADER"), "{}", t.banner());
        std::fs::remove_file(&path).ok();
    }

    /// The classification rule on its own, without a file, so its three arms are each exercised
    /// by an input constructed to hit them.
    #[test]
    fn the_rule_has_three_arms_and_each_one_is_reachable() {
        let base = ColumnEvidence {
            has_damage: false,
            has_mad: false,
            has_mp_con: false,
            wz_type: None,
            psd: false,
            processtype: None,
        };
        assert_eq!(classify(&ColumnEvidence { has_damage: true, ..base }), Kind::Attack);
        assert_eq!(classify(&ColumnEvidence { has_mad: true, ..base }), Kind::Attack);
        assert_eq!(classify(&ColumnEvidence { has_mp_con: true, ..base }), Kind::Buff);
        assert_eq!(classify(&base), Kind::Passive);
        // An attack that also costs MP is still an attack: `damage` wins over `mpCon`, which is
        // the case every one of the ten attacks actually is.
        assert_eq!(
            classify(&ColumnEvidence { has_damage: true, has_mp_con: true, ..base }),
            Kind::Attack
        );

        // `type` is reported, never consulted.
        assert_eq!(Kind::from_wz_type(1), Some(Kind::Attack));
        assert_eq!(Kind::from_wz_type(2), Some(Kind::Attack));
        assert_eq!(Kind::from_wz_type(10), Some(Kind::Buff));
        assert_eq!(Kind::from_wz_type(50), Some(Kind::Passive));
        assert_eq!(Kind::from_wz_type(41), None, "a type these 24 do not use");
        let disagree =
            ColumnEvidence { has_damage: true, wz_type: Some(50), ..base };
        assert_eq!(disagree.type_column_agrees(), Some(false), "and it can say so");
        assert_eq!(base.type_column_agrees(), None, "an empty cell has no opinion");
    }

    /// The columns the 2026-09-07 audit added load for the rows that carry them, and read
    /// absent - not zero - on the rows that do not.
    #[test]
    fn the_audit_columns_load_for_the_rows_that_carry_them() {
        let Some(t) = real_table() else { return };
        let l = |id: u32, lv: u32| t.level(id, lv).unwrap_or_else(|| panic!("{id} L{lv}"));
        assert!(l(3_101_005, 1).no_bullet_consume, "the hidden Arrow Bomb hit");
        assert!(!l(3_101_004, 1).no_bullet_consume, "and not Arrow Bomb itself");
        assert_eq!(l(3_111_002, 1).bullet_consume, Some(8), "Arrow Rain L1");
        assert_eq!(l(3_111_002, 30).bullet_consume, Some(4), "Arrow Rain L30");
        assert_eq!(l(2_111_005, 1).item_con, Some(4_006_000), "Spell Booster: a Magic Rock");
        assert_eq!(l(2_111_005, 1).item_con_no, Some(1));
        assert_eq!(l(2_311_001, 1).item_con_no, Some(2), "Mystic Door L1: two");
        assert_eq!(l(4_111_003, 1).money_con, Some(200), "Shadow Meso");
        assert_eq!(l(1_311_005, 1).indie_mhp_r, Some(10), "Hyper Body");
        assert_eq!(l(2_301_003, 1).indie_acc, Some(1), "Bless");
        assert_eq!(l(2_301_003, 20).indie_eva, Some(20));
        assert_eq!(l(1_101_002, 1).x, Some(-2), "Booster, signed");
        assert_eq!(l(4_101_002, 1).prop, Some(2), "Drain");
        assert_eq!(l(1_311_004, 1).y, Some(3), "Dragon Blood: every 3 s");
        let ps = l(1_001_001, 1);
        assert!(
            !ps.no_bullet_consume && ps.item_con.is_none() && ps.money_con.is_none() && ps.indie_mhp_r.is_none(),
            "Power Strike carries none of them"
        );
    }
}
