# The pick-up does not latch on `pool+0x90`, and that is not what stopped the attacks

Written 2026-08-20 to answer four questions about the `0x032C` pick-up request. The first
answer is a **retraction**: `pool+0x90` is not a lock, it gates nothing, and the client
clears it itself inside the same key press that sets it. What *did* stop the owner's attacks is a
different field on a different object, read by a four-instruction predicate that the melee
attack builder, the pick-up pre-check and the drop-pool clear all share - which explains, in
one mechanism, every measured fact about that run including why `pool+0x90` looked stuck.

Tags: **[L]** read off the listing, **[D]** derived from measured bytes, **[I]** inferred.

New files beside this one: `research/msexe-pickup-latch.txt`, `research/msexe-pickup-gates.txt`.
Fixtures: `research/fixtures/pickup-032c-latches-player-world.log` and
`-hook.log`, copied out of the repo root before `previous-runs/` could roll them.

---

## 0. The short version

| question | answer |
|---|---|
| **What clears `pool+0x90`?** | `FUN_1417a2c90` at `0x1417a2d85`, and the constructor. **No inbound packet does, and none needs to** - the client clears it itself, in the same keystroke, one call after it sets it. `pool+0x90` has 3 writers and **1 reader**, and the reader is `FUN_1417a2c90`'s own anti-cheat consistency check. **It gates nothing.** [L] |
| **What actually blocked the attacks?** | `FUN_140f810b0(user+0x100)` - the predicate `(user+0x6e4 & ~1) == 0x12`. **The melee attack builder `FUN_1428c1fa0` bails out on it at `0x1428c205a`**, and so do three of the other five `0x00DF` builders. The *same* predicate skips the drop-pool clear (`0x142cb9e7c`) and refuses the pick-up pre-check (`0x142cc6f8d`). One field, all three symptoms. **[L]**. What put the user in that state is **[I]**. |
| **`0x032C` layout** | 34 bytes, laid out in §3. Only two fields are the server's business: the drop object id at 13 and the player position at 9. |
| **A successful pick-up owes** | `0x0070` **first**, then `0x046F` leaveType 2. For **mesos** the `0x0070` is replaced by `0x007C` with `bits::MESO` (bit 18, u64). §4. |
| **A refused pick-up owes** | the same `0x0070`-shaped answer - `net::inventory::inventory_rejected()`, 7 bytes - and then nothing else. §5. |

---

## 1. `pool+0x90` is not a latch. Three writers, one reader, and the reader is the anti-cheat

### 1.1 What `FUN_1417a2b00` actually does

`FUN_1417a2b00(pool, dropObjectId)`, `.pdata 0x1417a2b00 .. 0x1417a2c80`, 384 bytes. **[L]
throughout.**

```asm
1417a2b1e  MOV  RBX,[RCX+0x58]        ; map B buckets ; miss -> return 1, nothing queued
1417a2b27  MOV  R8D,[RCX+0x60]        ; map B bucket count
1417a2b6d  MOV  RBX,[RBX+0x14]        ; map B value: a PACKED POINT (lo dword x, hi dword y)
1417a2b7c  MOV  RDI,[RCX+0x10]        ; map A buckets - the drop pool's own object-id map
1417a2bf0  MOV  RSI,[RDI+0x20]        ; the CDrop shared_ptr
1417a2c0f  CMP  dword ptr [RSI+0x194],0   ; motionType.  NON-ZERO -> skip, return 0
1417a2c18  MOV  dword ptr [RBP+0x90],1    ; <-- the write everyone has been calling a latch
1417a2c22  LEA  RCX,[RBP+0x78]
1417a2c26  CALL 0x1417d3d50               ; push_back onto the list at pool+0x78
1417a2c2b  MOV  [RAX],RBX                 ; the element is the drop's POSITION, not its id
1417a2c2e  MOV  R15D,1                    ; return 1 = "queued"
```

Two corrections to `research/item-drop.md` fall straight out of this:

* **`rcx` is the drop pool, not a CWvsContext sub-object.** The map at `[rcx+0x10]`/`[rcx+0x18]`
  with the shared_ptr at `node+0x20` is byte-for-byte the map `DropLeaveField` walks on its own
  `this` at `0x1417ad7ed`..`0x1417ad8ac`, and `FUN_141790b30` - which writes the singleton
  `[0x143ACE240]` at `0x141790b5b` - initialises exactly those fields. `FUN_142cc6770` reaches
  the same object through `FUN_142d2ea10(ctx+0x2938)`, which is a two-instruction
  `return this->_Ptr` getter. **[L]**
