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
| ~~what has to be answered~~ | ~~**nothing.**~~ **RETRACTED 2026-08-19 - see sections 10 and 10.1.** `0x03E4` **is** a `MobCtrlAck`: `FUN_141c82060` reads a `u16`, compares it against the client's own move counter at `141c82212`, and sets the mob's state. The old scan was right and the *set it was intersected with* was stale [L] |
| the rebroadcast to other clients | **`0x03D9`**, `FUN_141c813b0` - decoded in section 12. Not what freezes a single-player field [L] |
| why the mobs still froze | the client reported **one** move each and stopped; nothing answered. Section 10 [L] |
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
| a capstone range scan for a `[reg + DISP]` operand (`tools/rangescan.py`) | scanning `+0x2f4` over the mob range returned `141cb7ef3 MOV [R12+0x2f4],EAX`, an instruction read by hand first; scanning `+0x960` returned the constructor's `MOV byte [RSI+0x960],0` |

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

**`level == 0` RELEASES.** *(This paragraph said "is a despawn, not a 'drop control'" until
2026-09-04. The fact is owned by `crates/net/src/mobmove.rs::CONTROL_RELEASE` and worked in
`research/control-release-does-it-despawn.md`; what follows is the walk that was misread, kept
because the misreading is instructive - the two guards are in it and were skipped over.)*
The zero branch walks the pool's hash map
at `pool+0x68`, calls `[vtable+0x48]` (bail if 0), `[vtable+0x40](mob, 0)`, `FUN_141c543c0`
(bail if non-zero), and then erases the mob from `pool+0x38`, `pool+0x68` and `pool+0xa8`. [L]

> ### RETRACTED 2026-09-04 - level 0 RELEASES. See section 3.3 below and
> ### `research/control-release-does-it-despawn.md`.
>
> The sentence above is wrong, and the paragraph it sits in **contains its own refutation**:
> it lists two bail-outs and then names the branch after the third thing. Every clause in it
> is accurate; the summary is not. This is `CLAUDE.md`'s *"a conditional erase summarised as
> an unconditional one"*, and it propagated to `crates/net/src/mobmove.rs` (three doc blocks,
> a `debug_assert!` and a test name), `research/mob-share.md` (five places, including the
> design decision *"never rotate control"*) and `research/mob-hit-reaction.md:368`.

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
| 0 | u8 | `141d30ee3` | **controller level** (`0` releases - `net::mobmove::CONTROL_RELEASE`) |
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

### 3.3 The zero branch, read properly - it RELEASES

Added 2026-09-04. Full working in `research/control-release-does-it-despawn.md`; this is the
part that changes what section 3 says.

**Step 3 always bails for a mob that is in the field.** `FUN_141c543c0` is not a predicate on
anything the packet carries - it is a **getter** for an obfuscated triple, 14 bytes with no
`.pdata` entry: **[L]**

```asm
141c543c0  mov  edx,[rcx+0x2e0]   ; checksum word
141c543c6  add  rcx,0x2d8         ; the triple's base (the cookie; the VALUE is at +0x2dc)
141c543cd  jmp  0x141d11770       ; de-obfuscate: rol([base+4],5) ^ [base]
```

Its complete writer set, from `tools/callers.py` on both setters cross-checked against
`tools/rangescan.py` over all three words: **[L]**

```text
mob base constructor  FUN_141c4cee0  141c4d205 inline, and 141c4e6ce -> FUN_141d23730   ->  0
0x03C6 MobEnterField  FUN_141d33630  141d3372c (existing mob) and 141d338f2 (new mob)   ->  1
0x03D1 leave type 0   FUN_141d33c70  141d33dfc                                          ->  0
```

`FUN_141c543e0` has exactly **3 call sites in 2 functions**; `FUN_141d23730` exactly **1**, in
the constructor. Nothing else in the image writes it. It is a plain **"this mob is in the
field"** flag, and `FUN_141d34a70` - the `0x03D2` *grant* - is **not** among its writers.

