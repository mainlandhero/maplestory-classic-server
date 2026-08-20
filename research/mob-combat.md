# Combat: the attack packet, the death packet, and the stat change

Started 2026-08-19 after the owner reported that mobs render on map 40 but *"does not have any
touch damage to the player. The player also cannot kill the mobs to gain EXP and items."*

Markers, as everywhere in `research/`: **[L]** read out of the listing, a capture or the
image; **[D]** derived from two or more [L]; **[I]** inferred - including anything from the
v214 reference at `C:\Users\user\Desktop\ModernMapleSource`, which is a **different game
version** and scored 1 of 8 against a held-out control. Every [I] is a candidate.

**No Ghidra.** Everything here is capstone over `client-patched\MapleStory.exe` bounded by
`.pdata`, `tools/reads.py`, `tools/callers.py`, `tools/xref.py`, plus the real capture in
`world.log`.

---

## 0. Answer up front

| | |
|---|---|
| the client's attack packet | **`0x00DF`** melee, **`0x00E0`** shoot, **`0x00E1`** magic - **one shared body format** [L] |
| **it is in `world.log`** | one `0x00DF`, 127 bytes, at `23:58:33.878` on map 40, and its 127 bytes are accounted for **field for field** [L] |
| **the client computes the damage itself** | the body carries a per-target list of `u64` damages. Nothing the server sends supplies a number the client does not already have [L] |
| ~~that capture carried **zero targets**~~ | **RETRACTED - see §10.** The count really was zero, but the session had **no mobs in it**, so zero was the only possible answer. There has never been a capture of this client swinging at a mob it could see |
| **the mob's health bar moves** | **`0x03F0`**, `u32 objectId, u32 hp, u8 showBar` - nine bytes, and **nothing in the client moves that bar by itself**. §12, §13 [L] |
| the mob dies with | **`0x03D1`**, `u32 objectId, u8 deathType, u8, [u32, u32], …` [L] |
| EXP, HP, level, AP, SP all ride | **`0x007C` StatChanged**, a `u32` bitmask, **19 bits, 18 of them read** [L] |
| what is **not** established | the mob -> player touch-damage packet, and drops. See §7 |
| implemented in | `crates/net/src/combat.rs` |

---

## 1. The attack packet is `0x00DF`, and it was already in the capture

`world.log:353`, the only one in the whole session:

```text
23:58:33.878 <- 0x00DF UNKNOWN, 127 byte body
0000000000000000000000000000000001050000009fae3408010400000037acee06
d9018b0100000000d9018b010000000000000000000000000000000000000000000000
0000000100000001000000000a0055736572204d656c6565890100000000000000...
```

`55736572204d656c6565` is the ASCII string **`User Melee`**, length-prefixed `0a 00`. [L]

### 1.1 The string is an entry in a table of attack-type names

`tools/xref.py --string "User Melee"` finds four copies and **0 code references**, which is
the documented blind spot of that tool, not evidence. Dumping `.rdata` around `0x1432b3f10`
shows a table of 16-byte slots: [L]

```text
User Normal, User Melee, User Shoot, User Magic, User Body, User Bonus,
User Random Area, User Force Atom, User Force Atom Non-target, User Second Atom,
User Box2D, User Dot, User Screen, User Shoot Object, User Melee Target,
User Shoot Moving, User Event Skill Object, User Melee Skill System,
User Magic Skill System, Dragon Shoot, Dragon Magic, Zero Sub Melee, … (60+ entries)
```

So the client **names its own attack type on the wire**, as a string. That is unusual for
this game family and it is the reason the packet is self-describing.

### 1.2 Which builder ran, and why the other three do not matter here

`research/msexe-packet-fields.txt` lists **six** builders for `0x00DF`. They fall into two
groups, and this matters because they build *different bodies for the same opcode*: [L]

| group | builders | shape |
|---|---|---|
| **A** - three shared encoders | `FUN_1428baa50`, `FUN_1428c1fa0`, `FUN_1429a6590` | `COutPacket(0xDF)` then `FUN_140f31fe0` + `FUN_140f31f60` + `FUN_14083b270` |
| B - inline, classic GMS shape | `FUN_1428bc4d0`, `FUN_1428c01b0`, `FUN_1428c5aa0` | `u8, u8 (x\|0x10), u32, u32, u8, …` written inline, **no string** |

The captured body contains a length-prefixed string. **No group-B builder calls the string
writer `FUN_1406edc80` at all**, so the packet came from group A. [D]

Group A is also what `0x00E0` (`FUN_1429a7ab0`) and `0x00E1` (`FUN_1428cd6d0`,
`FUN_1429a7090`) use - `tools/callers.py 0x140f31fe0` gives exactly six call sites across
those three opcodes - so **shoot and magic share this body**. [L]
Group B is not explained. `0x00E2` (`FUN_1428d07c0`) uses the classic shape too. Treat
`0x00E2` as *not covered by this layout*.

### 1.3 The header: `FUN_140f31fe0`, 40 fields, straight-line

`.pdata` bounds `0x140f31fe0..0x140f321f0`. **39 `call`s to writer primitives and one
`jmp` tail-call to the `u8` writer at `0x140f321eb`**, and *no conditional branch of any
kind*. [L]

> **The tail jump is the trap.** A scan that looks only for `call` counts 39 fields and
> lands the body one byte short. The first pass here did exactly that and had to invent a
> byte elsewhere to make 127 work. `jmp <writer>` is a field.

Byte offsets are into the `0x00DF` **body** (after the 2-byte opcode). `struct` is the
offset in the source object, which is what pins the field boundaries.

| # | off | w | struct | captured value | note |
|---:|---:|---|---|---|---|
| 0 | 0 | u8 | `+0x00` | 0 | |
| 1 | 1 | u8 | `+0x04` | 0 | |
| 2 | 2 | u32 | `+0x08` | 0 | |
| 3 | 6 | u8 | `+0x0c` | 0 | |
| 4 | 7 | u8 | bool | 0 | `setne dl` |
| 5 | 8 | u32 | `+0x14` | 0 | |
| 6 | 12 | u32 | `+0x18` | 0 | |
| 7 | 16 | u8 | bool | **1** | `setne dl` |
| 8 | 17 | u32 | `+0x20` | **5** | |
| 9 | 21 | u32 | `+0x24` | `0x0834ae9f` | looks like a per-attack serial / nonce **[I]** |
| 10 | 25 | u8 | `+0x28` | **1** | |
| 11 | 26 | u32 | `+0x2c` | **4** | |
| 12 | 30 | u32 | `+0x30` | `0x06eeac37` | **the tick** - see 1.5 |
| 13 | 34 | u16 | `+0x34` | **473** | **x** - see 1.5 |
| 14 | 36 | u16 | `+0x38` | **395** | **y** |
| 15 | 38 | u32 | `+0x3c` | 0 | |
| 16 | 42 | u16 | `+0x40` | 473 | x again |
| 17 | 44 | u16 | `+0x44` | 395 | y again |
| 18 | 46 | u8 | bool | 0 | |
| 19 | 47 | u8 | `+0x48` | 0 | |
| 20 | 48 | u8 | bool | 0 | |
| 21 | 49 | u8 | bool | 0 | |
| 22 | 50 | u32 | `+0x50` | 0 | |
| 23 | 54 | u32 | `+0x54` | 0 | |
| 24 | 58 | u16 | `+0x58` | 0 | |
| 25 | 60 | u32 | `+0x60` | 0 | |
| 26 | 64 | u16 | `+0x64` | 0 | |
| 27 | 66 | u16 | `+0x68` | 0 | |
| 28 | 68 | u16 | `+0x6c` | 0 | |
| 29 | 70 | u16 | `+0x70` | 0 | |
| 30 | 72 | u32 | `+0x74` | **1** | |
| 31 | 76 | u8 | bool | **1** | |
| 32 | 77 | u32 | `+0x7c` | 0 | |
| 33 | 81 | **str** | `+0x80` | **"User Melee"** | the attack type |
| 34 | 93 | u32 | `+0x88` | **0x189 = 393** | |
| 35 | 97 | u32 | `+0x90` | 0 | |
| 36 | 101 | u32 | `+0x94` | 0 | |
| 37 | 105 | u16 | `+0x98` | 0 | |
| 38 | 107 | u16 | `+0x9c` | 0 | |
| 39 | 109 | u8 | `+0xa4` | 0 | **the tail jump** |

