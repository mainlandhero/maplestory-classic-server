# The vacuum is in the client, keyed on `wonderGrade == 6`, and fed by `0x0198`

2026-09-16, evening. The owner: *"longRange belongs to a Pet Skill that the clients have to purchase
and activate. If we already send that for free, we need to tie it to the pet skill instead of
having that for free. Decompile the pet functions first to figure out if vacuum already exists
before we take the burden of the calculations."*

It exists. Every step below is **[L]** - read off the listing or the decompile named beside it -
except where marked.

## 1. The chain, from the pet tick to the `0x0205` request

```text
FUN_141ec1e70(pet)                       the pet's "try to pick up" tick
  item = FUN_141ebdad0(pet)              the pet's own Cash-tab CItem
  if item->vtbl+0x378(item)              FUN_1402fda40: bit 1 of the item's u16 at +0x96
                                         (the SECOND `attribute` field, 14030467e) must be
                                         CLEAR - the pick-up on/off toggle. We send 0.
    FUN_14179e990(dropPool, pet, petPos, pet+0x550)
      grade = FUN_14038a5b0(itemInfo, item)
      box   = (-25,-50,25,10)            _DAT_14327c630, the walk-over box
      if FUN_140374c80(grade):           `cmp ecx, 6 ; sete al` - that is the whole test
        box = FUN_14113d940(grade)       one of two server-supplied boxes (section 3)
      box += petPos
      for each drop: if drop (x,y) inside box  ->  FUN_141ec1f20 = send 0x0205
```

`research/msexe-pet-pickup-decision.c` (FUN_14179e990, 12 951 bytes; the box compare is at
`1417a1a7c..1417a1b02`, the send at `1417a1bc6`) and `research/msexe-pet-pickup-rect.c`.

## 2. `wonderGrade`: where the 6 comes from

`FUN_14038a5b0(itemInfo, item)`:

* if the item is type 3 (a pet) and its obfuscated `u16` at `+0xba` (key `+0xbe`) reads
  `> 0`, that is the grade. The pet item decoder `FUN_140304550` stores the packet's `u16`
  read at **`0x140304730`** there - the field after `giantRate`, the second-to-last of the
  fourteen pet reads. The reference server encodes a literal 0 at that position
  (`PetItem.encode`, `encodeShort(0)` after `giantRate`) and carries `wonderGrade` on the
  `Pet` object instead; this client reads it off the item.
* otherwise the pet image's `info/wonderGrade` (`FUN_14038a530`, `L"wonderGrade"`). None of
  the classic eleven declare one.

So the server sets the box per pet, per item: `net::bag::pet_wonder_grade(skills)` writes 6
when the Expanded Auto Move bit is in the mask and 0 otherwise. **That is the tie to the
purchase** - the client never consults the skill bit for the box, only this.

## 3. `0x0198`: the boxes themselves

Case `0x198` of the `0x70..0x19f` dispatcher (`research/msexe-gamestage-cases.txt` line 267,
`FUN_14113d920` -> `FUN_140424370`):

```text
raw[16]  near box  (l, t, r, b as i32)  -> 0x143aca528
raw[16]  far box                        -> 0x143aca538
u32      n
u32 x n  item ids                       -> the tree at 0x143aca548
```

`FUN_14113d940` picks the far box when `FUN_140909e30(DAT_143aa84a0, id) > 0` for any id in
the tree - `FUN_1407b3df0(DAT_143aa84d0, ..., id, 0)`, an item-count lookup keyed by item id
(its switch special-cases `80001415`, `80001418`, `80003062`, `95001004`) **[D]** on what
exactly is counted; the near box is what a client with an empty tree gets, and that is what
this server sends. Both globals are `(0,0,0,0)` until the packet arrives: **a grade-6 pet on a
client that never received `0x0198` sweeps a box of no size and picks up nothing.** So it
rides after every SetField with the keymap and the SP pools.

The near box sent is `(-300,-370,300,220)`: the four `i32` at `0x14327c640`, immediately after
the walk-over box at `0x14327c630`, referenced by nothing in the image (`tools/xref.py`) -
Nexon's own long-range constant, compiled in beside the short one. Six hundred by five
hundred and ninety pixels around the pet.

## 4. What `sweepForDrop` / `longRange` actually are

