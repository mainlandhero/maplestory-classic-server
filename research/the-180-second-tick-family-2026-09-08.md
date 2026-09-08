# Decompiling did help: five 180-second tasks that load, increment or decrement, and store

The owner, 2026-09-08: *"Anything else we can do for now? Does decompiling help us here?"*

Yes. This file is what one static pass found, with **no client run spent**, and — as importantly
— what it did **not** establish. Tags: **[L]** read off this client's listing or a dump,
**[D]** derived, **[I]** inferred.

---

## 1. The search, and why it could be trusted

The writer's clock is 180.000 s (today's run learned 180.044 s including scheduling jitter), so
a millisecond timer carries `180000 = 0x2BF20`. A raw scan of `.text` for that immediate finds
**42 sites** [L].

**Controls first**, because a byte search over 50 MB finds something whatever you ask it: 60 s
(`0xEA60`) returns 98 hits and 30 s (`0x7530`) returns 154 — both intervals this client
demonstrably uses, so the search can speak.

One instrument note worth keeping: **`.themida` and `.boot` are zero bytes on disk**. They
unpack at runtime, so a zero result there is a property of the file and not evidence of absence.
Every number in this file is `.text` only.

## 2. The ticker template, confirmed

Sixteen of the 42 are one shape [L] — `research/the-180-second-clock-2026-09-07.md` predicted
it from the timing alone and here it is in code:

```
140c93551  mov  r8d, dword ptr [rsp + 0x24]   ; now
140c93556  mov  edx, 0x2bf20                  ; 180 000 ms
140c9355b  mov  ecx, dword ptr [rip + ...]    ; LAST
140c93561  call 0x1408fcaa0                   ; elapsed?
140c93566  movzx eax, al
140c9356b  je   <skip>
140c93571  mov  eax, dword ptr [rsp + 0x24]
140c93575  mov  dword ptr [rip + ...], eax    ; LAST := now, on the firing branch
```

So `FUN_1408fcaa0(last, interval, now)` is the elapsed test, and every timed task in the client
goes through it. That makes "which tasks run on a 180 s clock" a **bounded, enumerable
question** rather than a search.

## 3. Five of the sixteen have the writer's exact shape

Each allocates a small array, takes **element 0's address**, adds a **fixed offset**, then
**loads a 32-bit value, increments or decrements it, and stores it back** [L]:

| function | operation | offset | allocates |
|---|---|---|---|
| `FUN_140c93530` | **inc** | `+0x90` | yes |
| `FUN_140c936a0` | **inc** | `+0x94` | yes |
| `FUN_140c93810` | **dec** | `+0xc0` | yes |
| `FUN_140c93b70` | **dec** | `+0xe4` | yes |
| `FUN_140c93930` | **dec** | `+0x220` | yes |

The tail of the first, in full:

```
140c935d4  mov  edx, 5
140c935d9  lea  rcx, [rsp + 0x38]
140c935de  call 0x140ca61d0        ; THE 28-BYTE ALLOCATION
140c935e6  lea  rcx, [rsp + 0x38]
140c935eb  call 0x140ca22a0        ; element 0's address
140c935fa  add  rax, 0x90
140c93614  mov  eax, dword ptr [rax]
140c93616  inc  eax
140c9361d  mov  dword ptr [rcx], eax
```

`FUN_140ca61d0` with `edx = 5` is **the allocation this project already fingerprinted**: 28
bytes, the `0x20` class, observed firing 180 s apart roughly 100 ms before every sentry catch,
**14 times out of 14**. It was found by watching the client; it is now found again by reading
it, from the other direction.

**Two increments and three decrements.** The writer has been seen leaving `1`, `2` and `-1`
behind. An increment family and a decrement family account for all three values, which no
single-operation hypothesis does.

### The controls that make this mean something

If every timed task looked like this, the shape would say nothing about 180 s in particular:

| clock | tick functions | with the load/modify/store shape |
|---|---|---|
| **180 s** | 16 | **5** |
| 240 s | 3 | **0** |
| 90 s | 0 | **0** |
| 60 s | 22 | **1** |

The shape is concentrated in the one clock the corruption runs on [L].

### They are live code

`tools/callers.py`: `FUN_140c93530` is called from `0x1428923e0` and `FUN_140c93930` from
`0x142ce0130`, one call site each, no tail jumps, no data pointers [L]. Not dead code left
behind by the packer.

## 4. What this does NOT establish, stated plainly

**It is not the RIP.** Nothing here has been observed executing. The live caller of the
allocation recorded in the runs is `0x14491cafd`, which is in the **`.themida`** region, not at
`0x140c935de`. The most likely reading [I] is that Themida runs a relocated or virtualised copy
of this same routine, so `.text` tells us the *logic* while the *instruction* lives elsewhere —
which would be the ideal outcome for reading it, and is exactly why the write watch and the
guard page still matter.

**The overrun hypothesis is unconfirmed.** `FUN_140ca22a0` returns `buffer + index*4`,
bounds-checked against a count at `[buffer-8]` [L], so `+0x90` is 144 bytes past the start of a
28-byte allocation — an overrun of its own buffer, which would elegantly explain why the victim
class varies. **I tried to confirm it against both crash dumps and could not**: for each damaged
address, `damaged - 0x90` (and the other four offsets) has no pool header at `origin - 8`.

That test was **not decisive, and it is worth saying why rather than reporting a negative**: the
buffer is a local, allocated at the top of the tick and released at the bottom
(`call 0x140369050`), so by the time of the crash — minutes or hours later — that memory has
churned many times. The dump cannot see what was there at the moment of the write. A negative
from an instrument that cannot observe the thing is not evidence, which is this repo's oldest
lesson.

So the mechanism is still open between **an overrun of its own buffer** and **a stale or
dangling base pointer**, and this pass does not settle it.

## 5. What it buys, concretely

Five candidate addresses, free to watch. The probe already takes `watch@<va>` and the next run
was going to happen anyway:

```
-Probe "watch@140c93530,watch@140c936a0,watch@140c93810,watch@140c93b70,watch@140c93930"
```

* **They fire every 180 s** -> the `.text` copies are live, and the writer is one of five known
  instructions. The guard page's catch would then name which.
* **They never fire** -> the Themida copy is what executes, which is worth knowing before any
  more static effort is spent, and closes off this whole direction cheaply.

Either answer is progress, and it costs one flag on a run that is already planned.
