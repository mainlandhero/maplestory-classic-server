//! Sitting down and standing up, and what a chair adds to the idle tick.
//!
//! `net::chair` owns the wire format, `world::chairs` owns the table, this is the join.
//!
//! # Both opcodes must keep sending the unlock
//!
//! `0x00DA` and `0x00DB` are both in `net::dropmoney::LATCHING_REQUESTS`, and before this
//! module existed the catch-all arm answered them with the empty-mask `StatChanged`. That is
//! not incidental: the client's stand-up builder `FUN_142cd3a60` refuses to send at all while
//! `ctx+0x2330` is set, so dropping the unlock would leave a player unable even to *ask* to
//! stand, on top of freezing every later inventory action. Both handlers below return exactly
//! what the catch-all returned and add only the bookkeeping.
//!
//! # The reply, and why both directions need one
//!
//! The owner, 2026-09-08: *"I also cannot get out of the chair, the server won't let me."* The
//! client seats itself but will not stand until the server says so - `FUN_142cd3a60` re-arms
//! the latch and returns without touching the chair. `net::chair::USER_SIT` (`0x0318`) is that
//! reply, and both handlers send it: the sit is echoed so the state the client invented is the
//! state the server agrees to, and the stand carries **two zero words**, which is the only
//! form the client treats as a release. `research/chairs-2026-09-08.md` §12.

use super::{Reply, Session};

impl Session {
    /// `0x00DB` - the player sat on a Set Up chair.
    pub(super) fn on_chair_sit(&mut self, body: &[u8]) -> Vec<Reply> {
        let unlock = crate::mesodrop::unlock_unhandled_latching_request(net::chair::CLIENT_CHAIR_SIT);
        let Some(sit) = net::chair::parse_sit(body) else {
            crate::server::log(&format!(
                "   chair: 0x00DB did not decode ({} byte body); nobody was seated",
                body.len()
            ));
            return unlock;
        };
        let mut out = unlock;
        // A Set Up chair is not a map seat; taking one leaves no bench to release later.
        self.seated_map_seat = None;
        // Echo the seat back. The client has already built its own chair object, but the
        // server is the authority the *stand* is asked of, so it must agree the player is
        // seated - and the owner's point: other players have to be told too, which is the same
        // packet in the remote user's range.
        out.push(Reply {
            opcode: net::chair::USER_SIT,
            body: net::chair::user_sit(Some(sit.item_id)),
            what: format!(
                "UserSit: seated on chair {} (slot {}). Nothing authenticates - the item id \
                 is not checked against the player's inventory.",
                sit.item_id, sit.slot
            ),
        });
        if let Some(chr) = self.claimed_character() {
            self.publish_chair(chr.id, self.field_of(&chr), Some(sit.item_id));
        }
        match self.config.chairs.get(&sit.item_id) {
            Some(chair) => {
                self.seated_chair = Some(sit.item_id);
                crate::server::log(&format!(
                    "   chair: seated on {} (slot {}), idle recovery +{} HP +{} MP per tick",
                    sit.item_id, sit.slot, chair.recovery_hp, chair.recovery_mp
                ));
            }
            None => {
                // An id we have no row for is NOT treated as a chair that restores nothing -
                // it is treated as unknown, and the most likely cause is that `gm-handbook/`
                // has not been generated. Say which, because a silent zero here reads on
                // screen as "chairs do not work".
                self.seated_chair = None;
                crate::server::log(&format!(
                    "   chair: item {} is not in the chair table ({} row(s) loaded). No recovery \
                     bonus. If that table is empty, run: python tools/dump_chairs.py",
                    sit.item_id,
                    self.config.chairs.len()
                ));
            }
        }
        out
    }

