# The right-hand message area: `0x0089`, and why `0x00BB` is the wrong packet

The owner, with a screenshot of a live server:

> *"When the drops are picked up and when the players receive EXP, it should actually show
> on the right hand side of the client. We should not be outputting in the chat log
> regarding level ups and item pickups."*

Labels are the project's: **[L]** read off this client's listing or a capture · **[D]**
derived from two or more [L] facts · **[I]** inferred, not established.

No Ghidra, no client run. Everything below came from `client-patched/MapleStory.exe`
through `tools/reads.py`, `tools/listing.py`, `tools/callers.py`, `tools/rangescan.py`,
`tools/dataref.py`, `tools/dump_stringids.py` and `target/release/wz-dump.exe`.

---

## Summary

| | |
|---|---|
| **opcode** | **`0x0089`** — the one `crates/net/src/quest.rs` already calls `MESSAGE` |
| **first body byte** | a **type**, bounded at `0x23`, indexing a 36-entry jump table at `0x142d44a88` |
| **experience gained** | type **3** — `u8, u64 exp, u8 inChat, u64 bonusMask` = **18 bytes** |
| **item picked up** | type **0**, sub-mode **0** — `u8, i8 0, u32 itemId, u32 count, u8 where` |
| **mesos picked up** | type **0**, sub-mode **1** — `u8, i8 1, u8, i32 gain, u16, i32 bonus` |
| **who writes the words** | **the client**, from `%lld`-style format strings in its own encrypted string table. The server sends **numbers only** |

`0x0089` is not a new discovery — `research/quest-state.md` found the packet and decoded
sub-case **1**, the quest record, and wrote that *"the other 35 are unread and this module
does not guess at them"*. This document reads five more of them.

---

## 1. How the packet was found, and the control that makes it evidence

The wording in the owner's screenshot (`You have gained experience (+211)`) does **not** appear
in this client. That is a real negative rather than a failed search, and here is the
control that makes it one:

```
$ python tools/dump_stringids.py --id 1331
  [ 1331] 0x0533  '[Welcome] Welcome to MapleStory!!'
```

That is the line `research/talking-back.md` already identified as the client's own chat
greeting, and it comes out of the encrypted table cleanly — so the instrument speaks. It
then finds **6165** strings in locale 0, among them a **contiguous block, ids `0x00C0`
through `0x00F0`**, which is the whole message family: **[L]**

```
0x00C0  'Obtained %s.'
0x00C1  'You received EXP (+%lld)'          <- this client's wording for the owner's line
0x00C2  'You received bonus EXP (+%lld)'
0x00C3  'You have received some SP! (+%d)'
0x00C4  'You have earned some Level (%d) Job Advancement SP! (+%d)'
0x00DF  'You have gained fame. (+%d)'
0x00E1  'You have gained mesos (+%I64d)'
0x00E2  'You have gained additional Mesos (+%I64d)'
0x00E3  'Meso Penalty Applied (%I64d)'
0x00E5  'Spotting Small Change (+%d)'
0x00EC  '%s x%d earned. (%s)'
0x00ED  '%s x%d earned. (%s / Bag)'
0x00EE  '%s x%d earned. (%s and Bag)'
```

So **the client composes the sentence and the server sends the number.** That settles the
question the brief asked to settle: this is a *structured* packet, not a formatted-string
one. **[L]** — the format specifiers are in the strings themselves.

The owner's exact wording (`You have gained experience`, `Multi-kill bonus EXP`) is **not** in
this client's table. Their screenshot is of a different build. What we can reproduce is
`You received EXP (+211)`.

### Finding the code from the strings

`tools/dump_stringids.py` gives ids; the client resolves an id with
`FUN_1408a9e40(dest, id)` and the id arrives as a `mov r32, imm32`. A scan of every
executable section for `B8+r <id>` mapped back through the merged `.pdata` extent, run over
the whole `0xC0..0xF0` block and ranked by how many distinct ids each function holds:

```
FUN_140a10a40    47 ids     (three functions with all 47 - the string-table build itself)
FUN_140a165f0    47 ids
FUN_140a9d500    47 ids
FUN_142d5da20    30 ids     <- in the CWvsContext range 0x142c..0x142e
FUN_1418d1040    22 ids
```

