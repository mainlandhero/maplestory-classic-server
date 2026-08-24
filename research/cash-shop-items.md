# The cash shop's item table, read out of the client

2026-08-23. `research/cash-shop.md` ends with the client sending `0x00D5` and a server that
answers "not available". This is the other half: **what a real answer would have to be stocked
with**, taken from the client's own data rather than from a website.

Tags: **[L]** read out of a file or a listing, **[D]** derived from something read,
**[I]** inferred or a candidate from a third party.

Regenerate everything below with:

```
python C:\MapleCW\tools\dump_commodity.py
```

run from `C:\MapleCW`. It writes `gm-handbook/commodity.txt`,
`cashshopcategory.txt`, `cashpackage.txt` and `commoditylimit.txt` - generated, gitignored,
never hand-edited.

---

## 0. The short version

**159 sale rows, 146 distinct item ids, 138 on sale and 21 switched off, priced 100, 700 or
1000 NX.** `[L]` Every price in the shop is one of three numbers, and 130 of the 138 live rows
are 100 NX flat.

The list is authored per **sale row**, not per item: an SN is the thing bought, and the same
item id appears on up to three rows at different counts, periods and prices. A server that
keys its shop on item id will get the megaphone bundles wrong.

Nothing here is a protocol finding. The wire format of a cash-shop packet is not in Etc.wz and
is not in this file. Nothing here authenticates anything either - the game socket still
carries no credentials.

---

## 1. Where it lives

Five images in `client-patched/Data/Etc/Etc_000.wz`, all present and all parsing: `[L]`

| image | size | what it is |
|---|---|---|
| `Commodity.img` | 21 712 | the sale list - 159 nodes |
| `CashShopCategory.img` | 1 729 | the tabs and sub-tabs |
| `CashPackage.img` | 163 | bundles |
| `CommodityLimit.img` | 566 | country / policy blocks |
| `CommodityTradeBlockBuyList.img` | 121 | trade-block buy list |

`Commodity%d%02d.img` and `Etc/OldCommodity%d%02d.img` are also string literals in the client
(`0x14327857f`, `0x143278578`) - versioned commodity images the service could ship. **Neither
exists in this archive**; the tree at depth 1 lists 67 images, of which exactly three match
`Commodity*` and they are the three tabled above. `[L]`

---

## 2. The Commodity row, field by field

### 2.1 The field list is the client's, not mine

`FUN_140229d10` (10 416 bytes, `0x140229d10 .. 0x14022c5c0`) walks a Commodity node property
by property. Its property names come two ways - some as direct `lea` on a UTF-16 literal, some
through a static string object in `.data` - and **resolving both** gives the read order below.
`[L]`

That "both" matters, and it is this project's standing rule wearing a new face.
`tools/xref.py --string OnSale` and `tools/dataref.py 0x1432b24e0` each return **zero code
references**, and both are correct: nothing in an executable section points at that string.
The pointer to it sits in `.data` at `0x143a48768`, in a run of such slots -
`0x143a48708` "SN", `0x143a48710` "ItemId", … `0x143a48730` "ReqPOP" - and the reader loads
*that*, not the literal. The positive control is one line away and passes -
`xref.py --string originalPrice` finds its one `lea` at `0x14022ac62` - so the zeros are a
property of the instrument, not of the client. **A `lea`-scanner cannot see a literal reached
through a pointer**, exactly as the `mob+0x42c` write-scan could not see a store through a
`lea`'d pointer.

Read order, by address: `[L]`

