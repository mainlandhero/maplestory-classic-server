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
//! u8   activated     0: the pet at that index is put away. The REMOTE path reads nothing
//!                    more; the LOCAL path (FUN_1428a01a0, 0x1428a06fa) reads `u8 reason` -
//!                    see [`pet_deactivated`]
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
//! rest of the family: move (`0x0278`), action (`0x0279`), a word for the pet to react to
//! (`0x027A`, reads a `str` and looks it up in the command table), name change (`0x027B`),
//! `0x027C`, `0x027D`, and **`0x027E`, the performer - a trick by index or the eating
//! animation** ([`PET_ACTION_COMMAND`]). "Exception list" for the last was the reference's
//! guess and is wrong; the handler reads a food item id.
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

/// Server -> client: **the pet's long-range pickup boxes.** `0x0198`, case `0x198` of the
/// `0x70..0x19f` dispatcher (`research/msexe-gamestage-cases.txt`), handled inline by
/// `FUN_14113d920` -> `FUN_140424370`. `research/pet-vacuum-wondergrade-2026-09-16.md`.
///
/// ```text
/// raw[16]  near box   l, t, r, b as i32       -> 0x143aca528, used when nothing below matches
/// raw[16]  far box    l, t, r, b as i32       -> 0x143aca538
/// u32      count
/// u32 x n  item ids: the far box applies while the character has one (FUN_1407b3df0)
/// ```
///
/// **The box is used only for a pet whose item says `wonderGrade == 6`**
/// ([`crate::bag::PET_WONDER_GRADE_VACUUM`]); every other pet keeps the constant
/// `(-25,-50,25,10)` around itself, and this packet changes nothing for it. Until this
/// packet arrives both boxes are `(0,0,0,0)` - a grade-6 pet would pick up NOTHING - so it
/// rides after every SetField, the way the keymap and the SP pools do.
pub const PET_PICKUP_RANGE: u16 = 0x0198;

/// The near box this server sends: `(-300,-370,300,220)` around the pet, `[L]` as bytes -
/// the four `i32` sitting at `0x14327c640`, immediately after the walk-over box the pickup
/// scan starts from (`0x14327c630`, `(-25,-50,25,10)`), and referenced by nothing in the
/// image: Nexon's own long-range constant, compiled in beside the short one and left for the
/// packet to supply. Six hundred wide and nearly six hundred tall is most of a screen, which
/// is what a Luna Petite pet reaches in the modern game.
pub const PET_VACUUM_BOX: [i32; 4] = [-300, -370, 300, 220];

/// The `0x0198` body: `near`, `far`, and the item ids that select `far`.
pub fn pet_pickup_range(near: [i32; 4], far: [i32; 4], far_items: &[u32]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    for v in near.iter().chain(far.iter()) {
        w.u32(*v as u32);
    }
    w.u32(far_items.len() as u32);
    for id in far_items {
        w.u32(*id);
    }
    w.into_vec()
}

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
    w.str(&tag_name(&pet.name)); // 141eb99bf  the name tag - see tag_name
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

/// Why a pet was put away, as the OWNER's client reads it after `activated = 0`. `0` is the
/// plain removal; `1..=5` each open a message (`0x1428a0705..0x1428a0725`, the switch arms).
pub const PET_REMOVE_REASON_NONE: u8 = 0;

