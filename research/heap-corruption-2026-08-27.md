# The fourth and fifth dumps, and the first hard fact about *when* the slot is written

```
dumps\maplecw-crash-155108-c0000374-1.dmp   1 345 MB   403 s   failure type 4   1 damaged slot
dumps\maplecw-crash-158352-c0000374-1.dmp   1 275 MB  1046 s   failure type 8   2 damaged slots
```

Both written 2026-08-27 by the hook's own vectored handler. Below they are **dump A** (21:09,
pid 155108) and **dump B** (22:47, pid 158352). Dump B is the longest session this project has
recorded.

Tags: **[L]** read out of a file, **[D]** derived from something read, **[I]** inferred.

Nothing in this file edits `research/heap-wild-write.md` or `research/heap-third-dump.md`; it
answers the four questions the brief asked and one it did not.

---

## The short version

* **The damage is the same shape, a ninth time.** Both new dumps, exact value
  `0x0000000100000020`, both in the `0x20` size class. Across the five dumps of this family:
  **9 damaged slots in 962 112 enumerated pool slots, all nine identical, all nine in bucket 1,
  0 of 579 008 in the other three classes.** `[L]`
* **The one-per-250-s rate does not survive.** 1 046 s produced **2** damaged slots, not the
  ~4 the rate predicts. Damage still grows with age across the five points, but the fit is
  much worse than three points made it look. `[D]`
* **`-HeapFix` would have prevented both of these faults** — and it was **OFF in both runs**,
  so neither dump falsifies anything. `[L]` for the flag, `[D]` for the prevention: the fault
  frame `0x14019b58e` is the return address of the `call rbx` that *is* `HeapFree`, and it is
  reached only down the branch a `0x100000020` header selects. Read as a 32-bit value the same
  header selects bucket 1 and the block goes back to the pool free list instead.
* **New, and the first thing anyone has learned about the moment of the write:** in dump B one
  of the two damaged slots was **sitting on the pool's own free list**. A slot whose header is
  damaged can never be *pushed* onto that list — the free reads the header first and diverts to
  `HeapFree`, which is what kills the process. So that slot was **damaged while it belonged to
  nobody.** `[D]` That kills "the object living in the slot underruns its own buffer" outright
  and leaves an overrun from the preceding slot, or a stale pointer, on the table.

---

## 0. The instruments, and the controls that could have failed

Every tool was run with the repo as the working directory. No Ghidra.

| instrument | control | result |
|---|---|---|
| `tools/dumpwalk.py`, both dumps | its six self-checks | **6/6 PASS** in each `[L]` |
| the same, `.pdata` unwind vs the allocator's own captured stack | a low overlap means the unwinder is wrong, because the capture cannot be | **32 of 32 frames** in each `[L]` |
| `tools/poolchain.py`, dumps A, B **and** the archived 1224132 | per-chunk size identity `n*(slot+8)+8` | **23 190 chunks passed, 0 failed** `[L]` |
| a free-list walk (inline, not a new file) | `carved − freelist length == served`, the allocator's own two counters | **12 of 12 buckets PASS** across the three dumps `[L]` |
| `tools/listing.py` on `0x14019b4e0`, `0x14019d350`, `0x14019d3c0` | shares `reads.py`'s loader and extent | the three bytes at `+0x24` are `48 8b 07`, as `heapfix.rs` expects `[L]` |
| `tools/pdata_lookup.py` | — | `0x14019b58e` is `+0xae` in `0x14019b4e0 .. 0x14019b600`, 288 bytes `[L]` |

`poolchain.py` on `dumps\maplecw-crash-1224132-c0000374-1.dmp` was re-run rather than
remembered, and reproduces `heap-third-dump.md` exactly: 2 273 chunks in bucket 1, one damaged
slot at `0x39991e68`, `hdr 0x0000000100000020`, slot 31/32. `[L]`