```
140229eb7 SN            14022a64a Priority       14022ac62 originalPrice   14022b39f MonthlyLimited
140229ef1 ItemId        14022a6b5 Period         14022ace2 discount        14022b3eb gameWorld
14022a18a Count         14022a720 ReqPOP         14022ae2c Refundable      14022b5e2 possibleTrading
14022a251 Price         14022a78b ReqLEV         14022aeb8 bombSale        14022b674 exchangeableOnce
14022a318 mileageRate   14022a7f6 MaplePoint     14022af2e forcedCategory  14022b6ab expireOnNonPremiumLogin
14022a3f2 onlyMileage   14022a861 Meso           14022af9f forcedSubCat…   14022b71e expireOnLogout
14022a4bc token         14022a8cc Premium        14022b0c5 LimitMax        14022b7fb couponType
14022a583 Bonus         14022a93f Gender         14022b136 LimitQuestID    14022b87d blockRewardTrade
                        14022a9ad OnSale         14022b1a4 CheckQuest      14022b8fe ShowDiscount
                        14022aa20 Class          14022b212 favorType       14022bba9 SubstituteSN
                        14022aa8e Limit          14022b2b8 WSLimitMax      14022bbd5 ReqLevType
                        14022aaf9 PbCash         14022b326 WSLimitRecordID 14022bc07 Country
                        14022ab67 PbPoint
                        14022abd5 PbGift
```

**46 distinct properties read. 20 distinct properties present in this build's data. 18 in
both.** `[D]`

### 2.2 The 20 fields that are actually in the data

| WZ node | rows | values here | meaning | confidence |
|---|---|---|---|---|
| `SN` | 159 | 92000000 .. 160300005, all unique | the sale row's serial. **This is the shop's key**, and the client's own help text calls it "CommoditySN corresponding to the cash item ID" (`0x143425161`) | **[L]** |
| `ItemId` | 159 | 146 distinct | the item handed over | **[L]** |
| `Count` | 159 | 1 (149), 3, 5, 10, 11 | how many | **[L]** |
| `Price` | 159 | 0, 100, 700, 1000 | NX | **[L]** |
| `originalPrice` | 159 | 0, 100, 700, 1100 | the pre-discount price; equals `Price` except on the 7 discount rows | **[D]** |
| `discount` | 7 | always 1 | present only where `Price < originalPrice`. All 7 are 1000 vs 1100 | **[D]** |
| `Period` | 159 | 0 (49), 1, 7 (14), 90 (95) | **DAYS**, not minutes and not hours: 90 on cosmetics, 7 on trials, 1 on the cheap store-permit row, and meowdb names the same field `periodDays` with the same value 90. 0 = no expiry | **[D]**, corroborated **[I]** |
| `Gender` | 159 | 2 (157), 1 (1), 0 (1) | 0 male, 1 female, 2 either. The two non-2 rows are the swimsuits - `140300003` Blue Swimming Suit gender 1, `140300004` Blue Tiny Swim Shorts gender 0 - which is what makes 0/1 readable at all | **[D]** |
| `OnSale` | 159 | 1 (138), 0 (21) | switched on | **[L]** |
| `Priority` | 159 | always 100 | display order. Never varies here, so nothing in this data can show which direction sorts first | **[I]** |
| `ReqLEV` / `ReqPOP` | 159 | always 0 | required level / fame. Never varies here | **[I]** |
| `Bonus` | 159 | always 0 | never varies here | **[I]** |
| `Refundable` | 159 | always 0 | never varies here | **[I]** |
| `PbCash` `PbPoint` `PbGift` | 41 | always 0 | "purchase by" cash / point / gift. Never varies here | **[I]** |
| `Class` | 13 | 0 and 2 | read at `0x14022aa20`. **Meaning not established.** It is not "is in the Main tab" - I checked: `130000003` carries a Class and is not in Main, and four Main SNs carry none | **unresolved** |
| `WebShop` `IsGift` | 159 | always 0 | **neither string exists anywhere in the client.** `xref.py` reports 0 copies of each while finding 1 copy of `SubstituteSN` and 1 of `forcedCategory` as controls. Nothing reads them | **[L]** that nothing reads them |

The other 28 properties the client reads are absent from this build's data. Two are worth
naming because they are pricing paths a server would otherwise not expect: **`Meso`** (a
commodity priced in mesos) and **`MaplePoint` / `mileageRate` / `onlyMileage`** (a second
currency). Neither is exercised by any row here. `[L]`

---

## 3. The tab layout, and the category arithmetic

### 3.1 What CashShopCategory.img says

Seven tabs. `[L]` A sub-tab holds either a list of **scope** numbers or a list of explicit
**commoditySN**s:

```
0 Main       0 Main         sn x16
1 Seasonal   0 Packages     scope 700 701 702
2 Special    (no sub-tabs at all)
3 Fashion    0 Weapons 400 | 1 Hats 401 | 2 Outfits 403 | 3 Tops 404 | 4 Bottoms 405
             5 Capes 402   | 6 Shoes 406 | 7 Gloves 407 | 8 Accessories 408 409
             9 Rings 411   | 10 Effects 410
4 Beauty     0 Hairstyles 500 | 1 Faces 501 | 2 Misc 502 | 3 Expressions 503
5 Pets       0 Pets 600 | 1 Pet Equip 601 | 2 Pet Care 603
6 Utility    0 Convenience 300 301 305 | 1 Social 302 303
```

### 3.2 A scope is a function of the SN

Nothing in the WZ states the rule. **[D]**, and it is derived rather than guessed:

```
category = SN / 10000000 - 10
sub      = (SN / 100000) % 100
scope    = category * 100 + sub          i.e. simply  SN / 100000 - 1000
```

The check is that **every non-empty scope lands on exactly the item class its tab's own label
names**, across all 21 of them: `[L]`

```
  scope 400 Weapons  -> 170xxxx cash weapons     scope 300 Convenience -> 503xxxx 514xxxx
  scope 401 Hats     -> 100xxxx                  scope 301 Convenience -> 505xxxx
  scope 402 Capes    -> 110xxxx                  scope 302 Social      -> 507xxxx megaphones
  scope 403 Outfits  -> 105xxxx overalls         scope 303 Social      -> 512xxxx effects
  scope 404 Tops     -> 104xxxx                  scope 305 Convenience -> 568xxxx slot coupons
  scope 405 Bottoms  -> 106xxxx                  scope 500/501/502     -> 515xxxx coupons
  scope 406 Shoes    -> 107xxxx                  scope 503 Expressions -> 516xxxx
  scope 408 Accessor -> 101xxxx face             scope 600 Pets        -> 500xxxx
  scope 409 Accessor -> 102xxxx eye              scope 601 Pet Equip   -> 180xxxx
  scope 411 Rings    -> 111xxxx                  scope 603 Pet Care    -> 517/518/519xxxx
```

Twenty-one independent agreements, no exceptions. The client also carries **`forcedCategory`
and `forcedSubCategory`** as readable properties (`0x14022af2e`, `0x14022af9f`) - which only
makes sense if the category is *normally computed*, and is a second, independent reason to
believe the arithmetic. `[D]`

### 3.3 Five tabs that draw empty

Scopes named by a sub-tab with **no sale row behind them**: `407` Gloves, `410` Effects, and
`700`/`701`/`702` Seasonal>Packages. `[L]` The `Special` tab declares no sub-tabs at all.

So of the seven tabs, **Seasonal and Special have nothing in them**, and Fashion has two empty
sub-tabs. That is the client's own data saying so, not a gap in the dump.

---

## 4. What is switched off, and the block nobody has explained

All 21 `OnSale = 0` rows are the **`92xxxxxx`** block, `92000000 .. 92000020`, contiguous.
Every one is `Price 0`. Their SN is outside the category scheme entirely - `category` comes out
`-1`, `scope` `-80` - so **no tab can reach them**. `[L]`

```
92000000  5000054  Snail                      92000011  5130000  Safety Charm
92000001  1112108  Classic Beginnings Label   92000012  5030000  Mushroom House Elf   *
92000002  1112219  Classic Beginnings Chat    92000013  5030001  Teddy Bear Clerk
92000003  5140000  Regular Store Permit   *   92000014  5030002  The Robot Stand
92000004  5070000  Megaphone              *   92000015  5120000  Snowy Snow
92000005  5070001  Super Megaphone        *   92000016  5120001  Sprinkled Flowers
92000006  5121000  GM's Blessing of Wind      92000017  5120002  Soap Bubbles
92000007  5121001  GM's Blessing of Precision 92000018  5120003  Snowflakes
92000008  5121002  Red Snail Invasion         92000019  5120004  Sprinkled Presents
92000009  5121003  Orange Mushroom Invasion   92000020  5130000  Safety Charm
92000010  5121004  Ribbon Pig Invasion
                                              * also sold on a normal row elsewhere
```

