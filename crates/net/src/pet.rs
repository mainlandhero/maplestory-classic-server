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
//! u32  hue           -1 when undyed; 0 reads as "dyed with colour 0" - `bag::PET_HUE_UNDYED`
//! u32  itemId again
//! u16  wonderGrade
//! u16  giantRate      the pet's size in PERCENT - stored at pet+0x230, applied by FUN_141ec87b0
//!                     as a layer scale whenever it is not 100. 0 draws nothing. PET_SIZE_PERCENT
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

/// **Client -> server: the pet walked.** The owner, 2026-09-13: *"broadcast player pet movement so
/// other people can see pets moving even if it is not their own."*
///
/// Measured: `0x0202` arrives **504 times after a summon and 0 times before it**, and the first
/// point in its body is the exact spot the server placed the pet. Its head is **`u32 petIdx,
/// u8`** - five bytes - and then the movement path block, which begins with its own `u32`
/// (`usermove::MOVE_PATH_HEAD_LEN`: `u32, i16 x, i16 y, u16, u16, i16 count`), exactly as the
/// character's `0x00D9` path does. **[L]**: the builder `FUN_142b68a20` writes `w_u32, w_u8`
/// and then calls the path encoder `FUN_141d57c60` (`tools/encodes.py 0x142b68a20`).
///
/// Until 2026-09-15 this said `u32 petIdx, u32 tick, u8` - nine bytes - labelled **[D]**. That
/// was the path's leading `u32` misread as a tick (it is `0` in all 7 278 captured bodies, as
/// `0x00D9`'s is), and the four bytes it stole from the path are what crashed every observer.
/// See [`PET_MOVE`].
pub const CLIENT_PET_MOVE: u16 = 0x0202;

/// Server -> client, per user: **that character's pet moved.**
///
/// The dispatcher `FUN_142795b20` reads `charId` (the user pool) and then **consumes `petIdx`
/// itself** (`call 0x1406e8c20` at `0x142795b5e`, which advances the reader) before it tail-jumps
/// to the move handler `FUN_141ec3f20`. So the movement-path applier `FUN_141d598b0` /
/// `FUN_1404b2630` reads starting **after `petIdx`**, and its first read is a `u32` that lands in
/// `path+0x40` - the sender's move key, ignored by an observer - *before* the `i16 x, i16 y`.
/// This is the same shape `0x0293` (remote user) and `0x03D9` (mob) carry. **[L]**
///
/// # The path must be forwarded WHOLE, leading `u32` included - a 4-byte cut crashed every observer
///
/// 2026-09-15: summoning a pet on `the owner` crashed `Tester2` in the same map. The dump
/// (`research/pet-remote-crash-2026-09-15.md`) faulted at `0x141d59bf3`, `movups xmm0,[rax]`
/// with `rax = 0` - the applier's element list tail was null because it had appended **nothing**:
/// `FUN_1404b2630` bails at `0x1404b26b3 jle` when the element count `i16` is `<= 0`, and the
/// count it read was `-97`, the pet's own X coordinate. The path block the applier reads is
/// `u32, i16 x, i16 y, u16, u16, i16 count, elements` - `u32`-led, like `0x00D9`'s and the mob's.
/// [`CLIENT_PET_MOVE_HEAD_LEN`] was 9 when the head is **5**, so "the path" the server forwarded
/// began at `x`: its own leading `u32` had been swallowed into a phantom "tick", every field was
/// read four bytes early, and the count landed on a coordinate. With the head at its true length
/// the path goes out verbatim, `u32` and all: `u32 charId, u32 petIdx, <the 0x0202 path from
/// byte 5>` - [`pet_move_broadcast`]. **[L]** on the dispatch (`0x142795b5e` consumes `petIdx`),
/// the builder (`tools/encodes.py 0x142b68a20`) and the count offset, confirmed against the crash
/// dump's registers.
pub const PET_MOVE: u16 = 0x0278;

