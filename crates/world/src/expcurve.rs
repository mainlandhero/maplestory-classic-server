//! Levelling up: how much experience each level costs, and what a level is worth.
//!
//! # The table is not ours and the client has its own copy
//!
//! `data/exp-curve.txt`, from the community guide. **The client's copy wins if they ever
//! disagree**: it lives at `0x143AC2400`, 121 `u64`s, and the client draws the EXP bar from
//! it, so a server that disagrees shows a bar that does not fill when it should. That copy
//! is in the zero-initialised tail of `.data` and has no bytes on disk, which is why it took
//! a probe `dump=` to read at all - and why reading it statically once produced 120
//! confident wrong numbers that were exception-handling records.
//!
//! Every `-SetFieldProbe` run now dumps it; `python tools/decode_dump.py --exp-curve`
//! decodes it, and comparing the two is a five-second job that has never been done.
//!
//! # What a level gives you is OURS
//!
//! The curve is data. [`LevelGains`] is policy, in one named place, tagged `[I]` - nothing
//! in the client or the guide says what a level awards, and a wrong guess here is a
//! character that is quietly too strong or too weak rather than anything visible.

use std::collections::HashMap;
use std::path::Path;

/// The highest level a character can reach. A character there stops gaining.
///
/// **120, from the client's own table, since 2026-08-21.** It was 100 while `data/exp-curve.txt`
/// stopped at 99, and both moved together: the dumped table has 121 entries, `[0] = 0` as the
/// hole that makes the index 1-based, `[119] = 28171993`, and `[120] = 0` because 120 is the
/// cap and has no next level.
///
/// Leaving this at 100 after extending the file would have made levels 101..119 unreachable
/// while the client funded them - a cap that silently disagrees with the data beside it,
/// which is exactly the kind of quiet wrongness the module header warns about.
pub const MAX_LEVEL: u32 = 120;

/// Which of the five gain lines a character is on.
///
/// Keyed on the hundreds digit of the job id, which is the explorer tree this client's own
/// SP fork masks describe: `net::opcode::uses_extended_sp` decodes them as `100`/`110`/...
/// for warriors and the same shape at `200`, `300`, `400` and `500`. **[L]** for the job
/// numbering; the gains themselves are not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassLine {
    Beginner,
    Warrior,
    Magician,
    Bowman,
    Thief,
}

impl ClassLine {
    /// The line a job id is on.
    ///
    /// **Pirate (`500`) falls back to the beginner line and that is a placeholder, not a
    /// finding.** The source below has no pirate row - Classic World may not have the class
    /// at all - and inventing numbers for it would be the sort of confident guess this
    /// project keeps paying for. Nothing can reach it today: job advancement is goal E and
    /// does not exist, so every character is a beginner.
    pub fn of_job(job: u16) -> ClassLine {
        match job / 100 {
            1 => ClassLine::Warrior,
            2 => ClassLine::Magician,
            3 => ClassLine::Bowman,
            4 => ClassLine::Thief,
            _ => ClassLine::Beginner,
        }
    }
}

