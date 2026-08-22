# The first crash dump, and what it actually says

`dumps\maplecw-crash-1096760-c0000374-1.dmp` — 1,010 MB, full memory, written by the hook's
vectored exception handler (`crates/grap-stub/src/minidump.rs`) at the faulting instruction.
Read with `tools/dumpwalk.py`, which is new and is the only instrument used here.

```
python tools\dumpwalk.py dumps\maplecw-crash-1096760-c0000374-1.dmp
```

Every claim below is tagged **[L]** read out of the file, **[D]** derived from something
read, or **[I]** inferred.

---

## The short version

**This is not heap corruption.** The heap's metadata is intact — 156 consecutive block
headers walked from the segment's first entry, every checksum valid **[L]**. What happened
is that the client's own deallocator was handed a pointer **0x32ac0 bytes into the middle of
a live 264 KB block** and passed it to `RtlFreeHeap`, which refused it and raised
`STATUS_HEAP_CORRUPTION` because that is the status Windows raises for "this is not a
block". **[D]**

The reason it did that is a single stray DWORD. The block's 8-byte allocator size header
should have read `0x20`; it read `0x0000000100000020`. In the same 264 KB arena there are
**6,239** clean `0x20` headers and **exactly one** with a non-zero high dword — the one that
was freed. **[L]**

So the damage is real, but it is one 4-byte write into an allocator header, not a smashed
heap. And it is upstream of everything on this stack.

---

## Self-checks first

The parser is new, so it proves itself before it is believed. All six pass **[L]**:

| check | value |
|---|---|
| header magic | `MDMP` |
| stream directory | 12 streams |
| module list contains `MapleStory.exe` | `client-patched\MapleStory.exe` |
| its image base | `0x140000000` |
| process id | 1096760 |
| Memory64List covers the file | 1,058,357,248 of 1,058,811,810 bytes |

Two more controls, both of which could have failed and did not:

* The allocator's own captured backtrace ends in `kernel32!BaseThreadInitThunk` then
  `ntdll!RtlUserThreadStart` — the canonical bottom of a Windows thread. A wrong struct
  offset does not land on those two in that order. **[L]**
* The independently-written `.pdata` unwinder reproduces **29 of 29** frames of that
  captured stack, plus four deeper ones. Two readings of the same stack from two different
  data structures. **[L]**

Process was created at unix 1787357451 and the dump written at 1787358046 — **595 s alive**,
which matches the 596 s in the exit log. **[L]**

---

## The exception record

```
thread            1107924
code              0xC0000374   STATUS_HEAP_CORRUPTION
flags             0x00000001   EXCEPTION_NONCONTINUABLE
address           0x00007ffca83af509   ntdll.dll+0xff509
NumberParameters  1
   param[0]       0x00007ffca8419800   ntdll.dll+0x169800
```

**[L]** for all of it.

**There is exactly one parameter.** `EXCEPTION_RECORD` always carries fifteen slots and
`MiniDumpWriteDump` copies all fifteen; slots 1..14 in this file are stale stack bytes and
**must not be read as data**. `tools/dumpwalk.py` prints them but labels them `past[n]  <-
NOT a parameter`, because two of them (`0x152c34c14`, `0x152c34a6e`) are perfectly plausible
PCOM.dll code addresses and would be very easy to write up as evidence. They are not.

`param[0]` is a pointer into ntdll's writable data — `ntdll!RtlpHeapFailureInfo`, the
allocator's global failure record. **[I]** on the symbol name (no symbols on this machine),
but **[L]** that it is ntdll data and **[L]** that it decodes as a coherent
`_HEAP_FAILURE_INFORMATION`, which is what matters.

The register context corroborates the record independently: `Rbx = 0xC0000374`, `Rsi = 1`,
`Rdi = 0x7ffca8419800` — status code, parameter count, parameter, sitting in the argument
registers of a `RtlRaiseException` call. **[L]**

### The ntdll address is not the bug