**12 x `u8`, 11 x `u16`, 16 x `u32`, one `str`** = `12 + 22 + 64 = 98` fixed bytes plus
`2 + len`. For `"User Melee"`, **110**. [D]

> The first count here said "11 `u8`s, 97 fixed" and was one short - the same off-by-one
> the tail jump causes, made a second time by hand while writing the table up. The test
> `the_captured_length_is_accounted_for_byte_for_byte` now decomposes the constant, so a
> census error fails in `cargo test` rather than on the wire.

### 1.4 The target list: `FUN_140f31f60`, and the trailer

```asm
140f31f7a  u32  [rcx+0]        ; targetCount            body 110
140f31f85  u32  [rcx+4]        ;                        body 114
140f31f90  u32  [rcx+8]        ;                        body 118
140f31f97  cmp  [rdi],ebx / jle …     ; loop bound IS the first u32
140f31fa6  imul rcx,rax,0x1d8         ; 0x1d8-byte stride per target
140f31fb4  call 140f31bb0             ; one target
```

then `FUN_14083b270(packet, 0)` writes a final `u32` + `u8`. [L]

`110 + 12 + 5 = 127`, which is **exactly** the captured length with `targetCount == 0`.
Every one of the 127 bytes is accounted for. [D]

### 1.5 Two independent checks that the alignment is right, not just arithmetically lucky

* **Field 12 is a tick.** `0x06eeac37`. The `0x00D9` movement packet 852 ms earlier carries
  `0x06eea8d1` in its own tick field; the difference is `0x366` = **870**. Agreement to
  18 ms. [L]
* **Fields 13/14 are the player's position.** `473, 395`. The `0x00D9` at `23:58:34.046`
  reports the walk *starting* at `d9 01 8b 01` = `473, 395`. [L]

### 1.6 One target block - `FUN_140f31bb0`

Every length on the wire is either fixed or prefixed, so this is parseable end to end. Two
of the counts are **not** what they look like and are called out below.

```text
u32   objectId          [t+0x10]     <- the mob, see the caveat below
u32   ?                 [t+0x14]
u8    nDamage           [t+0x20]     <- written u8; the LOOP BOUND is the dword there
nDamage x {
   u8   flagA           [t+0x28+i*0x10] != 0
   u8   flagB           [t+0x29+i*0x10] != 0
   u64  damage          [t+0x30+i*0x10]
}
u8  bool [t+0x118]   u8 [t+0x11c]   u8 [t+0x120]   u8 [t+0x124]
u16 [t+0x128] [t+0x12c] [t+0x130] [t+0x134] [t+0x138] [t+0x13c]
u32 [t+0x148]   u32 [t+0x14c]
u8  [t+0x140]   u8 [t+0x144]   u8 [t+0x150]
u32 [t+0x154]
u8  bool [t+0x158]   u8 [t+0x15c]   u8 [t+0x160]   u8 bool [t+0x164]
u16 [t+0x168] [t+0x16c] [t+0x170] [t+0x174]
u32 [t+0x18]   u32 [t+0x1c]   u32 [t+0x188]
u16 mapSize             [t+0x198]    <- see below
mapSize x { u32 key, u32 value }
u8  hasObj  ([t+0x1a0] != 0)
   if hasObj: FUN_14025d3c0 -> u32, u32, raw(n)      <- n NOT resolved. See §7
u8  mode                [t+0x1b0]
   mode == 1: str, u32, u8, u8 hasSub, if hasSub: FUN_141fde400 -> u32, str
   mode == 2: str, u32, u8
   otherwise: nothing
```

**The `u16` before the pair loop really is its count.** The loop is a red-black-tree
in-order walk with no count written anywhere:

```asm
140f31dc6  movzx edx, word ptr [rsi+0x198] / call u16
140f31dd5  mov rbx,[rsi+0x190] / mov rbx,[rbx]      ; _Myhead
140f31ddf  cmp byte [rbx+0x19],0 / jne end          ; _Isnil
   loop:   u32 [rbx+0x1c] ; u32 [rbx+0x20] ; in-order successor
```

`_Myhead` at `+0x190` and `_Mysize` at `+0x198` is the MSVC `std::map` layout, so the
`u16` written one instruction earlier **is** the element count. [D] Getting this wrong is
the difference between a parser that works and one that desynchronises.

**`mode` is a 3-way, not a flag.** `mov ecx,[rsi+0x1b0] / sub ecx,1 / je <mode 1> /
cmp ecx,1 / jne <done>` - so 1 and 2 take *different* branches and 0 takes neither. [L]
This is exactly the "distinguish a real count from a flag" hazard.

> **Caveat on the two leading `u32`s.** Which of `[t+0x10]` and `[t+0x14]` is the mob's
> object id has **not** been established - the capture had no targets, and the field is
> filled during target collection in a 15 kB function that was not read. `[t+0x10]` is
> first, which is the reference server's order, so the parser exposes both and names the
> first `object_id` **[I]**. One client run with a connecting hit settles it: the id we
> assigned is 2000-2039 on map 40.

---

## 2. Nothing the server sends draws the damage numbers

The attack body carries `u64 damage` per hit, computed by the client. A client that has
already computed the number is not waiting for the server to tell it what the number is.
**[D]**, with the mechanism, and it is the single most consequential claim in this file:

> **"Attack a snail and see damage numbers" is not blocked on a reply. It is blocked
> earlier, on the client putting a target in the list at all** - and the one capture we have
> says it put **zero** in.

That splits the reported symptom in two:

| symptom | what it needs |
|---|---|
| no damage numbers | the client to *register* the mob as a target. Server sends nothing here |
| the mob never dies, no EXP, no drops | the server to read the packet, apply damage, and send `0x03D1` / `0x007C` / a drop |

The second half is buildable now and is what `crates/net/src/combat.rs` does. The first
half needs one client run to separate "they swung out of range" from "the client will not
target our mobs at all"; see §8.

### 2.1 What `0x03D6` is, and what it is not

The mob pool's `case 0x3d6` looked like the damage packet and it is **not**. Body: [L]

```text
u32 count
count x {
   u32 objectId
   u32 v
   u8  hasPos    if set: u32 x, u32 y
   u32 userObjectId
}
```
-> `FUN_141c99730(mob, v, &pos, userObjectId)` -> tail-jumps to `FUN_141d23960(v, mob,
&pos, isLocalUser)`.

