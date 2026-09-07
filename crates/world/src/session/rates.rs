//! `!exprate`, `!mesorate`, `!droprate`, `!rates`, and the scrolling banner.
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
//! The cost is a tiny query per tick per connection - three rows, twice a second.
//!
//! # The cycle, and what it is anchored to
//!
//! The owner: *"displayed for 2 minutes every 5 minutes until the EXP/Meso rate has been returned
//! to normal"*, and *"whenever the EXP or the Meso rate is changed, a scrolling message will
//! be broadcasted"*. Those two sentences are one rule if the cycle is anchored to the moment
//! of the change: at the anchor the banner goes up, it stays up for two minutes, it comes
//! down for three, and it repeats.
//!
//! **The rate itself never expires.** `!exprate 2` is 2x until somebody types `!exprate 1`.
//! Only the banner cycles. That was worth writing down because the summary of it read as
//! though the rate reset itself, and it does not.
//!
//! The anchor is a single timestamp across all three rates, not one each -
//! [`store::rates::Rates::anchor`]. Changing any rate restarts the cycle for all of them,
//! which is what makes several simultaneous events share one banner rather than fight over
//! the client's single banner object.
//!
//! # Endings are messages too
//!
//! The owner, 2026-08-20: *"When either EXP or Meso is set back to 1x again, you should also
//! immediately display a scrolling notice."* So a rate contributes a sentence when it is
//! running **and** for [`BANNER_VISIBLE_SECS`] after it stops. The two cases share the cycle
//! and the packet; only the wording differs. An ending is `ended_at` in the store, cleared
//! when a new event starts so a stale notice cannot reappear.
//!
//! An ending expires after exactly one visible window, which falls out of the arithmetic
//! rather than needing its own rule: the window and the ending are both
//! [`BANNER_VISIBLE_SECS`], so by the time the cycle would show the banner again the ending
//! has nothing left to say and the banner stays down.
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

use store::rates::{Rate, RateKind, Rates, ALL_KINDS};

use super::*;

/// How often the banner comes back: five minutes.
pub(super) const BANNER_CYCLE_SECS: i64 = 300;

/// How long it stays up each time, and how long an ending is worth announcing: two minutes.
pub(super) const BANNER_VISIBLE_SECS: i64 = 120;

/// Why a rate the player typed cannot be used, worded to be shown as-is.
///
/// **Every rate command refuses below 1x.** The owner, 2026-08-20: *"Make sure that the
/// number/decimal has to be greater than 1"*, then *"it should accept 1 as well"*, then
/// *"The individual rate setters should also behave the same way"*. So the floor is
/// inclusive and it belongs to the commands, not to `!setrates` alone.
///
/// **The floor is NOT in [`Rate`].** The type still represents 0.01x upwards, because the
/// drop rate multiplies a chance and the arithmetic has no business caring what a chat
/// command will accept. This is a policy about what a person is allowed to type, and it lives
/// at that boundary - one function, so the three setters and the combined one cannot drift.
fn below_normal(kind: RateKind, rate: Rate) -> Option<String> {
    if rate >= Rate::NORMAL {
        return None;
    }
    Some(format!(
        "the {} rate of {rate}x is below 1x, and a rate below 1x makes the game worse rather than better. 1 is the lowest these commands take, and it means normal.",
        kind.label()
    ))
}

impl Session {
    /// `!rates` - what the server is running at.
    ///
    /// **The owner asked for this as a "non-privileged" command, and on this server that
    /// distinction is documentary rather than enforced**: nothing authenticates, there is no
    /// permission check anywhere in `!`-command handling, and adding one would be theatre.
    /// What it really means is that this one is for players rather than for testing - it
    /// changes nothing, so it is safe to leave available when the setters eventually are not.
    pub(super) fn gm_rates(&mut self) -> Vec<Reply> {
        let rates = match self.store.rates() {
            Ok(r) => r,
            Err(e) => return self.gm_ack(format!("!rates: could not read the rates: {e}")),
        };
        let listed: Vec<String> = ALL_KINDS
            .iter()
            .map(|k| format!("{} {}x", k.label(), rates.get(*k)))
            .collect();
        let mut said = format!("Server rates: {}.", listed.join(", "));
        if rates.all_normal() {
            said.push_str(" No event is running.");
        }
        self.gm_ack(said)
    }

