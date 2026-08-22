# The third dump, which §10 of `heap-wild-write.md` asked for in advance

`dumps\maplecw-crash-1224132-c0000374-1.dmp`, 1 442 MB, **306 s**, failure type **9**.
Written 2026-08-22 08:26 by the hook's own vectored handler, from the run that ended when
The owner selected the character `GoodTest`.

The value of this dump is that the questions were **written down before it existed**.
`research/heap-wild-write.md` §10 lists three, all answerable by re-running the existing
instruments with no new code. All three are answered below. Nothing here edits that file.

Tags: **[L]** read out of a file, **[D]** derived from something read, **[I]** inferred.

---

## The short version

* **Q1 - is the value ever anything but 1?** No. **Six for six.** `[L]`
* **Q2 - is it ever a size class other than `0x20`?** No. **Six for six in bucket 1**, against
  **0 of 360 216** slots enumerated in the `0x10`, `0x40` and `0x80` classes across three
  dumps. `[L]`
* **Q3 - does the count grow with session length?** It tracks it, at roughly **one damaged
  slot per 250 s**:

  | dump | age | damaged | s/damage |
  |---|---:|---:|---:|
  | 1 | 596 s | 2 | 298 |
  | 2 | 644 s | 3 | 215 |
  | 3 | **306 s** | **1** | 306 |

  Three points on a line through the origin is a **rate**, and §10 says what a rate buys: the
  writer fires on a **repeated** event, not a rare one. `[D]`

And one thing §10 did not ask for, because nobody expected it: **this dump has a much shorter
fuse than the other two, and it names the free that lit it.**

---

## 1. The instruments, and that they were re-run rather than remembered

Both are in `tools/`, both were built for the first two dumps, and both were run against this
file with the repo as the working directory:

```
python tools/dumpwalk.py  dumps\maplecw-crash-1224132-c0000374-1.dmp
python tools/poolchain.py dumps\maplecw-crash-1224132-c0000374-1.dmp
```

`dumpwalk`'s six self-checks all **PASS** (header magic, stream directory, module list, image
base `0x140000000`, pid 1224132, `Memory64List` covering the file). Its `.pdata` unwind of the
faulting thread matches the allocator's own captured stack **25 of 25 frames** - two
independent readings of one stack, and the captured one cannot be wrong. `[L]`

`poolchain` walked all four buckets from the pool context at `0x143AD68A0` and every chunk
passed its per-chunk size identity: **873 + 2 273 + 2 186 + 2 484 = 7 816 chunks, 0 failed**.
The allocator's own carved-slot counters equal the number of slots walked in all four buckets.
**183 456 slots enumerated in this process alone.** `[L]`

---

## 2. Q1 and Q2: the same value, the same class, a sixth time

```
bucket 1: slot 0x20, 32 slots/chunk
  chunk chain: 2273 chunks passed the size identity, 0 failed it
  72736 slots enumerated, 1 with a non-zero high dword in the header
    0x39991e68  hdr 0x0000000100000020  chunk 0x39991988  slot 31/32  va%16=8
```

`[L]`. Buckets 0, 2 and 3 enumerated 55 872 + 34 976 + 19 872 = **110 720 slots, 0 damaged**.

And the one damaged slot is **the pointer `RtlFreeHeap` was given**:
`_HEAP_FAILURE_INFORMATION.Address` = `0x39991e68`, byte for byte. `[L]` Type 9 puts the
caller's pointer in that field, so the two readings are of the same object from opposite ends
- the allocator's bookkeeping and the exception record - and they agree.

