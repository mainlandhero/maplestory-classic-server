//! `!exprate` and `!mesorate`, and the scrolling banner that announces them.
//!
//! # "Broadcast to all players" without a broadcast
//!
//! There is no connection registry on this server. A channel is a process, each connection
//! is a thread with its own socket, and nothing can reach into another thread to push a
//! packet - `crates/world/src/server.rs`.
//!
//! So the banner is not pushed. **Every session works out for itself what should be on
//! screen right now**, from state that all of them share: the `server_rates` table. Both
//! channel processes read the same SQLite file, so they reach the same answer at the same
//! moment, and a player who logs in halfway through an event sees the banner on their first
//! tick rather than at the next cycle. That is the same result a broadcast would produce,
//! and it needs no registry.
//!
//! The cost is a tiny query per tick per connection - one row per rate, twice a second.
//!
//! # The cycle, and what it is anchored to
//!
//! The owner: *"displayed for 2 minutes every 5 minutes until the EXP/Meso rate has been returned
//! to normal"*, and *"whenever the EXP or the Meso rate is changed, a scrolling message will
//! be broadcasted"*. Those two sentences are one rule if the cycle is anchored to the
//! moment of the change: at `set_at` the banner goes up, it stays up for two minutes, it
//! comes down for three, and it repeats.
//!
//! The anchor is a single timestamp for **both** rates, not one each - see
//! [`store::rates::Rates::set_at`]. Changing either rate restarts the cycle for both, which
//! is what makes "if both rates are set to non-1x, the scrolling text should include both
//! messages" land as one banner rather than two fighting over the same singleton.
//!
//! # Why nothing re-sends the banner it already sent
//!
//! `net::broadcast`'s `case 4` resets the client's banner object **before** it looks at the
//! flag, so sending the same string again visibly restarts the scroll. Each session
//! remembers what it last put on screen and only sends when the answer changes.
//!
//! **[I], and worth one line of a test run:** whether the banner survives a field change. If
//! the client tears its UI down on `SetField`, an event that started before a `!map` would
//! vanish until the next five-minute cycle. Re-asserting it on every field entry would fix
//! that and would *also* restart the scroll on every map change, which is a visible wrong
//! answer in the far more common case - so this does nothing, deliberately, until a run says
//! which way it actually behaves.

use store::rates::{Rate, RateKind, Rates};

use super::*;

/// How often the banner comes back: five minutes.
pub(super) const BANNER_CYCLE_SECS: i64 = 300;

/// How long it stays up each time: two minutes.
pub(super) const BANNER_VISIBLE_SECS: i64 = 120;

impl Session {
    /// `!exprate <multiplier>` - the server's experience rate.
    pub(super) fn gm_exp_rate(&mut self, arg: &str) -> Vec<Reply> {
        self.gm_rate(RateKind::Exp, "exprate", arg)
    }

    /// `!mesorate <multiplier>` - the server's meso rate.
    pub(super) fn gm_meso_rate(&mut self, arg: &str) -> Vec<Reply> {
        self.gm_rate(RateKind::Meso, "mesorate", arg)
    }

    /// Both commands. They differ only in which row they write.
    fn gm_rate(&mut self, kind: RateKind, command: &str, arg: &str) -> Vec<Reply> {
        let current = match self.store.rates() {
            Ok(r) => r,
            Err(e) => return self.gm_ack(format!("!{command}: could not read the rates: {e}")),
        };
        // No argument reports rather than refusing. Reading the rate back is the only way to
        // tell "the multiplier is applied" apart from "the multiplier was never stored", and
        // those two look identical from inside the game.
        if arg.is_empty() {
            return self.gm_ack(format!(
                "!{command}: EXP is {}x and mesos are {}x. Set one with !{command} 2, or \
                 !{command} 1 to put it back to normal.",
                current.exp, current.meso
            ));
        }
        let rate = match Rate::parse(arg) {
            Ok(r) => r,
            Err(e) => return self.gm_ack(format!("!{command}: {e}")),
        };
        if rate == current.get(kind) {
            // Storing it anyway would restart the five-minute cycle and re-show a banner
            // that is already saying the right thing.
            return self.gm_ack(format!(
                "!{command}: the {} rate is already {rate}x. Nothing changed.",
                kind.label()
            ));
        }
        let now = now_unix();
        if let Err(e) = self.store.set_rate(kind, rate, now) {
            return self.gm_ack(format!("!{command}: could not save the rate: {e}"));
        }
        let after = self.store.rates().unwrap_or(current);
        let mut out = self.gm_ack(if rate.is_normal() {
            format!(
                "!{command}: the {} rate is back to normal (1x).{}",
                kind.label(),
                if after.all_normal() {
                    " The banner is coming down."
                } else {
                    " The banner keeps running for the other rate."
                }
            )
        } else {
            format!(
                "!{command}: the {} rate is now {rate}x, for everyone on every channel. The \
                 banner goes up now and comes back for 2 minutes out of every 5 until it is \
                 1x again.",
                kind.label()
            )
        });
        // Straight away, not on the next tick: the change is supposed to announce itself.
        out.extend(self.banner_replies(now));
        out
    }

    /// The banner as it should be right now. [`Session::tick`]'s entry point.
    ///
    /// The clock is read here rather than taken from `tick`, which is handed milliseconds
    /// since this connection opened. Two players who logged in ten minutes apart must see
    /// the same banner at the same moment, and a per-connection clock cannot do that.
    pub(super) fn banner_tick(&mut self) -> Vec<Reply> {
        let now = now_unix();
        self.banner_replies(now)
    }

