# `0x0224` — walking the decoder against the bytes that killed both clients

**2026-09-03. Static only — no client run, no Ghidra (the project was locked by a sibling).**
Instruments: `tools/reads.py`, `tools/listing.py`, `tools/callers.py`, `tools/pdata_lookup.py`,
`tools/dump_va.py`, all run with the repo as the working directory, plus the two archived
hook logs in `research/fixtures/`.

Tags: **[L]** read off a listing, raw bytes or a log; **[D]** derived from two or more [L];
**[I]** inferred.

---

## 0. The answer

**The body is 16 bytes short at body offset 185, and the fourth of those missing bytes is a
`u32` COUNT.** The client reads the count out of the middle of the avatar look instead, gets
**570 425 344**, and loops reading `u32`s until the reader throws.

```text
  the four missing fields, read by FUN_140862470 at 0x140a4a10e:

  140862498  u32                    Cobalt got  51 200   <- the u16 job + 2 bytes of padding
  1408624a2  u32                    Cobalt got       0
  1408624ad  u32                    Cobalt got       0
  1408624b8  u32  COUNT             Cobalt got 570 425 344 = 0x22000000
                                                              ^^ the top byte of face 20002
  1408624c8  loop COUNT x u32       -> 2 281 701 376 bytes wanted, 467 available
```

`FUN_140862470` is called **unconditionally** from inside `FUN_140a46e50`, the remote
temporary-stat decoder. It is not a *direct* read, so the three independent analyses in
`research/user-enter-verification.md` §1.3 — forced-value reachability, dominator analysis and
raw bytes — could not see it: **all three enumerate direct READS, and this is a CALL.** That is
the same class of blind spot that hid the 124→131 correction one level down, one level further
in.

**And `tools/reads.py` printed it the whole time.** `python tools/reads.py 0x140a46e50 3` has
always contained the line

```text
  0x140a4a10e  call 0x140862470 -> READS via helper: u32  gated?
```

It and `0x140a4a1c3` are the **only two** helper lines in that dump — lines 306 and 307 of 330 —
and both carry `gated?`. So does almost everything else: **328 of the 329 read lines in that
function are marked `gated?`**, because `reads.py` flags every read that follows any conditional
branch and `FUN_140a46e50` is 183 bit-tests long. Only the very first line, the mask read at
`0x140a46e95`, comes back unmarked.

That is the whole failure. `user-enter-field.md` §6 blind spot 5 already recorded that the
column over-reports; in this function it over-reports on everything, so it carries **no
information at all** and the two lines that mattered were invisible by contrast. This is
`CLAUDE.md`'s *"the fixed tool's own output had already printed the missing read; it scrolled
past unread"*, exactly. The instrument was right and was read as a negative.

**The generalisable form: a warning flag that fires on 328 of 329 rows is not a warning, it is
noise, and the rows it is hiding are the ones nobody has resolved by hand.** Two helper lines in
330 is a *small* set — small enough to open both, which is one `tools/listing.py` call each.

**This is why 128 trailing zeros did not help, and could never have helped.** The pad buys 32
loop iterations out of 570 million. It is not a shortfall that zeros can cover; it is a count.

### It is measured, not only derived

The fault stack in **both** archived runs carries the frame chain, in order:

```text
0x1406e8cb1  <- inside FUN_1406e8c20, the u32 reader (0x1406e8c20..0x1406e8cb3)  [pdata_lookup]
0x1408624d0  <- return address of `call 0x1406e8c20` at 0x1408624cb, INSIDE THE COUNT LOOP
0x140a4a113  <- return address of `call 0x140862470` at 0x140a4a10e
```

`research/fixtures/two-clients-one-map-both-died-inside-0x0224-hook.log` (pid 967104, 00:14:03)
and `research/fixtures/padded-0x0224-still-killed-both-clients-hook.log` (pid 970748, 00:28:19)
— different pids, different times, different bodies, different md5. Both contain
`0x140a4a113` three times and `0x1408624d0` twice. **[L]**

`0x1406e90e8` (inside the `str` reader, `0x1406e9050..0x1406e9144`) also appears on the same
line, *above* `0x140a4a113`. The hook prints raw stack words that look like code addresses, not
a real unwind, so that one is most likely residue from the name/guild string reads earlier in
the same frame. **Do not read it as a second live frame** — the three above are consecutive and
consistent, which is what makes them evidence.

