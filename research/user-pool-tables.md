# The user pool's inbound dispatch, enumerated - and where remote MOVE and ATTACK live

Written 2026-08-29. **No Ghidra** (another agent held the project lock), **no client run.**
Everything below was read out of `client-patched\MapleStory.exe` with `tools/listing.py`,
`tools/reads.py`, `tools/callers.py`, `tools/encodes.py` and five throwaway scripts whose
source is reproduced in section 8.

Tags on every claim:

* **[L]** read off the listing / the raw table bytes of the PE.
* **[D]** derived by analysis over those bytes.
* **[I]** inferred, or a candidate from the v214 reference - not provable from this client.

> ## THIS IS NOT WIRED
>
> Nothing in this document has been built or sent. It is a map, not a feature. Every byte
> layout below is static analysis: **[L]** for the shapes, **[I]** for "the client will like
> it".
>
> I nearly wrote *"these opcodes have never been on the wire"* here without checking, which
> is the mistake this repo keeps paying for. Grepping the archive instead - with a positive
> control first, `0x03D9`/`0x02FF` are found in dozens of world logs, so the search works:
>
> * **No `world.log` anywhere** contains `0x0293` or `0x029E..0x02A1`. This server has never
>   built one, and with one client connected there was never an occasion to.
> * **`research/fixtures/sweep-01f2-03c7-exit.log` does contain all five**, at `01:05:21` and
>   `01:05:23`, each sent with a **32-byte junk body**, and the client did not die on any of
>   them - it survived to `0x03C7` and answered 3465 bytes.
>
> **That second bullet is worth almost nothing and must not be quoted as "the client accepts
> these".** The log's own header is `tools/handshake_probe.py`'s
> `G=1 H=1 L=1, high=100 (all known gates satisfied)`, so it is a **login-stage** sweep on a
> fresh socket: `CField::OnPacket` was not the active dispatcher and `FUN_1429b9300` was
> never reached. What the sweep shows is that the *login* stage ignores them. It says nothing
> about any handler in this document.

---

## 0. The answers, up front

| the server wants to say | packet | body | status |
|---|---|---|---|
| "that other player walked" | **`0x0293`** | `u32 charId`, then the **movement path block verbatim** - `14`-byte head + variable elements, and **nothing else** | **[L]**, section 3 |
| "that other player attacked" | **`0x029E`, `0x029F`, `0x02A0`, `0x02A1`** | `u32 charId`, `u16`, a 13-field header, then a target/damage list | **[L]**, section 4 |
| (which of the four means what) | - | the handler **stores the opcode and never reads it back** | **[L]** that it is ignored; the melee/shoot/magic/body assignment is **[I]** |
| "that other player got hit" | `0x02A5` | `u32 charId` + 147-byte HITINFO | already decoded, `research/user-hit.md` section 5.3 |
| "that other player levelled" | `0x02AF` | `u32 charId, u8 effect` | already decoded, `research/level-up.md` section 6 |

The remote-move head is **four bytes**, not the ten of the client's own `0x00D9` and not the
twenty-four of `crate::mobmove::mob_move_broadcast`'s `0x03D9`. That is measured, and it is
the one number in this document most likely to be assumed wrong from a v83 memory.

---

## 1. The instruments, and what they were checked against first

`CLAUDE.md`: *"Verify the instrument before believing it."* All three controls run from the
repo root and all three reproduced before any of the work below was done. **[L]**

```
python tools/reads.py   0x140304100 2          -> raw @140304138, u8 @140304144, u8 @140304183, then a run of u16
python tools/listing.py 0x140304100 | grep READ -> the same reads at the same addresses
python tools/callers.py 0x1402fa9a0            -> 96 call sites in 15 functions, 43 of them in 0x140304b20
```

The `tools/` copies are the live ones - 9605-byte `reads.py`, which contains all eleven read
primitives including `0x1406e8fb0` (`f64`) and `0x142d23ef0`. Every script in section 8 was
run as `python - < script` **from the repo root**, which leaves `sys.path[0]` empty, so the
scratchpad's stale `reads.py` / `listing.py` / `callers.py` could not shadow them.

### The four positive controls the brief demanded - all four reproduced

| control | expected | this walk |
|---|---|---|
| table `0x1429bbc34` index `0x07`, opcode `0x02A5` = remote user hit | `research/user-hit.md` section 5.3 names stub **`0x1429bb9b2`** -> `FUN_1429d48c0` | **stub `0x1429bb9b2` -> `call 0x1429d48c0`, opcode `0x02A5`** - reproduced |
| table `0x1429bbc34` index `0x11` -> `call 0x1427863f0`, opcode `0x02AF` | `research/level-up.md` section 6 | **stub `0x1429bba36` -> `call 0x1427863f0`, opcode `0x02AF`** - reproduced |
| table `0x14289d660` index `0x0C` -> `call 0x1427863f0`, opcode `0x02D1` | `research/level-up.md` section 6 | **stub `0x14289a439` -> `call 0x1427863f0`, opcode `0x02D1`** - reproduced |
| table `0x1429bb5d0` index 0 -> opcode `0x0226` = `UserChat` -> `FUN_1427847a0` | `research/user-chat-round2.md` | **stub `0x1429bb124` -> `call 0x1427847a0`, opcode `0x0226`** - reproduced |

The brief's own text for the first control said *"the stub at `0x1429bba36`'s family"*. That
is a transcription slip in the brief: `0x1429bba36` is index `0x11`, the second control.
`research/user-hit.md` section 5.3 line 361 says **`<- FUN_1429bb720  stub at 1429bb9b2`**,
and that is what index `0x07` holds. The source document and this walk agree; only the brief
disagreed with both.

---

## 2. The routing, re-derived from the image - and there are FOUR tables, not three

`FUN_1429b9300`, read off the listing. **[L]**

```
1429b9317  sub  ecx, 0x224
1429b931f  je   1429b9467                 ; 0x224 inline
1429b9328  je   1429b944d                 ; 0x225 inline   (after cmp ecx,1)
1429b932e  lea  eax,[rdx - 0x226]
1429b9334  cmp  eax, 0x6c
1429b933b  ja   1429b9350
1429b934b  jmp  0x1429bafb0               ; 0x226..0x292   109 opcodes
1429b9350  lea  eax,[rdx - 0x293]
1429b9356  cmp  eax, 0x31
1429b9359  ja   1429b9372
1429b936d  jmp  0x1429bb720               ; 0x293..0x2C4    50 opcodes   REMOTE
1429b9372  lea  eax,[rdx - 0x2c5]
1429b9378  cmp  eax, 0xd9                 ; 0x2C5..0x39E   218 opcodes   LOCAL
```

That much matches the brief. What the brief did not have is that **`FUN_1429bb720` contains
two jump tables, and both are live.** Enumerating rather than looking up the one address I
was handed is the only reason this is here at all - `CLAUDE.md`: *"enumerate before you
filter"*. **[L]**

```
; ---- the flood gate, per remote user ----
1429bb8ae  lea  rcx,[rbx + 0x100]         ; rbx is the CUser found in the pool, not the pool
1429bb8b5  call 0x140f8abc0
1429bb8c1  test eax, eax
1429bb8c3  jne  1429bbaea                 ; throttled -> skip table B entirely
1429bb8c9  cmp  dword [rbx + 0x40d8], eax
1429bb8cf  jne  1429bbaea                 ; already flagged -> same
   ...  1000 ms window at [rbx+0x40d0], counter at [rbx+0x40d4], cap 100 (200 in some state),
        over cap sets [rbx+0x40d8] = 1 and the user is throttled for the rest of the session

; ---- table B: dense, 39 entries ----
1429bb940  lea  eax,[rsi - 0x29e]
1429bb946  cmp  eax, 0x26
1429bb949  ja   1429bbb10
1429bb951  mov  ecx,[r14 + rax*4 + 0x29bbc34]
1429bb95c  jmp  rcx

; ---- table C: byte-indexed, 49 opcodes ----
1429bbb10  add  esi, 0xfffffd6d           ; esi -= 0x293
1429bbb16  cmp  esi, 0x30
1429bbb19  ja   1429bbc08
1429bbb22  movzx eax, byte [r14 + rax + 0x29bbd10]     ; 0x31-entry byte index
1429bbb2b  mov   ecx, dword [r14 + rax*4 + 0x29bbcd0]  ; 16-entry dword table
1429bbb36  jmp   rcx
```

### The two tables are exactly complementary - no opcode is handled by both

This is worth stating because the control flow *looks* like double dispatch: most table-B
arms end `add esi,-0x293 / jmp 0x1429bbb1f`, which is table C's dispatch with the bound check
skipped. Walking every opcode in `0x293..0x2C4` against both tables: **[L]**

| | count | opcodes |
|---|---|---|
| table B only | 22 | `0x29e..0x2a9`, `0x2ac`, `0x2af`, `0x2b7`, `0x2bb..0x2bd`, `0x2bf..0x2c1`, `0x2c4` |
| table C only | 15 | `0x293`, `0x2aa`, `0x2ad`, `0x2ae`, `0x2b0..0x2b6`, `0x2b9`, `0x2be`, `0x2c2`, `0x2c3` |
| **both** | **0** | - |
| neither (silently dropped) | 13 | `0x294..0x29d`, `0x2ab`, `0x2b8`, `0x2ba` |

Every table-B slot that resolves to a real handler lands on a table-C index whose byte is
`0x0f`, the default. `0x2C4` is the one that needs a second look: its arm `0x1429bba4c` ends
`jmp 0x1429bbc08`, going **straight to the exit** rather than into table C - which matters,
because `0x2C4 - 0x293 = 0x31` is one past the end of the 0x31-entry byte table. The bound
check at `0x1429bbb16` (`cmp esi,0x30 / ja`) catches it on the other path. So the byte table
is never read out of bounds. **[L]**

**Under throttling** (`0x1429bbaea`) table B is skipped and opcodes `0x29e..0x2a1` - the four
attacks - go to `FUN_1429df530` instead, which reads **nothing** from the packet
(`tools/reads.py 0x1429df530 4`). So a flooding remote user's attacks stop being drawn while
everything table C handles keeps working. **[L]**

### Table A's default arm is not a dead end either

`FUN_1429bafb0`'s table covers `0x226..0x276` only. Opcodes `0x277..0x292` fall through
`ja 0x1429bb53f` into a chain of four range tests, each of which forwards `(pool, opcode,
pkt)`: **[L]**

| range | count | handler | reads (depth 4) |
|---|---|---|---|
| `0x277..0x27E` | 8 | `FUN_142795b20` | 8 sites: `u32`, then six helper subtrees |
| `0x27F..0x283` | 5 | `FUN_142798a40` | 5 sites |
| `0x284..0x28B` | 8 | `FUN_142846520` | 4 sites |
| `0x28C..0x291` | 6 | `FUN_1428340e0` | 2 sites, both `FUN_141404c80` |
| `0x292` | 1 | **none** - falls past all four | - |

---