/// What one level awards.
///
/// # Where these numbers come from, and how much to trust them
///
/// The HP and MP figures are **measured behaviour of the live COT2 service**, reported by
/// the same fan site whose citizenship quest list and drop tables have both been checked
/// against this client's own WZ and agreed - `research/meowdb-combat-formulas.md`. That
/// makes them **`[I]` with a good prior**, and specifically *not* `[L]`: they are not read
/// off a listing and this client cannot be made to say them.
///
/// **That last part was checked rather than assumed.** The EXP curve at `0x143AC2400` has no
/// sibling table - a `lea` scan of the whole neighbourhood that fills it finds one `.data`
/// target and it is the curve itself. Which fits: the client draws the EXP bar so it needs
/// the curve, and it is *told* new maxima on level up rather than computing them. The scan
/// cannot see a table reached through a register or from `.themida`, so that is "no evidence
/// in the obvious place", not "proved absent".
///
/// The site reports **zero variance** across every level-up it sampled, so these are flat
/// rather than rolled. That also happens to be the right property for a server nobody has
/// verified: a range would make two runs of the same actions produce different characters.
///
/// # What is ours rather than theirs
///
/// **Ability points at five a level.** The site says nothing about AP; five is this game
/// family's long-standing rule and it is what this server has always given.
///
/// **SP is deliberately not awarded.** A beginner has no skills to spend it on, and the stat
/// block's SP field forks on the job - the branch every character here takes sends a pool
/// list nobody has decoded. Job advancement is where that gets read.
///
/// # What is NOT here
///
/// The **500-point job-advancement bonus** and the **+25% from maxed Improving Max HP/MP**
/// are both in `research/meowdb-combat-formulas.md` and neither is implemented, because
/// neither has anything to hang off yet: job advancement is goal E and the skills are second
/// job. Saying so here rather than adding an unwired table is the point - `CLAUDE.md`'s
/// "built is not wired".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelGains {
    pub ap: u16,
    pub max_hp: u32,
    pub max_mp: u32,
}

impl LevelGains {
    /// The gains for one class line.
    pub fn for_class(class: ClassLine) -> LevelGains {
        let (max_hp, max_mp) = match class {
            ClassLine::Beginner => (16, 12),
            ClassLine::Warrior => (28, 12),
            ClassLine::Magician => (16, 22),
            ClassLine::Bowman => (22, 17),
            ClassLine::Thief => (22, 17),
        };
        LevelGains { ap: 5, max_hp, max_mp }
    }
}

impl Default for LevelGains {
    /// A beginner, which is what every character on this server is.
    fn default() -> Self {
        LevelGains::for_class(ClassLine::Beginner)
    }
}

/// How much experience each level costs.
///
/// **It no longer carries the gains.** There used to be a `pub gains: LevelGains` here, from
/// when every character gained the same amount. Now that the gains depend on the job, that
/// field would be a public knob that looks like it sets the level-up award and is ignored -
/// which is worse than not having one. [`LevelGains::for_class`] is the only source.
#[derive(Debug, Clone, Default)]
pub struct ExpCurve {
    /// level -> experience needed to reach `level + 1`.
    to_next: HashMap<u32, u64>,
    /// Lines that would not parse, surfaced in the startup banner.
    pub problems: Vec<String>,
}

/// What awarding experience did.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Awarded {
    pub level: u32,
    pub exp: u64,
    /// How many levels were gained. `0` is the ordinary case.
    pub levels: u32,
    pub max_hp: u32,
    pub max_mp: u32,
    pub ap: u16,
}

impl ExpCurve {
    /// A missing file is empty and means **nothing ever levels** - experience still
    /// accumulates, it just never crosses a threshold, because there are no thresholds.
    pub fn load(path: &Path) -> Self {
        match std::fs::read_to_string(path) {
            Ok(text) => Self::parse(&text),
            Err(e) => ExpCurve {
                problems: vec![format!("{}: {e} - nothing will ever level up", path.display())],
                ..Default::default()
            },
        }
    }

    /// `level | expToNextLevel`, `#` comments, blank lines skipped.
    pub fn parse(text: &str) -> Self {
        let mut out = ExpCurve::default();
        for (n, raw) in text.lines().enumerate() {
            let line = raw.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            let Some((lvl, need)) = line.split_once('|') else {
                out.problems.push(format!("line {}: needs `level | exp`", n + 1));
                continue;
            };
            match (lvl.trim().parse::<u32>(), need.trim().parse::<u64>()) {
                (Ok(l), Ok(e)) if e > 0 => {
                    out.to_next.insert(l, e);
                }
                (Ok(l), Ok(_)) => out.problems.push(format!("line {}: level {l} costs 0", n + 1)),
                _ => out.problems.push(format!("line {}: unparseable", n + 1)),
            }
        }
        out
    }

