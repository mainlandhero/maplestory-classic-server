# `0x00D9` - where the player is standing

Decoded 2026-08-20 because the item-drop work needs a position and **nothing on the wire had
one**. `0x0107`, the packet that asks to drop an item, carries a slot and a count and no
coordinates at all, and the client's own pick-up sweep is a box of `x-0x19..x+0x19` by
`y-0x32..y+0x0a` around the player - so a drop placed more than ~25 px from the player's feet
is drawn and can never be collected. On screen that is identical to nothing happening.

Read out of `client-patched/MapleStory.exe` with `tools/encodes.py`, `tools/reads.py`,
`tools/listing.py` and `tools/callers.py`, and checked against **1082 captured bodies**.
**[L]** is off the listing, **[D]** is derived from two or more [L] or measured on the
captures, **[I]** is inferred.

Parser: `crates/net/src/usermove.rs`. **It is not wired to anything.**

---

## 0. The answer, before the working

```text
off  0   u8    FUN_141829fb0(field)        0 in every captured body
off  1   u32   FUN_14185cdc0(field)        constant per FIELD, changes on a map change
off  5   u32   the tick                    milliseconds, same clock as 0x00DF's
off  9   u8    a local flag                0 in every captured body
off 10   ---- the movement path, the SAME block the mob's 0x02FF carries ----
    +0   u32                               0 in every captured body
    +4   i16   x   <-- where this walk STARTED
    +6   i16   y
    +8   u16, u16
    +12  i16   element count, signed
    +14  elements: a command byte and a payload whose length depends on it
off ..   u8    key-state count
         ceil(count / 2) bytes, two 4-bit entries packed per byte
```

**The head's x/y is not where the player is.** It is where the reported walk began. The
player's current position is the position of the **last element that carried one**.

---

## 1. There is one builder, and its head has no branches in it

`research/msexe-send-opcodes.txt` has exactly one row for `0x00D9`: `FUN_1409f6eb0`, with the
`COutPacket(0xd9)` at `0x1409f965d`. That file is a full enumeration of all 1894 call sites of
`FUN_1406ed520`, so "one builder" is a census, not a search. **[L]**

Between the constructor and the `SendPacket` at `0x1409f9769` the listing is **straight-line -
not one branch** - which is what makes the ten-byte head fixed rather than conditional: **[L]**

```asm
1409f9654  MOV   EDX,0xd9
1409f965d  CALL  1406ed520          ; COutPacket(0xD9)
1409f9663  CALL  141892840          ; the current field
1409f966b  CALL  141829fb0          ; -> AL
1409f9677  CALL  1406ed840          ; w_u8
1409f967c  CALL  141892840          ; the current field again
1409f9684  CALL  14185cdc0          ; -> EAX
1409f968f  CALL  1406ed9d0          ; w_u32
1409f9694  CALL  1429e3ef0          ; -> EAX, no arguments
1409f969f  CALL  1406ed9d0          ; w_u32
1409f96a4  MOVZX ESI,byte [RSP+0x6e]
1409f96b1  CALL  1406ed840          ; w_u8
1409f96d2  CALL  141d57c60          ; the movement path
1409f9769  CALL  1406ed610          ; SendPacket
```

`FUN_141892840` is the same "current field" getter the drop arm of `FUN_142cc5b00` calls at
`0x142cc5c3b` (`research/item-drop.md` §1), so the first two fields are **field properties**,
not character ones.

**Field 1 changes when the map changes.** In
`research/fixtures/melee-collector-runs-once-per-swing-world.log` it is `0x4dca5bef` for the
first packet and `0x5a9f8081` from the packet where the position also jumps, then constant for
the rest of the session. Field key by behaviour; the name is **[I]**. **[D]**

**Field 5 is a millisecond tick**, two ways. Consecutive captured packets 26.525 s apart in
the log differ by `0x6798` = 26 520, and the 510 ms movement cadence appears as `0x1FE`
every time. `research/mob-combat.md` §1.5 independently matched an `0x00D9` tick against an
`0x00DF` attack tick 852 ms later to 18 ms. **[D]**

---

## 2. The path block is shared with the mob packet, and that is the only thing they share

`FUN_141d57c60` is called by **both** builders - `0x1409f96d2` for the player and
`0x141cb8353` for the mob's `0x02FF`, each immediately before its own `SendPacket`. It calls
`FUN_1404b2000`, whose reader is `FUN_1404b2630`; `crates/net/src/mobmove.rs` already decodes
that reader's head as `u32, i16 x, i16 y, u16, u16, i16 count` from the reads at `1404b2650`,
`1404b265b`, `1404b2675`, `1404b2690`, `1404b269c`, `1404b26a9`. **[L]**