Its positive control: the same scan for id `0x533` returns five sites, **one of them
`FUN_14209ee50`** — the function `research/talking-back.md` independently named as the
poster of the `[Welcome]` line. **[L]**

`tools/callers.py 0x142d5da20` gives **one** call site: `0x142d43fb6`, inside
**`FUN_142d43ee0`** — which `research/msexe-gamestage-cases.txt` already lists as
**`case 0x0089`**. **[L]**

---

## 2. `FUN_142d43ee0` — the type byte and its 36-entry table

```asm
142d43f0c  call 1406e8ae0        ; u8  <- THE TYPE
142d43f14  cmp  eax, 0x23
142d43f17  ja   142d44a6a        ; > 35 -> nothing at all
142d43f1d  lea  rcx, [rip-0x2d43f24]      ; rcx = 0x140000000
142d43f24  mov  r8d, [rcx + rax*4 + 0x2d44a88]
142d43f2f  jmp  r8
```

The table at `0x142d44a88`, read straight out of the PE (36 dword RVAs, base
`0x140000000`): **[L]**

| type | target | handler | what it is |
|---:|---|---|---|
| **0** | `142d43f32` | `FUN_142d59360` | **drop pick-up** — item, meso and the refusals |
| **1** | `142d43f42` | `FUN_142d59e20` | quest record — already decoded, `research/quest-state.md` |
| 2 | `142d43fa0` | `FUN_142d5ce00` | unread |
| **3** | `142d43fb0` | `FUN_142d5da20` | **experience gained** |
| **4** | `142d43fc0` | inline | **skill points** — `u16`, `u8`; strings `0xC3`/`0xC4`/`0xC5` |
| **5** | `142d44070` | inline | **fame** — one `i32`; `0xDF` when >= 0, `0xE0` when < 0 |
| **6** | `142d44109` | `FUN_142d5f990` | **mesos** (string `0xE1`); `u64, u32, str` |
| 7 | `142d44178` | inline | unread, opens with a `u32` |
| 8 | `142d44211` | `FUN_142d60230` | unread |
| 9 | `142d44221` | inline | unread, `u32` |
| 10 | `142d44255` | inline | unread |
| 11 | `142d4433b` | inline | unread |
| 12 | `142d4437f` | inline | unread |
| 13 | `142d43f52` | `FUN_142d5c7b0` | unread |
| 14 | `142d43f62` | inline | `u32`, `str` |
| 15..18, 20, 21, 24..28, 30, 31, 34, 35 | | | unread |
| **19, 29, 32, 33** | `142d44a6a` | — | **no handler**, the same target as `type > 35` |
| 22 | `142d44119` | `FUN_142d5ff40` | unread |
| 23 | `142d44129` | inline | two `str`s |

Types 0, 1, 3, 4, 5 and 6 are exactly the `DROP_PICKUP / QUEST_RECORD / … / INC_EXP /
INC_SP / INC_POP / INC_MONEY` ordering the v214 reference tree uses, with `2` sitting
where its `CASH_ITEM_EXPIRE` sits. That is **corroboration, labelled [I]** — every value
in the table above is read from this client's own jump table and does not depend on it.

---

## 3. Type 3, experience: the body, field for field

`FUN_142d5da20` zeroes a 0x104-byte stack struct at `[rbp+0xb0]` (`142d5da5e`..`142d5daf6`,
covering `+0x00` through `+0x100` with `mov`s and eight `movdqa`s) and hands it to
**`FUN_1408cfcf0(dst, packet)`**, which is the whole reader. `tools/reads.py 0x1408cfcf0 4`
and the listing agree, and the struct being pre-zeroed is what makes the shortest body
safe. **[L]**

