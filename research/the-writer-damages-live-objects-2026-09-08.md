# The overnight run: the pool was clean, the sentry worked, and the client died anyway

The owner, 2026-09-08: *"Well, the client didn't last the entire night."*

It lasted **8 minutes 13 seconds** (01:33:13 → 01:41:26). This file is what its crash dump says,
and it moves the whole investigation, because for the first time the damage that **killed** the
client was captured rather than inferred.

Tags: **[L]** read off the dump or the listing, **[D]** derived, **[I]** inferred.

---

## 0. First, the instrument: the guard page was not in this run

`client-patched/maplecw-hook.session` reads `mode=2,create=on` [L] — **no `guardpage=` token**,
so `guardpage::install()` returned silently and never logged a line. There is no
`maplecw-hook.session.pin` either, so `-PinPatches` was not passed. The sentry flags *were*
(`adaptive armed (2000 ms)` and `REPAIR IS ON` are in the log), so the run was the sentry alone.

The installed `grap64.dll` is the current build and does contain the module (13 occurrences of
`GUARD PAGE`, 2 of `guardpage=`, against `POOL SENTRY ARMED` as a positive control) [L]. So the
DLL was fine and the flag simply was not given.

*A note on how that was nearly mis-diagnosed.* The first check used `strings`, which returned
**zero lines for the whole file** — the tool is not working in this environment. Every probe came
back 0, including `POOL SENTRY ARMED`, which is certainly present. Without a positive control
that would have read as "the DLL does not contain the guard page" and sent the next hour into a
build problem that does not exist. `CLAUDE.md`, again, and it nearly cost an hour.

**And it would not have mattered anyway** — §4.

## 1. The fault

```
01:41:24.698  CLIENT FAULT #1: code=0xc0000005 at 0x1407b2ae1
```

`0x1407b2ae1` is inside `FUN_1407b2910` [L], and the shape is unmistakable — a **red-black tree
descent**, the MSVC `_Tree` node layout with `_Left` at `+0`, `_Parent` at `+8`, `_Right` at
`+0x10`, `_Isnil` at `+0x19` and a `u32` key at `+0x1c`:

```
1407b2ad0  cmp  dword ptr [rax + 0x1c], edi   ; compare the key
1407b2ad3  jge  0x1407b2adb
1407b2ad5  mov  rax, qword ptr [rax + 0x10]   ; go RIGHT
1407b2ad9  jmp  0x1407b2ae1
1407b2adb  mov  rcx, rax
1407b2ade  mov  rax, qword ptr [rax]          ; go LEFT
1407b2ae1  cmp  byte ptr [rax + 0x19], 0      ; <<< FAULT: _Isnil through a bad node pointer
```

## 2. The registers name the bug on their own

```
rax  0xffffffff301bad30     the pointer the walk followed, and faulted on
rdx  0x00000000301bad30     THE SAME POINTER, uncorrupted, in another register
```

The low 32 bits are identical; only the **high dword** differs. The inaccessible address is
`rax + 0x19 = 0xffffffff301bad49`, which matches the exception record's parameter exactly [L].

## 3. The victim, and the cleanest control this project has had

A scan of all **1216.9 MB** of committed memory in the dump for the corrupted qword:

| | |
|---|---|
| `0xffffffff301bad30` (corrupted) | **2** occurrences — one heap, one stack spill |
| `0x00000000301bad30` (correct) | **13** occurrences — the positive control |

The heap one is at `0x3a2f9a88`, which is `_Right` of node `0x3a2f9a78`. That node's pool header
at `body − 8` reads `0x20` [L] — a **bucket-1 slot**. Its bytes:

```
_Left    30ad1b30 00000000   = 0x301bad30            INTACT
_Parent  e09e8433 00000000   = 0x33849ee0            (= rcx, the walk's parent)
_Right   30ad1b30 ffffffff   = 0xffffffff301bad30    HIGH DWORD SMASHED TO -1
_Color 1  _Isnil 0  key 0x057bf610
```

**Both children point at the same tree sentinel `0x301bad30`. One is intact and one is
corrupted.** Same node, same value, same eight-byte field shape, one damaged. There is no
reading of that in which the two fields differ for a legitimate reason.

`-1` is in the writer's known value family (`1`, `2`, `-1`), and the offset is **`+4` inside an
eight-byte field** — exactly where `0x20` became `0x0000000100000020` in every previous dump.
The writer's target has always been *the second dword of an eight-byte aligned field*.

