# The two crash dumps compared, and what actually damages the slot header

Second pass over `0xC0000374`, using **both** dumps against each other. The first dump's
analysis is `research/heap-corruption-dump.md` and is not edited here.

```
dumps\maplecw-crash-1096760-c0000374-1.dmp   1 010 MB   596 s   failure type 8
dumps\maplecw-crash-1160396-c0000374-1.dmp   1 332 MB   644 s   failure type 9
```

Tags: **[L]** read out of a file, **[D]** derived from something read, **[I]** inferred.

---

## The short version

**The two dumps show the *same* damage, not different damage.** Every damaged header in
both processes reads exactly `0x0000000100000020` — a stray `1` in the high dword of a
32-byte slot's size header. Five instances across two processes, five identical values.
**[L]**

The brief for this pass, and the fixture header it came from, say the second dump has
"differently-garbled bytes" and that the first dump's `+4` finding therefore "does NOT
generalise". **That is wrong, and §2 says exactly which line produced it.** The
`+4` finding generalises completely.

What is new and load-bearing:

* The size header is written **once**, at carve time, by a **full 8-byte store**
  (`mov [rax-8], rdi` in `FUN_14019d3c0`), and no code in the allocator ever writes it
  again. So a non-zero high dword can only be a foreign write. **[L]**
* Only **4 bytes** were written, not 8: the low dword still holds the carve value `0x20`,
  and an 8-byte-aligned qword store on x86-64 cannot tear. **This retires lead 3** — it is
  not a torn `{DWORD,DWORD}` write. **[D]**
* Across **408 184** enumerated pool slots in the two dumps, damage occurs **only** in the
  `0x20` size class: 5 of 158 688, against **0 of 249 496** in the `0x10`, `0x40` and
  `0x80` classes. **[L]**
* The two failure types are the **same defect down two arms of one `if`** in ntdll,
  selected by `test r13b, 0xf` — whether the freed pointer is 16-byte aligned. The pool's
  stride is 40, so consecutive slots alternate 0 and 8 mod 16 and the failure type is
  decided by the **parity of the damaged slot's index**. **[L]** for the branch, **[D]**
  for the parity.
* Neither PCOM nor the client's allocator disagrees about the block layout. **Both are
  right.** A third party writes 4 bytes at `body-4`. §7.
* The static form of lead 1 is **exhausted and could not have succeeded** — §8 names its
  blind spot rather than reporting a negative.

---

## 1. What was measured, and with what

Four instruments, each with a control that could have failed:

| instrument | what it does | control |
|---|---|---|
| `dumpwalk.py` | the dump itself | its own six self-checks, all PASS in both dumps **[L]** |
| a byte sweep of all mapped memory | finds every 8-aligned qword whose low dword is a bucket size and whose high dword is non-zero, then keeps only those whose neighbours at `±(8+size)` are headers of the same class | the already-known damaged address must come out of the sweep. **PASS in both dumps** **[L]** |
| a **carve-identity** filter | walks a candidate's slot run to its ends and requires `u64(chunkbase-8) == n*(size+8)+8` | rejected **108 of 110** dump-1 candidates and **139 of 142** dump-2 candidates while passing the known one **[L]** |
| a **chunk-chain walk** | enumerates every slot the allocator ever carved, from the pool context, with no scanning at all | the allocator's own carved-slot counter equals the number of slots walked, exactly, in all 8 buckets across both dumps **[L]** |

The last two matter because they have **different blind spots** and agree anyway: the byte
sweep starts from bytes and could miss a header it cannot pattern-match; the chain walk
starts from the allocator's own bookkeeping and could miss a chunk that fell off the list.
Both return the same 2 slots in dump 1 and the same 3 in dump 2.

### The pool, read out of the client

`FUN_14019b4e0` (free) hardcodes its pool context with `lea rbp,[rip+0x393b366]` at
`0x14019b533`, which resolves to **`0x143AD68A0`** **[L]**. That is the same address
`research/equip-crash.md` records as `FUN_14019b780(&DAT_143ad68a0, …)` and
`research/heap-corruption.md` calls "the heap handle" — it is **not** a heap handle, it is
the pool context. **[D]**

