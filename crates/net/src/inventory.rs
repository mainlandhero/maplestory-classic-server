//! Moving an item: the unequip drag, and the reply that makes it happen.
//!
//! Layout and every address: `research/msexe-setfield.md` (the file name is a fossil - that
//! document is the `0x0070` decode, and its own title says so).

use crate::PacketWriter;

/// The client asking to move an item. **Inbound `0x0107`, 11 bytes.**
///
/// ```text
/// u32 tick
/// u8  invType     1 Equip, 2 Consume, 3 Install, 4 Etc, 5 Cash, 6 Decoration
/// i16 src         NEGATIVE means an equipped slot
/// i16 dst
/// i16 count       -1 for a non-bundle item
/// ```
///
/// **This reached the wire for the first time on 2026-08-19**, after months of not doing so:
///
/// ```text
/// <- 0x0107, 11 byte body  06ad3c07 01 fbff 0100 ffff
///                          tick     ^  ^    ^    ^ count -1
///                          invType 1 |    dst 1
///                                    src -5  (equipped slot 5, the coat)
/// ```
///
/// Two watches settled that it was not being dropped client-side: `0x142cc5b00` (the send
/// builder) fired, and `0x142cc5c16` (its common bail, covering all six pre-send gates)
/// fired **zero** times. So the client asked, passed every gate, and sent - and the server
/// said nothing back, which is why the item never moved.
pub const CLIENT_INVENTORY_MOVE: u16 = 0x0107;

/// The reply that actually moves the item: **`0x0070` InventoryOperation**.
///
/// The one opcode name in the whole game-stage table that is *read out of this client*
/// rather than aligned against another version's enum - `FUN_142d51930` is
/// `CWvsContext::OnInventoryOperation`.
pub const INVENTORY_OPERATION: u16 = 0x0070;

/// Entry mode 2: move an item from one slot to another.
pub const MODE_MOVE: u8 = 2;

/// `invType` 1. A negative slot on this type is an equipped slot.
pub const INV_EQUIP: i8 = 1;

/// A parsed [`CLIENT_INVENTORY_MOVE`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InventoryMove {
    pub tick: u32,
    pub inv_type: i8,
    pub src: i16,
    pub dst: i16,
    pub count: i16,
}

impl InventoryMove {
    /// An unequip is a move **out of** a negative slot on the equip inventory.
    pub fn is_unequip(&self) -> bool {
        self.inv_type == INV_EQUIP && self.src < 0 && self.dst > 0
    }

    /// Which equipped slot this takes the item off, if it is an unequip.
    pub fn equipped_slot(&self) -> Option<u8> {
        self.is_unequip().then(|| u8::try_from(-i32::from(self.src)).ok())?
    }
}

/// Parse a [`CLIENT_INVENTORY_MOVE`] body (opcode already stripped).
pub fn parse_inventory_move(body: &[u8]) -> Option<InventoryMove> {
    if body.len() < 11 {
        return None;
    }
    Some(InventoryMove {
        tick: u32::from_le_bytes([body[0], body[1], body[2], body[3]]),
        inv_type: body[4] as i8,
        src: i16::from_le_bytes([body[5], body[6]]),
        dst: i16::from_le_bytes([body[7], body[8]]),
        count: i16::from_le_bytes([body[9], body[10]]),
    })
}

/// Length of an [`inventory_move_result`] body: header 7, one entry 6, one trailing byte.
pub const INVENTORY_MOVE_RESULT_LEN: usize = 7 + 6 + 1;

