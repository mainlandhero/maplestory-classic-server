# Mob spawn: `0x03C6`, `MobEnterField`, and the 137-byte body

Started 2026-08-19 after the owner made mobs and drops the priority: *"there should be tutorial
monsters spawning on East Entrance to Mushroom Town (ID 30)"*. Finished the same day.

Markers: **[L]** read out of the listing or the image, **[D]** derived from two or more [L],
**[I]** inferred - including anything from the v214 reference at
`C:\Users\user\Desktop\ModernMapleSource`, which is a **different game version** and scored
**1 of 8** against a held-out control. Every [I] is a candidate, never a fact.

---

## 0. Answer up front

| | |
|---|---|
| opcode | **`0x03C6`** [L] |
| smallest legal body | **137 bytes** [D] |
| made of | 11-byte head + 20-byte stat mask + 106-byte `encodeInit` [D] |
| the previously-blocking question | **settled**: `FUN_14046fba0` is not a movement-path decoder, it is a **mask-gated temporary-stat decoder, and an all-zero 20-byte mask costs zero bytes** [L] |
| what the last pass missed | **two more decoders**, both of which read the packet: `FUN_141cc9410` -> `FUN_14085acd0` (57 bytes, gated by a byte we control) and the **virtual** `vtable+0x38` = `FUN_141c4ff80` (52 reads, 106 bytes minimum) - which is where position, foothold and HP actually live [L] |
| implemented in | `crates/net/src/mob.rs` |

**It has now been sent to a client, and it killed it** - `0xC0000005 at 0x141c810b0` on the
**first** `0x03C6`. Section 11 is that run and what the crash actually proves. The short
version: **the 137-byte body is not the bug.** The field the client dereferenced,
`mob+0x2b8`, is a COM interface pointer the client obtains from *itself*; no byte of the
packet can set it or clear it. Sections 1-7 below were written before that run and stand.

---

## 1. Routing - settled, unchanged

| | |
|---|---|
| mob pool range | `0x3C6..0x44E`, 137 opcodes, singleton at `[0x143ABFE00]` |
| dispatcher | `FUN_141D30E80` |
| **enter field** | **`case 0x3c6` -> `FUN_141d33630`** (1582 bytes) |

`research/msexe-mobpool.c` line 26-28 is literally `case 0x3c6: FUN_141d33630(param_1,param_3);
break;` [L]. The exact analogue of the NPC pool's `0x44F` -> `FUN_141e36b20`.

Mobs are **server-sent** for the same reason NPCs are: the client's field loader walks the WZ
`life` node only to preload `Mob/%07d.img` (`research/npc-spawn.md` §2).

Game data is generated: `gm-handbook/mobs.txt`, 9928 spawns across 289 maps.

---

## 2. The four decoders, and the shape of the whole body

`FUN_141d33630`, bounded to `[0x141d33630, 0x141d33c5e)` by `.pdata`, contains **exactly 7
packet reads** (4 x `u8`, 3 x `u32`). *The first scan of this ran 4608 bytes past the function
end and reported 34; bound every dump by `.pdata`.* [L]

```
FUN_141d33630                                          11 bytes on the normal path
  |  u8   sealed?             141d3365a
  |  u32  objectId            141d3368e      (u32 again at 141d336a1 iff objectId == 0)
  |  u8   calcDamageIndex?    141d336f3
  |  u32  templateId          141d33701  -> FUN_140495990, the Mob/%07d.img loader
  |  FUN_141d2efc0(pool, objectId)    -- is this id already in the pool?
  |
  +-- FOUND (141d33734):  u8 gate, then FUN_141c76190, then JMP 141d339e8   <- NO encodeInit
  |
  +-- NEW   (141d338fa):  u8 gate, then FUN_141c76190, then the VIRTUAL     <- the real path
        |
        |  u8 gate  141d338fa   != 0 -> FUN_141cc9410 -> FUN_14085acd0      57 bytes
        |  FUN_141c76190                                                    20 bytes + stats
        |     |  raw[20]  141c76276      the temporary-stat presence mask
        |     +- FUN_14046fba0(stats, vec, &mask, packet, now)   0 bytes if mask == 0
        |  MOV RAX,[RDI]; CALL [RAX+0x38]   141d33929   = FUN_141c4ff80    106 bytes min
```

**The body is not self-describing.** Whether `encodeInit` is read at all depends on whether
the client already has that object id in its pool. The pool is destroyed and rebuilt on every
field entry (same mechanism as the NPC pool, `research/npc-spawn.md` §6.1), so on a fresh
field every spawn takes the NEW branch. **Re-sending a live object id would make the client
stop after 31 bytes and desynchronise the rest of the stream.** [L]

### 2.1 The two value rules, re-read and still true

* **`objectId` must be non-zero.** Zero pulls in a second `u32` at `141d336a1`. [L]
* **Avoid an `objectId` that is a multiple of 178 (`0xb2`).** Both branches test
  `uVar6 == (uVar6 / 0xb2) * 0xb2`; on the non-zero path a multiple falls into a call through
  `(*local_48)[2]`, slot 2 of a stack functor whose vtable is `PTR_LAB_143409208`. It reads no
  packet bytes, but it is unexplored and free to avoid. [L]

---

## 3. `FUN_14046fba0` - the question that was blocking, answered

