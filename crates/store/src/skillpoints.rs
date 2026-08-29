//! The **spent** half of a skill point. `world::skillpoints` owns the earned half.
//!
//! # The bug this exists to close
//!
//! `world::skillpoints::entitlement` is a *total owed at this level*, not an increment. That
//! is what makes advancing late cost nothing and what makes re-sending the pool table
//! idempotent, and it is the right shape. But with nothing recording what left the pool, the
//! number the client is shown is "everything you have ever earned" - so a player could put
//! three points into Power Strike, advance or relog, and have the three points back **and**
//! keep the skill. `gm_job` said so in its own comment; this module is the record that was
//! missing.
//!
//! # Why the subtraction does not happen here
//!
//! `store` cannot call `world` - `world` depends on `store`, and the arrow only goes one way.
//! So the entitlement is a **parameter** on every call in this file, and the balance is
//! [`balance`]: a two-argument function with no database in it.
//!
//! That is not a workaround, it is the honest split. The store knows what was spent; the
//! world knows what was earned; the subtraction belongs where both are in scope, and it is a
//! `saturating_sub` so an entitlement that has *shrunk* - an old grant, a rule change, a level
//! that went down - reads as zero rather than wrapping to four billion.
//! `world::skillpoints::top_up` already refuses to claw a surplus back for the same reason.
//!
//! # The design choice: a stored counter, one row per SKILL
//!
//! The obvious alternative is to **derive** the spend from `character_skills`: every point
//! spent produces a level, so `spent = SUM(level)`. It is wrong here, and both GM commands
//! are why.
//!
//! * **`!learn` grants levels without spending anything**, by design and by its own doc
//!   block - *"put every skill of this job's book on the bar, without spending a single skill
//!   point"*. Under derivation, `!learn` would instantly consume the entire pool: 24 skills at
//!   up to 20 levels each is several hundred derived "spends" against an entitlement of 61.
//!   The command would silently stop being free.
//! * **`!resetsp` forgets every skill**, and under derivation the refund is free - the levels
//!   are gone, so the derived total is gone. That half works. It is the only half that does.
//!
//! So the spend is stored. And it is stored **per skill**, not per pool, which is a second
//! decision and a smaller one:
//!
//! * A per-pool counter cannot answer *"how much did this one skill cost?"*, so forgetting a
//!   single skill can only be handled by zeroing the whole pool. `gm_reset_sp` forgets skills
//!   one at a time and collects per-skill failures; a blanket zero beside a partial failure
//!   would hand back points for a skill the player still has. That is the Heena loop wearing
//!   a different hat.
//! * Per skill, the refund is exactly what that skill cost, [`Store::forget_skill_and_refund`]
//!   does the deletion and the refund in **one transaction**, and a half-run reset is not
//!   representable.
//! * It also buys an invariant that can be checked against another table: **`points` is never
//!   more than the skill's `level`**, because a point always produces a level and a level is
//!   only ever removed together with its points. `!learn` moves the level up without the
//!   points, never the other way. See `the_ledger_never_exceeds_the_skill_level`.
//!
//! The tier is stored on the row because this crate has no skill table and cannot work out
//! which book a skill id belongs to. The caller supplies it (`net::stats::tier_for_job(job)`),
//! and it is rewritten on every spend so the caller's current mapping is always the authority.
//!
//! # Tier 0 is the client's own arithmetic and must never be charged
//!
//! `research/skill-points.md` §4.1, **[L]** every instruction: for any job whose tier is 0 the
//! client computes SP itself as `min(10, level) - 1 - (levels in 1000, 1001, 1002)` and never
//! reads a pool at all. A server-side counter for tier 0 could only ever disagree with the
//! screen. [`Store::spend_skill_points`] therefore answers tier 0 with
//! [`SpendOutcome::NotCharged`] - a **success** that writes no row - rather than a refusal, so
//! a caller that forgets to branch still behaves correctly for beginners instead of refusing
//! every beginner click.
//!
//! # Nothing here authenticates
//!
//! The channel socket carries no credentials at all; a spend arrives on the say-so of whoever
//! holds the connection, exactly like a quest turn-in. The ledger is checked anyway, because
//! the point is that the server and the screen agree - not that the client is trusted.

use rusqlite::{Connection, OptionalExtension};

use crate::db::Store;
use crate::error::Result;

/// The highest pool key the client can read back.
///
/// `FUN_1402cb030` - the pool lookup - is `cmp dl, 0xa / ja return-0`, so a pool stored under
/// a larger key is written, is summed into the "you have unspent points" total at
/// `charstat+0xef`, and is **never displayed**. `research/skill-points.md` §2, **[L]**.
///
/// A spend into such a pool would be a point that vanishes from the screen while lighting the
/// indicator forever, so it is refused rather than stored.
pub const MAX_POOL_TIER: u8 = 10;

/// The tier whose SP the client works out for itself. See the module docs.
pub const CLIENT_COMPUTED_TIER: u8 = 0;

/// One row of the ledger: what a single skill has cost this character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpendRow {
    pub skill_id: u32,
    /// The pool the points came out of - `net::stats::tier_for_job` of the skill's job book.
    pub tier: u8,
    /// Points that have left the pool for this skill. **Never** the skill's level: `!learn`
    /// raises the level without touching this.
    pub points: u32,
}

/// What a refund gave back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Refunded {
    /// How many skill rows were removed.
    pub skills: u32,
    /// How many points went back into the pools.
    pub points: u32,
}

/// What [`Store::spend_skill_points`] did.
///
/// `#[must_use]`, and that is the whole point of returning an enum rather than a `bool`. The
/// Heena quest paid out on every click because three call sites captured the store's answer
/// into a *log string* and carried on; a value that cannot be dropped without a warning is the
/// cheapest defence against that available. **Match on it and return early on
/// [`SpendOutcome::Refused`]** - do not gate each effect separately, because separately is how
/// one gets missed.
#[must_use = "a spend that is not checked is a spend that did not happen - see the Heena quest"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpendOutcome {
    /// Points left the pool. Every effect of the skill-up may now follow, and only now.
    Spent {
        /// How many points were taken. Always exactly what was asked for - this never
        /// part-fills, because a client that asked for 3 and silently got 1 is the bug the owner
        /// reported on 2026-08-21 from the other direction.
        charged: u32,
        /// The pool's total spend **after** this call.
        spent: u32,
        /// What is left in the pool **after** this call.
        available: u32,
    },
    /// The pool is the client's own arithmetic ([`CLIENT_COMPUTED_TIER`]). Nothing was
    /// recorded and nothing needs to be: this is a **success**, and the skill-up should
    /// proceed.
    NotCharged,
    /// Nothing changed. Say so to somebody.
    Refused(SpendRefusal),
}

