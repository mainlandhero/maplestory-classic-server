# The bag lists in the character record

**Written 2026-08-19, entirely statically. No client run was spent.** Every row is tagged
**[L]** (read off the client's own listing or its own bytes), **[D]** (derived by arithmetic
from [L] rows) or **[I]** (inferred, and not measured here).

This answers goal I's second half - "getting it back onto the wire, which is not decoded" -
for the one bag that matters today, and it **corrects the framing of the question**.

---

## 0. Read this first: two things the brief assumed that are not true

### 0.1 The four lists are not four bags

`presence[2]` opens four `u16`-terminated lists, and the brief's working assumption was that
they are inventory tabs 1, 2, 3 and 4. They are not.

| list | reader | where the item lands | position range |
|---|---|---|---|
| 1 | `FUN_14030b6f0(closure, 1)` @ `0x1403062ee` | `charData + 0x5d8` = **inventory type 1, the Equip tab** | `1 ..= inventorySize[1]` |
| 2 | `FUN_14030b9e0` outer index 2 | `charData + 0x5b8` | **3000 ..= 3031** |
| 3 | `FUN_14030b9e0` outer index 3 | `charData + 0x5c0` | **3100 ..= 3131** |
| 4 | `FUN_14030b9e0` outer index 4 | `charData + 0x5c8` | **3200 ..= 3231** |

**[L]** for all four. The three `FUN_14030b9e0` lists are **not inventory tabs at all** -
they are three of five equipped-like containers that sit immediately before the six
inventory pointers, and their positions are absolute values in three disjoint ranges
starting at 3000, 3100 and 3200. Nothing a starter character can hold goes there.

### 0.2 The Use / Set Up / Etc / Cash bags ARE in the record - behind their own presence bytes

`FUN_14030b6f0` has **six** call sites, not one, all in `FUN_140304b20` **[L]**
(`python tools/callers.py 0x14030b6f0`):

```text
1403062ee  CALL 0x14030b6f0   EDX = 1     inside the presence[2] region
1403062ff  CALL 0x14030b9e0   EDX = 1     inside the presence[2] region
140306815  CALL 0x14030b6f0   EDX = 6     inside the presence[44] region
140306826  CALL 0x14030b9e0   EDX = 6     inside the presence[44] region
140306845  CALL 0x14030b6f0   EDX = 2     ungated at this level
140306855  CALL 0x14030b6f0   EDX = 3     ungated at this level
140306865  CALL 0x14030b6f0   EDX = 4     ungated at this level
140306875  CALL 0x14030b6f0   EDX = 5     ungated at this level
```

The last four are always *called* - a closed `presence[44]` jumps straight to `0x140306830`
at `0x14030664f` **[L]** - but each **re-gates internally** on its own presence byte and
reads nothing when that byte is clear. That is why today's record, which sets only
`presence[0]`, `[2]` and `[7]`, is byte-exact with four terminators and is known-good on a
real client.

So the asymmetry the brief warned about is **not** "six sizes, four lists". It is:

> **six sizes, six list readers, and only one of the six is switched on.**

Section 6 gives the price of switching the others on, and it is not one byte.

---

## 1. How a bag list is gated

Both helpers start with the same three steps **[L]**:

```asm
14030b722  MOV  EDX,R15D           ; param_2, the inventory type
14030b725  LEA  RCX,[RSP + 0x50]
14030b72a  CALL 0x1403023d0        ; type -> gate key
14030b73d  CALL 0x1402fa9a0        ; 100-byte AND of the record's presence array with the key
14030b746  CMP  byte ptr [RAX],BPL ; scan 100 bytes for any non-zero
14030b755  JMP  0x14030b9b5        ; all zero -> return, reading nothing
```

`FUN_1403023d0` is a six-way jump table on `type - 1` (table at `0x1403024a4`) **[L]**,
`research/msexe-invkey-23d0.txt`. Each key's presence byte was read out of the key's own CRT
initialiser - `mov byte ptr [key + n], 1` - by scanning every `C6 05` RIP-relative byte store
in `.text` whose target lands in any of the nine key objects:

| inventory type | tab | gate key | initialiser | **presence byte** |
|---:|---|---|---|---:|
| 1 | Equip | `0x143abdb20` | `0x140022952` | **2** |
| 2 | Use | `0x143abdab0` | `0x140022932` | **3** |
| 3 | Set Up | `0x143abda40` | `0x1400229b2` | **4** |
| 4 | Etc | `0x143abd9d0` | `0x140022992` | **5** |
| 5 | Cash | `0x143abd960` | `0x140022912` | **6** |
| 6 | Deco | `0x143abd8f0` | `0x140022972` | **44** |
| other | - | `0x143abd880` | *(none)* | never fires |

All **[L]**. **The instrument carried two positive controls and both passed**: the same scan
reports `0x143abedb0` byte **2** at `0x140023442` and `0x143abeb80` byte **44** at
`0x140023462`, which is exactly what `research/naked-character.md` §2 and
`research/charrecord-presence-map.md` established by different means. And the whole column
reproduces `research/inventory-slots.md`'s six-turn size-loop order (2, 3, 4, 5, 6, 44) from
a completely different table. Two independent measurements, same permutation.

The tab **names** are still `inventory-slots.md`'s - Equip is measured, the other five came
off the owner's screenshot of a six-tab window. **[D]** for Equip, **[I]** for the rest. Nothing
on the wire depends on the names.

---

## 2. `FUN_14030b6f0` - one list, into inventory type `param_2`

Listing: `research/msexe-invlist-b6f0.txt`. Every row **[L]**.

```text
14030b768  MOV RAX,[RSI + 0x8]          ; the character-data object
14030b76c  LEA R14,[RAX + R15*0x8]      ; + type*8
14030b770  MOV RAX,[R14 + 0x5d0]        ; charData + 0x5d0 + type*8  = the inventory array
14030b77e  MOV ECX,[RAX + -0x8]         ; its element count
14030b781  DEC ECX                      ; slotMax = count - 1
14030b78d  CALL 0x1406e8b80             ; u16 pos
14030b792  MOVZX ECX,AX                 ; ZERO-extended  -> UNSIGNED
14030b79f  CMP dword ptr [RAX],EBP      ; EBP = 0
14030b7a1  JZ  0x14030b9b5              ; pos == 0  -> end of list
loop:
14030b7bd  CALL 0x1403095e0             ; the pooled item factory: u8 type, then vtable+0x358
14030b7c9  CMP ECX,0x1 / JL  discard    ; pos < 1
14030b7d6  CMP ECX,[RAX] / JG discard   ; pos > slotMax
14030b8c0  SHL RDI,0x4 / ADD RDI,RAX
14030b92b  MOV [RDI + 0x8],RBX          ; inventory[pos] = item
14030b99a  CALL 0x1406e8b80             ; u16 pos
14030b99f  MOVZX ECX,AX
14030b9af  JNZ loop
```

* **`param_2 = 1` for the call inside the `presence[2]` region** (`MOV EDX,0x1` at
  `0x1403062e2`), so the array is `charData + 0x5d8` - index 1 of the six the size loop at
  `0x140305de8` walks (`ADD RSI,0x5d8`, `research/inventory-slots.md`). **The list is the
  Equip tab's bag.** **[L]**
* **The position is an unsigned `u16`** - `MOVZX`, twice, at `0x14030b792` and
  `0x14030b99f`. **[L]**
* **The terminator is `0`.** **[L]**
* **Storable positions are `1 ..= slotMax`**, where `slotMax` is the array's count minus one
  - i.e. exactly the `V` sent as that inventory's size in the `presence[7]` block, because
  the size decoder resizes the array to `V + 1` (`research/inventory-slots.md`). A position
  outside the range is **decoded and thrown away**: the bytes are consumed, the item is
  dropped. Same failure shape as an equipped slot outside 1..31. **[L]**
* The per-entry shape the brief asked me to confirm rather than assume is confirmed:
  **`u16 pos` then an item body, terminated by `u16 0`.** **[L]**

`R12 = ((unsigned)(param_2 - 5) <= 1)` at `0x14030b764` selects a different store path for
types 5 and 6 (Cash, Deco). For type 1 it is 0 and the plain path above is taken. **[L]**

---

## 3. `FUN_14030b9e0` - three lists, and they are not bags

Listing: `research/msexe-invlist-b9e0.txt`.