/// Server -> client, per user: **the pet does something and says a line.** `FUN_141ec3fa0`
/// reads `u8, u8, str` and calls `FUN_141ec6680(pet, command1, command2, message, 0)`, whose
/// first act is `test r8d, r8d` on `command2` - so that byte is a flag. **[L]**
///
/// `command1` is the `interact` entry's index and `command2` is success/fail: the client owns
/// the animation, looking `interact/<index>/<success|fail>/0/act` up in the pet's own image, so
/// the server sends the index and the outcome rather than an animation name. **[I]**, and the
/// screen is the test - a wrong index plays the wrong trick, not a crash, because the client
/// indexes its own node list.
pub const PET_ACTION: u16 = 0x0279;

/// **Client -> server: the pet reached a drop and wants it.** The owner, 2026-09-15: *"Husky also
/// currently does not loot items on the ground."* Seven of these in that run, all unanswered:
///
/// ```text
/// 00000000 00 01f14d20 01000000 69ff d700 002d3101 9667e331 464b4c00     29 bytes
/// petIdx   u8 tick     u32      x    y    dropId   crc      itemId
/// ```
///
/// `FUN_141ec1f20` writes `u32, u8, u32, u32, u16, u16, u32, u32, u32, ...` (`tools/encodes.py`),
/// and the drop id at byte 17 is `0x01312d00` = 20 000 000, this server's first drop object id -
/// the offset `crate::drops::PET_PICK_UP_OBJECT_ID_AT` had from the reference and waited for a
/// capture to confirm. **[L]** Routed to the same pick-up handler as the player's request;
/// `Session::on_pick_up` already takes it by the pet offset.
pub const CLIENT_PET_PICK_UP: u16 = 0x0205;

/// **Client -> server: the pet did a trick on its own client.** `FUN_141ebe480`, `u32 petIdx,
/// u8, u16 interact index` - sent once, right before the chat line that triggered it (`world-
/// ch0.log` 02:56:28.893: `0004` then "bad", interact 4). A report: the server already answers
/// the chat line with the `0x0279` of its own choosing, and the client went on. Not answered.
pub const CLIENT_PET_ACTION_REPORT: u16 = 0x0204;

/// Server -> client, per user: **the pet's name changed.** `FUN_141ec4660`, the `0x27b` arm of
/// the pet sub-dispatcher, reads one `str` after the `charId`/`petIdx` the dispatcher consumed
/// (`tools/reads.py 0x141ec4660 1`). **[L]** Sent to the owner and the map after a Pet Name Tag.
pub const PET_NAME_CHANGED: u16 = 0x027B;

/// The bytes of `0x0202` before the movement path: `u32 petIdx, u8`. **Five**, from the
/// builder's own writes (`FUN_142b68a20`: `w_u32, w_u8`, then the path encoder). It was 9 until
/// 2026-09-15, which cut the path's leading `u32` off and crashed every observer of a pet move.
pub const CLIENT_PET_MOVE_HEAD_LEN: usize = 5;

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

/// **The pet's size, as a percentage. 100 is life-size; 0 is invisible.**
///
/// The `u16` at `0x141ebacbb` - the reference calls it `giantRate` - is stored at `pet+0x230`,
/// and `CPet`'s animation setter `FUN_141ec87b0` reads it back and, **whenever it is not
/// 100**, sets flag `2` on the pet's layer and `layer->vtbl[0x320](8, value)`: a scale. This
/// server sent `0` from the day the packet was built, so every summoned pet was drawn at
/// zero percent - visible, opaque, positioned, framed, and nothing on screen. **[L]**, 2026-09-14,
/// after eleven runs that measured everything else about the pet as correct.
///
/// It is the mob-size bug again (`CLAUDE.md`, "The unit, not the arithmetic"): a `0` meant as
/// "unset" that the client reads as **zero percent**. The name tag was never affected because
/// it has its own layer, and the Character Info window draws the same pet through a presenter
/// that never applies this field - which is how the two of them together said "the assets are
/// fine and the field is not" before the field itself was read.
pub const PET_SIZE_PERCENT: u16 = 100;

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
    w.u32(crate::bag::PET_HUE_UNDYED); // 141eba974  hue: -1 is undyed - see PET_HUE_UNDYED
    w.u32(pet.item_id); //          141ebab65
    w.u16(0); //                    141ebab71  wonderGrade
    w.u16(PET_SIZE_PERCENT); //     141ebacbb  giantRate - the SIZE, in percent. 0 is invisible
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

