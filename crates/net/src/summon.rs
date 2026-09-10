//! `0x0111` - the client using a **summoning sack**.
//!
//! The owner, 2026-09-09: *"I just also tried summoning the GM Black Sack Jr. Balrog lvl 80"*. The
//! packet arrived and nothing handled it. `world.log`:
//!
//! ```text
//! 01:26:17.632 <- 0x0111 UNKNOWN, 10 byte body 466f15010b00260b2000
//! ```
//!
//! `research/summon-sacks-2026-09-09.md` decoded it the same day, off the builder
//! `FUN_142ccb0f0`, and its answer table already said the server did not handle it.
//!
//! # It is `0x0111` and NOT `0x010E`, and that was established rather than assumed
//!
//! `FUN_1404169b0` is the predicate that gates `0x010E`, and the `0210` prefix is **not in
//! it**; the item-use dispatcher sends `2100000..2109999` to `FUN_142ccb0f0` instead. **[L]**
//!
//! # There is no trailing `u32`
//!
//! `0x010E` has one; this does not. **A parser copied from `useitem.rs` with its
//! `len() < 14` guard would reject every real `0x0111`** - which is exactly the shape of
//! mistake this module's length constant exists to prevent.

/// Client -> server: "I used the summoning sack in this slot."
pub const CLIENT_SUMMON_SACK: u16 = 0x0111;

/// The body length. **Ten, not fourteen** - see the module docs.
pub const SUMMON_SACK_BODY_LEN: usize = 10;

/// A decoded [`CLIENT_SUMMON_SACK`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SummonSackRequest {
    /// `FUN_1429e3ef0()`. Nothing reads it.
    pub tick: u32,
    /// The sack's slot in the **Use** tab, 1-based.
    pub slot: u16,
    /// The sack's item id. The client sends both this and the slot; a server that trusts one
    /// without checking the other lets a crafted packet summon from a slot holding anything.
    pub item_id: u32,
}

/// Decode a [`CLIENT_SUMMON_SACK`] body. `None` when it is not the length the client sends.
pub fn parse_summon_sack(body: &[u8]) -> Option<SummonSackRequest> {
    if body.len() < SUMMON_SACK_BODY_LEN {
        return None;
    }
    Some(SummonSackRequest {
        tick: u32::from_le_bytes([body[0], body[1], body[2], body[3]]),
        slot: u16::from_le_bytes([body[4], body[5]]),
        item_id: u32::from_le_bytes([body[6], body[7], body[8], body[9]]),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The owner's own packet**, byte for byte out of `world.log`, decoded to what they were doing:
    /// using the Jr. Balrog sack out of Use slot 11.
    ///
    /// A capture is the only thing that can disagree with a field-offset table, so the table
    /// is held against one rather than against itself.
    #[test]
    fn the_captured_request_decodes_to_the_sack_that_was_used() {
        // 01:26:17.632 <- 0x0111 UNKNOWN, 10 byte body 466f15010b00260b2000
        let body = [0x46, 0x6f, 0x15, 0x01, 0x0b, 0x00, 0x26, 0x0b, 0x20, 0x00];
        let r = parse_summon_sack(&body).expect("10 bytes is the length the client sends");
        assert_eq!(r.tick, 0x0115_6f46);
        assert_eq!(r.slot, 11, "the Use slot the sack was in");
        assert_eq!(r.item_id, 2_100_006, "GM Black Sack: Jr. Balrog Level 80");
    }

    /// **Ten bytes, not fourteen.** A parser copied from the `0x010E` one would reject every
    /// real request, and the symptom would be "the sack does nothing" - which is exactly what
    /// it already did for a different reason, so the two would be indistinguishable.
    #[test]
    fn ten_bytes_is_enough_and_nine_is_not() {
        assert_eq!(SUMMON_SACK_BODY_LEN, 10);
        assert!(parse_summon_sack(&[0u8; 10]).is_some());
        for n in 0..SUMMON_SACK_BODY_LEN {
            assert_eq!(parse_summon_sack(&[0u8; 10][..n]), None, "{n} bytes");
        }
        // A longer body still parses - the length is a minimum, and refusing a trailing byte
        // would be a guess about a client that has only ever been seen sending ten.
        assert!(parse_summon_sack(&[0u8; 14]).is_some());
    }
}
