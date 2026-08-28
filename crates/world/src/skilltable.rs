//! What each job may learn and how far - and what each *level* of a skill costs and does.
//!
//! The owner, 2026-08-27: *"I want to verify that all Magician 1st job skills are working first."*
//!
//! # This table is the thing `session::skills` said it was waiting for
//!
//! That handler refused every skill outside the three beginner ids, and its comment said why:
//! *"Refusing anything else is not a rule of the game - it is a refusal to invent one, since
//! what a job may learn is `Skill.wz` data nobody has read."* It has now been read.
//! `tools/dump_skills.py` writes `gm-handbook/skills.txt` - 176 skills, 4 164 skill-levels -
//! and this loads it.
//!
//! # Two layers, because a grant and a cast ask different questions
//!
//! | | question | type |
//! |---|---|---|
//! | [`Skill`] | may this job learn it, and how far? | one per skill id |
//! | [`SkillLevel`] | what does level `n` cost, and what numbers does it carry? | one per skill **level** |
//!
//! The generated file has **98** columns and one row per skill *level*, so the same skill id
//! appears `maxLevel` times. [`Skill`] keeps the columns that are constant across those rows;
//! [`SkillLevel`] keeps the ones that vary. Levels are stored in a vector indexed by level, but
//! that indexing is private - [`Skill::level`] takes the level as the client numbers it, `1`
//! for the first, so no caller can get the off-by-one wrong.
//!
//! # The units. Read this section before using any number below
//!
//! `research/magician-first-job.md` §6 measured every one of these against the client's own
//! tooltip text, which `gm-handbook/skills.txt` carries as its last column. They are not what a
//! reader would assume, and `CLAUDE.md`'s *"the unit, not the arithmetic"* records three bugs
//! this month that were a correct number in the wrong unit.
//!
//! * **`time` and `cooltime` are SECONDS.** `net::buff::TemporaryStat::duration_ms` is
//!   milliseconds. **This file does not convert**, and the fields are named
//!   [`SkillLevel::time_seconds`] and [`SkillLevel::cooltime_seconds`] so that a caller cannot
//!   mistake them. `net::buff::BuffLevel::granted_by` documents itself as *"the one place that
//!   conversion happens"*; a second multiplication site is exactly how a duration ends up
//!   1000x wrong, so there is deliberately not one here.
//! * **`mastery` is a LEVEL 1..10 in this build**, not the 10..60 percentage the same property
//!   is in other MapleStory versions. The client's own tooltip says *"Mastery level 3"*. The
//!   field is [`SkillLevel::mastery_level`] and the archive-wide range is asserted in the tests.
//!   What a mastery level does to the damage spread is **not in `Skill.wz` at all**.
//! * **`x` and `y` mean a different thing on every skill, and the WZ never says which.** Magic
//!   Guard's `x` is a percentage of incoming damage; Improved MP Recovery's `x` is a percentage
//!   of max MP; Flash Jump's `x` is a distance; a Booster's `x` is **negative** (`-1`, `-2`).
//!   They are carried raw, they are **signed**, and they are deliberately not named for one
//!   skill's meaning. Reading an `x` without reading that skill's tooltip is unverified.
//! * **`attackCount` is the number of hits per cast.** Magic Claw's is **2**, so its `mad` of
//!   45 is **per hit, not per cast**. A consumer that applies `mad` once produces half the
//!   damage and reads on screen as a formula bug rather than a count bug.
//! * **`mad` is the tooltip's "Basic Attack" number.** That it is a percentage multiplier is
//!   **[I]** - inferred by analogy with `damage` on the physical skills. No tooltip says so and
//!   no magic formula has been decoded. Report the number; do not assume the multiply.
//! * `mpCon` is flat MP points, `indiePdd`/`indieMdd` are flat defence points, `mmpR` is a
//!   **percent** of max MP (the `R` suffix means ratio), and `speed` is the Nimble Feet
//!   control's value.
//!
//! # An empty cell is not a zero
//!
//! Every per-level number is an [`Option`]. `None` means **the property is absent from the WZ
//! node**, which is a different fact from the property being zero, and here the difference is
//! load-bearing rather than tidy: **Magic Guard carries no `time` at any of its 15 levels.**
//! `research/magician-first-job.md` §4 shows all 16 `processtype 113` skills behave that way
//! and reads it as a toggle. A `u32` defaulting to `0` would turn that absence into a
//! zero-second buff, and nobody has measured what the client does with `duration_ms = 0`.
//!
//! No cell in the file as generated today is a *present* zero, so `None` and `Some(0)` cannot
//! currently be told apart by looking at the data - the distinction comes from the WZ's own
//! semantics, not from a measurement of this file.
//!
//! # Two things the generated file does that a naive parser gets wrong
//!
//! * **1 499 numeric cells in the WZ archive are quoted strings, not bare integers** - `mpCon`
//!   on 61 level nodes, `x` on 100, `mastery` on one. `tools/dump_skills.py` already strips the
//!   quotes, and as generated today the file contains no `"` at all; [`cell`] strips them again
//!   anyway so that a change on the generator side cannot silently turn every quoted cell into
//!   a dropped row.
//! * **Tooltip text may contain commas** and the file is comma-separated, so the generator
//!   replaces them with semicolons. That is what keeps every row at exactly 98 fields.

