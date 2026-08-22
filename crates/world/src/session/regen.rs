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

/// How much each tick restores, to both pools.
///
/// The owner's number. It is flat and it is the same for HP and MP - not a percentage, not
/// class-dependent, and deliberately not derived from anything, because nothing has been
/// measured that would justify a formula. When one is, this is the constant it replaces.
pub(super) const REGEN_AMOUNT: u32 = 10;

impl Session {
    /// Remember that the player just did something. Called from every packet that proves it.
    pub(super) fn note_activity(&mut self) {
        self.last_activity_ms = self.clock_ms;
        // Restart the countdown, rather than leaving a tick that was already due to fire the
        // instant they stop. Without this, running around for a minute and then pausing pays
        // out immediately instead of after ten seconds.
        self.next_regen_ms = None;
    }

    /// The regeneration this tick owes, if any. At most one `0x007C`.
    pub(super) fn regen_tick(&mut self, now_ms: u64) -> Vec<Reply> {
        if now_ms.saturating_sub(self.last_activity_ms) < IDLE_AFTER_MS {
            return Vec::new();
        }
        let Some(mut chr) = self.claimed_character() else { return Vec::new() };
        // Nothing to restore: do not send, and do not arm a timer. A player who sits at full
        // health for an hour should cost exactly nothing.
        if chr.hp >= chr.max_hp && chr.mp >= chr.max_mp {
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

        let hp = chr.hp.saturating_add(REGEN_AMOUNT).min(chr.max_hp);
        let mp = chr.mp.saturating_add(REGEN_AMOUNT).min(chr.max_mp);
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
        let mut out = vec![Reply {
            opcode: net::stats::STAT_CHANGED,
            body: net::stats::StatChange {
                hp: Some(chr.hp),
                mp: Some(chr.mp),
                ..Default::default()
            }
            .build(),
            what: format!(
                "StatChanged: idle regen +{healed_hp} hp +{healed_mp} mp -> {}/{} hp, {}/{} mp",
                chr.hp, chr.max_hp, chr.mp, chr.max_mp
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
    fn ticked(out: &[Reply]) -> bool {
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
}