/// The `0x0277` body that puts character `character_id`'s pet away: `charId, petIdx,
/// activated = 0, u8 reason`.
///
/// # The reason byte was missing, and the first put-away to reach an owner killed the client
///
/// 2026-09-15 23:34, the owner teaching the Husky Auto HP: the reply ended with a put-away and a
/// re-summon for the owner, and the client rejected the put-away in its own words -
/// `0x009E`, reason `0x26` (read past the end), position 15 = the 11-byte packet plus four,
/// the packet verbatim - and faulted at `0x140ce89d6`
/// (`research/fixtures/pet-putaway-to-owner-rejected-0x009E-needs-reason-byte-2026-09-15.log`).
///
/// The **remote** user's handler (`FUN_1429d6150`) stops after `activated`, and that is the
/// path `research/msexe-pet-activated.c` had read. The **local** user's (`FUN_1428a01a0`)
/// calls `SetPet(idx, null)` and then reads **one more `u8`** at `0x1428a06fa` and switches on
/// it - five arms show a message, anything else falls through to the plain removal. **[L]**
/// `tools/reads.py 0x1428a01a0 1` lists that read; no arm reads further. Every put-away this
/// server had sent before today went to observers (companions, the map), never to the owner,
/// so the missing byte was invisible until the owner-side re-summon existed.
///
/// One byte for both audiences: the remote handler returns before it and an unread trailing
/// byte is not a rejection - `0x009E` fires on a read past the end, not on bytes left over.
pub fn pet_deactivated(character_id: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character_id);
    w.u32(PET_INDEX);
    w.u8(0); //                       activated = 0: FUN_1427707e0(user, 0, null)
    w.u8(PET_REMOVE_REASON_NONE); //  1428a06fa  the OWNER reads this; 0 = no message
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
    pet_action_bytes(character_id, index, u8::from(success), message)
}

/// `0x0279` with both command bytes as given - [`pet_line_relay`] passes on what the owner's
/// client reported, byte for byte.
pub fn pet_action_bytes(character_id: u32, first: u8, second: u8, message: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character_id);
    w.u32(PET_INDEX);
    w.u8(first); //  141ec3fa0's first u8  -> FUN_141ec6680's second argument (edi -> edx)
    w.u8(second); // its second            -> the third (ebx -> r8d)
    w.str(message);
    w.into_vec()
}

/// **The owner's client reporting the line its pet just said.** Inbound `0x0203`.
///
/// The builder is the pet performer itself, `FUN_141ec6680(pet, a, b, line, flag)`, and it
/// sends only when `flag != 0` (`cmp [rbp+0x4f0],0 / je` at `141ec76a1`) - which of its eight
/// callers only the `0x027E` arm does, so this follows a feed (`pet_ate`) on the owner's own
/// client. Fields **[L]** (`tools/encodes.py 0x141ec6680`):
///
/// ```text
/// u32  FUN_14019a5d0(pet+0x138)       0 in all 59 deployed captures
/// u32  a per-call value               different every time
/// u8   the performer's second arg     2 in all 59 (food)
/// u8   its third, written 0 when < 9  0x0a for Moth, 0 for everyone else
/// str  the line                       "This is delicious...!", "Bark! Bark bark bark!!!"
/// ```
///
/// No latch after the send. `research/pet-line-report-0x0203-2026-09-25.md` has the whole
/// derivation and the 59 captures.
pub const CLIENT_PET_LINE_REPORT: u16 = 0x0203;

/// A parsed [`CLIENT_PET_LINE_REPORT`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PetLineReport {
    pub pet: u32,
    pub first: u8,
    pub second: u8,
    pub line: String,
}

/// Parse a [`CLIENT_PET_LINE_REPORT`] body (opcode stripped).
pub fn parse_pet_line_report(body: &[u8]) -> Option<PetLineReport> {
    let mut r = PacketReader::new(body);
    let pet = r.u32().ok()?;
    let _per_call = r.u32().ok()?;
    let first = r.u8().ok()?;
    let second = r.u8().ok()?;
    let line = r.str().ok()?;
    Some(PetLineReport { pet, first, second, line })
}

/// **`0x0279` to everyone else on the field: the owner's pet says the owner's line.** The
/// mirror of [`CLIENT_PET_LINE_REPORT`]: `FUN_141ec3fa0` reads `u8 -> edi, u8 -> ebx, str` and
/// calls the performer `(pet, edi, ebx, line, 0)` **[L]** - the report's own three fields, and
/// flag 0, so a relayed line is never reported back.
pub fn pet_line_relay(character_id: u32, report: &PetLineReport) -> Vec<u8> {
    pet_action_bytes(character_id, report.first, report.second, &report.line)
}