---

## 1. The instruments, and the control each passed first

* `python tools/reads.py 0x140304100 2` printed its documented control — reads through
  `FUN_1403035a0` and `FUN_140303b40` *and* directly, at the documented addresses. **[L]**
* The gating walker used throughout this document (a forward-conditional-interval test over a
  `tools/listing.py` dump) was run on `FUN_140a46e50` first and returned **exactly the five
  unconditional reads** — `raw` at `140a46e95`, `u8` at `140a4a007`, `u8` at `140a4a024`, `u32`
  at `140a4a041`, `u8` at `140a4a29e` — which `research/user-enter-verification.md` §1.3
  established by three different methods. A positive control on a subject whose answer was
  already known by other means. **[L]**
* The finding itself was then re-derived from **raw bytes, with no disassembler in the path**
  (§2.2), because two runs of the same interval algorithm are one opinion, not two.

---

## 2. `FUN_140862470` is unconditional

### 2.1 From the listing

```text
>000140a4a0bc  mov edx, 0x109 / lea rcx,[rsp+0x60] / call 0x1402bf6d0   the mask bit test
 000140a4a0d0  je  0x140a4a0ee                                          bit clear -> skip
 000140a4a0da  call 0x1406e8c20   READ u32                              (gated, we skip it)
>000140a4a0ee  mov rax,[rsp+0x180] / add rax,0x24e0                     <- the je LANDS HERE
 000140a4a101  mov rdx,[rsp+0x190]                                      the PACKET
 000140a4a109  mov rcx,[rsp+0x48]
 000140a4a10e  call 0x140862470                                         <<< NOT GATED
 000140a4a113  ...                                                      <- on the crash stack
```

A whole-function scan for *calls* rather than reads finds only three unconditional calls in
`FUN_140a46e50` that are neither the mask bit test `0x1402bf6d0`, a setter, nor the epilogue:
`0x140a4a10e → FUN_140862470`, `0x140a4a124 → FUN_140822790`, and
`0x140a4a1c3 → FUN_14087ae30`. **[L]**

### 2.2 From raw bytes, no disassembler

`python tools/dump_va.py 0x140a4a0bc 96`:

```text
000140a4a0cc  b6 c0 85 c0 74 1c 48 8b 8c 24 90 01 00 00 e8 41
                          ^^^^^ je +0x1c  ->  0x140a4a0d2 + 0x1c = 0x140a4a0ee
000140a4a0ec  e4 ff 48 8b 84 24 80 01 00 00 48 05 e0 24 00 00
000140a4a0fc  48 89 44 24 48 48 8b 94 24 90 01 00 00 48 8b 4c
000140a4a10c  24 48 e8 5d 83 e1 ff ...
                    ^^^^^^^^^^^^^^^ call rel32 -0x1e7ca3 -> 0x140a4a113 - 0x1e7ca3 = 0x140862470
```

Between `0x140a4a0ee` and `0x140a4a10e` the bytes are `48 8b 84 24 80 01 00 00 / 48 05 e0 24 00
00 / 48 89 44 24 48 / 48 8b 94 24 90 01 00 00 / 48 8b 4c 24 48 / e8`. **No `0x7x` short Jcc, no
`0f 8x` near Jcc, no `eb`.** The `je` of the preceding bit test lands *on* `0x140a4a0ee`, so the
block is reached by the branch and by fall-through both. **[L]**

### 2.3 What it reads

`python tools/listing.py 0x140862470` — 160 bytes, three merged `.pdata` entries:

```text
000140862484  mov [rcx], 0                    the constructor RESET: three dwords and
00014086248a  mov [rcx+8], 0                  an empty vector is the "absent" form, and
00014086248d  mov rax,[rcx+0x10] / [rcx+0x18] = rax     it is written before any read
000140862498  call 0x1406e8c20   READ u32     -> [dst+0]
0001408624a2  call 0x1406e8c20   READ u32     -> [dst+4]
0001408624ad  call 0x1406e8c20   READ u32     -> [dst+8]
0001408624b8  call 0x1406e8c20   READ u32     -> COUNT
0001408624bd  test eax,eax / jle 0x140862500  COUNT <= 0 skips the loop        <- zero is right
>0001408624c8  call 0x1406e8c20  READ u32     push into the vector at [dst+0x10]
0001408624f5  sub rdi,1 / jne 0x1408624c8     COUNT iterations
```

