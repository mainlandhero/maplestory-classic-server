//! Standing still gets your health back.
//!
//! The owner, 2026-08-20: *"Whenever the user is idle, they should get 10 HP and MP every 10
//! seconds."*
//!
//! # What "idle" means here, since the client never says so
//!
//! There is no "I am idle" packet. What the client does send is what it is *doing*: a
//! movement report (`0x00D9`), an attack (`0x02F4`), and its own "I took damage" (`0x00E5`).
//! So idleness is the absence of those, measured by a timestamp that each of them stamps -
//! [`Session::note_activity`].
//!
//! **The clock is the session's own `now_ms`**, milliseconds since this connection opened,
//! not a wall clock. Everything else in this session is a pure function of state and time so
//! that it can be a unit test, and regeneration is the one feature where the temptation to
//! reach for `SystemTime` is strongest - a tick every ten seconds is exactly the thing nobody
//! wants to test by sleeping. The scrolling banner uses a wall clock because it has to agree
//! across two processes; this does not.
//!
//! # Why it does not tick while you are already full
//!
//! A `0x007C` that changes nothing still costs a packet and still makes the client redraw.
//! With thirty mobs on screen the channel is already sending several hundred packets a second
//! and the last thing it needs is a heartbeat that says nothing. So a tick that would heal
//! nothing sends nothing, and the timer does not even start until something is missing.
//!
//! # Improved MP Recovery adds to the MP half. HP has no skill at all.
//!
//! Skill `2000000`'s **`x` column** is the second bonus on this passive, and it lands here.
//! The client's own tooltip, verbatim from `String.wz` and identical on all fifteen levels:
//!
//! > *"Regenerates **1% of Max MP** every 10 seconds; increases MP recovery from items by N%"*
//!
//! **[L]** - the pool, the proportion and the clock in one clause. It is a **percentage of max
//! MP**, not a flat amount; `x` is `1` at every level. The full working, every control and
//! every named negative, is in **`research/mp-regen.md`**.
//!
//! ## It ADDS to [`REGEN_AMOUNT`] rather than replacing it
//!
//! Three readings were possible - instead of the base, added to it, or a replacement above a
//! floor. **"Instead of" is eliminated by measurement** and the other two by one argument:
//!
//! ```text
//!   Cobalt, the character this was reported on:  max MP 237
//!     the flat base                              10 MP / tick
//!     1% of 237, floored                          2 MP / tick     <- the skill alone
//! ```
//!
//! **[D]** Replacement makes a learned passive a *five-fold downgrade*, and worse than that at
//! the bottom: 1% of anything under 100 floors to **zero**, and a fresh Magician's max MP is
//! `5`. "Replace" would mean learning a recovery skill switches recovery off. A
//! max-above-a-floor reading never harms, but it does nothing whatsoever until max MP reaches
//! 1000 - which no character on this server has ever had - so it is indistinguishable on
//! screen from the unwired state this change exists to remove.
//!
//! The word is also the client's. The second-job twin of this skill - same name, same
//! ten-second clock, and an `x` that **varies** 3..22 so the substitution is observable rather
//! than assumed - says it outright:
//!
//! ```text
//!   1110000 / 1210000  Improved MP Recovery   "Recover 22 additional MP every 10 sec."
//!                                                       ^^^^^^^^^^
//! ```
//!
//! That twin is also the reason the unit here cannot be taken on trust: **it is the same
//! column carrying the other unit.** Its slot has no `%` and says *"additional MP"*; this
//! one's has a `%` and says *"of Max MP"*. `CLAUDE.md`'s *"the unit, not the arithmetic"*,
//! with both readings live in one column of one book.
//!
//! ## The clock agrees, and the tooltip is the only thing that states it
//!
//! [`REGEN_EVERY_MS`] is `10_000` and the tooltip says *"every 10 seconds"*. They agree.
//!
//! Worth one sentence because the agreement is easy to mistake for corroboration: **no column
//! in `Skill.wz` carries an interval for this skill.** Enumerated - every non-empty field on
//! `2000000` is `skillId, job, level, maxLevel, name, type, hs, x, y`; there is no `time`, no
//! `subTime`, no `dotInterval`. The tooltip *text* is the single statement of the period
//! anywhere in this client's data, and this server already ticked on the same number for its
//! own reasons. [`tests::the_tick_interval_is_the_one_the_tooltip_states`] pins it.
//!
//! [`IDLE_AFTER_MS`] is a different quantity - how long you must stand still before any of
//! this starts - and the tooltip says nothing about it. It stays the owner's rule.
//!
//! # This heartbeat clears the client's request latch, and that is a side effect
//!
//! Every `0x007C` carries `excl_request_sent = 1` in byte 0, which clears
//! `CWvsContext+0x2330` - the one-request-outstanding latch that gates ability-point and
//! several other requests. So **idle regeneration silently re-opens the stat window every
//! ten seconds**, whether or not anything answered the request that latched it.
//!
//! Nothing here needs changing - clearing a latch nobody set is harmless, and that is why
//! `excl_request_sent` defaults to `true` in the first place. It is written down because it
//! is a **confound for testing**: a run that clicks `+`, waits, and clicks again will see the
//! second click work even if [`super::ability`] is completely broken. Measured, not feared -
//! in `world-20260821-001440.log` a regen `0x007C` landed 11 s after an AP request and did
//! exactly that. See `session::ability`'s module docs for the test that does discriminate.

use super::*;

/// How long the player must have done nothing before regeneration starts.
pub(super) const IDLE_AFTER_MS: u64 = 10_000;

/// How often it ticks once it has started.
pub(super) const REGEN_EVERY_MS: u64 = 10_000;

/// The **base** each tick restores, to both pools, before any skill.
///
/// The owner's number. Flat, the same for HP and MP, not class-dependent. It is no longer the whole
/// story for MP - see [`Session::mp_regen_amount`] - but it is still the floor under it, and
/// it remains the entire story for HP, because this client's data contains no HP regeneration
/// amount for anybody. See the `1000000` note on [`crate::itemrecovery::mp_regen_percent`].
pub(super) const REGEN_AMOUNT: u32 = 10;