Its layout, read from the two functions **[L]**:

```
ctx + i*4 + 0x04   slots carved          ctx + i*8 + 0x28   lock word (owner TEB)
ctx + i*4 + 0x14   objects live          ctx + i*8 + 0x68   free-list head
                                         ctx + i*8 + 0x88   chunk-list head (a body0)
```

and the bucket table, off the switch at `0x14019b7f0` **[L]**:

| i | slot | slots/chunk | chunk bytes |
|---|---|---|---|
| 0 | `0x10` | 64 | `0x608` |
| 1 | `0x20` | 32 | `0x508` |
| 2 | `0x40` | 16 | `0x488` |
| 3 | `0x80` | 8 | `0x448` |

Two arithmetic identities hold, in every bucket of both dumps, and they are the reason the
enumeration can be called complete **[L]**:

* slots walked == the `slots carved` counter (e.g. dump 1 bucket 1: 2 721 chunks × 32 =
  **87 072**, and the counter reads **87 072**)
* `slots carved − free-list length == objects live` (dump 1 bucket 1: 87 072 − 50 457 =
  **36 615**, counter **36 615**; dump 2: 71 616 − 9 822 = **61 794**, counter **61 794**)

---

## 2. The retraction: "differently-garbled bytes"

`research/fixtures/heap-second-dump-same-stack-different-type.log` says of dump 2:

> a differently-garbled header. So the first dump's 'a stray 1 at +4' is NOT a constant

That sentence came from this block of `dumpwalk.py` output **[L]**:

```
  Reading those 16 bytes as a _HEAP_ENTRY (XOR key 0x1e513d50dcbd):
    encoded 0072005000000010 -> decoded 00721e013d50dcad
    size 0xdcad  flags 0x50  prevsize 0x1e01  unused 0x0  header checksum BAD
```

**That decode is meaningless for a type-9 failure**, and the giveaway is in its own input:
`0072005000000010` is `10 00 00 00 50 00 72 00` — the length prefix `0x10` followed by
`"Pr"`. It is XOR-decoding the string `"Property"`.

The reason is that `_HEAP_FAILURE_INFORMATION.Address` means **different things for the two
failure types**, which §4 shows in ntdll's own instructions: type 8 reports the *heap entry*
(`pointer − 0x10`), type 9 reports the *pointer that was passed*. `dumpwalk.py` reads the
field as an entry either way, which is right for dump 1 and wrong for dump 2. The tool is
not at fault — nothing had told it there were two conventions.

Read at the right place, dump 2's bytes are **not** garbled differently. They are identical:

```
dump 1  3c218ad0   20 00 00 00 01 00 00 00 | 0c 00 00 00 | "NoCri1"
dump 2  2e8ee7f8   20 00 00 00 01 00 00 00 | 10 00 00 00 | "Property"
```

**[L]** for both. Same size class, same offset within the slot, same value.

---

## 3. The three questions

### Always the same size class? **Yes — exclusively.**

All five damaged headers are in bucket 1, the `0x20` class. **[L]**

| | `0x10` | `0x20` | `0x40` | `0x80` |
|---|---:|---:|---:|---:|
| dump 1 slots / damaged | 81 024 / 0 | 87 072 / **2** | 40 464 / 0 | 17 640 / 0 |
| dump 2 slots / damaged | 58 240 / 0 | 71 616 / **3** | 33 200 / 0 | 18 928 / 0 |

**0 damaged in 249 496 non-`0x20` slots.** **[L]** If damage were spread evenly over slots
one would expect about 3 of the 5 to land outside bucket 1; seeing none is worth roughly
one chance in a hundred, so it is suggestive rather than conclusive on its own — but it is
the same answer in two independent processes. **[D]**

### Always at the same offset in the arena? **No — nowhere near.**