`va%16=8` predicts failure type **9** by §4 of `heap-wild-write.md` (the ntdll branch is
`test r13b,0xf`, and the pool's 40-byte stride alternates 0 and 8 mod 16). It is type 9. That
is a **prediction made before this dump and confirmed by it.** `[D]`

### The trap this dump walks straight into, and the tool now refuses

The bytes at the failure address read:

```
39991e68  20 00 00 00 01 00 00 00 14 00 00 00 66 00 69 00   ..........f.i.
39991e78  65 00 6c 00 64 00 4c 00 69 00 6d 00 69 00 74 00  e.l.d.L.i.m.i.t.
```

`0x14` = 20 bytes = 10 UTF-16 characters = **`"fieldLimit"`**, a `Map.wz` `info` property
name, and the slot above it holds `"grassySoil"`. Read by eye, `20 00 00 00 01 00 00 00` looks
exactly like `{u32 size, u32 refcount}` of a perfectly ordinary string cell - which is the
same reasoning that once decoded a `"Property"` BSTR as a heap header and retracted a correct
finding on the strength of it (`CLAUDE.md`, "A field whose meaning depends on a type byte").

**The enumeration is what settles it.** One slot in 72 736 has that high dword. It is not the
shape of a healthy cell; it is the only one of its kind in the bucket. `dumpwalk.py` now
refuses the `_HEAP_ENTRY` decode outright for a type-9 failure and prints why, which is how
the eyeball reading got caught this time instead of being written down.

**And by §6 of `heap-wild-write.md`, `"fieldLimit"` says nothing about the writer.** The header
is written at carve and never again, so a slot may have been damaged tens of thousands of
allocations before whatever occupies it now. The contents are a **dead end by construction**;
they are quoted here only because they are the trap.

---

## 3. Q3: a rate, and what it rules out

One damaged slot at 306 s, two at 596 s, three at 644 s. Whatever writes the `1` fires on
something that happens **continuously during play**, at order 10^-2 Hz - not on a one-off like
a login, a first map load or a first mob spawn, all of which had happened long before 306 s in
every one of the three runs. `[D]`

It is also **not** driven by the crash: the process died with 1, 2 and 3 damaged slots
respectively, so the count is not a threshold. What kills the process is a **free of a damaged
slot**, and how soon that happens after the damage is chance.

---

## 4. What is new: the fuse, and the thing that is NOT established

This is the first of the three deaths where the trigger is visible in the logs.

| | dump 1 | dump 2 | **dump 3** |
|---|---|---|---|
| client died at | 00:20:46 | 03:01:04 | **12:26:45.45** |
| last `0x01A0` SetField before it | 00:19:05 | 02:58:59 | **12:26:45.12** |
| gap | 101 s | 125 s | **0.33 s** |

`[L]`, from `previous-runs/world-20260821-202046.log`, `world-20260821-230104.log` and
`world.log` respectively. The hook log pins the last one to the packet rather than the clock:

```
08:26:45.116  4636 opcode=0x0011 elapsed_us=1604.5      <- migrate, returned
08:26:45.125  WATCH #69..#72 ... while dispatching opcode 0x01A0
08:26:45.450  ***** CLIENT FAULT #1: code=0xc0000374 *****
```

**There is no dispatch line for `0x01A0`.** The hook writes one on handler *return*, so the
client entered the SetField handler and never came back - the same reading that identified the
equip crash. `[L]`

All three stacks are the same shape: `RtlFreeHeap` <- `MapleStory.exe+0x19b58e` (the client's
free) <- **PCOM.dll** <- `oleaut32!VariantClear` <- **ResMan.dll** <- the client's field code.
It is the WZ property-release path in all three. `[L]`

### What this does **not** establish, and it matters

The owner's report was *"I tried to log into GoodTest character and it immediately crashed."* That
is one observation, and the obvious reading - **map 10 is fatal** - is not the only one:

* `GoodTest` sits on map **10**, "Mushroom Town". The map is real: it is in `fields.txt` and
  has 94 foothold rows in `footholds.txt`. `[L]` No client has ever loaded it, though -
  `GoodTest` was on map 1 and map 30 in earlier runs, both of which loaded. `[L]`
* But the client was already **306 s old and carrying a damaged slot** when it entered. A
  character-select round trip drops one map's resources wholesale and loads another's, which
  is the largest release event in a session. A damaged slot that was going to be freed
  eventually would be freed *there*. `[I]`

Those two predict different things and the difference is one login:

> **Log in as `GoodTest` first, immediately after launch.** If it loads, map 10 is not the
> cause and the crash is the accumulated damage reaching a big release. If it dies again at
> ~40 s of client life, map 10 is the cause and `poolchain.py` on that dump should show **0 or
> 1** damaged slots rather than the one-per-250-s the rate predicts.

The dump answers this on its own afterwards, from `client-exit.log`'s age line and
`poolchain`'s count. No second variable is needed.

---

## 5. The patch from §11 is now built, and it is off by default

`crates/grap-stub/src/heapfix.rs`. `-HeapFix` on `tools/test-server.ps1` appends `heapfix=on`
to the session marker; without it nothing is written and the client behaves exactly as it
always has. Nothing in `client-patched/` changes on disk - the three bytes are written to the
mapped image at run time, like the create-character flag, and die with the process.

```
14019b504   48 8B 07   mov rax, qword ptr [rdi]     ->   8B 07 90   mov eax, dword ptr [rdi]
```

The on-disk bytes at that RVA were re-read from `client-patched\MapleStory.exe` for this pass
and are `48 8d 79 f8 | 48 8b 07 | 48 85 c0 | 79 03 | 48 f7 d0 | 48 83 f8 20`, matching
`heap-wild-write.md` §7 exactly. `[L]` The patch reads the three bytes before writing and
refuses if they are not `48 8b 07`, then reads them back afterwards; both outcomes are logged.

**It is an experiment with a stated risk** (§11: it discards the `not rax` one's-complement
path, unused in 481 000 enumerated slots but only across the four buckets of one pool
context), and it is a prediction that can come back false: if the client still dies with
`0xC0000374`, the chain in `heap-wild-write.md` is wrong somewhere and this says which half.

---

## 6. What a fourth dump would still be worth

Only two things, and both are cheap:

1. **A dump from a `-HeapFix` run that crashed anyway.** That is the falsification, and it is
   worth more than any number of confirming ones.
2. **A dump from a run that was killed deliberately at a known age** - not a crash. The rate
   in §3 rests on three points, all of which are deaths, and a death is a biased sample: it
   requires a damaged slot to have been freed. A clean sample at, say, 900 s would say whether
   the rate is really linear or whether the deaths are selecting the fast tail.

Everything else a post-mortem can answer about this structure has been answered three times.
The writer will not be found this way - `heap-wild-write.md` §8 names the blind spot that
makes a static sweep for it impossible - and §10's remaining lead, a write watch on `body-4`
of a carve-prone chunk, is a build in `crates/grap-stub`, not a dump.
