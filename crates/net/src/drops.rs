//! Items on the ground: dropping one, seeing it, and picking it up.
//!
//! Every address, every read site and the "what I did NOT establish" list is in
//! `research/item-drop.md`. Read that before changing a field here.
//!
//! # The three packets, and which of them is guesswork
//!
//! | | | |
//! |---|---|---|
//! | the request to drop | inbound `0x0107` with **`dst == 0`** | **read off the client** |
//! | the item appearing on the ground | outbound [`DROP_ENTER_FIELD`] `0x046E` | **read off the client** |
//! | the item leaving the ground | outbound [`DROP_LEAVE_FIELD`] `0x046F` | **read off the client** |
//! | the request to **pick up** | **not found** | see [`the_pickup_request_is_not_decoded`] |
//!
//! `0x0301` is in here too and it is **not** the player's pick-up. It is a *mob* picking a
//! drop up, reported by whichever client controls that mob. Reading it as the player's
//! request is a mistake this module exists partly to prevent - see
//! [`CLIENT_MOB_DROP_PICK_UP`].
//!
//! # The one rule that decides the body length
//!
//! `DropEnterField`'s handler has **two tails**, and they read a different number of bytes.
//! [`drop_enter_field`] always builds the **longer** one, and
//! [`why_the_long_tail_is_the_safe_one`] is the argument for why that is safe in both
//! directions. A short packet has killed this client twice; a long one has never hurt it,
//! because the frame carries its own length.

use crate::inventory::InventoryMove;
use crate::PacketWriter;

/// **`0x046E` DropEnterField** - an item is now lying on the ground.
///
/// Routed by `CField::OnPacket` (`FUN_141820080`) through the range test at `0x141821ec4`
/// (`lea eax,[r9-0x46e] / cmp eax,1`) to the drop pool's two-case `OnPacket`
/// `FUN_1417a1c30` on singleton `[0x143ACE240]`, whose `0x46E` arm tail-jumps to
/// `FUN_1417a2ee0`. **[L]**
///
/// The pool is exactly two opcodes wide, which is what identifies it: `0x469..0x46D` above
/// it is a five-opcode pool and `0x470..0x472` below it is a three-opcode one.
pub const DROP_ENTER_FIELD: u16 = 0x046E;

/// **`0x046F` DropLeaveField** - a drop is gone, and [`LeaveType`] says how.
///
/// `FUN_1417a1c30`'s other arm, tail-jumping to `FUN_1417ad7a0`. **[L]**
pub const DROP_LEAVE_FIELD: u16 = 0x046F;

/// **`0x0301`, inbound, and it is NOT the player picking something up.**
///
/// `FUN_141c8a4b0(mob, dropObjectId)` builds it: `u32 [mob+0x3a0]`, `u32 dropObjectId`.
/// `mob+0x3a0` is the **mob's** object id - the same field `research/mob-behaviour.md`
/// section 11 reads as field 0 of the outbound `0x02FF` MobMove. So this is the controlling
/// client saying *"the mob I am running walked onto drop N"*.
///
/// It is sent from `FUN_14179dd80`, a drop-pool proximity sweep called from the **mob**
/// update `FUN_141c626c0`, over a box of `x-0x14..x+0x14` by `y-0x28..y+0x0a`. It is rate
/// limited to one request per drop id per **3000 ms** by a list at `mob+0x5f8`.
///
/// This nearly went into the repo as "the pick-up request", which would have been wrong in
/// the most expensive way: the field it carries is a mob id, so the server would have looked
/// up a character that does not exist. What caught it was that `0x02FF` is already decoded
/// here as MobMove and reads `[obj+0x3a0]` as its object id from the same offset.
///
/// **Nothing latches on it** - `FUN_141c8a4b0` returns 1 the moment it has sent - so an
/// unanswered `0x0301` costs one mob's item theft and nothing else. It still must not be
/// answered with an *error*; see the module docs of `crate::inventory`.
pub const CLIENT_MOB_DROP_PICK_UP: u16 = 0x0301;

/// `dropType`, body offset 0. Selects which class the client allocates.
///
/// `FUN_1417a2ee0`'s first read (`0x1417a2f2b`) is a three-way branch: `1` allocates the
/// 0x238-byte object and then **overwrites its vtable** with `0x1433D3F68`; `0` and
/// everything else leave the constructor's vtable alone. **[L]**
pub const DROP_TYPE_MESO: u8 = 0;
/// See [`DROP_TYPE_MESO`]. This is the value to send for a real item.
pub const DROP_TYPE_ITEM: u8 = 1;

