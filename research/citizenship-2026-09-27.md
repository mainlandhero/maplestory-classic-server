# Citizenship - what the fan site says, matched against this client (2026-09-27)

The owner: *"See what you can gather from this site about the MapleStory citizenship feature:
https://meowdb.com/msclassic/citizenship. Match what you can to the WZ data that we have, this is
our next feature to implement."* This extends `STATUS.md` goal H (2026-08-19), which already
matched the quests; the site now also carries the shops, resident items and the rules.

Labels: **[L]** read off this client (WZ, `Quest.wz`, the executable); **[S]** the site only
(second closed online test - "values can change before launch", its own words); **[D]** derived.

## 1. Everything the site names exists in this client

| thing | site | this client | |
|---|---|---|---|
| quests | 88, ids 506000-506045 / 506100-506141 | **88, the same ids** (`gm-handbook/questlines.txt`) | [L] |
| sign-up NPCs | Arthur (Henesys Town Hall), Roxy (Kerning City Civic Center) | **229** on 10001007 Henesys Town Hall, **425** on 10003007 Kerning City Civic Center | [L] |
| Community Boards | one per town | **235** on 10001000 Henesys, **431** on 10003000 Kerning City | [L] |
| Henesys shops | Raymond, Oak, Flint, Tommy (estate) | **232, 233, 234, 230**, all on 10001007 | [L] |
| Kerning shops | Max, Weston, Ben, Jack (estate) | **428, 429, 430, 426**, all on 10003007 | [L] |
| shop inventories + grade locks | six stores | **already in `data/shops.txt`**, rank-tagged (`Town Resident+`); `shops.rs` parses the tags to `min_grade` 1..10 - **and nothing enforces it: a gated row is sold to anyone today** | [L] |
| resident chairs | Henesys / Kerning, 10,000 at Visitor | **3010009, 3010010** | [L] |
| town earrings | Lv 57, Citizen of Honor | **1032021 Henesys, 1032022 Kerning City** | [L] |
| buff potions | Supreme Sniper / Sharpness / Supreme Dexterity / Destructive | **2002011, 2002013, 2002012, 2002014** | [L] |
| Daydreams | Orange Mushroom / Ribbon Pig | **2210005, 2210006** | [L] |
| story items | Recipe, Cake, Hammer x2, Gummy, Cure | **4031111, 4031112, 4031116/4031117, 4031113, 4031114** | [L] |
| scrolls | Earring Crit/Evasion, Overall LUK/INT..., Gloves ATT/MATT (Lesser) | all present (2040316, 2040320, 2040516, 2040804, ...) | [L] |
| grade badges | ten grades | **`UI/Citizenship.img/badge/0..9`** - ten | [L] |
| the sign-up screen | "sign with Arthur" | **`UI/Citizenship.img/contract`**: title, town line, body, OK / Cancel, a 4-frame stamp, the badge | [L] |
| effects | - | **`Effect/BasicEff.img/CitizenshipGet`, `CitizenshipGradeUp`** - UserEffect 83 / 84 (`net::questeffect`) | [L] |
| town hall art | - | `Map/Obj/obj_citizenship.img` henesys/kerning interior + exterior | [L] |

## 2. What the quest data encodes [L]

`Quest.wz` carries the system's rules as keys, 86 of the 88 quests (the two sign-ups have none):

```text
Check.N.citizenshipTown        1 = Henesys, 2 = Kerning City (45 / 41 quests)
Check.N.citizenshipGrade       a grade gate, 1..9
Act.N.citizenshipContr.town    which town banks it
Act.N.citizenshipContr.amount  flat: weeklies, story arcs
Act.N.citizenshipContr.amountFormula  "100 + ( ( citizenshipGrade - 1 ) x 50 )" - every daily
```

The client evaluates that formula string itself (`FUN_1402C96E0` / `9930` / `9A00`, a small
recursive-descent parser whose one variable is `citizenshipGrade`) - to draw the reward in the
quest window. **The server has to compute the same number**, with the same grade.

### 2.1 Two corrections to the site, from the data [L]

* **The weekly pays a FIXED amount per quest, set by its gate, not by the player's grade.** Each
  weekly is gated `citizenshipGrade >= g` and pays `500 + 250 x (g - 1)`: 500 (Blue Snail Shell,
  g 1) ... 2,500 (Drake Skull / Medicine With Weird Vibes, g 9). The site's "500 at grade 1,
  +250 per grade" is the same table read as a function of the player.