    /// The EXP multiplier as it stands, or 1x if the table cannot be read.
    ///
    /// A database error must not silently zero somebody's experience, so the fallback is
    /// the normal rate rather than nothing.
    pub(super) fn exp_rate(&self) -> Rate {
        self.store.rates().map(|r| r.exp).unwrap_or(Rate::NORMAL)
    }

    /// The meso multiplier as it stands, or 1x if the table cannot be read.
    pub(super) fn meso_rate(&self) -> Rate {
        self.store.rates().map(|r| r.meso).unwrap_or(Rate::NORMAL)
    }

    /// The banner packet this connection owes, if the answer has changed since last time.
    ///
    /// At most one packet, and usually none.
    pub(super) fn banner_replies(&mut self, now: i64) -> Vec<Reply> {
        let Ok(rates) = self.store.rates() else {
            // A database error is not worth tearing the banner down over; leave the screen
            // as it is and try again on the next tick.
            return Vec::new();
        };
        let want = banner_text(&rates, now);
        if want == self.banner_shown {
            return Vec::new();
        }
        self.banner_shown = want.clone();
        match want {
            Some(text) => vec![Reply {
                opcode: net::broadcast::BROADCAST_MSG,
                body: net::broadcast::banner(&text),
                what: format!("BroadcastMsg type 4: banner UP - {text}"),
            }],
            None => vec![Reply {
                opcode: net::broadcast::BROADCAST_MSG,
                body: net::broadcast::clear_banner(),
                what: "BroadcastMsg type 4, flag 0: banner DOWN".to_string(),
            }],
        }
    }
}

/// What the banner should say at `now`, or `None` if it should not be on screen.
///
/// Pure, and separate from the session on purpose: the whole schedule is testable against a
/// clock a test controls, which a `SystemTime::now()` buried in a method would not be.
pub(super) fn banner_text(rates: &Rates, now: i64) -> Option<String> {
    if rates.all_normal() {
        return None;
    }
    // `max(0)` rather than `rem_euclid`: if the clock has gone backwards since the rate was
    // set, the honest reading is "this was just set", which shows the banner. Taking the
    // remainder of a negative number would hide it for most of a cycle instead.
    let elapsed = (now - rates.set_at).max(0);
    if elapsed % BANNER_CYCLE_SECS >= BANNER_VISIBLE_SECS {
        return None;
    }
    let mut parts = Vec::new();
    for kind in [RateKind::Exp, RateKind::Meso] {
        let rate = rates.get(kind);
        if !rate.is_normal() {
            parts.push(sentence(kind, rate));
        }
    }
    Some(parts.join(" "))
}

/// One rate's sentence, exactly as the owner wrote it.
fn sentence(kind: RateKind, rate: Rate) -> String {
    format!("[Event] The Server's {} rate has been set to {rate}x", kind.label())
}

/// Unix seconds. The banner's cycle has to be wall-clock: `Session::tick` is handed
/// milliseconds since **this connection** opened, so two players who logged in at different
/// times would otherwise scroll their banners at different moments.
fn now_unix() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rates(exp: u32, meso: u32, set_at: i64) -> Rates {
        Rates { exp: Rate::from_per_cent(exp), meso: Rate::from_per_cent(meso), set_at }
    }

    #[test]
    fn a_normal_server_shows_nothing() {
        assert_eq!(banner_text(&Rates::NORMAL, 0), None);
        assert_eq!(banner_text(&rates(100, 100, 5_000), 5_000), None);
    }

    #[test]
    fn the_wording_is_the_wording_that_was_asked_for() {
        assert_eq!(
            banner_text(&rates(200, 100, 1_000), 1_000).unwrap(),
            "[Event] The Server's EXP rate has been set to 2x"
        );
        assert_eq!(
            banner_text(&rates(100, 300, 1_000), 1_000).unwrap(),
            "[Event] The Server's Meso rate has been set to 3x"
        );
    }

    #[test]
    fn both_rates_share_one_banner() {
        assert_eq!(
            banner_text(&rates(200, 300, 1_000), 1_000).unwrap(),
            "[Event] The Server's EXP rate has been set to 2x \
             [Event] The Server's Meso rate has been set to 3x"
        );
    }

    #[test]
    fn two_minutes_up_and_three_minutes_down() {
        let r = rates(200, 100, 1_000);
        assert!(banner_text(&r, 1_000).is_some(), "up the moment it is set");
        assert!(banner_text(&r, 1_000 + 119).is_some(), "still up at 1:59");
        assert!(banner_text(&r, 1_000 + 120).is_none(), "down at 2:00");
        assert!(banner_text(&r, 1_000 + 299).is_none(), "still down at 4:59");
        assert!(banner_text(&r, 1_000 + 300).is_some(), "back up at 5:00");
        assert!(banner_text(&r, 1_000 + 419).is_some(), "up until 6:59");
        assert!(banner_text(&r, 1_000 + 420).is_none(), "down again at 7:00");
    }

    #[test]
    fn it_keeps_cycling_for_a_long_event() {
        let r = rates(150, 150, 0);
        // Ten hours in, still on the same two-in-five rhythm.
        let ten_hours = 36_000;
        assert!(banner_text(&r, ten_hours).is_some());
        assert!(banner_text(&r, ten_hours + 121).is_none());
    }

    #[test]
    fn a_clock_that_went_backwards_shows_the_banner() {
        // Not a hypothetical worth much, but the alternative reading - hiding it for four
        // fifths of a cycle - is silent, and this one is not.
        assert!(banner_text(&rates(200, 100, 9_000), 8_000).is_some());
    }

    #[test]
    fn fractional_rates_read_naturally() {
        assert_eq!(
            banner_text(&rates(150, 100, 0), 0).unwrap(),
            "[Event] The Server's EXP rate has been set to 1.5x"
        );
    }
}