/// How the drop arrives on screen. Body offset 1, and it changes the body length.
///
/// Read at `0x1417a2fde` into `drop+0x60`, and it drives two things:
///
/// * **The source-position block.** `0x1417a41b1` is `mov eax,[rbp+0x188] / test
///   eax,0xfffffffc / jne skip / cmp eax,2 / je skip`, so the `i16 srcX, i16 srcY, u32
///   delay` block is read **only for enter type 0, 1 or 3**. **[L]**
/// * **Whether the player can ever pick it up.** `0x1417a3041` is `lea eax,[r15-1] / cmp
///   eax,1 / setbe bl` into `drop+0x61`, so `drop+0x61` is set only for enter type **1 or
///   2** - and every pick-up sweep in the client gates on `drop+0x61 != 0`. Sending enter
///   type 0 or 3 produces a drop that is drawn and cannot be collected. **[L]**
///
/// So the only two values worth sending are 1 and 2, and [`ENTER_FLOATING`] is the one that
/// also animates.
pub mod enter_type {
    /// Appears with no animation. **Not pickable** - see the module doc.
    pub const DEFAULT: u8 = 0;
    /// Animates from the source position to the resting position, and is pickable.
    pub const FLOATING: u8 = 1;
    /// Already on the ground, pickable, and **skips the source-position block**.
    pub const INSTANT: u8 = 2;
    /// Fades in. **Not pickable.**
    pub const FADE_AWAY: u8 = 3;
}

/// The enter type to use for an item a player has just thrown out of the bag.
///
/// `1` rather than `2` so the client plays the arc from the player's feet, and because
/// `drop+0x61` - the pick-up gate - is set for `1` and `2` only.
pub const ENTER_FLOATING: u8 = enter_type::FLOATING;

/// The enter type for an item that is simply already lying there (a re-send on field entry).
pub const ENTER_INSTANT: u8 = enter_type::INSTANT;

/// `ownType`, body offset 27. Read at `0x1417a3539` into `drop+0x70`.
///
/// The client stores it and never gates on it, so **ownership is entirely the server's job**
/// here - the client will happily ask to pick up a drop it does not own. **[L]** for the
/// store, **[I]** for the meaning of each value, which comes from the reference.
pub const OWN_TYPE_USER: u8 = 0;
/// See [`OWN_TYPE_USER`].
pub const OWN_TYPE_PARTY: u8 = 1;
/// See [`OWN_TYPE_USER`]. "Anyone may take it."
pub const OWN_TYPE_EVERYONE: u8 = 2;

/// How a drop disappears. `DropLeaveField` body offset 4, and it changes the body length.
///
/// Read at `0x1417ad7d7` **after** the object id, and dispatched twice: a `cmp r14d,4` at
/// `0x1417ad81e`, a three-way at `0x1417ad8e5`, and then a nine-entry jump table at
/// `0x1417b21d0` covering `0..8`. **[L]**
pub mod leave_type {
    /// No extra fields.
    pub const FADE: u8 = 0;
    /// No extra fields.
    pub const NO_FADE: u8 = 1;
    /// `u32 pickUpCharacterId`. The animation arm is `0x1417b0311`.
    pub const CHAR_PICKUP: u8 = 2;
    /// `u32 pickUpCharacterId`. A second variant; arm `0x1417b0c7f`.
    pub const CHAR_PICKUP_2: u8 = 3;
    /// **Three `u32`s**, not the reference's one `u16`. Arm `0x1417ad982`.
    pub const DELAYED_PICKUP: u8 = 4;
    /// `u32 pickUpCharacterId` **then** `u32 petId` from the jump-table arm `0x1417b0d2c`.
    pub const PET_PICKUP: u8 = 5;
    /// No extra fields.
    pub const FADE_2: u8 = 6;
    /// `u32 key`, read in the jump-table arm at `0x1417ada08`.
    pub const ABSORB: u8 = 7;
    /// No extra fields. The last slot the jump table covers.
    pub const EIGHT: u8 = 8;
}

