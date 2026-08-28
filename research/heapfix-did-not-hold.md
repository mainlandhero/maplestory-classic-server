# `-HeapFix` armed, and the client died anyway — at a free the patch was never near

```
dumps\maplecw-crash-217724-c0000374-1.dmp   1 344 MB   270.7 s   failure type 9   1 damaged slot
client-patched\maplecw-hook.log             mode="mode=2,create=on,heapfix=on"
                                            HEAPFIX: 0x14019b504 is now [8b, 07, 90]
```

Tags: **[L]** read out of a file, **[D]** derived from something read, **[I]** inferred.

Nothing here edits `research/heap-wild-write.md`, `research/heap-third-dump.md`,
`research/heap-corruption-2026-08-27.md` or `research/heap-corruption.md`. It answers the five
questions the brief asked and adds one the brief did not.

---

## The short version

* **The patch took, and was still in place at the moment of death.** Read out of the dump:
  `0x14019b504` holds `8b 07 90`, not `48 8b 07`. `[L]`
* **The client's pool has more than one `free`, and they all load the header as a qword.** The
  fault is at `0x14019bbf3`, which is `+0x89` of a **different function** — and that function
  is a second, byte-for-byte equivalent pooled free that serves the **identical pool context
  `0x143AD68A0`**, takes the **identical branch**, and dies at the **identical instruction
  shape**: the return address of `HeapFree(GetProcessHeap(), 0, ptr − 8)`. Its own header load
  is at `0x14019bb63`, reads `48 8b 42 f8` in the dump, and `-HeapFix` does not touch it. `[L]`
* **The evidence supports reading (b), not (a).** The patch is irrelevant to *this* death, and
  the previous single-site story was too simple. Two independent reasons, either sufficient:
  1. The route this block took is a compiled `operator delete(void*, size_t)` call
     (`0x1401be17b`), which reaches `0x14019bb50` and can never reach `0x14019b4e0`. It does
     not depend on the patch. `[D]`
  2. **A client died at `0x14019bbf3` on 2026-08-20 with `-HeapFix` off** — indeed two days
     before `heapfix.rs` existed. The fixture was on disk the whole time. `[L]`
* **The damage is the same shape for the tenth time.** `0x0000000100000020`, bucket 1, one
  slot; 7 799 chunks passed the per-chunk size identity, 0 failed. `[L]`
* **The damaged slot was *not* on the free list this time, and no free list holds a damaged
  slot at all** — 4 of 4 buckets pass `carved − freelist == served`. `[L]` But see §4: the
  patch **destroys the argument** that made the previous free-list find meaningful.
* **Recommendation: switch `-HeapFix` off**, unless the same edit widens it to `0x14019bb63`.
  As it stands it patches one of three entry points, and the one it patches is the *minority*
  delete route. §5.

---

## 0. The instruments, and the controls that could have failed

Every tool run with the repo as the working directory. No Ghidra.

| instrument | control | result |
|---|---|---|
| `tools/listing.py` | its documented positive control on `0x140304100` | reads at `140304138 raw`, `140304144 u8`, `140304183 u8`, then the `u16` run — matches `tools/reads.py 0x140304100 2` exactly `[L]` |
| `tools/dumpwalk.py` | its six self-checks | **6/6 PASS** `[L]` |
| the same, `.pdata` unwind vs the allocator's own captured stack | a low overlap means the unwinder is wrong, because the capture cannot be | **28 of 28 frames** `[L]` |
| `tools/poolchain.py` | per-chunk size identity `n*(slot+8)+8` | **7 799 chunks passed, 0 failed** `[L]` |
| a free-list walk (inline, no new file) | `carved − freelist == served`, the allocator's own two counters | **4 of 4 buckets PASS**, head in the walked set in each `[L]` |
| `tools/callers.py` | its documented control `0x1402fa9a0` → 96 sites in 15 functions | passed `[L]` |
| a byte scan for the `test rax,rax / jns +3 / not rax` ladder | must find `0x14019b507`, the site whose listing is already known | found it as hit #1 of 56 `[L]` |
| the IAT slots, read **from the dump** rather than from the import table | must resolve to an exact export | `0x143ad5530 = kernel32!HeapFree`, `0x143ad5538 = kernel32!GetProcessHeap` `[L]` |

