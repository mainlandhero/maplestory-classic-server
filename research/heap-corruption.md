# `0xC0000374` — the heap corruption, four sightings and what to do next

**Status: not diagnosed, and a post-mortem stack cannot diagnose it.** This file exists so
the next person does not spend a client run re-establishing what is already known, and so the
one instrument that *would* answer it is written down.

## What it is

`STATUS_HEAP_CORRUPTION`. The Windows heap found a damaged block header and killed the
process. **It is raised at the next allocator walk, not where the damage happened**, so every
address in the fault line names the allocator or whoever happened to free next — never the
culprit. `tools/exit-forensics.ps1` says this in the exit log, and that line was added after
the code was twice read as an ordinary application exit.

## The sightings

| when | where | lifetime | what the capture has |
|---|---|---|---|
| 2026-08-19 | map 1013, Amherst | 193.5 s | exit code only — no in-process fault line, from the probe's old arming gap |
| earlier | map 20001075 | — | exit code only. Named in `research/npc-shop-crash.md`, which established both maps are **not** shop maps |
| 2026-08-20 22:24 | map 40, ~30 mobs | 389 s | fault line, thin stack: one code frame in a heap helper |
| 2026-08-20 22:46 | map 40, 5 field entries | 211.9 s | fault line, **five code frames** |

Fixtures: `research/fixtures/amherst-1013-heap-corruption-*` and
`research/fixtures/heap-corruption-2-map40-30mobs-*`.

**It predates every packet added on 2026-08-20.** The Amherst death is the day before, so
neither the scrolling banner (`0x00AC`, never sent in any of these runs — checked), nor the
rate multipliers, nor the per-class level gains can be the cause.

## What the best stack says, and what it does not

The 22:46 fault, at `0x7ffca83af509` in `ntdll.dll`:

```text
0x144fe3010(vm)  0x14374e6ec(?)  0x143ad68a0(?)
0x14019bbf3<-TEXT  0x140205833<-TEXT  0x1401be180<-TEXT
0x1415a1878<-TEXT  0x1415a1142<-TEXT
```

| frame | function | what it is |
|---|---|---|
| `0x14019bbf3` | `FUN_14019bb6a`, 242 bytes | the heap walk that noticed |
| `0x140205833` | `FUN_140205820`, 25 bytes | a thunk |
| `0x1401be180` | `FUN_1401be120`, 114 bytes | a **general** cleanup helper — called from the attack builder, the item loader and the classic shop alike, so it says nothing about the caller |
| `0x1415a1878` | `FUN_1415a1860`, 101 bytes | **a destructor.** It frees the member at `this+0x28`, installs a second vtable at `this+0x20`, then conditionally calls `operator delete` on the `dil & 1` flag — the scalar-deleting-destructor shape |
| `0x1415a1142` | `FUN_14159d240` +0x3f02, 17091 bytes | its caller |

`0x143ad68a0` is the heap handle this server's own decompilations keep seeing as the first
argument to the client's allocator (`FUN_14019b780(&DAT_143ad68a0, …)`), which is consistent
with a free rather than informative about it.

**The vtable installed at `this+0x20` has no RTTI**: the qword at `vtable-8` reads as UTF-16
string bytes, not a CompleteObjectLocator, so the class cannot be named this way. That is a
dead end rather than a mystery — it is a secondary vtable, not the object's primary one.

So the honest summary: **a destructor freed a block whose header was already damaged.** Which
destructor is known; what damaged the block is not, and no amount of staring at this stack
will produce it.

## The instrument that would answer it

**Windows page heap.** It puts each allocation on its own page with a guard page after it, so
an overrun faults **at the instruction that writes**, not at the next free. That converts this
from an undiagnosable post-mortem into a precise stack.

The owner runs this, not an agent — it writes an Image File Execution Options key, which is a
system setting. From an **elevated** prompt:

```
"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\gflags.exe" /p /enable MapleStory.exe /full
```

and afterwards, to put it back:

```
"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\gflags.exe" /p /disable MapleStory.exe
```

Two things to expect. Full page heap makes the client **noticeably slower and much hungrier
for memory**, which on a map holding thirty mobs may itself be a problem — `/p /enable
MapleStory.exe` without `/full` is the lighter variant if it is. And it may move the crash
rather than reproduce it, because changing allocation layout changes what an overrun lands on.
A run that does *not* crash under page heap is still evidence: it points at a use-after-free
or a double free rather than an overrun.

If gflags is not installed, the same key can be set by hand under
`HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\MapleStory.exe`
with `GlobalFlag` = `0x02000000`, but gflags is the supported route.

## The capture diff, done 2026-08-20 - and it eliminated six things

The comparison to make is **crashing runs against surviving ones**, not crash against crash.
`previous-runs/` archives every channel log, so session length and packet counts are an
unbiased sample - unlike the exit logs, which only exist for runs somebody chose to keep as a
fixture, and are therefore biased towards interesting deaths. That bias nearly produced a
confident wrong answer here: on the fixtures alone it looks as though no session has ever
survived past 190 s, and the archive says otherwise immediately.

| run | seconds | mob spawns | ctrl acks | field entries | drops | died |
|---|---:|---:|---:|---:|---:|---|
| 2026-08-20 01:26 | 476 | 60 | 3120 | 6 | 0 | no |
| 2026-08-20 14:42 | 429 | 30 | **10499** | 1 | 7 | no |
| 2026-08-19 22:27 | 257 | 34 | 1362 | 9 | 0 | no |
| 2026-08-19 22:08 | 237 | 30 | 0 | - | 0 | no |
| 2026-08-19 22:31 | 194 | 52 | 955 | 8 | 0 | **yes** |
| 2026-08-20 18:18 | 482 | 53 | 12723 | 1 | 38 | **yes** |
| 2026-08-20 22:24 | 390 | 38 | 10317 | 1 | 14 | **yes** |
| 2026-08-20 22:46 | 213 | 133 | 5316 | 5 | 19 | **yes** |

**Nothing separates the two groups.**

* **Not session length.** 476 s and 429 s survived; a 30 s run elsewhere in the archive died.
* **Not control-ack volume.** The *survivor* at 429 s sent 10 499 of them, more than a run
  that died at 390 s.
* **Not mob count.** 60 spawns survived, 38 died.
* **Not field entries.** 9 survived, 1 died.
* **Not drops or combat at all.** The Amherst death had zero drops and no combat packets of
  any kind, and two survivors had none either.
* **Not any packet added on 2026-08-20.** `0x00AC` has never been sent in any run, crashing
  or not.

So the cause is not visible in packet aggregates, which is what a lifetime bug - a use after
free, a double free, an overrun into a neighbour - looks like from the outside: it depends on
what happened to be next to what, not on how much of it there was.

**That is the argument for page heap.** It is not the cheapest instrument, it is the only one
left that discriminates.

There is one loose thread: the 482 s death of 2026-08-20 18:18 has no exit log, because the
launcher archives the channel logs and not `client-exit.log`. Whether that one was heap
corruption or something else is unknown, and archiving the exit log alongside the others would
close the gap for free.
