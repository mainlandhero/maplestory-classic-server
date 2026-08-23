//! Casting a buff, holding it, and letting it go.
//!
//! The owner, 2026-08-21 and again 2026-08-22: *"Nimble Feet still does not give me a buff despite
//! me activating the skill."*
//!
//! # It was arriving the whole time
//!
//! `0x013C` has been in `world.log` since the day skills went in - one on 2026-08-22 at
//! 13:00:49, 51 bytes, `skillId 1002 level 3` - logged as `UNKNOWN` and dropped. This is the
//! third instance this week of a subsystem that was decoded, written up and never connected,
//! which is why `CLAUDE.md` has a section called "Built is not wired".
//!
//! # `0x013C` does not latch, and that is measured rather than assumed
//!
//! Every other request this session answers - the pick-up, the inventory move, the storage
//! window, the AP dialog - sets a one-outstanding flag that exactly one inbound opcode
//! clears, and a refusal that sends the wrong packet kills the feature for the session. This
//! one does not: the 13:00:49 cast went **entirely unanswered** and the client played on for
//! **four more minutes**, walking, fighting and changing maps. So a refusal here can be a
//! chat line, and it is.
//!
//! # The client never removes a temporary stat by itself
//!
//! `0x007D`'s duration reaches the client's own `tExpire`, and it was reasonable to think
//! that meant the client would drop the stat on schedule. It does not: it **flashes the icon
//! and waits**. The owner, 2026-08-22: *"after the expiry, the buff did not go away. (It just kept
//! flashing, but the temporary stats were still there)"* - and at that moment the client sent
//! nothing at all.
//!
//! Right-clicking the icon is how it asks: `0x013F`, retried every ~180 ms until something
//! answers. So **removal is the server's job on both paths**, and `0x007E` is mandatory.
//!
//! # Every effect hangs off "the cast was allowed"
//!
//! MP, the stat change, the grant and the cooldown stamp are all reached through one `Ok`,
//! for the reason `CLAUDE.md` records under the Heena quest: separately-gated effects are how
//! one gets missed. A refused cast costs nothing and stamps nothing.

use super::*;

/// A temporary stat this character is holding, and when it runs out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ActiveBuff {
    pub(super) bit: u32,
    pub(super) skill_id: u32,
    pub(super) expires_ms: u64,
}