use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr;

/// One skill's grant-facing facts, plus its per-level table.
///
/// The fields here are the ones that are constant across a skill's level rows - measured, not
/// assumed: across all 176 skills in the generated file, zero have a row whose `job`,
/// `maxLevel` or `name` disagrees with the skill's other rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
    pub id: u32,
    /// The skill book that owns it - `0` for beginner, `200` for the Magician first job.
    pub job: u16,
    /// The highest level the client's own data describes.
    pub max_level: u32,
    pub name: String,
    /// Indexed by `level - 1`. Private so that the off-by-one lives in exactly one place:
    /// [`Skill::level`]. `None` in a slot means the file had no row for that level.
    levels: Vec<Option<SkillLevel>>,
}

/// The numbers one **level** of one skill carries.
///
/// Every field is an [`Option`] because an empty cell means *absent from the WZ node*, which is
/// not the same as zero - see the module header. There is deliberately no `Default`: a skill
/// level with every number missing is not a meaningful value, only a parse failure wearing one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkillLevel {
    /// As the client numbers it: `1` is the first level, never `0`.
    pub level: u32,
    /// `mpCon` - **flat MP points**, not a percentage. Nimble Feet level 1 is `4` and its
    /// tooltip says *"MP -4"*.
    pub mp_con: Option<u32>,
    /// `time` - **SECONDS.** The wire wants milliseconds; multiply in
    /// `net::buff::BuffLevel::granted_by` and nowhere else. `None` means the skill has no
    /// duration at this level, which for a `processtype 113` toggle is the data rather than an
    /// omission.
    pub time_seconds: Option<u32>,
    /// `cooltime` - **SECONDS.** Nimble Feet's `180` is the *"Cooldown: 3 min."* in its
    /// tooltip. None of the six job-200 skills carries one.
    pub cooltime_seconds: Option<u32>,
    /// `x` - **raw and skill-specific.** Signed: Booster skills carry `-1`/`-2` and Power Crash
    /// runs to `-18`. Its meaning changes per skill and the WZ does not say which; only that
    /// skill's tooltip does.
    pub x: Option<i32>,
    /// `y` - **raw and skill-specific**, same warning as [`SkillLevel::x`].
    pub y: Option<i32>,
    /// `mad` - the tooltip's *"Basic Attack"* number. **Per hit**, so read it together with
    /// [`SkillLevel::attack_count`]. Whether it is a percentage multiplier is inferred, not
    /// measured.
    pub mad: Option<u32>,
    /// `attackCount` - **hits per cast.** Magic Claw's is `2`: *"Use MP to attack an enemy
    /// twice."* Applying `mad` once for a 2-hit skill halves the damage.
    pub attack_count: Option<u32>,
    /// `mobCount` - targets per cast. `1` on both Magician attacks: single target.
    pub mob_count: Option<u32>,
    /// `mastery` - a **LEVEL 1..10 in this build**, never a percentage. Feeding it to a formula
    /// that expects a 10..60 percent collapses the damage spread.
    pub mastery_level: Option<u32>,
    /// `indiePdd` - **flat** Weapon Defence points. Magic Armor level 1 is `40` and its tooltip
    /// says *"Weapon Def. +40"*. Signed: Threaten carries negatives.
    pub indie_pdd: Option<i32>,
    /// `indieMdd` - **flat** Magic Defence points. Equal to `indie_pdd` at every level of Magic
    /// Armor, which is the only skill in the archive that carries it - but they are two
    /// properties and both are kept, so a skill that separates them cannot inherit the wrong
    /// one.
    pub indie_mdd: Option<i32>,
    /// `mmpR` - a **PERCENT of maximum MP**, not an amount. Max MP Increase level 1 is `10` and
    /// its tooltip says *"Max MP +10%"*.
    pub mmp_r: Option<u32>,
    /// `speed` - the movement-speed bonus. Nimble Feet's `10` is the *"speed +10"* the repo has
    /// confirmed on a client.
    pub speed: Option<u32>,
}

