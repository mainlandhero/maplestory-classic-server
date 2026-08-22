//! Mr. Kim's storage box: opening it, and the five things the player can ask it to do.
//!
//! The owner, 2026-08-22: *"Mr. Kim the storage keeper does not open the storage UI."* They did not,
//! because **nothing had ever sent a packet** - `crates/store/src/storage.rs` has had the
//! whole database layer for days and no opcode existed. `research/storage.md` has the working;
//! this is the wire.
//!
//! # The two opcodes
//!
//! * **[`STORAGE_RESULT`] `0x0572`**, server to client, first byte a mode. Mode
//!   [`RESULT_OPEN`] carries the NPC template and the whole box.
//! * **[`CLIENT_STORAGE`] `0x00F6`**, client to server, first byte a mode. See
//!   [`StorageRequest`].
//!
//! The chain to them was found the same way the revive dialog was: `UI/Storage.img/Trunk` has
//! one code reference, that function is **vtable slot 4** - the same slot the revive dialog
//! uses - its constructor has one call site, and above that exactly one opcode lands on the
//! handler in `CField::OnPacket`'s second jump table. **[L]**
//!
//! # Two traps that would each have cost a launch
//!
//! * **The "index" in a take-out request is positional, not a slot.** It is the 0-based
//!   position within its type's list **as the server sent it**, while the put-in request in
//!   the *same field offset* carries the player's real 1-based bag slot. Same place, two
//!   meanings. [`StorageRequest::TakeOut`] names it `position` for that reason.
//! * **Every request latches `dlg+0x334`, and only an inbound `0x0572` clears it.** An
//!   unanswered `0x00F6` leaves the window open with every button dead - the always-answer
//!   rule with a new field name.
//!
//! # The item blobs are not new code
//!
//! Storage reuses the bag's item encoding exactly - the client decodes both through
//! `FUN_140303530` - so [`crate::opcode::equipped_item`] and [`crate::bag::bundle_item`]
//! already produce legal blobs, leading type byte included. Writing a second encoder here
//! would be building the thing `bag.rs` exists to prevent.
//!
//! The presence bytes are the same namespace too, and that is the strongest single control in
//! `research/storage.md`: the seven storage gate keys come out as **mesos at 1 and the six
//! bags at `[2, 3, 4, 5, 6, 44]`**, byte-for-byte [`crate::bag::BAG_PRESENCE_BYTE`], derived
//! from a *different* key table built by *different* CRT initialisers. The `44` is what makes
//! that match mean something.

use crate::bag::BAG_PRESENCE_BYTE;
use crate::packet::PacketWriter;

/// `0x0572` - server to client. First body byte is a mode.
pub const STORAGE_RESULT: u16 = 0x0572;

/// `0x00F6` - client to server. First body byte is a mode.
pub const CLIENT_STORAGE: u16 = 0x00F6;

/// Mode 24: open the window, carrying the NPC template and the whole box.
pub const RESULT_OPEN: u8 = 24;
/// Mode 13: a put succeeded - the client rebuilds both grids and the trunk block.
pub const RESULT_PUT_OK: u8 = 13;
/// Mode 15: the trunk block only, for a take-out or a meso move.
pub const RESULT_TRUNK_REFRESH: u8 = 15;
/// Mode 9: the player's own inventory changed; no trunk block follows.
pub const RESULT_INVENTORY_REFRESH: u8 = 9;
/// Mode 10: "your inventory is full".
pub const RESULT_INVENTORY_FULL: u8 = 10;
/// Mode 11: "you do not have enough mesos".
pub const RESULT_NOT_ENOUGH_MESOS: u8 = 11;
/// Mode 16: not enough to pay the deposit fee.
pub const RESULT_NOT_ENOUGH_FEE: u8 = 16;
/// Mode 17: the storage box is full.
pub const RESULT_STORAGE_FULL: u8 = 17;
/// Mode 21: over the meso limit.
pub const RESULT_MESO_LIMIT: u8 = 21;

