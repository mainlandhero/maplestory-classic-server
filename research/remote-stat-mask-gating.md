# The 8-iteration `vtable[+0x30]` loop IS mask-gated — and `REMOTE_STAT_TAIL_LEN = 23` survives

**2026-09-03. Static only, no client run, no Ghidra** (the lock was held elsewhere).
Instruments: `tools/reads.py`, `tools/listing.py`, `tools/pdata_lookup.py`, all run with the
repo as the working directory, plus the live `0x0224` capture in `research/fixtures/`.

Tags: **[L]** read off a listing, raw bytes or a log; **[D]** derived from two or more [L];
**[I]** inferred.

Scope: the mask and the tail. The `0xC0000005` at `0x140f9295e` is a different agent's file
(`research/0x0224-remote-user-first-use-fault.md`) and is not touched here.

---

## 0. The answer

**Gated. Fully. Zero bytes.** The loop at `0x140a4a133..0x140a4a1a9` calls
`FUN_140878810(stat, i)->vtable[+0x30](obj, packet)` only when
`(received_mask & CONST[i]) != 0`. We send 124 zero bytes, and `0 & x == 0` for every `x`,
so `FUN_14080fa00` returns false on all eight iterations and the indirect call is **never
reached**. This is now **[L]** — every function in the gate chain has been disassembled —
where both prior passes tagged it **[I]** off the argument list.

**`REMOTE_STAT_TAIL_LEN = 23` survives.** Under an all-zero mask, `FUN_140a46e50` reaches
exactly **seven** packet-carrying call sites and they sum to **124 + 23 = 147**:

| # | site | callee | bytes | note |
|---|---|---|---|---|
| 1 | `0x140a46e95` | `0x1406e9170` | **124** | the mask itself, `mov r8d,0x7c` |
| 2 | `0x140a4a007` | `0x1406e8ae0` | 1 | `u8` |
| 3 | `0x140a4a024` | `0x1406e8ae0` | 1 | `u8` |
| 4 | `0x140a4a041` | `0x1406e8c20` | 4 | `u32` |
| 5 | `0x140a4a10e` | `0x140862470` | **16** | 3×`u32` + `u32 count`; `count = 0` skips the loop |
| 6 | `0x140a4a1c3` | `0x14087ae30` | **0** | per-bit loop, body gated on the mask bit |
| 7 | `0x140a4a29e` | `0x1406e8ae0` | 1 | `u8` |

`1+1+4+16+0+1 = 23`. **[L]**

The `0x140a4a1a5` indirect call is **not in the reachable set at all**, and neither is any
other indirect call or jump. **[L]**

---

## 1. What the gate actually is

Neither prior pass opened `FUN_14080fb80` or `FUN_14080fa00`; both said so, and both named
that as the blind spot. Opened, they are trivial and they settle it.

```text
 000140a4a129  mov  [rsp+0x24], 0                       i = 0
>000140a4a13d  cmp  dword ptr [rsp+0x24], 8
 000140a4a142  jge  0x140a4a1ab                         i >= 8 -> leave
 000140a4a144  mov  ecx, [rsp+0x24]
 000140a4a148  call 0x1402bf710                         rax = &CONST[i]
 000140a4a14d  mov  r8, rax
 000140a4a150  lea  rdx, [rsp+0xe0]                     out
 000140a4a158  lea  rcx, [rsp+0x60]                     THE RECEIVED MASK
 000140a4a15d  call 0x14080fb80                         out = mask & CONST[i]
 000140a4a162  mov  rcx, rax
 000140a4a165  call 0x14080fa00                         any bit set in out?
 000140a4a16f  je   0x140a4a1a9                         <<< no -> skip the body
 ...
 000140a4a18f  mov  rax, [rax+0x30]                     vtable slot at byte +0x30 (index 6)
 000140a4a1a5  call qword ptr [rsp+0x50]                obj->m(obj, packet)
```

**`[rsp+0x60]` is the received mask.** The prologue reads it there straight off the wire:

```text
 000140a46e78  lea  rcx, [rsp+0x60] / call 0x1402c24f0     construct/zero
 000140a46e82  mov  r8d, 0x7c                              124
 000140a46e88  lea  rdx, [rsp+0x60]                        DEST
 000140a46e8d  mov  rcx, [rsp+0x190]                       the packet
 000140a46e95  call 0x1406e9170   <<< READ raw
```
**[L]**

### `FUN_1402bf710` — a table of eight 124-byte constants