`STATUS_HEAP_CORRUPTION` is raised at the next allocator walk, not where the damage was
done. `0x7ffca83af509` names ntdll because ntdll is what noticed. `research/heap-corruption.md`
already says this and it is repeated here because this dump makes it tempting to forget: the
ntdll address is genuinely precise, and genuinely says nothing.

---

## `_HEAP_FAILURE_INFORMATION` — the artefact that made this dump worth taking

Decoded at `param[0]` **[L]**:

```
Version        2
Length         0x6d0
Type           8
Heap           0x00000000005f0000      (the process default heap)
Address        0x000000003c218ac0
Param1/2/3     0 0 0
StackTrace[32] 29 slots used, innermost first
```

The struct offsets are **[I]** — no symbols — but two things in the data agree with them and
would not agree with a wrong guess: `StackTrace` is 32 pointers at `+0x58`, so it ends
exactly at `+0x158`, and the live run stops at `+0x138` with `+0x140..+0x150` zero; and the
last two live entries are the thread-bottom pair named above.

### Type 8 is not a guess — the constant is in the instruction stream

The published `_HEAP_FAILURE_TYPE` enum calls 8 `heap_failure_block_not_busy`, but that list
is a claim and this repo has been burned by exactly that shape of claim before. It was
checked against something that can disagree — ntdll's own code, read out of the dump's mapped
image with `tools\dumpwalk.py --dis`:

```asm
7ffca82d5bae  lea       r14, [rdx - 0x10]         ; entry = BaseAddress - 0x10
7ffca82d5bb2  prefetchw byte ptr [r14]
7ffca82d5bb6  cmp       byte ptr [r14 + 0xf], 5   ; UnusedBytes == 5 -> back up by SegmentOffset
7ffca82d5bbb  jne       7ffca82d5bc9
7ffca82d5bbd  movzx     eax, byte ptr [r14 + 0xe]
7ffca82d5bc2  shl       rax, 4
7ffca82d5bc6  sub       r14, rax
7ffca82d5bc9  test      byte ptr [r14 + 0xf], 0x3f  ; <-- THE CHECK THAT FAILED
7ffca82d5bce  jne       7ffca82d5b43                ; nonzero -> a real busy block, carry on
7ffca82d5bd4  xor       r9d, r9d
7ffca82d5bdc  mov       r8, r14                     ; arg3 = the entry address
7ffca82d5be4  mov       rdx, rsi                    ; arg2 = the heap
7ffca82d5be7  lea       ecx, [r9 + 8]               ; arg1 = 8   <-- the failure type, literal
7ffca82d5beb  call      7ffca83be17c                ; capture 32 frames, then raise
7ffca82d5bf0  mov       r14, rdi                    ; <-- frame #5 of the captured stack
```

**[L]** for the disassembly. Three things fall out:

* **The type constant really is 8**, loaded by `lea ecx,[r9+8]` with `r9` just zeroed. Not
  read off an enum list. **[L]**
* **The failing check is `test byte [entry+0xf], 0x3f`** — the `UnusedBytes` field. Zero
  there means "not an allocated block", which is what the name `block_not_busy` describes.
  The name and the code agree, so the name can now be used. **[D]**
* **`r14 = rdx - 0x10`**, and at the fault `R14 = 0x3c218ac0`, `R13 = 0x3c218ad0`. So
  `_HEAP_FAILURE_INFORMATION.Address` is the **entry**, and the pointer actually passed to
  `RtlFreeHeap` was **`0x3c218ad0`**. **[D]**

The byte the test read is measurable: `[0x3c218acf] = 0x00`. It is the high half of the
UTF-16 NUL that terminates the string `"Property"` sitting at that address. **[L]**

---

## The stack

Innermost first. This is the allocator's own capture, reproduced frame-for-frame by the
`.pdata` unwind. `=` marks an exact export match; a name in `(~ )` is only the nearest
preceding export and names nothing.

