# Character Info: the reply to `0x01FC` is `0x00A2`, not `0x00BE`

2026-09-18. Static, no client run, no Ghidra (project locked). Every address below was read
with `tools/reads.py`, `tools/listing.py`, `tools/callers.py`, `tools/dataref.py`,
`tools/rangescan.py` and `tools/dump_stringids.py`, run from the repo root.

Tags: **[L]** read off the listing; **[I]** inferred from what the code does with a value;
**[D]** decompiler-only or a single-path reading with a named blind spot; **[R]** the reference
server (a different version - a candidate, never a fact).

## 0. Answer up front

| question | answer |
|---|---|
| the request | outbound **`0x01FC`** = `u32 tick, u32 characterId, str name, u8 bPetInfo`. The observed 11 bytes `b8102c00 d6000000 0000 00` are tick, id 214, an **empty string** (`u16` length 0) and **bPetInfo = 0**. [L] |
| the reply | inbound **`0x00A2`**, dispatcher case `0xa2` -> `FUN_142cd87d0` (`research/msexe-gamestage-dispatch.c:220`). [L] |
| **not** the reply | `0x00BE`. Its case body hands the packet to `FUN_141e75800` on `DAT_143aa84f8`, which `research/npc-spawn.md` established is the **NPC pool** (`0x44F..0x467` are its cases). The `0xbe` arm reads `u8 n; n x u32` and rebuilds a container at `pool+0x40`. Nothing in that tree touches a level, job, fame or pet. `FUN_141e36b20` is the NPC-create decoder (`npc-spawn.md` §4), not an item body. [L] |
| the window | a **remote-info window** at singleton `DAT_143ac8908`, class `0x141194650` (ctor). It is a different class from the self-info window the screenshot showed (`0x1414b8da0`, UI id `0x5b7`, singleton `DAT_143acda08`), which is opened by UI command `0x45a` and reads only the local user. [L] |
| minimum reply | **60 bytes + name length**, §3 |
| what a wrong answer costs | the request sets the shared `m_bExclRequestSent` latch `[world+0x2330]` (`ap-allocation.md` §5). Only a latch-clearing packet releases it; `0x00A2` does, so does `0x0142`. Until then **~35 other request builders send nothing.** [L] |

## 1. How the handler was found, because the first premise was wrong

The brief said the reply was `0x00BE`. Three independent facts refuted it before any decoding:

1. `FUN_141e75800`'s other cases are `0x44f, 0x450, 0x451, 0x452, 0x453..0x466, 0x467` - the
   NPC block - and `npc-spawn.md` §7.2 already records `0xbe -> FUN_141e75800 - the NPC pool`. [L]
2. Its `0xbe` arm is `u8 count; count x (u32 -> FUN_140409670(pool+0x40, &id))` after
   `FUN_14040d460(pool+0x40)`: a list of ids replacing a list of ids. No strings, no flags. [L]
3. The self-info window class (`0x1414b8000..0x1414c0000`, 41 functions) reads exactly these
   globals: `DAT_143aa8518` (local user), `DAT_143aa84a0` (world), string pool, Gr2D. No
   remote-info struct anywhere. Its constructor has **one** caller, the UI-command handler
   `FUN_142311a50` (`edx - 0x459`), which reads no packet. [L]

The real chain, each link measured:

```text
0x140d99620   the player context-menu / double-click handler: compares the clicked id with
              FUN_142cb9550(world) (own id); if a window exists at DAT_143ac8908 it reads
              that window's id via the 0x141198bb0 getter (= &window+0x328), closes it, and
              stops if the ids match (toggle); else FUN_142d1caf0(world, id, 0)      [L]
DAT_143ac8908 written by 0x141194650 (ctor) and 0x141194990 (dtor) only        dataref [L]
0x141194650   one caller: 0x1411571c0, the "open UI window kind N" helper, N = 0x11   [L]
0x1411571c0   called from 0x142cd87d0 at 0x142cd8895, with ecx = 0x11               [L]
0x142cd87d0   = dispatcher case 0xa2, and it clears [world+0x2330] at 0x142cd87f4  [L]
```

