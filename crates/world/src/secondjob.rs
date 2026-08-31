//! Second job advancement at level 30 - the pure decision.
//!
//! Full working, with every enumeration and every named negative:
//! **`research/second-job.md`**.
//!
//! Labels are the project's: **[L]** read off this client's listing, its WZ or a capture,
//! **[D]** derived from two or more [L] facts, **[I]** inferred - a policy nothing on this
//! machine can confirm.
//!
//! # Nothing here authenticates
//!
//! As everywhere in this project, the channel socket carries no credentials. A second job
//! advancement is granted on the say-so of whoever holds the socket.
//!
//! # THIS MODULE IS NOT WIRED
//!
//! `crates/world/src/session/` belongs to the coordinator and was not touched. Nothing calls
//! anything in this file. `research/second-job.md` §9 is the "WIRE IT LIKE THIS", and saying
//! so out loud is `CLAUDE.md`'s *"built is not wired"* rule, which has caught two subsystems
//! that `STATUS.md` listed as done while nothing called them.
//!
//! It is the **pure decision**, the same shape as [`crate::jobs`]: given a character and the
//! NPC they clicked, may they advance and to what. It sends no packets, touches no database
//! and knows nothing about sessions.
//!
//! # The five facts this file exists to keep in one place
//!
//! 1. **The ten second jobs are 110 / 120 / 130 / 210 / 220 / 230 / 310 / 320 / 410 / 420.**
//!    **[L]** - the `job` column of `gm-handbook/skills.txt` (which `tools/dump_skills.py`
//!    derives from the image each skill was found in, `Skill_000.wz/<book>.img`) holds
//!    exactly **25** values - the beginner book `0`, the four first jobs, these ten, and ten
//!    third-job books `111`/`121`/`131`/`211`/`221`/`231`/`311`/`321`/`411`/`421`. There is
//!    no fifth branch and no Pirate.
//!
//! 2. **The advancement happens at the FIRST-job instructor**, 511 / 313 / 221 / 411 - the
//!    same four NPCs [`crate::jobs`] already knows. **[L]** from the fourth quest of each
//!    chain: `20003`'s `Check.1.npc` is **511**, not the examiner, and its own
//!    `QuestInfo.2` reads *"#p511# in #m10004000# welcomed me warmly, and I gained a new
//!    job"*. The `<Job> Job Instructor` NPCs - 514 / 319 / 227 / 424 - **run the test and
//!    advance nobody**; their own idle line says so: *"Do you want to undergo your 2nd job
//!    advancement as a warrior? Then go see Dances with Balrog in Perion."*
//!
//! 3. **Level 30, and you must already be the matching first job.** **[L]** - every one of
//!    the 16 chain quests carries `Check.0.lvmin = 30` and a single `Check.0.job.0` of
//!    `100`/`200`/`300`/`400`. See [`LEVEL_MINIMUM`].
//!
//! 4. **There is no stat prerequisite, and that is a measured negative rather than a failed
//!    search.** See *No stat prerequisite, and why that is measured*, below.
//!
//! 5. **Thirteen skills in these books are `invisible = 1` and must never be granted.** See
//!    [`HIDDEN_SKILLS`]. This is the one place where a straightforward reading of
//!    [`crate::skilltable`] is currently wrong, and it is wrong only at second job.
//!
//! # No stat prerequisite, and why that is measured
//!
//! [`crate::jobs::STAT_MINIMUM`] is **[I]** - the owner's 35, from a fan site, unverifiable
//! against this client and questioned once already. **This module deliberately does not add a
//! second one.**
//!
//! That is not laziness. Every `Check` key shape in all 322 quests was enumerated - 37
//! distinct shapes - and the counts reproduce `crate::jobs`'s earlier census exactly:
//! `lvmin` 406, `job` 104, `skill` 48, `item` 798. **[L]** Not one of the 37 is `str`, `dex`,
//! `int` or `luk`, and the four chain quests per branch use only `lvmin`, `job`, `npc`,
//! `quest`, `item`, `startscript`, `endscript` and `failscript`. The requirement is not in
//! `Quest.wz`, so inventing a number here would create a second constant that nothing on this
//! machine could ever contradict. `CLAUDE.md` asks for that to be said out loud rather than
//! quietly chosen.
//!
//! The same census produced one correction to `crate::jobs`'s doc block, which says there is
//! *"no `Act.<n>.job` key anywhere in the tree"*. There are **57** `Act.1.item.N.job` cells
//! plus two `jobEx`. They are a **branch bitmask on a reward item** - quest `10007` hands
//! glove `1082000` to mask `1`, `1082054` to `2`, `1082057` to `4`, `1082060` to `8`,
//! `1082063` to `16` - not a job change. **[L]** The conclusion it supported is unaffected:
//! no quest in this client changes a job, so the advancement is the server's to perform.
//!
//! # What this client does NOT supply
//!
//! * **The ten job names.** `String.wz` names the four *first* jobs (quest text: *"Swordsman,
//!   Magician, Archer, Rogue"*) and names none of the ten second jobs. That negative has a
//!   positive control: the same grep of `gm-handbook/questlines.txt` finds `Swordsman` 3
//!   times, `Magician` 15, `Archer` 3 and `Rogue` 3, and finds `Fighter`, `Spearman`,
//!   `Wizard`, `Cleric`, `Crossbowman`, `Assassin` and `Bandit` **zero** times. **[L]** The
//!   only second-job words the client owns are the **skill book** names in
//!   `String.wz/Skill.img/<book>/bookName`, so [`SecondJob::book_name`] is [L] and
//!   [`SecondJob::job_name`] carries its own provenance in [`SecondJob::name_source`].
//! * **Any HP or MP jump.** See [`MAX_HP_GAIN_POINTS`].
//!
//! # Every number carries its unit
//!
//! `CLAUDE.md`'s *"the unit, not the arithmetic"*. In this file:
//!
//! | field | unit |
//! |---|---|
//! | [`LEVEL_MINIMUM`] | character **levels** |
//! | [`QuestChain::marble_count_items`] | **items**, not stacks and not a percentage |
//! | [`QuestChain::exp_per_quest`] | **experience points**, flat, per quest, x4 for the chain |
//! | [`Grant::sp_total_owed`] | **skill points**, and a **TOTAL OWED**, never an increment |
//! | [`Grant::sp_pool_key`] | a job **TIER** `0..=10`, never a job id |
//! | [`MAX_HP_GAIN_POINTS`] / [`MAX_MP_GAIN_POINTS`] | flat **HP/MP points**, not percent |
//!
//! # `0` is a value
//!
//! [`MAX_HP_GAIN_POINTS`] is `0` because this server grants no HP on advancement, which is a
//! decision. It is not "unset". If it ever becomes non-zero it is one edit, in one place,
//! and [`Grant`] carries it explicitly rather than leaving the caller to remember.

use net::opcode::Character;

use crate::skillpoints::{self, Tier};

// ---------------------------------------------------------------------------------------
// The numbers
// ---------------------------------------------------------------------------------------

/// The level at which the second job advancement becomes available, in **character levels**.
///
/// **[L]**, and unlike [`crate::jobs::LEVEL_MINIMUM`] it needs no corroboration from anybody's
/// memory: all sixteen quests of the four advancement chains carry `Check.0.lvmin = 30`, and
/// `20000`'s own text opens *"Having reached level 30 and mastered the way of the
/// Swordsman..."*. From `gm-handbook/questlines.txt`.
///
/// It is the same number as [`crate::skillpoints::SECOND_JOB_LEVEL`], which is labelled
/// **[I]** there because when it was written nobody had looked. It is [L] now. The two are
/// asserted equal in the tests rather than aliased, so that this file states its own evidence
/// and a change to either one fails loudly.
pub const LEVEL_MINIMUM: u32 = 30;

/// **Flat max-HP points granted by the advancement itself. Zero, and that is a decision.**
///
/// Nothing in this client states a number. The whole of `Etc_000.wz` was enumerated - 100+
/// images - and the only job-related tables in it are `FreeJobChange.img` (a *cash* job change
/// gated at level 105) and `MakeCharInfo.img` (character creation). There is no per-job and no
/// per-advancement HP/MP table anywhere in `Etc.wz`, `String.wz` or `Skill.wz`. **[L]**, and
/// the two images named are the positive control that the enumeration can see a job-related
/// table when there is one.
///
/// So any non-zero value here would be a **second** unverifiable constant beside
/// [`crate::jobs::STAT_MINIMUM`], and `CLAUDE.md` asks for that to be refused rather than
/// quietly chosen. It is a named constant so that the owner supplying a number is one edit.
///
/// The first advancement grants none either: `session::gm::gm_job` sends the job and the SP
/// table and nothing else.
pub const MAX_HP_GAIN_POINTS: u32 = 0;

/// **Flat max-MP points granted by the advancement itself. Zero.** Same evidence and same
/// reasoning as [`MAX_HP_GAIN_POINTS`].
pub const MAX_MP_GAIN_POINTS: u32 = 0;

/// **The thirteen skills that exist in the archive and must never be granted.**
///
/// Every one of them has `invisible = 1` in `Skill.wz` and **no entry at all** in
/// `String.wz/Skill.img`, so the client has no name and no tooltip to draw for them.
///
/// This is an exact equivalence rather than a coincidence, enumerated over all 176 skills in
/// all 25 books: **13 skills lack a String.wz name, 13 carry `invisible = 1`, and the two sets
/// are identical** - zero unnamed-but-visible, zero invisible-but-named. **[L]**
///
/// Four of them are the hidden half of a named skill, named by that skill's own
/// `extraSkillInfo` node: `2101004` (Poison Breath) -> `2101005`, `3101004` -> `3101005`,
/// `3111001` -> `3111006`, `3211001` -> `3211006`. **[L]** Poison Breath is the clearest
/// case: its own level node carries **no `mad`, no `damage`, no `dot` and no `mastery`**,
/// while its tooltip says *"Basic Attack 50; Mastery level 1; 45% success rate; deals 10 Basic
/// Attack over 5 sec"* - and `2101005` carries `mad 50`, `mastery 1`, `prop 45`, `dot 10`,
/// `dotTime 5` exactly. The other nine are referenced from nowhere in `Skill.wz`; the
/// discriminator that covers all thirteen is `invisible`, not `extraSkillInfo`.
///
/// # Why this constant is here rather than in `crate::skilltable`
///
/// **`skilltable::book()` would hand these out.** Its filter is `job != 0 && may_learn(...)`
/// and it does not parse the `invisible` column at all - `SkillTable` has no field for it. At
/// first job that is harmless, because none of the 24 first-job skills is invisible. At second
/// job `book(110)` returns **10** entries where the player may only ever have **8**.
///
/// `crates/world/src/skilltable.rs` is not this module's file to edit, so the correction lives
/// here as [`SecondJob::grantable_skills`] and is reported. `research/second-job.md` §7.
pub const HIDDEN_SKILLS: [u32; 13] = [
    1101006, 1101007, // Fighter
    1201006, 1201007, // Page
    1301006, 1301007, // Spearman
    2101005, // Wizard (Fire/Poison) - Poison Breath's damage half
    2111006, // (third job, Adv. Fire & Poison - listed for completeness of the negative)
    3101005, 3101006, // Hunter
    3111006, // (third job, Path of the Ranger)
    3201005, // Crossbowman
    3211006, // (third job, Sniper's Scope)
];

// ---------------------------------------------------------------------------------------
// The table
// ---------------------------------------------------------------------------------------

/// Where a [`SecondJob::job_name`] came from, so a reader can tell a measurement from a
/// convention without leaving the type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameSource {
    /// **[D]** - the name is the leading word of the client's own
    /// `String.wz/Skill.img/<book>/bookName`, e.g. `"Fighter"` from `"Fighter Techniques"`.
    /// The tests assert `book_name.starts_with(job_name)` for every entry that claims this.
    BookName,
    /// **[I]** - the conventional MapleStory name. **This client contains it nowhere**: not in
    /// `String.wz`, not in quest text, not as an ASCII or UTF-16 string in `MapleStory.exe`.
    /// Only the three Magician second jobs are in this state, because their book names
    /// (*"Fire & Poison Basics"*, *"Ice & Lightning Basics"*, *"Holy Magic Basics"*) describe
    /// the element rather than the job.
    NotInThisClient,
}

/// One second job: the id, the client's word for its skill book, and what is in it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecondJob {
    /// The job id the advancement grants, which is also the `Skill.wz` book id. **[L]**
    pub job: u16,
    /// What to *call* it. Read [`SecondJob::name_source`] before quoting this at a player.
    pub job_name: &'static str,
    /// Where [`SecondJob::job_name`] came from.
    pub name_source: NameSource,
    /// `String.wz/Skill.img/<job>/bookName`, verbatim. **[L]** - this is the only text this
    /// client has for a second job, and it is the book rather than the job.
    pub book_name: &'static str,
    /// **Every** skill id in the book, hidden ones included, in ascending order. **[L]** from
    /// the `job` column of `gm-handbook/skills.txt`.
    ///
    /// Use [`SecondJob::grantable_skills`] for what a player may actually be given.
    pub skill_book: &'static [u32],
}

