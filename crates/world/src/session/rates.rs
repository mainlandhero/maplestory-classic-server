//! `!rates`, `!setrates`, `!announce`, and the scrolling banner.
//!
//! # The banner says what a GM typed, and nothing else
//!
//! The owner, 2026-10-03: *"Remove the scrolling text at the top announcing the server's rates
//! on a cadence. The text should now be configurable with a GM command called !announce
//! <message> which will start that scrolling text. Players can check the EXP rates using the
//! public !rate command anyways"*, then *"The scrolling text should not start if there is no
//! configured message. The configured message should persist across server restarts."*
//!
//! So the rate events no longer write the banner. Until that day `!setrates` put
//! `[Event] The Server's EXP rate has been set to 2x` up for two minutes in every five, and an
//! "event has ended" line after it. **That is gone**; `!rates` (alias `!rate`) answers anyone
//! who asks.
//!
//! The text lives in the database (`store::announcement`), one row or none: `!announce <text>`
//! sets it, a bare `!announce` clears it. In the database because a channel is a process and
//! the SQLite file is the one thing both share, and it survives a restart because the file does.
//!
//! # "Broadcast to all players" without a broadcast
//!
//! The banner is not pushed. **Every session works out for itself what should be on screen**
//! from that row, on every tick, and sends only when its own answer changes - so a player who
//! logs in sees it on their first tick, on either channel, and a change reaches everyone within
//! a tick. The cost is a one-row query per tick per connection.
//!
//! # Why nothing re-sends the banner it already sent
//!
//! `net::broadcast`'s `case 4` resets the client's banner object **before** it looks at the
//! flag, so sending the same string again visibly restarts the scroll. Each session
//! remembers what it last put on screen (`Session::banner_shown`) and only sends when the
//! answer changes. With nothing configured it sends nothing at all - not even a teardown,
//! unless it had put something up.
//!
//! **[I], and worth one line of a test run:** whether the banner survives a field change. If
//! the client tears its UI down on `SetField`, the banner would vanish after a `!map` until it
//! next changes. Re-asserting it on every field entry would restart the scroll on every map
//! change, so this does nothing, deliberately, until a run says which way it behaves.

use store::rates::{Rate, RateKind, Rates, ALL_KINDS};

use super::*;

