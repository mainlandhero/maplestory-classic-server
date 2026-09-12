//! `0x0165` - **the Beauty Coupon dialog's CONFIRM**: "change my hair (or face) to this".
//!
//! The owner, 2026-09-12: *"I tried using the Ubel Hair Coupon but my hair did not change."*
//! `world.log` has the press, twice, unanswered:
//!
//! ```text
//! 04:19:50.442 <- 0x0165 UNKNOWN, 8 byte body 130027ce26000000
//!                                             1300      u16 slot 19   (the coupon's Use-tab slot)
//!                                                 27ce2600  u32 2543143  Übel Hair Coupon
//!                                                         0000  u16 0   not established
//! ```
//!
//! The builder is `FUN_142dc8100` - the same function that fetches the dialog's sentence
//! (string `0x0464`) - and `research/msexe-packet-fields.txt` gives its encode order as
//! `u16, u32, u16, u8`-ish helpers; the capture is 8 bytes, so the trailing field is two
//! bytes here. It is carried through and not interpreted.
//!
//! The dialog itself is client-side (it previews the cosmetic from the coupon's
//! `spec/cosmetic`), so the server's job is the decision: is that coupon in that slot, what
//! cosmetic does it name, apply it, consume it, and make the client redraw.

use crate::packet::PacketReader;

/// The Beauty Coupon confirm opcode.
pub const CLIENT_BEAUTY_COUPON_CONFIRM: u16 = 0x0165;

/// The captured body length.
pub const BEAUTY_COUPON_CONFIRM_LEN: usize = 8;

/// A parsed [`CLIENT_BEAUTY_COUPON_CONFIRM`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeautyCouponConfirm {
    /// The coupon's slot in the Use tab, 1-based.
    pub slot: u16,
    pub item_id: u32,
    /// Whatever follows the item id. `[0, 0]` in both captures; meaning not established.
    pub tail: Vec<u8>,
}

/// Parse the body after the opcode. `None` if it is shorter than slot + item id.
pub fn parse_beauty_coupon_confirm(body: &[u8]) -> Option<BeautyCouponConfirm> {
    let mut r = PacketReader::new(body);
    let slot = r.u16().ok()?;
    let item_id = r.u32().ok()?;
    Some(BeautyCouponConfirm { slot, item_id, tail: r.rest().to_vec() })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The owner's press, byte for byte.
    #[test]
    fn the_captured_confirm_parses() {
        let body = [0x13, 0x00, 0x27, 0xce, 0x26, 0x00, 0x00, 0x00];
        assert_eq!(body.len(), BEAUTY_COUPON_CONFIRM_LEN);
        let r = parse_beauty_coupon_confirm(&body).unwrap();
        assert_eq!(r.slot, 19);
        assert_eq!(r.item_id, 2_543_143, "the Übel Hair Coupon");
        assert_eq!(r.tail, vec![0, 0]);
        for n in 0..6 {
            assert_eq!(parse_beauty_coupon_confirm(&body[..n]), None, "{n} bytes parsed");
        }
        assert!(parse_beauty_coupon_confirm(&body[..6]).is_some(), "the tail is optional");
    }
}
