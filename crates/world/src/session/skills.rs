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
        if let Err(e) = self.store.set_skill_level(chr.id, req.skill_id, next) {
            return vec![self.skill_reply(
                net::skills::skill_up_refused(net::skills::SkillUpRefusal::NotYours),
                format!("could not save the skill: {e}"),
            )];
        }

        let change = net::skills::SkillChange::Learn(net::skills::Skill::at_level(
            req.skill_id,
            next,
        ));
        vec![self.skill_reply(
            net::skills::change_skill_record_result(true, true, &[change]),
            format!(
                "skill {} raised {level} -> {next} (asked for {}, granted {granted}{})",
                req.skill_id,
                req.count,
                if granted < req.count { ", clamped by the level table" } else { "" }
            ),
        )]
    }

    /// Every reply on this path is the same opcode, and every one of them clears the latch.
    fn skill_reply(&self, body: Vec<u8>, why: String) -> Reply {
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