* **The queue holds positions, not requests.** `pool+0x78` is a list whose element is the
  qword from map B, and `FUN_1417a2c90` reads it back as a packed `POINT`. `pool+0x7c` is the
  list's count, `pool+0x80` its head, `pool+0x88` its tail - `FUN_1417d4770` (list::clear)
  zeroes `+0x10`, `+0x8`, `+0x4` of `pool+0x78`, which are those three. **[L]**

### 1.2 The complete writer/reader set

`pool+0x90` is a **dword**. Every `[reg+0x90]` operand in the drop pool's whole code
neighbourhood (`0x141780000..0x1417f0000`) plus every other function in the image that holds
the pool pointer:

| | address | in | what |
|---|---|---|---|
| write `0` | `0x141790bcd` | `FUN_141790b30` | the constructor |
| write `1` | `0x1417a2c18` | `FUN_1417a2b00` | the queue |
| write `0` | `0x1417a2d85` | `FUN_1417a2c90` | the validate-and-clear |
| **read** | `0x1417a2cc1` | `FUN_1417a2c90` | `CMP dword ptr [RCX+0x90],0` |

**That is the entire set.** [L] Nothing tests `pool+0x90` before allowing an attack, a move,
a second pick-up, or anything else. The only code that reads it is the function that clears
it, four instructions into itself.

### 1.3 The instrument, and the blind spot it had

`tools/fieldrefs.py` and `tools/rangescan.py` **both drop `rbp`-based operands** as "stack
frames" (`SKIP_BASES`). `FUN_1417a2b00` is compiled without a frame pointer: it saves `rbp`
with `MOV [RSP+0x10],RBP` and then uses it as a general-purpose register holding `this`. So
the one write everybody cares about is `MOV dword ptr [RBP+0x90],1` and **`fieldrefs.py`
cannot see it.**

```
$ python tools/fieldrefs.py 0x90 --lo 0x141780000 --hi 0x1417f0000 | grep 1417a2c18
                                    (nothing - 211 rows, and the known writer is not one)
```

This is the same class of failure `CLAUDE.md` records for the `lea`'d-pointer write scan on
`mob+0x42c`: an instrument with a structural blind spot returning a clean, confident,
incomplete list. The scan behind the table above is a copy of `fieldrefs.py` with `rbp`
**kept** (`SKIP` reduced to `rsp`/`rip`), piped in from the scratchpad so `sys.path[0]` is
empty. It returns 296 rows where `fieldrefs.py` returns 211, and

```
65:1417a2c18  mov  dword ptr [rbp + 0x90], 1   in 0x1417a2b00
```

is the positive control that says it can speak. Every row is then classified by which object
`this` is: the great majority of `[reg+0x90]` rows in that range are the drop object's
obfuscated-field triple (`mov r8d,[X+0x90]` / `mov [X+0x90],r8d` / `mov [X+0x90],ecx`, the
value/key pair at `drop+0x90`/`drop+0x98` that `research/item-drop.md` §3 read 8 names), and
`CField::OnPacket` uses `[field+0x90]` as a **qword**. Neither is the pool.

> This tool is not committed. It is 90 lines and it exists because `fieldrefs.py` is
> *right* to drop `rbp` most of the time - in this binary it is wrong perhaps one function in
> fifty, and this was one of them. If someone wants it in `tools/`, it should be a `--keep-rbp`
> flag with the `1417a2c18` row as its documented control, not a second tool.

### 1.4 What `FUN_1417a2c90` is, and why the queue exists at all

`FUN_1417a2c90(pool, POINT* myPos)`, `.pdata 0x1417a2c90 .. 0x1417a2dba`. **[L]:**

```asm
1417a2cae  CMP  dword ptr [RCX+0x7c],0     ; queue is non-empty
1417a2cc1  CMP  dword ptr [RCX+0x90],0     ;   ...and the flag is NOT set
1417a2cd2  CALL 0x142dd4d30                ;      -> send 0x01A0 with code 1
...        ; then: for each queued POINT, if it is OUTSIDE the box
           ;       x-0x19..x+0x19 by y-0x32..y+0x0a around myPos:
1417a2d7c  CALL 0x142dd4d30                ;      -> send 0x01A0 with code 2
1417a2d81  LEA  RCX,[RDI+0x78]
1417a2d85  MOV  dword ptr [RDI+0x90],0     ; clear
1417a2d8f  CALL 0x1417d4770                ; empty the queue
1417a2db5  JMP  0x142cedbc0
```