/// A `0x0107` whose destination slot is `0` is a **drop**, not a move.
///
/// This is read off the client's own send builder `FUN_142cc5b00`, not inferred from the one
/// capture. The very first thing that builder does after its rate gate is
///
/// ```asm
/// 142cc5bb1  TEST EDI,EDI          ; dst
/// 142cc5bb3  JE   142cc5c3b        ; dst == 0 -> the drop arm
/// ...
/// 142cc5c3b  CALL 141892840        ; the current field
/// 142cc5c43  CALL 14182e950        ; a FIELD-level predicate
/// 142cc5c48  TEST EAX,EAX
/// 142cc5c4a  JE   142cc5c6f        ; allowed here -> carry on to the send
/// 142cc5c4c  MOV  EDX,0xc2f        ; otherwise show string 0xc2f
/// 142cc5c64  CALL 1415eca30
/// 142cc5c6a  JMP  142cc5cf5        ; -> return 0.  NOTHING is sent.
/// ```
///
/// So `dst == 0` is a first-class case in the client with its own **field-level** refusal,
/// which is only meaningful for a drop, and it deliberately bypasses the "is this slot legal"
/// arm that a move goes through. There is no second opcode: `FUN_1406ed520(buf, 0x107)` at
/// `0x142cc5eac` is the only packet this function builds, `0x0107` has exactly **one**
/// builder in `research/msexe-send-opcodes.txt`, and all **five** callers of
/// `FUN_142cc5b00` (`tools/callers.py`) reach that one construction. **[L]**
///
/// The captured drop, byte for byte out of `world.log` on 2026-08-20:
///
/// ```text
/// <- 0x0107, 11 byte body  75981808 01 0100 0000 0100
///                          tick     ^  ^    ^    ^ count 1
///                          invType 1 |    dst 0
///                                    src 1
/// ```
///
/// `count` is a real quantity here, not the `-1` an unequip carries: the client writes it
/// with the `u16` writer at `0x142cc5ef3` from its own drag dialog. The five writes in order
/// are `0x142cc5ebd` (u32 tick), `0x142cc5eca` (u8 invType), `0x142cc5ed7` (u16 src),
/// `0x142cc5ee3` (u16 dst), `0x142cc5ef3` (u16 count), and then `0x142cc5f01` is literally
/// `MOV dword ptr [RSI+0x2330],1` - the latch, set on a drop exactly as on a move.
pub const MOVE_DST_IS_A_DROP: i16 = 0;

/// Whether this `0x0107` is asking to throw the item on the ground.
pub fn is_a_drop(m: &InventoryMove) -> bool {
    m.dst == MOVE_DST_IS_A_DROP
}

/// How many of the item the client is asking to drop, clamped to something sane.
///
/// The client sends the quantity for a bundle and `1` for a single item. A negative value
/// has never been seen on a drop - `-1` is what an *unequip* carries - so it is treated as
/// one item rather than trusted.
pub fn drop_count(m: &InventoryMove) -> u16 {
    if m.count <= 0 {
        1
    } else {
        m.count as u16
    }
}

/// One item lying on the ground, as the server has to remember it.
///
/// The server owns four things the client cannot re-derive: the **object id** (the pool's
/// hash key), the **position**, the **owner**, and **when it expires**. Everything else in
/// the packet is either a constant or comes from the item itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldDrop {
    /// The pool key. `0x1417a2ff1` reads it and `0x1417a3014` looks it up **before** reading
    /// anything else: on a hit the handler jumps to `0x1417a46cd` and **the rest of the body
    /// is never read**. Same shape as the NPC pool. Ids must be unique within a field.
    pub object_id: u32,
    /// The item template id. Stored obfuscated at `drop+0x90`/`drop+0x98`.
    pub item_id: u32,
    /// Meso amount when [`FieldDrop::is_money`]; otherwise unused by this builder.
    pub meso: u32,
    /// The character allowed to pick it up, or `0` for nobody in particular.
    pub owner_id: u32,
    /// Whose feet it flies out of, or `0`. A character id or a mob object id.
    pub source_object_id: u32,
    /// Where it comes to rest.
    pub x: i16,
    /// See [`FieldDrop::x`].
    pub y: i16,
    /// Where the arc starts. Ignored when the enter type is [`ENTER_INSTANT`].
    pub source_x: i16,
    /// See [`FieldDrop::source_x`].
    pub source_y: i16,
    /// Milliseconds of flight. Ignored when the enter type is [`ENTER_INSTANT`].
    pub delay: u32,
    /// `1` puts the client on its meso path. Nothing in this server sets it yet.
    pub is_money: bool,
    /// See [`OWN_TYPE_USER`].
    pub own_type: u8,
}

impl FieldDrop {
    /// An item dropped by a player at their own feet.
    pub fn item(object_id: u32, item_id: u32, owner_id: u32, x: i16, y: i16) -> Self {
        FieldDrop {
            object_id,
            item_id,
            meso: 0,
            owner_id,
            source_object_id: owner_id,
            x,
            y,
            source_x: x,
            source_y: y,
            delay: 0,
            is_money: false,
            own_type: OWN_TYPE_USER,
        }
    }

    /// What goes in the id field: the item template, or the meso amount for money.
    fn wire_id(&self) -> u32 {
        if self.is_money {
            self.meso
        } else {
            self.item_id
        }
    }
}

/// Body length of a [`drop_enter_field`] for an item, source block included.
///
/// 84 for the head (reads 1..27), 8 for the source block, 2 for `isExplosiveDrop` and
/// `isSpecialDrop`, 27 for the long tail with the expiry in it. Pinned by a test so that a
/// field added by hand cannot change the length without someone noticing.
pub const DROP_ENTER_FIELD_LEN: usize = 84 + 8 + 29;

