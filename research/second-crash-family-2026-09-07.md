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

## 6. The free is interceptable, and the obvious way to do it would not have worked

2026-09-07, from the shipped `PCOM.dll` rather than the dump. The owner: *"since these functions
that corrupt the heap are coming from other dll, can we potentially stub them as well?"*

**First, the premise needs one correction.** PCOM, `oleaut32` and `ResMan` are on the **free**
path, not the write path - they are the victim. The writer is on a 180-second clock generated
inside `MapleStory.exe` (`the-180-second-clock-2026-09-07.md` §5a) and has never been named.
Stubbing anything on this stack cannot stop the corruption; it can only stop this particular
death.

**Second, the free wrapper cannot be stubbed outright.** `PCOM+0xe08a` is a `free`. Making it
return would leak every allocation the WZ property system makes, and that system churns on
every map change. It would take the client down faster than the bug does.

**What is available is a filtered stub**, and refusing is the *correct* action rather than a
workaround: a pool chunk is not PCOM's to free. The pool still owns it and will free it
itself; handing it to `RtlFreeHeap` can only corrupt the process heap or kill it.

### 6.1 The call site, decoded

```text
PCOM+0xe0fb  48 8b 1d 7e da 0c 00   mov  rbx, [rip+0xcda7e]   ; -> PCOM+0xdbb80
PCOM+0xe102  ff 15 80 da 0c 00      call [rip+0xcda80]        ; -> PCOM+0xdbb88
PCOM+0xe108  4c 8d 47 f8            lea  r8, [rdi-8]
PCOM+0xe10c  33 d2                  xor  edx, edx
PCOM+0xe10e  48 8b c8               mov  rcx, rax
PCOM+0xe111  ff d3                  call rbx                  ; ends at 0xe113
```

`[L]`, decoded from the DLL. **`call rbx` ends at `PCOM+0xe113`, which is exactly the return
address frame #3 recorded**, so this is provably the instruction that performed the fatal free
and not a plausible neighbour.

**The IAT is not the call site.** PCOM imports `HeapFree` at `PCOM+0xae4d0`, but this code
calls through a **cached function pointer** at `PCOM+0xdbb80`, which is past `.data`'s
`SizeOfRawData` - zero at load, filled at runtime. `[L]` An IAT hook would have installed
cleanly, logged success, and intercepted **nothing**. That is the failure this repo keeps
paying for, and it cost one `pefile` script to avoid rather than a client run.

Seven `mov reg,[PCOM+0xdbb80]` sites exist in `.text` and ten `call [PCOM+0xdbb88]`. `[L]`
So **one qword covers every free PCOM makes**, with no code patched, nothing relocated and no
short branch to move - which an inline hook on `PCOM+0xe08a` would have had, since its first
thirteen bytes end in a `jns +3`.

### 6.2 `lea r8, [rdi-8]`, and what it says about the writer

PCOM's data pointers sit **8 bytes past their block**. `rdi` held `0x6b4c2e8` - the pool
chunk's base - and PCOM freed `base − 8`, the chunk's size qword, as if it were its own block
header. `[D]`

That 8-byte prefix convention is the same one `heap-corruption-2026-09-06.md` §5.3 needs to
explain the damage offset: a `body+4` pointer (BSTR-style, 4-byte count at `body+0`) used by
code that assumes a cookie at `p − 8` writes into `body − 4`, which is the pool header's high
dword. **PCOM is code with exactly that convention, and it owns the WZ property strings that
live in the `0x20` class.** `[I]` - a reading, and `crate::writewatch` is what can settle it in
one run. It is written down because it arrived from a direction nothing else had tried: the
DLL, not the dump.

### 6.3 The guard

`crates/grap-stub/src/freeguard.rs`, `freeguard=on` (refuse) or `freeguard=observe` (log and
free anyway) in the session marker; `tools	est-server.ps1 -FreeGuard` / `-FreeGuardObserve`,
both needing `-PinPatches`. **Off by default, and off during a `writewatch` run** - it is one
more patch to the client and that run is the one measuring whether our patches matter.

The test is two identities that must agree on the same bucket, read from `mem+0`, `mem+8` and
`mem+0x10`: the chunk size, and the slot size in the low dword of slot 0's header. The high
dword is ignored, because it is `1` in precisely the case this exists for. The unit test
asserts the **exact three qwords the dump holds** at `0x6b4c2e0` -
`[0x508][0x3360dc68][0x0000000100000020]` - so if it ever stops matching, the guard would not
have caught the crash it was built for.

Its liveness control is a pass-through count printed every 120 s. Zero passes means the shim is
not on PCOM's free path, and no refusal count from that run means anything - a distinction that
would otherwise look identical to "it worked".

## 5. What could not be established without more work

1. **The instruction in `PCOM.dll` that frees the interior pointer.** Nexon code, needs a
   step-through or its own dump-time capture. §2.
2. **Whether stopping at map boundaries (a quieter idle spot) changes the rate.** Not a fix,
   and not worth a launch.

(An earlier draft asked whether the archived deaths were secretly this surface. Checked and
answered inside §2: they are not - all three are the pooled-free path, and this VARIANT death is
the first of its kind, appearing only once the repair closed the other.)