`FUN_141d23960` uses `v` as a **key into a global template table** (`FUN_1407b2910`), pulls
a wide string out of the record, copies it, and draws it at `(x, y)`; when the position is
absent it falls back to the **centre of the mob's bounding rect**:

```asm
141d23be9  mov rax,[r12] / call [rax+0x10]     ; mob->GetRect(&r, 1)
141d23c14  edi = left + (right-left)/2         ; x
141d23c31  esi = top  + (bottom-top)/2         ; y
```

So `0x03D6` shows a **string from a template** over a mob. The template accessor
`FUN_14079fe90` is the same one every attack builder calls, which makes `v` a **skill id**
rather than a number. **[D]** on the mechanism, **[I]** on the name - it has the shape of
`MobSpecialEffectBySkill`, not `MobDamaged`. **It is deliberately not implemented**, because
building it as a damage packet would be a guess dressed as a fact.

---

## 3. The mob dies: `0x03D1`

Routed by the mob pool `FUN_141d30e80` `case 0x3d1` -> `FUN_141d33c70`
(`.pdata 0x141d33c70..0x141d3426c`). Reads, in order: [L]

```text
u32 objectId                    141d33c92
u8  deathType                   141d33ca5   -> edi
u8  ?                           141d33cb0   -> FUN_141c56670(mob, v)
if FUN_1402b3b80(deathType):                 141d33ccf
    u32 a                       141d33cdb   -> FUN_141c56640(mob, deathType, a)
    u32 b                       141d33cea   -> FUN_141c56660(mob, b)
    if deathType == 4:  u32     141d33cfa
if deathType == 9:      u32     141d33f16
```

`FUN_1402b3b80` is a 12-entry jump table at `0x1402b3ba4`, read out of the image: [L]

| deathType | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | >= 12 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| two `u32`s follow | yes | **yes** | no | no | yes | no | yes | yes | yes | yes | yes | yes | no |

The branches after the pool lookup: [L]

* `deathType == 1` -> `FUN_142d156a0(global)`, then the common removal tail. **This is the
  animated death**, and it is the only type that notifies a global singleton - the shape of
  "tell the quest / monster-book system something died". **[D]** on the branch, **[I]** on
  the name.
* `deathType == 0` -> `FUN_141c543c0` / `FUN_141c543e0` / `FUN_141c54dd0` - an immediate
  teardown with no notification. This is "just remove it".
* `6` and `9` have their own paths; everything else falls through.

**14 bytes** for `deathType == 1`: `4 + 1 + 1 + 4 + 4`.

---

## 4. `0x007C` is StatChanged, and this is the whole mask

The single most useful thing in this file for everything after combat. Handler
`FUN_142d54780` (7144 bytes), gamestage `case 0x7c`. `tools/reads.py` says it reads **nine
fields directly** plus one helper - and that helper is the mask decoder. [L]

### 4.1 The body

```text
u8   exclRequest        142d547ad   non-zero -> FUN_142cc4430(ctx, 0)
u8   quiet              142d547d4   -> a local, see 4.4
u8   ?                  142d547e2   -> ctx+0x3bbc, see 4.4
u32  mask               1402cbb71   \
     ... fields, in ascending bit order ...   | FUN_1402cbb50
u8   flagA              142d548aa
     if flagA: u8       142d548b6   -> FUN_1428a7f00(global, v)
u8   flagB              142d548d2
     if flagB: u32, u32 142d548de / 142d548e8 -> FUN_140fd31f0(global, …)
```

Minimum body with an empty mask: **9 bytes**. Nothing else in the function reads. [L]

### 4.2 The mask - 19 bits, 18 read

The decoder is `FUN_1402cbb50`. **Its `.pdata` entry stops at `0x1402cbbb4` and the
function does not** - it continues through four more `.pdata` entries to `0x1402cbf70`. A
scan bounded by the first entry sees only three bits. [L]

Every destination offset below is a field of the **same object** the character-record stat
decoder `FUN_140302e30` fills, and `research/charstat-layout.md` already published that
offset table. Matching them is what gives every bit a name without a single reference-server
guess: **the bit order and the record order agree, field for field.** [D]