```
 #0  ntdll+0x10e1c1                                     ) the failure-report path
 #1  ntdll+0x25bf0                                      )
 #2  ntdll+0x247b1   = ntdll!RtlFreeHeap+0x51           <- exact
 #3  MapleStory.exe+0x19b58e   fn 0x14019b4e0           <- the client's free()
 #4  PCOM.dll+0x12d16          fn 0x152c12ce0           <- refcounted release
 #5  PCOM.dll+0x17078
 #6  PCOM.dll+0x16a1e
 #7  PCOM.dll+0x19c2e
 #8  PCOM.dll+0x16ec0
 #9  PCOM.dll+0x199cc
 #10 oleaut32+0x9c713 = oleaut32!VariantClear+0x193     <- exact
 #11 NAMESPACE.DLL+0x1009cf
 #12 NAMESPACE.DLL+0x100bbb
 #13 NAMESPACE.DLL+0x10149c
 #14 PCOM.dll+0x34bb9
 #15 PCOM.dll+0x34a3f
 #16 PCOM.dll+0xb405
 #17 PCOM.dll+0xbb6b
 #18 PCOM.dll+0x9f1c
 #19 oleaut32!VariantClear+0x193                        <- exact, second time
 #20 ResMan.dll+0xe637
 #21 ResMan.dll+0x118cd
 #22 ResMan.dll+0x3877
 #23 ResMan.dll+0x3820
 #24 MapleStory.exe+0x2c4b785  fn 0x142c4b730
 #25 MapleStory.exe+0x2c434c4  fn 0x142c42f30
 #26 MapleStory.exe+0x2ef4aea  fn 0x142ef49e4           <- the thread's start routine
 #27 kernel32!BaseThreadInitThunk+0x14
 #28 ntdll!RtlUserThreadStart+0x21
```

**[L]** for every address; **[L]** for the four exact export names; **[D]** for
`fn 0x142ef49e4` being the thread start routine (it is called directly by
`BaseThreadInitThunk`).

### Confidence in the client frames

| frame | confidence | why |
|---|---|---|
| `#3` `MapleStory.exe` `FUN_14019b4e0+0xae` | **certain** | already named in `research/equip-crash.md` as the deallocator that faulted; the `call rbx` at `0x14019b58c` returns to exactly `0x14019b58e`. Both the unwind and the captured stack put it here **[L]** |
| `#24` `fn 0x142c4b730+0x55` | **certain frame, unknown function** | present in both readings; no name for it anywhere in `research/` |
| `#25` `fn 0x142c42f30+0x594` | **certain frame, unknown function** | same |
| `#26` `fn 0x142ef49e4+0x106` | **certain frame**, and it is a thread entry point **[D]** | called by `BaseThreadInitThunk` |

**Noise count.** The heuristic stack scan finds 93 code-looking values in the thread's 6,032
bytes of live stack; 34 of them are frames the unwinder also produced. **The other 59 are
leads at best** — spilled function pointers, callee-saved copies, and return addresses from
calls that already returned. The scan is printed because it is occasionally the only trace of
a returned frame, not because it is a stack. Two specific traps in this one: `user32!PeekMessageA`
and `ntdll!RtlAllocateHeap` both appear, entirely stale, inside the 4 KB frame of the thread
start routine; and values like `0x143dea15c` resolve to a "function" spanning 0x6315c bytes,
which is a Themida region and not a function at all — `dumpwalk.py` labels those.

---

## What was actually freed

`RtlFreeHeap(heap = 0x5f0000, flags = 0, base = 0x3c218ad0)`. **[D]** from
`r14 = rdx - 0x10` above.

The 40 bytes at `0x3c218ad0` **[L]**:

```
3c218ad0  20 00 00 00 01 00 00 00   <- 8-byte allocator size header. Should be 0x20.
3c218ad8  0c 00 00 00               <- length prefix, 12 bytes
3c218adc  4e 00 6f 00 43 00 72 00 69 00 31 00 00 00  "NoCri1"
```