The pet template loader `FUN_1403e54e0` (`research/msexe-pet-loot.c` line 3191) reads eleven
keys into a *declared* mask: `pickupItem 1, longRange 2, sweepForDrop 4, ignorePickup 8,
pickupAll 0x10, consumeHP 0x20, consumeMP 0x40, autoBuff 0x80, smartPet 0x100, giantPet
0x200, shop 0x400`. That mask feeds the tooltip and the skill list; **nothing in the pickup
chain reads it**. This is why 2026-09-16's WZ trio, verified present in the packed
`Pet_000.wz` on D:\MapleCW (`wz-dump cat`: `pickupItem 1, sweepForDrop 1, longRange 1` on
5000006), changed nothing on screen. `research/pet-vacuum-2026-09-13.md` §3a is superseded.

## 5. What changed

* `net::bag::PET_SKILLS_LEARNED_AT_START` is Item Pouch alone; `store::pets::PET_SKILLS_AT_START`
  is `0b0001`. A pet that had the two Auto Move bits only from the old default loses them on
  read; one that bought a skill has the bit in its row and keeps it.
* `pet_item_with_state` writes `pet_wonder_grade(skills)` at the `0x140304730` field.
* `net::pet::PET_PICKUP_RANGE = 0x0198`, `pet_pickup_range(near, far, ids)`, `PET_VACUUM_BOX`;
  `Session::pet_pickup_range_reply` after both SetField builders.
* Buying Expanded Auto Move (`5190003`) already re-sends the pet item and re-summons the pet
  (`use_pet_skill_item`), so `CPet::Init` re-reads the item and the next tick uses the box.

## 6. The run

Plan step 8, LOOT. Fresh Husky: walks over drops only. Use the Expanded Auto Move item: the
tooltip says *(Learned)*, and a drop up to ~300 px away flies to the pet with no walk. Both
halves are needed and both are measurable separately: no wide pickup with *(Learned)* showing
means the `0x0198` box or the grade did not take (paste `world-ch0.log`'s `PetPickupRange`
line and the pet item bytes 61..63); a wide pickup WITHOUT the skill means the grade leaked.

## 7. The learned-mask bits were scrambled - the refusal that found it

2026-09-16, first client test after §5: the owner double-clicked the Expanded Auto Move item and
got the client's own **"You do not have a pet that can use this skill. ( This skill can only
be equipped on a pet that has the auto-loot function. )"** - a client-side refusal, no packet
sent.

Two findings:

* **`net::bag`'s pet-skill bits were `1 << index`, and the client's are not.** The tooltip
  builder `FUN_1414b89b0` derives each skill's bit from `ebx = 4`: index 0 -> `1`, 2 -> `2`,
  3 -> `4`, 5 -> `8`, 1 -> `0x20`, 4 -> `0x40`, 6 -> `0x80`. The modern `PetSkill` enum is
  identical. So the real mask is `Item Pouch 0x01, Expanded Auto Move 0x02, Auto Move 0x04,
  Ignore Item 0x08, Auto HP 0x20, Auto MP 0x40, Auto Buff 0x80`. Our `1 << index` put Auto HP
  on `0x02` (Expanded's bit), Expanded on `0x04` (Auto Move's), Auto Move on `0x08` (Ignore
  Item's) - which is why a pet that had "learned Auto Move" showed **Ignore Item (Learned)**
  in the tooltip, and every skill item taught the wrong skill. Fixed to the client's values;
  `the_pet_skill_bits_match_the_clients_own_mask` pins them.

* **The client enforces a chain: Expanded Auto Move needs Auto Move learned first.** The pet's
  learned mask (`pet+0x1c`, from the item body's `petSkill`) is what the gate reads - not the
  declared WZ keys, which already carry `sweepForDrop`. A fresh pet has Item Pouch only, so the
  gate refuses Expanded Auto Move until Auto Move (`5190002`, `dropSweep`) is applied. That is
  the "auto-loot function" the message names. So the vacuum is a two-item purchase: Auto Move,
  then Expanded Auto Move, at which point `pet_wonder_grade` sets 6 and `0x0198` widens the box.
  **[D]** on which bit exactly the gate tests (Auto Move 0x04 by elimination: the pet had Item
  Pouch and was still refused); the run confirms the order works with the corrected bits.