* ~~**Grade and level gates move together** - every quest gated at grade g has `lvmin = 12 + 5(g-1)`
  (g 2 -> 17, 5 -> 32, 9 -> 52).~~ **RETRACTED 2026-09-28, and the claim was the mistake.** Listing
  `Check.0.lvmin` beside `Check.0.citizenshipGrade` for all 86 quests: it holds for the **15 story
  quests only** (grade 2 -> `lvmin 17`, grade 4 -> `lvmin 27`). Every board quest is `lvmin 12`
  whatever its gate - the grade-9 donations (506035, 506136) and the grade-5 leader dailies
  included. So "5 -> 32, 9 -> 52" was never observed; it was the formula extended past its two
  data points. The site's level column stays **[S]**.

Story arcs pay small flat amounts: Bruce / Jake's arcs 50 per step and 100 for the last (grade
2); Chief Stan's 80 per step and 180 for the last (grade 4).

### 2.2 What the quest data does NOT carry

* **Signing up.** 506000 (Arthur) and 506100 (Roxy, requires 506000 done) set no town. The town is
  chosen by the contract window (`UI/Citizenship.img/contract`) - server-driven.
* **Daily / weekly cadence.** No `dayByDay`, `interval` or weekday keys on any of the 88. "The
  board posts one resident each day" and "one donation a week" are the server's to enforce - and
  the client decides quest availability locally, so the server must shape what it sees (reset or
  withhold quest rows).
* **Grade thresholds, discounts, storage fees, reactivation fees** - no Etc image for them (a
  tree of every archive finds only `UI/Citizenship.img` and `Map/Obj/obj_citizenship.img`). These
  stay [S].

## 3. The rules, as the site gives them [S]

| grade | name | Lv | contribution to reach | shops/taxis | storage disc. | storage fee |
|---:|---|---:|---:|---:|---:|---:|
| 1 | Traveler | 12 | from the sign-up | 5% | 5% | 95 |
| 2 | Visitor | 17 | 1,000 | 7% | 10% | 90 |
| 3 | Helpful Stranger | 22 | 2,000 | 9% | 15% | 85 |
| 4 | Recognized Guest | 27 | 3,000 | 11% | 20% | 80 |
| 5 | Town Resident | 32 | 4,000 | 13% | 25% | 75 |
| 6 | Trusted Neighbor | 37 | 5,000 | 15% | 30% | 70 |
| 7 | Distinguished Citizen | 42 | 6,000 | 17% | 35% | 65 |
| 8 | Town Patron | 47 | 7,000 | 19% | 40% | 60 |
| 9 | Guardian of the Village | 52 | 8,000 | 21% | 45% | 55 |
| 10 | Citizen of Honor | 57 | 10,000 | 25% | 50% | 50 |

Names corroborated by `data/shops.txt`'s tags; levels now [D] (§2.1); the rest [S].

* Unlocks at level 12. One town at a time, each with its own progress; switching starts the new
  town fresh and banks the old; returning restores it. First move free; reactivation 50,000 mesos
  at grade 1 (other grades unconfirmed by the site). The client itself carries the refusals:
  `[citizenship] Other town quest in progress - cannot transfer.`, `Town quest in progress -
  cannot inactivate.`, `Unknown town.` [L - the strings; where they are raised not traced].
* Dailies: 100 contribution at grade 1, +50 per grade (= the formula [L]). "First Greeting" runs
  once; "Asking After" repeats (it requires the First Greeting done [L]). Grade 5 adds the town
  leaders' VIP dailies (Chief Stan / Athena Pierce, Chun Ji / Dark Lord) [L gates].
* Discounts apply in the active town on tagged items - buff potions, pet food, return scrolls, the
  town cab (5%..25%); storage in that town uses its own 5%-per-grade schedule from 100 mesos/item.
  Dr. Faymus (410, Kerning Pharmacy) is the site's example of a discounted non-citizenship shop.
* Resident's Chair: trade-blocked, 20 HP / 5 MP per 10 s seated, one per town. Earrings: Lv 57,
  42 MDEF, +2% crit damage, +2 avoid, 5 slots, untradeable - **all [L]**, `equips.txt` rows 1032021/1032022
  match the site field for field.