`FUN_142cd87d0` is also the only dispatcher handler that both *creates* the window and *fills*
it (`FUN_141196d70(window, &struct)` at `0x142cd8a01`). The alignment table
`research/msexe-gamestage-opcodes.md` had it as `SortItemResult | IntrusionLobbyCandidateResult`;
both candidates are wrong, as that file warns they can be.

Read-count cross-check: `reads.py 0x142cd87d0 1` = 3 reads (raw, helper, u8);
`reads.py 0x1401cece0 5` = 21 reads. The listings show the same 3 and the same 21
(19 direct + 2 through `FUN_140303530`), at the same addresses. No decompile of either
function is on disk; nothing below rests on one.

## 2. The `0x00A2` body, field by field

Handler `FUN_142cd87d0(world, pkt)` (`0x142cd87d0..0x142cd8b90`) then the struct decoder
`FUN_1401cece0(&s, pkt)` (`0x1401cece0..0x1401cf0d3`), then one more byte in the handler.
`s` is a 0x90-byte stack struct, zero-filled first (`0x142cd8836..0x142cd8871`) and copied into
the window at `window+0x328..` by `FUN_141196d70` (`0x141196f20..0x141196fe6`). Offsets below
are into `s`; the window offset is `s + 0x300`.

| # | wire | read at | gate | stored | meaning | tag |
|---|---|---|---|---|---|---|
| 1 | `u32` | `142cd8828` (raw, 4) | - | stack | **result**. `cmp [rbp+0x67], 0 ; jne 142cd8b71` - anything non-zero **returns immediately** after clearing the latch; nothing is decoded, nothing shown. | [L] |
| 2 | `u32` | `1401cecf9` | - | `s+0x00` -> `w+0x328` | **characterId**. Compared with own id (`FUN_142cb9550`) at `141196da5` - equal -> the window fills from **local** data (`FUN_141197a40`) and the rest of `s` is discarded. Otherwise looked up in the user pool (`FUN_1429b5cf0`, `141196dee`) to draw the avatar; not found -> avatar skipped (`141196f1d`), fill continues. | [L] |
| 3 | `str` | `1401ced0b` | - | `s+0x08` -> `w+0x330` | **name**. The self path fills the same slot from `FUN_14276df30(localUser)`, the name getter. | [L] |
| 4 | `u32` | `1401ced53` | - | `s+0x10` -> `w+0x338` | **level**. Drawn with `%d` under string id `0x17F1` = `LEVEL` (`141195302`). Self path: stat row 10 (`cd+0x27`), which `charstat-layout.md` §6 names `level`. | [L] |
| 5 | `u32` | `1401ced5e` | - | `s+0x14` -> `w+0x33c` | **job** id. `FUN_1402b0250(job, 0)` at `141197106` resolves it to the job-name string drawn under `0x17F2` = `JOB`. Self path: stat row 11 (`cd+0x33`, `u16`, sign-extended), named `job`. | [L] |
| 6 | `u32` | `1401ced69` | - | `s+0x18` -> `w+0x340` | **fame**. `%d` under `0x17F3` = `FAME` (`141195426`). Self path: stat row 22 (`cd+0xb3`), named `fame`. | [L] |
| 7 | `str` | `1401ced7c` | - | `s+0x20` -> `w+0x348` | **guild name**. Drawn under `0x17F4` = `GUILD` (`141195555`). Self path: `FUN_142cc0410(world)` = the string at `[world+0x23e0]+8`, and `world+0x23e0` is the object every guild handler in `0x142dd1630..0x142ddd690` works on. Empty string allowed. | [L] for the slot, [I] "guild" via the guild object |
| 8 | `u32` | `1401cedc1` | - | `s+0x28` -> `w+0x350` | **pet item id** (the Cash item, e.g. `5000006`). Gates everything pet: `showPet` button enabled iff non-zero (`141195fa0`); the pet panel refuses to open if zero (`141198f2d`); the panel's body drawer passes it to `FUN_141ed3540`, the pet-template lookup by item id (`14119c81d`; `pet-draw-chain-2026-09-14.md` §13) and skips the draw if zero (`14119c805`). **0 = no pet.** | [L] |
| 9 | `str` | `1401cedd4` | - | `s+0x30` -> `w+0x358` -> panel`+0x308` | **pet type/name string**, drawn verbatim under `0x17F6` = `TYPE` (`14119bd6e`). Only this panel uses that label (one `mov edx,0x17f6` in the image), so there is no local control for *which* name. The v95 pet block has `sName` here. | [L] slot, [I] content |
| 10 | `u32` | `1401cee19` | - | `s+0x38` -> panel`+0x310` | **pet level**, under `0x17F1` = `LEVEL` (`14119ba69`) | [L] |
| 11 | `u32` | `1401cee24` | - | `s+0x3c` -> panel`+0x314` | **pet closeness**, under `0x17F7` = `CLOSENESS` (`14119bb73`) | [L] |
| 12 | `u32` | `1401cee2f` | - | `s+0x40` -> panel`+0x318` | **pet fullness**, under `0x17F8` = `FULLNESS` (`14119bc74`) | [L] |
| 13 | `u32` | `1401cee3a` | - | `s+0x44` -> panel`+0x31c` | the **4th argument of the frame loader** `FUN_140cd8da0` (`14119c869`). `CPet` passes `FUN_142770a00(owner, n)` there, which returns 0 whenever `n != 0`. **0 is a value the client itself produces.** Name unknown. | [L] use, [D] name |
| 14 | `u32` | `1401cee45` | - | `s+0x48` -> panel`+0x320` | **look-override item id**: if non-zero the body is drawn from `FUN_141ed3540(this)` instead of #8, falling back to #8's template if that returns null (`14119c825..14119c83f`). **0 = draw the pet at #8.** v95 has `dwPetWearItemID` at this position. | [L] mechanism, [R] name |
| 15 | `u8` | `1401cee54` | - | - | **hasPetItem**. Non-zero -> one whole `GW_ItemSlot` follows | [L] |
| 15a | item | `1401cee69` -> `FUN_140303530` | #15 != 0 | `s+0x50` -> `w+0x378` -> panel`+0x328` | `u8 type; <body>`. Type 3 = **exactly `net::bag::pet_item_with_state`** (`cash-shop-buy-done.md` §3.2: type byte + the type's `vt+0x358` decoder; a type outside 1..3 **crashes**). Present -> the panel builds a 0x11d8-byte item widget (`14119c543..`). The panel's level/closeness/fullness text does **not** come from here (§2, rows 10-12). | [L] |
| 16 | `u32` | `1401cef5d` | - | count | **item count, 0..32**. `> 0x20` -> the decoder **returns early** (`jg 1401cf0bf`), leaving #17/#18 unread and the cursor mid-body; `<= 0` -> empty. | [L] |
| 16a | item x n | `1401cef88` -> `FUN_140303530` | per entry | `s+0x60` vector -> `w+0x388` | whole `GW_ItemSlot`s. `FUN_14119a960` walks `w+0x388` and builds one 0x11d8-byte icon widget per entry - the **ITEM tab**. Which items belong there is not established (§5). | [L] shape, [I] tab |
| 17 | `u32` | `1401cf03f` | - | count | **record count, 0..2**. `> 2` -> early return as above. | [L] |
| 17a | `{u32,u32,u32}` x n | `1401cf063/6f/7b` | per entry | `s+0x78` vector -> `w+0x3a0` | **`{st, gr, ct}` per town, town 1 then town 2** (§6). `st == 1` -> `w+0x3c8 = gr` (the badge, `FUN_1411a2da0`); any non-zero `st` -> `w+0x3c4 = 0` (`1411970a1..1411970bd`), and `w+0x3c4` gates the `showCitizenship` button (`141195fc2`). The **CITIZENSHIP tab**'s data. | [L] |
| 18 | `u8` | `142cd8885` | - | bool | **showPetPanel**. `FUN_141198f20(window, b)` (`142cd8a0d`): `b != 0` **and** #8 `!= 0` and no panel open -> allocate the 0x338-byte pet panel (`FUN_14119aec0`); `b == 0` -> close any open panel. | [L] |

