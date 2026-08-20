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

/// The highest level this table describes. A character there stops gaining.
pub const MAX_LEVEL: u32 = 100;

/// What one level awards. **Policy, not measured - `[I]`.**
///
/// Ability points at five a level is this game family's long-standing rule and the one
/// number here worth any confidence. The HP and MP gains are a beginner's, flat rather than
/// rolled: a range would make two runs of the same actions produce different characters,
/// which is a bad property for something nobody has verified.
///
/// SP is deliberately **not** awarded. A beginner has no skills to spend it on, and the
/// stat block's SP field forks on the job - the branch every character here takes sends a
/// pool list nobody has decoded. Job advancement is where that gets read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelGains {
    pub ap: u16,
    pub max_hp: u32,
    pub max_mp: u32,
}

impl Default for LevelGains {
    fn default() -> Self {
        LevelGains { ap: 5, max_hp: 14, max_mp: 10 }
    }
}

/// How much experience each level costs, and what levelling awards.
#[derive(Debug, Clone, Default)]
pub struct ExpCurve {
    /// level -> experience needed to reach `level + 1`.
    to_next: HashMap<u32, u64>,
    pub gains: LevelGains,
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
    pub fn award(&self, level: u32, exp: u64, gained: u64) -> Awarded {
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
            out.ap += self.gains.ap;
            out.max_hp += self.gains.max_hp;
            out.max_mp += self.gains.max_mp;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn curve() -> ExpCurve {
        ExpCurve::parse("1 | 15\n2 | 34\n3 | 57\n")
    }

    #[test]
    fn the_real_file_parses_and_rises() {
        let c = ExpCurve::load(Path::new("../../data/exp-curve.txt"));
        assert!(c.problems.is_empty(), "{:?}", c.problems);
        assert_eq!(c.levels(), 99, "levels 1..99");
        assert_eq!(c.to_next(1), Some(15), "the classic first level");
        assert_eq!(c.to_next(99), Some(9_692_044));
        assert_eq!(c.to_next(100), None, "and 100 has nowhere to go");
        // Monotonic: a later level must never be cheaper than an earlier one.
        for l in 1..99 {
            assert!(
                c.to_next(l).unwrap() <= c.to_next(l + 1).unwrap(),
                "level {l} costs more than level {}",
                l + 1
            );
        }
    }

    #[test]
    fn not_enough_experience_is_not_a_level() {
        let a = curve().award(1, 0, 14);
        assert_eq!((a.level, a.exp, a.levels), (1, 14, 0));
        assert_eq!(a.ap, 0, "and nothing is awarded");
    }

    #[test]
    fn exactly_enough_levels_once_and_leaves_nothing_over() {
        let a = curve().award(1, 0, 15);
        assert_eq!((a.level, a.exp, a.levels), (2, 0, 1));
        assert_eq!(a.ap, LevelGains::default().ap);
    }

    /// **Experience carries over.** A single award worth several levels grants several.
    #[test]
    fn a_large_award_levels_more_than_once_and_keeps_the_remainder() {
        let a = curve().award(1, 0, 15 + 34 + 10);
        assert_eq!((a.level, a.exp, a.levels), (3, 10, 2));
        assert_eq!(a.ap, LevelGains::default().ap * 2, "two levels, two awards");
    }

    /// A level with no entry stops levelling rather than looping or panicking.
    #[test]
    fn running_off_the_end_of_the_table_stops() {
        let a = curve().award(3, 0, 1_000_000);
        assert_eq!(a.level, 4, "level 3 has an entry, level 4 does not");
        assert_eq!(a.exp, 1_000_000 - 57);
    }

    #[test]
    fn the_maximum_level_stops_gaining_levels_but_still_banks_experience() {
        let c = ExpCurve::parse("99 | 10\n100 | 10\n");
        let a = c.award(MAX_LEVEL, 0, 1_000);
        assert_eq!((a.level, a.levels), (MAX_LEVEL, 0));
        assert_eq!(a.exp, 1_000, "the experience is still banked");
    }

    #[test]
    fn an_empty_curve_never_levels_anything() {
        let c = ExpCurve::default();
        let a = c.award(1, 0, u64::MAX / 2);
        assert_eq!(a.levels, 0);
        assert!(c.is_empty());
    }

    #[test]
    fn a_zero_cost_level_is_rejected_rather_than_looping_forever() {
        let c = ExpCurve::parse("1 | 0\n");
        assert_eq!(c.problems.len(), 1, "{:?}", c.problems);
        assert_eq!(c.award(1, 0, 100).levels, 0);
    }
}