## 3. `0x0293` is the remote-player MOVE, and its head is FOUR bytes

### How it was found: reachability, not names

`crates/net/src/usermove.rs` and `crates/net/src/mobmove.rs` establish that **`FUN_1404b2630`**
is the 79-command movement-path decoder (79 commands, 13 case bodies, byte index at
`0x1404b2f24`, dword table at `0x1404b2ef0`). So the question is which user-pool handler
reaches it.

I did not grep for it. I enumerated **every** stub in all four tables, resolved each to its
handler, and walked the call graph to depth 6 over `tools/reads.py`'s `calls_of` - which sees
tail `jmp` as well as `call`. **315 distinct handler functions**, two of which reach
`0x1404b2630`: **[L]**

```
table C  opcode 0x0293  handler 0x1429d2e70    <- REMOTE, u32 charId already consumed
table D  opcode 0x02F5  handler 0x14279c140    <- LOCAL
```

`tools/callers.py` agrees from the other end and its `call`-only blind spot did not bite
here: `0x1404b2630` has **2 call sites, 0 tail jmps, 0 pointers in data** - `FUN_141d58e60`
and `FUN_141d598b0`. `FUN_141d598b0` has 8 callers, one of which is at `0x1429d2ec6`, inside
`FUN_1429d2e70`. **[L]**

> **The blind spot, named.** `FUN_141d58e60` has **zero** callers by all three of
> `callers.py`'s scans. It is almost certainly virtual, and a handler that reached the path
> decoder only through *its* vtable slot would be invisible to this sweep. That is the one
> way `0x0293` could fail to be the only remote mover. I could not close it without Ghidra.
> **[D]**

### The whole handler, and it reads nothing

`FUN_1429d2e70` is 102 bytes across three merged `.pdata` entries. **[L]**

```
1429d2e70  push rbx / sub rsp,0x30
1429d2e76  mov  rcx,[rcx + 0x11b8]        ; rcx = the CUser the pool looked up; NULL -> return
1429d2e7d  mov  rbx, rdx                  ; rbx = the packet
1429d2e85  add  rcx, -0x20
1429d2e8e  call 0x14099fe20               ; rdi = the movement-path object
1429d2e96  call 0x141892840               ; the field singleton
1429d2ea8  call 0x141829fd0               ; eax = a field value, 0 if no field
1429d2ead  mov  byte [rsp+0x28], 0        ; arg6 = 0
1429d2eb2  mov  r9d, eax                  ; arg4
1429d2eb5  xor  r8d, r8d                  ; arg3 = 0   <-- no trailing nibble block
1429d2eb8  mov  dword [rsp+0x20], 0       ; arg5 = 0
1429d2ec6  call 0x141d598b0
```

`tools/reads.py 0x1429d2e70 4` reports **exactly one read site**, and it is that `call`.
Nothing between the dispatcher's `u32 charId` and the path block reads a byte. **[L]**

`FUN_141d598b0` calls `FUN_1404b2630` at `0x141d59910` **before any read of its own**, and
its two direct reads (`0x141d5991f`, `0x141d59936`) are the trailing nibble-count block,
which is gated on argument 3. `crates/net/src/mobmove.rs` already documents this exact gate
from the mob side - `141cb8346 MOV R8D,R13D` for `0x02FF` puts the byte there,
`141c82000 XOR R8D,R8D` for `0x03D9` does not. `FUN_1429d2e70` is the second kind. **[L]**

### The body

```text
off  size  read at        field
  0  u32   1429bb745      charId            <- FUN_1429bb720, before the dispatch
  4  u32   1404b2650      path head field 1
  8  i16   1404b265b      x
 10  i16   1404b2675      y
 12  u16   1404b2690
 14  u16   1404b269c
 16  i16   1404b26a9      element count (signed; MOVSX / TEST / JLE bail)
 18  ...                  element count elements, variable length per command byte
                          (crate::net::usermove::element_len, 79 commands -> 13 bodies)
                    END - no trailing byte
```

So **4 bytes precede the path block, and they are `u32 charId`.** Total body =
`4 + 14 + sum(element_len)`. **[L]**

### The trailing key-state block: `0x0293` does NOT carry it. Use `path_span`.

This is the question the net-crate agent asked, and it is one instruction. **[L]**

`FUN_1404b2630` does not decide it. The path decoder takes `(rcx = pathObject,
rdx = packet)` and **never reads `r8d` or `r9d` on entry** - the only appearances of `r8d`
inside its 2371 bytes are as a scratch register it writes first (`0x1404b270e movzx r8d,cl`,
`0x1404b2df2 movzx r8d,al`). The trailer is read by the **wrapper**, `FUN_141d598b0`, after
the decoder has returned:

```
141d598dd  mov  ebx, r8d                 ; ebx = argument 3, saved across the call
141d59910  call 0x1404b2630              ; the path itself - head + elements, no trailer
141d59918  test ebx, ebx
141d5991a  je   141d5997a                ; ebx == 0 -> the trailer is NOT in this packet
141d5991f  READ u8                       ; nibble count
141d59936  READ u8   x (count+1)/2       ; two 4-bit key states per byte
```

So the discriminator is argument 3 **at the decode call site**. The remote user move's is
byte-identical to `0x03D9`'s, the side `mob_move_broadcast` already stops short of: **[L]**

| decode call site | argument 3 | key-state block |
|---|---|---|
| `0x02FF` mob move in, `141cb8346 MOV R8D,R13D` | non-zero | **read** |
| `0x03D9` mob move out, `141c82000 XOR R8D,R8D` -> `141c82009 call` | 0 | not read |
| **`0x0293` user move out, `1429d2eb5 XOR R8D,R8D` -> `1429d2ec6 call`** | **0** | **not read** |

**`0x0293` is on the `0x03D9` side. Send `UserMove::path_span`, not
`path_with_key_states_span`.** The extra `1 + (keyCount + 1) / 2` bytes would sit past the
end of a body the client stops reading - the same failure `mob_move_broadcast` avoids by
construction, and the same class of mistake that killed this client twice. **[L]**

#### The encoder is NOT symmetric with the decoder, and reading only the call sites says the opposite

I had this half wrong in a first draft and the correction is worth keeping, because the
obvious check gives the wrong answer. `FUN_1409f6eb0`, which builds the client's own
`0x00D9`, **also passes zero**: `1409f96c7 XOR R8D,R8D` immediately before
`1409f96d2 call 0x141d57c60`. Read the call sites alone and you would conclude `0x00D9`
carries no key states either - against 1082 captured bodies that are consumed exactly by
head + path + trailer.

The resolution is that **`FUN_141d57c60`'s third argument does not gate the trailer at all.**
The write is unconditional once the path is written: **[L]**

```
141d580bb  movzx r9d, byte [rbp + 0x77]      ; a branch target - one basic block from here
141d580c0  xor   r8d, r8d
141d580c9  call  0x1404b2000                 ; the path: head + elements
141d580ce  mov   rax, [rdi + 0x48]           ; the key-state array on the path object
141d580d2  test  rax, rax
141d580d5  jne   141d580dc
141d580d7  mov   eax, r14d                   ; ... 0 if there is no array
141d580da  jmp   141d580df
141d580dc  mov   eax, [rax - 8]              ; ... else its length
141d580df  movzx edx, al
141d580e5  call  0x1406ed840                 ; w_u8 count  <- NOTHING SKIPS THIS
141d580f0  ...  then (count + 1) / 2 packed bytes, two 4-bit states each
```

Both branches converge on `141d580df`, so `0x141d580e5` executes on every path that reaches
`0x141d580c9`. `tools/encodes.py` prints `gated?` against it, and that flag is doing exactly
what its own docstring warns about - it fires on *any* conditional branch earlier in the
function and over-reports. Reading the listing is what settles it. **[L]**

So: **the encoder always writes the block; only the decoder chooses whether to read it.**
Anything the client sends through `FUN_141d57c60` carries it; whether the server may send it
back is a property of the *receiving* handler, and for `0x0293` the answer is no. **[D]**

Contrast, all four from the same codec (`FUN_141d57c60` -> `FUN_1404b2000` writes,
`FUN_141d598b0` -> `FUN_1404b2630` reads):

| packet | head before the path | key-state block | other tail |
|---|---|---|---|
| `0x00D9` client -> server, its own move | 10 bytes (`u8`, `u32`, `u32`, `u8`) | **yes** | - |
| `0x02FF` client -> server, mob move | 46 bytes | **yes** | 29 bytes |
| `0x03D9` server -> client, mob move | 24 bytes | no | 1 byte, `141c82011` |
| **`0x0293` server -> client, user move** | **4 bytes** | **no** | **none** |

`0x0293` is the only one of the four with nothing at all after the path: `FUN_1429d2e70`
reads zero bytes itself and `FUN_141d598b0` stops at `141d5991a`. **[L]**

### Wire it like this **[D]**

`crate::net::usermove::parse_user_move` already retains the path span, and
`UserMove::path` / `path_with_key_states` already refuse unless `walk_closed`. So the
rebroadcast is:

```rust
// 0x0293, to everyone in the map EXCEPT the sender.
let mut b = Vec::new();
b.extend_from_slice(&char_id.to_le_bytes());   // 1429bb745
b.extend_from_slice(mv.path(body)?);            // 1404b2630, verbatim - do NOT re-encode
                                                // path(), NOT path_with_key_states()
```

The path is copied rather than rebuilt for the same reason `mob_move_broadcast` copies:
`0x00D9` and `0x0293` reach the **same** codec. `FUN_141d57c60` wrote those bytes at
`0x1409f96d2`; `FUN_1404b2630` reads them at `0x141d59910`. **[L]**

Two things that will bite:

* **Use `UserMove::path()`, not `path_with_key_states()`.** `FUN_1429d2e70` passes arg3 = 0,
  so the client does **not** read the key-state block after the path. Sending it adds
  `1 + (keyCount + 1) / 2` bytes past where the client stops - the same class of bug
  `mobmove.rs` documents for `0x03D9`. **[L]**
* **`0x0293` bails silently if `charId` is not a user in the receiving client's pool.** The
  lookup is the hash walk at `0x1429bb75c..0x1429bb784`; every failure path goes to
  `0x1429bbc19`, the plain epilogue. So the *enter-field* packet has to have landed first,
  and a mis-ordered enter/move pair looks exactly like "movement does not work". **[L]**

### `0x02F5` is the local twin, and it is NOT the same shape

`FUN_14279c140` (51 bytes) gates on `[localUser + 0x1260] != 0`, then tail-jumps to
`FUN_1409c70d0`, which calls `FUN_141d598b0` with **arg3 = 1** (`0x1409c70eb lea r8d,[r9+1]`).
So `0x02F5` *does* carry the trailing nibble block, and it has no `charId`. Do not reuse the
`0x0293` builder for it. **[L]**

---

## 4. The remote ATTACKS are `0x029E..0x02A1`, and one handler serves all four