/// One tick's MP regeneration, with the two halves kept apart.
///
/// Named fields rather than a `(u32, u32)`, for the reason `itemrecovery::Learned`'s doc gives
/// about positional pairs: [`Self::amount`] is the total and [`Self::bonus`] is only the
/// skill's share, and swapping them at a call site would be silent - the log line would say
/// the skill did all the work, or the tick would restore two points instead of twelve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MpRegen {
    /// What the tick actually restores: [`REGEN_AMOUNT`] + [`Self::bonus`].
    amount: u32,
    /// The skill's share alone, `0` when unlearned or when the floor ate it.
    bonus: u32,
    /// The percent of max MP that produced it, for the log line. `0` when unlearned.
    percent: u32,
}

impl Session {
    /// Remember that the player just did something. Called from every packet that proves it.
    pub(super) fn note_activity(&mut self) {
        self.last_activity_ms = self.clock_ms;
        // Restart the countdown, rather than leaving a tick that was already due to fire the
        // instant they stop. Without this, running around for a minute and then pausing pays
        // out immediately instead of after ten seconds.
        self.next_regen_ms = None;
    }

    /// What one idle tick restores to **MP**, with Improved MP Recovery folded in.
    ///
    /// [`REGEN_AMOUNT`] plus `x`% of `max_mp`, floored - see the module header for why it adds
    /// rather than replaces, and `research/mp-regen.md` §3 for the working.
    ///
    /// # Called after the early returns, not before
    ///
    /// This reads the database, and `regen_tick` runs on every pass of the session loop while
    /// costing nothing in the overwhelmingly common cases (not idle yet, already full, not
    /// due). Putting the query behind those gates makes it **one row per ten seconds per idle
    /// session** instead of one per loop.
    ///
    /// `Store::skill_level` answers `Ok(0)` for a skill the character never raised, so
    /// `unwrap_or(0)` is reached only on a real database failure - and there it degrades to the
    /// flat base rather than skipping the tick. `CLAUDE.md`'s *always answer*: a regeneration
    /// that refuses is a bar that stops moving for the rest of the session.
    fn mp_regen(&self, character_id: u32, max_mp: u32) -> MpRegen {
        let level = self
            .store
            .skill_level(character_id, crate::itemrecovery::IMPROVED_MP_RECOVERY)
            .unwrap_or(0);
        let percent =
            crate::itemrecovery::mp_regen_percent(crate::itemrecovery::IMPROVED_MP_RECOVERY, level);
        let bonus = crate::itemrecovery::regen_of_max(max_mp, percent);
        // **The third-job warriors' "Improved MP Recovery"** (`1110000` Crusader, `1210000`
        // White Knight) is a different skill from the Magician's `2000000` despite the name:
        // its `x` is FLAT - *"Recover 3 additional MP every 10 sec."*, 3..22 - not a percent
        // of the maximum. Read as such, and added on top. **[L]** for the unit, off the tooltip.
        let flat: u32 = crate::advbuffs::IMPROVED_MP_RECOVERY_3RD
            .iter()
            .filter_map(|&skill| {
                let level = self.store.skill_level(character_id, skill).unwrap_or(0);
                if level == 0 {
                    return None;
                }
                self.config.firstjob.level(skill, level)?.x.and_then(|x| u32::try_from(x).ok())
            })
            .sum();
        MpRegen {
            amount: REGEN_AMOUNT.saturating_add(bonus).saturating_add(flat),
            bonus: bonus.saturating_add(flat),
            percent,
        }
    }

