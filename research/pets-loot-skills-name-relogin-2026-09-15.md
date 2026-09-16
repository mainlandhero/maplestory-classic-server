# The pet, six things at once - 2026-09-15, from the two-client run

The owner, with two clients up and the Husky walking on both screens (the `0x0278` fix confirmed):

> Tester2 now sees the pet, no crash. The owner just tried to add the Auto HP, Auto MP and Auto
> Move skill onto Husky, it doesn't work. Husky also currently does not loot items on the
> ground. I also tried to rename Husky into Dummy using the Pet Name Tag. I also no longer see
> "Show Pet Info" as an available button. Wearing the Blue Top Hat on the pet does not show for
> different clients when first worn (upon loading into Cash Shop and then return it does show).
> Pets that were spawned from before does not survive a re-login.

Every one of these left a packet in `world-ch0.log`, and the fixture
`research/fixtures/pet-loot-0205-skills-nametag-0116-hat-0107-unhandled-2026-09-15-world-ch0.log`
is the slice. Labels: **[L]** read off a capture or listing, **[I]** inferred.

## 1. Loot: `0x0205` is the pet's own pick-up request, and it was UNKNOWN

Seven of them, all logged UNKNOWN and unanswered:

```
00000000 00 01f14d20 01000000 69ff d700 002d3101 9667e331 464b4c00     29 bytes
petIdx   u8 tick     u32      x    y    dropId   crc      5000006
```

`FUN_141ec1f20`, the builder, writes `u32, u8, u32, u32, u16, u16, u32, u32, u32, ...`
(`tools/encodes.py`). The `u32` at byte 17 is `0x01312d00` = **20 000 000**, this server's first
drop object id; the next bodies carry `..02`, `..03`, `..04`. So `net::drops::PET_PICK_UP_OBJECT_ID_AT
= 17`, taken from the reference on 2026-09-13 and waiting for a capture, is confirmed **[L]** -
and the opcode is settled: `0x0205`, not one of the player's six siblings. `Session::on_pick_up`
already took a drop by the pet offset with the pet leave type; it was never routed to.

## 2. Skills and the name tag: `0x0116` carries the pet's serial, and a string

The reset-scroll opcode. Four bodies:

```
f98e4e20 0200 70314f00 464b4c00d7000000                  5190000 Auto HP, slot 2
279d4e20 0300 71314f00 464b4c00d7000000                  5190001 Auto MP, slot 3
fda84e20 0400 72314f00 464b4c00d7000000                  5190002 Auto Move, slot 4
393f4f20 0600 50e34e00 464b4c00d7000000 0500 44756d6d79  5170000 Name Tag, slot 6, "Dummy"
tick     slot item     petLockerSN                       str
```

`464b4c00 d7000000` is `pet_serial(215, 5000006)` byte for byte - the serial this server minted
into the pet's Cash item and its `0x0277`, coming back to name the pet the item is for **[L]**.
`on_use_stat_reset_item` parsed ten bytes, matched neither reset scroll, and kept the item.

A skill is a bit in the pet ITEM's `petSkill` mask (`net::bag::pet_item_with_state`, the client's
own bit order from `research/pet-skills-2026-09-13.md`). Learning is: OR the bit into
`store::pets`, use the item up, re-send the pet's Cash item with the new mask - and, the pet
being out, put it away and summon it again **for the owner only**, so `CPet::Init` re-reads the
item. **[I]**: whether the client's auto-HP/auto-loot reads the refreshed item live is
unmeasured; the re-summon makes the question moot at the cost of one summon animation.

The name is stored per pet, cut to the wire's 12 usable bytes, re-sent in the Cash item, and
announced with **`0x027B`** - `FUN_141ec4660`, the `0x27b` arm of the pet sub-dispatcher, reads
one `str` after `charId`/`petIdx` (`tools/reads.py 0x141ec4660 1`) **[L]** - to the owner and the
map.

