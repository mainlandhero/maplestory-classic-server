//! Third job advancement at level 70 - the pure decision.
//!
//! Full working, with every enumeration and every named negative:
//! **`research/third-job.md`**.
//!
//! Labels are the project's: **[L]** read off this client's listing, its WZ or a capture,
//! **[D]** derived from two or more [L] facts, **[I]** inferred - a policy nothing on this
//! machine can confirm.
//!
//! # Nothing here authenticates
//!
//! As everywhere in this project, the channel socket carries no credentials. A third job
//! advancement is granted on the say-so of whoever holds the socket.
//!
//! # This one is AUTHORED, not decoded, and the difference matters
//!
//! The second advancement was decoded: the client shipped a sixteen-quest chain, the item
//! ids, the counts, four hidden maps and eight dedicated mobs, and the work was joining them
//! up. **The third advancement has none of that.**
//!
//! What this client ships is the *destination* - ten skill books, their names, and four
//! instructors standing in one room in El Nath. What it does not ship is any test:
//!
//! | | second job | third job |
//! |---|---|---|
//! | quest chain | 16 quests, `20000`..`20303` **[L]** | **none.** All 322 enumerated; nothing above `20303` |
//! | hidden test field | four, one portal each **[L]** | **none.** Every one-portal map in the archive enumerated |
//! | dedicated test mobs | eight, `800010`..`800017` **[L]** | **none** |
//! | test items | letter, 30 marbles, proof **[L]** | **none** |
//! | level gate in the data | `Check.0.lvmin = 30`, sixteen times **[L]** | **none** - see [`LEVEL_MINIMUM`] |
//!
//! So the gate here is **level and job and nothing else**, which is the owner's decision of
//! 2026-08-31 taken in full knowledge that the client offers no alternative. It is the same
//! shape as the *first* advancement, and deliberately so.
//!
//! # There is no choice to make, and that is the client's doing
//!
//! At second job a Swordsman picks one of three. **At third job there is exactly one
//! destination per second job** - `111` is the book that sits under `110` in `Skill.wz`, and
//! the ten third-job books map onto the ten second-job books one-to-one. **[D]** So this
//! module has no menu: [`advancement_for`] either names the single job or refuses.
//!
//! # The five facts this file exists to keep in one place
//!
//! 1. **The ten third jobs are 111 / 121 / 131 / 211 / 221 / 231 / 311 / 321 / 411 / 421.**
//!    **[L]** - the `job` column of `gm-handbook/skills.txt`, which `tools/dump_skills.py`
//!    derives from the image each skill was found in.
//!
//! 2. **The four instructors are 1104 Tylus, 1105 Robeira, 1106 Rene and 1107 Arec**, and all
//!    four stand in **Chief's Residence, map 20001001**, in El Nath. Each is placed exactly
//!    once in the whole archive. **[L]** They are identified by *their own idle lines*, not by
//!    a remembered roster - see [`Master`].
//!
//! 3. **The SP pool key is 3** for all ten, and **the SP encoding does not fork**. See
//!    [`Grant`].
//!
//! 4. **Three of the ten books carry an eighth, invisible skill** - `2111006`, `3111006`,
//!    `3211006`. They are already in [`crate::secondjob::HIDDEN_SKILLS`], which holds all
//!    thirteen of the archive's `invisible = 1` skills, so this module reuses that predicate
//!    rather than writing a second list. Two copies of one rule is how one of them gets
//!    missed.
//!
//! 5. **El Nath is not reachable on foot from Victoria Island.** Two portal components, 223
//!    maps and 87, and the only link is a ship. **[L]** Nothing in this module can fix that;
//!    it is named here because an advancement nobody can walk to is an advancement that does
//!    not exist. `research/third-job.md` §4.
//!
//! # Seven of the ten job names are the client's, three are not
//!
//! Exactly the same split as second job, and the same three Magician jobs. See
//! [`crate::secondjob::NameSource`], which is reused rather than redefined.
//!
//! **The invariant is weaker here than at second job**, and the test says so: second-job book
//! names are `"<Name> ..."` so `starts_with` holds, but `"Path of the Ranger"` and `"The Way
//! of Hermit"` put the name in the middle. The assertion is therefore `contains`, and it is
//! written that way because a `starts_with` copied from `secondjob.rs` would fail on two
//! entries and the temptation would be to "fix" the names rather than the assertion.

use net::opcode::Character;

use crate::secondjob::NameSource;
use crate::skillpoints::{self, Tier};

/// The level at which the third advancement becomes possible.
///
/// **[I], and more weakly held than any other level in this project.** [`crate::jobs::
/// LEVEL_MINIMUM`] is corroborated by quest `10001`'s own text and
/// [`skillpoints::SECOND_JOB_LEVEL`] by sixteen quests' `Check.0.lvmin`. **There is no third-
/// job quest in this client at all**, so there is nothing to read this off and nothing that
/// can ever contradict it. 70 is the conventional MapleStory number.
///
/// It lives in [`skillpoints`] because the skill-point tier needs the same number, and one
/// constant used twice cannot drift from itself.
pub const LEVEL_MINIMUM: u32 = skillpoints::THIRD_JOB_LEVEL;