So for any mob that arrived by `0x03C6`, `141d30f82` returns 1 and `141d30f89 jne 0x141d3116a`
returns straight out of the dispatcher (`141d3116a` is the epilogue). **The erase at
`141d30f8f` is unreachable.** [D]

**Step 2 is the mirror of the grant, not a teardown.** `[vtable+0x40]` is slot 8
`FUN_141c54200` - the same function section 4 reads, but section 4 only ever read the
`edx = 1` arm. `tools/listing.py` merges 4 contiguous `.pdata` entries to give the real
382-byte extent; the `edx == 0` arm at **`0x141c542b5` is its own 190-byte `.pdata` entry**,
which is why it had never been read: **[L]**

| | grant `edx=1` | release `edx=0` |
|---|---|---|
| animation-running test `0x1409c5080` | `141c54248 jne` -> already running, do nothing | `141c542bc je` -> not running, do nothing |
| state `mob+0x2e4` | `:= 3` | `:= -2` (from 3), `-3` (from 4), else `-1` |
| pump | `FUN_141c55750(mob, 1)` | `FUN_141c55750(mob, 0)` |
| `[anim+0x118]` | live de-obfuscated coordinates, `141c558b9` | **all zeros**, `141c54349` |
| touches the pool | no | no |

**Step 1, `[vtable+0x48]`, is slot 9 `FUN_141c54390`** (34 bytes, no `.pdata`): returns 0 if
`mob+0x2c0` is null, else the same `0x1409c5080` predicate. `0x03E4 MobCtrlAck` confirms the
polarity from a completely independent path - `141c8207d` calls slot 9 and, **if it is 0**,
calls slot 8 with `1` to *start*. Ack starts, level-0 stops. **[L]**

**Consequences for sections 7 and 13.** Section 7 item 4 and section 13's *"a naive rotation
deletes the mob"* are withdrawn. Rotation is available; the sequence is **release A, then grant
B, then relay `0x03D9` to A** - order matters, and granting first is what made mobs teleport on
2026-09-04. Two hazards that are *not* obvious: **`0x03E4` un-does a release** (do not ack a
`0x02FF` from a client that is no longer the controller), and a mob known to a client only from
a **137-byte `0x03D2`** never gets the in-field flag set, so for *that* mob level 0 really does
erase.

