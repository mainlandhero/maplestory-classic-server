# `FUN_140304b20` — character-record decoder: loop census

Source: full instruction listing `0x140304b20 .. 0x14030937d` (4934 instructions, 18525 bytes).
Method: parsed the listing, built a CFG, computed dominators, and took **natural loops** from
back-edges (`a -> h` where `h` dominates `a`). Loop bodies are dominator-derived, not address ranges.

Every claim below is tagged:

* **[L]** = read directly off the listing.
* **[D]** = derived by analysis over the listing (dominators, reaching definitions, control dependence).
* **[I]** = inferred, not provable from this listing alone.

---

## 0. Listing integrity

* **[L]** The listing is **fully contiguous** — 4934 instructions, no address gaps, no `??`, no
  "bad instruction data", no undefined opcodes. All mnemonics are sensible x86-64. **No Themida
  damage in this region.**
* **[L]** There is **no `RET` and no `POP` anywhere in the listing.** The last instruction is
  `0x140309379 CALL 0x142ef44b0` (the stack-cookie check). The epilogue therefore lies **past the
  stated function end**, at `0x14030937e` onward. The listing is truncated by roughly 15-20 bytes.
* **[L]** Three conditional jumps leave the listing: `0x14030588e JZ 0x140309398`,
  `0x140305bc1 JZ 0x14030939e`, `0x140305ca9 JZ 0x140309392`. Their targets are 6-byte stubs
  between the epilogue and the jump tables. **[I]** They cannot hold a packet read (the calling
  convention needs `MOV RCX,reg` + `CALL rel32` = 8 bytes minimum), so they do not affect this census.
* **[L]** Four indirect `JMP RAX`/`JMP RCX` dispatch through jump tables at `0x1403093a4`,
  `0x1403093bc`, `0x1403093d4`, `0x1403093ec` — **data past the function end, not in the listing.**
  Their case bodies *are* in the listing (four 53-byte runs at `0x14030552b`, `0x140305e90`,
  `0x1403068a3`, `0x140306995`); **[L]** each only does `LEA RDX,[<static string>]` and reconverges.
  **No packet reads inside them.** I reconnected them synthetically; the CFG is then 100% reachable.

---

## 1. Read primitives — corrected

**[L]** Seven primitives are called, not five. Confirmed against the coordinator's corrected table:

| target | kind | width | sites |
|---|---|---|---|
| `0x1406e8ae0` | u8 | 1 | 16 |
| `0x1406e8b80` | u16 | 2 | 34 |
| `0x1406e8c20` | u32 | 4 | 49 |
| `0x1406e8f00` | u32 (thunk → `0x1406e8c20`) | 4 | 5 |
| `0x1406e8f10` | u64 | 8 | 4 |
| `0x1406e9050` | string (u16 len + bytes) | 2+n | 7 |
| `0x1406e9170` | raw, n in R8D | n | 11 |

**Total 126 reads** (u8 16, u16 34, u32 54, u64 4, str 7, raw 11). **[L]** Matches exactly.

**[D]** All 9 previously-missed reads (5 thunked u32, 4 u64) are **inside loop bodies. None is a
loop count.** The straight-line spine is unchanged at 61 reads. They did, however, turn two
previously read-free loops into read-bearing ones (`0x140307550`, `0x140307616` — both cleanly
count-guarded; see the table).

| missed read | kind | enclosing loop | role |
|---|---|---|---|
| `0x140305195` | u64 | `0x140305171` | body |
| `0x1403074f3` | u32 | `0x1403074f0` | body |
| `0x140307553` | u32 | `0x140307550` | body |
| `0x1403075c8` | u32 | `0x1403075c5` | body |
| `0x140307619` | u32 | `0x140307616` | body |
| `0x140308a73` | u32 | `0x140308a70` | body |
| `0x140308f9a` | u64 | `0x140308f97` | body |
| `0x140308fe7` | u64 | `0x140308fe4` | body |
| `0x140309246` | u64 | `0x140309243` | body |

### Raw-read sizes (all constant)

**[L]** Every `0x1406e9170` call has a compile-time-constant `R8D`:

| site | R8D | source |
|---|---|---|
| `0x140304b95` | **0x64 = 100** | `LEA R8D,[RAX + 0x64]` at `0x140304b8e`; `XOR EAX,EAX` at `0x140304b72`, RAX untouched in between |
| `0x140304cc6` | 8 | `MOV R8D,0x8` |
| `0x140304d1f` | 8 | `MOV R8D,0x8` |
| `0x140304e2d` | 8 | `MOV R8D,0x8` |
| `0x140304ff5` | 8 | `MOV R8D,0x8` |
| `0x140306c1c` | 8 | `MOV R8D,0x8` |
| `0x140306ce2` | 8 | `MOV R8D,0x8` |
| `0x140306cf7` | 8 | `MOV R8D,0x8` |
| `0x140306dcf` | 8 | `MOV R8D,0x8` |
| `0x140307042` | 8 | `MOV R8D,0x8` |
| `0x1403075df` | 8 | `MOV R8D,0x8` |

The first read is **not** variable-length — `LEA R8D,[RAX+0x64]` is just MSVC materialising 100
while EAX is known-zero. **[L]** Its destination is `RDX` = **arg2**, which the prologue zeroes
across `[0..0x63]` (six `MOVUPS` + one `MOV dword [RDX+0x60]`) immediately before the read.

---

## 2. Back-edge triage

**[L]** 150 raw backward jumps. **[D]** 145 natural loops (distinct headers). The 150→145 gap is
multi-latch loops, not extra loops.

**[D]** Classification of the 145:

| class | count | what it is |
|---|---|---|
| 100-byte string scan | 47 | `CMP byte ptr [RAX],0 / INC ECX / INC RAX / CMP ECX,0x64 / JC` — the presence-flag test, **compiler artifact, no reads** |
| intrusive list walk | 16 | `MOV RBX,[RCX+8] ... TEST RBX,RBX / JNZ` — destructor/free walks over client-side lists, **no reads** |
| other | 82 | mix of real packet loops and client-side processing |
| **containing >=1 packet read** | **37** | the subject of this document |

108 of the 145 loops contain no packet read at all and are irrelevant to packet construction.
**[L]** Notably `0x140307c20..0x140308a1a` (3578 bytes) and `0x140305190..0x140305e47` (~3.2 KB)
contain **zero** packet reads — pure client-side processing.

---

## 3. The loop table

`Guard` = the branch **before** the loop head that jumps past it. **Every entry marked "top" was
verified two ways [D]: the guard's tested register traces back to the named read, and the loop's
own decrement counter traces back to the same read.**

