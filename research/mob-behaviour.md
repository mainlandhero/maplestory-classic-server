# Mob behaviour: the server does not move mobs. It hands one to a client and gets a path back

Started 2026-08-19 after the owner saw six snails render on map 40 and stand completely still:
*"they are static, and do not have any behavior. In the live game, they move around according
to their animation, occasionally stop and play their idle animation, and then rinse and
repeat."*

Markers: **[L]** read out of the listing or the image, **[D]** derived from two or more [L],
**[I]** inferred - including anything from the v214 reference at
`C:\Users\user\Desktop\ModernMapleSource`, which is a **different game version** and scored
**1 of 8** against a held-out control. Every [I] is a candidate, never a fact.

**No Ghidra.** Everything here is `tools/reads.py`, `tools/callers.py`, `tools/switch_cases.py`
and capstone over `client-patched/MapleStory.exe` bounded by `.pdata`. Nothing needs the
decompiler to be re-checked; the two decompiled files used (`research/msexe-mobpool.c` for the
dispatcher's case list, and the read counts) were both re-derived from the image.

---

## 0. Answer up front

| | |
|---|---|
| the packet that makes a mob move | **`0x03D2`, MobChangeController** [L] |
| smallest legal body, mob already spawned | **87 bytes** [D] |
| body, mob NOT yet spawned | **137 bytes - byte-for-byte `0x03C6`'s body with byte 0 reinterpreted** [D] |
| what the client sends back | outbound **`0x02FF`**, built by mob primary-vtable **slot 22** = `FUN_141cb6880` [L] |
| does the server ever push a movement path? | **yes, `0x03D9`** - but that is the *rebroadcast* to clients which do **not** control the mob, and it is not how autonomous behaviour is produced. See the retraction in section 2.2 [L] |
| what has to be answered | **nothing.** No mob-pool handler writes the client's move id `mob+0x2f4` [L] |
| implemented in | `crates/net/src/mobmove.rs` |

**Independent corroboration, from the combat agent on the same day.** It decoded the client's
attack packet `0x00DF` out of a real map-40 capture: the owner at `(473, 395)`, mob 2000 at
`(424, 395)` - 49 pixels away on the same ground line - and the attack carried **zero
targets**. The client would not even aim at a mob we had spawned. One missing packet that
hands the client responsibility for a mob explains both symptoms at once, and its leading
suspect, reached from a different direction, was also `0x03D2`.

**The lead in the brief was right.** The mechanism is control transfer, not movement
transmission. What the brief did not predict, and what makes this more than a naming
question, is the **one virtual call that separates the two handlers** - section 4.

---

## 1. The instrument, and its positive control

Every negative below rests on one of three tools. Each was proved able to find a positive
control on this binary before any absence was reported.

| tool | positive control that fired |
|---|---|
| `tools/reads.py 0x140304100 2` | the equipped-item decoder, showing a **mix** of `(direct)` and `-> READS via helper` - the documented control in its own docstring |
| `tools/callers.py` | `0x141cc20b0` returns 6 call sites in 5 functions, including `0x141d34a70` which I had already read by hand |
| a capstone range scan for a `[reg + DISP]` operand (`scratchpad/rangescan.py`) | scanning `+0x2f4` over the mob range returned `141cb7ef3 MOV [R12+0x2f4],EAX`, an instruction read by hand first; scanning `+0x960` returned the constructor's `MOV byte [RSI+0x960],0` |

The scan **disassembles** - it never looks for a `0xE8` byte - and it resyncs one byte at a
time on undecodable input, so it does not stop silently at the first bad byte.

**And the jump table was read out of the image rather than trusted from the decompiler**,
because "enumerate before you filter" is the rule this file most depends on. See section 2.

---

## 2. The complete inbound mob-packet set is eight opcodes, read from the table

`FUN_141d30e80` is not a 137-way switch. Its head is: [L]

```asm
141d30e9a  LEA  EAX,[RDX - 0x3c6]
141d30ea0  CMP  EAX,0x12
141d30ea3  JA   0x141d31157            ; default
141d30eab  LEA  RCX,[RIP - 0x1d30eb2]  ; = 0x140000000
141d30eb2  MOV  R9D,dword ptr [RCX + RAX*4 + 0x1d31184]
141d30eba  ADD  R9,RCX
141d30ebd  JMP  R9
```

19 entries at `0x141d31184`, each a rel32 from `0x140000000`. Read directly: [L]

| opcode | target | handler |
|---|---|---|
| `0x3C6` | `141d30ec0` | `FUN_141d33630` - **MobEnterField** |
| `0x3C7`..`0x3D0` | `141d31157` | **default - nothing** |
| `0x3D1` | `141d30ed0` | `FUN_141d33c70` |
| `0x3D2` | `141d30ee0` | inline + `FUN_141d34a70` - **MobChangeController** |
| `0x3D3` | `141d30fda` | inline -> `FUN_141cdf1e0` |
| `0x3D4` | `141d3103d` | `FUN_141d34440` |
| `0x3D5` | `141d31157` | **default - nothing** |
| `0x3D6` | `141d3104d` | inline -> `FUN_141c99730` |
| `0x3D7` | `141d31101` | inline, sets `mob+0x1100` |
| `0x3D8` | `141d3114a` | `FUN_141d34830` |

`research/mob-spawn.md` §1 says the mob pool range is `0x3C6..0x44E`. That is the **routing**
range in `CField::OnPacket`. The pool's *first* switch covers `0x3C6..0x3D8`; the rest goes to
`FUN_141d32b30`, which is a **second dispatcher** - section 2.2.

### 2.1 What the first eight read

`tools/reads.py`, depth 1-3, re-run after the tool's ten-primitive and tail-`jmp` fixes: [L]

| case | reads | name |
|---|---|---|
| `0x3C6` | `u8, u32, u8, u32, u8`, mask, `encodeInit` | **MobEnterField** |
| `0x3D1` | `u32, u8, u8`, then gated `u32, u32`, then gated `u32` | MobLeaveField (the combat agent's reading) |
| `0x3D2` | see section 3 | **MobChangeController** |
| `0x3D3` | `u32, u16, u32, u32, u8` -> `FUN_141cdf1e0` | |
| `0x3D4` | `u32`, gated `u32, u32` | |
| `0x3D6` | `u32 count`, then per element `u32, u32, u8, [u32, u32], u32` | |
| `0x3D7` | `u32, u8` | |
| `0x3D8` | `u32`, gated `u32` | |

### 2.2 RETRACTION: there IS a second jump table, and `0x03D9` decodes a movement path

**The first version of this file said "the server cannot move a mob, because this client has
no code to read a path". That is false, and it is false by exactly the mistake section 9 warns
about: I read one jump table and treated its `default:` as dead.**

`research/msexe-mobpool.c` renders the default as `FUN_141d32b30(param_1)` - one argument, no
packet - and I believed it. The decompiler dropped two arguments that are passed **implicitly**:
`FUN_141d30e80(pool, opcode, packet)` reaches its default at `141d31157` without ever
clobbering `RDX` or `R8`, so `141d31162 MOV RCX,RSI / CALL 0x141d32b30` hands on all three.
The function's own prologue proves it: [L]

```asm
141d32b42  MOV  RDI,R8          ; the packet
141d32b45  MOV  ESI,EDX         ; the opcode
141d32b47  MOV  RBX,RCX         ; the pool
141d32b4d  CALL 0x1406e8c20     ; u32 objectId  <- IT READS THE PACKET
141d32b57  CALL 0x141d2efc0     ; find that mob, or fall through to the default
141d32b68  LEA  EAX,[RSI - 0x3d9]
141d32b6e  CMP  EAX,0x74
141d32b71  JA   default
141d32b81  MOV  EDX,[RAX + RCX*4 + 0x1d33448]   ; a SECOND 117-entry jump table
141d32b8b  JMP  RDX
```

Read out of the image at `0x141d33448`, 117 entries: **102 live cases, 15 defaults.** Every
live stub is `MOV RDX,RDI / MOV RCX,RBX / CALL handler`, i.e. `handler(mob, packet)`. [L]

The tool that caught it is `tools/reads.py` after its ten-primitive fix: run on the
dispatcher at depth 2 it printed
`0x141d31165 call 0x141d32b30 -> READS via helper: u32 u8 u64 u16 raw str`, and that line is
the whole retraction.

**`0x03D9` is `MobMove`.** Its handler `FUN_141c813b0` (3233 bytes, 23 read sites): [L]

```asm
141c813d0  u8   flags       bit 0 -> BL, bit 2 -> R13B
141c813f1  u8   -> EDI
141c81401  the mob's own animation state at mob+0x2e4, de-obfuscated
141c81414  CMP EAX,1 / JNE
141c8141f  SHR ECX,1 / SUB ECX,0xd / CMP ECX,0x10 / JA bail
```

`SHR ECX,1` on the second byte is the **same `action * 2 + facing` split** `encodeInit` does at
`141c50dbf`, so byte 1 of a `0x03D9` body is a `move_action`. The handler ends with
`141c82009 CALL 0x141d598b0` - a decoder in the movement-path neighbourhood, one `.pdata` entry
away from the encoder `FUN_141d57c60` - and with `141c81784 MOV dword [R14+0xcd0],0`, clearing
the very field that blocks the move sender. [L]

**What this does and does not change.** `0x03D9` is how a client that does **not** control a
mob is told where that mob went - the rebroadcast of some other client's `0x02FF`. It is not a
way to author behaviour: the wander itself is rolled by the controlling client (section 5.1),
its path encoding is undecoded here, and with one player on the field there is no
non-controller to send it to. **The conclusion of this file is unchanged. One of the arguments
it rested on was wrong, and this section is what replaces it.**

---

## 3. `0x03D2` decoded, byte for byte

The dispatcher reads three fields itself and forks on the first: [L]

```asm
141d30ee3  CALL 0x1406e8ae0   ; u8  level   -> EBP        body offset 0
141d30eee  CALL 0x1406e8c20   ; u32 objectId -> EBX       body offset 1
141d30ef5  TEST EBP,EBP
141d30ef7  JE   0x141d30f1c   ; level == 0 -> REMOVE the mob from the pool
141d30efc  CALL 0x1406e8ae0   ; u8         -> R9D         body offset 5
141d30f12  CALL 0x141d34a70   ; (pool, level, objectId, thatByte, packet)
```

**`level == 0` is a despawn, not a "drop control".** The zero branch walks the pool's hash map
at `pool+0x68`, calls `[vtable+0x48]` (bail if 0), `[vtable+0x40](mob, 0)`, `FUN_141c543c0`
(bail if non-zero), and then erases the mob from `pool+0x38`, `pool+0x68` and `pool+0xa8`. [L]

### 3.1 `FUN_141d34a70` - what it reads, and the fork that changes the length

`tools/reads.py 0x141d34a70 1`: [L]

```
0x141d34aac  READ u32 (direct)                                  <- templateId, ALWAYS
0x141d34c32  READ u8  (direct)                                  <- forced-stat gate, new-mob branch
0x141d34c4f  call 0x141c76190 -> READS via helper: raw          <- 20-byte temp-stat mask
0x141d34c6a  READ u8  (direct)                                  <- forced-stat gate, existing branch
0x141d34c87  call 0x141c76190 -> READS via helper: raw
0x141d34c94  call 0x141c54060 -> READS via helper: u16 u8 u32 u64
```

`FUN_141d2efc0(pool, objectId)` decides the branch: [L]

```
141d34a99  FUN_141d2efc0(pool, objectId)
141d34ab7  TEST RDI,RDI
141d34aba  JNE  141d34c67                 ; already in the pool  -> FUN_141c54060   87 bytes
           ; else create it: FUN_140495990(template) -> FUN_141d3a540 -> pool inserts
141d34c5f  CALL qword ptr [RAX + 0x38]    ; not in the pool      -> encodeInit     137 bytes
```

So **`0x03D2` is `0x03C6`'s body with byte 0 reinterpreted**, and it stops 50 bytes earlier
when the mob is already on the client's field:

| off | size | read at | name |
|---|---|---|---|
| 0 | u8 | `141d30ee3` | **controller level** (`0` despawns) |
| 1 | u32 | `141d30eee` | **objectId** |
| 5 | u8 | `141d30efc` | the same byte `0x03C6` puts at offset 5 - it becomes `FUN_141c76190`'s third argument in **both** handlers (`141d3390e` passes `[rbp-0x75]`, the offset-5 byte; `141d34c7e` passes `r13d`, this one) |
| 6 | u32 | `141d34aac` | **templateId** |
| 10 | u8 | `141d34c32`/`141d34c6a` | forced-stat gate, `0` costs nothing |
| 11 | raw[20] | `141c76276` | temporary-stat mask, all-zero costs nothing |
| 31.. | | | **new mob**: `encodeInit`, 106 bytes, total **137** |
| 31..86 | | `FUN_141c54060` | **existing mob**: 56 bytes, total **87** |

### 3.2 `FUN_141c54060` is a strict prefix of `encodeInit`

25 read sites, `.pdata`-bounded to `0x141c54060..0x141c541eb`, from `tools/reads.py` and
confirmed against a hand trace of the listing: [L]

| body off | what | matches `encodeInit` at |
|---|---|---|
| 31 | i16 x | `141c4ffc5` |
| 33 | i16 y | `141c501b0` |
| 35 | u8 `move_action` - **read and discarded here** | `141c503f2` |
| — | u8 iff `FUN_14045b1a0(templateId)` | `141c5043d` |
| 36 | i16 fh | `141c50456` |
| 38 | i16 home fh | `141c50465` |
| 40 | u8 -> `mob+0x116c` | `141c50474` |
| 41 | i8 appearType | `141c50485` |
| — | u32 iff `appearType >= 0 \|\| == -3` | `141c504d0` |
| 42 | i16 -> `mob+0x1228` | `141c50499` |
| 44 | i16 -> `mob+0x122c` | `141c504aa` |
| 46 | u32 hp scale -> `mob+0x8c0` | `141c504db` |
| 50 | u64 hp | `141c504e9` |
| 58 | u32 effect item id | `141c5050e` |
| — | 4 x u32 iff `template[0x104]` (`patrol`) | `141c5052c` |
| 62,66,70 | u32 x3 | `141c5056f`, `141c505a8`, `141c50621` |
| 74,78,82 | i32, i32, u32 -> `FUN_141cdf1e0` | `141c5066f`, `141c50680`, `141c5068a` |
| 86 | u8 | `141c50694` |
| **87** | **end** | `encodeInit` continues with a count word |

**One difference, and it is real.** The appear-option gate is `appearType >= 0 || == -3` here
(`141c54101 MOV ECX,0x80000000 / LEA EAX,[RSI+RCX] / TEST ECX,EAX / JNE take` then
`CMP ESI,-3 / JNE skip`), where `encodeInit` also accepts **`-6`** (`research/mob-spawn.md`
§6.3). We send `-2`, which skips both, so nothing turns on it today - but a builder that
copied `encodeInit`'s predicate would desynchronise a `-6` mob. `crates/net/src/mobmove.rs`
implements the `0x3D2` predicate, not the `0x3C6` one. [L]

---

## 4. The mechanism: one virtual call the spawn packet never makes

This is the part that is not a naming exercise. Enumerate the **indirect** calls each handler
makes, out of the listing: [L]

```
FUN_141d33630   0x03C6   [rax+0x10] [rax+0x20] [rax] [rax] [rax+0x38] [rax] [rax] [rax] [r8+0x108]
FUN_141d34a70   0x03D2   [rax] [rax] [rax+0x38]  ***[rax+0x40]***
```

`[rax+0x40]` is mob primary-vtable **slot 8** = `FUN_141c54200`, and **only the
change-controller handler calls it** - with a hardcoded `EDX = 1`, regardless of the level
byte (`141d34ca9 MOV EDX,1`). [L]

```asm
141c54200  MOV  RDI,[RCX + 0x2c0]     ; the second animation interface
141c54217  JE   ret                   ; null -> nothing at all
141c5421d  MOV  RAX,[RCX + 0x2b8]     ; the first
141c54227  JE   ret
141c54241  CALL 0x1409c5080           ; is the animation object already running?
141c54248  JNE  ret                   ; yes -> nothing
141c5424e  LEA  EDX,[RAX + 3]         ; RAX == 0 here, so EDX = 3
141c54254  CALL 0x141c4ff30           ; mob's obfuscated state at mob+0x2e4 := 3
141c54261  CALL 0x141c55750(mob, 1)
```

`FUN_141c4ff30(mob, v)` is a 73-byte leaf that stores `v` XOR-obfuscated across
`mob+0x2e4 / +0x2e8 / +0x2ec` - the same three-word obfuscation `FUN_141c543e0` uses for
`mob+0x2d8` and `encodeInit` uses for `move_action` at `mob+0x3dc`. [L]

**And `mob+0x2e4` is read back by the mob-move sender**, `FUN_141cb6880` at `141cb864a`:

```asm
141cb8642  LEA  RSI,[R12 + 0x2e4]
141cb864a  MOV  ECX,[RSI]
141cb864c  MOV  EAX,[RSI + 4]
141cb864f  MOV  EDI,EAX / ROL EDI,5 / XOR EDI,ECX     ; the plaintext state back
```

So the chain is: [D]

```
0x03D2  ->  mob+0x960 := level              (141d34ca0)
        ->  [vtable+0x40] = FUN_141c54200(mob, 1)
              ->  FUN_141c4ff30(mob, 3)     mob's animation state := 3
              ->  FUN_141c55750(mob, 1)
        ->  FUN_141c54430(mob, level > 1, ...)
```

`0x03C6` performs none of it. **A mob that is spawned and never granted is created, pooled,
drawn - and never switched on.** That is exactly what six motionless snails look like. [D]

### 4.1 The `level > 1` hazard, and why to send `1`

`FUN_141cc1e40` is a 13-byte leaf: `XOR EAX,EAX / CMP byte [RCX+0x960],1 / SETA AL / RET` -
i.e. **`level > 1`**, the aggro/chase flag. Its value is `FUN_141d34a70`'s tail-call argument
2 into `FUN_141c54430`, and inside that function argument 2 gates this: [L]

```asm
141c544b3  TEST R14D,R14D
141c544b3  JE   141c5453e            ; level <= 1 -> the whole block below is skipped
...
141c544ed  MOV  EAX,[RCX + 0x178]    ; the template
141c544f3  DEC  EAX / CMP EAX,1 / JA skip
141c544fa  MOV  RAX,[RSI + 0x2c0]
141c54501  LEA  RDX,[RAX + 0xf84]
141c54508  MOV  R8D,0xfa4
141c5450e  TEST RAX,RAX
141c54511  CMOVE RDX,R8              ; null -> RDX = 0xfa4
141c54515  CMP  dword ptr [RDX],0    ; <-- the SAME unguarded shape that killed the client
```

This is the `mob+0x2c0`-is-null crash of `research/mob-spawn.md` §2g wearing a different hat.
It cannot fire on our flow (a mob that took `0x03C6` has already run `encodeInit`, which
creates `mob+0x2c0` at `141c50eed`), but **level `1` skips the block outright** and level `2`
does not. Send `1`. [D]

### 4.2 What `mob+0x960` is actually used for

A whole-image capstone scan for a `[reg + 0x960]` operand - 18.5M instructions, resyncing -
returns exactly **four** sites inside mob code, and its positive control (the constructor's
zero, read by hand first) is one of them: [L]

| where | what |
|---|---|
| `141c4dcb5` in `FUN_141c4cee0`, the base ctor | `MOV byte [RSI+0x960],0` |
| `141cb85ee` in `FUN_141cb6880` | `MOVZX EDX,byte [R12+0x960]` -> **encoded into the outbound `0x02FF` body** |
| `141cc1e42` (`FUN_141cc1e40`) | `CMP byte [RCX+0x960],1 / SETA` |
| `141d34ca0` in `FUN_141d34a70` | the write from the packet |

> **A negative I am reporting as a negative, not smoothing over.** There is **no
> `mob+0x960 != 0` test anywhere in the image.** The level byte does not itself gate the local
> AI; the activation is the slot-8 call in section 4, which happens for **any** non-zero level.
> So "0x03D2 starts the mob" is [D] through the slot-8 call, not through the level byte, and
> the level byte's only behavioural consumer is the `> 1` aggro flag. [L]

---

## 5. What the client sends back: outbound `0x02FF`

`FUN_141cb6880` is the mob's primary-vtable **slot 22**. The eight aligned `.rdata` qwords
holding it are `mob_vtable + 0xb0` for each of the eight mob vtables
`research/mob-spawn.md` §5 reached through their constructors - `0x1434077e8 + 0xb0 =
0x143407898`, and so on for all eight. It has **zero direct callers**. [L]

It contains exactly one `COutPacket` construction: [L]

```asm
141cb7ea5  MOV  EDX,0x2ff
141cb7eaa  LEA  RCX,[RBP + 0xd0]
141cb7eb1  CALL 0x1406ed520          ; COutPacket::COutPacket(opcode)
141cb7eb7  MOV  EDX,[R12 + 0x3a0]  -> Encode4    objectId
141cb7ecb ..141cb7f05               -> Encode2    a per-mob counter at mob+0x2f4, incremented
141cb7f0a  packed byte              -> Encode1
...
141cb8353  CALL 0x141d57c60          ; the movement path
141cb8365  CALL 0x1406ed610          ; SendPacket
```

The encode primitives were identified from their own bodies rather than assumed:
`1406ed520` = ctor (writes the opcode through `1406ed940`), `1406ed840` = 1 byte,
`1406ed940` = 2, `1406ed9d0` = 4, `1406edbc0` = 8, `1406edc80` = string, `1406ede20` = raw,
`1406ed610` = send. Each was read at its `ADD dword ptr [rbx+0x428],n`. [L]

**`mob+0x3a0` is the object id**, not a template pointer. A scan for writes to `+0x3a0` over
the mob range returns two: the constructor, and `141c4ffb4 MOV dword [RSI+0x3a0],EBX` inside
`encodeInit`, whose `EBX` is `encodeInit`'s second argument - the object id the spawn handler
passes at `141d33923 MOV EDX,R12D`. [L]
*(`research/mob-spawn.md` §11.10 says `mob+0x3a0` and `mob+0x3a8` are "both template
pointers". The **qword** at `mob+0x3a8` is the template, and `FUN_141c81040` reads it as
`[this+0x3a0]` because its `this` is `mob+8`. The **dword** at `mob+0x3a0` is the object id.
Both readings are of different-width accesses to different addresses.)*

`FUN_141d57c60` is the same movement-path encoder the player's own move packet uses:
`research/msexe-packet-fields.txt` line for `0x00D9` names `FUN_1409f6eb0` as its builder, and
`tools/callers.py 0x141d57c60` puts `0x1409f6eb0` in its 9-caller list. [L]

### 5.1 Where the path comes from: the client rolls it

Mob primary-vtable **slot 19** is `FUN_141c8d1b0`. At `141c8d31c` it loads the mob's own
vtable and takes **slot 22 into a local**: [L]

```asm
141c8d300  LEA  RCX,[R15 + 0x3e8]     ; the obfuscated "can act" field
141c8d30e  CALL 0x1401ba9d0           ; de-obfuscate
141c8d313  CMP  EAX,-1
141c8d316  JG   skip
141c8d31c  MOV  RAX,[R15]             ; the mob's vtable
141c8d31f  MOV  RAX,[RAX + 0xb0]      ; slot 22 = FUN_141cb6880
141c8d326  MOV  [RBP - 0x51],RAX      ; stashed for the call
...
141c8d342  MOV  EDX,0xc               ; 12-byte path elements
141c8d361  CALL 0x142f04924           ; a random source, called once per element field
141c8d366  ADD  EAX,EDI
141c8d37d  MOV  [RCX + 5],AL
141c8d388  MOV  [RCX + 6],AL
141c8d397  ... IMUL/SAR ... 0x6f      ; a modulo-111 step counter
141c8d3e5  CALL 0x142f04924
141c8d3f1  MOV  [RCX + 4],AL
141c8d3f8  MOV  [R11 + 8],R13W
```

**That is the wander: a list of 12-byte path elements built out of a random source, then
handed to the packet builder.** The server never sees a decision, only the result. [D]

The same "can act" field `mob+0x3e8` is the one `FUN_141cdea30` tests
(`obf(mob+0x3e8) <= -1 && mob+0xcd0 == 0 && mob+0xca4 == 0`), and `FUN_141cdea30` is called
once, from `FUN_141d30790` = `CMobPool::Update` at `141d30941`. [L]

### 5.2 `mob+0xcd0` must stay zero, and body offset 74 is why

`FUN_141cb6880`'s **first** act is to bail: [L]

```asm
141cb6907  CMP  dword ptr [R12 + 0xcd0],0
141cb6910  JE   continue
141cb6912  ...                        ; non-zero -> return without building anything
```

`mob+0xcd0` has three writers in mob code, found by the same range scan (positive control:
the `141c541c6` store I had already read by hand): the constructor, `141c530b8` in
`encodeInit`, and `141c541c6` in `FUN_141c54060` - and **both packet writers are gated on body
offset 74 being `>= 0`**:

```asm
encodeInit        141c530af  CMP dword ptr [RBP + 0xe8],-1 / JLE skip / MOV [RSI+0xcd0],EDI
FUN_141c54060     141c541ab  CMP EBP,-1 / JLE skip / ... / MOV dword [RDI+0xcd0],1
```

`crates/net/src/mob.rs` already sends **-1** there, for an unrelated reason (it skips a
`FUN_141cdf1e0` call with an out-of-range index). That choice turns out to be load-bearing for
movement as well: **a `0` at body offset 74 would set `mob+0xcd0` and the mob could never
report a move.** Keep it negative in both packets. [D]

---

## 6. Nothing has to be answered

The client stamps its own move id into `mob+0x2f4` and increments it locally
(`141cb7ecb` reads it, `141cb7ef3` writes it back). A range scan over the mob address space
for `+0x2f4` returns 20 sites; **not one of them is inside any of the eight mob-pool packet
handlers**, and the two writers are the constructor and the move builder itself. [L]

So there is **no `MobCtrlAck` in this client's mob pool**, and an unanswered `0x02FF` is not
the "always answer" hazard `CLAUDE.md` warns about - it is a notification, not a request.
[D, and it is a negative: it rests on the enumerated jump table of section 2 plus a scan whose
positive controls fired.]

**Log it anyway.** `0x02FF` arriving at all is the single cheapest confirmation that this whole
analysis is right, and it costs nothing.

---

## 7. What this does NOT settle

1. **Whether one `0x03D2` per mob is sufficient on screen.** Every link is [L] and the chain
   is [D], but the client has not been run. The slot-8 activation is the mechanism; whether
   the animation object then produces a wander without any further packet is **[I]**.
2. **What `move_action` should be for a *moving* mob.** The 16-entry jump table at
   `0x141c53fe4` maps `action = byte / 2` to an animation index: action 1 -> `0x4e` or `0`,
   2 -> 1, 3 -> 2, 6 -> 3, 8 -> 4, 13 -> `0x43`, 16 -> `0x2f`, everything else -> 1 or 3. We
   send byte `2` = action 1. The `0x03D2` existing-mob path **reads offset 35 and discards
   it** (`141c54093`, result never stored), so it cannot be used to change the stance of an
   already-spawned mob. [L]
3. **The controller level's meaning beyond `> 1`.** `1` vs `2` differs only in
   `FUN_141cc1e40`, which reaches `FUN_141c54430` and (per the v214 reference) means "chase
   the player". Naming it aggro is **[I]**.
4. **Which mob the server should hand to which client.** Irrelevant today - one player - but
   a real server rotates control on proximity, and `0x03D2` with level `0` **despawns** rather
   than releasing, so a naive "revoke" would delete the mob. Section 3.
5. **`0x3D1`, `0x3D3`, `0x3D4`, `0x3D6`, `0x3D7`, `0x3D8`.** Read counts only; no field
   meanings. `0x3D3`'s five reads land in `FUN_141cdf1e0`, the same consumer as `encodeInit`
   offsets 74-86, so it is a candidate for "server retargets a mob" - unread.
6. **Respawn and death.** Nothing here kills a mob, so nothing here respawns one.
7. **The 102 second-level cases `0x3D9..0x44D`.** Enumerated and read-counted (section 2.2);
   only `0x3D9` was decoded. The five largest by read site count, as a starting list for
   whoever needs them: `0x3D9` -> `FUN_141c813b0` (23), `0x413` -> `FUN_141c87890` (16),
   `0x3FE` -> `FUN_141cc38e0` (13), `0x3F7` -> `FUN_141c83a60` (11), `0x3EB` ->
   `FUN_141c82bd0` (10). **This whole range was invisible to this project until today.**

---

## 7a. Two corrections to `research/mob-spawn.md` §6.3, found by re-running the fixed tool

Both are gates that section calls free, and both cost bytes the moment they are not zero.
Neither changes `crates/net/src/mob.rs`, which sends `0` for both - but a future field that
sets either would desynchronise the rest of the body with nothing to resync on. [L]

| offset | §6.3 says | actually |
|---|---|---|
| 107, `u8` at `141c532ab` | "gate (no reads inside)" | `141c532b2 TEST AL,AL / JE 141c534d6` - the non-zero branch reaches `141c533bf CALL 0x14023dca0`, which reads **8 x u32 + u64 = 40 bytes** unconditionally, plus whatever else is in that block |
| 117, `u32` at `141c53518` | "gate (no reads inside)" | `141c53524 JE 0x141c535ab` jumps **past** `141c535a6 CALL 0x14028d820`, which reads **2 x u32 = 8 bytes** |

Both were invisible to the old read walk because it counted only direct calls to a primitive.
Both callees take the `CInPacket *` in `RDX` from `R12` at the call site, which is what
separates them from the false positive in section 9.

**And the 137-byte spawn body is confirmed *not* short.** Those are the only two read sites in
`encodeInit` that `crates/net/src/mob.rs` does not account for, and both sit inside branches
the bytes we send do not take. That was checked rather than assumed, because a body that is
48 bytes short is precisely the failure that has cost this project a client session.

---

## 8. The files this rests on

| file | what |
|---|---|
| `research/msexe-mobpool.c` | the dispatcher's case bodies. **Its case list is complete but its `default:` is misleading** - the real bound is `opcode - 0x3C6 <= 0x12`, section 2 |
| `research/msexe-packet-fields.txt` | named `FUN_1409f6eb0` as the `0x00D9` builder, which is how `FUN_141d57c60` was identified as the shared path encoder |
| `research/mob-spawn.md` | the 137-byte body, the eight mob vtables, and the `move_action` crash. Section 3 here is that body with byte 0 renamed |
| `crates/net/src/mob.rs` | the enter-field builder this reuses verbatim for the 137-byte form |
| `crates/net/src/mobmove.rs` | **new** - the change-controller builders and the `0x02FF` head parser |

---

## 9. Instrument notes worth keeping

* **A decompiled call can drop arguments that are passed implicitly.** `msexe-mobpool.c`
  renders the mob dispatcher's default as `FUN_141d32b30(param_1)`. It takes three, and the
  other two are `RDX` and `R8` still holding `FUN_141d30e80`'s own arguments. Believing the
  decompiler's arity hid **102 opcodes**, including the one that decodes a movement path. If a
  forwarding call looks like it drops the packet, read the callee's prologue.
* **`tools/reads.py` follows a helper without checking that the helper gets the packet.** It
  reported `FUN_141d32b30` as reading - correct - and it would equally report a helper that
  reads some unrelated stream. The discriminator is one instruction at the call site: is the
  `CInPacket *` in `RDX`? For `141c533bf` and `141c535a6` it is (`MOV RDX,R12`, and `R12` is
  the packet two instructions later). Check it every time; `research/mob-spawn.md` §10 says
  the same thing and it is still the rule that decides.
* **`call qword ptr [reg + 0xb0]` is not "mob vtable slot 22".** All four such sites in the mob
  address range are COM calls on an unrelated object - each is followed by
  `TEST EAX,EAX / JNS` and an `HRESULT` reporter at `0x142ef3ad0`. The real slot-22 use is a
  `MOV RAX,[RAX + 0xb0]` that stashes the pointer for a later indirect call. **A displacement
  is a class fact, not an offset fact** - the same trap as `research/mob-spawn.md` §11.7, one
  level up.
* **Do not put a file called `dis.py` on `sys.path`.** It shadows the standard library's `dis`,
  which `inspect` imports, which `capstone` imports - and capstone then fails with a circular
  import that looks nothing like the cause.
* **`.pdata` bounds lie for chained entries.** `FUN_141c54200`'s `.pdata` record is 49 bytes
  long and its code runs to `0x141c5437d`. Bound a *dump* by `.pdata`, but follow the jumps.
* **A decompiled `default:` can hide the switch's real domain.** `msexe-mobpool.c` renders the
  bound as `param_2 - 0x3d9U < 0x75`, which reads as "0x3D9..0x44D are handled". They are not
  reached at all: the table is 19 entries wide. Read the table.