`FUN_142dd4d30(ignored, u32 code)` is a **packet builder**: `COutPacket(0x01A0)`, one `u32`,
`SendPacket`. `0x1417a2c90` is therefore a **client-side anti-cheat consistency check** - "the
pick-ups I queued this frame must all have been in reach of where I am, and the flag must
agree with the queue" - and `pool+0x90` is the flag half of that pair, nothing more. **[L]**
for the shape, **[I]** for the name "anti-cheat".

The box is the same box the pick-up sweep uses at `0x14179ca54`, which is what identifies the
two as a matched pair.

### 1.5 Set and cleared inside one key press

`FUN_1417a2c90` has **zero direct callers**. It is reached only by the singleton thunk
`0x142dd4da0` (`mov rcx,[rip+…]` / `jmp 0x1417a2c90`), which has three call sites. One of them
is `FUN_142cb9df0`, and that function does **both halves in sequence**: **[L]**

```asm
142cb9e64  MOV  RCX,[RIP+0xe143d5]   ; [0x143ACE240] the drop pool
142cb9e70  CALL 0x14179c9d0          ; the sweep  -> ... -> FUN_1417a2b00: queue, pool+0x90 = 1
142cb9e7c  CALL 0x140f810b0          ; if (user+0x6e4 in {0x12,0x13})  -> SKIP the clear
142cb9e88  CALL 0x142889da0          ; if non-zero                     -> SKIP the clear
142cb9e99  CALL 0x142dd4da0          ; -> FUN_1417a2c90: validate, pool+0x90 = 0, queue emptied
```

So under normal operation the flag never survives the keystroke that sets it, and **no server
packet is involved in clearing it at any point.** [L]

The two predicates make the clear conditional, which is the only way `pool+0x90` can persist -
and even then it costs nothing, because nothing reads it except the next call to
`FUN_1417a2c90`, which clears it anyway.

### 1.6 So `research/item-drop.md` §6 and §8 item 4 are wrong on this point

That file says the request *"is queued at `pool+0x78` behind `pool+0x90 = 1`"* and argues
"assume the pick-up latches" from it. The queue and the flag are real; **"behind" is not.**
Nothing waits on either. I have not edited `item-drop.md`; this section is the retraction.

What that file gets right and this does not touch: the `0x0107` latch at `[ctx+0x2330]` is
real and is [L]. §2.2.2 shows it gates the pick-up sweep as well, which is a *different* and
still-live reason to answer every request.

---

## 2. What did stop the attacks: `user+0x6e4`, and the attack builder itself reads it

### 2.1 What the run measured

All from `research/fixtures/pickup-032c-latches-player-world.log`. The pick-up is at
**18:35:40.733**; the log runs to 18:42:15.

| | before | after |
|---|---:|---:|
| `0x00DF` melee attack | **9** | **0** |
| `0x00D9` user move | 24 | **44**, the last at **18:42:10** |
| `0x02FF` mob move | 617 | 9 882 |
| `0x032C` pick-up | 0 | **1, ever** |
| `0x0107` inventory | **0 in the entire session** | 0 |

Three things follow and all three matter:

1. **The client is not frozen and the player is not frozen.** They walked for another six and a
   half minutes. This is *not* the "unanswered packet freezes the whole UI" failure.
2. **Attacks stopped dead, and the client agrees.** `0x013D` is a periodic outbound report the
   client sends every 30 s; §2.3 decodes it, and after the pick-up every single one says
   `{opcode 0xDF: 9}` - the client's own count of how many `0x00DF` it has sent, frozen at 9,
   matching the 9 that arrived. **The client stopped *building* attacks; nothing was lost on
   the wire.** [D]
3. **Only one `0x032C` was ever sent**, with two drops still on the ground and a user actively
   trying. Whatever stopped the attack stopped the pick-up too.

### 2.2 One four-instruction predicate explains all three

```asm
140f810b0  MOV   EDX,[RCX+0x5e4]
140f810b8  AND   EDX,0xfffffffe
140f810bb  CMP   EDX,0x12
140f810be  SETE  AL
140f810c1  RET
```

