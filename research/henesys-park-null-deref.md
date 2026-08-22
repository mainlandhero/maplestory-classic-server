# A second crash family: a null dereference in the map load, and it is **not** the heap one

`dumps\maplecw-crash-1235208-c0000005-1.dmp`, 1 310 MB, **389 s**, 2026-08-22 09:04.

The owner: *"GoodTest teleporting to map 10001050 crashed again."* The word "again" is the thing
this file is about, because **it is a different fault**, and treating it as another instance
of `0xC0000374` would have put it on a lead that is three passes deep and does not fit it.

Tags: **[L]** read out of a file, **[D]** derived from something read, **[I]** inferred.

---

## The short version

| | the heap family (3 dumps) | **this one** |
|---|---|---|
| code | `0xC0000374` STATUS_HEAP_CORRUPTION | **`0xC0000005` ACCESS_VIOLATION** |
| raised by | `ntdll!RtlFreeHeap` refusing a bad pointer | **`MapleStory.exe+0x1d12df0`** |
| what happened | a pool slot with a damaged header reached `HeapFree` | **a read of `[0 + 0x3530]`** |
| damaged pool slots present | 1, 2 and 3 | **1** |
| damaged slot implicated | yes - it is the pointer that was freed | **no. It was never touched** |

`[L]` throughout, from `tools/dumpwalk.py` and `tools/poolchain.py`, both re-run on this dump
with their own self-checks passing.

`param[0] = 0` (a **read**) and `param[1] = 0x3530` (the address). A read of `0x3530` is a
null object pointer with a field at offset `0x3530` - not a corrupted pointer, which would be
a large arbitrary number. **[D]**

**The heap damage is present and innocent here.** `poolchain` found the usual one slot,
`0x3e1424d0  hdr 0x0000000100000020`, in the `0x20` class - **seven for seven** on the value
and the class now, 7 of 309 152 slots in the `0x20` class against **0 of 472 760** in the
other three across four dumps - and it is not the address that faulted, not on the stack, and not freed.
That is worth writing down for the opposite of the usual reason: it shows the two are
separable, and it is the first dump where the damaged slot is a bystander. **[L]**

---

## 1. Where it died, and how tightly that is pinned

```text
world.log
  13:04:53.745  <- 0x00E7  "!map 10001050"
  13:04:53.746  -> 0x01A0  SetField, GM !map 10001050
  (nothing else goes out before the fault)

client-patched\maplecw-hook.log
  09:04:53.759  WATCH #83..#88   while dispatching opcode 0x01A0
  09:04:54.074  CLIENT FAULT #1: code=0xc0000005 at 0x141d12df0
```

`[L]`. **328 ms, and no dispatch line for `0x01A0`** - the hook writes those on handler
*return*, so the client entered the SetField handler and never came back. Same reading as the
equip crash and the third heap dump.

The unwind, `.pdata`-driven **[L]**:

```text
00  0x141d12df0   fn 0x141d12ddf+0x11      <- faulted here, 17 bytes into a small function
01  0x141c4fee4   fn 0x141c4ed50+0x1194
02  0x141d1a274   fn 0x141d1a260+0x14
03  0x140f08fd3   fn 0x140f08f00+0xd3
04  0x140ee4078   fn 0x140ee4060+0x18
05  0x141d50ed2   fn 0x141d50e60+0x72
06  0x141d2519b   fn 0x141d25080+0x11b
07  0x142d300d8   fn 0x142d300c0+0x18
08  0x142d395ae   fn 0x142d39530+0x7e
09  0x142cae77d   fn 0x142cae700+0x7d      <- the packet-handler region
10  0x14209f2e3   fn 0x14209ee50+0x493     <- and these two are the field load
11  0x142098bbc   fn 0x142097f80+0xc3c
12  0x141821ff3   fn 0x141820080+0x1f73
13  0x144ada037   fn 0x144ad9fc8+0x6f
```