**It is not a movement-path decoder.** The previous note read "330 packet reads in repeating
`u32,u32,u16` triples" and inferred a switch over movement actions. The triples are real; the
inference was wrong. The function is `MobStat::DecodeTemporary`: **a flat list of optional
fields, each gated by one bit of the 20-byte mask.** [L]

`.pdata`: `0x14046fba0 .. 0x1404724c0`, 10528 bytes. Full listing already on disk as
`research/msexe-mobmove.txt` (the name is now wrong; the file is right).

### 3.1 The mask is the raw[20], and it is 160 bits

`FUN_141c76190` at `141c7625b`: [L]

```asm
141c7625b  XORPS  XMM0,XMM0
141c7625e  MOVUPS xmmword ptr [RBP + -0x1],XMM0     ; 16 zero bytes
141c76262  MOV    dword ptr [RBP + 0xf],0x0         ; + 4 = 20
141c76269  MOV    R8D,0x14                          ; n = 20
141c7626f  LEA    RDX,[RBP + -0x1]
141c76273  MOV    RCX,RSI                           ; the packet
141c76276  CALL   0x1406e9170                       ; raw(20)
...
141c7629a  CALL   0x1429e3ef0                       ; now
141c7629f  MOV    [RSP + 0x20],EAX                  ; arg5 = now
141c762a3  MOV    R9,RSI                            ; arg4 = the packet
141c762a6  LEA    R8,[RBP + -0x1]                   ; arg3 = the 20 bytes just read
141c762aa  LEA    RDX,[RBP + 0x17]                  ; arg2 = a vector
141c762ae  MOV    RCX,RDI                           ; arg1 = mob+0x3c8, the stat block
141c762b1  CALL   0x14046fba0
```

Inside `FUN_14046fba0`, `MOV R15,R8` at `14046fbbf` is the **only** write to R15 in the whole
10528 bytes (`POP R15` at the epilogue aside), so R15 is the mask for the entire function. [L]
Its first loop bounds the bit index at `CMP EDI,0xa0` - **160 bits = the 20 bytes** - and tests
`[R15 + (i>>5)*4]` bit `31 - (i & 31)`. [L]

### 3.2 Every read in it is gated, and a zero mask costs zero bytes

Enumerated rather than filtered. Every direct call target in the bounded range:

| target | n | what |
|---|---|---|
| `0x1406e8c20` | 233 | u32 |
| `0x1406e8b80` | 84 | u16 |
| `0x1401b9b60` | 83 | vector push (no packet) |
| `0x1406e8ae0` | 9 | u8 |
| `0x1406e8f10` | 3 | u64 |
| `0x1406e9050` | 1 | string |
| `0x1406e8f00` | 1 | u32 thunk |
| others | 7 | none take the packet |

**331 reads.** A guard-interval scan over the listing puts **328 of them inside a
`[R15…]`-gated block**; the other 3 (`140471db3`, `140471dc1`, `140471dd4`) sit behind
`BT EAX,0x1a / JC ... / BT EAX,0x19 / JNC skip` - an **OR of two mask bits**, which the
interval scan could not see because it only looked at the first conditional jump after the
test. Read by hand at `140471da4`. So: **331 of 331.** [L]

The mask-test forms, all of which had to be modelled (this is the "wrong shape" trap again):

```
TEST dword ptr [R15 + k],imm        TEST byte ptr [R15 + k],imm8
MOV EAX,[R15 + k] / AND EAX,imm     CMP dword ptr [R15 + k],0x0 / JGE
MOV EAX,[R15 + k] / BT EAX,n / JNC  ... / BT EAX,m / JNC        (the OR form)
```

The only other callee that receives the packet is `FUN_14036dec0(&arr[i], packet)` at
`14046fc46`, inside the reset loop, reached only when bit *i* is set. [L]

> **An all-zero 20-byte mask makes `FUN_14046fba0` read nothing and call nothing that could.**
> That is the whole answer to the question this file stopped on. [L]

Shape note, [D]: each gated block is `u32 nOption, u32 rOption, u16 duration`, and the
duration is `MOVSX; IMUL 0x1f4; ADD now` - **500 ms units added to the current tick**. 83 of
the 85 blocks have exactly that shape.

---

## 4. What the last pass missed: `FUN_141cc9410` reads 57 bytes

`research/mob-spawn.md` previously said `FUN_141cc9410` "reads NOTHING". That was a scan
bounded to the function itself. It calls `FUN_14085acd0(*(mob+0xa20), packet)` - **with the
packet** - and that function reads, unconditionally: [L]

```
14085ace3  u64 -> [dst+0x08]        14085ad31  u32 -> [dst+0x2c]
14085acef  u32 -> [dst+0x10]        14085ad47  u32 -> [dst+0x34]
14085acfa  u32 -> [dst+0x14]        14085ad52  u32 -> [dst+0x38]
14085ad05  u32 -> [dst+0x18]        14085ad5d  u32 -> [dst+0x30]
14085ad10  u32 -> [dst+0x1c]        14085ad68  u32 -> [dst+0x48]
14085ad1b  u32 -> [dst+0x20]        14085ad73  u8  -> [dst+0x04]
14085ad26  u32 -> [dst+0x28]
```

`8 + 12*4 + 1 = ` **57 bytes**, gated by the `u8` at `141d33734` / `141d338fa`. Send **0** and
none of it is read. [L]