    /// The regeneration this tick owes, if any. At most one `0x007C`.
    pub(super) fn regen_tick(&mut self, now_ms: u64) -> Vec<Reply> {
        if now_ms.saturating_sub(self.last_activity_ms) < IDLE_AFTER_MS {
            return Vec::new();
        }
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        // **The dead do not regenerate.** The owner, 2026-08-27: *"If the player is dead, they
        // should no longer have passive regeneration. Currently I can actually passive
        // regenerate out of death, which is not okay."*
        //
        // They are right, and it was worse than a cosmetic wrong: `hp == 0` is the ONLY thing
        // that makes a character dead here - `combat.rs`'s revive dialog gates on
        // `before > 0 && chr.hp == 0` - so a regen tick lifting HP to 10 quietly *undid the
        // death*. The dialog had already been shown and will not be shown again, because it
        // fires on the transition rather than the state, so the player ends up alive, in the
        // map they died in, with a revive dialog that no longer means anything.
        //
        // The timer is cleared as well as skipped. Leaving it armed would pay out the instant
        // the player revived, which is the same "already due the moment they stop" bug
        // `note_activity` exists to prevent; the revive path calls `note_activity` itself so
        // the countdown restarts from the revive.
        if chr.hp == 0 {
            self.next_regen_ms = None;
            return Vec::new();
        }
        // Nothing to restore: do not send, and do not arm a timer. A player who sits at full
        // health for an hour should cost exactly nothing.
        //
        // **"Full" is measured against the ceiling the CLIENT draws**, not the base in the
        // record. The owner, 2026-09-06: a Swordsman with Max HP Increase at 15 stood at 358/447
        // on screen while this line read `358 >= 358` and called them full - `session::pools`.
        let pools = self.pools(&chr);
        if chr.hp >= pools.max_hp && chr.mp >= pools.max_mp {
            self.next_regen_ms = None;
            return Vec::new();
        }
        // First tick lands one interval after they went idle, not the moment they do.
        let due = *self
            .next_regen_ms
            .get_or_insert(self.last_activity_ms.saturating_add(IDLE_AFTER_MS));
        if now_ms < due {
            return Vec::new();
        }
        self.next_regen_ms = Some(now_ms.saturating_add(REGEN_EVERY_MS));

        // **HP is the flat base and nothing else**, because `1000000` carries no `x` at any of
        // its fifteen levels and its tooltip states no HP number - so there is no HP
        // regeneration amount in this client's data to apply. Inventing one is the thing the
        // symmetry with MP most invites; `itemrecovery::mp_regen_percent` refuses the HP id
        // for the same reason.
        // **The chair.** The owner, 2026-09-08: *"my character just sat in a chair, but the idle
        // recovery did not adjust to match the chair's recovery stats."* The Red Chair's own
        // tooltip says *"Restores an additional 30 HP every 10 seconds"* - `additional`, so it
        // adds to the flat base rather than replacing it, and `10 seconds` is exactly
        // `REGEN_EVERY_MS`, so it is one chair figure per tick with no scaling. Both halves of
        // that are read off the client rather than assumed; `world::chairs`.
        //
        // `(0, 0)` when standing, when the chair id is unknown, and when `gm-handbook/` has
        // never been generated - all of which leave this line as it was before chairs existed.
        let (chair_hp, chair_mp) = self.chair_recovery();
        let hp = chr
            .hp
            .saturating_add(REGEN_AMOUNT)
            .saturating_add(chair_hp)
            .min(pools.max_hp.max(chr.hp));
        // The 1% is of the max the client shows, which is the one a learned Max MP Increase
        // has already raised - the tooltip says "of Max MP", and that is the number on screen.
        let mp_regen = self.mp_regen(chr.id, pools.max_mp);
        let mp = chr
            .mp
            .saturating_add(mp_regen.amount)
            .saturating_add(chair_mp)
            .min(pools.max_mp.max(chr.mp));
        let healed_hp = hp - chr.hp;
        let healed_mp = mp - chr.mp;
        chr.hp = hp;
        chr.mp = mp;
        if let Err(e) = self.store.save_character_progress(&chr) {
            // Do not spam the chat every ten seconds about a database that is not working.
            // The log is the right place for this one.
            return vec![Reply {
                opcode: net::notice::CHAT_NOTICE,
                body: net::notice::chat_notice("Could not save your health."),
                what: format!("regen: save failed: {e}"),
            }];
        }
        // **The blue number over the player's head.** The owner, 2026-08-21: *"the idle recovery
        // should pop up with a blue number of the recovery amount above the player's head. I
        // don't see that here, the HP bar just moves up without a number indication."*
        //
        // **This is NOT `0x007C`'s recovery trailer, and that trailer was tried and cannot
        // work.** It was sent for a build - `u8 flag`, then `u32 hpRecovery, u32 mpRecovery` -
        // on three regen ticks of one run, on a bar that visibly moved, and drew nothing. The
        // reason is not a bad body: `FUN_140fd31f0`, which the client hands those two values
        // to, is a **statistics counter**. It accumulates into running totals, separates
        // effective healing from wasted against `maxHp - oldHp`, keeps per-hour averages and
        // resets on an hour boundary. Its entire call list is two tick functions, a getter
        // twice and a tail `jmp`. There is no renderer on that path at all.
        //
        // The follow-up theory - that the `oldHp` snapshot is taken after the mask block
        // stores the new total, so the delta is zero - is dead too, and is worth recording
        // because testing it would have cost a client run: the snapshots are taken 65 bytes
        // *earlier*, into a stack slot and a callee-saved register the mask block cannot
        // reach. Dropping the hp/mp bits would have produced the identical blank screen.
        //
        // What actually draws it is `0x02D1` effect `0x41`, which reaches the same renderer as
        // the damage number with a **positive** argument - the sign is what selects blue over
        // violet. `net::revive::recovery_number`.
        //
        // Sent **after** the `0x007C`, so the bar and the number agree on screen. HP only: a
        // tick that restores both would otherwise stack two numbers on one head, and the owner
        // asked for the recovery amount, singular.
        // **A full bar is not touched at all.** The owner, 2026-08-22: *"the server should not try
        // to idle regenerate if a character is full HP."*
        //
        // The tick already returns early when BOTH bars are full, and the log bears that out -
        // the last tick of a heal is the capped `+4 hp -> 194/194` and then it stops. What it
        // did not do is handle the halves separately: with HP full and MP short, the `0x007C`
        // still carried `hp = <full>`, restating a value that had not moved.
        //
        // So each field is present only if it actually changed. That is strictly less traffic
        // and it cannot regress anything: the client applies the fields the mask names and
        // leaves the rest alone.
        let mut out = vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange {
                hp: (healed_hp > 0).then_some(chr.hp),
                mp: (healed_mp > 0).then_some(chr.mp),
                ..Default::default()
            }
            .build(),
            what: format!(
                "StatChanged: idle regen +{healed_hp} hp +{healed_mp} mp -> {}/{} hp, {}/{} mp{}{}",
                chr.hp, pools.max_hp, chr.mp, pools.max_mp,
                match (healed_hp > 0, healed_mp > 0) {
                    (true, true) => "",
                    (true, false) => " - MP is full, so the packet does not mention it",
                    (false, true) => " - HP is full, so the packet does not mention it",
                    (false, false) => " - NOTHING moved, which the early return should have caught",
                },
                // **The only place a run can show the MP bonus fired**, and without it a
                // capture cannot tell "the skill applied 0%" from "the wiring is absent" -
                // which is exactly the confusion that left the item half of this same skill
                // open for a week. It prints the pool it was a percentage OF, because the
                // floor makes a small pool contribute nothing and that reads like a failure.
                if mp_regen.bonus > 0 {
                    format!(
                        " [Improved MP Recovery: +{} MP on top of the flat {REGEN_AMOUNT}, {}% of {} max MP]",
                        mp_regen.bonus, mp_regen.percent, pools.max_mp
                    )
                } else if mp_regen.percent > 0 {
                    format!(
                        " [Improved MP Recovery is learned but {}% of {} max MP floors to 0 - the flat {REGEN_AMOUNT} is the whole tick]",
                        mp_regen.percent, pools.max_mp
                    )
                } else {
                    String::new()
                }
            ),
        }];
        if healed_hp > 0 {
            out.push(Reply {
                opcode: net::stats::USER_EFFECT_LOCAL,
                body: net::revive::recovery_number(healed_hp as i32, 0),
                what: format!(
                    "UserEffectLocal effect 0x41: the blue +{healed_hp} over the player's head. NOT the 0x007C recovery trailer - that one reaches a statistics counter, not a renderer, and drew nothing across three ticks of a real run"
                ),
            });
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::itemrecovery;
    use std::sync::Arc;
    use store::Store;

    /// A claimed session whose character is on 1 HP and 1 MP, so every tick has something to
    /// heal. Local rather than shared with `session::tests`: that module's helper is private
    /// to it, and a regeneration test wants a *hurt* character, which is the opposite of what
    /// every other test there wants.
    fn hurt_session() -> (Session, Arc<Store>) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Regen".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let config = Config { set_field_probe: true, ..Config::default() };
        let mut s = Session::new(store.clone(), Arc::new(config));
        assert!(s.claim_for_character(id).contains("claimed the migration"));
        let mut chr = s.claimed_character().unwrap();
        // Room to heal twice without hitting the ceiling. A fresh character's max MP is 5,
        // which is fine behaviour and a terrible fixture: the cap would fire on the first
        // tick and the test would be measuring the cap instead of the interval.
        chr.max_hp = 100;
        chr.max_mp = 100;
        chr.hp = 1;
        chr.mp = 1;
        store.save_character_progress(&chr).unwrap();
        (s, store)
    }

    /// **The blue number is a second packet, not a field of the first.**
    ///
    /// The owner asked for *"a blue number of the recovery amount above the player's head"*. The
    /// first attempt put it in `0x007C`'s recovery trailer; that reaches a statistics counter,
    /// not a renderer, and drew nothing across three ticks of a real run. What draws it is
    /// `0x02D1` effect `0x41`.
    ///
    /// This asserts the `0x007C` is byte-identical to one built with no trailer, so the dead
    /// route cannot creep back in beside the live one.
    #[test]
    fn a_regen_tick_sends_the_bar_and_then_the_blue_number() {
        let (mut s, _) = hurt_session();
        s.clock_ms = 10_000;
        let out = s.regen_tick(10_000);
        assert_eq!(out.len(), 2, "the 0x007C and the number");

        let want = net::stats::StatChange {
            hp: Some(1 + REGEN_AMOUNT),
            mp: Some(1 + REGEN_AMOUNT),
            ..Default::default()
        }
        .build();
        assert_eq!(out[0].body, want, "no recovery trailer - it cannot draw anything");
        assert_eq!(*out[0].body.last().unwrap(), 0, "the trailer flag byte is 0");

        assert_eq!(out[1].opcode, net::stats::USER_EFFECT_LOCAL);
        assert_eq!(out[1].body, net::revive::recovery_number(REGEN_AMOUNT as i32, 0));
        assert_eq!(out[1].body[0], net::revive::EFFECT_RECOVERY_NUMBER);
        assert_eq!(
            i32::from_le_bytes(out[1].body[1..5].try_into().unwrap()),
            REGEN_AMOUNT as i32,
            "the AMOUNT recovered, not the new total - the total would draw +11 for a 10 HP tick"
        );
    }

    /// **A bar that is already full is not mentioned in the packet.**
    ///
    /// The owner, 2026-08-22: *"the server should not try to idle regenerate if a character is full
    /// HP."* The both-full case was already an early return; this is the half that was not -
    /// with HP full and MP short, the `0x007C` still carried `hp = <full>`.
    ///
    /// Asserted against a hand-built expectation rather than by poking at the mask, so a
    /// change to any earlier field cannot shift an offset and still pass.
    #[test]
    fn a_full_bar_is_left_out_of_the_regen_packet() {
        let (mut s, store) = hurt_session();
        let mut chr = s.claimed_character().unwrap();
        chr.hp = chr.max_hp; // full HP, MP still at 1
        store.save_character_progress(&chr).unwrap();

        s.clock_ms = 10_000;
        let out = s.regen_tick(10_000);
        let want = net::stats::StatChange {
            hp: None,
            mp: Some(1 + REGEN_AMOUNT),
            ..Default::default()
        }
        .build();
        assert_eq!(out[0].body, want, "HP is full, so the packet must not carry it");
        assert_ne!(
            out[0].body,
            net::stats::StatChange {
                hp: Some(chr.max_hp),
                mp: Some(1 + REGEN_AMOUNT),
                ..Default::default()
            }
            .build(),
            "and it is not the same as restating the full value"
        );
    }

    /// A tick that only restores MP draws no number at all.
    ///
    /// The number is the HP recovery and the owner asked for one number, so an MP-only tick must
    /// send the bar update alone rather than a blue `+0`.
    #[test]
    fn an_mp_only_tick_draws_no_number() {
        let (mut s, store) = hurt_session();
        let mut chr = s.claimed_character().unwrap();
        chr.hp = chr.max_hp; // full HP, MP still at 1
        store.save_character_progress(&chr).unwrap();

        s.clock_ms = 10_000;
        let out = s.regen_tick(10_000);
        assert_eq!(out.len(), 1, "the bar update and nothing else");
        assert_eq!(out[0].opcode, net::stats::STAT_CHANGED);
        assert!(
            !out.iter().any(|r| r.opcode == net::stats::USER_EFFECT_LOCAL),
            "no blue +0 over the head"
        );
    }

    /// Did a tick fire? These tests are about the CLOCK, not the packet count, and counting
    /// replies made all three fail the day regeneration grew its second packet. Asking for the
    /// `0x007C` specifically survives another one being added beside it.
    pub(super) fn ticked(out: &[Reply]) -> bool {
        out.iter().any(|r| r.opcode == net::stats::STAT_CHANGED)
    }

    #[test]
    fn nothing_happens_before_ten_seconds_of_quiet() {
        let (mut s, _) = hurt_session();
        for now in [0u64, 500, 5_000, 9_999] {
            s.clock_ms = now;
            assert!(s.regen_tick(now).is_empty(), "regenerated at {now} ms");
        }
    }

    #[test]
    fn ten_hp_and_ten_mp_every_ten_seconds() {
        let (mut s, store) = hurt_session();
        let id = s.claimed_character().unwrap().id;
        s.clock_ms = 10_000;
        assert!(ticked(&s.regen_tick(10_000)), "the first tick lands at 10 s");
        let after = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
        assert_eq!(after.hp, 1 + REGEN_AMOUNT);
        assert_eq!(after.mp, 1 + REGEN_AMOUNT);

        assert!(s.regen_tick(15_000).is_empty(), "not again until the interval is up");
        assert!(ticked(&s.regen_tick(20_000)), "and then again at 20 s");
    }

    #[test]
    fn doing_something_restarts_the_countdown() {
        let (mut s, _) = hurt_session();
        s.clock_ms = 9_000;
        s.note_activity();
        // Ten seconds after the connection opened, but only one second after they moved.
        assert!(s.regen_tick(10_000).is_empty());
        assert!(s.regen_tick(18_999).is_empty(), "still inside the idle window");
        assert!(ticked(&s.regen_tick(19_000)), "ten seconds after the ACTIVITY");
    }

    /// **The dead do not regenerate, and that is not cosmetic.**
    ///
    /// The owner, 2026-08-27: *"Currently I can actually passive regenerate out of death, which is
    /// not okay."* `hp == 0` is the only thing that makes a character dead here, and the
    /// revive dialog fires on the TRANSITION (`before > 0 && hp == 0`), so a regen tick that
    /// lifted HP off zero silently un-killed the player and the dialog never came back.
    #[test]
    fn a_dead_player_does_not_regenerate_out_of_death() {
        let (mut s, store) = hurt_session();
        let mut chr = s.claimed_character().unwrap();
        chr.hp = 0;
        store.save_character_progress(&chr).unwrap();

        // Half an hour of perfect stillness must not restore a single point.
        for t in (10_000..1_800_000).step_by(10_000) {
            assert!(s.regen_tick(t).is_empty(), "a tick fired at {t} ms on a dead character");
        }
        assert_eq!(s.claimed_character().unwrap().hp, 0, "still dead");

        // And the timer was not left armed, so reviving does not pay out instantly.
        assert!(s.next_regen_ms.is_none(), "a dead character must not hold an armed timer");
    }

    /// Once revived, regeneration works again - or the gate above is a permanent off switch.
    #[test]
    fn reviving_restores_regeneration() {
        let (mut s, store) = hurt_session();
        let mut chr = s.claimed_character().unwrap();
        chr.hp = 0;
        store.save_character_progress(&chr).unwrap();
        assert!(s.regen_tick(20_000).is_empty());

        chr.hp = 50;
        store.save_character_progress(&chr).unwrap();
        // Reviving is doing something, so the countdown restarts FROM the revive. The clock
        // has to be moved first: `note_activity` stamps `clock_ms`, which the harness never
        // advances on its own.
        s.clock_ms = 20_000;
        s.note_activity();
        assert!(s.regen_tick(25_000).is_empty(), "five seconds after reviving is too soon");
        assert!(ticked(&s.regen_tick(30_000)), "and ten seconds after it, regen resumes");
    }

    /// **Combat resets the countdown, from both directions.**
    ///
    /// The owner: *"If the player gets hit or attacks another monster, the 10 second is reset."*
    /// Both were already wired - `combat.rs` calls `note_activity` on the swing and on the
    /// hit - but nothing asserted it, and an untested reset is one refactor away from being
    /// gone. This pins the property at the level the owner stated it.
    #[test]
    fn combat_in_either_direction_restarts_the_countdown() {
        for who in ["the player swings", "the mob connects"] {
            let (mut s, _) = hurt_session();
            s.clock_ms = 9_000;
            s.note_activity(); // what both combat paths do
            assert!(s.regen_tick(10_000).is_empty(), "{who}: one second after acting");
            assert!(s.regen_tick(18_999).is_empty(), "{who}: still inside the window");
            assert!(ticked(&s.regen_tick(19_000)), "{who}: ten seconds after acting");
        }
    }

    #[test]
    fn a_full_player_costs_nothing() {
        let (mut s, _) = hurt_session();
        let mut chr = s.claimed_character().unwrap();
        chr.hp = chr.max_hp;
        chr.mp = chr.max_mp;
        s.store.save_character_progress(&chr).unwrap();
        for now in [10_000u64, 20_000, 30_000, 3_600_000] {
            assert!(s.regen_tick(now).is_empty(), "sent a packet that healed nothing at {now}");
        }
    }

    // -----------------------------------------------------------------------------------
    // Improved MP Recovery - skill 2000000's `x` column
    // -----------------------------------------------------------------------------------

    /// Give the hurt character a skill at a level, and a pool big enough to see it.
    fn with_skill(skill_id: u32, level: u32, max_mp: u32) -> (Session, Arc<Store>, u32) {
        let (s, store) = hurt_session();
        let mut chr = s.claimed_character().unwrap();
        chr.max_mp = max_mp;
        chr.mp = 1;
        store.save_character_progress(&chr).unwrap();
        store.set_skill_level(chr.id, skill_id, level).unwrap();
        // The instrument speaks before its silence is believed: if the row did not land, every
        // assertion below would read as "the bonus is absent" and pass for the wrong reason.
        assert_eq!(store.skill_level(chr.id, skill_id).unwrap(), level, "the skill row is set");
        (s, store, chr.id)
    }

    /// **Max HP Increase raises the ceiling the server regenerates TO, not the amount.**
    ///
    /// The owner, 2026-09-06: Cobalt at 358/447 was not regenerating because the server compared
    /// 358 against its own 358. Base 100, skill at 15 (+25%): a character at 100 is NOT full,
    /// ticks to 110, and stops at 125 - while the `0x007C` carries the HP past the base, and
    /// the record's `max_hp` stays 100 because the client adds the percent itself.
    #[test]
    fn max_hp_increase_raises_the_ceiling_the_server_regenerates_to() {
        if !std::path::Path::new("../../gm-handbook/skills.txt").exists() {
            return; // generated, gitignored
        }
        let (mut s, store) = hurt_session();
        let mut cfg = (*s.config).clone();
        cfg.firstjob = crate::firstjob::CombatTable::load(std::path::Path::new("../../gm-handbook/skills.txt"));
        s.config = Arc::new(cfg);
        let mut chr = s.claimed_character().unwrap();
        chr.hp = 100; // at the BASE maximum
        chr.mp = 100;
        store.save_character_progress(&chr).unwrap();
        let id = chr.id;

        // Unlearned: 100/100 is full and nothing ticks.
        s.clock_ms = 10_000;
        assert!(!ticked(&s.regen_tick(10_000)), "full against the base: no tick");

        store.set_skill_level(id, super::super::pools::MAX_HP_INCREASE, 15).unwrap();
        s.note_activity(); // restart the idle clock so the first tick is one interval out
        s.clock_ms = 10_000;
        let out = s.regen_tick(20_000);
        assert!(ticked(&out), "learned: 100 of 125 is not full - {out:?}");
        let rec = |store: &Arc<Store>| store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
        assert_eq!(rec(&store).hp, 110, "past the base maximum");
        assert_eq!(rec(&store).max_hp, 100, "the base is untouched - the client adds the 25%");

        // Up to the ceiling and no further.
        for t in [30_000u64, 40_000, 50_000] {
            let _ = s.regen_tick(t);
        }
        assert_eq!(rec(&store).hp, 125, "capped at 100 + 25");
        assert!(!ticked(&s.regen_tick(60_000)), "and 125/125 is full");
    }

    /// **An unlearned character regenerates exactly the flat amount, unchanged.**
    ///
    /// The regression guard for the whole change: folding a skill lookup into this path must
    /// not move the number for the characters who have no skill, which is all of them until
    /// job 200. Stated separately from the tests above so that it fails as *"the base moved"*
    /// rather than as one of six clock assertions.
    #[test]
    fn an_unlearned_character_still_gets_exactly_the_flat_amount() {
        let (mut s, store) = hurt_session();
        let id = s.claimed_character().unwrap().id;
        assert_eq!(store.skill_level(id, itemrecovery::IMPROVED_MP_RECOVERY).unwrap(), 0);

        s.clock_ms = 10_000;
        assert!(ticked(&s.regen_tick(10_000)));
        let after = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
        assert_eq!(after.hp, 1 + REGEN_AMOUNT, "HP is the flat base");
        assert_eq!(after.mp, 1 + REGEN_AMOUNT, "and so is MP, with no skill to add to it");
    }

    /// **The bonus is 1% of MAX MP, added to the flat base - and it lands in all four places.**
    ///
    /// `CLAUDE.md`'s Heena rule: a handler with N effects needs a test that says something
    /// about N of them. This tick produces four - the database row, the `0x007C` body, the blue
    /// number, and the log line - and a test that checked only the reply would pass on a
    /// handler that healed the packet and not the character.
    ///
    /// Cobalt's real pool, 237 max MP: `floor(237 * 1 / 100) = 2`, so **12**, not 10 and not 2.
    #[test]
    fn improved_mp_recovery_adds_one_percent_of_max_mp_to_the_mp_half() {
        let (mut s, store, id) = with_skill(itemrecovery::IMPROVED_MP_RECOVERY, 15, 237);
        s.clock_ms = 10_000;
        let out = s.regen_tick(10_000);

        // (a) the database
        let after = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
        assert_eq!(after.mp, 1 + 12, "10 flat + floor(1% of 237) = 12");
        assert_eq!(after.hp, 1 + REGEN_AMOUNT, "HP is untouched by a Magician's MP passive");

        // (b) the 0x007C body
        let want = net::stats::StatChange {
            hp: Some(1 + REGEN_AMOUNT),
            mp: Some(1 + 12),
            ..Default::default()
        }
        .build();
        assert_eq!(out[0].body, want, "the packet carries the bonused total");

        // (c) the blue number is the HP recovery and must NOT have grown
        assert_eq!(
            out[1].body,
            net::revive::recovery_number(REGEN_AMOUNT as i32, 0),
            "the number over the head is the HP amount, which this skill does not change"
        );

        // (d) the log line, which is the only thing a capture can read
        assert!(
            out[0].what.contains("Improved MP Recovery: +2 MP"),
            "the log must say the bonus fired: {}",
            out[0].what
        );
        assert!(out[0].what.contains("1% of 237 max MP"), "{}", out[0].what);
    }

    /// **It ADDS rather than replacing, and this is the test that discriminates.**
    ///
    /// At 50 max MP the skill's own contribution is `floor(0.5) = 0`. The three candidate
    /// readings disagree here and nowhere else:
    ///
    /// ```text
    ///   instead of the base    ->  0 MP   a learned recovery passive that stops recovery
    ///   max(base, bonus)       -> 10 MP
    ///   base + bonus           -> 10 MP   <- and identical to max() until 1000 max MP
    /// ```
    ///
    /// So this pins the elimination of "instead of", which is the reading that would be
    /// *invisible in every other test here* - at 237 max MP a replacement gives 2, which is
    /// still non-zero and still looks like a working feature.
    #[test]
    fn a_pool_too_small_to_earn_a_bonus_still_regenerates_the_flat_amount() {
        let (mut s, store, id) = with_skill(itemrecovery::IMPROVED_MP_RECOVERY, 15, 50);
        s.clock_ms = 10_000;
        let out = s.regen_tick(10_000);

        let after = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
        assert_eq!(after.mp, 1 + REGEN_AMOUNT, "not 0, and not 1% of 50");

        // And the log says WHY the bonus is absent, so a capture cannot read it as unwired.
        assert!(
            out[0].what.contains("floors to 0"),
            "a learned skill contributing nothing must say so: {}",
            out[0].what
        );
    }

    /// **The Warrior twin adds nothing to either pool**, because this client's data has no HP
    /// regeneration amount anywhere.
    ///
    /// `1000000` carries no `x` at any of its fifteen levels;
    /// `itemrecovery::tests::only_the_magician_half_carries_an_x_column` pins that against the
    /// generated file. This is the session-level half: a Warrior at level 15 must regenerate
    /// exactly what an unlearned character does. Asserted on **both** pools, because the
    /// tempting bug is to mirror the MP path onto HP.
    #[test]
    fn the_warrior_twin_changes_neither_pool() {
        let (mut s, store, id) = with_skill(itemrecovery::IMPROVED_HP_RECOVERY, 15, 237);
        s.clock_ms = 10_000;
        let out = s.regen_tick(10_000);

        let after = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
        assert_eq!(after.hp, 1 + REGEN_AMOUNT, "no HP regen number exists to apply");
        assert_eq!(after.mp, 1 + REGEN_AMOUNT, "and the HP skill certainly does not touch MP");
        assert!(
            !out[0].what.contains("Improved MP Recovery"),
            "the MP skill did not fire: {}",
            out[0].what
        );
    }

    /// Every level pays what the table says, and level 0 pays nothing. Enumerated rather than
    /// sampled at 15, because the table is constant and a sample cannot tell a working lookup
    /// from one that returns the top row for everything.
    #[test]
    fn every_learned_level_pays_and_level_zero_does_not() {
        for level in 0..=itemrecovery::MAX_LEVEL {
            let (mut s, store, id) = with_skill(itemrecovery::IMPROVED_MP_RECOVERY, level, 500);
            s.clock_ms = 10_000;
            assert!(ticked(&s.regen_tick(10_000)), "level {level}");
            let after =
                store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
            let want = if level == 0 { REGEN_AMOUNT } else { REGEN_AMOUNT + 5 };
            assert_eq!(after.mp, 1 + want, "level {level}: 1% of 500 is 5");
        }
    }

    /// The bonused tick still stops at the maximum rather than putting a number past the end
    /// of the bar into the packet.
    #[test]
    fn the_bonused_tick_is_still_capped_at_the_maximum() {
        let (mut s, store, id) = with_skill(itemrecovery::IMPROVED_MP_RECOVERY, 15, 1000);
        let mut chr = s.claimed_character().unwrap();
        chr.mp = chr.max_mp - 3; // 20 would be owed; 3 are missing
        store.save_character_progress(&chr).unwrap();

        s.clock_ms = 10_000;
        let out = s.regen_tick(10_000);
        let after = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
        assert_eq!(after.mp, 1000, "capped, not 1017");
        let want = net::stats::StatChange {
            hp: Some(1 + REGEN_AMOUNT),
            mp: Some(1000),
            ..Default::default()
        }
        .build();
        assert_eq!(out[0].body, want, "the packet cannot carry a value above the maximum");
    }

    /// **The clock the tooltip states is the clock this module ticks on.**
    ///
    /// The tooltip text is the *only* statement of the period anywhere in this client's data -
    /// `2000000` carries no `time`, no `subTime` and no `dotInterval` - so this reads the
    /// generated file rather than a column, and asserts a positive control first: a check that
    /// silently skips looks exactly like one that passed, and `gm-handbook/` is gitignored.
    #[test]
    fn the_tick_interval_is_the_one_the_tooltip_states() {
        let path = std::path::Path::new("../../gm-handbook/skills.txt");
        if !path.exists() {
            return;
        }
        let text = std::fs::read_to_string(path).expect("the generated file is readable");
        let rows: Vec<&str> =
            text.lines().filter(|l| l.starts_with("2000000,")).collect();
        // The control: the file really does describe this skill, at every level.
        assert_eq!(rows.len(), itemrecovery::MAX_LEVEL as usize, "fifteen level rows for 2000000");

        for row in rows {
            assert!(
                row.contains("Regenerates 1% of Max MP every 10 seconds"),
                "the tooltip changed, and REGEN_EVERY_MS may no longer match it: {row}"
            );
        }
        assert_eq!(
            REGEN_EVERY_MS, 10_000,
            "the tooltip says every 10 seconds and this server must tick on the same clock"
        );
    }

    #[test]
    fn it_stops_at_the_maximum_rather_than_going_over() {
        let (mut s, store) = hurt_session();
        let id = s.claimed_character().unwrap().id;
        let mut chr = s.claimed_character().unwrap();
        chr.hp = chr.max_hp - 3;
        chr.mp = chr.max_mp - 3;
        let (max_hp, max_mp) = (chr.max_hp, chr.max_mp);
        s.store.save_character_progress(&chr).unwrap();

        assert!(ticked(&s.regen_tick(10_000)));
        let after = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
        assert_eq!((after.hp, after.mp), (max_hp, max_mp), "capped, not overshot");
        assert!(s.regen_tick(20_000).is_empty(), "and then it stops");
    }

    /// **The third-job warriors' Improved MP Recovery is FLAT**, not the Magician's percent.
    /// *"Recover 3 additional MP every 10 sec."* at level 1, 22 at 20.
    #[test]
    fn the_third_job_improved_mp_recovery_adds_a_flat_amount_per_tick() {
        let table = std::path::Path::new("../../gm-handbook/skills.txt");
        if !table.exists() {
            return; // python tools/dump_skills.py
        }
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Crusader".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let config = Config {
            set_field_probe: true,
            firstjob: crate::firstjob::CombatTable::load(table),
            ..Config::default()
        };
        let mut s = Session::new(store.clone(), Arc::new(config));
        s.claim_for_character(id);
        assert_eq!(s.mp_regen(id, 500).amount, REGEN_AMOUNT, "the control: no skill, base only");
        store.set_skill_level(id, 1_110_000, 1).unwrap();
        assert_eq!(s.mp_regen(id, 500).amount, REGEN_AMOUNT + 3, "Crusader L1: +3, flat");
        store.set_skill_level(id, 1_110_000, 20).unwrap();
        assert_eq!(s.mp_regen(id, 500).amount, REGEN_AMOUNT + 22, "L20: +22");
        // Both ids are one skill; the White Knight's reads the same.
        store.set_skill_level(id, 1_110_000, 0).unwrap();
        store.set_skill_level(id, 1_210_000, 1).unwrap();
        assert_eq!(s.mp_regen(id, 500).amount, REGEN_AMOUNT + 3);
    }
}