Order is the listing order; every read is unconditional except 15a (flag) and the two
count-driven loops. The decoder has one `ret` and the early-return branches only skip
*forward* to it, so no read can be reordered. [L]

After the decode the handler does, in order: open window kind `0x11` (`FUN_1411571c0(0x11, 0)`),
re-read the singleton (`142cd889a`; null -> free `s` and return, nothing shown), then
`FUN_141196d70(window, &s)` and `FUN_141198f20(window, #18)`, then free `s`. [L]

### 2.1 Two gates in front of the window

`FUN_1411571c0` first asks `FUN_141b1e8f0(DAT, 0x11)` and, if that says yes,
`FUN_14031f160(FUN_141892840())`; the second returning true raises message `0x11FB` instead of
opening anything (`1411571d8..141157205`). Not walked further. If the window never appears on
a run where the log shows `0x00A2` went out, this is the place to look. [D]

## 3. The minimum body

Everything optional absent (no pet, no items, no records, pet panel closed):

```text
00 00 00 00              u32   result          0 = show it
II II II II              u32   characterId     the clicked player's id (the request's u32)
LL LL <name>             str   name            u16 length, then bytes
VV VV VV VV              u32   level
JJ JJ JJ JJ              u32   job             the job id, e.g. 100 = Warrior
FF FF FF FF              u32   fame
GG GG <guild>            str   guild name      00 00 for none
00 00 00 00              u32   pet item id     0 = no pet: showPet greyed, panel never opens
00 00                    str   pet type name   ""
00 00 00 00              u32   pet level
00 00 00 00              u32   pet closeness
00 00 00 00              u32   pet fullness
00 00 00 00              u32   (frame-loader argument, 0)
00 00 00 00              u32   look override   0
00                       u8    hasPetItem      0
00 00 00 00              u32   item count      0
00 00 00 00              u32   record count    0
00                       u8    showPetPanel    0
```

