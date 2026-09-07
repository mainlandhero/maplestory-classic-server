//! First job advancement at level 10 - goal **E**.
//!
//! Full working, with every address, every enumeration and every named negative:
//! **`research/job-advancement.md`**.
//!
//! Labels are the project's: **[L]** read off this client's listing, its WZ or a capture,
//! **[D]** derived from two or more [L] facts, **[I]** inferred - a policy nothing on this
//! machine can confirm.
//!
//! # Nothing here authenticates
//!
//! As everywhere in this project, the channel socket carries no credentials. A job
//! advancement is granted on the say-so of whoever holds the socket.
//!
//! # What this module is, and what it deliberately is not
//!
//! It is the **pure decision**: given a character and the NPC they clicked, may they
//! advance, and to what. It sends no packets, touches no database and knows nothing about
//! sessions, so it can be tested without a client and without a server.
//!
//! It is **not** wired. `crates/world/src/session/` belongs to the coordinator and was not
//! touched; `research/job-advancement.md` §8 is the "WIRE IT LIKE THIS". Saying so out loud
//! is `CLAUDE.md`'s "built is not wired" rule, which has caught two subsystems that
//! `STATUS.md` listed as done while nothing called them.
//!
//! # The three facts this file exists to keep in one place
//!
//! 1. **The four first jobs are 100 / 200 / 300 / 400.** **[L]** from this client:
//!    `net::opcode::uses_extended_sp` decodes the SP fork's literal bit masks in
//!    `FUN_140302e30` and they give exactly the explorer tree.
//! 2. **The four instructors are 511 / 313 / 221 / 411**, and they are **not standing in the
//!    towns** - they are one map inside. **[L]** from `String.wz/Npc.img` and the `life`
//!    nodes of `Map.wz`, via `gm-handbook/npcstrings.txt` and `gm-handbook/npcs.txt`.
//! 3. **The 35-stat prerequisite is [I] and unverifiable.** It is the owner's number, from a fan
//!    site, and there is nothing in this client to check it against - see
//!    [`STAT_MINIMUM`]. That is why it is *one constant* rather than four literals.
//!
//! # The correction that matters most
//!
//! The `Test of Qualification` / `Proof of Qualification` quests (20002/20003 and the three
//! siblings) are **the second job advancement, at level 30**, not the first. Their own
//! `Check.0.job.0` is `100`/`200`/`300`/`400` - i.e. you must *already* be a first-job
//! character to start them - their `Check.0.lvmin` is `30`, and the client's own quest text
//! says *"the 2nd job advancement"* in as many words. **[L]** from
//! `gm-handbook/questlines.txt`.
//!
//! **The first advancement has no quest at all.** All 322 quests were enumerated: there is
//! no `Act.<n>.job` key anywhere in the tree, so no quest in this client can change a job
//! even in principle. `research/job-advancement.md` §3 has the enumeration and its controls.

use net::opcode::Character;

// ---------------------------------------------------------------------------------------
// The two numbers that are policy, in one named place each
// ---------------------------------------------------------------------------------------

/// The level at which the first job advancement becomes available. **The owner's rule**, and it
/// is corroborated by the client's own words rather than only by them:
/// `Quest.wz/QuestData/10001.img` ("Phil's Call") carries `Check.1.lvmin = 10` and says
/// *"once you reach Level 10, you can make a job advancement in each town of Victoria
/// Island"*. **[L]** for the text, **[D]** for the rule.
pub const LEVEL_MINIMUM: u32 = 10;

