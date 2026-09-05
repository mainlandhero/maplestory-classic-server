//! **A failure budget for guessing codes**, per peer and for the whole service.
//!
//! `store::codes` shrank from 16 characters to 8 on 2026-09-05 at the owner's instruction, which
//! took a code from ~78 bits to ~39 bits (`30^8 = 6.6e11`). Its own doc said what that would
//! mean: *"this module does no rate limiting. The margin is what makes the missing rate limit
//! harmless, so shortening a code for convenience later is not a cosmetic change."* This is
//! the rate limit that shortening it required.
//!
//! # What it bounds
//!
//! Only **failed** code redemptions count - a wrong or spent code at `/register` or
//! `/recover`. Successful redemptions and every other endpoint are untouched, so a player who
//! mistypes twice is not locked out of signing in.
//!
//! Two budgets, both over a sliding window:
//!
//! * **per peer address** - the ordinary case: one machine guessing;
//! * **across every peer** - so guessing from many addresses buys nothing.
//!
//! With the defaults (10 per peer, 200 overall, per 15 minutes) the service accepts at most
//! `200 / 900 s` wrong codes per second from the entire internet. Half the space of one code is
//! `3.3e11` guesses, which at that rate is about **47,000 years**. Without the limit, at a
//! thousand guesses per second over TLS, it would be ten years - still longer than any code
//! lives, but that is the wrong kind of margin to rely on.
//!
//! # What it is not
//!
//! Not persistent (a restart forgets), not shared between processes, and not a defence against
//! anything but online guessing. A peer behind a large NAT shares its budget with everyone
//! behind that NAT; the global budget is what stops that from being a way round the per-peer
//! one.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct Limiter {
    per_peer: u32,
    global: u32,
    window: Duration,
    state: Mutex<State>,
}

#[derive(Default)]
struct State {
    peers: HashMap<String, Vec<Instant>>,
    all: Vec<Instant>,
}

impl Limiter {
    pub const DEFAULT_PER_PEER: u32 = 10;
    pub const DEFAULT_GLOBAL: u32 = 200;
    pub const DEFAULT_WINDOW: Duration = Duration::from_secs(15 * 60);

    pub fn new(per_peer: u32, global: u32, window: Duration) -> Self {
        Limiter { per_peer, global, window, state: Mutex::new(State::default()) }
    }

    /// May this peer attempt a code right now?
    pub fn allowed(&self, peer: &str) -> bool {
        self.allowed_at(peer, Instant::now())
    }

    /// Record one failed attempt by this peer.
    pub fn failed(&self, peer: &str) {
        self.failed_at(peer, Instant::now())
    }

    /// [`Limiter::allowed`] with the clock supplied, so the window is testable without sleeping.
    pub fn allowed_at(&self, peer: &str, now: Instant) -> bool {
        let mut state = self.state.lock().unwrap_or_else(|p| p.into_inner());
        self.prune(&mut state, now);
        let mine = state.peers.get(peer).map(Vec::len).unwrap_or(0);
        (mine as u32) < self.per_peer && (state.all.len() as u32) < self.global
    }

    pub fn failed_at(&self, peer: &str, now: Instant) {
        let mut state = self.state.lock().unwrap_or_else(|p| p.into_inner());
        self.prune(&mut state, now);
        state.peers.entry(peer.to_string()).or_default().push(now);
        state.all.push(now);
    }

    /// Forget failures older than the window. Also drops peers with none left, so the map
    /// cannot grow without bound under a long scan from many addresses.
    fn prune(&self, state: &mut State, now: Instant) {
        let window = self.window;
        let fresh = |t: &Instant| now.saturating_duration_since(*t) < window;
        state.all.retain(fresh);
        state.peers.retain(|_, times| {
            times.retain(fresh);
            !times.is_empty()
        });
    }
}

impl Default for Limiter {
    fn default() -> Self {
        Limiter::new(Self::DEFAULT_PER_PEER, Self::DEFAULT_GLOBAL, Self::DEFAULT_WINDOW)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_peer_is_refused_after_its_budget_and_another_peer_is_not() {
        let l = Limiter::new(3, 100, Duration::from_secs(60));
        let t0 = Instant::now();
        for _ in 0..3 {
            assert!(l.allowed_at("a", t0));
            l.failed_at("a", t0);
        }
        assert!(!l.allowed_at("a", t0), "the fourth attempt from the same peer is refused");
        assert!(l.allowed_at("b", t0), "a different peer has its own budget");
    }

    #[test]
    fn the_global_budget_refuses_everybody_once_spent() {
        let l = Limiter::new(100, 5, Duration::from_secs(60));
        let t0 = Instant::now();
        for i in 0..5 {
            l.failed_at(&format!("peer{i}"), t0);
        }
        assert!(!l.allowed_at("someone-new", t0), "many addresses do not buy more guesses");
    }

    #[test]
    fn failures_expire_with_the_window() {
        let l = Limiter::new(1, 1, Duration::from_secs(60));
        let t0 = Instant::now();
        l.failed_at("a", t0);
        assert!(!l.allowed_at("a", t0));
        assert!(!l.allowed_at("a", t0 + Duration::from_secs(59)), "still inside the window");
        assert!(l.allowed_at("a", t0 + Duration::from_secs(61)), "and forgiven after it");
    }

    #[test]
    fn successes_are_not_counted_because_nothing_records_them() {
        // There is no `succeeded` method to call by mistake: the API can only count failures.
        let l = Limiter::new(1, 1, Duration::from_secs(60));
        assert!(l.allowed("a"));
        assert!(l.allowed("a"));
    }
}
