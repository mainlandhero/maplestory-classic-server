# `0x02A5` decoded, and the field that makes it visible is one the client never sends

2026-09-03. No client run was spent on this. Everything below is capstone over
`client-patched\MapleStory.exe` through the repo's own tools, plus **331 event-deduplicated
`0x00E5` bodies** out of `previous-runs/` and `research/fixtures/`. Ghidra was not used - the
project lock is held elsewhere.

Tags: **[L]** read off a listing, a table or a capture; **[D]** derived from two or more of
those; **[I]** inferred or from the v214 reference tree.

---

## 0. The headline, and it contradicts the doc block in `crates/net/src/userpool.rs`

`userpool.rs` currently says of `0x02A5`:

> *"There is no builder because there is nothing to build: the 147 bytes are the client's,
> and the server's job is to pass them on."*

**That is wrong, and it is wrong in the direction that produces a silent nothing on screen.**

The handler draws the remote damage number from HITINFO **`+0xa8`, body offset +143**, and
gates the remote flinch animation on the same field being `> 0`. **The client's own `0x00E5`
carries `0` there in 331 of 331 captured bodies**, and its own builder never writes the field
at all. So a straight echo of the client's 147 bytes makes every other client call the damage
renderer with the amount `0` - which is the MISS path - and play no animation.

**`+0xa8` is a server-fill field.** Set it to the damage the server decided, echo the rest,
and both halves of the owner's sentence follow: the number and the flinch.

| what the owner asked for | where it comes from |
|---|---|
| *"displaying the damage that the client is taking"* | `FUN_142771360(user, -HITINFO[+0xa8], 0, ...)` at `1429d4ed2` - the same renderer, digit set 3 (`NoViolet`), that `research/damage-number-draw.md` §2 documents. **[L]** |
| *"the blinking expression when they are taking damage"* | `FUN_14282d710(user, 1, 1500, 0)` at `1429d4e8e` and `[avatar_vtbl+0xd8](avatar, 0, 5000)` at `1429d4ea5`. **Gated on `HITINFO[+0xa8] > 0`.** **[L]** |

**The blink is driven by this packet, not by an HP change.** §5 settles that: the `0x02A5`
handler makes the *same four calls in the same order* as the client's own hit path, and none
of them reads HP.

---

## 1. Instruments, and the control each one passed

| instrument | positive control | result |
|---|---|---|
| `tools/reads.py` | `0x140304100 2` must show `raw@140304138`, `u8@140304144`, `u8@140304183`, then a `u16` run, through **both** helpers and direct | reproduced exactly |
| `tools/listing.py` | same target, same three | reproduced |
| the remote jump-table decode (§2) | index **17** must give `0x02AF`, which `research/level-up.md` §6 established independently, and its arm must call `FUN_1427863f0` | reproduced - `0x1429bba36 -> call 0x1427863f0` |
| the second jump-table decode (§3) | `0x02A5` must come out at index `0x12` -> `0x1429bbc08` *default*, which `research/user-pool-tables.md` line 867 recorded independently | reproduced |
| the capture decoder (§4) | attack index at body+4 must be **`-1`** - the value `research/user-hit.md` §3.2 derived from the listing, with no log involved | **331 of 331** |

That last control is the one the field values rest on: it proves the 147-byte alignment
before any number is read out of it. A decoder at the wrong offset still prints numbers.

**Event-deduplicated, not file-deduplicated**, per `CLAUDE.md`: 530 log files scanned,
`(timestamp, opcode, body)` as the key, **331 distinct `0x00E5` events**, all exactly 147
bytes.

### 1.1 One instrument artefact, recorded because it looks like a negative

`python tools/reads.py 0x1429bb720 2` does **not** list the call at `0x1429bb9b8` - our stub's
call into `FUN_1429d48c0` - while it lists eighteen of its siblings. That is a depth artefact,
not a finding: the chain is `FUN_1429d48c0 -> FUN_14025d750 -> FUN_14025da80 -> primitive`,
which needs depth 3, and the siblings reach a primitive in two hops. **A read walk of this
router at depth 2 reports `0x02A5` as reading nothing.** Read the listing.

`python tools/reads.py 0x1429bb9b2 3` returns *"has no `.pdata` entry - not a function
start?"*, because the stub is an interior jump-table arm rather than a function. Neither of
those is evidence about the packet.

---

## 2. The route, re-derived rather than trusted

`FUN_1429bb720(pool /*rcx*/, opcode /*edx*/, packet /*r8*/)`:

```asm
1429bb73a  mov  rdi, r8               ; the packet, preserved across the whole function
1429bb73d  mov  esi, edx              ; the opcode
1429bb73f  mov  rbx, rcx
1429bb742  mov  rcx, r8
1429bb745  call 0x1406e8c20           ; <<< READ u32  -> ebp = charId
1429bb74c  mov  r8, [rbx + 0xf8]      ; the pool's hash bucket array
...        hash by charId, walk the chain, compare [rcx+0x10] == charId
1429bb796  mov  rbx, [rcx + 0x20]     ; rbx becomes the CUser
...
1429bb940  lea  eax, [rsi - 0x29e]
1429bb946  cmp  eax, 0x26
1429bb949  ja   1429bbb10
1429bb951  mov  ecx, [r14 + rax*4 + 0x29bbc34]   ; r14 = image base
1429bb959  add  rcx, r14
1429bb95c  jmp  rcx
```

Table dumped in full (39 entries, `0x029E..0x02C4`). **Index 7 = `0x02A5` -> `0x1429bb9b2`**:

```asm
1429bb9b2  mov  rdx, rdi              ; the packet
1429bb9b5  mov  rcx, rbx              ; the CUser found by charId
1429bb9b8  call 0x1429d48c0
```

**[L]** So the calling convention is `FUN_1429d48c0(CUser *user, CInPacket *packet)`, and the
`u32 charId` prefix is read by the *router*, once, before the dispatch - exactly as
`research/level-up.md` §6 said.

**A charId that is not in the receiver's pool is a clean no-op.** Every failed lookup jumps to
`0x1429bbc19`, which is the function epilogue - four register restores and `ret`. No log, no
fault, no partial read. **[L]** That is what makes broadcasting this safe.

---

## 3. Enumerate before you filter: what *else* touches the packet

Three separate ways a consumer could hide, all checked.

### 3.1 The wrapper keeps no packet pointer **[L]**

`FUN_1429d48c0`'s prologue, in full, up to the decode call:

```asm
1429d48c0  mov  [rsp+0x18], rbx       ; rbx spilled into the r8 home slot
1429d48c5  push rbp/rsi/rdi/r12/r13/r14/r15
1429d48d0  lea  rbp, [rsp-0xb0]
1429d48d8  sub  rsp, 0x210
1429d48df  movaps [rsp+0x200], xmm6
1429d48e7  ...  stack cookie
1429d48f8  mov  r14, rcx              ; arg1, the CUser
1429d48fb  lea  rcx, [rbp-0x30]       ; the HITINFO
1429d48ff  call 0x14025d750
```

**`rdx` - the packet - is never spilled and never moved to a callee-saved register.** After
`1429d48ff` it is unrecoverable, so the function is structurally incapable of a second read.
All 1615 bytes of it read the decoded struct at `rbp-0x30`, not the stream.

### 3.2 There is a second dispatch after the handler returns, and `0x02A5` is not in it **[L]**

This is the shape that hid `0x00D5` for three sessions, so it was enumerated rather than
eyeballed. The stub's tail is:

```asm
1429bb9bd  add  esi, 0xfffffd6d       ; esi = opcode - 0x293
1429bb9c3  jmp  1429bbb1f
1429bbb1f  movsxd rax, esi
1429bbb22  movzx  eax, byte [r14 + rax + 0x29bbd10]     ; byte index table
1429bbb2b  mov    ecx, dword [r14 + rax*4 + 0x29bbcd0]  ; target table
1429bbb36  jmp    rcx
```

A **second, byte-indexed jump table** on the same opcode, and `rdi` still holds the packet.
Dumped in full, `0x0293..0x02C3`: **`0x02A5` is index `0x12`, byte `15`, target
`0x1429bbc08`** - the default arm, shared by 22 of the 49 entries. That arm is:

```asm
1429bbc08  mov  rcx, [rip+0x110cb91]  ; -> 0x143AC87A0, a singleton
1429bbc12  je   1429bbc19             ; null -> straight to the epilogue
1429bbc14  call 0x1415f0d70
```

`python tools/reads.py 0x1415f0d70 3` -> **no reads**. **[L]** So nothing after
`FUN_1429d48c0` consumes the packet. This reproduces `research/user-pool-tables.md` line 867
from a fresh dump.

### 3.3 The decoder has no indirect dispatch and no branches at all **[L]**

`FUN_14025da80`, 614 bytes: **zero `j*` instructions of any kind**, zero indirect calls, 44
direct calls to read primitives, then `ret`. `tools/reads.py` marks not one read `gated?`.

**Therefore the body is unconditionally 147 bytes. There are no optional fields, no gated
fields, no length prefix and no legal "absent" form.** That is established from the *decoder*,
which is the side that matters for a builder - `research/user-hit.md` §2 established the same
property from the *encoder*, and the two are independent code.

---

## 4. The body, field by field, with a read address for every field

Read directly off `FUN_14025da80`'s listing: each read's address, the primitive it calls, and
the store it feeds. **This is not transferred from `research/user-hit.md`'s encoder table** -
it is the decoder's own map, and it agrees with that table on all 44 offsets, which is real
corroboration from different code rather than a restatement.