**60 bytes + len(name) + len(guild).** For a player with no pet this is the whole answer.

### 3.1 With a summoned pet

Replace the pet section:

```text
46 4B 4C 00              u32   pet item id     5000006 (Husky)
NN NN <pet name>         str   drawn beside TYPE
LV 00 00 00              u32   pet level       PetVitals.level
CC CC 00 00              u32   pet closeness   PetVitals.closeness
FU 00 00 00              u32   pet fullness    PetVitals.fullness
00 00 00 00              u32   0
00 00 00 00              u32   0
01                       u8    hasPetItem
03 <pet body>            GW_ItemSlot = net::bag::pet_item_with_state(item_id, name, cash_sn, 1, &vitals)
                               (type byte 03 included by that function)
00 00 00 00              u32   item count      0
00 00 00 00              u32   record count    0
BB                       u8    showPetPanel    echo the request's bPetInfo (§4)
```

The item is what lets the panel build its item widget; the three numbers the panel *prints*
are rows 10-12, so **send both** and keep them equal to the item's vitals - the panel never
reads them out of the item. [L]

### 3.2 A refusal

`u32 result != 0`, four bytes, nothing else. The client clears the latch and shows nothing.
Use it for "no such character" and for a name that does not resolve. [L]

## 4. What the request carries

Two builders, both in `research/msexe-charinfo.c` and adjacent to the reply handler in the
image (`142d1caf0`, `142d1ccc0`, `142d1cdf0`):

| builder | writes | callers (`callers.py`) | meaning |
|---|---|---|---|
| `FUN_142d1caf0(world, id, u8)` | `tick, id, "", u8` | `0x140d99620` and the shim `0x140d9ea49` with **u8 = 0**; `0x1428b7600 x2`, `0x1428b8910` with **u8 = 1**; `0x1428b8d00` with 0 | **by id**. The double-click / context-menu entry passes 0. |
| `FUN_142d1ccc0(world, name, u8)` | `tick, 0, name, u8` | `0x1411bfec0` (from `0x1411bd380`) | **by name**; id is 0. Not the double-click. |

So the three trailing bytes of the observed body are **`00 00` = empty name string** and
**`00` = bPetInfo = 0**. [L]

`u8 = 1` is the **pet-info** entry (`0x1428b7f9d..0x1428b7ffa`): for the local user it opens
the window, fills it from local data and calls `FUN_141198f20(window, 1)` directly; for a remote
user it sends `0x01FC(id, 1)` and expects the reply's byte 18 to do the same. **The server
should echo the request's u8 into reply byte 18.** [L] for both mechanisms, [I] for "echo".

Both builders gate on: `[world+0x2338] == 0`, `[world+0x2330] == 0` (the latch), the local
record present, **HP > 0** (`FUN_1401ba9d0(cd+0x5b, [cd+0x63])` = stat row 16 `hp`), and
500 ms since `[world+0x2334]`. Then they set `[world+0x2330] = 1`. [L]