Table B indices `0x00..0x03` all hold the **same stub**, `0x1429bb95e`, which calls
`FUN_1429d2ee0(user, opcode, pkt)`. Four opcodes, one handler. The throttled path at
`0x1429bbaea` independently special-cases the *same four* (`sub ecx,0x29e / je`, then three
more) before falling into table C. Two separate pieces of control flow agree on the set.
**[L]**

Inbound, this server already parses `0x00DF` melee, `0x00E0` shoot, `0x00E1` magic
(`crates/net/src/attack.rs`, one shared body) and `0x00E2` body attack. **Four in, four out.**
**[D]**

### The read shape

`FUN_1429d2ee0` makes three reads of its own and delegates twice: **[L]**

```
1429d2f3f  READ u16              -> [user + 0x406c]
1429d2fd8  call 0x140f32200      the header
1429d302b  call 0x140f32440      the target / damage list
```

`FUN_140f32200`, 13 fields, in order, with the struct offset each lands in: **[L]**

```
140f32380  u32 -> hdr+0x08     <- attack.rs's SKILL ID sits at struct +0x08
140f3238b  u32 -> hdr+0x0c     <- attack.rs's skill level sits at +0x0c (u8 inbound, u32 here)
140f32396  u8  -> hdr+0x1c     (stored as a bool: TEST AL,AL / SETNE)
140f323a6  u32 -> hdr+0x34
140f323b1  u32 -> hdr+0x38
140f323bc  u32 -> hdr+0x40
140f323c7  u32 -> hdr+0x44
140f323d2  u32 -> hdr+0x20
140f323dd  u8  -> hdr+0x2c
140f323eb  u8  -> hdr+0x48
140f323f9  u32 -> hdr+0x50
140f32404  u32 -> hdr+0x54
140f3240f  u32 -> hdr+0x60
```

`FUN_140f32440`, the damage list - *"a count, then per-mob `u32 objectId` + damage entries"*,
exactly as the brief predicted: **[L]**

```
140f32523  u32  targetCount        -> [r15+0]   ; the loop bound at 140f32546 / 140f32626
140f3252e  u32                     -> [r15+4]
140f3253a  u32                     -> [r15+8]
  repeat targetCount, stride 0x1d8 in the struct:
140f32559    u32  mobObjectId      ; ZERO -> skip the whole rest of this entry (JE 140f3261c)
140f3256f    u16  damageCount
      repeat damageCount, stride 0x10:
140f32593      u8   (bool)
140f325a3      u8   (bool)
140f325b3      u64  damage         <- 64-bit, not 32
140f325cc    u8
140f325da    u8
140f325e7    u8
140f325f5    u16
140f32603    u16
140f32611    u8
```

`FUN_140f32200` / `FUN_140f32440` are the decode twins of the `FUN_140f31fe0` /
`FUN_140f31f60` pair that `crates/net/src/attack.rs` names as the client's *encoders*, and
they sit 0x210 bytes apart in the same block. They are **not** symmetric:
`tools/encodes.py 0x140f31fe0` counts **40 writes** against `FUN_140f32200`'s 13 reads, so
the broadcast is a genuinely reduced re-encode of the request, not an echo. **[D]**

### Which of the four means melee, shoot, magic, body - and why I cannot say

`FUN_1429d2ee0` takes the opcode in `edx`, stores it at `[rbp-0x74]` in its prologue, **and
never reads it back.** **[L]**

That negative has an instrument story worth keeping, because my first attempt at it was the
exact failure `CLAUDE.md` warns about. I scanned the function's bytes for the pair `45 8c`
(ModRM `[rbp+disp8]`, disp8 `0x8c` = `-0x74`) and got **zero hits** - a clean, confident,
wrong answer, because `45` fixes the reg field to `eax` and the real store is
`89 55 8c` (reg = `edx`). The correctly-shaped scan is `(modrm & 0xC7) == 0x45 &&
disp8 == 0x8C`, and it carries its own positive control: run the same scan for
`disp8 == 0x84` (`[rbp-0x7c]`, a slot the decoder shows being used seven times) and it must
find seven. It finds exactly seven, and exactly one for `-0x74`: the store. **[L]**

So the client's rendering of a remote attack cannot depend on which of the four opcodes
carried it. Picking `0x00DF -> 0x029E`, `0x00E0 -> 0x029F`, `0x00E1 -> 0x02A0`,
`0x00E2 -> 0x02A1` mirrors the inbound order and is what I would send, but that mapping is
**[I]** and this document does not establish it. What *is* established is that getting it
wrong costs nothing on screen. **[L]**

### The one place a linear read walk lies in this range

`FUN_1429d2ee0` is the only handler in the whole user pool where `reads.py` and `listing.py`
run off the end of the code. At `0x1429d31bc` it does `jmp 0x144dad6a6`, which is inside
**`.themida`** - a section with a **zero-byte raw size**, filled only at runtime. Everything
`.pdata` claims after that (the extent is 4631 bytes; the real code is 737, 0x1429d2ee0..0x1429d31c1) is VM data, and
a linear sweep happily disassembles it into plausible-looking nonsense.

I re-read the function by **recursive descent** instead, following only intra-extent branches:
175 instructions, one escape, and it is the last one. The escape sits immediately after
`mov edx,0xff / call 0x1415f0df0` - the same trace-logging call that immediately precedes the
ordinary epilogue in `FUN_1429bb720` (`0x1429bbc14 call 0x1415f0d70`). So what has been
virtualised is the **epilogue**, and the read list above is complete. **[D]**

Censused across all 315 distinct handler functions in the four tables: **3 escape into
`.themida`.** **[L]**

| handler | opcodes | extent | real instructions before the escape |
|---|---|---|---|
| `0x1429d2ee0` | `0x29e..0x2a1` | 4631 B | 175 - epilogue only |
| `0x1428e3ad0` | `0x33b` | 201 B | 4 - **fully virtualised**, nothing readable |
| `0x14292eaa0` | `0x377` | 96 B | 4 - **fully virtualised**, nothing readable |

Any read count this document gives for `0x33b` or `0x377` is worthless. Every other row is
real x86.

---

## 5. What the enumeration itself found

| | table A | table B | table C | table D |
|---|---|---|---|---|
| routed by | `FUN_1429bafb0` | `FUN_1429bb720` | `FUN_1429bb720` | `FUN_14289a3a0` |
| table address | `0x1429bb5d0` | `0x1429bbc34` | `0x1429bbcd0` (byte index `0x1429bbd10`) | `0x14289d660` |
| index | `opcode - 0x226` | `opcode - 0x29e` | `opcode - 0x293` | `opcode - 0x2c5` |
| entries | 81 | 39 | 49 opcodes -> 16 dword slots | 218 |
| distinct stubs | 67 | 20 | 16 | 175 |
| `default:` arm | `0x1429bb53f`, **15** slots | `0x1429bbb10`, **17** slots | `0x1429bbc08`, **34** slots | `0x14289d5f5`, **19** slots |
| second no-op arm | - | - | - | `0x14289d615`, **25** slots (an explicit empty case, not the default) |
| real handlers (distinct stubs minus the no-op arms) | 66 | 19 | 15 | **173** |
| opcode range | `0x226..0x276` (+ four ranges to `0x291`) | `0x29e..0x2c4` | `0x293..0x2c3` | `0x2c5..0x39e` |

Table D's `0x14289d5f5` reaches `call 0x14200d0e0` before the stack-cookie check; the 25 slots
on `0x14289d615` go straight to `0x142ef44b0` and `ret`. Both do nothing to the packet, but
only the first is the compiler's `default:`. Counting them together would have said the
default occupies 44 slots, which is wrong twice. **[L]**

`FUN_14289a3a0` also has an out-of-table case above the range: `0x14289a3e1 cmp edx,0x62b /
jg / je 0x14289d5a1`. Opcode `0x062B` is handled, and it is far outside the `0x2C5..0x39E`
the field router forwards, so it must arrive from a different caller. Not chased. **[L]**

### Three handlers are shared between the remote and local halves

A useful cross-check, and free corroboration for the whole table walk: **[L]**

| remote (table B) | local (table D) | handler |
|---|---|---|
| `0x02A6` | `0x02C5` | `FUN_1427862e0` |
| `0x02A7` | `0x02D0` | `FUN_142786350` |
| `0x02AF` | `0x02D1` | `FUN_1427863f0` (the effect handler, `research/level-up.md` section 6) |

The third pair is the documented one. The first two fall out of the same walk with no extra
work, and they are the same three-in-a-row block in both tables.

### 13 remote opcodes are accepted by the field router and then silently dropped

`0x294..0x29d`, `0x2ab`, `0x2b8`, `0x2ba`. `FUN_1429b9300` forwards them (they are inside
`0x293..0x2C4`), `FUN_1429bb720` reads their `u32 charId`, looks the user up, runs the flood
counter, and falls off both tables into the epilogue. **They will not freeze the client** -
the handler returns normally - but nothing happens. **[L]**

---

## 6. What I did NOT establish

Read this before building on anything above.

1. **Nothing here has been on the wire.** No client run, no capture. `0x0293` has never been
   sent by this server; the archived logs cannot contain one because only one client has ever
   connected.
2. **The melee/shoot/magic/body assignment across `0x029E..0x02A1` is [I].** What is measured
   is that the handler ignores the distinction, which is a weaker and much safer claim.
3. **`FUN_141d58e60`'s callers are unknown.** Zero by `call`, zero by tail `jmp`, zero by data
   pointer. If it is virtual, a second remote-move opcode could exist behind a vtable slot and
   this sweep would not see it. This is the named blind spot on the "`0x0293` is the only
   remote mover" claim.
4. **The header fields of the attack broadcast are unnamed except two.** `hdr+0x08` and
   `hdr+0x0c` are the skill id and level by analogy with `attack.rs`'s struct offsets, which
   is **[D]** from a different packet's parser, not **[L]** for this one. The other eleven are
   offsets with no meaning attached.
5. **`0x33b` and `0x377` are opaque.** Fully virtualised; their rows in section 9 carry
   numbers that describe Themida's stub, not the handler.
6. **The flood cap is 100 or 200 and I did not determine which state picks 200.**
   `0x1429bb8f8 mov ecx,0x64 / mov edx,0xc8 / cmovg ecx,edx`, on the sign of a value read
   through the field object at `[rax+0xd00]`. Relevant only if a rebroadcast storm ever
   throttles a legitimate player.
7. **`0x02F5`'s meaning.** It decodes a path for the *local* user with the nibble block on.
   Forced move, position correction, or replay - I did not chase it.

---

## 7. Reproducing every number in this document

All of these run **from the repo root**. The five throwaway scripts are reproduced in
section 8; each was run as `python - < script.py` so `sys.path[0]` is empty.