Wire prefix: **`u32 charId`**, read at `0x1429bb745` by the router. Then:

| body | width | read addr | struct | used by the handler? | measured, 331 events |
|---:|---|---|---|---|---|
| **-4** | u32 | `1429bb745` | *(router)* | **yes - the pool lookup** | *(server-supplied)* |
| 0 | u32 | `14025da93` | `+0x00` | no | 0 |
| **4** | u32 | `14025da9d` | `+0x04` | **yes - `edi`, the attack index; routes everything** | **-1 in 331/331** |
| 8 | u32 | `14025daa8` | `+0x08` | **no** | 1×259, 0×49, 3,5,7,9,10,12,15,200 |
| 12 | u32 | `14025dab3` | `+0x10` | no | client PRNG |
| 16 | u32 | `14025dabe` | `+0x14` | no | ms tick |
| 20 | u32 | `14025dac9` | `+0x18` | no | 0 |
| 24 | u8 | `14025dad4` | `+0x1c` | **yes - arg5 of the damage renderer**, §5 | 0 |
| 25 | u32 | `14025dae4` | `+0x20` | no | 0 |
| 29 | u8 | `14025daef` | `+0x24` | no | 0 |
| 30 | u32 | `14025daff` | `+0x0c` | no | 1×273, 2×11, … |
| 34 | **raw 4** | `14025db15` | `+0x28` | no | 1×282, 2×49 |
| 38 | u32 | `14025db24` | `+0x30` | no | == body+30 always |
| 42 | u32 | `14025db2f` | `+0x2c` | no | **== body+8 always** |
| **46** | u32 | `14025db3a` | `+0x34` | **yes** - mob-pool lookup for the §6 block | mob object id |
| **50** | u32 | `14025db45` | `+0x38` | **yes** - mob-pool lookup | mob object id |
| 54 | u8 | `14025db50` | `+0x3c` | yes -> `r13d`, **but zeroed again** when attack index < 0 | 1×192, 0×139 |
| 55 | u8 | `14025db60` | `+0x3d` | **yes - arg4 of the renderer on the damage<=0 arm** | 0 |
| 56 | u8 | `14025db70` | `+0x3e` | no | 0 |
| 57 | u8 | `14025db80` | `+0x44` | no | 0 |
| **58** | u8 | `14025db90` | `+0x3f` | **yes - gates the §7 sound/effect block** | 0 |
| 59 | u8 | `14025dba0` | `+0x48` | yes - inside the 16-byte block, §6 | 0 |
| 60 | u8 | `14025dbb0` | `+0x49` | yes - same block | 0 |
| **61** | **u64** | `14025dbc0` | `+0x50` | **yes - `> 0` gates the whole §6 block** | **0 in 331/331** |
| 69 | u32 | `14025dbcc` | `+0x58` | yes - §6 argument | 0 |
| 73 | u32 | `14025dbd7` | `+0x5c` | no - the player's X, **the remote client uses the pool's position instead** | player X |
| 77 | u32 | `14025dbe2` | `+0x60` | no - player Y | player Y |
| 81 | u32 | `14025dbed` | `+0x64` | no | 0 |
| 85 | u32 | `14025dbf8` | `+0x68` | no | 0 |
| 89 | u32 | `14025dc03` | `+0x6c` | no | 0 |
| 93 | u32 | `14025dc0e` | `+0x70` | no | -1 |
| 97 | u32 | `14025dc19` | `+0x74` | no | 0/1 |
| 101 | u32 | `14025dc24` | `+0x78` | no | -1 |
| 105 | u32 | `14025dc2f` | `+0x7c` | yes - **only on the attack-index `<= -2` path** | 0 |
| 109 | u32 | `14025dc3a` | `+0x80` | yes - same path only | 0 |
| **113** | u32 | `14025dc48` | `+0x84` | **yes** - `FUN_140495990(templateId)` | mob template id |
| 117 | u32 | `14025dc56` | `+0x88` | no | 0 |
| 121 | u32 | `14025dc64` | `+0x8c` | no | 0 |
| 125 | u32 | `14025dc72` | `+0x90` | yes - §6 argument | 0 |
| 129 | u32 | `14025dc80` | `+0x94` | no | 0 |
| 133 | u8 | `14025dc8e` | `+0x98` | no | **1 in 331/331** |
| 134 | u32 | `14025dca1` | `+0x9c` | yes - `<= -2` path only | 0 |
| 138 | u8 | `14025dcaf` | `+0xa0` | yes - `<= -2` path only | 0 |
| 139 | u32 | `14025dcc2` | `+0xa4` | no | 0 |
| **143** | **u32 / i32** | `14025dcd0` | **`+0xa8`** | **YES - THE DAMAGE. §5** | **0 in 331/331** |