/// Flat max-HP points the advancement adds. **Zero, and that is a decision, not "unset"** -
/// the same one [`crate::secondjob::MAX_HP_GAIN_POINTS`] records, for the same reason: all
/// 100+ images of `Etc_000.wz` were enumerated and there is no per-job HP/MP table in this
/// client. A number here would be a second unverifiable constant.
pub const MAX_HP_GAIN_POINTS: u32 = 0;

/// Flat max-MP points the advancement adds. See [`MAX_HP_GAIN_POINTS`].
pub const MAX_MP_GAIN_POINTS: u32 = 0;

/// The one map all four instructors stand on: **Chief's Residence**. **[L]**
pub const INSTRUCTOR_MAP: u32 = 20_001_001;

/// [`INSTRUCTOR_MAP`]'s name from `String.wz/Map.img`. **[L]**
pub const INSTRUCTOR_MAP_NAME: &str = "Chief's Residence";

/// The town [`INSTRUCTOR_MAP`] opens onto. **[L]** - one portal, `out00`.
pub const INSTRUCTOR_TOWN: u32 = 20_001_000;

/// [`INSTRUCTOR_TOWN`]'s name. **[L]**
pub const INSTRUCTOR_TOWN_NAME: &str = "El Nath";

// ---------------------------------------------------------------------------------------
// The ten third jobs
// ---------------------------------------------------------------------------------------

/// One third job: what it is called, what its book is, and what is in it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThirdJob {
    /// The **second** job a character must already hold. One-to-one: `110` -> `111`. **[D]**
    pub from_job: u16,
    /// The job id the advancement grants, which is also the `Skill.wz` book id. **[L]**
    pub job: u16,
    /// What to *call* it. Read [`ThirdJob::name_source`] before quoting this at a player.
    pub job_name: &'static str,
    /// Where [`ThirdJob::job_name`] came from.
    pub name_source: NameSource,
    /// `String.wz/Skill.img/<job>/bookName`, verbatim. **[L]**
    pub book_name: &'static str,
    /// **Every** skill id in the book, hidden ones included, ascending. **[L]**
    pub skill_book: &'static [u32],
}

impl ThirdJob {
    /// The skills a player may be granted: [`ThirdJob::skill_book`] minus the archive's
    /// `invisible = 1` set.
    ///
    /// Reuses [`crate::secondjob::is_hidden`] rather than a second list. The thirteen invisible
    /// skills are one fact about this client, not one fact per tier.
    pub fn grantable_skills(&self) -> impl Iterator<Item = u32> + '_ {
        self.skill_book.iter().copied().filter(|id| !crate::secondjob::is_hidden(*id))
    }

    /// How many of [`ThirdJob::skill_book`] a player may actually have. **Seven, for all ten
    /// books** - three of them carry an eighth that is invisible.
    pub fn grantable_count(&self) -> usize {
        self.grantable_skills().count()
    }
}

const CRUSADER_BOOK: [u32; 7] =
    [1_110_000, 1_111_000, 1_111_001, 1_111_002, 1_111_003, 1_111_004, 1_111_005];
const WHITE_KNIGHT_BOOK: [u32; 7] =
    [1_210_000, 1_211_000, 1_211_001, 1_211_002, 1_211_003, 1_211_004, 1_211_005];
const DRAGON_KNIGHT_BOOK: [u32; 7] =
    [1_310_000, 1_311_000, 1_311_001, 1_311_002, 1_311_003, 1_311_004, 1_311_005];
/// **Eight**, and `2111006` is invisible.
const FIRE_POISON_MAGE_BOOK: [u32; 8] =
    [2_110_000, 2_111_000, 2_111_001, 2_111_002, 2_111_003, 2_111_004, 2_111_005, 2_111_006];
const ICE_LIGHTNING_MAGE_BOOK: [u32; 7] =
    [2_210_000, 2_211_000, 2_211_001, 2_211_002, 2_211_003, 2_211_004, 2_211_005];
const PRIEST_BOOK: [u32; 7] =
    [2_310_000, 2_311_000, 2_311_001, 2_311_002, 2_311_003, 2_311_004, 2_311_005];
/// **Eight**, and `3111006` is invisible.
const RANGER_BOOK: [u32; 8] =
    [3_110_000, 3_111_000, 3_111_001, 3_111_002, 3_111_003, 3_111_004, 3_111_005, 3_111_006];
/// **Eight**, and `3211006` is invisible.
const SNIPER_BOOK: [u32; 8] =
    [3_210_000, 3_211_000, 3_211_001, 3_211_002, 3_211_003, 3_211_004, 3_211_005, 3_211_006];
const HERMIT_BOOK: [u32; 7] =
    [4_110_000, 4_111_000, 4_111_001, 4_111_002, 4_111_003, 4_111_004, 4_111_005];
const CHIEF_BANDIT_BOOK: [u32; 7] =
    [4_210_000, 4_211_000, 4_211_001, 4_211_002, 4_211_003, 4_211_004, 4_211_005];

