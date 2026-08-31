# Verifying `0x029E..0x02A1` against the client, field for field — and one thing that is wrong

Written 2026-08-31. **No Ghidra** (three sibling agents are running and the project locks),
**no cargo**, **no client run**. Everything below is `tools/listing.py`, `tools/reads.py`,
`tools/callers.py`, `tools/encodes.py` and three throwaway scripts over
`client-patched\MapleStory.exe`, plus a sweep of `previous-runs/` and `research/fixtures/`.

This **builds on** `crates/world/src/remoteattack.rs` and `research/user-pool-tables.md` §4
rather than re-deriving them. Where it re-derives something, it is because that claim is
load-bearing for a conclusion here, and it says so.

Tags: **[L]** read off this client's listing or the PE's own table bytes; **[D]** derived
from two or more [L]; **[I]** inferred.

---

## 0. The answers, up front

| question | answer |
|---|---|
| **Does the outbound body match the handler's reads?** | **Yes, exactly.** 29 read sites against 29 written fields, in the same order, at the same widths. 85 bytes for a one-hit melee swing, and the arithmetic closes three independent ways. §2 |
| **Is a short packet the risk?** | **No.** The lengths are right. §2 |
| **Is there a risk?** | **Yes, and it is the opposite one.** The client decodes the target list into a **fixed 15-slot stack array with no bound check**, and this server writes as many blocks as it parsed. **16 targets overwrites the return address.** §4 — this is the one thing that must change before two clients share a map. |
| **The 40-in-13-out asymmetry** | Verified. All 13 outbound struct offsets appear exactly once among the 40 inbound writes, so every field's source is unambiguous; the builder picks the right one for all 13 and writes them in the decoder's read order. §3 |
| **Which of the four opcodes means what** | **Not checkable without a client run, and it does not matter.** Re-derived with a stronger instrument than the original: one stub serves all four, and the opcode slot has exactly one reference in 175 instructions — the store. §5 |
| **Damage numbers: does A see B's?** | The packet **carries** the damage list, `u64` absolute, per hit. Whether the client *draws* it is **not statically answerable** — the consuming code is virtualised. §6 |
| **When B kills a mob next to A, what does A see?** | **A mob that never flinches, never dies, and cannot be attacked** — and A still gets the EXP. Every mob state transition is unicast to whoever caused it. §6.2 |
| **What else is missing** | Everything except move and attack. Nine enumerated packets, all unsent. §7 |

---

## 1. The instruments, and the control each one passed first

`CLAUDE.md` § *Verify the instrument before believing it*, and § *The scratchpad shadows the
real tools*. Every command was run **from the repo root**; the three throwaway scripts were
run as `python - < script.py`, which leaves `sys.path[0]` empty, and each does
`sys.path.insert(0, "tools")` explicitly. `tools/reads.py` is the live 9605-byte copy and the
scripts print `R.__file__` to prove it.

| instrument | control | result |
|---|---|---|
| `tools/reads.py` | `0x140304100 2` must give raw@`140304138`, u8@`140304144`, u8@`140304183`, then a `u16` run | reproduced exactly |
| `tools/listing.py` | same function, same reads at the same addresses | reproduced exactly |
| `tools/callers.py` | `0x1402fa9a0` must give **96 sites in 15 functions, 43 of them in `0x140304b20`** | reproduced exactly |
| table-B decode (§5) | index `0x07` must be stub `0x1429bb9b2` (`user-hit.md` §5.3) **and** index `0x11` must be `0x1429bba36` (`level-up.md` §6) | both reproduced |
| archive sweep (§1.1) | must find opcodes known to be present | **six** positive controls found, see below |

### 1.1 The archive sweep found its own broken row before it found anything else

The sweep's first run reported `0x03C7 MobLeaveField` as **0 files**, next to `0x03F0` at 314
events. Both are sent by the same function (`net::combat::mob_hit_replies`), so one of them
being absent was impossible. The instrument was fine: **`MOB_LEAVE_FIELD` is `0x03D1`**
(`crates/net/src/combat.rs:108`), and `0x03C7` was a row I had written from memory. Corrected,
it is 50 files / 322 events. `CLAUDE.md`: *a table row written from a quick read is a claim.*

Counts are deduplicated on **events** — `(timestamp, direction, body)` — not on files, because
`previous-runs/` and `research/fixtures/` overlap and a fixture copied mid-write hashes
differently from its own run. 185 world logs, 646 440 packet lines parsed. **[L]**

```text
opcode    files   events   what
0x00DF       57      514   UserMeleeAttack in            <- POSITIVE CONTROL
0x02FF       79   164117   MobMove in                    <- POSITIVE CONTROL
0x03C6       81     2338   MobEnterField out             <- POSITIVE CONTROL
0x03D1       50      322   MobLeaveField out             <- POSITIVE CONTROL
0x03F0       49      314   MobHpChange out               <- POSITIVE CONTROL
0x046E       49     2005   DropEnterField out            <- POSITIVE CONTROL
0x0226       25       37   UserChat (GM banner, unicast) <- POSITIVE CONTROL
---------------------------------------------------------------------------
0x0224        0        0   UserEnterField        (remote)
0x0225        0        0   UserLeaveField        (remote)
0x0293        0        0   UserMoveRemote
0x029E        0        0   UserAttackRemote melee
0x029F        0        0   UserAttackRemote shoot
0x02A0        0        0   UserAttackRemote magic
0x02A1        0        0   UserAttackRemote body
0x02A5        0        0   UserHitRemote
0x02AF        0        0   UserEffectRemote
0x02B0        0        0   remote temp-stat SET
0x02B1        0        0   remote temp-stat RESET
```

