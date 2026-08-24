//! `0x00D5` - the Cash Shop button, answered so the button works more than once.
//!
//! # What this does NOT do
//!
//! **It does not open a cash shop.** There is no cash shop server, no `SetCashShop`, no cash
//! inventory and no wallet. `research/cash-shop.md` §4 records the shape that would be, and
//! deliberately does not build it.
//!
//! What it does is **clear the latch**, and that is worth doing on its own.
//!
//! # Why an unanswered `0x00D5` is worse than it looks
//!
//! `0x00D5` is an **exclusive request**: `FUN_142caee70` checks `[ctx+0x2338]`,
//! `[ctx+0x2330]` and a 500 ms stamp at `[ctx+0x2334]` before it sends, and sets the latch
//! when it does. Measured across three clicks five seconds apart on 2026-08-22:
//!
//! ```text
//! click 1  [ctx+0x2330] = 0  -> sent 0x00D5, latch now 1
//! click 2  [ctx+0x2330] = 1  -> silent return
//! click 3  [ctx+0x2330] = 1  -> silent return
//! ```
//!
//! So an unanswered request costs **every later click of the session**, and `[ctx+0x2330]` is
//! the same field `research/pick-up-latch.md` shows gating the pick-up sweep. A latch left set
//! here is not confined to the Cash Shop button.
//!
//! # The reply, and why it is a `0x0070`
//!
//! `inventory_rejected()` - `bExclRequestSent = 1`, `nCount = 0` - is this project's known
//! clearer for `[ctx+0x2330]`, used on every refusal path in `ground.rs` and `inventory.rs`.
//! It changes nothing and releases the latch.
//!
//! It is **not** the reply a real server would send; that would be a migrate. It is labelled
//! as a latch-clear everywhere it appears, and the outcome is falsifiable on one run: if the
//! log shows **one `0x00D5` per click** instead of one per session, it worked.

use super::*;

impl Session {
    /// `0x00D5` - the player clicked Cash Shop.
    pub(super) fn on_cash_shop_request(&mut self, body: &[u8]) -> Vec<Reply> {
        let what = match net::cashshop::parse_cash_shop_request(body) {
            Some(req) => format!(
                "client tick {}, flag {} - the Cash Shop button. There is no cash shop server, \
                 so this ONLY clears the exclusive-request latch at ctx+0x2330",
                req.tick, req.flag
            ),
            // Answer anyway. The latch is set by the *builder*, before we ever see the body,
            // so a body we cannot read still costs the player every later click.
            None => format!(
                "unreadable {}-byte body {body:02x?}, expected {} - answered anyway, because \
                 the latch was set when the client sent it and not when we understood it",
                body.len(),
                net::cashshop::CASH_SHOP_REQUEST_LEN
            ),
        };

        let mut out = vec![Reply {
            opcode: net::inventory::INVENTORY_OPERATION,
            body: net::inventory::inventory_rejected(),
            what: format!(
                "InventoryOperation: clearing ctx+0x2330 after 0x00D5 - {what}. Without this \
                 the button fires ONCE per session and every later click returns silently, \
                 measured 2026-08-22 with a peek on that field"
            ),
        }];
        out.extend(self.notice(
            "The Cash Shop is not available on this server.".to_string(),
        ));
        out
    }
}