**16 bytes unconditional, then 4 per count.** `dst` is `CSecondaryStat + 0x24e0`. **[L]**

The right absent form is **16 zero bytes**, and that is [L] rather than [I]: the function's own
prologue resets the three dwords to `0` and the vector to empty *before* reading, so zeros
reproduce exactly the state the reset leaves. A name for the struct would be [I] and the v214
tree scores 1 of 8 — it is not needed to fix this and none is offered.

### 2.4 The two neighbours, and why they cost nothing

* **`0x140a4a1c3 → FUN_14087ae30(stat, mask, packet, 1)`** is also unconditional, and it reads
  **zero bytes from an all-zero mask**. Its own listing is a per-bit loop over the mask:
  `mov edx,[r14+rax*4] / shr edx,cl / and edx,1 / je skip`, with `r14 = rdx =` the 124-byte
  mask. Every bit clear means every iteration skips. **[L]**

  > **Corrected 2026-09-03 by `research/remote-stat-mask-gating.md`.** This paragraph said the
  > loop was "bounded by `cmp ebx, 0x3e0` = 992 = 124·8" and offered that as an independent
  > confirmation of the mask length. It is not the bound: the loop ends on
  > `i >= *(u32*)([stat+0x43d8]-8)`, a client-side array length, and `0x3e0` is an index guard
  > whose `jae` goes to `inc ebx` rather than to the exit. **The conclusion survives - zero
  > bytes either way - but the reason given for it was wrong, and it was being cited as
  > corroboration for a number it cannot speak to.** The mask length has six other independent
  > confirmations and never needed this one.
* **`0x140a4a124 → FUN_140822790`** takes `rcx = stat+0x2530` and no packet. `tools/reads.py`
  at depth 4 finds no reads under it, nor under `FUN_1402c23d0`, `FUN_1404a3180`,
  `FUN_14080fb80`, `FUN_14080fa00` or `FUN_1402bf6d0`. **[L]**
* The **8-iteration loop** at `0x140a4a133..0x140a4a1a9` decodes through
  `call qword [rsp+0x50]` with the packet in `rdx`, gated on
  `FUN_14080fa00(FUN_14080fb80(mask, …, FUN_1402bf710(i)))`. The mask is the first argument, so
  a zero mask skips all eight — **[D]**, and here is its blind spot: *I did not open
  `FUN_14080fb80` or `FUN_14080fa00`, so "zero mask ⇒ false" is read off the argument, not
  proved. If the eight ever fire they are indirect and no static instrument here can size them.*

---

## 3. The walk, offset by offset, against the captured Cobalt body

`research/fixtures/0x0224-bodies-that-killed-both-clients.txt`, 674 bytes, name 6 bytes, 5 equips.
Every offset below is the byte position in that file; every address is the instruction that
reads it. **[L]** for the bytes, **[L]** for the addresses.

| off | at | type | value | verdict |
|---|---|---|---|---|
| 0 | `1429ba40a` | u32 | 213 | ok |
| 4 | `1429ba43b` | u32 | 213 | ok — nonzero, so the three-u32 header, no fourth read |
| 8 | `1429ba4a0` | u32 | 0 | ok |
| 12 | `1429ce310` | u32 | 18 | ok, level |
| 16 | `1429ce323` | str | `"Cobalt"` | ok |
| 24 | `1429ce371` | str | `""` | ok |
| 26 | `1429ce3bb` | u32 | 0 | ok |
| 30 | `1429ce3ce` | str | `""` | ok |
| 32..60 | `…ce418`..`…ce4c2` | u16 u8 u16 u8 u32 u32 u8 u32 u32 u8 u32 u8 | all 0 | ok |
| 61 | `140a46e95` | raw 124 | all zero | ok — `mov r8d,0x7c` is 124, and no bit is set |
| 185 | `140a4a007` | u8 | 0 | ok |
| 186 | `140a4a024` | u8 | 0 | ok |
| 187 | `140a4a041` | u32 | 0 | ok |
| **191** | **`140862498`** | **u32** | **51 200** | **MISSING — the client is reading the job field** |
| **195** | **`1408624a2`** | **u32** | **0** | **MISSING** |
| **199** | **`1408624ad`** | **u32** | **0** | **MISSING** |
| **203** | **`1408624b8`** | **u32 COUNT** | **570 425 344** | **MISSING — this is the divergence** |
| 207+ | `1408624c8` | u32 × count | — | throws at iteration 117; 467 bytes were left |