`remoteattack.rs` reported this negative against **two** controls; it now stands against
**seven**, and the whole remote family is zero rather than just the four attacks. Note that
`0x0224`/`0x0225` being zero is itself consistent rather than alarming: `Bus::enter_field`
posts a spawn to *everyone except the newcomer*, and only one client has ever connected.

---

## 2. Q1 — the body matches the reads, field for field and length for length

### 2.1 The read count: **29 sites**, and where each one lives

Re-derived, not taken from the note. All **[L]**.

| where | address | reads | note |
|---|---|---|---|
| `FUN_1429bb720` (the router, before dispatch) | `1429bb745` | **1** — `u32 charId` | `reads.py 0x1429bb720 1`: exactly one direct read |
| `FUN_1429d2ee0` (the handler) | `1429d2f3f` | **1** — `u16` → `[user+0x406c]` | stable at depth 1, 4 and 6 |
| `FUN_140f32200` (the header) | `140f32380`..`140f3240f` | **13** | straight-line, no branch between them |
| `FUN_140f32440` (the target list) | `140f32523`..`140f32611` | **14 sites** — 3 head, 2 per-target, 3 per-hit, 6 tail | two nested loops |
| | | **29 sites** | |

For a one-target, one-hit swing that is **29 dynamic reads**, and
`remoteattack::user_attack_remote` performs **29 writes**. Byte totals: `4 + 2 + 43 + 12 +
(4 + 2 + 10 + 8)` = **85**, which is what
`a_real_swing_becomes_the_body_the_listing_describes` asserts and what `body_len` independently
recomputes. Three ways of arriving at 85; they agree. **[D]**

### 2.2 The header decoder, read off the listing rather than the note

`FUN_140f32200`, `0x140f32200..0x140f32430`. The only branch in the function is at
`0x140f3234b`, in the zero-init prologue, **before any read**. Every one of the 13 reads is
unconditional, so the 43-byte header is never short and never long. **[L]**

| # | address | width | → struct | builder writes |
|---|---|---|---|---|
| 1 | `140f32380` | `u32` | `+0x08` | `h.skill_id` |
| 2 | `140f3238b` | `u32` | `+0x0c` | `h.skill_level` (u8 → u32) |
| 3 | `140f32396` | `u8` → `setne` byte | `+0x1c` | `h.f7` |
| 4 | `140f323a6` | `u32` | `+0x34` | `h.x` sign-extended |
| 5 | `140f323b1` | `u32` | `+0x38` | `h.y` sign-extended |
| 6 | `140f323bc` | `u32` | `+0x40` | `h.f16` sign-extended |
| 7 | `140f323c7` | `u32` | `+0x44` | `h.f17` sign-extended |
| 8 | `140f323d2` | `u32` | `+0x20` | `h.f8` |
| 9 | `140f323dd` | `u8` → `movzx` dword | `+0x2c` | `h.f11 as u8` |
| 10 | `140f323eb` | `u8` → `movzx` dword | `+0x48` | `h.f19` |
| 11 | `140f323f9` | `u32` | `+0x50` | `h.f22` |
| 12 | `140f32404` | `u32` | `+0x54` | `h.f23` |
| 13 | `140f3240f` | `u32` | `+0x60` | `h.f25` |

`4+4+1+4+4+4+4+4+1+1+4+4+4` = **43** = `remoteattack::HEADER_LEN`. **The order matches
exactly.** [L]

### 2.3 The target list, and the tail offsets recomputed from `rsi`

`FUN_140f32440`, `0x140f32440..0x140f32656`. The loop bases `rsi` at `r15 + 0x154 + i*0x1d8`
(`140f3254f`, `140f3261f`), so every offset below is `rsi ± disp` resolved back to the target
struct. **[L]**

```text
140f32523  u32  -> [r15+0]   targetCount   ; the loop bound, 140f32546 cmp / jle
140f3252e  u32  -> [r15+4]
140f3253a  u32  -> [r15+8]
  per target, stride 0x1d8, base = r15 + 0x10 + i*0x1d8:
140f32559  u32  -> [rsi-0x134] = t+0x10   objectId   ; 140f32564 test/je -> loop tail
140f3256f  u16  -> [rsi-0x124] = t+0x20   hitCount   ; 140f32581 test/je -> skips hits
    per hit, stride 0x10, rbx = rsi-0x114 = t+0x30:
140f32593  u8   -> [rbx-8]  = t+0x28+j*0x10   flagA (setne)
140f325a3  u8   -> [rbx-7]  = t+0x29+j*0x10   flagB (setne)
140f325b3  u64  -> [rbx]    = t+0x30+j*0x10   damage
140f325cc  u8   -> [rsi-4]    = t+0x140
140f325da  u8   -> [rsi]      = t+0x144
140f325e7  u8   -> [rsi-0x20] = t+0x124
140f325f5  u16  -> [rsi+4]    = t+0x148
140f32603  u16  -> [rsi+8]    = t+0x14c
140f32611  u8   -> [rsi+0xc]  = t+0x150
```

`middle_at` in `remoteattack.rs` maps those six to middle offsets 24, 25, 3, 16, 20, 26 — and
the builder writes them in exactly that order. **The tail is right.** `4 + 2 + (1+1+1+2+2+1)`
= **14** = `TARGET_FIXED_LEN`; `1+1+8` = **10** = `HIT_LEN`. [L]

### 2.4 A seventh coincidence the module docs do not have: the same struct