```text
u8    -> dst+0x00     "white"      picks a colour on the screen post (see §5)
u64   -> dst+0x08     the EXP amount, and the ONLY number the sentence needs
u8    -> dst+0x10     inChat       0 = the message area, non-zero = the chat log (§5)
raw8  -> local        a u64 BITMASK of bonus-EXP categories

  mask & 0x01          raw8 -> dst+0x18
  mask & 0x04          u8   -> dst+0x20
  if dst+0x10 != 0:    u8   -> dst+0x24        <- else the pre-zeroed 0 is used
  if that value > 0:   u8   -> dst+0x28
  mask & 0x20          raw8 -> dst+0x38        <- note: 0x20 is read BEFORE 0x10
  mask & 0x10          raw8 -> dst+0x30
  mask & 0x40          raw8 -> dst+0x40
  mask & 0x80          raw8 -> dst+0x48
  mask & 0x100         raw8 -> dst+0x50
  mask & 0x200         raw8 -> dst+0x58
  mask & 0x400         raw8 -> dst+0x60
  mask & 0x800         raw8 -> dst+0x68
  mask & 0x1000        raw8 -> dst+0x70
  mask & 0x2000        raw8 -> dst+0x78
  mask & 0x4000        raw8 -> dst+0x80
  mask & 0x10000       raw8 -> dst+0x88
  mask & 0x20000       raw8 -> dst+0x90
  mask & 0x40000       raw8 -> dst+0x98
  mask & 0x80000       raw8 -> dst+0xa0
  mask & 0x100000      raw8 -> dst+0xa8
  mask & 0x200000      raw8 -> dst+0xb0
  mask & 0x800000      raw8 -> dst+0xb8
  mask & 0x1000000     raw8 -> dst+0xc8
  mask & 0x2000000     raw8 -> dst+0xd0
  mask & 0x4000000     raw8 -> dst+0xd8
  mask & 0x10000000    raw8 -> dst+0xe0
  mask & 0x20000000    raw8 -> dst+0xe8
  mask & 0x80000000    raw8 -> dst+0xf0
  mask & 0x100000000   raw8 -> dst+0xc0
  mask & 0x8000        raw8 -> dst+0xf8  AND  raw4 -> dst+0x100     <- TWO fields, read last
```

**Bits `0x02`, `0x08`, `0x400000`, `0x8000000`, `0x40000000` and everything above bit 32
are never tested.** Setting one costs nothing on the wire and does nothing on screen. **[L]**

### The shortest legal body is 18 bytes

With `inChat = 0` and `mask = 0`, `FUN_1408cfcf0` performs exactly four reads:
`1 + 8 + 1 + 8 = 18`. The `dst+0x10 == 0` branch reads `dst+0x24` **out of the struct**,
which the caller zeroed, so `test eax,eax / jle` skips the last conditional `u8`. **[L]**

Prepending the type byte, the whole `0x0089` body is **19 bytes**.

> **`inChat` costs a byte of its own, and that is the trap in this packet.** With
> `inChat != 0` the client reads a **fifth** field (`1408cfd7e`), and if that field is
> greater than zero a **sixth** (`1408cfd95`). A body that flips the byte without adding
> the extra `u8` is one byte short, which underruns the client's own packet reader — the
> failure `CLAUDE.md` records killing this client twice. `crates/net/src/message.rs`
> writes it, as a zero, and has a test named for it. This document said "18 bytes" for
> three drafts before the builder's own test caught it.

### What each mask bit is called

Matching the reader's bit -> offset table above against the string id `FUN_142d5da20` loads
for each offset. **[D]** — offsets from the reader [L], strings from the composer [L].