**[I], and it now has a shape worth naming:** a 32-bit counter at `+4` being incremented and
decremented through a stale pointer is what a **refcount on a freed object** looks like. That
would explain `+1`, `+2` and `-1` in one mechanism, and a 180 s timer touching a cached object
would explain the clock.

## 4. The finding that moves everything: every header the walk could reach was CLEAN

`tools/poolchain.py` over the death dump [L]:

| bucket | slots enumerated | damaged headers |
|---|---|---|
| 0 (`0x10`) | 53 376 | **0** |
| 1 (`0x20`) | 70 848 | **0** |
| 2 (`0x40`) | 33 360 | **0** |
| 3 (`0x80`) | 16 944 | **0** |

~~**Zero damaged headers in 174 528 slots.**~~ **Zero damaged headers in the ~70% of the
pool this walk can reach** - see the box below.

> **CORRECTED 2026-09-08 evening, and the correction is about the instrument, not the pool.**
> `tools/poolchain.py` and the live sentry follow **one chunk list per bucket**, from the pool
> context. There are **819** chunk lists [L]. Enumerating by chunk *shape* instead finds 3.1-3.4x
> more chunks in every dump, with the walked chunks a strict subset of them.
> `research/damage-enumeration-2026-09-08.md`.
>
> **The proof is this very dump**: the `0x40` object at `0x3b69a4a8` whose vtable pointer was
> incremented by 2 - the object that KILLED the 12:01 run, found by hand in §3 - is **not on the
> chunk list the sentry walks**. Verified by walking that list from the context head: 1967
> chunks, victim not among them [L].
>
> So "0 damaged" here, and in every sentry heartbeat ever printed, means *0 damaged where we
> looked*, and we looked at about seventy per cent. **14 of the 59 damaged objects now confirmed
> across 37 dumps sit where the sentry cannot see them.**
>
> **What survives unchanged**: the conclusion of this file. The object that killed this run was
> damaged in its PAYLOAD (`body+0x14`), not on a free header, so "the damage that kills is
> inside live objects and the sentry checks headers only" is if anything strengthened - the
> sentry is blind in two ways at once, by field and by chunk.

The sentry found one damaged header at 01:40:18 and repaired it, and every header it could
reach was intact at the end. Two things follow that this project has not said before:

* **The damage that kills is inside a LIVE object's payload**, at `body + 0x14` here — not on a
  free block's header. The sentry validates `body − 8` against the slot size and **nothing
  else**. It is not failing at this; it is structurally incapable of seeing it. Every "N headers
  repaired, pool clean" line is true and says nothing about whether the client is about to die.
* **The header damage we have been chasing for weeks is a subset** — the part of the writer's
  output that happens to land on a header rather than in a payload. We have been measuring the
  visible tail of the distribution and calling it the distribution.

## 5. The guard page would not have caught this either, and that is my error

`-GuardPage` defaults to bucket **`0x40`**, and I chose that default from runs 2 and 5, where the
victims were `0x40` map nodes. **Tonight's victim is a `0x20` slot.** So the recommended overnight
command, had it been typed in full, would have quarantined the wrong class and the client would
have died the same way with a `control PASS` line above it.

That is worth stating plainly: the flag was missing *and* the flag was aimed at the wrong bucket.
Only one of those is the owner's to fix.

The right run is `-GuardPage -GuardBucket 0x20`, and the quarantine mechanism does address this
death: the writer's stale pointer would land on a decommitted page instead of on a live tree
node, be logged with its RIP, and the page recommitted.

**The cost, stated before it is a surprise.** Bucket 1 is the hottest class: 56 744 live
allocations at the moment of death [L]. One page per live slot is roughly **230 MB** of extra
committed memory, and the retirement queue must absorb 600 seconds of `0x20` churn, which is
still unmeasured. If it cannot, the heartbeat says `FELL BACK` and the class is uncovered — which
is now the first number to read, not the last.

## 6. What this run does and does not settle

**Settled [L]:** the writer damages live objects; the pool being clean means nothing; the value
family includes `-1`; the offset is `+4` into an eight-byte field; the class this time is `0x20`;
and the sentry's repair is not, and can never be, a defence against the thing that kills.

**Not settled:** which code writes it. The dump has the *victim*, not the *writer* — nothing here
names the instruction, because the write happened long before the fault. That is exactly what the
guard page exists to answer, and it has still never run on a client.