/// `skillId -> Skill`.
#[derive(Debug, Clone, Default)]
pub struct SkillTable {
    by_id: BTreeMap<u32, Skill>,
    source: String,
    problems: usize,
    level_rows: usize,
}

/// Columns in `gm-handbook/skills.txt`. One row per skill **level**, so the same skill appears
/// many times: the grant-facing columns are taken from its first row and the rest are placed
/// per level.
const COLUMNS: usize = 98;
const COL_ID: usize = 0;
const COL_JOB: usize = 1;
const COL_LEVEL: usize = 2;
const COL_MAX_LEVEL: usize = 3;
const COL_NAME: usize = 5;
const COL_MP_CON: usize = 31;
const COL_TIME: usize = 37;
const COL_COOLTIME: usize = 40;
const COL_MAD: usize = 43;
const COL_ATTACK_COUNT: usize = 45;
const COL_MOB_COUNT: usize = 46;
const COL_MASTERY: usize = 48;
const COL_X: usize = 63;
const COL_Y: usize = 64;
const COL_SPEED: usize = 65;
const COL_INDIE_PDD: usize = 68;
const COL_INDIE_MDD: usize = 69;
const COL_MMP_R: usize = 77;

/// A bound on how big a level vector a single row may ask for.
///
/// **This is an allocation guard, not a claim about the game.** The highest level in the
/// client's own data is `30`. Without a cap, one corrupt `level` cell asks for a multi-gigabyte
/// `Vec` before anything gets a chance to notice it is nonsense.
const MAX_SKILL_LEVEL: u32 = 255;

/// Read one cell. **Empty means absent, not zero** - see the module header.
///
/// Quotes are stripped because 1 499 numeric cells in the WZ archive are quoted strings.
/// `tools/dump_skills.py` already unquotes them, so this is belt-and-braces against a change on
/// the generator side turning every one of those cells into a dropped row.
///
/// `Err(())` means the cell was present and unreadable, which is a different outcome from
/// absent and is what makes the caller drop the whole row.
fn cell<T: FromStr>(raw: &str) -> Result<Option<T>, ()> {
    let s = raw.trim().trim_matches('"').trim();
    if s.is_empty() {
        return Ok(None);
    }
    s.parse::<T>().map(Some).map_err(|_| ())
}

/// The per-level half of one row. `Err(())` if any cell is present and unreadable, so that the
/// caller can drop the row whole rather than half-read it.
fn parse_level(f: &[&str], level: u32) -> Result<SkillLevel, ()> {
    Ok(SkillLevel {
        level,
        mp_con: cell(f[COL_MP_CON])?,
        time_seconds: cell(f[COL_TIME])?,
        cooltime_seconds: cell(f[COL_COOLTIME])?,
        x: cell(f[COL_X])?,
        y: cell(f[COL_Y])?,
        mad: cell(f[COL_MAD])?,
        attack_count: cell(f[COL_ATTACK_COUNT])?,
        mob_count: cell(f[COL_MOB_COUNT])?,
        mastery_level: cell(f[COL_MASTERY])?,
        indie_pdd: cell(f[COL_INDIE_PDD])?,
        indie_mdd: cell(f[COL_INDIE_MDD])?,
        mmp_r: cell(f[COL_MMP_R])?,
        speed: cell(f[COL_SPEED])?,
    })
}

impl Skill {
    /// The numbers for one level, as the client numbers them - `1` is the first level.
    ///
    /// `None` for level `0`, for anything above [`Skill::max_level`], and for a level whose row
    /// was dropped as unreadable. A caller must not substitute a neighbouring level for a
    /// missing one.
    pub fn level(&self, level: u32) -> Option<&SkillLevel> {
        if level == 0 {
            return None;
        }
        self.levels.get(level as usize - 1)?.as_ref()
    }

    /// Every level that loaded, in ascending order. Skips any that did not.
    pub fn levels(&self) -> impl Iterator<Item = &SkillLevel> {
        self.levels.iter().flatten()
    }

    /// How many level rows actually loaded. Equal to [`Skill::max_level`] for all 176 skills in
    /// the file as generated today; a shortfall means rows were dropped and
    /// [`SkillTable::banner`] says so.
    pub fn levels_loaded(&self) -> usize {
        self.levels.iter().flatten().count()
    }
}

