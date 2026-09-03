# `FUN_1402ee8d0`'s two equipment loops — and why they are NOT what killed the clients

**2026-09-03. Static only, no client run, no Ghidra** (another process held the lock; everything
here comes from `tools/` against the raw image, from `research/msexe-avatarlook.c`, and from the
two archived `0x0224` runs).

Tags: **[L]** read off a listing, a capture or raw bytes; **[D]** derived from two or more [L];
**[I]** inferred.

---

## 0. The answer, before the working

* **The two-loops-one-terminator hypothesis is FALSE.** `FUN_1402ee8d0` has exactly two
  equipment loops, each ended by a **single `0xFF` slot byte**; `avatar_look()` emits both; and
  **both are present in the captured killer bodies** — at 244/245 (Cobalt) and 245/246
  (Tester2). The brief's "ONE `0xFFFF`" is two `0xFF` bytes, one per loop. **[L]**
* Simulating the client's documented read sequence over Cobalt's 674-byte body consumes
  **546 bytes**, with loop 1 taking 5 entries and loop 2 taking 0, terminating exactly where the
  builder intended. Nothing in the look runs away *on the offsets the builder assumes*. **[D]**
* **The real fault is 16 bytes earlier, in `REMOTE_STAT_TAIL_LEN`.** `FUN_140a46e50` has an
  **unconditional** `call 0x140862470` at `0x140a4a10e` that reads `u32, u32, u32, u32 count`
  and then **`count` × `u32`**. Those 16 bytes are not in the builder. **[L]**
* Because they are missing, the client reads its `count` from body offset **203** (Cobalt) /
  **204** (Tester2) — which lands on the **low byte of the avatar look's `face`** — giving
  `0x22000000` = **570 425 344** and `0x21000000` = **553 648 128**. The client then asks for
  **2.3 GB** and the `u32` primitive throws. **[D], and it reproduces in both bodies.**
* That is why 128 trailing zeros did not help, and it is also why the *shape* of the brief's
  hypothesis was right: a runaway list read, which zero padding cannot terminate. The runaway is
  just in a different list, 16 bytes upstream.

---

## 1. `FUN_1402ee8d0`, every loop enumerated

`research/msexe-avatarlook.c` lines 60–75, cross-checked against
`python tools/reads.py 0x1402ee8d0 3` (**24 direct reads, no helper reads at depth 3**). **[L]**

The function contains **two** loops and no others:

| loop | slot byte read at | item read at | per iteration | ends on | terminator width |
|---|---|---|---|---|---|
| 1 | `0x1402ee9a7` (first), `0x1402ee9e8` (subsequent) | `0x1402ee9b6` | `u8 slot`, then `u32 itemId` | `slot == 0xFF` | **one byte** |
| 2 | `0x1402ee9f7` (first), `0x1402eea3b` (subsequent) | `0x1402eea06` | `u8 slot`, then `u32 itemId` | `slot == 0xFF` | **one byte** |

```c
bVar2 = read_u8();                       // loop 1
while (bVar2 != 0xff) {
    uVar4 = read_u32();
    if (((byte)(bVar2 - 1) < 0x1f) && FUN_140253980(uVar4, bVar2, 2, 1))
        look[0x39 + bVar2*4] = uVar4;
    bVar2 = read_u8();
}
bVar2 = read_u8();                       // loop 2 — identical, base +0xb9
while (bVar2 != 0xff) { ... }
```

Three things worth pinning, all **[L]**:

* The terminator is compared **as a byte** (`bVar2 != 0xff`), so it is `FF`, **not** `FFFF`.
* The item `u32` is read **before** the slot is range-checked, so a bad slot still costs 5
  bytes — a wrong slot does not desynchronise, it just discards.
* `FUN_140253980` reads nothing from the packet (no helper reads at depth 3), so it cannot
  affect length.

**Minimum look = 195 bytes**: 19 head (`u8 u8 u32 u32 u32 u8 u32`) + 1 + 1 terminators + 174
tail (`4×u32, u32, u8, u32, raw4, raw128, u32, raw13`). **+5 per equipped item.** **[D]**, and
it agrees with `research/user-enter-field.md` §4 exactly.

---

## 2. What `avatar_look()` emits, in order

`crates/net/src/opcode.rs:1031`. **[L]**

