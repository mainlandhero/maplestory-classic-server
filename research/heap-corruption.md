# `0xC0000374` — the heap corruption, four sightings and what to do next

> **SUPERSEDED as a status, 2026-09-08. Still accurate as the earliest sighting record.**
> Much of what this file calls undiagnosed has since been measured. Read in this order:
> `the-180-second-clock-2026-09-07.md` (the writer runs on an exact 180 s clock),
> `live-client-crash-2026-09-08.md` (the same damage on OTHER PEOPLE's machines, so it is the
> client's bug and not our environment's), `the-writer-damages-live-objects-2026-09-08.md` (the
> damage that KILLS is inside live objects, so a clean pool proves nothing), and
> `the-180-second-tick-family-2026-09-08.md` (five candidate writers, found by decompiling).
> There is now a mitigation that has run on a client: the guard-page quarantine, `-GuardPage`.
> **Do not take this file's "what to do next" - it is two weeks out of date.**

**Status when written: not diagnosed, and a post-mortem stack cannot diagnose it.** This file
exists so the next person does not spend a client run re-establishing what is already known, and
so the one instrument that *would* answer it is written down.

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
system setting.

**`gflags.exe` is NOT installed on this machine.** A recursive search of both Windows Kits
trees and the Debugging Tools directory finds nothing, so the usual `gflags /p /enable` line
cannot be pasted anywhere; it needs the SDK's Debugging Tools feature installed first. That
key is the whole of what gflags does, so write it directly. From an **elevated PowerShell**:

```powershell
$k = "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\MapleStory.exe"
New-Item -Path $k -Force | Out-Null
Set-ItemProperty -Path $k -Name GlobalFlag -Value 0x02000000 -Type DWord
Set-ItemProperty -Path $k -Name PageHeapFlags -Value 0x3 -Type DWord
```

and afterwards, to put it back — remove the two values rather than the key, so anything else
under it survives:

```powershell
$k = "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\MapleStory.exe"
Remove-ItemProperty -Path $k -Name GlobalFlag,PageHeapFlags -ErrorAction SilentlyContinue
```

`PageHeapFlags = 0x3` is full page heap and `0x1` is the light variant, which is far cheaper
and still catches some of it. The key matches on **file name**, so it picks up
`client-patched/MapleStory.exe` without the path being named anywhere.

Three things to expect. Full page heap makes the client **noticeably slower and much hungrier
for memory**, which on a map holding thirty mobs may itself be a problem. It may move the
crash rather than reproduce it, because changing allocation layout changes what an overrun
lands on — a run that does *not* crash under page heap is still evidence, pointing at a
use-after-free or a double free rather than an overrun. And this client is Themida-packed with
anti-cheat modules loaded (`mssecure`, `nexoncm`): page heap changes process behaviour enough
that it may refuse to start for reasons unrelated to the bug. If it does, that is worth a line
in `STATUS.md` rather than a second attempt.

**A quoted path is not a command in PowerShell.** The first version of this section opened
with `"C:\...\gflags.exe" /p /enable`, which PowerShell parses as a string expression and
rejects at the `/`. A native executable with spaces in its path needs the call operator:
`& "C:\path\app.exe" args`. That is already in `CLAUDE.md`'s shell notes and it still got
written down wrong here.

## Page heap was run, 2026-08-20, and the fault code CHANGED

The owner enabled full page heap by the registry route and played twice.

| run | seconds | outcome |
|---|---:|---|
| first | 364 | **no crash**, closed by hand |
| second | 450 | **`0xC0000421`** - `STATUS_ASSERTION_FAILURE` |

**Page heap was definitely active**: the client's working set went from ~380 MB to ~990 MB,
which is the signature of every allocation getting its own page.

**The code changed from `0xC0000374` to `0xC0000421`, and that matters.** The first is the
heap noticing damage at some later walk; the second is a verifier stop, raised where the rule
was broken. Page heap bit. Fixtures: `research/fixtures/pageheap-c0000421-*`.

**What we did not get is the message.** There is no `CLIENT FAULT` line for it - the last
thing in the hook log is an ordinary `0x03E4` dispatch 240 ms before the exit - because a
verifier stop does not travel through SEH the way an access violation does, so
`crates/grap-stub`'s handler never sees it. The verifier's diagnosis goes to the debugger
port, and with no debugger attached it is discarded.

### Getting the message: a dump, NOT an attach

WinDbg was installed on 2026-08-20 — the **Store build** (`WinDbgX`). There is still no
`cdb.exe`, which only the SDK build provides.