impl SkillTable {
    /// Load `gm-handbook/skills.txt`. A missing file gives an empty table, not an error - the
    /// server then behaves exactly as it did before this existed.
    pub fn load(path: &Path) -> Self {
        let mut out = SkillTable { source: path.display().to_string(), ..SkillTable::default() };
        let Ok(text) = std::fs::read_to_string(path) else { return out };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split(',').map(str::trim).collect();
            // A row with the wrong column count is dropped WHOLE rather than partly read. A
            // half-read row here would put a wrong maximum on a skill and there is no second
            // check behind it - the same rule `world::shops::load_item_data` follows.
            if f.len() != COLUMNS {
                out.problems += 1;
                continue;
            }
            let (Ok(id), Ok(job), Ok(max_level), Ok(level)) = (
                f[COL_ID].parse::<u32>(),
                f[COL_JOB].parse::<u16>(),
                f[COL_MAX_LEVEL].parse::<u32>(),
                f[COL_LEVEL].parse::<u32>(),
            ) else {
                out.problems += 1;
                continue;
            };
            // There is no level 0, and `MAX_SKILL_LEVEL` bounds the allocation below.
            if level == 0 || level > MAX_SKILL_LEVEL {
                out.problems += 1;
                continue;
            }
            // **The same whole-row rule extends to the per-level cells.** A cell that is
            // present and unparseable drops the row rather than becoming `None`, because
            // `None` already means something specific - absent from the WZ - and quietly
            // reusing it for "unreadable" would hide the failure behind a legitimate value.
            let Ok(row) = parse_level(&f, level) else {
                out.problems += 1;
                continue;
            };

            let skill = out.by_id.entry(id).or_insert_with(|| Skill {
                id,
                job,
                max_level,
                name: f[COL_NAME].to_string(),
                levels: Vec::new(),
            });
            // **Placed by level rather than pushed**, so nothing here assumes the file is
            // sorted or gap-free. It is - 176 skills, every one with levels 1..maxLevel
            // contiguous and in order - but that is a property of today's generator, and an
            // append would turn a future reordering into silently wrong numbers instead of a
            // reported problem.
            let idx = level as usize - 1;
            if skill.levels.len() <= idx {
                skill.levels.resize(idx + 1, None);
            }
            if skill.levels[idx].is_some() {
                out.problems += 1; // a duplicate row for a level already loaded
                continue;
            }
            skill.levels[idx] = Some(row);
            out.level_rows += 1;
        }
        out
    }

    pub fn get(&self, id: u32) -> Option<&Skill> {
        self.by_id.get(&id)
    }

    /// The numbers for one level of one skill. `None` if either the skill or that level of it
    /// is not in the table.
    pub fn level(&self, skill_id: u32, level: u32) -> Option<&SkillLevel> {
        self.get(skill_id)?.level(level)
    }

    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }

    /// How many skill-**level** rows loaded across the whole table. `4164` for the file as
    /// generated today.
    pub fn level_rows(&self) -> usize {
        self.level_rows
    }

    /// **May a character of `job` put a point in `skill_id`?**
    ///
    /// Beginner skills (book `0`) are always allowed - every character keeps them. Otherwise
    /// the book must be in the character's own branch and **not above their tier**, so a
    /// Magician can raise first-job skills and a beginner cannot raise any of them.
    ///
    /// The `<=` is what lets a second-job character keep spending on first-job skills, which
    /// is the ordinary case a straight equality would break the moment anyone advanced twice.
    ///
    /// An **unknown** skill is refused. That is the same choice the handler made before this
    /// table existed, and for the same reason: granting a level in something the client has no
    /// data for gives it a number it cannot draw.
    pub fn may_learn(&self, job: u16, skill_id: u32) -> bool {
        match self.get(skill_id) {
            None => false,
            Some(s) if s.job == 0 => true,
            Some(s) => s.job / 100 == job / 100 && s.job <= job,
        }
    }

    /// The ceiling for one skill, or `None` if the table does not describe it.
    pub fn max_level(&self, skill_id: u32) -> Option<u32> {
        self.get(skill_id).map(|s| s.max_level)
    }

    /// Printed at start-up, in both directions - a banner that is silent when things are fine
    /// cannot be told apart from one that is not being printed.
    pub fn banner(&self) -> String {
        let from = if self.source.is_empty() { "(no file)" } else { self.source.as_str() };
        if self.by_id.is_empty() {
            return format!(
                "maplecw-world: skills: NONE LOADED from {from}. Only the three beginner skills \
                 will be grantable, exactly as before this table existed. Regenerate with: \
                 python tools/dump_skills.py"
            );
        }
        format!(
            "maplecw-world: skills: {} skills, {} skill-levels from {from}{}",
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The real table, or `None` on a clean checkout where `gm-handbook/` has not been
    /// generated. Every test below is guarded on this and asserts a **positive control first**,
    /// which is the shape `CLAUDE.md` demands of a check that can be skipped.
    fn real_table() -> Option<SkillTable> {
        let path = Path::new("../../gm-handbook/skills.txt");
        if !path.exists() {
            return None;
        }
        Some(SkillTable::load(path))
    }

    /// **The real table, when it has been generated.**
    ///
    /// `gm-handbook/` is generated and gitignored, so this is a no-op on a clean checkout -
    /// which is exactly the shape of check `CLAUDE.md` warns can be silently vacuous. It
    /// therefore asserts a **positive control first**: if the file is there at all, it must
    /// contain the six Magician first-job skills with the levels the research file measured.
    #[test]
    fn the_magician_first_job_book_is_what_the_client_ships() {
        let Some(t) = real_table() else { return };
        assert_eq!(t.problems, 0, "every row parses");

        // The six, and their ceilings. research/magician-first-job.md.
        let book: [(u32, &str, u32); 6] = [
            (2000000, "Improved MP Recovery", 15),
            (2000001, "Max MP Increase", 15),
            (2001000, "Magic Guard", 15),
            (2001001, "Magic Armor", 20),
            (2001002, "Energy Bolt", 20),
            (2001003, "Magic Claw", 20),
        ];
        for (id, name, max) in book {
            let s = t.get(id).unwrap_or_else(|| panic!("{name} ({id}) is missing"));
            assert_eq!(s.max_level, max, "{name}");
            assert_eq!(s.job, 200, "{name} belongs to the Magician book");
            assert_eq!(s.name, name);
            // **Every level the ceiling promises actually loaded.** A `max_level` of 20 with
            // 19 rows behind it would refuse a cast at the top level while the `+` button
            // still allowed the point.
            assert_eq!(s.levels_loaded(), max as usize, "{name} has all its level rows");
            assert_eq!(s.levels().count(), max as usize, "{name} iterates its levels");
            assert!(s.level(0).is_none(), "{name} has no level 0");
            assert!(s.level(max + 1).is_none(), "{name} stops at its ceiling");
            for lv in 1..=max {
                assert_eq!(
                    s.level(lv).map(|l| l.level),
                    Some(lv),
                    "{name} level {lv} is stored at its own index"
                );
            }
        }

        // **A Magician may learn them and a beginner may not.** This is the gate that used to
        // refuse everything, so both directions are asserted.
        for (id, name, _) in book {
            assert!(t.may_learn(200, id), "a Magician may raise {name}");
            assert!(!t.may_learn(0, id), "a BEGINNER must not raise {name}");
            assert!(!t.may_learn(100, id), "and neither may a Warrior");
            assert!(t.may_learn(210, id), "a second-job Magician keeps first-job skills");
        }

        // Beginner skills stay learnable by everyone - they are book 0.
        for id in net::skills::BEGINNER_SKILLS {
            assert!(t.may_learn(0, id), "beginner skill {id}");
            assert!(t.may_learn(200, id), "a Magician keeps beginner skill {id}");
        }

        // An id the client does not ship is refused rather than granted a default.
        assert!(!t.may_learn(200, 9_999_999));
        assert_eq!(t.max_level(9_999_999), None);
        assert_eq!(t.level(9_999_999, 1), None);
    }

    /// **Nimble Feet, which is the strongest control this repo has.**
    ///
    /// Its four numbers are stated three independent times: by `Skill.wz`, by the client's own
    /// tooltip (*"MP -4; speed +10 for 10 sec. Cooldown: 3 min."*), and by
    /// `crates/net/src/buff.rs`, whose values were **confirmed on a client**. The literals below
    /// are the WZ's; they are written out rather than compared against `net::buff::buff_level`
    /// on purpose, so that this file's check does not move when that one does.
    ///
    /// If this test fails, the extraction has drifted from the one skill whose behaviour has
    /// actually been watched on screen - suspect the reader, not the client.
    #[test]
    fn nimble_feet_matches_the_numbers_confirmed_on_a_client() {
        let Some(t) = real_table() else { return };
        let s = t.get(1002).expect("Nimble Feet (1002) is the control and must be present");
        assert_eq!(s.name, "Nimble Feet");
        assert_eq!(s.job, 0, "a beginner skill");
        assert_eq!(s.max_level, 3);

        // (level, mpCon, time SECONDS, cooltime SECONDS, speed)
        for (lv, mp, secs, cool, speed) in [(1u32, 4u32, 10u32, 180u32, 10u32), (2, 7, 20, 180, 10), (3, 10, 30, 180, 10)] {
            let l = s.level(lv).unwrap_or_else(|| panic!("Nimble Feet level {lv}"));
            assert_eq!(l.mp_con, Some(mp), "level {lv} mpCon");
            assert_eq!(l.time_seconds, Some(secs), "level {lv} time is SECONDS, not ms");
            assert_eq!(l.cooltime_seconds, Some(cool), "level {lv} cooltime is SECONDS");
            assert_eq!(l.speed, Some(speed), "level {lv} speed");
            // The unit trap, stated as an assertion: 10 here becomes 10 000 on the wire, and
            // that multiplication belongs to `net::buff::BuffLevel::granted_by`, not here.
            assert!(l.time_seconds.unwrap() < 1000, "seconds, so nothing here is a ms count");
        }
    }

    /// **The per-level numbers of the six, at both ends of each ladder.**
    ///
    /// Every literal here is from `research/magician-first-job.md`, which took them from the
    /// client's own `Skill.wz` and checked each against the tooltip the player reads.
    #[test]
    fn the_magician_per_level_numbers_are_what_the_research_measured() {
        let Some(t) = real_table() else { return };

        // 2000000 Improved MP Recovery - `x` percent of max MP per tick, `y` percent item bonus.
        // Both raw: the meaning is the tooltip's, not this field's.
        let mpr = t.get(2000000).expect("Improved MP Recovery");
        assert_eq!(mpr.level(1).unwrap().x, Some(1));
        assert_eq!(mpr.level(1).unwrap().y, Some(5));
        assert_eq!(mpr.level(15).unwrap().x, Some(1));
        assert_eq!(mpr.level(15).unwrap().y, Some(20));
        // A passive: it costs nothing to have.
        assert_eq!(mpr.level(1).unwrap().mp_con, None, "no mpCon on a passive");

        // 2000001 Max MP Increase - `mmpR` is a PERCENT of max MP, never an amount.
        let mmp = t.get(2000001).expect("Max MP Increase");
        assert_eq!(mmp.level(1).unwrap().mmp_r, Some(10), "\"Max MP +10%\"");
        assert_eq!(mmp.level(15).unwrap().mmp_r, Some(25), "\"Max MP +25%\"");

        // 2001000 Magic Guard - `x` 30 at level 1, 80 at 15, and NO duration at any level.
        let guard = t.get(2001000).expect("Magic Guard");
        assert_eq!(guard.level(1).unwrap().x, Some(30), "\"Replace 30% of HP damage with MP\"");
        assert_eq!(guard.level(15).unwrap().x, Some(80));
        assert_eq!(guard.level(1).unwrap().mp_con, Some(8), "\"MP -8\"");
        assert_eq!(guard.level(15).unwrap().mp_con, Some(12));
        // **The absence is the data.** `processtype 113` is a toggle: it carries no `time` at
        // any of its 15 levels, and `None` here must never be read as a zero-second buff.
        for l in guard.levels() {
            assert_eq!(l.time_seconds, None, "Magic Guard level {} is a toggle", l.level);
            assert_eq!(l.cooltime_seconds, None, "and has no cooldown either");
        }

        // 2001001 Magic Armor - `time` 300 SECONDS at level 1, 600 at 20. On the wire that is
        // 300_000 ms, and the x1000 happens in `net::buff`, not here.
        let armor = t.get(2001001).expect("Magic Armor");
        assert_eq!(armor.level(1).unwrap().time_seconds, Some(300), "\"for 300 sec\"");
        assert_eq!(armor.level(20).unwrap().time_seconds, Some(600));
        assert_eq!(armor.level(1).unwrap().mp_con, Some(8));
        assert_eq!(armor.level(20).unwrap().mp_con, Some(16));
        // Two separate flat defence properties, equal at every level but carried separately.
        assert_eq!(armor.level(1).unwrap().indie_pdd, Some(40), "\"Weapon Def. +40\"");
        assert_eq!(armor.level(1).unwrap().indie_mdd, Some(40), "\"Magic Def. +40\"");
        assert_eq!(armor.level(20).unwrap().indie_pdd, Some(120));
        assert_eq!(armor.level(20).unwrap().indie_mdd, Some(120));
        // A monotone ladder in 15-second steps, which is what makes a single wrong row visible.
        for lv in 2..=19 {
            let (prev, cur) = (
                armor.level(lv - 1).unwrap().time_seconds.unwrap(),
                armor.level(lv).unwrap().time_seconds.unwrap(),
            );
            assert_eq!(cur - prev, 15, "Magic Armor level {lv} duration step");
        }

        // 2001002 Energy Bolt - one hit per cast, mastery climbing 1..10 over 20 levels.
        let bolt = t.get(2001002).expect("Energy Bolt");
        assert_eq!(bolt.level(1).unwrap().mad, Some(90), "\"Basic Attack 90\"");
        assert_eq!(bolt.level(20).unwrap().mad, Some(130));
        assert_eq!(bolt.level(1).unwrap().attack_count, Some(1));
        assert_eq!(bolt.level(1).unwrap().mob_count, Some(1), "single target");
        assert_eq!(bolt.level(1).unwrap().mastery_level, Some(1));
        assert_eq!(bolt.level(20).unwrap().mastery_level, Some(10));

        // 2001003 Magic Claw - **attackCount 2 at every level.** `mad 45` is PER HIT; a
        // consumer that applies it once produces half the damage.
        let claw = t.get(2001003).expect("Magic Claw");
        for l in claw.levels() {
            assert_eq!(l.attack_count, Some(2), "Magic Claw level {} hits twice", l.level);
            assert_eq!(l.mob_count, Some(1), "and hits one mob");
        }
        assert_eq!(claw.level(1).unwrap().mad, Some(45), "per hit, so a cast is 2 x 45");
        assert_eq!(claw.level(20).unwrap().mad, Some(65));
        assert_eq!(claw.level(1).unwrap().mp_con, Some(10), "\"MP -10\"");
        assert_eq!(claw.level(20).unwrap().mp_con, Some(20));
        assert_eq!(claw.level(1).unwrap().mastery_level, Some(1));
        assert_eq!(claw.level(20).unwrap().mastery_level, Some(10));
        // Magic Claw is not a buff and carries neither duration nor cooldown.
        assert_eq!(claw.level(1).unwrap().time_seconds, None);
        assert_eq!(claw.level(1).unwrap().cooltime_seconds, None);
    }

    /// **The two unit claims, asserted archive-wide rather than on one skill.**
    ///
    /// A single skill agreeing proves nothing about the column; these are the checks that break
    /// loudly if a repack or a generator change switches either property to the form it takes in
    /// other MapleStory versions.
    #[test]
    fn mastery_is_a_level_and_x_is_signed_across_the_whole_archive() {
        let Some(t) = real_table() else { return };
        // Positive control first: the sweep below is worthless unless it is actually looking at
        // cells that carry these properties.
        assert!(t.len() > 100, "the table loaded: {} skills", t.len());
        assert_eq!(t.level_rows(), t.by_id.values().map(Skill::levels_loaded).sum::<usize>());

        let (mut mastery_cells, mut negative_x) = (0usize, 0usize);
        for s in t.by_id.values() {
            for l in s.levels() {
                if let Some(m) = l.mastery_level {
                    mastery_cells += 1;
                    // **1..=10, NOT the 10..60 percentage.** The client's tooltip says
                    // "Mastery level N"; if this ever holds 60 the unit has changed under us.
                    assert!(
                        (1..=10).contains(&m),
                        "{} ({}) level {} has mastery {m}, which is outside the 1..10 this \
                         build uses - see research/magician-first-job.md",
                        s.name,
                        s.id,
                        l.level
                    );
                }
                if l.x.is_some_and(|x| x < 0) {
                    negative_x += 1;
                }
            }
        }
        assert!(mastery_cells > 100, "mastery was actually read: {mastery_cells} cells");
        // **`x` must be signed**, and this is the control that proves it rather than asserting
        // it in a comment: Booster skills carry -1/-2 and Power Crash runs to -18. Parsed as
        // unsigned, these rows would have been dropped and `problems` would not be 0.
        assert!(negative_x > 100, "x carries negatives: {negative_x} cells");
        assert_eq!(t.problems, 0, "and no row was dropped getting them");
        assert_eq!(
            t.get(1101002).and_then(|s| s.level(1)).and_then(|l| l.x),
            Some(-2),
            "Sword Booster level 1 x = -2"
        );
    }

    /// A missing file leaves the server exactly as it was, and says so.
    #[test]
    fn a_missing_file_degrades_to_the_old_behaviour() {
        let t = SkillTable::load(Path::new("no/such/skills.txt"));
        assert!(t.is_empty());
        assert_eq!(t.level_rows(), 0);
        assert!(!t.may_learn(200, 2001003), "nothing is grantable from an empty table");
        assert_eq!(t.level(2001003, 1), None);
        assert!(t.banner().contains("NONE LOADED"), "{}", t.banner());
    }

    /// **The parser's own failure modes, on rows written here rather than found.**
    ///
    /// This is the one test that does not need `gm-handbook/`, and it exists because the checks
    /// above can only ever see cells the generator happens to produce today. Each row below
    /// states a rule the module header claims.
    #[test]
    fn a_bad_cell_drops_its_row_and_an_empty_one_means_absent() {
        use std::io::Write;

        // A row builder: 98 fields, all empty but the ones named.
        fn row(pairs: &[(usize, &str)]) -> String {
            let mut f = vec![String::new(); COLUMNS];
            for (i, v) in pairs {
                f[*i] = (*v).to_string();
            }
            f.join(",")
        }

        let dir = std::env::temp_dir().join("maplecw-skilltable-test");
        std::fs::create_dir_all(&dir).unwrap();
        // Named per process: other agents share this machine and `cargo test -p world` can be
        // running in two of them at once.
        let path = dir.join(format!("skills-{}.txt", std::process::id()));
        let mut file = std::fs::File::create(&path).unwrap();
        let lines = [
            "# a comment line, skipped".to_string(),
            // A good row, and the positive control for everything below.
            row(&[
                (COL_ID, "700"),
                (COL_JOB, "200"),
                (COL_LEVEL, "1"),
                (COL_MAX_LEVEL, "2"),
                (COL_NAME, "Control"),
                (COL_MP_CON, "8"),
                (COL_X, "-30"),
            ]),
            // **Quoted cells still parse**, because 1 499 of them are quoted in the archive.
            row(&[
                (COL_ID, "700"),
                (COL_JOB, "200"),
                (COL_LEVEL, "2"),
                (COL_MAX_LEVEL, "2"),
                (COL_NAME, "Control"),
                (COL_MP_CON, "\"10\""),
                (COL_MASTERY, "\"3\""),
            ]),
            // A present-but-unreadable cell drops the WHOLE row rather than becoming `None`.
            row(&[
                (COL_ID, "701"),
                (COL_JOB, "200"),
                (COL_LEVEL, "1"),
                (COL_MAX_LEVEL, "1"),
                (COL_NAME, "BadCell"),
                (COL_MAD, "not-a-number"),
            ]),
            // There is no level 0.
            row(&[
                (COL_ID, "702"),
                (COL_JOB, "200"),
                (COL_LEVEL, "0"),
                (COL_MAX_LEVEL, "1"),
                (COL_NAME, "LevelZero"),
            ]),
            // A level above the allocation guard is refused rather than allocated for.
            row(&[
                (COL_ID, "703"),
                (COL_JOB, "200"),
                (COL_LEVEL, "4000000000"),
                (COL_MAX_LEVEL, "1"),
                (COL_NAME, "Absurd"),
            ]),
            // Wrong column count.
            "704,200,1,1".to_string(),
        ];
        writeln!(file, "{}", lines.join("\n")).unwrap();
        drop(file);

        let t = SkillTable::load(&path);

        // Positive control first: the good rows loaded, so the negatives below are about the
        // rows and not about the reader failing to read anything at all.
        let s = t.get(700).expect("the control row loaded");
        assert_eq!(s.name, "Control");
        assert_eq!(s.levels_loaded(), 2);
        assert_eq!(s.level(1).unwrap().mp_con, Some(8));
        assert_eq!(s.level(1).unwrap().x, Some(-30), "x is signed");
        assert_eq!(s.level(2).unwrap().mp_con, Some(10), "a quoted cell parses");
        assert_eq!(s.level(2).unwrap().mastery_level, Some(3), "so does a quoted mastery");
        // **An empty cell is absent, not zero.**
        assert_eq!(s.level(1).unwrap().time_seconds, None, "empty means absent");
        assert_eq!(s.level(1).unwrap().mastery_level, None);
        assert_eq!(s.level(2).unwrap().x, None);

        // Each of the four bad rows was dropped whole, and counted.
        assert_eq!(t.get(701), None, "one unreadable cell drops the whole row");
        assert_eq!(t.get(702), None, "there is no level 0");
        assert_eq!(t.get(703), None, "and no level 4 000 000 000");
        assert_eq!(t.get(704), None, "nor a short row");
        assert_eq!(t.problems, 4, "and the banner reports them: {}", t.banner());
        assert!(t.banner().contains("4 unreadable line(s)"), "{}", t.banner());
        assert!(t.banner().contains("2 skill-levels"), "{}", t.banner());

        std::fs::remove_file(&path).ok();
    }
}
