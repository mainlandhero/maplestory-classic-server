# Cash equips did not survive a relog - 2026-09-12

The owner: *"I just noticed that I last had Cobalt wear the entire Ubel outfit, but upon a fresh
login, I do not see those cash items equipped anymore. This is a problem. Cash items should
persist across re-logs."*

Tags: **[L]** read off the client's listing or a database row; **[D]** derived from [L];
**[I]** inferred; **[R]** the reference server (scores 1 of 8 here - a candidate, not a fact).

## 1. It was never a persistence bug

A copy of the live database, `equipment` rows for character 213 (Cobalt): **[L]**

```text
slot  item_id
   5  1040021      ordinary top
   6  1062999      bottom
   7  1072999      shoes
  11  1322999      weapon
 105  1054562      Ubel's Overall     <- cash, worn slot = 100 + base
 107  1074238      Ubel's Shoes
 108  1082878      Ubel's Gloves
 111  1703726      Ubel's Cane
```

The store keeps a cash equip at `100 + base slot` (`Store::equip_from_tab`, Deco). So the
question was purely which packet lost them between the row and the screen.

## 2. Two packets, two different drops

### 2.1 The compact look - `FUN_1402ee8d0`

`avatar_look` wrote `(slot u8, itemId u32)` straight from the rows: `105, 1054562`. The
reader's loop guard is `(u8)(slot - 1) < 0x1f` at `0x1402ee9c0` - the item is read and
**discarded** for anything outside 1..=31 (`research/avatar-look-loops.md` §1). **[L]**
Five bytes on the wire, no garment. This look is what the character-select screen and
`UserEnterField` (other players) read.

The look has **two** `0xFF`-terminated maps, both indexed 1..=31: the first lands at
`look + 0x39 + slot*4`, the second at `look + 0xb9 + slot*4`. The first map's index 0
(`+0x39`) is the hair, written by the `u32` just before the loop - so the first map is the
drawn one. **[D]** The reference server names them `hairEquips` / `unseenEquips` and builds
`hairEquips` from every worn item with the cash one winning the slot. **[R]**

Fix: `net::opcode::look_maps` - a cash item at `100 + s` goes into the first map at `s`; the
ordinary item at `s`, if any, goes into the second map at `s`; a slot without a cash item is
drawn as before. No cash worn -> byte-identical to what has been through this reader since
2026-08-19.

### 2.2 The SetField record - `FUN_140304b20`

The record's inline equipped loop stores at `charData + 0x1a8 + slot*0x10` for 1..=31
only (`LEA EAX,[RCX-1] / CMP EAX,0x1e / JA` at `0x140306229`) **[L]**, and `equipped_block`
already dropped the 105s rather than pay for them. Nothing else in the record we sent carried
them, so after a login the client had the worn cash items in **no list at all**: not drawn
(the local player is dressed from its own worn lists - the SetField record carries no compact
look, which the new world test had to learn), not in the Deco equip window, and not removable,
while the server still held them worn.

The client has a **second equipped block** for exactly this, behind `presence[44]`
(`research/charrecord-presence-map.md` row 44; `research/bag-lists.md` §6 priced it and
nobody had sent it). Read off `python tools/dis_at.py 0x14030661c 0x220`: **[L]**

```text
140306640  gate: scan presence bytes 44..99 for any non-zero (CMP ECX,0x64 / JB)
14030665c  u8   flagA                       TEST AL / SETNE R14B - non-zero skips b6f0(6)
140306667  zero 32 slots at charData + 0x3a8            (mirror of 0x1a8 for the first block)
14030668c  u16  slot  (0 ends the list)
1403066ae  item        FUN_1403095e0 - the same pooled item factory as the first block
1403066b8  LEA EAX,[RCX-1] / CMP EAX,0x1e / JA discard  -> BASE slot 1..=31
14030675e  store at charData + 0x3a8 + slot*0x10          (or keyed -100-slot, 0x140306705,
                                                          when [rbp+0x3118] is set)
140306815  FUN_14030b6f0(closure, 6)   the Deco BAG list - u16 0 here
140306826  FUN_14030b9e0(6)            indices 0 and 1 only (FUN_140255790): two u16 0
140306830  rejoin
```

So the block is `u8 0`, `(u16 base, item)*`, `u16 0`, `u16 0`, `u16 0`, `u16 0` - nine
bytes plus entries - and it sits after the presence[2] region and before the skill gate
(`0x140306d28`) and both quest gates. `net::opcode::cash_equipped_block` builds it and
`character_record_for_set_field_with` appends it, setting byte 44, only when a slot in
101..=131 is worn. The Deco bag list is left empty because the Deco tab is restored by
`0x0070` mode 5 on field entry like the other bags; a second copy here would double it.

### 2.3 The weapon cover is not a weapon - 2026-09-12, after the first launch

The owner: *"I do see the cash equips on my character upon login, but for Ubel's weapon, I do not
see the proper rendering of it on character select. (It does show up fine in the game world)"*.

The field dresses from the worn lists the record carried (section 2.2), so it was right; the
select screen reads the compact look, and 2.1's rule had put the cover `1703726` (family 170)
in slot 11 of the drawn map with the real weapon `1322999` demoted to the covered map. A cover
has no weapon type and so no stance. The look's four `u32`s after the maps land at `+0x2d`,
`+0x31`, `+0x35`, `+0x1c1` **[L]**; the reference encodes them as `weaponStickerId`,
`weaponId`, `subWeaponId`, `0` **[R]**, and the classic client ships covers of its own
(`Character/Weapon/01702001.img`), so the field is exercised by the real thing. `look_layout`
now keeps the weapon in slot 11 of the drawn map and puts the cover in the first `u32`. Only
the sticker is filled; the other three stay 0, as they were on every screen so far.

## 3. What is measured and what is not

* Rows, reader guards, block layout, gate: **[L]**, above.
* Which look map is drawn: **[D]** from the hair sitting at index 0 of the first, plus **[R]**.
  If character select comes up naked while the field is dressed, this is the reading that is
  wrong - swap the maps.
* Whether the client's Deco equip window reads `charData + 0x3a8`: **[I]**. The block is the
  client's own storage for these slots, and there is nowhere else the record puts them; the
  test plan's step TO(g) names the screen outcome that would refute it.
* Nothing on a screen yet. The record has no resync point, so a wrong block is a fault at
  login, not a cosmetic miss - TO(g) says to stop and paste `client-exit.log` if so.

## 4. Tests

* `net::opcode::tests::a_cash_equip_is_drawn_at_its_base_slot_and_the_item_it_covers_goes_to_the_second_map`
* `net::opcode::tests::the_cash_equipped_block_uses_base_slots_and_carries_four_terminators`
* `net::opcode::tests::a_character_wearing_cash_opens_presence_44_and_one_without_sends_the_old_record`
* `world::session::tests::a_cash_equip_worn_at_105_is_drawn_and_listed_on_the_next_field_entry`