impl SecondJob {
    /// The skills a player may be granted: [`SecondJob::skill_book`] minus [`HIDDEN_SKILLS`].
    ///
    /// The filter is [`is_hidden`] rather than a second list written beside the first, so the
    /// book and the grant cannot disagree about what exists - `CLAUDE.md`'s Heena lesson.
    pub fn grantable_skills(&self) -> impl Iterator<Item = u32> + '_ {
        self.skill_book.iter().copied().filter(|id| !is_hidden(*id))
    }

    /// How many of [`SecondJob::skill_book`] a player may actually have.
    pub fn grantable_count(&self) -> usize {
        self.grantable_skills().count()
    }
}

/// The four-quest chain the client's own data lays out for one branch.
///
/// **This is carried as data and is deliberately NOT enforced by [`advancement_for`]** - see
/// [`REQUIRE_QUEST_CHAIN`]. It is here because a test run needs the item ids and the maps, and
/// because the chain is the only place this client states the level and the job gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuestChain {
    /// The four quest ids in order: *"The <Branch>'s Next Journey"*, *"Finding the
    /// Instructor"*, *"Test of Qualification"*, *"Proof of Qualification"*. **[L]**
    pub quests: [u32; 4],
    /// The letter the first-job instructor hands over on quest 2, `Act.0.item.0.id`. It is
    /// taken back (`Act.1.item.0.count = -1`) when the examiner accepts it. **[L]**
    pub letter_item: u32,
    /// The Black Marble the examiner's test asks for, `Check.1.item.0.id` of quest 3. **[L]**
    pub marble_item: u32,
    /// How many of [`QuestChain::marble_item`], in **items**. `30` for all four branches.
    /// **[L]**
    pub marble_count_items: u32,
    /// The proof the examiner hands over on quest 4, carried back to the instructor. **[L]**
    pub proof_item: u32,
    /// `Act.1.exp` in **experience points**, flat. The same `3150` on all sixteen quests, so
    /// a completed chain is `4 x 3150 = 12 600`. **[L]**
    pub exp_per_quest: u32,
}

/// **The hidden field the examiner sends you into**, and the two mob templates in it.
///
/// This is the piece `research/second-job.md` said did not exist - *"the Test of Qualification
/// hidden field and its `q20002s` script do not exist, so the client's own route to the
/// advancement is not walkable"*. The field was in the client the whole time. It is named,
/// it has footholds, it has mobs, and **three independent nodes of the archive agree on which
/// branch owns it**. `research/second-job-fields.md` is the working.
///
/// # Why it cannot be reached on foot, and why that is the point
///
/// Each of these four maps has **exactly one portal**, `sp`, the spawn point - against 31 for
/// Perion and 8 for the examiner's own map. **[L]** So there is no door in and no door out.
/// A player only ever arrives here because a server put them here, which is what makes it a
/// test rather than a place, and it is why [`Branch::test_field`] has to carry an exit: a
/// character standing in one with no way to leave is stuck.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TestField {
    /// The hidden map. **[L]** `gm-handbook/fields.txt` and `footholds.txt` both have it, so
    /// this server can load it like any other field.
    pub map_id: u32,
    /// `String.wz/Map.img`. **[L]**
    pub map_name: &'static str,
    /// **The NPC standing inside, and the only way out.** 800003 / 800004 / 800005 / 800006,
    /// each literally named `"<Branch> Job Instructor"` - the same name as the examiner and a
    /// different template. **[L]** `gm-handbook/npcs.txt` places each exactly once, in its own
    /// dungeon.
    ///
    /// `crate::jobs::first_job_at` already refuses all four, and so does [`branch_at`]. They
    /// advance nobody; they open the door.
    pub warden_npc: u32,
    /// The two mob templates that spawn here, in template order. **[L]**
    ///
    /// These are **dedicated clones**, not the ordinary mobs they are named after: the Fire
    /// Boar in Perion is template 30 at level 32, and the one in here is **800016 at level
    /// 30** - the same level as the advancement itself, and the same for all eight. That
    /// uniformity is what says these were built for this test rather than borrowed for it.
    pub mobs: [u32; 2],
    /// Where a player is put when they leave, and it is **the client's own answer**:
    /// `Map.wz/<map>/info/returnMap` and `forcedReturn` are both the examiner's map, on all
    /// four. **[L]** An ordinary map's `forcedReturn` is `999999999`; these four name a real
    /// map, which is the archive saying out loud that this is somewhere you get ejected from.
    pub exit_map_id: u32,
}

impl TestField {
    /// Whether this mob template is one of the two that spawn in this field.
    pub fn has_mob(&self, template: u32) -> bool {
        self.mobs.contains(&template)
    }
}

/// One branch's whole second advancement: who to click, who tests, and what may be chosen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Branch {
    /// The first job a character must already hold. `Check.0.job.0` of all four chain quests,
    /// and there is exactly one `job.N` entry - not a list. **[L]**
    pub from_job: u16,
    /// What the client calls a character of [`Branch::from_job`]. **[L]** - the same four
    /// names [`crate::jobs`] carries, from `Quest.wz`.
    pub from_job_name: &'static str,

    /// **The NPC that performs the advancement**: 511 / 313 / 221 / 411, the *first*-job
    /// instructor. `Check.1.npc` of the fourth quest. **[L]**
    pub instructor_npc: u32,
    /// `String.wz/Npc.img/<template>/name`. **[L]**
    pub instructor_name: &'static str,
    /// The map the instructor is actually on, from `Map.wz`'s `life` nodes via
    /// `gm-handbook/npcs.txt`. **[L]** Each of the four is placed exactly once in the whole
    /// archive.
    ///
    /// **This is not the town**, exactly as at first job - and the client's own quest text
    /// says the town: `20003` sends the player to `#m10004000#` (Perion) while 511 stands on
    /// `10004003`.
    pub instructor_map_id: u32,
    /// [`Branch::instructor_map_id`]'s name from `gm-handbook/maps.txt`. **[L]**
    pub instructor_map_name: &'static str,
    /// The town the quest text names. Kept beside the map id so the difference is visible.
    /// **[L]**
    pub town_id: u32,

    /// **The NPC that runs the test and advances nobody**: 514 / 319 / 227 / 424. **[L]**
    pub examiner_npc: u32,
    /// `String.wz/Npc.img/<template>/name` - all four are literally `"<Branch> Job
    /// Instructor"`. **[L]**
    pub examiner_name: &'static str,
    /// Where the examiner stands, from `gm-handbook/npcs.txt`. **[L]** These are the four maps
    /// a test run has to reach, and none of them is a town.
    pub examiner_map_id: u32,
    /// [`Branch::examiner_map_id`]'s name from `gm-handbook/maps.txt`. **[L]**
    pub examiner_map_name: &'static str,

    /// **The hidden field the examiner warps into.** See [`TestField`].
    pub test_field: TestField,

    /// The client's own four-quest route. Data, not a gate - see [`REQUIRE_QUEST_CHAIN`].
    pub chain: QuestChain,

    /// What this branch may advance **into**, in job-id order. Two for Bowman and Thief,
    /// three for Warrior and Magician. **[L]** from the `Skill.wz` book ids.
    pub choices: &'static [SecondJob],
}

/// **Whether [`advancement_for`] requires the four-quest chain to have been completed.**
///
/// `false`, and this is **[I]** - a policy, stated here rather than left implicit.
///
/// The chain's third quest, *Test of Qualification*, is gated on `Check.0.startscript =
/// q20002s` and hands out its Black Marbles inside a hidden field that this server does not
/// implement. Requiring it would make the second advancement untestable, which is the one
/// thing a feature in this repo may not be.
///
/// The chain is still carried as data ([`QuestChain`]) so that a caller which *does* have
/// quest state can enforce it in one place, and so a test run has the item ids.
pub const REQUIRE_QUEST_CHAIN: bool = false;

// --- the Warrior branch ----------------------------------------------------------------

const FIGHTER_BOOK: [u32; 10] = [
    1100000, 1100001, 1101000, 1101001, 1101002, 1101003, 1101004, 1101005, 1101006, 1101007,
];
const PAGE_BOOK: [u32; 10] = [
    1200000, 1200001, 1201000, 1201001, 1201002, 1201003, 1201004, 1201005, 1201006, 1201007,
];
const SPEARMAN_BOOK: [u32; 10] = [
    1300000, 1300001, 1301000, 1301001, 1301002, 1301003, 1301004, 1301005, 1301006, 1301007,
];

// --- the Magician branch ---------------------------------------------------------------

const FIRE_POISON_BOOK: [u32; 7] =
    [2100000, 2101000, 2101001, 2101002, 2101003, 2101004, 2101005];
const ICE_LIGHTNING_BOOK: [u32; 6] = [2200000, 2201000, 2201001, 2201002, 2201003, 2201004];
const CLERIC_BOOK: [u32; 6] = [2300000, 2301000, 2301001, 2301002, 2301003, 2301004];

// --- the Bowman branch -----------------------------------------------------------------

/// **`3101002` is absent, and that is the client's data rather than a typo.** In the classic
/// tree Power Knock-Back is `3101002`/`3201002`, a *second*-job skill. This build ships it as
/// **`3001003`, in the first-job Archer book**, and leaves both second-job slots empty.
/// **[L]** - `crate::firstjob` already carries it as one of the 24.
const HUNTER_BOOK: [u32; 8] =
    [3100000, 3100001, 3101000, 3101001, 3101003, 3101004, 3101005, 3101006];
/// `3201002` is absent for the same reason as `3101002` - see [`HUNTER_BOOK`].
const CROSSBOWMAN_BOOK: [u32; 7] =
    [3200000, 3200001, 3201000, 3201001, 3201003, 3201004, 3201005];

// --- the Thief branch ------------------------------------------------------------------

const ASSASSIN_BOOK: [u32; 6] = [4100000, 4100001, 4100002, 4101000, 4101001, 4101002];
const BANDIT_BOOK: [u32; 6] = [4200000, 4200001, 4201000, 4201001, 4201002, 4201003];

const WARRIOR_CHOICES: [SecondJob; 3] = [
    SecondJob {
        job: 110,
        job_name: "Fighter",
        name_source: NameSource::BookName,
        book_name: "Fighter Techniques",
        skill_book: &FIGHTER_BOOK,
    },
    SecondJob {
        job: 120,
        job_name: "Page",
        name_source: NameSource::BookName,
        book_name: "Page's Path",
        skill_book: &PAGE_BOOK,
    },
    SecondJob {
        job: 130,
        job_name: "Spearman",
        name_source: NameSource::BookName,
        book_name: "Spearman Techniques",
        skill_book: &SPEARMAN_BOOK,
    },
];

const MAGICIAN_CHOICES: [SecondJob; 3] = [
    SecondJob {
        job: 210,
        job_name: "Wizard (Fire/Poison)",
        name_source: NameSource::NotInThisClient,
        book_name: "Fire & Poison Basics",
        skill_book: &FIRE_POISON_BOOK,
    },
    SecondJob {
        job: 220,
        job_name: "Wizard (Ice/Lightning)",
        name_source: NameSource::NotInThisClient,
        book_name: "Ice & Lightning Basics",
        skill_book: &ICE_LIGHTNING_BOOK,
    },
    SecondJob {
        job: 230,
        job_name: "Cleric",
        name_source: NameSource::NotInThisClient,
        book_name: "Holy Magic Basics",
        skill_book: &CLERIC_BOOK,
    },
];

const BOWMAN_CHOICES: [SecondJob; 2] = [
    SecondJob {
        job: 310,
        job_name: "Hunter",
        name_source: NameSource::BookName,
        book_name: "Hunter's Guide",
        skill_book: &HUNTER_BOOK,
    },
    SecondJob {
        job: 320,
        job_name: "Crossbowman",
        name_source: NameSource::BookName,
        book_name: "Crossbowman Guide",
        skill_book: &CROSSBOWMAN_BOOK,
    },
];

const THIEF_CHOICES: [SecondJob; 2] = [
    SecondJob {
        job: 410,
        job_name: "Assassin",
        name_source: NameSource::BookName,
        book_name: "Assassin Skills",
        skill_book: &ASSASSIN_BOOK,
    },
    SecondJob {
        job: 420,
        job_name: "Bandit",
        name_source: NameSource::BookName,
        book_name: "Bandit's Tricks",
        skill_book: &BANDIT_BOOK,
    },
];

