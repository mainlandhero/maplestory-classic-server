//! Ability points: `0x0138` for one, `0x0139` for several, and the latch that ate the rest.
//!
//! The owner tried allocating AP on three separate occasions across two runs and nothing
//! happened. It was written up as *"the AP request is `0x0139`"*, and that was **half the
//! answer** - which is why one of those runs still produced nothing after the opcode was
//! known.
//!
//! # There are two requests
//!
//! The stat window's button handler is a flat chain of twelve name comparisons - six stats
//! by two kinds - and the two kinds call two different builders. **[L]**
//!
//! ```text
//! 141152e35  "strup"     edx=0x40  -> 0x142d4baa0  ->  0x0138   u32 tick, u32 statMask
//! 141152e5d  "strupall"  edx=0x40  -> 0x141153070  ->  0x0139   u32 tick, u32 count,
//!                                                               n x (u32 mask, u32 amount)
//! ```
//!
//! A plain `+` click sends **`0x0138`**. Answering only `0x0139` still looks broken to
//! anyone clicking the button, which is exactly what happened.
//!
//! # Why three clicks produced one packet
//!
//! Both builders end with `FUN_142cc4430(ctx, 1)`, setting `[ctx+0x2330] = 1`, and both open
//! with `FUN_142cc42d0`, which **refuses while that field is non-zero**. **[L]** So the
//! client will not send another AP request until something clears it.
//!
//! **That explains some of the missing clicks, not provably all three**, and the first draft
//! of this comment claimed otherwise. Only one of the two captures actually corroborates it:
//! there the latch demonstrably stayed set for 35 s. In the other it was cleared 11 s later
//! and the session then ran 37 s without a second request, which means the owner simply did not
//! click again. Stated precisely because the stronger version reads like a measurement.
//!
//! What clears it is **byte 0 of `0x007C`**. So every path here answers, including every
//! refusal. Same rule as `0x0107` and Log Out, and the same cost for breaking it: not one
//! lost click, but the stat window dead for the rest of the session.
//!
//! # The confound that would have made a broken build look fixed
//!
//! **`session::regen` already clears this latch, by accident.** It sends a `0x007C` every
//! 10 s of standing still and [`net::stats::StatChange::excl_request_sent`] defaults to
//! `true`, so idle regeneration re-opens the stat window on its own. That is not
//! hypothetical - in `world-20260821-001440.log` a regen `0x007C` landed 11 s after the owner's
//! `0x0139` and did exactly that. Field entry clears it too (`FUN_142caa4e0` writes
//! `[ctx+0x2330] = 0`), so a portal walk hides it as well.
//!
//! **So a test of "click, wait, click" passes whether or not this module works.** The test
//! has to be two clicks **within about three seconds**, with no map change - and
//! `grep 0x007C world.log` between the two `<- 0x0138` lines is the free cross-check: if a
//! regen slipped in, the run did not discriminate and is worth re-running rather than
//! believing.
//!
//! # The client will not stop an over-spend
//!
//! The gate at `142d4bc94` looks like a "do you have the points" check and is not: both
//! positive tests jump **to** the send, so the final comparison is only reachable when both
//! operands are non-positive. **The server is the only thing that can say no.** **[L]**
//!
//! Full working, the six-value mask table read out of this client's own string ids, and the
//! controls each instrument was verified against: `research/ap-allocation.md`.

use super::*;

/// How the two opcodes differ once parsed: they do not. Only the body shape does.
type Parse = fn(&[u8]) -> Option<net::abilityup::AbilityUpRequest>;

impl Session {
    /// `0x0138` - the player clicked `+` once beside a stat.
    pub(super) fn on_ability_up(&mut self, payload: &[u8]) -> Vec<Reply> {
        self.ability_up(payload, net::abilityup::parse_ability_up, "0x0138 (one point)")
    }

    /// `0x0139` - the bulk dialog, a counted list of `(mask, amount)` pairs.
    ///
    /// The one call site pushes exactly one pair, so 16 bytes is what has ever been on a
    /// wire - but the builder is a **loop** (`sar rdx,3` over `end - begin`), so the parser
    /// takes the general `8 + 8n` rather than the shape a single capture happens to show.
    /// `research/msexe-packet-fields.txt` records this as four flat `u32`s, which is the
    /// documented `encodes.py` blind spot: it flattens a loop into one iteration.
    pub(super) fn on_ability_mass_up(&mut self, payload: &[u8]) -> Vec<Reply> {
        self.ability_up(payload, net::abilityup::parse_ability_mass_up, "0x0139 (bulk)")
    }

