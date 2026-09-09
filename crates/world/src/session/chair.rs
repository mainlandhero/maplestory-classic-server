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
                crate::server::log("   chair: stood up - release sent, recovery bonus ends");
            }
            Some(Some(seat)) => {
                // **A MAP chair.** The owner, 2026-09-09: *"I still cannot sit down in chairs that
                // are present in the maps themselves, such as Henesys."* Their client sent
                // `0x00DA` with body `1800` - seat index 24 - seven times and got only the
                // unlock [L]. So `0x00DA` is not only "stand up": with a real index it means
                // "seat me on map chair N", and `0xFFFF` is the absence of one.
                //
                // **This reply is a HYPOTHESIS and is labelled one.** `0x0318` is the only
                // chair opcode the local user's dispatcher has - `FUN_14289a3a0`'s 218-entry
                // table has exactly one arm that touches a chair, index 83 - and there is no
                // seat-index path anywhere in it. So either the index rides that packet's
                // first field, or the map-chair reply lives outside that dispatcher and has
                // not been found. Sending it risks nothing already held: today the client
                // gets the unlock and stays standing.
                //
                // Reading the run: the player sits on the bench -> confirmed. Nothing happens
                // -> map chairs are not this opcode, and the search moves outside the local
                // user's table.
                self.seated_chair = None;
                out.push(Reply {
                    opcode: net::chair::USER_SIT,
                    body: net::chair::user_sit(Some(u32::from(seat))),
                    what: format!(
                        "UserSit: MAP chair, seat index {seat} - HYPOTHESIS, see \
                         session/chair.rs. Nothing authenticates."
                    ),
                });
                crate::server::log(&format!(
                    "   chair: 0x00DA carried seat index {seat} - a MAP chair, not a stand. \
                     Replying 0x0318 with it as the chair id. UNTESTED."
                ));
            }
            None => crate::server::log(&format!(
                "   chair: 0x00DA did not decode ({} byte body)",
                body.len()
            )),
        }
        out
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
