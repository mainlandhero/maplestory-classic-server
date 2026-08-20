# NPC shops: resolving the transcription, and the item table behind the two rules

Goal F and goal G. **No disassembly was needed for any of this** and none was done - the
Ghidra project was not opened. Everything below is `wz-dump` output, the generated name
table, and `data/shops.txt` itself.

Labels as the house uses them: **[L]** measured, **[D]** derived from measurements, **[I]**
inferred or adopted from outside this client.

## The headline

```text
before   804 resolved, 121 UNRESOLVED, 7 ambiguous
after    925 resolved,   0 unresolved, 0 ambiguous      (39 shops, 925 rows, 577 distinct ids)
```

Every one of the 121 was a **spelling difference, not a wrong item** - the transcription
named an item this client has, every time. The 7 ambiguous were all the same shape: one name
carried by a male item and a female item, with nothing in the row to pick between them.

New generated table: `gm-handbook/itemdata.txt`, **2785 items**, from `tools/dump_itemdata.py`.

| | |
|---|---:|
| items carrying `info/quest` - **may not be sold** | **119** |
| items carrying `info/tradeBlock` - **may not be stored** | **39** |
| items carrying a non-zero `info/price` | 1838 |
| items carrying `info/slotMax` | 290 |

Both of the owner's restrictions are therefore table lookups over the client's own data, with
nothing invented and nothing from a fan site. **[L]**

## 1. The instrument, checked before any of its answers were used

The 121 names were resolved with a **loose key**: lowercase, punctuation to space, every
token singularised. Two names sharing a loose key differ only in case, punctuation or plural
- exactly the corroborated failure classes and nothing wider. That is a fuzzy matcher, and a
fuzzy matcher that ships a wrong item id is the silent wrong answer this project keeps
paying for, so it was checked three ways before a single answer was believed. **[L]**

| check | result |
|---|---|
| **Control.** The 797 names that already resolved *exactly* must re-resolve to the same id through the loose key | **797 identical, 0 moved** |
| **Collisions.** How many loose keys cover more than one distinct client name | **1 in the whole 4302-name table**, and no shop name lands in it |
| **Edit class.** Every proposed correction must be one of the three known classes | 99 distinct names: 89 pure gender-suffix, 7 plural, 2 hyphen, 1 both - **and all 10 non-suffix ones are listed below by hand** |

The one collision is itself a finding, and it is why the `(M)`/`(F)` handling is shaped the
way it is: the client's own `String.wz` ships **`Natalie's Fashion Box (M)` / `(F)`** at
2430001 / 2430002, with the suffix baked into the item name. A resolver that peeled the tag
off first would look up a name that does not exist and report a real item as missing. Both
`check_shops.py` and `world::shops` therefore look the **whole name up first**. **[L]**

A fourth check, because a name resolving is not the same as it resolving to something
sensible: `items.txt` also carries **1272 hair, 468 face and 7 skin names**, and a shop name
matching one of those would resolve cleanly to a haircut. **No resolved id is below
1002000** - the lowest is Brown Skullcap, a cap - so nothing in these 39 shops resolved to an
appearance item. **[L]**

## 2. The three failure classes, and every correction made

### 2a. `(M)` / `(F)` - 90 rows, and the tag is load-bearing

MapleStory equips encode gender in the id: `(id / 1000) % 10` is 0 male, 1 female, 2+ unisex.
That is the family's usual convention, so it was **measured here rather than assumed**:

> Across the 90 tagged rows the owner transcribed, there is **not one** whose tag disagrees with
> the gender its id implies. **[L]**

That is a 90-row control, not a spot check, and `python tools/check_shops.py --gender-audit`
re-runs it and prints every row.

The tag is kept in the file rather than stripped, because for **three names it is the only
thing that disambiguates**: Sam sells `Green Bennis Chainmail (M)` (1040033) and `Green
Bennis Chainmail (F)` (1041040) as two rows at the same price. Stripping the suffix to
"correct" the file was measured as a counterfactual rather than argued about: it gives
**925 resolved, 0 unresolved, 13 ambiguous** - trading every unresolved row for an ambiguous
one and *doubling* the ambiguity that was there to start with. That is the same failure
wearing a better headline number.

**What is NOT established: when the live UI adds the suffix.** Three explanations were tried
and all three are contradicted by rows in the file itself:

* *"the item is gender-locked"* - `Green Hunter's Pants` is male-only (1060034) and carries no
  tag, while `Archer Pants (M)` is **unisex** (1062000) and carries one.
* *"two rows in this shop share a name"* - `Green Archer Top (M)` has no female twin anywhere
  in the client.
* *"the whole shop is tagged or none of it is"* - Sam's male tops all carry `(M)` while their
  male trousers block carries it on the first row only.

