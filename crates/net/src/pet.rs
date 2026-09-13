//! Pets on the field: the double-click that summons one, and the packet that puts it there.
//!
//! The owner, 2026-09-13: *"I tried summoning the Husky pet, but the pet does not come out."*
//! The shop and the bag existed (`bag::pet_item_with_cash_sn`); nothing answered the
//! double-click. `world.log` 18:02:29: `<- 0x0147 UNKNOWN, 6 byte body 509a1814 0100` -
//! twice, once per click, and nothing back.
//!
//! # The request, `0x0147` **[L]**
//!
//! `tools/encodes.py 0x142d4ced0` - the builder `research/msexe-send-opcodes.txt` names for
//! `0x0147` - writes `CTOR, u32, u16, SEND`: the client's tick and the **Cash-tab slot** the
//! pet sits in (`01 00` = slot 1, where the Husky was). The reference server's
//! `PetHandler.handleUserActivatePetRequest` reads the same two fields and one more (`bossMode`)
//! this client does not send. A second click on an active pet puts it away - the reference
//! toggles on `activeState`, and there is no separate "deactivate" request in the send table.
//!
//! # The answer, `0x0277` **[L]**
//!
//! A per-user packet: the user pool reads `u32 charId` and hands `0x0277` to the user's
//! vtable slot `+0x98` (`FUN_142795b20`, the `0x277..0x27E` sub-dispatcher of table A in
//! `research/user-pool-tables.md`). The local user's implementation is `FUN_1428a01a0`, the
//! remote user's `FUN_1429d6150`; both read (`research/msexe-pet-activated.c`):
//!
//! ```text
//! u32  petIdx        only 0 is accepted - one pet
//! u8   activated     0: the pet at that index is put away, nothing more is read
//! u8   init          1 on a fresh summon (the reference's "init")
//! -- CPet::Init, FUN_141eb9760, in `tools/listing.py` read order --
//! u32  itemId
//! str  name
//! raw8 petLockerSN   stored at pet+0x2a; the item's serial
//! u16  x
//! u16  y
//! u8   moveAction
//! u16  foothold      looked up in the field's foothold tree (FUN_142df6c50)
//! u32  hue
//! u32  itemId again
//! u16  wonderGrade
//! u16  giantRate
//! u8   nameTag
//! u8   chatBalloon
//! ```
//!
//! The widths and order are the client's **[L]**; the names of the last six are the reference's
//! `Pet.encode` and older builds' `nameTag`/`chatBalloon` tail **[I]**. `0x0278..0x027E` are the
//! rest of the family (move, action, speak, name change, hue, command, exception list) and are
//! not built - a solo pet needs none of them to appear.
//!
//! # What is deliberately NOT here
//!
//! * **No pet stat bit.** This client's stat decoder has no test for bit 3, the pet-serial bit
//!   of other builds (`stats::bits::NOT_DECODED_BIT_3`), so nothing rides in `0x007C`.
//! * **No persistence.** The active pet lives on the session; a relog puts it away, exactly as
//!   the reference's `initPets` re-summons only what the character row says is active - and this
//!   store has no such column yet.

use crate::packet::{PacketReader, PacketWriter};

/// Client -> server: a double-click on a pet in the Cash tab. `u32 tick, u16 slot`.
pub const CLIENT_PET_ACTIVATE: u16 = 0x0147;

/// Server -> client, per user: a pet appears beside (or vanishes from) a character.
pub const PET_ACTIVATED: u16 = 0x0277;

/// The one pet index this client accepts (`FUN_1429d6150`: `if (petIdx == 0)`).
pub const PET_INDEX: u32 = 0;

/// The parsed `0x0147`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PetActivate {
    pub tick: u32,
    /// The Cash-tab slot, 1-based, exactly as the bag numbers it.
    pub slot: u16,
}

/// Decode a `0x0147` body (after the opcode). `None` if it is not six bytes.
pub fn parse_pet_activate(body: &[u8]) -> Option<PetActivate> {
    let mut r = PacketReader::new(body);
    let tick = r.u32().ok()?;
    let slot = r.u16().ok()?;
    Some(PetActivate { tick, slot })
}

/// A pet as `0x0277` describes it - the fields `CPet::Init` reads, in its order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldPet {
    pub item_id: u32,
    pub name: String,
    /// The pet's serial, which the item in the Cash tab must also carry so the client can
    /// pair the two. Never zero.
    pub serial: u64,
    pub x: i16,
    pub y: i16,
    pub move_action: u8,
    pub foothold: u16,
}