`tools/dumpwalk.py`'s type guard fired and was **not** worked around. Failure type is **9**, so
it refused to decode `Address` as a `_HEAP_ENTRY` and said why. That refusal is correct and it
is also *information*: type 9 reports the caller's pointer, and `Address = 0x40e5ff58` is
exactly `R13`, exactly the pointer `RtlFreeHeap` was handed, and exactly the header address of
the damaged slot `poolchain.py` independently enumerated. Three readings of one number. `[L]`

---

## 1. Q1 — what `0x14019bb6a .. 0x14019bc5c` is, and what `+0x89` does

### It is a second pooled `free`, and `+0x89` is the `HeapFree` return address

`tools/pdata_lookup.py`: `0x14019bbf3 -> 0x14019bb6a .. 0x14019bc5c, +0x89`. `[L]` But the
`.pdata` entry starts *inside* the function. The real entry is 26 bytes earlier, and it is the
part that matters:

```asm
14019bb50  48 85 d2        test rdx, rdx
14019bb53  0f 84 03 01..   je   14019bc5c          ; free(NULL) -> return
14019bb59  48 89 6c 24 18  mov  [rsp+0x18], rbp
14019bb5e  57              push rdi
14019bb5f  48 83 ec 20     sub  rsp, 0x20
14019bb63  48 8b 42 f8     mov  rax, qword ptr [rdx-8]   ; <<< THE HEADER LOAD. Unpatched.
14019bb67  48 8b fa        mov  rdi, rdx                 ; rdi = body
14019bb6a  48 89 74 24 38  mov  [rsp+0x38], rsi          ; <-- where the .pdata entry begins
14019bb6f  48 8b e9        mov  rbp, rcx                 ; rbp = POOL CONTEXT, a parameter
14019bb72  48 85 c0        test rax, rax
14019bb75  79 03           jns  14019bb7a
14019bb77  48 f7 d0        not  rax
14019bb7a  48 89 5c 24 30  mov  [rsp+0x30], rbx
14019bb7f  48 83 f8 20     cmp  rax, 0x20
14019bb83  77 0b           ja   14019bb90
14019bb85  33 c9           xor  ecx, ecx
14019bb87  48 83 f8 10     cmp  rax, 0x10
14019bb8b  0f 97 c1        seta cl
14019bb8e  eb 47           jmp  14019bbd7
14019bb90  48 83 f8 40     cmp  rax, 0x40
14019bb94  77 2e           ja   14019bbc4
14019bb96  b9 02 00 00 00  mov  ecx, 2
14019bb9b  ...             ; rbx = ctx + i*16; take [rbx+0x28] spinlock
14019bbc4  48 83 f8 80     cmp  rax, 0x80
14019bbca  b9 ff ff ff ff  mov  ecx, 0xffffffff
14019bbcf  ba 03 00 00 00  mov  edx, 3
14019bbd4  0f 46 ca        cmovbe ecx, edx
14019bbd7  85 c9           test ecx, ecx
14019bbd9  79 c0           jns  14019bb9b            ; a bucket -> the pool
14019bbdb  48 8b 1d ..     mov  rbx, [rip+0x393994e] ; -> 0x143ad5530 = kernel32!HeapFree
14019bbe2  ff 15 ..        call qword ptr [rip+0x3939950] ; -> 0x143ad5538 = GetProcessHeap
14019bbe8  4c 8d 47 f8     lea  r8, [rdi-8]
14019bbec  33 d2           xor  edx, edx
14019bbee  48 8b c8        mov  rcx, rax
14019bbf1  ff d3           call rbx                  ; HeapFree(heap, 0, ptr-8)
14019bbf3  eb 53           jmp  14019bc48            ; <<< +0x89. THIS is the fault frame.
14019bc1f  ...             ; the pool arm: push onto [rbp + rsi*8 + 0x68], dec [rbp + rsi*4 + 0x14]
```