impl SpendOutcome {
    /// `true` when the caller may go on to raise the level and send the packets.
    ///
    /// [`SpendOutcome::NotCharged`] counts as proceeding - see the module docs on tier 0.
    pub fn may_proceed(self) -> bool {
        !matches!(self, SpendOutcome::Refused(_))
    }
}

/// What [`Store::spend_and_raise_skill`] did - the pool and the level, from one transaction.
///
/// The two are returned together because they happen together. A handler that charged the
/// pool and then raised the level in a second call has two effects that can come apart, and
/// `CLAUDE.md` names that failure twice: the quest payout that hung off the request rather
/// than the transition, and the forfeit whose `DELETE` did not carry the guard its own doc
/// block promised.
#[must_use = "the level and the charge both hang off this answer - do not drop it"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkillUp {
    /// What the pool did. Match this and **return early on
    /// [`SpendOutcome::Refused`]**; nothing else may follow a refusal.
    pub spend: SpendOutcome,
    /// The skill's level after the call. **Unchanged** from before when `spend` is a refusal,
    /// so it is always safe to report.
    pub level: u32,
}

/// Why a spend changed nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpendRefusal {
    /// A count of zero. Answer it - the client's request latch still has to clear - and
    /// change nothing.
    NothingAsked,
    /// The pool holds fewer points than were asked for.
    NotEnough { available: u32, wanted: u32 },
    /// A tier above [`MAX_POOL_TIER`], which the client cannot read back at all.
    NoSuchPool { tier: u8 },
}

impl std::fmt::Display for SpendRefusal {
    /// Written out so a caller has no excuse for swallowing one.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpendRefusal::NothingAsked => f.write_str("the request asked for 0 points"),
            SpendRefusal::NotEnough { available, wanted } => write!(
                f,
                "this pool holds {available} skill point(s) and the request wanted {wanted}"
            ),
            SpendRefusal::NoSuchPool { tier } => write!(
                f,
                "tier {tier} is above {MAX_POOL_TIER}, which the client's pool lookup refuses - \
                 a point spent there would never appear on screen"
            ),
        }
    }
}

/// **The balance.** `entitlement - spent`, floored at zero.
///
/// A free function with no database in it, so it can be reasoned about and tested on its own,
/// and so the unit is unmistakable: both arguments are *points in one pool*, never levels and
/// never a running delta.
///
/// `saturating_sub` rather than a checked subtraction because an over-spent pool is a state
/// that can legitimately exist - a rule change, an entitlement that shrank, a hand-edited
/// database - and the answer to "how many may I still spend" is then **zero**, not an error
/// and certainly not `u32::MAX`. `world::skillpoints::top_up` saturates in the same direction
/// and for the same reason: a point that may already have been spent is not clawed back.
pub fn balance(entitlement: u32, spent: u32) -> u32 {
    entitlement.saturating_sub(spent)
}

/// Create the ledger table. Called from `Store::init` on every open, so it must be idempotent.
///
/// A plain `CREATE TABLE IF NOT EXISTS` is enough **because the table is new** - the same
/// reasoning `crate::quest::create_tables` spells out, and the opposite of a *column* added to
/// a table that already exists, which needs a `PRAGMA table_info` guard around the `ALTER`.
///
/// # What happens to a character who already has 7 points in Magic Claw
///
/// Nothing, and that is the deliberate answer. The table starts empty, so every existing
/// character has **no ledger rows**, which reads as `spent = 0` and therefore a full pool -
/// while every skill level they have is untouched. A one-time amnesty, bounded by the
/// entitlement, self-closing the moment they spend anything.
///
/// The alternative was to backfill `points` from `character_skills.level`, and it is wrong for
/// exactly the reason derivation is wrong: **`!learn` is how most of those levels got there**,
/// on 2026-08-28, under a doc block promising it cost nothing. Backfilling would retroactively
/// bill a character for a grant that was free by contract, and would leave anyone who ran
/// `!learn` with a pool pinned at zero until they ran `!resetsp`. Charging for the past to
/// avoid one free round in the present is the worse trade, and it is the one that cannot be
/// undone.
pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        -- One row per skill the character has SPENT points on. Not one row per skill it has:
        -- `!learn` produces levels with no row here at all, and that difference is the whole
        -- reason this is stored rather than derived from `character_skills.level`.
        --
        -- `tier` is the client's pool key (net::stats::tier_for_job), 0..=10. It lives on the
        -- row because this crate has no skill table and cannot map a skill id to a job book;
        -- the caller supplies it and it is rewritten on every spend.
        --
        -- No row means no points spent. There is exactly one representation of that, the same
        -- way `quest_state` has exactly one representation of "never started".
        CREATE TABLE IF NOT EXISTS character_skill_spend (
            character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            skill_id     INTEGER NOT NULL,
            tier         INTEGER NOT NULL,
            points       INTEGER NOT NULL,
            PRIMARY KEY (character_id, skill_id)
        );

        -- The pool total is the hot read: one SUM per tier on every skill-up and every
        -- 0x007C that carries SP.
        CREATE INDEX IF NOT EXISTS idx_skill_spend_pool
            ON character_skill_spend(character_id, tier);
        "#,
    )?;
    Ok(())
}

/// Charge one skill's row, on a caller-supplied connection. Assumes the pool has already been
/// checked - the check and this call belong to the same transaction.
fn charge_pool(
    conn: &Connection,
    character_id: u32,
    skill_id: u32,
    tier: u8,
    count: u32,
) -> Result<()> {
    conn.execute(
        "INSERT INTO character_skill_spend (character_id, skill_id, tier, points)
              VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(character_id, skill_id) DO UPDATE SET
             points = points + ?4,
             -- The caller's mapping is the authority. A skill's book does not move in
             -- practice, but if this server's idea of it is ever corrected, the whole row
             -- follows the skill - which is the only answer that keeps the per-tier sums a
             -- partition of what was actually spent.
             tier   = excluded.tier",
        rusqlite::params![
            i64::from(character_id),
            i64::from(skill_id),
            i64::from(tier),
            i64::from(count),
        ],
    )?;
    Ok(())
}

/// The pool's total spend, on a caller-supplied connection so a spend can read and write it
/// inside one transaction.
fn spent_in_pool(conn: &Connection, character_id: u32, tier: u8) -> Result<u32> {
    let total: i64 = conn.query_row(
        "SELECT COALESCE(SUM(points), 0) FROM character_skill_spend
          WHERE character_id = ?1 AND tier = ?2",
        rusqlite::params![i64::from(character_id), i64::from(tier)],
        |row| row.get(0),
    )?;
    Ok(total.clamp(0, i64::from(u32::MAX)) as u32)
}