| bit | offset | string | text |
|---|---|---|---|
| `0x10` | `+0x30` | `0x0A52` | (outside the 0xC0..0xF0 block) |
| `0x20` | `+0x38` | `0x00C9` | `Bonus Wedding EXP (+%lld)` |
| `0x40` | `+0x40` | `0x00CA` | `Equip Item Bonus EXP (+%lld)` |
| `0x80` | `+0x48` | `0x00C6` | `PC Cafe Bonus EXP (+%lld)` |
| `0x100` | `+0x50` | `0x00C8` | `Rainbow Week Bonus EXP (+%lld)` |
| `0x200` | `+0x58` | `0x00CC` | `Boom Up Bonus EXP (+%lld)` |
| `0x400` | `+0x60` | `0x00CD` | `Potion Bonus EXP (+%lld)` |
| `0x1000` | `+0x70` | `0x00CF` | `Buff Bonus EXP (+%lld)` |
| `0x2000` | `+0x78` | `0x00D1` | `Rest Bonus EXP (+%lld)` |
| `0x4000` | `+0x80` | `0x00D2` | `Item Bonus EXP (+%lld)` |
| `0x8000` | `+0xf8`, `+0x100` | `0x00D8` | `%s EXP (+%lld)` — **the only two-field bit** |
| `0x10000` | `+0x88` | `0x00D4` | `%% additional EXP gained from the item (+%lld)` |
| `0x20000` | `+0x90` | `0x00D6` | `Value Pack Bonus EXP (+%lld)` |
| `0x40000` | `+0x98` | `0x00D5` | `%% additional Party Quest EXP …` |
| `0x80000` | `+0xa0` | `0x00C2` | **`You received bonus EXP (+%lld)`** |
| `0x100000` | `+0xa8` | `0x00D7` | `You have gained Kinship Ring bonus EXP (+%lld)` |
| `0x200000` | `+0xb0` | `0x00D9` | `You have gained the Icy Agent bonus EXP (+%lld)` |
| `0x800000` | `+0xb8` | `0x00DA` | `HP Risk EXP (+%lld)` |
| `0x1000000` | `+0xc8` | `0x00DB` | `Field Bonus EXP (+%lld)` |
| `0x2000000` | `+0xd0` | `0x00DC` | `Accumulated Hunt Bonus EXP (+%lld)` |
| `0x4000000` | `+0xd8` | `0x00DD` | `Event Bonus EXP (+%lld)` |
| `0x10000000` | `+0xe0` | `0x00DB` | `Field Bonus EXP (+%lld)` (again) |
| `0x20000000` | `+0xe8` | `0x13D8` | (outside the block) |
| `0x100000000` | `+0xc0` | `0x00CB` | `Guild Know-How Bonus EXP (+%lld)` |
| `0x01` | `+0x18` | — | not resolved |
| `0x04` | `+0x20` (u8) | — | not resolved |
| `0x800` | `+0x68` | — | not resolved |
| `0x80000000` | `+0xf0` | — | not resolved |

The main line is unconditional: `142d5dbec mov edx,0xc1 / call 1408a9e40` then
`14019ba10(&out, fmt, r8 = dst+0x08)` — string `0x00C1` `You received EXP (+%lld)` with the
`u64` from the body. **[L]** That is the whole feature; every mask bit is an extra line
under it.

---

## 4. Type 0, the drop pick-up — `FUN_142d59360`

Two `u8` reads, then a jump table. **[L]**

```asm
142d59380  call 1406e8ae0        ; u8  -> r12d   "accumulate" (see below)
142d59393  call 1406e8ae0        ; i8  -> the SUB-MODE, sign-extended
142d5939b  cmp  edx, 4    / je   142d59d35     ; mode 4  -> return, nothing
142d593a4  cmp  edx, -1   / jne  142d593b8
142d593a9  mov  dword [rsi+0x37a4], 1 / jmp    ; mode -1 -> set a latch, return
142d593b8  add  edx, 5    / cmp edx, 0xa / ja  142d59ced   ; default
142d593c7  jmp  [0x142d59d60 + (mode+5)*4]     ; 11 entries
```

| sub-mode | arm | body after the mode byte |
|---:|---|---|
| **0** | `142d59713` | **`u32 itemId, u32 count, u8 where`** — the item pick-up |
| **1** | `142d593da` | **`u8, i32 gain, u16, i32 bonus`** — the meso pick-up |
| 2 | `142d598ae` | `u32, u64` |
| 5 | `142d59c9f` | unread |
| -5..-2 | `142d59c4e`, `142d59bfd`, `142d59b93`, `142d59b42` | the refusals - decoded below, 2026-09-16 |
| 3, 4, -1 | | default / special-cased before the table |

### The refusals, and the one that is the inventory-full line (2026-09-16)

