//! The bag lists in the character record: the four `u16`-terminated lists that
//! `presence[2]` opens after the equipped-item loop.
//!
//! Full working, with every address and every `[L]`/`[I]` label:
//! **`research/bag-lists.md`**. Two things in that document contradict what this module was
//! commissioned to build, and both are load-bearing:
//!
//! 1. **The four lists are not four bags.** One of them is the **Equip tab's** bag
//!    (inventory type 1, `charData + 0x5d8`). The other three are equipped-*like* containers
//!    whose positions live in `3000..=3231`, and nothing a character can hold addresses
//!    them. See [`B9E0_RANGES`].
//! 2. **The Use / Set Up / Etc / Cash bags exist in the record too**, read by four more
//!    calls to the same helper at `0x140306845`, `0x140306855`, `0x140306865` and
//!    `0x140306875` - each re-gated on **its own** presence byte (3, 4, 5, 6). They read
//!    nothing while those bytes are clear, which is why today's ten zero bytes are correct.
//!    Switching them on is not free; [`BAG_PRESENCE_BYTE`] and `research/bag-lists.md` §6
//!    price it.
//!
//! So the only list this module can fill today is the **Equip tab**, and an equip in a bag
//! is still a **type 1** item - the same 125-byte body [`crate::opcode::equipped_item`]
//! already builds and that was confirmed on screen on 2026-08-19.
//!
//! [`bundle_item`] is here for the type-2 (stackable) body, decoded in the same pass, so
//! that goals F (shops) and G (storage) do not have to re-derive it. **It is still not
//! reachable from the character record** - the Use / Set Up / Etc / Cash bags sit behind
//! presence bytes 3, 4, 5 and 6, and bytes 3/4/5 each open a further undecoded block
//! (section 6 of `research/bag-lists.md` prices it). Since 2026-08-20 it *is* reachable
//! from `0x0070` mode 0, which is how `!item` and a shop purchase put a stack in the bag
//! without a field re-entry.

use crate::opcode::{equipped_item, EquipStats, ITEM_NEVER_EXPIRES};

// =========================================================================================
// Where each bag lives, and what switching it on costs
// =========================================================================================

/// The six inventory types, in the order `FUN_1403023d0` switches on them.
///
/// Index is `type - 1`. Names are `research/inventory-slots.md`'s: Equip is **[D]** (its
/// presence byte is the measured one), the other five came off the owner's screenshot of a
/// six-tab inventory window and are **[I]**. Nothing on the wire depends on a name.
pub const BAG_TAB_NAMES: [&str; 6] = ["Equip", "Use", "Set Up", "Etc", "Cash", "Deco"];

/// The presence byte that opens inventory type `n`'s bag list, indexed by `n - 1`. **[L]**
///
/// Read out of each gate key's own CRT initialiser - the `mov byte ptr [key + b], 1` that
/// builds the key's 100-byte mask - for the six keys `FUN_1403023d0`'s jump table at
/// `0x1403024a4` hands out:
///
/// ```text
/// type 1 Equip   key 0x143abdb20   init 0x140022952   byte  2
/// type 2 Use     key 0x143abdab0   init 0x140022932   byte  3
/// type 3 Set Up  key 0x143abda40   init 0x1400229b2   byte  4
/// type 4 Etc     key 0x143abd9d0   init 0x140022992   byte  5
/// type 5 Cash    key 0x143abd960   init 0x140022912   byte  6
/// type 6 Deco    key 0x143abd8f0   init 0x140022972   byte 44
/// ```
///
/// The same scan reports byte 2 for `0x143abedb0` and byte 44 for `0x143abeb80`, which is
/// exactly what `research/naked-character.md` and `research/charrecord-presence-map.md`
/// found by other means - two positive controls, both passed. The column also reproduces
/// `research/inventory-slots.md`'s six-turn size-loop permutation from a different table.
///
/// **Only index 0 (byte 2) is set today**, and only it is free: `research/bag-lists.md` §6
/// measures what bytes 3, 4, 5 and 44 also drag in, and three of them open blocks that have
/// never been decoded.
pub const BAG_PRESENCE_BYTE: [usize; 6] = [2, 3, 4, 5, 6, 44];

/// Inventory type 1 - the Equip tab, the one bag `presence[2]` already opens.
pub const BAG_EQUIP: u8 = 1;

/// The three position ranges `FUN_14030b9e0` accepts when called with `param_2 = 1`, as
/// `(lower, upper)` half-open pairs. **[L]**
///
/// Read straight out of the image at `0x14327dd50` (lower bounds) and `0x14327dd68` (upper),
/// the two tables `0x14030bc6d` and `0x14030bc7a` index by the outer loop counter. The
/// helper's outer loop runs indices 0..4 (`CMP EAX,0x5 / JL` at `0x14030be5c`) and proceeds
/// only where `FUN_140255790(i) == 0`; that function is nine bytes of
/// `TEST ECX,ECX / JZ / CMP ECX,1 / JZ / XOR EAX,EAX / RET`, so it returns 1 for 0 and 1 and
/// 0 otherwise - hence indices **2, 3 and 4**.
///
/// **These are not inventory tabs.** They are three of five equipped-like containers at
/// `charData + 0x5a8 + i*8`, sitting between the second equipped array and the six
/// inventory pointers. What they hold is **not established**. Their positions start at 3000,
/// which no item this server can create ever reaches, so all three are sent empty and the
/// six bytes are pure framing.
pub const B9E0_RANGES: [(u16, u16); 3] = [(3000, 3032), (3100, 3132), (3200, 3232)];

/// How many bytes the three `FUN_14030b9e0` lists cost when empty: one `u16` each.
pub const B9E0_EMPTY_LEN: usize = 6;

/// The ten bytes `equipped_block_with` writes today, and the regression anchor for this
/// module: `u16 0` ends the equipped list, `u16 0` ends `FUN_14030b6f0`'s Equip-tab bag,
/// and three `u16 0` end `FUN_14030b9e0`'s three lists.
///
/// This exact sequence is on the wire in the run that put a dressed character on map 1, so
/// it is the one byte pattern here that a real client has accepted.
pub const EMPTY_EQUIPPED_TAIL: [u8; 10] = [0; 10];

// =========================================================================================
// The Equip tab's bag
// =========================================================================================

/// One equipment item sitting **in the Equip tab of the bag** rather than worn.
///
/// `pos` is the bag slot, 1-based. `FUN_14030b6f0` stores the item at `inventory[pos]` only
/// while `1 <= pos <= slots`, where `slots` is the count sent for this inventory in the
/// `presence[7]` block (the client resizes the array to `slots + 1` and index 0 is the hole
/// that makes MapleStory's slots 1-based). A position outside that range is **decoded and
/// thrown away** - the bytes are consumed and the item vanishes. **[L]**, `0x14030b7c9` /
/// `0x14030b7d6`.
///
/// `stats` is the same [`EquipStats`] the equipped list uses, because a bag equip and a worn
/// equip are the same type-1 body.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BagEquip {
    /// The bag slot, 1-based.
    pub pos: u16,
    pub item_id: u32,
    pub stats: EquipStats,
}

impl BagEquip {
    /// A bag equip with all four bitmasks zero - the 125-byte body confirmed on screen.
    pub fn plain(pos: u16, item_id: u32) -> Self {
        Self { pos, item_id, stats: EquipStats::default() }
    }
}

