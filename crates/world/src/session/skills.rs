//! Raising a skill, and telling the client it went up.
//!
//! # One click is all you get, until we answer
//!
//! `FUN_142d4bd80` sets `user+0x2330 = 1` the moment the client sends `0x013B`, and
//! `FUN_142cc42d0` refuses to send **anything from that family** while it is set. Only a
//! server packet clears it. That gate has **264 call sites in 192 functions**, so an
//! unanswered skill-up silently disables a large part of the client's request surface - a
//! stronger version of the always-answer rule this project already lives by, and the same
//! latch the inventory move has.
//!
//! The owner clicked `+` on Three Snails and nothing happened; the request went out once, was
//! logged `UNKNOWN`, and was never answered. The second click never left the client.
//!
//! # The reply must come after the character record
//!
//! `0x0081`'s handler is a **silent no-op while `charData` is null**, and `STATUS.md`
//! records that field measuring `0` on the very first `SetField`. So this is a reply to a
//! request, which by definition arrives long after - but do not move it earlier.

use super::*;

impl Session {
    /// `0x013B` - the player clicked `+` on a skill.
    ///
    /// **Always answered**, including every refusal: the reply is what clears the request
    /// latch, and a silent refusal costs far more than the skill point.
    pub(super) fn on_skill_up(&mut self, payload: &[u8]) -> Vec<Reply> {
        let Ok(req) = net::skills::SkillUpRequest::parse(payload) else {
            // Even an unparseable body gets the latch cleared. We know the opcode; a body we
            // cannot read is our problem, not a reason to wedge the client.
            return vec![self.skill_reply(
                net::skills::skill_up_refused(net::skills::SkillUpRefusal::BadCount),
                "unparseable 0x013B - clearing the latch anyway".to_string(),
            )];
        };
        let Some(chr) = self.claimed_character() else {
            return vec![self.skill_reply(
                net::skills::skill_up_refused(net::skills::SkillUpRefusal::NotYours),
                "no character claimed".to_string(),
            )];
        };

        // **That data has now been read.** This used to refuse everything outside the three
        // beginner ids, saying so was "a refusal to invent a rule, since what a job may learn
        // is `Skill.wz` data nobody has read". `tools/dump_skills.py` reads it -
        // `gm-handbook/skills.txt`, 176 skills - and `world::skilltable` loads the two columns
        // a grant needs. The refusal is now the client's own table talking.
        //
        // **The old behaviour survives an empty table**, deliberately: with no file loaded
        // `may_learn` is false for everything, so the fallback below keeps the three beginner
        // skills working exactly as they did before. A missing generated file degrades to
        // something known rather than to something untested.
        let known = !self.config.skills.is_empty();
        let allowed = if known {
            self.config.skills.may_learn(chr.job, req.skill_id)
        } else {
            net::skills::BEGINNER_SKILLS.contains(&req.skill_id)
        };
        if !allowed {
            return vec![self.skill_reply(
                net::skills::skill_up_refused(net::skills::SkillUpRefusal::NotYours),
                match self.config.skills.get(req.skill_id) {
                    Some(sk) => format!(
                        "skill {} ({}) belongs to job book {}, and this character is job {}",
                        req.skill_id, sk.name, sk.job, chr.job
                    ),
                    None => format!(
                        "skill {} is not in this client's Skill.wz{}",
                        req.skill_id,
                        if known { "" } else { " - and no skill table is loaded, so only the three beginner skills are grantable. Regenerate with: python tools/dump_skills.py" }
                    ),
                },
            )];
        }

        // **`count` is a field on the request and it used to be thrown away.** The owner,
        // 2026-08-21: *"I just tried to bulk add 3 points into Three Snails, but it only went
        // up 1 point."* The capture is unambiguous - `<- 0x013B 940e8711 e8030000 03000000`
        // is tick, skill 1000, **count 3** - and this handler did `level + 1`.
        //
        // `SkillUpRequest::count`'s own doc block already said what it was and what to do
        // with it: *"`1` for `BtSpUp`, `min(sp, maxLevel - level)` for `BtSpUpAll` ... the
        // server must clamp again: nothing here authenticates, and nothing stops a crafted
        // body carrying any number at all."* Both halves were ignored, which is the same
        // shape as the quest payouts: the information was in front of the caller.
        //
        // **Clamped to the level table's own length**, not to the count. A body claiming
        // 4 000 000 000 must not overflow a level into something the client cannot draw, and
        // the client's own `Skill.wz` says these three stop at 3.
        // **The ceiling is the SKILL's own, not one constant.** `BEGINNER_SKILL_MAX_LEVEL`
        // is 3, which is right for the three beginner skills and wrong for every other skill
        // in the game - the Magician book runs to 15 and 20. Clamping a Magic Claw to 3 would
        // look exactly like the client refusing the click.
        let ceiling = self
            .config
            .skills
            .max_level(req.skill_id)
            .unwrap_or(net::skills::BEGINNER_SKILL_MAX_LEVEL);
        let level = self.store.skill_level(chr.id, req.skill_id).unwrap_or(0);
        if level >= ceiling {
            return vec![self.skill_reply(
                net::skills::skill_up_refused(net::skills::SkillUpRefusal::AtMaxLevel),
                format!(
                    "skill {} is already at {level}, the maximum ({ceiling}) this client's Skill.wz describes",
                    req.skill_id
                ),
            )];
        }
        // A zero count is the client asking for nothing. Answer it - the latch still has to
        // clear - and change nothing.
        if req.count == 0 {
            return vec![self.skill_reply(
                net::skills::skill_up_refused(net::skills::SkillUpRefusal::BadCount),
                format!("skill {} asked for 0 points; nothing to do", req.skill_id),
            )];
        }
        let room = ceiling - level;
        let granted = req.count.min(room);
        let next = level + granted;
        // **Say when a point will buy almost nothing.** The owner put seven points into Magic
        // Claw and it dealt 1, because they were a Rogue with 6 INT wearing a Magician's job id.
        // The skill was working; the character had no INT. Nothing on screen said so, and
        // "the skill only deals 1 damage" is what that silence produces.
        //
        // The warning rides on the acknowledgement rather than refusing the point: the stat
        // can be raised afterwards, so a refusal would be wrong as well as annoying.
        let scaling_warning = self
            .config
            .skills
            .level(req.skill_id, next)
            .filter(|l| l.mad.is_some())
            .and_then(|_| {
                let stat = crate::jobs::FIRST_JOBS.iter().find(|j| j.job == chr.job)?.stat;
                let have = stat.of(&chr);
                (have < crate::jobs::STAT_MINIMUM).then(|| {
                    format!(
                        " *** WARNING: this is an attack skill and you have only {have} {}.                          Its damage scales on that stat, so it will land on the floor of 1                          however many points go in. Raise {} first. ***",
                        stat.label(),
                        stat.label()
                    )
                })
            })
            .unwrap_or_default();

        // **The point is charged and the level raised in ONE transaction.**
        //
        // Until 2026-08-28 this called `set_skill_level` and nothing tracked the pool, so a
        // player could spend three points into Power Strike, relog, and have the three points
        // back *and* keep the skill. `gm_reset_sp`'s own chat line said so out loud - *"the
        // points come back on their own"* - which is how a farming loop gets documented
        // instead of fixed.
        //
        // `spend_and_raise_skill` raises the level by exactly the points it charged, inside
        // one transaction, so the two cannot come apart. `CLAUDE.md` names that failure twice:
        // the Heena payout that hung off the request rather than the transition, and the
        // forfeit whose `DELETE` did not carry the guard its own doc block promised.
        let tier = self.pool_tier(req.skill_id);
        let entitlement = self.pool_entitlement(tier, chr.level, chr.job);
        let up = match self.store.spend_and_raise_skill(
            chr.id,
            req.skill_id,
            tier,
            entitlement,
            granted,
        ) {
            Ok(up) => up,
            Err(e) => {
                return vec![self.skill_reply(
                    net::skills::skill_up_refused(net::skills::SkillUpRefusal::NotYours),
                    format!("could not save the skill: {e}"),
                )];
            }
        };
        // **Return early on the refusal.** Everything below is an effect of the transition,
        // and the one way to be sure none of them runs is to leave before any of them can.
        if let store::SpendOutcome::Refused(why) = up.spend {
            return vec![self.skill_reply(
                net::skills::skill_up_refused(net::skills::SkillUpRefusal::BadCount),
                format!("skill {} not raised: {why}", req.skill_id),
            )];
        }
        let next = up.level;

        let change = net::skills::SkillChange::Learn(net::skills::Skill::at_level(
            req.skill_id,
            next,
        ));
        // `show_effect` is FALSE. The owner, 2026-09-06, with four of them stacked in their chat:
        // *"Please also remove the 'skill has been activated' message, this behavior is not
        // present in MapleStory."* Byte 1 of `0x0081` is what asks the client for that line
        // ("<name> A skill has been activated.", strings 0xf6e/0xf6f) and for the level-up
        // flourish; a point spent from the skill window gets neither in the real game.
        //
        // A passive raised here - Max HP Increase, say - needs nothing else from this
        // handler: the server never carried the percent in a field, it reads the skill's
        // level wherever it caps HP or MP (`Session::pools`), so the new ceiling is in force
        // on the very next regen tick and the next potion.
        let mut out = vec![self.skill_reply(
            net::skills::change_skill_record_result(true, false, &[change]),
            format!(
                "skill {} raised {level} -> {next} (asked for {}, granted {granted}{}){scaling_warning}",
                req.skill_id,
                req.count,
                if granted < req.count { ", clamped by the level table" } else { "" }
            ),
        )];
        // **The pool on screen, or the database is right and the screen is wrong.** The
        // client never decrements a pool itself.
        out.extend(self.skill_point_reply(&chr));
        // **Only when there is something to say.** A chat line on every point would be noise,
        // and noise is how a warning stops being read.
        if !scaling_warning.is_empty() {
            out.extend(self.notice(scaling_warning.trim().trim_matches('*').trim().to_string()));
        }
        out
    }