/// **The prerequisite stat value. `[I]`, and nothing on this machine can check it.**
///
/// The owner, 2026-08-19: *"Each job has a pre-requisite, which I assume is widely available on
/// the internet. Magician 35 INT, Warrior 35 STR, Thief 35 LUK, Bowman 35 DEX."* They flagged
/// it as fan-site sourced themself.
///
/// **Revisited 2026-08-27 and left at 35.** The owner asked for 25 - *"at least 25 ability points
/// in that job branch's main stat"* - and then, told where the 35 came from, said *"If the fan
/// site says 35, let's go with 35."* Written down because the number has now been questioned
/// once, cannot be arbitrated by anything on this machine, and would otherwise be questioned
/// again.
///
/// # Why it is a constant and not four literals
///
/// A scan of every `Check` node in all 322 quests finds **no `int`, `str`, `dex` or `luk`
/// requirement anywhere** - and that is a *verified* negative rather than a failed search:
/// the same scan enumerates 32 other `Check` key shapes, including `lvmin` (406 uses), `job`
/// (104), `skill` (48) and `item` (798). The requirement is not in `Quest.wz`. **[L]**
///
/// So **the client will never contradict this number and nothing will catch it being
/// wrong.** It is one edit, here, exactly as `STATUS.md` goal E asks.
pub const STAT_MINIMUM: u16 = 35;

// ---------------------------------------------------------------------------------------
// The job table
// ---------------------------------------------------------------------------------------

/// Which ability score a first job asks for.
///
/// The *pairing* of stat to job is the uncontroversial half - a Magician wanting INT is not
/// a number anyone has to source - but the pairing is still **[I]** here, because it arrives
/// from the same sentence as [`STAT_MINIMUM`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stat {
    Strength,
    Dexterity,
    Intelligence,
    Luck,
}

impl Stat {
    /// The character's current value of this stat.
    pub fn of(self, chr: &Character) -> u16 {
        match self {
            Stat::Strength => chr.strength,
            Stat::Dexterity => chr.dexterity,
            Stat::Intelligence => chr.intelligence,
            Stat::Luck => chr.luck,
        }
    }

    /// What the client's own UI calls it, for a refusal sentence the player can act on.
    pub fn label(self) -> &'static str {
        match self {
            Stat::Strength => "STR",
            Stat::Dexterity => "DEX",
            Stat::Intelligence => "INT",
            Stat::Luck => "LUK",
        }
    }
}

/// One first-job advancement: the instructor, the job they grant, and where they stand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirstJob {
    /// `String.wz/Npc.img` template id. **[L]**
    pub npc_template: u32,
    /// The instructor's name, verbatim from `String.wz/Npc.img/<template>/name`. **[L]**
    pub npc_name: &'static str,
    /// The job id the advancement grants. **[L]**, from the client's own SP fork masks.
    pub job: u16,
    /// What the client's quest text calls a character of this job on advancement -
    /// `Quest.wz/QuestData/10001.img/Say/1/yes/0` names all four. **[L]**
    pub job_name: &'static str,
    /// The stat the advancement asks for. **[I]** - see [`STAT_MINIMUM`].
    pub stat: Stat,
    /// The map the instructor is **actually on**, from `Map.wz`'s `life` nodes via
    /// `gm-handbook/npcs.txt`. **[L]**
    ///
    /// **This is not the town.** Every one of the four is one map inside it, and clicking
    /// the town id in `!map` finds nobody - which is precisely the kind of detail that
    /// costs one of the owner's manual launches.
    pub map_id: u32,
    /// [`Self::map_id`]'s name from `gm-handbook/maps.txt`. **[L]**
    pub map_name: &'static str,
    /// The town the client's own text sends the player to. **[L]** from
    /// `QuestData/10001.img`. Kept beside [`Self::map_id`] so the difference is visible
    /// rather than surprising.
    pub town_id: u32,
    /// The instructor's own idle line, verbatim from `String.wz/Npc.img/<template>/d0`.
    /// **[L]** - and it is *all* the words this client has for them; see
    /// `research/job-advancement.md` §6.
    pub idle_line: &'static str,
}