/// The longest banner `!announce` takes. The client's chat box stops well short of it; this
/// only keeps a pasted wall of text from reaching every screen.
pub(super) const ANNOUNCE_MAX_CHARS: usize = 200;

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
        let mut said = format!("Server rates: {}.", listed(&rates).join(", "));
        if rates.all_normal() {
            said.push_str(" No event is running.");
        }
        self.gm_ack(said)
    }

    /// `!setrates <exp> <meso> <drop>` - all three at once, on one timestamp.
    ///
    /// The owner asked for it to avoid running three commands; everything that moves is stamped
    /// with one timestamp. **It announces nothing** since 2026-10-03 - players ask `!rates`, and a
    /// GM who wants it scrolled says so with `!announce`.
    ///
    /// **Every value must be at least 1x**, the same rule the individual setters follow -
    /// see [`below_normal`]. So `!setrates 1 1 1` is the way to end everything in one command,
    /// which is the natural counterpart to a command whose whole point is not typing three,
    /// and `!setrates 0.5 1 1` is refused.
    ///
    /// **Five fields since 2026-09-06** - the owner: *"!setrates <exp> <meso> <drop> <quest>
    /// <party%>"*. The fourth is a multiplier like the first three and applies to the EXP a
    /// quest completion pays. The fifth is **not a multiplier**: it is the percent of a kill's
    /// EXP that every other party member on the field receives as their own copy - *"30%
    /// split copy for party member means killer 70 EXP, party mem 2-6 30 EXP each"* - parsed
    /// as a whole percent from 0 to 100.
    pub(super) fn gm_set_rates(&mut self, arg: &str) -> Vec<Reply> {
        let words: Vec<&str> = arg.split_whitespace().collect();
        if words.len() != ALL_KINDS.len() {
            return self.gm_ack(format!(
                "!setrates wants {} fields - <exp> <meso> <drop> <quest> <party%>: four multipliers (1 or above; 1 is normal), then the percent of a kill each other party member on the map receives (0-100). Try !setrates 2 3 5 1 30, or !setrates 1 1 1 1 30 to end every event.",
                ALL_KINDS.len()
            ));
        }
        // Parse and validate ALL of them before writing ANY of them. A partial application
        // would leave the server on a combination nobody asked for, and the player would have
        // to work out which of the five had taken.
        let mut wanted = Vec::new();
        for (kind, word) in ALL_KINDS.iter().zip(&words) {
            let rate = if kind.is_event() {
                let rate = match Rate::parse(word) {
                    Ok(r) => r,
                    Err(e) => return self.gm_ack(format!("!setrates: the {} rate: {e}", kind.label())),
                };
                if let Some(why) = below_normal(*kind, rate) {
                    return self.gm_ack(format!("!setrates: {why}"));
                }
                rate
            } else {
                match Rate::parse_share_percent(word) {
                    Ok(r) => r,
                    Err(e) => return self.gm_ack(format!("!setrates: the {} share: {e}", kind.label())),
                }
            };
            wanted.push((*kind, rate));
        }

        // **Only the kinds that actually change are written.** Since 2026-09-06 this is the
        // one rate command (the per-kind setters are gone on the owner's instruction), so it
        // inherits their rule: an unchanged rate is not re-stamped, so `store::rates` keeps the
        // moment it really changed and a still-normal rate never records an ending that did
        // not happen.
        let current = match self.store.rates() {
            Ok(r) => r,
            Err(e) => return self.gm_ack(format!("!setrates: could not read the rates: {e}")),
        };
        let listed: Vec<String> = wanted.iter().map(|(k, r)| shown(*k, *r)).collect();
        let changed: Vec<(RateKind, Rate)> =
            wanted.iter().copied().filter(|(k, r)| *r != current.get(*k)).collect();
        if changed.is_empty() {
            return self.gm_ack(format!("!setrates: already {}. Nothing changed.", listed.join(", ")));
        }
        // One timestamp for everything that moved: `store::rates` records when each was set and
        // when an event ended.
        let now = now_unix();
        for (kind, rate) in &changed {
            if let Err(e) = self.store.set_rate(*kind, *rate, now) {
                return self.gm_ack(format!("!setrates: could not save the {} rate: {e}", kind.label()));
            }
        }
        // **No banner.** Since 2026-10-03 the banner is `!announce`'s alone; `!rates` tells anyone.
        self.gm_ack(format!("!setrates: {}. Players see it with !rates; nothing scrolls unless you !announce it.", listed.join(", ")))
    }

    /// `!announce <message>` - put `message` on every player's scrolling banner, on every
    /// channel, until it is replaced or cleared, restarts included. A bare `!announce` clears it.
    pub(super) fn gm_announce(&mut self, arg: &str) -> Vec<Reply> {
        let text = arg.trim();
        if text.chars().count() > ANNOUNCE_MAX_CHARS {
            return self.gm_ack(format!(
                "!announce: that is {} characters; the banner takes at most {ANNOUNCE_MAX_CHARS}. Nothing changed.",
                text.chars().count()
            ));
        }
        let before = self.store.announcement().ok().flatten();
        if let Err(e) = self.store.set_announcement(Some(text), now_unix()) {
            return self.gm_ack(format!("!announce: could not save it: {e}. Nothing changed."));
        }
        let mut out = if text.is_empty() {
            self.gm_ack(match before {
                Some(old) => format!("!announce: the banner is cleared (it said: {old})."),
                None => "!announce <message> scrolls a message across every player's screen until cleared; !announce alone clears it. Nothing is set now.".to_string(),
            })
        } else {
            self.gm_ack(format!("!announce: every player now sees: {text}"))
        };
        out.extend(self.banner_tick());
        out
    }


    /// The banner as it should be right now - the `!announce` text, or nothing.
    /// [`Session::tick`]'s entry point, and at most one packet, usually none.
    pub(super) fn banner_tick(&mut self) -> Vec<Reply> {
        let Ok(want) = self.store.announcement() else {
            // A database error is not worth tearing the banner down over; leave the screen
            // as it is and try again on the next tick.
            return Vec::new();
        };
        if want == self.banner_shown {
            return Vec::new();
        }
        self.banner_shown = want.clone();
        match want {
            Some(text) => vec![Reply {
                opcode: net::broadcast::BROADCAST_MSG,
                body: net::broadcast::banner(&text),
                what: format!("BroadcastMsg type 4: banner UP - {text} (!announce)"),
            }],
            None => vec![Reply {
                opcode: net::broadcast::BROADCAST_MSG,
                body: net::broadcast::clear_banner(),
                what: "BroadcastMsg type 4, flag 0: banner DOWN - the !announce text was cleared".to_string(),
            }],
        }
    }

    /// One rate as it stands, or 1x if the table cannot be read.
    ///
    /// A database error must not silently zero somebody's rewards, so the fallback is the
    /// normal rate rather than nothing.
    pub(super) fn rate(&self, kind: RateKind) -> Rate {
        self.store.rates().map(|r| r.get(kind)).unwrap_or(Rate::NORMAL)
    }
}