Four of the twenty-one duplicate an item that *is* on sale, at `Period 7` instead of the sale
row's 90 or 0. The set contains the GM blessings and the monster-invasion effects.

**[I]** and nothing stronger: this looks like a price-0 grant block - something a GM command or
a server-side gift would reference by SN rather than something a player browses to. Nothing in
these five images says so, and the client's reader treats them like any other row.

---

## 5. Packages: authored, and unbuyable

`CashPackage.img` defines two bundles: `[L]`

```
9100000  -> SN 170200001 170200002 170200003
9100001  -> SN 170200005 170200006 170200007 170200008 170200009
```

**All eight of those SNs are absent from `Commodity.img`** - the highest SN in the whole sale
list is `160300005`. `[L]` Their scope would be 702, which is one of the three Seasonal>Packages
scopes that has nothing behind it. So the packages tab, the package definitions and the sale
rows they need are three pieces of which only two shipped.

`gm-handbook/cashpackage.txt` carries an `snHasSaleRow` column so this cannot be missed by a
consumer; it is `0` on all eight rows.

Neither `9100000` nor `9100001` has a name in `String.wz`. `[L]`

---

## 6. Blocks

`gm-handbook/commoditylimit.txt`, 14 rows. `[L]` The only item-level block that names anything
is `LimitItem_Policy` / `LOOTBOX` (`BlockType 2`): **5150000, 5150001, 5151000, 5151001,
5152000** - the five random-outcome beauty coupons. `LimitItem_Policy / TRADE` (`BlockType 1`)
declares an empty item list, as do the two `LimitItem_CountryIP` entries (`VE`, `BY`) and
`CommodityTradeBlockBuyList / LimitItemID`. `LimitCountry` lists `BE` and `SK` and carries a
Korean note saying it is no longer used and was replaced by BlockPolicy.

`CommodityTradeBlockBuyList` sets `blockDay 0` and allows two drop exceptions, `4001001` and
`4001002`.

---

## 7. Does the client stock its own shop?

**[D], with the blind spot stated.** `FUN_140229d10` reads a Commodity row **by property
name** - `SN`, `ItemId`, `Count`, forty-odd of them. A packet is read positionally; a property
bag is read by name. So the client has a WZ-shaped Commodity parser, and the sale list is
something it can build for itself.

What I could **not** find is the call site that opens the image. `xref.py --string
"Commodity.img"` returns 0 code references, and the ASCII positive control passes in the same
breath - `xref.py --string "Etc/OldCommodity"` finds its one `lea` at `0x1402298f3`, inside
`FUN_1402298c0`, a 109-byte helper that formats the versioned path from a version number. The
named blind spot is the same one §2.1 already demonstrated: **a path assembled from a static
string object, or concatenated at run time, leaves no literal reference**, and that is
demonstrably how this client reaches its Commodity property names.

So: the client can parse the list `[D]`; whether the server must also send it is **not settled
here**, and the `0x00D5` reply this would hang off is still unbuilt (`research/cash-shop.md`
§7).

---

## 8. meowdb, as a cross-check only

`https://meowdb.com/msclassic/item-db/cash-shop` renders its table server-side and carries no
prices in the HTML. The data is at `https://meowdb.com/msclassic/api/items` - the same API
`tools/scrape_drops.py` already uses - where a cash row's `stats` blob carries `nxPrice`,
`onSale`, `periodDays` and, tellingly, **`cashShopLastSeenIn`** and `availableDate`. All
**[I]**.

### 8.1 The filter that nearly produced a confident wrong answer

`?type=Cash` returns 84 rows. Against those, none of the 89 cash-*equipment* ids in
`Commodity.img` appeared - so I widened to `?type=Equip`, paginated all 1 190 rows, and got
**zero** matches for all 89. Two queries agreeing.

Both were wrong the same way. Enumerating the whole table with no `type` filter - 2 555 items
over six pages - gives the type histogram:

```
Equip 1199   Cash Equip 560   Etc 358   Use 331   Cash 84   Setup 23
```

**`Cash Equip` is its own type**, and it holds all 89. Against the full index, **every one of
the 146 item ids in `Commodity.img` is present on meowdb.** This is `CLAUDE.md`'s "enumerate
before you filter" and "two scans agreeing is not corroboration when they share a blind spot",
hit live, on a third-party API rather than on the binary.