#[cfg(test)]
mod chair_tests {
    use super::tests::ticked;
    use super::*;
    use crate::chairs::Chair;
    use std::collections::HashMap;
    use std::sync::Arc;
    use store::Store;

    /// A hurt, claimed session already sitting on `item_id`, with the real chair table.
    ///
    /// The seat is taken by feeding a real `0x00DB` body through `on_chair_sit` rather than by
    /// setting the field, so these tests exercise the decode and the lookup too. Anything less
    /// would pass with the dispatch arm unwired, which is the "built is not wired" failure this
    /// repo keeps paying for.
    fn seated_session(item_id: u32) -> (Session, Arc<Store>) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Sitter".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();

        // The three rows this file needs, transcribed from gm-handbook/chairs.txt.
        let mut chairs = HashMap::new();
        chairs.insert(3_010_005, Chair { recovery_hp: 30, recovery_mp: 0, req_level: 5 });
        chairs.insert(3_010_008, Chair { recovery_hp: 0, recovery_mp: 10, req_level: 0 });
        let config = Config { set_field_probe: true, chairs, ..Config::default() };

        let mut s = Session::new(store.clone(), Arc::new(config));
        assert!(s.claim_for_character(id).contains("claimed the migration"));
        let mut c = s.claimed_character().unwrap();
        c.max_hp = 500;
        c.max_mp = 500;
        c.hp = 1;
        c.mp = 1;
        store.save_character_progress(&c).unwrap();