**The outer loop runs five times, indices 0..4** - `MOV dword ptr [RSP+0x20],0` at
`0x14030ba58`, then `INC EAX / CMP EAX,0x5 / JL 0x14030ba70` at `0x14030be4e`. **[L]**
This is read off the listing, not off `naked-character.md`'s summary, as the brief asked.

It proceeds only where `FUN_140255790(index) == R15`, with
`R15 = ((unsigned)(param_2 - 5) <= 1)` at `0x14030ba4d` - so **0 for `param_2 = 1`** and 1
for `param_2 = 5` or `6`. **[L]**

`FUN_140255790` has no `.pdata` entry, so it was read as raw bytes at `0x140255790`:

```asm
85 c9        TEST ECX,ECX
74 08        JZ   +8        -> mov eax,1 ; ret
83 f9 01     CMP  ECX,1
74 03        JZ   +3        -> mov eax,1 ; ret
33 c0 c3     XOR  EAX,EAX ; RET
```

**It returns 1 for index 0 and 1, and 0 for everything else.** **[L]** - confirming
`naked-character.md`'s claim from the bytes rather than trusting it.

So the call with `param_2 = 1` visits **indices 2, 3 and 4**: three lists. **[L]**

Each list reads exactly like `b6f0`'s - `u16 pos` (`MOVZX` at `0x14030bc3b` and
`0x14030be31`), `0` terminates, item body from `FUN_1403095e0` at `0x14030bc5d` **[L]** -
but the position is checked against a **pair of range tables** rather than an inventory size:

```asm
14030bc6d  CMP EDI,dword ptr [RBX + RSI*4 + 0x327dd50]   ; lower bound, JL  -> discard
14030bc7a  CMP EDI,dword ptr [RBX + RSI*4 + 0x327dd68]   ; upper bound, JGE -> discard
14030bd2b  SUB EDI,R14D                                  ; index = pos - lowerBound
```

Read straight out of `client-patched/MapleStory.exe` (`python tools/dump_va.py 0x14327dd50 64`):

| index | container | lower `0x14327dd50` | upper `0x14327dd68` | slots | visited when |
|---:|---|---:|---:|---:|---|
| 0 | `charData + 0x5a8` | 1200 | 1214 | 14 | `param_2` is 5 or 6 |
| 1 | `charData + 0x5b0` | 1800 | 1851 | 51 | `param_2` is 5 or 6 |
| 2 | `charData + 0x5b8` | **3000** | **3032** | 32 | **`param_2 = 1`** |
| 3 | `charData + 0x5c0` | **3100** | **3132** | 32 | **`param_2 = 1`** |
| 4 | `charData + 0x5c8` | **3200** | **3232** | 32 | **`param_2 = 1`** |

All **[L]**.

### What these five containers are - NOT established

The arithmetic places them exactly: the record's inline equipped list stores at
`charData + 0x1a8 + slot*0x10` for 32 slots (`0x1a8 .. 0x3a7`), the `presence[44]` block's
second equipped list at `charData + 0x3a8 + slot*0x10` for 32 slots (`0x3a8 .. 0x5a7`), then
five pointers at `0x5a8 .. 0x5cf`, then the seven inventory pointers at `0x5d0 .. 0x607` of
which 1..6 are used. **[D]** from the listing.

So they are five more **equipped-like** arrays, and their split (0 and 1 for the Cash/Deco
call, 2/3/4 for the normal call) is a real distinction the client makes. **What they hold is
not established.** I did not find a name, a string, or a WZ property for them, and I am not
guessing from the reference server: `CLAUDE.md` scores it 1 of 8.

**What matters for the encoder is settled regardless:** their positions live in
`[3000, 3232]`, no starter item is addressed there, and three `u16 0` cost six bytes and put
nothing anywhere.

---

## 4. `flagA` still has to be 0, and it is measured

```asm
1403061c9  MOV  RCX,R14
1403061cc  CALL 0x1406e8ae0        ; u8   the region's leading byte
1403061d1  TEST AL,AL
1403061d3  SETNZ SIL               ; flagA
...
1403062dd  TEST SIL,SIL
1403062e0  JNZ  0x1403062f3        ; non-zero  ->  SKIP the b6f0 call entirely
1403062e2  MOV  EDX,0x1
1403062ee  CALL 0x14030b6f0
1403062f3  MOV  EDX,0x1
1403062ff  CALL 0x14030b9e0        ; never skipped
```