/// Body length when the enter type is [`ENTER_INSTANT`], which omits the source block.
pub const DROP_ENTER_FIELD_LEN_INSTANT: usize = DROP_ENTER_FIELD_LEN - 8;

/// Build **`0x046E` DropEnterField**.
///
/// Field order is read off the listing of `FUN_1417a2ee0`; the meaning of the fields that are
/// not stored anywhere interesting is a candidate from the v214 reference, and
/// `research/item-drop.md` tags every row. The reference matched this client's read sequence
/// for the first **27 fields in a row, type for type**, which is far past coincidence - but
/// it is still the instrument `CLAUDE.md` scores at 1 of 8, so treat the *names* as labels
/// and the *offsets* as facts.
///
/// ```text
/// u8   dropType          0 meso, 1 item                  -> class selection
/// u8   enterType                                         -> drop+0x60, and drop+0x61
/// u32  objectId                                          -> drop+0x64, the pool key
/// u8   isMoney                                           -> obfuscated at drop+0x78/0x80
/// u32  motionType                                        -> drop+0x194
/// u32  random                                            -> drop+0x198
/// u32  (unknown)                                         -> float(v)/k  -> drop+0x1d8
/// u32  itemId / meso amount                              -> obfuscated at drop+0x90/0x98
/// u32  ownerId                                           -> drop+0x68
/// u8   ownType                                           -> drop+0x70
/// i16  x, i16 y                                          -> obfuscated point drop+0x128
/// i16  x4 (unknown)                                      -> drop+0x218..0x224
/// u32  sourceObjectId                                    -> drop+0x6c
/// u32  (unknown)                                         -> drop+0x1e0
/// u64  (unknown)                                         -> drop+0x1e8
/// u32  (unknown)                                         -> drop+0x1f0
/// u8   (unknown, bool)                                   -> drop+0x210
/// u64, u32, u64                                          -> drop+0x1f8/0x200/0x208
/// u8   moneyType                                         -> drop+0x16d
/// u8   (unknown, bool)
/// u8   (unknown, bool)
/// -- only when enterType is 0, 1 or 3 --
/// i16  srcX, i16 srcY, u32 delay
/// --
/// u8   isExplosiveDrop                                   -> drop+0x168
/// u8   isSpecialDrop                                     -> drop+0x228
/// -- the long tail, see why_the_long_tail_is_the_safe_one --
/// u64  expire FILETIME   (only when isMoney == 0)        -> drop+0x158, read as 8 raw bytes
/// u8   canBePickedUpByPet                                -> drop+0x160
/// u8   (unknown - a NON-ZERO here fires FUN_140dbb710(0xc0041f15))
/// i16  fallingVY                                         -> drop+0x164
/// u8   fadeInEffect
/// u32  collisionPickUp
/// u8   itemGrade
/// u8   prepareCollisionPickUp                            -> drop+0x190
/// u32  (unknown)                                         -> drop+0x19c
/// u32  (unknown)                                         -> drop+0x1a0
/// ```
///
/// # The expiry is sent as zero on purpose
///
/// See [`why_the_long_tail_is_the_safe_one`]. Zero is not a placeholder here - it is the
/// value the client's own two short paths write into `drop+0x158`, so it is by construction
/// a value it knows how to handle.
pub fn drop_enter_field(d: &FieldDrop, enter_type: u8) -> Vec<u8> {
    let mut w = PacketWriter::new();

    w.u8(if d.is_money { DROP_TYPE_MESO } else { DROP_TYPE_ITEM });
    w.u8(enter_type);
    w.u32(d.object_id);

    w.bool(d.is_money);
    w.u32(0); // motionType. drop+0x194; a NON-ZERO value stops the client queueing a
              // pick-up at all - FUN_1417a2b00 tests `[drop+0x194] != 0` and skips.
    w.u32(0); // the reference randomises this; nothing reads it back
    w.u32(0); // -> float(v)/k -> drop+0x1d8
    w.u32(d.wire_id());
    w.u32(d.owner_id);
    w.u8(d.own_type);
    w.pos(d.x, d.y);
    w.i16(0);
    w.i16(0);
    w.i16(0);
    w.i16(0);
    w.u32(d.source_object_id);
    w.u32(0);
    w.u64(0);
    w.u32(0);
    w.u8(0);
    // FUN_1403747c0: u64, u32, u64, unconditional, straight into drop+0x1f8/0x200/0x208.
    w.u64(0);
    w.u32(0);
    w.u64(0);
    w.u8(0); // moneyType -> drop+0x16d
    w.u8(0);
    w.u8(0);

    if enter_type_reads_the_source_block(enter_type) {
        w.pos(d.source_x, d.source_y);
        w.u32(d.delay);
    }

    w.u8(0); // isExplosiveDrop -> drop+0x168
    w.u8(0); // isSpecialDrop   -> drop+0x228

    if !d.is_money {
        w.u64(0); // expire FILETIME, read as 8 raw bytes at 0x1417a9bab
    }
    w.u8(1); // canBePickedUpByPet
    w.u8(0); // a non-zero here fires an effect we have not decoded - keep it zero
    w.i16(0); // fallingVY
    w.u8(0); // fadeInEffect
    w.u32(0); // collisionPickUp
    w.u8(0); // itemGrade
    w.u8(0); // prepareCollisionPickUp
    w.u32(0);
    w.u32(0);

    w.into_vec()
}

