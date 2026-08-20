# Dropping an item, seeing it on the ground, and picking it back up

Started 2026-08-20 after the owner dragged a sword out of the inventory window and the server
refused it. Everything here is read out of `client-patched/MapleStory.exe` with
`tools/listing.py`, `tools/reads.py`, `tools/callers.py`, `tools/dataref.py` and one Ghidra
headless run. Rows are tagged **[L]** when they were read off the listing and **[I]** when
they are inferred - almost always from the v214 reference, which `CLAUDE.md` scores at **1 of
8** against a held-out control.

New files beside this one: `research/msexe-droppool-onpacket.txt`,
`research/msexe-dropenter.txt`, `research/msexe-dropleave.txt`,
`research/msexe-droppickup.txt`, `research/msexe-droppool.c`.

---

## UNRESOLVED - read this before building on anything below

### 1. The player's pick-up request opcode is NOT found

The path is followed the whole way and ends at a wall that is documented in
`docs/ghidra.md`:

```text
FUN_1428af6d0            the CUserLocal key handler; 0x1428b0cec loads the pool [0x143ACE240]
  -> FUN_14179c9d0(pool, &userPos)        the sweep; box x-0x19..x+0x19 by y-0x32..y+0x0a
       -> FUN_142cc6770(ctx, pos, dropObjectId, ...)     the CWvsContext pre-checks
            -> FUN_1417a2b00(pool, dropObjectId)   queues it, sets pool+0x90 = 1
            -> jmp 0x144f14cca                     <-- into the Themida VM section
```

**Neither sweep builds a packet**, and that is a *verified* negative, not a failed search: a
disassembling sweep for every `call 0x1406ed520` (`COutPacket::COutPacket(op)`) in
`0x141780000..0x1417f0000` returns **four** sites, opcodes `0x25f`, `0x19d`, `0x12d`, `0x116`.
The positive control is in the same output - it finds `0x25f`'s builder `FUN_1417d29d0`,
which is real and is called six times from inside `DropEnterField`. So the instrument speaks;
the pick-up simply is not there. **[L]**

What the static work *does* pin down is where the opcode lives. The outbound pool blocks run

| block | mscw outbound | how it is known |
|---|---|---|
| mob | `0x02FF .. ~0x0323` | `0x02FF` MobMove is decoded in `research/mob-behaviour.md` **[L]**; every builder in the run is in `0x141c6..0x141d0` |
| NPC | `0x0327 .. 0x0328` | builders `FUN_141e4b620`, `FUN_141e3ff60`, the `0x141e3..0x141e7` NPC range **[L]** |
| **unclaimed gap** | **`0x0329 .. 0x032E`** | no `mov edx,imm / call 0x1406ed520` anywhere in the image uses them **[L]** |
| next claimed | `0x032F` | builders in `0x141f3..0x141f4` **[L]** |

The drop pool sits between the NPC pool and the reactor pool in every version of this
enumeration, so **the request is one of the six opcodes `0x0329..0x032E`**. Lining the v214
markers up (`END_NPC`, `END_LIFEPOOL`, `BEGIN_DROPPOOL`, `DROP_PICK_UP_REQUEST`) puts it at
**`0x032C`** - **[I]**, and from the instrument that scored 1 of 8.

> **Finish this with a measurement, not more static analysis.** The world server logs every
> inbound opcode. One walk over one drop names it in one line of `world.log`. That is
> cheaper and stronger than anything left to read.

### 2. Which of `DropEnterField`'s two tails a real item takes is NOT established