Frames **10 and 11 are the same two functions** the third heap dump unwinds through
(`0x14209ee50+0x3ae`, `0x142097f80+0xc3c`). Both crashes are inside the field load; they
differ in what goes wrong inside it. **[L]**

---

## 2. What was ruled out, cheaply, and the control that makes the negative mean something

This client is a **trimmed** build, so "the map names art that was cut" is a real hypothesis
rather than a hypothetical, and a null dereference in a loader is exactly its shape.

`tools/check_map_resources.py` (new) resolves every reference a map makes - each tile layer's
`info/tS` into `Tile/<tS>.img` and then `<u>/<no>`; each object's `oS` into `Obj/<oS>.img` and
then `<l0>/<l1>/<l2>/<no>`; each background's `bS` into `Back/<bS>.img` and then
`back|ani/<no>`.

```text
positive control: a bogus tile reference IS reported. The walk can say no.

map 10001050    529 references checked, 0 missing
map 10000013    282 references checked, 0 missing
map 40          552 references checked, 0 missing
map 10          165 references checked, 0 missing
```

**[L]**. The control is injected on every run for the reason `CLAUDE.md` gives at length: a
path walk with a bug in it returns "0 missing" for every map, and that is indistinguishable
from a clean result.

**This negative is bounded and the script prints its own bound**: a node that *exists* can
still carry art the client cannot decode, and nothing here can see that.

Also checked and not it: **[L]**

* `tile_woodMarble` - the one tile set unique to this map among the four - **is** in
  `Tile_000.wz`, along with 27 others.
* `mapMark: "Henesys"` **is** one of the 19 marks in `MapHelper.img`.
* `seat`, `ladderRope`, `reactor`, the eight tile layers, `miniMap`, `town: 1` - Henesys Park
  has nothing structurally that Lith Harbor (`10000000`) does not, and Lith Harbor has loaded
  on most days of this project.

---

## 3. The one thing that is NOT established, and the login that settles it

Every archived log, every map this client has ever been sent to: **[L]**

```text
10000000 x1   10000002 x1   10000010 x2   10000013 x1   10000022 x6   10001050 x1 (died)
```

So Victoria Island loads - the Lith Harbor maps are where most of this project happened.
**Map 10001050 has been tried exactly once, and that once it crashed.**

That is one observation, and it has the same two readings as the `GoodTest` question did
yesterday, which resolved the *other* way:

* **(a) map 10001050 is fatal to load.** It is the only map of its group ever attempted.
* **(b) it is the fourth map load of a 389-second session**, and something accumulates.

The evidence is genuinely split. Against (b): the fault is a *null pointer*, not a corrupted
one, and the one damaged pool slot in the process was untouched - so whatever the heap family
accumulates, it is not what nulled this field. Against (a): the client had loaded maps 10, 40
and 10000013 in the same session without complaint, and yesterday the identical-looking
"character X crashes it" turned out to be the session and not the map.

**One GM command settles it**, and it costs no launch of its own:

> `!map 10001050` **as the first thing after login**, at ~40 s of client life.
> Dies -> (a), the map. Survives -> (b), and the session is still usable.

A second command makes the same run distinguish "this map" from "this part of the world":
`!map 10001000` is Henesys town, the map `10001050`'s own `returnMap` points at, and it has
never been loaded either. **[L]**

---

## 4. What a fifth dump would be worth

Only one thing, and only if step 3 comes back (a): a dump from a **fresh** client that died
loading this map. It would carry `0x141d12ddf`'s caller chain with none of the session's
accumulated state around it, and frame 01 - `0x141c4ed50+0x1194`, a single call site 4 KB into
a large function - is then worth one Ghidra pass to name the field at `+0x3530`.

Doing that pass **now**, before the experiment, would be answering "why is this map
different" before anyone has established that it is - which is the mistake
`CLAUDE.md` records under "The thing you are comparing against may never have been a
control", and it cost three days the last time.