impl Session {
    /// `0x013C` - the player pressed a skill.
    pub(super) fn on_skill_use(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        let Some(req) = net::buff::parse_skill_use(body) else {
            return self.notice(format!("Unreadable skill-use body: {body:02x?}"));
        };
        let (skill_id, asked) = (req.skill_id, req.level);

        // **A skill this server grants nothing for still gets an answer on screen.** Three
        // Snails is an attack and Recovery's CTS bit is not identified, so neither has a
        // packet to send - and "nothing happened" with no explanation is the symptom the owner
        // reported twice for Nimble Feet. Saying so costs one chat line on a rare cast.
        let Some(level) = net::buff::buff_level(skill_id, asked) else {
            return self.notice(format!(
                "This server does not grant skill {skill_id}'s effect yet. Three Snails is an \
                 attack, and Recovery's stat bit has never been identified."
            ));
        };

        // **The client's claim about its own level is checked, not trusted.** It sends the
        // level it thinks it has, and nothing on this socket authenticates anybody.
        let has = self
            .store
            .skills(chr.id)
            .unwrap_or_default()
            .into_iter()
            .find(|s| s.id == skill_id)
            .map(|s| s.level)
            .unwrap_or(0);
        if has < asked {
            return self.notice(format!(
                "You asked to cast skill {skill_id} at level {asked} and you have it at {has}."
            ));
        }

        let now = self.clock_ms;
        if let Some(ready) = self.skill_ready_ms.get(&skill_id).copied() {
            if now < ready {
                let left = (ready - now).div_ceil(1000);
                // Loud on purpose. The cooldown is 180 s out of `Skill.wz`, and a silent
                // refusal here would be indistinguishable from the bug being fixed - which
                // is exactly what this run is trying to tell apart. `!buff` skips it.
                return self.notice(format!(
                    "Skill {skill_id} is on cooldown for another {left}s (cooltime is {}s in \
                     Skill.wz). !buff casts it anyway.",
                    level.cooldown_seconds
                ));
            }
        }
        if chr.mp < u32::from(level.mp_cost) {
            return self.notice(format!(
                "Not enough MP: skill {skill_id} level {asked} costs {} and you have {}.",
                level.mp_cost, chr.mp
            ));
        }

        // ---------------------------------------------------------------- the transition
        chr.mp = chr.mp.saturating_sub(u32::from(level.mp_cost));
        if let Err(e) = self.store.save_character_progress(&chr) {
            return self.notice(format!("Could not spend the MP, so nothing was cast: {e}"));
        }
        self.skill_ready_ms
            .insert(skill_id, now.saturating_add(u64::from(level.cooldown_seconds) * 1000));

        let mut out = vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange { mp: Some(chr.mp), ..Default::default() }.build(),
            what: format!(
                "StatChanged: skill {skill_id} level {asked} cost {} mp -> {}/{}",
                level.mp_cost, chr.mp, chr.max_mp
            ),
        }];
        out.extend(self.grant_buff(skill_id, level, now));
        out
    }

    /// Put the stat on, replacing whatever held that bit before.
    ///
    /// Shared by `0x013C` and the `!buff` GM command, so the command tests the packet the
    /// skill sends rather than a second builder that could drift from it.
    pub(super) fn grant_buff(
        &mut self,
        skill_id: u32,
        level: net::buff::BuffLevel,
        now_ms: u64,
    ) -> Vec<Reply> {
        self.grant_buff_with_tail(skill_id, level, now_ms, net::buff::TAIL_LEN)
    }

    /// [`Self::grant_buff`] with the tail length chosen by the caller - `!buff`'s third
    /// argument. See `net::buff::TAIL_LEN` for why that number is slack and not a length.
    pub(super) fn grant_buff_with_tail(
        &mut self,
        skill_id: u32,
        level: net::buff::BuffLevel,
        now_ms: u64,
        tail: usize,
    ) -> Vec<Reply> {
        let stat = level.granted_by(skill_id);
        // One holder per bit. Two entries for the same stat would leave the second expiry
        // clearing a buff the first had already replaced - and the client tracks one value
        // per bit, so our table has to as well.
        self.buffs.retain(|b| b.bit != stat.bit);
        self.buffs.push(ActiveBuff {
            bit: stat.bit,
            skill_id,
            expires_ms: now_ms.saturating_add(u64::from(stat.duration_ms)),
        });
        let body = net::buff::temporary_stat_set_with_tail(&[stat], tail);
        vec![Reply {
            opcode: net::buff::TEMPORARY_STAT_SET,
            body,
            what: format!(
                "TemporaryStatSet: skill {skill_id} grants CTS bit {} = {} for {} ms ({} s from Skill.wz), \
                 {tail}-byte tail. The 18-byte tail threw an unhandled C++ exception at 0x142d5690c on \
                 2026-08-22 - the u32 reader's own `cmp edi,4 / jb` underflow path - so this length is \
                 SLACK around an unknown, not a computed size. net::buff::TAIL_LEN has the working",
                stat.bit, stat.value, stat.duration_ms, level.seconds
            ),
        }]
    }

    /// Expire buffs whose time is up, and **tell the client**, because it will not do it.
    ///
    /// # The retraction, and the observation that forced it
    ///
    /// This function sent `0x007E` on expiry, then stopped, and now sends it again. The
    /// middle step was wrong and one run said so.
    ///
    /// The reasoning for stopping was that the client holds its own `tExpire` - true, and
    /// `research/buffs.md` §5.3 reads the decoder storing it - so it would drop the stat by
    /// itself and the packet was pure risk on the one path every buff takes. The owner,
    /// 2026-08-22: *"after the expiry, the buff did not go away. (It just kept flashing, but
    /// the temporary stats were still there)"*
    ///
    /// **`tExpire` drives the flashing and nothing else.** The client sent nothing at the
    /// thirty-second mark - not a request, not a report - and kept the stat. Removal is the
    /// server's job, and `0x007E` is mandatory.
    ///
    /// That is what the plan's outcome table called "the client does not self-expire after
    /// all - harmless, and very informative", which is the only reason the wrong version was
    /// worth shipping for one run: it was written down as a claim that could come back false,
    /// and it did.
    pub(super) fn buff_tick(&mut self, now_ms: u64) -> Vec<Reply> {
        let done: Vec<ActiveBuff> =
            self.buffs.iter().copied().filter(|b| now_ms >= b.expires_ms).collect();
        if done.is_empty() {
            return Vec::new();
        }
        self.buffs.retain(|b| now_ms < b.expires_ms);
        let bits: Vec<u32> = done.iter().map(|b| b.bit).collect();
        let skills: Vec<u32> = done.iter().map(|b| b.skill_id).collect();
        self.reset_reply(&bits, net::buff::TAIL_LEN, format!("expired (from skill(s) {skills:?})"))
    }

    /// `0x013F` - the player right-clicked a buff icon.
    ///
    /// # It retries until something answers
    ///
    /// Fourteen of these arrived in three seconds, one every ~180 ms, all identical and all
    /// dropped. That cadence is a retry loop rather than fourteen clicks, and it is the
    /// clearest statement the client has made that it is waiting on us.
    ///
    /// The body names both the skill and the CTS bits - `u32 skillId`, five bytes, then the
    /// same 124-byte mask `0x007D` uses. **The mask is preferred over the skill id** because
    /// it is what the client is actually pointing at; the skill id is used only to explain
    /// the refusal when nothing matches, and a mismatch between the two is worth a log line
    /// rather than a guess.
    pub(super) fn on_skill_cancel(&mut self, body: &[u8]) -> Vec<Reply> {
        if self.claimed_character().is_none() {
            return Vec::new();
        }
        let Some(req) = net::buff::parse_skill_cancel(body) else {
            return self.notice(format!(
                "Unreadable buff-cancel body, {} bytes - expected {}.",
                body.len(),
                net::buff::CLIENT_SKILL_CANCEL_LEN
            ));
        };
        // Only bits we believe are held. Answering for a bit we never granted would tell the
        // client to clear something it may hold from elsewhere.
        let held: Vec<u32> =
            req.bits.iter().copied().filter(|b| self.buffs.iter().any(|h| h.bit == *b)).collect();
        if held.is_empty() {
            // **Still worth a line, because this is the retry case.** Fourteen unanswered
            // requests is what the run before this looked like, and silence here would be
            // indistinguishable from the handler not existing.
            return self.notice(format!(
                "Nothing to cancel for skill {}: the server is not holding CTS bit(s) {:?}.",
                req.skill_id, req.bits
            ));
        }
        self.buffs.retain(|b| !held.contains(&b.bit));
        self.reset_reply(
            &held,
            net::buff::TAIL_LEN,
            format!("cancelled by right-click on skill {}", req.skill_id),
        )
    }

    /// One `0x007E`, with the length and the reason written into the log line.
    fn reset_reply(&self, bits: &[u32], tail: usize, why: String) -> Vec<Reply> {
        vec![Reply {
            opcode: net::buff::TEMPORARY_STAT_RESET,
            body: net::buff::temporary_stat_reset_with_tail(bits, tail),
            what: format!(
                "TemporaryStatReset: CTS bit(s) {bits:?} {why}. {}-byte body ({tail}-byte tail); \
                 the 127-byte version threw at 0x142d57322 on 2026-08-22 and the enumerated \
                 reads want 129, or 133 if the gated u32 fires",
                3 + net::buff::MASK_LEN + tail
            ),
        }]
    }

    /// Send `0x007E` for everything held, and drop it from the table. `!unbuff`.
    ///
    /// Kept beside the expiry and the right-click because it is the only one whose tail
    /// length is typeable, which is how the 191 gets bisected without a launch per attempt.
    pub(super) fn clear_buffs(&mut self, tail: usize) -> Vec<Reply> {
        if self.buffs.is_empty() {
            return Vec::new();
        }
        let bits: Vec<u32> = self.buffs.iter().map(|b| b.bit).collect();
        let skills: Vec<u32> = self.buffs.iter().map(|b| b.skill_id).collect();
        self.buffs.clear();
        self.reset_reply(&bits, tail, format!("cleared by !unbuff (from skill(s) {skills:?})"))
    }

}