/// The four second job advancements, ordered by [`Branch::from_job`].
///
/// Every field is **[L]**; the provenance of each is on the field itself.
pub const BRANCHES: [Branch; 4] = [
    Branch {
        from_job: 100,
        from_job_name: "Swordsman",
        instructor_npc: 511,
        instructor_name: "Dances with Balrog",
        instructor_map_id: 10004003,
        instructor_map_name: "Warriors' Sanctuary",
        town_id: 10004000,
        examiner_npc: 514,
        examiner_name: "Warrior Job Instructor",
        examiner_map_id: 10004023,
        examiner_map_name: "West Rocky Mountain IV",
        test_field: TestField {
            map_id: 80001300,
            map_name: "Warrior's Rocky Mountain",
            warden_npc: 800006,
            mobs: [800016, 800017],   // Fire Boar, Lupin
            exit_map_id: 10004023,
        },
        chain: QuestChain {
            quests: [20000, 20001, 20002, 20003],
            letter_item: 4031013,
            marble_item: 4031017,
            marble_count_items: 30,
            proof_item: 4031018,
            exp_per_quest: 3150,
        },
        choices: &WARRIOR_CHOICES,
    },
    Branch {
        from_job: 200,
        from_job_name: "Magician",
        instructor_npc: 313,
        instructor_name: "Grendel the Really Old",
        instructor_map_id: 10002003,
        instructor_map_name: "Magic Library",
        town_id: 10002000,
        examiner_npc: 319,
        examiner_name: "Magician Job Instructor",
        examiner_map_id: 10002070,
        examiner_map_name: "The Forest North of Ellinia",
        test_field: TestField {
            map_id: 80001100,
            map_name: "Magician's Tree Dungeon",
            warden_npc: 800004,
            mobs: [800012, 800013],   // Curse Eye, Horny Mushroom
            exit_map_id: 10002070,
        },
        chain: QuestChain {
            quests: [20100, 20101, 20102, 20103],
            letter_item: 4031014,
            marble_item: 4031019,
            marble_count_items: 30,
            proof_item: 4031020,
            exp_per_quest: 3150,
        },
        choices: &MAGICIAN_CHOICES,
    },
    Branch {
        from_job: 300,
        from_job_name: "Archer",
        instructor_npc: 221,
        instructor_name: "Athena Pierce",
        instructor_map_id: 10001051,
        instructor_map_name: "Bowman Instructional School",
        town_id: 10001000,
        examiner_npc: 227,
        examiner_name: "Bowman Job Instructor",
        examiner_map_id: 10001090,
        examiner_map_name: "The Road to the Dungeon",
        test_field: TestField {
            map_id: 80001000,
            map_name: "Ant Tunnel For Bowman",
            warden_npc: 800003,
            mobs: [800010, 800011],   // Evil Eye, Zombie Mushroom
            exit_map_id: 10001090,
        },
        chain: QuestChain {
            quests: [20200, 20201, 20202, 20203],
            letter_item: 4031015,
            marble_item: 4031021,
            marble_count_items: 30,
            proof_item: 4031022,
            exp_per_quest: 3150,
        },
        choices: &BOWMAN_CHOICES,
    },
    Branch {
        from_job: 400,
        from_job_name: "Rogue",
        instructor_npc: 411,
        instructor_name: "Dark Lord",
        instructor_map_id: 10003003,
        instructor_map_name: "Thieves' Hideout",
        town_id: 10003000,
        examiner_npc: 424,
        examiner_name: "Thief Job Instructor",
        examiner_map_id: 10003080,
        examiner_map_name: "Construction Site North of Kerning City",
        test_field: TestField {
            map_id: 80001200,
            map_name: "Thief's Construction Site",
            warden_npc: 800005,
            mobs: [800014, 800015],   // Cold Eye, Blue Mushroom
            exit_map_id: 10003080,
        },
        chain: QuestChain {
            quests: [20300, 20301, 20302, 20303],
            letter_item: 4031016,
            marble_item: 4031023,
            marble_count_items: 30,
            proof_item: 4031024,
            exp_per_quest: 3150,
        },
        choices: &THIEF_CHOICES,
    },
];

// ---------------------------------------------------------------------------------------
// Lookups
// ---------------------------------------------------------------------------------------

/// The branch whose **instructor** is this NPC, i.e. the NPC that actually advances.
///
/// **The four examiners - 514, 319, 227, 424 - are deliberately absent**, for the mirror of
/// the reason [`crate::jobs::first_job_at`] excludes them: they test, and the client's own
/// text sends the player back to the first-job instructor afterwards. Use
/// [`branch_examined_by`] for them.
pub fn branch_at(npc_template: u32) -> Option<&'static Branch> {
    BRANCHES.iter().find(|b| b.instructor_npc == npc_template)
}

/// The branch whose **examiner** is this NPC. Advances nobody; this exists so a caller can
/// say something true when the player clicks one.
pub fn branch_examined_by(npc_template: u32) -> Option<&'static Branch> {
    BRANCHES.iter().find(|b| b.examiner_npc == npc_template)
}

/// The branch whose **test field** this map is, or `None` for any ordinary map.
///
/// Four maps in the whole client answer `Some` here. The pairing is not a guess: three
/// independent nodes of the archive agree on it, and they were read separately -
/// `research/second-job-fields.md` section 2.
pub fn branch_of_test_field(map_id: u32) -> Option<&'static Branch> {
    BRANCHES.iter().find(|b| b.test_field.map_id == map_id)
}

/// The branch whose **warden** this NPC is - the one standing inside the test field.
///
/// 800003 / 800004 / 800005 / 800006. They share their `name` string with the four examiners
/// and are a different template, which is exactly the trap `crate::jobs` documents: a name is
/// not an id. [`branch_at`] and [`branch_examined_by`] both refuse all four.
pub fn branch_warded_by(npc_template: u32) -> Option<&'static Branch> {
    BRANCHES.iter().find(|b| b.test_field.warden_npc == npc_template)
}

/// Whether this item is one of the four Dark Marbles.
///
/// All four are literally named `Dark Marble` in `String.wz/Item.img`; only the id says which
/// branch's test it counts for. **[L]**
pub fn is_marble(item_id: u32) -> bool {
    BRANCHES.iter().any(|b| b.chain.marble_item == item_id)
}

/// Whether this item is one of the four `The Proof of a Hero`.
pub fn is_proof(item_id: u32) -> bool {
    BRANCHES.iter().any(|b| b.chain.proof_item == item_id)
}

/// **The Dark Marble a kill should drop**, given the mob and the map it died on - or `None`.
///
/// Both halves of the question matter, and the second one is not decoration:
///
/// * `800012`, `800013`, `800014`, `800016` and `800017` spawn **only** in their own test
///   field, so for those the map is redundant.
/// * `800010` (Evil Eye) and `800015` (Blue Mushroom) also spawn on **80003500**, and
///   `800011` (Zombie Mushroom) also spawns on **10006160, Precipice of Darkness**. **[L]**
///   Without the map test, an ordinary player grinding Precipice of Darkness would collect
///   the Bowman's test marbles by accident, and three of the eight templates would leak.
///
/// So the rule is *this mob, in its own field*, and the leak is closed by construction rather
/// than by a list of exceptions that someone has to remember to extend.
pub fn marble_for_kill(mob_template: u32, map_id: u32) -> Option<u32> {
    let branch = branch_of_test_field(map_id)?;
    branch
        .test_field
        .has_mob(mob_template)
        .then_some(branch.chain.marble_item)
}

/// **Whether a test mob drops its marble every time.**
///
/// `true`, and it is **[I]** - a policy, written down here rather than chosen quietly in a
/// data file. Nothing in this client carries a drop rate for anything: `data/drops.txt` says
/// so in its own header (*"THE CHANCES ARE OURS"*), and the rates in it for these eight mobs
/// came from a fan site at **6%**, which is 30 marbles in roughly 500 kills.
///
/// Two reasons to make it certain instead. The test asks for 30 and the field holds 26-30
/// mobs, so at 100% the test is a full clear and a bit - which is a test. And a feature that
/// takes hours to reach its first observation is a feature this project cannot check, which
/// `CLAUDE.md` calls the one thing a feature here may not be.
///
/// If the owner wants it rarer this is the single place to change it.
pub const MARBLE_DROP_IS_CERTAIN: bool = true;

/// The branch a first job belongs to, or `None` if `job` is not one of the four.
pub fn branch_from_job(job: u16) -> Option<&'static Branch> {
    BRANCHES.iter().find(|b| b.from_job == job)
}

/// The second job with this id, whichever branch it is in.
pub fn second_job(job: u16) -> Option<&'static SecondJob> {
    BRANCHES.iter().flat_map(|b| b.choices.iter()).find(|c| c.job == job)
}

/// Whether a job id is one of the ten second jobs.
pub fn is_second_job(job: u16) -> bool {
    second_job(job).is_some()
}

/// Whether a skill id is one the client draws no name for and no player may be granted.
/// See [`HIDDEN_SKILLS`].
pub fn is_hidden(skill_id: u32) -> bool {
    HIDDEN_SKILLS.contains(&skill_id)
}

// ---------------------------------------------------------------------------------------
// The decision
// ---------------------------------------------------------------------------------------