    /// `0x00DA` - the player asked to stand up, or moved to another chair.
    pub(super) fn on_chair_cancel(&mut self, body: &[u8]) -> Vec<Reply> {
        let unlock =
            crate::mesodrop::unlock_unhandled_latching_request(net::chair::CLIENT_CHAIR_CANCEL);
        let mut out = unlock;
        match net::chair::parse_cancel(body) {
            Some(None) => {
                self.seated_chair = None;
                // **Two zero words, and that is not a detail.** The handler's middle arm -
                // chair id 0 with a NON-zero second field - sets a cooldown and returns
                // WITHOUT releasing, which on screen is indistinguishable from the bug this
                // fixes. `net::chair::user_sit(None)` sends both zero and a test pins it.
                out.push(Reply {
                    opcode: net::chair::USER_SIT,
                    body: net::chair::user_sit(None),
                    what: "UserSit: RELEASE - both fields zero, the only form the client \
                           treats as standing up. Nothing authenticates."
                        .to_string(),
                });
                // **Only a player who actually took a map seat gets the `0x0252` release.**
                // The Set Up chair stand is confirmed on two screens and is left byte for
                // byte as it was; `0x0252` clears `CUser+0x3c28`, which a Set Up chair never
                // set. `bSit = 0` with the field ABSENT is the release - a `0xFFFF` written
                // into the u16 would `movzx` to 65535 and seat them on a nonexistent bench.
                let was_on_a_map_seat = self.seated_map_seat.take();
                if let Some(chr) = self.claimed_character() {
                    if let Some(seat) = was_on_a_map_seat {
                        let body = net::chair::user_sit_result(chr.id, None);
                        debug_assert_eq!(body.len(), net::chair::USER_SIT_RESULT_RELEASED_LEN);
                        out.push(Reply {
                            opcode: net::chair::USER_SIT_RESULT,
                            body,
                            what: format!(
                                "UserSitResult: character {} off map seat {seat}, 5 bytes \
                                 (u32 id, u8 bSit=0, no index). Nothing authenticates.",
                                chr.id
                            ),
                        });
                    }
                    self.publish_chair(chr.id, self.field_of(&chr), None);
                }
                crate::server::log(&format!(
                    "   chair: stood up - release sent, recovery bonus ends{}",
                    match was_on_a_map_seat {
                        Some(s) => format!(" (also 0x0252 clearing map seat {s})"),
                        None => String::new(),
                    }
                ));
            }
            Some(Some(seat)) => {
                // **A MAP chair.** The owner, 2026-09-09: *"I still cannot sit down in chairs that
                // are present in the maps themselves, such as Henesys."* Their client sent
                // `0x00DA` with body `1800` - seat index 24 - seven times and got only the
                // unlock [L]. So `0x00DA` is not only "stand up": with a real index it means
                // "seat me on map chair N", and `0xFFFF` is the absence of one.
                //
                // **`0x0318` was tried here first and REFUTED on a screen**: the reply went
                // out four times, the client retried four times and never sat. The local
                // dispatcher has no seat-index path at all - one chair arm in 218, and its
                // only `SetSeat` call is the release - so map chairs are not its opcode.
                //
                // **`0x02AD` was tried here too, and it could never have worked.** Not a wrong
                // body - a wrong *dispatcher*. `0x02AD` lives in `FUN_1429bb720`, which looks
                // the target up by going straight to the hash at `[pool+0xf8]`; the local
                // player is not in that hash. `0x0252` lives in `FUN_1429bafb0`, whose
                // `GetUser` checks `[pool+0x10]` - the local user - FIRST and returns it on an
                // id match. That one difference is the whole three-day failure.
                // `research/map-chair-seat-2026-09-09.md`.
                self.seated_chair = None;
                self.seated_map_seat = Some(seat);
                if let Some(chr) = self.claimed_character() {
                    let body = net::chair::user_sit_result(chr.id, Some(seat));
                    debug_assert_eq!(body.len(), net::chair::USER_SIT_RESULT_SEATED_LEN);
                    out.push(Reply {
                        opcode: net::chair::USER_SIT_RESULT,
                        body,
                        what: format!(
                            "UserSitResult: character {} on map seat {seat}, 7 bytes \
                             (u32 id, u8 bSit=1, u16 seat). Nothing authenticates - the seat \
                             index is not checked against the map's chair list.",
                            chr.id
                        ),
                    });
                    // The relay to everyone else stays `0x02AD`, unchanged. That path IS
                    // confirmed on two screens for Set Up chairs, and `0x0252` is deliberately
                    // NOT broadcast: the client re-validates the sitter's position against the
                    // seat and, on failure, makes **its own** player send a stand request - so
                    // a bystander holding a stale position for the sitter would stand itself
                    // up. Whether `0x02AD` renders a map seat correctly for bystanders is
                    // unmeasured, and is left as it was rather than guessed at.
                    self.publish_chair(chr.id, self.field_of(&chr), Some(u32::from(seat)));
                    crate::server::log(&format!(
                        "   chair: 0x00DA seat index {seat} - a MAP chair. Sent 0x0252 to the \
                         player (the dispatcher that can address them) and 0x02AD to the map."
                    ));
                }
            }
            None => crate::server::log(&format!(
                "   chair: 0x00DA did not decode ({} byte body)",
                body.len()
            )),
        }
        out
    }