/// One kind as `!rates` and `!setrates` print it: a multiplier as `EXP 2x`, the party share
/// as `Party EXP 30%`.
fn shown(kind: RateKind, rate: Rate) -> String {
    if kind.is_event() {
        format!("{} {rate}x", kind.label())
    } else {
        format!("{} {}", kind.label(), rate.as_percent())
    }
}

/// All five, in `!setrates` order.
fn listed(rates: &Rates) -> Vec<String> {
    ALL_KINDS.iter().map(|k| shown(*k, rates.get(*k))).collect()
}

/// Unix seconds, for when a rate or the banner text was set.
fn now_unix() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn session(store: &Arc<Store>, id: u32) -> Session {
        let mut s = Session::new(store.clone(), Arc::new(crate::config::Config::default()));
        assert!(s.claim_for_character(id).contains("claimed the migration"));
        s
    }

    fn one_character() -> (Arc<Store>, u32) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let id = store.create_character(account, 0, &net::opcode::Character { name: "Herald".into(), ..Default::default() }).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        (store, id)
    }

    fn banners(out: &[Reply]) -> Vec<Vec<u8>> {
        out.iter().filter(|r| r.opcode == net::broadcast::BROADCAST_MSG).map(|r| r.body.clone()).collect()
    }

    /// **Nothing configured, nothing scrolls** - not on the first tick, not after many, and a
    /// rate event no longer puts anything up either.
    #[test]
    fn with_no_announcement_nothing_ever_scrolls() {
        let (store, id) = one_character();
        let mut s = session(&store, id);
        for t in 1..=10u64 {
            assert!(banners(&s.tick(t * 500)).is_empty());
        }
        store.set_rate(RateKind::Exp, Rate::from_per_cent(200), now_unix()).unwrap();
        assert!(banners(&s.tick(6_000)).is_empty(), "a rate is not a banner any more");
    }

    /// **`!announce` puts it up once, every session picks it up on its next tick, and a bare
    /// `!announce` takes it down** - each only once, because a re-send restarts the scroll.
    #[test]
    fn announce_puts_the_banner_up_everywhere_and_a_bare_one_takes_it_down() {
        let (store, id) = one_character();
        let mut gm = session(&store, id);
        // Another player's connection: the banner needs no character, only the shared store.
        let mut other = Session::new(store.clone(), Arc::new(crate::config::Config::default()));
        let out = gm.gm_announce("  Double EXP all weekend!  ");
        assert_eq!(banners(&out), vec![net::broadcast::banner("Double EXP all weekend!")]);
        assert_eq!(banners(&other.tick(500)), vec![net::broadcast::banner("Double EXP all weekend!")], "another session, next tick");
        assert!(banners(&other.tick(1_000)).is_empty(), "and only once");
        assert!(banners(&gm.tick(500)).is_empty());

        let out = gm.gm_announce("");
        assert_eq!(banners(&out), vec![net::broadcast::clear_banner()]);
        assert_eq!(banners(&other.tick(1_500)), vec![net::broadcast::clear_banner()]);
        assert!(banners(&other.tick(2_000)).is_empty());
        assert_eq!(store.announcement().unwrap(), None);
    }

    /// **A restart keeps it**: a session opened after the text was stored shows it at once.
    #[test]
    fn a_new_session_after_a_restart_shows_the_stored_text() {
        let (store, _) = one_character();
        store.set_announcement(Some("Welcome back"), 1).unwrap();
        let mut fresh = Session::new(store.clone(), Arc::new(crate::config::Config::default()));
        assert_eq!(banners(&fresh.tick(500)), vec![net::broadcast::banner("Welcome back")]);
    }

    /// Too long is refused and changes nothing.
    #[test]
    fn an_announcement_over_the_limit_is_refused() {
        let (store, id) = one_character();
        let mut gm = session(&store, id);
        let out = gm.gm_announce(&"x".repeat(ANNOUNCE_MAX_CHARS + 1));
        assert!(banners(&out).is_empty());
        assert_eq!(store.announcement().unwrap(), None);
    }
}
