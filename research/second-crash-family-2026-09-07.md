# The map-change crash is the SAME writer, on a surface the repair cannot reach

The owner ran a client on the new payload, it died with `0xC0000374`, and put the logs and the dump
in `Desktop\Crash Investigation\`. Process 288744, alive 1247 s, playing **ouggh (215)**.

The first read of this looked like a second, unrelated crash family. It is not. The dump carries
the family's signature inside the very block that killed the client, so this is the 180-second
writer surfacing a second way. Diagnosed from the dump alone - no new instrumentation, no
client run.

Tags: **[L]** read out of a file, **[D]** derived, **[I]** inferred.

---

## 0. The chain, end to end

1. The repair **worked**: `tools/poolchain.py` on the dump finds **0 damaged slots of 193 632**
   across all four buckets - the first clean pool this project has walked. Four repairs are in
   the hook log on the usual clock (02:51:19, 02:54:19, 02:57:20, 03:00:20, ~180 s apart). `[L]`
2. The death is on the **NT process heap** (`0x400000`), not the client pool - failure type 8,
   "block not busy", a free of a pointer Windows never issued. `tools/dumpwalk.py`, from the
   allocator's own captured stack. `[L]`
3. That stack is **inside the map change**: `CField::OnPacket` (`0x141820080`) → the `SetField`
   handler (`0x142097f80`, opcode `0x01A0`) → `ResMan.dll` → `PCOM.dll` → `oleaut32!VariantClear`
   ×2 → `RtlFreeHeap`. World entry tearing down the previous map's resources. `[L]`
4. `world.log`: at 07:02:56.380 the server sent ouggh `0x01A0 SET_FIELD, portal "out00" -> map
   20`, after ouggh walked through a portal. The client loaded map 20, and faulted 1.3 s later
   at 07:02:57.656 during the teardown. `[L]`
5. **The freed pointer is interior to a pool segment, and that segment carries the family's
   damage.** §1. This is the join.

## 1. The block that was freed is a pool segment with a corrupted header in it

`dumpwalk` said the bad free target `0x6b4c2e0` sits inside a live 16 KB NT-heap block at
`0x6b4a2b0`. Read out of the dump, that block is **not opaque - it is a segment of the client
pool**: `[L]`

```text
scan of the 16 368-byte freed block:
  0x508  (the pool's bucket-1 chunk size)      : 11 occurrences
  0x0000000100000020  (the family signature)   : 1, at 0x6b4c2f0
```

`0x508` is exactly bucket 1's chunk size (`poolchain`: *"bucket 1: slot 0x20 ... chunk 0x508
bytes"*). The pool carves its 0x20-slot chunks out of 16 KB segments it takes from the NT heap,
and this is one of them - eleven chunks in it. `[D]` And sixteen bytes past the free target sits
`0x0000000100000020`, the exact damaged 0x20-slot header this project has chased for three
weeks. `[L]`

**Why `poolchain` and the repair both missed it:** this segment is **off the live chunk chain**.
A walk from the pool context's chunk-list head does not reach it. `[L]` The repair walks that
same live chain, so a corrupted header on a detached segment is invisible to it - and `poolchain`
reporting the live pool clean and this block carrying damage are the same fact from two sides.

## 2. So it is one bug, not two

The 180-second writer corrupts 0x20-slot headers to `0x0000000100000020`. The repair fixes those
**on the live chain**, which is why the pool-free deaths stopped and the live pool is clean. But:

* The 0x20 class holds **WZ resource strings** - the caught slots have held `"322007.img"`,
  `".img"`, `reqLUK` (`research/the-180-second-clock-2026-09-07.md` §2). Those belong to a map's
  resources.
* When the player changes maps, `SetField` tears those resources down and the pool hands whole
  segments back to the NT heap through the property/VARIANT teardown.
* A segment carrying a header the repair never reached makes that teardown compute a bad free
  target - here `0x6b4c2e0`, a chunk interior to the segment rather than the segment base
  `0x6b4a2b0` - and `RtlFreeHeap` on an interior pointer is the `0xC0000374`. `[D]`

The exact instruction that turns the corrupted header into the interior free target is not traced
(it is inside `PCOM.dll`, Nexon's code, and would need a step-through). `[I]` But the family
signature sitting sixteen bytes from the free target, in a block that is provably a pool segment,
is far past coincidence.

**The free target is a pool chunk base, reached from a VARIANT.** `0x6b4c2e0` reads
`[size 0x508][next 0x3360dc68][slot header 0x0000000100000020 @ +0x10]` - the exact shape of a
pool bucket-1 chunk, whose first slot is the corrupted one. `[L]` So `VariantClear` followed a
pointer that lands on **pool-managed memory** and tried to free it as an NT-heap block. A
property object holding a pointer into the pool's own segments is a **stale pointer into freed-
and-reused memory** - which is the same disease the 180 s writer is: that writer uses such a
pointer for a refcount *write* (landing at a pool header); this path uses one for a *free*. Two
uses of the same kind of dangling pointer, two faults. `[D]`

**Why the archive never showed this surface until now.** Every earlier `0xC0000374` in the
dumps - 84428, 212108, 238480 - has frame #3 `0x14019bbf3`, the client's **pooled free**
diverting a damaged header to `HeapFree`. `[L]` None goes through `VariantClear`. That path is
precisely the one the repair neutralises, so with it closed the *next* free to trip on the same
family of dangling pointer is a different one - the VARIANT teardown here. The repair did not
create this crash; it revealed it by removing the one that used to come first.

This also explains `research/heap-crash-pattern.md`'s standing observation - *"4 of 14 died
inside `SetField`; field entry is only dangerous in a client ~3 minutes old"* - which predates
the repair. Map change has always been the second surface; the repair closed the first one and
left this one, so now it is the surface that shows.

## 3. Is it ours? Same answer as before, unchanged

The trigger was a normal portal transfer and a normal `SetField`; nothing we sent is malformed
(`user_leave_field` is one `u32`, `crates/net/src/userpool.rs:406`). The corruption accrues on
the client's own 180 s clock, inside its own Ztl/WZ resource system, on idle traffic
(`the-180-second-clock` §4). Our environment may enable the writer - never measured, and
`is-the-corruption-ours-2026-09-06.md` §1 still stands: no unhooked run exists to compare. The
immediate cause is the client's internal corruption, not a bad packet.

## 4. What this means for the mitigation, and what is left

* **The repair is doing exactly what it was built to do and no more.** Its own doc block says it
  *"only sees damage shaped like a pool header"* and *"does not stop the writer."* This crash is
  that sentence coming true: the writer's damage reached a segment the repair structurally cannot
  walk, and killed the client on a path the repair does not guard.
* **The repair still earns its place.** The pool-free deaths were the majority of the family, and
  those are gone - this client survived 20 minutes and four corruptions before a map change
  caught it, where an unrepaired client of 2026-09-06 died at 23 minutes on the pool free itself.
  It converts most of the deaths and cannot convert this one.
* **Extending the repair to off-chain segments is not available.** The pool context does not
  track detached segments; there is nothing to walk. A repair that could cover this would have to
  hook the pool's segment-release and scan each segment on the way out - which is the same order
  of build as the guard-page allocator, and still a mitigation.
* **The only real fix is still to stop the writer.** That needs its name, which needs the
  guard-page build (`heap-corruption-2026-09-06.md` §3.2) - the instrumentation this pass was
  asked to avoid. Nothing in this dump names the writing instruction; the writer had finished
  long before the map change that exposed its work.

## 5. What could not be established without more work

1. **The instruction in `PCOM.dll` that frees the interior pointer.** Nexon code, needs a
   step-through or its own dump-time capture. §2.
2. **Whether stopping at map boundaries (a quieter idle spot) changes the rate.** Not a fix,
   and not worth a launch.

(An earlier draft asked whether the archived deaths were secretly this surface. Checked and
answered inside §2: they are not - all three are the pooled-free path, and this VARIANT death is
the first of its kind, appearing only once the repair closed the other.)
