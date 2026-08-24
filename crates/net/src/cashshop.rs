//! `0x00D5` - the Cash Shop button, which has been on the wire twice and was read as noise.
//!
//! # The correction, first, because it is the whole point of this file
//!
//! For two sessions this project reported that clicking Cash Shop **sent nothing**. It did.
//! `0x00D5` was in `world.log` both times, and it was dismissed because it arrives inside a
//! burst that also carries `0x0420`..`0x0426`, and that burst lands near the end of a session:
//!
//! ```text
//! 03:19:39.284  <- 0x0422     7 bytes
//! 03:19:39.293  <- 0x0421  1114 bytes
//! 03:19:39.293  <- 0x0420   133 bytes
//! 03:19:39.293  <- 0x0423   274 bytes
//! 03:19:39.293  <- 0x0426    20 bytes
//! 03:19:39.293  <- 0x00D5     5 bytes   e929ba0500
//! ```
//!
//! The whole burst was filed as "shutdown telemetry" **as a unit**, without separating the
//! opcodes in it. The control that breaks it costs one `grep` over the archived runs:
//!
//! ```text
//!                        0x00D5   0x0420
//!   five runs, no Cash Shop click     0        0..2      <- 0x0420 IS telemetry
//!   the two runs with a click         1        1         <- 0x00D5 only ever appears here
//! ```
//!
//! **[L]** `0x0420` appears without `0x00D5`; `0x00D5` never appears without a click. That is
//! `CLAUDE.md`'s "enumerate before you filter" exactly - the filter was *when it arrived*
//! rather than *which opcode it was*, and it produced a clean, confident, wrong negative that
//! stood for two days.
//!
//! # It is an EXCLUSIVE REQUEST, which is why only the first click of a session was ever seen
//!
//! `FUN_142caee70` - the sender, confirmed by a watch that fired on every click - checks the
//! same three fields `FUN_142cc42d0` gates every exclusive request on, inline: **[L]**
//!
//! ```asm
//! 142caef7c  cmp [ctx+0x2338], 0  ; jne -> silent return
//! 142caef88  cmp [ctx+0x2330], 0  ; jne -> silent return   <- the exclusive-request latch
//! 142caef9e  tick - [ctx+0x2334] < 0x1f4 -> silent return  <- 500 ms
//! ```
//!
//! And a peek on `[ctx+0x2330]` across three clicks five seconds apart says the rest:
//!
//! ```text
//! click 1  23:19:39.284  [ctx+0x2330] = 0   -> passed, sent 0x00D5, and SET the latch
//! click 2  23:19:44.915  [ctx+0x2330] = 1   -> silent return
//! click 3  23:19:50.866  [ctx+0x2330] = 1   -> silent return
//! ```
//!
//! **[L]** So the button is not broken and never was: it fires once, latches, and waits for a
//! reply that never comes. `research/cash-shop.md` part six.

use crate::packet::PacketReader;

/// `0x00D5` - "take me to the Cash Shop".
///
/// Built by `FUN_142caee70`, whose `COutPacket` constructor at `0x142caf180` is recorded as
/// this opcode in `research/msexe-send-opcodes.txt`. The watch that fired on that function on
/// every click, and the `0x00D5` that appeared on the same millisecond, are two independent
/// readings of the same event. **[L]**
pub const CLIENT_CASH_SHOP_REQUEST: u16 = 0x00D5;

/// A decoded [`CLIENT_CASH_SHOP_REQUEST`]. Five bytes: a client tick and one byte.
///
/// `e9 29 ba 05 | 00` from the 2026-08-22 capture. The field order is `w_u32` then `w_u8`,
/// which is what `tools/encodes.py` reads out of the builder at `0x142caf192` and
/// `0x142caf1a0` - so the split is measured rather than a plausible reading of five bytes.
/// **[L]**
///
/// The server needs neither field to decide anything today; they are parsed so the log can
/// show them and so a body of the wrong length is noticed rather than assumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CashShopRequest {
    /// The client's own millisecond tick, the same clock `0x013C` carries.
    pub tick: u32,
    pub flag: u8,
}

/// Body length of a [`CLIENT_CASH_SHOP_REQUEST`].
pub const CASH_SHOP_REQUEST_LEN: usize = 5;

/// Parse a `0x00D5` body (opcode already stripped). `None` if it is not five bytes.
pub fn parse_cash_shop_request(body: &[u8]) -> Option<CashShopRequest> {
    let mut r = PacketReader::new(body);
    let tick = r.u32().ok()?;
    let flag = r.u8().ok()?;
    Some(CashShopRequest { tick, flag })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The owner's real `0x00D5`, off the wire.** 2026-08-22, the first click of the session.
    #[test]
    fn the_real_request_parses() {
        let body = [0xe9, 0x29, 0xba, 0x05, 0x00];
        assert_eq!(body.len(), CASH_SHOP_REQUEST_LEN);
        let r = parse_cash_shop_request(&body).expect("five bytes");
        assert_eq!(r.tick, 0x05ba29e9, "w_u32 at 0x142caf192");
        assert_eq!(r.flag, 0, "w_u8 at 0x142caf1a0");

        // A body of the wrong length is refused rather than read past the end or padded.
        assert!(parse_cash_shop_request(&body[..4]).is_none());
        assert!(parse_cash_shop_request(&[]).is_none());
    }
}
