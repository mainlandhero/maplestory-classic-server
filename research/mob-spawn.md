# Mob spawn: `0x03C6`, and where the layout stops being readable

Started 2026-08-19 after the owner made mobs and drops the priority: *"there should be tutorial
monsters spawning on East Entrance to Mushroom Town (ID 30)"*.

## Routing - settled

| | |
|---|---|
| mob pool range | `0x3C6..0x44E`, 137 opcodes, singleton at `[0x143ABFE00]` |
| dispatcher | `FUN_141D30E80` |
| **enter field** | **`case 0x3c6` -> `FUN_141d33630`** (1582 bytes) |

The exact analogue of the NPC pool's `0x44F` -> `FUN_141e36b20`. Mobs are **server-sent** for
the same reason NPCs are: the client's field loader walks the WZ `life` node only to preload
`Mob/%07d.img` (`research/npc-spawn.md`).

The data is generated: `gm-handbook/mobs.txt`, **9928 spawns across 289 maps**, from every
field's `life` where `type == "m"`. Map 30 has six of template 1.

## The head - read off the listing

Reads inside `FUN_141d33630`, bounded to `[0x141d33630, 0x141d33c5e)`. **The first scan of
this ran 4608 bytes past the function end and reported 34 reads; there are 7.** Bound every
dump by `.pdata`.

```text
0x141d3365a  u8                  read 1
0x141d3368e  u32   objectId      read 2
0x141d336a1  u32                 read 3 - ONLY when objectId == 0
0x141d336f3  u8                  read 4
0x141d33701  u32   templateId    read 5 -> FUN_140495990, the Mob/%07d.img loader  [L]
0x141d33711        (pool lookup by object id)
             ...branch: found in pool, or newly constructed...
0x141d33734  u8    |  0x141d338fa  u8      read 6 - one per branch
             --> FUN_141cc9410(mob, packet)      reads NOTHING
             --> FUN_141c76190(mob, packet, ...) reads raw[20]
```

So on the normal path - a **non-zero** object id - the head is
`u8, u32 objectId, u8, u32 templateId, u8` = **11 bytes**, then 20 raw, then the path below.

**Two values to get right, both read [L]:**

* **objectId must be non-zero.** Zero pulls in read 3, an extra `u32`, and desynchronises
  everything after it.
* **Avoid an objectId that is a multiple of 178** (`0xb2`). Both branches test
  `uVar6 == (uVar6 / 0xb2) * 0xb2` and a multiple takes a path that calls through
  `(*local_48)[2]`, a vtable slot on what looks like an exception object. Not chased - it is
  free to avoid.

## The 20-byte block, and where it stops

`FUN_141c76190` zeroes exactly 20 bytes and immediately reads 20 into them:

```asm
141c7625b  XORPS XMM0,XMM0
141c7625e  MOVUPS xmmword ptr [RBP + -0x1],XMM0    ; 16
141c76262  MOV   dword ptr [RBP + 0xf],0x0         ; + 4 = 20
141c76269  MOV   R8D,0x14                          ; 20
141c76276  CALL  0x1406e9170
```

The zeroing of exactly the width about to be read is the same corroboration the character
record's 100-byte presence array had. **[L]**

It then calls **`FUN_14046fba0(..., &block, packet)`** - which also takes the packet, and
contains **330 packet reads** in repeating `u32, u32, u16` triples. That is a movement-path
decoder: a switch over movement action types, each case reading its own fields. **So the mob
body is variable-length and does not end at 31 bytes.**

## What is NOT known, and why I stopped here

**The movement path's framing** - what selects the action type, how many elements are read,
and whether zero elements is expressible. Until that is settled the body cannot be built,
because the record has no length prefix to resynchronise on.

That is the whole remaining question, and it is worth answering properly rather than guessing:
`research/npc-spawn.md`'s body was **structurally perfect** and still produced nothing on
screen because two field *values* were zero. A mob body that is structurally wrong will do
worse than nothing - it will desync.

> **Next step:** read the head of `FUN_14046fba0` for the element count and the dispatch
> value, then find the cheapest path through its switch - ideally "zero elements", which is
> what a stationary spawn should need.