/// Whether the client will actually keep an item at this bag position. **[L]**
///
/// `CMP ECX,0x1 / JL` then `CMP ECX,[slotMax] / JG` at `0x14030b7c9` and `0x14030b7d6`.
/// The position itself is read as an **unsigned** `u16` (`MOVZX` at `0x14030b792` and
/// `0x14030b99f`), and `0` is the list terminator, so `0` can never address a slot.
pub fn bag_pos_is_kept(pos: u16, slots: u16) -> bool {
    pos >= 1 && pos <= slots
}

/// `FUN_14030b6f0`'s list for the Equip tab: `u16 pos` + a type-1 item body per entry, then
/// `u16 0`.
///
/// Entries whose position the client would discard are dropped rather than sent, the same
/// way [`crate::opcode::equipped_block`] drops slots outside `EQUIP_SLOTS`. Sending them
/// would cost 127 bytes each and lose the item anyway.
///
/// **This list is read only when `flagA` is 0.** `TEST SIL,SIL / JNZ` at `0x1403062dd` skips
/// the whole `FUN_14030b6f0` call for a non-zero leading byte, so a non-zero `flagA` means
/// the Equip tab's bag cannot be sent at all. `equipped_block_with` sends 0 and must keep
/// doing so.
pub fn equip_bag_list(items: &[BagEquip], slots: u16) -> Vec<u8> {
    let mut b = Vec::new();
    for item in items {
        if !bag_pos_is_kept(item.pos, slots) {
            continue;
        }
        b.extend_from_slice(&item.pos.to_le_bytes()); // 14030b78d / 14030b99a  u16 pos
        b.extend_from_slice(&equipped_item(item.item_id, &item.stats)); // 14030b7bd
    }
    b.extend_from_slice(&0u16.to_le_bytes()); // the terminator
    b
}

/// The four lists `presence[2]` opens **after** the equipped list's own terminator:
/// `FUN_14030b6f0`'s one Equip-tab list, then `FUN_14030b9e0`'s three.
///
/// Eight zero bytes when the bag is empty.
pub fn bag_lists(equip_bag: &[BagEquip], slots: u16) -> Vec<u8> {
    let mut b = equip_bag_list(equip_bag, slots); // 1403062ee  FUN_14030b6f0(closure, 1)
    b.extend_from_slice(&[0u8; B9E0_EMPTY_LEN]); // 1403062ff  FUN_14030b9e0(closure, 1)
    b
}

/// Everything `equipped_block_with` writes after its last equipped item: the equipped
/// list's `u16 0`, then [`bag_lists`].
///
/// **This is the drop-in replacement for the three lines at the end of
/// `opcode::equipped_block_with`**, and with an empty bag it is byte-for-byte
/// [`EMPTY_EQUIPPED_TAIL`] - the ten bytes a real client has already accepted.
pub fn equipped_tail(equip_bag: &[BagEquip], slots: u16) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&0u16.to_le_bytes()); // 1403062c9  end of the equipped list
    b.extend_from_slice(&bag_lists(equip_bag, slots));
    b
}

// =========================================================================================
// The type-2 bundle item body - built, and NOT reachable from the record today
// =========================================================================================

/// The `u8` item type that selects the bundle decode `FUN_140304450`. **[L]**
///
/// `FUN_1403095e0` reads the byte at `0x1403095fb` and dispatches; the class it allocates
/// stores vtable `0x14327E588`, whose `+0x88` is literally `mov eax,2 ; ret` and whose
/// `+0x358` is `0x140304450`.
pub const BUNDLE_ITEM_TYPE: u8 = 2;

/// The `char[13]` buffer at `item + 0x6d`, read as raw bytes by `FUN_1406e9170` with
/// `R8D = 0xd` at `0x14030448e`. **[L]**
///
/// The constructor writes `0` to its first byte (`MOV byte ptr [RSI+0x6d],AL` at
/// `0x1402f7d7d`), so all-zero is the constructed, empty state.
pub const BUNDLE_OWNER_LEN: usize = 13;

/// A type-2 bundle body, for an item that is neither a throwing star nor a bullet.
///
/// ```text
///  1  u8    type = 2
///  4  u32   itemId
///  1  u8    hasCashSN = 0
///  8  raw   dateExpire
///  4  u32   0
///  1  u8    0
///  2  u16   quantity
/// 13  raw   owner name buffer
///  2  u16   attribute
///  1  u8    0
///  4  u32   0
/// ```
pub const BUNDLE_ITEM_LEN: usize = 41;

/// What a throwing star or bullet costs instead: [`BUNDLE_ITEM_LEN`] plus the `raw[8]` at
/// `0x14030451e`.
pub const BUNDLE_SERIAL_ITEM_LEN: usize = BUNDLE_ITEM_LEN + 8;

/// The two item-id families that carry an eight-byte serial in the bundle body. **[L]**
///
/// `LEA ECX,[RAX-0x1f95f0] / CMP ECX,0x2710 / JC` then `ADD EAX,0xffdc7270 / CMP EAX,0x2710
/// / JC` at `0x1403044ed`, i.e. `2070000..=2079999` and `2330000..=2339999`. In MapleStory
/// those are throwing stars and bullets - real game semantics falling out of the listing,
/// which is the strongest single check that this decode is being read correctly.
pub const BUNDLE_SERIAL_RANGES: [(u32, u32); 2] = [(2_070_000, 2_080_000), (2_330_000, 2_340_000)];

/// Whether `item_id` pulls in the extra `raw[8]` at `0x14030451e`.
pub fn bundle_has_serial(item_id: u32) -> bool {
    BUNDLE_SERIAL_RANGES.iter().any(|&(lo, hi)| item_id >= lo && item_id < hi)
}

/// How long [`bundle_item`] will be for this id.
pub fn bundle_item_len(item_id: u32) -> usize {
    if bundle_has_serial(item_id) {
        BUNDLE_SERIAL_ITEM_LEN
    } else {
        BUNDLE_ITEM_LEN
    }
}

/// The `u8` item type that selects the PET decode: `FUN_1403095e0` dispatches 3 to
/// `FUN_140304550`. **[L]**
pub const PET_ITEM_TYPE: u8 = 3;
/// The pet's name field: a fixed 13-byte string, like a bundle's owner.
pub const PET_NAME_LEN: usize = 13;
/// A pet body with no cash serial: the type byte, the 18-byte base (itemId, hasCashSN,
/// dateExpire, u32, u8) and the 48-byte pet tail (13 + 1 + 2 + 1 + 8 + 2 + 2 + 4 + 2 + 1 + 4 + 2 + 2 + 4).
pub const PET_ITEM_LEN: usize = 1 + 18 + 48;

/// **`petHue` when the pet has NOT been dyed.** The owner, 2026-09-13, with the Husky's tooltip:
/// *"My pet is not dyed, the pet should not have that line."*
///
/// The tooltip prints *"Your pet has been dyed!"* (string `0x102A`) under
/// `if (-1 < FUN_1401ba9d0(item+0xa6, item+0xae))` - and `FUN_1401ba9d0` is the client's
/// ordinary obfuscated-int reader (two dwords, `rol 5`, `xor 0xBAADF00D`), so the value it
/// returns is the hue itself. `0` is therefore "dyed, colour 0"; **only a negative hue is
/// undyed**, which is exactly what the reference server annotates on its own field
/// (`Pet.encode`: `encodeInt(getHue()); // -1`). **[L]** for the test, **[R]** for the value.
pub const PET_HUE_UNDYED: u32 = 0xFFFF_FFFF;