Jump table `142d59d60` read directly (11 dwords, image-relative), each arm's string id
decrypted with `tools/dump_stringids.py --id N`. Every arm but one posts to
`FUN_142572050` with `r9d = 2` - **the on-screen message area, the EXP line's printer** -
and none of them touches the chat log except -3's second string. **[L]**

| sub-mode | arm | string | text |
|---:|---|---|---|
| **-1** | `142d593a9` | *(latched)* `0x00F2` | **"You can't get anymore items."** - see below |
| -2 | `142d59b42` | `0x085B` | 'This item is unavailable for pick-up.' |
| -3 | `142d59b93` | `0x00F4`, then `0x00F5` to **chat 11** | 'You cannot acquire any items.' / '...because the game file has been damaged...' |
| -4 | `142d59bfd` | `0x00F3` | 'You cannot pick up this item.' |
| -5 | `142d59c4e` | `0x12CE` | "You can't pick that up." |
| 5 | `142d59c9f` | `0x12CF` | "You can't pick up the mesos, because you've already reached your maximum amount." |
| 3, out of range | `142d59ced` | `0x00F1` | 'Failed to acquire for an unknown reason.' |
| 4 | | | nothing |

**Mode -1 is the inventory-full line, and the client rate-limits it itself.** The handler
only sets `world+0x37a4 = 1` and returns. `FUN_142dada50` (one caller: the 22 KB field
update `FUN_1428923e0` at `14289247a`, guarded by `14289246c cmp eax, 0x7d0` against
`[r13+0x51f0]` - **once per 2000 ms**) reads the latch, posts `0x00F2` to the message area
and clears it. So a server can send `-1` on every refused pick-up - a pet retrying every
drop it stands on included - and the screen shows one line per two seconds and the chat
log nothing. `net::message::inventory_full()` is that body: `00 00 FF`. It replaced the
server's own yellow "Your bag would not take it" chat line on 2026-09-16 (the owner: *"use that
default behavior instead of our custom message"*).

### Sub-mode 0, the item

```text
u32  itemId       142d59716
u32  count        142d59720
u8   where        142d5972a     0 -> string 0xEC  '%s x%d earned. (%s)'
                                1 -> string 0xEE  '%s x%d earned. (%s and Bag)'
                                2 -> string 0xED  '%s x%d earned. (%s / Bag)'
                                anything else -> the handler ABANDONS the message
```

The client looks the item's name up itself (`FUN_140398ba0(itemStringTable, &name, itemId)`)
and the tab name too (`FUN_1403e5d10(&tab, itemId)`), and **bails out silently when the
name resolves empty** (`142d59752 test rcx,rcx / je`, `142d5975f cmp byte [rcx],0 / je`).
So an item id the client cannot name produces nothing at all. **[L]**

`142d59814 test esi,esi / jle` skips the formatting when `count <= 0`, which leaves the
output string null while the post still runs — **always send `count >= 1`.** **[L]**

### Sub-mode 1, the mesos

```text
u8   noticeLost   142d593e7   non-zero -> string 0xA6 'A portion was not found after
                              falling on the ground.' posted with printer type 7
i32  gain         142d593f3   the TOTAL, including `bonus`
u16  smallChange  142d593fe   non-zero -> string 0xE5 'Spotting Small Change (+%d)'
i32  bonus        142d5940a   > 0 -> 0xE2 'additional Mesos'; < 0 -> 0xE3 'Meso Penalty'
```

The plain line uses `gain - bonus`: `142d59482 mov ebx,r14d / sub ebx,edi`, then string
`0xE1` `You have gained mesos (+%I64d)`. **[L]**

Two things ride on the packet's **first** `u8` (`r12d`), which is common to every sub-mode:

* `142d596eb test r12d,r12d` — non-zero **accumulates** into `world+0x37a0` instead of
  calling `FUN_142d9bae0(world, gain)`, the meso-gain effect. So the first byte is a
  "quiet / batch this" flag. **[L]** for the branch, **[I]** for the name.
* A cap check runs first: `FUN_142cbec20(world)` against `0x746a5287fe`
  (499 999 999 998); over it, string `0x12D3` `You cannot obtain any more Mesos.` is shown
  and **no gain line at all**. **[L]**

