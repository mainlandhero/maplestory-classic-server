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

/// **Pet ids, which this server cannot yet put in a bag without killing the client.**
///
/// `5000000..=5009999`, read out of `FUN_1401B1040` - the client's own id-to-class function,
/// which returns **3** for this range. `research/cash-item-throw.md`, 2026-08-26.
///
/// # Why a range check is load-bearing
///
/// The client has **two** dispatches on an item's class and they consult different things.
/// The factory `FUN_1403095E0` believes the **type byte on the wire**; the tooltip
/// `FUN_142694130` re-derives the class from the **item id**. Send a pet as a bundle and they
/// disagree: the factory allocates a 126-byte bundle object, and the pet tooltip then reads
/// its integrity checksum at `item+0x7e` - **four bytes past the end of that allocation** -
/// fails the `0xBAADF00D / ror 5 / add` verify, and throws `ZException`. The process exits
/// with `0xE06D7363`.
///
/// That is measured, not reasoned: the owner's `!locker 1` moved a Brown Puppy (`5000001`) into
/// the Cash tab on 2026-08-26 and the client died 3.4 s later. The control is clean - a grep
/// of **every** archived log for `ADD: item 5xxxxxx` returns exactly one hit, that run. The
/// first time a pet id ever reached this client's bag is the run that died.
///
/// The real fix is a **67-byte type-3 body** (base 18 + a 48-byte pet block, with the pet's
/// `char[13]` name at `+0x4d`); ours is 41 and every field past the base is misplaced.
/// `store::ItemKind` has only `Equip` and `Bundle`, so that is a change with a shape, not a
/// one-liner. Until it exists, callers refuse rather than send.
pub fn is_pet(item_id: u32) -> bool {
    (5_000_000..=5_009_999).contains(&item_id)
}

/// Entry mode 0: **put a new item in a slot**, carrying the whole item body.
///
/// `research/msexe-setfield.md`'s mode table: mode 0's tail is the **item blob**, read at
/// `142d51bd9` through `FUN_140303530` - the same factory the character record uses, so
/// [`crate::opcode::equipped_item`] and [`crate::bag::bundle_item`] both produce a legal
/// body for it. The blob's own leading `u8` selects the decode: 1 equip, 2 bundle, 3 pet,
/// **anything else leaves the item null and reads nothing further**, which silently
/// truncates the entry rather than throwing.
pub const MODE_ADD: u8 = 0;

/// Entry mode 3: **take the item out of a slot**. No tail bytes at all.
///
/// Named here because it is the other half of a drop and of a sale, and because a mode with
/// no tail is exactly the kind of thing someone adds a phantom field to.
pub const MODE_REMOVE: u8 = 3;

/// Put an item into a bag slot, without a field re-entry.
///
/// This is what makes an item appear in the bag *now* - the alternative was to write the
/// database and re-send `SetField`, which works but redraws the whole field to move one
/// item.
///
/// `pos` is the **1-based** slot. `blob` is a complete item body **including its leading
/// type byte**: `crate::opcode::equipped_item(id, &stats)` for an equip,
/// `crate::bag::bundle_item(..)` for a stack.
///
/// # No trailing byte, and that is read rather than assumed
///
/// The conditional `addMovementInfo` byte is guarded by `avatarChanged`, and
/// `research/msexe-setfield.md` establishes that `avatarChanged` is set in **exactly two
/// places in the whole handler** - mode 2 with an equipped side, and mode 3 with an equipped
/// side where the client already holds the item. Mode 0 sets it nowhere, so this body ends
/// with the blob. Appending a byte here would leave one unread; the frame carries its own
/// length so that is survivable, but this packet is one where being a byte out has cost two
/// sessions and there is no reason to be sloppy in the direction of "probably harmless".
pub fn inventory_added(inv_type: i8, pos: i16, blob: &[u8]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(1); // bExclRequestSent - clears the +0x2330 latch, same as every other reply here
    w.u8(0);
    w.u32(1); // nCount, i32
    w.u8(0); // notRemoveAddInfo
    w.u8(MODE_ADD);
    w.u8(inv_type as u8);
    w.i16(pos);
    let mut out = w.into_vec();
    out.extend_from_slice(blob);
    out
}