`FUN_140f32200`'s zero-init prologue (`140f3229b`..`140f32376`) clears `+0x00, +0x04, +0x0c,
+0x10, +0x14, +0x1c, +0x20, +0x24, +0x28, +0x2c, +0x30, +0x34, +0x38, +0x3c, +0x40, +0x44,
+0x48, +0x4c, +0x4d, +0x4e, +0x50, +0x54, +0x58, +0x5c, +0x60, +0x64..+0x74, +0x78, +0x7c,
+0x80, +0x88, +0x90, +0x98, +0xa0` — **the same field set the request encoder reads**, and it
frees `[rbx+0x80]` with `add rcx,-0x10 / call 0x14019f2c0`, which is a `std::string`/BSTR
release. That independently confirms both that the two packets share one struct type and that
inbound field 33 (`+0x80`) is the string `attack_type`. **[L]**

### 2.5 What is NOT settled about the read list

`FUN_1429d2ee0`'s readable code is 737 bytes and ends at `0x1429d31bc jmp 0x144dad6a6`.
Confirmed **[L]**: `.themida` spans `0x143d87000..0x145173000` with **raw size 0**, and
`0x144dad6a6` is inside it. The 4631 bytes `.pdata` claims are VM data.

`remoteattack.rs` says the read list is complete because the escape sits after the same
trace-logging call that precedes `FUN_1429bb720`'s ordinary epilogue. **That is [D], not [L],
and its blind spot has a name:** the packet reader is live in `rbx` at the jump, so nothing
statically rules out further reads inside the VM. What *does* corroborate it independently is
that the 13 + 14 reads reconcile field for field against the 40-write request encoder with no
gaps and no leftovers — a decoder that read more would have to read fields the encoder never
writes. **The claim survives; the reason is the reconciliation, not the trace call.**

---

## 3. Q2 — the 40-in-13-out asymmetry, verified

`tools/encodes.py 0x140f31fe0` counts **40** writes; `tools/reads.py 0x140f32200 4` counts
**13** reads. Reproduced. But a count is not a mapping, so the 40 writes were read off the
listing with their **source struct offsets**, and intersected with the 13.

### 3.1 The 40 inbound writes, with their source offsets

Read off `0x140f31fe0`. Six of them are `cmp byte [rbx+N],0 / setne dl` — `crates/net/src/attack.rs`'s
`FIELD_WIDTHS` records those six as `None`, and **this recovers all six**: indices 4, 7, 18,
20, 21, 31 come from `+0x10, +0x1c, +0x4c, +0x4d, +0x4e, +0x78`. **[L]**

Wire order, with the source offset: `0x00 0x04 0x08 0x0c 0x10 0x14 0x18 0x1c 0x20 0x24 0x28
0x2c 0x30 0x34 0x38 0x3c 0x40 0x44 0x4c 0x48 0x4d 0x4e 0x50 0x54 0x58 0x60 0x64 0x68 0x6c
0x70 0x74 0x78 0x7c 0x80 0x88 0x90 0x94 0x98 0x9c 0xa4`.

That reproduces `FIELD_WIDTHS` entry for entry — **including the `0x4c` before `0x48` swap at
indices 18/19**, which is the one place a hand-written table would most plausibly be wrong.
`attack.rs::parse_header` reads in that same order and names index *n* `f<n>`, so the field
names line up with the struct offsets mechanically. **[D]**

### 3.2 Which 13 survive

Every one of the 13 outbound struct offsets appears **exactly once** among the 40, so each
outbound field has an unambiguous inbound source. **[D]**

| out order | struct | in index | in width | out width | builder | verdict |
|---|---|---|---|---|---|---|
| 1 | `+0x08` | 2 | `u32` | `u32` | `skill_id` | ✔ |
| 2 | `+0x0c` | 3 | `u8` | `u32` | `skill_level` | ✔ widened, lossless |
| 3 | `+0x1c` | 7 | `u8` bool | `u8` bool | `f7` | ✔ |
| 4 | `+0x34` | 13 | `u16` | `u32` | `x` | ✔ **sign** — §3.3 |
| 5 | `+0x38` | 14 | `u16` | `u32` | `y` | ✔ **sign** |
| 6 | `+0x40` | 16 | `u16` | `u32` | `f16` | ✔ **sign** |
| 7 | `+0x44` | 17 | `u16` | `u32` | `f17` | ✔ **sign** |
| 8 | `+0x20` | 8 | `u32` | `u32` | `f8` | ✔ |
| 9 | `+0x2c` | 11 | `u32` | `u8` | `f11 as u8` | ⚠ **narrowed by the client's own protocol** |
| 10 | `+0x48` | 19 | `u8` | `u8` | `f19` | ✔ |
| 11 | `+0x50` | 22 | `u32` | `u32` | `f22` | ✔ |
| 12 | `+0x54` | 23 | `u32` | `u32` | `f23` | ✔ |
| 13 | `+0x60` | 25 | `u32` | `u32` | `f25` | ✔ |

**The 27 that are dropped**, by inbound index: 0, 1, 4, 5, 6, 9, 10, 12, 15, 18, 20, 21, 24,
26–39. Two are worth naming:

* **index 12, `+0x30` — the `tick`.** The observer's client is given no timestamp for the
  swing and must use its own clock. That is the client's design, not an omission here.
* **index 33, `+0x80` — `attack_type`, the string** (`"User Melee"`, `"User Magic Skill
  System"`). It does **not** cross. The animation therefore has to come from `skill_id`
  (`+0x08`) and `skill_level` (`+0x0c`), which do. This is the strongest structural argument
  that the *opcode* cannot be what picks the animation — see §5.

### 3.3 The four sign-extensions: what is measured and what is chosen

The struct field is a **dword** (the zero-init writes `qword [rbx+0x34]` and `qword
[rbx+0x40]`, covering four dwords). The request carries **16 bits** of it. The broadcast
restores a **full 32**. Reconstructing the missing 16 bits is therefore a *choice the server
has to make*, and the listing does not make it: `movzx` on the encode side is the `u16` write
ABI and says nothing about the field's signedness.