**Two dumps of this family are no longer on disk.** `dumps\` holds only 1224132, 1235208 (the
`0xC0000005` outlier, not this family), and today's two. Every number below for the 596 s and
644 s processes is `[L]` **from `research/heap-wild-write.md`**, not re-measured here, and is
labelled where it matters.

---

## 1. Q1 — is it still the identical `0x0000000100000020`? Yes, nine for nine

```
dump A  bucket 1: 2551 chunks passed, 0 failed;  81632 slots, 1 damaged
          0x383de680  hdr 0x0000000100000020  chunk 0x383de3a8  slot 18/32  va%16=0   <<< freed
        buckets 0/2/3: 57664 + 40752 + 17728 = 116144 slots, 0 damaged

dump B  bucket 1: 2189 chunks passed, 0 failed;  70048 slots, 2 damaged
          0x47dcd9c0  hdr 0x0000000100000020  chunk 0x47dcd508  slot 30/32  va%16=0
          0x3b885280  hdr 0x0000000100000020  chunk 0x3b884eb8  slot 24/32  va%16=0   <<< freed
        buckets 0/2/3: 53376 + 32368 + 16904 = 102648 slots, 0 damaged
```

`[L]`. Passed/failed counts, not a sample: **8 215 chunks in dump A and 7 159 in dump B all
passed the per-chunk size identity, none failed it.**

The family, with the two older dumps quoted from `heap-wild-write.md`:

| dump | age | slots enumerated | damaged | value |
|---|---:|---:|---:|---|
| 1096760 `[L, quoted]` | 596 s | 226 200 | 2 | `0x0000000100000020` |
| 1160396 `[L, quoted]` | 644 s | 181 984 | 3 | `0x0000000100000020` |
| 1224132 `[L, re-run]` | 306 s | 183 456 | 1 | `0x0000000100000020` |
| **155108 (A)** `[L]` | **403 s** | **197 776** | **1** | `0x0000000100000020` |
| **158352 (B)** `[L]` | **1046 s** | **172 696** | **2** | `0x0000000100000020` |
| | | **962 112** | **9** | nine identical |

**0 damaged in 579 008 slots outside the `0x20` class.** `[L]` Not one other value has ever
appeared on a slot header in this pool.

### The failure-information field, and why the guard earns its keep here

Both dumps agree on where the reported address sits relative to the damage, and they do it
with *different* failure types:

| dump | type | `Address` | damaged header | relationship | R13 | R14 |
|---|---:|---|---|---|---|---|
| A | **4** | `0x383de670` | `0x383de680` | `Address + 0x10` | `0x383de680` | `0x383de670` |
| B | **8** | `0x3b885270` | `0x3b885280` | `Address + 0x10` | `0x3b885280` | `0x3b885270` |

`[L]` for all six columns. `R13` is the pointer `RtlFreeHeap` was given — the client's free
passes `rdi = ptr − 8`, i.e. the header address — and `R14 = R13 − 0x10` is the entry ntdll
computed from it, which is the listing in `heap-wild-write.md` §4 (`mov r8,r13` / `lea
r14,[rdx-0x10]`). `poolchain.py` marks a damaged slot when it equals `Address` **or**
`Address + 0x10`, which is why it found the right one under both conventions. `[D]`

**Type 4 is new to this family** (the previous four were types 8, 8, 9, 9), and it reports the
*entry*, not the caller's pointer — `0x383de670` is not a slot header, it is 24 bytes into the
body of slot 17. `[L]` `tools/dumpwalk.py`'s refusal message says of type 4 that it "puts the
CALLER'S POINTER in Address"; **this dump measures the opposite.** One observation, one
failure type, so it is not enough to change the tool on, and it does not need to be:

* the refusal is still **correct behaviour** for a better reason than the one it prints. The
  bytes at `Address` here are the tail of a pool slot body inside a 270 KB NT heap block, so
  decoding them as an `_HEAP_ENTRY` would print garbage whichever convention applies;
* it is a **claim that can now come back false**, which is what a fourth type is worth.

Do not undo the guard. The one thing worth noting for whoever owns `tools/`: the
**containing-block walk** sits in the same `elif`, so it was skipped for the type-4 dump. That
walk does not depend on the union's meaning at all, and on dump B it produced the single most
useful line in the file `[L]`:

```
  43 blocks stepped, every header checksum valid.
  0x3b885270 lies INSIDE the block whose entry is 0x3b869000:
    0x3b869000 .. 0x3b8ab010, 270352 bytes, flags 0x9 (BUSY=1), unused 0x20
    user data 0x3b869010 .. 0x3b8aaff0 (270304 bytes); the reported address is +0x1c260 into it.