| dump | header | chunk | position in chunk list | slot | va mod 16 | state |
|---|---|---|---:|---|---:|---|
| 1 | `0x3c218ad0` | `0x3c2187f8` | 390 of 2 721 | 18/32 | 0 | live, **this is the one that was freed** |
| 1 | `0x379d71d8` | `0x379d6d98` | 819 of 2 721 | 27/32 | 8 | **on the free list** |
| 2 | `0x2e8ee7f8` | `0x2e8ee458` | 1 186 of 2 238 | 23/32 | 8 | live, **this is the one that was freed** |
| 2 | `0x36b1eb90` | `0x36b1e8b8` | 681 of 2 238 | 18/32 | 0 | live |
| 2 | `0x3a4f01e8` | `0x3a4f0078` | 76 of 2 238 | 9/32 | 8 | live |

**[L]** for all of it. Different chunks, scattered slot indices, and chunk-list positions
spread from 76 to 1 186 — the chunk list is push-front, so that is spread across the whole
run, not clustered at one phase. **[D]**

The free-listed one in dump 1 is worth naming: a damaged slot sitting on the free list is a
**loaded gun**. It will be handed out again, filled, released, and kill the process. The
process that died at 596 s was already carrying a second death. **[D]**

### Is the corrupting value ever the same? **It is always the same.**

`0x0000000100000020`, five times out of five, two processes. **[L]** Not one other value
appears on any of the 408 184 enumerated slots. **[L]**

Also enumerated and worth recording: **zero** headers anywhere use the one's-complement
encoding that `FUN_14019b4e0`'s `test rax,rax / jns / not rax` exists to handle — such a
header would have high dword `0xFFFFFFFF` and would have been reported by the same walk.
**[L]** That branch appears to be dead in both runs.

### So: not a wild write

The brief's premise — *"a wild write into that arena, not one repeatable off-by-one"* —
does not survive. It is **one repeatable off-by-one**: same offset, same value, same size
class, in two processes.

---

## 4. Why the failure types differ, and why it is not a second bug

Both dumps' captured stacks come from the same ntdll function, `0x7ffca82d5710`, and both
frame #1s are the return of a `call 0x7ffca83be17c` inside it — the routine that captures
32 frames and raises. They are **two arms of one `if`** **[L]**:

```asm
7ffca82d5b1d  test  r13b, 0xf              ; is the freed pointer 16-byte aligned?
7ffca82d5b21  je    7ffca82d5bae           ; aligned -> fall through to the busy-bit test
7ffca82d5b27  xor   r9d, r9d
7ffca82d5b2f  mov   r8, r13                ; arg3 = the POINTER, not the entry
7ffca82d5b37  mov   rdx, rsi               ; arg2 = the heap
7ffca82d5b3a  lea   ecx, [r9 + 9]          ; arg1 = 9   <-- the failure type, literal
7ffca82d5b3e  call  7ffca83be17c
7ffca82d5b43                               ; <-- dump 2's captured frame #1

7ffca82d5bae  lea   r14, [rdx - 0x10]      ; the aligned arm: entry = pointer - 0x10
   ...                                     ; (this is the listing in heap-corruption-dump.md)
7ffca82d5be7  lea   ecx, [r9 + 8]          ; arg1 = 8
7ffca82d5beb  call  7ffca83be17c
7ffca82d5bf0                               ; <-- dump 1's captured frame #1
```

Both type constants are read out of the instruction stream, not off an enum list, which is
the standard `heap-corruption-dump.md` set for type 8.

Now the arithmetic. A chunk base is 16-aligned and slot 0's header sits at `base+8`, so
with a stride of 40 the headers alternate **8, 0, 8, 0 …** mod 16. **The failure type is
decided by the parity of the damaged slot's index** **[D]**:

* dump 1, slot **18** (even) → header `0x3c218ad0`, 0 mod 16 → aligned arm → **type 8**
* dump 2, slot **23** (odd) → header `0x2e8ee7f8`, 8 mod 16 → misaligned arm → **type 9**