`[L]` for every byte. The bytes at `0x14019bbf1` were read **out of the dump** and are
`ff d3 eb 53`, so `+0x89` really is the instruction after `call rbx`. `[L]`

**It is a free, not an allocate and not a coalesce.** Same four-bucket ladder
(`0x10/0x20/0x40/0x80`), same per-bucket spinlock at `ctx + i*16 + 0x28`, same free-list push
at `ctx + i*8 + 0x68`, same live-counter decrement at `ctx + i*4 + 0x14`, same `HeapFree` arm
through the same two IAT slots as `0x14019b4e0`. `[L]`

The only structural difference from the patched free is where the pool context comes from:
`0x14019b4e0` hardcodes it (`lea rbp,[rip+0x393b366]` → `0x143AD68A0`), and `0x14019bb50`
takes it in `rcx`. `[L]`

### It reads the header exactly the way the patched one did

`mov rax, qword ptr [rdx-8]` — 64 bits of a field whose only legal values are
`0x10/0x20/0x40/0x80`. **Yes, the same dword-vs-qword treatment would have saved it**, and the
arithmetic is the same table `heap-corruption-2026-08-27.md` §3 wrote for the other site:

| | `mov rax,[rdx-8]` (today) | `mov eax,[rdx-8]` (hypothetical) |
|---|---|---|
| rax after the load | `0x0000000100000020` | `0x0000000000000020` |
| `test`/`jns` | positive, no `not` | positive, no `not` |
| `cmp rax,0x20` | above → `14019bb90` | not above → `14019bb85` |
| `cmp rax,0x40` | above → `14019bbc4` | — |
| `cmp rax,0x10` | — | above → `cl = 1` |
| ecx | `-1` | **1** |
| `test ecx,ecx / jns` | not taken | taken → `14019bb9b` |
| outcome | `HeapFree(ptr−8)` → **`+0x89`** | pushed onto the **`0x20` free list** |

`[D]`, resting on the header value `[L]`, the fault offset `[L]` and the listing `[L]`.

The edit would be the same width, four bytes for four: `48 8b 42 f8` → `8b 42 f8 90`
(`mov eax,[rdx-8]; nop`). `[D]`

### There are at least three of these functions, and 56 sites with the ladder