```

**The NT heap's own metadata is intact.** 43 blocks stepped, every checksum valid. The process
was not killed by trampled heap structures; it was killed by being handed a pointer 115 296
bytes inside a live block. That is `heap-wild-write.md` §7 confirmed from a direction §7 did
not use. `[L]`

---

## 2. Q2 — does the count scale with session length? Weakly, and the rate over-predicts

| dump | age | damaged | s per damaged slot | `-> 0x01A0` SetField sends | `world` log lines |
|---|---:|---:|---:|---:|---:|
| 1096760 | 596 s | 2 | 298 | 13 | 32 030 |
| 1160396 | 644 s | 3 | 215 | 5 | 12 441 |
| 1224132 | 306 s | 1 | 306 | 3 | 14 221 |
| **A 155108** | **403 s** | **1** | **403** | **10** | **14 616** |
| **B 158352** | **1046 s** | **2** | **523** | **2** | **981** |

Ages `[L]` from each dump's own `process created`/`dump written` pair; dump B's agrees with
`client-exit.log`'s `after 1,046.5s of life` to within a second `[L]`. Packet counts `[L]` from
`previous-runs/world-20260821-202046.log`, `world-20260821-230104.log`,
`world-20260822-082646.log`, `world-20260827-210956.log` and `world.log`, paired to the dumps
by the archive timestamps.

**"About one damaged slot per 250 s" does not survive a session four times longer than the ones
it came from.** It predicts ~4.2 for dump B and the dump holds 2. `[D]`

What does survive:

* The rank correlation with **age** is 0.80 across the five points. `[D]`
* The rank correlation with **map loads** is **0.05** — none. `[D]`
* Neither is significant at n = 5 (Spearman needs |rho| = 1.0 at p = 0.05 for five points), so
  the number to take away is the **contrast**, not either value. `[D]`

The single cleanest case is dump B itself, and it is worth stating as an observation rather
than a statistic. Its last seventeen minutes were **idle**: `0x0453` NpcChat idle chatter every
two or three seconds, the 3 s session watch on schedule, **two map loads in the whole session**
`[L]`. Dump A ran 403 s, sent **ten** SetFields and logged 14 616 lines — fifteen times the
traffic in 40 % of the time — and accumulated **half** as much damage. `[L]`

So whatever writes the `1` is **not driven by map loads or by packet volume**, and it keeps
going while the client sits still in a town. That is a different claim from
`heap-third-dump.md` §3's "fires on something that happens continuously during play", and it is
sharper: *play* is not required.

**The bias `heap-third-dump.md` §6 named is still unaddressed.** All five points are deaths,
and a death requires a damaged slot to have been freed, so the sample is selected. A run killed
deliberately at a known age is still the missing measurement and still costs one launch.

And the count is emphatically **not** a fixed death time: 403 s with one damaged slot, 1 046 s
with two, 306 s with one. What kills is the *free* of a damaged slot, and when that happens is
chance. `[D]`

---

## 3. Q3 — what is at `0x14019b4e0`, and what `-HeapFix` changes about the faulting path

`tools/pdata_lookup.py`: `0x14019b4e0 .. 0x14019b600`, **288 bytes**, and `0x14019b58e` — the
address in both dumps' stacks and in the hook's `CLIENT FAULT` line — is **`+0xae`**. `[L]`

### What the function is

It is the client's **pooled `free`**, `void free_(void *p)`, and it serves exactly one pool
context: `lea rbp,[rip+0x393b366]` at `0x14019b533` resolves to `0x143AD68A0`, hardcoded.
`[L]` Full listing read with `tools/listing.py`; the parts that decide the fault:

```asm
14019b4e0  test rcx, rcx
14019b4e3  je   14019b5ff            ; free(NULL) -> return
14019b500  lea  rdi, [rcx - 8]       ; rdi = the 8-byte size header
14019b504  mov  rax, qword ptr [rdi] ; <-- +0x24, the -HeapFix site
14019b507  test rax, rax
14019b50a  jns  14019b50f
14019b50c  not  rax                  ; the one's-complement path
14019b50f  cmp  rax, 0x20
14019b513  ja   14019b520
14019b515  xor  ecx, ecx             ; rax <= 0x20:  bucket 0 or 1
14019b517  cmp  rax, 0x10
14019b51b  seta cl
14019b51e  jmp  14019b573
14019b520  cmp  rax, 0x40
14019b524  ja   14019b560
14019b526  mov  ecx, 2               ; 0x20 < rax <= 0x40 -> bucket 2
14019b52b  ...                       ; take the per-bucket spinlock, push onto the free list,
                                     ; dec the live counter at ctx + i*4 + 0x14, release