/// **How much life a summoned pet is told it has left.** One billion - and for this client's
/// pets it is **never read**.
///
/// The whole deadness question is one function, `FUN_1402cf680(item, 0)`, and it is a
/// three-way branch on the pet's own WZ image **[L]**:
///
/// ```text
///   limitedLife > 0   (FUN_14038a300, "limitedLife", default 0)   -> dead iff remainLife <= 0
///   life == 0         (FUN_14038a380, "life",        default 1)   -> ALIVE, unconditionally
///   otherwise                                                     -> dead iff dateDead >= 0x217E646BB058000
/// ```
///
/// `Item/Pet/5000006.img/info` has no `limitedLife`, so **the first row never fires for the
/// Husky and this constant is inert**. It was set to a billion on 2026-09-14 as a one-variable
/// candidate for the pet being dead; the run came back unchanged, which eliminated nothing -
/// the field was simply not consulted. It is kept, non-zero, because a pet whose image DOES
/// carry `limitedLife` would be dead the moment it is summoned with a `0` here.
///
/// A billion rather than `u32::MAX`: it is large in every unit this could be (31 years of
/// seconds, nonsense-but-harmless in days), stays positive if the client reads it as `i32`,
/// and leaves room above it for arithmetic that subtracts elapsed time without wrapping.
pub const PET_REMAIN_LIFE: u32 = 1_000_000_000;

/// **`dateDead`: when the pet dies - and it MUST NOT be the permanent sentinel.**
///
/// This is the field that was actually killing the Husky, and unlike everything before it the
/// finding is a **[L]** read of the client rather than a candidate:
///
/// ```text
///   1402cf718  mov  rax, [rip+...]            ; CompareFileTime
///   1402cf71f  lea  rdx, [rip+0x2fae662]      ; -> 0x14327dd88 = 150842304000000000
///   1402cf726  lea  rcx, [rdi+0x82]           ; the pet's dateDead
///   1402cf72d  call rax
///   1402cf733  setns cl                       ; DEAD iff dateDead >= that constant
/// ```
///
/// The constant at `0x14327dd88` is `ITEM_NEVER_EXPIRES` **byte for byte** - the same 2079
/// filetime this module hands every item as "no expiry". For an ordinary item that sentinel
/// means *never expires*; in this one comparison it means *this pet is a doll*. Sending it as
/// `dateDead` told the client the Husky was dead, which is why the client said so out loud
/// (*"Cannot move because the magic duration has ended"*) and why its tooltip switched to the
/// WZ's `descD`, *"The water of life has dried up..."*.
///
/// So a live pet needs `now < dateDead < ITEM_NEVER_EXPIRES`; the same function's other arm
/// (`param2 != 0`) compares a caller-supplied time against this field and calls the pet dead
/// once that time has passed it. 2077-01-01 satisfies both with fifty years to spare, and is
/// a date rather than `sentinel - 1` so that a reader can see at a glance that it is a real
/// moment and not a flag.
///
/// **Why this went unseen for two runs:** `info/life 0` takes the middle row above, which
/// returns ALIVE without ever looking at `dateDead`. Our own WZ edit had zeroed `life`, so
/// restoring it to `7` - correct in itself - is what switched the client onto the third row
/// and exposed a bad `dateDead` that had been sitting in every pet body since the day it was
/// written.
pub const PET_DATE_DEAD: u64 = 150_211_584_000_000_000;

/// **`petSkill`: the bitmask of skills a pet has learned**, and the client's own numbering.
///
/// The tooltip prints a line per skill the pet's IMAGE declares and then, for each, either
/// `(Learned)` (`0x9E5`) or *"This is an unregistered pet."* (`0x9E6`), by ANDing this `u16`
/// against that skill's bit (`FUN_14266f2d0`). **[L]**
///
/// The numbering is the client's, read off `FUN_141ed1ad0` - an eleven-entry jump table
/// (`cmp edx, 0xa`) that turns a skill index into its name string. **[L]**, and it is NOT the
/// reference server's order, which had `EXPANDED_AUTO_MOVE` second:
///
/// ```text
/// index  string  name                            the pet-image key that declares it
///   0    0x9D1   Item Pouch                      info/pickupItem
///   1    0x9D2   Auto HP Potion Pouch            info/consumeHP    (item 5190000)
///   2    0x9D4   Expanded Auto Move              info/longRange    (item 5190003)
///   3    0x9D5   Auto Move                       info/sweepForDrop (item 5190002 says dropSweep)
///   4    0x9D3   Auto MP Potion Pouch            info/consumeMP    (item 5190001)
///   5    0x9D7   Ignore Item
///   6    0x9D8   Auto Buff
///   7    0x9D9   Auto Feed and Movement Skill
///   8    0x9DA   Fatten Up
///   9    0x9DB   Pet Shop Skill
/// ```
///
/// **Meso Magnet (`0x9DC`) is not in that table at all** and needs no key: the owner, 2026-09-13,
/// with every skill key cleared, *"the Husky should by default come with Meso Magnet and Item
/// Pouch. Currently it is missing the Item Pouch skill by default"* - so Meso Magnet showed on
/// its own and Item Pouch did not. It is innate. **[D]**
///
/// **The bit is NOT `1 << index`** - that was [I], and it was wrong. The owner, 2026-09-16, using
/// the Expanded Auto Move item: *"You do not have a pet that can use this skill."* The client's
/// own tooltip builder `FUN_1414b89b0` maps each skill index to a bit off `ebx = 4`:
/// index 0 -> `ebx-3 = 1`, 2 -> `ebx-2 = 2`, 3 -> `ebx = 4`, 5 -> `ebx+4 = 8`, 1 -> `0x20`,
/// 4 -> `0x40`, 6 -> `0x80`. So the learned mask this client reads at `pet+0x1c` is:
///
/// ```text
///   0x01 Item Pouch   0x02 Expanded Auto Move   0x04 Auto Move   0x08 Ignore Item
///   0x20 Auto HP      0x40 Auto MP              0x80 Auto Buff
/// ```
///
/// **[L]**, and the modern reference's `PetSkill` enum is byte-for-byte the same
/// (`ITEM_PICKUP 0x1, EXPANDED_AUTO_MOVE 0x2, AUTO_MOVE 0x4, IGNORE_ITEM 0x8, AUTO_HP 0x20`).
/// The old `1 << index` values put Auto HP on Expanded Auto Move's bit, Expanded on Auto
/// Move's, and Auto Move on Ignore Item's - which is why a "learned" Auto Move showed as
/// "Ignore Item (Learned)" and every skill item taught the wrong thing.
pub const PET_SKILL_ITEM_POUCH: u16 = 0x01;
/// See [`PET_SKILL_ITEM_POUCH`]. Bought as item `5190003` (its key is `longRange`).
pub const PET_SKILL_EXPANDED_AUTO_MOVE: u16 = 0x02;
/// See [`PET_SKILL_ITEM_POUCH`]. Bought as item `5190002` (its key is `dropSweep`). **The
/// client requires this before it will apply Expanded Auto Move** - the chain the refusal
/// above was enforcing.
pub const PET_SKILL_AUTO_MOVE: u16 = 0x04;
/// See [`PET_SKILL_ITEM_POUCH`]. Index 5; no item this server sells grants it.
pub const PET_SKILL_IGNORE_ITEM: u16 = 0x08;
/// See [`PET_SKILL_ITEM_POUCH`]. Bought as item `5190000`.
pub const PET_SKILL_AUTO_HP: u16 = 0x20;
/// See [`PET_SKILL_ITEM_POUCH`]. Bought as item `5190001`.
pub const PET_SKILL_AUTO_MP: u16 = 0x40;