        // A real sit body: u32 tick, u32 item id, u32 slot, then the tail we do not decode.
        let mut body = Vec::new();
        body.extend_from_slice(&0x055D_4A80u32.to_le_bytes());
        body.extend_from_slice(&item_id.to_le_bytes());
        body.extend_from_slice(&4u32.to_le_bytes());
        body.extend_from_slice(&[0u8; 13]);
        let replies = s.on_chair_sit(&body);
        // The unlock must still go out or the client could never ask to stand.
        assert!(
            replies.iter().any(|r| r.opcode == net::combat::STAT_CHANGED),
            "sitting must still clear the exclusive-request latch"
        );
        (s, store)
    }

    fn hp_mp(store: &Store, id: u32) -> (u32, u32) {
        let c = store.characters_for(1, 0).unwrap().into_iter().find(|c| c.id == id).unwrap();
        (c.hp, c.mp)
    }

    /// The bug the owner reported: they sat on the Red Chair and the tick did not change.
    #[test]
    fn sitting_on_the_red_chair_adds_its_thirty_hp_to_the_tick() {
        let (mut s, store) = seated_session(3_010_005);
        let id = s.claimed_character().unwrap().id;
        s.clock_ms = 10_000;
        assert!(ticked(&s.regen_tick(10_000)), "a tick was due");
        let (hp, _) = hp_mp(&store, id);
        // The tooltip says "an ADDITIONAL 30 HP every 10 seconds", so it adds to the flat 10.
        assert_eq!(hp, 1 + REGEN_AMOUNT + 30);
    }

    /// The Blue Seal Cushion restores MP and no HP at all. A loader that defaulted its missing
    /// `recoveryHP` to the common 30 would pass the test above and fail this one.
    #[test]
    fn the_blue_seal_cushion_adds_mp_only() {
        let (mut s, store) = seated_session(3_010_008);
        let id = s.claimed_character().unwrap().id;
        s.clock_ms = 10_000;
        assert!(ticked(&s.regen_tick(10_000)));
        let (hp, mp) = hp_mp(&store, id);
        assert_eq!(hp, 1 + REGEN_AMOUNT, "this chair restores no HP");
        assert_eq!(mp, 1 + REGEN_AMOUNT + 10, "and 10 MP on top of the flat MP");
    }

    /// Standing up ends the bonus on the very next tick, even though the client stays seated
    /// on screen - that half is not ours to fix yet.
    #[test]
    fn standing_up_ends_the_bonus() {
        let (mut s, store) = seated_session(3_010_005);
        let id = s.claimed_character().unwrap().id;
        let replies = s.on_chair_cancel(&[0xff, 0xff]);
        assert!(
            replies.iter().any(|r| r.opcode == net::combat::STAT_CHANGED),
            "standing must still clear the latch"
        );
        s.clock_ms = 10_000;
        assert!(ticked(&s.regen_tick(10_000)));
        let (hp, _) = hp_mp(&store, id);
        assert_eq!(hp, 1 + REGEN_AMOUNT, "back to the flat base");
    }

    /// An id with no row is unknown, not "a chair that restores nothing", and either way the
    /// tick must not move. This is the clean-checkout case: `gm-handbook/` never generated.
    #[test]
    fn a_chair_we_have_no_row_for_changes_nothing() {
        let (mut s, store) = seated_session(9_999_999);
        let id = s.claimed_character().unwrap().id;
        s.clock_ms = 10_000;
        assert!(ticked(&s.regen_tick(10_000)));
        let (hp, mp) = hp_mp(&store, id);
        assert_eq!((hp, mp), (1 + REGEN_AMOUNT, 1 + REGEN_AMOUNT));
    }
}
