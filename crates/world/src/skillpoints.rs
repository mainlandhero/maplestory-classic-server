//! How many skill points a character is owed, per job tier.
//!
//! The owner, 2026-08-27: *"Once I became a Magician (or any job at level 10), I should immediately
//! get 1 skill point for 1st job. Currently I get none. For every level above 10 until level
//! 30, I should receive 3 skill points in 1st job. Same once the player reached level 30 and
//! completes the 2nd job, they would get 1 skill point for 2nd job, then 3 skill points for
//! every level they achieve thereafter. The player should not lose skill points if they job
//! advance late. If they job advance at level 11, they should receive 4 skill points
//! retroactively once they become that 1st job."*
//!
//! # WIRED as of 2026-08-27, and never yet on a wire
//!
//! This block used to say there was no packet. There is one now. The extended-SP encoding was
//! decoded (`research/skill-points.md`): `u8 count`, then `count` x (`u8 tier`, `u32 amount`),
//! and **the pool key is a job TIER 0..=10, not a job id** - `FUN_1402CB030` returns 0 for any
//! key above 10, so a job id would read an empty pool for every job in the game. `gm_job` now
//! sends the job change and the table in **one** `0x007C`.
//!
//! **No client has seen it.** Wired is not confirmed, and this is the module where that
//! distinction has already cost a day.
//!
//! # What is still missing: SPENDING
//!
//! The amount is computed from the **level** alone. That is what makes [`top_up`] idempotent
//! and the retroactive rule free - but nothing records what has been **spent**, so a point the
//! player spends comes back on the next advancement. Say that when reporting, because a player
//! who spends and sees it return will otherwise file it as a bug.
//!
//! # Why entitlement, and not "add 3 on level-up"
//!
//! The retroactive requirement is the whole design. An award that fires on the level-up event
//! has to be replayed to be made retroactive, and replaying events is how a grant gets applied
//! twice. Expressing the rule as a **total owed at this level** makes the late-advancement case
//! fall out for free and makes the top-up idempotent: run it a hundred times and the second
//! run grants nothing.
//!
//! It also survives the thing this project keeps hitting - a handler that runs twice. The
//! Heena quest paid out on every click because the payout hung off the *request* rather than
//! the *transition*. [`top_up`] cannot do that, because it asks "how many are missing", not
//! "how many should I add".

use std::fmt;

/// The level at which the first advancement becomes possible. Corroborated by the client's own
/// `Quest.wz/QuestData/10001.img` text - see [`crate::jobs::LEVEL_MINIMUM`], which is the same
/// number and the one to change if it is ever wrong.
pub const FIRST_JOB_LEVEL: u32 = crate::jobs::LEVEL_MINIMUM;

/// The level at which the second advancement becomes possible. **[I]** - the owner's sentence, and
/// nothing in this client has been found that states it.
///
/// It is corroborated *since*: all sixteen second-job chain quests carry `Check.0.lvmin = 30`.
/// **[L]** `research/second-job.md`. So this one is no longer only a policy.
pub const SECOND_JOB_LEVEL: u32 = 30;

/// The level at which the third advancement becomes possible.
///
/// **[I], and unlike [`SECOND_JOB_LEVEL`] there is nothing in this client to corroborate it.**
/// The second advancement's level could be read off sixteen quests; the third advancement has
/// **no quest at all** - `research/third-job.md` §3.1 enumerates all 322 and finds nothing
/// above `20303`. So 70 is the conventional MapleStory number and nothing on this machine will
/// ever catch it being wrong. One constant, one edit, said out loud rather than buried.
pub const THIRD_JOB_LEVEL: u32 = 70;

/// Granted once, on the advancement itself.
pub const SP_ON_ADVANCE: u32 = 1;

/// Granted for each level gained after the tier's own starting level.
pub const SP_PER_LEVEL: u32 = 3;

/// Which pool a point belongs to.
///
/// **Two pools, not one running total**, because the owner's rule is explicitly per-tier and the
/// client's extended-SP encoding is per-pool as well. Collapsing them into a scalar would make
/// second-job points spendable on first-job skills, which is the direction that cannot be
/// undone once a player has spent them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier {
    First,
    Second,
    Third,
}