/// The `0x0277` body that summons `pet` beside character `character_id`.
pub fn pet_activated(character_id: u32, pet: &FieldPet) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character_id); //         the user pool's own read - which user
    w.u32(PET_INDEX); //            1429d616d  petIdx
    w.u8(1); //                     1429d617b  activated
    w.u8(1); //                     1429d6187  init
    w.u32(pet.item_id); //          141eb99ad
    w.str(&pet.name); //            141eb99bf
    w.u64(pet.serial); //           141eb9a63  raw8
    w.i16(pet.x); //                141eba3ac
    w.i16(pet.y); //                141eba594
    w.u8(pet.move_action); //       141eba77b
    w.u16(pet.foothold); //         141eba959
    w.u32(0); //                    141eba974  hue
    w.u32(pet.item_id); //          141ebab65
    w.u16(0); //                    141ebab71  wonderGrade
    w.u16(0); //                    141ebacbb  giantRate
    w.u8(0); //                     141ebae0a  nameTag
    w.u8(0); //                     141ebaff0  chatBalloon
    w.into_vec()
}

/// The `0x0277` body that puts character `character_id`'s pet away.
pub fn pet_deactivated(character_id: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character_id);
    w.u32(PET_INDEX);
    w.u8(0); // activated = 0: FUN_1427707e0(user, 0, null) and nothing more is read
    w.into_vec()
}

/// A stable, non-zero serial for a character's pet item, since the store keeps no cash
/// serials: the character id in the high half, the item id in the low. The same value goes
/// into the pet body and into the Cash-tab item, which is all the client needs of it.
pub fn pet_serial(character_id: u32, item_id: u32) -> std::num::NonZeroU64 {
    let raw = (u64::from(character_id) << 32) | u64::from(item_id);
    std::num::NonZeroU64::new(raw).unwrap_or(std::num::NonZeroU64::MIN)
}

/// Bytes of a `0x0277` activation for a pet whose name is `name_len` bytes long.
pub const fn pet_activated_len(name_len: usize) -> usize {
    4 + 4 + 1 + 1 + 4 + (2 + name_len) + 8 + 2 + 2 + 1 + 2 + 4 + 4 + 2 + 2 + 1 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The two captured bodies from 2026-09-13 18:02: tick, then slot 1.
    #[test]
    fn the_double_click_is_a_tick_and_the_cash_tab_slot() {
        let a = parse_pet_activate(&[0x50, 0x9a, 0x18, 0x14, 0x01, 0x00]).unwrap();
        assert_eq!(a.slot, 1);
        assert_eq!(a.tick, 0x1418_9a50);
        let b = parse_pet_activate(&[0xf2, 0x9d, 0x18, 0x14, 0x01, 0x00]).unwrap();
        assert_eq!(b.slot, 1);
        assert!(parse_pet_activate(&[0, 0, 0, 0, 1]).is_none(), "five bytes is not the shape");
    }

    /// Every field of `CPet::Init`, at the offset the read order puts it.
    #[test]
    fn the_activation_body_follows_the_clients_read_order() {
        let pet = FieldPet {
            item_id: 5_000_006,
            name: "Husky".to_string(),
            serial: pet_serial(215, 5_000_006).get(),
            x: -120,
            y: 85,
            move_action: 4,
            foothold: 37,
        };
        let b = pet_activated(215, &pet);
        assert_eq!(b.len(), pet_activated_len(5));
        assert_eq!(&b[0..4], &215u32.to_le_bytes(), "the user pool's charId");
        assert_eq!(&b[4..8], &0u32.to_le_bytes(), "petIdx 0 - the only one accepted");
        assert_eq!(b[8], 1, "activated");
        assert_eq!(b[9], 1, "init");
        assert_eq!(&b[10..14], &5_000_006u32.to_le_bytes());
        assert_eq!(&b[14..16], &5u16.to_le_bytes(), "name length");
        assert_eq!(&b[16..21], b"Husky");
        assert_eq!(&b[21..29], &pet.serial.to_le_bytes());
        assert_eq!(&b[29..31], &(-120i16).to_le_bytes());
        assert_eq!(&b[31..33], &85i16.to_le_bytes());
        assert_eq!(b[33], 4, "moveAction");
        assert_eq!(&b[34..36], &37u16.to_le_bytes(), "foothold");
        assert_eq!(&b[36..40], &0u32.to_le_bytes(), "hue");
        assert_eq!(&b[40..44], &5_000_006u32.to_le_bytes(), "itemId again");
        assert_eq!(&b[44..50], &[0, 0, 0, 0, 0, 0], "wonderGrade, giantRate, nameTag, chatBalloon");
    }

    #[test]
    fn putting_a_pet_away_reads_nothing_past_the_activated_byte() {
        let b = pet_deactivated(215);
        assert_eq!(b, [0xd7, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn a_pet_serial_is_never_zero_and_pairs_character_and_item() {
        assert_eq!(pet_serial(215, 5_000_006).get(), (215u64 << 32) | 5_000_006);
        assert_eq!(pet_serial(0, 0).get(), 1, "the one degenerate input still yields a serial");
    }
}