/// The presence byte that gates the meso field. The six bags use
/// [`crate::bag::BAG_PRESENCE_BYTE`].
pub const PRESENCE_MESOS: usize = 1;

/// The gate mask is exactly 100 bytes, the same shape the character record uses.
pub const PRESENCE_LEN: usize = 100;

/// The six inventory types, in the order the trunk block lists them.
pub const INVENTORY_TYPES: usize = 6;

/// What the player asked the storage box to do. `0x00F6`, first byte the mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageRequest {
    /// Mode 4. **`position` is not a slot** - see the module docs.
    TakeOut { inv_type: u8, position: u8, count: u16 },
    /// Mode 5. `bag_slot` **is** a real 1-based bag slot, unlike `TakeOut`'s index.
    PutIn { bag_slot: u16, item_id: u32, count: u16 },
    /// Mode 6.
    Sort,
    /// Mode 7. **One signed `i64`**: positive withdraws from the box, negative deposits into
    /// it. A single field for both directions, which is why it must not be read as a `u64`.
    Mesos { amount: i64 },
    /// Mode 8.
    Close,
}

impl StorageRequest {
    /// Parse a `0x00F6` body, opcode already stripped.
    ///
    /// `None` for a mode we do not know or a body too short for the mode it claims. The
    /// caller must **still answer** - see the module docs on the latch.
    pub fn parse(body: &[u8]) -> Option<Self> {
        let mode = *body.first()?;
        let rest = &body[1..];
        let u16_at = |o: usize| -> Option<u16> {
            Some(u16::from_le_bytes(rest.get(o..o + 2)?.try_into().ok()?))
        };
        let u32_at = |o: usize| -> Option<u32> {
            Some(u32::from_le_bytes(rest.get(o..o + 4)?.try_into().ok()?))
        };
        match mode {
            4 => Some(StorageRequest::TakeOut {
                inv_type: *rest.first()?,
                position: *rest.get(1)?,
                count: u16_at(2)?,
            }),
            5 => Some(StorageRequest::PutIn {
                bag_slot: u16_at(0)?,
                item_id: u32_at(2)?,
                count: u16_at(6)?,
            }),
            6 => Some(StorageRequest::Sort),
            7 => Some(StorageRequest::Mesos {
                amount: i64::from_le_bytes(rest.get(0..8)?.try_into().ok()?),
            }),
            8 => Some(StorageRequest::Close),
            _ => None,
        }
    }
}

/// The box itself, as the wire wants it: a slot count, a gate mask, a meso balance and six
/// per-type item lists.
///
/// The blobs are built by the caller with [`crate::opcode::equipped_item`] or
/// [`crate::bag::bundle_item`], because deriving an equip's stats from its WZ template is the
/// world crate's job and this crate has no `Config`.
///
/// **All seven gates are always set.** A gate that is off is not "an empty list", it is "this
/// field is absent" - and an absent meso gate means a box that cannot show a balance at all.
/// An empty box with every gate on is 115 bytes and that is the right shape to send.
pub fn trunk_block(slots: u8, mesos: u64, per_type: &[Vec<Vec<u8>>; INVENTORY_TYPES]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(slots);

    let mut presence = [0u8; PRESENCE_LEN];
    presence[PRESENCE_MESOS] = 1;
    for byte in BAG_PRESENCE_BYTE {
        presence[byte] = 1;
    }
    w.bytes(&presence);

    // Eight bytes. The store keeps mesos as a u32 and this field is a u64; widening at the
    // wire edge is right, and reading it back as four would truncate a balance the client
    // can legitimately hold.
    w.u64(mesos);

    for list in per_type {
        w.u8(list.len().min(u8::MAX as usize) as u8);
        for blob in list.iter().take(u8::MAX as usize) {
            w.bytes(blob);
        }
    }
    w.into_vec()
}