/// Server -> client, per user: **the pet performs - a trick by index, or eats.** `FUN_141ec4780`,
/// the `0x27e` arm of the pet sub-dispatcher. `research/msexe-pet-activated.c` had it down as
/// "exception list" from the reference's opcode order; the listing says otherwise **[L]**:
///
/// ```text
/// 141ec47f7  u8  nType                    1 = a trick, 2 = food, anything else: nothing
///   nType 1:
/// 141ec49f9  u8  interact index           bounds-checked against the pet's own table
/// 141ec4a50  u8  success                  picks the success (+0x18) or fail (+0x20) lines
/// 141ec4a75  u8  1 or 2                   which line; other values skip the line
///   nType 2:
/// 141ec4b0c  u8  success
/// 141ec4b24  u32 itemId                   kept only if 2120000 <= id < 2130000 (a pet food)
/// then FUN_141ec6680(pet, nType, entry, line, 1) - the same performer 0x0279 calls with 0
/// ```
///
/// So the eating animation is `nType 2` with the food's id - [`pet_ate`]. The handler looks
/// the food up in the pet's own food table by the pet's level (`[pet+0x68]`, 24-byte rows with
/// a level range), so whether a given pet at a given level has an entry is the pet image's
/// business; a miss falls through to the common tail rather than a fault.
pub const PET_ACTION_COMMAND: u16 = 0x027E;

/// `PET_ACTION_COMMAND` type 2. The one value the food branch reads first.
pub const PET_ACTION_FOOD: u8 = 2;

/// The `0x027E` body for a successful feed: `u32 charId, u32 petIdx, u8 2, u8 1, u32 foodId`.
/// The owner, 2026-09-16: *"I do want the eating animation to play for the client and other
/// players."* Sent to the owner and published to the map.
///
/// **`food_item_id` is what makes the "Yum, yum!" balloon, and a hand-fed pet sends 0.** The owner,
/// 2026-09-18, with the balloon over a pet they had just fed by hand: *"The pet food dialogue
/// shown here should only be shown when it is being automatically fed, but that functionality
/// does not exist in our server because the pet skill does not exist. It's also off by 1."*
/// Both are the handler's own arithmetic, read at the listing **[L]**:
///
/// ```text
/// 141ec4b24  u32 itemId; kept only if 2120000 <= id < 2130000, else 0    -> r15d
/// 141ec4c52  if nType == 2 && r15d > 0:
/// 141ec4c80    count = the Use tab's count of that item          (FUN_1402e9260)
/// 141ec4c8a    shown = max(count - 1, 0)                          <- the "off by 1"
/// 141ec4d46    string 0x1039 "Yum, yum! %s x%d left!" -> the balloon
/// ```
///
/// The eating animation is chosen before that block, from the pet's own food table by its
/// level (`[pet+0x68]`), and never reads the id. So the balloon is an auto-feed message that
/// assumes the client's bag has not been decremented yet - this server sends the `0x0070`
/// first, which is why it read one short - and the id is the only switch for it. A manual feed
/// sends `0`: the pet eats, no balloon. [`PET_FOOD_NONE`]. Nothing here sends a non-zero id
/// until an auto-feed skill exists.
pub fn pet_ate(character_id: u32, food_item_id: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character_id);
    w.u32(PET_INDEX);
    w.u8(PET_ACTION_FOOD);
    w.u8(1); // success
    w.u32(food_item_id);
    w.into_vec()
}

/// The food id a hand-fed pet's [`pet_ate`] carries: outside the pet-food range, so the
/// handler plays the eating animation and skips the "Yum, yum! %s x%d left!" balloon that
/// belongs to an auto-feed.
pub const PET_FOOD_NONE: u32 = 0;