/// Whether this enter type makes the client read `i16 srcX, i16 srcY, u32 delay`.
///
/// The client's test, literally: `test eax,0xfffffffc / jne skip / cmp eax,2 / je skip` at
/// `0x1417a41b1`. So: enter type **0, 1 or 3**. The v214 reference writes the same block for
/// `enterType != 2 && enterType < 5`; mscw's upper bound is **4**, not 5. That difference is
/// invisible for every value this server sends and is recorded so nobody "fixes" it back.
pub fn enter_type_reads_the_source_block(enter_type: u8) -> bool {
    enter_type < 4 && enter_type != enter_type::INSTANT
}

/// Why [`drop_enter_field`] always writes the longer of the client's two tails.
///
/// `FUN_1417a2ee0` splits at `0x1417a800b` on `[rbp+0x1b4]`, which is set at `0x1417a411b`
/// from `itemId == 0`. The two arms read **different numbers of bytes**:
///
/// | | short arm (from `0x1417a833d`) | long arm (from `0x1417a9bab`) |
/// |---|---|---|
/// | expire FILETIME | not read | read, unless `isMoney` |
/// | the seven tail fields | read | read |
/// | two trailing `u32` | not read | read |
///
/// **Which arm a real item takes is not established** - `research/item-drop.md` says so at
/// the top. It does not have to be, because the two failure modes are not symmetric:
///
/// * Send the **short** body and let the client take the long arm and it reads **16 bytes
///   past the end of the frame**. That is the failure that killed this client on a chat
///   packet and again on a mob body.
/// * Send the **long** body and let the client take the short arm and it simply stops early.
///   Surplus bytes are inert - the frame carries its own length.
///
/// The one thing that could still go wrong is the short arm reading the *expiry* bytes as
/// its seven tail fields. That is why the expiry is written as **zero**: under the short arm
/// every one of those fields then reads as `0`, which is exactly what the client writes into
/// them itself on both of its own short paths (`0x1417a832a` and `0x1417a9bcb` both store
/// `drop+0x158 = 0`). A non-zero expiry would put `0xFF`s into a byte at `0x1417a8367` whose
/// only known effect is to fire `FUN_140dbb710(…,0xc0041f15)`.
pub fn why_the_long_tail_is_the_safe_one() {}

/// Why there is no `pickup_request` parser in this module.
///
/// The player's pick-up path is followed all the way and then stops at a wall:
///
/// ```text
/// FUN_1428af6d0   the key handler, at 0x1428b0cec loads the drop pool [0x143ACE240]
///   -> FUN_14179c9d0(pool, &userPos)      the sweep, box x-0x19..x+0x19, y-0x32..y+0x0a
///        -> FUN_142cc6770(ctx, pos, dropObjectId, ...)   the CWvsContext pre-checks
///             -> FUN_1417a2b00(pool, dropObjectId)  queues the id and sets pool+0x90 = 1
///             -> jmp 0x144f14cca                    <-- into the Themida VM section
/// ```
///
/// Neither `FUN_14179c9d0` nor `FUN_142cc6770` constructs a packet: a sweep of **every**
/// `call 0x1406ed520` in `0x141780000..0x1417f0000` finds four, and their opcodes are
/// `0x25f`, `0x19d`, `0x12d` and `0x116`. `0x25f`'s builder `FUN_1417d29d0` is called only
/// from inside `DropEnterField` and its body is `u32 itemId, u32 (arg)` - not a pick-up.
///
/// So the request is either built inside the virtualised region or from a site whose opcode
/// is not a `mov edx, imm32`. The outbound table has thirteen such sites and none of them is
/// in the drop pool.
///
/// **The cheap way to finish this is a measurement, not more static analysis.** The world
/// server logs every inbound opcode; walking a character over a drop names it in one line of
/// `world.log`. The static work narrows where to look: the outbound pools run mob
/// `0x02FF..~0x0323`, NPC `0x0327..0x0328`, and then a gap of **`0x0329..0x032E`** before
/// the next claimed opcode. The drop pool's request is in that gap. The reference's block
/// layout puts it at `0x032C`; that is **[I]** and the same reference scored 1 of 8 on a
/// control.
pub fn the_pickup_request_is_not_decoded() {}

