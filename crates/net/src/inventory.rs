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

/// `invType` 6, the Decoration tab.
///
/// It matters only to [`move_changes_the_avatar`]: the client's `avatarChanged` test names
/// **1 or 6**, not 1 alone, so a move on this type with a negative side also earns the
/// trailing byte. Nothing in this server puts an item there yet.
pub const INV_DECO: i8 = 6;

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

/// Length of an [`inventory_rejected`] body: the 7-byte header and nothing else.
pub const INVENTORY_REJECTED_LEN: usize = 7;

/// Refuse a move **without leaving the client's UI latched**.
///
/// `nCount = 0` makes `FUN_142d51930` skip its entire entry loop - the `TEST/JLE` on the
/// `i32` at `142d51b2b` - so no entry is read, no `avatarChanged` is set, and the
/// conditional trailing byte is not read either. Seven bytes, and nothing moves.
///
/// **But the header still runs**, and that is the whole point: `bExclRequestSent = 1` clears
/// `player+0x2330`, so the next request is not refused before it is built.
///
/// # Why this exists
///
/// The first version of this module answered an unsupported move with a chat notice and no
/// `0x0070` at all. On 2026-08-19 the owner unequipped an item successfully, tried to put it back
/// on, got the notice - and then **every further inventory interaction was dead**, including
/// unequipping a different item. The client had latched on the equip request and nothing
/// cleared it.
///
/// The module doc had already spelled out that hazard. Writing the explanation is not the
/// same as obeying it: **every `0x0107` must be answered with a `0x0070`, including - most
/// of all - the ones being refused.**
pub fn inventory_rejected() -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(1); // bExclRequestSent - the one byte this packet exists to deliver
    w.u8(0);
    w.u32(0); // nCount = 0: skip the entry loop entirely
    w.u8(0); // notRemoveAddInfo
    w.into_vec()
}

/// Length of an [`inventory_move_result`] body **for an equip or an unequip**: header 7,
/// one entry 6, one trailing byte.
///
/// A bag-to-bag move is one byte shorter, because it does not change the avatar and the
/// client never reads that byte. See [`move_changes_the_avatar`].
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
/// u8  0                        ONLY when this entry sets avatarChanged
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
/// alone when `(invType == 1 || invType == 6) && (oldPos < 0 || newPos < 0)`. An equip and an
/// unequip are exactly that case; a bag-to-bag move is not, and gets a body one byte shorter.
/// [`move_changes_the_avatar`] is that rule, and it used to be an assertion that the caller
/// was always on the equipped side of it.
///
/// `research/msexe-setfield.md` warns against driving this flag from mode 3, where it depends
/// on client-side inventory state the server cannot see - this builder never does.
pub fn inventory_move_result(inv_type: i8, old_pos: i16, new_pos: i16) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(1); // bExclRequestSent - clears the +0x2330 latch
    w.u8(0);
    w.u32(1); // nCount, i32
    w.u8(0); // notRemoveAddInfo
    w.u8(MODE_MOVE);
    w.u8(inv_type as u8);
    w.i16(old_pos);
    w.i16(new_pos);
    if move_changes_the_avatar(inv_type, old_pos, new_pos) {
        w.u8(0); // avatarChanged tail
    }
    w.into_vec()
}

/// Whether a mode-2 entry earns its trailing byte, **by the client's own rule**.
///
/// `(invType == 1 || invType == 6) && (oldPos < 0 || newPos < 0)` - one side of the move is
/// an equipped slot on an inventory that dresses the avatar. This is computed from the wire
/// values alone, which is the whole reason mode 2 is safe to drive and mode 3 is not: mode
/// 3's flag depends on client-side inventory state the server cannot see.
///
/// **This used to be a `debug_assert!` that the caller was always in that case**, and the
/// byte was appended unconditionally. That held while an unequip was the only move this
/// server would answer. It stopped holding the moment bag-to-bag moves were wired: two
/// positive positions would have panicked a debug build and, in release, appended a byte the
/// client never reads. Surplus bytes are harmless here because the frame carries its own
/// length - but "harmless because of a property of the framing" is not a reason to send a
/// byte that is wrong, and this is a packet where being one byte out has cost two sessions.
pub fn move_changes_the_avatar(inv_type: i8, old_pos: i16, new_pos: i16) -> bool {
    (inv_type == INV_EQUIP || inv_type == INV_DECO) && (old_pos < 0 || new_pos < 0)
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

    /// A bag-to-bag move does NOT change the avatar, so it does not get the trailing byte.
    ///
    /// The old builder appended it unconditionally behind a `debug_assert!` that one side was
    /// negative. That held while an unequip was the only move this server answered; the
    /// moment bag-to-bag moves were wired it would have panicked a debug build and, in
    /// release, sent a byte the client never reads.
    #[test]
    fn a_move_within_one_bag_is_a_byte_shorter() {
        let worn = inventory_move_result(INV_EQUIP, -5, 1);
        let in_bag = inventory_move_result(INV_EQUIP, 1, 2);
        assert_eq!(worn.len(), INVENTORY_MOVE_RESULT_LEN);
        assert_eq!(in_bag.len(), INVENTORY_MOVE_RESULT_LEN - 1);
        // The header, the mode and the invType are identical; only the positions and the
        // presence of the tail differ.
        assert_eq!(&worn[..9], &in_bag[..9], "same header, same mode 2, same invType");
    }

    /// The rule itself, both halves of the `&&` and both inventory types.
    #[test]
    fn only_an_equipped_side_on_an_avatar_inventory_changes_the_avatar() {
        assert!(move_changes_the_avatar(INV_EQUIP, -5, 1), "unequip");
        assert!(move_changes_the_avatar(INV_EQUIP, 1, -5), "equip");
        assert!(move_changes_the_avatar(INV_DECO, -1, 2), "the client's rule names 6 too");
        assert!(!move_changes_the_avatar(INV_EQUIP, 1, 2), "bag to bag");
        assert!(!move_changes_the_avatar(2, -1, 2), "Consume has no equipped side");
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

    /// A refusal is a packet, not a silence. This is the test for the bug that killed the
    /// inventory UI on 2026-08-19: an unsupported move answered with anything other than a
    /// `0x0070` leaves `player+0x2330` set and every later request is dropped before it is
    /// built.
    #[test]
    fn a_refusal_still_unlocks_the_latch_and_moves_nothing() {
        let b = inventory_rejected();
        assert_eq!(b.len(), INVENTORY_REJECTED_LEN, "header only - seven bytes");
        assert_eq!(b[0], 1, "bExclRequestSent: the entire reason this packet is sent");
        assert_eq!(
            u32::from_le_bytes([b[2], b[3], b[4], b[5]]),
            0,
            "nCount 0 skips the entry loop, so nothing moves and no trailing byte is read"
        );
    }
}