`FUN_140f810b0(x)` is `x->m_nState is 0x12 or 0x13`. It has no `.pdata` entry - a 17-byte leaf
`tools/listing.py` refuses - and **every** call site in the image passes `someUser + 0x100`, so
what it tests is a field on a subobject at `user+0x100`, i.e. `user+0x6e4`. 199 callers. **[L]**

Three of those callers are the entire story: **[L]**

| call site | in | what happens when it returns 1 |
|---|---|---|
| **`0x1428c2053`** | **`FUN_1428c1fa0`, the melee-attack builder** | `TEST EAX,EAX` / **`JNE 0x1428c5a03`** - the function abandons. **The `0x00DF` is never built.** |
| `0x142cb9e7c` | `FUN_142cb9df0`, the pick-up key handler | **skips `FUN_1417a2c90`** - `pool+0x90` and the queue are left set (§1.5) |
| `0x142cc6f8d` | `FUN_142cc6770`, the pick-up pre-check | shows string `0x12d2` and **refuses the pick-up** before anything is queued |

The first row is the one that matters and it is not an inference about a jump table: it is the
gate, 60 instructions into the builder this project already has 155 KB of listing for
(`research/msexe-attack-builder-1428c1fa0.txt` line 44). **Four of the six `0x00DF` builders
call it** - `FUN_1428bc4d0`, `FUN_1428c01b0`, `FUN_1428c1fa0`, `FUN_1428c5aa0`; the two that
do not are `FUN_1428baa50` and `FUN_1429a6590`. **[L]**

So one field, `user+0x6e4`, stuck at 18 or 19, produces:

* attacks that stop being built (and therefore a `0x013D` counter frozen at 9 - §2.3),
* a pick-up request that is refused before it is sent (and therefore exactly one `0x032C`),
* a `pool+0x90` that never gets cleared (and therefore the appearance of a latch),
* and **no effect on walking whatever**, because nothing in the movement path reads it.

### 2.2.1 The state has exactly one setter, and 18/19 is special to it

```asm
140f810f0  TEST  R8D,R8D            ; force
140f810f5  CMP   EDX,[RCX+0x5e4]    ; unchanged and not forced -> return
140f81108  AND   EAX,0xfffffffe
140f8110b  MOV   [RCX+0x5e4],EDX    ; the store
140f81111  CMP   EAX,0x12
140f81114  JNE   0x140f81125
140f81116  MOV   RAX,[RCX]
140f81119  CALL  [RAX+0xa0]         ; ENTERING 0x12/0x13 fires a virtual of its own
```

`FUN_140f810e0(this, newState, force, r9d)`. **[L]** `tools/pdata_lookup.py` reports its entry
as `0x140f810e0 .. 0x140f81103`, which is short - the body branches to `0x140f8121f` - so this
is one of the functions `.pdata` splits across contiguous entries. Bound any dump of it by the
branch targets, not by that number.

A whole-`.text`+`.boot` scan for `[reg+0x5e4]` stores returns **26** rows, and 25 of them are a
different class at the same displacement - `movsd` of a double, a `mov byte`, constants `0x18`,
`0x115`, `0x3f800000`, `0xffa5a198`. **`0x140f8110b` is the only writer of this field on this
class.** [L] Its one direct caller is `0x14276eed1` inside `FUN_14276ecf0`, itself slot 82 of
the 219-entry vtable at `0x14337f188`, and the value it passes is a register computed higher up
in that function. **The chain from "the pick-up happened" to "the state became 18" is not
read**, and that is the honest boundary of this section.

### 2.2.2 The CWvsContext latch is still real, and it is still the pick-up's problem

`FUN_142cc42d0(CWvsContext* ctx, int minMs, int skipUiCheck) -> BOOL`, `0x142cc42d0`. **[L]:**

```asm
142cc42da  CMP  dword ptr [RCX+0x2338],0   ; jne -> return 0
142cc42e8  CMP  dword ptr [RCX+0x2330],0   ; jne -> return 0     <- the exclusive-request latch
142cc42f1  TEST R8D,R8D                    ; skipUiCheck
142cc42f6  MOV  RAX,[RCX+0x2358]           ; if 0 -> return 0
142cc4309  CALL 0x1401ba9d0                ; if <= 0 -> return 0
142cc4317  SUB  EAX,[RBX+0x2334]           ; now - lastRequestTick
142cc431d  CMP  EAX,EDI                    ; < minMs -> return 0
```