/// **What every pet knows the day it is bought.** The owner, 2026-09-13: *"the Husky should by
/// default come with Meso Magnet and Item Pouch."*
///
/// Meso Magnet is innate and carries no bit. **Item Pouch only.** For most of 2026-09-16 this
/// also carried Auto Move and Expanded Auto Move (the owner: *"turn all pets into vacuum pets"*),
/// and by evening: *"longRange belongs to a Pet Skill that the clients have to purchase and
/// activate. If we already send that for free, we need to tie it to the pet skill instead of
/// having that for free."* So the four shop items (`5190000..3`) each teach a bit no pet is
/// born with. (The in-range vacuum is NOT one of them - it is free, [`PET_WONDER_GRADE_VACUUM`]
/// on every item.) the owner, 2026-09-17: *"The only default skills it should have is Meso
/// Magnet and Item Pouch."* The installer (step 4c) declares only `pickupItem` now, so a fresh
/// pet's tooltip lists only those two; a purchased skill still lists as *(Learned)* off the
/// mask, without its WZ key, so the declaration is not what lets it be bought.
pub const PET_SKILLS_LEARNED_AT_START: u16 = PET_SKILL_ITEM_POUCH;

/// The `wonderGrade` that turns the client's pet pickup box from "what it walks over" into a
/// long-range sweep, and the grade the client labels **"Petite Luna"**: **6**.
/// `research/pet-vacuum-wondergrade-2026-09-16.md`.
///
/// `FUN_14179e990` - the drop pool's "does this pet reach a drop" scan, the only caller of the
/// `0x0205` builder - starts from the constant box `(-25,-50,25,10)` around the pet and swaps
/// in the server-supplied wide box from `0x0198` when [`FUN_14038a5b0`](self) reads the pet's
/// wonder grade as `== 6` (`FUN_140374c80` is literally `cmp ecx,6`). The grade comes first
/// from the pet ITEM's own `u16` at `+0xba` (`0x140304730`, the field after `giantRate`) and
/// only when that is 0 from the pet image's `info/wonderGrade`. **[L]**, every step read.
///
/// **This is FREE on every pet.** The owner, 2026-09-17: *"vacuuming loot within a certain range of
/// the pet (Petite Luna) should be free. Auto move (the pet automatically moving towards the
/// loot to pick up items) should be a skill that players have to purchase. Expanded auto move
/// (longRange) ... should also remain a skill."* So the in-range suck-up (this box) and the
/// Petite Luna designation are unconditional, and every pet's item carries grade 6. The two
/// movement skills are the `sweepForDrop`/`longRange` bits, bought as `5190002`/`5190003`; the
/// client acts on them for the pet's *walking* toward drops, which is separate from this box.
pub const PET_WONDER_GRADE_VACUUM: u16 = 6;

/// The Pet Name Tag, `Item/Cash/0517.img`. Arrives on `0x0116` with the pet's serial and the
/// new name - `crate::cashitem::UseCashItem::text`.
pub const PET_NAME_TAG: u32 = 5_170_000;

/// **Which skill a Cash-shop skill item teaches**, or `None` for anything else. The four the
/// shop sells (`research/pet-skills-2026-09-13.md` §2), each `add 1` of one key:
///
/// ```text
/// 5190000  Auto HP Potion Skill      consumeHP     -> PET_SKILL_AUTO_HP
/// 5190001  Auto MP Potion Skill      consumeMP     -> PET_SKILL_AUTO_MP
/// 5190002  Auto Move Skill           dropSweep     -> PET_SKILL_AUTO_MOVE
/// 5190003  Expanded Auto Move Skill  longRange     -> PET_SKILL_EXPANDED_AUTO_MOVE
/// ```
pub fn pet_skill_bit_for_item(item_id: u32) -> Option<u16> {
    match item_id {
        5_190_000 => Some(PET_SKILL_AUTO_HP),
        5_190_001 => Some(PET_SKILL_AUTO_MP),
        5_190_002 => Some(PET_SKILL_AUTO_MOVE),
        5_190_003 => Some(PET_SKILL_EXPANDED_AUTO_MOVE),
        _ => None,
    }
}

/// A pet (item type 3) - what `FUN_140304550` reads after the shared base. The owner,
/// 2026-09-13: *"They should also be permanent duration. They should never need to be
/// revived."*
///
/// The base is the bundle's (`FUN_1403035a0`: itemId, hasCashSN [+ serial], dateExpire,
/// u32, u8). Then, in the order `tools/reads.py 0x140304550 2` lists them **[L]**, with the
/// meanings the reference server's `PetItem.encode` gives the same widths in the same order
/// **[R]**:
///
/// ```text
/// raw[13]  name            140304577   the pet's name; the item's own name until renamed
/// u8       level           14030457f   1
/// u16      closeness       1403045b5   0
/// u8       fullness        1403045cc   100 - fed
/// raw[8]   dateDead        14030460f   PET_DATE_DEAD - and NOT ITEM_NEVER_EXPIRES, which the
///                                      client reads as "this pet is a doll" (FUN_1402cf680)
/// u16      petAttribute    140304617   0
/// u16      petSkill        14030462e   PET_SKILLS_LEARNED_AT_START - Item Pouch; the rest are bought
/// u32      remainLife      140304645   0 - not a limited-life pet
/// u16      attribute       14030467e   0
/// u8       active          14030469b   0
/// u32      petHue          1403046da   PET_HUE_UNDYED - 0 is "dyed with colour 0"
/// u16      giantRate       140304713   0
/// u16      wonderGrade     140304730   PET_WONDER_GRADE_VACUUM - 6 on every pet, the free
///                                      Petite Luna in-range vacuum (the owner 2026-09-17)
/// u32                      14030474d   0
/// ```
///
/// Before this existed a pet was refused at purchase; a bundle body sent for one had killed
/// the client on 2026-08-26, which is the whole reason the type byte and the tail are read
/// off the client rather than assumed.
pub fn pet_item_with_cash_sn(item_id: u32, name: &str, cash_sn: Option<std::num::NonZeroU64>) -> Vec<u8> {
    pet_item_with_state(item_id, name, cash_sn, 0, &PetVitals::default())
}

/// **What the pet item says about the pet**: the four fields of the body that move.
///
/// Level, closeness and fullness are what the KEY BINDINGS-adjacent "Show Pet Info" panel and
/// the pet tooltip print (`research/pet-tooltip-and-commands-2026-09-13.md`), read from the
/// Cash item and nowhere else - so every change is a re-send of the item. `store::pets` holds
/// them; `crate::petfood` and `world::petlevel` move them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PetVitals {
    /// `1..=30`. `1403045 7f`.
    pub level: u8,
    /// Total closeness. `1403045b5`; a `u16` on the wire, so 30 000 (level 30) fits.
    pub closeness: u16,
    /// `0..=100`. `1403045cc`.
    pub fullness: u8,
    /// The learned-skill mask, [`PET_SKILLS_LEARNED_AT_START`] until a skill item adds a bit.
    pub skills: u16,
}

