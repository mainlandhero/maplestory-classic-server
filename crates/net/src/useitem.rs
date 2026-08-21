//! `0x010E` — the player double-clicked a consumable.
//!
//! The owner, 2026-08-21: *"Using a consumable item such as Roger's Apple does not recover HP. I
//! tried to consume Red Potion, but it did not recover 100 HP."*
//!
//! It did not, because nothing answered the request. The packet was on the wire and logged
//! `UNKNOWN`:
//!
//! ```text
//! <- 0x010E  14 byte body  f7e1140f 0100 80841e00 01000000
//!                          ^tick    ^slot ^itemId  ^tail
//! ```
//!
//! `0x001e8480` is **2000000**, the Red Potion, and slot 1 is where the database says theirs
//! Red Potions were. **[L]** — the id and the slot are both checkable against state this
//! server wrote, which is what makes this decode more than a plausible field split.
//!
//! # Once per session, until it is answered
//!
//! **There is exactly one `0x010E` in the whole run**, and the owner says they tried it more than
//! once. That is the same shape as `0x0107` and the two ability-point requests: the client
//! latches on send and an inbound handler has to clear it. So this is answered on **every**
//! path, refusals included, exactly like [`crate::inventory`].
//!
//! The latch has not been located for this opcode specifically, so that is **[I]** by
//! analogy — but the analogy is with three measured cases, and the cost of being wrong in
//! the other direction is a dead item window for the rest of the session.
//!
//! # What it restores is in `spec`, not `info`
//!
//! `gm-handbook/consumables.txt`, generated from the client's own `Item.wz`: the Red Potion's
//! `spec/hp` is **100** and Roger's Apple is **30**, which is what its own WZ description
//! says it does. `hp`/`mp` are flat amounts; `hpR`/`mpR` are **percentages of the maximum**.
//! Confusing those two is the "unit, not the arithmetic" mistake this project has made three
//! times.

use crate::packet::PacketWriter;

/// The client asking to use an item out of the Use tab. **Inbound `0x010E`, 14 bytes.**
pub const CLIENT_USE_ITEM: u16 = 0x010E;

/// A parsed [`CLIENT_USE_ITEM`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UseItem {
    pub tick: u32,
    /// The Use-tab slot, 1-based.
    pub slot: i16,
    /// The item id the client believes is in that slot. **Checked, not trusted** — the
    /// server holds the bag and the two can disagree.
    pub item_id: u32,
    /// The trailing `u32`, `1` in the one capture. Its meaning is **not established**, so it
    /// is carried rather than named: a field with a guessed name is worse than a numbered
    /// one, because the guess gets quoted later as though it were read.
    pub tail: u32,
}

/// Parse a [`CLIENT_USE_ITEM`] body (opcode already stripped).
///
/// Short bodies come off a socket, so this returns `None` rather than panicking - and the
/// caller still answers, because silence is what latches the client.
pub fn parse_use_item(body: &[u8]) -> Option<UseItem> {
    if body.len() < 14 {
        return None;
    }
    Some(UseItem {
        tick: u32::from_le_bytes([body[0], body[1], body[2], body[3]]),
        slot: i16::from_le_bytes([body[4], body[5]]),
        item_id: u32::from_le_bytes([body[6], body[7], body[8], body[9]]),
        tail: u32::from_le_bytes([body[10], body[11], body[12], body[13]]),
    })
}

/// Build a [`CLIENT_USE_ITEM`] body. Exists so the tests have a round trip, and so a probe
/// can replay one without hand-assembling bytes.
pub fn use_item(tick: u32, slot: i16, item_id: u32, tail: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(tick);
    w.i16(slot);
    w.u32(item_id);
    w.u32(tail);
    w.into_vec()
}

/// Length of a [`use_item`] body.
pub const USE_ITEM_LEN: usize = 14;

#[cfg(test)]
mod tests {
    use super::*;

    /// The real capture, byte for byte.
    #[test]
    fn the_captured_red_potion_use_decodes() {
        let body = [
            0xf7, 0xe1, 0x14, 0x0f, // tick
            0x01, 0x00, // slot 1
            0x80, 0x84, 0x1e, 0x00, // 2000000
            0x01, 0x00, 0x00, 0x00, // tail
        ];
        let m = parse_use_item(&body).expect("14 bytes is a whole body");
        assert_eq!(m.slot, 1);
        assert_eq!(m.item_id, 2_000_000, "the Red Potion");
        assert_eq!(m.tail, 1);
        assert_eq!(use_item(m.tick, m.slot, m.item_id, m.tail), body);
        assert_eq!(body.len(), USE_ITEM_LEN);
    }

    /// Every truncation returns `None` instead of panicking.
    #[test]
    fn short_bodies_are_refused_rather_than_read_past_the_end() {
        let body = use_item(1, 1, 2_000_000, 1);
        for n in 0..body.len() {
            assert!(parse_use_item(&body[..n]).is_none(), "len {n}");
        }
        assert!(parse_use_item(&body).is_some());
    }
}