Those are exactly the fields `research/item-drop.md` §1 documents for `0x0107`: `FUN_142cc5b00`,
the **only** builder of `0x0107` in the image, sets `[ctx+0x2330] = 1` at `0x142cc5f01` and
restamps `[ctx+0x2334]` at `0x142cc5f10`, on a drop exactly as on a move.

264 call sites in 192 functions. **`FUN_14179c9d0`, the player pick-up sweep, calls it in its
own first basic block** (`0x14179ca24`, `minMs = 0x1e`), and `FUN_142cc6770` calls it again at
`0x142cc6d84`. **[L]** So an unanswered inventory-style request *does* block pick-ups - §5
turns that into a rule.

**What it does NOT do is block the attack, and that was checked rather than assumed.** Of the
19 direct callers of the six `0x00DF` builders, **zero** appear in `FUN_142cc42d0`'s 192-caller
list. The two `FUN_142cc42d0` call sites inside the key handler `FUN_1428af6d0` (`0x1428af83e`
with `minMs = 0xc8`, `0x1428af931`) guard *some* key arm, and which arm is **not** established -
`FUN_1428af6d0` reaches no `0x00DF` builder in the readable image. An earlier draft of this file
said "the attack key action" there; that was wrong and this replaces it.

Independently: a scan of `0x142880000..0x1429c0000` for `[reg+0x2330]` returns two rows, both
`movsd` inside `FUN_1429446b0`, an unrelated doubles copy. **[L]**

### 2.3 `0x013D` is the client's own outbound-opcode counter, and it is a free instrument

Built by `FUN_142d19260` (`COutPacket` at `0x142d192d9`). Body, **[L]** from the encode order:

```
u8   changed              ; 1 when [ctx+0x232c] != [ctx+0x4070]
u32  outerCount           ; [ctx+0x4068]
repeat:                   ; std::map<u32, std::map<u32,u32>> at [ctx+0x4060]
    u32 opcode            ; node+0x20, the outer key
    u32 innerCount        ; node+0x30, the inner map's _Mysize
    repeat:
        u32 key           ; inner+0x1c
        u32 value         ; inner+0x20
u32  trailer
```

Observed after the pick-up, every 30 s, unchanged, thirteen times:

```
00 01000000 df000000 01000000 00000000 09000000 00000000
   count=1  op=0xDF  n=1      key=0    value=9  0
```

Nine. `0x00DF` arrived nine times. **[D]** This is worth keeping: it is a free, client-side
witness to what the client thinks it sent, and it is already in every capture.

### 2.4 What is [L] and what is [I], said plainly

**[L]** — `user+0x6e4 ∈ {0x12,0x13}` makes the melee-attack builder abandon, makes the pick-up
pre-check refuse, and makes the drop-pool clear be skipped. **[L]** — one setter,
`FUN_140f810e0`, and entering that state fires its own virtual. **[measured]** — after
18:35:40.733, zero attacks were built, one pick-up request was ever sent, and the player kept
walking for 6.5 minutes.

**[I]** — that the pick-up run is what put the user into that state, and that the state is
what fired rather than one of the other 196 things `FUN_140f810b0` guards. Two facts make it
the leading candidate rather than a guess: it is the *only* predicate the attack builder, the
pick-up pre-check and the drop-pool clear share, and all three symptoms appeared at the same
instant. That is a coincidence of three, not of one.

**[I]** — that anything set `[ctx+0x2330]`. The only place the pick-up could set it is the
tail of `FUN_142cc6770`:

```asm
142cc722a  CALL 0x1417a2b00        ; queue; returns 1
142cc722f  MOV  [RSP+0xcc],EAX
142cc7236  JMP  0x144f14cca        ; -> .themida
```

`.themida` has **`SizeOfRawData` = 0**. There are no bytes on disk to disassemble - not
"obfuscated", *absent*. That is the proved negative, and its control is the section table
itself: every other section this project reads has a non-zero raw size. `0x032C` therefore
cannot be walked at all, in either direction, by anybody.

**Three of the five `ClearExclRequest` helpers have zero callers** in `tools/callers.py`'s
call, tail-jump *and* data-pointer scans, so the inbound handlers that clear `[ctx+0x2330]`
are virtualised too. `item-drop.md`'s "only an inbound `0x0070` clears it" is **[I]** on that
same evidence and should stop being written as though it were read.

### 2.5 How to settle it, and it does not need a client run to start