```
python tools/reads.py    0x140304100 2        # the documented control - run this FIRST
python tools/listing.py  0x140304100 | grep READ
python tools/callers.py  0x1402fa9a0          # 96 sites, 43 in 0x140304b20

python tools/listing.py  0x1429b9300          # the four-way router
python tools/listing.py  0x1429bafb0          # table A + the four range tests
python tools/listing.py  0x1429bb720          # tables B and C, and the flood gate
python tools/listing.py  0x14289a3a0          # table D

python tools/callers.py  0x1404b2630          # 2 calls, 0 tail jmps, 0 data pointers
python tools/callers.py  0x141d598b0          # 8 callers, one is 0x1429d2ec6
python tools/reads.py    0x1429d2e70 4        # ONE read site, and it is the path call
python tools/reads.py    0x1404b2630 1        # the 14-byte path head, six reads
python tools/reads.py    0x140f32200 4        # the attack header, 13 fields
python tools/reads.py    0x140f32440 4        # the damage list
python tools/encodes.py  0x140f31fe0          # 40 writes - the request encoder, for contrast
```

Raw output of the two enumerations is checked in beside this file:

* `research/msexe-userpool-tables-raw.txt` - every table entry, its stub, and the stub's calls
* `research/msexe-userpool-tables-handlers.txt` - every handler with extent, read count, read
  types, and the reachability flag for `0x1404b2630`

---

## 8. The five scripts

All run from the repo root as `python - < script.py`, which leaves `sys.path[0]` empty so the
scratchpad's stale copies of `reads.py` / `listing.py` / `callers.py` cannot shadow `tools/`.
`foff` is the clamped VA -> file-offset helper from `research/level-up.md` section 9,
unchanged.

**1. Walk a dense table.** `walk_dense(0x1429BB5D0, 0x51, 0x226, 0x1429BB53F)` and the same
for `(0x1429BBC34, 0x27, 0x29E, 0x1429BBB10)` and `(0x14289D660, 0xDA, 0x2C5, 0x14289D5F5)`.

```python
import struct, sys
sys.path.insert(0, "tools")
from rtti import load_pe
DATA, BASE, SECTIONS = load_pe("client-patched/MapleStory.exe")
def walk_dense(table, count, base_op, default_arm):
    for i in range(count):
        arm = BASE + struct.unpack_from("<I", DATA, foff(table + i*4))[0]
        print(hex(base_op + i), hex(i), hex(arm), "DEFAULT" if arm == default_arm else "")
```

**2. Walk the byte-indexed table.** The one the brief did not know about.

```python
idxs = [DATA[foff(0x1429BBD10 + i)] for i in range(0x31)]        # 0x31 opcodes
arms = [BASE + struct.unpack_from("<I", DATA, foff(0x1429BBCD0 + j*4))[0]
        for j in range(max(idxs) + 1)]                            # 16 dword slots
for i, b in enumerate(idxs):
    print(hex(0x293 + i), "case", hex(b), hex(arms[b]))
```

**3. Resolve a stub to its handler.** Disassemble the arm until it leaves; the non-primitive
call targets are the handlers.

```python
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
MD = Cs(CS_ARCH_X86, CS_MODE_64)
def stub_calls(va, limit=48):
    out = []
    for ins in MD.disasm(DATA[foff(va):foff(va) + limit*8], va):
        if ins.mnemonic == "call" and ins.op_str.startswith("0x"):
            out.append(int(ins.op_str, 16))
        elif ins.mnemonic in ("jmp", "ret"):
            break
    return out
```

**4. Reachability to the path decoder, over `call` AND tail `jmp`.** `reads.calls_of` handles
both, which is why this is built on it rather than on a byte scan for `E8`.

```python
import reads as R
R._load("client-patched/MapleStory.exe")
memo = {}
def reaches_fn(fn, target, depth):
    if fn == target: return True
    if depth <= 0: return False
    if (fn, depth) in memo: return memo[(fn, depth)]
    memo[(fn, depth)] = False                       # cycle guard
    got = any(t == target or reaches_fn(t, target, depth - 1)
              for _, t, _ in R.calls_of(fn))
    memo[(fn, depth)] = got
    return got
```

**5. Recursive descent, for the one function a linear sweep cannot read.** Follows only
intra-extent branches, so data-in-code and a `jmp` into `.themida` stop the walk instead of
desynchronising it.

```python
COND = {"jo","jno","jb","jae","je","jne","jbe","ja","js","jns","jp","jnp","jl","jge","jle","jg"}
def rdisasm(start, end):
    seen, work = {}, [start]
    while work:
        va = work.pop()
        while start <= va < end and va not in seen:
            ins = next(MD.disasm(DATA[foff(va):foff(va)+16], va), None)
            if ins is None: break
            seen[va] = (ins.mnemonic, ins.op_str)
            if ins.mnemonic in COND and ins.op_str.startswith("0x"):
                t = int(ins.op_str, 16)
                if start <= t < end: work.append(t)
                va += ins.size
            elif ins.mnemonic == "jmp":
                t = int(ins.op_str, 16) if ins.op_str.startswith("0x") else None
                if t is not None and start <= t < end:
                    va = t; continue
                break                                # an escape - record it
            elif ins.mnemonic in ("ret", "int3", "ud2"): break
            else: va += ins.size
    return seen
```

**The correctly-shaped stack-slot scan** from section 4, with its own control:

```python
blob = DATA[foff(0x1429D2EE0):foff(0x1429D2EE0) + (0x1429D31C1 - 0x1429D2EE0)]
def slot(disp8):   # ModRM mod=01, rm=101 -> [rbp+disp8]; reg is NOT fixed
    return [i for i in range(len(blob)-1)
            if (blob[i] & 0xC7) == 0x45 and blob[i+1] == disp8]
assert len(slot(0x84)) == 7      # control: [rbp-0x7c], seven known accesses
assert len(slot(0x8C)) == 1      # [rbp-0x74], the opcode: the store, and nothing else
```

---

## 9. The complete map

387 opcodes. `bytes` is the handler's merged `.pdata` extent; `reads` is the number of read
*sites* `tools/reads.py` finds at depth 4 (a site inside a loop reads more than once);
`read types` is those sites in address order, `(a+b)` meaning "through a helper subtree",
`?` meaning a conditional branch precedes it somewhere in the function - a hint, not a
dominator analysis. A row with no handler is a `default:` slot.

Two rows carry numbers that are not real code: `0x033b` and `0x377`, per section 4.

### Table A - `0x1429bb5d0`, 81 entries, `index = opcode - 0x226`