impl Default for PetVitals {
    /// A pet fresh from the shop: level 1, no closeness, full, Item Pouch.
    fn default() -> Self {
        PetVitals { level: 1, closeness: 0, fullness: 100, skills: PET_SKILLS_LEARNED_AT_START }
    }
}

/// [`pet_item_with_cash_sn`] with the `active` byte set - `0` in the bag, `1` while the pet is
/// summoned (`crate::pet`); the reference's `PetItem.activeState` is `petIdx + 1`, and this
/// client accepts one pet, so the only live value is `1` - and the pet's [`PetVitals`].
pub fn pet_item_with_state(
    item_id: u32,
    name: &str,
    cash_sn: Option<std::num::NonZeroU64>,
    active: u8,
    vitals: &PetVitals,
) -> Vec<u8> {
    let mut b = Vec::with_capacity(PET_ITEM_LEN + 8);
    b.push(PET_ITEM_TYPE); //                                   1403095fb  u8   item type
    b.extend_from_slice(&item_id.to_le_bytes()); //             1403035c5  u32  itemId
    match cash_sn {
        Some(sn) => {
            b.push(1); //                                       140303787  u8   hasCashSN
            b.extend_from_slice(&sn.get().to_le_bytes()); //    14030379d  raw[8]
        }
        None => b.push(0),
    }
    b.extend_from_slice(&ITEM_NEVER_EXPIRES.to_le_bytes()); //  1403037b9  raw[8] dateExpire
    b.extend_from_slice(&0u32.to_le_bytes()); //                1403037c1  u32
    b.push(0); //                                               1403037cc  u8
    // FUN_140304550.
    let mut fixed = [0u8; PET_NAME_LEN];
    let bytes = name.as_bytes();
    let n = bytes.len().min(PET_NAME_LEN - 1); // a terminator stays
    fixed[..n].copy_from_slice(&bytes[..n]);
    b.extend_from_slice(&fixed); //                              140304577  raw[13] name
    b.push(vitals.level); //                                    14030457f  u8   level
    b.extend_from_slice(&vitals.closeness.to_le_bytes()); //    1403045b5  u16  closeness
    b.push(vitals.fullness); //                                 1403045cc  u8   fullness
    b.extend_from_slice(&PET_DATE_DEAD.to_le_bytes()); //       14030460f  raw[8] dateDead
    b.extend_from_slice(&0u16.to_le_bytes()); //                140304617  u16  petAttribute
    b.extend_from_slice(&vitals.skills.to_le_bytes()); //       14030462e  u16  petSkill
    b.extend_from_slice(&PET_REMAIN_LIFE.to_le_bytes()); //     140304645  u32  remainLife
    b.extend_from_slice(&0u16.to_le_bytes()); //                14030467e  u16  attribute
    b.push(active); //                                          14030469b  u8   active
    b.extend_from_slice(&PET_HUE_UNDYED.to_le_bytes()); //      1403046da  u32  petHue
    b.extend_from_slice(&0u16.to_le_bytes()); //                140304713  u16  giantRate
    b.extend_from_slice(&PET_WONDER_GRADE_VACUUM.to_le_bytes()); // 140304730 u16 wonderGrade -> item+0xba, read by FUN_14038a5b0; 6 = the wide pickup box AND the free Petite Luna designation (the owner 2026-09-17: the in-range vacuum is free)
    b.extend_from_slice(&0u32.to_le_bytes()); //                14030474d  u32
    debug_assert_eq!(b.len(), PET_ITEM_LEN + if cash_sn.is_some() { 8 } else { 0 });
    b
}

/// One stackable item, as `FUN_140304450` - the type-2 `vtable+0x358` decode - reads it.
///
/// Every field below is **[L]**, read off `research/msexe-itemslot-bundle-decode.txt` and
/// `research/msexe-itemslot-base.txt`, and **cross-checked against the client's own
/// encoder**: `FUN_1402cf5c0` is the same class's serialiser (vtable-only, so
/// `tools/callers.py` reports zero callers) and it emits the identical sequence - base,
/// `u16` from `+0x4d/+0x51`, `raw[13]` from `+0x6d`, `u16` from `+0x55/+0x59`, `u8` from
/// `+0x5d/+0x61`, the star/bullet `raw[8]` from `+0x65`, `u32` from `+0x7a`.
///
/// **Which `u16` is the quantity is measured, not assumed.** The two are symmetric on the
/// wire, so swapping them would be invisible until a client run. Vtable slot `+0x98` is
/// "how many of this item is here": `mov eax,1 ; ret` for type 1 and type 3, and for type 2
/// it is `FUN_1402fbe60`, five instructions that de-obfuscate `+0x4d/+0x51` and zero-extend
/// the result to 16 bits. That field is the **first** `u16` after the base decode. The
/// other `u16` is a bit field - `FUN_1402fdab0` reads it and immediately does `TEST AL,0x2`.
///
/// **`hasCashSN` must be 0.** Unlike the equip body, the bundle has no compensating
/// `raw[8]`, so a non-zero flag makes this body eight bytes *longer* rather than the same
/// length - and the character record has no length prefix and no resync point.
///
/// The zeros are the constructor's own values: `FUN_1402f7cd0` ends with
/// `XOR EAX,EAX / MOV [RSI+0x65],RAX / MOV [RSI+0x6d],AL / MOV [RSI+0x7a],EAX`.
///
/// **NOT WIRED.** No bag that holds stackables is switched on in the character record - the
/// Use, Set Up, Etc and Cash lists are behind presence bytes 3, 4, 5 and 6, and three of
/// those bytes also open blocks that have never been decoded (`research/bag-lists.md` §6).
/// This function exists so goals F and G start from a decoded body rather than a blank page.
pub fn bundle_item(item_id: u32, quantity: u16, attribute: u16, owner: &[u8; BUNDLE_OWNER_LEN]) -> Vec<u8> {
    bundle_item_with_cash_sn(item_id, quantity, attribute, owner, None)
}