### Answering the brief's third question

**Picking up an item is the same opcode with a different type byte**, not a different
opcode: `0x0089`, type `0`, sub-mode `0`. EXP is `0x0089`, type `3`.

---

## 5. Where the text lands: two destinations, and which byte picks

`FUN_142d5da20` posts through exactly two routines. **[L]**

```asm
; the fork, at the end of the composer
142d5efe9  cmp  dword [rbp+0xc0], 0        ; dst+0x10, the third body field
142d5eff0  jne  142d5f4d0                  ;   non-zero -> the FUN_1415eca30 route
142d5eff6  mov  rcx, [rip -> 0x143AD30D8]  ;   the singleton the other route needs
142d5effd  je   142d5f4d0                  ;   null -> fall back to FUN_1415eca30
142d5f006  ...
142d5f014  cmp  dword [rbp+0xb0], ebx      ; dst+0x00, "white"
142d5f01a  je   142d5f021                  ;   0        -> r8d = 4
142d5f01c  xor  r8d, r8d                   ;   non-zero -> r8d = 0
142d5f027  call 0x142572050                ; <- the other route
```

### `FUN_1415eca30(text, type)` is the chat log

`research/talking-back.md` established that **type 7 is the chat window**, from the
`[Welcome] Welcome to MapleStory!!` poster, and `0x00BB` — the packet this server uses
today — is exactly `FUN_1415eca30(&s, 7)`. **[L]** Three further readings agree that the
whole function is the chat log rather than one widget:

* `FUN_1415a3010`, which `FUN_1415eca30` reaches through `FUN_1415a87a0`, indexes
  **`this + 0x20 + type*24`** — one list per type, 36 of them — and trims a list that
  reaches **`0x1f4` (500)** entries down to `0x64` (100). A 500-line scrollback per
  category is a chat log, not a fading on-screen stack. **[L]**
* `FUN_1415b6c00(out, type)` is a **colour table**: types 1..0x24 index a jump table at
  `0x1415b6d60` whose arms are literal ARGB constants (`0xffff99cc`, `0xffbbbbbb`,
  `0xffffff00`, …). A per-type colour is what a chat category has. **[L]**
* `research/touch-damage.md` and `research/naked-character.md` both read other call sites
  of this function as "print into the chat window", at types `0xb` and `0xb` — and type 11
  is **723 of the 1133 sites**, i.e. the generic system-message channel.

Re-running `research/talking-back.md`'s own site enumeration as a check: scanning all
**1133** `call FUN_1415eca30` sites and reading the nearest preceding `mov edx, imm32`
reproduces its **17 type-7 sites at the same 17 addresses**, so the two scans are
equivalent instruments. **[L]** The same scan gives **42 type-6 sites**, and 10 of them are
inside the `0x0089` family (`FUN_142d43ee0` ×6, `FUN_142d5da20` ×4) with more in
`FUN_142d5ff40`, `FUN_142d600d0`, `FUN_142d60150`, `FUN_142d60400`, `FUN_142d5f7c0`,
`FUN_142d5f8b0`.

### `FUN_142572050` on singleton `0x143AD30D8` is the other place

* **51 call sites in 8 functions**, three of which are `FUN_142d43ee0`, `FUN_142d59360`
  and `FUN_142d5da20` — the `0x0089` family. It is not a general-purpose printer. **[L]**
* `tools/dataref.py 0x143AD30D8 --writes` gives one real writer, `FUN_142571a60`, a 1440-byte
  constructor that installs a function-pointer table at `0x14346F6A8`. **[L]**
* The **meso** arm of the pick-up handler posts *every* one of its three lines through
  `FUN_142572050` and through nothing else — no `FUN_1415eca30` call anywhere in it. A meso
  pick-up that never appears in the chat log is what the game does. **[L]**
* The client's RTTI carries exactly one class whose name matches `Msg`:
  **`.?AVCUIScreenMsg@@`** at `0x143aa2fe8`. **[I]** — I could not tie it to a vtable; a
  search of `.rdata`/`.data` for a CompleteObjectLocator naming that type descriptor returns
  nothing, and `tools/rtti.py --vtable` agrees ("0 locator(s)"). The name is suggestive and
  nothing more.