Its neighbours, at a strict 40-byte stride, are the same shape — a `0x20` size header, then
either a length-prefixed UTF-16 string or a free-list pointer. Strings recovered nearby
include `Property`, `NoCri1`, `.img`, `312000.img`, `atusBar.img`, `aracter/`, `mOverHair`,
`rOverHead`, `pressed`, `e_move`. **[L]** This is the WZ resource/property string pool,
which is consistent with `ResMan.dll` and `NAMESPACE.DLL` being on the stack. **[I]**

### The arithmetic, end to end

`PCOM.dll+0x12ce0` is a refcounted release **[L]**:

```asm
152c12ced  mov      rbx, [rcx]              ; obj
152c12cfa  lock xadd dword ptr [rbx+0x10], eax   ; eax = -1, refcount--
152c12cff  cmp      eax, 1                  ; was it the last reference?
152c12d04  mov      rcx, [rbx]              ; str = obj->[0]
152c12d0c  add      rcx, -4                 ; back up over the length prefix
152c12d10  call     qword ptr [rip+0xc92ca] ; -> MapleStory.exe FUN_14019b4e0 (free)
152c12d16                                   ; <- frame #4
```

`MapleStory.exe FUN_14019b4e0` is the client's `free` **[L]**:

```asm
14019b500  lea   rdi, [rcx - 8]        ; base = ptr - 8
14019b504  mov   rax, [rdi]            ; size = the 8-byte header  <-- read 0x100000020
14019b50f  cmp   rax, 0x20 / 0x40 / 0x80 ... ; pick a lookaside bucket
14019b570  cmovbe ecx, edx             ; > 0x80 -> ecx = -1, leave the image
14019b577  mov   rbx, [rip+0x3939fb2]  ; DAT_143ad5530
14019b57e  call  qword ptr [rip+0x3939fb4] ; DAT_143ad5538
14019b58c  call  rbx                   ; (handle, 0, ptr-8)
14019b58e  jmp   ...                   ; <- frame #3
```

So, working backwards from the measured `base = 0x3c218ad0` **[D]**:

| step | value |
|---|---|
| the BSTR-shaped string pointer PCOM held | `0x3c218adc` |
| PCOM's `add rcx,-4` (length prefix) | `0x3c218ad8` |
| the client `free`'s `lea rdi,[rcx-8]` (size header) | `0x3c218ad0` |
| `RtlFreeHeap`'s `lea r14,[rdx-0x10]` (heap entry) | `0x3c218ac0` = the reported `Address` |

Every subtraction lands where the data says it should. `0x3c218adc` is exactly the character
data of the `"NoCri1"` string, and `0x3c218ad8` exactly its `0x0c` length prefix. **[D]**
Nothing in that chain is wrong: PCOM's `-4` is right, the client `free`'s `-8` is right, and
`RtlFreeHeap`'s `-0x10` is right.

### The one thing that is wrong

`FUN_14019b4e0` read the size header as a **qword** and got `0x0000000100000020`, which is
larger than `0x80`, so it took the `RtlFreeHeap` path instead of returning the block to the
`0x20` lookaside bucket. Had the header read `0x20`, the block would never have left the
image and nothing would have crashed. **[D]**

Is that high `1` anomalous, or a legitimate field? Measured, over the whole 264 KB arena
**[L]**:

* `0x0000000000000020` — clean size header: **6,239** occurrences
* `0x0000000100000020` — **1** occurrence, at `0x3c218ad0`, the block that was freed
* any other value in the high dword above a `0x20` low dword: **0**

The matching allocator `FUN_14019b780(ctx, size)` buckets on `size <= 0x10 / 0x20 / 0x40 /
0x80` and never allocates a pooled block claiming more **[L]**, so a header above `0x80` on a
32-byte slot cannot be produced by the allocator itself. **[D]**