/// Entry mode 5: **put a new item in a slot, without waking the quest system.**
///
/// Same wire shape as [`MODE_ADD`] - `u8 mode; u8 invType; i16 pos;` then the item blob,
/// read by the same `FUN_140303530` - and the same store, `FUN_1402e4c20(charData, invType,
/// pos, &item)`. What it does **not** do is the two calls mode 0 makes right after that
/// store.
///
/// # Read from the jump table, not from a name
///
/// `142d51bb6 cmp ebx,0xc / 142d51bc6 mov ecx,[rdx + rbx*4 + 0x2d546bc]` is the mode switch.
/// The table at `0x142d546bc`, dumped out of the image (`tools/dump_va.py`), is 13 RVAs:
///
/// ```text
/// [ 0] 0x02d51bd2      [ 5] 0x02d531e4   <- this mode
/// [ 3] 0x02d52f64      [12] 0x02d53bef
/// ```
///
/// So mode 5's body is `0x142d531e4 .. 0x142d532c7`:
///
/// ```asm
/// 142d531eb  call 0x140303530        ; the item blob - the SAME reader mode 0 uses
/// 142d5320d  call 0x140255650        ; is `pos` a sub-bag slot (10101..)?  no, for 1..96
/// 142d532b7  call 0x1402e4c20        ; charData, invType, pos, &item  - mode 0's store
/// 142d532c7  jmp  0x142d52168        ; back to the entry loop. That is the whole case.
/// ```
///
/// # Why it exists here: the field-entry bag restore pops a quest tooltip
///
/// `CWvsContext::OnInventoryOperation` calls **`FUN_142d9b200(this, itemId, countBefore)`**
/// after modes 0, 1, 3, 8, 9 and 11 - six call sites, `142d51f67 142d52506 142d53136
/// 142d535bb 142d537fc 142d53bbb`, **none of them in mode 5's range**. That function looks
/// the item up in the quest manager's item->quest map (`questMan+0x238`), intersects it with
/// the character's *started* quest map (`charData+0x1273`), and for a matching item
/// requirement draws the collection hint - but only if the count actually moved:
///
/// ```asm
/// 142d9b6fa  cmp   r15d, r14d      ; count BEFORE this packet  vs  count now
/// 142d9b6fd  je    142d9b922       ; equal -> the whole hint block is skipped
/// 142d9b71c  cmp   r15d, [rsi]     ; before vs the quest's required count
/// 142d9b71f  jge   142d9b922
/// 142d9b7f1  call  0x142d93610     ; "n / N <item>"   (quest is in the tracker)
/// 142d9b916  call  0x142d934f0     ; the same, untracked
/// ```
///
/// A `SetField` leaves the Use / Set Up / Etc / Cash bags empty - which is the entire reason
/// `session::field::restore_bag_and_mesos` exists - so every restored stack is a 0 -> n
/// change and every quest item in the bag fires that hint on **every map change**. The owner,
/// 2026-08-30: *"whenever I change the map, I see the popup for my collection quest as a
/// tooltip every time."*
///
/// Mode 5 puts the item in the slot and says nothing.
///
/// # It has never been on a wire
///
/// **[L]** for the jump-table entry, the reads and the store; **[I]** for everything about
/// how it looks on screen. Mode 0 additionally calls `FUN_142ce53e0` (a second quest hook)
/// and records the slot in the before/after map the handler diffs for the quick slot; mode 5
/// does neither. The unconditional `FUN_142cbefd0(this,1,0,0)` after the entry loop runs for
/// every `0x0070` whatever the mode, so the generic "inventory changed" refresh is not lost.
///
/// **The failure to watch for is the bag looking empty**, which is far worse than the
/// tooltip. Test it on one item before using it for the whole restore.
pub const MODE_SET_QUIET: u8 = 5;

/// Put an item into a bag slot **without** the quest-progress hint - `0x0070` mode 5.
///
/// Byte-for-byte [`inventory_added`] with a `5` where the mode byte is. See
/// [`MODE_SET_QUIET`] for the listing, and for the reason not to trust it until a client has
/// drawn it.
pub fn inventory_set_quiet(inv_type: i8, pos: i16, blob: &[u8]) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(1); // bExclRequestSent - clears the +0x2330 latch, same as every other reply here
    w.u8(0);
    w.u32(1); // nCount, i32
    w.u8(0); // notRemoveAddInfo
    w.u8(MODE_SET_QUIET);
    w.u8(inv_type as u8);
    w.i16(pos);
    let mut out = w.into_vec();
    out.extend_from_slice(blob);
    out
}