`4 + 147 = 151` body bytes. The widths sum to exactly 147, and body+143+4 = 147, so `+0xa8`
is the **last** field.

Two notes on the table:

* Every `u8` is read through `0x1406e8ae0` and stored via `test al,al / setne` - they are
  **booleans**, not small integers. Any non-zero byte becomes `1`. **[L]**
* Body+34 is a **4-byte raw read**: `mov r8d, 4 / lea rdx,[rsp+0x30] / call 0x1406e9170`,
  and only `dword [rsp+0x30]` is consumed. Four bytes, not eight. **[L]**
* Struct `+0x40` is read by the handler (`[rbp+0x10]` at `1429d4c72`) and is **not on the
  wire** - the ctor zeroes `+0x3c..+0x43` as one qword and no read targets it. It is always
  `0` for a packet-decoded HITINFO. This is the one place where "the offset table" and "the
  fields you can influence" differ, and it is the base-frame trap from
  `research/user-enter-field.md` §2.4 in miniature: the handler's frame base is `rbp-0x30`,
  not `rbp`.

---

## 5. `+0xa8` is the damage, and the client never fills it

### 5.1 The tail of `FUN_1429d48c0`, which every path reaches **[L]**

```asm
>1429d4e64  mov   esi, [rbp-0x80]        ; = HITINFO +0x1c, captured at 1429d4907
>1429d4e67  mov   ebx, [rbp+0x78]        ; = HITINFO +0xa8   <-- body offset +143
 1429d4e6a  mov   rcx, r14               ; the remote CUser
 1429d4e6d  test  ebx, ebx
 1429d4e6f  jle   1429d4ebc              ; <= 0  -> no animation
 1429d4e71  mov   edx, 0xffffffff
 1429d4e76  call  0x14282d7c0            ; is a temporary action already latched?
 1429d4e7b  test  eax, eax
 1429d4e7d  jne   1429d4e93
 1429d4e7f  xor   r9d, r9d
 1429d4e82  lea   edx, [rax+1]           ; action id 1
 1429d4e85  mov   r8d, 0x5dc             ; 1500 ms
 1429d4e8b  mov   rcx, r14
 1429d4e8e  call  0x14282d710            ; tail-jmps to FUN_140f832e0(avatar, 1, 1500)  THE FLINCH
>1429d4e93  lea   rcx, [r14+0x100]       ; the avatar
 1429d4e9a  mov   rax, [rcx]
 1429d4e9d  xor   edx, edx
 1429d4e9f  mov   r8d, 0x1388            ; 5000 ms
 1429d4ea5  call  qword ptr [rax+0xd8]   ; the avatar effect                THE BLINK
 1429d4eab  mov   [rsp+0x28], r13d       ; arg6 = 0
 1429d4eb0  mov   [rsp+0x20], esi        ; arg5 = HITINFO +0x1c
 1429d4eb4  xor   r9d, r9d               ; arg4 = 0
 1429d4eb7  mov   rcx, r14
 1429d4eba  jmp   1429d4ecb
>1429d4ebc  movzx r9d, byte [rbp+0xd]    ; arg4 = HITINFO +0x3d
 1429d4ec1  mov   [rsp+0x28], r13d       ; arg6 = 0
 1429d4ec6  mov   [rsp+0x20], r13d       ; arg5 = 0
>1429d4ecb  neg   ebx                    ; <<< the value is NEGATED
 1429d4ecd  xor   r8d, r8d               ; nOffsetIndex = 0
 1429d4ed0  mov   edx, ebx
 1429d4ed2  call  0x142771360            ;                                 THE NUMBER
```

`FUN_142771360` is the renderer `research/damage-number-draw.md` §2 documents:
`FUN_142771360(CUser*, i32 nAmount, i32 nOffsetIndex, i32, i32, i32)`, and *"the sign is the
whole mechanism"* - negative selects digit set **3**, `NoViolet0`/`NoViolet1`, the damage
colour. **So `+0xa8` must be sent POSITIVE; the client negates it for us.** **[L]**

Three consequences a builder has to know:

* **`+0xa8 = 0` draws the MISS number and plays nothing.** `142771568 jns 14277166a ; exactly
  0 goes elsewhere - the MISS path`, quoted from `damage-number-draw.md` §2.1. That is
  arguably correct behaviour for a 0-damage hit, but it is not "nothing happens".
* **A negative `+0xa8` draws a BLUE number.** `neg` turns it positive, and positive is digit
  set 2, `NoBlue` - the recovery colour. Do not send one.
* The animation is gated `> 0` with a **signed** compare (`jle`).

### 5.2 The client's own builder never writes `+0xa8` **[L]**