/// **`0x0278`**: rebroadcast a pet's movement to everyone on the map.
///
/// `body` is the `0x0202` the client sent. The path block is copied byte for byte - the same
/// rule `0x0293` follows for a remote character, and for the same reason: the client that owns
/// the pet has already decided where it walked, and re-encoding the path could only lose
/// something. `None` when the body is too short to hold a head and a path.
pub fn pet_move_broadcast(character_id: u32, body: &[u8]) -> Option<Vec<u8>> {
    let pet_index = u32::from_le_bytes(body.get(0..4)?.try_into().ok()?);
    // The path, verbatim, from the byte after the five-byte head - so its own leading `u32`
    // (the field the observer's applier reads into `path+0x40` and ignores) is kept. Cutting
    // that `u32` off is what crashed every observer on 2026-09-15: every field read four bytes
    // early and the count came out as the pet's X coordinate. See [`PET_MOVE`].
    let path = body.get(CLIENT_PET_MOVE_HEAD_LEN..)?;
    // Below the path head (u32, x, y, u16, u16, count) there is nothing to forward, and the
    // count must be positive or the applier appends nothing and then dereferences the empty
    // list tail - the exact null-deref of the crash, guarded here so a zero-element path
    // (never yet observed; `research/remote-move-verification.md` §6.1) can never reach an
    // observer.
    let count = path.get(12..14)?;
    if i16::from_le_bytes([count[0], count[1]]) <= 0 {
        return None;
    }
    let mut w = PacketWriter::new();
    w.u32(character_id);
    w.u32(pet_index);
    w.bytes(path);
    Some(w.into_vec())
}

/// **`0x0279`**: the pet plays `interact` entry `index` and says `message`.
pub fn pet_action(character_id: u32, index: u8, success: bool, message: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character_id);
    w.u32(PET_INDEX);
    w.u8(index); //           141ec3fa0's first u8  -> FUN_141ec6680's command1
    w.u8(u8::from(success)); // its second          -> command2, tested as a flag
    w.str(message);
    w.into_vec()
}