**Do not attach it to a running client.** `research/protection-surface.md` is explicit:
*"Runtime anti-debug. Expect debugger detection; attaching to a running client is a fight,
independent of GameGuard."* This client is Themida-packed with anti-cheat modules loaded, so
an attach is as likely to produce a fight over the debugger as an answer about the heap.

> ## CORRECTED 2026-08-22: WER never wrote a dump for this client, and could not
>
> **Everything from here to the end of this section describes a method that does not work on
> this executable.** It is kept for its working, not its verdict. What is true:
>
> * The registry recipe below is **correct and was correctly applied**. That was verified the
>   expensive way and then the cheap way: `Disabled = 0` on the WER root key at 12:21:01, and a
>   **decoy** built to do nothing but dereference null, named `MapleStory.exe` because
>   LocalDumps keys match on the base name, wrote a **9.4 MB dump into `dumps\`** at 15:26.
> * The real client raised the **same** exception code, `0xC0000005`, at **13:49:56 - 88
>   minutes after WER was switched on** - and produced nothing at all. Same machine, same hour,
>   same executable name, same exception. Every variable held but one.
> * **The client ships its own crash reporting** - `CrashReportClient.exe` sits beside it, and
>   it uploads its own error log and call stack in `0x008F`/`0x0090`. A process that handles
>   its own faults never reaches `WerFault`. No amount of configuring fixes that.
>
> **The inference below is wrong and is the part that would have cost someone a week.** This
> section says an absent dump *"means WER is not handling the exit, so something else is
> terminating the process - which would point at the anti-cheat rather than at the heap."*
> Nothing else was terminating the process. The exception was raised, went unhandled, and
> ended the process exactly as documented; WER simply never saw it. Chasing the anti-cheat on
> the strength of that sentence would have been chasing nothing.
>
> **What actually produces the dump now:** the hook writes it itself, from the vectored
> exception handler that was already catching the fault and only logging it -
> `crates/grap-stub/src/minidump.rs`. It is **first-chance**, so the dump is taken at the
> faulting instruction rather than after the client has unwound. On 2026-08-21 20:20 it
> produced **1 010 MB for a `0xC0000374`**, the first crash dump this project has ever had,
> written in 878 ms.
>
> **Page heap is still OFF**, so the `!heap -p -a` advice below cannot deliver the allocation
> and free stacks it promises. That is a real gap and it is the owner's to close - page heap is an
> IFEO setting. Until then `kb` and `!heap -s` are what carry the weight, and
> `tools/analyse-dump.ps1` says so.

**Take a crash dump and open it offline instead.** Windows Error Reporting writes the dump
after the process is already dying, and nothing is attached while it runs — so there is
nothing for the anti-debug to detect. Same information, none of the fight.

The owner enables it, because it is a system setting. Elevated PowerShell:

```powershell
$w = "HKLM:\SOFTWARE\Microsoft\Windows\Windows Error Reporting\LocalDumps\MapleStory.exe"
New-Item -Path $w -Force | Out-Null
Set-ItemProperty -Path $w -Name DumpFolder -Value "C:\MapleCW\dumps" -Type ExpandString
Set-ItemProperty -Path $w -Name DumpType -Value 2 -Type DWord
Set-ItemProperty -Path $w -Name DumpCount -Value 2 -Type DWord
```

`DumpType 2` is a full dump — with page heap on that is roughly a gigabyte each, hence
`DumpCount 2`. `dumps/` is gitignored. To undo:

```powershell
Remove-Item -Path "HKLM:\SOFTWARE\Microsoft\Windows\Windows Error Reporting\LocalDumps\MapleStory.exe" -Recurse
```

Then `tools/analyse-dump.ps1` opens the newest dump. It drives `cdb.exe` when the SDK build is
present and otherwise prints the commands to paste into the Store WinDbg by hand, because the
Store build has no scriptable console mode worth relying on.

The command that matters is **`!heap -p -a <faulting address>`**. For a page-heap block it
prints the **allocation stack and the free stack** — exactly what a post-mortem `0xC0000374`
can never give, and the whole reason page heap was turned on.

**Page heap must stay on for this run**, since the dump is only worth taking of a verifier
stop.

**If no dump appears, that is a finding rather than a failure of the method**: it means WER is
not handling the exit, so something else is terminating the process — which would point at the
anti-cheat rather than at the heap.

**Do not leave page heap on** while doing anything else: it costs ~600 MB and a lot of CPU,
and it changes the failure mode, so any other crash seen while it is enabled is not comparable
with the runs before it.

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