What settles it is the corpus, not the listing: `net::attack` types all four `i16` because
captured bodies carry `0xFFD2` and `0xFF33`, which are −46 and −205 as map coordinates and
absurd as unsigned, and `MELEE_EMPTY_127_ODD` pins it in a test. **So: [L] that the field is a
dword and the wire is 16 bits; [D] that sign-extension is the reconstruction the client
wants.** The builder is right, and the risk if it is not is a character drawn at x = 65331,
not a crash.

### 3.4 The one narrowing, and it is not this server's bug

`+0x2c` is a `u32` inbound and the broadcast decoder reads a **`u8`** (`140f323dd`, `movzx
eax,al`). **The client's own broadcast cannot carry more than 255 there**, whatever the server
does. `attack.rs` notes the field is 4 or 6 in all 434 captured bodies, so the truncation never
fires in practice — but it is the client's protocol ceiling, not a server choice, and no
change here can lift it.

---

## 4. **THE BUG.** The client's target array is 15 slots and nothing checks the count

This is the one finding that should stop a two-client test until it is fixed. It is not the
shape the brief expected — the packet is not short; the *list* is unbounded.

### 4.1 The array is 15 slots, confirmed three ways

**[L]**, from `FUN_1429d2ee0`'s frame and `FUN_140f32440`'s prologue:

```text
1429d2ef0  lea   rbp,[rsp-0x1d80]      ; frame: rsp_final = rbp-0x160, top = rbp+0x1d80
1429d2f2b  xor   edi,edi               ; rdi = 0, and rdi is non-volatile - it survives
1429d2f17  mov   qword [rbp+0x1d60],rax        ; the stack cookie
1429d2fe6  mov   r8d,0x1bb0
1429d2fec  lea   rcx,[rbp+0x1a8]       ; memset 0x1bb0 bytes from rbp+0x1a8
1429d300b  mov   edx,0x1d8             ; element size
1429d3010  lea   r8d,[rdi+0xf]         ; count = 15
1429d3014  lea   rcx,[rbp+0x1b0]       ; array base   __ehvec_ctor_iter
1429d3024  lea   rcx,[rbp+0x1a0]       ; -> r15 in FUN_140f32440
1429d302b  call  0x140f32440
```

1. `lea r8d,[rdi+0xf]` with `rdi = 0` → **15**.
2. The memset is `0x1bb0` = 7088 from `rbp+0x1a8`, ending at `rbp+0x1d58`. The array is
   `rbp+0x1b0 + 15*0x1d8` = `rbp+0x1d58`. **The memset covers exactly the 8-byte head plus
   fifteen slots** — an arithmetic identity that does not depend on reading the `lea` right.
3. `FUN_140f32440` builds its own temp array the same way: `mov r8d,0x1bb8`/`0x1ba8` memsets
   and `mov esi,0xf` (`140f324eb`), `mov r8d,0xf` (`140f324be`). `0x1ba8` = 7080 = 15 × 0x1d8
   **exactly**.

### 4.2 Nothing bounds the loop

`140f32523` reads `targetCount` into `[r15]`; the next instruction is another read. Between
the read and the loop there is **no comparison against 15** — only `xor r14d,r14d / cmp
[r15],r14d / jle` (a signed test against zero) at the top and `cmp r14d,[r15] / jl` at the
bottom. The per-hit loop is bounded only by the `u16` hit count at `[rsi-0x124]`. **[L]**

The *request* encoder `FUN_140f31f60` is identically unbounded (`cmp [rdi],ebx / jle`,
`imul rcx,rax,0x1d8`), and its caller `FUN_1428c1fa0` builds the same 15-slot array
(`0x1428c283b mov r8d,0x1bb0`, `0x1428c2860 mov edx,0x1d8` with the same `+0xe`/`+0xf` count,
and six more `mov r8d,0xf` / `mov edx,0x1d8` pairs). **So the protocol's cap is 15 on both
sides and neither side enforces it.** [L]

### 4.3 Where the sixteenth target lands

Computed, not hand-derived (`scratchpad/frame.py`, every constant cited to its address):

```text
target array   rbp+0x1b0 .. rbp+0x1d58    (15 x 0x1d8 = 0x1ba8)
stack cookie   rbp+0x1d60 .. rbp+0x1d68
saved xmm6     rbp+0x1d70
frame top      rbp+0x1d80    <- 7 saved non-volatile regs, return address at rbp+0x1db8

target[14]  objectId rbp+0x1b90   hitCount rbp+0x1ba0   tail ends rbp+0x1cd4   (last legal)
target[15]  objectId rbp+0x1d68   hitCount rbp+0x1d78   tail ends rbp+0x1eac   <-- PAST THE
                                                                                   RETURN ADDRESS
target[16]  objectId rbp+0x1f40   hitCount rbp+0x1f50   tail ends rbp+0x2084
```

Note the nasty detail: target[15]'s first write is at `rbp+0x1d68`, **immediately after** the
cookie at `0x1d60..0x1d67`. The overflow **steps over the cookie** and lands on the saved
registers and the return address. `__security_check_cookie` would not see it — and in any case
the epilogue is the part that is virtualised.

A second, independent route exists via the hit count (inbound `hit_count` is a `u8`, so ≤ 255):

```text
target index  7 with 235+ hits reaches the cookie   (needs  8 targets)
target index 14 with  28+ hits reaches the cookie   (needs 15 targets)
```

### 4.4 Can this server be made to produce it? Yes, easily

`net::attack::parse` loops `for index in 0..target_count` where `target_count` is a raw `u32`
off the wire, and stops only when the reader underruns. The minimum inbound target block is
**68 bytes** (`4 + 4 + 1 + 55 + 2 + 1 + 1`, with zero hits, zero pairs, no sub-object).
`MAX_PACKET_LEN` is **16 MiB** (`crates/net/src/codec.rs:351`), so up to **246 723** targets
parse. `remoteattack::user_attack_remote` writes `targets.len() as u32` and one block each,
with `drawable_targets` filtering only zero ids.