So a tag is treated as a **hint that narrows the candidates**, never as an assertion about
the item, and a tag on an unambiguous name is accepted and ignored. This is recorded as open
rather than smoothed over; it costs nothing today, and it would matter if a future shop were
transcribed and its tags were expected to mean something.

### 2b. Hyphen where the client uses a space - 4 names, 2 of them also plural

    Red-Striped top          ->  1041004  Red Striped Top
    Red-Striped T-Shirt (F)  ->  1041015  Red Striped T-Shirt
    Subi Throwing-Star       ->  2070000  Subi Throwing Stars      (also plural)
    Subi Throwing-Stars      ->  2070000  Subi Throwing Stars

Both spellings of the throwing star appear in the file - four shops write it singular and four
write it plural - and both are the same item.

### 2c. Singular where the client uses plural - 6 names

    Blue Jeans Shorts   ->  1060001  Blue Jean Shorts     (the client singularises "Jean")
    Arrow for Bow       ->  2060000  Arrows for Bows      (the client pluralises BOTH nouns)
    Arrow for Crossbow  ->  2061000  Arrows for Crossbows
    Red Pao Bottom      ->  1060020  Red Pao Bottoms
    Blue Pao Bottom     ->  1060021  Blue Pao Bottoms
    Black Pao Bottom    ->  1060022  Black Pao Bottoms

`Blue Jeans Shorts` is the one worth naming separately, because it is the only correction
that goes the *other* way - the transcription is plural and the client is singular. It is
also the only one where an English-plural rule and the client disagree, which is why the
matcher singularises both sides rather than pluralising one.

Every correction is in `data/shops.txt` with the owner's original wording in the header block.
**No price was changed.**

## 3. The 7 ambiguous rows - settled from position, and labelled [D] not [L]

Six in Don Hwang's shop (`Red`/`Blue`/`Black Cloth Vest` and the matching `Cloth Pants`) and
one in Nuri the Fairy's (`Green Bennis Chainmail`). Each name is carried by a male item and a
female item at the same tier, and nothing in the row picks between them.

**Price does not discriminate** - the first instrument tried and it failed cleanly. `Coat
01040010` and `01041011` have *identical* `info` nodes: same `price` 1000, same `reqLevel`
10, same `reqLUK`, same `incPDD` 24. Recorded because it is the check someone would otherwise
repeat. **[L]**

**Position does.** Resolving every row of both shops to its id shows each list is laid out
**male block then female block, ascending price within each**:

```text
Don Hwang, tops              Don Hwang, trousers          Nuri, archer section
  Cloth Vest  2000  ???        Cloth Pants  1600  ???       Bennis Chainmail 7500 ???
  Nightshift  3000  1040019    Nightshift   2400  1060014   Hunter's Armor  12000 1040044 [M]
  Pao         6000  1040027    Pao Bottoms  4800  1060020   Huntress Armor  12000 1041052 [F]
  Sneak       7500  1040037    Sneak Pants  6000  1060026   Bennis Chain Pants 6000 1062004
  Stealer    12000  1040047    Stealer Pnts 9600  1060040   Hunter's Pants   9600 1060036 [M]
  ---- female block starts ----  ---- female block ----     Huntress Pants   9600 1061047 [F]
  Nightshift  3000  1041023    Nightshift   2400  1061019
  Qi Pao      6000  1041034    Qi Pao Pants 4800  1061029
```

Every neighbour above the female block is male, so the ambiguous row sits at the **head of a
male run**, and it is tagged `(M)`. **[D]** - derived from list order, not measured.

**And the same evidence says the transcription is short by seven rows.** In both shops, every
tier has a male row *and* a female row - except exactly the tier whose two items share a
name, which has one row. That is not a coincidence about those tiers; it is the same set. The
likely reading is that the owner wrote the duplicate-looking pair down once. So:

> **For the owner.** These seven are probably sold in both genders and the file has only the male
> one: `Red`/`Blue`/`Black Cloth Vest (F)` = 1041011/1041012/1041013 and `Cloth Pants (F)` =
> 1061009/1061010/1061011 at Don Hwang, and `Green Bennis Chainmail (F)` = 1041040 at Nuri,
> at the same prices as the male rows already in the file. **They have not been added** -
> adding shop rows nobody transcribed is inventing content, which is the one thing this file
> exists to avoid. One look at either shop settles it.

The alternative reading - that those shops genuinely sell only the male item at that tier -
is possible and would leave the file exactly as it is now.

## 4. `gm-handbook/itemdata.txt`

`tools/dump_itemdata.py`, following `tools/dump_equips.py` exactly: generated, gitignored,
regenerable, never hand-edited.

Two archive shapes, one walker:

* `Item/<cat>/<cat>_000.wz` - **grouped**, one image per 4-digit prefix whose children are the
  8-digit item ids. Consume 331, Etc 359, Install 23, Cash 300.