`FUN_1428aa0a0` is the builder that produced all 331 captured bodies
(`research/user-hit.md` §3.1). Its HITINFO base is `rbp+0x240`, verified from the encode call
itself at `1428ad6fe: lea rcx,[rbp+0x240] / call 0x14025d810`. Grepping its full listing for
`rbp + 0x2e8` (= `+0xa8`) returns **zero writes**. The ctor `FUN_14025d6a0` zeroes the field,
and nothing overwrites it.

Measured agreement: **body+143 is `0` in 331 of 331 events.** Same for `+0x50` (body+61).

**[D]** and it is the only reading that fits: `+0xa8` exists so the *server* can put a number
there. The client has no use for it - on its own screen it draws its own number from a local
variable at `1428aca14`, not from the struct.

---

## 6. `+0x50` gates a second block, and the local client skips it too **[L]/[D]**

```asm
>1429d4b04  movups xmm6, [rbp+0x18]      ; HITINFO +0x48..+0x57, 16 bytes
 1429d4b08  movdqa xmm0, xmm6
 1429d4b0c  psrldq xmm0, 8               ; the high qword = +0x50
 1429d4b16  test   rax, rax
 1429d4b19  jle    1429d4c4e             ; <= 0 -> skip the whole block
 1429d4b1f  mov    edx, [rbp+4]          ; +0x34, the mob object id
 1429d4b22  call   0x141d2efc0           ; mob pool lookup; null -> skip
 ...
 1429d4c49  call   0x141c5d1c0           ; a method ON THE MOB, 20+ arguments
```

The local hit path does the **identical** thing, on the same function, gated on the same
value - the local variable that becomes `+0x50`:

```asm
 1428ab7a5  cmp  qword ptr [rbp+0x18], 0
 1428ab7aa  jle  1428ab949               ; the same skip
 ...
 1428ab940  call 0x141c5d1c0             ; the same call, the same argument shape
 ...
 1428ad650  movups [rbp+0x288], xmm7     ; and [rbp+0x10..0x1f] -> HITINFO +0x48..+0x57
```

`[rbp+0x18]` is `0` in every capture (proved three ways: the value itself, and the two fields
`+0x3e` and `+0x44` that the encoder writes through `cmovg` on it at `1428ad661` are `0` in
331/331 as well). **So the client's own screen never ran `FUN_141c5d1c0` either.**

**Recommendation: leave `+0x50 = 0`.** Setting it enables a block whose function I have not
identified, to gain something the hurt player's own client did not show. Echoing `0`
reproduces exactly what the local client did, which is the definition of what the other
clients should see.

**Named blind spot: I did not identify `FUN_141c5d1c0`.** I cannot say what setting `+0x50`
would add, only that not setting it loses nothing relative to the local screen.

---

## 7. `+0x3f` gates a third block - also `0` in every capture **[L]**

```asm
>1429d4c4e  cmp   byte [rbp+0xf], 0      ; HITINFO +0x3f, body offset +58
 1429d4c52  je    1429d4e64
 1429d4c58  call  qword ptr [rax+0xf0]   ; two vtable calls on the CUser
 1429d4c6c  call  qword ptr [rdx+0xe8]
 1429d4c72  mov   r8d, [rbp+0x10]        ; +0x40 - NOT on the wire, always 0
 1429d4c78  call  0x1407e3ab0
 ...
 1429d4ccc  call  0x1427e0db0            ; user, ..., 1000 ms
```

The local path makes the same `FUN_1427e0db0(user, ..., 0x3e8, 0)` call at `1428ad5bd`,
so **[I]** this is the hit sound/effect. `0` in 331/331 captures; leave it `0`.

---

## 8. The full branch trace for the packet I am recommending

Body = `u32 charId` + the client's own 147 bytes with `+0xa8` overwritten. With the measured
field values (`+0x04 = -1`, `+0x50 = 0`, `+0x3f = 0`), every branch in `FUN_1429d48c0`
resolves:

```
1429d4915  cmp edi,-1 / jl      -1 is not < -1          -> fall through
1429d491e  r13d = +0x3c
1429d4923  FUN_140495990(+0x84)                          template lookup   (result discarded)
1429d492e  FUN_141d2efc0(mobPool, +0x38)                 mob lookup        (result discarded)
1429d494f  test edi,edi / js    -1 is negative           -> JUMP 1429d4b01
1429d4b01  r13d = 0
1429d4b04  +0x50 == 0 / jle                              -> JUMP 1429d4c4e
1429d4c4e  +0x3f == 0 / je                               -> JUMP 1429d4e64
1429d4e64  esi = +0x1c
1429d4e67  ebx = +0xa8 = DAMAGE
1429d4e6d  damage > 0  ->  SetTemporaryAction(1, 1500)
                           avatar_vtbl[0xd8](0, 5000)
1429d4ed2  FUN_142771360(user, -damage, 0, 0, +0x1c, 0)  the violet number
```