impl Tier {
    /// The level at which this tier's advancement becomes possible.
    pub fn starts_at(self) -> u32 {
        match self {
            Tier::First => FIRST_JOB_LEVEL,
            Tier::Second => SECOND_JOB_LEVEL,
            Tier::Third => THIRD_JOB_LEVEL,
        }
    }

    /// The level at which this tier stops accruing, if it does.
    ///
    /// The first tier stops at [`SECOND_JOB_LEVEL`] because the second takes over there.
    /// **Level 30 itself still pays the first tier**: a character is still first-job while
    /// levelling *to* 30, and only advances afterwards. The owner's *"until level 30"* is read as
    /// inclusive for that reason, and it is flagged here because the other reading is
    /// defensible and costs exactly three points.
    /// **The second tier now stops too, and that is a behaviour change worth naming.**
    ///
    /// It used to be `None` because there was no third tier to take over. There is one now, so
    /// the rule this function already documented for `First` applies to `Second` unchanged: a
    /// character who reaches 70 and does *not* advance stops accruing second-job points, in
    /// exactly the way a character who reaches 30 and does not advance stops accruing
    /// first-job points. That symmetry is the whole reason `stops_at` exists.
    ///
    /// **Nobody loses a point they already had.** [`top_up`] is `saturating_sub`, so a
    /// character granted 211 points under the old `None` keeps all 211 and simply gains no
    /// more. There is no negative correction, which is the one thing this module refuses to
    /// do - a point taken back may already have been spent.
    pub fn stops_at(self) -> Option<u32> {
        match self {
            Tier::First => Some(SECOND_JOB_LEVEL),
            Tier::Second => Some(THIRD_JOB_LEVEL),
            Tier::Third => None,
        }
    }
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Tier::First => "1st job",
            Tier::Second => "2nd job",
            Tier::Third => "3rd job",
        })
    }
}

/// **Every point this tier has ever been owed at `level`**, assuming the character has
/// advanced into it.
///
/// A total, not an increment - see the module docs. `0` before the tier's starting level.
///
/// ```text
/// tier    level     entitlement
/// First      9      0
/// First     10      1          the advancement itself
/// First     11      4          1 + 3            <- the owner's worked example
/// First     30      61         1 + 3 x 20
/// First     45      61         capped; the second tier has taken over
/// Second    29      0
/// Second    30      1
/// Second    31      4
/// ```
pub fn entitlement(tier: Tier, level: u32) -> u32 {
    let start = tier.starts_at();
    if level < start {
        return 0;
    }
    let counted = match tier.stops_at() {
        Some(stop) => level.min(stop),
        None => level,
    };
    SP_ON_ADVANCE + SP_PER_LEVEL * (counted - start)
}

/// **The first-job pool on this server: it keeps growing past 30 until the whole book can be
/// maxed.** A deliberate deviation from classic MapleStory - the owner, 2026-10-02, from player
/// complaints: *"Allow continuous accumulation of skill points for 1st job beyond level 30 until
/// all skills can be maxed in first job."*
///
/// `to_max_book` is the sum of every first-job skill's `maxLevel` in the character's own book:
/// 105 for Warrior, Magician and Bowman, 110 for Thief in this client. The pool keeps paying
/// [`SP_PER_LEVEL`] a level after 30 and stops at the first grant that covers the book - 106 at
/// level 45, 112 at 47 for a Thief. Never less than the classic [`entitlement`], so a book the
/// classic 61 already covers (or an empty skill table, `0`) changes nothing. The second-job
/// pool is untouched: past 30 both pools grow.
pub fn first_job_entitlement(level: u32, to_max_book: u32) -> u32 {
    let classic = entitlement(Tier::First, level);
    let start = Tier::First.starts_at();
    if level < start {
        return classic;
    }
    let uncapped = SP_ON_ADVANCE + SP_PER_LEVEL * (level - start);
    let covers_book =
        SP_ON_ADVANCE + SP_PER_LEVEL * to_max_book.saturating_sub(SP_ON_ADVANCE).div_ceil(SP_PER_LEVEL);
    uncapped.min(covers_book.max(classic))
}