**[L]**, confirming `naked-character.md`. A non-zero `flagA` costs the whole Equip-bag list -
with `flagA != 0` there are three lists, not four, and there is no way to send bag contents
for the Equip tab. **`flagA` must stay 0.** `crates/net` already sends 0.

---

## 5. The item body

`FUN_1403095e0(out, packet, pool)` reads a `u8 type` at `0x1403095fb` and dispatches to a
per-type allocator, then calls `(*(item->vtbl + 0x358))(item, packet)`. **[L]**

The three vtables, re-derived here from the constructors' own `LEA` displacements rather
than taken from the earlier document, with the `vtable + 0x88` control:

| type | ctor | vtable | `+0x88` reads | `+0x358` decode | length |
|---:|---|---|---|---|---:|
| 1 | `FUN_1402f7da0` | `0x14327E1D8` | `0x1402fc5e0` = `b8 01 00 00 00 c3` | `0x140304100` | **125** |
| 2 | `FUN_1402f7cd0` | `0x14327E588` | `0x1402fc5d0` = `b8 02 00 00 00 c3` | `0x140304450` | **41** |
| 3 | `FUN_1402f8b40` | `0x14327E910` | `0x1402fc5f0` = `b8 03 00 00 00 c3` | `0x140304550` | not decoded |

All **[L]**. Type 3's vtable and decode are new here; `research/msexe-setfield.md` had the
decode address but not the vtable.

### 5.1 Type 1, the equip - already built

`net::opcode::equipped_item`, 125 bytes with all-zero masks, confirmed on screen 2026-08-19.
**An equip in a bag is still a type-1 item**, so the Equip-tab bag needs no new item encoder
at all. That is the whole reason goal I's wire half is small.

### 5.2 Type 2, the bundle - new

`FUN_140304450`, listing `research/msexe-itemslot-bundle-decode.txt`. Every row **[L]**:

```text
len  read at      what                                        send
---- ----------- ------------------------------------------- ------------------------
  1  1403095fb   u8   item type                               2
---- FUN_1403035a0, the base decode, research/msexe-itemslot-base.txt --------------
  4  1403035c5   u32  itemId                                  2000000 / 4000001 / ...
  1  140303787   u8   hasCashSN                               0
 (8) 14030379d   raw[8] cash serial  ONLY if hasCashSN != 0   omitted
  8  1403037b9   raw[8] dateExpire                            ITEM_NEVER_EXPIRES
  4  1403037c1   u32  -> +0x48                                0
  1  1403037cc   u8   -> +0x4c (bool)                          0
---- back in FUN_140304450 --------------------------------------------------------
  2  14030446d   u16  QUANTITY   -> +0x4d/+0x51               the stack size
 13  14030448e   raw[13] a char[13] name buffer -> +0x6d      0 x13
  2  14030449a   u16  attribute flags -> +0x55/+0x59          0
  1  1403044b1   u8   -> +0x5d/+0x5e/+0x61                     0
 (8) 14030451e   raw[8] -> +0x65   ONLY if itemId is a star   omitted for normal items
                 or bullet: 2070000..2079999 or 2330000..2339999
  4  140304526   u32  -> +0x7a                                0
```

**Total 41 bytes** for an ordinary stackable, **49** for a throwing star or bullet. **[D]**

**The base decode does NOT include `FUN_140303b40`.** The two equip bitmasks belong to
`FUN_140304100`, not to `FUN_1403035a0` - which is why a bundle is 41 bytes and not 49.
Enumerated rather than filtered: the only calls in `[0x1403035a0, 0x1403037f5]` that touch
a read primitive are `2x 0x1406e8c20`, `2x 0x1406e8ae0`, `2x 0x1406e9170`. `tools/reads.py`
at depth 2 agrees (`u32, u8, raw, raw, u32, u8`), and the positive control
`python tools/reads.py 0x140304100 2` returns the full 25-row equip list first.