Two failure types, one defect. `heap_failure_invalid_argument` is not a different bug and
not a different heap problem; it is the same free of the same kind of pointer, which
happened to land on an odd slot.

---

## 5. The header is written once, and only four bytes changed

`FUN_14019d3c0(slotSize, count)`, the carve **[L]**:

```asm
14019d3d2  lea   rsi, [rcx + 8]        ; stride = slotSize + 8
14019d3eb  call  14019d350             ; base B for stride*count + 8 bytes
14019d3f3  mov   qword ptr [rax], r8   ; [B] = 0        (later the chunk-chain link)
14019d3fa  mov   qword ptr [rax+8], rdi ; [B+8] = slotSize   <-- QWORD store
14019d3f6  lea   rdx, [rax + 0x10]     ; body0 = B + 0x10
14019d410  lea   rax, [rcx + rsi]      ; loop: next body
14019d414  mov   qword ptr [rcx], rax  ;   free-list link, written in the BODY
14019d41a  mov   qword ptr [rax-8], rdi ;  [body-8] = slotSize  <-- QWORD store
```

Three things follow, and together they are the strongest result in this file:

1. **The size header is a full 8-byte store of the slot size.** The high dword is
   explicitly zeroed by the carve, so it is not stale residue from a previous occupant.
   **[D]** — this refutes the otherwise attractive "the allocator writes a dword and the
   free reads a qword" theory outright.
2. **Nothing rewrites it afterwards.** `FUN_14019b780` (alloc) only pops the free list and
   writes `[body0-0x10]`; the free path only pushes `[body]`. Neither touches `body-8`.
   So any non-zero high dword is a **foreign** write. **[D]**
3. **The foreign write was exactly 4 bytes wide.** The low dword still reads `0x20`, the
   carve's own value. Slot headers are 8-byte aligned, and an aligned 8-byte store on
   x86-64 is atomic — it cannot half-land. So this is a genuine `mov dword ptr [X], 1`
   at `X = body-4`, not the high half of something bigger. **[D]**

**This retires lead 3.** The brief proposed *"a torn `{DWORD,DWORD}` write from a second
thread would explain a stray value with no bug in either writer."* There is nothing to tear
against: the allocator wrote the header once and walked away, and the surviving low dword
proves only four bytes were replaced. A second thread may well be the one doing it, but it
is doing a whole, deliberate 4-byte store, and somebody's code contains it.

---

## 6. What the slot contained, and why that is a dead end by construction

| dump | damaged slot holds | the slot before it holds |
|---|---|---|
| 1 | string `"NoCri1"` | string `"Property"` |
| 1 | a free-list link + stale `"owcaseChair"` | a pointer + `".img/Focus2"` |
| 2 | string `"Property"` | string `"origin"` |
| 2 | string `"Cong!!!!"` | a pointer + stale `"ie…"` |
| 2 | an object of 3 pointers + `{1, 0x8f}` | the same object shape |

**[L]**. There is no shape in either column. That is not a surprising result, it is a
**necessary** one: the header is written at carve and never again, so a slot may have been
damaged tens of thousands of allocations ago and recycled ever since. **Current contents
cannot testify about the moment of damage.** **[D]** Any future pass that tries to identify
the writer from what is in the slot is answering a question the data structure cannot hold.

One negative here does have a control worth keeping. In dump 2's third case the damaged
header is `previous_body + 0x24` — four bytes past the end of the previous 32-byte slot —
and that previous slot holds an object whose **three immediate siblings in the same chunk
are byte-identical in shape and all clean** **[L]**. If that object type wrote a field at
`+0x24`, all four would carry it. So "a fixed 4-byte overrun off the end of the previous
object" is not supported. **[D]** Blind spot, stated so it can be quoted: the siblings show
what that object type does *now*, and the damage may predate the current occupant, so this
rules out a systematic overrun by *that* type and nothing more.

---

## 7. Which side is wrong

**Neither PCOM nor the client's allocator disagrees about the layout. Both are correct.**

Walking it end to end for dump 2 **[L]**:

| step | address | what is there |
|---|---|---|
| the string PCOM held | `0x2e8ee804` | `"Property"` in UTF-16 |
| PCOM's `add rcx,-4` | `0x2e8ee800` | the length prefix, `0x10` = 16 bytes |
| the client free's `lea rdi,[rcx-8]` | `0x2e8ee7f8` | the 8-byte size header |

Every subtraction lands exactly on the field it is supposed to. The block **is** a genuine
slot of this pool — it satisfies the carve identity, it is on the chunk chain, its size
field still reads `0x20` — so it was not allocated with a foreign header convention and
handed to the wrong deallocator. **[D]** The answer to the brief's question is **no**.

There is, however, a clear division between the **cause** and the **amplifier**, and the
amplifier is on the client's side:

```asm
14019b500  lea    rdi, [rcx - 8]
14019b504  mov    rax, qword ptr [rdi]   ; reads all 64 bits of a field whose only
14019b507  test   rax, rax               ; legal values are 0x10/0x20/0x40/0x80
14019b50a  jns    14019b50f
14019b50c  not    rax
14019b50f  cmp    rax, 0x20
   ...
14019b570  cmovbe ecx, edx               ; > 0x80  ->  ecx = -1
14019b577  mov    rbx, [rip+0x3939fb2]   ; HeapFree
14019b58c  call   rbx                    ; HeapFree(GetProcessHeap(), 0, ptr-8)
```

**[L]**. There is no validation at all. Four stray bytes in the top half of that field turn
a pooled 32-byte slot into a `HeapFree` of a pointer that is 250 KB inside a live heap
block, which Windows answers by killing the process. Had the read been 32-bit, or masked,
the block would have gone back to the `0x20` free list — the *correct* outcome, since it
really is a `0x20` slot — and the stray write would have been harmless. So the client's
free is not the bug, but it is the entire reason a 4-byte scribble becomes a process kill
ten minutes later. **[D]**

---

## 8. Lead 1, done, and why its silence proves nothing

Swept **PCOM.dll, NAMESPACE.DLL, ResMan.dll and MapleStory.exe** — 137 593 functions in the
four `.pdata` tables, **137 585 decoded** (8 exe entries skipped as larger than `0x20000`
bytes, which are Themida spans rather than functions), decoded per-function out of the
dump's mapped image, never a linear sweep.
Controls: the known refcount instruction `lock xadd dword ptr [rbx + 0x10], eax` at
`PCOM.dll+0x12cfa` must be decoded and classified as a 4-byte memory write (**PASS**, after
a first run **FAILED** it because capstone spells the mnemonic `lock xadd`, not `xadd`);
and the immediate matcher must fire at all (**PASS**, 32 435 hits). **[L]**

Enumerated first, filtered second — every `mov dword ptr [mem], 1`, histogrammed by
displacement **[L]**:

| module | such writes | distinct displacements | at **−4** |
|---|---:|---:|---:|
| NAMESPACE.DLL | 489 | 50 | **0** |
| PCOM.dll | 384 | 46 | **0** |
| ResMan.dll | 242 | 34 | **0** |
| MapleStory.exe | 31 320 | 928 | **1**, and it is `[rbp-4]`, a stack local |

Widened to **any** 4-byte store at displacement −4 whatever the source: 27 in the three
DLLs, 343 in the exe. Every one inspected in the DLLs is accounted for — `memset`'s
small-length tail (`0x152c42cf0` and its twins in ResMan and NAMESPACE are all the same
statically-linked `memset`), AES/digest buffer tails at `PCOM.dll+0x47b0` and `+0x6a80`,
`[base+rax-4]` string-copy tails, and `[rbp-4]` frame locals. **[L]**

**This negative is worth nothing on its own, and here is the blind spot in one sentence:**
a store through a pointer that was already adjusted — `lea rdx,[rcx-4]` then
`mov dword ptr [rdx], 1` — appears at **displacement 0**, and displacement 0 is the single
most common displacement in every module swept (124, 176, 86 and 1 565 immediate-1 stores
respectively). This is exactly the `mob+0x42c` failure `CLAUDE.md` records: a `[reg+disp]`
scan cannot see a store through a pointer that was `lea`'d and handed off. Lead 1's static
form **could not have succeeded**, and re-running it wider is not a second opinion.