/// The `0x027B` body: `u32 charId, u32 petIdx, str name`.
pub fn pet_name_changed(character_id: u32, name: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character_id);
    w.u32(PET_INDEX);
    w.str(name);
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
        assert_eq!(&b[36..40], &crate::bag::PET_HUE_UNDYED.to_le_bytes(), "hue: -1, or the tooltip says the pet was dyed");
        assert_eq!(&b[40..44], &5_000_006u32.to_le_bytes(), "itemId again");
        assert_eq!(&b[44..46], &0u16.to_le_bytes(), "wonderGrade");
        assert_eq!(
            &b[46..48],
            &PET_SIZE_PERCENT.to_le_bytes(),
            "giantRate is the pet SIZE in percent and it must be 100: 0 drew every pet at zero percent - visible, opaque, positioned, framed, and nothing on screen"
        );
        assert_eq!(PET_SIZE_PERCENT, 100, "100 is the one value FUN_141ec87b0 treats as unscaled");
        assert_eq!(&b[48..50], &[0, 0], "nameTag, chatBalloon");
    }

    #[test]
    fn putting_a_pet_away_reads_nothing_past_the_activated_byte() {
        let b = pet_deactivated(215);
        assert_eq!(b, [0xd7, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    /// The captured `0x0202`, 41 bytes: the head is nine and the path starts at the pet's own
    /// position (0x0136, 0x0112 = 310, 274 - where the server put it).
    #[test]
    fn a_pet_move_forwards_the_path_from_its_five_byte_head_so_the_client_reads_the_count() {
        // The owner's live capture, world-ch0.log 02:05:28.489, the 1-element move whose forwarding
        // crashed Tester2: petIdx 0, u8 0, then the u32-led path: key 0, x=-97, y=148, ...,
        // count=1. The head is FIVE bytes (the builder writes w_u32, w_u8, then the path
        // encoder); it was taken as nine, which cut the path's leading u32 off.
        let hex = "0000000000000000009fff9400000000000100009fff950000000000a60000000000000004fe010000";
        let body: Vec<u8> =
            (0..hex.len() / 2).map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap()).collect();
        assert_eq!(body.len(), 41);
        assert_eq!(CLIENT_PET_MOVE_HEAD_LEN, 5);
        assert_eq!(&body[5..9], &0u32.to_le_bytes(), "the path's own leading u32, zero as in every 0x00D9");
        assert_eq!(&body[9..11], &(-97i16).to_le_bytes(), "then the pet's x");

        let out = pet_move_broadcast(215, &body).unwrap();
        assert_eq!(&out[0..4], &215u32.to_le_bytes(), "charId");
        assert_eq!(&out[4..8], &0u32.to_le_bytes(), "petIdx");
        assert_eq!(&out[8..], &body[5..], "the path from byte 5, verbatim, leading u32 included");
        assert_eq!(out.len(), 8 + (41 - 5));

        // Decode the way the client does: the dispatcher has consumed charId+petIdx, so the
        // applier reads from byte 8 - u32 key, i16 x, i16 y, u16, u16, i16 count. The count
        // must come out positive, which is the whole point: with the u32 cut off it read -97.
        let at = 8;
        let x = i16::from_le_bytes(out[at + 4..at + 6].try_into().unwrap());
        let y = i16::from_le_bytes(out[at + 6..at + 8].try_into().unwrap());
        let count = i16::from_le_bytes(out[at + 12..at + 14].try_into().unwrap());
        assert_eq!((x, y, count), (-97, 148, 1), "the applier now reads the real x, y and count");

        assert!(pet_move_broadcast(215, &body[..5]).is_none(), "a head with no path is nothing to send");
        assert!(pet_move_broadcast(215, &body[..18]).is_none(), "a path cut before its count is nothing to send");
        assert!(pet_move_broadcast(215, &body[..3]).is_none());

        // A zero-element path is dropped rather than forwarded - it would null-deref the
        // observer's empty list tail (research/remote-move-verification.md §6.1).
        let mut zero = body.clone();
        zero[17..19].copy_from_slice(&0i16.to_le_bytes()); // the count: path offset 12 = body 17
        assert!(pet_move_broadcast(215, &zero).is_none(), "a zero-element pet path is not forwarded");
    }

    #[test]
    fn a_pet_action_carries_the_interact_index_the_outcome_and_the_line() {
        let b = pet_action(215, 3, true, "Bark bark!");
        assert_eq!(&b[0..4], &215u32.to_le_bytes());
        assert_eq!(&b[4..8], &PET_INDEX.to_le_bytes());
        assert_eq!(b[8], 3, "the interact entry");
        assert_eq!(b[9], 1, "success");
        assert_eq!(&b[10..12], &10u16.to_le_bytes(), "the line's length");
        assert_eq!(&b[12..22], b"Bark bark!");
        assert_eq!(pet_action(215, 3, false, "x")[9], 0, "fail");
    }

    #[test]
    fn a_pet_serial_is_never_zero_and_pairs_character_and_item() {
        assert_eq!(pet_serial(215, 5_000_006).get(), (215u64 << 32) | 5_000_006);
        assert_eq!(pet_serial(0, 0).get(), 1, "the one degenerate input still yields a serial");
    }
}