/// Body length of [`drop_leave_field`] for a leave type that carries no extra field.
pub const DROP_LEAVE_FIELD_LEN: usize = 5;

/// Body length of [`drop_picked_up_by_character`].
pub const DROP_PICKED_UP_LEN: usize = 9;

/// Build **`0x046F` DropLeaveField** for a leave type that carries nothing extra.
///
/// ```text
/// u32 objectId    read FIRST, at 0x1417ad7cd, and it is the pool's hash key
/// u8  leaveType   read SECOND, at 0x1417ad7d7
/// ```
///
/// **The order is the opposite of the v214 reference**, which writes the type first. This is
/// read off the listing: the `u32` at `0x1417ad7cd` goes straight into the `div`/bucket walk
/// at `0x1417ad7fe`, and the `u8` at `0x1417ad7d7` is what `cmp r14d,4` and the nine-entry
/// jump table at `0x1417b21d0` dispatch on. Getting this pair the reference's way round
/// would look almost right - a small object id and a small type byte - and silently address
/// the wrong drop.
///
/// # Panics
///
/// Never. A leave type that needs an extra field is not accepted here; use
/// [`drop_picked_up_by_character`] for 2, 3 and 5.
pub fn drop_leave_field(object_id: u32, leave_type: u8) -> Vec<u8> {
    debug_assert!(
        !leave_type_carries_a_character(leave_type)
            && leave_type != leave_type::DELAYED_PICKUP
            && leave_type != leave_type::ABSORB,
        "leave type {leave_type} needs an extra field - use the builder that supplies it"
    );
    let mut w = PacketWriter::new();
    w.u32(object_id);
    w.u8(leave_type);
    w.into_vec()
}

/// Build **`0x046F` DropLeaveField** for "character N picked it up".
///
/// ```text
/// u32 objectId
/// u8  leaveType = 2
/// u32 pickUpCharacterId    -> drop+0xec, read at 0x1417ad903
/// ```
///
/// The extra `u32` is read for leave types **2, 3 and 5** - the three-way at `0x1417ad8e5`
/// funnels all three into the same read - and for nothing else. Type 5 then reads a second
/// `u32` (the pet) from its jump-table arm at `0x1417b0d2c`.
///
/// `drop+0xec` is compared against the local character's id inside the type-7 arm, and the
/// type-2 arm at `0x1417b0311` uses it to aim the pick-up animation, so **it has to be the
/// real character id**, not a placeholder.
pub fn drop_picked_up_by_character(object_id: u32, character_id: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(object_id);
    w.u8(leave_type::CHAR_PICKUP);
    w.u32(character_id);
    w.into_vec()
}

/// Whether this leave type makes the client read a trailing `u32` character id.
pub fn leave_type_carries_a_character(leave_type: u8) -> bool {
    matches!(
        leave_type,
        leave_type::CHAR_PICKUP | leave_type::CHAR_PICKUP_2 | leave_type::PET_PICKUP
    )
}

/// A parsed [`CLIENT_MOB_DROP_PICK_UP`]. **A mob, not the player.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MobDropPickUp {
    /// The mob's object id, from `mob+0x3a0`.
    pub mob_object_id: u32,
    /// The drop the mob walked onto, from `drop+0x64`.
    pub drop_object_id: u32,
}