    /// Both requests, since the only difference is how the body is read.
    fn ability_up(&mut self, payload: &[u8], parse: Parse, which: &str) -> Vec<Reply> {
        let Some(mut chr) = self.claimed_character() else {
            // No character means no AP to report, and an empty `0x007C` is still a `0x007C`:
            // it clears the latch. Answering nothing here would wedge the stat window.
            return vec![self.ability_refused(0, which, "no character is claimed")];
        };

        let Some(req) = parse(payload) else {
            return vec![self.ability_refused(chr.ap, which, "the body did not parse")];
        };
        if !req.is_well_formed() {
            return vec![self.ability_refused(
                chr.ap,
                which,
                "an unknown stat mask, or a zero amount",
            )];
        }
        // **The client does not check this.** See the module docs.
        if req.total_points() > u32::from(chr.ap) {
            return vec![self.ability_refused(
                chr.ap,
                which,
                &format!("{} point(s) asked for, {} available", req.total_points(), chr.ap),
            )];
        }

        let mut change = net::stats::StatChange::default();
        let mut spent: Vec<String> = Vec::new();
        for entry in &req.entries {
            let Some(stat) = entry.stat() else { continue };
            let n = entry.amount;
            // The client ASSIGNS what it is sent - every arm calls the setter with the wire
            // value - so this is the new whole total, not the points just spent.
            let new_total = match stat {
                net::abilityup::ApStat::Str => {
                    chr.strength = chr.strength.saturating_add(n as u16);
                    u32::from(chr.strength)
                }
                net::abilityup::ApStat::Dex => {
                    chr.dexterity = chr.dexterity.saturating_add(n as u16);
                    u32::from(chr.dexterity)
                }
                net::abilityup::ApStat::Int => {
                    chr.intelligence = chr.intelligence.saturating_add(n as u16);
                    u32::from(chr.intelligence)
                }
                net::abilityup::ApStat::Luk => {
                    chr.luck = chr.luck.saturating_add(n as u16);
                    u32::from(chr.luck)
                }
                // **`0x800`/`0x2000` are MAX hp and MAX mp.** The button says "HP"; the bit
                // is not current HP, and the request cannot name current HP at all. Current
                // hp/mp are deliberately left alone - raising them is a policy decision the
                // request does not ask for.
                net::abilityup::ApStat::MaxHp => {
                    chr.max_hp = chr
                        .max_hp
                        .saturating_add(n * net::abilityup::policy::MAX_HP_PER_AP);
                    chr.max_hp
                }
                net::abilityup::ApStat::MaxMp => {
                    chr.max_mp = chr
                        .max_mp
                        .saturating_add(n * net::abilityup::policy::MAX_MP_PER_AP);
                    chr.max_mp
                }
            };
            // Picks the right StatChange field AND the right width - u16 for the four
            // stats, u32 for max HP/MP, in a body that has no resync point.
            stat.apply_to(&mut change, new_total);
            chr.ap = chr.ap.saturating_sub(n as u16);
            spent.push(format!("{n} into {} -> {new_total}", stat.name()));
        }

        if let Err(e) = self.store.save_character_progress(&chr) {
            // Still a 0x007C: refusing to answer because the database failed would turn a
            // lost point into a dead stat window.
            return vec![self.ability_refused(chr.ap, which, &format!("could not save: {e}"))];
        }
        // No local copy to update: `claimed_character` reads the store on every call, so
        // the write above IS the update.

        change.ap = Some(chr.ap); // bit 14. Not optional - the client's `+` bails at 0.
        vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: change.build(),
            what: format!(
                "StatChanged: {which} spent {} - {} ap left. Byte 0 clears the ctx+0x2330 latch; without it the client never sends another AP request this session",
                spent.join(", "),
                chr.ap
            ),
        }]
    }

    /// A refusal that still clears the latch and corrects the client's idea of its AP.
    ///
    /// **There is no dedicated failure packet.** The whole `0x70..0x19f` channel-stage table
    /// was searched for one and the reference version has none either - it answers a refused
    /// allocation with an empty latch-clear. So a refusal is an ordinary `0x007C` carrying
    /// only the AP the character really has: it moves no stat, and it leaves the client
    /// agreeing with the server about how many points are left.
    fn ability_refused(&self, ap: u16, which: &str, why: &str) -> Reply {
        let change = net::stats::StatChange { ap: Some(ap), ..Default::default() };
        Reply {
            opcode: net::stats::STAT_CHANGED,
            body: change.build(),
            what: format!(
                "StatChanged: {which} REFUSED ({why}); re-sending ap={ap}. Every refusal is still a 0x007C - byte 0 is what clears the client's request latch"
            ),
        }
    }
}