| opcode | idx | stub | handler | bytes | reads | read types (depth 4) | note |
|---|---|---|---|---|---|---|---|
| `0x0226` | 0x00 | `0x1429bb124` | `0x1427847a0` | 451 | 8 | u8 u32 str str (str+raw)? u8? u8? u8? | **UserChat** (`research/user-chat-round2.md`); its id read is special-cased at `0x1429bafcc` |
| `0x0227` | 0x01 | `0x1429bb53f` | *default* | | | | |
| `0x0228` | 0x02 | `0x1429bb53f` | *default* | | | | |
| `0x0229` | 0x03 | `0x1429bb53f` | *default* | | | | |
| `0x022a` | 0x04 | `0x1429bb53f` | *default* | | | | |
| `0x022b` | 0x05 | `0x1429bb53f` | *default* | | | | |
| `0x022c` | 0x06 | `0x1429bb53f` | *default* | | | | |
| `0x022d` | 0x07 | `0x1429bb53f` | *default* | | | | |
| `0x022e` | 0x08 | `0x1429bb53f` | *default* | | | | |
| `0x022f` | 0x09 | `0x1429bb53f` | *default* | | | | |
| `0x0230` | 0x0a | `0x1429bb53f` | *default* | | | | |
| `0x0231` | 0x0b | `0x1429bb134` | `0x142784970` | 5276 | 7 | u8 str (str+raw)? u8? u8? u8? (raw+u8+str)? | the chat handler this server already uses (`research/user-chat-round2.md`) |
| `0x0232` | 0x0c | `0x1429bb144` | `0x142785e20` | 1197 | 2 | u8? str? |  |
| `0x0233` | 0x0d | `0x1429bb154` | `0x1427956b0` | 944 | 10 | raw u32? raw? str? u32? u8? u8? u8? u8? (str+raw)? |  |
| `0x0234` | 0x0e | `0x1429bb164` | `0x142795a70` | 167 | 1 | (raw+u32+str) |  |
| `0x0235` | 0x0f | `0x1429bb174` | `0x14277b0e0` | 50 | 1 | u32 |  |
| `0x0236` | 0x10 | `0x1429bb184` | `0x142792340` | 4499 | 4 | u8? u8? u32? u32? |  |
| `0x0237` | 0x11 | `0x1429bb53f` | *default* | | | | |
| `0x0238` | 0x12 | `0x1429bb1f4` | `0x1427934e0` | 982 | 4 | u8? u8? u32? u32? |  |
| `0x0239` | 0x13 | `0x1429bb204` | `0x1427938c0` | 465 | 1 | u8? |  |
| `0x023a` | 0x14 | `0x1429bb194` | `0x142793aa0` | 353 | 2 | u16? u8? |  |
| `0x023b` | 0x15 | `0x1429bb1a4` | `0x142793c10` | 1296 | 4 | u8? u32? u32? u32? |  |
| `0x023c` | 0x16 | `0x1429bb1b4` | `0x1427948d0` | 1503 | 2 | u8? u32? |  |
| `0x023d` | 0x17 | `0x1429bb1c4` | `0x142794130` | 1030 | 4 | u8? u32? u32? u32? |  |
| `0x023e` | 0x18 | `0x1429bb1d4` | `0x142794540` | 362 | 2 | u8? u32? |  |
| `0x023f` | 0x19 | `0x1429bb1e4` | `0x1427946b0` | 523 | 4 | u8? u32? u8? u32? |  |
| `0x0240` | 0x1a | `0x1429bb214` | `0x142794ec0` | 394 | 9 | u8? u32? u32? u32? (u8)? u32? u32? u32? (u16+u32)? |  |
| `0x0241` | 0x1b | `0x1429bb244` | `0x142795330` | 411 | 9 | u8? u32? u32? u32? (u8)? u32? u32? u32? (u16+u32)? |  |
| `0x0242` | 0x1c | `0x1429bb254` | `0x1427954e0` | 458 | 4 | u8? u32? u32? u8? |  |
| `0x0243` | 0x1d | `0x1429bb264` | `0x142798c20` | - | - | - |  |
| `0x0244` | 0x1e | `0x1429bb274` | `0x142798c30` | - | - | - |  |
| `0x0245` | 0x1f | `0x1429bb284` | `0x142798c40` | 31 | 0 | - |  |
| `0x0246` | 0x20 | `0x1429bb294` | `0x142798c70` | 122 | 2 | u8 u32? |  |
| `0x0247` | 0x21 | `0x1429bb2a4` | `0x14279bc20` | 1293 | 4 | u32 u8? u32? u32? |  |
| `0x0248` | 0x22 | `0x1429bb2b4` | `0x1427922b0` | 138 | 2 | u32 u32 |  |
| `0x0249` | 0x23 | `0x1429bb2c4` | `0x142798cf0` | 81 | 3 | u32 (u32+u8)? (u32+u8)? |  |
| `0x024a` | 0x24 | `0x1429bb2d4` | `0x1427eeb40` | 231 | 1 | u32 |  |
| `0x024b` | 0x25 | `0x1429bb2e4` | `0x1427991b0` | 215 | 1 | u8 |  |
| `0x024c` | 0x26 | `0x1429bb2f4` | `0x142799290` | 809 | 6 | u32 u16 u16 u8? u32? u32? |  |
| `0x024d` | 0x27 | `0x1429bb304` | `0x1427995c0` | 51 | 1 | u32 |  |
| `0x024e` | 0x28 | `0x1429bb314` | `0x14279a6e0` | 1075 | 4 | u32 u8 u32? u32? |  |
| `0x024f` | 0x29 | `0x1429bb53f` | *default* | | | | |
| `0x0250` | 0x2a | `0x1429bb324` | `0x142826ab0` | 1432 | 29 | u8 u8? raw? u32? u32? u32? u32? u32? u8? raw? u32? u32? ... |  |
| `0x0251` | 0x2b | `0x1429bb334` | `0x1427bdd40` | 217 | 3 | u32 str str? |  |
| `0x0252` | 0x2c | `0x1429bb344` | `0x1428341c0` | 95 | 2 | u8? u16? |  |
| `0x0253` | 0x2d | `0x1429bb354` | `0x142834230` | 1968 | 5 | u32 u32 u32 u32 u32 |  |
| `0x0254` | 0x2e | `0x1429bb364` | `0x1428350a0` | 243 | 6 | u32 u8 u32 u32 u32 u32? |  |
| `0x0255` | 0x2f | `0x1429bb374` | `0x142835870` | 36 | 1 | (u8+u32) |  |
| `0x0256` | 0x30 | `0x1429bb384` | `0x1428358e0` | 55 | 1 | u32 |  |
| `0x0257` | 0x31 | `0x1429bb394` | `0x142835930` | 328 | 5 | u8 u32? u32? u8? u32? |  |
| `0x0258` | 0x32 | `0x1429bb3a4` | `0x142835a80` | 155 | 2 | u32 u32 |  |
| `0x0259` | 0x33 | `0x1429bb224` | `0x142795050` | 362 | 6 | u8? u32? u32? u32? (u8)? (u16+u32)? |  |
| `0x025a` | 0x34 | `0x1429bb234` | `0x1427951c0` | 362 | 6 | u8? u32? u32? u32? (u8)? (u16+u32)? |  |
| `0x025b` | 0x35 | `0x1429bb53f` | *default* | | | | |
| `0x025c` | 0x36 | `0x1429bb3b4` | `0x1428482a0` | 550 | 2 | u8 u32 |  |
| `0x025d` | 0x37 | `0x1429bb3c4` | `0x142834060` | 68 | 1 | (u32)? |  |
| `0x025e` | 0x38 | `0x1429bb424` | `0x142833e80` | 155 | 1 | (u32)? |  |
| `0x025f` | 0x39 | `0x1429bb3d4` | `0x142836380` | 38 | 1 | u32 |  |
| `0x0260` | 0x3a | `0x1429bb3e4` | `0x1428363b0` | 672 | 2 | u32? u32? |  |
| `0x0261` | 0x3b | `0x1429bb3f4` | `0x142836660` | 3787 | 8 | u32? u32? u32? u32? u32? u32? u32? u32? |  |
| `0x0262` | 0x3c | `0x1429bb404` | `0x1428377f0` | 289 | 2 | u32? raw? |  |
| `0x0263` | 0x3d | `0x1429bb414` | `0x142837540` | 669 | 2 | u32? u32? |  |
| `0x0264` | 0x3e | `0x1429bb434` | `0x142833ff0` | 29 | 1 | u32 |  |
| `0x0265` | 0x3f | `0x1429bb53f` | *default* | | | | |
| `0x0266` | 0x40 | `0x1429bb444` | `0x142853840` | 664 | 2 | u32 (u32+raw)? |  |
| `0x0267` | 0x41 | `0x1429bb454` | `0x142853ae0` | 290 | 2 | u32 u32? |  |
| `0x0268` | 0x42 | `0x1429bb464` | `0x1428542a0` | 632 | 3 | u32 u8? u32? |  |
| `0x0269` | 0x43 | `0x1429bb474` | `0x142854520` | 481 | 2 | u32 u32? |  |
| `0x026a` | 0x44 | `0x1429bb484` | `0x142854710` | 266 | 7 | u32 u32 u32 u32? u32? u32? u32? |  |
| `0x026b` | 0x45 | `0x1429bb494` | `qword ptr [rax + 0x90]` | - | - | - |  |
| `0x026c` | 0x46 | `0x1429bb4a8` | `0x1427fb990` | 108 | 2 | u8 u32 |  |
| `0x026d` | 0x47 | `0x1429bb4b8` | `0x1427fba10` | 79 | 1 | u32 |  |
| `0x026e` | 0x48 | `0x1429bb4d8` | `0x1427fba70` | 35 | 1 | u8 |  |
| `0x026f` | 0x49 | `0x1429bb4e8` | `0x1427fbaa0` | 35 | 1 | u8 |  |
| `0x0270` | 0x4a | `0x1429bb4c8` | `0x142859c60` | 106 | 1 | u8? |  |
| `0x0271` | 0x4b | `0x1429bb4f8` | `0x142859f00` | 103 | 1 | u8 |  |
| `0x0272` | 0x4c | `0x1429bb508` | `0x142859f70` | 2346 | 2 | u32 u32 |  |
| `0x0273` | 0x4d | `0x1429bb518` | `0x14285b4f0` | 168 | 2 | u32 u32 |  |
| `0x0274` | 0x4e | `0x1429bb525` | `0x14285f040` | 70 | 2 | u32 u8 |  |
| `0x0275` | 0x4f | `0x1429bb53f` | *default* | | | | |
| `0x0276` | 0x50 | `0x1429bb532` | `0x14285f8e0` | 551 | 1 | u32 |  |

### Table B - `0x1429bbc34`, 39 entries, `index = opcode - 0x29e`

| opcode | idx | stub | handler | bytes | reads | read types (depth 4) | note |
|---|---|---|---|---|---|---|---|
| `0x029e` | 0x00 | `0x1429bb95e` | `0x1429d2ee0` | 4631 | 3 | u16 (u32+u8) (u32+u16+u8+u64) | **REMOTE ATTACK** - one of four, all one handler |
| `0x029f` | 0x01 | `0x1429bb95e` | `0x1429d2ee0` | 4631 | 3 | u16 (u32+u8) (u32+u16+u8+u64) | **REMOTE ATTACK** |
| `0x02a0` | 0x02 | `0x1429bb95e` | `0x1429d2ee0` | 4631 | 3 | u16 (u32+u8) (u32+u16+u8+u64) | **REMOTE ATTACK** |
| `0x02a1` | 0x03 | `0x1429bb95e` | `0x1429d2ee0` | 4631 | 3 | u16 (u32+u8) (u32+u16+u8+u64) | **REMOTE ATTACK** |
| `0x02a2` | 0x04 | `0x1429bb970` | `0x1429d4100` | 646 | 4 | u32 u8 u16? u8? |  |
| `0x02a3` | 0x05 | `0x1429bb986` | `0x1429d4390` | 736 | 5 | u8 u8 u32? u16? u8? |  |
| `0x02a4` | 0x06 | `0x1429bb99c` | `0x1429d4680` | 565 | 2 | u32 u32 |  |
| `0x02a5` | 0x07 | `0x1429bb9b2` | `0x1429d48c0` | 1615 | 1 | (u32+u8+raw+u64) | remote user hit, 147-byte HITINFO (`research/user-hit.md` section 5.3) |
| `0x02a6` | 0x08 | `0x1429bb9c8` | `0x1427862e0` | 105 | 3 | u32 u32 u8 | shares its handler with local `0x02C5` |
| `0x02a7` | 0x09 | `0x1429bb9de` | `0x142786350` | 147 | 2 | u32 u32 | shares its handler with local `0x02D0` |
| `0x02a8` | 0x0a | `0x1429bb9f4` | `0x1429d4f20` | 32 | 1 | u32 |  |
| `0x02a9` | 0x0b | `0x1429bba0a` | `0x1429d4f50` | - | - | - |  |
| `0x02aa` | 0x0c | `0x1429bbb10` | *default* | | | | |
| `0x02ab` | 0x0d | `0x1429bbb10` | *default* | | | | |
| `0x02ac` | 0x0e | `0x1429bba20` | `0x1429d4f60` | 96 | 3 | u32 u32 u32 |  |
| `0x02ad` | 0x0f | `0x1429bbb10` | *default* | | | | |
| `0x02ae` | 0x10 | `0x1429bbb10` | *default* | | | | |
| `0x02af` | 0x11 | `0x1429bba36` | `0x1427863f0` | 45228 | 239 | u8 str? u32? u32? u32? u8? u32? u32? u8? u32? u32? str? ... | `USER_EFFECT_REMOTE`, effect 0 = LevelUp (`research/level-up.md` section 6) |
| `0x02b0` | 0x12 | `0x1429bbb10` | *default* | | | | |
| `0x02b1` | 0x13 | `0x1429bbb10` | *default* | | | | |
| `0x02b2` | 0x14 | `0x1429bbb10` | *default* | | | | |
| `0x02b3` | 0x15 | `0x1429bbb10` | *default* | | | | |
| `0x02b4` | 0x16 | `0x1429bbb10` | *default* | | | | |
| `0x02b5` | 0x17 | `0x1429bbb10` | *default* | | | | |
| `0x02b6` | 0x18 | `0x1429bbb10` | *default* | | | | |
| `0x02b7` | 0x19 | `0x1429bba5c` | `0x1429d5f70` | 148 | 1 | u32 |  |
| `0x02b8` | 0x1a | `0x1429bbb10` | *default* | | | | |
| `0x02b9` | 0x1b | `0x1429bbb10` | *default* | | | | |
| `0x02ba` | 0x1c | `0x1429bbb10` | *default* | | | | |
| `0x02bb` | 0x1d | `0x1429bba72` | `0x1429d6010` | 233 | 4 | u32 f64 u32 u8 |  |
| `0x02bc` | 0x1e | `0x1429bba88` | `0x1429d6100` | 63 | 1 | u32 |  |
| `0x02bd` | 0x1f | `0x1429bba9e` | `0x1429d7130` | 47 | 1 | u32 |  |
| `0x02be` | 0x20 | `0x1429bbb10` | *default* | | | | |
| `0x02bf` | 0x21 | `0x1429bbab1` | `0x1429de1b0` | 1713 | 10 | u32? u32? u32? u32? u8? u32? u8? (u32+u8+raw)? u32? (u32+u16+u8)? |  |
| `0x02c0` | 0x22 | `0x1429bbac4` | `0x1429deb60` | 38 | 3 | u32 u32 u8 |  |
| `0x02c1` | 0x23 | `0x1429bbad7` | `0x1429deae0` | 78 | 8 | u32 u8 u32 u32 u8 u32 u32 u32 |  |
| `0x02c2` | 0x24 | `0x1429bbb10` | *default* | | | | |
| `0x02c3` | 0x25 | `0x1429bbb10` | *default* | | | | |
| `0x02c4` | 0x26 | `0x1429bba4c` | `0x1429d5d90` | 464 | 12 | u32 u32 u32 u32 u32 u32 u32 u8 u32 u32 u32 (u32) |  |

