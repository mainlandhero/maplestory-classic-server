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

/// **A Return Scroll being used.** The client never sends [`CLIENT_USE_ITEM`] for a
/// `2030000..2039999` item: both of its item-use dispatchers (`FUN_141784fa0` for a bag
/// double-click, `FUN_1428af6d0` for the quick slot) route that range to their own builder,
/// `FUN_142d4be40`, which writes **`0x0123`** with the same three fields - `u32 tick, u16
/// slot, u32 itemId` - and then sets the exclusive-request latch (`FUN_142cc4430(ctx, 1)`).
/// `research/summon-sacks-2026-09-09.md` §3.1 read the routing; the live server's logs of
/// 2026-09-18 had fifty `0x010E`s and not one naming a scroll. The handler existed for
/// three weeks on the wrong opcode - CLAUDE.md's "built is not wired". **[L]**
///
/// Before sending, the builder itself refuses on some fields (`0x0F25` *"Return Stones
/// cannot be used here."* for `2030023`, `0x00B8` *"cannot use that in this map"* for the
/// rest) - so a scroll that produces no packet at all is the client's own map rule, not
/// this server's.
pub const CLIENT_USE_RETURN_SCROLL: u16 = 0x0123;

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

/// Length of a [`CLIENT_USE_RETURN_SCROLL`] body: **ten, not fourteen.**
///
/// `0x010E` carries a trailing `u32` after the item id; `0x0123` does not, and this is the
/// difference that broke the feature. Measured on the wire, three captures, every one ten
/// bytes:
///
/// ```text
/// world-ch0.log      <- 0x0123  10 byte body  cf80f90c 1600 b2f91e00   slot 22, 2030002
/// Server Investigation  ...     10 byte body  f904ab1d 0300 b2f91e00   slot  3, 2030002
/// Server Investigation  ...     10 byte body  06aa531d 0d00 b4f91e00   slot 13, 2030004
/// ```
///
/// 2030002 is the Return Scroll to Ellinia - the one the owner used when they reported this.
pub const USE_RETURN_SCROLL_LEN: usize = 10;

/// Parse a [`CLIENT_USE_RETURN_SCROLL`] body (opcode already stripped).
///
/// # Why this is not [`parse_use_item`]
///
/// Until 2026-09-21 it was. `session/consume.rs` forwarded `0x0123` straight into the
/// `0x010E` walk, whose parser has a `len() < 14` guard, so **every** return scroll the
/// client actually sent was refused with *"the 0x010E body did not parse"* - which is the
/// message the owner saw on screen.
///
/// The tests did not catch it because every one of them built its request with
/// [`use_item`], a fourteen-byte body the client never sends for this opcode. `CLAUDE.md`:
/// *a test that pins what the code already does is not a check.*
///
/// [`UseItem::tail`] is reported as `0`: this body has no such field, and inventing a value
/// would be a guess wearing a measurement's clothes. Nothing on the scroll path reads it.
pub fn parse_use_return_scroll(body: &[u8]) -> Option<UseItem> {
    if body.len() < USE_RETURN_SCROLL_LEN {
        return None;
    }
    Some(UseItem {
        tick: u32::from_le_bytes([body[0], body[1], body[2], body[3]]),
        slot: i16::from_le_bytes([body[4], body[5]]),
        item_id: u32::from_le_bytes([body[6], body[7], body[8], body[9]]),
        tail: 0,
    })
}

/// Build a [`CLIENT_USE_RETURN_SCROLL`] body - ten bytes, so a test cannot accidentally
/// exercise the fourteen-byte shape the client does not send.
pub fn use_return_scroll(tick: u32, slot: i16, item_id: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(tick);
    w.i16(slot);
    w.u32(item_id);
    w.into_vec()
}

#[cfg(test)]
mod return_scroll_tests {
    use super::*;

    /// **The three bodies that were actually on the wire**, pasted from the logs rather than
    /// rebuilt, because rebuilding them is exactly how this bug survived its own tests.
    #[test]
    fn the_captured_return_scroll_bodies_parse_to_the_scroll_that_was_used() {
        for (hex, slot, item) in [
            ("cf80f90c1600b2f91e00", 22i16, 2_030_002u32),
            ("f904ab1d0300b2f91e00", 3, 2_030_002),
            ("06aa531d0d00b4f91e00", 13, 2_030_004),
        ] {
            let body: Vec<u8> = (0..hex.len() / 2).map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap()).collect();
            assert_eq!(body.len(), USE_RETURN_SCROLL_LEN, "{hex}");
            let got = parse_use_return_scroll(&body).unwrap_or_else(|| panic!("{hex} must parse"));
            assert_eq!((got.slot, got.item_id), (slot, item), "{hex}");
            // The old path is the bug, stated as an assertion: the 0x010E parser refuses it.
            assert_eq!(parse_use_item(&body), None, "{hex} is too short for the 0x010E shape");
        }
    }

    #[test]
    fn a_short_return_scroll_body_is_refused_rather_than_panicked_on() {
        let full = use_return_scroll(7, 3, 2_030_002);
        assert_eq!(full.len(), USE_RETURN_SCROLL_LEN);
        assert_eq!(parse_use_return_scroll(&full), Some(UseItem { tick: 7, slot: 3, item_id: 2_030_002, tail: 0 }));
        for n in 0..USE_RETURN_SCROLL_LEN {
            assert_eq!(parse_use_return_scroll(&full[..n]), None, "len {n}");
        }
    }
}

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
