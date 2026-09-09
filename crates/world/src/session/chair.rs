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
//! # What this does not fix, and it is the thing the owner noticed second
//!
//! The owner, 2026-09-08: *"I also cannot get out of the chair, the server won't let me."* Theirs
//! client sent **eight** `0x00DA` stand requests in four minutes and stayed seated, so leaving
//! a chair needs a server packet that has not been identified. This module clears the seated
//! state on request - so the recovery bonus stops immediately, which is the half that is ours
//! to get right - but the character stays seated **on screen** until that packet is found.
//! `research/chairs-2026-09-08.md` §4 lists what was eliminated.

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
        unlock
    }

    /// `0x00DA` - the player asked to stand up, or moved to another chair.
    pub(super) fn on_chair_cancel(&mut self, body: &[u8]) -> Vec<Reply> {
        let unlock =
            crate::mesodrop::unlock_unhandled_latching_request(net::chair::CLIENT_CHAIR_CANCEL);
        match net::chair::parse_cancel(body) {
            Some(None) => {
                if self.seated_chair.take().is_some() {
                    crate::server::log(
                        "   chair: stood up - recovery bonus ends. NOTE: the client stays seated \
                         on screen; the server-to-client packet that releases a chair has not \
                         been found. research/chairs-2026-09-08.md section 4",
                    );
                }
            }
            Some(Some(id)) => {
                // A non-`0xFFFF` id has never been captured. Recorded rather than acted on.
                crate::server::log(&format!(
                    "   chair: 0x00DA carried chair id {id} rather than 0xFFFF - never seen \
                     before, not acted on"
                ));
            }
            None => crate::server::log(&format!(
                "   chair: 0x00DA did not decode ({} byte body)",
                body.len()
            )),
        }
        unlock
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