/// The four first job advancements.
///
/// Ordered by job id, which is also the order the client's own `10001` text lists them in
/// (*"#rSwordsman#k, #rMagician#k, #rArcher#k, or #rRogue#k"*).
pub const FIRST_JOBS: [FirstJob; 4] = [
    FirstJob {
        npc_template: 511,
        npc_name: "Dances with Balrog",
        job: 100,
        job_name: "Swordsman",
        stat: Stat::Strength,
        map_id: 10004003,
        map_name: "Warriors' Sanctuary",
        town_id: 10004000,
        idle_line: "Those who want to become a warrior, come see me...",
    },
    FirstJob {
        npc_template: 313,
        npc_name: "Grendel the Really Old",
        job: 200,
        job_name: "Magician",
        stat: Stat::Intelligence,
        map_id: 10002003,
        map_name: "Magic Library",
        town_id: 10002000,
        idle_line: "All who desire to become a magician, talk to me...",
    },
    FirstJob {
        npc_template: 221,
        npc_name: "Athena Pierce",
        job: 300,
        job_name: "Archer",
        stat: Stat::Dexterity,
        map_id: 10001051,
        map_name: "Bowman Instructional School",
        town_id: 10001000,
        idle_line: "Those who want to become a bowman... Talk to me...",
    },
    FirstJob {
        npc_template: 411,
        npc_name: "Dark Lord",
        job: 400,
        job_name: "Rogue",
        stat: Stat::Luck,
        map_id: 10003003,
        map_name: "Thieves' Hideout",
        town_id: 10003000,
        idle_line: "Those that want to be a thief, come...",
    },
];

/// What to call a character of `job`, or `None` for an id no table here knows.
///
/// The owner, 2026-09-06: *"!job <id> should just say Cobalt is now a <Job Name>, such as Cobalt is
/// now a Swordsman."* Beginner is `0`; the first jobs come from [`FIRST_JOBS`], the second
/// from `secondjob::second_job` and the third from `thirdjob::all` - each carries its own
/// provenance note on the `job_name` field, and a second-job name is the client's **book**
/// name ("Fighter" from "Fighter Techniques") rather than a job string this client ships.
pub fn job_name(job: u16) -> Option<&'static str> {
    if job == 0 {
        return Some("Beginner");
    }
    FIRST_JOBS
        .iter()
        .find(|j| j.job == job)
        .map(|j| j.job_name)
        .or_else(|| crate::secondjob::second_job(job).map(|j| j.job_name))
        .or_else(|| crate::thirdjob::all().find(|j| j.job == job).map(|j| j.job_name))
}

/// The advancement an NPC template grants, if it grants one.
///
/// **The four `<Job> Job Instructor` NPCs - 514, 319, 227, 424 - are deliberately absent.**
/// They are the *second* advancement's examiners: `Quest.wz` puts them on `Check.0.npc` of
/// the four `Test of Qualification` quests, whose `Check.0.job.0` is already `100`..`400`
/// and whose `Check.0.lvmin` is `30`. **[L]** So a beginner clicking one of them is not
/// asking for a first advancement, and answering as though they were would advance a
/// character in the wrong place.
pub fn first_job_at(npc_template: u32) -> Option<&'static FirstJob> {
    FIRST_JOBS.iter().find(|j| j.npc_template == npc_template)
}

/// Whether a job id is one of the four first jobs.
pub fn is_first_job(job: u16) -> bool {
    FIRST_JOBS.iter().any(|j| j.job == job)
}

// ---------------------------------------------------------------------------------------
// The decision
// ---------------------------------------------------------------------------------------

/// What clicking an instructor should do.
///
/// Every arm carries the numbers the refusal sentence needs, so the caller never has to
/// re-derive them and the two cannot drift apart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Advancement {
    /// This NPC does not advance anybody. The caller should fall through to whatever it
    /// does for an ordinary NPC.
    NotAnInstructor,
    /// The character already has a job. **Advancement is one-way** - the client's own
    /// `10001` text says *"a job advancement cannot be undone once made"* **[L]** - so this
    /// is a refusal, not a re-offer.
    AlreadyAdvanced { job: u16 },
    /// Below [`LEVEL_MINIMUM`].
    TooLowLevel { level: u32, needed: u32 },
    /// At level, but short on the stat. Carries the stat so the sentence can name it.
    StatTooLow { stat: Stat, have: u16, needed: u16 },
    /// Everything passes. `job` is what to store and what to put in the `0x007C`.
    Eligible { job: u16, job_name: &'static str },
}