* **16 targets inbound** = 1088 bytes of list. **16 targets outbound** = 285 bytes.
* A legitimate client cannot send this — its own array is 15 slots (§4.2).
* **Nothing on this socket authenticates anybody.** `CLAUDE.md` standing constraint. One
  modified client on the map smashes every other player's stack, at will, with a 1.1 KB packet.

**This is a [D] built from two [L] halves — the array size and the missing bound — and the
arithmetic between them. It has not been observed on a client, and it should not be.**

---

## 5. Q3 — the opcode mapping. Not checkable, and free

**Plainly: no, this cannot be settled without a client run, and it does not need to be.**

What *is* settled, re-derived here with a stronger instrument than the original:

**(a) One stub serves all four.** Read out of the PE's own table bytes at `0x1429bbc34`,
stride 4, with two controls that reproduce (`index 0x07 -> 0x1429bb9b2`, `index 0x11 ->
0x1429bba36`):

```text
opcode 0x029e  index 0x00  ->  stub 0x1429bb95e
opcode 0x029f  index 0x01  ->  stub 0x1429bb95e
opcode 0x02a0  index 0x02  ->  stub 0x1429bb95e
opcode 0x02a1  index 0x03  ->  stub 0x1429bb95e     distinct stubs: 1
opcode 0x02a2  index 0x04  ->  stub 0x1429bb970     (for contrast - the next four differ)
```

```text
1429bb95e  mov r8,rdi / mov edx,esi / mov rcx,rbx
1429bb966  call 0x1429d2ee0
1429bb96b  jmp  0x1429bbb10        ; table C, where 0x29e..0x2a1 are all *default*
```

There is **no per-opcode dispatch and no second handler.** **[L]**

**(b) The handler never reads the opcode back.** The original scan was a byte pattern —
`(modrm & 0xC7) == 0x45 && disp8 == 0x8C` — which has exactly the structural blind spot
`CLAUDE.md` records for `mob+0x42c`: it cannot see a `disp32` encoding and it cannot see a
`lea` that hands the slot's address off. I re-ran it with capstone's decoded operands, which
sees all three. **Both instruments give the same numbers**:

```text
REAL code (737 B, 175 instructions)
   byte-pattern:        [rbp-0x74] 1   [rbp-0x7c] 7
   capstone operands:   [rbp-0x74] 1   [rbp-0x7c] 7
     0x1429d2f21  mov dword ptr [rbp - 0x74], edx     <-- the opcode slot: a STORE
```

Seven for the control slot, one for the opcode slot, and the one is the store. `edx` is not
consumed anywhere else before it is clobbered. **[L]**

**(c) The animation cannot come from the opcode anyway**, because `attack_type` — the string
that *names* the attack — is one of the 27 dropped fields (§3.2), while `skill_id` and
`skill_level` both cross. Whatever picks the animation reads those. **[D]**

**The named blind spot, since a [D] negative needs one:** `[rbp-0x74]` is still live in the
frame at `0x1429d31bc jmp 0x144dad6a6`, and `.themida` has zero bytes on disk. A read of that
slot inside the VM cannot be excluded by any static means. Three things make it not worth
chasing: the virtualised part is the epilogue, the reads reconcile with no gaps (§2.5), and
**the worst case is a wrong animation, not a crash.** `remote_attack_opcode`'s mapping mirrors
the inbound order, which is the only argument available, and it is fine.

---

## 6. Q4 — damage numbers

### 6.1 The packet carries the damage; whether it draws it is not statically answerable

The `u64` at `[t+0x30+j*0x10]` is written by the request encoder (`140f31c1d`) and read back
by the broadcast decoder (`140f325b3`) into the same slot at the same stride. **Absolute, not
a percentage, not scaled** — the attacker's own claim, which is right, because the attacker's
screen already drew that number. The percentage in this neighbourhood is `0x03F0`'s mob HP,
a different packet. All of that is **[L]** and `remoteattack.rs` has it correct.

**Whether the observer's client renders a number from it, I cannot say.** The handler's
readable tail computes four values and hands them to the VM without using them:

```text
1429d3147  call 0x140c93640
1429d316a  cmovl r13d, ebx
1429d316e  mov   dword [rbp-0x80], r13d      ; a damage/count value
1429d317a  sete  r13b                        ; a comparison flag
1429d3163  sete  dil                         ; [r15+0x30] == 3
1429d31bc  jmp   0x144dad6a6                 ; .themida, raw size 0
```

Those are arguments for something inside the virtualised region. **[D] that the drawing is
there; [I] that it draws.** This is a genuine "only a client run answers it", and it is one of
the things T2 measures.

### 6.2 What A actually sees when B kills a mob — and this is the real answer

**The packet is not the problem. The mob is.** There are exactly **four** cross-connection
delivery paths in this server, enumerated from `crates/world/src/broadcast.rs`'s API and every
production call site of each:

| path | production callers | what crosses |
|---|---|---|
| `Bus::enter_field` / `leave_field` | `session/multiplayer.rs:132,146` | `0x0224` spawn, `0x0225` farewell |
| `Bus::publish` | **two**: `multiplayer.rs:202` (`0x0293` move), `multiplayer.rs:277` (`0x029E..` attack) | those two packets |
| `Bus::send_to_character` | `session/combat.rs:607` | `Event::Experience` — the EXP share |

**That is the entire list.** Everything else a session produces goes into its own `out` vector
and reaches one client. In `session/combat.rs::on_attack`, `publish_user_attack` is called
*before* the target loop, and then **every consequence of the swing is pushed to `out`**:
`mob_hit_replies` (`0x03F0` HP bar / `0x03D1` death), `drops_from_kill` (`0x046E`),
`award_kill_experience`, `credit_kill_to_quests`.