/// [`bundle_item`] carrying a **cash serial** in the item's `+0x38` - and it is **eight bytes
/// longer** for it, which is the whole reason [`bundle_item`] keeps the flag at zero.
///
/// Where it is safe: a body the client reads on its own, with nothing after it that depends
/// on its length - the cash shop's `0x19` locker-to-bag reply, whose handler erases the
/// item's `+0x38` from the locker map (see `crate::opcode::equipped_item_with_cash_sn` for
/// the listing and the run that found it). Where it is not: the character record and the
/// bag lists, which have no resync point.
pub fn bundle_item_with_cash_sn(
    item_id: u32,
    quantity: u16,
    attribute: u16,
    owner: &[u8; BUNDLE_OWNER_LEN],
    cash_sn: Option<std::num::NonZeroU64>,
) -> Vec<u8> {
    let mut b = Vec::with_capacity(bundle_item_len(item_id) + 8);
    b.push(BUNDLE_ITEM_TYPE); //                              1403095fb  u8   item type

    // FUN_1403035a0, the base decode. Enumerated rather than filtered: the only read
    // primitives called anywhere in 0x1403035a0..0x1403037f5 are 2x 0x1406e8c20,
    // 2x 0x1406e8ae0 and 2x 0x1406e9170. It does NOT call FUN_140303b40 - the two equip
    // bitmasks belong to FUN_140304100, which is why a bundle is 41 bytes and not 49.
    b.extend_from_slice(&item_id.to_le_bytes()); //            1403035c5  u32  itemId
    match cash_sn {
        Some(sn) => {
            b.push(1); //                                      140303787  u8   hasCashSN
            b.extend_from_slice(&sn.get().to_le_bytes()); //   14030379d  raw[8] -> +0x38
        }
        None => b.push(0), //                                  140303787  u8   hasCashSN
    }
    b.extend_from_slice(&ITEM_NEVER_EXPIRES.to_le_bytes()); // 1403037b9  raw[8] dateExpire
    b.extend_from_slice(&0u32.to_le_bytes()); //               1403037c1  u32  -> +0x48
    b.push(0); //                                              1403037cc  u8   -> +0x4c

    // Back in FUN_140304450.
    b.extend_from_slice(&quantity.to_le_bytes()); //           14030446d  u16  -> +0x4d/+0x51
    b.extend_from_slice(owner); //                             14030448e  raw[13] -> +0x6d
    b.extend_from_slice(&attribute.to_le_bytes()); //          14030449a  u16  -> +0x55/+0x59
    b.push(0); //                                              1403044b1  u8   -> +0x5d
    if bundle_has_serial(item_id) {
        b.extend_from_slice(&[0u8; 8]); //                     14030451e  raw[8] -> +0x65
    }
    b.extend_from_slice(&0u32.to_le_bytes()); //               140304526  u32  -> +0x7a

    debug_assert_eq!(b.len(), bundle_item_len(item_id) + if cash_sn.is_some() { 8 } else { 0 });
    b
}

/// One stackable in a bag: a position and a bundle body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BagBundle {
    /// The bag slot, 1-based.
    pub pos: u16,
    pub item_id: u32,
    /// How many are in the stack. Vtable slot `+0x98`; see [`bundle_item`].
    pub quantity: u16,
    /// The attribute bit field at `+0x55/+0x59`. Only bit 1 is known to be read.
    pub attribute: u16,
    /// The `char[13]` name buffer at `+0x6d`. Zeros is the constructed, empty state.
    pub owner: [u8; BUNDLE_OWNER_LEN],
}

impl BagBundle {
    /// A plain stack: no attribute bits, no owner name.
    pub fn plain(pos: u16, item_id: u32, quantity: u16) -> Self {
        Self { pos, item_id, quantity, attribute: 0, owner: [0; BUNDLE_OWNER_LEN] }
    }

    /// This item's wire body.
    pub fn body(&self) -> Vec<u8> {
        bundle_item(self.item_id, self.quantity, self.attribute, &self.owner)
    }
}