* one image per item, the id being the stem - `Item/Pet` 12, and **every equip**, which lives
  under `Character/<slot>` and not under `Item.wz` at all: 1760 across 12 slots.

**`Item/Special` contributes 0 rows, and that is the data rather than the tool.** Its items
carry `icon`, `iconRaw` and `name` at the image root and **no `info` node at all** - checked
on `0900.img` and `0910.img`. There is nothing to read. **[L]**

### Coverage, and the gap fully accounted for

`items.txt` names **4302** ids; `itemdata.txt` has **2785** rows. The 1747 named-but-absent
break down as **1272 hair + 468 face + 7 skin** - exactly the three `Character.wz`
directories both generators exclude on purpose, as appearance rather than equipment.
1272+468+7 = 1747, so the gap is closed with no residue. **[L]** The other direction, 230 ids
with an `info` node and no `String.wz` name, is ordinary unnamed content (`5021xxx`,
`4006002`, `1392000`).

**Every one of the 577 distinct ids the 39 shops sell has a row.** **[L]**

### price is the SELL price and the buy price is not derivable

The ratio holds and then does not, which is what makes it not a rule:

| | WZ `price` | shop buy | ratio |
|---|---:|---:|---:|
| Red Potion 2000000 | 5 | 50 | 10.00 |
| Orange Potion | 15 | 150 | 10.00 |
| Apple / Egg / Orange | 2 / 3 / 5 | 20 / 30 / 50 | 10.00 |
| **Pet Food** | **15** | **35** | **2.33** |

Nine of nine consumables at exactly 10.00x, and then one at 2.33x. **[L]** That is why
`data/shops.txt` exists at all.

### One measured artefact with no interpretation

`Item/Item_000.wz` has an image `ItemSellPriceStandard.img` whose single node `400` maps
`n -> 2n` for n in 1..250, with **zero exceptions**. **[L]** Key `400` matches the ETC
category prefix. What the client does with it has **not** been read - no disassembly was done
here - so it is recorded as an artefact, not as a pricing rule, and it was **not** used for
anything above. Worth a look before anyone hand-authors ETC sell prices.

## 5. `crates/world/src/shops.rs`

Resolves names to ids at load time and **reports every drop**. `ShopTable::problems` is a
`Vec<String>` the server prints to **stdout** at startup:

```text
maplecw-world: 39 NPC shops, 925 item rows, 0 unresolved
```

Stdout rather than stderr, deliberately: the mob default that "silently did nothing" cost a
whole client launch, and its warning went to `world.log.err`, which nobody opens during a run.

Six ways a row can fail to load, each with its own message: unknown name, ambiguous name, a
`(M)`/`(F)` tag no candidate satisfies, an unparseable price, a name with no `itemdata.txt`
row, and an unrecognised citizenship rank. A quest item being stocked is reported but **not**
dropped - it is a transcription worth a look, not a rule violation, and as it happens no shop
in the file stocks one.

### The rank gate fails closed

Per the mapping now settled in goal H, a rank tag is a grade name and `+` means that grade or
higher, so `Town Resident+` is `min_grade = Some(5)`. **An unrecognised rank drops the row.**
Loading it ungated would turn a Citizen-of-Honor scroll into one anybody can buy, and nothing
downstream could ever detect that - a shop row with no gate looks exactly like a shop row
that is meant to have no gate. There is a test named after this rule.

40 rows across the file are gated, over 9 of the 10 grades (no row requires Traveler).

The grade *names* are corroborated by being exactly the tags in the transcription; the
grade-to-level thresholds are **[I]** from a fan site and are deliberately **not** encoded in
this module - it carries the grade a row requires and nothing else. Discounts are goal H's
and are not applied to these prices, which are the undiscounted list prices.

## 6. What is still open

1. **The 7 derived `(M)` tags, and the 7 rows that may be missing** - §3. **[D]**, one look at
   either shop settles both.
2. **NPC name -> template id.** `shops.txt` names the NPC the way the UI does; nothing maps
   that onto `gm-handbook/npcstrings.txt`'s templates. **A shop cannot be opened without it**,
   and it belongs with the shop packet rather than here.
3. **Map label -> map id.** `Victoria Road: Perion Weapon Store` is a street label, not an id.
   Not needed to serve a shop, since the NPC identifies it.
4. **When the live UI adds `(M)`/`(F)`** - §2a. Three explanations tried, all contradicted.
5. **The shop dialog packet**, in both directions, and the buy/sell request. Not attempted;
   out of scope by instruction.
6. **`ItemSellPriceStandard.img`** - §4.
7. **The grade-to-level thresholds** remain **[I]**.

Nothing in the 121 names or the 7 ambiguous rows is left unsettled. The residue is the seven
items above, and **none of them is a name that failed to resolve** - the only one that could
change a shop's contents is (1), and it would add rows rather than correct any.