`[dst+0x08]` is consumed by `FUN_141c8a730` as the mob's **max HP**, overriding the template's
(`*(mob+0xa20) + 8`). [L] The v214 reference calls this block `ForcedMobStat` and encodes
`maxHP(long), maxMP, exp(long), pad, mad, pdr, mdr, acc, eva, pushed, speed, level, userCount,
byte` - 14 fields against this client's 13, so the shape matches and the contents do not. [I]

---

## 5. The virtual `vtable+0x38` - where position, foothold and HP live

At `141d33929`, on the NEW-mob branch only: [L]

```asm
141d3391d  MOV RAX,qword ptr [RDI]      ; the mob's primary vtable
141d33920  MOV R8,R14                   ; the packet
141d33923  MOV EDX,R12D                 ; objectId
141d33926  MOV RCX,RDI
141d33929  CALL qword ptr [RAX + 0x38]  ; slot 7
```

`FUN_141d3a540(template)` is the mob factory: it picks one of **eight** concrete classes by
template id and template flags, allocating 0x1238 / 0x1270 / 0x1290 / 0x1298 / 0x12c0 / 0x12d0
/ 0x12d8 bytes. Each constructor's primary vtable was read from its own `MOV [obj],RAX`, and
**slot 7 of all eight is the same address, `0x141c4ff80`**: [L]

| ctor | alloc | vtable | slot 7 |
|---|---|---|---|
| `FUN_141c4cee0` (base) | 0x1238 | `0x1434077e8` | `0x141c4ff80` |
| `FUN_141fdb720` | 0x1270 | `0x14341f548` | `0x141c4ff80` |
| `FUN_141fcf850` | 0x1290 | `0x14341e510` | `0x141c4ff80` |
| `FUN_141fd4010` | 0x1298 | `0x14341e758` | `0x141c4ff80` |
| `FUN_140ff7fa0` | 0x1298 | `0x1433765c0` | `0x141c4ff80` |
| `FUN_141fd4a00` | 0x12c0 | `0x14341ede0` | `0x141c4ff80` |
| `FUN_140ff87c0` | 0x12d0 | `0x1433767f0` | `0x141c4ff80` |
| `FUN_141fd6dc0` | 0x12d8 | `0x14341eff0` | `0x141c4ff80` |

This is **not** the `/OPT:ICF` trap `docs/ghidra.md` warns about: the vtables were reached
through the constructors that install them, not by aligning tables that happen to share an
entry. No mob class overrides the body decoder. [D]

> `tools/rtti.py --vtable` reports **0 complete-object locators for every class**, including
> the control `CLoginQueueDlg`. Its type-descriptor scan works; the locator half does not
> resolve on this binary. The RVA of `.?AVCMob@@` appears **nowhere** in the file as a dword.
> Do not read a zero from that tool as evidence. [L]

### 5.1 Read count, cross-checked

`.pdata`: `0x141c4ff80 .. 0x141c54054`, 16596 bytes.

* listing, bounded, counting the seven known primitives: **50**
* decompiler: **52**

They disagreed, so the instrument was wrong - and it was. Sweeping **all 116 distinct direct
call targets** and reading the first bytes of each found `0x1406e8ef0` = `E9 8B FC FF FF` =
`JMP 0x1406e8b80`: **an eighth read primitive, a u16 thunk, called twice.** With it included
both instruments say **52** (9 u8, 6 u16, 34 u32, 1 u64, 1 str, 1 raw). [L]

> **`docs/ghidra.md`'s table of seven primitives is short by one.** Add
> `0x1406e8ef0` -> u16 (bare `JMP 0x1406e8b80`). This is the second thunk this project has
> been bitten by and the exact failure the "wrong set" warning describes.

### 5.2 The minimum path: 106 bytes

Built as a shortest-path problem over the function's CFG - each read costs its width,
everything else costs zero, from the entry to the single reachable `RET` at `0x141c53dca`.
The answer is **106 bytes over 35 reads**, and it agrees read-for-read with a hand trace of
the listing. Three indirect `JMP RCX` exist (`141c50de1`, `141c51a97`, `141c52adc`) and were
not followed; they are off the minimum path. [L]

---

## 6. The body, byte for byte

All offsets are into the **body** (after the 2-byte opcode). Every row's `[L]` address is
where the client reads it.

### 6.1 Head - `FUN_141d33630`, 11 bytes

| off | size | read at | goes to | name | mark |
|---|---|---|---|---|---|
| 0 | u8 | `141d3365a` | `FUN_141c55ab0` -> `mob+0x508` | `sealedInsteadDead` | [L] read, [I] name |
| 1 | u32 | `141d3368e` | pool key, `mob+0x3a0` | **objectId** | [L] |
| 5 | u8 | `141d336f3` | `mob+0x620` | `calcDamageIndex` | [L] read, [I] name |
| 6 | u32 | `141d33701` | `FUN_140495990` -> `Mob/%07d.img` | **templateId** | [L] |
| 10 | u8 | `141d33734`/`141d338fa` | gate | **forced-stat present** - non-zero costs 57 more bytes | [L] |

### 6.2 Temporary stats - `FUN_141c76190`, 20 bytes

| off | size | read at | name | mark |
|---|---|---|---|---|
| 11 | raw[20] | `141c76276` | **temporary-stat mask**, 160 bits. All zero = no stat fields follow | [L] |

### 6.3 `encodeInit` - the virtual `FUN_141c4ff80`, 106 bytes