### 8.2 The comparison, on the full 2 555-row index

Of the **146** WZ item ids: `[I]` on the meowdb side throughout.

| | count |
|---|---|
| meowdb carries neither `nxPrice` nor `onSale` - it simply says nothing | **42** |
| agrees with the WZ on **both** price and on-sale | **32** |
| disagrees | **72** |

All 72 are a price mismatch; 65 of the 72 are also an on-sale mismatch. That 65 splits
60 / 5, and the big side is one-directional:

* **60 ids the WZ sells and meowdb marks `onSale: false` with no price** - 55 `Cash Equip`
  and 5 `Cash`. `1007000 Brown Flight Headgear`, `1077020 Bunny Slippers`, `1702009 Tiger
  Paw`, `1802002 Red Hat` and so on; the WZ has a live 100 NX row for each.
* **5 ids meowdb marks on sale at 100 NX that the WZ has switched off** - `5120000..5120004`,
  the weather effects, whose only rows here are the price-0 `92000015..92000019` block. Note
  the WZ *does* sell the neighbouring `5120005..5120009` at 100 (one) and 1000 (eleven).
* The remaining 7 disagree on price alone.
* Separately, **87 items carry an `nxPrice` on meowdb and have no Commodity row in this client
  at all** - 77 `Cash Equip` and 10 `Cash`, among them `5000000 Brown Kitty`,
  `5010000..5010004` (the "Smiley Sun" effect family) and `5140003 Fall Store Permit`.
* And of the 42 silent rows, **38 are ones the WZ has on sale** - meowdb simply has no cash
  data for them.

**Where they disagree, the WZ wins**, and in this case the reason is structural rather than a
judgement call: `cashShopLastSeenIn: "COT1"` and `availableDate: "2026-04-21"` say meowdb is
recording **what a live service rotated into the shop over time**. `Commodity.img` is the
authored list this client ships. They are not two measurements of one thing, so the 72
"disagreements" are mostly two sources answering different questions - which is why the count
is reported rather than reconciled.

One concrete agreement worth having: meowdb prices `5120005 Sprinkled Chocolate` at 100 NX,
and the WZ has `SN 130300000` at 100 for one and `SN 130300001` at 1000 for eleven. The
bundle pricing is in the WZ and not on the site.

---

## 9. Totals

```
sale rows                              159
distinct item ids                      146   (130 of them on at least one live row)
on sale                                138
switched off                            21   (all SN 92xxxxxx, all price 0)
price, live rows                       100 NX x130,  700 NX x1,  1000 NX x7
period                                 90d x95,  0 x49,  7d x14,  1d x1
count                                  1 x149,  11 x7,  10 x1,  5 x1,  3 x1
discounted rows                          7   (1000 against an originalPrice of 1100)
gender-restricted rows                   2
tabs / sub-tabs                        7 / 21
scopes with no rows behind them          5   (407, 410, 700, 701, 702)
package member SNs with no sale row      8   (all of them)
item ids blocked by policy               5   (LOOTBOX)
```

---

## 10. Not resolved

1. **`Class`.** 13 rows, values 0 and 2, read by the client, meaning unknown. Not the Main tab.
2. **The `92xxxxxx` block.** Price 0, unreachable from any tab, contains GM items. §4's reading
   is **[I]**.
3. **The call site that opens `Etc/Commodity.img`**, and therefore whether the server must send
   the list. §7 names the blind spot that kept it hidden.
4. **`WebShop` / `IsGift`.** In every row and read by nothing in the client. **[L]** that no
   such string exists; nothing can be said about what the authoring tool meant.
5. **Constant fields.** `Priority`, `ReqLEV`, `ReqPOP`, `Bonus`, `Refundable`, `Pb*` never vary
   in this data, so their direction and units are **[I]** from their names alone. `CLAUDE.md`'s
   unit rule applies to every one of them the first time a server writes one to the wire.
6. **Nothing is wired.** This is a data extraction. No server reads `commodity.txt` yet, and the
   `0x00D5` reply is still `inventory_rejected()` plus a chat line.