---

## 9. Lead 2, and a correction to the brief's stack claim

The brief says the stack is *"the same path, frame for frame, in both dumps."* **It is not.**
**[L]**

Frames #0–#21 are identical: `RtlFreeHeap` ← the client's free ← `PCOM fn 0x152c12ce0` ←
PCOM ×5 ← `oleaut32!VariantClear` ← NAMESPACE ×3 ← PCOM ×5 ← `VariantClear` ←
`ResMan+0xe637` ← `ResMan+0x118cd`. Above that they diverge and only rejoin at the thread
start:

```
dump 1  #22 ResMan+0x3877  #23 ResMan+0x3820  #24 fn 0x142c4b730+0x55  #25 fn 0x142c42f30+0x594
dump 2  #22 fn 0x14181d650+0x885  #23 fn 0x144eb76b0+0x12c  #24 fn 0x144e5ee89+0xaf  #25 fn 0x142c42f30+0x43b
```

`fn 0x144eb76b0` and `fn 0x144e5ee89` have begin-RVAs `0x4eb76b0` and `0x4e5ee89`, inside
the `0x3d87000 … 0x4fe283d` range that exists **only** in the live function table —
Themida's own code. `tools/pdata_lookup.py` on the file cannot see them; the dump can.
**[L]**

Naming what was asked for:

* **`fn 0x142c42f30`** is the worker thread's long linear body: two no-argument API calls
  stored to globals, then a run of ~30 consecutive no-argument `call`s. The two crash sites
  are two *different* calls late in that run — `call 0x142c45e50` at `+0x436` (dump 2) and
  `call 0x142c4b730` at `+0x58f` (dump 1). **[L]**
* **`fn 0x142c4b730`** is a **release-globals routine** **[L]**: a repeated
  `rcx = [global]; if (rcx) { rax = [rcx]; call [rax+0x10]; } [global] = 0` — release a COM-ish
  singleton and null it. Dump 1's return address `0x142c4b785` is the instruction after the
  virtual call at `0x142c4b782`, on the global at **`0x143ADD058`**. **[D]**
* That global is identified, not guessed. It holds `0x066c3ef8`, whose vtable is
  **`ResMan.dll+0x848e0`**, and whose `vtbl+0x10` — the slot actually called — is
  `ResMan.dll fn 0x1530036b0`. Dump 1's next two frames, `ResMan+0x3820` and
  `ResMan+0x3877`, are both inside that function. **[L]** So dump 1's crash is the client
  releasing its **resource-manager singleton**, and the frames line up with the vtable
  three ways: the global, the slot, and the return addresses.

### The two crashes happen in different phases, and that is a result

Counting the same event in two logs, as `CLAUDE.md` asks **[L]**:

```
run 1  20:20:46.130  SOCKET ... closed and cleared by the client (FUN_1415e3b60 ran)
       20:20:46.740  session: object at 0x0 is now UNREADABLE
       20:20:46.782  CLIENT FAULT #1: code=0xc0000374
run 2  23:00:59.855  SESSION obj=0x5b2a4f8 ... (the ordinary 3 s watch cadence)
       23:01:02.331  CLIENT FAULT #1: code=0xc0000374        <- no socket teardown at all
```

Dump 1 died **652 ms after the client tore its own socket down**, in a routine that
releases global singletons — teardown. Dump 2 died mid-run with the session watch still
ticking on schedule and three threads started 40 s earlier. **[D]**

So the shared inner chain is not one call site that happens to be buggy; it is the
**common consumer** — releasing a PCOM property tree — reachable from at least two
different phases, and `research/heap-corruption.md`'s older sighting (a destructor
`FUN_1415a1860` via `FUN_1401be120`) is a third distinct route into it. **[D]**