const WARRIOR_THIRD: [ThirdJob; 3] = [
    ThirdJob {
        from_job: 110,
        job: 111,
        job_name: "Crusader",
        name_source: NameSource::BookName,
        book_name: "Crusader's Guide",
        skill_book: &CRUSADER_BOOK,
    },
    ThirdJob {
        from_job: 120,
        job: 121,
        job_name: "White Knight",
        name_source: NameSource::BookName,
        book_name: "White Knight's Code",
        skill_book: &WHITE_KNIGHT_BOOK,
    },
    ThirdJob {
        from_job: 130,
        job: 131,
        job_name: "Dragon Knight",
        name_source: NameSource::BookName,
        book_name: "Dragon Knight's Path",
        skill_book: &DRAGON_KNIGHT_BOOK,
    },
];

const MAGICIAN_THIRD: [ThirdJob; 3] = [
    ThirdJob {
        from_job: 210,
        job: 211,
        // The book names the ELEMENT, not the job - "Adv. Fire & Poison". Same three jobs the
        // second-job table has to invent names for, one tier up.
        job_name: "Mage (Fire/Poison)",
        name_source: NameSource::NotInThisClient,
        book_name: "Adv. Fire & Poison",
        skill_book: &FIRE_POISON_MAGE_BOOK,
    },
    ThirdJob {
        from_job: 220,
        job: 221,
        job_name: "Mage (Ice/Lightning)",
        name_source: NameSource::NotInThisClient,
        book_name: "Adv. Ice & Lightning",
        skill_book: &ICE_LIGHTNING_MAGE_BOOK,
    },
    ThirdJob {
        from_job: 230,
        job: 231,
        job_name: "Priest",
        name_source: NameSource::NotInThisClient,
        book_name: "Adv. Holy Magic",
        skill_book: &PRIEST_BOOK,
    },
];

const BOWMAN_THIRD: [ThirdJob; 2] = [
    ThirdJob {
        from_job: 310,
        job: 311,
        // "Path of the Ranger" - the name is at the END, which is why the test asserts
        // `contains` rather than `starts_with`.
        job_name: "Ranger",
        name_source: NameSource::BookName,
        book_name: "Path of the Ranger",
        skill_book: &RANGER_BOOK,
    },
    ThirdJob {
        from_job: 320,
        job: 321,
        job_name: "Sniper",
        name_source: NameSource::BookName,
        book_name: "Sniper's Scope",
        skill_book: &SNIPER_BOOK,
    },
];

const THIEF_THIRD: [ThirdJob; 2] = [
    ThirdJob {
        from_job: 410,
        job: 411,
        // "The Way of Hermit" - also mid-string.
        job_name: "Hermit",
        name_source: NameSource::BookName,
        book_name: "The Way of Hermit",
        skill_book: &HERMIT_BOOK,
    },
    ThirdJob {
        from_job: 420,
        job: 421,
        job_name: "Chief Bandit",
        name_source: NameSource::BookName,
        book_name: "Chief Bandit's Tricks",
        skill_book: &CHIEF_BANDIT_BOOK,
    },
];

// ---------------------------------------------------------------------------------------
// The four instructors
// ---------------------------------------------------------------------------------------

/// One of the four NPCs who perform the third advancement.
///
/// # They are identified by their own words
///
/// This matters more than it sounds. A remembered roster of the retail game gets **two** of
/// these wrong - it puts *Helena* on the Bowman branch (this client has no Helena; the closest
/// name is *Hella*, mentioned once in another NPC's dialogue with nobody behind it), and it
/// expects *Chief Stan* in Chief's Residence. **Chief Stan is NPC 202, in Henesys**, a father
/// in a gold-watch quest. **[L]**
///
/// So each row below is anchored on the instructor's own `idle0`, quoted in [`Master::says`]:
///
/// ```text
///   1104 Tylus    "Do you want to be a more powerful warrior than you ever were?"
///   1105 Robeira  "You'll need to see me in order to become the best magician in the world."
///   1106 Rene     "The path of the bowman is long and treacherous."
///   1107 Arec     "You want to become a powerful thief? Then you've come to the right person."
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Master {
    /// `Map.wz`'s `life` node id. Placed exactly once in the whole archive. **[L]**
    pub npc: u32,
    /// `String.wz/Npc.img/<template>/name`. **[L]**
    pub name: &'static str,
    /// The **first** job whose line this instructor serves - 100/200/300/400. Not a gate on its
    /// own; the gate is [`ThirdJob::from_job`], which is a second job.
    pub branch_first_job: u16,
    /// What the client's own quest text calls that branch. **[L]**
    pub branch_name: &'static str,
    /// The instructor's own `idle0`, verbatim. **[L]** This is the evidence for the pairing
    /// above it, kept in the struct so it cannot be separated from the claim it supports.
    pub says: &'static str,
    /// The second jobs this instructor advances, and what each becomes.
    pub serves: &'static [ThirdJob],
}