| off | size | read at | goes to | name | mark |
|---|---|---|---|---|---|
| 31 | i16 | `141c4ffc5` | obf container `mob+0x860` -> `mob+0xd30` low | **x** | [D] |
| 33 | i16 | `141c501b0` | obf container `mob+0x848` -> `mob+0xd30` high | **y** | [D] |
| 35 | u8 | `141c503f2` | XOR-obfuscated into `mob+0x3e0` | `moveAction` | [L] read, [I] name |
| — | u8 | `141c5043d` | **only** if templateId is 8909488, 8909588 or 9990545 | — | [L] |
| 36 | i16 | `141c50456` | `FUN_142df6c50(DAT_143ac18d8, v)` - the field's foothold map | **fh** | [D] |
| 38 | i16 | `141c50465` | the same foothold map | **homeFh** | [D] |
| 40 | u8 | `141c50474` | `mob+0x116c` | unknown | [L] |
| 41 | i8 | `141c50485` | `mob+0x1168` | **appearType** | [D] |
| — | u32 | `141c504d0` | **only** if appearType is `-3`, `-6` or `>= 0` | `option` | [L] |
| 42 | i16 | `141c50499` | `mob+0x1228` | unknown | [L] |
| 44 | i16 | `141c504aa` | `mob+0x122c` | unknown | [L] |
| 46 | u32 | `141c504db` | `mob+0x8c0` | **hp scale %** - `FUN_141c8a730` multiplies max HP by it unless it is 0 or 100 | [D] |
| 50 | u64 | `141c504e9` | `(hp*100)/maxHp` -> `mob+0xb60` | **current HP** | [D] |
| 58 | u32 | `141c5050e` | a local | `effectItemId` | [L] read, [I] name |
| — | 4x u32 | `141c5052c`.. | **only** if `template[0x104] != 0` | patrol range | [L] |
| 62 | u32 | `141c5056f` | virtual `[*+0x110](mob, old, new)` | unknown | [L] |
| 66 | u32 | `141c505a8` | `mob+0xb6c` | unknown | [L] |
| 70 | u32 | `141c50621` | `mob+0xb54`; **non-zero loads another `Mob/%07d.img`** | `refImgMobId` | [D] |
| 74 | i32 | `141c5066f` | `FUN_141cdf1e0(mob, v-13, …)` if `v >= 0 && next > 0` | unknown | [L] |
| 78 | i32 | `141c50680` | the `> 0` half of that gate | unknown | [L] |
| 82 | u32 | `141c5068a` | argument to the same | unknown | [L] |
| 86 | u8 | `141c50694` | argument to the same | unknown | [L] |
| 87 | u32 | `141c506bd` | **count**, then `count` x (u32, u32) | a timed-id map | [L] |
| 91 | u32 | `141c507bb` | `mob+0xd64` | unknown | [L] |
| 95 | u8 | `141c507c9` | gate; non-zero costs u32 + u32 | unknown | [L] |
| 96 | u32 | `141c50808` | **count**, then `count` x (u32 nType, u32 objectId **looked up in the mob pool**) | attached mobs | [D] |
| 100 | u8 | `141c509d2` | gate; non-zero costs **raw[120]** | unknown | [L] |
| 101 | str | `141c50a66` | `mob+0xf78` | a name/script string | [L] |
| — | u32 | `141c50ac0` | **only** if `template[0x1a0] != 0` | target from server | [L] |
| 103 | u32 | `141c50ada` | **count**, then `count` x u32 | unknown | [L] |
| 107 | u8 | `141c532ab` | gate (no reads inside) | unknown | [L] |
| 108 | u8 | `141c534d9` | `mob+0xfdc` | unknown | [L] |
| 109 | u32 | `141c534ea` | `mob+0xfe0` | unknown | [L] |
| 113 | u32 | `141c534f8` | `mob+0xfe4`; **non-zero costs a further u32** | unknown | [L] |
| 117 | u32 | `141c53518` | gate (no reads inside) | unknown | [L] |
| 121 | u32 | `141c535ae` | a local | unknown | [L] |
| 125 | u32 | `141c535b8` | `mob+0x1118` | unknown | [L] |
| 129 | u32 | `141c53695` | **count**, then `count` x u32 | unknown | [L] |
| 133 | u32 | `141c536df` | a local | unknown | [L] |
| 137 | | | | end | |

### 6.4 How x and y were pinned, since it is the field a swap would ruin

`141c4ffca MOVSX R8D,AX` - both are **sign-extended i16**. [L] Each goes into one of the
client's obfuscated-value containers (`mob+0x860` for the first, `mob+0x848` for the second),
then `FUN_1409d3c60(mob+0x818, mob+0x848)` copies **both** fields of a two-member point -
the decompiled body calls `FUN_1401b0340(src+0x18)` -> `dst+0x18` and `FUN_1401b0340(src)` ->
`dst` - so `0x830 <- 0x860` (first read) and `0x818 <- 0x848` (second). `FUN_14019a5d0` reads
its container at base+0, so `FUN_14019a5d0(mob+0x830)` returns the **first** read and lands in
the **low** dword of `mob+0xd30`, and the second read lands in the high dword. A `POINT` is
`{int x; int y;}`, so **first = x, second = y.** [D]

### 6.5 The `IDIV` with no guard