/// Take an item out of a **bag** slot. Mode 3, and it carries no tail at all.
///
/// # `pos` must be positive, and that is a safety property rather than a convention
///
/// Mode 3 is the one mode whose `avatarChanged` depends on **client-side state**: it is set
/// when `(invType == 1 || invType == 6) && oldPos < 0` *and the client already holds an item
/// at that slot*. The server cannot see the second half, so a negative `pos` would make the
/// body's length depend on something we cannot know - the client would read a trailing byte
/// we did not send, or not read one we did. `research/msexe-setfield.md` says outright: do
/// not drive that flag from mode 3.
///
/// A positive `pos` is a bag slot, the first half of the condition is false whatever the
/// inventory type, and the entry is exactly six bytes. Unequipping goes through
/// [`inventory_move_result`] instead, which drives the flag from mode 2 where it depends on
/// the wire values alone.
pub fn inventory_removed(inv_type: i8, pos: i16) -> Vec<u8> {
    debug_assert!(pos > 0, "mode 3 on an equipped slot makes the body length client-dependent");
    let mut w = PacketWriter::new();
    w.u8(1); // bExclRequestSent
    w.u8(0);
    w.u32(1); // nCount, i32
    w.u8(0); // notRemoveAddInfo
    w.u8(MODE_REMOVE);
    w.u8(inv_type as u8);
    w.i16(pos);
    w.into_vec()
}

/// Length of an [`inventory_removed`] body: header 7, one entry 4, no tail.
pub const INVENTORY_REMOVE_LEN: usize = 7 + 4;

/// Entry mode 1: **the stack in this slot is now `quantity`**. Tail is one `i16`.
///
/// `research/msexe-setfield.md`'s mode table, read at `142d521fe`. **[L]**
pub const MODE_QUANTITY: u8 = 1;

/// Say that a slot's stack changed size, without emptying it.
///
/// # Why this is not [`inventory_removed`]
///
/// Mode 3 removes the **whole slot**. Using one potion out of a stack of two and sending
/// mode 3 would empty the slot on screen while the server still held one - the two ends then
/// disagree about a slot, which is the failure this module's header is entirely about.
///
/// `take_quest_item` in `session/npc.rs` sends mode 3 for a partial take too. It is correct
/// there only by accident: quest 1001 takes one item out of a stack of one, so "the rest of
/// the stack" is empty either way. It is worth fixing the day a quest takes 2 of 5.
///
/// **No trailing byte.** `avatarChanged` is set only by modes 2 and 3 on `invType` 1 or 6
/// with a negative position, so mode 1 never earns one.
pub fn inventory_quantity(inv_type: i8, pos: i16, quantity: u16) -> Vec<u8> {
    debug_assert!(pos > 0, "a stack lives in a bag slot, and bag slots are 1-based");
    let mut w = PacketWriter::new();
    w.u8(1); // bExclRequestSent - clears the +0x2330 latch
    w.u8(0);
    w.u32(1); // nCount, i32
    w.u8(0); // notRemoveAddInfo
    w.u8(MODE_QUANTITY);
    w.u8(inv_type as u8);
    w.i16(pos);
    w.i16(quantity as i16);
    w.into_vec()
}

/// Length of an [`inventory_quantity`] body: header 7, one entry 4, an `i16` tail.
pub const INVENTORY_QUANTITY_LEN: usize = 7 + 4 + 2;

/// The fixed cost of an [`inventory_added`] body, before the item blob.
pub const INVENTORY_ADD_HEAD_LEN: usize = 7 + 4;

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

// -------------------------------------------------------------------------------------
// `0x007B` InventoryGrow - a tab gets more slots, without a relog
// -------------------------------------------------------------------------------------