```text
 0001402bf710  movsxd rax, ecx
 0001402bf713  lea    rcx, [rip + 0x37fdc06]      -> 0x143abd320
 0001402bf71a  imul   rax, rax, 0x7c              stride 124
 0001402bf71e  add    rax, rcx
 0001402bf721  ret
```
A leaf. It touches no packet. **[L]**

### `FUN_14080fb80` — a dword-wise AND of two 124-byte masks

437 bytes of straight-line code, no branches. It ANDs `[rcx+X] & [r8+X]` for
`X = 0x00, 0x04, … 0x78` — **31 dwords** — and writes all 124 bytes back through `r9`
(`movups [r9], [r9+0x10] … [r9+0x60]`, `movsd [r9+0x70]`, `mov [r9+0x78], edx`), returning
`rax = r9`. No gaps, no packet. **[L]**

### `FUN_14080fa00` — "is any bit set", over exactly 124 bytes

```text
 00014080fa00  xor  eax, eax
>00014080fa02  cmp  dword ptr [rcx + rax*4], 0
 00014080fa06  ja   0x14080fa1a          -> return 1
 00014080fa08  inc  rax
 00014080fa0b  cmp  rax, 0x1e            30 dwords in the loop
 00014080fa0f  jl   0x14080fa02
 00014080fa11  cmp  dword ptr [rcx + 0x78], 0    + the 31st, unrolled
 00014080fa15  ja   0x14080fa1a
 00014080fa17  xor  al, al / ret         <<< all zero -> FALSE
```
A leaf, 30 bytes, no packet. **[L]**

So the gate is exactly `if (received_mask & CONST[i]) != 0`, and it is false for all eight
**independently of what `CONST[i]` contains**, because the received mask is all zero. That
is the strongest available form of the answer: it rests on no unknown.

### The control that says the gate is not vacuous

A gate whose constant were all zero would be false for every input and would prove nothing —
the "instrument that can only return one answer" failure. It is not:

```text
CONST[0] @0x143abd320  popcount 346      CONST[4] @0x143abd510  popcount 348
CONST[1] @0x143abd39c  popcount 324      CONST[5] @0x143abd58c  popcount 331
CONST[2] @0x143abd418  popcount 346      CONST[6] @0x143abd608  popcount 378
CONST[3] @0x143abd494  popcount 338      CONST[7] @0x143abd684  popcount 388

union: 736 distinct bits of 992
```

**736 of the 992 mask bits would fire at least one of the eight.** The gate discriminates,
and the loop is a live length term the moment any remote buff is sent. **[L]**

---

## 2. The instrument, and why it is a different question

Both prior passes asked *"is this call bypassed by some forward branch"* — an interval test
over a listing. Re-running that would have been the mistake `CLAUDE.md` names: *"re-running
the same tool is not a second opinion; changing the question is."*

This pass asks **"starting at the entry, with the mask known to be all zero, which
instructions execute"** — a CFG walk with an abstract state. Exactly two facts are injected
(`0x1402bf6d0` returns 0; `0x14080fa00` returns 0 at `0x140a4a165`), both justified above.
**Every other branch is taken both ways**, so the result over-approximates: a call that does
**not** appear is a strong negative.

```text
CONTROL  extent 0x140a46e50..0x140a4a58f  2978 instructions
CONTROL  186 jumps (0 indirect), 1 ret   [prior pass: 186 jumps, 0 indirect, 1 ret]

1152 of 2978 instructions reachable under the all-zero-mask model
180 branches collapsed to a single edge by the two injected facts
0 indirect jumps/calls reached: none
197 call sites reached; 662 not reached, of which 322 are direct packet reads
```

Controls run **before** anything was believed:

* `python tools/reads.py 0x140304100 2` printed its documented positive control — helper
  reads through `FUN_1403035a0`/`FUN_140303b40`, then direct reads at `140304138 raw`,
  `140304144 u8`, `140304183 u8`, then the `u16` run. **[L]**
* `R.PRIM` carries **11** primitives including `0x142d23ef0` — the one `CLAUDE.md` records a
  stale scratchpad copy missing. Every script here asserts `tools/reads.py` is >8000 bytes
  and inserts `tools/` on `sys.path` explicitly, so the scratchpad copy cannot shadow it.
* The structural triple (186 jumps / 0 indirect / 1 ret) reproduces the prior pass's count
  before the walk is trusted on anything new.
* The indirect-call sweep asserts it can **see** `0x140a4a1a5` before being believed about
  absence. It found 7 indirect calls in the function; the walk reaches none of them.

### The sensitivity control: fact 2 does exactly one thing

An injected fact that quietly suppresses more than it should would produce this same clean
answer. So the walk was re-run with **fact 2 disabled** and everything else identical:

```text
  with both facts:  197 call sites reached,   0 indirect reached
  fact 2 disabled:  199 call sites reached,   1 indirect reached
                    ^^^ the two added are exactly:
                        0x140a4a17d  call 0x140878810          (the object lookup)
                        0x140a4a1a5  call qword ptr [rsp+0x50] (the vtable call)
```

**The same five direct reads, and no new one.** Fact 2 removes the loop body and nothing
else — it is not hiding a read somewhere else in the function. And `0x140a4a1a5` *is*
reachable without it, which is what makes the walk capable of returning the other answer.
**[L]**

### The second filter, composed in the opposite order

A callee cannot consume packet bytes without holding the packet. The packet is argument 3
(`mov [rsp+0x18], r8` at entry; `[rsp+0x190]` after `sub rsp,0x178`). For each of the 197
reached call sites, a backward scan within its own basic block asks which of `rcx/rdx/r8/r9`
was loaded from `[rsp+0x190]`.

`research/avatar-look-loops.md` composed these the other way — packet-carriers first, then
gating. Same universe, opposite order, and they agree on the same seven. **[D]**

Two controls: `0x140a4a041` must come back `rcx` (it is a `u32` read), `0x140a4a017` must
come back empty (it is a setter). Both passed.

The remaining 179 reached call sites are all `call 0x1402bf6d0`, and **all 179 of the 179 in
the function** load `rcx` with `lea rcx,[rsp+0x60]` — the mask, never the packet. Checked
programmatically, not by eye. **[L]**

### Why this also closes the "unknown read primitive" hole

If `R.PRIM` were missing a twelfth primitive, it would show up in the reached set as a plain
`call`. The reached set has only 18 non-bit-test calls and every one is accounted for below —
and the packet-carrier filter does not consult `R.PRIM` at all. A primitive we do not know
about would still have to be handed the packet, and only seven sites are. **[D]**

The packet pointer is **never stored**: every mention of `[rsp+0x190]` in all 2978
instructions is a load into `rcx`, `rdx`, `r8` or `r9`. There is no stack-argument slot and no
global carrying it out of an argument register. **[L]** (The one non-matching hit is
`0x140a495fc mov edx, 0x190`, an immediate.)

---

## 3. Every reached callee, sized

| site | callee | packet? | reads |
|---|---|---|---|
| `0x140a46e7d` | `0x1402c24f0` | no | leaf, mask constructor |
| `0x140a46e95` | `0x1406e9170` | **rcx** | **raw 124** |
| `0x140a4a007` | `0x1406e8ae0` | **rcx** | **u8** |
| `0x140a4a017` | `0x1408977b0` | no | setter, 0 reads at depth 5 |
| `0x140a4a024` | `0x1406e8ae0` | **rcx** | **u8** |
| `0x140a4a034` | `0x140897860` | no | setter, 0 reads |
| `0x140a4a041` | `0x1406e8c20` | **rcx** | **u32** |
| `0x140a4a050` | `0x1408a0fb0` | no | setter, 0 reads |
| `0x140a4a10e` | `0x140862470` | **rdx** | **16** (see below) |
| `0x140a4a124` | `0x140822790` | no | 0 reads |
| `0x140a4a148` | `0x1402bf710` | no | leaf, returns `&CONST[i]` |
| `0x140a4a15d` | `0x14080fb80` | no | the AND, 0 reads |
| `0x140a4a165` | `0x14080fa00` | no | leaf, the any-bit test |
| `0x140a4a1c3` | `0x14087ae30` | **r8** | **0 with a zero mask** |
| `0x140a4a29e` | `0x1406e8ae0` | **rcx** | **u8** |
| `0x140a4a2b0` | `0x14089a340` | no | setter, 0 reads |
| `0x140a4a56a` | `0x1402c23d0` | no | epilogue destructor |
| `0x140a4a582` | `0x142ef44b0` | no | `__security_check_cookie` |

### `FUN_140862470` is 16 bytes, re-read end to end

160 bytes, 3 merged `.pdata` entries, **no indirect calls**, exactly five read sites:

```text
 000140862498  call 0x1406e8c20   READ u32   -> [rsi]
 0001408624a2  call 0x1406e8c20   READ u32   -> [rsi+4]
 0001408624ad  call 0x1406e8c20   READ u32   -> [rsi+8]
 0001408624b8  call 0x1406e8c20   READ u32   <- THE COUNT
 0001408624bd  test eax,eax / jle 0x140862500          count <= 0 -> straight to the epilogue
>0001408624c8  call 0x1406e8c20   READ u32   loop body, `count` times
 0001408624f5  sub rdi,1 / jne 0x1408624c8
>000140862500  ...epilogue, no further reads
```
**[L]** Nothing follows the loop but the epilogue, so with `count = 0` the cost is exactly
16 bytes.

