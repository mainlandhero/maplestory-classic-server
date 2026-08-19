# `SetField` - inbound `0x01A0`, handler `FUN_142097f80`

> **CONFIRMED ON A LIVE CLIENT, 2026-08-19.** Not inferred any more. The client's own
> dispatcher entered `FUN_142097f80` *while dispatching opcode `0x01A0`*, and the probe
> caught it:
>
> ```
> WATCH: 0x142097f80 ENTERED on tid 141492 while dispatching opcode 0x01A0
>        called-from=0x141b26567
> WATCH: 0x142cfb500 ENTERED ... [rcx+0x33f4]=u8:0x00  called-from=0x142097ff8
> ```
>
> Three predictions landed in one run:
>
> * **`0x01A0` is `SetField`.** The block arithmetic, the channel-change string and the
>   8-byte clock base were all pointing at the right function.
> * **No stage transition is needed.** `called-from=0x141b26567` is inside `FUN_141b25f30`,
>   the **login** stage's `OnPacket` (which spans to `0x141b265da`) - the
>   `if (opcode - 0x1a0 < 4)` chain to the stage base class, exactly as read.
> * **The latch is clear at migrate time.** `[world+0x33f4] = 0`, so neither early return
>   fired. That had been an expectation, not a measurement; now it is measured.
>
> The client then **faulted** at `FUN_1402fa540+0x1c` (`0xC0000005`), one millisecond later.
> That call site is `142098685`, on the **`characterData == 0` branch**, between its first
> `u8` and its next `u32`. Which is what the short form should do here: it is the
> "same character, new map" packet and it assumes a character and a field that a freshly
> migrated client does not have yet. **The crash is the short branch being the wrong branch,
> not evidence against the opcode.**
>
> So the next packet must carry `characterData = 1` and a real character record.


The packet that answers a migration hello and puts a character into a map. This is the
layout as the client reads it, decoded 2026-08-19.

**Method.** The decompiled body and a full disassembly of all 11726 bytes were read
together. Both contain **exactly 50 calls to the packet-read primitives** - checked again
after two more primitives turned up (a `u64` reader and a thunk to the `u32` one); this
function uses neither, so 50 stands. Nothing
is hidden behind a Themida gap or an unfollowed tail jump in this function, and the
disassembly is what fixes the *order* - the decompiler's ordering stops being trustworthy
once it reaches the string machinery. Every offset below is read from the listing.

## Two early returns, both silent

Before any field past offset 8 is touched:

```asm
142097fe7  TEST R14,R14           ; R14 = DAT_143aa84a0, the world object
142097fea  JZ   14209acea         ; return
142097ff3  CALL 142cfb500         ; return *(u8*)(world + 0x33f4)
142097ffa  JNZ  14209acea         ; return
```

Neither logs, warns or shows anything. **A packet that trips either is indistinguishable
from one the client rejected**, so a run that produces silence cannot tell you which
happened. That is the main hazard in testing this.

* `world == NULL` - [[maplecw-inbound-dispatch]] establishes the world object is already
  live before the channel socket opens, so this should not fire.
* `world+0x33f4` is a latch, **measured as `0x00` at migrate time** so it does not fire. It is **set to 1 by `FUN_142cfb470`**, which also builds and
  sends outbound `0x1BE` - i.e. the client sets it when *it* asks to change field. It is
  **cleared by `FUN_142d3c670`**, which runs in the world-init chain (`FUN_142c42f30` ->
  `FUN_142d3c970`... -> the constructor `FUN_142ca5c50` that writes `DAT_143aa84a0`).
  On a fresh migration nothing has set it, and the probe read it as `0x00` on the wire.

## The fixed head: 33 bytes, unconditional

Every one of these is read on every path.

| off | type | read at | what the client does with it |
|---:|---|---|---|
| 0 | `u8[8]` | `142097fcb` | -> `FUN_1408f67d0`: stores it at `_DAT_143ac3120` **and** stamps `DAT_143ac3128` from `(*DAT_143262db0)()`, a tick source. A **server clock base**, paired with the local tick at the moment of receipt. Read *before* both early returns. |
| 8 | `u32` | `142098065` | **channel id** -> `world+0x2260` |
| 12 | `u8` | `142098077` | -> `world+0x226c` |
| 13 | `u32` | `14209808a` | -> `world+0x2884` |
| - | - | `14209809c` | re-reads `world+0x2260` and compares with the value from before offset 8. **On a difference it displays "Channel"** - the string is built with a CR, a TAB, a CR and an LF spliced through it, which is why searching the image for the word never found this handler. |
| 17 | `u8` | `142098132` | `if (== 1)` -> `FUN_142d16ef0(world)`, which walks the tree at `world+0x3bd0` clearing field 5 of every node |
| 18 | `u32` | `14209814a` | **read and discarded** - the return value is never stored |
| 22 | `u32` | `142098152` | -> `[RBP-0x70]` |
| 26 | `u32` | `14209815d` | -> `[RSP+0x70]` |
| 30 | `u8` | `142098169` | **`characterData`** - the fork, below |
| 31 | `u16` | `142098177` | **string count** - `TEST EBX,EBX / JZ 142098386` skips the entire string block when zero |

## Then two branches

**The string block** (offset 33 on), entered only when the `u16` at 31 is non-zero: one
string, then a loop of strings. **The loop bound is not pinned** - the same register is
tested twice and the decompiled loop is tangled up with vector growth. Send `0` and the
whole thing is skipped by a single jump, which is the safe course and is certain from the
listing.

**`characterData`** at `1420983a3`, `CMP dword [RBP-0x60], 0`:

*Non-zero* - three `u32`s, then

```
14209842d  CALL 0x140304b20      the full character-record decoder, 18525 bytes
142098435  u8   -> if zero, jump past the rest
142098445  u8   -> if non-zero, a u32
142098471  8 raw bytes
142098479  u8
14209849f  8 raw bytes
1420984c2  8 raw bytes
1420984ca  u32
```

*Zero* - a short path beginning `u8` at `14209854b`.

`FUN_140304b20(user, scratch, packet, 0)` - the second argument is a 112-byte **stack
buffer**, not a presence mask from the wire, so there is no obvious way to ask for a
smaller record from outside it.

## This resolves an open question the other way

`research/msexe-setfield.md` argued: "if SetField in this client carried a full character
record, it would show up as a third caller of `FUN_140304b20`, and it does not." **It does.**
That search covered `FUN_142cbaa80`, the world dispatcher, and `SetField` is not in it -
it is a *stage* handler, in a different function reached by a different mechanism. The
inference was sound and the search was aimed one dispatcher short.

So this client's `SetField` **does** carry a full character record, which is what the
reference server does too: its whole answer to a migrate-in is one `SetField` with
`characterData = true`.

## What is left

1. **The character record** (`FUN_140304b20`). The big one. It **shares**
   `FUN_140302e30`, the character-stat decoder, with the character-list path
   `FUN_1403094b0` - literally the same function, so 108 bytes of it are already built and
   proven. (This entry used to call `FUN_140304b20` its *only* caller. It is not.)
2. **The string block's loop bound.** Avoidable by sending `0`, so this is not on the
   critical path.
3. ~~The short `characterData == 0` path~~ - **answered by the run, and not the way to go.**
   It faults at `FUN_1402fa540+0x1c`, reached from `142098685`, before its second field
   read. It is the "same character, new map" form and assumes state a freshly migrated
   client does not have.
4. ~~Whether `world+0x33f4` is really 0 at migrate time~~ - **measured: it is `0x00`.**
   Neither early return fires.