`0x22000000` is the top byte of **face id 20002 = 0x4E22**, sitting at body offset 206 inside the
avatar look, arriving in the count's most significant byte. **[D]**

**The second client is the same field, the same way.** Tester2, 675 bytes, name 7 bytes: the
whole block shifts one byte, so the count is read at offset 204 and picks up the top byte of
**face 20001 = 0x4E21** → `0x21000000` = **553 648 128**. Both throw on iteration 117 with 467
bytes remaining. Two bodies, two counts, one mechanism — and "the same" is a much stronger
observation than "different". **[L]/[D]**

---

## 4. Everything else in the body is correct

Checked and **not** the problem, so that the fix below is one change and not several:

* **Every loop in `FUN_1429ce270` whose iteration count comes from the body terminates on what
  we send.** All twenty backward branches enumerated; only five contain reads:
  the pet loop `1429cec30`, the familiar loop `1429cecb0` (both `u8 != 0` to continue — a zero
  ends them), the vehicle loop `1429cfc20` (client-state gated, §3 of `user-enter-field.md`),
  and the two count loops below. **[L]**
* **The two `u32 count` fields at builder offsets 500 and 504 — zero is right, and it is right
  for the reason that matters.** Both are `test eax,eax / jle`:
  `0x1429cfd5b jle 0x1429cff0b` and `0x1429cff1c jle 0x1429d001a`. A *signed* compare, so zero
  skips the loop and does not wrap to four billion. This was the first thing checked and it is a
  clean negative. **[L]** Blind spot named: `jle` also skips on negative, so a body that ever
  carried a negative count would be silently ignored rather than caught — not a hazard while we
  send literal zeros.
* **The unconditional callees are all the length the builder assumes.** `FUN_14073a7b0` =
  `raw 4` (`mov r8d,4`) + `u32` + `raw 4` (`mov r8d,4`) + `str` + `u32` = **18**;
  `FUN_142835840 → FUN_1408cf0d0` = `u8` + `u32` = **5**; `FUN_1428358a0` = `u32` = **4**;
  `FUN_142834df0` = one unconditional `u32` count; `FUN_1427eec30` and `FUN_142833030` read
  nothing at depth 4. **[L]**
* **The 409-byte gap between `0x1429ce510` and the avatar-look call at `0x1429ce6a9` contains no
  packet read** — only `FUN_14019b600` (allocator) and two `FUN_142e5xxxx` bounds checks. Same
  for `0x1429ce8b1..0x1429cea87`, where the wide branch at `0x1429ce916 → 0x1429d02e3` is a
  COM/HRESULT bail (`0x80004002`, `0x80000000`), not a wire gate. **[L]**
* **The avatar look is not the problem and I did not re-derive it** — it is
  `research/avatar-look-loops.md`'s subject. For the record of where my walk handed off: for
  Cobalt it starts at **body offset 200** (`USER_ENTER_FIELD_LOOK_AT` 194 + 6 name bytes) and
  runs 220 bytes to 420; `FUN_1402ee8d0` has exactly **two** backward branches, matching the two
  `0xFF` terminators the builder writes; and the bytes at 200..420 decode as gender 0, skin 0,
  face 20002, job 200, hair 30025, five equips, `ff ff`. Everything after 420 lines up with the
  builder on that basis.

---

## 5. The change, in `crates/net/src/userpool.rs`

Not applied — the brief forbade editing `crates/`. Three edits and their tests.

**1. `REMOTE_STAT_TAIL_LEN`: `7` → `23`.** Every derived constant then moves on its own:
`USER_ENTER_FIELD_MIN_LEN` 515 → **531**, `USER_ENTER_FIELD_LOOK_AT` 194 → **210**,
`USER_ENTER_FIELD_POS_AT` 433 → **449**.