So on **A**'s screen, when **B** kills a mob standing between them:

| what A sees | why |
|---|---|
| B's swing animates, with B's damage numbers in the body | `0x029E` publishes — §6.1 for whether they draw |
| **the mob's HP bar does not move** | `0x03F0` is in B's `out` |
| **the mob does not die** — it stands there | `0x03D1` is in B's `out` |
| **the drop does not appear** | `0x046E` is in B's `out` |
| **A gets the EXP anyway** | `send_to_character` delivers the share |
| **A's own swings at that mob do nothing at all** | `fields.mob_hp(map, id)` is `None` — `hurt` removed it from the shared field — so `on_attack` hits `continue`: no HP bar, no death, no drop, no EXP |
| on respawn, **one of the two clients gets `0x03C6` and the other does not** | `Fields::due_respawns` **drains** `field.pending` (`std::mem::take` + partition) and is called per session — whoever ticks first takes the whole list |
| if A is the one who ticks first, A receives a **second `0x03C6` for an object id its client still has** | A never got the `0x03D1` |
| the mob is in **two different places** on the two screens | both clients were granted `CONTROL_NORMAL` for every mob (`session/field.rs:116`), both run the wander locally, and `0x03D9` is built and deliberately not sent (`combat.rs:36-41`) |

**"A gets EXP for a mob that is still standing on A's screen" is the cheapest possible
discriminator for this whole class**, and it costs nothing to watch for.

---

## 7. Q5 — everything else in "seeing another player fight", enumerated

Not sampled. Every remote-user opcode in table B and table C was checked against this
server's production call sites.

| what | opcode | decoded? | built? | **sent?** | evidence |
|---|---|---|---|---|---|
| remote player appears | `0x0224` | yes | `net::userpool::user_enter_field` | **YES** | `multiplayer.rs:132` |
| remote player leaves | `0x0225` | yes | `user_leave_field` | **YES** | `multiplayer.rs:146` |
| remote player walks | `0x0293` | yes | `user_move_remote` | **YES** | `multiplayer.rs:202` |
| **remote player attacks** | `0x029E..0x02A1` | yes | `remoteattack::user_attack_remote` | **YES — wired, never on a wire** | `multiplayer.rs:277`, `combat.rs:245` |
| **remote player is hit by a mob** | `0x02A5` | **yes**, `user-hit.md` §5.3 — `u32 charId` + 147-byte HITINFO = 151 bytes | **no builder in `net`** | **no** | zero references to `USER_HIT_REMOTE` in `crates/world/` |
| **remote player levels / plays an effect** | `0x02AF` | yes, `level-up.md` §6 — `u32 charId, u8 effect` | `net::userpool::user_effect_remote` **exists** | **no** | zero references to `user_effect_remote` / `USER_EFFECT_REMOTE` in `crates/world/` — **built and not wired** |
| **buffs appearing on a remote player** | **`0x02B0`** — table C idx `0x1d`, handler `FUN_1429d62f0` | partly — see §7.1 | no | **no** | new here |
| **buffs expiring on a remote player** | **`0x02B1`** — handler `FUN_1429d6500` | partly | no | **no** | new here |
| remote player's mob damage / HP bar | `0x03F0`, `0x03D1`, `0x046E`, `0x03C6` | yes | yes | **unicast only** | §6.2 |
| mob movement seen by a second player | `0x03D9` | yes | `net::mobmove::mob_move_broadcast`, tested | **no** | `combat.rs:36-41`, deliberately |
| remote player chats | `0x0226` | yes | `net::userchat` | **no relay** — `gm.rs:1788` is a unicast banner | 37 archived events, all local |
| **remote player dies** | **unknown** | **no** | no | **no** | `user-hit.md` §6.3 stopped before finding the death/tomb sequence; the `"dead"` stance strings at `0x1432793f8`/`0x143279418`/`0x143279438` are where to resume |

### 7.1 `0x02B0` / `0x02B1` are the remote buff pair — new, and [D]

`CSecondaryStat::DecodeForRemote` (`FUN_140a46e50`, the 124-byte mask decoder,
`research/msexe-secondarystat-remote-140a46e50.txt`) has **exactly two callers**
(`tools/callers.py`: 2 call sites, 0 tail jmps, 0 data pointers):

* `0x1429ce270` — the `0x0224` `CUser::Init` decoder, already known;
* `0x1429d62f0` — reached from `0x1429bb720` at `0x1429bbb6e`, which is table C index `0x1d`,
  i.e. **opcode `0x02B0`**.

`FUN_1429d62f0` then reads `u16`, `u8`, applies the per-user flood gate `0x140f8abc0`, and
stores a long run of dwords into `[user+0x40e4]` onward — a temporary-stat apply. Its sibling
`FUN_1429d6500` (`0x02B1`) reads a **`0x7c` = 124-byte** raw mask (`0x1429d6581 lea
r8d,[rbx+0x7c]`, `0x1429d658f`) and a `u8`, with no value list — a reset. **[D]**

`research/user-pool-tables.md` §9 lists both rows with an empty note; this fills them in.

### 7.2 An unknown character id is safe, and that matters for reading T2

`FUN_1429bb720` reads the `u32 charId`, hashes it (`div rcx` on `[pool+0x100]`), walks the
bucket chain comparing `[node+0x10]`, and has **six** separate miss/null guards
(`1429bb756`, `1429bb770`, `1429bb784`, `1429bb790`, `1429bb79d`, `1429bb7aa`) that all jump
to the same exit `0x1429bbc19`. **A packet naming a character the pool does not have is
dropped silently and does not enter the handler.** **[L]**