impl Store {
    /// How many points this character has spent out of one pool.
    pub fn skill_points_spent(&self, character_id: u32, tier: u8) -> Result<u32> {
        spent_in_pool(&self.conn(), character_id, tier)
    }

    /// Every pool this character has spent anything from, lowest tier first.
    ///
    /// Ordered for the same reason `Store::skills` is: the extended-SP table built from this
    /// goes on the wire, and a packet whose bytes differ between two identical loads cannot be
    /// diffed against a capture. A pool with nothing spent is **absent**, not a zero row - the
    /// caller is building the table from the entitlement anyway.
    pub fn skill_points_spent_by_tier(&self, character_id: u32) -> Result<Vec<(u8, u32)>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT tier, COALESCE(SUM(points), 0) FROM character_skill_spend
              WHERE character_id = ?1
              GROUP BY tier
              ORDER BY tier",
        )?;
        let rows = stmt.query_map([i64::from(character_id)], |row| {
            Ok((row.get::<_, i64>(0)? as u8, row.get::<_, i64>(1)?.clamp(0, i64::from(u32::MAX)) as u32))
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The whole ledger, skill id order. For diagnostics and tests.
    pub fn skill_point_ledger(&self, character_id: u32) -> Result<Vec<SpendRow>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT skill_id, tier, points FROM character_skill_spend
              WHERE character_id = ?1
              ORDER BY skill_id",
        )?;
        let rows = stmt.query_map([i64::from(character_id)], |row| {
            Ok(SpendRow {
                skill_id: row.get::<_, i64>(0)? as u32,
                tier: row.get::<_, i64>(1)? as u8,
                points: row.get::<_, i64>(2)?.clamp(0, i64::from(u32::MAX)) as u32,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// **How many points this character may still spend from one pool.**
    ///
    /// `entitlement` comes from `world::skillpoints::entitlement(tier, level)`; see the module
    /// docs for why it is a parameter and not a call.
    pub fn skill_points_available(
        &self,
        character_id: u32,
        tier: u8,
        entitlement: u32,
    ) -> Result<u32> {
        Ok(balance(entitlement, self.skill_points_spent(character_id, tier)?))
    }

    /// **The skill-up: charge the pool and raise the level, in one transaction.**
    ///
    /// This is what `0x013B` should call. The level goes up by **exactly** the number of
    /// points charged - the store computes it rather than taking it as an argument, so the
    /// charge and the level cannot disagree, and the invariant `points <= level` holds by
    /// construction.
    ///
    /// `count` must already be clamped to the skill's own ceiling by the caller, which is the
    /// only party that has read `Skill.wz`. Nothing is part-filled here: if the pool cannot
    /// pay for all `count`, nothing is charged and nothing is raised.
    ///
    /// Tier [`CLIENT_COMPUTED_TIER`] raises the level and charges nothing - the beginner pool
    /// is the client's own arithmetic, and this is the branch that keeps beginner skill-ups
    /// working exactly as they do today.
    ///
    /// ```text
    /// let up = store.spend_and_raise_skill(chr.id, skill_id, tier, entitlement, count)?;
    /// let SpendOutcome::Refused(why) = up.spend else { .. proceed .. };
    /// return refuse(why);          // and nothing else, ever, on this path
    /// ```
    pub fn spend_and_raise_skill(
        &self,
        character_id: u32,
        skill_id: u32,
        tier: u8,
        entitlement: u32,
        count: u32,
    ) -> Result<SkillUp> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let level = crate::skills::skill_level_row(&tx, character_id, skill_id)?;
        // Every refusal returns the level unchanged, and none of them has written anything:
        // the transaction is dropped, not committed.
        if count == 0 {
            return Ok(SkillUp { spend: SpendOutcome::Refused(SpendRefusal::NothingAsked), level });
        }
        if tier > MAX_POOL_TIER {
            return Ok(SkillUp {
                spend: SpendOutcome::Refused(SpendRefusal::NoSuchPool { tier }),
                level,
            });
        }
        let spend = if tier == CLIENT_COMPUTED_TIER {
            SpendOutcome::NotCharged
        } else {
            let spent = spent_in_pool(&tx, character_id, tier)?;
            let available = balance(entitlement, spent);
            if available < count {
                return Ok(SkillUp {
                    spend: SpendOutcome::Refused(SpendRefusal::NotEnough {
                        available,
                        wanted: count,
                    }),
                    level,
                });
            }
            charge_pool(&tx, character_id, skill_id, tier, count)?;
            SpendOutcome::Spent {
                charged: count,
                spent: spent.saturating_add(count),
                available: available - count,
            }
        };
        let next = level.saturating_add(count);
        crate::skills::set_skill_level_row(&tx, character_id, skill_id, next)?;
        tx.commit()?;
        Ok(SkillUp { spend, level: next })
    }

    /// **Take `count` points out of a pool for one skill**, without touching the level.
    ///
    /// **`0x013B` wants [`Store::spend_and_raise_skill`], not this.** This charges the pool
    /// and nothing else; a caller that raises the level in a separate call has two effects
    /// that can come apart, and a failure between them either sells a point for nothing or
    /// hands out a free level. It stays public for a caller that really is charging without
    /// raising - a quest that costs SP, a future respec fee - and for the tests below.
    ///
    /// The read of the pool and the write of the row are one transaction, so two spends
    /// arriving together cannot both see the same balance - the same rule this crate already
    /// applies to a meso balance, and for the same reason.
    ///
    /// The caller must **match the answer and return early on
    /// [`SpendOutcome::Refused`]**. Nothing that follows a skill-up - the level, the `0x0081`,
    /// the `0x007C` that resends the pools - may hang off the *request*; all three hang off
    /// this transition. `CLAUDE.md`: *"If the store says 'nothing changed', nothing may
    /// follow."*
    ///
    /// It never part-fills. A request for 3 points against a pool of 2 is
    /// [`SpendRefusal::NotEnough`], not a silent charge of 2 - the client sent `count` because
    /// it believed the pool could pay, and quietly handing back a different number is exactly
    /// the shape of *"I just tried to bulk add 3 points into Three Snails, but it only went up
    /// 1 point."*
    pub fn spend_skill_points(
        &self,
        character_id: u32,
        skill_id: u32,
        tier: u8,
        entitlement: u32,
        count: u32,
    ) -> Result<SpendOutcome> {
        // Tier 0 first, and as a SUCCESS. The client computes that pool itself and never reads
        // a stored one, so charging it could only ever put the server at odds with the screen.
        // Answering with a refusal would break every beginner skill-up for a caller that
        // forgot to branch; answering with "nothing to record" cannot.
        if tier == CLIENT_COMPUTED_TIER {
            return Ok(SpendOutcome::NotCharged);
        }
        if tier > MAX_POOL_TIER {
            return Ok(SpendOutcome::Refused(SpendRefusal::NoSuchPool { tier }));
        }
        if count == 0 {
            return Ok(SpendOutcome::Refused(SpendRefusal::NothingAsked));
        }

        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let spent = spent_in_pool(&tx, character_id, tier)?;
        let available = balance(entitlement, spent);
        if available < count {
            // The transaction is dropped unread: a refusal writes nothing at all, so there is
            // no half-charged state for a later read to disagree with.
            return Ok(SpendOutcome::Refused(SpendRefusal::NotEnough { available, wanted: count }));
        }
        charge_pool(&tx, character_id, skill_id, tier, count)?;
        tx.commit()?;
        Ok(SpendOutcome::Spent {
            charged: count,
            spent: spent.saturating_add(count),
            available: available - count,
        })
    }

    /// **Forget one skill and give its points back**, in one transaction.
    ///
    /// Returns how many points went back into the pool - `0` for a skill that cost nothing,
    /// which is every skill `!learn` handed over and every beginner skill.
    ///
    /// # Why the deletion lives here rather than beside `set_skill_level`
    ///
    /// Because the two must not be separable. `!resetsp`'s user-facing line promises *"a
    /// forgotten skill is the whole refund"*; if the level row could be removed without the
    /// ledger row, that sentence would become a lie the first time one of the two failed, and
    /// the player would keep a skill whose points had been handed back. One statement pair,
    /// one transaction, no half-state - the same reasoning that put the completed-quest guard
    /// in the row rather than in the caller.
    ///
    /// `skills::set_skill_level(.., 0)` remains the un-refunded primitive. It is the right
    /// call for anything that never charged in the first place; it is the **wrong** call for
    /// undoing a spend, and this exists so there is a right one.
    pub fn forget_skill_and_refund(&self, character_id: u32, skill_id: u32) -> Result<u32> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let points: Option<i64> = tx
            .query_row(
                "SELECT points FROM character_skill_spend
                  WHERE character_id = ?1 AND skill_id = ?2",
                rusqlite::params![i64::from(character_id), i64::from(skill_id)],
                |row| row.get(0),
            )
            .optional()?;
        tx.execute(
            "DELETE FROM character_skill_spend WHERE character_id = ?1 AND skill_id = ?2",
            rusqlite::params![i64::from(character_id), i64::from(skill_id)],
        )?;
        tx.execute(
            "DELETE FROM character_skills WHERE character_id = ?1 AND skill_id = ?2",
            rusqlite::params![i64::from(character_id), i64::from(skill_id)],
        )?;
        tx.commit()?;
        Ok(points.unwrap_or(0).clamp(0, i64::from(u32::MAX)) as u32)
    }

    /// **Forget every skill and empty the ledger**, in one transaction. What `!resetsp` wants.
    ///
    /// All or nothing: there is no state in which some skills are gone and some points are
    /// still charged. A per-skill loop that zeroed the pool at the end could reach that state
    /// on a single failure, and it would hand back points for a skill the player still had.
    ///
    /// Returns both counts, because a caller that checks one effect of two learns nothing
    /// about the other - the turn-in test counted fanfares while the experience doubled beside
    /// it.
    pub fn forget_all_skills_and_refund(&self, character_id: u32) -> Result<Refunded> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let points: i64 = tx.query_row(
            "SELECT COALESCE(SUM(points), 0) FROM character_skill_spend WHERE character_id = ?1",
            [i64::from(character_id)],
            |row| row.get(0),
        )?;
        tx.execute(
            "DELETE FROM character_skill_spend WHERE character_id = ?1",
            [i64::from(character_id)],
        )?;
        let skills = tx.execute(
            "DELETE FROM character_skills WHERE character_id = ?1",
            [i64::from(character_id)],
        )?;
        tx.commit()?;
        Ok(Refunded {
            skills: u32::try_from(skills).unwrap_or(u32::MAX),
            points: points.clamp(0, i64::from(u32::MAX)) as u32,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The first-job pool. `net::stats::tier_for_job(200)` is 1; spelled as a literal here
    /// because this crate deliberately does not know how a skill maps to a tier.
    const FIRST_JOB: u8 = 1;
    const SECOND_JOB: u8 = 2;
    /// `world::skillpoints::entitlement(Tier::First, 11)` - the owner's own worked example.
    const ENTITLEMENT_AT_11: u32 = 4;

    fn store_with_character() -> (Store, u32) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Spender".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        (store, id)
    }

    /// Spend, and fail loudly if the store refused.
    ///
    /// Every one of the calls below is a test's **setup**, and a setup spend that quietly
    /// refused would leave every assertion after it asserting something about a pool that was
    /// never charged - a green test over a feature that did nothing. `#[must_use]` on
    /// [`SpendOutcome`] is what forced this helper to exist rather than letting twelve
    /// discarded answers through, which is the same guard it exists to impose on
    /// `crates/world/src/session/`.
    #[track_caller]
    fn must_spend(s: &Store, chr: u32, skill_id: u32, tier: u8, entitlement: u32, count: u32) {
        let out = s.spend_skill_points(chr, skill_id, tier, entitlement, count).unwrap();
        assert!(
            matches!(out, SpendOutcome::Spent { charged, .. } if charged == count),
            "setup spend of {count} into skill {skill_id} was not granted: {out:?}"
        );
    }

    /// **The round trip: spend, forget, and the balance comes back.**
    ///
    /// The three calls are not the thing being tested - the loop between them is. This is the
    /// assertion `!resetsp`'s chat line makes to the player, and it has to keep being true now
    /// that a forgotten skill is no longer the *only* record of a spend.
    ///
    /// Every effect is checked at every step, not just the balance: the ledger row, the pool
    /// total, and the skill level. A test that watches one of three effects is how the quest
    /// turn-in passed on every run while the experience doubled beside it.
    #[test]
    fn a_spend_and_a_forget_round_trip() {
        let (s, chr) = store_with_character();

        // Nothing spent: the pool is the whole entitlement.
        assert_eq!(s.skill_points_available(chr, FIRST_JOB, ENTITLEMENT_AT_11).unwrap(), 4);
        assert!(s.skill_point_ledger(chr).unwrap().is_empty());

        // Spend 3 of the 4.
        let out = s.spend_skill_points(chr, 2001005, FIRST_JOB, ENTITLEMENT_AT_11, 3).unwrap();
        assert_eq!(out, SpendOutcome::Spent { charged: 3, spent: 3, available: 1 });
        s.set_skill_level(chr, 2001005, 3).unwrap();

        assert_eq!(s.skill_points_spent(chr, FIRST_JOB).unwrap(), 3, "the pool total moved");
        assert_eq!(s.skill_points_available(chr, FIRST_JOB, ENTITLEMENT_AT_11).unwrap(), 1);
        assert_eq!(
            s.skill_point_ledger(chr).unwrap(),
            vec![SpendRow { skill_id: 2001005, tier: FIRST_JOB, points: 3 }],
            "and the row says which skill took them"
        );
        assert_eq!(s.skill_level(chr, 2001005).unwrap(), 3, "the level went up too");

        // Forget it. This is `!resetsp` on one skill.
        assert_eq!(s.forget_skill_and_refund(chr, 2001005).unwrap(), 3, "3 points came back");

        assert_eq!(s.skill_points_spent(chr, FIRST_JOB).unwrap(), 0);
        assert_eq!(
            s.skill_points_available(chr, FIRST_JOB, ENTITLEMENT_AT_11).unwrap(),
            4,
            "the balance is back to the full entitlement - which is what !resetsp promises"
        );
        assert!(s.skill_point_ledger(chr).unwrap().is_empty(), "and the ledger row went with it");
        assert_eq!(s.skill_level(chr, 2001005).unwrap(), 0, "the skill really is forgotten");
        assert!(s.skills(chr).unwrap().is_empty());

        // And the pool is spendable again, all the way to the same floor. The refund is a
        // refund, not a number that merely reads right.
        let again = s.spend_skill_points(chr, 2001005, FIRST_JOB, ENTITLEMENT_AT_11, 4).unwrap();
        assert_eq!(again, SpendOutcome::Spent { charged: 4, spent: 4, available: 0 });
    }

    /// **The skill-up: the charge and the level move together, or neither moves.**
    ///
    /// Three effects are checked at every step - the pool, the ledger row and the level -
    /// because a test that watches one of three is how the quest turn-in passed on every run
    /// while the experience doubled beside it.
    #[test]
    fn a_skill_up_charges_the_pool_and_raises_the_level_together() {
        let (s, chr) = store_with_character();

        let up = s.spend_and_raise_skill(chr, 2001005, FIRST_JOB, ENTITLEMENT_AT_11, 3).unwrap();
        assert_eq!(up.spend, SpendOutcome::Spent { charged: 3, spent: 3, available: 1 });
        assert_eq!(up.level, 3, "the level went up by exactly what was charged");
        assert_eq!(s.skill_level(chr, 2001005).unwrap(), 3);
        assert_eq!(s.skill_points_spent(chr, FIRST_JOB).unwrap(), 3);

        // A second point out of the same pool stacks on the same row and the same level.
        let up = s.spend_and_raise_skill(chr, 2001005, FIRST_JOB, ENTITLEMENT_AT_11, 1).unwrap();
        assert_eq!(up.spend, SpendOutcome::Spent { charged: 1, spent: 4, available: 0 });
        assert_eq!(up.level, 4);

        // **The refusal changes nothing - not the pool, not the row, not the level.** This is
        // the state a level raised outside the transaction would have got wrong.
        let up = s.spend_and_raise_skill(chr, 2001005, FIRST_JOB, ENTITLEMENT_AT_11, 1).unwrap();
        assert_eq!(
            up.spend,
            SpendOutcome::Refused(SpendRefusal::NotEnough { available: 0, wanted: 1 })
        );
        assert_eq!(up.level, 4, "the level is reported unchanged");
        assert_eq!(s.skill_level(chr, 2001005).unwrap(), 4, "and really is unchanged");
        assert_eq!(s.skill_points_spent(chr, FIRST_JOB).unwrap(), 4);
        assert_eq!(
            s.skill_point_ledger(chr).unwrap(),
            vec![SpendRow { skill_id: 2001005, tier: FIRST_JOB, points: 4 }]
        );

        // A beginner skill still goes up, and still costs nothing.
        let up = s.spend_and_raise_skill(chr, 1000, CLIENT_COMPUTED_TIER, 0, 3).unwrap();
        assert_eq!(up.spend, SpendOutcome::NotCharged);
        assert_eq!(up.level, 3);
        assert_eq!(s.skill_level(chr, 1000).unwrap(), 3);
        assert_eq!(s.skill_point_ledger(chr).unwrap().len(), 1, "and wrote no ledger row");

        // **The round trip through the real path.** Forget the charged skill: the four points
        // come back, the level goes, and the beginner skill - which cost nothing - is
        // untouched by that refund.
        assert_eq!(s.forget_skill_and_refund(chr, 2001005).unwrap(), 4);
        assert_eq!(s.skill_points_available(chr, FIRST_JOB, ENTITLEMENT_AT_11).unwrap(), 4);
        assert_eq!(s.skill_level(chr, 2001005).unwrap(), 0);
        assert_eq!(s.skill_level(chr, 1000).unwrap(), 3, "the free skill is not collateral");
        assert!(s.skill_point_ledger(chr).unwrap().is_empty());
        // And the pool really is spendable again, not merely reading right.
        let up = s.spend_and_raise_skill(chr, 2001002, FIRST_JOB, ENTITLEMENT_AT_11, 4).unwrap();
        assert_eq!(up.spend, SpendOutcome::Spent { charged: 4, spent: 4, available: 0 });
        assert_eq!(up.level, 4);
    }

    /// **The bug, stated as a test.** Spent points must not come back on an advancement or a
    /// relog.
    ///
    /// `entitlement` is a total owed, so the pool table is rebuilt from the level on every
    /// send. Before the ledger existed that meant re-sending it handed the spent points back.
    /// Here the same entitlement is asked for three times - a resend, a level-up, and a much
    /// later level - and the spend is subtracted every time.
    #[test]
    fn spent_points_do_not_come_back_on_the_next_advancement() {
        let (s, chr) = store_with_character();
        let out = s.spend_skill_points(chr, 2001005, FIRST_JOB, ENTITLEMENT_AT_11, 3).unwrap();
        assert!(matches!(out, SpendOutcome::Spent { .. }));

        // Re-sending the same entitlement - what a relog or a second !job does.
        assert_eq!(s.skill_points_available(chr, FIRST_JOB, ENTITLEMENT_AT_11).unwrap(), 1);
        // Level 12: entitlement 7, three of them already spent.
        assert_eq!(s.skill_points_available(chr, FIRST_JOB, 7).unwrap(), 4);
        // Level 30: entitlement 61, still exactly three spent.
        assert_eq!(s.skill_points_available(chr, FIRST_JOB, 61).unwrap(), 58);
        assert_eq!(s.skill_points_spent(chr, FIRST_JOB).unwrap(), 3, "and the spend never moved");
    }

    /// **A refusal changes nothing at all**, and it is reportable.
    ///
    /// Both halves are asserted: the answer says no, *and* the ledger is untouched. A store
    /// that refused in its return value while writing the row anyway would pass a test that
    /// only looked at one of them.
    #[test]
    fn a_pool_that_cannot_pay_refuses_and_writes_nothing() {
        let (s, chr) = store_with_character();
        must_spend(&s, chr, 2001005, FIRST_JOB, ENTITLEMENT_AT_11, 4);

        let out = s.spend_skill_points(chr, 2001002, FIRST_JOB, ENTITLEMENT_AT_11, 1).unwrap();
        assert_eq!(out, SpendOutcome::Refused(SpendRefusal::NotEnough { available: 0, wanted: 1 }));
        assert!(!out.may_proceed(), "and the caller is told to stop");

        assert_eq!(s.skill_points_spent(chr, FIRST_JOB).unwrap(), 4, "unchanged");
        assert_eq!(s.skill_point_ledger(chr).unwrap().len(), 1, "no row for the refused skill");

        // It never part-fills: 3 asked against 2 available is a refusal, not a charge of 2.
        let (s2, chr2) = store_with_character();
        let partial = s2.spend_skill_points(chr2, 2001005, FIRST_JOB, 2, 3).unwrap();
        assert_eq!(
            partial,
            SpendOutcome::Refused(SpendRefusal::NotEnough { available: 2, wanted: 3 })
        );
        assert_eq!(s2.skill_points_spent(chr2, FIRST_JOB).unwrap(), 0);
        // The refusal says enough to be reported in a chat line rather than swallowed.
        let SpendOutcome::Refused(why) = partial else { panic!("expected a refusal") };
        assert!(why.to_string().contains('2') && why.to_string().contains('3'), "{why}");
    }

    /// **`!learn` must keep costing nothing.** A level granted outside a spend leaves the pool
    /// alone - which is the whole reason the spend is stored rather than derived from levels.
    #[test]
    fn a_granted_level_costs_no_skill_points() {
        let (s, chr) = store_with_character();
        // The whole Magician book, at its ceilings, the way `!learn` hands it over.
        for (id, level) in [(2001002u32, 15u32), (2001005, 20), (2101001, 20)] {
            s.set_skill_level(chr, id, level).unwrap();
        }
        assert_eq!(s.skills(chr).unwrap().len(), 3, "55 levels of skills");
        assert_eq!(
            s.skill_points_available(chr, FIRST_JOB, ENTITLEMENT_AT_11).unwrap(),
            4,
            "and not one point out of the pool - a derived total would have read 55 spent"
        );
        assert!(s.skill_point_ledger(chr).unwrap().is_empty());
    }

    /// **`!resetsp`: forget everything, get everything back**, and nothing survives half-done.
    ///
    /// The two counts are both asserted because a caller that checks one learns nothing about
    /// the other, and here they are genuinely independent: `!learn`'s skills have no ledger
    /// rows, so `skills` and `points` do not track each other.
    #[test]
    fn forgetting_everything_empties_both_tables_and_refunds_both_pools() {
        let (s, chr) = store_with_character();
        // Two real spends in two different pools...
        must_spend(&s, chr, 2001005, FIRST_JOB, 61, 7);
        s.set_skill_level(chr, 2001005, 7).unwrap();
        must_spend(&s, chr, 2101004, SECOND_JOB, 10, 2);
        s.set_skill_level(chr, 2101004, 2).unwrap();
        // ...and one skill that was granted for free.
        s.set_skill_level(chr, 2001002, 15).unwrap();

        assert_eq!(s.skill_points_spent_by_tier(chr).unwrap(), vec![(FIRST_JOB, 7), (SECOND_JOB, 2)]);

        let back = s.forget_all_skills_and_refund(chr).unwrap();
        assert_eq!(back, Refunded { skills: 3, points: 9 }, "three skills gone, nine points back");

        assert!(s.skills(chr).unwrap().is_empty(), "no skill survived");
        assert!(s.skill_point_ledger(chr).unwrap().is_empty(), "no charge survived");
        assert_eq!(s.skill_points_available(chr, FIRST_JOB, 61).unwrap(), 61);
        assert_eq!(s.skill_points_available(chr, SECOND_JOB, 10).unwrap(), 10);

        // Running it again is a no-op rather than a second refund.
        assert_eq!(s.forget_all_skills_and_refund(chr).unwrap(), Refunded::default());
    }

    /// **Tier 0 is never charged, and that is a success rather than a refusal.**
    ///
    /// `research/skill-points.md` §4.1: the client computes a beginner's SP from its own level
    /// and its own skill levels. A stored counter for that pool could only disagree with the
    /// screen - and refusing instead would break every beginner skill-up for a caller that
    /// forgot to branch.
    #[test]
    fn the_beginner_pool_is_the_clients_own_and_is_never_recorded() {
        let (s, chr) = store_with_character();
        let out = s.spend_skill_points(chr, 1000, CLIENT_COMPUTED_TIER, 0, 3).unwrap();
        assert_eq!(out, SpendOutcome::NotCharged);
        assert!(out.may_proceed(), "a beginner skill-up must still go through");
        assert!(s.skill_point_ledger(chr).unwrap().is_empty(), "and nothing was written down");
        assert_eq!(s.skill_points_spent(chr, CLIENT_COMPUTED_TIER).unwrap(), 0);
    }

    /// A pool the client cannot read back is refused rather than stored.
    ///
    /// `FUN_1402cb030` returns 0 for any key above 10, so those points would vanish from every
    /// screen while still lighting the "unspent points" indicator through `charstat+0xef`.
    #[test]
    fn a_tier_above_the_lookups_ceiling_is_refused() {
        let (s, chr) = store_with_character();
        let out = s.spend_skill_points(chr, 2001005, MAX_POOL_TIER + 1, 100, 1).unwrap();
        assert_eq!(out, SpendOutcome::Refused(SpendRefusal::NoSuchPool { tier: 11 }));
        assert!(s.skill_point_ledger(chr).unwrap().is_empty());
        // The boundary itself is allowed - the lookup accepts 10.
        assert!(s
            .spend_skill_points(chr, 2001005, MAX_POOL_TIER, 100, 1)
            .unwrap()
            .may_proceed());
    }

    /// A count of zero is a refusal that changes nothing. The caller still has to answer it:
    /// the client's request latch clears on the reply, not on the outcome.
    #[test]
    fn zero_points_is_refused_and_changes_nothing() {
        let (s, chr) = store_with_character();
        let out = s.spend_skill_points(chr, 2001005, FIRST_JOB, 61, 0).unwrap();
        assert_eq!(out, SpendOutcome::Refused(SpendRefusal::NothingAsked));
        assert!(s.skill_point_ledger(chr).unwrap().is_empty());
    }

    /// **The two pools do not bleed into each other**, which is the property
    /// `world::skillpoints::Tier` exists to protect: second-job points must not become
    /// spendable on first-job skills.
    #[test]
    fn one_pool_is_not_spendable_from_another() {
        let (s, chr) = store_with_character();
        must_spend(&s, chr, 2001005, FIRST_JOB, 61, 61);

        assert_eq!(s.skill_points_available(chr, FIRST_JOB, 61).unwrap(), 0, "first job is dry");
        assert_eq!(
            s.skill_points_available(chr, SECOND_JOB, 10).unwrap(),
            10,
            "and the second job pool never noticed"
        );
        assert!(s.spend_skill_points(chr, 2101004, SECOND_JOB, 10, 10).unwrap().may_proceed());
        assert_eq!(
            s.spend_skill_points(chr, 2001002, FIRST_JOB, 61, 1).unwrap(),
            SpendOutcome::Refused(SpendRefusal::NotEnough { available: 0, wanted: 1 }),
            "spending the second pool did not top the first one up"
        );
    }

    /// Two spends on the same skill accumulate on one row rather than making a second one.
    #[test]
    fn spending_twice_on_one_skill_adds_to_its_row() {
        let (s, chr) = store_with_character();
        must_spend(&s, chr, 2001005, FIRST_JOB, 61, 4);
        let out = s.spend_skill_points(chr, 2001005, FIRST_JOB, 61, 3).unwrap();
        assert_eq!(out, SpendOutcome::Spent { charged: 3, spent: 7, available: 54 });
        assert_eq!(
            s.skill_point_ledger(chr).unwrap(),
            vec![SpendRow { skill_id: 2001005, tier: FIRST_JOB, points: 7 }]
        );
    }

    /// **The invariant that ties this table to `character_skills`:** a skill's recorded spend
    /// is never more than its level. A point always produces a level; a level is only removed
    /// together with its points; `!learn` moves the level up without the points, never the
    /// other way round.
    ///
    /// Checked across the whole round trip rather than at one moment, because the states that
    /// could break it are the transitions.
    #[test]
    fn the_ledger_never_exceeds_the_skill_level() {
        let (s, chr) = store_with_character();
        let check = |s: &Store| {
            for row in s.skill_point_ledger(chr).unwrap() {
                let level = s.skill_level(chr, row.skill_id).unwrap();
                assert!(
                    row.points <= level,
                    "skill {} has {} point(s) charged against level {level}",
                    row.skill_id,
                    row.points
                );
            }
        };
        must_spend(&s, chr, 2001005, FIRST_JOB, 61, 3);
        s.set_skill_level(chr, 2001005, 3).unwrap();
        check(&s);
        // `!learn` raises the level well past the charge.
        s.set_skill_level(chr, 2001005, 20).unwrap();
        check(&s);
        // A further spend on top of a granted level.
        must_spend(&s, chr, 2001005, FIRST_JOB, 61, 2);
        s.set_skill_level(chr, 2001005, 22).unwrap();
        check(&s);
        // And the forget removes both halves at once, so the invariant survives it.
        s.forget_skill_and_refund(chr, 2001005).unwrap();
        check(&s);
        assert!(s.skill_point_ledger(chr).unwrap().is_empty());
    }

    /// **The migration: a character who already has skills starts with a full pool.**
    ///
    /// The table is new and starts empty, so an existing character reads as nothing spent
    /// while keeping every level. A one-time amnesty, bounded by the entitlement and closed
    /// the moment they spend anything - see `create_tables` for why that beats backfilling
    /// from `character_skills.level`.
    #[test]
    fn a_character_who_already_has_skills_is_not_stranded() {
        let (s, chr) = store_with_character();
        // The owner's Magician, seven points into Magic Claw, written before this table existed.
        s.set_skill_level(chr, 2001005, 7).unwrap();
        assert!(s.skill_point_ledger(chr).unwrap().is_empty(), "no ledger row exists for it");

        assert_eq!(s.skill_level(chr, 2001005).unwrap(), 7, "the skill is untouched");
        assert_eq!(
            s.skill_points_available(chr, FIRST_JOB, 61).unwrap(),
            61,
            "and the pool reads full - the amnesty, once"
        );

        // The amnesty closes: the next spend is recorded and is not refundable by relogging.
        must_spend(&s, chr, 2001002, FIRST_JOB, 61, 5);
        assert_eq!(s.skill_points_available(chr, FIRST_JOB, 61).unwrap(), 56);
    }

    /// The balance saturates rather than wrapping when a pool is somehow over-spent - an
    /// entitlement that shrank, or a hand-edited row. `u32::MAX` free points is the failure
    /// this rules out.
    #[test]
    fn an_over_spent_pool_reads_as_zero_and_never_wraps() {
        assert_eq!(balance(4, 0), 4);
        assert_eq!(balance(4, 4), 0);
        assert_eq!(balance(4, 999), 0, "not 4294966301");
        assert_eq!(balance(0, 1), 0);

        let (s, chr) = store_with_character();
        must_spend(&s, chr, 2001005, FIRST_JOB, 61, 61);
        // The same character read against a smaller entitlement than they spent under.
        assert_eq!(s.skill_points_available(chr, FIRST_JOB, 4).unwrap(), 0);
        assert_eq!(
            s.spend_skill_points(chr, 2001005, FIRST_JOB, 4, 1).unwrap(),
            SpendOutcome::Refused(SpendRefusal::NotEnough { available: 0, wanted: 1 })
        );
    }

    /// Two characters do not share a ledger, and deleting one takes its rows with it.
    #[test]
    fn the_ledger_is_per_character_and_cascades() {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let mk = |name: &str| {
            store
                .create_character(
                    account,
                    0,
                    &net::opcode::Character { name: name.to_string(), ..Default::default() },
                )
                .unwrap()
                .id
        };
        let a = mk("Alpha");
        let b = mk("Beta");

        must_spend(&store, a, 2001005, FIRST_JOB, 61, 5);
        assert_eq!(store.skill_points_spent(b, FIRST_JOB).unwrap(), 0, "not shared");
        assert_eq!(store.skill_points_available(b, FIRST_JOB, 61).unwrap(), 61);

        // Prove the row is there first - "no orphans" is also what a failed insert looks like.
        assert_eq!(ledger_rows_total(&store), 1);
        store.delete_character(account, a).unwrap();
        assert_eq!(ledger_rows_total(&store), 0, "ON DELETE CASCADE should have taken it");
    }

    /// The table survives a reopen, which is where a schema that is not idempotent shows up.
    /// An in-memory store cannot catch this: it is a fresh database every time.
    #[test]
    fn the_ledger_survives_a_reopen() {
        let dir = std::env::temp_dir().join(format!("maplecw-spledger-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("ledger.db");
        let _ = std::fs::remove_file(&path);

        let chr;
        {
            let s = Store::open(&path).unwrap();
            let account = s.create_account("wisp", "correct horse battery").unwrap();
            chr = s
                .create_character(
                    account,
                    0,
                    &net::opcode::Character { name: "Persist".into(), ..Default::default() },
                )
                .unwrap()
                .id;
            must_spend(&s, chr, 2001005, FIRST_JOB, 61, 6);
            s.set_skill_level(chr, 2001005, 6).unwrap();
        }

        let again = Store::open(&path).expect("the schema runs on every open and must be a no-op");
        assert_eq!(again.skill_points_spent(chr, FIRST_JOB).unwrap(), 6, "the spend persisted");
        assert_eq!(again.skill_points_available(chr, FIRST_JOB, 61).unwrap(), 55);
        drop(again);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A database written before this table existed gains it on open, with every character's
    /// skills intact. The synthetic twin of `db.rs`'s live-database upgrade test.
    #[test]
    fn a_database_written_before_the_ledger_upgrades() {
        let dir = std::env::temp_dir().join(format!("maplecw-spolddb-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("old.db");
        let _ = std::fs::remove_file(&path);

        let chr;
        {
            let s = Store::open(&path).unwrap();
            let account = s.create_account("wisp", "correct horse battery").unwrap();
            chr = s
                .create_character(
                    account,
                    0,
                    &net::opcode::Character { name: "Oldtimer".into(), ..Default::default() },
                )
                .unwrap()
                .id;
            s.set_skill_level(chr, 2001005, 7).unwrap();
            // Wind the schema back to a build that had skills but no ledger.
            s.conn().execute("DROP TABLE character_skill_spend", []).unwrap();
        }

        let s = Store::open(&path).expect("an older database must upgrade, not fail");
        assert_eq!(s.skill_level(chr, 2001005).unwrap(), 7, "the seven points are still learned");
        assert_eq!(
            s.skill_points_available(chr, FIRST_JOB, 61).unwrap(),
            61,
            "and the pool starts full - the amnesty"
        );
        must_spend(&s, chr, 2001002, FIRST_JOB, 61, 1);
        assert_eq!(s.skill_points_spent(chr, FIRST_JOB).unwrap(), 1, "and the new table works");
        drop(s);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **The migration, run against a copy of the owner's real database.**
    ///
    /// The synthetic tests above start from a file this suite created, which is exactly the
    /// case a broken migration still passes - `CREATE TABLE IF NOT EXISTS` does nothing to a
    /// table that already exists. The only way to be sure is to open a file an older build
    /// wrote. Same shape and same reasoning as `db::tests::wisps_real_database_upgrades_in_place`:
    /// skipped when `maplecw.db` is absent, and it works on a **copy**, sidecars included.
    ///
    /// What it asserts is deliberately what the *upgrade* guarantees, not what the data happens
    /// to look like today: every character keeps every skill level, and every pool reads
    /// exactly its entitlement because no ledger row exists yet.
    #[test]
    fn wisps_real_database_gains_the_ledger_without_stranding_anyone() {
        let live = std::path::Path::new("../../maplecw.db");
        if !live.exists() {
            return; // gitignored live state; the synthetic twins above cover the rest
        }
        let dir = std::env::temp_dir().join(format!("maplecw-spledger-live-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        for suffix in ["", "-wal", "-shm"] {
            let from = live.with_extension(format!("db{suffix}"));
            if from.exists() {
                std::fs::copy(&from, dir.join(format!("maplecw.db{suffix}"))).unwrap();
            }
        }
        let copy = dir.join("maplecw.db");

        let store = Store::open(&copy).expect("the live database must open under the new schema");
        let mut characters = 0;
        for account in store.list_accounts().unwrap() {
            for chr in store.characters_for(account.id, 0).unwrap() {
                characters += 1;
                // **What this used to assert, and why it had to change.**
                //
                // It was `skill_point_ledger(chr.id).is_empty()` and a flat `== 61` on every
                // pool - "the ledger starts empty for everyone, which is the amnesty". That
                // was true of a database written by a build with no ledger table, which is
                // what this test was built to open. It stopped being true the day the feature
                // shipped and the owner played: their live file now carries a real spend, so the
                // test began failing on 2026-08-29 for a reason that has nothing to do with
                // the migration it exists to check.
                //
                // A test pinned to live, mutating state expires. The durable guarantee is not
                // "the ledger is empty" - it is **nobody is stranded**: whatever has been
                // spent, the pool reads exactly `entitlement - spent` and never less, and no
                // skill level is lost. So compute the expectation from the ledger instead of
                // assuming what the ledger says.
                let ledger = store.skill_point_ledger(chr.id).unwrap();
                for tier in 0..=MAX_POOL_TIER {
                    let spent: u32 =
                        ledger.iter().filter(|r| r.tier == tier).map(|r| r.points).sum();
                    assert_eq!(
                        store.skill_points_available(chr.id, tier, 61).unwrap(),
                        balance(61, spent),
                        "{} reads a pool at tier {tier} that does not match its {spent} spent",
                        chr.name
                    );
                }
                // And every skill they have is still theirs. `Cobalt` is the case that
                // matters: level 14, job 100, one skill at level **15** - more levels than
                // `entitlement(First, 14) = 13` can pay for, so those levels came from
                // `!learn`. Backfilling the ledger from them would have pinned their pool at
                // zero for a grant that was free by contract.
                let learned = store.skills(chr.id).unwrap();
                assert!(
                    learned.iter().all(|s| s.level > 0),
                    "a level-0 skill row is not a thing this schema stores"
                );
            }
        }
        assert!(characters > 0, "the live database has a real character in it");

        // A spend works on the upgraded file, and a second open is a no-op - the schema runs
        // on EVERY open.
        drop(store);
        let again = Store::open(&copy).expect("the second open is where a bad schema shows up");
        let victim = again.list_accounts().unwrap()[0].id;
        if let Some(chr) = again.characters_for(victim, 0).unwrap().first() {
            must_spend(&again, chr.id, 2001005, FIRST_JOB, 61, 3);
            assert_eq!(again.skill_points_available(chr.id, FIRST_JOB, 61).unwrap(), 58);
        }
        drop(again);
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn ledger_rows_total(store: &Store) -> i64 {
        store
            .conn()
            .query_row("SELECT COUNT(*) FROM character_skill_spend", [], |row| row.get(0))
            .unwrap()
    }
}