/// What clicking an NPC should do about a *second* advancement.
///
/// Every arm carries the numbers a refusal sentence needs, so the caller never re-derives them
/// and the two cannot drift apart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Advancement {
    /// This NPC has nothing to do with a second advancement. Fall through to whatever the
    /// caller does for an ordinary NPC.
    NotAnInstructor,
    /// The examiner. **Advances nobody** - it runs the *Test of Qualification* and sends the
    /// player back to `instructor`. The client's own idle line for this NPC says exactly that.
    Examines { branch: &'static Branch },
    /// Job `0`. **This is [`crate::jobs`]'s business, not a refusal** - the caller must hand
    /// the click to `jobs::advancement_for` rather than say no, because the same four NPCs
    /// give the *first* advancement.
    StillABeginner,
    /// A first-job character at the wrong instructor - a Magician clicking Dances with Balrog.
    /// `Check.0.job.0` is a single value per chain, so this is the client's own gate. **[L]**
    WrongBranch { have_job: u16, branch_wants: u16 },
    /// Already second job or beyond. One-way, exactly as the first advancement is.
    AlreadyAdvanced { job: u16 },
    /// Below [`LEVEL_MINIMUM`].
    TooLowLevel { level: u32, needed: u32 },
    /// Level and job pass, but the character is not carrying `The Proof of a Hero`.
    ///
    /// **This is the client's own gate**, not one this server invented: quest `20003`'s
    /// `Check.1.item.0` is one `proof_item` handed in at the instructor. See
    /// [`REQUIRE_PROOF_ITEM`], which is what decides whether this arm can ever be produced.
    NeedsProof { branch: &'static Branch },
    /// Everything passes. The player must now **pick one of** `branch.choices`; feed the
    /// choice back through [`advancement_to`].
    Choose { branch: &'static Branch },
}

/// May this character take a second advancement by talking to this NPC?
///
/// **Pure.** No database, no packets, no clock, and no quest state - see
/// [`REQUIRE_QUEST_CHAIN`].
///
/// The order of the checks is the order the refusals should be *said* in. A beginner is
/// handed back to [`crate::jobs`] before anything else, because at these four NPCs a beginner
/// is asking a different question. Level is checked before the branch mismatch only where the
/// branch matches; a level-12 Magician at Dances with Balrog is told about the branch, because
/// "come back at 30" would be advice that never becomes true.
pub fn advancement_for(chr: &Character, npc_template: u32) -> Advancement {
    if let Some(branch) = branch_examined_by(npc_template) {
        return Advancement::Examines { branch };
    }
    let Some(branch) = branch_at(npc_template) else {
        return Advancement::NotAnInstructor;
    };
    if chr.job == 0 {
        return Advancement::StillABeginner;
    }
    if chr.job != branch.from_job {
        // Anything that is not this branch's first job. Two readings, and they get different
        // sentences: a character already past first job cannot advance again anywhere, while
        // one in a different branch is simply at the wrong NPC.
        if !crate::jobs::is_first_job(chr.job) {
            return Advancement::AlreadyAdvanced { job: chr.job };
        }
        return Advancement::WrongBranch { have_job: chr.job, branch_wants: branch.from_job };
    }
    if chr.level < LEVEL_MINIMUM {
        return Advancement::TooLowLevel { level: chr.level, needed: LEVEL_MINIMUM };
    }
    Advancement::Choose { branch }
}

/// The character picked `chosen_job`. May they have it?
///
/// Returns the [`Grant`] to apply, or the [`Advancement`] that explains the refusal. A choice
/// from another branch is a [`Advancement::WrongBranch`] rather than a panic, because the
/// choice arrives off a socket.
pub fn advancement_to(
    chr: &Character,
    npc_template: u32,
    chosen_job: u16,
) -> Result<Grant, Advancement> {
    let outcome = advancement_for(chr, npc_template);
    let Advancement::Choose { branch } = outcome else {
        return Err(outcome);
    };
    if !branch.choices.iter().any(|c| c.job == chosen_job) {
        return Err(Advancement::WrongBranch {
            have_job: chr.job,
            branch_wants: branch.from_job,
        });
    }
    grant(chosen_job, chr.level).ok_or(Advancement::NotAnInstructor)
}

/// The sentence to put in the refusal box, or `None` when there is nothing to refuse.
///
/// **A refusal is still an answer.** `CLAUDE.md`'s first expensive rule is that an unanswered
/// packet freezes the client's entire UI; a script box that says no keeps the conversation
/// alive and ends it cleanly.
///
/// [`Advancement::StillABeginner`] returns `None` **on purpose**: it is not a refusal, it is a
/// hand-off to [`crate::jobs::refusal`], and a caller that printed something here would say no
/// to a level-10 beginner who is entitled to a first advancement.
pub fn refusal(chr: &Character, npc_template: u32) -> Option<String> {
    refusal_for(&advancement_for(chr, npc_template))
}

/// **Whether the advancement requires `The Proof of a Hero` in the bag.**
///
/// `true`. This is the client's own `Check.1.item.0` on quest `20003` - **[L]** - and it is
/// the gate that makes the whole chain mean something: without it a level-30 Swordsman could
/// walk past the examiner, the test field and the marbles and take the advancement anyway.
///
/// It is a **narrower** requirement than [`REQUIRE_QUEST_CHAIN`] and deliberately so. The
/// quest rows can be missing, refused or out of order for reasons that have nothing to do
/// with the player; the item is a fact about the bag, it is granted by exactly one place
/// ([`TestStep::Pass`]), and `!item` can put one there for a test run without pretending a
/// quest happened. `CLAUDE.md` asks for a gate whose answer is actually consulted, and this
/// one is - see [`advancement_for_holding`].
pub const REQUIRE_PROOF_ITEM: bool = true;

/// [`advancement_for`], plus the proof-of-a-hero check the client's own quest carries.
///
/// Split from [`advancement_for`] rather than folded into it, because the pure decision has
/// no business knowing about a bag and every other caller of it wants the version that does
/// not. When [`REQUIRE_PROOF_ITEM`] is `false` this is exactly [`advancement_for`].
pub fn advancement_for_holding(
    chr: &Character,
    npc_template: u32,
    holds_proof: bool,
) -> Advancement {
    match advancement_for(chr, npc_template) {
        Advancement::Choose { branch } if REQUIRE_PROOF_ITEM && !holds_proof => {
            Advancement::NeedsProof { branch }
        }
        other => other,
    }
}

/// The sentence for an [`Advancement`] that has already been decided.
///
/// Exists so [`refusal`] and [`advancement_for_holding`]'s caller cannot end up with two
/// different wordings for the same arm - which is how the first-job and second-job refusals
/// would have drifted, since each would have written its own.
pub fn refusal_for(outcome: &Advancement) -> Option<String> {
    match *outcome {
        Advancement::NotAnInstructor
        | Advancement::StillABeginner
        | Advancement::Choose { .. } => None,
        Advancement::NeedsProof { branch } => Some(format!(
            "You carry no proof. Take {}'s test in {} and bring me {}.",
            branch.examiner_name, branch.examiner_map_name, PROOF_ITEM_NAME
        )),
        Advancement::Examines { branch } => Some(format!(
            "I only administer the test. Go and see {} in {} first.",
            branch.instructor_name, branch.instructor_map_name
        )),
        Advancement::WrongBranch { have_job, branch_wants } => Some(format!(
            "Your path is not mine to guide. I teach job {branch_wants}; you walk job {have_job}."
        )),
        Advancement::AlreadyAdvanced { job } => {
            Some(format!("You have already taken that step. Job {job} cannot be undone."))
        }
        Advancement::TooLowLevel { level, needed } => Some(format!(
            "Come back when you have reached Level {needed}. You are only Level {level}."
        )),
    }
}

// ---------------------------------------------------------------------------------------
// The test: the examiner, the hidden field, and the warden at the door
// ---------------------------------------------------------------------------------------

/// What `String.wz/Item.img` calls all four marbles. **[L]**
pub const MARBLE_ITEM_NAME: &str = "Dark Marble";

/// What `String.wz/Item.img` calls all four proofs. **[L]**
pub const PROOF_ITEM_NAME: &str = "The Proof of a Hero";

/// What clicking one of the four **examiners** should do.
///
/// Pure, like everything else here: it is told how many marbles and whether the proof is held,
/// and it decides. The caller does the bag arithmetic and the warp.
///
/// **Every effect hangs off one arm.** Only [`TestStep::Enter`] carries a map and only
/// [`TestStep::Pass`] carries items to move, so a refusal cannot warp anybody and cannot pay
/// anybody - there is nothing on the other arms to read. That is `CLAUDE.md`'s Heena rule
/// applied in the type rather than in the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestStep {
    /// The examiner will not test this character. The sentence is the whole answer, and there
    /// **is** one - an unanswered click freezes the client's UI.
    Refused(String),
    /// Send them in. The line goes out as a notice **before** the field change, because a
    /// script box sent with or just before a `SetField` is torn down silently by field entry -
    /// the same ordering `jobguide::Step::Ride` uses and the same one observed working.
    Enter { branch: &'static Branch, field: TestField, line: String },
    /// The marbles are in the bag. Take `take_marbles` of `marble_item`, hand over
    /// `proof_item`, and send them back to the instructor.
    Pass {
        branch: &'static Branch,
        marble_item: u32,
        take_marbles: u32,
        proof_item: u32,
        line: String,
    },
    /// The proof is already held. Nothing to take, nothing to give, and **nothing to warp** -
    /// re-entering the field for a test that is already passed would only strand them.
    AlreadyPassed { branch: &'static Branch, line: String },
}

/// Whether this character may take this branch's test at all, or the sentence saying why not.
///
/// Reuses [`advancement_for`] against the branch's **instructor** rather than restating the
/// rules, because the test and the advancement are gated on exactly the same two facts -
/// `Check.0.lvmin` and `Check.0.job.0`, identical on all sixteen chain quests. **[L]** Two
/// copies of one predicate is how one of them gets missed.
fn may_take_the_test(chr: &Character, branch: &'static Branch) -> Result<(), String> {
    match advancement_for(chr, branch.instructor_npc) {
        Advancement::Choose { .. } => Ok(()),
        // Not a refusal in `refusal_for`'s eyes - it is a hand-off to `crate::jobs`. Here it
        // is a real answer, because the examiner is not where a beginner's story starts.
        Advancement::StillABeginner => Err(format!(
            "You have no job to advance from. See {} in {} first.",
            branch.instructor_name, branch.instructor_map_name
        )),
        other => {
            Err(refusal_for(&other).unwrap_or_else(|| String::from("I cannot test you.")))
        }
    }
}

/// The examiner's whole decision. `None` for any NPC that is not one of the four.
pub fn test_step(
    chr: &Character,
    npc_template: u32,
    marbles_held: u32,
    holds_proof: bool,
) -> Option<TestStep> {
    let branch = branch_examined_by(npc_template)?;
    if let Err(why) = may_take_the_test(chr, branch) {
        return Some(TestStep::Refused(why));
    }
    if holds_proof {
        return Some(TestStep::AlreadyPassed {
            branch,
            line: format!(
                "You already carry {PROOF_ITEM_NAME}. Take it to {} in {} - the advancement is                  theirs to give, not mine.",
                branch.instructor_name, branch.instructor_map_name
            ),
        });
    }
    let wanted = branch.chain.marble_count_items;
    if marbles_held >= wanted {
        return Some(TestStep::Pass {
            branch,
            marble_item: branch.chain.marble_item,
            take_marbles: wanted,
            proof_item: branch.chain.proof_item,
            line: format!(
                "{wanted} {MARBLE_ITEM_NAME}s. You have passed. Take {PROOF_ITEM_NAME} to {} in                  {} and your new path is yours.",
                branch.instructor_name, branch.instructor_map_name
            ),
        });
    }
    Some(TestStep::Enter {
        branch,
        field: branch.test_field,
        line: format!(
            "Into {} with you. Bring me {wanted} {MARBLE_ITEM_NAME}s - you have {marbles_held}.              Talk to the instructor inside when you want to come out.",
            branch.test_field.map_name
        ),
    })
}

/// What clicking the **warden** - the NPC inside the test field - should do.
///
/// There is only one arm, and that is the point: these four maps have exactly one portal each
/// and it is the spawn point, so the warden is the only door. A branch here that could refuse
/// would be a branch that strands a player in a map with no exit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WardenStep {
    pub branch: &'static Branch,
    /// The examiner's map - the client's own `returnMap` and `forcedReturn` for this field.
    pub to_map_id: u32,
    pub line: String,
}

/// The warden's decision. `None` for any NPC that is not one of the four.
///
/// **It never refuses**, and `marbles_held` only changes the wording. See [`WardenStep`].
pub fn warden_step(npc_template: u32, marbles_held: u32) -> Option<WardenStep> {
    let branch = branch_warded_by(npc_template)?;
    let wanted = branch.chain.marble_count_items;
    let line = if marbles_held >= wanted {
        format!(
            "{marbles_held} {MARBLE_ITEM_NAME}s - that is enough. I will send you back to {};              the {} is waiting there.",
            branch.examiner_map_name, branch.examiner_name
        )
    } else {
        format!(
            "You have {marbles_held} of {wanted} {MARBLE_ITEM_NAME}s. Leave whenever you like -              I will put you back in {}.",
            branch.examiner_map_name
        )
    };
    Some(WardenStep { branch, to_map_id: branch.test_field.exit_map_id, line })
}

// ---------------------------------------------------------------------------------------
// The choice, on screen
// ---------------------------------------------------------------------------------------

/// The client's markup for a line break inside a script box.
///
/// Two characters, a backslash and an `n` - **not** an escape. `crate::taxi::LINE_BREAK` is
/// the same two characters for the same reason, and all 33 authored menus in
/// `gm-handbook/questlines.txt` are written this way. **[L]**
pub const LINE_BREAK: &str = "\\n";

/// Where [`Conversation::path`](crate::session) parks while the choice box is on screen.
///
/// Namespaced so that `crate::taxi::is_taxi_path` and this cannot both claim the same reply.
/// A type-6 body carries no speaker, so **the path is the only thing that says who asked** -
/// that is `crate::taxi::MENU_PATH`'s argument and it applies here word for word.
pub const MENU_PATH: &str = "secondjob.menu";

/// Whether a conversation path is this module's menu.
pub fn is_menu_path(path: &str) -> bool {
    path == MENU_PATH
}

/// The whole choice box: what the instructor asks, then one line per second job.
///
/// **The names are not all the client's.** Seven of the ten are the leading word of the
/// skill book's own `bookName` and three are ours - see [`NameSource`] - so a Magician's box
/// says `Wizard (Fire/Poison)` on a line this project wrote. The box says so rather than
/// pretending: an unnamed job printed as a bare id would be worse, and a made-up name printed
/// as though it came from Nexon would be worse still.
pub fn menu_text(branch: &Branch) -> String {
    let mut out = format!(
        "You have earned this. Which path will you walk as a {}?",
        branch.from_job_name
    );
    out.push_str(LINE_BREAK);
    for (selection, choice) in branch.choices.iter().enumerate() {
        out.push_str(LINE_BREAK);
        out.push_str(&format!("#d#L{selection}# {}#l#k", choice.job_name));
    }
    out
}

/// The second job at menu position `selection`, or `None` if the number is not on the list.
///
/// The selection arrives off a socket, so an out-of-range one is an ordinary answer to give
/// rather than something to panic about - and [`advancement_to`] re-checks the branch anyway.
pub fn choice_at(branch: &Branch, selection: u32) -> Option<&'static SecondJob> {
    branch.choices.get(usize::try_from(selection).ok()?)
}

// ---------------------------------------------------------------------------------------
// What the advancement grants
// ---------------------------------------------------------------------------------------

/// Everything the server owes a character the moment the advancement is accepted.
///
/// **Every field is idempotent or constant.** [`Grant::sp_total_owed`] is a *total owed*
/// rather than an increment, which is [`crate::skillpoints`]'s whole design and the reason
/// re-sending this cannot double-pay. The HP/MP fields are zero and named, not absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grant {
    /// The job id to store and to put in the `0x007C` `StatChange`.
    pub job: u16,
    /// What to call it on screen. Read [`Grant::job_name_source`] first.
    pub job_name: &'static str,
    /// Where [`Grant::job_name`] came from.
    pub job_name_source: NameSource,
    /// Which pool the points go in.
    pub sp_tier: Tier,
    /// **The key the wire wants: a job TIER `0..=10`, never a job id.**
    ///
    /// `net::stats::tier_for_job`, from `FUN_140286E90`. Every second job ends in `0`, so this
    /// is `2` for all ten. **[D]** from that function's arithmetic. `FUN_1402CB030` returns `0`
    /// for any key above 10, so a job id here would read an empty pool and grey the `+` button
    /// with nothing on screen to say why.
    pub sp_pool_key: u8,
    /// **Skill points owed IN TOTAL to [`Grant::sp_tier`] at this level** - not an increment.
    /// `crate::skillpoints::entitlement(Tier::Second, level)`.
    pub sp_total_owed: u32,
    /// Flat max-HP points to add. Always [`MAX_HP_GAIN_POINTS`]; carried so a caller reads it
    /// rather than remembers it.
    pub max_hp_gain_points: u32,
    /// Flat max-MP points to add. Always [`MAX_MP_GAIN_POINTS`].
    pub max_mp_gain_points: u32,
    /// **Whether the `StatChange` SP encoding forks between the old job and the new one.**
    ///
    /// `false` for every first -> second advancement in this client, because
    /// `net::opcode::uses_extended_sp` accepts `x10`..`x12`, `x20`..`x22` and `x30`..`x32` in
    /// the 1xx and 2xx branches and `x10`..`x12`, `x20`..`x22` in 3xx/4xx. **[D]** - and it is
    /// carried rather than assumed, because a job like `101` genuinely is on the other side
    /// and sending the wrong shape desynchronises the whole packet rather than merely losing
    /// the points.
    pub sp_encoding_changes: bool,
}

