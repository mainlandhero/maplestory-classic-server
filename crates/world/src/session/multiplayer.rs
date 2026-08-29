//! This connection's end of the channel's message bus: what it is owed,
//! and what it owes everyone else.
//!
//! The bus itself is `crate::broadcast`. It holds a mailbox per connection
//! and knows no packet layouts. This file is the half that does know them -
//! it turns "this character is standing here dressed like this" into the
//! bytes a remote client needs, and turns arriving mail into `Reply`s.
//!
//! # What is wired here today, and what is not
//!
//! **Wired - the delivery half.** Every connection takes a mailbox at
//! construction (`Session::joining`), hands it back in `Drop`, collects its
//! mail in both `Session::handle` and `Session::tick`, and drops out of its
//! field on a clean log out or a channel change. That machinery is tested
//! in `crate::broadcast`, and it is exactly what `research/exp-sharing.md`
//! and `session/combat.rs` both name as the missing piece:
//!
//! > *"`Fields::hurt` already returns the whole split; **only the delivery
//! > is missing**."*
//!
//! **NOT wired - the packets themselves.** Per `CLAUDE.md`'s "Built is not
//! wired" rule, said out loud rather than left looking finished: nothing
//! calls `Bus::enter_field`, so no player is announced to any other, so
//! nothing is ever published and every mailbox stays empty. On screen this
//! is identical to the bus not existing.
//!
//! The reason is deliberate, and it is this project's oldest rule: **a
//! wrong reply moves the client into a state nobody has read, which is
//! worse than silence.** The remote-player opcodes are `0x0224`
//! UserEnterField, `0x0225` UserLeaveField and the remote family at
//! `0x293..0x2C4`, and as of 2026-08-29 the *ordering* of those is **[D]** -
//! inferred from the classic `CUserPool` enum matching, not read out of
//! this client - and not one of the bodies has been decoded at all.
//! Guessing one here would be the cash-shop mistake in a new place.
//!
//! When `research/user-enter-field.md` and `research/user-pool-tables.md`
//! land, field entry builds a [`crate::broadcast::Presence`] and the
//! `0x00D9` arm in `session/mod.rs` publishes a move. This file is the only
//! place that has to change.
//!
//! # Why field entry is one call and not two
//!
//! `Bus::enter_field` both announces this player and returns everyone
//! already here, because the failure it prevents is asymmetric: A sees B and
//! B does not see A. That reads on one screen as "it works", and this
//! project has only ever had one screen.

use crate::broadcast::Bus;
use crate::session::{Reply, Session};

impl Session {
    /// This channel's bus. Reached through `Fields` because that `Arc` was
    /// already being handed to every session - see `crate::fields::Fields::bus`.
    pub(super) fn bus(&self) -> &Bus {
        self.fields.bus()
    }

    /// Everything other connections have said to this one since it last looked.
    ///
    /// Called from `Session::handle` (after the reply to whatever arrived) and
    /// from `Session::tick` (for a client that is standing still and therefore
    /// sending nothing). Both, because either alone leaves a case uncovered.
    pub(super) fn collect_mail(&mut self) -> Vec<Reply> {
        self.bus().drain(self.subscriber)
    }

    /// Drop out of the field without ending the connection.
    ///
    /// A log out or a channel change: the character stops being on this map and
    /// everyone still there must be told, but the socket lives on and the
    /// mailbox stays. A **portal walk is not this** - that is
    /// `Bus::enter_field` with the new map, which does the leaving as its first
    /// act so the two halves cannot half-happen.
    pub(super) fn leave_the_field(&mut self) {
        self.bus().leave_field(self.subscriber);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use store::Store;

    use crate::broadcast::Presence;
    use crate::config::Config;
    use crate::fields::Fields;
    use crate::session::{Reply, Session};

    /// The channel's shape: **one** `Fields`, and therefore one bus, shared by
    /// every connection on it - `crate::server::serve`. `Session::new` gives each
    /// session a private one, which is right for the other 490 tests in this
    /// crate and is exactly wrong for these.
    fn channel() -> (Arc<Store>, Arc<Config>, Arc<Fields>) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        // Nothing here answers a packet at all without it; `Session::handle` and
        // `Session::tick` both return early. See `Config::set_field_probe`.
        let config = Arc::new(Config { set_field_probe: true, ..Config::default() });
        (store, config, Arc::new(Fields::new()))
    }