## 3. The hat: it is in the CHARACTER's look, and nobody re-announced the look

`0x0107` at 02:59:25: `invType 6, slot 1 -> -114`, the Blue Top Hat. The server equipped it
(Deco, worn slot 114) and answered the owner alone. Why Tester2 saw it after a Cash Shop trip:
`look_layout` pairs cash slot 114 with body slot 14, so the hat rides the character's `0x0224`
look at slot 14, and the remote client dresses the pet from that entry - a fresh field entry
sends a fresh `0x0224`. So the live fix is to re-announce the look when the pet-equip slot
changes (`broadcast_look_change`, a second `0x0224`) and put the pet away and back on the map so
its init runs against it. **[I]** on the remote redrawing a user it already has - the same
open question the beauty coupon left; the plan step is the falsifier.

## 4. Re-login: the active pet was session state

`Session::active_pet` was set by the double-click and nothing else. `store::pets` now holds
`(character, pet item) -> name, skills, active`; the summon and the put-away write `active`, and
`restore_active_pet` runs at claim time - before the login `SetField` is built - so the record's
Cash item already says `active = 1`, the first field entry re-sends the `0x0277`, and everyone
already there is handed it as a companion. The reference server's `initPets`, keyed the way the
client itself pairs (by item id through the serial), which is also the one caveat: two of the
same pet on one character share a row.

## 5. `0x0204`, a report

`FUN_141ebe480`: `u32 petIdx, u8, u16 interact index`, sent once right before the chat line
"bad" - `0004` = interact 4, the entry the server then answered with its own `0x0279`. The pet
telling its server it did a trick locally. Filed as a client report, answered with nothing; the
client went on.

## 6. Show Pet Info: still greyed, and not solved here

The button is disabled in `FUN_1414be310` (`research/msexe-userinfo-pet.c` line 225: `if
(local_40 == 0) ... vtbl+0x70(x, 0)`). `local_40` is filled by something between its declaration
and that test that this pass did not find - it is not `GetPet` by inspection of the greps. No
packet is involved for the owner's own window (nothing left the client when it opened), so it is
a local condition on the pet object or the Cash item. **Open.** The right next instrument is a
watch on `FUN_1414be310`'s stores to `[rsp+..]` for `local_40`, or the Ghidra decompile with the
locals renamed - not a guess at which field.

## What changed

* `net::pet`: `CLIENT_PET_PICK_UP = 0x0205`, `CLIENT_PET_ACTION_REPORT = 0x0204`,
  `PET_NAME_CHANGED = 0x027B` + `pet_name_changed`.
* `net::cashitem::UseCashItem` gains `pet_serial` and `text`; `net::bag::pet_item_with_state`
  takes the skill mask; `pet_skill_bit_for_item`, `PET_NAME_TAG`.
* `store::pets` - `character_pets`, `pet_state` / `active_pet` / `set_pet_active` /
  `set_pet_name` / `learn_pet_skill`.
* `session/pet.rs`: `restore_active_pet`, `use_pet_skill_item`, `use_pet_name_tag`,
  `republish_pet_look`, `PET_EQUIP_WORN_SLOT`; the summon/put-away write the store; the field
  pet and the Cash item read name and skills from it. `session/mod.rs` routes `0x0205` to
  `on_pick_up` and restores at claim; `session/cashitem.rs` sends the pet items here;
  `session/inventory.rs` republishes when the pet-equip slot changes.
* Tests: `the_pets_0x0205_loot_request_takes_a_mob_drop`,
  `a_pet_skill_item_sets_the_bit_and_is_used_up`,
  `a_pet_name_tag_renames_the_pet_and_the_name_sticks`,
  `a_pet_that_was_out_is_out_again_after_a_relogin`,
  `a_hat_put_on_the_pet_is_re_announced_to_the_map`, plus `store::pets` and `net::cashitem`.

None of it has been on a screen yet. Plan step 8 says what each outcome means.
