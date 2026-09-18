# The hair-hats would not go on: `islot HrCp` - 2026-09-12

The owner: *"The Aura, Lugner and Linie hair does not wear when double clicked on."* These are the
three "Hair (Hat)" caps of the Signature Style sets: 1006910 (Aura), 1006911 (Linie), 1006912
(Lugner) - cash caps that stand in for a hairstyle.

Tags: **[L]** a log line or a WZ leaf; **[I]** inferred.

## 1. No packet

`grep 1006910|1006911|1006912` over `world.log` and `previous-runs/` finds only the field-entry
restores (`0x0070` mode 5 into the Deco tab) - **not one `0x0107`** for any of them, across every
run they were in a bag. **[L]** The client refused the double-click on its own. Same shape as
the Collection box (`research/collection-box-id-2026-09-12.md`): when the client sends nothing,
look at the item's own data, not at the server.

## 2. The type string

`info` of the three, from the modern client (`backport/signature-style/wz/Cap/<id>/prop.json`):

```text
islot HrCp   vslot CpH1H2H3H4H5H6HfHbHxHs   reqJob 0  reqLevel 0  cash 1  collabo 1  noPrism 1
```

Every classic cash cap in `Cap_000.wz.bak` (01007063..67 sampled): `islot Cp`, `vslot CpH1H5`.
**[L]** `HrCp` is the modern two-slot type: the item occupies the hair slot AND the cap slot,
which is how a "hair hat" replaces the hairstyle in that client. The classic client has no
such slot.

Whole-string search of the image: `HrCp` 0 hits, and `MaPn` (the overalls' type) also 0 hits
- yet the overalls equip. So the client does not match the whole string; it reads two-letter
tokens (`Ma`,`Pn` / `Hr`,`Cp`), and the first token of a hair-hat is `Hr`: hair, which is not
something a bag item can be put on. The double-click resolved no destination slot and sent
nothing. **[I]** for the token reading; the fix is the test.

## 3. Installed

`tools/backport_install.py` step 1d: for every set cap whose `islot` starts with `Hr`, one
`patch` line `info/islot  str  Cp`. Built from the pristine `.bak`, installed, read back:
all three say `Cp`. `vslot` is untouched - its `H2 H3 H4 H6 Hf Hb Hx Hs` tokens may be
unknown to this client and merely ignored (`CpH1H5` is the classic form); if the player's own
hair shows through the hat, that is the next variant, not this one.

## 4. Not established

* Whether the client hides the hair under the hat with the modern `vslot` (see above).
* The token reading itself - a listing of the islot parser would settle it; the screen will
  settle whether it matters.


## 5. `islot Cp` was not the gate - 2026-09-12, second launch

The owner: *"Nope, equipping the hair caps still does not work."* world.log for that run: two
`0x0107`s, both the cape (Deco slot 15 to -109 and back); **none for a hat**. The islot reading
in section 2 is refuted as the gate - it may still be a gate, but not the first one.

What the static side gave before it stopped paying:

* `FUN_1417dd7e0` is the double-click equip path: it checks the tab (`cmp eax, 6` at
  `0x1417de178` for Deco), calls the slot validator `FUN_140253980` at `0x1417de24c`, the
  worn-slot lookup `FUN_140397680`, the requirement check `FUN_140397db0` at `0x1417de965`
  (level, STR/DEX/INT/LUK, job - it is handed the character's stat fields from `+0x13a..+0x172`),
  and ends in the `0x0107` builder `FUN_142cc5b00` at `0x1417dea90`. **[L]**
* `FUN_140253980` is a category jump table on `itemId / 10000 - 100` with a gender check in
  front (`(id/1000) % 10`: 0 male, 1 female, else any); category 100 maps to body part 1. A hat
  passes it. **[L]**
* The message boxes that function can raise (`0x4e8..0x4ed`, `0xb63`) are the ring and
  bonus-EXP limits. Not this. **[L]**
* A dozen further exits (`je 0x1417dd910`) sit between entry and the send, and reading them
  all is more expensive than one instrumented double-click.

So the next launch measures: watches on `1417dd7e0`, `140397db0` and `142cc5b00`. Which of the
three fire, in that order, names the gate's neighbourhood; plan step TO(i) has the readings.


## 6. The gate: the id's fourth digit is the gender, and 6 is female - 2026-09-18

Sections 2-5 were looking in the wrong function. `FUN_1417dd7e0` is the **drag from a worn
slot** path - its one caller `FUN_142382330` refuses unless the source slot is negative. The
**double-click** on a bag item is `FUN_141784fa0`, which asks `FUN_142d44b20(user, item, 0)`
for the item's body part and calls the equip function `FUN_1417da2a0(tab, slot, part)` **only
when that is non-zero**. Zero is silent: no call, no packet, no box. **[L]**

`FUN_142d44b20` fills a list of candidate parts through `FUN_140253f90` -> `FUN_1402543e0`, a
switch on `id / 10000` (100 -> part 1, a cap). Before the switch:

```c
if (FUN_1402531f0(id) == 0 && FUN_140416760(id) == 0 && FUN_140416820(id) == 0
    && (g = FUN_140253130(id), charGender != 2 && g != 2) && g != charGender)
    return;                                   // empty list -> part 0 -> nothing happens
```

`FUN_140253130` is the gender-from-id rule, and this build's table is longer than the one
everybody remembers:

```c
d = (id / 1000) % 10;
if (d == 0) return 0;          // male
if (d != 1) {
    if (d == 5) return 0;      // male
    if (d != 6) return 2;      // unisex
}
return 1;                      // 1 and 6: female
```

The three hats are 100**6**910..912. Every test character is male (`characters.gender = 0`,
all five rows). The cape (110**3**918) and the coats (105**4**56x) are unisex by the same rule,
which is why they equipped all along. The three predicates in front of the digit are
hard-coded id ranges (1340000.., 1350000.., 1090000 shields, 1669008..1679007 -> part 0x1c),
not WZ keys - there is nothing in the data that makes a 1006xxx cap unisex in this build.

**Fix, per the owner** (*"Nexon has made these items unisex ... fix it in the WZ data instead of
patching the client"*): the property image is copied under **1007910..1007912** (digit 7 is
unisex; 316 classic equips carry it; the numbers are free), the `Eqp.img` string moves with
it, the `islot Cp` patch targets the new image, and the `_Canvas` image keeps its old name
because the property image reaches its frames by explicit outlink path
(`Character/Cap/_Canvas/01006910.img/...`) - the same reason the Collection box's canvas
stayed at 0522. `world::signaturestyle::HAIR_HAT_IDS` and the sets say the new numbers;
`store::ITEM_ID_RENAMES` renumbers hats already in a bag, worn, in the locker or in storage on
the first start. `every_set_equip_is_unisex_under_the_clients_digit_rule` pins the rule so no
set ships a gendered id again.

Installed 2026-09-18 01:53 and read back: `01007910..12.img` present with `islot Cp`, the old
names gone, canvases under the old names, `items.txt` naming all three. Unverified on screen;
the plan's step (i) has the readings. What the `vslot` modern tokens do to the wearer's own
hair is still the open variant from section 3.