/// The four, in first-job order.
pub const MASTERS: [Master; 4] = [
    Master {
        npc: 1104,
        name: "Tylus",
        branch_first_job: 100,
        branch_name: "Warrior",
        says: "Do you want to be a more powerful warrior than you ever were?",
        serves: &WARRIOR_THIRD,
    },
    Master {
        npc: 1105,
        name: "Robeira",
        branch_first_job: 200,
        branch_name: "Magician",
        says: "You'll need to see me in order to become the best magician in the world.",
        serves: &MAGICIAN_THIRD,
    },
    Master {
        npc: 1106,
        name: "Rene",
        branch_first_job: 300,
        branch_name: "Bowman",
        says: "The path of the bowman is long and treacherous.",
        serves: &BOWMAN_THIRD,
    },
    Master {
        npc: 1107,
        name: "Arec",
        branch_first_job: 400,
        branch_name: "Thief",
        says: "You want to become a powerful thief? Then you've come to the right person.",
        serves: &THIEF_THIRD,
    },
];

// ---------------------------------------------------------------------------------------
// Lookups
// ---------------------------------------------------------------------------------------

/// The instructor this NPC is, or `None` for every other NPC in the game.
pub fn master_at(npc_template: u32) -> Option<&'static Master> {
    MASTERS.iter().find(|m| m.npc == npc_template)
}

/// What a character holding `second_job` advances into, or `None` if that is not a second job.
///
/// One-to-one, which is what makes the third advancement a statement rather than a question.
pub fn third_job_from(second_job: u16) -> Option<&'static ThirdJob> {
    MASTERS.iter().flat_map(|m| m.serves).find(|t| t.from_job == second_job)
}

/// The instructor who serves a character holding `second_job`.
pub fn master_for_job(second_job: u16) -> Option<&'static Master> {
    MASTERS.iter().find(|m| m.serves.iter().any(|t| t.from_job == second_job))
}

/// Whether `job` is one of the ten third jobs.
pub fn is_third_job(job: u16) -> bool {
    MASTERS.iter().flat_map(|m| m.serves).any(|t| t.job == job)
}

/// Every third job, for a test or a GM command.
pub fn all() -> impl Iterator<Item = &'static ThirdJob> {
    MASTERS.iter().flat_map(|m| m.serves.iter())
}

// ---------------------------------------------------------------------------------------
// The decision
// ---------------------------------------------------------------------------------------