**And the claim had never been tested.** A census of 505 distinct archived log files
(240 788 events, deduplicated on `(timestamp, direction, opcode, body)`) finds **2 664**
`0x03D2` bodies with level `1` and **zero** with level `0`.

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
141cb8358  CMP  byte [RBP-0x80], 0   ; <-- A BRANCH. Not the end of the packet.
141cb835c  JE   141cb8374            ;     zero -> carry on and send
141cb835e  LEA  RCX,[RBP+0xd0]
141cb8365  CALL 0x1406ed610          ; ~COutPacket - the ABORT arm, destroyed UNSENT
...                                  ; 11 more encodes on the je path
141cb8632  CALL 0x1415d01c0          ; the real SEND
141cb8777  CALL 0x1406ed610          ; ~COutPacket on the normal path
```

The encode primitives were identified from their own bodies rather than assumed:
`1406ed520` = ctor (writes the opcode through `1406ed940`), `1406ed840` = 1 byte,
`1406ed940` = 2, `1406ed9d0` = 4, `1406edbc0` = 8, `1406edc80` = string, `1406ede20` = raw.
Each was read at its `ADD dword ptr [rbx+0x428],n`. [L]

## The body above is TRUNCATED, and the sentence above is how it happened

**Corrected 2026-09-09.** That list used to end `` `1406ed610` = send ``, inside a sentence
claiming all of them were read at an `ADD dword ptr [rbx+0x428],n`. **`1406ed610` has no such
instruction**, so the stated control cannot have covered it - it covered seven of the eight
things it named. `tools/dis_at.py 0x1406ed610` is eleven instructions: release `this+0x438`,
tail-jump releasing `this+0x408`. It is `~COutPacket`. The send is `1415d01c0`, and the
encoders are a positive control that the instrument discriminates - `1406ed9d0` shows
`add [rbx+0x428],4` and `1406ed840` shows `inc [rbx+0x428]`. [L]

**The cost was not cosmetic.** One buffer, `[rbp+0xd0]`, runs through the whole function, and
at `141cb8358` it forks. The non-zero arm destroys the packet and jumps to the epilogue - it
**sends nothing**. Under the old label that arm read as "SendPacket", so the transcript
followed the abort path and stopped there. On the arm that actually sends, **eleven further
encodes run first**: `141cb83e7, 8412, 84b7, 8548, 8557, 8571, 85d5, 85e9, 85fe, 860e, 8626`
- five `w_u8` and six `w_u32` - before `141cb8632`. [L]

So **the field list above is a prefix of `0x2FF`, not the whole of it.**

**But the trailing eleven are not unknown - they are decoded in the code, and have been all
along.** `crates/net/src/mobmove.rs` accounts for `0x02FF` as a head, the movement path and a
**tail**, and it did not get that from this file: it names `141cb85fe` - which is in the
post-fork run above - as `mob+0x960`, the controller level, and observes that every captured
body carries the `1` this server granted. Its layout closes exactly on **30 of 30** captured
bodies, and the later type-aware path walk closes on **860 656 of 860 656** deduplicated
archived bodies. A layout that lands to the byte on that many real packets is not resting on
this transcript. [L]

That is the shape of the defect, and it is worth naming: **the truncation was in the
write-up, not in the implementation.** This file is the stale copy of something the code
already had right - the inverse of *"built is not wired"*, and the same failure as
`loop_builders.py` holding a table its own comment said to keep in step. Read `mobmove.rs`
for the field list; this section is the disassembly landmark for it, nothing more.

**What survives unchanged:** everything below rests on the *first* `Encode4` and on a write
scan, both before the fork, so `mob+0x3a0` = object id is untouched by this.

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

## 6. Nothing has to be answered — **RETRACTED, see section 10**

> **This section is wrong, and section 10 is the replacement.** The scan it rests on was
> right; the *intersection* was made against "the eight mob-pool handlers" while the second
> jump table's 102 opcodes were still undiscovered — the same mistake section 2.2 retracts
> one level up, made twice in one file. `0x03E4` is a `MobCtrlAck` and it reads the move id
> back. The text below is kept unaltered because the scan in it is reusable and its exact
> wording is what section 10.1 dissects.

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
4. **Which mob the server should hand to which client.** Answered 2026-09-04: whoever hits a
   mob they do not hold is given it, because the flinch is local to whoever holds the grant.
   A change is a release to the old holder then a grant to the new one, **in that order**.
   `research/mob-hit-reaction.md`.
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

---

# THE 2026-08-19 RUN: the grant works, the mobs move once, and section 6 was wrong

Written after the owner's runs of 2026-08-19. Everything above this line stands - `0x03D2` is the
packet that starts a mob, and it was confirmed on screen. What follows is what the runs added,
and the one retraction they forced.

> **`world.log` is a LIVE capture and it rolled while this was being written.** The `01:21:37`
> session analysed below is preserved as
> `research/fixtures/mobs-move-once-then-freeze-0x02ff-bodies.txt`; the file in the repo root
> now holds the `02:06:47` session. **That is a gift rather than a nuisance** - the two are
> independent runs and section 10 holds on both. Nothing here rests on a file that has since
> changed.

## 10. What the runs measured, twice

**The grant works.** Thirty `0x03D2` went out at `01:21:36.919`-`.924`, one per mob on map 40,
level `1`, 87 bytes each. The client accepted every one and began simulating. **[L]**

**The mobs moved exactly once each and stopped.** Thirty `0x02FF` arrived at
`01:21:37.122`-`.125` - 198 ms after the grants, and **all thirty inside three
milliseconds**. Nothing after that for the rest of the session. The owner, on screen: *"the mobs
moved for half a second before freezing again."* **[L]**

**Every one of the thirty carried `moveId = 1`.** The client's counter is `deobf(mob+0x2f0)`
and the builder sends `that + 1` (`141cb7ee0 LEA EDI,[RAX+1]`), so `1` means the counter never
left `0`. **No mob ever reported a second step.** **[L]**

That shape rules something out immediately. Thirty mobs firing in the same 3 ms window and
then all falling silent is not thirty independent timers expiring; it is one condition opening
for all of them at once and closing for all of them at once. **[D]**

**And it reproduced, on a second session nobody set up as a control.** `world.log` rolled to a
later run while this analysis was in progress, and that run is the same measurement again:
30 grants at `02:06:47.242`-`.248`, 30 reports at `02:06:47.417`-`.420` (175 ms later, all 30
inside 3 ms), **every one `moveId = 1`**, nothing after. Two sessions, 60 bodies, identical
behaviour. **[L]**

### 10.1 RETRACTION: `0x02FF` has an acknowledgement, and it is `0x03E4`

Section 6 says:

> *"A range scan over the mob address space for `+0x2f4` returns 20 sites; **not one of them
> is inside any of the eight mob-pool packet handlers**."*

Re-run today, the identical scan over `0x141c40000..0x141d60000` returns the identical **20
sites** (`tools/rangescan.py`; its positive controls `141cb7ef3` and the constructor's
`+0x960` zero both fired). **The scan was never the problem.** One of those twenty is:

```text
141c821f8  MOV EDX,dword ptr [RDI + 0x2f4]     in FUN_141c82060
```

and `tools/callers.py 0x141c82060` returns **exactly one** call site, `141d32ba3` - the stub
for **`case 0x3E4`** in the second dispatcher. `FUN_141c82060` *is* a mob-pool packet handler.
**[L]**

**The intersection was made against a set of eight.** Section 2.2 had already established that
the pool has 110 handlers rather than eight, and section 6 was written against the old list
anyway. This is `CLAUDE.md`'s "enumerate before you filter", failed twice in one document -
the second time immediately after retracting the first.

The general form is worth keeping: **a negative built from `scan` intersected with a
hand-maintained set fails silently when the set grows, and it keeps returning the same
confident answer.** The scan is not the instrument; the *set* is. Re-derive the set inside the
same script that does the intersection - `tools/poolscan.py` reads both jump tables out
of the image on every run for exactly that reason, and prints the handler count in its header
so a shrunken set is visible on sight.

### 10.2 `0x03E4` decoded - `MobCtrlAck`, 26 bytes, no conditional read

`FUN_141c82060(mob, packet)`. The dispatcher `FUN_141d32b30` reads the object id itself at
`141d32b4d` and looks the mob up; **an unknown id returns silently** (`141d32b62 JE` to the
epilogue), so a stale id costs nothing. **[L]**

The handler's only branch before its reads is at `141c82082`, and both arms converge at
`141c82092`. A disassembly of `141c82092..141c82121` contains **no jump of any kind** - all
eight reads are straight-line, so the body is a fixed **26 bytes**. **[L]**

| off | size | read at | what |
|---:|---|---|---|
| 0 | u32 | `141d32b4d` | object id, read by the dispatcher |
| 4 | u16 | `141c8209e` | **the move id being acknowledged** |
| 6 | u8 | `141c820ab` | non-zero makes the resulting state `4` instead of `3` |
| 7 | u32 | `141c820b7` | obfuscated into `mob+0x3b0`/`+0x3b4`/`+0x3b8`. **[I]** MP |
| 11 | u32 | `141c820f0` | skill id - `0` short-circuits the whole skill block |
| 15 | u16 | `141c820fa` | skill level; the two feed `FUN_14049a080(template, id, lvl, 0)` |
| 17 | u32 | `141c82105` | `0` skips the `FUN_140401b30` call at `141c821f3` |
| 21 | u32 | `141c82114` | **read and discarded** - `EAX` is clobbered by the next read |
| 25 | u8 | `141c8211c` | **read and discarded** |

Two things it does, and both matter.

**It compares our move id against the client's own counter, and sets the mob's state.** **[L]**

```asm
141c821f8  MOV   EDX,[RDI + 0x2f4]         ; the obfuscation KEY
141c821fe  LEA   RCX,[RDI + 0x2f0]         ; the obfuscated move id
141c82205  CALL  0x1401ab420               ; -> AX, the client's current move id
141c8220a  MOVSX ECX,AX
141c8220d  MOVSX EAX,word ptr [RSP + 0x60] ; the value WE sent
141c82212  CMP   EAX,ECX
141c82214  JNS   141c8221d                 ; ack >= current
141c82216  MOV   EBX,2                     ; ack <  current  -> state 2
141c8221b  JMP   141c82226
141c8221d  TEST  R13D,R13D                 ; the u8 at body offset 6
141c82220  SETNE BL
141c82223  ADD   EBX,3                     ; -> state 3, or 4 if that byte is set
141c82232  MOV   [RDI + 0x2e4],EAX         ; ... obfuscated across +0x2e4/+0x2e8/+0x2ec
```

> **A naming correction to sections 5 and 6.** The move id is **not** at `mob+0x2f4`. It is
> the obfuscated pair `mob+0x2f0` (value) / `mob+0x2f4` (key), and both the sender
> (`141cb7ecb`/`141cb7ed3`) and this handler (`141c821f8`/`141c821fe`) use it that way. The
> scan found `+0x2f4` because the key is what gets loaded into a register; the *field* is the
> pair. Same class of error as `mob+0x3a0` vs `mob+0x3a8` in section 5. **[L]**

**It re-runs the same activation `0x03D2` runs.** `141c8207d` calls `[vtable+0x48]` (slot 9);
if that returns 0, `141c8208f` calls `[vtable+0x40]` - **slot 8, `FUN_141c54200`** - with a
hardcoded `EDX = 1`. Section 4 called that "the one virtual call the spawn packet never makes"
and said only the change-controller handler makes it. **`0x03E4` makes it too, and it is the
only other packet that does.** **[L]**

Slot 8, re-read directly rather than from section 4's paraphrase, has one branch section 4 did
not record:

```asm
141c5423a  MOV  RCX,RDI
141c5423d  TEST EDX,EDX
141c5423f  JE   141c542b5          ; arg2 == 0 -> a different path entirely
141c54241  CALL 0x1409c5080        ; = FUN_1409d4840(anim + 0x108): is it running?
141c54248  JNE  141c5436e          ; ALREADY RUNNING -> do nothing at all
141c5424e  LEA  EDX,[RAX + 3]      ; RAX == 0 here, so 3
141c54254  CALL 0x141c4ff30        ; mob+0x2e4 := 3
141c54261  CALL 0x141c55750(mob, 1)
```

**"Do nothing if the animation is already running, otherwise start it" is the shape of a
pump**, not of an initialiser. **[D]**

### 10.3 The state field, and what is still NOT proven

`mob+0x2e4` (obfuscated across `+0x2e4`/`+0x2e8`/`+0x2ec`) has these writers in the mob code
range - `tools/rangescan.py 0x2e4 0x141c40000 0x141d60000`, 31 sites: **[L]**

| where | what it writes |
|---|---|
| `141c4d232` | the base constructor, `FUN_141c4cee0` |
| `141c4ff4b` | the generic setter `FUN_141c4ff30(mob, v)` |
| `141c52ba7`, `141c5373f` | inside `encodeInit` |
| `141c82232` | **`0x03E4`** - 2, 3 or 4 |
| `141cb8748` | **the move sender itself**, immediately before `SendPacket` |

That last one is the interesting one, and it is read here rather than assumed:

```asm
141cb8642  LEA  RSI,[R12 + 0x2e4]
141cb864a  ... de-obfuscate -> EDI
141cb86ba  CMP  EDI,-2 / JE 141cb8748      ; -2 -> keep EBX (which is 1)
141cb873e  CMP  EDI,1  / JE 141cb8748      ; 1  -> keep EBX
141cb8743  MOV  EBX,2                      ; anything else -> 2
141cb8748  ... obfuscate EBX into mob+0x2e4/+0x2e8/+0x2ec
141cb8777  CALL SendPacket
```

So: **the grant sets the state to 3; sending a move drops it to 1 or 2; the ack puts it back
to 3 (or 4).** That is a request/response cycle written into one field. **[D]**

> **What is NOT established, and this is the honest part.** No instruction has been found that
> *blocks* the second wander roll. Slot 19 (`FUN_141c8d1b0`, where the wander is rolled -
> section 5.1) reads the state at `141c8dcbe` and only requires `state == -2 || state > 0`,
> which both `1` and `2` satisfy. Its harder gates are `mob+0x4e4 == 0` at `141c8d1e7` and
> `FUN_1409c5080(anim) != 0` at `141c8d2bc` - the second being "the animation is running", the
> exact thing slot 8 restarts.
>
> The chain is therefore: **the ack is the only packet in this client that can restart a
> stopped mob animation, and a stopped animation is what slot 19 refuses to act on.** Every
> link in that sentence is **[L]**. That the animation actually stops after one wander is
> **[I]** - it is the missing measurement, and only a client run can supply it.
>
> Do not write "sending `0x03E4` makes mobs keep moving" as measured. Write "it is the only
> thing in the client that can", which is what the listing supports.

## 11. `0x02FF` decoded in full, and verified against sixty real bodies

The `01:21` capture is 30 bodies of **111, 132, 153 and 174** bytes: a clean 21-byte ladder
over element counts 1, 2, 3 and 4, i.e. `90 + 21n`. The `02:06` capture is 30 more, at 111,
132 and 153. **[L]**

Field order is `FUN_141cb6880`'s encode order, taken with `tools/encodes.py` - a mirror
of `tools/reads.py` that reuses its loader and `calls_of`, so it inherits that tool's
positive control. Its own control is in its docstring and fires: the `0x2ff` `COutPacket`
ctor at `141cb7eb1` and `SendPacket` at `141cb8365`, both hand-read in section 5.

> **A correction to section 5 found by that control.** `FUN_141cb6880` has **two**
> `SendPacket` calls. `141cb8365` is an *early* send taken when `FUN_141d57c60` sets its
> out-param at `[rbp-0x80]`; `141cb8777` is the normal one, and it is the one the captured
> bodies came from. Section 5's "exactly one `COutPacket` construction" is correct; "one
> send" would not have been, and a decoder written against the early branch would be 29 bytes
> short.

| off | size | encoded at | what |
|---:|---|---|---|
| 0 | u32 | `141cb7ec6` | object id, `mob+0x3a0` |
| 4 | u16 | `141cb7f05` | **move id**, `deobf(mob+0x2f0) + 1` |
| 6 | u8 | `141cb7f20` | packed, `((a<<2)\|b)<<2\|c` - **`0` in all 30** |
| 7 | u8 | `141cb7f31` | **`0xFF` in all 30** |
| 8 | u64 | `141cb7f44` | **`0` in all 30** |
| 16 | u8 | `141cb7f55` | |
| 17 | u8 | `141cb7f65` | |
| 18 | u8 + n x (u16,u16) | `141cb7f80`/`91`, `141cb7ff2`, `141cb803a` | list from `mob+0x8c8` - **empty in all 30** |
| .. | u8 + n x u16 | `141cb8063`/`71`, `141cb80ca` | list from `mob+0x8d0` - **empty in all 30** |
| .. | u32 | `141cb80e9` | `mob+0x10b0`; if non-zero, **11 more u32** follow |
| .. | u8 | `141cb820e` | |
| 25 | u32 | `141cb8231` | `1` in all 30 |
| **29** | u32 | `141cb828c` | **`0x00ffddcc`** |
| **33** | u32 | `141cb829a` | **`0x00ffddcc`** |
| 37 | u32 | `141cb82b4` | `0x3cd98750` in all 30 |
| 41 | u32 | `141cb82d9` | |
| 45 | u8 | `141cb82f0` | |
| **46** | | `141cb8353` -> `FUN_141d57c60` | **the movement path** |
| .. | u8 + ceil(n/2) | `141d580e5`, `141d58182` | a nibble list - **only `0x02FF` carries it** |
| .. | 29 bytes | `141cb83e7` .. `141cb8626` | the tail |

The path itself, from `FUN_1404b2630` - the *reader* that `FUN_141d598b0` calls; the encoder
side is `FUN_1404b2000`, its immediate neighbour, with the mirrored `u32 u16 u8` signature:

| off | size | read at | what |
|---:|---|---|---|
| 0 | u32 | `1404b2650` | -> `this+0x40` |
| 4 | i16 | `1404b265b` | **x**, stored obfuscated at `this+0x20`, key `+0x24` |
| 6 | i16 | `1404b2675` | **y**, `this+0x28`, key `+0x2c` |
| 8 | u16 | `1404b2690` | |
| 10 | u16 | `1404b269c` | |
| 12 | i16 | `1404b26a9` | **element count**, signed (`MOVSX / TEST / JLE` bails) |
| 14 | 21 x n | | the elements |

### 11.1 Four independent checks, because a byte layout that merely adds up is a coincidence

1. **It adds up sixty times out of sixty, across two sessions.** Head + path + tail lands
   exactly on the body length for every captured body, at four different element counts. A
   wrong width anywhere before the path would break all of them at once.
   `python tools/decode_mobmove.py world.log` re-runs it and exits non-zero if it ever stops
   being 100%.
2. **A literal from the encoder turns up where the layout predicts it.** `141cb827e` is
   `MOV EBX,0xffddcc`, encoded at `141cb828c` and `141cb829a`; body offsets 29 and 33 are
   `cc dd ff 00`, twice.
3. **The tail echoes our own grant back.** `141cb85fe` encodes `mob+0x960`, the controller
   level. Every captured body carries **`1`** there - the value this server sent. The client
   is telling us, in its own words, that the grant landed.
4. **The coordinates match a capture nobody here made.** Section 0 records the combat agent
   decoding `0x00DF` on the same map with mob 2000 at **`(424, 395)`**. This decoder reads mob
   2000's `0x02FF` path head as **`(424, 395)`**. Two packets, two agents, two methods, same
   pixel.

`crates/net/src/mobmove.rs` carries checks 1-4 as unit tests over two of the real bodies.

## 12. `0x03D9` decoded - and it is NOT a verbatim relay

`FUN_141c813b0`, 3233 bytes, 23 read sites (`tools/reads.py 0x141c813b0 3`).

| off | size | read at | what |
|---:|---|---|---|
| 0 | u32 | `141d32b4d` | object id, read by the dispatcher |
| 4 | u8 | `141c813d0` | bit 0 -> `BL`, bit 2 -> `R13B`. **This is `0x02FF` offset 6** |
| 5 | u8 | `141c813f1` | `action*2 + facing` (`141c81766 SHR EBP,1`); **`0xFF` = none** (`141c8176d CMP AL,0xFF / JE`). **This is `0x02FF` offset 7** |
| 6 | u64 | `141c814c1` | |
| 14 | u8 + n x (u16,u16) | `141c814f4`, `141c81566`, `141c815a8` | -> `mob+0x8c8`, the same list `0x02FF` sends |
| .. | u8 + n x u16 | `141c815fe`, `141c81614` | -> `mob+0x8d0`, likewise |
| .. | u32 | `141c81635` | -> `mob+0x10b0`; if non-zero, **11 more u32** into `mob+0x10cc`, `+0x10c4`, `+0x10c8` and eight floats from `+0x10d0` |
| .. | u32 | `141c8173e` | outside that gate - `141c81648 JE` lands just before it |
| .. | | `141c82009` -> `FUN_141d598b0` | **the path** |
| .. | u8 | `141c82011` | non-zero reaches `[vtable]` and `mob+0x988`; send `0` |

The two count-prefixed lists and the gate-plus-eleven block are **the same blocks writing the
same fields** as `0x02FF` sends. The differences: no move id, no pair of `u8`s at `0x02FF`
offsets 16/17, and one `u32` where `0x02FF` has `u8 + 5 x u32 + u8`. **[L]**

**The one-byte trap.** `FUN_141d598b0`'s third argument gates the nibble block at
`141d5991a TEST EBX,EBX / JE`, and the two call sites pass different values:

```text
0x02FF   141cb8346  MOV  R8D,R13D     -> the nibble count byte IS written
0x03D9   141c82000  XOR  R8D,R8D      -> it is NOT read
```

So a rebroadcast that copies `0x02FF`'s path *including* that byte is one byte long, every
time. This is the same failure mode as the tail-`jmp` bug in `CLAUDE.md` - a length that is
right in one direction and wrong in the other. `MobMoveRequest::path` stops before it, and a
unit test pins that.

`141c81784 MOV dword ptr [R14+0xcd0],0` - the handler **clears** the field that blocks the
move sender (section 5.2). Consistent with `0x03D9` being for a client that does *not* control
the mob: it is told where the mob went, and its own sender is left unblocked.

Minimum body: **`25 + 14 + 21n`** - 60 bytes for a one-element path.

### 12.1 What `0x03D9` is for, and what it is not for

The owner: *"the server needs all of the clients to see the same mob movement."* That is exactly
right, and `0x03D9` is exactly that packet. **It is also not what froze anything on
2026-08-19**, because there was one player on the field and nobody to send it to. The two
things are independent and should not be conflated:

* `0x03E4` -> **the controller**, so it keeps simulating. One player needs this.
* `0x03D9` -> **everyone else**, so they see the same mob. Two players need this.

**Never send `0x03D9` to the client that sent the `0x02FF`.** `FUN_141c813b0` overwrites the
mob's position, animation and `mob+0xcd0` from the packet - state the controller owns. **[I]**
as to what that looks like on screen; the overwrite itself is **[L]**.

## 13. Nothing here needs a timer

Both new packets are **reactive**: one `0x03E4` per inbound `0x02FF`, and one `0x03D9` per
inbound `0x02FF` per *other* client on the field. No periodic send is required by anything
read here, and none should be added speculatively - the client is the clock, and it is the
thing that decides when a mob has finished a step.

**Re-granting was the one open scheduling question and it is closed.** Control rotates on a
hit: `0x03D2` level `0` to the old holder, then level `1` to the attacker. Level `0` releases
- it does not delete the mob, which is what this section assumed until 2026-09-04. See
`crates/net/src/mobmove.rs::CONTROL_RELEASE`.

## 14. Instruments added, with their controls

| tool | what it does | control that fired |
|---|---|---|
| `tools/encodes.py` | `tools/reads.py` for the **encode** side; same loader, same `calls_of`, same tail-`jmp` handling | `0x141cb6880` depth 1 must show the ctor at `141cb7eb1` and a send at `141cb8365`, both hand-read in section 5 |
| `tools/mobpool_tables.py` | reads the second jump table (`0x141d33448`, 117 entries) out of the image and names all 102 live handlers | cross-checked with `tools/callers.py`: `0x141c82060` has exactly one caller and it is that table's `0x3E4` stub |
| `tools/poolscan.py` | scans **both** tables' handler subtrees for a `[reg+DISP]` operand, and **re-derives the handler set on every run** | `0x960` must name `141d34ca0 MOV [RDI+0x960],AL` inside `FUN_141d34a70`; section 4.2 read it by hand |
| `tools/rangescan.py` | bounded whole-range operand scan, disassembling and resyncing | `0x2f4` must return `141cb7ef3`; it returns 20 sites, the same 20 section 6 got |
| `tools/decode_mobmove.py` | decodes every captured `0x02FF` and **asserts the total length** | 30/30 exact |

Two notes worth carrying beyond this file:

* **`capstone` with `detail = True` over the whole 52 MB `.text` takes 17 minutes**, and it
  answers a question nobody asked: `+0x2f4` exists on dozens of unrelated classes, because a
  displacement is a class fact (section 9). The bounded version - `rangescan.py` over the mob
  module, `poolscan.py` over a handler subtree - takes seconds and answers the real question.
  Bound it, and say in the write-up what you bounded it to.
* **A tool that intersects a scan with a hand-maintained set of addresses keeps giving the
  same answer after the set goes stale.** Put the set's derivation *inside* the tool and make
  it print the set's size. That one line is what would have caught section 6.