```asm
141c504e9  CALL 0x1406e8f10          ; u64 hp   -> RBX
141c504f4  CALL 0x141c8a730          ; max HP   -> RCX
141c504fc  IMUL RAX,RBX,0x64
141c50500  CQO
141c50502  IDIV RCX                  ; <-- no test of RCX
141c50505  MOV  [RSI + 0xb60],EAX
```

`FUN_141c8a730` returns the template's `+0x20` qword (via `FUN_14047a100`) unless overridden.
**If it is zero the client takes a #DE and dies.** Send a template id that exists. [L]

---

## 7. Corroboration from v214 - shape yes, numbers no

`MobPool.enterField` in the reference encodes, in order: [I]

```java
encodeByte(mob.isSealedInsteadDead());      // == head byte 0
encodeInt(mob.getObjectId());               // == head 1..4
encodeByte(mob.getCalcDamageIndex());       // == head 5
encodeInt(mob.getTemplateId());             // == head 6..9
encodeByte(fms != null);                    // == head 10, the 57-byte gate
if (fms != null) fms.encode(outPacket);
MobTemporaryStat.encode(...);               // == the 20-byte mask
mob.encodeInit(outPacket);                  // == the virtual
```

**Five for five on the head, and the three blocks after it in the right order.** Inside
`encodeInit` the reference also matches on: position first; `moveAction`; the *three special
template ids* that add a byte; `curFoothold` then `homeFoothold`; `appearType` with the
identical and unusual condition `-3 || -6 || >= 0`; `scale` immediately before
`encodeLong(getHp())`; `isPatrolMob()` gating exactly four ints; a byte gating exactly
**120** raw bytes; an empty string; and a count-then-`(nType, objectId)` loop whose ids it
looks up among the field's mobs.

**And it is wrong about the numbers, in the way this project has learned to expect.** The
three special template ids are `8910000, 8910100, 9990033` in the reference; in mscw
`FUN_14045b1a0` compares against `0x87f4b0, 0x87f514, 0x986f91` = **8909488, 8909588,
9990545**. Same structure, different values. Use it for names, never for bytes.

---

## 8. What is NOT settled

1. ~~**The three template-driven optional blocks.**~~ **SETTLED 2026-08-19, section 11.2.**
   `template[0x104]` is the WZ node **`patrol`**; `template[0x1a0]` is the WZ property
   **`targetFromSvr`**. **No mob image in this client sets either** - all 193 of them were
   dumped and checked. Both blocks are absent for every mob in the game, so 137 bytes is
   right and this is *not* why the first attempt failed. [L]
2. **Meanings for 17 of the 35 `encodeInit` fields.** They are sent as zero. That is the same
   "no readable consumer" argument `research/setfield-zero-audit.md` was written about, and it
   is weaker than a measurement.
3. **Whether `appearType` should be `-1` or `-2`.** Both skip the extra `u32`. The reference
   annotates `// init -> -2, -1 else`. Not settled; one byte to change.
4. **Whether a mob renders without a controller packet.** `0x3D2` is the pool's
   change-controller opcode (`FUN_141d30e80` case `0x3d2`, `u8 flag; u32 objectId;` then either
   a removal or `FUN_141d34a70(pool, flag, id, u8, packet)`). Not decoded, not needed to
   create the object. [L]
5. ~~**Nothing has been sent to a client.**~~ It has - see section 11. What is still open is
   **why the client called `FUN_141c81040` on a mob whose `+0x2b8` had not been filled yet**,
   which section 11.5 turns into two watches.

---

## 9. The files this rests on

| file | what |
|---|---|
| `msexe-mobpool.c` | `FUN_141d30e80`, the pool dispatcher - `case 0x3c6` is on line 26 |
| `msexe-mobspawn.txt` / `.c` | `FUN_141d33630`. **The listing runs to `141d3482f`, 4608 bytes past the function; filter to `141d33c5e`** |
| `msexe-mobinit-b.txt`, `msexe-mobinit.c` | `FUN_141c76190`, the raw[20] and the call into the stat decoder |
| `msexe-mobmove.txt` | `FUN_14046fba0`, the whole 10528 bytes. The name is a leftover from when it was believed to be a movement-path decoder |
| `msexe-mobforcedstat.txt` | `FUN_14085acd0`, the 57-byte forced-stat block |
| `msexe-mobfactory.txt`, `msexe-mobctors.txt` | `FUN_141d3a540` and the eight constructors, with the resolved vtable slot 7 |
| `msexe-mobinit-virtual.txt` / `.c` | **`FUN_141c4ff80`**, the 106-byte `encodeInit` |
| `msexe-mobtpl-gate.txt` | `FUN_14045b1a0`, the three special template ids |
| `msexe-mobtemplate.c`, `msexe-mobflag-consumers.c` | `FUN_140495990`, `FUN_141c8a730`, `FUN_14047a100`, `FUN_141c55ab0` |
| `msexe-obfpoint.c` / `.txt` | `FUN_1409d3c60` and `FUN_14019a5d0` - how x and y were pinned |
| `msexe-mobspawn2.c` | `FUN_14085acd0`, `FUN_141d3a540`, `FUN_141d2efc0` and the pool helpers |

**Section 11 used no Ghidra at all** - another agent held the project lock. It is capstone
against `client-patched\MapleStory.exe` plus `.pdata` for bounds, `tools/xref.py --string`,
and `target/release/wz-dump`. `research/npc-click.md` and `research/channel-select.md` are the
other analyses done that way. Nothing in section 11 needs the decompiler to be re-checked.

