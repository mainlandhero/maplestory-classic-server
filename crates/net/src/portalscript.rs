//! `0x014A` - **the script-portal request**. A `pt 7` or `pt 8` portal never sends the
//! ordinary transfer-field request; it sends this.
//!
//! The owner, 2026-09-10, the morning after the Ellinia Station door's destination went into the
//! portal table: *"The portal in Ellinia to go to Ellinia Station still currently does not
//! exist."* Their two presses are in `world.log`:
//!
//! ```text
//! 11:27:53.035 <- 0x014A UNKNOWN, 11 byte body 000400696e30333a0303f4
//!
//!   00             u8    0
//!   0400 696e3033  str   "in03"           u16 length, then the bytes
//!   3a03           i16   826              the character's x
//!   03f4           i16   -3069            and y; portal 38 is at (819, -3072)
//! ```
//!
//! Twelve captures across three archived runs plus this one, every one `"in03"`, and the
//! client's own builder `FUN_1428b2330` encodes `u8, str, i16, i16`
//! (`research/msexe-packet-fields.txt`). So the shape is read from the client and the
//! captures agree with it. `research/script-portal-request-2026-09-10.md`.
//!
//! What the leading byte means is not established; it has been `0` in all twelve. It is
//! carried through, not interpreted.
//!
//! The Free Market's four doors are `pt 7` and send this too, which is why that wiring
//! looked as dead as the station's. `world::session::field::on_portal_script` resolves both
//! through the same lookup a walked door uses.

use crate::packet::PacketReader;

/// The script-portal request opcode.
pub const CLIENT_PORTAL_SCRIPT: u16 = 0x014A;

/// What the client asked for in a [`CLIENT_PORTAL_SCRIPT`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortalScriptRequest {
    /// The leading byte. `0` in every capture; meaning not established.
    pub flag: u8,
    /// The portal's `pn` on the current map - the key into the portal table.
    pub portal_name: String,
    /// Where the character was standing when they pressed up.
    pub x: i16,
    pub y: i16,
}

/// Parse a [`CLIENT_PORTAL_SCRIPT`] body (opcode already stripped). `None` if it is too
/// short for the four fields.
pub fn parse_portal_script(body: &[u8]) -> Option<PortalScriptRequest> {
    let mut r = PacketReader::new(body);
    let flag = r.u8().ok()?;
    let portal_name = r.str().ok()?;
    let x = r.i16().ok()?;
    let y = r.i16().ok()?;
    Some(PortalScriptRequest { flag, portal_name, x, y })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The owner's first press, byte for byte.
    #[test]
    fn the_captured_press_parses() {
        let body = [0x00, 0x04, 0x00, b'i', b'n', b'0', b'3', 0x3a, 0x03, 0x03, 0xf4];
        assert_eq!(
            parse_portal_script(&body),
            Some(PortalScriptRequest { flag: 0, portal_name: "in03".to_string(), x: 826, y: -3069 })
        );
        // The second press, 0.9 s later, a few pixels along.
        let body = [0x00, 0x04, 0x00, b'i', b'n', b'0', b'3', 0x3f, 0x03, 0x02, 0xf4];
        let r = parse_portal_script(&body).unwrap();
        assert_eq!((r.x, r.y), (831, -3070));
    }

    /// **The y is signed.** `0xf403` as a `u16` is 62467; the portal is at -3072, so an
    /// unsigned read would put the press four screens away from the portal and any future
    /// distance check would refuse every real press.
    #[test]
    fn the_position_is_signed() {
        let r = parse_portal_script(&[0, 1, 0, b'a', 0xff, 0xff, 0x00, 0x80]).unwrap();
        assert_eq!((r.x, r.y), (-1, i16::MIN));
    }

    /// Every truncation is refused rather than panicked on - the same rule as every other
    /// parser here, and this one is fed by the network.
    #[test]
    fn every_truncation_is_refused() {
        let body = [0x00, 0x04, 0x00, b'i', b'n', b'0', b'3', 0x3a, 0x03, 0x03, 0xf4];
        for n in 0..body.len() {
            assert_eq!(parse_portal_script(&body[..n]), None, "{n} bytes parsed");
        }
        assert!(parse_portal_script(&body).is_some(), "and the whole thing parses");
        // A length prefix that runs past the end is a refusal, not a read past the slice.
        assert_eq!(parse_portal_script(&[0x00, 0x09, 0x00, b'i', b'n']), None);
    }
}