**Cheapest first, and it costs nothing.** `client-patched` already logs `WATCH` lines.
**Arm a watch on `FUN_140f810e0` (`0x140f810e0`) and log `edx`** - the new state - and
`rcx`. One ordinary play session then prints the whole state machine: what the state is while
walking, what it becomes on a pick-up press, and whether it ever comes back. That is a single
`WATCH` line against a 35-byte function with one caller, and it answers §2 outright instead
of narrowing it. Do this before designing a packet experiment.

**The packet experiment, if a watch is not possible.** Change one thing: on the `0x032C`,
send a `0x0070` back **before** the `0x046F`. `net::inventory::inventory_rejected()` already
exists - seven bytes, `nCount = 0` - so it touches no other subsystem and needs no meso or bag
work.

| outcome | what it means |
|---|---|
| a **second `0x032C`** arrives | the pick-up's own gate (`FUN_142cc42d0`, §2.2.2) had been closed and `0x0070` opened it. §4/§5's packet order is right and should ship |
| attacks also resume | the same latch was behind both, and §2.2's state predicate was a red herring |
| no second `0x032C`, no attacks | it is the `user+0x6e4` state. Nothing the server sends will clear it; go to the watch above |

The **second `0x032C`** is the line to grep for. It separates the two mechanisms on its own,
independently of whether attacks come back, and it is one `grep " <- 0x032C" world.log`.

---

## 3. The `0x032C` body, 34 bytes

**The builder cannot be read.** It is behind `jmp 0x144f14cca` into `.themida`, which has zero
raw bytes. So every row below is **[D]** from the one measured body cross-referenced against
other packets from the same client, or **[I]**. Nothing here is [L].

Measured, once, at 18:35:40.733:

```
00 e471ed0a 00000000 5003 8b01 032d3101 00000000 01 400b2700 400b2700 0c000000
```

| off | size | value | reading | tag |
|---:|---|---|---|---|
| 0 | u8 | `0x00` | unknown. `0x00D9` USER_MOVE opens with the same `u8 0` | [I] |
| 1 | u32 | `0x0AED71E4` | **tick, in milliseconds** | **[D]** |
| 5 | u32 | `0` | unknown. `0x00D9` has a `u32 0` in the same relative place | [I] |
| 9 | i16 | `848` | **the player's x** | **[D]** |
| 11 | i16 | `395` | **the player's y** | **[D]** |
| **13** | u32 | `20000003` | **the drop object id** | **measured, logged by the server** |
| 17 | u32 | `0` | unknown | [I] |
| 21 | u8 | `1` | unknown | [I] |
| 22 | u32 | `0x00270B40` | unknown; candidate = arg 4 of `FUN_142cc6770` | [I] |
| 26 | u32 | `0x00270B40` | unknown; identical to the previous field | [I] |
| 30 | u32 | `12` | candidate = arg 5 of `FUN_142cc6770`, `(int)drop+0x214 / 2` | [I] |

### 3.1 How the tick and the position were established

**Position.** The `0x00D9` sent 288 ms earlier ends its path at `50038b01`, and the
`0x032C` carries `50038b01` at offset 9. The client's point encoding is `i16 x, i16 y` little
-endian: every path element of that same `0x00D9` is `xx xx yy yy` with `y = 0x018B = 395`
throughout, and the header pair `82038b01` decodes to (898, 395), a plausible walk start.
`0x0350 = 848`, `0x018B = 395`. **[D]**

**Tick.** Six samples, all from this one capture, all agreeing that the unit is milliseconds
against the server's own timestamps:

| packet | field | value | Δ value | Δ wall clock |
|---|---|---|---:|---:|
| `0x00DF` 18:35:34.203 | | `0x0AED5858` | | |
| `0x00DF` 18:35:35.493 | | `0x0AED5D62` | 1290 | 1.290 s |
| `0x00DF` 18:35:37.153 | | `0x0AED63D4` | 1650 | 1.660 s |
| `0x00DF` 18:35:38.405 | | `0x0AED68A2` | 1230 | 1.252 s |
| `0x00D9` 18:35:40.445 | | `0x0AED70D6` | 2100 | 2.040 s |
| `0x032C` 18:35:40.733 | offset 1 | `0x0AED71E4` | 270 | 0.288 s |

**[D]**, and it is worth more than "it looks like a tick": it means offset 1 is the *same*
tick field the attack and move packets carry, so a server that ever wants to rate-limit
pick-ups has a usable clock.

### 3.2 Where the two unknown u32s probably come from