/// **How many points to grant now**, given how many this tier has already been granted.
///
/// This is the whole retroactivity mechanism and the whole double-grant guard. It is
/// idempotent: calling it again immediately returns `0`.
///
/// It **never returns a negative correction**. If `already_granted` exceeds the entitlement -
/// a character who was granted under an older rule, or a level that went down - this returns
/// `0` rather than trying to claw points back. Taking a point away from a player who may
/// already have spent it is not a correction, it is a second bug.
pub fn top_up(tier: Tier, level: u32, already_granted: u32) -> u32 {
    entitlement(tier, level).saturating_sub(already_granted)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **Past 30 the first-job pool keeps paying until the book can be maxed, then stops.**
    /// 105 is a Magician's book (six skills, 15/15/15/20/20/20), 110 a Thief's.
    #[test]
    fn the_first_job_pool_grows_until_the_whole_book_can_be_maxed() {
        for level in 0..=30 {
            assert_eq!(first_job_entitlement(level, 105), entitlement(Tier::First, level), "unchanged to 30: {level}");
        }
        assert_eq!(first_job_entitlement(31, 105), 64, "30 no longer ends it");
        assert_eq!(first_job_entitlement(44, 105), 103, "one level short of the book");
        assert_eq!(first_job_entitlement(45, 105), 106, "the first grant that covers 105");
        assert_eq!(first_job_entitlement(90, 105), 106, "and it stops there");
        assert_eq!(first_job_entitlement(46, 110), 109);
        assert_eq!(first_job_entitlement(47, 110), 112, "a Thief's 110");
        assert_eq!(first_job_entitlement(120, 110), 112);
        // An empty skill table, or a book the classic 61 covers: the classic rule exactly.
        for level in [10, 30, 31, 70, 200] {
            assert_eq!(first_job_entitlement(level, 0), entitlement(Tier::First, level));
            assert_eq!(first_job_entitlement(level, 40), entitlement(Tier::First, level));
        }
    }

    /// **The owner's two worked examples, verbatim.**
    #[test]
    fn the_examples_from_the_request() {
        // "Once I became a Magician (or any job at level 10), I should immediately get 1
        // skill point for 1st job."
        assert_eq!(entitlement(Tier::First, 10), 1);
        // "If they job advance at level 11, they should receive 4 skill points retroactively."
        assert_eq!(entitlement(Tier::First, 11), 4);
        // "they would get 1 skill point for 2nd job, then 3 skill points for every level"
        assert_eq!(entitlement(Tier::Second, 30), 1);
        assert_eq!(entitlement(Tier::Second, 31), 4);
    }

    /// **Advancing late costs nothing.** This is the property the whole design exists for, so
    /// it is asserted as an equality between two histories rather than as two numbers.
    #[test]
    fn advancing_late_gives_exactly_what_advancing_early_would_have() {
        // Advanced at 10, then levelled to 11 - two top-ups.
        let mut granted = 0;
        granted += top_up(Tier::First, 10, granted);
        granted += top_up(Tier::First, 11, granted);

        // Advanced at 11 - one top-up, and never saw level 10 as a first-job character.
        let late = top_up(Tier::First, 11, 0);

        assert_eq!(granted, late, "the history must not change the total");
        assert_eq!(granted, 4);
    }

    /// **Running the top-up again grants nothing**, however many times it runs.
    ///
    /// The Heena quest paid out on every click because the payout hung off the request rather
    /// than the transition. This function cannot do that, and here is the proof.
    #[test]
    fn topping_up_twice_is_the_same_as_topping_up_once() {
        let first = top_up(Tier::First, 17, 0);
        assert_eq!(first, 1 + 3 * 7);
        for _ in 0..100 {
            assert_eq!(top_up(Tier::First, 17, first), 0, "idempotent");
        }
    }

    /// Below the tier's level nothing is owed, and the boundary is exact.
    #[test]
    fn nothing_is_owed_before_the_tier_opens() {
        for level in 0..FIRST_JOB_LEVEL {
            assert_eq!(entitlement(Tier::First, level), 0, "level {level}");
        }
        assert_eq!(entitlement(Tier::First, FIRST_JOB_LEVEL), SP_ON_ADVANCE);
        for level in 0..SECOND_JOB_LEVEL {
            assert_eq!(entitlement(Tier::Second, level), 0, "level {level}");
        }
        assert_eq!(entitlement(Tier::Second, SECOND_JOB_LEVEL), SP_ON_ADVANCE);
    }

    /// **The first tier stops at 30 and the second starts there**, so no level pays twice into
    /// the same pool and none is skipped between them.
    #[test]
    fn the_first_tier_caps_where_the_second_begins() {
        assert_eq!(entitlement(Tier::First, 30), 61, "1 + 3 x 20");
        for level in 30..60 {
            assert_eq!(
                entitlement(Tier::First, level),
                61,
                "level {level} must not keep paying the first pool"
            );
        }
        // Every level from 31 up adds exactly SP_PER_LEVEL to the second pool and nothing to
        // the first - which is the same statement, checked from the other side.
        //
        // **This loop used to run to 80 and it caught the change that added a third tier**,
        // which is what a test is for. The second pool now stops at [`THIRD_JOB_LEVEL`] for
        // exactly the reason the first stops at [`SECOND_JOB_LEVEL`]: the next tier takes
        // over. The bound is written as the constant rather than as `70` so the two cannot
        // drift.
        for level in SECOND_JOB_LEVEL..THIRD_JOB_LEVEL {
            let step = entitlement(Tier::Second, level + 1) - entitlement(Tier::Second, level);
            assert_eq!(step, SP_PER_LEVEL, "level {level} -> {}", level + 1);
        }
        // And past it, the second pool is flat - the symmetric statement to the one above
        // about the first pool.
        let at_seventy = entitlement(Tier::Second, THIRD_JOB_LEVEL);
        assert_eq!(at_seventy, 1 + SP_PER_LEVEL * (THIRD_JOB_LEVEL - SECOND_JOB_LEVEL));
        for level in THIRD_JOB_LEVEL..THIRD_JOB_LEVEL + 30 {
            assert_eq!(
                entitlement(Tier::Second, level),
                at_seventy,
                "level {level} must not keep paying the second pool"
            );
        }
    }

    /// **The third tier, and the one thing it does differently: it never stops.**
    ///
    /// There is no fourth job in this client - `Skill.wz` holds 25 books and the highest are
    /// the ten third-job ones - so nothing takes over from this pool and capping it would
    /// strand every point a character earns past whatever number was chosen.
    #[test]
    fn the_third_tier_starts_at_seventy_and_never_caps() {
        assert_eq!(entitlement(Tier::Third, 69), 0, "before the advancement, nothing");
        assert_eq!(entitlement(Tier::Third, 70), 1, "the advancement itself");
        assert_eq!(entitlement(Tier::Third, 71), 4, "then three a level");
        assert_eq!(Tier::Third.starts_at(), THIRD_JOB_LEVEL);
        assert_eq!(Tier::Third.stops_at(), None, "nothing takes over from the third pool");
        for level in THIRD_JOB_LEVEL..200 {
            let step = entitlement(Tier::Third, level + 1) - entitlement(Tier::Third, level);
            assert_eq!(step, SP_PER_LEVEL, "level {level} -> {}", level + 1);
        }
        // Advancing late still costs nothing - the property the whole module exists for,
        // asserted for the new tier rather than assumed to carry over.
        let mut granted = 0;
        granted += top_up(Tier::Third, 70, granted);
        granted += top_up(Tier::Third, 71, granted);
        assert_eq!(granted, top_up(Tier::Third, 71, 0), "history must not change the total");

        // **And nobody loses a point they already had.** A character granted under the old
        // uncapped second-tier rule keeps every one: `top_up` saturates rather than clawing
        // back, and a point taken away may already have been spent.
        let under_the_old_rule = 1 + SP_PER_LEVEL * (120 - SECOND_JOB_LEVEL);
        assert_eq!(top_up(Tier::Second, 120, under_the_old_rule), 0, "no negative correction");
        assert_eq!(Tier::Third.to_string(), "3rd job");
    }

    /// **A surplus is never clawed back.** A point already granted may already have been
    /// spent, so taking it away is a second bug rather than a correction.
    #[test]
    fn an_over_grant_is_left_alone_rather_than_reversed() {
        assert_eq!(top_up(Tier::First, 10, 999), 0);
        assert_eq!(top_up(Tier::First, 5, 4), 0, "and a level that went down does not either");
    }

    /// The two tiers are separate pools, and a level pays into exactly one of them.
    #[test]
    fn the_pools_do_not_bleed_into_each_other() {
        assert_eq!(entitlement(Tier::Second, 20), 0, "a level-20 character has no 2nd job SP");
        assert_ne!(entitlement(Tier::First, 40), entitlement(Tier::Second, 40));
        assert!(Tier::First < Tier::Second, "ordered, so a UI can list them in tier order");
    }
}