---

## 10. Instrument notes worth keeping

* **`docs/ghidra.md`'s primitive table is short by one:** `0x1406e8ef0` is a bare
  `JMP 0x1406e8b80` (u16). Counting only the seven listed undercounts by 2 in this function.
* **A guard-interval scan that only looks at the first conditional jump after a mask test
  misses the OR form** (`BT / JC take / BT / JNC skip`). It reported 3 unguarded reads where
  there are none.
* **`tools/rtti.py --vtable` returns "0 locator(s)" for every class in this binary**,
  including a control that certainly has a vtable. Its `--list` half works.
* **Bounding a call census to a function is not bounding it to a code path.**
  `FUN_141cc9410` contains no read primitive and reads 57 bytes; the reads are one call deeper.
  Chase every callee that receives the `CInPacket *`, and only those.
* **`.pdata` does not cover every function.** The template accessors `FUN_140479e60` and
  `FUN_140479e70` are `MOVZX EAX,byte [RCX+d]; RET` with no unwind data and therefore **no
  `.pdata` entry**, and so does about a third of every mob vtable. A vtable walk that stops
  at the first entry which is not a `.pdata` function start truncates the base mob vtable at
  **slot 1**. Walk while the qword lands in an executable section instead.
* **A linear capstone sweep stops silently at the first undecodable byte.** `.text` needed
  20 034 resyncs and `.boot` 646 905; without a resync everything after the first bad byte in
  a section is never examined and the scan reports a confident zero. And resync by re-slicing
  `bytes` is `O(n^2)` - it never finished on `.boot`. Slice a `memoryview`.

---

## 11. The first client run - 2026-08-19 - and what the crash proves

Capture: `research/fixtures/mob-body-faults-client-{world,hook,exit}.log`. Map 40,
character 204, 40 mobs of template 2 sent in one burst after two `0x044F` NPCs.

```
0xC0000005 at 0x141c810b0
```

### 11.1 It faulted inside the FIRST `0x03C6`, not half a second later [L]

`crates/grap-stub/src/hook.rs:285` writes the `N opcode=0x…… elapsed_us=` line **after** the
trampoline returns. The fixture's last such line is `9 opcode=0x044F`; there is **no line for
any `0x03C6`**. So the first mob dispatch never returned - the fault is inside
`FUN_141d33630`'s call tree, synchronously. (An earlier reading of this fixture called it a
per-tick or per-render virtual arriving ~0.5 s later. That was wrong: the two logs are stamped
in different zones - `world.log` says `18:27` where `maplecw-hook.log` says `14:27`, a whole
four hours - and the sub-second fields agree, 52.929 sent to 52.952 faulted, **23 ms**.)

It is also the **first** mob, not the 40th: ten dispatches completed (`0x0032` x3, `0x0000`,
`0x000B` x2, `0x0010`, `0x0011`, `0x044F` x2) and the next packet in the stream is mob #1.
**`-MobLimit 1` will reproduce it.** Volume is not the variable.

### 11.2 `template[0x104]` is `patrol`, `template[0x1a0]` is `targetFromSvr` - both absent [L]

The parser is `FUN_14047d990` (44 762 bytes; found by `tools/xref.py --string bodyAttack`).

```asm
140483bea  LEA  RDX,[rip -> u"patrol"]        ; 0x14328b748, utf-16
140483bf8  CALL 0x1401a5890                   ; wrap it as a name string
140483c0a  CALL 0x1401e4330                   ; node->GetItem(name)  -> [rbp+0xc8]
140483c56  CMP  qword ptr [RBP + 0xc8],0
140483c5e  JE   140483d23                     ; absent
140483c64  MOV  byte ptr [RAX + 0x104],1      ; present
140483d23  MOV  byte ptr [RAX + 0x104],0      ; absent

14048094f  LEA  RDX,[rip -> u"targetFromSvr"] ; 0x14328ae58, utf-16
14048095d  CALL 0x140910ca0                   ; GetInt(node, name, default 0)
140480962  TEST EAX,EAX
140480964  SETNE SIL
140480979  MOV  byte ptr [R13 + 0x1a0],SIL
```

`client-patched/Data/Mob/` is one partition (`Mob.ini` says `LastWzIndex|0`) holding
**193 mob images**. Every one was dumped with `wz-dump cat` and searched:

| key | images carrying it |
|---|---|
| `bodyAttack` | 193 |
| `link` | 37 |
| `boss` | 13 |
| `firstAttack` | 15 |
| `notAttack`, `noFlip`, `fixedDamage` | 1 each |
| **`patrol`** | **0** |
| **`targetFromSvr`** | **0** |