```text
u8  gender          u8  skin            u32 0               u32 face
u32 job             u8  0 (discarded)   u32 hair
  per equip: u8 slot, u32 itemId
u8  0xFF            <- end of loop 1's map
u8  0xFF            <- end of loop 2's map
u32 0 ×4            u32 0 (%360)        u8 0                u32 0
raw 4 zeros         raw 128 zeros       u32 0               raw 13 zeros
```

**Both terminators are there, in the right order, and the builder is byte-for-byte what the
reader wants.** No change to `avatar_look()` is indicated by anything in this document.

---

## 3. The lineup against the body that killed both clients

`research/fixtures/0x0224-bodies-that-killed-both-clients.txt`, Cobalt (213), 674 bytes.
Simulated field by field; the avatar look resolves to base **200** from the `face` value alone
(face `20002` = `22 4E` at 206 ⇒ base 206 − 6 = 200), which is independently what
`USER_ENTER_FIELD_LOOK_AT + name.len()` = 194 + 6 predicts. **[D]**

```text
 200  u8  gender 0
 201  u8  skin 0
 202  u32 0
 206  u32 face  = 20002
 210  u32 job   = 200
 214  u8  discarded
 215  u32 hair  = 30025
 219  loop1: slot 1  item 1002996      (219..223)
 224         slot 5  item 1042999
 229         slot 6  item 1062999
 234         slot 7  item 1072999
 239         slot 11 item 1322999      (239..243)
 244  0xFF   <-- loop 1 TERMINATES
 245  0xFF   <-- loop 2 TERMINATES with zero entries
 246  ...tail, 174 bytes...
 419  look ends.  220 bytes consumed = 195 + 5x5.  Exactly what avatar_look() wrote.
```

Total consumed by the whole body model: **546 of 674**, leaving the 128-byte pad untouched.
**The reader does not run away here, and there is no offset at which it would run past 673.**
The same walk on Tester2 (675 bytes, 7-char name) gives terminators at 245/246 and the same
5-entry / 0-entry split. **[D]**

**So: the loops are not the problem, `avatar_look()` needs no byte changed, and the reason
`0x0107` / `0x0114` / `0x0138` / character-select all work with the same function is simply
that it is correct.**

---

## 4. What IS consistent with the evidence

### 4.1 The throw stack is an exact call chain, not stack litter

`research/fixtures/padded-0x0224-still-killed-both-clients-hook.log`, C++ THROW #5. Three
consecutive `<-TEXT` entries, innermost first, each resolved with `tools/pdata_lookup.py`:

```text
0x1406e8cb1  offset +0x91 of FUN_1406e8c20 (147 bytes) -- the u32 read primitive's throw path
0x1408624d0  = 0x1408624cb + 5, the RETURN ADDRESS of  1408624cb call 0x1406e8c20
0x140a4a113  = 0x140a4a10e + 5, the RETURN ADDRESS of  140a4a10e call 0x140862470
```

Every one of those is the return address of a **statically known call site**, and they nest in
the order the listing predicts. **[L]** — this is a real unwind, not the raw stack scan picking
up leftovers, which is the alternative reading and is ruled out by the three addresses forming
a chain the disassembly independently produces.

### 4.2 The function that threw: `FUN_140862470` reads a length-prefixed list

`python tools/listing.py 0x140862470`. **[L]**

```text
140862498  call 0x1406e8c20   READ u32   -> [rsi+0]
1408624a2  call 0x1406e8c20   READ u32   -> [rsi+4]
1408624ad  call 0x1406e8c20   READ u32   -> [rsi+8]
1408624b8  call 0x1406e8c20   READ u32   <-- test eax,eax / jle 0x140862500 : THE COUNT
1408624c6  mov edi, eax
1408624c8: call 0x1406e8c20   READ u32   -> push_back into the vector at [rsi+0x10]
1408624f5  sub rdi,1 / jne 0x1408624c8   <-- count iterations
```

`{ u32; u32; u32; u32 count; u32 items[count]; }` — **16 bytes minimum, 4 per entry.**

### 4.3 It is called unconditionally, and `REMOTE_STAT_TAIL_LEN` does not account for it

`0x140a4a10e call 0x140862470` sits in the block starting at `0x140a4a0ee`, which is the
**target** of the `je` at `0x140a4a0d0`; there is no branch between that label and the call.
**[L]**

Enumerating *every* call in `FUN_140a46e50` that is reached with the packet pointer
(`[rsp+0x190]`) in a register — 330 of them — and testing each against all 186 literal jump
targets gives **exactly seven that nothing jumps over**:

| addr | callee | reads | in the builder? |
|---|---|---|---|
| `0x140a46e95` | `0x1406e9170` | **raw 0x7c = 124** (`mov r8d,0x7c` at `0x140a46e82`) — the mask | yes |
| `0x140a4a007` | `0x1406e8ae0` | u8 | yes |
| `0x140a4a024` | `0x1406e8ae0` | u8 | yes |
| `0x140a4a041` | `0x1406e8c20` | u32 | yes |
| **`0x140a4a10e`** | **`0x140862470`** | **u32 ×4 + count×u32 = 16 min** | **NO** |
| `0x140a4a1c3` | `0x14087ae30` | a mask-**bit** loop (`edx = mask[i>>5] >> (31-(i&31)) & 1`); reads nothing with an all-zero mask | n/a |
| `0x140a4a29e` | `0x1406e8ae0` | u8 | yes |

So the remote temporary-stat **block is 147 bytes**: 124 mask + `u8 u8 u32` + **16** + `u8`.
`REMOTE_STAT_TAIL_LEN` is **23**, not 7. **[L]/[D]**

*Note the mask is read by `FUN_140a46e50` itself at `0x140a46e95`, not by the caller —
`research/user-enter-field.md` §2.2 attributes it to `0x1429ce4e4`, which is the call site. Same
single read either way; the builder emits it once and that is correct.*

### 4.4 The arithmetic, and it reproduces in both captured bodies

With the 16 bytes missing, the client's `count` lands 16 bytes early — on the low byte of the
avatar look's `face`:

```text
              count read from   bytes         value          bytes demanded   body has
Cobalt  (213) body[203..206]    00 00 00 22   570,425,344    2,281,701,376    467
Tester2 (214) body[204..207]    00 00 00 21   553,648,128    2,214,592,512    467
```

`0x22` is the low byte of face `20002` = `0x4E22`; `0x21` is the low byte of face `20001` =
`0x4E21`. **[D]**, computed from the fixture. The first iteration of the loop at `0x1408624c8`
exhausts the body and `FUN_1406e8c20` throws at `0x1406e8cb1` — **which is the frame the hook
log recorded.** Both halves come from the same capture.

**And it explains the padding result exactly:** a 2.3 GB demand is not something 128 zero bytes
can absorb, and zeros are not terminators for a length-prefixed list.

### 4.5 The discriminating control

`0x140a4a113` appears in **2 of 98 distinct hook logs** (deduplicated by content hash over
`previous-runs/` and `research/fixtures/`), and both are the two `0x0224` runs. Zero in the
other 96. `0x1406e8cb1` — "a `u32` read ran off the end of a body" — appears in 4: those two,
plus `npc-action-byte-swap-kills-client-on-sera-chatter-hook.log` and
`maplecw-hook-20260822-205640.log`, both already-known packet-underrun deaths. **[L]** The
signature discriminates.

---

## 5. The change I would make (I have not edited `crates/`)

In `crates/net/src/userpool.rs::user_enter_field`, insert **16 bytes between the `u32` at
`0x140a4a041` and the `u8` at `0x140a4a29e`** — the order is fixed by the addresses:

```rust
    w.u8(0);  // 179  0x140a4a007
    w.u8(0);  // 180  0x140a4a024
    w.u32(0); // 181  0x140a4a041
    // 0x140a4a10e -> FUN_140862470, UNCONDITIONAL. Three u32s, then a u32 COUNT, then
    // count x u32. Omitting these 16 bytes is what read the count out of the avatar look's
    // face and asked the client for 2.3 GB - see research/avatar-look-loops.md.
    w.u32(0); //      0x140862498
    w.u32(0); //      0x1408624a2
    w.u32(0); //      0x1408624ad
    w.u32(0); //      0x1408624b8   <- THE COUNT. Nonzero here reads count x u32.
    w.u8(0);  //      0x140a4a29e
```

and the constants that hang off it:

| constant | from | to |
|---|---|---|
| `REMOTE_STAT_TAIL_LEN` | 7 | **23** |
| `USER_ENTER_FIELD_MIN_LEN` | `508 + 7` = 515 | `508 + 23` = **531** |
| `USER_ENTER_FIELD_LOOK_AT` | `187 + 7` = 194 | `187 + 23` = **210** |
| `USER_ENTER_FIELD_POS_AT` | `426 + 7` = 433 | `426 + 23` = **449** |