/// Server -> client: **this bag tab now holds this many slots.**
///
/// The owner, 2026-09-09, on the 5-slot coupons. The first version of that feature wrote the new
/// count to the database and told the player *"change maps or relog to see them"*, because
/// nothing here could change the count live - the client draws it from the character record
/// it was handed at field entry.
///
/// # It was found while chasing the station clock, and it is decoded rather than named
///
/// `research/msexe-gamestage-opcodes.md`'s candidate column calls `0x007B` *InventoryGrow*,
/// and a candidate name from the v214 tree is worth nothing on its own - `CLAUDE.md` scores
/// that source at **1 of 8**. The handler settles it. `FUN_142d54700`, 107 bytes, entire:
///
/// ```asm
/// 142d5471d  call 1406e8ae0        ; u8  -> ebx
/// 142d54728  call 1406e8ae0        ; u8  -> edx
/// 142d54735  add  rsi, 0x5d0       ; charData + 0x5d0
/// 142d5473c  inc  edx              ; slots + 1
/// 142d54741  lea  rcx, [rsi+rbx*8] ; + invType*8   -> that tab's list
/// 142d54745  call 14030ee00        ; resize it
/// ```
///
/// **`0x5d0` and the `+ index*8` are the same base this project established from a different
/// packet.** `equipped_block_with_bag`'s own doc block, written for the character record:
/// *"`[R14 + 0x5d0]` with `R15 = 1` is `charData + 0x5d8`, index 1 of the six the size loop
/// walks - the Equip tab."* Two independent reads landing on one offset is what makes this
/// **[L]** rather than a name someone liked the look of.
///
/// The `inc edx` is the **1-based slot hole** this module's header is about: the client
/// allocates `slots + 1` entries so that slot 0 can be the hole. So the field carries the
/// logical slot count - the same number `inventory_size_block` sends - and **must not** be
/// pre-incremented here, or every coupon would add six.
pub const INVENTORY_GROW: u16 = 0x007B;

/// Body length: two bytes, and there is no tail.
pub const INVENTORY_GROW_LEN: usize = 2;

/// Build an [`INVENTORY_GROW`] body.
///
/// `inv_type` is the wire tab number - the same 1..=5 `InventoryType::as_u8` gives, which is
/// the index the handler multiplies by 8. `slots` is the tab's new logical size.
pub fn inventory_grow(inv_type: u8, slots: u8) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(inv_type); // 142d5471d -> ebx, the tab index
    w.u8(slots); //    142d54728 -> edx, and the client adds the 1-based hole itself
    w.into_vec()
}


#[cfg(test)]
mod pet_tests {
    use super::*;

    /// The window is the client's own, and its EDGES are what matter - one digit outside it
    /// and the client's tooltip takes the bundle path, which is the shape we do send.    /// **`0x007B` InventoryGrow, against the handler rather than against a candidate name.**
    ///
    /// `research/msexe-gamestage-opcodes.md` calls this one *InventoryGrow* from the v214
    /// tree, and this repo scores that source at 1 of 8 - so the name is not the evidence.
    /// `FUN_142d54700` is: two `u8` reads, then
    /// `resize(charData + 0x5d0 + invType*8, slots + 1)`. Both fields are pinned here.
    #[test]
    fn inventory_grow_is_two_bytes_the_tab_then_its_logical_size() {
        let b = inventory_grow(2, 40);
        assert_eq!(b.len(), INVENTORY_GROW_LEN, "two bytes, no tail");
        assert_eq!(b[0], 2, "the tab index the handler multiplies by 8");
        assert_eq!(b[1], 40, "the LOGICAL count - the client adds the 1-based hole itself");

        // **Not pre-incremented.** The handler does `inc edx` before resizing, so adding one
        // here would make every 5-slot coupon add six.
        for slots in [24u8, 100, 150] {
            assert_eq!(inventory_grow(1, slots)[1], slots, "{slots} must go out unchanged");
        }
        // Every real tab index round-trips, including the Cash tab the coupons live in.
        for inv in [1u8, 2, 3, 4, 5] {
            assert_eq!(inventory_grow(inv, 30)[0], inv);
        }
    }


    #[test]
    fn the_pet_window_is_the_clients_own() {
        assert!(is_pet(5_000_000), "the first");
        assert!(is_pet(5_000_001), "Brown Puppy - the one that killed the client");
        assert!(is_pet(5_009_999), "the last");
        assert!(!is_pet(4_999_999));
        assert!(!is_pet(5_010_000), "one past the end");
        assert!(!is_pet(5_070_000), "a Megaphone is a plain cash bundle");
        assert!(!is_pet(0));
    }
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

    /// Mode 3 is four bytes of entry and nothing else.
    #[test]
    fn a_remove_has_no_tail_at_all() {
        let b = inventory_removed(2, 5);
        assert_eq!(b.len(), INVENTORY_REMOVE_LEN);
        assert_eq!(b[0], 1, "bExclRequestSent");
        assert_eq!(b[7], MODE_REMOVE);
        assert_eq!(b[8] as i8, 2);
        assert_eq!(i16::from_le_bytes([b[9], b[10]]), 5);
    }