    /// Experience needed to get from `level` to the next one, if there is one.
    pub fn to_next(&self, level: u32) -> Option<u64> {
        self.to_next.get(&level).copied()
    }

    pub fn is_empty(&self) -> bool {
        self.to_next.is_empty()
    }

    pub fn levels(&self) -> usize {
        self.to_next.len()
    }

    /// Award experience, levelling as many times as it pays for.
    ///
    /// **Experience carries over between levels**, which is what makes a big award behave
    /// sensibly: killing something worth ten levels' worth grants ten levels rather than one
    /// and a discarded remainder.
    ///
    /// At [`MAX_LEVEL`], or with no entry for the current level, experience simply
    /// accumulates and no level is granted. That is deliberately not an error: an unknown
    /// level should stop levelling, not panic and not level infinitely.
    pub fn award(&self, job: u16, level: u32, exp: u64, gained: u64) -> Awarded {
        let gains = LevelGains::for_class(ClassLine::of_job(job));
        let mut out = Awarded { level, exp: exp.saturating_add(gained), ..Default::default() };
        // A bounded loop. `to_next` is positive by construction (the parser rejects 0), so
        // this terminates, but the cap makes that true by inspection rather than by argument.
        for _ in 0..MAX_LEVEL {
            if out.level >= MAX_LEVEL {
                break;
            }
            let Some(need) = self.to_next(out.level) else { break };
            if out.exp < need {
                break;
            }
            out.exp -= need;
            out.level += 1;
            out.levels += 1;
            out.ap += gains.ap;
            out.max_hp += gains.max_hp;
            out.max_mp += gains.max_mp;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Job 0. Every character on this server is one, so it is the case that matters.
    const BEGINNER: u16 = 0;

    fn curve() -> ExpCurve {
        ExpCurve::parse("1 | 15\n2 | 34\n3 | 57\n")
    }

    #[test]
    fn the_real_file_parses_and_rises() {
        let c = ExpCurve::load(Path::new("../../data/exp-curve.txt"));
        assert!(c.problems.is_empty(), "{:?}", c.problems);
        // **119, not 99, since 2026-08-21.** The client's own table was finally dumped and
        // compared: levels 1..99 agree EXACTLY, 99 of 99, so the fan-sourced rows are now
        // confirmed rather than assumed - and the dump also carries 100..119, which the
        // guide did not. Those rows come straight from the running client.
        //
        // The client's entry for level **120** is `0`: 120 is the cap and has nowhere to go,
        // which is the same shape this assertion always made, one level higher up.
        assert_eq!(c.levels(), 119, "levels 1..119, from the client's own table");
        assert_eq!(c.to_next(1), Some(15), "the classic first level");
        assert_eq!(c.to_next(99), Some(9_692_044));
        assert_eq!(c.to_next(119), Some(28_171_993), "the last level the client funds");
        assert_eq!(c.to_next(120), None, "and 120 is the cap - the client's own entry is 0");
        // Monotonic: a later level must never be cheaper than an earlier one.
        for l in 1..119 {
            assert!(
                c.to_next(l).unwrap() <= c.to_next(l + 1).unwrap(),
                "level {l} costs more than level {}",
                l + 1
            );
        }
    }

    #[test]
    fn not_enough_experience_is_not_a_level() {
        let a = curve().award(BEGINNER, 1, 0, 14);
        assert_eq!((a.level, a.exp, a.levels), (1, 14, 0));
        assert_eq!(a.ap, 0, "and nothing is awarded");
    }

    #[test]
    fn exactly_enough_levels_once_and_leaves_nothing_over() {
        let a = curve().award(BEGINNER, 1, 0, 15);
        assert_eq!((a.level, a.exp, a.levels), (2, 0, 1));
        assert_eq!(a.ap, LevelGains::default().ap);
    }

    /// **Experience carries over.** A single award worth several levels grants several.
    #[test]
    fn a_large_award_levels_more_than_once_and_keeps_the_remainder() {
        let a = curve().award(BEGINNER, 1, 0, 15 + 34 + 10);
        assert_eq!((a.level, a.exp, a.levels), (3, 10, 2));
        assert_eq!(a.ap, LevelGains::default().ap * 2, "two levels, two awards");
    }

    /// A level with no entry stops levelling rather than looping or panicking.
    #[test]
    fn running_off_the_end_of_the_table_stops() {
        let a = curve().award(BEGINNER, 3, 0, 1_000_000);
        assert_eq!(a.level, 4, "level 3 has an entry, level 4 does not");
        assert_eq!(a.exp, 1_000_000 - 57);
    }

    #[test]
    fn the_maximum_level_stops_gaining_levels_but_still_banks_experience() {
        let c = ExpCurve::parse("99 | 10\n100 | 10\n");
        let a = c.award(BEGINNER, MAX_LEVEL, 0, 1_000);
        assert_eq!((a.level, a.levels), (MAX_LEVEL, 0));
        assert_eq!(a.exp, 1_000, "the experience is still banked");
    }

    #[test]
    fn an_empty_curve_never_levels_anything() {
        let c = ExpCurve::default();
        let a = c.award(BEGINNER, 1, 0, u64::MAX / 2);
        assert_eq!(a.levels, 0);
        assert!(c.is_empty());
    }

    /// The five gain lines, as `research/meowdb-combat-formulas.md` reports them.
    #[test]
    fn each_class_has_its_own_hp_and_mp_line() {
        for (job, hp, mp) in [
            (0u16, 16u32, 12u32),   // beginner
            (100, 28, 12),          // warrior
            (200, 16, 22),          // magician
            (300, 22, 17),          // bowman
            (400, 22, 17),          // thief
        ] {
            let g = LevelGains::for_class(ClassLine::of_job(job));
            assert_eq!((g.max_hp, g.max_mp), (hp, mp), "job {job}");
            assert_eq!(g.ap, 5, "AP is ours and is five for everyone");
        }
    }

    /// Second-job ids are on the same line as their first job: 110 is still a warrior.
    #[test]
    fn the_line_is_the_hundreds_digit() {
        for job in [100u16, 110, 111, 112, 120, 130, 132] {
            assert_eq!(ClassLine::of_job(job), ClassLine::Warrior, "job {job}");
        }
        assert_eq!(ClassLine::of_job(422), ClassLine::Thief);
    }

    /// Pirate has no row in the source, so it falls back rather than inventing numbers.
    #[test]
    fn an_unknown_job_falls_back_to_the_beginner_line() {
        assert_eq!(ClassLine::of_job(500), ClassLine::Beginner);
        assert_eq!(ClassLine::of_job(9999), ClassLine::Beginner);
        assert_eq!(LevelGains::default(), LevelGains::for_class(ClassLine::Beginner));
    }

    /// The job actually reaches the award, rather than being accepted and ignored.
    #[test]
    fn the_job_changes_what_a_level_gives() {
        let beginner = curve().award(0, 1, 0, 15);
        let warrior = curve().award(100, 1, 0, 15);
        assert_eq!((beginner.max_hp, beginner.max_mp), (16, 12));
        assert_eq!((warrior.max_hp, warrior.max_mp), (28, 12));
        assert_eq!(beginner.ap, warrior.ap, "AP does not depend on the class");
    }

    #[test]
    fn a_zero_cost_level_is_rejected_rather_than_looping_forever() {
        let c = ExpCurve::parse("1 | 0\n");
        assert_eq!(c.problems.len(), 1, "{:?}", c.problems);
        assert_eq!(c.award(BEGINNER, 1, 0, 100).levels, 0);
    }
}
