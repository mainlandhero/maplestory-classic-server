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
//! # NOTHING HERE IS WIRED. There is no packet yet.
//!
//! `crates/net/src/opcode.rs` records why, on `Character::exp`: the stat block **forks on the
//! job**, and job `0` - every character this server has - takes the *extended* branch, where
//! SP is a count followed by per-pool entries rather than a `u16`. That encoding has not been
//! read, so a skill point computed here cannot yet be put on screen.
//!
//! This module is therefore the **rules only**, and it is deliberately a pure function of the
//! character's level so that it can be finished and tested without a client run. `STATUS.md`
//! marks it as unwired; say so when reporting progress rather than letting "implemented" be
//! heard as "working".
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
pub const SECOND_JOB_LEVEL: u32 = 30;

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
}

impl Tier {
    /// The level at which this tier's advancement becomes possible.
    pub fn starts_at(self) -> u32 {
        match self {
            Tier::First => FIRST_JOB_LEVEL,
            Tier::Second => SECOND_JOB_LEVEL,
        }
    }

    /// The level at which this tier stops accruing, if it does.
    ///
    /// The first tier stops at [`SECOND_JOB_LEVEL`] because the second takes over there.
    /// **Level 30 itself still pays the first tier**: a character is still first-job while
    /// levelling *to* 30, and only advances afterwards. The owner's *"until level 30"* is read as
    /// inclusive for that reason, and it is flagged here because the other reading is
    /// defensible and costs exactly three points.
    pub fn stops_at(self) -> Option<u32> {
        match self {
            Tier::First => Some(SECOND_JOB_LEVEL),
            Tier::Second => None,
        }
    }
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Tier::First => "1st job",
            Tier::Second => "2nd job",
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
        for level in SECOND_JOB_LEVEL..80 {
            let step = entitlement(Tier::Second, level + 1) - entitlement(Tier::Second, level);
            assert_eq!(step, SP_PER_LEVEL, "level {level} -> {}", level + 1);
        }
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