/// What clicking one of the four instructors should do.
///
/// Every arm carries the numbers a refusal sentence needs, so the caller never re-derives them
/// and the two cannot drift apart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Advancement {
    /// Not one of the four. Fall through to whatever the caller does for an ordinary NPC.
    NotAnInstructor,
    /// Not a second-job character. **Covers a beginner and a first job alike**, because at
    /// *these* four NPCs both are the same answer - unlike 511/313/221/411, where a beginner is
    /// [`crate::jobs`]'s business and a first job is [`crate::secondjob`]'s.
    NotSecondJobYet { job: u16, master: &'static Master },
    /// A second-job character at the wrong instructor - a Wizard clicking Tylus.
    WrongBranch { have_job: u16, master: &'static Master },
    /// Already third job or beyond. One-way, exactly as the other two advancements are.
    AlreadyAdvanced { job: u16 },
    /// Below [`LEVEL_MINIMUM`].
    TooLowLevel { level: u32, needed: u32, master: &'static Master },
    /// Everything passes. **There is nothing to choose** - `third` is the only job this
    /// character can advance into.
    Eligible { third: &'static ThirdJob, master: &'static Master },
}

/// May this character take a third advancement by talking to this NPC?
///
/// **Pure.** No database, no packets, no clock, no quest state - there is no quest.
///
/// The order of the checks is the order the refusals should be *said* in. Branch is checked
/// before level, because a Wizard at Tylus is at the wrong NPC whatever their level, and
/// "come back at 70" would be advice that never becomes true.
pub fn advancement_for(chr: &Character, npc_template: u32) -> Advancement {
    let Some(master) = master_at(npc_template) else {
        return Advancement::NotAnInstructor;
    };
    if is_third_job(chr.job) {
        return Advancement::AlreadyAdvanced { job: chr.job };
    }
    let Some(third) = third_job_from(chr.job) else {
        // Not a second job at all: a beginner, a first job, or something outside the tree.
        return Advancement::NotSecondJobYet { job: chr.job, master };
    };
    if !master.serves.iter().any(|t| t.from_job == chr.job) {
        return Advancement::WrongBranch { have_job: chr.job, master };
    }
    if chr.level < LEVEL_MINIMUM {
        return Advancement::TooLowLevel { level: chr.level, needed: LEVEL_MINIMUM, master };
    }
    Advancement::Eligible { third, master }
}

/// The sentence for an [`Advancement`] that has already been decided, or `None` when there is
/// nothing to refuse.
///
/// **A refusal is still an answer.** `CLAUDE.md`'s first expensive rule is that an unanswered
/// packet freezes the client's entire UI.
///
/// Split from [`advancement_for`] so a caller cannot end up with two wordings for one arm -
/// the same shape as [`crate::secondjob::refusal_for`].
pub fn refusal_for(outcome: &Advancement) -> Option<String> {
    match *outcome {
        Advancement::NotAnInstructor | Advancement::Eligible { .. } => None,
        Advancement::NotSecondJobYet { job, master } => Some(if job == 0 {
            format!(
                "You have no job at all yet. Come back when you have walked the {} road far \
                 enough to have taken its second step.",
                master.branch_name
            )
        } else {
            format!(
                "Job {job} is only the first step. Take your second advancement before you \
                 come looking for a third."
            )
        }),
        Advancement::WrongBranch { have_job, master } => Some(format!(
            "I teach the {} road, and job {have_job} does not walk it. One of the others in \
             this room is yours.",
            master.branch_name
        )),
        Advancement::AlreadyAdvanced { job } => {
            Some(format!("You have already taken that step. Job {job} cannot be undone."))
        }
        Advancement::TooLowLevel { level, needed, .. } => Some(format!(
            "Come back when you have reached Level {needed}. You are only Level {level}."
        )),
    }
}

/// [`refusal_for`], re-deciding from the character. Convenience for a caller that has not
/// already computed the outcome.
pub fn refusal(chr: &Character, npc_template: u32) -> Option<String> {
    refusal_for(&advancement_for(chr, npc_template))
}

// ---------------------------------------------------------------------------------------
// What the advancement grants
// ---------------------------------------------------------------------------------------

/// Everything the server owes a character the moment the advancement is accepted.
///
/// **Every field is idempotent or constant.** [`Grant::sp_total_owed`] is a *total owed*
/// rather than an increment - [`skillpoints`]'s whole design, and the reason re-sending this
/// cannot double-pay.
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
    /// `net::stats::tier_for_job` is `2 + (job % 10)` for a non-round job, and every third job
    /// ends in `1`, so this is **3** for all ten. **[D]** `FUN_1402CB030` returns `0` for any
    /// key above 10, so a job id here would read an empty pool and grey the `+` button with
    /// nothing on screen to say why.
    pub sp_pool_key: u8,
    /// **Skill points owed IN TOTAL to [`Grant::sp_tier`] at this level** - not an increment.
    pub sp_total_owed: u32,
    /// Flat max-HP points to add. Always [`MAX_HP_GAIN_POINTS`].
    pub max_hp_gain_points: u32,
    /// Flat max-MP points to add. Always [`MAX_MP_GAIN_POINTS`].
    pub max_mp_gain_points: u32,
    /// **Whether the `StatChange` SP encoding forks between the old job and the new one.**
    ///
    /// `false` for every second -> third advancement in this client. Read off
    /// `net::opcode::uses_extended_sp` rather than remembered: its `1|2` branch accepts
    /// `0 | 10..=12 | 20..=22 | 30..=32` and its `3..=5` branch accepts `0 | 10..=12 |
    /// 20..=22`, so every `x10 -> x11`, `x20 -> x21` and `x30 -> x31` pair is on the same
    /// side. **[D]**
    ///
    /// Carried rather than assumed, because sending the wrong shape desynchronises the whole
    /// packet rather than merely losing the points.
    pub sp_encoding_changes: bool,
}

/// What advancing to `job` at `level` grants, or `None` if `job` is not a third job.
///
/// Pure, and safe to call repeatedly: [`Grant::sp_total_owed`] is a total, so applying it
/// twice grants nothing the second time.
pub fn grant(job: u16, level: u32) -> Option<Grant> {
    let third = all().find(|t| t.job == job)?;
    Some(Grant {
        job: third.job,
        job_name: third.job_name,
        job_name_source: third.name_source,
        sp_tier: Tier::Third,
        sp_pool_key: net::stats::tier_for_job(job),
        sp_total_owed: skillpoints::entitlement(Tier::Third, level),
        max_hp_gain_points: MAX_HP_GAIN_POINTS,
        max_mp_gain_points: MAX_MP_GAIN_POINTS,
        sp_encoding_changes: crate::jobs::sp_encoding_changes(third.from_job, job),
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
            ..Default::default()
        }
    }

    /// **The ten jobs, the four NPCs, and the one-to-one map between second and third.**
    #[test]
    fn the_ten_third_jobs_and_their_four_instructors() {
        let jobs: Vec<u16> = all().map(|t| t.job).collect();
        assert_eq!(jobs, vec![111, 121, 131, 211, 221, 231, 311, 321, 411, 421]);

        let froms: Vec<u16> = all().map(|t| t.from_job).collect();
        assert_eq!(froms, vec![110, 120, 130, 210, 220, 230, 310, 320, 410, 420]);

        // One-to-one in both directions, which is the property that makes this a statement
        // rather than a menu.
        for t in all() {
            assert_eq!(third_job_from(t.from_job).map(|x| x.job), Some(t.job));
            assert_eq!(t.job, t.from_job + 1, "the client's own numbering");
        }
        assert_eq!(third_job_from(100), None, "a FIRST job advances at 511, not here");
        assert_eq!(third_job_from(0), None);
        assert_eq!(third_job_from(111), None, "and a third job advances nowhere");

        assert_eq!(MASTERS.map(|m| m.npc), [1104, 1105, 1106, 1107]);
        for m in MASTERS {
            assert!(master_at(m.npc).is_some());
            assert!(!m.says.is_empty(), "the evidence for the pairing travels with it");
        }
        // The NPCs that are NOT these four, including every other job NPC in the game.
        for other in [511, 313, 221, 411, 514, 319, 227, 424, 800_006, 101, 202, 1123, 1125] {
            assert!(master_at(other).is_none(), "NPC {other} is not a third-job instructor");
        }
    }

    /// **Seven grantable skills per book, for all ten** - three books carry an eighth that is
    /// invisible, and the filter is `secondjob`'s, not a second copy.
    #[test]
    fn every_book_grants_seven_and_the_hidden_three_are_filtered() {
        for t in all() {
            assert_eq!(t.grantable_count(), 7, "{} ({})", t.job_name, t.job);
        }
        let eight: Vec<u16> = all().filter(|t| t.skill_book.len() == 8).map(|t| t.job).collect();
        assert_eq!(eight, vec![211, 311, 321], "only these three have an eighth skill");
        for id in [2_111_006, 3_111_006, 3_211_006] {
            assert!(crate::secondjob::is_hidden(id), "{id} must be in HIDDEN_SKILLS");
            assert!(!all().any(|t| t.grantable_skills().any(|s| s == id)), "{id} is grantable");
        }
        // Every book id belongs to its own job, which is what says a book was not pasted into
        // the wrong row - the failure a table like this actually has.
        for t in all() {
            for id in t.skill_book {
                assert_eq!(
                    id / 10_000,
                    u32::from(t.job),
                    "skill {id} is in {}'s book",
                    t.job_name
                );
            }
        }
    }

    /// **The name provenance is asserted in both directions, and the invariant is `contains`.**
    ///
    /// Weaker than `secondjob`'s `starts_with` on purpose: `"Path of the Ranger"` and `"The Way
    /// of Hermit"` put the job name in the middle. Copying the stronger assertion across would
    /// fail on those two and the temptation would be to change the *names* rather than the
    /// assertion, which is how a table starts telling comfortable lies.
    #[test]
    fn job_names_carry_honest_provenance() {
        let mut from_book = 0;
        let mut ours = 0;
        for t in all() {
            match t.name_source {
                NameSource::BookName => {
                    from_book += 1;
                    assert!(
                        t.book_name.contains(t.job_name),
                        "{} claims the book name but {:?} is not in {:?}",
                        t.job,
                        t.job_name,
                        t.book_name
                    );
                }
                NameSource::NotInThisClient => {
                    ours += 1;
                    assert!(
                        !t.book_name.contains(t.job_name),
                        "{} claims to be ours but the book already says it",
                        t.job
                    );
                }
            }
        }
        assert_eq!((from_book, ours), (7, 3), "seven of ten are the client's");
        // And the three that are ours are the three Magician jobs - the same three as at
        // second job, which is a property of String.wz rather than of how hard anybody looked.
        let invented: Vec<u16> = all()
            .filter(|t| t.name_source == NameSource::NotInThisClient)
            .map(|t| t.job)
            .collect();
        assert_eq!(invented, vec![211, 221, 231]);
        // The two mid-string names, named so a future `starts_with` fails loudly here.
        assert!(!"Path of the Ranger".starts_with("Ranger"));
        assert!(!"The Way of Hermit".starts_with("Hermit"));
    }

    /// **The decision, arm by arm, and the order the refusals are said in.**
    #[test]
    fn the_advancement_decides_and_refuses_in_the_right_order() {
        // Eligible: a level-70 Fighter at Tylus becomes a Crusader and nothing else.
        match advancement_for(&character(70, 110), 1104) {
            Advancement::Eligible { third, master } => {
                assert_eq!(third.job, 111);
                assert_eq!(master.npc, 1104);
            }
            other => panic!("expected Eligible, got {other:?}"),
        }
        // 69 is not 70.
        assert!(matches!(
            advancement_for(&character(69, 110), 1104),
            Advancement::TooLowLevel { needed: 70, .. }
        ));
        // **Branch before level**: a level-1 Wizard at Tylus is told about the branch, because
        // "come back at 70" would be advice that never becomes true for them.
        assert!(matches!(
            advancement_for(&character(1, 210), 1104),
            Advancement::WrongBranch { .. }
        ));
        // A first-job or beginner character is not a refusal about branches.
        assert!(matches!(
            advancement_for(&character(70, 100), 1104),
            Advancement::NotSecondJobYet { job: 100, .. }
        ));
        assert!(matches!(
            advancement_for(&character(70, 0), 1104),
            Advancement::NotSecondJobYet { job: 0, .. }
        ));
        // One-way.
        assert!(matches!(
            advancement_for(&character(120, 111), 1104),
            Advancement::AlreadyAdvanced { job: 111 }
        ));
        // Every other NPC in the game falls through.
        assert_eq!(advancement_for(&character(70, 110), 511), Advancement::NotAnInstructor);
        assert_eq!(advancement_for(&character(70, 110), 9_999_999), Advancement::NotAnInstructor);

        // Every branch pairs with its own instructor and refuses the other three.
        for m in MASTERS {
            for t in m.serves {
                assert!(matches!(
                    advancement_for(&character(70, t.from_job), m.npc),
                    Advancement::Eligible { .. }
                ));
                for other in MASTERS.iter().filter(|o| o.npc != m.npc) {
                    assert!(
                        matches!(
                            advancement_for(&character(70, t.from_job), other.npc),
                            Advancement::WrongBranch { .. }
                        ),
                        "job {} must be refused by {}",
                        t.from_job,
                        other.name
                    );
                }
            }
        }
    }

    /// **Every refusal has words and every pass has none.**
    ///
    /// The second half is the one that matters: a caller that prints whatever `refusal_for`
    /// returns must not say anything on the arm that advances somebody.
    #[test]
    fn every_refusal_has_words_and_every_pass_has_none() {
        assert_eq!(refusal(&character(70, 110), 1104), None, "Eligible says nothing");
        assert_eq!(refusal(&character(70, 110), 511), None, "and neither does a stranger");
        for (level, job, npc) in
            [(69, 110, 1104), (70, 210, 1104), (70, 100, 1104), (70, 0, 1104), (99, 111, 1104)]
        {
            let text = refusal(&character(level, job), npc)
                .unwrap_or_else(|| panic!("level {level} job {job} must be answered"));
            assert!(!text.is_empty());
        }
        // The level refusal names the number, so the advice is actionable.
        let text = refusal(&character(69, 110), 1104).unwrap();
        assert!(text.contains("70"), "got {text:?}");
        // The wrong-branch refusal names the branch, not the level.
        let text = refusal(&character(1, 210), 1104).unwrap();
        assert!(text.contains("Warrior"), "got {text:?}");
        assert!(!text.contains("70"), "and does NOT give useless advice: {text:?}");
    }

    /// **What the grant is: tier 3, a total, no fork, and no invented HP.**
    #[test]
    fn the_grant_is_tier_three_and_idempotent() {
        for t in all() {
            let g = grant(t.job, 70).expect("every third job grants");
            assert_eq!(g.job, t.job);
            assert_eq!(g.sp_tier, Tier::Third);
            assert_eq!(g.sp_pool_key, 3, "{} must land in pool 3", t.job);
            assert!(!g.sp_encoding_changes, "{} -> {} must not fork the SP shape", t.from_job, t.job);
            assert_eq!(g.max_hp_gain_points, 0);
            assert_eq!(g.max_mp_gain_points, 0);
        }
        // Controls for the pool key, so "3" is a measurement of `tier_for_job` and not a
        // constant this file happens to agree with.
        assert_eq!(net::stats::tier_for_job(0), 0);
        assert_eq!(net::stats::tier_for_job(100), 1);
        assert_eq!(net::stats::tier_for_job(110), 2);
        assert_eq!(net::stats::tier_for_job(111), 3);

        // A total, not an increment: granting twice owes nothing the second time.
        let g = grant(111, 75).unwrap();
        assert_eq!(g.sp_total_owed, skillpoints::entitlement(Tier::Third, 75));
        assert_eq!(skillpoints::top_up(Tier::Third, 75, g.sp_total_owed), 0);
        // And below the minimum it is zero rather than negative.
        assert_eq!(grant(111, 69).unwrap().sp_total_owed, 0);
        assert_eq!(grant(111, 70).unwrap().sp_total_owed, 1);
        assert_eq!(grant(111, 71).unwrap().sp_total_owed, 4);

        // Not a third job, no grant - including the second job it comes from.
        assert!(grant(110, 70).is_none());
        assert!(grant(100, 70).is_none());
        assert!(grant(0, 70).is_none());
    }

    // -----------------------------------------------------------------------------------
    // Against the client's own generated dumps
    // -----------------------------------------------------------------------------------

    /// Every row of `gm-handbook/skills.txt` as `(skillId, jobBook, invisible)`, or `None` on a
    /// clean checkout where `gm-handbook/` has not been generated.
    ///
    /// **`invisible` is column index 24**, which is `$25` to `awk`. That off-by-one produced a
    /// clean, confident and wrong "no third-job skill is invisible" during the research pass,
    /// and it was caught only because a control already knew the answer was 13.
    fn real_skill_rows() -> Option<Vec<(u32, u16, bool)>> {
        let path = Path::new("../../gm-handbook/skills.txt");
        if !path.exists() {
            return None;
        }
        let text = std::fs::read_to_string(path).ok()?;
        const COLUMNS: usize = 98;
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
            let (Ok(id), Ok(job)) = (f[0].parse::<u32>(), f[1].parse::<u16>()) else {
                continue;
            };
            out.push((id, job, f[24] == "1"));
        }
        Some(out)
    }

    /// **The ten books are exactly what this client ships**, and the eighth skills are exactly
    /// the invisible ones.
    ///
    /// `gm-handbook/` is generated and gitignored, so this degrades to a no-op on a clean
    /// checkout - the shape of check `CLAUDE.md` warns can be silently vacuous. It therefore
    /// asserts a **positive control first**.
    #[test]
    fn every_third_job_book_is_exactly_the_client_s_own() {
        let Some(rows) = real_skill_rows() else { return };
        let ids: BTreeSet<u32> = rows.iter().map(|r| r.0).collect();
        assert_eq!(ids.len(), 176, "positive control: the file holds all 176 skills");

        let mut by_book: BTreeMap<u16, BTreeSet<u32>> = BTreeMap::new();
        for (id, job, _) in &rows {
            by_book.entry(*job).or_default().insert(*id);
        }
        for t in all() {
            let want: BTreeSet<u32> = t.skill_book.iter().copied().collect();
            let got = by_book
                .get(&t.job)
                .unwrap_or_else(|| panic!("book {} is missing from the archive", t.job));
            assert_eq!(got, &want, "book {} ({})", t.job, t.job_name);
        }
        // The invisible set, restricted to third-job books, read off the file rather than
        // transcribed - and it must be exactly the three eighth skills.
        let invisible: BTreeSet<u32> = rows
            .iter()
            .filter(|(_, job, inv)| *inv && all().any(|t| t.job == *job))
            .map(|(id, _, _)| *id)
            .collect();
        assert_eq!(
            invisible,
            BTreeSet::from([2_111_006, 3_111_006, 3_211_006]),
            "the invisible third-job skills, from the file"
        );
    }

    /// **The four instructors stand where this table says**, and each is placed exactly once.
    ///
    /// Reads `Map.wz`'s `life` nodes and `String.wz/Npc.img` - two different archives - so a
    /// wrong template id fails here rather than as a click that says nothing on the owner's screen.
    #[test]
    fn the_four_instructors_are_placed_where_this_says() {
        let npcs = Path::new("../../gm-handbook/npcs.txt");
        let strings = Path::new("../../gm-handbook/npcstrings.txt");
        if !npcs.exists() || !strings.exists() {
            return;
        }
        let npc_text = std::fs::read_to_string(npcs).expect("npcs.txt");
        let rows: Vec<Vec<String>> = npc_text
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .map(|l| l.split(',').map(|c| c.trim().to_string()).collect())
            .collect();
        assert!(rows.len() > 250, "positive control: {} npc placements", rows.len());
        // Control: an NPC known to be placed once, from a different feature.
        assert_eq!(rows.iter().filter(|r| r[1] == "101").count(), 1, "Phil is placed once");

        let str_text = std::fs::read_to_string(strings).expect("npcstrings.txt");
        let mut names: BTreeMap<u32, String> = BTreeMap::new();
        let mut idle0: BTreeMap<u32, String> = BTreeMap::new();
        for line in str_text.lines() {
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() != 3 {
                continue;
            }
            let Ok(id) = f[0].parse::<u32>() else { continue };
            match f[1] {
                "name" => {
                    names.insert(id, f[2].to_string());
                }
                "idle0" => {
                    idle0.insert(id, f[2].to_string());
                }
                _ => {}
            }
        }
        assert!(names.len() > 200, "positive control: {} npc names", names.len());

        for m in MASTERS {
            let t = m.npc.to_string();
            let placements: Vec<&Vec<String>> = rows.iter().filter(|r| r[1] == t).collect();
            assert_eq!(placements.len(), 1, "{} ({}) is placed once", m.name, m.npc);
            assert_eq!(
                placements[0][0], INSTRUCTOR_MAP.to_string(),
                "{} stands in Chief's Residence",
                m.name
            );
            assert_eq!(names.get(&m.npc).map(String::as_str), Some(m.name));
            // **The evidence, checked.** `Master::says` is the reason this file pairs Rene with
            // the Bowman line rather than with Helena, who does not exist here. If the quoted
            // line ever stops matching the archive, the pairing has lost its support.
            assert_eq!(
                idle0.get(&m.npc).map(String::as_str),
                Some(m.says),
                "{}'s idle0 is the evidence for their branch",
                m.name
            );
            assert!(
                m.says.to_lowercase().contains(&m.branch_name.to_lowercase()),
                "{}'s own line must name the {} branch: {:?}",
                m.name,
                m.branch_name,
                m.says
            );
        }
    }

    /// **There is no third-job quest in this client**, and the check knows what one looks like.
    ///
    /// This is the negative the whole module's design rests on: if a chain existed, the gate
    /// should be that chain rather than a bare level. The positive control is the *second*-job
    /// chain, which the same query finds immediately.
    #[test]
    fn this_client_ships_no_third_job_quest() {
        let path = Path::new("../../gm-handbook/questlines.txt");
        if !path.exists() {
            return;
        }
        let text = std::fs::read_to_string(path).expect("questlines.txt");
        let ids: BTreeSet<u32> = text
            .lines()
            .filter(|l| !l.starts_with('#'))
            .filter_map(|l| l.split('\t').next()?.parse::<u32>().ok())
            .collect();
        assert_eq!(ids.len(), 322, "positive control: all 322 quests loaded");
        // The control: the second-job chain IS there, so an empty third-job result is a
        // measurement rather than a failed parse.
        for q in [20000, 20003, 20100, 20303] {
            assert!(ids.contains(&q), "positive control: quest {q} is a second-job quest");
        }
        let third: Vec<u32> = ids.iter().copied().filter(|q| (20304..80000).contains(q)).collect();
        assert!(third.is_empty(), "no quest exists between the 2nd-job chain and the PQ: {third:?}");
    }
}