### Table C - `0x1429bbcd0` via byte index `0x1429bbd10`, 49 opcodes, `index = opcode - 0x293`

| opcode | idx | stub | handler | bytes | reads | read types (depth 4) | note |
|---|---|---|---|---|---|---|---|
| `0x0293` | 0x00 | `0x1429bbb38` | `0x1429d2e70` | 102 | 1 | (u32+u16+u8)? | **REMOTE USER MOVE** - reaches `FUN_1404b2630`, the 79-command path decoder |
| `0x0294` | 0x01 | `0x1429bbc08` | *default* | | | | |
| `0x0295` | 0x02 | `0x1429bbc08` | *default* | | | | |
| `0x0296` | 0x03 | `0x1429bbc08` | *default* | | | | |
| `0x0297` | 0x04 | `0x1429bbc08` | *default* | | | | |
| `0x0298` | 0x05 | `0x1429bbc08` | *default* | | | | |
| `0x0299` | 0x06 | `0x1429bbc08` | *default* | | | | |
| `0x029a` | 0x07 | `0x1429bbc08` | *default* | | | | |
| `0x029b` | 0x08 | `0x1429bbc08` | *default* | | | | |
| `0x029c` | 0x09 | `0x1429bbc08` | *default* | | | | |
| `0x029d` | 0x0a | `0x1429bbc08` | *default* | | | | |
| `0x029e` | 0x0b | `0x1429bbc08` | *default* | | | | |
| `0x029f` | 0x0c | `0x1429bbc08` | *default* | | | | |
| `0x02a0` | 0x0d | `0x1429bbc08` | *default* | | | | |
| `0x02a1` | 0x0e | `0x1429bbc08` | *default* | | | | |
| `0x02a2` | 0x0f | `0x1429bbc08` | *default* | | | | |
| `0x02a3` | 0x10 | `0x1429bbc08` | *default* | | | | |
| `0x02a4` | 0x11 | `0x1429bbc08` | *default* | | | | |
| `0x02a5` | 0x12 | `0x1429bbc08` | *default* | | | | |
| `0x02a6` | 0x13 | `0x1429bbc08` | *default* | | | | |
| `0x02a7` | 0x14 | `0x1429bbc08` | *default* | | | | |
| `0x02a8` | 0x15 | `0x1429bbc08` | *default* | | | | |
| `0x02a9` | 0x16 | `0x1429bbc08` | *default* | | | | |
| `0x02aa` | 0x17 | `0x1429bbb78` | `0x1429d6f50` | 187 | 2 | u32? u32? |  |
| `0x02ab` | 0x18 | `0x1429bbc08` | *default* | | | | |
| `0x02ac` | 0x19 | `0x1429bbc08` | *default* | | | | |
| `0x02ad` | 0x1a | `0x1429bbb48` | `0x1429d4fd0` | 658 | 3 | u32 u32 u8? |  |
| `0x02ae` | 0x1b | `0x1429bbb58` | `0x1429d5290` | 888 | 18 | u8 (u8+u32+raw)? u8? u8? u8? raw? raw? u32? u8? raw? raw? u32? ... |  |
| `0x02af` | 0x1c | `0x1429bbc08` | *default* | | | | |
| `0x02b0` | 0x1d | `0x1429bbb68` | `0x1429d62f0` | 516 | 3 | (raw+u16+u32+u8) u16 u8 |  |
| `0x02b1` | 0x1e | `0x1429bbb88` | `0x1429d6500` | 2621 | 3 | raw (u32)? u8? |  |
| `0x02b2` | 0x1f | `0x1429bbbbc` | `0x1429d5610` | 235 | 2 | u32 u32 |  |
| `0x02b3` | 0x20 | `0x1429bbb95` | `0x1429d5710` | 810 | 11 | u32 str u16? u8? u16? u8? u32? u32? u8? u32? (u8)? |  |
| `0x02b4` | 0x21 | `0x1429bbba2` | `0x1429d5a40` | 115 | 1 | str |  |
| `0x02b5` | 0x22 | `0x1429bbbaf` | `0x1429d5ac0` | 714 | 9 | u16 u8 u16 u8 u32 u32? u8? u32? (u8)? |  |
| `0x02b6` | 0x23 | `0x1429bbbc9` | `0x1429d7020` | 195 | 3 | u32 u32 u8 |  |
| `0x02b7` | 0x24 | `0x1429bbc08` | *default* | | | | |
| `0x02b8` | 0x25 | `0x1429bbc08` | *default* | | | | |
| `0x02b9` | 0x26 | `0x1429bbbd6` | `0x1429d70f0` | 54 | 0 | - |  |
| `0x02ba` | 0x27 | `0x1429bbc08` | *default* | | | | |
| `0x02bb` | 0x28 | `0x1429bbc08` | *default* | | | | |
| `0x02bc` | 0x29 | `0x1429bbc08` | *default* | | | | |
| `0x02bd` | 0x2a | `0x1429bbc08` | *default* | | | | |
| `0x02be` | 0x2b | `0x1429bbbe3` | `0x1429ddb20` | 95 | 1 | u64 |  |
| `0x02bf` | 0x2c | `0x1429bbc08` | *default* | | | | |
| `0x02c0` | 0x2d | `0x1429bbc08` | *default* | | | | |
| `0x02c1` | 0x2e | `0x1429bbc08` | *default* | | | | |
| `0x02c2` | 0x2f | `0x1429bbbf0` | `0x1429d7170` | 157 | 3 | str u16? u16? |  |
| `0x02c3` | 0x30 | `0x1429bbbfd` | `0x1429d7220` (+1) | 34 | 1 | u8 |  |

### Table D - `0x14289d660`, 218 entries, `index = opcode - 0x2c5`