/// Mode 24 - open the window at `npc_template`, carrying the whole box.
///
/// **`npc_template` is the NPC's template id, not its object id.** The client loads
/// `Npc.wz` from it to find the deposit fee, so an object id here charges the wrong fee or
/// none.
pub fn open_storage(
    npc_template: u32,
    slots: u8,
    mesos: u64,
    per_type: &[Vec<Vec<u8>>; INVENTORY_TYPES],
) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(RESULT_OPEN);
    w.u32(npc_template);
    w.bytes(&trunk_block(slots, mesos, per_type));
    w.into_vec()
}

/// A mode that carries a fresh trunk block and nothing else - [`RESULT_TRUNK_REFRESH`] after
/// a take-out or a meso move, [`RESULT_PUT_OK`] after a put.
pub fn storage_refresh(
    mode: u8,
    slots: u8,
    mesos: u64,
    per_type: &[Vec<Vec<u8>>; INVENTORY_TYPES],
) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(mode);
    w.bytes(&trunk_block(slots, mesos, per_type));
    w.into_vec()
}

/// A refusal: one mode byte and nothing else. Clears the latch, changes nothing.
///
/// The latch is the whole reason this exists. A refused request that sends nothing leaves the
/// window open with every button dead until the player closes it by hand.
pub fn storage_refused(mode: u8) -> Vec<u8> {
    vec![mode]
}

/// The deposit fee each storage keeper charges, by NPC template. **[L]**, from the client's
/// own `Npc.wz` `info/trunkPut`.
///
/// Ten NPCs, and **none of them charges to withdraw** - `trunkGet` is absent on all ten, so
/// taking an item out is free. Re-derive with:
/// `target/release/wz-dump cat "client-patched/Data/Npc/Npc_000.wz" 0000105.img`
///
/// Hardcoded rather than dumped because it is ten rows and a generated file for ten rows is
/// more moving parts than the thing it replaces. The command above is the check.
pub const STORAGE_KEEPERS: [(u32, u32); 10] = [
    (105, 100),      // Mr. Kim
    (220, 100),      // Mr. Lee
    (307, 100),      // Mr. Park
    (405, 100),      // Mr. Hong
    (505, 100),      // Mr. Wang
    (604, 100),      // Mr. Oh
    (704, 100),      // Cave Fairy's Storage
    (1011, 100),     // Trina
    (1110, 150),     // Mr. Thalj - the one that is not 100
    (800_009, 100),  // Scrooge
];