**So `dst+0x10` — the third body field of type 3 — is the switch the owner's request turns on:
`0` sends the line to the on-screen area and not the chat log; non-zero sends it to the
chat log as a type-6 line.** **[L]** for the branch and both destinations' identity;
**[I]** for "the on-screen area is specifically the bottom-right stack in the screenshot",
which is a match of behaviour to a picture, not a measurement.

With `mask = 0` and `inChat = 0` the whole handler produces **exactly one post** — the
main EXP line, on screen — because every bonus string stays empty and `142d5f488`'s
`test rcx,rcx / je` skips the only other output. **[L]**

### The item pick-up posts to the screen only — the chat-log copy is unreachable

Sub-mode 0 posts to `FUN_142572050` unconditionally (given the singleton exists) and
*additionally* to `FUN_1415eca30` type 6 when `FUN_141829fd0(FUN_141892840()) == 0x56` —
written obliquely as `lea edx,[rax-0x50]`. **[L]** There is no `inChat` byte in this
sub-mode.

**And that condition can never be true in this client.** `FUN_141892840()` is the current
field and `FUN_141829fd0` is its **type** — the same pair is compared against `0x35`/`0x48`
in `msexe-droppool.c` and `0x55`/`0x58`/`0xad`/`0xcd` in `msexe-invop-00ac.c`, i.e. a small
set of special map kinds. Enumerating `info/fieldType` across **all 426 map images**
(2026-08-21):

| fieldType | maps |
|---:|---:|
| 0 | 366 |
| 2 | 4 |
| 4 | 4 |
| 6 | 3 |
| 218 | 1 |
| 500 | 1 |
| absent | 47 |

**`0x56` (86) appears on none of them.** So an item pick-up in this client posts to the
bottom-right area and **not** to the chat log, always, and the server neither chooses that
nor can change it.

That is what the owner wants — *"when you pick up an item, it should only go to the bottom right
white text, it should not be in the chat log"* — so there is nothing to build. The earlier
wording here said a pick-up "may appear in both places", which was true of the code and
false of this client, and it read as a live hazard.

### The consequence for quest ITEM rewards, which is not what was hoped

The owner also asked for quest rewards to read `<Item> x<quantity> earned. (<Tab>)` **in the chat
log**. The wording is already right — sub-mode 0 draws string `0x00EC`, `'%s x%d earned.
(%s)'`, with `0x00ED`/`0x00EE` for the two bag variants — but the **destination is not
reachable**:

* sub-mode 0's only chat-log call is the `fieldType == 0x56` site above, which no map
  satisfies;
* `0x00BB` hard-codes `FUN_1415eca30(&s, 7)` and `crates/net/src/notice.rs` records that its
  colour and tab are not controllable;
* type 3's `inChat` byte routes **EXP** and nothing else.

So there is no decoded route today. **It may still exist**: `§2` of this document records
that **28 of the 36 `0x0089` sub-cases are unread**, and an item-in-chat sub-case among them
would not be surprising. Saying it exists would be a guess; saying it does not would be the
"searched a known list" mistake `CLAUDE.md` warns about. It is open, and it is a small,
well-shaped question: enumerate the 28.

> **Answered, 2026-08-21 — `research/message-subcases.md`.** All 36 were enumerated. The
> negative holds and is now *verified*: string `0x00EC` is loaded by exactly two functions in
> the whole image, and the `0x0089` one is the type-0 handler above. **But the second
> function is `FUN_1427863f0`, the `0x02D1` / `0x02AF` user-effect handler, and its effect
> `8` reads `u8 count, count × (u32 itemId, i32 quantity, u8 inBag)` and posts
> `'%s x%d earned. (%s)'` to chat category 6.** The enumeration also found `0x0089` sub-case
> **12** — `u32 chatType, str text` — which posts server text at a **server-chosen**
> category, so §5's "not controllable" applies to `0x00BB` and not to this packet. Chat
> category 6 is `0xFFBBBBBB`, grey; category 7, which `0x00BB` uses, is `0xFFFFFF00`,
> yellow. Both builders are in `crates/net/src/message.rs`; **neither is wired.**