The 1-image and 13-image rows are the positive control: the search resolves keys that are
present in a single file, so **0 is a real zero**, not a broken search. Templates **1** (map
30's snail) and **2** (map 40) carry neither, and neither does any other mob in the client.

> `research/mob-spawn.md` section 8 used to call these "the single most likely reason a first
> attempt fails". They are not a reason at all. **137 bytes is correct for every mob in this
> client.**

### 11.3 The body layout is right - verified by a third instrument [L]

Independently of the earlier hand trace and shortest-path solve:

* **52 packet reads** in `FUN_141c4ff80`, bounded by `.pdata` to `0x141c4ff80..0x141c54054`
  (9 u8, 4 u16, 2 u16-thunk, 34 u32, 1 u64, 1 str, 1 raw). Same as section 5.1.
* A dominator test over the function's CFG asks of each read: *can the last read
  `0x141c536df` be reached without executing it?* **Exactly 35 reads dominate it**, and those
  35 are **character-for-character the set `crates/net/src/mob.rs` emits** - no unconditional
  read is missing and no gated read is being sent. 35 + the 17 gated = 52.
* Controls for the dominator tool: the HP read `0x141c504e9` dominates (it should); the
  patrol read `0x141c5052c` does not (it should not).

### 11.4 `mob+0x2b8` is a COM interface the client gives itself. No byte of ours reaches it [L]

The faulting sequence, out of a capstone disassembly bounded by `.pdata` to
`0x141c81040..0x141c81248` (the last 8 bytes are the function's jump table, and two bytes of
it do not decode - hence the resync):

```asm
141c81094  MOV   RAX,[RSI + 0x2b8]
141c8109b  MOV   EDX,0x848
141c810a0  TEST  RAX,RAX
141c810a3  LEA   RCX,[RAX + 0x828]
141c810aa  CMOVE RCX,RDX            ; RAX == 0  ->  RCX = 0x848
141c810b0  CMP   qword ptr [RCX],RDX
```

`0x848 = 0x20 + 0x828`, which is exactly `&((Obj*)nullptr)->field` for a pointer that aims at
`obj+0x20`. That is a self-consistent read of the constant and it is the whole bug: **null is
unguarded here.**

**What writes `mob+0x2b8`.** `tools/dataref.py` finds RIP-relative *globals*; nothing in
`tools/` matches a *struct field*. So: a whole-image sweep of `.text` and `.boot` with capstone
(18 524 276 instructions, **666 939 resync points** - 20 034 of them in `.text`) for every
instruction with a `[reg + 0x2b8]` memory operand. **3 906 hits**; 1 394 of those are
`CALL qword ptr [reg+0x2b8]` on unrelated objects, leaving **562 stores**, of which 258 are
`[rbp/rsp + 0x2b8]` stack frames - so **304 candidate object-field stores** in the whole image.

Positive controls before believing any of it: the sweep found `141c50505
MOV [RSI+0xb60],EAX` and `141c81094 MOV RAX,[RSI+0x2b8]`, both read by hand first.

Filtered to the **174 distinct methods in the eight mob vtables**, exactly **two** stores
touch the field:

| where | what |
|---|---|
| `141c4d1dd` in `FUN_141c4cee0`, the mob base ctor | `MOV [RSI+0x2b8],R14` with `R14 = 0` (`XOR R14D,R14D` at `141c4cf2b`) - **zero-initialised** |
| `141c50c9c` in `FUN_141c4ff80`, **`encodeInit` itself** | the assignment below |

Nothing else in mob code touches it; `FUN_141d33630`, `FUN_141c76190`, `FUN_141cc9410`,
`FUN_141d3a540` and the eight constructors contain no write to it.

**The value is not from the packet.** At `141c50bed`, immediately after the offset-103 count
loop:

```asm
141c50c26  CALL 0x142af7be0          ; new 0x10b8-byte object; null only if the allocator fails
141c50c35  JE   141c50c6d            ; null -> ECX = 0x80004002 (E_NOINTERFACE), RBX = 0
141c50c41  LEA  RCX,[RAX + 0x20]     ; an embedded interface sub-object
141c50c52  LEA  RDX,[rip -> 0x143273488]   ; {F28BD1ED-3DEB-4F92-9EEC-10EF5A1C3FB4}
141c50c59  CALL R9                   ; slot 0 = QueryInterface
141c50c63  CMOVNS RBX,[RBP + 0x1f8]  ; SUCCEEDED -> RBX = the interface
141c50c90  MOV  RCX,[RSI + 0x2b8]
141c50c97  CMP  RCX,RBX
141c50c9a  JE   141c50cb2            ; unchanged -> no store
141c50c9c  MOV  [RSI + 0x2b8],RBX    ; *** the only real writer ***
```

The `QueryInterface` on the far end is `FUN_142bcea20` (slot 0 of the vtable
`0x14348c928` that `FUN_142abe3b0` installs at `obj+0x20`). It compares the caller's IID
against four accepted ones and **`0x143273488` is the fourth of them** - `142bcea81` compares
the first qword against `[0x143273488]` and `142bcea8e` the second against `[0x143273490]`,
then falls into `142bcea97 LEA RAX,[RCX-0x20] / MOV [R8],RCX / XOR EAX,EAX`, i.e. it hands
back a non-null pointer and `S_OK`. Only an IID matching none of the four takes
`142bceab4 MOV EAX,0x80004002` and writes null. So on the normal path
**`mob+0x2b8` ends up non-null.** [L]

**And that block is unavoidable.** `0x141c50c90` **dominates** `0x141c532ab`, the read at body
offset 107 - there is no path from the entry of `encodeInit` to the tail of the body that
skips the assignment. (`0x141c50c9c` itself does not dominate it, only because of the
`CMP RCX,RBX / JE` idempotence check one instruction earlier.)

> **Conclusion, and it is the answer to "is the body implicated".** The 137-byte body cannot
> make `mob+0x2b8` null. If the field was null when `FUN_141c81040` ran, then either
> **(a)** that virtual ran *before* `encodeInit` reached `0x141c50c90`, or **(b)** `encodeInit`
> threw before getting there. **[D]**

### 11.5 Which of (a) and (b) - and the two watches that separate them

`FUN_141c81040` appears as a qword in **exactly eight** aligned `.rdata` slots and they are
the eight mob vtables, so whatever calls it, **the receiver is a mob** and `[rcx+0x2b8]` is
this field. (It sits at slot 46 in `0x1434077e8` and slot 48 in six others, while slot 7 and
slot 28 are identical across all eight - so it is one address serving more than one virtual,
the `/OPT:ICF` shape. It has **zero** direct callers.)

For **(b)**: an over-read raises rather than returning zeros. `FUN_1406e8c20` is

```asm
1406e8c32  MOV  EDI,[RCX + 0x18]     ; length
1406e8c35  SUB  EDI,[RCX + 0x24]     ; - cursor  = remaining
1406e8c79  CMP  EDI,4
1406e8c7c  JB   1406e8c91            ; short -> build an exception object and throw
```

so `CInPacket` is `+0x10` data, `+0x18` length, **`+0x24` cursor** - which is independently
`VIEW_DATA` / `VIEW_LEN` / `VIEW_CURSOR` in `crates/grap-stub/src/probe.rs:108`, and that file
also records that the buffer opens with a **4-byte frame header** and the opcode moves the
cursor **4 -> 6**. So body offset *N* is cursor *6 + N*. The fixture logs no C++
throw - but that is **not evidence**: `probe.rs`'s `THROW_LOG_AFTER_MS` is **25 000 ms** and
the fault landed **11.7 s** after the hook armed, so a throw at that moment could not have
been logged. [L]

A direct-call graph of the whole image (100 977 callers) rooted at every function on the
spawn path - `FUN_141d33630`, the factory, the eight constructors, `FUN_141c543e0`,
`FUN_141cc9410`, `FUN_141c76190`, `FUN_141cc15c0` (`vtable+0xe0`, called unconditionally at
`141c76563` *before* `encodeInit`), `FUN_141c9c210`, `FUN_14046fba0` - reaches 2 464 functions
and contains **three** indirect call sites at a vtable offset where `FUN_141c81040` lives, all
`CALL [RAX+0x170]`. Two are behind `FUN_141c9c210 -> FUN_140dc17f0 -> FUN_140e5afa0`; one is
`FUN_141c76190 -> FUN_141c77860 -> FUN_141c7a610`, and *that* one is inside the block
`141c764e8` guards with `TEST RCX,RCX / JE` on `mob+0x2b8` itself, so it cannot be the
faulting call. **This does not settle it** - the graph has only direct edges, and every step
of the real path goes through a vtable. Stop inferring here.

**Watch 1 - `141c81040:peek=2b8:hits=20`.** At the entry `rcx` is the mob, so the probe's
`peek` prints `[rcx+0x2b8]`, and `deref(rcx)` prints the low dword of the mob's vtable.

| what the line says | what it means |
|---|---|
| `[rcx+0x2b8]=…/u32:0x00000000` | the field is null when the crash function runs - the diagnosis above is confirmed, and `called-from=` plus the stack trace finally **name the caller of a virtual with no direct callers** |
| `[rcx+0x2b8]` non-zero | the field is fine and `0x141c810b0` is faulting on `[p+0x828]` for some other reason - a different bug, and the body is exonerated outright |
| the `[…]` after `rcx=` is `0x434077e8` / `0x4341f548` / `0x4341e510` / `0x4341e758` / `0x433765c0` / `0x4341ede0` / `0x433767f0` / `0x4341eff0` | the receiver is a mob of that class |
| no line at all, and the client still faults | the int3 did not arm - check the `int3 verified` line, not the theory |

**Watch 2 - `141c532ab:peek=24:hits=20`.** `141c532a8` is `MOV RCX,R12`, so at `141c532ab`
`rcx` **is the `CInPacket`** and `peek=24` reads the **cursor**. `141c532ab` is body offset
107, and `0x141c50c90` dominates it.

| what the line says | what it means |
|---|---|
| fires, cursor `0x71` (113 = 6 + 107) | `encodeInit` reached offset 107, so it **executed the `+0x2b8` assignment**, and the first 107 bytes of the body are byte-exact. Case (a). |
| fires, cursor anything else | the body layout is off by `cursor - 113` bytes at that point - and the number says by how much. Fix `mob.rs`. |
| never fires | `encodeInit` did not get that far: case (a) with the virtual firing early, or case (b), a throw. Watch 1's stack trace tells which. |

Run them together, with `-Mobs -MobLimit 1`, and change **nothing else** - the body is the one
thing already verified three ways, and altering it would confound the reading.

### 11.6 Two smaller corrections to this file

* Section 6.1 says the `objectId` at `141d3368e` "goes to `mob+0x3a0`". It does not: it goes
  to the stack local `[rbp-0x71]` and thence to `FUN_141d4f320(pool+0x68, &objectId, &mob)`,
  the pool's map insert. **`mob+0x3a0` and `mob+0x3a8` are both template pointers** -
  `141c50aad` passes `[rsi+0x3a8]` to the `template+0x1a0` accessor, and `141c8108d` uses
  `[rsi+0x3a0]` as a fallback template. [L]
* Section 5 lists `0x1434077e8` as the base vtable. That is right - slot 7 is `0x141c4ff80`
  and slot 28 is `0x141cc15c0` in all eight tables, which is the alignment check.