**Because attack index `-1` short-circuits at `1429d494f`, the template and mob lookups are
performed but their results are thrown away.** On the touch path only three fields can change
the outcome: **`+0xa8`, `+0x1c` and `+0x3d`.** Everything else on the wire is decoded into a
stack local that dies with the function. That is a strong safety statement: an echoed body
cannot take a path this trace has not covered.

**It is only that strong for `attack_index == -1`.** A numbered mob attack (`>= 0`) runs the
attack lookup and `FUN_1429f28f0`, and an index `<= -2` takes a different branch entirely
(`1429d4cd6`, where only `-8` is handled). Both are traced far less well. Every archived
capture is `-1`; **if a body ever arrives with anything else, do not broadcast it until that
arm has been read.**

---

## 9. Compared with the inbound `0x00E5` - what is [L] and what is transferred

| claim | status |
|---|---|
| The 147-byte layout is identical in both directions | **[L]**, from the decoder's own 44 stores, matching the encoder's 44 fields on every offset. Two independent functions. |
| Body is unconditionally 147, no gated fields | **[L]** twice: encoder has no branches (`user-hit.md` §2), decoder has no branches (§3.3 here). |
| `+0x08` is the inbound damage | **[L]** from the encoder's register trace (`user-hit.md` §3.3). **The outbound handler never reads it** (§4). The two directions use different damage fields. |
| `+0x34` and `+0x38` are both the mob **object** id | **[L]** - and now confirmed from the *consumer* side: `1429d4931` and `1429d4b22` both feed `FUN_141d2efc0(mobPool, id)`. |
| `+0x84` is the mob **template** id | **[L]** - confirmed from the consumer side: `1429d4923` feeds `FUN_140495990(templateId)`, a different lookup from the pool one. |
| `+0x5c`/`+0x60` are the player's position | **[L]** inbound. **Not used outbound** - the handler gets the position from `[user_vtbl+0x10](user, &rect, 1)` at `1429d49b3`. Position does not need to be correct in a broadcast. |
| `0x02A5` = `u32 charId` + HITINFO | **[L]**, router read + handler chain. |

**Transferred assumption, named:** `research/user-hit.md`'s *meanings* for the fields the
outbound handler does not read (`+0x10` PRNG, `+0x14` tick, `+0x0c`/`+0x30`) are inherited,
not re-derived here. They do not matter for this builder - nothing reads them.

**A useful side result.** `crates/net/src/userhit.rs`'s doc comment on `damage` says it is
*"Not discriminated by the captures"* and that *"a capture with damage other than 1 would
settle it"*. There are now **72 such events** (49 zeroes and 23 others, up to 200), and body+8
co-varies exactly with body+42 (`+0x2c`) and nothing else in the candidate set. The comment is
out of date; offset 8 is now discriminated by measurement.

---

## 10. The v214 reference, labelled [I] and weak

`OutHeader.java` has `REMOTE_HIT(1102)` and `USER_HIT_BY_COUNTER(963)`; `InHeader.java` has
`USER_HIT(275)`. The version's numbering does not match this client and no field list in that
tree resembles a 147-byte HITINFO. **`UserHitRemote` / `RemoteHit` as a name is [I] and worth
nothing more than a label.** `CLAUDE.md` scores that tree 1 of 8, and "fame at offset 41" came
from it and was wrong this week. Every field name in §4 came from the client's own listing.

---

## 11. The builder

**Not written - `crates/` is off-limits to this pass.** This is the exact code to add to
`crates/net/src/userpool.rs`, beside `USER_HIT_REMOTE`.