**2. Four `u32`s into the tail, between `0x140a4a041` and `0x140a4a29e`** — the order is the
address order, and `FUN_140a46e50` has no backward branch that reorders them:

```rust
    w.u8(0);  //                  179  0x140a4a007
    w.u8(0);  //                  180  0x140a4a024
    w.u32(0); //                  181  0x140a4a041
    // 185: FUN_140862470, called UNCONDITIONALLY at 0x140a4a10e - three u32 and then a
    // u32 COUNT, with `count` more u32 behind it. Not a direct read, which is why three
    // separate read-enumerations missed it; 0x140a4a113 and 0x1408624d0 are on the fault
    // stack of both archived runs. A nonzero count here is 4*n more bytes.
    w.u32(0); //                  185  0x140862498
    w.u32(0); //                  189  0x1408624a2
    w.u32(0); //                  193  0x1408624ad
    w.u32(0); //                  197  0x1408624b8   COUNT - jle, so 0 skips the loop
    w.u8(0);  //                  201  0x140a4a29e
```

**3. Keep `USER_ENTER_FIELD_PAD_LEN = 128` for this one run.** It is already in the crashing
body, so leaving it means exactly one thing changes, which is what `CLAUDE.md` asks for. It also
stays a net: if a *second* short field exists the run survives and we get to see it. Remove it
after a launch that does not die.

### Tests that will need to move, and one that is already vacuous

* `an_empty_name_and_no_equips_is_the_515_byte_minimum` — `515` → **531**, and
  `REMOTE_STAT_MASK_LEN + REMOTE_STAT_TAIL_LEN` `131` → **147**. Rename it too.
* `only_the_name_and_the_equips_change_the_length` — three literal `515`s → **531**.
* `the_remote_stat_block_is_the_124_byte_mask_plus_a_seven_byte_tail` — `REMOTE_STAT_TAIL_LEN`
  `7` → **23**, `block` `55..186` → **`55..202`**, and the job read-back moves from
  `body[186..188]` to `body[202..204]`. This test is the one that is *supposed* to catch this
  class of error and it should be the one that fails first.
* **`there_is_no_miniroom_and_no_chair` asserts nothing today.** It checks `body[451..455]`,
  `body[418..422]`, `body[435]`, `body[436]`, `body[437]` are zero — but those offsets have been
  stale by `REMOTE_STAT_TAIL_LEN` since 2026-08-31, and *every* byte from 246 to the end of the
  body is zero, so it passes on any body of roughly the right shape. It agreed with the builder
  while the builder was wrong. Add `+ USER_ENTER_FIELD_TAIL_SHIFT` (or express the offsets
  relative to `USER_ENTER_FIELD_POS_AT`) and give it a nonzero control byte to discriminate
  against, or it will keep passing through the next one of these.

### What this does not settle

* **Nothing here has been on the wire.** No corrected `0x0224` has been sent. The claim is that
  the body was 16 bytes short at 185; the claim that 531 is now the *whole* length rests on the
  same kind of enumeration that has now been wrong twice at two different depths.
* **The failure was found by enumerating unconditional CALLS, not reads.** The same walk has not
  been done on `FUN_1429ce270`'s own unconditional calls to the same standard — I checked the
  six named in §4 and the two inside `FUN_140a46e50`. A packet-consuming call at some other
  unconditional site in `FUN_1429ce270` would look exactly like this one did.
* **`FUN_140862470`'s sibling call site is gated, so the local path is unaffected.**
  `tools/callers.py` gives two call sites: `0x140a4a10e` here, and `0x140a45ad8` inside
  `FUN_140a165f0`, the *local* secondary-stat decoder — where it **is** gated by a forward
  branch (`0x140a45ab0 → 0x140a45add`). That is why this has never bitten the local buff path.
  Blind spot: I tested that one call site's gating, not the whole local decoder.

### The watch worth arming on the next run

`0x1408624b8` (the count read) with `0x140304100:hits=200` as the positive control. If the
count comes back `0`, the field landed where the client reads it. If it comes back large again,
the tail is still the wrong length and the value it prints says by how much.