**Two independent instruments agree on the field order.** `FUN_1402cf5c0` is the client's own
**encoder** for the same class - it is vtable-only, so `tools/callers.py` reports 0 callers -
and it emits, in order: base encode, `u16` from `+0x4d/+0x51`, `raw[13]` from `+0x6d`, `u16`
from `+0x55/+0x59`, `u8` from `+0x5d/+0x61`, the same star/bullet-gated `raw[8]` from `+0x65`,
and a tail-jumped `u32` from `+0x7a`. **[L]**, decoded from the bytes at `0x1402cf5c0`.

#### Which `u16` is the quantity - measured, not taken from the reference server

The two `u16` are symmetric on the wire, so getting them the wrong way round would be
invisible until a client run. It is settled from the **vtables**:

| vtable slot | type 1 (equip) | type 2 (bundle) | type 3 (pet) |
|---|---|---|---|
| `+0x98` | `0x1402fbe80` = `mov eax,1 ; ret` | `0x1402fbe60` | `0x1402fbe90` = `mov eax,1 ; ret` |

and `FUN_1402fbe60` is five instructions:

```asm
1402fbe64  MOV  EDX,dword ptr [RCX + 0x51]
1402fbe67  ADD  RCX,0x4d
1402fbe6b  CALL 0x1401b00e0            ; the 16-bit de-obfuscator
1402fbe70  MOVZX EAX,AX
1402fbe77  RET
```

So slot `+0x98` is "how many of this item is here": **1 for an equip, 1 for a pet, and the
`+0x4d/+0x51` field for a bundle** - and that field is the **first** `u16` after the base
decode, at `0x14030446d`. **[L]**

The other `u16` (`+0x55/+0x59`) is a **bit field**, not a count: `FUN_1402fdab0` reads it
through the same de-obfuscator and immediately does `TEST AL,0x2` at `0x1402fdace`. **[L]**

#### The values to send, and why

* `hasCashSN = 0`. Unlike the equip decode, **the bundle has no compensating `raw[8]`** - a
  non-zero `hasCashSN` makes the body 8 bytes *longer*, not the same length. **[L]**
* `dateExpire` = `net::opcode::ITEM_NEVER_EXPIRES`, the same constant the equip body uses.
  **[I]** by convention, unchanged from `naked-character.md` §3.4.
* the `raw[13]`, the trailing `u32`, and the `u8` are all **0**, and that is what the
  constructor writes: `FUN_1402f7cd0` ends with `XOR EAX,EAX / MOV [RSI+0x65],RAX /
  MOV [RSI+0x6d],AL / MOV [RSI+0x7a],EAX`. **[L]** A fresh bundle has an empty name buffer
  and a zero tail, so zeros are the constructed state and not a guess.
* the attribute `u16` is **0**. Bit 1 is read by `FUN_1402fdab0`; the rest is unread here.

### 5.3 Type 3 - pets, and it is not needed

`FUN_140304550` (`vtable + 0x358` of `0x14327E910`) reads the base plus **14** more fields
(`tools/reads.py 0x140304550 3`): `raw`, `u8`, `u16`, `u8`, `raw`, `u16`, `u16`, `u32`,
`u16`, `u8`, `u32`, `u16`, `u16`, `u32`. `research/msexe-setfield.md` calls it the **pet**
decode. **[I]** for the name; **[L]** for the read list.

**Nothing a starter character can hold is a type-3 item**, and no path in `crates/store`
creates one. Not decoded, deliberately.

---

## 6. The price of switching on the other five bags - and it is not one byte

The brief's "if the four lists turn out to be types 1,2,3,4 then Cash and Deco are not in
the record at all - say so loudly" has an answer, and it is a different loud one:

> **Every tab is in the record. Setting `presence[N]` to send tab N's bag also opens
> whatever else that byte gates, and for three of the five it opens a block that has never
> been decoded.**

Measured by joining `research/charrecord-presence-map.md`'s gate table against the two
dynamic-gate loops in `FUN_140304b20`, then enumerating read primitives in each region off
`research/msexe-charrecord-full.txt`:

| byte | tab | what it opens **besides** the `b6f0` list |
|---:|---|---|
| **2** | Equip | the equipped-item block itself and `b9e0`'s three lists. **Nothing else** - the two dynamic-gate loops both start at type 2, and byte 2's other static gate `0x140305776` guards a region with 0 reads. This is why today's record is already correct. **[L]** |
| **3** | Use | `[0x140305104, 0x1403051b6)` - **4x `u32` + 1x `u64` in a count-driven loop**, gate `0x1403050bd`, entry 5. **And** `[0x140306a0a, 0x140306b44)` - a `u32` count then, per entry, a `u32` and a **whole item** via `FUN_1402dddb0` -> `FUN_140303530`. **[L]** |
| **4** | Set Up | `[0x140306a0a, 0x140306b44)`, as above. **[L]** |
| **5** | Etc | `[0x140306a0a, 0x140306b44)`, as above. **[L]** |
| **6** | Cash | nothing else found. The two dynamic loops cover types 2..4 only (`INC EDI / CMP EDI,0x4 / JLE` at `0x14030695a`; `INC R15D / CMP R15D,0x4 / JLE` at `0x140306b44`). **[L]** |
| **44** | Deco | the entire second equipped block at `0x14030661c` - its own `u8` flag, a 32-slot equipped list into `charData + 0x3a8`, `b6f0(6)` and `b9e0(6)`'s **two** lists (indices 0 and 1, the 1200 and 1800 ranges). **[L]** |

`FUN_1402e5020`, the body of the *first* dynamic loop, has **no reads** (`tools/reads.py`,
depth 3) - so that loop is free. `FUN_1402dddb0` is not: `u32`, `u32`, an item, `u32`.

**Consequence for goals F and G.** Putting a potion in the Use tab through the character
record is **not** a small change: it needs `presence[3]`, which drags in two undecoded
blocks, and the record has no length prefix and no resync point. The cheaper route for
shops and drops is `0x0070` / `0x00AC`, which already move items on screen
(`crates/net/src/inventory.rs`, confirmed by the owner). **This document does not settle that
choice; it prices it.**

---

## 7. What I did NOT establish

* **What the five `charData + 0x5a8` containers are.** Position ranges measured; identity
  unknown. Section 3.
* **The meaning of three fields in the bundle body**: the `u8` at `+0x5d`, the trailing
  `u32` at `+0x7a`, and the fifteen unread bits of the attribute `u16`. Widths and offsets
  are [L]; meanings are not. Zeros are the constructor's own values, which is a reason, not
  a measurement.
* **Whether `ITEM_NEVER_EXPIRES` is required for a bundle.** It is the equip convention and
  no client-side expiry check was found for either type. **[I]**, unchanged.
* **The two blocks that `presence[3]`, `[4]` and `[5]` open.** Read *counts* measured; field
  meanings not decoded at all. Section 6.
* **The type-3 (pet) body.** Deliberately not decoded.
* **Anything on a screen.** No client run was spent. Every claim here is static, and the
  first client-visible test of any of it is a character whose Equip tab has an item in it.
* **Whether a bag item needs anything the equipped list does not.** `FUN_14030b6f0` performs
  no type check between the item and the inventory it stores into - an equip (type 1) in the
  Equip tab is stored by the same three instructions as anything else - but *whether the
  Equipment window renders it* is a client-side question this pass cannot answer.

---

## 8. Wire it like this

`crates/net/src/bag.rs`. The only change `crates/net/src/opcode.rs` needs is in
`equipped_block_with`, replacing its last three lines:

```rust
// was:
//   b.extend_from_slice(&0u16.to_le_bytes()); // end of the equipped list
//   b.extend_from_slice(&0u16.to_le_bytes()); // FUN_14030b6f0
//   b.extend_from_slice(&[0u8; 6]);           // FUN_14030b9e0, three lists
b.extend_from_slice(&bag::equipped_tail(equip_bag, slots));
```

where `equip_bag: &[bag::BagEquip]` and `slots: u16` is the Equip tab's slot count -
`chr.inventory_slots[0]`, the same number already sent in the `presence[7]` block.
`bag::equipped_tail(&[], slots)` is `[0u8; 10]`, byte-for-byte what is on the wire today,
and a test asserts it.

`Character` needs one new field: the Equip tab's bag contents, `Vec<(u16, u32)>` of
`(pos, itemId)` - or `Vec<BagEquip>` if per-item stats are wanted, which the equipped list
already supports through `EquipStats`. Nothing else in the record changes.