```rust
/// The HITINFO is **always** exactly 147 bytes. `FUN_14025da80` has zero branches:
/// 44 reads, then `ret`. `research/user-hit-remote-0x02A5.md` §3.3. **[L]**
pub const USER_HIT_REMOTE_HITINFO_LEN: usize = 147;

/// Body offset of the damage the **other** clients draw - HITINFO `+0xa8`, read at
/// `0x14025dcd0`, consumed at `0x1429d4e67`.
///
/// **The client never fills this field.** Its builder `FUN_1428aa0a0` has no write to
/// `rbp+0x2e8`, and 331 event-deduplicated captures carry `0` here. Echo the client's
/// 147 bytes without overwriting it and every remote client calls the damage renderer
/// with `0` - the MISS path - and plays no animation.
pub const USER_HIT_REMOTE_DAMAGE_AT: usize = 143;

/// Build a [`USER_HIT_REMOTE`] body: `u32 charId`, then the client's own 147-byte HITINFO
/// with the server's damage written into it. **151 bytes.**
///
/// `hit_info` is the raw inbound [`crate::userhit::CLIENT_USER_HIT`] body, opcode already
/// stripped - the same slice `parse_user_hit` was given.
///
/// # Why the damage is a separate argument and not read out of `hit_info`
///
/// The two directions use **different** damage fields. Inbound, the client's claim is at
/// body `+8` (HITINFO `+0x08`); the outbound handler `FUN_1429d48c0` **never reads
/// `+0x08`**. It reads `+0xa8` at `0x1429d4e67`, negates it at `0x1429d4ecb`, and passes it
/// to `FUN_142771360` - the renderer whose negative branch selects digit set 3,
/// `NoViolet`, the damage colour (`research/damage-number-draw.md` §2.1). **[L]**
///
/// # `damage` must be positive
///
/// * `> 0` - the violet number **and** the flinch: `FUN_14282d710(user, 1, 1500, 0)` at
///   `0x1429d4e8e` and `[avatar_vtbl+0xd8](avatar, 0, 5000)` at `0x1429d4ea5`, both gated
///   on `test ebx,ebx / jle`.
/// * `0` - the MISS number, no animation. Legal, and arguably right for a 0-damage hit.
/// * **negative** - `neg` makes it positive, which selects digit set 2, `NoBlue`: the
///   *recovery* colour. A red-looking bug with no crash. Do not send one.
///
/// # Send it to everyone EXCEPT the player who was hit
///
/// Their own client already drew its own number at send time, from a local variable at
/// `0x1428aca14` - not from this packet. `Bus::publish` excludes the publisher, which is
/// the behaviour wanted. A `charId` absent from the receiver's `CUserPool` is a clean
/// no-op: every failed lookup in `FUN_1429bb720` jumps to the function epilogue. **[L]**
///
/// # Returns `None` rather than a short body
///
/// Under 147 bytes there is nothing to echo. `FUN_142907b60` appends a second structure
/// after the HITINFO (`research/user-hit.md` §8.5), so a longer body is legal and its tail
/// is dropped here - the decoder stops after 147 bytes.
pub fn user_hit_remote(char_id: u32, hit_info: &[u8], damage: i32) -> Option<Vec<u8>> {
    let src = hit_info.get(..USER_HIT_REMOTE_HITINFO_LEN)?;
    let mut info = [0u8; USER_HIT_REMOTE_HITINFO_LEN];
    info.copy_from_slice(src);
    info[USER_HIT_REMOTE_DAMAGE_AT..USER_HIT_REMOTE_DAMAGE_AT + 4]
        .copy_from_slice(&damage.to_le_bytes());

    let mut w = crate::PacketWriter::new();
    w.u32(char_id);
    w.bytes(&info);
    Some(w.into_vec())
}
```

Tests worth pinning, one per claim that could regress:

```rust
#[test]
fn the_remote_hit_body_is_the_char_id_and_147_more() {
    let body = user_hit_remote(200, &[0u8; 147], 7).unwrap();
    assert_eq!(body.len(), 4 + 147);
}

#[test]
fn the_damage_lands_at_body_offset_143_because_that_is_hitinfo_0xa8() {
    let body = user_hit_remote(200, &[0u8; 147], 7).unwrap();
    assert_eq!(&body[4 + 143..4 + 147], &7i32.to_le_bytes());
}

#[test]
fn every_other_byte_of_the_hitinfo_is_echoed_verbatim() {
    let src: Vec<u8> = (0..147u16).map(|i| i as u8).collect();
    let body = user_hit_remote(200, &src, 7).unwrap();
    assert_eq!(&body[4..4 + 143], &src[..143]);
}

#[test]
fn a_short_hitinfo_is_refused_rather_than_padded() {
    assert!(user_hit_remote(200, &[0u8; 146], 7).is_none());
}
```

### 11.1 Wiring, in `crates/world/src/session/combat.rs::on_user_hit`

**Not done here** - `CLAUDE.md` § *Built is not wired*. `on_user_hit` already computes the
authoritative number in the local `applied` (line ~1007), after the Magic Guard split has been
decided. Add a sibling of `publish_user_attack`
(`crates/world/src/session/multiplayer.rs:413`), called from `on_user_hit` once `applied` and
`chr` exist:

```rust
pub(super) fn publish_user_hit(&mut self, hit: &net::userhit::UserHit,
                               payload: &[u8], applied: u32) {
    // Only the touch arm is traced end to end. research/user-hit-remote-0x02A5.md §8.
    if !hit.is_touch() { return; }
    let Some(chr) = self.claimed_character() else { return };
    let Some(map) = self.bus().map_of(self.subscriber) else { return };
    let damage = i32::try_from(applied).unwrap_or(i32::MAX);
    let Some(body) = net::userpool::user_hit_remote(chr.id, payload, damage) else {
        return; // under 147 bytes - nothing to echo, and a short body has killed this
                // client twice
    };
    self.bus().publish(
        self.subscriber,
        map,
        Reply {
            opcode: net::userpool::USER_HIT_REMOTE,
            body,
            what: format!(
                "UserHitRemote: char {} took {damage} from mob {} - 151 bytes, the \
                 client's own HITINFO with the damage written into +0xa8 (body +143). \
                 Draws the violet number and the 1500 ms flinch on every OTHER \
                 client's screen; this player's own number is already on their screen.",
                chr.id, hit.mob_object_id
            ),
        },
        None,
    );
}
```

`supersedes: None`, like `publish_user_attack`: two hits are two events and the second must
not swallow the first.

**Note the damage argument is `applied`, not `hit.damage`.** `hit.damage` is the client's
claim at body `+8`; `applied` is what the `0x007C` actually moved the bar by. Sending
`hit.damage` would put a number on the other screens that disagrees with the bar the hurt
player is watching, which is the exact complaint `research/damage-number-draw.md` opens with.

**`on_user_hit` returns early when `hit.damage == 0`** (line 962), before any of this, so a
claimed-zero hit stays invisible to everyone. That is existing behaviour and this change does
not alter it.

Note that `on_user_hit` currently **returns early when `hit.damage == 0`**. That early return
happens before any broadcast would be added, so a claimed-zero hit stays invisible to
everyone - which is the existing behaviour and is fine.

---

## 12. Corrections to files I do not own

Recorded here as `CLAUDE.md` instructs; those files are untouched.

1. **`crates/net/src/userpool.rs`, the `USER_HIT_REMOTE` doc block** - *"There is no builder
   because there is nothing to build: the 147 bytes are the client's, and the server's job is
   to pass them on."* **Falsified.** The field the handler draws the number from is `+0xa8`,
   and the client's builder never writes it: 331 of 331 captured bodies carry `0`, and
   `FUN_1428aa0a0` has no write to `rbp+0x2e8`. A pass-through produces the MISS number and
   no animation. §5.

2. **`research/same-map-capability-sweep.md` row 6** - *"Echo the inbound `0x00E5` body
   behind the `charId`."* Same correction: the echo is right for 146 of the 147 bytes and
   wrong for the one that matters. The row's other two claims - `>= 147` never `== 147`, and
   *"`crates/net` has no HITINFO type"* - both stand.

3. **`research/user-hit.md` §5.3** - *"the broadcast is `0x02A5` = `u32 charId` + the same
   147-byte HITINFO = 151 body bytes"*. The **length and the route are exactly right**; §5.3
   simply never asked which fields the handler reads, and called the packet *"cosmetic"*.
   It is cosmetic in the sense that it moves no HP, but it is the only thing that puts the
   number and the flinch on another player's screen, and it needs one field filled in.
   Its §9 step 4 - *"Optionally broadcast `0x02A5` (`u32 charId` + the 147 bytes echoed)"* -
   needs the same amendment.

4. **`research/same-map-capability-sweep.md` §4.5 - corroboration, not a correction.** It
   reports `FUN_14282D710` as reached by **23 handlers** and therefore *"a generic `CUser`
   method"*. That is consistent with what it does here and is the reason this file does not
   name it: the argument shape `(user, 1, 1500, 0)` and the mirror with the local hit path are
   the evidence, not the identity of the function.

5. **`crates/net/src/userhit.rs`, the `damage` doc comment** - *"Not discriminated by the
   captures ... A capture with damage other than 1 would settle it."* There are now **72**
   such events. §9.

---

## 13. Blind spots, named

* **Nothing here has been on screen.** The whole of §5 is a complete branch trace plus the
  structural mirror in §6 - it is **[D]**, not [L]. The falsifier is one client run with two
  clients: A stands in a snail, B watches. Violet number and a 1500 ms flinch over A on B's
  screen, or nothing.
* **`FUN_141c5d1c0` is unidentified** (§6). Leaving `+0x50 = 0` is what the local client
  does, so nothing is lost relative to the local screen - but I cannot say what it would add.
* **`[avatar_vtbl+0xd8]` and `FUN_140f832e0`'s action id `1` are unnamed.** That they are the
  flinch and the damaged-flash is **[I]** from the argument shape and the 1500/5000 ms
  durations; that they are *the same two calls the client makes on itself when hurt* is
  **[L]**, and that is the claim the recommendation rests on.
* **Only the `attack_index == -1` arm is traced.** §8.
* **The pool's hash function was not verified** - only that a miss lands on the epilogue.
* **`+0x1c` and `+0x3d` reach the renderer as arg5/arg4 and I did not chase what those two
  arguments do.** Both are `0` in every capture, so echoing them is the measured-safe choice;
  a builder that synthesises a body should write `0`.