/// A `FUN_14030b6f0` list of stackables - `u16 pos` + a type-2 body per entry, then `u16 0`.
///
/// **NOT WIRED, and do not wire it without reading `research/bag-lists.md` §6.** The helper
/// is the same one the Equip tab uses, so the framing is right, but reaching it for
/// inventory type 2..5 means setting presence byte 3, 4, 5 or 6, and bytes 3, 4 and 5 also
/// open regions that read four `u32`, a `u64`, a count and a whole extra item apiece -
/// none of which is decoded. The record has no resync point.
pub fn bundle_bag_list(items: &[BagBundle], slots: u16) -> Vec<u8> {
    let mut b = Vec::new();
    for item in items {
        if !bag_pos_is_kept(item.pos, slots) {
            continue;
        }
        b.extend_from_slice(&item.pos.to_le_bytes());
        b.extend_from_slice(&item.body());
    }
    b.extend_from_slice(&0u16.to_le_bytes());
    b
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcode::{DEFAULT_INVENTORY_SLOTS, EQUIPPED_ITEM_LEN};

    /// **The regression anchor.** An empty bag has to reproduce, byte for byte, the ten
    /// zero bytes `equipped_block_with` sends today - the sequence in the record that put a
    /// dressed character on map 1 with no client fault. Everything else in this module is
    /// static analysis; this is the one thing a real client has already accepted.
    #[test]
    fn an_empty_bag_is_the_ten_zero_bytes_already_on_the_wire() {
        assert_eq!(equipped_tail(&[], DEFAULT_INVENTORY_SLOTS), EMPTY_EQUIPPED_TAIL);
        assert_eq!(equipped_tail(&[], DEFAULT_INVENTORY_SLOTS).len(), 10);
        // and the same split the old code wrote: 2 for the equipped list, 2 for b6f0, 6 for
        // b9e0's three lists.
        assert_eq!(bag_lists(&[], DEFAULT_INVENTORY_SLOTS), vec![0u8; 8]);
        assert_eq!(equip_bag_list(&[], DEFAULT_INVENTORY_SLOTS), vec![0u8; 2]);
    }

    #[test]
    fn one_bag_equip_is_a_u16_position_then_a_125_byte_type_1_body() {
        let bag = [BagEquip::plain(1, 1_040_003)];
        let list = equip_bag_list(&bag, DEFAULT_INVENTORY_SLOTS);
        assert_eq!(list.len(), 2 + EQUIPPED_ITEM_LEN + 2);
        assert_eq!(&list[..2], &1u16.to_le_bytes());
        assert_eq!(list[2], 1, "the item type byte the factory reads at 1403095fb");
        assert_eq!(&list[3..7], &1_040_003u32.to_le_bytes());
        assert_eq!(&list[list.len() - 2..], &0u16.to_le_bytes(), "the list terminator");
    }

    #[test]
    fn the_whole_tail_is_the_equipped_terminator_then_the_bag() {
        let bag = [BagEquip::plain(3, 1_060_002)];
        let tail = equipped_tail(&bag, DEFAULT_INVENTORY_SLOTS);
        assert_eq!(&tail[..2], &0u16.to_le_bytes(), "the equipped list still ends first");
        assert_eq!(&tail[2..], &bag_lists(&bag, DEFAULT_INVENTORY_SLOTS)[..]);
        assert_eq!(tail.len(), 2 + 2 + EQUIPPED_ITEM_LEN + 2 + B9E0_EMPTY_LEN);
        assert_eq!(&tail[tail.len() - B9E0_EMPTY_LEN..], &[0u8; 6]);
    }

    /// `0x14030b7c9` / `0x14030b7d6`: the client keeps `1 <= pos <= slotMax` and consumes
    /// then discards anything else. Sending a discarded item costs 127 bytes and loses it,
    /// so it is dropped here instead.
    #[test]
    fn positions_the_client_would_discard_are_not_sent() {
        assert!(!bag_pos_is_kept(0, 30), "0 is the terminator, never a slot");
        assert!(bag_pos_is_kept(1, 30));
        assert!(bag_pos_is_kept(30, 30));
        assert!(!bag_pos_is_kept(31, 30));

        let bag = [BagEquip::plain(0, 1_040_003), BagEquip::plain(31, 1_040_003)];
        assert_eq!(equip_bag_list(&bag, 30), vec![0u8; 2], "both dropped, list still ends");

        // and a bigger bag keeps the one that now fits
        assert_eq!(equip_bag_list(&bag, 125).len(), 2 + EQUIPPED_ITEM_LEN + 2);
    }

    /// The position is read with `MOVZX` at `0x14030b792` and `0x14030b99f`, so it is
    /// unsigned and the whole `1..=65535` range is expressible. Nothing signed here.
    #[test]
    fn the_position_is_an_unsigned_u16() {
        let bag = [BagEquip::plain(40_000, 1_040_003)];
        let list = equip_bag_list(&bag, 65_535);
        assert_eq!(&list[..2], &40_000u16.to_le_bytes());
    }

    #[test]
    fn presence_bytes_match_the_key_initialisers() {
        assert_eq!(BAG_PRESENCE_BYTE, [2, 3, 4, 5, 6, 44]);
        assert_eq!(BAG_PRESENCE_BYTE[usize::from(BAG_EQUIP) - 1], crate::opcode::PRESENCE_EQUIPPED);
        assert_eq!(BAG_TAB_NAMES.len(), BAG_PRESENCE_BYTE.len());
    }

    /// The three `FUN_14030b9e0` lists are not tabs and cannot hold anything this server
    /// creates: their positions start at 3000.
    #[test]
    fn the_b9e0_ranges_are_nowhere_near_a_bag_slot() {
        assert_eq!(B9E0_RANGES, [(3000, 3032), (3100, 3132), (3200, 3232)]);
        for (lo, hi) in B9E0_RANGES {
            assert!(lo > crate::opcode::MAX_INVENTORY_SLOTS);
            assert_eq!(hi - lo, 32);
        }
    }

    // -------------------------------------------------------------------------------------
    // The type-2 bundle body
    // -------------------------------------------------------------------------------------

    #[test]
    fn a_bundle_body_is_41_bytes_with_every_field_where_the_listing_puts_it() {
        let b = bundle_item(2_000_000, 50, 0, &[0; BUNDLE_OWNER_LEN]);
        assert_eq!(b.len(), BUNDLE_ITEM_LEN);
        assert_eq!(b.len(), 41);

        assert_eq!(b[0], 2, "1403095fb  the factory's type byte");
        assert_eq!(&b[1..5], &2_000_000u32.to_le_bytes(), "1403035c5  itemId");
        assert_eq!(b[5], 0, "140303787  hasCashSN - a 1 here would lengthen the body by 8");
        // And with the serial it IS eight longer: flag 1, raw[8] serial, then the same bytes.
        let sn = std::num::NonZeroU64::new(0x1_0000_0003).unwrap();
        let c = bundle_item_with_cash_sn(2000000, 50, 0, &[0u8; BUNDLE_OWNER_LEN], Some(sn));
        assert_eq!(c.len(), BUNDLE_ITEM_LEN + 8);
        assert_eq!(&c[..5], &b[..5]);
        assert_eq!(c[5], 1, "140303787  hasCashSN");
        assert_eq!(&c[6..14], &sn.get().to_le_bytes(), "14030379d  raw[8] -> +0x38");
        assert_eq!(&c[14..], &b[6..], "everything after the flag is unchanged");
        assert_eq!(&b[6..14], &ITEM_NEVER_EXPIRES.to_le_bytes(), "1403037b9  dateExpire");
        assert_eq!(&b[14..18], &0u32.to_le_bytes(), "1403037c1  -> +0x48");
        assert_eq!(b[18], 0, "1403037cc  -> +0x4c");
        assert_eq!(&b[19..21], &50u16.to_le_bytes(), "14030446d  quantity -> +0x4d/+0x51");
        assert_eq!(&b[21..34], &[0u8; 13], "14030448e  raw[13] -> +0x6d");
        assert_eq!(&b[34..36], &0u16.to_le_bytes(), "14030449a  attribute -> +0x55/+0x59");
        assert_eq!(b[36], 0, "1403044b1  u8 -> +0x5d");
        assert_eq!(&b[37..41], &0u32.to_le_bytes(), "140304526  u32 -> +0x7a");
    }

    /// The quantity is the **first** `u16`, and it is there because vtable slot `+0x98` -
    /// `mov eax,1 ; ret` for an equip, `FUN_1402fbe60` reading `+0x4d/+0x51` for a bundle -
    /// says so. If this ever has to be swapped with the attribute, that measurement is the
    /// thing to re-check first.
    #[test]
    fn the_quantity_is_the_first_u16_and_the_attribute_the_second() {
        let b = bundle_item(4_000_001, 0x1234, 0x5678, &[0; BUNDLE_OWNER_LEN]);
        assert_eq!(&b[19..21], &0x1234u16.to_le_bytes());
        assert_eq!(&b[34..36], &0x5678u16.to_le_bytes());
    }

    #[test]
    fn the_owner_buffer_is_thirteen_raw_bytes_and_is_not_length_prefixed() {
        let mut owner = [0u8; BUNDLE_OWNER_LEN];
        owner[..4].copy_from_slice(b"Wisp");
        let b = bundle_item(2_000_001, 1, 0, &owner);
        assert_eq!(b.len(), BUNDLE_ITEM_LEN, "a name must not change the length");
        assert_eq!(&b[21..34], &owner);
    }

    /// `0x1403044ed`: `itemId - 2070000 < 10000` or `itemId - 2330000 < 10000` reads an
    /// extra `raw[8]`. Throwing stars and bullets - the only two consumable families in this
    /// game that carry a serial.
    #[test]
    fn stars_and_bullets_are_eight_bytes_longer() {
        assert!(bundle_has_serial(2_070_000), "Subi throwing stars, in gm-handbook/itemdata.txt");
        assert!(bundle_has_serial(2_079_999));
        assert!(!bundle_has_serial(2_080_000));
        assert!(bundle_has_serial(2_330_000));
        assert!(bundle_has_serial(2_339_999));
        assert!(!bundle_has_serial(2_340_000));
        assert!(!bundle_has_serial(2_000_000), "a Red Potion is an ordinary stackable");
        assert!(!bundle_has_serial(4_000_001), "an ETC drop is an ordinary stackable");

        let star = bundle_item(2_070_000, 200, 0, &[0; BUNDLE_OWNER_LEN]);
        assert_eq!(star.len(), BUNDLE_SERIAL_ITEM_LEN);
        assert_eq!(star.len(), 49);
        // the serial sits between the u8 at +0x5d and the trailing u32
        assert_eq!(&star[37..45], &[0u8; 8]);
        assert_eq!(&star[45..49], &0u32.to_le_bytes());
    }

    #[test]
    fn a_bundle_list_frames_like_the_equip_one() {
        let bag = [BagBundle::plain(1, 2_000_000, 100), BagBundle::plain(2, 4_000_001, 3)];
        let list = bundle_bag_list(&bag, DEFAULT_INVENTORY_SLOTS);
        assert_eq!(list.len(), 2 * (2 + BUNDLE_ITEM_LEN) + 2);
        assert_eq!(&list[..2], &1u16.to_le_bytes());
        assert_eq!(&list[list.len() - 2..], &0u16.to_le_bytes());
        assert_eq!(bundle_bag_list(&[], DEFAULT_INVENTORY_SLOTS), vec![0u8; 2]);
    }
}

#[cfg(test)]
mod pet_tests {
    use super::*;

