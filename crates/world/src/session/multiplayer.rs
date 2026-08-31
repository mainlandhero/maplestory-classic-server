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
//! **Nothing *here* has been on the wire**, and that is still true of every
//! packet this file builds. What used to follow it was not: this block read
//! *"no packet in `0x224..0x39F` has ever been observed to do anything"*, and
//! `0x02D1` is in that range, appears in 49 archived files, and is recorded
//! in `STATUS.md` as working on screen twice - the quest fanfare and the blue
//! recovery number. Corrected 2026-08-31; `net::userpool`'s module docs carry
//! the counts and the control.
//!
//! It matters because that sentence was the reason the whole remote family
//! looked unproven. The route above the pool is live. The one hop still
//! unverified is whether `0x0224` puts a `CUser` in the pool. The first run should carry a watch on `0x1429ba60b` -
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
        let mut out = self.bus().drain(self.subscriber);

        // **Facts become packets here, and only here.** See `crate::broadcast::Event` for
        // why they cross the bus in that shape rather than as a `Reply`: an EXP award
        // carries the recipient's own new total and level, which only this session can
        // compute from this character's own record.
        //
        // Bound to a local first so the `&Bus` borrow ends before the loop needs `&mut self`.
        let events = self.bus().drain_events(self.subscriber);
        for event in events {
            match event {
                crate::broadcast::Event::Experience { amount, why, white } => {
                    out.extend(self.award_experience(amount, &why, white, false));
                }
            }
        }
        out
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
        // **A path with no elements dereferences null in the client**, and `walk_closed`
        // does not cover it.
        //
        // `141d59bef mov rax,[rbp+0x18]` reads the head of the decoded element list and
        // then eight `movups` out of its tail, unconditionally and with no null check.
        // The list is empty exactly when `element_count <= 0` - and a 25-byte `0x00D9`
        // with a count of zero **walks closed**, because `for _ in 0..count` runs no
        // iterations, `p` stays at the head length, and the key-state trailer then lands
        // exactly on the end. A negative count is the same hole: `0..negative` is an empty
        // range in Rust, so it closes too.
        //
        // **This is latent, not active.** All 170 806 movement paths in the archive were
        // checked - 6 689 `0x00D9` and 164 117 `0x02FF`, both written by the client's own
        // encoder `FUN_141d57c60` - and **not one** has an empty element list. So an honest
        // client never sends it. That is precisely why the guard belongs here rather than
        // nowhere: the packet is *rebroadcast to other people*, so a client that sends one
        // would be killing somebody else's session, and this server is the only thing
        // between the two. `CLAUDE.md`'s "always answer" is about the same asymmetry seen
        // from the other side.
        if m.element_count <= 0 {
            crate::server::log(&format!(
                "   move NOT rebroadcast: element_count is {}, and an empty element list \
                 dereferences null in the remote client at 141d59bef. Never seen in 170 806 \
                 archived paths, so a client sending this is not an honest one",
                m.element_count
            ));
            return;
        }
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

    /// Rebroadcast a swing to everyone else on this field.
    ///
    /// **The attack half of what the owner asked for on 2026-08-29.**
    /// [`Session::publish_user_move`] is the movement half and was, until this,
    /// the only production caller of [`crate::broadcast::Bus::publish`] in the
    /// whole crate.
    ///
    /// `opcode` and `payload` are the inbound `0x00DF`/`0x00E0`/`0x00E1`
    /// exactly as [`Session::on_attack`] received them.
    ///
    /// # Three things this deliberately does not do
    ///
    /// * **It does not echo the inbound body.** The two encoders are not
    ///   symmetric - 40 header fields in, 13 out - so `crate::remoteattack`
    ///   projects the parse field for field. `research/user-pool-tables.md` §4.
    ///   The movement rebroadcast's byte-for-byte copy is licensed by a shared
    ///   encoder that does not exist on this side.
    /// * **It does not take the character id from the packet.** Nothing on this
    ///   socket authenticates anybody, so the id comes from this session's own
    ///   claimed migration.
    /// * **It does not supersede.** `Bus::publish` is called with `None`
    ///   because an attack is an *event*: two swings are two events, and a
    ///   supersede key would render a whole fight as one hit. Pinned by
    ///   `remoteattack::tests::three_swings_are_three_packets_and_would_not_be_with_a_supersede_key`.
    ///
    /// # A parse failure is not a refusal to answer
    ///
    /// `CLAUDE.md`'s always-answer rule. This returns nothing on every failure
    /// path and the caller's reply to the attacker is untouched, so a body this
    /// server cannot re-encode costs the *observers* a swing and costs the
    /// attacker nothing at all.
    pub(super) fn publish_user_attack(&mut self, opcode: u16, payload: &[u8]) {
        let Some(out_opcode) = crate::remoteattack::remote_attack_opcode(opcode) else {
            return; // 0x00E2 body attack - an undecoded layout, nothing to re-encode
        };
        let Some(chr) = self.claimed_character() else { return };
        let Some(map) = self.bus().map_of(self.subscriber) else { return };
        let attack = match net::attack::parse(opcode, payload) {
            Ok(a) => a,
            Err(e) => {
                crate::server::log(&format!(
                    "   swing NOT rebroadcast: {e}. The attacker's own reply is unaffected \
                     - this server would be emitting a body it could not account for, and a \
                     wrong length on this wire has killed this client twice."
                ));
                return;
            }
        };
        let body = crate::remoteattack::user_attack_remote(
            chr.id,
            // `[user + 0x406c]`, the same dword `0x0224` UserEnterField fills at its body
            // offset 12 with the level. See `crate::remoteattack`'s module docs: the
            // argument is consistency between the two packets, not identification.
            u16::try_from(chr.level).unwrap_or(u16::MAX),
            &attack,
        );
        let what = crate::remoteattack::describe(chr.id, out_opcode, &attack, body.len());
        self.bus().publish(
            self.subscriber,
            map,
            Reply { opcode: out_opcode, body, what },
            // An event, not a state. See the doc block above.
            None,
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

    /// **A kill on one connection pays a character on another.** This is the whole
    /// EXP-share feature end to end, and it is the test that fails if any link is
    /// unhooked: `award_kill_experience`, `Bus::send_to_character`, the mailbox,
    /// `Bus::drain_events`, or the conversion back into a packet in `collect_mail`.
    ///
    /// The assertion that matters most is the one *before* the tick. The helper has
    /// earned nothing at the moment the killer is paid, because the fact has crossed
    /// the bus but nobody has turned it into a packet yet - which is the design:
    /// `0x007C` carries the recipient's own new total, so only the recipient's session
    /// may compute it. If a future change ever builds the helper's packet inside the
    /// killer's session, that line is what catches it.
    #[test]
    fn a_helpers_share_of_a_kill_reaches_their_own_session() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Killer", "Helper"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: 104_040_000,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }

        let mut killer = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut helper = Session::joining(store.clone(), config, fields.clone());
        killer.claim_for_character(ids[0]);
        helper.claim_for_character(ids[1]);
        killer.on_field_entered();
        helper.on_field_entered();
        // Clear the entry announcements so what is left is the EXP and nothing else.
        killer.tick(1_000);
        helper.tick(1_000);

        let exp_of = |id: u32| {
            store.characters_for(account, 0).unwrap().into_iter().find(|c| c.id == id).unwrap().exp
        };

        // 70/30, so the killer holds the majority and the helper is owed 30 of 100.
        let shares = vec![
            crate::fields::DamageShare { character: ids[0], dealt: 70, total: 100, majority: true },
            crate::fields::DamageShare { character: ids[1], dealt: 30, total: 100, majority: false },
        ];
        let paid = killer.award_kill_experience(100, "a kill", ids[0], &shares);

        assert!(
            paid.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
            "the killer is paid in their own reply: {paid:?}"
        );
        assert_eq!(exp_of(ids[0]), 100, "the majority holder takes the whole amount");
        assert_eq!(
            exp_of(ids[1]),
            0,
            "and the helper has NOT been paid yet - the fact is on the bus, but only the              helper's own session may build a packet carrying the helper's own total"
        );

        // The helper collects their mail, and only now is anything written for them.
        let mail = helper.tick(2_000);
        assert!(
            mail.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
            "the helper's own session turns the fact into a packet: {mail:?}"
        );
        assert_eq!(exp_of(ids[1]), 30, "30/100 of the damage, so 30 of the 100 exp");

        // **And the killer collects their own mail too.** Without this tick the test
        // cannot see a double payout at all: a share the killer wrongly addressed to
        // itself would sit undrained in its own mailbox and every assertion above would
        // still pass. Verified by injecting exactly that bug - dropping the
        // `s.character != chr_id` filter in `award_kill_experience` - which this line
        // catches and nothing else did.
        let again = killer.tick(3_000);
        assert!(
            !again.iter().any(|r| r.opcode == net::stats::STAT_CHANGED),
            "the killer must not have addressed a share to itself: {again:?}"
        );
        assert_eq!(exp_of(ids[0]), 100, "and so it is still paid exactly once");
    }

    /// A share owed to somebody who is not on this channel is dropped, not retried and
    /// not panicked over. They logged out between landing the hit and the mob dying,
    /// which is ordinary. The killer is still paid.
    #[test]
    fn a_share_owed_to_an_absent_character_is_dropped_and_the_killer_still_pays_itself() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character {
            name: "Lonely".to_string(),
            map_id: 104_040_000,
            ..Default::default()
        };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();

        let mut killer = Session::joining(store.clone(), config, fields);
        killer.claim_for_character(id);
        killer.on_field_entered();
        killer.tick(1_000);

        // 9_999 is nobody: no session on this channel is playing them.
        let shares = vec![
            crate::fields::DamageShare { character: id, dealt: 70, total: 100, majority: true },
            crate::fields::DamageShare { character: 9_999, dealt: 30, total: 100, majority: false },
        ];
        let paid = killer.award_kill_experience(100, "a kill", id, &shares);

        assert!(paid.iter().any(|r| r.opcode == net::stats::STAT_CHANGED), "{paid:?}");
        let exp = store
            .characters_for(account, 0)
            .unwrap()
            .into_iter()
            .find(|c| c.id == id)
            .unwrap()
            .exp;
        assert_eq!(exp, 100, "the absent share changes nothing about the killer's own");
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

    /// `previous-runs/world-20260820-121055.log` 16:10:28.598, `0x00DF`, 229 bytes: a
    /// plain swing that connected — mob 2002, one hit of 19, not critical. The same body
    /// `net::attack` and `crate::remoteattack` both test against, so all three agree about
    /// what the bytes mean.
    const MELEE_229: &str = "0001000000000000000000000000000001050000009fae340801040000003b80680a70028b010000000070028b0100000000000000000000000000000000000000000000000000000100000001000000000a0055736572204d656c65658901000000000000000000000000000000010000000000000000000000d20700000200000001000013000000000000000000000736028b0136028b0135027b01890100000000000001000002000000000001012302710148028b01000000007e6c3c6600000000030000000000d5c057820100000092e9bc2707000000bc6509e5000080e8da8f00";

    fn hex_body(hex: &str) -> Vec<u8> {
        (0..hex.len() / 2)
            .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).expect("fixture hex"))
            .collect()
    }

    /// Two characters on one map, and a **real captured swing** crossing between them.
    ///
    /// This is the attack half of the owner's 2026-08-29 sentence end to end, and it fails if
    /// any link is unhooked: the `is_attack_opcode` arm, `on_attack`,
    /// `publish_user_attack`, `crate::remoteattack`, `Bus::publish`, the mailbox, or the
    /// drain in `handle`.
    ///
    /// **Four effects are asserted, not one.** `CLAUDE.md`: a test that checks one of
    /// several effects gives false confidence about the rest.
    ///
    /// **Nothing here has been on a screen.** Every offset in the body is static analysis
    /// (`research/user-pool-tables.md` §4), so this proves the server does what it was told
    /// to do, not that the client likes it.
    #[test]
    fn a_real_captured_swing_is_rebroadcast_to_the_other_player() {
        let payload = hex_body(MELEE_229);
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Watcher", "Swinger"] {
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
        let mut attacker = Session::joining(store.clone(), config, fields.clone());
        watcher.claim_for_character(ids[0]);
        attacker.claim_for_character(ids[1]);
        // The `u16` at body offset 4 is `[user + 0x406c]` - the same dword `0x0224` fills
        // with the level. Read it from the record rather than assuming, so this test also
        // pins WHICH number the session chose to send.
        let level = store
            .characters_for(account, 0)
            .unwrap()
            .into_iter()
            .find(|c| c.id == ids[1])
            .unwrap()
            .level;
        watcher.on_field_entered();
        attacker.on_field_entered();
        let _ = watcher.tick(1_000); // clear the arrivals
        let _ = attacker.tick(1_000);

        // The parse has to be the same one the session does, or this test asserts against
        // its own arithmetic rather than against the client's bytes.
        let parsed = net::attack::parse(net::combat::USER_MELEE_ATTACK, &payload)
            .expect("a real captured body");
        let expected = crate::remoteattack::user_attack_remote(
            ids[1],
            u16::try_from(level).unwrap(),
            &parsed,
        );

        let mut packet = net::combat::USER_MELEE_ATTACK.to_le_bytes().to_vec();
        packet.extend_from_slice(&payload);
        let own = attacker.handle(&packet);

        // (1) The attacker never receives their own swing - their client drew it already.
        assert!(
            !own.iter().any(|r| r.opcode == 0x029E),
            "the attacker must not be sent their own broadcast: {own:?}"
        );

        // (2) The other player does, exactly once.
        let mail = watcher.handle(&NOTHING);
        let swings: Vec<&Reply> = mail.iter().filter(|r| r.opcode == 0x029E).collect();
        assert_eq!(swings.len(), 1, "one swing should have crossed: {mail:?}");

        // (3) ...and the body is the one `crate::remoteattack` builds, byte for byte,
        //     addressed to the attacker and 85 bytes long.
        assert_eq!(swings[0].body, expected, "the body must be the projected re-encode");
        assert_eq!(
            u32::from_le_bytes(swings[0].body[0..4].try_into().unwrap()),
            ids[1],
            "addressed to the swinger"
        );
        assert_eq!(
            u16::from_le_bytes(swings[0].body[4..6].try_into().unwrap()),
            u16::try_from(level).unwrap(),
            "the u16 at 4 is the attacker's level, the same field 0x0224 sets at its 12"
        );
        assert_eq!(swings[0].body.len(), 85, "6 + 43 + 12 + (6 + 10 + 8)");
        assert!(
            swings[0].body.len() < payload.len(),
            "and it is SHORTER than the inbound body - 13 header fields of 40, no trailer"
        );
        assert_eq!(
            u64::from_le_bytes(swings[0].body[69..77].try_into().unwrap()),
            19,
            "the damage the attacker's own client computed, absolute and unscaled"
        );

        // (4) A second swing is a second packet. If `publish` ever gained a supersede key
        //     here, a whole fight would render as one hit.
        let _ = attacker.handle(&packet);
        let _ = attacker.handle(&packet);
        assert_eq!(
            watcher.handle(&NOTHING).iter().filter(|r| r.opcode == 0x029E).count(),
            2,
            "two swings are two events - nothing coalesces"
        );
    }

    /// **The broadcast is additive: the attacker gets exactly what they got before.**
    ///
    /// The same swing is run twice, on two independent channels - once with a second
    /// player on the map and once alone - and the attacker's own replies must be
    /// identical. That is the property the whole feature has to preserve, and it cannot be
    /// checked by looking at one session.
    ///
    /// The swing is patched to carry **Power Strike level 5** so the attacker's reply is
    /// genuinely non-empty (`session/combat.rs::spend_attack_mp` sends a `0x007C`);
    /// comparing two empty vectors would pass against a broadcast that had eaten the
    /// reply. That needs the generated skill table, so the test returns early without it,
    /// exactly as `session::tests::an_attack_skill_costs_mp_and_a_potion_does_not_undo_it`
    /// does.
    #[test]
    fn the_broadcast_does_not_change_what_the_attacker_gets() {
        let skills = std::path::Path::new("../../gm-handbook/skills.txt");
        if !skills.exists() {
            return; // generated, gitignored - python tools/dump_skills.py
        }
        const POWER_STRIKE: u32 = 1_001_001;

        let mut payload = hex_body(MELEE_229);
        payload[2..6].copy_from_slice(&POWER_STRIKE.to_le_bytes());
        payload[6] = 5;
        assert!(
            net::attack::parse(net::combat::USER_MELEE_ATTACK, &payload).is_ok(),
            "the captured body must still parse after patching"
        );
        let mut packet = net::combat::USER_MELEE_ATTACK.to_le_bytes().to_vec();
        packet.extend_from_slice(&payload);

        // One channel, one or two players on it, and the attacker's own reply.
        let swing_with = |companion: bool| -> Vec<(u16, Vec<u8>)> {
            let store = Arc::new(Store::open_in_memory().unwrap());
            let config = Arc::new(Config {
                set_field_probe: true,
                firstjob: crate::firstjob::CombatTable::load(skills),
                ..Config::default()
            });
            let fields = Arc::new(Fields::new());
            let account = store.create_account("maplecw", "correct horse battery").unwrap();

            let mut make = |name: &str| {
                let chr = net::opcode::Character {
                    name: name.to_string(),
                    map_id: 104_040_000,
                    ..Default::default()
                };
                let id = store.create_character(account, 0, &chr).unwrap().id;
                let mut rec = store
                    .characters_for(account, 0)
                    .unwrap()
                    .into_iter()
                    .find(|c| c.id == id)
                    .unwrap();
                rec.job = 100;
                rec.mp = 100;
                rec.max_mp = 100;
                store.save_character_progress(&rec).unwrap();
                store.set_skill_level(id, POWER_STRIKE, 5).unwrap();
                store.create_migration(account, id, 0, 0).unwrap();
                id
            };
            let attacker_id = make("Swinger");
            let watcher_id = if companion { Some(make("Watcher")) } else { None };

            let mut attacker =
                Session::joining(store.clone(), config.clone(), fields.clone());
            attacker.claim_for_character(attacker_id);
            let mut watcher = watcher_id.map(|id| {
                let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
                s.claim_for_character(id);
                s
            });
            if let Some(w) = watcher.as_mut() {
                w.on_field_entered();
                let _ = w.tick(1_000);
            }
            attacker.on_field_entered();
            let _ = attacker.tick(1_000);

            attacker
                .handle(&packet)
                .into_iter()
                .map(|r| (r.opcode, r.body))
                .collect()
        };

        let alone = swing_with(false);
        let watched = swing_with(true);

        assert!(
            alone.iter().any(|(op, _)| *op == net::stats::STAT_CHANGED),
            "the MP spend must be in the reply, or this comparison is two empty vectors"
        );
        assert_eq!(
            watched, alone,
            "a second player on the map must not change one byte of what the attacker gets"
        );
        assert!(
            !watched.iter().any(|(op, _)| *op == 0x029E),
            "and the broadcast must never leak into the attacker's own reply"
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

    /// A real captured `0x00D9`: 14:01:40.792, map 1, three elements, walks closed.
    /// Byte-identical to `net::usermove::tests::MAP1_FIRST`, which asserts every field of
    /// it against the `world.log` line it came from.
    const REAL_MOVE: &str = "0057a301a8c139cc04000000000043ffab010000000003000043ffd7010000a401000000000000ffff06d200000043ffe50100000000000000000000ffff061e00000043ffe501000000002b0000000000ffff040e010011000000000000000000";

    fn hex(h: &str) -> Vec<u8> {
        (0..h.len() / 2).map(|i| u8::from_str_radix(&h[i * 2..i * 2 + 2], 16).unwrap()).collect()
    }

    /// A `0x00D9` packet: the opcode, then the body.
    fn move_packet(body: &[u8]) -> Vec<u8> {
        let mut b = net::usermove::CLIENT_USER_MOVE.to_le_bytes().to_vec();
        b.extend_from_slice(body);
        b
    }

    /// **The smallest body that walks closed with NO elements**: the 10-byte head, the
    /// 14-byte path head with `element_count = 0`, and a key-state count of 0.
    ///
    /// 25 bytes. Nothing in the archive looks like this - all 170 806 captured paths carry
    /// at least one element - which is exactly why it has to be constructed here rather
    /// than quoted from a capture.
    fn empty_element_body() -> Vec<u8> {
        let mut b = vec![0u8; net::usermove::USER_MOVE_HEAD_LEN];
        b.extend_from_slice(&[0u8; net::usermove::MOVE_PATH_HEAD_LEN]);
        // element_count is an i16 at path-head offset 12.
        let at = net::usermove::USER_MOVE_HEAD_LEN + 12;
        b[at..at + 2].copy_from_slice(&0i16.to_le_bytes());
        b.push(0); // key-state count
        b
    }

    /// **An empty element list is not rebroadcast, and a real path is.**
    ///
    /// The pair is the whole test: a guard that refuses everything passes the first half on
    /// its own, and this project has shipped exactly that shape before.
    ///
    /// # What the guard is for
    ///
    /// `141d59bef mov rax,[rbp+0x18]` reads the head of the decoded element list and then
    /// eight `movups` out of its tail, unconditionally and with no null check. The list is
    /// empty when `element_count <= 0`, and such a body **walks closed** - `for _ in
    /// 0..count` runs no iterations, so `p` never moves off the path head and the key-state
    /// trailer lands exactly on the end. `UserMove::walk_closed` therefore does not cover
    /// it, which is the reason this is a separate check and not a stronger `walk_closed`.
    ///
    /// It is **latent**: no honest client sends one. The guard exists because the packet is
    /// rebroadcast *to other people* - a client that sent this would be killing somebody
    /// else's session, and this server is the only thing in between.
    #[test]
    fn a_movement_path_with_no_elements_is_not_rebroadcast() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Watcher", "Mover"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: 1,
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
        watcher.collect_mail(); // the arrival of the mover; not what this is about

        // ---- the control: a real captured path IS rebroadcast --------------------------
        let real = hex(REAL_MOVE);
        assert_eq!(real.len(), 97, "the world.log line says 97 bytes");
        mover.handle(&move_packet(&real));
        let out = watcher.collect_mail();
        assert_eq!(out.len(), 1, "a real walk must reach the other player: {out:?}");
        assert_eq!(out[0].opcode, net::userpool::USER_MOVE_REMOTE);

        // ---- the guard: an empty element list is NOT --------------------------------
        let empty = empty_element_body();
        assert_eq!(empty.len(), 25);
        // It parses, and it walks closed - which is the trap.
        let parsed = net::usermove::parse_user_move(&empty).expect("it parses");
        assert_eq!(parsed.element_count, 0);
        assert!(parsed.walk_closed, "THIS is why walk_closed cannot be the guard");
        assert!(parsed.path(&empty).is_some(), "and the path accessor hands it over");

        mover.handle(&move_packet(&empty));
        assert!(
            watcher.collect_mail().is_empty(),
            "an empty element list must never reach another client - it dereferences null"
        );

        // And the mover is still able to move afterwards: the guard drops one packet, it
        // does not wedge the connection.
        mover.handle(&move_packet(&real));
        assert_eq!(watcher.collect_mail().len(), 1, "the next real walk still goes out");
    }

    /// **Where a player was standing is a fact about the map they just left.**
    ///
    /// `last_position` is fed only by `0x00D9` and by attack packets, so after a warp it
    /// held the *previous* field's coordinates until the first step. Three readers used it
    /// and all three were wrong across a map change - the `0x0224` spawn other players are
    /// told, and the two drop paths.
    ///
    /// `None` is the honest answer, and an off-map coordinate is worse than no coordinate:
    /// the origin is at least inside the field, and a drop outside the client's own pick-up
    /// box is drawn and can never be collected.
    #[test]
    fn a_map_change_forgets_where_the_player_was_standing() {
        let (store, config, fields) = channel();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character {
            name: "Walker".to_string(),
            map_id: 1,
            ..Default::default()
        };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let mut s = Session::joining(store.clone(), config, fields);
        s.claim_for_character(id);

        // A real walk on map 1 gives the session a position - the control, because a test
        // that only checks `None` at the end passes on a field that was never set.
        s.handle(&move_packet(&hex(REAL_MOVE)));
        let before = s.last_position.expect("the walk must set a position");
        assert_ne!(before, (0, 0), "and it is a real coordinate, not the origin");

        let mut moved = s.claimed_character().expect("a claimed character");
        s.go_to_map(&mut moved, 104_040_000, 0, "a portal walk".to_string());
        assert_eq!(
            s.last_position, None,
            "the new map must not inherit the old map's coordinates"
        );
    }


    /// Two players on one map, and a helper to put them there.
    fn two_on_a_map(store: &Arc<Store>, map: u32) -> (u32, u32) {
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Stayer", "Leaver"] {
            let chr = net::opcode::Character {
                name: name.to_string(),
                map_id: map,
                ..Default::default()
            };
            let id = store.create_character(account, 0, &chr).unwrap().id;
            store.create_migration(account, id, 0, 0).unwrap();
            ids.push(id);
        }
        (ids[0], ids[1])
    }

    /// **A server-initiated warp publishes its own farewell**, without waiting for the
    /// warped client to volunteer anything.
    ///
    /// Until this, the departure was a side effect of the *arrival*: `Bus::enter_field`
    /// leaves the old field first, and that runs from `on_field_entered`, which fires on the
    /// client's `0x00DC`. So the old map's players stopped seeing you only if your client
    /// sent a packet. 273 SetFields against 268 markers across 444 archived logs - and the
    /// five that did not answer are the entire point, because the thing that would have
    /// removed the ghost was the packet that never came.
    ///
    /// The test deliberately **never calls `on_field_entered` on the leaver after the warp**.
    /// That is what makes it a test of this fix rather than of the arrival path.
    #[test]
    fn a_server_warp_removes_the_player_from_the_old_map_by_itself() {
        let (store, config, fields) = channel();
        let (stayer_id, leaver_id) = two_on_a_map(&store, 1);
        let mut stayer = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut leaver = Session::joining(store, config, fields.clone());
        stayer.claim_for_character(stayer_id);
        leaver.claim_for_character(leaver_id);
        stayer.on_field_entered();
        leaver.on_field_entered();
        assert_eq!(
            stayer.collect_mail().len(),
            1,
            "the control: the stayer was told the leaver arrived"
        );

        let mut chr = leaver.claimed_character().expect("a claimed character");
        leaver.go_to_map(&mut chr, 104_040_000, 0, "a server warp".to_string());

        let out = stayer.collect_mail();
        assert_eq!(out.len(), 1, "the stayer must be told, without the leaver's help: {out:?}");
        assert_eq!(out[0].opcode, net::userpool::USER_LEAVE_FIELD);

        // And the leaver really is gone from the old field, not merely announced as gone -
        // a later publish on map 1 must not reach them.
        fields.bus().publish(
            stayer.subscriber,
            1,
            Reply { opcode: 0x02B0, body: vec![1], what: "the stayer waved".into() },
            None,
        );
        assert!(
            leaver.collect_mail().is_empty(),
            "a departed player must not keep receiving the old map's traffic"
        );
    }

    /// **Entering the Cash Shop publishes a farewell too.**
    ///
    /// The shop is a different stage: the client tears the field down and the player is not
    /// standing anywhere. Without this they stay drawn on the map they left, frozen, until
    /// they come back - and if they log out from inside the shop, until the socket drops.
    #[test]
    fn entering_the_cash_shop_removes_the_player_from_the_map() {
        let (store, config, fields) = channel();
        let (stayer_id, shopper_id) = two_on_a_map(&store, 1);
        let mut stayer = Session::joining(store.clone(), config.clone(), fields.clone());
        let mut shopper = Session::joining(store, config, fields.clone());
        stayer.claim_for_character(stayer_id);
        shopper.claim_for_character(shopper_id);
        stayer.on_field_entered();
        shopper.on_field_entered();
        assert_eq!(stayer.collect_mail().len(), 1, "the control: the shopper arrived");

        shopper.on_cash_shop_request(&[]);
        let out = stayer.collect_mail();
        assert_eq!(out.len(), 1, "the shopper must vanish from the map: {out:?}");
        assert_eq!(out[0].opcode, net::userpool::USER_LEAVE_FIELD);
    }

}