    fn presence(character: u32, map: u32) -> Presence {
        Presence {
            character,
            map,
            spawn: Reply {
                opcode: 0x0224,
                body: Vec::new(),
                what: format!("spawn {character}"),
            },
            farewell: Reply {
                opcode: 0x0225,
                body: Vec::new(),
                what: format!("farewell {character}"),
            },
        }
    }

    /// An opcode this dispatch has no arm for, so `handle` returns whatever the
    /// mailbox contributed and nothing else.
    const NOTHING: [u8; 2] = [0xFF, 0xFE];

    /// The wiring, in one test: a packet published by one session comes out of
    /// another session's `handle`. Everything else here is a corner of this.
    #[test]
    fn mail_published_by_one_session_comes_out_of_another_handle() {
        let (store, config, fields) = channel();
        let mut watcher = Session::joining(store.clone(), config.clone(), fields.clone());
        let mover = Session::joining(store, config, fields.clone());

        fields.bus().enter_field(watcher.subscriber, presence(200, 7));
        fields.bus().enter_field(mover.subscriber, presence(201, 7));
        assert_eq!(watcher.collect_mail().len(), 1, "the arrival of 201");

        fields.bus().publish(
            mover.subscriber,
            7,
            Reply { opcode: 0x02B0, body: vec![1], what: "201 walked".into() },
            Some(201),
        );

        let out = watcher.handle(&NOTHING);
        assert_eq!(out.len(), 1, "the walk should reach the other session");
        assert_eq!(out[0].what, "201 walked");
        assert!(watcher.handle(&NOTHING).is_empty(), "and only once");
    }

    /// The idle case, and the reason `tick` collects too. A player standing
    /// still sends no packets at all, so `handle` is never called on their
    /// connection - if the drain lived only there, the one client that most
    /// needs to see someone walk past would be the one that never did.
    #[test]
    fn a_session_that_sends_nothing_still_collects_its_mail_on_tick() {
        let (store, config, fields) = channel();
        let mut watcher = Session::joining(store.clone(), config.clone(), fields.clone());
        let mover = Session::joining(store, config, fields.clone());
        fields.bus().enter_field(watcher.subscriber, presence(200, 7));
        fields.bus().enter_field(mover.subscriber, presence(201, 7));
        let _ = watcher.collect_mail();

        fields.bus().publish(
            mover.subscriber,
            7,
            Reply { opcode: 0x02B0, body: vec![1], what: "201 walked".into() },
            Some(201),
        );

        let out = watcher.tick(1_000);
        assert!(
            out.iter().any(|r| r.what == "201 walked"),
            "a tick must deliver mail: {:?}",
            out.iter().map(|r| &r.what).collect::<Vec<_>>()
        );
    }

    /// A connection that dies without logging out out - a killed client, a dropped
    /// soccP socket - must still take its character off everyone else's field.
    /// This is the case `Drop for Session` exists for, and it is the one
    /// multiplayer bug that cannot be cleaned up from the client side.
    #[test]
    fn a_dropped_session_hands_back_its_mailbox_and_says_goodbye() {
        let (store, config, fields) = channel();
        let mut watcher = Session::joining(store.clone(), config.clone(), fields.clone());
        fields.bus().enter_field(watcher.subscriber, presence(200, 7));

        {
            let leaver = Session::joining(store, config, fields.clone());
            fields.bus().enter_field(leaver.subscriber, presence(201, 7));
            assert_eq!(fields.bus().subscribers(), 2);
            assert_eq!(watcher.collect_mail().len(), 1, "the arrival of 201");
        }

        assert_eq!(fields.bus().subscribers(), 1, "the mailbox went back");
        let out = watcher.handle(&NOTHING);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].what, "farewell 201");
    }

    /// A clean log out is the tidy version of the same thing, and it must
    /// announce the departure ***once*: the `Drop` that follows a moment later
    /// must not announce a second.
    #[test]

fn logging_out_leaves_the_field_and_the_later_drop_says_nothing_more() {
        let (store, config, fields) = channel();
        let mut watcher = Session::joining(store.clone(), config.clone(), fields.clone());
        fields.bus().enter_field(watcher.subscriber, presence(200, 7));

        {
            let mut leaver = Session::joining(store, config, fields.clone());
            fields.bus().enter_field(leaver.subscriber, presence(201, 7));
            let _ = watcher.collect_mail();
            leaver.leave_the_field();
        }

        let out = watcher.handle(&NOTHING);
        assert_eq!(
            out.iter().map(|r| r.what.clone()).collect::<Vec<_>>(),
            vec!["farewell 201"],
            "one departure, not two"
        );
    }
}