/// **The pet-effect arm of `UserEffect`.** `FUN_1427863f0`'s second switch, table
/// `0x142791348`, index **9** -> `0x14278df5b`: reads `u8 subtype`, `u32 petIdx`, finds the
/// user's pet (`FUN_1427703d0`) and calls `CPet::OnEffect(pet, subtype)` = `FUN_141ebf340`
/// **[L]**. Every other arm of the 85 was scanned for a `GetPet` call and only this one (and
/// arm 69, which is not a pet arm - it reads a `u32` and formats a string) has one.
pub const USER_EFFECT_PET: u8 = 9;

/// `CPet::OnEffect` subtype **0** is `Effect/PetEff.img/Basic/LevelUp` - `0x141ebf454 test
/// edi, edi / je 0x141ebf686`, and that arm loads the LevelUp string at `0x141ebf686`. 1 is
/// `Basic/Teleport`, 2 is `%07d/warp`. **[L]**
pub const PET_EFFECT_LEVEL_UP: u8 = 0;

/// `0x02D1` `UserEffectLocal` for the owner: `u8 9, u8 0, u32 petIdx`. The owner, 2026-09-16:
/// *"When closeness levels up, it should also play an animation to the client and other
/// players in the map."*
pub fn pet_level_up_local() -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(USER_EFFECT_PET);
    w.u8(PET_EFFECT_LEVEL_UP);
    w.u32(PET_INDEX);
    w.into_vec()
}

/// `0x02AF` `UserEffectRemote` for everyone else: `u32 charId`, then the same three fields. The
/// remote handler runs the same decoder (`FUN_1427863f0`) against the remote user.
pub fn pet_level_up_remote(character_id: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character_id);
    w.u8(USER_EFFECT_PET);
    w.u8(PET_EFFECT_LEVEL_UP);
    w.u32(PET_INDEX);
    w.into_vec()
}

/// The `0x027B` body: `u32 charId, u32 petIdx, str name`.
pub fn pet_name_changed(character_id: u32, name: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(character_id);
    w.u32(PET_INDEX);
    w.str(&tag_name(name));
    w.into_vec()
}

/// **The name drawn on the tag under a summoned pet is folded to ASCII.** The owner, 2026-10-04:
/// Lil Übel's tag read `Lil □bel` in the field while the Cash Shop and the inventory drew
/// the `Ü` - those take the name from the client's own WZ strings, and the tag takes it from
/// this packet. `PacketWriter::str` sends `Ü` as the one byte `0xDC`, and the tag's font has no
/// glyph there: the chat notices' fault of 2026-09-11 (`crate::notice::ascii_fold`) on
/// another layer. The stored name keeps its accent; only the wire copy is folded.
fn tag_name(name: &str) -> String {
    crate::notice::ascii_fold(name)
}

/// A stable, non-zero serial for one of a character's pets: the character id in the high
/// half, the **pet id** (`store::pets`' row, 2026-09-16 - the item id until then) in the low.
/// The same value goes into the pet body and into the Cash-tab item, which is all the client
/// needs of it, and a request that carries it back names one pet - one Husky of two.
pub fn pet_serial(character_id: u32, pet_id: u32) -> std::num::NonZeroU64 {
    let raw = (u64::from(character_id) << 32) | u64::from(pet_id);
    std::num::NonZeroU64::new(raw).unwrap_or(std::num::NonZeroU64::MIN)
}

/// Bytes of a `0x0277` activation for a pet whose name is `name_len` bytes long.
pub const fn pet_activated_len(name_len: usize) -> usize {
    4 + 4 + 1 + 1 + 4 + (2 + name_len) + 8 + 2 + 2 + 1 + 2 + 4 + 4 + 2 + 2 + 1 + 1
}

#[cfg(test)]
mod pet_line_tests {
    use super::*;

