//! What each job may learn, and how far - from the client's own `Skill.wz`.
//!
//! The owner, 2026-08-27: *"I want to verify that all Magician 1st job skills are working first."*
//!
//! # This table is the thing `session::skills` said it was waiting for
//!
//! That handler refused every skill outside the three beginner ids, and its comment said why:
//! *"Refusing anything else is not a rule of the game - it is a refusal to invent one, since
//! what a job may learn is `Skill.wz` data nobody has read."* It has now been read.
//! `tools/dump_skills.py` writes `gm-handbook/skills.txt` - 176 skills, 4 164 skill-levels -
//! and this loads the two columns the refusal actually needed.
//!
//! # Only two columns, deliberately
//!
//! The generated file has **98** of them. This reads the skill id, the job that owns it, and
//! the maximum level, because those are what a *grant* has to decide. Everything else -
//! `mpCon`, `mad`, `time`, `mastery` - belongs to whatever eventually computes an effect, and
//! `research/magician-first-job.md` records that several of those columns mean different
//! things on different skills and that the WZ never says which. Loading a number here that
//! nothing checks would be inventing a claim.
//!
//! # The unit trap this file does NOT walk into
//!
//! `maxLevel` is a count of levels and needs no conversion. The columns that do are called out
//! in the research file and are not read here: `time` is **seconds** where `net::buff` wants
//! milliseconds, `mastery` is a **level 1..10** in this build rather than the percentage it is
//! in other versions, and `mmpR` is a **percent** of max MP. Anyone adding a column here
//! should read that section first.

use std::collections::BTreeMap;
use std::path::Path;

/// One skill, reduced to what a grant needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
    pub id: u32,
    /// The skill book that owns it - `0` for beginner, `200` for the Magician first job.
    pub job: u16,
    /// The highest level the client's own data describes.
    pub max_level: u32,
    pub name: String,
}

/// `skillId -> Skill`.
#[derive(Debug, Clone, Default)]
pub struct SkillTable {
    by_id: BTreeMap<u32, Skill>,
    source: String,
    problems: usize,
}

/// Columns in `gm-handbook/skills.txt`. One row per skill **level**, so the same skill appears
/// many times and only the first row of each is kept.
const COLUMNS: usize = 98;
const COL_ID: usize = 0;
const COL_JOB: usize = 1;
const COL_MAX_LEVEL: usize = 3;
const COL_NAME: usize = 5;

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
            let (Ok(id), Ok(job), Ok(max_level)) = (
                f[COL_ID].parse::<u32>(),
                f[COL_JOB].parse::<u16>(),
                f[COL_MAX_LEVEL].parse::<u32>(),
            ) else {
                out.problems += 1;
                continue;
            };
            out.by_id.entry(id).or_insert_with(|| Skill {
                id,
                job,
                max_level,
                name: f[COL_NAME].to_string(),
            });
        }
        out
    }

    pub fn get(&self, id: u32) -> Option<&Skill> {
        self.by_id.get(&id)
    }

    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
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
            "maplecw-world: skills: {} skills from {from}{}",
            self.by_id.len(),
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

    /// **The real table, when it has been generated.**
    ///
    /// `gm-handbook/` is generated and gitignored, so this is a no-op on a clean checkout -
    /// which is exactly the shape of check `CLAUDE.md` warns can be silently vacuous. It
    /// therefore asserts a **positive control first**: if the file is there at all, it must
    /// contain the six Magician first-job skills with the levels the research file measured.
    #[test]
    fn the_magician_first_job_book_is_what_the_client_ships() {
        let path = Path::new("../../gm-handbook/skills.txt");
        if !path.exists() {
            return;
        }
        let t = SkillTable::load(path);
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
    }

    /// A missing file leaves the server exactly as it was, and says so.
    #[test]
    fn a_missing_file_degrades_to_the_old_behaviour() {
        let t = SkillTable::load(Path::new("no/such/skills.txt"));
        assert!(t.is_empty());
        assert!(!t.may_learn(200, 2001003), "nothing is grantable from an empty table");
        assert!(t.banner().contains("NONE LOADED"), "{}", t.banner());
    }
}