`USER_ENTER_FIELD_PAD_LEN` should stay for one more run. Its measurement has now returned
"they still die → not a shortfall", and that reading was **wrong**: it *was* a shortfall, of a
kind padding structurally cannot fix. Keeping it costs 128 bytes and separates "the 16 bytes
were the whole story" from "there is a second one".

### Three test problems this exposes, none of them mine to fix

* `an_empty_name_and_no_equips_is_the_515_byte_minimum` asserts `body.len() == 515` with the
  literal spelled out beside the constant *specifically* so a wrong constant fails. It did not
  fail, because the constant and the literal were wrong **together**. The number has to come
  from somewhere that can disagree — the read enumeration, not a second copy of the same claim.
* `the_remote_stat_block_is_the_124_byte_mask_plus_a_seven_byte_tail` checks the job lands at
  `body[186..188]`. It will need `body[202..204]`. Its `assert_ne!` on the *old* offset is the
  half that is actually doing work and should be kept, retargeted.
* `there_is_no_miniroom_and_no_chair` checks `body[451..455]`, `body[418..422]`, `body[435..438]`
  — **min-body offsets with no tail added at all.** The real miniroom is already at 458 today
  and would be 474 after this change. Every one of those assertions passes vacuously because the
  neighbourhood is all zeros: this is `CLAUDE.md`'s "a test that pins what the code already does
  is not a check", and it would pass just as happily on the body that killed two clients.

---

## 6. Instrument checks, and the blind spots

### Controls run first

* `python tools/reads.py 0x140304100 2` printed reads at `140304138 raw`, `140304144 u8`,
  `140304183 u8`, then a run of `u16` — its documented positive control, matching
  `research/user-enter-field.md` §6 exactly. Passed **before** anything else was run.
  `tools/listing.py` shares that loader and extent by construction.
* The unconditional-path scan counts **186 literal jumps, 0 indirect jumps, 1 `ret` at
  `0x140a4a58e`** — the same three numbers `crates/net/src/userpool.rs`'s
  `REMOTE_STAT_TAIL_LEN` doc block arrived at independently. The scan reproduces a known count
  before being trusted on a new one.
* The hook-log sweep deduplicated **by content hash** before counting, per `CLAUDE.md`.

### One instrument was broken and caught itself

The packet-carrying-call scan's first version reported **330 calls, 0 gated** — structurally
impossible in a function with 186 conditional jumps. Its jump regex was anchored with `$` and
`tools/listing.py` appends `[branch]` after the target, so it parsed **zero** jumps and every
call came back "UNCONDITIONAL". It now asserts `len(jumps) == 186` before doing anything. This
is the same failure as the `awk` test summary in `CLAUDE.md`: a clean, confident answer from a
filter that could only ever return one value.

### Named blind spots

1. **The absence of `0x1402ee8d0` from the hook logs is NOT evidence.** It appears in **zero**
   of 98 logs — including runs where the avatar look demonstrably worked (character select,
   `0x0107`). The hook's stack scan simply never captures it. The look is exonerated by the
   byte-level walk in §3, not by that absence.
2. **`0x140a4a1a5 call qword ptr [rsp+0x50]`** — an 8-iteration loop calling `vtable[0x30](obj,
   packet)`. Indirect; no static instrument here follows it. The scan shows it **gated** (inside
   the `if (FUN_14080fa00(...))` test, whose input comes from the mask via
   `FUN_14080fb80(mask, buf, FUN_1402bf710(i))`), so an all-zero mask should skip all eight —
   but "should" is **[I]**, and if a residue remains after the 16-byte fix this is the first
   place to look.
3. **`FUN_14087ae30` reads nothing only because the mask is zero.** Its loop reads one `u32` per
   **set** bit (`0x14087aebd call 0x14036c1f0`). Correct today; it becomes a length term the
   moment a remote buff is ever sent, and the bit order is big-endian within each dword
   (`31 - (i & 31)`).
4. **`count` is signed-tested** (`test eax,eax / jle`), so a negative count is safe — but any
   nonzero positive value here is `4 × count` bytes. This is a second field with the same hazard
   as the miniroom dword at 451.
5. **Nothing here has been on the wire.** The fix is derived from a listing and two archived
   bodies. `0x0224` has never once returned from its handler.