The readable half of `FUN_142cc6770` is called as **five** arguments: **[L]**

```asm
14179dd14  MOV  [RSP+0x20],EDI       ; arg5 = (int)[drop+0x214] / 2
14179dd18  MOV  R9D,[RBP-0x74]       ; arg4 = FUN_1403a1f90(<a hash map>, <an id>)
14179dd1c  MOV  R8D,[RAX+0x64]       ; arg3 = the drop object id
14179dd20  MOV  RDX,[RBP+8]          ; arg2 = &playerPos
14179dd24  MOV  RCX,[RBP+0x10]       ; arg1 = [0x143aa84a0], the CWvsContext singleton
14179dd28  CALL 0x142cc6770
```

Args 2 and 3 are on the wire at offsets 9 and 13. Args 4 and 5 are **never used in the
readable half**, which means their only consumer is the virtualised tail - i.e. they are on
the wire. `12` at offset 30 is a good fit for arg 5, since `(int)[drop+0x214] / 2` reading 12
means `drop+0x214 == 24`. **[I]**, one sample, and it does not matter: the server needs none
of them.

### 3.3 What the server should do with it

Read the `u32` at **13**. Sanity-check the `i16` pair at **9** against the pick-up box the
client itself enforces (`x-0x19..x+0x19` by `y-0x32..y+0x0a`, **[L]** at `0x14179ca54`) if the
server ever tracks a position. **Ignore everything else, and do not reject on it** - four of
the ten fields have exactly one observed value and no meaning.

---

## 4. What a SUCCESSFUL pick-up owes the client

### 4.1 An item

```
1.  0x0070   inventory Add (mode 0) or UpdateQuantity (mode 1)
2.  0x046F   DropLeaveField, leaveType 2, u32 objectId then u8 2 then u32 characterId
```

**In that order.** The reasons, ranked by how well they are established:

* **`0x0070` first is the only ordering that can clear `[ctx+0x2330]`** if the pick-up set it
  (§2). If it did not, the order costs nothing. This is the asymmetric-failure argument, the
  same one `item-drop.md` §3.3 makes about the long tail. **[I]** on the premise, but free.
* `0x046F` **must** go out or the drop stays drawn on the floor forever - nothing in the client
  expires a drop on any path anyone has followed (`item-drop.md` §8 item 6). **[L]**
* `0x046F` field order is `u32 objectId` **then** `u8 leaveType`, which is the opposite of the
  v214 reference (`item-drop.md` §4, `0x1417ad7cd` / `0x1417ad7d7`). **[L]**
* leaveType 2 reads a `u32` into `drop+0xec` and the arm at `0x1417b0311` compares it against
  `FUN_142cb9550(ctx)`, the local character id, to decide whose animation to play - so **it has
  to be a real character id.** **[L]**

**One thing to expect and not to debug:** `0x046F` type 2 computes
`r15 = ((drop+0x194) - 1) <= 1` at `0x1417b0351`, i.e. "motionType is 1 or 2", and **every
animation branch in the arm is gated on `r15`**. `net::drops` sends `motionType = 0`, so the
fly-into-the-character animation is skipped. That is not a bug to chase: `FUN_1417a2b00`
**refuses to queue a pick-up unless motionType is 0** (`0x1417a2c0f`), so 0 is the only value
that lets the request happen at all. The client accepts the leave and removes the drop either
way. **[L]** for both halves.

### 4.2 Mesos, which is the case that actually ran

**A meso pick-up needs `0x007C` with bit 18, and it needs it instead of the `0x0070`, not as
well as.** Mesos are not in a bag; there is no slot to add to. `crates/net/src/stats.rs`
already has it: `bits::MESO = 0x0004_0000`, bit 18, written as a **u64**. That bit is one of
the 18 `FUN_1402cbb50` actually tests (**[L]**) and its target `record+0xbf` appears in none
of the `SetField` stat block's 29 reads, so it is the only way to move that field
(**[L]** for the absence); *that the field is the meso balance* is **[I]**. That file's own
doc says so, and it is the same tagging.

```
1.  0x007C   StatChanged, mask |= bits::MESO, meso = the NEW TOTAL (u64)
2.  0x046F   DropLeaveField, leaveType 2
```

In the run this half was skipped entirely: the server tried to add "item id 0" to a bag,
placed nothing, and sent no `0x007C`. The `0x046E` that created drop 20000003 carries
`isMoney = 1` at body offset 6 and `dropType = 0` at offset 0, so the client knew it was money
and the server did not.

