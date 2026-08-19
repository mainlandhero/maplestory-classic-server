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

**Nothing here has been sent to the client.** Everything below is static analysis of
`client-patched\MapleStory.exe`. No client run, no capture.

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

1. **The three template-driven optional blocks.** `template[0x104]` adds 16 bytes,
   `template[0x1a0]` adds 4, and three specific template ids add 1. The accessors are
   `FUN_140479e60` (`template+0x83`), `FUN_140479e70` (`template+0x1a0`) and the inline
   `CMP byte [RCX+0x104]` at `141c50520`. **I did not find which WZ property fills any of
   them** - `FUN_140495990` is a cache and registry, not the parser. If the tutorial mob is a
   patrol mob the body is 16 bytes longer and the client will desync. The reference calls
   `template[0x104]` "isPatrolMob", which for an ordinary field mob is false. **[I], and it is
   the single most likely reason a first attempt fails.**
   *Next instrument:* find the function that writes `template+0x104`, and read the WZ property
   name it is next to; or dump the mob template for id 1 / 100100 out of `Mob.wz` and look for
   patrol data.
2. **Meanings for 17 of the 35 `encodeInit` fields.** They are sent as zero. That is the same
   "no readable consumer" argument `research/setfield-zero-audit.md` was written about, and it
   is weaker than a measurement.
3. **Whether `appearType` should be `-1` or `-2`.** Both skip the extra `u32`. The reference
   annotates `// init -> -2, -1 else`. Not settled; one byte to change.
4. **Whether a mob renders without a controller packet.** `0x3D2` is the pool's
   change-controller opcode (`FUN_141d30e80` case `0x3d2`, `u8 flag; u32 objectId;` then either
   a removal or `FUN_141d34a70(pool, flag, id, u8, packet)`). Not decoded, not needed to
   create the object. [L]
5. **Nothing has been sent to a client.** No run, no capture, no confirmation.

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