14019b560  mov  ecx, 0xffffffff      ; rax > 0x40
14019b565  mov  edx, 3
14019b56a  cmp  rax, 0x80
14019b570  cmovbe ecx, edx           ; <= 0x80 -> bucket 3, otherwise ecx stays -1
14019b573  test ecx, ecx
14019b575  jns  14019b52b            ; a bucket -> the pool
14019b577  mov  rbx, [rip+0x3939fb2] ; HeapFree
14019b57e  call [rip+0x3939fb4]      ; GetProcessHeap()
14019b587  mov  r8, rdi              ; ptr - 8
14019b58c  call rbx                  ; HeapFree(heap, 0, ptr - 8)
14019b58e  jmp  14019b5eb            ; <<< +0xae. THIS is the address in every stack.
```

`[L]` for all of it.

**`+0xae` is the return address of the `HeapFree` call and nothing else.** It is reached only
when the 64-bit value at `ptr − 8` is greater than `0x80` (or greater than `0x80` after the
`not`). `[D]`

### The `HeapFree` arm is not an error path

`0x14019d350(size)` is the large allocator: it calls `HeapAlloc(GetProcessHeap(), 0, size + 8)`,
stores `mov qword ptr [rax], rsi` — the **plain size, a full 8-byte store** — and returns
`rax + 8`. `[L]` So a genuine large block carries a positive size header at `ptr − 8` and
`HeapFree(ptr − 8)` is exactly the right thing to do with it. The `> 0x80` branch is the normal
large-block path; a damaged `0x20` header simply makes a pool slot look like one. `[D]`

The carve, `0x14019d3c0`, was read rather than quoted, and confirms `heap-wild-write.md` §5:
`mov qword ptr [rax+8], rdi` and `mov qword ptr [rax-8], rdi`, both **full 8-byte stores of the
plain slot size**, with the high dword explicitly zeroed. `[L]`

### What the patch does to *this* fault

`crates/grap-stub/src/heapfix.rs` replaces `48 8b 07` at `0x14019b504` with `8b 07 90`.
Tracing the real value through both codes:

| | `mov rax,[rdi]` (today) | `mov eax,[rdi]` (`-HeapFix`) |
|---|---|---|
| rax after the load | `0x0000000100000020` | `0x0000000000000020` |
| `test`/`jns` | positive, no `not` | positive, no `not` |
| `cmp rax,0x20` | above → `14019b520` | not above → `14019b515` |
| `cmp rax,0x40` | above → `14019b560` | — |
| `cmp rax,0x10` | — | above → `cl = 1` |
| ecx | `-1` | **1** |
| `test ecx,ecx / jns` | not taken | taken → `14019b52b` |
| outcome | `HeapFree(ptr−8)` → **`+0xae`** | pushed onto the **`0x20` free list** |

**So yes: `-HeapFix` would have prevented both of today's faults.** `[D]`, resting on three
measured things — the header value is exactly `0x0000000100000020` in both dumps `[L]`, the
fault frame is the `HeapFree` return address `[L]`, and the branch arithmetic above `[L]`. The
slot genuinely *is* a `0x20` slot, so returning it to bucket 1 is the correct outcome and not a
suppression.

### The flag was never on

**Both runs were unpatched.** `[L]`

```
previous-runs\maplecw-hook-20260827-210956.log:6   mode="mode=2,create=on"
client-patched\maplecw-hook.log:6                  mode="mode=2,create=on"
```

No `heapfix=` token in either marker, and `grep -i heapfix` over both hook logs returns
nothing. `heapfix.rs` logs on every outcome once the flag is set — patched, refused, write did
not take, address unreadable — so silence means the flag was off, not that the patch failed.
It **is** wired (`crates/grap-stub/src/hook.rs:517` calls `heapfix::install()`;
`tools/test-server.ps1:825` appends the token for `-HeapFix`); it has simply never been
switched on. **Neither of these dumps is a falsification of anything.**

### The risk, narrowed on one side and unchanged on the other

`heapfix.rs` states one risk: the patch discards the `not rax` path. Two things move:

* **Legitimate large blocks are unaffected**, which `heapfix.rs` does not say. The patch can
  only change a decision for a header whose **high dword is non-zero**, i.e. a size of 4 GB or
  more. `0x14019d350` puts every request `>= 0xc800000` (200 MB) through a diagnostic call
  (`mov ecx,0x3a; call 0x142e541f0`) before it allocates `[L]`, so a request twenty-one times
  larger than that threshold is not something this allocator serves quietly. For every size
  under 2^32 the low dword *is* the value and `mov eax,[rdi]` reads the same number. `[D]`
* **Nothing in this allocator family can write a sign-bit-set header.** Both writers — the
  carve at `0x14019d3c0` and the large allocator at `0x14019d350` — store a plain positive
  size with a full 8-byte store. `[L]` So the `not` path is unreachable *from them*.

The residual risk is unchanged and is a pointer from **some other** allocator reaching this
free. That surface is large: `tools/callers.py` reports **3 853 call sites in 2 504 functions**,
plus 5 tail `jmp`s and 0 data pointers `[L]`. Enumerating where each of those pointers was
allocated is not a pass that was attempted here.

**One instrument tried and failed, recorded so nobody repeats it.** A whole-memory byte sweep
of both dumps for the four one's-complement headers (`~0x10`, `~0x20`, `~0x40`, `~0x80`) at
8-aligned addresses found 113–655 occurrences of each per dump. Its positive control passed —
it found all three known damaged headers `[L]` — and the result is still **worth nothing**: a
qword that equals −33 is not a header, and a byte sweep cannot tell the difference. It is
recorded as a dead instrument, not as a negative.

---

## 4. Q4 — who writes it? Not answered, but the *question* has moved

The static form of this hunt is exhausted and `heap-wild-write.md` §8 names its blind spot in
one sentence: a store through a pointer that was already adjusted (`lea rdx,[rcx-4]` then
`mov dword ptr [rdx], 1`) appears at displacement **0**, the commonest displacement in every
module. That scan was not re-run.

What is new comes from the free list, and it is the first constraint anyone has put on the
*moment* of the write.

### A damaged slot was found sitting on the pool's free list

Walking bucket 1's free list from `ctx + 0x70` and testing membership, with the allocator's own
counters as the control `[L]`:

```
dump A   bucket 1: carved 81632 - freelist 22255 = 59377, served 59377   [PASS]
           damaged 0x383de680 -> body 0x383de688 : on free list = False
