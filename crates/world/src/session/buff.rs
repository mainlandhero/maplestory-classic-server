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
        vec![Reply {
            opcode: net::buff::TEMPORARY_STAT_SET,
            body: net::buff::temporary_stat_set(&[stat]),
            what: format!(
                "TemporaryStatSet: skill {skill_id} grants CTS bit {} = {} for {} ms ({} s from Skill.wz). \
                 OUTBOUND 0x007D is TemporaryStatSet; the client's INBOUND 0x007D is the migration hello, \
                 a different opcode space. The value's width is the one guess here - i16, research/buffs.md 5.5",
                stat.bit, stat.value, stat.duration_ms, level.seconds
            ),
        }]
    }

    /// Send `0x007E` for anything whose time is up. Called from [`Session::tick`].
    ///
    /// **The server expires it as well as the client**, even though the client holds its own
    /// `tExpire` and would drop the icon on its own. Two reasons: our table is what decides
    /// whether a recast is allowed to replace the bit, and a reset that the client has
    /// already performed is harmless, while a bit we think is held and the client does not is
    /// a desync nobody would see until the next cast did nothing.
    pub(super) fn buff_tick(&mut self, now_ms: u64) -> Vec<Reply> {
        let done: Vec<ActiveBuff> =
            self.buffs.iter().copied().filter(|b| now_ms >= b.expires_ms).collect();
        if done.is_empty() {
            return Vec::new();
        }
        self.buffs.retain(|b| now_ms < b.expires_ms);
        let bits: Vec<u32> = done.iter().map(|b| b.bit).collect();
        let skills: Vec<u32> = done.iter().map(|b| b.skill_id).collect();
        vec![Reply {
            opcode: net::buff::TEMPORARY_STAT_RESET,
            body: net::buff::temporary_stat_reset(&bits),
            what: format!(
                "TemporaryStatReset: CTS bit(s) {bits:?} expired (from skill(s) {skills:?})"
            ),
        }]
    }

}