    /// Mode 0 carries the item body and nothing after it.
    #[test]
    fn an_add_is_the_head_then_the_blob() {
        let blob = [1u8, 2, 3, 4, 5];
        let b = inventory_added(INV_EQUIP, 7, &blob);
        assert_eq!(b.len(), INVENTORY_ADD_HEAD_LEN + blob.len());
        assert_eq!(b[0], 1, "bExclRequestSent");
        assert_eq!(u32::from_le_bytes([b[2], b[3], b[4], b[5]]), 1, "nCount");
        assert_eq!(b[7], MODE_ADD);
        assert_eq!(b[8] as i8, INV_EQUIP);
        assert_eq!(i16::from_le_bytes([b[9], b[10]]), 7, "the 1-based slot");
        assert_eq!(&b[INVENTORY_ADD_HEAD_LEN..], &blob, "the blob, ending the body");
    }

    /// Mode 5 is mode 0 with one byte changed, and that byte is the mode.
    ///
    /// This is the test that matters: the *only* thing this builder may do differently is
    /// the mode byte. If it ever grows a field of its own, the client reads mode 5's body
    /// with mode 0's reader - one blob and nothing else - and the frame goes out of step.
    #[test]
    fn the_quiet_set_differs_from_an_add_in_exactly_the_mode_byte() {
        let blob = [2u8, 5, 9, 61, 0, 0, 0, 1];
        for (inv, pos) in [(INV_EQUIP, 1i16), (2, 7), (4, 29), (5, 96)] {
            let loud = inventory_added(inv, pos, &blob);
            let quiet = inventory_set_quiet(inv, pos, &blob);
            assert_eq!(loud.len(), quiet.len(), "same length or the reader desynchronises");
            let differing: Vec<usize> =
                (0..loud.len()).filter(|&i| loud[i] != quiet[i]).collect();
            assert_eq!(
                differing,
                vec![7],
                "index 7 is the mode byte and nothing else may move"
            );
            assert_eq!(loud[7], MODE_ADD);
            assert_eq!(quiet[7], MODE_SET_QUIET);
        }
    }

    /// The whole point of the packet: a `5` there, because a `0` is what pops the tooltip.
    ///
    /// The constant is checked against the literal rather than against itself - the jump
    /// table at `0x142d546bc` has thirteen entries and only index 5 is `0x02d531e4`.
    #[test]
    fn the_quiet_mode_is_five_and_the_head_is_unchanged() {
        let b = inventory_set_quiet(4, 29, &[2u8, 5, 9, 61]);
        assert_eq!(MODE_SET_QUIET, 5, "jump table index, dumped from the image");
        assert_eq!(b.len(), INVENTORY_ADD_HEAD_LEN + 4);
        assert_eq!(b[0], 1, "bExclRequestSent - a 0 here leaves the UI latched");
        assert_eq!(b[1], 0);
        assert_eq!(u32::from_le_bytes([b[2], b[3], b[4], b[5]]), 1, "nCount is i32");
        assert_eq!(b[6], 0, "notRemoveAddInfo");
        assert_eq!(b[7], 5);
        assert_eq!(b[8] as i8, 4, "the Etc tab");
        assert_eq!(i16::from_le_bytes([b[9], b[10]]), 29, "the 1-based slot");
        assert_eq!(&b[INVENTORY_ADD_HEAD_LEN..], &[2u8, 5, 9, 61], "the blob ends the body");
    }

    /// Mode 5 sets `avatarChanged` nowhere either - its body is four calls and a `jmp`.
    #[test]
    fn a_quiet_set_never_earns_the_trailing_byte() {
        for inv in [INV_EQUIP, INV_DECO, 2, 4] {
            let b = inventory_set_quiet(inv, 1, &[1u8]);
            assert_eq!(
                b.len(),
                INVENTORY_ADD_HEAD_LEN + 1,
                "0x142d531e4..0x142d532c7 reads the blob and nothing after it"
            );
        }
    }

    /// An Add never sets avatarChanged, so it never earns the trailing byte - whatever the
    /// inventory type and whatever the slot.
    #[test]
    fn an_add_never_earns_the_trailing_byte() {
        for inv in [INV_EQUIP, INV_DECO, 2, 4] {
            let b = inventory_added(inv, 1, &[1u8]);
            assert_eq!(
                b.len(),
                INVENTORY_ADD_HEAD_LEN + 1,
                "mode 0 sets avatarChanged nowhere - research/msexe-setfield.md"
            );
        }
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