    /// **Which pool a skill spends from.** The client's own key, not a job id.
    ///
    /// `FUN_1402CB030` returns 0 for any key above 10, so a job id would read an empty pool
    /// and grey the `+` button with nothing on screen to say why. `net::stats::tier_for_job`
    /// is the client's own arithmetic; the job is `skillId / 10000`.
    pub(super) fn pool_tier(&self, skill_id: u32) -> u8 {
        net::stats::tier_for_job(u16::try_from(skill_id / 10_000).unwrap_or(0))
    }

    /// **How many points that pool has ever been owed**, from the character's level.
    ///
    /// `world::skillpoints::entitlement` is a **total owed** rather than an increment, which
    /// is what makes re-sending it idempotent and advancing late pay the same as advancing
    /// early. The ledger subtracts what has been spent; this is the other half.
    ///
    /// Tier 0 is the beginner pool and returns `0` deliberately - the client computes that one
    /// itself, and the store treats a tier-0 spend as a success that writes no row.
    ///
    /// **Tier 1 keeps growing past 30** until the character's first-job book can be maxed -
    /// `skillpoints::first_job_entitlement`, a deliberate deviation the owner asked for on
    /// 2026-10-02.
    pub(super) fn pool_entitlement(&self, tier: u8, level: u32, job: u16) -> u32 {
        match tier {
            1 => crate::skillpoints::first_job_entitlement(level, self.first_job_book_points(job)),
            2 => crate::skillpoints::entitlement(crate::skillpoints::Tier::Second, level),
            3 => crate::skillpoints::entitlement(crate::skillpoints::Tier::Third, level),
            // Fourth job onward is not modelled - there is no fourth-job book in this client
            // at all, so the pool would be points for skills that do not exist. Saying `0`
            // rather than guessing is the point.
            _ => 0,
        }
    }