* Double-click a player -> their grade and progress: the Character Info packet already has a
  CITIZENSHIP section - `{u32 kind, u32 value, u32}` x 0..2, `kind == 1 -> value` drawn, any
  non-zero kind hides the `showCitizenship` button (`research/character-info-2026-09-18.md` row
  17a). What value 2 is and how grade/progress are drawn was not read.
* Ten NPCs per town greet differently by grade - the idle-chatter machinery keyed on grade.
* Housing / the estate agents (Tommy, Jack): the site has no details yet.

## 4. The client already has hooks for it [L]

| | where | today |
|---|---|---|
| shop row: required citizenship **town** and **grade** | `net::classicshop` item record `+0x104`, `+0x108` | sent as 0 - the client never shows a row as locked, and the server does not refuse it either (`min_grade` is parsed, never read) |
| Character Info CITIZENSHIP section | `net::charinfo` row 17 | count 0 |
| effects 83 CitizenshipGet / 84 CitizenshipGradeUp | `net::questeffect` | never sent |
| GM command | the string `/citizenship <townID> <state|grade|contr> <val>` | not handled |
| contract window | `UI/Citizenship.img/contract` (code: `FUN_1411A2DA0` loads it) | never opened |

## 5. The grade lock: SOLVED - quest 510000's key=value record [L]

The owner, 2026-09-27: *"Please figure out the grade-locking system. Decompile or static analysis if
you need to. The daily and the weeklies are offered by the server, those are not client chosen."*

**There is no citizenship packet.** The client keeps a character's citizenship as `key=value`
pairs in the "ex" record of a hidden quest, **510000** - the same mechanism MapleStory uses for
any per-character string state. Every reader goes through it:

```text
FUN_1402C8870(out, kind, town)   builds the key:  kind 0 "st", 1 "gr", 2 "ct"  + "%d" town
                                 (the prefixes at 0x14327D940 / 944 / 948, the format at 0x143274298)
FUN_140729FB0(510000, key, 0)    reads quest 510000's ex string, finds key, atoi()s it
FUN_1402E01C0(chr, out, q, key)  the lookup: a hash map at CHARACTER + 0x12BB, quest id -> string
FUN_140193890 / 140193CD0        parse the string as  key=value;key=value   (split on 0x3D, 0x3B)
```