/// Parse a [`CLIENT_MOB_DROP_PICK_UP`] body (opcode already stripped).
pub fn parse_mob_drop_pick_up(body: &[u8]) -> Option<MobDropPickUp> {
    if body.len() < 8 {
        return None;
    }
    Some(MobDropPickUp {
        mob_object_id: u32::from_le_bytes([body[0], body[1], body[2], body[3]]),
        drop_object_id: u32::from_le_bytes([body[4], body[5], body[6], body[7]]),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::parse_inventory_move;

    /// The captured drop from 2026-08-20, byte for byte, parsed by the module that owns
    /// `0x0107` and classified by this one.
    #[test]
    fn the_captured_drop_is_a_drop_and_an_unequip_is_not() {
        let drop = [
            0x75, 0x98, 0x18, 0x08, // tick
            0x01, // invType 1 = Equip
            0x01, 0x00, // src 1
            0x00, 0x00, // dst 0
            0x01, 0x00, // count 1
        ];
        let m = parse_inventory_move(&drop).expect("11 bytes is a whole body");
        assert_eq!(m.src, 1);
        assert_eq!(m.dst, 0);
        assert_eq!(m.count, 1);
        assert!(is_a_drop(&m), "dst == 0 is the drop");
        assert!(!m.is_unequip(), "and it must not be read as an unequip");
        assert_eq!(drop_count(&m), 1);

        // The 2026-08-19 unequip, from crates/net/src/inventory.rs's own test.
        let unequip = [0x06, 0xad, 0x3c, 0x07, 0x01, 0xfb, 0xff, 0x01, 0x00, 0xff, 0xff];
        let u = parse_inventory_move(&unequip).expect("11 bytes");
        assert!(!is_a_drop(&u), "dst 1 is a move");
        assert!(u.is_unequip());
    }

    /// `count = -1` is what an unequip carries; a drop must not turn it into "drop -1".
    #[test]
    fn a_negative_count_drops_one() {
        for c in [-1i16, 0, -32768] {
            let m = crate::inventory::InventoryMove { tick: 0, inv_type: 1, src: 1, dst: 0, count: c };
            assert_eq!(drop_count(&m), 1);
        }
        let m = crate::inventory::InventoryMove { tick: 0, inv_type: 2, src: 3, dst: 0, count: 7 };
        assert_eq!(drop_count(&m), 7);
    }

    fn sample() -> FieldDrop {
        FieldDrop {
            object_id: 0x0000_2711,
            item_id: 1_302_000,
            meso: 0,
            owner_id: 200,
            source_object_id: 200,
            x: 473,
            y: 395,
            source_x: 470,
            source_y: 300,
            delay: 0,
            is_money: false,
            own_type: OWN_TYPE_USER,
        }
    }

    /// Every field offset this module claims, checked against the built bytes.
    #[test]
    fn the_enter_body_is_the_layout_the_handler_reads() {
        let d = sample();
        let b = drop_enter_field(&d, ENTER_FLOATING);
        assert_eq!(b.len(), DROP_ENTER_FIELD_LEN);

        assert_eq!(b[0], DROP_TYPE_ITEM, "dropType, 0x1417a2f2b");
        assert_eq!(b[1], ENTER_FLOATING, "enterType, 0x1417a2fde");
        assert_eq!(u32::from_le_bytes([b[2], b[3], b[4], b[5]]), d.object_id, "0x1417a2ff1");
        assert_eq!(b[6], 0, "isMoney, 0x1417a3086");
        assert_eq!(u32::from_le_bytes([b[7], b[8], b[9], b[10]]), 0, "motionType, drop+0x194");
        // 11..15 random, 15..19 the float source
        assert_eq!(
            u32::from_le_bytes([b[19], b[20], b[21], b[22]]),
            d.item_id,
            "itemId, 0x1417a3310"
        );
        assert_eq!(
            u32::from_le_bytes([b[23], b[24], b[25], b[26]]),
            d.owner_id,
            "ownerId, 0x1417a3513"
        );
        assert_eq!(b[27], OWN_TYPE_USER, "ownType, 0x1417a3539");
        assert_eq!(i16::from_le_bytes([b[28], b[29]]), d.x, "x, 0x1417a3560");
        assert_eq!(i16::from_le_bytes([b[30], b[31]]), d.y, "y, 0x1417a356f");
        // 32..40 the four i16 the client stores at drop+0x218..0x224
        assert_eq!(
            u32::from_le_bytes([b[40], b[41], b[42], b[43]]),
            d.source_object_id,
            "sourceObjectId, 0x1417a35db"
        );
        // 44..48 drop+0x1e0, 48..56 drop+0x1e8, 56..60 drop+0x1f0, 60 drop+0x210,
        // 61..81 the FUN_1403747c0 block, 81 moneyType, 82/83 two bools.
        assert_eq!(b[81], 0, "moneyType, 0x1417a36d2");

        // The source block, only present for enter types 0/1/3.
        assert_eq!(i16::from_le_bytes([b[84], b[85]]), d.source_x, "srcX, 0x1417a41d2");
        assert_eq!(i16::from_le_bytes([b[86], b[87]]), d.source_y, "srcY, 0x1417a43af");
        assert_eq!(u32::from_le_bytes([b[88], b[89], b[90], b[91]]), d.delay, "0x1417a4646");
        assert_eq!(b[92], 0, "isExplosiveDrop, 0x1417a4658");
        assert_eq!(b[93], 0, "isSpecialDrop, 0x1417a4688");

        // The long tail: an eight-byte expiry of zero, then the seven fields, then two u32.
        assert_eq!(&b[94..102], &[0u8; 8], "the expiry MUST be zero - see the doc comment");
        assert_eq!(b[102], 1, "canBePickedUpByPet");
        assert_eq!(b[103], 0, "the byte that fires 0xc0041f15 when non-zero");
        assert_eq!(i16::from_le_bytes([b[104], b[105]]), 0, "fallingVY");
        assert_eq!(b[106], 0, "fadeInEffect");
        assert_eq!(u32::from_le_bytes([b[107], b[108], b[109], b[110]]), 0, "collisionPickUp");
        assert_eq!(b[111], 0, "itemGrade");
        assert_eq!(b[112], 0, "prepareCollisionPickUp");
        assert_eq!(&b[113..121], &[0u8; 8], "the two trailing u32");
    }

    /// The enter type is the only thing that changes the fixed part's length.
    #[test]
    fn instant_omits_the_source_block_and_nothing_else() {
        let d = sample();
        let floating = drop_enter_field(&d, ENTER_FLOATING);
        let instant = drop_enter_field(&d, ENTER_INSTANT);
        assert_eq!(floating.len(), DROP_ENTER_FIELD_LEN);
        assert_eq!(instant.len(), DROP_ENTER_FIELD_LEN_INSTANT);
        assert_eq!(instant.len() + 8, floating.len(), "srcX, srcY and delay");
        assert_eq!(&floating[2..84], &instant[2..84], "everything before the block matches");
        assert_eq!(
            &floating[92..],
            &instant[84..],
            "and everything after it matches too, so only the block moved"
        );
    }

    /// The client's own test, both halves of it, for all four defined enter types.
    #[test]
    fn only_types_0_1_and_3_read_the_source_block() {
        assert!(enter_type_reads_the_source_block(enter_type::DEFAULT));
        assert!(enter_type_reads_the_source_block(enter_type::FLOATING));
        assert!(!enter_type_reads_the_source_block(enter_type::INSTANT));
        assert!(enter_type_reads_the_source_block(enter_type::FADE_AWAY));
        // `test eax,0xfffffffc` rejects everything from 4 up, including the reference's 4.
        for t in 4u8..=255 {
            assert!(!enter_type_reads_the_source_block(t), "enter type {t}");
        }
    }

    /// Money skips the expiry, which is the client's `if (!isMoney)` at `0x1417a9b7b`.
    #[test]
    fn money_is_eight_bytes_shorter() {
        let mut d = sample();
        d.is_money = true;
        d.meso = 1234;
        let b = drop_enter_field(&d, ENTER_FLOATING);
        assert_eq!(b.len(), DROP_ENTER_FIELD_LEN - 8);
        assert_eq!(b[0], DROP_TYPE_MESO);
        assert_eq!(b[6], 1, "isMoney");
        assert_eq!(
            u32::from_le_bytes([b[19], b[20], b[21], b[22]]),
            1234,
            "money goes in the item-id field"
        );
    }

    /// The pick-up gate is a property of the enter type and it is easy to get wrong.
    #[test]
    fn the_two_enter_types_we_send_are_the_pickable_ones() {
        // drop+0x61 = ((enterType - 1) <= 1), i.e. exactly 1 and 2.
        for t in 0u8..=255 {
            let pickable = t == 1 || t == 2;
            assert_eq!(
                pickable,
                t == ENTER_FLOATING || t == ENTER_INSTANT,
                "enter type {t}: the constants this module offers must be the pickable pair"
            );
        }
    }

    #[test]
    fn the_leave_body_puts_the_object_id_first() {
        let b = drop_picked_up_by_character(0x2711, 200);
        assert_eq!(b.len(), DROP_PICKED_UP_LEN);
        assert_eq!(
            u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            0x2711,
            "the object id is read FIRST - 0x1417ad7cd - not after the type"
        );
        assert_eq!(b[4], leave_type::CHAR_PICKUP, "0x1417ad7d7");
        assert_eq!(u32::from_le_bytes([b[5], b[6], b[7], b[8]]), 200, "drop+0xec");

        let plain = drop_leave_field(0x2711, leave_type::FADE);
        assert_eq!(plain.len(), DROP_LEAVE_FIELD_LEN);
        assert_eq!(u32::from_le_bytes([plain[0], plain[1], plain[2], plain[3]]), 0x2711);
        assert_eq!(plain[4], leave_type::FADE);
    }

    #[test]
    fn the_three_leave_types_that_carry_a_character_are_2_3_and_5() {
        for t in 0u8..=255 {
            assert_eq!(
                leave_type_carries_a_character(t),
                matches!(t, 2 | 3 | 5),
                "leave type {t}"
            );
        }
    }

    #[test]
    fn the_mob_pickup_parses_and_a_short_body_is_none() {
        let m = parse_mob_drop_pick_up(&[0xd0, 0x07, 0, 0, 0x11, 0x27, 0, 0]).unwrap();
        assert_eq!(m.mob_object_id, 2000, "mob+0x3a0, NOT a character id");
        assert_eq!(m.drop_object_id, 0x2711);
        assert!(parse_mob_drop_pick_up(&[0u8; 7]).is_none());
    }

    /// The two opcodes are adjacent because the pool is exactly two opcodes wide.
    #[test]
    fn the_pool_is_two_opcodes_wide() {
        assert_eq!(DROP_LEAVE_FIELD, DROP_ENTER_FIELD + 1);
        assert_eq!(DROP_ENTER_FIELD, 0x046E);
    }
}
