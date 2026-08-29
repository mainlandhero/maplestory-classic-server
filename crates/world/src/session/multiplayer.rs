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
//! **Wired as of 2026-08-29 - and never seen on a screen.** Three packets
//! go out now: `0x0224` UserEnterField, `0x0225` UserLeaveField and
//! `0x0293` UserMoveRemote, decoded in `research/user-enter-field.md` and
//! `research/user-pool-tables.md`. Every offset is read out of this client
//! rather than taken from the v214 reference, and the enter/leave
//! assignment is **[L]** from the bodies (`0x0224` allocates a `CUser` and
//! inserts it; `0x0225` unlinks a node and calls its destructor) rather
//! than **[D]** from enum order.
//!
//! **Nothing here has been on the wire.** No packet in `0x224..0x39F` has
//! ever been observed to do anything in any archived run, so every claim
//! below is static. The first run should carry a watch on `0x1429ba60b` -
//! the allocation past every gate - because if that never fires, the
//! answer is one of the six gates in `user-enter-field.md` §5 and no
//! amount of body work will help.
//!
//! # The one thing known to be wrong, and it self-heals
//!
//! **A character who has just arrived has no position.** The server's only
//! source is the client's own `0x00D9` movement reports (and attack
//! packets), so a player who has not yet moved is announced at whatever
//! [`Session::last_position`] holds - `(0, 0)` on a fresh arrival, which is
//! the map origin.
//!
//! It corrects itself on that player's first step, because `0x0293` moves
//! a remote user the client already has. The visible residue is a
//! newly-arrived character appearing at the origin until they move, and a
//! player who arrives and stands **perfectly still** staying there.
//!
//! The real fix is portal coordinates: `gm-handbook/portals.txt` carries
//! `map, index, portal, target map, target portal` and **no x/y**, so
//! `tools/dump_portals.py` would have to emit them. That is a WZ dump
//! change and a handbook regeneration, which is why it is not in this
//! commit.
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

    /// Where this character is, as far as anyone here knows.
    ///
    /// `last_position` is fed by `0x00D9` and by attack packets, and is
    /// `None` until one of them arrives - see the module docs for what that
    /// costs and why it heals. `foothold` stays `0`, which is legal and
    /// means "resolve it yourself"; sending a foothold id from the map we
    /// just left would be worse than sending none.
    pub(super) fn remote_at(&self) -> net::userpool::RemoteAt {
        let (x, y) = self.last_position.unwrap_or((0, 0));
        net::userpool::RemoteAt { x, y, move_action: 0, foothold: 0 }
    }

    /// This character as everyone else on the field needs to hear about it.
    ///
    /// Both packets are built **now**, including the farewell, because the
    /// farewell has to survive into `Drop` - where there is no store to
    /// load a character from. See `crate::broadcast::Presence`.
    fn presence(&self, chr: &net::opcode::Character) -> crate::broadcast::Presence {
        crate::broadcast::Presence {
            character: chr.id,
            map: chr.map_id,
            spawn: Reply {
                opcode: net::userpool::USER_ENTER_FIELD,
                body: net::userpool::user_enter_field(chr, self.remote_at()),
                what: format!(
                    "UserEnterField: {} ({}) on map {} at ({}, {}) - {} bytes. \
                     research/user-enter-field.md; nothing here authenticates anybody.",
                    chr.name,
                    chr.id,
                    chr.map_id,
                    self.remote_at().x,
                    self.remote_at().y,
                    net::userpool::user_enter_field_len(chr),
                ),
            },
            farewell: Reply {
                opcode: net::userpool::USER_LEAVE_FIELD,
                body: net::userpool::user_leave_field(chr.id),
                what: format!("UserLeaveField: character {} is gone", chr.id),
            },
        }
    }

    /// Announce this character to the field, and return everyone already on it.
    ///
    /// Called from `on_field_entered` - the `0x00DC` marker, which arrives
    /// once per field entry, every time, including after a portal walk.
    /// That is the right hook for the same reason the NPC and mob re-sends
    /// use it: the client destroys and rebuilds its pools on every
    /// `SetField`, so the user pool has to be refilled after each one too.
    ///
    /// **Our own character is not in the returned list.** A `0x0224`
    /// carrying our own id clears one dword and returns without reading the
    /// body (`research/user-enter-field.md` §5) - harmless, but there is no
    /// reason to send it, and `Bus::enter_field` excludes the sender.
    pub(super) fn announce_field_entry(&mut self) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else { return Vec::new() };
        let presence = self.presence(&chr);
        let here = self.bus().enter_field(self.subscriber, presence);
        if here.is_empty() {
            return here;
        }
        crate::server::log(&format!(
            "   field {} has {} other player(s) - sending {} UserEnterField",
            chr.map_id,
            here.len(),
            here.len()
        ));
        here
    }

    /// Rebroadcast a movement report to everyone else on this field.
    ///
    /// `body` is the `0x00D9` body with the opcode already stripped, and
    /// `m` is what [`net::usermove::parse_user_move`] made of it.
    ///
    /// **A path that did not walk closed is not rebroadcast.** `UserMove::path`
    /// returns `None` there, and that is a refusal, not an empty path: this
    /// server would be re-emitting bytes it could not itself account for,
    /// and a body of the wrong length has killed this client twice.
    ///
    /// Superseded per character, so a stalled observer gets the latest
    /// position rather than a replay - `crate::broadcast`.
    pub(super) fn publish_user_move(&mut self, m: &net::usermove::UserMove, body: &[u8]) {
        let Some(chr) = self.claimed.as_ref().map(|c| c.character_id) else { return };
        let Some(map) = self.bus().map_of(self.subscriber) else { return };
        let Some(path) = m.path(body) else {
            crate::server::log(
                "   move NOT rebroadcast: the element walk did not close, so the path \
                 cannot be re-emitted - see net::usermove::UserMove::walk_closed",
            );
            return;
        };
        self.bus().publish(
            self.subscriber,
            map,
            Reply {
                opcode: net::userpool::USER_MOVE_REMOTE,
                body: net::userpool::user_move_remote(chr, path),
                what: format!(
                    "UserMoveRemote: character {chr} to ({}, {}), {} path bytes copied \
                     verbatim (no key-state trailer - 1429d2eb5 XOR R8D,R8D)",
                    m.x,
                    m.y,
                    path.len()
                ),
            },
            Some(chr),
        );
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

    /// Two sessions, one channel, one map - and a **real** `0x0224` crosses
    /// between them. This is the end-to-end claim of the whole feature, and it
    /// is the test that fails if any link in the chain is unhooked: the
    /// `0x00DC` arm, `announce_field_entry`, `Bus::enter_field`, the mailbox,
    /// or the drain in `handle`.
    ///
    /// **Nothing here has been on a screen.** Every offset in the body is
    /// static analysis (`research/user-enter-field.md`), so this proves the
    /// server does what it was told to do, not that the client likes it.
    #[test]
    fn two_players_entering_one_map_are_announced_to_each_other() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Wanderer", "Stranger"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: 104_040_000,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }

        let mut first = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut second = Session::joining(store, config, fields.clone());
        first.claim_for_character(ids[0]);
        second.claim_for_character(ids[1]);

        // The first player arrives to an empty map and is told about nobody.
        let alone = first.on_field_entered();
        assert!(
            !alone.iter().any(|r| r.opcode == net::userpool::USER_ENTER_FIELD),
            "there is nobody else on the map yet"
        );

        // The second arrives, and is handed the first.
        let joined = second.on_field_entered();
        let spawns: Vec<&Reply> = joined
            .iter()
            .filter(|r| r.opcode == net::userpool::USER_ENTER_FIELD)
            .collect();
        assert_eq!(spawns.len(), 1, "the second player should be told about the first");
        assert_eq!(
            u32::from_le_bytes(spawns[0].body[4..8].try_into().unwrap()),
            ids[0],
            "and it should carry the first player's character id"
        );

        // ...and the first hears about the second without having asked.
        let mail = first.tick(1_000);
        let told: Vec<&Reply> = mail
            .iter()
            .filter(|r| r.opcode == net::userpool::USER_ENTER_FIELD)
            .collect();
        assert_eq!(told.len(), 1, "the sighting must go both ways: {mail:?}");
        assert_eq!(
            u32::from_le_bytes(told[0].body[4..8].try_into().unwrap()),
            ids[1],
        );
    }

    /// **A real captured walk crosses between two sessions.** This is the other
    /// half of what was asked for, end to end: a `0x00D9` body this client
    /// actually sent, fed to one session's `handle`, comes out of the other's as
    /// `0x0293` carrying the path **byte for byte**.
    ///
    /// The body is `14:01:40.792` from
    /// `research/fixtures/character-on-map1-playable-world.log` - the same
    /// capture `net::usermove`'s own tests parse.
    #[test]
    fn a_real_captured_walk_is_rebroadcast_to_the_other_player() {
        const MAP1_FIRST: &str = "0057a301a8c139cc04000000000043ffab010000000003000043\
ffd7010000a401000000000000ffff06d200000043ffe50100000000000000000000ffff061e00000043ffe5\
01000000002b0000000000ffff040e010011000000000000000000";
        let path_bytes: Vec<u8> = (0..MAP1_FIRST.len() / 2)
            .map(|i| u8::from_str_radix(&MAP1_FIRST[i * 2..i * 2 + 2], 16).unwrap())
            .collect();

        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Wanderer", "Stranger"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: 104_040_000,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        let mut watcher = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut mover = Session::joining(store, config, fields.clone());
        watcher.claim_for_character(ids[0]);
        mover.claim_for_character(ids[1]);
        watcher.on_field_entered();
        mover.on_field_entered();
        let _ = watcher.tick(1_000); // clear the arrival

        // The parse has to be the same one the session does, or this test is
        // asserting against its own arithmetic rather than the client's bytes.
        let parsed = net::usermove::parse_user_move(&path_bytes).expect("a real body");
        assert!(parsed.walk_closed, "the fixture must walk closed or nothing is sent");
        let path = parsed.path(&path_bytes).expect("a closed walk has a span");

        let mut packet = net::usermove::CLIENT_USER_MOVE.to_le_bytes().to_vec();
        packet.extend_from_slice(&path_bytes);
        assert!(mover.handle(&packet).is_empty(), "the mover itself is not answered");

        let mail = watcher.handle(&[0xFF, 0xFE]);
        let moves: Vec<&Reply> = mail
            .iter()
            .filter(|r| r.opcode == net::userpool::USER_MOVE_REMOTE)
            .collect();
        assert_eq!(moves.len(), 1, "one move should have crossed: {mail:?}");

        let body = &moves[0].body;
        assert_eq!(
            u32::from_le_bytes(body[0..4].try_into().unwrap()),
            ids[1],
            "addressed to the mover's character"
        );
        assert_eq!(&body[4..], path, "the path is copied verbatim");
        assert_eq!(
            body.len(),
            4 + path.len(),
            "no key-state trailer - 0x0293 passes zero at 1429d2eb5"
        );
        assert!(
            body.len() < path_bytes.len(),
            "and it is therefore SHORTER than the inbound body, which carries one"
        );
    }

    /// Different maps, no sighting. The bus filters by map, and a spawn packet
    /// for a character on another field names an object the client's pool does
    /// not have.
    #[test]
    fn two_players_on_different_maps_are_not_announced() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for (name, map) in [("Wanderer", 104_040_000u32), ("Stranger", 100_000_000)] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: map,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }

        let mut first = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut second = Session::joining(store, config, fields.clone());
        first.claim_for_character(ids[0]);
        second.claim_for_character(ids[1]);
        first.on_field_entered();

        let joined = second.on_field_entered();
        assert!(
            !joined.iter().any(|r| r.opcode == net::userpool::USER_ENTER_FIELD),
            "a different map is a different field"
        );
        assert!(
            !first
                .tick(1_000)
                .iter()
                .any(|r| r.opcode == net::userpool::USER_ENTER_FIELD),
            "and the first player hears nothing either"
        );
    }

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