`FUN_1417a2ee0` branches at `0x1417a800b` on a local that `0x1417a411b` sets from
`itemId == 0`, and the two arms **read a different number of bytes**. [L] The consequence is
handled rather than resolved - see [The two tails](#the-two-tails-and-why-the-long-one-is-the-one-to-send) -
and `net::drops::drop_enter_field` always sends the longer body, which is safe under either
arm. What is *not* known is which arm the client actually takes for a sword.

### 3. `0x0301` is a MOB picking something up, not the player

It nearly went in as "the pick-up request". Its first field is `[obj+0x3a0]`, and
`research/mob-behaviour.md` section 11 already reads `[mob+0x3a0]` as field 0 of the outbound
`0x02FF` MobMove - the **mob's object id**. The caller chain confirms it: the sweep that
sends it is called from the mob update `FUN_141c626c0`, not from anything on the player.
Reading it as the player's request would have had the server look up a character id that is
really a mob id. **[L]**

---

## 1. A drop is `0x0107` with `dst = 0`, and it is read off the client, not off the capture

The owner's brief asked for this to be confirmed against the client rather than against one
capture whose `count` happened to be 1. It confirms.

`FUN_142cc5b00(this, invType, src, dst, count)` is the only builder of `0x0107` in the whole
image (`research/msexe-send-opcodes.txt`, one row), and all **five** of its callers
(`tools/callers.py 0x142cc5b00`) reach the single `FUN_1406ed520(buf, 0x107)` at
`0x142cc5eac`. Inside it, `dst == 0` is a **first-class case with its own refusal**: **[L]**

```asm
142cc5bb1  TEST EDI,EDI          ; dst
142cc5bb3  JE   142cc5c3b        ; dst == 0 -> the drop arm, jumping OVER
142cc5bb9  MOV  EDX,R12D         ;   the ordinary move's slot checks
142cc5bbf  CALL 140255650        ;   FUN_140255650(src, invType)
...
142cc5c3b  CALL 141892840        ; the current field
142cc5c43  CALL 14182e950        ; a FIELD-level predicate
142cc5c48  TEST EAX,EAX
142cc5c4a  JE   142cc5c6f        ; allowed here -> carry on to the send
142cc5c4c  MOV  EDX,0xc2f        ; otherwise show string 0xc2f
142cc5c64  CALL 1415eca30
142cc5c6a  JMP  142cc5cf5        ; -> return 0.  NOTHING is sent.
```

Three things follow, and all three are load-bearing:

1. **A drop is not a move with a funny slot.** The `dst == 0` arm jumps over the slot
   validation entirely (`LAB_142cc5c6f`), which is why the server's `slot 0 is outside
   1..=30` refusal is the wrong answer rather than a lenient one. **[L]**
2. **The refusal it does have is a *field* property**, `FUN_14182e950(field)` - "you may not
   drop things on this map". A field-level check is only meaningful for a drop. **[L]**
3. **There is no separate drop opcode.** `0x0107` has one builder, that builder builds one
   packet, and the drag-to-field case is inside it. **[L]**

### The captured body, and what `count` means

```text
<- 0x0107, 11 byte body   75981808 01 0100 0000 0100
                          tick     ^  ^    ^    ^ count 1
                          invType 1 |    dst 0
                                    src 1
```

The five writes, in order: `0x142cc5ebd` u32 tick, `0x142cc5eca` u8 invType, `0x142cc5ed7`
u16 src, `0x142cc5ee3` u16 dst, `0x142cc5ef3` u16 count. **[L]** On an *unequip* `count` is
`-1`; on a drop it is a real quantity, so `net::drops::drop_count` treats anything `<= 0` as
one item rather than trusting it.

### Two gates on `FUN_142cc5b00` worth knowing about

| | |
|---|---|
| `[ctx+0x2330]` and `[ctx+0x2338]` must both be 0 | the one-request-outstanding latch. `0x142cc5f01` is literally `MOV dword ptr [RSI+0x2330],1`, and it runs **after sending a drop too**, so an unanswered drop kills every later inventory action exactly the way an unanswered unequip did on 2026-08-19. **[L]** |
| `now - [ctx+0x2334] >= 0x1f4` | `0x142cc5b93` / `0x142cc5b99` - a **500 ms** cooldown between inventory requests, and `0x142cc5f10` restamps it on every send. Dragging two items out in under half a second silently drops the second. **[L]** |

---

## 2. Where the drop pool lives

`CField::OnPacket` is `FUN_141820080`. Its range chain at `0x141821e24` routes by opcode to
one pool object each. The rows around the drop pool, **read off the listing**: **[L]**

| opcodes | handler | singleton |
|---|---|---|
| `0x3C6..0x44E` | `FUN_141D30E80` | mob pool |
| `0x44F..0x468` | `FUN_141E75800` | NPC pool |
| `0x469..0x46D` | `FUN_141F0C450` | *(static call: `ecx = opcode`, `rdx = packet`)* |
| **`0x46E..0x46F`** | **`FUN_1417A1C30`** | **`[0x143ACE240]` - the drop pool** |
| `0x470..0x472` | `FUN_141C33F20` | |
| `0x473..0x475` | `FUN_140D42ED0` | |
| `0x476..0x477` | `FUN_142141E50` | |

> **A correction to `research/npc-spawn.md` section 3.** That table lists `0x469..0x46D` as
> `FUN_14290C44E`. The listing at `0x141821eba` says `call 0x141f0c450`, and the call is a
> *static* one - `mov rdx,rbx / mov ecx,r9d` - not a singleton method like its neighbours.
> Nothing in this document depends on it; it is recorded because the row is wrong.

`FUN_1417A1C30` has **no `.pdata` entry** - it is a 32-byte leaf that `tools/listing.py` and
`tools/reads.py` both refuse, which is worth knowing before concluding a function is missing:

```asm
1417a1c30  SUB   EDX,0x46e
1417a1c36  JE    1417a1c45
1417a1c38  CMP   EDX,1
1417a1c3b  JNE   1417a1c4d       ; neither -> return
1417a1c3d  MOV   RDX,R8
1417a1c40  JMP   1417ad7a0       ; 0x46F  DropLeaveField
1417a1c45  MOV   RDX,R8
1417a1c48  JMP   1417a2ee0       ; 0x46E  DropEnterField
```

**The pool is exactly two opcodes wide**, which is what identifies it: its neighbours are
five and three wide, and every version of this enumeration has exactly one two-wide pool
between the NPC pool and the message-box pool. **[L]** for the width, **[I]** for the name.

---

## 3. `0x046E` DropEnterField

`FUN_1417a2ee0`, `.pdata 0x1417a2ee0 .. 0x1417ad796` - **one** entry, 43 190 bytes.
`tools/reads.py 0x1417a2ee0 3` reports **47 read sites**. Ghidra's decompiler **times out**
on it, so every row below is from the listing.

> **The 47 sites are not in wire order.** The tail is duplicated into two arms and the
> address order interleaves them. Reading `reads.py`'s output top to bottom as a field list
> produces a body that is wrong in the middle, which is exactly the mistake that ships a
> short packet. The order below was recovered by walking the branches.

### The head, in wire order

| # | type | read at | stored | name | tag |
|---:|---|---|---|---|---|
| 1 | u8 | `1417a2f2b` | *(class selection)* | **dropType** - `1` allocates the 0x238-byte object and overwrites its vtable with `0x1433D3F68`; `0` and anything else leave the constructor's | [L] shape, [I] name |
| 2 | u8 | `1417a2fde` | `drop+0x60`, and `drop+0x61 = ((v-1) <= 1)` | **enterType**. See §3.1 | [L] |
| 3 | u32 | `1417a2ff1` | `drop+0x64`, **the pool hash key** | **objectId** | [L] |
| 4 | u8 | `1417a3086` | obfuscated at `drop+0x78` / `drop+0x80` | **isMoney** | [L] store, [I] name |
| 5 | u32 | `1417a3284` | `drop+0x194` | **motionType**. `FUN_1417a2b00` refuses to queue a pick-up unless this is **0** | [L] |
| 6 | u32 | `1417a32ad` | `drop+0x198` | *(the reference randomises it)* | [L] store |
| 7 | u32 | `1417a32d6` | `float(v)/k` -> `drop+0x1d8` | *(unknown - a speed or a scale)* | [L] |
| 8 | u32 | `1417a3310` | obfuscated at `drop+0x90` / `drop+0x98` | **itemId**, or the meso amount | [L] store, [I] name |
| 9 | u32 | `1417a3513` | `drop+0x68` | **ownerId** | [L] store, [I] name |
| 10 | u8 | `1417a3539` | `drop+0x70` (as a dword) | **ownType**. Stored and never tested | [L] |
| 11 | i16 | `1417a3560` | -> obfuscated point `drop+0x128`, then `drop+0xf8` and `drop+0x20` | **x** | [L] |
| 12 | i16 | `1417a356f` | as above | **y** | [L] |
| 13 | i16 | `1417a3597` | `drop+0x218` | *(unknown)* | [L] |
| 14 | i16 | `1417a35a8` | `drop+0x21c` | *(unknown)* | [L] |
| 15 | i16 | `1417a35b9` | `drop+0x220` | *(unknown)* | [L] |
| 16 | i16 | `1417a35ca` | `drop+0x224` | *(unknown)* | [L] |
| 17 | u32 | `1417a35db` | `drop+0x6c` | **sourceObjectId** | [L] store, [I] name |
| 18 | u32 | `1417a3601` | `drop+0x1e0` | *(unknown)* | [L] |
| 19 | u64 | `1417a362a` | `drop+0x1e8` | *(unknown)* | [L] |
| 20 | u32 | `1417a3655` | `drop+0x1f0` | *(unknown)* | [L] |
| 21 | u8 | `1417a367e` | `drop+0x210` as a bool | *(unknown)* | [L] |
| 22 | u64 | `1403747d3` | `drop+0x1f8` | via `FUN_1403747c0(drop+0x1f8, packet)`, **unconditional** | [L] |
| 23 | u32 | `1403747de` | `drop+0x200` | same helper | [L] |
| 24 | u64 | `1403747e9` | `drop+0x208` | same helper | [L] |
| 25 | u8 | `1417a36d2` | `drop+0x16d` | **moneyType** | [L] store, [I] name |
| 26 | u8 | `1417a36fc` | a local bool | *(unknown)* | [L] |
| 27 | u8 | `1417a370a` | a local bool | *(unknown)* | [L] |

**Reads 1..27 match the v214 `DropPool.dropEnterField` encoder field for field and type for
type, twenty-seven in a row.** That is far past coincidence and is the reason the *names*
above are worth anything at all - but it is still the reference, so they stay **[I]** and the
offsets stay the fact.

### 3.1 `enterType` decides two things, and both bite

**It decides whether the source block is on the wire.** `0x1417a41b1`: **[L]**

```asm
1417a41b1  MOV   EAX,dword ptr [RBP + 0x188]   ; enterType
1417a41b7  TEST  EAX,0xfffffffc                ; >= 4 ?
1417a41bc  JNE   1417a4651                     ;   -> skip the block
1417a41c2  CMP   EAX,2
1417a41c5  JE    1417a4651                     ; == 2 -> skip the block
1417a41cb  ...   READ u16                      ; srcX
```

so `i16 srcX, i16 srcY, u32 delay` are read for **enterType 0, 1 or 3 only**. The v214
reference writes the same block for `enterType != 2 && enterType < 5`; **mscw's upper bound
is 4, not 5.** Invisible for the values we send, recorded so nobody "corrects" it back.

**It decides whether the drop can ever be picked up.** `0x1417a3041`: **[L]**

```asm
1417a3041  LEA   EAX,[R15 + -1]      ; enterType - 1
1417a3045  CMP   EAX,1
1417a3048  SETBE BL
1417a3064  MOV   byte ptr [RAX + 0x61],BL     ; drop+0x61 = (enterType == 1 || == 2)
```

and **every pick-up sweep in the client gates on `drop+0x61 != 0`** - the mob one at
`0x14179e8eb` and, by the same field, the player one. So a drop sent with enterType `0` or
`3` is drawn and is uncollectable. `net::drops` offers only `1` (Floating, animates) and `2`
(Instant, skips the source block).

`0..3` are `Default, Floating, Instant, FadeAway` **[I]**, from the reference; the numeric
behaviour above is **[L]** and does not depend on the names.

### 3.2 The conditional middle

| when | fields | read at |
|---|---|---|
| `enterType` is 0, 1 or 3 | `i16 srcX`, `i16 srcY`, `u32 delay` | `1417a41d2`, `1417a43af`, `1417a4646` |
| always | `u8 isExplosiveDrop` -> `drop+0x168` | `1417a4658` |
| always | `u8 isSpecialDrop` -> `drop+0x228` | `1417a4688` |

### 3.3 The two tails, and why the long one is the one to send
<a id="the-two-tails-and-why-the-long-one-is-the-one-to-send"></a>

At `0x1417a800b` the handler branches on a local that `0x1417a411b` computes as
**`itemId == 0`** - `itemId` being read 8, deobfuscated out of `drop+0x98`: **[L]**

```asm
1417a411b  CMP   dword ptr [RBP + 0x38],0x0    ; the decoded itemId
1417a411f  MOV   dword ptr [RBP + 0x1b4],1
1417a4129  JE    1417a4132
1417a412b  MOV   dword ptr [RBP + 0x1b4],R12D  ; 0
...
1417a800b  CMP   dword ptr [RBP + 0x1b4],0x0
1417a8012  JE    1417a8927                     ; -> the LONG arm
                                               ; fall through -> the SHORT arm
```

| | short arm, from `1417a833d` | long arm, from `1417a9bab` |
|---|---|---|
| `u64` expire FILETIME | **not read**; `drop+0x158` zeroed at `1417a832a` | **read** as 8 raw bytes, `if (isMoney == 0)`, `1417a9b7b` |
| `u8 canBePickedUpByPet` -> `drop+0x160` | `1417a833d` | `1417a9bde` |
| `u8` *(non-zero fires `FUN_140dbb710(…,0xc0041f15)`)* | `1417a8367` | `1417a9c08` |
| `i16 fallingVY` -> `drop+0x164` | `1417a83a6` | `1417a9c47` |
| `u8 fadeInEffect` | `1417a83f8` | `1417a9c9a` |
| `u32 collisionPickUp` | `1417a8424` | `1417aa063` |
| `u8 itemGrade` | `1417a8630` | `1417ac3ba` |
| `u8 prepareCollisionPickUp` -> `drop+0x190` | `1417a863c` | `1417ac3c5` |
| `u32` -> `drop+0x19c` | **not read** | `1417ac3ef` |
| `u32` -> `drop+0x1a0` | **not read** | `1417ac418` |

Sixteen bytes apart. **Which arm a sword takes is not established** - it turns on what the
client means by `itemId == 0`, and both readings are defensible. It does not have to be
resolved, because the two failure modes are not symmetric:

* Send the **short** body and let the client take the long arm: it reads **16 bytes past the
  end of the frame**. That is the failure that killed this client on a chat packet and again
  on a mob body.
* Send the **long** body and let the client take the short arm: it stops early and ignores
  the surplus. Frames carry their own length, so surplus bytes are inert - the same argument
  `crates/net/src/inventory.rs` makes about the `avatarChanged` byte.

So `net::drops::drop_enter_field` **always writes the long tail.** The one remaining hazard
is the short arm reading the *expiry* bytes as its seven tail fields, and that is why the
expiry is written as **zero**: under the short arm every tail field then reads `0`, which is
precisely what the client stores there itself on both of its own short paths (`1417a832a`
and `1417a9bcb` both write `drop+0x158 = 0`). A `FileTime.MAX_TIME()` expiry would put
`0xFF`s into the byte at `1417a8367`, whose only known effect is to fire an undecoded error
path.

### 3.4 A duplicate object id makes the client read nothing

`0x1417a3014` looks the object id up in the pool **before read 4**, and on a hit jumps to
`0x1417a46cd` - the rest of the body is never read. **[L]** Identical to the NPC pool's
`0x044F`, and it means an id reused inside one field silently produces no drop.

### 3.5 Body length

| | bytes |
|---|---|
| head, reads 1..27 | 84 |
| the source block (enterType 0/1/3) | +8 |
| `isExplosiveDrop`, `isSpecialDrop` | +2 |
| the long tail for an item (expiry included) | +27 |
| **item, enterType 1** | **121** |
| item, enterType 2 | 113 |
| money, enterType 1 | 113 |

Pinned by `net::drops::DROP_ENTER_FIELD_LEN` and a test that walks every offset.

---

## 4. `0x046F` DropLeaveField

`FUN_1417ad7a0`, `.pdata 0x1417ad7a0 .. 0x1417b21f4`, **11 read sites**.

```text
u32 objectId      1417ad7cd    the pool hash key - fed straight into the DIV at 1417ad7fe
u8  leaveType     1417ad7d7    MOVSX; dispatched three ways, see below
```

> **The order is the opposite of the v214 reference**, which writes the type first. **[L]**,
> and it is the kind of error that looks right: a small object id and a small type byte swap
> cleanly and simply address the wrong drop. The listing settles it - the first `u32` is what
> the bucket walk at `0x1417ad810` compares against, and the `u8` is what `CMP R14D,0x4` and
> the nine-entry jump table at `0x1417b21d0` dispatch on.

| leaveType | extra fields | where | tag |
|---:|---|---|---|
| 0 | - | arm `1417adac1` | [L] |
| 1 | - | arm `1417ad855` (the common exit) | [L] |
| **2** | `u32 pickUpCharacterId` -> `drop+0xec` | read at `1417ad903`; animation arm `1417b0311` | [L] |
| 3 | `u32 pickUpCharacterId` | same read; arm `1417b0c7f` | [L] |
| 4 | **`u32`, `u32`, `u32`** | `1417ad985`, `1417ad98f`, `1417ad997`; sets `drop+0xd4 = v + now`, `drop+0xd8 = 4` | [L] |
| 5 | `u32 pickUpCharacterId`, then `u32 petId` | `1417ad903`, then `1417b0d68` in arm `1417b0d2c` | [L] |
| 6 | - | arm `1417b112e` | [L] |
| 7 | `u32 key` | `1417ada08` in arm `1417ad9cc` | [L] |
| 8 | - | arm `1417b15ff` | [L] |
| >= 9 | - | falls past the table (`CMP R14D,0x8 / JA`) | [L] |

Names - `Fade, NoFade, CharPickup, CharPickup2, DelayedPickup, PetPickup, Fade2, Absorb` -
are **[I]** from the reference. Type 4's shape is *not* the reference's: v214 writes one
`u16` delay, mscw reads **three `u32`s**. **[L]**

**Two behaviours worth knowing.** A leaveType 4 is consumed even when the object id is not in
the pool (`0x1417ad81e`), so the packet is drained consistently; types 2/3/5/7 on an unknown
id read nothing further, which is harmless because frames are length-delimited. And type 2's
arm reads the drop's stored position back out of `drop+0x110` / `drop+0xf8` to aim the
pick-up animation, so **`drop+0xec` has to be a real character id.** **[L]**

---

## 5. `0x0301` - the MOB's pick-up (decoded, and not what we need)

```text
-> 0x0301   u32 mobObjectId     from mob+0x3a0
            u32 dropObjectId    from drop+0x64
```

Built by `FUN_141c8a4b0(mob, dropObjectId)`; the `COutPacket` call is at `0x141c8a561`. Sent
from `FUN_14179dd80`, a drop-pool sweep over a box of `x-0x14..x+0x14` by `y-0x28..y+0x0a`, called
from the mob update `FUN_141c626c0` at `0x141c630a7`. Gated on `drop+0x61 != 0 &&
drop+0xd8 == 3`, and rate limited to **one request per drop id per 3000 ms** by a list at
`mob+0x5f8`. **[L] throughout.**

Nothing latches on it - `FUN_141c8a4b0` returns as soon as it has sent - so leaving it
unanswered costs one mob's theft and nothing else. It still must not be answered with an
error; see `CLAUDE.md`.

---

## 6. What the server owes the client, and what happens if it does not pay

| packet | must answer? | what an unanswered one costs |
|---|---|---|
| **`0x0107` with `dst = 0`** (the drop) | **YES, always, including a refusal** | `0x142cc5f01` sets the latch `[ctx+0x2330] = 1` on a drop exactly as on a move, and only an inbound handler clears it. **Every later inventory action in the session is dropped before it is built.** This is the 2026-08-19 failure repeating. The refusal shape is already built: `net::inventory::inventory_rejected()`, seven bytes, `nCount = 0`. **[L]** |
| **the pick-up request** | **treat as YES** | Unknown, because the opcode is unknown - the request is queued at `pool+0x78` behind `pool+0x90 = 1` and the send is virtualised. `CLAUDE.md`'s rule is to answer anyway, and there is a concrete reason to: it goes through `CWvsContext`, the same object that owns the `0x2330` latch. **[I]** |
| `0x0301` (mob pick-up) | no, but never error | costs one mob theft. **[L]** |
| `0x046E` / `0x046F` | n/a, outbound | - |

`0x046E` and `0x046F` are one-way: the client sends nothing back for either, so a drop that
appears is not confirmation that anything else works.

---

## 7. Wire it like this

`crates/net/src/drops.rs` is built and tested; **none of it is called from anywhere**, so by
`CLAUDE.md`'s "built is not wired" rule this section is the wiring and until someone does it
the feature does not exist on screen.

### 7.1 `on_inventory_move`, when `dst == 0`

`crates/world/src/session.rs::on_inventory_move` currently rejects slot 0 before it gets
here. Insert the drop case **before** the slot validation:

```rust
use net::drops::{self, FieldDrop, ENTER_FLOATING};

let m = net::inventory::parse_inventory_move(body)?;   // already there

if drops::is_a_drop(&m) {
    let count = drops::drop_count(&m);

    // 1. Take it out of the bag, in one transaction, exactly the way the unequip does.
    //    src > 0 is a bag slot; src < 0 would be an equipped slot, and the client CAN
    //    drag straight off the character - if src < 0, refuse with inventory_rejected()
    //    until the equipped-to-ground path is tested on its own.
    let Some(item) = store.take_from_bag(char_id, m.inv_type, m.src, count) else {
        return Ok(vec![net::inventory::inventory_rejected()]);   // ALWAYS answer
    };

    // 2. Tell the client the bag changed.  Mode 3 = Remove, and it takes NO tail bytes
    //    (research/msexe-setfield.md).  For a partial stack use mode 1 UpdateQuantity
    //    with the i16 remainder instead.
    let op = net::inventory::inventory_remove(m.inv_type, m.src);   // to be added there

    // 3. Put it on the ground.  The object id is the server's, must be unique in the
    //    field, and must NOT be reused - a repeat makes the client read nothing at all.
    let d = FieldDrop::item(field.next_drop_id(), item.item_id, char_id, px, py);
    let enter = drops::drop_enter_field(&d, ENTER_FLOATING);

    field.remember_drop(d, count, Instant::now());
    return Ok(vec![op, broadcast(drops::DROP_ENTER_FIELD, enter)]);
}
```

Order on the wire: **the `0x0070` first, then the `0x046E`.** The `0x0070` is what clears the
`0x2330` latch, and until it is sent the client will refuse the player's next action.

`net::inventory::inventory_remove` does not exist yet and belongs in that file, which this
agent does not own. It is `bExclRequestSent = 1, u8 0, i32 nCount = 1, u8 0, u8 mode = 3,
i8 invType, i16 oldPos` - **seven header bytes plus a four-byte entry and no tail**, because
mode 3 takes none. Do **not** append an `avatarChanged` byte: `research/msexe-setfield.md`
warns that mode 3's flag depends on client-side inventory state the server cannot see.

### 7.2 What the world server has to remember about a drop

| | why |
|---|---|
| **object id**, unique per field | it is the pool's hash key. A repeat makes `0x046E` read the id and stop (§3.4), and `0x046F` address the wrong object |
| **item id and quantity** | to put it back in a bag on pick-up |
| **position** `(x, y)` | the client picks up by proximity, so the position is what makes it reachable. Use the dropping character's own `(x, y)`; map 1's ground line is in `research/map1-exists.md` |
| **owner character id** and **when it was dropped** | the client does not enforce ownership at all (`ownType` is stored and never tested), so the server is the only thing that can. A real service opens the drop to everyone after a few seconds |
| **expiry** | nothing in the client expires a drop by itself on the paths read here. If the server wants drops to vanish it sends `0x046F` with leaveType 0 or 1 itself |
| that it is **per field, not per character** | the pool is destroyed and rebuilt on every field entry, the same way the NPC pool is (`research/npc-spawn.md` section 6.1). **Every drop on the field must be re-sent after every `SetField`**, with `ENTER_INSTANT` so it does not re-animate |

### 7.3 On a pick-up

```rust
// opcode TBD - log it first; see UNRESOLVED item 1.
let drop = field.take_drop(request.object_id) else { return Ok(vec![/* answer anyway */]) };

// 1. Put it in the bag: 0x0070 mode 0 (Add) with the item blob, or mode 1
//    (UpdateQuantity, i16) if it stacks onto a slot that already has some.
//    The blob shapes are research/msexe-setfield.md "The item blob" and
//    net::bag::bundle_item / net::opcode::equipped_item.
// 2. Tell the field it is gone.  Broadcast, INCLUDING to the picker.
let leave = drops::drop_picked_up_by_character(drop.object_id, char_id);
```

`drop_picked_up_by_character` is leaveType **2**, whose arm plays the "flies into the
character" animation using `drop+0xec`, so the character id has to be real.

If the bag is full, send **only** the `0x046F`-less refusal - do not send a leave for a drop
that is still there, and do answer the request with something.

---

## 8. What I did NOT establish

1. **The player's pick-up request opcode.** Narrowed to `0x0329..0x032E`; the reference says
   `0x032C` and the reference is the 1-of-8 instrument. The send is behind a `jmp` into the
   Themida VM section. **Measure it.**
2. **Which `DropEnterField` tail arm a real item takes**, and therefore whether the expiry
   FILETIME is on the wire for an item at all. Handled by always sending the long body with a
   zero expiry (§3.3); not resolved.
3. **The meaning of 12 of the 27 head fields** - reads 6, 7, 13, 14, 15, 16, 18, 19, 20, 21,
   26, 27, plus the two trailing `u32`s of the long tail. All are stored somewhere on the
   drop object and none is read back on any path that was followed. They are sent as zero.
4. **Whether an unanswered pick-up request latches the UI.** Argued as "assume yes" because
   the path runs through `CWvsContext`, the object that owns the `0x2330` latch. Not read.
5. **What `FUN_14182e950(field)` actually tests** - the client's own "no drops on this map"
   predicate. It returned 0 on the map the owner was standing on, because the packet was sent.
   Worth decoding before wondering why a drop silently does nothing on some map.
6. **Whether the drop pool expires drops on its own.** `drop+0x1e8`, `drop+0x1f0` and the
   `FUN_1403747c0` block are all timestamps by shape and none was traced to a timer.
7. **Mesos.** `dropType 0` and `isMoney 1` are built and length-checked, and nothing else
   about them is read - `moneyType` at `drop+0x16d` in particular.
8. ~~**`0x025F`.** Built by `FUN_1417d29d0` as `u32 itemId, u32 (arg)` and sent **six times
   from inside `DropEnterField`**. It is outbound traffic the client will produce the moment
   a drop appears.~~ **RETRACTED 2026-08-20 - see §10.** Six *call sites*, not six sends, and
   every one of them is a null-check failure branch that then abandons the drop. It is an
   error report, it will not arrive on a healthy drop, and it needs no reply.

---

## 9. The one-variant test

**One variant.** The only change is that `0x0107` with `dst == 0` is now answered and a
`0x046E` follows it. Nothing else on the wire moves, so any failure is attributable.

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

`-SetFieldProbe` is not optional - without it the channel answers nothing at all.

| # | The owner does | what to watch | what each outcome means |
|---|---|---|---|
| 1 | Enter the world on **map 1**, open the bag, drag the **Sword** out of the window and drop it on the floor | the inventory window | **The sword leaves the slot** = the `0x0070` mode 3 landed. **It snaps back** = the store wrote nothing; `world.log` names the `0x0070`. **The whole inventory UI goes dead afterwards** = the `0x0070` did not go out at all and the `0x2330` latch is stuck - that is the 2026-08-19 failure, and it is the one thing here that ends the session |
| 2 | *same moment* | the ground under the character | **A sword icon is lying there** = `0x046E` is right end to end. **Nothing on the ground, no fault** = the packet went out and the pool ignored it; check `world.log` for the `0x046E` first, then suspect the object id (a repeat is silently ignored) or `motionType` being non-zero. **The client faults or freezes** = the body is the wrong length; read `client-patched\maplecw-hook.log` for a CLIENT FAULT line and `login.log` at the **start of the next run** for the ELog the client uploads |
| 3 | Walk **onto** the sword and press the pick-up key | `world.log`'s last inbound line | **This step exists to name one opcode.** Whatever arrives that is not `0x02FF`, `0x0107` or a keep-alive is the pick-up request. Expect it in `0x0329..0x032E`. **Nothing arrives at all** = the client refused before sending - most likely `drop+0x61`, so the enter type is wrong; re-send with `ENTER_INSTANT` |
| 4 | *after step 3, whatever happened* | the UI | **The UI freezes** = the pick-up request latches and must be answered. That is itself the measurement, and `world.log`'s last inbound line names the packet nobody answered |
| 5 | Walk a portal, or `!map 40` and back | the ground | **The sword is gone** = expected, and it is not a bug: the drop pool is rebuilt empty on every field entry and nothing re-sends. It is the reminder that §7.2's last row is not optional |

**Do not** re-enable mobs in the same run (`--mobs` stays off), and do not arm the
mob-targeting watch - a `0x0301` arriving from a mob that walked onto the sword would look
exactly like the player's pick-up in the log and would be read as one.

---

## 10. `0x025F` is the client saying the drop FAILED, and it needs no reply

Decoded 2026-08-20 because §8 item 8 had it wrong in a way that would have cost a launch: it
was written up as traffic the client produces "the moment a drop appears", which made it look
like something the server has to answer or risk a frozen UI. It is the opposite of routine.

### 10.1 Six call sites, not six sends, and all six are failure branches

`tools/callers.py 0x1417d29d0` finds **6 call sites in exactly 1 function** - `FUN_1417a2ee0`,
`DropEnterField` - and **no pointer to it anywhere in the image**, so there is no vtable path
and no other caller. **[L]**

All six are byte-for-byte the same shape: **[L]**

```asm
1417a57d0  TEST  R12,R12
1417a57d3  JNE   1417a5803        ; the pointer is fine -> carry on with the drop
1417a57d5  MOV   RDX,[RBP-0x78]   ; a lazily made 0x428-byte context object
1417a57de  MOV   ECX,0x428
1417a57e3  CALL  142e52ed0
1417a57ec  MOV   R8D,0x3c8        ; <- a per-site constant
1417a57f2  LEA   RCX,[RBP+0xcf0]
1417a57f9  CALL  1417d29d0        ; -> builds and sends 0x025F
1417a57fe  JMP   1417ad50e        ; and ABANDONS the handler
```

| # | call site | code | what was null | where it goes |
|---:|---|---:|---|---|
| 1 | `1417a57f9` | `0x3c8` (968) | `r12` | `jmp 1417ad50e` |
| 2 | `1417a5f25` | `0x3d1` (977) | `[rbp+0x178]`, the result of `FUN_14039f600(…, id=[rbp+0x44])` - **a lookup that returned nothing** | `jmp 1417a71f8` |
| 3 | `1417a71c5` | `0x3e7` (999) | `r12` | destructors, then out |
| 4 | `1417a78d1` | `0x400` (1024) | `r15` | `jmp 1417ad50e` |
| 5 | `1417aac9d` | `0x4b1` (1201) | `[rbp+8]` | `jmp 1417ad4a9` |
| 6 | `1417ab85f` | `0x4ba` (1210) | `[rbp+8]` | `jmp 1417ad4a9` |

**The codes are almost certainly `__LINE__`.** They increase monotonically with the call
site's address, six times out of six - 968, 977, 999, 1024, 1201, 1210 - and the gaps scale
with the code between them. **[D]** for "monotonic with address", **[I]** for "line number".

### 10.2 The body is the code, not an item id

`FUN_1417d29d0(rcx, rdx, r8d)` stores its **third** argument immediately:

```asm
1417d2a00  MOV   RSI,RDX                  ; rsi = the 0x428 context object, NOT rcx
1417d2a03  MOV   [RBP-0x80],R8D           ; the call-site code
...
1417d2e42  JNE   1417d3733                ; if [[rsi+0x80]] != 0 the whole send is SKIPPED
1417d2e48  MOV   EDX,0x25f
1417d2e51  CALL  1406ed520                ; COutPacket(0x25F)
1417d32b2  CALL  1406ed9d0                ; w_u32  <- deobfuscated [rsi+0x90]/[rsi+0x98]
1417d32be  CALL  1406ed9d0                ; w_u32  <- [rbp-0x80], the code
1417d372d  CALL  1406ed610                ; SendPacket
1417d3733  ...                            ; stack cookie, RET
```

So the body is **`u32 <a field of the context object>, u32 <the call-site code>`**. **[L]**

The earlier note called the first field `itemId`. That is **[I]** and probably wrong: the
object is `0x428` bytes, and the drop `DropEnterField` allocates is `0x238` (§3, read 1), so
this is a different object. `+0x90`/`+0x98` is the value/key spacing this client uses for
*every* obfuscated field, so the offset matching the drop's `itemId` slot is a layout
convention, not an identification. Left unnamed.

### 10.3 Does it need a reply? No - and the honest form of that answer

| | |
|---|---|
| the builder sets **no** state | `SendPacket` at `1417d372d` is followed by the stack-cookie check and `RET` at `1417d375c`. Nothing is stored, no flag, no timer. **[L]** |
| there is no latch like `[ctx+0x2330]` | the only writes after the `COutPacket` go into its own local packet buffer. **[L]** |
| the callers discard the result | five of six `jmp` straight to the handler's exit; the sixth runs destructors first. **[L]** |
| it has **never been seen on the wire** | not one `0x025F` in any capture in `research/fixtures/` or `previous-runs/`, across 1082 `0x00D9` bodies' worth of sessions - because this server has never sent a `0x046E` for it to fail on. **[D]** |
| unanswered packets are not fatal per se | 47 distinct inbound opcodes appear in the fixtures and this server answers about a dozen; the client plays on. The ones that freeze it are the ones that **latch**, and this does not. **[D]** |

**What is not established, and must not be smoothed over:** that no *other* subsystem sets a
flag before the drop handler runs which only a reply would clear. Proving that negative means
enumerating everything that touches drop-pool state, which is exactly the shape of search
`CLAUDE.md` says usually measures itself rather than the client. What is established is
narrower and is enough to act on: **on the path from the null check to the send and back out
of the handler, the only state created is a local.**

### 10.4 So treat it as a diagnostic, and a good one

`0x025F` arriving means **our `0x046E` was rejected and the drop was abandoned** - which
otherwise looks exactly like "nothing on the ground, no fault, nothing in any log". Log it
loudly with both `u32`s: the second one names which of the six checks failed, and §10.1's
table turns that into an address.

Site 2 (`0x3d1`) is the one to expect first: it is the only site whose null comes from a
**lookup by an id** rather than from a field of the packet, so an item id the client cannot
resolve is the leading candidate. **[I]**