So "A saw nothing" on T2 has an **innocent** reading — A's client never got B's `0x0224`, or
the ids disagree — as well as a broken-body reading. They must be told apart by the hook log,
not by the screen.

---

## 8. WIRE IT LIKE THIS

Nothing below has been applied — this document does not edit any file. Two changes; the first
is not optional.

### 8.1 Cap the target list at what the client's array holds — `crates/world/src/remoteattack.rs`

**Anchor:** the doc block and body of `pub fn drawable_targets` (currently around line 272).

Add beside `TARGET_FIXED_LEN`:

```rust
/// **The client's target array is fifteen slots, and its decoder does not check.**
///
/// `FUN_1429d2ee0` builds a fixed array on its own stack — `1429d3010 lea r8d,[rdi+0xf]`
/// with `rdi = 0`, element size `0x1d8` — and `FUN_140f32440` walks it for as many entries
/// as the count says (`140f32546`, `140f32626`), with no bound. The memset beside it is
/// `0x1bb0` = 8 + 15 × 0x1d8 exactly, which confirms the fifteen without depending on the
/// `lea`. Target index 15 writes from `rbp+0x1d68` to `rbp+0x1eac`, **stepping over the
/// stack cookie at `rbp+0x1d60` and past the return address at `rbp+0x1db8`**.
///
/// The client's own request builder uses the same fifteen-slot array (`0x1428c283b`,
/// `0x1428c2860`), so a legitimate client cannot exceed this. Nothing on this socket
/// authenticates anybody, so a crafted one can — and this server would relay it to every
/// other player on the map. `research/remote-attack-verification.md` §4.
pub const MAX_TARGETS: usize = 15;

/// Hits per target, bounded by the same struct: the array starts at `t+0x30`, strides
/// `0x10`, and the struct is `0x1d8` — so `0x30 + (n-1) * 0x10 + 8 <= 0x1d8` gives 27.
/// Inbound `hit_count` is a `u8`, so 8 targets with 235+ hits is the other way over.
pub const MAX_HITS_PER_TARGET: usize = 27;
```

Then `.take(MAX_TARGETS)` on the iterator, and clamp `hits_of` to `MAX_HITS_PER_TARGET`
instead of `u16::MAX`:

```rust
pub fn drawable_targets(attack: &Attack) -> impl Iterator<Item = &AttackTarget> {
    attack.targets.iter().filter(|t| t.object_id != 0).take(MAX_TARGETS)
}

fn hits_of(t: &AttackTarget) -> &[net::attack::AttackHit] {
    &t.hits[..t.hits.len().min(MAX_HITS_PER_TARGET)]
}
```

`body_len` already goes through both, so the recomputation follows for free and the
`debug_assert_eq!` keeps holding. **Extend `describe` to say when either cap fired**, in the
same shape as the existing zero-id line — a cap that fires is the signature of a crafted body
and must not be silent. `CLAUDE.md`: *a refusal that is reported to no one will be ignored.*

**Tests to add** (all four, because a test covering one of N effects gives false confidence
about the rest):

1. 20 targets in → 15 blocks out, **and the count field says 15**;
2. 1 target with 200 hits in → 27 hits out, and the count says 27;
3. `body_len` agrees with the writer's cursor in both;
4. the independent walk in `every_fixture_round_trips_its_own_length` still lands on the end.

### 8.2 The mob is invisible to the observer — `crates/world/src/session/combat.rs`

**Anchor:** `on_attack`, the `for (opcode, body) in net::combat::mob_hit_replies(...)` loop
(around line 275) and the `drops_from_kill` call above it.

Those replies go into `out`, the attacker's own list. Publishing the same `Reply` to the bus
with `supersedes = None` would put the HP bar, the death and the drop on every screen. **Two
things to get right before doing it**, and neither is settled here:

* `0x03F0` carries a **percentage** and `0x03D1` a death animation — both are field state, not
  per-viewer, so they are safe to broadcast verbatim. `0x046E` `DropEnterField` has a
  per-viewer "instant vs. animated" distinction (`DROP_ENTER_FIELD_LEN_INSTANT`) that needs a
  decision before it is published.
* `Fields::due_respawns` **drains** and is called per session, so respawns reach exactly one
  client. That is a separate fix — the tick has to become field-owned and its output
  published, not drained per connection.

**Record this as unwired.** `CLAUDE.md` § *Built is not wired*.

---

## 9. What to watch on T2 — two players killing one mob together

Every step names what each outcome would mean, so the run is a measurement rather than a look.
**Do §9.0 first**; the rest is worthless if it fails.

**Before launching: apply §8.1.** Without it, one modified client is a stack smash on the
other — and more usefully, without it a crash in this test has two readings and you cannot
tell them apart.

**0. The precondition, at ~40 s of client life.** Does A's screen show B's *character* at all?
* **B is visible** → the `0x0224` spawn crossed; everything below is about the attack packet.
* **B is invisible** → **stop.** `FUN_1429bb720` drops a packet naming an unknown id in
  silence (§7.2), so every later step would come back empty for a reason that has nothing to
  do with `0x029E`. Fix the spawn first.

**1. B swings at empty air. Does B's character animate on A's screen?**
* **Yes** → `0x029E` reaches `FUN_1429d2ee0` and the body is accepted. The whole 85-byte
  layout of §2 is confirmed in one observation.
* **No, and A's client is still responsive** → the packet was dropped before the handler:
  either the charId lookup missed, or table B was skipped because the flood gate tripped
  (`1429bb8c3`, cap 100 or 200 per 1000 ms — throttled attacks go to `FUN_1429df530`, which
  reads nothing).