| bit | mask | wire | writes | record read | **field** |
|---:|---|---|---|---|---|
| 0 | `0x00000001` | `u8` then `u32` | `+0x1a`, `+0x1b` | 6, 7 | **skin** (+ the record's always-zero `u32`) |
| 1 | `0x00000002` | `u32` | `+0x1f` | 8 | **face** |
| 2 | `0x00000004` | `u32` | `+0x23` | 9 | **hair** |
| 3 | `0x00000008` | — | — | — | **not read at all** (the reference's `PET` slot) |
| 4 | `0x00000010` | `u32` | obf `+0x27` | 10 | **level** |
| 5 | `0x00000020` | `u16` then `u16` | `+0x33`, `+0x10c` | 11, 25 | **job** and **subJob**, together |
| 6 | `0x00000040` | `u16` | `+0x3b` | 12 | **str** |
| 7 | `0x00000080` | `u16` | `+0x43` | 13 | **dex** |
| 8 | `0x00000100` | `u16` | `+0x4b` | 14 | **int** |
| 9 | `0x00000200` | `u16` | `+0x53` | 15 | **luk** |
| 10 | `0x00000400` | `u32` | obf `+0x5b` | 16 | **hp** |
| 11 | `0x00000800` | `u32` | obf `+0x67` | 17 | **maxHp** |
| 12 | `0x00001000` | `u32` | obf `+0x73` | 18 | **mp** |
| 13 | `0x00002000` | `u32` | obf `+0x7f` | 19 | **maxMp** |
| 14 | `0x00004000` | `u16` | `+0x8b` | 20 | **ap** |
| 15 | `0x00008000` | job-dependent, below | `+0xd7` or `+0x93` | SP fork | **sp** |
| 16 | `0x00010000` | **`u64`** | `+0x9b` | 21 | **exp** |
| 17 | `0x00020000` | `u32` | obf `+0xb3` | 22 | **fame** |
| 18 | `0x00040000` | **`u64`** | `+0xbf` | — | **meso** (no record equivalent) |

Bits **19 and above are not read**; the decoder returns immediately after bit 18. [L]

**Yes, several stats travel in one packet.** The decoder is one straight run of bit tests
over one mask, in ascending order - a level-up sending `level | maxHp | maxMp | ap | sp`
in a single `0x007C` is exactly what the code expects. [L]

### 4.3 Bit 15, the SP fork - identical to the character record's

```asm
1402cbd99  job = deobf(rbx+0x33)
1402cbda8  call 0x1403024c0(job)        ; the same predicate as the record
1402cbdaf  je 1402cbeb2                 ; NOT extended -> plain u16 -> +0x93
   extended:
1402cbdc6  FUN_1402fe400(rbx+0xd7)      ; clear the pool list
1402cbdd1  u8 count
   count x { u8 jobLevel ; u32 sp }
1402cbea4  [rbx+0xef] = sum of the sp values
```

`net::opcode::uses_extended_sp` already implements that predicate for the record, and it is
the same function on the same job value. **Job 0 is extended**, so a fresh Beginner sends
`u8 count` + `count x (u8, u32)`. [D]

### 4.4 The three header bytes, and the level-up tell

* **`exclRequest`** - non-zero calls `FUN_142cc4430(ctx, 0)`, which is
  `[ctx+0x2330] = 0` plus a tick stamp at `+0x2334`. That is **the one-request-outstanding
  latch** `STATUS.md` §2d warned about: 37 functions set it, 7 clear it, and a class of
  requests goes silently dead if nothing ever clears it. `0x007C` with `exclRequest = 1` is
  the standard release. [L]
* **`quiet`** (the second `u8`) selects which mask bits get an effect:

  ```asm
  142d549be  cmp [rbp-0x54],0 / jne …
  142d549d6  test edi, 0x40030      ; quiet == 0 -> level | job | meso
  142d549f0  test dil, 0x30         ; quiet != 0 -> level | job
  142d56242  if (mask & 0x40000) && quiet == 0 -> FUN_142d9bae0(ctx, 0)   ; the meso effect
  ```

  So a non-zero second byte **suppresses the meso-gain effect**. Send `0`. [L] for the
  branches, [D] for "0 is the ordinary case".
* **The third `u8`** goes to `ctx+0x3bbc`. A whole-subsystem scan of `0x142c..0x142e` finds
  seven writes and **no reads**: `FUN_142caa4e0` (field entry), `FUN_142ca5c50`,
  `FUN_142cad420` and `FUN_142d62e30` all store the literal **`1`**; `0x007C`, `0x007D` and
  `0x007E` store what arrived on the wire. **Send `1`** - it is the value the client writes
  for itself. [D]

> **And the client detects its own level-up.** `142d54827` deobfuscates the level
> **before** the mask decode into `[rbp+0x250]`; `142d549b2` deobfuscates it again
> **after**; `142d549c4 cmp eax,[rbp+0x250] / jle` then takes a branch only when the new
> level is **greater**. So a `0x007C` carrying a higher `level` is enough for the client to
> know a level-up happened - whether that alone plays the animation is **not** established,
> and the separate level-up effect packet is somebody else's to find. [L] for the compare,
> [I] for what it drives.

---

## 5. Where combat lives in the opcode map

Filling in `research/npc-spawn.md` §3's routing table for the mob block. `FUN_141d30e80`
dispatches `0x3c6..0x3d8` through a 19-entry table at `0x141d31184`, and **everything from
`0x3d9` to `0x44d` goes to `FUN_141d32b30`**, which reads `u32 objectId`, finds the mob,
and jumps through a **117-entry table at `0x141d33448`**. [L]

| opcode | handler | what it does |
|---|---|---|
| `0x03C6` | `FUN_141d33630` | MobEnterField - `research/mob-spawn.md` |
| `0x03D1` | `FUN_141d33c70` | **MobLeaveField** - §3 |
| `0x03D2` | inline | change controller: `u8 flag, u32 objectId`, then remove or `FUN_141d34a70` |
| `0x03D3` | inline | `u32 id, u16, u32, u32, u8` -> `FUN_141cdf1e0` |
| `0x03D4` | `FUN_141d34440` | `u32` x3 |
| `0x03D6` | inline | the per-mob skill-effect batch - §2.1 |
| `0x03D7` | inline | `u32 id, u8` -> `mob+0x1100` |
| `0x03D8` | `FUN_141d34830` | `u32` x2 |
| `0x03D9..0x044D` | `FUN_141d32b30` | **per-mob commands**, `u32 objectId` + a 117-way switch. `0x03D9` -> `FUN_141c813b0` is the first |

`0x03DA..0x03E3`, `0x03E5`, `0x03EC`, `0x03EE`, `0x0434`, `0x0439`, `0x044E` fall to the
common exit and do nothing. [L]

---

## 6. Instrument notes worth keeping

* **A tail `jmp` to a writer is a field.** `FUN_140f31fe0` ends `jmp 0x1406ed840`. Counting
  only `call` gives 39 fields where there are 40, and the missing byte gets "explained"
  somewhere else - which is what happened on the first pass here.
* **`.pdata` bounds a *chunk*, not a function.** `FUN_1402cbb50`'s entry ends at
  `0x1402cbbb4`; the function runs to `0x1402cbf70` across five entries. Bounding the mask
  decoder by the first entry reports **3 mask bits out of 18**, cleanly and confidently.
  Check whether the last instruction in the range is a `ret`.
* **`tools/callers.py` cannot see a tail call.** `FUN_141d23960` reports **0 callers** and
  is reached from `FUN_141c99730`'s `jmp`. A zero from that tool means "no `call rel32`".
* **An MSVC `std::map` on the wire looks like a missing count.** `_Myhead` / `_Mysize` are
  adjacent, so the `u16` written immediately before a tree walk *is* the walk's length.
* **`tools/xref.py --string` found 0 references to `"User Melee"`** while the string is
  plainly used - it is indexed out of a table, not `lea`'d. Same blind spot the tool
  documents.
* **The WRITE-side primitive set has now been enumerated, not filtered.** The read side has
  been wrong three times over its primitive count; the send side had never been checked.
  Every `jmp rel32` in `.text` and `.boot` landing on one of the seven `COutPacket` writers
  and preceded by `0xCC` padding: **exactly one thunk, `0x1406ee000` -> `0x1406ed840`
  (`u8`)**, sitting alone in an `int3` field. **None of the six encoders in this file calls
  it**, so every field count here is over the full set. [L]
* **Every read count in this file was re-run against the corrected ten-primitive set**
  (six readers, four thunks including `0x142d23ef0`, which is 6 MB from the cluster) with
  tail `jmp`s counted, over explicit address ranges rather than a single `.pdata` entry:
  `FUN_1402cbb50` **24** read sites, `FUN_142d54780` **8**, `FUN_141d33c70` **7**. All
  three are unchanged from the by-hand reading above, and 24 is exactly the mask table's
  `1 + 2 + 1 + 1 + 1 + 2 + 4 + 4 + 1 + 4 + 1 + 1 + 1`. [L]

---

## 7. What is NOT established

1. **Touch damage.** The mob -> player packet was not found. What is known: `0x00E5` has
   twelve builders sharing one body encoder `FUN_14025d810`, and one of them
   (`FUN_14185ad30`) calls the mob-pool lookup `FUN_141d2efc0` - so `0x00E5` is a
   **candidate family** and nothing more. **[I]**, explicitly a guess. The *server* half is
   independent of the answer: whatever the client sends, the reply that moves the HP bar is
   `0x007C` with bit 10, which §4 settles.
2. **Item drops.** The drop pool is `0x3A0..0x3C5` -> `FUN_1420F8A20`
   (`.pdata 0x1420f8a20..0x1420f929c`), which reads a leading `u32` before its switch. Not
   decoded.
3. **Which of `[t+0x10]` / `[t+0x14]` is the mob object id** in a target block - §1.6.
4. **`FUN_14025d3c0`'s `raw(n)`** inside an attack target, so a target with
   `hasObj != 0` cannot be skipped. The parser reports how far it got instead of guessing.
5. **The meaning of 30 of the 40 header fields.** Fields 8 (`5`), 11 (`4`), 30 (`1`),
   31 (`1`) and 34 (`393`) were non-zero in the one capture and none of them is explained.
6. **Whether `0x00E2`** (the fourth attack opcode) shares this body. Its only builder is in
   group B. Assume not.

---

## 8. The one client run this needs, and what each outcome means

> **SUPERSEDED by §16, and its premise is retracted in §10.** The capture this section
> reasons from came from a session with **no mobs in it**, and that session's log no longer
> exists. The run below is still the right run; the outcome table under it assigns the wrong
> meaning to the 127-byte case, because 127 bytes has never yet been measured against a mob
> the client could see. Read §16 instead.

Everything above is static or from a capture that carried no targets. **One run separates
the two halves of the symptom**, and it needs no server change at all beyond logging:

> Stand next to a snail on map 40 - the capture shows the client at `(473, 395)` and mob
> `2000` at `(424, 395)`, 49 px apart on the same ground line - and **attack it four or
> five times**, then read the `0x00DF` bodies in `world.log`.

| what `world.log` shows | what it means |
|---|---|
| `0x00DF` bodies **longer than 127 bytes** | the client targeted the mob. Bytes 110-113 are the target count; the mob id and the `u64` damages follow, and the server can start applying damage. **Also settles §7.3** - the id will be 2000-2039 |
| every `0x00DF` is exactly **127 bytes** | the client will not target our mobs. No server reply can fix that; the next instrument is the target-collection loop in `FUN_1428c1fa0` (`cmp r12d,[rbp+0x870]` at `0x1428c53a2`), and the leading suspects are the mob's controller (`0x03D2`, never sent) and the 17 `encodeInit` fields we send as zero |
| **no `0x00DF` at all** | the client is not even building an attack. Different problem entirely |

Do it **before** wiring any reply, because the reply set in `combat.rs` is chosen on the
assumption that the client draws its own numbers, and this run is what tests that.

---

## 9. Wire it like this

> **§9.2 is superseded by §15.2**, which adds the health-bar packet and the mob-HP state.
> Everything else in §9 still stands.

`crates/net/src/combat.rs` builds bodies and parses inbound bodies. It touches no session
state and edits neither `crates/world/src/session.rs` nor `crates/net/src/mob.rs`.

### 9.1 Inbound opcodes to dispatch

| opcode | do |
|---|---|
| **`0x00DF`** melee, **`0x00E0`** shoot, **`0x00E1`** magic | `combat::parse_attack(body)`. `combat::is_attack_opcode` is the test |
| `0x00E2` body attack | **log it, answer nothing yet.** Different body; not decoded |

`parse_attack` returns `AttackRequest { attack_type, tick, x, y, targets, target_count,
truncated }`. `targets` is very often **empty** - that is what the one capture contains, and
an empty attack is not an error.

### 9.2 What to send back

```text
for each target in req.targets:
    mob = field.mob(target.object_id)          // 2000-based ids, as mob.rs assigns
    if mob is None: continue                   // stale id, or the wrong field of the two
    mob.hp -= target.total_damage()            // u64, saturating
    if mob.hp == 0:
        send  0x03D1  combat::mob_leave_field(target.object_id, combat::death::ANIMATED)
        send  0x007C  combat::stat_changed(&StatChange::exp_only(new_total_exp))
        // drops: not established, see §7
```

**Nothing else.** No reply carries the damage numbers - §2.

Three rules that come out of the reading and would each cost a session:

1. **`exp` is the character's NEW TOTAL, not the delta.** The field lands in the same
   obfuscated slot `+0x9b` the character record's `exp` lands in.
2. **Do not re-use a mob object id after sending its `0x03D1`.** `research/mob-spawn.md`
   §2 already says a repeat id makes `0x03C6` stop after 31 bytes and desynchronise the
   stream; a respawn must take a fresh id.
3. **Answer even when `parse_attack` errors.** `AttackRequest::truncated` and a parse error
   are both "we did not understand the whole body", and neither is a reason to return
   nothing - `CLAUDE.md`'s always-answer rule. Sending nothing at all is the correct
   *content* here only because it is what the client expects, not because we gave up.

### 9.3 Player HP, when touch damage arrives

The inbound packet is not identified (§7.1). The reply is:

```text
send 0x007C  combat::stat_changed(&StatChange::hp_only(new_hp))
```

and if the player dies, that is a separate packet nobody has looked for.

### 9.4 For the level-up work

`combat::StatChange` carries `level`, `max_hp`, `max_mp`, `ap`, `sp` and `exp`, and
[`combat::stat_changed`] writes them **in ascending bit order in one packet** - which is
what the client's single straight run of bit tests requires. `job_for_sp` must be the job
the client already holds, because that is the job the decoder reads back out of its own
object to choose the SP encoding (`1402cbd99`), *not* any job in the same packet.

The mask is §4.2. `stat::ALL_READ` is every bit the client reads; **bit 3 (`0x8`) is never
read** and setting it would shift every later field.

### 9.5 Two names for `crates/net/src/names.rs`

Not added here, because that file is shared and several agents are in it. The lines:

```rust
0x00DF => "CLIENT_MELEE_ATTACK (u64 damages per target; the CLIENT computes them)",
0x00E0 => "CLIENT_SHOOT_ATTACK (same body as 0x00DF)",
0x00E1 => "CLIENT_MAGIC_ATTACK (same body as 0x00DF)",
0x007C => "STAT_CHANGED (u8 excl, u8 quiet, u8 1, u32 mask, fields in bit order)",
0x03D1 => "MOB_LEAVE_FIELD (u32 objectId, u8 deathType, u8, [u32, u32])",
```

**`0x00DF` must NOT be truncated** - add it to `never_truncate`. Its body is the only
evidence there is about whether the client targets our mobs, and §8's run depends on
reading it whole.

---
---

# Second pass, 2026-08-19 (later): the wall was never measured, and the health bar has a packet

Everything above this line is the first pass. Two of its conclusions change, one of them
completely, and one new packet answers the first half of the owner's request outright.

## 10. RETRACTION: "the client will not target our mobs" rests on nothing

§1 and §8 are built on one sentence:

> *"The owner stood at `(473, 395)`; mob 2000 was at `(424, 395)`, 49 pixels away on the same
> ground line, and the client registered no mob at all."*

**The two halves of that sentence are from two different sessions.** The `(424, 395)` mob is
`world.log:35` of a run made *after* the attack; the attack itself came from a `world.log`
that no longer exists, because `world.log` is gitignored and every run overwrites it. The
tick `0x06eeac37` and the body `...9fae340801040000...` quoted in §1 appear in **no** file in
this repository any more - `grep -rn "37acee06" research/fixtures/` is empty, and so is a
grep for the whole prefix.

What *does* survive, counted rather than remembered: [L]

| log | `0x00DF` bodies | `0x03C6` sent |
|---|---:|---:|
| `research/fixtures/channel-list-shows-two-but-unselectable-world.log` | **9** | **0** |
| `research/fixtures/sweep-0024-01c3-reply-0171.log` | **1** | **0** |
| `research/fixtures/mob-body-faults-client-world.log` | 0 | many |
| `research/fixtures/mob-watch-2b8-null-second-object-world.log` | 0 | 1 |
| `world.log` (newest: 30 mobs on map 40, five minutes of play) | **0** | 30 |

**No log in this repository contains both an attack and a mob spawn.** Every zero-target
attack we hold was swung on a map where the server had sent no mobs at all - the
`channel-list...` session was on map 30 and then map 40 with mobs switched off - so a target
count of zero is the only thing those nine packets *could* have carried.

So:

* **The layout work stands.** All 127 bytes are still accounted for field for field, and the
  parser still consumes the capture exactly. That reading never depended on there being a
  mob.
* **The targeting conclusion does not stand.** There is no measurement of what this client
  does when a mob is in range, and §8's "every `0x00DF` is exactly 127 bytes -> the client
  will not target our mobs" row has never been run.
* **The mechanism that would have explained it did not exist yet either.** `0x03D2`
  MobChangeController was written after that session; the newest run shows 30 mobs handed
  over and 30 `0x02FF` movement reports coming back.

This is the failure mode `STATUS.md` names: *a prediction recorded in the past tense, with no
capture cited beside it.* The defence that would have caught it is one `grep -c` on the file
being cited. `crates/net/src/names.rs` now lists `0x00DF`/`0x00E0`/`0x00E1` in
`never_truncate`, so the next run's bodies survive whole in the log even though they are
named.

## 11. What a mob must satisfy before the client will collect it - `FUN_141d31b20`

§8 named `0x1428c53a2` as the next instrument. That address turns out to be the **bottom** of
a loop that walks targets already collected (stride `0x1d8`, matching `FUN_140f31f60`'s
per-target stride). The collection itself is one call, twenty lines earlier:

```asm
1428c2c2d  call 0x141d31b20                 ; CMobPool::GetMobsInRect-equivalent
1428c2c32  mov  [rbp+0x870], eax            ; <- THE TARGET COUNT
```

`FUN_141d31b20` (`.pdata 0x141d31b20..0x141d32b1c`, 4092 bytes) walks the pool and applies
**eleven filters** in order. Each failure is `jmp 0x141d32a24`, the loop's continue. `rbx` is
the pool node and the mob is `[rbx+8]`; every `call 0x142e52ed0` with `ecx = 0x431` is the
null assertion on it and is noise. All [L].

| # | at | test | mob passes when |
|---:|---|---|---|
| 1 | `141d31cb8` | `FUN_141c543c0(mob)` | **non-zero** |
| 2 | `141d31cde` | `FUN_141c55a80(mob)` = `mob+0x504 != 0` | **zero** |
| 3 | `141d31d10` | arg5 non-null: compare `mob+0x3a0` with arg5's | ids **differ** (exclude-self) |
| 4 | `141d31d4b` | arg6 non-zero: `mob+0x3a0` | **equals arg6** (a single-target filter) |
| 5 | `141d31d7c` | arg8 non-zero: `template+0x60`, the template id | **equals arg8** |
| 6 | `141d31dac` | skill node non-null: `FUN_1407a2eb0(skillNode, templateId)` | **true** |
| 7 | `141d31ddc` | `FUN_141c55b00(mob)` = `template+0x130 > 0` | **`template+0x130 == 0`**, or arg10 non-zero |
| 8 | `141d31e0b` | `FUN_141caeaf0(mob)` = `mob+0x300 == 0x38` | **`mob+0x300 != 0x38`** |
| 9 | `141d31e39` | arg9 zero: `FUN_141c565b0(mob)` | `[mob+0x3c8]+0x1e0 == 0` **and** `template+0x138 == 0` |
| 10 | `141d31e61` | `FUN_141c561a0(mob, 0)` | `[mob+0x3c8]+0x2c0 == 0`, `mob->vtbl[0x58]() == 0`, `template+0x150 == 0` |
| 11 | `141d31e87` | `mob+0xb50 == arg13` and arg12 non-zero | the rect-centre distance is `<= arg12` |

### 11.1 The argument map, because every gate above is conditioned on one

`FUN_141d31b20` builds its frame as `lea rbp,[rsp-0x560]` after **seven** pushes, so
`rbp = R - 0x598` where `R` is the entry `RSP`. Stack argument *N* is at `[R + 8N]`, giving:

| in the listing | argument |
|---|---|
| `[rbp+0x5c0]` (`[rbp-0x78]`) | 5 - a mob to exclude |
| `[rbp+0x5c8]` (`r15d`) | 6 - a single object id to accept |
| `[rbp+0x5d8]` | 8 - a template id to accept |
| `[rbp+0x5e0]` | 9 - suppress filter 9 |
| `[rbp+0x5e8]` | 10 - **accept mobs with `template+0x130 > 0`** |
| `[rbp+0x5f0]`, `[rbp+0x5f8]`, `[rbp+0x600]` | 11, 12, 13 - the filter-11 triple |
| `[rbp+0x608]` | 14 - a skill id, looked up at `141d31b8e` |

### 11.2 What gates 1, 2 and 8 actually are, since those are the ones our packets touch

* **Gate 1 is the "in the field" flag, and both mob packets set it.** `FUN_141c543c0` is
  `FUN_141d11770(mob+0x2d8, mob+0x2e0)` - the client's obfuscated-value getter,
  `rol([mob+0x2dc],5) xor [mob+0x2d8]` with `mob+0x2e0` as the checksum, the same shape
  `research/mob-spawn.md` §2g found on `move_action`. Its setter `FUN_141c543e0` has
  **three call sites in the whole image**: `141d3372c` and `141d338f2` inside
  **`0x03C6` MobEnterField**, both passing **1**, and `141d33dfc` inside **`0x03D1`
  MobLeaveField**, passing **0**. So spawning a mob makes it targetable and killing it makes
  it untargetable, with no packet field involved. [L]
* **Gate 2 (`mob+0x504`) and gate 8 (`mob+0x300`) are both written by `encodeInit`.**
  `tools/fieldrefs.py 0x504 --write` over the mob class gives 13 writers, four of them
  inside `FUN_141c4ff80` (`141c523a9`, `141c52708`, `141c52764`, `141c52b4b`);
  `0x300 --write` gives exactly two, the constructor's `0xffffffff` at `141c4d26e` and
  `141c52b44` inside `encodeInit`. Both live in the `move_action` jump-table region
  (`141c52a95..141c52b7d`), which is the same 16-way switch `research/mob-spawn.md` §2g
  traced. **A different `move_action` therefore lands the mob in a different state**, and
  `mob+0x300 == 0x38` is a state that is not targetable. We send `2`; which state that is
  has **not** been read. [L] for the writers, **[I]** that this could matter.

### 11.3 The honest limits of §11

Three things this does **not** establish, written down so nobody builds on them:

1. **This is not proven to be the melee path.** The call site read here is inside
   `FUN_1428c1fa0`, and the rect it passes comes from `FUN_14079fe90(skill, level) + 0x5e0` -
   **the skill's own attack rectangle**, deobfuscated four fields at a time at
   `1428c2af8..1428c2b39`. That is a *skill* attack. `FUN_1428c1fa0` has two other
   collectors (`FUN_141d2a4f0` at `1428c30b1`, `FUN_141d25360` at `1428c32e9`) writing the
   same count slot, and the other group-A builder `FUN_1428baa50` calls **no** collector at
   all - it receives an already-built list of up to 15 targets from its caller
   (`142ef44fc(&[rbp+0x5c0], 0x1d8, 0xf, ...)` at `1428bb314`). Which builder produced the
   `"User Melee"` capture is **not** established.
2. **`FUN_141d31b20` is a shared primitive, not the attack's own code.** `tools/callers.py`
   gives **86 call sites in 74 functions**. Argument values are per call site; §11.1's
   values are the ones `1428c2c2d` passes and nothing more.
3. **`template+0x130` is unnamed.** It is copied at `1404966c5` from `[rdi+0x4d8]` in
   `FUN_140496180`, and that source offset could not be tied to a WZ property name - a scan
   of the parser `FUN_14047d990` for `[reg+0x4d8]` finds only its own stack frame. A second
   predicate `FUN_141c55b20` tests `template+0x130 == 2`, so it is a small integer rather
   than a flag. At this call site argument 10 is **zero** (`1428c2bfa mov [rsp+0x48], ebx`
   with `ebx = 0`), so gate 7 is live here - but naming the field, and therefore knowing
   whether a snail passes it, is open.

## 12. `0x03F0` MOB_HP_CHANGE - the packet that moves the health bar. [L]

This is the answer to the first half of the owner's request, and it was found by asking a
different question: *what does the client divide to draw a mob's health bar?*

```asm
FUN_141cbb320(mob):                          ; recompute the bar percentage
141cbb37f    ecx = [template + 0x100]
141cbb387    if ecx == 0: return [mob + 0x8b4]           ; raw, no percentage
141cbb395    return (int)( [mob+0x8b4] * 100.0 / (double)ecx )
```

So `mob+0x8b4` is the mob's absolute HP as the client holds it. Who writes it?

```text
python tools/fieldrefs.py 0x8b4 --lo 0x141c40000 --hi 0x141d60000 --write
  141c4eb7e  mov [rsi+0x8b4], eax   in 0x141c4cee0   the mob constructor
  141c83460  mov [rdi+0x8b4], eax   in 0x141c83440   <- and this one takes a CInPacket
```

`FUN_141c83440` has **one** caller, `141d32c03`, inside `FUN_141d32b30` - the second-level
mob dispatcher for `0x03D9..0x044D`. Decoding its 117-entry jump table at `0x141d33448`
(`target = 0x140000000 + dword[table + i*4]`) puts `141d32bfd` at index **23**, so:

> **`0x03D9 + 23` = `0x03F0`.**

### 12.1 The body: nine bytes

```text
u32 objectId      141d32b4d   read by FUN_141d32b30 before the switch
u32 hp            141c83458   -> mob+0x8b4 AND mob+0x8bc
u8  showBar       141c8346c   -> a bool
```

`tools/reads.py 0x141c83440` reports **exactly those two reads and nothing else**, so there
is no gated tail. [L]

### 12.2 What the client does with it

* `mob+0x8b4 = mob+0x8bc = hp`, unconditionally.
* If the template says this mob owns a gauge - `template+0x81 != 0`, `template+0x105 == 0`,
  `template+0x380 == 0`, `template+0x17c == 0`, and a global check at `140479ea0` - it
  **tail-jumps to `FUN_141cd7b40`** (`141c834c2`), which recomputes `mob+0xb60`, sets the
  dirty flag `mob+0xb64 = 1` and calls the mob's own `vtable+0xc0` to redraw. That is the
  health bar moving.
* Otherwise it pushes `hp` into the timed list at `mob+0x6d8` - the floating display over
  the mob - and stamps `mob+0x6c0` with the tick when `template+0xfc != 0` **or** `showBar`
  is set. So `showBar` is "show it now even though this template would not". [L] for the
  branches, **[I]** for the name.

### 12.3 `hp` is absolute, and `template+0x100` is maxHP - derived, not assumed

`0x03C6` also carries an absolute HP, but `encodeInit` converts it to a percentage
immediately using a *different* divisor (`FUN_141c8a730`, the template's `+0x20` qword -
`research/mob-spawn.md` §6.5) and keeps only the percentage. `0x03F0` keeps the absolute
number and divides on demand by `template+0x100`.

That those are the same quantity is **[D]**, from the constructor:

```asm
141c4eb5e  ecx = [template + 0x100]
141c4eb66  je  <default>                  ; zero -> a constant from .rdata
141c4eb7e  [mob + 0x8b4] = (int)ecx       ; current HP := template+0x100
```

A freshly constructed mob therefore reads `hp*100/template[0x100]` = **100%**, which is only
true if `template+0x100` is the maximum. And `world::config::MobTemplate::max_hp` comes from
`tools/dump_mobs.py` reading the WZ's own `maxHP`, so the server and the client are dividing
by the same number. Send the mob's true remaining HP.

## 13. The client never computes a new HP for a mob it hits. [L]

This is the claim that makes §12 *necessary* rather than merely available, so it was made
over the whole of `.text` rather than the mob class:

```text
python tools/fieldrefs.py 0x8b4 --sections .text --write        # 3m06s, 11 hits
```

| hit | in | verdict |
|---|---|---|
| `141c4eb7e` | `0x141c4cee0` | **the mob constructor** |
| `141c83460` | `0x141c83440` | **the `0x03F0` handler** |
| `140887bd0`, `142945a72` | `0x140886810`, `0x1429446b0` | `movsd`, inside a **bulk struct copy** that moves `+0x8a8`, `+0x8b0`, `+0x8b4`, `+0x8bc`, `+0x8c0` in a row - a clone, not a computation |
| the other seven | `0x1408222e0`, `0x14089b830`, `0x1410d9b70`, `0x142b79390`, `0x142b7ff60` (x2), `0x142bb0f80` | no `CMob` signature at all: no `[reg+0x3a8]`, no `0x431` assert, no mob-pool call |

**Nothing anywhere in the client decrements a mob's HP when the player hits it.** It works
out the damage, draws the number, and leaves the bar where it was. The bar is entirely ours
to move.

### 13.1 The instrument had to be repaired before that negative meant anything

`tools/fieldrefs.py --write` **crashed with `NameError: READ_ONLY_DEST` on every
invocation** in the commit this pass started from - the name is referenced in `main()` and
was never defined. A `--write` run was not returning a wrong answer; it was returning no
answer, and any conclusion drawn from "I ran the scan" would have been drawn from a
traceback. The constant is restored (`cmp`, `test`, `push`, `bt`, `jmp`, `call`, the string
compares), and the tool's **own documented positive control** reproduces exactly before any
count above was believed:

```text
python tools/fieldrefs.py 0x2f4 --lo 0x141c40000 --hi 0x141d60000 --write
  141c4d261  in 0x141c4cee0
  141c4e6ee  in 0x141c4cee0
  141cb7ef3  in 0x141cb6880
```

## 14. Touch damage: still not found, and here is what has been eliminated

§7.1 offered `0x00E5` as a candidate family. **It is not the packet**, and the search is
recorded so it is not repeated:

* **`0x00E5` has fifteen builders**, all sharing `FUN_14025d810`. The one that looked
  promising, `FUN_14185ad30`, calls the mob-pool lookup `FUN_141d2efc0` and `FUN_141c54dd0`
  **after** `FUN_1406ed610` - i.e. after the packet is already destroyed. Fifteen builders
  on one opcode is a multiplexed channel, not a hit report.
* **`0x0154` is the only outbound builder in the whole image that looks up a mob while
  building a body.** `grep FUN_141d2efc0 research/msexe-packet-fields.txt` gives four hits;
  the other three are `0x01D1`, `0x02BE` and the `0x00E5` above. Read at `FUN_141001570`:
  its body is `u16 5, u32, u32 0, u32 0xd9, u32` - a **hard-coded `0xd9`** and a literal
  subtype 5. It is a diagnostic about the movement opcode, not a hit. [L]
* **The mob attack-type name table exists** at `0x1432b4510` - `Summon, Melee, Magic,
  Screen, Force Atom Non-target, Body, Summon Attack`, the mob-side twin of the
  `User Normal / User Melee / ...` table §1.1 found. Nothing `lea`s either table
  (`tools/xref.py --va` returns 0 for both bases, and §6 already records that both are
  indexed rather than addressed), so it does not lead anywhere by itself.
* **`FUN_141000fa0` and `FUN_140fffc90`** read `template+0x130` through the same two
  predicates gate 7 uses, and looked like the collision path. They are not: both look up
  **two** mobs and both test the same `0xd9` literal. Same anti-cheat family as `0x0154`.

### 14.1 The cheap way to find it is a client run, not more static analysis

The client is the mob's controller now. In this game family the controller is what decides a
mob's body has touched the player, computes the damage and tells the server. So:

> **Walk into a snail on map 40 and read `world.log`.** A packet that has never been seen
> before, arriving at the moment of contact, *is* the answer - and every inbound opcode is
> already logged with its full body when it is unknown.

The newest run's inbound set is `0x0070, 0x007D, 0x00B8, 0x00DC, 0x00D9, 0x00E7, 0x00ED,
0x00F3, 0x0107, 0x013D, 0x0151, 0x0184, 0x0194, 0x01A5, 0x01BE, 0x01ED, 0x0238, 0x024D,
0x02B2, 0x02DE, 0x02EB, 0x02FF, 0x0408, 0x0420-0x0426`. None of them is combat-shaped
(`0x013D` is a 30-second keepalive, `0x00B8` a 1-byte toggle), so whatever arrives on
contact will be new.

### 14.2 What the server can do meanwhile, which is everything that matters on screen

The server already knows both positions - the client reports mob movement as `0x02FF` and
its own as `0x00D9` - so it can decide a touch itself. The reply that moves the player's HP
bar is `0x007C` with bit 10, which §4 settles completely. `combat::touch_damage` holds the
damage formula in one place; its inputs (`PADamage`, `level`) are **[L]** out of
`gm-handbook/mobtemplates.txt`, and the way they combine is **[I]**.

## 15. Wire it like this - the version that supersedes §9.2

`crates/net/src/combat.rs` still touches no session state.

### 15.1 Inbound

| opcode | do |
|---|---|
| `0x00DF` / `0x00E0` / `0x00E1` | `combat::parse_attack(body)`; `combat::is_attack_opcode` is the test |
| `0x00E2` | log it, answer nothing - different body, not decoded |
| anything new that arrives when the player walks into a mob | **log the whole body and tell the owner** - that is §14's run |

### 15.2 Server-side state: one `u64` per live mob

`net::mob::FieldMob` already carries `hp`. Seed it from
`world::config::MobTemplate::max_hp` - the same number `0x03C6` sends and the same number
the client divides by - and keep it as the authority.

```rust
for target in &req.targets {
    let Some(mob) = field.mob_mut(target.object_id) else { continue };   // stale id
    let hit = combat::apply_damage(mob.hp, target.total_damage());
    mob.hp = hit.hp_after;
    for (opcode, body) in combat::mob_hit_replies(target.object_id, &hit) {
        send(opcode, body);          // 0x03F0 while alive, 0x03D1 on the kill
    }
    if hit.died {
        exp_gained += template.exp;  // gm-handbook/mobtemplates.txt column 4
        // and free the object id: a respawn must take a fresh one
    }
}
if exp_gained > 0 {
    character.exp += exp_gained;
    send(0x007C, combat::stat_changed(&combat::StatChange::exp_only(character.exp)));
}
```

Five rules, each of which would otherwise cost a session:

1. **`0x03F0` while alive, `0x03D1` on the kill, and never both.** `mob_hit_replies`
   enforces it. A bar update for an object the client is tearing down writes into a pool
   entry that is being removed.
2. **`exp` is the character's NEW TOTAL, not the delta** - it lands in the same obfuscated
   slot `+0x9b` the character record's `exp` does.
3. **One `0x007C` per swing, not per mob.** EXP belongs to the player; a five-mob swing is
   one packet.
4. **Do not re-use a mob object id after its `0x03D1`.** `research/mob-spawn.md` §2 - a
   repeat id makes `0x03C6` stop after 31 bytes and desynchronises the stream.
5. **Answer even when `parse_attack` errors or truncates.** `CLAUDE.md`'s always-answer
   rule. For an attack the correct *content* is often nothing at all, but that is a decision,
   not a give-up.

### 15.3 A mob hurts the player

```rust
let damage = combat::touch_damage(template.pa_damage, template.level, character.level);
character.hp = combat::player_hp_after(character.hp, damage);
send(0x007C, combat::stat_changed(&combat::StatChange::hp_only(character.hp)));
```

`MobTemplate` does not carry `pa_damage` yet; the column is already in
`gm-handbook/mobtemplates.txt` (index 5) and `world::config::load_mob_templates` reads only
the first five. Adding it is four lines in `crates/world/src/config.rs`.

**There is no death packet.** Nothing has looked for one, so a character reaching 0 HP will
sit at 0 with an empty bar and no death screen. Clamp to 1 until somebody finds it, and say
so in the log rather than silently.

## 16. The client run this needs, and what each outcome means

One run, and it now tests three things instead of one. **Mobs are on by default; map 40 has
snails; `-SetFieldProbe` is still required.**

| # | do | look at | what it means |
|---:|---|---|---|
| 1 | Stand next to a snail and swing **five or six times** | `world.log`, the `0x00DF` bodies (now logged whole) | **Longer than 127 bytes** - the client targets our mobs, the wall never existed, and bytes 110-113 are the count. **Exactly 127 every time** - now it *is* a measurement, and §11 is the instrument: gate 7 and `move_action`'s effect on `mob+0x300` are the two leads |
| 2 | If any swing had a target, watch the **snail's HP bar** | the bar over the mob | The server is sending `0x03F0`. **Bar moves** - §12 is right end to end. **Bar still full after damage** - `template+0x100` is not maxHP after all, and §12.3's derivation is wrong. **No bar at all** - this template does not own a gauge; look for the floating number instead |
| 3 | Keep swinging until one dies | the mob | **Death animation and it leaves** - `0x03D1` with `deathType 1` works. **It vanishes with no animation** - wrong death type. **Client fault** - `0x03D1`'s two trailing `u32`s are not free after all |
| 4 | Watch the **EXP bar** | the bar | `0x007C` bit 16 with the new total |
| 5 | **Walk into a snail and stand there** | `world.log`'s inbound lines | **A new opcode** - that is the touch-damage packet, §14. **Nothing new at all** - the client does not report it, and the server must detect the touch itself from `0x02FF` + `0x00D9` |

Steps 1-4 are one chain sharing one observable, so they do not violate one-variant-at-a-time;
step 5 is a separate subsystem with a separate observable and can be done in the same
session.

## 17. Instrument notes from this pass

* **A gitignored log is not a citation.** `world.log` is overwritten by every run. Anything
  quoted from it must be copied into `research/fixtures/` in the same sitting, or the claim
  becomes unfalsifiable within a day. §10 is what that costs.
* **`tools/fieldrefs.py --write` was broken and said so loudly** - a `NameError`, not a
  wrong number. That is the *good* kind of broken; the dangerous kind is §13.1's opposite.
  Run the tool's documented positive control anyway.
* **A `.pdata`-bounded disassembly needs an instruction-aligned start.** Disassembling from
  an arbitrary mid-function address produces plausible-looking garbage - `0x141c52ab0`
  decodes as `push rsi / dec byte ptr [rax-0x75] / popfq` and none of it is real. Always
  start from the function head and index into the listing.
* **A jump table is worth decoding rather than reading one arm of.** The 117-entry table at
  `0x141d33448` turned "some handler in a range nobody has looked at" into an exact opcode
  in one script: `target = 0x140000000 + dword[table + i*4]`, `opcode = 0x3D9 + i`.
* **Ask what the client *divides*, not what it *stores*.** `0x03F0` was found from the
  health bar's arithmetic, after a direct search for a damage packet had already produced
  one wrong answer (§2.1's `0x03D6`).