/// The deposit fee for `template`, or `None` if it is not a storage keeper.
pub fn storage_fee(template: u32) -> Option<u32> {
    STORAGE_KEEPERS.iter().find(|(t, _)| *t == template).map(|(_, fee)| *fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An empty box with every gate on is **115 bytes**, and a mode-24 body is 120.
    ///
    /// Asserted from the field widths rather than against the literals alone, so a change to
    /// the mask length cannot leave a stale total passing beside it.
    #[test]
    fn an_empty_box_is_the_documented_length() {
        let empty: [Vec<Vec<u8>>; INVENTORY_TYPES] = Default::default();
        let block = trunk_block(4, 0, &empty);
        assert_eq!(block.len(), 1 + PRESENCE_LEN + 8 + INVENTORY_TYPES);
        assert_eq!(block.len(), 115);

        let open = open_storage(105, 4, 0, &empty);
        assert_eq!(open.len(), 1 + 4 + 115);
        assert_eq!(open.len(), 120);
        assert_eq!(open[0], RESULT_OPEN);
        assert_eq!(u32::from_le_bytes(open[1..5].try_into().unwrap()), 105, "the TEMPLATE id");
    }

    /// The gate mask is the bag's, and the `44` is the part that makes it a real cross-check.
    ///
    /// Two independent derivations landing on `[2,3,4,5,6,44]` is worth something precisely
    /// because the last entry does not follow the others; a run of 2..7 could have been a
    /// coincidence.
    #[test]
    fn the_gates_are_the_bags_own_presence_bytes() {
        let empty: [Vec<Vec<u8>>; INVENTORY_TYPES] = Default::default();
        let block = trunk_block(4, 0, &empty);
        let mask = &block[1..1 + PRESENCE_LEN];
        assert_eq!(mask[PRESENCE_MESOS], 1, "the meso gate is always on");
        for byte in BAG_PRESENCE_BYTE {
            assert_eq!(mask[byte], 1, "bag gate {byte}");
        }
        assert_eq!(BAG_PRESENCE_BYTE, [2, 3, 4, 5, 6, 44]);
        assert_eq!(mask.iter().filter(|&&b| b == 1).count(), 7, "seven gates, no more");
    }

    /// Mesos are **eight** bytes, and a balance above `u32::MAX` survives.
    #[test]
    fn the_meso_field_is_sixty_four_bits() {
        let empty: [Vec<Vec<u8>>; INVENTORY_TYPES] = Default::default();
        let big = 5_000_000_000u64;
        let block = trunk_block(4, big, &empty);
        let at = 1 + PRESENCE_LEN;
        assert_eq!(u64::from_le_bytes(block[at..at + 8].try_into().unwrap()), big);
    }

    /// The meso request is **signed**, and the sign is the direction.
    ///
    /// Reading it as a `u64` would turn every deposit into a withdrawal of about eighteen
    /// quintillion, which is the kind of thing that is obvious once and invisible forever
    /// after.
    #[test]
    fn a_meso_request_carries_its_direction_in_the_sign() {
        let mut body = vec![7u8];
        body.extend_from_slice(&(-250i64).to_le_bytes());
        assert_eq!(StorageRequest::parse(&body), Some(StorageRequest::Mesos { amount: -250 }));

        let mut body = vec![7u8];
        body.extend_from_slice(&1_000i64.to_le_bytes());
        assert_eq!(StorageRequest::parse(&body), Some(StorageRequest::Mesos { amount: 1_000 }));
    }

    /// Take-out and put-in put different things at the same offset, which is the trap.
    #[test]
    fn take_out_carries_a_position_and_put_in_carries_a_slot() {
        let take = StorageRequest::parse(&[4, 2, 3, 1, 0]).unwrap();
        assert_eq!(take, StorageRequest::TakeOut { inv_type: 2, position: 3, count: 1 });

        let mut body = vec![5u8];
        body.extend_from_slice(&7u16.to_le_bytes()); // bag slot 7, ONE-based
        body.extend_from_slice(&2_000_000u32.to_le_bytes());
        body.extend_from_slice(&5u16.to_le_bytes());
        assert_eq!(
            StorageRequest::parse(&body),
            Some(StorageRequest::PutIn { bag_slot: 7, item_id: 2_000_000, count: 5 })
        );
    }

    /// A short or unknown body parses to `None` rather than panicking - it arrives off a
    /// socket - and the caller still has to answer it.
    #[test]
    fn a_truncated_or_unknown_request_is_none_and_never_panics() {
        assert_eq!(StorageRequest::parse(&[]), None);
        assert_eq!(StorageRequest::parse(&[99]), None, "unknown mode");
        assert_eq!(StorageRequest::parse(&[4]), None, "mode 4 with no fields");
        assert_eq!(StorageRequest::parse(&[7, 1, 2, 3]), None, "meso amount cut short");
        for mode in 0u8..=32 {
            for len in 0..12 {
                let body: Vec<u8> =
                    std::iter::once(mode).chain(std::iter::repeat_n(0, len)).collect();
                let _ = StorageRequest::parse(&body);
            }
        }
    }

    /// Mr. Kim charges 100 to deposit and nothing to withdraw, and Mr. Thalj is the one
    /// keeper who is different - so a test that only checked Mr. Kim would pass on a table
    /// that had flattened every fee to 100.
    #[test]
    fn the_keeper_fees_are_the_wz_values_including_the_odd_one() {
        assert_eq!(storage_fee(105), Some(100), "Mr. Kim");
        assert_eq!(storage_fee(1110), Some(150), "Mr. Thalj, the only one that is not 100");
        assert_eq!(storage_fee(2), None, "Sera is not a storage keeper");
        assert_eq!(STORAGE_KEEPERS.len(), 10);
    }
}