| key | town 1 = Henesys | town 2 = Kerning City | meaning |
|---|---|---|---|
| state | `st1` | `st2` | **`1` = the active citizenship**. Every lock tests `== 1`. Other values [I]: 0 never, anything else banked/inactive |
| grade | `gr1` | `gr2` | 1..10 |
| contribution | `ct1` | `ct2` | the running total (the `/citizenship <townID> <state|grade|contr> <val>` GM command's third word) |

`FUN_1402C8AF0(town)` accepts only towns 1 and 2 (`town - 1 < 2`); town names are the map names of
10001000 / 10003000 (`FUN_1402C8C00`).

So a Henesys Town Resident with 4,150 contribution is quest 510000 = `st1=1;gr1=5;ct1=4150`.

### 5.1 The locks, each read off its own code

| where | test | refusal | |
|---|---|---|---|
| **quest start** `FUN_14070FE30` (quest check struct `+0x248` town, `+0x24C` grade - from `Check.N.citizenshipTown/Grade`) | `st<town> == 1 && gr<town> >= required` | reason **`0x50`** | [L] |
| **quest completion** `FUN_140711E50` | the same two keys, the same rule | `0x32` on an invalid town | [L] |
| **shop row** `+0x104` town / `+0x108` grade (`research/classic-shop-rows.md` rows 39-40) | `FUN_1402C90F0` (= `st == 1`), `FUN_1402C9150 >= +0x108` | strings `0x17DA` "%s citizenship required." / `0x17DB` "...Grade %s or higher..." | [L] |
| **NPC grade lines** `FUN_141E83B30` (the `c10`..`c31` lines in `String/Npc.img`) | `st == 1`, grade **exactly** the line's, within 200 px of the NPC | - | [L] |
| NPC line text `FUN_141E3B510` | substitutes `/town`, `/grade` (grade -> name, `FUN_1402C8CF0`), `/name` | - | [L] |
| quest window reward `FUN_141F15FB0` | evaluates `amountFormula` with `gr<town>` | - | [L] |

The daily and weekly quests are **offered** by the server (the owner), but the client still runs the
start check above on them - so without quest 510000's record every citizenship quest is refused
with `0x50` whatever the server offers. Setting the record is the prerequisite for all of it.

### 5.2 How the record reaches the client [L]

1. **At every SetField, in the character record:** block `#28`, behind **presence byte 16**
   (`research/charrecord-presence-map.md`): `u16 count`, then `count x (u32 questId, str value)`,
   stored by `FUN_1402E19A0` into `+0x12BB`. It sits after the completed-quests block
   (presence 14) and before the record's final ungated `u8` (`0x140308B3F`) - in
   `net::opcode::character_record_for_set_field_with_quests_and_skills` that is after
   `quests.completed_block()` and before `out.push(tail)`.
2. **Live, as `0x0089` sub-case 13:** `u32 questId, str value` -> `FUN_142D5C7B0` ->
   `FUN_142D5AA10` -> `FUN_1402E19A0`. It **replaces the whole string** for that quest. Quest
   510000 (`0x7C830`) has no special reaction there (the handler reacts to `0x1896C`, `0x30DD`,
   `0x34C0`, `0x1E68`, ...); the UI reads the record when it needs it.

The two other `(u32, str)` stores are NOT this one - checked, not assumed: sub-case 14 / block
`#32` write `+0x12D3` (`FUN_1402E1A20`), sub-case 27 / block `#30` write `+0x1317`
(`FUN_1402E1AC0`), block `#21` writes the ordinary started-quest records at `+0x1273`
(`FUN_1402E0C30`).

### 5.3 The rest of the wire, for the build

* **`0x0089` sub-case 35**: `u8 town, u32 amount` -> *"You have gained %s Contribution (+%d)"*
  (`FUN_142D975A0`, string `0x17CD`; an invalid town drops it).
* UserEffect **83 `CitizenshipGet`**, **84 `CitizenshipGradeUp`** (`net::questeffect`).
* Shop rows: send `+0x104 = town`, `+0x108 = grade` - the client then locks and refuses rows itself.
* Not yet walked: what opens the contract window (`FUN_1411A2DA0` loads `UI/Citizenship.img`).

## 5.4 The contract window: SOLVED - ScriptMessage types 0x42..0x46 [L]

The owner, 2026-09-27: *"Figure out the sign-up contract window."* It is an **NPC script dialog**: the
server sends `0x055B` with message type `0x42 + variant`, the ordinary script head (`handle` in
head field 1 - these five are the only types besides Say that keep it, `research/script-reply.md`
§4), and the answers come back on `0x00F3`. One window class for all five - ctor `FUN_1410DCC60`
(0x390 bytes), setup `FUN_1410DD230`, layout `FUN_1410DD520` (`UI/Citizenship.img/contract`),
text `FUN_1410DFB90` / `FUN_1410DEC20`.

| type | variant | handler | body after the head | window fields |
|---|---|---|---|---|
| `0x42` | 0 **Oath of Citizenship** | `FUN_141F765D0` | `u8 town, u32 npc` | town `+0x2A4` |
| `0x43` | 1 **Transfer of Citizenship** | `FUN_141F76920` | `u8 newTown, u8 oldTown, u32 npc` | `+0x2A4` new, `+0x2B8` old |
| `0x44` | 2 **Citizenship Reactivation** | `FUN_141F76C80` | `u8 town, u8 grade, u32 fee, u32 npc` | fee `+0x2B4` |
| `0x45` | 3 **Renunciation of Citizenship** | `FUN_141F76FF0` | `u8 town, u8 grade, u32 npc` | |
| `0x46` | 4 **Citizenship Grade Update** | `FUN_141F77350` | `u8 town, u8 grade, u32 ?, u32 npc` | grade `+0x2AC` |

* **`npc` is an NPC TEMPLATE id** looked up in the field's NPC pool (`FUN_141E768F0` on
  `DAT_143AA84F8` compares `npc+0x198 -> template id`, `FUN_141E39AF0`) - its NAME is drawn
  (`FUN_141E39B10`, `template+8`). It must be an NPC standing on the map; a miss draws an empty
  name, not a fault. Arthur 229 / Roxy 425.
* **town** names: 1 Henesys, 2 Kerning City (`FUN_1402C8C00`, the map names of 10001000/10003000).
* **grade (variant 4):** the window draws grade `+0x2AC + 1`'s name (`FUN_1402C8CF0`) and uses
  `+0x2AC` for the badge - **send the NEW grade minus one** [D]. Variants 2/3 carry it; nothing
  draws it there.