    /// **The learned-mask bits are the client's, not `1 << index`.** `FUN_1414b89b0` and the
    /// modern `PetSkill` enum agree; a scramble here taught every skill item the wrong skill.
    #[test]
    fn the_pet_skill_bits_match_the_clients_own_mask() {
        assert_eq!(PET_SKILL_ITEM_POUCH, 0x01);
        assert_eq!(PET_SKILL_EXPANDED_AUTO_MOVE, 0x02);
        assert_eq!(PET_SKILL_AUTO_MOVE, 0x04);
        assert_eq!(PET_SKILL_IGNORE_ITEM, 0x08);
        assert_eq!(PET_SKILL_AUTO_HP, 0x20);
        assert_eq!(PET_SKILL_AUTO_MP, 0x40);
        // The four the shop sells, and the skill each item's WZ key grants.
        assert_eq!(pet_skill_bit_for_item(5_190_000), Some(PET_SKILL_AUTO_HP));
        assert_eq!(pet_skill_bit_for_item(5_190_001), Some(PET_SKILL_AUTO_MP));
        assert_eq!(pet_skill_bit_for_item(5_190_002), Some(PET_SKILL_AUTO_MOVE));
        assert_eq!(pet_skill_bit_for_item(5_190_003), Some(PET_SKILL_EXPANDED_AUTO_MOVE));
        // Item Pouch is the default; none of the four are in it.
        assert_eq!(PET_SKILLS_LEARNED_AT_START, PET_SKILL_ITEM_POUCH);
    }

    /// **Every pet's item carries `wonderGrade 6` - the free in-range vacuum and the Petite
    /// Luna designation.** The owner, 2026-09-17: *"vacuuming loot within a certain range of the pet
    /// (Petite Luna) should be free."* So the grade does not follow any skill bit; a fresh pet
    /// and one that bought every skill both read 6 at `0x140304730` (item `+0xba`), and
    /// `FUN_14038a5b0 == 6` is the client's whole test for the wide pickup box.
    #[test]
    fn every_pet_carries_the_vacuum_grade_regardless_of_skills() {
        assert_eq!(PET_WONDER_GRADE_VACUUM, 6);
        let plain = pet_item_with_state(5000006, "Husky", None, 0, &PetVitals::default());
        let skilled = pet_item_with_state(
            5000006,
            "Husky",
            None,
            0,
            &PetVitals {
                skills: PET_SKILLS_LEARNED_AT_START | PET_SKILL_AUTO_MOVE | PET_SKILL_EXPANDED_AUTO_MOVE,
                ..PetVitals::default()
            },
        );
        assert_eq!(&plain[61..63], &6u16.to_le_bytes(), "a fresh pet is a vacuum pet: grade 6");
        assert_eq!(&skilled[61..63], &6u16.to_le_bytes(), "and a fully-skilled one is no different");
        // Only the petSkill mask (46..48) differs; the grade does not move with it.
        assert_eq!(&plain[46..48], &PET_SKILLS_LEARNED_AT_START.to_le_bytes());
        assert_eq!(&skilled[61..63], &plain[61..63]);
    }

    /// The pet body is the bundle's base followed by the fourteen reads of FUN_140304550, in
    /// their order and widths; both dates are the never-expires sentinel.
    #[test]
    fn a_pet_body_is_type_3_with_the_fourteen_pet_reads_after_the_base() {
        let b = pet_item_with_cash_sn(5000001, "Brown Puppy", None);
        assert_eq!(b.len(), PET_ITEM_LEN);
        assert_eq!(b[0], PET_ITEM_TYPE);
        assert_eq!(u32::from_le_bytes(b[1..5].try_into().unwrap()), 5000001);
        assert_eq!(b[5], 0, "no cash serial");
        assert_eq!(&b[6..14], &ITEM_NEVER_EXPIRES.to_le_bytes(), "dateExpire: never");
        // 14..18 u32, 18 u8: the base's tail. Then the pet.
        let name = &b[19..32];
        assert_eq!(&name[..11], b"Brown Puppy");
        assert_eq!(name[11], 0, "terminated inside the 13");
        assert_eq!(b[32], 1, "level");
        assert_eq!(&b[33..35], &0u16.to_le_bytes(), "closeness");
        assert_eq!(b[35], 100, "fullness");
        assert_eq!(
            &b[36..44],
            &PET_DATE_DEAD.to_le_bytes(),
            "dateDead: it must be strictly BELOW ITEM_NEVER_EXPIRES. FUN_1402cf680 calls a pet dead when this field is >= that sentinel, and the client then refuses to move it"
        );
        assert!(PET_DATE_DEAD < ITEM_NEVER_EXPIRES, "the sentinel is the client's dead marker");
        assert_eq!(b[44..].len(), 2 + 2 + 4 + 2 + 1 + 4 + 2 + 2 + 4);
        assert_eq!(&b[44..46], &0u16.to_le_bytes(), "petAttribute");
        // The two fields the tooltip reads. A zero skill mask makes every skill the WZ grants
        // read "This is an unregistered pet."; a zero hue makes it read "Your pet has been dyed!".
        assert_eq!(&b[46..48], &PET_SKILLS_LEARNED_AT_START.to_le_bytes(), "petSkill: Item Pouch, learned from the start");
        assert_eq!(PET_SKILLS_LEARNED_AT_START, 0b0001, "Item Pouch alone - Meso Magnet is innate and has no bit, the four shop skills are bought");
        assert_eq!(
            PET_SKILLS_LEARNED_AT_START & (PET_SKILL_AUTO_HP | PET_SKILL_AUTO_MP | PET_SKILL_AUTO_MOVE | PET_SKILL_EXPANDED_AUTO_MOVE),
            0,
            "The owner, 2026-09-16: longRange belongs to a Pet Skill the clients have to purchase"
        );
        assert_eq!(
            &b[48..52],
            &PET_REMAIN_LIFE.to_le_bytes(),
            "remainLife, and it must not be 0 - the client answered a 0 here with \"the magic \
             duration has ended\" and refused to move or draw the pet"
        );
        assert_eq!(&b[52..54], &0u16.to_le_bytes(), "attribute");
        assert_eq!(b[54], 0, "active");
        assert_eq!(&b[55..59], &PET_HUE_UNDYED.to_le_bytes(), "petHue: undyed, not colour 0");
        assert_eq!(&b[59..61], &0u16.to_le_bytes(), "giantRate: 0");
        assert_eq!(&b[61..63], &PET_WONDER_GRADE_VACUUM.to_le_bytes(), "wonderGrade 6: the free Petite Luna vacuum, on every pet");
        assert!(b[63..].iter().all(|&x| x == 0), "the tail: zero");

        // With a serial: eight more bytes after the flag, everything else in place.
        let sn = std::num::NonZeroU64::new(0x1122_3344_5566_7788).unwrap();
        let c = pet_item_with_cash_sn(5000001, "Brown Puppy", Some(sn));
        assert_eq!(c.len(), PET_ITEM_LEN + 8);
        assert_eq!(c[5], 1);
        assert_eq!(&c[6..14], &sn.get().to_le_bytes());
        assert_eq!(&c[27..40], &b[19..32], "the name follows the serial");

        // A long name is cut to twelve bytes and a terminator, never thirteen.
        let d = pet_item_with_cash_sn(5000010, "A name longer than thirteen", None);
        assert_eq!(d.len(), PET_ITEM_LEN);
        assert_eq!(d[31], 0);
    }
}
