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

        // Only the beginner skills exist for a job-0 character, and this server has no job
        // advancement yet. Refusing anything else is not a rule of the game - it is a refusal
        // to invent one, since what a job may learn is `Skill.wz` data nobody has read.
        if !net::skills::BEGINNER_SKILLS.contains(&req.skill_id) {
            return vec![self.skill_reply(
                net::skills::skill_up_refused(net::skills::SkillUpRefusal::NotYours),
                format!("skill {} is not one this server knows how to grant", req.skill_id),
            )];
        }

        let level = self.store.skill_level(chr.id, req.skill_id).unwrap_or(0);
        let next = level.saturating_add(1);
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
            format!("skill {} raised {level} -> {next}", req.skill_id),
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