| opcode | idx | stub | handler | bytes | reads | read types (depth 4) | note |
|---|---|---|---|---|---|---|---|
| `0x02c5` | 0x00 | `0x14289a419` | `0x1427862e0` | 105 | 3 | u32 u32 u8 | shares its handler with remote `0x02A6` |
| `0x02c6` | 0x01 | `0x14289d5f5` | *default* | | | | |
| `0x02c7` | 0x02 | `0x14289d5f5` | *default* | | | | |
| `0x02c8` | 0x03 | `0x14289d5f5` | *default* | | | | |
| `0x02c9` | 0x04 | `0x14289d5f5` | *default* | | | | |
| `0x02ca` | 0x05 | `0x14289d5f5` | *default* | | | | |
| `0x02cb` | 0x06 | `0x14289d5f5` | *default* | | | | |
| `0x02cc` | 0x07 | `0x14289d5f5` | *default* | | | | |
| `0x02cd` | 0x08 | `0x14289d5f5` | *default* | | | | |
| `0x02ce` | 0x09 | `0x14289d5f5` | *default* | | | | |
| `0x02cf` | 0x0a | `0x14289d5f5` | *default* | | | | |
| `0x02d0` | 0x0b | `0x14289a429` | `0x142786350` | 147 | 2 | u32 u32 | shares its handler with remote `0x02A7` |
| `0x02d1` | 0x0c | `0x14289a439` | `0x1427863f0` | 45228 | 239 | u8 str? u32? u32? u32? u8? u32? u32? u8? u32? u32? str? ... | `USER_EFFECT_LOCAL`, effect 0 = LevelUp (`research/level-up.md` section 6) |
| `0x02d2` | 0x0d | `0x14289a449` | `0x142cc4430` (+1) | 55 | 0 | - |  |
| `0x02d3` | 0x0e | `0x14289d5f5` | *default* | | | | |
| `0x02d4` | 0x0f | `0x14289a53b` | `0x14289db10` | 554 | 1 | u32 |  |
| `0x02d5` | 0x10 | `0x14289a54b` | `0x1408a9e40` (+1) | 75 | 0 | - |  |
| `0x02d6` | 0x11 | `0x14289a591` | `0x14289dd90` | 9212 | 18 | u8? u32? u32? u32? u8? u32? u8? u16? u32? u32? u16? u32? ... |  |
| `0x02d7` | 0x12 | `0x14289a5a1` | `0x1428d93b0` | 1203 | 0 | - |  |
| `0x02d8` | 0x13 | `0x14289a5be` | `0x1428a1460` | 1235 | 3 | raw u8? u16? |  |
| `0x02d9` | 0x14 | `0x14289a5ce` | *inline* | | | |  |
| `0x02da` | 0x15 | `0x14289a7b6` | `0x1428a1a70` | 441 | 6 | str u16 u16 u8 u32? u32? |  |
| `0x02db` | 0x16 | `0x14289a7c6` | `0x1403ede40` (+2) | 314 | 0 | - |  |
| `0x02dc` | 0x17 | `0x14289a81c` | `0x1403ede40` (+3) | 314 | 0 | - |  |
| `0x02dd` | 0x18 | `0x14289a87e` | `0x1428a1d20` | 1725 | 20 | u32 u32? u32? u32? u32? u32? u32? u32? u32? u8? u32? u32? ... |  |
| `0x02de` | 0x19 | `0x14289d615` | *no-op arm* | | | | |
| `0x02df` | 0x1a | `0x14289a88e` | `0x142cb1020` | 13577 | 0 | - |  |
| `0x02e0` | 0x1b | `0x14289a8cd` | `0x142cb4530` | 5997 | 0 | - |  |
| `0x02e1` | 0x1c | `0x14289a8f4` | `0x1428a2480` | 933 | 4 | u32 u32 u32 u32? |  |
| `0x02e2` | 0x1d | `0x14289a914` | `0x142cc1c30` (+2) | 184 | 0 | - |  |
| `0x02e3` | 0x1e | `0x14289a950` | `0x1428a2ad0` | 903 | 5 | u8? u8? u8? u8? u8? |  |
| `0x02e4` | 0x1f | `0x14289a960` | `0x142cc3d60` (+2) | - | - | - |  |
| `0x02e5` | 0x20 | `0x14289a9d1` | `0x1428eb430` | 224 | 0 | - |  |
| `0x02e6` | 0x21 | `0x14289aa14` | `0x142e52ed0` (+1) | 4890 | 0 | - |  |
| `0x02e7` | 0x22 | `0x14289a99a` | `0x1428a2f80` | 371 | 0 | - |  |
| `0x02e8` | 0x23 | `0x14289aad5` | `0x1428a3200` | 1009 | 4 | u8 u32 u32 u8 |  |
| `0x02e9` | 0x24 | `0x14289aae5` | `0x1428a3600` | 468 | 3 | u32 u32 u8 |  |
| `0x02ea` | 0x25 | `0x14289aaf5` | `0x1428a37e0` | 541 | 2 | u32 u32 |  |
| `0x02eb` | 0x26 | `0x14289ab05` | `0x1428a3a10` | 463 | 3 | u32 u32 u32 |  |
| `0x02ec` | 0x27 | `0x14289ab15` | `0x142cb1020` | 13577 | 0 | - |  |
| `0x02ed` | 0x28 | `0x14289ab3b` | `0x14019a260` (+1) | 485 | 0 | - |  |
| `0x02ee` | 0x29 | `0x14289aba0` | `0x1415eca30` | 471 | 0 | - |  |
| `0x02ef` | 0x2a | `0x14289abca` | `0x1415ecd00` | 126 | 0 | - |  |
| `0x02f0` | 0x2b | `0x14289abf9` | `0x142ef6a98` (+6) | 506 | 0 | - |  |
| `0x02f1` | 0x2c | `0x14289acfa` | `0x1428ee5d0` | 505 | 2 | str u32 |  |
| `0x02f2` | 0x2d | `0x14289ad0a` | `0x1428ee7d0` | 938 | 1 | u32 |  |
| `0x02f3` | 0x2e | `0x14289ad1a` | `0x1428a3bf0` | 1701 | 5 | u32 u32 u32 u32 u32 |  |
| `0x02f4` | 0x2f | `0x14289ad2a` | `0x14295e330` (+2) | 405 | 0 | - |  |
| `0x02f5` | 0x30 | `0x14289ae0b` | `0x14279c140` | 51 | 1 | (u32+u16+u8)? | local move-path packet - `FUN_1409c70d0` passes arg3 **1**, so the trailing nibble block IS read |
| `0x02f6` | 0x31 | `0x14289ae1b` | `0x142cd87b0` | - | - | - |  |
| `0x02f7` | 0x32 | `0x14289d5f5` | *default* | | | | |
| `0x02f8` | 0x33 | `0x14289d615` | *no-op arm* | | | | |
| `0x02f9` | 0x34 | `0x14289caf4` | `0x1428fad20` (+1) | 274 | 0 | - |  |
| `0x02fa` | 0x35 | `0x14289a904` | `0x1428a2830` | 586 | 4 | u32 u32 u8 u8 |  |
| `0x02fb` | 0x36 | `0x14289ae5f` | *inline* | | | |  |
| `0x02fc` | 0x37 | `0x14289d5f5` | *default* | | | | |
| `0x02fd` | 0x38 | `0x14289ae73` | `0x142954270` (+1) | 40 | 0 | - |  |
| `0x02fe` | 0x39 | `0x14289ae8f` | `0x1428fb0f0` | 606 | 2 | u8 u32 |  |
| `0x02ff` | 0x3a | `0x14289ae9f` | `0x14298fa30` | 299 | 0 | - |  |
| `0x0300` | 0x3b | `0x14289aeaf` | `0x14298fb70` | 141 | 1 | u32 |  |
| `0x0301` | 0x3c | `0x14289aebf` | `0x14019a260` (+1) | 485 | 0 | - |  |
| `0x0302` | 0x3d | `0x14289af3e` | `0x14019a260` (+1) | 485 | 0 | - |  |
| `0x0303` | 0x3e | `0x14289af84` | `0x1428fae50` | 601 | 3 | u16 u16? u32? |  |
| `0x0304` | 0x3f | `0x14289af94` | `0x142cdec50` | - | - | - |  |
| `0x0305` | 0x40 | `0x14289afaf` | `0x1428a6720` | 639 | 2 | u32 u8 |  |
| `0x0306` | 0x41 | `0x14289b014` | `0x142e52ed0` (+3) | 4890 | 0 | - |  |
| `0x0307` | 0x42 | `0x14289b0a3` | `0x1401c00e0` (+1) | 102 | 0 | - |  |
| `0x0308` | 0x43 | `0x14289b168` | `0x141892a90` (+1) | 124 | 0 | - |  |
| `0x0309` | 0x44 | `0x14289b1fa` | `0x142cb9550` (+3) | - | - | - |  |
| `0x030a` | 0x45 | `0x14289b2f8` | `0x142bf3f70` (+7) | 1062 | 0 | - |  |
| `0x030b` | 0x46 | `0x14289b421` | `0x1413b8e00` | - | - | - |  |
| `0x030c` | 0x47 | `0x14289b452` | `0x142cb1020` (+2) | 13577 | 0 | - |  |
| `0x030d` | 0x48 | `0x14289b153` | *inline* | | | |  |
| `0x030e` | 0x49 | `0x14289b4b9` | *inline* | | | |  |
| `0x030f` | 0x4a | `0x14289b4c6` | `0x142a33fd0` (+1) | - | - | - |  |
| `0x0310` | 0x4b | `0x14289b4f7` | `0x1427703d0` (+1) | 99 | 0 | - |  |
| `0x0311` | 0x4c | `0x14289afbf` | `0x1428e1fa0` | 6514 | 46 | u16? u32? u32? u32? u32? u16? u8? u16? u16? u32? u16? u16? ... |  |
| `0x0312` | 0x4d | `0x14289afcf` | `0x141001fc0` | - | - | - |  |
| `0x0313` | 0x4e | `0x14289b543` | `0x142903600` | 1724 | 2 | u8 u32 |  |
| `0x0314` | 0x4f | `0x14289b553` | `0x140f810b0` (+12) | - | - | - |  |
| `0x0315` | 0x50 | `0x14289b656` | `0x142cbecc0` (+5) | 46 | 0 | - |  |
| `0x0316` | 0x51 | `0x14289b7c7` | `0x1429040b0` | 1100 | 6 | u16 u32? u32? u32? u32? u32? |  |
| `0x0317` | 0x52 | `0x14289b7d7` | `0x14019b780` (+5) | 380 | 0 | - |  |
| `0x0318` | 0x53 | `0x14289b8be` | `0x1429e3ef0` (+1) | - | - | - |  |
| `0x0319` | 0x54 | `0x14289b9ea` | `qword ptr [rax + 0x100]` (+5) | - | - | - |  |
| `0x031a` | 0x55 | `0x14289a7a6` | `0x1428e3ba0` | 397 | 3 | u32 u8 u32? |  |
| `0x031b` | 0x56 | `0x14289bab0` | `0x142797050` (+3) | 892 | 0 | - |  |
| `0x031c` | 0x57 | `0x14289bb92` | `0x14276df20` (+4) | - | - | - |  |
| `0x031d` | 0x58 | `0x14289bbef` | `0x142cf6a50` (+3) | - | - | - |  |
| `0x031e` | 0x59 | `0x14289bce6` | `0x141ef2180` (+2) | 110 | 0 | - |  |
| `0x031f` | 0x5a | `0x14289bd43` | *inline* | | | |  |
| `0x0320` | 0x5b | `0x14289bd57` | `qword ptr [r10 + 0x70]` | - | - | - |  |
| `0x0321` | 0x5c | `0x14289bd94` | `0x142906da0` | 835 | 9 | str u16 u16 u16 u16 u16 u16 u16 u16 |  |
| `0x0322` | 0x5d | `0x14289bda4` | `qword ptr [rax + 0x118]` | - | - | - |  |
| `0x0323` | 0x5e | `0x14289d615` | *no-op arm* | | | | |
| `0x0324` | 0x5f | `0x14289be90` | `0x142908730` | 746 | 1 | u32? |  |
| `0x0325` | 0x60 | `0x14289bea0` | `0x14019a260` (+1) | 485 | 0 | - |  |
| `0x0326` | 0x61 | `0x14289bef1` | `0x14019a260` (+1) | 485 | 0 | - |  |
| `0x0327` | 0x62 | `0x14289bf28` | `0x142961650` (+2) | 106 | 0 | - |  |
| `0x0328` | 0x63 | `0x14289c00e` | `0x141e768f0` (+2) | 127 | 0 | - |  |
| `0x0329` | 0x64 | `0x14289c07a` | `0x1406ed520` (+2) | 106 | 0 | - |  |
| `0x032a` | 0x65 | `0x14289c113` | `0x14019f2c0` (+1) | 404 | 0 | - |  |
| `0x032b` | 0x66 | `0x14289c18f` | `0x1428fb960` | 3960 | 0 | - |  |
| `0x032c` | 0x67 | `0x14289c1ca` | `0x14290cd80` | 905 | 1 | u32 |  |
| `0x032d` | 0x68 | `0x14289c1da` | `0x142e52ed0` (+2) | 4890 | 0 | - |  |
| `0x032e` | 0x69 | `0x14289c270` | `0x14290a1a0` | 940 | 10 | u8 u32? u8? u32? str? u32? u32? u32? str? u32? |  |
| `0x032f` | 0x6a | `0x14289c778` | `0x141025a00` | 3292 | 34 | u8 u32? u8? u32? u32? u32? u8? u32? u32? u32? u8? u32? ... |  |
| `0x0330` | 0x6b | `0x14289c280` | `0x141892840` (+1) | 66 | 0 | - |  |
| `0x0331` | 0x6c | `0x14289c37c` | `0x14290d4e0` | 961 | 11 | u32? u32? str? str? u8? u32? u8? u8? u32? u8? (u8+u32+raw)? |  |
| `0x0332` | 0x6d | `0x14289c38c` | `0x141edf840` | - | - | - |  |
| `0x0333` | 0x6e | `0x14289c3a6` | `0x1407e7d10` (+1) | - | - | - |  |
| `0x0334` | 0x6f | `0x14289c3ee` | `0x141e768f0` (+2) | 127 | 0 | - |  |
| `0x0335` | 0x70 | `0x14289c44e` | `0x141e768f0` (+1) | 127 | 0 | - |  |
| `0x0336` | 0x71 | `0x14289c4a8` | `0x14290dd60` | 354 | 0 | - |  |
| `0x0337` | 0x72 | `0x14289c4b5` | `0x142992960` | 1287 | 10 | u32? u32? u8? u32? u32? u32? u32? u32? u32? u32? |  |
| `0x0338` | 0x73 | `0x14289c4c5` | `0x1429934a0` | 730 | 5 | u32 u32 u32? u32? u32? |  |
| `0x0339` | 0x74 | `0x14289c4d5` | `0x142993780` | 2076 | 2 | u32 u32 |  |
| `0x033a` | 0x75 | `0x14289a6c1` | `0x1423d58b0` | 1044 | 0 | - |  |
| `0x033b` | 0x76 | `0x14289a759` | `0x1428e3ad0` | 201 | 0 | - |  |
| `0x033c` | 0x77 | `0x14289a769` | `0x140fcec30` | 628 | 0 | - |  |
| `0x033d` | 0x78 | `0x14289c4e5` | `0x1429e3ef0` | - | - | - |  |
| `0x033e` | 0x79 | `0x14289c514` | `0x1407f7900` (+3) | - | - | - |  |
| `0x033f` | 0x7a | `0x14289c6b8` | `0x1428437d0` | 75 | 2 | u64 u8 |  |
| `0x0340` | 0x7b | `0x14289c6c8` | `0x1406e88d0` (+1) | 266 | 0 | - |  |
| `0x0341` | 0x7c | `0x14289c6e7` | *inline* | | | |  |
| `0x0342` | 0x7d | `0x14289d615` | *no-op arm* | | | | |
| `0x0343` | 0x7e | `0x14289c700` | `0x142bf3f70` (+1) | 1062 | 0 | - |  |
| `0x0344` | 0x7f | `0x14289d615` | *no-op arm* | | | | |
| `0x0345` | 0x80 | `0x14289b4b9` | *inline* | | | |  |
| `0x0346` | 0x81 | `0x14289c738` | `0x142919080` | 1147 | 3 | u16 u16 u8 |  |
| `0x0347` | 0x82 | `0x14289c758` | `0x14291a2c0` | 525 | 4 | u32? u32? u32? raw? |  |
| `0x0348` | 0x83 | `0x14289c768` | `0x14291a4e0` | 567 | 6 | u32 u32 u32 u32? raw? u8? |  |
| `0x0349` | 0x84 | `0x14289c748` | `0x142919510` | 636 | 1 | u32 |  |
| `0x034a` | 0x85 | `0x14289d5f5` | *default* | | | | |
| `0x034b` | 0x86 | `0x14289c785` | `0x14291b050` | 2461 | 3 | u32? u16? (u32+raw+u16+u8)? |  |
| `0x034c` | 0x87 | `0x14289c795` | `0x14291bef0` | 632 | 3 | u8 (u16+u32)? u32? |  |
| `0x034d` | 0x88 | `0x14289d615` | *no-op arm* | | | | |
| `0x034e` | 0x89 | `0x14289c7a5` | *inline* | | | |  |
| `0x034f` | 0x8a | `0x14289c7b9` | `0x14291f2f0` | 450 | 5 | u8? u32? u32? u32? u32? |  |
| `0x0350` | 0x8b | `0x14289c7c9` | `0x142d0b290` (+1) | 71 | 0 | - |  |
| `0x0351` | 0x8c | `0x14289c7fb` | `0x142d9cd90` | 935 | 0 | - |  |
| `0x0352` | 0x8d | `0x14289c82b` | `0x14291f740` | 410 | 1 | str |  |
| `0x0353` | 0x8e | `0x14289cc34` | `0x142cbe730` (+3) | - | - | - |  |
| `0x0354` | 0x8f | `0x14289d615` | *no-op arm* | | | | |
| `0x0355` | 0x90 | `0x14289d5f5` | *default* | | | | |
| `0x0356` | 0x91 | `0x14289c8ff` | `0x142907cd0` | 619 | 2 | u32 u32 |  |
| `0x0357` | 0x92 | `0x14289c90f` | `0x142d09590` | - | - | - |  |
| `0x0358` | 0x93 | `0x14289c83b` | `0x1428b3da0` | 417 | 4 | u32 u32 u8 u8 |  |
| `0x0359` | 0x94 | `0x14289c84b` | `0x1428349f0` | 229 | 5 | u32 u8 u8 u32 u32 |  |
| `0x035a` | 0x95 | `0x14289c94c` | `0x140fb7330` (+1) | 1267 | 0 | - |  |
| `0x035b` | 0x96 | `0x14289c992` | `0x1428485b0` | 1403 | 0 | - |  |
| `0x035c` | 0x97 | `0x14289d615` | *no-op arm* | | | | |
| `0x035d` | 0x98 | `0x14289c85b` | `0x1428b3f50` | 797 | 4 | u32 u32 raw u32 |  |
| `0x035e` | 0x99 | `0x14289c87b` | `0x1428b4b40` | 473 | 0 | - |  |
| `0x035f` | 0x9a | `0x14289c86b` | `0x1428b4280` | 704 | 2 | u8 u32 |  |
| `0x0360` | 0x9b | `0x14289c8bb` | `0x141d2efc0` | 153 | 0 | - |  |
| `0x0361` | 0x9c | `0x14289c99f` | `0x1408f6690` (+1) | 73 | 0 | - |  |
| `0x0362` | 0x9d | `0x14289cb2b` | `0x1407e8ca0` (+1) | - | - | - |  |
| `0x0363` | 0x9e | `0x14289cb9b` | `0x1429237b0` | 936 | 3 | u32 u32 u32 |  |
| `0x0364` | 0x9f | `0x14289d615` | *no-op arm* | | | | |
| `0x0365` | 0xa0 | `0x14289cbab` | `0x1429274f0` | 713 | 5 | str? u32? u32? u32? u32? |  |
| `0x0366` | 0xa1 | `0x14289cbbb` | *inline* | | | |  |
| `0x0367` | 0xa2 | `0x14289cbcf` | `0x1428da150` | 4072 | 0 | - |  |
| `0x0368` | 0xa3 | `0x14289cbf6` | `0x142927830` | 406 | 1 | u32 |  |
| `0x0369` | 0xa4 | `0x14289d615` | *no-op arm* | | | | |
| `0x036a` | 0xa5 | `0x14289cc06` | *inline* | | | |  |
| `0x036b` | 0xa6 | `0x14289cc13` | `0x1429e3ef0` | - | - | - |  |
| `0x036c` | 0xa7 | `0x14289d615` | *no-op arm* | | | | |
| `0x036d` | 0xa8 | `0x14289d615` | *no-op arm* | | | | |
| `0x036e` | 0xa9 | `0x14289d615` | *no-op arm* | | | | |
| `0x036f` | 0xaa | `0x14289cd5d` | `0x140495990` (+1) | 1697 | 0 | - |  |
| `0x0370` | 0xab | `0x14289ce61` | `0x142961870` (+3) | 106 | 0 | - |  |
| `0x0371` | 0xac | `0x14289cf5e` | `qword ptr [rip + 0x9c5e2e]` (+2) | - | - | - |  |
| `0x0372` | 0xad | `0x14289d615` | *no-op arm* | | | | |
| `0x0373` | 0xae | `0x14289d615` | *no-op arm* | | | | |
| `0x0374` | 0xaf | `0x14289d615` | *no-op arm* | | | | |
| `0x0375` | 0xb0 | `0x14289d615` | *no-op arm* | | | | |
| `0x0376` | 0xb1 | `0x14289cfc0` | `0x1429e3ef0` | - | - | - |  |
| `0x0377` | 0xb2 | `0x14289cfd1` | `0x14292eaa0` | 96 | 0 | - |  |
| `0x0378` | 0xb3 | `0x14289d615` | *no-op arm* | | | | |
| `0x0379` | 0xb4 | `0x14289d615` | *no-op arm* | | | | |
| `0x037a` | 0xb5 | `0x14289cfe1` | `0x14292c720` | 1628 | 3 | u8 u32? u32? |  |
| `0x037b` | 0xb6 | `0x14289cff1` | `0x14292cd90` | 492 | 2 | u32? (raw)? |  |
| `0x037c` | 0xb7 | `0x14289d001` | `0x14292cf90` | 1126 | 6 | u32? u32? u32? u32? u32? u32? |  |
| `0x037d` | 0xb8 | `0x14289d011` | `0x14292d8d0` | 1549 | 2 | u32 u8 |  |
| `0x037e` | 0xb9 | `0x14289d021` | `0x14292e500` | 905 | 6 | u32 u32 u32 u32 u32? u32? |  |
| `0x037f` | 0xba | `0x14289cc24` | `0x142921a20` | 689 | 2 | u32 u8 |  |
| `0x0380` | 0xbb | `0x14289d031` | `0x14292eb10` | 700 | 2 | u32 u32? |  |
| `0x0381` | 0xbc | `0x14289d041` | `0x14292ede0` | 2072 | 1 | u32 |  |
| `0x0382` | 0xbd | `0x14289d051` | `0x14292f600` | 1552 | 2 | str u32 |  |
| `0x0383` | 0xbe | `0x14289d061` | `0x14292fd50` | 1062 | 5 | u32 u32 str str u8 |  |
| `0x0384` | 0xbf | `0x14289d071` | `0x142930180` | 1005 | 2 | u32 u32 |  |
| `0x0385` | 0xc0 | `0x14289d081` | `0x142e52ed0` (+5) | 4890 | 0 | - |  |
| `0x0386` | 0xc1 | `0x14289d140` | `0x142930680` | 4840 | 6 | u32 u32 u32 u32 u32 u32 |  |
| `0x0387` | 0xc2 | `0x14289d615` | *no-op arm* | | | | |
| `0x0388` | 0xc3 | `0x14289d615` | *no-op arm* | | | | |
| `0x0389` | 0xc4 | `0x14289d150` | `0x142931990` | 799 | 2 | str str |  |
| `0x038a` | 0xc5 | `0x14289d160` | `0x142cbecc0` (+1) | 46 | 0 | - |  |
| `0x038b` | 0xc6 | `0x14289d615` | *no-op arm* | | | | |
| `0x038c` | 0xc7 | `0x14289d194` | `0x142578d90` | 258 | 0 | - |  |
| `0x038d` | 0xc8 | `0x14289d217` | `0x142cf6960` (+1) | 37 | 0 | - |  |
| `0x038e` | 0xc9 | `0x14289d294` | `0x142934dc0` | 2438 | 4 | u32 u32 u32 u32 |  |
| `0x038f` | 0xca | `0x14289d615` | *no-op arm* | | | | |
| `0x0390` | 0xcb | `0x14289d2a4` | `0x142cc4430` (+1) | 55 | 0 | - |  |
| `0x0391` | 0xcc | `0x14289d615` | *no-op arm* | | | | |
| `0x0392` | 0xcd | `0x14289d2ce` | `0x1407b3a90` (+4) | 243 | 0 | - |  |
| `0x0393` | 0xce | `0x14289d3b0` | `0x1408dc810` (+2) | 59 | 3 | u32 u32 (raw) |  |
| `0x0394` | 0xcf | `0x14289d46e` | `0x140e6c990` | 1301 | 0 | - |  |
| `0x0395` | 0xd0 | `0x14289d615` | *no-op arm* | | | | |
| `0x0396` | 0xd1 | `0x14289d48a` | `0x1408a9e40` (+9) | 75 | 0 | - |  |
| `0x0397` | 0xd2 | `0x14289d563` | `0x1429e3ef0` (+1) | - | - | - |  |
| `0x0398` | 0xd3 | `0x14289d58b` | `0x1410e7970` | 682 | 3 | u32? raw? raw? |  |
| `0x0399` | 0xd4 | `0x14289b4e4` | `0x142cc4430` | 55 | 0 | - |  |
| `0x039a` | 0xd5 | `0x14289d5f5` | *default* | | | | |
| `0x039b` | 0xd6 | `0x14289d5f5` | *default* | | | | |
| `0x039c` | 0xd7 | `0x14289d5f5` | *default* | | | | |
| `0x039d` | 0xd8 | `0x14289d5f5` | *default* | | | | |
| `0x039e` | 0xd9 | `0x14289a6f2` | `0x142cbe730` (+1) | - | - | - |  |