    /// Two of the 59 deployed captures, byte for byte, and the relay each becomes.
    #[test]
    fn the_deployed_line_reports_parse_and_relay_as_0x0279() {
        let hex = |h: &str| (0..h.len()).step_by(2).map(|i| u8::from_str_radix(&h[i..i + 2], 16).unwrap()).collect::<Vec<u8>>();
        let moth = parse_pet_line_report(&hex("0000000061107122020a1500546869732069732064656c6963696f75732e2e2e21")).unwrap();
        assert_eq!(moth, PetLineReport { pet: 0, first: 2, second: 0x0a, line: "This is delicious...!".into() });
        let pebble = parse_pet_line_report(&hex("00000000a896ee19020017004261726b21204261726b206261726b206261726b212121")).unwrap();
        assert_eq!((pebble.first, pebble.second, pebble.line.as_str()), (2, 0, "Bark! Bark bark bark!!!"));

        let relay = pet_line_relay(219, &moth);
        let mut want = 219u32.to_le_bytes().to_vec();
        want.extend(PET_INDEX.to_le_bytes());
        want.extend([2, 0x0a]);
        want.extend(hex("1500546869732069732064656c6963696f75732e2e2e21"));
        assert_eq!(relay, want, "charId, petIdx, the two bytes as reported, the same str");
        assert_eq!(parse_pet_line_report(&hex("0000000061107122020a15")), None, "a cut-off string is refused");
    }
}

#[cfg(test)]
mod tests {
    /// `0x0198` as `FUN_140424370` reads it: two raw 16-byte boxes, a count, then the ids.
    #[test]
    fn the_pickup_range_body_is_two_boxes_a_count_and_the_ids() {
        let b = super::pet_pickup_range(super::PET_VACUUM_BOX, [-1, -2, 3, 4], &[5_000_006]);
        assert_eq!(b.len(), 16 + 16 + 4 + 4);
        let i32_at = |o: usize| i32::from_le_bytes(b[o..o + 4].try_into().unwrap());
        assert_eq!([i32_at(0), i32_at(4), i32_at(8), i32_at(12)], [-300, -370, 300, 220]);
        assert_eq!([i32_at(16), i32_at(20), i32_at(24), i32_at(28)], [-1, -2, 3, 4]);
        assert_eq!(i32_at(32), 1);
        assert_eq!(i32_at(36), 5_000_006);
        let none = super::pet_pickup_range(super::PET_VACUUM_BOX, super::PET_VACUUM_BOX, &[]);
        assert_eq!(none.len(), 36);
        assert_eq!(&none[32..36], &[0, 0, 0, 0]);
        // The box is a superset of the walk-over one, and centred where a pet stands.
        assert!(super::PET_VACUUM_BOX[0] < -25 && super::PET_VACUUM_BOX[2] > 25);
        assert!(super::PET_VACUUM_BOX[1] < -50 && super::PET_VACUUM_BOX[3] > 10);
    }

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
        let ubel = pet_activated(215, &FieldPet { item_id: 5_000_475, name: "Lil Übel".to_string(), ..pet.clone() });
        assert_eq!(&ubel[14..24], b"\x08\x00Lil Ubel", "the tag's name goes out as ASCII - 0xDC drew a box");
        assert_eq!(&pet_name_changed(215, "Übel")[8..], b"\x04\x00Ubel");
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

    /// The owner's handler reads a reason byte after `activated = 0`; without it the client
    /// rejected the packet (`0x009E`, pos 15 = 11 + 4) and died, 2026-09-15 23:34.
    #[test]
    fn putting_a_pet_away_carries_the_reason_byte_the_owner_reads() {
        let b = pet_deactivated(215);
        assert_eq!(b, [0xd7, 0, 0, 0, 0, 0, 0, 0, 0, PET_REMOVE_REASON_NONE]);
        assert_eq!(b.len(), 10, "eleven with the opcode - the rejected packet was eleven MINUS this byte");
        // The rejected packet, verbatim from the 0x009E: ours ended at activated.
        let rejected = [0x77u8, 0x02, 0xd7, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(&rejected[2..], &b[..9]);
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