/// May this character advance by talking to this NPC, and to what?
///
/// **Pure.** No database, no packets, no clock.
///
/// The order of the checks is the order the refusals should be *said* in: an eighth-level
/// beginner is told to come back at ten, not that their LUK is short, because the second
/// sentence is useless advice while the first is still true.
pub fn advancement_for(chr: &Character, npc_template: u32) -> Advancement {
    let Some(first) = first_job_at(npc_template) else {
        return Advancement::NotAnInstructor;
    };
    if chr.job != 0 {
        return Advancement::AlreadyAdvanced { job: chr.job };
    }
    if chr.level < LEVEL_MINIMUM {
        return Advancement::TooLowLevel { level: chr.level, needed: LEVEL_MINIMUM };
    }
    let have = first.stat.of(chr);
    if have < STAT_MINIMUM {
        return Advancement::StatTooLow { stat: first.stat, have, needed: STAT_MINIMUM };
    }
    Advancement::Eligible { job: first.job, job_name: first.job_name }
}

/// The sentence to put in the refusal box, or `None` when the character may advance.
///
/// **A refusal is still an answer.** `CLAUDE.md`'s first expensive rule is that an
/// unanswered packet freezes the client's entire UI; a script box that says no keeps the
/// conversation alive and ends it cleanly.
pub fn refusal(chr: &Character, npc_template: u32) -> Option<String> {
    match advancement_for(chr, npc_template) {
        Advancement::NotAnInstructor | Advancement::Eligible { .. } => None,
        Advancement::AlreadyAdvanced { .. } => {
            Some("You have already chosen your path. It cannot be undone.".to_string())
        }
        Advancement::TooLowLevel { level, needed } => Some(format!(
            "Come back when you have reached Level {needed}. You are only Level {level}."
        )),
        Advancement::StatTooLow { stat, have, needed } => Some(format!(
            "You are not ready. You need {needed} {} and you have {have}.",
            stat.label()
        )),
    }
}

// ---------------------------------------------------------------------------------------
// The one thing about the wire that this module has to own
// ---------------------------------------------------------------------------------------

/// Whether changing `from` to `to` changes which SP encoding the client will read.
///
/// **This is the trap that would silently desynchronise the stat block**, and it is checked
/// here so a future job table cannot introduce it without a test failing.
///
/// `FUN_1402cbb50` reads the job **back out of the character record** at `+0x33` and forks
/// the SP field on it (`net::opcode::uses_extended_sp`). Job `0` and all four first jobs are
/// on the *same* side of that fork - all extended - so a first advancement does **not**
/// change the encoding, and `net::stats::Sp::empty_extended()` stays correct across it.
/// **[L]** from `research/charstat-layout.md` §on the SP fork.
///
/// It would not stay correct across every advancement: job `101`, for instance, takes the
/// plain-`u16` branch. This function is what a second-job table would have to consult.
pub fn sp_encoding_changes(from: u16, to: u16) -> bool {
    net::opcode::uses_extended_sp(from) != net::opcode::uses_extended_sp(to)
}