---

## 6. Wire it like this

**Nothing here is wired.** `crates/net/src/message.rs` builds the bodies and has tests
pinning every byte; nothing in `crates/world` references it.

The coordinator owns `crates/world/src/session/`. Two edits:

**a. EXP for a kill.** Where the server currently sends a `notice::chat_notice` about
experience, send instead:

```rust
use net::message;

// after net::combat::stat_changed(&StatChange::exp_only(new_total)) - the two are
// independent: 0x007C moves the bar, 0x0089 draws the words.
replies.push((message::MESSAGE, message::exp_gained(gained)));
```

`exp_gained` sets `white = 0`, `in_chat = 0`, `mask = 0` — 19 bytes including the type
byte, the shortest body the client will accept, and it lands on screen and **not** in the
chat log.

**b. Item and meso pick-up.** Where the server currently sends a `chat_notice` about a
pick-up:

```rust
replies.push((message::MESSAGE, message::item_gained(item_id, count)));
// or
replies.push((message::MESSAGE, message::meso_gained(amount)));
```

`item_gained` asserts `count >= 1` (a zero count leaves the client formatting a null
string) and uses `where = 0`, the plain `'%s x%d earned. (%s)'` form.

**c. Level up.** There is **no level-up type in this table** that I read. The level-up
animation already comes free from `0x007C`'s own handler (`crates/net/src/stats.rs`), and
type 4 (`u16, u8`, strings `0xC3`/`0xC4`) is the *skill point* message, not a level
message. Do not invent one; drop the level-up `chat_notice` and let `0x007C` do it, or
keep a `0x00BB` for it deliberately and say so.

**d. Order and preconditions.** Unlike the quest sub-case, I did **not** find a
`world+0x2358` / singleton-bit gate at the top of `FUN_142d43ee0` — the type read and the
jump happen before anything else. But `research/quest-state.md`'s rule still applies to
sub-case 1 and costs nothing to honour here: send `0x0089` only from a settled field, not
alongside a `SetField`.

**e. What a wrong byte does.** The reader is unconditional up to the mask, so a body one
byte short underruns `FUN_1406e8f10` inside the client's packet reader — the failure mode
that has killed this client twice. The tests in `message.rs` pin all four field widths.

---

## 7. What I did NOT establish

* **That the type-6 / `FUN_142572050` destination is the bottom-right stack in the owner's
  screenshot.** I established that it is *not* where `0x00BB` goes, that mesos go there and
  only there, and that the family posting to it is exactly the EXP / meso / item / fame /
  SP family. The pixel location is inference. **The one-byte experiment settles it**: send
  type 3 with `in_chat = 0`; if the line appears bottom-right, done; if it appears in the
  chat log, flip that byte to 1 and it is the other way round. No other change.
* **Which widget `0x143AD30D8` is.** `CUIScreenMsg` is the only candidate name in the RTTI
  and has no locator, so the tie is unmade. `FUN_142571a60` is its constructor.
* **The colour parameter of `FUN_142572050`** (`r8d`, seen as 0, 4, 6 and 8 at different
  sites) and what `white` therefore does on screen.
* **28 of the 36 type values.** Types 2, 7..18, 20, 21, 24..28, 30, 31, 34, 35 are unread.
  Types **19, 29, 32 and 33 have no handler** — that much is [L] from the table.
* **Sub-mode 2 of the pick-up** (`u32, u64`, arm `142d598ae`). The refusals -5..-2, -1
  and 5 are decoded in §4 above.
* **Four of the EXP mask bits** (`0x01`, `0x04`, `0x800`, `0x80000000`) — their offsets are
  [L], their meanings unread.
* **Whether `0x0089` has an inbound-latch or field precondition.** I read the dispatch and
  the three handlers, not `FUN_142cbaa80`'s prologue.
* **Any level-up message type.** Searched the table, did not find one; that is a "did not
  find", not a "there is none".