| # | head | back-edge | count field | width | test position | zero count skips? | reads in body |
|---|---|---|---|---|---|---|---|
| 1 | `0x140304c70` | `0x140304c8d` | `0x140304c61` | u8 | top (`TEST EAX,EAX / JZ 0x140304c8f`) | **yes** | `0x140304c73` u32 |
| 2 | `0x140304ca0` | `0x140304ce6` | `0x140304c92` | u32 | top (`TEST EAX,EAX / JLE 0x140304ce8`) | **yes** | `0x140304ca3` u32, `0x140304cc6` raw(8) |
| 3 | `0x140304d12` | `0x140304dfe` | `0x140304d02` | u32 | top (`TEST EAX,EAX / JLE 0x140304e0e`) | **yes** | `0x140304d1f` raw(8) |
| 4 | `0x140304e20` | `0x140304e36` | `0x140304e11` | u32 | top (`TEST EAX,EAX / JLE 0x140304e38`) | **yes** | `0x140304e2d` raw(8) |
| 5 | `0x140305171` | `0x1403051b2` | `0x140305166` | u32 | top (`TEST EAX,EAX / JLE 0x1403051b4`) | **yes** | `0x140305174` u32, `0x14030517f` u32, `0x14030518a` u32, `0x140305195` **u64** |
| 6 | `0x140305e01` | `0x140306092` | **none — fixed 6** | — | n/a | **NO — see §4** | `0x140305e48` u16 (gated by flag #13) |
| 7 | `0x140306210` | `0x1403062d7` | **sentinel, not a count** | u16 | top | **yes, send 0** | `0x1403062c9` u16 (the next sentinel) |
| 8 | `0x1403066a0` | `0x1403067fc` | **sentinel, not a count** | u16 | top | **yes, send 0** | `0x1403067ee` u16 (the next sentinel) |
| 9 | `0x140306972` | `0x140306b4b` | **none — fixed 3** | — | n/a | **NO — see §4** | `0x140306a0d` u32 (count for #10) |
| 10 | `0x140306a20` | `0x140306b39` | `0x140306a0d` | u32 | top (`TEST EAX,EAX / JLE 0x140306b44`) | **yes** | `0x140306a23` u32 |
| 11 | `0x140306c01` | `0x140306c38` | `0x140306bf6` | u32 | top (`TEST EAX,EAX / JLE 0x140306c3a`) | **yes** | `0x140306c04` u32, `0x140306c1c` raw(8) |
| 12 | `0x140306cd1` | `0x140306d13` | `0x140306cc6` | u32 | top (`TEST EAX,EAX / JLE 0x140306d15`) | **yes** | `0x140306ce2` raw(8), `0x140306cf7` raw(8) |
| 13 | `0x140306d93` | `0x140306e19` | `0x140306d57` | u16 | top (`TEST EDI,EDI / JZ 0x1403073c3`) | **yes** | `0x140306d96` u32, `0x140306da3` u32, `0x140306dcf` raw(8), `0x140306df9` u32 |
| 14 | `0x140306e40` | `0x140306f18` | `0x140306e29` | u16 | top (`TEST ESI,ESI / JZ 0x140306f1e`) | **yes** | `0x140306e43` u32, `0x140306e52` u32 |
| 15 | `0x140306f31` | `0x140306ffb` | `0x140306f21` | u16 | top (`TEST EDI,EDI / JZ 0x140307001`) | **yes** | `0x140306f34` u32 |
| 16 | `0x140307026` | `0x1403070fc` | `0x14030700f` | u16 | top (`TEST ESI,ESI / JZ 0x140307102`) | **yes** | `0x140307029` u32, `0x140307042` raw(8) |
| 17 | `0x140307115` | `0x1403071db` | `0x140307105` | u16 | top (`TEST EDI,EDI / JZ 0x1403071e1`) | **yes** | `0x140307118` u32 |
| 18 | `0x140307200` | `0x1403072d8` | `0x1403071e4` | u16 | top (`TEST ESI,ESI / JZ 0x1403072de`) | **yes** | `0x140307203` u32, `0x140307212` u32 |
| 19 | `0x1403072f1` | `0x1403073bb` | `0x1403072e1` | u16 | top (`TEST EDI,EDI / JZ 0x1403073c1`) | **yes** | `0x1403072f4` u32 |
| 20 | `0x140307462` | `0x14030748e` | `0x140307456` | u16 | top (`TEST EBX,EBX / JZ 0x140307490`) | **yes** | `0x140307465` u32, `0x140307471` u32 |
| 21 | `0x1403074f0` | `0x140307531` | `0x1403074e1` | u16 | top (`TEST EAX,EAX / JZ 0x140307533`) | **yes** | `0x1403074f3` **u32(thunk)**, `0x140307501` str |
| 22 | `0x140307550` | `0x140307566` | `0x14030753b` | u16 | top (`TEST EAX,EAX / JZ 0x140307568`) | **yes** | `0x140307553` **u32(thunk)** |
| 23 | `0x1403075c5` | `0x1403075fc` | `0x1403075b3` | u16 | top (`TEST EAX,EAX / JZ 0x140307600`) | **yes** | `0x1403075c8` **u32(thunk)**, `0x1403075df` raw(8) |
| 24 | `0x140307616` | `0x14030762c` | `0x140307608` | u16 | top (`TEST EAX,EAX / JZ 0x14030762e`) | **yes** | `0x140307619` **u32(thunk)** |
| 25 | `0x140307690` | `0x1403077e9` | `0x140307667` | u16 | top (`TEST ESI,ESI / JZ 0x1403077ff`) | **yes** | `0x14030773f`, `0x14030775c`, `0x140307767`, `0x140307772`, `0x14030777d` — all u32 |
| 26 | `0x140307bc8` | `0x140307bda` | **none — fixed 5** | — | n/a | **NO — see §4** | `0x140307bcb` u32 |
| 27 | `0x140307be8` | `0x140307bfa` | **none — fixed 10** | — | n/a | **NO — see §4** | `0x140307beb` u32 |
| 28 | `0x140308a70` | `0x140308ab0` | `0x140308a5e` | u16 | top (`TEST ESI,ESI / JZ 0x140308ab6`) | **yes** | `0x140308a73` **u32(thunk)**, `0x140308a82` str |
| 29 | `0x140308b03` | `0x140308b38` | `0x140308af5` | u16 | top (`TEST EAX,EAX / JZ 0x140308b3a`) | **yes** | `0x140308b06` u32, `0x140308b12` u16 |
| 30 | `0x140308b90` | `0x140308bcd` | `0x140308b84` | u32 | top (`TEST EAX,EAX / JLE 0x140308bcf`) | **yes** | `0x140308b93` u32, `0x140308ba1` str |
| 31 | `0x140308c10` | `0x140308c42` | `0x140308c04` | u32 | top (`TEST EAX,EAX / JLE 0x140308c44`) | **yes** | `0x140308c13` u32, `0x140308c1f` u32 |
| 32 | `0x140308ea1` | `0x140308ede` | `0x140308e95` | u16 | top (`TEST ESI,ESI / JZ 0x140308ee0`) | **yes** | `0x140308ea4` u32, `0x140308eb2` str |
| 33 | `0x140308f97` | `0x140308fc7` | `0x140308f85` | u16 | top (`TEST EAX,EAX / JZ 0x140309094`) | **yes** | `0x140308f9a` **u64**, `0x140308fa6` u16 |
| 34 | `0x140308fe4` | `0x14030908e` | `0x140308fd1` | u16 | top (`TEST EAX,EAX / JZ 0x140309099`) | **yes** | `0x140308fe7` **u64**, `0x140308ff6` u16 |
| 35 | `0x1403090e0` | `0x140309112` | `0x1403090d4` | u16 | top (`TEST EDI,EDI / JZ 0x140309114`) | **yes** | `0x1403090e3` u32, `0x1403090ef` u32 |
| 36 | `0x140309243` | `0x1403092e8` | `0x140309233` | u32 | top (`TEST EAX,EAX / JLE 0x1403092f3`) | **yes** | `0x140309246` **u64**, `0x140309252` u32 (count for #37) |
| 37 | `0x140309271` | `0x1403092ab` | `0x140309252` | u32 | top (`TEST EAX,EAX / JLE 0x1403092af`) | **yes** | `0x140309274` u32 |

**Nesting:** only two nests — #9 contains #10, and #36 contains #37. Everything else is flat.

### 3a. Top vs bottom testing

**[D] Every counted loop in this function is MSVC-rotated: a guard before the head plus a
bottom back-edge.** The guard is what matters, and in all 31 counted loops the guard tests the
count and jumps *past* the loop body (verified: the guard's branch target is not in the
dominator-derived loop body). **A zero count therefore skips the body cleanly — no read fires.**

**There is no bottom-tested (do-while) counted loop in this function.** I looked for one
specifically; there is none.

### 3b. The two sentinel loops — read this carefully

**[L]** Loops #7 (`0x140306210`) and #8 (`0x1403066a0`) are **not counted loops.** Their shape is:

```
                                  ; -- loop 8 --
14030668c  CALL 0x1406e8b80       ; u16
140306698  TEST EAX,EAX
14030669a  JZ  0x140306804        ; 0 -> skip the whole list
1403066a0  <body>                 ; head
1403067ee  CALL 0x1406e8b80       ; u16 -- the NEXT element's key
1403067fa  TEST EAX,EAX
1403067fc  JNZ 0x1403066a0        ; non-zero -> another element
```

The wire format is `u16 key; while (key != 0) { <body>; u16 key; }` — a **zero-terminated list**,
not a count-prefixed one. To skip it you send a single `u16 = 0`, and that terminator is the
*first* read, so it is top-tested and skips cleanly. But note: **each iteration costs one extra
trailing u16.** Treating these as counts would desync the stream.

---

## 4. Loops whose trip count does NOT come from the packet

**Four loops. [L] all four initialisers are immediate constants.**

| loop | trip count | proof | reads it forces |
|---|---|---|---|
| #6 `0x140305e01` | **exactly 6** | `MOV R15D,0x6` @ `0x140305def`; `SUB R15,0x1 / JNZ` @ `0x14030608e` | `0x140305e48` u16, itself gated by flag **#13** |
| #9 `0x140306972` | **exactly 3** | `MOV R15D,0x2` @ `0x140306830` → `[RSP+0x50]` → `MOV R15D,[RSP+0x50]` @ `0x140306963`; `INC R15D / CMP R15D,0x4 / JLE` @ `0x140306b47` (2,3,4) | `0x140306a0d` u32, gated by a switch-selected flag |
| #26 `0x140307bc8` | **exactly 5** | `MOV R12D,0x5` @ `0x1403077ef`; `SUB R12,0x1 / JNZ` | 5 × u32 at `0x140307bcb` |
| #27 `0x140307be8` | **exactly 10** | `LEA EDI,[R12 + 0xa]` @ `0x140307be3` with R12 == 0 after #26 drains it; `SUB RDI,0x1 / JNZ` | 10 × u32 at `0x140307beb` |

**These cannot be collapsed with a count.** But **[D] all four are neutralisable through the
presence array instead**:

* #6's only read sits behind flag **#13**.
* #9's only read sits behind a switch-selected flag (index 0-6).
* #26 and #27 are both inside flag **#23**'s region — clear #23 and the whole 15 × u32 block
  (60 bytes) disappears.

**[L]** #26 + #27 together are a fixed **5 u32 then 10 u32** block. If flag #23 is ever set, you
must emit exactly 15 u32; there is no count to shorten it.

---

## 5. The presence array — which flag guards which block

**[L]** Field 1 (the 100-byte raw read at `0x140304b95`, into arg2) is queried 43 times by
`0x1402fa9a0(arg2, scratch, key)`. **[D] I verified by reaching-definition analysis that all 43
call sites pass arg2 as the first argument** — the gate input is the leading 100-byte field and
nothing else. (8 sites report an extra candidate definition; that is a false positive from
`__chkstk` at `0x140304b3e`, which I model conservatively as clobbering RDX. It does not.)

**[L]** The key is a static pointer into an array based at `0x143abeb10` with stride **0x70**.
**All 38 static keys land on an exact multiple — zero remainder** — which is what makes the index
assignment below trustworthy. 5 further sites use a switch-selected key (indices 0-6).

| flag | gate call | guarded region | reads gated | loops gated |
|---|---|---|---|---|
| #7 | `0x140304e49` | `[0x140304e64, 0x140304fa6)` | 7 | — |
| #8 | `0x140304fb7` | `[0x140304fd1, 0x140304ffa)` | 2 | — |
| #9 | `0x14030500b` | `[0x140305023, 0x140305033)` | 0 | — |
| #10 | `0x140305044` | `[0x140305061, 0x140305071)` | 0 | — |
| #11 | `0x140305082` | `[0x1403050a1, 0x1403050ac)` | 0 | — |
| **#5 OR #12** | `0x1403050bd` / `0x1403050e4` | `[0x140305104, 0x1403051b6)` | 5 | #5 |
| #1 | `0x14030525a` | `[0x140305275, 0x1403054df)` | 0 | — |
| dyn | `0x14030558a` | `[0x1403055a5, 0x140305744)` | 0 | — |
| #6 | `0x140305776` | `[0x140305794, 0x140305d7d)` | 0 | — |
| #13 | `0x140305e29` | `[0x140305e45, 0x140305e54)` | 1 | inside #6 (fixed 6) |
| dyn | `0x140305eea` | `[0x140305f05, 0x140306087)` | 0 | — |
| #14 | `0x140306147` | `[0x140306171, 0x14030618f)` | 2 | — |
| #6 | `0x1403061a0` | `[0x1403061c9, 0x14030661c)` | 3 | #7 |
| #1 | `0x140306632` | `[0x140306654, 0x140306830)` | 3 | #8 |
| dyn | `0x1403068fd` | `[0x140306915, 0x140306958)` | 0 | — |
| dyn | `0x1403069ef` | `[0x140306a0a, 0x140306b44)` | 2 | #9, #10 |
| #15 | `0x140306b62` | `[0x140306b89, 0x140306c3c)` | 3 | #11 |
| #16 | `0x140306c4d` | `[0x140306c68, 0x140306d17)` | 3 | #12 |
| **#17** | `0x140306d28` | `[0x140306d44, 0x1403073c3)` | **21** | #13-#19 |
| #18 | `0x1403073d4` | `[0x1403073f4, 0x140307492)` | 3 | #20 |
| #19 | `0x1403074a3` | `[0x1403074c4, 0x14030756a)` | 6 | #21, #22 |
| #20 | `0x14030757b` | `[0x140307596, 0x140307633)` | 6 | #23, #24 |
| #21 | `0x140307644` | `[0x140307664, 0x140307801)` | 6 | #25 |
| #22 | `0x140307812` | `[0x140307834, 0x140307b97)` | 6 | — |
| **#23** | `0x140307ba8` | `[0x140307bc1, 0x140307bfc)` | 2 | **#26, #27 (the fixed 15 u32)** |
| #24 | `0x140308a36` | `[0x140308a56, 0x140308ab8)` | 3 | #28 |
| #25 | `0x140308ace` | `[0x140308ae6, 0x140308b3c)` | 3 | #29 |
| #26 | `0x140308b63` | `[0x140308b81, 0x140308bcf)` | 3 | #30 |
| #27 | `0x140308be0` | `[0x140308c01, 0x140308c44)` | 3 | #31 |
| #28 | `0x140308cf3` | `[0x140308d11, 0x140308d20)` | 0 | — |
| #29 | `0x140308d31` | `[0x140308d51, 0x140308d68)` | 0 | — |
| #30 | `0x140308d79` | `[0x140308d92, 0x140308da9)` | 0 | — |
| #31 | `0x140308dba` | `[0x140308dd3, 0x140308de2)` | 0 | — |
| #32 | `0x140308df3` | `[0x140308e11, 0x140308e2f)` | 2 | — |
| #33 | `0x140308e40` | `[0x140308e59, 0x140308e68)` | 0 | — |
| #34 | `0x140308e79` | `[0x140308e92, 0x140308ee0)` | 3 | #32 |
| #35 | `0x140308ef1` | `[0x140308f14, 0x140309099)` | 7 | #33, #34 |
| #36 | `0x1403090b5` | `[0x1403090d1, 0x140309114)` | 3 | #35 |
| #37 | `0x140309125` | `[0x140309141, 0x140309156)` | 0 | — |
| #38 | `0x140309167` | `[0x140309181, 0x140309196)` | 0 | — |
| #39 | `0x1403091a7` | `[0x1403091c1, 0x1403091d6)` | 0 | — |
| #40 | `0x1403091e7` | `[0x140309204, 0x1403092f3)` | 4 | #36, #37 |

**[L]** `0x1403050bd` (#5) and `0x1403050e4` (#12) both branch to the same target `0x140305104`,
with `JMP 0x1403051b6` only on the doubly-false path — so that block is `flag#5 || flag#12`.

**[L]** Flag **#17** is by far the biggest lever: one flag gates 21 reads and 7 loops.
Flag **#22** gates 6 reads and 6 loops whose bodies call sub-decoders. Flag **#23** gates the only
uncollapsible fixed block.

### What I could NOT determine about the gates

* ~~**[I, unproven]** The exact mapping from a key index to a **byte offset inside the
  100-byte array**.~~ **SETTLED 2026-08-19 - and the guess in this bullet was wrong.** See
  `research/charrecord-presence-map.md` for the full table. `0x1402fa9a0` computes
  `out[i] = arg2[i] & key[i]` over 100 bytes, and each key is built at startup as all-zero
  with **exactly one byte set to 1**, so a gate fires iff that one presence byte is set.
  The mapping is a **permutation, not the identity**: entry 7 -> byte **0**, entry 8 ->
  byte **62**, entry 1 -> byte **44**. The reasoning recorded here - "plausible given the
  1:1 stride and the 0x64 bound matching the array length" - was sound and produced the
  wrong answer; the stride and the bound are real and say nothing about the offset. The
  labels `#k` in the table above remain correct as **entry** numbers.
* **[I]** The 5 dynamic-key gates select among indices 0-6 via the jump tables. Case *k* maps to
  index *6-k* and the `JA` default is index #0 **[L, from the case-body ordering]**, but the table
  contents themselves are outside the listing.

---

## 6. The straight-line spine

**[D]** 61 of the 126 reads lie **outside every natural loop** — no count value can change whether
they fire. Gate column is the presence flag that must be set for the read to happen.

| # | addr | kind | bytes | gate | | # | addr | kind | bytes | gate |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `0x140304b95` | raw | **100** | none | | 32 | `0x140307105` | u16 | 2 | #17 |
| 2 | `0x140304ba7` | u8 | 1 | none | | 33 | `0x1403071e4` | u16 | 2 | #17 |
| 3 | `0x140304bba` | u32 | 4 | none | | 34 | `0x1403072e1` | u16 | 2 | #17 |
| 4 | `0x140304c61` | u8 | 1 | none | | 35 | `0x140307456` | u16 | 2 | #18 |
| 5 | `0x140304c92` | u32 | 4 | none | | 36 | `0x1403074c7` | u8 | 1 | #19 |
| 6 | `0x140304cf2` | u8 | 1 | none | | 37 | `0x1403074e1` | u16 | 2 | #19 |
| 7 | `0x140304d02` | u32 | 4 | `[cf2]` | | 38 | `0x14030753b` | u16 | 2 | #19 |
| 8 | `0x140304e11` | u32 | 4 | `[cf2]` | | 39 | `0x140307599` | u8 | 1 | #20 |
| 9 | `0x140304e79` | u8 | 1 | #7 | | 40 | `0x1403075b3` | u16 | 2 | #20 |
| 10 | `0x140304e8c` | u8 | 1 | #7 | | 41 | `0x140307608` | u16 | 2 | #20 |
| 11 | `0x140304e9c` | str | 2+n | #7 | | 42 | `0x140307667` | u16 | 2 | #21 |
| 12 | `0x140304ee9` | u8 | 1 | #7 | | 43 | `0x140307844` | u16 | 2 | #22 |
| 13 | `0x140304efc` | str | 2+n | #7 | | 44 | `0x140307868` | u16 | 2 | #22 |
| 14 | `0x140304f4c` | u8 | 1 | #7 | | 45 | `0x1403078bb` | u16 | 2 | #22 |
| 15 | `0x140304f5c` | str | 2+n | #7 | | 46 | `0x14030794c` | u16 | 2 | #22 |
| 16 | `0x140304fd4` | u16 | 2 | #8 | | 47 | `0x140307a31` | u16 | 2 | #22 |
| 17 | `0x140304ff5` | raw | **8** | #8 | | 48 | `0x140307b0e` | u16 | 2 | #22 |
| 18 | `0x140305166` | u32 | 4 | #5\|#12 | | 49 | `0x140308a5e` | u16 | 2 | #24 |
| 19 | `0x140306174` | u8 | 1 | #14 | | 50 | `0x140308af5` | u16 | 2 | #25 |
| 20 | `0x140306183` | u8 | 1 | #14 | | 51 | `0x140308b3f` | u8 | 1 | **none** |
| 21 | `0x1403061cc` | u8 | 1 | #6 | | 52 | `0x140308b84` | u32 | 4 | #26 |
| 22 | `0x1403061fc` | u16 | 2 | #6 | | 53 | `0x140308c04` | u32 | 4 | #27 |
| 23 | `0x14030665c` | u8 | 1 | #1 | | 54 | `0x140308e14` | u32 | 4 | #32 |
| 24 | `0x14030668c` | u16 | 2 | #1 | | 55 | `0x140308e23` | u32 | 4 | #32 |
| 25 | `0x140306bf6` | u32 | 4 | #15 | | 56 | `0x140308e95` | u16 | 2 | #34 |
| 26 | `0x140306cc6` | u32 | 4 | #16 | | 57 | `0x140308f17` | u8 | 1 | #35 |
| 27 | `0x140306d47` | u8 | 1 | #17 | | 58 | `0x140308f85` | u16 | 2 | #35 |
| 28 | `0x140306d57` | u16 | 2 | #17 | | 59 | `0x140308fd1` | u16 | 2 | #35 |
| 29 | `0x140306e29` | u16 | 2 | #17 | | 60 | `0x1403090d4` | u16 | 2 | #36 |
| 30 | `0x140306f21` | u16 | 2 | #17 | | 61 | `0x140309233` | u32 | 4 | #40 |
| 31 | `0x14030700f` | u16 | 2 | #17 | | | | | | |

Plus **[D]** four reads that are inside *fixed* loops, so they are also count-immune but repeat:

| addr | kind | multiplicity | gate |
|---|---|---|---|
| `0x140305e48` | u16 | ×6 | #13 |
| `0x140306a0d` | u32 | ×3 | dyn (switch-selected) |
| `0x140307bcb` | u32 | ×5 | #23 |
| `0x140307beb` | u32 | ×10 | #23 |

### 6a. The mode fork at `0x140306d47`

**[L]** Inside flag #17, the u8 at `0x140306d47` is a **format selector**, not data:

```
140306d47  CALL 0x1406e8ae0    ; u8
140306d4f  TEST AL,AL
140306d51  JZ 0x140306e29      ; ==0 -> SIX separate lists (loops #14..#19)
140306d57  CALL 0x1406e8b80    ; !=0 -> ONE combined list (loop #13)
```

Non-zero selects loop #13 (u32, u32, raw8, u32 per element). Zero selects the six lists at
`0x140306e29`, `0x140306f21`, `0x14030700f`, `0x140307105`, `0x1403071e4`, `0x1403072e1`, each a
u16 count + its own body. **Both arms collapse fully with zero counts.**

**[L]** A second fork of the same kind: `0x140307834 CMP dword ptr [RBP+0x3118],0 / JZ 0x1403078ea`
inside flag #22 — three u16-counted lists either way (`0x140307844/868/8bb` vs `0x14030794c/a31/b0e`).
`[RBP+0x3118]` is a client-side local, **[I]** not directly a packet field.
A third: `0x140308f1e JZ 0x140308fce` inside flag #35.

### 6b. Sub-decoders that consume the stream

**[D] 26 non-primitive calls receive the packet pointer (arg3)** and read bytes not counted among
the 126. All are inside loop bodies or gated regions, so zeros still collapse them — but if you
ever set those counts non-zero, the wire cost is larger than the 126 reads suggest.

In loops: `0x1403095e0` (loops #7,#8), `0x1402dddb0` (#10), `0x1402d1bb0`/`0x1402d1bf0`/`0x1402d1c30`
(the six flag-#22 loops). Not in loops but gated: `0x1402d3090` ×3 (flags #28-#30), `0x1403096a0`
(#31), `0x1402ddf80` (#33), `0x1402ea370` (#35), `0x1402caae0` ×3 (flags #37-#39).

---

## 7. Minimum record

**[D]** With **every count 0, every presence flag clear, and the `0x140304cf2` boolean 0**, exactly
**seven** reads execute:

| addr | kind | bytes |
|---|---|---|
| `0x140304b95` | raw | 100 |
| `0x140304ba7` | u8 | 1 |
| `0x140304bba` | u32 | 4 |
| `0x140304c61` | u8 (count → 0) | 1 |
| `0x140304c92` | u32 (count → 0) | 4 |
| `0x140304cf2` | u8 (bool → 0) | 1 |
| `0x140308b3f` | u8 | 1 |

**112 bytes.** **[D]** Verified by walking the gate skip-target chain: each gate's `JMP` lands
before the next gate, so an all-clear presence array walks the whole chain and reaches the end,
and `0x140308b3f` is reached on both the skip and non-skip paths of flag #25.

---

## 8. Unresolved

1. **[I]** `0x1402fa9a0`'s body — key index to array byte offset. See §5.
2. **[L]** The four jump tables (`0x1403093a4`, `+0x18`, `+0x30`, `+0x48`) are past the listing end.
   Case bodies recovered; table contents not.
3. **[L]** Function epilogue past `0x14030937e`, and three 6-byte stubs at `0x140309392`/`98`/`9e`.
   **[I]** Cannot hold reads.
4. **[I]** The `0x140307834` fork reads `[RBP+0x3118]`, a client-side local. I did not trace its
   full provenance; it is written from client state around `0x140304e64`, not read from the packet
   at that point.
5. **[I]** Whether flag #13's gate is loop-invariant across loop #6's 6 iterations. Its inputs
   (arg2, static key `0x143abf0c0`, scratch `[RBP+0x25b0]`) are all loop-invariant **[L]**, so the
   read at `0x140305e48` should fire either 0 or 6 times — **unless `0x1402fa9a0` mutates state.**
   Not provable here.