---

## 4. `FUN_14087ae30` re-checked independently — the conclusion holds, the *reason* given was wrong

`research/avatar-look-loops.md` and `research/0x0224-body-walk.md` both wrote that the loop is
*"bounded by `cmp ebx, 0x3e0` = 992 = 124·8"*, and offered that as independent confirmation of
the mask length. **The loop is not bounded by `0x3e0`.** All 169 bytes:

```text
>00014087ae51  mov  r10, [rdi + 0x43d8]      the array base
 00014087ae58  test r10, r10 / je  0x14087aec6        null -> RETURN
 00014087ae5d  mov  r8d, [r10 - 8]           the array's ELEMENT COUNT
 00014087ae61  cmp  ebx, r8d / jae 0x14087aec6        i >= count -> RETURN   <<< the real bound
 00014087ae7d  movsxd rax, ebx / imul r11, rax, 0x58 / add r11, r10          &array[i]
 00014087ae87  cmp  ebx, 0x3e0 / jae 0x14087aec2      -> `inc ebx`, KEEPS LOOPING  <<< not a bound
 00014087aea1  mov  edx, [r14 + rax*4] / shr edx, cl / and edx, 1
 00014087aeaa  je   0x14087aec2                       MASK BIT CLEAR -> skip the body
 00014087aebd  call 0x14036c1f0                       the only packet consumer, reads u32
>00014087aec2  inc  ebx / jmp 0x14087ae51
```

`0x3e0` is an **index guard on the mask**, not the loop's termination condition; the loop ends
on `i >= *(u32*)([stat+0x43d8] - 8)`, a **client-side array length** we do not control. **[L]**

The conclusion is unchanged and in fact stronger: for `i < 0x3e0` the body is gated on the mask
bit (zero → skipped), and for `i >= 0x3e0` the body is skipped outright. **Zero bytes under an
all-zero mask, on both arms.** **[L]**

This matters as a habit, not as a correction to a number: the earlier text used `0x3e0` as
*evidence for the mask length*, and it is not that evidence. The mask length has six other
independent confirmations and does not need this one:

1. `mov r8d, 0x7c` at the raw read — 124.
2. `FUN_1402bf6d0`: `cmp edx, 0x3e0` — 992 bits.
3. `FUN_14087ae30`: the same guard.
4. `FUN_14080fa00`: 30 dwords + the dword at `+0x78` = 31 dwords = 124 bytes.
5. `FUN_14080fb80`: ANDs 31 dwords, writes 0x00..0x7c.
6. `FUN_1402bf710`: `imul rax, rax, 0x7c` — stride 124.

---

## 5. The wire agrees, and it discriminates

`research/fixtures/0x0224-decodes-now-remote-user-faults-on-first-use-world.log` +
`…-hook.log`, same run. **[L]**

```text
world  01:35:36.110  -> 0x0224 Cobalt  (213)  562 bytes      531 + 6 + 5x5
world  01:35:36.212  -> 0x0224 Tester2 (214)  563 bytes      531 + 7 + 5x5
hook   21:35:36.164     9 opcode=0x0224 elapsed_us=2742.8 ret=1
hook   21:35:36.215   536 opcode=0x0224 elapsed_us=1891.2 ret=1
hook   21:35:36.373  CLIENT FAULT #1 0xc0000005 at 0x140f9295e ... 4 C++ throw(s) seen, 4 logged
```

The dispatch line is written **on handler return**, so `ret=1` twice means both handlers were
entered and came back. The last C++ throw is **#4 at 21:35:36.105**, *before* the first
`0x0224` — there is no throw #5, and the fault line's own count confirms it. The reader threw
nothing. **The underrun is gone.** **[L]**

### The check that "no underrun" cannot give

A tail that is too **short** underruns and throws — that is what we fixed. A tail that is too
**long** does not throw: the client reads our surplus as the next field, desyncs, and finishes
early with bytes left over. `ret=1` is blind to that.

So the live bodies were walked against the client's read order, checking a field whose values
the **server** chose:

```text
Cobalt, 562 bytes            predicted        found
  mask offset                61               61,  all 124 zero
  tail                       61+124 .. 208    all 23 zero
  FUN_140862470 count at     203              0            <- the field that read 570 425 344
  avatar look base           216              216
    +6  face                 20002            20002
    +10 job                  200              200
    +15 hair                 30025            30025
    equips                   5 x (slot,item)  (1,1002996) (5,1042999) (6,1062999)
                                              (7,1072999) (11,1322999)
    loop 1 / loop 2 0xFF     260 / 261        260 / 261
```

Tester2 lines up the same way one byte later — look base **217**, face **20001** at +6, both
`0xFF` terminators at 261/262, five equips, count field `0`. **[L]/[D]**

**Negative control, and it is the important half:** with the *old* `REMOTE_STAT_TAIL_LEN = 7`
the look would sit at 200/201, where `face` reads **13 107 200** and **0** instead of 20002 and
20001. The walk therefore **discriminates** — it is not a check that passes on any body of
roughly the right shape, which is the failure mode `CLAUDE.md` records in
`there_is_no_miniroom_and_no_chair`.

---

## 6. What this does not settle — the named blind spots

1. **This says nothing about the fault at `0x140f9295e`.** It says only that the stat decoder
   is not the length term behind it: `FUN_140a46e50` consumes 147 bytes, the builder sends 147,
   the handler returns, and nothing throws. If the residue is a *length* problem it is not in
   this function. That is a negative with a boundary, not a diagnosis.
2. **The vtable behind `0x140a4a1a5` is statically unresolvable here, and it does not matter
   today.** `FUN_140878810(stat, i)` is `return *(void**)(stat + (0x436 + i) * 16)` — a runtime
   pointer at `stat+0x4360 + i*16` (immediately before the `stat+0x43d8` array
   `FUN_14087ae30` walks). Its class is heap state, and `tools/rtti.py`'s own doc records that
   `--vtable` returns nothing for any game class because Themida took the locators. **So the
   eight cannot be sized without a runtime watch** — but they are never entered with a zero
   mask, which is why that does not block this answer. It *will* block the first remote buff.
3. **The moment any mask bit is set, two unsized length terms open at once.** Up to 8
   iterations of `vtable[+0x30](obj, packet)` (unknown width, 736 of 992 bits reach at least
   one) and one `u32` per set bit through `FUN_14087ae30 → FUN_14036c1f0`. Neither is in the
   builder. Remote buffs are not merely unimplemented — sending one with today's builder is a
   guaranteed desync.
4. **A callee could in principle reach the packet through the stat object.** I proved
   `FUN_140a46e50` never stores the packet pointer; I did not audit whether its *caller*
   (`0x1429ce4e4`) stashed it in the object at `[rsp+0x180]` beforehand. Cheap to close and
   not closed.
5. **The two injected facts are the model's whole trust surface.** If `0x1402bf6d0` could
   return nonzero for a zero mask, or `0x14080fa00` for a zero input, 180 collapsed branches
   reopen. Both are 16- and 30-byte leaves, disassembled in full above, with no memory operand
   other than the mask.
6. **Everything outside `FUN_140a46e50` is out of scope.** `research/0x0224-body-walk.md` §5
   already flags that `FUN_1429ce270`'s own unconditional calls have not been enumerated to
   this standard. That is still true and this pass did not do it.
7. **One stale doc block, in a file I was told not to edit.**
   `crates/net/src/userpool.rs:311` still opens *"The seven bytes `FUN_140a46e50` reads AFTER
   the mask"* and lists four reads, above a constant that is now `23` and a body that writes
   eight fields. The number is right and the sentence above it is wrong — exactly the
   *"a comment describing a guarantee is not the guarantee"* shape. Worth one edit by whoever
   owns that file.

---

## 7. Reproducing this

Scripts are in the session scratchpad, not the repo (they answer one question and
`tools/` should not accumulate one-offs). Each asserts `tools/reads.py` is the real one before
running, and each is piped in rather than named, so `sys.path[0]` cannot be the scratchpad:

```
cd C:\MapleCW
python tools/reads.py 0x140304100 2                 <- the control, first, always
python tools/listing.py 0x14087ae30                 <- the per-bit loop, 169 bytes, all of it
python tools/listing.py 0x140862470                 <- the 16 bytes, 160 bytes, all of it
python tools/listing.py 0x14080fb80                 <- the AND
```

`0x1402bf6d0`, `0x1402bf710`, `0x14080fa00`, `0x140878810` and `0x1402c24f0` are **leaves with
no `.pdata` entry**, so `tools/listing.py` refuses them by design — it prints *"has no .pdata
entry - not a function start?"*, which is the tool being right, not a finding about the
function. They were disassembled from raw bytes through `reads.py`'s own loader instead.
