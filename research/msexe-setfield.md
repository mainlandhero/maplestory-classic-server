# Inbound game opcode `0x0070` — what it actually is

**It is not SetField.** `FUN_142d51930` is the client's **InventoryOperation /
ModifyInventoryItem** handler (`CWvsContext::OnInventoryOperation`). It never touches a
map, a field, a portal, a spawn point or a channel id. Every branch in it resolves to
"find the item at *(inventory type, slot)* in the local character's inventory, and put
something there / take something away / renumber it", followed by a UI refresh.

The argument, from what the code *does* with the values, is in
[Why it is not SetField](#why-it-is-not-setfield) at the bottom. Read that before using
anything above it, because the layout only makes sense under the right name.

Decompilation: `research/msexe-setfield.c` (file name kept for continuity — see the note
at the top of that file). Dispatch chain: `research/msexe-gamestage-dispatch.md`.

---

## Reading key

Every row is tagged:

* **read** — taken directly from the decompiled call sequence, and cross-checked against a
  raw disassembly dump of the same function (`DumpAsm`). No inference.
* **inferred** — a name or meaning I attached; the *size and position* are still `read`,
  only the label is a guess.

The primitives (already in `docs/opcodes.md`):

| function | reads |
|---|---|
| `FUN_1406e8ae0` | u8, `pos += 1` |
| `FUN_1406e8b80` | u16, `pos += 2` |
| `FUN_1406e8c20` | u32, `pos += 4` |
| `FUN_1406e8f10` | **u64, `pos += 8`** (confirmed in `research/msexe-decoders3.c`) |
| `FUN_1406e9050(p,&dst)` | string: u16 length, then that many bytes |
| `FUN_1406e9170(p,dst,n)` | n raw bytes |

Signedness below is what the *instruction* does with the return value (`MOVSX` vs `MOVZX`),
not a guess.

### Themida / decompiler caveats

`DumpAsm` over the whole 0x2d80-byte body of `FUN_142d51930` and over all six item
decoders reported **zero** `<no instruction>` regions and Ghidra emitted no "bad
instruction data" warning for any of them. Nothing in this packet is behind the VM. The
only decompiler artifacts are dropped arguments (`FUN_140303530(out)` /
`FUN_1403035a0()` / `FUN_140303800()` printed with fewer args than they take), and each of
those three call sites was checked in raw asm and does pass the packet pointer in RDX.

---

## Wire layout

### Header — always present, 7 bytes

| # | type | read/inferred | name | what the client does with it |
|---|---|---|---|---|
| H1 | u8 | read @ `142d51973` | `bExclRequestSent` *(inferred)* | if non-zero → `FUN_142cc4430(this,0)`: stores 0 at `this+0x2330` and two timestamps at `+0x2334`. This is the client's "your exclusive request is finished, unlock the UI" latch. |
| H2 | u8 | read @ `142d51989` | *(unknown; reference always sends 0)* | held to the very end of the handler; if non-zero **and** `DAT_143aa8518 != NULL` → `FUN_1428f4eb0(DAT_143aa8518, 0x58f, 1)` (a UI message). Consumes no extra bytes either way. |
| H3 | **i32** | read @ `142d51b2b` (`MOVSXD`) | `nCount` | number of entries. `TEST/JLE` → if `<= 0` the entire entry loop is skipped; the tail still runs. |
| H4 | u8 | read @ `142d51b3b` | `notRemoveAddInfo` *(inferred)* | only ever consulted inside mode 3, and only for items whose `itemID/10000 == 223`; if non-zero it skips one cosmetic call. |

Note H3 is a **4-byte** count, not the 1-byte count of old GMS builds.

### Entry — repeated `nCount` times, 4 bytes + a mode-specific tail

| # | type | read/inferred | name |
|---|---|---|---|
| E1 | u8 | read @ `142d51b88` (`MOVSX`, then `CMP …,0xC / JA`) | `mode` |
| E2 | **i8** | read @ `142d51b94` (`MOVSX`) | `invType` |
| E3 | **i16** | read @ `142d51ba4` (`MOVSX`) | `oldPos` |

`invType` values the code distinguishes (all **read**; names **inferred** from the v214
reference, which matches exactly): `1` Equip, `2` Consume, `3` Install, `4` Etc, `5` Cash,
`6` Decoration. Negative `oldPos` means an equipped slot.

`mode` selects the tail. **`mode >= 13` (or any byte ≥ 0x80, which sign-extends past the
unsigned compare) reads nothing further** — the entry ends after 4 bytes and the client
silently does nothing. This is a real desync trap, not a guess: the `JA` at `142d51bb9`
jumps straight to the loop tail at `142d52168`.

| mode | tail bytes | read site | name *(inferred, v214)* |
|---|---|---|---|
| 0 | **item blob** (variable) | `142d51bd9` | `Add` |
| 1 | i16 | `142d521fe` | `UpdateQuantity` |
| 2 | i16 `newPos` | `142d52528` | `Move` |
| 3 | — | — | `Remove` |
| 4 | **u64** | `142d53187` | `ItemExp` |
| 5 | **item blob** | `142d531eb` | `Lock` |
| 6 | — | — | `PresetChange` (writes `(i8)oldPos` to `charData+0x1a6`) |
| 7 | **u32** | `142d532e0` | `UpdateBagPos` |
| 8 | i16 — **conditional, see below** | `142d5342e` | `UpdateBagQuantity` |
| 9 | — | — | `BagRemove` |
| 10 | i16 | `142d5380f` | `BagToBag` |
| 11 | **item blob** | `142d53917` | `BagNewItem` |
| 12 | — | — | `BagRemoveSlot` |
| ≥13 | — (entry ends) | — | unhandled |

That is all 18 packet-read call sites in the function; the disassembly sweep found exactly
18 and no more, so there is no hidden read.

#### Mode 8's conditional short

Mode 8 reads its i16 **only if `FUN_140255650(oldPos, invType) != 0`**. That function is
fully decompiled and reads nothing; it is a pure predicate (**read**):

```
invType must be 2, 3 or 4                       (Consume / Install / Etc)
oldPos  must be  > 10100 (0x2774)
oldPos  must be <= 10340 (invType 2)
                 11040 (invType 3)
                 10740 (invType 4)
```

i.e. mode 8 only consumes its short for a **bag slot** position. `FUN_140255700` decodes
such a position as `bag = (pos-10000)/100 - 1`, `slot = (pos-10000)%100 - 1`, and
`FUN_140255590` gives the bag count per type: 3 / 10 / 7 for Consume / Install / Etc.
Sending mode 8 with an ordinary slot number desyncs the stream by two bytes.

Modes 5, 9, 10 and 11 also call `FUN_140255650`, but only to choose which container to
write into — no read depends on it there.

### Trailing byte — conditional

| type | read site | name *(inferred)* |
|---|---|---|
| u8 | `142d5219f` | `addMovementInfo` |

Guard, straight out of the asm at `142d52187`:

```
if (avatarChanged) {
    if (DAT_143aa8518 != NULL) {        // the local-user global
        u8 b = Decode1();
        FUN_1428a7f00(DAT_143aa8518, b);
    }
}
```

`avatarChanged` starts false and is set true in exactly two places (verified — there are
only two assignments in the whole function):

* **mode 2**, when `(invType == 1 || invType == 6) && (oldPos < 0 || newPos < 0)`.
  Unconditional — this fires on the wire values alone.
* **mode 3**, when `(invType == 1 || invType == 6) && oldPos < 0` **and the client already
  holds an item at that (invType, oldPos)**. The `bVar5 = true` sits inside
  `if (item != NULL)`.

**Practical rule for the server:** it is safe to append the trailing byte when you sent at
least one mode-2 entry matching the first condition. Driving it from mode 3 alone makes
frame length depend on client-side inventory state, which you cannot see — avoid it.
There is also the `DAT_143aa8518 != NULL` term, which is a client global, not wire data; in
the game stage it is populated, but it is a second reason not to lean on this byte.

---

## The item blob

`FUN_140303530(out, pkt)` — used by modes 0, 5 and 11.

```
u8 slotType                                   [read]
  1 -> Equip   decode = FUN_140304100         (vtbl 14327e1d8 + 0x358)
  2 -> Bundle  decode = FUN_140304450         (vtbl 14327e588 + 0x358)
  3 -> Pet     decode = FUN_140304550         (vtbl 14327e910 + 0x358)
  anything else -> item = NULL and NOTHING FURTHER IS READ
```

The `else` arm of `FUN_1402cc180` returns null without consuming a byte, and
`FUN_140303530` then skips the virtual `Decode` entirely. A bad slot type does not throw;
it silently truncates the entry.

### Common head — `FUN_1403035a0` (all three call it first)

| type | notes |
|---|---|
| u32 | item id — stored obfuscated at `+0x20/+0x28`, read back everywhere via `FUN_14019a5d0(this+0x20)` |
| u8 | `bHasSN` *(inferred)* |
| 8 bytes | **only if `bHasSN != 0`** (`14030378e  TEST AL,AL / JZ`), else the field is zeroed |
| 8 bytes | expiry — a FILETIME *(inferred)* |
| u32 | |
| u8 | |

18 bytes if `bHasSN == 0`, 26 if not.

### Bundle — `FUN_140304450`

| type | notes |
|---|---|
| *(common head)* | |
| i16 | quantity *(inferred)* |
| **13 raw bytes** | fixed-width buffer; the client force-NULs the 13th byte |
| i16 | |
| u8 | |
| 8 bytes | **only if** `itemID/10000 == 207` **or** `== 233` (`0x1f95f0`/`0x238d90` = 2 070 000 / 2 330 000 — throwing stars and bullets) |
| u32 | |

### Pet — `FUN_140304550`

| type |
|---|
| *(common head)* |
| **13 raw bytes** (pet name *(inferred)*) |
| u8 |
| i16 |
| u8 |
| **8 raw bytes** |
| i16 |
| i16 |
| u32 |
| i16 |
| u8 |
| u32 |
| i16 |
| i16 |
| u32 |

No conditionals at all. 48 bytes after the head.

### Equip — `FUN_140304100`

In order:

| # | type | notes |
|---|---|---|
| 1 | *(common head)* | `FUN_1403035a0` |
| 2 | **stat block** | `FUN_140303b40` — variable, see below |
| 3 | 13 raw bytes | title / engraving *(inferred)* |
| 4 | u8 | |
| 5 | u8 | |
| 6 | i16 × 7 | |
| 7 | 8 raw bytes | **only if the common head's `bHasSN` was 0** (`CMP qword [RSI+0x38],0 / JNZ` at `14030428a`). Note the *inverse* sense — combined with the head, an equip always costs the same 8 bytes one way or the other. |
| 8 | 8, 8, u32, u32, u32, u32 | `FUN_1402cce00`, 32 bytes, unconditional |
| 9 | 8, u32 | `FUN_1402cd090`, 12 bytes, unconditional |
| 10 | u32 | |
| 11 | i16 × 3 | |
| 12 | **android block** | **only if `itemID/10000 == 166`** — `FUN_1402cb4f0`: i16, u32, u32, **string (u16 len + bytes)**, u32, 8 raw bytes, u32 |
| 13 | u8 | |
| 14 | u8 | |
| 15 | **optional-short block** | `FUN_140303800` |
| 16 | u8 | gate for the next row |
| 17 | **optional-short block** | **only if row 16 != 0** |

#### Optional-short block — `FUN_140303800`

```
u32 mask
for b in 0..16:            // 17 bits, LSB first
    if mask & (1<<b):  i16
```

Minimum 4 bytes, maximum 38. Bits 17..31 are ignored and consume nothing.

#### Equip stat block — `FUN_140303b40`

```
optional-short block      (FUN_140303800 — mask + up to 17 shorts)
u32 flags
```

then, in ascending bit order, one field per set bit (all sizes read from the asm):

| bit | size | | bit | size | | bit | size |
|---|---|---|---|---|---|---|---|
| 0 | u8 | | 7 | u32 | | 14 | u8 |
| 1 | u8 | | 8 | u8 | | 15 | u8 |
| 2 | i16 | | 9 | i16 | | 16 | u8 |
| 3 | u8 | | 10 | u32 | | 17 | u8 |
| 4 | u8 | | 11 | u8 | | 18 | u8 |
| 5 | **u64** | | 12 | u8 | | 19 | **u64** |
| 6 | u32 | | 13 | u8 | | 20 | u32 |

Bits 21..31 consume nothing. Minimum 8 bytes for the whole stat block
(empty short-mask + zero flags).

### Smallest legal blobs

Sending `flags = 0` and `mask = 0` everywhere:

| slot type | bytes (incl. the leading slotType byte) |
|---|---|
| Bundle | **41** |
| Pet | **67** |
| Equip | **125** |

So the smallest complete `0x0070` that adds one stackable item is
`7 (header) + 4 (entry) + 41 (blob)` = **52 bytes of body**.

---

## Why it is not SetField

Argue from behaviour, not names:

1. **The object it operates on is the inventory.** The first thing the handler does after
   the header is `local_430 = FUN_142cbe730(this)`, which is a one-line getter returning
   `*(this + 0x2358)` — the character-data pointer. Every case then calls one of
   `FUN_1402e3cd0(cd, out, invType, pos)` (get item), `FUN_1402e4c20(cd, invType, pos, ref)`
   (set item — passing a null ref is how mode 3 removes), `FUN_14022f570` /
   `FUN_14022f680` (the same pair for bag slots), or `FUN_1402e5020` (remove). None of
   these takes a map id, and none of them is a field/stage call.

2. **The mode set is an inventory verb list.** The 13 cases map one-for-one, in value
   order, onto the v214 reference's `InventoryOperation` enum — `Add, UpdateQuantity,
   Move, Remove, ItemExp, Lock, PresetChange, UpdateBagPos, UpdateBagQuantity, BagRemove,
   BagToBag, BagNewItem, BagRemoveSlot` — *and* the payload of each matches: item blob for
   0/5/11, i16 for 1/2/10, i64 for 4, i32 for 7, nothing for 3/6/9/12.

3. **The header matches that reference field for field.** `WvsContext.inventoryOperation`
   in `ModernMapleSource/v214 src` encodes `byte exclRequestSent; byte 0; int size;
   byte notRemoveAddInfo;` then per entry `byte type; byte invType; short oldPos;` and
   finally `if (addMovementInfo != 0) byte addMovementInfo`. That is exactly the four
   header reads, the three entry reads and the one conditional trailing read found in the
   binary, including the guard `(invType == EQUIP || invType == DECORATION) && (oldPos < 0
   || newPos < 0)` which the client spells as `invType == 1 || invType == 6`. Two
   independent instruments agree.

   (`H4`/`notRemoveAddInfo` is a particularly good check: the reference's *name* predicts
   "if set, do not remove the add-info", and the binary uses that byte in exactly one
   place — inside mode 3 (`Remove`) — to skip one call. It could not have been guessed
   from the binary alone, and it fits.)

4. **Nothing in the handler loads or changes a field.** The tail refreshes UI
   (`FUN_142ce53e0`, `FUN_142d9b200`, `FUN_142ce5e60`), fires item-gained popups, and
   diffs a "before" item-count map against an "after" one to decide which item to show in
   the quick-slot. A SetField handler would have to build the map, place the avatar and
   restart the field renderer; there is no such call anywhere in the 11 648 bytes.

5. **The reference's opcode ordering says the same thing.** In `ins.txt`,
   `BEGIN_CHARACTERDATA` and `InventoryOperation` share the value `0x37` — i.e.
   InventoryOperation is the *first* opcode of the character-data block, which is exactly
   the position `0x0070` holds in `FUN_142cbaa80` (its lowest case). `SetField` there is
   `0x167`, immediately after `END_CHARACTERDATA = 0x166` — a completely different part of
   the range. Treat this as corroboration only; the shift between the two clients is known
   not to be constant.

### So where is SetField?

Not established. Two things worth recording, both cheap by-products of this work:

* **`gamecases.txt` is incomplete and should not be used as an opcode census.**
  A scan of the decompiled `FUN_142cbaa80` finds at least **273** `case` labels; that file
  lists 181, and **96** of the labels are missing from it. (The scan is itself imperfect —
  four rows in `gamecases.txt`, `0xc8 0x12b 0x12c 0x190`, are labels the scan did not
  match — so the true label count is ≥277. The point stands either way.) The missing ones
  are the cases whose body is inlined rather than a single call — e.g. `0xa3, 0xa4, 0xa5, 0xaa,
  0xb2, 0xb3, 0xb7, 0xb8, 0xbe…0xc0, 0xc9, 0xcb…0xd1, 0x121…0x126, 0x13a…0x140, 0x146,
  0x14d, 0x275, 0x39a`. Any conclusion drawn from that file's *gaps* is unsafe.
  (`0x71`–`0x7a`, however, really are absent — no case label at all, so they fall through
  to `default` and are ignored.)

* **The full character-record decoder is `FUN_140304b20`** (18 525 bytes; it is the only
  caller of the already-documented `FUN_140302e30` character-stat decoder). Inside the
  game-stage dispatcher it is reached from exactly one case, **`0x14d`**, whose body
  decodes a record into `this[0x46b]` and then opens a UI window via `FUN_14085b3b0` —
  a "character info / other player's stats" packet, not SetField. `FUN_142d01120` is an
  out-of-line duplicate of that same body and has no direct callers (vtable-reached).

  This is informative: **if SetField in this client carried a full character record, it
  would show up as a third caller of `FUN_140304b20`, and it does not.** Either this
  client's SetField never sends the record (only the short "same-character, new map" form:
  `type / fieldID / portal / hp / …`), or its handler is one of the 92 inline cases and
  reaches the record by a path Ghidra did not resolve.