Byte scan for `48 85 c0 79 03 48 f7 d0` (`test rax,rax / jns +3 / not rax`) across the image,
positive control passed (it found `0x14019b507`, hit #1):

**56 sites.** `[L]` 53 of them show a `mov rax, qword ptr [reg-8]` immediately before; the other
three landed mid-instruction for the backward decode and were not chased. `[L]`

Three are standalone functions:

| entry | shape | ladder | header load | patched? |
|---|---|---|---|---|
| `0x14019b4e0` | `free(p)`, context hardcoded `0x143AD68A0` | `0x10/0x20/0x40/0x80` | `0x14019b504` `48 8b 07` | **yes** |
| `0x14019bb50` | `free(ctx, p)` | `0x10/0x20/0x40/0x80` | `0x14019bb63` `48 8b 42 f8` | no |
| `0x14019ba40` | `free(ctx, p)` | `0x28/0x38/0x58/0x98` | `0x14019ba53` `48 8b 42 f8` | no |

`[L]`. `0x14019ba40` and `0x14019bb50` have **byte-identical prologues** and differ only in the
size ladder. `[L]`

The remaining 53 are the same ladder inlined into ordinary functions. **52 of the 56 sit in a
function that also carries a `lea` to `0x143AD68A0`** `[L]` — which is an indication, not a
measurement: I did **not** trace the register that supplies the context to each inlined push,
so a function that loads `0x143AD68A0` for an *allocation* would be counted the same way. `[I]`
Stated so it can be quoted: **the number of inlined free sites that can actually reach the
damaged `0x20` class is not established.**

---

## 2. Q2 — did the patch take? Yes, and it was still there when the process died

Read out of `dumps\maplecw-crash-217724-c0000374-1.dmp`. `MapleStory.exe` is at
`0x140000000` in that process — no relocation, `dumpwalk.py`'s own self-check confirms it. `[L]`

```
00014019b504  8b 07 90 48 85 c0 79 03   <- the -HeapFix site: mov eax,[rdi]; nop.  PATCHED.
00014019bb63  48 8b 42 f8 48 8b fa 48   <- the fatal site:    mov rax,[rdx-8].     NOT PATCHED.
00014019ba53  48 8b 42 f8               <- the third free's load.                  NOT PATCHED.
00014019bbf1  ff d3 eb 53               <- call rbx / jmp, so +0x89 is HeapFree's return
```

`[L]`. That is one read and it does what the brief said it would: **"the patch was undone" is
eliminated.** The write took, it survived 270 seconds, and it was still `8b 07 90` at the
moment the process was killed.

The hook log agrees and is a second, independent witness written at patch time:

```
23:57:05.621  mode="mode=2,create=on,heapfix=on"
23:57:05.623  ***** HEAPFIX: 0x14019b504 is now [8b, 07, 90] (mov eax,[rdi]; nop) *****
```

`[L]`

---

## 3. Q3 — the damage is the same shape for the tenth time

`tools/poolchain.py`, passed/failed counts, not a sample:

```
bucket 0  slot 0x10:   939 chunks passed, 0 failed;  60096 slots, 0 damaged
bucket 1  slot 0x20:  2418 chunks passed, 0 failed;  77376 slots, 1 damaged
    0x40e5ff58  hdr 0x0000000100000020  chunk 0x40e5fc08  slot 21/32   <<< the one that was freed
bucket 2  slot 0x40:  2329 chunks passed, 0 failed;  37264 slots, 0 damaged
bucket 3  slot 0x80:  2113 chunks passed, 0 failed;  16904 slots, 0 damaged
```

`[L]` — 7 799 chunks, 0 failed the size identity; 191 640 slots enumerated.

The family:

| dump | age | slots | damaged | value | `-HeapFix` |
|---|---:|---:|---:|---|---|
| 1096760 `[L, quoted]` | 596 s | 226 200 | 2 | `0x0000000100000020` | off |
| 1160396 `[L, quoted]` | 644 s | 181 984 | 3 | `0x0000000100000020` | off |
| 1224132 `[L, quoted]` | 306 s | 183 456 | 1 | `0x0000000100000020` | off |
| 155108 `[L, quoted]` | 403 s | 197 776 | 1 | `0x0000000100000020` | off |
| 158352 `[L, quoted]` | 1046 s | 172 696 | 2 | `0x0000000100000020` | off |
| **217724** `[L]` | **270.7 s** | **191 640** | **1** | `0x0000000100000020` | **on** |
| | | **1 153 752** | **10** | **ten identical** | |

**0 damaged in 693 272 slots outside the `0x20` class.** `[L]` Not one other value has ever
appeared on a slot header in this pool, across six dumps and six processes.

The header is readable directly in the dump beside the object it belongs to, and the object
identifies itself:

```
0000000040e5ff58  20 00 00 00 01 00 00 00   <- the damaged header, 0x0000000100000020
0000000040e5ff60  00 00 00 00 00 00 00 00   <- body[0x00] = 0
0000000040e5ff68  00 00 00 00 00 00 00 00   <- body[0x08] = 0
0000000040e5ff70  00 00 00 00 ...           <- body[0x10] = 0  (the refcount, after the xadd)
```

`[L]` That is exactly the state `0x1401be120` leaves a block in before it deletes it: it zeroes
`[rbx]` at `0x1401be156`, zeroes `[rbx+8]` at `0x1401be16b`, and reaches the delete only when
the `lock xadd dword ptr [rbx+0x10], -1` returned 1. `[D]` Three fields, three matches.

**Type 9 this time.** The family's failure types are now 8, 8, 9, 9, 4, 9 — the union member's
meaning has changed under us three times, and `dumpwalk.py`'s guard has been right each time.

---

## 4. Q4 — the free list, and the thing `-HeapFix` quietly breaks

### Measured: no free list holds a damaged slot, and the fatal one was live

Walking each bucket's free list from `ctx + i*8 + 0x68`, with the allocator's own two counters
as the control `[L]`:

```
bucket 0  carved 60096 - freelist  6894 = 53202,  served 53202   [PASS]
bucket 1  carved 77376 - freelist 17004 = 60372,  served 60372   [PASS]
bucket 2  carved 37264 - freelist 16058 = 21206,  served 21206   [PASS]
bucket 3  carved 16904 - freelist   310 = 16594,  served 16594   [PASS]

damaged hdr 0x40e5ff58 -> body 0x40e5ff60 : on free list = False
free-list members with a non-zero high dword in their header: 0
```

The identity closes in **4 of 4 buckets** and the head is in the walked set in each, so the walk
is not short. `[L]`

So the answer to the question as asked is **no**: the damaged slot was live and being freed, and
no free list anywhere in the pool holds a damaged header. `[L]`

### The mechanism is still worth naming, and it has a second edge

`-HeapFix` is *designed* to push a damaged slot back onto the `0x20` free list. Two consequences,
and the second is worse than the first:

* **A damaged slot returned to the free list will be handed out again.** The next free of it may
  take the `0x14019bb50` route — which is unpatched — and die there. So the patch does not remove
  a death, it **relocates and defers** it, and it hands the block to a different object type in
  between, which makes the consumer at the top of the fatal stack *less* informative than it was.
  `[D]` No measurement here shows this happened; only one damaged slot existed in this process and
  it was consumed by the unpatched route.
* **It destroys the only constraint anyone has on *when* the `1` is written.**
  `heap-corruption-2026-08-27.md` §4's argument is: *a slot whose header is damaged can never be
  pushed onto the free list, because the free reads the header first and diverts to `HeapFree`;
  therefore a damaged slot found on the free list was damaged while it belonged to nobody.* That
  argument is **valid only while `0x14019b4e0` is unpatched.** With `-HeapFix` on, `0x14019b4e0`
  will happily push a damaged slot. `[D]` Every dump taken with the flag on is therefore
  **unusable** for the one inference that killed "the object underruns its own buffer".

That is a real, recurring cost, paid on every run the flag is on, in exchange for a benefit that
has never been demonstrated.

### What could not be discriminated

If the patched free *had* pushed a damaged slot and that slot had since been re-allocated, it
would now be live and off-list — indistinguishable from "the patched free never saw a damaged
slot". `[I]` The measurement above is *"no damaged slot is on any free list at the moment of the
dump"*, not *"the patch never pushed one"*.

---

## 5. Which reading the evidence supports — (b), and it is not close

The brief's two readings:

> **(a)** the patch worked at its site and the corruption killed the process somewhere else.
> **(b)** the patch is irrelevant and this was always a second, independent failure mode.

Neither is quite right as worded, so here is the shape the measurements actually make, and then
which of the two it is nearer.

**It is not an independent failure mode.** Same pool context `0x143AD68A0`, same size class,
same header value `0x0000000100000020`, same four-bucket ladder, same `> 0x80` arm, same
`HeapFree` through the same two IAT slots, same "fault is the return address of that call".
`[L]` The chain in `heap-wild-write.md` is **confirmed**, not falsified. What is falsified is
the belief that `0x14019b4e0` is *the* free for this pool.

**But the patch is irrelevant to this death**, and there are two independent reasons, either of
which is sufficient.

### Reason 1: this block could never have reached the patched free

The unwind (28 of 28 frames corroborated by the allocator's own capture, `[L]`):

```
#2  ntdll!RtlFreeHeap+0x51
#3  0x14019bbf3   fn 0x14019bb6a+0x89     the unpatched pooled free
#4  0x140205833   fn 0x140205820+0x13     sub rsp,0x28 / mov rdx,rcx / lea rcx,[->0x143AD68A0] / call 0x14019bb50
#5  0x1401be180   fn 0x1401be120+0x60     the refcount release: mov edx,0x18 / mov rcx,rbx / call 0x142ef3bb8
```

* `0x142ef3bb8` is a five-byte thunk: `jmp 0x140205820`. `[L]`
* `0x140205820` takes `(void* p in rcx, size_t n in rdx)`, **discards the size** (`mov rdx,rcx`
  overwrites it), loads `rcx` with `lea rcx,[rip+0x38d1072]` → **`0x143AD68A0`**, and calls
  `0x14019bb50`. `[L]` That is `operator delete(void*, size_t)` for the global pool.
* `0x1401be120` reaches it through an ordinary compiled `delete` expression at `0x1401be17b`,
  `operator delete(rbx, 0x18)`. `[L]`

An object's free route is a property of the code that deletes it, fixed at compile time. It does
not depend on which byte of the image `-HeapFix` rewrote. **This block would have gone to
`0x14019bb50` with the flag on or off, and `0x14019bb50` would have sent it to `HeapFree`
either way.** `[D]`

And the block is 0x18 bytes, so it lives in the **0x20** bucket — the one class that has ever
been damaged. `[D]`

### Reason 2: site 2 had already killed the client, unpatched, a week earlier

Enumerating every `CLIENT FAULT ... c0000374` line in `previous-runs/`, `research/fixtures/` and
`client-patched/`, with the session marker from each file's own header — deduplicated by fault
timestamp:

| fault time | site | marker | dump |
|---|---|---|---|
| 02:11:58.203 | `0x14019b58e` | `mode=2,create=on` | — (equip into empty hat slot) |
| 11:47:10.850 | `0x14019b58e` | `mode=2,create=on` | — (jr sentinel turn-in) |
| 20:20:46.782 | `0x14019b58e` | `mode=2,create=on` | 1096760 |
| 22:24:40.345 | `0x14019b58e` | `mode=2,create=on` | — (map 40, 30 mobs) |
| **22:46:38.086** | **`0x14019bbf3`** | **`mode=2,create=on`** | — (five field entries) |
| 23:01:02.331 | `0x14019b58e` | `mode=2,create=on` | 1160396 |
| 08:26:45.450 | `0x14019b58e` | `mode=2,create=on` | 1224132 |
| 21:09:54.954 | `0x14019b58e` | `mode=2,create=on` | 155108 |
| 22:47:23.060 | `0x14019b58e` | `mode=2,create=on` | 158352 |
| **00:01:34.244** | **`0x14019bbf3`** | **`mode=2,create=on,heapfix=on`** | **217724** |

`[L]` for every cell. **Ten deaths: eight at site 1, two at site 2, and one of the two was
unpatched.**

`research/fixtures/heap-corruption-3-five-field-entries-hook.log`, mtime **2026-08-20 22:55**:

```
22:46:38.086 ***** CLIENT FAULT #1: code=0xc0000374 ... *****
      stack: 0x144fe3010(vm) 0x14374e6ec(?) 0x143ad68a0(?) 0x14019bbf3<-TEXT
             0x140205833<-TEXT 0x1401be180<-TEXT 0x1415a1878<-TEXT 0x1415a1142<-TEXT
```

`grep -c -i heapfix` on that file returns **0**, its marker carries no `heapfix=` token, and
`crates/grap-stub/src/heapfix.rs` has mtime **2026-08-22 08:41** — the patch did not exist yet.
`[L]` The first three MapleStory frames are the identical `0x14019bbf3 / 0x140205833 /
0x1401be180` chain, on the identical pool context, differing only above `0x1401be120`.

**Blind spot, stated so it can be quoted:** that line is the hook's **shallow stack scan, not an
unwind** — the same instrument `heap-corruption-2026-08-27.md` §5 caught carrying two stale
frames. No dump exists for that run, so the chain cannot be checked against an unwind, and a
scan can in principle show a stale chain left by an earlier free that survived. What makes it
worth more than an isolated address is that each frame *explains* the next: `0x140205833` only
exists as a return address if `0x14019bb50` was called from `0x140205820`, and `0x1401be180`
only exists if `0x142ef3bb8` was called from `0x1401be17b`. `[D]`

**Reason 2 is not load-bearing.** Even if that attribution is wrong, Reason 1 stands on the
dump alone. Reason 2 is corroboration, and it is offered as corroboration.

### The scale of the miss

`tools/callers.py`, control passed:

| function | what it is | call sites | tail jmps |
|---|---|---:|---:|
| `0x14019b4e0` | pooled free, context hardcoded — **the patched one** | 3 853 | 5 |
| `0x14019bb50` | pooled free `(ctx, p)` — **the fatal one** | 232 | 0 |
| `0x140205820` | `operator delete(void*, size_t)` → `0x14019bb50` | 23 | 9 |
| `0x142ef3bb8` | five-byte `jmp 0x140205820` | **33 740** | **4 623** |

`[L]` A second, independent instrument agrees on the pool: a byte scan for
`lea rXX,[rip → 0x143AD68A0]` immediately followed by `call 0x14019bb50` finds **232** — the
same number `callers.py` reports for that function, i.e. **every call site of the fatal free
passes the same pool context the patched free serves.** `[L]`

Two counts agreeing is not corroboration when they share a blind spot, and these two do: both
are byte scans of the same static image and **neither can see a call through a register or a
vtable.** The agreement is worth something because the two scans key on different bytes
(`E8 rel32` vs `48 8D /r disp32`), not because there are two of them.

The headline number is the last row. `-HeapFix` patches the free that ~3 850 call sites reach,
and leaves untouched the one that `operator delete(void*, size_t)` reaches from **38 363**
sites in 22 486 functions. `[L]` MSVC emits sized delete for most `delete p` on a complete
type, which is why that route dominates. `[I]`

**A call-site count is not a frequency.** Nothing here measures how many *executed* frees take
each route, and one hot loop can outweigh ten thousand cold call sites. What the counts do
establish is that the patched path is not a choke point.

---

## 6. Q5 — should `-HeapFix` stay on?

**No. Switch it off**, unless the same edit widens it.

Plainly, and the reasons in order of weight:

1. **It cannot produce the falsification it was built for.** `heapfix.rs`'s own doc block says
   the point is a prediction that can come back false: *"if the client stops dying with
   `0xC0000374` while damaged slots still accumulate, the chain is confirmed end to end."* With
   two of three entry points unpatched and the majority delete route among them, the client will
   keep dying whatever the chain says, so the experiment cannot resolve. `[D]`
2. **It costs a measurement every run.** §4: it invalidates the free-list argument, which is the
   only constraint anyone has on *when* the `1` is written and the thing that killed
   "the object underruns its own buffer". Leaving the flag on means no future dump can be used
   that way. `[D]`
3. **It has never been shown to prevent anything**, and cannot be, post-mortem. `[L]`
4. Its stated risk — the discarded `not rax` path — is unchanged and still not closed.

**What would make it worth switching back on**, in one edit (`crates/` is not mine to write):

```
0x14019bb63   48 8b 42 f8   ->   8b 42 f8 90     mov eax,[rdx-8]; nop     (the fatal one)
0x14019ba53   48 8b 42 f8   ->   8b 42 f8 90     mov eax,[rdx-8]; nop     (optional; different
                                                 ladder, 2 call sites, never seen to kill)
```

Same width, same transformation, same verify-before-write. Two cautions for whoever writes it:

* **The read-before-write guard is much weaker here.** `48 8b 42 f8` is a common encoding and
  appears at both new sites *identically*, so the byte check cannot tell them apart or catch a
  wrong RVA the way it could for the hand-picked `48 8b 07`. The RVA carries the whole
  identification. Say so in the log line.
* **Even all three is not a fix.** There are 53 further inlined copies of the same ladder, 52 of
  them in functions that touch the same pool context. A byte patch has no choke point to aim at.
  **The only choke point is the write itself, and it is still not found.**

---

## 7. What could not be established

1. **Whether the patch prevented any death in this run.** Nothing instruments `0x14019b4e0`, and
   a dump cannot say whether it was ever reached with a damaged header. Unknown, and unknowable
   from this dump.
2. **Whether the patch pushed a damaged slot back onto a free list.** Measured: none is on any
   free list *now* `[L]`. A pushed slot that was subsequently re-allocated would be invisible, so
   this is not the same claim as "it never pushed one". `[I]`
3. **Who writes the `1`.** Unchanged, and this dump adds nothing: its single damaged slot was
   live, so it gives no free-list constraint at all.
4. **How many of the 53 inlined ladder sites can reach the damaged `0x20` class.** 52 of 56 sit
   in a function that also references `0x143AD68A0`, but that reference may belong to an
   allocation; the context register per inlined site was not traced. `[I]`
5. **The runtime frequency of each delete route.** Call sites were counted, not executions.
6. **Whether the `not rax` path is reachable.** Unchanged from
   `heap-corruption-2026-08-27.md` §6.4. Widening the patch to `0x14019bb50` and `0x14019ba40`
   would discard that path at two more sites, and nothing here narrows the risk.
7. **The 2026-08-20 site-2 death's stack**, beyond the shallow scan. §5, Reason 2, blind spot
   quoted there. No dump exists for that run.

---

## 8. Two things about this run that are not about the heap

* **The death was not inside a packet handler**, counted in two logs. `world.log` sent **9**
  `-> 0x01A0 SET_FIELD`; `client-patched\maplecw-hook.log` has **9** `opcode=0x01A0` dispatch
  lines, and the dispatch line is written on handler *return*. `[L]` The last dispatch line of
  any kind is `2461 opcode=0x0453` at `00:01:33.114` and the fault is **1.13 s later** at
  `00:01:34.244` `[L]`. That is run B's signature, not run A's — no handler was entered and
  abandoned.
* **Two variables moved in one run.** The marker carries `heapfix=on` and the archived fixture is
  named `magic-claw-1-damage-and-heapfix-armed-hook.log`, so the magic-claw damage question was
  being tested in the same session. It does not touch anything above — the free route is
  compiled in and the header value is identical to nine previous processes — but
  `CLAUDE.md`'s "test one variant at a time" was not held, and the next run should hold it.

## 9. One correction to the record

`research/heap-corruption.md`, 2026-08-21, already had this stack. It tabulated
`0x14019bbf3` as *"`FUN_14019bb6a`, 242 bytes — the heap walk that noticed"* and `0x140205833`
as *"a thunk"*, and correctly noted that `0x143ad68a0` is the client allocator's first
argument. All three rows were read quickly and none was counted: `0x14019bb6a` is not a heap
walk, it is a second `free` for that same allocator; `0x140205820` is not a thunk, it is
`operator delete(void*, size_t)`; and `0x143ad68a0` appearing in a *free* stack was the fact
that mattered.

Everything after that narrowed onto `0x14019b4e0` — correctly, for the eight deaths that
happened there — and `-HeapFix` was built on the narrowed picture. The counter-example was in
`research/fixtures/` the whole time, under a name that says what map the run visited rather
than where it died. **A table row written from a quick read is a claim**, and this file's own
§1 is the third time that sentence has had to be written down.