// ---------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn beginner(level: u32) -> Character {
        Character {
            // 200 is FIRST_CHARACTER_ID. A record renumbered from 1 has bitten this project
            // before, so the fixtures here use a real id rather than a small one.
            id: 200,
            level,
            job: 0,
            strength: 4,
            dexterity: 4,
            intelligence: 4,
            luck: 4,
            ..Character::default()
        }
    }

    /// **The numbers the whole feature rests on.** If a later edit renumbers a job or moves
    /// an instructor, this fails here rather than on the owner's screen.
    #[test]
    fn the_four_first_jobs_are_the_ones_the_client_names() {
        let ids: Vec<u16> = FIRST_JOBS.iter().map(|j| j.job).collect();
        assert_eq!(ids, vec![100, 200, 300, 400]);
        let npcs: Vec<u32> = FIRST_JOBS.iter().map(|j| j.npc_template).collect();
        assert_eq!(npcs, vec![511, 313, 221, 411]);
    }

    /// Every first job takes the **extended** SP branch, and so does job 0. That is what
    /// makes a first advancement safe to send without touching the SP encoding.
    #[test]
    fn a_first_advancement_does_not_change_the_sp_encoding() {
        assert!(net::opcode::uses_extended_sp(0), "job 0 is extended");
        for j in FIRST_JOBS {
            assert!(net::opcode::uses_extended_sp(j.job), "job {} is extended", j.job);
            assert!(!sp_encoding_changes(0, j.job), "0 -> {} must not fork", j.job);
        }
        // The control: a job that is genuinely on the other side, so this test is not
        // asserting that `sp_encoding_changes` always returns false.
        assert!(!net::opcode::uses_extended_sp(101));
        assert!(sp_encoding_changes(100, 101), "100 -> 101 DOES change the encoding");
    }

    /// The four second-job examiners must not be mistaken for first-job instructors.
    /// `Quest.wz` gates their quests on `job.0` 100..400 and `lvmin` 30. **[L]**
    #[test]
    fn the_second_job_examiners_are_not_in_the_table() {
        for t in [514u32, 319, 227, 424, 800003, 800004] {
            assert!(first_job_at(t).is_none(), "template {t} must not advance anyone");
        }
        // And the positive control, so the negative above is not a property of the search.
        for t in [511u32, 313, 221, 411] {
            assert!(first_job_at(t).is_some(), "template {t} must advance someone");
        }
    }

    /// The instructors are **not** in the towns. This is a test rather than a comment
    /// because getting it wrong costs a manual launch.
    #[test]
    fn every_instructor_is_one_map_inside_the_town() {
        for j in FIRST_JOBS {
            assert_ne!(j.map_id, j.town_id, "{} stands in {}, not {}", j.npc_name, j.map_id, j.town_id);
        }
        assert_eq!(first_job_at(511).unwrap().map_id, 10004003);
        assert_eq!(first_job_at(313).unwrap().map_id, 10002003);
        assert_eq!(first_job_at(221).unwrap().map_id, 10001051);
        assert_eq!(first_job_at(411).unwrap().map_id, 10003003);
    }

    /// Each instructor asks for their own stat and no other.
    #[test]
    fn each_instructor_asks_for_its_own_stat() {
        let mut chr = beginner(10);
        chr.strength = STAT_MINIMUM;
        assert_eq!(
            advancement_for(&chr, 511),
            Advancement::Eligible { job: 100, job_name: "Swordsman" }
        );
        // The same character is refused by the other three, each naming a different stat.
        for (npc, stat) in [(313, Stat::Intelligence), (221, Stat::Dexterity), (411, Stat::Luck)] {
            assert_eq!(
                advancement_for(&chr, npc),
                Advancement::StatTooLow { stat, have: 4, needed: STAT_MINIMUM },
                "npc {npc}"
            );
        }
    }

    /// Level is checked before the stat, because "come back at 10" is the useful sentence
    /// while it is still true.
    #[test]
    fn level_is_refused_before_the_stat() {
        let chr = beginner(9);
        assert_eq!(
            advancement_for(&chr, 511),
            Advancement::TooLowLevel { level: 9, needed: 10 }
        );
        // Exactly at the boundary the level stops being the complaint.
        let mut ten = beginner(10);
        assert!(matches!(advancement_for(&ten, 511), Advancement::StatTooLow { .. }));
        ten.strength = STAT_MINIMUM - 1;
        assert_eq!(
            advancement_for(&ten, 511),
            Advancement::StatTooLow { stat: Stat::Strength, have: 34, needed: 35 },
            "34 is short by one"
        );
        ten.strength = STAT_MINIMUM;
        assert!(matches!(advancement_for(&ten, 511), Advancement::Eligible { .. }));
    }

    /// Advancement is one-way: the client's own text says it cannot be undone. **[L]**
    #[test]
    fn an_advanced_character_is_refused_by_everyone() {
        let mut chr = beginner(50);
        chr.strength = 999;
        chr.dexterity = 999;
        chr.intelligence = 999;
        chr.luck = 999;
        chr.job = 100;
        for j in FIRST_JOBS {
            assert_eq!(
                advancement_for(&chr, j.npc_template),
                Advancement::AlreadyAdvanced { job: 100 },
                "{} must refuse an advanced character", j.npc_name
            );
        }
    }

    /// A non-instructor is not refused - it is *not this module's business*, which is a
    /// different thing and the caller must fall through rather than say no.
    #[test]
    fn an_ordinary_npc_is_not_this_modules_business() {
        let chr = beginner(10);
        // Roger (3), Phil (101), Olaf (107), Lucy the shopkeeper (21).
        for t in [3u32, 101, 107, 21] {
            assert_eq!(advancement_for(&chr, t), Advancement::NotAnInstructor);
            assert_eq!(refusal(&chr, t), None, "template {t} must not produce a refusal");
        }
    }

    /// Every refusal produces a sentence, and an eligible character produces none. The
    /// "always answer" rule reaches into this module as "always have something to say".
    #[test]
    fn every_refusal_has_words() {
        assert!(refusal(&beginner(1), 511).unwrap().contains("Level 10"));
        let mut short = beginner(10);
        short.strength = 30;
        let s = refusal(&short, 511).unwrap();
        assert!(s.contains("35") && s.contains("STR") && s.contains("30"), "{s}");
        short.strength = STAT_MINIMUM;
        assert_eq!(refusal(&short, 511), None, "an eligible character is not refused");
    }

    /// `is_first_job` agrees with the table, and says no to the second-job ids.
    #[test]
    fn is_first_job_knows_the_second_job_ids_are_not_first() {
        for j in [100u16, 200, 300, 400] {
            assert!(is_first_job(j));
        }
        for j in [0u16, 110, 120, 130, 210, 310, 410, 500] {
            assert!(!is_first_job(j), "job {j} is not a FIRST job");
        }
    }

    /// The prerequisite lives in exactly one place. This test is the thing that makes
    /// "one edit to change" true rather than aspirational.
    #[test]
    fn the_prerequisite_is_one_constant() {
        assert_eq!(STAT_MINIMUM, 35, "[I] - the owner's number, unverifiable against this client");
        assert_eq!(LEVEL_MINIMUM, 10);
        let mut chr = beginner(LEVEL_MINIMUM);
        for j in FIRST_JOBS {
            // Set only this job's own stat to the minimum and expect exactly this job.
            let mut c = chr.clone();
            match j.stat {
                Stat::Strength => c.strength = STAT_MINIMUM,
                Stat::Dexterity => c.dexterity = STAT_MINIMUM,
                Stat::Intelligence => c.intelligence = STAT_MINIMUM,
                Stat::Luck => c.luck = STAT_MINIMUM,
            }
            assert_eq!(
                advancement_for(&c, j.npc_template),
                Advancement::Eligible { job: j.job, job_name: j.job_name }
            );
        }
        chr.level = 0;
        assert!(matches!(advancement_for(&chr, 511), Advancement::TooLowLevel { .. }));
    }

    /// **The reachability arithmetic, which decides whether the feature is testable at
    /// all.** A character is created with a client-rolled 25 points across four stats and
    /// gains 5 AP a level (`expcurve::LevelGains`), so level 10 carries 25 + 9*5 = 70
    /// points. Even a stat that started at the floor can pass 35.
    ///
    /// That is the *arithmetic*. Whether the owner can perform it is a different question, and
    /// when this was written the answer was no: there is no GM command that sets a stat,
    /// and a sibling agent's AP-allocation handlers existed but were not yet dispatched
    /// from `session/mod.rs`. **Re-check the dispatcher rather than trusting that** -
    /// `research/job-advancement.md` §9.1 says how.
    #[test]
    fn thirty_five_is_reachable_by_level_ten_on_paper() {
        const AP_PER_LEVEL: u16 = 5; // world::expcurve::LevelGains::ap
        let ap_by_ten = AP_PER_LEVEL * u16::try_from(LEVEL_MINIMUM - 1).unwrap();
        assert_eq!(ap_by_ten, 45);
        // The worst case: the client rolled this stat down to 4 and every point since has
        // to come from AP.
        assert!(4 + ap_by_ten >= STAT_MINIMUM, "4 + {ap_by_ten} must reach {STAT_MINIMUM}");
    }
}