* The `u32 ?` in `0x46` lands at `+0x2B0` and nothing found reads it - send 0.
* Stamp: variant 4 uses `stampPos/4`, the others `stampPos/0` - the two positions in the WZ.

### The text - string pool, all [L] (`tools/dump_stringids.py`)

| id | text |
|---|---|
| `0x17E0`..`0x17E4` | titles: Oath of Citizenship / Transfer of Citizenship / Citizenship Reactivation / Renunciation of Citizenship / Citizenship Grade Update |
| `0x17E5` | Do you solemnly swear to become a proud citizen of %s? |
| `0x17E6` | Welcome to %s! Would you like to start your new citizenship here? |
| `0x17E7` | Welcome back! Would you like to resume your citizenship in %s? |
| `0x17E8` | Are you sure you want to renounce your citizenship in %s? |
| `0x17E9` | Under the protection and authority of #e%s#n (the NPC), your Citizenship grade has been officially upgraded. ... Your influence in #e%s#n (the town) continues to grow ... |
| `0x17EA` | [Terms & Conditions] one town at a time; activity begins now; you may renounce any time |
| `0x17EB` | [Warning & Conditions] your journey in %s starts today; citizenship in %s (Grade, Home, Perks) will be frozen; a Meso fee to return to %s later |
| `0x17EC` | [Notice] **Reactivation Fee: %s Mesos**; previous Grade and Perks fully restored; an active citizenship elsewhere will be frozen |
| `0x17ED` | [Warning & Conditions] Grade and Contribution locked until you return; Home entry and Specialty Shop access suspended; a Meso fee to return |
| `0x17EE`..`0x17F0` | Citizen / Appointed by / New rank assigned |
| `0x17CF`..`0x17D8` | the ten grade names |
| `0x17D9` | Let us all congratulate %s for becoming a Citizen of Honor in %s! |
| `0x17CD` | You have gained %s Contribution (+%d) - `0x0089` sub-case 35 |

So the client itself states the rules the site only reported: one active town; switching FREEZES
the old one (grade, home, perks); returning costs a fee and restores everything; renouncing locks
grade and contribution and suspends home and specialty shops.

### The answers on `0x00F3` - `u32 handle, u8, u8` (`FUN_1410DE8C0`, `FUN_1410DE9A0`)

| event | bytes | then |
|---|---|---|
| **OK** (button id 1000) | `handle, 0x42+v, 1` | both buttons disabled, `+0x320` set; the window **waits for the server** (see below) |
| **Cancel** (1001) or **Esc** | `handle, 0x42+v, 0` | the window closes |
| **the stamp finished** (~2,000 ms after OK, `FUN_1410DE710`) | `handle, **0x47**, 0x42+v` | the window closes |

**CORRECTED 2026-09-29 by the first client run - the stamp is the SERVER's to start.** This
section said OK plays the stamp. It does not: the owner signed, the server wrote the citizenship and
said so, and the window sat on screen with its buttons greyed and never closed
(`research/fixtures/citizenship-oath-signed-but-window-never-closed-no-server-force-close-world.log`,
03:22:01 - the OK arrived, no `0x47` answer ever followed). `FUN_1410DEA80` is what starts the
stamp (sets `+0x321`, plays the `+0x308` layer), and **its only caller is the `0x055B` reader's
force-close branch**, `0x141F6F486` (`tools/callers.py`: one call site, no jmps, no pointers):
message type `0x47`, one `u8`, `== 1` passed as the flag. So:

* after acting on OK the server sends **`0x055B` type `0x47`, result 1** - the stamp plays, and
  2 s after it finishes `FUN_1410DE710` sends the third answer and closes the window (vtable
  `+0x138`, 6);
* a contract the server refuses gets **result 0** - `FUN_1410DE9A0` sends the third answer (since
  `+0x320` is set) and the window closes at once (7), no stamp.

One accepted contract is still **two** `0x00F3`s: act on the first (`answer 1`), answer it with the
force-close, and swallow the second. The table's "stamp plays" row was read off the OK handler's
button-disable and the stamp art's existence, not off what calls the stamp - the missing step was
enumerating `FUN_1410DEA80`'s callers. Our `parse_script_reply`
reads a Say shape and would drop these 6-byte bodies: they need their own parser, checked before
it, the way the taxi and the menus are.