    /// Tell everyone else on the map that this character's chair changed.
    ///
    /// The owner, 2026-09-08: *"the server needs to relay that action to all of the players in the
    /// map so other players can see you sitting in a specific chair ID as well."* `0x02AD`
    /// carries the character id, so it is the one packet that says *whose* chair changed -
    /// the local `0x0318` cannot, and that is why both are sent.
    ///
    /// `Some(chr)` supersedes on the character id, so a second sit replaces the first in the
    /// queue rather than stacking behind it.
    fn publish_chair(&mut self, character: u32, map: crate::fields::FieldKey, chair: Option<u32>) {
        // **This packet killed Tester2's client on 2026-09-09, and it is on because the BODY
        // was wrong, not the idea.** The owner: *"the chair appearance across different clients is
        // an important part of the game."*
        //
        // What went wrong: the body was built from the first 26 instructions of
        // `FUN_1429d4fd0` and stopped after two `Decode4`. `tools/reads.py` - which exists
        // because a truncated packet killed this client twice before - counts **three** reads,
        // the third a `u8` 260 bytes further in. Twelve bytes underflowed the client's
        // `Decode1`; it faulted `0xc0000005` five milliseconds after the packet went out.
        //
        // What makes it safe now, and all three were missing the first time:
        //
        // * the read count was redone **at depth 6**, so a read behind a helper would show;
        // * the pool's own head reads exactly **one** `u32` before it dispatches, so the
        //   character id is not double-counted against the arm's two;
        // * `net::chair::user_sit_remote` writes all four fields and a test asserts the length
        //   is **13**, with a note that a length failing downward makes this a killer again.
        let what = match chair {
            Some(c) => format!("UserSitRemote: character {character} sat on chair/seat {c}"),
            None => format!("UserSitRemote: character {character} stood up"),
        };
        self.bus().publish(
            self.subscriber,
            map,
            Reply {
                opcode: net::chair::USER_SIT_REMOTE,
                body: net::chair::user_sit_remote(character, chair),
                what,
            },
            Some(character),
        );
    }

    /// What the chair the player is sitting on adds to one idle tick, as `(hp, mp)`.
    ///
    /// `(0, 0)` when standing, when the chair is unknown, or when the table is missing - all
    /// of which leave the tick exactly as it was before chairs existed.
    pub(super) fn chair_recovery(&self) -> (u32, u32) {
        self.seated_chair
            .and_then(|id| self.config.chairs.get(&id))
            .map(|c| (c.recovery_hp, c.recovery_mp))
            .unwrap_or((0, 0))
    }
}