/// What advancing to `job` at `level` grants, or `None` if `job` is not a second job.
///
/// Pure, and safe to call repeatedly: [`Grant::sp_total_owed`] is a total, so applying it
/// twice grants nothing the second time.
pub fn grant(job: u16, level: u32) -> Option<Grant> {
    let second = second_job(job)?;
    let from = branch_from_job(job / 100 * 100)?.from_job;
    Some(Grant {
        job: second.job,
        job_name: second.job_name,
        job_name_source: second.name_source,
        sp_tier: Tier::Second,
        sp_pool_key: net::stats::tier_for_job(job),
        sp_total_owed: skillpoints::entitlement(Tier::Second, level),
        max_hp_gain_points: MAX_HP_GAIN_POINTS,
        max_mp_gain_points: MAX_MP_GAIN_POINTS,
        sp_encoding_changes: crate::jobs::sp_encoding_changes(from, job),
    })
}

// ---------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};
    use std::path::Path;

    fn character(level: u32, job: u16) -> Character {
        Character {
            // 200 is FIRST_CHARACTER_ID. A record renumbered from 1 has bitten this project
            // before, so the fixtures here use a real id rather than a small one.
            id: 200,
            level,
            job,
            strength: 4,
            dexterity: 4,
            intelligence: 4,
            luck: 4,
            ..Character::default()
        }
    }

    /// Every row of `gm-handbook/skills.txt` as `(skillId, jobBook, invisible)`, or `None` on
    /// a clean checkout where `gm-handbook/` has not been generated.
    ///
    /// This reads the file directly rather than through [`crate::skilltable`] because
    /// `SkillTable` does not parse the `invisible` column - which is the whole point of
    /// [`HIDDEN_SKILLS`].
    fn real_skill_rows() -> Option<Vec<(u32, u16, bool)>> {
        let path = Path::new("../../gm-handbook/skills.txt");
        if !path.exists() {
            return None;
        }
        let text = std::fs::read_to_string(path).ok()?;
        const COLUMNS: usize = 98;
        const COL_ID: usize = 0;
        const COL_JOB: usize = 1;
        const COL_INVISIBLE: usize = 24;
        let mut out = Vec::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split(',').map(str::trim).collect();
            if f.len() != COLUMNS {
                continue;
            }
            let (Ok(id), Ok(job)) = (f[COL_ID].parse::<u32>(), f[COL_JOB].parse::<u16>()) else {
                continue;
            };
            out.push((id, job, f[COL_INVISIBLE] == "1"));
        }
        Some(out)
    }

    /// `gm-handbook/questlines.txt` as `(questId, node, path) -> value`, or `None` on a clean
    /// checkout.
    fn real_questlines() -> Option<BTreeMap<(u32, String, String), String>> {
        let path = Path::new("../../gm-handbook/questlines.txt");
        if !path.exists() {
            return None;
        }
        let text = std::fs::read_to_string(path).ok()?;
        let mut out = BTreeMap::new();
        for line in text.lines() {
            if line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() != 4 {
                continue;
            }
            let Ok(id) = f[0].parse::<u32>() else { continue };
            out.insert((id, f[1].to_string(), f[2].to_string()), f[3].to_string());
        }
        Some(out)
    }

    /// **The numbers the whole feature rests on.** If a later edit renumbers a job or moves an
    /// instructor, this fails here rather than on the owner's screen.
    #[test]
    fn the_ten_second_jobs_and_their_eight_npcs() {
        let jobs: Vec<u16> =
            BRANCHES.iter().flat_map(|b| b.choices.iter()).map(|c| c.job).collect();
        assert_eq!(jobs, vec![110, 120, 130, 210, 220, 230, 310, 320, 410, 420]);

        let instructors: Vec<u32> = BRANCHES.iter().map(|b| b.instructor_npc).collect();
        assert_eq!(instructors, vec![511, 313, 221, 411], "the FIRST-job instructors advance");
        let examiners: Vec<u32> = BRANCHES.iter().map(|b| b.examiner_npc).collect();
        assert_eq!(examiners, vec![514, 319, 227, 424], "the Job Instructors only test");

        // Every choice is in its own branch's decade, and the branch table agrees with the
        // lookup - so `grant`'s `job / 100 * 100` can never reach the wrong branch.
        for b in BRANCHES {
            for c in b.choices {
                assert_eq!(c.job / 100, b.from_job / 100, "{} is in branch {}", c.job, b.from_job);
                assert_eq!(branch_from_job(b.from_job).unwrap().from_job, b.from_job);
                assert!(is_second_job(c.job));
            }
        }
        // And the ids that are NOT second jobs, including the third-job books next door.
        for j in [0u16, 100, 200, 300, 400, 111, 121, 131, 211, 221, 231, 311, 321, 411, 421, 500] {
            assert!(!is_second_job(j), "job {j} is not a SECOND job");
        }
    }

    /// **The skill books are exactly what the client ships.**
    ///
    /// `gm-handbook/` is generated and gitignored, so this is a no-op on a clean checkout -
    /// which is exactly the shape of check `CLAUDE.md` warns can be silently vacuous. It
    /// therefore asserts a **positive control first**: the file must contain the whole archive
    /// before any of its absences are believed.
    #[test]
    fn every_skill_book_is_exactly_the_client_s_own() {
        let Some(rows) = real_skill_rows() else { return };
        let ids: BTreeSet<u32> = rows.iter().map(|r| r.0).collect();
        // Positive control: the file loaded and holds the whole archive.
        assert_eq!(ids.len(), 176, "the generated file holds all 176 skills");
        assert!(ids.contains(&2001003), "Magic Claw, the strongest control this repo has");

        let mut by_book: BTreeMap<u16, BTreeSet<u32>> = BTreeMap::new();
        for (id, job, _) in &rows {
            by_book.entry(*job).or_default().insert(*id);
        }
        // The 25 books, enumerated rather than assumed - this is what says there is no fifth
        // branch and no Pirate.
        let books: Vec<u16> = by_book.keys().copied().collect();
        assert_eq!(
            books,
            vec![
                0, 100, 110, 111, 120, 121, 130, 131, 200, 210, 211, 220, 221, 230, 231, 300,
                310, 311, 320, 321, 400, 410, 411, 420, 421
            ]
        );

        for b in BRANCHES {
            for c in b.choices {
                let want: BTreeSet<u32> = c.skill_book.iter().copied().collect();
                let have = by_book.get(&c.job).unwrap_or_else(|| panic!("book {}", c.job));
                assert_eq!(&want, have, "{} ({}) skill book", c.job_name, c.job);
                // Ascending and duplicate-free, so `grantable_skills` is a stable order.
                assert_eq!(c.skill_book.len(), want.len(), "{} has no duplicate id", c.job);
                assert!(c.skill_book.windows(2).all(|w| w[0] < w[1]), "{} is sorted", c.job);
            }
        }

        // **The two gaps are the client's data, not a transcription slip.** Power Knock-Back
        // is `3001003` in the FIRST-job Archer book here, where the classic tree puts it at
        // 3101002/3201002.
        assert!(!by_book[&310].contains(&3101002), "310 has no 3101002");
        assert!(!by_book[&320].contains(&3201002), "320 has no 3201002");
        assert!(by_book[&300].contains(&3001003), "and 300 has Power Knockback instead");
    }

    /// **`HIDDEN_SKILLS` is exactly the set the client marks `invisible = 1`.**
    ///
    /// Asserted in both directions over the whole archive, so it cannot pass by naming a
    /// subset - which is the failure mode `CLAUDE.md` calls "enumerate before you filter".
    #[test]
    fn the_hidden_skills_are_exactly_the_invisible_ones() {
        let Some(rows) = real_skill_rows() else { return };
        let mut invisible: BTreeSet<u32> = BTreeSet::new();
        let mut visible: BTreeSet<u32> = BTreeSet::new();
        for (id, _, inv) in &rows {
            if *inv { invisible.insert(*id); } else { visible.insert(*id); }
        }
        // Positive control: the column was actually read and it is not empty everywhere.
        assert_eq!(invisible.len(), 13, "13 invisible skills in the archive");
        assert!(visible.len() > 150, "and the rest are visible: {}", visible.len());
        assert!(invisible.is_disjoint(&visible), "no skill is both");

        let declared: BTreeSet<u32> = HIDDEN_SKILLS.iter().copied().collect();
        assert_eq!(declared, invisible, "HIDDEN_SKILLS must be the invisible set exactly");
        for id in HIDDEN_SKILLS {
            assert!(is_hidden(id));
        }
        for id in [2001003u32, 1001001, 3001003, 4001003, 1002] {
            assert!(!is_hidden(id), "skill {id} is a real, named skill");
        }
    }

    /// **The grantable book is the book minus the hidden ones, and the counts are stated.**
    ///
    /// This is the correction to `skilltable::book()`, which does not parse `invisible` and
    /// would hand a Fighter ten skills where the player may only ever have eight.
    #[test]
    fn grantable_skills_exclude_the_ones_the_client_will_not_draw() {
        let expected: [(u16, usize, usize); 10] = [
            // (job, all in book, grantable)
            (110, 10, 8),
            (120, 10, 8),
            (130, 10, 8),
            (210, 7, 6),
            (220, 6, 6),
            (230, 6, 6),
            (310, 8, 6),
            (320, 7, 6),
            (410, 6, 6),
            (420, 6, 6),
        ];
        let mut total_grantable = 0usize;
        for (job, all, grantable) in expected {
            let c = second_job(job).unwrap_or_else(|| panic!("job {job}"));
            assert_eq!(c.skill_book.len(), all, "{} book size", c.job_name);
            assert_eq!(c.grantable_count(), grantable, "{} grantable", c.job_name);
            assert!(
                c.grantable_skills().all(|id| !is_hidden(id)),
                "{} grants nothing hidden",
                c.job_name
            );
            total_grantable += grantable;
        }
        assert_eq!(total_grantable, 66, "66 grantable second-job skills across the ten books");
        // The control that this filter does something: the three Warrior books really do lose
        // two each, and the Thief books lose none.
        assert_eq!(second_job(110).unwrap().grantable_count() + 2, 10);
        assert_eq!(second_job(410).unwrap().grantable_count(), 6);
    }

    /// **The quest chains, checked against the client's own generated quest dump.**
    ///
    /// This is the [L] behind [`LEVEL_MINIMUM`], behind the branch gate and behind every
    /// number in [`QuestChain`]. A no-op on a clean checkout, with a positive control first.
    #[test]
    fn the_quest_chains_are_what_this_client_ships() {
        let Some(q) = real_questlines() else { return };
        // Positive control: the dump loaded and holds a quest everybody already knows.
        assert!(
            q.contains_key(&(10001, "QuestInfo".into(), "name".into())),
            "the questline dump loaded"
        );

        for b in BRANCHES {
            let [q0, q1, q2, q3] = b.chain.quests;
            let get = |id: u32, node: &str, path: &str| {
                q.get(&(id, node.to_string(), path.to_string())).cloned()
            };

            // Level 30 and the first-job gate, on every quest of the chain.
            for id in b.chain.quests {
                assert_eq!(
                    get(id, "Check", "0.lvmin").as_deref(),
                    Some(LEVEL_MINIMUM.to_string().as_str()),
                    "quest {id} lvmin"
                );
                assert_eq!(
                    get(id, "Check", "0.job.0").as_deref(),
                    Some(b.from_job.to_string().as_str()),
                    "quest {id} job gate"
                );
                // A single `job.N`, not a list - which is what makes WrongBranch the client's
                // own rule rather than ours.
                assert_eq!(get(id, "Check", "0.job.1"), None, "quest {id} gates on one job");
                assert_eq!(
                    get(id, "Act", "1.exp").as_deref(),
                    Some(b.chain.exp_per_quest.to_string().as_str()),
                    "quest {id} exp"
                );
            }

            // Who each step is turned in to. The last one is the INSTRUCTOR, not the examiner:
            // this is the fact the whole module is shaped around.
            assert_eq!(
                get(q0, "Check", "1.npc").as_deref(),
                Some(b.instructor_npc.to_string().as_str())
            );
            assert_eq!(
                get(q1, "Check", "1.npc").as_deref(),
                Some(b.examiner_npc.to_string().as_str()),
                "the letter goes to the examiner"
            );
            assert_eq!(
                get(q2, "Check", "1.npc").as_deref(),
                Some(b.examiner_npc.to_string().as_str()),
                "the test is turned in to the examiner"
            );
            assert_eq!(
                get(q3, "Check", "1.npc").as_deref(),
                Some(b.instructor_npc.to_string().as_str()),
                "THE ADVANCEMENT ITSELF IS AT {} ({}), not at the examiner",
                b.instructor_npc,
                b.instructor_name
            );

            // The three items, and the count in ITEMS.
            assert_eq!(
                get(q1, "Act", "0.item.0.id").as_deref(),
                Some(b.chain.letter_item.to_string().as_str())
            );
            assert_eq!(
                get(q2, "Check", "1.item.0.id").as_deref(),
                Some(b.chain.marble_item.to_string().as_str())
            );
            assert_eq!(
                get(q2, "Check", "1.item.0.count").as_deref(),
                Some(b.chain.marble_count_items.to_string().as_str())
            );
            assert_eq!(
                get(q3, "Act", "0.item.0.id").as_deref(),
                Some(b.chain.proof_item.to_string().as_str())
            );

            // The chain is a chain: each quest names the next and requires the last completed.
            assert_eq!(get(q0, "Act", "1.nextQuest").as_deref(), Some(q1.to_string().as_str()));
            assert_eq!(get(q1, "Act", "1.nextQuest").as_deref(), Some(q2.to_string().as_str()));
            assert_eq!(get(q2, "Act", "1.nextQuest").as_deref(), Some(q3.to_string().as_str()));
            assert_eq!(get(q3, "Act", "1.nextQuest"), None, "and it ends there");
            assert_eq!(get(q3, "Check", "0.quest.0.id").as_deref(), Some(q2.to_string().as_str()));
            assert_eq!(get(q3, "Check", "0.quest.0.state").as_deref(), Some("2"), "completed");

            // **No quest in this chain changes a job, and none asks for a stat.** The negative
            // that says the advancement is the server's to perform.
            for id in b.chain.quests {
                for stat in ["str", "dex", "int", "luk"] {
                    assert_eq!(get(id, "Check", &format!("0.{stat}")), None, "quest {id} {stat}");
                }
                for n in 0..4 {
                    assert_eq!(get(id, "Act", &format!("{n}.job")), None, "quest {id} Act {n}");
                }
            }
        }
    }

    /// **No second advancement changes the SP encoding**, so the combined `0x007C` that
    /// `session::gm::job_change_reply` builds stays correct across all ten.
    ///
    /// With the control that makes the assertion mean something: `101` genuinely is on the
    /// other side of the fork.
    #[test]
    fn no_second_advancement_forks_the_sp_encoding() {
        for b in BRANCHES {
            assert!(net::opcode::uses_extended_sp(b.from_job), "job {} extended", b.from_job);
            for c in b.choices {
                assert!(net::opcode::uses_extended_sp(c.job), "job {} extended", c.job);
                assert!(
                    !crate::jobs::sp_encoding_changes(b.from_job, c.job),
                    "{} -> {} must not fork",
                    b.from_job,
                    c.job
                );
                assert!(!grant(c.job, 30).unwrap().sp_encoding_changes);
            }
        }
        // The control: this function does not always answer false.
        assert!(!net::opcode::uses_extended_sp(101));
        assert!(crate::jobs::sp_encoding_changes(100, 101), "100 -> 101 DOES fork");
    }

    /// **The SP pool key is a TIER, and it is 2 for every second job.**
    ///
    /// A job id here would read an empty pool - `FUN_1402CB030` returns 0 above 10 - and grey
    /// the `+` button with nothing on screen to say why.
    #[test]
    fn the_sp_pool_key_is_tier_two_for_all_ten() {
        for b in BRANCHES {
            for c in b.choices {
                let g = grant(c.job, 30).unwrap();
                assert_eq!(g.sp_pool_key, 2, "{} pool key", c.job_name);
                assert_eq!(g.sp_tier, Tier::Second);
                assert!(u16::from(g.sp_pool_key) < c.job, "a tier, not a job id");
            }
        }
        // Controls, so "2" is not simply what this function always says.
        assert_eq!(net::stats::tier_for_job(0), 0, "beginner");
        assert_eq!(net::stats::tier_for_job(100), 1, "first job");
        assert_eq!(net::stats::tier_for_job(111), 3, "third job");
    }

    /// **The grant is a TOTAL OWED, so applying it twice grants nothing the second time.**
    ///
    /// This is [`crate::skillpoints`]'s property and it is asserted here because a second
    /// reader of that number is a second chance to turn it into an increment.
    #[test]
    fn the_sp_grant_is_a_total_and_re_granting_is_idempotent() {
        assert_eq!(grant(110, 30).unwrap().sp_total_owed, 1, "the advancement itself");
        assert_eq!(grant(110, 31).unwrap().sp_total_owed, 4, "1 + 3");
        assert_eq!(grant(110, 40).unwrap().sp_total_owed, 1 + 3 * 10);
        // Below the level it is zero rather than negative or absent.
        assert_eq!(grant(110, 29).unwrap().sp_total_owed, 0);
        // Idempotent by construction: the same level always gives the same total, so a caller
        // that stores "granted" and subtracts can never double-pay.
        for _ in 0..50 {
            assert_eq!(grant(230, 37).unwrap().sp_total_owed, skillpoints::entitlement(Tier::Second, 37));
            assert_eq!(skillpoints::top_up(Tier::Second, 37, grant(230, 37).unwrap().sp_total_owed), 0);
        }
        // And this file's own level agrees with `skillpoints`, stated as an assertion rather
        // than an alias so a change to either fails loudly.
        assert_eq!(LEVEL_MINIMUM, skillpoints::SECOND_JOB_LEVEL);
        assert_eq!(LEVEL_MINIMUM, 30);
    }

    /// The HP/MP jump is zero, and the constant exists so that is visible rather than absent.
    #[test]
    fn the_hp_and_mp_gain_are_zero_and_named() {
        assert_eq!(MAX_HP_GAIN_POINTS, 0, "[L] negative: this client states no number");
        assert_eq!(MAX_MP_GAIN_POINTS, 0);
        for b in BRANCHES {
            for c in b.choices {
                let g = grant(c.job, 30).unwrap();
                assert_eq!(g.max_hp_gain_points, MAX_HP_GAIN_POINTS);
                assert_eq!(g.max_mp_gain_points, MAX_MP_GAIN_POINTS);
            }
        }
    }

    /// **A job name that claims to come from the client's book name really does**, and the
    /// three that admit they do not, do not. This is what stops [`NameSource`] being set by
    /// habit.
    #[test]
    fn job_names_carry_honest_provenance() {
        let mut from_book = 0;
        let mut invented = 0;
        for b in BRANCHES {
            for c in b.choices {
                match c.name_source {
                    NameSource::BookName => {
                        assert!(
                            c.book_name.starts_with(c.job_name),
                            "{:?} claims to come from {:?} and does not",
                            c.job_name,
                            c.book_name
                        );
                        from_book += 1;
                    }
                    NameSource::NotInThisClient => {
                        assert!(
                            !c.book_name.starts_with(c.job_name),
                            "{:?} IS in {:?} - mark it BookName",
                            c.job_name,
                            c.book_name
                        );
                        invented += 1;
                    }
                }
                assert!(!c.book_name.is_empty());
            }
        }
        assert_eq!(from_book, 7, "seven names the client half-supplies");
        assert_eq!(invented, 3, "and the three Magician jobs it names nowhere at all");
        // Those three, named, so a later edit cannot quietly promote one.
        for j in [210u16, 220, 230] {
            assert_eq!(second_job(j).unwrap().name_source, NameSource::NotInThisClient);
        }
    }

    /// The examiners advance nobody, and the instructors are on maps that are not their towns.
    #[test]
    fn the_examiner_tests_and_the_instructor_advances() {
        for b in BRANCHES.iter() {
            assert!(branch_at(b.examiner_npc).is_none(), "{} must not advance", b.examiner_name);
            assert_eq!(branch_examined_by(b.examiner_npc).unwrap().from_job, b.from_job);
            assert!(branch_examined_by(b.instructor_npc).is_none());

            // A fully-qualified character at the examiner is told where to go, not advanced.
            let chr = character(50, b.from_job);
            assert_eq!(advancement_for(&chr, b.examiner_npc), Advancement::Examines { branch: b });
            assert!(refusal(&chr, b.examiner_npc).unwrap().contains(b.instructor_name));

            // Neither NPC stands in the town, and they do not stand together either.
            assert_ne!(b.instructor_map_id, b.town_id, "{} is one map inside", b.instructor_name);
            assert_ne!(b.examiner_map_id, b.town_id);
            assert_ne!(b.examiner_map_id, b.instructor_map_id);
        }
        // The four maps a test run has to reach, spelled out - getting one wrong costs a
        // manual launch.
        assert_eq!(branch_at(511).unwrap().examiner_map_id, 10004023);
        assert_eq!(branch_at(313).unwrap().examiner_map_id, 10002070);
        assert_eq!(branch_at(221).unwrap().examiner_map_id, 10001090);
        assert_eq!(branch_at(411).unwrap().examiner_map_id, 10003080);
    }

    /// **A beginner is handed back to `crate::jobs`, not refused.**
    ///
    /// The same four NPCs give both advancements, so a module that said "no" here would break
    /// the first one. The cross-module control is in the assertion: `jobs` must want the
    /// click that this module declines.
    #[test]
    fn a_beginner_at_an_instructor_is_jobs_rs_business() {
        let mut chr = character(10, 0);
        chr.strength = crate::jobs::STAT_MINIMUM;
        for b in BRANCHES {
            assert_eq!(advancement_for(&chr, b.instructor_npc), Advancement::StillABeginner);
            assert_eq!(refusal(&chr, b.instructor_npc), None, "and says nothing, so jobs can");
            assert_ne!(
                crate::jobs::advancement_for(&chr, b.instructor_npc),
                crate::jobs::Advancement::NotAnInstructor,
                "jobs::advancement_for must want template {}",
                b.instructor_npc
            );
        }
    }

    /// A first-job character at the wrong instructor is told about the branch, at every level.
    #[test]
    fn the_wrong_branch_is_refused_before_the_level() {
        for b in BRANCHES {
            for other in BRANCHES {
                if other.from_job == b.from_job {
                    continue;
                }
                for level in [1u32, 29, 30, 99] {
                    let chr = character(level, other.from_job);
                    assert_eq!(
                        advancement_for(&chr, b.instructor_npc),
                        Advancement::WrongBranch {
                            have_job: other.from_job,
                            branch_wants: b.from_job
                        },
                        "job {} at npc {} level {level}",
                        other.from_job,
                        b.instructor_npc
                    );
                    // "Come back at 30" would be advice that never becomes true here.
                    let s = refusal(&chr, b.instructor_npc).unwrap();
                    assert!(!s.contains("Level 30"), "{s}");
                }
            }
        }
    }

    /// Level is refused after the branch matches, and the boundary is exact.
    #[test]
    fn level_thirty_is_the_boundary() {
        let b = &BRANCHES[0];
        for level in 0..LEVEL_MINIMUM {
            assert_eq!(
                advancement_for(&character(level, 100), b.instructor_npc),
                Advancement::TooLowLevel { level, needed: LEVEL_MINIMUM },
                "level {level}"
            );
        }
        assert_eq!(
            advancement_for(&character(LEVEL_MINIMUM, 100), b.instructor_npc),
            Advancement::Choose { branch: b }
        );
        assert!(refusal(&character(29, 100), 511).unwrap().contains("Level 30"));
        assert_eq!(refusal(&character(30, 100), 511), None);
    }

    /// **There is no stat prerequisite**, and this asserts it rather than leaving it to the
    /// doc block. A level-30 Swordsman with the floor in every stat may advance.
    #[test]
    fn there_is_no_stat_prerequisite() {
        let chr = character(LEVEL_MINIMUM, 100);
        assert_eq!(chr.strength, 4, "floor STR");
        assert!(chr.strength < crate::jobs::STAT_MINIMUM, "and below the FIRST job's [I] 35");
        assert!(matches!(advancement_for(&chr, 511), Advancement::Choose { .. }));
        assert!(advancement_to(&chr, 511, 110).is_ok());
    }

    /// Advancement is one-way: a second-job character is refused everywhere.
    #[test]
    fn an_advanced_character_is_refused_by_every_instructor() {
        for job in [110u16, 120, 130, 210, 220, 230, 310, 320, 410, 420, 111, 231] {
            let chr = character(90, job);
            for b in BRANCHES {
                assert_eq!(
                    advancement_for(&chr, b.instructor_npc),
                    Advancement::AlreadyAdvanced { job },
                    "job {job} at {}",
                    b.instructor_name
                );
            }
            assert!(refusal(&chr, 511).unwrap().contains(&job.to_string()));
        }
    }

    /// A choice from another branch is refused rather than granted, because the choice arrives
    /// off a socket and nothing upstream has to have validated it.
    #[test]
    fn a_choice_from_another_branch_is_refused() {
        let chr = character(40, 100);
        assert_eq!(advancement_to(&chr, 511, 110).unwrap().job, 110);
        assert_eq!(advancement_to(&chr, 511, 120).unwrap().job, 120);
        assert_eq!(advancement_to(&chr, 511, 130).unwrap().job, 130);
        for bad in [210u16, 220, 230, 310, 320, 410, 420, 111, 100, 0, 9999] {
            assert_eq!(
                advancement_to(&chr, 511, bad),
                Err(Advancement::WrongBranch { have_job: 100, branch_wants: 100 }),
                "a Swordsman must not become job {bad} at 511"
            );
        }
        // And the refusals from `advancement_for` come straight through.
        assert_eq!(
            advancement_to(&character(29, 100), 511, 110),
            Err(Advancement::TooLowLevel { level: 29, needed: 30 })
        );
        assert_eq!(advancement_to(&character(40, 0), 511, 110), Err(Advancement::StillABeginner));
        assert_eq!(advancement_to(&character(40, 100), 3, 110), Err(Advancement::NotAnInstructor));
    }

    /// An ordinary NPC is **not this module's business**, which is a different thing from a
    /// refusal - the caller must fall through rather than say no.
    #[test]
    fn an_ordinary_npc_is_not_this_modules_business() {
        let chr = character(40, 100);
        // Heena (1), Roger (3), Phil (101), Lucy the shopkeeper (21), and the second-job
        // examiner ids read as job ids by mistake.
        for t in [1u32, 3, 21, 101, 107, 110, 120, 210] {
            assert_eq!(advancement_for(&chr, t), Advancement::NotAnInstructor, "template {t}");
            assert_eq!(refusal(&chr, t), None);
        }
        // The positive control, so the negative above is not a property of the search.
        for t in [511u32, 313, 221, 411] {
            assert!(matches!(advancement_for(&chr, t), Advancement::Choose { .. } | Advancement::WrongBranch { .. }));
        }
    }

    /// Every refusal produces a sentence, and every non-refusal produces none. The "always
    /// answer" rule reaching into this module as "always have something to say".
    #[test]
    fn every_refusal_has_words_and_every_pass_has_none() {
        let cases: [(Character, u32, bool); 6] = [
            (character(29, 100), 511, true),           // too low
            (character(40, 200), 511, true),           // wrong branch
            (character(40, 110), 511, true),           // already advanced
            (character(40, 100), 514, true),           // the examiner
            (character(40, 100), 511, false),          // eligible
            (character(40, 0), 511, false),            // beginner: jobs.rs answers
        ];
        for (chr, npc, expect_words) in cases {
            let r = refusal(&chr, npc);
            assert_eq!(
                r.is_some(),
                expect_words,
                "job {} level {} at npc {npc}: {r:?}",
                chr.job,
                chr.level
            );
            if let Some(s) = r {
                assert!(s.len() > 20, "a sentence, not a token: {s:?}");
            }
        }
    }

    /// `grant` refuses anything that is not one of the ten, rather than defaulting.
    #[test]
    fn grant_refuses_a_job_it_does_not_know() {
        for j in [0u16, 100, 111, 231, 500, 9999] {
            assert!(grant(j, 30).is_none(), "job {j}");
        }
        for j in [110u16, 120, 130, 210, 220, 230, 310, 320, 410, 420] {
            let g = grant(j, 30).unwrap();
            assert_eq!(g.job, j);
            assert_eq!(g.job_name, second_job(j).unwrap().job_name);
        }
    }

    /// The quest chain is carried but not enforced, and that policy is stated in one place.
    #[test]
    fn the_quest_chain_is_data_not_a_gate() {
        // A const block, because clippy is right that this cannot vary at runtime - the point
        // is that changing the constant has to break something that says why.
        const { assert!(!REQUIRE_QUEST_CHAIN, "[I]: the Test of Qualification script is not implemented, so requiring the chain would make the second advancement untestable") };
        // A character who has never touched the chain is still eligible, which is what
        // REQUIRE_QUEST_CHAIN = false means in practice.
        assert!(matches!(advancement_for(&character(30, 100), 511), Advancement::Choose { .. }));
        // The chain's own numbers are distinct per branch, so nothing here is a copy-paste.
        let letters: BTreeSet<u32> = BRANCHES.iter().map(|b| b.chain.letter_item).collect();
        let marbles: BTreeSet<u32> = BRANCHES.iter().map(|b| b.chain.marble_item).collect();
        let proofs: BTreeSet<u32> = BRANCHES.iter().map(|b| b.chain.proof_item).collect();
        assert_eq!(letters.len(), 4);
        assert_eq!(marbles.len(), 4);
        assert_eq!(proofs.len(), 4);
        for b in BRANCHES {
            assert_eq!(b.chain.marble_count_items, 30, "30 ITEMS, not a percentage");
            assert_eq!(b.chain.exp_per_quest, 3150);
        }
    }

    // -----------------------------------------------------------------------------------
    // The hidden test fields
    // -----------------------------------------------------------------------------------

    /// Every comma-separated row of a `gm-handbook/` dump, or `None` on a clean checkout.
    fn real_rows(name: &str) -> Option<Vec<Vec<String>>> {
        let path = Path::new("../../gm-handbook/").join(name);
        if !path.exists() {
            return None;
        }
        let text = std::fs::read_to_string(path).ok()?;
        Some(
            text.lines()
                .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
                .map(|l| l.split(',').map(|c| c.trim().to_string()).collect())
                .collect(),
        )
    }

    /// **The four test fields are the client's own, and three separate nodes say so.**
    ///
    /// This is the check the whole feature rests on, and it is deliberately built out of
    /// *different* parts of the archive rather than one:
    ///
    /// * `fields.txt` and `footholds.txt` come from `Map.wz`'s field images - the map exists
    ///   and can be stood on.
    /// * `mobs.txt` and `npcs.txt` come from `Map.wz`'s `life` nodes - who is in it.
    /// * `returnmaps.txt` comes from `Map.wz`'s `info` node - where you go when you leave.
    /// * `maps.txt` comes from **`String.wz`**, a different archive entirely - what it is
    ///   called.
    ///
    /// If the pairing of branch to dungeon were wrong, these would disagree with each other.
    /// They do not, four times over. `CLAUDE.md`: *two scans agreeing is not corroboration
    /// when they share a blind spot* - so this uses five nodes across two archives, and each
    /// one is asked a question the others cannot answer.
    #[test]
    fn the_four_test_fields_are_exactly_what_this_client_ships() {
        let (Some(fields), Some(portals), Some(mobs), Some(npcs), Some(names)) = (
            real_rows("fields.txt"),
            real_rows("portals.txt"),
            real_rows("mobs.txt"),
            real_rows("npcs.txt"),
            real_rows("maps.txt"),
        ) else {
            return; // clean checkout; gm-handbook/ is generated and gitignored
        };
        // **Positive controls first.** A file that failed to parse must not read as a set of
        // clean negatives, which is the exact failure this project keeps re-learning.
        assert!(fields.len() > 400, "fields.txt holds the whole archive, got {}", fields.len());
        assert!(portals.len() > 3000, "portals.txt loaded, got {}", portals.len());
        assert!(mobs.len() > 9000, "mobs.txt loaded, got {}", mobs.len());
        assert!(
            portals.iter().filter(|r| r[0] == "10004000").count() > 20,
            "control: Perion has many portals, so 'one portal' below means something"
        );

        for b in BRANCHES {
            let f = b.test_field;
            let map = f.map_id.to_string();

            // 1. It is a real field.
            assert!(
                fields.iter().any(|r| r[0] == map),
                "{} ({}) has a field image",
                f.map_id,
                f.map_name
            );

            // 2. **Exactly one portal, and it is the spawn point.** This is the fact that
            //    makes the warp the only way in and the warden the only way out.
            let mine: Vec<&Vec<String>> = portals.iter().filter(|r| r[0] == map).collect();
            assert_eq!(mine.len(), 1, "{} has exactly one portal, got {:?}", f.map_id, mine);
            assert_eq!(mine[0][2], "sp", "{}'s only portal is the spawn point", f.map_id);

            // 3. Its mobs are the two this branch claims, and nothing else.
            let mut here: BTreeSet<u32> = BTreeSet::new();
            for row in mobs.iter().filter(|r| r[0] == map) {
                here.insert(row[1].parse().unwrap());
            }
            let want: BTreeSet<u32> = f.mobs.iter().copied().collect();
            assert_eq!(here, want, "the mobs in {} ({})", f.map_id, f.map_name);

            // 4. The warden stands in it, and it is their only placement in the archive.
            let warden = f.warden_npc.to_string();
            let placements: Vec<&Vec<String>> =
                npcs.iter().filter(|r| r[1] == warden).collect();
            assert_eq!(placements.len(), 1, "NPC {warden} is placed once");
            assert_eq!(placements[0][0], map, "NPC {warden} stands in {}", f.map_id);

            // 5. String.wz agrees about the name - a different archive from all of the above.
            let named = names.iter().find(|r| r[0] == map).map(|r| r[1..].join(", "));
            assert_eq!(named.as_deref(), Some(f.map_name), "String.wz name for {}", f.map_id);
        }
    }

    /// **The exit is the client's own `returnMap`, not a choice this server made.**
    ///
    /// Split from the test above because it reads a different file with a different separator
    /// and, more importantly, because it is the strongest single piece of evidence for the
    /// branch-to-dungeon pairing: an ordinary map's `forcedReturn` is `999999999`, and these
    /// four name a real map - their own examiner's.
    #[test]
    fn each_test_field_is_ejected_into_its_own_examiner_s_map() {
        let path = Path::new("../../gm-handbook/returnmaps.txt");
        if !path.exists() {
            return;
        }
        let text = std::fs::read_to_string(path).expect("returnmaps.txt");
        let mut rows: BTreeMap<u32, (u32, u32)> = BTreeMap::new();
        for line in text.lines() {
            if line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() < 3 {
                continue;
            }
            let (Ok(map), Ok(ret), Ok(forced)) =
                (f[0].parse::<u32>(), f[1].parse::<u32>(), f[2].parse::<u32>())
            else {
                continue;
            };
            rows.insert(map, (ret, forced));
        }
        assert!(rows.len() > 400, "positive control: the file loaded, got {}", rows.len());
        // Control: an ordinary map has NO forced return, so "these four do" is a statement.
        assert_eq!(
            rows.get(&10004000).map(|r| r.1),
            Some(999_999_999),
            "control: Perion is not a map you get ejected from"
        );

        for b in BRANCHES {
            let (ret, forced) = rows[&b.test_field.map_id];
            assert_eq!(ret, b.examiner_map_id, "{} returnMap", b.test_field.map_name);
            assert_eq!(forced, b.examiner_map_id, "{} forcedReturn", b.test_field.map_name);
            assert_eq!(
                b.test_field.exit_map_id, b.examiner_map_id,
                "and this table says the same thing"
            );
        }
    }

    /// **A marble drops for its own mob in its own field, and nowhere else.**
    ///
    /// The three leaks are named individually rather than tested as a class, because they are
    /// what the map half of the rule exists for: `800010` and `800015` also spawn on 80003500
    /// and `800011` also spawns on 10006160, *Precipice of Darkness* - an ordinary field.
    #[test]
    fn a_dark_marble_drops_only_in_its_own_test_field() {
        for b in BRANCHES {
            for mob in b.test_field.mobs {
                assert_eq!(
                    marble_for_kill(mob, b.test_field.map_id),
                    Some(b.chain.marble_item),
                    "mob {mob} in {}",
                    b.test_field.map_name
                );
            }
        }
        // The three real leaks, by name.
        assert_eq!(marble_for_kill(800010, 80003500), None, "Evil Eye outside its field");
        assert_eq!(marble_for_kill(800015, 80003500), None, "Blue Mushroom outside its field");
        assert_eq!(
            marble_for_kill(800011, 10006160),
            None,
            "Zombie Mushroom on Precipice of Darkness, an ORDINARY field"
        );
        // A test mob in the wrong branch's field: the map matches a branch, the mob does not.
        assert_eq!(marble_for_kill(800016, 80001000), None, "Fire Boar in the Bowman's tunnel");
        // And an ordinary mob in a test field earns nothing.
        assert_eq!(marble_for_kill(30, 80001300), None, "the ordinary Fire Boar, template 30");
        // The four marbles are recognised, and an ordinary item is not.
        for b in BRANCHES {
            assert!(is_marble(b.chain.marble_item));
            assert!(is_proof(b.chain.proof_item));
            assert!(!is_marble(b.chain.proof_item), "a proof is not a marble");
        }
        assert!(!is_marble(2000000), "a Red Potion is not a marble");
    }

    /// **The examiner's four outcomes, and the one that must not carry an effect.**
    #[test]
    fn the_examiner_tests_and_advances_nobody() {
        let b = &BRANCHES[0]; // Warrior
        let ready = character(30, 100);

        // Not enough marbles -> in you go, and the arm carries the field.
        match test_step(&ready, b.examiner_npc, 0, false) {
            Some(TestStep::Enter { field, .. }) => {
                assert_eq!(field.map_id, 80001300);
                assert_eq!(field.warden_npc, 800006);
            }
            other => panic!("expected Enter, got {other:?}"),
        }
        // One short is still short.
        assert!(matches!(
            test_step(&ready, b.examiner_npc, 29, false),
            Some(TestStep::Enter { .. })
        ));
        // Thirty passes, and the arm carries exactly what moves.
        match test_step(&ready, b.examiner_npc, 30, false) {
            Some(TestStep::Pass { marble_item, take_marbles, proof_item, .. }) => {
                assert_eq!(marble_item, 4031017);
                assert_eq!(take_marbles, 30);
                assert_eq!(proof_item, 4031018);
            }
            other => panic!("expected Pass, got {other:?}"),
        }
        // Holding the proof already: nothing to take, nothing to give, and NO field to
        // re-enter. That last one matters - a second Enter would strand a finished player.
        match test_step(&ready, b.examiner_npc, 99, true) {
            Some(TestStep::AlreadyPassed { .. }) => {}
            other => panic!("expected AlreadyPassed, got {other:?}"),
        }
        // A beginner is refused with a sentence rather than warped, and the sentence names
        // where to actually go.
        match test_step(&character(30, 0), b.examiner_npc, 99, false) {
            Some(TestStep::Refused(line)) => assert!(
                line.contains("Dances with Balrog"),
                "a beginner is sent to the instructor, got {line:?}"
            ),
            other => panic!("expected Refused, got {other:?}"),
        }
        // Level 29 with 30 marbles is still refused - the marbles do not buy the level.
        assert!(matches!(
            test_step(&character(29, 100), b.examiner_npc, 30, false),
            Some(TestStep::Refused(_))
        ));
        // Wrong branch.
        assert!(matches!(
            test_step(&character(30, 200), b.examiner_npc, 30, false),
            Some(TestStep::Refused(_))
        ));
        // And the examiner is not an NPC that advances: every other template is None.
        assert!(test_step(&ready, b.instructor_npc, 30, false).is_none());
        assert!(test_step(&ready, 9_999_999, 30, false).is_none());
    }

    /// **The warden never refuses**, because refusing would strand somebody.
    #[test]
    fn the_warden_always_opens_the_door() {
        for b in BRANCHES {
            for held in [0, 1, 29, 30, 100] {
                let step = warden_step(b.test_field.warden_npc, held)
                    .expect("the warden always answers");
                assert_eq!(step.to_map_id, b.examiner_map_id);
                assert!(!step.line.is_empty(), "and it always says something");
            }
        }
        // Only those four templates. The examiner is NOT a warden even though the client
        // gives them the same `name` string, which is the trap this pairing exists to avoid.
        for b in BRANCHES {
            assert!(warden_step(b.examiner_npc, 0).is_none());
            assert!(warden_step(b.instructor_npc, 0).is_none());
        }
    }

    /// **The proof gate is asked, and its answer is used.**
    ///
    /// `CLAUDE.md`'s Heena section: a guard whose answer is ignored is not a guard. This
    /// asserts both directions, because a gate that always says no is as broken as one that
    /// always says yes.
    #[test]
    fn the_advancement_needs_the_proof_in_hand() {
        assert!(REQUIRE_PROOF_ITEM, "if this is turned off, the test below means nothing");
        let ready = character(30, 100);
        assert!(matches!(
            advancement_for_holding(&ready, 511, false),
            Advancement::NeedsProof { .. }
        ));
        assert!(matches!(
            advancement_for_holding(&ready, 511, true),
            Advancement::Choose { .. }
        ));
        // The refusal names the examiner and the item, so the player knows what to do next.
        let text = refusal_for(&advancement_for_holding(&ready, 511, false)).expect("a sentence");
        assert!(text.contains("Warrior Job Instructor"), "got {text:?}");
        assert!(text.contains(PROOF_ITEM_NAME), "got {text:?}");
        // Holding the proof does not buy the level or the branch.
        assert!(matches!(
            advancement_for_holding(&character(29, 100), 511, true),
            Advancement::TooLowLevel { .. }
        ));
        assert!(matches!(
            advancement_for_holding(&character(30, 200), 511, true),
            Advancement::WrongBranch { .. }
        ));
        // And a beginner is still handed back to `crate::jobs` rather than refused.
        assert!(matches!(
            advancement_for_holding(&character(30, 0), 511, true),
            Advancement::StillABeginner
        ));
        assert_eq!(refusal_for(&Advancement::StillABeginner), None);
    }

    /// **The menu lists every choice, and `choice_at` is its inverse.**
    ///
    /// Written as a round trip rather than as two separate assertions because the failure
    /// that matters is the two disagreeing: a menu that draws three lines and a decoder that
    /// only knows two grants the wrong job to whoever picks the third.
    #[test]
    fn the_choice_menu_and_its_decoder_agree() {
        for b in BRANCHES {
            let text = menu_text(&b);
            for (i, choice) in b.choices.iter().enumerate() {
                let line = format!("#L{i}# {}", choice.job_name);
                assert!(text.contains(&line), "{} is missing {line:?}", b.from_job_name);
                assert_eq!(
                    choice_at(&b, i as u32).map(|c| c.job),
                    Some(choice.job),
                    "position {i} decodes back to the job it drew"
                );
            }
            // One past the end is not a job, it is an answer off a socket.
            assert!(choice_at(&b, b.choices.len() as u32).is_none());
            assert!(choice_at(&b, u32::MAX).is_none());
            // The line break is the client's two-character markup, not an escape.
            assert!(text.contains("\\n"), "the menu uses the client's own break");
            assert_eq!(LINE_BREAK.len(), 2, "backslash and n, not a newline");
        }
        assert!(is_menu_path(MENU_PATH));
        assert!(!is_menu_path(crate::taxi::MENU_PATH), "the two must not claim each other");
        assert!(!crate::taxi::is_taxi_path(MENU_PATH));
    }


    /// **The chain, as the server will actually have it: WZ plus the authored overlay.**
    ///
    /// Everything else in this file checks the generated dump. This one checks the two files
    /// *joined the way `world_server` joins them*, because the second-job chain is the first
    /// feature that depends on both halves at once - the client ships the proof grant, and
    /// `data/quest-scripts.txt` supplies the marble take-back that the missing `q<id>e`
    /// endscript used to do.
    ///
    /// The control is the part that comes from the **client**: if `20003`'s `Act.0` did not
    /// hand over the proof, the whole design would be resting on a row that is not there.
    #[test]
    fn the_authored_overlay_and_the_client_s_own_rows_join_up() {
        let wz = Path::new("../../gm-handbook/questlines.txt");
        let overlay = Path::new("../../data/quest-scripts.txt");
        if !wz.exists() || !overlay.exists() {
            return; // gm-handbook/ is generated and gitignored
        }
        let mut quests = crate::config::load_quests(wz);
        assert!(quests.len() > 300, "positive control: {} quests loaded", quests.len());
        let touched = crate::config::overlay_quests(&mut quests, overlay);
        assert!(touched > 0, "positive control: the overlay was applied, not skipped");

        for b in BRANCHES {
            let [_first, finding, test_of, proof_of] = b.chain.quests;

            // --- from the CLIENT: the letter and the proof are its own Act.0 rows ---
            let finding_q = &quests[&finding];
            assert!(
                finding_q.start_items.contains(&(b.chain.letter_item, 1)),
                "quest {finding} hands over the letter - that row is the client's"
            );
            let proof_q = &quests[&proof_of];
            assert!(
                proof_q.start_items.contains(&(b.chain.proof_item, 1)),
                "quest {proof_of} hands over the proof - THE CONTROL. If this ever fails, the \
                 advancement's gate has nothing to open it"
            );

            // --- from the OVERLAY: the endscript's take-back, and words where there were none
            let test_q = &quests[&test_of];
            let want = i32::try_from(b.chain.marble_count_items).unwrap();
            assert_eq!(
                test_q.complete_items,
                vec![(b.chain.marble_item, -want)],
                "quest {test_of} takes the {want} marbles back - the WZ's Act.1 is exp and \
                 nextQuest only, so without the overlay they stay in the bag"
            );
            assert!(test_q.say.contains_key("0"), "quest {test_of} has an opening now");
            assert!(test_q.say.contains_key("1"), "quest {test_of} has a closing now");

            // --- and the chain still links the way the client wrote it ---
            assert_eq!(quests[&finding].next_quest, Some(test_of));
            assert_eq!(test_q.next_quest, Some(proof_of));
            assert_eq!(proof_q.next_quest, None, "and it ends at the instructor");
            assert_eq!(quests[&test_of].end_npc, Some(b.examiner_npc));
            assert_eq!(proof_q.end_npc, Some(b.instructor_npc), "the LAST one ends at 511-ish");
        }
    }


    /// **The server's own loaders populate the four fields**, not just this file's parser.
    ///
    /// Separate from `the_four_test_fields_are_exactly_what_this_client_ships` on purpose.
    /// That test reads the dumps with a parser written next to it, which proves the *data* is
    /// right and proves nothing about whether `world_server` can see it. This one goes through
    /// `Config::load_mobs` and `Config::load_npcs` - the exact calls the binary makes - so a
    /// loader that silently drops eight-digit map ids, or a column count that moved, fails
    /// here instead of on the owner's screen as an empty room with no way out.
    #[test]
    fn the_server_s_own_loaders_fill_the_four_fields() {
        let mobs_path = Path::new("../../gm-handbook/mobs.txt");
        let npcs_path = Path::new("../../gm-handbook/npcs.txt");
        let templates_path = Path::new("../../gm-handbook/mobtemplates.txt");
        if !mobs_path.exists() || !npcs_path.exists() || !templates_path.exists() {
            return;
        }
        let templates = crate::config::load_mob_templates(templates_path);
        assert!(templates.len() > 150, "positive control: {} templates", templates.len());
        let (fields, _respawn) = crate::config::Config::load_mobs(mobs_path, &templates);
        let npcs = crate::config::Config::load_npcs(npcs_path);
        assert!(fields.len() > 200, "positive control: {} maps have mobs", fields.len());
        assert!(npcs.len() > 100, "positive control: {} maps have npcs", npcs.len());

        for b in BRANCHES {
            let f = b.test_field;
            let here = fields.get(&f.map_id).unwrap_or_else(|| {
                panic!("{} ({}) has no mobs after loading", f.map_id, f.map_name)
            });
            // 26 or 30 spawn points, and every one of them is one of this branch's two.
            assert!(here.len() >= 26, "{} spawns {} mobs", f.map_id, here.len());
            for m in here {
                assert!(
                    f.has_mob(m.template_id),
                    "{} spawned template {} which is not one of {:?}",
                    f.map_id,
                    m.template_id,
                    f.mobs
                );
                // A mob loaded with no template would come up at DEFAULT_MOB_HP, which is a
                // different fight from the one the test is meant to be.
                assert!(
                    templates.contains_key(&m.template_id),
                    "template {} has no Mob.wz row, so its HP would be a default",
                    m.template_id
                );
            }
            // The warden is there, and they are the only NPC - so a click in that map cannot
            // land on anything else.
            let who = npcs.get(&f.map_id).unwrap_or_else(|| {
                panic!("{} has no NPCs after loading - THE MAP HAS NO DOOR", f.map_id)
            });
            assert_eq!(who.len(), 1, "{} holds only the warden, got {:?}", f.map_id, who);
            assert_eq!(who[0].template_id, f.warden_npc);
        }
    }

}