/// Tell the client to perform a move it asked for.
///
/// ```text
/// u8  bExclRequestSent = 1     unlocks the UI - see below
/// u8  0                        unknown; the reference always sends 0
/// i32 nCount = 1               a 4-BYTE count, not the u8 of older builds
/// u8  notRemoveAddInfo = 0
/// --- entry ---
/// u8  mode = 2                 Move
/// i8  invType
/// i16 oldPos
/// i16 newPos
/// --- trailing ---
/// u8  0                        read ONLY because this entry sets avatarChanged
/// ```
///
/// # The first byte is the important one
///
/// `bExclRequestSent != 0` runs `FUN_142cc4430(this, 0)`, which stores `0` at
/// `this+0x2330` - **the one-request-outstanding latch**. `FUN_142cc5b00` sets that latch to
/// `1` immediately after sending `0x0107` (`142cc5f01`), and gate 2 at `142cc5b5d` refuses
/// every later request while it is set. 37 functions set it; only 7 clear it, and all seven
/// are inbound packet handlers.
///
/// So **an unanswered `0x0107` does not just fail to move one item - it silently blocks every
/// subsequent inventory action for the rest of the session.** Same failure class as
/// `world->[0x33f4]` and Log Out. Sending `1` here is what unlocks it.
///
/// # The trailing byte is conditional and this entry earns it
///
/// It is read only when `avatarChanged` is set, and mode 2 sets that on the wire values
/// alone when `(invType == 1 || invType == 6) && (oldPos < 0 || newPos < 0)`. An unequip is
/// exactly that case, so the byte is always appended here. `research/msexe-setfield.md`
/// warns against driving it from mode 3, where the flag depends on client-side inventory
/// state the server cannot see - this builder never does.
pub fn inventory_move_result(inv_type: i8, old_pos: i16, new_pos: i16) -> Vec<u8> {
    debug_assert!(
        old_pos < 0 || new_pos < 0,
        "the trailing byte is appended unconditionally, and mode 2 only earns it when one \
         side of the move is an equipped slot"
    );
    let mut w = PacketWriter::new();
    w.u8(1); // bExclRequestSent - clears the +0x2330 latch
    w.u8(0);
    w.u32(1); // nCount, i32
    w.u8(0); // notRemoveAddInfo
    w.u8(MODE_MOVE);
    w.u8(inv_type as u8);
    w.i16(old_pos);
    w.i16(new_pos);
    w.u8(0); // avatarChanged tail
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The real capture, byte for byte.
    #[test]
    fn the_captured_unequip_parses() {
        let body = [
            0x06, 0xad, 0x3c, 0x07, // tick
            0x01, // invType 1 = Equip
            0xfb, 0xff, // src -5
            0x01, 0x00, // dst 1
            0xff, 0xff, // count -1
        ];
        let m = parse_inventory_move(&body).expect("11 bytes is a whole body");
        assert_eq!(m.inv_type, INV_EQUIP);
        assert_eq!(m.src, -5);
        assert_eq!(m.dst, 1);
        assert_eq!(m.count, -1);
        assert!(m.is_unequip());
        assert_eq!(m.equipped_slot(), Some(5), "slot 5 is the coat");
    }

    #[test]
    fn a_short_body_is_none_rather_than_a_panic() {
        assert!(parse_inventory_move(&[0u8; 10]).is_none());
    }

    /// Equipping is the same packet the other way round, and must NOT be read as an unequip.
    #[test]
    fn a_move_into_an_equipped_slot_is_not_an_unequip() {
        let m = InventoryMove { tick: 0, inv_type: INV_EQUIP, src: 1, dst: -5, count: -1 };
        assert!(!m.is_unequip());
        assert_eq!(m.equipped_slot(), None);
    }

    #[test]
    fn the_result_is_the_layout_the_handler_reads() {
        let b = inventory_move_result(INV_EQUIP, -5, 1);
        assert_eq!(b.len(), INVENTORY_MOVE_RESULT_LEN);
        assert_eq!(b[0], 1, "bExclRequestSent - a 0 here leaves the UI latched");
        assert_eq!(b[1], 0);
        assert_eq!(u32::from_le_bytes([b[2], b[3], b[4], b[5]]), 1, "nCount is i32");
        assert_eq!(b[6], 0);
        assert_eq!(b[7], MODE_MOVE);
        assert_eq!(b[8] as i8, INV_EQUIP);
        assert_eq!(i16::from_le_bytes([b[9], b[10]]), -5);
        assert_eq!(i16::from_le_bytes([b[11], b[12]]), 1);
        assert_eq!(b[13], 0, "the avatarChanged tail");
    }

    /// The latch byte is not decorative: without it every later inventory action is dropped.
    #[test]
    fn every_result_unlocks_the_request_latch() {
        for (t, o, n) in [(INV_EQUIP, -5i16, 1i16), (INV_EQUIP, -11, 3), (6, -1, 2)] {
            assert_eq!(inventory_move_result(t, o, n)[0], 1);
        }
    }
}