That also disposes of a puzzle that file leaves open. Its capture diff eliminated session
length, control-ack volume, mob count, field entries and drops, and concluded *"nothing
separates the two groups."* Nothing would: whether a run dies does not depend on what
happened during play, it depends on whether a damaged slot is reached by a free before the
process ends. **[I]** — plausible and consistent with everything above, but not measured.

---

## 10. What a third dump would have to show

The dumps have now given everything a post-mortem can give about *this* structure. A third
one is only worth taking if it is aimed, and these are the questions it could still settle:

1. **Is the value ever anything but 1?** Five for five so far. A sixth `1` makes the
   "refcount or flag initialised to 1" reading very hard to avoid; a single different value
   breaks it and puts a wild write back on the table. **This is free** — the chunk-chain
   walk in this file runs on any dump and takes about a minute.
2. **Is it ever a size class other than `0x20`?** 0 of 249 496 so far. One outside bucket 1
   would say the writer targets an address rather than an object type.
3. **Does the count grow with session length?** 2 damaged at 596 s, 3 at 644 s. Two points
   are not a trend. Four or five dumps of differing lengths would give a rate, and a rate
   tells you whether the writer fires on a repeated event (a map load, a resource eviction)
   or a rare one.

**None of these needs a new instrument, and none needs page heap** — which, as
`heap-corruption-dump.md` establishes, cannot see inside this arena anyway.

What would actually name the writer is a **write watch on `body-4` of a slot known to be
damaged-prone**, which a post-mortem cannot provide. The one cheap approximation available
here: the hook already patches the client, and a 4-byte guard page or a `WATCH` on the
carve function could record which code path last touched a chunk before its header went
bad. That is a build, not a dump, and it belongs to whoever owns `crates/grap-stub`.

---

## 11. One cheap experiment, with its risk stated

Because §5 shows the corruption is **four bytes in a field whose low half is still correct**,
and §7 shows the client's free reads all 64 bits of that field with no validation, there is
a one-instruction client patch that should convert this crash into nothing:

```
14019b504   48 8B 07      mov rax, qword ptr [rdi]     ; current
            8B 07 90      mov eax, dword ptr [rdi]     ; proposed - zero-extends, 3 bytes
```

Those three bytes were read back out of the mapped image before being written down here —
`14019b500: 48 8d 79 f8 48 8b 07 …`, so `lea rdi,[rcx-8]` then the qword load at exactly
`0x14019b504`, and the replacement is the same length. **[L]**

The damaged block would then be recognised as the `0x20` slot it actually is and returned
to the pool free list, which is the correct outcome rather than a papering-over.

**The risk, stated plainly:** it discards the `not rax` path at `0x14019b50c`, which exists
to handle a one's-complement-encoded header. That path looks dead — **0 of 408 184**
enumerated slots in two dumps carry such a header **[L]** — but "not used in two runs" is
not "unreachable", and the blind spot is that the walk only covers the four buckets of the
global context at `0x143AD68A0`; a block from a *different* pool context freed through the
same function would not be in that count. **[I]**

It is also an **experiment, not just a mitigation**: if the client stops dying with
`0xC0000374` after this patch and the damaged-slot count in a dump keeps rising, the whole
chain in this file is confirmed end to end. If it dies anyway, something here is wrong.
That decision is the owner's, and it is a change to `client-patched/`, so it is not made here.

---

## Appendix: the scripts

Written for this pass and left in the session scratchpad rather than `tools/`, because each
is a few dozen lines around `dumpwalk`'s reader and none is a general instrument:
`poolchain.py` (the chunk-chain enumeration and the two identities), `wildscan.py` (the
byte sweep), `chunkverify.py` (the carve identity filter), `slotstate.py` (chunk age and
free-list membership), `neigh.py`, `dwordwrite.py`. The one worth promoting to `tools/` is
`poolchain.py` — it answers questions 1–3 of §10 on any future dump in about a minute, and
it self-checks against two counters the allocator maintains itself.