dump B   bucket 1: carved 70048 - freelist  4501 = 65547, served 65547   [PASS]
           damaged 0x47dcd9c0 -> body 0x47dcd9c8 : on free list = TRUE
           damaged 0x3b885280 -> body 0x3b885288 : on free list = False
1224132  bucket 1: carved 72736 - freelist 13085 = 59651, served 59651   [PASS]
           damaged 0x39991e68 -> body 0x39991e70 : on free list = False
```

The identity `carved − freelist == served` closes exactly in **12 of 12 buckets across the
three dumps** `[L]`, and the free-list head is in the walked set in each — the walk is not
short.

### Why that is decisive about the moment of the write

The free at `0x14019b4e0` **reads the header before it pushes**. A slot whose header is
`0x0000000100000020` cannot be pushed onto the free list: the comparison chain sends it to
`HeapFree`, which is the death. So consider every history that ends with a damaged slot on the
free list:

* *carved → allocated → damaged by its occupant → freed* — impossible, that free is the fatal
  one and the slot would not be on the list;
* *carved → allocated → freed cleanly → damaged* — the write happened while the slot was free;
* *carved → never allocated → damaged* — the write happened while the slot was free;
* *damaged → allocated → freed again* — impossible, the second free reads the damaged header.

**In every surviving history the slot was on the free list when the `1` was written.** `[D]`

That is `0x47dcd9c0` in dump B, measured today. `heap-wild-write.md` §3 records the same state
for `0x379d71d8` in the 596 s dump `[L, quoted]` — that dump is no longer on disk, so it is a
second instance on the strength of that file, not of a re-measurement.

Its **predecessor**, slot 29 at `0x47dcd998`, is also on the free list right now, and its own
header reads a clean `0x0000000000000020` `[L]`:

```
0000000047dcd998  20 00 00 00 00 00 00 00  f0 93 dc 47 00 00 00 00   <- clean header, free-list link
0000000047dcd9a8  00 00 00 00 00 00 00 00  00 00 00 00 77 00 42 00
0000000047dcd9b8  6f 00 64 00 79 00 00 00  20 00 00 00 01 00 00 00   <- the damaged header
0000000047dcd9c8  88 b5 dc 47 00 00 00 00  00 00 00 00 00 00 00 00   <- and its body, also free-listed
```

### What that kills, and what it leaves standing

* **Killed: "the object living in the damaged slot writes at its own `body − 4`."** For this
  instance there was no such object. `[D]`
* **Not killed: "the object in the preceding slot overruns its 32-byte body by 4."** Slot 29 is
  free *now*; it could have been live when it overran and been freed afterwards, and its own
  header is clean so that free would be unremarkable. `[D]`
* **Not killed: a stale pointer or a wild write from anywhere else.**

**The blind spot, stated so it can be quoted:** the argument assumes the only two ways onto that
free list are the push at `0x14019b5c2..0x14019b5ca` and the initial chain the carve builds. I
did **not** enumerate every writer of `ctx + i*8 + 0x68`; a second pusher that does not read the
header would break it. That enumeration is a `[reg+disp]` scan of exactly the shape `CLAUDE.md`
warns about, so it needs a different form — the constant `0x143AD68A0` and its `lea`, not the
displacement.

### And the contents are still a dead end, for the reason §6 gave

For the record only. Dump A's fatal slot holds `"obj_guide"`, its predecessor a free-list link
and stale `"10198"`. Dump B's fatal slot holds `"[s]wwwigxecom"`, its predecessor
`"[s]wwwgpko"`. `[L]` The header is written at carve and never again, so a slot may have been
damaged tens of thousands of allocations before whatever occupies it now. Nine damaged slots
have now produced nine unrelated strings.

---

## 5. The two deaths, counted in two logs

`world.log` says what the server sent; `client-patched\maplecw-hook.log` writes one dispatch
line per packet **on handler return**. Counting the same event in both `[L]`:

| | SetFields sent | dispatch lines | reading |
|---|---:|---:|---|
| run A (403 s) | 10 | **9** | the tenth handler was entered and never came back |
| run B (1046 s) | 2 | **2** | the fatal free is not inside a packet handler |

Run A died **inside a `0x01A0` SetField**, 6 ms after `WATCH #29 ... while dispatching opcode
0x01A0`, in the middle of a field-entry inventory batch `[L]` — the same signature as the
1224132 death. Run B's last dispatch line is `392 opcode=0x0453 elapsed_us=106.1 ret=1` at
22:47:22.762, and the fault is 0.3 s later at 22:47:23.060 `[L]`; the unwind puts it under the
window procedure (`0x142c8c8f0` → `0x142c20d90` → `0x142c119a0` → `0x1416a79a0` → `0x1411af030`
→ `0x1415b3120` → `0x1408bed00` → `0x1408c2080` → PCOM). Run A's unwind goes through
`ResMan.dll` and four nested `oleaut32!VariantClear` recursions instead.

So the fatal free is reached from **two more distinct consumers**, neither of them a socket
teardown, which is `heap-wild-write.md` §9's "common consumer, several routes" with two more
routes on it. `[D]`

### One correction to the hook's own fault line

The brief quotes run B's `CLIENT FAULT` stack:

```
stack: 0x144fe3010(vm) 0x143a361a4(?) 0x141a6aab9<-TEXT 0x141a63671<-TEXT 0x14019b58e<-TEXT
```

That line is the hook's **shallow scan**, not an unwind. `0x141a6aab9` and `0x141a63671` appear
nowhere in dump B's `.pdata` unwind or in the allocator's 32-frame capture; they are at
`0x14d2a8` and `0x14d388` in `dumpwalk.py`'s heuristic stack-scan section, i.e. stale slots
`[L]`. `0x143a361a4` and `0x144fe3010` fall in no function at all `[L]`. **Only `0x14019b58e`
in that line is corroborated** — and it is corroborated three ways: the unwind, the allocator's
captured stack (frame #3 in both dumps), and `pdata_lookup`. The brief's link between the fault
and the `-HeapFix` site is right; it just does not need the other four addresses.

---

## 6. What could not be established

1. **Who writes the `1`.** Not answered. The static form is exhausted with a named blind spot
   and was not repeated; the free-list result constrains *when*, not *who*.
2. **Overrun from the preceding slot vs. a stale pointer.** §4 kills only the damaged slot's own
   occupant. Nothing here distinguishes the other two.
3. **Whether the rate is real.** All five points are deaths and a death is a selected sample.
   The clean sample `heap-third-dump.md` §6 asked for — a run killed deliberately at a known
   age — has still never been taken, and the two correlations in §2 are n = 5 and neither is
   significant.
4. **Whether the `not rax` path is reachable.** Narrowed to "not from either allocator that
   writes these headers" `[L]`; not closed. 3 853 call sites were counted, not traced. The
   byte sweep that tried to close it could not discriminate and is recorded as a dead
   instrument.
5. **Whether `-HeapFix` works.** Unknown, and unknowable from these two dumps: the flag was off
   in both. `[L]`
6. **What `Address` means for failure type 4 in general.** One measurement, one dump. It says
   the entry convention; `dumpwalk.py`'s refusal message says the pointer convention. Not
   enough to move the tool, and the refusal is right either way.
7. **Whether slot 29 was live when slot 30 was damaged.** A post-mortem cannot say.

---

## 7. What the next run is worth, given all of the above

Only two things, and they are one launch each.

**Switch `-HeapFix` on.** It is the only outstanding question with a prediction that can come
back false, and §3 shows it addresses the exact branch both of today's faults took. The two
outcomes and what each means, so the run is a measurement:

* **The client stops dying with `0xC0000374`.** Then take a dump anyway (or kill it
  deliberately, which also gets §2 its unbiased point) and run `poolchain.py`. If damaged slots
  are still accumulating while nothing dies, the chain in `heap-wild-write.md` is confirmed end
  to end and the remaining work is only "who writes it".
* **It dies with `0xC0000374` anyway.** Then check the hook log for the `HEAPFIX:` line first —
  patched, refused, or write-did-not-take are three different runs — and if it says patched,
  something in that chain is wrong and the dump says which half.
* **It dies with something new**, e.g. `0xC0000005` inside `HeapFree`. That is the `not rax`
  path firing, which no measurement here could reach, and it names the risk `heapfix.rs`
  states.

**And whatever else that run does, leave the client idle for ten minutes.** §2's sharpest result
is that dump B accumulated damage while doing nothing but receiving idle NPC chatter. If a
deliberately-killed idle run of ~900 s shows two or three damaged slots, the writer is on a
timer and the search narrows enormously; if it shows zero, the age correlation is an artefact of
five deaths and §2 comes back false.