**[I], and this is the load-bearing inference:** a stray 4-byte write of `1` landed at
`0x3c218ad4`. That is 4 bytes before where a length prefix belongs and 4 bytes after where a
size header begins — the offset you get if something writes a `{DWORD, DWORD}` header where a
single `qword` header is expected, or an in-use/refcount flag at `base+4`.

### The heap itself was never damaged

Walking the containing segment's block chain from its `FirstEntry` **[L]**:

```
156 blocks stepped, every header checksum valid.
0x3c218ac0 lies INSIDE the block whose entry is 0x3c1e6000:
  0x3c1e6000 .. 0x3c228010, 270352 bytes, flags 0x9 (BUSY=1), unused 0x20
  user data 0x3c1e6010 .. 0x3c227ff0 (270304 bytes); the address is +0x32ab0 into it.
```

The walk verifies itself: each step decodes the entry with heap `0x5f0000`'s XOR key
(`0xe22a22bc11f7`, read from `Heap+0x88`), checks the standard `b0^b1^b2 == b3` header
checksum, and steps by the entry's own size. A wrong key or a wrong offset fails on the first
block, not the 156th.

So the pool is one large busy heap allocation, the heap's own bookkeeping around it is
correct, and the address that ntdll rejected is interior to it. **[D]**

**This retires the standing description of this bug.** `research/heap-corruption.md` says *"a
destructor freed a block whose header was already damaged"*, and adds that "what damaged the
block is not [known]". Half of that is now measured — a header was damaged and a release path
freed it — but the header was **the client allocator's**, not the Windows heap's, and the
Windows heap was never corrupt. **[D]** That distinction changes what page heap will and will
not catch; see below.

---

## Two things this dump settles that were previously inferred

**1. `DAT_143ad5530` and `DAT_143ad5538` are `HeapFree` and `GetProcessHeap`.**
`research/equip-crash.md` calls this **[D]** *"from the argument shape `(handle, 0, block)`"*
and notes both are uninitialised `.data` resolved by the packer, so they "cannot be named
statically". They can be named dynamically. Read from the live image **[L]**:

```
DAT_143ad5530 = 0x00007ffca64858b0  = kernel32!HeapFree
DAT_143ad5538 = 0x00007ffca6485ef0  = kernel32!GetProcessHeap
```

The inference was correct. It is now a measurement.

**2. `FUN_14019b4e0`'s ntdll path does not mean the block was large.**
`equip-crash.md` states, correctly, that the branch is taken only when *"the block header at
`ptr-8` says the block exceeds `0x80` bytes"* — and then reasons about which real allocations
are big enough to qualify (17-plus hash buckets, and so on). This dump shows a **32-byte**
block taking that branch, because the header lied. **[L]** Any argument of the form "only
blocks over 0x80 reach ntdll, therefore the freed object was one of these large ones" is
unsound. That reasoning is used in `equip-crash.md` §6.1 and should be revisited by whoever
owns that file — I have not edited it.

---

## An instrument correction that affects other work

`tools/pdata_lookup.py` locates the function table by **section name** (`.pdata`) and finds
120,981 entries — the figure `CLAUDE.md` quotes. The PE's **exception data directory**, which
is what the CPU and `RtlLookupFunctionEntry` actually use, points at RVA `0x4fe3010` instead,
inside `.themida`, a section with `SizeOfRawData = 0`. **[L]**

Compared against the table the running image used **[L]**:

* all **120,981** on-disk entries are present, byte-identical, in the live table
* the live table has **4,399** more, all with begin-RVAs in `0x3d87000 .. 0x4fe283d` — the
  Themida region

So `pdata_lookup.py` is **correct for client code and blind to `.themida`**, which is the
blind spot `research/damage-formula.md`, `research/item-drop.md` and
`research/mob-collector-callsites.md` already name. Nothing needs fixing; it is worth knowing
that the on-disk table is not the one the CPU used, and that a dump can read the one that was.
`tools/dumpwalk.py --dis` disassembles from mapped memory, so it can read `.themida` code that
has no file bytes at all.

---