* **A's client freezes or dies** → the body is wrong. `world.log` will have the
  `UserAttackRemote` line with its byte count; `client-patched\maplecw-hook.log` will be
  **missing the dispatch line**, which is written on *return*. That mismatch is the signal
  (`CLAUDE.md` § *Count the same event in two logs*), and it says the client died **inside**
  the handler.

**2. B hits a mob. Does a damage number appear over that mob on A's screen?**
This is the §6.1 question and it has no static answer.
* **A number appears** → the virtualised tail does draw from the target list. Record the
  value: it should be **B's number, identical to B's screen**, because the packet carries the
  attacker's claim and not the server's clamp.
* **The swing animates but no number** → the target list is decoded and not rendered, or it is
  rendered only for the local player. Either way the animation half works and the numbers need
  a different packet.

**3. Watch the mob on A's screen while B kills it.** This is the prediction §6.2 makes, and it
should come back exactly as written:
* **HP bar does not move, mob does not die, no drop appears, and A still gets EXP** →
  confirmed. All four are one cause: `mob_hit_replies` and `drops_from_kill` go into B's
  `out`. Fix is §8.2.
* **Any of those four crosses** → something publishes that this enumeration missed, and the
  "two production `Bus::publish` callers" claim in §6.2 is wrong. Say which one, and re-run
  the call-site sweep.

**4. After the kill, A attacks the same spot.** A's swing should do **nothing at all** — no
bar, no death, no drop, no EXP — because `fields.mob_hp` returns `None` for a mob already
removed. If A's attack *does* damage it, the field is not shared and `Fields` is per-session,
which would be a much bigger finding than anything in this document.

**5. Wait for the respawn.** Exactly one of the two clients should get `0x03C6`.
* **Only B sees the new mob** → `due_respawns` drained on B's tick. Predicted.
* **Only A sees it, and A now has two mobs drawn at that spawn point** → same cause, and the
  duplicate is A's stale object from step 3 plus the new `0x03C6` for the **same object id**.
  Watch for a fault here specifically: a re-`MobEnterField` on a live object id is untested.
* **Both see it** → `due_respawns` is not draining the way the source reads, and §6.2's last
  three rows are wrong.

**6. Both players swing at one mob at the same time.** Two swings must be **two** packets on
the other screen, not one — `Bus::publish` is called with `supersedes = None` for exactly this
reason. If a rapid exchange renders as a single hit, a supersede key has crept in.

**Do not change two things at once.** If §8.1 and §8.2 are both applied before the run, a
crash has two candidate causes and the run does not settle either.

---

## 10. What this document does NOT establish

1. **Nothing here has been on a wire.** The archive sweep is a negative against seven positive
   controls; it is not a client run.
2. **Whether the observer's client draws a damage number** (§6.1). The consuming code is in
   `.themida`, raw size 0.
3. **Whether the read list is complete** is [D], not [L] (§2.5). The named blind spot: the
   packet reader is live in `rbx` at the VM entry. The reconciliation against the 40-write
   encoder is what carries the claim.
4. **The melee/shoot/magic/body assignment** is [I] and always will be, short of a run (§5).
   Getting it wrong costs an animation, not a crash.
5. **The sign of the four coordinate fields** is [D] from the corpus, not [L] from the listing
   (§3.3).
6. **What `[user+0x406c]` means.** `0x0224` writes it as a `u32` (`1429ce310`/`1429ce315`),
   `0x029E` as a `u16`, `0x02A3` from a `u8`, and `0x1429c7536` initialises it to `0x3e8` =
   1000, which is not a level. Four widths, one dword, four packets. The four consumers
   (`FUN_1429d7250`, `0x1429d84f0`, `0x1429da0a0`, `0x1429db230`) are still unread. Sending
   `chr.level` is a consistency argument, not an identification.
7. **The `0x02B0`/`0x02B1` bodies are not decoded** (§7.1) — only their identity and their
   first read. Building them needs the 124-byte mask walk that `research/buffs.md` already has
   for the local side.
8. **A remote player's death has no packet** and nobody has looked since `user-hit.md` §6.3.
9. **The overflow in §4 has not been observed.** It is arithmetic over two measured constants.
   It should be fixed rather than demonstrated.

---

## 11. Reproducing every number here

From the **repo root**:

```
python tools/reads.py    0x140304100 2        # the control - run this FIRST
python tools/callers.py  0x1402fa9a0          # 96 sites in 15 functions, 43 in 0x140304b20

python tools/reads.py    0x1429d2ee0 1        # 3 sites; same at depth 4 and 6
python tools/listing.py  0x1429d2ee0          # the frame, the array ctor, the .themida escape
python tools/listing.py  0x140f32200          # the 13 header reads and their struct offsets
python tools/listing.py  0x140f32440          # the target list; rsi = r15 + 0x154 + i*0x1d8
python tools/listing.py  0x140f31fe0          # the 40 request writes and THEIR struct offsets
python tools/listing.py  0x140f31f60          # the request's target loop - also unbounded
python tools/listing.py  0x1429bb720          # the charId read and the six miss guards
python tools/listing.py  0x1429d62f0          # 0x02B0, the remote temp-stat apply
python tools/callers.py  0x140a46e50          # DecodeForRemote: exactly two callers
python tools/encodes.py  0x140f31fe0          # 40 writes against 13 reads
python tools/reads.py    0x1429bb720 1        # one direct read: the charId
```

The three throwaway scripts are in this session's scratchpad and are short enough to retype:
`slotscan.py` (the `[rbp-0x74]` operand scan with its `[rbp-0x7c]` control), `tableb.py` (the
table-B slot decode with its two controls), `frame.py` (the frame arithmetic), and
`archive.py` (the event-deduplicated opcode sweep). Each does
`sys.path.insert(0, "tools")` and is run as `python - < script.py` so `sys.path[0]` is empty.