**So the two packets diverge only in the head.** The mob's is long and variable - object id,
move id, a packed byte, a move action, a `u64`, two counted arrays, a gated eleven-`u32`
block, twenty-odd more bytes. The player's is the ten bytes above. Everything from the path
head onward is byte-identical machinery.

One difference that is easy to miss: the mob's reader is wrapped by `FUN_141d598b0`, which
reads a trailing byte **gated on its third argument** - set for `0x02FF`, cleared for
`MOB_MOVE`. The player's packet has its own trailer instead, §4.

---

## 3. The elements, from the jump table rather than from the captures

`FUN_1404b2630` reads the command byte at `0x1404b26ee`, deobfuscates it, and dispatches:

```asm
1404b2731  CMP   EAX,0x4e
1404b2734  JA    1404b2d56              ; past the table
1404b273a  LEA   RDX,[RIP - 0x4b2741]   ; = 0x140000000, the image base
1404b2741  MOVZX EAX,byte [RDX+RAX+0x4b2f24]   ; index table at 0x1404b2f24, 79 entries
1404b2749  MOV   ECX,dword [RDX+RAX*4+0x4b2ef0] ; offsets at 0x1404b2ef0, 13 targets
1404b2753  JMP   RCX
```

Both tables were read straight out of the image. **79 commands, 13 case bodies**, each a
straight run of reads ending in a `jmp` to the loop tail at `0x1404b2d7f`. **[L]**

| case | commands | n | payload | tail? | element |
|---|---|---:|---|---|---:|
| `1404b2755` | `00 08 13 37..3c 44 4e` | 11 | 8 x u16 | yes | **21** |
| `1404b2755` | `0f 11` | 2 | 9 x u16 - the ninth is gated at `1404b27de` on the command being one of exactly these two | yes | **23** |
| `1404b2846` | `2c 36` | 2 | 5 x u16 | yes | 15 |
| `1404b28c6` | `01 02 12 15 30..35 46` | 11 | 2 x u16 | yes | 9 |
| `1404b292e` | 29 commands | 29 | none | yes | 5 |
| `1404b297b` | `18 21` | 2 | u32 | yes | 9 |
| `1404b29f8` | `17` | 1 | 2 x u32 | yes | 13 |
| `1404b2a85` | `03..07 09..0b 0d 29 2a` | 11 | 3 x u16, u32 | yes | 15 |
| `1404b2b23` | `16` | 1 | 3 x u16 | yes | 11 |
| `1404b2b6c` | `0e 10` | 2 | 3 x u16 | yes | 11 |
| `1404b2beb` | `14 48 49` | 3 | 2 x u16, then `1404b28f5`'s 2 x u16 | yes | 13 |
| `1404b2c20` | `27` | 1 | 5 x u16 | yes | 15 |
| `1404b2ca0` | `0c` | 1 | u8 | **no** | 2 |
| `1404b2e58` | `3d 3f` | 2 | 7 x u16 | **no** | 15 |
| - | anything `> 0x4e` | - | none | yes | 5 |

The **common tail** at `0x1404b2d7f` is `u8` (`1404b2d82`), `u16` (`1404b2db7`), `u8`
(`1404b2dce`) - four bytes. Two cases skip it by jumping straight to the loop back-edge at
`0x1404b2ec0`.

**21 bytes is the ordinary element**, which is exactly `net::mobmove::MOB_PATH_ELEMENT_LEN` -
independently measured there from a 21-byte ladder across 30 captured `0x02FF` bodies. The two
instruments agree without being told about each other. **[D]**

### 3.1 Which elements carry a position

The common tail deobfuscates element slots `+0x00`, `+0x40`, `+0x30` and `+0x10` into
`rbp`/`r15`/`r12`/`[rsp+0x20]` at `0x1404b2e0b`-`0x1404b2e3b`, and the cases that read no
coordinates **re-store those saved values into the next element** (`1404b28c9`, `1404b2931`,
`1404b297e`, `1404b2b6f`, `1404b2cda`). So an element either supplies `x, y` as its **first
two `u16`** or inherits the previous one's. **[L]**

Thirty-one of the 79 commands supply them. `net::usermove::element_carries_position` is that
set.

---

## 4. The trailer - and how it was found, because the instrument lied

With the head at 10, the path head at 14 and the element table above, every captured body came
out **exactly 10 bytes short**. `tools/encodes.py` reported no other writes in either the
builder or the path wrapper.

`encodes.py` was the wrong instrument to trust there. It matches calls against a table of
**eight** encode primitives, one address per type - and the read side of this same client is
known to have **ten** primitives with aliases (`CLAUDE.md`, and `tools/reads.py` carries the
history). A write it does not have an address for is invisible, and it does not say so.