### What the server does with it

* Arthur / Roxy, a character of Lv 12+ with no citizenship: `0x42` (town, npc) -> OK -> set
  `st<town>=1; gr<town>=1; ct<town>=0` in quest 510000 (character record + `0x0089` 13), play
  UserEffect 83 CitizenshipGet.
* Already a citizen elsewhere, first time here: `0x43` (new, old) -> OK -> old town's `st` to the
  frozen value, new town active.
* Returning to a frozen town: `0x44` (town, grade, fee) -> OK -> take the fee, restore.
* Renouncing: `0x45` -> OK -> freeze.
* A grade-up: `0x46` (town, newGrade-1, 0, npc) as the certificate, with UserEffect 84.

## 7. The Community Board: SOLVED - the group's record lists what is posted [L]

The owner, 2026-09-27: *"The daily and the weeklies are offered by the server, those are not client
chosen."* The mechanism, read 2026-09-28:

* `Quest/RecurringQuestGroup.img` (loader `FUN_140720C10`, from `FUN_141B10350`) builds, on the
  quest manager, `+0x830` quest id -> group id and `+0x818` group id -> `{type, qrID +0x24,
  qrKey +0x28, selectCount, list, doNotRepeat (a set)}`. Four groups over **71** quests:
  `510001 q1_d` 506001..506018, `510002 q1_w` 506019..506035, `510003 q1_d` 506101..506118,
  `510004 q1_w` 506119..506136; `selectCount 1` in all four; `doNotRepeat` = the odd dailies (the
  *First Greeting* halves). (Group 0, crafting's weekly `90007 craftingWeekly`, has no list.)
* **`FUN_14070FAE0`** - the general "may this quest start" check, 47 call sites - runs the
  ordinary check (`FUN_14070FE30`) and then, for a quest in `+0x830`: reads quest `qrID`'s ex
  record under `qrKey` (`FUN_1402E0240` -> the `+0x12BB` map -> `FUN_1401938F0`, the same
  key=value parse as 510000), splits it on **`|`** (`DAT_14329643C`), and compares each piece
  with `"%d"` of the quest id. No match -> **`0x51`**. So the posting is
  **`510001 = "q1_d=506005|506006"`**, and anything not listed is unavailable.
* A **completed** quest in a group goes through `FUN_14070FE30`'s completed path
  (`0x1407104D5`, after `FUN_1402E3390` = "in the completed map `+0x1347`"): in `doNotRepeat` ->
  **`0x19`**; group type 0 (daily) completed today (`FUN_1408F6850`) -> **`0x13`**; weekly -> days
  since completion `< 8 - weekday(completion)` -> **`0x16`** (weekday 0 = Sunday, so the week turns
  on **Monday**); otherwise it may start again. The completion stays in the completed map until
  then - the owner's "remain in the completed tab until it is chosen again" is the client's own rule.

## 6. What a first build would be

**Built 2026-09-28** (`crates/{net,store,world}/src/citizenship.rs`, `session/citizenship.rs`) -
items 1-5 and the GM command. Not built: shop/storage/taxi **discounts** (which items are tagged
is [S] and not in `data/shops.txt`), Character Info's CITIZENSHIP section (what `kind`/`value`
mean was never read), and item 7. The original list:

1. `store`: per character, per town `(active, grade, contribution)`; the active town. Sent as
   quest 510000 `st1=..;gr1=..;ct1=..;st2=..;gr2=..;ct2=..` - in the character record (presence 16)
   and as `0x0089` sub-case 13 whenever it changes.
2. Sign-up at Arthur / Roxy - the contract window (after §5), else a yes/no - with the
   CitizenshipGet effect.
3. `Check.citizenshipTown/Grade` honoured on quest start; `Act.citizenshipContr` paid on completion
   (flat or the formula, with the grade at completion); grade-up at the thresholds with the
   CitizenshipGradeUp effect.
4. Daily / weekly: reset the board quests' rows on the day / week boundary; one posted daily.
5. Shops: send `+0x104/+0x108` so locked rows show locked; apply the town discount to tagged items
   and to storage.
6. Character Info's CITIZENSHIP section; the `/citizenship` GM command.
7. Later: town switching and its fee, grade dialogue, chairs' regen, earrings at grade 10, housing.