**Nothing in the client credits mesos by itself.** The type-2 arm past `0x1417b04df` is
animation and UOL string work - `FUN_140d1a5b0`, two `BSTR` paths, `FUN_140d5e2e0` - and there
is no arithmetic on a balance anywhere in it. **[L]** for what is there; **[I]** for "therefore
the server must send it", though it is the same [I] every MapleStory server makes.

### 4.3 Does the `0x0070` have to arrive before the `0x046F`?

**Nothing in the drop pool requires it.** `0x046F` looks the object id up in `pool+0x10` and
does not consult the inventory. The ordering in §4.1 is a `[ctx+0x2330]` argument, not a drop
-pool one. If a future run shows the latch is not involved, either order works. **[L]** for the
drop pool's indifference.

---

## 5. What a REFUSED pick-up owes the client

Three of the five `PickUp` outcomes in `crates/world/src/drops.rs` - `Unknown`, `NotYours`,
`Untradeable` - have no packet today, and `PickUp::replies()` returns an empty vector for
them.

**Send the `0x0070` refusal.** `net::inventory::inventory_rejected()`, seven bytes,
`nCount = 0`, already built and tested. Then the chat notice `PickUp::notice()` already
produces. Then nothing else.

The reasoning, and its honest limits:

* **It is the only answer that can clear `[ctx+0x2330]`,** and `[ctx+0x2330] != 0` provably
  closes the pick-up: `FUN_14179c9d0`, the sweep, tests it in its own first basic block
  (`0x14179ca24`, **[L]**). A refusal that sends nothing therefore risks locking **every later
  pick-up** for the rest of the session even though the drops are still on the floor - which is
  what one `0x032C` in six and a half minutes looks like. **[L]** for the gate, **[I]** for
  "the refusal is what would have closed it".
* **It cannot make things worse.** `0x0070` with `nCount = 0` describes no slot change, and
  the client has been receiving it as a refusal since 2026-08-19.
* **Do not send `0x046F` for a drop that is still there.** `Unknown`, `NotYours` and
  `Untradeable` all leave the drop on the floor; a leave would delete it from the client's
  pool and it would never come back, because the pool is only refilled by `0x046E`.
  `PickUp::Expired` is the exception - it *has* swept the drop, and its leaveType 0 is correct.
* **`pool+0x90` needs nothing.** §1. The client cleared it before the server even saw the
  request.

### 5.1 The thing that must not be done

**Never answer a `0x032C` with an error or with silence** - `CLAUDE.md`'s always-answer rule.
But note the specific shape of the evidence here: the 2026-08-20 run did *not* show the
whole-UI freeze that rule is written about. It showed a **partial** lock - one class of key
action dead, movement fine - which reads on screen as "my character is broken" rather than as
a crash, and which no watchdog will catch. That is a second failure mode worth naming, and it
is the one a pick-up can produce.

---

## 6. What is NOT established

1. **What `user+0x6e4 ∈ {0x12,0x13}` means, and what put the user in it.** The predicate and
   its three consequences are [L]; the cause is not. `FUN_140f810e0` is the only setter, its
   one caller is vtable slot 82 of `0x14337f188`, and the value passed is computed. **A
   `WATCH` on `0x140f810e0` logging `edx` settles this in one ordinary session** - §2.5.
2. **Whether the pick-up sets `[ctx+0x2330]`.** The setter would be in `.themida`, which has
   zero raw bytes on disk.
3. **Which inbound opcode clears `[ctx+0x2330]`.** Three of the five `ClearExclRequest`
   helpers have zero callers across `tools/callers.py`'s three scans, so their callers are
   virtualised. `item-drop.md`'s "`0x0070` clears it" is **[I]**, not [L], and this file did
   not manage to raise it.
4. **The meaning of six of the ten `0x032C` fields.** One sample, no builder. §3.
5. **What map B at `pool+0x58` is keyed and filled by.** It maps the drop object id to a
   packed `POINT`, and `FUN_1417a2b00` misses in it → returns 1 without queuing anything,
   which is a *success* return with no side effect. Not chased.
6. **Whether `FUN_142889da0(user)`** - the second predicate that can skip the drop-pool clear
   at `0x142cb9e88` - ever returns non-zero in practice. If it does, `pool+0x90` can persist
   across frames, and §1 says that still costs nothing, but it has not been read.