Disassembling `FUN_141d57c60` after its call to `FUN_1404b2000` shows what it missed - not an
unknown primitive but a **loop**: **[L]**

```asm
141d580c9  CALL  1404b2000              ; the path
141d580ce  MOV   RAX,[RDI+0x48]         ; a byte array
141d580dc  MOV   EAX,[RAX-8]            ;   its length
141d580e5  CALL  1406ed840              ; w_u8   <- the count
141d580f0  ...                          ; loop: take two entries
141d5810e  MOVZX EBX,byte [R14+RCX] / AND BL,0xf
141d58171  MOVZX EAX,byte [R14+RCX+1] / SHL AL,4 / OR BL,AL
141d58182  CALL  1406ed840              ; w_u8   <- one packed byte per PAIR
141d58187  ADD   ESI,2 / ADD R14,2 / JMP 141d580f0
```

So the trailer is `u8 count` then `ceil(count/2)` bytes, two 4-bit entries per byte - a
key-state list. Every captured body carries `count = 0x11` (17) and therefore 9 packed bytes,
except a handful with 1, 3, 4 and 13. **[L]** for the shape, **[D]** for the values.

`encodes.py` is not wrong about what it found; it is wrong about what it did not. A field walk
built only from its output is short, clean and confident - which is the exact failure
`CLAUDE.md` describes for the read side, on the other side of the wire.

---

## 5. The check: 1082 bodies, to the byte

`research/fixtures/*world*.log` and `previous-runs/world*.log` together hold **1082**
`0x00D9` bodies from many sessions. With the tables above:

| | |
|---|---|
| bodies that walk and land **exactly** on the end of the body | **1082 of 1082** |
| consecutive pairs where one report ends exactly where the next begins | **1000 of 1064** |
| pairs that do not | **64**, every one of them at a map change or a teleport |

The second row is the strong one, because it needs no field names to be right: it is a
consistency property of the decode against itself over a thousand independent packets. The
element table was derived from the jump table **before** it was run against any capture.

---

## 6. Why the head is not good enough, measured

`melee-collector-runs-once-per-swing-world.log` carries `0x00D9` and `0x00DF` from one session,
and `net::combat::AttackRequest` reads the player's position out of the attack at body offsets
34 and 36 - a different packet decoded by a different agent. **[L]**

| attack says | the last `0x00D9`'s **start** | its **end** | gap |
|---|---|---|---|
| `05:31:40.570 (503, 395)` | 484 | **498** | start 19 px out, end 5 px |
| `05:31:41.669 (522, 395)` | 508 | **509** | 14 px / 13 px |
| `05:31:42.679 (571, 395)` | 537 | **554** | 34 px / 17 px |
| `05:31:43.800 (636, 395)` | 586 | **605** | 50 px / 31 px |
| `05:31:44.570 (653, 395)` | 605 | **651** | 48 px / 2 px |
| `05:31:45.440 (671, 395)` | 658 | **658** | 13 px / 13 px |

`y` is 395 in every row, in both packets, on both readings.

**The head is up to 50 px behind while running - twice the pick-up box.** The end is 2-31 px
behind at gaps of 71-502 ms, and **exact when the player is standing still**, because a
stationary report starts and ends in the same place: `05:31:39.477` and `05:31:46.108` both
have `start == end`.

A drag out of the inventory window happens while standing still, so for the drop the end
position is the right answer and the error is zero. Nothing here needs the head.

---

## 7. Does it have to be answered?

**No, as far as 1082 packets can say.** Every one of them went unanswered - `world.log` marks
each `0x00D9 UNKNOWN is not answered yet` - and the client kept playing for minutes
afterwards in every session. It does not latch the way `0x0107` does. **[D]**

That is a measured negative over the captures, not a proof about the client. It is the same
kind of evidence as the 47 distinct inbound opcodes this server has received and mostly
ignored without a freeze.

---

## 8. What is NOT established

1. **`field_flag` (body 0) and `flag` (body 9).** Both zero in every captured body. The
   second is the same local the client tests at `0x1409f96f9` to decide whether to run a
   field-level update after sending, so it is a mode of some kind.
2. **`field_key` (body 1).** Per-field and stable; the name is **[I]**.
3. **The path head's `u32` at +0 and the two `u16` at +8/+10.** Read at `1404b2650`,
   `1404b2690`, `1404b269c` and stored; zero in every captured body.
4. **What each of the 79 commands means.** Only their lengths and whether they carry a
   position were needed, and only those were taken. Naming them would be **[I]** from the
   reference that scores 1 of 8.
5. **The key-state entries.** Four bits each, 17 of them in nearly every body. Not decoded,
   not needed.
6. **Whether the server is expected to broadcast this to other players.** There is exactly one
   player here, so it has never mattered.