    /// **What maxing every skill in this character's first-job book costs**: the sum of their
    /// `maxLevel`s - 105 for Warrior, Magician and Bowman, 110 for Thief. `0` for a beginner and
    /// when the skill table did not load, which leaves the classic 61 in force.
    pub(super) fn first_job_book_points(&self, job: u16) -> u32 {
        if net::stats::tier_for_job(job) < 1 {
            return 0;
        }
        let first = job / 100 * 100;
        self.config.skills.book(first).iter().filter(|s| s.job == first).map(|s| s.max_level).sum()
    }

    /// **Every pool, as one `0x007C`.** Sent after anything that moves a balance.
    ///
    /// `research/skill-points.md` §6.2, **[L]**: the client never decrements a pool itself, and
    /// the extended arm **clears the whole list before reading it**. So this has to carry every
    /// pool, not the one that changed - a packet with only the spent pool would blank the
    /// others. Without it the database is right and the screen is wrong, which is the failure
    /// this server has shipped twice.
    pub(super) fn skill_point_reply(&self, chr: &net::opcode::Character) -> Vec<Reply> {
        let pools = self.sp_pools(chr.id, chr.level, chr.job);
        if pools.is_empty() {
            return Vec::new();
        }
        let owed: Vec<String> =
            pools.iter().map(|p| format!("tier {} = {}", p.job_level, p.amount)).collect();
        let table = net::stats::Sp::Extended(pools);
        if !table.matches_job(chr.job) {
            // A job outside the explorer tree takes the plain `u16` encoding, and sending the
            // extended shape would desynchronise the rest of the packet rather than merely
            // lose the points.
            return Vec::new();
        }
        vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange { sp: Some(table), ..Default::default() }.build(),
            what: format!(
                "StatChanged: skill points now [{}]. Mask bit 15, EVERY pool - the extended \
                 arm clears the whole list before reading, so a packet carrying only the pool \
                 that changed would blank the others",
                owed.join(", ")
            ),
        }]
    }

    /// **What is LEFT in every pool this character holds** - owed minus spent, the one number
    /// the skill window may show. Both `0x007C`s that carry the table build it here: this
    /// file's refresh and `gm::job_change_reply`, the advancement's packet.
    ///
    /// The owner, 2026-10-04, after a second advancement: *"my 1st job shows me a bunch of skill
    /// points I cannot use"* - 64 on the first-job tab of a level-31 Magician who had spent 61.
    /// `job_change_reply` built its own table from the ENTITLEMENT, never subtracting what the
    /// ledger says was spent, so the advancement handed the client the lifetime total; the
    /// server, which spends against the balance (3), refused every one past the third. The next
    /// SetField's refresh would have put it right, which is why it looked like a job-change bug.
    ///
    /// Only pools the character has actually ADVANCED into. `pool_entitlement` returns a level's
    /// worth for any tier - `entitlement(First, 12)` is 7 whether or not the character is a first
    /// job yet - so without this gate a beginner refreshed after a SetField would be handed 7
    /// first-job points they never earned. `tier_for_job` is the job's own pool (0 beginner,
    /// 1 first job, 2 second, 3 third), and a character holds every pool up to it. **Tier 3 is
    /// in the walk**: the refresh used to stop at 2, so a third job's pool would have been
    /// blanked by every refresh (the extended arm clears the whole list before reading it).
    pub(super) fn sp_pools(&self, character_id: u32, level: u32, job: u16) -> Vec<net::stats::SpPool> {
        let spent = self.store.skill_points_spent_by_tier(character_id).unwrap_or_default();
        let reached = net::stats::tier_for_job(job);
        let mut pools = Vec::new();
        for tier in [1u8, 2, 3] {
            if tier > reached {
                continue;
            }
            let owed = self.pool_entitlement(tier, level, job);
            let used = spent.iter().find(|(t, _)| *t == tier).map(|(_, n)| *n).unwrap_or(0);
            if owed > 0 {
                pools.push(net::stats::SpPool { job_level: tier, amount: store::balance(owed, used) });
            }
        }
        pools
    }

    /// Every reply on this path is the same opcode, and every one of them clears the latch.
    pub(super) fn skill_reply(&self, body: Vec<u8>, why: String) -> Reply {
        Reply {
            opcode: net::skills::CHANGE_SKILL_RECORD_RESULT,
            body,
            what: format!(
                "ChangeSkillRecordResult: {why}. The first byte clears user+0x2330 - without \
                 it the client will not send another request from that family at all."
            ),
        }
    }
}