## Page heap: what it would have added, and what it would not

**Page heap was off.** Not assumed — measured from the dump: all 13 process heaps have
`ForceFlags = 0` and signature `0xFFEEFFEE` (ordinary NT heaps, no verifier). **[L]** The
working-set figures agree: 339.8 MB at the fault, 497.8 MB peak, against the ~990 MB
`research/heap-corruption.md` records for a page-heap run. **[L]**

(The first draft of this section said "peak working set 339.8 MB". That was the *current*
working set: `parse_vm_counters` had the `MINIDUMP_PROCESS_VM_COUNTERS_2` field offsets
shifted by one field and was reporting a quota counter as the working set. Fixed, and the
tool now prints a layout check that fails loudly if the offsets are wrong again. The
conclusion did not depend on it — `ForceFlags = 0` is the real evidence — but the number was
wrong and had been written down.)

Consequently there are **no allocation or free stacks anywhere in this file**. That is a
property of the configuration and says nothing about the client. With page heap on, every
block would carry the stack that allocated it and, for a freed block, the stack that freed it.

But be precise about what that would buy **here**, because it is less than it looks:

* Page heap instruments the **Windows** heap. The block that was damaged is a 32-byte slot
  inside a 264 KB client-allocator arena. Page heap does not see individual slots inside that
  arena at all, and would not guard the 4 bytes at `0x3c218ad4`. **[D]**
* It would still have helped indirectly: with full page heap the 264 KB arena gets its own
  pages, and the earlier run under page heap changed the fault code to `0xC0000421` — a
  verifier stop at the rule violation, which is a different and better vantage point.

The honest reading is that **page heap is the wrong instrument for this particular write**,
and that is a new statement — the previous conclusion (`heap-corruption.md`, "it is not the
cheapest instrument, it is the only one left that discriminates") was reasonable on the
evidence available then, which did not include a dump.

---

## What to look at next, in order of cost

1. **Find who writes `1` at `slot+4`.** The target is 4 bytes into the client allocator's
   8-byte size header. A data breakpoint is the direct instrument and needs a debugger, which
   is off the table here. The cheap static version: sweep `PCOM.dll`, `NAMESPACE.DLL` and
   `ResMan.dll` for a `mov dword ptr [reg+4], 1` (or `mov [reg], 1` on a pointer that is
   `base+4`) near anything that also touches a `+0x10` refcount, since `PCOM.dll+0x12ce0`
   shows the object model keeps its refcount at `obj+0x10` and its string at `obj+0`. A
   structure whose *own* header is `{DWORD size; DWORD refs;}` laid over a block from an
   allocator whose header is one qword is exactly the shape that produces this. These three
   DLLs have export tables, are not Themida-packed, and can be read statically — unlike the
   exe.

2. **`tools/dumpwalk.py --dis` the two unnamed client frames**, `fn 0x142c4b730` and
   `fn 0x142c42f30`. They are the client's entry into `ResMan.dll`. Naming what resource
   operation was in flight would say whether this is a load, an unload, or a cache eviction —
   and `research/` has no name for either address today.

3. **Check the thread.** `fn 0x142ef49e4` is a worker thread's start routine, not the UI
   thread (that is thread 1110024, parked in `win32u!NtUserGetMessage`) and not obviously the
   packet thread. Whether this arena is touched from more than one thread is worth knowing,
   because a torn `{DWORD,DWORD}` write from a second thread would explain a stray `1` with no
   bug in either writer.

4. **Only then, another dump.** This one is complete for what it is. A second dump of the same
   fault would show whether the damaged header is always `+4` and always `1`, which would
   separate "one specific bad struct layout" from "a racing write". That costs one client run
   and needs nothing configured — the hook already writes the dump.

**What NOT to do:** do not re-read the ntdll address, do not treat `param[1..14]` as data, and
do not re-enable page heap for this specific write. The first two are already-made mistakes in
this file's ancestry; the third is a new one this dump makes available.