    /// `!setrates <exp> <meso> <drop>` - all three at once, on one timestamp.
    ///
    /// The owner asked for it to avoid running three commands, and **the single timestamp is the
    /// substantive part**, not the typing saved. Three separate commands re-anchor the
    /// banner's cycle three times, so the first two events are announced and then immediately
    /// replaced; one call anchors once and the banner names all three together from the
    /// start.
    ///
    /// **Every value must be at least 1x**, the same rule the individual setters follow -
    /// see [`below_normal`]. So `!setrates 1 1 1` is the way to end everything in one command,
    /// which is the natural counterpart to a command whose whole point is not typing three,
    /// and `!setrates 0.5 1 1` is refused.
    pub(super) fn gm_set_rates(&mut self, arg: &str) -> Vec<Reply> {
        let words: Vec<&str> = arg.split_whitespace().collect();
        if words.len() != ALL_KINDS.len() {
            return self.gm_ack(format!(
                "!setrates wants {} multipliers - EXP, then Meso, then Drop. Try !setrates 2 3 5, or !setrates 1 1 1 to end everything.",
                ALL_KINDS.len()
            ));
        }
        // Parse and validate ALL of them before writing ANY of them. A partial application
        // would leave the server on a combination nobody asked for, and the player would have
        // to work out which of the three had taken.
        let mut wanted = Vec::new();
        for (kind, word) in ALL_KINDS.iter().zip(&words) {
            let rate = match Rate::parse(word) {
                Ok(r) => r,
                Err(e) => return self.gm_ack(format!("!setrates: the {} rate: {e}", kind.label())),
            };
            if let Some(why) = below_normal(*kind, rate) {
                return self.gm_ack(format!("!setrates: {why}"));
            }
            wanted.push((*kind, rate));
        }

        // **Only the kinds that actually change are written.** Since 2026-09-06 this is the
        // one rate command (the per-kind setters are gone on the owner's instruction), so it
        // inherits their rule: storing an unchanged rate would restart its five-minute banner
        // cycle and re-announce an event that is already announced - and stamping a still-
        // normal rate with a fresh time would announce the END of an event that never ran.
        let current = match self.store.rates() {
            Ok(r) => r,
            Err(e) => return self.gm_ack(format!("!setrates: could not read the rates: {e}")),
        };
        let listed: Vec<String> =
            wanted.iter().map(|(k, r)| format!("{} {r}x", k.label())).collect();
        let changed: Vec<(RateKind, Rate)> =
            wanted.iter().copied().filter(|(k, r)| *r != current.get(*k)).collect();
        if changed.is_empty() {
            return self.gm_ack(format!("!setrates: already {}. Nothing changed.", listed.join(", ")));
        }
        // One timestamp for everything that moved, so `Rates::anchor` is a single moment and
        // the banner shows the whole event rather than the last third of it.
        let now = now_unix();
        for (kind, rate) in &changed {
            if let Err(e) = self.store.set_rate(*kind, *rate, now) {
                return self.gm_ack(format!("!setrates: could not save the {} rate: {e}", kind.label()));
            }
        }
        let mut out = self.gm_ack(format!("!setrates: {}.", listed.join(", ")));
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

    /// One rate as it stands, or 1x if the table cannot be read.
    ///
    /// A database error must not silently zero somebody's rewards, so the fallback is the
    /// normal rate rather than nothing.
    pub(super) fn rate(&self, kind: RateKind) -> Rate {
        self.store.rates().map(|r| r.get(kind)).unwrap_or(Rate::NORMAL)
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
    // `max(0)` rather than `rem_euclid`: if the clock has gone backwards since the anchor,
    // the honest reading is "this was just set", which shows the banner. Taking the remainder
    // of a negative number would hide it for most of a cycle instead.
    let elapsed = (now - rates.anchor()).max(0);
    if elapsed % BANNER_CYCLE_SECS >= BANNER_VISIBLE_SECS {
        return None;
    }
    let mut parts = Vec::new();
    for kind in ALL_KINDS {
        let row = rates.row(kind);
        if !row.rate.is_normal() {
            parts.push(running(kind, row.rate));
        } else if row.ended_at > 0 && now - row.ended_at < BANNER_VISIBLE_SECS {
            parts.push(ended(kind));
        }
    }
    if parts.is_empty() {
        return None;
    }
    Some(parts.join(" "))
}

/// A running event's sentence, exactly as the owner wrote it.
fn running(kind: RateKind, rate: Rate) -> String {
    format!("[Event] The Server's {} rate has been set to {rate}x", kind.label())
}

/// A finished event's sentence, exactly as the owner wrote it - including the full stop, which the
/// running one does not have.
fn ended(kind: RateKind) -> String {
    format!("[Event] The {} rate-up event has ended.", kind.label())
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
    use store::rates::RateRow;

    fn running_at(exp: u32, meso: u32, drop: u32, set_at: i64) -> Rates {
        let row = |p: u32| RateRow { rate: Rate::from_per_cent(p), set_at, ended_at: 0 };
        Rates { exp: row(exp), meso: row(meso), drop: row(drop) }
    }

    #[test]
    fn a_normal_server_shows_nothing() {
        assert_eq!(banner_text(&Rates::NORMAL, 0), None);
        assert_eq!(banner_text(&running_at(100, 100, 100, 5_000), 5_000), None);
    }

    #[test]
    fn the_wording_is_the_wording_that_was_asked_for() {
        assert_eq!(
            banner_text(&running_at(200, 100, 100, 1_000), 1_000).unwrap(),
            "[Event] The Server's EXP rate has been set to 2x"
        );
        assert_eq!(
            banner_text(&running_at(100, 300, 100, 1_000), 1_000).unwrap(),
            "[Event] The Server's Meso rate has been set to 3x"
        );
        assert_eq!(
            banner_text(&running_at(100, 100, 400, 1_000), 1_000).unwrap(),
            "[Event] The Server's Drop rate has been set to 4x"
        );
    }

    #[test]
    fn all_three_share_one_banner_in_a_fixed_order() {
        let text = banner_text(&running_at(200, 300, 400, 1_000), 1_000).unwrap();
        assert_eq!(
            text,
            "[Event] The Server's EXP rate has been set to 2x [Event] The Server's Meso rate has been set to 3x [Event] The Server's Drop rate has been set to 4x"
        );
    }

    #[test]
    fn an_ending_is_announced_and_expires_after_one_window() {
        let mut r = Rates::NORMAL;
        r.exp = RateRow { rate: Rate::NORMAL, set_at: 1_000, ended_at: 1_000 };
        assert_eq!(
            banner_text(&r, 1_000).unwrap(),
            "[Event] The EXP rate-up event has ended."
        );
        assert!(banner_text(&r, 1_000 + 119).is_some(), "still up at 1:59");
        assert!(banner_text(&r, 1_000 + 120).is_none(), "gone at 2:00");
        // ...and it does not come back with the next cycle, because the ending has expired.
        assert!(banner_text(&r, 1_000 + 300).is_none(), "and does NOT return at 5:00");
    }

    #[test]
    fn an_ending_rides_alongside_an_event_that_is_still_running() {
        let mut r = running_at(100, 300, 100, 2_000);
        r.exp = RateRow { rate: Rate::NORMAL, set_at: 2_000, ended_at: 2_000 };
        assert_eq!(
            banner_text(&r, 2_000).unwrap(),
            "[Event] The EXP rate-up event has ended. [Event] The Server's Meso rate has been set to 3x"
        );
        // Once the ending expires the surviving event keeps its own cycle, alone.
        assert_eq!(
            banner_text(&r, 2_000 + 300).unwrap(),
            "[Event] The Server's Meso rate has been set to 3x"
        );
    }

    #[test]
    fn an_ending_that_never_happened_is_not_announced() {
        // ended_at of 0 is "no event has ever finished on this rate", and unix time 0 is not
        // a plausible ending. Without the guard, every fresh server would announce three.
        let r = Rates::NORMAL;
        assert_eq!(banner_text(&r, 0), None);
        assert_eq!(banner_text(&r, 60), None);
    }

    #[test]
    fn two_minutes_up_and_three_minutes_down() {
        let r = running_at(200, 100, 100, 1_000);
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
        let r = running_at(150, 150, 150, 0);
        let ten_hours = 36_000;
        assert!(banner_text(&r, ten_hours).is_some());
        assert!(banner_text(&r, ten_hours + 121).is_none());
    }

    #[test]
    fn a_clock_that_went_backwards_shows_the_banner() {
        // Not a hypothetical worth much, but the alternative reading - hiding it for four
        // fifths of a cycle - is silent, and this one is not.
        assert!(banner_text(&running_at(200, 100, 100, 9_000), 8_000).is_some());
    }

    #[test]
    fn fractional_rates_read_naturally() {
        assert_eq!(
            banner_text(&running_at(150, 100, 100, 0), 0).unwrap(),
            "[Event] The Server's EXP rate has been set to 1.5x"
        );
    }
}