### 4.1 The latch is shared

`rangescan 0x2330` over `0x142c90000..0x142e00000`: 125 sites, ~35 request senders of the shape
`cmp [ctx+0x2330],0 ... mov [ctx+0x2330],1`, and the clearers `0xb7`, `0xb8/0xf5`, `0xf8`,
`0xf9`, `0xa1`, `0xa2`, `0x142`, `FUN_142cae700`, `FUN_142caa4e0` (field entry) and a few
more. `ap-allocation.md` §5 has the same finding for AP. An unanswered `0x01FC` therefore
blocks every one of those senders until the next field entry or one of those packets.
**Always answer `0x01FC`**, with §3.2 if nothing else. [L]

## 5. Not established

* **What goes in the ITEM tab** (row 16a). The client will draw up to 32 whole item slots as
  icon widgets; which items the server is meant to put there (equips? medals? a showcase?) was
  not derived. The v214 reference sends a full character record here and is no help. Sending
  count 0 is safe.
* ~~**The two 12-byte CITIZENSHIP records** (row 17a)~~ - settled 2026-10-04, §6. (This bullet
  also had the button backwards: a non-zero first word clears `w+0x3c4`, which ENABLES it.)
* **Row 13** (`s+0x44`): reaches the frame loader's fourth parameter; 0 is what `CPet` passes in
  the common case. Name unknown.
* **Row 14** (`s+0x48`): the override mechanism is measured; that it is v95's
  `dwPetWearItemID` is a reference alignment only.
* **Row 9's content**: the given pet name vs the item name. The client draws whatever string it
  gets; nothing local disambiguates.
* **`FUN_141b1e8f0` / `FUN_14031f160`** (§2.1): the pre-open gates were not walked.
* **Nothing here has been on a wire.** One `0x00A2` from `tools/channel_smoke.py`-style
  transport after a double-click is the falsifier: the window appears with the sent numbers,
  or the `WATCH` on `0x142cd8895` never fires.
* **`0x00BE`**: beyond "an id list handed to the NPC pool" its purpose was not pursued. It is
  not the character-info reply and should not be sent as one.

## 6. The CITIZENSHIP records: the client's own fill is the control (2026-10-04)

The owner, with a screenshot of Tester2 viewing a Henesys citizen: *"Citizenship data cannot be viewed
by other players"* - the CITIZENSHIP button was greyed, because the reply sent row 17 = 0.
Decompiled into `research/msexe-charinfo-citizenship.c`. All **[L]**:

* **The self path builds the same vector.** On the viewer's own character the window fills from
  local data, `FUN_141197a40`. At its end (`141198891..`) it empties `w+0x3a0` and, for town
  `t = 1, 2`, pushes `{FUN_1402c90f0(t), FUN_1402c9150(t), FUN_1402c92a0(t)}` - and applies the
  same two rules the remote fill applies (`st == 1` -> `w+0x3c8 = gr`; `st != 0` -> `w+0x3c4 = 0`).
* **The three getters are quest 510000's keys.** Each is `FUN_1402c8870(key, kind, t)` then
  `FUN_140729fb0(510000, key, 0)`, with kinds **0, 1, 2** = `st`, `gr`, `ct`
  (`research/citizenship-2026-09-27.md` §5).
* **The button.** `FUN_141195060`: `showCitizenship` is enabled when `w+0x310 != 0` or
  `w+0x3c4 == 0`. A remote fill with no record with a non-zero first word leaves `w+0x3c4` set -
  greyed, which is the screenshot.

So the reply carries **exactly two records, town 1 then town 2, `{st, gr, ct}` from the same
`citizenship` rows that make the character's own quest-510000 string** (`net::charinfo::TownRecord`,
`Session::charinfo_towns`). A town never signed is all zero, as the self path's getters return
for an absent key.

**Not walked:** how the citizenship panel lays the vector out. `FUN_1411a00d0` (the only other
reader of `w+0x3a0`) iterates it and refreshes; the drawing is behind a vtable call. It does not
change the answer - the remote reply now fills the same vector, in the same order, with the
same words, as the window the citizen sees for themself - but the "town names come from the
position, not a field" reading is [I] until a run shows a Kerning-only citizen.
